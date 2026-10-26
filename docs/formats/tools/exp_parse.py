#!/usr/bin/env python3
"""Walk any legacy EXP fixture from byte 0 to the end and print its structure.

Checks: that the layout written in docs/formats/exp.md (header, two 102-byte blocks, string list,
text items, column descriptors, rows, `ff ff`, trailer text, zero fill) consumes the whole file
without a gap, for conventional and direct-path files, table mode and owner mode, one or several
tables, any RECORDLENGTH, Oracle 18c and 11g. Prints every header field, every item of the text
region with its kind, every table with its descriptors, and (with --rows) every row with the
file offset of each field.
Expects: one `*_exp_*.dmp` fixture of corpus/fixtures/18c, 11g or 11g-cp1251, e.g.
    uv run exp_parse.py ../../../corpus/fixtures/18c/own_exp_18c.dmp --rows
Options: --rows (print rows), --items=generic|keyword (item rule, default generic),
         --empty=text|declared (rule for "direct path, no rows", default text).
Also importable: `parse(path, items="generic", empty="text")` returns a dict used by the other
tools in this directory.
Exploration tool only, not the product parser. Reads the fixture, never writes it.

Changed in Analyst session 19 (R-048):
- Conventional path (flag 0), type 8 or 24, u16 `fa ff`: a 4-byte length follows (`fa ff 00 00` = 65,530,
  `fc ff 00 00` = 65,532, `fd ff 00 00` = 65,533 in long4_exp_<v>.dmp rows 3 to 5; read as u32 LE, the upper
  half `00 00` in every case, so "u32" is a hypothesis), then the value bytes; `t["long_fa"]` lists
  (row index, offset, length). Lengths 0xfff1 and 0xfff5 are plain one-field lengths (rows 1, 2); 65,534 and
  65,535 use the `fd ff` chunk form (rows 6, 7).

Changed in Analyst session 17 (R-040):
- Conventional path (flag 0), type 8 (LONG) or 24 (LONG RAW), u16 `fd ff`: the value follows as chunks
  `u16 n + n bytes` ended by a chunk `00 00`; `t["long_chunks"]` lists (row index, offset, chunk u16 values).
  Seen in long3_exp_<v>.dmp rows 5 to 7 (65,535, 65,536, 100,000 bytes; chunks of 4000). Every other
  conventional u16 below 0xfffe is read as a length (0xfff0 = 65,520 is one field in long3_exp_<v>.dmp row 4).

Changed in Analyst session 9 (R-031):
- Direct path (flag 1), type 8 (LONG) or 24 (LONG RAW): the field is a chain of pieces `u16 n + bytes`;
  if bit 0x8000 of n is set, n & 0x7fff bytes follow and another piece follows; a piece with the bit
  clear is the last one (an ordinary short value is one such piece). `t["long_pieces"]` lists
  (row index, offset, piece u16 values). Seen in long_exp_direct_<v>.dmp (156 + 4 x 7961 bytes).
- Composite partitions (R-037): lines `COMPARTITION "p"` and `SUBPARTITION "s"` inside a table section name
  the partition and subpartition of the next binary part (`t["partition"]`, `t["subpartition"]`);
  `ENDCOMPARTITION` and `ENDPARTITION` are ordinary lines.

Changed in Analyst session 8 (R-027, R-029, R-030):
- LOB tables (exp.md 14.6, rewritten): type codes 112 (CLOB, NCLOB; 8-byte descriptor) and 113
  (BLOB; 4-byte descriptor). The u16 before the path flag is m (0, or the column count when a LOB
  column exists); after the path flag m column entries `u8 n, n name bytes, 2 bytes` follow (n = 0
  for a non-LOB column). A LOB field in the row is a locator (`u16 n + n bytes`, or `fe ff`). After
  the last field of a row the u16 k (the `00 00` of rule R4 when k = 0) counts the LOB values that
  follow, one per non-NULL LOB column in column order; a value is `u16 n + n bytes`, or `fc ff` +
  8-byte LE total length + chunks `u16 n + n bytes` up to the total. `t["lob_values"]` holds
  (row index, column index, offset, bytes, form, chunk lengths); `t["lob_chunks"]` keeps the old
  shape (row index, offset, whole value) for older tools.
- Partitioned tables: a line `PARTITION "p"` inside a table section is followed by an `INSERT INTO`
  line and its own binary part; each binary part becomes its own entry of `tables` with the key
  `partition` (None for an unpartitioned table). `ENDPARTITION` is an ordinary line.

Changed in Analyst session 4:
- Owner mode (`RUSERS` in the header): the first item is `METRICSU`, not `METRICST`; the tables
  stand between `METRICSU...` / `METRICEU n` pairs and end with one `ENDTABLE` line. The walker no
  longer has a "trailer" state: a table section is a `TABLE "name"` line that is directly
  followed by a `CREATE TABLE` line, wherever it stands; a `TABLE` line followed by anything else
  is a statement group (exp.md 4.5, 7.1). Statements after the rows belong to the table until the
  next `TABLE "` line, an `ENDTABLE` line or a `METRICS...` line.
- A third field form (exp.md 4.2): pieces WITHOUT the `fd ff` marker, ended by a piece of length
  0, used for the text of a view (`CREATE VIEW ... AS `). Rule `generic` recognises it by "the
  byte after a plain field is not LF"; rule `keyword` by the introducing line.
- Rule `keyword`: the list of introducing lines of session 3 was incomplete. Added:
  `PROCACTION`, `CREATE FORMAT71 ...` (procedures), `CREATE FORMAT92TRIG ` (triggers),
  `CREATE VIEW ... AS `. The form of the field (plain / `fd ff` pieces / unmarked pieces) is
  taken from the keyword, and a mismatch with the bytes is reported as a problem.
  `--items=keyword3` keeps the session 3 list, to show where it fails.
  `--items=generic3` keeps the session 3 generic rule (no unmarked pieces), for the same reason.
- Every item gets a `kind` label (CREATE TABLE, INDEX, CONSTRAINT, COMMENT, ...) and the name of
  the `METRICS?` section and `TABLE` group it stands in (`kind_of`, `zone`).
- Type code 112 (CLOB, outside phase 1): the descriptor is read with 8 bytes, the 12 bytes
  between descriptors and rows and the rows are read with the HYPOTHESIS of exp.md 14.6 so that
  the walk can continue behind the table; the table is marked `lob=True`.
- Text is kept as bytes (`raw`) next to a latin-1 rendering, because text items can be in any
  client character set (exp.md 14).

Changed in Analyst session 5:
- Rule `keyword`: the field after an `ANALSTATS xx ...` line is no longer expected to be plain.
  It is plain for table and index statistics and a field in pieces after `fd ff` for column
  statistics (exp.md 4.6), so the keyword entry now has the form `plain_or_pieces`: the bytes
  decide (`fd ff` or not), and neither is reported. Before this change the rule printed "keyword
  rule expected form plain" for every column-statistics field (41 or 31 per R-016 file), which
  check_all.py showed as RULES DISAGREE.
- New labels for the lines `ANALSTATS T?` (table or column statistics), `ANALSTATS I?` (index),
  `ANALSTATS C?` (index of a constraint, in the trailer) and `ANALCOMPUTE`; the old label
  `ANALSTATS (intro)` is kept as a prefix so that older tools still match.
- The header records the user line and the mode line (`UCORPUS`, `RTABLES` / `RUSERS`).

Changed in Analyst session 3:
- The text region is a sequence of ITEMS (exp.md section 4.2): a text line ending in LF, or a
  FIELD followed by LF. Two ways to tell a field from a line are implemented:
    generic: an item is a field when its second byte is below 0x20 or its first two bytes are
             `fd ff`; otherwise it is a text line;
    keyword: an item is a field when the previous line is one of the introducing lines.
- The walk ends after the two `EXIT` lines instead of at the first zero byte.
- "Direct path, no rows" (no `ff ff`): rule `text` (the byte where the first row would start is
  LF and the byte after it is an upper-case ASCII letter) or rule `declared` (the session 2
  heuristic; exp.md section 5.4).

Rules used (each one is a row in exp.md with its evidence and status):
- descriptor = u16 type, u16 length; types 1 (VARCHAR2/NVARCHAR2), 96 (CHAR) and 112 (CLOB) have
  4 more bytes (u16 character set id, u16 form);
- field = u16 LE length + bytes; 0xFFFE = NULL; 0xFFFF where a row would start = end of rows;
- conventional path: `00 00` after every row; direct path: nothing between rows.
"""
import re
import sys

