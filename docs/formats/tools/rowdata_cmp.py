#!/usr/bin/env python3
"""Are the row data of one table the same bytes in exports made with different client sets?

Checks (exp.md sections 2.1, 6.3, 14.1; Part F1 of session 4): for every table name that occurs
in several of the given legacy EXP fixtures, compares the binary part of the table section
(column count, descriptors, the bytes before the rows, all rows, end marker) byte for byte
between the files of the same export path, and prints for each file the client character set id
(header 0x0001), the id at header 0x0032, the offsets of the region and its SHA-256 prefix.
Also tests the survey statement "no value is valid UTF-8, all decode as cp1251" for VARCHAR2 and
CHAR values: counts values that are pure ASCII, valid UTF-8 with non-ASCII bytes, and not UTF-8.
Tables are matched by their position in the file when the name is not ASCII (the name of a table
is written in the client character set, exp.md 14.3).
Expects: the fixtures to compare, e.g.
    uv run rowdata_cmp.py ../../../corpus/fixtures/11g-cp1251/ru_tbl_exp_*.dmp
Reads the fixtures, never writes them. New in Analyst session 4.
"""
import hashlib
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

groups: dict = {}
for path in sys.argv[1:]:
    r = parse(path)
    d = open(path, "rb").read()
    h = r["header"]
    for t in r["tables"]:
        nm = t["name"] if t["name"].isascii() and "?" not in t["name"] else "(non-ASCII or replaced name)"
        key = (nm, "direct" if t["direct"] else "conventional")
        region = d[t["bin_off"]:t["rows_end"] + 1]
        st = {"ascii": 0, "utf8": 0, "notutf8": 0}
        for _ro, row in t["rows"]:
            for c, (_fo, v) in zip(t["cols"], row):
                if v is None or c["type"] not in (1, 96) or c.get("csform") != 1:
                    continue
                if v.isascii():
                    st["ascii"] += 1
                else:
                    try:
                        v.decode("utf-8")
                        st["utf8"] += 1
                    except UnicodeDecodeError:
                        st["notutf8"] += 1
        groups.setdefault(key, []).append((os.path.basename(path), h["cs_at_1"], h["cs_b"], t["bin_off"],
                                           t["rows_off"], t["rows_end"], region, len(t["rows"]), st))
for (name, pth), lst in sorted(groups.items()):
    print(f"table {name}, {pth} path: {len(lst)} file(s)")
    ref = lst[0][6]
    for fn, cs, csb, bo, ro, re_, region, n, st in lst:
        same = "= first" if region == ref else "DIFFERS from first"
        if region != ref and len(region) == len(ref):
            i = next(i for i in range(len(ref)) if ref[i] != region[i])
            same += f" (same length, first difference at region offset 0x{i:x})"
        print(f"  {fn:38s} client {cs:4d} hdr0x32 {csb:4d} binary part 0x{bo:x}-0x{re_:x} rows from 0x{ro:x}, "
              f"{n} rows, sha256 {hashlib.sha256(region).hexdigest()[:12]} {same}; "
              f"V/C values: ascii {st['ascii']}, valid UTF-8 non-ASCII {st['utf8']}, not UTF-8 {st['notutf8']}")
