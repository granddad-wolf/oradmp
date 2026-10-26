#!/usr/bin/env bash
# make_stc.sh <18c|11g>
# R-023 (R-020 extra exports): the T_STC1..5 tables of make_stc.sh exported with other settings.
# Adds to corpus/fixtures/<ver>/:  stc_exp_us7ascii_<ver>.dmp (client NLS_LANG=AMERICAN_AMERICA.US7ASCII),
#   stc_exp_none_<ver>.dmp (statistics=none), their logs. Writes truth_stc_extra.txt just before exporting. Does NOT recreate tables or truth_stc.txt and does not
#   touch the existing stc_* fixtures. Needs the tables left by make_stc.sh in the database container (EXP-00011 in a log = they are gone).
#   Stops if SHA256SUMS does not verify or if the extra files already exist (FORCE=1).

set -euo pipefail
VERSION="make_stc_extra.sh v2 (2026-10-08)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
STC_LOG="$WORK_HOST/logs/stc_extra_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$STC_LOG"

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

# 2. extra files already there?
if [ -e "$FIX_HOST/stc_exp_us7ascii_${VER}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "stc_exp_us7ascii_${VER}.dmp already exists; nothing to do (FORCE=1 recreates the two extra dumps)."
  exit 0
fi
[ -s "$FIX_HOST/stc_exp_${VER}.dmp" ] && [ -s "$FIX_HOST/truth_stc.txt" ] \
  || { echo "!! run make_stc.sh $VER first" >&2; exit 1; }
rm -f "$FIX_HOST"/stc_exp_us7ascii_"${VER}".* "$FIX_HOST"/stc_exp_none_"${VER}".* "$FIX_HOST/truth_stc_extra.txt"

# 3. truth for these exports, written right before them (statistics may have changed since make_stc.sh)
run_sql corpus "$CORPUS_PW" 10b_stc_truth.sql
n=$(grep -cE 'ORA-|SP2-|PLS-' "$WORK_HOST/logs/10b_stc_truth.log" || true)
[ "$n" = "0" ] && [ -s "$FIX_HOST/truth_stc_extra.txt" ] \
  || { echo "!! truth_stc_extra.txt was not written ($n error lines in 10b_stc_truth.log)" >&2; exit 1; }
echo "   truth_stc_extra.txt: $(grep -c '^COLUMN|' "$FIX_HOST/truth_stc_extra.txt") column rows"

# 4. exports. st_exp <file_stem> <NLS_LANG or -> <direct y|n> <statistics estimate|compute|none>
st_exp() {
  local stem="$1" nls="$2" direct="$3" st="$4"
  local name="${stem}_${VER}"
  local envargs=()
  if [ "$nls" != "-" ]; then
    envargs=(-e "NLS_LANG=$nls")
  fi
  echo "   $name.dmp   (client ${nls}, direct=$direct, statistics=$st)"
  docker exec "${envargs[@]}" "$CONTAINER" exp "$CONNECT_CORPUS" tables=t_stc1,t_stc2,t_stc3,t_stc4,t_stc5 \
      direct="$direct" statistics="$st" file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" \
      >> "$STC_LOG" 2>&1 \
    || echo "   !! exp reported an error for ${name} (see the log in the folder)"
}

st_exp stc_exp_us7ascii AMERICAN_AMERICA.US7ASCII n estimate
st_exp stc_exp_none     -   n none

make_files_readable
( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "R-023 extra fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Lines to look at (the exports should end without warnings):"
grep -H -E 'EXP-|ORA-|Export done|successfully|Warning' "$FIX_HOST"/stc_exp_{us7ascii,none}_"${VER}".log || true
echo
ls -l "$FIX_HOST"/stc_exp_{us7ascii,none}_"${VER}".dmp "$FIX_HOST"/truth_stc.txt "$FIX_HOST"/truth_stc_extra.txt