CHAR_TYPES = (1, 96, 112)
LOB_TYPES = (112, 113)
LONG_TYPES = (8, 24)     # session 9: pieces on the direct path

# rule K, session 3 list (kept to show that it is incomplete)
INTRO3 = re.compile(r'^(ALTER TABLE .* MODIFY DEFAULT|ANALSTATS .*|CALTER TABLE|COMMENT ON .* IS )$', re.S)
# rule K, session 4 list: introducing line -> form of the field that follows
INTRO4 = [
    (re.compile(r'^ALTER TABLE .* MODIFY DEFAULT$', re.S), "plain"),
    (re.compile(r'^ANALSTATS [A-Z]{2} .*$', re.S), "plain_or_pieces"),   # session 5: T? columns are `fd ff` pieces
    (re.compile(r'^COMMENT ON .* IS $', re.S), "plain"),
    (re.compile(r'^CALTER TABLE$'), "pieces"),
    (re.compile(r'^PROCACTION$'), "pieces"),
    (re.compile(r'^CREATE FORMAT\d+\S* .*$|^CREATE FORMAT\d+\S* $', re.S), "pieces"),
    (re.compile(r'^CREATE .*VIEW .* AS $', re.S), "upieces"),
]

KINDS = [
    (r'^METRICS[TU]$', "METRICS (start)"),
    (r'^METRICS[TU].', "METRICS section start"),
    (r'^METRICE[TU]', "METRICE"),
    (r'^TABLE "', "TABLE"),
    (r'^ENDTABLE$', "ENDTABLE"),
    (r'^CREATE TABLE ', "CREATE TABLE"),
    (r'^ALTER TABLE .* MODIFY DEFAULT$', "ALTER TABLE MODIFY DEFAULT (intro)"),
    (r'^INSERT INTO ', "INSERT INTO"),
    (r'^CREATE (UNIQUE )?INDEX ', "CREATE INDEX"),
    (r'^ANALSTATS T[A-Z] ', "ANALSTATS (intro) table/column"),
    (r'^ANALSTATS I[A-Z] ', "ANALSTATS (intro) index"),
    (r'^ANALSTATS C[A-Z] ', "ANALSTATS (intro) constraint index"),
    (r'^ANALSTATS ', "ANALSTATS (intro)"),
    (r'^ANALCOMPUTE ', "ANALCOMPUTE"),
    (r'^CALTER TABLE$', "CALTER TABLE (intro)"),
    (r'^ALTER TABLE .* ADD .*PRIMARY KEY', "constraint PRIMARY KEY"),
    (r'^ALTER TABLE .* ADD .*FOREIGN KEY', "constraint FOREIGN KEY"),
    (r'^ALTER TABLE .* ENABLE CONSTRAINT', "ENABLE CONSTRAINT"),
    (r'^COMMENT ON ', "COMMENT (intro)"),
    (r'^GRANT ', "GRANT"),
    (r'^CREATE SEQUENCE ', "CREATE SEQUENCE"),
    (r'^CREATE SYNONYM ', "CREATE SYNONYM"),
    (r'^CREATE .*VIEW ', "CREATE VIEW (intro)"),
    (r'^CREATE FORMAT\d+ PROCEDURE', "procedure (intro)"),
    (r'^CREATE FORMAT\d+TRIG', "trigger (intro)"),
    (r'^ALTER TRIGGER ', "ALTER TRIGGER"),
    (r'^ALTER PROCEDURE ', "ALTER PROCEDURE"),
    (r'^ALTER SESSION ', "ALTER SESSION"),
    (r'^PROCACTION$', "PROCACTION (intro)"),
    (r'^EXIT$', "EXIT"),
]
KINDS = [(re.compile(a, re.S), b) for a, b in KINDS]


