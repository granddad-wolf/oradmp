The analyst finished all of Parts A to F. Two things are only partly answered: no fixture separates the corrected framing rules G and K, and CLOB is read by hypothesis only. This is the analyst's report; I only checked the list of changed files and that no `t_probe11*` fixture exists.

## Part A

- **Checksums:** all `OK` — 99 of 99 in `18c/`, 99 of 99 in `11g/`, 16 of 16 in `11g-cp1251/`, re-checked at the end.
- **Session 3 tools on the new files, before changes:** 48 problems in each of the 6 `own_exp_*` files (the first is "no LF after field item at 0x34d1" in `own_exp_18c.dmp`), "RULES DISAGREE" in 10 files, and 23 tables per version with no truth.
- **Tool crashes, fixed:** `clientset_diff.py` stopped with a `TypeError` and `header_fields.py` with an `IsADirectoryError`.
- **One table DIFF, fixed:** `check_truth.py` reported "missing 2" in `ru_exp_w1252_11g.dmp`, because the failed table's name there is `bf bf bf 5f bf bf bf`, not `???_???`.
- **Final state:** 133 files walked, 0 problems, 0 disagreements between the corrected rules. The session 3 rules are kept as `generic3` / `keyword3` and still print "FAILS" in 12 files and "SILENT" in 16 on purpose.

| | `18c/` | `11g/` | `11g-cp1251/` |
|---|---|---|---|
| Files walked | 59 | 59 | 15 |
| Tables found (all with `INSERT INTO`) | 87 | 87 | 28 |
| Tables equal to truth | 85 | 85 | 22 |
| Tables equal to SQL or log only | 2 | 2 | 6 |
| Tables differing | 0 | 0 | 0 |
| Fields equal / differing | 3615 / 0 | 3615 / 0 | 27837 / 0 |

## Confirmed, suggested, unknown

**Confirmed**
- Header 0x0032 and block 1 +0x3c hold the database character set: 873 in 118 files, 171 in all 15 cp1251 files (`ru_tbl_exp_us7ascii_11g.dmp`@0x0035 `00 ab`, @0x083c).
- `fd ff` fields in pieces: six more examples per owner-mode file on both versions, with pieces equal to the truth's source line lengths (`own_exp_18c.dmp`@0x3554, 0x5c29, 0x5ee6, 0x2989).
- Text field lengths above 255 are u16 LE (`own_exp_18c.dmp`@0x2d5a `2e 01`).
- Inside a comment, LF is written raw and a quote is doubled (`own_exp_18c.dmp`@0x2d1d).
- A table section is byte-identical in owner mode and table mode: 12 comparisons, for example `own_exp_18c.dmp`@0x241c-0x2e8a against `own_tables_exp_18c.dmp`@0x2390-0x2dfe.
- The NVARCHAR2 descriptor length is twice the declared character count.
- The second record starts at the RECORDLENGTH named in the header, in all 133 files.

**Only suggested**
- The view text is written as pieces without `fd ff`, ended by `00 00` (`own_exp_18c.dmp`@0x349c); every view has just one piece.
- The missing `ff ff` after an empty table on 18c direct path comes from a table without a segment, not from the version.
- `T` / `U` in `METRICST` / `METRICSU` is the export mode.
- The CLOB layout, and the meaning of the second `ANALSTATS` letter.

**Unknown**
- A rule for the number after `METRICET` / `METRICEU`: +1 per statement in most sections, but +8 (18c) or +12 (11g) in the two "procedural objects" sections.
- Header 0x0034 (2000) and the TSTZ region bytes.
- TSLTZ, UTC or database time zone: the 11g database is +00:00 too.
- `ANALSTATS T?` lines, and an `exp` client older than the database: no fixture has either.

## Differences between 11g and 18c

Of 59 file pairs, 44 are byte-identical from offset R to the end.

