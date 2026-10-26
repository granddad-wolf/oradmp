#!/usr/bin/env python3
"""Generates the expected values of tests/t1_walk.rs and tests/t3a_walker.rs from the Analyst's tools.

Run (read-only on the fixtures; the tools are imported, not copied; writes three TSV files next to this
script, nothing else):

    cd docs/formats/tools && uv run ../../../crates/oradmp-core/tests/data/gen_walk_expected.py

Outputs (in tests/data/):
  walk_expected.tsv   one line per file: header fields, sections, parts, rows, fields, NULL fields, lines
                      and text fields per entry of list K, empty tables per path, LOB counts
  walk_parts.tsv      one line per binary part (450 for the 229 files): section ordinal, partition, offsets
                      of the INSERT INTO line, the descriptors, the path flag and the rows, rows, NULLs,
                      end marker
  lob_expected.tsv    one line per binary part of a LOB table (m, the m column entries as hex, k per row,
                      every LOB value with row, column, offset, form, stored length and chunk lengths)

Files covered (task T-3c, rules file v3.4): all 229 `*_exp_*.dmp` fixtures (105 / 97 / 27; T-19b: the 18 files of Analyst sessions 19 and 20), the 2 files `ru_lob2_*` of the
Analyst's session 10 and the 22 files of session 9 included (`exp_parse.py` reads chains and composite partitions since session 9).

Sources of each column:
  header columns      exp_parse.parse()["header"]: the same fields tools/header_fields.py prints
  sections, parts     len(parse()["sections"]) and len(parse()["tables"]) (check_all.py `sections`; corpus_counts.py)
  rows, fields, NULLs corpus_counts.py (rows, row_fields, null_fields), per file
  lines, e0..e7       exp_parse.parse() items; the entry table and its matching are those of
                      tools/intro_list.py (ENTRIES, matches)
  empty_*             corpus_counts.py (empty_direct, empty_conventional), per file
  LOB columns         exp_parse.parse() tables with `lob`: t["lob_values"] (row index, column index, offset,
                      bytes, form, chunk lengths), the same data that tools/lob_check.py checks
Totals over the 229 files, as quoted in implementation-plan.md and the rules file v3.5, are asserted in
main() so that the generator fails if the tools change.
"""
import glob
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TOOLS = os.path.abspath(os.getcwd())
sys.path.insert(0, TOOLS)
from exp_parse import parse  # noqa: E402
from intro_list import ENTRIES, matches  # noqa: E402

FIX = os.path.abspath(os.path.join(TOOLS, "../../../corpus/fixtures"))
FORM = {"plain": 0, "pieces": 1, "upieces": 2}

COLS = ["file", "size", "client_b1", "client_again", "db", "national", "reclen", "direct_flag", "mode",
        "version", "user", "first_item", "sections", "parts", "rows", "fields", "nulls", "lines",
        "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "empty_direct", "empty_conv", "part_rows",
        "lob_sections", "lob_rows", "lob_values", "lob_short", "lob_empty", "lob_long", "lob_bytes",
        "lob_zero_total"]
PART_COLS = ["file", "section", "part", "table", "partition", "subpartition", "ncols", "insert_off", "desc_off", "flag_off",
             "rows_off", "path", "m", "rows", "nulls", "end_marker", "end_marker_off", "rows_end", "long"]
LOB_COLS = ["file", "section", "part", "table", "ncols", "m", "entries", "ks", "values"]


def esc(raw: bytes) -> str:
    return "".join(chr(b) if 0x20 <= b <= 0x7E and b != 0x5C else f"\\x{b:02x}" for b in raw)


def counts(items):
    lines = 0
    per = [[0, 0, 0] for _ in ENTRIES]
    for i, it in enumerate(items):
        if it["kind"] == "line":
            lines += 1
            continue
        prev = items[i - 1]
        m = matches(prev["text"])
        per[m[0]][FORM[it["kind"]]] += 1
    return lines, per