def kind_of(text: str) -> str:
    for rx, name in KINDS:
        if rx.match(text):
            return name
    return "OTHER LINE"


def u16(d: bytes, p: int) -> int:
    return d[p] | (d[p + 1] << 8)


def read_pieces(d: bytes, q: int):
    """Pieces `u16 LE n + n bytes` from q up to a piece of length 0 -> (payload, pieces, next)."""
    pieces, buf = [], b""
    while True:
        n = u16(d, q)
        pieces.append((q, n))
        q += 2
        if n == 0:
            break
        buf += d[q:q + n]
        q += n
    return buf, pieces, q


def read_field(d: bytes, p: int, form: str):
    """Field at p -> (kind, payload bytes, list of (offset, length) pieces, next offset, note).

    form: "auto" (rule generic), "auto3" (rule generic of session 3), or the form rule keyword
    expects: "plain", "pieces" (after `fd ff`), "upieces" (pieces without marker),
    "plain_or_pieces" (session 5: plain, or pieces after `fd ff`; the bytes decide)."""
    note = None
    marked = d[p:p + 2] == b"\xfd\xff"
    if form in ("pieces", "plain", "upieces") and marked != (form == "pieces"):
        note = f"keyword rule expected form {form} at 0x{p:x}, bytes start {d[p:p + 2].hex(' ')}"
    if marked:
        buf, pieces, q = read_pieces(d, p + 2)
        return "pieces", buf, pieces, q, note
    if form == "upieces":
        buf, pieces, q = read_pieces(d, p)
        return "upieces", buf, pieces, q, note
    v = u16(d, p)
    q = p + 2 + v
    if form == "auto" and d[q] != 0x0A:
        buf, pieces, q2 = read_pieces(d, p)
        if d[q2] == 0x0A:
            return "upieces", buf, pieces, q2, note
    return "plain", d[p + 2:q], [(p, v)], q, note


