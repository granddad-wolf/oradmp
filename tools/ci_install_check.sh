#!/usr/bin/env bash
# One installed-wheel check for the `manylinux2014` job of ci.yml (T-16c, ADR-115 item 4): a fresh venv, the wheel
# and the named packages installed with `--only-binary :all:` (a source build would hide a missing wheel), then the
# full pytest suite against the installed package, run from the current folder (a copy of the checkout).
#
#   tools/ci_install_check.sh NAME PYTHON "PACKAGES" "EXTRA_SKIP_FILE"
#
#   NAME             label of the venv and of the log files
#   PYTHON           the interpreter: a path (INSTALLER=pip) or a version for uv (INSTALLER=uv)
#   PACKAGES         extra packages, e.g. "pyarrow==15.0.2 pandas==2.2.3"
#   EXTRA_SKIP_FILE  a test file whose skips are allowed besides the three probe tests ("" for none)
#
# Environment: WHEEL (path), VERSION (what `oradmp --version` must say after "oradmp "), VENV_DIR, LOG_DIR (exist),
# INSTALLER (pip or uv, default pip), PYTEST_PIN (default pytest==9.1.1). When the user is root the three root-only
# tests (tests/test_cli.py twice, tests/test_paths.py once, reason "root can ...") must be the only other skips.
# Exits non-zero on any failure, any failed test, any other skip. Written for bash 4.2 (the manylinux2014 image).
set -eo pipefail

name="$1" py="$2" pkgs="$3" extra="$4"
: "${WHEEL:?WHEEL is not set}" "${VERSION:?VERSION is not set}" "${VENV_DIR:?VENV_DIR is not set}" "${LOG_DIR:?LOG_DIR is not set}"
installer="${INSTALLER:-pip}"
pin="${PYTEST_PIN:-pytest==9.1.1}"
venv="$VENV_DIR/venv-$name"
log="$LOG_DIR/pytest-$name.log"

case "$installer" in
    uv)
        uv venv --quiet --python "$py" "$venv"
        # shellcheck disable=SC2086
        uv pip install --quiet --only-binary :all: --python "$venv/bin/python" "$WHEEL" "$pin" $pkgs
        ;;
    pip)
        "$py" -m venv "$venv"
        # shellcheck disable=SC2086
        "$venv/bin/python" -m pip install --quiet --only-binary :all: "$WHEEL" "$pin" $pkgs
        ;;
    *)
        echo "ci_install_check: INSTALLER must be pip or uv, not '$installer'" >&2
        exit 2
        ;;
esac

"$venv/bin/python" -c 'import importlib.util as u, sys, oradmp, pyarrow
def ver(m):
    return m + " " + __import__(m).__version__ if u.find_spec(m) else "no " + m
print("Python %d.%d.%d, pyarrow %s, %s, %s" % (*sys.version_info[:3], pyarrow.__version__, ver("numpy"), ver("pandas")))
print("oradmp", oradmp.__version__, oradmp.__file__)' | tee "$log.info"

location="$("$venv/bin/python" -c 'import oradmp; print(oradmp.__file__)')"
case "$location" in
    */site-packages/oradmp/*) ;;
    *)
        echo "oradmp is not the installed package" >&2
        exit 1
        ;;
esac

# from the copy of the checkout (rootdir, tests/, corpus/fixtures/), no python/ on the path
"$venv/bin/pytest" -rs -q -p no:cacheprovider 2>&1 | tee "$log"

{ grep '^SKIPPED' "$log" || true; } >"$log.skips"
cp "$log.skips" "$log.rest"
probes=0
roots=0
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
if [ "$(id -u)" = 0 ]; then
    roots="$(count 'root can ')"
    drop 'root can '
    if [ "$roots" != 3 ]; then
        echo "expected exactly 3 root-only tests skipped in $name, got $roots" >&2
        exit 1
    fi
fi
if [ -s "$log.rest" ]; then
    echo "unexpected skips in $name:" >&2
    cat "$log.rest" >&2
    exit 1
fi
if [ "$probes" != 3 ]; then
    echo "expected exactly 3 probe tests skipped in $name, got $probes" >&2
    exit 1
fi

out="$("$venv/bin/oradmp" --version)"
echo "$name: $out"
echo "$out" >"$log.version"
test "$out" = "oradmp $VERSION"
