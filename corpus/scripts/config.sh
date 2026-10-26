#!/usr/bin/env bash
# config.sh - shared settings and helper functions.
# Sourced by the other scripts (never run directly). The caller sets VER (18c or 11g) first.

: "${VER:?VER must be set to 18c or 11g}"

#ORADUMP_HOME="${ORADUMP_HOME:-$HOME/oradump}"
ORADUMP_HOME="${ORADUMP_HOME:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)}"
CORPUS_DIR="$ORADUMP_HOME/corpus"
SQL_HOST="$CORPUS_DIR/sql"
WORK_HOST="$CORPUS_DIR/work/$VER"          # mounted as /work     (not committed)
FIX_HOST="$CORPUS_DIR/fixtures/$VER"       # mounted as /fixtures (committed)

# Password of the CORPUS user. It is NOT kept in the repository. Put it in
#   ~/.config/oradmp/corpus.env     (one line:  CORPUS_PW='...'  and chmod 600)
# or export CORPUS_PW before you run a script. run_sql passes it to 00_setup.sql as &1.
if [ -z "${CORPUS_PW:-}" ] && [ -f "${HOME}/.config/oradmp/corpus.env" ]; then
  . "${HOME}/.config/oradmp/corpus.env"
fi
: "${CORPUS_PW:?Set CORPUS_PW (the password of the CORPUS user): export it or put CORPUS_PW='...' in ~/.config/oradmp/corpus.env}"

case "$VER" in
  18c) IMAGE="gvenzl/oracle-xe:18"; CONTAINER="ora18"; HOST_PORT=1521; SERVICE="XEPDB1" ;;
  11g) IMAGE="gvenzl/oracle-xe:11"; CONTAINER="ora11"; HOST_PORT=1522; SERVICE="XE" ;;
  *)   echo "unknown version '$VER' (use 18c or 11g)" >&2; exit 2 ;;
esac

CONNECT_CORPUS="corpus/${CORPUS_PW}@//localhost:1521/${SERVICE}"   # port 1521 is the port INSIDE the container

need_sys_password() {
  : "${ORACLE_PASSWORD:?Set ORACLE_PASSWORD (the SYS/SYSTEM password of the container), e.g. export ORACLE_PASSWORD='...'}"
}

# run_sql <user> <password> <script.sql>
# Runs a script from corpus/sql (mounted read-only as /sql) with SQL*Plus inside the container.
run_sql() {
  local user="$1" pw="$2" script="$3"
  local log="$WORK_HOST/logs/${script%.sql}.log"
  mkdir -p "$WORK_HOST/logs"
  echo "== running $script"
  docker exec "$CONTAINER" sqlplus -L "$user/$pw@//localhost:1521/$SERVICE" "@/sql/$script" "$CORPUS_PW" > "$log" 2>&1 || true
  local n
  n=$(grep -cE 'ORA-|SP2-' "$log" || true)
  echo "   finished, $n error line(s) in $log"
}

# dp_export <directory_object> <name> <expdp scope args...>
dp_export() {
  local dirobj="$1" name="$2"; shift 2
  docker exec "$CONTAINER" expdp "$CONNECT_CORPUS" "$@" directory="$dirobj" \
    dumpfile="${name}_dp_${VER}.dmp" logfile="${name}_dp_${VER}.log" reuse_dumpfiles=y \
    >> "$WORK_HOST/logs/export_${VER}.log" 2>&1
}

# exp_export <container_dir> <name> <direct y|n> <exp scope args...>
exp_export() {
  local cdir="$1" name="$2" direct="$3"; shift 3
  local tag="exp"
  [ "$direct" = "y" ] && tag="exp_direct"
  docker exec "$CONTAINER" exp "$CONNECT_CORPUS" "$@" direct="$direct" \
    file="${cdir}/${name}_${tag}_${VER}.dmp" log="${cdir}/${name}_${tag}_${VER}.log" \
    >> "$WORK_HOST/logs/export_${VER}.log" 2>&1
}

# export_all <directory_object> <container_dir> <name> <comma-separated tables>
# Produces three files: Data Pump, legacy EXP (conventional path), legacy EXP (direct=y).
export_all() {
  local dirobj="$1" cdir="$2" name="$3" tables="$4"
  echo "== $name ($tables)"
  dp_export "$dirobj" "$name" tables="$tables"            || echo "   !! expdp failed for $name"
  exp_export "$cdir" "$name" n tables="$tables"           || echo "   !! exp failed for $name"
  exp_export "$cdir" "$name" y tables="$tables"           || echo "   !! exp direct=y failed for $name"
}

# Data Pump creates its dump files readable only by the oracle user (mode 640).
# Make every file in the two mounts world-readable so the host user can read it.
# (The minimal container image has no "find", so a plain shell loop is used.)
#make_files_readable() {
#  docker exec "$CONTAINER" bash -c 'for f in /work/* /fixtures/*; do if [ -f "$f" ]; then chmod a+r "$f"; fi; done'
#}
make_files_readable() {
  docker exec "$CONTAINER" bash -c '
    for f in /work/* /fixtures/*; do
      if [ -f "$f" ] && [ "$(stat -c %U "$f")" = "$(id -un)" ]; then chmod a+r "$f"; fi
    done
    true'
}