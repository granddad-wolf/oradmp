#!/usr/bin/env python3
"""R-051: conventional LONG of 65,526 to 65,531 bytes; LONG RAW of 65,521 to 65,534 bytes (18c, 11g); LONG RAW on 11g.

Checks (exp.md 6.6 and 5.6, session 20; rules R2, R8, Y9): in
  long5_exp_<v>.dmp / long5_exp_direct_<v>.dmp (corpus/fixtures/18c and 11g; T_LONG5 = ID NUMBER(3), L LONG,
    V VARCHAR2(10); 6 rows, 65,526 to 65,531 bytes, `L` + 6 digits + `y` x (n - 7), V = `v` + n), and
  longraw5_exp_<v>.dmp / longraw5_exp_direct_<v>.dmp (T_LONGRAW5 = ID NUMBER(3), LR LONG RAW; n x 0xab;
    18c: 7 rows 65,521 .. 65,534; 11g: the same 7 rows plus ids 8 to 11 = 32,768, 65,520, 65,535, 100,000),
for every row: the offset of the LONG / LONG RAW field, its form (one field `u16 n + n bytes`; `fa ff` + 4 bytes
+ n bytes; `fd ff` + chunks `u16 n + n bytes` + `00 00`; direct-path piece chain, bit 0x8000 = more), the first
8 bytes of the field, every u16 of the form, and the value against truth_long5.txt / truth_longraw5.txt (length,
first and last 20 bytes of LONG / 10 bytes of LONG RAW, V) and against the arithmetic.
Then the T_FRAME section of each file against t_frame_exp(_direct)_<v>.dmp (rule T8, via frame_t8.py), and
whether the 18c and 11g files of a pair have the same bytes from 0x2000 on (count of differing bytes; for
longraw5 the 11g file has 4 more rows, so the count is not expected to be 0 there).
Finally, field by field (the whole encoded field, from its first u16 to its last byte): longraw5_exp(_direct)_11g
ids 1 to 7 against longraw5_exp(_direct)_18c, and ids 8 to 11 against longraw4_exp(_direct)_18c ids 1 to 4.
Expects: no argument (the eight files under corpus/fixtures/18c and 11g, and longraw4_exp(_direct)_18c.dmp for
the last comparison), or fixture paths of long5 / longraw5 files, e.g.
    .venv/bin/python long5_check.py ../../../corpus/fixtures/11g/longraw5_exp_11g.dmp
Uses exp_parse.py and frame_t8.py. Reads the fixtures, never writes them. New in Analyst session 20.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402
from frame_t8 import frame_bytes  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = os.path.join(HERE, "../../../corpus/fixtures")


def u16(d: bytes, p: int) -> int:
    return d[p] | (d[p + 1] << 8)


def fields(path: str):
    """Return (data, table, list of (id, field offset, field end, form text, value, V or None))."""
    name = os.path.basename(path)
    raw = name.startswith("longraw")
    tname = {"longraw5": "T_LONGRAW5", "longraw4": "T_LONGRAW4", "long5": "T_LONG5"}[name.split("_")[0]]
    ltype = 24 if raw else 8
    d = open(path, "rb").read()
    r = parse(path)
    t = [x for x in r["tables"] if x["name"] == tname][0]
    ci_long = [i for i, c in enumerate(t["cols"]) if c["type"] == ltype][0]
    ci_v = [i for i, c in enumerate(t["cols"]) if c["type"] == 1]
    pieces = {ri: pl for ri, _o, pl in t.get("long_pieces", [])}
    chunks = {ri: cl for ri, _o, cl in t.get("long_chunks", [])}
    fa = {ri: n for ri, _o, n in t.get("long_fa", [])}
    out = []
    for ri, (_ro, row) in enumerate(t["rows"]):
        rid = row[0][1]
        idv = (rid[1] - 1) if rid and rid[0] == 0xC1 else None
        fo, val = row[ci_long]
        if ri in pieces:
            pl = pieces[ri]
            end = fo + sum(2 + (n & 0x7FFF) for n in pl)
            form = "chain " + " ".join(f"{n:04x}" for n in pl) + " = " + " + ".join(str(n & 0x7FFF) for n in pl)
        elif ri in chunks:
            cl = chunks[ri]
            end = fo + 2 + sum(2 + n for n in cl)
            form = f"fd ff + {len(cl) - 1} chunks " + " + ".join(str(n) for n in cl[:-1]) + " + end 0000"
        elif ri in fa:
            end = fo + 6 + fa[ri]
            form = f"fa ff + {d[fo + 2:fo + 6].hex(' ')} (n = {fa[ri]}) + bytes @0x{fo + 6:x}"
        else:
            end = fo + 2 + u16(d, fo)
            form = f"one field u16 {d[fo:fo + 2].hex(' ')} ({u16(d, fo)})"
        vv = None
        if ci_v:
            vo, vb = row[ci_v[0]]
            vv = (vo, vb.decode())
        out.append((idv, fo, end, form, val, vv))
    return d, t, r, out


def check_file(f: str) -> int:
    name = os.path.basename(f)
    v = name.rsplit("_", 1)[1].split(".")[0]
    raw = name.startswith("longraw")
    tfile = "truth_longraw5.txt" if raw else "truth_long5.txt"
    truth = {}
    for ln in open(os.path.join(os.path.dirname(f), tfile)):
        p = ln.rstrip("\n").split("|")
        if len(p) >= 4 and p[0].isdigit():
            truth[int(p[0])] = (int(p[1]), bytes.fromhex(p[2]), bytes.fromhex(p[3]), p[4] if len(p) > 4 else None)
    d, t, r, rows = fields(f)
    bad = 0
    print(f"== {name}: flag {int(t['direct'])}, descriptor order {[c['type'] for c in t['cols']]}, "
          f"rows {len(rows)}, end marker @0x{t['end_marker']:x}, problems {r['problems'] or 'none'}")
    for idv, fo, _end, form, val, vv in rows:
        tl, th, tt, tv = truth[idv]
        exp_val = (b"\xab" * tl) if raw else (b"L%06d" % tl + b"y" * (tl - 7))
        ok = len(val) == tl and val[:len(th)] == th and val[-len(tt):] == tt and val == exp_val
        vs = ""
        if vv is not None:
            ok = ok and vv[1] == tv
            vs = f"; V @0x{vv[0]:x} {d[vv[0]:vv[0] + 2].hex(' ')} {vv[1]!r}"
        bad += not ok
        print(f"  id {idv} n {tl}: field @0x{fo:x} first 8 bytes {d[fo:fo + 8].hex(' ')}: {form}{vs}; "
              f"truth {'OK' if ok else 'DIFF'}")
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
    return bad


def cross_11g() -> int:
    """longraw5 11g fields vs longraw5 18c (ids 1-7) and longraw4 18c (ids 8-11 = longraw4 ids 1-4)."""
    bad = 0
    for path in ("", "direct_"):
        g = os.path.join(FIX, "11g", f"longraw5_exp_{path}11g.dmp")
        c5 = os.path.join(FIX, "18c", f"longraw5_exp_{path}18c.dmp")
        c4 = os.path.join(FIX, "18c", f"longraw4_exp_{path}18c.dmp")
        if not all(os.path.exists(x) for x in (g, c5, c4)):
            continue
        dg, _, _, rg = fields(g)
        d5, _, _, r5 = fields(c5)
        d4, _, _, r4 = fields(c4)
        ref = {i: (d5, fo, end) for i, fo, end, *_ in r5}
        ref.update({i + 7: (d4, fo, end) for i, fo, end, *_ in r4})
        refname = {i: os.path.basename(c5) if i <= 7 else os.path.basename(c4) + f" id {i - 7}" for i in ref}
        print(f"== field bytes of {os.path.basename(g)} vs the 18c files")
        for i, fo, end, *_ in rg:
            rd, rfo, rend = ref[i]
            same = dg[fo:end] == rd[rfo:rend]
            bad += not same
            print(f"  id {i}: 11g @0x{fo:x}..0x{end:x} ({end - fo} bytes) vs {refname[i]} @0x{rfo:x}..0x{rend:x}: "
                  f"{'IDENTICAL' if same else 'DIFFERENT'}")
    return bad


def main() -> None:
    files = sys.argv[1:] or sorted(glob.glob(os.path.join(FIX, "*/long5_exp_*.dmp"))
                                   + glob.glob(os.path.join(FIX, "*/longraw5_exp_*.dmp")))
    bad = sum(check_file(f) for f in files)
    if not sys.argv[1:]:
        bad += cross_11g()
    print(f"differences: {bad}")


if __name__ == "__main__":
    main()
