"""Checks a built wheel and sdist (ADR-48 items 3 and 4, ADR-45 items 2 and 6). Not collected by pytest.

    python tests/check_dist.py [--build-root ROOT] WHEEL SDIST

Exit status 0 when every assertion holds; prints the listings of both archives first.
With ``--build-root ROOT`` (ADR-90 item 2) every build path in the wheel's SBOM member must begin with ROOT/src/.
"""

from __future__ import annotations

import configparser
import re
import sys
import tarfile
import zipfile
from email.parser import Parser
from pathlib import Path, PurePosixPath

# The old project name, case-insensitive, built from parts so that no tracked file of ours contains it (ADR-54).
OLD_NAME = ("ora" + "dump").encode()
# The one line that may hold it (ADR-54 item 2), in the README, the sdist's PKG-INFO and the wheel's METADATA.
ALLOWED_LINE = (
    "oradmp (not ora" + 'dump, which is an unrelated project on PyPI): "dmp" as in Oracle dump files, '
    "legacy exp now and Data Pump later."
).encode()

FORBIDDEN_NAMES = {"exp_survey.py"}
# ADR-57 item 1: a member fails if ANY component of its path (after an sdist's root folder) is one of these
FORBIDDEN_COMPONENTS = {"corpus", "tests", "docs", "tools", "__pycache__", "target", ".venv", ".git"}
FORBIDDEN_SUFFIXES = (".pyc", ".pyo")
# ADR-57 item 2: byte strings that no member of either archive may contain (case-sensitive), and the ones that are
# searched case-insensitively (a Windows home, plain and as the JSON-escaped form). No exception list.
HOME_PATHS = (b"/home/", b"/Users/", b"/root/")
HOME_PATHS_ANY_CASE = (b"C:\\Users\\", b"C:\\\\Users\\\\")
# ADR-59 item 5 (c): the test probe of a build with the cargo feature `test-probes`
PROBE_NAME = b"_probe_panic"


def forbidden(path: str, top_index: int) -> str | None:
    """Why a member path must not be in an archive (``top_index``: parts to skip, 1 for an sdist's root folder)."""
    p = PurePosixPath(path)
    parts = p.parts[top_index:]
    if p.name in FORBIDDEN_NAMES:
        return "the survey tool"
    if p.suffix == ".dmp":
        return "a dump file"
    if p.name.startswith("truth_") and p.suffix == ".txt":
        return "a truth file"
    for part in parts:
        if part in FORBIDDEN_COMPONENTS:
            return f"under a {part}/ folder"
    if p.name.endswith(FORBIDDEN_SUFFIXES):
        return "a compiled Python file"
    return None


def content_hits(label: str, data: bytes) -> list[str]:
    """One FAIL text per hit of a home path (ADR-57 item 2): member, byte offset, line number."""
    hits = []
    lower = data.lower()
    for needle, haystack in [(n, data) for n in HOME_PATHS] + [(n.lower(), lower) for n in HOME_PATHS_ANY_CASE]:
        pos = haystack.find(needle)
        while pos >= 0:
            line = data.count(b"\n", 0, pos) + 1
            hits.append(f"{label}: a home path ({needle.decode()}) at byte offset {pos} (line {line})")
            pos = haystack.find(needle, pos + 1)
    return hits


# ADR-90 item 2: a build path in the SBOM: a `path+file://` reference, or a path below /var/tmp/ or /tmp/ (not the tail
# of a longer path). The path ends at a quote, a backslash or white space (the SBOM is JSON).
BUILD_PATH = re.compile(rb"path\+file://(/[^\"\\\s]*)|(?<![\w./-])((?:/var/tmp|/tmp)/[^\"\\\s]*)")


def build_root_hits(label: str, data: bytes, root: str) -> list[str]:
    """One FAIL text per build path in ``data`` that does not begin with ``<root>/src/`` (byte offset of the path)."""
    prefix = root.rstrip("/").encode() + b"/src/"
    hits = []
    for m in BUILD_PATH.finditer(data):
        group = 1 if m.group(1) is not None else 2
        path = m.group(group)
        if not path.startswith(prefix):
            hits.append(
                f"{label}: a build path at byte offset {m.start(group)} is not below {prefix.decode()} "
                f"(path length {len(path)})"
            )
    return hits


