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


# Task T-16c (a), ADR-115 item 3: ``--manylinux 2014|2_28``. A bad or a missing value exits 2 before "no such commit",
# before the root rules and before any folder is made.


def test_manylinux_bad_value_exits_2_naming_it_before_anything_else() -> None:
    for value in ("2010", "2_17", "manylinux2014", "", "2_28 "):
        r = run(BOGUS, "--release", "--manylinux", value, root=OTHER)
        assert r.returncode == 2, (value, r.returncode, r.stderr)
        assert f"'{value}'" in r.stderr and "--manylinux" in r.stderr, (value, r.stderr)
        assert "no such commit" not in r.stderr and "needs the canonical build root" not in r.stderr
        assert "release root: canonical" not in r.stdout
    assert not Path(OTHER).exists()


def test_manylinux_missing_value_exits_2() -> None:
    r = run(BOGUS, "--manylinux", root=OTHER)
    assert r.returncode == 2 and "needs a value" in r.stderr, r.stderr
    assert "no such commit" not in r.stderr
    assert not Path(OTHER).exists()


def test_manylinux_good_values_get_past_the_option_checks() -> None:
    for value in ("2014", "2_28"):
        r = run(BOGUS, "--manylinux", value, root=OTHER)
        assert r.returncode == 1 and "no such commit" in r.stderr, (value, r.returncode, r.stderr)
        r = run("--manylinux", value, BOGUS, "--release", root=None)
        # The canonical root may exist (the wheel job has just built there); the script checks the root before the
        # commit, so the run ends with one of the two documented reasons, both exit 1.
        reason = "exists already" if Path(CANONICAL).exists() else "no such commit"
        assert "release root: canonical" in r.stdout, (value, r.stdout, r.stderr)
        assert r.returncode == 1 and reason in r.stderr, (value, r.returncode, r.stderr)
    assert not Path(OTHER).exists()


def test_help_names_manylinux() -> None:
    r = run("--help", root=None)
    assert r.returncode == 0
    assert "--manylinux 2014|2_28" in r.stdout and "manylinux2014" in r.stdout and "manylinux_2_28" in r.stdout
