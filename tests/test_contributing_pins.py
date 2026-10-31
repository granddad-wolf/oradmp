"""Task T-21 (d), review 6c N-5: the tool versions that CONTRIBUTING.md names are the ones CI installs.

Read from ci.yml: the ``pip install`` line of the ``checks`` job (maturin, ruff, mypy, pytest) and the ``tool:`` line of
``taiki-e/install-action`` in the same job (cargo-nextest, cargo-deny). Read from CONTRIBUTING.md: the ``uv pip
install`` line and the ``cargo install --locked`` line. Mutation: change one version in either file and the case for
that tool fails.
"""

from __future__ import annotations

import re
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parent.parent
CI = (ROOT / ".github" / "workflows" / "ci.yml").read_text(encoding="utf-8")
CONTRIBUTING = (ROOT / "CONTRIBUTING.md").read_text(encoding="utf-8")

PIP_TOOLS = ["maturin", "pytest", "ruff", "mypy"]
CARGO_TOOLS = ["cargo-nextest", "cargo-deny"]


def _checks_job() -> str:
    start = CI.index("\n  checks:")
    match = re.search(r"\n  [a-z0-9_-]+:\n", CI[start + 1 :])
    end = start + 1 + match.start() if match else len(CI)
    return CI[start:end]


def _pins(text: str, names: list[str], sep: str) -> dict[str, str]:
    found: dict[str, str] = {}
    for name in names:
        versions = set(re.findall(rf"(?<![\w-]){re.escape(name)}{sep}(\d+\.\d+\.\d+)", text))
        assert len(versions) == 1, (name, versions)
        found[name] = versions.pop()
    return found


def _ci_pip_line() -> str:
    lines = [ln for ln in _checks_job().splitlines() if "-m pip install" in ln and "maturin==" in ln]
    assert len(lines) == 1
    return lines[0]


def _ci_tool_line() -> str:
    lines = [ln for ln in _checks_job().splitlines() if ln.strip().startswith("tool:")]
    assert len(lines) == 1
    return lines[0]


def _contributing_line(marker: str) -> str:
    lines = [ln for ln in CONTRIBUTING.splitlines() if marker in ln]
    assert len(lines) == 1, marker
    return lines[0]


@pytest.mark.parametrize("name", PIP_TOOLS)
def test_python_tool_pins_equal(name: str) -> None:
    ci = _pins(_ci_pip_line(), [name], "==")
    doc = _pins(_contributing_line("uv pip install"), [name], "==")
    assert ci == doc


@pytest.mark.parametrize("name", CARGO_TOOLS)
def test_cargo_tool_pins_equal(name: str) -> None:
    ci = _pins(_ci_tool_line(), [name], "@")
    doc = _pins(CONTRIBUTING, [name], "@")
    assert ci == doc
