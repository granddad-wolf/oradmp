#!/usr/bin/env bash
# make_bn.sh <18c|11g>
# R-013 (docs/formats/requests.md): a record boundary (0x4000) inside a two-byte length field and
# inside an `fe ff` NULL marker. Adds to corpus/fixtures/<ver>/:
#   t_bn1..t_bn4 _exp_<ver>.dmp and _exp_direct_<ver>.dmp, their logs, truth_bn1.txt .. truth_bn4.txt
# Designed cases: t_bn1 conventional (`eb 04`), t_bn2 direct (`ed 04`), t_bn3 conventional and t_bn4
# direct (`fe ff`); the other path of each table is a control.
# It never touches existing fixtures:
#   1. it verifies SHA256SUMS first and stops if an existing fixture has changed;
#   2. it skips any dump that already exists (FORCE=1 overwrites the t_bn* dumps);
#   3. it rewrites SHA256SUMS at the end.
# At the end it checks the bytes at the expected offsets (the assumption of R-013: the table section
# of a copy has the same length as that of T_RAW / T_LEN). A MISMATCH line is not an error of the
# export: send me the output and I recompute the lengths.

set -euo pipefail
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"

mkdir -p "$WORK_HOST/logs"
BN_LOG="$WORK_HOST/logs/bn_${VER}.log"
echo "=== make_bn.sh ${VER} $(date -u +%Y-%m-%dT%H:%M:%SZ) ===" >> "$BN_LOG"

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

# 2. tables and truth files
run_sql corpus "$CORPUS_PW" 06_bn.sql
n=$(grep -cE 'ORA-|SP2-' "$WORK_HOST/logs/06_bn.log" || true)
if [ "$n" != "0" ]; then
  echo "!! $n error line(s) in $WORK_HOST/logs/06_bn.log. Stopping before any export." >&2
  exit 1
fi

# bn_exp <file_stem> <direct y|n> <table>
bn_exp() {
  local stem="$1" direct="$2" table="$3"
  local name="${stem}_${VER}"
  if [ -e "$FIX_HOST/${name}.dmp" ] && [ "${FORCE:-0}" != "1" ]; then
    echo "   exists, skipped: ${name}.dmp"
    return 0
  fi
  echo "   $name.dmp"
  docker exec "$CONTAINER" exp "$CONNECT_CORPUS" tables="$table" direct="$direct" \
      file="/fixtures/${name}.dmp" log="/fixtures/${name}.log" \
      >> "$BN_LOG" 2>&1 \
    || echo "   !! exp failed for ${name} (see $BN_LOG)"
}

echo "== R-013 boundary tables"
for t in t_bn1 t_bn2 t_bn3 t_bn4; do
  bn_exp "${t}_exp"        n "$t"
  bn_exp "${t}_exp_direct" y "$t"
done

make_files_readable

( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

# 3. check the design: bytes at the expected offsets (decimal offsets; 0x3fff = 16383)
hexat() { od -An -v -tx1 -j "$2" -N "$3" "$1" | tr -d ' \n'; }
bad=0
expect() {   # <file> <offset> <expected hex> <label>
  local file="$1" off="$2" want="$3" label="$4" got
  got=$(hexat "$file" "$off" $(( ${#want} / 2 )))
  if [ "$got" = "$want" ]; then
    echo "   OK        $label @$off: $got"
  else
    echo "   MISMATCH  $label @$off: got $got, want $want"
    bad=1
  fi
}

echo
echo "== layout check (first row at 0x2249 = 8777 for t_bn1/t_bn2, 0x2251 = 8785 for t_bn3/t_bn4)"
F="$FIX_HOST"
# t_bn1: ID field `02 00 c1 xx`, then the length field of R
expect "$F/t_bn1_exp_${VER}.dmp"        8777  0200c1 "t_bn1 conv   first row, ID field"
expect "$F/t_bn1_exp_${VER}.dmp"        16379 0200c1 "t_bn1 conv   row 7, ID field"
expect "$F/t_bn1_exp_${VER}.dmp"        16383 eb04   "t_bn1 conv   length field across 0x4000 (eb | 04)"
expect "$F/t_bn1_exp_direct_${VER}.dmp" 8777  0200c1 "t_bn1 direct (control) first row, ID field"
# t_bn2
expect "$F/t_bn2_exp_direct_${VER}.dmp" 8777  0200c1 "t_bn2 direct first row, ID field"
expect "$F/t_bn2_exp_direct_${VER}.dmp" 16379 0200c1 "t_bn2 direct row 7, ID field"
expect "$F/t_bn2_exp_direct_${VER}.dmp" 16383 ed04   "t_bn2 direct length field across 0x4000 (ed | 04)"
expect "$F/t_bn2_exp_${VER}.dmp"        8777  0200c1 "t_bn2 conv   (control) first row, ID field"
# t_bn3: NULL ID `fe ff`, then the length field of A (256 = `00 01`)
expect "$F/t_bn3_exp_${VER}.dmp"        8785  feff0001 "t_bn3 conv   first row: NULL + length 256"
expect "$F/t_bn3_exp_${VER}.dmp"        16383 feff   "t_bn3 conv   NULL marker across 0x4000 (fe | ff)"
expect "$F/t_bn3_exp_direct_${VER}.dmp" 8785  feff   "t_bn3 direct (control) first row, NULL"
# t_bn4 (258 = `02 01`)
expect "$F/t_bn4_exp_direct_${VER}.dmp" 8785  feff0201 "t_bn4 direct first row: NULL + length 258"
expect "$F/t_bn4_exp_direct_${VER}.dmp" 16383 feff   "t_bn4 direct NULL marker across 0x4000 (fe | ff)"
expect "$F/t_bn4_exp_${VER}.dmp"        8785  feff   "t_bn4 conv   (control) first row, NULL"

echo
echo "Boundary fixtures for $VER are in $FIX_HOST (checksums updated)."
echo "Export output: $BN_LOG   (look for ORA-/EXP- lines)"
if [ "$bad" != "0" ]; then
  echo "Some checks did not match. Send me this output and, for each file, the result of:"
  echo "   xxd -s 0x2200 -l 160 <file>"
  exit 3
fi
echo "All layout checks passed."
