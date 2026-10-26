#!/usr/bin/env python3
"""List the `ANALSTATS xx` / `ANALCOMPUTE xx` lines of every legacy EXP fixture with their context.

Checks (exp.md section 14.5, Part F6 of session 4): the hypothesis "the second letter after
ANALSTATS / ANALCOMPUTE is `R` when the export log has `EXP-00091: Exporting questionable
statistics.` and `S` otherwise", and what else the letters depend on. For every file it prints
the client character set id (header 0x0001), the id at header 0x0032, the two-letter codes seen,
the number of `EXP-00091` lines in the log next to the dump, the form of the item that follows an
ANALSTATS line and the first 60 bytes of it. The last lines give the totals and every file that
contradicts the hypothesis.
Expects: fixture directories (default: the three folders of corpus/fixtures):
    uv run anal_lines.py [dir ...] [-v]
-v prints one line per ANAL item. Reads the fixtures and their logs, never writes them.
New in Analyst session 4.
Analyst session 5: no change of the code. It runs on the R-016 files (`stat_exp_*`,
`ru_stat_exp_*`): the first letter can now be `T` or `C` as well as `I`; 46 of 153 files have such
lines and all 46 fit the hypothesis. stat_items.py looks inside the fields and counts the
`EXP-00091` lines against the plain fields.
"""
import glob
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

here = os.path.dirname(os.path.abspath(__file__))
verbose = "-v" in sys.argv
dirs = [a for a in sys.argv[1:] if not a.startswith("-")] or \
    [os.path.join(here, "../../../corpus/fixtures", x) for x in ("18c", "11g", "11g-cp1251")]
tot = {"files": 0, "files_with_anal": 0, "fit": 0, "contradict": 0}
codes_all: dict = {}
for fdir in dirs:
    for f in sorted(glob.glob(os.path.join(fdir, "*_exp_*.dmp"))):
        r = parse(f)
        h = r["header"]
        log = open(f[:-4] + ".log", "rb").read()
        n91 = log.count(b"EXP-00091")
        codes: dict = {}
        its = r["items"]
        for i, it in enumerate(its):
            if it["kind"] != "line":
                continue
            m = re.match(r"^(ANALSTATS|ANALCOMPUTE) ([A-Z]{2}) ", it["text"])
            if not m:
                continue
            key = f"{m.group(1)} {m.group(2)}"
            codes[key] = codes.get(key, 0) + 1
            codes_all[key] = codes_all.get(key, 0) + 1
            if verbose:
                nxt = its[i + 1] if m.group(1) == "ANALSTATS" else None
                extra = "" if nxt is None else f" -> {nxt['kind']} [{len(nxt['raw'])}] {nxt['raw'][:60]!r}"
                print(f"    {os.path.basename(f)} 0x{it['off']:x} {it['text'][:60]!r}{extra}")
        tot["files"] += 1
        if not codes:
            continue
        tot["files_with_anal"] += 1
        second = {k.split()[1][1] for k in codes}
        fit = (second == {"R"} and n91 > 0) or (second == {"S"} and n91 == 0)
        tot["fit" if fit else "contradict"] += 1
        print(f"{os.path.basename(f):40s} client {h['cs_at_1']:4d} hdr0x32 {h['cs_b']:4d} "
              f"EXP-00091 x{n91}  {codes}  {'fits' if fit else 'CONTRADICTS the hypothesis'}")
print(tot)
print("codes seen:", codes_all)
