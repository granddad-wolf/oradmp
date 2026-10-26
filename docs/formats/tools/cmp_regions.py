#!/usr/bin/env python3
"""Compare two dumps byte by byte (like `cmp -l`) and group the differences into ranges.

Checks: (1) which contiguous offset ranges differ at the same position, with the bytes of both
files; (2) a shift-aware alignment (difflib) that shows inserted / deleted / replaced byte runs,
so that an insertion of a few bytes is not reported as one long differing region.
Expects: two legacy EXP fixtures of the same table, e.g.
    uv run cmp_regions.py ../../../corpus/fixtures/18c/t_frame_exp_18c.dmp \
                          ../../../corpus/fixtures/18c/t_frame_exp_direct_18c.dmp
Exploration tool only. Reads the fixtures, never writes them.
"""
import difflib
import sys


def show(b: bytes) -> str:
    return b.hex(" ") if len(b) <= 24 else b[:24].hex(" ") + f" ...({len(b)} bytes)"


def main() -> None:
    a = open(sys.argv[1], "rb").read()
    b = open(sys.argv[2], "rb").read()
    gap = int(sys.argv[3]) if len(sys.argv) > 3 else 1
    print(f"A={sys.argv[1]} ({len(a)} bytes)\nB={sys.argv[2]} ({len(b)} bytes)")
    diffs = [i for i in range(min(len(a), len(b))) if a[i] != b[i]]
    print(f"\n== positional differences: {len(diffs)} bytes")
    ranges = []
    for i in diffs:
        if ranges and i - ranges[-1][1] <= gap:
            ranges[-1][1] = i
        else:
            ranges.append([i, i])
    for s, e in ranges:
        print(f"0x{s:04x}-0x{e:04x} ({e - s + 1:4d})  A[{show(a[s:e + 1])}]  B[{show(b[s:e + 1])}]")
    print("\n== shift-aware alignment (non-equal opcodes)")
    sm = difflib.SequenceMatcher(None, a, b, autojunk=False)
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            continue
        print(f"{tag:8s} A@0x{i1:04x}+{i2 - i1:<3d} [{show(a[i1:i2])}]"
              f"  B@0x{j1:04x}+{j2 - j1:<3d} [{show(b[j1:j2])}]")


if __name__ == "__main__":
    main()
