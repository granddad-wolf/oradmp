"""Task T-13g (ADR-54): the old project name is gone from everything the Implementer owns. The name is built from
parts here so that this file does not contain it."""

from __future__ import annotations

import re
import shutil
import subprocess
from pathlib import Path

import pytest
from check_dist import ALLOWED_LINE

ROOT = Path(__file__).resolve().parent.parent
OLD = "ora" + "dump"
# the paths of the grep of T-13g (c); the README is checked on its own below
PATHS = [
    "crates",
    "python",
    "tests",
    "Cargo.toml",
    "Cargo.lock",
    "pyproject.toml",
    "deny.toml",
    "CONTRIBUTING.md",
    ".gitattributes",
    "LICENSE-MIT",
    "LICENSE-APACHE",
]


def tracked(paths: list[str]) -> list[str]:
    if shutil.which("git") is None or not (ROOT / ".git").exists():
        pytest.skip("not a git clone")
    out = subprocess.run(
        ["git", "ls-files", "-z", "--", *paths], cwd=ROOT, check=True, capture_output=True
    ).stdout.decode()
    return [p for p in out.split("\0") if p and (ROOT / p).is_file()]


def scan(root: Path, files: list[str], needle: str) -> list[str]:
    hits = []
    for rel in files:
        if needle in rel.lower():
            hits.append(f"{rel}: in the path")
        data = (root / rel).read_bytes().lower()
        pos = data.find(needle.encode())
        while pos >= 0:
            hits.append(f"{rel}: byte offset {pos}")
            pos = data.find(needle.encode(), pos + 1)
    return hits


def test_scan_finds_a_planted_name(tmp_path: Path) -> None:
    (tmp_path / "x.txt").write_text("a " + OLD.upper() + " b")
    assert scan(tmp_path, ["x.txt"], OLD) == ["x.txt: byte offset 2"]


def test_no_old_name_in_the_tracked_files() -> None:
    assert scan(ROOT, tracked(PATHS), OLD) == []


def test_readme_has_the_old_name_only_in_the_one_line() -> None:
    text = (ROOT / "README.md").read_bytes()
    assert text.lower().count(OLD.encode()) == 1
    assert ALLOWED_LINE in text.splitlines()
    assert text.startswith(b"# oradmp\n\n" + ALLOWED_LINE + b"\n")


# --- T-13j (g), review 05 N-8: the README is the PyPI page; a relative link does not resolve there.
LINK = re.compile(r"\]\(([^)\s]*)")


def relative_links(text: str) -> list[str]:
    return [t for t in LINK.findall(text) if not t.startswith(("https://", "http://", "#"))]


def test_the_readme_has_no_relative_link() -> None:
    assert relative_links((ROOT / "README.md").read_text()) == []


def test_the_relative_link_check_finds_a_planted_link() -> None:
    assert relative_links("see [LICENSE-MIT](LICENSE-MIT) and [x](https://a.b) and [y](#z)") == ["LICENSE-MIT"]
