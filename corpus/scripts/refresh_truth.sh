#!/usr/bin/env bash
# refresh_truth.sh <18c|11g>
# Rewrites the truth_*.txt files of corpus/fixtures/<ver>/ from the live database, WITHOUT touching
# any dump file. Use it after 02_truth_fixtures.sql has been corrected (for example the RAW truth, R-010).
#
# Safety:
#   - the database container must be running and the tables must exist (db_setup.sh ran earlier);
#   - after the refresh it compares everything with the old SHA256SUMS: only truth_*.txt files may
#     differ. If a dump file differs, it stops and does not touch SHA256SUMS;
#   - it prints which truth files changed, then rewrites SHA256SUMS.

set -euo pipefail
VER="${1:?usage: $0 <18c|11g>}"
source "$(dirname "$0")/config.sh"

[ -f "$FIX_HOST/SHA256SUMS" ] || { echo "No SHA256SUMS in $FIX_HOST" >&2; exit 1; }

run_sql corpus "$CORPUS_PW" 02_truth_fixtures.sql
make_files_readable

cd "$FIX_HOST"
changed="$(sha256sum -c SHA256SUMS 2>/dev/null | grep -v ': OK$' || true)"

if echo "$changed" | grep -q '\.dmp'; then
  echo "!! a dump file differs from SHA256SUMS. Stopping; SHA256SUMS not changed:" >&2
  echo "$changed" >&2
  exit 1
fi

if [ -n "$changed" ]; then
  echo "Truth files that changed:"
  echo "$changed" | sed 's/^/   /'
else
  echo "No truth file changed."
fi

sha256sum ./*.dmp ./truth_*.txt > SHA256SUMS
echo "SHA256SUMS rewritten ($VER)."
