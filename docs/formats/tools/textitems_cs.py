#!/usr/bin/env python3
"""In which character set are the text items (comments, CHECK text, view text, identifiers)?

Checks (exp.md section 14.2, 14.3; Parts C6 and F4 of session 4): for every legacy EXP fixture
given, takes every item of the text region that contains a byte above 0x7e or a replacement
character next to a known non-ASCII object, and tests three statements:
  (1) the item decodes without error in the CLIENT character set (id at header 0x0001);
  (2) the length field of a field counts the bytes as written (always true by construction of the
      walk; printed so that the byte count can be compared with the truth's database byte count);
  (3) the comment texts equal the truth (`COLCOMMENT` / `TABCOMMENT` lines of truth_own_ddl.txt or
      truth_ru.txt, which hold the DATABASE bytes) after converting the truth from the database
      set to the client set, with the replacement character of the client set for characters it
      does not have (`?` for US7ASCII, 0xbf for WE8MSWIN1252, and `<` `>` for the guillemets with
      US7ASCII: all three observed in the fixtures, see exp.md 14.2). A truth line that shows only
      the first 40 bytes of a comment (truth_own_ddl.txt, the 300-byte comment) is compared by
      length and by these 40 bytes, and only when the comment is ASCII.
Prints one line per item with offset, form, length, the bytes in hex and the decoded text, and a
summary per file. Identifiers are covered because `TABLE "..."`, `CREATE TABLE`, `INSERT INTO`,
`CREATE INDEX`, `ANALSTATS` lines with non-ASCII names are items like any other.
Expects: fixtures of corpus/fixtures/11g-cp1251 (truth_ru.txt) or the own_exp files of 18c / 11g
(truth_own_ddl.txt):
    uv run textitems_cs.py ../../../corpus/fixtures/11g-cp1251/ru_exp_*.dmp [-q]
Reads the fixtures and truth files, never writes them. New in Analyst session 4.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

CODEC = {1: "ascii", 171: "cp1251", 178: "cp1252", 873: "utf-8"}
REPL = {1: b"?", 178: b"\xbf"}
# look-alike replacements seen in the fixtures (not a complete table): US7ASCII client, guillemets
BESTFIT = {1: {"\u00ab": b"<", "\u00bb": b">"}}


def to_client(dbbytes: bytes, dbcs: int, ccs: int) -> bytes:
    """Database bytes -> client bytes, unmappable characters -> replacement byte of the client set."""
    out = b""
    for ch in dbbytes.decode(CODEC[dbcs]):
        try:
            out += ch.encode(CODEC[ccs])
        except UnicodeEncodeError:
            out += BESTFIT.get(ccs, {}).get(ch, REPL[ccs])
    return out


def truth_comments(tdir: str):
    """-> list of database byte strings of all comments in the truth files of the directory."""
    res = []
    partial.clear()
    def hexlist(s):
        return bytes(int(x, 16) for x in s.split(": ", 1)[1].split(","))
    p = os.path.join(tdir, "truth_ru.txt")
    if os.path.exists(p):
        for ln in open(p, encoding="utf-8"):
            f = ln.rstrip("\n").split("|")
            if f[0] in ("TABCOMMENT", "COLCOMMENT"):
                res.append(hexlist(f[-1]))
    p = os.path.join(tdir, "truth_own_ddl.txt")
    if os.path.exists(p):
        txt = open(p, encoding="utf-8", newline="").read()
        for m in re.finditer(r"(?m)^(TABCOMMENT|COLCOMMENT)\|[^|]*\|(\d+)\|(Typ=1 Len=\d+ CharacterSet=\w+: [0-9a-f,]+)$", txt):
            b = hexlist(m.group(3))
            if len(b) == int(m.group(2)):          # the truth shows at most 40 bytes of a column comment
                res.append(b)
            elif b.isascii():
                partial.append((int(m.group(2)), b))
    return res


partial: list = []      # (database byte length, first bytes) of comments the truth shows in part
quiet = "-q" in sys.argv
for path in [a for a in sys.argv[1:] if not a.startswith("-")]:
    r = parse(path)
    h = r["header"]
    ccs, dbcs = h["cs_at_1"], h["cs_b"]
    comments = truth_comments(os.path.dirname(os.path.abspath(path)))
    want = {b"'" + to_client(c, dbcs, ccs).replace(b"'", b"''") + b"'" for c in comments}
    n_items = n_dec = n_cmt = n_cmt_ok = 0
    print(f"{os.path.basename(path)}: client set {ccs}, header 0x0032 (database set) {dbcs}")
    for it in r["items"]:
        raw = it["raw"]
        is_cmt = it["kind"] != "line" and it["intro"].startswith("COMMENT ON ")
        nonascii = any(b > 0x7e for b in raw) or (b"?" in raw and ccs == 1 and re.search(rb"\?{2,}", raw))
        if not nonascii and not is_cmt:
            continue
        n_items += 1
        try:
            txt = raw.decode(CODEC[ccs])
            n_dec += 1
            dec = "decodes in the client set"
        except UnicodeDecodeError:
            txt = raw.decode("latin-1")
            dec = "DOES NOT DECODE in the client set"
        extra = ""
        if is_cmt:
            n_cmt += 1
            ok = raw in want or any(len(raw) == n + 2 and raw[1:1 + len(b)] == b for n, b in partial)
            n_cmt_ok += ok
            extra = "; comment = truth converted to the client set: " + ("yes" if ok else "NO (or truth shows only a part)")
        if not quiet:
            form = it["kind"] if it["kind"] != "line" else "line"
            print(f"  0x{it['off']:05x} {form:7s} len {len(raw):3d} {dec}{extra}\n"
                  f"          {raw[:90].hex(' ')}{' ..' if len(raw) > 90 else ''}\n          {txt[:110]!r}")
    print(f"  items with non-ASCII or replaced text, or comments: {n_items}; decode in the client set: {n_dec}; "
          f"comments: {n_cmt}, equal to the converted truth: {n_cmt_ok}")
