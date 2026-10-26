# Review 01: golden files and the truth-file tests (T-3b to T-3d, T-10)

Reviewer session 1, 2026-10-17. Master at `7211137` (Implementer T-3d). Prompt: `docs/prompts/review-01-prompt.md`
(the human's `review-01-golden.md`). All experiments ran on a copy of the repository under the session scratchpad
(`.../scratchpad/repo`, `CARGO_TARGET_DIR=.../scratchpad/target`); nothing in the repository was edited except this
file. Baseline in the copy: `cargo nextest run` 259 passed, 0 skipped, 0 ignored; `cargo fmt --check` clean;
`cargo clippy --all-targets -- -D warnings` clean.

## Summary

- **Blocking: none.** No golden or expected file comes from the reader: all six regenerate byte-identically from
  the Analyst's `exp_parse.py` / `intro_list.py` (section 1).
- **Should fix: 6** (S-1 to S-6). Most of them concern the truth-file test and the delegated families of T-10: the
  test accepts a truth file as "read" on loose evidence, and several delegated tables are never compared with a
  truth file through the reader.
- Corpus totals (201 / 386 / 400 / 41,212 / 243,007 / 19,900 / 126 / 86,080 / 1,907) re-derived with the tools:
  all agree with `common/mod.rs`, the plan and N-24. Two documents are stale (section 3).

## 1. Golden and expected files: generator, input, provenance

Each generator was re-run in the scratch copy (`cd docs/formats/tools && python3 -I <generator>`) and its output
compared with `cmp` against the committed file.

| file | lines (data) | generator | input | reproduces |
|---|---|---|---|---|
| `tests/golden/exp_raw.tsv` | 400 | `tests/golden/gen_exp_raw.py` | `exp_parse.parse()` over the 201 `*_exp_*.dmp` | byte-identical |
| `tests/golden/exp_catalog.tsv` | 386 | `tests/golden/gen_exp_catalog.py` | `exp_parse.parse()` (items, sections, parts) | byte-identical |
| `crates/oradump-core/tests/data/walk_expected.tsv` | 201 | `gen_walk_expected.py` | `exp_parse.parse()`, `intro_list.ENTRIES/matches` | byte-identical |
| `.../tests/data/walk_parts.tsv` | 400 | `gen_walk_expected.py` | same | byte-identical |
| `.../tests/data/lob_expected.tsv` | 21 | `gen_walk_expected.py` | same (`t["lob_values"]`) | byte-identical |
| `.../tests/data/value_parts.tsv` | 400 | `gen_number_parts.py` | `exp_parse.parse()` | byte-identical |

None of the generators imports or runs anything from `crates/` or `target/` (checked by reading the imports). The
generators print and assert the 201-file totals (`gen_walk_expected.py` printed `files 201, sections 386, parts
400, rows 41212, fields 243007, nulls 19900, lob_values 126 (80 short, 12 empty, 46 long), lob_sections 21`;
`gen_number_parts.py` `201 files, 400 parts, 86080 NUMBER fields`). Verdict: provenance is the Analyst's tools,
not the reader. Not blocking.

Other expected tables in the tests:
- `t5_types.rs` per-form table (BLOB 12 ... VARCHARn(n) 998, 1,907 definitions): equals the `collist.py` printout
  I re-ran (all 28 forms identical).
- `t10_truth.rs` `EXPECTED_CHECKED` / `EXPECTED_DELEGATED` (48 keys, 386 sections): the module comment says they
  come "from the exploration of this task (a listing of the catalogs)", i.e. from the reader. I recomputed them from
  `tests/golden/exp_catalog.tsv` (tool output) with the same keying (T_STCn, T_STATn, damaged names): 0 differences,
  386 = 386. The values are right; the provenance is weak (note N-2).
- `EXPECTED_WARNINGS` is an inventory (observed); each entry has a reason in a rule or the plan (section 4).

## 2. `t10_truth.rs` and the truth-file test

### 2.1 Which truth files have a real checker

43 distinct names, 82 files (18c 41, 11g 38, 11g-cp1251 3).

Compared value by value with the truth file, through the reader:
`truth_frame`, `truth_nullmix`, `truth_num`, `truth_num2`, `truth_num3`, `truth_numtypes`, `truth_date`, `truth_ts`,
`truth_tstz`, `truth_text`, `truth_raw`, `truth_wide`, `truth_probe6`, `truth_ddl`, `truth_own`, `truth_stat`
(18c/11g `ROW` lines), `truth_nchar`, `truth_nchar2`, `truth_probe11`, `truth_part` (`P|`, `TOTAL`), `truth_counts`
(row counts), and through delegated tests `truth_misc` (`t6_read.rs::intervals_and_rowids_equal_truth_misc`),
`truth_part2` (`t6_parts.rs::t_part2_gives_100_rows_...`), `truth_ru` (R_CLOB, R_TEXT in `t6l_ru.rs`, `t9_charset.rs`),
`truth_lob_ru` (`r_lob2_decodes_...`), `truth_lob` / `truth_lob3` (`t6l_lob.rs`; not reviewed in detail).

Compared only in part:
- `truth_bn1..4`: length and first bytes from the truth, the rest from the RPAD of `06_bn.sql`: exact. Good.
- `truth_len`: prefix of 12 bytes, length, and a character class; **not exact** (S-5).
- `truth_stc`: opened, the `TAB` count is **read but not used** (S-3).

"Named somewhere" only (the test counts them as read, no test compares the reader's values with them):
- `truth_part2.txt` is credited through `t6_parts.rs` because that file is in `DELEGATED` for T_PART; the T_PART2
  entry points to `t8_direct.rs::partitioned_pairs_give_equal_batches_per_part`, a pair-equality test that opens no
  truth file (S-1 b).
- `11g-cp1251/truth_stat.txt` is credited by the name `truth_stat.txt` of the 18c / 11g checkers; no test opens the
  cp1251 file (S-1 c).

Listed without a checker (`TRUTH_WITHOUT_CHECKER`, `t10_truth.rs:1373`), reason checked:

| file | stated reason | true? |
|---|---|---|
| `truth_float.txt` | T_FLOAT only in a Data Pump file | yes: no T_FLOAT in `exp_catalog.tsv`; only `t_float_dp_<v>.log` names it |
| `truth_csmap.txt` | T-9 map test | yes: `t9_charset.rs::map_test_768_lines_of_truth_csmap_in_both_folders` opens it |
| `truth_charsets.txt` | T-10b | yes (planned, T-10b (3)) |
| `truth_env.txt` | NLS parameters, no table | yes (6 `key=value` lines) |
| `truth_own_ddl.txt` | objects and comments, no rows | yes (OBJECT / TABCOMMENT / COLCOMMENT / CONSTRAINT) |
| `truth_stc_extra.txt` | statistics items, no row values | yes; it also has `TAB` counts (50) that nothing uses |
| `truth_decl.txt` | declared types, `t5_types.rs` | yes: `t5_types.rs` opens it |
| `truth_r045.txt` | SQL*Plus spool, no EXP table | yes (ORA-14085 spool) |
| `truth_long2.txt` | "T_LONG2 is compared with 11_types.sql; the file is cited in comments only" | **partly false** (S-2) |
| `truth_misc2.txt` | opened by `t4_values.rs`, not by the registry's test | true, but it hides a gap (S-2) |

### 2.2 How the truth-file test decides "read"

`every_truth_file_is_read_or_listed` (`t10_truth.rs:1398`) collects the names `truth_*.txt` that appear as literals
in non-comment lines of `t10_truth.rs` (up to the list) and of every whole file named in `DELEGATED`, and compares
them with the names in the three `SHA256SUMS`. Three looseness points, see S-1.

## 3. Counts

Re-derived in the scratch copy (read-only, `python3 -I`):
- `corpus_counts.py` (three folders): TOTAL files 201, sections 386, rows 41,212, row_fields 243,007, null_fields
  19,900, text_fields 783 (344 plain, 427 pieces, 12 upieces), empty_conventional 24, empty_direct 17, problems 0;
  per folder 89/85/27 files, 170/164/52 sections, 9,525/3,311/28,376 rows, 31,585/12,943/198,479 fields,
  2,256/2,240/15,404 NULL, 263/223/297 text fields, codes 112: 6/6/9, 113: 5/5/2.
- `rowlen_stats.py`: parts 400, LOB values 126 (80 short, 46 long, 12 empty), lob_sections 21, findings 0.
- `num_decode.py`: 86,080 NUMBER fields decoded and re-encoded, PROBLEMS 0.
- `collist.py`: 386 `CREATE TABLE` lines, 1,907 definitions, PROBLEMS 0.
- `check_all.py`: problems 0 on every file (per-file lines inspected for the misc / long / lob files).

| quantity | `common/mod.rs` | plan (top table) | N-24 | rules v3.5 | tools now |
|---|---|---|---|---|---|
| files | 201 (89/85/27) | 201 | 201 | 201 | 201 |
| sections | 386 (170/164/52) | 386 | 386 | 386 | 386 |
| parts | 400 (184/164/52) | 400 | 400 | 400 | 400 |
| rows | 41,212 | 41,212 | 41,212 | 41,212 | 41,212 |
| fields / NULL | 243,007 / 19,900 | same | same | same | same |
| LOB values | 126 = 80 + 46 | 126 | 126 | 126 (L4) | 126 |
| NUMBER fields | 86,080 | 86,080 | 86,080 | 86,080 (Y1) | 86,080 |
| column definitions | 1,907 (literal in `t5_types.rs:171`) | "386 lines" | 1,907 | **1885 / 382 lines (T9)** | 1,907 |

Numbers that disagree:
- `docs/formats/exp_reader_rules.md` row T9 (line 226): "382 lines, 1885 column definitions, `tools/collist.py`" -
  `collist.py` now gives 386 / 1,907. The rules file's own "Changes in session 10" row (line 16) does not mention
  collist. For the Analyst.
- `docs/formats/exp_reader_rules.md` X2 (line 251): evidence `ru_exp_us7ascii_11g`, `ru_exp_w1252_11g` and "log:
  `EXP-00056`" only; `exp.md` 14.3 and `ru_exp_direct_us7ascii_11g.log` add the third file with `EXP-00008` /
  `ORA-00604` / `ORA-00942`. For the Analyst.
- `crates/oradump-core/tests/data/gen_walk_expected.py:183-185`: the comment above the asserts still says "Architect
  session 4, 2026-10-15 ... [L4] counts 74 short and 40 long values in 19 sections"; the asserts say 80 / 46 / 126.
- `docs/design/implementer-notes.md` N-23 (line 836): "all 256 tests but the 11 of T-3d pass". At `a937f2f` the
  tree has 219 integration + 38 unit `#[test]` = 257 (255 at `0f0e4f7`, 259 at `7211137`).
- `common/mod.rs:43` `FolderTotals.lob_sections` is documented "sections with a column of type 112 or 113
  (`codes_outside_T5`)"; the value is the 112 entry (6/6/9). The two counts coincide in this corpus (every section
  with 113 also has 112; `gen_walk_expected.py` counts 21 LOB sections), so no number is wrong; the doc names the
  wrong source key.

## 4. The pinned `W-POSSIBLY-INCOMPLETE` entries (T-3d)

`EXPECTED_WARNINGS` (`t10_truth.rs:1105`) has `("???_???", W-POSSIBLY-INCOMPLETE, 2)` and
`("\u{bf}\u{bf}\u{bf}_\u{bf}\u{bf}\u{bf}", ..., 1)`. I checked the three files:

| file | log after the table line | catalog (`exp_catalog.tsv`) |
|---|---|---|
| `ru_exp_us7ascii_11g` | `EXP-00056: ORACLE error 942`, `ORA-00942` | `???_???`, 1 part, 0 rows, `ffff` |
| `ru_exp_direct_us7ascii_11g` | `EXP-00008: ORACLE error 604`, `ORA-00604`, `ORA-00942` | `???_???`, 0 rows |
| `ru_exp_w1252_11g` | `EXP-00056`, `ORA-00942` | `\xbf\xbf\xbf_\xbf\xbf\xbf`, 0 rows |

The same table exported 2 rows with the CL8MSWIN1251 and AL32UTF8 clients (`ru_exp_cp1251_11g.log`, catalog 2
rows). `exp.md` 14.3 describes exactly these three as cut short, and arch 6.5 / N-21 define the flag as "0 rows and
a damaged name". So the three entries are correct and are cut-short exports by the logs and the rules. Minor: the
comment at `t10_truth.rs:1107` names only `EXP-00056`; the direct file's code is `EXP-00008`. The plan (T-3d (b))
said "the warning inventory unchanged"; the change is the Implementer's (N-24) and is justified, but no Architect
answer records it yet.

## 5. Mutation tests (scratch copy)

Each mutation was applied alone, the full suite run (`cargo nextest run --no-fail-fast`), the file restored and
compared with `cmp` against the original.

| id | mutation | failed tests | did not fail but should have |
|---|---|---|---|
| a1 | `18c/t_wide_exp_18c.dmp` @0x3804: `v115` -> `v116` (a T_WIDE value byte) | `t10_truth::every_table_of_the_exp_files_is_classified_and_read`, `t2_manifest::x_fixture_checksums_...`, `t2_manifest::x_walker_with_hashing_...`, `t8_direct::every_direct_file_equals_its_conventional_partner` | `t3_catalog::*` (by design: the catalog does not look at values) |
| a2 | `11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp` @0x227b5: `da` -> `c2` (a Cyrillic byte inside R_TEXT) | `t2_manifest` (both), `t8_direct::every_direct_file_...` | **`t10_truth::every_table_...`** (R_TEXT "delegated"), every `t6l_ru` / `t9_charset` test |
| a3 | `11g/misc2_exp_11g.dmp` @0x24cf: `33` -> `34` (T_ROWID2 row 1) | `t2_manifest` (both) | **`t10_truth::every_table_...`**, `t6_read::misc2_intervals_and_the_logical_urowid` (18c only) |
| a4 | `18c/multi_framing_exp_18c.dmp` @0x24ad: one `x` of T_LEN `L126_xxx...` -> `7` | `t2_manifest` (both), `t8_direct::every_direct_file_...` | **`t10_truth::every_table_...`** (`check_len` accepts digits anywhere) |
| b1 | `18c/truth_wide.txt`: `v115` -> `v116` | `t10_truth::every_table_...`, `t2_manifest::x_fixture_checksums_...` | - |
| b2 | `11g/truth_misc2.txt`: `ROWID|2|...AAB` -> `...AAX` | `t4_values::e_rowids`, checksum guard | no reader-level test |
| b3 | `18c/truth_misc2.txt`: `*BAMAFbMCwQL+` -> `...L-` | `t4_values::e_rowids`, checksum guard | `t6_read::misc2_intervals_...` (literal, does not open the file) |
| b4 | `18c/truth_long2.txt`: `...C3A9` -> `...C3A8` | checksum guard only | **`t6_read::t_long2_decodes_...`** (literal copy of the truth) |
| b5 | `11g-cp1251/truth_stat.txt`: `TAB|R_STAT|2000` -> `2001` | checksum guard only | any R_STAT test; `every_truth_file_is_read_or_listed` |
| b6 | `18c/truth_stc.txt`: `TAB|T_STC1|50` -> `51` | checksum guard only | **`t7_owner::stc_family_reads_through_the_same_code`** (reads the line, asserts a literal 50) |
| c | `reader.rs` `lob_piece`: for `Kind::LobText` with `Charset::Win1251`, drop every byte >= 0x80 before `PieceDecoder::feed` | `t6l_ru::r_clob_in_all_seven_files`, `t6l_ru::r_lob2_decodes_to_the_truth_in_both_client_files` | `t10_truth::every_table_...` (delegated: only the row count); `t6l_ru::a_long_form_clob_of_database_set_171_decodes` (compares long with short form, both equally wrong) |
| d | `dump.rs` `Dump::rows`: `if parts != 1` -> `if parts != 1 && false` (first part only again) | `t3_catalog::x_a_reader_at_rows_offset_reads_exactly_num_rows_...`, `t5f_followup::rows_on_a_table_of_several_parts_is_e_config_...` | - |
| d2 | `reader.rs` `TableBatches::new`: `.take(1)` on the part plans (`read_table` reads only the first part) | `t10_truth::every_table_...`, `t10_truth::t_part_matches_the_p_lines_...`, `t6_parts` (3 tests), `t6l_ru::a_partitioned_lob_table_reads_both_parts`, `t8_direct::partitioned_pairs_...` | - |
| e | `t10_truth.rs`: `check_wide` compares nothing (`let _ = (got, want);`) and `check_stat` compares only the row count | none (259 passed) | `every_truth_file_is_read_or_listed` (a literal name is enough); there is no break test per checker (T-10b (4)) |

The checksum guard `t2_manifest::x_fixture_checksums_match_the_three_sha256sums_files` fails on every fixture
mutation, as it should; the column "did not fail" names the tests whose job it is to check the value.

## 6. Notes N-19 to N-24: deviations and their status

| note | deviation / choice | justified? | decided by |
|---|---|---|---|
| N-19 | `lob_start` gets `form`; two diagnostic counters; refusal order; which values are unconfirmed | yes | Architect, ADR-34 (session 7) |
| N-20 | test table `t_num3` instead of `t_num`; `NUMBER(*,s)` = Declared; `W-TSLTZ`; `part_rows` `E-CONFIG` | yes (the plan's table cannot show the difference) | Architect, ADR-35 |
| N-21 | `own_exp_w1252: ddl_lossy` cannot hold with arch 6.5; flag definition; `possibly_incomplete`; overrides; LONG with an unsupported header set stays `E-CONFIG` | yes for `ddl_lossy` (the `?` is in a COMMENT item, checked by the Implementer, not by me); the rest is a design choice | **only one point decided** (ADR-25 session 8 item 2: the override does not change the LOB decision). The `ddl_lossy` plan text (plan T-9 line 590) still stands; the LONG proposal has no answer |
| N-22 | T-10 built before T-9; 29 checkers + 18 delegated families; partial truth texts; warning inventory; T_NUM stops by default | the delegation is where S-1 to S-4 sit; the rest yes | partly: the Architect answered only "leftovers are T-10b"; the inventory and the partial-truth handling are not explicitly accepted |
| N-23 | static messages per database set (no `Cow`); test name kept; behaviour change for headers outside 873/171 | yes (ADR-34 item 9 forbids an API change; the behaviour is that of plan T-6M (a)) | not answered yet |
| N-24 | generator asserts edited by hand; `W-POSSIBLY-INCOMPLETE` added to the inventory; `t4_values.rs` fixed outside the list; loose literal match; `truth_long2` / `truth_misc2` moved to the explicit list | the hand edits are acceptable (the outputs reproduce from the tools, section 1); the inventory entries are right (section 4); the list additions are a workaround for the loose match (S-1, S-2) | not answered yet |

## Findings

### Blocking

None.

### Should fix

**S-1 `every_truth_file_is_read_or_listed` accepts a truth file on loose evidence** (`t10_truth.rs:1398-1464`).
(a) A literal name in a non-comment line counts as "read": mutation e made two checkers compare nothing and every
test still passed. (b) A delegated *file* is scanned, not the delegated *function*: `truth_part2.txt` is credited
through `t6_parts.rs` (T_PART's entry), while T_PART2's entry (`t10_truth.rs:957-961`) points to
`t8_direct.rs::partitioned_pairs_give_equal_batches_per_part`, which opens no truth file; the real T_PART2 check is
`t6_parts.rs::t_part2_gives_100_rows_and_each_part_holds_its_own_rows`. (c) Names are collapsed across folders:
`11g-cp1251/truth_stat.txt` is never opened by any test (mutation b5), but `truth_stat.txt` is "read" through the
18c/11g checker. Fix: record the paths actually opened (e.g. `common::truth` adds `(folder, name)` to a static set
that the test reads after running the checkers, or a registry `(folder, file, checker)`); key on `(folder, name)`;
point T_PART2 to the `t6_parts.rs` test; list `11g-cp1251/truth_stat.txt` with a reason or check its
`TAB|R_STAT|2000` line.

**S-2 Two explicit-list entries hide unchecked values** (`t10_truth.rs:1385-1392`).
`truth_long2.txt`: the reason says T_LONG2 is compared with `11_types.sql`; T_LONG2 is built by `14_long2.sql`,
and `t6_read.rs:403-407` compares with the literal `D0B4D0B620436166C3A9` copied from `truth_long2.txt` line 2
(`1|10|...`); a changed truth file goes unnoticed (b4). Open the file and compare length and hex.
`truth_misc2.txt`: the reader-level test `t6_read.rs::misc2_intervals_and_the_logical_urowid` (`:591`) runs on 18c
only, uses literals, checks T_ROWID2 row 10 only; T_ROWID2 rows 1 to 3 and all of `misc2_exp_11g` T_ROWID2 / the
IDS columns of T_INTERVAL2 are never compared through the reader (a3: a changed ROWID byte fails only the manifest
tests). Add a T_INTERVAL2 / T_ROWID2 checker to `t10_truth.rs` that opens `truth_misc2.txt` of each folder.

**S-3 The T_STC delegated test reads the truth count and ignores it** (`t7_owner.rs:116-135`).
`counts` is read from `truth_stc.txt` but the assertion is `assert_eq!(row_count(&base), 50, ...)`; `n` is used only
to skip `NULL`. Mutation b6 (`50` -> `51`) passes. T_STC values are only compared across the four files of the
family, never with `10_stc.sql`. Fix: `assert_eq!(row_count(&base).to_string(), *n)`, and compare the values with
the SQL rule; T_STC4 (count `NULL` in `truth_stc.txt`, 50 in `truth_stc_extra.txt`) needs a stated expectation.

**S-4 Delegated families with most sections unchecked against a truth** (`t10_truth.rs:962-976`, `:1080-1100`).
R_TEXT: 27 sections; the named test `r_text_and_the_cyrillic_table_around_r_clob` reads 7 files, `r_lob2_...` 2 more
(columns V, C, VC only), `t9_charset.rs::r_text_equals_truth_for_every_client_set_and_path` 8 more (not in the
registry); the 10 R_TEXT sections of the `ru_stat_*` files are compared with nothing through the reader (a2).
R_STAT: 10 sections, one (`ru_stat_exp_cp1251_est_11g`) hash-compared with the manifest, the rest row counts only.
T_LONG_EMPTY is delegated to a pair test (its "truth" is 0 rows, acceptable but say so). For a delegated table,
`t10_truth.rs:1165-1166` only checks `r.len() == num_rows`, where `num_rows` is the reader's own catalog. T-10b (1)
covers R_TEXT; extend it to R_STAT (manifest hash per file is enough: the hash comes from `exp_parse.py`).

**S-5 `check_len` is not exact** (`t10_truth.rs:271-299`). Beyond the 12 DUMP bytes it only checks the character
class (`digit`, `_`, `x`); a digit in the padding passes (a4). `01_corpus_tables.sql:69` gives the exact value
`RPAD(SUBSTR('L' || n || '_', 1, n), n, 'x')`; compare with that string.

**S-6 Expected instance counts are documented as coming from the reader** (`t10_truth.rs:12-14`, `:1046`).
`EXPECTED_CHECKED` / `EXPECTED_DELEGATED` are said to come from "a listing of the catalogs" made in T-10, i.e. the
reader's scan. They agree with `exp_catalog.tsv` (0 differences, checked here), so no value is wrong, but a later
edit has no independent source. Derive them in the test from `tests/golden/exp_catalog.tsv` (keyed like
`delegated()`), as T-3d (b) asks for the other counts.

### Notes

- N-1 `gen_walk_expected.py:183-185`: stale comment (74 / 40 / 19 sections, Architect session 4); the asserts are
  right.
- N-2 `t5_types.rs:171` keeps the Analyst total 1,907 as a literal; T-3d (b) wants the Analyst's totals once in
  `tests/common/` (add `column_definitions` to `TOTALS`).
- N-3 `common/mod.rs:43`: the `lob_sections` doc names "112 or 113" but the source key is the 112 count (equal here).
- N-4 `t10_truth.rs:1107`: the comment names `EXP-00056`; the direct file's log has `EXP-00008` / `ORA-00604`.
- N-5 `t3_catalog` did not fail on a changed value byte (a1): expected, the catalog tests check structure; value
  bytes are covered by `t2_manifest` (walker hash against `exp_parse.py`) and the truth checks.
- N-6 The agreement `common::TOTALS` vs the TSVs (`t1_walk.rs:187`) is between two outputs of the same Python parser
  family (`corpus_counts.py` and `exp_parse.py`); the independent check is the Rust walker reproducing the TSVs
  (`t2_manifest`, `t1_walk`, `t3a_walker`), which is the design. Fine, but the TOTALS test alone proves little.
- N-7 Rules file T9 and X2 are stale (section 3): for the Analyst, not the Implementer.
- N-8 N-23's test count (256) should be 257.

## What I did not check

- `t6l_lob.rs` (T_LOB, T_LOB2, T_LOB3 against `truth_lob*.txt`), `t6l_ru.rs` beyond the three tests read, most of
  `t9_charset.rs`, `t4_values.rs`, `t5_types.rs` beyond the per-form table, `t3a_walker.rs`, `t3b_parts.rs`,
  `t3c_walker.rs`, `t6_patched.rs`, `t7_owner.rs` except the STC test: not read line by line.
- The correctness of `exp_parse.py` itself: only that `check_all.py`, `corpus_counts.py`, `rowlen_stats.py`,
  `num_decode.py`, `collist.py` report 0 problems; I did not compare the parser with the format documents.
- The header columns of `walk_expected.tsv` against `header_fields.py` output, and `data/exp_intro_lines.txt`
  against the rules block.
- N-21's claim that the `?` of `own_exp_w1252` stands only in a COMMENT item (not checked in the bytes).
- N-24's statement that 18 tests failed after regeneration (the pre-T-3d state was not rebuilt).
- `docs/design/decisions.md` was searched for N-19 to N-24 only, not read in full.
- Python (`python/`), Data Pump files, `corpus/work/` (not read, by rule).
- Mutations were single-shot; I did not try every checker or every delegated family.

## git status

Before (recorded at the start): `?? .claude/agents/reviewer.md`, `?? docs/prompts/review-01-prompt.md`.
After (end of session): the two untracked files and `docs/review/` (this file), **plus** changes this session did
not make: modified `crates/oradump-core/src/{dump.rs, error.rs, exp/intro.rs, lib.rs, reader.rs, types/map.rs,
types/resolve.rs}`, `crates/oradump-core/tests/common/mod.rs`, and a new untracked `crates/oradump-core/src/survey.rs`
(mtimes 12:02 to 12:04 local, during the session, still growing at the end). This review did not write them: every
code mutation was made in the scratch copy, and the scratch copies of the modified source files are identical to
`HEAD` (`git show HEAD:<file> | cmp`). They look like a concurrent Implementer session on master (a survey counting
module is T-11's). So the status does **not** match the start, but the only difference this session caused is
`docs/review/review-01-golden.md`. The review itself is of `HEAD` = `7211137`, not of that working tree.
