#!/usr/bin/env python3
"""Run the walker and the truth comparison over every legacy EXP fixture of one or more directories.

Checks (exp.md sections 4.2, 4.4, 5.4, 6, 13):
- that the two item rules of exp_parse.py (`generic`, `keyword`) and both "direct path, no rows"
  rules (`text`, `declared`) walk every file to the two `EXIT` lines without a problem and give
  the same tables, rows and items; every disagreement is printed with the first differing offset;
- what the item rules of session 3 (`generic3`, `keyword3`) do on the same files: a problem, or a
  walk that ends without a problem but with a different item list (printed as SILENT);
- how many table sections are found (`TABLE "..."` line followed by `CREATE TABLE`), how many have
  an `INSERT INTO` line and a column count, how many decode to the truth with 0 differing fields
  and the truth's row count;
- per column type and per path: fields compared with the truth, fields differing.
Expects: fixture directories, default corpus/fixtures/18c:
    uv run check_all.py ../../../corpus/fixtures/18c ../../../corpus/fixtures/11g
Prints one line per file and the totals per directory. Reads the fixtures, never writes them.

Changed in Analyst session 4:
- several directories; works on 11g and 11g-cp1251;
- four item rules instead of two; a disagreement is reported with the offset of the first item
  that differs, and "SILENT" marks a rule that ends without a problem but reads other items;
- sections are counted by exp_parse.py (`sections`), so owner-mode files are counted correctly;
- per-type and per-path field counts (`types`); tables compared with SQL instead of a truth file
  are counted apart (`sql_ok`); tables with a LOB column (hypothesis reader) are counted apart.

Changed in Analyst session 9: every table of the R-031 to R-037 files has a truth or SQL comparison in
check_truth.py (T_NUM3, T_NCHAR2, T_PROBE11, T_LOB3, T_INTERVAL2, T_ROWID2, T_LONG2, T_LONG_EMPTY, T_PART2 by
subpartition), so `no_truth` is 0 in all three folders; LONG / LONG RAW on the direct path are read as piece
chains (exp_parse.py) and compared by column name (the direct path writes the LONG column last).

Changed in Analyst session 8: type names for 113 (BLOB), NCLOB, 182 / 183 (INTERVAL), 69 (ROWID),
208 (UROWID); 112 is no longer called a hypothesis (exp.md 14.6 rewritten). A partitioned table gives
one entry per partition (exp_parse.py), so `with_insert` counts binary parts, not sections.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from check_truth import check  # noqa: E402

TYPE_NAMES = {1: "VARCHAR2", "1n": "NVARCHAR2", 2: "NUMBER", 12: "DATE", 23: "RAW", 96: "CHAR",
              112: "CLOB data", 180: "TIMESTAMP", 181: "TSTZ", 231: "TSLTZ",
              "96n": "NCHAR", 8: "LONG", 24: "LONG RAW",   # 8, 24, 96n: session 7
              113: "BLOB data", "112n": "NCLOB data", 182: "INTERVAL YM", 183: "INTERVAL DS",
              69: "ROWID", 208: "UROWID"}                   # session 8


def sig_of(r):
    return ([(t["name"], t["rows_off"], len(t["rows"]), t["rows_end"], t["end_marker"]) for t in r["tables"]],
            [(i["off"], i["kind"], i["text"]) for i in r["items"]], r["problems"])


def first_diff(a, b):
    for x, y in zip(a[1], b[1]):
        if x != y:
            return f"first differing item at 0x{x[0]:x} ({x[1]} / {y[1]})"
    return f"item count {len(a[1])} / {len(b[1])}"


def run(fdir: str) -> None:
    files = sorted(glob.glob(os.path.join(fdir, "*_exp_*.dmp")))
    tot = {"files": 0, "walked_ok": 0, "sections": 0, "with_insert": 0, "truth_ok": 0, "sql_ok": 0,
           "no_truth": 0, "bad": 0, "empty_direct": 0, "empty_conv": 0, "fields": 0, "problems": 0,
           "rule_diff": 0, "empty_rule_diff": 0, "field_items": 0, "lob_tables": 0,
           "generic3_fails": 0, "keyword3_fails": 0, "keyword3_silent": 0, "generic3_silent": 0}
    types: dict = {}
    for f in files:
        name = os.path.basename(f)
        base = None
        for items in ("generic", "keyword"):
            for empty in ("text", "declared"):
                r, res = check(f, True, items, empty)
                sig = sig_of(r)
                if base is None:
                    base, r0, res0 = sig, r, res
                elif sig != base:
                    tot["rule_diff"] += 1
                    print(f"  RULES DISAGREE in {name}: items={items} empty={empty}: {first_diff(base, sig)}")
        for old in ("generic3", "keyword3"):
            try:
                ro, _ = check(f, True, old, "text")
                so = sig_of(ro)
                if ro["problems"]:
                    tot[old + "_fails"] += 1
                    print(f"  {old} FAILS in {name}: {len(ro['problems'])} problems, first: {ro['problems'][0]}")
                elif so != base:
                    tot[old + "_silent"] += 1
                    print(f"  {old} SILENT in {name}: no problem, but {first_diff(base, so)}")
            except Exception as e:  # noqa: BLE001
                tot[old + "_fails"] += 1
                print(f"  {old} FAILS in {name}: {e!r}")
        tot["files"] += 1
        tot["problems"] += len(r0["problems"])
        tot["walked_ok"] += not r0["problems"]
        for pr in r0["problems"]:
            print(f"  PROBLEM in {name}: {pr}")
        nsec = len(r0["sections"])
        tot["sections"] += nsec
        tot["with_insert"] += len(r0["tables"])
        tot["field_items"] += len([1 for i in r0["items"] if i["kind"] != "line"])
        parts = []
        for t, s in zip(r0["tables"], res0):
            if t["empty_rule_text"] != t["empty_rule_declared"]:
                tot["empty_rule_diff"] += 1
                print(f"  EMPTY RULES DISAGREE in {name}: table {t['name']} at 0x{t['rows_off']:x}")
            if not t["rows"]:
                tot["empty_direct" if t["direct"] else "empty_conv"] += 1
            tot["fields"] += s["ok"]
            tot["lob_tables"] += s["lob"]
            for k, (g, b) in s["types"].items():
                e = types.setdefault((k, s["path"]), [0, 0])
                e[0] += g
                e[1] += b
            if s["truth_rows"] is None:
                tot["no_truth"] += 1
                v = "no truth"
            elif s["bad"] == 0 and s["missing"] == 0 and s["rows"] == s["truth_rows"]:
                tot["sql_ok" if s["note"].startswith("sql") else "truth_ok"] += 1
                v = "ok"
            else:
                tot["bad"] += 1
                v = f"DIFF {s['bad']} missing {s['missing']}"
            parts.append(f"{t['name']}:{s['rows']}:{v}")
        print(f"{name:42s} sections {nsec} problems {len(r0['problems'])}  " + " ".join(parts))
    print(f"== {fdir}")
    print(tot)
    print("fields compared with the truth, per type and path (equal / differing):")
    for (k, path), (g, b) in sorted(types.items(), key=lambda x: (str(x[0][0]), x[0][1])):
        print(f"  {TYPE_NAMES.get(k, k):24s} {path:13s} {g:6d} / {b}")


here = os.path.dirname(os.path.abspath(__file__))
dirs = [a for a in sys.argv[1:] if not a.startswith("-")] or [os.path.join(here, "../../../corpus/fixtures/18c")]
for d_ in dirs:
    run(d_)
