#!/usr/bin/env python3
"""Where `exp` switches a LOB value from the short form (`u16 n`) to the long form (`fc ff` + 8-byte total + chunks).

Checks (Analyst session 9, request R-035; exp.md 14.6, rules L4, L5): for every LOB value of T_LOB3 in the given
lob3_exp_<v>.dmp files (walked by exp_parse.py), prints id, column, offset, form, byte length, the number and
sizes of the chunks, and against truth_lob3.txt (`T_LOB3|id|GETLENGTH(c)|GETLENGTH(b)|GETLENGTH(n)|DUMP(SUBSTR(c,20,1),1016)`):
- the byte length (CLOB / NCLOB: 2 x GETLENGTH, BLOB: GETLENGTH);
- the hypothesis "long iff the byte length is larger than one chunk", where the chunk size of a column is the
  first chunk of its long values in the same file (they must all agree); the largest short value per column;
- for the long values: every chunk but the last has the chunk size, the last is the rest, the 8-byte total
  equals the byte length and its upper 4 bytes are zero.
Expects: lob3_exp_18c.dmp, lob3_exp_11g.dmp, e.g.
    uv run lob3_switch.py ../../../corpus/fixtures/18c/lob3_exp_18c.dmp ../../../corpus/fixtures/11g/lob3_exp_11g.dmp
Uses exp_parse.py. Reads the fixtures, never writes them. New in Analyst session 9.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402


def num_int(b: bytes) -> int:
    e = b[0] - 0xC0
    digs = [x - 1 for x in b[1:]] + [0] * (e - (len(b) - 1))
    v = 0
    for x in digs[:e]:
        v = v * 100 + x
    return v


def main() -> None:
    ok = bad = 0

    def rep(name: str, good: bool, detail: str = "") -> None:
        nonlocal ok, bad
        ok += good
        bad += not good
        if not good or "-v" in sys.argv:
            print(f"    {name}: {'OK' if good else 'DIFF'} {detail}")

    for path in (a for a in sys.argv[1:] if not a.startswith("-")):
        r = parse(path)
        d = open(path, "rb").read()
        tdir = os.path.dirname(os.path.abspath(path))
        truth = {int(f[1]): f for f in (ln.split("|") for ln in open(os.path.join(tdir, "truth_lob3.txt"),
                                                                      encoding="utf-8").read().split("\n") if ln)}
        t = [x for x in r["tables"] if x["name"] == "T_LOB3"][0]
        names = {1: "C (CLOB)", 2: "B (BLOB)", 3: "N (NCLOB)"}
        print(f"{os.path.basename(path)}: problems {r['problems'] or 'none'}")
        chunk: dict = {}
        for _ri, ci, _vo, _buf, form, lens in t["lob_values"]:
            if form == "long":
                chunk.setdefault(ci, set()).add(lens[0])
        for ci, s in chunk.items():
            rep(f"{names[ci]} one chunk size in the file", len(s) == 1, str(sorted(s)))
        chunk = {ci: min(s) for ci, s in chunk.items()}
        print(f"  chunk size per column: " + ", ".join(f"{names[c]} {n}" for c, n in sorted(chunk.items())))
        largest_short: dict = {}
        print("    id col          offset    form  bytes  chunks")
        for ri, ci, vo, buf, form, lens in t["lob_values"]:
            i = num_int(t["rows"][ri][1][0][1])
            f = truth[i]
            want = (2 * int(f[2]) if ci == 1 else int(f[3]) if ci == 2 else 2 * int(f[4]))
            ct = f"{len(lens)} x " + (str(lens[0]) if len(set(lens)) == 1 else f"{lens[0]} .. last {lens[-1]}")
            print(f"    {i:2d} {names[ci]:10s} 0x{vo:06x} {form:5s} {len(buf):6d}  {ct}")
            rep(f"id {i} {names[ci]} byte length = truth", len(buf) == want, f"{len(buf)} / {want}")
            if form == "short":
                largest_short[ci] = max(largest_short.get(ci, 0), len(buf))
            if ci in chunk:
                rep(f"id {i} {names[ci]} long iff > one chunk", (form == "long") == (len(buf) > chunk[ci]),
                    f"{len(buf)} vs chunk {chunk[ci]}")
            if form == "long":
                total = int.from_bytes(d[vo + 2:vo + 10], "little")
                rep(f"id {i} {names[ci]} total = length, upper 4 bytes 0", total == len(buf) and d[vo + 6:vo + 10] == b"\0" * 4,
                    d[vo + 2:vo + 10].hex(" "))
                rep(f"id {i} {names[ci]} chunks full but the last", all(n == chunk[ci] for n in lens[:-1])
                    and 0 < lens[-1] <= chunk[ci], str(lens))
        print("  largest short value per column: " + ", ".join(f"{names[c]} {n}" for c, n in sorted(largest_short.items())))
    print(f"TOTAL OK {ok}, DIFF {bad}")


if __name__ == "__main__":
    main()
