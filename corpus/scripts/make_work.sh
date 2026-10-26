#!/usr/bin/env bash
# make_work.sh <18c|11g>
# Creates the LARGE files that are NOT committed: corpus/work/<ver>/
#   - truth_rows.txt                    expected values of T_ROWS (100,000 lines)
#   - t_rows_*, schema_* dumps          one big table and the whole schema, both formats
#   - optional tables (LOB, interval, partitioned), skipped if a table does not exist
# Logs of the run go to corpus/work/<ver>/logs/.

set -euo pipefail
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"

run_sql corpus "$CORPUS_PW" 03_truth_work.sql

: > "$WORK_HOST/logs/export_${VER}.log"

export_all work_dir /work t_rows "t_rows"

# optional tables from 01b_optional_tables.sql
for t in t_lob t_interval t_part; do
  export_all work_dir /work "$t" "$t"
done

# the whole schema: tables, sequence, view, constraints, index, comments
echo "== schema (owner corpus)"
dp_export  work_dir schema schemas=corpus          || echo "   !! expdp failed for schema"
exp_export /work schema n owner=corpus             || echo "   !! exp failed for schema"
exp_export /work schema y owner=corpus             || echo "   !! exp direct=y failed for schema"

make_files_readable

echo
echo "Work files for $VER are in $WORK_HOST"
echo "Export output: $WORK_HOST/logs/export_${VER}.log   (failures for t_part on 11g are expected)"
