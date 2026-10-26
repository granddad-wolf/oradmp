#!/usr/bin/env bash
# make_big.sh <18c|11g>
# O-17: builds the table T_BIG (5,000,000 rows, 15_big.sql) in the CORPUS schema and exports it with the legacy
# exp, conventional path, into the SCRATCH folder (corpus/work/<ver>/, mounted as /work). Nothing is committed.
# Output (in corpus/work/<ver>/):  t_big_exp_<v>.dmp   t_big_exp_<v>.log   truth_big.txt   SHA256SUMS_big
# Does nothing if t_big_exp_<v>.dmp exists (FORCE=1 recreates it). Needs about 5 GB free disk and a running container.
# Save next to make_extra.sh (it sources config.sh from the same folder); 15_big.sql goes into corpus/sql/.

set -uo pipefail
VERSION="make_big.sh v1 (2026-10-23)"
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
echo "$VERSION  ($VER)"

mkdir -p "$WORK_HOST/logs"
BIG_LOG="$WORK_HOST/logs/big_${VER}.log"
echo "=== $VERSION ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$BIG_LOG"

docker ps --format '{{.Names}}' | grep -qx "$CONTAINER" \
  || { echo "!! container $CONTAINER is not running (docker start $CONTAINER)" >&2; exit 1; }

[ -f "$SQL_HOST/15_big.sql" ] \
  || { echo "!! $SQL_HOST/15_big.sql is missing" >&2; exit 1; }

DMP="$WORK_HOST/t_big_exp_${VER}.dmp"
if [ -e "$DMP" ] && [ "${FORCE:-0}" != "1" ]; then
  echo "$DMP exists; nothing to do. FORCE=1 recreates it."
  exit 0
fi

free_kb=$(df -Pk "$WORK_HOST" | awk 'NR==2 {print $4}')
if [ "${free_kb:-0}" -lt 5000000 ]; then
  echo "!! less than 5 GB free in $WORK_HOST (${free_kb:-?} KB). Stopping." >&2
  exit 1
fi

rm -f "$WORK_HOST"/t_big_exp_"${VER}".{dmp,log} "$WORK_HOST/truth_big.txt" "$WORK_HOST/SHA256SUMS_big"

echo "== creating T_BIG (a few minutes)"
run_sql corpus "$CORPUS_PW" 15_big.sql || true
log="$WORK_HOST/logs/15_big.log"
n=$(grep -cE 'ORA-|SP2-|PLS-' "$log" 2>/dev/null || true)
if [ "${n:-0}" != "0" ] || [ ! -s "$WORK_HOST/truth_big.txt" ]; then
  echo "!! creating T_BIG failed: ${n:-?} error line(s) in $log; truth_big.txt $( [ -s "$WORK_HOST/truth_big.txt" ] && echo present || echo missing )" >&2
  grep -E 'ORA-|SP2-|PLS-' "$log" 2>/dev/null | head -5 | sed 's/^/      /' >&2
  exit 1
fi
echo "   truth_big.txt:"; sed 's/^/      /' "$WORK_HOST/truth_big.txt"
if ! grep -qx 'rows=5000000' "$WORK_HOST/truth_big.txt"; then
  echo "!! the row count is not 5000000. Stopping." >&2; exit 1
fi

echo "== exporting (exp, conventional path; a few minutes)"
exp_export /work t_big n tables=T_BIG statistics=none \
  || echo "   !! exp reported an error (see $WORK_HOST/t_big_exp_${VER}.log)"

make_files_readable
if [ ! -s "$DMP" ]; then
  echo "!! $DMP was not created" >&2; exit 1
fi
( cd "$WORK_HOST" && sha256sum "t_big_exp_${VER}.dmp" truth_big.txt > SHA256SUMS_big )

echo
echo "Done. Files in $WORK_HOST (scratch, not committed):"
ls -l "$DMP" "$WORK_HOST/t_big_exp_${VER}.log" "$WORK_HOST/truth_big.txt" "$WORK_HOST/SHA256SUMS_big"
echo
echo "Lines to look at in the export log (expect 5000000 rows exported and 'terminated successfully'):"
grep -E 'EXP-|ORA-|Export done|exporting table|successfully|Warning|conventional' "$WORK_HOST/t_big_exp_${VER}.log" || true
echo
echo "First bytes of the dump (expect the EXPORT:V header):"
head -c 12 "$DMP" | od -c | head -2
