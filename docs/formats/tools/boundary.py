#!/usr/bin/env python3
"""Which element of the table data lies on each multiple of RECORDLENGTH?

Checks (exp.md section 1.2, rule R5; request R-013): for every table of a legacy EXP fixture and
every file offset B that is a multiple of the RECORDLENGTH of the header and lies inside the row
data, prints the bytes at B-1 and B and the element they belong to:
  "inside length field"  the two bytes of a u16 length are B-1 and B
  "inside NULL marker"   the two bytes `fe ff` are B-1 and B
  "inside row end"       the two bytes `00 00` after a conventional-path row are B-1 and B
  "inside end marker"    the two bytes `ff ff` are B-1 and B
  "inside value"         B-1 and B are value bytes of the same field
  "between ..."          B is the first byte of an element
The walk is the continuous one of exp_parse.py (nothing is skipped at B), so "all rows equal the
truth" in check_truth.py together with this list is the test of "nothing is inserted, repeated or
changed at a record boundary".
Expects: one or more `*_exp_*.dmp` fixtures (written for t_bn1..t_bn4 and t_probe6 of
corpus/fixtures/18c and 11g, and for the RECORDLENGTH 2048 files of corpus/fixtures/11g-cp1251):
    uv run boundary.py ../../../corpus/fixtures/18c/t_bn1_exp_18c.dmp [-q]
-q prints only the count per class. Reads the fixtures, never writes them.
New in Analyst session 4.
Changed in Analyst session 8: LOB tables are no longer skipped. After a row of a LOB table the
elements are the u16 k ("row end / LOB count"), and per LOB value either a short length field + value,
or the marker `fc ff`, the 8-byte total length and the chunks (chunk length field + chunk bytes),
as exp_parse.py reads them (exp.md 14.6). Expects also lob_exp_<v>.dmp.
Changed in Analyst session 9: a LONG / LONG RAW field on the direct path is a chain of pieces (exp_parse.py
`long_pieces`); each piece is a "LONG piece length" element and a value element (exp.md 6.6). Expects also
long_exp_direct_<v>.dmp.
Changed in Analyst session 17: a conventional-path LONG value in the `fd ff` form (exp_parse.py `long_chunks`)
is the marker, then per chunk a "LONG chunk length" element and a value element, then the closing `00 00`
(exp.md 6.6, R-040). Expects also long3_exp(_direct)_<v>.dmp.
Changed in Analyst session 19: a conventional-path LONG in the `fa ff` form (exp_parse.py `long_fa`) is the marker,
the 4-byte length and the value (exp.md 6.6, R-048). Expects also long4_exp(_direct)_<v>.dmp, longraw4_exp(_direct)_18c.dmp.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402


def elements(t, d):
    """Yield (start, end, label) for every element of the row data of table t."""
    ncol = len(t["cols"])
    byrow: dict = {}
    for ri, ci, vo, buf, form, lens in t.get("lob_values", []):
        byrow.setdefault(ri, []).append((ci, vo, buf, form, lens))
    chains = {fo: pl for _ri, fo, pl in t.get("long_pieces", [])}
    chunked = {fo: cl for _ri, fo, cl in t.get("long_chunks", [])}
    fa = {fo: n for _ri, fo, n in t.get("long_fa", [])}
    for ri, (ro, row) in enumerate(t["rows"]):
        for ci, (fo, v) in enumerate(row):
            if fo in chains:
                # session 9: a direct-path LONG / LONG RAW value as a chain of pieces
                q = fo
                for n in chains[fo]:
                    yield q, q + 2, f"LONG piece length ({n:#06x}), row {ri + 1} col {ci + 1}"
                    yield q + 2, q + 2 + (n & 0x7FFF), f"value, LONG piece row {ri + 1} col {ci + 1}"
                    q += 2 + (n & 0x7FFF)
                continue
            if fo in chunked:
                # session 17: conventional LONG `fd ff` + chunks `u16 n + n bytes` + `00 00`
                yield fo, fo + 2, f"LONG marker fd ff, row {ri + 1} col {ci + 1}"
                q = fo + 2
                for n in chunked[fo]:
                    yield q, q + 2, f"LONG chunk length ({n}), row {ri + 1} col {ci + 1}"
                    if n:
                        yield q + 2, q + 2 + n, f"value, LONG chunk row {ri + 1} col {ci + 1}"
                    q += 2 + n
                continue
            if fo in fa:
                # session 19: conventional LONG `fa ff` + 4-byte length + value (R-048)
                yield fo, fo + 2, f"LONG marker fa ff, row {ri + 1} col {ci + 1}"
                yield fo + 2, fo + 6, f"LONG 4-byte length ({fa[fo]}), row {ri + 1} col {ci + 1}"
                yield fo + 6, fo + 6 + fa[fo], f"value, LONG fa form row {ri + 1} col {ci + 1}"
                continue
            if v is None:
                yield fo, fo + 2, f"NULL marker, row {ri + 1} col {ci + 1}"
            else:
                yield fo, fo + 2, f"length field ({len(v)}), row {ri + 1} col {ci + 1}"
                yield fo + 2, fo + 2 + len(v), f"value, row {ri + 1} col {ci + 1}"
        if not t["direct"]:
            last_fo, last_v = row[ncol - 1]
            e = last_fo + 2 + (0 if last_v is None else len(last_v))
            if last_fo in chunked:
                e = last_fo + 2 + sum(2 + n for n in chunked[last_fo])
            if last_fo in fa:
                e = last_fo + 6 + fa[last_fo]
            yield e, e + 2, f"row end 00 00, row {ri + 1}"
            for ci, vo, buf, form, lens in byrow.get(ri, []):
                if form == "short":
                    yield vo, vo + 2, f"LOB length field ({len(buf)}), row {ri + 1} col {ci + 1}"
                    if buf:
                        yield vo + 2, vo + 2 + len(buf), f"value, LOB row {ri + 1} col {ci + 1}"
                    continue
                yield vo, vo + 2, f"LOB marker fc ff, row {ri + 1} col {ci + 1}"
                yield vo + 2, vo + 10, f"LOB total length, row {ri + 1} col {ci + 1}"
                q = vo + 10
                for n in lens:
                    yield q, q + 2, f"LOB chunk length ({n}), row {ri + 1} col {ci + 1}"
                    yield q + 2, q + 2 + n, f"value, LOB chunk row {ri + 1} col {ci + 1}"
                    q += 2 + n
    if t["end_marker"] is not None:
        yield t["end_marker"], t["end_marker"] + 2, "end marker ff ff"


def main() -> None:
    quiet = "-q" in sys.argv
    for path in [a for a in sys.argv[1:] if not a.startswith("-")]:
        r = parse(path)
        d = open(path, "rb").read()
        R = r["header"]["reclen"]
        counts: dict = {}
        print(f"{os.path.basename(path)}: reclen {R}, problems {r['problems'] or 'none'}")
        for t in r["tables"]:
            els = list(elements(t, d))
            # the elements must tile the row data without a gap
            pos = t["rows_off"]
            for s, e, _l in els:
                assert s == pos, f"gap at 0x{pos:x}"
                pos = e
            assert pos == t["rows_end"], "elements do not end at the LF"
            b = (t["rows_off"] // R + 1) * R
            i = 0
            while b < t["rows_end"]:
                while els[i][1] <= b:
                    i += 1
                s, e, label = els[i]
                kind = label.split(",")[0].split(" (")[0]
                if s == b:
                    cls = f"between elements (B starts a {kind})"
                elif kind == "value":
                    cls = "inside value"
                else:
                    cls = f"inside {kind}"
                counts[cls] = counts.get(cls, 0) + 1
                if not quiet or not (cls == "inside value" or cls.startswith("between")):
                    print(f"  {t['name']} B=0x{b:x}: bytes at B-1, B = {d[b - 1]:02x} {d[b]:02x}; {cls}: "
                          f"{label} at 0x{s:x}-0x{e - 1:x}; bytes 0x{max(s, b - 4):x}.. = "
                          f"{d[max(s, b - 4):min(e, b + 4)].hex(' ')}")
                b += R
        for k in sorted(counts):
            print(f"    {counts[k]:4d} x {k}")


if __name__ == "__main__":
    main()
