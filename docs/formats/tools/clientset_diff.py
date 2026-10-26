#!/usr/bin/env python3
"""Show what changes in a legacy EXP file when only the client character set (NLS_LANG) changes.

Checks (Analyst session 3, exp.md sections 2.1, 4.1, 6.3): compares a table exported with the
AL32UTF8 client against the same table exported with another client set, byte by byte, and
reports (a) every differing offset, grouped into ranges and named by region (header ids, date
text, file name, block 2), (b) whether the whole table section (from `TABLE "..."` to the
trailer) is byte-identical, (c) for T_TEXT, the V / C / NV fields of rows 2 to 5 with offsets.
Expects: two fixtures of the same table and path, e.g.
    uv run clientset_diff.py ../../../corpus/fixtures/18c/t_text_exp_18c.dmp \
                             ../../../corpus/fixtures/18c/t_text_exp_w1252_18c.dmp
Reads the fixtures, never writes them.

Changed in Analyst session 4: the end of a table section is taken from exp_parse.py (`sec_end`:
the next `TABLE "` line, `ENDTABLE` or `METRICS...` line) instead of the removed `trailer_off`,
so the tool also works on files with several tables and on owner-mode files, where a "DIFFERENT"
section is expected when the client set changes a text item (exp.md 14.2). Works on 11g.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

a, b = sys.argv[1], sys.argv[2]
da, db = open(a, "rb").read(), open(b, "rb").read()
ra, rb = parse(a), parse(b)
R = ra["header"]["reclen"]
diff = [i for i in range(min(len(da), len(db))) if da[i] != db[i]]
ranges = []
for i in diff:
    if ranges and i == ranges[-1][1] + 1:
        ranges[-1][1] = i
    else:
        ranges.append([i, i])
for lo, hi in ranges:
    if lo < 3:
        reg = "header: client character set id at 0x0001"
    elif lo < 0x45:
        reg = "header: binary fields (client id copy / file name length)"
    elif lo < 0x80:
        reg = "header: date text"
    elif lo < R:
        reg = "header: file name"
    elif lo < R + 204:
        reg = f"block {1 + (lo - R) // 102} +0x{(lo - R) % 102:02x}"
    else:
        reg = "TEXT REGION / TABLE DATA"
    print(f"0x{lo:04x}-0x{hi:04x} {da[lo:hi + 1].hex(' ')} -> {db[lo:hi + 1].hex(' ')}   {reg}")
print(f"client id: {ra['header']['cs_at_1']} -> {rb['header']['cs_at_1']}; "
      f"header third id: {ra['header']['cs_b']} -> {rb['header']['cs_b']}")
for ta, tb in zip(ra["tables"], rb["tables"]):
    ea = ta["sec_end"]
    sa, sb = da[ta["table_off"]:ea], db[tb["table_off"]:tb["sec_end"]]
    print(f"{ta['name']}: section 0x{ta['table_off']:04x}-0x{ea - 1:04x} ({len(sa)} bytes) "
          f"{'IDENTICAL' if sa == sb else 'DIFFERENT'} in both files; "
          f"descriptors {[(c['type'], c['len'], c.get('csid'), c.get('csform')) for c in tb['cols']]}; "
          f"post-descriptor {tb['post_desc'][1:]}")
    if ta["name"] == "T_TEXT":
        for ro, row in tb["rows"][1:5]:
            print("  row @0x%04x: " % ro + " | ".join(
                f"@0x{fo:04x} [{len(v)}] {v.hex(' ')}" for fo, v in row[1:]))
