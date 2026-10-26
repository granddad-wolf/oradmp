# Legacy EXP: rules for a reader of table data

Extract of `exp.md` (Analyst session 20, "Rules for a reader" version 3.9). It lists what an
implementer needs to read the **rows** of a legacy `exp` dump and to skip everything else.
`exp.md` is the reference: every rule names the section that holds its evidence (fixture file and
hex offset) and no evidence is repeated here. If this file and `exp.md` disagree, `exp.md` wins
and this file must be regenerated.

Rule numbers (H1, I3, ...) are those of `exp.md` section 11. New numbers exist only in this file
where section 11 has no rule: `G*` (gaps a reader must handle), `X*` (what to stop on).

## Changes in session 20 (version 3.8 -> 3.9)

New fixtures (owner, 2026-10-25, `make_pending3.sh`): `long5_exp(_direct)_<v>` (R-051 (a), LONG 65,526 to 65,531 bytes, both
versions), `longraw5_exp(_direct)_<v>` (R-051 (b), (c): LONG RAW 65,521 to 65,534 bytes on 18c and 11g, plus 32,768, 65,520,
65,535, 100,000 bytes on 11g); 229 EXP files (105 / 97 / 27).

| rule | change | why (`exp.md`) |
|---|---|---|
| R2, Y9 | conventional form (a), one field, for a first u16 of **1 to 0xfff9** (0xfff6 to 0xfff9 now seen); form (b) `fa ff` + u16 n + `00 00` + n for **65,530 to 65,533** (all four seen); form (c) `fd ff` chunks from 65,534. **LONG RAW = LONG** in every form, 18c and 11g | 6.6 |
| R8 | in a conventional LONG / LONG RAW column: 1 to 0xfff9 = length (0xfff2 to 0xfff4 not in the fixtures), `fa ff`, `fd ff` = forms of R2, `fe ff` = NULL; **0, 0xfffb, 0xfffc: not in the fixtures, stop** | 5.6 |
| counts | 229 files, 450 binary parts, 41,494 rows, 243,905 column fields, 20,084 NULL (`tools/rowlen_stats.py`, `tools/corpus_counts.py`, `problems 0`) | 12 |

Lowered statuses: none. Raised: the one-field form for 0xfff6 to 0xfff9 (unknown -> confirmed, LONG 2 versions; 0xfff9
also LONG RAW); the `fa ff` form for LONG RAW and LONG RAW above 32,000 bytes on 11g (assumed -> confirmed).

## Changes in session 19 (version 3.7 -> 3.8)

New fixtures (owner, 2026-10-25, `make_pending2.sh`): `long4_exp(_direct)_<v>` (R-048, LONG, both versions),
`longraw4_exp(_direct)_18c` (R-048, LONG RAW, 18c only), `tsltz2_exp(_direct)_<v>` (R-050), `11g/truth_r049.txt` (R-049, a
record of ORA-00439, no dump); 221 EXP files (101 / 93 / 27).

| rule | change | why (`exp.md`) |
|---|---|---|
| R2, Y9 | a conventional LONG has **three** forms: one field (n up to 0xfff0, and 0xfff1, 0xfff5 seen), **`fa ff` + u16 n + `00 00` + n bytes** (65,530, 65,532, 65,533 seen), `fd ff` + chunks + `00 00` from **65,534** (was 65,535). LONG RAW (18c): the one-field and `fd ff` forms seen at 32,768, 65,520, 65,535, 100,000 bytes, with the LONG u16 values | 6.6 |
| R8 | in a conventional LONG / LONG RAW column: 1 to 0xfff5 = length (0xfff2 to 0xfff4 assumed), `fa ff` and `fd ff` = forms of R2, `fe ff` = NULL; **0, 0xfff6 to 0xfff9, 0xfffb, 0xfffc: not in the fixtures, stop** | 5.6 |
| T5, Y8 | TSLTZ (231): descriptor 7 for f = 0, 11 for f = 3, 6, 9; value 7 bytes with a zero fraction in every precision, 11 otherwise (= Y5) | 6.5 |
| E2, E3 | unchanged: R-049 is negative (11g XE refuses `SEGMENT CREATION DEFERRED`, ORA-00439); the 11g side of "segment, not version" cannot be tested in XE | 5.4, 13.2 |
| counts | 221 files, 434 binary parts, 41,378 rows, 243,593 column fields, 20,020 NULL (`tools/rowlen_stats.py`, `tools/corpus_counts.py`, `problems 0`) | 12 |

Lowered statuses: none. Raised: TSLTZ 7-byte values (unknown -> confirmed); LONG RAW from 32,768 bytes on 18c (assumed ->
confirmed for the two forms seen).

## Changes in session 17 (version 3.6 -> 3.7)

New fixtures (owner, 2026-10-25, `make_pending.sh`): `tsp_exp(_direct)_<v>` (R-047), `long3_exp(_direct)_<v>` (R-040),
`seg_exp(_direct)_18c` (R-017); 211 EXP files (95 / 89 / 27).

| rule | change | why (`exp.md`) |
|---|---|---|
| T5, Y5 | TIMESTAMP(f): descriptor length 11 for every f from 1 to 9, 7 for f = 0; value 7 or 11 bytes for every f; the fraction is the stored (database-rounded) value | 6.4 |
| R2, Y9 | **new exception**: a conventional-path LONG is one field with a u16 length up to 0xfff0 (65,520; bit 0x8000 is part of the length), or `fd ff` + chunks `u16 n + n bytes` + `00 00` (65,535 and more); a direct-path LONG is a chain for every size | 6.6 |
| R8 | **narrowed**: `f0 ff` and `fd ff` occur in a conventional LONG column (a length; the chunk marker); the "stop on a conventional LONG length from 0x8000" advice is withdrawn | 5.6, 6.6 |
| E2, E3 | the cause of the missing `ff ff` is the missing segment (confirmed on 18c); the reader rule does not change | 5.4, 13.2 |
| counts | 211 files, 418 binary parts, 41,284 rows, 243,303 column fields, 19,960 NULL (`tools/rowlen_stats.py`, `tools/corpus_counts.py`, `problems 0`) | 12 |

Lowered statuses: R8's absence statement no longer covers a conventional LONG column. Raised: E2's "why" (18c).

## Changes in session 11 (version 3.5 -> 3.6)

Housekeeping only: no new dump file (the one new file, `18c/truth_r045.txt`, is an SQL*Plus spool, not a dump); every
count below was regenerated over the 201 EXP files and is unchanged against session 10 unless stated.

| rule | change | why (`exp.md`) |
|---|---|---|
| counts | **unchanged** and re-derived in session 11: 201 EXP files (89 / 85 / 27; 189 `RTABLES`, 12 `RUSERS`), 386 table sections (170 / 164 / 52), 400 binary parts (184 / 164 / 52), 41,212 rows, 243,007 row fields, 19,900 NULL, 126 LOB values (80 short, 46 long), 86,080 NUMBER fields, 1,907 column definitions, 783 text-region fields, 522 `ANALSTATS` fields, 122 `ANALCOMPUTE` lines; `problems 0` in every tool (`tools/corpus_counts.py`, `tools/check_all.py`, `tools/rowlen_stats.py`, `tools/num_decode.py`, `tools/collist.py`, `tools/intro_list.py`, `tools/desc_len_check.py`, `tools/header_fields.py`) | 12 |
| T9 | **stale counts corrected**: "382 lines, 1885 column definitions" (the session 9 numbers) -> **386 lines, 1,907 column definitions** of 201 files (`tools/collist.py`: PROBLEMS 0, 6 sections "moved"). No change to the rule | 4.7, 12 |
| X2 | **evidence completed**: the third failed export, `ru_exp_direct_us7ascii_11g` (direct path; log lines 19 to 22: `???_???`, `EXP-00008`, `ORA-00604`, `ORA-00942`; `ff ff` @0x14cdc after the path flag `01 00` @0x14cda), added beside the two conventional files (`EXP-00056` / `ORA-00942`); the wording names both log codes. Status unchanged (unknown: no certain sign in the file) | 14.3 |
| C6, C8, S1, S2 | stale numbers corrected: `tools/collist.py` character-semantics columns 55 -> **57** (30 AL32UTF8 + 27 CL8MSWIN1251; the 2 `ru_lob2_*` files of session 10 add 2); C8 238,657 -> **243,007** column fields (223,085 non-NULL fields of types other than 8 / 24 compared by `tools/desc_len_check.py`, `EXCEEDS: 0`); S1 482 -> **522** fields; S2 118 -> **122** lines. No change to the rules | 6.3, 4.6, 12 |
| section 11 | R-045 **generated and used** (owner, 2026-10-17, `18c/truth_r045.txt`): Oracle 18c refuses LONG and LONG RAW in a partitioned table and `ALTER TABLE ... ADD (l LONG)` on one (three times ORA-14085). No partitioned LONG / LONG RAW fixture can exist on 18c; the T3 / T10 open point is closed by record. R-040 now `ready` (owner: `sqlldr` exists in the 18c container) | `requests.md` R-045, R-040 |

Lowered or raised statuses: none. No rule is changed in substance.

## Changes in session 10 (version 3.4 -> 3.5)

| rule | change | why (`exp.md`) |
|---|---|---|
| counts | 199 -> **201** EXP files (89 / 85 / 27: 2 new in `cp1251`, R-038); table sections 382 -> **386** (170 / 164 / 52; **400 binary parts**); rows 40,588 -> **41,212**; row fields 238,657 -> **243,007** (19,900 NULL); text-region fields 777 -> **783** (344 plain, 427 marked, 12 unmarked); `ANALSTATS` 522 (180 / 342) and `ANALCOMPUTE` 122 unchanged; NUMBER fields 85,456 -> **86,080** (now reproduced by `tools/num_decode.py`, note A-24); LOB values after the rows 126 (80 short, 46 long). Largest column field 32,000, largest LOB value 200,000, largest short LOB value 16,264 bytes: unchanged. `tools/corpus_counts.py`, `tools/rowlen_stats.py`, `tools/check_all.py`, `tools/num_decode.py` | 12 |
| L4 | **extended**: in the CL8MSWIN1251 database (11g) a 20,000-character CLOB is long with chunks of **8,132 bytes** (8,132 + 8,132 + 3,736), an NCLOB of 20,000 characters is long with chunks of **16,264 bytes** (16,264 + 16,264 + 7,472: the first long NCLOB of the corpus), a BLOB 8,132 + 8,132 + 3,736; where the short form ends for CLOB / NCLOB there: unknown (R-046) | 14.6 "Session 10" |
| L5 | **confirmed for the single-byte database** (R-038, note A-20): CLOB of set 171 = the database bytes in the short **and the long** form; NCLOB = UTF-16 BE short and long; BLOB = the bytes; identical with a CL8MSWIN1251 and a US7ASCII client. "A single-byte set = that set" is now stated for 171; for 1 / 178 as database sets: assumed | 14.6 "Session 10" |
| L6, T8 | more evidence: R_LOB2 conventional (flag 0); R_TEXT after R_LOB2 byte-identical to `ru_tbl_exp_*` (2 files) | 14.6 |
| C6 | w = 1 for set 171 **confirmed** (note A-23); other single-byte database sets: assumed | 6.3 |
| R2, R8 | note A-18: what a piece u16 of a direct-path LONG chain can be is stated; from 0xfff0, length 0: not in the fixtures (unknown) | 6.6 |
| E2, E2a | note A-19: **one** list of item starts for both (with the five partition lines) | 5.4 |
| X5 | NCLOB / BLOB / long CLOB in a single-byte database left the "not covered" list for CL8MSWIN1251 | 14.6 |

Lowered statuses: none. Raised: C6 (set 171), L4 / L5 (single-byte database, CL8MSWIN1251, 11g). Narrowed in wording:
the session 8 sentence "the CLOB chunk is twice the BLOB chunk" holds for a UTF-16 CLOB only (`exp.md` 9.8). No rule of
version 3.4 is refuted by the 2 new files. Tool defects found and fixed: `check_truth.py` (R_LOB2 had no branch and
stopped `check_all.py` / `num_decode.py` with a `TypeError`), `num_decode.py` step 3 (`KeyError: 0`, note A-24); neither
affects a rule.

