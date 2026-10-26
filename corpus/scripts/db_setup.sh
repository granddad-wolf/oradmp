#!/usr/bin/env bash
# db_setup.sh <18c|11g>
# Creates the CORPUS user, the directory objects and all test tables. Safe to run again:
# it drops and recreates the CORPUS user and its tables.
#
# Required: export ORACLE_PASSWORD='...'   (same value you used for docker_run.sh)

set -euo pipefail
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
need_sys_password

run_sql system "$ORACLE_PASSWORD" 00_setup.sql
run_sql corpus "$CORPUS_PW" 01_corpus_tables.sql
run_sql corpus "$CORPUS_PW" 01b_optional_tables.sql

echo
echo "Check the logs in $WORK_HOST/logs/."
echo "Expected on 11g: the partitioned table T_PART fails (ORA-00439), nothing else."
