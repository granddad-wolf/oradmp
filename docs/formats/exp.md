# Legacy EXP dump layout

State: Analyst session 19 (Oracle 18c and 11g, table mode and owner mode, AL32UTF8 and
CL8MSWIN1251 databases). Session 1 covered row framing of `t_frame`.
Session 2 added: character set ids, RECORDLENGTH, record boundaries, several tables in one file,
an empty table, `METRICET`, length fields above 255, NULLs of NUMBER / DATE columns, NUMBER and
DATE value bytes, byte-or-character lengths. Session 3 adds: text exported with another client
character set, empty tables between other tables, the statements around the table data (`ddl`),
RAW against a truth file, the fields of TIMESTAMP WITH TIME ZONE, and "Rules for a reader".
Session 4 adds: Oracle 11g with its own offsets (section 13), owner mode (4.5), a record boundary
inside a length field and inside `fe ff` (1.2), a database with another character set
(section 14), and "Rules for a reader" version 2 (section 11).
Session 5 adds: optimizer statistics (`ANALSTATS T?`, `ANALSTATS C?`, `ANALCOMPUTE T?`: section
4.6), their differences between 18c and 11g (13.9), text in statistics and comments written with a
US7ASCII client (14.10), and "Rules for a reader" version 3 with the list of lines that introduce
a field (section 11).
Session 6 adds: tables in which not every column has statistics (R-020; 4.6, "Columns without
statistics"), the counts of the real dump reconciled (4.6), the place of `GRANT` lines relative to
the statistics items, where the date text of the header stands (section 2), "Rules for a reader"
version 3.1 (section 11) and the extract `exp_reader_rules.md` for the Architect.
Session 7 adds: the conversion of NUMBER bytes to a decimal number as a rule, with boundary values
(R-024; 6.1), the column list of `CREATE TABLE` as a grammar (R-025; new 4.7), NCHAR and `CHAR(n
CHAR)` (R-026a; 4.1, 6.3), Oracle's byte-to-Unicode map of three single-byte sets (R-026b; new
14.11), LONG and LONG RAW (R-028; 4.1, new 6.6), the R-020 tables with a US7ASCII client and with
`statistics=none` (R-023; 4.6), the length values that never occur in row data (new 5.6), and
"Rules for a reader" version 3.2.
Session 8 adds: LOB columns CLOB, NCLOB and BLOB in an AL32UTF8 database on both versions (R-027;
4.1, 14.6 rewritten: the bytes before the rows, the locator fields, the LOB values after each row,
values above 65,535 bytes), INTERVAL YEAR TO MONTH, INTERVAL DAY TO SECOND, ROWID and UROWID (R-029;
4.1, new 6.7), a range-partitioned table (R-030, 18c; new 4.8), answers to the Architect's notes A-6
to A-10, and "Rules for a reader" version 3.3.
Session 9 adds: NUMBER values computed by SQL (R-032; 6.1), non-BMP characters in NCHAR, CHAR(n CHAR),
VARCHAR2(n CHAR), CLOB and NCLOB (R-033; 6.3, 14.6), the descriptor length of character-semantics
columns with the byte caps (R-034; 4.1, 6.3), LOB values around the short / long switch and
non-ASCII CLOB text (R-035; 14.6), intervals with mixed signs and other precisions, more ROWIDs and a
logical UROWID (R-036; 6.7), LONG on the direct path, a non-ASCII LONG and LONG with a US7ASCII
client (R-031; 6.6, 5.4), a composite (range-hash) partitioned table with an empty partition and
partition statistics (R-037, 18c; 4.8), and "Rules for a reader" version 3.4.
Session 10 adds: CLOB, NCLOB and BLOB values in the CL8MSWIN1251 database, short and long, with a
CL8MSWIN1251 and a US7ASCII client (R-038; 14.6), answers to the Architect's notes A-18 to A-24 (6.6,
5.4, 6.3, 14.6, 12), and "Rules for a reader" version 3.5.
Session 11 (housekeeping) adds: the R-045 record (`18c/truth_r045.txt`: Oracle 18c refuses a partitioned table with a
LONG or LONG RAW column, ORA-14085), stale counts corrected in rules T9, C6, C8, S1, S2, the third failed export in rule
X2 of the extract, and "Rules for a reader" version 3.6 (counts re-derived, no rule changed in substance).
Session 17 adds: TIMESTAMP(1) to (5), (7) and (8) (R-047; 6.4), LONG values of 32,767 to 100,000 bytes on both paths
(R-040; 6.6, 5.6), empty tables with and without a segment on the 18c direct path (R-017; 5.4, 13.2), and "Rules for a
reader" version 3.7 (T5, Y5, R2, R8, Y9, E2, E3 changed; section 11).
Session 19 adds: conventional LONG values of 65,521 to 65,535 bytes, which show a fourth field form `fa ff` + 4-byte
length (R-048; 6.6, 5.6), LONG RAW of 32,768 to 100,000 bytes on 18c (R-048; 6.6), TIMESTAMP WITH LOCAL TIME ZONE with
f = 0, 3, 9 (R-050; 6.5), the negative answer of R-049 (11g XE cannot create a table without a segment; 5.4, 13.2), and
"Rules for a reader" version 3.8 (T5, R2, R8, Y8, Y9 changed; section 11).
Session 20 adds: conventional LONG values of 65,526 to 65,531 bytes (the switch from one field to `fa ff` lies between
65,529 and 65,530), LONG RAW of 65,521 to 65,534 bytes on 18c and 11g and LONG RAW of 32,768 to 100,000 bytes on 11g
(R-051; 6.6, 5.6: LONG RAW = LONG in every form, 11g = 18c), and "Rules for a reader" version 3.9 (R2, R8, Y9 changed;
section 11).

Everything here comes from the fixtures in the tables below, their logs and the `truth_*.txt`
files. Sections 1 to 9 were written for Oracle 18c and keep their 18c offsets; what holds for 11g
is said in section 13 and in the `11g` column of section 11.

## Changes in session 20

### Fixtures added in session 20

Made by the owner with `corpus/scripts/make_pending3.sh` on 2026-10-25 (read, not run); uncommitted at the time of this
session. Legacy `exp`, table mode, `statistics=none`, database AL32UTF8, national AL16UTF16, client AL32UTF8 (logs:
"Export done in AL32UTF8 character set and AL16UTF16 NCHAR character set"). 18c: "Version 18.4.0.0.0"; 11g: "Oracle
Database 11g Express Edition Release 11.2.0.2.0 - 64bit Production". `sha256sum -c SHA256SUMS` in `18c/` and `11g/`: all OK.

| File | Oracle | Path | Request | Tables, rows (log) | SHA-256 prefix |
|---|---|---|---|---|---|
| `18c/long5_exp_18c.dmp` | 18c | conventional | R-051 (a) | T_LONG5 6, T_FRAME 7 | `17f6d4154f92` |
| `18c/long5_exp_direct_18c.dmp` | 18c | direct | R-051 (a) | T_LONG5 6, T_FRAME 7 | `0d070d8f208e` |
| `11g/long5_exp_11g.dmp` | 11g | conventional | R-051 (a) | T_LONG5 6, T_FRAME 7 | `640ed48e4954` |
| `11g/long5_exp_direct_11g.dmp` | 11g | direct | R-051 (a) | T_LONG5 6, T_FRAME 7 | `b4440ba2b9b1` |
| `18c/longraw5_exp_18c.dmp` | 18c | conventional | R-051 (b) | T_LONGRAW5 7, T_FRAME 7 | `2625c036c5bf` |
| `18c/longraw5_exp_direct_18c.dmp` | 18c | direct | R-051 (b) | T_LONGRAW5 7, T_FRAME 7 | `719eb81947ba` |
| `11g/longraw5_exp_11g.dmp` | 11g | conventional | R-051 (c) | T_LONGRAW5 11, T_FRAME 7 | `1d70b7605d66` |
| `11g/longraw5_exp_direct_11g.dmp` | 11g | direct | R-051 (c) | T_LONGRAW5 11, T_FRAME 7 | `71fe56dac457` |
| `18c/truth_long5.txt`, `11g/truth_long5.txt` (identical) | both | - | R-051 (a) | id, `GETLENGTH` (65,526 to 65,531), first and last 20 bytes, V | `da8f5040295e` |
| `18c/truth_longraw5.txt` | 18c | - | R-051 (b) | id, `GETLENGTH` (65,521, 65,525, 65,529, 65,530, 65,531, 65,533, 65,534), first and last 10 bytes | `333fb7a97ff1` |
| `11g/truth_longraw5.txt` | 11g | - | R-051 (c) | the 7 rows of 18c plus ids 8 to 11 (32,768, 65,520, 65,535, 100,000) | `3a7facf06546` |

Logs of the loaders: `long5_sqlldr_<v>.log` `6 Rows successfully loaded.` (18c line 33, 11g line 32);
`longraw5_sqlldr_11g.log` (hex text in `CHAR(200000)`): 11 x `ORA-01461`, `0 Rows successfully loaded.` (line 64);
`longraw5_sqlldr_varraw_11g.log` (`LONG VARRAW`, `"var 7"`): `11 Rows successfully loaded.` (line 32). The 18c LONG RAW rows
were bound by python-oracledb (no log in the fixture folder). The 18c and the 11g file of each `long5` pair differ only in
22 header bytes below 0xa0 (`cmp -l`); from 0x2000 on they are byte-identical. The `longraw5` pairs are byte-identical from
0x2000 to the end of row 7 (6.6 session 20). Owner-specific text: none in the new logs and truth files (paths are the
container's `/work/...`; no `/home/`, e-mail address or password).

### What changed

| section | change | evidence |
|---|---|---|
| 6.6, 5.6 | conventional LONG: one field `f6 ff` to `f9 ff` (65,526 to 65,529), `fa ff` + `fa ff 00 00` / `fb ff 00 00` (65,530, 65,531): the switch lies between 65,529 and 65,530; the session 19 hypothesis "one field up to 0xfff9, `fa ff` for 0xfffa to 0xfffd, chunks from 0xfffe" holds for every size in a fixture | `long5_exp_<v>`@0x2271, 0x12277, 0x2227e, 0x32286, 0x4228f, 0x5229d; `tools/long5_check.py` |
| 6.6 | conventional LONG RAW = LONG: `f1 ff`, `f5 ff`, `f9 ff` one field; `fa ff` at 65,530, 65,531, 65,533; `fd ff` chunks at 65,534; direct path chains with the LONG piece sizes; 18c and 11g | `longraw5_exp_<v>`@0x225d, 0x12256, 0x22253, 0x32254, 0x4225a, 0x52261, 0x6226a |
| 6.6, 13 | LONG RAW on 11g (32,768 to 100,000 bytes): every field byte-identical to 18c (`longraw5_exp_18c` ids 1 to 7, `longraw4_exp_18c` for ids 8 to 11), both paths | `longraw5_exp(_direct)_11g`; `tools/long5_check.py` (`cross_11g`) |
| 6.6 | the complete table of conventional sizes from 32,768 to 65,535 in the fixtures, and the list of first u16 values seen / never seen | 6.6 session 20 |
| 11 | rules R2, R8, Y9: version 3.9 | the rows above |
| 12 | new tool `long5_check.py` | 12 |

Counts (`tools/rowlen_stats.py`, `tools/corpus_counts.py`, session 20): **229** EXP files (105 / 97 / 27), 450 binary
parts, 41,494 rows, **243,905** column fields, 20,084 NULL; `problems 0`.

Session 20, the tools on the 8 new files, without any change to an existing tool: `tools/exp_parse.py` walks all 8 with
"problems: none" (the session 19 reading of `fa ff`, and every other u16 below 0xfffd read as a length, cover them).
`tools/check_all.py`: 105 / 97 / 27 files walked, `problems 0`, `rule_diff 0`, `empty_rule_diff 0`, `bad 0`; `no truth` for
T_LONG5 and T_LONGRAW5 (checked by `long5_check.py` instead); T_FRAME `ok` in the 8 files. `tools/corpus_counts.py` TOTAL:
229 files, `problems 0`, 41,494 rows, 243,905 row fields, 20,084 NULL. `tools/rowlen_stats.py`: `len0` 0, `high` 0,
`len_8000_up` 0, `long_conv_8000_up` 28, `long_conv_fa` 16, `long_conv_chunked` 16, 72 direct-path chains (66 with more
than one piece); piece u16 values seen: none from 0xfff0, no piece of length 0. `tools/frame_t8.py`: 58 identical, 0
different. `tools/boundary.py -q`: 48 record boundaries in each `long5` file, 56 in each 18c `longraw5` file, 88 in each
11g `longraw5` file, all inside a value. `tools/long3_check.py`, `tools/long4_check.py`: differences 0 (unchanged).
`tools/long5_check.py`: differences 0 (8 files and the 22 field comparisons of `cross_11g`).
`python/exp_survey.py` (the owner's survey, v3.4): stops on all 8 files at the descriptor of the LONG / LONG RAW column
("FATAL: unknown column type code 8" / "24", e.g. `long5_exp_18c.dmp`@0x225d, `longraw5_exp_18c.dmp`@0x2251): v3.4 does
not know codes 8 and 24 (rule T5 v3.2). So it cannot report LONG lengths for Q-4 as it stands (observed on these
fixtures; the file is the owner's, not changed here).

## Changes in session 19

### Fixtures added in session 19

Made by the owner with `corpus/scripts/make_pending2.sh` on 2026-10-25 (read, not run); uncommitted at the time of this
session. Legacy `exp`, table mode, `statistics=none`, database AL32UTF8, national AL16UTF16, client AL32UTF8 (logs:
"Export done in AL32UTF8 character set and AL16UTF16 NCHAR character set"). 18c: Oracle 18c XE "Version 18.4.0.0.0";
11g: "Oracle Database 11g Express Edition Release 11.2.0.2.0". `sha256sum -c SHA256SUMS` in `18c/` and `11g/`: all OK.

| File | Oracle | Path | Request | Tables, rows (log) | SHA-256 prefix |
|---|---|---|---|---|---|
| `18c/long4_exp_18c.dmp` | 18c | conventional | R-048 | T_LONG4 7, T_FRAME 7 | `6472d61c0c57` |
| `18c/long4_exp_direct_18c.dmp` | 18c | direct | R-048 | T_LONG4 7, T_FRAME 7 | `b771ab217a47` |
| `11g/long4_exp_11g.dmp` | 11g | conventional | R-048 | T_LONG4 7, T_FRAME 7 | `3936ec2925a4` |
| `11g/long4_exp_direct_11g.dmp` | 11g | direct | R-048 | T_LONG4 7, T_FRAME 7 | `ed1d597a0acc` |
| `18c/longraw4_exp_18c.dmp` | 18c | conventional | R-048 | T_LONGRAW4 4, T_FRAME 7 | `872e2dfbff0c` |
| `18c/longraw4_exp_direct_18c.dmp` | 18c | direct | R-048 | T_LONGRAW4 4, T_FRAME 7 | `67b7b4dc6c87` |
| `18c/tsltz2_exp_18c.dmp` | 18c | conventional | R-050 | T_TSLTZ2 4 | `bdd59f6dd50c` |
| `18c/tsltz2_exp_direct_18c.dmp` | 18c | direct | R-050 | T_TSLTZ2 4 | `55f882cc4668` |
| `11g/tsltz2_exp_11g.dmp` | 11g | conventional | R-050 | T_TSLTZ2 4 | `e584c80da091` |
| `11g/tsltz2_exp_direct_11g.dmp` | 11g | direct | R-050 | T_TSLTZ2 4 | `8bd7e9501f84` |
| `18c/truth_long4.txt`, `11g/truth_long4.txt` (identical) | both | - | R-048 | id, `GETLENGTH` (65,521, 65,525, 65,530, 65,532, 65,533, 65,534, 65,535), first and last 20 bytes, V | `ec5cf8ea817c` |
| `18c/truth_longraw4.txt` | 18c | - | R-048 | id, `GETLENGTH` (32,768, 65,520, 65,535, 100,000), first and last **10** bytes (all `AB`) | `c8c15641f001` |
| `18c/truth_tsltz2.txt`, `11g/truth_tsltz2.txt` (identical) | both | - | R-050 | `+00:00\|+00:00` (dbtimezone, sessiontimezone), `user_tab_columns` (data_length 7, 11, 11), decimal `DUMP` of 4 rows, `TO_CHAR ... FF9` | `7fd3f0804828` |
| `11g/truth_r049.txt` | 11g | - | R-049 | SQL*Plus record: `ORA-00439: feature not enabled: Deferred Segment Creation` (lines 28 to 32); no dump | `e221cbec2fac` |

The SQL*Loader logs `long4_sqlldr_<v>.log` say `7 Rows successfully loaded.` (18c line 33, 11g line 32). The 18c and the
11g file of each `long4` and `tsltz2` pair differ only in the header (`cmp -l`: 6 or 7 bytes, all below offset 0xa0:
date and version text); from 0x2000 on they are byte-identical. The LONG RAW files exist for 18c only (python-oracledb
thin mode needs Oracle 12.1 or later; owner's note).

### What changed

| section | change | evidence |
|---|---|---|
| 6.6, 5.6 | conventional LONG of 65,521 to 65,535 bytes: one field `f1 ff` (65,521) and `f5 ff` (65,525); a **fourth form** `fa ff` + 4 bytes (u16 n, `00 00`) + n bytes for 65,530, 65,532, 65,533; `fd ff` + chunks from **65,534** (was: from 65,535); the direct path a chain for all | `long4_exp_<v>`@0x2271, 0x12272, 0x22277, 0x32285, 0x42295, 0x522a6, 0x622d8; `tools/long4_check.py` |
| 6.6 | conventional LONG RAW (18c): one field `00 80` (32,768) and `f0 ff` (65,520), `fd ff` + 4000-byte chunks for 65,535 and 100,000: the LONG forms; direct path: chains | `longraw4_exp_18c`@0x225d, 0xa265, 0x1a25d, 0x2a288; `longraw4_exp_direct_18c`@0x225d |
| 6.5 | TSLTZ: descriptor `e7 00 07 00` for f = 0, `e7 00 0b 00` for f = 3, 9; values 7 bytes with a zero fraction in every precision, 11 otherwise; stored values rounded by the database (L0 of `.6` = second 6); same bytes on both paths and versions | `tsltz2_exp_<v>`@0x22cb to 0x22d3, 0x22df to 0x2356; `tools/tsltz2_check.py` |
| 5.4, 13.2 | R-049 negative: 11g XE refuses `SEGMENT CREATION DEFERRED` (ORA-00439); assumed from that: every 11g XE table gets its segment at `CREATE TABLE`, so "segment, not version" cannot be tested on 11g XE | `11g/truth_r049.txt` lines 28-32, 35-43 |
| 11 | rules T5, R2, R8, Y8, Y9: version 3.8 | the rows above |
| 12 | new tools `long4_check.py`, `tsltz2_check.py`; `exp_parse.py` reads the `fa ff` form (`long_fa`); `rowlen_stats.py`, `boundary.py` know it | 12 |

Counts (`tools/corpus_counts.py`, `tools/rowlen_stats.py`, session 19): **221** EXP files (101 / 93 / 27), 434 binary
parts, 41,378 rows, **243,593** column fields, 20,020 NULL; `problems 0` in `check_all.py` for every file.

## Changes in session 17

### Fixtures added in session 17

Made by the owner with `corpus/scripts/make_pending.sh` on 2026-10-25 (read, not run). Legacy `exp`, table mode,
database character set AL32UTF8, national set AL16UTF16, client AL32UTF8 (logs: "Export done in AL32UTF8 character set
and AL16UTF16 NCHAR character set"). 18c files: Oracle 18c XE, "Version 18.4.0.0.0"; 11g files: "Oracle Database 11g
Express Edition Release 11.2.0.2.0". `SHA256SUMS` of both folders verified in session 17 (`sha256sum -c`: all OK).

| File | Oracle | Path | Request | Tables, rows (log) | SHA-256 prefix |
|---|---|---|---|---|---|
| `18c/tsp_exp_18c.dmp` | 18c | conventional | R-047 | T_TSP 4 | `8c1392dc9eaf` |
| `18c/tsp_exp_direct_18c.dmp` | 18c | direct | R-047 | T_TSP 4 | `401739d2d0d8` |
| `11g/tsp_exp_11g.dmp` | 11g | conventional | R-047 | T_TSP 4 | `d06b6e4e5779` |
| `11g/tsp_exp_direct_11g.dmp` | 11g | direct | R-047 | T_TSP 4 | `7b6c1181ac32` |
| `18c/long3_exp_18c.dmp` | 18c | conventional | R-040 | T_LONG3 7, T_FRAME 7 | `96bede9b80f8` |
| `18c/long3_exp_direct_18c.dmp` | 18c | direct | R-040 | T_LONG3 7, T_FRAME 7 | `b14302d44964` |
| `11g/long3_exp_11g.dmp` | 11g | conventional | R-040 | T_LONG3 7, T_FRAME 7 | `a283109c491c` |
| `11g/long3_exp_direct_11g.dmp` | 11g | direct | R-040 | T_LONG3 7, T_FRAME 7 | `466494696545` |
| `18c/seg_exp_18c.dmp` | 18c | conventional | R-017 | T_SEG0 0, T_SEG1 0, T_SEG2 0 | `d1720911b43a` |
| `18c/seg_exp_direct_18c.dmp` | 18c | direct | R-017 | T_SEG0 0, T_SEG1 0, T_SEG2 0 | `c52d14d69154` |
| `18c/truth_tsp.txt`, `11g/truth_tsp.txt` (identical) | both | - | R-047 | `user_tab_columns` (data_length 11 for T1 to T8), decimal `DUMP` of 4 rows, `TO_CHAR ... FF9` of t1, t3, t8 | `29050816fb16` |
| `18c/truth_long3.txt`, `11g/truth_long3.txt` (identical) | both | - | R-040 | id, `GETLENGTH`, first and last 20 bytes, V of 7 rows | `a16cb98ddc2b` |
| `18c/truth_seg.txt` | 18c | - | R-017 | `T_SEG0\|segment_created=YES\|segments=1`, `T_SEG1\|...=YES\|segments=1`, `T_SEG2\|...=NO\|segments=0`, all `rows=0` | `520c0a0e9b9f` |

The SQL*Loader logs `long3_sqlldr_<v>.log` (R-040 route 1) say `7 Rows successfully loaded.` (18c line 33, 11g line
32) and `Maximum field length is 100000`. The 18c and the 11g file of each pair differ only in header bytes (version
text, date, file name): `cmp -l` 22 bytes for `tsp_exp`, `tsp_exp_direct` and `long3_exp`; `long3_exp_direct` differs
in 40 bytes, 18 of them inside the chain of T_LONG3 row 1 (6.6, session 17).

### What changed

| section | change | evidence |
|---|---|---|
| 6.4 | TIMESTAMP(f) for f = 1 to 5, 7, 8: descriptor length 11 for every f > 0; values 7 bytes (fraction 0) or 11; the values were rounded by Oracle at INSERT, `exp` copies the stored bytes; conventional = direct = 18c = 11g | `tsp_exp_<v>`@0x22fb to 0x2313; `tools/tsp_check.py` |
| 6.6, 5.6 | conventional LONG: one field with a u16 length up to 0xfff0 (65,520 seen), `fd ff` + 4000-byte chunks + `00 00` from 65,535; direct LONG: the piece chain for every size (100,000 seen) | `long3_exp_<v>`@0xa280, 0x2228f, 0x3228f; `long3_exp_direct_<v>`@0x522fb; `tools/long3_check.py` |
| 5.4, 13.2 | the 18c direct path writes `ff ff` for an empty table **with** a segment and nothing for one without: the R-017 hypothesis is confirmed on 18c | `seg_exp_direct_18c`@0x226a, 0x23ca, 0x24c7 |
| 11 | rules T5, Y5, R2, R8, Y9, E2, E3: version 3.7 | the rows above |
| 12 | new tools `tsp_check.py`, `long3_check.py`; `exp_parse.py` reads the conventional `fd ff` form; `rowlen_stats.py` counts conventional LONG forms apart | 12 |

Counts (`tools/rowlen_stats.py`, `tools/check_all.py`, session 17): **211** EXP files (95 / 89 / 27), 418 binary parts,
41,284 rows, **243,303** column fields, 19,960 NULL; `problems 0` in `check_all.py` for every file; 0 column fields of
length 0; outside the LONG columns of `long3_exp_<v>` 0 fields from 0x8000 up (other than `fe ff` / `ff ff`).

## Changes in session 11

| Section | Change |
|---|---|
| Fixtures | No new dump. One new non-log file, `18c/truth_r045.txt` (R-045, the owner's SQL*Plus spool, 2026-10-17); table "Fixtures added in session 11". `sha256sum -c SHA256SUMS`: **147 of 147 OK** in `18c/` (the owner's `SHA256SUMS` lists `./truth_r045.txt`, prefix `b5fb62d1b3a0`), 140 of 140 in `11g/`, 30 of 30 in `11g-cp1251/`; every non-log file of the three folders is listed, no `.log` file is (as before). The tools run on all **201** EXP files (89 / 85 / 27) |
| 4.7, 6.3 | Stale counts: `tools/collist.py` gives 386 lines, 1,907 column definitions and 57 character-semantics columns (30 AL32UTF8 + 27 CL8MSWIN1251) for the 201 files; the rows that still quoted the session 9 numbers (382, 1885, 55) say so now |
| 5.4 | The E2a remark on a partitioned single-column LONG table: R-045 is no longer "not yet run" (ORA-14085 observed on 18c) |
| 5.6 | `tools/desc_len_check.py` count updated to 243,007 column fields of 201 files |
| 11 | "Rules for a reader" version 3.6: counts only (T9) |
| 12 | Session 11 tool run recorded (all tools, 0 problems; counts unchanged against session 10) |

## Changes in session 10

| Section | Change |
|---|---|
| Fixtures | 2 new EXP files in `11g-cp1251/` (`ru_lob2_exp_cp1251_11g`, `ru_lob2_exp_us7ascii_11g`, R-038) with logs and `truth_lob_ru.txt`; table "Fixtures added in session 10". `sha256sum -c SHA256SUMS`: 146 of 146 OK in `18c/`, 140 of 140 in `11g/`, **30 of 30** in `11g-cp1251/` (the owner's `SHA256SUMS` lists the three new non-log files); every non-log file is listed. The tools run on all **201** EXP files (89 / 85 / 27) |
| 14.6 | **New subsection "Session 10"**: in the CL8MSWIN1251 database a CLOB is the database bytes in the short **and the long** form (20,000 characters = `fc ff` + T = 20,000 + 8,132 + 8,132 + 3,736); NCLOB is UTF-16 BE, short (132 bytes) and long (40,000 = 16,264 + 16,264 + 7,472: the first long NCLOB of the corpus); BLOB as in AL32UTF8 (8,132 + 8,132 + 3,736); the CLOB chunk of the single-byte database is 8,132 bytes, not 16,264; the LOB values do not depend on the client set (CL8MSWIN1251 / US7ASCII: identical bytes at identical offsets); R_TEXT after R_LOB2 is byte-identical to `ru_tbl_exp_*` (T8). The "not in the fixtures" rows are updated |
| 6.3 | Note A-23: w = 1 for set 171 is confirmed (one column, truth); other single-byte database sets: assumed |
| 6.6, 5.4 | Notes A-18 (piece u16 from 0xfff0, a piece of length 0: not in the fixtures) and A-19 (one list of item starts for E2 and E2a) |
| 9.8 | **New:** statements of session 9 withdrawn or weakened |
| 11 | "Rules for a reader" version 3.5: R2, C6, L4, L5, E2a, the gaps; counts 201 files |
| 12 | Tools: one new (`lob_ru_check.py`), two changed (`check_truth.py`: R_LOB2, which fell into the branch of the Cyrillic-named table and stopped `check_all.py` / `num_decode.py` with a `TypeError`; `num_decode.py`: step 3 repaired, note A-24); counts for 201 files |

Statuses lowered or statements withdrawn in session 10: see section 9.8.

## Changes in session 9

| Section | Change |
|---|---|
| Fixtures | 12 new EXP files in `18c/` (`t_num3`, `nchar2`, `t_probe11` + direct, `lob3`, `misc2`, `long2` + direct + us7ascii, `long_exp_direct`, `part2` + direct) and 10 in `11g/` (the same without `part2`), with logs and `truth_num3.txt`, `truth_nchar2.txt`, `truth_probe11.txt`, `truth_lob3.txt`, `truth_misc2.txt`, `truth_long2.txt`, `truth_part2.txt` (18c only); table "Fixtures added in session 9". `sha256sum -c SHA256SUMS`: 146 of 146 OK in `18c/`, 140 of 140 in `11g/`, 27 of 27 in `11g-cp1251/`; every non-log file is listed. The tools run on all **199** EXP files (89 / 85 / 25) |
| 4.1 | Character semantics: the descriptor length is min(4n, 4000) for `VARCHAR2(n CHAR)` and min(4n, 2000) for `CHAR(n CHAR)` in AL32UTF8 (= `data_length`, 7 columns, 2 versions, 2 paths). **New:** on the direct path a LONG / LONG RAW column is moved to the **end** of the `INSERT INTO` column list and of the descriptors |
| 4.8 | **New:** composite partitions (`COMPARTITION "p"`, `SUBPARTITION "s"`, `ENDCOMPARTITION`), one binary part per subpartition, empty subpartitions (E1 / E2), partition statistics before `ENDPARTITION` |
| 5.4 | E2 on 3 more empty direct-path binary parts (18c): an empty LONG table, two empty subpartitions; new item starts after p: `SUBPARTITION "`, `ENDCOMPARTITION` |
| 6.1 | **New:** computed quotients and an average have 20 digit bytes (39 or 40 significant digits) on both versions (H40 holds; H38 refuted) |
| 6.3 | **New:** non-BMP character in NCHAR, NVARCHAR2, `CHAR(n CHAR)`, `VARCHAR2(n CHAR)`; padding of NCHAR counts UTF-16 code units, of `CHAR(n CHAR)` characters; descriptor length rule for character semantics |
| 6.6 | **New:** LONG on the direct path (pieces with the flag 0x8000; the column moved last); a non-ASCII LONG is database bytes with an AL32UTF8 and a US7ASCII client |
| 6.7 | **New:** intervals: every field of a negative interval is negative (confirmed), precisions 9 / 9 and 2 / 0, the text with f = 0; ROWID prefix `01 00 00 00 17` in 6 more values; logical UROWID (first byte `02`) and its text |
| 9.7 | **New:** statements of session 8 withdrawn or weakened |
| 11 | "Rules for a reader" version 3.4 |
| 12 | Tools: four new (`desc_len_check.py`, `frame_t8.py`, `lob3_switch.py`, `part2_check.py`), eight changed (`exp_parse.py`, `check_truth.py`, `check_all.py`, `collist.py`, `rowlen_stats.py`, `boundary.py`, `num_decode.py`, `misc_types.py`); counts for 199 files |
| 14.6 | **New:** LOB sizes around the switch (long iff larger than one chunk, both versions, CLOB and BLOB); non-ASCII and non-BMP CLOB / NCLOB in AL32UTF8 (surrogate pairs; `GETLENGTH` counts UTF-16 code units) |

Statuses lowered or statements withdrawn in session 9: see section 9.7.

## Changes in session 8

| Section | Change |
|---|---|
| Fixtures | 5 new EXP files in `18c/` (`lob_exp_18c`, `lob_exp_direct_18c`, `misc_exp_18c`, `part_exp_18c`, `part_exp_direct_18c`) and 3 in `11g/` (`lob_exp_11g`, `lob_exp_direct_11g`, `misc_exp_11g`), with logs and `truth_lob.txt`, `truth_misc.txt`, `truth_part.txt` (18c only); table "Fixtures added in session 8". `sha256sum -c SHA256SUMS`: 127 of 127 OK in `18c/`, 124 of 124 in `11g/`, 27 of 27 in `11g-cp1251/`. The tools of section 12 run on all **177** EXP files (77 / 75 / 25) |
| 4.1 | Descriptors of types 112 (CLOB and NCLOB: 8 bytes), 113 (BLOB), 182, 183 (INTERVAL), 69 (ROWID), 208 (UROWID): 4 bytes each |
| 4.7 | Type texts `BLOB`, `NCLOB`, `ROWID`, `UROWID(n)`, `INTERVAL YEAR (p) TO MONTH ` (with a space before the separator), `INTERVAL DAY (p) TO SECOND (f)`; 334 lines |
| 4.8 | **New:** a partitioned table: one binary part per partition, each after a `PARTITION "name"` line and its own `INSERT INTO` line; one `ENDPARTITION` line after the last |
| 5.6 | LOB tables included (237,347 fields in column positions, 0 of length 0, 0 from 0xfff0 to 0xfffd); the LOB value area after a row is separate and does hold the length 0 (empty LOB) and the marker `fc ff` |
| 6.1 | **Correction (note A-6):** row 30 (pi) has 38 significant digits, not 39; the only value above 38 is row 31 (40) |
| 6.7 | **New:** INTERVAL YEAR TO MONTH, INTERVAL DAY TO SECOND, ROWID, UROWID: bytes and conversion to the `TO_CHAR` / `ROWIDTOCHAR` text |
| 9.6 | **New:** statements of session 7 withdrawn or weakened |
| 11 | "Rules for a reader" version 3.3: rules H4a, H6, T5, T6, T8, R4, R8, Y1, Y9, E2 changed; new rules T10 (partitions), L1 to L6 (LOB), Y10, Y11 (INTERVAL, ROWID, UROWID); counts for 177 files; the list of introducing lines is unchanged |
| 12 | Tools: three new (`lob_check.py`, `misc_types.py`, `part_check.py`), seven changed (`exp_parse.py`, `check_truth.py`, `check_all.py`, `collist.py`, `rowlen_stats.py`, `boundary.py`, `num_decode.py`); `corpus_counts.py` unchanged |
| 14.6 | Rewritten: LOB columns are no longer "observed only"; the session 4 hypothesis is replaced by the layout of the session 8 files (it was wrong in one detail, the 8 bytes after the path flag, and right in the others) |

Statuses lowered or statements withdrawn in session 8: see section 9.6.

## Changes in session 7

| Section | Change |
|---|---|
| Fixtures | 6 new EXP files per version in `18c/` and `11g/` (R-023: `stc_exp_us7ascii_<v>`, `stc_exp_none_<v>`; R-024: `t_num2_exp_<v>`; R-025: `t_decl_exp_<v>`; R-026a: `nchar_exp_<v>`; R-028: `long_exp_<v>`; with logs) and the truth files `truth_stc_extra.txt`, `truth_num2.txt`, `truth_decl.txt`, `truth_nchar.txt`, `truth_csmap.txt`; table "Fixtures added in session 7". `sha256sum -c SHA256SUMS`: 119 of 119 OK in `18c/`, 119 of 119 in `11g/`, 27 of 27 in `11g-cp1251/`. The tools of section 12 run on all **169** EXP files (72 / 72 / 25) |
| 4.1 | Rows for NCHAR (type 96, form 2), `CHAR(n CHAR)` in AL32UTF8 (descriptor length 4n: R-011 answered for CHAR), LONG (type 8) and LONG RAW (type 24) with a 4-byte descriptor of length 0; the ten numeric forms of `t_decl` all give `02 00 16 00` |
| 4.6 | New subsection "The R-020 tables with a US7ASCII client and with `statistics=none` (session 7, R-023)": the automatic statistics job changed the statistics between the two exports; `truth_stc_extra.txt` |
| 4.7 | **New:** the column list of the `CREATE TABLE` line as a grammar, the text `exp` writes for 14 declared forms, and precision / scale against `truth_decl.txt` |
| 5.2, 5.6 | 5.2: largest length now 32,000 (a LONG value). **New 5.6:** a length of 0 and the u16 values 0xfff0 to 0xfffd (`fd ff` among them) never occur as a length field in the row data of the 169 files |
| 6.1 | The paragraph "How to turn the bytes into a decimal number is not part of this document's evidence" is replaced by a rule with a decoding procedure, tested on 72,928 NUMBER fields; the 14 vectors of R-024 |
| 6.3 | New subsection: NCHAR and `CHAR(n CHAR)` (R-026a) |
| 6.6 | **New:** LONG and LONG RAW (R-028) |
| 9 | Unknown rows added (descriptor length of `VARCHAR2(n CHAR)` in AL32UTF8 still open; LONG above 65,533 bytes; LONG on the direct path); open questions 4, 7, 11 updated; new 9.5 (statements of session 6 withdrawn or weakened) |
| 11 | "Rules for a reader" version 3.2: rules T5, T8, R2, Y1, C1, E2 changed; new rules T9 (column list), R8 (lengths that do not occur), C5 (NCHAR), C6 (character semantics), C7 (single-byte maps), Y9 (LONG, LONG RAW); the list block has a fourth field (the allowed forms); counts for 169 files |
| 12 | Tools: five new (`num_decode.py`, `collist.py`, `csmap_cmp.py`, `rowlen_stats.py`, `corpus_counts.py`), four changed (`check_truth.py`, `check_all.py`, `stc_items.py`, `intro_list.py`) |
| 13.1 | 72 pairs now; the 6 new stems of R-024 to R-028 are byte-identical from offset R on (18c against 11g) |
| 14.11 | **New:** Oracle's byte-to-Unicode map for US7ASCII, WE8MSWIN1252, CL8MSWIN1251 against Python's codecs (R-026b) |

Statuses lowered or statements withdrawn in session 7: see section 9.5.

## Changes in session 6

| Section | Change |
|---|---|
| Fixtures | 2 new EXP files per version in `18c/` and `11g/` (R-020: `stc_exp_<v>.dmp`, `stc_exp_direct_<v>.dmp`, logs) and `truth_stc.txt`; table "Fixtures added in session 6". `sha256sum -c SHA256SUMS`: 108 of 108 OK in `18c/`, 108 of 108 in `11g/`, 27 of 27 in `11g-cp1251/`. The tools of section 12 run on all 157 EXP files |
| 2 | The row of the date text: its place and format stated as a rule a tool can use (ends at 0x7f; 21 to 23 bytes; no leading zeros), checked on 157 files |
| 4 | One sentence: the statistics items are one per column **that has statistics** |
| 4.6 | Result paragraph, two rows of "Item forms", the counts, the order of the items after the rows (`GRANT` stands **before** the table item), the row "One item per table plus one per column" (**weakened**: one per column that has statistics); new subsections "Columns without statistics (session 6, R-020)" and "The real dump, reconciled (session 6)". The session 5 text about the real dump is kept and marked as answered |
| 7.1 | The counter on the R-020 files (one paragraph) |
| 8 | One more pair of evidence for "items after the rows are identical on both paths" |
| 9 | Open question 12 updated; new 9.4 (statements of session 5 withdrawn or weakened) |
| 11 | "Rules for a reader" version 3.1: rules T7, S3 and S6 changed, new rule S7, one gap row changed; the list of introducing lines is **unchanged** (counts updated to 157 files) |
| 12 | Tools: one changed (`check_truth.py`), two new (`stc_items.py`, `header_date.py`) |
| 13.9 | One row: column statistics of an all-NULL column, 18c against 11g; 66 pairs now |

Statuses lowered or statements withdrawn in session 6: see section 9.4.

## Changes in session 5

| Section | Change |
|---|---|
| Fixtures | 10 new EXP files per version in `18c/` and `11g/` (R-016 part a: 5 dumps each, with logs) and 10 in `11g-cp1251/` (R-016 part b), with `truth_stat.txt` in each folder; table "Fixtures added in session 5". `sha256sum -c SHA256SUMS`: 105 of 105 OK in `18c/`, 105 of 105 in `11g/`, 27 of 27 in `11g-cp1251/` (the `.log` files are not listed in `SHA256SUMS`). The tools of section 12 run on all 153 EXP files |
| 4, 4.2 | Statistics items after the rows; the field after an `ANALSTATS` line is plain **or** in pieces after `fd ff`; a new line form `ANALSTATS C? "T" ("col" ) `; new subsection "Session 5" in 4.2 |
| 4.6 | **New:** optimizer statistics items (`ANALSTATS T?` / `I?` / `C?`, `ANALCOMPUTE T?` / `I?`): forms, the walk of the R-016 files, which table gets which items, what is inside the field (compared with `truth_stat.txt`), `statistics=none` / `compute`, the direct path, the letters and the log, the counts of the real dump with a structure-only script specification |
| 7.1 | A trailer group can hold a field (`ANALSTATS C?`): one row changed |
| 8 | "items before and after the rows are identical on both paths": more evidence |
| 9 | Unknown rows and open questions 6 and 12 updated; new 9.3 (statements of session 4 withdrawn or weakened) |
| 11 | "Rules for a reader" version 3: rules I4, I6, T7 changed, new rules S1 to S6, and the list of lines that introduce a field in a form a tool can read |
| 12 | Tools: two changed, three new |
| 13 | 13.1 (64 pairs now), 13.2 (one more table), 13.5 (**correction**: the last argument of `SET_INDEX_STATS` is not a version constant); new 13.9 (statistics items, 18c against 11g) |
| 14 | 14.5 completed (letters `T` and `C`; the count of `EXP-00091`); new 14.10 (text in statistics; comments written with a US7ASCII client and what a reader returns for them) |

Statuses lowered or statements withdrawn in session 5: see section 9.3.

## Changes in session 4

| Section | Change |
|---|---|
| Fixtures | 13 new EXP files per version (R-012, R-013) and the folder `corpus/fixtures/11g-cp1251/` (15 EXP files, R-015). `sha256sum -c SHA256SUMS`: 99 of 99 OK in `18c/`, 99 of 99 OK in `11g/`, 16 of 16 OK in `11g-cp1251/`. Three fixture tables instead of one |
| 1.2 | The last row ("boundary between the two bytes of a length field: unknown") is replaced by the result of R-013: nothing is inserted, for a length field, an `fe ff` marker and a `00 00` row end; RECORDLENGTH 2048 added |
| 2, 2.1, 3 | Header 0x0032 and block 1 +0x3c: status raised from hypothesis to confirmed (database character set; second database with id 171). User line and mode line: raised (three users, two modes). The offsets of the binary header fields are not fixed (they follow the text lines). `METRICSU` instead of `METRICST` in owner mode |
| 4.1 | +2 for `VARCHAR2(n CHAR)` in a single-byte database; +4 = 171; type code 112 (CLOB) has an 8-byte descriptor |
| 4.2 | **Correction of session 3:** neither rule G nor rule K as written consumes the new files. A third field form (pieces without `fd ff`, view text); four more introducing lines; `fd ff` raised from observed to confirmed; lengths above 255 in text fields |
| 4.4 | Step 1 and step 4 changed for owner mode; counts for all 133 files |
| 4.5 | New: owner mode, with the item walk of the R-012 files (both versions) |
| 5.1, 5.2 | Largest field length seen is now 3000 |
| 5.4 | **Correction of session 3:** "direct path, table without rows: no `ff ff`" holds on 18c only. On 11g `ff ff` is written. Empty table with its own statements; a table section that was cut short |
| 6.5 | TSLTZ: the 11g database has the time zone +00:00 too, the question stays open |
| 7, 7.1 | `METRICSU` / `METRICEU`; the counter after `METRICE?` on the new files; one `ENDTABLE` can close several `TABLE` lines |
| 8 | A file written with `direct=y` can hold a conventional-path table (LOB table); empty table on the direct path differs by version |
| 9 | Unknown rows and open questions 1, 2, 3, 6, 7, 8, 10 updated; new 9.2 (statements of session 3 withdrawn or weakened) |
| 10 | Replaced by a pointer to section 13 |
| 11 | "Rules for a reader" version 2, with an `11g` column |
| 12 | Tools: three rewritten, six new |
| 13 | New: Oracle 11g |
| 14 | New: character sets (row data, text items, identifiers), `ANALSTATS` variants, CLOB (outside phase 1), survey statements that the fixtures did not bear out |

Statuses lowered or statements withdrawn in session 4: see section 9.2.

## Changes in session 3

| Section | Change |
|---|---|
| Fixtures | 8 new files (R-008, R-009); `truth_raw.txt` is usable now. All 46 EXP files of `corpus/fixtures/18c/` verified with `SHA256SUMS` (all OK). Counts such as "38 fixtures" or "19 pairs" in rows written in session 2 name the files of that session; `tools/header_fields.py` and `tools/check_all.py` were run again on all 46 files with the same result (no invariant violation, no differing field) |
| 2, 2.1 | New experiment with non-ASCII text (`t_text`, three client sets, both paths): the table section is byte-identical for every client set. The "Open, important" paragraph is replaced by the result |
| 4 | **Correction of a session 2 statement:** a table section does not end with the LF after its rows. Statements can follow the rows (indexes, constraints, comments). New rows for them; new sections 4.2 (items of the text region, framing rule), 4.3 (walk of `ddl`), 4.4 (what a reader needs) |
| 4.1 | +4 of a character descriptor: status raised (it names the set the value bytes are in, for every client set tested). +6 unchanged by the client set |
| 5.4 | Rewritten: empty table first, in the middle, last; the heuristic is replaced by two candidate rules, both tested on all fixtures |
| 6.3 | New rows: value bytes and length fields do not depend on the client character set |
| 6.5 | TIMESTAMP WITH TIME ZONE described field by field; TSLTZ rows 101 / 102; RAW compared with the corrected truth (confirmed) |
| 7.1 | `multi_empty` / `multi_empty2` added as evidence; `TABLE` ... `ENDTABLE` groups inside the trailer (from `ddl`) |
| 8 | "table without rows: no `ff ff`" raised from observed to confirmed |
| 9 | Unknown rows for `ddl` replaced by references to 4.2; open questions 1, 2, 5, 6 updated; new open questions 9 to 11 |
| 10 | Two more sanity runs on 11g (still not an analysis) |
| 11 | New: "Rules for a reader". The tools table moved from 11 to 12; no other section number changed |

Statuses lowered or statements withdrawn in session 3: see section 9.1.

## Fixtures cited

Fixtures of sessions 1 to 3 (table below). All files are in `corpus/fixtures/18c/`: Oracle 18c XE (`truth_env.txt`: 18.0.0.0.0; logs:
"Version 18.4.0.0.0"), legacy `exp`, table mode, database character set AL32UTF8, national
character set AL16UTF16. `SHA256SUMS` verified at the start of sessions 2 and 3 (all OK).
For every `X_exp_18c.dmp` (conventional path) there is an `X_exp_direct_18c.dmp` (`direct=y`).

| Fixture stem | Tables, rows (`truth_counts.txt`, logs) | Client character set (log) | Option | SHA-256 prefix, conventional file | SHA-256 prefix, direct file |
|---|---|---|---|---|---|
| `t_frame` | T_FRAME 7 | AL32UTF8 | - | `c68e05cf6f96` | `71230caa9ab4` |
| `t_frame_exp_us7ascii_18c.dmp` / `t_frame_exp_direct_us7ascii_18c.dmp` | T_FRAME 7 | US7ASCII | `NLS_LANG` | `b4414699d85b` | `a7c2b09063af` |
| `t_frame_exp_w1252_18c.dmp` / `t_frame_exp_direct_w1252_18c.dmp` | T_FRAME 7 | WE8MSWIN1252 | `NLS_LANG` | `67d7bbf939f9` | `600a64ded5f2` |
| `t_frame_exp_rl4096_18c.dmp` | T_FRAME 7 | AL32UTF8 | `recordlength=4096` | `f5780bbed3d8` | (none) |
| `t_frame_exp_rl16384_18c.dmp` | T_FRAME 7 | AL32UTF8 | `recordlength=16384` | `895c241821de` | (none) |
| `t_probe6` | T_PROBE6 300 | AL32UTF8 | - | `a10542f008d3` | `ad6d357e007a` |
| `t_empty` | T_EMPTY 0 | AL32UTF8 | - | `8bb40f3b7cf1` | `e5fb0d9bc1ab` |
| `multi_framing` | T_FRAME 7, T_LEN 22, T_NULLMIX 8 | AL32UTF8 | - | `7906c9a8455a` | `1a07a358c727` |
| `multi_types` | T_NUM 33, T_DATE 11, T_TS 6, T_TEXT 10 | AL32UTF8 | - | `08975828008c` | `38734c4b78f7` |
| `t_len` | T_LEN 22 | AL32UTF8 | - | `679e5f9204f2` | `72ef53b275d5` |
| `t_nullmix` | T_NULLMIX 8 | AL32UTF8 | - | `e8dcb8129c73` | `39659cbede40` |
| `t_num` | T_NUM 33 | AL32UTF8 | - | `3df4a6c57e9b` | `9c7cc3b42656` |
| `t_date` | T_DATE 11 | AL32UTF8 | - | `7ccd14583ddf` | `7478602935cd` |
| `t_text` | T_TEXT 10 | AL32UTF8 | - | `527af43e57d2` | `cdcf20959893` |
| `t_ts` | T_TS 6 | AL32UTF8 | - | `674544c84061` | `4c738ee15887` |
| `t_tstz` | T_TSTZ 10 | AL32UTF8 | - | `45e2c8b55648` | `593557aebf71` |
| `t_numtypes` | T_NUMTYPES 5 | AL32UTF8 | - | `013ffc508d90` | `0c03568ad636` |
| `t_raw` | T_RAW 7 | AL32UTF8 | - | `33f6e8ade0af` | `97a2e845240a` |
| `t_wide` | T_WIDE 4 | AL32UTF8 | - | `bbd24496bb6d` | `a82eaff935c4` |
| `ddl` | T_PARENT 2, T_DDL 3 | AL32UTF8 | - | `ba98abf8b160` | `e7e1e5610dcc` |
| `t_text_exp_w1252_18c.dmp` / `t_text_exp_direct_w1252_18c.dmp` (session 3, R-008) | T_TEXT 10 | WE8MSWIN1252 (id 178) | `NLS_LANG` | `d4857a91ba25` | `2dbc21b6bdaa` |
| `t_text_exp_us7ascii_18c.dmp` / `t_text_exp_direct_us7ascii_18c.dmp` (session 3, R-008) | T_TEXT 10 | US7ASCII (id 1) | `NLS_LANG` | `6a8377c2f14b` | `92438ca8a085` |
| `multi_empty` (session 3, R-009) | T_FRAME 7, T_EMPTY 0, T_LEN 22 (this order) | AL32UTF8 | - | `03abb758e9a0` | `a31db1a194c7` |
| `multi_empty2` (session 3, R-009) | T_EMPTY 0, T_FRAME 7 (this order) | AL32UTF8 | - | `111a5ecd25cf` | `82e86e10095a` |

The last six stems (`t_ts` to `ddl`) were not in the fixture list of session 2. They are cited
only in the rows that name them.

Truth used: `truth_charsets.txt`, `truth_env.txt`, `truth_counts.txt`, `truth_frame.txt`,
`truth_len.txt`, `truth_nullmix.txt`, `truth_num.txt`, `truth_date.txt`, `truth_text.txt`,
`truth_probe6.txt`, `truth_ts.txt`, `truth_tstz.txt`, `truth_numtypes.txt`, and the export logs.
Session 3 also uses `truth_raw.txt` (corrected by R-010, SHA-256 prefix `0572059f0c46`: every byte
of every value in one or two parts), `truth_wide.txt` and `truth_ddl.txt`. T_PARENT has no truth
file; its two rows are compared with the INSERT statements in `corpus/sql/01_corpus_tables.sql`
(lines 299-300) and that comparison is called "sql", not truth.

Short names: **CONV** = `t_frame_exp_18c.dmp`, **DIRECT** = `t_frame_exp_direct_18c.dmp`.
Other files are named by their stem, e.g. `t_num` = `t_num_exp_18c.dmp`, `t_num/direct` =
`t_num_exp_direct_18c.dmp`.

### Fixtures added in session 4

`corpus/fixtures/18c/` (Oracle 18c XE, database AL32UTF8 / AL16UTF16) and `corpus/fixtures/11g/`
(Oracle 11g XE; `truth_env.txt`: `version=11.2.0.2.0`, `NLS_CHARACTERSET=AL32UTF8`,
`NLS_NCHAR_CHARACTERSET=AL16UTF16`, `dbtimezone=+00:00`). The folder `11g/` holds every stem of
the table above with `_11g` instead of `_18c` (46 EXP files, made from the same SQL), plus the 13
new files. `SHA256SUMS` of both folders verified at the start of session 4 (99 of 99 OK each).
Names: `own` = `own_exp_18c.dmp`, `own/direct` = `own_exp_direct_18c.dmp`, and `own_11g`,
`own_11g/direct` and so on for the 11g files.

| Fixture stem | Objects (logs, truth) | Mode, user | Client character set (log) | 18c: SHA-256 prefix conv / direct | 11g: SHA-256 prefix conv / direct |
|---|---|---|---|---|---|
| `own` (R-012) | O_CHILD 3, O_EMPTY 0, O_PARENT 2 (log order), index, constraints, comments, sequence, view, synonym, 2 procedures, trigger, grant | owner (`RUSERS`), CORPUS_OWN | AL32UTF8 | `36b2fcee1455` / `43e016c8c0e7` | `46cd8f048490` / `38701b1b7f14` |
| `own_exp_w1252_<v>.dmp` (R-012) | the same | owner, CORPUS_OWN | WE8MSWIN1252 | `52614a0fa402` / (none) | `70ce5c6df5e3` / (none) |
| `own_tables` (R-012) | O_PARENT 2, O_CHILD 3, O_EMPTY 0 (log order) | table (`RTABLES`), CORPUS_OWN | AL32UTF8 | `bb4bc7fb3017` / `58561004f8d5` | `f1d5c2ae086b` / `4b2a267bfa69` |
| `t_bn1` (R-013) | T_BN1 8 (RAW of 1259 bytes) | table, CORPUS | AL32UTF8 | `e429ef560571` / `c367e4524c3c` | `fbdcd08a8f12` / `9e9cd1d228c4` |
| `t_bn2` (R-013) | T_BN2 8 (RAW of 1261 bytes) | table, CORPUS | AL32UTF8 | `2f3102676dc5` / `50d4186ab4df` | `21365c44356b` / `61f39d536101` |
| `t_bn3` (R-013) | T_BN3 32 (NULL, VARCHAR2 of 256 bytes) | table, CORPUS | AL32UTF8 | `d6fe626f0258` / `f94b55200eb1` | `315114402d33` / `b9e33467d022` |
| `t_bn4` (R-013) | T_BN4 32 (NULL, VARCHAR2 of 258 bytes) | table, CORPUS | AL32UTF8 | `558ac5fc70d0` / `9994151f1d0e` | `1b0f41459703` / `546a28782c01` |

Truth added: `truth_own.txt` (rows; identical in both folders, prefix `5e46a6928d16`),
`truth_own_ddl.txt` (object list, comment lengths and bytes, constraint list, source line lengths;
18c `418809cf2c3b`, 11g `d4db20a6a2c4`: the names `SYS_C...` differ and 11g has no
`search_condition_vc`), `truth_bn1.txt` to `truth_bn4.txt` (identical in both folders: length and
the first 4 bytes or 5 characters of each value, not all bytes).

The three 18c owner-mode logs contain `EXP-00008` / `ORA-06550` / `PLS-00201` / `EXP-00083`
(no package `SYS.DBMS_CUBE_EXP` in 18c XE) at the steps "pre-schema" and "post-schema procedural
objects and actions". This says only that these files hold no OLAP metadata; a dump from another
installation may hold more items in these two sections. The 11g logs have no such lines.

`corpus/fixtures/11g-cp1251/` (R-015): Oracle 11g XE 11.2.0.2, `NLS_CHARACTERSET` CL8MSWIN1251 (id
171), `NLS_NCHAR_CHARACTERSET` AL16UTF16 (`truth_ru.txt` lines `ENV|`), user CORPUS_RU. All exports
were asked for `recordlength=2048` (`corpus/scripts/make_ru.sh` line 135, `make_ru_tables.sh` line
28; scripts read, not run). `SHA256SUMS` verified (16 of 16 OK). Truth: `truth_ru.txt` (prefix
`91c2847d01a8`): `DUMP(col,16)` of every R_TEXT column except LV (length and first 60 bytes),
R_CLOB (length and first 60 bytes), names and comments.

| Fixture (`.dmp`) | Tables, rows (log) | Mode | Path | Client character set (log) | RECORDLENGTH in the header | SHA-256 prefix |
|---|---|---|---|---|---|---|
| `ru_tbl_exp_us7ascii_11g` | R_TEXT 309 | table | conventional | US7ASCII | 2048 | `02d8b1c2c924` |
| `ru_tbl_exp_cp1251_11g` | R_TEXT 309 | table | conventional | CL8MSWIN1251 | 2048 | `388b8046f8de` |
| `ru_tbl_exp_al32utf8_11g` | R_TEXT 309 | table | conventional | AL32UTF8 | 2048 | `21dce639310a` |
| `ru_tbl_exp_w1252_11g` | R_TEXT 309 | table | conventional | WE8MSWIN1252 | 2048 | `85262699f8ed` |
| `ru_tbl_exp_direct_us7ascii_11g` | R_TEXT 309 | table | direct | US7ASCII | 8192 | `b52a22a98fa8` |
| `ru_tbl_exp_direct_cp1251_11g` | R_TEXT 309 | table | direct | CL8MSWIN1251 | 8192 | `e5a23e59eec8` |
| `ru_tbl_exp_direct_al32utf8_11g` | R_TEXT 309 | table | direct | AL32UTF8 | 8192 | `45402ddd450e` |
| `ru_tbl_exp_direct_w1252_11g` | R_TEXT 309 | table | direct | WE8MSWIN1252 | 8192 | `e0c7528a8ea1` |
| `ru_tables_exp_us7ascii_11g` | R_TEXT 309, R_CLOB 3 | table | conventional | US7ASCII | 2048 | `33884b319bb3` |
| `ru_exp_us7ascii_11g` | R_CLOB 3, R_TEXT 309, `???_???` (failed, `EXP-00056` / `ORA-00942`) | owner | conventional | US7ASCII | 2048 | `4a8a85d1cebd` |
| `ru_exp_cp1251_11g` | R_CLOB 3, R_TEXT 309, table with a Cyrillic name 2 | owner | conventional | CL8MSWIN1251 | 2048 | `a33ad18d7bd0` |
| `ru_exp_al32utf8_11g` | R_CLOB 3, R_TEXT 309, table with a Cyrillic name 2 | owner | conventional | AL32UTF8 | 2048 | `35e8dcc5bad9` |
| `ru_exp_w1252_11g` | R_CLOB 3, R_TEXT 309, name written as `bf bf bf 5f bf bf bf` (failed, `EXP-00056` / `ORA-00942`) | owner | conventional | WE8MSWIN1252 | 2048 | `677cb4b2e2a8` |
| `ru_exp_direct_us7ascii_11g` | R_CLOB 3 ("will be exported in conventional path"), R_TEXT 309, `???_???` (failed, `EXP-00008` / `ORA-00604` / `ORA-00942`) | owner | direct | US7ASCII | 8192 | `0706c0babfa1` |
| `ru_exp_direct_cp1251_11g` | R_CLOB 3 (conventional path), R_TEXT 309, table with a Cyrillic name 2 | owner | direct | CL8MSWIN1251 | 8192 | `2d06922634a2` |

### Fixtures added in session 5

R-016, optimizer statistics. All are table mode (`RTABLES`), legacy `exp` of the same version as
the database. Export options from `corpus/scripts/make_stat.sh` and `make_ru_stats.sh` (scripts
read, not run) and from the logs.

Part a, `corpus/fixtures/18c/` and `corpus/fixtures/11g/` (database AL32UTF8, user CORPUS,
`tables=t_stat1,t_stat2,t_stat3,t_stat4`; SQL: `corpus/sql/09_stat.sql`). T_STAT1: 50 rows,
`ID NUMBER(6)` primary key without a name, `V VARCHAR2(30)`, `N NUMBER`, `D DATE`, index
T_STAT1_IX, `DBMS_STATS` with `FOR ALL COLUMNS SIZE 10`; T_STAT2: the same rows, `DBMS_STATS`
with `SIZE 1`; T_STAT3: the same rows, `ANALYZE TABLE ... COMPUTE STATISTICS`; T_STAT4: no rows,
2 columns, `DBMS_STATS`.

| Fixture (`.dmp`) | Path | Client character set (log) | `statistics=` | `EXP-00091` lines in the log | 18c: SHA-256 prefix | 11g: SHA-256 prefix |
|---|---|---|---|---|---|---|
| `stat_exp_<v>` | conventional | AL32UTF8 | estimate | 0 | `37b955ce40dc` | `8b983d30c8d0` |
| `stat_exp_direct_<v>` | direct | AL32UTF8 | estimate | 0 | `15a964035a1c` | `bbd4e524b6a3` |
| `stat_exp_us7ascii_<v>` | conventional | US7ASCII | estimate | 6 | `bb73dbf2fce4` | `e237cc53c760` |
| `stat_exp_none_<v>` | conventional | AL32UTF8 | none | 0 | `c4db512a0a12` | `cc1660cca2df` |
| `stat_exp_compute_<v>` | conventional | AL32UTF8 | compute | 0 | `17cae03bae76` | `e91fb68919e8` |

Truth: `truth_stat.txt` (18c prefix `92440d7dadc7`, 11g `9cea8d5e92d3`): `ROW|` (the 50 rows of
T_STAT1), `TABSTAT|`, `COLSTAT|`, `TAB|`, `COL|`, `HIST|`, `IND|`. The two files differ: the
histogram of T_STAT1.ID and T_STAT1.D is `HYBRID` on 18c and `HEIGHT BALANCED` on 11g, and the
18c file has an endpoint actual value for ID and N. T_STAT2 and T_STAT3 have no `ROW|` lines; their
rows are compared with those of T_STAT1 because the SQL creates them with `CREATE TABLE ... AS
SELECT * FROM t_stat1` ("sql", not truth).

Part b, `corpus/fixtures/11g-cp1251/` (Oracle 11g XE, database CL8MSWIN1251, user CORPUS_RU,
`tables=r_stat,r_text`, `recordlength=2048` asked for; SQL: `corpus/sql/08_ru_stats_objects.sql`).
R_STAT: 2000 rows, `ID NUMBER(6)`, `GRP NUMBER(3)`, `TXT VARCHAR2(40)` (Cyrillic), `CODE CHAR(6)`,
`DT DATE`, `AMT NUMBER(10,2)`, `NOTE VARCHAR2(100)`; indexes R_STAT_PK, R_STAT_TXT_IX,
R_STAT_GD_IX; histograms on GRP, TXT, DT, AMT. R_TEXT: the table of R-015, now with statistics.

| Fixture (`.dmp`) | Path | Client character set (log) | `statistics=` | RECORDLENGTH in the header | `EXP-00091` lines | SHA-256 prefix |
|---|---|---|---|---|---|---|
| `ru_stat_exp_us7ascii_est_11g` | conventional | US7ASCII | estimate | 2048 | 6 | `32f063e06810` |
| `ru_stat_exp_us7ascii_cmp_11g` | conventional | US7ASCII | compute | 2048 | 6 | `0b9a48d0c9e4` |
| `ru_stat_exp_cp1251_est_11g` | conventional | CL8MSWIN1251 | estimate | 2048 | 0 | `fcdc60966ddc` |
| `ru_stat_exp_cp1251_cmp_11g` | conventional | CL8MSWIN1251 | compute | 2048 | 0 | `60cb44a82b70` |
| `ru_stat_exp_al32utf8_est_11g` | conventional | AL32UTF8 | estimate | 2048 | 6 | `a62672f4ddfd` |
| `ru_stat_exp_us7ascii_none_11g` | conventional | US7ASCII | none | 2048 | 0 | `411029a9e666` |
| `ru_stat_exp_direct_us7ascii_est_11g` | direct | US7ASCII | estimate | 8192 | 6 | `58c2a4343a81` |
| `ru_stat_exp_direct_us7ascii_cmp_11g` | direct | US7ASCII | compute | 8192 | 6 | `acf3b195b9ff` |
| `ru_stat_exp_direct_cp1251_est_11g` | direct | CL8MSWIN1251 | estimate | 8192 | 0 | `f04b25e5a53a` |
| `ru_stat_exp_direct_cp1251_cmp_11g` | direct | CL8MSWIN1251 | compute | 8192 | 0 | `36c3794cd203` |

Truth: `truth_stat.txt` of that folder (prefix `e83140375e07`): `TAB|`, `COL|`, `HIST|`, `IND|`. It
has **no rows** of R_STAT; the 2000 rows are compared with what the PL/SQL loop of
`08_ru_stats_objects.sql` must produce (all 7 columns; "sql", not truth). Its lines for R_CLOB
and for the table and index with Cyrillic names are not used: those objects are not in these
dumps. The statistics of this truth file describe the database at the time of the R-016 exports,
not at the time of the R-015 exports of the same folder.

State of the working tree in session 5: the part a files are committed; the part b files, their
`truth_stat.txt` and the changed `SHA256SUMS` of `11g-cp1251/` were in the folder but **not yet
committed** to git (`git status`: untracked). They are cited here as given.

Every table of the 20 files equals its truth or SQL (`tools/check_all.py`): T_STAT1 150 fields per
file from the truth; T_STAT2, T_STAT3 150 each by SQL; T_STAT4 0 rows; R_STAT 12000 fields by SQL;
R_TEXT 1854 fields (`truth_ru.txt`); 0 differ. The rows of R_STAT are not in key order (first row
written: ID 653, `ru_stat_exp_us7ascii_est_11g.dmp`@0x0b15), rule R6.

### Fixtures added in session 6

R-020, tables in which not every column has statistics. Table mode (`RTABLES`), user CORPUS,
database AL32UTF8, default client (AL32UTF8; the logs have no `EXP-00091` line), default
`statistics=estimate`, `tables=t_stc1,t_stc2,t_stc3,t_stc4,t_stc5` (log order), legacy `exp` of the
same version as the database. SQL: `corpus/sql/10_stc.sql`. Every table has the same 50 rows
(`ID NUMBER(6)`, `A VARCHAR2(20)`, `B VARCHAR2(20)`, `C DATE`, `ALLNULL VARCHAR2(10)` NULL in every
row); T_STC2 has `ID, A, B, C` and two columns added after its statistics were gathered
(`ADDED_LATER VARCHAR2(10)` NULL in every row, `ADDED_NUM NUMBER` = `MOD(id, 3)`).

| Fixture (`.dmp`) | Path | Client character set (log) | `EXP-00091` lines | 18c: SHA-256 prefix | 11g: SHA-256 prefix |
|---|---|---|---|---|---|
| `stc_exp_<v>` | conventional | AL32UTF8 | 0 | `c261381fc172` | `c0fc139c7ad9` |
| `stc_exp_direct_<v>` | direct | AL32UTF8 | 0 | `c2a5b7e70610` | `729af85594f3` |

| Table | Statistics at export time (`10_stc.sql`) | `truth_stc.txt` |
|---|---|---|
| T_STC1 | `GATHER_TABLE_STATS(... 'FOR COLUMNS SIZE 1 id a')`; also a `GRANT`, a table comment and a comment on column B | `TAB\|T_STC1\|50\|2026-10-07`; ID `stats:50:0`, A `stats:7:0`, B, C, ALLNULL `NO STATISTICS` |
| T_STC2 | `FOR ALL COLUMNS SIZE 1` on four columns, then `ALTER TABLE ... ADD (added_later, added_num)` and an `UPDATE` of ADDED_NUM | `TAB\|T_STC2\|50\|...`; ID, A, B, C `stats:`; ADDED_LATER, ADDED_NUM `NO STATISTICS` |
| T_STC3 | `FOR ALL COLUMNS SIZE 1`, then `DELETE_COLUMN_STATS` of B | ID, A, C `stats:`; B `NO STATISTICS`; ALLNULL `stats:0:50` |
| T_STC4 | none (control); also a `GRANT` | `TAB\|T_STC4\|NULL\|NULL`; all five `NO STATISTICS` |
| T_STC5 | `ANALYZE TABLE ... COMPUTE STATISTICS FOR TABLE` | `TAB\|T_STC5\|50\|2026-10-07`; all five `NO STATISTICS` |

Truth: `truth_stc.txt` (the same bytes in both folders, prefix `bedf4f59e3dc`): `TAB|table|num_rows|last_analyzed`
and `COLUMN|table|column_id|column_name|stats:distinct:nulls` or `NO STATISTICS` (the query tests
`last_analyzed IS NOT NULL`). It has **no rows** and no `blocks`, `avg_row_len`,
`density`, low / high values: of a statistics item only `num_rows`, `num_distinct` and `num_nulls`
can be compared with a truth line. The 50 rows of each table are compared with what `10_stc.sql`
must produce ("sql", not truth): 200 fields per table (250 for T_STC2) in each of the 4 files, 0
differ (`tools/check_truth.py`).

Not made (they were in the request or would be useful, see `requests.md` R-023): an export of these
tables with a US7ASCII client, and one with `statistics=none`. (Session 7: made; see the next table.)

### Fixtures added in session 7

All are legacy `exp` of the same version as the database, table mode (`RTABLES`), user CORPUS,
database AL32UTF8 / AL16UTF16, RECORDLENGTH 8192 in the header, conventional path only (header has
no `D` LF). R-024 to R-028: SQL `corpus/sql/11_types.sql`, default client (AL32UTF8),
`statistics=none` (`corpus/scripts/make_types.sh`, read, not run). R-023: no new SQL; the tables of
`10_stc.sql`, `corpus/scripts/make_stc_extra.sh` (read, not run) writes `truth_stc_extra.txt`
with `corpus/sql/10b_stc_truth.sql` immediately before its two exports. Every log ends "Export
terminated successfully" (`with warnings` only for the two US7ASCII files: 5 `EXP-00091` lines
each).

| Fixture (`.dmp`) | Request | Tables, rows (log) | Client (log) | Options | 18c: SHA-256 prefix | 11g: SHA-256 prefix |
|---|---|---|---|---|---|---|
| `stc_exp_us7ascii_<v>` | R-023 | T_STC1 to T_STC5, 50 each | US7ASCII | `statistics=estimate` | `d6972111ec3f` | `2986ad0c997b` |
| `stc_exp_none_<v>` | R-023 | T_STC1 to T_STC5, 50 each | AL32UTF8 | `statistics=none` | `e154b6945227` | `242287a728f8` |
| `t_num2_exp_<v>` | R-024 | T_NUM2 14 (`ID NUMBER(3)`, `V NUMBER`) | AL32UTF8 | `statistics=none` | `33e8e0420545` | `a298789c52c1` |
| `t_decl_exp_<v>` | R-025 | T_DECL 1 (14 columns) | AL32UTF8 | `statistics=none` | `39a88ee59b85` | `0f69eacbf62b` |
| `nchar_exp_<v>` | R-026a | T_NCHAR 3 (`ID NUMBER(3)`, `NC NCHAR(5)`, `CC CHAR(4 CHAR)`, `NV NVARCHAR2(5)`) | AL32UTF8 | `statistics=none` | `5779e218f480` | `d5488a153a38` |
| `long_exp_<v>` | R-028 | T_LONG 3 (`ID NUMBER(3)`, `L LONG`, `V VARCHAR2(10)`), T_LONGRAW 2 (`ID NUMBER(3)`, `LR LONG RAW`), T_FRAME 7 | AL32UTF8 | `tables=t_long,t_longraw,t_frame`, `statistics=none` | `924c85881fb9` | `4d7098afb987` |

Truth: `truth_num2.txt` (14 lines `id|TO_CHAR(v,'TM9')|DUMP(v,16)`, prefix `a9625274cc62`),
`truth_decl.txt` (14 lines `column_id|name|data_type|precision|scale|data_length|nullable`,
`d14b2983dce9`), `truth_nchar.txt` (3 lines `id|DUMP(nc,16)|DUMP(cc,16)|DUMP(nv,16)`,
`9a85a30d3151`), `truth_csmap.txt` (768 lines `set|byte|DUMP(UTL_I18N.RAW_TO_NCHAR(byte, set),16)`,
`2a7e7a0c4614`; R-026b, no dump), `truth_stc_extra.txt` (the format of `truth_stc.txt`,
`d5a61b7d13f9`). Each of the five is byte-identical in `18c/` and `11g/`. T_DECL, T_LONG and
T_LONGRAW have no truth rows: their values are compared with `11_types.sql` ("sql", not truth);
the row counts agree with the logs. The six files of R-024 to R-028 are byte-identical between 18c
and 11g from offset R = 0x2000 on (`tools/version_diff.py`: "bytes differing from offset R on: 0"
for `t_num2_exp_`, `t_decl_exp_`, `nchar_exp_`, `long_exp_`), so every offset of those files below
holds for both versions; they are two exports from two databases, not one file.

### Fixtures added in session 8

All are legacy `exp` of the same version as the database, table mode (`RTABLES`), user CORPUS,
database AL32UTF8 / AL16UTF16, default client (AL32UTF8, every log: "Export done in AL32UTF8
character set and AL16UTF16 NCHAR character set"), RECORDLENGTH 8192 in the header,
`statistics=none`. SQL: `corpus/sql/01b_optional_tables.sql` (T_LOB, T_INTERVAL, T_PART),
`corpus/sql/13_optional.sql` (T_LOB2, T_ROWID and the truth files), `corpus/sql/13b_part_truth.sql`
(`truth_part.txt`); script `corpus/scripts/make_optional.sh` (read, not run). 11g has no partition
fixture (11g XE has no Partitioning option). Every log ends "Export terminated successfully without
warnings".

| Fixture (`.dmp`) | Request | Tables, rows (log) | Path (log) | 18c: SHA-256 prefix | 11g: SHA-256 prefix |
|---|---|---|---|---|---|
| `lob_exp_<v>` | R-027 | T_LOB 5 (`ID NUMBER(3)`, `C CLOB`, `B BLOB`), T_LOB2 2 (`ID NUMBER(3)`, `A CLOB`, `V VARCHAR2(10)`, `N NCLOB`, `B BLOB`), T_FRAME 7 | conventional | `1cb1921e123e` | `8891213bb650` |
| `lob_exp_direct_<v>` | R-027 | the same | `direct=y`; the log says "Table T_LOB will be exported in conventional path." and the same for T_LOB2; T_FRAME direct | `8cb8a0888522` | `a85ae6ee6642` |
| `misc_exp_<v>` | R-029 | T_INTERVAL 4 (`ID NUMBER(3)`, `IYM INTERVAL YEAR(4) TO MONTH`, `IDS INTERVAL DAY(4) TO SECOND(6)`), T_ROWID 2 (`ID NUMBER(3)`, `RID ROWID`, `URID UROWID`), T_FRAME 7 | conventional | `1a7e6fc08325` | `a9adf27c5eac` |
| `part_exp_18c` | R-030 | T_PART (`ID NUMBER(6)`, `SALE_DATE DATE`, `AMOUNT NUMBER(10,2)`; range partitions P2019, P2020, PMAX): the log has one line "exporting table T_PART" and three lines "exporting partition P2019 1095 rows", "P2020 1098", "PMAX 807" | conventional | `fadd70224ff8` | - |
| `part_exp_direct_18c` | R-030 | the same | direct | `09256629e070` | - |

Truth: `truth_lob.txt` (7 lines; `T_LOB|id|DBMS_LOB.GETLENGTH(c)|GETLENGTH(b)|RAWTOHEX(DBMS_LOB.SUBSTR(b,30,1))`
and `T_LOB2|id|GETLENGTH(a)|v|GETLENGTH(n)|GETLENGTH(b)`; prefix `1e4feb760d3c`, the same in both
folders), `truth_misc.txt` (6 lines; `INTERVAL|id|TO_CHAR(iym)|DUMP(iym,16)|TO_CHAR(ids)|DUMP(ids,16)`
and `ROWID|id|ROWIDTOCHAR(rid)|DUMP(rid,16)|DUMP(urid,16)`; 18c `99245414cacd`, 11g `b0cb1a55c0f5`:
they differ only in the ROWID), `truth_part.txt` (4 lines, `P|partition|rows` and
`TOTAL|count|sum(amount)|min(date)|max(date)`; `553baa9e3d4c`). The truth gives LOB lengths and the
first 30 bytes of B only; the whole LOB values are compared with the SQL ("sql", not truth). The row
counts of the logs agree with the truth in every file. `misc_exp_<v>` 18c against 11g: equal from
offset R on except 10 bytes inside the two ROWID values (`tools/version_diff.py`). `lob_exp_<v>`
18c against 11g: the binary parts of the two LOB tables differ (locators, chunk sizes; 14.6), the
T_FRAME sections are equal.

### Fixtures added in session 9

All are legacy `exp` of the same version as the database, table mode (`RTABLES`), user CORPUS,
database AL32UTF8 / AL16UTF16, RECORDLENGTH 8192 in the header, client AL32UTF8 ("Export done in
AL32UTF8 character set and AL16UTF16 NCHAR character set") except `long2_exp_us7ascii_<v>` ("Export
done in US7ASCII character set ... server uses AL32UTF8 character set (possible charset conversion)"),
`statistics=none` except `part2_*` (default statistics). SQL: `corpus/sql/14_num3.sql`,
`14_nchar2.sql`, `14_probe11.sql`, `14_lob3.sql`, `14_misc2.sql`, `14_long2.sql`, `14_part2.sql`;
script `corpus/scripts/make_extra.sh` (read, not run). Every log ends "Export terminated successfully
without warnings". 11g has no `part2` (no Partitioning option in 11g XE).

| Fixture (`.dmp`) | Request | Tables, rows (log) | Path (log) | 18c: SHA-256 prefix | 11g: SHA-256 prefix |
|---|---|---|---|---|---|
| `t_num3_exp_<v>` | R-032 | T_NUM3 6 (`ID NUMBER(3)`, `V NUMBER`) | conventional | `4be18b8743e2` | `2cdad9f67813` |
| `nchar2_exp_<v>` | R-033 | T_NCHAR2 1 (`ID`, `NC NCHAR(3)`, `NV NVARCHAR2(3)`, `CC CHAR(3 CHAR)`, `VC VARCHAR2(3 CHAR)`, `CL CLOB`, `NCL NCLOB`), T_FRAME 7 | conventional | `236423d207d5` | `7859874f8bde` |
| `t_probe11_exp_<v>` | R-034 | T_PROBE11 2 (`ID`, `B VARCHAR2(10 BYTE)`, `C VARCHAR2(10 CHAR)`, `K CHAR(3 CHAR)`, `W VARCHAR2(1000 CHAR)`, `X VARCHAR2(1001 CHAR)`, `Y CHAR(500 CHAR)`, `Z CHAR(501 CHAR)`), T_FRAME 7 | conventional | `027755edbc8c` | `b00b9bd4788b` |
| `t_probe11_exp_direct_<v>` | R-034 | the same | direct | `42c57d02ceb6` | `a0b455036b2f` |
| `lob3_exp_<v>` | R-035 | T_LOB3 12 (`ID`, `C CLOB`, `B BLOB`, `N NCLOB`), T_FRAME 7 | conventional | `9bd643e0c776` | `1c3411a2a4ac` |
| `misc2_exp_<v>` | R-036 | T_INTERVAL2 3 (`ID`, `IYM INTERVAL YEAR(9) TO MONTH`, `IDS INTERVAL DAY(9) TO SECOND(9)`, `IDS0 INTERVAL DAY(2) TO SECOND(0)`), T_ROWID2 4 (`ID`, `RID ROWID`, `URID UROWID`), T_FRAME 7 (T_IOT is not exported; its ROWID is in T_ROWID2 row 10) | conventional | `f2f03ead4665` | `348fb88cfc67` |
| `long2_exp_<v>` | R-031 | T_LONG2 1 (`ID`, `L LONG`, `V VARCHAR2(10)`; row 2 was refused by Oracle, ORA-22835), T_FRAME 7 | conventional | `b9d8f238249f` | `5cc70b0bc22a` |
| `long2_exp_direct_<v>` | R-031 | the same | direct ("About to export specified tables via Direct Path"; no "conventional path" line for T_LONG2) | `3bc9f17dd57b` | `45555224d483` |
| `long2_exp_us7ascii_<v>` | R-031 | the same | conventional, client US7ASCII | `cf64b6129df6` | `c8b94b792cfc` |
| `long_exp_direct_<v>` | R-031 | T_LONG 3, T_LONGRAW 2, T_LONG_EMPTY 0 (`L LONG`, `V VARCHAR2(10)`), T_FRAME 7 | direct (no table switched to the conventional path) | `b141a98ae919` | `ec771e102c6b` |
| `part2_exp_18c` | R-037 | T_PART2 (`ID NUMBER(6)`, `K NUMBER(3)`, `V VARCHAR2(10)`; range on ID: P1, P_EMPTY, P3; hash subpartitions on K, 2 each): log "exporting table T_PART2", "exporting composite partition P1", "exporting subpartition SYS_SUBP368 21 rows", SYS_SUBP369 29, P_EMPTY: SYS_SUBP370 0, SYS_SUBP371 0, P3: SYS_SUBP372 21, SYS_SUBP373 29; T_FRAME 7 | conventional | `98407eec5e69` | - |
| `part2_exp_direct_18c` | R-037 | the same | direct | `26eb62536e0d` | - |

Truth (SHA-256 prefix): `truth_num3.txt` 6 lines `id|TO_CHAR(v,'TM9')|DUMP(v,16)` (`3e323ca5301f`, both
folders); `truth_nchar2.txt` 1 line `id|DUMP(nc)|DUMP(nv)|DUMP(cc)|DUMP(vc)|GETLENGTH(cl)|GETLENGTH(ncl)|
DUMP(SUBSTR(cl,10,1))|DUMP(SUBSTR(ncl,10,1))` (`bfb677ebd01f`, both); `truth_probe11.txt` 8 `COL|name|
data_type|data_length|char_length|char_used` + 2 `ROW|id|DUMP(b)|DUMP(c)|DUMP(k)|DUMP(w)|DUMP(x)|
LENGTHB(y)|LENGTHB(z)` (`b10ec5eae33e`, both); `truth_lob3.txt` 12 lines `T_LOB3|id|GETLENGTH(c)|
GETLENGTH(b)|GETLENGTH(n)|DUMP(SUBSTR(c,20,1),1016)` (`dd5d7091ce7d`, both); `truth_misc2.txt` 3
`INTERVAL|...` + 4 `ROWID|id|ROWIDTOCHAR(rid)|DUMP(rid)|CAST(urid AS VARCHAR2(4000))|DUMP(urid)` (18c
`e3b441852d20`, 11g `671499252f99`: only the ROWIDs differ); `truth_long2.txt` 2 lines, `1|10|D0B4D0B6
20436166C3A9` and `2|INSERT REFUSED|ORA_22835 ...` (`50974906119a`, both); `truth_part2.txt` 6 `SP|
partition|subpartition` + 100 `ROW|id|subpartition` (`f8dee8cbb89a`). The row counts of the logs
agree with the truth in every file. 18c against 11g (`tools/version_diff.py`): `t_num3`, `t_probe11`
(both paths), `long2` (all three) are byte-identical from offset R on; `nchar2` and `lob3` differ in
the LOB binary part (locators; `lob3` chunk sizes, 14.6), `misc2` in the ROWID values (39 bytes),
`long_exp_direct` in T_LONG_EMPTY (`ff ff` on 11g, 5.4).

**Limit of "confirmed" in session 9.** Computed NUMBER values: 6 values, 1 column, 2 versions,
conventional path. Character semantics: 7 columns of one table (AL32UTF8) on 2 versions and 2
paths, plus the CHAR / VARCHAR2 columns of `nchar`, `nchar2`; non-BMP: one character (U+1F600) in one
row of 6 columns. LOB sizes: 1 table, 11 sizes per CLOB / BLOB column, 2 versions, conventional path,
AL32UTF8 only (a CLOB in a single-byte database with these sizes, and NCLOB long values: not in the
fixtures). Intervals and ROWIDs: 1 table each, 3 rows / 4 rows, 2 versions. LONG on the direct path:
3 tables (5 non-NULL values, 2 of them 32,000 bytes), 2 versions; non-ASCII LONG: one 10-byte value
with 2 client sets. Composite partitions: 1 table (range-hash), 18c only, 2 paths, with statistics.
Not covered: a LONG value above 32,000 bytes (R-040), a LONG table on the direct path in a
single-byte database, list / interval / reference partitioning, subpartition statistics, a
partitioned table in owner mode or with indexes.

**Limit of "confirmed" in session 8.** LOB columns: 2 tables (5 LOB columns) on 2 versions, 1
database set (AL32UTF8) and 1 client set for CLOB / NCLOB / BLOB, plus R_CLOB of session 4 (1 CLOB
column, CL8MSWIN1251, 7 files, 4 client sets); LOB values up to 200,000 bytes; LOB tables are always
written on the conventional path. INTERVAL, ROWID, UROWID: 1 table each, 2 versions, conventional
path, 3 non-NULL values per interval column, 1 non-NULL ROWID / UROWID per version. Partitions: 1
range-partitioned table, 18c only, both paths, no statistics, table mode. Not covered: a LOB in a
single-byte database other than CL8MSWIN1251, NCLOB / BLOB in a single-byte database, a LOB value
between 6,000 and 100,000 bytes (where the long form starts), a CLOB with non-ASCII characters in
AL32UTF8, BFILE; INTERVAL values with mixed fields (negative with hours), other precisions; a UROWID
of an index-organised table; subpartitions, partitioned tables in owner mode, an empty partition,
partition statistics.

**Limit of "confirmed" in session 7.** As after session 6. The new types (NCHAR, `CHAR(n CHAR)`,
LONG, LONG RAW) and the declared forms rest on one table each, 2 versions, the conventional path
only, client = database set (AL32UTF8). Not covered: these types on the direct path, with another
client set, in a single-byte database; a LONG value above 32,000 bytes.

**Limit of "confirmed" in session 6.** As after session 5. For "which column gets a statistics
item": 5 tables, 26 columns, 2 versions, 2 paths, one database character set, one client set,
`exp` of the same version as the database. Not covered: an `exp` client older than the database
(the real dump), partitioned tables, hidden or virtual columns, extended statistics.

**Limit of "confirmed" in session 5.** As after session 4 (below), and for the statistics items:
6 tables, 2 versions, 2 databases, statistics made by `DBMS_STATS` and by `ANALYZE`. Not covered:
a table in which only some columns have statistics, a histogram with endpoint actual values that
`exp` writes out, partitioned tables, and again an `exp` client older than the database.

**Limit of "confirmed" after session 4.** Two Oracle versions (18c XE 18.4, 11g XE 11.2.0.2), two
database character sets (AL32UTF8 on both versions, CL8MSWIN1251 on 11g only), three users, two
export modes, `exp` of the same version as the database, one host. Not covered: an `exp` client
older than the database (the project's real dumps are said to be written by an `exp` 10.2 client
against an 11.2.0.4 server: no fixture, nothing here is evidence for that combination).

### Fixtures added in session 10

Folder `corpus/fixtures/11g-cp1251/`: Oracle 11g XE 11.2.0.2 (log: "Oracle Database 11g Express Edition Release
11.2.0.2.0 - 64bit Production"), database `ora11ru`, database set CL8MSWIN1251 (id 171), national set AL16UTF16,
user CORPUS_RU, legacy `exp` 11.2 (version line `EXPORT:V11.02.00`), table mode (`RTABLES`), conventional path,
`statistics=none`, default RECORDLENGTH (8192 in the header text @0x27). SQL `corpus/sql/15_ru_lob2.sql`, script
`corpus/scripts/make_ru_lob2.sh` (read, not run). Both logs: "exporting table R_LOB2 3 rows exported", "R_TEXT 309 rows
exported", "Export terminated successfully without warnings".

| Fixture (`.dmp`) | Request | Tables, rows (log) | Client set (log; header bytes 1-2) | SHA-256 prefix |
|---|---|---|---|---|
| `ru_lob2_exp_cp1251_11g` | R-038 | R_LOB2 3 (`ID NUMBER(3)`, `C CLOB`, `N NCLOB`, `B BLOB`), R_TEXT 309 | CL8MSWIN1251 ("Export done in CL8MSWIN1251 character set and AL16UTF16 NCHAR character set"; `00 ab`) | `f76087903815` |
| `ru_lob2_exp_us7ascii_11g` | R-038 | the same | US7ASCII ("Export done in US7ASCII character set ... server uses CL8MSWIN1251 character set (possible charset conversion)"; `00 01`) | `594f9c80c2c1` |

Truth: `truth_lob_ru.txt` (`5254de5dac6c`), 3 lines `R_LOB2|id|GETLENGTH(c)|GETLENGTH(n)|GETLENGTH(b)|DUMP head C|DUMP
head N|DUMP tail C|DUMP tail N` (4 characters each): `R_LOB2|1|66|66|66|...c0,c1,c2,c3|...4,10,4,11,4,12,4,13|...fe,ff,a8,b8|
...4,4e,4,4f,4,1,4,51`, `R_LOB2|2|20000|20000|20000|...|...a8,b8,c0,c1|...4,1,4,51,4,10,4,11`, `R_LOB2|3|NULL|...`. R_TEXT is
the table of R-015 and is compared with `truth_ru.txt`.

**Limit of "confirmed" in session 10.** LOB values in a single-byte database: 1 table (CLOB, NCLOB, BLOB), 1 database
set (CL8MSWIN1251), 1 version (11g XE 11.2.0.2), conventional path, 2 client sets (CL8MSWIN1251, US7ASCII), 2 non-NULL
values per column (66 and 20,000 characters / bytes), Cyrillic letters 0xC0 to 0xFF, 0xA8, 0xB8 only. Not covered: 18c
with a single-byte database (no such fixture; the chunk there is unknown), a database set WE8MSWIN1252 or US7ASCII, a
CLOB or NCLOB between 67 and 19,999 characters in this database (where the short form ends: R-046), byte 0x98 (undefined
in code page 1251, C7) inside a LOB, an `exp` client older than the database (R-022).

### Fixtures added in session 11

No dump. Folder `corpus/fixtures/18c/`, one file, made by the owner with `corpus/scripts/run_r045.sh` (read, not run) from
the R-045 SQL of `requests.md`; Oracle 18c XE (spool line 5: "Oracle Database 18c Express Edition Release 18.0.0.0.0 -
Production"), user `CORPUS` (assumed: the script connects with the corpus connect string of `config.sh`; the spool has no
`SHOW USER`).

| File | Request | Content (line numbers of the file) | SHA-256 prefix |
|---|---|---|---|
| `truth_r045.txt` (40 lines) | R-045 | line 1 `=== R-045 2026-10-17T13:48:50Z ===`; line 5 the banner; lines 10 to 13: `CREATE TABLE t_plong (id NUMBER(3), l LONG)` -> `ORA-14085: partitioned table cannot have column with LONG datatype`; lines 16 to 19: `CREATE TABLE t_plongraw (id NUMBER(3), lr LONG RAW)` -> the same `ORA-14085` (for LONG RAW too, with the same message text); line 23 `Table created.` (statement 3, the partitioned `t_plong2` without LONG; not echoed); lines 25 to 28: `ALTER TABLE t_plong2 ADD (l LONG)` -> `ORA-14085`; line 32 `Table dropped.` (statement 5); lines 34 to 39 "what Oracle accepted": two `no rows selected` (by the script: `user_tables` and `user_tab_columns` for `T_PLONG%`, so no such table or column was left) | `b5fb62d1b3a0` |

Result (observed on 18c, one database; consistent with Oracle's error documentation quoted in R-045): a partitioned table
cannot have a LONG or a LONG RAW column, at `CREATE TABLE` and at `ALTER TABLE ... ADD`, while the same partitioned table
without it is created (line 23). So no 18c dump can hold a partitioned table section [T10] whose descriptors include type 8
or 24; the direct-path "LONG last" order [T3] can meet a partition only in a dump Oracle cannot write. 11g: not run
(assumed the same). No dump bytes are involved, so no offset applies.

## How to read the status column

- `confirmed`: seen in at least two independent fixtures (different table, or different export
  option) and in agreement with a truth file or an export log.
- `observed`: the bytes are there, but either only one fixture shows them or nothing in the
  truth says what they mean. Offset and bytes are facts; the meaning is a description only.
- `hypothesis`: a meaning that the data suggest and that was not tested.
- `unknown`: bytes with no explanation.

**Limit of "confirmed".** In sections 1 to 9, `confirmed` in a row written before session 4 means
"confirmed for 18c in the fixtures of that session" (one database, AL32UTF8 / AL16UTF16, one user,
table mode). Rows marked "Session 4" and sections 4.5, 13, 14 say which version and database they
rest on. See "Limit of confirmed after session 4" above.

"u16 LE" = two bytes, low byte first. "u16 BE" = two bytes, high byte first. The header and the
two blocks of section 3 use BE; the table data use LE (both shown below with values above 255).

## 1. Regions

The file is one continuous byte stream, cut into records of RECORDLENGTH bytes only in the sense
that (a) the header is padded with zero bytes to one full record and (b) the end of the file is
padded with zero bytes to a multiple of RECORDLENGTH. Offsets below are for CONV (RECORDLENGTH
8192 = 0x2000); `R` stands for RECORDLENGTH.

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x0000-0x009c | 0x80 + file name length | file header | section 2 | see section 2 |
| 0x009d-0x1fff | to R-1 | zero fill to the end of the first record | CONV@0x009d-0x1fff; `t_frame_exp_rl4096`@0x00a4-0x0fff; `t_frame_exp_rl16384`@0x00a5-0x3fff; all 38 EXP fixtures (`tools/header_fields.py`) | confirmed |
| R + 0x00 | 204 | two binary blocks of 102 bytes | CONV@0x2000, 0x2066; `t_frame_exp_rl4096`@0x1000, 0x1066; `t_frame_exp_rl16384`@0x4000, 0x4066 | observed, mostly unknown (section 3) |
| R + 0xcc | 56 | 8 strings with u16 LE length, then LF | CONV@0x20cc | observed (section 3) |
| R + 0x104 | var | text line `METRICST` + LF, then one section per table (section 4, 5) | CONV@0x2104; `multi_framing`@0x2104 | confirmed as text |
| per table | var | `TABLE "name"` LF, `CREATE TABLE ...` LF, (other items, section 4.2), `INSERT INTO ...` LF, column count, descriptors, 2 x u16, rows, end marker, LF, (other items: indexes, constraints, comments) | CONV@0x210d-0x22f5; `multi_framing`@0x210d, 0x22f6, 0x3469; `ddl`@0x2369-0x2a6c | confirmed (sections 4, 5) |
| after the last table | var | trailer text lines, `EXIT` LF `EXIT` LF | CONV@0x22f6-0x23ef | observed (section 7) |
| to end of file | var | zero fill to the next multiple of R | CONV@0x23f0-0x3fff; `t_probe6`@0xa6a5-0xbfff (file 0xc000 = 6 x 0x2000); `t_frame_exp_rl4096`@0x13f0-0x1fff (file 0x2000 = 2 x 0x1000) | confirmed |

### 1.1 RECORDLENGTH

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x0024 | 4 or 5 + LF | decimal text of RECORDLENGTH: `8192`, `4096`, `16384` | CONV@0x0024 `8192`; `t_frame_exp_rl4096`@0x0024 `4096`; `t_frame_exp_rl16384`@0x0024 `16384`; values = the `recordlength=` argument in `corpus/scripts/make_probes.sh` lines 78-79 (script read, not run) | confirmed |
| R | - | the second record starts at file offset R; everything from there to the last `EXIT` is identical in the three files (0x3f0 bytes) | CONV@0x2000-0x23ef = `t_frame_exp_rl4096`@0x1000-0x13ef = `t_frame_exp_rl16384`@0x4000-0x43ef | confirmed |
| - | - | file size = smallest multiple of R that holds the data | all 38 fixtures (`tools/header_fields.py`, "invariant violations: none") | confirmed |

The text line has a variable width (`16384` is 5 characters), so every header field after it
moves by one byte in `t_frame_exp_rl16384` (binary fields at 0x0031 instead of 0x0030).

### 1.2 Record boundaries inside row data

`t_probe6` has 0x8342 bytes of row data (0x225e-0xa59f) and crosses four boundaries.

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x4000, 0x6000, 0x8000, 0xa000 | 0 | nothing is inserted at a record boundary: no padding, no header, no marker. A value simply continues | `t_probe6`@0x4000: row R00132, column V, field 0x3fb6-0x401b, 72 value bytes before and 28 after the boundary; @0x6000: row R00205 col V (88 / 12); @0xa000: row R00052 col V (8 / 92). `t_probe6/direct`@0x4000: row R00133 col V, field 0x3f9e-0x4003 (96 / 4); @0x6000 (38 / 62); @0x8000 (90 / 10); @0xa000 (32 / 68). All 300 rows decode to `truth_probe6.txt` (600 fields equal, `tools/check_truth.py`) | confirmed |
| 0x7ffe | 2 + 6 | a boundary can fall between a length field and its value | `t_probe6`@0x7ffe `06 00`, value `R00279` at 0x8000 | observed (one case) |
| 0x3fff, 0x4000 | 1 + 1 | Session 4 (R-013): a boundary **between the two bytes of a length field**. The low byte is the last byte of a record, the high byte the first byte of the next; nothing stands between them and the value follows at once | conventional: `t_bn1`@0x3fff `eb`, @0x4000 `04` (row 7, column R, length 1259 = 0x04eb; the row starts at 0x3ffb with `02 00 c1 08`; value 0x4001-0x44eb, 1259 x `a7`). Direct: `t_bn2/direct`@0x3fff `ed`, @0x4000 `04` (row 7, length 1261). 11g, same offsets and bytes: `t_bn1_exp_11g.dmp`@0x3fff, `t_bn2_exp_direct_11g.dmp`@0x3fff. RECORDLENGTH 2048: `ru_exp_cp1251_11g.dmp`@0x7fff `17`, @0x8000 `00` (R_TEXT, column V of the 82nd row written, length 23) | confirmed (2 paths, 2 versions, 2 record lengths) |
| 0x3fff, 0x4000 | 1 + 1 | Session 4 (R-013): a boundary **inside an `fe ff` NULL marker** | conventional: `t_bn3`@0x3fff `fe`, @0x4000 `ff` (row 30, column ID; the A field follows at 0x4001 `00 01` + 256 bytes). Direct: `t_bn4/direct`@0x3fff `fe`, @0x4000 `ff` (row 30; A at 0x4001 `02 01` + 258 bytes). 11g: `t_bn3_exp_11g.dmp`@0x3fff, `t_bn4_exp_direct_11g.dmp`@0x3fff, same bytes | confirmed |
| 0xdfff, 0xe000 | 1 + 1 | Session 4: a boundary inside the `00 00` that ends a conventional-path row | `ru_exp_cp1251_11g.dmp`@0xdfff `00`, @0xe000 `00` (after the 199th row of R_TEXT; RECORDLENGTH 2048). The same bytes stand at the same offsets in the three other conventional `ru_exp_*` files | observed (one position) |
| - | - | Session 4: the other path of each R-013 table is a control: there the boundary falls inside a value | `t_bn1/direct`@0x4000 (row 7, value 0x3ff5-0x44df); `t_bn2`@0x4000 (row 6, value 0x3b18-0x4004); `t_bn3/direct`@0x4000 (row 30, value 0x3fc9-0x40c8); `t_bn4`@0x4000 (row 29, value 0x3f35-0x4036) | confirmed |
| - | - | Session 4: every row of the 16 R-013 files decodes to the truth with the continuous walk (no byte skipped, none read twice): T_BN1, T_BN2: 8 rows, ids 1 to 8, R of 1259 / 1261 bytes; T_BN3, T_BN4: 32 rows, ID NULL, A of 256 / 258 bytes. The first rows start at 0x2249 (T_BN1, T_BN2) and 0x2251 (T_BN3, T_BN4) in all 16 files, as designed | `tools/check_truth.py`, `tools/boundary.py`. **The truth files give the length and the first 4 bytes (first 5 characters) of each value only**; the remaining bytes were compared with what `corpus/sql/06_bn.sql` must produce (all bytes equal to the first one; all `x`), which is a comparison with the SQL, not with a truth file | confirmed for lengths and first bytes (truth); confirmed against the SQL for the rest |
| every multiple of 2048 | 0 | Session 4: RECORDLENGTH 2048. In each of the four conventional `ru_tbl_exp_*` files 33 boundaries fall inside the row data of R_TEXT (32 inside a value, 1 exactly between a length field and its value); all 309 rows equal the truth | `tools/boundary.py -q` on `corpus/fixtures/11g-cp1251/`; e.g. `ru_tbl_exp_cp1251_11g.dmp`, rows 0x0b10-0x116ae | confirmed |

**Result (R-013, rule R5).** Nothing is inserted, repeated or changed at a multiple of
RECORDLENGTH: not for a value, not for a length field, not for an `fe ff` marker, not for a
`00 00` row end, on either path, on 18c and 11g, for RECORDLENGTH 8192 and 2048. The same holds
inside a field of the text region (section 4.5: `own`@0x3fc2). Open question 3 is closed.

Consequence for a reader: after the first record, RECORDLENGTH can be ignored; the
only use of R is "the data start at offset R". A reader must not process records separately.

## 2. File header (first record)

Offsets for CONV. In a direct-path file the two bytes `D` LF are inserted at 0x0003, so all
fields from `EXPORT:` to the file-name-length field are 2 bytes later; the date and the file name
are **not** shifted.

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x0000 | 1 | `03` | all 38 fixtures @0x0000 | observed; meaning unknown |
| 0x0001 | 2 | u16 BE: id of the **client** character set of the export session (the set named in the log line "Export done in ... character set") | CONV@0x0001 `03 69` = 873 (log: AL32UTF8); `t_frame_exp_us7ascii`@0x0001 `00 01` = 1 (log: US7ASCII); `t_frame_exp_w1252`@0x0001 `00 b2` = 178 (log: WE8MSWIN1252); same three values in the three direct files @0x0001; ids agree with `truth_charsets.txt` | confirmed |
| (0x0003) | 2 | direct path only: text `D` + LF | DIRECT@0x0003 and all 18 other `*_exp_direct_*` fixtures; absent in all 19 conventional fixtures; logs say "via Direct Path" / "via Conventional Path" | confirmed |
| 0x0003 | 17 | text `EXPORT:V18.00.00` + LF | CONV@0x0003; DIRECT@0x0005 | confirmed as text (`truth_env.txt` version 18.0.0.0.0) |
| 0x0014 | 8 | text `UCORPUS` + LF | CONV@0x0014 | observed in session 2. Session 4: confirmed as `U` + exporting user with two more users: `own`@0x0014 `UCORPUS_OWN` (log: "About to export CORPUS_OWN's objects"), `ru_tbl_exp_cp1251_11g.dmp`@0x0014 `UCORPUS_RU`. The line has a variable length, so everything after it moves |
| 0x001c | 8 | text `RTABLES` + LF | CONV@0x001c | observed in session 2. Session 4: confirmed as `R` + mode for two modes: `RTABLES` in every table-mode file (`own_tables`@0x0020), `RUSERS` in every owner-mode file (`own`@0x0020, `own_11g`@0x0020, `ru_exp_cp1251_11g.dmp`@0x001f); the logs say "About to export specified tables" / "About to export CORPUS_OWN's objects". Other modes: not in the fixtures |
| 0x0024 | var | RECORDLENGTH as text + LF | section 1.1 | confirmed |
| 0x0029 | 2 | text `0` + LF | CONV@0x0029 | unknown (same in all 38 fixtures) |
| 0x002b | 3 | text `72` + LF | CONV@0x002b | unknown (same in all 38 fixtures) |
| 0x002e | 2 | text `0` + LF | CONV@0x002e | unknown (same in all 38 fixtures) |
| 0x0030 | 2 | u16 BE: client character set id again (always equal to 0x0001) | CONV@0x0030 `03 69`; `t_frame_exp_us7ascii`@0x0030 `00 01`; `t_frame_exp_w1252`@0x0030 `00 b2`; direct files @0x0032 with the same three values | confirmed |
| 0x0032 | 2 | `03 69` = 873. Did **not** change when the client character set changed | CONV@0x0032; `t_frame_exp_us7ascii`@0x0032 `03 69`; `t_frame_exp_w1252`@0x0032 `03 69`; direct @0x0034 | observed. Hypothesis: database character set id (873 = AL32UTF8 = `NLS_CHARACTERSET` in `truth_env.txt`; the us7ascii / w1252 logs say "server uses AL32UTF8"). Not testable: one database only. Session 3: it is also the set the VARCHAR2 / CHAR value bytes are written in, for all three client sets (`t_text_exp_w1252`@0x0032 `03 69`, `t_text_exp_us7ascii`@0x0032 `03 69`; section 2.1). **Session 4: confirmed as the database character set.** In the 15 files of the CL8MSWIN1251 database the field is `00 ab` = 171 for all four client sets (`ru_tbl_exp_us7ascii_11g.dmp`@0x0035, `ru_tbl_exp_al32utf8_11g.dmp`@0x0035, `ru_exp_w1252_11g.dmp`@0x0034; `truth_ru.txt` `ENV\|NLS_CHARSET_ID(CL8MSWIN1251)\|171`); section 14.4. Its offset is 0x0032 only in files whose user line is `UCORPUS`, mode line `RTABLES` and RECORDLENGTH has 4 digits |
| 0x0034 | 2 | `07 d0` = 2000. Did not change | CONV@0x0034; `t_frame_exp_us7ascii`@0x0034; `t_frame_exp_w1252`@0x0034; direct @0x0036 | observed. Hypothesis: a national character set id (2000 = AL16UTF16). Client and database NCHAR set are both AL16UTF16 in every fixture, so which of the two it is cannot be told |
| 0x0036 | 10 | `00 01 00 00 00 00 00 00 00 00` | CONV@0x0036; same in all 38 fixtures | unknown |
| 0x0040 | 2 | u16 BE (reading assumed): length of the file name text at 0x0080 | CONV@0x0040 `00 1d` = 29; DIRECT@0x0042 `00 24` = 36; 38 fixtures with 18 different lengths from 25 to 45, each equal to the length of the text at 0x0080 (`tools/header_fields.py`) | confirmed for the low byte. That the field is 2 bytes BE is assumed: no file name is longer than 255 bytes |
| 0x0042 | 1 | `00` | CONV@0x0042; all fixtures | unknown |
| 0x0043 | var | spaces (0x20) up to the date text | CONV@0x0043-0x0068 (38); DIRECT@0x0045-0x0068 (36); `t_frame_exp_rl16384`@0x0044-0x0068 (37); `t_empty`@0x0043-0x0069 (39) | confirmed: the date text is right-aligned so that it ends at 0x007f |
| ends at 0x007f | 22-23 | export date as text, e.g. `Sun Oct 4 22:11:58 2026`; numbers are not zero-padded (`Sun Oct 4 22:13:8 2026`, 22 characters) | CONV@0x0069; `t_empty`@0x006a (`22:13:8`); `t_frame_exp_us7ascii`@0x006a (`23:44:4`); day agrees with the fixtures README (2026-10-04). **Session 6 (rule for a tool):** let P be the offset after the LF of the seventh text line (the eighth LF of a direct-path file, which has the line `D` first); P + 19 is the first byte after the three u16 ids, the 10 bytes, the u16 file-name length and one `00`. The bytes P + 19 to 0x7f are spaces (0x20) followed by exactly one text `Www Mmm D H:M:S YYYY`: single spaces, English three-letter day and month names, day of month, hour, minute and second **without a leading zero**, four-digit year. The text is **right-aligned: its last byte is at 0x7f**, so it starts at 0x80 minus its length: 0x69 (23 bytes, `stc_exp_18c.dmp`@0x0069 `Wed Oct 7 21:49:44 2026`), 0x6a (22 bytes, `ru_exp_cp1251_11g.dmp`@0x006a `Wed Oct 7 0:25:52 2026`) or 0x6b (21 bytes, `t_frame_exp_direct_us7ascii_11g.dmp`@0x006b `Mon Oct 5 1:17:7 2026`). It is not a C `ctime` text (that has 24 characters with padded fields) and has no LF or NUL at its end: the file name follows directly at 0x80. `tools/header_date.py`: 157 of 157 files match; 81 x 23 bytes, 68 x 22, 8 x 21; no field with a leading zero (one-digit hour in 39 files, minute in 15, second in 30). Session 7: 169 of 169 files match; 4 texts of 20 bytes, starting at 0x6c (e.g. `long_exp_11g.dmp`@0x006c `Thu Oct 8 0:9:9 2026`); a month day 8 (`Thu Oct 8`) is new | confirmed as place and format (157 files, 2 versions, 3 folders); the day agrees with the fixtures README for the first files (2026-10-04); there is no truth for the time of day. Not in the fixtures: a day of month with two digits (every export was made on the 4th to 7th), which would give 24 bytes from 0x68 (hypothesis). **Session 8: confirmed**, 24 bytes from 0x68 in 4 files (`lob_exp_18c.dmp`@0x0068 `Wed Oct 14 22:20:57 2026`; `tools/header_date.py`: 177 files, 0 problems). Still not in the fixtures: a month other than `Oct`; a client language other than AMERICAN |
| 0x0080 | n | file name as given to `exp`, e.g. `/fixtures/t_frame_exp_18c.dmp`; fixed offset 0x0080 in every file | all 38 fixtures: text at 0x0080 = `/fixtures/` + the fixture's file name | confirmed |

The names `AL32UTF8`, `US7ASCII`, `WE8MSWIN1252`, `AL16UTF16` do not occur as text in any dump
(`strings -n 3`); only the numeric ids do.

Session 4. (a) The offsets of this table are those of CONV. Every field after the user line moves
with the lengths of the user line, the mode line and the RECORDLENGTH text: a reader finds the
binary fields after the seventh LF-terminated text line, not at a fixed offset (section 14.4).
(b) Oracle 11g: the same layout with the text `EXPORT:V11.02.00` (section 13.7).
(c) The header of an owner-mode file differs from a table-mode file only in the mode line.

### 2.1 Character set ids: what changed with the client character set

Experiment: `t_frame` exported three times, differing only in `NLS_LANG` (AL32UTF8 / US7ASCII /
WE8MSWIN1252), on both paths. `cmp -l` of `t_frame_exp_us7ascii` against `t_frame_exp_w1252`
gives exactly these differences: 0x0002, 0x0031, 0x0041 and 0x0096-0x00a5 (file name), 0x20a3,
0x20a5. Nothing else in the file differs.

| field | AL32UTF8 client | US7ASCII client | WE8MSWIN1252 client | changed? | status |
|---|---|---|---|---|---|
| header 0x0001 (u16 BE) | 873 | 1 | 178 | yes, follows the client set | confirmed |
| header 0x0030 (u16 BE) | 873 | 1 | 178 | yes, follows the client set | confirmed |
| header 0x0032 (u16 BE) | 873 | 873 | 873 | no | session 4: confirmed, database set (171 in `11g-cp1251/`, section 14.4) |
| header 0x0034 (u16 BE) | 2000 | 2000 | 2000 | no | observed |
| block 1 +0x3c, +0x3e (0x203c, 0x203e) | 873, 2000 | 873, 2000 | 873, 2000 | no | +0x3c: session 4: confirmed, database set (171 in `11g-cp1251/`); +0x3e: observed |
| block 2 +0x3c, +0x3e (0x20a2, 0x20a4) | 873, 873 | 1, 1 | 178, 178 | yes, both follow the client set | confirmed (as "follows the client id"; what the two fields are for is unknown) |
| column descriptor +4 (u16 LE; 0x2263, 0x226b, 0x2273) | 873 | 873 | 873 | no | observed (see section 4.1) |
| row data 0x227b-0x22f4 | - | identical | identical | no | observed: `t_frame` holds only ASCII, so this does not show whether values are converted (see the `t_text` experiment below) |

So: 0x0001 / 0x0030 are the client (export session) character set; 0x0032 is not. The Scout's
lead "a legacy EXP file begins with `03 00 01`" (H-021) is the US7ASCII case:
`t_frame_exp_us7ascii`@0x0000 = `03 00 01`.

#### Session 3: the same experiment with non-ASCII text (`t_text`, R-008)

`t_text` (rows of `truth_text.txt`: Cyrillic, accented Latin, CJK, an emoji) exported with the
AL32UTF8, WE8MSWIN1252 and US7ASCII clients, on both paths: six files. Tool:
`tools/clientset_diff.py`. Complete list of the bytes that differ between `t_text_exp_18c.dmp`
and `t_text_exp_w1252_18c.dmp` (`cmp -l`):

| offset (hex) | AL32UTF8 client | WE8MSWIN1252 client | what it is | status |
|---|---|---|---|---|
| 0x0001-0x0002 | `03 69` | `00 b2` | client character set id | confirmed (section 2) |
| 0x0030-0x0031 | `03 69` | `00 b2` | client character set id again | confirmed |
| 0x0041 | `1c` | `22` | file name length (28 / 34) | confirmed |
| 0x006a-0x007a (7 of these bytes) | `Sun Oct 4 22:13:4 2026` | `Mon Oct 5 1:16:35 2026` | export date text | observed |
| 0x0095-0x00a1 | `18c.dmp` | `w1252_18c.dmp` | file name | confirmed |
| 0x20a2-0x20a5 | `03 69 03 69` | `00 b2 00 b2` | block 2 +0x3c, +0x3e | confirmed |
| 0x210d-0x258f (1155 bytes) | - | **no difference** | the whole table section of T_TEXT: `TABLE` line, `CREATE TABLE` text, `INSERT INTO` line, column count, descriptors, the two u16, all rows, `ff ff`, LF | confirmed |
| 0x2590 to the last `EXIT` | - | no difference | trailer | confirmed |

The same holds for the other three comparisons: `t_text_exp_us7ascii_18c.dmp` differs from
`t_text_exp_18c.dmp` only at 0x0001-0x0002 (`00 01`), 0x0030-0x0031, 0x0041, the date, the file
name and 0x20a2-0x20a5 (`00 01 00 01`); the two direct files differ from
`t_text_exp_direct_18c.dmp` only at 0x0001-0x0002, 0x0032-0x0033, 0x0043, the date, the file
name and 0x20a2-0x20a5, and their table section 0x210d-0x257b (1135 bytes) is identical.

Result, observed in these six files (18c, AL32UTF8 database, table mode):

| statement | evidence | status |
|---|---|---|
| The client character set changes **nothing** in the table section: not the value bytes, not the length fields, not the descriptors, not the path flag, not the `CREATE TABLE` / `INSERT INTO` text | table above; `tools/clientset_diff.py` on the four pairs | confirmed (2 client sets x 2 paths) |
| VARCHAR2 and CHAR value bytes are the database (AL32UTF8) bytes of `truth_text.txt` for every client set. No character is replaced, no byte count changes | section 6.3; `tools/check_truth.py`: 30 fields equal the truth in each of the six files | confirmed |
| The ids at 0x0001, 0x0030 and in block 2 are therefore **not** the character set of the row data when the client set differs from the set in the descriptor | `t_text_exp_w1252`@0x0001 `00 b2` while row 3 V at @0x230c is `43 61 66 c3 a9 ...` (UTF-8, 16 bytes) | confirmed |
| Whether the text items (`CREATE TABLE`, comments, ...) are converted to the client set | session 3: every text item in every fixture is pure ASCII, so the files cannot show it. **Session 4: yes, they are** (section 14.2: `own`@0x2ce3 against `own_exp_w1252_18c.dmp`@0x2ce3) | confirmed in session 4 |
| What happens when the **database** character set is not AL32UTF8, or is a single-byte set and the client set is another one | session 4: the same: row data are the database bytes (CL8MSWIN1251), for four client sets (section 14.1) | confirmed in session 4 |

The logs of the four new exports say (both lines in each of `t_text_exp_w1252_18c.log`,
`t_text_exp_direct_w1252_18c.log`, `t_text_exp_us7ascii_18c.log`,
`t_text_exp_direct_us7ascii_18c.log`, with `US7ASCII` in the last two):

    Export done in WE8MSWIN1252 character set and AL16UTF16 NCHAR character set
    server uses AL32UTF8 character set (possible charset conversion)

and end with "Export terminated successfully without warnings." There is no line about lost or
replaced characters. The log of the AL32UTF8 export has the first line only (with `AL32UTF8`).
So the log announces a *possible* conversion; in the row data of these files none took place.

## 3. Start of the second record (R + 0x00 to R + 0x103)

Identical in all 38 fixtures except the two fields noted (`tools/header_fields.py`).
Session 4: identical in all 133 EXP files of the three folders (18c and 11g, table mode and owner
mode) except block 1 +0x3c (database set: 873 or 171) and block 2 +0x3c / +0x3e (client set). The
offsets below are R + n with R = 0x2000; with RECORDLENGTH 2048 the same bytes stand at 0x0800
(`ru_tbl_exp_cp1251_11g.dmp`@0x0800-0x0903). In an owner-mode file the item at R + 0x104 is
`METRICSU` (section 4.5).

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x2000 | 102 | block 1: `64 00 00 00 00 60 01 24 0f 05 0b 0c 03 0c 0c 05 04 05 0d 06 09 07 08 05 05 05 05 05 0f 05 05 05 05 05 0a 05 05 05 05 05 04 05 06 07 08 08 23 47 23 47 08 11 23 08 11 41 b0 47 00 83` + `03 69` + `07 d0` + `03` + 37 zero bytes | CONV@0x2000; same bytes in every fixture | unknown, except: +0x3c = 873, +0x3e = 2000 (u16 BE), unchanged by the client set. Session 4: +0x3c is the database character set (confirmed: `00 ab` at `ru_tbl_exp_cp1251_11g.dmp`@0x083c and `ru_tbl_exp_us7ascii_11g.dmp`@0x083c); it equals header csB, and +0x3e equals header csC, in all 133 files |
| 0x2066 | 102 | block 2: same as block 1 except +0x3c and +0x3e, which both hold the client character set id | CONV@0x20a2 `03 69 03 69`; `t_frame_exp_us7ascii`@0x20a2 `00 01 00 01`; `t_frame_exp_w1252`@0x20a2 `00 b2 00 b2` | the two fields: confirmed (follow the client id); the rest: unknown |
| 0x20cc | 8 | `06 00` + `+00:00` | CONV@0x20cc | observed. `truth_env.txt` `dbtimezone=+00:00`; only one zone in the fixtures |
| 0x20d4 | 2 | `00 00` (string of length 0) | CONV@0x20d4 | unknown |
| 0x20d6 | 6 | `04 00` + `BYTE` | CONV@0x20d6 | observed; meaning unknown |
| 0x20dc | 8 | `06 00` + `UNUSED` | CONV@0x20dc | observed; meaning unknown |
| 0x20e4 | 3 | `01 00` + `2` | CONV@0x20e4 | observed; meaning unknown |
| 0x20e7 | 13 | `0b 00` + `INTERPRETED` | CONV@0x20e7 | observed; meaning unknown |
| 0x20f4 | 13 | `0b 00` + `DISABLE:ALL` | CONV@0x20f4 | observed; meaning unknown |
| 0x2101 | 2 | `00 00` | CONV@0x2101 | unknown |
| 0x2103 | 1 | LF | CONV@0x2103 | observed |

## 4. Table section: text and column descriptors

A table section starts with the line `TABLE "name"` and ends where the next `TABLE "name"` line
or the first trailer line (`METRICST` + text, section 7) starts. With several tables in one file,
the sections follow each other directly (no `METRICET`, no count between them).

**Session 4.** (a) In owner mode the table sections do not follow `METRICST`: they stand after
ten `METRICSU` / `METRICEU` pairs and end with an `ENDTABLE` line (section 4.5). A table section
ends at the next line that starts with `TABLE "`, is `ENDTABLE`, or starts with `METRICS`.
(b) More statement kinds after the rows: `GRANT` (4.5). (c) The offsets and texts of this section
are the same on 11g, except the `CREATE TABLE` text of a table without rows (13.3).

**Session 5.** A table that has optimizer statistics carries them after its rows: one item
`ANALSTATS T? "name"` + field for the table and one per column (section 4.6); an index item
`ANALSTATS I?` follows its `CREATE INDEX` line. None of them stands before the rows.

**Session 6.** "One per column" is too strong: a column item is written only for a column that has
statistics. A table can have a table item and fewer column items than columns, a table item and no
column item, or no statistics item at all (4.6, "Columns without statistics").

**Correction (session 3).** Session 2 wrote "a table section ... ends with the LF after its row
data". That holds only for tables without indexes, constraints and comments. In `ddl` statements
follow the rows of both tables (`ddl`@0x2286 after the rows of T_PARENT, `ddl`@0x2656-0x2a6c after
the rows of T_DDL); see 4.3. The text region is also not purely "lines ending in LF"; see 4.2.

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x2104 | 9 | text `METRICST` + LF, once, before the first table | CONV@0x2104; `multi_framing`@0x2104 (once for 3 tables) | observed; meaning unknown |
| 0x210d | var | text `TABLE "T_FRAME"` + LF | CONV@0x210d; `multi_framing`@0x210d, @0x22f6 (`T_LEN`), @0x3469 (`T_NULLMIX`) | confirmed (table names of the logs) |
| 0x211d | var | text `CREATE TABLE "T_FRAME" (...) ... NOCOMPRESS` + LF | CONV@0x211d; `t_num`@0x211b | confirmed as text (column names and types agree with the truth files) |
| - | var | optional items between `CREATE TABLE` and `INSERT INTO`: one pair "line `ALTER TABLE "T" MODIFY DEFAULT` + field" per column with a DEFAULT | `ddl`@0x24c1-0x2581, `ddl/direct`@0x24bd-0x257d (2 columns with DEFAULT in `corpus/sql/01_corpus_tables.sql` lines 305, 307); sections 4.2, 4.3 | confirmed for DEFAULT (one table, both paths); other statement kinds before the data: not in the fixtures |
| 0x2223 | var | text `INSERT INTO "T_FRAME" ("A", "B", "C") VALUES (:1, :2, :3)` + LF | CONV@0x2223; `t_num`@0x2208 | confirmed as text; gives column names and order |
| 0x225d | 2 | u16 LE: number of columns | CONV@0x225d `03 00`; `t_num`@0x2238 `02 00`; `t_nullmix`@0x2272 `04 00`; `t_wide`@0x318d `78 00` (120 columns) | confirmed |
| 0x225f | 4 or 8 each | one descriptor per column (4.1) | CONV@0x225f, 0x2267, 0x226f | confirmed |
| 0x2277 | 2 | `00 00` | CONV@0x2277; every table of every fixture | unknown (always zero) |
| 0x2279 | 2 | u16 LE: 0 = conventional path, 1 = direct path | CONV@0x2279 `00 00`, DIRECT@0x2279 `01 00`; `t_num`@0x2244 / `t_num/direct`@0x2244; every table in all 19 pairs (in `multi_framing/direct` at 0x2279, 0x242a, 0x35ac); logs. Session 3: also for tables without rows (`multi_empty/direct`@0x23e3 `01 00`, `multi_empty`@0x23f1 `00 00`) and for every client set (`t_text_exp_direct_w1252`@0x2297 `01 00`) | confirmed |
| after the LF that ends the rows | var | optional items after the rows: `CREATE [UNIQUE] INDEX`, `ANALSTATS` + field, `CALTER TABLE` + field, `ALTER TABLE ... ADD ... PRIMARY KEY`, `COMMENT ON ...` + field | `ddl`@0x2286 (T_PARENT), `ddl`@0x2656-0x2a6c (T_DDL); `ddl/direct`@0x2282, 0x264c-0x2a62; section 4.3 | confirmed as text for these kinds (one fixture pair; the texts agree with the SQL). Other kinds (triggers, grants, ...): not in the fixtures. Session 5: also `ANALSTATS T?` + field, once for the table and once per column (`stat_exp_18c.dmp`@0x2925-0x3107; section 4.6) |

The table section of a table is byte-identical whether the table is exported alone or with
others: `multi_framing`@0x22f6-0x3468 = `t_len`@0x210d-0x327f (4467 bytes); the same holds for
all 7 tables of `multi_framing` and `multi_types` on both paths (14 comparisons, all identical).
Session 3: also for the 5 tables of `multi_empty` and `multi_empty2` on both paths (10
comparisons, all identical), e.g. `multi_empty`@0x23f6-0x3568 = `t_len`@0x210d-0x327f and
`multi_empty/direct`@0x22e8-0x23e5 = `t_empty/direct`@0x210d-0x220a (254 bytes).

### 4.1 Column descriptor

| offset in descriptor | length | meaning | evidence | status |
|---|---|---|---|---|
| +0 | 2 | u16 LE: data type code, the same number that `DUMP()` prints as `Typ=`: 1 VARCHAR2 and NVARCHAR2, 2 NUMBER (also `FLOAT`), 12 DATE, 96 CHAR, 180 TIMESTAMP | type 1: CONV@0x225f `01 00`; type 2: `t_num`@0x223a `02 00`; type 12: `t_date`@0x223f `0c 00`; type 96: `t_text`@0x2285 `60 00`; type 180: `t_ts`@0x2286 `b4 00`. Truth: `Typ=2` (`truth_num.txt`), `Typ=12` (`truth_date.txt`), `Typ=1` / `Typ=96` (`truth_text.txt`), `Typ=180` (`truth_ts.txt`) | confirmed |
| +0 | 2 | further codes: 23 RAW, 181 TIMESTAMP WITH TIME ZONE, 231 TIMESTAMP WITH LOCAL TIME ZONE | `t_raw`@0x2241 `17 00`; `t_tstz`@0x2296 `b5 00`, @0x229a `e7 00`; `truth_tstz.txt` `Typ=181`, `Typ=231` | 181 / 231: confirmed (one table, both paths, truth). 23: confirmed in session 3 (one table, both paths, `truth_raw.txt`; section 6.5) |
| +2 | 2 | u16 LE: maximum byte length of a value. VARCHAR2(n): n; CHAR(n): n; NVARCHAR2(n): 2n; NUMBER: 22 whatever the precision; DATE: 7; TIMESTAMP(0): 7; TIMESTAMP(6), (9): 11; RAW(n): n | VARCHAR2(10) CONV@0x2261 `0a 00`; VARCHAR2(100) `t_probe6`@0x2254 `64 00`; VARCHAR2(20) `t_empty`@0x21fc `14 00`; VARCHAR2(400) `t_len`@0x2247 `90 01`; CHAR(10) `t_text`@0x2287 `0a 00`; NVARCHAR2(50) `t_text`@0x228f `64 00`; NUMBER(3,0) and NUMBER `t_num`@0x223c, 0x2240 `16 00`; DATE `t_date`@0x2241 `07 00`; TIMESTAMP(0)/(6)/(9) `t_ts`@0x2288 `07 00`, 0x228c `0b 00`, 0x2290 `0b 00`; RAW(2000) `t_raw`@0x2243 `d0 07` | confirmed for VARCHAR2 (4 declared lengths = DDL text). Other types: observed (one declared size each). The VARCHAR2 columns were all declared with byte semantics; `VARCHAR2(n CHAR)` is not in the fixtures. Session 4: more declared sizes, all in `ru_tbl_exp_cp1251_11g.dmp`: VARCHAR2(200) @0x0ae2 `c8 00`, CHAR(12) @0x0aea `0c 00`, VARCHAR2(4000) @0x0b02 `a0 0f`, NVARCHAR2(40) @0x0afa `50 00` (80: status of "NVARCHAR2(n): 2n" raised to confirmed, two declared sizes, two databases), and `VARCHAR2(20 CHAR)` @0x0af2 `14 00` (20; the database is single-byte, so this does not say whether the field holds characters or bytes: section 14.4) |
| +4 | 2 | only for types 1 and 96: u16 LE character set id of the column. 873 for VARCHAR2 and CHAR, 2000 for NVARCHAR2 | 873: CONV@0x2263 `69 03`; `t_text`@0x2281 (V), 0x2289 (C). 2000: `t_text`@0x2291 `d0 07` (NV); `multi_types`@0x2a14. `truth_text.txt`: `CharacterSet=AL32UTF8` for V and C, `CharacterSet=AL16UTF16` for NV; `truth_charsets.txt`: 873, 2000 | confirmed that it is 873 / 2000 by column kind, in agreement with the truth. It did **not** change with the client character set (`t_frame_exp_us7ascii`@0x2263 and `t_frame_exp_w1252`@0x2263 are `69 03`). Session 3: it names the set the value bytes are really written in, for all three client sets and both paths: `t_text_exp_w1252`@0x2281 `69 03` and `t_text_exp_us7ascii`@0x2281 `69 03` with UTF-8 value bytes (section 6.3); @0x2291 `d0 07` with UTF-16BE value bytes. Status: confirmed (6 files, `truth_text.txt`). Limit: in every fixture this id equals header 0x0032 (for NVARCHAR2: 0x0034), so the data cannot say which of the two a reader should trust when they differ. Session 4: with a CL8MSWIN1251 database the id is 171 for VARCHAR2 and CHAR (`ru_tbl_exp_cp1251_11g.dmp`@0x0ae4 `ab 00`) and the value bytes are CL8MSWIN1251 for all four client sets (section 14.1); it still equals header csB in every file |
| +6 | 2 | only for types 1 and 96: u16 LE, 1 for VARCHAR2 and CHAR, 2 for NVARCHAR2 | 1: CONV@0x2265 `01 00`; `t_text`@0x2283, 0x228b. 2: `t_text`@0x2293 `02 00`; `multi_types`@0x2a16 | observed. Hypothesis: 1 = database character set, 2 = national character set. Session 3: unchanged by the client set (`t_text_exp_w1252`@0x2283 `01 00`, @0x2293 `02 00`; same in `t_text_exp_us7ascii` and the two direct files) |

Descriptor size: 8 bytes for types 1 and 96, 4 bytes for types 2, 12, 23, 180, 181, 231. With
this rule the descriptors of all tables in all 38 fixtures end exactly where the `00 00` /
flag pair and the rows begin, and all rows decode (`tools/exp_parse.py`, no problems reported).
For types not in the fixtures the size is unknown.
Session 4: the same holds for all 133 files (202 table sections). One more type code occurs, in
`11g-cp1251/` only: 112 (CLOB), with an 8-byte descriptor `70 00 48 00 ab 00 01 00`
(`ru_tables_exp_us7ascii_11g.dmp`@0x11adc); observed only, outside phase 1 (section 14.6).

**Session 7 (R-025, R-026a, R-028).** Offsets hold for the 18c and the 11g file of each stem (byte-identical
from R on, "Fixtures added in session 7").

| offset in file | descriptor bytes | column (declared) | what it shows | evidence | status |
|---|---|---|---|---|---|
| `nchar_exp_<v>`@0x2282 | `60 00 0a 00 d0 07 02 00` | NC `NCHAR(5)` | type 96 as for CHAR; length 10 = 2 x 5; set id 2000 (AL16UTF16); +6 = 2. The Architect's hypothesis of R-026a is right byte for byte | `truth_nchar.txt` `Typ=96 Len=10` (row 1, padded value); `truth_charsets.txt` 2000 | confirmed (2 versions; one table) |
| `nchar_exp_<v>`@0x228a | `60 00 10 00 69 03 01 00` | CC `CHAR(4 CHAR)` | length **16** = 4 characters x 4 bytes (the largest AL32UTF8 character), not 4. Set 873, +6 = 1 | `truth_nchar.txt` `Typ=96 Len=4` (row 1) and `Len=6` (row 2): values shorter than 16 | observed (one column, 2 versions). Hypothesis: n x the maximum character width of the database set; this fits `VARCHAR2(20 CHAR)` = 20 in the single-byte CL8MSWIN1251 database (`ru_tbl_exp_cp1251_11g.dmp`@0x0af2). `VARCHAR2(n CHAR)` in AL32UTF8 is still not in the fixtures (R-011) |
| `nchar_exp_<v>`@0x2292 | `01 00 0a 00 d0 07 02 00` | NV `NVARCHAR2(5)` | as in session 2 (2n, 2000, 2) | `truth_nchar.txt` `Typ=1` | confirmed |
| `long_exp_<v>`@0x225a | `08 00 00 00` | L `LONG` | type 8 (the `Typ=` of LONG), **length 0**, no further bytes: the next descriptor (V, `01 00 0a 00 69 03 01 00`) starts at @0x225e and the `00 00` + path flag at @0x2266, after which the rows decode to the SQL values | `11_types.sql`; log "T_LONG 3 rows" | confirmed as a 4-byte descriptor (2 versions; a column follows it, so a wrong size would misplace V) |
| `long_exp_<v>`@0xa0d4 | `18 00 00 00` | LR `LONG RAW` | type 24, length 0, 4 bytes: `00 00 00 00` (the two bytes + path flag 0) at @0xa0d8, row 1 at @0xa0dc | `11_types.sql`; log "T_LONGRAW 2 rows" | confirmed as 4 bytes (2 versions). Weaker than LONG: LR is the last column; an 8-byte reading would take `00 00 00 00` as descriptor bytes and the path flag would be `c1 02`, which is not 0 or 1 |
| `t_decl_exp_<v>`@0x2461-0x2488 | 10 x `02 00 16 00` | `NUMBER`, `NUMBER(*,2)`, `NUMBER(38)`, `NUMBER(5,-2)`, `NUMBER(3,5)`, `INTEGER`, `DECIMAL(10,2)`, `FLOAT`, `FLOAT(126)`, `REAL` | the descriptor is the same 4 bytes for all ten: precision, scale, and the words INTEGER / DECIMAL / FLOAT / REAL leave no trace in it. Precision and scale are only in the `CREATE TABLE` text (4.7) | `truth_decl.txt` rows 1-10 (`data_length` 22) | confirmed (10 columns, 2 versions) |
| `t_decl_exp_<v>`@0x2489, 0x248d | `b4 00 0b 00`, `0c 00 07 00` | `TIMESTAMP` (no precision), `DATE NOT NULL` | `TIMESTAMP` is `TIMESTAMP(6)` (length 11); `NOT NULL` leaves no trace in the descriptor | `truth_decl.txt` rows 11, 12 (`TIMESTAMP(6)`, 11; `DATE`, 7, `N`) | confirmed |

Descriptor size rule, session 7: 8 bytes for types 1 and 96 (including NCHAR and CHAR with
character semantics), 4 bytes for 2, 8, 12, 23, 24, 180, 181, 231. With it all 314 table sections
of the 169 files decode (`tools/check_all.py`: `problems 0`); type 112 stays as in session 4.

**Session 8 (R-027, R-029).** Offsets of `lob_exp_18c.dmp` / `lob_exp_11g.dmp` (the descriptors are
the same bytes in both, 6 bytes earlier in the 11g file because its `CREATE TABLE` line is shorter)
and of `misc_exp_<v>.dmp` (same offsets in both versions). The type code equals the `Typ=` of
`DUMP` where the truth prints one (`truth_misc.txt`: `Typ=182`, `Typ=183`, `Typ=69`, `Typ=208`);
for the LOB types `DUMP` prints nothing, and the codes 112 / 113 are the ones the walk needs.

| offset in file (18c / 11g) | descriptor bytes | column (declared) | what it shows | evidence | status |
|---|---|---|---|---|---|
| `lob`@0x23fb / 0x23f5 | `70 00 48 00 69 03 01 00` | C `CLOB` | type 112, length 72, set 873 (the database set), form 1: 8 bytes, as for 1 and 96. The same 8 bytes for A `CLOB` of T_LOB2 (@0x4db79 / 0x4df72) | the next descriptor follows after 8 bytes; the walk reaches the rows and every value equals `truth_lob.txt` and the SQL (`tools/lob_check.py`) | confirmed (2 columns, 2 versions; with R_CLOB of session 4: `70 00 48 00 ab 00 01 00`, set 171) |
| `lob`@0x4db89 / 0x4df82 | `70 00 48 00 d0 07 02 00` | N `NCLOB` | type **112** like CLOB; set 2000 (AL16UTF16), form 2: NCLOB differs from CLOB only in the set id and the form, like NVARCHAR2 from VARCHAR2 | as above; `truth_lob.txt` `T_LOB2\|1\|3\|x\|3\|2` | confirmed (1 column, 2 versions) |
| `lob`@0x2403 / 0x23fd | `71 00 48 00` | B `BLOB` | type **113**, length 72, **4 bytes** (no set id). The same for B of T_LOB2 (@0x4db91 / 0x4df8a), the last column, after which `05 00 00 00` and the column entries follow | T_LOB: an 8-byte reading would take `03 00 00 00` (@0x2407) as set id and form, and the path flag would be the u16 at @0x240d, `00 01` = 256, neither 0 nor 1. With 4 bytes every value decodes to the truth | confirmed (2 columns, 2 versions) |
| `misc`@0x2297 | `b6 00 05 00` | IYM `INTERVAL YEAR(4) TO MONTH` | type 182 = `Typ=182`, length 5 = `Len=5` of every non-NULL value; 4 bytes | the next descriptor at @0x229b; rows decode to `truth_misc.txt` | confirmed (1 column, 2 versions) |
| `misc`@0x229b | `b7 00 0b 00` | IDS `INTERVAL DAY(4) TO SECOND(6)` | type 183, length 11; 4 bytes: `00 00 00 00` (the two bytes and path flag 0) at @0x229f, row 1 at @0x22a3 | `truth_misc.txt` `Typ=183 Len=11` | confirmed (1 column, 2 versions) |
| `misc`@0x2459 | `45 00 17 00` | RID `ROWID` | type 69 = `Typ=69`; length **23**, not the 10 of `DUMP`: the value in the file is 23 bytes (6.7) | next descriptor at @0x245d | confirmed (1 column, 2 versions) |
| `misc`@0x245d | `d0 00 a0 0f` | URID `UROWID` (written `UROWID(4000)` in the `CREATE TABLE` line) | type 208 = `Typ=208`, length 4000 (the declared maximum); 4 bytes: `00 00 00 00` at @0x2461, row 1 at @0x2465 | `truth_misc.txt` `Typ=208 Len=13` | confirmed (1 column, 2 versions) |

Descriptor size rule, session 8: **8 bytes for types 1, 96 and 112**; **4 bytes for 2, 8, 12, 23,
24, 69, 113, 180, 181, 182, 183, 208, 231**. With it all 334 table sections (338 binary parts) of
the 177 files decode (`tools/check_all.py`: `problems 0`). The descriptor length of a LOB column
(72) is not a value length (LOB values of 200,000 bytes occur); for 112 and 113 it is the same 72
in 11 files. Not in the fixtures: BFILE, object, XML, nested-table and VARRAY columns.

**Session 9 (R-033, R-034, R-031).** `t_probe11_exp_<v>` and `t_probe11_exp_direct_<v>`: the same
offsets in all four files (18c / 11g, conventional / direct; they differ only in the path flag at
0x2343). `nchar2_exp_<v>`: 18c offset / 11g offset (the 11g `CREATE TABLE` line is 6 bytes shorter).

| offset in file | descriptor bytes | column (declared) | `data_length` (truth) | what it shows | status |
|---|---|---|---|---|---|
| `t_probe11`@0x2309 | `01 00 0a 00 69 03 01 00` | B `VARCHAR2(10 BYTE)` | 10 | byte semantics: n (as before) | confirmed |
| `t_probe11`@0x2311 | `01 00 28 00 69 03 01 00` | C `VARCHAR2(10 CHAR)` | 40 | **4n**: R-011 / X4 answered for AL32UTF8 | confirmed (4 files, `truth_probe11.txt` `COL\|C\|VARCHAR2\|40\|10\|C`) |
| `t_probe11`@0x2319 | `60 00 0c 00 69 03 01 00` | K `CHAR(3 CHAR)` | 12 | 4n, as `CHAR(4 CHAR)` = 16 in `nchar` | confirmed |
| `t_probe11`@0x2321 | `01 00 a0 0f 69 03 01 00` | W `VARCHAR2(1000 CHAR)` | 4000 | 4n = 4000 | confirmed |
| `t_probe11`@0x2329 | `01 00 a0 0f 69 03 01 00` | X `VARCHAR2(1001 CHAR)` | 4000 | **capped**: 4000, not 4004 | confirmed |
| `t_probe11`@0x2331 | `60 00 d0 07 69 03 01 00` | Y `CHAR(500 CHAR)` | 2000 | 4n = 2000 | confirmed |
| `t_probe11`@0x2339 | `60 00 d0 07 69 03 01 00` | Z `CHAR(501 CHAR)` | 2000 | **capped**: 2000, not 2004 | confirmed |
| `nchar2`@0x2482 / 0x247c | `60 00 06 00 d0 07 02 00` | NC `NCHAR(3)` | - | 2n (C5) | confirmed |
| `nchar2`@0x248a / 0x2484 | `01 00 06 00 d0 07 02 00` | NV `NVARCHAR2(3)` | - | 2n (C3) | confirmed |
| `nchar2`@0x2492 / 0x248c | `60 00 0c 00 69 03 01 00` | CC `CHAR(3 CHAR)` | - | 4n | confirmed |
| `nchar2`@0x249a / 0x2494 | `01 00 0c 00 69 03 01 00` | VC `VARCHAR2(3 CHAR)` | - | 4n | confirmed |
| `nchar2`@0x24a2 / 0x249c, 0x24aa / 0x24a4 | `70 00 48 00 69 03 01 00`, `70 00 48 00 d0 07 02 00` | CL `CLOB`, NCL `NCLOB` | - | as in session 8 (72) | confirmed |

Rule (descriptor length of a character column, type 1 or 96, form 1): byte semantics = n; character
semantics = min(w x n, cap) with w = the largest character of the descriptor's set in bytes (4 for
873 AL32UTF8, 1 for the single-byte sets) and cap = 4000 for VARCHAR2, 2000 for CHAR; it equals
`data_length` of the dictionary in all 7 columns of `truth_probe11.txt`. `tools/collist.py` checks
it on every character-semantics column of the corpus: 57 columns of 201 files (session 11: 30 in AL32UTF8 incl. 2 x
12 of `t_probe11`, 27 `VARCHAR2(20 CHAR)` in CL8MSWIN1251 = 20; 55 with 25 in CL8MSWIN1251 in session 9, the 2 `ru_lob2_*`
files of session 10 add 2), 0 differ. Status: confirmed for 873 (w 4)
and 171 (w 1); for other sets w is assumed. Form 2 (national): 2n (C3, C5). The `CREATE TABLE` text
gives n and the word `CHAR`, so a reader that wants the declared length takes it from there (T9),
not from the descriptor.

**LONG on the direct path: the column is moved last.** In every direct-path file with a LONG table the
`INSERT INTO` line lists the LONG column **last**, and the descriptors are in that order, while the
`CREATE TABLE` line keeps the declared order. The conventional files keep the declared order.

| file | `CREATE TABLE` order | `INSERT INTO` order and descriptors | status |
|---|---|---|---|
| `long2_exp_<v>`, `long2_exp_us7ascii_<v>` (conventional) | ID, L, V | ID, L, V: `02 00 16 00` @0x2259, `08 00 00 00` @0x225d, `01 00 0a 00 69 03 01 00` @0x2261 | confirmed (4 files) |
| `long2_exp_direct_<v>` | ID, L, V | **ID, V, L**: `02 00 16 00` @0x2259, `01 00 0a 00 69 03 01 00` @0x225d, `08 00 00 00` @0x2265 (`INSERT INTO "T_LONG2" ("ID", "V", "L")` @0x221c) | confirmed (2 versions) |
| `long_exp_direct_<v>` T_LONG | ID, L, V | **ID, V, L**: `INSERT INTO` 18c @0x221a / 11g @0x221b; L descriptor 18c @0x2262 / 11g @0x2263 | confirmed (2 versions) |
| `long_exp_direct_<v>` T_LONG_EMPTY | L, V | **V, L**: `INSERT INTO` 18c @0x11e9a / 11g @0x11eff; `01 00 0a 00 69 03 01 00` 18c @0x11ed2 / 11g @0x11f37, `08 00 00 00` 18c @0x11eda / 11g @0x11f3f | confirmed (2 versions) |
| `long_exp_direct_<v>` T_LONGRAW | ID, LR | ID, LR (LR is already last) | confirmed |

So "the names of the `INSERT INTO` line equal those of the `CREATE TABLE` line in order" (rule T9 of
version 3.2 / 3.3) is **refuted** for LONG tables on the direct path; the `INSERT INTO` line and the
descriptors agree with each other and with the row fields (rule T3 holds), and the move is exactly
"the LONG / LONG RAW column to the end, the others in declared order" in all 6 sections
(`tools/collist.py`: 6 sections "moved", 0 problems). A reader takes the column order of the rows
from the `INSERT INTO` line, never from `CREATE TABLE`. Why: hypothesis (the direct path writes the
LONG piece chain of 6.6 last); a table with two LONG columns cannot exist in Oracle (assumed).

### 4.2 Items of the text region (framing rule)

**Session 4, read this first.** The new owner-mode fixtures (R-012) show three things that this
section, as written in session 3, did not know: (1) a third form of field, pieces **without** the
`fd ff` marker (view text); (2) four more introducing lines; (3) both rules G and K of session 3
fail on the new files, K without noticing. The session 3 text is kept below; the corrections stand
in the rows marked "Session 4" and in the subsection "Session 4: which rule survives".

From `METRICST` (R + 0x104) to the last `EXIT`, the file is a sequence of **items**. Apart from
the binary block that follows an `INSERT INTO` line (column count to the LF after the rows,
sections 4, 5), there are two kinds of item:

| item | layout | evidence | status |
|---|---|---|---|
| line | text bytes, then LF (0x0a). The text contains no LF | every `TABLE`, `CREATE TABLE`, `INSERT INTO`, `CREATE INDEX`, `METRICST...`, `METRICET...`, `EXIT` line of all 46 files (`tools/exp_parse.py`: 1068 lines, no problem reported) | confirmed |
| field, plain | u16 LE length n, then n bytes, then LF. The LF is **not** counted in n. The n bytes may contain LF | `ddl`@0x24e4 `34 00` + 52 bytes, LF @0x251a; @0x253e `41 00` + 65 bytes (LF **inside** the value at 0x257f), LF @0x2581; @0x281c `59 00` + 89 bytes, LF @0x2877; @0x2a0b `25 00` + 37 bytes, LF @0x2a32; @0x2a5a `10 00` + 16 bytes, LF @0x2a6c. Same five in `ddl/direct`@0x24e0, 0x253a, 0x2812, 0x2a01, 0x2a50 | confirmed (5 fields x 2 paths, one fixture pair; the texts agree with `corpus/sql/01_corpus_tables.sql` lines 305, 307, 315, 316). Lengths above 255 not seen in a text field. Session 4: a length above 255 is the same u16 LE: `own`@0x2d5a `2e 01` + 302 bytes, LF @0x2e8a (confirmed, both versions). The count is in bytes of the client character set (section 14.2) |
| field, in pieces | `fd ff`, then pieces "u16 LE length n + n bytes", ended by a piece of length 0 (`00 00`), then LF. The pieces joined give one statement | `ddl`@0x2885 `fd ff`; pieces @0x2887 `17 00` (23 bytes `ALTER TABLE "T_DDL" ADD`), @0x28a0 `16 00` (22, ` CONSTRAINT "T_DDL_CK"`), @0x28b8 `08 00` (8, ` CHECK (`), @0x28c2 `19 00` (25, `status IN ('NEW', 'DONE')`), @0x28dd `01 00` (1, `)`), @0x28e0 `12 00` (18, ` ENABLE NOVALIDATE`), @0x28f4 `00 00`; LF @0x28f6. `ddl/direct`@0x287b, pieces @0x287d, 0x2896, 0x28ae, 0x28b8, 0x28d3, 0x28d6, 0x28ea; LF @0x28ec | session 3: observed (one statement, both paths). **Session 4: confirmed.** Six more examples per owner-mode file, on both versions: two CHECK constraints (`own`@0x2906, 0x2989), `PROCACTION` (`own`@0x214a), two procedures (`own`@0x3554 with 126 pieces, @0x5c29), a trigger (`own`@0x5ee6); the pieces of the PL/SQL sources equal the source line lengths of `truth_own_ddl.txt`, the 414-byte piece equals the truth's constraint length; 11g: `own_11g`@0x290b, 0x298e, 0x214a, 0x36e5, 0x5dba, 0x6077; also `ru_exp_cp1251_11g.dmp`@0x1351b. Piece lengths are not constant; the field ends with a piece of length 0 and LF (section 4.5) |
| field, in pieces without marker (session 4) | pieces "u16 LE length n + n bytes", ended by a piece of length 0 (`00 00`), then LF; **no** `fd ff` in front | `own`@0x349c `33 00` + 51 bytes (`SELECT id, status FROM o_child WHERE status = 'NEW'`), `00 00` @0x34d1, LF @0x34d3; `own_11g`@0x362d, `00 00` @0x3662, LF @0x3664; `ru_exp_cp1251_11g.dmp`@0x13962 `29 00` + 41 bytes + `00 00` + LF; `ru_exp_al32utf8_11g.dmp`@0x139d8 `2c 00` + 44 bytes | observed as bytes in 12 files (2 views, 2 versions). That it is "pieces" and not "a plain field followed by `00 00`" is a hypothesis: every example has exactly one piece (R-018) |

A field never stands alone: in the fixtures every field follows a line that introduces it.

| introducing line (ends in LF) | the field holds | evidence | status |
|---|---|---|---|
| `ALTER TABLE "T" MODIFY DEFAULT` | a complete statement `ALTER TABLE "T" MODIFY ("COL" DEFAULT <expression>)`; the expression is copied as stored, including a trailing LF | `ddl`@0x24c1 + field @0x24e4 (`'NEW' `), @0x251b + field @0x253e (`DATE '2020-01-01'` LF) | observed (2 per path) |
| `ANALSTATS IS "T"` | a PL/SQL block `BEGIN  DBMS_STATS.SET_INDEX_STATS(...); END;` | `ddl`@0x2807 + field @0x281c | observed (1 per path). Session 5: confirmed, and the letters can be `T?` or `C?` with other blocks and a field in pieces (4.6) |
| `CALTER TABLE` | (in pieces) `ALTER TABLE "T" ADD CONSTRAINT "name" CHECK (<condition>) ENABLE NOVALIDATE` | `ddl`@0x2878 + field @0x2885 | observed (1 per path) |
| `COMMENT ON TABLE "T" IS ` (trailing space) | the comment as a quoted SQL literal `'...'` | `ddl`@0x29ee + field @0x2a0b | observed (1 per path) |
| `COMMENT ON COLUMN "T"."COL" IS ` (trailing space) | the comment as a quoted SQL literal | `ddl`@0x2a33 + field @0x2a5a | observed (1 per path) |

Session 4: more introducing lines (offsets: `own`, 18c conventional; all occur on 11g too, 4.5).

| introducing line (ends in LF) | the field holds | form | evidence | status |
|---|---|---|---|---|
| `PROCACTION` | a PL/SQL block `BEGIN  ` LF `sys.dbms_logrep_imp.instantiate_schema(...);` LF `COMMIT; END;` LF | `fd ff` pieces | `own`@0x213f + field @0x214a; `ru_exp_cp1251_11g.dmp`@0x093f + @0x094a | confirmed (12 owner-mode files) |
| `CREATE FORMAT71 PROCEDURE "name" ` (trailing space) | `CREATE ` + the stored source, one piece per source line | `fd ff` pieces | `own`@0x352b + @0x3554; @0x5c05 + @0x5c29 | confirmed (2 procedures x 6 files) |
| `CREATE FORMAT92TRIG ` (trailing space) | the stored trigger text from `TRIGGER name ...` on, one piece per line | `fd ff` pieces | `own`@0x5ed1 + @0x5ee6; table mode: `own_tables`@0x32f4 + @0x3309 | confirmed (1 trigger x 10 files) |
| `CREATE VIEW "name" (columns) AS ` (trailing space) | the query text | pieces without marker | `own`@0x3473 + @0x349c; `ru_exp_cp1251_11g.dmp`@0x1393e + @0x13962 | observed (2 views) |
| `ANALSTATS IR "T"` (as `ANALSTATS IS`) | the same PL/SQL block | plain | `own_exp_w1252_18c.dmp`@0x2884 + @0x289b; section 14.5 | confirmed |
| `CALTER TABLE` (second and third example) | as in session 3 | `fd ff` pieces | `own`@0x28f9 + @0x2906, @0x297c + @0x2989 | confirmed |

More lines that are **not** followed by a field (session 4): `GRANT ...`, `CREATE SEQUENCE ...`,
`CREATE SYNONYM ...`, `ALTER SESSION SET ...`, `ALTER PROCEDURE ... COMPILE ...`, `ALTER TRIGGER ...
ENABLE`, `ENDTABLE`, every `METRICSU...` / `METRICEU ...` line (offsets in 4.5).

Lines that look similar but are **not** followed by a field (the whole statement is the line):
`ALTER TABLE "T" ADD  PRIMARY KEY ...` (`ddl`@0x2286), `ALTER TABLE "T" ADD  CONSTRAINT ...
PRIMARY KEY ...` (@0x28f7), `CREATE UNIQUE INDEX` (@0x2656), `CREATE INDEX` (@0x272a), and in the
trailer `ALTER TABLE ... ADD CONSTRAINT ... FOREIGN KEY ...` (@0x2ba4), `ANALCOMPUTE IS "T"
ANALYZE ...` (@0x2d10), `ALTER TABLE ... ENABLE CONSTRAINT ...` (@0x2d9d, 0x2dce).

**How to tell a field from a line.** Two rules consume all 46 files identically
(`tools/check_all.py`: 0 disagreements; only the two `ddl` files contain fields, 6 each):

| rule | statement | what it relies on | status |
|---|---|---|---|
| G (generic) | at the start of an item: if the first two bytes are `fd ff`, or the second byte is below 0x20, the item is a field; otherwise it is a line | a line starts with two printable ASCII characters; a plain field shorter than 0x2000 bytes has a second byte below 0x20 | consistent with all fixtures. Not proven: a plain text field of 8192 bytes or more would be read as a line (none exists; whether such a value is written plain or in pieces is unknown) |
| K (keyword) | an item is a field exactly when the previous line is one of the five introducing lines above | the list of introducing lines being complete | consistent with all fixtures. Not proven: the list comes from one table; other introducing lines may exist (hypothesis: any statement that carries free text, e.g. triggers or views in other export modes) |

The two rules differ only on input the fixtures do not contain. R-012 asks for it.

#### Session 4: which rule survives

`tools/check_all.py` runs four rules on all 133 files: `generic3` and `keyword3` (rules G and K
exactly as written above) and `generic`, `keyword` (corrected).

| rule | result on the new files | first failure | status |
|---|---|---|---|
| G as written in session 3 (`generic3`) | **fails** on all 12 owner-mode files: the view text is read as a plain field, and no LF follows it | `own`@0x349c: field of 51 bytes, then `00 00` @0x34d1 instead of LF; `own_11g`@0x362d / 0x3662; `ru_exp_cp1251_11g.dmp`@0x13962 / 0x1398d. The walk then loses its place (48 problems in `own`). It passes the table-mode files, `own_tables` included | withdrawn as written |
| K as written in session 3 (`keyword3`) | **reads wrong items without any error** on all 12 owner-mode files and on the 4 `own_tables` files: the list of introducing lines was incomplete, so the pieces of `PROCACTION`, of the procedures and of the trigger are read as text lines. Because every source line ends in LF, the walk still reaches the two `EXIT` lines | `own`@0x214a (`fd ff 08 00 BEGIN  ` read as a line); `own_tables`@0x3309 (trigger); `own_tables_exp_11g.dmp`@0x3495. 270 "items" instead of 134 in `own` | withdrawn as written. **"The walk ends without a problem" is not a test of a framing rule**; the item list must be compared |
| G corrected (`generic`): field if the first two bytes are `fd ff` or the second byte is below 0x20; after "u16 n + n bytes", if the next byte is not LF, read on as pieces up to a piece of length 0 | walks all 133 files without a problem | - | consistent with all fixtures |
| K corrected (`keyword`): a field follows exactly the 8 introducing lines of rule I6 (section 11), and the line decides the form (plain / `fd ff` pieces / pieces without marker) | walks all 133 files without a problem; a field whose form does not match its line would be reported; none is | - | consistent with all fixtures |
| G corrected against K corrected | identical items, tables and rows in all 133 files (0 disagreements) | - | no fixture separates them |

Which fixture decided: `own_exp_<v>.dmp` (any of the six) refutes G of session 3, at the view;
`own_tables_exp_<v>.dmp` and `own_exp_<v>.dmp` refute K of session 3, at the trigger and at
`PROCACTION`. Between the two corrected rules no fixture decides. Where each can still fail:

| rule | input on which it would fail | in the fixtures? |
|---|---|---|
| G corrected | a plain field of 8192 bytes or more (second byte 0x20 or more); a view in several pieces whose second piece has a length with low byte 0x0a (read as the closing LF); a line of one character | no (largest plain field: 302 bytes; all views have one piece) |
| K corrected | any introducing line that is not in the list (functions, packages, types, `ANALSTATS T?`, ...): its field is read as lines, silently if the field is PL/SQL source | no |

Recommendation for a reader (a judgement, not a confirmed rule): use the list (K corrected), and
when a line is not in the list test the next item with G corrected; if G says "field", read it as a
field and report the unknown introducing line. A reader built on K alone is the one that fails
silently on a real dump with object kinds the fixtures lack. R-018 asks for the missing cases.

Is the framing the same on both paths? Yes in the fixtures: the items of `ddl` and `ddl/direct`
have the same kinds, lengths and texts, shifted by 4 bytes after the rows of T_PARENT and by 10
after the rows of T_DDL (the missing `00 00` per row); `tools/exp_parse.py` prints both lists.

#### Session 5: the field after an `ANALSTATS` line (R-016)

| statement | evidence | status |
|---|---|---|
| The field after an `ANALSTATS` line is **not always plain**: for column statistics it is a field in pieces after `fd ff` (5, 7 or 8 pieces with the ending one); for table and index statistics it is plain. The line does not say which: the line of a table item and the lines of its column items are the same bytes | `stat_exp_18c.dmp`@0x2925 `ANALSTATS TS "T_STAT1"` + @0x293c `52 00` (plain); @0x2991 the same line + @0x29a8 `fd ff 8a 00`; 11g: `stat_exp_11g.dmp`@0x293c, 0x29a8; `ru_stat_exp_us7ascii_est_11g.dmp`@0x190b6 `54 00`, @0x19123 `fd ff 7b 00` | confirmed (382 fields after `ANALSTATS` lines in 46 files: 144 plain, 238 in pieces) |
| An `ANALSTATS` line can carry text after the table name: `ANALSTATS CS "T_STAT1" ("ID" ) ` (31 bytes, trailing space) | `stat_exp_18c.dmp`@0x4e25; `stat_exp_11g.dmp`@0x4e6d | confirmed (8 files) |
| Exactly one field follows every `ANALSTATS` line, and no field follows an `ANALCOMPUTE` line | 382 `ANALSTATS` lines, 382 fields; 92 `ANALCOMPUTE` lines (`tools/stat_items.py`, `tools/intro_list.py`: `entry_without_field` 0) | confirmed |
| Rule K with the entry "prefix `ANALSTATS `" consumes all files, **if the entry allows both forms** and the reader decides plain / pieces by the first two bytes (`fd ff` or not). The entry of session 4 expected a plain field; `tools/exp_parse.py` of session 4 then still read the right bytes (it looks at the marker first) and only reported "keyword rule expected form plain", 14 times per `stat_exp_*` or `ru_stat_exp_*` file | `tools/check_all.py` before the change: "RULES DISAGREE" for the 17 files with `T` items; after: `rule_diff` 0 in the three folders | confirmed |
| Rule G corrected (4.2) reads the same items without any change | `tools/check_all.py`: 0 disagreements between `generic` and `keyword` on 153 files; `tools/intro_list.py`: 619 fields, each after a line of the list, 0 fields without an entry | consistent with all fixtures |
| The prediction of session 4 ("K corrected reads the field after `ANALSTATS T?` as lines, silently") was wrong for a list entry written as a prefix: the line `ANALSTATS TS "T"` matches the entry of `ANALSTATS IS "T"`. A list that names `ANALSTATS IS` and `ANALSTATS IR` literally would fail as predicted | - | - |

The largest field in pieces after an `ANALSTATS` line has 1189 payload bytes, the largest piece
797 (`ru_stat_exp_us7ascii_est_11g.dmp`@0x19422, piece @0x194d4 `1d 03`). A histogram with 254
endpoints of a text column would give a piece of about 9600 bytes; not in the fixtures (R-021).

### 4.3 Walk of the two table sections of `ddl`

Offsets: `ddl_exp_18c.dmp` / `ddl_exp_direct_18c.dmp`. "Kind" is my label; the Data Pump log of
the same tables (`ddl_dp_18c.log`) names the object types TABLE, COMMENT, INDEX,
INDEX_STATISTICS, CONSTRAINT, REF_CONSTRAINT.

| offset conv / direct | item | length | text (shortened) | kind |
|---|---|---|---|---|
| 0x210d / 0x210d | line | 16 + LF | `TABLE "T_PARENT"` | section start |
| 0x211e / 0x211e | line | 264 + LF | `CREATE TABLE "T_PARENT" ("ID" NUMBER(6, 0), "NAME" VARCHAR2(30) NOT NULL ENABLE)  PCTFREE ...` | CREATE TABLE (NOT NULL is inside it) |
| 0x2227 / 0x2227 | line | 53 + LF | `INSERT INTO "T_PARENT" ("ID", "NAME") VALUES (:1, :2)` | INSERT |
| 0x225d / 0x225d | binary | 2 | `02 00` column count; descriptors 0x225f, 0x2263; `00 00` 0x226b; path flag 0x226d (`00 00` / `01 00`); 2 rows from 0x226f; `ff ff` 0x2283 / 0x227f; LF 0x2285 / 0x2281 | table data |
| 0x2286 / 0x2282 | line | 226 + LF | `ALTER TABLE "T_PARENT" ADD  PRIMARY KEY ("ID") USING INDEX PCTFREE ... ENABLE ` | constraint (unnamed primary key) |
| 0x2369 / 0x2365 | line | 13 + LF | `TABLE "T_DDL"` | section start |
| 0x2377 / 0x2373 | line | 329 + LF | `CREATE TABLE "T_DDL" ("ID" NUMBER(6, 0), "PARENT_ID" NUMBER(6, 0), "STATUS" VARCHAR2(10) NOT NULL ENABLE, "AMOUNT" NUMBER(10, 2), "CREATED" DATE)  PCTFREE ...` | CREATE TABLE (no DEFAULT in it) |
| 0x24c1 / 0x24bd | line | 34 + LF | `ALTER TABLE "T_DDL" MODIFY DEFAULT` | introduces the next field |
| 0x24e4 / 0x24e0 | field | `34 00` + 52 + LF | `ALTER TABLE "T_DDL" MODIFY ("STATUS" DEFAULT 'NEW' )` | ALTER TABLE ... MODIFY (default) |
| 0x251b / 0x2517 | line | 34 + LF | `ALTER TABLE "T_DDL" MODIFY DEFAULT` | introduces the next field |
| 0x253e / 0x253a | field | `41 00` + 65 + LF | `ALTER TABLE "T_DDL" MODIFY ("CREATED" DEFAULT DATE '2020-01-01'` LF `)` | ALTER TABLE ... MODIFY (default) |
| 0x2582 / 0x257e | line | 98 + LF | `INSERT INTO "T_DDL" ("ID", "PARENT_ID", "STATUS", "AMOUNT", "CREATED") VALUES (:1, :2, :3, :4, :5)` | INSERT |
| 0x25e5 / 0x25e1 | binary | 2 | `05 00` column count; 5 descriptors from 0x25e7 / 0x25e3; `00 00` 0x25ff / 0x25fb; path flag 0x2601 / 0x25fd; 3 rows from 0x2603 / 0x25ff; `ff ff` 0x2653 / 0x2649; LF 0x2655 / 0x264b | table data (12 fields equal `truth_ddl.txt`) |
| 0x2656 / 0x264c | line | 211 + LF | `CREATE UNIQUE INDEX "T_DDL_PK" ON "T_DDL" ("ID" )  PCTFREE ...` | CREATE INDEX |
| 0x272a / 0x2720 | line | 220 + LF | `CREATE INDEX "T_DDL_IX" ON "T_DDL" ("AMOUNT" , "CREATED" )  PCTFREE ...` | CREATE INDEX |
| 0x2807 / 0x27fd | line | 20 + LF | `ANALSTATS IS "T_DDL"` | introduces the next field |
| 0x281c / 0x2812 | field | `59 00` + 89 + LF | `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"T_DDL_IX"',NULL,NULL,NULL,3,1,3,1,1,1,0,6); END;` | index statistics |
| 0x2878 / 0x286e | line | 12 + LF | `CALTER TABLE` | introduces the next field |
| 0x2885 / 0x287b | field in pieces | `fd ff` + 6 pieces + `00 00` + LF | `ALTER TABLE "T_DDL" ADD CONSTRAINT "T_DDL_CK" CHECK (status IN ('NEW', 'DONE')) ENABLE NOVALIDATE` | constraint (CHECK) |
| 0x28f7 / 0x28ed | line | 246 + LF | `ALTER TABLE "T_DDL" ADD  CONSTRAINT "T_DDL_PK" PRIMARY KEY ("ID") USING INDEX PCTFREE ... ENABLE ` | constraint (primary key) |
| 0x29ee / 0x29e4 | line | 28 + LF | `COMMENT ON TABLE "T_DDL" IS ` | introduces the next field |
| 0x2a0b / 0x2a01 | field | `25 00` + 37 + LF | `'table with constraints and defaults'` | COMMENT (table) |
| 0x2a33 / 0x2a29 | line | 38 + LF | `COMMENT ON COLUMN "T_DDL"."AMOUNT" IS ` | introduces the next field |
| 0x2a5a / 0x2a50 | field | `10 00` + 16 + LF | `'amount in euro'` | COMMENT (column) |
| 0x2a6d / 0x2a63 | line | 41 + LF | `METRICSTreferential integrity constraints` | first trailer line: end of the last table section |

Not in the table sections: the foreign key. It is written in the trailer, inside a group
`TABLE "T_DDL"` ... `ENDTABLE` (`ddl`@0x2b96, 0x2ba4, 0x2c1b); see section 7.1.

Everything from 0x2104 to the last `EXIT` is consumed by this walk without a gap, in both files.
No byte of the two table sections is unexplained; what stays unknown is the meaning of the
prefix letter `C` in `CALTER TABLE` and why this one statement is written in pieces (hypothesis:
the CHECK condition is stored in a column that is fetched piece by piece).

### 4.4 What a reader needs to reach the `INSERT INTO` line and the column count

Minimum, as a rule that was run on every fixture (`tools/exp_parse.py`, `tools/check_all.py`):

1. Start at R + 0x104 (after the LF that ends the string list of section 3). Session 4: the item
   there is `METRICST` in table mode and `METRICSU` in owner mode.
2. Read one item: if it is a field (rule G or rule K of 4.2, **as corrected in session 4**), skip
   "u16 length + bytes" (or, after `fd ff`, pieces until a piece of length 0; or, for a view,
   pieces without marker until a piece of length 0) and one LF; otherwise read a line up to LF.
3. If the line starts with `TABLE "`, remember the name. If it starts with `INSERT INTO `, the
   bytes after its LF are the column count (u16 LE), the descriptors, `00 00`, the path flag and
   the rows (sections 4, 4.1, 5); after the LF that ends the rows, continue with step 2.
4. Stop at the second of two consecutive `EXIT` lines.

A reader that only splits on LF fails on `ddl`: the field at 0x253e contains an LF, and the bytes
`0a` can also occur inside lengths. A reader must never search for the text `INSERT INTO`
without walking the items, because row data can contain any bytes.

A `TABLE "name"` line is not always followed by `CREATE TABLE` and `INSERT INTO`: inside the
trailer it is followed by other lines and `ENDTABLE` (`ddl`@0x2af0 `TABLE "T_PARENT"` + `ENDTABLE`
@0x2b01). Only an `INSERT INTO` line starts table data.

Session 4: a `TABLE "name"` line starts a table section exactly when the next item is a
`CREATE TABLE` line; this holds in table mode and in owner mode and is how `tools/exp_parse.py`
counts sections now. Result on all files of the three folders, with the corrected rules G and K
and both empty-table rules (identical results, `tools/check_all.py`):

| measure | `18c/` | `11g/` | `11g-cp1251/` |
|---|---|---|---|
| EXP files | 59 | 59 | 15 |
| files walked to the two `EXIT` lines without a problem | 59 | 59 | 15 |
| table sections found (`TABLE "` line followed by `CREATE TABLE`) | 87 | 87 | 28 |
| of these, with an `INSERT INTO` line and a column count | 87 | 87 | 28 |
| tables whose rows all equal the truth file, with the truth's row count | 85 | 85 | 22 (15 x R_TEXT; 7 x R_CLOB: ids, and LOB data read with the hypothesis of 14.6) |
| tables without truth rows, equal to the SQL or the log | 2 (T_PARENT) | 2 (T_PARENT) | 6 (3 x the Cyrillic-named table with 2 rows; 3 x the same table cut short, 0 rows, section 14.3) |
| tables with a differing field or a wrong row count | 0 | 0 | 0 |
| fields compared with truth, equal / differing | 3615 / 0 | 3615 / 0 | 27837 / 0 |
| tables without rows: conventional / direct | 6 / 5 | 6 / 5 | 2 / 1 (the cut-short table) |

O_PARENT, O_CHILD and O_EMPTY have `truth_own.txt` now and are counted in the 85 (10 files x 3
tables per version). The table below is the session 3 result on the 46 files of that session.

Result of the test on all 46 EXP files of `corpus/fixtures/18c/`, with rule G and with rule K
(identical results):

| measure | count |
|---|---|
| files walked to the two `EXIT` lines without a problem | 46 of 46 |
| table sections found (`TABLE "` lines before the trailer) | 64 |
| of these, with an `INSERT INTO` line and a column count | 64 |
| tables whose rows all equal the truth file, with the truth's row count | 62 (3272 fields compared, 0 differ; 6 of the 62 are T_EMPTY with 0 rows against `truth_counts.txt`) |
| tables without a truth file, rows equal to the SQL | 2 (T_PARENT in `ddl`, `ddl/direct`) |
| tables with a differing field or a wrong row count | 0 |

Session 4: owner mode is no longer unknown (section 4.5); the paragraph below is the state of
session 3. Nothing under `corpus/work/` was read in session 4.

Work files: the prompt of session 3 allowed reading `corpus/work/18c/schema_exp_18c.dmp` and
`schema_exp_direct_18c.dmp` for exploration. Neither file exists on this machine
(`corpus/work/18c/` holds only `logs/`), so nothing in this document comes from an owner-mode
export, and there is no `hypothesis (work file, not reproducible)` entry. Owner mode stays
unknown; R-012 requests a small committed fixture.

### 4.5 Owner mode (session 4, R-012; both versions)

Fixtures: `own_exp_<v>.dmp`, `own_exp_direct_<v>.dmp`, `own_exp_w1252_<v>.dmp` (owner mode, header
line `RUSERS`) and `own_tables_exp_<v>.dmp`, `own_tables_exp_direct_<v>.dmp` (the same three tables
in table mode, header line `RTABLES`), for `<v>` = `18c` and `11g`; also the six `ru_exp_*` files of
`11g-cp1251/`. Tools: `tools/exp_parse.py`, `tools/exp_items.py` (the two tables at the end of this
section are its output), `tools/check_all.py`.

#### What owner mode changes

| statement | evidence | status |
|---|---|---|
| The first item, at R + 0x104, is the line `METRICSU`, not `METRICST`. Every later section line is `METRICSU<text>` and every counter line `METRICEU <n>`; the last line before the two `EXIT` lines is `METRICEUG0` | `own`@0x2104, 0x210d, 0x21f7, 0x6313; `own_11g`@0x2104, 0x21f2, 0x64a4; `ru_exp_cp1251_11g`@0x0904 (R = 2048). Table mode: `METRICST`, `METRICET`, `METRICETG0` (`own_tables`@0x2104, 0x3135, 0x371a) | confirmed (12 owner-mode files, 2 versions, 2 databases). That the letter means the mode (`T` tables, `U` users) is the reading that fits the header line `RTABLES` / `RUSERS`; other modes (full database) are not in the fixtures |
| Before the tables stand 10 pairs `METRICSU<text>` / `METRICEU n`: "pre-schema procedural objects and actions" (with a `PROCACTION` item), "foreign function library names for user    ", "PUBLIC type synonyms", "private type synonyms", "object type definitions for user    ", "pre-schema procedural objects and actions" (again, empty), "database links", "sequence numbers" (with `CREATE SEQUENCE`), "cluster definitions", "XDB Schemas by user " | `own`@0x210d-0x241b; `own_11g`@0x210d-0x2420; walk below, items 2 to 24 | confirmed as a list of these fixtures (same 10 on both versions and in `ru_exp_*`). Other installations may write items into sections that are empty here (see the OLAP note under "Fixtures added in session 4") |
| The table sections follow the line `METRICEU n` of "XDB Schemas by user " directly; there is **no** `METRICSU` line for the tables. After the last table section stands one line `ENDTABLE`, then `METRICSUsynonyms` | `own`@0x241c `TABLE "O_CHILD"`, @0x341a `ENDTABLE`, @0x3423; `own_11g`@0x2421, 0x35ab, 0x35b4; `ru_exp_cp1251_11g`@0x0ba1 (`TABLE "R_CLOB"`), 0x1390b (`ENDTABLE`) | confirmed. In table mode there is no `ENDTABLE` after the table sections (`own_tables`@0x310b: `METRICSTreferential integrity constraints` follows the last table) |
| A table section is byte-identical in owner mode and in table mode, including the statements before and after the rows | 12 comparisons (3 tables x 2 paths x 2 versions), all identical: `own`@0x241c-0x2e8a = `own_tables`@0x2390-0x2dfe (O_CHILD, 2671 bytes); `own`@0x2e8b-0x3196 = `own_tables`@0x2dff-0x310a (O_EMPTY, 780 bytes); `own`@0x3197-0x3419 = `own_tables`@0x210d-0x238f (O_PARENT, 643 bytes); `own/direct` O_CHILD @0x241c-0x2e84 = `own_tables/direct`@0x238c-0x2df4 (2665 bytes); 11g: `own_11g`@0x2e90-0x3327 = `own_tables_11g`@0x2dff-0x3296 (O_EMPTY, 1176 bytes). Also R_TEXT: `ru_exp_us7ascii_11g`@0x2628-0x135ee = `ru_tables_exp_us7ascii_11g`@0x090d-0x118d3 (69575 bytes) | confirmed |
| The order of the tables differs: owner mode writes them in the order of the log (here alphabetical: O_CHILD, O_EMPTY, O_PARENT), table mode in the order of its log (O_PARENT, O_CHILD, O_EMPTY, which is the order of the `tables=` argument in R-012) | section offsets above; `own_exp_18c.log`, `own_tables_exp_18c.log` | confirmed as "order of the log lines" (section 5.5) |
| `GRANT` on a table is a line inside the table section, after the rows and before the constraint lines, in both modes | `own`@0x3310 `GRANT SELECT ON "O_PARENT" TO "CORPUS"`; `own_tables`@0x2286; `own_11g`@0x34a1 | confirmed (one grant, 2 modes, 2 paths, 2 versions) |
| Objects that are not tables stand in their own `METRICSU` sections after the `ENDTABLE` line: synonym (`CREATE SYNONYM`, a line), view (line `CREATE VIEW "V" (columns) AS ` + field), procedures (section "stored procedures": line `ALTER SESSION SET "_LOAD_WITHOUT_COMPILE" = PLSQL`, per procedure a line `CREATE FORMAT71 PROCEDURE "name" ` + field, line `ALTER SESSION ... = NONE`, per procedure a line `ALTER PROCEDURE "name" COMPILE REUSE SETTINGS TIMESTAMP '...'`). The sequence stands before the tables | `own`@0x3434 (synonym), @0x3473 + 0x349c (view), @0x34f9, 0x352b + 0x3554, 0x5c05 + 0x5c29, 0x5cba, 0x5ceb, 0x5d40 (procedures), @0x2342 (sequence); `own_11g`@0x35c5, 0x3604 + 0x362d, 0x368a, 0x36bc + 0x36e5, 0x5d96 + 0x5dba, 0x5e4b, 0x5e7c, 0x5ed1, 0x2344 | confirmed for these kinds (one object of each kind, two procedures; 2 versions). Functions, packages, types, materialized views, jobs, database links: not in the fixtures |
| Foreign key, trigger, `ANALCOMPUTE` and `ENABLE CONSTRAINT` statements stand in groups `TABLE "name"` ... `ENDTABLE` inside a section, in both modes. Trigger: line `CREATE FORMAT92TRIG ` + field + line `ALTER TRIGGER "name"  ENABLE` | owner mode: `own`@0x5e11-0x5e9c (FK), @0x5ec1-0x5f95 (trigger), @0x615f-0x6215 (two `ANALCOMPUTE` lines under "statistics"), @0x624e-0x62fe ("Enable enforce constraints "). Table mode: `own_tables`@0x3234-0x32bf, 0x32e4-0x33b8, 0x3568-0x361e (under "Deferred analyze commands "), 0x3656-0x3706 | confirmed (section 7.1) |
| In table mode the trailer has one group of five `METRICST` / `METRICET` pairs per table (section 7.1). In owner mode each section name occurs once for the whole file, and there are more sections: after the tables 19 pairs from "synonyms" to "Enable enforce constraints " | walk below: `own` items 65 to 131; `own_tables` items 41 to 95 | confirmed for these fixtures |
| The second record (two blocks, string list) is the same as in table mode | `tools/header_fields.py`: no invariant violation in the 12 owner-mode files; `own`@0x2000-0x2103 = `own_tables`@0x2000-0x2103 | confirmed |

#### Answers to the questions of the session prompt (Part C)

| question | answer | evidence | status |
|---|---|---|---|
| How is the source of `O_PROC_LONG` (9643 bytes, `SOURCETOTAL` of `truth_own_ddl.txt`) written? | As **one field in pieces after `fd ff`**: 125 pieces and the ending piece of length 0. Piece 1 is `CREATE ` (7 bytes, not part of the stored source), pieces 2 to 125 are the 124 source lines, each with its LF except the last: lengths 25, 6, 120 x 80, 8, 4 = the `SOURCE|O_PROC_LONG|...` lines of the truth. 7 + 9643 = 9650 payload bytes. No piece is longer than 80 bytes; no u16 `ab 25` occurs | `own`@0x3554 `fd ff`, first pieces @0x3556 (7), 0x355f (25), 0x357a (6), 0x3582 (80); last @0x5ba0 (80), 0x5bf2 (8), 0x5bfc (4), 0x5c02 `00 00`; LF @0x5c04. `own_11g`@0x36e5; pieces @0x36e7 ... 0x5d93; LF @0x5d95. Same piece lengths in all six owner-mode files | confirmed |
| A field that crosses a multiple of RECORDLENGTH in the text region | Nothing is inserted there either: the piece of 80 bytes at `own`@0x3fc2 (length field 0x3fc2, data 0x3fc4-0x4013) continues across 0x4000 | `own`@0x3fc2; `own_11g`@0x3fb9 | confirmed (2 versions, 3 files each) |
| Procedures and the trigger | `O_PROC`: `fd ff`, pieces 7 (`CREATE `), 37, 6, 20, 54, 4 (= the 5 source lines of the truth), 0. `O_TRG`: `fd ff`, pieces 52, 6, 30, 26, 10, 4 (= the 6 source lines of the truth; no `CREATE ` piece: the introducing line is `CREATE FORMAT92TRIG `), 0. The LFs of the source are inside the pieces | `own`@0x5c29 (pieces @0x5c2b, 0x5c34, 0x5c5b, 0x5c63, 0x5c79, 0x5cb1, 0x5cb7); `own`@0x5ee6 (pieces @0x5ee8, 0x5f1e, 0x5f26, 0x5f46, 0x5f62, 0x5f6e, 0x5f74); table mode: `own_tables`@0x3309; 11g: `own_11g`@0x5dba, 0x6077 | confirmed |
| `O_CHILD_CK2` (CHECK condition of 414 bytes, `truth_own_ddl.txt` `CONSTRAINT\|O_CHILD_CK2\|C\|414`) | In pieces after `fd ff`: 25 (`ALTER TABLE "O_CHILD" ADD `), 25 (` CONSTRAINT "O_CHILD_CK2"`), 8 (` CHECK (`), **414** (the condition, one piece, with its LFs), 1 (`)`), 18 (` ENABLE NOVALIDATE`), 0. The piece size is **not** constant: the pieces are the parts the statement is built from, and the stored condition is one piece whatever its length. The field ends with the piece of length 0 and then LF | `own`@0x2989 `fd ff`, pieces @0x298b, 0x29a6, 0x29c1, 0x29cb (`9e 01`), 0x2b6b, 0x2b6e, 0x2b82 `00 00`, LF @0x2b84; `own_11g`@0x298e, pieces @0x2990 ... 0x2b87, LF @0x2b89; `own_tables`@0x28fd. Same structure as `ddl`@0x2885 (pieces 23, 22, 8, 25, 1, 18, 0) and as `O_CHILD_CK` (`own`@0x2906: 25, 24, 8, 25, 1, 18, 0) | confirmed |
| Lengths above 255 in text fields | u16 LE, as in the row data: the comment on `O_CHILD.NOTE` (300 bytes in the truth) is a plain field of 302 bytes, `2e 01` (the two quotes are counted); the piece of 414 bytes is `9e 01` | `own`@0x2d5a `2e 01 27 4c 33 30 30 5f ...`, LF @0x2e8a; `own_11g`@0x2d5f; `own`@0x29cb `9e 01` | confirmed. Lengths of 8192 and more in a plain field: not in the fixtures |
| LF and a quote inside a comment (`O_CHILD.STATUS`, 16 bytes in the truth: `line1` LF `line2 it's`) | The comment is written as an SQL literal: opening quote, the bytes with the LF **raw** (0x0a, not escaped), the quote **doubled** (`27 27`), closing quote: 19 bytes = 16 + 2 + 1 | `own`@0x2d1d `13 00 27 6c 69 6e 65 31 0a 6c 69 6e 65 32 20 69 74 27 27 73 27`, LF @0x2d32; `own_11g`@0x2d22; truth `COLCOMMENT\|O_CHILD.STATUS\|16\|... 6c,69,6e,65,31,a,6c,69,6e,65,32,20,69,74,27,73` | confirmed |
| Non-ASCII in a text item; is the text converted to the client set? | **Yes.** See section 14.2: `Café € д` is `43 61 66 c3 a9 20 e2 82 ac 20 d0 b4` (12 bytes, the truth's database bytes) with the AL32UTF8 client and `43 61 66 e9 20 80 20 bf` (8 bytes) with the WE8MSWIN1252 client; the length field counts the bytes written (14 / 10 with the quotes) | `own`@0x2ce3 `0e 00 27 ...`; `own_exp_w1252_18c.dmp`@0x2ce3 `0a 00 27 43 61 66 e9 20 80 20 bf 27`; 11g: @0x2ce8 in both files | confirmed |
| Empty table with its own statements (`O_EMPTY`) | Section 5.4. Conventional: `ff ff` + LF after the path flag, then the statements. Direct, 18c: LF directly after the path flag, then the statements. Direct, 11g: `ff ff` + LF as on the conventional path | `own`@0x2f77; `own/direct`@0x2f71; `own_11g/direct`@0x2fd9 | confirmed |
| Is the `METRICET` rule of 7.1 consistent? | Section 7.1: the counter grows by one per statement in most sections, but by 8 (18c) or 12 (11g) in the two "procedural objects and actions" sections, one of which holds no item. No rule fits all files | `own`@0x21f7, 0x6140; `own_11g`@0x21f2, 0x62d1 | observed; no rule |

#### Items found in the R-012 files, by kind

"Line" items hold a whole statement; "intro + field" items are a line that introduces a field
(section 4.2). Offsets: `own` (18c, conventional); the walk tables give the other files.

| kind | form | where | `own` offset |
|---|---|---|---|
| `PROCACTION` + PL/SQL block | intro + field in `fd ff` pieces (8, 141, 13, 0; 11g: 8, 136, 13, 0) | section "pre-schema procedural objects and actions" | 0x213f, 0x214a |
| `CREATE SEQUENCE` | line | section "sequence numbers", before the tables | 0x2342 |
| `TABLE "name"` | line | start of a table section, and start of a group inside a section | 0x241c; 0x5e11 |
| `CREATE TABLE` | line | table section, first item | 0x242c |
| `ALTER TABLE "T" MODIFY DEFAULT` + statement | intro + plain field | table section, before `INSERT INTO` | 0x2566, 0x258b; 0x25c4, 0x25e9 |
| `INSERT INTO` + table data | line + binary part | table section | 0x261f, 0x2673 |
| `CREATE [UNIQUE] INDEX` | line | table section, after the rows | 0x26ce, 0x27a6 |
| `ANALSTATS IS "T"` + PL/SQL block | intro + plain field | table section, after the indexes | 0x2884, 0x289b |
| `CALTER TABLE` + CHECK constraint | intro + field in `fd ff` pieces | table section | 0x28f9, 0x2906; 0x297c, 0x2989 |
| `ALTER TABLE ... ADD [CONSTRAINT ...] PRIMARY KEY` | line | table section | 0x2b85; 0x3337 |
| `COMMENT ON TABLE / COLUMN ... IS ` + literal | intro + plain field | table section, last items | 0x2c80, 0x2c9f; 0x2cb7, 0x2ce3; 0x2cf4, 0x2d1d; 0x2d33, 0x2d5a |
| `GRANT ... ON "T" TO ...` | line | table section, after the rows, before the constraints | 0x3310 |
| `ENDTABLE` | line | after the last table section (owner mode only), and end of a group | 0x341a; 0x5e9c |
| `CREATE SYNONYM` | line | section "synonyms" | 0x3434 |
| `CREATE VIEW "V" (cols) AS ` + query text | intro + field in pieces **without** marker | section "views" | 0x3473, 0x349c |
| `ALTER SESSION SET "_LOAD_WITHOUT_COMPILE" = PLSQL` / `= NONE` | line | section "stored procedures" | 0x34f9; 0x5cba |
| `CREATE FORMAT71 PROCEDURE "P" ` + source | intro + field in `fd ff` pieces | section "stored procedures" | 0x352b, 0x3554; 0x5c05, 0x5c29 |
| `ALTER PROCEDURE "P" COMPILE REUSE SETTINGS TIMESTAMP '...'` | line | section "stored procedures" | 0x5ceb, 0x5d40 |
| `ALTER TABLE ... ADD CONSTRAINT ... FOREIGN KEY` | line | group `TABLE "O_CHILD"` in "referential integrity constraints" | 0x5e21 |
| `CREATE FORMAT92TRIG ` + source, `ALTER TRIGGER "T"  ENABLE` | intro + field in `fd ff` pieces, then a line | group in "triggers" | 0x5ed1, 0x5ee6, 0x5f77 |
| `ANALCOMPUTE IS "T" ANALYZE  INDEX ...` | line | group in "statistics" (owner mode) / "Deferred analyze commands " (table mode) | 0x616f, 0x61ca |
| `ALTER TABLE ... ENABLE CONSTRAINT` | line | group in "Enable enforce constraints " | 0x625e, 0x6293, 0x62c9 |

#### Walk of the owner-mode files (`tools/exp_items.py`)

All six files have the same 134 items in the same order and of the same form; the walk consumes
every byte from R + 0x104 to the second `EXIT` in each of them (`tools/exp_parse.py`: no problem).
"Where": backquoted text is the name of the `METRICSU` section the item stands in.

<!-- own_exp_18c.dmp: 134 items, problems: none -->
<!-- own_exp_direct_18c.dmp: 134 items, problems: none -->
<!-- own_exp_w1252_18c.dmp: 134 items, problems: none -->
<!-- own_exp_11g.dmp: 134 items, problems: none -->
<!-- own_exp_direct_11g.dmp: 134 items, problems: none -->
<!-- own_exp_w1252_11g.dmp: 134 items, problems: none -->

| # | own_exp_18c | own_exp_direct_18c | own_exp_w1252_18c | own_exp_11g | own_exp_direct_11g | own_exp_w1252_11g | form | length | kind | where | text (start) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 0x2104 | 0x2104 | 0x2104 | 0x2104 | 0x2104 | 0x2104 | line | 8 | METRICS (start) | `(no name)` | `METRICSU` |
| 2 | 0x210d | 0x210d | 0x210d | 0x210d | 0x210d | 0x210d | line | 49 | METRICS section start | `pre-schema procedural objects and actions` | `METRICSUpre-schema procedural objects and actions` |
| 3 | 0x213f | 0x213f | 0x213f | 0x213f | 0x213f | 0x213f | line | 10 | PROCACTION (intro) | `pre-schema procedural objects and actions` | `PROCACTION` |
| 4 | 0x214a | 0x214a | 0x214a | 0x214a | 0x214a | 0x214a | field, `fd ff` pieces: 8, 141, 13, 0 (piece lengths differ between the files) | 162 / 162 / 162 / 157 / 157 / 157 | field of: PROCACTION (intro) | `pre-schema procedural objects and actions` | `BEGIN  \nsys.dbms_logrep_imp.instantiate_schema(schema_name=>SYS_CONTEX...` (text differs between the files) |
| 5 | 0x21f7 | 0x21f7 | 0x21f7 | 0x21f2 | 0x21f2 | 0x21f2 | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `pre-schema procedural objects and actions` | `METRICEU 8` (text differs between the files) |
| 6 | 0x2202 | 0x2202 | 0x2202 | 0x21fe | 0x21fe | 0x21fe | line | 51 | METRICS section start | `foreign function library names for user` | `METRICSUforeign function library names for user    ` |
| 7 | 0x2236 | 0x2236 | 0x2236 | 0x2232 | 0x2232 | 0x2232 | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `foreign function library names for user` | `METRICEU 8` (text differs between the files) |
| 8 | 0x2241 | 0x2241 | 0x2241 | 0x223e | 0x223e | 0x223e | line | 28 | METRICS section start | `PUBLIC type synonyms` | `METRICSUPUBLIC type synonyms` |
| 9 | 0x225e | 0x225e | 0x225e | 0x225b | 0x225b | 0x225b | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `PUBLIC type synonyms` | `METRICEU 8` (text differs between the files) |
| 10 | 0x2269 | 0x2269 | 0x2269 | 0x2267 | 0x2267 | 0x2267 | line | 29 | METRICS section start | `private type synonyms` | `METRICSUprivate type synonyms` |
| 11 | 0x2287 | 0x2287 | 0x2287 | 0x2285 | 0x2285 | 0x2285 | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `private type synonyms` | `METRICEU 8` (text differs between the files) |
| 12 | 0x2292 | 0x2292 | 0x2292 | 0x2291 | 0x2291 | 0x2291 | line | 44 | METRICS section start | `object type definitions for user` | `METRICSUobject type definitions for user    ` |
| 13 | 0x22bf | 0x22bf | 0x22bf | 0x22be | 0x22be | 0x22be | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `object type definitions for user` | `METRICEU 8` (text differs between the files) |
| 14 | 0x22ca | 0x22ca | 0x22ca | 0x22ca | 0x22ca | 0x22ca | line | 49 | METRICS section start | `pre-schema procedural objects and actions` | `METRICSUpre-schema procedural objects and actions` |
| 15 | 0x22fc | 0x22fc | 0x22fc | 0x22fc | 0x22fc | 0x22fc | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `pre-schema procedural objects and actions` | `METRICEU 8` (text differs between the files) |
| 16 | 0x2307 | 0x2307 | 0x2307 | 0x2308 | 0x2308 | 0x2308 | line | 22 | METRICS section start | `database links` | `METRICSUdatabase links` |
| 17 | 0x231e | 0x231e | 0x231e | 0x231f | 0x231f | 0x231f | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `database links` | `METRICEU 8` (text differs between the files) |
| 18 | 0x2329 | 0x2329 | 0x2329 | 0x232b | 0x232b | 0x232b | line | 24 | METRICS section start | `sequence numbers` | `METRICSUsequence numbers` |
| 19 | 0x2342 | 0x2342 | 0x2342 | 0x2344 | 0x2344 | 0x2344 | line | 127 | CREATE SEQUENCE | `sequence numbers` | `CREATE SEQUENCE "O_SEQ" MINVALUE 1 MAXVALUE 99999999999999999999999999...` |
| 20 | 0x23c2 | 0x23c2 | 0x23c2 | 0x23c4 | 0x23c4 | 0x23c4 | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `sequence numbers` | `METRICEU 9` (text differs between the files) |
| 21 | 0x23cd | 0x23cd | 0x23cd | 0x23d0 | 0x23d0 | 0x23d0 | line | 27 | METRICS section start | `cluster definitions` | `METRICSUcluster definitions` |
| 22 | 0x23e9 | 0x23e9 | 0x23e9 | 0x23ec | 0x23ec | 0x23ec | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `cluster definitions` | `METRICEU 9` (text differs between the files) |
| 23 | 0x23f4 | 0x23f4 | 0x23f4 | 0x23f8 | 0x23f8 | 0x23f8 | line | 28 | METRICS section start | `XDB Schemas by user` | `METRICSUXDB Schemas by user ` |
| 24 | 0x2411 | 0x2411 | 0x2411 | 0x2415 | 0x2415 | 0x2415 | line | 10 / 10 / 10 / 11 / 11 / 11 | METRICE | `XDB Schemas by user` | `METRICEU 9` (text differs between the files) |
| 25 | 0x241c | 0x241c | 0x241c | 0x2421 | 0x2421 | 0x2421 | line | 15 | TABLE | start of a table section | `TABLE "O_CHILD"` |
| 26 | 0x242c | 0x242c | 0x242c | 0x2431 | 0x2431 | 0x2431 | line | 313 | CREATE TABLE | table section O_CHILD, before the rows | `CREATE TABLE "O_CHILD" ("ID" NUMBER(6, 0), "PARENT_ID" NUMBER(6, 0), "...` |
| 27 | 0x2566 | 0x2566 | 0x2566 | 0x256b | 0x256b | 0x256b | line | 36 | ALTER TABLE MODIFY DEFAULT (intro) | table section O_CHILD, before the rows | `ALTER TABLE "O_CHILD" MODIFY DEFAULT` |
| 28 | 0x258b | 0x258b | 0x258b | 0x2590 | 0x2590 | 0x2590 | field, plain | 54 | field of: ALTER TABLE MODIFY DEFAULT (intro) | table section O_CHILD, before the rows | `ALTER TABLE "O_CHILD" MODIFY ("STATUS" DEFAULT 'NEW' )` |
| 29 | 0x25c4 | 0x25c4 | 0x25c4 | 0x25c9 | 0x25c9 | 0x25c9 | line | 36 | ALTER TABLE MODIFY DEFAULT (intro) | table section O_CHILD, before the rows | `ALTER TABLE "O_CHILD" MODIFY DEFAULT` |
| 30 | 0x25e9 | 0x25e9 | 0x25e9 | 0x25ee | 0x25ee | 0x25ee | field, plain | 51 | field of: ALTER TABLE MODIFY DEFAULT (intro) | table section O_CHILD, before the rows | `ALTER TABLE "O_CHILD" MODIFY ("NOTE" DEFAULT 'n/a')` |
| 31 | 0x261f | 0x261f | 0x261f | 0x2624 | 0x2624 | 0x2624 | line | 83 | INSERT INTO | group TABLE O_CHILD in `XDB Schemas by user` | `INSERT INTO "O_CHILD" ("ID", "PARENT_ID", "STATUS", "NOTE") VALUES (:1...` |
| - | 0x2673 | 0x2673 | 0x2673 | 0x2678 | 0x2678 | 0x2678 | binary | 91 / 85 / 91 / 91 / 85 / 91 | table data | table section O_CHILD | column count 4, descriptors, `00 00`, path flag, rows. own_exp_18c: flag `00 00` @0x268f, 3 rows from 0x2691, `ff ff` 0x26cb, LF 0x26cd; own_exp_direct_18c: flag `01 00` @0x268f, 3 rows from 0x2691, `ff ff` 0x26c5, LF 0x26c7; own_exp_w1252_18c: flag `00 00` @0x268f, 3 rows from 0x2691, `ff ff` 0x26cb, LF 0x26cd; own_exp_11g: flag `00 00` @0x2694, 3 rows from 0x2696, `ff ff` 0x26d0, LF 0x26d2; own_exp_direct_11g: flag `01 00` @0x2694, 3 rows from 0x2696, `ff ff` 0x26ca, LF 0x26cc; own_exp_w1252_11g: flag `00 00` @0x2694, 3 rows from 0x2696, `ff ff` 0x26d0, LF 0x26d2 |
| 32 | 0x26ce | 0x26c8 | 0x26ce | 0x26d3 | 0x26cd | 0x26d3 | line | 215 | CREATE INDEX | table section O_CHILD, after the rows | `CREATE UNIQUE INDEX "O_CHILD_PK" ON "O_CHILD" ("ID" )  PCTFREE 10 INIT...` |
| 33 | 0x27a6 | 0x27a0 | 0x27a6 | 0x27ab | 0x27a5 | 0x27ab | line | 221 | CREATE INDEX | table section O_CHILD, after the rows | `CREATE INDEX "O_CHILD_IX" ON "O_CHILD" ("STATUS" , "NOTE" )  PCTFREE 1...` |
| 34 | 0x2884 | 0x287e | 0x2884 | 0x2889 | 0x2883 | 0x2889 | line | 22 | ANALSTATS (intro) | table section O_CHILD, after the rows | `ANALSTATS IS "O_CHILD"` (text differs between the files) |
| 35 | 0x289b | 0x2895 | 0x289b | 0x28a0 | 0x289a | 0x28a0 | field, plain | 91 | field of: ANALSTATS (intro) | table section O_CHILD, after the rows | `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"O_CHILD_IX"',NULL,NULL,NULL,3...` (text differs between the files) |
| 36 | 0x28f9 | 0x28f3 | 0x28f9 | 0x28fe | 0x28f8 | 0x28fe | line | 12 | CALTER TABLE (intro) | table section O_CHILD, after the rows | `CALTER TABLE` |
| 37 | 0x2906 | 0x2900 | 0x2906 | 0x290b | 0x2905 | 0x290b | field, `fd ff` pieces: 25, 24, 8, 25, 1, 18, 0 | 101 | field of: CALTER TABLE (intro) | table section O_CHILD, after the rows | `ALTER TABLE "O_CHILD" ADD CONSTRAINT "O_CHILD_CK" CHECK (status IN ('N...` |
| 38 | 0x297c | 0x2976 | 0x297c | 0x2981 | 0x297b | 0x2981 | line | 12 | CALTER TABLE (intro) | table section O_CHILD, after the rows | `CALTER TABLE` |
| 39 | 0x2989 | 0x2983 | 0x2989 | 0x298e | 0x2988 | 0x298e | field, `fd ff` pieces: 25, 25, 8, 414, 1, 18, 0 | 491 | field of: CALTER TABLE (intro) | table section O_CHILD, after the rows | `ALTER TABLE "O_CHILD" ADD CONSTRAINT "O_CHILD_CK2" CHECK (\n    note IS...` |
| 40 | 0x2b85 | 0x2b7f | 0x2b85 | 0x2b8a | 0x2b84 | 0x2b8a | line | 250 | constraint PRIMARY KEY | table section O_CHILD, after the rows | `ALTER TABLE "O_CHILD" ADD  CONSTRAINT "O_CHILD_PK" PRIMARY KEY ("ID") ...` |
| 41 | 0x2c80 | 0x2c7a | 0x2c80 | 0x2c85 | 0x2c7f | 0x2c85 | line | 30 | COMMENT (intro) | table section O_CHILD, after the rows | `COMMENT ON TABLE "O_CHILD" IS ` |
| 42 | 0x2c9f | 0x2c99 | 0x2c9f | 0x2ca4 | 0x2c9e | 0x2ca4 | field, plain | 21 | field of: COMMENT (intro) | table section O_CHILD, after the rows | `'short ascii comment'` |
| 43 | 0x2cb7 | 0x2cb1 | 0x2cb7 | 0x2cbc | 0x2cb6 | 0x2cbc | line | 43 | COMMENT (intro) | table section O_CHILD, after the rows | `COMMENT ON COLUMN "O_CHILD"."PARENT_ID" IS ` |
| 44 | 0x2ce3 | 0x2cdd | 0x2ce3 | 0x2ce8 | 0x2ce2 | 0x2ce8 | field, plain | 14 / 14 / 10 / 14 / 14 / 10 | field of: COMMENT (intro) | table section O_CHILD, after the rows | `'Caf\xc3\xa9 \xe2\x82\xac \xd0\xb4'` (text differs between the files) |
| 45 | 0x2cf4 | 0x2cee | 0x2cf0 | 0x2cf9 | 0x2cf3 | 0x2cf5 | line | 40 | COMMENT (intro) | table section O_CHILD, after the rows | `COMMENT ON COLUMN "O_CHILD"."STATUS" IS ` |
| 46 | 0x2d1d | 0x2d17 | 0x2d19 | 0x2d22 | 0x2d1c | 0x2d1e | field, plain | 19 | field of: COMMENT (intro) | table section O_CHILD, after the rows | `'line1\nline2 it''s'` |
| 47 | 0x2d33 | 0x2d2d | 0x2d2f | 0x2d38 | 0x2d32 | 0x2d34 | line | 38 | COMMENT (intro) | table section O_CHILD, after the rows | `COMMENT ON COLUMN "O_CHILD"."NOTE" IS ` |
| 48 | 0x2d5a | 0x2d54 | 0x2d56 | 0x2d5f | 0x2d59 | 0x2d5b | field, plain | 302 | field of: COMMENT (intro) | table section O_CHILD, after the rows | `'L300_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx...` |
| 49 | 0x2e8b | 0x2e85 | 0x2e87 | 0x2e90 | 0x2e8a | 0x2e8c | line | 15 | TABLE | start of a table section | `TABLE "O_EMPTY"` |
| 50 | 0x2e9b | 0x2e95 | 0x2e97 | 0x2ea0 | 0x2e9a | 0x2e9c | line | 148 / 148 / 148 / 247 / 247 / 247 | CREATE TABLE | table section O_EMPTY, before the rows | `CREATE TABLE "O_EMPTY" ("ID" NUMBER(6, 0), "NAME" VARCHAR2(20))  PCTFR...` (text differs between the files) |
| 51 | 0x2f30 | 0x2f2a | 0x2f2c | 0x2f98 | 0x2f92 | 0x2f94 | line | 52 | INSERT INTO | group TABLE O_EMPTY in `XDB Schemas by user` | `INSERT INTO "O_EMPTY" ("ID", "NAME") VALUES (:1, :2)` |
| - | 0x2f65 | 0x2f5f | 0x2f61 | 0x2fcd | 0x2fc7 | 0x2fc9 | binary | 21 / 19 / 21 / 21 / 21 / 21 | table data | table section O_EMPTY | column count 2, descriptors, `00 00`, path flag, rows. own_exp_18c: flag `00 00` @0x2f75, 0 rows from 0x2f77, `ff ff` 0x2f77, LF 0x2f79; own_exp_direct_18c: flag `01 00` @0x2f6f, 0 rows from 0x2f71, `ff ff` none, LF 0x2f71; own_exp_w1252_18c: flag `00 00` @0x2f71, 0 rows from 0x2f73, `ff ff` 0x2f73, LF 0x2f75; own_exp_11g: flag `00 00` @0x2fdd, 0 rows from 0x2fdf, `ff ff` 0x2fdf, LF 0x2fe1; own_exp_direct_11g: flag `01 00` @0x2fd7, 0 rows from 0x2fd9, `ff ff` 0x2fd9, LF 0x2fdb; own_exp_w1252_11g: flag `00 00` @0x2fd9, 0 rows from 0x2fdb, `ff ff` 0x2fdb, LF 0x2fdd |
| 52 | 0x2f7a | 0x2f72 | 0x2f76 | 0x2fe2 | 0x2fdc | 0x2fde | line | 116 / 116 / 116 / 215 / 215 / 215 | CREATE INDEX | table section O_EMPTY, after the rows | `CREATE UNIQUE INDEX "O_EMPTY_PK" ON "O_EMPTY" ("ID" )  PCTFREE 10 INIT...` (text differs between the files) |
| 53 | 0x2fef | 0x2fe7 | 0x2feb | 0x30ba | 0x30b4 | 0x30b6 | line | 111 / 111 / 111 / 210 / 210 / 210 | CREATE INDEX | table section O_EMPTY, after the rows | `CREATE INDEX "O_EMPTY_IX" ON "O_EMPTY" ("NAME" )  PCTFREE 10 INITRANS ...` (text differs between the files) |
| 54 | 0x305f | 0x3057 | 0x305b | 0x318d | 0x3187 | 0x3189 | line | 22 | ANALSTATS (intro) | table section O_EMPTY, after the rows | `ANALSTATS IS "O_EMPTY"` (text differs between the files) |
| 55 | 0x3076 | 0x306e | 0x3072 | 0x31a4 | 0x319e | 0x31a0 | field, plain | 91 | field of: ANALSTATS (intro) | table section O_EMPTY, after the rows | `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"O_EMPTY_IX"',NULL,NULL,NULL,0...` (text differs between the files) |
| 56 | 0x30d4 | 0x30cc | 0x30d0 | 0x3202 | 0x31fc | 0x31fe | line | 151 / 151 / 151 / 250 / 250 / 250 | constraint PRIMARY KEY | table section O_EMPTY, after the rows | `ALTER TABLE "O_EMPTY" ADD  CONSTRAINT "O_EMPTY_PK" PRIMARY KEY ("ID") ...` (text differs between the files) |
| 57 | 0x316c | 0x3164 | 0x3168 | 0x32fd | 0x32f7 | 0x32f9 | line | 30 | COMMENT (intro) | table section O_EMPTY, after the rows | `COMMENT ON TABLE "O_EMPTY" IS ` |
| 58 | 0x318b | 0x3183 | 0x3187 | 0x331c | 0x3316 | 0x3318 | field, plain | 9 | field of: COMMENT (intro) | table section O_EMPTY, after the rows | `'no rows'` |
| 59 | 0x3197 | 0x318f | 0x3193 | 0x3328 | 0x3322 | 0x3324 | line | 16 | TABLE | start of a table section | `TABLE "O_PARENT"` |
| 60 | 0x31a8 | 0x31a0 | 0x31a4 | 0x3339 | 0x3333 | 0x3335 | line | 264 | CREATE TABLE | table section O_PARENT, before the rows | `CREATE TABLE "O_PARENT" ("ID" NUMBER(6, 0), "NAME" VARCHAR2(30) NOT NU...` |
| 61 | 0x32b1 | 0x32a9 | 0x32ad | 0x3442 | 0x343c | 0x343e | line | 53 | INSERT INTO | group TABLE O_PARENT in `XDB Schemas by user` | `INSERT INTO "O_PARENT" ("ID", "NAME") VALUES (:1, :2)` |
| - | 0x32e7 | 0x32df | 0x32e3 | 0x3478 | 0x3472 | 0x3474 | binary | 41 / 37 / 41 / 41 / 37 / 41 | table data | table section O_PARENT | column count 2, descriptors, `00 00`, path flag, rows. own_exp_18c: flag `00 00` @0x32f7, 2 rows from 0x32f9, `ff ff` 0x330d, LF 0x330f; own_exp_direct_18c: flag `01 00` @0x32ef, 2 rows from 0x32f1, `ff ff` 0x3301, LF 0x3303; own_exp_w1252_18c: flag `00 00` @0x32f3, 2 rows from 0x32f5, `ff ff` 0x3309, LF 0x330b; own_exp_11g: flag `00 00` @0x3488, 2 rows from 0x348a, `ff ff` 0x349e, LF 0x34a0; own_exp_direct_11g: flag `01 00` @0x3482, 2 rows from 0x3484, `ff ff` 0x3494, LF 0x3496; own_exp_w1252_11g: flag `00 00` @0x3484, 2 rows from 0x3486, `ff ff` 0x349a, LF 0x349c |
| 62 | 0x3310 | 0x3304 | 0x330c | 0x34a1 | 0x3497 | 0x349d | line | 38 | GRANT | table section O_PARENT, after the rows | `GRANT SELECT ON "O_PARENT" TO "CORPUS"` |
| 63 | 0x3337 | 0x332b | 0x3333 | 0x34c8 | 0x34be | 0x34c4 | line | 226 | constraint PRIMARY KEY | table section O_PARENT, after the rows | `ALTER TABLE "O_PARENT" ADD  PRIMARY KEY ("ID") USING INDEX PCTFREE 10 ...` |
| 64 | 0x341a | 0x340e | 0x3416 | 0x35ab | 0x35a1 | 0x35a7 | line | 8 | ENDTABLE | after the last table section (owner mode) | `ENDTABLE` |
| 65 | 0x3423 | 0x3417 | 0x341f | 0x35b4 | 0x35aa | 0x35b0 | line | 16 | METRICS section start | `synonyms` | `METRICSUsynonyms` |
| 66 | 0x3434 | 0x3428 | 0x3430 | 0x35c5 | 0x35bb | 0x35c1 | line | 37 | CREATE SYNONYM | `synonyms` | `CREATE SYNONYM "O_SYN" FOR "O_PARENT"` |
| 67 | 0x345a | 0x344e | 0x3456 | 0x35eb | 0x35e1 | 0x35e7 | line | 10 | METRICE | `synonyms` | `METRICEU 3` |
| 68 | 0x3465 | 0x3459 | 0x3461 | 0x35f6 | 0x35ec | 0x35f2 | line | 13 | METRICS section start | `views` | `METRICSUviews` |
| 69 | 0x3473 | 0x3467 | 0x346f | 0x3604 | 0x35fa | 0x3600 | line | 40 | CREATE VIEW (intro) | `views` | `CREATE VIEW "O_VIEW" ("ID","STATUS") AS ` |
| 70 | 0x349c | 0x3490 | 0x3498 | 0x362d | 0x3623 | 0x3629 | field, pieces without marker: 51, 0 | 51 | field of: CREATE VIEW (intro) | `views` | `SELECT id, status FROM o_child WHERE status = 'NEW'` |
| 71 | 0x34d4 | 0x34c8 | 0x34d0 | 0x3665 | 0x365b | 0x3661 | line | 10 | METRICE | `views` | `METRICEU 4` |
| 72 | 0x34df | 0x34d3 | 0x34db | 0x3670 | 0x3666 | 0x366c | line | 25 | METRICS section start | `stored procedures` | `METRICSUstored procedures` |
| 73 | 0x34f9 | 0x34ed | 0x34f5 | 0x368a | 0x3680 | 0x3686 | line | 49 | ALTER SESSION | `stored procedures` | `ALTER SESSION SET "_LOAD_WITHOUT_COMPILE" = PLSQL` |
| 74 | 0x352b | 0x351f | 0x3527 | 0x36bc | 0x36b2 | 0x36b8 | line | 40 | procedure (intro) | `stored procedures` | `CREATE FORMAT71 PROCEDURE "O_PROC_LONG" ` |
| 75 | 0x3554 | 0x3548 | 0x3550 | 0x36e5 | 0x36db | 0x36e1 | field, `fd ff` pieces: 7, 25, 6, 80, ... (126 pieces incl. the last of 0) ..., 8, 4, 0 | 9650 | field of: procedure (intro) | `stored procedures` | `CREATE PROCEDURE o_proc_long IS\nBEGIN\n  -- 001 padding line pppppppppp...` |
| 76 | 0x5c05 | 0x5bf9 | 0x5c01 | 0x5d96 | 0x5d8c | 0x5d92 | line | 35 | procedure (intro) | `stored procedures` | `CREATE FORMAT71 PROCEDURE "O_PROC" ` |
| 77 | 0x5c29 | 0x5c1d | 0x5c25 | 0x5dba | 0x5db0 | 0x5db6 | field, `fd ff` pieces: 7, 37, 6, 20, 54, 4, 0 | 128 | field of: procedure (intro) | `stored procedures` | `CREATE PROCEDURE o_proc (p_id IN NUMBER) IS\nBEGIN\n  -- a comment line\n...` |
| 78 | 0x5cba | 0x5cae | 0x5cb6 | 0x5e4b | 0x5e41 | 0x5e47 | line | 48 | ALTER SESSION | `stored procedures` | `ALTER SESSION SET "_LOAD_WITHOUT_COMPILE" = NONE` |
| 79 | 0x5ceb | 0x5cdf | 0x5ce7 | 0x5e7c | 0x5e72 | 0x5e78 | line | 84 | ALTER PROCEDURE | `stored procedures` | `ALTER PROCEDURE "O_PROC_LONG" COMPILE REUSE SETTINGS TIMESTAMP '2026-1...` (text differs between the files) |
| 80 | 0x5d40 | 0x5d34 | 0x5d3c | 0x5ed1 | 0x5ec7 | 0x5ecd | line | 79 | ALTER PROCEDURE | `stored procedures` | `ALTER PROCEDURE "O_PROC" COMPILE REUSE SETTINGS TIMESTAMP '2026-10-06:...` (text differs between the files) |
| 81 | 0x5d90 | 0x5d84 | 0x5d8c | 0x5f21 | 0x5f17 | 0x5f1d | line | 10 | METRICE | `stored procedures` | `METRICEU 6` |
| 82 | 0x5d9b | 0x5d8f | 0x5d97 | 0x5f2c | 0x5f22 | 0x5f28 | line | 17 | METRICS section start | `operators` | `METRICSUoperators` |
| 83 | 0x5dad | 0x5da1 | 0x5da9 | 0x5f3e | 0x5f34 | 0x5f3a | line | 10 | METRICE | `operators` | `METRICEU 6` |
| 84 | 0x5db8 | 0x5dac | 0x5db4 | 0x5f49 | 0x5f3f | 0x5f45 | line | 35 | METRICS section start | `Analyze cluster statements` | `METRICSUAnalyze cluster statements ` |
| 85 | 0x5ddc | 0x5dd0 | 0x5dd8 | 0x5f6d | 0x5f63 | 0x5f69 | line | 10 | METRICE | `Analyze cluster statements` | `METRICEU 6` |
| 86 | 0x5de7 | 0x5ddb | 0x5de3 | 0x5f78 | 0x5f6e | 0x5f74 | line | 41 | METRICS section start | `referential integrity constraints` | `METRICSUreferential integrity constraints` |
| 87 | 0x5e11 | 0x5e05 | 0x5e0d | 0x5fa2 | 0x5f98 | 0x5f9e | line | 15 | TABLE | group TABLE O_CHILD in `referential integrity constraints` | `TABLE "O_CHILD"` |
| 88 | 0x5e21 | 0x5e15 | 0x5e1d | 0x5fb2 | 0x5fa8 | 0x5fae | line | 122 | constraint FOREIGN KEY | group TABLE O_CHILD in `referential integrity constraints` | `ALTER TABLE "O_CHILD" ADD CONSTRAINT "O_CHILD_FK" FOREIGN KEY ("PARENT...` |
| 89 | 0x5e9c | 0x5e90 | 0x5e98 | 0x602d | 0x6023 | 0x6029 | line | 8 | ENDTABLE | group TABLE O_CHILD in `referential integrity constraints` | `ENDTABLE` |
| 90 | 0x5ea5 | 0x5e99 | 0x5ea1 | 0x6036 | 0x602c | 0x6032 | line | 10 | METRICE | `referential integrity constraints` | `METRICEU 7` |
| 91 | 0x5eb0 | 0x5ea4 | 0x5eac | 0x6041 | 0x6037 | 0x603d | line | 16 | METRICS section start | `triggers` | `METRICSUtriggers` |
| 92 | 0x5ec1 | 0x5eb5 | 0x5ebd | 0x6052 | 0x6048 | 0x604e | line | 15 | TABLE | group TABLE O_CHILD in `triggers` | `TABLE "O_CHILD"` |
| 93 | 0x5ed1 | 0x5ec5 | 0x5ecd | 0x6062 | 0x6058 | 0x605e | line | 20 | trigger (intro) | group TABLE O_CHILD in `triggers` | `CREATE FORMAT92TRIG ` |
| 94 | 0x5ee6 | 0x5eda | 0x5ee2 | 0x6077 | 0x606d | 0x6073 | field, `fd ff` pieces: 52, 6, 30, 26, 10, 4, 0 | 128 | field of: trigger (intro) | group TABLE O_CHILD in `triggers` | `TRIGGER o_trg BEFORE INSERT ON o_child FOR EACH ROW\nBEGIN\n  IF :NEW.st...` |
| 95 | 0x5f77 | 0x5f6b | 0x5f73 | 0x6108 | 0x60fe | 0x6104 | line | 29 | ALTER TRIGGER | group TABLE O_CHILD in `triggers` | `ALTER TRIGGER "O_TRG"  ENABLE` |
| 96 | 0x5f95 | 0x5f89 | 0x5f91 | 0x6126 | 0x611c | 0x6122 | line | 8 | ENDTABLE | group TABLE O_CHILD in `triggers` | `ENDTABLE` |
| 97 | 0x5f9e | 0x5f92 | 0x5f9a | 0x612f | 0x6125 | 0x612b | line | 10 | METRICE | `triggers` | `METRICEU 8` |
| 98 | 0x5fa9 | 0x5f9d | 0x5fa5 | 0x613a | 0x6130 | 0x6136 | line | 18 | METRICS section start | `indextypes` | `METRICSUindextypes` |
| 99 | 0x5fbc | 0x5fb0 | 0x5fb8 | 0x614d | 0x6143 | 0x6149 | line | 10 | METRICE | `indextypes` | `METRICEU 8` |
| 100 | 0x5fc7 | 0x5fbb | 0x5fc3 | 0x6158 | 0x614e | 0x6154 | line | 49 | METRICS section start | `bitmap, functional and extensible indexes` | `METRICSUbitmap, functional and extensible indexes` |
| 101 | 0x5ff9 | 0x5fed | 0x5ff5 | 0x618a | 0x6180 | 0x6186 | line | 10 | METRICE | `bitmap, functional and extensible indexes` | `METRICEU 8` |
| 102 | 0x6004 | 0x5ff8 | 0x6000 | 0x6195 | 0x618b | 0x6191 | line | 26 | METRICS section start | `posttables actions` | `METRICSUposttables actions` |
| 103 | 0x601f | 0x6013 | 0x601b | 0x61b0 | 0x61a6 | 0x61ac | line | 10 | METRICE | `posttables actions` | `METRICEU 8` |
| 104 | 0x602a | 0x601e | 0x6026 | 0x61bb | 0x61b1 | 0x61b7 | line | 26 | METRICS section start | `materialized views` | `METRICSUmaterialized views` |
| 105 | 0x6045 | 0x6039 | 0x6041 | 0x61d6 | 0x61cc | 0x61d2 | line | 10 | METRICE | `materialized views` | `METRICEU 8` |
| 106 | 0x6050 | 0x6044 | 0x604c | 0x61e1 | 0x61d7 | 0x61dd | line | 21 | METRICS section start | `snapshot logs` | `METRICSUsnapshot logs` |
| 107 | 0x6066 | 0x605a | 0x6062 | 0x61f7 | 0x61ed | 0x61f3 | line | 10 | METRICE | `snapshot logs` | `METRICEU 8` |
| 108 | 0x6071 | 0x6065 | 0x606d | 0x6202 | 0x61f8 | 0x61fe | line | 18 | METRICS section start | `job queues` | `METRICSUjob queues` |
| 109 | 0x6084 | 0x6078 | 0x6080 | 0x6215 | 0x620b | 0x6211 | line | 10 | METRICE | `job queues` | `METRICEU 8` |
| 110 | 0x608f | 0x6083 | 0x608b | 0x6220 | 0x6216 | 0x621c | line | 35 | METRICS section start | `refresh groups and children` | `METRICSUrefresh groups and children` |
| 111 | 0x60b3 | 0x60a7 | 0x60af | 0x6244 | 0x623a | 0x6240 | line | 10 | METRICE | `refresh groups and children` | `METRICEU 8` |
| 112 | 0x60be | 0x60b2 | 0x60ba | 0x624f | 0x6245 | 0x624b | line | 18 | METRICS section start | `dimensions` | `METRICSUdimensions` |
| 113 | 0x60d1 | 0x60c5 | 0x60cd | 0x6262 | 0x6258 | 0x625e | line | 10 | METRICE | `dimensions` | `METRICEU 8` |
| 114 | 0x60dc | 0x60d0 | 0x60d8 | 0x626d | 0x6263 | 0x6269 | line | 37 | METRICS section start | `Post-inst procedural actions` | `METRICSUPost-inst procedural actions ` |
| 115 | 0x6102 | 0x60f6 | 0x60fe | 0x6293 | 0x6289 | 0x628f | line | 10 | METRICE | `Post-inst procedural actions` | `METRICEU 8` |
| 116 | 0x610d | 0x6101 | 0x6109 | 0x629e | 0x6294 | 0x629a | line | 50 | METRICS section start | `post-schema procedural objects and actions` | `METRICSUpost-schema procedural objects and actions` |
| 117 | 0x6140 | 0x6134 | 0x613c | 0x62d1 | 0x62c7 | 0x62cd | line | 11 | METRICE | `post-schema procedural objects and actions` | `METRICEU 16` (text differs between the files) |
| 118 | 0x614c | 0x6140 | 0x6148 | 0x62dd | 0x62d3 | 0x62d9 | line | 18 | METRICS section start | `statistics` | `METRICSUstatistics` |
| 119 | 0x615f | 0x6153 | 0x615b | 0x62f0 | 0x62e6 | 0x62ec | line | 15 | TABLE | group TABLE O_CHILD in `statistics` | `TABLE "O_CHILD"` |
| 120 | 0x616f | 0x6163 | 0x616b | 0x6300 | 0x62f6 | 0x62fc | line | 74 | ANALCOMPUTE | group TABLE O_CHILD in `statistics` | `ANALCOMPUTE IS "O_CHILD" ANALYZE  INDEX "O_CHILD_IX"  ESTIMATE STATIST...` (text differs between the files) |
| 121 | 0x61ba | 0x61ae | 0x61b6 | 0x634b | 0x6341 | 0x6347 | line | 15 | TABLE | group TABLE O_EMPTY in `statistics` | `TABLE "O_EMPTY"` |
| 122 | 0x61ca | 0x61be | 0x61c6 | 0x635b | 0x6351 | 0x6357 | line | 74 | ANALCOMPUTE | group TABLE O_EMPTY in `statistics` | `ANALCOMPUTE IS "O_EMPTY" ANALYZE  INDEX "O_EMPTY_IX"  ESTIMATE STATIST...` (text differs between the files) |
| 123 | 0x6215 | 0x6209 | 0x6211 | 0x63a6 | 0x639c | 0x63a2 | line | 8 | ENDTABLE | group TABLE O_EMPTY in `statistics` | `ENDTABLE` |
| 124 | 0x621e | 0x6212 | 0x621a | 0x63af | 0x63a5 | 0x63ab | line | 11 | METRICE | `statistics` | `METRICEU 18` (text differs between the files) |
| 125 | 0x622a | 0x621e | 0x6226 | 0x63bb | 0x63b1 | 0x63b7 | line | 35 | METRICS section start | `Enable enforce constraints` | `METRICSUEnable enforce constraints ` |
| 126 | 0x624e | 0x6242 | 0x624a | 0x63df | 0x63d5 | 0x63db | line | 15 | TABLE | group TABLE O_CHILD in `Enable enforce constraints` | `TABLE "O_CHILD"` |
| 127 | 0x625e | 0x6252 | 0x625a | 0x63ef | 0x63e5 | 0x63eb | line | 52 | ENABLE CONSTRAINT | group TABLE O_CHILD in `Enable enforce constraints` | `ALTER TABLE "O_CHILD" ENABLE CONSTRAINT "O_CHILD_CK"` |
| 128 | 0x6293 | 0x6287 | 0x628f | 0x6424 | 0x641a | 0x6420 | line | 53 | ENABLE CONSTRAINT | group TABLE O_CHILD in `Enable enforce constraints` | `ALTER TABLE "O_CHILD" ENABLE CONSTRAINT "O_CHILD_CK2"` |
| 129 | 0x62c9 | 0x62bd | 0x62c5 | 0x645a | 0x6450 | 0x6456 | line | 52 | ENABLE CONSTRAINT | group TABLE O_CHILD in `Enable enforce constraints` | `ALTER TABLE "O_CHILD" ENABLE CONSTRAINT "O_CHILD_FK"` |
| 130 | 0x62fe | 0x62f2 | 0x62fa | 0x648f | 0x6485 | 0x648b | line | 8 | ENDTABLE | group TABLE O_CHILD in `Enable enforce constraints` | `ENDTABLE` |
| 131 | 0x6307 | 0x62fb | 0x6303 | 0x6498 | 0x648e | 0x6494 | line | 11 | METRICE | `Enable enforce constraints` | `METRICEU 21` (text differs between the files) |
| 132 | 0x6313 | 0x6307 | 0x630f | 0x64a4 | 0x649a | 0x64a0 | line | 10 | METRICE | end of the data | `METRICEUG0` |
| 133 | 0x631e | 0x6312 | 0x631a | 0x64af | 0x64a5 | 0x64ab | line | 4 | EXIT | end of the data | `EXIT` |
| 134 | 0x6323 | 0x6317 | 0x631f | 0x64b4 | 0x64aa | 0x64b0 | line | 4 | EXIT | end of the data | `EXIT` |

#### Walk of the same three tables in table mode (`tools/exp_items.py`)

All four files have the same 97 items. What owner mode adds, compared with this list: the 24
items before the first table (sections, `PROCACTION`, `CREATE SEQUENCE`), the `ENDTABLE` after the
tables, the synonym, the view, the procedures, and the sections that are empty in these fixtures;
the table sections themselves are identical (see above).

<!-- own_tables_exp_18c.dmp: 97 items, problems: none -->
<!-- own_tables_exp_direct_18c.dmp: 97 items, problems: none -->
<!-- own_tables_exp_11g.dmp: 97 items, problems: none -->
<!-- own_tables_exp_direct_11g.dmp: 97 items, problems: none -->

| # | own_tables_exp_18c | own_tables_exp_direct_18c | own_tables_exp_11g | own_tables_exp_direct_11g | form | length | kind | where | text (start) |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 0x2104 | 0x2104 | 0x2104 | 0x2104 | line | 8 | METRICS (start) | `(no name)` | `METRICST` |
| 2 | 0x210d | 0x210d | 0x210d | 0x210d | line | 16 | TABLE | start of a table section | `TABLE "O_PARENT"` |
| 3 | 0x211e | 0x211e | 0x211e | 0x211e | line | 264 | CREATE TABLE | table section O_PARENT, before the rows | `CREATE TABLE "O_PARENT" ("ID" NUMBER(6, 0), "NAME" VARCHAR2(30) NOT NU...` |
| 4 | 0x2227 | 0x2227 | 0x2227 | 0x2227 | line | 53 | INSERT INTO | group TABLE O_PARENT in `(no name)` | `INSERT INTO "O_PARENT" ("ID", "NAME") VALUES (:1, :2)` |
| - | 0x225d | 0x225d | 0x225d | 0x225d | binary | 41 / 37 / 41 / 37 | table data | table section O_PARENT | column count 2, descriptors, `00 00`, path flag, rows. own_tables_exp_18c: flag `00 00` @0x226d, 2 rows from 0x226f, `ff ff` 0x2283, LF 0x2285; own_tables_exp_direct_18c: flag `01 00` @0x226d, 2 rows from 0x226f, `ff ff` 0x227f, LF 0x2281; own_tables_exp_11g: flag `00 00` @0x226d, 2 rows from 0x226f, `ff ff` 0x2283, LF 0x2285; own_tables_exp_direct_11g: flag `01 00` @0x226d, 2 rows from 0x226f, `ff ff` 0x227f, LF 0x2281 |
| 5 | 0x2286 | 0x2282 | 0x2286 | 0x2282 | line | 38 | GRANT | table section O_PARENT, after the rows | `GRANT SELECT ON "O_PARENT" TO "CORPUS"` |
| 6 | 0x22ad | 0x22a9 | 0x22ad | 0x22a9 | line | 226 | constraint PRIMARY KEY | table section O_PARENT, after the rows | `ALTER TABLE "O_PARENT" ADD  PRIMARY KEY ("ID") USING INDEX PCTFREE 10 ...` |
| 7 | 0x2390 | 0x238c | 0x2390 | 0x238c | line | 15 | TABLE | start of a table section | `TABLE "O_CHILD"` |
| 8 | 0x23a0 | 0x239c | 0x23a0 | 0x239c | line | 313 | CREATE TABLE | table section O_CHILD, before the rows | `CREATE TABLE "O_CHILD" ("ID" NUMBER(6, 0), "PARENT_ID" NUMBER(6, 0), "...` |
| 9 | 0x24da | 0x24d6 | 0x24da | 0x24d6 | line | 36 | ALTER TABLE MODIFY DEFAULT (intro) | table section O_CHILD, before the rows | `ALTER TABLE "O_CHILD" MODIFY DEFAULT` |
| 10 | 0x24ff | 0x24fb | 0x24ff | 0x24fb | field, plain | 54 | field of: ALTER TABLE MODIFY DEFAULT (intro) | table section O_CHILD, before the rows | `ALTER TABLE "O_CHILD" MODIFY ("STATUS" DEFAULT 'NEW' )` |
| 11 | 0x2538 | 0x2534 | 0x2538 | 0x2534 | line | 36 | ALTER TABLE MODIFY DEFAULT (intro) | table section O_CHILD, before the rows | `ALTER TABLE "O_CHILD" MODIFY DEFAULT` |
| 12 | 0x255d | 0x2559 | 0x255d | 0x2559 | field, plain | 51 | field of: ALTER TABLE MODIFY DEFAULT (intro) | table section O_CHILD, before the rows | `ALTER TABLE "O_CHILD" MODIFY ("NOTE" DEFAULT 'n/a')` |
| 13 | 0x2593 | 0x258f | 0x2593 | 0x258f | line | 83 | INSERT INTO | group TABLE O_CHILD in `(no name)` | `INSERT INTO "O_CHILD" ("ID", "PARENT_ID", "STATUS", "NOTE") VALUES (:1...` |
| - | 0x25e7 | 0x25e3 | 0x25e7 | 0x25e3 | binary | 91 / 85 / 91 / 85 | table data | table section O_CHILD | column count 4, descriptors, `00 00`, path flag, rows. own_tables_exp_18c: flag `00 00` @0x2603, 3 rows from 0x2605, `ff ff` 0x263f, LF 0x2641; own_tables_exp_direct_18c: flag `01 00` @0x25ff, 3 rows from 0x2601, `ff ff` 0x2635, LF 0x2637; own_tables_exp_11g: flag `00 00` @0x2603, 3 rows from 0x2605, `ff ff` 0x263f, LF 0x2641; own_tables_exp_direct_11g: flag `01 00` @0x25ff, 3 rows from 0x2601, `ff ff` 0x2635, LF 0x2637 |
| 14 | 0x2642 | 0x2638 | 0x2642 | 0x2638 | line | 215 | CREATE INDEX | table section O_CHILD, after the rows | `CREATE UNIQUE INDEX "O_CHILD_PK" ON "O_CHILD" ("ID" )  PCTFREE 10 INIT...` |
| 15 | 0x271a | 0x2710 | 0x271a | 0x2710 | line | 221 | CREATE INDEX | table section O_CHILD, after the rows | `CREATE INDEX "O_CHILD_IX" ON "O_CHILD" ("STATUS" , "NOTE" )  PCTFREE 1...` |
| 16 | 0x27f8 | 0x27ee | 0x27f8 | 0x27ee | line | 22 | ANALSTATS (intro) | table section O_CHILD, after the rows | `ANALSTATS IS "O_CHILD"` |
| 17 | 0x280f | 0x2805 | 0x280f | 0x2805 | field, plain | 91 | field of: ANALSTATS (intro) | table section O_CHILD, after the rows | `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"O_CHILD_IX"',NULL,NULL,NULL,3...` (text differs between the files) |
| 18 | 0x286d | 0x2863 | 0x286d | 0x2863 | line | 12 | CALTER TABLE (intro) | table section O_CHILD, after the rows | `CALTER TABLE` |
| 19 | 0x287a | 0x2870 | 0x287a | 0x2870 | field, `fd ff` pieces: 25, 24, 8, 25, 1, 18, 0 | 101 | field of: CALTER TABLE (intro) | table section O_CHILD, after the rows | `ALTER TABLE "O_CHILD" ADD CONSTRAINT "O_CHILD_CK" CHECK (status IN ('N...` |
| 20 | 0x28f0 | 0x28e6 | 0x28f0 | 0x28e6 | line | 12 | CALTER TABLE (intro) | table section O_CHILD, after the rows | `CALTER TABLE` |
| 21 | 0x28fd | 0x28f3 | 0x28fd | 0x28f3 | field, `fd ff` pieces: 25, 25, 8, 414, 1, 18, 0 | 491 | field of: CALTER TABLE (intro) | table section O_CHILD, after the rows | `ALTER TABLE "O_CHILD" ADD CONSTRAINT "O_CHILD_CK2" CHECK (\n    note IS...` |
| 22 | 0x2af9 | 0x2aef | 0x2af9 | 0x2aef | line | 250 | constraint PRIMARY KEY | table section O_CHILD, after the rows | `ALTER TABLE "O_CHILD" ADD  CONSTRAINT "O_CHILD_PK" PRIMARY KEY ("ID") ...` |
| 23 | 0x2bf4 | 0x2bea | 0x2bf4 | 0x2bea | line | 30 | COMMENT (intro) | table section O_CHILD, after the rows | `COMMENT ON TABLE "O_CHILD" IS ` |
| 24 | 0x2c13 | 0x2c09 | 0x2c13 | 0x2c09 | field, plain | 21 | field of: COMMENT (intro) | table section O_CHILD, after the rows | `'short ascii comment'` |
| 25 | 0x2c2b | 0x2c21 | 0x2c2b | 0x2c21 | line | 43 | COMMENT (intro) | table section O_CHILD, after the rows | `COMMENT ON COLUMN "O_CHILD"."PARENT_ID" IS ` |
| 26 | 0x2c57 | 0x2c4d | 0x2c57 | 0x2c4d | field, plain | 14 | field of: COMMENT (intro) | table section O_CHILD, after the rows | `'Caf\xc3\xa9 \xe2\x82\xac \xd0\xb4'` |
| 27 | 0x2c68 | 0x2c5e | 0x2c68 | 0x2c5e | line | 40 | COMMENT (intro) | table section O_CHILD, after the rows | `COMMENT ON COLUMN "O_CHILD"."STATUS" IS ` |
| 28 | 0x2c91 | 0x2c87 | 0x2c91 | 0x2c87 | field, plain | 19 | field of: COMMENT (intro) | table section O_CHILD, after the rows | `'line1\nline2 it''s'` |
| 29 | 0x2ca7 | 0x2c9d | 0x2ca7 | 0x2c9d | line | 38 | COMMENT (intro) | table section O_CHILD, after the rows | `COMMENT ON COLUMN "O_CHILD"."NOTE" IS ` |
| 30 | 0x2cce | 0x2cc4 | 0x2cce | 0x2cc4 | field, plain | 302 | field of: COMMENT (intro) | table section O_CHILD, after the rows | `'L300_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx...` |
| 31 | 0x2dff | 0x2df5 | 0x2dff | 0x2df5 | line | 15 | TABLE | start of a table section | `TABLE "O_EMPTY"` |
| 32 | 0x2e0f | 0x2e05 | 0x2e0f | 0x2e05 | line | 148 / 148 / 247 / 247 | CREATE TABLE | table section O_EMPTY, before the rows | `CREATE TABLE "O_EMPTY" ("ID" NUMBER(6, 0), "NAME" VARCHAR2(20))  PCTFR...` (text differs between the files) |
| 33 | 0x2ea4 | 0x2e9a | 0x2f07 | 0x2efd | line | 52 | INSERT INTO | group TABLE O_EMPTY in `(no name)` | `INSERT INTO "O_EMPTY" ("ID", "NAME") VALUES (:1, :2)` |
| - | 0x2ed9 | 0x2ecf | 0x2f3c | 0x2f32 | binary | 21 / 19 / 21 / 21 | table data | table section O_EMPTY | column count 2, descriptors, `00 00`, path flag, rows. own_tables_exp_18c: flag `00 00` @0x2ee9, 0 rows from 0x2eeb, `ff ff` 0x2eeb, LF 0x2eed; own_tables_exp_direct_18c: flag `01 00` @0x2edf, 0 rows from 0x2ee1, `ff ff` none, LF 0x2ee1; own_tables_exp_11g: flag `00 00` @0x2f4c, 0 rows from 0x2f4e, `ff ff` 0x2f4e, LF 0x2f50; own_tables_exp_direct_11g: flag `01 00` @0x2f42, 0 rows from 0x2f44, `ff ff` 0x2f44, LF 0x2f46 |
| 34 | 0x2eee | 0x2ee2 | 0x2f51 | 0x2f47 | line | 116 / 116 / 215 / 215 | CREATE INDEX | table section O_EMPTY, after the rows | `CREATE UNIQUE INDEX "O_EMPTY_PK" ON "O_EMPTY" ("ID" )  PCTFREE 10 INIT...` (text differs between the files) |
| 35 | 0x2f63 | 0x2f57 | 0x3029 | 0x301f | line | 111 / 111 / 210 / 210 | CREATE INDEX | table section O_EMPTY, after the rows | `CREATE INDEX "O_EMPTY_IX" ON "O_EMPTY" ("NAME" )  PCTFREE 10 INITRANS ...` (text differs between the files) |
| 36 | 0x2fd3 | 0x2fc7 | 0x30fc | 0x30f2 | line | 22 | ANALSTATS (intro) | table section O_EMPTY, after the rows | `ANALSTATS IS "O_EMPTY"` |
| 37 | 0x2fea | 0x2fde | 0x3113 | 0x3109 | field, plain | 91 | field of: ANALSTATS (intro) | table section O_EMPTY, after the rows | `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"O_EMPTY_IX"',NULL,NULL,NULL,0...` (text differs between the files) |
| 38 | 0x3048 | 0x303c | 0x3171 | 0x3167 | line | 151 / 151 / 250 / 250 | constraint PRIMARY KEY | table section O_EMPTY, after the rows | `ALTER TABLE "O_EMPTY" ADD  CONSTRAINT "O_EMPTY_PK" PRIMARY KEY ("ID") ...` (text differs between the files) |
| 39 | 0x30e0 | 0x30d4 | 0x326c | 0x3262 | line | 30 | COMMENT (intro) | table section O_EMPTY, after the rows | `COMMENT ON TABLE "O_EMPTY" IS ` |
| 40 | 0x30ff | 0x30f3 | 0x328b | 0x3281 | field, plain | 9 | field of: COMMENT (intro) | table section O_EMPTY, after the rows | `'no rows'` |
| 41 | 0x310b | 0x30ff | 0x3297 | 0x328d | line | 41 | METRICS section start | group TABLE O_EMPTY in `referential integrity constraints` | `METRICSTreferential integrity constraints` |
| 42 | 0x3135 | 0x3129 | 0x32c1 | 0x32b7 | line | 10 | METRICE | group TABLE O_EMPTY in `referential integrity constraints` | `METRICET 0` |
| 43 | 0x3140 | 0x3134 | 0x32cc | 0x32c2 | line | 16 | METRICS section start | group TABLE O_EMPTY in `triggers` | `METRICSTtriggers` |
| 44 | 0x3151 | 0x3145 | 0x32dd | 0x32d3 | line | 10 | METRICE | group TABLE O_EMPTY in `triggers` | `METRICET 0` |
| 45 | 0x315c | 0x3150 | 0x32e8 | 0x32de | line | 49 | METRICS section start | group TABLE O_EMPTY in `bitmap, functional and extensible indexes` | `METRICSTbitmap, functional and extensible indexes` |
| 46 | 0x318e | 0x3182 | 0x331a | 0x3310 | line | 16 | TABLE | group TABLE O_PARENT in `bitmap, functional and extensible indexes` | `TABLE "O_PARENT"` |
| 47 | 0x319f | 0x3193 | 0x332b | 0x3321 | line | 8 | ENDTABLE | group TABLE O_PARENT in `bitmap, functional and extensible indexes` | `ENDTABLE` |
| 48 | 0x31a8 | 0x319c | 0x3334 | 0x332a | line | 10 | METRICE | `bitmap, functional and extensible indexes` | `METRICET 1` |
| 49 | 0x31b3 | 0x31a7 | 0x333f | 0x3335 | line | 26 | METRICS section start | `posttables actions` | `METRICSTposttables actions` |
| 50 | 0x31ce | 0x31c2 | 0x335a | 0x3350 | line | 10 | METRICE | `posttables actions` | `METRICET 1` |
| 51 | 0x31d9 | 0x31cd | 0x3365 | 0x335b | line | 37 | METRICS section start | `Post-inst procedural actions` | `METRICSTPost-inst procedural actions ` |
| 52 | 0x31ff | 0x31f3 | 0x338b | 0x3381 | line | 10 | METRICE | `Post-inst procedural actions` | `METRICET 1` |
| 53 | 0x320a | 0x31fe | 0x3396 | 0x338c | line | 41 | METRICS section start | `referential integrity constraints` | `METRICSTreferential integrity constraints` |
| 54 | 0x3234 | 0x3228 | 0x33c0 | 0x33b6 | line | 15 | TABLE | group TABLE O_CHILD in `referential integrity constraints` | `TABLE "O_CHILD"` |
| 55 | 0x3244 | 0x3238 | 0x33d0 | 0x33c6 | line | 122 | constraint FOREIGN KEY | group TABLE O_CHILD in `referential integrity constraints` | `ALTER TABLE "O_CHILD" ADD CONSTRAINT "O_CHILD_FK" FOREIGN KEY ("PARENT...` |
| 56 | 0x32bf | 0x32b3 | 0x344b | 0x3441 | line | 8 | ENDTABLE | group TABLE O_CHILD in `referential integrity constraints` | `ENDTABLE` |
| 57 | 0x32c8 | 0x32bc | 0x3454 | 0x344a | line | 10 | METRICE | `referential integrity constraints` | `METRICET 2` |
| 58 | 0x32d3 | 0x32c7 | 0x345f | 0x3455 | line | 16 | METRICS section start | `triggers` | `METRICSTtriggers` |
| 59 | 0x32e4 | 0x32d8 | 0x3470 | 0x3466 | line | 15 | TABLE | group TABLE O_CHILD in `triggers` | `TABLE "O_CHILD"` |
| 60 | 0x32f4 | 0x32e8 | 0x3480 | 0x3476 | line | 20 | trigger (intro) | group TABLE O_CHILD in `triggers` | `CREATE FORMAT92TRIG ` |
| 61 | 0x3309 | 0x32fd | 0x3495 | 0x348b | field, `fd ff` pieces: 52, 6, 30, 26, 10, 4, 0 | 128 | field of: trigger (intro) | group TABLE O_CHILD in `triggers` | `TRIGGER o_trg BEFORE INSERT ON o_child FOR EACH ROW\nBEGIN\n  IF :NEW.st...` |
| 62 | 0x339a | 0x338e | 0x3526 | 0x351c | line | 29 | ALTER TRIGGER | group TABLE O_CHILD in `triggers` | `ALTER TRIGGER "O_TRG"  ENABLE` |
| 63 | 0x33b8 | 0x33ac | 0x3544 | 0x353a | line | 8 | ENDTABLE | group TABLE O_CHILD in `triggers` | `ENDTABLE` |
| 64 | 0x33c1 | 0x33b5 | 0x354d | 0x3543 | line | 10 | METRICE | `triggers` | `METRICET 3` |
| 65 | 0x33cc | 0x33c0 | 0x3558 | 0x354e | line | 49 | METRICS section start | `bitmap, functional and extensible indexes` | `METRICSTbitmap, functional and extensible indexes` |
| 66 | 0x33fe | 0x33f2 | 0x358a | 0x3580 | line | 10 | METRICE | `bitmap, functional and extensible indexes` | `METRICET 3` |
| 67 | 0x3409 | 0x33fd | 0x3595 | 0x358b | line | 26 | METRICS section start | `posttables actions` | `METRICSTposttables actions` |
| 68 | 0x3424 | 0x3418 | 0x35b0 | 0x35a6 | line | 10 | METRICE | `posttables actions` | `METRICET 3` |
| 69 | 0x342f | 0x3423 | 0x35bb | 0x35b1 | line | 37 | METRICS section start | `Post-inst procedural actions` | `METRICSTPost-inst procedural actions ` |
| 70 | 0x3455 | 0x3449 | 0x35e1 | 0x35d7 | line | 10 | METRICE | `Post-inst procedural actions` | `METRICET 3` |
| 71 | 0x3460 | 0x3454 | 0x35ec | 0x35e2 | line | 41 | METRICS section start | `referential integrity constraints` | `METRICSTreferential integrity constraints` |
| 72 | 0x348a | 0x347e | 0x3616 | 0x360c | line | 10 | METRICE | `referential integrity constraints` | `METRICET 3` |
| 73 | 0x3495 | 0x3489 | 0x3621 | 0x3617 | line | 16 | METRICS section start | `triggers` | `METRICSTtriggers` |
| 74 | 0x34a6 | 0x349a | 0x3632 | 0x3628 | line | 10 | METRICE | `triggers` | `METRICET 3` |
| 75 | 0x34b1 | 0x34a5 | 0x363d | 0x3633 | line | 49 | METRICS section start | `bitmap, functional and extensible indexes` | `METRICSTbitmap, functional and extensible indexes` |
| 76 | 0x34e3 | 0x34d7 | 0x366f | 0x3665 | line | 10 | METRICE | `bitmap, functional and extensible indexes` | `METRICET 3` |
| 77 | 0x34ee | 0x34e2 | 0x367a | 0x3670 | line | 26 | METRICS section start | `posttables actions` | `METRICSTposttables actions` |
| 78 | 0x3509 | 0x34fd | 0x3695 | 0x368b | line | 10 | METRICE | `posttables actions` | `METRICET 3` |
| 79 | 0x3514 | 0x3508 | 0x36a0 | 0x3696 | line | 37 | METRICS section start | `Post-inst procedural actions` | `METRICSTPost-inst procedural actions ` |
| 80 | 0x353a | 0x352e | 0x36c6 | 0x36bc | line | 10 | METRICE | `Post-inst procedural actions` | `METRICET 3` |
| 81 | 0x3545 | 0x3539 | 0x36d1 | 0x36c7 | line | 34 | METRICS section start | `Deferred analyze commands` | `METRICSTDeferred analyze commands ` |
| 82 | 0x3568 | 0x355c | 0x36f4 | 0x36ea | line | 15 | TABLE | group TABLE O_CHILD in `Deferred analyze commands` | `TABLE "O_CHILD"` |
| 83 | 0x3578 | 0x356c | 0x3704 | 0x36fa | line | 74 | ANALCOMPUTE | group TABLE O_CHILD in `Deferred analyze commands` | `ANALCOMPUTE IS "O_CHILD" ANALYZE  INDEX "O_CHILD_IX"  ESTIMATE STATIST...` |
| 84 | 0x35c3 | 0x35b7 | 0x374f | 0x3745 | line | 15 | TABLE | group TABLE O_EMPTY in `Deferred analyze commands` | `TABLE "O_EMPTY"` |
| 85 | 0x35d3 | 0x35c7 | 0x375f | 0x3755 | line | 74 | ANALCOMPUTE | group TABLE O_EMPTY in `Deferred analyze commands` | `ANALCOMPUTE IS "O_EMPTY" ANALYZE  INDEX "O_EMPTY_IX"  ESTIMATE STATIST...` |
| 86 | 0x361e | 0x3612 | 0x37aa | 0x37a0 | line | 8 | ENDTABLE | group TABLE O_EMPTY in `Deferred analyze commands` | `ENDTABLE` |
| 87 | 0x3627 | 0x361b | 0x37b3 | 0x37a9 | line | 10 | METRICE | `Deferred analyze commands` | `METRICET 5` |
| 88 | 0x3632 | 0x3626 | 0x37be | 0x37b4 | line | 35 | METRICS section start | `Enable enforce constraints` | `METRICSTEnable enforce constraints ` |
| 89 | 0x3656 | 0x364a | 0x37e2 | 0x37d8 | line | 15 | TABLE | group TABLE O_CHILD in `Enable enforce constraints` | `TABLE "O_CHILD"` |
| 90 | 0x3666 | 0x365a | 0x37f2 | 0x37e8 | line | 52 | ENABLE CONSTRAINT | group TABLE O_CHILD in `Enable enforce constraints` | `ALTER TABLE "O_CHILD" ENABLE CONSTRAINT "O_CHILD_CK"` |
| 91 | 0x369b | 0x368f | 0x3827 | 0x381d | line | 53 | ENABLE CONSTRAINT | group TABLE O_CHILD in `Enable enforce constraints` | `ALTER TABLE "O_CHILD" ENABLE CONSTRAINT "O_CHILD_CK2"` |
| 92 | 0x36d1 | 0x36c5 | 0x385d | 0x3853 | line | 52 | ENABLE CONSTRAINT | group TABLE O_CHILD in `Enable enforce constraints` | `ALTER TABLE "O_CHILD" ENABLE CONSTRAINT "O_CHILD_FK"` |
| 93 | 0x3706 | 0x36fa | 0x3892 | 0x3888 | line | 8 | ENDTABLE | group TABLE O_CHILD in `Enable enforce constraints` | `ENDTABLE` |
| 94 | 0x370f | 0x3703 | 0x389b | 0x3891 | line | 10 | METRICE | `Enable enforce constraints` | `METRICET 8` |
| 95 | 0x371a | 0x370e | 0x38a6 | 0x389c | line | 10 | METRICE | end of the data | `METRICETG0` |
| 96 | 0x3725 | 0x3719 | 0x38b1 | 0x38a7 | line | 4 | EXIT | end of the data | `EXIT` |
| 97 | 0x372a | 0x371e | 0x38b6 | 0x38ac | line | 4 | EXIT | end of the data | `EXIT` |

### 4.6 Optimizer statistics items: `ANALSTATS` and `ANALCOMPUTE` (session 5, R-016)

Fixtures: the 20 files of "Fixtures added in session 5" (`stat_exp_*` on 18c and 11g, database
AL32UTF8; `ru_stat_exp_*` on 11g, database CL8MSWIN1251), and for comparison the older files with
index statistics (`ddl`, `own`, `ru_tbl_*`). Truth: `truth_stat.txt` of each folder (lines `TAB|`,
`COL|`, `HIST|`, `IND|`). Tools: `tools/stat_items.py` (item list, decoding, comparison with the
truth), `tools/stat_cmp.py` (two files item by item), `tools/intro_list.py`.
Short names in this section: `stat18` = `stat_exp_18c.dmp`, `stat11` = `stat_exp_11g.dmp`,
`ru_us7` = `ru_stat_exp_us7ascii_est_11g.dmp`, `ru_cp` = `ru_stat_exp_cp1251_est_11g.dmp`,
`ru_utf` = `ru_stat_exp_al32utf8_est_11g.dmp`.

**Result in one paragraph.** A statistics item is a line `ANALSTATS <2 letters> "<table>"...`
followed by **exactly one field**, or a line `ANALCOMPUTE <2 letters> "<table>" ANALYZE ...`
without a field. The field is ASCII text: a PL/SQL block that calls `DBMS_STATS.SET_TABLE_STATS`,
`SET_COLUMN_STATS` or `SET_INDEX_STATS`. The table and the index block are a plain field; the
column block is a field in pieces after `fd ff`. A table with statistics gets one table item and
one item per column **that has statistics** (session 6: possibly fewer than its columns, possibly
none), after its rows; no item holds table data. A reader of table data skips them
with the framing of 4.2 and needs none of their contents.

#### Item forms

| item | layout | where it stands | evidence | status |
|---|---|---|---|---|
| table statistics | line `ANALSTATS T<x> "<table>"`, then a **plain field** (u16 LE length, text, LF): `BEGIN  DBMS_STATS.SET_TABLE_STATS(NULL,'"<table>"',NULL,NULL,NULL,<4 numbers>); END;` | in the table section, after the rows, after the index and constraint items and after the `GRANT` lines (session 6: `stc_exp_18c.dmp`@0x27ca `GRANT`, @0x27ed table item), before the comments | `stat18`@0x2925 line, @0x293c `52 00` + 82 bytes, LF @0x2990; `stat11`@0x2925, 0x293c; `ru_us7`@0x190a0 line `ANALSTATS TR "R_STAT"`, @0x190b6 `54 00` + 84 bytes | confirmed (50 items in 17 files, 2 versions, 2 databases, both paths; session 6: 16 more in the 4 R-020 files, `stc_exp_18c.dmp`@0x27ed + 0x2803 `51 00`) |
| column statistics | line `ANALSTATS T<x> "<table>"` (the **same text** as the line of the table item: the column is named only inside the field), then a **field in pieces** after `fd ff`: `DECLARE  SREC DBMS_STATS.STATREC; BEGIN ... DBMS_STATS.SET_COLUMN_STATS(...); END;` | directly after the table item of the same table, one per column **that has statistics** (session 6), in the column order of the `INSERT INTO` line; a column without statistics leaves no trace | `stat18`@0x2991 line, @0x29a8 `fd ff`, pieces @0x29aa (138), 0x2a36 (3), 0x2a3b (35), 0x2a60 (27), 0x2a7d (38), 0x2aa5 (27), 0x2ac2 (116), 0x2b38 `00 00`, LF @0x2b3a; `stat11`@0x2991, 0x29a8 (pieces 123, 35, 30, 38, 22, 112, 0); `ru_us7`@0x1940c, 0x19422 (137, 35, 797, 38, 52, 130, 0) | confirmed (238 items in 17 files; session 6: 40 more in the 4 R-020 files) |
| index statistics | line `ANALSTATS I<x> "<table>"`, then a plain field `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"<index>"',NULL,NULL,NULL,<8 numbers>); END;` (session 4, section 14.5) | in the table section, directly after the `CREATE [UNIQUE] INDEX` line of that index | `stat18`@0x26fd `CREATE INDEX "T_STAT1_IX"`, @0x27cd line, @0x27e4 `5c 00` + 92; `ru_us7`@0x18ba9 `CREATE UNIQUE INDEX "R_STAT_PK"`, @0x18c7f, 0x18c95; @0x18cf9 / 0x18dce / 0x18de4; @0x18e4e / 0x18f29 / 0x18f3f | confirmed (86 items in 46 files) |
| statistics of the index of a system-named constraint (new) | line `ANALSTATS C<x> "<table>" ("<column>" ) ` (**with text after the table name and a trailing space**), then a plain field `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"SYS_C<spaces>"',NULL,NULL,NULL,<8 numbers>); END;`. The index name is `SYS_C` filled with spaces to 128 characters (18c) or 30 characters (11g); the real name (`truth_stat.txt` `IND\|SYS_C007675\|T_STAT1`) is not in the file | **in the trailer**, in a group `TABLE "<table>"` ... `ENDTABLE` of the section `METRICSTbitmap, functional and extensible indexes` | `stat18`@0x4e15 `TABLE "T_STAT1"`, @0x4e25 line (31 bytes), @0x4e45 `d3 00` + 211 bytes, LF @0x4f1a, @0x4f1b `ENDTABLE`; `stat11`@0x4e6d, 0x4e8d `71 00` + 113 bytes; `stat_exp_us7ascii_18c.dmp`@0x4e25 `ANALSTATS CR ...` | confirmed as bytes (8 files, 2 versions, one table: T_STAT1 with an unnamed primary key). What decides that an index is written this way and not as `I`: hypothesis (the index belongs to a constraint without a user-given name) |
| deferred analyze, table | line `ANALCOMPUTE T<x> "<table>" ANALYZE  TABLE "<table>"  ESTIMATE STATISTICS ` (trailing space; `COMPUTE STATISTICS ` with `statistics=compute`). **No field** | in the trailer, section `METRICSTDeferred analyze commands `, one `TABLE "<table>"` line per table, one `ENDTABLE` for all | `stat18`@0x5235 section line, @0x5258 `TABLE "T_STAT1"`, @0x5268 line (71 bytes), ..., @0x53b8 `ENDTABLE`, @0x53c1 `METRICET 5`; `stat_exp_compute_18c.dmp`@0x5268 (70 bytes, `COMPUTE`); `ru_us7`@0x2be4f, 0x2be81, 0x2bed6, 0x2bf1c | confirmed (50 lines in 17 files; session 6: 16 more, `stc_exp_18c.dmp`@0x59e8) |
| deferred analyze, index | line `ANALCOMPUTE I<x> "<table>" ANALYZE  INDEX "<index>"  ESTIMATE STATISTICS ` (session 4) | the same trailer section | `ddl`@0x2d10; `ru_tbl_exp_cp1251_11g.dmp`@0x119f6 | confirmed (42 lines in 29 files). Not written for the indexes of a table that has an `ANALCOMPUTE T<x>` line (below) |

All counts: `tools/stat_items.py` on the three folders: 46 files contain statistics items; 474
items; 382 fields; 0 fields whose text does not match one of the three blocks; forms: table 50 x
plain, column 238 x pieces, index 86 x plain (`I`) and 8 x plain (`C`).
Session 6, with the 4 R-020 files: 50 files contain statistics items; 546 items; 438 fields; 0
unknown; table 66 x plain, column 278 x pieces, index 86 + 8 (unchanged).

#### Walk: `stat_exp_<v>.dmp` (table mode, `tables=t_stat1,t_stat2,t_stat3,t_stat4`)

"section X" = inside the table section of X, after its rows. The lengths are payload bytes
(without the length fields); the numbers in brackets are the piece lengths.

| # | `stat_exp_18c` | `stat_exp_direct_18c` | `stat_exp_11g` | line | where | field (first file; payload bytes, pieces) | field length in the other files | content (first file) |
|---|---|---|---|---|---|---|---|---|
| 1 | 0x27cd | 0x2769 | 0x27cd | `ANALSTATS IS "T_STAT1"` | section T_STAT1 | plain @0x27e4, `5c 00` + 92 | 92 / 92 | index `T_STAT1_IX`: `SET_INDEX_STATS(...,50,1,7,1,1,1,0,6)` |
| 2 | 0x2925 | 0x28c1 | 0x2925 | `ANALSTATS TS "T_STAT1"` | section T_STAT1 | plain @0x293c, `52 00` + 82 | 82 / 82 | table: `SET_TABLE_STATS(...,50,5,17,6)` |
| 3 | 0x2991 | 0x292d | 0x2991 | `ANALSTATS TS "T_STAT1"` | section T_STAT1 | `fd ff` @0x29a8, 384 (138, 3, 35, 27, 38, 27, 116, 0) | 384 / 360 | column `ID`: EPC 10, `SET_COLUMN_STATS(...,50,.019802,0,srec,3,6)` |
| 4 | 0x2b3b | 0x2ad7 | 0x2b21 | `ANALSTATS TS "T_STAT1"` | section T_STAT1 | `fd ff` @0x2b52, 582 (123, 35, 258, 38, 19, 109, 0) | 582 / 582 | column `V`: EPC 7, `SET_COLUMN_STATS(...,7,.01,0,srec,3,6)` |
| 5 | 0x2da9 | 0x2d45 | 0x2d8f | `ANALSTATS TS "T_STAT1"` | section T_STAT1 | `fd ff` @0x2dc0, 344 (136, 3, 35, 9, 38, 14, 109, 0) | 344 / 326 | column `N`: EPC 5, `SET_COLUMN_STATS(...,5,.01,0,srec,3,6)` |
| 6 | 0x2f2b | 0x2ec7 | 0x2efd | `ANALSTATS TS "T_STAT1"` | section T_STAT1 | `fd ff` @0x2f42, 437 (143, 35, 79, 38, 27, 115, 0) | 437 / 436 | column `D`: EPC 10, `SET_COLUMN_STATS(...,50,.019802,0,srec,8,6)` |
| 7 | 0x36f8 | 0x3630 | 0x36c9 | `ANALSTATS TS "T_STAT2"` | section T_STAT2 | plain @0x370f, `52 00` + 82 | 82 / 82 | table: `SET_TABLE_STATS(...,50,4,17,6)` |
| 8 | 0x3764 | 0x369c | 0x3735 | `ANALSTATS TS "T_STAT2"` | section T_STAT2 | `fd ff` @0x377b, 314 (123, 35, 4, 38, 3, 111, 0) | 314 / 314 | column `ID`: EPC 2, `SET_COLUMN_STATS(...,50,.02,0,srec,3,6)` |
| 9 | 0x38c6 | 0x37fe | 0x3897 | `ANALSTATS TS "T_STAT2"` | section T_STAT2 | `fd ff` @0x38dd, 394 (123, 35, 73, 38, 3, 122, 0) | 394 / 394 | column `V`: EPC 2, `SET_COLUMN_STATS(...,7,.142857142857143,0,srec,3,6)` |
| 10 | 0x3a78 | 0x39b0 | 0x3a49 | `ANALSTATS TS "T_STAT2"` | section T_STAT2 | `fd ff` @0x3a8f, 308 (121, 35, 3, 38, 3, 108, 0) | 308 / 308 | column `N`: EPC 2, `SET_COLUMN_STATS(...,5,.2,0,srec,3,6)` |
| 11 | 0x3bd4 | 0x3b0c | 0x3ba5 | `ANALSTATS TS "T_STAT2"` | section T_STAT2 | `fd ff` @0x3beb, 344 (143, 35, 15, 38, 3, 110, 0) | 344 / 344 | column `D`: EPC 2, `SET_COLUMN_STATS(...,50,.02,0,srec,8,6)` |
| 12 | 0x4344 | 0x4218 | 0x4315 | `ANALSTATS TS "T_STAT3"` | section T_STAT3 | plain @0x435b, `52 00` + 82 | 82 / 82 | table: `SET_TABLE_STATS(...,50,4,20,0)` |
| 13 | 0x43b0 | 0x4284 | 0x4381 | `ANALSTATS TS "T_STAT3"` | section T_STAT3 | `fd ff` @0x43c7, 314 (123, 35, 4, 38, 3, 111, 0) | 314 / 314 | column `ID`: EPC 2, `SET_COLUMN_STATS(...,50,.02,0,srec,2,0)` |
| 14 | 0x4512 | 0x43e6 | 0x44e3 | `ANALSTATS TS "T_STAT3"` | section T_STAT3 | `fd ff` @0x4529, 394 (123, 35, 73, 38, 3, 122, 0) | 394 / 394 | column `V`: EPC 2, `SET_COLUMN_STATS(...,7,.142857142857143,0,srec,2,0)` |
| 15 | 0x46c4 | 0x4598 | 0x4695 | `ANALSTATS TS "T_STAT3"` | section T_STAT3 | `fd ff` @0x46db, 308 (121, 35, 3, 38, 3, 108, 0) | 308 / 308 | column `N`: EPC 2, `SET_COLUMN_STATS(...,5,.2,0,srec,2,0)` |
| 16 | 0x4820 | 0x46f4 | 0x47f1 | `ANALSTATS TS "T_STAT3"` | section T_STAT3 | `fd ff` @0x4837, 344 (143, 35, 15, 38, 3, 110, 0) | 344 / 344 | column `D`: EPC 2, `SET_COLUMN_STATS(...,50,.02,0,srec,7,0)` |
| 17 | 0x4a89 | 0x495b | 0x4abd | `ANALSTATS TS "T_STAT4"` | section T_STAT4 | plain @0x4aa0, `50 00` + 80 | 80 / 80 | table: `SET_TABLE_STATS(...,0,0,0,6)` |
| 18 | 0x4af3 | 0x49c5 | 0x4b27 | `ANALSTATS TS "T_STAT4"` | section T_STAT4 | `fd ff` @0x4b0a, 300 (119, 35, 38, 108, 0) | 300 / 306 | column `ID`: EPC 0, `SET_COLUMN_STATS(...,0,0,0,srec,0,6)` |
| 19 | 0x4c43 | 0x4b15 | 0x4c81 | `ANALSTATS TS "T_STAT4"` | section T_STAT4 | `fd ff` @0x4c5a, 299 (119, 35, 38, 107, 0) | 299 / 305 | column `V`: EPC 0, `SET_COLUMN_STATS(...,0,0,0,srec,0,6)` |
| 20 | 0x4e25 | 0x4cf7 | 0x4e6d | `ANALSTATS CS "T_STAT1" ("ID" ) ` | trailer 'bitmap, functional and extensible indexes', group "T_STAT1" | plain @0x4e45, `d3 00` + 211 | 211 / 113 | index `SYS_C` + spaces: `SET_INDEX_STATS(...,50,1,50,1,1,1,0,6)` |
| 21 | 0x5268 | 0x513a | 0x524e | `ANALCOMPUTE TS "T_STAT1" ANALYZE  TABLE "T_STAT1"  ESTIMATE STATISTICS ` | trailer 'Deferred analyze commands ', group "T_STAT1" | none (the line is the item) | - | - |
| 22 | 0x52c0 | 0x5192 | 0x52a6 | `ANALCOMPUTE TS "T_STAT2" ANALYZE  TABLE "T_STAT2"  ESTIMATE STATISTICS ` | the same section, group "T_STAT2" | none | - | - |
| 23 | 0x5318 | 0x51ea | 0x52fe | `ANALCOMPUTE TS "T_STAT3" ANALYZE  TABLE "T_STAT3"  ESTIMATE STATISTICS ` | the same section, group "T_STAT3" | none | - | - |
| 24 | 0x5370 | 0x5242 | 0x5356 | `ANALCOMPUTE TS "T_STAT4" ANALYZE  TABLE "T_STAT4"  ESTIMATE STATISTICS ` | the same section, group "T_STAT4" | none | - | - |

Order of the items after the rows of T_STAT1 (`stat18`): LF after the rows @0x26fc; `CREATE INDEX
"T_STAT1_IX"` @0x26fd; `ANALSTATS IS` @0x27cd + field; `ALTER TABLE "T_STAT1" ADD  PRIMARY KEY ...`
@0x2843; `ANALSTATS TS` @0x2925 + table field; four `ANALSTATS TS` + column fields @0x2991 to
0x3107; `TABLE "T_STAT2"` @0x3108. T_STAT2 to T_STAT4 have no index and no constraint: the table
item follows the LF after the rows directly (`stat18`@0x36f7 LF, @0x36f8 `ANALSTATS TS`; T_STAT4,
no rows: `ff ff` @0x4a86, LF @0x4a88, `ANALSTATS TS` @0x4a89).

#### Walk: `ru_stat_exp_*_est_11g.dmp` (11g, CL8MSWIN1251 database, `tables=r_stat,r_text`)

| # | `us7ascii_est` | `cp1251_est` | `al32utf8_est` | `direct_us7ascii_est` | line (first file) | where | field (first file; payload bytes, pieces) | field length in the other files | content (first file) |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 0x18c7f | 0x18c7f | 0x18c7f | 0x194df | `ANALSTATS IR "R_STAT"` | section R_STAT | plain @0x18c95, `61 00` + 97 | 97 / 97 / 97 | index `R_STAT_PK`: `SET_INDEX_STATS(...,2000,4,2000,1,1,13,1,6)` |
| 2 | 0x18dce | 0x18dce | 0x18dce | 0x1962e | `ANALSTATS IR "R_STAT"` | section R_STAT | plain @0x18de4, `67 00` + 103 | 103 / 103 / 103 | index `R_STAT_TXT_IX`: `SET_INDEX_STATS(...,1883,5,150,1,11,1768,1,6)` |
| 3 | 0x18f29 | 0x18f29 | 0x18f29 | 0x19789 | `ANALSTATS IR "R_STAT"` | section R_STAT | plain @0x18f3f, `65 00` + 101 | 101 / 101 / 101 | index `R_STAT_GD_IX`: `SET_INDEX_STATS(...,2000,7,680,1,2,2000,1,6)` |
| 4 | 0x190a0 | 0x190a0 | 0x190a0 | 0x19900 | `ANALSTATS TR "R_STAT"` | section R_STAT | plain @0x190b6, `54 00` + 84 | 84 / 84 / 84 | table: `SET_TABLE_STATS(...,2000,13,40,6)` |
| 5 | 0x1910d | 0x1910d | 0x1910d | 0x1996d | `ANALSTATS TR "R_STAT"` | section R_STAT | `fd ff` @0x19123, 319 (123, 35, 6, 38, 3, 114, 0) | 319 / 319 / 319 | column `ID`: EPC 2, `SET_COLUMN_STATS(...,2000,.0005,0,srec,4,6)` |
| 6 | 0x19273 | 0x19273 | 0x19273 | 0x19ad3 | `ANALSTATS TR "R_STAT"` | section R_STAT | `fd ff` @0x19289, 370 (123, 35, 17, 38, 44, 113, 0) | 370 / 370 / 370 | column `GRP`: EPC 9, `SET_COLUMN_STATS(...,9,.00025,0,srec,3,6)` |
| 7 | 0x1940c | 0x1940c | 0x1940c | 0x19c6c | `ANALSTATS TR "R_STAT"` | section R_STAT | `fd ff` @0x19422, 1189 (137, 35, 797, 38, 52, 130, 0) | 1189 / 1189 / 1189 | column `TXT`: EPC 21, `SET_COLUMN_STATS(...,150,.00668219914380534,117,srec,7,6)` |
| 8 | 0x198d8 | 0x198d8 | 0x198d8 | 0x1a138 | `ANALSTATS TR "R_STAT"` | section R_STAT | `fd ff` @0x198ee, 400 (139, 35, 73, 38, 3, 112, 0) | 400 / 400 / 400 | column `CODE`: EPC 2, `SET_COLUMN_STATS(...,50,.02,0,srec,7,6)` |
| 9 | 0x19a8f | 0x19a8f | 0x19a8f | 0x1a2ef | `ANALSTATS TR "R_STAT"` | section R_STAT | `fd ff` @0x19aa5, 438 (143, 35, 87, 38, 22, 113, 0) | 438 / 438 / 438 | column `DT`: EPC 11, `SET_COLUMN_STATS(...,200,.005,0,srec,8,6)` |
| 10 | 0x19c6c | 0x19c6c | 0x19c6c | 0x1a4cc | `ANALSTATS TR "R_STAT"` | section R_STAT | `fd ff` @0x19c82, 372 (125, 35, 33, 38, 11, 130, 0) | 372 / 372 / 372 | column `AMT`: EPC 6, `SET_COLUMN_STATS(...,1000,.00104480196665761,181,srec,4,6)` |
| 11 | 0x19e07 | 0x19e07 | 0x19e07 | 0x1a667 | `ANALSTATS TR "R_STAT"` | section R_STAT | `fd ff` @0x19e1d, 424 (143, 35, 73, 38, 3, 132, 0) | 424 / 424 / 424 | column `NOTE`: EPC 2, `SET_COLUMN_STATS(...,1715,.000583090379008746,285,srec,9,6)` |
| 12 | 0x2ae4a | 0x2ae4a | 0x2ae4a | 0x2b440 | `ANALSTATS IR "R_TEXT"` | section R_TEXT | plain @0x2ae60, `5f 00` + 95 | 95 / 95 / 95 | index `R_TEXT_IX`: `SET_INDEX_STATS(...,309,1,309,1,1,10,0,6)` |
| 13 | 0x2af41 | 0x2af41 | 0x2af44 | 0x2b537 | `ANALSTATS TR "R_TEXT"` | section R_TEXT | plain @0x2af57, `54 00` + 84 | 84 / 84 / 84 | table: `SET_TABLE_STATS(...,309,13,213,6)` |
| 14 | 0x2afae | 0x2afae | 0x2afb1 | 0x2b5a4 | `ANALSTATS TR "R_TEXT"` | section R_TEXT | `fd ff` @0x2afc4, 332 (125, 35, 5, 38, 3, 126, 0) | 332 / 332 / 332 | column `ID`: EPC 2, `SET_COLUMN_STATS(...,309,.00323624595469256,0,srec,4,6)` |
| 15 | 0x2b121 | 0x2b121 | 0x2b124 | 0x2b717 | `ANALSTATS TR "R_TEXT"` | section R_TEXT | `fd ff` @0x2b137, 476 (199, 35, 74, 38, 3, 127, 0) | 476 / 476 / 476 | column `V`: EPC 2, `SET_COLUMN_STATS(...,265,.00377358490566038,44,srec,66,6)` |
| 16 | 0x2b324 | 0x2b324 | 0x2b327 | 0x2b91a | `ANALSTATS TR "R_TEXT"` | section R_TEXT | `fd ff` @0x2b33a, 437 (163, 35, 74, 38, 3, 124, 0) | 437 / 437 / 437 | column `C`: EPC 2, `SET_COLUMN_STATS(...,70,.0142857142857143,1,srec,13,6)` |
| 17 | 0x2b500 | 0x2b500 | 0x2b503 | 0x2baf6 | `ANALSTATS TR "R_TEXT"` | section R_TEXT | `fd ff` @0x2b516, 432 (155, 35, 74, 38, 3, 127, 0) | 432 / 432 / 432 | column `VC`: EPC 2, `SET_COLUMN_STATS(...,308,.00324675324675325,1,srec,12,6)` |
| 18 | 0x2b6d7 | 0x2b6d7 | 0x2b6da | 0x2bccd | `ANALSTATS TR "R_TEXT"` | section R_TEXT | `fd ff` @0x2b6ed, 449 (175, 35, 71, 38, 3, 127, 0) | 449 / 449 / 449 | column `N`: EPC 2, `SET_COLUMN_STATS(...,280,.00357142857142857,29,srec,38,6)` |
| 19 | 0x2b8bf | 0x2b8bf | 0x2b8c2 | 0x2beb5 | `ANALSTATS TR "R_TEXT"` | section R_TEXT | `fd ff` @0x2b8d5, 411 (135, 35, 74, 38, 3, 126, 0) | 411 / 411 / 411 | column `LV`: EPC 2, `SET_COLUMN_STATS(...,31,.032258064516129,278,srec,74,6)` |
| 20 | 0x2ba81 | 0x2ba81 | 0x2ba84 | 0x2c077 | `ANALSTATS TR "R_TEXT"` | section R_TEXT | `fd ff` @0x2ba97, 359 (143, 35, 15, 38, 3, 125, 0) | 359 / 359 / 359 | column `D`: EPC 2, `SET_COLUMN_STATS(...,308,.00324675324675325,1,srec,8,6)` |
| 21 | 0x2be81 | 0x2be81 | 0x2be98 | 0x2c477 | `ANALCOMPUTE TR "R_STAT" ANALYZE  TABLE "R_STAT"  ESTIMATE STATISTICS ` | trailer 'Deferred analyze commands ', group "R_STAT" | none | - | - |
| 22 | 0x2bed6 | 0x2bed6 | 0x2beed | 0x2c4cc | `ANALCOMPUTE TR "R_TEXT" ANALYZE  TABLE "R_TEXT"  ESTIMATE STATISTICS ` | the same section, group "R_TEXT" | none | - | - |

In the `cp1251_est` file the letters are `IS` / `TS` (every line of the table above with `S`
instead of `R`). The offsets of `al32utf8_est` are 3 bytes later from item 13 on because the CHECK
condition before it (`CALTER TABLE` field @0x2aecf) is 3 bytes longer in UTF-8 (100 against 97
payload bytes).

Order of the items after the rows of R_TEXT (`ru_us7`): `CREATE INDEX "R_TEXT_IX"` @0x2ad7b;
`ANALSTATS IR` @0x2ae4a + field; `CALTER TABLE` @0x2aec2 + field in pieces @0x2aecf; `ANALSTATS TR`
@0x2af41 + table field; seven `ANALSTATS TR` + column fields @0x2afae to 0x2bc0e; `COMMENT ON TABLE
"R_TEXT" IS ` @0x2bc0f + field; `COMMENT ON COLUMN "R_TEXT"."V" IS ` @0x2bc38 + field; `METRICST...`
@0x2bc71. So the order in a table section is: rows; per index `CREATE INDEX` and its `ANALSTATS I`;
constraints; table statistics; column statistics; comments. Status: observed (6 tables).
Session 6: a `GRANT` line stands **before** the table item, directly after the LF that ends the
rows (`stc_exp_18c.dmp`@0x27c9 LF, @0x27ca `GRANT SELECT ON "T_STC1" TO PUBLIC`, @0x27ed
`ANALSTATS TS "T_STC1"`, @0x2b67 `COMMENT ON TABLE`; the same in the other three R-020 files).
`GRANT` also stands before the constraint lines (4.5, `own`@0x3310). No table of the fixtures has
an index, a grant, a constraint **and** statistics together, so the full order "rows; indexes with
their `I?` items; grants; constraints; table item; column items; comments" is put together from
three kinds of tables: observed, and a reader does not rely on it (rule S3).

#### Which items a table gets (question B2)

| table | how its statistics were made (`09_stat.sql`, `08_ru_stats_objects.sql`; `truth_stat.txt`) | columns | table items | column items | `ANALCOMPUTE T?` lines | index items |
|---|---|---|---|---|---|---|
| T_STAT1 | `DBMS_STATS`, histogram on every column (`COLSTAT\|T_STAT1\|...` FREQUENCY, HYBRID / HEIGHT BALANCED) | 4 | 1 | 4 | 1 | 1 x `I` (T_STAT1_IX), 1 x `C` (unnamed primary key) |
| T_STAT2 | `DBMS_STATS`, no histograms (`COLSTAT\|T_STAT2\|...\|1\|NONE`) | 4 | 1 | 4 | 1 | - |
| T_STAT3 | `ANALYZE TABLE ... COMPUTE STATISTICS` | 4 | 1 | 4 | 1 | - |
| T_STAT4 | `DBMS_STATS`, table without rows (`TAB\|T_STAT4\|0\|0`) | 2 | 1 | 2 | 1 | - |
| R_STAT | `DBMS_STATS`, histograms on 4 of 7 columns | 7 | 1 | 7 | 1 | 3 x `I` |
| R_TEXT (R-016 files) | `DBMS_STATS`, no histograms | 7 | 1 | 7 | 1 | 1 x `I` |
| R_TEXT (R-015 files, exported before the statistics were gathered) | none on the table; the index has statistics | 7 | 0 | 0 | 0 (one `ANALCOMPUTE I?` line instead) | 1 x `I` |
| every table of sessions 1 to 4 (T_FRAME, T_DDL, O_CHILD, ...) | none known | - | 0 | 0 | 0 | 0 or 1 x `I` |

The same counts in every one of the 17 files that has `T` items (4 per version in `18c/` and
`11g/`, 9 in `11g-cp1251/`); the files made with `statistics=none` have no statistics item at all.

| reading | test | result | status |
|---|---|---|---|
| "One item per table plus one per column" | 50 table items in 17 files; each is followed by exactly as many column items as the table has columns (u16 column count of the table: `stat18`@0x226c `04 00`, @0x4a74 `02 00`; `ru_us7`@0x0ae7 `07 00`), in the column order of the `INSERT INTO` line (`tools/stat_items.py`: `col_order_ok` 50, `col_order_bad` 0; 238 = 8 x 14 + 9 x 14 column items) | **holds** for all 6 tables, whatever made the statistics: with or without histograms, `ANALYZE`, a table without rows | confirmed for these 6 tables, in which every column has statistics. **Session 6: weakened as a rule.** The count is one per column that has statistics: 2 of 5 (T_STC1), 4 of 6 (T_STC2), 4 of 5 (T_STC3), 0 of 5 with a table item (T_STC5); see "Columns without statistics" below |
| "`ANALCOMPUTE` = the table was analysed with the old `ANALYZE`" | T_STAT3 is the only table analysed with `ANALYZE`; all four `T_STAT` tables and both `R_` tables have an `ANALCOMPUTE T?` line | **refuted**: `stat18`@0x5268, 0x52c0, 0x5318, 0x5370 (one line per table) | refuted |
| What T_STAT3 (`ANALYZE`) has that the others have not | the last argument of its table block and of its four column blocks is `0`; it is `6` in every block of the five `DBMS_STATS` tables, on both versions | `stat18`@0x435b `...,50,4,20,0); END;`, @0x43c7 `...,srec,2,0); END;`; `stat11`@0x432c, 0x4398; against `stat18`@0x293c `...,50,5,17,6)` | observed (one table, 2 versions, 8 files): 16 column blocks and 4 table blocks with `0` per version, 40 and 12 with `6`. Meaning of the argument: unknown (hypothesis: flags that say how the statistics were made) |
| What decides that a table has `ANALSTATS T` items | the table has optimizer statistics in the data dictionary at export time, and the export is not `statistics=none` | with statistics: 6 tables above. Without: R_TEXT in `ru_tbl_exp_us7ascii_11g.dmp` (no `T` line; `ANALSTATS IR` @0x11781 only), the same table with statistics in `ru_us7`@0x2af41 | observed. That the older tables had **no** statistics at export time is the fixture author's statement (`requests_r016.md`: "R_TEXT dumps of R-015 have no statistics"), not a row of a truth file; `truth_stat.txt` of `11g-cp1251/` has `TAB\|R_CLOB\|NULL\|...` (no statistics) but R_CLOB is not in the R-016 dumps |
| What decides that a table has an `ANALCOMPUTE T?` line | the same condition: in all 17 files a table has the line exactly when it has `ANALSTATS T` items | 50 lines / 50 table items | confirmed for the fixtures |
| `ANALCOMPUTE I?` (session 4) | written for an index with statistics when the table has **no** `ANALCOMPUTE T?` line; never together with it | `ddl`@0x2d10 (T_DDL: no table statistics); `stat18`: no `ANALCOMPUTE I` although T_STAT1_IX has an `ANALSTATS IS` item (@0x27cd); `ru_us7`: 4 `ANALSTATS IR`, no `ANALCOMPUTE IR` | observed (29 files with `ANALCOMPUTE I?`, 17 with `ANALCOMPUTE T?`, none with both for one table) |

#### What is inside the field (question B3)

The field is **text, a PL/SQL block**, in every one of the 382 fields. First 60 bytes of a table
field (`stat18`@0x293e): `BEGIN  DBMS_STATS.SET_TABLE_STATS(NULL,'"T_STAT1"',NULL,NULL`. First
60 bytes of a column field (`stat18`@0x29ac, after `fd ff` and the first piece length `8a 00`):
`DECLARE  SREC DBMS_STATS.STATREC; BEGIN SREC.MINVAL := 'C102`. There is no binary encoding: every
number is decimal text (no leading zero before the point: `.019802`), every raw value is
upper-case hexadecimal text in single quotes. Byte offsets inside the field are therefore fixed
only up to the first variable part; the tables below give the parts in order.

Table block (plain field). Example `stat18`@0x293c, 82 bytes:
`BEGIN  DBMS_STATS.SET_TABLE_STATS(NULL,'"T_STAT1"',NULL,NULL,NULL,50,5,17,6); END;`

| offset in field | length | meaning | evidence | status |
|---|---|---|---|---|
| +0 | 41 | constant text `BEGIN  DBMS_STATS.SET_TABLE_STATS(NULL,'"` (two spaces after `BEGIN`) | `stat18`@0x293e; `ru_us7`@0x190b8 | confirmed (50 fields) |
| +41 | var | table name, then `"',NULL,NULL,NULL,` | `stat18`@0x2967 `T_STAT1`; `ru_us7`@0x190e1 `R_STAT` | confirmed |
| then | var | number 1: `num_rows` | `stat18`@0x293c `50` = `TAB\|T_STAT1\|50`; `ru_us7`@0x190b6 `2000` = `TAB\|R_STAT\|2000`; T_STAT4 `0` (`stat18`@0x4aa0) | confirmed (50 of 50 equal the truth) |
| then | var | number 2: `blocks` | `5` = `TAB\|T_STAT1\|50\|5`; `13` = `TAB\|R_STAT\|2000\|13` | confirmed (50 of 50) |
| then | var | number 3: `avg_row_len` | `17` (T_STAT1), `20` (T_STAT3), `40` (R_STAT), `213` (R_TEXT), column 8 of the `TAB\|` line | confirmed (50 of 50) |
| then | 1 | number 4: `6` or `0` | `6` for the `DBMS_STATS` tables, `0` for T_STAT3 (`stat18`@0x435b) | observed; meaning unknown |
| end | 7 | `); END;` | all | confirmed |

Not in the block: `empty_blocks`, `avg_space`, `chain_cnt`, `sample_size`, `last_analyzed` (for
T_STAT3 the truth has `empty_blocks` 4 and `avg_space` 6956; neither `6956` nor a date occurs in
`stat18`).

Index block (plain field; `I` and `C` items). Example `ru_us7`@0x18c95, 97 bytes:
`BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"R_STAT_PK"',NULL,NULL,NULL,2000,4,2000,1,1,13,1,6); END;`

| position | meaning | evidence | status |
|---|---|---|---|
| numbers 1 to 7 | `num_rows`, `leaf_blocks`, `distinct_keys`, `avg_leaf_blocks_per_key`, `avg_data_blocks_per_key`, `clustering_factor`, `blevel` | `2000,4,2000,1,1,13,1` = `IND\|R_STAT_PK\|R_STAT\|UNIQUE\|1\|4\|2000\|1\|1\|13\|2000` (the truth line has blevel first, num_rows last but one); `ru_us7`@0x18de4 `1883,5,150,1,11,1768,1` = `IND\|R_STAT_TXT_IX\|...\|1\|5\|150\|1\|11\|1768\|1883`; `stat18`@0x27e4 `50,1,7,1,1,1,0` = `IND\|T_STAT1_IX\|...\|0\|1\|7\|1\|1\|1\|50` | confirmed (all index items of the 17 files: 7 numbers each, all equal the truth; the three indexes of R_STAT separate the positions) |
| number 8 | `6` or `0` | `6` in all R-016 files; `0` in the 11g files of session 4, `6` in the 18c ones (13.5, 13.9) | observed; meaning unknown |
| index name | in the client set (14.5); `SYS_C` + spaces for the `C` item | `stat18`@0x4e45 | confirmed |

Column block (field in pieces). Example `stat18`@0x2b52 (column V of T_STAT1), 582 payload bytes:
`DECLARE  SREC DBMS_STATS.STATREC; BEGIN SREC.MINVAL := '7630'; SREC.MAXVAL := '7636'; SREC.EAVS := 0; SREC.CHVALS := NULL; SREC.NOVALS := DBMS_STATS.NUMARRAY(613664584968085000000000000000000000,...); SREC.BKVALS := DBMS_STATS.NUMARRAY(7,15,22,29,36,43,50); SREC.EPC := 7; DBMS_STATS.SET_COLUMN_STATS(NULL,'"T_STAT1"','"V"', NULL ,NULL,NULL,7,.01,0,srec,3,6); END;`

| piece | length | text | evidence | status |
|---|---|---|---|---|
| 1 | var (119 to 199 seen) | `DECLARE  SREC DBMS_STATS.STATREC; BEGIN SREC.MINVAL := <min>; SREC.MAXVAL := <max>; SREC.EAVS := <n>; SREC.CHVALS := NULL; ` (ends with a space) | `stat18`@0x2b54 (123 bytes); `ru_us7`@0x19424 (137) | confirmed (230 of 238 fields) |
| 1, other form | var | the same up to `SREC.CHVALS := DBMS_STATS.CHARARRAY(`, and then **one more piece** `); ` (3 bytes) | `stat18`@0x29aa (138 bytes, `... SREC.EAVS := 4; SREC.CHVALS := DBMS_STATS.CHARARRAY(`), @0x2a36 `03 00 29 3b 20`; also @0x2dc2 / 0x2e4c (column N) | observed: 8 fields, 18c only, columns ID and N of T_STAT1. The list between `(` and `)` is **empty** in all 8; a list with elements is not in the fixtures |
| 2 | 35 | `SREC.NOVALS := DBMS_STATS.NUMARRAY(` | `stat18`@0x2bd1 | confirmed (238) |
| 3 | var (3 to 797 seen) | the endpoint values, decimal numbers separated by commas. **Absent** when the list is empty | `stat18`@0x2bf6 (258 bytes, 7 numbers of 36 digits); `ru_us7`@0x194d4 (797 bytes, 21 numbers of 37 digits); `ru_us7`@0x19d28 `0,28.43,57.14,85.71,114.29,142.71`; absent: `stat18`@0x4b0a (pieces 119, 35, 38, 108, 0) | confirmed |
| 4 | 38 | `); SREC.BKVALS := DBMS_STATS.NUMARRAY(` | `stat18`@0x2cfa | confirmed (238) |
| 5 | var (3 to 52 seen) | the endpoint numbers, separated by commas. Absent when the list is empty | `stat18`@0x2d22 `7,15,22,29,36,43,50` | confirmed |
| 6 | var (107 to 132 seen) | `); SREC.EPC := <n>; DBMS_STATS.SET_COLUMN_STATS(NULL,'"<table>"','"<column>"', NULL ,NULL,NULL,<distcnt>,<density>,<nullcnt>,srec,<avgclen>,<flags>); END;` (note ` NULL ` with spaces) | `stat18`@0x2d37 (109 bytes) | confirmed (238) |
| end | 0 | piece of length 0, then LF | `stat18`@0x2da6 `00 00`, @0x2da8 `0a` | confirmed |

Number of pieces including the ending one: 7 in 222 fields, 5 in 8 fields (18c, T_STAT4: both
lists empty), 8 in 8 fields (18c, with `CHARARRAY(`). The pieces are the parts the statement is
built from, as for `CALTER TABLE` (4.5); their lengths are not constant.

| part of the text | meaning (`truth_stat.txt`) | evidence | status |
|---|---|---|---|
| `SREC.MINVAL`, `SREC.MAXVAL` | `NULL`, or the `low_value` / `high_value` of the column as hexadecimal text: the internal bytes of the value (NUMBER, DATE, text in the database character set, NVARCHAR2 in UTF-16BE) | `stat18`@0x29aa `'C102'` / `'C133'` = `COL\|T_STAT1\|ID\|...\|C102\|C133` (1 and 50 by the rule of 6.1); @0x2f44 `'78780102010101'` / `'78780214010101'` (DATE 2020-01-02 and 2020-02-20, rule of 6.2); `ru_us7`@0x19424 `'C0C1C2'` / `'F1F2F3F4F5F6F7F8'` = `COL\|R_STAT\|TXT\|...\|C0C1C2\|F1F2F3F4F5F6F7F8`; `ru_us7`@0x2b6ef `'0070006C00610069006E'` (NVARCHAR2 `plain`); `NULL`: `stat18`@0x4b0c | confirmed (476 of 476 equal the truth) |
| `SREC.EAVS` | `0`, or `4` in the 8 fields with `CHARARRAY(` | `stat18`@0x29aa | unknown. Observed: `4` exactly for the two columns whose `HIST\|` rows have an endpoint actual value (`HIST\|T_STAT1\|ID\|1\|1\|Typ=1 Len=1: 31`), 18c only |
| `SREC.CHVALS` | `NULL` or an empty `DBMS_STATS.CHARARRAY()` | above | unknown (hypothesis: endpoint actual values; if so the fixtures show that 18c `exp` writes the list empty) |
| `SREC.NOVALS` | the `endpoint_value` of every histogram row, in order | `stat18`@0x2a60 `1,6,12,17,23,28,34,39,44,50` = the ten `HIST\|T_STAT1\|ID\|` rows; `ru_us7`@0x194d4 = the 21 `HIST\|R_STAT\|TXT\|` rows | confirmed (238 lists; compared to 15 significant digits, the precision the truth prints) |
| `SREC.BKVALS` | the `endpoint_number` of every histogram row | `stat18`@0x2d22; `ru_us7`@0x19368 `1000,1200,1314,1428,1543,1658,1772,1886,2000` = `HIST\|R_STAT\|GRP\|` | confirmed (238 lists) |
| `SREC.EPC` | the number of histogram rows (= elements of both lists) | 10, 7, 5, 2, 21, 11, 9, 6 as in the truth | confirmed, with one exception: T_STAT4 on 11g has `EPC := 2`, `NUMARRAY(0,0)`, `NUMARRAY(0,1)` (`stat11`@0x4b3e) while the truth has no `HIST\|T_STAT4` row; 18c writes `EPC := 0` and empty lists (`stat18`@0x4b0a). 8 of 3116 comparisons differ, all here |
| `<distcnt>` | `num_distinct` | `50`, `7`, `5`; `150` (R_STAT.TXT) | confirmed (238 of 238) |
| `<density>` | `density`, the same digits as the truth prints | `.019802`, `.142857142857143`, `.00668219914380534` | confirmed (238 of 238) |
| `<nullcnt>` | `num_nulls` | `117` (R_STAT.TXT), `181` (AMT), `285` (NOTE), `278` (R_TEXT.LV) | confirmed (238 of 238) |
| `<avgclen>` | `avg_col_len` | `3`, `8`; `2`, `7` for T_STAT3; `66` (R_TEXT.V) | confirmed (238 of 238) |
| `<flags>` | `6` or `0` | as in the table block | observed; meaning unknown |

Not in the block: `num_buckets`, the histogram type (`FREQUENCY`, `HEIGHT BALANCED`, `HYBRID`: no
such word in the files), `sample_size`, `last_analyzed`. A text value is never written as text:
the low and high value are hexadecimal, and the histogram endpoints of a text column are the
numbers the dictionary holds (`endpoint_value`).

Comparison with the truth (`tools/stat_items.py`, the 17 files with `T` items): 3108 values equal,
8 differ (the 11g `EPC` of T_STAT4 above, 2 columns x 4 files).

#### `statistics=none`, `compute`, and the direct path (questions B4, C3)

`tools/stat_cmp.py`, item by item.

| comparison | result | evidence | status |
|---|---|---|---|
| `stat_exp_<v>` against `stat_exp_none_<v>` | all 20 `ANALSTATS` lines and their 20 fields are gone (`I`, `T` and `C`), and the whole section `METRICSTDeferred analyze commands ` (section line, 4 `TABLE` lines, 4 `ANALCOMPUTE` lines, `ENDTABLE`, `METRICET 5`): 111 items against 60, 51 only in the first file, the other 60 byte-identical. The group `TABLE "T_STAT1"` / `ENDTABLE` under "bitmap, functional and extensible indexes" **stays**, empty | `stat_exp_none_18c.dmp`@0x26fd `CREATE INDEX`, @0x27cd `ALTER TABLE ... PRIMARY KEY` (no `ANALSTATS` between), @0x35fb `TABLE "T_STAT1"`, @0x360b `ENDTABLE`, last section `Post-inst procedural actions ` @0x38f4, `METRICETG0` @0x3925; 11g: `stat_exp_none_11g.dmp`, the same 51 items missing | confirmed (2 versions) |
| `ru_us7` against `ru_stat_exp_us7ascii_none_11g.dmp` | the same: 93 items against 46; 20 `ANALSTATS` lines + 20 fields and the 7 items of "Deferred analyze commands " are gone; one item differs (`METRICET 312` / `310`) | `ru_stat_exp_us7ascii_none_11g.dmp`@0x2a0ca | confirmed |
| binary part of every table (column count to the LF after the rows) with and without statistics | identical | T_STAT1: `stat18`@0x226c-0x26fc = `stat_exp_none_18c.dmp`@0x226c-0x26fc (1168 bytes); T_STAT2 @0x3267 = @0x2a0e; R_STAT: `ru_us7`@0x0ae7-0x18ba8 = none file @0x0ae7-0x18ba8 (98497 bytes); R_TEXT @0x1a1a3 = @0x190f5 (68567 bytes) | confirmed |
| `stat_exp_<v>` against `stat_exp_compute_<v>` (`statistics=compute`) | same items, same order; **only the four `ANALCOMPUTE` lines differ**: `COMPUTE STATISTICS ` instead of `ESTIMATE STATISTICS ` (one byte shorter). Every `ANALSTATS` field is byte-identical | `stat18`@0x5268 (71 bytes) / `stat_exp_compute_18c.dmp`@0x5268 (70 bytes); 107 of 111 items identical, 4 differ; 11g the same (@0x524e) | confirmed (2 versions) |
| `ru_*_est_*` against `ru_*_cmp_*` (question C3), three pairs: cp1251 client, US7ASCII client, direct path with cp1251 client | the same: 91 of 93 items identical, the two `ANALCOMPUTE` lines differ; statistics fields identical | `ru_cp`@0x2be81 (69 bytes) / `ru_stat_exp_cp1251_cmp_11g.dmp`@0x2be81 (68 bytes); `ru_stat_exp_direct_cp1251_est_11g.dmp`@0x2c477 / `..._cmp_...`@0x2c477 | confirmed |
| `stat_exp_<v>` against `stat_exp_direct_<v>`; `ru_cp` against `ru_stat_exp_direct_cp1251_est_11g.dmp`; `ru_stat_exp_us7ascii_cmp` against `ru_stat_exp_direct_us7ascii_cmp` (question C3) | **every item is byte-identical** (111 of 111; 93 of 93); only the binary parts of the tables differ (no `00 00` after the rows; 18c: no `ff ff` for T_STAT4, 13.2) and the offsets | `stat18`@0x29a8 = `stat_exp_direct_18c.dmp`@0x2944 (the field of item 3) | confirmed (2 versions, 2 databases) |

So the option `statistics=` decides whether the items exist (`none`: no item) and which word the
`ANALCOMPUTE` line carries; it does not change the `ANALSTATS` fields. The export path changes
nothing in them.

#### The second letter and the log (questions B5, C1)

| statement | evidence | status |
|---|---|---|
| With a US7ASCII client the second letter is `R` in **every** statistics line of the file (`IR`, `TR`, `CR`, and `ANALCOMPUTE TR`); with the default client it is `S` in every line | `stat_exp_us7ascii_18c.dmp`@0x27cd `ANALSTATS IR`, @0x2925 `ANALSTATS TR`, @0x4e25 `ANALSTATS CR`, @0x5268 `ANALCOMPUTE TR`: 24 lines; `stat18`: 24 lines with `S` | confirmed |
| It is the **only** difference after the first record, apart from the client ids of block 2: `cmp` of `stat18` and `stat_exp_us7ascii_18c.dmp` from offset R on gives 28 differing bytes: 4 at 0x20a2-0x20a5 (`03 69 03 69` / `00 01 00 01`) and 24 times `53` / `52` (`S` / `R`), one per line (first @0x27d8, last @0x537d). 11g: the same 24 items | `tools/stat_cmp.py`: 87 of 111 items identical, 24 differ at byte 11 (`ANALSTATS`) or 13 (`ANALCOMPUTE`) | confirmed (2 versions) |
| The letter follows the log in all 46 files with statistics items: `R` in every line when the log has `EXP-00091`, `S` in every line when it has not; no file mixes `S` and `R` | `tools/stat_items.py`: `letter_fit` 46, `letter_bad` 0. CL8MSWIN1251 database (question C1): US7ASCII client `R` (4 files), AL32UTF8 client `R` (`ru_utf`@0x18c7f), CL8MSWIN1251 client `S` (4 files) | confirmed for the fixtures |
| The number of `EXP-00091` lines is **not** the number of lines with `R`: 6 against 24 (`stat_exp_us7ascii_<v>`), 6 against 22 (`ru_us7`) | `stat_exp_us7ascii_18c.log` (6 lines), `ru_stat_exp_us7ascii_est_11g.log` (6 lines); `tools/stat_items.py`: the two numbers are equal only in the 26 files where both are 0 | confirmed |
| It **is** the number of `ANALSTATS ?R` lines that are followed by a **plain** field, in all 46 files: one message per table with statistics (table item; its column items give none) and one per index item (`I` or `C`) | `stat_exp_us7ascii_18c.dmp`: 4 table items + 1 `IR` + 1 `CR` = 6 = log; log order: 2 messages after T_STAT1, 1 after T_STAT2, 1 after T_STAT3, 2 after T_STAT4 (the second is written at the end, where the `CR` item stands). `ru_us7`: 2 table items + 4 `IR` = 6 = log (4 after R_STAT, 2 after R_TEXT). `tools/stat_items.py`: `exp91_eq_R_plain_fields` 46 of 46 | confirmed for the fixtures |
| As in session 4 (14.5), "`R`" coincides in every file with "client set differs from the database set" (header bytes 1-2 against csB) | 46 of 46 | the two conditions still cannot be separated |

The first letter: `T` = table and column statistics, `I` = index, `C` = index of a system-named
constraint. Status: confirmed as a description of the 474 lines (each letter always with the same
block: `T` with `SET_TABLE_STATS` / `SET_COLUMN_STATS`, `I` and `C` with `SET_INDEX_STATS`); other
letters are not in the fixtures.

#### Columns without statistics (session 6, R-020)

Fixtures: `stc_exp_<v>.dmp` and `stc_exp_direct_<v>.dmp` (18c and 11g; "Fixtures added in session
6"). Truth: `truth_stc.txt`. Tools: `tools/stc_items.py` (everything in this subsection; it prints
`problems: 0` on the four files), `tools/stat_items.py`, `tools/stat_cmp.py`, `tools/exp_items.py`.
Short names: `stc18` = `stc_exp_18c.dmp`, `stc18d` = `stc_exp_direct_18c.dmp`, `stc11` =
`stc_exp_11g.dmp`, `stc11d` = `stc_exp_direct_11g.dmp`.

**Result in one paragraph.** A column has a statistics item exactly when it has statistics in the
data dictionary at the time `exp` runs. The column items of a table stand in ascending column
order; a column without statistics is simply not there: no empty item, no placeholder, no count.
A table with table statistics and no column statistics has the table item alone. A table without
statistics has no statistics item and no `ANALCOMPUTE` line. The file does not say **why** a
column has no item.

**Walk (question B1).** All `ANALSTATS` / `ANALCOMPUTE` items of the four files, in file order.
The line is `ANALSTATS TS "<table>"` (21 bytes) for every `ANALSTATS` item; "pos" is the 1-based
position, in the column list of the `INSERT INTO` line, of the column named **inside the field**
(`SET_COLUMN_STATS(NULL,'"T"','"<column>"'`). Lengths are payload bytes; piece lengths in brackets.
The field starts 22 bytes after the line in every row (the offset given is that of the line).

| # | `stc18` | `stc18d` | `stc11` | `stc11d` | item | framing, length (pieces) | content (`stc18`) | truth line |
|---|---|---|---|---|---|---|---|---|
| 1 | 0x27ed | 0x2789 | 0x27ed | 0x2789 | T_STC1 table | plain, `51 00` + 81 | `SET_TABLE_STATS(...,50,5,18,6)` | `TAB\|T_STC1\|50\|2026-10-07` |
| 2 | 0x2857 | 0x27f3 | 0x2857 | 0x27f3 | T_STC1 column ID, pos 1 | `fd ff`, 313 (123, 35, 4, 38, 3, 110, 0) | MINVAL `'C102'`, MAXVAL `'C133'`, `NUMARRAY(1,50)`, `NUMARRAY(0,1)`, EPC 2; `...,50,.02,0,srec,3,6` | `COLUMN\|T_STC1\|1\|ID\|stats:50:0` |
| 3 | 0x29b7 | 0x2953 | 0x29b7 | 0x2953 | T_STC1 column A, pos 2 | `fd ff`, 393 (123, 35, 73, 38, 3, 121, 0) | MINVAL `'6130'`, MAXVAL `'6136'`, EPC 2; `...,7,.142857142857143,0,srec,3,6` | `COLUMN\|T_STC1\|2\|A\|stats:7:0` |
| - | - | - | - | - | T_STC1 columns B, C, ALLNULL (pos 3, 4, 5) | **no item**: after the LF of item 3 (`stc18`@0x2b66) the next item is `COMMENT ON TABLE "T_STC1" IS ` @0x2b67 | - | three lines `NO STATISTICS` |
| 4 | 0x3378 | 0x32b0 | 0x3378 | 0x32b0 | T_STC2 table | plain, 81 | `SET_TABLE_STATS(...,50,4,18,6)` | `TAB\|T_STC2\|50\|...` |
| 5 | 0x33e2 | 0x331a | 0x33e2 | 0x331a | T_STC2 column ID, pos 1 | `fd ff`, 313 (123, 35, 4, 38, 3, 110, 0) | as item 2 with the other table name | `stats:50:0` |
| 6 | 0x3542 | 0x347a | 0x3542 | 0x347a | T_STC2 column A, pos 2 | `fd ff`, 393 (123, 35, 73, 38, 3, 121, 0) | as item 3 | `stats:7:0` |
| 7 | 0x36f2 | 0x362a | 0x36f2 | 0x362a | T_STC2 column B, pos 3 | `fd ff`, 381 (123, 35, 73, 38, 3, 109, 0) | MINVAL `'6231'`, MAXVAL `'6239'`, EPC 2; `...,50,.02,0,srec,4,6` | `stats:50:0` |
| 8 | 0x3896 | 0x37ce | 0x3896 | 0x37ce | T_STC2 column C, pos 4 | `fd ff`, 343 (143, 35, 15, 38, 3, 109, 0) | MINVAL `'78780102010101'`, MAXVAL `'78780214010101'`, `NUMARRAY(2458851,2458900)`, EPC 2; `...,50,.02,0,srec,8,6` | `stats:50:0` |
| - | - | - | - | - | T_STC2 columns ADDED_LATER, ADDED_NUM (pos 5, 6) | **no item**: the LF of item 8 (`stc18`@0x3a13) is followed by `TABLE "T_STC3"` @0x3a14 | - | two lines `NO STATISTICS` |
| 9 | 0x40d1 | 0x3fa5 | 0x40d1 | 0x3fa5 | T_STC3 table | plain, 81 | `SET_TABLE_STATS(...,50,4,18,6)` | `TAB\|T_STC3\|50\|...` |
| 10 | 0x413b | 0x400f | 0x413b | 0x400f | T_STC3 column ID, pos 1 | `fd ff`, 313 | as item 2 | `stats:50:0` |
| 11 | 0x429b | 0x416f | 0x429b | 0x416f | T_STC3 column A, pos 2 | `fd ff`, 393 | as item 3 | `stats:7:0` |
| - | - | - | - | - | T_STC3 column B (pos 3) | **no item**: the field of item 11 ends with `00 00` LF at `stc18`@0x4448-0x444a and the line of item 12 starts at 0x444b | - | `COLUMN\|T_STC3\|3\|B\|NO STATISTICS` |
| 12 | 0x444b | 0x431f | 0x444b | 0x431f | T_STC3 column C, pos 4 | `fd ff`, 343 (143, 35, 15, 38, 3, 109, 0) | as item 8 | `stats:50:0` |
| 13 | 0x45c9 | 0x449d | 0x45c9 | 0x449d | T_STC3 column ALLNULL, pos 5 | 18c: `fd ff`, 305 (119, 35, 38, 113, 0); 11g: 311 (119, 35, 3, 38, 3, 113, 0) | MINVAL `NULL`, MAXVAL `NULL`, both lists empty, EPC 0; `...,0,0,50,srec,0,6` (11g: `NUMARRAY(0,0)`, `NUMARRAY(0,1)`, EPC 2) | `COLUMN\|T_STC3\|5\|ALLNULL\|stats:0:50` |
| - | - | - | - | - | T_STC4, all columns | **no item**: `stc18`@0x4dd9 LF after the rows, @0x4dda `GRANT SELECT ON "T_STC4" TO PUBLIC`, @0x4dfd `TABLE "T_STC5"` | - | `TAB\|T_STC4\|NULL\|NULL`, five lines `NO STATISTICS` |
| 14 | 0x54ba | 0x52c6 | 0x54c4 | 0x52d0 | T_STC5 table | plain, `51 00` + 81 | `SET_TABLE_STATS(...,50,4,21,0)` | `TAB\|T_STC5\|50\|2026-10-07` |
| - | - | - | - | - | T_STC5, all columns | **no item**: the LF of item 14 (`stc18`@0x5523) is followed by `METRICSTreferential integrity constraints` @0x5524 | - | five lines `NO STATISTICS` |
| 15 | 0x59e8 | 0x57f4 | 0x59f2 | 0x57fe | `ANALCOMPUTE TS "T_STC1" ANALYZE  TABLE "T_STC1"  ESTIMATE STATISTICS ` (69 bytes) | line, no field | trailer "Deferred analyze commands " (`stc18`@0x59b6), group `TABLE "T_STC1"` @0x59d9 | - |
| 16 | 0x5a3d | 0x5849 | 0x5a47 | 0x5853 | the same for T_STC2 | line | group @0x5a2e | - |
| 17 | 0x5a92 | 0x589e | 0x5a9c | 0x58a8 | the same for T_STC3 | line | group @0x5a83 | - |
| 18 | 0x5ae7 | 0x58f3 | 0x5af1 | 0x58fd | the same for T_STC5 | line | group @0x5ad8; one `ENDTABLE` @0x5b2d, `METRICET 54` @0x5b36 | - |

The survey of the project owner is **confirmed, column by column**: T_STC1 1 + 2 (ID, A), T_STC2
1 + 4 (ID, A, B, C), T_STC3 1 + 4 (ID, A, C, ALLNULL), T_STC4 none, T_STC5 1 + 0; `ANALCOMPUTE`
for T_STC1, 2, 3 and 5; the same in all four files. Per file: 26 columns, 10 with a `stats:` line,
10 column items, 4 table items, 4 `ANALCOMPUTE` lines.

**Answers to the questions of the session prompt (Part B).**

| question | answer | evidence | status |
|---|---|---|---|
| B2. Rule: "a column has an item iff it has statistics (`last_analyzed` not NULL) when `exp` runs" | holds for all 26 columns of the 5 tables in all 4 files: 10 with `stats:` and an item, 16 with `NO STATISTICS` and no item; `num_distinct` and `num_nulls` of the 10 items equal the truth; `num_rows` of the 4 table items equals the `TAB\|` line | `tools/stc_items.py`: 0 problems in 4 files (104 column comparisons, 16 table items) | confirmed (2 versions, 2 paths, 3 different causes) |
| B2a. Columns added after the gather (T_STC2) | no item, for the column that is NULL in every row (ADDED_LATER) **and** for the one that is filled in every row (ADDED_NUM). The data of ADDED_NUM are in the rows: first row `stc18`@0x2db2 `02 00 c1 02` (ID 1) ... @0x2dc7 `fe ff` (ADDED_LATER) @0x2dc9 `02 00 c1 02` (ADDED_NUM 1). So "the column has values" does not give it an item | `stc18`@0x3896-0x3a13 (last item: C), @0x3a14 `TABLE "T_STC3"`; `stc11`@0x3a14 | confirmed |
| B2b. The all-NULL column that has statistics (T_STC3.ALLNULL) | it **has** an item. Against a filled column (item 12, column C) its field has: `SREC.MINVAL := NULL; SREC.MAXVAL := NULL` instead of two hexadecimal texts (first piece 119 bytes instead of 143); `SREC.EAVS := 0; SREC.CHVALS := NULL` as everywhere; on 18c `SREC.NOVALS := DBMS_STATS.NUMARRAY()` and `SREC.BKVALS := DBMS_STATS.NUMARRAY()` with the two list pieces **absent** (5 pieces) and `SREC.EPC := 0`; on 11g `NUMARRAY(0,0)`, `NUMARRAY(0,1)`, `SREC.EPC := 2` (7 pieces); arguments `<distcnt>` 0, `<density>` 0, `<nullcnt>` 50, `<avgclen>` 0, `<flags>` 6 | `stc18`@0x45df `fd ff`, @0x45e1 `77 00` (119), @0x465a `23 00` (35), @0x467f `26 00` (38), @0x46a7 `71 00` (113), @0x471a `00 00`, LF @0x471c; `stc11`@0x467f `03 00` + `0,0`, @0x46ac `03 00` + `0,1`; truth `stats:0:50` | confirmed (2 versions). It is the same field shape as a column of a table without rows (T_STAT4, 13.9): "no value" looks the same whether the table is empty or the column is all NULL; `<nullcnt>` tells them apart (50 against 0) |
| B2c. The gap in T_STC3 (B deleted) | the remaining items stand in ascending column position 1, 2, 4, 5 with nothing between items 11 and 12. The position of an item in the file is **not** related to the column number: the third column item of T_STC3 belongs to column 4. The only link between an item and its column is the column name inside the field | `stc18`@0x4448 `00 00`, @0x444a LF, @0x444b line of item 12; `tools/stc_items.py`: "bytes between two neighbouring items" is the one LF in all 28 cases (7 per file) | confirmed |
| B2d. T_STC1: two columns of five | items for ID and A (positions 1, 2), none for B, C, ALLNULL; the comment on column B (`stc18`@0x2b8e) is written although B has no statistics item: comments and statistics are independent | `stc18`@0x2857, 0x29b7, 0x2b67 | confirmed |
| B3. T_STC5: what is in the table item, and does it differ from T_STC1 | the same plain field of 81 bytes, the same text but for the name and the numbers: `BEGIN  DBMS_STATS.SET_TABLE_STATS(NULL,'"T_STC5"',NULL,NULL,NULL,50,4,21,0); END;` against `...'"T_STC1"',NULL,NULL,NULL,50,5,18,6); END;`. `num_rows` 50 = truth in both. The last argument is `0` (statistics made by `ANALYZE`) against `6` (`DBMS_STATS`), as for T_STAT3 in session 5; `avg_row_len` 21 against 18 (T_STAT3: 20 against 17). Nothing in the item says that no column item follows | `stc18`@0x54d0 `51 00`, text @0x54d2, LF @0x5523; `stc18`@0x2803; `stc11`@0x54da | confirmed as bytes (2 versions, 2 paths). `blocks` and `avg_row_len` are not in `truth_stc.txt` |
| B4. `ANALCOMPUTE` | one line for every table that has a table item (T_STC1, 2, 3, 5), none for T_STC4; no table with statistics and no line, and none the other way round. The keyword is the same for all four, `ESTIMATE STATISTICS `, whether the statistics were gathered with `DBMS_STATS` (T_STC1 to 3) or `ANALYZE` (T_STC5): the word follows the export option `statistics=` (4.6 above), not the way the statistics were made | `stc18`@0x59e8, 0x5a3d, 0x5a92, 0x5ae7 (69 bytes each, equal but for the names); now 66 lines / 66 table items in 21 files | confirmed |
| B5. Is the place of the items in the table section the same for all five tables | yes: directly after the LF that ends the rows, or after the `GRANT` line if there is one; first the table item, then the column items, then the comments. T_STC1 `G T C C M M` (0x27ca, 0x27ed, 0x2857, 0x29b7, 0x2b67, 0x2b8e); T_STC2 `T C C C C` (0x3378 ...); T_STC3 `T C C C C` (0x40d1 ...); T_STC4 `G` (0x4dda) and nothing else; T_STC5 `T` (0x54ba). The table item is always the first statistics item of its section and no table has a column item without a table item | `tools/stc_items.py`, "order after the rows", the same strings in the 4 files | confirmed for these tables |
| B6. Conventional against direct | **every item is byte-identical** (114 of 114 on both versions; the 18 statistics items included); only the binary parts differ: no `00 00` after a row (1331 against 1231 bytes for a table of 50 rows and 5 columns: 2 bytes per row) | `tools/stat_cmp.py` `stc18` / `stc18d`; `stc18`@0x2857 = `stc18d`@0x27f3 | confirmed |
| B6. 11g against 18c | 113 of 114 items byte-identical, all 5 binary parts identical (`stc18`@0x2296-0x27c9 = `stc11`@0x2296-0x27c9, 1331 bytes); **one item differs**: the field of T_STC3.ALLNULL (item 13: 305 against 311 bytes, see B2b), which moves everything after it by 10 bytes. The rule of B2 and the order are the same on both versions | `tools/stat_cmp.py` `stc18` / `stc11`: "differ at byte 154" of the field @0x45df; `tools/version_diff.py` | confirmed |

**What a reader cannot tell from the file (B7).** A missing column item has three different causes
in the fixtures and they leave the same bytes, namely none: the column was added after the
statistics were gathered (T_STC2, positions 5 and 6), the column was not named in `method_opt`
(T_STC1, positions 3 to 5), its statistics were deleted (T_STC3, position 3). There is no flag, no
date and no count in the table item or anywhere else that separates them, and no item says "this
column has no statistics". A reader can state only: "column X of table T has no statistics item
in this file". It also cannot tell "table statistics made by `ANALYZE ... FOR TABLE`" from "column
statistics all deleted" (T_STC5 shows the first; the second is not in the fixtures; hypothesis:
the same bytes but for the last argument).

#### The real dump: what the counts mean (questions D1, D3)

**Session 6:** the question of this subsection ("why 53 and not 54 or 55") is answered in "The
real dump, reconciled" below; hypothesis 2 of the list here is the one the per-table counts show.
The text is kept as written in session 5.

Counts supplied by the project owner (no fixture, **not evidence**): 6 tables with 9, 12, 13, 9,
12 and 8 columns (63), no index; `ANALSTATS TR` 58, each followed by one field (5 plain, 53 in
pieces; 84 to 531 bytes); `ANALCOMPUTE TR` 5; `TABLE` 11; `ENDTABLE` 1; `METRICST` / `METRICET` 32
each.

| count of the real dump | what the fixtures predict | fits? |
|---|---|---|
| 5 plain fields, 5 `ANALCOMPUTE TR` lines | one table item and one `ANALCOMPUTE` line per table **with statistics**: 5 of the 6 tables have statistics, one has none | yes, if one table has no statistics |
| `TABLE` 11, `ENDTABLE` 1 | 6 table sections + one `TABLE` line per `ANALCOMPUTE` line under "Deferred analyze commands " (5), closed by one `ENDTABLE` (7.1; `stat18`@0x5258-0x53b8: 4 + 1) | yes: 6 + 5 = 11 |
| `METRICST` 32 | the first line `METRICST`, 5 section lines per table (7.1), and "Deferred analyze commands " once: 1 + 5 x 6 + 1 (`stat18`: 1 + 5 x 4 + 1 = 22 lines) | yes: 32 |
| 53 fields in pieces | one column item per column of the 5 tables with statistics: 63 minus the columns of the sixth table = 54 (9 columns), 51 (12), 50 (13) or 55 (8) | **no**: 53 is none of these |
| plain 84 to 531 bytes; all fields in pieces | table block = 66 constant bytes + table name + the four numbers with three commas (84 bytes for `R_STAT`: `ru_us7`@0x190b6); column blocks without a histogram are 299 to 476 bytes in the fixtures, with a histogram up to 1189 | consistent |

So the reading "one item per table plus one per column" predicts 5 and 5 but **not 53**; the reading
"`ANALCOMPUTE` = old-style `ANALYZE`" predicts nothing here and is refuted by the fixtures. What
would have to be true for 53 (hypotheses; nothing in the fixtures decides):

1. the table without statistics has 9 columns and **one column** of another table has no column
   item (54 - 1); or
2. the table without statistics has 8 columns and **two columns** have no item (55 - 2).
   (A table without statistics with 12 or 13 columns is excluded: 51 and 50 are below 53, unless
   a table has column items without a table item, which no fixture shows.)

A column without an item is not in the fixtures: every column of every analysed table has one,
also the columns of the table without rows (`MINVAL := NULL`). Candidate causes, none tested: the
column was added after the statistics were gathered; the statistics were gathered for some columns
only; the statistics of one column were deleted; a column type for which no statistics exist; a
difference of the 10.2 `exp` client. Fixture request R-020 tests the first three.

#### The real dump, reconciled (session 6; counts only)

Counts per table supplied by the project owner from a structure-only survey of the real dump (**no
fixture, no bytes, not evidence**; no name and no value of the real dump is in this repository):

| real table | columns | `ANALCOMPUTE` | table item | column items | columns without an item | case of the R-020 fixtures with the same picture |
|---|---|---|---|---|---|---|
| T1, T2, T4, T5 | 9, 12, 9, 12 | 1 each | 1 each | 9, 12, 9, 12 | none | every column has statistics: T_STAT1 to T_STAT4, R_STAT, R_TEXT (session 5) |
| T3 | 13 | 1 | 1 | 11 | the 12th and 13th (the last two) | a table item and fewer column items than columns: T_STC1, T_STC2, T_STC3. Which of the three: **cannot be told from the file** (B7) |
| T6 | 8 | 0 | 0 | 0 | all 8 | T_STC4: no statistics at all (`stc_exp_18c.dmp`@0x4dd9-0x4dfc: rows, `GRANT`, next table) |

| count | from the rule of this section | real dump | fits |
|---|---|---|---|
| table items (`ANALSTATS TR` + plain field) | one per table with statistics: 5 | 5 | yes |
| column items (`ANALSTATS TR` + field in pieces) | one per column with statistics: 9 + 12 + 11 + 9 + 12 = 53 | 53 | yes |
| `ANALSTATS TR` lines | 5 + 53 = 58 | 58 | yes |
| `ANALCOMPUTE TR` lines | one per table with a table item: 5 | 5 | yes |

So the R-020 fixtures **predict exactly these numbers**, given which columns have statistics, and
the survey shows no table with column items and no table item, as in the fixtures. This is the
second of the two hypotheses of session 5 (table without statistics with 8 columns, two columns
of another table without an item). The fixtures cannot predict *which* columns have statistics:
that is a fact of the source database.

Why the 12th and 13th column of T3 have no item: **unverified hypothesis** (the project owner will
ask the DBA): the two columns were added to the table after its statistics were last gathered, as
in T_STC2. What speaks for it: they are the last two columns, and one of them is a NUMBER that is
almost always filled, the other a VARCHAR2(1) that is almost always NULL: T_STC2 shows that a
filled column (ADDED_NUM) and an empty one (ADDED_LATER) both stay without an item until the
statistics are gathered again, and T_STC3.ALLNULL shows that "almost always NULL" is no reason for
a missing item. What it does not exclude: statistics gathered with a column list that names the
first 11 columns (T_STC1 also loses its last columns), or deleted for these two (T_STC3). The file
is the same in all three cases.

Limits of this reconciliation: the fixtures were written by an `exp` of the same version as the
database and with the AL32UTF8 client; the real dump is said to come from a 10.2 `exp` client
against an 11.2.0.4 database with a US7ASCII client (letters `TR`). In the R-016 fixtures the
client set changes the letter only (4.6, "The second letter"); the R-020 tables were not exported
with a US7ASCII client (R-023), and no fixture has an older client (R-022).

**Consequence for a reader** (rule S7 of section 11). For a table with n columns the number of
column items is anything from 0 to n; a table can have a table item and no column item (T_STC5),
or no statistics item at all (T_STC4), also in a file whose other tables have them. A reader of
table data therefore never counts statistics items, never expects n + 1 of them, and never pairs
the k-th column item with the k-th column: it skips each item by its framing (rules S1, S2) until
the section ends (rule T2). A tool that reports statistics names the column from the text of the
field.

**Structure-only script specification** (for the project owner's tool; it must print counts and
positions, never a name, a number of the statistics or a value). Walk the items with rule I6
(section 11). For every table section, in file order, print one line:

1. ordinal of the table section (1, 2, ...), **not** its name; number of columns (the u16 after
   the `INSERT INTO` line); whether the table has rows (`ff ff` directly after the path flag: yes / no);
2. number of `ANALSTATS T?` items in the section whose field is plain, and whose field is in
   pieces; the letters seen;
3. for every item in pieces: find `SET_COLUMN_STATS(NULL,'"` and take the second quoted name; print
   only its **position in the column list of the `INSERT INTO` line** (1-based), or `not found`.
   Then print the list of column positions that have **no** item, and for each of them the type
   code and the length of its descriptor (rule T5: two u16; these are declared sizes, not data);
4. for every item in pieces: the number of pieces, and five flags: `MINVAL := NULL` (yes / no),
   `CHVALS := NULL` (yes / no), `EAVS := 0` (yes / no), the piece between `NUMARRAY(` and `)` absent
   (yes / no), last argument before `); END;` (print the digit: it is `0` or `6` in the fixtures and
   is not data);
5. for every plain item: whether its text starts with `BEGIN  DBMS_STATS.SET_TABLE_STATS(`; the
   last argument; nothing else;
6. whether the section has `COMMENT ON` items, `GRANT` lines, and the order of the kinds after the
   rows as a string of letters (`G` grant, `T` table statistics, `C` column statistics, `M`
   comment), so that the place of `GRANT` is known;
7. per table ordinal: whether an `ANALCOMPUTE T?` line names it (match by name, print yes / no and
   the word `ESTIMATE` or `COMPUTE`);
8. any field whose text matches none of the three blocks of this section: its form, its length,
   and its first 40 bytes with every digit replaced by `9` and every letter inside quotes by `x`.

What decides: (a) which table has 0 items, and its column count (9 or 8 expected); (b) which
column positions have no item and their type codes; (c) whether a table has column items without
a table item. Never print `MINVAL` / `MAXVAL` (they are the smallest and largest value of the
column, in hexadecimal) or the lists.

**Does a reader of table data need to decode statistics (D3)?** No. Minimum: skip by framing. An
`ANALSTATS` line (prefix `ANALSTATS `, list of section 11) is followed by one field, plain or in
pieces after `fd ff`; an `ANALCOMPUTE` line is a line. Evidence that nothing more is needed: the
binary part of every table is identical with and without the statistics items (table above), all
rows of the 20 R-016 files equal the truth or the SQL without reading any statistics
(`tools/check_all.py`: T_STAT1 150 fields per file, R_STAT 12000, R_TEXT 1854, 0 differ), and no
statistics item stands before the rows of its table. A reader **may** use the table block as a
plausibility check only: `num_rows` is the row count at the time the statistics were gathered, not
at export time, so it must never be used as the row count.

#### The R-020 tables with a US7ASCII client and with `statistics=none` (session 7, R-023)

`stc_exp_us7ascii_<v>.dmp` (client US7ASCII, `statistics=estimate`) and `stc_exp_none_<v>.dmp`
(client AL32UTF8, `statistics=none`), both conventional, both versions, compared with
`stc_exp_<v>.dmp` by `tools/stat_cmp.py` and with `truth_stc_extra.txt` by `tools/stc_items.py`.

| question | `stc_exp_none_<v>` | `stc_exp_us7ascii_<v>` | status |
|---|---|---|---|
| Binary parts (column count to the LF after the rows) | identical to `stc_exp_<v>` for all 5 tables (1331 / 1519 bytes each), e.g. T_STC2 `stc_exp_18c.dmp`@0x2d88-0x3377 = `stc_exp_none_18c.dmp`@0x2a0e-0x2ffd | identical for all 5 tables, e.g. T_STC3 `stc_exp_18c.dmp`@0x3b9d-0x40d0 = `stc_exp_us7ascii_18c.dmp`@0x3e65-0x4398 | confirmed (2 versions x 2 files x 5 tables) |
| Statistics items | none: 0 `ANALSTATS`, 0 `ANALCOMPUTE`, no section "Deferred analyze commands ". 114 - 75 = 39 items fewer: 14 lines + 14 fields + 11 trailer items, exactly the arithmetic of R-023 (b). The `GRANT` and `COMMENT ON` items stay (T_STC1 `G M M`, T_STC4 `G`) | 22 `ANALSTATS TR` (5 table items, 17 column items) and 5 `ANALCOMPUTE TR`, against 14 + 4 `TS` in `stc_exp_<v>` | confirmed (R-023 (b) holds; (a) is refuted in the counts, see below) |
| Only the second letter changed? | - | for the items that exist in both files: yes, `S` -> `R` in 14 `ANALSTATS` and 4 `ANALCOMPUTE` lines (e.g. `stc_exp_18c.dmp`@0x27ed `TS` / `stc_exp_us7ascii_18c.dmp`@0x27ed `TR`), apart from one field: the table item of T_STC2 says `...,50,4,18,6)` in `stc_exp_<v>` and `...,50,4,21,6)` here (@0x338e, both versions; avg_row_len 18 -> 21) | confirmed |
| New items | - | T_STC2: column items for ADDED_LATER and ADDED_NUM (`stc_exp_us7ascii_18c.dmp`@0x3a14, 0x3b6c; 11g @0x3a14, 0x3b76); T_STC4: a table item and 5 column items (lines and fields from 18c @0x50c5, 11g @0x50d9) and an `ANALCOMPUTE TR "T_STC4"` line (18c @0x659f); `METRICET 54` -> `METRICET 55` (18c @0x6643, 11g @0x6661) | confirmed |
| Against the truth | every statistics item is absent although `truth_stc_extra.txt` lists 17 columns with statistics: `statistics=none` removes them whatever the database holds | `tools/stc_items.py`: 0 problems against `truth_stc_extra.txt`: a column item exactly for the 17 `stats:` lines, a table item for the 5 tables with a date, an `ANALCOMPUTE` line exactly with a table item, num_distinct / num_nulls of every item equal | confirmed (rules S6, S7 hold on the new files) |
| `EXP-00091` in the log | 0 | 5 (one per plain field = per table item; rule of 4.6) | confirmed |

**What happened between the exports (the lesson).** `truth_stc.txt` was written with the R-020
exports; `truth_stc_extra.txt` immediately before the R-023 exports. They differ in exactly the
lines that the US7ASCII files differ in: `TAB|T_STC4|NULL|NULL` became `TAB|T_STC4|50|2026-10-07`,
the five T_STC4 columns and T_STC2.ADDED_LATER / ADDED_NUM went from `NO STATISTICS` to `stats:`.
Observed: the statistics of the tables changed in the database between two exports, and each file
holds the statistics of its own moment. That an automatic statistics job of the database made
them is a hypothesis (the comment of `corpus/sql/10b_stc_truth.sql` says so; nothing in the files
names the cause). Consequences: (1) a truth file for statistics is valid only for exports written
right after it; the R-023 arithmetic (a), computed from `truth_stc.txt`, was wrong for that reason,
not because of the client set; (2) two exports of the same table can differ in their statistics
items with nothing changed by the user, so no rule may rest on the number of items (rules S7, T7
already say so); (3) the client set changes the letter and nothing else in the items that both
files have, as in the R-016 files.

### 4.7 The column list of the `CREATE TABLE` line (session 7, R-025)

The descriptor gives no precision or scale (4.1). A reader that wants the declared type reads the
`CREATE TABLE` line of the section (rule T1). `tools/collist.py` parses the column list of all 314
`CREATE TABLE` lines of the 169 files with the grammar below: 0 lines it cannot read; for the 314
sections the number of definitions equals the column count, every quoted name equals the
`INSERT INTO` name at the same position (byte for byte), and every type text agrees with the
descriptor's type code and length (1647 columns, 0 differences).

Grammar (bytes of the line; the line is in the client character set, rule C2):

    line     := 'CREATE TABLE ' qname ' (' coldef (', ' coldef)* ')' '  ' storage-text
    coldef   := qname ' ' type [' NOT NULL ENABLE']
    qname    := '"' (any byte except '"')* '"'
    type     := 'NUMBER' | 'NUMBER(' p ', ' s ')' | 'NUMBER(*,' s ')' | 'FLOAT(' b ')'
              | 'VARCHAR2(' n [' CHAR'] ')' | 'CHAR(' n [' CHAR'] ')' | 'NVARCHAR2(' n ')' | 'NCHAR(' n ')'
              | 'RAW(' n ')' | 'DATE' | 'TIMESTAMP (' f ')' [' WITH TIME ZONE' | ' WITH LOCAL TIME ZONE']
              | 'CLOB' | 'LONG' | 'LONG RAW'
              | 'NCLOB' | 'BLOB' | 'ROWID' | 'UROWID(' n ')'                         (session 8)
              | 'INTERVAL YEAR (' p ') TO MONTH '   (ends with a space)             (session 8)
              | 'INTERVAL DAY (' p ') TO SECOND (' f ')'                            (session 8)
    p, n, b, f := decimal digits;  s := ['-'] decimal digits

What `exp` writes for the 14 columns of T_DECL (`t_decl_exp_<v>.dmp`@0x211c, the line is 518
bytes; offsets of each definition in the 18c and the 11g file), against `truth_decl.txt`:

| declared in `11_types.sql` | text in the file | offset | precision / scale from the text | `truth_decl.txt` (type, precision, scale, length, nullable) | status |
|---|---|---|---|---|---|
| `n_plain NUMBER` | `"N_PLAIN" NUMBER` | 0x2133 | none / none | NUMBER, NULL, NULL, 22, Y | confirmed |
| `n_star NUMBER(*,2)` | `"N_STAR" NUMBER(*,2)` (no space after the comma) | 0x2145 | none / 2 | NUMBER, NULL, 2 | confirmed |
| `n_38 NUMBER(38)` | `"N_38" NUMBER(38, 0)` | 0x215b | 38 / 0 | NUMBER, 38, 0 | confirmed |
| `n_neg NUMBER(5,-2)` | `"N_NEG" NUMBER(5, -2)` | 0x2171 | 5 / -2 | NUMBER, 5, -2 | confirmed (also `t_numtypes`) |
| `n_sgp NUMBER(3,5)` | `"N_SGP" NUMBER(3, 5)` | 0x2188 | 3 / 5 (scale above precision) | NUMBER, 3, 5 | confirmed |
| `i_int INTEGER` | `"I_INT" NUMBER(*,0)` | 0x219e | none / 0 | NUMBER, NULL, 0 | confirmed |
| `d_dec DECIMAL(10,2)` | `"D_DEC" NUMBER(10, 2)` | 0x21b3 | 10 / 2 | NUMBER, 10, 2 | confirmed |
| `f_plain FLOAT` | `"F_PLAIN" FLOAT(126)` | 0x21ca | binary precision 126 / none | FLOAT, 126, NULL | confirmed |
| `f_126 FLOAT(126)` | `"F_126" FLOAT(126)` | 0x21e0 | 126 / none | FLOAT, 126, NULL | confirmed |
| `r_real REAL` | `"R_REAL" FLOAT(63)` | 0x21f4 | 63 / none | FLOAT, 63, NULL | confirmed |
| `ts_def TIMESTAMP` | `"TS_DEF" TIMESTAMP (6)` (space before the parenthesis) | 0x2208 | fraction 6 | TIMESTAMP(6), NULL, 6, 11 | confirmed |
| `d_nn DATE NOT NULL` | `"D_NN" DATE NOT NULL ENABLE` | 0x2220 | - | DATE, 7, **N** | confirmed |
| `v_def VARCHAR2(10) DEFAULT 'a,b)'` | `"V_DEF" VARCHAR2(10)`: **no DEFAULT in the line**. The default is the field after the line `ALTER TABLE "T_DECL" MODIFY DEFAULT` (@0x2323): `ALTER TABLE "T_DECL" MODIFY ("V_DEF" DEFAULT 'a,b)')` (plain, 52 bytes @0x2347), between `CREATE TABLE` and `INSERT INTO` | 0x223d | - | VARCHAR2, 10 | confirmed (also `ddl`, 4.3) |
| `"low,er(x" VARCHAR2(5)` | `"low,er(x" VARCHAR2(5)` (quoted as declared; comma and parenthesis inside the quotes; the same in the `INSERT INTO` line @0x237e) | 0x2253 | - | VARCHAR2, 5 | confirmed |

The line ends `VARCHAR2(5))  PCTFREE ...`: one `)` closes the list and two spaces follow it, in all
314 lines (the storage text that follows differs by version and table: 13.5).

| statement | evidence | status |
|---|---|---|
| Between a quoted name and the next `, "` or the closing `)` stand only one space, the type text and optionally ` NOT NULL ENABLE`. No `DEFAULT`, no `CHECK`, no constraint name appears in the column list; defaults come as `MODIFY DEFAULT` fields (4.2) and constraints as later items | 314 lines, `tools/collist.py`; forms seen: `NUMBER` 70, `NUMBER(p, s)` 279 (+ 35 with `NOT NULL ENABLE`), `NUMBER(p, -s)` 6, `NUMBER(*,s)` 4, `FLOAT(b)` 10, `VARCHAR2(n)` 896 (+ 28), `VARCHAR2(n CHAR)` 25, `CHAR(n)` 51, `CHAR(n CHAR)` 2, `NCHAR(n)` 2, `NVARCHAR2(n)` 43, `RAW(n)` 12, `DATE` 137 (+ 2), `TIMESTAMP (f)` 26, `... WITH TIME ZONE` 4, `... WITH LOCAL TIME ZONE` 4, `CLOB` 7, `LONG` 2, `LONG RAW` 2 | confirmed for the fixtures |
| A name is always in double quotes and ends at the next `"`; commas, parentheses and lower-case letters inside it are part of it. The separator is `, ` after a type text, never inside quotes | `t_decl_exp_<v>`@0x2253 `"low,er(x"` | confirmed (one name with `,` and `(`); a name containing `"` cannot be created in Oracle (assumed, not tested) |
| Precision and scale: `NUMBER` = none / none; `NUMBER(p, s)` = p / s (s may be negative or larger than p); `NUMBER(*,s)` = none / s; `FLOAT(b)` = binary precision b, no scale. The text gives exactly the `data_precision` and `data_scale` of `user_tab_columns` for all 14 columns | `truth_decl.txt`, `tools/collist.py` (14 of 14 per version) | confirmed |
| The text does not keep the declared spelling: `INTEGER` = `NUMBER(*,0)`, `DECIMAL(10,2)` = `NUMBER(10, 2)`, `FLOAT` = `FLOAT(126)`, `REAL` = `FLOAT(63)`, `TIMESTAMP` = `TIMESTAMP (6)`. The dictionary does not keep it either (`truth_decl.txt`), so nothing that the database knows is lost | same | confirmed |
| `NUMBER(p)` is written `NUMBER(p, 0)`; a `NUMBER(*,s)` has no space after the comma, a `NUMBER(p, s)` has one | @0x215b, @0x2145 | confirmed |
| `TIMESTAMP (f)` has a space before `(`; `VARCHAR2(`, `NUMBER(`, `FLOAT(` have none | @0x2208 | confirmed |
| Not in the fixtures: `BINARY_FLOAT` / `BINARY_DOUBLE` (exp refuses those tables, `docs/scope.md`), `BFILE`, object and XML types, virtual or invisible columns, `VARCHAR2(n BYTE)` written with the word `BYTE`, a column list longer than one line. (Session 8: `INTERVAL`, `ROWID`, `UROWID`, `BLOB`, `NCLOB` left this row, see below) | - | unknown |
| (session 8) The types of R-027 and R-029 are written `BLOB`, `NCLOB`, `ROWID`, `UROWID(4000)` (declared as `UROWID`), `INTERVAL YEAR (4) TO MONTH ` and `INTERVAL DAY (4) TO SECOND (6)`: a space before each `(`, and after `MONTH` a space **before** the separator `, ` (`"IYM" INTERVAL YEAR (4) TO MONTH , "IDS" ...`). A grammar that expects `, ` right after the type text fails on the year-month interval | `misc_exp_<v>`@0x2120 (`CREATE TABLE "T_INTERVAL"`, +52: `INTERVAL YEAR (4) TO MONTH , `), @0x230e (`"RID" ROWID, "URID" UROWID(4000)`); `lob_exp_<v>`@0x211b, @0x4d792 / 0x4db94; `tools/collist.py` with the grammar extended: 334 lines of 177 files, 1715 column definitions against the descriptors, 0 problems | confirmed (1 table per type, 2 versions); the precision (4) and fraction (6) are those of the SQL (`01b_optional_tables.sql`) |
| (session 8) A partitioned table: the column list is followed, after the two spaces, by the storage text with `PARTITION BY RANGE ("SALE_DATE" )  (PARTITION "P2019" VALUES LESS THAN (TO_DATE(' 2020-01-01 00:00:00', 'SYYYY-MM-DD HH24:MI:SS', 'NLS_CALENDAR=GREGORIAN')) ... , PARTITION "PMAX" VALUES LESS THAN (MAXVALUE) ... )`; the grammar of the column list is unaffected | `part_exp_18c.dmp`@0x211c (1035 bytes) | observed (one table) |
| (session 9) A composite-partitioned table: `... TABLESPACE "USERS"  PARTITION BY RANGE ("ID" )  SUBPARTITION BY HASH ("K" )  SUBPARTITIONS 2 (PARTITION "P1" VALUES LESS THAN (100) ... (SUBPARTITION "SYS_SUBP368"  TABLESPACE "USERS", SUBPARTITION "SYS_SUBP369"  TABLESPACE "USERS" ), ...)`; the column list is unaffected | `part2_exp_18c.dmp`@0x211d (842 bytes), `part2_exp_direct_18c.dmp`@0x211d | observed (one table, 2 files) |
| (session 9) **The names of the `INSERT INTO` line equal those of `CREATE TABLE` in order: refuted for LONG tables on the direct path** (the LONG column stands last in `INSERT INTO` and in the descriptors, 4.1). Everywhere else the statement holds (session 11: 386 lines of 201 files, 1,907 column definitions; session 9: 382 lines of 199 files, 1885 definitions; `tools/collist.py`: 0 problems, 6 sections "moved") | `long2_exp_direct_<v>`@0x211d / 0x221c; `long_exp_direct_<v>`@0x211c, 0x11e0c (18c) | confirmed (6 sections, 2 versions) |
| (session 9) The character-semantics forms `VARCHAR2(n CHAR)`, `CHAR(n CHAR)` and the byte-semantics `VARCHAR2(10)` (declared `VARCHAR2(10 BYTE)`: the word `BYTE` is not written) | `t_probe11_exp_<v>`@0x211f: `"B" VARCHAR2(10), "C" VARCHAR2(10 CHAR), "K" CHAR(3 CHAR), "W" VARCHAR2(1000 CHAR), "X" VARCHAR2(1001 CHAR), "Y" CHAR(500 CHAR), "Z" CHAR(501 CHAR)` | confirmed (4 files, `truth_probe11.txt` `char_used` B / C) |

### 4.8 A partitioned table (session 8, R-030; `part_exp_18c.dmp`, `part_exp_direct_18c.dmp`)

T_PART (`ID NUMBER(6)`, `SALE_DATE DATE`, `AMOUNT NUMBER(10,2)`), range-partitioned on SALE_DATE into
P2019, P2020, PMAX; 3000 rows (`01b_optional_tables.sql`). Oracle 18c only, table mode, both paths,
`statistics=none`. Offsets: conventional / direct.

| offset (conv / direct) | item | meaning | evidence | status |
|---|---|---|---|---|
| 0x210d / 0x210d | line `TABLE "T_PART"` | one table section for the whole table (rule T1: followed by `CREATE TABLE`) | both files | confirmed (2 files) |
| 0x211c / 0x211c | line `CREATE TABLE "T_PART" (...)  ... PARTITION BY RANGE ...` (1035 bytes) | one `CREATE TABLE` line; the partitions are named only inside its storage text (4.7) | both files | confirmed |
| 0x2528 / 0x2528 | line `PARTITION "P2019"` | **new line kind**: names the partition whose rows follow. A plain line; no field follows it | both files; `tools/intro_list.py` 0 problems (it is not an introducing line) | confirmed (3 lines per file, 2 files) |
| 0x253a / 0x253a | line `INSERT INTO "T_PART" ("ID", "SALE_DATE", "AMOUNT") VALUES (:1, :2, :3)` | an ordinary `INSERT INTO` line (rule T3), the same text for every partition | both files | confirmed |
| 0x2581 / 0x2581 | `03 00`, 3 descriptors, `00 00`, path flag (0 / 1) | a complete binary part (T4 to T6) for this partition; the 14 bytes from the column count to the flag are identical in all three parts | `tools/part_check.py` | confirmed |
| 0x2593 / 0x2593 | rows of P2019 | 1095 rows, then `ff ff` @0x82e0 / 0x7a52 and LF | `truth_part.txt` `P\|P2019\|1095`; log "P2019 1095 rows exported" | confirmed |
| 0x82e3 / 0x7a55 | line `PARTITION "P2020"`, `INSERT INTO` @0x82f5 / 0x7a67, binary part @0x833c / 0x7aae, rows @0x834e / 0x7ac0, `ff ff` @0xe104 / 0xcfe2 | second partition: 1098 rows | `P\|P2020\|1098`; log | confirmed |
| 0xe107 / 0xcfe5 | line `PARTITION "PMAX"`, `INSERT INTO` @0xe118 / 0xcff6, binary part @0xe15f / 0xd03d, rows @0xe171 / 0xd04f, `ff ff` @0x1265a / 0x10eea | third partition: 807 rows | `P\|PMAX\|807`; log | confirmed |
| 0x1265d / 0x10eed | line `ENDPARTITION` | **new line**, once, after the last partition's LF; then the trailer (`METRICST...`) | both files | confirmed (2 files; one table) |

| statement | evidence | status |
|---|---|---|
| A partitioned table has **one** table section and inside it **one binary part per partition**, each introduced by `PARTITION "name"` and its own `INSERT INTO` line. A reader that allows only one `INSERT INTO` per section stops or misreads at the second one | both files; `tools/exp_parse.py` (session 8) reads each part as its own entry | confirmed (1 table, 3 partitions, 2 paths) |
| The rows of each part belong to that partition: every SALE_DATE of P2019 is before 2020-01-01, of P2020 in 2020, of PMAX from 2021-01-01 on; every one of the 3000 rows equals the SQL (`ID`, `SALE_DATE` = 2019-01-01 + MOD(13 id, 1000), `AMOUNT` = MOD(31 id, 100000) / 100); ids 1 to 3000 each once; count 3000, sum 1395465, min 2019-01-01, max 2021-09-26 = the `TOTAL` line | `tools/part_check.py`: 38 checks, 0 differ | confirmed |
| The row counts per part are 1095, 1098, 807 = `truth_part.txt` = the log = the arithmetic of R-030 (365, 366, 269 days x 3) | same | confirmed |
| The order of the parts is the order of the partitions in the `CREATE TABLE` text and the log | both files | observed (one table) |
| Nothing marks a partition boundary except the items above: no count, no extra bytes; the binary part of each partition is laid out exactly like an unpartitioned table (T4 to T6, R1 to R4, E1); on the direct path every part has flag 1 and ends with `ff ff` (three parts with rows; an empty partition is not in the fixtures) | both files; `tools/check_all.py` `problems 0` | confirmed |
| `METRICET` after the trailer sections says 807, the row count of the last partition (`part_exp_18c.dmp`@0x12694 `METRICET 807`): it is not the table's row count (3000); rule R6 holds | trailer | observed |
| Not in the fixtures: subpartitions; list or hash partitions; an empty partition (on the 18c direct path probably without `ff ff`, as in 5.4); a partitioned table with statistics, indexes, or in owner mode; partitioned tables on 11g | - | unknown (session 9: subpartitions, an empty partition and statistics are answered below) |

#### Session 9: composite partitions, an empty partition, partition statistics (R-037)

T_PART2 (`ID NUMBER(6)`, `K NUMBER(3)`, `V VARCHAR2(10)`), `PARTITION BY RANGE (id) SUBPARTITION BY
HASH (k) SUBPARTITIONS 2`: P1 (ids 1 to 50), P_EMPTY (none), P3 (ids 300 to 349); statistics gathered
with `DBMS_STATS.GATHER_TABLE_STATS` before the export, default `statistics`. 18c only. Offsets:
conventional / direct. `tools/part2_check.py`: 52 checks, 0 differ; `tools/check_truth.py`: every row
of every subpartition equals the `ROW` lines of `truth_part2.txt` (id) and the SQL (K, V).

| offset (conv / direct) | item | meaning | evidence | status |
|---|---|---|---|---|
| 0x210d / 0x210d | `TABLE "T_PART2"`, `CREATE TABLE` @0x211d (842 bytes, with `PARTITION BY RANGE ... SUBPARTITION BY HASH ...`) | one section for the whole table | both files | confirmed |
| 0x2468 / 0x2468 | line `COMPARTITION "P1"` | **new line kind**: opens a composite partition (no field follows; not in list K) | both files; `tools/intro_list.py` 0 problems | confirmed (3 per file, 2 files) |
| 0x247a / 0x247a | line `SUBPARTITION "SYS_SUBP368"` | **new line kind**: names the subpartition whose binary part follows | `truth_part2.txt` `SP\|P1\|SYS_SUBP368` | confirmed (6 per file) |
| 0x2495 / 0x2495 | `INSERT INTO "T_PART2" ("ID", "K", "V") VALUES (:1, :2, :3)` | the ordinary line (T3), the same text for every subpartition | both files | confirmed |
| 0x24d0 / 0x24d0 | `03 00 02 00 16 00 02 00 16 00 01 00 0a 00 69 03 01 00 00 00` + flag | a complete binary part per **subpartition**; the 20 bytes from the column count to m are identical in all 6 parts | `tools/part2_check.py` | confirmed (12 parts) |
| 0x24e6 / 0x24e6 | 21 rows, `ff ff` @0x261d / 0x25f3 | rows of SYS_SUBP368 = the 21 `ROW` lines with SYS_SUBP368; log "SYS_SUBP368 21 rows" | truth, log | confirmed |
| 0x2620 / 0x25f6 | `SUBPARTITION "SYS_SUBP369"`, `INSERT INTO`, binary part @0x2676 / 0x264c, 29 rows, `ff ff` @0x2833 / 0x27cf | second subpartition of P1 | truth, log | confirmed |
| 0x2836 / 0x27d2 | line `ENDCOMPARTITION` | **new line**: closes the composite partition | both files | confirmed (3 per file) |
| 0x2846 / 0x27e2 | `COMPARTITION "P_EMPTY"`, `SUBPARTITION "SYS_SUBP370"` @0x285d / 0x27f9, binary part @0x28b3 / 0x284f | an **empty** subpartition | log "SYS_SUBP370 0 rows" | confirmed |
| 0x28c9 / 0x2865 | conventional: `ff ff 0a` right after the flag (E1). Direct: **no `ff ff`**: `0a` and then `SUBPARTITION "SYS_SUBP371"` (E2; u16 at p = `0a 53` = 21258 > 22) | the empty-table rules apply per binary part | both files | confirmed (2 parts per file) |
| 0x2938 / 0x28d2 | SYS_SUBP371: conventional `ff ff 0a`; direct `0a` + `ENDCOMPARTITION` (u16 `0a 45` = 17674 > 22) | second empty subpartition, followed by the closing line | both files | confirmed |
| 0x294b / 0x28e3 | `COMPARTITION "P3"`, SYS_SUBP372 (21 rows), SYS_SUBP373 (29 rows), `ENDCOMPARTITION` @0x2d4a / 0x2c7e | third partition | truth, log | confirmed |
| 0x2d5a / 0x2c8e | 16 `ANALSTATS TS "T_PART2"` lines + fields: the table (partition `NULL`: plain field @0x2d71 / 0x2ca5, `SET_TABLE_STATS(NULL,'"T_PART2"',NULL,...,100,...)`), its 3 columns (pieces), then for P1, P_EMPTY, P3 one table item (`SET_TABLE_STATS(NULL,'"T_PART2"','"P1"',...,50,...)` @0x3258 / 0x318c) and 3 column items each | statistics of a partitioned table stand **inside** the partition block: after the last `ENDCOMPARTITION`, before `ENDPARTITION`; per partition, none per subpartition (whether the database had subpartition statistics is not in the truth) | `tools/part2_check.py` skeleton; `tools/intro_list.py` (all 16 fields well-formed by rule S1) | confirmed (2 files) |
| 0x407a / 0x3fae | line `ENDPARTITION` | once, after the statistics items | both files | confirmed |
| 0x4087 / 0x3fbb | `TABLE "T_FRAME"` ... | the next section, byte-identical to `t_frame_exp(_direct)_18c` (rule T8; `tools/frame_t8.py`) | both files | confirmed |
| 0x4a02 / - | `ANALCOMPUTE TS "T_PART2" ANALYZE  TABLE "T_PART2"  ESTIMATE STATISTICS ` in the trailer group "Deferred analyze commands " | as for any table with statistics (S3) | `part2_exp_18c.dmp` | observed |

| statement | evidence | status |
|---|---|---|
| A composite-partitioned table has one binary part per **subpartition**; a partition with subpartitions has no binary part of its own. Lines: `COMPARTITION "p"`, then per subpartition `SUBPARTITION "s"` + `INSERT INTO` + binary part, then `ENDCOMPARTITION`; after all partitions (and their statistics) one `ENDPARTITION` | both files, table above | confirmed (1 table, 3 partitions, 6 subpartitions, 2 paths) |
| The rows of a part are exactly the rows of that subpartition (`ROW\|id\|subpartition`); the subpartition belongs to the partition of the enclosing `COMPARTITION` line (`SP` lines); 100 rows in all | `tools/part2_check.py`, `tools/check_truth.py` | confirmed |
| An empty (sub)partition is an ordinary empty binary part: E1 on the conventional path, E2 (no `ff ff`) on the 18c direct path, where the item after p is a `SUBPARTITION` or `ENDCOMPARTITION` line. Both tests of E2 agree | both files; `tools/check_all.py` `empty_rule_diff` 0 | confirmed (18c) |
| Statistics items of a partitioned table come after the last part and before `ENDPARTITION`, so a reader that ends the partition block at the last `INSERT INTO` part must still read items until `ENDPARTITION`; all are ordinary items (S1) | both files | confirmed (1 table) |
| `ENDPARTITION` directly follows the last binary part when there are no statistics (session 8, `part_exp_18c`) | 4.8 above | confirmed |
| Not in the fixtures: list, interval, reference or system partitioning; range-list or range-range; a subpartition template; subpartition-level statistics; a partitioned table with indexes, LOB, LONG, or in owner mode; partitions on 11g | - | unknown |

## 5. Row data

### 5.1 Layout

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x227b (first field) | 2 | every field starts with a u16 LE length; the length counts the value bytes only, not itself | 1 to 10: CONV@0x227b, 0x22bf, 0x22c2, 0x22c6, 0x22cd; 126 to 400 including 255 -> 256: table 5.2; 2000: `t_raw`@0x226a `d0 07` | confirmed (`truth_len.txt` all 21 non-NULL rows, both paths) |
| - | - | the width of the length field does not change with the length: always 2 bytes | 250: `t_len`@0x277b `fa 00`; 251: @0x287e `fb 00`; 254: @0x2b8d `fe 00`; 255: @0x2c94 `ff 00`; 256: @0x2d9c `00 01`; 257: @0x2ea5 `01 01` | confirmed |
| after the length | n | n value bytes, no terminator | all rows of all fixtures | confirmed |
| 0x2295 | 2 | NULL = `fe ff` (0xFFFE) in place of the length, no value bytes. Same for every column type | VARCHAR2: CONV@0x2295; NUMBER: `t_nullmix`@0x22a9, `t_num`@0x2451; DATE: `t_nullmix`@0x22f2, `t_date`@0x22e1; CHAR and NVARCHAR2: `t_text`@0x238b, 0x238d; TIMESTAMP: `t_ts`@0x235f | confirmed (`truth_nullmix.txt`, `truth_num.txt` row 33, `truth_date.txt` row 11, `truth_text.txt` row 6, `truth_ts.txt` row 6) |
| - | - | a length of 254 (`fe 00`) and NULL (`fe ff`) differ in the second byte | `t_len`@0x2b8d `fe 00` (row 254) | confirmed |
| - | 2 each | trailing NULLs are written, one `fe ff` per column | `t_nullmix`@0x22f2 (row 5), @0x230e, 0x2310 (row 7), @0x2318-0x231d (row 8); CONV@0x22a5, 0x22a7 | confirmed |
| 0x228d | 2 | conventional path: `00 00` after the last field of every row | CONV@0x228d ...; `t_nullmix`@0x22a3, 0x22ba, ... (8 times); `t_probe6` 300 times; every conventional fixture | confirmed |
| (absent) | 0 | direct path: nothing between rows | DIRECT@0x228d; `t_nullmix/direct`@0x22a3 (row 2 starts right after the DATE of row 1); every direct fixture | confirmed |
| 0x22f3 | 2 | `ff ff` where the next row would start = end of this table's rows; then LF | CONV@0x22f3; `multi_framing`@0x22f3, 0x3466, 0x367c (one per table); `t_probe6`@0xa59e | confirmed |
| - | - | every row has exactly as many fields as the column count; no per-row length, no per-row column count, no row number | all tables, `tools/exp_parse.py` + `tools/check_truth.py` | confirmed |
| - | - | no row count is stored before the rows | `t_empty`@0x21f4-0x220b compared with CONV@0x225d-0x227a: same structure | confirmed (as absence) |

### 5.2 Length field around 250 to 257 bytes (`t_len`)

Column A is VARCHAR2(400). "truth len" is the `Len=` of `DUMP(a)` in `truth_len.txt`; the value
bytes equal the 12 bytes the truth file shows and the rest is `x` (the truth file shows only 12).

| truth row (id = length) | length field | `t_len` offset | `t_len/direct` offset |
|---|---|---|---|
| 0 (NULL) | `fe ff` | 0x3279 | 0x324f |
| 1 | `01 00` | 0x2255 | 0x2255 |
| 2 | `02 00` | 0x225e | 0x225c |
| 3 | `03 00` | 0x2268 | 0x2264 |
| 10 | `0a 00` | 0x2273 | 0x226d |
| 126 | `7e 00` | 0x2286 | 0x227e |
| 127 | `7f 00` | 0x230d | 0x2303 |
| 128 | `80 00` | 0x2395 | 0x2389 |
| 129 | `81 00` | 0x241e | 0x2410 |
| 200 | `c8 00` | 0x24a7 | 0x2497 |
| 248 | `f8 00` | 0x2578 | 0x2566 |
| 249 | `f9 00` | 0x2679 | 0x2665 |
| 250 | `fa 00` | 0x277b | 0x2765 |
| 251 | `fb 00` | 0x287e | 0x2866 |
| 252 | `fc 00` | 0x2982 | 0x2968 |
| 253 | `fd 00` | 0x2a87 | 0x2a6b |
| 254 | `fe 00` | 0x2b8d | 0x2b6f |
| 255 | `ff 00` | 0x2c94 | 0x2c74 |
| 256 | `00 01` | 0x2d9c | 0x2d7a |
| 257 | `01 01` | 0x2ea5 | 0x2e81 |
| 300 | `2c 01` | 0x2fae | 0x2f88 |
| 400 | `90 01` | 0x30e2 | 0x30ba |

Status: confirmed. There is no change of width at 250 / 251 or at 127 / 128 or at 255 / 256.
Largest length seen: 2000 (`t_raw`@0x226a); session 4: 3000 (`b8 0b`,
`ru_tbl_exp_cp1251_11g.dmp`@0x4727, a VARCHAR2(4000) value). Lengths of 0xFFFE and more cannot be written with
this field; how longer values (LONG, LOB: out of scope) are written is unknown.
Session 7: 32,000 (`00 7d`), one field, for a LONG value (`long_exp_<v>`@0x2289) and a LONG RAW
value (@0xa0ea); section 6.6. Values above 65,533 bytes are still not in the fixtures.
Session 8: a LOB value is not a column field; above 6,000 bytes `exp` can write it as `fc ff` + an
8-byte length + chunks (14.6, rule L4). Whether a LONG value above 65,533 bytes uses the same form is
not known (R-031; hypothesis worth testing first, because the marker `fc ff` lies in the range
0xfff0 to 0xfffd that no column field uses).

### 5.3 NULL patterns (`t_nullmix`: ID NUMBER(2), N NUMBER, S VARCHAR2(10), D DATE)

All eight patterns of (N, S, D); `v` = value, `-` = NULL. Row bytes of `t_nullmix` (conventional):

| truth row | N S D | row offset | bytes after the ID field | `t_nullmix/direct` row offset |
|---|---|---|---|---|
| 1 | v v v | 0x228c | `02 00 c1 0c` `04 00 SSSS` `07 00 78 78 02 1d 01 01 01` `00 00` | 0x228c |
| 2 | - v v | 0x22a5 | `fe ff` `04 00 SSSS` `07 00 78 78 02 1d 01 01 01` `00 00` | 0x22a3 |
| 3 | v - v | 0x22bc | `02 00 c1 0c` `fe ff` `07 00 78 78 02 1d 01 01 01` `00 00` | 0x22b8 |
| 4 | - - v | 0x22d1 | `fe ff` `fe ff` `07 00 78 78 02 1d 01 01 01` `00 00` | 0x22cb |
| 5 | v v - | 0x22e4 | `02 00 c1 0c` `04 00 SSSS` `fe ff` `00 00` | 0x22dc |
| 6 | - v - | 0x22f6 | `fe ff` `04 00 SSSS` `fe ff` `00 00` | 0x22ec |
| 7 | v - - | 0x2306 | `02 00 c1 0c` `fe ff` `fe ff` `00 00` | 0x22fa |
| 8 | - - - | 0x2314 | `fe ff` `fe ff` `fe ff` `00 00` | 0x2306 |

The direct file has the same bytes without the `00 00`. All 24 fields equal `truth_nullmix.txt`
on both paths. Status: confirmed. The session 1 rule holds unchanged: NULL is `fe ff` for every
type present, in every position, and trailing NULLs are written.

### 5.4 Table without rows (`t_empty`, `multi_empty`, `multi_empty2`)

T_EMPTY (ID NUMBER, NAME VARCHAR2(20), D DATE; 0 rows in `truth_counts.txt` and in every log) is
exported last (`t_empty`), between two tables (`multi_empty`: T_FRAME, T_EMPTY, T_LEN) and first
(`multi_empty2`: T_EMPTY, T_FRAME), on both paths. Its table section is the same 256 bytes
(conventional) or 254 bytes (direct) in all three positions.

| file | `TABLE` line | column count | `00 00` + path flag | where the first row would start | bytes there | next item |
|---|---|---|---|---|---|---|
| `t_empty` | 0x210d | 0x21f4 `03 00` | 0x2206 `00 00 00 00` | 0x220a | `ff ff 0a` | 0x220d `METRICSTreferential ...` (trailer) |
| `multi_empty` | 0x22f6 | 0x23dd `03 00` | 0x23ef `00 00 00 00` | 0x23f3 | `ff ff 0a` | 0x23f6 `TABLE "T_LEN"` |
| `multi_empty2` | 0x210d | 0x21f4 `03 00` | 0x2206 `00 00 00 00` | 0x220a | `ff ff 0a` | 0x220d `TABLE "T_FRAME"` |
| `t_empty/direct` | 0x210d | 0x21f4 `03 00` | 0x2206 `00 00 01 00` | 0x220a | `0a` | 0x220b `METRICSTreferential ...` (trailer) |
| `multi_empty/direct` | 0x22e8 | 0x23cf `03 00` | 0x23e1 `00 00 01 00` | 0x23e5 | `0a` | 0x23e6 `TABLE "T_LEN"` |
| `multi_empty2/direct` | 0x210d | 0x21f4 `03 00` | 0x2206 `00 00 01 00` | 0x220a | `0a` | 0x220b `TABLE "T_FRAME"` |

| statement | evidence | status |
|---|---|---|
| Conventional path, no rows: `ff ff` stands where the first row would start, then LF. The same two markers as after the last row of a table with rows | `t_empty`@0x220a, `multi_empty`@0x23f3, `multi_empty2`@0x220a | confirmed (3 files, `truth_counts.txt` T_EMPTY 0) |
| Direct path, no rows: **no `ff ff`**. The LF follows the path flag at once, and then the next item | `t_empty/direct`@0x220a `0a 4d` (`\nM`), `multi_empty/direct`@0x23e5 `0a 54` (`\nT`), `multi_empty2/direct`@0x220a `0a 54` | confirmed (3 files) |
| Nothing else marks the empty table: no count, no extra bytes; the next `TABLE` line or the trailer follows as after any table | table above | confirmed |
| The tables after an empty table are not affected: their sections are byte-identical to the single-table exports | `multi_empty2/direct`@0x220b-0x23e5 = `t_frame/direct`@0x210d-0x22e7 (475 bytes); `multi_empty/direct`@0x23e6-0x352c = `t_len/direct`@0x210d-0x3253 (4423 bytes); section 4 | confirmed |
| On the direct path a table **with** rows always ends in `ff ff` + LF | every direct table with rows in the 22 direct files (28 tables), e.g. `multi_empty/direct`@0x22e5, 0x352a | confirmed |

**How a reader decides "this table has no rows".** Let p be the offset after the path flag.

- Conventional path (flag 0): the u16 at p is `ff ff`. This is the ordinary end marker; no
  special case is needed. Status: confirmed.
- Direct path (flag 1): the u16 at p is not `ff ff`, so a rule is needed. Two rules give the
  right answer for all 31 direct-path tables of the fixtures (28 with rows: "has rows"; 3 empty:
  "no rows"; `tools/check_all.py` evaluates both for every table, 0 disagreements):

| rule | statement | why it works in the fixtures | where it could fail |
|---|---|---|---|
| E1 (declared length; the session 2 heuristic) | no rows if the u16 at p is neither `fe ff` nor `ff ff` and is larger than the declared length of column 1 | at p stand `0a 4d` = 19722 or `0a 54` = 21514; the largest declared length in the fixtures is 2000 | a first column declared with 19722 bytes or more (not possible for the types of the fixtures) |
| E2 (text follows) | no rows if the byte at p is `0a` and the byte at p + 1 is an upper-case ASCII letter (0x41-0x5a) | an item after a table always starts with an upper-case letter in the fixtures (`TABLE`, `METRICST`, and in `ddl` `ALTER`, `CREATE`) | a first field of 16650 to 23050 bytes whose length has the low byte 0x0a; or an item after an empty table that does not start with an upper-case letter |

Both rules are stated for what the fixtures contain; neither is proven to be what Oracle does.
They would be separated by an empty table, on the direct path, (a) followed by statements of its
own (index, constraint, comment; R-012 contains one), which tests "LF, then an item", and (b)
with a first column longer than 16649 bytes, which is outside phase 1. `tools/exp_parse.py`
uses E2 by default (`--empty=declared` selects E1).

Not seen (session 3): an empty table whose section has statements after the data (T_EMPTY has no
index, no constraint, no comment).

#### Session 4: an empty table with its own statements; Oracle 11g; a table cut short

O_EMPTY (ID NUMBER(6) with a named primary key, NAME VARCHAR2(20), an index, a table comment; 0
rows in `truth_own.txt`) in `own` and `own_tables`, both paths, both versions. p = the offset
after the path flag.

| file | `00 00` + path flag | bytes at p | next item | rule E1 / E2 say "no rows" |
|---|---|---|---|---|
| `own` (18c, conv) | 0x2f73 `00 00 00 00` | 0x2f77 `ff ff 0a 43` | 0x2f7a `CREATE UNIQUE INDEX "O_EMPTY_PK" ...` | not needed (`ff ff`) |
| `own_tables` (18c, conv) | 0x2ee7 `00 00 00 00` | 0x2eeb `ff ff 0a 43` | 0x2eee `CREATE UNIQUE INDEX ...` | not needed |
| `own/direct` (18c) | 0x2f6d `00 00 01 00` | 0x2f71 `0a 43 52 45` (**no `ff ff`**) | 0x2f72 `CREATE UNIQUE INDEX ...` | yes / yes |
| `own_tables/direct` (18c) | 0x2edd `00 00 01 00` | 0x2ee1 `0a 43 52 45` | 0x2ee2 `CREATE UNIQUE INDEX ...` | yes / yes |
| `own_11g` (conv) | 0x2fdb `00 00 00 00` | 0x2fdf `ff ff 0a 43` | 0x2fe2 `CREATE UNIQUE INDEX ...` | not needed |
| `own_11g/direct` | 0x2fd5 `00 00 01 00` | 0x2fd9 **`ff ff 0a 43`** | 0x2fdc `CREATE UNIQUE INDEX ...` | not needed (`ff ff`) |
| `own_tables_exp_direct_11g.dmp` | 0x2f40 `00 00 01 00` | 0x2f44 **`ff ff 0a 43`** | 0x2f47 `CREATE UNIQUE INDEX ...` | not needed |

After the marker (or, on the 18c direct path, after the LF) the statements of the table follow as
after any table: two `CREATE INDEX` lines, `ANALSTATS` + field, the primary key line, `COMMENT ON
TABLE` + field (walk in 4.5, items 52 to 58).

| statement | evidence | status |
|---|---|---|
| Where `ff ff` would stand on the 18c direct path there is the LF, and then the first statement of the table. Rules E1 and E2 both still say "no rows" (`0a 43` = 17162 is larger than the declared 22; LF followed by `C`) | `own/direct`@0x2f71, `own_tables/direct`@0x2ee1; `tools/check_all.py`: 0 disagreements between E1 and E2 in all 133 files | confirmed (18c). The two rules are still not separated: every item that follows an empty table in the fixtures starts with an upper-case letter (`TABLE`, `METRICS`, `CREATE`, `ENDTABLE`), and no first column is longer than 4000 bytes |
| **Correction of section 5.4 / 8 as written in session 3:** "direct path, no rows: no `ff ff`" is true for the 18c fixtures only. On 11g the direct path writes `ff ff` + LF for a table without rows, like the conventional path | 5 files: `t_empty_exp_direct_11g.dmp`@0x226d, `multi_empty_exp_direct_11g.dmp`@0x2448, `multi_empty2_exp_direct_11g.dmp`@0x226d, `own_11g/direct`@0x2fd9, `own_tables_exp_direct_11g.dmp`@0x2f44; section 13.2 | confirmed for 11g |
| In the 11 direct-path cases, `ff ff` is absent exactly when the `CREATE TABLE` text has no `STORAGE(...)` clause | section 13.2 | observed. Hypothesis: the marker is missing when the table has no segment, not because of the version (R-017). **Session 17: confirmed on 18c** (`seg_exp_direct_18c.dmp`, see below) |
| A table whose export failed has the bytes of an empty table with `ff ff`, on both paths, and no statements after it | `ru_exp_us7ascii_11g.dmp`@0x13746; `ru_exp_direct_us7ascii_11g.dmp`@0x14cdc; section 14.3 | confirmed (3 files) |

**Rule a reader uses** (replaces the choice between E1 and E2 as the main question). Let p be the
offset after the path flag, on either path:

1. if the u16 at p is `ff ff`: no rows; skip it and one LF;
2. else, if the path flag is 1 and the byte at p is LF and the bytes at p + 1 start a line the
   reader knows as an item start (`TABLE "`, `ENDTABLE`, `METRICS`, `CREATE `, `ALTER `, `COMMENT `,
   `GRANT `, `ANALSTATS `, `CALTER `): no rows; skip the LF;
3. else read rows until `ff ff`.

Step 2 is rule E2 with the list of item starts made explicit; it is needed for 18c and was never
needed on 11g. Status: confirmed for all 202 table sections of the 133 files; step 2 is a
heuristic (a first field of exactly 0x..0a bytes whose value starts like a statement would be
misread; such a value has at least 10 and at most 65290 bytes with low length byte 0x0a).

#### Session 9: an empty LONG table and empty subpartitions on the direct path (R-031, R-037)

| file | binary part | p | bytes at p | next item | E2 (text) / (declared length) | 11g |
|---|---|---|---|---|---|---|
| `long_exp_direct_18c` | T_LONG_EMPTY @0x11ed0 (`02 00`, V `01 00 0a 00 69 03 01 00`, L `08 00 00 00`, `00 00 01 00`) | 0x11ee2 | `0a 54` | 0x11ee3 `TABLE "T_FRAME"` | yes / yes (21514 > 10: column 1 is V, length 10) | `long_exp_direct_11g`@0x11f47 `ff ff 0a` (E1) |
| `part2_exp_direct_18c` | SYS_SUBP370 @0x284f | 0x2865 | `0a 53` | 0x2866 `SUBPARTITION "SYS_SUBP371"` | yes / yes (21258 > 22) | - |
| `part2_exp_direct_18c` | SYS_SUBP371 @0x28bc | 0x28d2 | `0a 45` | 0x28d3 `ENDCOMPARTITION` | yes / yes (17674 > 22) | - |

| statement | evidence | status |
|---|---|---|
| The 18c direct path writes no `ff ff` for these three empty binary parts either; 11g writes `ff ff` (T_LONG_EMPTY). The two tests of E2 agree on all 9 empty direct-path parts of 18c and on every direct-path part with rows (`tools/check_all.py`: `empty_rule_diff` 0, 199 files) | table | confirmed (18c) |
| The item starts seen after p are now `TABLE "`, `METRICS`, `CREATE `, `ANALSTATS `, **`SUBPARTITION "`**, **`ENDCOMPARTITION`**; a reader's list for test (text) needs every line kind that can follow a binary part inside a section, including `PARTITION "`, `ENDPARTITION`, `COMPARTITION "`, `ENDCOMPARTITION`, `SUBPARTITION "` | table; 4.8 | confirmed for the two new starts; `PARTITION "` / `ENDPARTITION` / `COMPARTITION "` after an empty direct part: not in the fixtures, included because they can follow a part (4.8) |
| T_LONG_EMPTY has no `STORAGE(...)` clause in its 18c `CREATE TABLE` line (`long_exp_direct_18c`@0x11e0c), like every other 18c table without `ff ff`; the 11g line has one | both files | observed (fits the R-017 hypothesis "no segment, no `ff ff`") |
| **Note A-7 (a LONG first column on the direct path) does not arise in these files**: on the direct path the LONG column is moved to the end (4.1), so column 1 of T_LONG_EMPTY is V (length 10) and test (declared length) works. It would arise only for a table whose **only** column is LONG or LONG RAW (Oracle allows one LONG column per table, assumed): then column 1 has length 0 and rule E2a applies | `long_exp_direct_<v>` T_LONG_EMPTY descriptors @0x11ed2 / 0x11f37 | confirmed for the move; E2a for a single-column LONG table: still suggested, not in the fixtures |

**Session 10, note A-19 (one list of item starts).** E2 test (a) and E2a name item starts in two lists that differ
only by the partition lines. Observed after an empty direct-path binary part: `METRICS`, `TABLE "`, `CREATE `,
`ANALSTATS ` (sessions 3 to 7), `TABLE "` after the empty LONG table (`long_exp_direct_18c.dmp`@0x11ee2 `0a 54`),
`SUBPARTITION "` (`part2_exp_direct_18c.dmp`@0x2865 `0a 53`) and `ENDCOMPARTITION` (@0x28d2 `0a 45`). `PARTITION "`,
`COMPARTITION "` and `ENDPARTITION` can follow a binary part by T10 (`part_exp_18c`, `part2_exp_18c`: always after a part
that ends with `ff ff` LF there). There is no reason in the files for two lists; the difference was editorial. Version 3.5
of the rules uses **one** list for E2 (a) and E2a: `TABLE "`, `ENDTABLE`, `METRICS`, `CREATE `, `ALTER `, `COMMENT ON `,
`GRANT `, `ANALSTATS `, `CALTER TABLE`, `PARTITION "`, `COMPARTITION "`, `SUBPARTITION "`, `ENDCOMPARTITION`,
`ENDPARTITION` (the five partition lines: seen two of them after p, three by T10 only). A single-column LONG table that is
partitioned cannot exist on 18c: ORA-14085 for LONG and LONG RAW (session 11, R-045, `18c/truth_r045.txt` lines 13, 19,
28); so for E2a the partition lines are reachable only
through a non-LONG first column, where E2 (b) decides anyway.

#### Session 17: an empty table with and without a segment on the 18c direct path (R-017)

`seg_exp_18c.dmp` (conventional) and `seg_exp_direct_18c.dmp` (direct), Oracle 18c, table mode, `tables=(t_seg0,t_seg1,
t_seg2)`, 0 rows each (logs). `truth_seg.txt`: T_SEG0 `segment_created=YES|segments=1` (`SEGMENT CREATION IMMEDIATE`),
T_SEG1 `segment_created=YES|segments=1` (a row inserted and deleted), T_SEG2 `segment_created=NO|segments=0`
(`SEGMENT CREATION DEFERRED`). The three tables have the same columns as T_EMPTY. p = the offset after the path flag.

| table | `CREATE TABLE` line (both files) | `STORAGE(...)` | path flag (direct) | bytes at p, direct | bytes at p, conventional | segment (truth) |
|---|---|---|---|---|---|---|
| T_SEG0 | @0x211c, 250 bytes | yes: `STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 FREELISTS 1 FREELIST GROUPS 1 BUFFER_POOL DEFAULT)` | @0x2268 `01 00` | @0x226a **`ff ff 0a`** | @0x226a `ff ff 0a` | YES, 1 |
| T_SEG1 | @0x227c, 250 bytes | yes (same clause) | @0x23c8 `01 00` | @0x23ca **`ff ff 0a`** | @0x23ca `ff ff 0a` | YES, 1 |
| T_SEG2 | @0x23dc, 151 bytes | **no** (`... MAXTRANS 255 TABLESPACE "USERS" LOGGING NOCOMPRESS`) | @0x24c5 `01 00` | @0x24c7 **`0a 4d`** (LF, then `METRICST...`) | @0x24c7 `ff ff 0a` | NO, 0 |

`cmp -l seg_exp_18c.dmp seg_exp_direct_18c.dmp` from 0x2000 on: the three path flags (0x2268, 0x23c8, 0x24c5) and,
from 0x24c7, the shift by the two bytes `ff ff` the direct file lacks; nothing else.

| statement | evidence | status |
|---|---|---|
| The prediction of R-017 holds: on the 18c direct path, T_SEG0 and T_SEG1 (empty, with a segment) carry a `STORAGE` clause and `ff ff 0a` follows their path flag; T_SEG2 (empty, no segment) has no clause and `0a` follows its path flag | table; `truth_seg.txt` | confirmed (18c; 3 tables in one file, the two segment cases made in two different ways) |
| **The hypothesis "the marker depends on the segment, not on the version" is confirmed on 18c**: the same 18c `exp` writes `ff ff` for an empty table with a segment and nothing for one without. The 18c / 11g difference of 13.2 is explained by the segment: every 11g empty table of the fixtures has a `STORAGE` clause (a segment, assumed from the clause), every earlier 18c one lacks it | table; 13.2 | confirmed for 18c. On 11g a table without a segment is not in the fixtures, so "11g behaves the same" is a hypothesis (R-049) |
| The conventional path writes `ff ff 0a` for all three, segment or not | `seg_exp_18c.dmp`@0x226a, 0x23ca, 0x24c7 | confirmed |
| The `STORAGE` clause of the `CREATE TABLE` line and the presence of `ff ff` on the direct path agree in every empty direct-path table of the corpus (the cases of 13.2, T_STAT4 of session 5, T_LONG_EMPTY of session 9, and these 3); empty subpartitions (session 9) have no `CREATE TABLE` line of their own and are not counted | 13.2; 5.4 session 9; table | observed. A reader must not use the clause as the test (it is DDL text; rules E1 / E2 decide from the bytes) |

#### Session 19: 11g XE cannot create a table without a segment (R-049, negative)

`11g/truth_r049.txt` (Oracle Database 11g Express Edition Release 11.2.0.2.0, line 5; a SQL*Plus record with echo, no
dump made) holds the R-017 statements for T_SEG0 and T_SEG2:

| line | statement | result | status |
|---|---|---|---|
| 24-26 | `CREATE TABLE t_seg0 (...) SEGMENT CREATION IMMEDIATE` | `Table created.` | observed |
| 28-32 | `CREATE TABLE t_seg2 (...) SEGMENT CREATION DEFERRED` | `ORA-00439: feature not enabled: Deferred Segment Creation` | observed |
| 35-43 | `SELECT table_name, segment_created FROM user_tables WHERE table_name LIKE 'T_SEG_'` | one row, `T_SEG0 YES` | observed |

(Lines 9-15 drop a T_SEG0 and a T_SEG1 that existed before the record, assumed from an earlier run of the R-017 SQL that
stopped at the same error; T_SEG1 was not created again, which does not matter for the answer.)

| statement | evidence | status |
|---|---|---|
| 11g XE 11.2.0.2 refuses deferred segment creation, so a table without a segment cannot be made there, and the R-049 dumps cannot exist | `11g/truth_r049.txt` lines 28-32 | observed (one database) |
| Every 11g XE table has a segment from `CREATE TABLE` on | follows from the error if XE has no other way to a segment-less table | assumed |
| What this means for the corpus: every 11g empty table of the fixtures has a segment (it shows a `STORAGE` clause, 13.2), and the 11g direct path wrote `ff ff` for every one of them; that is what "the marker follows the segment" (confirmed on 18c, `seg_exp_direct_18c.dmp`) predicts for 11g XE | 13.2; this table | consistent with the hypothesis; not a test of it (no 11g table without a segment exists) |
| What it means for a real 11g **Enterprise Edition** dump: deferred segment creation exists there (assumed from Oracle documentation, not from a fixture; the error text "feature not enabled" names an edition feature). An empty EE table created without a segment may then, on the 11g direct path, have no `ff ff` as on 18c. Whether it does: unknown | - | hypothesis (untestable with the XE containers) |
| The reader rule does not depend on it: E1 / E2 / E3 accept both forms (`ff ff`, or LF + an item start) after a path flag of 1 on either version and never decide by the version line | rules E1, E2, E3 (section 11) | confirmed for the forms seen; no change |

### 5.5 Order of rows and of tables

| statement | evidence | status |
|---|---|---|
| Tables are written in the order of the log lines ". . exporting table" | `multi_framing`@0x210d T_FRAME, @0x22f6 T_LEN, @0x3469 T_NULLMIX (`multi_framing_exp_18c.log`); `multi_types`@0x210d T_NUM, @0x2458 T_DATE, @0x2633 T_TS, @0x2890 T_TEXT; same order in both direct files | confirmed. Whether this is the order of the `tables=` argument was not checked |
| Rows are **not** in `ORDER BY` order, and not necessarily in the order in which the INSERT produced them (`corpus/sql/04_probes.sql`: one `INSERT ... SELECT ... CONNECT BY LEVEL <= 300`) | `t_probe6`@0x225e: first row is `R00065`; dump order is R00065..R00300, then R00001..R00064 (row R00001 at `t_probe6`@0x899e, `t_probe6/direct`@0x87c6). `truth_probe6.txt` is ordered by id | confirmed (both paths). All 300 rows are present, each once |
| `t_len`: dump order is the insertion order of `corpus/sql/01_corpus_tables.sql` (id 0 last), not the id order of `truth_len.txt` | `t_len`@0x3276 (row id 0 is the last row) | observed |

A reader must not assume any row order; compare with truth as a set or by key.

### 5.6 Length values that never occur in row data (session 7, note A-4)

`tools/rowlen_stats.py` reads the length field of every field of every row of the 307 table
sections without a LOB column in the 169 files (34,069 rows, 219,043 fields, 18,915 of them NULL)
and counts the u16 values.

| statement | evidence | status |
|---|---|---|
| A length of 0 (`00 00` in place of a length) never occurs. An empty string is NULL in Oracle and is written `fe ff` (assumed for the cause; observed as absence) | 0 of 219,043 fields; smallest length per type: 1 (types 1, 2, 23), 2 (24), 4 (96), 5 (8), 7 (12, 180), 11 (231), 13 (181) | confirmed as absence in the fixtures; what a reader should do with a length 0 is not shown by any file |
| `fd ff` (0xfffd), and every other value from 0xfff0 to 0xfffd, never occurs as a length field. The only values above 32,000 are `fe ff` (NULL) and `ff ff` (end of rows) | 0 of 219,043 fields; largest length per type: 3000 (1), 21 (2), 32,000 (8, 24), 7 (12), 2000 (23), 12 (96), 11 (180, 231), 13 (181) | confirmed as absence in the fixtures |
| In the text region `fd ff` marks a field in pieces (rule I4); in row data it does not occur, so nothing says it would mean the same there | 4.2; this table | the meaning of `fd ff` in row data: unknown |

**Session 8.** `tools/rowlen_stats.py` now reads the LOB tables too (their column fields with the
locators, 14.6) and the new types: **237,347 fields** in column positions of 338 binary parts of
the 177 files (40,172 rows, 18,994 NULL): still **0** of length 0 and **0** from 0xfff0 to 0xfffd.
New smallest / largest lengths per type: 69 (ROWID) 23 / 23, 112 (CLOB / NCLOB locator) 102 / 150,
113 (BLOB locator) 102 / 1102, 182 5 / 5, 183 11 / 11, 208 13 / 13. The largest column field is
still 32,000 bytes.

The **LOB value area** after a row of a LOB table (14.6) is not a column field and does not follow
these statements: there a length 0 occurs (an empty LOB, 12 times: e.g. `lob_exp_18c.dmp`@0x261c,
0x261e, 0x4dda5) and `fc ff` (0xfffc) marks a value in chunks (8 times: e.g. @0x4358, 0x350bc). A
reader applies the statements of this section to the column fields only.

**Session 9.** 199 files, 396 binary parts, 40,588 rows, **238,657 column fields** (19,186 NULL):
still 0 of length 0 and 0 from 0xfff0 to 0xfffd (`tools/rowlen_stats.py`, 0 findings). One new
exception to "a column field is one u16 + bytes": a LONG / LONG RAW field on the **direct path** is a
chain of pieces whose u16 has bit 0x8000 set when another piece follows (6.6): `9c 80` (0x809c),
`19 9f` (0x9f19), `19 1f` (0x1f19) in `long_exp_direct_<v>`. Every other column field of the corpus
has a u16 below 0x8000 or `fe ff` / `ff ff` (`len_8000_up` 0), so the flag cannot be confused with a
length in the fixtures; types 1, 23 and 96 cannot exceed 4000 bytes anyway. Lengths seen per type:
LONG 5 to 32,000, LONG RAW 2 to 32,000 (value lengths of the chains), 96 up to 507 (`CHAR(501 CHAR)`).
The LOB value area: 114 values (74 short, 40 long), 12 of length 0; chunk lengths 8060, 8132,
16120, 16264 (full chunks) and the remainders.

**Session 17 (R-040).** 211 files, 418 binary parts, 41,284 rows, **243,303 column fields** (19,960 NULL;
`tools/rowlen_stats.py`). Still 0 of length 0. The statement "0xfff0 to 0xfffd never occurs" is **refuted for one
case**: a LONG column on the conventional path. There `f0 ff` is a length (65,520 bytes, `long3_exp_<v>.dmp`@0x2228f)
and `fd ff` starts a value in chunks ended by `00 00` (`long3_exp_<v>.dmp`@0x3228f, 0x422c2, 0x522f6; 6.6); lengths
from 0x8000 are ordinary lengths there (`00 80` @0xa280, `ef ff` @0x12290). Outside these 6 conventional LONG fields
no column field of the corpus has a u16 from 0x8000 up other than `fe ff` / `ff ff` (`len_8000_up` 0, `high` 0). On
the direct path a LONG of any size stays a chain whose piece u16 values are below 0xfff0 (largest 0x9f19). The table of
this section therefore holds for every column type except LONG (and, assumed, LONG RAW) on the conventional path; for
0xfff1 to 0xfffc in such a column nothing is known (R-048).

**Session 19 (R-048).** 221 files, 434 binary parts, 41,378 rows, **243,593 column fields** (20,020 NULL;
`tools/rowlen_stats.py`). Still 0 of length 0; `high` 0 and `len_8000_up` 0 outside the conventional LONG / LONG RAW
columns. In those columns the first u16 of a field was, per value (`long3`, `long4`, `longraw4`; 18c and 11g):

| first u16 | meaning | values seen | evidence | status |
|---|---|---|---|---|
| 1 to 0x7fff | length | many | 6.6 | confirmed |
| 0x8000 to 0xfff0 | length (bit 0x8000 part of it) | 0x8000, 0xffef, 0xfff0 (LONG); 0x8000, 0xfff0 (LONG RAW, 18c) | `long3_exp_<v>`@0xa280, 0x12290, 0x2228f; `longraw4_exp_18c`@0x225d, 0xa265 | confirmed (LONG 2 versions; LONG RAW 18c) |
| 0xfff1 to 0xfff5 | length | 0xfff1, 0xfff5 (LONG) | `long4_exp_<v>`@0x2271 `f1 ff`, @0x12272 `f5 ff` | confirmed for the two values; 0xfff2 to 0xfff4 lie between them: hypothesis (length) |
| 0xfff6 to 0xfff9 | ? | none | - | unknown (no value of 65,526 to 65,529 bytes) |
| 0xfffa | marker: `fa ff` + 4 bytes + value (6.6) | 3 values of 65,530, 65,532, 65,533 bytes | `long4_exp_<v>`@0x22277, 0x32285, 0x42295 | confirmed (LONG, 2 versions) |
| 0xfffb, 0xfffc | ? | none as a first u16 (`fc ff` occurs as the length **inside** the `fa ff` form, @0x32287) | - | unknown |
| 0xfffd | marker: `fd ff` + chunks + `00 00` | 65,534 to 100,000 bytes | `long4_exp_<v>`@0x522a6; `long3_exp_<v>`@0x3228f; `longraw4_exp_18c`@0x1a25d | confirmed |
| 0xfffe | NULL | - | 5.3 | confirmed |

So a reader **does** meet 0xfff1 and 0xfff5 as lengths, meets 0xfffa as a marker and never (in these files) meets
0xfff6 to 0xfff9, 0xfffb or 0xfffc as the first u16 of a conventional LONG field. Outside the conventional LONG / LONG
RAW columns every statement of this section stands (0 fields from 0x8000 up other than `fe ff` / `ff ff`).

**Session 20 (R-051).** 229 files (105 / 97 / 27), 450 binary parts, 41,494 rows, **243,905 column fields** (20,084
NULL; `tools/rowlen_stats.py`): still 0 of length 0; `high` 0 and `len_8000_up` 0 outside the conventional LONG / LONG
RAW columns; inside them 28 one-field values from 0x8000, 16 `fa ff` values, 16 chunked values (all decoded to the truth,
6.6 session 20). The first u16 of a conventional LONG / LONG RAW field (replaces the session 19 table):

| first u16 | meaning | values seen | evidence | status |
|---|---|---|---|---|
| 1 to 0x7fff | length | many | 6.6 | confirmed |
| 0x8000 to 0xfff0 | length (bit 0x8000 part of it) | 0x8000, 0xffef, 0xfff0 (LONG); 0x8000, 0xfff0 (LONG RAW, 18c and 11g) | `long3_exp_<v>`@0xa280, 0x12290, 0x2228f; `longraw4_exp_18c`@0x225d, 0xa265; `longraw5_exp_11g`@0x72294, 0x7a29c | confirmed |
| 0xfff1 | length | LONG, LONG RAW | `long4_exp_<v>`@0x2271; `longraw5_exp_<v>`@0x225d | confirmed (2 types, 2 versions) |
| 0xfff2 to 0xfff4 | ? | none | - | hypothesis: lengths (sizes 65,522 to 65,524 not in the fixtures; one field on both sides) |
| 0xfff5 | length | LONG, LONG RAW | `long4_exp_<v>`@0x12272; `longraw5_exp_<v>`@0x12256 | confirmed (2 types, 2 versions) |
| 0xfff6 to 0xfff8 | length | LONG | `long5_exp_<v>`@0x2271, 0x12277, 0x2227e | confirmed (LONG, 2 versions); LONG RAW: hypothesis (same form) |
| 0xfff9 | length (the largest one-field length) | LONG, LONG RAW | `long5_exp_<v>`@0x32286; `longraw5_exp_<v>`@0x22253 | confirmed (2 types, 2 versions) |
| 0xfffa | marker: `fa ff` + u16 n + `00 00` + n bytes (n 0xfffa to 0xfffd) | 65,530 to 65,533 (LONG); 65,530, 65,531, 65,533 (LONG RAW) | `long4_exp_<v>`@0x22277, 0x32285, 0x42295; `long5_exp_<v>`@0x4228f, 0x5229d; `longraw5_exp_<v>`@0x32254, 0x4225a, 0x52261 | confirmed (2 types, 2 versions) |
| 0xfffb, 0xfffc | ? | never as a first u16 (they occur as the n **inside** the `fa ff` form: @0x5229f `long5`, @0x32287 `long4`, @0x4225c `longraw5`) | - | unknown as a first u16; the sizes 65,531 / 65,532 are written `fa ff` + n (6.6 session 20) |
| 0xfffd | marker: `fd ff` + chunks + `00 00` | 65,534 to 100,000 bytes | `long4_exp_<v>`@0x522a6; `long3_exp_<v>`@0x3228f; `longraw5_exp_<v>`@0x6226a; `longraw4_exp_18c`@0x1a25d | confirmed (2 types, 2 versions) |
| 0xfffe | NULL | - | 5.3 | confirmed |

So a reader meets 0xfff0, 0xfff1 and 0xfff5 to 0xfff9 as lengths, 0xfffa and 0xfffd as markers, and never (in these
files) 0, 0xfff2 to 0xfff4, 0xfffb or 0xfffc as the first u16 of a conventional LONG / LONG RAW field.

## 6. Value encodings

Result of `tools/check_truth.py` on all 18c EXP fixtures: every compared field equals the truth
(0 differences), on both paths, alone and in the `multi_*` files.

### 6.1 NUMBER (`t_num`: ID NUMBER(3), V NUMBER)

Value bytes of column V = the bytes of `DUMP(v,16)` in `truth_num.txt`, for all 33 rows, on both
paths. The dump adds only the u16 LE length in front; there is no further length byte inside the
value, and the length equals `Len=` of the truth.

| truth row | value | length field | value bytes in the dump | `t_num` offset | `t_num/direct` offset | vs `DUMP(v,16)` |
|---|---|---|---|---|---|---|
| 1 | 0 | `01 00` | `80` | 0x224a | 0x224a | equal |
| 2 | 1 | `02 00` | `c1 02` | 0x2253 | 0x2251 | equal |
| 3 | -1 | `03 00` | `3e 64 66` | 0x225d | 0x2259 | equal |
| 4 | 5 | `02 00` | `c1 06` | 0x2268 | 0x2262 | equal |
| 5 | -5 | `03 00` | `3e 60 66` | 0x2272 | 0x226a | equal |
| 6 | 10 | `02 00` | `c1 0b` | 0x227d | 0x2273 | equal |
| 7 | 100 | `02 00` | `c2 02` | 0x2287 | 0x227b | equal |
| 8 | 123 | `03 00` | `c2 02 18` | 0x2291 | 0x2283 | equal |
| 9 | -123 | `04 00` | `3d 64 4e 66` | 0x229c | 0x228c | equal |
| 10 | 100000 | `02 00` | `c3 0b` | 0x22a8 | 0x2296 | equal |
| 11 | 1234567 | `05 00` | `c4 02 18 2e 44` | 0x22b2 | 0x229e | equal |
| 12 | .5 | `02 00` | `c0 33` | 0x22bf | 0x22a9 | equal |
| 13 | .05 | `02 00` | `c0 06` | 0x22c9 | 0x22b1 | equal |
| 14 | -.5 | `03 00` | `3f 33 66` | 0x22d3 | 0x22b9 | equal |
| 15 | .1 | `02 00` | `c0 0b` | 0x22de | 0x22c2 | equal |
| 16 | .01 | `02 00` | `c0 02` | 0x22e8 | 0x22ca | equal |
| 17 | 123.456 | `05 00` | `c2 02 18 2e 3d` | 0x22f2 | 0x22d2 | equal |
| 18 | -123.456 | `06 00` | `3d 64 4e 38 29 66` | 0x22ff | 0x22dd | equal |
| 19 | 10^33 | `02 00` | `d1 0b` | 0x230d | 0x22e9 | equal |
| 20 | 10^33 + 1 | `12 00` | `d1 0b 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 02` | 0x2317 | 0x22f1 | equal |
| 21 | 12345678901234567890123456789012345678 | `14 00` | `d3 0d 23 39 4f 5b 0d 23 39 4f 5b 0d 23 39 4f 5b 0d 23 39 4f` | 0x2331 | 0x2309 | equal |
| 22 | -12345678901234567890123456789012345678 | `15 00` | `2c 59 43 2d 17 0b 59 43 2d 17 0b 59 43 2d 17 0b 59 43 2d 17 66` | 0x234d | 0x2323 | equal |
| 23 | 1234567890123456789012345678901234567.8 | `15 00` | `d3 02 18 2e 44 5a 02 18 2e 44 5a 02 18 2e 44 5a 02 18 2e 44 51` | 0x236a | 0x233e | equal |
| 24 | 99999999999999999999999999999999999999 | `14 00` | `d3` + 19 x `64` | 0x2387 | 0x2359 | equal |
| 25 | .12345678901234567890123456789012345678 | `14 00` | `c0 0d 23 39 4f 5b 0d 23 39 4f 5b 0d 23 39 4f 5b 0d 23 39 4f` | 0x23a3 | 0x2373 | equal |
| 26 | 1E+125 | `02 00` | `ff 0b` | 0x23bf | 0x238d | equal |
| 27 | 1E-125 | `02 00` | `82 0b` | 0x23c9 | 0x2395 | equal |
| 28 | 9.99...9E+125 (38 nines) | `14 00` | `ff` + 19 x `64` | 0x23d3 | 0x239d | equal |
| 29 | -1E+125 | `03 00` | `00 5b 66` | 0x23ef | 0x23b7 | equal |
| 30 | 3.1415926535897932384626433832795028841 | `15 00` | `c1 04 0f 10 5d 42 24 5a 50 21 27 2f 1b 2c 27 21 50 33 1d 55 0b` | 0x23fa | 0x23c0 | equal |
| 31 | .3333333333333333333333333333333333333333 | `15 00` | `c0` + 20 x `22` | 0x2417 | 0x23db | equal |
| 32 | -1234567890123456789012345678901234567.8 | `15 00` | `2c 64 4e 38 22 0c 64 4e 38 22 0c 64 4e 38 22 0c 64 4e 38 22 15` | 0x2434 | 0x23f6 | equal |
| 33 | NULL | `fe ff` | (none) | 0x2451 | 0x2411 | NULL = NULL |

Status: confirmed (33 rows x 2 paths; also in `multi_types` and `multi_types/direct`).

Rows with 20 mantissa bytes (21 bytes in all): 23, 30, 31 (positive) and 32 (negative).
Row 32 has 20 mantissa bytes and **no** trailing `66`; row 22 (negative, 19 mantissa bytes, 21
bytes in all) ends in `66`. Both are as in the truth file. The largest length seen is 21, below
the declared 22 of the descriptor.

Also equal to truth: `t_numtypes` columns N_P10S2 and N_NEG (10 fields per path; the truth file
has `DUMP` for these two columns only). `FLOAT(24)` has type code 2, length 22
(`t_numtypes`@0x22d2).

How to turn the bytes into a decimal number was **not** part of this document's evidence up to
session 6 (the truth pairs above were kept as test vectors). Session 7 states and tests it below.

#### Bytes to a decimal number (session 7, R-024)

`t_num2_exp_<v>.dmp`: T_NUM2 (`ID NUMBER(3)`, `V NUMBER`), 14 rows, conventional path. The value
bytes of V equal `DUMP(v,16)` of `truth_num2.txt` in all 14 rows on both versions
(`tools/check_truth.py`: 28 fields OK per file, 0 DIFF). Offsets are those of the length field
of V, the same in the 18c and the 11g file.

| id | value (`TO_CHAR(v,'TM9')`) | value bytes in the dump | offset | Architect's hypothesis (R-024) |
|---|---|---|---|---|
| 1 | 1000000 | `c4 02` | 0x224d | confirmed |
| 2 | -1000000 | `3b 64 66` | 0x2257 | confirmed |
| 3 | .001 | `bf 0b` | 0x2262 | confirmed |
| 4 | -.001 | `40 5b 66` | 0x226c | confirmed |
| 5 | 99 | `c1 64` | 0x2277 | confirmed |
| 6 | -99 | `3e 02 66` | 0x2281 | confirmed |
| 7 | -100 | `3d 64 66` | 0x228c | confirmed |
| 8 | 999999999999999999 | `c9` + 9 x `64` (10 bytes) | 0x2297 | confirmed |
| 9 | 1000000000000000000 | `ca 02` | 0x22a9 | confirmed |
| 10 | 9223372036854775807 | `ca 0a 17 22 49 04 45 37 4e 3b 08` | 0x22b3 | confirmed |
| 11 | 9223372036854775808 | `ca 0a 17 22 49 04 45 37 4e 3b 09` | 0x22c6 | confirmed |
| 12 | -9223372036854775808 | `35 5c 4f 44 1d 62 21 2f 18 2b 5d 66` (12 bytes) | 0x22d9 | confirmed |
| 13 | 1E-130 | `80 02` | 0x22ed | confirmed |
| 14 | .00000000001 | `bb 0b` | 0x22f7 | confirmed |

All 14 byte strings of the hypothesis are confirmed; none is refuted.

**Rule (decoding).** Let b[0..L-1] be the value bytes (L = the length field).

| part | statement | evidence | status |
|---|---|---|---|
| length | 1 to 21 bytes (the descriptor says 22) | longest: `t_num`@0x2331 (20), 0x234d, 0x236a, 0x23fa, 0x2417, 0x2434 (21) | confirmed |
| zero | L = 1 and b[0] = `80` | `t_num`@0x224a (truth row 1) | confirmed |
| sign | bit 0x80 of b[0] set: positive; clear: negative | every value of the truth files | confirmed |
| exponent, positive | E = b[0] - 0xC1, the power of 100 of the first digit: value = sum of d[i] x 100^(E - i) | `c1 02` = 1, `c2 02` = 100, `c0 33` = 0.5, `bf 0b` = 0.001, `ff 0b` = 1E+125, `80 02` = 1E-130 | confirmed |
| exponent, negative | E = 0x3E - b[0] (b[0] is the one's complement of the positive exponent byte) | `3e 64 66` = -1, `3d 64 66` = -100, `40 5b 66` = -0.001, `00 5b 66` = -1E+125 | confirmed |
| digits, positive | b[1..L-1], each d = byte - 1, one base-100 digit (00..99) per byte, most significant first; bytes `01` to `64` | `c1 64` = 99, `c9` + 9 x `64` | confirmed |
| digits, negative | d = 101 - byte (`65` = 0 .. `02` = 99), most significant first; the mantissa ends at a byte `66` if there is one | `3e 02 66` = -99, `35 5c ... 5d 66` | confirmed |
| closing `66` | a negative number ends with `66` when it has fewer than 20 digit bytes; with 20 digit bytes (L = 21) it has none. `66` cannot be a digit byte (digit bytes are `02` to `65`), so "drop a last byte `66`" is unambiguous | `t_num`@0x2434 (row 32, 21 bytes, no `66`); @0x234d (row 22, 21 bytes, 19 digits + `66`) | confirmed |
| canonical form | the first and the last digit are never 0 (trailing zero pairs are dropped: 10^18 is `ca 02`, 2 bytes); every value has exactly one byte string | `tools/num_decode.py`: encode(decode(b)) = b for all 72,928 NUMBER fields of the 169 files | confirmed |
| limits | at most 20 digit bytes = 40 decimal digits (row 31 of `t_num`: 40 threes); E from -65 (1E-130, `80 02`) to 62 (1E+125, 9.99..E+125, `ff ...`) in the fixtures | `t_num`, `t_num2` | observed (the bounds of Oracle's type are assumed, not tested: these are the extreme values that were inserted) |

Procedure (pseudo-code; `tools/num_decode.py` implements it as a test, not as product code):

    if L == 1 and b[0] == 0x80: return 0
    if L == 1: error                       # a single byte other than 80: not in the fixtures
    if b[0] & 0x80:                        # positive
        E = b[0] - 0xC1;  D = [x - 1 for x in b[1:]]          # each x must be 0x01..0x64
    else:                                  # negative
        E = 0x3E - b[0];  m = b[1:]
        if m[-1] == 0x66: m = m[:-1]       # else L must be 21
        D = [101 - x for x in m]           # each x must be 0x02..0x65
    require D[0] != 0 and D[-1] != 0
    value = sign * sum(D[i] * 100^(E - i) for i in 0..len(D)-1)
    # decimal digits: 2 per element of D (the first may have a leading 0, the last a trailing 0);
    # decimal exponent of the last digit pair: 2 * (E - len(D) + 1)

Test (`tools/num_decode.py`, Python `decimal` with 60 digits): (1) the 108 non-NULL values of
`truth_num.txt`, `truth_num2.txt` (both folders) and the two DUMP columns of `truth_numtypes.txt`:
decode(DUMP bytes) equals the decimal text exactly, and encoding the decimal text gives the DUMP
bytes back; (2) all 72,928 NUMBER fields of the 169 files decode without a validity error and
re-encode to the same bytes; 72,890 of them are compared with a truth value or the SQL value
(the ID columns, T_DDL.AMOUNT, T_STAT1.N, R_STAT.AMT, T_DECL, ...): 0 differences.

What a reader must know about the range (observed values; the type mapping is the Architect's):
- 9223372036854775807 (Int64 max) is 11 bytes, E = 9 (`ca`); -9223372036854775808 is 12 bytes.
  Any value with E >= 10 (b[0] >= `cb` or <= `34`) is at least 10^20 and does not fit Int64.
- 38 significant digits fit a 128-bit decimal; but a NUMBER **without** precision can hold 40
  significant digits (`t_num` row 31: `.3333333333333333333333333333333333333333`, 20 digit bytes
  `22`) and exponents far outside any fixed scale (1E+125, 1E-130). Such values exist in the
  fixtures in a column declared `NUMBER`. **Correction (session 8, note A-6):** session 7 wrote "39
  and 40 significant digits (rows 30, 31)". Row 30, `3.1415926535897932384626433832795028841`, has
  **38** significant decimal digits: it has 20 digit bytes, but the first (`04` = 03) and the last
  (`0b` = 10) each carry a zero that is not significant. Counted in all three truth files
  (`truth_num.txt`, `truth_num2.txt`, `truth_numtypes.txt`), the values with 30 or more significant
  digits have 34 (row 20), 38 (rows 21, 22, 23, 24, 25, 28, 30, 32) or 40 (row 31). **No value of
  the fixtures has 39 significant digits.** "Digit" in "the first and last digit are never 0" (rule
  Y1) means a base-100 digit, i.e. a digit **byte**, not a decimal digit (note A-10).
- A value with a fraction in a column declared `NUMBER(p, 0)` cannot occur (Oracle rounds on
  insert; assumed); the file itself does not enforce any declared scale.

#### Session 9: values computed by SQL (R-032, `t_num3_exp_<v>.dmp`)

T_NUM3 (`ID NUMBER(3)`, `V NUMBER`), rows inserted as `1/3`, `2/3`, `1/7`, `100/3`, `-1/3` and
`AVG(1, 2, 2)` (= 5/3) (`corpus/sql/14_num3.sql`). Conventional path; the 18c and the 11g file are
byte-identical from offset R on (`tools/version_diff.py`), so the offsets hold for both. The value
bytes equal `DUMP(v,16)` of `truth_num3.txt` in all 6 rows (`tools/check_truth.py`), and decode to the
`TO_CHAR(v,'TM9')` text exactly (`tools/num_decode.py`).

| id | expression | value bytes (offset of the length field) | digit bytes | significant decimal digits | H40 (20 digit bytes) | H38 "19 digit bytes" | H38 "38 significant digits" (note A-11) |
|---|---|---|---|---|---|---|---|
| 1 | 1/3 | `c0` + 20 x `22` (@0x224d, `15 00`) | 20 | 40 | holds | refuted | refuted |
| 2 | 2/3 | `c0` + 19 x `43` + `44` (@0x226a) | 20 (last rounded up: 67) | 40 | holds | refuted | refuted |
| 3 | 1/7 | `c0` + 6 x `0f 1d 3a` + `0f 1e` (@0x2287) | 20 | 40 | holds | refuted | refuted |
| 4 | 100/3 | `c1` + 20 x `22` (@0x22a4) | 20 | 40 (33.333...: 2 + 38) | holds | refuted | refuted |
| 5 | -1/3 | `3f` + 20 x `44`, no closing `66` (@0x22c1) | 20 | 40 | holds | refuted | refuted |
| 6 | AVG = 5/3 | `c1 02` + 18 x `43` + `44` (@0x22de) | 20 (first `02` = 1) | **39** (1 + 38 decimals) | holds | refuted (would be `c1 02` + 17 x `43` + `44`, Len=20) | refuted (would end `47`) |

| statement | evidence | status |
|---|---|---|
| A quotient or an average that does not terminate is stored with the full **20 digit bytes** (Len = 21), rounded half up in the last base-100 digit; this is 40 significant decimal digits when the first digit byte is 10 or more and 39 when it is below 10 (row 6). H40 holds for all six rows; both readings of H38 are refuted | table; `truth_num3.txt` both folders | confirmed (6 values, 2 versions) |
| 18c and 11g store the same bytes | `t_num3_exp_18c` = `t_num3_exp_11g` from R on | confirmed |
| Correction of session 8 ("no value of the fixtures has 39 significant digits"): row 6 has 39 | 6.1 above; rule Y1 | confirmed |
| A computed value is not limited to 38 digits: Oracle's "38 digits of precision" is not what it stores for an unconstrained `NUMBER`; the stored value is whatever fits 20 digit bytes | rows 1 to 6 | confirmed for division and AVG; other functions (SQRT, LN, EXP, POWER with fractions): not in the fixtures, assumed the same |

**For the project owner (Q-1).** A column declared `NUMBER` without precision that holds results of
division or averages will hold values with 39 or 40 significant digits **as a matter of course**,
not as a rarity: every non-terminating quotient (1/3, 2/3, 1/7, 100/3, 5/3 in the fixtures) is stored
with the full 20 digit bytes on both 18c and 11g. Such values never fit a 38-digit decimal exactly,
and with Decimal128(38, 10) every one of them is rounded (or rejected) unless the column is read as a
string or a wider type.

### 6.2 DATE (`t_date`: ID NUMBER(3), D DATE)

| truth row | value | length field | value bytes in the dump | `t_date` offset | `t_date/direct` offset | vs `DUMP(d,16)` |
|---|---|---|---|---|---|---|
| 1 | 2012-09-17 13:31:01 | `07 00` | `78 70 09 11 0e 20 02` | 0x224b | 0x224b | equal |
| 2 | 2016-03-26 00:00:00 | `07 00` | `78 74 03 1a 01 01 01` | 0x225a | 0x2258 | equal |
| 3 | 1992-11-30 15:17:00 | `07 00` | `77 c0 0b 1e 10 12 01` | 0x2269 | 0x2265 | equal |
| 4 | 0001-01-01 00:00:00 | `07 00` | `64 65 01 01 01 01 01` | 0x2278 | 0x2272 | equal |
| 5 | 9999-12-31 23:59:59 | `07 00` | `c7 c7 0c 1f 18 3c 3c` | 0x2287 | 0x227f | equal |
| 6 | 2000-01-01 00:00:00 | `07 00` | `78 64 01 01 01 01 01` | 0x2296 | 0x228c | equal |
| 7 | 1999-12-31 23:59:59 | `07 00` | `77 c7 0c 1f 18 3c 3c` | 0x22a5 | 0x2299 | equal |
| 8 | -0100-06-15 12:00:00 (BC) | `07 00` | `63 64 06 0f 0d 01 01` | 0x22b4 | 0x22a6 | equal |
| 9 | -4712-01-01 00:00:00 (BC) | `07 00` | `35 58 01 01 01 01 01` | 0x22c3 | 0x22b3 | equal |
| 10 | 2020-02-29 12:00:00 | `07 00` | `78 78 02 1d 0d 01 01` | 0x22d2 | 0x22c0 | equal |
| 11 | NULL | `fe ff` | (none) | 0x22e1 | 0x22cd | NULL = NULL |

Status: confirmed (11 rows x 2 paths; also in `multi_types`, and the D column of `t_nullmix`).
Every non-NULL DATE is 7 bytes. Observed in these pairs: byte 1 = century + 100, byte 2 = year
in century + 100, bytes 3-4 = month, day, bytes 5-7 = hour + 1, minute + 1, second + 1. For the
BC rows the first two bytes are below 100: row 8 `63 64` (99, 100), row 9 `35 58` (53, 88),
which fits 100 - 1 / 100 - 0 and 100 - 47 / 100 - 12 for years -0100 and -4712.

### 6.3 Character values: bytes or characters (`t_text`)

T_TEXT: V VARCHAR2(200), C CHAR(10), NV NVARCHAR2(50). `v_chars` / `v_bytes` are `LENGTH(v)` /
`LENGTHB(v)` from `truth_text.txt`. Offsets are `t_text` / `t_text/direct`.

| truth row | v_chars | v_bytes | V length field | C truth Len | C length field | NV truth Len | NV length field |
|---|---|---|---|---|---|---|---|
| 1 | 11 | 11 | `0b 00` @0x229d / 0x229d | 10 | `0a 00` @0x22aa / 0x22aa | 22 | `16 00` @0x22b6 / 0x22b6 |
| 2 | 11 | 20 | `14 00` @0x22d4 / 0x22d2 | 10 | `0a 00` @0x22ea / 0x22e8 | 12 | `0c 00` @0x22f6 / 0x22f4 |
| 3 | 13 | 16 | `10 00` @0x230a / 0x2306 | 10 | `0a 00` @0x231c / 0x2318 | 22 | `16 00` @0x2328 / 0x2324 |
| 4 | 3 | 9 | `09 00` @0x2346 / 0x2340 | 10 | `0a 00` @0x2351 / 0x234b | 6 | `06 00` @0x235d / 0x2357 |
| 5 | 1 | 4 | `04 00` @0x236b / 0x2363 | 10 | `0a 00` @0x2371 / 0x2369 | 4 | `04 00` @0x237d / 0x2375 |
| 6 | NULL | NULL | `fe ff` @0x2389 / 0x237f | NULL | `fe ff` @0x238b / 0x2381 | NULL | `fe ff` @0x238d / 0x2383 |
| 7 | 18 | 18 | `12 00` @0x2395 / 0x2389 | 10 | `0a 00` @0x23a9 / 0x239d | 36 | `24 00` @0x23b5 / 0x23a9 |
| 8 | 200 | 200 | `c8 00` @0x23e1 / 0x23d3 | 10 | `0a 00` @0x24ab / 0x249d | 100 | `64 00` @0x24b7 / 0x24a9 |
| 9 | 15 | 15 | `0f 00` @0x2523 / 0x2513 | 10 | `0a 00` @0x2534 / 0x2524 | 22 | `16 00` @0x2540 / 0x2530 |
| 10 | 11 | 11 | `0b 00` @0x255e / 0x254c | 10 | `0a 00` @0x256b / 0x2559 | 18 | `12 00` @0x2577 / 0x2565 |

| statement | evidence | status |
|---|---|---|
| The length field counts **bytes**, not characters | rows 2, 3, 4, 5: `v_chars` 11, 13, 3, 1 against length fields 20, 16, 9, 4 = `v_bytes` | confirmed (4 rows x 2 paths, also `multi_types`) |
| VARCHAR2 and CHAR value bytes are the AL32UTF8 bytes of `DUMP(col,1016)` | e.g. row 2 V `d0 9f d1 80 d0 b8 ...` at `t_text`@0x22d6; row 5 V `f0 9f 98 80` at @0x236d (a 4-byte character) | confirmed for client set = database set = AL32UTF8; session 3: also for the WE8MSWIN1252 and US7ASCII clients (rows below) |
| CHAR(10) is blank-padded to 10 **bytes** with 0x20 | row 2 C `d0 b4 d0 b0 20 20 20 20 20 20` at `t_text`@0x22ec (2 characters of 2 bytes + 6 spaces) | confirmed |
| NVARCHAR2 value bytes are AL16UTF16 = UTF-16 big-endian, 2 bytes per code unit; the length counts bytes | row 1 NV `00 70 00 6c ...` at `t_text`@0x22b8 (22 bytes for 11 characters); row 5 NV `d8 3d de 00` at @0x237f (surrogate pair, 4 bytes for 1 character) | confirmed |
| Trailing spaces of a VARCHAR2 are kept | row 7 V, 18 bytes ending `20 20 20`, at `t_text`@0x2397 | confirmed |
| A value may contain LF (0x0a) and TAB; it is not escaped. Values are found by their length, never by searching for LF | row 9 V `6c 69 6e 65 31 0a 6c 69 6e 65 32 09 74 61 62` at `t_text`@0x2525 | confirmed |

All 30 fields per path equal `truth_text.txt`.

#### Session 3: the same table with a WE8MSWIN1252 and a US7ASCII client (R-008)

The offsets of every field in `t_text_exp_w1252_18c.dmp` and `t_text_exp_us7ascii_18c.dmp` are
the offsets of `t_text` in the table above (first number), and those of the two direct files are
the offsets of `t_text/direct` (second number), because the table sections are byte-identical
(section 2.1). `tools/check_truth.py` on the four new files: 30 fields equal `truth_text.txt`,
0 differ, in each.

| question | what the files show | evidence (same offset in `t_text_exp_w1252` and `t_text_exp_us7ascii`; direct files in brackets) | status |
|---|---|---|---|
| V and C: client set or database set? | The database set. The bytes are the AL32UTF8 bytes of `DUMP(col,1016)` | row 3 V @0x230c [0x2308] `43 61 66 c3 a9 20 5a c3 bc 72 69 63 68 20 c3 b1`; row 3 C @0x231e [0x231a] `c3 a9 c3 a8 20 20 20 20 20 20` | confirmed |
| Row 3 (`Café Zürich ñ`) with the WE8MSWIN1252 client: one byte per accented letter? | **No.** `é`, `ü`, `ñ` are 2 bytes each (`c3 a9`, `c3 bc`, `c3 b1`); the value has 16 bytes, not 13. The expectation in the session prompt and in R-008 was wrong | `t_text_exp_w1252`@0x230a length `10 00`, value @0x230c; `t_text_exp_direct_w1252`@0x2306 `10 00` | confirmed (refutes the expectation) |
| Rows 2, 4, 5 (characters the client set cannot represent): what bytes, how many? | The unchanged database bytes: row 2 V 20 bytes `d0 9f d1 80 ...`, row 4 V 9 bytes `e6 97 a5 e6 9c ac e8 aa 9e`, row 5 V 4 bytes `f0 9f 98 80`. No replacement character (`3f`, `bf`) appears | @0x22d4 `14 00` [0x22d2]; @0x2346 `09 00` [0x2340]; @0x236b `04 00` [0x2363] | confirmed |
| Is the length field the number of bytes written? | Yes, and here that is also the database byte count (`v_bytes` of the truth): 20, 16, 9, 4 | same offsets | confirmed. Whether the length would follow a converted byte count cannot be seen: nothing was converted |
| CHAR(10) padding | Still padded to 10 **bytes** with 0x20, counted in database bytes | row 2 C @0x22ec [0x22ea] `d0 b4 d0 b0` + 6 spaces | confirmed |
| NV (NVARCHAR2) affected by the client set? | No. UTF-16BE, same bytes | row 2 NV @0x22f8 [0x22f6] `04 1f 04 40 04 38 04 32 04 35 04 42`; row 5 NV @0x237f [0x2377] `d8 3d de 00` | confirmed |
| Does anything else in the table section change? | No: `CREATE TABLE` text, `INSERT INTO` line, column count, descriptors (+0, +2, +4 = 873 / 2000, +6 = 1 / 2), `00 00`, path flag, `ff ff` are identical | section 2.1 (`cmp -l`: no difference from 0x210d to the end of the data) | confirmed |

**Rule for a reader (testable).** Decode a VARCHAR2 or CHAR value with the character set whose
id stands at +4 of its column descriptor (873 = AL32UTF8 in every fixture), and an NVARCHAR2
value with the id at +4 of its descriptor (2000 = AL16UTF16, big-endian). Do **not** use the id at
header 0x0001 / 0x0030: in the four new files it is 178 or 1 while the bytes are UTF-8.

- What the data support: the descriptor id is right in 6 files with 3 client sets on 2 paths;
  the id at 0x0001 is wrong in 4 of them (decoding row 3 as WE8MSWIN1252 gives `CafÃ© ZÃ¼rich Ã±`;
  US7ASCII cannot decode the bytes above 0x7f at all).
- What the data do not support: a choice between the descriptor id and header 0x0032. Both are
  873 in every file, because there is one database. The descriptor is per column and separates
  VARCHAR2 from NVARCHAR2, which 0x0032 alone cannot, so the rule names the descriptor.
- Not covered: a database whose character set is not AL32UTF8 (the conversion the log calls
  "possible" may happen there); text items outside the row data (all ASCII in the fixtures).

#### Session 7: NCHAR and `CHAR(n CHAR)` (R-026a, `nchar_exp_<v>.dmp`)

T_NCHAR: `ID NUMBER(3)`, `NC NCHAR(5)`, `CC CHAR(4 CHAR)`, `NV NVARCHAR2(5)` in the AL32UTF8 database
(national set AL16UTF16); descriptors in 4.1. Value bytes against `truth_nchar.txt`
(`tools/check_truth.py`: 9 fields per file, 0 DIFF; offsets of the length fields, the same in the
18c and the 11g file):

| row | NC (`NCHAR(5)`) | CC (`CHAR(4 CHAR)`) | NV (`NVARCHAR2(5)`) |
|---|---|---|---|
| 1 (`'ab'`) | @0x22a2 `0a 00` + `00 61 00 62 00 20 00 20 00 20` | @0x22ae `04 00` + `61 62 20 20` | @0x22b4 `04 00` + `00 61 00 62` |
| 2 (U+0434 U+0436) | @0x22c0 `0a 00` + `04 34 04 36 00 20 00 20 00 20` | @0x22cc `06 00` + `d0 b4 d0 b6 20 20` | @0x22d4 `04 00` + `04 34 04 36` |
| 3 (NULL) | @0x22e0 `fe ff` | @0x22e2 `fe ff` | @0x22e4 `fe ff` |

| statement | evidence | status |
|---|---|---|
| NCHAR (type 96 with set id 2000, form 2) holds UTF-16 big-endian, padded with U+0020 (`00 20`) to n **characters**: always 2n bytes for characters of the Basic Multilingual Plane | rows 1, 2 NC: 10 bytes = 5 code units | confirmed (2 rows, 2 versions; a character outside the BMP is not in the fixtures) |
| `CHAR(n CHAR)` (type 96, set 873, form 1) is padded with 0x20 to n **characters**, not to n bytes and not to the descriptor length: `'ab'` is 4 bytes, U+0434 U+0436 is 6 bytes (2 x 2 bytes + 2 spaces), descriptor 16 | rows 1, 2 CC | confirmed (2 rows, 2 versions) |
| So for type 96 the value length is not fixed: a reader must not expect "descriptor length" bytes or "n bytes"; it reads the length field (rule R2) like for any type | rows 1, 2 CC; `t_text` C (byte semantics, always 10) | confirmed |
| NVARCHAR2 is not padded | NV 4 bytes in rows 1, 2 | confirmed |
| The form value at +6 is 2 for both national types (NCHAR, NVARCHAR2) and 1 for both database-set types (CHAR, VARCHAR2): with set ids 2000 and 873 it separates the two sets | 4.1, this table | confirmed as a description (4 types); the meaning "1 = database set, 2 = national set" is still a hypothesis, because id and form always change together |

#### Session 9: a character outside the BMP (R-033, `nchar2_exp_<v>.dmp`)

T_NCHAR2 row 1 holds `'a'` + U+1F600 in every column (UTF-16: `00 61 d8 3d de 00`, 3 code units;
UTF-8: `61 f0 9f 98 80`, 2 characters). Offsets of the length fields, 18c / 11g. Every value equals
`truth_nchar2.txt` (`tools/check_truth.py`: 6 fields per file, 0 DIFF).

| column (descriptor length) | field | bytes | what it shows | status |
|---|---|---|---|---|
| NC `NCHAR(3)` (6) | @0x24d4 / 0x24ce `06 00` | `00 61 d8 3d de 00` | UTF-16 BE with the **surrogate pair** `d8 3d de 00`; **no padding**: the value has 3 code units = the declared 3, so NCHAR pads to n UTF-16 **code units**, not to n characters (with characters it would be `00 61 d8 3d de 00 00 20`, 8 bytes, more than the descriptor) | confirmed (2 versions; `truth_nchar2.txt` `Typ=96 Len=6`) |
| NV `NVARCHAR2(3)` (6) | @0x24dc / 0x24d6 `06 00` | `00 61 d8 3d de 00` | the same, not padded | confirmed |
| CC `CHAR(3 CHAR)` (12) | @0x24e4 / 0x24de `06 00` | `61 f0 9f 98 80 20` | UTF-8 (4 bytes for U+1F600, not CESU-8's 6), padded with **one** space to 3 **characters**: U+1F600 counts as 1 character in AL32UTF8 | confirmed (`Typ=96 Len=6`) |
| VC `VARCHAR2(3 CHAR)` (12) | @0x24ec / 0x24e6 `05 00` | `61 f0 9f 98 80` | UTF-8, not padded | confirmed |
| CL `CLOB`, NCL `NCLOB` | LOB values after the row: @0x25dd / 0x25cb and @0x25e5 / 0x25d3, `06 00` each (k = 2 @0x25db / 0x25c9) | `00 61 d8 3d de 00` both | the CLOB of an AL32UTF8 database is UTF-16 BE with the surrogate pair, like the NCLOB (rule L5); `DBMS_LOB.GETLENGTH` = 3 for both: it counts UTF-16 **code units**, not characters (2) | confirmed (2 versions; truth `\|3\|3\|`, `DUMP(SUBSTR(cl,10,1))` = `61,f0,9f,98,80` in AL32UTF8, `DUMP(SUBSTR(ncl,10,1))` = `0,61,d8,3d,de,0`) |

So a reader decodes NCHAR / NVARCHAR2 / NCLOB / CLOB(873) as UTF-16 BE **with** surrogate pairs (a
strict UCS-2 decoder fails on `d8 3d`), and CHAR / VARCHAR2 in AL32UTF8 as UTF-8 with 4-byte
sequences. "n characters" in Oracle's length semantics means code units for the national set
(AL16UTF16) and code points for AL32UTF8.

#### Session 9: character semantics with the byte caps (R-034, `t_probe11_exp(_direct)_<v>.dmp`)

Descriptors in 4.1. Values (offsets of the length fields; conventional file, direct file; the same
in 18c and 11g). Every value equals `truth_probe11.txt` (`tools/check_truth.py`: 14 fields per file,
0 DIFF; Y and Z by `LENGTHB` + the inserted text + spaces).

| row | B `VARCHAR2(10 BYTE)` | C `VARCHAR2(10 CHAR)` | K `CHAR(3 CHAR)` | W, X `VARCHAR2(1000/1001 CHAR)` | Y `CHAR(500 CHAR)` | Z `CHAR(501 CHAR)` |
|---|---|---|---|---|---|---|
| 1 (`abc`) | @0x2349 `03 00 61 62 63` | @0x234e, 3 bytes | @0x2353 `03 00 61 62 63` (3 characters: no padding) | @0x2358, @0x235d, 3 bytes | @0x2362 `f4 01` = 500 bytes: `abc` + 497 spaces | @0x2558 `f5 01` = 501 |
| 2 (U+65E5 U+672C U+8A9E) | @0x2755 / 0x2753 `fe ff` (NULL) | @0x2757 / 0x2755 `09 00 e6 97 a5 e6 9c ac e8 aa 9e` | @0x2762 / 0x2760 9 bytes (no padding) | @0x276d, @0x2778 / 0x276b, 0x2776, 9 bytes | @0x2783 / 0x2781 `fa 01` = 506 = 9 + 497 | @0x297f / 0x297d `fb 01` = 507 = 9 + 498 |

| statement | evidence | status |
|---|---|---|
| `CHAR(n CHAR)` is padded with 0x20 to n characters; its byte length is (bytes of the text) + (n - characters of the text), from n up to the cap | rows 1, 2 of K, Y, Z; `nchar`, `nchar2` CC | confirmed (5 columns, 2 versions, 2 paths) |
| The descriptor length is min(4n, 4000) / min(4n, 2000) (4.1); every value is at most that long (506, 507 < 2000) | 4.1; `tools/desc_len_check.py` | confirmed |
| Conventional and direct path give the same value bytes | 4 files | confirmed (rule R7) |
| A value whose padded length would exceed the cap (`CHAR(501 CHAR)` with 501 three-byte characters = 1503 bytes fits; 501 four-byte characters = 2004 bytes would not) | not in the fixtures; Oracle presumably refuses the insert (assumed) | unknown |

**What a reader may check against the descriptor length** (`tools/desc_len_check.py`, 243,007 column
fields of 201 files in session 11, of which 223,085 non-NULL fields of types other than 8 / 24 compared; 238,657 of 199
files in session 9): for types 1, 2, 12, 23, 69, 96, 180, 181, 182, 183, 208, 231 the value length
never exceeds the descriptor length (0 exceptions); for LOB columns (112, 113) the field is a locator
of 102 to 150 / 1102 bytes against a descriptor of 72, and for LONG / LONG RAW the descriptor is 0. So
a reader may treat "value longer than the descriptor" as an error **for types 1, 23, 96** (where the
descriptor is a real maximum: n, 2n, or min(4n, cap)) and must not apply the check to 8, 24, 112, 113.
It must not expect the value to **equal** the descriptor length: only CHAR with byte semantics and
NCHAR without non-BMP characters are padded to it (27,850 of 27,880 CHAR / NCHAR values are; the 30
others are character-semantics values), and VARCHAR2 is never padded. Recommendation: a warning, not
a stop, because the descriptor carries no information the reader needs to read the value (the length
field does).

**Session 10, note A-23 (character-semantics width in CL8MSWIN1251).** `R_TEXT."VC"` is declared `VARCHAR2(20 CHAR)`
(CREATE TABLE text, e.g. `ru_lob2_exp_cp1251_11g.dmp`@0x16292) and its descriptor is `01 00 14 00 ab 00 01 00` = 20 bytes
with set 171 (@0x16466; the same in every `ru_tbl_*`, `ru_exp_*`, `ru_tables_*`, `ru_lob2_*` file); `truth_ru.txt` row 9
has `DUMP(vc)` = `Typ=1 Len=20` for 20 Cyrillic characters. So w = 1 for set 171 is **confirmed** (1 column, 1 table, 1
database, many files with 4 client sets; the truth gives the 20-byte value, the descriptor the 20-byte maximum).
w = 1 for the other single-byte sets (US7ASCII, WE8MSWIN1252) as **database** sets: assumed (no such database in the
fixtures).

### 6.4 TIMESTAMP (first wave; `t_ts`)

| statement | evidence | status |
|---|---|---|
| Value bytes = `DUMP(col,16)` of `truth_ts.txt` for TS0, TS6, TS9, all 6 rows, both paths (18 fields each) | `tools/check_truth.py`; e.g. row 1 TS6 `78 74 03 1a 10 0a 1c 26 b6 72 38` at `t_ts`@0x22a5 | confirmed |
| A value is 7 bytes when the fraction is zero (any declared precision) and 11 bytes otherwise | row 2: TS6 at `t_ts`@0x22cc and TS9 at @0x22d5 are `07 00` + 7 bytes; row 3: TS6 @0x22ed, TS9 @0x22fa are `0b 00` + 11 bytes | confirmed |
| The 4 fraction bytes are nanoseconds, big-endian | row 3 TS9 (`.000000001`) ends `00 00 00 01` (`t_ts`@0x2303); row 3 TS6 (`.000001`) ends `00 00 03 e8` = 1000 (@0x22f6) | confirmed |
| The first 7 bytes follow the DATE layout | row 1 TS0 `78 74 03 1a 10 0a 1c` = 2016-03-26 15:09:27 (`t_ts`@0x229c) | confirmed |

#### Session 17: TIMESTAMP(1) to (5), (7), (8) (R-047; `tsp_exp(_direct)_<v>.dmp`)

Table T_TSP (`ID NUMBER(2)`, `T1 TIMESTAMP(1)` ... `T5 TIMESTAMP(5)`, `T7 TIMESTAMP(7)`, `T8 TIMESTAMP(8)`), 4 rows:
row 1 fraction 0, row 2 the largest fraction of each precision, row 3 the literal `.123456789` in every column, row 4
NULL in every TIMESTAMP column (R-047 SQL). Oracle 18c and 11g, conventional and direct, `statistics=none`.
`tools/tsp_check.py` compares every descriptor and every field of the four files with `truth_tsp.txt`: 0 differences.
Offsets are those of the 18c and 11g **conventional** files (the two versions have the same offsets); direct-path
offsets are given where they differ (no `00 00` after a row, so row 2 starts 2 bytes earlier, row 3 4, row 4 6).

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x22f5 | 2 | `08 00`: 8 columns | `tsp_exp_<v>`@0x22f5; INSERT line @0x2288 names 8 columns | confirmed (4 files) |
| 0x22f7 | 4 | `02 00 16 00`: ID, NUMBER, 22 | `truth_tsp.txt` `ID\|NUMBER\|22\|0` | confirmed (4 files) |
| 0x22fb, 0x22ff, 0x2303, 0x2307, 0x230b, 0x230f, 0x2313 | 4 each | `b4 00 0b 00`: type 180, maximum length **11**, for T1, T2, T3, T4, T5, T7, T8 | `truth_tsp.txt` `T1\|TIMESTAMP(1)\|11\|1` ... `T8\|TIMESTAMP(8)\|11\|8` | confirmed (4 files, 7 precisions) |
| 0x2317 | 4 | `00 00 00 00` (conventional) / `00 00 01 00` (direct): m 0, path flag | rule T6 | confirmed |
| 0x231b | 4 | row 1: ID `02 00 c1 02` | id 1 | confirmed |
| 0x231f, 0x2328, 0x2331, 0x233a, 0x2343, 0x234c, 0x2355 | 9 each | row 1, t1 ... t8: `07 00` + `78 78 01 02 04 05 06` (2020-01-02 03:04:05, 7 bytes, no fraction bytes) | `truth_tsp.txt` row 1: `Typ=180 Len=7: 120,120,1,2,4,5,6` x 7 | confirmed (4 files) |
| 0x235e | 2 | `00 00` row end (conventional only) | rule R4 | confirmed |
| 0x2364, 0x2371, 0x237e, 0x238b, 0x2398, 0x23a5, 0x23b2 (direct 0x2362 ... 0x23b0) | 13 each | row 2: `0b 00` + `78 78 01 02 04 05 06` + u32 BE: t1 `35 a4 e9 00` (900,000,000), t2 `3b 02 33 80` (990,000,000), t3 `3b 8b 87 c0` (999,000,000), t4 `3b 99 43 60` (999,900,000), t5 `3b 9a a2 f0` (999,990,000), t7 `3b 9a c9 9c` (999,999,900), t8 `3b 9a c9 f6` (999,999,990) | `truth_tsp.txt` row 2 (e.g. t1 `Len=11: 120,120,1,2,4,5,6,53,164,233,0`); `TO_CHAR` row 2: `.900000000`, `.999000000`, `.999999990` | confirmed (4 files) |
| 0x23c5, 0x23d2, 0x23df, 0x23ec, 0x23f9, 0x2406, 0x2413 (direct 0x23c1 ... 0x240f) | 13 each | row 3: `0b 00` + DATE part + t1 `05 f5 e1 00` (100,000,000), t2 `07 27 0e 00` (120,000,000), t3 `07 54 d4 c0` (123,000,000), t4 `07 5c 75 e0` (123,500,000), t5 `07 5b d9 a0` (123,460,000), t7 `07 5b cd 20` (123,456,800), t8 `07 5b cd 16` (123,456,790) | `truth_tsp.txt` row 3; `TO_CHAR` row 3: `.100000000`, `.123000000`, `.123456790` | confirmed (4 files) |
| 0x2426 to 0x2432 (direct 0x2420 to 0x242c) | 2 each | row 4: `fe ff` in each of the 7 TIMESTAMP fields (ID `02 00 c1 05` @0x2422 / 0x241c) | `truth_tsp.txt` row 4 `NULL` x 7 | confirmed (4 files) |
| 0x2436 (direct 0x242e) | 3 | `ff ff 0a`: end of rows | rule E1 | confirmed |

| statement | evidence | status |
|---|---|---|
| The descriptor length of TIMESTAMP(f) is 11 for every f from 1 to 9 seen (1, 2, 3, 4, 5, 6, 7, 8, 9) and 7 for f = 0; it equals `user_tab_columns.data_length` | table; `t_ts_exp_<v>`@0x2288 (f = 0: 7), @0x228c, @0x2290 (f = 6, 9: 11) | confirmed (all 10 precisions, 2 versions, 2 paths) |
| A value is 7 bytes when the fraction is 0 and 11 bytes otherwise, for every precision (Y5) | rows 1 to 3 | confirmed |
| Row 3: the stored fraction is the literal **rounded** to f digits, not truncated. t1 to t3 do not separate the two (the next digit of `.123456789` is below 5); t4 (123,500,000, truncation would give 123,400,000), t5 (123,460,000 / 123,450,000), t7 (123,456,800 / 123,456,700), t8 (123,456,790 / 123,456,780) do | `tsp_exp_<v>`@0x23ec, 0x23f9, 0x2406, 0x2413; `truth_tsp.txt` row 3 | confirmed. The rounding is done by the database at `INSERT`; the dump holds the stored bytes (= `DUMP`), so a reader decodes, never rounds |
| The field bytes are the same on both paths (R7) and on both versions: the sequence of all 32 fields (u16 + value) is identical in the four files | `tools/tsp_check.py` "field bytes ... IDENTICAL" (6 pairs); `cmp -l tsp_exp_18c.dmp tsp_exp_11g.dmp`: 22 bytes, all in the header | confirmed |
| `CREATE TABLE` text: `"T1" TIMESTAMP (1)` ... `"T8" TIMESTAMP (8)` (with a space before the parenthesis, as for f = 0, 6, 9) | `tsp_exp_<v>`@0x211b | confirmed (4 files) |

### 6.5 Other types: RAW, TIMESTAMP WITH TIME ZONE, WITH LOCAL TIME ZONE

#### RAW (`t_raw`: ID NUMBER(3), R RAW(2000); `truth_raw.txt` as corrected by R-010)

Every byte of every value was compared (`tools/check_truth.py`; the truth gives the length and the
hex of all bytes, in two parts for the 2000-byte value).

| truth row | truth length | length field | value bytes in the dump | `t_raw` offset | `t_raw/direct` offset | vs truth |
|---|---|---|---|---|---|---|
| 1 | 4 | `04 00` | `00 ff 80 7f` | 0x224d | 0x224d | equal |
| 2 | 1 | `01 00` | `00` | 0x2259 | 0x2257 | equal |
| 3 | NULL | `fe ff` | (none) | 0x2262 | 0x225e | NULL = NULL |
| 4 | 2000 | `d0 07` | 2000 x `ab` | 0x226a | 0x2264 | equal (part 1 + part 2 of the truth) |
| 5 | 251 | `fb 00` | 251 x `cd` | 0x2a42 | 0x2a3a | equal |
| 6 | 255 | `ff 00` | 255 x `ef` | 0x2b45 | 0x2b3b | equal |
| 7 | 256 | `00 01` | 256 x `12` | 0x2c4c | 0x2c40 | equal |

| statement | evidence | status |
|---|---|---|
| A RAW value is its own bytes after the u16 LE length; nothing is added, escaped or translated | 7 rows x 2 paths, table above | confirmed |
| Bytes that are not valid UTF-8 pass unchanged although the database set is AL32UTF8 | row 1 `00 ff 80 7f` (`ff` and a lone `80` are not UTF-8) at `t_raw`@0x224f, `t_raw/direct`@0x224f | confirmed |
| A 2000-byte value is one field, not split into pieces | `t_raw`@0x226a `d0 07` + 2000 bytes, next row at 0x2a3e; `t_raw/direct`@0x2264 | confirmed |
| No conversion when the client set differs from the database set | no RAW fixture with another client set exists; client = database = AL32UTF8 in both files | not tested (hypothesis; consistent with section 6.3, where even character data are not converted) |
| Type code 23, descriptor length = declared size | `t_raw`@0x2241 `17 00`, @0x2243 `d0 07` (RAW(2000)) | confirmed for the code; the length is observed (one declared size) |

#### TIMESTAMP WITH TIME ZONE (`t_tstz`, column TSTZ TIMESTAMP(6) WITH TIME ZONE, type 181)

All six non-NULL values are 13 bytes and equal `DUMP` in `truth_tstz.txt` on both paths
(session 2). Session 3 reads the 13 bytes field by field (`tools/tstz_fields.py`). The truth gives
the local time, the offset and the zone name as the database shows them.

| value offset (in the 13 bytes) | length | meaning | evidence | status |
|---|---|---|---|---|
| +0 | 7 | date and time in the DATE layout of section 6.2, **in UTC**, not in the zone of the value | all 6 rows, table below: e.g. row 1 `78 74 03 1a 0f 05 38` = 2016-03-26 14:04:55 for local 19:34:55 +05:30 (`t_tstz`@0x22a8); row 3 `78 74 03 1b 04 23 38` = 2016-03-27 03:34:55 for local 2016-03-26 19:34:55 -08:00 (the date changes too, @0x22d6) | confirmed (6 rows x 2 paths) |
| +7 | 4 | fraction in nanoseconds, big-endian | rows 1-5 `07 5b ca 00` = 123456000 (`.123456`), @0x22af; row 6 `00 00 00 00` (@0x2322) | confirmed |
| +7 | 4 | the fraction bytes are present even when the fraction is zero: the value is always 13 bytes (unlike TIMESTAMP, section 6.4) | row 6, `t_tstz`@0x2319 `0d 00`, `t_tstz/direct`@0x230f `0d 00` | observed (one row, both paths) |
| +11, +12 | 1 + 1 | zone given as an offset: hour + 20, minute + 60 | row 1 `19 5a` = 25, 90 = +05:30 (@0x22b3); row 2 `14 3c` = 20, 60 = +00:00 (@0x22ca); row 3 `0c 3c` = 12, 60 = -08:00 (@0x22e1) | confirmed (3 rows x 2 paths). A negative offset with minutes (e.g. -03:30) is not in the fixtures: whether the minute byte is then below 60 is unknown |
| +11, +12 | 2 | zone given as a region name: **not** hour + 20 / minute + 60. Bit 0x80 of byte +11 is set in all three region rows and clear in all three offset rows | row 4 Europe/Moscow `86 48` (@0x22f8), row 5 UTC `d0 04` (@0x230f), row 6 America/New_York `81 90` (@0x2326); with the offset reading they would be `17 3c`, `14 3c`, `10 3c` | observed. Hypothesis: bit 0x80 = "region", the other 15 bits identify the region in Oracle's time zone table (3 values, no truth for an id) |

| truth row | truth text (local time, offset, zone) | 13 bytes | bytes 1-7 read as DATE | truth local time minus offset | bytes 12-13 | `t_tstz` / `t_tstz/direct` value offset |
|---|---|---|---|---|---|---|
| 1 | 2016-03-26 19:34:55.123456 +05:30 | `78 74 03 1a 0f 05 38 07 5b ca 00 19 5a` | 2016-03-26 14:04:55 | 2016-03-26 14:04:55 | `19 5a` = 5 + 20, 30 + 60 | 0x22a8 / 0x22a8 |
| 2 | 2016-03-26 19:34:55.123456 +00:00 | `78 74 03 1a 14 23 38 07 5b ca 00 14 3c` | 2016-03-26 19:34:55 | 2016-03-26 19:34:55 | `14 3c` = 0 + 20, 0 + 60 | 0x22bf / 0x22bd |
| 3 | 2016-03-26 19:34:55.123456 -08:00 | `78 74 03 1b 04 23 38 07 5b ca 00 0c 3c` | 2016-03-27 03:34:55 | 2016-03-27 03:34:55 | `0c 3c` = -8 + 20, 0 + 60 | 0x22d6 / 0x22d2 |
| 4 | 2016-03-26 19:34:55.123456 +03:00 EUROPE/MOSCOW | `78 74 03 1a 11 23 38 07 5b ca 00 86 48` | 2016-03-26 16:34:55 | 2016-03-26 16:34:55 | `86 48` (region) | 0x22ed / 0x22e7 |
| 5 | 2016-03-26 19:34:55.123456 +00:00 UTC | `78 74 03 1a 14 23 38 07 5b ca 00 d0 04` | 2016-03-26 19:34:55 | 2016-03-26 19:34:55 | `d0 04` (region) | 0x2304 / 0x22fc |
| 6 | 2016-07-01 12:00:00.000000 -04:00 AMERICA/NEW_YORK | `78 74 07 01 11 01 01 00 00 00 00 81 90` | 2016-07-01 16:00:00 | 2016-07-01 16:00:00 | `81 90` (region) | 0x231b / 0x2311 |

What the data show for the region rows 4 and 6: the date and time part is UTC exactly as for the
offset rows (row 4: 16:34:55 = 19:34:55 - 03:00; row 6: 16:00:00 = 12:00:00 + 04:00, daylight
time). So the instant can be read from every value without knowing the zone. The offset and the
name of a region zone cannot be read from the 13 bytes with what the fixtures establish: the two
bytes are not an offset, and row 5 (`UTC` as a region, `d0 04`) differs from row 2 (`+00:00`,
`14 3c`) although both are the same instant and the same offset.

Consequence for a reader: instant = bytes 1-11 (UTC); local time = instant + offset, where the
offset comes from bytes 12-13 only if bit 0x80 of byte 12 is clear. For region zones the reader
can deliver the instant, and the zone only as "region, id bytes xx xx" until a mapping exists.

#### TIMESTAMP WITH LOCAL TIME ZONE (`t_tstz`, column TSLTZ, type 231)

Rows 101 and 102 store the same instant, inserted from session time zones +00:00 and +05:30
(`corpus/sql/01_corpus_tables.sql` lines 201-204: `19:34:55.123456` in +00:00 and
`2016-03-27 01:04:55.123456` in +05:30).

| statement | evidence | status |
|---|---|---|
| The two values are byte-identical in the dump: 11 bytes `78 74 03 1a 14 23 38 07 5b ca 00` | row 101 `t_tstz`@0x233d (`0b 00`, value @0x233f), row 102 @0x2353 (value @0x2355); `t_tstz/direct`@0x232f, @0x2343; equal to `DUMP` of `truth_tstz.txt` rows 101, 102 | confirmed (H-016) |
| No zone bytes: the layout is that of TIMESTAMP (section 6.4): 7 bytes DATE layout + 4 bytes nanoseconds | same bytes: 2016-03-26 19:34:55, 123456000 ns | confirmed |
| The stored time is the instant expressed in zone +00:00 | 19:34:55 for an instant that was entered as 01:04:55 +05:30 (row 102) | confirmed as a fact of these fixtures. +00:00 is both UTC and the database time zone (`truth_env.txt` `dbtimezone=+00:00`), so the fixtures cannot say which of the two the rule is. Session 4: the 11g database has `dbtimezone=+00:00` too (`truth_env.txt` of `11g/`), and the 11g bytes are the same (`t_tstz_exp_11g.dmp`@0x233f, 0x2355), so the question is still open (R-019) |
| A TSLTZ value with a zero fraction (7 bytes?) | not in the fixtures (session 3). **Session 19: 7 bytes** in every precision, `tsltz2_exp_<v>`@0x22df, 0x22e8, 0x22f1 (below) | unknown (session 3); confirmed in session 19 |

#### Session 19: TIMESTAMP(0), (3), (9) WITH LOCAL TIME ZONE (R-050; `tsltz2_exp(_direct)_<v>.dmp`)

Table T_TSLTZ2 (`ID NUMBER(2)`, `L0`, `L3`, `L9` = `TIMESTAMP(0 / 3 / 9) WITH LOCAL TIME ZONE`), 4 rows (R-050 SQL,
`corpus/sql/17_tsltz2.sql`): row 1 `2020-01-02 03:04:05`, row 2 `... 03:04:05.123456789`, row 3 `... 03:04:05.6`, row 4
NULL x 3; inserted and read in one SQL*Plus session with `TIME_ZONE = '+00:00'`. `truth_tsltz2.txt` (18c = 11g):
line 1 `+00:00|+00:00` (dbtimezone | sessiontimezone), `data_length` 7 / 11 / 11, the decimal `DUMP` and `TO_CHAR ...
FF9` of every value. Oracle 18c and 11g, conventional and direct, `statistics=none`. `tools/tsltz2_check.py` compares
every descriptor and every field of the four files with the truth and with the bytes R-050 predicted: **0 differences**.
Offsets are those of the conventional files (18c = 11g); the direct files are the same up to row 1 and then 2, 4, 6
bytes earlier (no `00 00` after a row).

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x22c5 | 2 | `04 00`: 4 columns | `tsltz2_exp_<v>`@0x22c5 | confirmed (4 files) |
| 0x22c7 | 4 | `02 00 16 00`: ID, NUMBER, 22 | truth `ID\|NUMBER\|22\|0` | confirmed (4 files) |
| 0x22cb | 4 | `e7 00 07 00`: L0, type 231, **length 7** | truth `L0\|TIMESTAMP(0) WITH LOCAL TIME ZONE\|7\|0` | confirmed (4 files) |
| 0x22cf | 4 | `e7 00 0b 00`: L3, 231, 11 | truth `L3\|...\|11\|3` | confirmed (4 files) |
| 0x22d3 | 4 | `e7 00 0b 00`: L9, 231, 11 | truth `L9\|...\|11\|9` | confirmed (4 files) |
| 0x22d7 | 4 | `00 00 00 00` (conventional) / `00 00 01 00` (direct): m 0, path flag | 4.1 | confirmed |
| 0x22db | 4 | row 1 ID `02 00 c1 02` | id 1 | confirmed |
| 0x22df, 0x22e8, 0x22f1 | 2 + 7 each | row 1 L0, L3, L9: `07 00 78 78 01 02 04 05 06` (2020-01-02 03:04:05, DATE layout) **in all three precisions** | truth row 1 `Typ=231 Len=7: 120,120,1,2,4,5,6` x 3 | confirmed (4 files) |
| 0x2300 | 2 + 7 | row 2 L0: `07 00 78 78 01 02 04 05 06`: `.123456789` rounded to 0 | truth row 2 L0 `Len=7`; `TO_CHAR` `03:04:05.000000000` | confirmed |
| 0x2309 | 2 + 11 | row 2 L3: `0b 00` + the 7 bytes + `07 54 d4 c0` (123,000,000) | truth `Len=11: ...,7,84,212,192`; `TO_CHAR` `.123000000` | confirmed |
| 0x2316 | 2 + 11 | row 2 L9: `0b 00` + the 7 bytes + `07 5b cd 15` (123,456,789) | truth `...,7,91,205,21`; `.123456789` | confirmed |
| 0x2329 | 2 + 7 | row 3 L0: `07 00 78 78 01 02 04 05 07`: `.6` rounded **up** to second 6 | truth row 3 L0 `Len=7: 120,120,1,2,4,5,7`; `TO_CHAR` `03:04:06.000000000` | confirmed |
| 0x2332, 0x233f | 2 + 11 each | row 3 L3, L9: `0b 00` + the 7 bytes + `23 c3 46 00` (600,000,000) | truth `...,35,195,70,0` x 2 | confirmed |
| 0x2352, 0x2354, 0x2356 | 2 each | row 4: `fe ff` x 3 | truth `4\|NULL\|NULL\|NULL` | confirmed |
| 0x235a | 3 | `ff ff 0a` end of rows (direct: @0x2352) | E1 | confirmed |

| statement | evidence | status |
|---|---|---|
| The descriptor length of type 231 is 7 for f = 0 and 11 for f = 3 and 9 (= `user_tab_columns.data_length`): the TIMESTAMP rule of T5 holds for TSLTZ | @0x22cb, 0x22cf, 0x22d3; truth | confirmed for f = 0, 3, 6 (`t_tstz`), 9; f = 1, 2, 4, 5, 7, 8 of type 231: not in the fixtures (hypothesis: 11, as for type 180) |
| A TSLTZ value is 7 bytes when its stored fraction is zero, whatever the declared precision, and 11 otherwise: a column whose descriptor says 11 holds 7-byte values (row 1 L3, L9) | @0x22e8, 0x22f1 (7 bytes under `e7 00 0b 00`); `tools/desc_len_check.py`: type 231 lengths 7 and 11, none longer than the descriptor | confirmed (4 files) |
| The fraction and the seconds are what the database stored: Oracle rounded at INSERT (L0 `.6` gives second 6; L3 `.123456789` gives 123,000,000); `exp` copies the stored bytes (field = `DUMP`) | rows 2, 3; truth `DUMP` | confirmed |
| Conventional = direct, 18c = 11g: the 16 fields (u16 + value) are identical in the four files | `tools/tsltz2_check.py` "field bytes ... IDENTICAL" (6 pairs); `cmp -l tsltz2_exp_18c.dmp tsltz2_exp_11g.dmp`: 6 bytes, all below 0xa0 | confirmed |
| Every prediction of R-050 (descriptors, all 12 value byte strings, rounding, `fe ff`, same bytes on both paths) holds | `tools/tsltz2_check.py` (`EXPECT` = the R-050 list) | confirmed |
| Whether the stored bytes depend on the session time zone | these data cannot say: every value was inserted and read with session time zone +00:00, `dbtimezone` +00:00 (truth line 1); the `exp` sessions ran in containers whose `TZ` and `ORA_SDTZ` were empty and whose clock was UTC (owner's run log in ORADMP_WORK, not a fixture) | not tested by R-050. The only evidence stays `t_tstz` rows 101 / 102 (two session zones, the same bytes; 6.5 above); UTC or database zone: unknown (R-019) |

### 6.6 LONG and LONG RAW (session 7, R-028; `long_exp_<v>.dmp`)

Tables (SQL `corpus/sql/11_types.sql`, no truth file; the values below are what the SQL inserts):
T_LONG (`ID NUMBER(3)`, `L LONG`, `V VARCHAR2(10)`): `(1, 'short', 'a')`, `(2, NULL, 'b')`,
`(3, 'L' + 31,999 x 'x', 'c')`; T_LONGRAW (`ID NUMBER(3)`, `LR LONG RAW`): `(1, 00 FF)`,
`(2, 32,000 x AB)`; then T_FRAME. Conventional path, both versions; offsets are the same in the
18c and the 11g file.

| offset | bytes | meaning | evidence | status |
|---|---|---|---|---|
| 0x2254 | `03 00` | T_LONG: 3 columns | log | confirmed |
| 0x225a | `08 00 00 00` | descriptor of L: type 8, length 0, 4 bytes (4.1) | the next descriptor at 0x225e | confirmed |
| 0x2266 | `00 00 00 00` | the two bytes and path flag 0, as for any table without a LOB column | rule T6 | confirmed |
| 0x226a | `02 00 c1 02` `05 00 73 68 6f 72 74` `01 00 61` `00 00` | row 1: ID 1, L = `short` (5 bytes, a plain field), V = `a`, row end | SQL row 1 | confirmed |
| 0x227a | `02 00 c1 03` `fe ff` `01 00 62` `00 00` | row 2: L NULL is `fe ff`, V follows | SQL row 2 | confirmed |
| 0x2285 | `02 00 c1 04` `00 7d` + 32,000 bytes (`4c` + 31,999 x `78`) | row 3: L in **one** field, length 32,000 = 0x7d00; the value crosses the record boundaries 0x4000, 0x6000, 0x8000 with nothing inserted (`tools/boundary.py`) | SQL row 3 | confirmed |
| 0x9f8b | `01 00 63` `00 00` | V of row 3 directly after the 32,000 bytes: the LONG value ends where its length says | SQL row 3 | confirmed |
| 0x9f90 | `ff ff 0a` | end of rows, LF | rule E1 | confirmed |
| 0xa0ce | `02 00`, `02 00 16 00`, `18 00 00 00`, `00 00 00 00` | T_LONGRAW: 2 columns, NUMBER, LONG RAW (type 24, length 0), two bytes + flag 0 | log | confirmed |
| 0xa0dc | `02 00 c1 02` `02 00 00 ff` `00 00` | row 1: LR = `00 ff`, the raw bytes | SQL | confirmed |
| 0xa0e6 | `02 00 c1 03` `00 7d` + 32,000 x `ab` `00 00` | row 2: one field of 32,000 bytes across 0xc000, 0xe000, 0x10000 | SQL | confirmed |
| 0x11dee | `ff ff 0a` | end of rows | rule E1 | confirmed |
| 0x11df1-0x11fd9 | 489 bytes | the T_FRAME section is byte-identical to `t_frame_exp_<v>.dmp`@0x210d-0x22f5 (rule T8): after a LONG and a LONG RAW table the walk is in step | `tools/check_all.py` (T_FRAME 7 rows OK); comparison of the two sections | confirmed (2 versions) |

| statement | status |
|---|---|
| LONG (8) and LONG RAW (24) have a 4-byte descriptor with length 0 and are written like VARCHAR2 / RAW: u16 LE length + bytes, `fe ff` for NULL, no chunks, no extra bytes before or after the rows, `00 00` after each row | confirmed for values up to 32,000 bytes, conventional path, AL32UTF8 database and client, 2 versions (2 tables, 5 rows, 4 non-NULL values) |
| A LONG value is in the database character set like VARCHAR2 | hypothesis: every LONG value of the fixtures is ASCII, which the database and the client set write the same way; the descriptor has **no** set id |
| A value above 65,533 bytes (it cannot have a u16 length; Oracle allows 2 GB) | unknown: not in the fixtures (R-031) |
| LONG / LONG RAW on the direct path; with another client set | unknown: not in the fixtures (R-031) (session 9: answered below) |

#### Session 9: a non-ASCII LONG, a US7ASCII client, LONG on the direct path (R-031)

**What the files cannot answer.** Oracle refused the 100,000-byte LONG (`truth_long2.txt` line 2:
`INSERT REFUSED|ORA_22835: Buffer too small for CLOB to CHAR or BLOB to RAW conversion (actual:
100000, maximum: 4000)`), so T_LONG2 has row 1 only. A LONG value above 32,000 bytes is still not in
the fixtures; request R-040 asks for it by another route.

**Character set (note A-8).** Row 1 of T_LONG2 is `UNISTR('\0434\0436 Caf\00E9')`; `truth_long2.txt`
`1|10|D0B4D0B620436166C3A9` (LENGTHB 10, the AL32UTF8 bytes).

| file | client | bytes of L | status |
|---|---|---|---|
| `long2_exp_<v>`@0x2271 | AL32UTF8 | `0a 00` + `d0 b4 d0 b6 20 43 61 66 c3 a9` (the database bytes) | confirmed (2 versions) |
| `long2_exp_us7ascii_<v>`@0x2271 | US7ASCII (header bytes 1-2 `00 01`) | `0a 00` + `d0 b4 d0 b6 20 43 61 66 c3 a9`: **the same bytes, not converted** (US7ASCII would have given `?` or 0x3f) | confirmed (2 versions) |

`cmp -l long2_exp_<v>.dmp long2_exp_us7ascii_<v>.dmp`: the files differ only in the client set ids
(0x1-0x2, 0x30-0x31, R + 0xa2-0xa5), the header date and the file name (0x41, 0x7a, 0x94-0xa3); every
byte of both table sections is equal. So a LONG value is written in the **database** character set
whatever the client set, like VARCHAR2 (C1); its descriptor has no set id, so the reader takes the
database set of the header (H4). Status: confirmed for AL32UTF8 with 2 client sets on 2 versions; a
single-byte database: not in the fixtures (assumed the same, as for VARCHAR2 and the CLOB of
CL8MSWIN1251).

**Direct path.** The logs of `long2_exp_direct_<v>` and `long_exp_direct_<v>` say "About to export
specified tables via Direct Path ..." and list every table with its row count; unlike LOB tables (L6)
no LONG table is switched to the conventional path, and every binary part has path flag 1. Two
things change on the direct path:

1. the LONG / LONG RAW column is moved to the end of the `INSERT INTO` column list and of the
   descriptors (4.1);
2. a LONG / LONG RAW value is a **chain of pieces**: u16 LE n, then (n & 0x7fff) bytes; if bit 0x8000
   of n is set another piece follows, if it is clear this was the last piece. A short value is one
   piece with the bit clear, i.e. an ordinary field.

| offset (18c / 11g) | bytes | meaning | status |
|---|---|---|---|
| `long2_exp_direct_<v>`@0x2257 | `03 00`, `02 00 16 00`, `01 00 0a 00 69 03 01 00`, `08 00 00 00`, `00 00 01 00` | T_LONG2: ID, **V, L**; m 0, flag 1 | confirmed (2 versions) |
| @0x226d | `02 00 c1 02` `01 00 61` `0a 00 d0 b4 d0 b6 20 43 61 66 c3 a9` | row 1: ID, V, then L (one piece of 10 bytes); no `00 00` (direct path) | confirmed |
| @0x2280 | `ff ff 0a` | end of rows | confirmed |
| `long_exp_direct_<v>`@0x226a / 0x226b | `02 00 c1 02` `01 00 61` `05 00 73 68 6f 72 74` | T_LONG row 1: L = `short` (one piece) | confirmed |
| @0x2278 / 0x2279 | `02 00 c1 03` `01 00 62` `fe ff` | row 2: L NULL = `fe ff` | confirmed |
| @0x2288 / 0x2289 | `9c 80` + 156 bytes (`4c` + 155 x `78`) | row 3: L, first piece: 0x809c = 0x8000 + 156 | confirmed |
| @0x2326 / 0x2327, 0x4241 / 0x4242, 0x615c / 0x615d | `19 9f` + 7961 bytes each | pieces 2 to 4: 0x9f19 = 0x8000 + 7961 | confirmed |
| @0x8077 / 0x8078 | `19 1f` + 7961 bytes | last piece: 0x1f19 = 7961, bit clear; 156 + 4 x 7961 = 32,000 = the SQL value | confirmed |
| @0x9f92 / 0x9f93 | `ff ff 0a` | end of rows (the value was the last column) | confirmed |
| @0xa0e2 / 0xa0e4 and @0xa0ea / 0xa0ec | `02 00 00 ff`; `9c 80` + 156, 3 x (`19 9f` + 7961), `19 1f` + 7961 (`ab` bytes) | T_LONGRAW rows 1 and 2: the same chain for LONG RAW (32,000 bytes) | confirmed |
| @0x11ed0 / 0x11f35 | T_LONG_EMPTY: `02 00`, V, L, `00 00 01 00`; then `0a` (18c) / `ff ff 0a` (11g) | empty LONG table on the direct path (5.4) | confirmed |
| T_FRAME @0x11ee3 / 0x11f4a | 475 bytes = `t_frame_exp_direct_<v>`@0x210d (rule T8) | the walk is in step after the chains | confirmed (`tools/frame_t8.py`) |

| statement | evidence | status |
|---|---|---|
| On the direct path a LONG / LONG RAW value is a piece chain; the pieces add up to the value (32,000 = 156 + 4 x 7961 in all four long values); a piece boundary is not a record boundary (pieces cross 0x4000 ... 0x8000 with nothing inserted, `tools/boundary.py`) | table | confirmed (4 chains, 2 types, 2 versions) |
| The piece sizes: first 156, then 7961; why: unknown (156 = 32,000 - 4 x 7961, i.e. the remainder comes first; hypothesis). A reader must not assume any size | table | observed |
| On the conventional path the same 32,000-byte values are one ordinary field `00 7d` (session 7); `00 7d` has bit 0x8000 clear, so the conventional reading is the same rule with one piece | `long_exp_<v>`@0x2285 | confirmed for 32,000; a conventional value of 32,768 or more (bit 0x8000 of its length set): unknown (R-040). **Session 17**: one field up to 65,520 (bit 0x8000 part of the length), `fd ff` + chunks from 65,535; not the chain (see below) |
| The values decode to the SQL values on both paths (T_LONG 6 fields, T_LONGRAW 2, T_LONG2 2 per file) | `tools/check_truth.py` (by column name) | confirmed |
| LONG on the direct path in a single-byte database; a LONG RAW value above 32,000 bytes; a chain whose piece u16 would collide with `fe ff` / `ff ff` (a piece of 0x7ffe or 0x7fff bytes with the flag) | not in the fixtures | unknown |

**Session 10, note A-18 (what a piece u16 of a direct-path LONG chain can be).** Observed in the fixtures
(`tools/rowlen_stats.py`, all 201 files): the first u16 of a direct-path LONG / LONG RAW field is `fe ff` (NULL,
`long_exp_direct_18c.dmp`@0x227f), a one-piece length without the flag (`01 00`, `05 00`, `0a 00`, e.g.
`long_exp_direct_18c.dmp`@0x2271), or 0x809c (156 bytes + "more", @0x2288); later pieces are 0x9f19 (7961 + "more") and
0x1f19 (7961, last; `long_exp_direct_11g.dmp`@0x8078). The largest piece is 7,961 bytes. **Not in the fixtures**: a
piece u16 from 0xfff0 to 0xffff (a piece of 0x7ff0 bytes or more with the flag), a piece of length 0 with or without
the flag (0x8000, 0x0000), a first piece that has the flag and length 0, `fe ff` or `ff ff` in any position after the
first piece. Which reading wins for such a u16 (marker or piece) is therefore **unknown**; the fixtures neither support
nor contradict the design's choice (a later piece u16 from 0xfff0 = `Framing`; a piece of length 0 readable with a
warning). Hypothesis, not tested: pieces are bounded by an internal buffer below one 8 KB record (7,961 seen), so a
piece of 32 KB or more would not be written at all.

**Session 10, note A-22 (conventional LONG length from 0x8000).** No conventional LONG field of the corpus has a u16
from 0x8000 (largest 32,000 = `00 7d`, `long_exp_11g.dmp`@0x2289, row 3 @0x2285). Nothing in the fixtures makes one of R-040's readings
(one field with a 16-bit length, or the direct-path chain) more likely than the other; the LOB value area uses a third
mechanism (`fc ff` + 8-byte total), which is no evidence for LONG. So there is no reason from the files to prefer a
warning: stopping (the design's `UnsupportedLayout`) is consistent with what is known. Unknown until R-040.
**Session 17**: answered below; a conventional LONG length from 0x8000 to 0xfff0 is a plain length.

#### Session 17: LONG values of 32,767 to 100,000 bytes (R-040; `long3_exp(_direct)_<v>.dmp`)

T_LONG3 (`ID NUMBER(3)`, `L LONG`, `V VARCHAR2(10)`), loaded with SQL*Loader (route 1 of R-040; `long3_sqlldr_<v>.log`:
`7 Rows successfully loaded.`), then T_FRAME; `tables=t_long3,t_frame`, `statistics=none`. Each L is `4c` + the 6 ASCII
digits of its size + `79` x (size - 7); V = `v` + the size. `truth_long3.txt` (both versions identical) gives the
`GETLENGTH` 32,767, 32,768, 65,519, 65,520, 65,535, 65,536, 100,000 and the first and last 20 bytes. `tools/long3_check.py`
decodes every L and V of the four files and compares them with the truth and with the arithmetic: **0 differences**;
T_FRAME is byte-identical to `t_frame_exp_<v>.dmp` (conventional files) and `t_frame_exp_direct_<v>.dmp` (direct files)
in all four files (rule T8).

**Conventional path** (path flag 0 at `long3_exp_<v>`@0x226b; column order of `INSERT INTO` and the descriptors: ID, L,
V, as declared; the 18c and 11g files have the same bytes from 0x2000 on, `cmp -l`: 22 header bytes differ).

| offset (hex) | bytes | meaning | evidence | status |
|---|---|---|---|---|
| 0x2257 | `03 00`, `02 00 16 00`, `08 00 00 00`, `01 00 0a 00 69 03 01 00`, `00 00 00 00` | 3 columns: NUMBER, LONG (length 0), VARCHAR2(10); m 0, flag 0 | INSERT line @0x221c `("ID", "L", "V")` | confirmed (2 versions) |
| 0x2271 | `ff 7f` + 32,767 bytes | row 1: **one field**, length 0x7fff | truth row 1 (32,767); V @0xa272 `06 00 76 33 32 37 36 37`, row end `00 00` @0xa27a | confirmed (2 versions) |
| 0xa280 | `00 80` + 32,768 bytes | row 2: **one field**, length 0x8000 = 32,768: bit 0x8000 is part of the length, not a flag | truth row 2; V @0x12282 `v32768` | confirmed (2 versions) |
| 0x12290 | `ef ff` + 65,519 bytes | row 3: one field, length 0xffef | truth row 3; V @0x22281 `v65519` | confirmed (2 versions) |
| 0x2228f | `f0 ff` + 65,520 bytes | row 4: **one field, length 0xfff0 = 65,520** (a value that R8 lists as "never occurs") | truth row 4; V @0x32281 `06 00` + `v65520` directly after the 65,520 bytes | confirmed (2 versions) |
| 0x3228f | `fd ff`, then 16 x (`a0 0f` + 4000 bytes), `ff 05` + 1535 bytes, `00 00` | row 5 (65,535): **marker `fd ff`, chunks `u16 n + n bytes`, ended by a chunk of length 0**; 16 x 4000 + 1535 = 65,535 | truth row 5; chunk u16 at 0x32291, ..., 0x41cb1 `ff 05`; `00 00` @0x422b2; V @0x422b4 `v65535` | confirmed (2 versions) |
| 0x422c2 | `fd ff`, 16 x (`a0 0f` + 4000), `00 06` + 1536, `00 00` | row 6 (65,536): 64,000 + 1,536 | truth row 6; `00 06` @0x51ce4; `00 00` @0x522e6; V @0x522e8 `v65536` | confirmed (2 versions) |
| 0x522f6 | `fd ff`, 25 x (`a0 0f` + 4000), `00 00` | row 7 (100,000): 25 x 4000, no shorter last chunk; the `00 00` still ends the value | truth row 7; `00 00` @0x6a9ca; V @0x6a9cc `07 00` + `v100000`; row end `00 00` @0x6a9d5 | confirmed (2 versions) |
| 0x6a9d7 | `ff ff 0a` | end of rows of T_LONG3; T_FRAME follows @0x6a9da | rule E1 | confirmed |

**Direct path** (path flag 1 at `long3_exp_direct_<v>`@0x226b; the LONG column moved last: ID, V, L, as in session 9).
Every L is a piece chain (R2): u16 n, (n & 0x7fff) bytes, another piece while bit 0x8000 is set.

| row (size) | V @ | L @ (18c = 11g) | pieces 18c | pieces 11g | status |
|---|---|---|---|---|---|
| 1 (32,767) | 0x2271 | 0x2279 | `9b 83` (923, more), 3 x `19 9f` (7961, more), `19 1f` (7961, last): 923 + 4 x 7961 | 4 x `10 9f` (7952, more), `bf 03` (959, last): 4 x 7952 + 959 | confirmed; **the two versions cut the same value differently** |
| 2 (32,768) | 0xa286 | 0xa28e | 4 x `10 9f` (7952), `c0 03` (960) | same | confirmed (2 versions) |
| 3 (65,519) | 0x1229c | 0x122a4 | 8 x 7952, `6f 07` (1903) | same | confirmed (2 versions) |
| 4 (65,520) | 0x222a9 | 0x222b1 | 8 x 7952, `70 07` (1904) | same | confirmed (2 versions) |
| 5 (65,535) | 0x322b7 | 0x322bf | 8 x 7952, `7f 07` (1919) | same | confirmed (2 versions) |
| 6 (65,536) | 0x422d4 | 0x422dc | 8 x 7952, `80 07` (1920) | same | confirmed (2 versions) |
| 7 (100,000) | 0x522f2 | 0x522fb | 12 x 7952, `e0 11` (4576) | same | confirmed (2 versions) |

End of rows `ff ff 0a` @0x6a9b5; T_FRAME @0x6a9b8. The 18 bytes in which the 18c and the 11g direct files differ are
the piece u16 values of row 1 and the bytes their different positions displace (0x2279-0x227a, 0x2616-0x2617,
0x418b-0x418c, 0x4531-0x4532, 0x609d-0x609e, 0x644c-0x644d, 0x7faf-0x7fb0, 0x8367-0x8368, 0x9ec1-0x9ec2): the value
bytes are the same, only the cut differs.

**Answers to R-040**

| question | 18c | 11g | status |
|---|---|---|---|
| (a) conventional, 32,768 bytes or more | one ordinary field whose u16 is the length, bit 0x8000 included (hypothesis A): `00 80` (32,768), `ef ff` (65,519), `f0 ff` (65,520); **not** the direct-path chain (hypothesis B refuted) | same bytes | confirmed (2 versions; 3 values from 0x8000) |
| (b) 65,520 and above 65,535 | 65,520 is still one field `f0 ff`. 65,535, 65,536 and 100,000 use a third form: `fd ff` + chunks `u16 n + n bytes` (4000 each, the remainder last) + `00 00`. Where the switch lies between 65,521 and 65,535, and whether 0xfff1 to 0xfffc ever stand as a length: **unknown** (R-048) | same bytes | confirmed for the 7 sizes; the boundary unknown |
| (c) direct, longer values | simply a longer chain, for every size (13 pieces for 100,000); piece u16 values seen 0x9f10, 0x9f19, 0x839b, 0x809c with the flag, last pieces 959 to 7961 without; none from 0xfff0, no piece of length 0 | same, except the cut of row 1 | confirmed |

| statement | evidence | status |
|---|---|---|
| Conventional LONG: one field `u16 n + n bytes` for 1 <= n <= 0xfff0; `fd ff` + chunks + `00 00` at least from 65,535 | table | confirmed (LONG, AL32UTF8, 2 versions; sizes 5 to 65,520 one field, 65,535 to 100,000 chunked) |
| The conventional chunk size is 4000 and the remainder comes last; a reader must not assume the size | rows 5 to 7 | observed (one size seen); why 4000: unknown |
| The `fd ff` form is the text region's "field in pieces" marker (rule I4) reused in row data; it ends with a chunk `00 00` there too | 4.2 (I4); rows 5 to 7 | hypothesis for the identity of the two (the chunk lengths differ: I4 pieces are not 4000); the ending `00 00` is confirmed for row data |
| LONG RAW above 32,000 bytes on either path | not in the fixtures | unknown: assumed to be framed like LONG (session 9 saw the same framing for both up to 32,000), R-048 |
| Direct-path piece sizes: 18c cuts 32,767 as 923 + 4 x 7961 (remainder first, like 156 + 4 x 7961 for 32,000 in session 9) and every larger value as 7952 x k + remainder last; 11g cuts every T_LONG3 value as 7952 x k + remainder | table | observed; the rule behind the sizes: unknown. A reader uses only the flag |
| The value `V` after every LONG and the table after T_LONG3 are intact on both paths | `tools/long3_check.py`: 7 V values = truth; T_FRAME IDENTICAL x 4 | confirmed |

#### Session 19: LONG of 65,521 to 65,535 bytes; LONG RAW of 32,768 to 100,000 bytes (R-048)

**LONG** (`long4_exp(_direct)_<v>.dmp`, 18c and 11g). T_LONG4 (`ID NUMBER(3)`, `L LONG`, `V VARCHAR2(10)`), loaded with
SQL*Loader (`long4_sqlldr_<v>.log`: `7 Rows successfully loaded.`), then T_FRAME; `tables=t_long4,t_frame`,
`statistics=none`. L = `4c` + 6 ASCII digits of the size + `79` x (size - 7), V = `v` + the size (R-040 generator).
`truth_long4.txt` (18c = 11g) gives `GETLENGTH` 65,521, 65,525, 65,530, 65,532, 65,533, 65,534, 65,535 and the first and
last 20 bytes. `tools/long4_check.py` decodes every L and V of the four files and compares them with the truth and with
the arithmetic: **0 differences**; T_FRAME is byte-identical to `t_frame_exp_<v>.dmp` / `t_frame_exp_direct_<v>.dmp` in
all four files (T8). The 18c and 11g files are byte-identical from 0x2000 on (conventional and direct).

Conventional path (descriptors @0x2257 `03 00`, `02 00 16 00`, `08 00 00 00`, `01 00 0a 00 69 03 01 00`, flag
`00 00 00 00`; rows in the order ID, L, V; offsets 18c = 11g):

| row (size) | row @ | L @ | first bytes of L | form | end of L / V @ | status |
|---|---|---|---|---|---|---|
| 1 (65,521) | 0x226d | 0x2271 | `f1 ff` | **one field**, u16 0xfff1 = 65,521; value @0x2273 `4c 30 36 35 35 32 31 79` | V @0x12264 `06 00` `v65521`, row end `00 00` @0x1226c | confirmed (2 versions) |
| 2 (65,525) | 0x1226e | 0x12272 | `f5 ff` | **one field**, u16 0xfff5 = 65,525 | V @0x22269 `v65525` | confirmed (2 versions) |
| 3 (65,530) | 0x22273 | 0x22277 | `fa ff fa ff 00 00` | **new form**: marker `fa ff`, then 4 bytes `fa ff 00 00` = 65,530, then 65,530 value bytes @0x2227d | value ends @0x32276; V @0x32277 `v65530` | confirmed (2 versions) |
| 4 (65,532) | 0x32281 | 0x32285 | `fa ff fc ff 00 00` | `fa ff` + `fc ff 00 00` (65,532) + bytes @0x3228b | V @0x42287 `v65532` | confirmed (2 versions) |
| 5 (65,533) | 0x42291 | 0x42295 | `fa ff fd ff 00 00` | `fa ff` + `fd ff 00 00` (65,533) + bytes @0x4229b | V @0x52298 `v65533` | confirmed (2 versions) |
| 6 (65,534) | 0x522a2 | 0x522a6 | `fd ff a0 0f` | **chunked** (R-040 form): `fd ff`, 16 x (`a0 0f` + 4000), `fe 05` + 1534 (@0x61cc8), `00 00` (@0x622c8) | V @0x622ca `v65534` | confirmed (2 versions) |
| 7 (65,535) | 0x622d4 | 0x622d8 | `fd ff a0 0f` | chunked: 16 x 4000, `ff 05` + 1535 (@0x71cfa), `00 00` (@0x722fb) | V @0x722fd `v65535`; row end @0x72305; `ff ff 0a` @0x72307 | confirmed (2 versions) |

Direct path (flag `01 00` @0x226b; the LONG column last: ID, V, L, as in sessions 9 and 17): every L is a chain of 8
pieces `10 9f` (7952 with the flag) and a last piece of 1905, 1909, 1914, 1916, 1917, 1918, 1919 bytes (`71 07` ...
`7f 07`); L of row 1 @0x2279, row 7 @0x62304; end `ff ff 0a` @0x72315; 18c = 11g (here the two versions cut every
value the same way).

| statement | evidence | status |
|---|---|---|
| The R-048 prediction "one field up to some n, the `fd ff` form above n" is **refuted** as stated: there is a third conventional form between them. Lengths 65,521 and 65,525 are one field (`f1 ff`, `f5 ff`); 65,530, 65,532 and 65,533 are `fa ff` + a 4-byte length + the bytes; 65,534 and 65,535 are `fd ff` + chunks + `00 00` | table | confirmed (7 sizes, 2 versions) |
| The 4 bytes after `fa ff` hold the length as u16 LE followed by `00 00` in all 3 values. Read as one u32 LE, they give the same lengths | @0x22279, 0x32287, 0x42297 | the u16 + `00 00` reading: observed (3 values, 2 versions); u32 LE: hypothesis (the upper half is never non-zero, it cannot be at these sizes) |
| The switch from one field to `fa ff` lies between 65,526 and 65,530 (0xfff6 to 0xfffa); `fa ff` is used at least for 65,530 to 65,533; `fd ff` from 65,534 (session 17's "from 65,535" is lowered to 65,534) | rows 2, 3, 6 | confirmed for the bounds; the exact switch: unknown (65,526 to 65,529 and 65,531 are not in the fixtures; R-051). **Session 20**: between 65,529 and 65,530, confirmed (`long5_exp_<v>`@0x32286, 0x4228f) |
| Hypothesis that fits all 14 conventional sizes of `long3` and `long4`: a length n is written as one field when n <= 0xfff9, as `fa ff` + n when 0xfffa <= n <= 0xfffd (n would otherwise be read as a marker: `fa`, `fb`, `fc` (LOB long value), `fd` (chunks)), and in chunks when n >= 0xfffe (n does not fit or would be NULL / end marker) | `long3_exp_<v>`, `long4_exp_<v>` | hypothesis (R-051 tests 65,526 to 65,529 and 65,531). **Session 20**: confirmed for every size in a fixture (LONG 65,525 to 65,535 and LONG RAW 11 sizes, 2 versions); 65,522 to 65,524: hypothesis |
| The chunked form for 65,534: 16 chunks of 4000 and the remainder 1534 last, ended by `00 00`, as in session 17 | row 6 | confirmed |
| `V` after every LONG and the table after T_LONG4 are intact on both paths | `tools/long4_check.py`: 7 V = truth per file; T_FRAME IDENTICAL x 4 | confirmed |

**LONG RAW** (`longraw4_exp(_direct)_18c.dmp`, 18c only). T_LONGRAW4 (`ID NUMBER(3)`, `LR LONG RAW`), filled with
python-oracledb (`DB_TYPE_LONG_RAW`; the bind script is in the owner's ORADMP_WORK folder), then T_FRAME. Four values,
every byte `ab`: 32,768, 65,520, 65,535, 100,000 bytes (`truth_longraw4.txt`: `GETLENGTH` and the first / last 10 bytes,
`ABAB...`). Descriptors @0x224b `02 00`, `02 00 16 00`, `18 00 00 00`; flag `00 00 00 00` (conventional) / `00 00 01 00`
(direct). `tools/long4_check.py`: every value = truth and = n x `ab`, 0 differences; T_FRAME IDENTICAL in both files.

| row (size) | conventional: LR @, form | direct: LR @, chain | status |
|---|---|---|---|
| 1 (32,768) | @0x225d `00 80` + 32,768 bytes: **one field**, row end `00 00` @0xa25f | @0x225d 4 x `10 9f` (7952) + `c0 03` (960) | confirmed (18c) |
| 2 (65,520) | @0xa265 `f0 ff` + 65,520 bytes: one field | @0xa26b 8 x 7952 + `70 07` (1904) | confirmed (18c) |
| 3 (65,535) | @0x1a25d `fd ff`, 16 x (`a0 0f` + 4000), `ff 05` + 1535 (@0x29c7f), `00 00` (@0x2a280): chunked | @0x1a271 8 x 7952 + `7f 07` (1919) | confirmed (18c) |
| 4 (100,000) | @0x2a288 `fd ff`, 25 x (`a0 0f` + 4000), `00 00` (@0x4295c): chunked; `ff ff 0a` @0x42960 | @0x2a286 12 x 7952 + `e0 11` (4576); `ff ff 0a` @0x42940 | confirmed (18c) |

| statement | evidence | status |
|---|---|---|
| A conventional LONG RAW uses the LONG forms: one field with the u16 length (bit 0x8000 part of it) up to 65,520, `fd ff` + 4000-byte chunks + `00 00` for 65,535 and 100,000; the u16 values and chunk sizes are those of the LONG values of the same sizes in `long3_exp_<v>` | table; 6.6 session 17 | confirmed (18c, 4 sizes) |
| The direct-path LONG RAW chains have the same piece sizes as the LONG chains of the same size (32,768: 4 x 7952 + 960 as `long3_exp_direct_<v>` row 2; 65,535: 8 x 7952 + 1919; 100,000: 12 x 7952 + 4576) | table; session 17 table | confirmed (18c) |
| The `fa ff` form for a LONG RAW (65,526 to 65,533 bytes) | not in the fixtures | hypothesis: as for LONG (every LONG RAW form seen equals the LONG form of the same size) |
| LONG RAW on 11g | not in the fixtures (python-oracledb thin mode needs 12.1+) | hypothesis: as on 18c, because every 11g LONG file of R-040 and R-048 is byte-identical to its 18c file from 0x2000 on, and LONG and LONG RAW were framed the same on both versions up to 32,000 bytes (session 9, `long_exp_<v>`); R-051 asks for it |

(Session 20: the last two rows are answered below: the `fa ff` form for LONG RAW is **confirmed** at 65,530, 65,531 and
65,533 bytes, and LONG RAW on 11g is **confirmed** equal to 18c.)

#### Session 20: LONG of 65,526 to 65,531 bytes; LONG RAW of 65,521 to 65,534 bytes; LONG RAW on 11g (R-051)

New fixtures (owner, 2026-10-25, `corpus/scripts/make_pending3.sh`; Oracle 18c XE 18.4 and 11g XE 11.2.0.2, table mode,
`statistics=none`, database and client AL32UTF8; uncommitted at the time of this session):

| fixture | version, path | table, rows (sizes in bytes) | loaded by |
|---|---|---|---|
| `long5_exp_<v>.dmp`, `long5_exp_direct_<v>.dmp` | 18c and 11g; conventional / direct | T_LONG5 (`ID NUMBER(3)`, `L LONG`, `V VARCHAR2(10)`): 6 rows, 65,526 to 65,531; then T_FRAME | SQL*Loader (`long5_sqlldr_<v>.log`: `6 Rows successfully loaded.`) |
| `longraw5_exp_18c.dmp`, `longraw5_exp_direct_18c.dmp` | 18c; conventional / direct | T_LONGRAW5 (`ID NUMBER(3)`, `LR LONG RAW`): 7 rows of `ab`: 65,521, 65,525, 65,529, 65,530, 65,531, 65,533, 65,534; then T_FRAME | python-oracledb thin mode |
| `longraw5_exp_11g.dmp`, `longraw5_exp_direct_11g.dmp` | 11g; conventional / direct | T_LONGRAW5: the same 7 rows plus ids 8 to 11 = 32,768, 65,520, 65,535, 100,000; then T_FRAME | SQL*Loader with `LONG VARRAW(200000)` (`longraw5_sqlldr_varraw_11g.log`: `11 Rows successfully loaded.`) |

The logs say `T_LONG5 6 rows exported`, `T_LONGRAW5 7 rows exported` (18c), `T_LONGRAW5 11 rows exported` (11g),
`T_FRAME 7 rows exported`, "via Conventional Path" / "via Direct Path". Truth: `truth_long5.txt` (18c = 11g:
`GETLENGTH`, first and last 20 bytes, V), `truth_longraw5.txt` (`GETLENGTH`, first and last 10 bytes). `tools/long5_check.py`
decodes every L / LR and V of the eight files and compares them with the truth and with the arithmetic (`L` + 6 digits
+ `y` x (n - 7); n x `ab`): **0 differences**; T_FRAME is byte-identical to `t_frame_exp_<v>.dmp` (conventional) and
`t_frame_exp_direct_<v>.dmp` (direct) in all eight files (rule T8). `long5_exp(_direct)_18c.dmp` and its 11g file are
byte-identical from 0x2000 on (0 differing bytes; 22 header bytes below 0x2000 differ, as in every pair).

The 11g LONG RAW records of the two routes: SQL*Loader with the value as hex text in a `CHAR(200000)` field was refused
on all 11 records with `ORA-01461: can bind a LONG value only for insert into a LONG column` (`longraw5_sqlldr_11g.log`
lines 29 to 60; `0 Rows successfully loaded.`, line 64); SQL*Loader with `LONG VARRAW` (binary value with a 4-byte length
in front, data file in `"var 7"` record format, `longraw5_sqlldr_varraw_11g.log` lines 8, 27) loaded all 11 (line 32). So
`LONG VARRAW` is a working route for LONG RAW values above 32,000 bytes on 11g XE (where the thin python-oracledb route
does not exist).

**(a) LONG, conventional path** (descriptors @0x2257 `03 00`, `02 00 16 00`, `08 00 00 00`, `01 00 0a 00 69 03 01 00`;
flag `00 00 00 00` @0x2269; rows ID, L, V; offsets 18c = 11g):

| offset (hex) | first bytes | meaning | evidence | status |
|---|---|---|---|---|
| 0x2271 | `f6 ff 4c 30 36 35 35 32` | row 1 (65,526): **one field**, u16 0xfff6 = 65,526; value @0x2273 | truth row 1; V @0x12269 `06 00` `v65526`, row end `00 00` @0x12271 | confirmed (2 versions) |
| 0x12277 | `f7 ff 4c ...` | row 2 (65,527): one field, 0xfff7 | truth row 2; V @0x22270 `v65527` | confirmed (2 versions) |
| 0x2227e | `f8 ff 4c ...` | row 3 (65,528): one field, 0xfff8 | truth row 3; V @0x32278 `v65528` | confirmed (2 versions) |
| 0x32286 | `f9 ff 4c ...` | row 4 (65,529): **one field, 0xfff9**: the largest one-field length | truth row 4; V @0x42281 `v65529` | confirmed (2 versions) |
| 0x4228f | `fa ff fa ff 00 00 4c 30` | row 5 (65,530): `fa ff` + `fa ff 00 00` + 65,530 bytes @0x42295 (the control of R-051; = `long4_exp_<v>`@0x22277) | truth row 5; V @0x5228f `v65530` | confirmed (2 versions) |
| 0x5229d | `fa ff fb ff 00 00 4c 30` | row 6 (65,531): `fa ff` + `fb ff 00 00` + 65,531 bytes @0x522a3 | truth row 6; V @0x6229e `v65531`; row end @0x622a6; `ff ff 0a` @0x622a8 | confirmed (2 versions) |

**(a) LONG, direct path** (flag `00 00 01 00` @0x2269; rows ID, V, L): every L is 8 pieces `10 9f` (7952, more) + a last
piece of 1910 to 1915 bytes (`76 07` ... `7b 07`): L @0x2279, 0x1228d, 0x222a2, 0x322b8, 0x422cf, 0x522e7; end `ff ff 0a`
@0x622f4; 18c = 11g (confirmed, 2 versions; the piece sizes are those of every other value from 32,768 bytes, 6.6
session 17).

**(b) LONG RAW, 18c, and (c) the same rows on 11g** (descriptors @0x224b `02 00`, `02 00 16 00`, `18 00 00 00`; flag
`00 00 00 00` / `00 00 01 00` @0x2255; offsets 18c = 11g for ids 1 to 7):

| id (size) | conventional: LR @, first bytes, form | direct: LR @, chain | status |
|---|---|---|---|
| 1 (65,521) | 0x225d `f1 ff ab ab`: one field 0xfff1 | 0x225d 8 x `10 9f` + `71 07` (1905) | confirmed (2 versions) |
| 2 (65,525) | 0x12256 `f5 ff`: one field 0xfff5 | 0x12264 8 x 7952 + 1909 | confirmed (2 versions) |
| 3 (65,529) | 0x22253 `f9 ff`: one field 0xfff9 | 0x2226f 8 x 7952 + 1913 | confirmed (2 versions) |
| 4 (65,530) | 0x32254 `fa ff fa ff 00 00`, value @0x3225a | 0x3227e 8 x 7952 + 1914 | confirmed (2 versions) |
| 5 (65,531) | 0x4225a `fa ff fb ff 00 00`, value @0x42260 | 0x4228e 8 x 7952 + 1915 | confirmed (2 versions) |
| 6 (65,533) | 0x52261 `fa ff fd ff 00 00`, value @0x52267 | 0x5229f 8 x 7952 + 1917 | confirmed (2 versions) |
| 7 (65,534) | 0x6226a `fd ff`, 16 x (`a0 0f` + 4000), `fe 05` + 1534 (@0x71c8c), `00 00` (@0x7228c); row end @0x7228e | 0x622b2 8 x 7952 + 1918 | confirmed (2 versions) |
| 8 (32,768), 11g only | 0x72294 `00 80`: one field | 0x722c6 4 x 7952 + 960 | confirmed (11g; = `longraw4_exp(_direct)_18c` row 1) |
| 9 (65,520), 11g only | 0x7a29c `f0 ff`: one field | 0x7a2d4 8 x 7952 + 1904 | confirmed (11g; = `longraw4` row 2) |
| 10 (65,535), 11g only | 0x8a294 `fd ff`, 16 x 4000 + 1535 + `00 00` | 0x8a2da 8 x 7952 + 1919 | confirmed (11g; = `longraw4` row 3) |
| 11 (100,000), 11g only | 0x9a2bf `fd ff`, 25 x 4000 + `00 00`; `ff ff 0a` @0xb2997 | 0x9a2ef 12 x 7952 + 4576; `ff ff 0a` @0xb29a9 | confirmed (11g; = `longraw4` row 4) |

End of rows on 18c: `ff ff 0a` @0x72290 (conventional), @0x722c2 (direct). Comparison of the files (`tools/long5_check.py`,
`cross_11g`): `longraw5_exp_18c.dmp` and `longraw5_exp_11g.dmp` are byte-identical from 0x2000 to 0x7228f (the table
section text, the descriptors and rows 1 to 7; direct: 0x2000 to 0x722c1); there the 18c file has `ff ff` and the 11g
file row 8 (`02 00 c1 09 00 80`, 0x72290). Every encoded field of 11g ids 8 to 11 (from its first u16 to its last byte)
is byte-identical to the field of `longraw4_exp_18c.dmp` / `longraw4_exp_direct_18c.dmp` ids 1 to 4.

| statement | evidence | status |
|---|---|---|
| R-051 (a): 65,526 to 65,529 bytes are one field `f6 ff` to `f9 ff`; 65,530 and 65,531 are `fa ff` + `fa ff 00 00` / `fb ff 00 00` + the bytes. The prediction of R-051 holds | (a) table | confirmed (LONG, 2 versions) |
| The switch from one field to `fa ff` lies between 65,529 and 65,530 (0xfff9 / 0xfffa) | rows 4, 5 | confirmed (LONG 2 versions; LONG RAW 2 versions) |
| Session 19's hypothesis "one field for n <= 0xfff9, `fa ff` + n for 0xfffa <= n <= 0xfffd, chunks for n >= 0xfffe" fits every conventional size of `long3`, `long4`, `long5`, `longraw4`, `longraw5` (LONG: 65,519 to 65,521 and every size from 65,525 to 65,535; LONG RAW: 11 sizes). The three sizes 65,522 to 65,524 (0xfff2 to 0xfff4) are not in any fixture | the tables of 6.6 sessions 17, 19, 20 | confirmed for every size in a fixture; 65,522 to 65,524: hypothesis (one field, between seen one-field sizes on both sides) |
| R-051 (b): a conventional LONG RAW uses exactly the LONG forms, with the same u16 values: one field `f1 ff`, `f5 ff`, `f9 ff`; `fa ff` at 65,530, 65,531, 65,533 (also the 4 bytes after it); `fd ff` + 16 x 4000 + 1534 + `00 00` at 65,534 (= `long4_exp_<v>` row 6, the same chunk u16 values) | (b) table | confirmed (18c and 11g) |
| R-051 (c): 11g writes LONG RAW like 18c byte for byte from 0x2000 on, for every common size, on both paths | comparison above | confirmed (11 sizes, both paths) |
| On the direct path every LONG RAW value is the chain of the LONG value of the same size (8 x 7952 + remainder from 65,520; 4 x 7952 + 960 at 32,768); 18c and 11g cut the same | (b) table; 6.6 session 17 | confirmed (2 versions) |
| The 4 bytes after `fa ff` are u16 n + `00 00` for every n seen (0xfffa, 0xfffb, 0xfffc, 0xfffd; 0xfffc for LONG only); as one u32 LE they give the same n | @0x42291, 0x5229f (`long5`), 0x32256, 0x4225c, 0x52263 (`longraw5`); `long4_exp_<v>`@0x32287 | u16 + `00 00`: confirmed (4 values, 2 types, 2 versions); u32 LE: hypothesis (the upper half cannot be non-zero at these sizes) |
| `V` after every LONG and the table after T_LONG5 / T_LONGRAW5 are intact on both paths and versions | `tools/long5_check.py`: 6 V = truth per `long5` file; T_FRAME IDENTICAL x 8 | confirmed |

**Conventional LONG / LONG RAW sizes in the fixtures from 32,768 to 65,535 bytes** (after R-040, R-048, R-051; offsets
are the same on 18c and 11g unless a version is named; `X` = `X_exp_<v>.dmp`):

| size | 0x | LONG (18c and 11g): first bytes @ | LONG RAW: first bytes @ | form |
|---|---|---|---|---|
| 32,768 | 0x8000 | `00 80` `long3`@0xa280 | `00 80` `longraw4_exp_18c`@0x225d; `longraw5_exp_11g`@0x72294 | one field |
| 65,519 | 0xffef | `ef ff` `long3`@0x12290 | - | one field |
| 65,520 | 0xfff0 | `f0 ff` `long3`@0x2228f | `f0 ff` `longraw4_exp_18c`@0xa265; `longraw5_exp_11g`@0x7a29c | one field |
| 65,521 | 0xfff1 | `f1 ff` `long4`@0x2271 | `f1 ff` `longraw5`@0x225d | one field |
| 65,522 to 65,524 | 0xfff2 to 0xfff4 | not in the fixtures | not in the fixtures | hypothesis: one field |
| 65,525 | 0xfff5 | `f5 ff` `long4`@0x12272 | `f5 ff` `longraw5`@0x12256 | one field |
| 65,526 | 0xfff6 | `f6 ff` `long5`@0x2271 | not in the fixtures | one field |
| 65,527 | 0xfff7 | `f7 ff` `long5`@0x12277 | not in the fixtures | one field |
| 65,528 | 0xfff8 | `f8 ff` `long5`@0x2227e | not in the fixtures | one field |
| 65,529 | 0xfff9 | `f9 ff` `long5`@0x32286 | `f9 ff` `longraw5`@0x22253 | one field |
| 65,530 | 0xfffa | `fa ff fa ff 00 00` `long4`@0x22277, `long5`@0x4228f | `fa ff fa ff 00 00` `longraw5`@0x32254 | `fa ff` |
| 65,531 | 0xfffb | `fa ff fb ff 00 00` `long5`@0x5229d | `fa ff fb ff 00 00` `longraw5`@0x4225a | `fa ff` |
| 65,532 | 0xfffc | `fa ff fc ff 00 00` `long4`@0x32285 | not in the fixtures | `fa ff` |
| 65,533 | 0xfffd | `fa ff fd ff 00 00` `long4`@0x42295 | `fa ff fd ff 00 00` `longraw5`@0x52261 | `fa ff` |
| 65,534 | 0xfffe | `fd ff a0 0f` `long4`@0x522a6 | `fd ff a0 0f` `longraw5`@0x6226a | `fd ff` chunks |
| 65,535 | 0xffff | `fd ff a0 0f` `long3`@0x3228f, `long4`@0x622d8 | `fd ff a0 0f` `longraw4_exp_18c`@0x1a25d; `longraw5_exp_11g`@0x8a294 | `fd ff` chunks |

(`longraw4` exists for 18c only; `longraw5` rows 1 to 7 have the same offsets in both versions. Above 65,535: 65,536 and
100,000 bytes are `fd ff` chunks for LONG (`long3`) and 100,000 for LONG RAW (`longraw4_exp_18c`@0x2a288,
`longraw5_exp_11g`@0x9a2bf); sizes between them: hypothesis, chunks.)

First u16 of a conventional LONG / LONG RAW field, 0xfff0 to 0xfffe: **seen** 0xfff0, 0xfff1, 0xfff5 to 0xfff9 (lengths),
0xfffa (marker of the 4-byte form), 0xfffd (marker of the chunked form), 0xfffe (NULL). **Never seen**: 0xfff2 to 0xfff4
(their sizes are not in the fixtures; hypothesis: lengths) and 0xfffb, 0xfffc. For 0xfffb and 0xfffc the fixtures show
more than absence: the sizes they would be as a length (65,531, 65,532) are written `fa ff` + n (LONG both; LONG RAW
65,531), and every size from 65,525 to 65,535 is in a fixture, so no LONG of these two versions puts 0xfffb or 0xfffc
first (observed for every size of the range; that `exp` never writes them at all is the inference, not a byte).

### 6.7 INTERVAL, ROWID, UROWID (session 8, R-029; `misc_exp_<v>.dmp`)

T_INTERVAL (`ID NUMBER(3)`, `IYM INTERVAL YEAR(4) TO MONTH`, `IDS INTERVAL DAY(4) TO SECOND(6)`; 4 rows,
the fourth all NULL) and T_ROWID (`ID NUMBER(3)`, `RID ROWID`, `URID UROWID`; row 1 = the ROWID of a
row of T_FRAME, row 2 NULL). Conventional path; the same offsets in the 18c and the 11g file (the
ROWID values differ: they are physical addresses in two databases). `tools/misc_types.py`: 40
checks of the bytes and of the conversion rules below against `truth_misc.txt`, 0 differ.

| offset | bytes | meaning | truth (`truth_misc.txt`) | status |
|---|---|---|---|---|
| 0x2291 | `03 00`, `02 00 16 00`, `b6 00 05 00`, `b7 00 0b 00`, `00 00 00 00` | T_INTERVAL: 3 columns; NUMBER; type 182 length 5; type 183 length 11; m = 0 and flag 0 (rule T6) | `Typ=182 Len=5`, `Typ=183 Len=11` | confirmed (2 versions) |
| 0x22a7 | `05 00` `80 00 00 02 42` | row 1 IYM: one ordinary field (u16 length + bytes), the bytes = `DUMP` | `+0002-06`, `80,0,0,2,42` | confirmed |
| 0x22ae | `0b 00` `80 00 00 0a 48 5a 69 87 5b ca 00` | row 1 IDS | `+0010 12:30:45.123456`, `80,0,0,a,48,5a,69,87,5b,ca,0` | confirmed |
| 0x22c1, 0x22c8 | `7f ff ff fd 3a`; `7f ff ff ff 3c 3c 3c 80 00 00 00` | row 2 (negative values) | `-0003-02`; `-0001 00:00:00.000000` | confirmed |
| 0x22db, 0x22e2 | `80 00 00 00 3c`; `80 00 00 00 3c 3c 3c 80 00 00 00` | row 3 (zero) | `+0000-00`; `+0000 00:00:00.000000` | confirmed |
| 0x22f5, 0x22f7 | `fe ff`, `fe ff` | row 4: NULL as for every type (R3); `00 00` row end; `ff ff` @0x22fb | `NULL` | confirmed |
| 0x2453 | `03 00`, `02 00 16 00`, `45 00 17 00`, `d0 00 a0 0f`, `00 00 00 00` | T_ROWID: NUMBER; type 69 length 23; type 208 length 4000; m = 0, flag 0 | `Typ=69`, `Typ=208` | confirmed (2 versions) |
| 0x2469 | `17 00` `01 00 00 00 17` + `AAAR8lAAMAAAACHAAA` (18c) / `AAAE5IAAEAAAACGAAA` (11g) | row 1 RID: **23 bytes = 5 bytes + the 18 characters of `ROWIDTOCHAR`**, not the 10 bytes of `DUMP(rid)` | `AAAR8lAAMAAAACHAAA`, `Typ=69 Len=10: 0,1,1f,25,3,0,0,87,0,0` (18c) | confirmed as "ends with the text" (1 value per version); the 5 bytes `01 00 00 00 17`: unknown (the same in both values) |
| 0x2482 | `0d 00` `01 00 01 1f 25 00 0c 00 00 00 87 00 00` (18c) / `01 00 00 4e 48 00 04 00 00 00 86 00 00` (11g) | row 1 URID: 13 bytes = `DUMP(urid)` | `Typ=208 Len=13: 1,0,1,1f,25,0,c,0,0,0,87,0,0` | confirmed (1 value per version) |
| 0x2493 | `02 00 c1 03 fe ff fe ff 00 00`, `ff ff` @0x249d | row 2: both NULL; end of rows | `NULL` | confirmed |
| 0x24a0-0x2688 | 489 bytes | T_FRAME, byte-identical to `t_frame_exp_<v>`@0x210d-0x22f5 (rule T8) | `tools/lob_check.py --frame` | confirmed |

**Conversion rules** (b = the value bytes; tested on the 3 non-NULL values of each interval column and
the ROWID / UROWID of each version by `tools/misc_types.py`; the text is what `TO_CHAR` /
`ROWIDTOCHAR` printed):

- INTERVAL YEAR(p) TO MONTH (182, 5 bytes): Y = u32 BE b[0..4] - 0x80000000 (signed years), M =
  b[4] - 60 (signed months). Text: `-` if Y < 0 or M < 0, else `+`; |Y| with p digits (p = the
  year precision of the `CREATE TABLE` line, 4.7: `INTERVAL YEAR (4) TO MONTH `); `-`; |M| with 2
  digits. `80 00 00 02 42` = Y 2, M 6 = `+0002-06`; `7f ff ff fd 3a` = Y -3, M -2 = `-0003-02`;
  `80 00 00 00 3c` = `+0000-00`. Status: confirmed for these 3 values (2 versions). Both fields of a
  negative interval are negative (M = -2 for -3-2); stated for the one negative value only.
- INTERVAL DAY(p) TO SECOND(f) (183, 11 bytes): D = u32 BE b[0..4] - 0x80000000, H = b[4] - 60, Mi =
  b[5] - 60, S = b[6] - 60, N = u32 BE b[7..11] - 0x80000000 (nanoseconds, signed). Text: sign as
  above; |D| with p digits; space; `HH:MI:SS` with 2 digits each; `.` and the first f digits of |N|
  written with 9 digits (f = the fractional precision of the `CREATE TABLE` line). `80 00 00 0a 48 5a
  69 87 5b ca 00` = 10 d, 12 h, 30 min, 45 s, 123,456,000 ns = `+0010 12:30:45.123456`; `7f ff ff ff
  3c 3c 3c 80 00 00 00` = `-0001 00:00:00.000000`; zero = `+0000 00:00:00.000000`. Status: confirmed
  for these 3 values. Not tested: a negative value with non-zero hours, minutes, seconds or fraction
  (hypothesis: every field negative, so -1 02:03:04.5 would be `7f ff ff ff 3a 39 38 62 32 9b 00`),
  other precisions, values that round.
- ROWID (69): the field is 23 bytes; its last 18 bytes are the `ROWIDTOCHAR` text (extended ROWID,
  base-64 alphabet `A-Z a-z 0-9 + /`), which a reader can return as it is. The text equals the 10
  bytes of `DUMP(rid)` read as: object number = u32 BE b[0..4] (6 characters), relative file = u32
  BE b[4..8] >> 22 (3 characters), block = u32 BE b[4..8] & 0x3FFFFF (6 characters), row = u16 BE
  b[8..10] (3 characters), each written in base 64, most significant first: 0x11f25 = `AAAR8l`, 12 =
  `AAM`, 0x87 = `AAAACH`, 0 = `AAA`. Status: confirmed (one value per version; the arithmetic on
  `DUMP` only explains the text, the file holds the text). The 5 bytes before the text: unknown;
  a reader that meets other bytes there or a length other than 23 has a ROWID form outside the
  fixtures (restricted ROWID, a ROWID of another kind).
- UROWID (208): the field is the `DUMP` bytes. With first byte `01` (a physical ROWID, the only
  form in the fixtures) the 13 bytes are `01`, object u32 BE, relative file u16 BE, block u32 BE,
  row u16 BE: `01 | 00 01 1f 25 | 00 0c | 00 00 00 87 | 00 00` gives the same `AAAR8lAAMAAAACHAAA`
  as RID (11g: `AAAE5IAAEAAAACGAAA`). Status: confirmed as bytes (2 values); the conversion to text:
  consistent with `ROWIDTOCHAR(rid)` of the same row (the truth has no text of URID). A logical
  UROWID (index-organised table, other first byte) is not in the fixtures.

What a reader can do: read all four as ordinary fields (R2, R3); convert the intervals to their
signed fields (years / months; days / hours / minutes / seconds / nanoseconds) or to the text
above; return ROWID as its 18-character text and UROWID as bytes or, for first byte `01`, as the
same text. The precision needed for the interval text is only in the `CREATE TABLE` line.

#### Session 9: mixed negative intervals, other precisions, more ROWIDs, a logical UROWID (R-036)

`misc2_exp_<v>.dmp`: T_INTERVAL2 (`ID`, `IYM INTERVAL YEAR(9) TO MONTH`, `IDS INTERVAL DAY(9) TO
SECOND(9)`, `IDS0 INTERVAL DAY(2) TO SECOND(0)`), T_ROWID2 (`ID`, `RID ROWID`, `URID UROWID`; rows 1 to
3 the ROWIDs of three T_PROBE6 rows, row 10 RID NULL and URID the ROWID of the one row of the
index-organised table T_IOT), T_FRAME. The same offsets in both versions. `tools/misc_types.py`: 33
checks per version on `misc2`, 0 differ (with `misc`: 106 checks, 0 differ).

| offset | bytes | `truth_misc2.txt` | meaning | status |
|---|---|---|---|---|
| 0x22c7 | `04 00`, `02 00 16 00`, `b6 00 05 00`, `b7 00 0b 00`, `b7 00 0b 00`, `00 00 00 00` | `Typ=182 Len=5`, `Typ=183 Len=11` | descriptor lengths 5 and 11 also for YEAR(9) and DAY(9) / DAY(2) TO SECOND(9) / SECOND(0): the precision is not in the descriptor | confirmed (3 columns, 2 versions) |
| 0x22e1 | `05 00` `7f ff ff ff 31` | `-000000001-11` | IYM row 1: Y = -1, M = 0x31 - 60 = **-11** | confirmed |
| 0x22e8 | `0b 00` `7f ff ff ff 3a 39 38 62 32 9b 00` | `-000000001 02:03:04.500000000` | IDS row 1: D -1, H **-2**, Mi **-3**, S **-4**, N 0x62329b00 - 0x80000000 = **-500,000,000** | confirmed |
| 0x22f5 | `0b 00` `80 00 00 00 3c 3c 3b 80 00 00 00` | `-00 00:00:01` | IDS0 row 1: D 0, S **-1**: the sign of the text comes from a lower field; no `.` and no fraction when f = 0 | confirmed |
| 0x2308, 0x230f, 0x231c | `bb 9a c9 ff 47`; `bb 9a c9 ff 53 77 77 bb 9a c9 ff`; `80 00 00 63 53 77 77 80 00 00 00` | `+999999999-11`; `+999999999 23:59:59.999999999`; `+99 23:59:59` | row 2: the largest values (0x3b9ac9ff = 999,999,999) | confirmed |
| 0x232f, 0x2336, 0x2343 | `44 65 36 01 31`; `44 65 36 01 25 01 01 44 65 36 01`; `7f ff ff 9d 25 01 01 80 00 00 00` | `-999999999-11`; `-999999999 23:59:59.999999999`; `-99 23:59:59` | row 3: the smallest values; H = 0x25 - 60 = -23, Mi = S = 1 - 60 = -59, N = -999,999,999 | confirmed |
| 0x24af | `03 00`, `02 00 16 00`, `45 00 17 00`, `d0 00 a0 0f`, `00 00 00 00` | `Typ=69`, `Typ=208` | as in `misc` | confirmed |
| 0x24c3, 0x24f1, 0x251f | `17 00` `01 00 00 00 17` + 18 characters (`AAASLVAAMAAAAbbAAA`, `...AAB`, `...AAC` on 18c; `AAAE/3AAEAAAAezAAA` ... on 11g) | `ROWIDTOCHAR` of rows 1 to 3 | RID: the same 5-byte prefix `01 00 00 00 17` in all 6 new values (8 values in all, 2 databases, 3 objects) | confirmed as constant in the fixtures; meaning unknown (hypothesis: `17` = 23, the field length) |
| 0x24dc, 0x250a, 0x2538 | `0d 00` `01 00 01 22 d5 00 0c 00 00 06 db 00 00` ... | `DUMP(urid)`; `CAST(urid AS VARCHAR2(4000))` = the ROWID text | physical UROWID (first byte `01`); its text by the base-64 rule of session 8 equals the truth text in all 6 values | confirmed (the text is now truth, not only "consistent") |
| 0x254d | `fe ff` | `NULL` | RID row 10 NULL | confirmed |
| 0x254f | `0a 00` `02 04 03 00 15 b3 02 c1 02 fe` (18c) / `02 04 01 00 0a d3 02 c1 02 fe` (11g) | `*BAMAFbMCwQL+` (18c), `*BAEACtMCwQL+` (11g); `Typ=208 Len=10` | **logical UROWID**: first byte `02`, 10 bytes = `DUMP`. Text = `*` + the bytes after the first in base 64 (A-Z a-z 0-9 + /), 3 bytes to 4 characters, most significant first: `04 03 00` = `BAMA`, `15 b3 02` = `FbMC`, `c1 02 fe` = `wQL+` | confirmed for the text rule (1 value per version, `tools/misc_types.py`); the content of the bytes: hypothesis (`c1 02` = the primary key value 1 as a NUMBER, preceded by its length `02`; `04 03 00 15 b3` and the last `fe`: unknown) |

| statement | status |
|---|---|
| Every field of a negative interval is negative (the request's hypothesis): rows 1 and 3 of IYM, IDS, IDS0 | confirmed (6 values, 2 versions) |
| Text rule (Y10) with these values: sign `-` if any field is negative (row 1 IDS0: only S is negative), leading field with p digits (9 and 2), `HH:MI:SS`, and `.` + f digits only when f > 0 | confirmed (9 texts per version) |
| The descriptor lengths are 5 and 11 whatever the precision (4, 9; 2, 4, 9; fraction 0, 6, 9) | confirmed |
| A logical UROWID text whose tail is not a multiple of 3 bytes (padding?) | unknown (one length, 9 bytes) |

## 7. Trailer and `METRICST` / `METRICET`

Session 4: in owner mode the same lines are written with `METRICSU` / `METRICEU` and the last
one is `METRICEUG0` (`own`@0x6313); the list of sections is longer (section 4.5). On 11g the
trailer of `t_frame` is byte-identical (`t_frame_exp_11g.dmp`@0x22f6-0x23ef).

Text lines, each ending in LF. For CONV (same text in DIRECT, 14 bytes earlier):

| offset (hex) | length | meaning | evidence | status |
|---|---|---|---|---|
| 0x22f6 | 42 | `METRICSTreferential integrity constraints` | CONV@0x22f6 | observed |
| 0x2320 | 11 | `METRICET 7` | CONV@0x2320 | observed |
| 0x232b | 17 | `METRICSTtriggers` | CONV@0x232b | observed |
| 0x233c | 11 | `METRICET 7` | CONV@0x233c | observed |
| 0x2347 | 50 | `METRICSTbitmap, functional and extensible indexes` | CONV@0x2347 | observed |
| 0x2379 | 11 | `METRICET 7` | CONV@0x2379 | observed |
| 0x2384 | 27 | `METRICSTposttables actions` | CONV@0x2384 | observed |
| 0x239f | 11 | `METRICET 7` | CONV@0x239f | observed |
| 0x23aa | 38 | `METRICSTPost-inst procedural actions ` (trailing space) | CONV@0x23aa | observed |
| 0x23d0 | 11 | `METRICET 7` | CONV@0x23d0 | observed |
| 0x23db | 11 | `METRICETG0` | CONV@0x23db; last `METRICET` line of all 38 fixtures | observed; meaning unknown |
| 0x23e6 | 5 | `EXIT` | CONV@0x23e6 | observed |
| 0x23eb | 5 | `EXIT`; its LF is the last non-zero byte of the file | CONV@0x23eb; all 38 fixtures end with `EXIT` LF `EXIT` LF | confirmed (as the end of the data) |

### 7.1 What the number after `METRICET` is

| statement | evidence | status |
|---|---|---|
| One table per file: the number equals the row count of the table, in all five lines | `t_empty`@0x2237 `METRICET 0`; CONV@0x2320 `METRICET 7`; `t_nullmix`@0x234d `METRICET 8`; `t_probe6`@0xa5cb `METRICET 300`; also 22 (`t_len`), 33 (`t_num`), 11 (`t_date`), 10 (`t_text`); `truth_counts.txt` | confirmed |
| Several tables per file: the number is the row count of the **last** table only; the counts of the other tables are not written anywhere | `multi_framing` (7, 22, 8 rows): 15 lines `METRICET 8`, first @0x36a9; no `METRICET 7` or `METRICET 22` in the file. `multi_types` (33, 11, 6, 10 rows): 20 lines `METRICET 10`, first @0x2d3d. Session 3: `multi_empty` (7, 0, 22 rows): 15 lines `METRICET 22`, first @0x3593; `multi_empty2` (0, 7 rows): 10 lines `METRICET 7`, first @0x2420; no `METRICET 0` in either | confirmed |
| Several tables per file: the group of five `METRICST` / `METRICET` pairs is written once per table (3 groups for 3 tables, 4 for 4), after all table sections | `multi_framing`@0x367f, 0x3764, 0x3849 (start of the 3 groups); `multi_types`@0x2d13, 0x2dfd, 0x2ee7, 0x2fd1 | confirmed |
| The number is not a row count in general: it grows when DDL statements follow | `ddl` (T_PARENT 2 rows, T_DDL 3 rows): `METRICET 3` @0x2a97, `METRICET 4` @0x2b0a, `METRICET 5` @0x2c24, `METRICET 6` @0x2d60, `METRICET 8` @0x2e08 | observed (one fixture pair) |
| Inside the trailer, statements of a table stand in a group `TABLE "name"` LF, lines, `ENDTABLE` LF, placed between a `METRICST` line and its `METRICET` line. All items of these groups are lines (no field) | `ddl`@0x2af0 `TABLE "T_PARENT"` + `ENDTABLE` @0x2b01 (empty group, under "bitmap, functional and extensible indexes"); @0x2b96 `TABLE "T_DDL"` + `ALTER TABLE ... ADD CONSTRAINT "T_DDL_FK" FOREIGN KEY ...` @0x2ba4 + `ENDTABLE` @0x2c1b (under "referential integrity constraints"); @0x2d02 + `ANALCOMPUTE IS "T_DDL" ANALYZE  INDEX ...` @0x2d10 + `ENDTABLE` @0x2d57 (under `METRICSTDeferred analyze commands `, @0x2cdf); @0x2d8f + two `ALTER TABLE ... ENABLE CONSTRAINT` lines + `ENDTABLE` @0x2dff (under `METRICSTEnable enforce constraints `, @0x2d6b). Same in `ddl/direct` (10 bytes earlier) | observed (one fixture pair). `ENDTABLE` occurs only in the trailer, never after the rows of a table section. **Session 5: "all items are lines" is withdrawn.** The group under "bitmap, functional and extensible indexes" can hold a line `ANALSTATS C? "T" ("col" ) ` and its plain field (`stat_exp_18c.dmp`@0x4e15 `TABLE "T_STAT1"`, @0x4e25 line, @0x4e45 field of 211 bytes, @0x4f1b `ENDTABLE`; section 4.6). A reader walks the groups with the item rule, like everything else |
| In `ddl` the number after `METRICET` rises by 1 after the group with no statement, by 1 after each group with one statement and by 2 after the group with two statements | offsets above | observed; no rule derived |

Session 4, the same counter on the new files (`METRICET` in table mode, `METRICEU` in owner
mode); "+n" is the change against the line before:

| statement | evidence | status |
|---|---|---|
| When the sections after the tables start, the counter holds the row count of the **last table written**, 0 for an empty table | `own_tables` (O_EMPTY last): `METRICET 0` @0x3135; `own` (O_PARENT last, 2 rows): `METRICEU 3` @0x345a after "synonyms" with one `CREATE SYNONYM` (2 + 1); `ru_exp_cp1251_11g.dmp`: `METRICEU 2` after "synonyms" (2 rows, no synonym) | confirmed |
| A table whose export failed does not set it | `ru_exp_us7ascii_11g.dmp`: `METRICEU 309` after "synonyms" (R_TEXT) | observed |
| In owner mode the counter also runs **before** the tables and is overwritten by them: 8 (18c) or 12 (11g) after the first section, +1 for `CREATE SEQUENCE`, then 3 after the tables | `own`@0x21f7 `METRICEU 8`, @0x23c2 `METRICEU 9`, @0x345a `METRICEU 3`; `own_11g`@0x21f2 `12`, @0x23c4 `13`, @0x35eb `3` | observed |
| +1 per statement in these sections: sequence, synonym, view (line + field = 1), each procedure (+2 for two procedures, 8 items), foreign key, trigger (3 items = 1), each `ANALCOMPUTE` line, each `ENABLE CONSTRAINT` line | `own`: 3 -> 4 (views, @0x34d4) -> 6 (stored procedures, @0x5d90) -> 7 (referential integrity, @0x5ea5) -> 8 (triggers, @0x5f9e) ... -> 18 (statistics, @0x621e) -> 21 (Enable enforce constraints, @0x6307); `own_tables`: 0, 0, 1, 1, 1, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 5, 8 | observed |
| Not one per statement: the section "pre-schema procedural objects and actions" with one `PROCACTION` item adds 8 (18c) or 12 (11g); "post-schema procedural objects and actions" adds the same number with **no** item; a group `TABLE "T"` + `ENDTABLE` with no statement under "bitmap, functional and extensible indexes" adds 1 (table mode only: O_PARENT, T_PARENT, R_CLOB) | `own`@0x21f7 (8), @0x6140 (8 -> 16); `own_11g`@0x21f2 (12), @0x62d1 (8 -> 20); `own_tables`@0x318e group, `METRICET 1` @0x31a8; `ru_tables_exp_us7ascii_11g.dmp`@0x134c3 group, `METRICET 4` @0x134db | observed; **no rule fits all files** |
| One `ENDTABLE` can close several `TABLE` lines: under "statistics" / "Deferred analyze commands " every table with an `ANALCOMPUTE` line has its own `TABLE` line, and one `ENDTABLE` ends the list | `own`@0x615f `TABLE "O_CHILD"`, @0x616f, @0x61ba `TABLE "O_EMPTY"`, @0x61ca, @0x6215 `ENDTABLE`; `own_tables`@0x3568, 0x35c3, 0x361e; `own_11g`@0x62f0, 0x634b, 0x63a6 | confirmed. A reader must not count `TABLE` / `ENDTABLE` as pairs |
| The trigger and the foreign key stand in groups in table mode too | `own_tables`@0x32e4-0x33b8 (trigger under "triggers"), @0x3234-0x32bf | confirmed |

Session 5, the counter on the R-016 files: `stat_exp_18c.dmp`: `METRICET 0` @0x4dbc (T_STAT4, the
last table, has no rows), `METRICET 1` @0x4f24 after the group with the `ANALSTATS CS` item, and
the same `METRICET 1` @0x3614 in `stat_exp_none_18c.dmp`, where that group is empty; `METRICET 5`
@0x53c1 after the four `ANALCOMPUTE TS` lines (+1 each). `ru_stat_exp_us7ascii_est_11g.dmp`: 309
(R_TEXT), `METRICET 311` @0x2bf25 after two `ANALCOMPUTE TR` lines, `METRICET 312` @0x2bfa1 after one
`ENABLE CONSTRAINT` line. The items `ANALSTATS` + field inside the table sections do not change the
counter. Observed; fits the rows above.

Session 6, the counter on the R-020 files: `stc_exp_18c.dmp`: `METRICET 50` in the 25 lines of the
five groups (first @0x554e; T_STC5, the last table, has 50 rows), `METRICET 54` @0x5b36 after the
four `ANALCOMPUTE TS` lines (+1 each). The 14 `ANALSTATS` items inside the table sections and the
two `GRANT` lines do not change it. Observed; fits the rows above.

Hypothesis: the number is a running counter that is not reset between sections; it holds the
row count of the last table when the trailer starts and is increased by later sections that
write statements. **A reader must not use `METRICET` as the row count of a table.** The row count
of a table is known only by reading its rows to the end marker.

## 8. Conventional vs direct path

| difference | evidence | status |
|---|---|---|
| header: `D` LF after the first 3 bytes | section 2 | confirmed |
| per table: u16 flag after the descriptors is 1 instead of 0 | section 4 | confirmed |
| rows: no `00 00` after each row | section 5.1 | confirmed |
| table without rows: no `ff ff` on the direct path | section 5.4 | confirmed for **18c** (5 fixtures). Session 4: **not on 11g**, where `ff ff` is written on both paths (5 fixtures, section 13.2). Session 17: the cause is the missing segment: an 18c empty table **with** a segment gets `ff ff` on the direct path too (`seg_exp_direct_18c.dmp`@0x226a, 0x23ca), one without gets none (@0x24c7) (5.4) |
| Session 17: a LONG value: conventional = one field up to 65,520 bytes or `fd ff` + 4000-byte chunks + `00 00` (65,535 and more); direct = a piece chain with the flag 0x8000 for every size, the LONG column moved last | `long3_exp_<v>`@0x2228f, 0x3228f; `long3_exp_direct_<v>`@0x2279; section 6.6 | confirmed (2 versions) |
| Session 4: a file made with `direct=y` (header `D` LF) can contain a table written on the conventional path: path flag 0 and `00 00` after the rows | `ru_exp_direct_cp1251_11g.dmp`@0x0003 `D` LF; R_CLOB @0x25b3 `00 00` (read as the path flag, section 14.6), row 3 `fe ff 00 00` before `ff ff`; log: "Table R_CLOB will be exported in conventional path."; R_TEXT in the same file has flag `01 00` @0x4028 | observed (one LOB table, 2 files). The per-table flag decides, not the header. **Session 8: confirmed**: in `lob_exp_direct_<v>` (header `D` LF) T_LOB and T_LOB2 have flag 0 (@0x2409, 0x4db97 on 18c; @0x2403, 0x4df90 on 11g) and the k after each row (14.6), and the log says "Table T_LOB will be exported in conventional path." for both; T_FRAME in the same file has flag 1 (@0x4df16 on 18c) and is byte-identical to `t_frame_exp_direct_<v>` (6 LOB tables in 4 `direct=y` files of 2 versions in all, with `ru_exp_direct_cp1251_11g` and `ru_exp_direct_us7ascii_11g`; every log agrees) |
| Session 4: RECORDLENGTH in the header is 8192 on the direct path although 2048 was asked for | section 14.7 | confirmed (6 files) |
| value bytes, length fields, NULL marker, descriptors, text: identical | every pair: all fields decode to the same bytes on both paths (`tools/check_truth.py`); types VARCHAR2, CHAR, NVARCHAR2, NUMBER, DATE, TIMESTAMP, TSTZ, TSLTZ, RAW | confirmed for these types |
| items before and after the rows (DEFAULT, index, statistics, constraint, comment), fields in pieces: identical | `ddl` / `ddl/direct`, section 4.3 | confirmed (one fixture pair). Session 5: every item of `stat_exp_<v>` and `stat_exp_direct_<v>` is byte-identical (111 of 111, both versions), and of three `ru_stat_exp_*` pairs (93 of 93): table, column and index statistics included (4.6). Session 6: `stc_exp_<v>` / `stc_exp_direct_<v>`, 114 of 114 items identical on both versions, `GRANT` lines and the statistics items of tables with missing column items included (`stc_exp_18c.dmp`@0x27ca-0x2bd1 = `stc_exp_direct_18c.dmp`@0x2766-0x2b6d) |
| effect of the client character set: none on either path | section 2.1 | confirmed |
| row order: the same on both paths | `t_probe6` and `t_probe6/direct` both start with R00065 | observed |

## 9. Unknown

| offset (hex) | length | what is not explained | evidence | status |
|---|---|---|---|---|
| 0x0000 | 1 | `03` | all fixtures | unknown |
| 0x0029 | 7 | text lines `0`, `72`, `0` | CONV@0x0029 | unknown |
| 0x0032, 0x0034 | 2 + 2 | 873 and 2000: which character sets (database? national of client or database?) | section 2.1 | session 4: the first is the database character set (confirmed, 14.4). The second (2000): still unknown whether it is the national set of the client or of the database |
| 0x0036 | 10 | `00 01 00 00 00 00 00 00 00 00` | CONV@0x0036 | unknown |
| 0x0040-0x0042 | 3 | width and byte order of the file-name-length field; the `00` after it | CONV@0x0040 | unknown |
| 0x2000, 0x2066 | 2 x 102 | the two blocks, except the four character-set fields | CONV@0x2000 | unknown |
| 0x20d4-0x2102 | 47 | the strings `BYTE`, `UNUSED`, `2`, `INTERPRETED`, `DISABLE:ALL` and the two empty ones | CONV@0x20d4 | unknown |
| +6 of a character descriptor | 2 | 1 / 2 | section 4.1 | hypothesis only |
| 0x2277 | 2 | `00 00` before the path flag | every table | unknown |
| 0x2104 and trailer | - | `METRICST`, `METRICETG0`, the two `EXIT` lines; the exact rule of the `METRICET` number | section 7 | unknown |
| `ddl`@0x2878 | 1 | the letter `C` in front of `ALTER TABLE` in the line `CALTER TABLE` | `ddl`@0x2878, `ddl/direct`@0x286e | unknown |
| `ddl`@0x2885 | 2 | `fd ff`: read as "a value in pieces follows" (section 4.2); one example only | `ddl`@0x2885, `ddl/direct`@0x287b | session 4: confirmed (4.2, 4.5: six more examples per owner-mode file, pieces equal the truth's line lengths). Why CHECK constraints, PL/SQL and `PROCACTION` use it and comments do not: unknown |
| `own`@0x34d1 | 2 | (session 4) the `00 00` after the text of a view, before the LF; read as "end of pieces without marker" | `own`@0x34d1; `ru_exp_cp1251_11g.dmp`@0x1398b | hypothesis (one-piece views only) |
| `own`@0x2104 | 1 | (session 4) the letter `T` / `U` in `METRICST` / `METRICSU`, `METRICET` / `METRICEU` | `own_tables`@0x2104, `own`@0x2104 | hypothesis: the export mode (tables / users) |
| `own`@0x352b, 0x5ed1 | - | (session 4) the numbers in `CREATE FORMAT71 PROCEDURE` and `CREATE FORMAT92TRIG`; the leading piece `CREATE ` of a procedure that a trigger does not have | `own`@0x352b, 0x3556, 0x5ed1, 0x5ee8 | unknown |
| `own`@0x21f7, 0x6140 | - | (session 4) why the two "procedural objects and actions" sections add 8 (18c) or 12 (11g) to the counter | section 7.1, 13.6 | unknown |
| `ddl`@0x286f | 1 | (session 4) last argument of `SET_INDEX_STATS`: `6` on 18c, `0` on 11g | section 13.5 | unknown. Session 5: the same argument closes `SET_TABLE_STATS` and `SET_COLUMN_STATS`; it is `6` for statistics made by `DBMS_STATS` and `0` for `ANALYZE` on **both** versions (`stat_exp_18c.dmp`@0x293c / 0x435b, `stat_exp_11g.dmp`@0x293c / 0x432c), so it is not a version constant (13.5, 13.9). Hypothesis: flags that say how the statistics were made |
| `ANALSTATS` / `ANALCOMPUTE` | 2 | (session 4) the two letters: `I` in every line, then `S` or `R` | section 14.5 | first letter: hypothesis (index); second: fits "questionable statistics" and "client set differs from database set" equally. Session 5: first letter `T`, `I` or `C`, each always with the same block (confirmed as a description, 4.6); second letter: unchanged, the two conditions still coincide in all 46 files |
| `stat_exp_18c.dmp`@0x29aa | - | (session 5) `SREC.EAVS := 4` and `SREC.CHVALS := DBMS_STATS.CHARARRAY()` with an empty list (18c, 8 fields); `0` and `NULL` elsewhere | section 4.6, 13.9 | unknown |
| `stat_exp_11g.dmp`@0x4b3e | - | (session 5) 11g writes `EPC := 2` and the lists `0,0` / `0,1` for the columns of a table without rows, 18c `EPC := 0` | section 13.9 | unknown |
| `stat_exp_18c.dmp`@0x4e25 | - | (session 5) what makes an index item a `C` item in the trailer instead of an `I` item after `CREATE INDEX`; why the name is `SYS_C` + spaces | section 4.6 | hypothesis (system-named constraint) |
| real dump | - | (session 5) why 53 column items for 5 tables whose columns add up to 54 or 55 (no fixture) | section 4.6 | unknown (R-020) |
| `ru_tables_exp_us7ascii_11g.dmp`@0x11ae4-0x11aef, 0x11af6-0x11b61 | 12, 108 | (session 4) the bytes between descriptors and rows of a LOB table; the 108 / 106 bytes of a CLOB field | section 14.6 | **session 8: explained as framing** (m, flag, column entries; the field is a locator); still unknown: the 2 bytes `00 00` after each column entry, and the content of a locator |
| `ru_tables_exp_us7ascii_11g.dmp`@0x11ade | 2 | (session 4) `48 00` = 72 in the descriptor of a CLOB column | section 14.6 | unknown (72 for every CLOB, NCLOB and BLOB of 11 files; not a value length) |
| `lob_exp_<v>`@0x240e / 0x2408 etc. | 2 per column | (session 8) the 2 bytes `00 00` that end every column entry after the path flag of a LOB table | 14.6 | unknown (always `00 00`, 10 entries) |
| `lob_exp_<v>` locators | 102 to 1102 | (session 8) the bytes of a LOB locator field: on 11g they hold the data of a small LOB in-row (`lob_exp_11g.dmp`@0x2676: 1000 x `ab` inside the 1102-byte field of row 4), one byte per locator differs between two exports (+0x57 on 18c, +0x39 on 11g); the first u16 **BE** of the field is its length - 2 (`00 70` in a 114-byte field) | 14.6 | observed; not needed to read the values |
| `misc_exp_<v>`@0x246b | 5 | (session 8) the bytes `01 00 00 00 17` before the 18 characters of a ROWID value | 6.7 | unknown (one value per version, the same 5 bytes) |
| `lob_exp_<v>` long LOB values | 8 | (session 8) the 8 bytes after `fc ff`: the total byte length of the value, little-endian; whether u64 or u32 + 4 zero bytes | 14.6 | the length: confirmed (8 values); the width: unknown (no value of 4 GB) |
| last 2 bytes of a TSTZ value with a region zone | 2 | `86 48`, `d0 04`, `81 90` | `t_tstz`@0x22f8, 0x230f, 0x2326 | unknown (section 6.5) |
| trailer groups `TABLE` ... `ENDTABLE` | - | which statements go into the trailer and which after the rows; the rule of the `METRICET` number | section 7.1 | unknown |
| `nchar_exp_<v>`@0x228c | 2 | (session 7) descriptor length 16 of `CHAR(4 CHAR)`: "n x the largest character width of the set" is a hypothesis; `VARCHAR2(n CHAR)` in AL32UTF8 is not in the fixtures | section 4.1 | hypothesis (R-011) |
| `long_exp_<v>`@0x225c, 0xa0d6 | 2 | (session 7) the length 0 in the descriptors of LONG and LONG RAW; how a LONG value above 65,533 bytes is written; LONG on the direct path; the character set of a non-ASCII LONG value | section 6.6 | unknown (R-031) |
| any row field | 2 | (session 7) what a length 0 or `fd ff` would mean in row data: neither occurs in 219,043 fields | section 5.6 | unknown |

(The session 2 rows for `ddl`@0x24e4, 0x253e, 0x281c, 0x2a0b, 0x2a5a are explained now: they are
plain fields, section 4.2.)

Open questions:

1. ~~Are value bytes and DDL text converted to the client character set?~~ Answered for row data
   (session 3, sections 2.1, 6.3): **no**, with an AL32UTF8 database and a WE8MSWIN1252 or US7ASCII
   client the table section is byte-identical; the length counts database bytes.
   **Session 4, closed:** (a) text items (comments, CHECK conditions, view text) and identifiers
   **are** converted to the client set, with replacement characters where the client set lacks a
   character, and their length fields count client bytes (sections 14.2, 14.3; `own`@0x2ce3
   against `own_exp_w1252_18c.dmp`@0x2ce3); (b) with a CL8MSWIN1251 database the row data are
   CL8MSWIN1251 bytes for four client sets (14.1). Still open: (c) whether a reader should trust
   the descriptor id or the header's database set when they differ (they are equal in all 133
   files); (d) a database NCHAR set other than AL16UTF16; (e) PL/SQL source with non-ASCII text;
   (f) an `exp` client older than the database.
2. ~~Direct path, table without rows, followed by another table.~~ Answered (session 3, section
   5.4): no `ff ff`, the LF follows the path flag, then the next item. Still open: which of the
   rules E1 / E2 is right in general; an empty table that has its own statements after the data
   (R-012). **Session 4:** the answer of session 3 holds for 18c only; on 11g `ff ff` is written
   (5.4, 13.2). An empty table with its own statements: answered (5.4), both rules still agree.
   Open: whether the 18c behaviour comes from the version or from a table without a segment
   (R-017; **session 17: from the segment**, 5.4); E1 against E2 (not separated, and no longer the main question: see the rule in 5.4).
3. ~~A record boundary between the two bytes of a length field or of a NULL marker.~~ **Session 4,
   closed** (section 1.2, R-013): nothing is inserted; both versions, both paths, RECORDLENGTH
   8192 and 2048. Not tested: RECORDLENGTH 1024 and 65535 (R-014 is reserved for that).
4. `VARCHAR2(n CHAR)` columns: what the descriptor length holds (not in the fixtures; R-011).
   Session 4: `VARCHAR2(20 CHAR)` in a single-byte database gives 20 (14.4), which does not answer
   the question; `requests.md` marks R-011 as generated but no `t_probe11*` file is in the fixture
   folders. Still open for a multi-byte database.
   **Session 7:** answered for `CHAR(4 CHAR)` in AL32UTF8: 16 (4.1, `nchar_exp_<v>`@0x228c); the
   value is padded to 4 characters (6.3). `VARCHAR2(n CHAR)` in AL32UTF8: still not in the
   fixtures (R-011 stays open at low priority).
5. ~~RAW has no truth file.~~ Answered (session 3, section 6.5): all bytes equal the truth. Not
   tested: RAW with a client set other than the database set.
6. Text region with length-prefixed statements (`ddl`): the framing is described (section 4.2) and
   a reader can reach every `INSERT INTO` of the fixtures (section 4.4). Still open: whether the
   list of introducing lines is complete (rule K) and whether rule G is safe (a text field of
   8192 bytes or more); `fd ff` has one example; other statement kinds (triggers, views, grants,
   sequences, procedures) and other export modes were not seen. R-012.
   **Session 4:** the list of session 3 was not complete and rule G as written was not safe (4.2);
   both are corrected and agree on 133 files. `fd ff` is confirmed. Triggers, views, grants,
   sequences, synonyms, procedures and owner mode are described (4.5). The source of 9643 bytes is
   written in pieces, so it did not test "a plain field of 8192 bytes or more". Still open: a view
   in several pieces; a plain field of 8192 bytes or more; functions, packages, types and other
   object kinds; `ANALSTATS T?`; full database mode. R-016, R-018.
   **Session 5:** `ANALSTATS T?` and `C?` are covered by the entry "prefix `ANALSTATS `" once the
   entry allows a field in pieces (4.2, 4.6); the list is at the end of section 11 and is checked
   by `tools/intro_list.py` (619 fields in 153 files, none without an entry). The other points of
   this question are unchanged (R-018).
7. Types not in the fixtures; export modes other than table mode; users other than CORPUS.
   Session 4: owner mode and two more users are covered (4.5). Open: full database mode;
   tablespace mode; types other than those of rule T5 (CLOB observed only, 14.6).
   **Session 7:** NCHAR (96, form 2), LONG (8) and LONG RAW (24) are described (4.1, 6.3, 6.6).
   Open: LOB types (R-027), INTERVAL, ROWID, UROWID (R-029), partitioned tables (R-030).
   **Session 8:** CLOB, NCLOB, BLOB (112, 113; 14.6), INTERVAL YEAR TO MONTH and DAY TO SECOND (182,
   183), ROWID (69), UROWID (208) (6.7) and a range-partitioned table (4.8) are described. Open:
   BFILE, object / XML / collection types, subpartitions, full database mode, tablespace mode.
8. ~~Oracle 11g (section 10).~~ Session 4: analysed, section 13. Open: an `exp` client of another
   version than the database (the real dumps: `exp` 10.2 against 11.2.0.4), which no fixture has.
9. TIMESTAMP WITH TIME ZONE: the two zone bytes of a region zone; a negative offset with minutes.
10. TIMESTAMP WITH LOCAL TIME ZONE: UTC or database time zone (needs a database whose time zone is
    not +00:00); a value with a zero fraction. Session 4: the 11g database is +00:00 too, so the
    question is not answered (R-019).
11. The `00 00` before the path flag, and whether the path flag can have other values. Session 4: the
    first u16 is `02 00` for the one table with a LOB column (14.6); the flag is 0 or 1 in all 202
    table sections. Session 7: `00 00` and a flag of 0 or 1 in all 307 sections without a LOB
    column of the 169 files, LONG and LONG RAW tables included.
    **Session 8, answered for the fixtures:** the first u16 is m = 0 for a table without a LOB
    column (323 binary parts) and m = the column count for a table with one (`03 00` T_LOB, `05 00`
    T_LOB2, `02 00` R_CLOB: 15 sections of 15 files); when m > 0, m column entries follow the flag
    (14.6). The flag is 0 or 1 in all 338 binary parts; LOB tables have 0 even in `direct=y` files.
12. ~~(session 4) `ANALSTATS T?` lines (table and column statistics): reported for the real dumps,
    not in any fixture (R-016).~~ **Session 5, closed for the framing** (4.6): one field after every
    `ANALSTATS` line, plain or in pieces after `fd ff`; `ANALCOMPUTE` is a line. Open: a table in
    which only some columns have statistics (the counts of the real dump need it, R-020); a
    `CHARARRAY` list with elements and a piece of 8192 bytes or more (R-021).
    **Session 6:** the first open point is answered (4.6, "Columns without statistics": a column
    item exists exactly for a column that has statistics; the counts of the real dump fit). Still
    open: R-021; an older `exp` client (R-022); why a column has no item is not in the file.
13. (session 4) How a reader can tell a table whose export failed from an empty table (14.3).

### 9.1 Statements of session 2 that session 3 withdrew or weakened

| where | session 2 said | now | reason |
|---|---|---|---|
| section 4, first paragraph | "A table section ... ends with the LF after its row data"; "nothing is written between them" | withdrawn: statements can follow the rows; the section ends at the next `TABLE "` line or at the trailer | `ddl`@0x2286, 0x2656-0x2a6c |
| section 1 ("confirmed as text"), `tools/exp_parse.py` | the text region is a list of lines ending in LF | weakened: lines **and** length-prefixed fields; splitting on LF is wrong for `ddl` | `ddl`@0x253e (LF inside a field at 0x257f) |
| section 5.4, `tools/exp_parse.py` | the "larger than the declared length of column 1" test was the only way to detect an empty table on the direct path | kept as rule E1 next to rule E2; neither is proven | section 5.4 |
| section 2.1 / R-008 (expectation, not a status) | row 3 of `t_text` "would be 13 bytes in WE8MSWIN1252"; rows 2, 4, 5 would show a lossy conversion | refuted for these fixtures: 16 bytes, nothing converted | `t_text_exp_w1252`@0x230a |
| section 7.1 | "the group of five `METRICST` / `METRICET` pairs" as the shape of the trailer | weakened: with constraints and indexes the trailer has more `METRICST` lines (`Deferred analyze commands `, `Enable enforce constraints `) and `TABLE` ... `ENDTABLE` groups | `ddl`@0x2cdf, 0x2d6b |

No status in sections 5.1 to 6.4 was lowered. Raised: section 4.1 (+4, type 23), section 5.4 and
section 8 (empty table), section 6.5 (RAW, TSTZ fields).

### 9.2 Statements of session 3 that session 4 withdrew or weakened

| where | session 3 said | now | reason |
|---|---|---|---|
| section 4.2, rule G | "if the first two bytes are `fd ff`, or the second byte is below 0x20, the item is a field; otherwise a line": consistent with all fixtures | withdrawn as written; corrected (pieces without marker) | fails at the view text: `own`@0x349c / 0x34d1; 12 owner-mode files |
| section 4.2, rule K | "an item is a field exactly when the previous line is one of the five introducing lines": consistent with all fixtures | withdrawn as written; the list has 8 lines now and is known to be complete for the fixtures only | reads `PROCACTION`, procedures and triggers as lines without any error: `own`@0x214a, `own_tables`@0x3309 |
| section 4.2 | "A field never stands alone: every field follows a line that introduces it" | kept, with more introducing lines | 4.2 |
| sections 4.4, 10 | "both item rules walk all files without a problem" used as evidence for the rules | weakened: reaching the two `EXIT` lines without a problem does not show that the items were read correctly (rule K of session 3 does it on wrong items) | `tools/check_all.py`: "keyword3 SILENT" in 16 files |
| sections 5.4, 8, rule E3 of section 11 (version 1) | "Direct path, table without rows: there is no `ff ff`", status confirmed | lowered to "confirmed for 18c only"; on 11g `ff ff` is written | `t_empty_exp_direct_11g.dmp`@0x226d and 4 more files |
| section 10 | "the same counts as 18c", "nothing wrong" on 11g (a sanity run) | the run was right about the counts but did not see the difference above, because the walker accepts both forms | 13.2 |
| section 11 (version 1), rule T2 | "The section ends at the next `TABLE "name"` line or at the first line that starts with `METRICST`" | weakened: also at `ENDTABLE`, and `METRICSU` in owner mode; a `TABLE` line is a section start only if `CREATE TABLE` follows | `own`@0x341a |
| section 11 (version 1), rule T6 | "It [the path flag] agrees with H2 in every fixture" | withdrawn: a `direct=y` file can hold a conventional-path table | `ru_exp_direct_cp1251_11g.dmp`@0x25b3 (LOB table; reading of the flag position is a hypothesis there) |
| section 11 (version 1), rule H5 | "the first item, the line `METRICST`, stands at R + 0x104" | weakened: `METRICSU` in owner mode | `own`@0x2104 |
| section 11 (version 1), rule H1 / Y3 | header bytes 1-2 "is not the character set of the row data"; nothing said about the text | kept, and completed: it **is** the character set of the text items and identifiers | 14.2, 14.3 |
| section 7.1 | "the group of five pairs is written once per table" | kept for table mode; in owner mode each section is written once per file | 4.5 |
| section 2, 4.1 | header 0x0032 and descriptor +4 at fixed offsets / "873 in every fixture" | the values depend on the database; the header offsets are not fixed | 14.4 |

Raised in session 4: `fd ff` pieces (observed -> confirmed); header 0x0032 and block 1 +0x3c
(hypothesis -> confirmed, database set); user line and mode line (hypothesis -> confirmed for 3
users, 2 modes); NVARCHAR2 descriptor length 2n (observed -> confirmed); record boundary inside a
length field (unknown -> confirmed). No status of sections 5.1 to 6.4 was lowered.

### 9.3 Statements of session 4 that session 5 withdrew or weakened

| where | session 4 said | now | reason |
|---|---|---|---|
| 14.5, last row; section 9 ("first letter: hypothesis (index)") | "The first letter is `I` in every line: all statistics in the fixtures are index statistics" (observed) | superseded: `T` (table and column statistics) and `C` (index of a system-named constraint) exist; `I` is confirmed as "index" | `stat_exp_18c.dmp`@0x2925, 0x4e25 |
| 4.2 (introducing lines), rule I6 of section 11 version 2 | "`ANALSTATS I? "T"` ... (plain field)"; the keyword rule expected a plain field | weakened: the field is plain **or** in pieces after `fd ff`; the line can carry more text after the table name | `stat_exp_18c.dmp`@0x29a8 `fd ff`; @0x4e25 |
| 4.2, table "where each can still fail" | "K corrected: any introducing line that is not in the list (..., `ANALSTATS T?`, ...): its field is read as lines, silently" | not borne out for `ANALSTATS T?`: a list entry written as the prefix `ANALSTATS ` matches it. The warning stays true for a list that spells out `ANALSTATS IS` / `IR` | 4.2, "Session 5" |
| 7.1, row "Inside the trailer ... groups" | "All items of these groups are lines (no field)" (observed) | withdrawn: the group under "bitmap, functional and extensible indexes" can hold an `ANALSTATS C?` line **and its field** | `stat_exp_18c.dmp`@0x4e15-0x4f1b (field @0x4e45) |
| 13.5, first row; section 9 (`ddl`@0x286f) | "The last argument of `DBMS_STATS.SET_INDEX_STATS(...)` ... is `6` on 18c and `0` on 11g" (observed), read as a difference of the version | weakened: 11g writes `6` too, for statistics made by `DBMS_STATS` (`stat_exp_11g.dmp`@0x27e4 `...,1,1,1,0,6); END;`); `0` is also written on 18c, for `ANALYZE` (`stat_exp_18c.dmp`@0x435b). The observation of session 4 holds for the files it named (index statistics that exist since the index was created) | 13.5, 13.9 |
| 14.5, second row | "The second letter is `R` exactly in the files whose log has `EXP-00091`" (confirmed for the fixtures, 29 files) | kept, now 46 of 46 files; added: the letter is the same in every line of a file, and the **number** of `EXP-00091` lines is the number of `ANALSTATS ?R` lines with a plain field, not the number of lines with `R` | 4.6 |
| 2.1, 6.3, 13.7 ("the table section is byte-identical for every client set") | written for tables without text items; session 4 added the comment and CHECK texts as exceptions | one more exception: the letter `S` / `R` of every statistics line of the section. The binary part (column count to the LF after the rows) stays identical | `stat_exp_18c.dmp`@0x27d8 `53` / `stat_exp_us7ascii_18c.dmp`@0x27d8 `52`; binary parts @0x226c-0x26fc identical |
| 13.1 | "59 pairs", "44 of 59 identical from offset R on" | 64 pairs now; the same 44 are identical (none of the 5 new pairs is); the counts of 13.1 are those of session 4 | `tools/version_diff.py` |
| Section 11 version 2, gap "`ANALSTATS T?` ... not in the fixtures" | open | closed (rules S1 to S6) | 4.6 |
| 4.4, table of counts (59 / 59 / 15 files) | counts of session 4 | 64 / 64 / 25 files now, 107 / 107 / 48 table sections, 0 differing fields; the table in 4.4 is kept as the session 4 state | `tools/check_all.py` |

Raised in session 5: "items after the rows are identical on both paths" (one pair -> 3 fixture
families, 2 versions, 2 databases); the `fd ff` form (more examples: 238 statistics fields).
No status of sections 1 to 3 and 5 to 6 was changed.

### 9.4 Statements of session 5 that session 6 withdrew or weakened

| where | statement of session 5 | what session 6 found | evidence |
|---|---|---|---|
| 4.6, result paragraph and "Item forms"; section 4 ("Session 5"); rule S3 of section 11 version 3 | "A table with statistics gets one table item and one item per column" (confirmed for the fixtures, 6 tables) | weakened: one item per column **that has statistics**. The statement was true of the 6 tables it rested on, in which every column has statistics, and was marked "not tested: a table in which only some columns have statistics" | `stc_exp_18c.dmp`@0x2857, 0x29b7 (2 items for 5 columns), @0x54ba (table item, no column item) |
| 4.6, "Which items a table gets", first row of the second table | "each [table item] is followed by exactly as many column items as the table has columns" | holds for 50 of the 66 table items now; not for the 16 of the R-020 files | `tools/stc_items.py`, `tools/stat_items.py` |
| 4.6, order of the items after the rows; rule S3 | "the place of `GRANT` lines relative to `ANALSTATS T` is not in the fixtures" | closed: `GRANT` stands before the table item | `stc_exp_18c.dmp`@0x27ca / 0x27ed; `stc_exp_11g.dmp`@0x27ca / 0x27ed |
| 4.6, "The real dump", hypotheses 1 and 2 | two hypotheses for the 53 column items, "nothing in the fixtures decides" | the per-table counts of the owner's survey are those of hypothesis 2; the fixtures of R-020 show that such a file is what `exp` writes when two columns have no statistics. The **cause** stays an unverified hypothesis | 4.6, "The real dump, reconciled" |
| 4.6, candidate causes ("a column type for which no statistics exist; a difference of the 10.2 `exp` client") | listed as untested | not needed to explain the counts any more; still untested (no fixture) | - |
| 13.9, "Column statistics of a table without rows (T_STAT4)": 18c `EPC := 0` and empty lists, 11g `EPC := 2`, `NUMARRAY(0,0)`, `NUMARRAY(0,1)` | observed (2 columns x 4 files per version), read as a property of a table without rows | extended: the same difference for an **all-NULL column of a table with rows** (T_STC3.ALLNULL). It belongs to "the column has no value", not to "the table has no rows" | `stc_exp_18c.dmp`@0x45df (5 pieces), `stc_exp_11g.dmp`@0x45df (7 pieces) |
| Section 2, date text ("observed (no truth for the time of day)") | place and format described for 38 files | raised to confirmed as place and format on 157 files, with the rule a tool needs (right-aligned to 0x7f, no leading zeros) | `tools/header_date.py` |
| Section 11 version 3 and section 12: "153 files", "619 fields" | counts of session 5 | 157 files, 683 fields now (4 files x 14 `ANALSTATS` fields and 2 comment fields); no entry added to the list | `tools/intro_list.py` |

Not withdrawn: the framing of the statistics items (rules S1, S2), the three PL/SQL blocks, the
letters, `statistics=none` / `compute`. No status of sessions 1 to 4 was changed.

### 9.5 Statements of session 6 that session 7 withdrew or weakened

| where | statement of session 6 | what session 7 found | evidence |
|---|---|---|---|
| 4.1, rule T5 / X1 of version 3.1 | "For any other type code [than 1, 2, 12, 23, 96, 180, 181, 231] stop"; the size of a LONG descriptor unknown | narrowed: codes 8 (LONG) and 24 (LONG RAW) have a 4-byte descriptor and plain fields (new rule Y9); the "stop" stays for every other code | `long_exp_<v>`@0x225a, 0xa0d4; 6.6 |
| 6.1, last paragraph | "How to turn the bytes into a decimal number is not part of this document's evidence" | replaced by a rule with status confirmed (6.1, "Bytes to a decimal number") | `t_num2_exp_<v>`; `tools/num_decode.py` |
| 4.1, +6 | "Hypothesis: 1 = database character set, 2 = national character set" | unchanged as hypothesis; one more pair of types (CHAR / NCHAR) fits it | `nchar_exp_<v>`@0x2288, 0x2290 |
| 4.1, +2: "CHAR(n): n" | stated for byte semantics; read by the Architect as "CHAR is n bytes" | made precise: with character semantics the descriptor holds 16 for `CHAR(4 CHAR)` and the value is padded to 4 **characters** (4 to 16 bytes), so rule C1's "padded to its declared byte length" holds for byte semantics only | `nchar_exp_<v>`@0x228a, 0x22cc |
| `requests.md` R-023 (a), arithmetic | "14 `ANALSTATS TR` lines and 4 `ANALCOMPUTE TR` lines, each field byte-identical; 4 `EXP-00091`" | refuted in the counts (22, 5, 5): the database's statistics had changed between the exports. The part "only the letter changes" holds for every item that both files have, except one number of the T_STC2 table item | 4.6, "The R-020 tables with a US7ASCII client ..." |
| Rules file and section 11: "157 files", "683 fields", "117 table sections per version", "438 fields", "108 lines" | counts of session 6 | 169 files, 737 fields, 133 table sections per version, 482 `ANALSTATS` fields, 118 `ANALCOMPUTE` lines (`tools/corpus_counts.py`, which reproduces the session 6 numbers when the 12 new files are left out) | section 12 |

No status of sessions 1 to 6 was lowered by evidence against it; the statement on CHAR padding is
narrowed to byte semantics.

### 9.6 Statements of session 7 that session 8 withdrew or weakened

| where | statement of session 7 | what session 8 found | evidence |
|---|---|---|---|
| 14.6, row 0x11ae8 (session 4 hypothesis) | after the path flag of a LOB table: "`00 00`, u16 **BE** length of a column name, the name, `00 00`" | **withdrawn**: with one LOB column it gives the same bytes, with several it fails. The bytes are m column entries `u8 n, n name bytes, 2 bytes` (n = 0 for a non-LOB column); `00 00 00 02 43 4c 00 00` of R_CLOB is the entry of ID (`00 00 00`) and of CL (`02 43 4c 00 00`) | `lob_exp_<v>`@0x240b (11 bytes, T_LOB), @0x4db99 (18 bytes, T_LOB2); 14.6 |
| 14.6, row 0x11ae4 | `02 00` "where every other table has `00 00`": "hypothesis: a count or a column number connected with the LOB column" | answered: it is the column count of the table (3, 5, 2) when the table has a LOB column | 14.6 |
| 14.6, row 0x11b62 | "`01 00` ... number of data **chunks** that follow the row" | narrowed: it counts LOB **values** (one per non-NULL LOB column); a value of more than 65,535 bytes is itself in chunks after `fc ff` and an 8-byte length | `lob_exp_<v>`@0x4356 `02 00`, then 2 values of 13 chunks each |
| 4.1, rule T5 / X1 | "For any other type code [than the ten of T5] stop"; 112 "observed only" | narrowed again: 112, 113, 182, 183, 69, 208 are described with descriptor size and value layout; "stop" stays for every other code | 4.1, 14.6, 6.7 |
| 6.1, rule Y1 | "a column declared `NUMBER` holds values of 39 and 40 significant digits (rows 30, 31)" | **corrected**: 38 (row 30) and 40 (row 31); no value with 39 | 6.1 (note A-6) |
| 2, rule H4a | "a day of month with two digits ... would give 24 bytes from 0x68 (hypothesis)" | **confirmed**: `lob_exp_18c.dmp`@0x0068 `Wed Oct 14 22:20:57 2026` (24 bytes) and 3 more files | `tools/header_date.py`: 177 files, 0 problems |
| 5.6, rule R8 | "In the row data of the fixtures a length field is never 0 and never one of 0xfff0 to 0xfffd" | narrowed to the **column fields**: the LOB value area after a row holds the length 0 (empty LOB) and `fc ff` | 5.6 |
| 4, rule T3 / T2 | one `INSERT INTO` line per table section (implied by every fixture until session 7) | a partitioned table has one `INSERT INTO` line and binary part per partition in **one** section | 4.8 |
| Rules file: "169 files", "314 table sections", "219,043 fields" | counts of session 7 | 177 files, 334 table sections (338 binary parts), 237,347 column fields (LOB tables included) | section 12 |

No status was lowered by evidence against it except the withdrawn LOB hypothesis of session 4 and
the corrected digit count of row 30.

### 9.7 Statements of session 8 that session 9 withdrew or weakened

| where | statement of session 8 | what session 9 found | evidence |
|---|---|---|---|
| 4.7, rule T9 | "The names equal those of the `INSERT INTO` line, and the definitions match the descriptors in number, type and length" (in order) | **refuted for LONG tables on the direct path**: the LONG column stands last in `INSERT INTO` and descriptors. T9 is narrowed: same names and number; order and descriptor match by name | `long2_exp_direct_<v>`@0x221c, `long_exp_direct_<v>`@0x221a, 0x11e9a (18c); 4.1 |
| 6.6, rule Y9 | "each value is one ordinary field ... no chunks" | **narrowed to the conventional path**: on the direct path a LONG / LONG RAW value is a chain of pieces with flag bit 0x8000 | `long_exp_direct_<v>`@0x2288 `9c 80`; 6.6 |
| 5.6, rule R8 | "In a column field the u16 is 1 to 0xffef = a length" | narrowed: in a direct-path LONG / LONG RAW field a u16 with bit 0x8000 is a piece length with "more follows" (0x809c, 0x9f19 seen); for every other column field the statement holds | 5.6 |
| 6.1, rule Y1 | "No value of the fixtures has 39 significant digits" | **corrected**: AVG(1,2,2) = 5/3 has 39 (`t_num3`@0x22de); "39 or 40" is the normal count for a computed non-terminating value | 6.1 |
| 6.7, rule Y10 | "a negative value with non-zero lower fields (all fields negative?)": hypothesis | confirmed | 6.7 |
| 6.7, rule Y11 | "Other first bytes (logical UROWID): unknown" | first byte `02` = logical; text rule found | 6.7 |
| 14.6, rule L4 | "where `exp` switches between the forms: unknown"; "up to 6,000 seen" | the switch is "long iff the byte length is larger than one chunk" on both versions; short values up to 16,120 (18c) / 16,264 (11g) bytes | 14.6 |
| 4.1, rule C6 / X4 | "descriptor length as n x the largest character width: suggested"; `VARCHAR2(n CHAR)` in AL32UTF8 unknown | min(4n, cap), confirmed for both types with the caps | 4.1 |
| 5.4, rule E2a | "no LONG table on the direct path"; the case "column 1 has length 0" expected for an empty LONG table | LONG tables are written on the direct path, but the LONG column is moved last, so the case arises only for a table whose only column is LONG / LONG RAW | 5.4 |
| 4.8, rule T10 | "after the last partition one line `ENDPARTITION`" (directly) | with statistics, the `ANALSTATS` items of the table and its partitions stand between the last part and `ENDPARTITION`; composite partitions add `COMPARTITION` / `SUBPARTITION` / `ENDCOMPARTITION` | 4.8 |

No confirmed status was lowered except T9 (narrowed by a counter-example: the order clause) and
the absence statement of Y1 (corrected). Y9 and R8 are narrowed in scope, not refuted on the
fixtures they were based on.

### 9.8 Statements of session 9 that session 10 withdrew or weakened

| where | statement of session 9 | what session 10 found | evidence |
|---|---|---|---|
| 11, rule L5 (note A-20) | "A reader decodes a CLOB by the descriptor's set id: a single-byte set = that set" (written for every form; evidence for short values only) | **made precise**: confirmed for set 171 in the short **and the long** form (R-038); for the single-byte sets 1 and 178 as database sets still assumed. The sentence now names its evidence | 14.6 "Session 10" |
| 14.6 (session 8 table) | "the CLOB chunk is twice the BLOB chunk (8,060 and 8,132 characters)" | **narrowed**: true for a UTF-16 CLOB (AL32UTF8 database); in the CL8MSWIN1251 database the CLOB chunk equals the BLOB chunk (8,132 bytes on 11g) | `ru_lob2_exp_cp1251_11g.dmp`@0x29ce |
| 14.6 (session 9), rule L4 | "NCLOB: only short values; its chunk size is unknown (assumed = CLOB)" | NCLOB long values exist in the CL8MSWIN1251 database: chunks 16,264 bytes on 11g (= the AL32UTF8 CLOB chunk, not the CL8MSWIN1251 CLOB chunk). "Assumed = CLOB" is right only for a UTF-16 CLOB | @0x77fe |
| 6.3, rule C6 | "w for other sets: assumed" | w = 1 for set 171 **confirmed** (raised, note A-23); other single-byte sets still assumed | 6.3 |
| `requests.md` R-038 (Architect's text) | "+ the first two characters ... `Аб`" in the arithmetic | the bytes and the truth tail are `c0 c1` = `АБ` (capital); a slip in the request text, not in a rule | `truth_lob_ru.txt` row 2 |

No confirmed status was lowered. One status was raised (C6 for set 171) and L4 / L5 gained the single-byte database.

### 9.9 Statements of sessions 7 to 11 that session 17 withdrew or weakened

| where | earlier statement | what session 17 found | evidence |
|---|---|---|---|
| 5.6 (session 7), rule R8 | "`fd ff` (0xfffd), and every other value from 0xfff0 to 0xfffd, never occurs as a length field"; "the only ones a reader treats as an error" | **narrowed**: true for every column except LONG on the conventional path, where `f0 ff` is a length (65,520) and `fd ff` starts a chunked value (R-040). The statement was correct as an absence in the fixtures of its time | `long3_exp_<v>`@0x2228f, 0x3228f |
| 5.6 (session 7) | "In the text region `fd ff` marks a field in pieces (rule I4); in row data it does not occur ... the meaning of `fd ff` in row data: unknown" | `fd ff` occurs in row data (conventional LONG of 65,535 bytes or more) with the same shape as I4: chunks `u16 n + n bytes` up to `00 00` (without the LF) | `long3_exp_<v>`@0x3228f |
| 6.6 (session 10, note A-22), rule R8 / Y9 | "A conventional-path LONG length from 0x8000 up ... treat it as unknown (stop or warn), not as a plain length" | answered: from 0x8000 to 0xfff0 it **is** a plain length (hypothesis A of R-040); the chain reading (B) is refuted for the conventional path | `long3_exp_<v>`@0xa280 `00 80`, @0x12290 `ef ff` |
| 6.6 (session 7), rule Y9 | "A value above 65,533 bytes (it cannot have a u16 length)" | 65,520 is one field `f0 ff`; 65,535 and more use `fd ff` + chunks | `long3_exp_<v>`@0x2228f, 0x3228f |
| 6.6 (session 10, note A-18) | "Hypothesis, not tested: pieces are bounded by an internal buffer below one 8 KB record (7,961 seen)" | consistent: the largest piece of 13 chains up to 100,000 bytes is 7,961; a piece of 7,952 is the common size on both versions. Still a hypothesis for the cause | `long3_exp_direct_<v>` |
| 5.4, 13.2 (sessions 4, 5) | "the marker is missing when the table has no segment, not because of the version" (hypothesis) | **raised to confirmed** for 18c (R-017) | `seg_exp_direct_18c`@0x226a, 0x24c7 |

No confirmed status was lowered: R8 and the 5.6 absence statements were confirmed as absences and stay true for every
column type and path except the conventional LONG column, which is now described by R2 / Y9.

### 9.10 Statements of session 17 that session 19 withdrew or weakened

| where | earlier statement | what session 19 found | evidence |
|---|---|---|---|
| 6.6 (session 17), rules R2, Y9 | "`fd ff` + chunks + `00 00` at least from 65,535" | from **65,534** | `long4_exp_<v>`@0x522a6 |
| 6.6 (session 17), rules R2, Y9 | two conventional forms: one field (1 <= n <= 0xfff0) or `fd ff` chunks | **three** forms: one field also for 0xfff1 and 0xfff5; the new `fa ff` + 4-byte length form for 65,530 to 65,533 | `long4_exp_<v>`@0x2271, 0x12272, 0x22277 |
| 5.6 (session 17), rule R8 | "0 and 0xfff1 to 0xfffc there are not in the fixtures (stop)" | 0xfff1, 0xfff5 are lengths and 0xfffa is a marker there; 0xfff6 to 0xfff9, 0xfffb, 0xfffc are still not in the fixtures | 5.6 session 19 |
| 6.6 (sessions 9, 17), rule R8 | "LONG RAW above 32,000 bytes ... assumed framed like LONG" | confirmed on 18c for 32,768 to 100,000 bytes on both paths; still assumed for the `fa ff` sizes and for 11g | `longraw4_exp(_direct)_18c` |
| 6.5 (session 3) | "A TSLTZ value with a zero fraction (7 bytes?): unknown" | 7 bytes in every precision | `tsltz2_exp_<v>`@0x22df |
| 6.6 (session 17), direct path | "18c and 11g cut the same value differently" (row 1 of T_LONG3, 32,767 bytes) | not for the 7 values of T_LONG4 (every 18c / 11g pair byte-identical from 0x2000); the different cut stays observed for one value only | `long4_exp_direct_<v>` |

No confirmed status was lowered: the session 17 statements were confirmed for the sizes they named and stay true for them.

### 9.11 Statements of session 19 that session 20 withdrew or weakened

| where | earlier statement | what session 20 found | evidence |
|---|---|---|---|
| 5.6 (session 19), rule R8 | "0, 0xfff6 to 0xfff9, 0xfffb, 0xfffc: not in the fixtures, stop" | 0xfff6 to 0xfff9 are one-field lengths (LONG; 0xfff9 also LONG RAW); 0, 0xfffb, 0xfffc still not in the fixtures | `long5_exp_<v>`@0x2271, 0x12277, 0x2227e, 0x32286; `longraw5_exp_<v>`@0x22253 |
| 6.6 (session 19), rules R2, Y9 | "the switch from one field to `fa ff` lies between 65,526 and 65,530 (unknown where)" | between 65,529 and 65,530 | `long5_exp_<v>`@0x32286, 0x4228f |
| 6.6 (session 19) | "the `fa ff` form for a LONG RAW: hypothesis"; "LONG RAW on 11g: hypothesis" | both confirmed (65,530, 65,531, 65,533; 11 sizes on 11g, = 18c) | `longraw5_exp_<v>`@0x32254, 0x4225a, 0x52261; 6.6 session 20 |
| 6.6 (session 19) | the hypothesis "one field for n <= 0xfff9, `fa ff` for 0xfffa to 0xfffd, chunks from 0xfffe" | confirmed for every size in a fixture (LONG 65,525 to 65,535, LONG RAW 11 sizes); 65,522 to 65,524 still not in the fixtures | 6.6 session 20 |

No confirmed status was lowered.

## 10. Oracle 11g

Session 4: Oracle 11g is analysed in section 13, with its own offsets; the `11g` column of
section 11 says what holds. The two paragraphs that stood here in sessions 2 and 3 described
sanity runs ("the tools run over the 11g files without a problem"). They were correct as far as
they went, but such a run could not see the one layout difference between the versions (an
empty table on the direct path, section 13.2), because the walker accepts both forms; see 9.2.

## 11. Rules for a reader (version 3.9, session 20)

Only rules whose status in this document is `confirmed`. The column **11g** says whether the rule
was tested on the Oracle 11g fixtures: `same` (holds, with one 11g reference), `differs (see
13.x)`, or `not tested`. A rule without a note in the 18c text holds on 18c as written. Limits:
`exp` of the same version as the database, 18c XE and 11g XE, databases AL32UTF8 (both versions)
and CL8MSWIN1251 (11g only), table mode and owner mode. All multi-byte integers of the table data
and of the text fields are little-endian; those of the header and the two blocks are big-endian.
Version 1 (session 3) is replaced; what changed is listed in section 9.2.
Version 3 (session 5) changes rules I4, I6 and T7, adds the rules S1 to S6 (statistics items) and
the list at the end of this section; what changed is listed in section 9.3. The counts "133 files"
in rules written in session 4 are now 153 files (`tools/check_all.py`, `tools/intro_list.py`).
Version 3.1 (session 6) changes rules T7, S3 and S6, adds rule S7 and changes one row of the
gaps; the list at the end of this section is unchanged. What changed is listed in section 9.4. The
counts are now 157 files. A standalone extract of the rules a reader of table **data** needs is in
`exp_reader_rules.md` (generated from this section; this document stays the reference).
Version 3.2 (session 7) changes rules T5, R2, Y1, C1 and E2 (E2: the test is stated), adds rules T9
(column list), R8 (length values that do not occur), C5 (NCHAR), C6 (character semantics), C7
(single-byte maps) and Y9 (LONG, LONG RAW), gives every entry of the list a field for its forms,
and moves the counts to 169 files. What changed is listed in section 9.5.
Version 3.3 (session 8) changes rules H6 (the pairing with the mode line), T5 and T6 (LOB, INTERVAL,
ROWID, UROWID descriptors; m), T8 (more evidence), R4 (k after a row of a LOB table), R8 (column
fields only; only 0 and 0xfff0 to 0xfffd are errors), Y1 (38, not 39 digits), Y9 (the character set
a reader uses), E2 (a first column of descriptor length 0), and adds rules T10 (partitioned table),
L1 to L6 (LOB columns), Y10 (INTERVAL) and Y11 (ROWID, UROWID). The list at the end is unchanged.
Counts: 177 files. What changed is listed in section 9.6.
Version 3.4 (session 9) changes rules T3 (column order from `INSERT INTO`), T8 (more evidence), T9
(order clause narrowed: LONG on the direct path), T10 (composite partitions, statistics before
`ENDPARTITION`), R2 / R8 (a direct-path LONG field is a piece chain), C5 (padding in code units, non-BMP),
C6 (descriptor min(4n, cap), confirmed; replaces X4), Y1 (computed values: 39 / 40 digits), Y9 (database
set confirmed; direct path), Y10, Y11 (mixed signs, f = 0, logical UROWID), L4 (the switch), L5
(non-BMP, `GETLENGTH`), E2 (more item starts) and E2a (when it applies). The list at the end is
unchanged. Counts: 199 files. What changed is listed in section 9.7.
Version 3.5 (session 10) changes rules L4 (forms and chunks in the CL8MSWIN1251 database; the first long NCLOB), L5
(CLOB long form, NCLOB and BLOB in the single-byte database confirmed; note A-20), C6 (w = 1 for set 171 confirmed,
note A-23), R2 (what is not known about a piece u16, note A-18) and E2a (one list of item starts with E2, note A-19).
The list at the end is unchanged. Counts: 201 files. What changed is listed in section 9.8.
Version 3.6 (session 11) changes no rule in substance: the counts of T9 (386 lines, 1,907 column definitions) are
corrected, and the extract's X2 names the third failed export (`ru_exp_direct_us7ascii_11g`, 14.3). The list at the end is
unchanged. Counts: 201 files.
Version 3.7 (session 17) changes rules T5 and Y5 (TIMESTAMP(f) for every f: descriptor 11 for f > 0, R-047), R2, R8
and Y9 (a conventional-path LONG is one field up to 65,520 bytes, `fd ff` + chunks from 65,535; R8 narrowed for that
column, R-040) and E2 / E3 (the cause of the missing `ff ff`: no segment, R-017). The list at the end is unchanged.
Counts: 211 files. What changed is listed in section 9.9.
Version 3.8 (session 19) changes rules R2, R8 and Y9 (a conventional LONG has a third form `fa ff` + 4-byte length for
65,530 to 65,533 bytes; one field also for 0xfff1 and 0xfff5; `fd ff` from 65,534; LONG RAW framed like LONG, 18c; R-048),
T5 and Y8 (TSLTZ: descriptor 7 for f = 0 and 11 otherwise, values 7 or 11 bytes; R-050). E2 / E3 are unchanged (R-049
negative). The list at the end is unchanged. Counts: 221 files. What changed is listed in section 9.10.
Version 3.9 (session 20) changes rules R2, R8 and Y9 (R-051: one field up to 0xfff9, `fa ff` for 65,530 to 65,533, the
first u16 0xfff6 to 0xfff9 seen as lengths; LONG RAW = LONG on 18c and 11g). The list at the end is unchanged. Counts:
229 files. What changed is listed in section 9.11.

### File header

| # | rule | section | 11g |
|---|---|---|---|
| H1 | Byte 0 is `03`; bytes 1-2 are a u16 BE character set id: the **client** set of the export session. It is the character set of the text items and identifiers (rule C2), not of the row data (rule C1) | 2, 2.1, 14 | same: `t_frame_exp_us7ascii_11g.dmp`@0x0000 `03 00 01`; `ru_tbl_exp_cp1251_11g.dmp`@0x0000 `03 00 ab` |
| H2 | If bytes 3-4 are `D` LF, the export was started with `direct=y`; all text lines of the header are then 2 bytes later. The path of a table is given per table by rule T6, which can differ from this (a LOB table in a `direct=y` file) | 2, 8 | same: `t_frame_exp_direct_11g.dmp`@0x0003; LOB case 11g only: `ru_exp_direct_cp1251_11g.dmp`@0x25b3 |
| H3 | Then seven LF-terminated text lines follow: the version (`EXPORT:V18.00.00`, `EXPORT:V11.02.00`), `U` + exporting user, `R` + mode (`RTABLES` table mode, `RUSERS` owner mode), RECORDLENGTH R as decimal text (seen: 2048, 4096, 8192, 16384), and three lines `0`, `72`, `0` of unknown meaning. The binary fields follow the seventh line; their offset is not fixed | 1.1, 2, 14.4 | same, version text differs: `t_frame_exp_11g.dmp`@0x0003; `ru_exp_cp1251_11g.dmp`@0x0003-0x0031 |
| H4 | After the seventh line: three u16 BE: the client set again, the **database** character set, a national character set (2000 in every fixture) | 2, 14.4 | same: `t_frame_exp_11g.dmp`@0x0030 `03 69 03 69 07 d0`; `ru_tbl_exp_us7ascii_11g.dmp`@0x0033 `00 01 00 ab 07 d0` |
| H5 | The first record is R bytes long: the header, then zero bytes up to offset R. The second record starts at offset R. The file size is a multiple of R. R is the number in the header, which can differ from the `recordlength=` that was asked for (direct path) | 1, 1.1, 14.7 | same: `t_frame_exp_rl4096_11g.dmp`@0x1000; `ru_tbl_exp_direct_cp1251_11g.dmp`@0x0029 `8192`, @0x2000 |
| H6 | The first item stands at R + 0x104 and is the line `METRICST` (table mode) or `METRICSU` (owner mode). The 260 bytes before it are constant in the fixtures apart from the character set ids (section 3) and mostly unexplained, so a reader should verify the line and stop if it is not there. Version 3.3 (note A-10): the pairing `RTABLES` - `METRICST`, `RUSERS` - `METRICSU` holds in **every** fixture: 165 files `RTABLES` (74 / 72 / 19 in `18c` / `11g` / `cp1251`; version 3.4: 187, 86 / 82 / 19), all with `METRICST`; 12 files `RUSERS` (`own_exp*` 3 + 3, `ru_exp_*` 6), all with `METRICSU`; no other mode line. Confirmed for the two modes; any other mode (full database `RENTIRE`?) is not in the fixtures | 1, 3, 4.5 | same: `t_frame_exp_11g.dmp`@0x2104; `own_exp_11g.dmp`@0x2104 `METRICSU`; `ru_exp_cp1251_11g.dmp`@0x0904 |

### Items of the text region

| # | rule | section | 11g |
|---|---|---|---|
| I1 | From R + 0x104 to the second of two consecutive `EXIT` lines the file is a sequence of items; the binary part after an `INSERT INTO` line (rules T4 to E3) is the only thing that is not an item | 4.2, 4.4 | same: all 74 files of `11g/` and `11g-cp1251/` are consumed without a gap |
| I2 | A **line** is text ended by LF | 4.2 | same: `t_frame_exp_11g.dmp`@0x210d |
| I3 | A **plain field** is a u16 LE length n, n bytes (which may contain LF and quotes), then LF. n counts the bytes as written and can exceed 255 | 4.2, 4.5 | same: `own_exp_11g.dmp`@0x2d5f `2e 01` (302 bytes), @0x2d22 (LF inside) |
| I4 | A **field in pieces** starts with `fd ff`; then pieces "u16 LE length n + n bytes" follow up to a piece of length 0, then LF. The pieces joined are one statement or source text. Piece lengths are not constant (1 to 797 seen); a field can have more than 100 pieces and more than 8192 bytes. Session 5: column statistics use this form too | 4.2, 4.5, 4.6 | same: `own_exp_11g.dmp`@0x298e (pieces 25, 25, 8, 414, 1, 18, 0), @0x36e5 (126 pieces, 9650 bytes); `stat_exp_11g.dmp`@0x29a8 (123, 35, 30, 38, 22, 112, 0); `ru_stat_exp_us7ascii_est_11g.dmp`@0x19422 (piece of 797 bytes @0x194d4) |
| I5 | The text of a view is a field in pieces **without** `fd ff`: piece(s) "u16 LE length + bytes", a piece of length 0, LF. It follows the line `CREATE VIEW "name" (columns) AS ` (trailing space). Only one-piece views are in the fixtures | 4.2 | same: `own_exp_11g.dmp`@0x362d `33 00` + 51 bytes + `00 00` + LF @0x3664; `ru_exp_cp1251_11g.dmp`@0x13962 |
| I6 | In the fixtures a field follows exactly the lines of the list at the end of this section (8 entries): `ALTER TABLE "T" MODIFY DEFAULT`, `COMMENT ON ... IS ` (plain field); `ANALSTATS ...` (**plain field or field after `fd ff`: the first two bytes decide**, rule S1); `CALTER TABLE`, `PROCACTION`, `CREATE FORMAT71 PROCEDURE "P" `, `CREATE FORMAT92TRIG ` (field after `fd ff`); `CREATE VIEW ... AS ` (pieces without marker). Exactly one field follows such a line. Every other line is a complete statement. The list is complete for the fixtures only. Session 5: no new entry; the `ANALSTATS` entry changed (any two letters, text can follow the table name, two forms) | 4.2, 4.6 | same: `tools/check_all.py` and `tools/intro_list.py`, 89 files of `11g/` and `11g-cp1251/`, both item rules agree; `stat_exp_11g.dmp`@0x2925 + 0x293c (plain), @0x2991 + 0x29a8 (`fd ff`) |
| I7 | Nothing is inserted at a multiple of R inside a field of the text region | 4.5 | same: `own_exp_11g.dmp`@0x3fb9 (piece of 80 bytes across 0x4000) |

### Table section

| # | rule | section | 11g |
|---|---|---|---|
| T1 | A line `TABLE "name"` that is directly followed by a line `CREATE TABLE ...` starts a table section. A `TABLE "name"` line followed by anything else starts a group of statements that ends with `ENDTABLE` | 4, 4.4, 7.1 | same: `own_exp_11g.dmp`@0x2421 (section), @0x5fa2 (group) |
| T2 | The section ends at the next line that starts with `TABLE "`, is `ENDTABLE`, or starts with `METRICS` | 4, 4.5 | same: `own_exp_11g.dmp`@0x2e90, 0x35ab; `t_frame_exp_11g.dmp`@0x22f6 |
| T3 | Table data follow the LF of a line `INSERT INTO "name" ("col", ...) VALUES (:1, ...)`; that line gives the column names in order (in the client character set, rule C2). No other line starts table data. Version 3.4: the order of the descriptors and of the row fields is the order of **this** line, which can differ from `CREATE TABLE` (a LONG column on the direct path stands last, 4.1) | 4, 4.4, 4.1 | same: `t_frame_exp_11g.dmp`@0x2223; `ru_exp_cp1251_11g.dmp`@0x136fb |
| T4 | After the `INSERT INTO` line: u16 LE column count, then one descriptor per column | 4 | same: `t_frame_exp_11g.dmp`@0x225d `03 00` |
| T5 | Descriptor: u16 LE type code, u16 LE maximum byte length. For type 1 (VARCHAR2, NVARCHAR2), 96 (CHAR, NCHAR) and **112 (CLOB, NCLOB)** four more bytes follow: u16 LE character set id, u16 LE (1 or 2). Types 2 (NUMBER, also FLOAT, INTEGER, DECIMAL, REAL), 8 (LONG, length 0), 12 (DATE), 23 (RAW), 24 (LONG RAW, length 0), **69 (ROWID, length 23)**, **113 (BLOB)**, 180 (TIMESTAMP), 181 (TIMESTAMP WITH TIME ZONE), **182 (INTERVAL YEAR TO MONTH)**, **183 (INTERVAL DAY TO SECOND)**, **208 (UROWID)**, 231 (TIMESTAMP WITH LOCAL TIME ZONE) have 4 bytes. For any other type code stop. Version 3.2: codes 8 and 24 and NCHAR added. Version 3.3: 112, 113, 69, 182, 183, 208 added (4.1, 6.7, 14.6) Version 3.7 (session 17, R-047): the maximum length of TIMESTAMP (180) is 7 for f = 0 and 11 for every f from 1 to 9 (all ten precisions now seen; it equals `user_tab_columns.data_length`) Version 3.8 (session 19, R-050): the same for TIMESTAMP WITH LOCAL TIME ZONE (231): 7 for f = 0, 11 for f = 3, 6, 9 (seen; = `data_length`); f = 1, 2, 4, 5, 7, 8 of 231 not seen (11 assumed) | 4.1, 6.5 | same: `t_text_exp_11g.dmp`@0x2285, 0x228d; `ru_tbl_exp_cp1251_11g.dmp`@0x0ae0 `01 00 c8 00 ab 00 01 00`; `nchar_exp_11g.dmp`@0x2282 `60 00 0a 00 d0 07 02 00`; `long_exp_11g.dmp`@0x225a `08 00 00 00`, @0xa0d4 `18 00 00 00`; version 3.3: `lob_exp_11g.dmp`@0x23f5 `70 00 48 00 69 03 01 00`, @0x23fd `71 00 48 00`, @0x4df82 `70 00 48 00 d0 07 02 00`; `misc_exp_11g.dmp`@0x2297, 0x229b, 0x2459, 0x245d (= 18c); version 3.7: `tsp_exp_11g.dmp`@0x22fb to 0x2313 `b4 00 0b 00` x 7 (f = 1 to 5, 7, 8); version 3.8: `tsltz2_exp_11g.dmp`@0x22cb `e7 00 07 00`, @0x22cf, 0x22d3 `e7 00 0b 00` |
| T6 | After the descriptors: u16 LE **m** (`00 00` for every table without a LOB column; version 3.3: the column count for a table with a LOB column, rule L1), then u16 LE path flag: 0 = conventional, 1 = direct. If m = 0 the rows start after the flag; if m > 0 the m column entries of rule L1 come first | 4, 14.6 | same: `t_frame_exp_11g.dmp`@0x2277; `t_frame_exp_direct_11g.dmp`@0x2279 `01 00`; `lob_exp_11g.dmp`@0x2401 `03 00 00 00` |
| T7 | Items can stand between `CREATE TABLE` and `INSERT INTO` (DEFAULT values) and after the rows (indexes, index statistics, grants, constraints, table and column statistics, comments); a reader of table data skips them item by item **and does not rely on their number or order** (rule S7). No statistics item stands before the rows of its table | 4, 4.3, 4.5, 4.6 | same: `own_exp_11g.dmp`@0x256b-0x2623, 0x26d3-0x2e8f; `stat_exp_11g.dmp`@0x26fd-0x30d8 (T_STAT1: index, index statistics, primary key, 1 + 4 statistics items) |
| T8 | A table section is the same bytes in table mode and in owner mode, alone or with other tables | 4, 4.5 | same: `own_exp_11g.dmp`@0x2421-0x2e8f = `own_tables_exp_11g.dmp`@0x2390-0x2dfe; version 3.2: `long_exp_11g.dmp`@0x11df1-0x11fd9 = `t_frame_exp_11g.dmp`@0x210d-0x22f5 (after a LONG and a LONG RAW table); version 3.3: `lob_exp_11g.dmp`@0x4e181-0x4e369 (after two LOB tables), `misc_exp_11g.dmp`@0x24a0-0x2688 (after INTERVAL and ROWID tables), and the same in the 18c and the direct files; version 3.4: T_FRAME after the 22 new files' tables (LONG on the direct path, LOB3, INTERVAL2 / ROWID2, NCHAR2, PROBE11, composite partitions): 40 files compared by `tools/frame_t8.py`, all identical |
| T9 | (version 3.2) The `CREATE TABLE` line of a section has the column list `' (' coldef (', ' coldef)* ')'` followed by two spaces; `coldef` = quoted name, one space, type text, optionally ` NOT NULL ENABLE`. A quoted name runs to the next `"` (it may hold `,` and `(`). No `DEFAULT` or constraint stands in the list. The type texts and the precision / scale they give are those of 4.7 (`NUMBER`, `NUMBER(p, s)`, `NUMBER(*,s)`, `FLOAT(b)`, ...); they equal `data_precision` / `data_scale` of the dictionary. The names equal those of the `INSERT INTO` line, and the definitions match the descriptors in number, type and length. Version 3.4: **in order, except** a LONG / LONG RAW column on the direct path, which stands last in `INSERT INTO` and the descriptors (6 sections); match by name. Character semantics: descriptor min(4n, 4000 / 2000) in AL32UTF8 (C6) | 4.7, 4.1 | same: `t_decl_exp_11g.dmp`@0x211c (byte-identical to 18c); 314 lines of 169 files, `tools/collist.py`. Version 3.6: 386 lines of 201 files, 1,907 column definitions. Version 3.3: 334 lines of 177 files; types `BLOB`, `NCLOB`, `ROWID`, `UROWID(n)`, `INTERVAL YEAR (p) TO MONTH ` (a space before the separator), `INTERVAL DAY (p) TO SECOND (f)` (4.7) |
| T10 | (version 3.3) A partitioned table has **one** table section with one `CREATE TABLE` line and, inside it, for each partition: a line `PARTITION "name"`, an `INSERT INTO` line and a complete binary part (T4 to E1/E2); after the last partition one line `ENDPARTITION`. The rows of the table are the rows of all parts in file order; each part's rows belong to the named partition. `PARTITION` and `ENDPARTITION` are plain lines (no field follows; not in list K). Version 3.4: a **composite** partition is a line `COMPARTITION "p"`, then per subpartition `SUBPARTITION "s"` + `INSERT INTO` + binary part, then `ENDCOMPARTITION`; the partition itself has no binary part. An empty (sub)partition is an empty binary part (E1; on the 18c direct path E2). The statistics items of the table and of each partition (S1) stand after the last part and **before** `ENDPARTITION` | 4.8 | not tested: 11g XE has no Partitioning option; 18c: `part_exp_18c.dmp`@0x2528, 0x82e3, 0xe107, 0x1265d; `part_exp_direct_18c.dmp`@0x2528, 0x7a55, 0xcfe5, 0x10eed; version 3.4: `part2_exp_18c.dmp`@0x2468 `COMPARTITION "P1"`, @0x247a `SUBPARTITION`, @0x2836 `ENDCOMPARTITION`, @0x2d5a first `ANALSTATS`, @0x407a `ENDPARTITION`; `part2_exp_direct_18c.dmp`@0x2865, 0x28d2 (empty subpartitions, E2) |

### Statistics items (session 5)

| # | rule | section | 11g |
|---|---|---|---|
| S1 | A line that starts with `ANALSTATS ` is followed by exactly one field. If the two bytes after the LF of the line are `fd ff` it is a field in pieces (rule I4), otherwise a plain field (rule I3). Both forms occur after lines with the same text. The line has two letters after `ANALSTATS ` (seen: `TS`, `TR`, `IS`, `IR`, `CS`, `CR`), then the table name in double quotes, and can have more text after it (`ANALSTATS CS "T" ("ID" ) `) | 4.2, 4.6 | same: `stat_exp_11g.dmp`@0x2925 + 0x293c `52 00`; @0x2991 + 0x29a8 `fd ff`; @0x4e6d + 0x4e8d `71 00`; `ru_stat_exp_us7ascii_est_11g.dmp`@0x190a0 + 0x190b6, @0x1910d + 0x19123 |
| S2 | A line that starts with `ANALCOMPUTE ` is a complete item; no field follows | 4.6, 14.5 | same: `stat_exp_11g.dmp`@0x524e; `ru_stat_exp_us7ascii_est_11g.dmp`@0x2be81 |
| S3 | Statistics items stand (a) in a table section after the rows: `I?` directly after the `CREATE INDEX` line of its index, `T?` after the index and constraint items and the `GRANT` lines and before the comments, first one item for the table, then one per column **that has statistics**, in ascending column order (version 3.1); (b) in the trailer inside `TABLE` ... `ENDTABLE` groups: `C?` + field under "bitmap, functional and extensible indexes", `ANALCOMPUTE` lines under "Deferred analyze commands " (table mode) or "statistics" (owner mode). The order inside the table section is observed on 11 tables (no table has an index, a grant, a constraint and statistics together), so a reader does not rely on these places; it skips the items where they occur | 4.6, 4.5, 7.1 | same: `stat_exp_11g.dmp`@0x27cd, 0x2925-0x30d8, 0x4e6d, 0x524e; `ru_stat_exp_us7ascii_est_11g.dmp`@0x18c7f, 0x2af41-0x2bc0e; `stc_exp_11g.dmp`@0x27ca `GRANT`, @0x27ed table item, @0x2b67 comment |
| S4 | The field is ASCII text, one PL/SQL block: `BEGIN  DBMS_STATS.SET_TABLE_STATS(NULL,'"T"',NULL,NULL,NULL,rows,blocks,avg_row_len,f); END;` (plain), `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"I"',NULL,NULL,NULL,<8 numbers>); END;` (plain), or `DECLARE  SREC DBMS_STATS.STATREC; BEGIN SREC.MINVAL := ...; ... DBMS_STATS.SET_COLUMN_STATS(NULL,'"T"','"C"', NULL ,NULL,NULL,distinct,density,nulls,srec,avg_col_len,f); END;` (pieces). Numbers are decimal text; the low and high value of a column are hexadecimal text of the value bytes of the database (never characters); histogram endpoints are numbers. Table, column and index names inside the block are in the client set like every text item (rule C2); with ASCII names every byte of the field is below 0x80. The field holds no table data, and a reader of table data does not decode it | 4.6, 14.10 | same: `stat_exp_11g.dmp`@0x293c `...,50,5,17,6); END;` (`TAB\|T_STAT1\|50\|5\|...\|17`); `ru_stat_exp_us7ascii_est_11g.dmp`@0x19424 `SREC.MINVAL := 'C0C1C2'` |
| S5 | The second letter is `S` or `R`, the same in every statistics line of a file; `R` in the files whose export log has `EXP-00091` ("questionable statistics"), which in the fixtures are the files whose client set differs from the database set. The fields are the same bytes with either letter. A reader ignores the letter | 4.6, 14.5 | same: `stat_exp_us7ascii_11g.dmp`@0x2925 `ANALSTATS TR`; `ru_stat_exp_cp1251_est_11g.dmp`@0x190a0 `TS` / `ru_stat_exp_us7ascii_est_11g.dmp`@0x190a0 `TR` |
| S6 | A file made with `statistics=none` has no `ANALSTATS` and no `ANALCOMPUTE` item. A table can also have none in a file made with statistics, next to tables that have them (version 3.1: T_STC4, `truth_stc.txt` `TAB\|T_STC4\|NULL\|NULL`; it then has no `ANALCOMPUTE` line either). The `num_rows` of a table block is not the row count of the table in the file | 4.6 | same: `stat_exp_none_11g.dmp` (60 items against 111); `ru_stat_exp_us7ascii_none_11g.dmp`; `stc_exp_11g.dmp`@0x4de3 LF after the rows, @0x4de4 `GRANT`, @0x4e07 `TABLE "T_STC5"` |
| S7 | (version 3.1) The number of statistics items of a table is not fixed by its column count. A column has a column item exactly when it has statistics at export time; a table with n columns has 0 or 1 table item and 0 to n column items; with a table item there can be no column item; without a table item there is none (in the fixtures). A missing item leaves no bytes. The k-th column item is not the k-th column: the column is named only inside the field. The file does not say why an item is missing. A reader of table data skips by framing (S1, S2) up to the end of the section (T2) and never counts items | 4.6 ("Columns without statistics") | same: `stc_exp_11g.dmp`@0x2857, 0x29b7 (T_STC1: 2 of 5), @0x3896-0x3a13 (T_STC2: 4 of 6), @0x429b, 0x444b (T_STC3: positions 2 and 4 adjacent), @0x54c4 (T_STC5: table item only); `truth_stc.txt` |

### Rows and fields

| # | rule | section | 11g |
|---|---|---|---|
| R1 | A row is exactly "column count" fields in column order. There is no row header, row length or row number | 5.1 | same: `tools/check_all.py` on `11g/`: 85 tables equal the truth |
| R2 | A field is a u16 LE length n followed by n value bytes. The length field is always 2 bytes (seen: 1 to 32,000) and does not count itself. Version 3.4: **except** a LONG / LONG RAW field on the direct path, which is a chain of pieces: u16 n, (n & 0x7fff) bytes, another piece while bit 0x8000 of n is set (rule Y9). Version 3.5 (note A-18): seen as piece u16 values: `fe ff` (NULL, first position only), lengths without the flag up to 7,961, 0x809c and 0x9f19; a piece u16 from 0xfff0, a piece of length 0 (0x8000 or 0x0000) and `fe ff` / `ff ff` after the first piece are **not in the fixtures**: which reading wins is unknown (6.6) Version 3.7 (session 17, R-040): **a second exception, a LONG field on the conventional path (flag 0)**: either one ordinary field whose u16 n is the length for 1 <= n <= 0xfff0 (bit 0x8000 is part of the length: `00 80` = 32,768, `ef ff` = 65,519, `f0 ff` = 65,520 seen), or `fd ff` followed by chunks `u16 n + n bytes` up to a chunk of length 0 (`00 00`) (65,535, 65,536, 100,000 seen; chunks of 4000, the remainder last). Where the switch lies between 65,521 and 65,535 is unknown (R-048). Version 3.8 (session 19, R-048): **three** conventional forms: (a) one field for n up to 0xfff0 and also for 0xfff1 and 0xfff5 (seen); (b) **`fa ff` + 4 bytes + n bytes** for n = 65,530, 65,532, 65,533 (seen): the 4 bytes are u16 n followed by `00 00` (read as u32 LE: hypothesis); (c) `fd ff` + chunks + `00 00` from **65,534** (seen 65,534 to 100,000). First u16 0xfff6 to 0xfff9, 0xfffb, 0xfffc: not in the fixtures. LONG RAW (18c): forms (a) and (c) seen (32,768, 65,520; 65,535, 100,000), (b) assumed. A direct-path LONG of any size is a chain (100,000 = 12 x 7952 + 4576 seen; pieces of 7952 or 7961 with the flag; the cut of the same value differs between 18c and 11g); a piece u16 from 0xfff0 or a piece of length 0 is still not in the fixtures. Version 3.9 (session 20, R-051), **replaces the conventional part of 3.8**: a conventional LONG / LONG RAW field (flag 0) has three forms chosen by its first u16 u: (a) u = 1 to 0xfff9: one field of u bytes (seen above 0x7fff: 0x8000, 0xffef, 0xfff0, 0xfff1, 0xfff5, 0xfff6, 0xfff7, 0xfff8, 0xfff9; 0xfff2 to 0xfff4 lie between seen one-field values: hypothesis); (b) u = 0xfffa: 4 bytes follow, u16 LE n and `00 00`, then n bytes (n = 65,530 to 65,533 all seen; u32 LE: hypothesis); (c) u = 0xfffd: chunks `u16 c + c bytes` up to `00 00` (65,534 to 100,000 seen). The switch (a) -> (b) is between 65,529 and 65,530, (b) -> (c) between 65,533 and 65,534, both seen on LONG and LONG RAW. u = 0xfffb, 0xfffc: never seen as a first u16 (their sizes are written in form (b)). **LONG RAW = LONG** in every form, the same u16 values and chunk sizes, 18c and 11g | 5.1, 5.2, 14.1, 6.6 | same: `t_len_exp_11g.dmp`@0x2c94 `ff 00`, @0x2d9c `00 01`; `ru_tbl_exp_cp1251_11g.dmp`@0x4727 `b8 0b`; `long_exp_11g.dmp`@0x2289 `00 7d`; version 3.7: `long3_exp_11g.dmp`@0xa280 `00 80`, @0x2228f `f0 ff`, @0x3228f `fd ff`; `long3_exp_direct_11g.dmp`@0x522fb; version 3.8: `long4_exp_11g.dmp`@0x2271 `f1 ff`, @0x22277 `fa ff fa ff 00 00`, @0x522a6 `fd ff` (= 18c); LONG RAW: 18c only (`longraw4_exp_18c.dmp`@0x225d, 0x1a25d); version 3.9: `long5_exp_11g.dmp`@0x32286 `f9 ff`, @0x5229d `fa ff fb ff 00 00` (= 18c); LONG RAW on 11g: `longraw5_exp_11g.dmp`@0x22253 `f9 ff`, @0x32254 `fa ff fa ff 00 00`, @0x6226a `fd ff`, @0x72294 `00 80`, @0x8a294 `fd ff` (= 18c from 0x2000 to the end of the common rows; ids 8 to 11 = `longraw4_exp_18c.dmp` fields) |
| R3 | `fe ff` in place of the length is NULL, for every type and position; no value bytes follow. Trailing NULLs are written, one `fe ff` each | 5.1, 5.3 | same: `t_frame_exp_11g.dmp`@0x2295; `ru_tbl_exp_cp1251_11g.dmp`@0x4639-0x4644 (six NULLs) |
| R4 | Conventional path: `00 00` follows the last field of every row. Direct path: nothing stands between rows. Version 3.3: in a table with a LOB column (always conventional in the fixtures) the u16 after the row is k, the number of LOB values that follow (rule L3); k = 0 is the `00 00` | 5.1, 8, 14.6 | same: `t_frame_exp_11g.dmp`@0x228d; `t_frame_exp_direct_11g.dmp`@0x228d |
| R5 | Nothing is inserted, repeated or changed at a multiple of R inside the table data: a value, a length field, an `fe ff` marker and a `00 00` row end continue across it | 1.2 | same: `t_bn1_exp_11g.dmp`@0x3fff `eb` / @0x4000 `04`; `t_bn3_exp_11g.dmp`@0x3fff `fe` / @0x4000 `ff`; `ru_exp_cp1251_11g.dmp`@0xdfff `00` / @0xe000 `00` |
| R6 | Rows are not sorted and not necessarily in insertion order; the order can differ between two exports of the same table. No row count is stored; the number after `METRICET` / `METRICEU` is not the row count of a table | 5.5, 7.1, 13.4 | same (and the order differs from 18c): `t_probe6_exp_11g.dmp`@0x225e R00257 |
| R7 | The path changes nothing else: values, lengths, NULL markers and descriptors are identical on both paths | 8 | same: 1854 fields equal the truth on both paths in `11g-cp1251/`; table of 13.1 |
| R8 | (version 3.3, note A-9) In a **column field** of the row data the u16 is one of: 1 to 0xffef = a length; `fe ff` = NULL; `ff ff` (at the start of a row only) = end of rows. The values 0 and 0xfff0 to 0xfffd never occur in the fixtures and are the only ones a reader treats as an error. Lengths up to 32,000 are seen; a larger length is **not** an error (it is simply not in the fixtures). The LOB value area after a row (rules L3, L4) is not a column field: there 0 is an empty LOB and `fc ff` starts a long value Version 3.7 (session 17, R-040): **narrowed**: the error values 0xfff0 to 0xfffd do occur in one place, a LONG column on the conventional path, where `f0 ff` is a length (65,520) and `fd ff` starts the chunked form of R2; lengths from 0x8000 are plain lengths there. So in a LONG / LONG RAW column with flag 0: 1 to 0xfff0 = a length, `fd ff` = chunked value, `fe ff` = NULL; 0 and 0xfff1 to 0xfffc = not in the fixtures (a reader stops: unknown). In every other column field the rule stands as written (243,303 column fields of 211 files: no other u16 from 0x8000 up). LONG RAW: assumed the same as LONG (not in the fixtures above 32,000 bytes) Version 3.8 (session 19, R-048): in a LONG / LONG RAW column with flag 0 the first u16 is: 1 to 0xfff5 = a length (0xfff1, 0xfff5 seen above 0xfff0; 0xfff2 to 0xfff4 assumed); `fa ff` = the 4-byte-length form (R2 (b)); `fd ff` = chunked (R2 (c)); `fe ff` = NULL; **0, 0xfff6 to 0xfff9, 0xfffb, 0xfffc = not in the fixtures** (a reader stops: unknown). LONG RAW: confirmed like LONG on 18c for 0x8000, 0xfff0 and `fd ff`. Every other column field: unchanged (243,593 column fields of 221 files) Version 3.9 (session 20, R-051): in a LONG / LONG RAW column with flag 0 the first u16 is: 1 to 0xfff9 = a length (0xfff1, 0xfff5 to 0xfff9 seen above 0xfff0; 0xfff2 to 0xfff4 not in the fixtures, between seen lengths); `fa ff` = R2 (b); `fd ff` = R2 (c); `fe ff` = NULL; **0, 0xfffb, 0xfffc = not in the fixtures** (0xfffb / 0xfffc: their sizes are written `fa ff` + n; a reader stops). LONG RAW: confirmed like LONG on 18c and 11g. Every other column field: unchanged (243,905 column fields of 229 files) | 5.6 | same: 237,347 column fields of 177 files, LOB tables included, `tools/rowlen_stats.py`. Version 3.4: 238,657 column fields of 199 files, 0 findings; the pieces of a direct-path LONG field (R2, Y9) have u16 values with bit 0x8000 (0x809c, 0x9f19) and are not "lengths" in this sense: `long_exp_direct_11g.dmp`@0x2289. Version 3.5: 243,007 column fields of 201 files, 0 findings; version 3.7: `long3_exp_11g.dmp`@0x2228f `f0 ff`, @0x3228f `fd ff` (= 18c); version 3.8: `long4_exp_11g.dmp`@0x2271 `f1 ff`, @0x12272 `f5 ff`, @0x22277 `fa ff` (= 18c); version 3.9: `long5_exp_11g.dmp`@0x2271 `f6 ff` to @0x32286 `f9 ff`, @0x4228f `fa ff`; `longraw5_exp_11g.dmp`@0x225d `f1 ff`, @0x52261 `fa ff fd ff 00 00` (= 18c) |

### Character sets

| # | rule | section | 11g |
|---|---|---|---|
| C1 | Row data are the bytes of the database: VARCHAR2 (1) and CHAR (96) values are in the character set named at +4 of the column descriptor (873 AL32UTF8 or 171 CL8MSWIN1251 in the fixtures; equal to the database set of rule H4), whatever the client set is. The length counts bytes. CHAR with byte semantics is padded with 0x20 to its declared byte length (version 3.2: with character semantics to its declared number of characters, rule C6). Values can contain any byte, including LF, TAB, `ff ff` | 2.1, 4.1, 6.3, 14.1 | same: `t_text_exp_w1252_11g.dmp`@0x230a `10 00` + UTF-8; `ru_tbl_exp_us7ascii_11g.dmp`@0x4494 `0b 00 cf f0 e8 ...` |
| C2 | Every line and field of the text region, identifiers included, is in the client character set of rule H1; the length of a field counts those bytes. Characters the client set lacks were replaced when the file was written (`?`, or 0xbf with WE8MSWIN1252) | 14.2, 14.3 | same: `own_exp_w1252_11g.dmp`@0x2ce8; `ru_exp_al32utf8_11g.dmp`@0x135ae; `ru_exp_us7ascii_11g.dmp`@0x135ef |
| C3 | NVARCHAR2 (type 1 with character set id 2000 at +4, and 2 at +6): UTF-16 big-endian, the length counts bytes; the descriptor length is twice the declared character count | 4.1, 6.3, 14.1, 14.4 | same: `t_text_exp_11g.dmp`@0x237f `d8 3d de 00`; `ru_tbl_exp_cp1251_11g.dmp`@0x461a |
| C5 | (version 3.2) NCHAR (type 96 with set id 2000 at +4 and 2 at +6): UTF-16 big-endian, padded with `00 20` to the declared number of characters; the descriptor length is twice the declared character count. Version 3.4: "characters" are UTF-16 **code units**: a non-BMP character is a surrogate pair and counts 2 (`NCHAR(3)` with `a` + U+1F600 is 6 bytes, unpadded); a reader decodes UTF-16 with surrogate pairs (also C3, L5) | 4.1, 6.3 | same: `nchar_exp_11g.dmp`@0x2282, @0x22a2 `0a 00 00 61 00 62 00 20 00 20 00 20` |
| C6 | (version 3.2) `CHAR(n CHAR)` (type 96, set 873, form 1) in an AL32UTF8 database: descriptor length 4n; the value is padded with 0x20 to n **characters**, so its byte length varies (n to 4n). Rule C1's "padded to its declared byte length" is for byte semantics. Version 3.4: the descriptor length of `VARCHAR2(n CHAR)` and `CHAR(n CHAR)` is **min(4n, 4000)** / **min(4n, 2000)** in AL32UTF8 and n in the single-byte database, = `data_length` (replaces X4); "characters" are code points (U+1F600 counts 1). The descriptor is a maximum, not the declared length: take n from `CREATE TABLE` (T9). Version 3.5 (note A-23): w = 1 for set 171 is confirmed (`VARCHAR2(20 CHAR)` = 20, truth `Len=20` for 20 characters); for the other single-byte sets as database sets: assumed | 4.1, 6.3 | same: `nchar_exp_11g.dmp`@0x228a `60 00 10 00`, @0x22cc `06 00 d0 b4 d0 b6 20 20`; version 3.4: `t_probe11_exp_11g.dmp`@0x2311 `01 00 28 00`, @0x2329 `01 00 a0 0f` (1001 CHAR), @0x2339 `60 00 d0 07` (501 CHAR), @0x2783 `fa 01` (506 bytes) |
| C7 | (version 3.2) Oracle maps every byte of CL8MSWIN1251 and WE8MSWIN1252 to one Unicode character: the Windows code page, and for the bytes the code page leaves undefined (0x98 in 1251; 0x81, 0x8D, 0x8F, 0x90, 0x9D in 1252) the C1 control of the same number. US7ASCII: bytes 0x00 to 0x7F as themselves, every byte above 0x7F to U+FFFD | 14.11 | same (`truth_csmap.txt` of `11g/` equals that of `18c/`) |

### Types

| # | rule | section | 11g |
|---|---|---|---|
| Y1 | NUMBER (2): the value bytes are the bytes `DUMP(col,16)` shows (1 to 21 bytes seen), with no extra length byte. Version 3.2: the decimal value follows from them by the procedure of 6.1 ("Bytes to a decimal number"): `80` alone is 0; bit 0x80 of the first byte is the sign; exponent E = b0 - 0xC1 (positive) or 0x3E - b0 (negative), in powers of 100; digit bytes d + 1 (positive) or 101 - d (negative), base 100, most significant first; a negative number ends with `66` unless it has 20 digit bytes. At most 20 digit bytes (40 decimal digits); E from -65 to 62 seen. Version 3.3: the first and last base-100 digit (digit **byte**) are never 0; the largest number of significant decimal digits seen is 40 (row 31), the next 38; no value has 39 (note A-6). Version 3.4: a value computed by SQL (quotient, AVG) that does not terminate has 20 digit bytes (Len 21) on both versions: 40 significant decimal digits, or 39 when the first digit byte is below 10 (`t_num3` row 6, 5/3); "no value has 39" is corrected | 6.1 | same: `t_num_exp_11g.dmp`@0x2291 `03 00 c2 02 18`; `t_num2_exp_11g.dmp`@0x22d9 (-9223372036854775808); `t_num3_exp_11g.dmp`@0x22de `15 00 c1 02` + 18 x `43` + `44` |
| Y2 | DATE (12): 7 bytes: century + 100, year in century + 100, month, day, hour + 1, minute + 1, second + 1; for years before 1 the first two bytes are below 100 (100 - century, 100 - year) | 6.2 | same: `t_date_exp_11g.dmp`@0x224b, @0x22b4 |
| Y3 | (moved to C1) | - | - |
| Y4 | (moved to C3) | - | - |
| Y5 | TIMESTAMP (180): 7 bytes in the DATE layout when the fraction is zero, otherwise 11 bytes: DATE layout + u32 BE nanoseconds Version 3.7 (session 17, R-047): holds for every precision 0 to 9 (f = 1 to 5, 7, 8 added: 7 bytes in row 1, 11 in rows 2 and 3, `fe ff` in row 4, both paths and versions byte-identical). The nanoseconds are whatever the database stored; Oracle rounded a 9-digit literal to f digits at INSERT (t4 123,500,000, t8 123,456,790), so a reader decodes, never rounds | 6.4 | same: `t_ts_exp_11g.dmp`@0x22cc (7), @0x22fa (11); version 3.7: `tsp_exp_11g.dmp`@0x231f (7), @0x23ec `07 5c 75 e0`, @0x2413 `07 5b cd 16` |
| Y6 | RAW (23): the value bytes are the RAW bytes, unchanged | 6.5 | same: `t_raw_exp_11g.dmp`@0x224d; `t_bn1_exp_11g.dmp`@0x224d (`eb 04` + 1259 x `a1`) |
| Y7 | TIMESTAMP WITH TIME ZONE (181): 13 bytes: 7 bytes DATE layout **in UTC**, u32 BE nanoseconds, 2 zone bytes. If bit 0x80 of the first zone byte is clear, the zone is an offset: hour + 20, minute + 60 (confirmed for +05:30, +00:00, -08:00) | 6.5 | same: `t_tstz_exp_11g.dmp`@0x22a8; region bytes equal to 18c (@0x22f8 `86 48`) |
| Y8 | TIMESTAMP WITH LOCAL TIME ZONE (231): TIMESTAMP layout without zone bytes; the same instant gives the same bytes whatever the session time zone was (11 bytes seen; the time is in zone +00:00, which is UTC and the database time zone in every fixture) Version 3.8 (session 19, R-050): the value is **7 bytes when the stored fraction is zero, in every precision** (also under a descriptor of 11), 11 bytes otherwise, as Y5; the seconds and fraction are the stored (database-rounded) values; field bytes identical on both paths and versions. The session-zone part rests on `t_tstz` only (R-050 used +00:00 throughout) | 6.5 | same: `t_tstz_exp_11g.dmp`@0x233f = @0x2355; version 3.8: `tsltz2_exp_11g.dmp`@0x22e8 `07 00` + 7 bytes, @0x2316 `0b 00` + 11 bytes (= 18c) |
| Y9 | (version 3.2) LONG (8) and LONG RAW (24): 4-byte descriptor with length 0; each value is one ordinary field (u16 LE length + bytes, `fe ff` for NULL), no chunks, nothing before or after the rows that other tables lack. Confirmed for values up to 32,000 bytes on the conventional path. A value above 65,533 bytes, the direct path and the character set of a LONG value are not in the fixtures. Version 3.3 (note A-8): a reader decodes LONG with the **database** set of H4 (the second u16 after the seventh header line), never with header bytes 1-2 (C4). What the fixtures show: nothing about the set (every LONG value is ASCII, and client = database = AL32UTF8 in both LONG files). Why the database set: VARCHAR2 values (C1) and the CLOB of the single-byte database (L5) are database bytes whatever the client set was. Status of the choice: suggested (R-031 row 1 tests it). Version 3.4: **confirmed** for AL32UTF8: a non-ASCII LONG (`d0 b4 d0 b6 20 43 61 66 c3 a9`) is the same database bytes with an AL32UTF8 and a US7ASCII client. **Direct path**: LONG tables are written on the direct path (flag 1); the LONG column stands last (T3, T9); its value is a piece chain (R2): u16 n, (n & 0x7fff) bytes, more pieces while bit 0x8000 is set (32,000 = 156 + 4 x 7961 seen); a short value is one piece. A value above 32,000 bytes: unknown (R-040) Version 3.7 (session 17, R-040): **conventional path**: a LONG value of up to 65,520 bytes is one field with a u16 length (`ff 7f`, `00 80`, `ef ff`, `f0 ff` seen); 65,535 bytes and more are `fd ff` + chunks + `00 00` (R2); the open point of version 3.2 ("a value above 65,533 bytes") is answered. **Direct path**: the chain for every size up to 100,000 seen. Conventional = direct = truth for all 7 values, and V and T_FRAME are intact after them on both paths and versions Version 3.8 (session 19, R-048): conventional path: one field also at 65,521 and 65,525; **65,530 to 65,533 bytes: `fa ff` + u16 n + `00 00` + n bytes**; `fd ff` + chunks from 65,534. LONG RAW (18c): one field at 32,768 and 65,520, `fd ff` chunks at 65,535 and 100,000, chains on the direct path, the same u16 values as LONG. Values, V and T_FRAME intact (`tools/long4_check.py`) Version 3.9 (session 20, R-051): conventional path: one field up to **65,529** (`f6 ff` to `f9 ff` seen), `fa ff` + u16 n + `00 00` + n for **65,530 to 65,533**, `fd ff` chunks from 65,534 (every size from 65,525 to 65,535 now in a fixture; 65,522 to 65,524 not). **LONG RAW = LONG** on 18c and 11g: one field `f1 ff`, `f5 ff`, `f9 ff`, `fa ff` at 65,530, 65,531, 65,533, `fd ff` chunks at 65,534; chains on the direct path with the LONG piece sizes; 11g = 18c byte for byte. Values, V and T_FRAME intact (`tools/long5_check.py`) | 6.6 | same: `long_exp_11g.dmp`@0x2285 (32,000-byte LONG), @0xa0e6 (32,000-byte LONG RAW), @0x9f8b (the column after the LONG); version 3.4: `long2_exp_us7ascii_11g.dmp`@0x2271, `long_exp_direct_11g.dmp`@0x2289 `9c 80`, @0x8078 `19 1f`; version 3.7: `long3_exp_11g.dmp`@0x2271, 0xa280, 0x12290, 0x2228f (one field), 0x3228f, 0x422c2, 0x522f6 (`fd ff`); `long3_exp_direct_11g.dmp`@0x2279 (`10 9f`); version 3.8: `long4_exp_11g.dmp`@0x22277, 0x32285, 0x42295 (`fa ff`), @0x522a6 (`fd ff`, 65,534); LONG RAW above 32,000 bytes on 11g: not tested; version 3.9: `long5_exp_11g.dmp`@0x2271 to 0x32286 (`f6 ff` to `f9 ff`), 0x4228f, 0x5229d (`fa ff`); `longraw5_exp_11g.dmp`@0x225d to 0x9a2bf (11 values, = 18c); `longraw5_exp_direct_11g.dmp`@0x225d (`10 9f`) |
| Y10 | (version 3.3) INTERVAL YEAR TO MONTH (182, 5 bytes) and INTERVAL DAY TO SECOND (183, 11 bytes): ordinary fields, the bytes are `DUMP`. 182: years = u32 BE - 0x80000000, months = byte 4 - 60. 183: days = u32 BE - 0x80000000, hours, minutes, seconds = bytes 4, 5, 6 - 60, nanoseconds = u32 BE bytes 7-10 - 0x80000000. The `TO_CHAR` text: sign, years / days with the leading precision of the `CREATE TABLE` line, `-MM` / ` HH:MI:SS.fffff` with the fractional precision (6.7). Confirmed for 3 values per column (positive, negative, zero); a negative value with non-zero hours to fraction: hypothesis (every field negative). Version 3.4: **every field of a negative interval is negative** (confirmed: 6 values); precisions YEAR(9), DAY(9) TO SECOND(9), DAY(2) TO SECOND(0) give the same descriptor lengths 5 / 11; text sign `-` if any field is negative (`-00 00:00:01`), and no `.` and no fraction when f = 0 | 6.7 | same: `misc_exp_11g.dmp`@0x22a7, 0x22ae (= 18c); `misc2_exp_11g.dmp`@0x22e8, 0x22f5 (= 18c) |
| Y11 | (version 3.3) ROWID (69): the field is 23 bytes, the last 18 are the `ROWIDTOCHAR` text; the first 5 (`01 00 00 00 17` in both values) are unknown. UROWID (208): the field is the `DUMP` bytes; first byte `01` = physical: object u32 BE, file u16 BE, block u32 BE, row u16 BE, which written in base 64 (6 + 3 + 6 + 3 characters) give the same text as ROWIDTOCHAR. Version 3.4: the ROWID prefix is `01 00 00 00 17` in all 8 values (2 databases); first byte `02` = **logical** UROWID (index-organised table): its text is `*` + base 64 of the bytes after the first (3 bytes to 4 characters), `02 04 03 00 15 b3 02 c1 02 fe` = `*BAMAFbMCwQL+`; any other first byte: report the bytes | 6.7 | same layout, other values: `misc_exp_11g.dmp`@0x2469 `AAAE5IAAEAAAACGAAA`, @0x2482 `01 00 00 4e 48 ...`; `misc2_exp_11g.dmp`@0x254f `02 04 01 00 0a d3 02 c1 02 fe` = `*BAEACtMCwQL+` |

### LOB columns (version 3.3, session 8)

| # | rule | section | 11g |
|---|---|---|---|
| L1 | A table has a LOB column when a descriptor has type 112 (CLOB, NCLOB; 8 bytes) or 113 (BLOB; 4 bytes). Then the u16 m before the path flag is the column count (not 0), and after the flag stand m **column entries**, one per column in order: u8 n, n bytes (the column name for a LOB column, nothing for another column: n = 0), 2 bytes `00 00`. The rows start after the last entry. A reader verifies m = column count and n > 0 exactly for the LOB columns, and stops otherwise | 14.6 | same: `lob_exp_11g.dmp`@0x2401-0x240f = `lob_exp_18c.dmp`@0x2407-0x2415; `ru_tables_exp_us7ascii_11g.dmp`@0x11ae4 (CL8MSWIN1251) |
| L2 | In a row, the field of a LOB column is `fe ff` (NULL) or a **locator**: u16 length + bytes (102 to 1102 bytes seen; 114 on 18c, variable on 11g). It is not the data; a reader skips it | 14.6 | same as a field; other lengths (`lob_exp_11g.dmp`@0x2414 `78 00`, 120 bytes) |
| L3 | After the last field of a row: u16 k = the number of LOB columns of this row with a locator (not NULL); then k **LOB values** in column order. k = 0 is the ordinary `00 00` row end | 14.6 | same: `lob_exp_11g.dmp`@0x2500 `02 00`, @0x252a `00 00` |
| L4 | A LOB value is either **short**: u16 n + n bytes (n = 0 for an empty LOB; up to 6,000 seen), or **long**: `fc ff`, 8 bytes LE = total byte length T, then chunks `u16 c + c bytes` until the chunks add up to T (no terminator; chunk sizes differ by version and type, never assume them). NULL and empty differ: NULL has `fe ff` in its column and no value; empty has a locator and a value of length 0. Version 3.4: `exp` writes the long form **exactly when the byte length exceeds one chunk** (18c: CLOB 16,120, BLOB 8,060; 11g: 16,264, 8,132); every chunk but the last is full. A reader accepts both forms anyway and does not use the chunk size. Version 3.5: in the CL8MSWIN1251 database (11g) a CLOB of 20,000 characters is long with chunks of **8,132** bytes (8,132 + 8,132 + 3,736), an NCLOB of 20,000 characters is long with chunks of **16,264** bytes (the first long NCLOB of the corpus), a BLOB 8,132; where the short form ends for CLOB / NCLOB there is not in the fixtures (R-046) | 14.6 | same; chunk sizes 16,264 / 8,132 (11g) against 16,120 / 8,060 (18c): `lob_exp_11g.dmp`@0x475a, 0x354be; version 3.4: `lob3_exp_11g.dmp`@0x1434d short 16,264, @0x1a389 long 16,264 + 2 |
| L5 | Value bytes: BLOB = the bytes. NCLOB = UTF-16 BE. CLOB = UTF-16 BE in an AL32UTF8 database (set id 873 in the descriptor), the database bytes in the CL8MSWIN1251 database (set id 171), with every client set. A reader decodes a CLOB by the descriptor's set id: a single-byte set = that set; 873 = UTF-16 BE; any other id: unknown (report). Version 3.4: non-ASCII and non-BMP text in AL32UTF8 confirmed: UTF-16 BE with surrogate pairs (`04 34`, `00 e9`, `d8 3d de 00`); `GETLENGTH` counts code units. Version 3.5 (R-038, note A-20): in the CL8MSWIN1251 database (set 171) the CLOB is the database bytes in the short **and the long** form (66 and 20,000 bytes), NCLOB is UTF-16 BE short and long, BLOB the bytes, with a CL8MSWIN1251 and a US7ASCII client alike. "A single-byte set = that set" is confirmed for **171** (both forms, 11g); for 1 and 178 as database sets it is assumed | 14.6 | same (`lob_exp_11g.dmp`@0x2502 `00 68 00 65`); R_CLOB `cf f0 e8 e2 e5 f2`; version 3.5: `ru_lob2_exp_cp1251_11g.dmp`@0x274c `42 00 c0 c1`, @0x2790 `84 00 04 10`, @0x29c4 `fc ff 20 4e 00 00 00 00 00 00 c4 1f`, @0x77f4 `fc ff 40 9c ..` (= `ru_lob2_exp_us7ascii_11g.dmp` at the same offsets) |
| L6 | A LOB table is written on the conventional path even in a `direct=y` file (flag 0, k after each row); the tables after it are not affected (T8) | 8, 14.6 | same: `lob_exp_direct_11g.dmp`@0x2403 flag 0, T_FRAME flag 1 |

What L1 to L6 do **not** cover (a reader stops or warns): type 114 (BFILE) and every other code;
a LOB table with path flag 1; a CLOB whose descriptor set id is multi-byte but not 873. Version 3.4:
a non-BMP character in a CLOB is a surrogate pair, and the switch from the short to the long form is
known (L4); neither changes what a reader does. Version 3.5: a CLOB / NCLOB / BLOB of the CL8MSWIN1251 database is
covered in both forms (11g); a LOB of a database whose set is US7ASCII or WE8MSWIN1252, and a single-byte database on
18c, are not.

### End of data

| # | rule | section | 11g |
|---|---|---|---|
| E1 | `ff ff` where the first field of a row would start ends the rows of a table; one LF follows, then the next item. If it stands directly after the path flag the table has no rows in the file (it is empty, or its export failed: 14.3) | 5.1, 5.4 | same: `t_frame_exp_11g.dmp`@0x22f3; `t_empty_exp_11g.dmp`@0x226d; `t_empty_exp_direct_11g.dmp`@0x226d |
| E2 | Direct path only (path flag 1): let p be the offset after the path flag. The table has no rows, and there is no `ff ff`, when **both** tests of 5.4 hold: (text) the byte at p is LF and the bytes at p + 1 start an item (in the fixtures: an upper-case letter, and one of `TABLE "`, `ENDTABLE`, `METRICS`, `CREATE `, `ALTER `, `COMMENT `, `GRANT `, `ANALSTATS `, `CALTER `), and (declared length) the u16 LE at p is neither `fe ff` nor `ff ff` and is larger than the descriptor length of column 1. Version 3.2: the tests are stated (note A-2); in all 13 empty direct-path tables and all direct-path tables with rows the two tests agree. Version 3.4: also for an empty LONG table and two empty subpartitions (18c); new item starts after p: `SUBPARTITION "`, `ENDCOMPARTITION`; a reader's list also contains `PARTITION "`, `COMPARTITION "`, `ENDPARTITION` (they can follow a part, T10) Version 3.7 (session 17, R-017): the cause is known for 18c: the direct path writes no `ff ff` for an empty table **without a segment** and writes `ff ff` for an empty table with one (`seg_exp_direct_18c.dmp`@0x226a, 0x23ca: `ff ff 0a`; @0x24c7: `0a 4d`). So both E1 and E2 occur in one 18c file | 5.4 | differs (see 13.2): not seen on 11g, where `ff ff` is written (`t_empty_exp_direct_11g.dmp`@0x226d; `long_exp_direct_11g.dmp`@0x11f47). 18c: `t_empty/direct`@0x220a `0a 4d`, `own/direct`@0x2f71 `0a 43`, `stat_exp_direct_18c.dmp`@0x495a `0a 41`; `long_exp_direct_18c.dmp`@0x11ee2 `0a 54`, `part2_exp_direct_18c.dmp`@0x2865 `0a 53`, @0x28d2 `0a 45`; version 3.7 (18c): `seg_exp_direct_18c.dmp`@0x226a `ff ff 0a` (segment), @0x24c7 `0a 4d` (no segment) |
| E3 | A reader accepts both E1 and E2 after a path flag of 1 and does not decide by the version line. Version 3.7 (session 17, R-017): confirmed by one 18c file that holds both forms (`seg_exp_direct_18c.dmp`): the form follows the segment, not the version | 5.4, 13.2 | - |
| E2a | (version 3.3, note A-7) If column 1 has descriptor length 0 (LONG, LONG RAW), test (declared length) of E2 cannot fail and says nothing. Then: read test (text) alone **and** require that the line from p + 1 to the next LF is a complete item line of a known kind (`TABLE "...`, `ENDTABLE`, `METRICS...`, `CREATE ...`, `ALTER ...`, `COMMENT ON ... IS `, `GRANT ...`, `ANALSTATS ...`, `CALTER TABLE`; version 3.5, note A-19: and the partition lines `PARTITION "`, `COMPARTITION "`, `SUBPARTITION "`, `ENDCOMPARTITION`, `ENDPARTITION`: one list for E2 (a) and E2a, 5.4) and, if it introduces a field (list K), that the field after it is well-formed; otherwise read a row. Version 3.4: `exp` does write LONG tables on the direct path, but it moves the LONG column last (T3), so column 1 has length 0 only in a table whose **only** column is LONG or LONG RAW; the empty two-column LONG table of the fixtures is decided by E2 as usual. Status: suggested (a recommendation for the single-column case, untested) | 5.4 | not tested (the single-column case); the two-column case: same as E2 |
| E4 | The data of the file end with the lines `EXIT` LF `EXIT` LF; zero bytes fill the file to a multiple of R | 1, 7 | same: `t_frame_exp_11g.dmp`@0x23e6, 0x23eb; `own_exp_11g.dmp`@0x64af, 0x64b4 |

### Needed by a reader, but not confirmed (these are not rules)

| gap | state | section |
|---|---|---|
| How to tell a field from a line when the line before it is not in the list of I6 | Rule G (second byte below 0x20, or `fd ff`) and rule K (list I6) agree on all 133 files after both were corrected in session 4; each fails on input the fixtures do not contain (4.2). Recommendation, not a rule: use the list, and treat an unknown line followed by an item that rule G calls a field as a field, with a warning | 4.2 |
| A view text in more than one piece; a plain field of 8192 bytes or more | not in the fixtures (R-018) | 4.2 |
| Statement kinds not in the fixtures: functions, packages, types, materialized views, jobs, database links | not in the fixtures (R-018). Session 5: `ANALSTATS T?` is no longer in this row (rules S1 to S6) | 4.5 |
| Statistics: a `CHARARRAY` list with elements; a piece of 8192 bytes or more; first letters other than `T`, `I`, `C`; partitioned tables; the R-020 tables written with a US7ASCII client or an older `exp` client | not in the fixtures (R-021, R-022, R-023). The framing rule S1 does not depend on them. Version 3.1: "a table in which only some columns have statistics" left this row (rule S7) | 4.6 |
| Why `ff ff` is missing after an empty table on the direct path of 18c (version or missing segment) | version 3.7: **answered for 18c** (no segment, no `ff ff`; R-017, `seg_exp_direct_18c.dmp`); an 11g table without a segment: not in the fixtures (R-049). The reader rule E1 / E2 / E3 does not depend on it | 5.4, 13.2 |
| How to recognise a table whose export failed | no certain sign in the file | 14.3 |
| Zone bytes of a TSTZ value with a region zone; TSLTZ: UTC or database time zone | unknown | 6.5 |
| Any type code not listed in T5; full database mode; an `exp` client of another version than the database | not in the fixtures. Version 3.2: LONG and LONG RAW left this row (rule Y9). Version 3.3: CLOB, NCLOB, BLOB (L1 to L6), INTERVAL (Y10), ROWID, UROWID (Y11) left it | 9, 14.6 |
| (version 3.3) LOB: BFILE; a LOB in another single-byte or multi-byte database (version 3.5: CL8MSWIN1251 left this row; US7ASCII / WE8MSWIN1252 databases and 18c with a single-byte database remain; where the short form ends there: R-046); partitions in owner mode, with indexes, other partitioning methods, subpartition statistics; the meaning of the 5 bytes before the ROWID text and of the logical UROWID bytes. Version 3.4: non-BMP CLOB text, the LOB switch, subpartitions, an empty partition, partition statistics, mixed negative intervals, a logical UROWID left this row | not in the fixtures | 14.6, 4.8, 6.7 |
| LONG above 32,000 bytes (conventional: a length with bit 0x8000?; direct: longer chains); LONG in a single-byte database. Version 3.4: the direct path, a non-ASCII LONG and `VARCHAR2(n CHAR)` in AL32UTF8 left this row. Version 3.7: LONG of 32,767 to 100,000 bytes left this row (R-040: R2, Y9); what remains: the switch from one field to `fd ff` between 65,521 and 65,535 bytes (u16 0xfff1 to 0xfffc), LONG RAW above 32,000 bytes, a LONG in a single-byte database | not in the fixtures (R-048, R-043) | 6.6 |

### Lines that introduce a field (rule I6), as a list

One entry per line: kind, byte length of the text, the allowed forms of the field (version 3.2,
notes A-3 / N-3: `plain` = rule I3, `marked` = pieces after `fd ff`, rule I4, `unmarked` = pieces
without marker, rule I5; several joined by `|`), the ASCII text (trailing spaces belong to the
text; the length makes them visible). Format `kind:length:forms:text`; `tools/intro_list.py` reads
this block and the one in `exp_reader_rules.md` and checks every field of every entry. `prefix`: the line starts with the text; `exact`: the line
is the text; `suffix`: the line ends with the text. "Line" is the bytes up to, not including, LF.

    suffix:15:plain: MODIFY DEFAULT
    prefix:10:plain|marked:ANALSTATS 
    prefix:11:plain:COMMENT ON 
    exact:12:marked:CALTER TABLE
    exact:10:marked:PROCACTION
    prefix:26:marked:CREATE FORMAT71 PROCEDURE 
    exact:20:marked:CREATE FORMAT92TRIG 
    prefix:12:unmarked:CREATE VIEW 

| # | kind | text | field form(s) | full line in the fixtures | fields in 169 files (157 files) |
|---|---|---|---|---|---|
| 1 | suffix | ` MODIFY DEFAULT` (leading space) | plain | `ALTER TABLE "T" MODIFY DEFAULT` | 30 (28) |
| 2 | prefix | `ANALSTATS ` (trailing space) | plain, or pieces after `fd ff` | `ANALSTATS TS "T"`, `ANALSTATS IR "T"`, `ANALSTATS CS "T" ("C" ) ` | 170 plain, 312 in pieces (160, 278) |
| 3 | prefix | `COMMENT ON ` (trailing space) | plain | `COMMENT ON TABLE "T" IS `, `COMMENT ON COLUMN "T"."C" IS ` | 130 (122) |
| 4 | exact | `CALTER TABLE` | pieces after `fd ff` | the same | 49 |
| 5 | exact | `PROCACTION` | pieces after `fd ff` | the same | 12 |
| 6 | prefix | `CREATE FORMAT71 PROCEDURE ` (trailing space) | pieces after `fd ff` | `CREATE FORMAT71 PROCEDURE "P" ` | 12 |
| 7 | exact | `CREATE FORMAT92TRIG ` (trailing space) | pieces after `fd ff` | the same | 10 |
| 8 | prefix | `CREATE VIEW ` (trailing space) | pieces without marker (rule I5) | `CREATE VIEW "V" ("C1","C2") AS ` | 12 |

Checked by `tools/intro_list.py` on the 157 EXP files of the three folders, with the fields found
by rule G (which uses no list): 683 fields; 0 fields after a line that matches no entry; 0 lines
that match an entry and have no field; 0 lines that match two entries; 0 fields of a form the
entry does not allow. Status: confirmed for the fixtures; complete for the fixtures only.

Session 6 (version 3.1): **the list above is unchanged**, the same eight entries in the same
order as in session 5; only the counts of the table moved (153 to 157 files, 619 to 683 fields:
the 4 R-020 files add 16 plain and 40 pieced `ANALSTATS` fields and 8 comment fields). Applied to
all 157 files it gives the same items as rule G: `tools/check_all.py` `rule_diff` 0 (66 / 66 / 25
files), `tools/intro_list.py` `field_without_entry` 0, `entry_without_field` 0. No difference to
report.

Session 7 (version 3.2): the eight entries are unchanged in kind, text and order; each line now
carries its forms (`kind:length:forms:text`). On 169 files: 737 fields (683 + 54: the two `stc_exp_us7ascii_<v>` files
add 44 `ANALSTATS` fields, 10 plain and 34 in pieces; the four R-023 files add 8 `COMMENT ON`
fields; the two `t_decl_exp_<v>` files add 2 ` MODIFY DEFAULT` fields);
`tools/intro_list.py`: `block_diff` 0 (both blocks equal the tool's list in all four fields),
`field_without_entry` 0, `entry_without_field` 0, `two_entries` 0, `form_not_allowed` 0.

Session 8 (version 3.3): the eight entries are unchanged. The 8 new files have no field in the text
region (`statistics=none`; no index, comment, default), so the counts stay 737 fields in 177 files;
`tools/intro_list.py`: `block_diff` 0 and 0 in every error count. The new lines `PARTITION "name"`
and `ENDPARTITION` (4.8) are plain lines and are rightly not in the list.

Session 9 (version 3.4): the eight entries are unchanged. The two `part2` files add 40 `ANALSTATS`
fields (10 plain, 30 in pieces: 16 for T_PART2 with its partition items, 4 for T_FRAME, per file);
the other 20 new files have no field. 199 files, 777 fields; `tools/intro_list.py`: `block_diff` 0,
`field_without_entry` 0, `entry_without_field` 0, `two_entries` 0, `form_not_allowed` 0. The new
lines `COMPARTITION "p"`, `SUBPARTITION "s"`, `ENDCOMPARTITION` are plain lines, not in the list.

Notes for a tool. (1) Entry 2 is the only one with two forms; decide by the bytes (`fd ff`), not
by the letters. (2) Lines that start with `ANALCOMPUTE ` are **not** in the list. (3) Entries 6 to
8 are as narrow as the fixtures: functions, packages and types probably need more entries
(hypothesis; R-018), and a view created with `FORCE` or other keywords is not in the fixtures.
(4) Session 4 had the same eight entries; the difference of version 3 is the form of entry 2.
Which five of them the project owner's tool has is not known to this document; its output names
the fields it finds after a line it does not know.


## 12. Tools

All in `docs/formats/tools/`; each has a header that says what it checks, on which fixtures, and
what changed. Run with `.venv/bin/python` or `uv run --no-project` (session 17: never plain `uv run`, CLAUDE.md rule 9).

Session 5, the tools on the 20 new files, before any change: `tools/check_all.py` printed "RULES
DISAGREE ... item count 111 / 111" (93 / 93) for the 17 files with `T` items, because the keyword
rule reported "expected form plain" for every column-statistics field (the items read were the
same); it printed `no truth` for T_STAT1 to T_STAT4 and `R_STAT:2000:DIFF 2000` for R_STAT (the
table was compared with the two rows of the Cyrillic-named table). No tool lost its place in a
file, and `problems 0` in every walk. After the changes of `exp_parse.py` and `check_truth.py`:
`rule_diff` 0, `bad` 0 in the three folders (64 / 64 / 25 files; 107 / 107 / 48 table sections;
5865 / 5865 / 166377 fields equal, 0 differ). `tools/header_fields.py`: "invariant violations:
none" on 153 files. `tools/boundary.py`, `tools/rowdata_cmp.py`, `tools/textitems_cs.py`,
`tools/clientset_diff.py`, `tools/exp_items.py`, `tools/version_diff.py`, `tools/anal_lines.py` run
unchanged and report no problem on the new files.

Session 6, the tools on the 4 new files, before any change: no tool printed a problem.
`tools/exp_parse.py` (both item rules): "problems: none", 114 items per file; `tools/check_all.py`:
`problems 0`, `rule_diff 0`, and `no truth` for T_STC1 to T_STC5 (20 tables); `tools/stat_items.py`:
0 unknown fields, letters fit the logs, `col_order_ok` 62; `tools/intro_list.py`: 0 in all four
error counts; `tools/header_fields.py`: "invariant violations: none" on 157 files;
`tools/boundary.py`: "problems none" (one record boundary inside row data, `stc_exp_18c.dmp`@0x4000,
between a length field and its value, T_STC3 row 43); `tools/stat_cmp.py`, `tools/version_diff.py` (66 pairs, the same 44
identical), `tools/anal_lines.py`, `tools/textitems_cs.py`, `tools/rowdata_cmp.py`,
`tools/exp_items.py`: no problem. After the change of `check_truth.py`: 66 / 66 / 25 files, 117 /
117 / 48 table sections, `no_truth` 0, `bad` 0, 7965 / 7965 / 166377 fields equal, 0 differ.

Session 7, the tools on the 12 new files, before any change: no tool printed a problem.
`tools/check_all.py`: `problems 0`, `rule_diff 0` in all three folders, and `no truth` for T_NUM2,
T_DECL, T_NCHAR, T_LONG, T_LONGRAW (the walker of `exp_parse.py` reads every type code outside 1,
96, 112 as a 4-byte descriptor, which is how it walked the LONG files to the end); `tools/intro_list.py`:
0 in all error counts, 737 fields; `tools/header_date.py`: 169 files, 0 problems (date texts of 20
to 23 bytes; `long_exp_11g.dmp` has the 20-byte `Thu Oct 8 0:9:9 2026`);
`tools/header_fields.py`: "invariant violations: none"; `tools/anal_lines.py`: 52 files with
statistics lines, 0 contradict; `tools/boundary.py`: "problems none" on the 12 files (the LONG and
LONG RAW values cross 6 record boundaries); `tools/version_diff.py`: 72 pairs, see 13.1;
`tools/stc_items.py` on the `_none_` files reported every statistics line of `truth_stc.txt` as
missing (44 "problems"), because it did not know `statistics=none`. After the changes:
`tools/check_all.py` 72 / 72 / 25 files, 133 / 133 / 48 table sections, `no_truth` 0, `bad` 0,
10130 / 10130 / 166377 fields equal, 0 differ (T_NUM2, T_NCHAR against their truth; T_DECL,
T_LONG, T_LONGRAW against the SQL); `tools/stc_items.py` 0 problems on the four R-023 files.
`tools/corpus_counts.py` gives the counts of the rules file: 169 files; 314 table sections (303
with only the 8 type codes of T5 version 3.1, 4 with LONG / LONG RAW, 7 with CLOB); 34,090 rows;
219,085 row fields (18,922 NULL); 737 text-region fields (330 plain, 395 in pieces after `fd ff`,
12 without marker); 482 `ANALSTATS` fields (170 plain, 312 in pieces); 118 `ANALCOMPUTE` lines;
22 + 13 empty tables (conventional / direct); 2 to 120 columns. With the 12 files of session 7
left out it gives the session 6 numbers (157 files, 282 sections, 683 fields, 438 and 108).

Session 8, the tools on the 8 new files, before any change: `tools/exp_parse.py` walked
`misc_exp_<v>` to the end without a problem (every code outside 1, 96, 112 read as 4 bytes, which
holds for 182, 183, 69, 208), but stopped with a `KeyError` on `lob_exp_<v>` (its session 4 LOB
hypothesis misread T_LOB, whose BLOB has a 4-byte descriptor and whose bytes after the flag are 11,
not 8) and read the three partitions of `part_exp_18c` as **one** table entry overwritten three times
(it printed only the last part, PMAX, 807 rows: a silent misreading of the kind rule T10 now
prevents). `tools/collist.py`: 12 lines it could not read (the new type texts). After the changes
(table below): `tools/check_all.py` 77 / 75 / 25 files, 144 / 142 / 48 table sections (148 / 142 /
48 binary parts), `problems 0`, `rule_diff 0`, `no_truth 0`, `bad 0`; 22,241 / 10,241 / 166,377
fields equal, 0 differ (new: CLOB data 14, NCLOB data 4, BLOB data 14, INTERVAL YM 4, INTERVAL DS
4, ROWID 2, UROWID 2 per version; CLOB data 21 in `11g-cp1251`; T_PART 2 x 3000 rows against the
SQL); `tools/lob_check.py` 60 + 60 + 21 checks, `tools/misc_types.py` 40, `tools/part_check.py`
38, all OK; `tools/intro_list.py` 737 fields, 0 problems; `tools/header_fields.py` "invariant
violations: none" on 177 files; `tools/header_date.py` 177 files, 0 problems, date texts of 20 to
**24** bytes (4 files with a two-digit day, e.g. `lob_exp_18c.dmp`@0x0068 `Wed Oct 14 22:20:57
2026`); `tools/anal_lines.py` 52 files with statistics lines, 0 contradict; `tools/num_decode.py`
84,968 NUMBER fields decode and re-encode, 84,930 compared, 0 problems; `tools/collist.py` 334
lines, 1715 columns, 0 problems; `tools/boundary.py` "problems none" on the new files (LOB values
cross 37 / 38 boundaries); `tools/rowlen_stats.py` 0 findings.

Counts of the rules file (`tools/corpus_counts.py`, all three folders): 177 files; 334 table
sections (`tools/check_all.py`: 338 binary parts, the partitioned table has 3); 40,172 rows;
237,347 row fields (18,994 NULL); 737 text-region fields (330 plain, 395 in pieces after `fd ff`,
12 without marker), unchanged; 482 `ANALSTATS` fields (170 plain, 312 in pieces) and 118
`ANALCOMPUTE` lines, unchanged; 22 + 13 empty tables (conventional / direct), unchanged; 2 to 120
columns; type codes outside the eight of version 3.1 of T5: 112 in 15 sections, 113 in 8, 8, 24,
182, 183, 69, 208 in 2 each. Largest column field: 32,000 bytes (unchanged); largest LOB value
200,000 bytes (in chunks).

Session 9, the tools on the 22 new files, before any change: `tools/exp_parse.py` (and so
`tools/check_all.py`) stopped with an `IndexError` on `long_exp_direct_<v>`: it read the first piece
u16 `9c 80` of the 32,000-byte LONG as a length of 32,924 and ran off the file (a direct-path LONG is
a piece chain, 6.6). On the other 20 files it walked to the end with `problems 0`, but T_LONG2 on the
direct path was read with the columns in `INSERT INTO` order, which `tools/collist.py` then reported
as 3 problems per LONG section ("names differ from INSERT INTO", 18 in all); `tools/boundary.py` stopped
with "gap" on `long_exp_direct_<v>`; `tools/check_all.py` printed `no truth` for every new table and did
not know `COMPARTITION` / `SUBPARTITION` (the 6 binary parts of T_PART2 had no partition name). After
the changes: `tools/check_all.py` 89 / 85 / 25 files, 170 / 164 / 48 table sections (184 / 164 / 48
binary parts), `problems 0`, `rule_diff 0`, `empty_rule_diff 0`, **`no_truth 0`**, `bad 0`; fields
equal 22,979 / 10,537 / 166,377, 0 differ (the session 3 rules `generic3` / `keyword3` fail or misread
the same 3 + 3 + 6 owner-mode files as before). `tools/num_decode.py`: 120 truth values, 85,456 NUMBER
fields decode and re-encode, 84,940 compared, 0 problems. `tools/misc_types.py` 106 checks,
`tools/lob3_switch.py` 162, `tools/part2_check.py` 52, `tools/frame_t8.py` 40 files identical, all OK;
`tools/collist.py` 382 lines, 1885 column definitions, 55 character-semantics descriptors checked, 6
"moved" LONG sections, 0 problems; `tools/desc_len_check.py` 0 values longer than their descriptor
(LOB locators and LONG excluded); `tools/intro_list.py` 777 fields, 0 problems; `tools/header_fields.py`
"invariant violations: none"; `tools/header_date.py` 199 files, 0 problems (20 to 24 bytes);
`tools/anal_lines.py` 54 files with statistics lines, 0 contradict; `tools/boundary.py` "problems none"
(a record boundary splits `fc ff` once: `lob3_exp_11g.dmp`@0x47fff); `tools/rowlen_stats.py` 0 findings.
`tools/stat_items.py` on `part2_exp_18c` reports the 12 partition-level statistics fields as
"unknown" (it knows only `SET_TABLE_STATS(NULL,'"T"',NULL,...)`): a limit of that tool, not of the
framing; not changed (statistics are skip-only).

Counts of the rules file (`tools/corpus_counts.py`, all three folders, session 9): **199 files**
(89 / 85 / 25); **382 table sections** (170 / 164 / 48; `tools/check_all.py`: **396 binary parts**,
T_PART has 3 and T_PART2 6 per file); **40,588 rows**; **238,657 row fields** (19,186 NULL); **777
text-region fields** (340 plain, 425 in pieces after `fd ff`, 12 without marker); **522 `ANALSTATS`
fields** (180 plain, 342 in pieces) and **122 `ANALCOMPUTE` lines** (all new ones from `part2`); 24 + 17
empty binary parts (conventional / direct); 2 to 120 columns; type codes outside the eight of version
3.1 of T5: 112 in 19 sections, 113 in 10, 8 in 12, 24 in 4, 182, 183, 69, 208 in 4 each. Largest
column field: 32,000 bytes (unchanged; on the direct path as a chain of 5 pieces); largest LOB value
200,000 bytes (unchanged); largest short LOB value 16,264 bytes (11g CLOB). Against session 8: +22
files, +48 sections (+58 binary parts), +416 rows, +1,310 fields (+192 NULL), +40 text fields (all
`ANALSTATS`, from `part2`), +4 `ANALCOMPUTE`.

Session 10, the tools on the 2 new files, before any change: `tools/exp_parse.py` walked both to the end with `problems
none` and read R_LOB2 by L1 to L4 (3 rows, 9 LOB values, short and long). `tools/check_all.py` and `tools/num_decode.py`
**stopped** on `11g-cp1251/` with `TypeError: 'NoneType' object is not subscriptable` in `check_truth.num_int`:
`check_truth.expected()` had no branch for R_LOB2, so R_LOB2 fell into the branch written for the Cyrillic-named table
(any unknown table of a folder with `truth_ru.txt`, the same defect as R_STAT in session 5) and its key function read
column 2 (a NULL LOB in row 3). `tools/num_decode.py` also stopped, on `18c/` and `11g/` alone, with `KeyError: 0` in step
3 (note A-24; defect since session 9, see the tool's header: first field hit `long2_exp_11g.dmp`@0x226d, the ID of
T_LONG2 row 1). No other tool printed a problem: `tools/corpus_counts.py` `problems 0`; `tools/rowlen_stats.py` 0
findings; `tools/intro_list.py` 783 fields, 0 problems, `block_diff 0`; `tools/header_fields.py` "invariant violations:
none" on 201 files; `tools/header_date.py` 201 files, 0 problems; `tools/desc_len_check.py` 0 longer than the
descriptor; `tools/collist.py` 0 problems; `tools/boundary.py` "problems none" on the new files (10 boundaries inside the
LOB chunks of R_LOB2, 8 inside R_TEXT values); `tools/anal_lines.py` 54 files with statistics lines, 0 contradict;
`tools/frame_t8.py` 40 identical; `tools/lob3_switch.py` 162, `tools/lob_check.py` 141, `tools/misc_types.py` 106,
`tools/part2_check.py` 52 checks, all OK; `tools/textitems_cs.py` on the new files: 3 items in the client set, equal to
the truth converted. After the changes: `tools/check_all.py` 89 / 85 / 27 files, 170 / 164 / 52 table sections (184 / 164
/ 52 binary parts), `problems 0`, `rule_diff 0`, `empty_rule_diff 0`, `no_truth 0`, `bad 0`; fields equal **22,979 /
10,537 / 170,109**, 0 differ (`11g-cp1251/` new: CLOB data 31 (was 21: per R_LOB2 file 3 C values and the NULL C and N fields of row 3, which
the walker files under type 112), NCLOB data 6 (3 per file), BLOB data 8 (3 values + the NULL B field per file); the session 3 rules `generic3` / `keyword3` fail or misread the same 3 + 3 + 6 owner-mode files as
before). `tools/num_decode.py`: 120 truth values; **86,080** NUMBER fields of 201 files decode and re-encode (= 85,456 of
the 199 files of session 9 + 2 x 312 new: the session 9 figure of Y1 is reproduced by the tool itself), 86,042 compared,
0 problems. `tools/lob_ru_check.py` 77 checks, 0 differ.

Counts of the rules file (`tools/corpus_counts.py`, all three folders, session 10): **201 files** (89 / 85 / 27); **386
table sections** (170 / 164 / 52; **400 binary parts**); **41,212 rows**; **243,007 row fields** (19,900 NULL); **783
text-region fields** (344 plain, 427 in pieces after `fd ff`, 12 without marker); 522 `ANALSTATS` fields (180 plain, 342
in pieces) and 122 `ANALCOMPUTE` lines, unchanged; 24 + 17 empty binary parts, unchanged; 2 to 120 columns; type codes
outside the eight of version 3.1 of T5: 112 in **21** sections, 113 in **12**, 8 in 12, 24 in 4, 182, 183, 69, 208 in 4
each. LOB values after the rows (`tools/rowlen_stats.py`): 126 (80 short, 46 long; 12 of length 0) in 21 LOB sections.
Largest column field 32,000 bytes, largest LOB value 200,000 bytes, largest short LOB value 16,264 bytes (all unchanged);
new: long NCLOB values (40,000 bytes). Against session 9: +2 files, +4 sections (+4 binary parts), +624 rows, +4,350
fields (+714 NULL), +6 text fields (per file 2 `COMMENT ON` plain, 1 `CALTER TABLE` in pieces), +0 statistics.

Session 11 (no new dump), all tools over the three folders: `sha256sum -c SHA256SUMS` 147 / 140 / 30 OK.
`tools/corpus_counts.py` TOTAL: 201 files, `problems 0`, 386 sections, 41,212 rows, 243,007 row fields, 19,900 NULL, 783
text fields (344 / 427 / 12), 522 `ANALSTATS` fields (180 / 342), 122 `ANALCOMPUTE` lines, 24 + 17 empty parts, 2 to 120
columns, codes outside T5 as in session 10. `tools/check_all.py`: 89 / 85 / 27 files walked, 170 / 164 / 52 sections, 184 /
164 / 52 binary parts (`with_insert`, sum 400), `problems 0`, `rule_diff 0`, `empty_rule_diff 0`, `no_truth 0`, `bad 0`,
0 fields differ (the session 3 rules `generic3` / `keyword3` fail or misread the same 3 + 3 + 6 owner-mode files as
before: expected, not a problem of the current rules). `tools/rowlen_stats.py`: 400 sections, 41,212 rows, 243,007
fields, 19,900 NULL, 0 of length 0, 0 from 0xfff0, `len_8000_up` 0; 126 LOB values (80 short, 46 long, 12 of length 0) in
21 LOB sections, 81 LOB rows. `tools/num_decode.py`: 86,080 NUMBER fields decode and re-encode, 86,042 compared with a
truth or SQL value, PROBLEMS 0. `tools/collist.py`: 386 lines, 1,907 column definitions, 57 character-semantics columns, 6
"moved" LONG sections, PROBLEMS 0. `tools/desc_len_check.py`: `EXCEEDS: 0`. `tools/intro_list.py`: 783 fields, `block_diff
0`. `tools/header_fields.py`: "invariant violations: none" (189 `RTABLES`, 12 `RUSERS`). `tools/frame_t8.py`: 40
identical. `tools/lob_check.py` 141 OK, `tools/lob3_switch.py` 162 OK, `tools/lob_ru_check.py` 77 OK, `tools/misc_types.py`
106 OK, 0 differ. Every count equals session 10.

Session 17, the tools on the 10 new files. Before any change: `tools/exp_parse.py` stopped with an `IndexError` on
`long3_exp_18c.dmp` and `long3_exp_11g.dmp` (it read `fd ff` @0x3228f as a length of 65,533 and ran out of step); it
walked the other 8 files with "problems: none" (it already read `00 80`, `ef ff` and `f0 ff` as lengths).
`tools/rowlen_stats.py` listed the conventional LONG lengths from 0x8000 as findings and, by a flaw of its own, the
**value** lengths of the direct-path chains above 0xffef as "length field" findings. `tools/boundary.py` stopped
("gap") on the `fd ff` form. After the changes (`exp_parse.py`: the `fd ff` form for LONG / LONG RAW with flag 0;
`rowlen_stats.py`: conventional LONG forms counted apart, chain values by value length; `boundary.py`: the `fd ff`
form): `sha256sum -c SHA256SUMS` OK in `18c/` and `11g/`. `tools/check_all.py`: 95 / 89 / 27 files walked, `problems
0`, `rule_diff 0`, `empty_rule_diff 0`, `bad 0`; `no truth` for T_TSP, T_LONG3, T_SEG0 to T_SEG2 (checked by the two new
tools instead). `tools/corpus_counts.py` TOTAL: 211 files, `problems 0`, 404 sections, 41,284 rows, 243,303 row fields,
19,960 NULL, 783 text fields, 122 `ANALCOMPUTE` lines. `tools/rowlen_stats.py`: 418 binary parts, 0 of length 0, `high`
0, `len_8000_up` 0, `long_conv_8000_up` 6 and `long_conv_chunked` 6 (all in `long3_exp_<v>`), 24 direct-path chains (18
with more than one piece), type 8 largest 100,000. `tools/num_decode.py`: 86,124 NUMBER fields of 211 files, PROBLEMS 0.
`tools/collist.py`: PROBLEMS 0 (T_LONG3 "moved" on the direct path). `tools/desc_len_check.py`: `EXCEEDS: 0`.
`tools/intro_list.py`: 211 files, 783 fields, `block_diff 0`. `tools/header_fields.py`: "invariant violations: none".
`tools/frame_t8.py`: 44 identical, 0 different. `tools/boundary.py -q` on the four `long3` files: 52 record boundaries
each, all inside a value (chunks, pieces and fields cross 8192-byte boundaries with nothing inserted).
`tools/tsp_check.py`: differences 0. `tools/long3_check.py`: differences 0.

Session 19, the tools on the 10 new files, before any change: `tools/exp_parse.py` stopped with `IndexError` on
`long4_exp_18c.dmp` and `long4_exp_11g.dmp` (it read `fa ff` @0x22277 as a length of 65,530 and ran out of step) and
walked the other 8 with "problems: none". `tools/boundary.py` stopped ("gap at 0x32273") on the same two files. After the
changes (`exp_parse.py`: the `fa ff` form, `long_fa`; `boundary.py`, `rowlen_stats.py`: the same form): `sha256sum -c
SHA256SUMS` OK in `18c/` and `11g/`. `tools/check_all.py`: 101 / 93 / 27 files walked, `problems 0`, `rule_diff 0`,
`empty_rule_diff 0`, `bad 0`; `no truth` for T_LONG4, T_LONGRAW4, T_TSLTZ2 (as for T_TSP and T_LONG3 in session 17: checked
by the new tools instead); T_FRAME `ok` in the 6 R-048 files. `tools/corpus_counts.py` TOTAL: 221 files, `problems 0`, 420
sections, 41,378 rows, 243,593 row fields, 20,020 NULL, 783 text fields. `tools/rowlen_stats.py`: 434 binary parts,
`len0` 0, `high` 0, `len_8000_up` 0, `long_conv_8000_up` 12 (`long3` 6, `long4` 4, `longraw4` 2), `long_conv_fa` 6
(`long4`), `long_conv_chunked` 12, 42 direct-path chains; type 24 largest 100,000; type 231 smallest 7.
`tools/num_decode.py`: 86,176 NUMBER fields of 221 files, PROBLEMS 0. `tools/collist.py`: PROBLEMS 0 (T_LONG4 "moved").
`tools/desc_len_check.py`: `EXCEEDS: 0` (type 231 lengths 7 and 11). `tools/intro_list.py`: 221 files, `block_diff 0`.
`tools/header_fields.py`: "invariant violations: none". `tools/frame_t8.py`: 50 identical, 0 different.
`tools/boundary.py -q`: 56 record boundaries in each `long4` file and 32 in each `longraw4` file, all inside a value; none
in the row data of `tsltz2`. `tools/long3_check.py`, `tools/tsp_check.py`: differences 0 (unchanged by the new form).
`tools/long4_check.py`: differences 0 (6 files). `tools/tsltz2_check.py`: differences 0 (4 files).

| Script | Purpose |
|---|---|
| `tools/long5_check.py` | (session 20) R-051: per row of T_LONG5 (`long5_exp(_direct)_<v>`) and T_LONGRAW5 (`longraw5_exp(_direct)_<v>`, 18c and 11g) the form of the value (one field, `fa ff` + 4 bytes, `fd ff` chunks, direct-path chain) with its first 8 bytes and every u16, the value against `truth_long5.txt` / `truth_longraw5.txt` and the arithmetic, V; T_FRAME against `t_frame_exp(_direct)_<v>`; 18c against 11g from 0x2000; every encoded LONG RAW field of 11g against `longraw5_exp(_direct)_18c` (ids 1 to 7) and `longraw4_exp(_direct)_18c` (ids 8 to 11) (6.6) |
| `tools/long4_check.py` | (session 19) R-048: per row of T_LONG4 (`long4_exp(_direct)_<v>`) and T_LONGRAW4 (`longraw4_exp(_direct)_18c`) the form of the value (one field, `fa ff` + 4 bytes, `fd ff` chunks, direct-path chain) with every u16, the value against `truth_long4.txt` / `truth_longraw4.txt` and the arithmetic, V; T_FRAME against `t_frame_exp(_direct)_<v>`; 18c against 11g from 0x2000 (6.6) |
| `tools/tsltz2_check.py` | (session 19) R-050: T_TSLTZ2 descriptors against `truth_tsltz2.txt`, the rule "7 for f = 0, 11 otherwise", every field of the 4 rows against the decimal `DUMP` and the R-050 predictions, `fe ff` for NULL, the `TO_CHAR ... FF9` lines, field bytes compared between the 4 files (6.5) |
| `tools/tsp_check.py` | (session 17) R-047: T_TSP descriptors against `truth_tsp.txt` (type, data_length), every field of the 4 rows against the decimal `DUMP`, `fe ff` for NULL, rounded or truncated in row 3, the `TO_CHAR ... FF9` lines, field bytes compared between the 4 files (6.4) |
| `tools/long3_check.py` | (session 17) R-040: per row of T_LONG3 the form of L (one field, `fd ff` chunks, direct-path chain) with every u16, value against `truth_long3.txt` and the arithmetic, V; T_FRAME against `t_frame_exp(_direct)_<v>` (6.6) |
| `tools/lob_ru_check.py` | (session 10) the R-038 files: R_LOB2 descriptors, m, entries, k, every LOB value (offset, form, length, chunks, total) against `truth_lob_ru.txt` and `15_ru_lob2.sql`; CLOB decoded with `cp1251`, NCLOB with UTF-16 BE; the two client files byte by byte with each differing range named; R_TEXT against `ru_tbl_exp_*` (T8) (14.6) |
| `tools/desc_len_check.py` | (session 9) every column field against the maximum length of its descriptor, per type and form; CHAR values equal to / shorter than the descriptor; LOB locators and LONG excluded from "longer" (6.3) |
| `tools/frame_t8.py` | (session 9) rule T8 on every multi-table file of `18c/` and `11g/`: the T_FRAME section against `t_frame_exp(_direct)_<v>` byte for byte |
| `tools/lob3_switch.py` | (session 9) `lob3_exp_<v>`: form, byte length and chunks of every LOB value; "long iff larger than one chunk"; the total T; against `truth_lob3.txt` (14.6) |
| `tools/part2_check.py` | (session 9) `part2_exp(_direct)_18c`: the skeleton of the composite-partitioned section (lines, binary parts, how each ends, statistics items with their partition), rows per subpartition against `truth_part2.txt` and the log, identical descriptors (4.8) |
| `tools/lob_check.py` | (session 8) every LOB table: descriptors, m, flag, column entries, per row the locators, k and every LOB value (form, length, chunks); values against `truth_lob.txt` / `truth_ru.txt` and the SQL; two files of the same tables compared byte by byte with the field each differing byte lies in; `--frame`: T_FRAME against `t_frame_exp(_direct)_<v>` (14.6) |
| `tools/misc_types.py` | (session 8) INTERVAL, ROWID, UROWID of `misc_exp_<v>`: descriptors, bytes against `DUMP`, the conversion rules of 6.7 against the `TO_CHAR` / `ROWIDTOCHAR` text. Session 9: also `misc2_exp_<v>` (precisions from the `CREATE TABLE` line per column, f = 0, the logical UROWID text) |
| `tools/part_check.py` | (session 8) the partitioned table: items of the section, one binary part per partition, row counts against `truth_part.txt` and the log, every row against the SQL and the partition bounds, the `TOTAL` line (4.8) |
| `tools/num_decode.py` | (session 7) the NUMBER decoding procedure of 6.1 as a test: decode and re-encode every value of `truth_num.txt`, `truth_num2.txt`, `truth_numtypes.txt` and every NUMBER field of the fixtures; the 14 vectors of R-024. Session 9: `truth_num3.txt`, with digit bytes and significant digits per value. Session 10 (note A-24): step 3 maps expectations by column name (T_LONG, T_LONG2) to the `INSERT INTO` order; it had stopped with `KeyError: 0` |
| `tools/collist.py` | (session 7) the grammar of 4.7 on every `CREATE TABLE` line: names against `INSERT INTO`, type text against the descriptors, T_DECL against `truth_decl.txt`. Session 8: BLOB, NCLOB, ROWID, UROWID, INTERVAL. Session 9: the LONG column moved last on the direct path ("moved", compared by name); character-semantics descriptor lengths min(w x n, cap) |
| `tools/csmap_cmp.py` | (session 7) `truth_csmap.txt` against Python's `cp1251` / `cp1252` (14.11) |
| `tools/rowlen_stats.py` | (session 7) every length field of the row data: length 0, values 0xfff0 and up, smallest and largest per type (5.6). Session 8: LOB tables included; the LOB value area counted apart (forms, empty values, chunk lengths, k). Session 9: direct-path LONG piece chains counted apart; any other field u16 from 0x8000 to 0xffef is a finding. Session 17: conventional LONG fields from 0x8000 and in the `fd ff` form counted apart (`long_conv_8000_up`, `long_conv_chunked`); chain values counted by value length only. Session 19: the `fa ff` form (`long_conv_fa`); `long_conv_8000_up` up to 0xfff9, conventional path only |
| `tools/corpus_counts.py` | (session 7) the counts quoted in the rules file, per folder and in total; `--without=` reproduces the counts of an earlier session |
| `tools/exp_parse.py` | walks any EXP fixture from byte 0 to the end with the rules of this document; prints header, items (lines and fields) with their kind, descriptors, rows with offsets; reports gaps as "problems". Session 4: owner mode, pieces without marker, extended keyword list, the session 3 rules kept as `generic3` / `keyword3`, type 112 with the hypothesis of 14.6. Session 5: the keyword entry for `ANALSTATS` accepts a plain field or pieces after `fd ff`; labels for `T?` / `I?` / `C?` lines. Session 8: LOB tables by L1 to L4 (types 112 / 113, m, column entries, k, short and long values; `lob_values`), a partitioned table as one entry per binary part (`partition`). Session 9: direct-path LONG / LONG RAW piece chains (`long_pieces`); `COMPARTITION` / `SUBPARTITION` lines (`partition`, `subpartition`). Session 17: the conventional `fd ff` form of a LONG / LONG RAW value (`long_chunks`). Session 19: the conventional `fa ff` + 4-byte length form (`long_fa`) |
| `tools/stat_items.py` | (session 5) lists every `ANALSTATS` / `ANALCOMPUTE` item with its place and field form, decodes the three PL/SQL blocks, compares every number with `truth_stat.txt`, compares the letters and the number of plain fields with the `EXP-00091` lines of the log (sections 4.6, 13.9, 14.10) |
| `tools/stat_cmp.py` | (session 5) two files item by item: items only in one file, items that differ, binary parts of the tables (4.6: `statistics=none` / `compute`, direct path, client sets, versions) |
| `tools/intro_list.py` | (session 5) checks the list of introducing lines at the end of section 11 against every fixture. Session 7: also reads the blocks of `exp.md` and `exp_reader_rules.md` (`kind:length:forms:text`) and counts every difference from its own list (`block_diff`) |
| `tools/stc_items.py` | (session 6; session 7: the R-023 files against `truth_stc_extra.txt`, and "no item at all" for `statistics=none`) the R-020 files against `truth_stc.txt`: items after the rows of every table in file order, the column each column item names and its position, "item iff statistics", order, bytes between neighbouring items, `ANALCOMPUTE` lines, and the statistics items of several files byte by byte (section 4.6, "Columns without statistics") |
| `tools/header_date.py` | (session 6) place and format of the date text in the first record of every fixture (section 2) |
| `tools/check_truth.py` | compares the rows decoded by `exp_parse.py` with the truth files, field by field, with offsets. Session 4: O_PARENT, O_CHILD, O_EMPTY, T_BN1 to T_BN4, R_TEXT, R_CLOB, the Cyrillic-named table. Session 5: T_STAT1 (truth), T_STAT2, T_STAT3, R_STAT (SQL), T_STAT4 (0 rows). Session 6: T_STC1 to T_STC5 (SQL). Session 7: T_NUM2, T_NCHAR (truth), T_DECL, T_LONG, T_LONGRAW (SQL). Session 8: T_INTERVAL, T_ROWID (truth), T_PART per partition (SQL), T_LOB, T_LOB2 (ids and V here, LOB values against the SQL). Session 9: T_NUM3, T_NCHAR2 (with CL / NCL values), T_PROBE11, T_LOB3 (whole LOB values), T_INTERVAL2, T_ROWID2, T_LONG2, T_LONG_EMPTY, T_PART2 per subpartition; expectations by column name (T_LONG, T_LONG2). Session 10: R_LOB2 (ids, NULLs; whole LOB values against `15_ru_lob2.sql`, GETLENGTH and head / tail DUMPs against `truth_lob_ru.txt`); before, R_LOB2 fell into the Cyrillic-named-table branch and stopped the tool |
| `tools/check_all.py` | runs the item rules and both empty-table rules over every EXP file of one or more directories; counts sections, tables equal to truth, fields per type and path; reports where the session 3 rules fail or silently misread. Session 4: rewritten. Session 7: type names for NCHAR, LONG, LONG RAW. Session 8: type names for BLOB, NCLOB, INTERVAL, ROWID, UROWID. Session 9: header note only (every new table has a comparison in `check_truth.py`) |
| `tools/exp_items.py` | (session 4) item walk of several files side by side as a Markdown table (section 4.5) |
| `tools/boundary.py` | (session 4) which element lies on each multiple of RECORDLENGTH (section 1.2). Session 8: LOB tables included (k, LOB length fields, `fc ff`, total, chunks). Session 9: direct-path LONG piece chains. Session 17: the conventional `fd ff` LONG form. Session 19: the `fa ff` form |
| `tools/version_diff.py` | (session 4) 18c against 11g, pair by pair: header, blocks, items, table binary parts (section 13) |
| `tools/rowdata_cmp.py` | (session 4) the binary part of a table in exports with different client sets; UTF-8 validity of the values (section 14.1) |
| `tools/textitems_cs.py` | (session 4) character set of text items and identifiers; comments against the truth converted to the client set (sections 14.2, 14.3) |
| `tools/anal_lines.py` | (session 4) `ANALSTATS` / `ANALCOMPUTE` variants against the logs (section 14.5). Session 5: runs unchanged on 153 files (46 with such lines); `tools/stat_items.py` does more |
| `tools/header_fields.py` | header and block fields of many fixtures side by side; asserts the invariants of sections 1 to 3. Session 4: directories, three folders, differences between folders byte by byte |
| `tools/clientset_diff.py` | (session 3) what differs between two exports of the same table with different client character sets, by region. Session 4: repaired after the change of `exp_parse.py` (it had stopped with a `TypeError`) |
| `tools/tstz_fields.py` | (session 3) splits the 13 bytes of every TSTZ value and tests "UTC + hour+20 / minute+60" against `truth_tstz.txt`; compares TSLTZ rows 101 / 102. Runs unchanged on 11g |
| `tools/find_truth_values.py` | (session 1) offsets of every truth value in a dump |
| `tools/cmp_regions.py` | (session 1) `cmp -l` grouped into ranges plus an alignment. Slow on large zero-filled files; use `cmp -l` for those |
| `tools/exp_walk.py` | (session 1) walker for the two `t_frame` fixtures only; superseded by `exp_parse.py` |

## 13. Oracle 11g

Fixtures: `corpus/fixtures/11g/` (59 EXP files: the 46 stems of session 3 and the 13 of session
4; Oracle 11g XE; database AL32UTF8 / AL16UTF16) and, where named, `corpus/fixtures/11g-cp1251/`
(15 EXP files; database CL8MSWIN1251). The same SQL made the `18c/` and `11g/` folders, so every
18c file has an 11g partner. `truth_env.txt` of `11g/`:

    version=11.2.0.2.0
    NLS_LANGUAGE=AMERICAN
    NLS_TERRITORY=AMERICA
    NLS_CHARACTERSET=AL32UTF8
    NLS_NCHAR_CHARACTERSET=AL16UTF16
    dbtimezone=+00:00

The logs say "Oracle Database 11g Express Edition Release 11.2.0.2.0 - 64bit Production".
Tools: `tools/version_diff.py` (pair by pair), `tools/check_all.py`, `tools/header_fields.py`,
`tools/tstz_fields.py`, `tools/clientset_diff.py`, `tools/boundary.py`.

### 13.1 Result in one table

Session 5: 64 pairs now. The five `stat_exp*` pairs have the same number of items (111, or 60
with `statistics=none`); none of them is identical from offset R on (statistics values, 13.9), so
"44 identical" still names the same 44 pairs. The table below is the session 4 state.
Session 6: 66 pairs. The two `stc_exp*` pairs have 114 items each, 113 byte-identical, all 5
table binary parts identical (13.9); the same 44 pairs are identical from offset R on.
Session 7: 72 pairs (`tools/version_diff.py`). 49 are identical from offset R on: the 44, the four
pairs of R-024 to R-028 (`t_num2_exp_`, `t_decl_exp_`, `nchar_exp_`, `long_exp_`) and
`stc_exp_none_` (no statistics items, so nothing version-dependent is left). `stc_exp_us7ascii_`
differs in 10,487 bytes (statistics, 13.9: the column items of T_STC2.ADDED_LATER and of the
all-NULL columns are 309 / 315 bytes on 18c / 11g, the 11g `EPC := 2` form); its 5 table binary
parts are identical. Table binary parts over all pairs: 125 identical, 8 different (the 7 of
session 4 and T_STAT4 on the direct path, 13.2).
Session 8: 75 pairs (`part_exp_` has no 11g file). Still 49 identical from offset R on.
`misc_exp_` differs in 10 bytes, all inside the ROWID / UROWID values of T_ROWID (another database,
other addresses; 6.7); T_INTERVAL and T_FRAME are identical. `lob_exp_` and `lob_exp_direct_` have
the same 53 items in both versions; their T_FRAME parts are identical, the T_LOB and T_LOB2 parts
differ (the locators, 102 to 1102 bytes on 11g against 114 on 18c, and the chunk sizes of the long
values, 14.6), the bytes before the rows (m, flag, entries) are identical. Table binary parts over
all pairs: 129 identical, 13 different (the 8 above, T_ROWID, and T_LOB / T_LOB2 in two files).

`tools/version_diff.py` over the 59 pairs `X_18c.dmp` / `X_11g.dmp`:

| measure | result |
|---|---|
| pairs with the same number of items, the same form and kind of every item | 59 of 59 |
| pairs whose bytes from offset R to the end of the file are **identical** | 44 of 59 (every single-table and multi-table fixture of sessions 1 to 3 except `t_empty`, `t_probe6`, `multi_empty`, `multi_empty2`, `ddl`; and all 8 `t_bn*` files) |
| pairs that differ only in the first record | the same 44: the version line (`18.00` / `11.02`; conventional files 0x000c and 0x000f), the date text, the file name |
| table binary parts (column count to the LF after the rows) identical | 80 of 87 |
| table binary parts that differ | 7: T_PROBE6 on both paths (row order, 13.4); T_EMPTY and O_EMPTY on the direct path, 5 files (13.2) |
| differing text items | 26 x storage clause (13.3), 12 x last argument of `DBMS_STATS.SET_INDEX_STATS` (13.5), 9 x a date, an SCN or the database name inside the text, 39 x the number after `METRICEU` (13.6) |
| header fields other than the version line, the date and the file name | no difference in any pair |
| the two blocks and the string list of the second record | no difference: `ddl_exp_18c.dmp`@0x2000-0x2103 = `ddl_exp_11g.dmp`@0x2000-0x2103, and the same in every file of both folders (`tools/header_fields.py`) |

`tools/check_all.py` on `11g/`: 59 files walked to the two `EXIT` lines without a problem; 87
table sections, 87 with `INSERT INTO`; 85 tables equal to the truth, 2 equal to the SQL (T_PARENT);
0 differing; 3615 fields compared. Per type and path (equal / differing), the same numbers as on
18c:

| type | conventional | direct |
|---|---|---|
| VARCHAR2 (1) | 1463 / 0 | 1413 / 0 |
| NVARCHAR2 (1, form 2) | 40 / 0 | 40 / 0 |
| CHAR (96) | 40 / 0 | 40 / 0 |
| NUMBER (2) | 171 / 0 | 168 / 0 |
| DATE (12) | 41 / 0 | 41 / 0 |
| RAW (23) | 23 / 0 | 23 / 0 |
| TIMESTAMP (180) | 36 / 0 | 36 / 0 |
| TIMESTAMP WITH TIME ZONE (181) | 10 / 0 | 10 / 0 |
| TIMESTAMP WITH LOCAL TIME ZONE (231) | 10 / 0 | 10 / 0 |

(NUMBER counts only the compared NUMBER fields; id columns used as keys are not counted. The
conventional and direct counts differ because `t_frame_exp_rl4096` / `rl16384` and
`own_exp_w1252` have no direct partner.) No type exists in the fixtures of one version only. Type
code 112 (CLOB) occurs only in `11g-cp1251/` (section 14.6).

### 13.2 Difference: a table without rows on the direct path

| statement | evidence | status |
|---|---|---|
| 11g, direct path, table without rows: `ff ff` **is** written after the path flag, then LF, exactly as on the conventional path | `t_empty_exp_direct_11g.dmp`@0x2269 `00 00 01 00`, @0x226d `ff ff 0a 4d`; `multi_empty_exp_direct_11g.dmp`@0x2448 `ff ff 0a 54`; `multi_empty2_exp_direct_11g.dmp`@0x226d; `own_exp_direct_11g.dmp`@0x2fd9 `ff ff 0a 43`; `own_tables_exp_direct_11g.dmp`@0x2f44 (5 files, tables T_EMPTY and O_EMPTY; `truth_counts.txt` T_EMPTY 0, `truth_own.txt` `O_EMPTY\|count=0`) | confirmed for 11g |
| 18c, direct path, table without rows: no `ff ff` (section 5.4) | `t_empty/direct`@0x220a `0a 4d`; `own/direct`@0x2f71 `0a 43`; 5 files | confirmed for 18c |
| In all 11 direct-path cases of a table without rows, `ff ff` is absent exactly when the `CREATE TABLE` line of that table has no `STORAGE(...)` clause | 18c: `t_empty/direct`@0x211d, 152 bytes, `... MAXTRANS 255 TABLESPACE "USERS" LOGGING NOCOMPRESS` (no `STORAGE`); 11g: `t_empty_exp_direct_11g.dmp`@0x211d, 251 bytes, with `STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 FREELISTS 1 FREELIST GROUPS 1 BUFFER_POOL DEFAULT)`; the cut-short table of `ru_exp_direct_us7ascii_11g.dmp`@0x14cdc has `ff ff` and a `STORAGE` clause (@0x14b95) | observed |
| Session 5: one more table, this time with optimizer statistics (T_STAT4, `DBMS_STATS` on a table without rows): the same difference | 18c: `stat_exp_direct_18c.dmp`@0x4958 `01 00`, @0x495a `0a`, no `STORAGE` clause in the `CREATE TABLE` line (@0x4884 in that file; 145 bytes); 11g: `stat_exp_direct_11g.dmp`@0x498c `01 00`, @0x498e `ff ff`, @0x4990 `0a`, with `STORAGE` clause (244 bytes) | confirmed (6 files per version now). Gathering statistics does not create the segment on 18c |
| Why the versions differ | hypothesis: on 18c the empty table has no segment yet (the 18c `CREATE TABLE` text of every empty table lacks the storage clause, that of every table with rows has it), and the direct path writes nothing for a table without a segment; on 11g XE the empty tables have a segment. The fixtures cannot separate "version" from "segment": every 18c table without rows lacks the clause and every 11g one has it | hypothesis. Fixture request R-017 |
| **Session 17 (R-017): why the versions differ** | `seg_exp_direct_18c.dmp` (18c, direct): T_SEG0 and T_SEG1, empty **with** a segment (`truth_seg.txt` `segment_created=YES\|segments=1`), have a `STORAGE` clause (@0x211c, @0x227c) and `ff ff 0a` after the path flag (@0x226a, @0x23ca); T_SEG2, empty **without** a segment (`segment_created=NO\|segments=0`), has no clause (@0x23dc) and `0a` after the path flag (@0x24c7). The conventional file `seg_exp_18c.dmp` has `ff ff 0a` for all three (@0x226a, 0x23ca, 0x24c7). So on 18c the marker follows the segment: the hypothesis above is **confirmed for 18c**, and the 18c / 11g difference of the earlier rows is one of segments (the 11g empty tables had one), not of the version | confirmed (18c). 11g with a table without a segment: not in the fixtures (R-049; whether 11g XE 11.2.0.2 creates such a table is not known either) |
| **Session 19 (R-049): 11g XE cannot make a table without a segment** | `11g/truth_r049.txt` lines 28-32: `SEGMENT CREATION DEFERRED` is refused with `ORA-00439: feature not enabled: Deferred Segment Creation` (11g XE 11.2.0.2.0, line 5); `SEGMENT CREATION IMMEDIATE` works (lines 24-26). So the 11g half of "segment, not version" cannot be tested in the 11g XE container; every 11g empty table of the fixtures has a segment, which fits the 18c finding but does not test it. A real 11g Enterprise Edition dump may hold tables without a segment (assumed: the feature exists in EE); for them the 11g direct-path form is unknown, and rules E1 / E2 / E3 read either form | observed (the error); the EE consequence: hypothesis. No new request: no container at hand can answer it (5.4, session 19) |

Consequence for a reader: do not decide by version. After the path flag, on either path: `ff ff`
ends the rows (no rows if it comes first); on the direct path also accept "LF, then an item"
(rules E2 / E3 of section 11). Session 17: the cause is now known for 18c (no segment, no `ff ff`), which
confirms the rule: the version line cannot predict the form, because one 18c file holds both.

### 13.3 Difference: text of the statements for a table without rows

On 11g the `CREATE TABLE`, `CREATE INDEX` and `... PRIMARY KEY ... USING INDEX` statements of the
empty tables carry a `STORAGE(...)` clause, on 18c they do not; for tables with rows the texts are
identical in both versions.

| item | 18c | 11g |
|---|---|---|
| `CREATE TABLE "T_EMPTY"` | `t_empty`@0x211d, 152 bytes | `t_empty_exp_11g.dmp`@0x211d, 251 bytes |
| `CREATE TABLE "O_EMPTY"` | `own`@0x2e9b, 148 bytes | `own_11g`@0x2ea0, 247 bytes |
| `CREATE UNIQUE INDEX "O_EMPTY_PK"` | `own`@0x2f7a, 116 bytes | `own_11g`@0x2fe2, 215 bytes |
| `CREATE INDEX "O_EMPTY_IX"` | `own`@0x2fef, 111 bytes | `own_11g`@0x30ba, 210 bytes |
| `ALTER TABLE "O_EMPTY" ADD  CONSTRAINT "O_EMPTY_PK" PRIMARY KEY` | `own`@0x30d4, 151 bytes | `own_11g`@0x3202, 250 bytes |
| `CREATE TABLE "T_FRAME"` (table with rows) | CONV@0x211d, 261 bytes | `t_frame_exp_11g.dmp`@0x211d, 261 bytes, identical |

Status: confirmed (26 items in 11 file pairs). This is statement text, not layout; the items have
the same form and position. It moves every later offset of these files (99 bytes per statement).

### 13.4 Difference: row order of `t_probe6`

| statement | evidence | status |
|---|---|---|
| The 300 rows are written in another order than on 18c: 11g starts with R00257, 18c with R00065 | `t_probe6_exp_11g.dmp`@0x225e `06 00 52 30 30 32 35 37`, R00001 at @0x359e; `t_probe6_exp_direct_11g.dmp`@0x225e, R00001 at @0x3546; 18c: `t_probe6`@0x225e R00065, R00001 at @0x899e | confirmed |
| All 300 rows are present, each once, and equal the truth (600 fields per file); the row data have the same total length as on 18c (rows end at 0xa5a0 / 0xa348) | `tools/check_truth.py` | confirmed |

This confirms rule R6 (no row order may be assumed) rather than a format difference.

### 13.5 Differences inside statement texts

| statement | evidence | status |
|---|---|---|
| The last argument of `DBMS_STATS.SET_INDEX_STATS(...)` in the field after `ANALSTATS` is `6` on 18c and `0` on 11g | `ddl`@0x286f `36` / `ddl_exp_11g.dmp`@0x286f `30` (the only byte that differs from offset R on in this pair); `own`@0x289b / `own_11g`@0x28a0 | observed; meaning unknown (a statistics value, not layout). **Session 5, correction:** this holds for the files named here (indexes that were never analysed after their creation) and is **not** a property of the version: in the R-016 files 11g writes `6` for every index, table and column analysed with `DBMS_STATS` (`stat_exp_11g.dmp`@0x27e4 `...,50,1,7,1,1,1,0,6); END;`; `ru_stat_exp_cp1251_est_11g.dmp`@0x18c95), and both versions write `0` for the table analysed with `ANALYZE` (`stat_exp_18c.dmp`@0x435b, `stat_exp_11g.dmp`@0x432c). In `11g-cp1251/` the R-015 files have `0` and the R-016 files `6` for the same index R_TEXT_IX (`ru_tbl_exp_cp1251_11g.dmp`@0x11797 / `ru_stat_exp_cp1251_est_11g.dmp`@0x2ae60) |
| The `PROCACTION` block names the database: `export_db_name=>'XEPDB1', inst_scn=>'1957711'` (18c), `export_db_name=>'XE', inst_scn=>'470370'` (11g); the second piece is 141 / 136 bytes | `own`@0x214a (pieces 8, 141, 13, 0); `own_11g`@0x214a (8, 136, 13, 0) | observed |
| `ALTER PROCEDURE ... TIMESTAMP '...'` carries the compile time | `own`@0x5ceb; `own_11g`@0x5e7c | observed |
| Names of system-generated constraints (`SYS_C007604` / `SYS_C007115` in the truth) do not occur in the dumps: the unnamed primary key is written without a name | `own`@0x3337 `ALTER TABLE "O_PARENT" ADD  PRIMARY KEY ("ID") ...`; `own_11g`@0x34c8 | confirmed |

### 13.6 Difference: the number after `METRICEU`

| statement | evidence | status |
|---|---|---|
| The counter after the first section ("pre-schema procedural objects and actions", one `PROCACTION` item) is 8 on 18c and 12 on 11g; the section "post-schema procedural objects and actions" (no item in either version) adds 8 on 18c and 12 on 11g | `own`@0x21f7 `METRICEU 8`, @0x6102 `METRICEU 8` -> @0x6140 `METRICEU 16`; `own_11g`@0x21f2 `METRICEU 12`, @0x6293 `METRICEU 8` -> @0x62d1 `METRICEU 20`; the same 12 in `ru_exp_cp1251_11g.dmp`@0x09f2 | observed; no rule (section 7.1). The 18c export skipped a call in these two steps (`EXP-00008` in the log); whether that explains 8 against 12 is not known |
| In table mode the `METRICET` numbers are equal in both versions | `multi_framing`, `multi_types`, `multi_empty`, `multi_empty2`, `ddl`, `own_tables`: the lines are byte-identical (the first four pairs are identical from offset R on; `ddl_exp_11g.dmp`@0x2a97 `METRICET 3`, @0x2b0a `4`, @0x2c24 `5`, @0x2d60 `6`, @0x2e08 `8`; `own_tables_exp_11g.dmp`: 0, 0, 1, 1, 1, 2, 3, 3, ..., 5, 8) | confirmed |

### 13.7 What is identical, with 11g offsets

Because 44 pairs are byte-identical from offset R on, every 18c offset of sections 3 to 8 that
lies in one of these files is also the 11g offset. The rows below name one 11g offset per topic.

| topic (section) | 11g evidence | status for 11g |
|---|---|---|
| Header (2): `03`, client id, `EXPORT:V11.02.00` + LF, `UCORPUS`, `RTABLES`, RECORDLENGTH, `0`, `72`, `0`, three u16 BE, `00 01` + 8 zero bytes, file name length, spaces, date ending at 0x7f, file name at 0x80 | `t_frame_exp_11g.dmp`@0x0000 `03 03 69`, @0x0003 `EXPORT:V11.02.00`, @0x0014, @0x001c, @0x0024 `8192`, @0x0029, @0x002b, @0x002e, @0x0030 `03 69 03 69 07 d0`, @0x0036 `00 01 00 ...`, @0x0040 `00 1d`, @0x0069 date, @0x0080 `/fixtures/t_frame_exp_11g.dmp`; direct: `t_frame_exp_direct_11g.dmp`@0x0003 `D` LF, @0x0005 `EXPORT:V11.02.00` | confirmed; the only difference from 18c is the text `V11.02.00` (same length) |
| Client character set ids (2.1) | `t_frame_exp_us7ascii_11g.dmp`@0x0000 `03 00 01`, @0x0030 `00 01 03 69 07 d0`, @0x20a2 `00 01 00 01`; `t_frame_exp_w1252_11g.dmp`@0x0001 `00 b2`, @0x0030 `00 b2`, @0x20a2 `00 b2 00 b2`; `cmp` of the two files: 0x0002, 0x0031, 0x0041, date, file name, 0x20a3, 0x20a5 only | confirmed |
| Database character set at header 0x0032 and block 1 +0x3c | 873 in `11g/` (`t_frame_exp_11g.dmp`@0x0032 `03 69`, @0x203c `03 69`), 171 in `11g-cp1251/` (`ru_tbl_exp_cp1251_11g.dmp`@0x0035 `00 ab`, @0x083c `00 ab`); section 14.4 | confirmed (two databases) |
| RECORDLENGTH (1.1) | `t_frame_exp_rl4096_11g.dmp`@0x0024 `4096`, @0x1000-0x13ef = `t_frame_exp_11g.dmp`@0x2000-0x23ef = `t_frame_exp_rl16384_11g.dmp`@0x4000-0x43ef; 2048 and 8192 in `11g-cp1251/` | confirmed |
| Second record (3): two blocks, 8 strings, first item at R + 0x104 | `t_frame_exp_11g.dmp`@0x2000, 0x2066, 0x20cc `06 00 +00:00`, ..., @0x2104 `METRICST`; all 74 files of `11g/` and `11g-cp1251/` (`tools/header_fields.py`: no invariant violation) | confirmed; bytes equal to 18c |
| Table section, descriptors (4, 4.1) | `t_frame_exp_11g.dmp`@0x210d, 0x211d, 0x2223, 0x225d `03 00`, 0x225f `01 00 0a 00 69 03 01 00`, 0x2277 `00 00 00 00`; `t_text_exp_11g.dmp`@0x2285 `60 00 0a 00 69 03 01 00` (CHAR(10)), @0x228d `01 00 64 00 d0 07 02 00` (NVARCHAR2(50): length 100, id 2000, form 2); `t_ts_exp_11g.dmp`@0x2286 `b4 00`; `t_tstz_exp_11g.dmp`@0x2296 `b5 00`, @0x229a `e7 00`; `t_raw_exp_11g.dmp`@0x2241 `17 00 d0 07` | confirmed; values equal to 18c (873 / 2000, form 1 / 2) |
| Rows, NULL, row end, end marker (5.1 to 5.3) | `t_frame_exp_11g.dmp`@0x227b, @0x2295 `fe ff`, @0x228d `00 00`, @0x22f3 `ff ff 0a`; `t_len_exp_11g.dmp`@0x2c94 `ff 00`, @0x2d9c `00 01`; `t_nullmix_exp_11g.dmp` 24 fields equal the truth | confirmed |
| Record boundaries (1.2) | `t_probe6_exp_11g.dmp` (4 boundaries), `t_bn1_exp_11g.dmp`@0x3fff `eb 04`, `t_bn2_exp_direct_11g.dmp`@0x3fff `ed 04`, `t_bn3_exp_11g.dmp`@0x3fff `fe ff`, `t_bn4_exp_direct_11g.dmp`@0x3fff `fe ff`; RECORDLENGTH 2048: `ru_exp_cp1251_11g.dmp`@0x7fff `17 00` | confirmed |
| NUMBER, DATE, TIMESTAMP, RAW, character values (6.1 to 6.4, RAW of 6.5) | table of 13.1; e.g. `t_num_exp_11g.dmp`@0x2291 `03 00 c2 02 18`; `t_date_exp_11g.dmp`@0x224b `07 00 78 70 09 11 0e 20 02`; `t_ts_exp_11g.dmp`@0x22fa (11 bytes ending `00 00 00 01`); `t_raw_exp_11g.dmp`@0x224d `04 00 00 ff 80 7f`, @0x226a `d0 07`; `t_text_exp_11g.dmp`@0x22d4 `14 00 d0 9f ...`, @0x237f `d8 3d de 00` | confirmed |
| TIMESTAMP WITH TIME ZONE (6.5) | the 13 bytes follow the 18c layout (UTC date and time, u32 BE nanoseconds, hour + 20 / minute + 60, bit 0x80 for regions), and the region bytes are the same as on 18c: `t_tstz_exp_11g.dmp`@0x22a8 `78 74 03 1a 0f 05 38 07 5b ca 00 19 5a`, @0x22f8 `86 48` (Europe/Moscow), @0x230f `d0 04` (UTC), @0x2326 `81 90` (America/New_York); `tools/tstz_fields.py`: the same OK / DIFF lines as on 18c | confirmed. The fixtures show no trace of a different time zone file version: the three region codes are equal |
| TIMESTAMP WITH LOCAL TIME ZONE (6.5) | rows 101 and 102 byte-identical: `t_tstz_exp_11g.dmp`@0x233f and @0x2355 `78 74 03 1a 14 23 38 07 5b ca 00`. The 11g database time zone is +00:00 (`truth_env.txt`), as on 18c, so "UTC or database time zone" still cannot be answered | confirmed (H-016); question open |
| Client set changes nothing in the table section (2.1, 6.3) | `tools/clientset_diff.py`: `t_text_exp_11g.dmp` against `t_text_exp_w1252_11g.dmp` and `t_text_exp_us7ascii_11g.dmp`: section 0x210d-0x258f identical; direct: 0x210d-0x257b identical; differences only at 0x0001, 0x0030 (direct 0x0032), file name length, date, file name, 0x20a2-0x20a5 | confirmed |
| `ddl` (4.2, 4.3) | same 62 items, same forms and lengths; the `CALTER TABLE` + `fd ff` item is at the same offset: `ddl_exp_11g.dmp`@0x2878, 0x2885 (pieces 23, 22, 8, 25, 1, 18, 0); the trailer has the same groups (`ddl_exp_11g.dmp`@0x2af0, 0x2b96, 0x2d02, 0x2d8f) | confirmed |
| Owner mode (4.5) | `own_exp_11g.dmp`: the same 134 items as `own`; walk in 4.5 | confirmed |
| Trailer (7) | `t_frame_exp_11g.dmp`@0x22f6-0x23ef: the same 13 lines; `METRICET 7` @0x2320 | confirmed |

### 13.8 What is certain for which version

**Certain for both versions** (confirmed with 18c and 11g offsets): the header layout; RECORDLENGTH
and the zero fill; the second record; the item framing of section 4.2 (lines, plain fields, fields
in pieces with and without `fd ff`); the table section and descriptors for types 1, 2, 12, 23, 96,
180, 181, 231; rows, NULL marker, row end on the conventional path, `ff ff`; all value encodings of
section 6; "nothing is inserted at a record boundary"; owner mode; "row data are not converted to
the client set, text items are" (section 14: AL32UTF8 database on both versions).

**Certain for 18c only:** direct path, table without rows: no `ff ff` (5 files). The version text
`EXPORT:V18.00.00`. The counter values 8 / 16 of the two "procedural objects" sections.

**Certain for 11g only:** direct path, table without rows: `ff ff` is written (5 files, and the
cut-short table). The version text `EXPORT:V11.02.00`. Everything that needs a database character
set other than AL32UTF8 (section 14: header 0x0032 = 171, descriptor +4 = 171, row data in
CL8MSWIN1251, identifiers in the client set), type code 112 and a `direct=y` file with a
conventional-path table: the CL8MSWIN1251 database exists on 11g only. RECORDLENGTH 2048, and
"the direct path writes 8192 into the header although 2048 was asked for": `11g-cp1251/` only.

**Not tested on either version:** an `exp` client of another version than the database.


### 13.9 Statistics items: 18c against 11g (session 5)

Pairs `stat_exp*_18c.dmp` / `stat_exp*_11g.dmp` (5 pairs; `tools/stat_cmp.py`,
`tools/version_diff.py`, `tools/stat_items.py`). `stat18` = `stat_exp_18c.dmp`, `stat11` =
`stat_exp_11g.dmp`.

| statement | 18c | 11g | status |
|---|---|---|---|
| Same items, same forms, same order | 111 items (60 with `statistics=none`) | the same | confirmed (5 pairs). 104 of the 111 items of `stat18` / `stat11` are byte-identical |
| The seven items that differ | `stat18`@0x29a8, 0x2dc0, 0x2f42 (columns ID, N, D of T_STAT1), @0x49b0 (`CREATE TABLE "T_STAT4"`), @0x4b0a, 0x4c5a (columns of T_STAT4), @0x4e45 (`C` item) | `stat11`@0x29a8, 0x2da6, 0x2f14, 0x4981, 0x4b3e, 0x4c98, 0x4e8d | confirmed; the rows below explain each |
| Histogram of T_STAT1.ID and T_STAT1.D | `HYBRID` in the truth; `EPC := 10`, endpoint values `1,6,12,17,...,50`, endpoint numbers `1,6,12,...,50`, density `.019802` (`stat18`@0x2a60, 0x2aa5, 0x2ac2) | `HEIGHT BALANCED`; `EPC := 11`, `1,5,10,...,50`, `0,1,2,...,10`, density `.02` (`stat11`@0x2a4c, 0x2a94, 0x2aac) | confirmed against both truth files. A difference of the statistics the database holds, not of the layout |
| `SREC.EAVS` / `SREC.CHVALS` for the columns whose histogram has endpoint actual values in the truth (ID, N of T_STAT1; 18c truth only) | `SREC.EAVS := 4; SREC.CHVALS := DBMS_STATS.CHARARRAY(` and an extra piece `); `: 8 pieces (`stat18`@0x29aa, 0x2a36; @0x2dc2, 0x2e4c). Field 384 and 344 bytes | `SREC.EAVS := 0; SREC.CHVALS := NULL; `: 7 pieces (`stat11`@0x29aa, 0x2da8). Field 360 and 326 bytes | observed (2 columns x 4 files, 18c). Not seen on 11g, whose truth has no endpoint actual value |
| Column statistics of a table without rows (T_STAT4) | `EPC := 0`, both lists empty and their pieces absent: 5 pieces (`stat18`@0x4b0a: 119, 35, 38, 108, 0) | `EPC := 2`, `NUMARRAY(0,0)`, `NUMARRAY(0,1)`: 7 pieces (`stat11`@0x4b3e: 119, 35, 3, 38, 3, 108, 0), although the truth has no histogram row | observed (2 columns x 4 files per version) |
| Session 6: column statistics of a column that is NULL in every row, in a table **with** rows (T_STC3.ALLNULL, `truth_stc.txt` `stats:0:50`) | the same as for T_STAT4: `EPC := 0`, lists empty, 5 pieces (`stc_exp_18c.dmp`@0x45df: 119, 35, 38, 113, 0; 305 bytes) | the same as for T_STAT4: `EPC := 2`, `NUMARRAY(0,0)`, `NUMARRAY(0,1)`, 7 pieces (`stc_exp_11g.dmp`@0x45df: 119, 35, 3, 38, 3, 113, 0; 311 bytes) | observed (1 column x 2 files per version). It is the **only** item that differs between `stc_exp_18c.dmp` and `stc_exp_11g.dmp` (113 of 114 identical), and between the two direct files |
| `C` item: length of the padded index name | `SYS_C` + 123 spaces = 128 characters; field of 211 bytes (`stat18`@0x4e45 `d3 00`) | `SYS_C` + 25 spaces = 30 characters; field of 113 bytes (`stat11`@0x4e8d `71 00`) | observed (4 files per version). Hypothesis: the maximum length of an identifier in that version |
| Last argument of the three blocks | `6` for statistics made by `DBMS_STATS`, `0` for `ANALYZE` (T_STAT3) | the same | confirmed (13.5 corrected) |
| Last argument of `SET_INDEX_STATS` for an index that got its statistics when it was created (the files of session 4) | `6` (`ddl`@0x286f) | `0` (`ddl_exp_11g.dmp`@0x286f) | observed (13.5) |
| T_STAT4, `CREATE TABLE` text and direct path | no `STORAGE` clause (145 bytes, `stat18`@0x49b0); direct path: LF after the path flag, no `ff ff` (`stat_exp_direct_18c.dmp`@0x4958 `01 00`, @0x495a `0a`) | `STORAGE` clause (244 bytes, `stat11`@0x4981); direct path: `ff ff` (`stat_exp_direct_11g.dmp`@0x498c `01 00`, @0x498e `ff ff`, @0x4990 `0a`) | confirmed: the difference of 13.2 / 13.3, here for a table that **has statistics** |
| Table binary parts | - | identical in the 4 conventional pairs (4 of 4 tables each); in the direct pair T_STAT4 differs as above | confirmed |
| Fields of T_STAT2 and T_STAT3 (no histogram), and of T_STAT1.V (frequency histogram) | byte-identical in both versions | (`stat18`@0x377b = `stat11`@0x374c, 314 bytes; `stat18`@0x2b52 = `stat11`@0x2b38, 582 bytes) | confirmed |
| Letters and log | `S` / `R`, 24 lines with `R` and 6 `EXP-00091` with the US7ASCII client | the same | confirmed |

Nothing in this list changes the framing: on both versions an `ANALSTATS` line is followed by one
field, plain or in pieces. Everything of section 4.6 that rests on `ru_stat_exp_*` (CL8MSWIN1251
database, RECORDLENGTH 2048, the AL32UTF8 client with `R`) is 11g-only evidence.


## 14. Character sets

Which character set each part of a file is in, and which field tells a reader. Evidence from two
databases: AL32UTF8 (`18c/`, `11g/`) and CL8MSWIN1251 (`11g-cp1251/`, R-015), each exported with
several client sets. Tools: `tools/rowdata_cmp.py`, `tools/textitems_cs.py`,
`tools/header_fields.py`, `tools/check_truth.py`, `tools/anal_lines.py`.
Character set ids used: 1 US7ASCII, 171 CL8MSWIN1251 (`truth_ru.txt` `ENV|NLS_CHARSET_ID(CL8MSWIN1251)|171`),
178 WE8MSWIN1252, 873 AL32UTF8, 2000 AL16UTF16 (`truth_charsets.txt`).

### 14.1 Row data: the bytes of the database (F1)

| statement | evidence | status |
|---|---|---|
| The binary part of the table section of R_TEXT (column count, descriptors, `00 00`, path flag, 309 rows, `ff ff`, LF) is **byte-identical** for the four client sets US7ASCII, CL8MSWIN1251, AL32UTF8, WE8MSWIN1252 | conventional, 9 files, SHA-256 prefix `a5d66b925022` in all: `ru_tbl_exp_us7ascii_11g.dmp`, `ru_tbl_exp_cp1251_11g.dmp`, `ru_tbl_exp_al32utf8_11g.dmp`, `ru_tbl_exp_w1252_11g.dmp` @0x0ada-0x116b1 (rows from 0x0b10), `ru_tables_exp_us7ascii_11g.dmp`@0x0ad9-0x116b0 (rows from 0x0b0f), the four conventional `ru_exp_*` @0x27f4-0x133cb. Direct, 6 files, prefix `d8ab7e5185b7`: the four `ru_tbl_exp_direct_*` @0x22da-0x12c47 (rows from 0x2310), the two `ru_exp_direct_*` @0x3ff4-0x14961 (`tools/rowdata_cmp.py`) | confirmed |
| Every field of every row equals the database bytes of `truth_ru.txt`: 1854 fields per file (309 rows x 6 columns), 0 differ, in all 15 files | `tools/check_truth.py`. V, C, VC, N: `DUMP(col,16)`; LV: length and first 60 bytes from the truth, all bytes from the generating SQL; D: 7 bytes. E.g. row 2: V `0b 00 cf f0 e8 e2 e5 f2 2c 20 ec e8 f0` at `ru_tbl_exp_cp1251_11g.dmp`@0x4494 and, the same bytes, at `ru_tbl_exp_us7ascii_11g.dmp`@0x4494 and `ru_tbl_exp_al32utf8_11g.dmp`@0x4494; N `0c 00 04 1f 04 40 04 38 04 32 04 35 04 42` @0x44b7; direct: `ru_tbl_exp_direct_us7ascii_11g.dmp`@0x5c1a, 0x5c3d | confirmed |
| VARCHAR2 and CHAR values are CL8MSWIN1251 bytes, one byte per character; nothing is replaced with a US7ASCII or WE8MSWIN1252 client, nothing becomes UTF-8 with an AL32UTF8 client | row 3 V (the alphabet) `40 00 c0 c1 c2 ... ff` (64 bytes) @0x44dc; row 4 V `05 00 a8 b8 b9 ab bb` (Ё ё № « ») @0x457d; row 5 V `0a 00 93 94 84 97 96 85 95 88 99 a0` @0x45bb, in all 9 conventional files at the offsets of their region | confirmed |
| NVARCHAR2 values are UTF-16BE whatever the database and client sets are; they keep characters the database set does not have | row 6 N `08 00 00 e9 4e 2d 00 20 00 78` (é, a Chinese character, space, x) @0x461a, while V of the same row is `04 00 65 3f 20 78` @0x4602 (`3f` stored by the database: `truth_ru.txt` row 6 `65,3f,20,78`) | confirmed |
| CHAR(12) is padded with 0x20 to 12 bytes | row 5 C `0c 00 97 96 88 20 20 20 20 20 20 20 20 20` @0x45c7 | confirmed |
| A long value is one field: LV of 3000 bytes is `b8 0b` + 3000 bytes, LV of 2000 bytes `d0 07` | row 8 @0x4727 (next field @0x52e1); row 9 @0x5430; direct @0x5ea1, 0x6ba8 | confirmed (largest length field in the fixtures: 3000) |
| Together with section 6.3 (AL32UTF8 database, three client sets, UTF-8 bytes in every file) | 2 databases x 3 or 4 client sets x 2 paths | confirmed: **row data are the database bytes; the client set of the export changes nothing in them** |

Limits. (1) Two database sets, one of them single-byte, both on installations where client and
server are the same version. (2) What `exp` writes for a character the *database* cannot hold cannot
be tested: the database stored `?` at INSERT time (row 6). (3) No fixture has a database NCHAR set
other than AL16UTF16. (4) LONG, CLOB (14.6) and other LOB data are outside these statements.

### 14.2 Text items: the client character set (C6, F4)

| statement | evidence | status |
|---|---|---|
| The text of comments, CHECK conditions and views is written in the **client** character set of the export (header 0x0001), converted from the database set | AL32UTF8 database: comment `Café € д`: `own`@0x2ce3 `0e 00 27 43 61 66 c3 a9 20 e2 82 ac 20 d0 b4 27` (AL32UTF8 client; the 12 bytes of `truth_own_ddl.txt` `COLCOMMENT\|O_CHILD.PARENT_ID\|12`), `own_exp_w1252_18c.dmp`@0x2ce3 `0a 00 27 43 61 66 e9 20 80 20 bf 27` (WE8MSWIN1252 client); 11g: `own_11g`@0x2ce8, `own_exp_w1252_11g.dmp`@0x2ce8, same bytes. CL8MSWIN1251 database: comment on R_TEXT (truth `d2,e5,ea,f1,f2,fb`): `ru_exp_cp1251_11g.dmp`@0x135ab `08 00 27 d2 e5 ea f1 f2 fb 27`; `ru_exp_al32utf8_11g.dmp`@0x135ae `0e 00 27 d0 a2 d0 b5 d0 ba d1 81 d1 82 d1 8b 27`; `ru_exp_us7ascii_11g.dmp`@0x135ab `08 00 27 3f 3f 3f 3f 3f 3f 27`; `ru_exp_w1252_11g.dmp`@0x135ab `08 00 27 bf bf bf bf bf bf 27` | confirmed (2 databases, 4 client sets, 2 versions) |
| The length field of a text field counts the bytes **as written** (client set), not the database bytes | 14 against 10 for the same comment (`own`@0x2ce3 / `own_exp_w1252_18c.dmp`@0x2ce3); 8 against 14 (`ru_exp_cp1251_11g.dmp`@0x135ab / `ru_exp_al32utf8_11g.dmp`@0x135ae). The same holds for pieces: CHECK condition piece `17 00` (23 bytes, `v IS NULL OR v <> 'яяя'` in CL8MSWIN1251) at `ru_exp_cp1251_11g.dmp`@0x1355a against `1a 00` (26 bytes, UTF-8) at `ru_exp_al32utf8_11g.dmp`@0x1355a | confirmed |
| A character the client set does not have is **replaced, and the original cannot be recovered from the file**: `?` (0x3f) with a US7ASCII client, 0xbf with a WE8MSWIN1252 client. With US7ASCII the guillemets become `<` and `>` | `д` -> `bf`: `own_exp_w1252_18c.dmp`@0x2ced; Cyrillic -> `3f`: `ru_exp_us7ascii_11g.dmp`@0x135ae; `Значение («тест»)` (truth: 17 bytes `c7 ed e0 f7 e5 ed e8 e5 20 28 ab f2 e5 f1 f2 bb 29`) -> `ru_exp_us7ascii_11g.dmp`@0x135d9 `13 00 27 3f 3f 3f 3f 3f 3f 3f 3f 20 28 3c 3f 3f 3f 3f 3e 29 27` and -> `ru_exp_w1252_11g.dmp`@0x135d9 `13 00 27 bf bf bf bf bf bf bf bf 20 28 ab bf bf bf bf bb 29 27` (WE8MSWIN1252 has the guillemets) | confirmed for these characters. The full replacement table of a client set is not known |
| The comparison with the truth: every comment field of the 25 files tested equals the truth's database bytes converted to the client set (86 comments) | `tools/textitems_cs.py` on the 15 `ru_*` and the 10 `own_*` files: 146 items with non-ASCII or replaced text or comments, all decode in the client set; 86 of 86 comments equal the converted truth (the 300-byte comment by length and first 40 bytes) | confirmed |
| Row data in the same file stay in the database set (14.1). So one file holds two character sets when client and database set differ | `ru_exp_al32utf8_11g.dmp`: comment in UTF-8 @0x135ae, rows of the Cyrillic-named table in CL8MSWIN1251 @0x1377b `03 00 e0 e1 e2`; `own_exp_w1252_18c.dmp`: comment in WE8MSWIN1252 @0x2ce3, table binary parts identical to `own` (@0x2673, 0x2f61 / 0x2f65, 0x32e3 / 0x32e7) | confirmed |
| The view text and PL/SQL source are converted the same way | view: `ru_exp_cp1251_11g.dmp`@0x13962 `29 00 SELECT ... <> '` `ff ff ff` `'` `00 00`; `ru_exp_al32utf8_11g.dmp`@0x139d8 `2c 00 ... d1 8f d1 8f d1 8f ...`; `ru_exp_us7ascii_11g.dmp`@0x137a2 `... 3f 3f 3f ...`. PL/SQL source: ASCII only in the fixtures | view: confirmed; PL/SQL source with non-ASCII text: not in the fixtures |
| In a CL8MSWIN1251 file a text field can contain the bytes `ff ff` and `fe ff`-like sequences as text (я is 0xff) | `ru_exp_cp1251_11g.dmp`@0x13962 (view text with `27 ff ff ff 27`), @0x1355a (CHECK piece) | confirmed. Harmless for a reader that walks by lengths; fatal for one that searches for markers |

What changes between `own_exp_<v>.dmp` and `own_exp_w1252_<v>.dmp` after the first record (complete
list, `tools/clientset_diff.py` and item comparison): block 2 +0x3c / +0x3e (@0x20a2); the
`inst_scn` digits in the `PROCACTION` field (@0x21df, not a character set effect); the comment on
`O_CHILD.PARENT_ID` (@0x2ce3: length and bytes, 4 bytes shorter, which moves everything after it);
`ANALSTATS IS` -> `IR` (@0x288f, 0x306a / 0x3066) and `ANALCOMPUTE IS` -> `IR` (14.5). The binary
parts of the three tables are identical. Same list on 11g.

### 14.3 Identifiers: the client character set (F4)

| statement | evidence | status |
|---|---|---|
| Table, column and index names inside the items (`TABLE "..."`, `CREATE TABLE`, `INSERT INTO`, `CREATE INDEX`, `ANALSTATS`, `COMMENT ON`, the `SET_INDEX_STATS` argument) are written in the client set | table name `ТАБ_РУС` (truth `TABLE\|... d2,c0,c1,5f,d0,d3,d1`): `ru_exp_cp1251_11g.dmp`@0x135ef `54 41 42 4c 45 20 22 d2 c0 c1 5f d0 d3 d1 22`; `ru_exp_al32utf8_11g.dmp`@0x13606 `TABLE "` + `d0 a2 d0 90 d0 91 5f d0 a0 d0 a3 d0 a1` + `"`; index name (truth `c8,cd,c4,5f,d0,d3,d1`): `ru_exp_cp1251_11g.dmp`@0x1375c, `ru_exp_al32utf8_11g.dmp`@0x13791; direct: `ru_exp_direct_cp1251_11g.dmp`@0x14b85 | confirmed (2 client sets that have the letters, both paths) |
| The lines have no length field; their byte length is that of the client set | `INSERT INTO` line of that table: 56 bytes at `ru_exp_cp1251_11g.dmp`@0x136fb, 68 bytes at `ru_exp_al32utf8_11g.dmp`@0x13724 | confirmed |
| With a client set that lacks the letters the name is written with replacement characters, **in the dump too**: `???_???` (US7ASCII), `bf bf bf 5f bf bf bf` (WE8MSWIN1252); the column names become `???_1`, `???_2` | `ru_exp_us7ascii_11g.dmp`@0x135ef `TABLE "???_???"`, @0x135ff, @0x136fb; `ru_exp_w1252_11g.dmp`@0x135ef `54 41 42 4c 45 20 22 bf bf bf 5f bf bf bf 22`; `ru_exp_direct_us7ascii_11g.dmp`@0x14b85 | confirmed |
| Such a table is **cut short**: `exp` then asks the database for a table of that replaced name and fails (`EXP-00056` / `ORA-00942` in the conventional logs, `EXP-00008` / `ORA-00604` / `ORA-00942` in the direct log). The dump holds the `TABLE` line, the `CREATE TABLE` line, the `INSERT INTO` line, the column count, both descriptors, `00 00`, the path flag, then `ff ff` and LF: no row, although the table has 2 rows; and none of the statements that follow the rows in the complete files (index, `ANALSTATS`, two comments). The next item is the `ENDTABLE` of the owner-mode table list | complete: `ru_exp_cp1251_11g.dmp`@0x135ef-0x1390a: rows @0x13746 `03 00 e0 e1 e2 02 00 c1 02 00 00 fe ff 02 00 c1 03 00 00`, `ff ff` @0x13759, then `CREATE INDEX` @0x1375c, `ANALSTATS IS` @0x1382d, comments @0x1389f, 0x138d5, `ENDTABLE` @0x1390b. Cut short: `ru_exp_us7ascii_11g.dmp`@0x135ef-0x13748: `02 00 01 00 1e 00 ab 00 01 00 02 00 16 00 00 00 00 00` @0x13734, `ff ff 0a` @0x13746, `ENDTABLE` @0x13749; same offsets in `ru_exp_w1252_11g.dmp`; direct: `ru_exp_direct_us7ascii_11g.dmp`@0x14cca, flag `01 00` @0x14cda, `ff ff 0a` @0x14cdc, `ENDTABLE` @0x14cdf | confirmed (3 files) |
| The counter after `METRICEU` is not set by the cut-short table: it keeps the 309 of R_TEXT | `ru_exp_us7ascii_11g.dmp`: `METRICEU 309` after "synonyms", `310` after "views"; complete files: `2`, `3` (`ru_exp_cp1251_11g.dmp`) | observed |
| R_CLOB and R_TEXT, which stand before the failed table, are complete in the three files; the rest of the file (synonyms to the two `EXIT` lines) is written normally | `tools/check_all.py`: R_TEXT 309 rows equal the truth in all six `ru_exp_*` files; the walk ends without a problem | confirmed |

**How a reader can recognise a table section that was cut short.** The file gives no certain
sign: on the conventional path the bytes are those of an empty table (section 5.4). Signs that
the fixtures show, none of them proof:

1. the table name or a column name consists of or contains replacement characters (`?` or 0xbf)
   while the client set (header 0x0001) differs from the database set (header 0x0032);
2. the section has no item after the rows although tables of the same file have (weak: a table
   need not have an index or a comment);
3. (hypothesis, one file) on the direct path of 11g an empty table and a cut-short table look
   the same (`ff ff`), so the path does not help.

The certain source is the export log (`EXP-00056` or `EXP-00008` after the table's line). A reader
should deliver such a table as "0 rows, name contains replacement characters: possibly
incomplete", and never as a verified empty table.

### 14.4 Header and descriptor fields (F2, F3)

The binary header fields stand after the seven text lines; their offsets move with the lengths of
the user line, the mode line and the RECORDLENGTH text (0x0030 / 0x0032 / 0x0034 in `t_frame`,
0x0033 / 0x0035 / 0x0037 in `ru_tbl_exp_*`, 0x0032 / 0x0034 / 0x0036 in `ru_exp_*`, two more on the
direct path). "csA, csB, csC" below are the three u16 BE in that order.

| field | AL32UTF8 database (`18c/`, `11g/`) | CL8MSWIN1251 database (`11g-cp1251/`) | meaning | status |
|---|---|---|---|---|
| bytes 1-2, and csA | client id: 873, 1, 178 | client id: 1 (`ru_tbl_exp_us7ascii_11g.dmp`@0x0001 `00 01`, @0x0033 `00 01`), 171 (`ru_tbl_exp_cp1251_11g.dmp`@0x0001 `00 ab`, @0x0033), 873 (`ru_tbl_exp_al32utf8_11g.dmp`@0x0001 `03 69`), 178 (`ru_tbl_exp_w1252_11g.dmp`@0x0001 `00 b2`) | client character set of the export = character set of the text items and identifiers | confirmed |
| csB | 873 in all 118 files | **171** in all 15 files, for every client set (`ru_tbl_exp_us7ascii_11g.dmp`@0x0035 `00 ab`; `ru_tbl_exp_al32utf8_11g.dmp`@0x0035 `00 ab`; `ru_exp_w1252_11g.dmp`@0x0034 `00 ab`; direct `ru_tbl_exp_direct_us7ascii_11g.dmp`@0x0037 `00 ab`) | **database character set** | confirmed (was a hypothesis: two databases now, and the value does not follow the client) |
| csC | 2000 | 2000 (`ru_tbl_exp_cp1251_11g.dmp`@0x0037 `07 d0`) | a national character set; client and database NCHAR set are AL16UTF16 in every fixture | observed (unchanged: cannot be told which of the two) |
| block 1 +0x3c, +0x3e | 873, 2000 | 171, 2000 (`ru_tbl_exp_cp1251_11g.dmp`@0x083c `00 ab 07 d0`; `ru_tbl_exp_us7ascii_11g.dmp`@0x083c `00 ab 07 d0`; direct @0x203c) | database set, a national set (= csB, csC in all 133 files) | +0x3c confirmed; +0x3e observed |
| block 2 +0x3c, +0x3e | client id twice | client id twice (`ru_tbl_exp_us7ascii_11g.dmp`@0x08a2 `00 01 00 01`; `ru_tbl_exp_cp1251_11g.dmp`@0x08a2 `00 ab 00 ab`; `ru_tbl_exp_al32utf8_11g.dmp`@0x08a2 `03 69 03 69`) | follow the client set | confirmed |
| rest of both blocks | - | byte-identical to the AL32UTF8 files (`tools/header_fields.py`: "block1 +0x3c 03->00, +0x3d 69->ab; block2 same") | unknown | unknown |
| string list | `+00:00`, empty, `BYTE`, `UNUSED`, `2`, `INTERPRETED`, `DISABLE:ALL`, empty | the same 8 strings (`ru_tbl_exp_cp1251_11g.dmp`@0x08cc-0x0903) | unknown, except that the list did not change with the database set | observed |
| descriptor +4 of VARCHAR2 / CHAR | 873 | **171** (`ru_tbl_exp_cp1251_11g.dmp`@0x0ae4 `ab 00`, @0x0aec, @0x0af4, @0x0b04; same in all 15 files) | character set the value bytes are in = database set | confirmed (now with two databases; in every file it equals csB) |
| descriptor +4 of NVARCHAR2 | 2000 | 2000 (@0x0afc `d0 07`) | AL16UTF16 | confirmed |
| descriptor +6 | 1 / 2 | 1 for V, C, VC, LV (@0x0ae6 `01 00`), 2 for N (@0x0afe `02 00`) | 1 = database set, 2 = national set (hypothesis, unchanged) | observed |
| descriptor +2 | bytes | V VARCHAR2(200): 200 (@0x0ae2 `c8 00`); C CHAR(12): 12; **VC VARCHAR2(20 CHAR): 20** (@0x0af2 `14 00`); N NVARCHAR2(40): **80** (@0x0afa `50 00`); LV VARCHAR2(4000): 4000 (@0x0b02 `a0 0f`); ID NUMBER(5): 22; D DATE: 7 | maximum byte length | confirmed for byte-length columns and for NVARCHAR2 (2 bytes per declared character: 100 for (50) on 18c / 11g, 80 for (40) here). For `VARCHAR2(n CHAR)` the value 20 does **not** decide between "characters" and "bytes": in a single-byte database both are 20 (and the `CREATE TABLE` text keeps `VARCHAR2(20 CHAR)`, `ru_tbl_exp_cp1251_11g.dmp`@0x091c). Open question 4 stays open for a multi-byte database |

### 14.5 `ANALSTATS` / `ANALCOMPUTE` variants (F6)

`tools/anal_lines.py` over the 133 files of the three folders: 29 files contain such lines (84
lines).

| statement | evidence | status |
|---|---|---|
| Four line forms occur: `ANALSTATS IS "T"` + plain field (26), `ANALSTATS IR "T"` + plain field (16), `ANALCOMPUTE IS "T" ANALYZE  INDEX "ix"  ESTIMATE STATISTICS ` (26, a line without field), `ANALCOMPUTE IR ...` (16) | e.g. `ddl`@0x2807 (`IS`), `own_exp_w1252_18c.dmp`@0x2884 (`IR`), `ru_tbl_exp_cp1251_11g.dmp`@0x11781 (`IS`), `ru_tbl_exp_us7ascii_11g.dmp`@0x11781 (`IR`), `ru_tbl_exp_cp1251_11g.dmp`@0x119f6 (`ANALCOMPUTE IS`) / `ru_tbl_exp_al32utf8_11g.dmp`@0x11a0d (`ANALCOMPUTE IR`) | confirmed |
| The second letter is `R` exactly in the files whose log has `EXP-00091: Exporting questionable statistics.`, and `S` in the others | 29 of 29 files fit (13 with `R`, 16 with `S`); 0 contradict | confirmed for the fixtures |
| In the same 29 files the second letter is `R` exactly when the client set (header 0x0001) differs from the database set (csB) | `own` (873 / 873) `IS`, `own_exp_w1252_<v>` (178 / 873) `IR`, `ru_*_cp1251_*` (171 / 171) `IS`, `ru_*_us7ascii_*`, `ru_*_al32utf8_*`, `ru_*_w1252_*` (1, 873, 178 / 171) `IR` | confirmed for the fixtures. The two conditions coincide in every file, so the fixtures cannot say which one `exp` tests (other causes of "questionable statistics" are not in the fixtures) |
| The field after `ANALSTATS I?` is the same in both variants: `BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"index"',NULL,NULL,NULL,<8 numbers>); END;`, a plain field of 88 to 94 bytes; the index name inside is in the client set | `ru_exp_cp1251_11g.dmp`@0x13844 (88 bytes), `ru_exp_al32utf8_11g.dmp`@0x1388e (94 bytes) | confirmed |
| The first letter is `I` in every line: all statistics in the fixtures are index statistics | 84 of 84 lines | observed in session 4. **Session 5: superseded.** With the R-016 files there are 474 lines in 46 files: `ANALSTATS IS` 48, `IR` 38, `TS` 172, `TR` 116, `CS` 6, `CR` 2, `ANALCOMPUTE IS` 26, `IR` 16, `TS` 32, `TR` 18 (`tools/anal_lines.py`). What follows a `T` or `C` line: section 4.6 |
| Session 5: the letter is the same in every statistics line of a file, and the number of `EXP-00091` lines in the log equals the number of `ANALSTATS ?R` lines followed by a **plain** field (one per table with statistics, one per index item), not the number of lines with `R` | `stat_exp_us7ascii_18c.dmp`: 24 lines with `R`, 6 plain fields after `ANALSTATS ?R`, 6 messages; `ru_stat_exp_al32utf8_est_11g.dmp`: 22 / 6 / 6; `tools/stat_items.py`: equal in 46 of 46 files | confirmed for the fixtures |

For a reader: skip by the item rule of 4.2 (a field follows an `ANALSTATS` line; `ANALCOMPUTE` is a
line). Known from fixtures: `IS`, `IR`. Reported by the project owner for real dumps, not in any
fixture: `TR`.
Session 5: `TS`, `TR`, `CS`, `CR` are in the fixtures now (4.6); the field after an `ANALSTATS`
line can be in pieces after `fd ff` (rule S1 of section 11).

### 14.6 LOB columns: CLOB, NCLOB, BLOB (session 4: one CLOB table; session 8: R-027)

**Session 8.** The section is rewritten from `lob_exp_18c.dmp`, `lob_exp_11g.dmp` and their
`direct=y` files (R-027: T_LOB `ID NUMBER(3), C CLOB, B BLOB`, 5 rows; T_LOB2 `ID NUMBER(3), A CLOB,
V VARCHAR2(10), N NCLOB, B BLOB`, 2 rows; then T_FRAME; AL32UTF8 database) and checked again on the
7 R_CLOB files of session 4 (`11g-cp1251/`, CL8MSWIN1251 database, 4 client sets). The session 4
table is kept below the new one; its hypotheses are replaced (9.6). `tools/lob_check.py` walks
every LOB table with the layout below and compares every LOB value with `truth_lob.txt` /
`truth_ru.txt` and with the SQL: 60 checks per version in the `lob_exp` pair, 21 in the R_CLOB
files, 0 differ; `tools/check_all.py`: `problems 0` in all 177 files, and the tables after the
LOB tables (T_FRAME; R_TEXT and the Cyrillic table in `ru_exp_*`) are read and equal their truth.

**Layout of a table with a LOB column** (offsets: `lob_exp_18c.dmp`; 11g in brackets where they
differ; the bytes are the same in both versions unless a row says otherwise):

| offset | length | bytes | meaning | evidence | status |
|---|---|---|---|---|---|
| 0x23f5 (0x23ef) | 2 | `03 00` | column count (T4) | log, `INSERT INTO` line | confirmed |
| 0x23f7 (0x23f1) | 4 + 8 + 4 | `02 00 16 00`, `70 00 48 00 69 03 01 00`, `71 00 48 00` | descriptors: NUMBER; CLOB = type 112, 8 bytes; BLOB = type 113, 4 bytes (4.1) | 4.1 | confirmed |
| 0x2407 (0x2401) | 2 | `03 00` | **m**: where a table without a LOB column has `00 00` (T6), a table with one has its **column count** (T_LOB 3; T_LOB2 `05 00` @0x4db95 / 0x4df8e; R_CLOB `02 00` @0x11ae4 of `ru_tables_exp_us7ascii_11g.dmp`) | 3 tables, 15 sections; 0 in the 323 other binary parts | confirmed (that it equals the column count: 3 tables with 2, 3 and 5 columns) |
| 0x2409 (0x2403) | 2 | `00 00` | path flag, 0 (conventional) also in the `direct=y` files (log: "will be exported in conventional path") | 6 LOB tables in `direct=y` files | confirmed |
| 0x240b (0x2405) | 11 | `00 00 00` `01 43 00 00` `01 42 00 00` | **m column entries**, one per column in column order: u8 n, n bytes of the column name (n = 0 for a column that is not a LOB), 2 bytes `00 00`. T_LOB: ID (n = 0), `C`, `B`. T_LOB2 @0x4db99 / 0x4df92 (18 bytes): `00 00 00` `01 41 00 00` `00 00 00` `01 4e 00 00` `01 42 00 00` = ID, A, V, N, B. R_CLOB: `00 00 00` `02 43 4c 00 00` = ID, CL | 3 tables; the names equal the `INSERT INTO` names of the LOB columns | confirmed as framing (10 entries); the width of n (u8) is shown by names of 1 and 2 bytes only; the 2 bytes `00 00`: unknown |
| 0x2416 (0x2410) | | `02 00 c1 02` | first row; its fields are ordinary fields (R1 to R3) in column order | `truth_lob.txt` id 1 | confirmed |
| 0x241a (0x2414) | 2 + 114 (11g: 2 + 120) | `72 00` + locator | the field of a LOB column with a value (also an empty one) is a **locator**: u16 length + bytes, not the data. Its length varies on 11g (102 to 1102 bytes; BASICFILE, the data of a small LOB stand inside it in-row: `lob_exp_11g.dmp`@0x2676) and is 114 on 18c (SECUREFILE). One byte per locator differs between the conventional and the direct file (+0x57 on 18c, +0x39 on 11g). A reader skips it | `tools/lob_check.py` (field map of the differences) | confirmed as a field (every locator of 11 files is followed by the next field); content: observed, not needed |
| 0x2528 (0x2526) | 2 | `fe ff` | a NULL LOB is `fe ff` in its column, as for every type (row 2: C and B NULL) | `truth_lob.txt` `T_LOB\|2\|NULL\|NULL` | confirmed |
| 0x2502 (0x2500) | 2 | `02 00` | after the last field of a row on the conventional path: **k**, the number of LOB values that follow = the number of LOB columns of this row that are not NULL. k = 0 is the `00 00` of rule R4 (row 2 @0x252c) | rows of T_LOB: k = 2, 0, 2, 2, 2; T_LOB2: 3 (@0x4dd0e), 1 (@0x4dda3); R_CLOB: 1, 1, 0 | confirmed (49 rows: 38 with k > 0, 11 with k = 0) |
| 0x2504 (0x2502) | 2 + 18 | `12 00` + `00 68 00 65 00 6c 00 6c 00 6f 00 20 00 6c 00 6f 00 62` | first LOB value (column C): **short form** = u16 length n + n bytes. Here `hello lob` in **UTF-16 BE**: 9 characters, 18 bytes | `truth_lob.txt` len(C) = 9; SQL `TO_CLOB('hello lob')` | confirmed |
| 0x2518 (0x2516) | 2 + 10 | `0a 00` + `48 65 6c 6c 6f 20 62 6c 6f 62` | second LOB value (column B): the BLOB bytes | `truth_lob.txt` len(B) 10, `48656C6C6F20626C6F62` | confirmed |
| 0x261a (0x2600) | 2 + 2 + 2 | `02 00` `00 00` `00 00` | row 3, `EMPTY_CLOB()` and `EMPTY_BLOB()`: the columns hold locators, k = 2, and each value has **length 0**. NULL (no locator, not counted in k) and empty (locator, a value of length 0) differ | `T_LOB\|3\|0\|0`; T_LOB2 row 2: N empty @0x4dda5 `00 00` with A and B NULL | confirmed (3 empty values, 2 versions) |
| 0x270e (0x2ac8) | 2 + 6000; 2 + 1000 | `70 17` + 3000 x `00 6d`; `e8 03` @0x3e80 (0x423a) + 1000 x `ab` | row 4: 3000 characters = 6000 bytes in one short value; B 1000 bytes | `T_LOB\|4\|3000\|1000\|ABAB...` | confirmed |
| 0x4358 (0x475a) | 2 + 8 + chunks | `fc ff` `40 0d 03 00 00 00 00 00` then `f8 3e` + 16,120 bytes, ... | row 5, C: **long form** = `fc ff`, the total byte length as 8 bytes LE (200,000), then chunks `u16 n + n bytes` until their sum is the total; no terminator. 18c: 12 chunks of 16,120 + 1 of 6,560; 11g: 12 of 16,264 + 1 of 4,832. The text is `chunk001.....` in UTF-16 BE, 100,000 characters | `T_LOB\|5\|100000\|...`; SQL (100 x `RPAD('chunk' \|\| LPAD(i,3,'0'), 1000, '.')`) | confirmed (2 versions) |
| 0x350bc (0x354be) | 2 + 8 + chunks | `fc ff` `a0 86 01 00 00 00 00 00` | row 5, B: 100,000 bytes; 18c 12 x 8,060 + 3,280; 11g 12 x 8,132 + 2,416 | `T_LOB\|5\|100000\|100000\|CDCD...` | confirmed |
| 0x4d780 (0x4db82) | 3 | `ff ff 0a` | end of rows (E1), after the last row's LOB values | | confirmed |
| 0x4ddaa-0x4df92 (0x4e181-0x4e369) | 489 | `TABLE "T_FRAME"` ... | the next table is byte-identical to `t_frame_exp_<v>`@0x210d-0x22f5 (direct files: to `t_frame_exp_direct_<v>`, 475 bytes): rule T8, the LOB section ends exactly at the `ff ff` LF | `tools/lob_check.py --frame` | confirmed (4 files) |

| statement | evidence | status |
|---|---|---|
| CLOB and NCLOB have type 112 and an 8-byte descriptor; they differ in set id and form (873 / 1 against 2000 / 2; 171 / 1 for the CLOB of the CL8MSWIN1251 database). BLOB has type 113 and a 4-byte descriptor. The length is 72 for all of them | 4.1; 11 files | confirmed |
| Bytes before the rows: `m` (0 or the column count) instead of `00 00`, the path flag, and, if m > 0, m column entries. They depend on the table (column count, LOB column names), not on the version: identical in 18c and 11g | 3 tables; `lob_exp_18c` @0x2407-0x2415 = `lob_exp_11g` @0x2401-0x240f | confirmed |
| A LOB column's field in the row is a locator (or `fe ff`); the LOB data follow the row as k values in column order, after the `k` that takes the place of the `00 00` row end | 49 rows | confirmed |
| A LOB value is written in the short form (`u16 n + n`) up to at least 6,000 bytes and in the long form (`fc ff` + 8-byte length + chunks) at 100,000 and 200,000 bytes | 8 long values, 54 short (0 to 6,000 bytes) | confirmed as two forms; where Oracle switches from one to the other: unknown (no value between 6,001 and 99,999 bytes; R-035). A reader accepts both forms for any length |
| Chunk lengths of the long form are not constant across versions or types (16,120 / 8,060 on 18c, 16,264 / 8,132 on 11g; the last chunk is the rest); the CLOB chunk is twice the BLOB chunk (8,060 and 8,132 characters). Never assume a chunk size: sum to the total | `tools/rowlen_stats.py` (chunk lengths) | confirmed (4 values per version) |
| **CLOB data in an AL32UTF8 database are UTF-16 BE** (2 bytes per character for the ASCII values here: 9, 3000, 100,000 characters = 18, 6000, 200,000 bytes; `DBMS_LOB.GETLENGTH` counts characters), although the descriptor names set 873. NCLOB data are UTF-16 BE (set 2000). In the CL8MSWIN1251 database the CLOB data are the database bytes, 1 byte per character, with every client set (session 4) | `truth_lob.txt` lengths; R_CLOB `cf f0 e8 e2 e5 f2` | confirmed for ASCII text (AL32UTF8) and Cyrillic text (CL8MSWIN1251). Hypothesis: the CLOB set is AL16UTF16 whenever the database set is multi-byte (varying width). Characters outside the BMP (surrogate pairs; does `GETLENGTH` count them as 2?) and non-ASCII CLOB text in AL32UTF8: not in the fixtures (R-033, R-035) |
| Nothing is inserted at a multiple of R inside LOB values or chunks | `tools/boundary.py` (session 8: LOB tables included): 37 (`lob_exp_18c`) and 38 (`lob_exp_11g`) record boundaries fall inside the row data of the LOB tables, every one inside a value, locator or chunk; every value equals the SQL | confirmed |
| LOB tables are written on the conventional path in a `direct=y` file (flag 0, `D` LF in the header, log line "Table ... will be exported in conventional path."); they differ from the conventional file only in one byte inside each locator | `lob_exp_direct_<v>`; `tools/lob_check.py` | confirmed (2 versions; R_CLOB in 2 more files) |
| After the rows of a LOB table the trailer gets one empty group `TABLE "name"` / `ENDTABLE` per LOB column under "bitmap, functional and extensible indexes" (T_LOB 2, T_LOB2 3, R_CLOB 1), and the counter after `METRICET` grows by the same number | `lob_exp_18c.dmp`@0x4e016-0x4e03b, 0x4e129-0x4e168; `ru_tables_exp_us7ascii_11g.dmp`@0x134c3 | observed (3 tables). Rule T1 already reads them as statement groups |
| Not in the fixtures: BFILE; a LOB on the direct path (never written so here); a LOB in a single-byte database other than CL8MSWIN1251; ~~NCLOB or BLOB in a single-byte database~~ (session 10: R-038, below); ~~LOB values between 6,001 and 99,999 bytes~~ (session 9: R-035); ~~non-ASCII or non-BMP CLOB text in AL32UTF8~~ (session 9); a LOB above 4 GB (the 8-byte length would allow it); an `exp` client older than the database | - | unknown |

**Session 4 table (R_CLOB, `ru_tables_exp_us7ascii_11g.dmp`), kept for reference.** Its rows now
read as follows: 0x11ae4 `02 00` = m (column count 2); 0x11ae6 `00 00` = path flag; 0x11ae8 `00 00
00 02 43 4c 00 00` = the column entries of ID and CL (the session 4 reading "`00 00`, u16 BE name
length, name, `00 00`" is withdrawn, 9.6); 0x11af4 the CL locator; 0x11b62 `01 00` = k; 0x11b64 `06
00` + 6 bytes = the value in the short form. Row 2: k = 1 and a short value of 6000 bytes; row 3:
CL NULL, k = 0.

| offset (`ru_tables_exp_us7ascii_11g.dmp`) | length | meaning (session 4) | status (session 8) |
|---|---|---|---|
| 0x118e3 | 448 | `CREATE TABLE "R_CLOB" ("ID" NUMBER(3, 0), "CL" CLOB) ... LOB ("CL") STORE AS BASICFILE (...)` | confirmed (text) |
| 0x11ad6 | 2 | column count `02 00` | confirmed |
| 0x11adc | 8 | descriptor of CL: `70 00` type 112, `48 00` = 72, `ab 00` = 171, `01 00` | confirmed |
| 0x11ae4 | 2 | `02 00` where tables without a LOB have `00 00` | confirmed: m = column count |
| 0x11ae6 | 2 | `00 00` path flag | confirmed |
| 0x11ae8 | 8 | `00 00 00 02 43 4c 00 00` | confirmed as 2 column entries (reading of session 4 withdrawn) |
| 0x11af0 | 4 | first row `02 00 c1 02` | confirmed |
| 0x11af4 | 2 + 108 | locator of CL | confirmed as a field |
| 0x11b62 | 2 | `01 00` | confirmed: k = 1 |
| 0x11b64 | 2 + 6 | `06 00 cf f0 e8 e2 e5 f2`, database bytes | confirmed |
| row 2 | 2 + 6000 | `01 00`, `70 17` + 6000 bytes | confirmed |
| 0x13350 | 10 | row 3: `02 00 c1 04`, `fe ff`, `00 00` (k = 0); `ff ff` @0x13358 | confirmed |

Session 9 note on the table above: "`DBMS_LOB.GETLENGTH` counts characters" holds for BMP text only;
it counts UTF-16 code units (a non-BMP character counts 2), see below.

#### Session 9: LOB sizes around the short / long switch; non-ASCII and non-BMP CLOB text (R-035, R-033)

`lob3_exp_<v>.dmp`: T_LOB3 (`ID`, `C CLOB`, `B BLOB`, `N NCLOB`); rows 1 to 11: C = S characters `x`,
B = S bytes `5a`, N NULL, for S = 8000, 8060, 8061, 8132, 8133, 16120, 16121, 32766, 32767, 32768,
65536; row 20: C = N = `дж Café a` + U+1F600, B NULL. Conventional path (the request asked for no
direct file). Descriptors `70 00 48 00 69 03 01 00`, `71 00 48 00`, `70 00 48 00 d0 07 02 00`; m = 4,
column entries `00 00 00 01 43 00 00 01 42 00 00 01 4e 00 00` (rule L1). Every value equals the truth
lengths and the SQL contents (`tools/check_truth.py`: 36 values per file, 0 DIFF; `tools/lob3_switch.py`:
162 checks over both files, 0 differ). Offsets of the value (its first u16), 18c / 11g; C in bytes = 2S.

| id | S | C (CLOB) 18c | C 11g | B (BLOB) 18c | B 11g |
|---|---|---|---|---|---|
| 1 | 8000 | short 16000 @0x25ff | short @0x25ea | short 8000 @0x6481 | short @0x646c |
| 2 | 8060 | short **16120** @0x84b3 | short @0x8492 | short **8060** @0xc3ad | short @0xc38c |
| 3 | 8061 | **long** 16122: 16120 + 2 @0xe41b | short 16122 @0xe3ee | **long** 8061: 8060 + 1 @0x12323 | short @0x122ea |
| 4 | 8132 | long 16120 + 144 | short **16264** @0x1434d | long 8060 + 72 | short **8132** @0x182d7 |
| 5 | 8133 | long 16120 + 146 | **long** 16264 + 2 @0x1a389 | long 8060 + 73 | **long** 8132 + 1 @0x1e321 |
| 6 | 16120 | long 2 x 16120 | long 16264 + 15976 | long 2 x 8060 | long 8132 + 7988 |
| 7 | 16121 | long 2 x 16120 + 2 | long 16264 + 15978 | long 2 x 8060 + 1 | long 8132 + 7989 |
| 8 | 32766 | long 4 x 16120 + 1052 (@0x38040: marker `fc ff`, then T = `fc ff 00 00 00 00 00 00` = 65,532: the first two bytes of T can look like the marker; position decides) | long 4 x 16264 + 476 | long 4 x 8060 + 526 | long 4 x 8132 + 238 |
| 9 | 32767 | long 4 x 16120 + 1054 | long 4 x 16264 + 478 | long 4 x 8060 + 527 | long 4 x 8132 + 239 |
| 10 | 32768 | long 65,536: 4 x 16120 + 1056 (T = `00 00 01 00 00 00 00 00` @0x68269) | long 4 x 16264 + 480 | long 32,768: 4 x 8060 + 528 | long 4 x 8132 + 240 |
| 11 | 65536 | long 131,072: 8 x 16120 + 2112 | long 8 x 16264 + 960 | long 65,536: 8 x 8060 + 1056 | long 8 x 8132 + 480 |

| statement | evidence | status |
|---|---|---|
| A LOB value is written in the **long** form exactly when its byte length is larger than one chunk; the chunk is 16,120 bytes for CLOB and 8,060 for BLOB on 18c, 16,264 and 8,132 on 11g. The largest short value is one full chunk (18c: 16,120 / 8,060 at row 2; 11g: 16,264 / 8,132 at row 4) | table; `tools/lob3_switch.py` "long iff > one chunk" 44 times OK | confirmed (2 versions, CLOB and BLOB, 11 sizes each). NCLOB: only short values (22 bytes) in the fixtures; its chunk size is unknown (assumed = CLOB) |
| In the long form every chunk but the last is exactly one chunk, the last is the rest (1 to one chunk); the 8 bytes after `fc ff` are the total byte length T; bytes 4 to 7 of T are 0 in every value (T up to 131,072 = `00 00 02 00 00 00 00 00`) | `tools/lob3_switch.py`; e.g. `lob3_exp_18c`@0x8037f | confirmed; whether T is u64 or u32 + 4 bytes: still unknown |
| A short value never reaches 0xfff0 (16,264 at most), so its u16 cannot collide with `fc ff` | table | confirmed (as a consequence of the chunk size) |
| A record boundary can split the `fc ff` marker: `lob3_exp_11g.dmp`@0x47fff `fc`, @0x48000 `ff` (row 8, B); nothing is inserted (R5) | `tools/boundary.py` | confirmed (1 case) |
| Row 20 (non-ASCII, non-BMP): C and N are both short, 22 bytes `04 34 04 36 00 20 00 43 00 61 00 66 00 e9 00 20 00 61 d8 3d de 00` (UTF-16 BE; U+1F600 as the surrogate pair `d8 3d de 00`); `GETLENGTH` = 11 for both = UTF-16 code units (10 characters) | 18c C @0xb04a7, N @0xb04bf; 11g @0xb04e6, @0xb04fe; truth `T_LOB3\|20\|11\|NULL\|11\|...d0,b4,d0,b6,20,43,61,66,c3,a9,20,61,f0,9f,98,80` | confirmed (2 versions; also `nchar2`, 6.3) |
| So the CLOB of an AL32UTF8 database is UTF-16 BE for non-ASCII text too (Cyrillic `04 34`, Latin-1 `00 e9`), not UTF-8 and not CESU-8; `GETLENGTH` of a CLOB / NCLOB counts code units (session 8 wrote "characters": true for BMP text only) | rows 20; `nchar2` | confirmed |
| `t_frame` after `t_lob3`: byte-identical to `t_frame_exp_<v>` (rule T8) | `lob3_exp_18c`@0xb04da-0xb06c2, `lob3_exp_11g`@0xb0519-0xb0701; `tools/frame_t8.py` | confirmed |
| A CLOB in a **single-byte** database with values around the switch (1 byte per character: is the chunk then 8,060 characters = 8,060 bytes?), NCLOB / BLOB in a single-byte database | not in these files (`lob3_exp_cp1251_11g.dmp` was optional and was not generated). Session 10: NCLOB and BLOB in the CL8MSWIN1251 database, and the CLOB chunk there (8,132 bytes on 11g), are answered by R-038 (below); the values **around** the switch are still not in the fixtures (R-046) | unknown (session 10: partly answered) |

#### Session 10: LOB values in the CL8MSWIN1251 database (R-038)

`ru_lob2_exp_cp1251_11g.dmp` and `ru_lob2_exp_us7ascii_11g.dmp` (11g, database CL8MSWIN1251, clients CL8MSWIN1251 and
US7ASCII; table "Fixtures added in session 10"). R_LOB2 (`ID NUMBER(3), C CLOB, N NCLOB, B BLOB`): row 1 = the 66
characters `blk` = bytes 0xC0 to 0xFF, 0xA8, 0xB8 (`А`..`я`, `Ё`, `ё`) in C, N and B; row 2 = 303 x `blk` + its first two
characters = 20,000 characters in each; row 3 all NULL. `tools/lob_ru_check.py`: 77 checks, 0 differ;
`tools/check_truth.py`: R_LOB2 12 fields and 9 LOB values per file equal the truth and the SQL, R_TEXT 1,854 fields
equal `truth_ru.txt`. Offsets below are those of `ru_lob2_exp_cp1251_11g.dmp`; **the US7ASCII file has the same bytes at
the same offsets** from 0x2104 to 0x16282 except one byte inside each locator (last table of this subsection).

| offset (hex) | length | bytes | meaning | evidence | status |
|---|---|---|---|---|---|
| 0x24d9 | 2 | `04 00` | column count 4 (T4) | `INSERT INTO "R_LOB2" ("ID", "C", "N", "B")` @0x2496 | confirmed (2 files) |
| 0x24db | 4 + 8 + 8 + 4 | `02 00 16 00`, `70 00 48 00 ab 00 01 00`, `70 00 48 00 d0 07 02 00`, `71 00 48 00` | NUMBER; CLOB type 112 with set **171** form 1; NCLOB type 112 with set 2000 form 2; BLOB type 113 (T5) | CREATE TABLE @0x211c; 4.1 | confirmed (2 files; CLOB 171 also in R_CLOB) |
| 0x24f3 | 2 + 2 | `04 00` `00 00` | m = 4 = column count, path flag 0 (T6, L1) | | confirmed |
| 0x24f7 | 15 | `00 00 00` `01 43 00 00` `01 4e 00 00` `01 42 00 00` | 4 column entries: ID (n = 0), `C`, `N`, `B` (L1) | `INSERT INTO` names | confirmed |
| 0x2506 | 4 | `02 00 c1 02` | row 1, ID = 1 | `truth_lob_ru.txt` id 1 | confirmed |
| 0x250a, 0x25b4, 0x26a0 | 2 + 166, 2 + 232, 2 + 166 | `a8 00` .., `ea 00` .., `a8 00` .. | the three locators (L2). 168 = 102 + 66 and 234 = 102 + 132: on 11g (BASICFILE) the locator of a small LOB holds the value in-row (C: `00 42` = 66 @0x256a, then the 66 bytes @0x2572-0x25b3, `c0 c1` .. `a8 b8`; the locator then ends). A reader skips it all the same (L2) | | observed (content); confirmed as fields |
| 0x274a | 2 | `03 00` | k = 3 (L3) | 3 non-NULL LOB columns | confirmed |
| 0x274c | 2 + 66 | `42 00` + `c0 c1 c2 .. fe ff a8 b8` | C, **short**, 66 bytes = the **database bytes**, 1 per character (L5) | truth GETLENGTH 66, head `c0,c1,c2,c3`, tail `fe,ff,a8,b8`; decodes with `cp1251` to `А..яЁё` | confirmed (2 client sets) |
| 0x2790 | 2 + 132 | `84 00` + `04 10 04 11 .. 04 4f 04 01 04 51` | N, **short**, 132 bytes, **UTF-16 BE** (U+0410 to U+044F, U+0401, U+0451) | truth GETLENGTH 66, head `4,10,4,11,4,12,4,13`, tail `4,4e,4,4f,4,1,4,51` | confirmed (2 client sets) |
| 0x2816 | 2 + 66 | `42 00` + the 66 bytes of C | B, short, the bytes | `UTL_RAW.CAST_TO_RAW(blk)`; GETLENGTH 66 | confirmed |
| 0x285a | 4 | `02 00 c1 03` | row 2, ID = 2; locators 114, 122, 114 bytes @0x285e, 0x28d2, 0x294e (out of row: 114 / 122 < 102 + value) | | confirmed (fields) |
| 0x29c2 | 2 | `03 00` | k = 3 | | confirmed |
| 0x29c4 | 2 + 8 | `fc ff` `20 4e 00 00 00 00 00 00` | C, **long** form; T = 20,000 = the database byte count (not 40,000 = UTF-16) | truth GETLENGTH 20000 | confirmed (2 client sets) |
| 0x29ce, 0x4994, 0x695a | 2 + 8,132, 2 + 8,132, 2 + 3,736 | `c4 1f` .., `c4 1f` .., `98 0e` .. | the chunks of C: **8,132 + 8,132 + 3,736** = 20,000; the database bytes `c0 c1 ..` continue across the chunk borders (`blk` restarts at byte 66 x k) | `tools/lob_ru_check.py`; 15_ru_lob2.sql; truth tail `a8,b8,c0,c1` = the last 4 bytes @0x77f0 | confirmed |
| 0x77f4 | 2 + 8 | `fc ff` `40 9c 00 00 00 00 00 00` | N, **long** form, T = 40,000 bytes = 20,000 UTF-16 code units: the **first long NCLOB** of the corpus | truth GETLENGTH 20000 | confirmed (2 client sets) |
| 0x77fe, 0xb788, 0xf712 | 2 + 16,264, 2 + 16,264, 2 + 7,472 | `88 3f` .., `88 3f` .., `30 1d` .. | chunks of N: **16,264 + 16,264 + 7,472** = 40,000 (= the CLOB chunk of the AL32UTF8 database on 11g) | truth tail `4,1,4,51,4,10,4,11` | confirmed |
| 0x11444 | 2 + 8 + chunks | `fc ff` `20 4e 00 00 00 00 00 00`, `c4 1f` .. | B, long: **8,132 + 8,132 + 3,736** = 20,000 bytes (= the BLOB chunk of 11g in AL32UTF8) | GETLENGTH 20000; bytes = C | confirmed |
| 0x16274 | 2 + 2 + 2 + 2 + 2 | `02 00 c1 04` `fe ff` `fe ff` `fe ff` `00 00` | row 3: ID 3, three NULL LOB columns, k = 0 (@0x1627e) | truth `R_LOB2\|3\|NULL\|...` | confirmed |
| 0x16280 | 3 | `ff ff 0a` | end of rows (E1) | | confirmed |
| 0x16283-0x27027 | 69,029 | `TABLE "R_TEXT"` .. `ff ff 0a` | R_TEXT section **byte-identical** to `ru_tbl_exp_cp1251_11g.dmp`@0x90d-0x116b1 (and the US7ASCII file to `ru_tbl_exp_us7ascii_11g.dmp`@0x90d-0x116b1, R-015b, RECORDLENGTH 2048 there): rule T8; its 309 rows equal `truth_ru.txt` | `tools/lob_ru_check.py`, `tools/check_truth.py` | confirmed |

| statement | evidence | status |
|---|---|---|
| In the CL8MSWIN1251 database a CLOB value (descriptor set 171) is the **database bytes**, 1 byte per character, in the short **and the long** form; the 8-byte total of the long form counts these bytes. A reader decodes it with the descriptor set; Python's `cp1251` gives exactly `blk` x 303 + `АБ` for the 20,000 bytes (the request text said `Аб`; the bytes and the truth tail say `c0 c1` = `АБ`) | 0x274c, 0x29c4; `tools/lob_ru_check.py` "C decoded with cp1251" OK twice per file | confirmed (2 files, 2 values, 2 client sets) |
| An NCLOB (set 2000, form 2) is UTF-16 BE in the single-byte database too, short and long; decoded it equals the CLOB text character for character | 0x2790, 0x77f4; "N decoded as UTF-16 BE = C decoded" | confirmed (2 files) |
| A BLOB is the bytes, short and long, as in AL32UTF8 | 0x2816, 0x11444 | confirmed |
| Chunk sizes of 11g: CLOB of the single-byte database **8,132 bytes**, NCLOB **16,264 bytes**, BLOB 8,132 bytes. With session 8 / 9 (11g, AL32UTF8: CLOB 16,264, BLOB 8,132) every 11g chunk is **8,132 storage units**, a unit being 1 byte (single-byte CLOB, BLOB) or 2 bytes (UTF-16 CLOB, NCLOB). The session 8 sentence "the CLOB chunk is twice the BLOB chunk" holds for a UTF-16 CLOB only | 0x29ce, 0x77fe, 0x1144e | confirmed as sizes (11g, 1 table here); "8,132 units": hypothesis (consistent with 4 kinds of LOB on 11g; 18c in a single-byte database not in the fixtures) |
| In every long value here every chunk but the last is full and bytes 4 to 7 of T are 0 (as L4) | `tools/lob_ru_check.py` | confirmed |
| The LOB value area and the locator lengths do **not** depend on the client set: the R_LOB2 binary part (0x24d9-0x16282) of the two files differs in exactly 6 bytes, one per locator, each at locator + 0x39 (`07` / `08`: 0x2543, 0x25ed, 0x26d9, 0x2897, 0x290b, 0x2987). That is the same position as the byte that differs between a conventional and a `direct=y` export of `lob_exp_11g` (+0x39, session 8): it changes from one export to the next, whatever changes in the options | `tools/lob_ru_check.py` (differing ranges); `tools/rowdata_cmp.py` | confirmed (values identical); the meaning of the byte: unknown (not needed) |
| The other differences between the two files are the header (client id @0x0002, 0x0034; file-name length @0x0044; date seconds @0x0079-0x007a; file name @0x0096-0x00a5), block 2 (@0x20a3, 0x20a5: client id) and 3 text items in the trailer after R_TEXT, written in the client set (C2): the CHECK text `'яяя'` @0x27158 (`ff ff ff` / `3f 3f 3f`) and the two comments @0x27197, 0x271c5 (`?` for each Cyrillic letter, `«»` as `<>`). Nothing else differs; both files are 163,840 bytes | `cmp -l`; `tools/textitems_cs.py` (3 items, all equal to the truth converted to the client set) | confirmed |
| A record boundary falls 10 times inside the R_LOB2 value area (0x4000 to 0x16000, every 0x2000), always inside a chunk; nothing is inserted (R5) | `tools/boundary.py` | confirmed |
| Where the short form ends for C and N in this database (H-a: long iff larger than one chunk, i.e. above 8,132 bytes for C and 16,264 bytes for N; H-b: a fixed 16,264-byte threshold for C) | the files hold 66 and 20,000 characters only; both hypotheses predict these two forms | unknown (R-046). A reader does not need it (it accepts both forms) |

### 14.7 RECORDLENGTH on the direct path (F5)

| statement | evidence | status |
|---|---|---|
| All 15 exports were asked for `recordlength=2048`. The 9 conventional files say `2048` in the header and are multiples of 2048 | `ru_tbl_exp_cp1251_11g.dmp`@0x0027 `2048`, size 73728 = 36 x 2048; `ru_exp_cp1251_11g.dmp`@0x0026, 81920 = 40 x 2048; `ru_tables_exp_us7ascii_11g.dmp` 79872 = 39 x 2048 | confirmed |
| The 6 direct-path files say **`8192`** and are multiples of 8192 | `ru_tbl_exp_direct_cp1251_11g.dmp`@0x0029 `8192`, size 81920 = 10 x 8192; `ru_exp_direct_cp1251_11g.dmp`@0x0028, 90112 = 11 x 8192 | confirmed (the argument is in `corpus/scripts/make_ru.sh` line 135 and `make_ru_tables.sh` line 28; scripts read, not run) |
| The second record starts at the RECORDLENGTH **of the header** in every file: the block signature `64 00 00 00` stands there and the first item at R + 0x104 | conventional: `ru_tbl_exp_cp1251_11g.dmp`@0x0800, `METRICST` @0x0904; direct: `ru_tbl_exp_direct_cp1251_11g.dmp`@0x2000, @0x2104; all 133 files (`tools/header_fields.py`) | confirmed |
| Nothing is inserted at multiples of 2048 either | section 1.2 | confirmed |

A reader must take R from the header text and never assume a value or the value that was asked
for. Whether the direct path always writes 8192, or the smallest allowed size, is not known (one
requested value).

### 14.8 Rule for a reader: which character set to use

| part of the file | character set | where the reader finds it | status |
|---|---|---|---|
| VARCHAR2 and CHAR values (type 1 / 96 with +6 = 1) | the database set | +4 of the column descriptor (in every fixture equal to header csB) | confirmed (ids 873 and 171) |
| NVARCHAR2 values (type 1 with +6 = 2) | AL16UTF16, big-endian | +4 of the column descriptor (2000) | confirmed |
| Text items: `CREATE TABLE`, comments, CHECK conditions, view text, `TABLE "..."`, `INSERT INTO` and every other line or field of the text region | the client set of the export | header bytes 1-2 (= csA = block 2 +0x3c) | confirmed (ids 1, 171, 178, 873) |
| Identifiers (table and column names) | the client set: they are part of the text items. The column names a reader takes from the `INSERT INTO` line are therefore in the client set, possibly with replacement characters | header bytes 1-2 | confirmed |
| Never | header bytes 1-2 for row data; the descriptor id for text items | - | confirmed (`ru_exp_al32utf8_11g.dmp`, `own_exp_w1252_18c.dmp`) |

If the client set lacks characters of the database set (US7ASCII client, CL8MSWIN1251 database:
the case of the project's real dumps), names and comments in the file are damaged for good; the
row data are not.

### 14.9 Statements of the owner's survey that the fixtures did not bear out

The survey (session prompt, Part F) was a pointer. Checked statement by statement:

| survey statement | result | evidence |
|---|---|---|
| F1 "no value is valid UTF-8, all decode as cp1251" | **not borne out as written**: 8 of the 912 non-NULL VARCHAR2 / CHAR values of R_TEXT are pure ASCII and therefore valid UTF-8 (rows 1 and 6); the other 904 are not valid UTF-8. "All decode as cp1251" holds and is confirmed against the truth | `tools/rowdata_cmp.py`: "ascii 8, valid UTF-8 non-ASCII 0, not UTF-8 904" in each of the 15 files |
| F1 "`ru_tbl_exp_*`: rows from 0xb10; direct from 0x2310" | borne out for the 8 `ru_tbl_*` files. In `ru_tables_exp_us7ascii_11g.dmp` the rows of the same table start at 0x0b0f (its `CREATE TABLE` text is one byte shorter: `INITIAL 65536` against `INITIAL 131072`) | `ru_tbl_exp_cp1251_11g.dmp`@0x0b10; `ru_tables_exp_us7ascii_11g.dmp`@0x0b0f |
| F2 "0x0001 and 0x0030 hold the client set; 0x0032 and 0x0034 are 171 and 2000" | values borne out; the **offsets are not**: in these files the three fields stand at 0x0033 / 0x0035 / 0x0037 (`ru_tbl_exp_*`), 0x0032 / 0x0034 / 0x0036 (`ru_exp_*`), 0x0035 / 0x0037 / 0x0039 and 0x0034 / 0x0036 / 0x0038 (direct). They follow the text lines | 14.4 |
| F3 descriptors (171, 1; 2000 and 80 for NVARCHAR2(40); 20 for `VARCHAR2(20 CHAR)`) | borne out | 14.4 |
| F4 "with US7ASCII and WE8MSWIN1252 clients the name became `???_???`" | borne out for US7ASCII. With WE8MSWIN1252 the name in the dump **and in the log** is `bf bf bf 5f bf bf bf` (`¿¿¿_¿¿¿`), not question marks | `ru_exp_w1252_11g.dmp`@0x135ef; `ru_exp_w1252_11g.log` line with "exporting table" |
| F4 "the second record of an owner-mode file is different from table mode (a survey looking for `METRICST` at R + 0x104 failed)" | **not borne out**: the second record (blocks, string list) is the same and the first item stands at R + 0x104 in all owner-mode files. What differs is the text of that item: `METRICSU` | `ru_exp_cp1251_11g.dmp`@0x0904 `METRICSU`; `own`@0x2104 |
| F5 conventional 2048, direct 8192, files multiples of it | borne out | 14.7 |
| F6 "`IS` in the two cp1251-client files and `IR` in the other `ru_tbl_*` files"; hypothesis R = questionable statistics | borne out, in all 29 files of the three folders that have such lines; an equivalent condition (client set differs from database set) fits equally | 14.5 |
| F6 "the real corporate dump has `TR`" | cannot be checked: no fixture has a `T` line | 14.5, R-016 |
| F7 "a walker met type code 112 at 0x11adc" | borne out (`70 00` @0x11adc) | 14.6 |
| Prompt, Part A: "the first rows start at 0x2249 / 0x2251" (R-013) | borne out, both versions, both paths | `t_bn1`@0x2249; `t_bn3`@0x2251 |

### 14.10 Text in statistics; comments written with a US7ASCII client (session 5)

Fixtures: `ru_stat_exp_*` (database CL8MSWIN1251; clients CL8MSWIN1251, US7ASCII, AL32UTF8).
`ru_us7` = `ru_stat_exp_us7ascii_est_11g.dmp`, `ru_cp` = `ru_stat_exp_cp1251_est_11g.dmp`, `ru_utf`
= `ru_stat_exp_al32utf8_est_11g.dmp`. Column `R_STAT.TXT` holds Cyrillic words and has a
height-balanced histogram; `truth_stat.txt`: `COL|R_STAT|TXT|150|117|20|...|C0C1C2|F1F2F3F4F5F6F7F8`.

| statement | evidence | status |
|---|---|---|
| The low and high value of a text column are written as **hexadecimal text of the database bytes**: `SREC.MINVAL := 'C0C1C2'; SREC.MAXVAL := 'F1F2F3F4F5F6F7F8'` (the ASCII characters `43 30 43 31 43 32`), not as the characters themselves, not as `?`, not as hexadecimal of UTF-8 | `ru_us7`@0x19424 (first piece of the field @0x19422): `... 4d 49 4e 56 41 4c 20 3a 3d 20 27 43 30 43 31 43 32 27 ...` @0x19453-0x19464 | confirmed (truth row above) |
| The answer does **not** depend on the client set: the whole field of `TXT` (1189 payload bytes) is byte-identical with the US7ASCII, CL8MSWIN1251 and AL32UTF8 clients | `ru_us7`@0x19422-0x198d7 = `ru_cp`@0x19422-0x198d7 = `ru_utf`@0x19422-0x198d7 (`cmp`); all 20 statistics fields of the file are identical in the three files, only the letter in the lines differs (`tools/stat_cmp.py`: the differing items are the 22 lines, the CHECK condition and the two comments) | confirmed (3 client sets) |
| The same for every text column of R_TEXT: `V` (`'4C6174696E206F6E6C79'` to `'FFC6CD...D8'`, 32 bytes), `C` (CHAR, with the padding spaces `...202020`), `VC`, `LV`; NVARCHAR2 column `N` in UTF-16BE (`'0070006C00610069006E'`) | `ru_us7`@0x2b139, 0x2b33c, 0x2b518, 0x2b8d7, 0x2b6ef; all equal the `COL\|R_TEXT\|...` rows | confirmed |
| The histogram endpoints of a text column are **numbers** (the dictionary's `endpoint_value`), written as decimal digits; no character of the column occurs | `ru_us7`@0x194d4: 21 numbers of 37 digits = the 21 `HIST\|R_STAT\|TXT\|` rows | confirmed |
| Every byte of every statistics field of the 17 files with `T` items is below 0x80 | `tools/stat_items.py` (`non_ascii_payloads`: only the 3 `I` items of session 4 whose index name is Cyrillic, 14.5) | confirmed |
| **Which of the three sets of session 4 (database, client, none)?** The values: **none of the conversions applies**, because they are hexadecimal digits of the database bytes; a reader that wants them decodes the hexadecimal and then treats the bytes as row data (database set, rule C1; NVARCHAR2 as rule C3). The field as a whole is a text item (client set, rule C2); with ASCII names it is pure ASCII, so the client set changes nothing in it | above | confirmed for the values. Names: the index name inside an `I` item is in the client set (14.5: `ru_exp_al32utf8_11g.dmp`@0x1388e); a table or column name with non-ASCII letters inside a `T` item is **not in the fixtures** (the table with the Cyrillic name has no statistics in any dump): hypothesis, client set |
| Text that could be written as characters: the list of `SREC.CHVALS := DBMS_STATS.CHARARRAY(...)` | empty in all 8 fields that have it (18c, 4.6); `NULL` in the 230 others. `truth_stat.txt` of `11g-cp1251/` has no endpoint actual value (`HIST\|R_STAT\|TXT\|...\|NULL`) | unknown: no fixture shows a list with elements (R-021) |

**Comments.** The R-016 files repeat the result of 14.2 for comments, now next to statistics:

| client | comment on R_TEXT (truth `d2,e5,ea,f1,f2,fb`) | comment on R_TEXT.V (truth: 17 bytes, `Значение («тест»)`) |
|---|---|---|
| CL8MSWIN1251 | `ru_cp`@0x2bc2d `08 00 27 d2 e5 ea f1 f2 fb 27` | `ru_cp`@0x2bc5b `13 00 27 c7 ed e0 f7 e5 ed e8 e5 20 28 ab f2 e5 f1 f2 bb 29 27` |
| US7ASCII | `ru_us7`@0x2bc2d `08 00 27 3f 3f 3f 3f 3f 3f 27` | `ru_us7`@0x2bc5b `13 00 27 3f 3f 3f 3f 3f 3f 3f 3f 20 28 3c 3f 3f 3f 3f 3e 29 27` |
| AL32UTF8 | `ru_utf`@0x2bc30 `0e 00 27 d0 a2 d0 b5 d0 ba d1 81 d1 82 d1 8b 27` | `ru_utf`@0x2bc64 `21 00 27 d0 97 ...` (33 bytes) |

Status: confirmed (3 more files per client; `tools/textitems_cs.py`: 2 of 2 comments equal the
converted truth in each of the 10 `ru_stat_exp_*` files).

**The real dump (Part D2 of the session prompt; counts only, not evidence).** Reported: 1949 of
2348 bytes of the column comments and 253 of 293 bytes of the table comments are `?`, and no byte
is 0x80 or more; client US7ASCII, database CL8MSWIN1251. This is what rule C2 predicts: with a
US7ASCII client every character outside ASCII is written as one `?` (0x3f), the guillemets as `<`
and `>`, and no byte of 0x80 or more can occur in a text item; the bytes that are not `?` are the
ASCII characters of the comments (spaces, digits, punctuation, Latin letters) and the quotes of
the literals. In the fixture comment above, 12 of the 19 field bytes are `?`. The prediction
cannot say how many `?` to expect, only that Cyrillic comments are lost.

**What a reader returns for a comment.**

1. The field after `COMMENT ON TABLE "T" IS ` or `COMMENT ON COLUMN "T"."C" IS ` is an SQL
   literal: remove the first and the last `'`, replace every `''` by `'` (4.5: `own`@0x2d1d). The
   rest is the comment, with LF and other control bytes as they are.
2. Decode it with the **client** set (header bytes 1-2, rule C2), never with the database set.
3. If the client set differs from the database set (header csB): mark the comment as "converted
   at export, possibly damaged". If the client set is US7ASCII, or a `?` / 0xbf stands in the
   text, the original characters **cannot be recovered from the file**: return the text as it is
   (the `?` included) with that mark; do not guess, and do not return an empty comment. A `?`
   that was in the original comment cannot be told from a replaced character.
4. For the real dump this means: the comments come out as strings that are mostly `?`, with the
   ASCII parts intact, each marked as damaged. The comments are not needed for the table data.

### 14.11 Oracle's byte-to-Unicode map of three single-byte sets (session 7, R-026b)

Truth only, no dump: `truth_csmap.txt` (768 lines, identical in `18c/` and `11g/`, prefix
`2a7e7a0c4614`) gives, for US7ASCII, WE8MSWIN1252 and CL8MSWIN1251 and every byte 0x00 to 0xFF,
`DUMP(UTL_I18N.RAW_TO_NCHAR(byte, set), 16)`: the AL16UTF16 code unit Oracle makes of the byte.
Every line has `Typ=1 Len=2` (one code unit; no byte is dropped, none gives two units).
`tools/csmap_cmp.py` compares it with Python's codecs.

| set | compared with | bytes equal | bytes that differ (Oracle / Python) | status |
|---|---|---|---|---|
| CL8MSWIN1251 | Python `cp1251` | 255 of 256 | 0x98: Oracle U+0098, Python undefined | confirmed (truth, both databases) |
| WE8MSWIN1252 | Python `cp1252` | 251 of 256 | 0x81, 0x8D, 0x8F, 0x90, 0x9D: Oracle U+0081, U+008D, U+008F, U+0090, U+009D (the C1 control with the same number), Python undefined | confirmed |
| both | "Python, and the C1 control U+0080 + (byte - 0x80) where Python has no character" | 256 of 256 each | none | confirmed for this comparison. That this is exactly the WHATWG `windows-1251` / `windows-1252` index (what `encoding_rs` implements) is **assumed**, not tested here: the tool does not read the WHATWG tables |
| US7ASCII | the byte itself | 0x00-0x7F: 128 of 128 | 0x80-0xFF: all 128 give U+FFFD (`ff,fd`), not `?`, not the byte with bit 7 cleared, not a Latin-1 character | confirmed |

Limits. `UTL_I18N.RAW_TO_NCHAR` is a conversion inside the database (from the named set to the
national set); that `exp` row data, decoded by a reader, should give the same characters is the
assumption that makes this table useful. Bytes 0x00 to 0x1F and 0x7F map to U+0000 to U+001F and
U+007F in all three sets (part of the 255 / 251 / 128 equal bytes). The map is the same on 18c and
11g (one truth file, the same bytes; whether the 11g script really ran its own query cannot be
seen in the file).

**Rule for a reader (C7).** For CL8MSWIN1251 (171) and WE8MSWIN1252 (178), a table equal to the
codec with the holes filled by C1 controls reproduces Oracle's map for all 256 bytes: no byte needs
an error or a replacement. A decoder that treats 0x98 (1251) or 0x81, 0x8D, 0x8F, 0x90, 0x9D (1252)
as an error (Python's strict codecs, for example) differs from Oracle in exactly those bytes. For
US7ASCII (1), Oracle itself turns every byte above 0x7F into U+FFFD; a reader that does the same
agrees with Oracle, and one that raises an error is stricter than Oracle. Row data in a US7ASCII
**database** are not in the fixtures (every US7ASCII fixture is a client set), so how often such
bytes occur there is unknown.
