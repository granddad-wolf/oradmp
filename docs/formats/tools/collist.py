#!/usr/bin/env python3
"""Parse the column list of every `CREATE TABLE` line of the legacy EXP fixtures with a stated grammar.

Checks (exp.md section 4.7, Analyst session 7, request R-025): the grammar below must consume the column
list of every `CREATE TABLE` line of every `*_exp_*.dmp` file (walked with exp_parse.py) and, for every
table section with an `INSERT INTO` line,
  - the number of column definitions equals the column count of the binary part (rule T4),
  - the quoted names equal, byte for byte and in order, the names of the `INSERT INTO` line,
  - the type text agrees with the descriptor type code (rule T5) and with the descriptor length by the
    table LEN below,
  - for T_DECL: precision and scale taken from the text equal data_precision / data_scale of
    truth_decl.txt, and data_type agrees.
Grammar (bytes of the line, client character set; tested here, see exp.md 4.7 for the status):
    line     := 'CREATE TABLE ' qname ' (' coldef (', ' coldef)* ')' '  ' storage-text
    coldef   := qname ' ' type [' NOT NULL ENABLE']
    qname    := '"' (any byte except '"')* '"'
    type     := 'NUMBER' | 'NUMBER(' p ', ' s ')' | 'NUMBER(*,' s ')' | 'FLOAT(' b ')'
              | 'VARCHAR2(' n [' CHAR'] ')' | 'CHAR(' n [' CHAR'] ')' | 'NVARCHAR2(' n ')' | 'NCHAR(' n ')'
              | 'RAW(' n ')' | 'DATE' | 'TIMESTAMP (' f ')' [' WITH TIME ZONE' | ' WITH LOCAL TIME ZONE']
              | 'CLOB' | 'LONG' | 'LONG RAW'
              | 'NCLOB' | 'BLOB' | 'ROWID' | 'UROWID(' n ')'                       (session 8)
              | 'INTERVAL YEAR (' p ') TO MONTH '      (note the space at the end)  (session 8)
              | 'INTERVAL DAY (' p ') TO SECOND (' f ')'                            (session 8)
    p, n, b, f := decimal digits; s := ['-'] decimal digits
Every line, name or type that the grammar does not consume is printed as PROBLEM.
Expects: fixture directories (default: the three folders of corpus/fixtures):
    uv run collist.py [dir ...] [-v]
-v prints every column definition with its descriptor. Reads the fixtures, never writes them.
New in Analyst session 7. Exploration tool, not product code.
Changed in Analyst session 8: the types of R-027 / R-029 (NCLOB, BLOB, ROWID, UROWID(n), the two INTERVAL
forms) with their descriptor codes and lengths (exp.md 4.1); CLOB now expects length 72.
Changed in Analyst session 9 (R-031): when the names of the INSERT INTO line differ from the CREATE TABLE
order, the section is on the direct path and the INSERT INTO order equals the CREATE TABLE order with the
LONG / LONG RAW column(s) moved to the end, the section is listed as "moved" (not a problem) and the
definitions are compared with the descriptors in the INSERT INTO order (exp.md 6.6).
Also session 9 (R-034): `VARCHAR2(n CHAR)` / `CHAR(n CHAR)` are checked against the descriptor length
min(width x n, 4000 / 2000), width 4 for set 873 (AL32UTF8) and 1 for the single-byte sets 1, 171, 178.
"""
import collections
import glob
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

TYPE_RX = re.compile(
    rb"(?P<t>NUMBER\((?P<np>\d+), (?P<ns>-?\d+)\)|NUMBER\(\*,(?P<ss>-?\d+)\)|NUMBER|FLOAT\((?P<fb>\d+)\)"
    rb"|(?P<vk>VARCHAR2|NVARCHAR2|NCHAR|CHAR|RAW)\((?P<n>\d+)(?P<sem> CHAR)?\)"
    rb"|TIMESTAMP \((?P<tf>\d+)\)(?P<tz> WITH TIME ZONE| WITH LOCAL TIME ZONE)?|DATE|NCLOB|CLOB|BLOB|LONG RAW|LONG|UROWID\((?P<ur>\d+)\)|ROWID"
    rb"|INTERVAL YEAR \((?P<iy>\d+)\) TO MONTH |INTERVAL DAY \((?P<id>\d+)\) TO SECOND \((?P<is>\d+)\))"
    rb"(?P<nn> NOT NULL ENABLE)?")


