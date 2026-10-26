#!/usr/bin/env python3
"""Check the list of lines that introduce a field (exp.md section 11, rule I6) against every fixture.

Checks (Analyst session 5, Part E): the list below is the one printed at the end of section 11 of
exp.md, in the form a tool can use: kind (`prefix`, `exact` or `suffix`), the ASCII text, and the
field forms seen after such a line. For every `*_exp_*.dmp` file the items are found with the
GENERIC rule of exp_parse.py (which does not use any list), and the tool then verifies that
- every field follows a line that matches exactly one entry of the list (a field after any other
  line is printed as "FIELD WITHOUT ENTRY");
- every line that matches an entry is followed by a field (otherwise "ENTRY WITHOUT FIELD");
- no line matches two entries;
- the form of the field (plain / pieces after `fd ff` / pieces without marker) is one the entry
  allows.
It prints, per entry, the number of lines and the forms seen, and the totals.
Expects: fixture directories (default: the three folders of corpus/fixtures):
    uv run intro_list.py [dir ...]
Uses exp_parse.py from the same directory. Exploration tool only. Reads the fixtures, never
writes them. New in Analyst session 5.
Changed in Analyst session 7 (notes A-3 / N-3): the block of list K in docs/formats/exp_reader_rules.md
(chapter 2, lines `    kind:length:forms:text`) and the block at the end of exp.md section 11 are read
and compared with ENTRIES below, entry by entry: kind, byte length (which must also equal the length of
the text), the allowed forms (`plain`, `marked` = pieces after `fd ff`, `unmarked` = pieces without
marker) and the text. Any difference is printed as BLOCK DIFFERS and counted in `block_diff`, so a change
of a form in either document is detected.
"""
import glob
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

# kind, text, allowed forms. Keep in step with the list at the end of exp.md section 11.
ENTRIES = [
    ("suffix", ' MODIFY DEFAULT', {"plain"}),
    ("prefix", 'ANALSTATS ', {"plain", "pieces"}),
    ("prefix", 'COMMENT ON ', {"plain"}),
    ("exact", 'CALTER TABLE', {"pieces"}),
    ("exact", 'PROCACTION', {"pieces"}),
    ("prefix", 'CREATE FORMAT71 PROCEDURE ', {"pieces"}),
    ("exact", 'CREATE FORMAT92TRIG ', {"pieces"}),
    ("prefix", 'CREATE VIEW ', {"upieces"}),
]


FORM_NAMES = {"plain": "plain", "marked": "pieces", "unmarked": "upieces"}   # block name -> exp_parse kind


def read_block(path: str) -> list:
    """Entries `kind:length:forms:text` of the indented block(s) of a Markdown file."""
    out = []
    for ln in open(path, encoding="utf-8").read().split("\n"):
        m = re.match(r"^    (prefix|exact|suffix):(\d+):([a-z|]+):(.*)$", ln)
        if m:
            out.append((m.group(1), int(m.group(2)), m.group(3), m.group(4)))
    return out


def check_block(path: str) -> int:
    blk = read_block(path)
    diffs = 0
    want = [(k, len(t), "|".join(n for n, kind in FORM_NAMES.items() if kind in f), t) for k, t, f in ENTRIES]
    for i in range(max(len(blk), len(want))):
        b = blk[i] if i < len(blk) else None
        w = want[i] if i < len(want) else None
        bad = b != w or (b is not None and b[1] != len(b[3].encode("ascii")))
        if bad:
            diffs += 1
            print(f"BLOCK DIFFERS {os.path.basename(path)} entry {i + 1}: block {b!r}, tool {w!r}")
    print(f"block of {os.path.basename(path)}: {len(blk)} entries, {diffs} differences")
    return diffs


def matches(line: str):
    out = []
    for k, (kind, text, _forms) in enumerate(ENTRIES):
        if (kind == "prefix" and line.startswith(text)) or (kind == "exact" and line == text) \
                or (kind == "suffix" and line.endswith(text)):
            out.append(k)
    return out


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    dirs = [a for a in sys.argv[1:] if not a.startswith("-")] or \
        [os.path.join(here, "../../../corpus/fixtures", x) for x in ("18c", "11g", "11g-cp1251")]
    seen = [dict() for _ in ENTRIES]
    docs = os.path.join(here, "..")
    block_diff = check_block(os.path.join(docs, "exp_reader_rules.md")) + check_block(os.path.join(docs, "exp.md"))
    tot = {"block_diff": block_diff, "files": 0, "fields": 0, "field_without_entry": 0, "entry_without_field": 0, "two_entries": 0,
           "form_not_allowed": 0}
    for fdir in dirs:
        for f in sorted(glob.glob(os.path.join(fdir, "*_exp_*.dmp"))):
            r = parse(f, "generic")
            tot["files"] += 1
            its = r["items"]
            for i, it in enumerate(its):
                nxt = its[i + 1] if i + 1 < len(its) else None
                if it["kind"] == "line":
                    m = matches(it["text"])
                    if len(m) > 1:
                        tot["two_entries"] += 1
                        print(f"TWO ENTRIES {os.path.basename(f)} 0x{it['off']:x} {it['text'][:60]!r}")
                    if m and (nxt is None or nxt["kind"] == "line"):
                        tot["entry_without_field"] += 1
                        print(f"ENTRY WITHOUT FIELD {os.path.basename(f)} 0x{it['off']:x} {it['text'][:60]!r}")
                    continue
                tot["fields"] += 1
                prev = its[i - 1]
                m = matches(prev["text"]) if prev["kind"] == "line" else []
                if not m:
                    tot["field_without_entry"] += 1
                    print(f"FIELD WITHOUT ENTRY {os.path.basename(f)} 0x{it['off']:x} after {prev['text'][:60]!r}")
                    continue
                k = m[0]
                seen[k][it["kind"]] = seen[k].get(it["kind"], 0) + 1
                if it["kind"] not in ENTRIES[k][2]:
                    tot["form_not_allowed"] += 1
                    print(f"FORM NOT ALLOWED {os.path.basename(f)} 0x{it['off']:x} {it['kind']} after {prev['text'][:60]!r}")
    for (kind, text, _forms), s in zip(ENTRIES, seen):
        print(f"{kind:7s} {text!r:32s} fields: {s}")
    print(tot)


if __name__ == "__main__":
    main()
