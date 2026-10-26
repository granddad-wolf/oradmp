#!/usr/bin/env python3
"""INTERVAL YEAR TO MONTH, INTERVAL DAY TO SECOND, ROWID and UROWID in legacy EXP: bytes against truth_misc.txt.

Checks (Analyst session 8, request R-029; exp.md 4.1 and 6.7):
1. The descriptors of T_INTERVAL (ID, IYM, IDS) and T_ROWID (ID, RID, URID): type code, length, size
   (exp_parse.py reads every code outside 1 / 96 / 112 with 4 bytes; the walk reaching the end of the
   file without a problem is the test of the 4-byte size).
2. Every value against truth_misc.txt:
   INTERVAL|id|TO_CHAR(iym)|DUMP(iym,16)|TO_CHAR(ids)|DUMP(ids,16)
       value bytes == DUMP bytes, and the conversion rules below give exactly the TO_CHAR text;
   ROWID|id|ROWIDTOCHAR(rid)|DUMP(rid,16)|DUMP(urid,16)
       RID field: a 5-byte prefix + the 18 characters of ROWIDTOCHAR (prefix printed); the text
       computed from the 10 DUMP bytes equals ROWIDTOCHAR; URID field == DUMP(urid) bytes, and the
       text computed from those 13 bytes equals ROWIDTOCHAR too.
Conversion rules tested (b = value bytes):
   INTERVAL YEAR(p) TO MONTH, 5 bytes: Y = u32 BE b[0:4] - 0x80000000, M = b[4] - 60. Text: sign
       ('-' if Y < 0 or M < 0 else '+'), |Y| with p digits (p from the CREATE TABLE line), '-',
       |M| with 2 digits.
   INTERVAL DAY(p) TO SECOND(f), 11 bytes: D = u32 BE b[0:4] - 0x80000000, h = b[4] - 60,
       m = b[5] - 60, s = b[6] - 60, ns = u32 BE b[7:11] - 0x80000000. Text: sign, |D| with p
       digits, ' ', hh:mi:ss with 2 digits each, '.', the first f digits of |ns| written with 9.
   ROWID text (extended, 18 characters of A-Z a-z 0-9 + /): object number 6 characters, relative
       file 3, block 6, row 3, each a base-64 number. From DUMP(rowid) (10 bytes): object = u32 BE
       b[0:4], file = (u32 BE b[4:8]) >> 22, block = (u32 BE b[4:8]) & 0x3fffff, row = u16 BE b[8:10].
       From the UROWID bytes (13 bytes, first byte 01): object = u32 BE b[1:5], file = u16 BE b[5:7],
       block = u32 BE b[7:11], row = u16 BE b[11:13].
Changed in Analyst session 9 (R-036): also misc2_exp_<v>.dmp against truth_misc2.txt (T_INTERVAL2 with
IYM YEAR(9), IDS DAY(9) TO SECOND(9), IDS0 DAY(2) TO SECOND(0); T_ROWID2 with three ROWIDs and a logical
UROWID of an index-organised table). Every interval column takes p and f from the CREATE TABLE line;
f = 0 gives no '.' and no fraction. UROWID text: first byte 01 = physical (as before); first byte 02 =
logical: '*' + base 64 (same alphabet, most significant first, 3 bytes -> 4 characters) of the bytes after
the first byte; the truth is CAST(urid AS VARCHAR2(4000)).
Expects: misc_exp_<v>.dmp or misc2_exp_<v>.dmp of corpus/fixtures/18c or 11g, e.g.
    uv run misc_types.py ../../../corpus/fixtures/18c/misc_exp_18c.dmp ../../../corpus/fixtures/11g/misc_exp_11g.dmp
Exploration tool only. Reads the fixtures, never writes them.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

B64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"


def num_int(b: bytes) -> int:
    e = b[0] - 0xC0
    digs = [x - 1 for x in b[1:]] + [0] * (e - (len(b) - 1))
    v = 0
    for x in digs[:e]:
        v = v * 100 + x
    return v


def dump_bytes(s: str):
    if s.strip() == "NULL":
        return None
    return bytes(int(x, 16) for x in s.split(": ", 1)[1].split(","))


def b64(n: int, width: int) -> str:
    out = ""
    for _ in range(width):
        out = B64[n % 64] + out
        n //= 64
    return out


def iym_text(b: bytes, p: int) -> str:
    y = int.from_bytes(b[0:4], "big") - 0x80000000
    m = b[4] - 60
    sign = "-" if y < 0 or m < 0 else "+"
    return f"{sign}{abs(y):0{p}d}-{abs(m):02d}"


def ids_text(b: bytes, p: int, f: int) -> str:
    d = int.from_bytes(b[0:4], "big") - 0x80000000
    h, mi, s = b[4] - 60, b[5] - 60, b[6] - 60
    ns = int.from_bytes(b[7:11], "big") - 0x80000000
    sign = "-" if min(d, h, mi, s, ns) < 0 else "+"
    frac = f"{abs(ns):09d}"[:f]
    return f"{sign}{abs(d):0{p}d} {abs(h):02d}:{abs(mi):02d}:{abs(s):02d}" + (f".{frac}" if f else "")


def rowid_text(obj: int, fno: int, blk: int, row: int) -> str:
    return b64(obj, 6) + b64(fno, 3) + b64(blk, 6) + b64(row, 3)


def rowid_from_dump(b: bytes) -> str:
    x = int.from_bytes(b[4:8], "big")
    return rowid_text(int.from_bytes(b[0:4], "big"), x >> 22, x & 0x3FFFFF, int.from_bytes(b[8:10], "big"))


def rowid_from_urowid(b: bytes) -> str:
    assert b[0] == 1, "only the physical UROWID form (first byte 01) is known"
    return rowid_text(int.from_bytes(b[1:5], "big"), int.from_bytes(b[5:7], "big"),
                      int.from_bytes(b[7:11], "big"), int.from_bytes(b[11:13], "big"))


def urowid_text(b: bytes) -> str:
    """Session 9: physical (01) -> ROWIDTOCHAR text; logical (02) -> '*' + base 64 (A-Z a-z 0-9 + /) of
    the bytes after the first, 3 bytes -> 4 characters (only lengths divisible by 3 are in the fixtures)."""
    if b[0] == 1:
        return rowid_from_urowid(b)
    if b[0] == 2:
        rest = b[1:]
        assert len(rest) % 3 == 0, "logical UROWID: tail not a multiple of 3 bytes (not in the fixtures)"
        return "*" + "".join(b64(int.from_bytes(rest[i:i + 3], "big"), 4) for i in range(0, len(rest), 3))
    raise ValueError(f"UROWID first byte {b[0]:02x}")


def main() -> None:
    ok = bad = 0

    def rep(name: str, good: bool, detail: str = "") -> None:
        nonlocal ok, bad
        ok += good
        bad += not good
        print(f"    {name}: {'OK' if good else 'DIFF'} {detail}")

    for path in sys.argv[1:]:
        r = parse(path)
        tdir = os.path.dirname(os.path.abspath(path))
        two = os.path.basename(path).startswith("misc2_")          # session 9: R-036 file
        truth = [ln.split("|") for ln in open(os.path.join(tdir, "truth_misc2.txt" if two else "truth_misc.txt"),
                                               encoding="utf-8").read().split("\n") if ln]
        print(f"{os.path.basename(path)}: problems {r['problems'] or 'none'}")
        t_int, t_rid = ("T_INTERVAL2", "T_ROWID2") if two else ("T_INTERVAL", "T_ROWID")
        for t in r["tables"]:
            if t["name"] not in (t_int, t_rid):
                continue
            print(f"  {t['name']}: " + ", ".join(f"@0x{c['off']:x} type {c['type']} len {c['len']}" for c in t["cols"])
                  + f"; post {t['post_desc'][1]} flag {t['post_desc'][2]}")
            create = [it for it in t["pre_items"] if it["kind"] == "line" and it["text"].startswith("CREATE TABLE")][0]["text"]
            names = re.findall(r'"([^"]+)"', t["insert"].split(" VALUES ")[0])[1:]
            byid = {num_int(row[0][1]): row for _ro, row in t["rows"]}
            if t["name"] == t_int:
                # (column index, truth index of TO_CHAR, kind, p, f) for every interval column
                spec = []
                for ci, nm in enumerate(names[1:], 1):
                    m = re.search(r'"%s" INTERVAL YEAR \((\d+)\) TO MONTH' % nm, create)
                    if m:
                        spec.append((ci, nm, "ym", int(m.group(1)), 0))
                        continue
                    m = re.search(r'"%s" INTERVAL DAY \((\d+)\) TO SECOND \((\d+)\)' % nm, create)
                    spec.append((ci, nm, "ds", int(m.group(1)), int(m.group(2))))
                print("    precisions from the CREATE TABLE line: "
                      + ", ".join(f"{nm} {k} ({p}, {f_})" for _ci, nm, k, p, f_ in spec))
                for f in (x for x in truth if x[0] == "INTERVAL"):
                    i = int(f[1])
                    row = byid[i]
                    for j, (ci, nm, k, p, f_) in enumerate(spec):
                        o, v = row[ci]
                        want_txt, want_b = f[2 + 2 * j], dump_bytes(f[3 + 2 * j])
                        rep(f"id {i} {nm} @0x{o:x} bytes", v == want_b, f"{None if v is None else v.hex(' ')}")
                        if v is not None:
                            txt = iym_text(v, p) if k == "ym" else ids_text(v, p, f_)
                            rep(f"id {i} {nm} text", txt == want_txt, f"{txt!r} truth {want_txt!r}")
            else:
                for f in (x for x in truth if x[0] == "ROWID"):
                    i = int(f[1])
                    row = byid[i]
                    (o1, v1), (o2, v2) = row[1], row[2]
                    d_urid = dump_bytes(f[5] if two else f[4])
                    if f[2] == "NULL":
                        rep(f"id {i} RID NULL @0x{o1:x}", v1 is None)
                    else:
                        rep(f"id {i} RID @0x{o1:x} = prefix + ROWIDTOCHAR", v1 is not None and v1[-18:] == f[2].encode(),
                            f"len {len(v1)}, prefix {v1[:-18].hex(' ')}, text {v1[-18:].decode('latin-1')!r}")
                        rep(f"id {i} text from DUMP(rid)", rowid_from_dump(dump_bytes(f[3])) == f[2],
                            rowid_from_dump(dump_bytes(f[3])))
                    if d_urid is None:
                        rep(f"id {i} URID NULL", v2 is None)
                        continue
                    rep(f"id {i} URID @0x{o2:x} bytes == DUMP(urid)", v2 == d_urid, v2.hex(" "))
                    want_txt = f[4] if two else f[2]
                    rep(f"id {i} URID text (first byte {v2[0]:02x})", urowid_text(v2) == want_txt,
                        f"{urowid_text(v2)!r} truth {want_txt!r}")
    print(f"TOTAL OK {ok}, DIFF {bad}")


if __name__ == "__main__":
    main()