1. Version line: `EXPORT:V11.02.00` against `V18.00.00`, same length.
2. **Empty table on the direct path, the only layout difference:** 11g writes `ff ff` + LF (`t_empty_exp_direct_11g.dmp`@0x226d), 18c writes only LF (`t_empty_exp_direct_18c.dmp`@0x220a).
3. `CREATE TABLE` text of empty tables: 11g has a `STORAGE(...)` clause, 18c has none.
4. `t_probe6` row order: 11g starts at R00257, 18c at R00065; all 300 rows equal truth on both.
5. Last argument of `SET_INDEX_STATS`: `0` on 11g, `6` on 18c (`ddl_exp_*`@0x286f).
6. `PROCACTION` text and the counter after it: 12 on 11g, 8 on 18c.

Header layout, both blocks, the string list, descriptors, all value encodings (including TSTZ region bytes `86 48`, `d0 04`, `81 90`), item framing, owner mode and trailer are identical.

## Part B: record boundary

Nothing is inserted, repeated or changed at a multiple of RECORDLENGTH; rule R5 is confirmed and open question 3 is closed.

- **Length field:** `t_bn1_exp_18c.dmp`@0x3fff `eb`, @0x4000 `04`; `t_bn2_exp_direct_18c.dmp` `ed` / `04`.
- **`fe ff` marker:** `t_bn3` and `t_bn4`@0x3fff `fe`, @0x4000 `ff`.
- **11g:** the same bytes at the same offsets.
- **Truth coverage:** the truth gives only length and first bytes; the rest was compared with what `06_bn.sql` must produce.

## Part C: framing rule

Neither session 3 rule survives as written.

- **Rule G** fails visibly at the view text; the deciding fixture is `own_exp_<v>.dmp`@0x349c.
- **Rule K** reads `PROCACTION`, procedure and trigger source as text lines and still reaches both `EXIT` lines without an error (`own_tables_exp_<v>.dmp`@0x3309, `own_exp_<v>.dmp`@0x214a). So "walked without a problem" is not a test of a framing rule.
- **Corrected rules:** G reads on as pieces when no LF follows a plain field; K has eight introducing lines instead of five. They agree on all 133 files, so no fixture separates them.
- **Recommendation, not a confirmed rule:** use the list, and for an unknown line test the next item with G and warn.

Other answers:
- **`O_PROC_LONG`:** one `fd ff` field of 125 pieces plus the ending piece, 7 + 9643 = 9650 bytes, no piece longer than 80.
- **`O_CHILD_CK2`:** pieces of 25, 25, 8, 414, 1, 18, 0, so the piece size is not constant.
- **`O_EMPTY` on 18c direct:** LF, then its statements (`own_exp_direct_18c.dmp`@0x2f71); E1 and E2 both still say "no rows".
- **Owner mode:** the first item is `METRICSU`, and one `ENDTABLE` can close several `TABLE` lines.

## Part F: CL8MSWIN1251 database

- **F1.** Yes. The R_TEXT row region is byte-identical for all four client sets, and 1854 fields per file equal `truth_ru.txt`.
- **F2.** The second header field is 171, so "database set" is confirmed; the fourth is 2000. Block 1 holds 171 / 2000 and block 2 the client id twice. The 8 strings are unchanged.
- **F3.** Descriptor +4 / +6 is 171 / 1 for VARCHAR2 and CHAR, and 2000 / 2 for NVARCHAR2. `NVARCHAR2(40)` has length 80. `VARCHAR2(20 CHAR)` has length 20, which decides nothing in a single-byte database.
- **F4.** Text items and identifiers:
  - Comments, CHECK text, view text and identifiers are in the client set, and their length fields count client bytes (8 bytes at `ru_exp_cp1251_11g.dmp`@0x135ab against 14 at `ru_exp_al32utf8_11g.dmp`@0x135ae).
  - Missing characters are replaced for good: `?` with US7ASCII, 0xbf with WE8MSWIN1252.
  - The failed table has its `TABLE`, `CREATE TABLE` and `INSERT INTO` lines, descriptors, path flag, then `ff ff` + LF. It has no rows and none of its index, statistics or comment items (`ru_exp_us7ascii_11g.dmp`@0x135ef-0x13748).
  - Nothing in the file marks it for certain. The best hint is replacement characters in the name while client and database set differ; the log is the only proof.
