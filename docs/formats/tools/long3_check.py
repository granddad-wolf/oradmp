#!/usr/bin/env python3
"""R-040: how a LONG of 32,767 to 100,000 bytes is written, conventional and direct path.

Checks (exp.md 6.6, session 17; rules Y9, R2, R8): in long3_exp_<v>.dmp (path flag 0) and
long3_exp_direct_<v>.dmp (path flag 1) of corpus/fixtures/18c and 11g, for each of the 7 rows of
T_LONG3: the offset of the LONG field, its form (one field `u16 n + n bytes`; `fd ff` + chunks ended by
`00 00`; direct-path piece chain with bit 0x8000 = more), every u16 of the form, the sum of the bytes,
and compares the value with truth_long3.txt (length, first 20 bytes, last 20 bytes, V). Then the
T_FRAME section of each file against t_frame_exp(_direct)_<v>.dmp (rule T8, via frame_t8.py).
Expects: no argument (the four files of 18c and 11g), or the fixture paths, e.g.
    .venv/bin/python long3_check.py ../../../corpus/fixtures/18c/long3_exp_18c.dmp
Uses exp_parse.py (session 17: it reads the conventional `fd ff` chunk form). Reads the fixtures,
never writes them. New in Analyst session 17.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402
from frame_t8 import frame_bytes  # noqa: E402


def u16(d: bytes, p: int) -> int:
    return d[p] | (d[p + 1] << 8)


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    files = sys.argv[1:] or sorted(glob.glob(os.path.join(here, "../../../corpus/fixtures/*/long3_exp_*.dmp")))
    bad = 0
    for f in files:
        name = os.path.basename(f)
        v = name.rsplit("_", 1)[1].split(".")[0]
        truth = {}
        for ln in open(os.path.join(os.path.dirname(f), "truth_long3.txt")):
            p = ln.rstrip("\n").split("|")
            if len(p) == 5:
                truth[int(p[0])] = (int(p[1]), bytes.fromhex(p[2]), bytes.fromhex(p[3]), p[4])
        d = open(f, "rb").read()
        r = parse(f)
        t = [x for x in r["tables"] if x["name"] == "T_LONG3"][0]
        ci_long = [i for i, c in enumerate(t["cols"]) if c["type"] == 8][0]
        ci_v = [i for i, c in enumerate(t["cols"]) if c["type"] == 1][0]
        print(f"== {name}: flag {int(t['direct'])}, descriptor order "
              f"{[c['type'] for c in t['cols']]}, rows {len(t['rows'])}, end marker @0x{t['end_marker']:x}, "
              f"problems {r['problems'] or 'none'}")
        pieces = {ri: pl for ri, _o, pl in t.get("long_pieces", [])}
        chunks = {ri: cl for ri, _o, cl in t.get("long_chunks", [])}
        for ri, (_ro, row) in enumerate(t["rows"]):
            rid = row[0][1]
            idv = (rid[1] - 1) if rid and rid[0] == 0xC1 else None
            fo, val = row[ci_long]
            vo, vv = row[ci_v]
            if ri in pieces:
                pl = pieces[ri]
                form = "chain " + " ".join(f"{n:04x}" for n in pl) + " = " + " + ".join(
                    str(n & 0x7FFF) for n in pl)
            elif ri in chunks:
                cl = chunks[ri]
                form = (f"fd ff + {len(cl) - 1} chunks " + " + ".join(str(n) for n in cl[:-1]) + " + end 0000")
            else:
                form = f"one field u16 {u16(d, fo):04x} ({u16(d, fo)})"
            tl, th, tt, tv = truth[idv]
            ok = (len(val) == tl and val[:20] == th and val[-20:] == tt and vv.decode() == tv
                  and val == b"L%06d" % tl + b"y" * (tl - 7))
            bad += not ok
            print(f"  row {idv}: L @0x{fo:x} len {len(val)}: {form}; V @0x{vo:x} {vv.decode()!r}; "
                  f"truth {'OK' if ok else 'DIFF'}")
        tf, a, _ = frame_bytes(f)
        ref = os.path.join(os.path.dirname(f), f"t_frame_exp_{'direct_' if tf['direct'] else ''}{v}.dmp")
        _t2, b, _ = frame_bytes(ref)
        print(f"  T_FRAME @0x{tf['table_off']:x}..0x{tf['rows_end']:x} flag {int(tf['direct'])} vs "
              f"{os.path.basename(ref)}: {'IDENTICAL' if a == b else 'DIFFERENT'}")
        bad += a != b
    print(f"differences: {bad}")


if __name__ == "__main__":
    main()
