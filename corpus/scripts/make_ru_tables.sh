#!/usr/bin/env bash
# make_ru_tables.sh
# R-015b: table-mode exports of R_TEXT (no CLOB, so a walker can read the whole file) from the
# CL8MSWIN1251 database of make_ru.sh, for four client character sets on both paths.
# Run make_ru.sh first (it creates the container ora11ru, the user and the data).
# Adds corpus/fixtures/11g-cp1251/ru_tbl_exp[_direct]_<client>_11g.dmp (+ .log) and updates SHA256SUMS.
# Existing files are never overwritten (FORCE=1 overwrites only the ru_tbl_* files).

set -euo pipefail
VERSION="make_ru_tables.sh v1 (2026-10-07)"
echo "$VERSION"
CORPUS_DIR="${CORPUS_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
FIX_HOST="$CORPUS_DIR/fixtures/11g-cp1251"
LOG_HOST="$CORPUS_DIR/work/11g-cp1251/logs"
CONTAINER="ora11ru"
CONNECT_RU="corpus_ru/Corpus_ru_1@//localhost:1521/XE"
mkdir -p "$LOG_HOST"
LOG="$LOG_HOST/make_ru_tables.log"
echo "=== $VERSION $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" || { echo "!! container $CONTAINER is not running. Run make_ru.sh first." >&2; exit 1; }
[ -f "$FIX_HOST/truth_ru.txt" ] || { echo "!! $FIX_HOST/truth_ru.txt is missing. Run make_ru.sh first." >&2; exit 1; }

tbl_exp() {   # <stem> <NLS_LANG> <direct y|n>
  local name="$1_11g" nls="$2" direct="$3"
  if [ -e "$FIX_HOST/${name}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then echo "   exists, skipped: $name.dmp"; return 0; fi
  echo "   $name.dmp   (client $nls, direct=$direct)"
  docker exec -e "NLS_LANG=$nls" "$CONTAINER" exp "$CONNECT_RU" tables=r_text direct="$direct" recordlength=2048 \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$LOG" 2>&1 \
    || echo "     (exp reported an error for $name; see the log in the folder)"
}

for c in US7ASCII CL8MSWIN1251 AL32UTF8 WE8MSWIN1252; do
  lc=$(tr 'A-Z' 'a-z' <<< "$c"); lc=${lc/cl8mswin1251/cp1251}; lc=${lc/we8mswin1252/w1252}
  tbl_exp "ru_tbl_exp_${lc}"         "AMERICAN_AMERICA.$c" n
  tbl_exp "ru_tbl_exp_direct_${lc}"  "AMERICAN_AMERICA.$c" y
done

docker exec "$CONTAINER" bash -c 'for f in /fixtures/*; do if [ -f "$f" ]; then chmod a+r "$f"; fi; done'
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )
echo
grep -H -E 'EXP-|ORA-|Export done|successfully' "$FIX_HOST"/ru_tbl_*_11g.log || true
ls -l "$FIX_HOST"/ru_tbl_*.dmp