**Consequence for ADR-25 (for the Architect).** For a dump whose database set [H4] is **171 (CL8MSWIN1251)**, the
fixtures now confirm (11g XE 11.2.0.2, `exp` 11.2, conventional path, clients CL8MSWIN1251 and US7ASCII, 1 table,
values of 66 and 20,000 characters): (1) CLOB in the **long** form = the database bytes, decoded with set 171, exactly
like the short form; T counts bytes = characters; (2) **NCLOB** in the short and the long form = UTF-16 BE, as in
AL32UTF8; (3) BLOB = the bytes in both forms. The two readings ADR-25 refused to choose between are decided: the long
CLOB is **not** UTF-16 (T = 20,000, not 40,000; the bytes decode with `cp1251` to the SQL text) and the NCLOB is **not**
single-byte (132 / 40,000 bytes for 66 / 20,000 characters). Wording the Architect can use: *"ADR-25 item 2 is lifted
for database set 171: a CLOB of the CL8MSWIN1251 database is decoded with set 171 in both forms and an NCLOB of that
database as UTF-16 BE in both forms, without `E-LOB-UNCONFIRMED` and without a warning [L4, L5 v3.5; R-038]. For the
single-byte database sets 1 (US7ASCII) and 178 (WE8MSWIN1252) the fixtures hold no database, so item 2 stays (or becomes
a counted warning, by the Architect's choice); this rests on an assumption that the mechanism is the same for every
single-byte set."* What stays unknown and does not block the lifting: where the short form ends for CLOB / NCLOB in the
single-byte database (R-046; a reader accepts both forms), a single-byte database on 18c (chunk sizes there), an `exp`
client older than the database (R-022, as for every rule).

## Changes in session 9 (version 3.3 -> 3.4)

| rule | change | why (`exp.md`) |
|---|---|---|
| counts | 177 -> **199** EXP files (89 / 85 / 25: 18c 12 new, 11g 10 new); table sections 334 -> **382** (170 / 164 / 48; **396 binary parts**: T_PART 3, T_PART2 6 per file); rows 40,172 -> **40,588**; row fields 237,347 -> **238,657** (19,186 NULL); text-region fields 737 -> **777** (340 plain, 425 marked, 12 unmarked); `ANALSTATS` 482 -> **522** (180 plain, 342 marked); `ANALCOMPUTE` 118 -> **122** (all new ones from `part2`). Unchanged: largest column field 32,000 bytes (on the direct path now also as a 5-piece chain), largest LOB value 200,000 bytes; new: largest short LOB value 16,264 bytes. `tools/corpus_counts.py`, `tools/rowlen_stats.py`, `tools/check_all.py` | 12 |
| T3, T9 | **narrowed**: on the direct path a LONG / LONG RAW column stands **last** in `INSERT INTO` and in the descriptors, while `CREATE TABLE` keeps the declared order (6 sections, 2 versions). Row order = `INSERT INTO` order; match `CREATE TABLE` definitions by name | 4.1, 4.7 |
| R2, R8, Y9 | **new exception**: on the direct path a LONG / LONG RAW field is a chain of pieces, u16 n with bit 0x8000 = "another piece follows" (`9c 80` + 156, 3 x `19 9f` + 7961, `19 1f` + 7961 = 32,000). LONG tables are written on the direct path (flag 1) | 6.6, 5.6 |
| Y9 | the character set of a LONG: the **database** set, **confirmed** (AL32UTF8 database, AL32UTF8 and US7ASCII clients, 2 versions) | 6.6 |
| Y1 | computed quotients / averages have 20 digit bytes (39 or 40 significant digits) on both versions; "no value has 39" corrected (5/3) | 6.1 |
| C5 | NCHAR pads to UTF-16 **code units**; a non-BMP character is a surrogate pair | 6.3 |
| C6, X4 | descriptor length of `VARCHAR2(n CHAR)` / `CHAR(n CHAR)` = min(4n, 4000) / min(4n, 2000) in AL32UTF8, **confirmed** (= `data_length`); X4 closed | 4.1, 6.3 |
| C8 | **new**: what a reader may check against the descriptor length | 6.3 |
| L4 | the short / long switch: long iff the byte length exceeds one chunk, **confirmed** (CLOB, BLOB, 2 versions) | 14.6 |
| L5 | non-ASCII and non-BMP CLOB / NCLOB in AL32UTF8: UTF-16 BE with surrogate pairs, **confirmed**; `GETLENGTH` counts code units | 14.6, 6.3 |
| Y10 | every field of a negative interval is negative, **confirmed**; precisions 9 / 9 and 2 / 0; no `.` when f = 0 | 6.7 |
| Y11 | ROWID prefix in 8 values; logical UROWID (first byte `02`) and its text | 6.7 |
| T10 | composite partitions (`COMPARTITION`, `SUBPARTITION`, `ENDCOMPARTITION`), empty subpartitions, statistics items before `ENDPARTITION` | 4.8 |
| E2, E2a | 3 more empty direct-path parts (18c); new item starts `SUBPARTITION "`, `ENDCOMPARTITION`; E2a applies only to a table whose only column is LONG / LONG RAW | 5.4 |
| T8 | 40 multi-table files compared, all identical (`tools/frame_t8.py`) | 4, 14.6 |
| R5 | a record boundary can split the LOB marker `fc ff` | 14.6 |

Lowered statuses: T9 loses "in order" for direct-path LONG tables (a counter-example in 6
sections); the absence statement of Y1 ("no value has 39 significant digits") is corrected. R2, R8
and Y9 are narrowed in scope (direct-path LONG), not refuted on the fixtures they were based on. No
other rule is refuted by the 22 new files.

## Changes in session 8 (version 3.2 -> 3.3)

| rule | change | why (`exp.md`) |
|---|---|---|
| counts | 169 -> **177** EXP files (77 / 75 / 25: 18c 5 new, 11g 3 new); table sections 314 -> **334** (144 / 142 / 48; **338 binary parts**, the partitioned table has 3); row fields 219,043 -> **237,347** (now with the LOB tables; 18,994 NULL); rows 40,172. **Unchanged:** 737 text-region fields (330 plain, 395 marked, 12 unmarked), 482 `ANALSTATS` fields (170 plain, 312 marked), 118 `ANALCOMPUTE` lines, largest column field 32,000 bytes (new: largest LOB value 200,000 bytes, in chunks). Date texts 20 to **24** bytes. `tools/corpus_counts.py`, `tools/rowlen_stats.py`, `tools/check_all.py` | 12 |
| A-4 check | the walk (`tools/rowlen_stats.py`) still finds **no** column field of length 0 and **no** `fd f0` to `fd ff` (0xfff0 to 0xfffd) in the 8 new files or anywhere else (237,347 fields). The LOB value area after a row is not a column field: there length 0 (empty LOB, 12 values) and `fc ff` (long LOB value, 8 values) occur | 5.6 |
| H4a | a two-digit day gives a 24-byte date text from 0x68 (4 files): no longer unknown | 2 |
| H6 | the pairing `RTABLES`-`METRICST`, `RUSERS`-`METRICSU` is **confirmed** in all 177 files (note A-10) | 11 |
| T5, X1 | codes 112 (CLOB, NCLOB; 8 bytes), 113 (BLOB), 69 (ROWID), 182, 183 (INTERVAL), 208 (UROWID) (4 bytes each) added; X1 now covers every other code | 4.1, 6.7, 14.6 |
| T6 | the two bytes before the flag are **m**: 0, or the column count of a table with a LOB column; then m column entries (L1) | 14.6 |
| T8 | more evidence: T_FRAME after LOB tables and after INTERVAL / ROWID tables | 14.6, 6.7 |
| T9 | more type texts (`BLOB`, `NCLOB`, `ROWID`, `UROWID(n)`, `INTERVAL YEAR (p) TO MONTH ` with a trailing space, `INTERVAL DAY (p) TO SECOND (f)`); 334 lines | 4.7 |
| T10 | **new**: a partitioned table (one section, one `PARTITION "p"` + `INSERT INTO` + binary part per partition, `ENDPARTITION`) | 4.8 |
| L1 to L6 | **new**: LOB columns (chapter 4a) | 14.6 |
| R4 | after a row of a LOB table the u16 is k (number of LOB values that follow) | 14.6 |
| R8 | reworded (note A-9): only 0 and 0xfff0 to 0xfffd are errors in a **column** field; larger lengths are not errors | 5.6 |
| E2a | **new**, suggested (note A-7): the direct-path "no rows" test when column 1 has descriptor length 0 | 5.4 |
| Y1 | correction (note A-6): 40 and 38 significant digits, not 39; "digit" = base-100 digit byte | 6.1 |
| Y9 | the character set a reader uses for LONG: the database set (note A-8), suggested | 6.6 |
| Y10, Y11 | **new**: INTERVAL; ROWID and UROWID, with conversion rules | 6.7 |
| X4 | R-011 still open; now request R-034 | 4.1 |

Lowered statuses: the session 4 hypothesis on the 8 bytes after the path flag of a LOB table is
**withdrawn** (it fitted one LOB column only); Y1 "39 significant digits" is corrected. No rule of
version 3.2 is refuted by the new files; R8 and C1 are narrowed in wording only.

## Changes in session 7 (version 3.1 -> 3.2)

| rule | change | why (`exp.md`) |
|---|---|---|
| counts | 157 -> **169** EXP files (72 / 72 / 25); 683 -> 737 fields after an introducing line; 117 -> 133 table sections per version; 438 -> 482 `ANALSTATS` fields; 108 -> 118 `ANALCOMPUTE` lines; length fields seen up to 32,000 (was 3000); date texts of 20 to 23 bytes (was 21 to 23). Regenerated with `tools/corpus_counts.py` (note A-1) | 12, 9.5 |
| T5 | type codes 8 (LONG) and 24 (LONG RAW) added (4 bytes, length 0); 96 also covers NCHAR (form 2); the numeric forms FLOAT, INTEGER, DECIMAL, REAL are all type 2, length 22 | 4.1 |
| X1 | narrowed to codes outside T5; states that type 112 also changes the bytes **before** the rows (note A-5) | 14.6 |
| T9 | **new**: the column list of the `CREATE TABLE` line (grammar, precision and scale) | 4.7 |
| R2 | lengths seen 1 to 32,000 | 5.2, 6.6 |
| R8 | **new**: a length 0 and the values 0xfff0 to 0xfffd (`fd ff`) never occur in row data (note A-4) | 5.6 |
| E2 | the test is stated: both tests of `exp.md` 5.4 (note A-2) | 5.4 |
| C1 | CHAR padding "to its declared byte length" holds for byte semantics only (**narrowed**) | 6.3, 9.5 |
| C5, C6, C7 | **new**: NCHAR; `CHAR(n CHAR)`; Oracle's byte-to-Unicode map of the single-byte sets | 6.3, 14.11 |
| X4 | `CHAR(n CHAR)` answered (4n in AL32UTF8); `VARCHAR2(n CHAR)` in AL32UTF8 still unknown | 4.1 |
| Y1 | **extended**: the conversion of the bytes to a decimal number, with limits | 6.1 |
| Y9 | **new**: LONG and LONG RAW (values up to 32,000 bytes, conventional path) | 6.6 |
| list K | each entry has a field for its allowed forms: `kind:length:forms:text` (notes A-3, N-3); kinds, texts and order unchanged; `tools/intro_list.py` checks the block | 4.2, 11 |
| S6, S7 | unchanged; two more files support them (`stc_exp_none_<v>`, `stc_exp_us7ascii_<v>` against `truth_stc_extra.txt`) | 4.6 |

Lowered statuses: none by evidence against a rule. Narrowed: C1 (padding, byte semantics only),
X1 (fewer codes stop the file). The R-023 prediction "14 + 4 statistics lines" was wrong because
the database's statistics changed between two exports (`exp.md` 4.6), not because of a rule.

## Status and scope

| status | meaning here |
|---|---|
| **confirmed** | `confirmed` in `exp.md`: at least two independent fixtures and agreement with a truth file or an export log |
| **suggested** | `observed` or `hypothesis` in `exp.md`, or a recommendation: the fixtures are consistent with it, they do not prove it |
| **unknown** | not in the fixtures, or bytes without an explanation. The reader must have a defined behaviour (stop, warn, or skip) |

What "confirmed" covers (`exp.md`, "Limit of confirmed"): Oracle 18c XE 18.4 and 11g XE 11.2.0.2;
`exp` **of the same version as the database**; databases AL32UTF8 (both versions) and CL8MSWIN1251
(11g only); clients AL32UTF8, US7ASCII, WE8MSWIN1252, CL8MSWIN1251; table mode and owner mode;
conventional and direct path; RECORDLENGTH 2048, 4096, 8192, 16384; 229 EXP files. The types of
session 7 (NCHAR, `CHAR(n CHAR)`, LONG, LONG RAW, the 14 declared forms of T_DECL) rest on one
table each, on both versions, conventional path, client = database = AL32UTF8. Session 8: LOB
columns rest on 2 tables (CLOB, NCLOB, BLOB) on both versions in AL32UTF8 and 1 CLOB table in
CL8MSWIN1251 (4 client sets); INTERVAL, ROWID, UROWID on 1 table each, both versions, 3 / 1
non-NULL values per column; partitions on 1 range-partitioned table, 18c only, both paths, no
statistics. Session 9: computed NUMBER values on 1 column (6 values); character semantics on 7 + 4
columns (AL32UTF8, 2 versions, both paths); one non-BMP character in 6 column kinds; LOB sizes on 1
table (11 sizes, CLOB and BLOB, 2 versions); intervals and ROWIDs on 1 table each; LONG on the direct
path on 3 tables (values up to 32,000 bytes) and one non-ASCII LONG with 2 client sets; one
range-hash composite-partitioned table with statistics (18c, both paths). Session 10: CLOB, NCLOB and BLOB values of
the CL8MSWIN1251 database (11g) on 1 table, 2 values per column (66 and 20,000 characters), 2 client sets. **Not covered by any
fixture:** (session 17: TIMESTAMP(1) to (8) and LONG of 32,767 to 100,000 bytes are now covered: `tsp`, `long3`, 2 versions, both paths) an `exp` client older than the database (the project's real dumps are said to be written
by a 10.2 client against 11.2.0.4), full database mode, compression, BFILE, (session 19: a conventional LONG of 65,521 to
65,535 bytes and a LONG RAW of 32,768 to 100,000 bytes on 18c are now covered: `long4`, `longraw4`) (session 20: a conventional LONG of
65,526 to 65,531 bytes and a LONG RAW of 32,768 to 100,000 bytes on 18c and 11g are now covered: `long5`, `longraw5`) a conventional LONG or LONG RAW of 65,522 to 65,524 bytes, a LONG RAW of 65,526 to 65,528 or 65,532 bytes (R-052, proposed), a LOB in a single-byte database with values around the switch (R-046), a LOB in a database
whose set is US7ASCII or WE8MSWIN1252, a single-byte database on 18c, list / interval partitioning, partitions in owner
mode.

Fixture folders: `18c` = `corpus/fixtures/18c/`, `11g` = `corpus/fixtures/11g/`, `cp1251` =
`corpus/fixtures/11g-cp1251/`. Fixture names below are stems; `X` means `X_exp_<v>.dmp` and
`X_exp_direct_<v>.dmp` in `18c` and `11g`; the stems of session 7 (`t_num2`, `t_decl`, `nchar`,
`long`) and `misc` (session 8) have no direct file; `part` (session 8) exists in `18c` only. Session
9 stems: `t_num3`, `nchar2`, `lob3`, `misc2` (conventional only), `t_probe11`, `long2` (both paths;
`long2_exp_us7ascii_<v>` with a US7ASCII client), `long_exp_direct_<v>` (direct only), `part2` (both
paths, `18c` only). Session 10: `ru_lob2_exp_cp1251_11g`, `ru_lob2_exp_us7ascii_11g` (`cp1251`, conventional). Session 17:
`tsp`, `long3` (both paths, `18c` and `11g`), `seg` (both paths, `18c` only). Session 19: `long4`, `tsltz2` (both paths,
`18c` and `11g`), `longraw4` (both paths, `18c` only). Session 20: `long5`, `longraw5` (both paths, `18c` and `11g`; `longraw5` has 7 rows on 18c and 11 on 11g).

Byte order: integers of the header and of the two blocks of the second record are big-endian
(BE); every integer of the table data and of the text fields is little-endian (LE).

## 1. First record: header

| rule | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|
| H1 | Byte 0 is `03`. Bytes 1-2: u16 BE id of the **client** character set of the export session (text items and identifiers, rule C2) | confirmed (meaning of `03`: unknown) | `t_frame` with 3 client sets, `ru_tbl_*` with 4 (`cp1251`) | 2, 2.1, 14 |
| H2 | If bytes 3-4 are `D` LF the export was started with `direct=y`; the text lines then start 2 bytes later. The path of each table is given by rule T6, not by this | confirmed | every `*_exp_direct_*` file; `ru_exp_direct_cp1251_11g` | 2, 8 |
| H3 | Seven LF-terminated text lines follow: version (`EXPORT:V18.00.00`, `EXPORT:V11.02.00`), `U` + user, `R` + mode (`RTABLES`, `RUSERS`), RECORDLENGTH **R** as decimal text, then `0`, `72`, `0`. Their lengths vary, so nothing after them has a fixed offset | confirmed (the three numbers: unknown meaning; other modes: unknown) | all files; `own`, `ru_exp_*` for `RUSERS` | 1.1, 2, 14.4 |
| H4 | After the seventh line: three u16 BE: client set again, **database** character set, national character set (2000 in every fixture). Then 10 bytes, u16 BE length of the file name, one `00` | confirmed (ids); unknown (the 10 bytes) | `t_frame_exp_us7ascii`, `ru_tbl_exp_us7ascii_11g` | 2, 14.4 |
| H4a | Informative only: spaces, then the export date as text ending at 0x7f (`Www Mmm D H:M:S YYYY`, no leading zeros, 20 to 24 bytes seen; 24 with a two-digit day), then the file name at 0x80. A reader of data needs neither | confirmed as place and format | all 201 files (`tools/header_date.py`); two-digit day: `lob_exp_18c`@0x0068 | 2 |
| H5 | The first record is R bytes: header, then zero bytes. The second record starts at offset R. The file size is a multiple of R. Take R from the header text; never assume it, and never use the value that was asked for (direct path writes 8192 when 2048 was asked) | confirmed | `t_frame_exp_rl4096`, `t_frame_exp_rl16384`, `ru_tbl_exp_direct_*` | 1, 1.1, 14.7 |
| H6 | R + 0x000 to R + 0x103: two blocks of 102 bytes and 8 strings with u16 LE length, then LF. Constant in the fixtures apart from character set ids; mostly unexplained. Skip them. The first item is at **R + 0x104** and is the line `METRICST` (table mode) or `METRICSU` (owner mode): verify it and stop if it is not there. The pairing with the mode line of H3 (`RTABLES` - `METRICST`, `RUSERS` - `METRICSU`) holds in every fixture (note A-10) | confirmed (as constant bytes, as the place of the first item, and the pairing for these two modes: 189 + 12 files, no exception); unknown (their meaning; other modes) | all 201 files (`tools/header_fields.py`) | 1, 3, 4.5, 11 |

## 2. Items of the text region (how to skip everything that is not rows)

From R + 0x104 to the second of two consecutive `EXIT` lines the file is a sequence of **items**;
the binary part after an `INSERT INTO` line (chapters 3 to 5 below) is the only thing that is not
an item (rule I1, confirmed, 201 files).

| rule | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|
| I2 | **Line**: text bytes ended by LF (0x0a) | confirmed | all | 4.2 |
| I3 | **Plain field** (`plain`): u16 LE length n, n bytes (LF and quotes allowed inside), then one LF not counted in n. n can exceed 255 (302 seen) | confirmed | `ddl`, `own`, `stat`, `stc`, `t_decl` | 4.2, 4.5 |
| I4 | **Field in pieces** (`marked`): `fd ff`, then pieces "u16 LE n + n bytes" up to a piece with n = 0, then LF. Piece lengths vary (1 to 797 seen); more than 100 pieces and more than 8192 bytes in one field occur | confirmed | `ddl`, `own`, `stat`, `stc`, `ru_stat_*` | 4.2, 4.5, 4.6 |
| I5 | **Pieces without marker** (`unmarked`, text of a view): pieces "u16 LE n + n bytes" up to n = 0, then LF, with no `fd ff` | suggested as "pieces" (every view of the fixtures has one piece); confirmed as bytes | `own`, `ru_exp_*` | 4.2 |
| I6 | A field follows exactly the lines of the list below (rule K); exactly one field per such line, in one of the forms the entry allows; every other line is a complete item | confirmed for the fixtures; the list is complete **for the fixtures only** | all 201 files: 783 fields, 0 exceptions (`tools/intro_list.py`) | 4.2, 11 |
| I7 | Nothing is inserted at a multiple of R inside a field of the text region | confirmed | `own` | 4.5 |
| G1 | Fallback when a line is not in the list (rule G): look at the next item; if its first two bytes are `fd ff`, or its second byte is below 0x20, it is a field: read "u16 n + n bytes", and if the next byte is not LF read on as pieces up to n = 0. Treat it as a field and **warn** (unknown introducing line). Rule G alone reads the same items as the list on all 201 files | suggested (a recommendation; no fixture separates the two rules) | all 201 files (`tools/check_all.py`: `rule_diff` 0) | 4.2, 11 (gaps) |
| G2 | Rule G fails on a plain field of 8192 bytes or more, a view in several pieces, a line of one character; the list fails silently on an introducing line it lacks (functions, packages, types are not in the fixtures) | unknown (R-018) | - | 4.2 |

**List K**, the lines that introduce a field. One entry per line, `kind:length:forms:text`.
`kind`: `prefix` (the line starts with the text), `exact` (the line is the text), `suffix` (the
line ends with it). `length`: byte length of the text, trailing spaces included. `forms`: the
allowed forms of the field, joined by `|`: `plain` (I3), `marked` (I4), `unmarked` (I5). `text`:
to the end of the line. This block is the single source of the list; `tools/intro_list.py` reads
it (and the copy at the end of `exp.md` section 11) and checks every field of every entry against
its own list and against the 201 files (`block_diff` 0). Session 10: unchanged. Session 8: unchanged; the lines
`PARTITION "name"` and `ENDPARTITION` (T10) are plain lines, not introducing lines. Session 9:
unchanged; `COMPARTITION "p"`, `SUBPARTITION "s"`, `ENDCOMPARTITION` (T10) are plain lines too.

    suffix:15:plain: MODIFY DEFAULT
    prefix:10:plain|marked:ANALSTATS 
    prefix:11:plain:COMMENT ON 
    exact:12:marked:CALTER TABLE
    exact:10:marked:PROCACTION
    prefix:26:marked:CREATE FORMAT71 PROCEDURE 
    exact:20:marked:CREATE FORMAT92TRIG 
    prefix:12:unmarked:CREATE VIEW 

Fields per entry in the 201 files: 30, 180 plain + 342 marked, 134, 51, 12, 12, 10, 12. For
`ANALSTATS ` decide the form by the two bytes after the LF (`fd ff` = marked), rule S1, never by
the letters. Lines that start with `ANALCOMPUTE ` are **not** in the list.

## 3. Table section

| rule | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|
| T1 | A line `TABLE "name"` directly followed by a line `CREATE TABLE ...` starts a table section. A `TABLE "name"` line followed by anything else starts a statement group that ends with `ENDTABLE` (one `ENDTABLE` can close several `TABLE` lines: do not pair them). Session 8: a LOB table gets one empty `TABLE "name"` / `ENDTABLE` group per LOB column in the trailer; they are such groups | confirmed | `ddl`, `own`, `own_tables`, `stat`, `stc`; `lob` | 4, 4.4, 7.1, 14.6 |
| T2 | The section ends at the next line that starts with `TABLE "`, is `ENDTABLE`, or starts with `METRICS` | confirmed | `multi_*`, `own` | 4, 4.5 |
| T3 | Table data follow the LF of the line `INSERT INTO "name" ("col", ...) VALUES (:1, ...)`. That line gives the column names in order, in the client character set (rule C2). No other line starts table data. Never search the file for the text `INSERT INTO`: row data can hold any bytes. A section can have **several** `INSERT INTO` lines (one per partition or subpartition, T10). **Session 9: the descriptors and the row fields are in the order of this line, which is not always the `CREATE TABLE` order**: on the direct path a LONG / LONG RAW column stands last (`long2_exp_direct_<v>`: `"ID", "V", "L"` against `"ID" ..., "L" LONG, "V" ...`) | confirmed (the reordering: 6 sections, 2 versions) | all; `part`, `part2`; `long2`/direct, `long_exp_direct_<v>` | 4, 4.4, 4.8, 4.1 |
| T4 | After that LF: u16 LE column count, then one descriptor per column | confirmed | all (1 to 120 columns) | 4 |
| T5 | Descriptor: u16 LE type code, u16 LE maximum byte length; for type 1 (VARCHAR2, NVARCHAR2), 96 (CHAR, NCHAR) and **112 (CLOB, NCLOB)** four more bytes: u16 LE character set id, u16 LE form (1 with the database set, 2 with the national set 2000). Types 2 (NUMBER; also what was declared FLOAT, INTEGER, DECIMAL, REAL: always length 22), 8 (LONG, length 0), 12 (DATE), 23 (RAW), 24 (LONG RAW, length 0), **69 (ROWID, length 23)**, **113 (BLOB, length 72)**, 180 (TIMESTAMP), 181 (TIMESTAMP WITH TIME ZONE), **182 (INTERVAL YEAR TO MONTH, 5)**, **183 (INTERVAL DAY TO SECOND, 11)**, **208 (UROWID, the declared maximum)**, 231 (TIMESTAMP WITH LOCAL TIME ZONE): 4 bytes. The type code equals the `Typ=` of `DUMP`. The descriptor carries no precision, scale or NOT NULL (rule T9). The length of a LOB descriptor (72) is not a value length **Session 17 (R-047)**: the maximum length of 180 is 7 for TIMESTAMP(0) and 11 for every f from 1 to 9 (all ten precisions seen, = `user_tab_columns.data_length`) **Session 19 (R-050)**: the same for 231 (TIMESTAMP WITH LOCAL TIME ZONE): 7 for f = 0, 11 for f = 3, 6, 9 (= `data_length`); f = 1, 2, 4, 5, 7, 8 of 231 not seen (11 assumed) | confirmed for these 16 codes (meaning of the form value: suggested; 112, 113, 69, 182, 183, 208: 1 to 2 tables each, 2 versions); TIMESTAMP length 11 for f = 1 to 9: confirmed (2 versions, 2 paths) | `t_text`, `t_num`, `t_date`, `t_raw`, `t_ts`, `t_tstz`, `ru_tbl_*`, `t_decl`, `nchar`, `long`, `lob`, `misc`, `ru_tables_exp_us7ascii_11g`, `tsp`, `tsltz2` | 4.1, 6.5, 6.7, 14.6 |
| X1 | Any other type code (BFILE 114, object, XML, collection types, ...): the descriptor size is not known. **Stop reading this file** (the rest cannot be located) | unknown | - | 4.1 |
| T6 | After the descriptors: u16 LE **m**, then u16 LE path flag: 0 = conventional, 1 = direct. m = 0 for every table without a LOB column: the rows start after the flag. m = the column count for a table with a LOB column: the m column entries of rule L1 follow the flag, then the rows. Any other m: stop (not in the fixtures) | confirmed (flag; m = 0 in 379 binary parts, m = column count in 21 LOB sections of 6 tables); unknown (other flag values) | every pair conventional / direct; `lob`, `lob3`, `nchar2`, `ru_tables_exp_us7ascii_11g`, `ru_lob2_*` | 4, 8, 14.6 |
| T7 | Items can stand between `CREATE TABLE` and `INSERT INTO` (DEFAULT values) and after the rows (indexes, index statistics, grants, constraints, table and column statistics, comments). Skip them item by item; do not rely on their number or order. No statistics item stands before the rows of its table | confirmed | `ddl`, `own`, `stat`, `stc`, `ru_stat_*`, `t_decl` | 4, 4.3, 4.5, 4.6 |
| T8 | A table section is the same bytes in table mode and in owner mode, alone or with other tables | confirmed | `own` / `own_tables`, `multi_*`; `long` (T_FRAME after LONG tables = `t_frame`); `lob`, `misc` (T_FRAME after LOB, INTERVAL and ROWID tables = `t_frame`, both paths, both versions); session 9: 40 multi-table files of `18c` / `11g` compared by `tools/frame_t8.py`, all identical (after LONG piece chains, LOB3, NCHAR2, PROBE11, INTERVAL2 / ROWID2, composite partitions); session 10: R_TEXT after R_LOB2 = R_TEXT of `ru_tbl_exp_cp1251_11g` / `ru_tbl_exp_us7ascii_11g` (`tools/lob_ru_check.py`) | 4, 4.5, 6.6, 6.7, 14.6 |
| T9 | The `CREATE TABLE` line: `CREATE TABLE "t" (` coldef (`, ` coldef)* `)` + two spaces + storage text. coldef = `"name"` + one space + type + optionally ` NOT NULL ENABLE`. A name runs from `"` to the next `"` (commas, parentheses inside are part of it). Nothing else stands in the list: no DEFAULT (that is a ` MODIFY DEFAULT` field before the `INSERT INTO` line), no constraint. Types and what they give: `NUMBER` (precision and scale none), `NUMBER(p, s)` (p, s; s may be negative or above p), `NUMBER(*,s)` (none, s; INTEGER is `NUMBER(*,0)`), `FLOAT(b)` (binary precision b; FLOAT = `FLOAT(126)`, REAL = `FLOAT(63)`), `VARCHAR2(n)`, `VARCHAR2(n CHAR)`, `CHAR(n)`, `CHAR(n CHAR)`, `NVARCHAR2(n)`, `NCHAR(n)`, `RAW(n)`, `DATE`, `TIMESTAMP (f)` (space before the parenthesis; plain TIMESTAMP is `(6)`), `TIMESTAMP (f) WITH TIME ZONE`, `TIMESTAMP (f) WITH LOCAL TIME ZONE`, `CLOB`, `LONG`, `LONG RAW`; session 8: `NCLOB`, `BLOB`, `ROWID`, `UROWID(n)`, `INTERVAL YEAR (p) TO MONTH ` (**a space follows `MONTH` before the separator `, ` or the closing `)`**), `INTERVAL DAY (p) TO SECOND (f)` (p, f give the leading and fractional precision Y10 needs). The text gives exactly `data_precision` / `data_scale` of the dictionary; the names equal those of the `INSERT INTO` line, and the definitions match the descriptors in number, type and length, **in order except for a LONG / LONG RAW column on the direct path, which stands last in `INSERT INTO` and the descriptors (session 9: match by name)**. Character semantics: the descriptor length is min(4n, 4000 / 2000) in AL32UTF8 (C6); `VARCHAR2(n BYTE)` is written `VARCHAR2(n)`. A composite-partitioned table has `PARTITION BY RANGE (...)  SUBPARTITION BY HASH (...)  SUBPARTITIONS k (...)` in its storage text. A partitioned table has its `PARTITION BY ...` clause in the storage text after the two spaces. Any other type text: keep the text, report the declared type as unknown | confirmed for the forms listed (386 lines, 1,907 column definitions of 201 files, `tools/collist.py`: 0 problems, 6 sections "moved"; T_DECL against `truth_decl.txt`; session 11); "in order": **refuted** for direct-path LONG tables, confirmed elsewhere; other forms: unknown | `t_decl` (14 forms), `lob`, `misc`, `part`, `part2`, `t_probe11`, `long_exp_direct_<v>`, all files | 4.7, 4.1 |
| T10 | **Partitioned table** (session 8): one table section (`TABLE "name"`, one `CREATE TABLE` line), and inside it, per partition, a plain line `PARTITION "p"`, an `INSERT INTO` line and a complete binary part (T4 to T6, rows, `ff ff`, LF); after the last one a plain line `ENDPARTITION`; then the section ends as usual (T2). The rows of the table are the rows of all parts in file order; a reader can report the partition name of each part. The parts have the same descriptors; each has its own path flag. **Session 9, composite partitions**: per partition a plain line `COMPARTITION "p"`, then per subpartition a plain line `SUBPARTITION "s"`, an `INSERT INTO` line and a complete binary part, then a plain line `ENDCOMPARTITION`; the partition has no binary part of its own. An empty (sub)partition is an empty binary part (E1; E2 on the 18c direct path). With statistics, the `ANALSTATS` items of the table and of every partition (not of subpartitions, in the fixture) stand after the last part and **before** `ENDPARTITION`; without statistics `ENDPARTITION` follows the last part directly. What a reader does differently from a plain table: accept several `INSERT INTO` lines per section; keep reading items after a part until the section ends (T2), not until the first `ENDPARTITION`-like line; report the partition / subpartition name per part if wanted; sum the rows of all parts; check that every part has the same column count and descriptors; include the partition lines in the item starts of E2 | confirmed (2 tables: range 3 partitions without statistics; range-hash 3 x 2 with statistics and an empty partition; both paths, 18c; row counts = truth = log); other partitioning methods, owner mode, indexes: unknown | `part`, `part2` (18c only) | 4.8 |

## 4. Rows and fields

| rule | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|
| R1 | A row is exactly "column count" fields in column order (the order of the `INSERT INTO` line, T3). No row header, row length or row number | confirmed | all (386 table sections, 400 binary parts) | 5.1 |
| R2 | A field is a u16 LE length n followed by n value bytes. The length field is always 2 bytes (1 to 32,000 seen), does not count itself; 255 / 256 / 257 need no escape. **Exception (session 9): a LONG / LONG RAW field on the direct path (flag 1) is a chain of pieces**: u16 n, then (n & 0x7fff) bytes; if bit 0x8000 of n is set another piece follows, else the value ends (a short value is one piece = an ordinary field). NULL is `fe ff` as always. **Session 10 (note A-18)**: seen as piece u16: `fe ff` (NULL; first position only), lengths without the flag (1 to 7,961), 0x809c, 0x9f19; the largest piece 7,961 bytes. **Not in the fixtures**: a piece u16 from 0xfff0 (a piece of 0x7ff0 bytes or more with the flag), a piece of length 0 with or without the flag (0x8000, 0x0000; also as a first piece), `fe ff` / `ff ff` after the first piece. Which reading wins there (marker or piece) is **unknown**; treating a later piece u16 from 0xfff0 as a framing error is a recommendation consistent with the fixtures, not evidence **Session 17 (R-040): a second exception, a LONG field on the conventional path (flag 0)**: either one ordinary field whose u16 n is the length for 1 <= n <= 0xfff0 (bit 0x8000 is part of the length: `00 80` = 32,768, `ef ff` = 65,519, `f0 ff` = 65,520 seen), or `fd ff` followed by chunks `u16 n + n bytes` up to a chunk `00 00` (65,535, 65,536 and 100,000 bytes seen; chunks of 4000, remainder last; never assume the size). Where the switch lies between 65,521 and 65,535 bytes is unknown (R-048): a u16 from 0xfff1 to 0xfffc in such a field = stop. **Session 19 (R-048), replaces the last sentence**: a conventional LONG / LONG RAW field (flag 0) has **three forms**, chosen by its first u16 u: (a) u = 1 to 0xfff5: one field of u bytes (0x8000, 0xffef, 0xfff0, 0xfff1, 0xfff5 seen above 0x7fff; 0xfff2 to 0xfff4 lie between seen values, assumed); (b) **u = 0xfffa (`fa ff`): 4 bytes follow, u16 LE n and `00 00`, then n value bytes** (n = 65,530, 65,532, 65,533 seen; reading the 4 bytes as u32 LE is a hypothesis; stop if the last two bytes are not `00 00`); (c) u = 0xfffd: chunks up to `00 00` (65,534 to 100,000 seen). u = 0xfff6 to 0xfff9, 0xfffb, 0xfffc: not in the fixtures, stop (R-051). On the direct path a LONG of any size is a chain (100,000 = 12 x 7952 + 4576 seen; 18c and 11g cut the same value differently). **Session 20 (R-051), replaces (a) to (c) above**: (a) u = 1 to **0xfff9**: one field of u bytes (seen above 0x7fff: 0x8000, 0xffef, 0xfff0, 0xfff1, 0xfff5, 0xfff6, 0xfff7, 0xfff8, 0xfff9; 0xfff2 to 0xfff4 lie between seen values: hypothesis); (b) u = 0xfffa: 4 bytes u16 LE n + `00 00`, then n bytes (n = 65,530, 65,531, 65,532, 65,533 seen, i.e. 0xfffa to 0xfffd; stop if the last two bytes are not `00 00`; u32 LE: hypothesis); (c) u = 0xfffd: chunks up to `00 00` (65,534 to 100,000 seen; chunks of 4000 seen, never assume the size). u = 0xfffb, 0xfffc: not in the fixtures, stop. LONG RAW uses the same three forms with the same u16 values, on 18c and 11g | confirmed (the chain: 4 values of 32,000 bytes, 156 + 4 x 7961, LONG and LONG RAW, 2 versions; 6 one-piece values); session 17: the conventional LONG forms confirmed (7 values 32,767 to 100,000, LONG only, 2 versions); LONG RAW above 32,000: assumed the same, not in the fixtures; session 19: form (a) confirmed at 0xfff1 and 0xfff5, form (b) at 3 sizes, form (c) from 65,534 (LONG, 2 versions, 7 sizes); LONG RAW (18c): forms (a) at 32,768 / 65,520 and (c) at 65,535 / 100,000 confirmed, form (b) assumed; LONG RAW on 11g above 32,000: assumed (every 11g LONG file is byte-identical to its 18c file); session 20: form (a) confirmed at 0xfff6 to 0xfff9 (LONG, 2 versions), the switch (a) -> (b) between 65,529 and 65,530 and (b) -> (c) between 65,533 and 65,534 on both types and versions; LONG RAW: forms (a), (b), (c) confirmed on 18c and 11g (11 sizes, 11g = 18c byte for byte) | `t_len`, `t_bn3`, `t_bn4`, `ru_tbl_*`, `long`; `long_exp_direct_<v>`, `long2`/direct, `long3`, `long4`, `longraw4_exp(_direct)_18c`, `long5`, `longraw5` | 5.1, 5.2, 14.1, 6.6 |
| R3 | `fe ff` in place of the length is NULL, for every type and position; no value bytes follow. Trailing NULLs are written, one `fe ff` each. (A length of 254 is `fe 00`.) | confirmed | `t_frame`, `t_nullmix`, `stc`, `nchar`, `long` | 5.1, 5.3 |
| R4 | Conventional path (flag 0): `00 00` follows the last field of every row. Direct path (flag 1): nothing stands between rows. In a table with a LOB column (m > 0) the u16 after the row is k, followed by k LOB values (L3); k = 0 is the `00 00` | confirmed | every pair; `lob` | 5.1, 8, 14.6 |
| R5 | Nothing is inserted, repeated or changed at a multiple of R inside the table data: a value, a length field, an `fe ff` marker and a `00 00` row end continue across it. Read the file as one byte stream | confirmed | `t_bn1` to `t_bn4`, `t_probe6`, `ru_exp_cp1251_11g`, `long` (32,000-byte values across 3 boundaries each); session 9: the LOB marker `fc ff` split at `lob3_exp_11g`@0x47fff / 0x48000; LONG piece chains across boundaries (`long_exp_direct_<v>`) | 1.2, 6.6, 14.6 |
| R6 | Rows are not sorted; the order can differ between two exports of the same table. No row count is stored. The number after `METRICET` / `METRICEU` is **not** the row count of a table; the `num_rows` of a statistics item is not either | confirmed | `t_probe6`, `multi_*`, `stat` | 5.5, 7.1, 13.4 |
| R7 | The path changes nothing else: values, lengths, NULL markers and descriptors are identical on both paths | confirmed | every pair | 8 |
| R8 | In a **column field** the u16 is: 1 to 0xffef = a length; `fe ff` = NULL; `ff ff` = end of rows (at the start of a row only). Only **0** and **0xfff0 to 0xfffd** (`fd ff` among them) are errors: treat them as an error at that offset (recommendation). A length above 32,000 is **not** an error; it is only not in the fixtures, and a reader reads it like any length. An empty string is NULL. The LOB value area after a row (L3, L4) is not a column field: there 0 is an empty LOB and `fc ff` starts a long value. Session 9: in a direct-path LONG / LONG RAW field (R2) a u16 with bit 0x8000 is a piece length with "more follows" (0x809c, 0x9f19 seen), not a length; every other column field of the corpus has a u16 below 0x8000 (or `fe ff`, `ff ff`). A conventional-path LONG length from 0x8000 up is not in the fixtures (R-040): treat it as unknown (stop or warn), not as a plain length **Session 17 (R-040), narrowed**: in a LONG / LONG RAW column with flag 0 the u16 is 1 to 0xfff0 = a length (from 0x8000 too), `fd ff` = chunked value (R2), `fe ff` = NULL; 0 and 0xfff1 to 0xfffc there are not in the fixtures (stop). The sentence above "a conventional-path LONG length from 0x8000 up ... treat it as unknown" is withdrawn. Every other column field: unchanged **Session 19 (R-048)**: in a LONG / LONG RAW column with flag 0 the first u16 is 1 to 0xfff5 = a length, `fa ff` = the 4-byte-length form (R2 (b)), `fd ff` = chunked (R2 (c)), `fe ff` = NULL; **0, 0xfff6 to 0xfff9, 0xfffb and 0xfffc = not in the fixtures (stop)** **Session 20 (R-051), replaces the session 19 sentence**: in a LONG / LONG RAW column with flag 0 the first u16 is 1 to **0xfff9** = a length, `fa ff` = R2 (b), `fd ff` = R2 (c), `fe ff` = NULL; **0, 0xfffb and 0xfffc = not in the fixtures (stop)**. The sizes 0xfffb and 0xfffc would stand for (65,531, 65,532) are written `fa ff` + n | confirmed as absence for every column except a conventional LONG (243,303 column fields, 211 files, `tools/rowlen_stats.py`: `len0` 0, `high` 0, `len_8000_up` 0; the 12 conventional LONG fields from 0x8000 / `fd ff` all in `long3_exp_<v>` and decoded to the truth); what 0 or 0xfff1 to 0xfffc would mean: unknown; session 19: 243,593 column fields of 221 files, still 0 outside the conventional LONG / LONG RAW columns; inside them 0xfff1, 0xfff5 (lengths) and `fa ff` (6 fields, `long4`) seen; 0xfff2 to 0xfff4 assumed lengths; 0, 0xfff6 to 0xfff9, 0xfffb, 0xfffc: unknown; session 20: 243,905 column fields of 229 files, still 0 outside those columns; inside them 0xfff6 to 0xfff9 seen as lengths (`long5`, `longraw5`), `fa ff` in 16 fields; 0xfff2 to 0xfff4: hypothesis (lengths); 0, 0xfffb, 0xfffc as a first u16: unknown | all | 5.6 |

## 5. End of rows, end of data

| rule | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|
| E1 | `ff ff` where the first field of a row would start ends the rows of the table; one LF follows, then the next item. Directly after the path flag it means: no rows in the file | confirmed | all; `t_empty`, `multi_empty*` | 5.1, 5.4 |
| E2 | Direct path only (flag 1). Let p be the offset after the path flag. The table has no rows, there is **no** `ff ff`, and the LF at p is followed by the next item, when **both** tests of `exp.md` 5.4 hold: (a) "text follows": the byte at p is LF and the bytes at p + 1 start an item; the fixtures have only `TABLE "`, `ENDTABLE`, `METRICS`, `CREATE `, `ALTER `, `COMMENT `, `GRANT `, `ANALSTATS `, `CALTER ` there (6 cases: `METRICS`, `TABLE "` twice, `CREATE ` twice, `ANALSTATS `; session 9: 3 more, `TABLE "` after the empty LONG table, `SUBPARTITION "` and `ENDCOMPARTITION` after two empty subpartitions; a reader's list also holds `PARTITION "`, `COMPARTITION "`, `ENDPARTITION`, which can follow a part by T10; session 10, note A-19: this list and the list of E2a are **one** list: `TABLE "`, `ENDTABLE`, `METRICS`, `CREATE `, `ALTER `, `COMMENT ON `, `GRANT `, `ANALSTATS `, `CALTER TABLE`, `PARTITION "`, `COMPARTITION "`, `SUBPARTITION "`, `ENDCOMPARTITION`, `ENDPARTITION`); (b) "declared length": the u16 LE at p is neither `fe ff` nor `ff ff` and is larger than the maximum length in the descriptor of column 1 (`0a 41` = 16650 at the least). Seen on 18c only; 11g writes `ff ff` (E1) **Session 17 (R-017)**: the cause is the segment: on 18c an empty table **with** a segment gets `ff ff` (E1) on the direct path, one without gets none (E2), both in one file | confirmed for 18c (the two tests agree on every direct-path binary part of the 201 files: 9 empty on 18c); why (version, or a table without a segment): suggested (R-017; T_LONG_EMPTY has no `STORAGE` clause on 18c, like every other case); why: **confirmed for 18c** (session 17, `seg_exp_direct_18c`: no segment, no `ff ff`); 11g without a segment: not in the fixtures (R-049) | `t_empty`, `multi_empty*`, `own`, `stat`, `long_exp_direct_18c`, `part2_exp_direct_18c`, `seg_exp_direct_18c` | 5.4, 13.2 |
| E3 | Accept both E1 and E2 after a path flag of 1; do not decide by the version line. If exactly one of the two tests of E2 holds, the fixtures do not say what it is: stop with an error at p (recommendation) Session 17: one 18c file holds both forms (`seg_exp_direct_18c`), so the version line cannot predict them | suggested (follows from E1, E2); "not by version" confirmed by `seg_exp_direct_18c` | - | 5.4, 13.2 |
| E2a | (note A-7) If column 1 has descriptor length 0 (LONG, LONG RAW), test (b) of E2 cannot fail and says nothing, and E3 must not be read as "stop". Instead: "no rows" when test (a) holds **and** the line from p + 1 to the next LF is a complete item of a known kind (`TABLE "...`, `ENDTABLE`, `METRICS...`, `CREATE `, `ALTER `, `COMMENT ON `, `GRANT `, `ANALSTATS `, `CALTER TABLE`) whose field, if list K says one follows, is well-formed; otherwise read a row. Session 10 (note A-19): the known kinds are the one list of E2 (a), partition lines included (`PARTITION "`, `COMPARTITION "`, `SUBPARTITION "`, `ENDCOMPARTITION`, `ENDPARTITION`); the two lists differed for no reason in the files. A LONG value whose length has low byte 0x0a and whose bytes are such an item would be misread; that cannot be excluded, only made unlikely | suggested (recommendation). Session 9: `exp` writes LONG tables on the direct path but moves the LONG column last (T3), so column 1 has length 0 only in a table whose **only** column is LONG / LONG RAW (Oracle allows one LONG per table, assumed); the empty two-column LONG table of the fixtures is decided by E2 (`long_exp_direct_18c`@0x11ee2 `0a 54` > 10). The single-column case is not in the fixtures | `long_exp_direct_<v>` (two-column case) | 5.4, 11 |
| E4 | The data end with `EXIT` LF `EXIT` LF; zero bytes fill the file to a multiple of R | confirmed | all | 1, 7 |
| X2 | A table whose export failed (log: `EXP-00056` on the conventional path, `EXP-00008` with `ORA-00604` on the direct path, both followed by `ORA-00942`) looks like an empty table in the file: `TABLE`, `CREATE TABLE`, `INSERT INTO`, descriptors, m, path flag, then `ff ff` LF and no statements after it, on both paths. 3 files, all 11g, owner mode, a table whose Cyrillic name the client set could not write | unknown: no certain sign in the file | `ru_exp_us7ascii_11g` (`ff ff` @0x13746, log lines 18 to 20), `ru_exp_w1252_11g` (`ff ff` @0x13746, log lines 18 to 20), `ru_exp_direct_us7ascii_11g` (flag `01 00` @0x14cda, `ff ff` @0x14cdc, log lines 19 to 22) | 14.3 |

## 6. Character sets

| rule | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|
| C1 | VARCHAR2 (type 1, form 1) and CHAR (96, form 1) values are the bytes of the **database**, in the character set named at +4 of the column descriptor (873 AL32UTF8, 171 CL8MSWIN1251 in the fixtures; equal to the database set of H4), whatever the client set was. Lengths count bytes. CHAR with **byte** semantics is padded with 0x20 to its declared byte length (character semantics: C6). Values can hold any byte (LF, TAB, `ff ff`) | confirmed | `t_text` with 3 clients, `ru_tbl_*` with 4 | 2.1, 4.1, 6.3, 14.1 |
| C2 | Every line and field of the text region, identifiers included (table names, column names of the `INSERT INTO` line and of the `CREATE TABLE` line), is in the **client** set of H1. Characters the client set lacks were replaced when the file was written (`?`, or 0xbf): such names are damaged for good, the row data are not | confirmed | `own_exp_w1252`, `ru_exp_*` | 14.2, 14.3, 14.8 |
| C3 | NVARCHAR2 (type 1 with character set id 2000 and form 2): UTF-16 big-endian; the length counts bytes; the descriptor length is twice the declared character count; no padding | confirmed | `t_text`, `ru_tbl_*`, `nchar` | 4.1, 6.3, 14.1 |
| C4 | Never use header bytes 1-2 for row data, and never the descriptor id for text items | confirmed | `ru_exp_al32utf8_11g`, `own_exp_w1252` | 14.8 |
| C5 | NCHAR (type 96 with set id 2000 and form 2): UTF-16 big-endian, padded with `00 20` to the declared number of characters; descriptor length = 2 x declared characters (`60 00 0a 00 d0 07 02 00` for `NCHAR(5)`). Session 9: "characters" are UTF-16 **code units**: U+1F600 is the surrogate pair `d8 3d de 00` and counts 2, so `NCHAR(3)` holding `a` + U+1F600 is 6 bytes with no padding. Decode NCHAR, NVARCHAR2 (C3) and NCLOB (L5) as UTF-16 with surrogate pairs, not as UCS-2 | confirmed (2 columns, 2 versions; one non-BMP character) | `nchar`, `nchar2` | 4.1, 6.3 |
| C6 | Character semantics (type 1 or 96, form 1). **Descriptor length** = min(w x n, cap): w = 4 for set 873 (AL32UTF8), 1 for the single-byte sets; cap 4000 for VARCHAR2, 2000 for CHAR; it equals `data_length` (`VARCHAR2(10 CHAR)` 40, `VARCHAR2(1001 CHAR)` 4000, `CHAR(3 CHAR)` 12, `CHAR(501 CHAR)` 2000; `VARCHAR2(20 CHAR)` 20 in CL8MSWIN1251). **Values**: `CHAR(n CHAR)` is padded with 0x20 to n **characters** (code points: U+1F600 counts 1), so its byte length varies from n up to the cap (`61 62 20 20`; `61 f0 9f 98 80 20`; 506 bytes for 3 CJK characters + 497 spaces); `VARCHAR2(n CHAR)` is not padded. Read the length field; never assume n or w x n bytes. The descriptor never gives n: take it from `CREATE TABLE` (T9) | confirmed (session 9: 7 columns of `t_probe11` in 4 files against `truth_probe11.txt`; `nchar`, `nchar2`; `tools/collist.py` 57 columns of 201 files (30 AL32UTF8, 27 CL8MSWIN1251; 55 in session 9), 0 differ); **w = 1 for set 171: confirmed** (session 10, note A-23: `R_TEXT.VC` `VARCHAR2(20 CHAR)`, descriptor `01 00 14 00 ab 00 01 00` = 20 in every `ru_*` file, `truth_ru.txt` row 9 `Typ=1 Len=20` for 20 characters); w for the other single-byte sets (1, 178 as database sets): assumed | `t_probe11`, `nchar`, `nchar2`; `ru_tbl_*` | 4.1, 6.3 |
| C7 | Oracle's map of single-byte sets to Unicode: CL8MSWIN1251 and WE8MSWIN1252 map each of the 256 bytes to one character, namely the Windows code page, and for the bytes the code page leaves undefined (1251: 0x98; 1252: 0x81, 0x8D, 0x8F, 0x90, 0x9D) the C1 control of the same number (U+0098, U+0081, ...). A decoder that rejects those bytes differs from Oracle in exactly them. US7ASCII: 0x00 to 0x7F as themselves, **every byte above 0x7F becomes U+FFFD** in Oracle. Whether the WHATWG tables of `encoding_rs` equal the first statement in all 256 bytes is assumed, not tested here | confirmed (truth only, `truth_csmap.txt`, both databases; no row data with such bytes in the fixtures) | `truth_csmap.txt` (`tools/csmap_cmp.py`) | 14.11 |
| C8 | (session 9) **What a reader may check against the descriptor length.** For types 1, 23 and 96 the descriptor length is a true maximum (n, 2n, min(w x n, cap)): no value of the corpus is longer (243,007 column fields of 201 files, of which 223,085 non-NULL fields of types other than 8 / 24 compared; `tools/desc_len_check.py`: `EXCEEDS: 0`; session 11), so a longer value is an error in the file or a case outside the fixtures: **warn** (recommendation; the length field alone decides how many bytes to read). Do **not** check types 8 / 24 (descriptor 0), 112 / 113 (descriptor 72, the field is a locator of 102 to 1102 bytes). Do **not** expect equality: only CHAR with byte semantics and NCHAR of BMP text are padded to the descriptor length; character-semantics CHAR, VARCHAR2 and NVARCHAR2 are shorter | confirmed as absence; the reaction is a recommendation | all; `t_probe11`, `nchar2` | 6.3 |
| X3 | Mapping of a character set id to a name: only 1 (US7ASCII), 171 (CL8MSWIN1251), 178 (WE8MSWIN1252), 873 (AL32UTF8), 2000 (AL16UTF16) are in the fixtures. Any other id: report it and return bytes | unknown | `truth_charsets.txt`, `truth_ru.txt` | 2.1, 14.4 |
| X4 | (closed in session 9) `VARCHAR2(n CHAR)` in AL32UTF8: min(4n, 4000), now part of C6 | confirmed (R-034) | `t_probe11` | 4.1 |

## 7. Value bytes by type

The value bytes are what `DUMP(col, 16)` shows in the database; `exp` adds nothing but the length
field of R2.

| rule | type (code) | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|---|
| Y1 | NUMBER (2) | Oracle NUMBER bytes, 1 to 21; 0 is the single byte `80`. **To a decimal number** (b = value bytes, L = length): if L = 1 and b0 = `80`: 0. Bit 0x80 of b0 set = positive: E = b0 - 0xC1, digits d = b[i] - 1 (bytes `01` to `64`). Clear = negative: E = 0x3E - b0; drop a last byte `66` (it is absent only when L = 21); d = 101 - b[i] (bytes `02` to `65`). value = sign x sum of d[i] x 100^(E - i), i = 0 for the first digit byte. The first and last **base-100 digit** (digit byte) are never 0 (canonical: one byte string per value); a decimal digit 0 can stand at either end inside such a byte (`04` = 03). Limits seen: at most 20 digit bytes = 40 decimal digits; E from -65 (1E-130) to 62 (9.99..E+125). Int64 max = `ca 0a 17 22 49 04 45 37 4e 3b 08`; E >= 10 (b0 >= `cb`, or b0 <= `34` when negative) is beyond Int64. A column declared `NUMBER` holds values of **40** significant decimal digits (`t_num` row 31) and of 38 (row 30, pi, and seven more), and exponents of +125 / -130 in the fixtures. **Session 9 (R-032): a quotient or an average computed by SQL that does not terminate is stored with the full 20 digit bytes (Len 21), on 18c and 11g alike: 40 significant decimal digits, or 39 when the first digit byte is below 10 (`c1 02` + 18 x `43` + `44` = 5/3); "no value has 39" is corrected.** Such values are ordinary data, not rarities. Any byte string that breaks these checks (single byte other than `80`, digit byte out of range, leading or trailing zero digit, negative without `66` and shorter than 21): error at that offset | confirmed (86,080 NUMBER fields of 201 files decode and re-encode to the same bytes, `tools/num_decode.py` step 3 (session 10, A-24; 85,456 in the 199 files of session 9); 120 truth values exactly; the 14 R-024 vectors; the 6 computed values of R-032 on 2 versions) | `t_num`, `t_num2`, `t_num3`, `t_numtypes`, all | 6.1 |
| Y2 | DATE (12) | 7 bytes: century + 100, year in century + 100, month, day, hour + 1, minute + 1, second + 1; for years before 1 the first two bytes are 100 - century, 100 - year | confirmed | `t_date` | 6.2 |
| Y5 | TIMESTAMP (180) | 7 bytes in the DATE layout when the fraction is zero, otherwise 11: DATE layout + u32 BE nanoseconds **Session 17 (R-047)**: holds for every f from 0 to 9 (f = 1 to 5, 7, 8 added). The nanoseconds are what the database stored (Oracle rounds a longer literal at INSERT: t4 of `.123456789` = 123,500,000); a reader decodes, never rounds | confirmed | `t_ts`, `t_decl`, `tsp` (both paths, both versions, byte-identical fields) | 6.4 |
| Y6 | RAW (23) | the RAW bytes, unchanged | confirmed | `t_raw`, `t_bn1`, `t_bn2` | 6.5 |
| Y7 | TIMESTAMP WITH TIME ZONE (181) | 13 bytes: DATE layout **in UTC**, u32 BE nanoseconds, 2 zone bytes. If bit 0x80 of the first zone byte is clear: offset, hour + 20, minute + 60 | confirmed for offsets +05:30, +00:00, -08:00; a region zone (bit set): unknown | `t_tstz` | 6.5 |
| Y8 | TIMESTAMP WITH LOCAL TIME ZONE (231) | TIMESTAMP layout, no zone bytes; the same instant gives the same bytes whatever the session zone was **Session 19 (R-050)**: 7 bytes in the DATE layout when the stored fraction is zero, in every precision (also in a column whose descriptor says 11), otherwise 11 bytes (DATE layout + u32 BE nanoseconds), as Y5; the seconds and the fraction are the stored, database-rounded values; a reader decodes, never rounds | confirmed as bytes; 7-byte values: confirmed (f = 0, 3, 9; 2 versions, 2 paths); whether the time is UTC or the database time zone: unknown (both are +00:00 in every fixture, R-019; R-050 used session zone +00:00 throughout and adds nothing on that point) | `t_tstz`, `tsltz2` | 6.5 |
| Y9 | LONG (8), LONG RAW (24) | Descriptor 4 bytes, length 0 (T5). **Conventional path**: each value is one ordinary field: u16 LE length + bytes (LONG: text bytes; LONG RAW: the raw bytes), `fe ff` for NULL; nothing extra before the rows, after a row (`00 00` as always) or after the end marker. **Direct path (session 9)**: LONG tables are written on the direct path (flag 1; the log has no "conventional path" line for them); the LONG column stands **last** in `INSERT INTO` and the descriptors (T3); its value is a **piece chain** (R2): u16 n, (n & 0x7fff) bytes, more pieces while bit 0x8000 is set; NULL `fe ff`. The tables after it are unaffected (T8). **Character set (note A-8)**: the value bytes are the **database** bytes whatever the client set: decode LONG with the database set of H4, never with header bytes 1-2 (C4) **Session 17 (R-040)**: conventional path: up to 65,520 bytes one field with a u16 length (bit 0x8000 included), from 65,535 bytes `fd ff` + chunks + `00 00` (R2, R8); direct path: the chain for every size (100,000 seen). Values, V after them and the next table intact on both paths and versions | confirmed: conventional up to 32,000 bytes (session 7); direct: 3 tables, 4 chains of 32,000 bytes (156 + 4 x 7961) and 6 one-piece values, LONG and LONG RAW, 2 versions; character set: one non-ASCII value `d0 b4 d0 b6 20 43 61 66 c3 a9` with an AL32UTF8 and a US7ASCII client, 2 versions (AL32UTF8 database only). A value above 32,000 bytes (conventional: a length from 0x8000?; above 65,535?): unknown (R-040); piece sizes: never assume; session 17: values of 32,767 to 100,000 bytes confirmed on both paths (LONG, 2 versions); the switch between 65,521 and 65,535 and LONG RAW above 32,000: unknown (R-048); **session 19 (R-048)**: conventional LONG: one field at 65,521 and 65,525, `fa ff` + u16 n + `00 00` + n at 65,530 to 65,533, `fd ff` chunks from 65,534 (R2); conventional LONG RAW (18c): one field at 32,768 and 65,520, `fd ff` chunks at 65,535 and 100,000; direct path: chains for all (LONG RAW chains = LONG chains of the same size). The switch from one field to `fa ff` lies between 65,526 and 65,530 (unknown where; R-051); **session 20 (R-051)**: one field up to 65,529 (`f6 ff` to `f9 ff` seen), `fa ff` for 65,530 to 65,533, `fd ff` from 65,534 (every size from 65,525 to 65,535 in a fixture; 65,522 to 65,524 not); **LONG RAW = LONG** on 18c and 11g (one field `f1 ff`, `f5 ff`, `f9 ff`, `00 80`, `f0 ff`; `fa ff` at 65,530, 65,531, 65,533; `fd ff` at 65,534, 65,535, 100,000; direct-path chains with the LONG piece sizes; 11g = 18c byte for byte) | `long`, `long2` (3 files), `long_exp_direct_<v>`, `long3`, `long4`, `longraw4_exp(_direct)_18c`, `long5`, `longraw5` | 6.6 |
| Y10 | INTERVAL YEAR TO MONTH (182), INTERVAL DAY TO SECOND (183) | Ordinary fields; the bytes are `DUMP`. 182 (5 bytes): years = u32 BE b[0..4] - 0x80000000, months = b[4] - 60. 183 (11 bytes): days = u32 BE b[0..4] - 0x80000000, hours = b[4] - 60, minutes = b[5] - 60, seconds = b[6] - 60, nanoseconds = u32 BE b[7..11] - 0x80000000 (all signed). `TO_CHAR` text: `-` if any field is negative, else `+`; years / days with p digits (the leading precision of the `CREATE TABLE` text, T9); then `-MM`, or ` HH:MI:SS` and `.` + the first f digits of the 9-digit nanoseconds (f = fractional precision). `80 00 00 02 42` = `+0002-06`; `7f ff ff fd 3a` = `-0003-02`; `80 00 00 0a 48 5a 69 87 5b ca 00` = `+0010 12:30:45.123456`; `7f ff ff ff 3c 3c 3c 80 00 00 00` = `-0001 00:00:00.000000`. Session 9: **every field of a negative interval is negative** (`7f ff ff ff 3a 39 38 62 32 9b 00` = `-000000001 02:03:04.500000000`); the sign of the text is `-` if any field is negative (`80 00 00 00 3c 3c 3b 80 00 00 00` = `-00 00:00:01`); with f = 0 there is no `.` and no fraction; descriptor lengths 5 / 11 for every precision (YEAR(9), DAY(9) TO SECOND(9), DAY(2) TO SECOND(0)); extremes ±999999999 (`bb 9a c9 ff`, `44 65 36 01`) | confirmed (session 8: 3 values per column; session 9: 9 more, 2 versions; texts from `TO_CHAR`) | `misc`, `misc2` (`tools/misc_types.py`: 106 checks) | 6.7 |
| Y11 | ROWID (69), UROWID (208) | ROWID: the field is **23 bytes**: 5 bytes (`01 00 00 00 17` in both values; unknown) + the 18-character `ROWIDTOCHAR` text (`AAAR8lAAMAAAACHAAA`). Return the last 18 bytes as text; any other length or prefix: report it. UROWID: the field is the `DUMP` bytes; first byte `01` = physical: object u32 BE, relative file u16 BE, block u32 BE, row u16 BE; written in base 64 (`A-Z a-z 0-9 + /`, 6 + 3 + 6 + 3 characters, most significant first) they give the `ROWIDTOCHAR` text (the same text as the ROWID of the same row). Session 9: first byte `02` = **logical** UROWID (index-organised table): the field is the `DUMP` bytes (10 seen); text = `*` + base 64 (same alphabet, 3 bytes to 4 characters, most significant first) of the bytes after the first: `02 04 03 00 15 b3 02 c1 02 fe` = `*BAMAFbMCwQL+`. Any other first byte: report the bytes | confirmed: ROWID text and prefix `01 00 00 00 17` in 8 values (2 databases); physical UROWID text = `CAST(urid AS VARCHAR2)` in 6 values; logical UROWID 1 value per version (text rule; a tail not a multiple of 3 bytes: unknown); the meaning of the prefix and of the logical bytes: unknown | `misc`, `misc2` (`tools/misc_types.py`) | 6.7 |

BINARY_FLOAT and BINARY_DOUBLE do not occur: `exp` refuses such tables (`docs/scope.md`).

## 7a. LOB columns (session 8)

A table has a LOB column when one of its descriptors has type 112 or 113 (T5). The LOB data are
not in the column fields; they follow the row. Evidence: `lob_exp_<v>` and `lob_exp_direct_<v>` (18c,
11g; AL32UTF8) and the 7 R_CLOB files of `cp1251`; `tools/lob_check.py` (141 checks, 0 differ),
`tools/check_all.py` (`problems 0`; the tables after the LOB tables equal their truth). Session 9:
`lob3_exp_<v>` (11 sizes around the switch, non-BMP text; `tools/lob3_switch.py` 162 checks) and
`nchar2_exp_<v>` (non-BMP CLOB / NCLOB). Session 10: `ru_lob2_exp_cp1251_11g`, `ru_lob2_exp_us7ascii_11g` (CLOB,
NCLOB, BLOB in the CL8MSWIN1251 database, short and long; `tools/lob_ru_check.py` 77 checks, 0 differ).

| rule | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|
| L1 | Before the rows (T6): m = the column count; the path flag; then **m column entries**, one per column in column order: u8 n, n bytes, 2 bytes (`00 00` in the fixtures). n = 0 for a column that is not a LOB; for a LOB column n bytes of its name (in the fixtures equal to the `INSERT INTO` name). Verify m = column count and "n > 0 exactly for types 112 / 113", else stop. Example T_LOB (ID, C, B): `03 00` `00 00` `00 00 00` `01 43 00 00` `01 42 00 00` | confirmed as framing (6 tables, 25 entries, 2 versions, 2 databases); the 2 bytes after each entry: unknown; names longer than 255 bytes: impossible in Oracle (assumed) | `lob`, `lob3`, `nchar2`, `ru_tables_exp_us7ascii_11g`, `ru_lob2_*` | 14.6 |
| L2 | In a row, the field of a LOB column is NULL (`fe ff`) or a **locator**: an ordinary field (u16 length + bytes, 102 to 1102 bytes seen; on 11g it can hold a small value in-row). The locator is not the value: skip it. It differs between two exports of the same data | confirmed as a field; content: observed only, not needed | `lob` | 14.6 |
| L3 | Conventional path (every LOB table in the fixtures): after the last field of a row, u16 **k** = the number of LOB columns of this row that are not NULL; then **k LOB values**, one per such column, in column order. k = 0 is the `00 00` of R4. Verify k against the row, else stop | confirmed (81 rows, 21 sections, k = 0 to 3; `tools/rowlen_stats.py`) | `lob`, `lob3`, `nchar2`, `ru_*` | 14.6 |
| L4 | A LOB value is **short**: u16 n + n bytes, n from 0 (an empty LOB: `EMPTY_CLOB()`, `EMPTY_BLOB()`) to 6,000 seen; or **long**: `fc ff`, 8 bytes LE = total byte length T, then chunks u16 c + c bytes until their sum is T (no terminator; chunk sizes 16,120 / 8,060 on 18c, 16,264 / 8,132 on 11g, last chunk the rest: never assume a size). A chunk length of 0 or from 0xfff0, or chunks that overshoot T: error. NULL (no locator, not counted in k) and empty (locator, value of length 0) are different values | confirmed (80 short, 46 long values, 201 files; 2 versions). **Session 9: the switch** is "long iff the byte length is larger than one chunk": 18c CLOB 16,120 / BLOB 8,060 bytes, 11g 16,264 / 8,132; the largest short value is one full chunk; in the long form every chunk but the last is full. A reader does not need this (it accepts both forms); a writer of tests does. The first 2 bytes of T can be `fc ff` (T = 65,532): read T by position. Bytes 4 to 7 of T are 0 in every value (T up to 200,000): u64 or u32 + 4 bytes still unknown, read 8 bytes. **Session 10, CL8MSWIN1251 database (11g)**: CLOB of 20,000 characters long, T = 20,000 (bytes), chunks **8,132** + 8,132 + 3,736 (the CLOB chunk there equals the BLOB chunk); NCLOB of 20,000 characters long, T = 40,000, chunks **16,264** + 16,264 + 7,472 (the first long NCLOB: its chunk equals the UTF-16 CLOB chunk of 11g); BLOB 8,132 + 8,132 + 3,736; every 11g chunk is 8,132 storage units of 1 or 2 bytes (hypothesis). Where the short form ends for CLOB / NCLOB in the single-byte database: unknown (66 short, 20,000 long; R-046); a reader does not need it. 18c in a single-byte database: not in the fixtures | `lob`, `lob3`, `ru_lob2_*` | 14.6 |
| L5 | Value bytes: BLOB (113) = the bytes. NCLOB (112, set 2000, form 2) = UTF-16 BE. CLOB (112, form 1): in the AL32UTF8 database (descriptor set 873) **UTF-16 BE**, 2 bytes per character for every value of the fixtures, though the descriptor says 873; in the CL8MSWIN1251 database (set 171) the database bytes, 1 per character, with every client set, in both forms (session 10). A reader decodes a CLOB by the descriptor's set id: 171 = that set (confirmed, both forms); another single-byte id (1, 178) = that set (assumed); 873 = UTF-16 BE; any other id: report it and return bytes | confirmed for ASCII (AL32UTF8: 9, 3000, 100,000 characters = 18, 6000, 200,000 bytes, `truth_lob.txt`) and Cyrillic (CL8MSWIN1251, `truth_ru.txt`). **Session 9: non-ASCII and non-BMP text in AL32UTF8 confirmed**: CLOB and NCLOB are UTF-16 BE with surrogate pairs (`04 34 04 36 00 20 00 43 00 61 00 66 00 e9 00 20 00 61 d8 3d de 00`); `DBMS_LOB.GETLENGTH` counts UTF-16 code units (11 for 10 characters), so a reader's character count differs from Oracle's for non-BMP text. **Session 10 (R-038, note A-20): the single-byte database confirmed** for set 171 (CL8MSWIN1251, 11g): a CLOB is the database bytes in the short **and the long** form (66 bytes `42 00 c0 c1 ..`; 20,000 bytes `fc ff` + T 20,000 + chunks; decoded with set 171 = the SQL text), an NCLOB is UTF-16 BE in both forms (132 bytes; 40,000 bytes), a BLOB the bytes; the bytes are identical with a CL8MSWIN1251 and a US7ASCII client. So "a single-byte set = that set" holds for **both forms** for set **171**; for 1 and 178 as **database** sets (no such database in the fixtures) it is assumed. Around the switch in the single-byte database: unknown (R-046) | `lob`, `lob3`, `nchar2`, `ru_*`, `ru_lob2_*` | 14.6, 6.3 |
| L6 | `exp` writes a LOB table on the conventional path even when the file was made with `direct=y` (flag 0; the log says "Table ... will be exported in conventional path."); the other tables of the file keep flag 1. Nothing after a LOB table is affected (T8): the table section ends at the `ff ff` LF after the last row's LOB values | confirmed (6 LOB tables in 4 `direct=y` files, 2 versions; session 10: R_LOB2 flag 0, R_TEXT after it = `ru_tbl_exp_*`) | `lob/direct`, `ru_exp_direct_*` | 8, 14.6 |
| X5 | A LOB table with path flag 1, m other than 0 or the column count, a LOB type with an 8-byte descriptor other than 112, or BFILE (114): not in the fixtures; stop. (Session 10: a long CLOB, an NCLOB and a BLOB of the CL8MSWIN1251 database are no longer in this row: L4, L5) | unknown | - | 14.6 |

**Consequence for ADR-12 (for the Architect).** A LOB column no longer has to stop the file. Types
112 (CLOB, NCLOB) and 113 (BLOB) have a known descriptor size, the bytes between descriptors and
rows are known (m, flag, m column entries), and the row layout is known (locator fields, then k
values in a short or a long form), so a reader can locate the end of a LOB table exactly and read
every table after it; the confirmed status rests on 3 tables, 2 versions, 2 database sets, values
up to 200,000 bytes. The reader has two honest choices per LOB table: **read** the values (L5
decides the bytes: BLOB bytes, NCLOB UTF-16 BE, CLOB by the set id) or **skip** the table (or only
its LOB columns) while still walking L1 to L4, which needs nothing beyond u16 / 8-byte lengths. A
reader that skips without walking cannot find the next table: there is no table length. Errors
remain errors: a k that does not match the row, a chunk sum that does not reach T, an m that is
neither 0 nor the column count. What still stops the file is a type code outside T5 (BFILE,
object types, ...). ADR-12's "exactly the 7 CLOB files stop" becomes "no file of the corpus stops".

## 8. Statistics items (skip only)

| rule | statement | status | fixtures | `exp.md` |
|---|---|---|---|---|
| S1 | A line that starts with `ANALSTATS ` is followed by exactly one field: in pieces if the two bytes after its LF are `fd ff`, otherwise plain. Lines with identical text are followed by either form | confirmed | `stat`, `stc`, `ru_stat_*`, `ddl`, `own`, `part2` (522 fields of 201 files, session 11) | 4.2, 4.6 |
| S2 | A line that starts with `ANALCOMPUTE ` is a complete item; no field follows | confirmed | the same (122 lines of 201 files, session 11) | 4.6, 14.5 |
| S7 | The number of statistics items of a table says nothing about its columns: 0 or 1 table item, and one column item per column **that has statistics** (0 to n), in ascending column order, the column named only inside the field. A table can have a table item and no column item, or no statistics item at all. The statistics can change in the database between two exports of the same table (session 7: an automatic job, suggested), so two files of the same tables can differ in their items. Never count them; never expect them | confirmed | `stc` (5 tables, 2 versions, 2 paths), `stc_exp_us7ascii` against `truth_stc_extra.txt`, `truth_stc.txt` | 4.6 |
| S4 | The fields are ASCII PL/SQL text (`DBMS_STATS.SET_TABLE_STATS`, `SET_COLUMN_STATS`, `SET_INDEX_STATS`) and hold no table data. A reader of data does not decode them | confirmed | `stat`, `stc`, `ru_stat_*` | 4.6, 14.10 |
| S6 | A file made with `statistics=none` has none of these items, whatever statistics the database holds; a table without statistics has none in any file | confirmed | `stat_exp_none`, `stc_exp_none` (`truth_stc_extra.txt` lists statistics for 17 columns), `stc` (T_STC4) | 4.6 |

## 9. Reading procedure

1. Read the header (H1 to H5): client set, R, database set. Seek to R + 0x104; require `METRICST`
   or `METRICSU` (H6).
2. Loop over items: read a line (I2). If it matches list K, read its one field in a form the
   entry allows (I3, I4, I5; `ANALSTATS `: S1). If it does not match, apply G1.
3. `TABLE "name"` followed by `CREATE TABLE` opens a table section (T1). Keep the `CREATE TABLE`
   line if declared types are wanted and parse its column list by T9 (text in the client set; the
   descriptors are enough to decode values, except the interval precisions of Y10). A line
   `PARTITION "p"` inside the section names the partition of the next binary part; `COMPARTITION
   "p"` / `SUBPARTITION "s"` name the partition and subpartition (T10).
4. On an `INSERT INTO` line (there can be several in one section, T10): take the column names **in
   this line's order** (T3; it can differ from `CREATE TABLE`);
   read column count, descriptors, m and the path flag (T4 to T6; X1 on an unknown type code); if
   m > 0, read the m column entries (L1).
5. Rows: at the start of each row, `ff ff` ends the table (E1); with flag 1, the two tests of E2
   decide "no rows" (E2a when column 1 has length 0). Otherwise read one field per column (R2, R3,
   R8; a LOB column holds a locator, L2; with flag 1 a LONG / LONG RAW column is a piece chain, R2; with flag 0 a LONG / LONG RAW field is one field (first u16 up to 0xfff9), `fa ff` + 4-byte length + bytes, or `fd ff` + chunks + `00 00`, R2, R8); with flag 0 consume the u16 after the row: `00 00`, or in
   a LOB table k followed by k LOB values (R4, L3, L4). After `ff ff` consume one LF and go to step 2.
6. Stop at the second of two consecutive `EXIT` lines (E4). Reaching the zero fill or the end of
   the file before that is an error (a file cut short: unknown what it looks like).
7. Decode values by the descriptor type (chapter 7, 7a) and character set (chapter 6).

## 10. What a reader must not do

| statement | status | `exp.md` |
|---|---|---|
| Use `METRICET` / `METRICEU` numbers, or `num_rows` of a statistics item, as a row count | confirmed that they are not | 7.1, 4.6 |
| Count `TABLE` / `ENDTABLE` as pairs | confirmed | 7.1 |
| Assume RECORDLENGTH 8192, or a fixed offset for anything in the header after the first text line | confirmed | 1.1, 14.7 |
| Split the text region on LF only, or search for keywords in the byte stream | confirmed | 4.4 |
| Split the column list of `CREATE TABLE` on every comma, or take a name to the next comma | confirmed (a quoted name can hold `,` and `(`; `NUMBER(p, s)` holds `, `) | 4.7 |
| Expect CHAR values of a fixed byte length | confirmed (character semantics, C6) | 6.3 |
| Rely on the place, order or number of the items after the rows (indexes, grants, constraints, statistics, comments) | suggested: the order is observed on few tables | 4.6, 11 (S3) |
| Decide "empty table on the direct path" by the version line | suggested | 13.2 |
| Assume one `INSERT INTO` line (one binary part) per table section | confirmed that a partitioned table has one per partition, a composite-partitioned table one per subpartition (T10) | 4.8 |
| Take the column order of the rows from `CREATE TABLE` | confirmed wrong for LONG tables on the direct path (T3) | 4.1 |
| Read a direct-path LONG / LONG RAW field as one u16 + bytes | confirmed wrong (R2: a piece chain) | 6.6 |
| Read a conventional-path LONG length from 0x8000 as a piece chain, or `f0 ff` / `fd ff` in a conventional LONG column as an error | confirmed wrong (session 17, R2, R8: a plain length up to 0xfff0; `fd ff` = chunks) | 6.6, 5.6 |
| Read `fa ff` in a conventional LONG / LONG RAW column as a length of 65,530 | confirmed wrong (session 19, R2 (b): a marker; the length follows in 4 bytes) | 6.6, 5.6 |
| Treat a first u16 0xfff6 to 0xfff9 in a conventional LONG / LONG RAW column as an error or a marker, or expect LONG RAW to be framed differently from LONG | confirmed wrong (session 20, R2 (a): lengths; LONG RAW = LONG, 18c and 11g) | 6.6, 5.6 |
| Expect a TSLTZ value of the descriptor's length (11) | confirmed wrong (session 19, Y8: 7 bytes when the fraction is zero) | 6.5 |
| Assume `ENDPARTITION` directly follows the last binary part | confirmed wrong with statistics (T10) | 4.8 |
| Skip a LOB table by searching for the next `TABLE "` text, or take a LOB column's field (the locator) for its value | confirmed (no table length exists; the values follow the row, L2 to L4) | 14.6 |
| Treat `fc ff` or a length 0 in the LOB value area as an error, or treat a column-field length above 32,000 as an error | confirmed (L4, R8) | 5.6, 14.6 |
| Trust any rule here for a file whose version line is neither `EXPORT:V18.00.00` nor `EXPORT:V11.02.00`: read it, but report the version as untested | unknown (R-022) | 9, "Limit of confirmed" |

## 11. Open points that can change these rules

| point | request | affects |
|---|---|---|
| `exp` client older than the database (10.2 against 11.2.0.4) | R-022 | everything: no fixture |
| Plain field of 8192 bytes or more; view text in several pieces; functions, packages, types | R-018 | I5, I6, G1, G2 |
| ~~Empty table with a segment on the 18c direct path~~ answered (session 17): with a segment `ff ff`, without none; ~~11g without a segment~~ not possible in 11g XE (session 19, ORA-00439); 11g Enterprise Edition: unknown, no container | R-017 (generated and used); R-049 (generated and used, result negative) | E2 (not changed) |
| Region time zones; database time zone other than +00:00 (R-050 did not touch it: session zone +00:00 throughout) | R-019 | Y7, Y8 |
| ~~`VARCHAR2(n CHAR)` in a multi-byte database~~ answered (session 9) | R-034 | C6 |
| Statistics field with a piece of 8192 bytes or more | R-021 | I4 (skip only) |
| ~~LOB columns: descriptor, bytes before the rows, chunks~~ answered (session 8) | R-027 | L1 to L6 |
| ~~LOB: where the long form starts; non-ASCII / non-BMP CLOB text~~ answered (session 9); ~~a long CLOB, NCLOB and BLOB in a single-byte database~~ answered for CL8MSWIN1251 (session 10); a CLOB / NCLOB **around the switch** in a single-byte database | R-035; R-038 (generated); **R-046** | L4 (L5 not affected) |
| A single-byte database on 18c; a database set US7ASCII or WE8MSWIN1252 with LOB columns | none written (owner: only if Q-4 shows such a database) | L4, L5 |
| ~~Oracle refuses a partitioned table with a LONG / LONG RAW column~~ answered by record (session 11): ORA-14085 for LONG, LONG RAW and `ALTER ... ADD (l LONG)` on 18c (`18c/truth_r045.txt` lines 10 to 28); no partitioned LONG fixture can exist on 18c | R-045 (generated and used, result negative) | T3, T10 (design only) |
| ~~LONG on the direct path, non-ASCII LONG~~ answered (session 9); ~~LONG above 32,000 bytes~~ answered (session 17: R2, R8, Y9); ~~the switch from one field to `fd ff` between 65,521 and 65,535 bytes; LONG RAW above 32,000 bytes~~ answered (session 19: three forms; LONG RAW on 18c); ~~the exact switch to `fa ff` (65,526 to 65,529, 65,531), LONG RAW in the `fa ff` sizes, LONG RAW above 32,000 bytes on 11g~~ answered (session 20: switch between 65,529 and 65,530; LONG RAW = LONG on 18c and 11g); the sizes 65,522 to 65,524 (first u16 0xfff2 to 0xfff4) and LONG RAW 65,526 to 65,528, 65,532; a single-column LONG table on the direct path (E2a) | R-031; R-040, R-048, R-051 (generated and used); **R-052** (proposed, very low); R-041 | Y9, R2, R8, E2a |
| ~~INTERVAL, ROWID, UROWID; mixed negative intervals, other precisions, more ROWIDs, a logical UROWID~~ answered (sessions 8, 9); the meaning of the ROWID prefix and of the logical UROWID bytes | R-029, R-036 | Y10, Y11 |
| ~~Partitioned tables; an empty partition, subpartitions, statistics~~ answered for 2 tables (sessions 8, 9); other partitioning methods, owner mode, indexes | R-030, R-037 | T10 |
| ~~Computed NUMBER values (38 or 40 digits)~~ answered (session 9): 39 / 40 | R-032 | Y1 (evidence for ADR-14) |
