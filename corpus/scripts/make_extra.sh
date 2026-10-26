#!/usr/bin/env bash
# make_extra.sh <18c|11g>
# Requests R-031, R-032, R-033, R-034, R-035, R-036 (both versions) and R-037 (18c only) from docs/formats/requests.md.
# Adds to corpus/fixtures/<ver>/ (statistics=none unless said otherwise):
#   R-032 t_num3_exp_<v>.dmp                       truth_num3.txt
#   R-033 nchar2_exp_<v>.dmp                       truth_nchar2.txt
#   R-034 t_probe11_exp_<v>.dmp, t_probe11_exp_direct_<v>.dmp   truth_probe11.txt
#   R-035 lob3_exp_<v>.dmp                         truth_lob3.txt
#   R-036 misc2_exp_<v>.dmp                        truth_misc2.txt
#   R-031 long2_exp_<v>.dmp, long2_exp_direct_<v>.dmp, long2_exp_us7ascii_<v>.dmp, long_exp_direct_<v>.dmp   truth_long2.txt
#   R-037 (18c only, default statistics) part2_exp_18c.dmp, part2_exp_direct_18c.dmp   truth_part2.txt
# Every request is a separate step: if its SQL fails, that step is reported and skipped, the others go on.
# It verifies SHA256SUMS first, does nothing if t_num3_exp_<v>.dmp exists (FORCE=1 recreates exactly these files),
# and rewrites SHA256SUMS at the end. Needs the original corpus tables, T_PROBE6 (R-006) and the tables of make_types.sh
# (T_LONG, T_LONGRAW).

set -uo pipefail
VERSION="make_extra.sh v1 (2026-10-15)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
EX_LOG="$WORK_HOST/logs/extra_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$EX_LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" \
  || { echo "!! container $CONTAINER is not running" >&2; exit 1; }

if [ -f "$FIX_HOST/SHA256SUMS" ]; then
  echo "== verifying existing fixtures"
  if ! ( cd "$FIX_HOST" && sha256sum -c --quiet SHA256SUMS ); then
    echo "!! an existing fixture differs from SHA256SUMS. Stopping." >&2
    exit 1
  fi
  echo "   ok"
fi

if [ -e "$FIX_HOST/t_num3_exp_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "extra fixtures for $VER already exist; nothing to do. FORCE=1 recreates them."
  exit 0
fi
rm -f "$FIX_HOST"/{t_num3_exp,nchar2_exp,t_probe11_exp,t_probe11_exp_direct,lob3_exp,misc2_exp,long2_exp,long2_exp_direct,long2_exp_us7ascii,long_exp_direct,part2_exp,part2_exp_direct}_"${VER}".{dmp,log} \
      "$FIX_HOST"/truth_{num3,nchar2,probe11,lob3,misc2,long2,part2}.txt

OK_STEPS=""; FAILED_STEPS=""

# step <name> <sql file> <truth file>   -> returns 0 when the SQL ran without errors and the truth file exists
step() {
  local name="$1" sql="$2" truth="$3" log
  echo "== $name  ($sql)"
  run_sql corpus "$CORPUS_PW" "$sql" || true
  log="$WORK_HOST/logs/${sql%.sql}.log"
  local n; n=$(grep -cE 'ORA-|SP2-|PLS-|compilation errors' "$log" 2>/dev/null || true)
  if [ "${n:-0}" != "0" ] || [ ! -s "$FIX_HOST/$truth" ]; then
    echo "   !! $name FAILED: ${n:-?} error line(s) in $log, truth file $truth $( [ -s "$FIX_HOST/$truth" ] && echo present || echo missing )"
    grep -E 'ORA-|SP2-|PLS-' "$log" 2>/dev/null | head -3 | sed 's/^/      /'
    FAILED_STEPS="$FAILED_STEPS $name"
    return 1
  fi
  echo "   truth: $truth, $(grep -c . "$FIX_HOST/$truth") lines"
  OK_STEPS="$OK_STEPS $name"
  return 0
}

# t_exp <file stem> <tables> [extra exp args]; NLS_CLIENT may be set for the call
t_exp() {
  local stem="$1" tables="$2"; shift 2
  local name="${stem}_${VER}"
  echo "   $name.dmp   (tables=$tables $*)"
  local envarg=()
  [ -n "${NLS_CLIENT:-}" ] && envarg=(-e "NLS_LANG=$NLS_CLIENT")
  docker exec "${envarg[@]}" "$CONTAINER" exp "$CONNECT_CORPUS" tables="$tables" "$@" \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" >> "$EX_LOG" 2>&1 \
    || echo "   !! exp reported an error for ${name} (see the log in the folder)"
}

if step R-032 14_num3.sql truth_num3.txt; then
  t_exp t_num3_exp t_num3 statistics=none
fi
if step R-033 14_nchar2.sql truth_nchar2.txt; then
  t_exp nchar2_exp t_nchar2,t_frame statistics=none
fi
if step R-034 14_probe11.sql truth_probe11.txt; then
  t_exp t_probe11_exp        t_probe11,t_frame statistics=none
  t_exp t_probe11_exp_direct t_probe11,t_frame statistics=none direct=y
fi
if step R-035 14_lob3.sql truth_lob3.txt; then
  t_exp lob3_exp t_lob3,t_frame statistics=none
fi
if step R-036 14_misc2.sql truth_misc2.txt; then
  t_exp misc2_exp t_interval2,t_rowid2,t_frame statistics=none
fi
if step R-031 14_long2.sql truth_long2.txt; then
  t_exp long2_exp        t_long2,t_frame statistics=none
  t_exp long2_exp_direct t_long2,t_frame statistics=none direct=y
  NLS_CLIENT=AMERICAN_AMERICA.US7ASCII t_exp long2_exp_us7ascii t_long2,t_frame statistics=none
  t_exp long_exp_direct  t_long,t_longraw,t_long_empty,t_frame statistics=none direct=y
  echo "   truth_long2.txt (read it: the line for id 2 says whether Oracle accepted the 100,000-byte LONG):"
  sed 's/^/      /' "$FIX_HOST/truth_long2.txt"
fi
if [ "$VER" = "18c" ]; then
  if step R-037 14_part2.sql truth_part2.txt; then
    t_exp part2_exp        t_part2,t_frame
    t_exp part2_exp_direct t_part2,t_frame direct=y
  fi
fi

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "Steps ok:     ${OK_STEPS:- none}"
echo "Steps failed: ${FAILED_STEPS:- none}"
echo "Extra fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Lines to look at (warnings are possible, for example for US7ASCII and for LOB / LONG tables on the direct path):"
grep -H -E 'EXP-|ORA-|Export done|successfully|Warning|conventional' \
  "$FIX_HOST"/{t_num3_exp,nchar2_exp,t_probe11_exp,t_probe11_exp_direct,lob3_exp,misc2_exp,long2_exp,long2_exp_direct,long2_exp_us7ascii,long_exp_direct,part2_exp,part2_exp_direct}_"${VER}".log 2>/dev/null || true
echo
ls -l "$FIX_HOST"/{t_num3_exp,nchar2_exp,t_probe11_exp,t_probe11_exp_direct,lob3_exp,misc2_exp,long2_exp,long2_exp_direct,long2_exp_us7ascii,long_exp_direct,part2_exp,part2_exp_direct}_"${VER}".dmp \
      "$FIX_HOST"/truth_{num3,nchar2,probe11,lob3,misc2,long2,part2}.txt 2>/dev/null || true
