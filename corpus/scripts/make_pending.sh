#!/usr/bin/env bash
# make_pending.sh <18c|11g>
# Requests R-017 (18c only), R-040 and R-047 (both versions) from docs/formats/requests.md.
# Adds to corpus/fixtures/<ver>/ (statistics=none):
#   R-047 tsp_exp_<v>.dmp, tsp_exp_direct_<v>.dmp                      truth_tsp.txt
#   R-040 long3_exp_<v>.dmp, long3_exp_direct_<v>.dmp, long3_sqlldr_<v>.log   truth_long3.txt
#   R-017 (18c only) seg_exp_18c.dmp, seg_exp_direct_18c.dmp           truth_seg.txt
# Each request is a separate step: if it fails, it is reported and skipped, the others go on.
# ONLY="R-047 R-040" runs just those steps. A step whose first dump exists is skipped (FORCE=1 recreates it).
# Verifies SHA256SUMS first and rewrites it at the end (like make_extra.sh). Also packs the new files into
# corpus/work/<ver>/pending_<ver>.tar.gz (scratch). Save next to make_extra.sh; the four 16_*.sql files go to corpus/sql/.

set -uo pipefail
VERSION="make_pending.sh v1 (2026-10-25)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
PL_LOG="$WORK_HOST/logs/pending_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$PL_LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" \
  || { echo "!! container $CONTAINER is not running (docker start $CONTAINER)" >&2; exit 1; }

for f in 16_seg.sql 16_tsp.sql 16_long3_pre.sql 16_long3_post.sql; do
  [ -f "$SQL_HOST/$f" ] || { echo "!! $SQL_HOST/$f is missing" >&2; exit 1; }
done

if [ -f "$FIX_HOST/SHA256SUMS" ]; then
  echo "== verifying existing fixtures"
  if ! ( cd "$FIX_HOST" && sha256sum -c --quiet SHA256SUMS ); then
    echo "!! an existing fixture differs from SHA256SUMS. Stopping." >&2
    exit 1
  fi
  echo "   ok"
fi

OK_STEPS=""; FAILED_STEPS=""; SKIPPED_STEPS=""; NEW_FILES=()

wanted() { [ -z "${ONLY:-}" ] || [[ " $ONLY " == *" $1 "* ]]; }

