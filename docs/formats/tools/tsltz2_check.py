#!/usr/bin/env python3
"""R-050: TIMESTAMP WITH LOCAL TIME ZONE (type 231) with f = 0, 3, 9: descriptor length and value bytes.

Checks (exp.md 6.5, session 19; rules T5, Y5, Y8, R7): in tsltz2_exp_<v>.dmp and tsltz2_exp_direct_<v>.dmp of
corpus/fixtures/18c and 11g, table T_TSLTZ2 (ID NUMBER(2), L0 / L3 / L9 TIMESTAMP(0 / 3 / 9) WITH LOCAL TIME ZONE):
- every column descriptor (type, maximum length) against truth_tsltz2.txt (data_type, data_length);
- every field of the 4 rows (offset, length, bytes) against the decimal `DUMP(x)` of truth_tsltz2.txt
  (`Typ=231 Len=n: b1,...` or `NULL`; a NULL field must be `fe ff`);
- the R-050 expectations written in docs/formats/requests.md (the bytes computed there from the literals:
  `78 78 01 02 04 05 06`, + `07 54 d4 c0`, + `07 5b cd 15`, + `23 c3 46 00`, and `... 05 07` for L0 row 3);
- that the `TO_CHAR ... FF9` lines agree with the decoded value (seconds and nanoseconds);
- that the descriptor length is 7 for f = 0 and 11 for f > 0 (the reader's rule) and that every value is
  7 or 11 bytes;
- that the field bytes are the same on both paths and both versions (rule R7);
- the first truth line (dbtimezone | sessiontimezone).
Expects: no argument (the four files), or tsltz2 fixture paths, e.g.
    .venv/bin/python tsltz2_check.py ../../../corpus/fixtures/18c/tsltz2_exp_18c.dmp
Uses exp_parse.py. Reads the fixtures, never writes them. New in Analyst session 19.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

PREC = [None, 0, 3, 9]                       # column index -> f (ID has none)
BASE = bytes([0x78, 0x78, 0x01, 0x02, 0x04, 0x05, 0x06])   # 2020-01-02 03:04:05 (R-050 text)
EXPECT = {                                   # R-050 "Expected" list, row -> (L0, L3, L9)
    1: (BASE, BASE, BASE),
    2: (BASE, BASE + bytes.fromhex("0754d4c0"), BASE + bytes.fromhex("075bcd15")),
    3: (BASE[:6] + b"\x07", BASE + bytes.fromhex("23c34600"), BASE + bytes.fromhex("23c34600")),
    4: (None, None, None),
}


def read_truth(path: str):
    env, cols, dumps, text = None, [], {}, {}
    for ln in open(path):
        p = ln.rstrip("\n").split("|")
        if len(p) == 2 and env is None:
            env = tuple(p)
        elif len(p) == 4 and not p[0].isdigit():
            cols.append((p[0], p[1], int(p[2])))
        elif len(p) == 4 and p[0].isdigit() and (p[1].startswith("Typ=") or p[1] == "NULL"):
            dumps[int(p[0])] = [None if x == "NULL" else bytes(int(b) for b in x.split(": ", 1)[1].split(","))
                                for x in p[1:]]
        elif len(p) == 4 and p[0].isdigit():
            text[int(p[0])] = p[1:]
    return env, cols, dumps, text


def as_text(b: bytes) -> str:
    y = (b[0] - 100) * 100 + b[1] - 100
    ns = int.from_bytes(b[7:11], "big") if len(b) == 11 else 0
    return f"{y:04d}-{b[2]:02d}-{b[3]:02d} {b[4] - 1:02d}:{b[5] - 1:02d}:{b[6] - 1:02d}.{ns:09d}"


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    files = sys.argv[1:] or sorted(glob.glob(os.path.join(here, "../../../corpus/fixtures/*/tsltz2_exp_*.dmp")))
    bad = 0
    fields_by_file = {}
    for f in files:
        name = os.path.basename(f)
        env, cols, dumps, text = read_truth(os.path.join(os.path.dirname(f), "truth_tsltz2.txt"))
        d = open(f, "rb").read()
        r = parse(f)
        t = [x for x in r["tables"] if x["name"] == "T_TSLTZ2"][0]
        print(f"== {name}: flag {int(t['direct'])}, rows {len(t['rows'])}, end marker @0x{t['end_marker']:x}, "
              f"problems {r['problems'] or 'none'}; truth dbtimezone|sessiontimezone {'|'.join(env)}")
        desc = []
        for ci, (c, (cn, ct, cl)) in enumerate(zip(t["cols"], cols)):
            ok = c["len"] == cl and c["type"] == (231 if "LOCAL TIME ZONE" in ct else 2)
            if ci:
                ok = ok and c["len"] == (7 if PREC[ci] == 0 else 11)
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
                req = EXPECT[ri][ci - 1]
                if val is None:
                    ok = exp is None and req is None and d[fo:fo + 2] == b"\xfe\xff"
                    out.append(f"L{PREC[ci]} @0x{fo:x} fe ff {'OK' if ok else 'DIFF'}")
                else:
                    ok = val == exp and val == req and len(val) in (7, 11) and d[fo:fo + 2] == bytes([len(val), 0])
                    out.append(f"L{PREC[ci]} @0x{fo:x} {d[fo:fo + 2].hex(' ')} + {val.hex(' ')} "
                               f"{'OK' if ok else 'DIFF'}")
                bad += not ok
            print(f"  row {ri}: " + " | ".join(out))
        for ri, tx in text.items():
            row = t["rows"][ri - 1][1]
            for ci, s in zip((1, 2, 3), tx):
                val = row[ci][1]
                ok = (s == "" and val is None) or (val is not None and s == as_text(val))
                bad += not ok
                if not ok:
                    print(f"  TO_CHAR row {ri} L{PREC[ci]}: truth {s!r} dump {as_text(val) if val else None!r} DIFF")
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