def parse(path: str, items: str = "generic", empty: str = "text") -> dict:
    d = open(path, "rb").read()
    out: dict = {"path": path, "size": len(d), "tables": [], "lines": [], "items": [],
                 "problems": [], "table_lines": [], "sections": []}
    # ---- header -------------------------------------------------------------------------
    h: dict = {"b0": d[0], "cs_at_1": (d[1] << 8) | d[2]}
    p = 3
    h["direct_marker"] = d[p:p + 2] == b"D\n"
    if h["direct_marker"]:
        p += 2
    texts = []
    for _ in range(7):                      # EXPORT:V.., U.., R.., reclen, 0, 72, 0
        e = d.index(b"\n", p)
        texts.append((p, d[p:e].decode("latin-1")))
        p = e + 1
    h["texts"] = texts
    h["version"], h["user"], h["mode"] = texts[0][1], texts[1][1], texts[2][1]
    h["bin_off"] = p
    h["cs_a"] = (d[p] << 8) | d[p + 1]
    h["cs_b"] = (d[p + 2] << 8) | d[p + 3]
    h["cs_c"] = (d[p + 4] << 8) | d[p + 5]
    h["bin_rest"] = d[p + 6:p + 16]
    h["fnlen_off"] = p + 16
    h["fnlen"] = (d[p + 16] << 8) | d[p + 17]
    h["after_fnlen"] = d[p + 18]
    sp = p + 19
    q = sp
    while d[q] == 0x20:
        q += 1
    h["spaces"] = (sp, q - sp)
    h["date"] = (q, d[q:0x80].decode("latin-1"))
    h["fname"] = (0x80, d[0x80:0x80 + h["fnlen"]].decode("latin-1"))
    reclen = int(texts[3][1])
    h["reclen"] = reclen
    hdr_end = 0x80 + h["fnlen"]
    if any(d[hdr_end:reclen]):
        out["problems"].append(f"non-zero bytes between 0x{hdr_end:x} and reclen")
    if len(d) % reclen:
        out["problems"].append(f"file size 0x{len(d):x} is not a multiple of reclen {reclen}")
    out["header"] = h
    # ---- second record: two blocks + string list ------------------------------------------
    p = reclen
    out["block1"] = (p, d[p:p + 102])
    out["block2"] = (p + 102, d[p + 102:p + 204])
    p += 204
    strs = []
    for _ in range(8):
        n = u16(d, p)
        strs.append((p, d[p + 2:p + 2 + n].decode("latin-1")))
        p += 2 + n
    out["strings"] = strs
    if d[p] != 0x0A:
        out["problems"].append(f"no LF after string list at 0x{p:x}")
    p += 1
    out["first_item_off"] = p
    # ---- items and table data -------------------------------------------------------------
    cur = None            # table section being filled: pre_items until INSERT, post_items after
    pending_table = None  # a TABLE line whose next item is not yet seen
    prev_line = ""
    exits = 0
    zone = None           # text of the last METRICS? section line
    group = None          # name of the open TABLE group
    while p < len(d) and exits < 2:
        form = None
        if items == "generic":
            if d[p + 1] < 0x20 or d[p:p + 2] == b"\xfd\xff":
                form = "auto"
        elif items == "generic3":
            if d[p + 1] < 0x20 or d[p:p + 2] == b"\xfd\xff":
                form = "auto3"
        elif items == "keyword3":
            if INTRO3.match(prev_line):
                form = "auto3"
        else:
            for rx, f in INTRO4:
                if rx.match(prev_line):
                    form = f
                    break
        if form is not None:
            kind, payload, pieces, q, note = read_field(d, p, form)
            if note:
                out["problems"].append(note)
            it = {"off": p, "kind": kind, "text": payload.decode("latin-1"), "raw": payload,
                  "pieces": pieces, "end": q, "intro": prev_line}
            if q >= len(d) or d[q] != 0x0A:
                out["problems"].append(f"no LF after field item at 0x{q:x} (item at 0x{p:x})")
            p = q + 1
            prev_line = ""
        else:
            e = d.find(b"\n", p)
            if e < 0:
                out["problems"].append(f"no LF after 0x{p:x}")
                break
            raw = d[p:e]
            line = raw.decode("latin-1")
            it = {"off": p, "kind": "line", "text": line, "raw": raw, "end": e}
            out["lines"].append((p, line))
            p = e + 1
            prev_line = line
            exits = exits + 1 if line == "EXIT" else 0
        line = it["text"] if it["kind"] == "line" else None
        it["label"] = kind_of(line) if line is not None else "field of: " + kind_of(it["intro"])
        # zone / group bookkeeping
        if line is not None and (re.match(r"^METRICS[TU]", line) or line == "ENDTABLE"
                                 or re.match(r'^TABLE "', line)):
            if cur is not None:
                cur["sec_end"] = it["off"]      # first byte after the table section
            cur = None
        if line is not None and re.match(r"^METRICS[TU]", line):
            zone = line[8:]
        it["zone"], it["group"] = zone, group
        out["items"].append(it)
        if pending_table is not None:
            off, name = pending_table
            pending_table = None
            if line is not None and line.startswith("CREATE TABLE "):
                cur = {"table_off": off, "name": name, "pre_items": [], "post_items": [],
                       "has_insert": False, "zone": zone}
                out["sections"].append(cur)
        if line is not None and line == "ENDTABLE":
            group = None
        if line is not None and re.match(r'^TABLE "', line):
            out["table_lines"].append((it["off"], line))
            pending_table = (it["off"], line[7:-1])
            group = line[7:-1]
            it["group"] = group
            cur = None
            continue
        if cur is None:
            if line is not None and line.startswith("INSERT INTO "):
                out["problems"].append(f"INSERT INTO outside a table section at 0x{it['off']:x}")
            continue
        if line is None or not line.startswith("INSERT INTO "):
            cur["post_items" if cur["has_insert"] else "pre_items"].append(it)
            if line is not None and line.startswith('PARTITION "'):
                cur["cur_partition"] = line[11:-1]
                cur["cur_subpartition"] = None
            if line is not None and line.startswith('COMPARTITION "'):      # session 9: composite partition
                cur["cur_partition"] = line[14:-1]
                cur["cur_subpartition"] = None
            if line is not None and line.startswith('SUBPARTITION "'):      # session 9: its subpartitions
                cur["cur_subpartition"] = line[14:-1]
            continue
        if cur["has_insert"]:
            # second or later binary part of the same section (partitioned table, session 8)
            t = {"table_off": cur["table_off"], "name": cur["name"], "pre_items": [], "post_items": [],
                 "zone": cur["zone"], "section": cur}
        else:
            t = cur
        t["partition"] = cur.get("cur_partition")
        t["subpartition"] = cur.get("cur_subpartition")
        t.update({"insert_off": it["off"], "insert": line, "insert_raw": it["raw"], "bin_off": p,
                  "has_insert": True})
        cur["has_insert"] = True
        ncols = u16(d, p)
        p += 2
        cols = []
        for _ in range(ncols):
            typ, ln = u16(d, p), u16(d, p + 2)
            c = {"off": p, "type": typ, "len": ln}
            p += 4
            if typ in CHAR_TYPES:
                c["csid"], c["csform"] = u16(d, p), u16(d, p + 2)
                p += 4
            cols.append(c)
        t["cols"] = cols
        t["lob"] = any(c["type"] in LOB_TYPES for c in cols)
        t["post_desc"] = (p, u16(d, p), u16(d, p + 2))
        direct = u16(d, p + 2) == 1
        t["direct"] = direct
        m = u16(d, p)
        if not t["lob"] and m != 0:
            out["problems"].append(f"u16 before the path flag is not 0 at 0x{p:x}")
        if t["lob"] and m != ncols:
            out["problems"].append(f"LOB table: u16 before the path flag is {m}, not the column count {ncols} at 0x{p:x}")
        if u16(d, p + 2) not in (0, 1):
            out["problems"].append(f"path flag is neither 0 nor 1 at 0x{p + 2:x}")
        p += 4
        t["lob_extra"] = (p, b"")
        t["lob_names"] = []
        if m:
            # session 8: m column entries `u8 n, n name bytes, 2 bytes` (n = 0 for a non-LOB column)
            q0 = p
            for ci in range(m):
                n = d[p]
                name = d[p + 1:p + 1 + n]
                tail = d[p + 1 + n:p + 3 + n]
                t["lob_names"].append(name)
                if tail != b"\x00\x00":
                    out["problems"].append(f"LOB entry of column {ci + 1} at 0x{p:x}: last 2 bytes {tail.hex(' ')}")
                if ci < ncols and (n > 0) != (cols[ci]["type"] in LOB_TYPES):
                    out["problems"].append(f"LOB entry of column {ci + 1} at 0x{p:x}: name length {n} "
                                           f"but type {cols[ci]['type']}")
                p += 3 + n
            t["lob_extra"] = (q0, d[q0:p])
        t["rows_off"] = p
        rows = []
        # both "no rows on the direct path" rules are evaluated, so that they can be compared
        v0 = u16(d, p)
        t["empty_rule_declared"] = direct and v0 not in (0xFFFE, 0xFFFF) and v0 > cols[0]["len"]
        t["empty_rule_text"] = direct and d[p] == 0x0A and 0x41 <= d[p + 1] <= 0x5A
        no_rows = t["empty_rule_text"] if empty == "text" else t["empty_rule_declared"]
        t["lob_chunks"] = []
        t["lob_values"] = []
        t["end_marker"] = None
        while True:
            v = u16(d, p)
            if direct and not rows and no_rows:
                t["end_marker"] = None          # direct path, zero rows: no ff ff
                break
            if v == 0xFFFF:
                t["end_marker"] = p
                p += 2
                break
            row = []
            rstart = p
            for _c in cols:
                v = u16(d, p)
                if direct and _c["type"] in LONG_TYPES and v not in (0xFFFE, 0xFFFF):
                    # session 9: on the direct path a LONG / LONG RAW value is a chain of pieces
                    # `u16 n + bytes`; bit 0x8000 of n set = another piece follows (n & 0x7fff bytes)
                    fo, buf, plens = p, b"", []
                    while True:
                        n = u16(d, p)
                        plens.append(n)
                        buf += d[p + 2:p + 2 + (n & 0x7FFF)]
                        p += 2 + (n & 0x7FFF)
                        if not n & 0x8000:
                            break
                    row.append((fo, buf))
                    t.setdefault("long_pieces", []).append((len(rows), fo, plens))
                    continue
                if not direct and _c["type"] in LONG_TYPES and v == 0xFFFD:
                    # session 17 (R-040): conventional LONG of 65,535 bytes or more: `fd ff`, then chunks
                    # `u16 n + n bytes` (4000 seen), ended by a chunk `00 00`
                    fo, buf, clens = p, b"", []
                    p += 2
                    while True:
                        n = u16(d, p)
                        clens.append(n)
                        p += 2 + n
                        if n == 0:
                            break
                        buf += d[p - n:p]
                    row.append((fo, buf))
                    t.setdefault("long_chunks", []).append((len(rows), fo, clens))
                    continue
                if not direct and _c["type"] in LONG_TYPES and v == 0xFFFA:
                    # session 19 (R-048): conventional LONG of 65,530 to 65,533 bytes: `fa ff`, then 4 bytes
                    # `n 00 00` (u16 n = the length, then `00 00`; read as u32 LE, hypothesis), then n bytes
                    fo = p
                    n = u16(d, p + 2)
                    if u16(d, p + 4) != 0:
                        out["problems"].append(f"fa ff form at 0x{p:x}: bytes 4-5 are not 00 00")
                    n |= u16(d, p + 4) << 16
                    row.append((fo, d[p + 6:p + 6 + n]))
                    t.setdefault("long_fa", []).append((len(rows), fo, n))
                    p += 6 + n
                    continue
                if v == 0xFFFE:
                    row.append((p, None))
                    p += 2
                elif v == 0xFFFF:
                    out["problems"].append(f"ff ff inside a row at 0x{p:x}")
                    p += 2
                    row.append((p, None))
                else:
                    row.append((p, d[p + 2:p + 2 + v]))
                    p += 2 + v
            if not direct:
                k = u16(d, p)
                if k != 0 and not t["lob"]:
                    out["problems"].append(f"expected 00 00 after row at 0x{p:x}")
                p += 2
                if t["lob"]:
                    # session 8: k LOB values, one per non-NULL LOB column, in column order
                    lob_cols = [ci for ci, c in enumerate(cols) if c["type"] in LOB_TYPES and row[ci][1] is not None]
                    if k != len(lob_cols):
                        out["problems"].append(f"LOB row at 0x{rstart:x}: k = {k}, non-NULL LOB columns {len(lob_cols)}")
                    for vi in range(k):
                        ci = lob_cols[vi] if vi < len(lob_cols) else None
                        vo = p
                        if u16(d, p) == 0xFFFC:
                            total = int.from_bytes(d[p + 2:p + 10], "little")
                            p += 10
                            buf, lens = b"", []
                            while len(buf) < total:
                                n = u16(d, p)
                                if n == 0 or n >= 0xFFF0:
                                    out["problems"].append(f"LOB chunk length {n:#x} at 0x{p:x}")
                                    break
                                lens.append(n)
                                buf += d[p + 2:p + 2 + n]
                                p += 2 + n
                            if len(buf) != total:
                                out["problems"].append(f"LOB value at 0x{vo:x}: chunks give {len(buf)}, total {total}")
                            form = "long"
                        else:
                            n = u16(d, p)
                            buf, lens = d[p + 2:p + 2 + n], [n]
                            p += 2 + n
                            form = "short"
                        t["lob_values"].append((len(rows), ci, vo, buf, form, lens))
                        t["lob_chunks"].append((len(rows), vo, buf))
            rows.append((rstart, row))
            if p >= len(d) - 2:
                out["problems"].append(f"rows of {t['name']} run to the end of the file")
                break
        t["rows"] = rows
        t["rows_end"] = p
        if p >= len(d) or d[p] != 0x0A:
            out["problems"].append(f"no LF after rows of {t['name']} at 0x{p:x}")
        p += 1
        prev_line = ""
        out["tables"].append(t)
    out["data_end"] = p
    if exits < 2:
        out["problems"].append("the walk did not end with two EXIT lines")
    if any(d[p:]):
        out["problems"].append(f"non-zero bytes after 0x{p:x}")
    return out


