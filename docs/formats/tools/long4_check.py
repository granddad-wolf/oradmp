#!/usr/bin/env python3
"""R-048: how a LONG of 65,521 to 65,535 bytes and a LONG RAW of 32,768 to 100,000 bytes are written.

Checks (exp.md 6.6, session 19; rules R2, R8, Y9): in long4_exp_<v>.dmp / long4_exp_direct_<v>.dmp
(corpus/fixtures/18c and 11g; table T_LONG4 = ID NUMBER(3), L LONG, V VARCHAR2(10)) and in
longraw4_exp_18c.dmp / longraw4_exp_direct_18c.dmp (18c only; T_LONGRAW4 = ID NUMBER(3), LR LONG RAW),
for every row: the offset of the LONG / LONG RAW field, its form
  - one field `u16 n + n bytes`,
  - `fa ff` + 4 bytes (u16 n + `00 00`) + n bytes (new in session 19),
  - `fd ff` + chunks `u16 n + n bytes` ended by `00 00`,
  - direct-path piece chain (bit 0x8000 = more),
every u16 of the form, and the value against truth_long4.txt / truth_longraw4.txt (length, first and
last 20 bytes of LONG / 10 bytes of LONG RAW, V) and against the arithmetic (LONG: `L` + 6 digits + `y` x (n - 7); LONG RAW: n x 0xab).
Then the T_FRAME section of each file against t_frame_exp(_direct)_<v>.dmp (rule T8, via frame_t8.py),
and whether the 18c and 11g files of a pair have the same bytes from 0x2000 on (count of differing bytes).
Expects: no argument (the six files), or fixture paths, e.g.
    .venv/bin/python long4_check.py ../../../corpus/fixtures/18c/long4_exp_18c.dmp
Uses exp_parse.py (session 19: it reads the conventional `fa ff` form). Reads the fixtures, never writes
them. New in Analyst session 19.
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
    files = sys.argv[1:] or sorted(
        glob.glob(os.path.join(here, "../../../corpus/fixtures/*/long4_exp_*.dmp"))
        + glob.glob(os.path.join(here, "../../../corpus/fixtures/*/longraw4_exp_*.dmp")))
    bad = 0
    for f in files:
        name = os.path.basename(f)
        v = name.rsplit("_", 1)[1].split(".")[0]
        raw = name.startswith("longraw4")
        tname, tfile, ltype = ("T_LONGRAW4", "truth_longraw4.txt", 24) if raw else ("T_LONG4", "truth_long4.txt", 8)
        truth = {}
        for ln in open(os.path.join(os.path.dirname(f), tfile)):
            p = ln.rstrip("\n").split("|")
            if len(p) >= 4 and p[0].isdigit():
                truth[int(p[0])] = (int(p[1]), bytes.fromhex(p[2]), bytes.fromhex(p[3]), p[4] if len(p) > 4 else None)
        d = open(f, "rb").read()
        r = parse(f)
        t = [x for x in r["tables"] if x["name"] == tname][0]
        ci_long = [i for i, c in enumerate(t["cols"]) if c["type"] == ltype][0]
        ci_v = [i for i, c in enumerate(t["cols"]) if c["type"] == 1]
        print(f"== {name}: flag {int(t['direct'])}, descriptor order {[c['type'] for c in t['cols']]}, "
              f"rows {len(t['rows'])}, end marker @0x{t['end_marker']:x}, problems {r['problems'] or 'none'}")
        pieces = {ri: pl for ri, _o, pl in t.get("long_pieces", [])}
        chunks = {ri: cl for ri, _o, cl in t.get("long_chunks", [])}
        fa = {ri: n for ri, _o, n in t.get("long_fa", [])}
        for ri, (ro, row) in enumerate(t["rows"]):
            rid = row[0][1]
            idv = (rid[1] - 1) if rid and rid[0] == 0xC1 else None
            fo, val = row[ci_long]
            if ri in pieces:
                pl = pieces[ri]
                form = "chain " + " ".join(f"{n:04x}" for n in pl) + " = " + " + ".join(
                    str(n & 0x7FFF) for n in pl)
            elif ri in chunks:
                cl = chunks[ri]
                form = f"fd ff + {len(cl) - 1} chunks " + " + ".join(str(n) for n in cl[:-1]) + " + end 0000"
            elif ri in fa:
                form = f"fa ff + {d[fo + 2:fo + 6].hex(' ')} (n = {fa[ri]}) + bytes @0x{fo + 6:x}"
            else:
                form = f"one field u16 {d[fo:fo + 2].hex(' ')} ({u16(d, fo)})"
            tl, th, tt, tv = truth[idv]
            exp_val = (b"\xab" * tl) if raw else (b"L%06d" % tl + b"y" * (tl - 7))
            # truth_longraw4.txt gives 10 bytes (20 hex digits) at each end, truth_long4.txt 20 bytes
            ok = len(val) == tl and val[:len(th)] == th and val[-len(tt):] == tt and val == exp_val
            vs = ""
            if ci_v:
                vo, vv = row[ci_v[0]]
                ok = ok and vv.decode() == tv
                vs = f"; V @0x{vo:x} {d[vo:vo + 2].hex(' ')} {vv.decode()!r}"
            bad += not ok
            print(f"  row {idv} @0x{ro:x}: LONG @0x{fo:x} len {len(val)}: {form}{vs}; truth {'OK' if ok else 'DIFF'}")
        tf, a, _ = frame_bytes(f)
        ref = os.path.join(os.path.dirname(f), f"t_frame_exp_{'direct_' if tf['direct'] else ''}{v}.dmp")
        _t2, b, _ = frame_bytes(ref)
        print(f"  T_FRAME @0x{tf['table_off']:x}..0x{tf['rows_end']:x} flag {int(tf['direct'])} vs "
              f"{os.path.basename(ref)}: {'IDENTICAL' if a == b else 'DIFFERENT'}")
        bad += a != b
        if v == "18c":
            other = f.replace("/18c/", "/11g/").replace("_18c.dmp", "_11g.dmp")
            if os.path.exists(other):
                e = open(other, "rb").read()
                n = sum(1 for x, y in zip(d[0x2000:], e[0x2000:]) if x != y) + abs(len(d) - len(e))
                print(f"  vs {os.path.basename(other)} from 0x2000: {n} bytes differ (sizes {len(d)} / {len(e)})")
    print(f"differences: {bad}")


if __name__ == "__main__":
    main()
