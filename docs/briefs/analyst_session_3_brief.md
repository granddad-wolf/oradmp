The analyst finished session 3: every part of [docs/prompts/analyst-03-exp-18c.md](docs/prompts/analyst-03-exp-18c.md) is done except Part E step 4, which was skipped because `schema_exp_18c.dmp` and `schema_exp_direct_18c.dmp` are not on the machine. I confirmed that `corpus/work/18c/` holds only `logs/` and that the changed files match the list below; the format findings are the analyst's report, which I have not re-checked against the fixtures.

Nothing was committed, and the analyst reports the fixture checksums unchanged (all 80 entries OK before and after).

## Main finding: the client character set does not change row data

Part C step 3 of the prompt expected one byte per accented letter with the WE8MSWIN1252 client. The analyst found no conversion at all:

- **Table section:** byte-identical across the AL32UTF8, WE8MSWIN1252 and US7ASCII clients on both paths (`t_text_exp_w1252` 0x210d–0x258f equals `t_text_exp`).
- **Example row:** `Café Zürich ñ` is 16 bytes of UTF-8 with the w1252 client, at `t_text_exp_w1252` 0x230a.
- **What does differ:** only the header ids at 0x0001 and 0x0030 (0x0032 on direct), the file name and its length, the date, and four bytes at 0x20a2–0x20a5.
- **Reader rule:** decode VARCHAR2 and CHAR with the id at +4 of the column descriptor, never with header 0x0001. The header id is wrong in 4 of the 6 files.

## Other confirmed results

- **Whole corpus:** all 46 EXP files walk to the two `EXIT` lines. Of 64 table sections, 62 equal their truth (3272 fields, 0 differ); the other 2 are T_PARENT, which has no truth file and matches the SQL.
- **Empty tables:** on the conventional path `ff ff 0a` follows the path flag (`t_empty` 0x220a). On the direct path there is no `ff ff`, only the LF and then the next item.
- **Text region:** it contains length-prefixed fields (u16 LE length, bytes, LF) as well as lines, and the bytes may contain LF (`ddl` 0x253e). Statements can follow the rows of a table (`ddl` 0x2286, 0x2656–0x2a6c).
- **RAW (H-017):** all 7 rows equal the truth on both paths, including 2000 bytes as one field at `t_raw` 0x226a.
- **TSTZ:** always 13 bytes: 7 in the DATE layout in UTC, 4 of big-endian nanoseconds, 2 zone bytes. For offset zones the zone bytes are hour+20 and minute+60 (`t_tstz` 0x22b3).
- **TSLTZ (H-016):** rows 101 and 102 are byte-identical, 11 bytes.

## Still hypothesis

- **`fd ff`:** read as "a value in pieces follows", ended by `00 00`. There is one example per path (`ddl` 0x2885).
- **Field versus line:** two candidate rules give identical results on all fixtures; only the two `ddl` files contain fields.
- **Direct-path "no rows":** two candidate rules agree on all 31 direct tables; neither is proven.
- **TSTZ region zones:** bit 0x80 of byte 12 is set (Moscow, UTC, New York); the other bits are not understood.

## Unknown

- Whether comments and identifiers are converted to the client set (all fixture text is ASCII).
- Behaviour with a database that is not AL32UTF8.
- Owner mode and other statement kinds (triggers, views, grants, sequences).
- Whether TSLTZ is stored in UTC or database time zone (both are +00:00 here).

## Session 2 statements withdrawn

These are listed in section 9.1 of [exp.md](docs/formats/exp.md):

- "A table section ends with the LF after its rows" is wrong; statements can follow.
- "The text region is lines ending in LF" is wrong; it also holds length-prefixed fields.
- The trailer is not always five `METRICST`/`METRICET` pairs.
- The empty-table heuristic is now one of two candidate rules.

## Fixture requests for you

Both are in [requests.md](docs/formats/requests.md); the SQL has not been run.

- **R-012:** a small user `CORPUS_OWN` exported in owner mode and table mode on both paths, plus one w1252 owner export. It covers the unknowns above (non-ASCII comment, empty table with index and comment, sequence, view, trigger, grant and so on).
- **R-013:** four tables with uniform row sizes that put a length field and an `fe ff` marker across the 0x3fff/0x4000 boundary. It assumes a copy's `CREATE TABLE` text has the same length as the original; the request says how to check.

The analyst also added notes to R-008, R-009 and R-010 suggesting they be marked `generated`.

## Things to review

- **Extra table in the rules section:** section 11 "Rules for a reader" includes a separate table "Needed by a reader, but not confirmed", because the confirmed rules alone cannot walk `ddl`. Remove it if you want that section strictly confirmed-only.
- **Renumbering:** Tools moved from section 11 to 12.
- **New Result lines:** H-015 and H-016 had no Result line, so the analyst added one each.

## Files

Changed:
- [docs/formats/exp.md](docs/formats/exp.md)
- [docs/formats/requests.md](docs/formats/requests.md)
- [docs/formats/tools/exp_parse.py](docs/formats/tools/exp_parse.py)
- [docs/formats/tools/check_truth.py](docs/formats/tools/check_truth.py)
- [docs/research/hypotheses.md](docs/research/hypotheses.md)

Created:
- [docs/formats/tools/check_all.py](docs/formats/tools/check_all.py)
- [docs/formats/tools/clientset_diff.py](docs/formats/tools/clientset_diff.py)
- [docs/formats/tools/tstz_fields.py](docs/formats/tools/tstz_fields.py)