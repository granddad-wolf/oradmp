"""Task T-16a2 (b), ADR-90 item 1: ``tools/release_build.sh --release`` refuses any root but the canonical one.

The script is run with a commit that does not exist, so that no clone, venv or build is made; what is under test is
the order of the checks: the root rule comes before everything that writes."""

from __future__ import annotations

import os
import subprocess
from pathlib import Path

SCRIPT = Path(__file__).resolve().parent.parent / "tools" / "release_build.sh"
CANONICAL = "/var/tmp/oradmp-release"
OTHER = "/var/tmp/oradmp-release-t16a2-not-created"
BOGUS = "0000000000000000000000000000000000000000"


def run(*args: str, root: str | None) -> subprocess.CompletedProcess[str]:
    env = {k: v for k, v in os.environ.items() if k != "ORADMP_BUILD_ROOT"}
    if root is not None:
        env["ORADMP_BUILD_ROOT"] = root
    return subprocess.run(["bash", str(SCRIPT), *args], env=env, capture_output=True, text=True, check=False)


def test_release_refuses_another_root_and_names_both_paths() -> None:
    r = run(BOGUS, "--release", root=OTHER)
    assert r.returncode != 0
    assert CANONICAL in r.stderr and OTHER in r.stderr, r.stderr
    assert "release root: canonical" not in r.stdout
    assert "no such commit" not in r.stderr  # refused before anything else


def test_release_refuses_a_root_that_resolves_elsewhere(tmp_path: Path) -> None:
    link = tmp_path / "link"
    link.symlink_to("/var/tmp/oradmp-release-t16a2-not-created")
    r = run("--release", BOGUS, root=str(link))
    assert r.returncode != 0 and "needs the canonical build root" in r.stderr, r.stderr


def test_release_accepts_the_canonical_root_by_default_and_by_name() -> None:
    for root in (None, CANONICAL, CANONICAL + "/", "/var/tmp/../tmp/oradmp-release"):
        r = run(BOGUS, "--release", root=root)
        assert "release root: canonical" in r.stdout, (root, r.stdout, r.stderr)
        assert "needs the canonical build root" not in r.stderr


def test_without_release_another_root_is_accepted_as_before() -> None:
    r = run(BOGUS, root=OTHER)
    assert "needs the canonical build root" not in r.stderr
    assert "release root: canonical" not in r.stdout
    assert "no such commit" in r.stderr, r.stderr  # it got past the root checks
    assert not Path(OTHER).exists()


def test_help_names_release_and_the_canonical_root() -> None:
    r = run("--help", root=None)
    assert r.returncode == 0
    assert "--release" in r.stdout and CANONICAL in r.stdout
