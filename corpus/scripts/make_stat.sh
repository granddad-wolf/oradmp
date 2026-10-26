#!/usr/bin/env bash
# make_stat.sh <18c|11g>
# R-016 part a (docs/formats/requests.md): tables with optimizer statistics (ANALSTATS T?, ANALCOMPUTE T?).
# Adds to corpus/fixtures/<ver>/:
#   stat_exp_<ver>.dmp                 tables=t_stat1..t_stat4, conventional, default client
#   stat_exp_direct_<ver>.dmp          direct path
#   stat_exp_us7ascii_<ver>.dmp        US7ASCII client (expected: EXP-00091 and `T R`)
#   stat_exp_none_<ver>.dmp            statistics=none (control)
#   stat_exp_compute_<ver>.dmp         statistics=compute
#   their logs, truth_stat.txt
# It never touches existing fixtures:
#   1. it verifies SHA256SUMS first and stops if an existing fixture has changed;
#   2. it does nothing if stat_exp_<ver>.dmp already exists (FORCE=1 recreates all stat_* files and
#      truth_stat.txt together: the statistics and the names of system-generated indexes change per run);
#   3. it rewrites SHA256SUMS at the end.
# It stops before exporting if creating the tables or the truth file reports an error.

set -euo pipefail
VERSION="make_stat.sh v1 (2026-10-07)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
ST_LOG="$WORK_HOST/logs/stat_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$ST_LOG"

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
if [ -e "$FIX_HOST/stat_exp_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "stat_* fixtures for $VER already exist; nothing to do."
  echo "FORCE=1 recreates all of them (dumps and truth file together)."
  exit 0
fi
rm -f "$FIX_HOST"/stat_exp*_"${VER}".dmp "$FIX_HOST"/stat_exp*_"${VER}".log "$FIX_HOST"/truth_stat.txt

# 3. tables, statistics, truth
run_sql corpus "$CORPUS_PW" 09_stat.sql
n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$WORK_HOST/logs/09_stat.log" || true)
if [ "$n" != "0" ]; then
  echo "!! $n error line(s) in $WORK_HOST/logs/09_stat.log. Stopping before any export." >&2
  exit 1
fi
[ -s "$FIX_HOST/truth_stat.txt" ] || { echo "!! truth_stat.txt was not written" >&2; exit 1; }
echo "   truth_stat.txt: $(grep -c '^COL|' "$FIX_HOST/truth_stat.txt") column rows, $(grep -c '^HIST|' "$FIX_HOST/truth_stat.txt") histogram rows"

# 4. exports. st_exp <file_stem> <NLS_LANG or -> <direct y|n> <statistics estimate|compute|none>
st_exp() {
  local stem="$1" nls="$2" direct="$3" st="$4"
  local name="${stem}_${VER}"
  local envargs=()
  if [ "$nls" != "-" ]; then
    envargs=(-e "NLS_LANG=$nls")
  fi
  echo "   $name.dmp   (client ${nls}, direct=$direct, statistics=$st)"
  docker exec "${envargs[@]}" "$CONTAINER" exp "$CONNECT_CORPUS" tables=t_stat1,t_stat2,t_stat3,t_stat4 \
      direct="$direct" statistics="$st" file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" \
      >> "$ST_LOG" 2>&1 \
    || echo "   !! exp reported an error for ${name} (see the log in the folder)"
}

st_exp stat_exp          -                                 n estimate
st_exp stat_exp_direct   -                                 y estimate
st_exp stat_exp_us7ascii "AMERICAN_AMERICA.US7ASCII"       n estimate
st_exp stat_exp_none     -                                 n none
st_exp stat_exp_compute  -                                 n compute

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "R-016 part a fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Lines to look at (EXP-00091 = questionable statistics; expected only in the US7ASCII log):"
grep -H -E 'EXP-|ORA-|Export done|successfully|Warning' "$FIX_HOST"/stat_exp*_"${VER}".log || true
echo
ls -l "$FIX_HOST"/stat_exp*_"${VER}".dmp "$FIX_HOST"/truth_stat.txt
