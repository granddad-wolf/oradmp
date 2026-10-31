#!/usr/bin/env bash
# Runs inside `rockylinux:8` (glibc 2.28) for the `manylinux2014` job of ci.yml (T-16c, ADR-115 item 4). Mounted by the
# job: the clone of the commit at /src (read-only), the dist folder at /dist (read-only), and a writable log folder at
# /out. Not meant to be run by hand.
#
# Environment: WHEEL (path below /dist), VERSION, UV_VERSION. Python 3.11 comes from `dnf install python3.11`, uv
# (pinned) from pip, Python 3.14 from `uv python install 3.14`; if one of them cannot be had this script fails with
# its message (then the owner runs the three venvs on the VM as in the a4 record, step 9). The three venvs are those of
# the `wheel` job: (1) 3.11, pyarrow 15.0.2, pandas 2.2.3; (2) 3.14, the newest pyarrow and pandas; (3) 3.11, pyarrow only.
set -eo pipefail

: "${WHEEL:?WHEEL is not set}" "${VERSION:?VERSION is not set}" "${UV_VERSION:?UV_VERSION is not set}"

ldd_out="$(ldd --version 2>&1)"
ldd_first="${ldd_out%%$'\n'*}"
echo "$ldd_first" | tee /out/ldd.txt
case "$ldd_first" in
    *" 2.28") ;;
    *)
        echo "this is not glibc 2.28" >&2
        exit 1
        ;;
esac
grep '^PRETTY_NAME=' /etc/os-release | tee /out/os-release.txt

dnf install -y python3.11 python3.11-pip git
python3.11 --version | tee /out/interpreters.txt
python3.11 -m pip install --quiet --root-user-action=ignore "uv==$UV_VERSION"
uv --version | tee -a /out/interpreters.txt
uv python install 3.14
uv python find 3.14 | tee -a /out/interpreters.txt

git config --global --add safe.directory /src
mkdir -p /work/venvs
cp -R /src /work/src
git config --global --add safe.directory /work/src
cd /work/src

export WHEEL VERSION LOG_DIR=/out VENV_DIR=/work/venvs INSTALLER=uv
check="bash /work/src/tools/ci_install_check.sh"
$check py311-pa15-pd223 /usr/bin/python3.11 "pyarrow==15.0.2 pandas==2.2.3" ""
$check py314-newest 3.14 "pyarrow pandas" ""
$check py311-pyarrow-only /usr/bin/python3.11 "pyarrow" "tests/test_consumers.py"
echo "container rocky 8: done"
