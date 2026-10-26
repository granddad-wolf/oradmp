#!/usr/bin/env python3
"""R-047: TIMESTAMP(f) for f = 1 to 5, 7, 8: descriptor length and value bytes against truth_tsp.txt.

Checks (exp.md 6.4, session 17; rules T5, Y5, R7): in tsp_exp_<v>.dmp and tsp_exp_direct_<v>.dmp of
corpus/fixtures/18c and 11g, table T_TSP:
- every column descriptor (type, maximum length) against truth_tsp.txt (data_type, data_length);
- every field of the 4 rows (u16 offset, length, bytes) against the decimal `DUMP(x)` of truth_tsp.txt
  (`Typ=180 Len=n: b1,...` or `NULL`; a NULL field must be `fe ff`);
- for row 3 (literal .123456789), whether the stored nanoseconds equal the value rounded or truncated to
  f digits, and that the `TO_CHAR ... FF9` lines agree with the decoded nanoseconds (t1, t3, t8);
- that the conventional and the direct file of one version have the same field bytes (rule R7), and
  that 18c and 11g have the same field bytes.
Expects: no argument (the four files), or tsp fixture paths, e.g.
    .venv/bin/python tsp_check.py ../../../corpus/fixtures/18c/tsp_exp_18c.dmp
Uses exp_parse.py. Reads the fixtures, never writes them. New in Analyst session 17.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

PREC = [None, 1, 2, 3, 4, 5, 7, 8]          # column index -> f (ID has none)
LIT = 123456789                              # row 3 literal, nanoseconds


def read_truth(path: str):
    cols, dumps, text = [], {}, {}
    for ln in open(path):
        p = ln.rstrip("\n").split("|")
        if len(p) == 4 and not p[0].isdigit():
            cols.append((p[0], p[1], int(p[2])))
        elif len(p) == 8:
            vals = []
            for x in p[1:]:
                if x == "NULL":
                    vals.append(None)
                else:
                    vals.append(bytes(int(b) for b in x.split(": ", 1)[1].split(",")))
            dumps[int(p[0])] = vals
        elif len(p) == 4 and p[0].isdigit():
            text[int(p[0])] = p[1:]
    return cols, dumps, text


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    files = sys.argv[1:] or sorted(glob.glob(os.path.join(here, "../../../corpus/fixtures/*/tsp_exp_*.dmp")))
    bad = 0
    fields_by_file = {}
    for f in files:
        name = os.path.basename(f)
        cols, dumps, text = read_truth(os.path.join(os.path.dirname(f), "truth_tsp.txt"))
        d = open(f, "rb").read()
        r = parse(f)
        t = [x for x in r["tables"] if x["name"] == "T_TSP"][0]
        print(f"== {name}: flag {int(t['direct'])}, rows {len(t['rows'])}, end marker @0x{t['end_marker']:x}, "
              f"problems {r['problems'] or 'none'}")
        desc = []
        for c, (cn, ct, cl) in zip(t["cols"], cols):
            ok = c["len"] == cl and c["type"] == (180 if ct.startswith("TIMESTAMP") else 2)
            bad += not ok
            desc.append(f"{cn} {c['type']}/{c['len']} (truth {ct} {cl}) {'OK' if ok else 'DIFF'}")
        print("  descriptors @0x%x: " % (t["post_desc"][0] - 4 * len(t["cols"])) + "; ".join(desc))
        fb = []
        for ri, (_ro, row) in enumerate(t["rows"], 1):
            out = []
            for ci, (fo, val) in enumerate(row):
                fb.append(d[fo:fo + 2 + (len(val) if val is not None else 0)])
                if ci == 0:
                    out.append(f"ID @0x{fo:x} {val.hex(' ')}")
                    continue
                exp = dumps[ri][ci - 1]
                if val is None:
                    ok = exp is None and d[fo:fo + 2] == b"\xfe\xff"
                    out.append(f"t{PREC[ci]} @0x{fo:x} fe ff {'OK' if ok else 'DIFF'}")
                else:
                    ok = val == exp
                    s = f"t{PREC[ci]} @0x{fo:x} [{len(val)}] {val.hex(' ')} {'OK' if ok else 'DIFF'}"
                    if ri == 3:
                        ns = int.from_bytes(val[7:11], "big")
                        q = 10 ** (9 - PREC[ci])
                        rnd = (LIT + q // 2) // q * q
                        trc = LIT // q * q
                        s += f" ns={ns} ({'rounded' if ns == rnd else ''}{'/' if ns == rnd == trc else ''}" \
                             f"{'truncated' if ns == trc else ''})"
                        ok = ok and ns == rnd
                    out.append(s)
                bad += not ok
            print(f"  row {ri}: " + " | ".join(out))
        # TO_CHAR FF9 lines (t1, t3, t8) against the decoded nanoseconds
        for ri, tx in text.items():
            row = t["rows"][ri - 1][1]
            for ci, s in zip((1, 3, 7), tx):
                val = row[ci][1]
                got = "" if val is None else ("%09d" % (int.from_bytes(val[7:11], "big") if len(val) == 11 else 0))
                ok = (s == "" and val is None) or (s.endswith("." + got))
                bad += not ok
                if not ok:
                    print(f"  TO_CHAR row {ri} t{PREC[ci]}: truth {s!r} dump {got!r} DIFF")
        fields_by_file[name] = fb
    names = sorted(fields_by_file)
    for a in names:
        for b in names:
            if a < b:
                same = fields_by_file[a] == fields_by_file[b]
                print(f"field bytes {a} vs {b}: {'IDENTICAL' if same else 'DIFFERENT'}")
                bad += not same
    print(f"differences: {bad}")


if __name__ == "__main__":
    main()
