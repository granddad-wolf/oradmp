"""Task T-13g (ADR-54 item 7): ``oradmp.__version__`` is the installed metadata, which is the PEP 440 form of the
root ``Cargo.toml`` version (``check_dist.py``'s conversion, imported)."""

from __future__ import annotations

import re
from importlib import metadata
from pathlib import Path

import oradmp
from check_dist import cargo_version, pep440

ROOT = Path(__file__).resolve().parent.parent


def test_version_is_the_installed_metadata_and_the_cargo_version() -> None:
    assert oradmp.__version__ == metadata.version("oradmp")
    assert oradmp.__version__ != "0+unknown"
    assert oradmp.__version__ == pep440(cargo_version())


# --- T-13j (h), review 05 N-6: the README names the version that is built.
README_VERSIONS = re.compile(r"oradmp(?:==|-)(\d+\.\d+\.\d+(?:a\d+)?)")


def test_every_version_named_in_the_readme_is_the_cargo_version() -> None:
    found = README_VERSIONS.findall((ROOT / "README.md").read_text())
    assert len(found) >= 3  # the pip pin (twice), the wheel and the sdist file names
    assert set(found) == {pep440(cargo_version())}


def test_the_readme_version_check_finds_a_wrong_version() -> None:
    assert README_VERSIONS.findall('pip install "oradmp==0.1.0a1" oradmp-0.1.0a2.tar.gz oradmp-core') == [
        "0.1.0a1",
        "0.1.0a2",
    ]


# --- T-18c (e), review 06a N-1: a version in the prose of the README counts too.
ANY_PRERELEASE = re.compile(r"\b\d+\.\d+\.\d+(?:a|b|rc)\d+\b")


def test_every_pre_release_version_in_the_readme_is_the_cargo_version() -> None:
    found = ANY_PRERELEASE.findall((ROOT / "README.md").read_text())
    assert len(found) >= 3
    assert set(found) == {pep440(cargo_version())}


def test_the_wider_readme_check_finds_a_version_in_prose() -> None:
    assert ANY_PRERELEASE.findall("Alpha (version 0.1.0a2) and 1.2.3rc4, 1.0.0 but not 1.0") == ["0.1.0a2", "1.2.3rc4"]
