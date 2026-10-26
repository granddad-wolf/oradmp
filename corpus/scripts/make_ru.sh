#!/usr/bin/env bash
# make_ru.sh (see VERSION below)
# R-015 (docs/formats/requests.md): a database whose character set is CL8MSWIN1251 holding Cyrillic
# data, exported with several client character sets (the real corporate dump comes from such a
# database exported with a US7ASCII client).
#
# It uses its OWN throwaway container (ora11ru, image gvenzl/oracle-xe:11, no host port) and its own
# fixture folder corpus/fixtures/11g-cp1251/. It touches neither ora11/ora18 nor any existing fixture.
# The XE image creates an AL32UTF8 database; this script switches the still empty database to
# CL8MSWIN1251 with the internal command ALTER DATABASE CHARACTER SET INTERNAL_USE (unsupported by
# Oracle, fine for a database that is thrown away). If the switch fails, the script stops and tells you.
#
# Required: export ORACLE_PASSWORD='...'    (any password; used to start the container and the SYSTEM user)
# FORCE=1 recreates user, data, truth and ALL dumps of this folder.

set -euo pipefail
VERSION="make_ru.sh v6 (2026-10-07)"
echo "$VERSION"
: "${ORACLE_PASSWORD:?Set ORACLE_PASSWORD, e.g. export ORACLE_PASSWORD='...'}"

# The corpus folder is the one this script lives in (corpus/scripts/..), not $HOME/oradump.
CORPUS_DIR="${CORPUS_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
SQL_HOST="$CORPUS_DIR/sql"
FIX_HOST="$CORPUS_DIR/fixtures/11g-cp1251"
LOG_HOST="$CORPUS_DIR/work/11g-cp1251/logs"
CONTAINER="ora11ru"
IMAGE="gvenzl/oracle-xe:11"
SERVICE="XE"
RU_PW="Corpus_ru_1"
CONNECT_RU="corpus_ru/${RU_PW}@//localhost:1521/${SERVICE}"

for f in 07_ru_charset.sql 07_ru_setup.sql 07_ru_objects.sql 07_ru_truth.sql; do
  [ -f "$SQL_HOST/$f" ] || { echo "!! $SQL_HOST/$f not found" >&2; exit 1; }
done
echo "corpus folder: $CORPUS_DIR"
mkdir -p "$FIX_HOST" "$LOG_HOST"
chmod 777 "$FIX_HOST" "$LOG_HOST"
LOG="$LOG_HOST/make_ru.log"
echo "=== make_ru.sh $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$LOG"

if [ -e "$FIX_HOST/ru_exp_us7ascii_11g.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "R-015 fixtures already exist in $FIX_HOST; nothing to do (FORCE=1 recreates them)."
  exit 0
fi

# ---------------------------------------------------------------- 1. container
# A container keeps its mounts for life. If it was created with a wrong folder, throw it away
# (it only holds throwaway data) and create it again.
if docker ps -a --format '{{.Names}}' | grep -qx "$CONTAINER"; then
  src=$(docker inspect -f '{{range .Mounts}}{{if eq .Destination "/sql"}}{{.Source}}{{end}}{{end}}' "$CONTAINER")
  if [ "$src" != "$SQL_HOST" ]; then
    echo "== container $CONTAINER mounts /sql from '$src', not '$SQL_HOST'; recreating it"
    docker rm -f "$CONTAINER" >> "$LOG" 2>&1
  fi
fi
if docker ps -a --format '{{.Names}}' | grep -qx "$CONTAINER" \
   && ! docker ps --format '{{.Names}}' | grep -qx "$CONTAINER"; then
  echo "== container $CONTAINER exists but is not running (an earlier run died); recreating it"
  docker rm -f "$CONTAINER" >> "$LOG" 2>&1
fi
if ! docker ps -a --format '{{.Names}}' | grep -qx "$CONTAINER"; then
  echo "== creating container $CONTAINER (first start takes a few minutes)"
  docker run -d --name "$CONTAINER" \
    -e ORACLE_PASSWORD="$ORACLE_PASSWORD" \
    -v "$FIX_HOST:/fixtures" \
    -v "$SQL_HOST:/sql:ro" \
    "$IMAGE" >> "$LOG" 2>&1
fi
docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" || docker start "$CONTAINER" >> "$LOG" 2>&1

echo "== waiting for the database (checks every 5 s, up to 15 min)"
# Ready = the image printed its own "DATABASE IS READY TO USE" line AND the instance is OPEN.
ready=0
for i in $(seq 1 180); do
  logs=$(docker logs "$CONTAINER" 2>&1 || true)
  if [[ "$logs" == *"DATABASE IS READY TO USE"* ]]; then
    out=$(docker exec "$CONTAINER" bash -c 'echo "select status from v\$instance;" | sqlplus -S "/ as sysdba"' 2>&1 || true)
    if [[ "$out" == *OPEN* ]]; then ready=1; break; fi
  fi
  if [ $((i % 6)) -eq 0 ]; then echo "   $((i * 5)) s: not ready yet"; fi
  sleep 5
done
[ "$ready" = "1" ] || { echo "!! timed out. See: docker logs --tail 60 $CONTAINER" >&2; exit 1; }
sleep 5
echo "   database is up and open"

sysql() {   # run a query as SYSDBA, print the bare value
  docker exec -i "$CONTAINER" sqlplus -S "/ as sysdba" <<< "set heading off feedback off pagesize 0
$1
exit" | tr -d ' \t\r\n'
}

# ---------------------------------------------------------------- 2. character set
cs=$(sysql "select value from nls_database_parameters where parameter='NLS_CHARACTERSET';")
echo "   database character set now: $cs"
case "$cs" in
  *[!A-Z0-9_]*|"") echo "!! unexpected answer for the character set: $cs" >&2; exit 1 ;;