def parse_collist(line: bytes):
    """Return (table name, [(name bytes, type bytes, match, offset in line)], rest) or raise ValueError."""
    if not line.startswith(b'CREATE TABLE "'):
        raise ValueError("not CREATE TABLE")
    p = 13
    q = line.index(b'"', p + 1)
    tname = line[p + 1:q]
    p = q + 1
    if line[p:p + 2] != b" (":
        raise ValueError(f"no ' (' after the table name at +{p}")
    p += 2
    cols = []
    while True:
        if line[p:p + 1] != b'"':
            raise ValueError(f"no quoted name at +{p}")
        q = line.index(b'"', p + 1)
        name = line[p + 1:q]
        if line[q + 1:q + 2] != b" ":
            raise ValueError(f"no space after the name at +{q + 1}")
        m = TYPE_RX.match(line, q + 2)
        if m is None:
            raise ValueError(f"type not in the grammar at +{q + 2}: {line[q + 2:q + 40]!r}")
        cols.append((name, m.group("t"), m, p))
        p = m.end()
        if line[p:p + 2] == b", ":
            p += 2
            continue
        if line[p:p + 3] == b")  ":
            return tname, cols, line[p + 3:]
        raise ValueError(f"after a column definition neither ', ' nor ')  ' at +{p}: {line[p:p + 20]!r}")


