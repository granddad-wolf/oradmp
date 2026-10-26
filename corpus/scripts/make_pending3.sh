#!/usr/bin/env bash
# make_pending3.sh <18c|11g>
# Request R-051 (docs/formats/requests.md), parts (a), (b) and (c).
# Adds to corpus/fixtures/<ver>/ (statistics=none):
#   (a) both versions: long5_exp_<v>.dmp, long5_exp_direct_<v>.dmp, long5_sqlldr_<v>.log   truth_long5.txt
#       LONG of 65526 .. 65531 bytes by SQL*Loader (ids 1..6)
#   (b) 18c:  longraw5_exp_18c.dmp, longraw5_exp_direct_18c.dmp   truth_longraw5.txt
#       LONG RAW rows (id, bytes of 0xab): (1,65521) (2,65525) (3,65529) (4,65530) (5,65531) (6,65533) (7,65534) by python-oracledb
#   (c) 11g:  the same tables plus (8,32768) (9,65520) (10,65535) (11,100000), loaded by SQL*Loader with the value as HEX TEXT
#       in a CHAR(200000) field (untested route; if Oracle refuses, the log and the printed lines are the answer)
# Each part is a separate step. ONLY="a b" runs just those; FORCE=1 recreates; NO_LONGRAW=1 skips the LONG RAW part.
# Verifies SHA256SUMS first and rewrites it at the end. Save next to make_extra.sh; the 18_*.sql files go to corpus/sql/.

set -uo pipefail
VERSION="make_pending3.sh v2 (2026-10-25: 11g LONG RAW also by LONG VARRAW)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
PL_LOG="$WORK_HOST/logs/pending3_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$PL_LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" \
  || { echo "!! container $CONTAINER is not running (docker start $CONTAINER)" >&2; exit 1; }
for f in 18_long5_pre.sql 18_long5_post.sql 18_longraw5_pre.sql 18_longraw5_post.sql; do
  [ -f "$SQL_HOST/$f" ] || { echo "!! $SQL_HOST/$f is missing" >&2; exit 1; }
done
if [ -f "$FIX_HOST/SHA256SUMS" ]; then
  echo "== verifying existing fixtures"
  ( cd "$FIX_HOST" && sha256sum -c --quiet SHA256SUMS ) || { echo "!! an existing fixture differs from SHA256SUMS. Stopping." >&2; exit 1; }
  echo "   ok"
fi
docker exec "$CONTAINER" bash -c 'test -x "$ORACLE_HOME/bin/sqlldr"' \
  || { echo "!! sqlldr is missing in $CONTAINER" >&2; exit 1; }

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
    echo "   !! $name: ${n:-?} error line(s) in $log${truth:+, truth file $truth $( [ -s "$FIX_HOST/$truth" ] && echo present || echo "missing (an SQL error is hidden by TERMOUT OFF: run the statements by hand)")}"
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

# run sqlldr: ctl name, log name (in /fixtures), expected rows
sqlldr_run() {
  local ctl="$1" logname="$2" rows="$3"
  docker exec "$CONTAINER" sqlldr userid="$CONNECT_CORPUS" control="/work/$ctl" \
      log="/fixtures/$logname" readsize=1048576 bindsize=1048576 rows=1 >> "$PL_LOG" 2>&1 \
    || echo "   (sqlldr returned a non-zero status; read the log)"
  NEW_FILES+=("$logname")
  make_files_readable
  grep -E 'SQL\*Loader-|ORA-|Rows successfully loaded|Rows not loaded' "$FIX_HOST/$logname" 2>/dev/null | sed 's/^/      /'
  grep -qE "^ *${rows} Rows? successfully loaded" "$FIX_HOST/$logname" 2>/dev/null \
    || echo "   !! not all $rows rows were loaded: the dumps still show what Oracle accepted (that is also an answer)"
}

