#!/usr/bin/env bash
# make_types.sh <18c|11g>
# Architect requests R-024, R-025, R-026, R-028 (docs/design/open-questions.md). Adds to corpus/fixtures/<ver>/:
#   t_num2_exp_<ver>.dmp  t_decl_exp_<ver>.dmp  nchar_exp_<ver>.dmp  long_exp_<ver>.dmp  (+ logs)
#   truth_num2.txt  truth_decl.txt  truth_nchar.txt  truth_csmap.txt (the last one is optional)
# Exports use statistics=none (these requests are not about statistics) and the default client.
# It never touches existing fixtures: it verifies SHA256SUMS first, does nothing if t_num2_exp_<ver>.dmp
# exists (FORCE=1 recreates exactly the files listed above), and rewrites SHA256SUMS at the end.
# R-028 needs T_FRAME (original corpus tables) in the CORPUS schema; the SQL stops with an error if it is missing.
# R-027 (LOB), R-029 (INTERVAL, ROWID) and R-030 (partitions) are not in this script.

set -euo pipefail
VERSION="make_types.sh v1 (2026-10-08)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
TY_LOG="$WORK_HOST/logs/types_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$TY_LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" \
  || { echo "!! container $CONTAINER is not running" >&2; exit 1; }

# 1. existing fixtures must be unchanged
if [ -f "$FIX_HOST/SHA256SUMS" ]; then
  echo "== verifying existing fixtures"
  if ! ( cd "$FIX_HOST" && sha256sum -c --quiet SHA256SUMS ); then
    echo "!! an existing fixture differs from SHA256SUMS. Stopping." >&2
    exit 1
  fi
  echo "   ok"
fi

# 2. all or nothing
if [ -e "$FIX_HOST/t_num2_exp_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "types fixtures for $VER already exist; nothing to do."
  echo "FORCE=1 recreates them (dumps and truth files together)."
  exit 0
fi
rm -f "$FIX_HOST"/t_num2_exp_"${VER}".* "$FIX_HOST"/t_decl_exp_"${VER}".* \
      "$FIX_HOST"/nchar_exp_"${VER}".* "$FIX_HOST"/long_exp_"${VER}".* \
      "$FIX_HOST"/truth_num2.txt "$FIX_HOST"/truth_decl.txt "$FIX_HOST"/truth_nchar.txt "$FIX_HOST"/truth_csmap.txt

# 3. tables and truth
run_sql corpus "$CORPUS_PW" 11_types.sql
n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$WORK_HOST/logs/11_types.log" || true)
if [ "$n" != "0" ]; then
  echo "!! $n error line(s) in $WORK_HOST/logs/11_types.log. Stopping before any export." >&2
  grep -E 'ORA-|SP2-|PLS-' "$WORK_HOST/logs/11_types.log" | head -5 >&2 || true
  exit 1
fi
for f in truth_num2.txt truth_decl.txt truth_nchar.txt; do
  [ -s "$FIX_HOST/$f" ] || { echo "!! $f was not written" >&2; exit 1; }
done
echo "   truth_num2: $(grep -c . "$FIX_HOST/truth_num2.txt") lines (14 expected), truth_decl: $(grep -c . "$FIX_HOST/truth_decl.txt") (14), truth_nchar: $(grep -c . "$FIX_HOST/truth_nchar.txt") (3)"

# 3b. R-026(b), optional
run_sql corpus "$CORPUS_PW" 12_csmap.sql || true
if grep -qE 'ORA-|SP2-|PLS-' "$WORK_HOST/logs/12_csmap.log" 2>/dev/null || [ ! -s "$FIX_HOST/truth_csmap.txt" ]; then
  echo "   NOTE: truth_csmap.txt (R-026b) was not produced on $VER:"
  grep -E 'ORA-|SP2-|PLS-' "$WORK_HOST/logs/12_csmap.log" 2>/dev/null | head -3 || true
  rm -f "$FIX_HOST/truth_csmap.txt"
else
  echo "   truth_csmap.txt: $(grep -c . "$FIX_HOST/truth_csmap.txt") lines (768 expected)"
fi

# 4. exports. t_exp <file_stem> <tables>
t_exp() {
  local stem="$1" tables="$2"
  local name="${stem}_${VER}"
  echo "   $name.dmp   (tables=$tables)"
  docker exec "$CONTAINER" exp "$CONNECT_CORPUS" tables="$tables" statistics=none \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$TY_LOG" 2>&1 \
    || echo "   !! exp reported an error for ${name} (see the log in the folder)"
}
t_exp t_num2_exp t_num2
t_exp t_decl_exp t_decl
t_exp nchar_exp  t_nchar
t_exp long_exp   t_long,t_longraw,t_frame

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "Types fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Lines to look at (the exports should end without warnings):"
grep -H -E 'EXP-|ORA-|Export done|successfully|Warning' "$FIX_HOST"/{t_num2,t_decl,nchar,long}_exp_"${VER}".log || true
echo
ls -l "$FIX_HOST"/{t_num2,t_decl,nchar,long}_exp_"${VER}".dmp "$FIX_HOST"/truth_{num2,decl,nchar,csmap}.txt 2>/dev/null || true
