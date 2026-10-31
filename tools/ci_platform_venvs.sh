#!/usr/bin/env bash
# One installed-wheel check of the `platforms` job of ci.yml (T-16b, ADR-118 item 5): a fresh venv made by uv, the wheel
# and the named packages installed with `--only-binary :all:` (a source build would hide a missing wheel), then the full
# pytest suite against the installed package, run from the current folder (a clone of the commit).
# A separate script from tools/ci_install_check.sh (Linux, unchanged): this one knows the Windows venv layout (Scripts/)
# and the skips that are allowed off Linux. A failure does not stop the caller's other venvs: the exit status says it.
#
#   bash tools/ci_platform_venvs.sh NAME PYTHON "PACKAGES" "EXTRA_SKIP_FILE"
#
#   NAME             label of the venv and of the log files
#   PYTHON           a version for uv, e.g. 3.11 (UV_PYTHON_DOWNLOADS=never: an interpreter that is not there fails)
#   PACKAGES         extra packages, e.g. "pyarrow==15.0.2 pandas==2.2.3"
#   EXTRA_SKIP_FILE  a test file whose skips are allowed besides the others ("" for none)
#
# Environment: WHEEL (path), VERSION (what `oradmp --version` must say after "oradmp "), VENV_DIR, LOG_DIR (exist),
# PYTEST_PIN (default pytest==9.1.1). The user must not be root.
# Allowed skips (plan T-16b (b)): the three probe tests (tests/test_internal.py); EXTRA_SKIP_FILE; off Linux,
# tests/test_paths.py with the reason "POSIX byte file names"; on Windows the tests with the reason "POSIX permissions".
# Anything else fails. Written for bash 3.2 (macOS) and Git Bash.
set -eo pipefail

name="$1" py="$2" pkgs="$3" extra="$4"
: "${WHEEL:?WHEEL is not set}" "${VERSION:?VERSION is not set}" "${VENV_DIR:?VENV_DIR is not set}" "${LOG_DIR:?LOG_DIR is not set}"
pin="${PYTEST_PIN:-pytest==9.1.1}"
venv="$VENV_DIR/venv-$name"
log="$LOG_DIR/pytest-$name.log"

if [ "$(id -u)" = 0 ]; then
    echo "ci_platform_venvs: running as root; the root-only skips are not expected on this job" >&2
    exit 1
fi
kernel="$(uname -s)"

uv venv --quiet --python "$py" "$venv"
if [ -d "$venv/Scripts" ]; then bin="$venv/Scripts"; else bin="$venv/bin"; fi
# shellcheck disable=SC2086
uv pip install --quiet --only-binary :all: --python "$bin/python" "$WHEEL" "$pin" $pkgs

"$bin/python" -c 'import importlib.util as u, sys, oradmp, pyarrow
def ver(m):
    return m + " " + __import__(m).__version__ if u.find_spec(m) else "no " + m
print("Python %d.%d.%d, pyarrow %s, %s, %s" % (*sys.version_info[:3], pyarrow.__version__, ver("numpy"), ver("pandas")))
print("oradmp", oradmp.__version__, oradmp.__file__)' | tee "$log.info"

location="$("$bin/python" -c 'import oradmp; print(oradmp.__file__.replace(chr(92), "/"))')"
case "$location" in
    */site-packages/oradmp/*) ;;
    *)
        echo "oradmp is not the installed package" >&2
        exit 1
        ;;
esac

# from the clone of the commit (rootdir, tests/, corpus/fixtures/), no python/ on the path; a failing pytest is recorded
failed=0
"$bin/pytest" -rs -q -p no:cacheprovider 2>&1 | tee "$log" || failed=1
tr -d '\r' <"$log" >"$log.lf"
mv "$log.lf" "$log"

{ grep '^SKIPPED' "$log" || true; } >"$log.skips"
cp "$log.skips" "$log.rest"
count() { # the sum of the [n] counts of the skip lines in $log.skips that match the fixed string $1
    { grep -F -- "$1" "$log.skips" || true; } | awk '{ sub(/^SKIPPED \[/, ""); n += $0 + 0 } END { print n + 0 }'
}
drop() { # remove the lines matching the fixed string $1 from $log.rest
    { grep -v -F -- "$1" "$log.rest" || true; } >"$log.rest2"
    mv "$log.rest2" "$log.rest"
}
probes="$(count 'tests/test_internal.py')"
drop 'tests/test_internal.py'
if [ -n "$extra" ]; then
    drop "$extra"
fi
paths=0
perms=0
case "$kernel" in
    Linux) ;;
    *)
        paths="$(count 'POSIX byte file names')"
        drop 'POSIX byte file names'
        ;;
esac
case "$kernel" in
    MINGW* | MSYS* | CYGWIN*)
        perms="$(count 'POSIX permissions')"
        drop 'POSIX permissions'
        ;;
esac
echo "allowed skips in $name: probes $probes, POSIX byte file names $paths, POSIX permissions $perms" | tee "$log.allowed"
if [ -s "$log.rest" ]; then
    echo "unexpected skips in $name:" >&2
    cat "$log.rest" >&2
    failed=1
fi
if [ "$probes" != 3 ]; then
    echo "expected exactly 3 probe tests skipped in $name, got $probes" >&2
    failed=1
fi

out="$("$bin/oradmp" --version | tr -d '\r')"
echo "$name: $out"
echo "$out" >"$log.version"
if [ "$out" != "oradmp $VERSION" ]; then
    echo "oradmp --version says '$out', expected 'oradmp $VERSION'" >&2
    failed=1
fi
if [ "$failed" != 0 ]; then
    echo "ci_platform_venvs: $name FAILED (pytest, skips or version; see above)" >&2
    exit 1
fi
