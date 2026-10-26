#!/usr/bin/env bash
# make_own.sh <18c|11g>
# R-012 (docs/formats/requests.md): a small owner-mode fixture with every statement kind a reader
# must skip. Adds to corpus/fixtures/<ver>/:
#   own_exp_<ver>.dmp, own_exp_direct_<ver>.dmp              owner=corpus_own, conventional / direct
#   own_tables_exp_<ver>.dmp, own_tables_exp_direct_<ver>.dmp  the same three tables in table mode
#   own_exp_w1252_<ver>.dmp                                   owner mode, WE8MSWIN1252 client
#   their logs, truth_own.txt, truth_own_ddl.txt
# It never touches existing fixtures:
#   1. it verifies SHA256SUMS first and stops if an existing fixture has changed;
#   2. it does nothing if own_exp_<ver>.dmp already exists (FORCE=1 recreates ALL own_* files together:
#      the names of system-generated constraints change on every run, so dumps and truth must match);
#   3. it rewrites SHA256SUMS at the end.
# It stops before exporting if creating the objects or the truth files reports an error.
#
# Required: export ORACLE_PASSWORD='...'   (SYSTEM password, needed to create the user CORPUS_OWN)

set -euo pipefail
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
need_sys_password

OWN_PW="Corpus_own_1"                      # must match 05_own_setup.sql
CONNECT_OWN="corpus_own/${OWN_PW}@//localhost:1521/${SERVICE}"

mkdir -p "$WORK_HOST/logs"
OWN_LOG="$WORK_HOST/logs/own_${VER}.log"
echo "=== make_own.sh ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$OWN_LOG"

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
if [ -e "$FIX_HOST/own_exp_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "own_* fixtures for $VER already exist; nothing to do."
  echo "FORCE=1 recreates all of them (dumps and truth files together)."
  exit 0
fi

# stop if a SQL*Plus log holds errors (also PL/SQL objects created with compilation errors)
check_log() {
  local log="$WORK_HOST/logs/${1%.sql}.log" n
  n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$log" || true)
  if [ "$n" != "0" ]; then
    echo "!! $n error line(s) in $log. Stopping before any export; nothing was added to the fixtures." >&2
    echo "   Send me that log; I will adjust R-012." >&2
    exit 1
  fi
}

# 3. user, objects, truth
run_sql system "$ORACLE_PASSWORD" 05_own_setup.sql;  check_log 05_own_setup.sql
run_sql corpus_own "$OWN_PW" 05_own_objects.sql;     check_log 05_own_objects.sql
run_sql corpus_own "$OWN_PW" 05_own_truth.sql;       check_log 05_own_truth.sql

# 4. exports. own_exp <file_stem> <NLS_LANG or -> <direct y|n> <exp scope args...>
own_exp() {
  local stem="$1" nls="$2" direct="$3"; shift 3
  local name="${stem}_${VER}"
  local envargs=()
  if [ "$nls" != "-" ]; then
    envargs=(-e "NLS_LANG=$nls")
  fi
  echo "   $name.dmp"
  docker exec "${envargs[@]}" "$CONTAINER" exp "$CONNECT_OWN" "$@" direct="$direct" \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" \
      >> "$OWN_LOG" 2>&1 \
    || echo "   !! exp failed for ${name} (see $OWN_LOG)"
}

echo "== owner mode"
own_exp own_exp               -                              n owner=corpus_own
own_exp own_exp_direct        -                              y owner=corpus_own

echo "== the same three tables in table mode"
own_exp own_tables_exp        -                              n tables=o_parent,o_child,o_empty
own_exp own_tables_exp_direct -                              y tables=o_parent,o_child,o_empty

echo "== owner mode, WE8MSWIN1252 client (are the comments converted?)"
own_exp own_exp_w1252         "AMERICAN_AMERICA.WE8MSWIN1252" n owner=corpus_own

make_files_readable

( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "Owner-mode fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Export output: $OWN_LOG"
echo "Lines to look at (the exports should end 'successfully'; EXP-00091 'exporting questionable statistics' is harmless):"
grep -H -E 'EXP-|ORA-|successfully|Warning' "$FIX_HOST"/own_*_"${VER}".log || true
echo
ls -l "$FIX_HOST"/own_*_"${VER}".dmp "$FIX_HOST"/truth_own*.txt