def rle(chunks):
    if not chunks:
        return "-"
    out, run = [], 1
    for a, b in zip(chunks, chunks[1:] + [None]):
        if a == b:
            run += 1
        else:
            out.append(f"{a}" if run == 1 else f"{a}x{run}")
            run = 1
    return "+".join(out)


def section_index(r, t):
    sec = t.get("section", t)
    for i, s in enumerate(r["sections"], 1):
        if s is sec:
            return i
    raise AssertionError("table without section")


def long_stats(t):
    """Per LONG / LONG RAW column (type 8 / 24): `column:values:chained:largest:above32000:chunked:band`, ';'
    separated, '-' for none. Column is 1-based in INSERT INTO order. A value is a non-NULL field; `chained`
    counts values of more than one piece (`long_pieces` of exp_parse.py); `chunked` counts the conventional
    values in the `fd ff` form (`long_chunks`, T-19); `largest` is the assembled byte length; `band` (T-19b,
    ADR-101 item 4) counts the values of 65,521 to 65,534 bytes, from the lengths of the values the tool returns."""
    out = []
    chained = {}
    chunked = {fo: True for _ri, fo, _cl in t.get("long_chunks", [])}
    for _ri, fo, plens in t.get("long_pieces", []):
        if len(plens) > 1:
            chained[fo] = True
    for ci, c in enumerate(t["cols"]):
        if c["type"] not in (8, 24):
            continue
        vals = [row[ci] for _o, row in t["rows"] if row[ci][1] is not None]
        ch = sum(1 for fo, _b in vals if fo in chained)
        ck = sum(1 for fo, _b in vals if fo in chunked)
        out.append(f"{ci + 1}:{len(vals)}:{ch}:{max((len(b) for _f, b in vals), default=0)}:"
                   f"{sum(1 for _f, b in vals if len(b) > 32000)}:{ck}:"
                   f"{sum(1 for _f, b in vals if 65521 <= len(b) <= 65534)}")
    return ";".join(out) if out else "-"


def files():
    out = []
    for d in ("18c", "11g", "11g-cp1251"):
        for f in sorted(glob.glob(os.path.join(FIX, d, "*_exp_*.dmp"))):
            name = os.path.basename(f)
            out.append((d, f, name))
    return out


