#!/usr/bin/env bash
# make_ru_stats.sh
# R-016 (docs/formats/requests.md): optimizer statistics in an EXP dump. Table mode exports of R_STAT and
# R_TEXT, which have statistics (R_STAT with histograms), from the CL8MSWIN1251 database of make_ru.sh,
# with statistics=estimate / compute / none, several client character sets, both paths.
# Run make_ru.sh first (it creates the container ora11ru, the user and the data).
# Adds to corpus/fixtures/11g-cp1251/: truth_stat.txt and ru_stat_exp*_11g.dmp (+ .log), updates SHA256SUMS.
# Existing files are never overwritten (FORCE=1 recreates the R-016 files, with new statistics).
# NOTE: this script gathers statistics on R_TEXT too. Dumps of R_TEXT made BEFORE (R-015, ru_tbl_*)
# have no statistics; do not run make_ru_tables.sh with FORCE=1 afterwards if you want to keep it that way.

set -euo pipefail
VERSION="make_ru_stats.sh v1 (2026-10-07)"
echo "$VERSION"
CORPUS_DIR="${CORPUS_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
SQL_HOST="$CORPUS_DIR/sql"
FIX_HOST="$CORPUS_DIR/fixtures/11g-cp1251"
LOG_HOST="$CORPUS_DIR/work/11g-cp1251/logs"
CONTAINER="ora11ru"
SERVICE="XE"
RU_PW="Corpus_ru_1"
CONNECT_RU="corpus_ru/${RU_PW}@//localhost:1521/${SERVICE}"
mkdir -p "$LOG_HOST"
LOG="$LOG_HOST/make_ru_stats.log"
echo "=== $VERSION $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$LOG"

for f in 08_ru_stats_objects.sql 08_ru_stats_truth.sql; do
  [ -f "$SQL_HOST/$f" ] || { echo "!! $SQL_HOST/$f not found" >&2; exit 1; }
done
docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" || { echo "!! container $CONTAINER is not running. Run make_ru.sh first." >&2; exit 1; }
[ -f "$FIX_HOST/truth_ru.txt" ] || { echo "!! $FIX_HOST/truth_ru.txt is missing. Run make_ru.sh first." >&2; exit 1; }
docker exec "$CONTAINER" test -f /sql/08_ru_stats_objects.sql \
  || { echo "!! /sql/08_ru_stats_objects.sql is not visible inside $CONTAINER (it mounts another sql folder?). Run: docker rm -f $CONTAINER; then make_ru.sh with FORCE=1" >&2; exit 1; }

if [ -e "$FIX_HOST/ru_stat_exp_us7ascii_est_11g.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "R-016 fixtures already exist in $FIX_HOST; nothing to do (FORCE=1 recreates them)."
  exit 0
fi
rm -f "$FIX_HOST"/ru_stat_exp*_11g.dmp "$FIX_HOST"/ru_stat_exp*_11g.log "$FIX_HOST"/truth_stat.txt

run_sql() {  # <script>
  local log="$LOG_HOST/${1%.sql}.log" n
  echo "== running $1"
  docker exec "$CONTAINER" sqlplus -L "corpus_ru/${RU_PW}@//localhost:1521/$SERVICE" "@/sql/$1" > "$log" 2>&1 || true
  n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$log" || true)
  if [ "$n" != "0" ]; then
    echo "!! $n error line(s) in $log. Stopping before any export." >&2
    exit 1
  fi
}
run_sql 08_ru_stats_objects.sql
run_sql 08_ru_stats_truth.sql
docker exec "$CONTAINER" bash -c 'chmod a+r /fixtures/truth_stat.txt'
echo "   truth_stat.txt: $(grep -c '^COL|' "$FIX_HOST/truth_stat.txt") column rows, $(grep -c '^HIST|' "$FIX_HOST/truth_stat.txt") histogram rows"

# st_exp <stem> <NLS_LANG> <direct y|n> <statistics estimate|compute|none>
st_exp() {
  local name="$1_11g" nls="$2" direct="$3" st="$4"
  echo "   $name.dmp   (client $nls, direct=$direct, statistics=$st)"
  docker exec -e "NLS_LANG=$nls" "$CONTAINER" exp "$CONNECT_RU" tables=r_stat,r_text direct="$direct" \
      statistics="$st" recordlength=2048 file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$LOG" 2>&1 \
    || echo "     (exp reported an error for $name; the log is in the folder)"
}

for c in US7ASCII CL8MSWIN1251; do
  lc=$(tr 'A-Z' 'a-z' <<< "$c"); lc=${lc/cl8mswin1251/cp1251}
  st_exp "ru_stat_exp_${lc}_est"        "AMERICAN_AMERICA.$c" n estimate
  st_exp "ru_stat_exp_${lc}_cmp"        "AMERICAN_AMERICA.$c" n compute
  st_exp "ru_stat_exp_direct_${lc}_est" "AMERICAN_AMERICA.$c" y estimate
  st_exp "ru_stat_exp_direct_${lc}_cmp" "AMERICAN_AMERICA.$c" y compute
done
st_exp "ru_stat_exp_al32utf8_est"       "AMERICAN_AMERICA.AL32UTF8" n estimate
st_exp "ru_stat_exp_us7ascii_none"      "AMERICAN_AMERICA.US7ASCII" n none

docker exec "$CONTAINER" bash -c 'for f in /fixtures/*; do if [ -f "$f" ]; then chmod a+r "$f"; fi; done'
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )
echo
echo "R-016 fixtures are in $FIX_HOST (checksums written)."
echo "Lines to look at in the exp logs (EXP-00091 = questionable statistics):"
grep -H -E 'EXP-|ORA-|Export done|server uses|successfully|Warning' "$FIX_HOST"/ru_stat_exp*_11g.log || true
echo
ls -l "$FIX_HOST"/ru_stat_exp*.dmp
