#!/usr/bin/env python3
"""Check the LOB layout of exp.md 14.6 (session 8) against truth_lob.txt / truth_ru.txt and the SQL.

Checks (Analyst session 8, request R-027):
1. Every LOB table of the given `*_exp_*.dmp` files is walked by exp_parse.py (session 8 LOB rules:
   descriptors 112 / 113, the u16 m before the path flag, m column entries, the locator fields, the
   u16 k after each row, k values in short form `u16 n + n` or long form `fc ff` + 8-byte LE total +
   chunks). The tool prints, per table, the descriptors, the bytes between the path flag and the
   first row, and per row: the locator lengths, k, and every LOB value (offset, form, byte length,
   chunk lengths).
2. Values against the truth:
   T_LOB  (truth_lob.txt `T_LOB|id|len(C)|len(B)|first 30 bytes of B`): C decoded as UTF-16 BE must
          have len(C) characters, B must have len(B) bytes and start with the hex; NULL in the truth
          = locator `fe ff` and no value; and the whole value against 01b_optional_tables.sql
          (row 1 'hello lob' / 48656C..; row 3 empty; row 4 'm' x 3000 / AB x 1000; row 5 the 100
          pieces RPAD('chunk' || LPAD(i,3,'0'), 1000, '.') / CD x 100000), reported as "sql".
   T_LOB2 (`T_LOB2|id|len(A)|V|len(N)|len(B)`): A and N as UTF-16 BE characters, V as text, B
          bytes; whole values against 13_optional.sql ('abc', 'abc', 00FF; row 2: EMPTY_CLOB()).
   R_CLOB (11g-cp1251, truth_ru.txt `R_CLOB|id|len|first 60 bytes hex`): value bytes in the
          database set (CL8MSWIN1251), one byte per character.
3. With two files of the same tables (conventional and direct), compares the binary part of every
   LOB table byte for byte and names the field every differing byte lies in.
4. `--frame`: T_FRAME of the LOB file against T_FRAME of t_frame_exp(_direct)_<v>.dmp in the same
   folder (from the `TABLE "T_FRAME"` line to the LF after its rows), rule T8.
Expects: lob_exp_<v>.dmp, lob_exp_direct_<v>.dmp (18c, 11g) and the R_CLOB files of 11g-cp1251, e.g.
    uv run lob_check.py ../../../corpus/fixtures/18c/lob_exp_18c.dmp ../../../corpus/fixtures/18c/lob_exp_direct_18c.dmp --frame
Exploration tool only. Reads the fixtures, never writes them.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import LOB_TYPES, parse  # noqa: E402


def num_int(b: bytes) -> int:
    e = b[0] - 0xC0
    digs = [x - 1 for x in b[1:]] + [0] * (e - (len(b) - 1))
    v = 0
    for x in digs[:e]:
        v = v * 100 + x
    return v


def sql_values(table: str):
    """Whole values the generating SQL must give, as {id: [value per LOB column in order]} (None = NULL)."""
    if table == "T_LOB":
        c5 = "".join(("chunk%03d" % i).ljust(1000, ".") for i in range(1, 101))
        return {1: ["hello lob", bytes.fromhex("48656C6C6F20626C6F62")], 2: [None, None], 3: ["", b""],
                4: ["m" * 3000, b"\xab" * 1000], 5: [c5, b"\xcd" * 100000]}
    if table == "T_LOB2":
        return {1: ["abc", "abc", b"\x00\xff"], 2: [None, "", None]}
    return None


def truth_lines(tdir: str, name: str, prefix: str):
    p = os.path.join(tdir, name)
    if not os.path.exists(p):
        return []
    return [ln.split("|") for ln in open(p, encoding="utf-8").read().split("\n") if ln.startswith(prefix)]


def check_table(t: dict, tdir: str, data: bytes) -> tuple:
    ok = bad = 0
    cols = t["cols"]
    lobc = [i for i, c in enumerate(cols) if c["type"] in LOB_TYPES]
    print(f"  {t['name']}: ncols {len(cols)}, descriptors "
          + ", ".join(f"{c['type']}/{c['len']}" + (f"/{c['csid']}/{c['csform']}" if 'csid' in c else "") for c in cols))
    o, m, flag = t["post_desc"]
    print(f"    @0x{o:x} m={m} flag={flag}; column entries @0x{t['lob_extra'][0]:x}: {t['lob_extra'][1].hex(' ')}"
          f" -> names {[x.decode('latin-1') for x in t['lob_names']]}; first row @0x{t['rows_off']:x}")
    vals: dict = {}
    for ri, ci, vo, buf, form, lens in t["lob_values"]:
        vals[(ri, ci)] = (vo, buf, form, lens)
    sqlv = sql_values(t["name"])
    for ri, (ro, row) in enumerate(t["rows"]):
        rid = num_int(row[0][1])
        locs = ["NULL" if row[ci][1] is None else f"loc[{len(row[ci][1])}]@0x{row[ci][0]:x}" for ci in lobc]
        kv = [(ci, vals[(ri, ci)]) for ci in lobc if (ri, ci) in vals]
        print(f"    row id {rid} @0x{ro:x}: {', '.join(locs)}; values: "
              + "; ".join(f"col {ci + 1} @0x{v[0]:x} {v[2]} {len(v[1])} bytes chunks {len(v[3])}"
                          + (f" {v[3][:2]}..{v[3][-1:]}" if len(v[3]) > 1 else "") for ci, v in kv))
        got = []
        for ci in lobc:
            v = vals.get((ri, ci))
            if row[ci][1] is None:
                got.append(None if v is None else "VALUE WITHOUT LOCATOR")
                continue
            if v is None:
                got.append("LOCATOR WITHOUT VALUE")
                continue
            buf = v[1]
            c = cols[ci]
            if c["type"] == 112 and (c.get("csid") in (873, 2000)):
                got.append(buf.decode("utf-16-be"))
            elif c["type"] == 112:
                got.append(buf)            # single-byte database: the database bytes (R_CLOB)
            else:
                got.append(buf)
        # truth
        checks = []
        if t["name"] == "T_LOB":
            for f in truth_lines(tdir, "truth_lob.txt", "T_LOB|"):
                if int(f[1]) == rid:
                    c, b = got
                    checks.append(("truth len(C) chars", f[2] == "NULL" and c is None or c is not None and str(len(c)) == f[2]))
                    checks.append(("truth len(B) bytes", f[3] == "NULL" and b is None or b is not None and str(len(b)) == f[3]))
                    checks.append(("truth B first 30 bytes", f[4] == "NULL" and (b is None or b == b"")
                                   or b is not None and b[:30] == bytes.fromhex(f[4])))
        if t["name"] == "T_LOB2":
            for f in truth_lines(tdir, "truth_lob.txt", "T_LOB2|"):
                if int(f[1]) == rid:
                    a, n, b = got
                    checks.append(("truth len(A)", f[2] == "NULL" and a is None or a is not None and str(len(a)) == f[2]))
                    checks.append(("truth V", row[2][1] == f[3].encode()))
                    checks.append(("truth len(N)", f[4] == "NULL" and n is None or n is not None and str(len(n)) == f[4]))
                    checks.append(("truth len(B)", f[5] == "NULL" and b is None or b is not None and str(len(b)) == f[5]))
        if t["name"] == "R_CLOB":
            for f in truth_lines(tdir, "truth_ru.txt", "R_CLOB|"):
                if int(f[1]) == rid:
                    (c,) = got
                    if f[2] == "NULL":
                        checks.append(("truth NULL", c is None))
                    else:
                        first = bytes(int(x, 16) for x in f[3].split(": ", 1)[1].split(",")) if ": " in f[3] \
                            else bytes.fromhex(f[3])
                        checks.append(("truth len + first bytes", c is not None and len(c) == int(f[2])
                                       and c[:len(first)] == first))
        if sqlv is not None and rid in sqlv:
            checks.append(("sql whole values", got == sqlv[rid]))
        for name, good in checks:
            ok += bool(good)
            bad += not good
            if not good:
                print(f"      DIFF {name}: got {[g if g is None or len(g) < 40 else (len(g), g[:20]) for g in got]}")
        print(f"      checks: " + ", ".join(f"{n} {'OK' if g else 'DIFF'}" for n, g in checks))
    return ok, bad


def field_map(t: dict) -> list:
    """(start, end, name) of every element of the binary part, for naming differing bytes."""
    m = [(t["bin_off"], t["rows_off"], "column count, descriptors, m, flag, entries")]
    for ri, (ro, row) in enumerate(t["rows"]):
        for ci, (fo, v) in enumerate(row):
            m.append((fo, fo + 2 + (0 if v is None else len(v)), f"row {ri + 1} col {ci + 1} "
                      + ("locator" if t["cols"][ci]["type"] in LOB_TYPES else "field")))
    for ri, ci, vo, buf, form, lens in t["lob_values"]:
        m.append((vo, vo + (10 if form == "long" else 0) + 2 * len(lens) + len(buf), f"row {ri + 1} col {ci + 1} LOB value"))
    return m


def main() -> None:
    files = [a for a in sys.argv[1:] if not a.startswith("--")]
    tot_ok = tot_bad = 0
    parsed = []
    for f in files:
        r = parse(f)
        data = open(f, "rb").read()
        parsed.append((f, r, data))
        print(f"{os.path.basename(f)}: problems {r['problems'] or 'none'}")
        for t in r["tables"]:
            if t["lob"]:
                o, b = check_table(t, os.path.dirname(os.path.abspath(f)), data)
                tot_ok += o
                tot_bad += b
            else:
                print(f"  {t['name']}: no LOB column, flag {t['post_desc'][2]}, {len(t['rows'])} rows")
        if "--frame" in sys.argv:
            tf = [t for t in r["tables"] if t["name"] == "T_FRAME"]
            if tf:
                t = tf[0]
                v = os.path.basename(f).rsplit("_", 1)[1]
                ref = os.path.join(os.path.dirname(f), f"t_frame_exp_{'direct_' if t['direct'] else ''}{v}")
                rr = parse(ref)
                dr = open(ref, "rb").read()
                t2 = rr["tables"][0]
                a = data[t["table_off"]:t["rows_end"] + 1]
                b = dr[t2["table_off"]:t2["rows_end"] + 1]
                print(f"  T_FRAME @0x{t['table_off']:x}..0x{t['rows_end']:x} against {os.path.basename(ref)} "
                      f"@0x{t2['table_off']:x}..0x{t2['rows_end']:x}: {'IDENTICAL' if a == b else 'DIFFERENT'} ({len(a)} bytes)")
    if len(parsed) == 2:
        (fa, ra, da), (fb, rb, db) = parsed
        for ta, tb in zip(ra["tables"], rb["tables"]):
            if not ta["lob"]:
                continue
            pa = da[ta["bin_off"]:ta["rows_end"]]
            pb = db[tb["bin_off"]:tb["rows_end"]]
            print(f"{ta['name']}: binary part {len(pa)} / {len(pb)} bytes, "
                  f"{'identical' if pa == pb else 'different'}")
            if len(pa) == len(pb) and pa != pb:
                fm = field_map(ta)
                diffs = [i for i in range(len(pa)) if pa[i] != pb[i]]
                where: dict = {}
                for i in diffs:
                    off = ta["bin_off"] + i
                    names = [n for s, e, n in fm if s <= off < e]
                    rel = [off - s for s, e, n in fm if s <= off < e]
                    key = (names[0] if names else "?", rel[0] if rel else -1)
                    where.setdefault(key, []).append(f"{pa[i]:02x}/{pb[i]:02x}")
                for (n, rel), v in where.items():
                    print(f"   differs: {n} +0x{rel:x}: {v}")
    print(f"TOTAL checks OK {tot_ok}, DIFF {tot_bad}")


if __name__ == "__main__":
    main()
