#!/usr/bin/env python3
"""Split the 13 bytes of every TIMESTAMP WITH TIME ZONE value of T_TSTZ into fields.

Checks (Analyst session 3, exp.md section 6.5): for rows 1-6 of t_tstz, reads the first 7 bytes
with the DATE layout (section 6.2), the next 4 as a big-endian nanosecond count, and the last 2
as "hour + 20, minute + 60", then compares with truth_tstz.txt: local time text and offset text
of the truth -> expected UTC time and expected last two bytes. Prints OK / DIFF per field, and
whether bit 0x80 of byte 12 is set. Also prints whether the TSLTZ values of rows 101 and 102 are
byte-identical.
Expects: a t_tstz EXP fixture, e.g.
    uv run tstz_fields.py ../../../corpus/fixtures/18c/t_tstz_exp_18c.dmp
Reads the fixture and truth_tstz.txt, never writes them.
"""
import datetime as dt
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402
from check_truth import num_int  # noqa: E402

path = sys.argv[1]
truth = {}
for ln in open(os.path.join(os.path.dirname(os.path.abspath(path)), "truth_tstz.txt")):
    f = ln.rstrip("\n").split("|")
    if len(f) > 1:
        truth[int(f[0])] = f
t = parse(path)["tables"][0]
ltz = {}
for ro, row in t["rows"]:
    i = num_int(row[0][1])
    fo, v = row[1]
    if row[2][1] is not None:
        ltz[i] = row[2]
    if v is None:
        continue
    m = re.match(r"tstz=(\S+ \S+) ([+-])(\d\d):(\d\d) (\S+)", truth[i][1])
    local = dt.datetime.strptime(m.group(1), "%Y-%m-%d %H:%M:%S.%f")
    sign = 1 if m.group(2) == "+" else -1
    oh, om = sign * int(m.group(3)), sign * int(m.group(4))
    utc = local - dt.timedelta(hours=oh, minutes=om)
    got = dt.datetime((v[0] - 100) * 100 + v[1] - 100, v[2], v[3], v[4] - 1, v[5] - 1, v[6] - 1)
    ns = int.from_bytes(v[7:11], "big")
    print(f"row {i} @0x{fo + 2:04x} {v.hex(' ')}  zone {m.group(5)}")
    print(f"   bytes 1-7  as DATE = {got}  expected UTC {utc.replace(microsecond=0)}  "
          f"{'OK' if got == utc.replace(microsecond=0) else 'DIFF'}"
          f"   (local time would be {local.replace(microsecond=0)})")
    print(f"   bytes 8-11 = {ns} ns  expected {local.microsecond * 1000}  "
          f"{'OK' if ns == local.microsecond * 1000 else 'DIFF'}")
    print(f"   bytes 12-13 = {v[11]:02x} {v[12]:02x}; hour+20, minute+60 would be "
          f"{oh + 20:02x} {om + 60:02x}  {'OK' if (v[11], v[12]) == (oh + 20, om + 60) else 'DIFF'}; "
          f"bit 0x80 of byte 12 {'set' if v[11] & 0x80 else 'clear'}")
if 101 in ltz and 102 in ltz:
    (o1, v1), (o2, v2) = ltz[101], ltz[102]
    print(f"TSLTZ row 101 @0x{o1 + 2:04x} {v1.hex(' ')}")
    print(f"TSLTZ row 102 @0x{o2 + 2:04x} {v2.hex(' ')}  {'IDENTICAL' if v1 == v2 else 'DIFFERENT'}")
