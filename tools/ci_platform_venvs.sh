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
# Allowed skips (plan T-16f item 0, ADR-131 item 4; every category matches on the test path AND the reason):
#   - tests/test_internal.py, reason "_core has no _probe_panic": exactly 3 (the probe tests, everywhere);
#   - EXTRA_SKIP_FILE: exactly one skip line from that file where it is set (the pandas module skip of the pyarrow-only venv);
#   - off Linux, tests/test_paths.py, reason "POSIX byte file names": exactly 8;
#   - off Linux, tests/test_release_build.py, reason "release_build.sh is Linux-only": exactly 9 (every test of the module);
#   - on Windows, tests/test_cli.py, reason "POSIX permissions".
# Anything else fails, and so does a count that is not the exact one (a skipped test that starts to run is a change).
# `bash tools/ci_platform_venvs.sh --self-test` runs the gate on synthetic `pytest -rs` outputs with a stubbed `uname`.
# Written for bash 3.2 (macOS) and Git Bash.
set -eo pipefail

# Expected skip counts, each with the module that owns the tests (a change of that module changes the constant).
PROBE_SKIPS=3    # tests/test_internal.py: three probe tests (pytestmark skipif: _core built without --features test-probes)
PATHS_SKIPS=8    # tests/test_paths.py: the eight tests of the module (pytestmark skipif: sys.platform != "linux")
RELEASE_SKIPS=9  # tests/test_release_build.py: every test of the module (pytestmark skipif: not Linux)
EXTRA_SKIPS=1    # EXTRA_SKIP_FILE (tests/test_consumers.py in the pyarrow-only venv): the one module-level skip line

count() { # count PATH REASON: the sum of the [n] counts of the lines of $log.skips that hold both fixed strings
    { grep -F -- "$1" "$log.skips" | grep -F -- "$2" || true; } | awk '{ sub(/^SKIPPED \[/, ""); n += $0 + 0 } END { print n + 0 }'
}
drop() { # drop PATH REASON: remove the lines that hold both fixed strings from $log.rest
    P="$1" R="$2" awk 'index($0, ENVIRON["P"]) && index($0, ENVIRON["R"]) { next } { print }' "$log.rest" >"$log.rest2"
    mv "$log.rest2" "$log.rest"
}

skip_gate() { # skip_gate NAME EXTRA: the skip gate over $log (a pytest -rs output); 0 if every skip is allowed and counted exactly
    local name="$1" extra="$2" kernel probes paths perms rel nextra bad=0
    kernel="$(uname -s)"
    { grep '^SKIPPED' "$log" || true; } >"$log.skips"
    cp "$log.skips" "$log.rest"
    probes="$(count 'tests/test_internal.py' '_core has no _probe_panic')"
    drop 'tests/test_internal.py' '_core has no _probe_panic'
    nextra=0
    if [ -n "$extra" ]; then
        nextra="$(count "$extra" '')"
        nlines="$({ grep -F -- "$extra" "$log.skips" || true; } | wc -l | tr -d ' ')"
        drop "$extra" ''
        if [ "$nextra" != "$EXTRA_SKIPS" ] || [ "$nlines" != 1 ]; then
            echo "expected exactly $EXTRA_SKIPS skip line from $extra in $name, got $nextra skips on $nlines lines" >&2
            bad=1
        fi
    fi
    paths=0
    perms=0
    rel=0
    case "$kernel" in
        Linux) ;;
        *)
            rel="$(count 'tests/test_release_build.py' 'release_build.sh is Linux-only')"
            drop 'tests/test_release_build.py' 'release_build.sh is Linux-only'
            paths="$(count 'tests/test_paths.py' 'POSIX byte file names')"
            drop 'tests/test_paths.py' 'POSIX byte file names'
            ;;
    esac
    case "$kernel" in
        MINGW* | MSYS* | CYGWIN*)
            perms="$(count 'tests/test_cli.py' 'POSIX permissions')"
            drop 'tests/test_cli.py' 'POSIX permissions'
            ;;
    esac
    echo "allowed skips in $name: probes $probes, POSIX byte file names $paths, Linux-only release tooling $rel, POSIX permissions $perms, extra $nextra" | tee "$log.allowed"
    if [ -s "$log.rest" ]; then
        echo "unexpected skips in $name:" >&2
        cat "$log.rest" >&2
        bad=1
    fi
    if [ "$kernel" != Linux ] && [ "$rel" != "$RELEASE_SKIPS" ]; then
        echo "expected exactly $RELEASE_SKIPS Linux-only release tooling tests skipped in $name, got $rel" >&2
        bad=1
    fi
    if [ "$kernel" != Linux ] && [ "$paths" != "$PATHS_SKIPS" ]; then
        echo "expected exactly $PATHS_SKIPS POSIX byte file names tests skipped in $name, got $paths" >&2
        bad=1
    fi
    if [ "$probes" != "$PROBE_SKIPS" ]; then
        echo "expected exactly $PROBE_SKIPS probe tests skipped in $name, got $probes" >&2
        bad=1
    fi
    return "$bad"
}

