#!/usr/bin/env python3
"""Where the export date text stands in the first record of a legacy EXP file, and its format.

Checks (exp.md section 2, Analyst session 6, question D4 of the session prompt): in every
`*_exp_*.dmp` fixture the bytes from "end of the seventh header text line + 19" (three u16 BE
character set ids, 10 bytes, the u16 BE file-name length, one `00`) up to offset 0x7f are spaces
followed by ONE text of the form

    Www Mmm D H:M:S YYYY      (single spaces; day, hour, minute and second WITHOUT a leading zero)

which ENDS at 0x7f (so it starts at 0x80 minus its length; the file name starts at 0x80). The tool
prints, per text length, the start offset and the number of files, an example of every length,
how many day / hour / minute / second fields have one digit, and any field written with a leading
zero. A file that does not match is printed as PROBLEM.
Expects: fixture directories (default: the three folders of corpus/fixtures):
    uv run header_date.py [dir ...]
Exploration tool only. Reads the fixtures, never writes them. New in Analyst session 6.
"""
import collections
import glob
import os
import re
import sys

RX = re.compile(rb"^( *)((Mon|Tue|Wed|Thu|Fri|Sat|Sun) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) "
                rb"(\d{1,2}) (\d{1,2}):(\d{1,2}):(\d{1,2}) (\d{4}))$")
here = os.path.dirname(os.path.abspath(__file__))
dirs = [a for a in sys.argv[1:] if not a.startswith("-")] or \
    [os.path.join(here, "../../../corpus/fixtures", d) for d in ("18c", "11g", "11g-cp1251")]
by_len: dict = collections.Counter()
example: dict = {}
one_digit: dict = collections.Counter()
zero_padded = problems = files = 0
for d in dirs:
    for p in sorted(glob.glob(os.path.join(d, "*_exp_*.dmp"))):
        h = open(p, "rb").read(0x400)
        files += 1
        pos = 0
        for _ in range(8 if h[3:5] == b"D\n" else 7):      # `D` LF of a direct-path file is one more line
            pos = h.index(b"\n", pos) + 1
        m = RX.match(h[pos + 19:0x80])
        if not m:
            problems += 1
            print(f"PROBLEM {os.path.basename(p)}: bytes 0x{pos + 19:x}-0x7f are {h[pos + 19:0x80]!r}")
            continue
        text = m.group(2)
        start = 0x80 - len(text)
        by_len[(len(text), start, pos + 19, len(m.group(1)))] += 1
        example.setdefault(len(text), (os.path.basename(p), text.decode()))
        for name, g in (("day", 5), ("hour", 6), ("minute", 7), ("second", 8)):
            one_digit[name] += len(m.group(g)) == 1
            if len(m.group(g)) == 2 and m.group(g)[:1] == b"0":
                zero_padded += 1
                print(f"leading zero in {name}: {os.path.basename(p)} {text!r}")
print(f"files {files}, problems {problems}, fields with a leading zero {zero_padded}")
for (ln, start, sp_off, sp_n), n in sorted(by_len.items()):
    print(f"  text of {ln} bytes: 0x{start:x}-0x7f, after {sp_n} spaces from 0x{sp_off:x}: {n} files")
for ln, (f, t) in sorted(example.items()):
    print(f"  example, {ln} bytes: {f} '{t}'")
print(f"  fields with one digit: {dict(one_digit)} (of {files - problems} each)")
