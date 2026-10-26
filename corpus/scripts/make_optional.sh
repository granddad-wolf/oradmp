#!/usr/bin/env bash
# make_optional.sh <18c|11g>
# Architect requests R-027 (LOB), R-029 (INTERVAL, ROWID, UROWID), R-030 (partitions, 18c only).
# Adds to corpus/fixtures/<ver>/:
#   lob_exp_<ver>.dmp  lob_exp_direct_<ver>.dmp  misc_exp_<ver>.dmp  (+ logs)  truth_lob.txt  truth_misc.txt
#   18c only: part_exp_18c.dmp  part_exp_direct_18c.dmp (+ logs)  truth_part.txt
# Exports use statistics=none. It verifies SHA256SUMS first, does nothing if lob_exp_<ver>.dmp exists
# (FORCE=1 recreates exactly these files), and rewrites SHA256SUMS at the end.
# It runs 01b_optional_tables.sql first (drops and recreates T_LOB, T_INTERVAL, T_PART, T_BIG; errors on 11g for T_PART are expected).
# Needs the original corpus tables (T_FRAME) in the CORPUS schema.

set -euo pipefail
VERSION="make_optional.sh v1 (2026-10-15)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
OP_LOG="$WORK_HOST/logs/optional_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$OP_LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" \
  || { echo "!! container $CONTAINER is not running" >&2; exit 1; }

if [ -f "$FIX_HOST/SHA256SUMS" ]; then
  echo "== verifying existing fixtures"
  if ! ( cd "$FIX_HOST" && sha256sum -c --quiet SHA256SUMS ); then
    echo "!! an existing fixture differs from SHA256SUMS. Stopping." >&2
    exit 1
  fi
  echo "   ok"
fi

if [ -e "$FIX_HOST/lob_exp_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "optional fixtures for $VER already exist; nothing to do. FORCE=1 recreates them."
  exit 0
fi
rm -f "$FIX_HOST"/lob_exp_"${VER}".* "$FIX_HOST"/lob_exp_direct_"${VER}".* "$FIX_HOST"/misc_exp_"${VER}".* \
      "$FIX_HOST"/part_exp_"${VER}".* "$FIX_HOST"/part_exp_direct_"${VER}".* \
      "$FIX_HOST"/truth_lob.txt "$FIX_HOST"/truth_misc.txt "$FIX_HOST"/truth_part.txt

# 1. tables from 01b (partitioning errors on 11g are expected)
run_sql corpus "$CORPUS_PW" 01b_optional_tables.sql || true
HAVE_PART=0
if grep -q 'ORA-' "$WORK_HOST/logs/01b_optional_tables.log" 2>/dev/null; then
  echo "   errors while creating the optional tables (expected only for T_PART on 11g):"
  grep -E 'ORA-' "$WORK_HOST/logs/01b_optional_tables.log" | sort | uniq -c | head -5
fi

# 2. extra tables and truth
run_sql corpus "$CORPUS_PW" 13_optional.sql
n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$WORK_HOST/logs/13_optional.log" || true)
if [ "$n" != "0" ]; then
  echo "!! $n error line(s) in $WORK_HOST/logs/13_optional.log. Stopping before any export." >&2
  grep -E 'ORA-|SP2-|PLS-' "$WORK_HOST/logs/13_optional.log" | head -5 >&2 || true
  exit 1
fi
if [ "$VER" = "18c" ]; then
  run_sql corpus "$CORPUS_PW" 13b_part_truth.sql
  if grep -qE 'ORA-|SP2-|PLS-' "$WORK_HOST/logs/13b_part_truth.log" 2>/dev/null; then
    echo "!! T_PART truth failed:" >&2; grep -E 'ORA-' "$WORK_HOST/logs/13b_part_truth.log" | head -3 >&2; exit 1
  fi
  HAVE_PART=1
fi
for f in truth_lob.txt truth_misc.txt; do
  [ -s "$FIX_HOST/$f" ] || { echo "!! $f was not written" >&2; exit 1; }
done
echo "   truth_lob: $(grep -c . "$FIX_HOST/truth_lob.txt") lines (7 expected), truth_misc: $(grep -c . "$FIX_HOST/truth_misc.txt") (6)"
[ "$HAVE_PART" = "1" ] && echo "   truth_part: $(grep -c . "$FIX_HOST/truth_part.txt") lines (4 expected)"

# 3. exports
t_exp() {
  local stem="$1" tables="$2" extra="${3:-}"
  local name="${stem}_${VER}"
  echo "   $name.dmp   (tables=$tables $extra)"
  docker exec "$CONTAINER" exp "$CONNECT_CORPUS" tables="$tables" statistics=none $extra \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$OP_LOG" 2>&1 \
    || echo "   !! exp reported an error for ${name} (see the log in the folder)"
}
t_exp lob_exp        t_lob,t_lob2,t_frame
t_exp lob_exp_direct t_lob,t_lob2,t_frame direct=y
t_exp misc_exp       t_interval,t_rowid,t_frame
if [ "$HAVE_PART" = "1" ]; then
  t_exp part_exp        t_part
  t_exp part_exp_direct t_part direct=y
fi

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "Optional fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Lines to look at (warnings are possible for the direct exports with LOBs):"
grep -H -E 'EXP-|ORA-|Export done|successfully|Warning' "$FIX_HOST"/{lob_exp,lob_exp_direct,misc_exp,part_exp,part_exp_direct}_"${VER}".log 2>/dev/null || true
echo
ls -l "$FIX_HOST"/{lob_exp,lob_exp_direct,misc_exp,part_exp,part_exp_direct}_"${VER}".dmp "$FIX_HOST"/truth_{lob,misc,part}.txt 2>/dev/null || true