self_test() { # synthetic pytest -rs outputs, a stubbed uname; every case names its expected exit status
    local tmp bad=0 kern
    tmp="$(mktemp -d "${TMPDIR:-/tmp}/ci_platform_venvs_selftest.XXXXXX")"
    log="$tmp/pytest-case.log"
    uname() { echo "$kern"; } # stub: the gate asks for the kernel name only
    local probe='SKIPPED [3] tests/test_internal.py:18: _core has no _probe_panic: built without --features test-probes'
    local paths='SKIPPED [8] tests/test_paths.py:22: POSIX byte file names'
    local rel='SKIPPED [9] tests/test_release_build.py:21: release_build.sh is Linux-only (ADR-110)'
    local pandas="SKIPPED [1] tests/test_consumers.py:18: could not import 'pandas': No module named 'pandas'"
    local perm='SKIPPED [2] tests/test_cli.py:595: POSIX permissions'
    case_() { # case_ LABEL KERNEL EXTRA EXPECTED_EXIT LINE...
        local label="$1" got want="$4"
        kern="$2"
        local extra="$3"
        shift 4
        { echo "..... [100%]"; for l in "$@"; do echo "$l"; done; echo "246 passed in 1.00s"; } >"$log"
        skip_gate "self-test" "$extra" >"$tmp/out" 2>&1 && got=0 || got=$?
        if [ "$got" = "$want" ]; then
            echo "self-test ok:   $label (exit $got)"
        else
            echo "self-test FAIL: $label (exit $got, expected $want)"
            cat "$tmp/out"
            bad=1
        fi
    }
    case_ 'macOS as recorded, 3 + 8 + 9' Darwin '' 0 "$probe" "$paths" "$rel"
    case_ 'macOS pyarrow-only venv with its pandas line' Darwin tests/test_consumers.py 0 "$probe" "$paths" "$rel" "$pandas"
    case_ 'one unknown skip' Darwin '' 1 "$probe" "$paths" "$rel" 'SKIPPED [1] tests/test_stream.py:9: unknown'
    case_ 'probes missing' Darwin '' 1 "$paths" "$rel"
    case_ 'release tooling 8 instead of 9' Darwin '' 1 "$probe" "$paths" 'SKIPPED [8] tests/test_release_build.py:21: release_build.sh is Linux-only (ADR-110)'
    case_ 'POSIX permissions on macOS' Darwin '' 1 "$probe" "$paths" "$rel" "$perm"
    case_ 'POSIX byte file names 0 of 8' Darwin '' 1 "$probe" "$rel"
    case_ 'POSIX byte file names 7 of 8' Darwin '' 1 "$probe" 'SKIPPED [7] tests/test_paths.py:22: POSIX byte file names' "$rel"
    case_ 'POSIX byte file names from tests/test_truth.py' Darwin '' 1 "$probe" 'SKIPPED [8] tests/test_truth.py:22: POSIX byte file names' "$rel"
    case_ 'pyarrow-only venv without its pandas line' Darwin tests/test_consumers.py 1 "$probe" "$paths" "$rel"
    case_ 'Linux form with a POSIX byte file names skip' Linux '' 1 "$probe" "$paths"
    case_ 'Linux form as expected (probes only)' Linux '' 0 "$probe"
    case_ 'Windows POSIX permissions allowed' MINGW64_NT-10.0 '' 0 "$probe" "$paths" "$rel" "$perm"
    rm -rf "$tmp"
    return "$bad"
}

if [ "${1:-}" = "--self-test" ]; then
    self_test && echo "ci_platform_venvs: self-test passed" || {
        echo "ci_platform_venvs: self-test FAILED" >&2
        exit 1
    }
    exit 0
fi

name="$1" py="$2" pkgs="$3" extra="$4"
: "${WHEEL:?WHEEL is not set}" "${VERSION:?VERSION is not set}" "${VENV_DIR:?VENV_DIR is not set}" "${LOG_DIR:?LOG_DIR is not set}"
pin="${PYTEST_PIN:-pytest==9.1.1}"
venv="$VENV_DIR/venv-$name"
log="$LOG_DIR/pytest-$name.log"

if [ "$(id -u)" = 0 ]; then
    echo "ci_platform_venvs: running as root; the root-only skips are not expected on this job" >&2
    exit 1
fi

uv venv --quiet --python "$py" "$venv"
if [ -d "$venv/Scripts" ]; then bin="$venv/Scripts"; else bin="$venv/bin"; fi
# shellcheck disable=SC2086
# a native uv checks that the file named by --python exists; on Windows that file is python.exe (bash alone adds the .exe)
pyexe="$bin/python"
if [ -f "$bin/python.exe" ]; then pyexe="$bin/python.exe"; fi
uv pip install --quiet --only-binary :all: --python "$pyexe" "$WHEEL" "$pin" $pkgs

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

skip_gate "$name" "$extra" || failed=1

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