def old_name_hits(label: str, data: bytes, allow_line: bool) -> list[str]:
    """One FAIL text per case-insensitive hit of the old name in ``data`` (byte offset, line number).

    With ``allow_line`` the exact line ALLOWED_LINE is blanked out first (same length, so offsets stay true).
    """
    if allow_line:
        data = data.replace(ALLOWED_LINE, b" " * len(ALLOWED_LINE))
    lower = data.lower()
    hits = []
    pos = lower.find(OLD_NAME)
    while pos >= 0:
        line = data.count(b"\n", 0, pos) + 1
        hits.append(f"{label}: the old project name at byte offset {pos} (line {line})")
        pos = lower.find(OLD_NAME, pos + 1)
    return hits


WHEEL_NAME = re.compile(r"^oradmp-(?P<version>[^-]+)-cp311-abi3-(?P<platform>[A-Za-z0-9_.]+)\.whl$")
ROOT_VERSION = re.compile(r"^\s*version\s*=\s*\"([^\"]+)\"", re.MULTILINE)


def pep440(semver: str) -> str | None:
    """PEP 440 form of a Cargo version (ADR-50 item 2): only X.Y.Z and X.Y.Z-alpha.N are verified."""
    if re.fullmatch(r"\d+\.\d+\.\d+", semver):
        return semver
    m = re.fullmatch(r"(\d+\.\d+\.\d+)-alpha\.(\d+)", semver)
    if m:
        return f"{m.group(1)}a{m.group(2)}"
    return None


def cargo_version() -> str:
    """``[workspace.package] version`` of the root Cargo.toml (the one source of the version)."""
    text = (Path(__file__).resolve().parent.parent / "Cargo.toml").read_text()
    start = text.index("[workspace.package]")
    m = ROOT_VERSION.search(text, start)
    if m is None:
        raise SystemExit("check_dist: no version in [workspace.package]")
    return m.group(1)


def check_metadata(text: str, label: str, errors: list[str], version: str) -> None:
    msg = Parser().parsestr(text)
    if msg["Version"] != version:
        errors.append(f"{label}: Version is {msg['Version']!r}, the wheel name says {version!r}")
    meta_version = tuple(int(x) for x in msg["Metadata-Version"].split("."))
    if meta_version < (2, 4):
        errors.append(f"{label}: Metadata-Version {msg['Metadata-Version']} is below 2.4")
    if msg["License-Expression"] != "MIT OR Apache-2.0":
        errors.append(f"{label}: License-Expression is {msg['License-Expression']!r}")
    if sorted(msg.get_all("License-File") or []) != ["LICENSE-APACHE", "LICENSE-MIT"]:
        errors.append(f"{label}: License-File lines are {msg.get_all('License-File')!r}")
    if msg["Requires-Python"] != ">=3.11":
        errors.append(f"{label}: Requires-Python is {msg['Requires-Python']!r}")
    if "pyarrow>=15" not in (msg.get_all("Requires-Dist") or []):
        errors.append(f"{label}: Requires-Dist {msg.get_all('Requires-Dist')!r} lacks pyarrow>=15")
    if not str(msg.get_payload()).lstrip().startswith("# oradmp"):
        errors.append(f"{label}: the description is not the README")
    if any(c.startswith("License ::") for c in msg.get_all("Classifier") or []):
        errors.append(f"{label}: a License :: classifier beside License-Expression")


ENTRY_POINT = ("oradmp", "oradmp._cli:main")


def check_entry_points(text: str, label: str, errors: list[str]) -> None:
    """ADR-72 item 11: exactly one section ``[console_scripts]`` holding exactly ``oradmp = oradmp._cli:main``."""
    parser = configparser.RawConfigParser(delimiters=("=",), strict=True)
    parser.optionxform = str  # type: ignore[assignment,method-assign]
    try:
        parser.read_string(text)
    except configparser.Error:
        errors.append(f"{label}: entry_points.txt cannot be parsed")
        return
    if parser.sections() != ["console_scripts"]:
        errors.append(f"{label}: entry_points.txt sections are {parser.sections()!r}, not ['console_scripts']")
        return
    items = [(k, v.strip()) for k, v in parser.items("console_scripts")]
    if items != [ENTRY_POINT]:
        errors.append(f"{label}: [console_scripts] is {items!r}, not [{ENTRY_POINT!r}]")


