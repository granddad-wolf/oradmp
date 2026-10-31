#!/usr/bin/env bash
# Runs inside `quay.io/pypa/manylinux2014_x86_64` (glibc 2.17) for the `manylinux2014` job of ci.yml (T-16c, ADR-115
# item 4). Mounted by the job: the clone of the commit at /src (read-only), the dist folder at /dist (read-only), and
# a writable log folder at /out. Not meant to be run by hand.
#
# Environment: WHEEL (path below /dist), VERSION (the version `oradmp --version` must print). Four venvs, every
# install `--only-binary :all:`: (i) 3.11, pyarrow 15.0.2, pandas 2.2.3; (ii) 3.11, the newest pyarrow and pandas that
# have a wheel for glibc 2.17; (iii) 3.13, the newest pyarrow, no pandas; (iv) 3.14, only the pyarrow install is tried:
# its failure is printed and does not fail the script (if it works, (iv) runs as (iii)). Written for bash 4.2.
set -eo pipefail

: "${WHEEL:?WHEEL is not set}" "${VERSION:?VERSION is not set}"

ldd_out="$(ldd --version 2>&1)"
ldd_first="${ldd_out%%$'\n'*}"
echo "$ldd_first" | tee /out/ldd.txt
case "$ldd_first" in
    *" 2.17") ;;
    *)
        echo "this is not glibc 2.17" >&2
        exit 1
        ;;
esac
ls /opt/python | tee /out/interpreters.txt

git --version
git config --global --add safe.directory /src
mkdir -p /work/venvs
cp -R /src /work/src
git config --global --add safe.directory /work/src
cd /work/src

export WHEEL VERSION LOG_DIR=/out VENV_DIR=/work/venvs INSTALLER=pip
check="bash /work/src/tools/ci_install_check.sh"
py311=/opt/python/cp311-cp311/bin/python
py313=/opt/python/cp313-cp313/bin/python
py314=/opt/python/cp314-cp314/bin/python

$check py311-pa15-pd223 "$py311" "pyarrow==15.0.2 pandas==2.2.3" ""
$check py311-newest "$py311" "pyarrow pandas" ""
$check py313-newest "$py313" "pyarrow" "tests/test_consumers.py"

if [ ! -x "$py314" ]; then
    echo "py314: no $py314 in this image" | tee /out/py314.txt
elif "$py314" -m venv /work/venvs/probe314 && /work/venvs/probe314/bin/python -m pip install --quiet --only-binary :all: pyarrow >/out/py314-probe.log 2>&1; then
    echo "py314: pyarrow installs on glibc 2.17; running as (iii)" | tee /out/py314.txt
    $check py314-newest "$py314" "pyarrow" "tests/test_consumers.py"
else
    echo "py314: the pyarrow install failed (expected, no cp314 wheel for glibc 2.17); last lines of pip:" | tee /out/py314.txt
    tail -n 3 /out/py314-probe.log | tee -a /out/py314.txt
fi
echo "container 2.17: done"
