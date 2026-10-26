"""T-18d (b), ADR-82: dump file names that are not valid UTF-8 (CP1251 names copied from Windows).

The name N is the bytes ``d2 e5 f1 f2 2e 64 6d 70`` (``Тест.dmp`` in CP1251); a copy of the 18c ``T_TEXT`` fixture is
made under ``tmp_path`` with that name. The fixture itself is only read. POSIX only: other file systems refuse the name.
"""

from __future__ import annotations

import errno
import os
import pathlib
import subprocess
import sys
from pathlib import Path

import oradmp
import pytest
from conftest import FIXTURES, fixture
from oradmp import _survey

pytestmark = [
    pytest.mark.skipif(sys.platform != "linux", reason="POSIX byte file names"),
    pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning"),
]

T_TEXT = "18c/t_text_exp_18c.dmp"
NAME = bytes.fromhex("d2e5f1f22e646d70")
PKG = Path(oradmp.__file__).resolve().parent


def child_env() -> dict[str, str]:
    return dict(os.environ, PYTHONPATH=str(PKG.parent))


def tool(*argv: str | bytes | os.PathLike[str]) -> subprocess.CompletedProcess[bytes]:
    return subprocess.run([sys.executable, "-m", "oradmp", *argv], capture_output=True, env=child_env(), check=False)


@pytest.fixture
def odd(tmp_path: Path) -> bytes:
    """The path (bytes) of the copy named N."""
    path = os.fsencode(tmp_path) + b"/" + NAME
    with open(path, "wb") as f:
        f.write((FIXTURES / T_TEXT).read_bytes())
    return path


def test_the_api_opens_and_reads_a_name_that_is_not_utf8(odd: bytes) -> None:
    text = os.fsdecode(odd)
    with pytest.raises(UnicodeEncodeError):
        text.encode("utf-8")  # the test would prove nothing with a name that is valid UTF-8
    expected = oradmp.open(fixture(T_TEXT)).read_table("T_TEXT").read_all()
    for arg in (text, pathlib.Path(text)):
        dump = oradmp.open(arg)
        assert [t.name for t in dump.tables] == ["T_TEXT"]
        assert dump.read_table("T_TEXT").read_all().equals(expected, check_metadata=True)


def test_the_api_names_a_missing_file_with_enoent(tmp_path: Path) -> None:
    gone = os.fsdecode(os.fsencode(tmp_path) + b"/" + NAME)
    with pytest.raises(OSError) as info:
        oradmp.open(gone)
    assert not isinstance(info.value, UnicodeError)
    assert info.value.errno == errno.ENOENT
    assert getattr(info.value, "code", None) == "E-IO"


def test_the_tool_works_on_a_name_that_is_not_utf8(odd: bytes, tmp_path: Path) -> None:
    csv = tmp_path / "t.csv"
    for argv in (
        ("tables", odd),
        ("columns", odd, "T_TEXT"),
        ("head", odd, "T_TEXT", "-n", "3"),
        ("export", odd, "T_TEXT", "-o", csv),
    ):
        r = tool(*argv)
        assert (r.returncode, r.stderr) == (0, b""), (argv[0], r.stderr[-300:])
        assert r.stdout or argv[0] == "export", argv[0]
    assert csv.stat().st_size > 0


def test_the_tool_writes_a_missing_name_as_ascii(tmp_path: Path) -> None:
    gone = os.fsencode(tmp_path) + b"/" + NAME
    r = tool("tables", gone)
    assert r.returncode == 3 and r.stdout == b""
    expected = f"oradmp: error: [E-IO] {os.strerror(errno.ENOENT)}: {tmp_path}/\\xd2\\xe5\\xf1\\xf2.dmp\n"
    assert r.stderr.decode("ascii") == expected


def test_survey_writes_the_file_line_with_escapes(odd: bytes) -> None:
    r = tool("survey", odd)
    assert r.returncode == 0, r.stderr[-300:]
    out = r.stdout.decode("ascii")  # no lone surrogate, no raw byte
    assert any(line.startswith("file: \\xd2\\xe5\\xf1\\xf2.dmp  size: ") for line in out.splitlines())
    hidden = tool("survey", odd, "--no-names")
    assert hidden.returncode == 0
    assert any(line.startswith("file: (hidden)  size: ") for line in hidden.stdout.decode("ascii").splitlines())


def test_path_text() -> None:
    assert _survey.path_text(os.fsdecode(b"a\xffb\t.dmp")) == "a\\xffb\\t.dmp"
    assert _survey.path_text("Тест.dmp") == "Тест.dmp"


OUT_NAME = bytes.fromhex("d2e5f1f22e637376")  # `Тест.csv` in CP1251


def test_the_tool_writes_an_existing_output_target_as_escapes(tmp_path: Path) -> None:
    """Review 06b SF-4: the "exists" refusal of ``export -o`` names the target as ``\\xHH``, not ``\\udcHH``."""
    target = os.fsencode(tmp_path) + b"/" + OUT_NAME
    with open(target, "wb") as f:
        f.write(b"keep")
    r = tool("export", FIXTURES / T_TEXT, "T_TEXT", "-o", target)
    assert r.returncode == 2 and r.stdout == b""
    text = r.stderr.decode("ascii")
    assert f"{tmp_path}/\\xd2\\xe5\\xf1\\xf2.csv exists" in text, text
    assert "\\udc" not in text
    assert Path(os.fsdecode(target)).read_bytes() == b"keep"


@pytest.mark.skipif(hasattr(os, "geteuid") and os.geteuid() == 0, reason="root can write to a read-only folder")
def test_the_tool_writes_the_filename_of_an_output_error_as_escapes(tmp_path: Path) -> None:
    """Review 06b SF-4: the ``filename`` of an ``OSError`` raised while the partial file is opened."""
    ro = tmp_path / "ro"
    ro.mkdir()
    ro.chmod(0o500)
    try:
        r = tool("export", FIXTURES / T_TEXT, "T_TEXT", "-o", os.fsencode(ro) + b"/" + OUT_NAME)
    finally:
        ro.chmod(0o700)
    assert r.returncode == 3 and r.stdout == b""
    text = r.stderr.decode("ascii")
    assert f"{ro}/\\xd2\\xe5\\xf1\\xf2.csv.oradmp-partial" in text, text
    assert "\\udc" not in text
