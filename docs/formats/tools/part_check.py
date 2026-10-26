#!/usr/bin/env python3
"""A range-partitioned table in legacy EXP: one binary part per partition, checked against truth_part.txt.

Checks (Analyst session 8, request R-030; exp.md 4.8):
1. The item sequence of the table section: `TABLE "T_PART"`, `CREATE TABLE` (with the PARTITION BY
   clause), then per partition a line `PARTITION "name"`, an `INSERT INTO` line and a binary part
   (column count, descriptors, `00 00`, path flag, rows, `ff ff`, LF), and one `ENDPARTITION` line after
   the last one. Prints every item with its offset (exp_parse.py, session 8, gives each binary part
   its own entry with the key `partition`).
2. Per partition: the row count against `P|name|rows` of truth_part.txt and the export log line
   ". . exporting partition NAME n rows exported"; whether the column count and descriptors are
   byte-identical in every part.
3. Every row against the generating SQL (01b_optional_tables.sql: id = lvl 1..3000, sale_date =
   DATE '2019-01-01' + MOD(lvl * 13, 1000), amount = MOD(lvl * 31, 100000) / 100), with DATE decoded by
   rule Y2 and NUMBER by rule Y1 (num_decode.py); every row's date lies inside its partition's
   bounds (P2019 < 2020-01-01 <= P2020 < 2021-01-01 <= PMAX); the TOTAL line (count, sum of amount,
   min and max date) against the truth.
Expects: part_exp_18c.dmp and part_exp_direct_18c.dmp, e.g.
    uv run part_check.py ../../../corpus/fixtures/18c/part_exp_18c.dmp ../../../corpus/fixtures/18c/part_exp_direct_18c.dmp
Exploration tool only. Reads the fixtures and their logs, never writes them.
"""
import datetime as dt
import os
import re
import sys
from decimal import Decimal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402
from num_decode import decode  # noqa: E402

BOUNDS = {"P2019": (None, dt.date(2020, 1, 1)), "P2020": (dt.date(2020, 1, 1), dt.date(2021, 1, 1)),
          "PMAX": (dt.date(2021, 1, 1), None)}


def date_of(b: bytes) -> dt.date:
    return dt.date((b[0] - 100) * 100 + b[1] - 100, b[2], b[3])


def main() -> None:
    ok = bad = 0

    def rep(name: str, good: bool, detail: str = "") -> None:
        nonlocal ok, bad
        ok += good
        bad += not good
        print(f"  {name}: {'OK' if good else 'DIFF'} {detail}")

    for path in sys.argv[1:]:
        r = parse(path)
        tdir = os.path.dirname(os.path.abspath(path))
        truth = [ln.split("|") for ln in open(os.path.join(tdir, "truth_part.txt"), encoding="utf-8").read().split("\n") if ln]
        log = open(path[:-4] + ".log", encoding="utf-8", errors="replace").read()
        logn = dict(re.findall(r"exporting partition\s+(\S+)\s+(\d+) rows exported", log))
        print(f"{os.path.basename(path)}: problems {r['problems'] or 'none'}")
        sec_start = min(t["table_off"] for t in r["tables"])
        for it in r["items"]:
            if it["off"] >= sec_start and it["kind"] == "line" and not it["text"].startswith("METRICS") \
                    and not it["text"].startswith("METRICE") and it["text"] != "EXIT":
                print(f"  0x{it['off']:05x} line {it['text'][:90]!r}{' ...' if len(it['text']) > 90 else ''}")
        parts = [t for t in r["tables"] if t["name"] == "T_PART"]
        desc0 = None
        allrows = []
        for t in parts:
            d = open(path, "rb").read()
            desc = d[t["bin_off"]:t["post_desc"][0]]
            desc0 = desc0 or desc
            n = len(t["rows"])
            want = [x for x in truth if x[0] == "P" and x[1] == t["partition"]]
            print(f"  partition {t['partition']}: INSERT INTO @0x{t['insert_off']:x}, column count @0x{t['bin_off']:x}, "
                  f"flag {t['post_desc'][2]}, first row @0x{t['rows_off']:x}, ff ff @0x{t['end_marker']:x}, rows {n}")
            rep(f"{t['partition']} rows = truth_part.txt", bool(want) and int(want[0][2]) == n, f"truth {want[0][2] if want else '-'}")
            rep(f"{t['partition']} rows = log", logn.get(t["partition"]) == str(n), f"log {logn.get(t['partition'])}")
            rep(f"{t['partition']} descriptors identical to the first part", desc == desc0, desc.hex(" "))
            lo, hi = BOUNDS[t["partition"]]
            inside = 0
            sqlok = 0
            for _ro, row in t["rows"]:
                i = int(decode(row[0][1]))
                day = date_of(row[1][1])
                amt = decode(row[2][1]) if row[2][1] is not None else None
                inside += (lo is None or day >= lo) and (hi is None or day < hi)
                exp_day = dt.date(2019, 1, 1) + dt.timedelta(days=(i * 13) % 1000)
                exp_amt = Decimal((i * 31) % 100000) / 100
                sqlok += day == exp_day and row[1][1][4:] == b"\x01\x01\x01" and \
                    (amt == exp_amt if exp_amt != 0 else row[2][1] == b"\x80")
                allrows.append((i, day, amt))
            rep(f"{t['partition']} every date inside the partition bounds", inside == n, f"{inside} of {n}")
            rep(f"{t['partition']} every row equals the SQL", sqlok == n, f"{sqlok} of {n}")
        tot = [x for x in truth if x[0] == "TOTAL"][0]
        ids = sorted(i for i, _d, _a in allrows)
        rep("ids are exactly 1..3000", ids == list(range(1, 3001)))
        rep("TOTAL count", str(len(allrows)) == tot[1], f"{len(allrows)} truth {tot[1]}")
        s = sum(a for _i, _d, a in allrows if a is not None)
        rep("TOTAL sum(amount)", format(s.normalize(), "f") == tot[2], f"{s} truth {tot[2]}")
        mn, mx = min(d for _i, d, _a in allrows), max(d for _i, d, _a in allrows)
        rep("TOTAL min / max date", (mn.strftime("%d-%b-%y").upper(), mx.strftime("%d-%b-%y").upper()) == (tot[3], tot[4]),
            f"{mn} {mx} truth {tot[3]} {tot[4]}")
    print(f"TOTAL OK {ok}, DIFF {bad}")


if __name__ == "__main__":
    main()
