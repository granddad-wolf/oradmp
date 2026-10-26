#!/usr/bin/env python3
"""Compare every legacy EXP fixture of Oracle 18c with the 11g fixture of the same stem.

Checks (exp.md section 13): what differs between the two versions when the SQL is the same.
For every pair `X_18c.dmp` / `X_11g.dmp` it compares
- the header fields (everything except the version line, the date and the file name),
- the two 102-byte blocks and the string list of the second record,
- the item list of the text region: number of items, kind of each item (line / plain field /
  pieces), and the text of each item; differing texts are classified (version-dependent storage
  clause, system-generated name SYS_C..., date or SCN inside the text, statistics, other),
- (same file size only) the number of differing bytes from offset RECORDLENGTH to the end,
- the binary part of every table section (column count, descriptors, `00 00`, path flag, rows,
  end marker): byte-identical or not, and the offset of the part in both files.
Prints one line per pair and, with -v, every differing item with both offsets and both texts.
At the end: totals per class of difference.
Expects: the two directories (defaults: corpus/fixtures/18c and corpus/fixtures/11g):
    uv run version_diff.py [-v] [dir18c dir11g]
Reads the fixtures, never writes them. New in Analyst session 4.
"""
import glob
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

here = os.path.dirname(os.path.abspath(__file__))
verbose = "-v" in sys.argv
args = [a for a in sys.argv[1:] if not a.startswith("-")]
d18 = args[0] if args else os.path.join(here, "../../../corpus/fixtures/18c")
d11 = args[1] if len(args) > 1 else os.path.join(here, "../../../corpus/fixtures/11g")


def classify(a: str, b: str) -> str:
    def norm(s):
        s = re.sub(r"SYS_C\d+", "SYS_Cn", s)
        s = re.sub(r"\d{4}-\d\d-\d\d:\d\d:\d\d:\d\d", "TS", s)
        s = re.sub(r"inst_scn=>'\d+'", "inst_scn", s)
        s = re.sub(r"export_db_name=>'[A-Z0-9_.]+'", "export_db_name", s)
        return s
    if norm(a) == norm(b):
        return "name / date / SCN / database name inside the text"
    if a.startswith("METRICE") and b.startswith("METRICE"):
        return "METRICE number"
    if "DBMS_STATS" in a and "DBMS_STATS" in b:
        return "statistics arguments"
    if re.match(r"^(CREATE (TABLE|UNIQUE INDEX|INDEX)|ALTER TABLE .* ADD )", a):
        return "storage clause of CREATE TABLE / INDEX / PRIMARY KEY"
    return "other"


tot: dict = {}
pairs = 0
for f18 in sorted(glob.glob(os.path.join(d18, "*_exp_*18c.dmp"))):
    stem = os.path.basename(f18)[:-len("18c.dmp")]
    f11 = os.path.join(d11, stem + "11g.dmp")
    if not os.path.exists(f11):
        print(f"{stem}: no 11g file")
        continue
    pairs += 1
    a, b = parse(f18), parse(f11)
    da, db = open(f18, "rb").read(), open(f11, "rb").read()
    notes = []
    ha, hb = a["header"], b["header"]
    for k in ("b0", "cs_at_1", "direct_marker", "user", "mode", "reclen", "cs_a", "cs_b", "cs_c", "bin_rest",
              "after_fnlen"):
        if ha[k] != hb[k]:
            notes.append(f"header {k}: {ha[k]!r} / {hb[k]!r}")
    if [t for _o, t in ha["texts"][4:]] != [t for _o, t in hb["texts"][4:]]:
        notes.append("header text lines after RECORDLENGTH differ")
    if a["block1"][1] != b["block1"][1] or a["block2"][1] != b["block2"][1]:
        notes.append("blocks differ")
    if [s for _o, s in a["strings"]] != [s for _o, s in b["strings"]]:
        notes.append("string list differs")
    if a["first_item_off"] != b["first_item_off"]:
        notes.append("first item offset differs")
    ia, ib = a["items"], b["items"]
    if [(i["kind"], i["label"]) for i in ia] != [(i["kind"], i["label"]) for i in ib]:
        notes.append(f"ITEM LISTS DIFFER in kind or label ({len(ia)} / {len(ib)} items)")
    else:
        for x, y in zip(ia, ib):
            if x["raw"] != y["raw"]:
                c = classify(x["text"], y["text"])
                tot[c] = tot.get(c, 0) + 1
                if x["kind"] != "line" and [n for _o, n in x["pieces"]] != [n for _o, n in y["pieces"]]:
                    tot["field piece lengths differ"] = tot.get("field piece lengths differ", 0) + 1
                if verbose:
                    print(f"    {stem} 0x{x['off']:x} / 0x{y['off']:x} [{c}] len {len(x['raw'])} / {len(y['raw'])}")
                    if c != "METRICE number":
                        print(f"        18c: {x['text'][:400]!r}\n        11g: {y['text'][:400]!r}")
    same_bin = 0
    for ta, tb in zip(a["tables"], b["tables"]):
        ra, rb = da[ta["bin_off"]:ta["rows_end"] + 1], db[tb["bin_off"]:tb["rows_end"] + 1]
        if ta["name"] != tb["name"] or ra != rb:
            notes.append(f"BINARY PART of {ta['name']} differs (0x{ta['bin_off']:x} / 0x{tb['bin_off']:x})")
            tot["table binary parts differing"] = tot.get("table binary parts differing", 0) + 1
        else:
            same_bin += 1
            tot["table binary parts identical"] = tot.get("table binary parts identical", 0) + 1
    if len(a["tables"]) != len(b["tables"]):
        notes.append("number of tables differs")
    R = ha["reclen"]
    if len(da) == len(db) and ha["reclen"] == hb["reclen"]:
        nd = sum(1 for i in range(R, len(da)) if da[i] != db[i])
        tot["pairs whose bytes from offset R on are identical"] = \
            tot.get("pairs whose bytes from offset R on are identical", 0) + (nd == 0)
        notes.append(f"bytes differing from offset R on: {nd}")
    print(f"{stem:36s} items {len(ia)}/{len(ib)} tables identical {same_bin}/{len(a['tables'])} "
          f"data end 0x{a['data_end']:x}/0x{b['data_end']:x}  {'; '.join(notes) or 'no structural difference'}")
print(f"pairs compared: {pairs}")
for k in sorted(tot):
    print(f"  {tot[k]:5d} x {k}")