def show_item(it: dict) -> str:
    txt = it["text"]
    short = repr(txt[:100]) + (" ..." if len(txt) > 100 else "")
    if it["kind"] == "line":
        return f"0x{it['off']:04x} line  [{len(txt)}+LF] {short}"
    if it["kind"] == "plain":
        return f"0x{it['off']:04x} field u16={len(txt)} + LF@0x{it['end']:04x} {short}"
    pcs = it["pieces"]
    if len(pcs) > 14:
        ptxt = " ".join(f"@{o:x}:{n}" for o, n in pcs[:6]) + f" ... ({len(pcs)} pieces) ... " \
            + " ".join(f"@{o:x}:{n}" for o, n in pcs[-3:])
    else:
        ptxt = " ".join(f"@{o:x}:{n}" for o, n in pcs)
    mark = "fd ff, " if it["kind"] == "pieces" else "no marker, "
    return f"0x{it['off']:04x} field {mark}pieces {ptxt}, total {len(txt)}, LF@0x{it['end']:04x} {short}"


def main() -> None:
    path = sys.argv[1]
    show_rows = "--rows" in sys.argv
    opt = dict(a[2:].split("=") for a in sys.argv[2:] if "=" in a)
    r = parse(path, opt.get("items", "generic"), opt.get("empty", "text"))
    h = r["header"]
    print(f"{path}: {r['size']} bytes, reclen {h['reclen']}")
    print(f"0x0000 b0={h['b0']:02x} cs_at_1={h['cs_at_1']} direct_marker={h['direct_marker']}")
    for off, t in h["texts"]:
        print(f"0x{off:04x} text {t!r}")
    b = h["bin_off"]
    print(f"0x{b:04x} cs_a={h['cs_a']} cs_b={h['cs_b']} cs_c={h['cs_c']} rest={h['bin_rest'].hex(' ')}")
    print(f"0x{h['fnlen_off']:04x} fnlen={h['fnlen']} next={h['after_fnlen']:02x}")
    print(f"0x{h['spaces'][0]:04x} {h['spaces'][1]} spaces")
    print(f"0x{h['date'][0]:04x} date {h['date'][1]!r}")
    print(f"0x{h['fname'][0]:04x} file name {h['fname'][1]!r}")
    for name in ("block1", "block2"):
        off, blk = r[name]
        print(f"0x{off:04x} {name} {blk[:65].hex(' ')} + {sum(1 for x in blk[65:] if x == 0)} zero bytes")
    for off, s in r["strings"]:
        print(f"0x{off:04x} string {s!r}")
    by_insert = {t["insert_off"]: t for t in r["tables"]}
    for it in r["items"]:
        print(show_item(it) + f"   <{it['label']}>")
        t = by_insert.get(it["off"])
        if t is None:
            continue
        print(f"  0x{t['bin_off']:04x} ncols={len(t['cols'])}")
        for c in t["cols"]:
            extra = f" csid={c['csid']} form={c['csform']}" if "csid" in c else ""
            print(f"  0x{c['off']:04x} col type={c['type']} len={c['len']}{extra}")
        o, a, f = t["post_desc"]
        print(f"  0x{o:04x} post-descriptor u16s {a}, {f}  (direct={t['direct']})")
        if t.get("partition"):
            print(f"  partition {t['partition']}" + (f" subpartition {t['subpartition']}" if t.get("subpartition") else ""))
        if t["lob"]:
            print(f"  0x{t['lob_extra'][0]:04x} LOB table, column entries {t['lob_extra'][1].hex(' ')} "
                  f"names {[x.decode('latin-1') for x in t['lob_names']]}")
        print(f"  0x{t['rows_off']:04x} rows: {len(t['rows'])}; end marker ff ff at "
              f"{'(absent)' if t['end_marker'] is None else hex(t['end_marker'])}; "
              f"LF at 0x{t['rows_end']:04x}")
        if show_rows:
            for i, (ro, row) in enumerate(t["rows"], 1):
                fs = []
                for fo, v in row:
                    if v is None:
                        fs.append(f"@{fo:x} NULL")
                    else:
                        hx = v.hex(" ") if len(v) <= 24 else v[:12].hex(" ") + f" ..({len(v)})"
                        fs.append(f"@{fo:x} [{len(v)}] {hx}")
                print(f"    row {i:3d} 0x{ro:04x}: " + " | ".join(fs))
            for ri, ci, vo, cv, form, lens in t["lob_values"]:
                lt = lens if len(lens) <= 4 else lens[:2] + ["..."] + lens[-1:]
                print(f"    LOB value of row {ri + 1} col {None if ci is None else ci + 1} @0x{vo:x} {form} "
                      f"[{len(cv)}] chunks {len(lens)} {lt} {cv[:12].hex(' ')}")
    print(f"0x{r['data_end']:04x} zero fill to end of file (0x{r['size']:x})")
    print("problems:", r["problems"] or "none")


if __name__ == "__main__":
    main()
