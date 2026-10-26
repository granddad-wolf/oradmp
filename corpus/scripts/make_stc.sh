#!/usr/bin/env bash
# make_stc.sh <18c|11g>
# R-020 (docs/formats/requests.md): tables where not every column has statistics.
# Adds to corpus/fixtures/<ver>/:
#   stc_exp_<ver>.dmp, stc_exp_direct_<ver>.dmp   tables=t_stc1..t_stc5, conventional / direct path
#   their logs, truth_stc.txt
# It never touches existing fixtures:
#   1. it verifies SHA256SUMS first and stops if an existing fixture has changed;
#   2. it does nothing if stc_exp_<ver>.dmp already exists (FORCE=1 recreates all stc_* files and truth_stc.txt);
#   3. it rewrites SHA256SUMS at the end.
# It stops before exporting if creating the tables or the truth file reports an error.

set -euo pipefail
VERSION="make_stc.sh v1 (2026-10-08)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
STC_LOG="$WORK_HOST/logs/stc_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$STC_LOG"

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
if [ -e "$FIX_HOST/stc_exp_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "stc_* fixtures for $VER already exist; nothing to do."
  echo "FORCE=1 recreates all of them (dumps and truth file together)."
  exit 0
fi
rm -f "$FIX_HOST"/stc_exp*_"${VER}".dmp "$FIX_HOST"/stc_exp*_"${VER}".log "$FIX_HOST"/truth_stc.txt

# 3. tables, statistics, truth
run_sql corpus "$CORPUS_PW" 10_stc.sql
n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$WORK_HOST/logs/10_stc.log" || true)
if [ "$n" != "0" ]; then
  echo "!! $n error line(s) in $WORK_HOST/logs/10_stc.log. Stopping before any export." >&2
  exit 1
fi
[ -s "$FIX_HOST/truth_stc.txt" ] || { echo "!! truth_stc.txt was not written" >&2; exit 1; }
echo "   truth_stc.txt: $(grep -c '^COLUMN|' "$FIX_HOST/truth_stc.txt") column rows"

# 4. exports. st_exp <file_stem> <NLS_LANG or -> <direct y|n> <statistics estimate|compute|none>
st_exp() {
  local stem="$1" nls="$2" direct="$3" st="$4"
  local name="${stem}_${VER}"
  local envargs=()
  if [ "$nls" != "-" ]; then
    envargs=(-e "NLS_LANG=$nls")
  fi
  echo "   $name.dmp   (client ${nls}, direct=$direct, statistics=$st)"
  docker exec "${envargs[@]}" "$CONTAINER" exp "$CONNECT_CORPUS" tables=t_stc1,t_stc2,t_stc3,t_stc4,t_stc5 \
      direct="$direct" statistics="$st" file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" \
      >> "$STC_LOG" 2>&1 \
    || echo "   !! exp reported an error for ${name} (see the log in the folder)"
}

st_exp stc_exp          -   n estimate
st_exp stc_exp_direct   -   y estimate

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "R-020 fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Lines to look at (the exports should end without warnings):"
grep -H -E 'EXP-|ORA-|Export done|successfully|Warning' "$FIX_HOST"/stc_exp*_"${VER}".log || true
echo
ls -l "$FIX_HOST"/stc_exp*_"${VER}".dmp "$FIX_HOST"/truth_stc.txt
