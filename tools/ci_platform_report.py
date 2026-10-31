"""Helpers for the `platforms` job of ci.yml (T-16b, ADR-118).

The steps run in Git Bash on Windows and in bash on macOS, so hashing, listing and comparing are done here in Python
and not with sha256sum, unzip or awk (macOS has no sha256sum, Git Bash has no unzip). Standard library only; prints
ASCII only (a Windows console may be cp1252).

    python -I tools/ci_platform_report.py checksums DIR...   verify DIR/SHA256SUMS; exit 1 on a mismatch
    python -I tools/ci_platform_report.py sums DIR           write DIR/SHA256SUMS for the wheel and the sdist
    python -I tools/ci_platform_report.py members DIR        "sha256 size member" of the wheel and of the sdist
    python -I tools/ci_platform_report.py tags WHEEL         the wheel's file name and its Tag: lines
    python -I tools/ci_platform_report.py compare A B        both hashes of each archive: identical, or differs

Not a gate: `compare` exits 0 whether the archives are equal or not (ADR-118 item 7: recorded, not a gate).
"""

from __future__ import annotations

import glob
import hashlib
import sys
import tarfile
import zipfile
from pathlib import Path


def sha256_file(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            h.update(block)
    return h.hexdigest()


def find(directory: str, pattern: str) -> str:
    found = sorted(glob.glob(str(Path(directory) / pattern)))
    if len(found) != 1:
        raise SystemExit(f"ci_platform_report: expected exactly one {pattern} in {directory}, found {len(found)}")
    return found[0]


def members(path: str) -> dict[str, tuple[str, int]]:
    """Member name -> (sha256, size) of the files in a wheel or an sdist."""
    out: dict[str, tuple[str, int]] = {}
    if path.endswith(".whl"):
        with zipfile.ZipFile(path) as z:
            for info in z.infolist():
                if not info.is_dir():
                    out[info.filename] = (hashlib.sha256(z.read(info)).hexdigest(), info.file_size)
    else:
        with tarfile.open(path) as t:
            for m in t:
                if m.isfile():
                    f = t.extractfile(m)
                    out[m.name] = (hashlib.sha256(f.read() if f else b"").hexdigest(), m.size)
    return out


def cmd_checksums(dirs: list[str]) -> int:
    bad = 0
    for d in dirs:
        listing = Path(d) / "SHA256SUMS"
        count = 0
        for line in listing.read_text(encoding="ascii").splitlines():
            line = line.strip()
            if not line:
                continue
            digest, _, name = line.partition(" ")
            name = name.strip().lstrip("*")
            actual = sha256_file(str(Path(d) / name))
            count += 1
            if actual != digest.lower():
                bad += 1
                print(f"checksum MISMATCH: {d}/{name}")
        print(f"checksums ok: {d} ({count} files)" if bad == 0 else f"checksums FAILED: {d}")
    return 1 if bad else 0


def cmd_sums(directory: str) -> int:
    lines = []
    for pattern in ("oradmp-*.whl", "oradmp-*.tar.gz"):
        p = find(directory, pattern)
        lines.append(f"{sha256_file(p)}  {Path(p).name}")
    text = "\n".join(lines) + "\n"
    (Path(directory) / "SHA256SUMS").write_text(text, encoding="ascii", newline="\n")
    sys.stdout.write(text)
    return 0


def cmd_members(directory: str) -> int:
    for pattern in ("oradmp-*.whl", "oradmp-*.tar.gz"):
        for p in sorted(glob.glob(str(Path(directory) / pattern))):
            for name, (digest, size) in members(p).items():
                print(digest, size, name)
    return 0


def cmd_tags(wheel: str) -> int:
    print(Path(wheel).name)
    with zipfile.ZipFile(wheel) as z:
        for name in z.namelist():
            if name.endswith(".dist-info/WHEEL"):
                for line in z.read(name).decode("utf-8").splitlines():
                    if line.startswith("Tag:"):
                        print(line)
    return 0


def cmd_compare(a: str, b: str) -> int:
    for label, pattern in (("wheel", "oradmp-*.whl"), ("sdist", "oradmp-*.tar.gz")):
        pa, pb = find(a, pattern), find(b, pattern)
        ha, hb = sha256_file(pa), sha256_file(pb)
        print(f"{label} build 1: {ha}  {Path(pa).name}")
        print(f"{label} build 2: {hb}  {Path(pb).name}")
        if ha == hb:
            print(f"{label}: identical")
            continue
        ma, mb = members(pa), members(pb)
        differ = sorted(n for n in set(ma) | set(mb) if ma.get(n) != mb.get(n))
        if not differ:
            print(f"{label}: differs: no member content differs (names, order or archive metadata)")
        else:
            print(f"{label}: differs: {', '.join(differ)}")
            for n in differ:
                print(f"  {n}: build 1 {ma.get(n, ('absent', 0))}  build 2 {mb.get(n, ('absent', 0))}")
    return 0


def main(argv: list[str]) -> int:
    if len(argv) >= 2 and argv[0] == "checksums":
        return cmd_checksums(argv[1:])
    if len(argv) == 2 and argv[0] == "sums":
        return cmd_sums(argv[1])
    if len(argv) == 2 and argv[0] == "members":
        return cmd_members(argv[1])
    if len(argv) == 2 and argv[0] == "tags":
        return cmd_tags(argv[1])
    if len(argv) == 3 and argv[0] == "compare":
        return cmd_compare(argv[1], argv[2])
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
