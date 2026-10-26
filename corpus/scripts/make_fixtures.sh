#!/usr/bin/env bash
# make_fixtures.sh <18c|11g>
# Creates the SMALL files that go into git: corpus/fixtures/<ver>/
#   - truth_*.txt            expected values (SQL*Plus spool)
#   - <name>_dp_<ver>.dmp    Data Pump dump of one small table or table group
#   - <name>_exp_<ver>.dmp   legacy EXP dump, conventional path
#   - <name>_exp_direct_<ver>.dmp   legacy EXP dump, direct=y
#   - SHA256SUMS             checksums of everything above
# Logs of the run go to corpus/work/<ver>/logs/.

set -euo pipefail
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"

run_sql corpus "$CORPUS_PW" 02_truth_fixtures.sql

: > "$WORK_HOST/logs/export_${VER}.log"

# one dump per table
for t in t_frame t_len t_nullmix t_num t_numtypes t_date t_ts t_tstz t_raw t_text t_empty t_wide; do
  export_all fixtures_dir /fixtures "$t" "$t"
done

# BINARY_FLOAT / BINARY_DOUBLE are not supported by legacy exp (EXP-00104): Data Pump only.
echo "== t_float (Data Pump only)"
dp_export fixtures_dir t_float tables=t_float || echo "   !! expdp failed for t_float"

# groups of tables in one dump (several tables per file, constraints, foreign key)
export_all fixtures_dir /fixtures multi_framing "t_frame,t_len,t_nullmix"
export_all fixtures_dir /fixtures multi_types   "t_num,t_date,t_ts,t_text"
export_all fixtures_dir /fixtures ddl           "t_parent,t_ddl"

make_files_readable

( cd "$FIX_HOST" && sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS )

echo
echo "Fixtures for $VER are in $FIX_HOST"
echo "Export output: $WORK_HOST/logs/export_${VER}.log   (look for ORA-/EXP-/UDE- lines)"