def check_wheel(path: str, errors: list[str], build_root: str | None = None) -> str | None:
    """Checks the wheel and returns the version of its file name (None if the name is wrong)."""
    m = WHEEL_NAME.match(Path(path).name)
    if m is None:
        errors.append(f"wheel: file name {Path(path).name!r} is not oradmp-<version>-cp311-abi3-<platform>.whl")
        return None
    version = m.group("version")
    dist_info = f"oradmp-{version}.dist-info"
    expected = pep440(cargo_version())
    if expected is None:
        errors.append(f"Cargo.toml version {cargo_version()!r}: conversion not verified")
    elif expected != version:
        errors.append(f"wheel: version {version!r} in the file name, {expected!r} from Cargo.toml")
    with zipfile.ZipFile(path) as z:
        names = z.namelist()
        print(f"--- {path}")
        for n in names:
            print(n)
        for n in names:
            top = n.split("/", 1)[0]
            if top not in ("oradmp", dist_info):
                errors.append(f"wheel: {n} is outside oradmp/ and {dist_info}/")
            why = forbidden(n, 0)
            if why:
                errors.append(f"wheel: {n} is {why}")
        for n in names:
            allow = n == f"{dist_info}/METADATA"
            data = z.read(n)
            errors.extend(old_name_hits(f"wheel member {n}", data, allow))
            errors.extend(content_hits(f"wheel member {n}", data))
            if build_root is not None and n.startswith(f"{dist_info}/sboms/"):
                errors.extend(build_root_hits(f"wheel member {n}", data, build_root))
            if n.endswith((".so", ".pyd", ".dylib")) and PROBE_NAME in data:
                errors.append(f"wheel member {n}: contains {PROBE_NAME.decode()} (built with --features test-probes)")
            errors.extend(old_name_hits(f"wheel member name {n}", n.encode(), False))
        for lic in ("LICENSE-MIT", "LICENSE-APACHE"):
            if f"{dist_info}/licenses/{lic}" not in names:
                errors.append(f"wheel: {lic} is not under .dist-info/licenses/")
        if f"{dist_info}/entry_points.txt" not in names:
            errors.append(f"wheel: {dist_info}/entry_points.txt is missing")
        else:
            check_entry_points(z.read(f"{dist_info}/entry_points.txt").decode(), "wheel entry_points.txt", errors)
        if f"{dist_info}/METADATA" not in names:
            errors.append(f"wheel: {dist_info}/METADATA is missing")
        else:
            check_metadata(z.read(f"{dist_info}/METADATA").decode(), "wheel METADATA", errors, version)
    return version


def check_sdist(path: str, errors: list[str], version: str | None) -> None:
    with tarfile.open(path) as t:
        names = t.getnames()
        print(f"--- {path}")
        for n in names:
            print(n)
        for n in names:
            why = forbidden(n, 1)
            if why:
                errors.append(f"sdist: {n} is {why}")
        root = names[0].split("/", 1)[0]
        if version is not None and root != f"oradmp-{version}":
            errors.append(f"sdist: root folder {root!r} is not oradmp-{version}")
        if version is not None and Path(path).name != f"oradmp-{version}.tar.gz":
            errors.append(f"sdist: file name {Path(path).name!r} is not oradmp-{version}.tar.gz")
        for need in (
            "Cargo.toml",
            "Cargo.lock",
            "pyproject.toml",
            "README.md",
            "LICENSE-MIT",
            "LICENSE-APACHE",
            "crates/oradmp-core/src/lib.rs",
            "crates/oradmp-core/data/exp_intro_lines.txt",
            "crates/oradmp-py/src/lib.rs",
            "python/oradmp/__init__.py",
            "python/oradmp/_cli.py",
        ):
            if f"{root}/{need}" not in names:
                errors.append(f"sdist: {need} is missing")
        for m in t.getmembers():
            errors.extend(old_name_hits(f"sdist member name {m.name}", m.name.encode(), False))
            if m.isfile():
                f = t.extractfile(m)
                if f is not None:
                    allow = m.name in (f"{root}/README.md", f"{root}/PKG-INFO")
                    data = f.read()
                    errors.extend(old_name_hits(f"sdist member {m.name}", data, allow))
                    errors.extend(content_hits(f"sdist member {m.name}", data))
        try:
            member = t.extractfile(f"{root}/PKG-INFO")
        except KeyError:  # tarfile raises for an absent member (T-13j (e): print the FAIL line instead)
            member = None
        if member is None:
            errors.append("sdist: PKG-INFO is missing")
        else:
            check_metadata(member.read().decode(), "sdist PKG-INFO", errors, version or "")


def main(argv: list[str]) -> int:
    build_root: str | None = None
    if len(argv) == 4 and argv[0] == "--build-root":
        build_root = argv[1]
        argv = argv[2:]
    if len(argv) != 2 or not argv[0] or argv[0].startswith("--"):
        print(__doc__)
        return 2
    errors: list[str] = []
    print("check_dist: build root", build_root if build_root is not None else "(not given)")
    version = check_wheel(argv[0], errors, build_root)
    check_sdist(argv[1], errors, version)
    for e in errors:
        print("FAIL:", e)
    print("check_dist:", "FAILED" if errors else "ok")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