# ---------------------------------------------------------------- (a) LONG 65526..65531
step_a() {
  if ! sql_ok R-051a/pre 18_long5_pre.sql; then FAILED_STEPS="$FAILED_STEPS R-051a"; return; fi
  local dat="$WORK_HOST/long5b.dat" ctl="$WORK_HOST/long5.ctl" sizes=(65526 65527 65528 65529 65530 65531) i=0 n got want
  : > "$dat"
  for n in "${sizes[@]}"; do
    i=$((i + 1))
    { printf '%d|L%06d' "$i" "$n"; head -c $((n - 7)) /dev/zero | tr '\0' 'y'; printf '|v%d\n' "$n"; } >> "$dat"
  done
  got=$(LC_ALL=C awk -F'|' '{printf "%s%d", (NR>1?",":""), length($2)}' "$dat"); want=$(IFS=,; echo "${sizes[*]}")
  if [ "$got" != "$want" ]; then echo "   !! wrong value sizes in the data file: $got (wanted $want)"; FAILED_STEPS="$FAILED_STEPS R-051a"; return; fi
  cat > "$ctl" <<CTL
LOAD DATA
INFILE '/work/long5b.dat'
APPEND INTO TABLE t_long5
FIELDS TERMINATED BY '|'
(id INTEGER EXTERNAL, l CHAR(100000), v CHAR(10))
CTL
  echo "   running sqlldr (6 rows, 65,526 to 65,531 bytes)"
  sqlldr_run long5.ctl "long5_sqlldr_${VER}.log" 6
  if ! sql_ok R-051a/truth 18_long5_post.sql truth_long5.txt; then FAILED_STEPS="$FAILED_STEPS R-051a"; return; fi
  NEW_FILES+=(truth_long5.txt)
  t_exp long5_exp        t_long5,t_frame statistics=none
  t_exp long5_exp_direct t_long5,t_frame statistics=none direct=y
  OK_STEPS="$OK_STEPS R-051a"
  echo "   truth_long5.txt:"; cut -c1-120 "$FIX_HOST/truth_long5.txt" | sed 's/^/      /'
}

