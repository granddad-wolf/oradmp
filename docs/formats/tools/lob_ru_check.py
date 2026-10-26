#!/usr/bin/env python3
"""LOB values in a single-byte (CL8MSWIN1251) database: the R-038 files against truth_lob_ru.txt and the SQL.

Checks (Analyst session 10, request R-038; exp.md 14.6, rules L1 to L6 of exp_reader_rules.md):
- R_LOB2 (`ID NUMBER(3), C CLOB, N NCLOB, B BLOB`; corpus/sql/15_ru_lob2.sql): descriptors, m, path flag and
  column entries (L1); per row the fields (locator / NULL), k (L3) and every LOB value with its offset, form
  (short `u16 n` / long `fc ff` + 8-byte total + chunks, L4), byte length, chunk sizes, and the 8-byte total;
- the value bytes: C = database bytes (cp1251), N = UTF-16 BE, B = bytes, against the SQL (blk = 0xc0..0xff,
  0xa8, 0xb8; row 2 = 303 x blk + the first 2 bytes) and against truth_lob_ru.txt (GETLENGTH, head / tail DUMPs);
- decode: C with Python's `cp1251`, N with `utf-16-be`: equal to 303 x 'А..яЁё' + 'АБ' (row 2; c0 c1 = 'АБ') / 'А..яЁё' (row 1);
- the two files (client CL8MSWIN1251 / US7ASCII) byte by byte: every differing range, named by region;
  whether the whole R_LOB2 binary part except the locator fields is identical;
- rule T8: the R_TEXT section (from `TABLE "R_TEXT"` to the LF after its rows) against the same bytes of
  ru_tbl_exp_cp1251_11g.dmp / ru_tbl_exp_us7ascii_11g.dmp (same client set; R-015b files).
Expects: corpus/fixtures/11g-cp1251/ru_lob2_exp_cp1251_11g.dmp and ru_lob2_exp_us7ascii_11g.dmp (default), e.g.
    uv run lob_ru_check.py [-v]
Uses exp_parse.py. Reads the fixtures, never writes them. New in Analyst session 10.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = os.path.join(HERE, "../../../corpus/fixtures/11g-cp1251")
BLK = bytes(range(0xC0, 0x100)) + b"\xa8\xb8"
FULL = {1: BLK, 2: BLK * 303 + BLK[:2]}
NAMES = {1: "C (CLOB)", 2: "N (NCLOB)", 3: "B (BLOB)"}

ok = bad = 0


def rep(name: str, good: bool, detail: str = "") -> None:
    global ok, bad
    ok += good
    bad += not good
    if not good or "-v" in sys.argv:
        print(f"    {name}: {'OK' if good else 'DIFF'} {detail}")


def dump_bytes(s: str):
    return None if s == "NULL" else bytes(int(x, 16) for x in s.split(": ", 1)[1].split(","))


def num_int(b: bytes) -> int:
    e = b[0] - 0xC0
    digs = [x - 1 for x in b[1:]] + [0] * (e - (len(b) - 1))
    v = 0
    for x in digs[:e]:
        v = v * 100 + x
    return v


def section(path: str, name: str):
    r = parse(path)
    d = open(path, "rb").read()
    t = [x for x in r["tables"] if x["name"] == name][0]
    return t, d[t["table_off"]:t["rows_end"] + 1], r, d


def check_file(path: str) -> dict:
    print(f"{os.path.basename(path)}")
    truth = {int(f[1]): f for f in (ln.split("|") for ln in open(os.path.join(FIX, "truth_lob_ru.txt"),
                                                                  encoding="utf-8").read().split("\n") if ln)}
    t, _sec, r, d = section(path, "R_LOB2")
    rep("walk without problems", not r["problems"], str(r["problems"]))
    i0 = t["insert_end"] if "insert_end" in t else None
    cols = [(c["type"], c.get("csid"), c.get("csform")) for c in t["cols"]]
    rep("descriptors NUMBER, CLOB 171/1, NCLOB 2000/2, BLOB", cols == [(2, None, None), (112, 171, 1), (112, 2000, 2),
                                                                       (113, None, None)], str(cols))
    rep("conventional path (flag 0)", not t["direct"])
    print(f"  rows at: " + ", ".join(f"0x{ro:x}" for ro, _ in t["rows"]))
    rows = {num_int(row[0][1]): (ri, ro, row) for ri, (ro, row) in enumerate(t["rows"])}
    vals = {(ri, ci): (vo, buf, form, lens) for ri, ci, vo, buf, form, lens in t["lob_values"]}
    out = {}
    print("    id col         offset    form   bytes  chunks                  total (8 bytes LE)")
    for i in (1, 2, 3):
        ri, ro, row = rows[i]
        f = truth[i]
        nonnull = [ci for ci in (1, 2, 3) if row[ci][1] is not None]
        # k: the u16 after the last field of the row
        last_o, last_v = row[-1]
        ko = last_o + 2 + (len(last_v) if last_v is not None else 0)
        k = int.from_bytes(d[ko:ko + 2], "little")
        rep(f"id {i} k = number of non-NULL LOB columns", k == len(nonnull), f"k @0x{ko:x} = {k}")
        print(f"    id {i}: k @0x{ko:x} = {k}; LOB fields: " + ", ".join(
            f"{NAMES[ci][:1]} @0x{row[ci][0]:x} " + ("NULL" if row[ci][1] is None else f"locator {len(row[ci][1])}")
            for ci in (1, 2, 3)))
        for ci in (1, 2, 3):
            if f[1 + ci] == "NULL":
                rep(f"id {i} {NAMES[ci]} NULL = truth", row[ci][1] is None and (ri, ci) not in vals)
                continue
            vo, buf, form, lens = vals[(ri, ci)]
            txt = FULL[i]
            want = txt.decode("cp1251").encode("utf-16-be") if ci == 2 else txt
            total = d[vo + 2:vo + 10].hex(" ") if form == "long" else "-"
            print(f"    {i:2d} {NAMES[ci]:10s} 0x{vo:06x} {form:5s} {len(buf):6d}  {str(lens):22s}  {total}")
            out[(i, ci)] = (vo, form, len(buf), lens, d[vo:vo + 2].hex(" "))
            rep(f"id {i} {NAMES[ci]} bytes = SQL", buf == want, f"{len(buf)} / {len(want)}")
            n = int(f[1 + ci])
            rep(f"id {i} {NAMES[ci]} GETLENGTH", n == len(txt), f"{n}")
            if ci == 1:
                rep(f"id {i} C head / tail = truth DUMP", buf[:4] == dump_bytes(f[5]) and buf[-4:] == dump_bytes(f[7]),
                    f"{buf[:4].hex(',')} .. {buf[-4:].hex(',')}")
                dec = buf.decode("cp1251")
                rep(f"id {i} C decoded with cp1251", dec == ("АБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдежзийклмнопрстуфхцчшщъыьэюяЁё"
                                                              * (1 if i == 1 else 303) + ("" if i == 1 else "АБ")),
                    f"{len(dec)} characters")
            if ci == 2:
                rep(f"id {i} N head / tail = truth DUMP", buf[:8] == dump_bytes(f[6]) and buf[-8:] == dump_bytes(f[8]),
                    f"{buf[:8].hex(',')} .. {buf[-8:].hex(',')}")
                dec = buf.decode("utf-16-be")
                rep(f"id {i} N decoded as UTF-16 BE = C decoded", dec == FULL[i].decode("cp1251"), f"{len(dec)} characters")
            if form == "long":
                tv = int.from_bytes(d[vo + 2:vo + 10], "little")
                rep(f"id {i} {NAMES[ci]} total = byte length, bytes 4-7 zero", tv == len(buf) and d[vo + 6:vo + 10] == bytes(4))
                rep(f"id {i} {NAMES[ci]} chunks full but the last", all(x == lens[0] for x in lens[:-1])
                    and 0 < lens[-1] <= lens[0], str(lens))
    end = t["rows_end"]
    print(f"  end of rows: ff ff @0x{end - 2:x}, LF @0x{end:x}: {d[end - 2:end + 1].hex(' ')}")
    rep("ff ff LF after the last row's values", d[end - 2:end + 1] == b"\xff\xff\n")
    return {"t": t, "d": d, "vals": out}


def regions(a: bytes, b: bytes, ta: dict):
    diffs = [i for i in range(min(len(a), len(b))) if a[i] != b[i]]
    rng = []
    for i in diffs:
        if rng and i <= rng[-1][1] + 4:
            rng[-1][1] = i
        else:
            rng.append([i, i])
    loc = [(o, o + 2 + len(v)) for _ro, row in ta["rows"] for ci, (o, v) in enumerate(row) if ci and v is not None]
    for s, e in rng:
        where = ("header (first record)" if e < ta.get("reclen", 8192) else
                 "second record, blocks / strings" if e < ta.get("reclen", 8192) + 0x104 else
                 next((f"inside the locator field @0x{lo:x}" for lo, le in loc if lo <= s and e < le), None) or
                 ("text region after the last table's rows (client set, C2)" if s > ta["last_rows_end"] else
                  "text region between R_LOB2 and the end of R_TEXT" if s > ta["rows_end"] else "R_LOB2 binary part"))
        yield s, e, where


def main() -> None:
    a = check_file(os.path.join(FIX, "ru_lob2_exp_cp1251_11g.dmp"))
    b = check_file(os.path.join(FIX, "ru_lob2_exp_us7ascii_11g.dmp"))
    print("value offsets equal in both files:", all(a["vals"][k] == b["vals"][k] for k in a["vals"]))
    rep("LOB values identical in both files (offset, form, length, chunks)", a["vals"] == b["vals"])
    da, db = a["d"], b["d"]
    rep("same file size", len(da) == len(db), f"{len(da)} / {len(db)}")
    print("differing ranges, cp1251 file vs us7ascii file:")
    t = dict(a["t"])
    t["reclen"] = 8192
    t["last_rows_end"] = max(x["rows_end"] for x in parse(os.path.join(FIX, "ru_lob2_exp_cp1251_11g.dmp"))["tables"])
    for s, e, where in regions(da, db, t):
        print(f"  0x{s:05x}-0x{e:05x} ({e - s + 1:4d}) {where}: {da[s:e + 1][:24].hex(' ')} | {db[s:e + 1][:24].hex(' ')}")
    # the R_LOB2 binary part without the locator bytes
    ta, tb = a["t"], b["t"]
    rep("R_LOB2 section at the same offsets", (ta["table_off"], ta["rows_end"]) == (tb["table_off"], tb["rows_end"]))
    # T8: R_TEXT against the R-015b files of the same client set
    for mine, ref in ((os.path.join(FIX, "ru_lob2_exp_cp1251_11g.dmp"), "ru_tbl_exp_cp1251_11g.dmp"),
                      (os.path.join(FIX, "ru_lob2_exp_us7ascii_11g.dmp"), "ru_tbl_exp_us7ascii_11g.dmp")):
        t1, s1, _r, _d = section(mine, "R_TEXT")
        t2, s2, _r, _d = section(os.path.join(FIX, ref), "R_TEXT")
        rep(f"T8: R_TEXT of {os.path.basename(mine)} @0x{t1['table_off']:x}-0x{t1['rows_end']:x} = {ref} "
            f"@0x{t2['table_off']:x}-0x{t2['rows_end']:x}", s1 == s2, f"{len(s1)} / {len(s2)} bytes")
    print(f"TOTAL OK {ok}, DIFF {bad}")


if __name__ == "__main__":
    main()
