#!/usr/bin/env bash
# docker_run.sh <18c|11g>
# Creates and starts the Oracle XE container with three fixed mounts:
#   corpus/work/<ver>      -> /work       (large files, not in git)
#   corpus/fixtures/<ver>  -> /fixtures   (small files, in git)
#   corpus/sql             -> /sql        (read-only, the SQL scripts)
# The mounts never change, so every dump and truth file can be regenerated at any time.
#
# Required: export ORACLE_PASSWORD='...'   (the SYS/SYSTEM password you choose)

set -euo pipefail
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"
need_sys_password

mkdir -p "$WORK_HOST/logs" "$FIX_HOST"
chmod 777 "$WORK_HOST" "$WORK_HOST/logs" "$FIX_HOST"      # the container's oracle user must be able to write here

if docker ps -a --format '{{.Names}}' | grep -qx "$CONTAINER"; then
  echo "Container $CONTAINER already exists."
  echo "  start it:           docker start $CONTAINER"
  echo "  or recreate it:     docker rm -f $CONTAINER && $0 $VER"
  exit 1
fi

docker run -d --name "$CONTAINER" \
  -p "${HOST_PORT}:1521" \
  -e ORACLE_PASSWORD="$ORACLE_PASSWORD" \
  -v "$WORK_HOST:/work" \
  -v "$FIX_HOST:/fixtures" \
  -v "$SQL_HOST:/sql:ro" \
  "$IMAGE"

echo "Waiting for the database (the first start takes a few minutes)..."
for _ in $(seq 1 120); do
  if docker logs "$CONTAINER" 2>&1 | grep -q "DATABASE IS READY TO USE"; then
    echo "Database $VER is ready (container $CONTAINER, host port $HOST_PORT, service $SERVICE)."
    exit 0
  fi
  sleep 5
done
echo "Timed out. Check: docker logs $CONTAINER" >&2
exit 1
