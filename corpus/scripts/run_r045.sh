#!/usr/bin/env bash
# run_r045.sh  (R-045)
# Runs corpus/work/18c/045.sql on the 18c container and writes the output, with every statement echoed, to
# corpus/work/18c/r045_18c.txt. ORA- errors in the output are the ANSWER (Oracle refusing a partitioned table
# with a LONG column), not a failure of the script. Afterwards it lists what was created and drops it.
# Uses the container and the corpus connect string of corpus/scripts/config.sh.

set -euo pipefail
VERSION="run_r045.sh v1 (2026-10-17)"
echo "$VERSION"
CORPUS_DIR="${CORPUS_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
SRC="$CORPUS_DIR/work/18c/045.sql"
OUT="$CORPUS_DIR/work/18c/r045_18c.txt"
[ -f "$SRC" ] || { echo "!! $SRC is missing" >&2; exit 1; }

# config.sh: take CONTAINER and CONNECT_CORPUS for 18c from the project config. It is run in a SUBSHELL, so an
# "exit" inside it cannot end this script; the variables are printed back.
CFG="$CORPUS_DIR/scripts/config.sh"
[ -f "$CFG" ] || { echo "!! $CFG is missing" >&2; exit 1; }
get_cfg() {   # <args...> : prints "CONTAINER|CONNECT_CORPUS" as config.sh sets them
  ( set -- "$@"; V=18c; VERSION=18c; source "$CFG" >/dev/null 2>&1 || true
    printf '%s|%s' "${CONTAINER:-}" "${CONNECT_CORPUS:-}" ) 2>/dev/null || true
}
CFGOUT="$(get_cfg 18c)"
[ -n "${CONNECT_CORPUS:-}" ] && CFGOUT="${CONTAINER:-}|${CONNECT_CORPUS}"
[ "${CFGOUT#*|}" = "" ] && CFGOUT="$(get_cfg)"
CONTAINER="${CFGOUT%%|*}"; CONNECT_CORPUS="${CFGOUT#*|}"
CONTAINER="${CONTAINER:-ora18}"
if [ -z "$CONNECT_CORPUS" ]; then
  echo "!! config.sh did not give CONNECT_CORPUS. Lines of config.sh that matter:" >&2
  grep -n -E 'CONNECT_CORPUS|CONTAINER|\$1|\$\{1|exit|VERSION|case' "$CFG" >&2 || true
  echo "!! Set it by hand and rerun, e.g.:  CONNECT_CORPUS='user/password@//localhost:1521/XEPDB1' $0" >&2
  exit 1
fi
echo "   container: $CONTAINER"
docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" || { echo "!! container $CONTAINER is not running" >&2; exit 1; }

TMP="$(mktemp)"; trap 'rm -f "$TMP"' EXIT
{
  echo "set echo on feedback on linesize 200 pagesize 100 define off"
  echo "prompt === R-045 $(date -u +%Y-%m-%dT%H:%M:%SZ) ==="
  echo "select banner from v\$version where rownum = 1;"
  echo "whenever sqlerror continue"
  # a clean start, silent
  echo "set echo off feedback off"
  for t in T_PLONG T_PLONGRAW T_PLONG2; do
    echo "begin execute immediate 'drop table $t purge'; exception when others then null; end;"
    echo "/"
  done
  echo "set echo on feedback on"
  echo "prompt === the request ==="
  cat "$SRC"
  echo
  echo "prompt === what Oracle accepted ==="
  echo "set echo off"
  echo "select table_name, partitioned from user_tables where table_name like 'T\_PLONG%' escape '\\' order by 1;"
  echo "select table_name, column_name, data_type from user_tab_columns where table_name like 'T\_PLONG%' escape '\\' order by 1, column_id;"
  echo "set feedback off"
  for t in T_PLONG T_PLONGRAW T_PLONG2; do
    echo "begin execute immediate 'drop table $t purge'; exception when others then null; end;"
    echo "/"
  done
  echo "exit"
} > "$TMP"

docker exec -i "$CONTAINER" sqlplus -s "$CONNECT_CORPUS" < "$TMP" 2>&1 | tee "$OUT"
echo
echo "== written: $OUT"
grep -c 'ORA-' "$OUT" | sed 's/^/   ORA- lines: /'
echo "Send me $OUT."