def expect(m) -> tuple:
    """(type code, descriptor length or None if not fixed by the text, precision, scale, data_type)."""
    t = m.group("t")
    if m.group("np") is not None:
        return 2, 22, int(m.group("np")), int(m.group("ns")), "NUMBER"
    if m.group("ss") is not None:
        return 2, 22, None, int(m.group("ss")), "NUMBER"
    if t == b"NUMBER":
        return 2, 22, None, None, "NUMBER"
    if m.group("fb") is not None:
        return 2, 22, int(m.group("fb")), None, "FLOAT"
    if m.group("vk") is not None:
        k, n = m.group("vk").decode(), int(m.group("n"))
        code = {"VARCHAR2": 1, "NVARCHAR2": 1, "CHAR": 96, "NCHAR": 96, "RAW": 23}[k]
        if k in ("NVARCHAR2", "NCHAR"):
            ln = 2 * n
        elif m.group("sem"):
            ln = None                    # character semantics: depends on the database set (exp.md 4.1)
        else:
            ln = n
        return code, ln, None, None, k
    if m.group("tf") is not None:
        f, tz = int(m.group("tf")), m.group("tz")
        if tz == b" WITH TIME ZONE":
            return 181, 13, None, f, f"TIMESTAMP({f}) WITH TIME ZONE"
        if tz == b" WITH LOCAL TIME ZONE":
            return 231, 11 if f else 7, None, f, f"TIMESTAMP({f}) WITH LOCAL TIME ZONE"
        return 180, 11 if f else 7, None, f, f"TIMESTAMP({f})"
    if m.group("ur") is not None:
        return 208, int(m.group("ur")), None, None, "UROWID"
    if m.group("iy") is not None:
        return 182, 5, int(m.group("iy")), None, f"INTERVAL YEAR({m.group('iy').decode()}) TO MONTH"
    if m.group("id") is not None:
        return 183, 11, int(m.group("id")), int(m.group("is")), \
            f"INTERVAL DAY({m.group('id').decode()}) TO SECOND({m.group('is').decode()})"
    return {b"DATE": (12, 7, None, None, "DATE"), b"CLOB": (112, 72, None, None, "CLOB"),
            b"NCLOB": (112, 72, None, None, "NCLOB"), b"BLOB": (113, 72, None, None, "BLOB"),
            b"ROWID": (69, 23, None, None, "ROWID"),
            b"LONG": (8, 0, None, None, "LONG"), b"LONG RAW": (24, 0, None, None, "LONG RAW")}[t]


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    verbose = "-v" in sys.argv
    dirs = [a for a in sys.argv[1:] if not a.startswith("-")] or \
        [os.path.join(here, "../../../corpus/fixtures", v) for v in ("18c", "11g", "11g-cp1251")]
    files = sorted(f for d in dirs for f in glob.glob(os.path.join(d, "*_exp_*.dmp")))
    problems = []
    reordered: list = []
    charsem: list = []
    forms: collections.Counter = collections.Counter()
    nlines = ncols = 0
    for f in files:
        base = os.path.basename(f)
        r = parse(f)
        tdir = os.path.dirname(os.path.abspath(f))
        for s in r["sections"]:
            ct = next(it for it in r["items"] if it["off"] > s["table_off"])
            line = ct["raw"]
            nlines += 1
            try:
                tname, cols, _rest = parse_collist(line)
            except ValueError as ex:
                problems.append(f"{base}@0x{ct['off']:x}: {ex}")
                continue
            for _n, t, m, _p in cols:
                forms[re.sub(rb"\d+", b"n", t).decode("latin-1") + (" NOT NULL ENABLE" if m.group("nn") else "")] += 1
            if not s.get("has_insert"):
                continue
            ins = s["insert_raw"]
            inames = re.findall(rb'"([^"]*)"', ins.split(b") VALUES (")[0])[1:]
            if [n for n, *_ in cols] != inames:
                # session 9: on the direct path the LONG / LONG RAW column stands last in INSERT INTO and descriptors
                moved = [c for c in cols if not c[2].group("t").startswith(b"LONG")] + \
                        [c for c in cols if c[2].group("t").startswith(b"LONG")]
                if s.get("direct") and [n for n, *_ in moved] == inames:
                    reordered.append(f"{base}@0x{ct['off']:x} {s['name']}: {[n.decode() for n, *_ in cols]} -> "
                                     f"{[n.decode() for n in inames]}")
                    cols = moved
                else:
                    problems.append(f"{base}@0x{ct['off']:x}: names differ from INSERT INTO: {[n for n, *_ in cols]} / {inames}")
            if len(cols) != len(s["cols"]):
                problems.append(f"{base}@0x{ct['off']:x}: {len(cols)} definitions, {len(s['cols'])} descriptors")
                continue
            truth = {}
            if s["name"] == "T_DECL":
                for ln in open(os.path.join(tdir, "truth_decl.txt"), encoding="utf-8").read().split("\n"):
                    if ln:
                        x = ln.split("|")
                        truth[x[1].encode()] = (x[2], None if x[3] == "NULL" else int(x[3]),
                                                None if x[4] == "NULL" else int(x[4]), int(x[5]), x[6])
            for (name, t, m, p), dsc in zip(cols, s["cols"]):
                ncols += 1
                code, ln, prec, scale, dtype = expect(m)
                if ln is None and m.group("sem"):
                    # session 9 (R-034): character semantics: min(width x n, cap), width = the largest character
                    # of the descriptor's set (873 AL32UTF8: 4; the single-byte sets: 1), cap 4000 / 2000
                    width = {873: 4, 171: 1, 178: 1, 1: 1}.get(dsc.get("csid"))
                    if width is not None:
                        ln = min(width * int(m.group("n")), 4000 if code == 1 else 2000)
                        charsem.append(f"{base} {s['name']}.{name.decode('latin-1')} {t.decode()} set "
                                       f"{dsc.get('csid')}: descriptor {dsc['len']}, rule {ln}")
                ok = dsc["type"] == code and (ln is None or dsc["len"] == ln)
                if not ok:
                    problems.append(f"{base}@0x{ct['off'] + p:x}: {name!r} {t!r}: expected type {code} len {ln}, "
                                    f"descriptor @0x{dsc['off']:x} type {dsc['type']} len {dsc['len']}")
                if verbose:
                    print(f"{base}@0x{ct['off'] + p:x} {name.decode('latin-1')!r} {t.decode()!r}"
                          f"{' NOT NULL' if m.group('nn') else ''} -> type {code} p {prec} s {scale}; descriptor "
                          f"@0x{dsc['off']:x} {dsc['type']}/{dsc['len']}")
                if truth:
                    tt = truth.get(name)
                    if tt is None:
                        problems.append(f"{base}: {name!r} not in truth_decl.txt")
                        continue
                    good = (tt[1], tt[2]) == (prec, scale) and tt[0] == dtype and tt[3] == dsc["len"] \
                        and (tt[4] == "N") == bool(m.group("nn"))
                    print(f"  T_DECL {base}@0x{ct['off'] + p:x} {name.decode()!r:12} text {t.decode()!r:16} "
                          f"-> {dtype} p={prec} s={scale} nn={bool(m.group('nn'))} | truth {tt[0]} p={tt[1]} "
                          f"s={tt[2]} len={tt[3]} nullable={tt[4]} | descriptor {dsc['type']}/{dsc['len']} "
                          f"{'OK' if good else 'DIFF'}")
                    if not good:
                        problems.append(f"{base}: T_DECL {name!r} text {t!r} gives {dtype} {prec} {scale}, truth {tt}")
    print(f"files {len(files)}, CREATE TABLE lines {nlines}, column definitions checked against descriptors {ncols}")
    print("type forms seen:")
    for k, v in sorted(forms.items()):
        print(f"  {v:5d}  {k}")
    print(f"character-semantics columns checked by min(width x n, cap): {len(charsem)}")
    if verbose:
        for x in charsem:
            print("  " + x)
    print(f"direct path, LONG column moved last in INSERT INTO and descriptors: {len(reordered)} sections")
    for x in reordered:
        print("  " + x)
    print(f"PROBLEMS: {len(problems)}")
    for x in problems:
        print("  " + x)


if __name__ == "__main__":
    main()