def main() -> None:
    wl, pl, ll = [], [], []
    tot = dict(files=0, sections=0, parts=0, rows=0, fields=0, nulls=0, lob_sections=0, lob_rows=0,
               lob_values=0, lob_short=0, lob_empty=0, lob_long=0)
    for d, f, name in files():
        r = parse(f)
        assert not r["problems"], (name, r["problems"])
        h = r["header"]
        tabs = r["tables"]
        lines, per = counts(r["items"])
        rows = sum(len(t["rows"]) for t in tabs)
        fields = sum(len(row) for t in tabs for _o, row in t["rows"])
        nulls = sum(1 for t in tabs for _o, row in t["rows"] for _fo, v in row if v is None)
        ed = sum(1 for t in tabs if not t["rows"] and t["direct"])
        ec = sum(1 for t in tabs if not t["rows"] and not t["direct"])
        lobs = [t for t in tabs if t["lob"]]
        vals = [v for t in lobs for v in t["lob_values"]]
        short = [v for v in vals if v[4] == "short"]
        lob = dict(sections=len(lobs), rows=sum(len(t["rows"]) for t in lobs), values=len(vals),
                   short=len(short), empty=sum(1 for v in short if not v[3]),
                   long=sum(1 for v in vals if v[4] == "long"), bytes=sum(len(v[3]) for v in vals))
        wl.append([d + "/" + name, r["size"], h["cs_at_1"], h["cs_a"], h["cs_b"], h["cs_c"], h["reclen"],
                   int(h["direct_marker"]), h["mode"], h["version"], h["user"], r["first_item_off"],
                   len(r["sections"]), len(tabs), rows, fields, nulls, lines]
                  + ["/".join(map(str, p)) for p in per]
                  + [ed, ec, ",".join(str(len(t["rows"])) for t in tabs)]
                  + [lob["sections"], lob["rows"], lob["values"], lob["short"], lob["empty"], lob["long"],
                     lob["bytes"], sum(1 for v in vals if v[4] == "long" and not v[3])])
        seen = {}
        for t in tabs:
            si = section_index(r, t)
            pi = seen.get(si, 0)
            seen[si] = pi + 1
            flag_off = t["post_desc"][0] + 2
            pl.append([d + "/" + name, si, pi, esc(t["name"].encode("latin-1")),
                       esc(t["partition"].encode("latin-1")) if t.get("partition") else "",
                       esc(t["subpartition"].encode("latin-1")) if t.get("subpartition") else "",
                       len(t["cols"]), t["insert_off"], t["bin_off"], flag_off, t["rows_off"],
                       "direct" if t["direct"] else "conv", t["post_desc"][1], len(t["rows"]),
                       sum(1 for _o, row in t["rows"] for _fo, v in row if v is None),
                       "lf" if t["end_marker"] is None else "ffff",
                       t["rows_end"] if t["end_marker"] is None else t["end_marker"], t["rows_end"],
                       long_stats(t)])
            if t["lob"]:
                ks = [0] * len(t["rows"])
                for v in t["lob_values"]:
                    ks[v[0]] += 1
                vs = ";".join(f"{v[0] + 1}:{v[1] + 1}:{v[2]}:{v[4]}:{len(v[3])}:{rle(v[5]) if v[4] == 'long' else '-'}"
                              for v in t["lob_values"])
                ll.append([d + "/" + name, si, pi, esc(t["name"].encode("latin-1")), len(t["cols"]),
                           t["post_desc"][1], t["lob_extra"][1].hex(), ",".join(map(str, ks)), vs])
        tot["files"] += 1
        tot["sections"] += len(r["sections"])
        tot["parts"] += len(tabs)
        tot["rows"] += rows
        tot["fields"] += fields
        tot["nulls"] += nulls
        tot["lob_sections"] += lob["sections"]
        tot["lob_rows"] += lob["rows"]
        for k in ("values", "short", "empty", "long"):
            tot["lob_" + k] += lob[k]
    # the numbers of the plan, 229 files (T-19b, ADR-99 to ADR-103; rules file v3.9; tools/corpus_counts.py and
    # rowlen_stats.py, run in T-19b: 229 files, 436 sections, 450 parts, 41,494 rows, 243,905 fields, 20,084
    # NULL): files, sections, binary parts, rows, row fields, NULL fields; LOB:
    # tools/rowlen_stats.py counts 126 LOB values (80 short, 12 empty, 46 long) in 21 sections (a column of
    # type 112 or 113); [L4] of rules v3.5. (T-10b, review N-1: the comment said 74 / 40 / 19.)
    assert tot["files"] == 229 and tot["sections"] == 436 and tot["parts"] == 450, tot
    assert tot["rows"] == 41494 and tot["fields"] == 243905 and tot["nulls"] == 20084, tot
    assert tot["lob_short"] == 80 and tot["lob_long"] == 46, tot
    assert tot["lob_values"] == 126, tot
    for name, cols, rows in (("walk_expected.tsv", COLS, wl), ("walk_parts.tsv", PART_COLS, pl),
                             ("lob_expected.tsv", LOB_COLS, ll)):
        with open(os.path.join(HERE, name), "w", newline="\n") as out:
            out.write(f"# {name}: generated by gen_walk_expected.py from docs/formats/tools (exp_parse.py,\n")
            out.write("# intro_list.py; header fields as printed by header_fields.py). Do not edit by hand.\n")
            out.write("\t".join(cols) + "\n")
            for row in rows:
                out.write("\t".join(str(x) for x in row) + "\n")
    print(tot)


if __name__ == "__main__":
    main()