# already_done <first dump stem>: true if the step should be skipped
already_done() {
  [ -e "$FIX_HOST/${1}_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]
}

# sql_ok <name> <sql> [truth]  -> runs the SQL, returns 0 if no error lines and the truth file (if named) is not empty
sql_ok() {
  local name="$1" sql="$2" truth="${3:-}" log n
  echo "== $name  ($sql)"
  run_sql corpus "$CORPUS_PW" "$sql" || true
  log="$WORK_HOST/logs/${sql%.sql}.log"
  n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$log" 2>/dev/null || true)
  if [ "${n:-0}" != "0" ] || { [ -n "$truth" ] && [ ! -s "$FIX_HOST/$truth" ]; }; then
    echo "   !! $name: ${n:-?} error line(s) in $log${truth:+, truth file $truth $( [ -s "$FIX_HOST/$truth" ] && echo present || echo missing )}"
    grep -E 'ORA-|SP2-|PLS-' "$log" 2>/dev/null | head -3 | sed 's/^/      /'
    return 1
  fi
  [ -n "$truth" ] && echo "   truth: $truth, $(grep -c . "$FIX_HOST/$truth") lines"
  return 0
}

# t_exp <file stem> <tables> [extra exp args]
t_exp() {
  local stem="$1" tables="$2"; shift 2
  local name="${stem}_${VER}"
  echo "   $name.dmp   (tables=$tables $*)"
  docker exec "$CONTAINER" exp "$CONNECT_CORPUS" tables="$tables" "$@" \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$PL_LOG" 2>&1 \
    || echo "   !! exp reported an error for ${name} (see the log in the folder)"
  NEW_FILES+=("${name}.dmp" "${name}.log")
}

# ---------------------------------------------------------------- R-047
step_r047() {
  if ! sql_ok R-047 16_tsp.sql truth_tsp.txt; then FAILED_STEPS="$FAILED_STEPS R-047"; return; fi
  NEW_FILES+=(truth_tsp.txt)
  t_exp tsp_exp        t_tsp statistics=none
  t_exp tsp_exp_direct t_tsp statistics=none direct=y
  OK_STEPS="$OK_STEPS R-047"
  echo "   truth_tsp.txt:"; sed 's/^/      /' "$FIX_HOST/truth_tsp.txt"
}

# ---------------------------------------------------------------- R-017 (18c only)
step_r017() {
  if ! sql_ok R-017 16_seg.sql truth_seg.txt; then FAILED_STEPS="$FAILED_STEPS R-017"; return; fi
  NEW_FILES+=(truth_seg.txt)
  t_exp seg_exp        t_seg0,t_seg1,t_seg2 statistics=none
  t_exp seg_exp_direct t_seg0,t_seg1,t_seg2 statistics=none direct=y
  OK_STEPS="$OK_STEPS R-017"
  echo "   truth_seg.txt:"; sed 's/^/      /' "$FIX_HOST/truth_seg.txt"
}

# ---------------------------------------------------------------- R-040
step_r040() {
  echo "== R-040  (route 1: SQL*Loader inside $CONTAINER)"
  if ! docker exec "$CONTAINER" bash -c 'test -x "$ORACLE_HOME/bin/sqlldr"'; then
    echo "   !! sqlldr is missing in $CONTAINER; R-040 route 2 (python-oracledb) is not automated here. Skipped."
    SKIPPED_STEPS="$SKIPPED_STEPS R-040"; return
  fi
  if ! sql_ok R-040/pre 16_long3_pre.sql; then FAILED_STEPS="$FAILED_STEPS R-040"; return; fi

  # data file on the host (scratch, mounted as /work): id|L<6-digit size><y x (size-7)>|v<size>
  local dat="$WORK_HOST/long3b.dat" ctl="$WORK_HOST/long3.ctl" sizes=(32767 32768 65519 65520 65535 65536 100000) i=0 n
  : > "$dat"
  for n in "${sizes[@]}"; do
    i=$((i + 1))
    { printf '%d|L%06d' "$i" "$n"; head -c $((n - 7)) /dev/zero | tr '\0' 'y'; printf '|v%d\n' "$n"; } >> "$dat"
  done
  local got want
  got=$(LC_ALL=C awk -F'|' '{printf "%s%d", (NR>1?",":""), length($2)}' "$dat")
  want=$(IFS=,; echo "${sizes[*]}")
  if [ "$got" != "$want" ]; then
    echo "   !! the data file has wrong value sizes: $got (wanted $want)"; FAILED_STEPS="$FAILED_STEPS R-040"; return
  fi
  cat > "$ctl" <<CTL
LOAD DATA
INFILE '/work/long3b.dat'
APPEND INTO TABLE t_long3
FIELDS TERMINATED BY '|'
(id INTEGER EXTERNAL, l CHAR(100000), v CHAR(10))
CTL
  echo "   running sqlldr (7 rows, up to 100,000 bytes)"
  docker exec "$CONTAINER" sqlldr userid="$CONNECT_CORPUS" control=/work/long3.ctl \
      log="/fixtures/long3_sqlldr_${VER}.log" readsize=1048576 bindsize=1048576 rows=1 >> "$PL_LOG" 2>&1 \
    || echo "   (sqlldr returned a non-zero status; read the log)"
  NEW_FILES+=("long3_sqlldr_${VER}.log")
  make_files_readable
  echo "   sqlldr log lines:"
  grep -E 'SQL\*Loader-|ORA-|Rows successfully loaded|Rows not loaded' "$FIX_HOST/long3_sqlldr_${VER}.log" 2>/dev/null | sed 's/^/      /'
  grep -qE '^ *7 Rows? successfully loaded' "$FIX_HOST/long3_sqlldr_${VER}.log" 2>/dev/null \
    || echo "   !! not all 7 rows were loaded: the dumps below still show what Oracle accepted (that is also an answer)"

  if ! sql_ok R-040/truth 16_long3_post.sql truth_long3.txt; then FAILED_STEPS="$FAILED_STEPS R-040"; return; fi
  NEW_FILES+=(truth_long3.txt)
  t_exp long3_exp        t_long3,t_frame statistics=none
  t_exp long3_exp_direct t_long3,t_frame statistics=none direct=y
  OK_STEPS="$OK_STEPS R-040"
  echo "   truth_long3.txt (rows 6 and 7 must show 65536 and 100000):"
  cut -c1-120 "$FIX_HOST/truth_long3.txt" | sed 's/^/      /'
}

if wanted R-047; then
  if already_done tsp_exp; then SKIPPED_STEPS="$SKIPPED_STEPS R-047(exists)"; else step_r047; fi
fi
if wanted R-040; then
  if already_done long3_exp; then SKIPPED_STEPS="$SKIPPED_STEPS R-040(exists)"; else step_r040; fi
fi
if [ "$VER" = "18c" ] && wanted R-017; then
  if already_done seg_exp; then SKIPPED_STEPS="$SKIPPED_STEPS R-017(exists)"; else step_r017; fi
fi

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

if [ "${#NEW_FILES[@]}" -gt 0 ]; then
  ( cd "$FIX_HOST" && tar czf "$WORK_HOST/pending_${VER}.tar.gz" "${NEW_FILES[@]}" ) 2>/dev/null \
    && echo "Packed: $WORK_HOST/pending_${VER}.tar.gz"
fi

echo
echo "Steps ok:      ${OK_STEPS:- none}"
echo "Steps failed:  ${FAILED_STEPS:- none}"
echo "Steps skipped: ${SKIPPED_STEPS:- none}"
echo "Lines to look at in the export logs:"
for f in "${NEW_FILES[@]}"; do
  case "$f" in *.log) grep -H -E 'EXP-|ORA-|Export done|successfully|Warning|conventional|rows exported' "$FIX_HOST/$f" ;; esac
done 2>/dev/null || true
echo
ls -l "${NEW_FILES[@]/#/$FIX_HOST/}" 2>/dev/null || true
