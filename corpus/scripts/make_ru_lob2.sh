#!/usr/bin/env bash
# make_ru_lob2.sh
# R-038: LOB values (long Cyrillic CLOB, NCLOB, BLOB) in the CL8MSWIN1251 database of make_ru.sh.
# Run make_ru.sh first (it creates the container ora11ru, the user CORPUS_RU and R_TEXT).
# Adds to corpus/fixtures/11g-cp1251/: truth_lob_ru.txt, ru_lob2_exp_cp1251_11g.dmp/.log and
# ru_lob2_exp_us7ascii_11g.dmp/.log (conventional, statistics=none, tables r_lob2 and r_text), updates SHA256SUMS.
# Existing files are never overwritten (FORCE=1 overwrites only the ru_lob2_* files and the truth file).

set -euo pipefail
VERSION="make_ru_lob2.sh v1 (2026-10-17)"
echo "$VERSION"
CORPUS_DIR="${CORPUS_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
FIX_HOST="$CORPUS_DIR/fixtures/11g-cp1251"
LOG_HOST="$CORPUS_DIR/work/11g-cp1251/logs"
CONTAINER="ora11ru"
CONNECT_RU="corpus_ru/Corpus_ru_1@//localhost:1521/XE"
SQL="$CORPUS_DIR/sql/15_ru_lob2.sql"
mkdir -p "$LOG_HOST"
LOG="$LOG_HOST/make_ru_lob2.log"
echo "=== $VERSION $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" || { echo "!! container $CONTAINER is not running. Run make_ru.sh first." >&2; exit 1; }
[ -f "$FIX_HOST/truth_ru.txt" ] || { echo "!! $FIX_HOST/truth_ru.txt is missing. Run make_ru.sh first." >&2; exit 1; }
[ -f "$SQL" ] || { echo "!! $SQL is missing." >&2; exit 1; }

echo "== preflight"
docker exec -i "$CONTAINER" sqlplus -s "$CONNECT_RU" <<'Q' | tee -a "$LOG"
set feedback off pagesize 0 heading off
select 'DBCS=' || value from nls_database_parameters where parameter = 'NLS_CHARACTERSET';
select 'NCHAR=' || value from nls_database_parameters where parameter = 'NLS_NCHAR_CHARACTERSET';
select 'R_TEXT rows=' || count(*) from r_text;
exit
Q
docker exec -i "$CONTAINER" sqlplus -s "$CONNECT_RU" <<'Q' | grep -q 'DBCS=CL8MSWIN1251' || { echo "!! database character set is not CL8MSWIN1251" >&2; exit 1; }
set feedback off pagesize 0 heading off
select 'DBCS=' || value from nls_database_parameters where parameter = 'NLS_CHARACTERSET';
exit
Q

TRUTH="$FIX_HOST/truth_lob_ru.txt"
if [ -e "$TRUTH" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "   exists, skipped: truth_lob_ru.txt (table R_LOB2 is not rebuilt)"
else
  echo "== create R_LOB2 and write the truth file"
  docker exec -i -e NLS_LANG=AMERICAN_AMERICA.CL8MSWIN1251 "$CONTAINER" sqlplus -s "$CONNECT_RU" < "$SQL" > "$TRUTH.tmp" 2>> "$LOG"
  grep -q '^R_LOB2|1|' "$TRUTH.tmp" || { echo "!! truth file has no R_LOB2 rows; see $LOG and $TRUTH.tmp" >&2; exit 1; }
  mv "$TRUTH.tmp" "$TRUTH"
fi
cat "$TRUTH"

lob_exp() {   # <stem> <NLS_LANG>
  local name="$1_11g" nls="$2"
  if [ -e "$FIX_HOST/${name}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then echo "   exists, skipped: $name.dmp"; return 0; fi
  echo "   $name.dmp   (client $nls, conventional)"
  docker exec -e "NLS_LANG=$nls" "$CONTAINER" exp "$CONNECT_RU" tables=r_lob2,r_text direct=n statistics=none \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$LOG" 2>&1 \
    || echo "     (exp reported an error for $name; see the log in the folder)"
}
echo "== exports"
lob_exp ru_lob2_exp_cp1251  AMERICAN_AMERICA.CL8MSWIN1251
lob_exp ru_lob2_exp_us7ascii AMERICAN_AMERICA.US7ASCII

docker exec "$CONTAINER" bash -c 'for f in /fixtures/*; do if [ -f "$f" ]; then chmod a+r "$f"; fi; done'
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )
echo
grep -H -E 'EXP-|ORA-|Export done|successfully' "$FIX_HOST"/ru_lob2_*_11g.log || true
ls -l "$FIX_HOST"/ru_lob2_*.dmp "$TRUTH"
