#!/usr/bin/env python3
"""Compare the text items of two legacy EXP fixtures item by item (written for the R-016 files).

Checks (exp.md sections 4.6 and 13.9, Analyst session 5): what differs between two exports of the
same tables that differ in one export option (statistics=estimate / compute / none, conventional /
direct path, client character set) or in the Oracle version. The items of both files (lines and
fields of exp_parse.py, rule `keyword`) are aligned with difflib; the tool prints
- the number of items of each file and of items that are byte-identical,
- every item that exists in one file only (offset, form, first 90 bytes),
- every pair of items at the same position whose bytes differ, with both texts cut around the
  first differing byte,
- for every table: whether the binary part (column count to the LF after the rows) is identical.
It does not interpret anything; the reading is in exp.md.
Expects: two `*_exp_*.dmp` fixtures, e.g.
    uv run stat_cmp.py ../../../corpus/fixtures/18c/stat_exp_18c.dmp \
                       ../../../corpus/fixtures/18c/stat_exp_none_18c.dmp
Option: -q prints only the counts. Uses exp_parse.py from the same directory.
Exploration tool only. Reads the fixtures, never writes them. New in Analyst session 5.
"""
import difflib
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402


def show(it: dict, at: int = 0) -> str:
    raw = it["raw"]
    lo = max(0, at - 30)
    txt = raw[lo:lo + 90].decode("latin-1").replace("\n", "\\n")
    return f"0x{it['off']:x} {it['kind']} [{len(raw)}] {'..' if lo else ''}{txt!r}"


def main() -> None:
    quiet = "-q" in sys.argv
    a, b = [x for x in sys.argv[1:] if not x.startswith("-")]
    ra, rb = parse(a, "keyword"), parse(b, "keyword")
    ia, ib = ra["items"], rb["items"]
    # alignment key: form + text with digits removed, so that a changed number still aligns
    def key(it):
        t = it["raw"].decode("latin-1")
        return it["kind"] + "|" + "".join("#" if c.isdigit() else c for c in t)[:60]
    sm = difflib.SequenceMatcher(None, [key(x) for x in ia], [key(x) for x in ib], autojunk=False)
    same = diff = 0
    only_a, only_b, pairs = [], [], []
    for tag, i1, i2, j1, j2 in sm.get_opcodes():
        if tag == "equal":
            for x, y in zip(ia[i1:i2], ib[j1:j2]):
                if x["raw"] == y["raw"]:
                    same += 1
                else:
                    diff += 1
                    pairs.append((x, y))
        else:
            n = min(i2 - i1, j2 - j1) if tag == "replace" else 0
            for x, y in zip(ia[i1:i1 + n], ib[j1:j1 + n]):
                if x["kind"] == y["kind"] and x["raw"].split(b" ")[0] == y["raw"].split(b" ")[0]:
                    diff += 1
                    pairs.append((x, y))
                else:
                    only_a.append(x)
                    only_b.append(y)
            only_a += ia[i1 + n:i2]
            only_b += ib[j1 + n:j2]
    print(f"A {os.path.basename(a)}: {len(ia)} items; B {os.path.basename(b)}: {len(ib)} items; "
          f"identical {same}, differing {diff}, only in A {len(only_a)}, only in B {len(only_b)}")
    ta = {t["name"]: t for t in ra["tables"]}
    for t in rb["tables"]:
        u = ta.get(t["name"])
        if u is None:
            continue
        da = open(a, "rb").read()[u["bin_off"]:u["rows_end"]]
        db = open(b, "rb").read()[t["bin_off"]:t["rows_end"]]
        print(f"  binary part of {t['name']}: A 0x{u['bin_off']:x}-0x{u['rows_end']:x}, B 0x{t['bin_off']:x}-0x{t['rows_end']:x}: "
              f"{'identical' if da == db else 'DIFFERENT'} ({len(da)} / {len(db)} bytes)")
    if quiet:
        return
    for x in only_a:
        print("  only A:", show(x))
    for y in only_b:
        print("  only B:", show(y))
    for x, y in pairs:
        n = next((k for k, (p, q) in enumerate(zip(x["raw"], y["raw"])) if p != q), min(len(x["raw"]), len(y["raw"])))
        print(f"  differ at byte {n}:\n      A {show(x, n)}\n      B {show(y, n)}")


if __name__ == "__main__":
    main()
