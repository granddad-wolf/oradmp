#!/usr/bin/env python3
"""Compare the rows decoded from a legacy EXP fixture with the truth files, row by row.

Checks: for every table in the dump that has a truth file, that the value bytes in the dump
equal the `DUMP(col,16)` bytes of the truth file (t_num, t_date, t_ts, t_tstz, t_text,
t_numtypes, t_len), the hex of the truth file (t_raw), or the text of the truth file (t_frame,
t_probe6, t_nullmix, t_wide, t_ddl). For t_empty it checks "0 rows" against truth_counts.txt.
Prints one line per row with the file offset of the compared field and OK / DIFF.
Also reports whether the rows are in truth order.
Expects: one `*_exp_*.dmp` fixture of corpus/fixtures/18c, 11g or 11g-cp1251; the truth files are
read from the directory of the fixture, e.g.
    uv run check_truth.py ../../../corpus/fixtures/18c/t_num_exp_18c.dmp [-q]
(-q prints only the per-table summary). Uses exp_parse.py from the same directory.
Importable: `check(path, quiet=True, items="generic", empty="text")` returns the parse result
and one summary dict per table (used by check_all.py).
Exploration tool only. Reads the fixtures, never writes them.

The ID columns are NUMBERs; they are decoded with the positive-integer rule that t_num rows
2, 6, 7, 8, 10, 11 confirm (first byte 0xc1 + exponent, then base-100 digits + 1).

Changed in Analyst session 9 (R-031 to R-037):
- New tables: T_NUM3 (truth_num3.txt), T_NCHAR2 (truth_nchar2.txt: DUMP of NC, NV, CC, VC; CL and NCL whole
  values as UTF-16 BE from GETLENGTH and the DUMP of SUBSTR), T_PROBE11 (truth_probe11.txt ROW lines; Y and Z
  by LENGTHB + the inserted text + spaces), T_LOB3 (truth_lob3.txt lengths + 14_lob3.sql contents),
  T_INTERVAL2 and T_ROWID2 (truth_misc2.txt), T_LONG2 (truth_long2.txt; the refused row 2 is skipped),
  T_LONG_EMPTY (0 rows), T_PART2 (one entry per subpartition; ids from the ROW lines of truth_part2.txt,
  K and V from 14_part2.sql).
- An expectation can be a dict by column name; the names come from the INSERT INTO line. T_LONG and T_LONG2
  use it because the direct path writes the LONG column last (exp.md 6.6).

Changed in Analyst session 8:
- New tables of R-027, R-029, R-030: T_INTERVAL (truth_misc.txt, DUMP bytes of IYM and IDS), T_ROWID
  (truth_misc.txt: RID = 5 bytes + the 18 characters of ROWIDTOCHAR, compared as "length 23 and ends
  with the text"; URID = DUMP bytes), T_PART (one table entry per partition; every row computed from
  01b_optional_tables.sql, restricted to the partition's date range, reported as "sql"; the row counts
  per partition are checked by part_check.py against truth_part.txt), T_LOB and T_LOB2 (ID and V here;
  the LOB values decoded by the session 8 rules of exp_parse.py are compared with truth_lob.txt and the
  SQL by lob_check.py, and here with the SQL values of lob_check.py: types 112 / 113 counted apart).

Changed in Analyst session 7:
- New tables of R-024 to R-028: T_NUM2 (truth_num2.txt, DUMP bytes), T_NCHAR (truth_nchar.txt, DUMP
  bytes of NC, CC, NV), T_DECL (one row; values from corpus/sql/11_types.sql, reported as "sql"),
  T_LONG and T_LONGRAW (values from 11_types.sql, reported as "sql"; the LONG / LONG RAW columns
  are read with the 4-byte descriptor that exp_parse.py gives every code outside CHAR_TYPES).

Changed in Analyst session 6:
- New tables of R-020: T_STC1 to T_STC5 (truth_stc.txt has statistics lines only, no rows: every
  column of the 50 rows is compared with what corpus/sql/10_stc.sql must produce, reported as "sql").

Changed in Analyst session 10 (R-038):
- R_LOB2 of corpus/fixtures/11g-cp1251 (truth_lob_ru.txt): ids and NULL / locator per LOB column; every LOB value as
  whole bytes against corpus/sql/15_ru_lob2.sql (C = database bytes, N = UTF-16 BE, B = bytes) and against the
  GETLENGTH and the head / tail DUMPs of the truth. Before, R_LOB2 had no branch, fell into the branch of the
  Cyrillic-named table (any unknown table of a folder with truth_ru.txt) and stopped check_all.py and num_decode.py
  with a TypeError in num_int (NULL LOB field of row 3 taken as the key).

Changed in Analyst session 5:
- New tables of R-016: T_STAT1 (truth_stat.txt, lines `ROW|id|v|n|date`), T_STAT2 and T_STAT3
  (`CREATE TABLE ... AS SELECT * FROM t_stat1` in corpus/sql/09_stat.sql: compared with the same
  ROW lines and reported as "sql", because the truth file has rows for T_STAT1 only), T_STAT4
  (0 rows: `TAB|T_STAT4|0|...` of truth_stat.txt and the export log) and R_STAT of
  corpus/fixtures/11g-cp1251 (no truth rows: all 7 columns of the 2000 rows are compared with
  what corpus/sql/08_ru_stats_objects.sql must produce and reported as "sql").
- Before this change R_STAT fell into the branch written for the table with the Cyrillic name
  (any unknown table of a folder that has truth_ru.txt) and was reported as "DIFF 2000".

Changed in Analyst session 4:
- New tables: O_PARENT, O_CHILD, O_EMPTY (truth_own.txt; R-012), T_BN1 to T_BN4 (truth_bn1.txt to
  truth_bn4.txt; R-013), R_TEXT and R_CLOB (truth_ru.txt of corpus/fixtures/11g-cp1251; R-015) and
  the table with the Cyrillic name of the same folder (no truth rows: compared with the INSERT
  statements of corpus/sql/07_ru_objects.sql and reported as "sql"; when the name was exported as
  replacement characters (`???_???`, or `bf bf bf 5f bf bf bf` with a WE8MSWIN1252 client) the expectation is "0 rows, cut short", from the export log).
- The truth of T_BN1 to T_BN4 gives the length and the first 4 bytes / 5 characters only, and
  truth_ru.txt gives the length and the first 60 bytes of the LV column only. The rest of each
  value is compared with what the generating SQL (06_bn.sql, 07_ru_objects.sql) must produce; a
  mismatch there is a DIFF too. The summary line says which part is truth and which is SQL.
- R_CLOB: the LOB data chunks that exp_parse.py finds with the hypothesis of exp.md 14.6 are
  compared with the length and the first 60 bytes of the truth. The locator field is not compared.
- The table summary carries `lob` and the per-type counts (`types`) used by check_all.py.

Changed in Analyst session 3:
- T_RAW: reads the corrected truth_raw.txt (R-010): lines `id|len=<n>|part=1|<hex>` and, for
  values above 1000 bytes, `id|len=<n>|part=2|<hex>`; the two parts are joined and every byte
  and the length are compared. NULL is `<NULL>`.
- T_WIDE (truth_wide.txt, 120 text columns), T_DDL (truth_ddl.txt; AMOUNT is decoded with the
  positive NUMBER rule that t_num rows 12 and 17 confirm), T_EMPTY (0 rows, truth_counts.txt)
  are now compared. T_PARENT has no truth file; its two rows are compared with the INSERT
  statements of corpus/sql/01_corpus_tables.sql and reported as "sql", not as truth.
- T_TEXT works unchanged on the R-008 files (us7ascii / w1252 clients): the value bytes there
  are the database bytes of truth_text.txt (exp.md section 6.3).
- The comparison is a function (`check`) so that check_all.py can count tables per fixture.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402


def num_int(b: bytes) -> int:
    """Positive integer NUMBER only (enough for the id columns)."""
    if b == b"\x80":
        return 0
    e = b[0] - 0xC0
    digs = [x - 1 for x in b[1:]] + [0] * (e - (len(b) - 1))
    v = 0
    for x in digs[:e]:
        v = v * 100 + x
    return v


def num_pos(b: bytes) -> str:
    """Positive NUMBER -> decimal text (rule: t_num rows 2, 8, 12, 17). Enough for T_DDL.AMOUNT."""
    from decimal import Decimal
    e = b[0] - 0xC1
    v = Decimal(0)
    for i, x in enumerate(b[1:]):
        v += Decimal(x - 1) * (Decimal(100) ** (e - i))
    return format(v.normalize(), "f")


def dump_bytes(s: str):
    """'Typ=2 Len=3: c2,2,18' -> bytes; 'NULL' -> None."""
    if s.strip() == "NULL":
        return None
    return bytes(int(x, 16) for x in s.split(": ", 1)[1].split(","))


def dump_len(s: str) -> int:
    return int(re.search(r"Len=(\d+)", s).group(1))


def date_bytes(s: str):
    """' 2020-02-29 00:00:00' -> 7 bytes by the rule that truth_date.txt confirms (AD only)."""
    if s == "<NULL>":
        return None
    y, mo, dd, hh, mi, ss = map(int, re.split(r"[- :]", s.strip()))
    return bytes([y // 100 + 100, y % 100 + 100, mo, dd, hh + 1, mi + 1, ss + 1])


def expected(table: str, tdir: str, partition=None, subpartition=None):
    """Return (key function, {key: [expected bytes or None or Ellipsis, ...]}, note)."""
    def lines(name):
        return open(os.path.join(tdir, name), encoding="utf-8", newline="").read().split("\n")

    rows = {}
    if table == "T_FRAME":
        for i, ln in enumerate(x for x in lines("truth_frame.txt") if x):
            rows[i] = [None if v == "<NULL>" else v.encode() for v in ln.split("|")]
        return None, rows, "text; key = row position"
    if table == "T_PROBE6":
        for ln in (x for x in lines("truth_probe6.txt") if x):
            a, b = ln.split("|")
            rows[a.encode()] = [a.encode(), b.encode()]
        return (lambda r: r[0]), rows, "text; key = ID value"
    if table == "T_NULLMIX":
        for ln in (x for x in lines("truth_nullmix.txt") if x):
            i, n, s, dt = ln.split("|")
            rows[int(i)] = [..., None if n == "<NULL>" else bytes.fromhex("c10c"),
                            None if s == "<NULL>" else s.encode(), date_bytes(dt)]
        return (lambda r: num_int(r[0])), rows, "N=11 as c1 0c (rule from t_num), D by the t_date rule"
    if table in ("T_NUM", "T_DATE"):
        for ln in (x for x in lines(f"truth_{table[2:].lower()}.txt") if x):
            f = ln.split("|")
            rows[int(f[0])] = [..., dump_bytes(f[2])]
        return (lambda r: num_int(r[0])), rows, "DUMP(col,16) bytes"
    if table == "T_LEN":
        for ln in (x for x in lines("truth_len.txt") if x):
            f = ln.split("|")
            if f[2] == "NULL":
                rows[int(f[0])] = [..., None]
            else:
                rows[int(f[0])] = [..., ("prefix", dump_len(f[2]), dump_bytes(f[2]))]
        return (lambda r: num_int(r[0])), rows, "length + first 12 bytes (truth has only 12)"
    if table == "T_TS":
        for ln in (x for x in lines("truth_ts.txt") if x):
            f = ln.split("|")
            rows[int(f[0])] = [..., dump_bytes(f[2]), dump_bytes(f[4]), dump_bytes(f[6])]
        return (lambda r: num_int(r[0])), rows, "DUMP(col,16) bytes"
    if table == "T_TSTZ":
        for ln in (x for x in lines("truth_tstz.txt") if x):
            f = ln.split("|")
            rows[int(f[0])] = [..., dump_bytes(f[2]), dump_bytes(f[4])]
        return (lambda r: num_int(r[0])), rows, "DUMP(col,16) bytes"
    if table == "T_NUMTYPES":
        for ln in (x for x in lines("truth_numtypes.txt") if x):
            f = ln.split("|")
            rows[int(f[0])] = [..., ..., dump_bytes(f[5]), dump_bytes(f[6]), ...]
        return (lambda r: num_int(r[0])), rows, "DUMP bytes of N_P10S2 and N_NEG only"
    if table == "T_TEXT":
        # a value contains LF (row 9), so split the file on the row pattern instead of on lines
        txt = open(os.path.join(tdir, "truth_text.txt"), encoding="utf-8", newline="").read()
        for m in re.finditer(r"(?m)^(\d+)\|v_chars=([^|]*)\|v_bytes=([^|]*)\|([^|]*)\|([^|]*)\|([^|\n]*)$", txt):
            rows[int(m.group(1))] = [..., dump_bytes(m.group(4)), dump_bytes(m.group(5)),
                                     dump_bytes(m.group(6))]
        return (lambda r: num_int(r[0])), rows, "DUMP(col,1016) bytes"
    if table == "T_RAW":
        parts: dict = {}
        for ln in (x for x in lines("truth_raw.txt") if x):
            i, ln_, part, hx = ln.split("|")
            parts.setdefault(int(i), {})[part] = (ln_[4:], hx)
        for i, pp in parts.items():
            n = pp["part=1"][0]
            if n == "<NULL>":
                rows[i] = [..., None]
                continue
            val = bytes.fromhex(pp["part=1"][1] + (pp["part=2"][1] if "part=2" in pp else ""))
            assert len(val) == int(n), f"truth_raw.txt row {i}: len={n} but {len(val)} bytes of hex"
            rows[i] = [..., val]
        return (lambda r: num_int(r[0])), rows, "all bytes (truth_raw.txt part 1 + part 2)"
    if table == "T_WIDE":
        for i, ln in enumerate(x for x in lines("truth_wide.txt") if x.strip()):
            rows[i] = [None if v.strip() == "<NULL>" else v.strip().encode() for v in ln.split("|")]
        return None, rows, "text; key = row position"
    if table == "T_DDL":
        for ln in (x for x in lines("truth_ddl.txt") if x):
            i, par, st, am, cr = ln.split("|")
            rows[int(i)] = [..., None if par == "<NULL>" else ("int", int(par)), st.encode(),
                            None if am == "<NULL>" else ("num", am), date_bytes(cr)]
        return (lambda r: num_int(r[0])), rows, "PARENT_ID / AMOUNT decoded, STATUS text, CREATED by the t_date rule"
    if table == "T_PARENT":
        rows = {1: [..., b"p1"], 2: [..., b"p2"]}
        return (lambda r: num_int(r[0])), rows, "sql (01_corpus_tables.sql lines 299-300; no truth file)"
    if table in ("O_PARENT", "O_CHILD", "O_EMPTY"):
        for ln in (x for x in lines("truth_own.txt") if x.startswith(table + "|")):
            f = ln.split("|")[1:]
            if table == "O_EMPTY":
                assert f == ["count=0"]
                return None, {}, "0 rows (truth_own.txt)"
            if table == "O_PARENT":
                rows[int(f[0])] = [..., f[1].encode()]
            else:
                rows[int(f[0])] = [..., None if f[1] == "<NULL>" else ("int", int(f[1])), f[2].encode(),
                                   None if f[3] == "<NULL>" else f[3].encode()]
        return (lambda r: num_int(r[0])), rows, "truth_own.txt (ID / PARENT_ID decoded, text columns as bytes)"
    if table in ("T_BN1", "T_BN2"):
        for ln in (x for x in lines(f"truth_bn{table[-1]}.txt") if x):
            i, ln_, hx = ln.split("|")
            first = bytes.fromhex(hx)
            rows[int(i)] = [..., ("fill", int(ln_[4:]), first, first[:1])]
        return (lambda r: num_int(r[0])), rows, "truth: length + first 4 bytes; rest: all bytes equal the first (06_bn.sql)"
    if table in ("T_BN3", "T_BN4"):
        for ln in (x for x in lines(f"truth_bn{table[-1]}.txt") if x):
            i, ln_, pre = ln.split("|")
            assert i == "<NULL>"
            rows[pre.encode()] = [None, ("fill", int(ln_[4:]), pre.encode(), b"x")]
        return (lambda r: r[1][:5]), rows, "truth: ID NULL, length + first 5 characters; rest: all 'x' (06_bn.sql)"
    if table == "R_TEXT":
        lv = bytes(0xC0 + (i * 7) % 64 for i in range(1, 3001))   # 07_ru_objects.sql, in CL8MSWIN1251
        for ln in (x for x in lines("truth_ru.txt") if x.startswith("R_TEXT|")):
            f = ln.split("|")
            i = int(f[1])
            if f[6] == "NULL":
                lvw = None
            else:
                full = lv if i == 8 else lv[:2000] if i == 9 else lv[:600 + i] if i >= 10 else dump_bytes(f[8])
                lvw = ("full", int(f[6]), dump_bytes(f[8]), full)
            dt = None if f[9] == "NULL" else date_bytes(f[9] + " 00:00:00")
            rows[i] = [..., dump_bytes(f[2]), dump_bytes(f[3]), dump_bytes(f[4]), dump_bytes(f[5]), lvw, dt]
        return (lambda r: num_int(r[0])), rows, ("truth_ru.txt DUMP bytes of V, C, VC, N; LV: length + first 60 "
                                                 "bytes (truth), all bytes (07_ru_objects.sql); D by the t_date rule")
    if table == "R_CLOB":
        for ln in (x for x in lines("truth_ru.txt") if x.startswith("R_CLOB|")):
            f = ln.split("|")
            rows[int(f[1])] = [..., ...]
        return (lambda r: num_int(r[0])), rows, "ids only here; LOB data are compared separately (hypothesis, exp.md 14.6)"
    if table in ("T_STAT1", "T_STAT2", "T_STAT3"):
        for ln in (x for x in lines("truth_stat.txt") if x.startswith("ROW|")):
            _r, i, v, n, dt = ln.split("|")
            rows[int(i)] = [..., v.encode(), ("int", int(n)), date_bytes(dt + " 00:00:00")]
        if table == "T_STAT1":
            return (lambda r: num_int(r[0])), rows, "truth_stat.txt ROW lines: V text, N decoded, D by the t_date rule"
        return (lambda r: num_int(r[0])), rows, ("sql (09_stat.sql: CREATE TABLE AS SELECT * FROM t_stat1; "
                                                 "compared with the ROW lines of T_STAT1)")
    if table == "T_STAT4":
        f = [x for x in lines("truth_stat.txt") if x.startswith("TAB|T_STAT4|")][0].split("|")
        assert f[2] == "0"
        return None, {}, "0 rows (truth_stat.txt TAB|T_STAT4 num_rows 0; export log)"
    if table in ("T_STC1", "T_STC2", "T_STC3", "T_STC4", "T_STC5"):
        from datetime import date, timedelta
        for i in range(1, 51):                           # 10_stc.sql: CONNECT BY LEVEL <= 50
            base = [..., b"a%d" % (i % 7), b"b%d" % i,
                    date_bytes((date(2020, 1, 1) + timedelta(days=i)).isoformat() + " 00:00:00")]
            # T_STC2: two columns added later, ADDED_LATER left NULL, ADDED_NUM = MOD(id, 3); the others: ALLNULL
            rows[i] = base + ([None, ("int", i % 3)] if table == "T_STC2" else [None])
        return (lambda r: num_int(r[0])), rows, "sql (10_stc.sql: all columns computed from the INSERT ... CONNECT BY; no truth rows)"
    if table == "R_STAT":
        from datetime import date, timedelta
        from decimal import ROUND_HALF_UP, Decimal
        for i in range(1, 2001):                         # 08_ru_stats_objects.sql, the PL/SQL loop
            st, ln_ = (i * 13) % 50, 3 + i % 6           # word(i): code points 1040+st .., in CL8MSWIN1251 0xc0+st ..
            txt = None if i % 17 == 0 else bytes(0xC0 + st + k for k in range(ln_))
            grp = 1 if i % 2 == 0 else 2 if i % 5 == 0 else i % 7 + 3
            dt = date(2026, 1, 1) + timedelta(days=(i * 3) % 200)
            amt = None if i % 11 == 0 else ("num", format(
                (Decimal((i * 37) % 1000) / Decimal(7)).quantize(Decimal("0.01"), ROUND_HALF_UP).normalize(), "f"))
            rows[i] = [..., ("int", grp), txt, b"K%03d  " % (i % 50), date_bytes(dt.isoformat() + " 00:00:00"),
                       amt, None if i % 7 == 0 else b"note %d" % i]
        return (lambda r: num_int(r[0])), rows, "sql (08_ru_stats_objects.sql: all 7 columns computed from the loop; no truth rows)"
    if table == "R_LOB2":
        # session 10 (R-038): ids and NULL / locator per LOB column; LOB values are checked by lob_ru_check.py
        for ln in (x for x in lines("truth_lob_ru.txt") if x.startswith("R_LOB2|")):
            f = ln.split("|")
            rows[int(f[1])] = [...] + [None if f[k] == "NULL" else ... for k in (2, 3, 4)]
        return (lambda r: num_int(r[0])), rows, "truth_lob_ru.txt ids, NULL per LOB column; LOB values: whole bytes by 15_ru_lob2.sql, GETLENGTH and head / tail DUMPs by truth_lob_ru.txt"
    if os.path.exists(os.path.join(tdir, "truth_ru.txt")):
        # the table with the Cyrillic name; its name bytes depend on the client character set
        if set(table) <= set("?\xbf_"):
            # US7ASCII client writes `?` (3f), WE8MSWIN1252 client writes 0xbf for each Cyrillic letter
            return None, {}, "sql/log: name exported as replacement characters, the log reports ORA-00942: 0 rows expected (cut short)"
        rows = {1: [bytes.fromhex("e0e1e2"), ...], 2: [None, ...]}
        return (lambda r: num_int(r[1])), rows, "sql (07_ru_objects.sql: ('abv' in Cyrillic = e0 e1 e2, 1), (NULL, 2); no truth rows)"
    if table == "T_NUM2":
        for ln in (x for x in lines("truth_num2.txt") if x):
            f = ln.split("|")
            rows[int(f[0])] = [..., dump_bytes(f[2])]
        return (lambda r: num_int(r[0])), rows, "DUMP(v,16) bytes (truth_num2.txt)"
    if table == "T_NCHAR":
        for ln in (x for x in lines("truth_nchar.txt") if x):
            f = ln.split("|")
            rows[int(f[0])] = [..., dump_bytes(f[1]), dump_bytes(f[2]), dump_bytes(f[3])]
        return (lambda r: num_int(r[0])), rows, "DUMP(col,16) bytes of NC, CC, NV (truth_nchar.txt)"
    if table == "T_DECL":
        # 11_types.sql: one row; no truth file for the values. NUMBER bytes written here by the rule of
        # exp.md 6.1 (session 7), TS_DEF / D_NN by the TIMESTAMP / DATE rules, V_DEF = the DEFAULT 'a,b)'.
        rows[1] = [..., ("num", "1.25"), ("int", 1), ("int", 100), ("num", "0.00123"), ("int", 1),
                   ("num", "1.25"), ("num", "1.5"), ("num", "1.5"), ("num", "1.5"),
                   date_bytes(" 2020-01-02 03:04:05") + (123456000).to_bytes(4, "big"),
                   date_bytes(" 2020-01-02 00:00:00"), b"a,b)", b"x"]
        return (lambda r: num_int(r[0])), rows, "sql (11_types.sql; no truth rows)"
    if table == "T_INTERVAL":
        for ln in (x for x in lines("truth_misc.txt") if x.startswith("INTERVAL|")):
            f = ln.split("|")
            rows[int(f[1])] = [..., dump_bytes(f[3]), dump_bytes(f[5])]
        return (lambda r: num_int(r[0])), rows, "DUMP(col,16) bytes (truth_misc.txt)"
    if table == "T_ROWID":
        for ln in (x for x in lines("truth_misc.txt") if x.startswith("ROWID|")):
            f = ln.split("|")
            rows[int(f[1])] = [..., None if f[2] == "NULL" else ("suffix", 23, f[2].encode()), dump_bytes(f[4])]
        return (lambda r: num_int(r[0])), rows, "RID: length 23 ending in ROWIDTOCHAR; URID: DUMP bytes (truth_misc.txt)"
    if table == "T_PART":
        import datetime as dt
        lo, hi = {"P2019": (None, dt.date(2020, 1, 1)), "P2020": (dt.date(2020, 1, 1), dt.date(2021, 1, 1)),
                  "PMAX": (dt.date(2021, 1, 1), None)}[partition]
        for i in range(1, 3001):                         # 01b_optional_tables.sql, CONNECT BY LEVEL <= 3000
            day = dt.date(2019, 1, 1) + dt.timedelta(days=(i * 13) % 1000)
            if (lo is None or day >= lo) and (hi is None or day < hi):
                amt = (i * 31) % 100000
                rows[i] = [..., date_bytes(" " + day.isoformat() + " 00:00:00"),
                           ("num", format((__import__("decimal").Decimal(amt) / 100).normalize(), "f")) if amt else b"\x80"]
        return (lambda r: num_int(r[0])), rows, f"sql (01b_optional_tables.sql, rows of partition {partition})"
    if table in ("T_LOB", "T_LOB2"):
        for ln in (x for x in lines("truth_lob.txt") if x.startswith(table + "|")):
            f = ln.split("|")
            rows[int(f[1])] = [...] * (3 if table == "T_LOB" else 5)
            if table == "T_LOB2":
                rows[int(f[1])][2] = f[3].encode()
        return (lambda r: num_int(r[0])), rows, "ids (and V) here; LOB values against the SQL (lob_check.py sql_values)"
    if table == "T_LONG":
        # session 9: by column name, because the direct path writes the LONG column last (exp.md 6.6)
        rows = {1: {"ID": ..., "L": b"short", "V": b"a"}, 2: {"ID": ..., "L": None, "V": b"b"},
                3: {"ID": ..., "L": b"L" + b"x" * 31999, "V": b"c"}}
        return (lambda r: num_int(r[0])), rows, "sql (11_types.sql; no truth rows; columns by name)"
    if table == "T_LONG2":
        for ln in (x for x in lines("truth_long2.txt") if x):
            f = ln.split("|")
            if f[1] == "INSERT REFUSED":
                continue                                 # row 2 was refused by Oracle (ORA-22835)
            val = bytes.fromhex(f[2])
            assert len(val) == int(f[1])
            rows[int(f[0])] = {"ID": ..., "L": val, "V": b"a"}
        return (lambda r: num_int(r[0])), rows, "truth_long2.txt (L: LENGTHB + all bytes), V: sql (14_long2.sql); columns by name"
    if table == "T_LONG_EMPTY":
        return None, {}, "0 rows (14_long2.sql creates it empty; export log)"
    if table == "T_NUM3":
        for ln in (x for x in lines("truth_num3.txt") if x):
            f = ln.split("|")
            rows[int(f[0])] = [..., dump_bytes(f[2])]
        return (lambda r: num_int(r[0])), rows, "DUMP(v,16) bytes (truth_num3.txt)"
    if table == "T_NCHAR2":
        for ln in (x for x in lines("truth_nchar2.txt") if x):
            f = ln.split("|")
            rows[int(f[0])] = [..., dump_bytes(f[1]), dump_bytes(f[2]), dump_bytes(f[3]), dump_bytes(f[4]), ..., ...]
        return (lambda r: num_int(r[0])), rows, "DUMP(col,16) of NC, NV, CC, VC; CL, NCL values below (truth_nchar2.txt)"
    if table == "T_PROBE11":
        for ln in (x for x in lines("truth_probe11.txt") if x.startswith("ROW|")):
            f = ln.split("|")[1:]
            w = dump_bytes(f[4])                         # W holds the inserted text; Y, Z are it padded with spaces
            rows[int(f[0])] = [..., dump_bytes(f[1]), dump_bytes(f[2]), dump_bytes(f[3]), w, dump_bytes(f[5]),
                               ("fill", int(f[6]), w, b" "), ("fill", int(f[7]), w, b" ")]
        return (lambda r: num_int(r[0])), rows, "DUMP bytes of B, C, K, W, X; Y, Z: LENGTHB + the W text + spaces (truth_probe11.txt)"
    if table == "T_LOB3":
        for ln in (x for x in lines("truth_lob3.txt") if x.startswith("T_LOB3|")):
            rows[int(ln.split("|")[1])] = [..., ..., ..., ...]
        return (lambda r: num_int(r[0])), rows, "ids here; LOB values below (truth_lob3.txt + 14_lob3.sql)"
    if table == "T_INTERVAL2":
        for ln in (x for x in lines("truth_misc2.txt") if x.startswith("INTERVAL|")):
            f = ln.split("|")
            rows[int(f[1])] = [..., dump_bytes(f[3]), dump_bytes(f[5]), dump_bytes(f[7])]
        return (lambda r: num_int(r[0])), rows, "DUMP(col,16) bytes (truth_misc2.txt)"
    if table == "T_ROWID2":
        for ln in (x for x in lines("truth_misc2.txt") if x.startswith("ROWID|")):
            f = ln.split("|")
            rows[int(f[1])] = [..., None if f[2] == "NULL" else ("suffix", 23, f[2].encode()), dump_bytes(f[5])]
        return (lambda r: num_int(r[0])), rows, "RID: length 23 ending in ROWIDTOCHAR; URID: DUMP bytes (truth_misc2.txt)"
    if table == "T_PART2":
        for ln in (x for x in lines("truth_part2.txt") if x.startswith("ROW|")):
            _r, i, sp = ln.split("|")
            i = int(i)
            if sp != subpartition:
                continue
            lvl = i if i <= 50 else i - 299              # 14_part2.sql: ids 1..50 ('v'), 300..349 ('w')
            rows[i] = [..., ("int", lvl % 7) if lvl % 7 else b"\x80", (b"v%d" if i <= 50 else b"w%d") % lvl]
        return (lambda r: num_int(r[0])), rows, f"truth_part2.txt ROW lines of {subpartition} (ids); K, V: sql (14_part2.sql)"
    if table == "T_LONGRAW":
        rows = {1: [..., bytes.fromhex("00ff")], 2: [..., b"\xab" * 32000]}
        return (lambda r: num_int(r[0])), rows, "sql (11_types.sql; no truth rows)"
    if table == "T_EMPTY":
        n = dict(x.split("|") for x in lines("truth_counts.txt") if x)["T_EMPTY"]
        assert n == "0"
        return None, {}, "0 rows (truth_counts.txt)"
    return None, None, "no byte-level truth handled"


def check(path: str, quiet: bool = True, items: str = "generic", empty: str = "text"):
    tdir = os.path.dirname(os.path.abspath(path))
    r = parse(path, items, empty)
    data = open(path, "rb").read()
    res = []
    for t in r["tables"]:
        keyf, exp, note = expected(t["name"], tdir, t.get("partition"), t.get("subpartition"))
        names = re.findall(r'"([^"]*)"', t["insert"].split(") VALUES (")[0].split(" (", 1)[1]) if "insert" in t else []
        path_name = "direct" if t["direct"] else "conventional"
        if exp is None:
            res.append({"name": t["name"], "path": path_name, "rows": len(t["rows"]), "truth_rows": None,
                        "ok": 0, "bad": 0, "missing": 0, "order": "", "note": note, "lines": [],
                        "types": {}, "lob": t["lob"]})
            continue
        ok = bad = 0
        types: dict = {}
        keys = []
        out_lines = []
        for pos, (ro, row) in enumerate(t["rows"]):
            vals = [v for _o, v in row]
            key = keyf(vals) if keyf else pos
            keys.append(key)
            want = exp.get(key)
            if isinstance(want, dict):               # session 9: expectation by column name
                want = [want[n] for n in names]
            if want is None:
                bad += 1
                out_lines.append(f"    row at 0x{ro:x}: key {key!r} not in truth")
                continue
            for ci, w in enumerate(want):
                if w is ...:
                    continue
                fo, got = row[ci]
                if isinstance(w, tuple) and w[0] == "prefix":     # ("prefix", total length, first bytes)
                    good = got is not None and len(got) == w[1] and got[:len(w[2])] == w[2] \
                        and got[len(w[2]):] == b"x" * (w[1] - len(w[2]))
                    wtxt = f"len {w[1]}, starts {w[2].hex(' ')}, rest all 'x'"
                elif isinstance(w, tuple) and w[0] == "fill":     # ("fill", length, first bytes, fill byte)
                    good = got is not None and len(got) == w[1] and got[:len(w[2])] == w[2] \
                        and got[len(w[2]):] == w[3] * (w[1] - len(w[2]))
                    wtxt = f"len {w[1]}, starts {w[2].hex(' ')}, rest all {w[3].hex()}"
                elif isinstance(w, tuple) and w[0] == "full":     # ("full", length, first bytes (truth), all bytes (sql))
                    good = got is not None and len(got) == w[1] and got[:len(w[2])] == w[2] and got == w[3]
                    wtxt = f"len {w[1]}, starts {w[2][:8].hex(' ')} .., all bytes by SQL"
                elif isinstance(w, tuple) and w[0] == "suffix":   # ("suffix", total length, last bytes)
                    good = got is not None and len(got) == w[1] and got.endswith(w[2])
                    wtxt = f"len {w[1]}, ends {w[2]!r}"
                elif isinstance(w, tuple) and w[0] == "int":
                    good = got is not None and num_int(got) == w[1]
                    wtxt = f"number {w[1]}"
                elif isinstance(w, tuple) and w[0] == "num":
                    good = got is not None and num_pos(got) == w[1]
                    wtxt = f"number {w[1]}"
                else:
                    good = got == w
                    wtxt = "NULL" if w is None else f"[{len(w)}] " + (w.hex(" ") if len(w) <= 24 else w[:10].hex(" ") + " ..")
                ok += good
                bad += not good
                tk = t["cols"][ci]["type"]
                if tk in (1, 96) and t["cols"][ci].get("csform") == 2:
                    tk = "1n" if tk == 1 else "96n"
                types.setdefault(tk, [0, 0])[0 if good else 1] += 1
                if not quiet or not good:
                    gtxt = "NULL" if got is None else f"[{len(got)}] " + (got.hex(" ") if len(got) <= 24 else got[:10].hex(" ") + " ..")
                    out_lines.append(f"    truth key {key!r} col {ci + 1} @0x{fo:04x} "
                                     f"len-field {data[fo:fo + 2].hex(' ')}  dump {gtxt}  "
                                     f"truth {wtxt}  {'OK' if good else 'DIFF'}")
        order = "same order as truth" if keys == sorted(exp.keys(), key=lambda k: (str(type(k)), k)) \
            else f"NOT truth order (first keys in dump: {keys[:4]})"
        missing = set(exp) - set(keys)
        if t["name"] == "R_CLOB":
            # LOB data chunks (hypothesis): length and first 60 bytes of truth_ru.txt
            want = {}
            for ln in open(os.path.join(tdir, "truth_ru.txt"), encoding="utf-8").read().split("\n"):
                f = ln.split("|")
                if f[0] == "R_CLOB":
                    want[int(f[1])] = None if f[2] == "NULL" else (int(f[2]), dump_bytes(f[3]))
            got_c = {}
            for ri, _co, cv in t["lob_chunks"]:
                got_c[keys[ri]] = got_c.get(keys[ri], b"") + cv
            for k_, w in want.items():
                g = got_c.get(k_)
                good = (g is None) if w is None else (g is not None and len(g) == w[0] and g[:len(w[1])] == w[1])
                ok += good
                bad += not good
                types.setdefault(112, [0, 0])[0 if good else 1] += 1
                if not quiet or not good:
                    out_lines.append(f"    R_CLOB id {k_}: LOB data {'NULL' if g is None else len(g)} bytes, truth "
                                     f"{'NULL' if w is None else w[0]}  {'OK' if good else 'DIFF'}")
        if t["name"] in ("T_NCHAR2", "T_LOB3"):
            # session 9: whole LOB values; CLOB (form 1, set 873) and NCLOB are UTF-16 BE (rule L5)
            vals = {(ri, ci): buf for ri, ci, _vo, buf, _f, _l in t["lob_values"]}
            want_l: dict = {}
            if t["name"] == "T_NCHAR2":
                for ln in (x for x in open(os.path.join(tdir, "truth_nchar2.txt"), encoding="utf-8").read().split("\n") if x):
                    f = ln.split("|")
                    cl = dump_bytes(f[7]).decode("utf-8").encode("utf-16-be")   # DUMP of SUBSTR(cl) is AL32UTF8
                    ncl = dump_bytes(f[8])                                      # DUMP of SUBSTR(ncl) is UTF-16 BE
                    assert len(cl) == 2 * int(f[5]) and len(ncl) == 2 * int(f[6])
                    want_l[int(f[0])] = {5: cl, 6: ncl}
            else:
                for ln in (x for x in open(os.path.join(tdir, "truth_lob3.txt"), encoding="utf-8").read().split("\n")
                           if x.startswith("T_LOB3|")):
                    f = ln.split("|")
                    i = int(f[1])
                    if i == 20:      # 14_lob3.sql: the same text in C and N; truth: GETLENGTH and the AL32UTF8 bytes
                        txt = dump_bytes(f[5]).decode("utf-8").encode("utf-16-be")
                        assert len(txt) == 2 * int(f[2]) == 2 * int(f[4]) and f[3] == "NULL"
                        want_l[i] = {1: txt, 2: None, 3: txt}
                    else:            # 14_lob3.sql: C = 'x' x S, B = 0x5A x S, N NULL; truth: S, S, NULL
                        assert f[4] == "NULL"
                        want_l[i] = {1: "x".encode("utf-16-be") * int(f[2]), 2: b"Z" * int(f[3]), 3: None}
            for ri, (_ro, row) in enumerate(t["rows"]):
                for ci, w in want_l.get(keys[ri], {}).items():
                    g = vals.get((ri, ci))
                    good = (w is None and g is None and row[ci][1] is None) if w is None else g == w
                    ok += good
                    bad += not good
                    c = t["cols"][ci]
                    types.setdefault(c["type"] if c.get("csform") != 2 else "112n", [0, 0])[0 if good else 1] += 1
                    if not quiet or not good:
                        out_lines.append(f"    {t['name']} id {keys[ri]} col {ci + 1}: LOB value "
                                         f"{'NULL' if g is None else len(g)} bytes  {'OK' if good else 'DIFF'}")
        if t["name"] == "R_LOB2":
            # session 10 (R-038): C = database bytes (CL8MSWIN1251), N = UTF-16 BE, B = bytes (rule L5);
            # full values by 15_ru_lob2.sql (blk = c0..ff a8 b8; row 2 = 303 x blk + c0 c1); GETLENGTH, head and
            # tail DUMPs by truth_lob_ru.txt
            blk = bytes(range(0xC0, 0x100)) + b"\xa8\xb8"
            full = {1: blk, 2: blk * 303 + blk[:2]}
            vals = {(ri, ci): buf for ri, ci, _vo, buf, _f, _l in t["lob_values"]}
            tr = {int(f[1]): f for f in (x.split("|") for x in open(os.path.join(tdir, "truth_lob_ru.txt"),
                                                                     encoding="utf-8").read().split("\n")
                                         if x.startswith("R_LOB2|"))}
            for ri, (_ro, row) in enumerate(t["rows"]):
                f = tr[keys[ri]]
                for ci in (1, 2, 3):
                    g = vals.get((ri, ci))
                    if f[1 + ci] == "NULL":
                        good = g is None and row[ci][1] is None
                    else:
                        n = int(f[1 + ci])
                        txt = full[keys[ri]]
                        w = txt.decode("cp1251").encode("utf-16-be") if ci == 2 else txt
                        good = g == w and len(txt) == n
                        if ci == 1:
                            good = good and g[:4] == dump_bytes(f[5]) and g[-4:] == dump_bytes(f[7])
                        if ci == 2:
                            good = good and g[:8] == dump_bytes(f[6]) and g[-8:] == dump_bytes(f[8])
                    ok += good
                    bad += not good
                    c = t["cols"][ci]
                    types.setdefault(c["type"] if c.get("csform") != 2 else "112n", [0, 0])[0 if good else 1] += 1
                    if not quiet or not good:
                        out_lines.append(f"    R_LOB2 id {keys[ri]} col {ci + 1}: LOB value "
                                         f"{'NULL' if g is None else len(g)} bytes  {'OK' if good else 'DIFF'}")
        if t["name"] in ("T_LOB", "T_LOB2"):
            from lob_check import sql_values
            sv = sql_values(t["name"])
            lobc = [i for i, c in enumerate(t["cols"]) if c["type"] in (112, 113)]
            vals = {(ri, ci): buf for ri, ci, _vo, buf, _f, _l in t["lob_values"]}
            for ri, (_ro, row) in enumerate(t["rows"]):
                for j, ci in enumerate(lobc):
                    want_v = sv[keys[ri]][j]
                    g = vals.get((ri, ci))
                    if row[ci][1] is None:
                        good = want_v is None and g is None
                    elif g is None:
                        good = False
                    else:
                        c = t["cols"][ci]
                        gv = g.decode("utf-16-be") if c["type"] == 112 else g
                        good = gv == want_v
                    ok += good
                    bad += not good
                    types.setdefault(t["cols"][ci]["type"] if t["cols"][ci].get("csform") != 2 else "112n",
                                     [0, 0])[0 if good else 1] += 1
                    if not quiet or not good:
                        out_lines.append(f"    {t['name']} id {keys[ri]} col {ci + 1}: LOB value "
                                         f"{'NULL' if g is None else len(g)} bytes  {'OK' if good else 'DIFF'}")
        res.append({"name": t["name"], "path": path_name, "rows": len(t["rows"]), "truth_rows": len(exp),
                    "ok": ok, "bad": bad, "missing": len(missing), "order": order, "note": note,
                    "lines": out_lines, "types": types, "lob": t["lob"]})
    return r, res


def main() -> None:
    path = sys.argv[1]
    quiet = "-q" in sys.argv
    r, res = check(path, quiet)
    print(f"{os.path.basename(path)}  problems={r['problems'] or 'none'}")
    for t in res:
        if t["truth_rows"] is None:
            print(f"  {t['name']}: {t['rows']} rows decoded; {t['note']}")
            continue
        for ln in t["lines"]:
            print(ln)
        print(f"  {t['name']} ({t['path']}): {t['rows']} rows, truth rows {t['truth_rows']}, "
              f"fields OK {t['ok']}, DIFF {t['bad']}, missing keys {t['missing']}; "
              f"{t['order']}; compared: {t['note']}")


if __name__ == "__main__":
    main()
