#!/usr/bin/env bash
# make_probes.sh <18c|11g>
# Adds the probe fixtures requested by the Analyst (docs/formats/requests.md) to corpus/fixtures/<ver>/.
# It never touches existing fixtures:
#   1. it verifies SHA256SUMS first and stops if an existing fixture has changed;
#   2. it skips any probe file that already exists (set FORCE=1 to overwrite probe files);
#   3. it rewrites SHA256SUMS at the end; the old hashes stay the same because the old files did not change.
#
# Probes:
#   R-005  t_frame exported with another CLIENT character set (NLS_LANG), conventional and direct:
#            t_frame_exp_w1252_*, t_frame_exp_direct_w1252_*   (WE8MSWIN1252, id 178)
#            t_frame_exp_us7ascii_*, t_frame_exp_direct_us7ascii_*   (US7ASCII, id 1)
#   R-006  t_probe6 (300 rows, crosses 8192-byte boundaries): exp, exp direct=y, Data Pump
#   R-007  t_frame exported with another RECORDLENGTH (conventional only):
#            t_frame_exp_rl4096_*, t_frame_exp_rl16384_*
#   R-008  t_text (non-ASCII text) exported with another client character set, conventional and direct:
#            t_text_exp_w1252_*, t_text_exp_direct_w1252_*, t_text_exp_us7ascii_*, t_text_exp_direct_us7ascii_*
#   R-009  an empty table before and between other tables, conventional and direct:
#            multi_empty_exp_* (t_frame,t_empty,t_len), multi_empty2_exp_* (t_empty,t_frame)
#
# The log of this script (work/<ver>/logs/probes_<ver>.log) is appended to on every run. Each export
# also leaves its own log next to the dump in the fixtures folder.
#
# The direct-path exports with a client character set different from the database may be refused by
# Oracle; a failure is reported and logged, and the other probes still run.

set -euo pipefail
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"

mkdir -p "$WORK_HOST/logs"
PROBE_LOG="$WORK_HOST/logs/probes_${VER}.log"
# Append, never truncate: a second run that skips existing files must not erase the log of the first.
echo "=== make_probes.sh ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$PROBE_LOG"

# 1. existing fixtures must be unchanged
if [ -f "$FIX_HOST/SHA256SUMS" ]; then
  echo "== verifying existing fixtures"
  if ! ( cd "$FIX_HOST" && sha256sum -c --quiet SHA256SUMS ); then
    echo "!! an existing fixture differs from SHA256SUMS. Stopping." >&2
    exit 1
  fi
  echo "   ok"
fi

# 2. probe table and its truth file
run_sql corpus "$CORPUS_PW" 04_probes.sql

# probe_exp <file_stem> <NLS_LANG or -> <direct y|n> <table> [extra exp arguments...]
probe_exp() {
  local stem="$1" nls="$2" direct="$3" table="$4"; shift 4
  local name="${stem}_${VER}"
  if [ -e "$FIX_HOST/${name}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
    echo "   exists, skipped: ${name}.dmp"
    return 0
  fi
  local envargs=()
  if [ "$nls" != "-" ]; then
    envargs=(-e "NLS_LANG=$nls")
  fi
  echo "   $name.dmp"
  docker exec "${envargs[@]}" "$CONTAINER" exp "$CONNECT_CORPUS" \
      tables="$table" direct="$direct" \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" "$@" \
      >> "$PROBE_LOG" 2>&1 \
    || echo "   !! exp failed for ${name} (see $PROBE_LOG)"
}

echo "== R-005 other client character set"
probe_exp t_frame_exp_w1252          "AMERICAN_AMERICA.WE8MSWIN1252" n t_frame
probe_exp t_frame_exp_direct_w1252   "AMERICAN_AMERICA.WE8MSWIN1252" y t_frame
probe_exp t_frame_exp_us7ascii       "AMERICAN_AMERICA.US7ASCII"     n t_frame
probe_exp t_frame_exp_direct_us7ascii "AMERICAN_AMERICA.US7ASCII"    y t_frame

echo "== R-006 row data longer than one 8192-byte unit"
probe_exp t_probe6_exp         - n t_probe6
probe_exp t_probe6_exp_direct  - y t_probe6
if [ ! -e "$FIX_HOST/t_probe6_dp_${VER}.dmp" ] || [ "${FORCE:-0}" = "1" ]; then
  echo "   t_probe6_dp_${VER}.dmp"
  dp_export fixtures_dir t_probe6 tables=t_probe6 || echo "   !! expdp failed for t_probe6"
else
  echo "   exists, skipped: t_probe6_dp_${VER}.dmp"
fi

echo "== R-007 other RECORDLENGTH"
probe_exp t_frame_exp_rl4096   - n t_frame recordlength=4096
probe_exp t_frame_exp_rl16384  - n t_frame recordlength=16384

echo "== R-008 non-ASCII text (t_text) with another client character set"
probe_exp t_text_exp_w1252            "AMERICAN_AMERICA.WE8MSWIN1252" n t_text
probe_exp t_text_exp_direct_w1252     "AMERICAN_AMERICA.WE8MSWIN1252" y t_text
probe_exp t_text_exp_us7ascii         "AMERICAN_AMERICA.US7ASCII"     n t_text
probe_exp t_text_exp_direct_us7ascii  "AMERICAN_AMERICA.US7ASCII"     y t_text

echo "== R-009 empty table between and before other tables"
probe_exp multi_empty_exp           - n "t_frame,t_empty,t_len"
probe_exp multi_empty_exp_direct    - y "t_frame,t_empty,t_len"
probe_exp multi_empty2_exp          - n "t_empty,t_frame"
probe_exp multi_empty2_exp_direct   - y "t_empty,t_frame"

make_files_readable

( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "Probe fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Export output: $PROBE_LOG   (look for ORA-/EXP-/UDE- lines)"
