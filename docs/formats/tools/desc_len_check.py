#!/usr/bin/env python3
"""Compare every column field length with the maximum length of its column descriptor.

Checks (exp.md 4.1, 6.3, rules C1, C3, C5, C6, X4 of exp_reader_rules.md; session 9, R-034):
- for every non-NULL column field of every table of every `*_exp_*.dmp` file: is its byte length at
  most the descriptor length (u16 at +2 of the descriptor)? Counted per type code and form;
  every field longer than its descriptor is printed as EXCEEDS (file, offset, lengths);
- for CHAR (96): how many values have exactly the descriptor length (byte semantics padding, C1)
  and how many are shorter (character semantics, C6, or NCHAR with a non-BMP character);
- for type 8 / 24 (descriptor length 0) nothing is compared (Y9); for 112 / 113 the field is a LOB
  locator (L2) and is counted but never printed as EXCEEDS.
Expects: fixture directories (default the three folders of corpus/fixtures):
    uv run desc_len_check.py ../../../corpus/fixtures/18c ../../../corpus/fixtures/11g ../../../corpus/fixtures/11g-cp1251
Uses exp_parse.py of this directory. Reads the fixtures, never writes them. New in Analyst session 9.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    dirs = sys.argv[1:] or [os.path.join(here, "../../../corpus/fixtures", v) for v in ("18c", "11g", "11g-cp1251")]
    stats: dict = {}
    exceeds = []
    for f in sorted(x for d in dirs for x in glob.glob(os.path.join(d, "*_exp_*.dmp"))):
        r = parse(f)
        for t in r["tables"]:
            for _ro, row in t["rows"]:
                for c, (fo, v) in zip(t["cols"], row):
                    if v is None or c["type"] in (8, 24):
                        continue
                    key = (c["type"], c.get("csform", 0))
                    s = stats.setdefault(key, {"fields": 0, "equal": 0, "shorter": 0, "longer": 0, "maxlen": 0,
                                               "desc": set()})
                    s["fields"] += 1
                    s["maxlen"] = max(s["maxlen"], len(v))
                    s["desc"].add(c["len"])
                    if len(v) == c["len"]:
                        s["equal"] += 1
                    elif len(v) < c["len"]:
                        s["shorter"] += 1
                    else:
                        s["longer"] += 1
                        if c["type"] in (112, 113):
                            continue                 # a LOB field is a locator (L2), its descriptor is 72
                        exceeds.append(f"{os.path.basename(f)} {t['name']}@0x{fo:x}: type {c['type']} "
                                       f"descriptor {c['len']} value {len(v)}")
    print("type form   fields   =desc  <desc  >desc  longest  descriptor lengths seen")
    for (ty, fm), s in sorted(stats.items()):
        dl = sorted(s["desc"])
        dtxt = ", ".join(map(str, dl[:12])) + (" ..." if len(dl) > 12 else "")
        print(f"{ty:4d} {fm:4d} {s['fields']:8d} {s['equal']:7d} {s['shorter']:6d} {s['longer']:6d} {s['maxlen']:8d}  {dtxt}")
    print(f"EXCEEDS: {len(exceeds)}")
    for x in exceeds[:50]:
        print("  " + x)


if __name__ == "__main__":
    main()
