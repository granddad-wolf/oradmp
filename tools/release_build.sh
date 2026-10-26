#!/usr/bin/env bash
# Builds the release wheel and sdist of oradmp from a clean clone at a neutral path (ADR-56).
#
#   tools/release_build.sh [COMMIT] [--twice] [--release]
#
#   COMMIT   the commit to build (default HEAD of this repository). Uncommitted changes are not built.
#   --twice  after the first build, build the wheel and the sdist a second time from scratch into $ROOT/dist2
#            and print whether the hashes agree and, if not, which members differ (reproducibility is not a
#            release gate, ADR-56 item 3).
#   --release  a release build (ADR-90): the build root must be the canonical /var/tmp/oradmp-release (after
#            `realpath -m`), so ORADMP_BUILD_ROOT must be unset or name that folder; any other root is refused
#            with both paths named. Prints `release root: canonical`. Without it any root is accepted.
#
# Environment:
#   ORADMP_BUILD_ROOT  the build folder (default /var/tmp/oradmp-release). It must not exist, must not be
#                      inside $HOME, and no component of it may be the user's name: the paths of the build end
#                      up in the SBOM and the panic locations of the wheel, and tests/check_dist.py rejects a
#                      home path.
#   CARGO_HOME         default $HOME/.cargo (remapped to /cargo in the binary).
#
# Needs: git, uv, a Rust toolchain on PATH, the network for `uv pip install` (maturin 1.15.0 and ziglang 0.17.0) and
# for the crates (once). About 1 GB free below the build root. The output is $ROOT/dist with the wheel, the
# sdist and SHA256SUMS. Exit status non-zero on any failure.
set -euo pipefail

commit=HEAD
twice=0
release=0
for arg in "$@"; do
    case "$arg" in
        --twice) twice=1 ;;
        --release) release=1 ;;
        -h | --help)
            sed -n '2,22p' "$0"
            exit 0
            ;;
        -*)
            echo "release_build: unknown option $arg" >&2
            exit 2
            ;;
        *) commit="$arg" ;;
    esac
done

repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROOT="${ORADMP_BUILD_ROOT:-/var/tmp/oradmp-release}"
ROOT="${ROOT%/}"
home_dir="${HOME:-}"
user_name="$(id -un)"

fail() {
    echo "release_build: $*" >&2
    exit 1
}

case "$ROOT" in
    /*) ;;
    *) fail "ORADMP_BUILD_ROOT must be an absolute path: $ROOT" ;;
esac
# ADR-65 item 8: resolve symlinks and `..` before the home and user-name checks, so that a link from /var/tmp into
# the home directory does not put a home path into the wheel. `realpath -m` accepts a folder that does not exist yet.
ROOT="$(realpath -m -- "$ROOT")"
canonical_root="$(realpath -m -- /var/tmp/oradmp-release)"
if [ "$release" = 1 ]; then
    [ "$ROOT" = "$canonical_root" ] || fail "--release needs the canonical build root $canonical_root, but the build root is $ROOT"
    echo "release root: canonical"
fi
if [ -n "$home_dir" ]; then
    home_dir="$(realpath -m -- "$home_dir")"
fi
if [ -n "$home_dir" ]; then
    case "$ROOT/" in
        "$home_dir"/*) fail "the build root $ROOT is inside the home directory $home_dir" ;;
    esac
fi
IFS=/ read -r -a components <<<"$ROOT"
for c in "${components[@]}"; do
    if [ -n "$c" ] && [ "$c" = "$user_name" ]; then
        fail "the build root $ROOT has the user name ($user_name) as a path component"
    fi
done
[ ! -e "$ROOT" ] || fail "the build root $ROOT exists already; remove it or set ORADMP_BUILD_ROOT"

full="$(git -C "$repo" rev-parse --verify "$commit^{commit}")" || fail "no such commit: $commit"
cargo_home="${CARGO_HOME:-$home_dir/.cargo}"

mkdir -p "$ROOT"
git clone --no-local --quiet "$repo" "$ROOT/src"
git -C "$ROOT/src" -c advice.detachedHead=false checkout --quiet "$full"
epoch="$(git -C "$ROOT/src" log -1 --format=%ct "$full")"

uv venv --quiet "$ROOT/venv"
uv pip install --quiet --python "$ROOT/venv/bin/python" "maturin==1.15.0" "ziglang==0.17.0"
export PATH="$ROOT/venv/bin:$PATH"

export CARGO_TARGET_DIR="$ROOT/target"
export RUSTFLAGS="--remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$ROOT=/build"
export SOURCE_DATE_EPOCH="$epoch"

build() { # build into the folder $1
    local out="$1"
    (
        cd "$ROOT/src"
        maturin build --release --locked --zig --compatibility manylinux_2_28 --out "$out"
        maturin sdist --out "$out"
    )
}

check() { # check the archives of the folder $1 and write its SHA256SUMS
    local out="$1" wheel sdist
    wheel="$(ls "$out"/oradmp-*.whl)"
    sdist="$(ls "$out"/oradmp-*.tar.gz)"
    "$ROOT/venv/bin/python" -I "$ROOT/src/tests/check_dist.py" --build-root "$ROOT" "$wheel" "$sdist"
    (cd "$out" && sha256sum -- *.whl *.tar.gz >SHA256SUMS)
}

build "$ROOT/dist"
check "$ROOT/dist"

echo "--- release_build"
echo "commit:   $full"
echo "rustc:    $(rustc --version)"
echo "maturin:  $(maturin --version)"
echo "zig:      $(python -c 'import ziglang, subprocess, sys; print(subprocess.check_output([sys.executable, "-m", "ziglang", "version"], text=True).strip())')"
echo "epoch:    $epoch (SOURCE_DATE_EPOCH)"
echo "root:     $ROOT"
cat "$ROOT/dist/SHA256SUMS"

if [ "$twice" = 1 ]; then
    rm -rf "$ROOT/target"
    build "$ROOT/dist2"
    check "$ROOT/dist2"
    echo "--- --twice"
    python -I - "$ROOT/dist" "$ROOT/dist2" <<'PY'
import glob
import hashlib
import sys
import tarfile
import zipfile

a, b = sys.argv[1:3]


def digest(p):
    return hashlib.sha256(open(p, "rb").read()).hexdigest()


def members(p):
    if p.endswith(".whl"):
        with zipfile.ZipFile(p) as z:
            return {n: hashlib.sha256(z.read(n)).hexdigest() for n in z.namelist()}
    with tarfile.open(p) as t:
        out = {}
        for m in t.getmembers():
            f = t.extractfile(m)
            out[m.name] = hashlib.sha256(f.read()).hexdigest() if f else "dir"
        return out


same = True
for pattern in ("oradmp-*.whl", "oradmp-*.tar.gz"):
    pa, pb = glob.glob(f"{a}/{pattern}")[0], glob.glob(f"{b}/{pattern}")[0]
    if digest(pa) == digest(pb):
        print(f"identical: {pa.rsplit('/', 1)[1]}")
        continue
    same = False
    ma, mb = members(pa), members(pb)
    differ = sorted(n for n in set(ma) | set(mb) if ma.get(n) != mb.get(n))
    print(f"DIFFERENT: {pa.rsplit('/', 1)[1]}; members that differ: {differ}")
print("reproducible" if same else "not reproducible (not a release gate)")
PY
fi
