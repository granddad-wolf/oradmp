#!/usr/bin/env python3
"""A range-hash composite-partitioned table in legacy EXP (R-037): lines, binary parts, rows, statistics.

Checks (Analyst session 9, request R-037; exp.md 4.8, rules T10, E1, E2):
1. The skeleton of the T_PART2 section: every line from `CREATE TABLE` to `ENDPARTITION` that names a
   partition or subpartition (`COMPARTITION "p"`, `SUBPARTITION "s"`, `ENDCOMPARTITION`, `PARTITION "p"`,
   `ENDPARTITION`), every `INSERT INTO` line with its binary part (offset, path flag, rows, how the rows end:
   `ff ff` (E1) or "no ff ff" (E2) with the bytes after it), and every statistics item (`ANALSTATS`,
   `ANALCOMPUTE`) with the partition named inside its field.
2. Per binary part: the subpartition name against the `SP|partition|subpartition` lines of truth_part2.txt
   (and the partition of the enclosing `COMPARTITION` line), the ids of its rows against the `ROW|id|subpartition`
   lines, and the row count against the export log line ". . exporting subpartition NAME n rows exported".
3. That the column count, descriptors and the u16 m of every binary part are byte-identical.
Expects: part2_exp_18c.dmp and part2_exp_direct_18c.dmp (18c only), e.g.
    uv run part2_check.py ../../../corpus/fixtures/18c/part2_exp_18c.dmp ../../../corpus/fixtures/18c/part2_exp_direct_18c.dmp
Uses exp_parse.py. Reads the fixtures and their logs, never writes them. New in Analyst session 9.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

NAME_LINES = re.compile(r'^(COMPARTITION "|SUBPARTITION "|PARTITION "|ENDCOMPARTITION$|ENDPARTITION$)')


def num_int(b: bytes) -> int:
    e = b[0] - 0xC0
    digs = [x - 1 for x in b[1:]] + [0] * (e - (len(b) - 1))
    v = 0
    for x in digs[:e]:
        v = v * 100 + x
    return v


def main() -> None:
    ok = bad = 0

    def rep(name: str, good: bool, detail: str = "") -> None:
        nonlocal ok, bad
        ok += good
        bad += not good
        print(f"  {name}: {'OK' if good else 'DIFF'} {detail}")

    for path in sys.argv[1:]:
        r = parse(path)
        d = open(path, "rb").read()
        tdir = os.path.dirname(os.path.abspath(path))
        truth = [ln.split("|") for ln in open(os.path.join(tdir, "truth_part2.txt"), encoding="utf-8").read().split("\n") if ln]
        sp_of = {f[2]: f[1] for f in truth if f[0] == "SP"}
        row_sp = {int(f[1]): f[2] for f in truth if f[0] == "ROW"}
        log = open(path[:-4] + ".log", encoding="utf-8", errors="replace").read()
        log_rows = {m.group(1): int(m.group(2)) for m in re.finditer(r"exporting subpartition\s+(\S+)\s+(\d+) rows", log)}
        print(f"{os.path.basename(path)}: problems {r['problems'] or 'none'}")
        parts = [t for t in r["tables"] if t["name"] == "T_PART2"]
        by_insert = {t["insert_off"]: t for t in parts}
        sec = parts[0]
        end = next(it["off"] for it in r["items"] if it["off"] > sec["table_off"] and it["kind"] == "line"
                   and it["text"] == "ENDPARTITION")
        print("  skeleton:")
        for it in r["items"]:
            if not sec["table_off"] <= it["off"] <= end:
                continue
            txt = it["text"]
            if it["kind"] == "line" and (NAME_LINES.match(txt) or txt.startswith("TABLE ")):
                print(f"    0x{it['off']:04x} line {txt!r}")
            elif it["kind"] == "line" and txt.startswith("INSERT INTO"):
                t = by_insert[it["off"]]
                if t["end_marker"] is None:
                    how = f"no ff ff (E2): LF at 0x{t['rows_end']:x}, then {d[t['rows_end'] + 1:t['rows_end'] + 18]!r}"
                else:
                    how = f"ff ff at 0x{t['end_marker']:x}"
                print(f"    0x{it['off']:04x} INSERT INTO -> binary part 0x{t['bin_off']:x}, flag {int(t['direct'])}, "
                      f"{len(t['rows'])} rows, {how}")
            elif it["kind"] == "line" and txt.startswith(("ANALSTATS", "ANALCOMPUTE")):
                pass
            elif it["kind"] != "line" and it["intro"].startswith("ANALSTATS"):
                m = re.search(r"SET_TABLE_STATS\(NULL,'\"T_PART2\"',(NULL|'\"[^']*\"')", it["text"])
                m2 = re.search(r"SET_COLUMN_STATS\(NULL,'\"T_PART2\"','\"([^\"]*)\"', ?(NULL|'\"[^']*\"')", it["text"])
                what = (f"table statistics, partition {m.group(1)}" if m else
                        f"column {m2.group(1)} statistics, partition {m2.group(2)}" if m2 else "other")
                print(f"    0x{it['off']:04x} ANALSTATS field ({it['kind']}): {what}")
            elif it["kind"] == "line":
                print(f"    0x{it['off']:04x} line {txt[:60]!r}")
        # skipped: CREATE TABLE text etc. are printed by exp_parse.py
        head0 = d[parts[0]["bin_off"]:parts[0]["post_desc"][0] + 2]
        for t in parts:
            sp = t.get("subpartition")
            ids = sorted(num_int(row[0][1]) for _ro, row in t["rows"])
            want = sorted(i for i, s in row_sp.items() if s == sp)
            rep(f"{t['partition']}/{sp} @0x{t['bin_off']:x} partition of the subpartition", sp_of.get(sp) == t["partition"],
                f"(truth SP: {sp_of.get(sp)})")
            rep(f"{t['partition']}/{sp} row ids = truth ROW lines", ids == want, f"{len(ids)} rows")
            rep(f"{t['partition']}/{sp} row count = log", log_rows.get(sp) == len(ids), f"log {log_rows.get(sp)}")
            rep(f"{t['partition']}/{sp} column count, descriptors, m identical to the first part",
                d[t["bin_off"]:t["post_desc"][0] + 2] == head0, head0.hex(" "))
        rep("every subpartition of truth_part2.txt has a binary part", set(sp_of) == {t.get("subpartition") for t in parts})
        rep("all 100 ROW lines found", sum(len(t["rows"]) for t in parts) == len(row_sp), f"{len(row_sp)} ROW lines")
    print(f"TOTAL OK {ok}, DIFF {bad}")


if __name__ == "__main__":
    main()