esac
if [ "$cs" != "CL8MSWIN1251" ]; then
  echo "== switching the empty database to CL8MSWIN1251"
  docker exec "$CONTAINER" test -f /sql/07_ru_charset.sql \
    || { echo "!! /sql/07_ru_charset.sql is not visible inside $CONTAINER" >&2; exit 1; }
  docker exec "$CONTAINER" sqlplus -S "/ as sysdba" @/sql/07_ru_charset.sql >> "$LOG" 2>&1 || true
  sleep 10
  cs=$(sysql "select value from nls_database_parameters where parameter='NLS_CHARACTERSET';")
  echo "   database character set after the switch: $cs"
  if [ "$cs" != "CL8MSWIN1251" ]; then
    echo "---- last lines of the log:"; tail -25 "$LOG"; echo "----"
    echo "!! the switch did not work. Send me: $LOG   and the output of: docker logs --tail 40 $CONTAINER" >&2
    exit 1
  fi
fi

# ---------------------------------------------------------------- 3. user, data, truth
run_sql() {  # <user> <password> <script>
  local log="$LOG_HOST/${3%.sql}.log" n
  echo "== running $3"
  docker exec "$CONTAINER" sqlplus -L "$1/$2@//localhost:1521/$SERVICE" "@/sql/$3" > "$log" 2>&1 || true
  n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$log" || true)
  if [ "$n" != "0" ]; then
    echo "!! $n error line(s) in $log. Stopping before any export." >&2
    exit 1
  fi
}
run_sql system "$ORACLE_PASSWORD" 07_ru_setup.sql
run_sql corpus_ru "$RU_PW" 07_ru_objects.sql
run_sql corpus_ru "$RU_PW" 07_ru_truth.sql

# ---------------------------------------------------------------- 4. exports
# ru_exp <stem> <NLS_LANG> <direct y|n> <exp scope args...>   (RECORDLENGTH 2048 as in the real dump)
ru_exp() {
  local stem="$1" nls="$2" direct="$3"; shift 3
  local name="${stem}_11g"
  echo "   $name.dmp   (client $nls, direct=$direct)"
  docker exec -e "NLS_LANG=$nls" "$CONTAINER" exp "$CONNECT_RU" "$@" direct="$direct" recordlength=2048 \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$LOG" 2>&1 \
    || echo "     (exp reported an error for $name; the log is in the folder, send it to me)"
}

echo "== owner mode, conventional path, four client character sets"
ru_exp ru_exp_us7ascii  AMERICAN_AMERICA.US7ASCII      n owner=corpus_ru
ru_exp ru_exp_cp1251    AMERICAN_AMERICA.CL8MSWIN1251  n owner=corpus_ru
ru_exp ru_exp_al32utf8  AMERICAN_AMERICA.AL32UTF8      n owner=corpus_ru
ru_exp ru_exp_w1252     AMERICAN_AMERICA.WE8MSWIN1252  n owner=corpus_ru
echo "== owner mode, direct path"
ru_exp ru_exp_direct_us7ascii AMERICAN_AMERICA.US7ASCII     y owner=corpus_ru
ru_exp ru_exp_direct_cp1251   AMERICAN_AMERICA.CL8MSWIN1251 y owner=corpus_ru
echo "== table mode like the real dump (US7ASCII client, conventional)"
ru_exp ru_tables_exp_us7ascii AMERICAN_AMERICA.US7ASCII n tables=r_text,r_clob

docker exec "$CONTAINER" bash -c 'for f in /fixtures/*; do if [ -f "$f" ]; then chmod a+r "$f"; fi; done'
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "R-015 fixtures are in $FIX_HOST (checksums written)."
echo "Lines to look at in the exp logs:"
grep -H -E 'EXP-|ORA-|Export done|server uses|successfully|Warning' "$FIX_HOST"/ru_*_11g.log || true
echo
ls -l "$FIX_HOST"
echo
head -4 "$FIX_HOST/truth_ru.txt"