# ---------------------------------------------------------------- (b) 18c / (c) 11g LONG RAW
step_b() {
  if [ "${NO_LONGRAW:-0}" = "1" ]; then SKIPPED_STEPS="$SKIPPED_STEPS R-051b/c(NO_LONGRAW)"; return; fi
  if ! sql_ok R-051b/pre 18_longraw5_pre.sql; then FAILED_STEPS="$FAILED_STEPS R-051b"; return; fi
  if [ "$VER" = "18c" ]; then
    local venv="${TMPDIR:-/var/tmp}/oradmp-oravenv"
    if [ ! -x "$venv/bin/python" ]; then
      echo "   making $venv with python-oracledb (pip, needs network)"
      python3 -m venv "$venv" >> "$PL_LOG" 2>&1 && "$venv/bin/pip" install -q oracledb >> "$PL_LOG" 2>&1 \
        || { echo "   !! could not install python-oracledb; LONG RAW skipped"; rm -rf "$venv"; SKIPPED_STEPS="$SKIPPED_STEPS R-051b(no-oracledb)"; return; }
    fi
    cat > "$WORK_HOST/longraw5_load.py" <<'PY'
import os
import oracledb
con = oracledb.connect(user="corpus", password=os.environ["CORPUS_PW"], dsn=os.environ["DSN"])
cur = con.cursor()
rows = [(1, 65521), (2, 65525), (3, 65529), (4, 65530), (5, 65531), (6, 65533), (7, 65534)]
for i, n in rows:
    try:
        cur.setinputsizes(None, oracledb.DB_TYPE_LONG_RAW)
        cur.execute("INSERT INTO t_longraw5 VALUES (:1, :2)", [i, bytes([0xAB]) * n])
        print("row", i, n, "inserted")
    except Exception as e:
        print("row", i, n, "REFUSED:", str(e).replace("\n", " "))
con.commit()
PY
    CORPUS_PW="$CORPUS_PW" DSN="localhost:${HOST_PORT}/${SERVICE}" "$venv/bin/python" -I "$WORK_HOST/longraw5_load.py" 2>&1 | sed 's/^/      /' | tee -a "$PL_LOG"
  else
    # 11g: SQL*Loader, the value as hex text in CHAR(200000)
    local dat="$WORK_HOST/longraw5b.dat" ctl="$WORK_HOST/longraw5.ctl" sizes=(65521 65525 65529 65530 65531 65533 65534 32768 65520 65535 100000) i=0 n got want
    : > "$dat"
    for n in "${sizes[@]}"; do
      i=$((i + 1))
      { printf '%d|' "$i"; yes AB | head -n "$n" | tr -d '\n'; printf '\n'; } >> "$dat"
    done
    got=$(LC_ALL=C awk -F'|' '{printf "%s%d", (NR>1?",":""), length($2)/2}' "$dat"); want=$(IFS=,; echo "${sizes[*]}")
    if [ "$got" != "$want" ]; then echo "   !! wrong value sizes in the data file: $got (wanted $want)"; FAILED_STEPS="$FAILED_STEPS R-051c"; return; fi
    cat > "$ctl" <<CTL
LOAD DATA
INFILE '/work/longraw5b.dat'
APPEND INTO TABLE t_longraw5
FIELDS TERMINATED BY '|'
(id INTEGER EXTERNAL, lr CHAR(200000))
CTL
    echo "   running sqlldr (11 rows, the value as hex text)"
    sqlldr_run longraw5.ctl "longraw5_sqlldr_${VER}.log" 11
    if ! grep -qE '^ *11 Rows? successfully loaded' "$FIX_HOST/longraw5_sqlldr_${VER}.log" 2>/dev/null; then
      # second route: LONG VARRAW (4-byte binary length + the bytes), records in "var 7" format (7 ASCII digits = record length)
      echo "   hex route failed (its error is kept in longraw5_sqlldr_${VER}.log); trying LONG VARRAW"
      local datv="$WORK_HOST/longraw5v.dat" ctlv="$WORK_HOST/longraw5v.ctl" m b0 b1 b2 b3
      : > "$datv"
      i=0
      for n in "${sizes[@]}"; do
        i=$((i + 1))
        b0=$((n & 255)); b1=$(((n >> 8) & 255)); b2=$(((n >> 16) & 255)); b3=$(((n >> 24) & 255))
        {
          printf '%07d' $((3 + 4 + n))
          printf '%03d' "$i"
          printf "\\x$(printf %02x $b0)\\x$(printf %02x $b1)\\x$(printf %02x $b2)\\x$(printf %02x $b3)"
          head -c "$n" /dev/zero | tr '\0' '\253'
        } >> "$datv"
      done
      cat > "$ctlv" <<CTL
LOAD DATA
INFILE '/work/longraw5v.dat' "var 7"
TRUNCATE INTO TABLE t_longraw5
(id POSITION(1:3) INTEGER EXTERNAL, lr POSITION(4) LONG VARRAW(200000))
CTL
      echo "   running sqlldr (11 rows, LONG VARRAW)"
      sqlldr_run longraw5v.ctl "longraw5_sqlldr_varraw_${VER}.log" 11
    fi
  fi
  if ! sql_ok R-051b/truth 18_longraw5_post.sql truth_longraw5.txt; then FAILED_STEPS="$FAILED_STEPS R-051b/c"; return; fi
  NEW_FILES+=(truth_longraw5.txt)
  echo "   truth_longraw5.txt:"; sed 's/^/      /' "$FIX_HOST/truth_longraw5.txt"
  # export only when rows exist
  if [ ! -s "$FIX_HOST/truth_longraw5.txt" ]; then echo "   !! no LONG RAW row was stored; no export"; FAILED_STEPS="$FAILED_STEPS R-051b/c"; return; fi
  t_exp longraw5_exp        t_longraw5,t_frame statistics=none
  t_exp longraw5_exp_direct t_longraw5,t_frame statistics=none direct=y
  OK_STEPS="$OK_STEPS R-051b/c"
}

wanted a && { if already_done long5_exp; then SKIPPED_STEPS="$SKIPPED_STEPS R-051a(exists)"; else step_a; fi; }
wanted b && { if already_done longraw5_exp; then SKIPPED_STEPS="$SKIPPED_STEPS R-051b/c(exists)"; else step_b; fi; }

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )
if [ "${#NEW_FILES[@]}" -gt 0 ]; then
  ( cd "$FIX_HOST" && tar czf "$WORK_HOST/pending3_${VER}.tar.gz" "${NEW_FILES[@]}" ) 2>/dev/null \
    && echo "Packed: $WORK_HOST/pending3_${VER}.tar.gz"
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
