#!/usr/bin/env bash
# make_pending2.sh <18c|11g>
# Requests R-048 (both versions), R-049 (11g only) and R-050 (both) from docs/formats/requests.md.
# Adds to corpus/fixtures/<ver>/ (statistics=none):
#   R-050 tsltz2_exp_<v>.dmp, tsltz2_exp_direct_<v>.dmp                        truth_tsltz2.txt
#   R-048 long4_exp_<v>.dmp, long4_exp_direct_<v>.dmp, long4_sqlldr_<v>.log     truth_long4.txt
#         (18c only, if python-oracledb installs:) longraw4_exp_<v>.dmp, longraw4_exp_direct_<v>.dmp   truth_longraw4.txt
#   R-049 (11g only) seg_exp_11g.dmp, seg_exp_direct_11g.dmp                   truth_seg.txt   (uses 16_seg.sql)
# Each request is a separate step. ONLY="R-050 R-048" runs just those; FORCE=1 recreates; NO_LONGRAW=1 skips the LONG RAW part.
# LONG RAW: a venv with python-oracledb (thin mode) is made in ${TMPDIR:-/var/tmp}/oradmp-oravenv (needs network for pip);
# thin mode does not talk to 11g, so LONG RAW runs on 18c only. Verifies SHA256SUMS first and rewrites it at the end.
# Save next to make_extra.sh; the 17_*.sql files go to corpus/sql/ (16_seg.sql must be there already).

set -uo pipefail
VERSION="make_pending2.sh v1 (2026-10-25)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
PL_LOG="$WORK_HOST/logs/pending2_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$PL_LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" \
  || { echo "!! container $CONTAINER is not running (docker start $CONTAINER)" >&2; exit 1; }
for f in 16_seg.sql 17_long4_pre.sql 17_long4_post.sql 17_longraw4_pre.sql 17_longraw4_post.sql 17_tsltz2.sql; do
  [ -f "$SQL_HOST/$f" ] || { echo "!! $SQL_HOST/$f is missing" >&2; exit 1; }
done

if [ -f "$FIX_HOST/SHA256SUMS" ]; then
  echo "== verifying existing fixtures"
  ( cd "$FIX_HOST" && sha256sum -c --quiet SHA256SUMS ) || { echo "!! an existing fixture differs from SHA256SUMS. Stopping." >&2; exit 1; }
  echo "   ok"
fi

