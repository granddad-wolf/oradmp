#!/usr/bin/env python3
"""Which values of the u16 length field occur in the row data of the legacy EXP fixtures?

Checks (exp_reader_rules.md rule R2, note A-4 of the Architect, Analyst session 7): walks every
`*_exp_*.dmp` file with exp_parse.py (generic item rule) and, for every field of every row of
every table section, takes the two bytes of its length field (u16 LE). It counts
  - NULL markers `fe ff`,
  - the length 0 (`00 00` as a field length, not the `00 00` that ends a conventional-path row),
  - the marker `fd ff` and every other value from 0xfff0 up (other than fe ff / ff ff),
  - the smallest and the largest length per column type code,
and lists every file and offset where a length 0 or a value from 0xfff0 up (other than fe ff) is
found. The table sections of files that exp_parse.py reads as LOB tables (type 112, hypothesis of
exp.md 14.6) are counted separately, because their rows are followed by chunks the rules do not
cover.
Expects: fixture directories (default: the three folders of corpus/fixtures):
    uv run rowlen_stats.py ../../../corpus/fixtures/18c ../../../corpus/fixtures/11g \
                           ../../../corpus/fixtures/11g-cp1251
Reads the fixtures, never writes them. New in Analyst session 7.
Changed in Analyst session 8: the column fields of LOB tables (locators included) are now counted with
all others (the session 8 LOB rules of exp_parse.py read them), and the LOB values after the rows are
counted apart (`lob_values`): their form (short `u16 n` / long `fc ff`), the number with length 0
(an empty LOB), the chunk lengths, and the u16 k after each row of a LOB table.
Changed in Analyst session 9: a LONG / LONG RAW field on the direct path is a chain of pieces (exp_parse.py
`long_pieces`); it is counted with its value length, and its piece u16 values are listed apart
(`long_chains`). Every other column field whose u16 is from 0x8000 to 0xffef (bit 0x8000 set, not a
marker) is counted as `len_8000_up` and listed as a finding: it would collide with the piece flag.
Changed in Analyst session 17 (R-040): the per-type largest length is now the value length for every LONG
form (a direct-path chain could exceed 0xffff and was listed as a "length field" finding by mistake). A
conventional-path LONG / LONG RAW field is counted apart: `long_conv_8000_up` (one field with a u16 from
0x8000 to 0xfff0, e.g. `00 80`, `ef ff`, `f0 ff` in long3_exp_<v>.dmp) and `long_conv_chunked` (`fd ff` +
chunks + `00 00`, exp_parse.py `long_chunks`); they are listed as findings of their own kind, not as errors.
Changed in Analyst session 19 (R-048): `long_conv_fa` counts the conventional LONG / LONG RAW form `fa ff` + 4 bytes
+ value (exp_parse.py `long_fa`; long4_exp_<v>.dmp rows 3 to 5); `long_conv_8000_up` now takes one-field u16 values
up to 0xfff9 (0xfff1 and 0xfff5 are seen in long4_exp_<v>.dmp; 0xfff6 to 0xfff9 are not in the fixtures) and only
on the conventional path.
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    dirs = sys.argv[1:] or [os.path.join(here, "../../../corpus/fixtures", v) for v in ("18c", "11g", "11g-cp1251")]
    files = sorted(f for d in dirs for f in glob.glob(os.path.join(d, "*_exp_*.dmp")))
    tot = {"files": 0, "sections": 0, "rows": 0, "fields": 0, "null": 0, "len0": 0, "high": 0, "lob_sections": 0,
           "lob_values": 0, "lob_short": 0, "lob_long": 0, "lob_len0": 0, "lob_rows_k0": 0, "lob_rows_k": 0,
           "len_8000_up": 0, "long_chains": 0, "long_chains_multi": 0, "long_conv_8000_up": 0,
           "long_conv_chunked": 0, "long_conv_fa": 0}
    piece_lens: dict = {}
    chunk_lens: dict = {}
    per_type: dict = {}
    findings = []
    for f in files:
        d = open(f, "rb").read()
        r = parse(f)
        tot["files"] += 1
        for t in r["tables"]:
            if t["lob"]:
                tot["lob_sections"] += 1
                for _ri, _ci, _vo, buf, form, lens in t["lob_values"]:
                    tot["lob_values"] += 1
                    tot["lob_" + form] += 1
                    tot["lob_len0"] += len(buf) == 0
                    if form == "long":
                        for n in lens:
                            chunk_lens[n] = chunk_lens.get(n, 0) + 1
                rows_with = {ri for ri, *_x in t["lob_values"]}
                tot["lob_rows_k"] += len(rows_with)
                tot["lob_rows_k0"] += len(t["rows"]) - len(rows_with)
            tot["sections"] += 1
            chains = {fo: pl for _ri, fo, pl in t.get("long_pieces", [])}
            chunked = {fo: cl for _ri, fo, cl in t.get("long_chunks", [])}
            fa = {fo: n for _ri, fo, n in t.get("long_fa", [])}
            longcols = {ci for ci, c in enumerate(t["cols"]) if c["type"] in (8, 24)}
            for _ri, _fo, pl in t.get("long_pieces", []):
                tot["long_chains"] += 1
                tot["long_chains_multi"] += len(pl) > 1
                for n in pl:
                    piece_lens[n] = piece_lens.get(n, 0) + 1
            for _ro, row in t["rows"]:
                tot["rows"] += 1
                for ci, (fo, v) in enumerate(row):
                    tot["fields"] += 1
                    n = d[fo] | (d[fo + 1] << 8)
                    if v is None:
                        tot["null"] += 1
                        if n != 0xFFFE:
                            findings.append(f"{os.path.basename(f)}@0x{fo:x}: NULL with length field {n:04x}")
                        continue
                    typ = t["cols"][ci]["type"]
                    if fo in chains:
                        n = len(v)                   # session 9: a direct-path LONG piece chain: count the value length
                        lo, hi = per_type.get(typ, (1 << 16, -1))
                        per_type[typ] = (min(lo, n), max(hi, n))
                        continue
                    if fo in chunked:                # session 17: conventional LONG `fd ff` + chunks + `00 00`
                        tot["long_conv_chunked"] += 1
                        findings.append(f"{os.path.basename(f)}@0x{fo:x}: conventional LONG fd ff + "
                                        f"{len(chunked[fo]) - 1} chunks, {len(v)} bytes ({t['name']} col {ci + 1})")
                        lo, hi = per_type.get(typ, (1 << 16, -1))
                        per_type[typ] = (min(lo, len(v)), max(hi, len(v)))
                        continue
                    if fo in fa:                     # session 19: conventional LONG `fa ff` + u16 n + `00 00` + n bytes
                        tot["long_conv_fa"] += 1
                        findings.append(f"{os.path.basename(f)}@0x{fo:x}: conventional LONG fa ff + "
                                        f"{d[fo + 2:fo + 6].hex(' ')}, {len(v)} bytes ({t['name']} col {ci + 1})")
                        lo, hi = per_type.get(typ, (1 << 16, -1))
                        per_type[typ] = (min(lo, len(v)), max(hi, len(v)))
                        continue
                    if ci in longcols and not t["direct"] and 0x8000 <= n <= 0xFFF9:
                        tot["long_conv_8000_up"] += 1   # session 17: conventional LONG, one field, u16 from 0x8000
                        findings.append(f"{os.path.basename(f)}@0x{fo:x}: conventional LONG one field, length "
                                        f"{n:04x} = {n} ({t['name']} col {ci + 1})")
                        lo, hi = per_type.get(typ, (1 << 16, -1))
                        per_type[typ] = (min(lo, n), max(hi, n))
                        continue
                    if 0x8000 <= n < 0xFFF0:
                        tot["len_8000_up"] += 1
                        findings.append(f"{os.path.basename(f)}@0x{fo:x}: length field {n:04x} with bit 0x8000 "
                                        f"({t['name']} col {ci + 1})")
                    lo, hi = per_type.get(typ, (1 << 16, -1))
                    per_type[typ] = (min(lo, n), max(hi, n))
                    if n == 0:
                        tot["len0"] += 1
                        findings.append(f"{os.path.basename(f)}@0x{fo:x}: length 0 ({t['name']} col {ci + 1})")
                    if n >= 0xFFF0:
                        tot["high"] += 1
                        findings.append(f"{os.path.basename(f)}@0x{fo:x}: length field {n:04x} ({t['name']} col {ci + 1})")
    print(tot)
    print("  chunk lengths of long LOB values:", dict(sorted(chunk_lens.items())))
    print("  piece u16 values of direct-path LONG / LONG RAW chains:",
          {f"{k:#06x}": v for k, v in sorted(piece_lens.items())})
    for typ in sorted(per_type):
        print(f"  type {typ:3d}: smallest length {per_type[typ][0]}, largest {per_type[typ][1]}")
    print("findings:", len(findings))
    for x in findings:
        print("  " + x)


if __name__ == "__main__":
    main()
