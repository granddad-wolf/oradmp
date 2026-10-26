#!/usr/bin/env bash
# One libFuzzer run of one fuzz target (T-13i, ADR-64 item 3). Usage: tools/fuzz_run.sh <walk|read|values> [seconds]
# (default 600). Runs from fuzz/, so rustup takes fuzz/rust-toolchain.toml (the dated nightly) and not the root's pin.
# TMPDIR defaults to /var/tmp/oradmp-fuzz (ORADMP_FUZZ_TMP): /tmp may be a small tmpfs. Needs 2 GiB free there.
# Exit status: non-zero when libFuzzer fails or fuzz/artifacts/<target>/ holds a file (a finding).
set -euo pipefail

target="${1:-}"
seconds="${2:-600}"
case "$target" in
    walk | read | values) ;;
    *)
        echo "usage: tools/fuzz_run.sh <walk|read|values> [seconds]" >&2
        exit 2
        ;;
esac
case "$seconds" in
    '' | *[!0-9]*)
        echo "fuzz_run: seconds must be a whole number" >&2
        exit 2
        ;;
esac

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tmp="${ORADMP_FUZZ_TMP:-/var/tmp/oradmp-fuzz}"
mkdir -p "$tmp"
free_kib="$(df -Pk "$tmp" | awk 'NR == 2 { print $4 }')"
if [ "${free_kib:-0}" -lt 2097152 ]; then
    echo "fuzz_run: less than 2 GiB free in $tmp (set ORADMP_FUZZ_TMP to a bigger folder)" >&2
    exit 1
fi

if [ ! -d "$root/target/fuzz-seeds" ]; then
    "$root/tools/fuzz_seeds.sh"
fi
mkdir -p "$root/fuzz/corpus/$target"

cd "$root/fuzz"
status=0
TMPDIR="$tmp" cargo fuzz run "$target" "corpus/$target" ../target/fuzz-seeds -- \
    "-max_total_time=$seconds" -max_len=1048576 -timeout=10 -rss_limit_mb=2048 -malloc_limit_mb=64 \
    -jobs=1 -workers=1 || status=$?

# leftovers of this run: the temporary folders of the `read` target and libFuzzer's log
rm -rf "$tmp"/oradmp-fuzz-* 2>/dev/null || true
if [ -f fuzz-0.log ]; then
    mv -f fuzz-0.log "$tmp/fuzz-0.log"
fi

if [ -d "artifacts/$target" ] && [ -n "$(ls -A "artifacts/$target")" ]; then
    echo "fuzz_run: FINDING in fuzz/artifacts/$target/ (see CONTRIBUTING.md, Fuzzing)" >&2
    exit 1
fi
exit "$status"
