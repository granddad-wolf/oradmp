#!/usr/bin/env python3
"""Rule T8: is the T_FRAME section of a multi-table file byte-identical to T_FRAME of t_frame_exp_<v>.dmp?

Checks (exp.md 4, 4.5, 14.6; rule T8 of exp_reader_rules.md; generalises `lob_check.py --frame`): for every
given `*_exp_*.dmp` file that holds a table T_FRAME next to other tables, the bytes from its `TABLE "T_FRAME"`
line to the LF after its rows against the same bytes of t_frame_exp_<v>.dmp (path flag 0) or
t_frame_exp_direct_<v>.dmp (path flag 1) of the same folder. Prints IDENTICAL / DIFFERENT and, when
different, the first differing offset of both files.
Expects: fixture files of corpus/fixtures/18c or 11g (no argument: every `*_exp_*.dmp` of those two folders
that has T_FRAME and at least one other table), e.g.
    uv run frame_t8.py ../../../corpus/fixtures/18c/lob3_exp_18c.dmp
Uses exp_parse.py. Reads the fixtures, never writes them. New in Analyst session 9.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402


def frame_bytes(path: str):
    r = parse(path)
    d = open(path, "rb").read()
    tf = [t for t in r["tables"] if t["name"] == "T_FRAME"]
    if not tf:
        return None, None, r
    t = tf[0]
    return t, d[t["table_off"]:t["rows_end"] + 1], r


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    files = sys.argv[1:] or sorted(f for v in ("18c", "11g")
                                   for f in glob.glob(os.path.join(here, "../../../corpus/fixtures", v, "*_exp_*.dmp")))
    same = diff = 0
    for f in files:
        name = os.path.basename(f)
        if name.startswith("t_frame_"):
            continue
        t, a, r = frame_bytes(f)
        if t is None or len({x["name"] for x in r["tables"]}) < 2:
            continue
        v = name.rsplit("_", 1)[1].split(".")[0]
        ref = os.path.join(os.path.dirname(f), f"t_frame_exp_{'direct_' if t['direct'] else ''}{v}.dmp")
        t2, b, _ = frame_bytes(ref)
        if a == b:
            same += 1
            res = "IDENTICAL"
        else:
            diff += 1
            i = next((k for k in range(min(len(a), len(b))) if a[k] != b[k]), min(len(a), len(b)))
            res = f"DIFFERENT first at {name}@0x{t['table_off'] + i:x} / {os.path.basename(ref)}@0x{t2['table_off'] + i:x}"
        print(f"{name:40s} T_FRAME @0x{t['table_off']:x}..0x{t['rows_end']:x} ({len(a)} bytes, flag "
              f"{int(t['direct'])}) vs {os.path.basename(ref)}: {res}")
    print(f"identical {same}, different {diff}")


if __name__ == "__main__":
    main()