OK_STEPS=""; FAILED_STEPS=""; SKIPPED_STEPS=""; NEW_FILES=()
wanted() { [ -z "${ONLY:-}" ] || [[ " $ONLY " == *" $1 "* ]]; }
already_done() { [ -e "$FIX_HOST/${1}_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]; }

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

t_exp() {
  local stem="$1" tables="$2"; shift 2
  local name="${stem}_${VER}"
  echo "   $name.dmp   (tables=$tables $*)"
  docker exec "$CONTAINER" exp "$CONNECT_CORPUS" tables="$tables" "$@" \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$PL_LOG" 2>&1 \
    || echo "   !! exp reported an error for ${name} (see the log in the folder)"
  NEW_FILES+=("${name}.dmp" "${name}.log")
}

# ---------------------------------------------------------------- R-050
step_r050() {
  { echo "--- container time zone settings at the run:"; docker exec "$CONTAINER" bash -c 'echo "TZ=$TZ ORA_SDTZ=$ORA_SDTZ"; date'; } >> "$PL_LOG" 2>&1
  if ! sql_ok R-050 17_tsltz2.sql truth_tsltz2.txt; then FAILED_STEPS="$FAILED_STEPS R-050"; return; fi
  NEW_FILES+=(truth_tsltz2.txt)
  t_exp tsltz2_exp        t_tsltz2 statistics=none
  t_exp tsltz2_exp_direct t_tsltz2 statistics=none direct=y
  OK_STEPS="$OK_STEPS R-050"
  echo "   truth_tsltz2.txt:"; cut -c1-200 "$FIX_HOST/truth_tsltz2.txt" | sed 's/^/      /'
}

# ---------------------------------------------------------------- R-049 (11g only; the SQL is that of R-017)
step_r049() {
  if ! sql_ok R-049 16_seg.sql truth_seg.txt; then FAILED_STEPS="$FAILED_STEPS R-049"; return; fi
  NEW_FILES+=(truth_seg.txt)
  t_exp seg_exp        t_seg0,t_seg1,t_seg2 statistics=none
  t_exp seg_exp_direct t_seg0,t_seg1,t_seg2 statistics=none direct=y
  OK_STEPS="$OK_STEPS R-049"
  echo "   truth_seg.txt:"; sed 's/^/      /' "$FIX_HOST/truth_seg.txt"
}

# ---------------------------------------------------------------- R-048
step_r048() {
  echo "== R-048  (LONG by SQL*Loader inside $CONTAINER)"
  if ! docker exec "$CONTAINER" bash -c 'test -x "$ORACLE_HOME/bin/sqlldr"'; then
    echo "   !! sqlldr is missing in $CONTAINER. Skipped."; SKIPPED_STEPS="$SKIPPED_STEPS R-048"; return
  fi
  if ! sql_ok R-048/pre 17_long4_pre.sql; then FAILED_STEPS="$FAILED_STEPS R-048"; return; fi

  local dat="$WORK_HOST/long4b.dat" ctl="$WORK_HOST/long4.ctl" sizes=(65521 65525 65530 65532 65533 65534 65535) i=0 n got want
  : > "$dat"
  for n in "${sizes[@]}"; do
    i=$((i + 1))
    { printf '%d|L%06d' "$i" "$n"; head -c $((n - 7)) /dev/zero | tr '\0' 'y'; printf '|v%d\n' "$n"; } >> "$dat"
  done
  got=$(LC_ALL=C awk -F'|' '{printf "%s%d", (NR>1?",":""), length($2)}' "$dat")
  want=$(IFS=,; echo "${sizes[*]}")
  if [ "$got" != "$want" ]; then
    echo "   !! the data file has wrong value sizes: $got (wanted $want)"; FAILED_STEPS="$FAILED_STEPS R-048"; return
  fi
  cat > "$ctl" <<CTL
LOAD DATA
INFILE '/work/long4b.dat'
APPEND INTO TABLE t_long4
FIELDS TERMINATED BY '|'
(id INTEGER EXTERNAL, l CHAR(100000), v CHAR(10))
CTL
  echo "   running sqlldr (7 rows, 65,521 to 65,535 bytes)"
  docker exec "$CONTAINER" sqlldr userid="$CONNECT_CORPUS" control=/work/long4.ctl \
      log="/fixtures/long4_sqlldr_${VER}.log" readsize=1048576 bindsize=1048576 rows=1 >> "$PL_LOG" 2>&1 \
    || echo "   (sqlldr returned a non-zero status; read the log)"
  NEW_FILES+=("long4_sqlldr_${VER}.log")
  make_files_readable
  grep -E 'SQL\*Loader-|ORA-|Rows successfully loaded|Rows not loaded' "$FIX_HOST/long4_sqlldr_${VER}.log" 2>/dev/null | sed 's/^/      /'
  grep -qE '^ *7 Rows? successfully loaded' "$FIX_HOST/long4_sqlldr_${VER}.log" 2>/dev/null \
    || echo "   !! not all 7 rows were loaded: the dumps still show what Oracle accepted (that is also an answer)"
  if ! sql_ok R-048/truth 17_long4_post.sql truth_long4.txt; then FAILED_STEPS="$FAILED_STEPS R-048"; return; fi
  NEW_FILES+=(truth_long4.txt)
  t_exp long4_exp        t_long4,t_frame statistics=none
  t_exp long4_exp_direct t_long4,t_frame statistics=none direct=y
  OK_STEPS="$OK_STEPS R-048(LONG)"
  echo "   truth_long4.txt:"; cut -c1-120 "$FIX_HOST/truth_long4.txt" | sed 's/^/      /'

  # ---- LONG RAW part (18c only, python-oracledb thin mode)
  if [ "${NO_LONGRAW:-0}" = "1" ]; then SKIPPED_STEPS="$SKIPPED_STEPS R-048(LONG-RAW:NO_LONGRAW)"; return; fi
  if [ "$VER" != "18c" ]; then
    echo "   LONG RAW: skipped on $VER (python-oracledb thin mode needs 12.1 or later; the request allows leaving it out)"
    SKIPPED_STEPS="$SKIPPED_STEPS R-048(LONG-RAW:11g)"; return
  fi
  local venv="${TMPDIR:-/var/tmp}/oradmp-oravenv"
  if [ ! -x "$venv/bin/python" ]; then
    echo "   making $venv with python-oracledb (pip, needs network)"
    python3 -m venv "$venv" >> "$PL_LOG" 2>&1 && "$venv/bin/pip" install -q oracledb >> "$PL_LOG" 2>&1 \
      || { echo "   !! could not install python-oracledb; LONG RAW skipped"; rm -rf "$venv"; SKIPPED_STEPS="$SKIPPED_STEPS R-048(LONG-RAW:no-oracledb)"; return; }
  fi
  if ! sql_ok R-048/LONGRAW-pre 17_longraw4_pre.sql; then FAILED_STEPS="$FAILED_STEPS R-048(LONG-RAW)"; return; fi
  cat > "$WORK_HOST/longraw4_load.py" <<'PY'
import os, sys
import oracledb
con = oracledb.connect(user="corpus", password=os.environ["CORPUS_PW"], dsn=os.environ["DSN"])
cur = con.cursor()
rows = [(1, 32768), (2, 65520), (3, 65535), (4, 100000)]
for i, n in rows:
    try:
        cur.setinputsizes(None, oracledb.DB_TYPE_LONG_RAW)
        cur.execute("INSERT INTO t_longraw4 VALUES (:1, :2)", [i, bytes([0xAB]) * n])
        print("row", i, n, "inserted")
    except Exception as e:
        print("row", i, n, "REFUSED:", str(e).replace("\n", " "))
con.commit()
PY
  CORPUS_PW="$CORPUS_PW" DSN="localhost:${HOST_PORT}/${SERVICE}" "$venv/bin/python" -I "$WORK_HOST/longraw4_load.py" 2>&1 | sed 's/^/      /' | tee -a "$PL_LOG"
  if ! sql_ok R-048/LONGRAW-truth 17_longraw4_post.sql truth_longraw4.txt; then FAILED_STEPS="$FAILED_STEPS R-048(LONG-RAW)"; return; fi
  NEW_FILES+=(truth_longraw4.txt)
  t_exp longraw4_exp        t_longraw4,t_frame statistics=none
  t_exp longraw4_exp_direct t_longraw4,t_frame statistics=none direct=y
  OK_STEPS="$OK_STEPS R-048(LONG-RAW)"
  echo "   truth_longraw4.txt:"; sed 's/^/      /' "$FIX_HOST/truth_longraw4.txt"
}

wanted R-050 && { if already_done tsltz2_exp; then SKIPPED_STEPS="$SKIPPED_STEPS R-050(exists)"; else step_r050; fi; }
wanted R-048 && { if already_done long4_exp;  then SKIPPED_STEPS="$SKIPPED_STEPS R-048(exists)"; else step_r048; fi; }
if [ "$VER" = "11g" ] && wanted R-049; then
  if already_done seg_exp; then SKIPPED_STEPS="$SKIPPED_STEPS R-049(exists)"; else step_r049; fi
fi

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )
if [ "${#NEW_FILES[@]}" -gt 0 ]; then
  ( cd "$FIX_HOST" && tar czf "$WORK_HOST/pending2_${VER}.tar.gz" "${NEW_FILES[@]}" ) 2>/dev/null \
    && echo "Packed: $WORK_HOST/pending2_${VER}.tar.gz"
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