- **F5.** Confirmed: conventional headers say 2048, direct headers say 8192. A reader must use the header number.
- **F6.** `R` appears exactly when the log has `EXP-00091`, in all 29 files with such lines. It equally fits "client set differs from database set", and the fixtures cannot separate the two. Known forms are `ANALSTATS IS` / `IR` followed by a plain field and `ANALCOMPUTE IS` / `IR` as a line.
- **F7.** Type 112 has an 8-byte descriptor (`70 00 48 00 ab 00 01 00`), and 12 bytes instead of 4 stand before the rows. A `direct=y` file holds this table on the conventional path. All of it is observation or hypothesis, in section 14.6 marked "outside phase 1".

**Character set rule for a reader**

| Part of the file | Character set | Where to read it |
|---|---|---|
| Row data (VARCHAR2, CHAR) | database set | +4 of the column descriptor |
| NVARCHAR2 | UTF-16BE | +4 of the descriptor is 2000 |
| Text items | client set | header bytes 1-2 |
| Identifiers, including column names in `INSERT INTO` | client set, possibly with replacement characters | header bytes 1-2 |

**Survey claims the fixtures did not bear out**
1. "No value is valid UTF-8": 8 of 912 values are pure ASCII.
2. Header offsets 0x0030 / 0x0032 / 0x0034: the fields move with the text lines (0x0033 / 0x0035 / 0x0037 in `ru_tbl_*`).
3. `???_???` with the WE8MSWIN1252 client: the name is `bf bf bf 5f bf bf bf`, in the dump and in the log.
4. "The second record of an owner-mode file is different": it is the same; only the item text is `METRICSU`.
5. Rows from 0xb10: true for `ru_tbl_*`, but 0x0b0f in `ru_tables_exp_us7ascii_11g.dmp`.

## New requests

- **R-016** (most urgent): tables with table and column statistics, to see `ANALSTATS T?`.
- **R-017:** 18c empty table with a segment, direct path.
- **R-018:** long view, plain field above 8192 bytes, function / package / type — the cases that would separate G from K.
- **R-019:** database time zone other than +00:00, TSTZ offsets with minutes.

## Hypotheses updated

The Result lines of H-001 to H-021 each got a session 4 note with 11g offsets; H-015 and H-016 carry the 11g version note, and H-007 and H-021 are extended. H-022 is untouched.

## Statuses lowered (section 9.2 of `exp.md`)

- Rule G and rule K as written in session 3: withdrawn.
- "Direct path, table without rows: no `ff ff`": now confirmed for 18c only.
- "The path flag agrees with the header `D` in every fixture": withdrawn.
- "Section ends at the next `TABLE` line or `METRICST`": weakened (`ENDTABLE`, `METRICSU`).
- "First item is `METRICST`": weakened.
- "Both rules walk all files without a problem" as evidence: weakened.
- The session 3 sanity run on 11g missed the one layout difference.

## Where the session departed from the prompt

- **Section 11 rule ids moved.** New groups "Items" and "Character sets" were added, Y3 / Y4 became C1 / C3, and the E-rules were reordered.
- **All of H-001 to H-021 got notes.** The prompt said to change only the hypotheses tested and named four.
- **R-011 is marked `generated`**, as the prompt instructed for R-008 to R-013, but no `t_probe11*` file exists in any fixture folder.
- **It read files under `corpus/`:** `make_ru.sh` and `make_ru_tables.sh` (with grep, to confirm `recordlength=2048`) and the SQL files `05_own_objects.sql`, `06_bn.sql`, `07_ru_objects.sql`. Nothing was run.
- **Still open:** a plain field of 8192 bytes or more, and `VARCHAR2(n CHAR)` in a multi-byte database.
- **Minor:** it deleted `docs/formats/tools/__pycache__/`.

## Files

Changed: [exp.md](docs/formats/exp.md) (now 2007 lines), [requests.md](docs/formats/requests.md), [hypotheses.md](docs/research/hypotheses.md), and in [docs/formats/tools/](docs/formats/tools/) `exp_parse.py`, `check_truth.py`, `check_all.py`, `header_fields.py`, `clientset_diff.py`.

Created in [docs/formats/tools/](docs/formats/tools/): `exp_items.py`, `boundary.py`, `version_diff.py`, `rowdata_cmp.py`, `textitems_cs.py`, `anal_lines.py`.

Nothing is committed.