#!/usr/bin/env python3
"""Count what the legacy EXP fixtures contain, for the numbers quoted in exp_reader_rules.md.

Checks (note A-1 of the Architect, Analyst session 7): the counts that the rules file quotes
("N files", "fields after an introducing line", "table sections", "ANALSTATS fields", "ANALCOMPUTE
lines", "column type codes") are regenerated from the files, per folder and in total, by walking every
`*_exp_*.dmp` with exp_parse.py (generic item rule). With `--without=stem1,stem2,...` the files whose
name starts with one of the stems are left out, so that the counts of an earlier session can be
reproduced (session 6: 157 files = all 169 without `long_exp`, `nchar_exp`, `t_decl_exp`, `t_num2_exp`,
`stc_exp_us7ascii`, `stc_exp_none`).
Counts: files; table sections (`TABLE` line + `CREATE TABLE`); sections whose descriptors contain only
the 8 type codes of rule T5; sections with a type code outside them (and which codes); rows; row fields;
NULL fields; empty tables (conventional / direct); fields of the text region (after an introducing
line) by form; ANALSTATS fields by form; ANALCOMPUTE lines; column counts (smallest, largest).
Expects: fixture directories (default: the three folders of corpus/fixtures):
    uv run corpus_counts.py [dir ...] [--without=long_exp,nchar_exp,t_decl_exp,t_num2_exp,stc_exp_us7ascii,stc_exp_none]
Reads the fixtures, never writes them. New in Analyst session 7.
"""
import collections
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

T5 = {1, 2, 12, 23, 96, 180, 181, 231}


def count(files: list) -> dict:
    c: dict = collections.Counter()
    codes_out: dict = collections.Counter()
    ncols = []
    for f in files:
        r = parse(f)
        c["files"] += 1
        c["problems"] += len(r["problems"])
        c["sections"] += len(r["sections"])
        for t in r["tables"]:
            ncols.append(len(t["cols"]))
            codes = {col["type"] for col in t["cols"]}
            if codes <= T5:
                c["sections_T5_only"] += 1
            else:
                c["sections_other_code"] += 1
                for x in sorted(codes - T5):
                    codes_out[x] += 1
            c["rows"] += len(t["rows"])
            for _ro, row in t["rows"]:
                c["row_fields"] += len(row)
                c["null_fields"] += sum(1 for _o, v in row if v is None)
            if not t["rows"]:
                c["empty_direct" if t["direct"] else "empty_conventional"] += 1
        for it in r["items"]:
            if it["kind"] != "line":
                c["text_fields"] += 1
                c["text_fields_" + it["kind"]] += 1
                if it["intro"].startswith("ANALSTATS "):
                    c["analstats_" + it["kind"]] += 1
            elif it["text"].startswith("ANALCOMPUTE "):
                c["analcompute_lines"] += 1
    c["ncols_min"], c["ncols_max"] = (min(ncols), max(ncols)) if ncols else (0, 0)
    c["codes_outside_T5"] = dict(codes_out)
    return dict(c)


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    without = [s for a in sys.argv[1:] if a.startswith("--without=") for s in a[10:].split(",") if s]
    dirs = args or [os.path.join(here, "../../../corpus/fixtures", v) for v in ("18c", "11g", "11g-cp1251")]
    allf = []
    for d in dirs:
        files = sorted(f for f in glob.glob(os.path.join(d, "*_exp_*.dmp"))
                       if not any(os.path.basename(f).startswith(s + "_") for s in without))
        allf += files
        print(os.path.basename(os.path.normpath(d)), count(files))
    print("TOTAL", count(allf))


if __name__ == "__main__":
    main()
