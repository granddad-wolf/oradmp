#!/usr/bin/env python3
"""Locate every non-NULL value of a truth file inside a dump and print its offsets.

Checks: where each truth value (encoded as UTF-8) occurs in the dump, the 8 bytes before
and the 4 bytes after each occurrence.
Expects: a legacy EXP fixture and its truth file, e.g.
    uv run find_truth_values.py ../../../corpus/fixtures/18c/t_frame_exp_18c.dmp \
                                ../../../corpus/fixtures/18c/truth_frame.txt
Exploration tool only. Reads the fixtures, never writes them.
"""
import sys


def main() -> None:
    dump_path, truth_path = sys.argv[1], sys.argv[2]
    data = open(dump_path, "rb").read()
    rows = [line.rstrip("\n").split("|") for line in open(truth_path, encoding="utf-8")]
    print(f"{dump_path}: {len(data)} bytes, {len(rows)} truth rows")
    for r, row in enumerate(rows, 1):
        for c, val in enumerate(row, 1):
            if val == "<NULL>":
                print(f"row {r} col {c}: <NULL>")
                continue
            needle = val.encode("utf-8")
            hits = []
            pos = data.find(needle)
            while pos != -1:
                hits.append(pos)
                pos = data.find(needle, pos + 1)
            for h in hits:
                before = data[max(0, h - 8):h].hex(" ")
                after = data[h + len(needle):h + len(needle) + 4].hex(" ")
                print(f"row {r} col {c}: {val!r} len={len(needle)} @0x{h:04x}"
                      f"  before[{before}]  after[{after}]")
            if not hits:
                print(f"row {r} col {c}: {val!r} NOT FOUND")


if __name__ == "__main__":
    main()
