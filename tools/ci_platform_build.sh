#!/usr/bin/env bash
# The native build of the `platforms` job of ci.yml (T-16b, ADR-118): wheel and sdist with maturin 1.15.0 (no --zig),
# path remaps as tools/release_build.sh does, tests/check_dist.py with --build-root, then the wheel and the sdist a second
# time into another folder for the reproducibility measurement (Q-15; recorded, not a gate). Nothing is published or kept.
# tools/release_build.sh stays the Linux release route; this script is separate because that one needs `maturin --zig`,
# GNU `realpath -m` and a POSIX venv layout (bin/), none of which exist on macOS or Windows (Git Bash).
#
#   bash tools/ci_platform_build.sh
#
# Environment: PLAT_ROOT (the build folder, which must not exist; a form both bash and native Python read, `D:/a/_temp/...`
# on Windows; on macOS /private/var/tmp/..., because the runner's temporary folder is inside the user's home and
# check_dist rejects /Users/ paths), GITHUB_SHA (the commit), GITHUB_WORKSPACE (the checkout). SOURCE_DATE_EPOCH is
# derived here from the commit time. Below PLAT_ROOT: src (a clone of the commit), venv, target, dist, dist2.
# Exit status: 0 only if both builds worked, check_dist passed, and the file names are as expected. The comparison of
# the two builds never changes the exit status. Written for bash 3.2 (macOS) and Git Bash.
set -euo pipefail

: "${GITHUB_SHA:?GITHUB_SHA is not set}" "${GITHUB_WORKSPACE:?GITHUB_WORKSPACE is not set}"
if command -v cygpath >/dev/null 2>&1; then
    windows=1
    here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -W)"
    workspace="$(cygpath -m "$GITHUB_WORKSPACE")"
else
    windows=0
    here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    workspace="$GITHUB_WORKSPACE"
fi
report="$here/ci_platform_report.py"
ROOT="${PLAT_ROOT:?PLAT_ROOT is not set}"
[ ! -e "$ROOT" ] || {
    echo "ci_platform_build: the build root $ROOT exists already" >&2
    exit 1
}

mkdir -p "$ROOT"
git clone --no-local --quiet "$workspace" "$ROOT/src"
git -C "$ROOT/src" -c advice.detachedHead=false checkout --quiet "$GITHUB_SHA"
epoch="$(git -C "$ROOT/src" log -1 --format=%ct "$GITHUB_SHA")"

python -m venv "$ROOT/venv"
if [ -d "$ROOT/venv/Scripts" ]; then bin="$ROOT/venv/Scripts"; else bin="$ROOT/venv/bin"; fi
"$bin/python" -m pip install --quiet "maturin==1.15.0"

# the cargo home in every form rustc may print it (a Windows path has backslashes and a drive letter)
cargo_home="${CARGO_HOME:-$HOME/.cargo}"
flags="--remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$ROOT=/build"
if [ "$windows" = 1 ]; then
    flags="$flags --remap-path-prefix=$(cygpath -m "$cargo_home")=/cargo --remap-path-prefix=$(cygpath -w "$cargo_home")=/cargo"
    flags="$flags --remap-path-prefix=$(cygpath -w "$ROOT")=/build"
fi
export RUSTFLAGS="$flags"
export CARGO_TARGET_DIR="$ROOT/target"
export SOURCE_DATE_EPOCH="$epoch"
export PATH="$bin:$PATH"

build() { # build the wheel and the sdist into the folder $1
    (
        cd "$ROOT/src"
        maturin build --release --locked --out "$1"
        maturin sdist --out "$1"
    )
}

echo "--- ci_platform_build"
echo "commit:   $GITHUB_SHA"
echo "rustc:    $(rustc --version)"
echo "maturin:  $(maturin --version)"
echo "python:   $(python -V 2>&1)"
echo "epoch:    $epoch (SOURCE_DATE_EPOCH)"
echo "root:     $ROOT"
echo "RUSTFLAGS: $RUSTFLAGS"

build "$ROOT/dist"
python -I "$report" sums "$ROOT/dist"
wheel="$(ls "$ROOT"/dist/oradmp-*.whl)"
python -I "$report" tags "$wheel"
case "$(basename "$wheel")" in
    *-cp311-abi3-*.whl) ;;
    *)
        echo "ci_platform_build: the wheel name $(basename "$wheel") does not hold -cp311-abi3-" >&2
        exit 1
        ;;
esac

# check_dist is not weakened: a failure is reported after the second build, so that one dispatch shows both results
status=0
python -I "$ROOT/src/tests/check_dist.py" --build-root "$ROOT" "$wheel" "$(ls "$ROOT"/dist/oradmp-*.tar.gz)" || status=$?
if [ "$status" = 0 ]; then echo "check_dist: ok (exit 0)"; else echo "check_dist: FAILED (exit $status)"; fi

rm -rf "$ROOT/target"
build "$ROOT/dist2"
echo "--- two builds, same commit, SOURCE_DATE_EPOCH=$epoch (Q-15; recorded, not a gate)"
python -I "$report" compare "$ROOT/dist" "$ROOT/dist2"

exit "$status"
