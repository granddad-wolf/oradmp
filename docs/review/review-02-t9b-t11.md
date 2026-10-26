# Review 02: T-9b, T-10b, T-11 and the test files session 1 did not read

Reviewer session 2, 2026-10-17. Prompt: `docs/prompts/review-02-prompt.md` (to be saved as
`review-02-t9b-t11.md`; untracked at the start, not touched). Reviewed: **HEAD `7941e40`** (T-10b), with
`e308d9a` (T-9b) and `7db4667` (T-11) below it. All three are committed; the working tree had no change
except the untracked prompt, so HEAD is what was reviewed.

Method. `git archive HEAD` into the session scratchpad (`.../scratchpad/repo`, plus an untouched `pristine`
copy); Cargo target directory outside the repository on disk (`<home>/.cache/oradump-review-02/target`,
because `/tmp` is a 3.9 GB tmpfs that the build filled once; removed at the end). Baseline in the copy:
`cargo fmt --check` clean, `cargo clippy --all-targets -- -D warnings` clean, `cargo nextest run`
**371 passed, 0 skipped, 0 ignored**. Every mutation was applied alone to the copy, the full suite run with
`--no-fail-fast`, and the copy restored from `pristine` (`diff -r` identical; all `.rs` files touched so that
Cargo never reuses a binary built from a mutated source: three early runs that had reused one were discarded and
repeated, section 1). The Analyst's tools were run read-only in the copy (`python3 -I -B`).

## Summary

- **Blocking: none.**
- **Should fix: 4** (S-1 to S-4). The most important: the T-11 test "no row value in any error or warning
  text" cannot fail for any message the fixtures produce (two mutations that write the failing value into the
  message pass all 371 tests, S-1). Today the guarantee holds by construction (`message: &'static str`, numbers
  only in `Display`), which I checked by reading the builders; it will rest on the test alone once T-12 makes
  `message` a `Cow`.
- Review 01's S-1 to S-6: **all closed**. The four mutations that no value test caught in session 1 (a3, b4, b5,
  b6) now fail a named check; so do a2, a4 and e.
- The gap the Implementer names (the `assert!` of a thin wrapper such as `check_wide`) is real and shown (rated
  note N-1).
- T-9b: both fixes are pinned by `t9b_fixes.rs`; the offsets the copies patch agree with the fixture bytes. The
  offset rule of ADR-41 item 6 is not held by two other reports of the same LONG chain (S-3).
- Numbers: every corpus total re-derived with the tools agrees with `common/mod.rs` and the plan; the test count is
  371 (prompt: 371; T-10b note: 370).

## 1. Review 01's findings S-1 to S-6 (T-10b)

| finding | how T-10b answers it | shown here |
|---|---|---|
| S-1 (a) a literal name counted as "read" | `common::truth_text` (`common/mod.rs:323`) is the only opener; it records `(folder, name)` in `OPENED_TRUTH`; `every_truth_file_is_read_or_listed` (`t10_truth.rs:1671`) runs `scan()`, `check_t_part()` and all 13 `FAMILIES`, then compares the recorded set plus `TRUTH_NOT_OPENED` with the 82 `truth_*.txt` lines of the three `SHA256SUMS` (disjoint, no extra, no missing) | mutation g2 (`lob3_diffs` skips 11g): only `every_truth_file_is_read_or_listed` fails, although `t6l_lob.rs` still opens `11g/truth_lob3.txt` in its own process. A grep of `tests/*.rs` finds no `read_to_string` / `fs::read` of a truth file outside `truth_text` |
| S-1 (b) T_PART2 delegated to a pair test | registry entry `t10_truth.rs:1090-1095` points to `t6_parts.rs::t_part2_gives_100_rows_and_each_part_holds_its_own_rows`, which calls `part2_diffs` | read |
| S-1 (c) names collapsed across folders | the key is `(folder, name)`; `11g-cp1251/truth_stat.txt` is opened by `r_stat_diffs` | b5 below |
| S-2 `truth_long2`, `truth_misc2` hidden in the list | both left the list; `long2_diffs` (6 files, length + hex per line, `INSERT REFUSED`), `misc2_diffs` (both folders, every row and column) | b4, a3 below |
| S-3 T_STC count read and ignored | `stc_counts` (T_STC4 from `truth_stc_extra.txt`), `stc_diff` (count, `COLUMN` statistics, values by `10_stc.sql`) in all 8 files; `t7_owner.rs:118` asserts the truth count | b6 and f1 (11g `TAB|T_STC2|50` -> `51`) below. `stc_rule` checked against `corpus/sql/10_stc.sql`: ID = LEVEL, A = `'a' \|\| MOD(LEVEL,7)`, B = `'b' \|\| LEVEL`, C = 2020-01-01 + LEVEL, ALLNULL NULL, T_STC2 ADDED_LATER NULL / ADDED_NUM = MOD(ID,3): agrees |
| S-4 R_TEXT / R_STAT | `r_text_diffs` over the 27 sections of `exp_catalog.tsv`; `r_stat_diffs` over 10 (count from `TAB`, NULL / distinct from `COL`, values by rule). `r_stat_rule` checked against `08_ru_stats_objects.sql` (GRP, word(i), `'K' \|\| LPAD(..,3,'0')` in CHAR(6), DT, `ROUND(MOD(i*37,1000)/7,2)` = floor((200m+7)/14) cents, NOTE): agrees. T_LONG_EMPTY has its stated expectation (`t10_truth.rs:1735`) | a2, b5 below |
| S-5 `check_len` not exact | `len_rule` = `RPAD(SUBSTR('L'\|\|n\|\|'_',1,n),n,'x')` (`t10_truth.rs:278`, same text as `01_corpus_tables.sql:69`) plus the DUMP bytes, character count, byte length | a4 below |
| S-6 instance counts from the reader | `expected_counts()` (`t10_truth.rs:1234`) derives both maps from `tests/golden/exp_catalog.tsv` through `delegated()` / `checker_tag()`; the literal tables are gone | read; `checker_tag`'s 28 tags + `T_STAT` equal the arms of `check_table` |

The review's mutations, repeated (each alone; the checksum guard
`t2_manifest::x_fixture_checksums_match_the_three_sha256sums_files` also fails for every fixture mutation, not
repeated below):

| id | mutation | named check that fails | other failures |
|---|---|---|---|
| a3 | `11g/misc2_exp_11g.dmp` @0x24cf `33`->`34` (T_ROWID2 row 1 `AAAE/3...` -> `/4`) | **`t10_truth::t_interval2_and_t_rowid2_equal_truth_misc2`** | `every_truth_file_is_read_or_listed`, `break_misc_and_misc2_...`, `t2_manifest::x_walker_with_hashing_...` |
| b4 | `18c/truth_long2.txt` `...C3A9` -> `...C3A8` | **`t6_read::t_long2_decodes_cyrillic_and_accents_without_a_warning`** | `every_truth_file_is_read_or_listed`, `break_t_long2_...` |
| b5 | `11g-cp1251/truth_stat.txt` `TAB|R_STAT|2000` -> `2001` | **`t10_truth::r_stat_equals_truth_stat_and_the_sql_rule`** | `every_truth_file_is_read_or_listed`, `break_r_stat_...` |
| b6 | `18c/truth_stc.txt` `TAB|T_STC1|50` -> `51` | **`t7_owner::stc_family_reads_through_the_same_code`** | `every_truth_file_is_read_or_listed` (`break_t_stc_...` uses a literal `"50"`, so it does not fail: fine, the family check does) |
| a2 | `11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp` @0x227b5 `da`->`c2` | **`t10_truth::r_text_equals_truth_ru_in_all_27_sections`** | `every_truth_file_...`, `break_r_text_...`, `t2_manifest::x_walker_...`, `t8_direct::every_direct_file_...` |
| a4 | `18c/multi_framing_exp_18c.dmp` @0x24ad `x`->`7` | **`t10_truth::every_table_of_the_exp_files_is_classified_and_read`** (T_LEN) | `every_truth_file_...`, `break_t_len_...`, `t2_manifest::x_walker_...`, `t8_direct` |
| e1 | `wide_diff` compares nothing (`if got != want && false`) | **`t10_truth::break_t_wide_and_t_stat_one_changed_value_gives_one_difference`** only | - |
| f1 | own: `11g/truth_stc.txt` `TAB|T_STC2|50` -> `51` | **`t7_owner::stc_family_...`** | `every_truth_file_...` |
| f3 | own: `11g/truth_charsets.txt` `US7ASCII=1` -> `=11` | **`t10_truth::charsets_and_the_oracle_map_equal_the_readers_tables`** | `every_truth_file_...`, `break_charsets_...` |
| g2 | own: `truth_checks::lob3_diffs` skips the 11g file | **`t10_truth::every_truth_file_is_read_or_listed`** | - |

The results agree with the T-10b note's table (b9), except that b6 does not fail `break_t_stc_...` (the note
does not claim it does).

The lists:
- `TRUTH_NOT_OPENED` (`t10_truth.rs:1576`, 9 entries): checked by reading the files. `truth_float` (18c, 11g):
  no T_FLOAT section in `exp_catalog.tsv` (also asserted by `explicit_reasons_hold`). `truth_env` (18c, 11g): 6
  `key=value` NLS lines, no table. `truth_own_ddl` (18c, 11g): OBJECT 14, CONSTRAINT 8, SOURCE 135, SOURCETOTAL 3,
  TABCOMMENT 2, COLCOMMENT 3 lines: no row values (comments and constraints are not exposed by the reader).
  `truth_r045` (18c only; 11g has none): an SQL*Plus spool. `truth_decl` (18c, 11g): `t5_types.rs::t_decl_gives_...`
  opens it in both folders and compares 14 rows. All true. 82 = 73 opened + 9 listed.
- `DELEGATED` (`t10_truth.rs:1017`, 19 entries): every function exists and calls its family check
  (`delegated_tests_exist`). The five entries without a family check have no truth file: T_LONG / T_LONGRAW
  (`11_types.sql`), T_LONG_EMPTY (0 rows), T_PART (`check_t_part` in this file, which opens `18c/truth_part.txt`;
  T_PART exists only in the two 18c files), CYRILLIC (the 6 damaged / Cyrillic-named sections; values
  `(абв,1),(NULL,2)` from `07_ru_objects.sql`, no truth line). True.
- `checker_tag` / `delegated()` keying: equal to `check_table`'s arms; the keying of `T_STCn` and damaged names
  is applied to the golden catalog and to the reader's names alike.

## 2. The remaining gap: the thin wrappers, and the duplicates

Shown on the copy:
- e3: `check_wide` (`t10_truth.rs:693-700`) with `assert!(diffs.is_empty(), ...)` replaced by `let _ = diffs;`:
  **371 passed**.
- e4: e3 plus a changed T_WIDE value in `11g/t_wide_exp_11g.dmp` (@0x3804 `v115` -> `v116`, decimal 14341):
  only `t2_manifest` (checksum guard, walker hash) and `t8_direct::every_direct_file_equals_its_conventional_partner`
  fail; no value test. The break test reads only `18c/t_wide_exp_18c.dmp`, so it still catches a reader bug that
  shows in the 18c file, but not one that shows only in another instance.

The same holds for every wrapper with a pure `*_diff` (`check_len`, `check_stat`, `check_nchar`, `check_wide`) and,
equally, for every inline `assert_eq!` of the other checkers (`check_frame`, `check_num`, ...): deleting an assertion
is an edit to the test, which no test can catch without a meta-test. Rating: **note** (N-1), because it is the
general case of "remove an assertion", the break tests made the comparison logic itself testable (e1 is caught), and
the edit would be visible in review. Cheap fix if wanted: let the checkers return their diff lists and assert the
union once in `every_table_of_the_exp_files_is_classified_and_read`, plus one test that feeds a mutated truth line
through `check_table` for each tag and expects a non-empty union.

Duplicates (delegated tests that run their old assertions and the new family check): I compared each pair.

| test | old assertions | family check | weaker one hidden? |
|---|---|---|---|
| `t6l_lob::t_lob_equals_...` | full values from `01b_optional_tables.sql`, truth lengths and B head | lengths, B head | no; the old test is stronger (SQL values) |
| `t6l_lob::t_lob2_equals_...` | full values from `13_optional.sql` | lengths, V | no |
| `t6l_lob::lob3_values_...` | full values (`x` / `5a` x S, row 20 text) | lengths, C head, `x` / `5a`, row 20 | no, equal |
| `t6_read::intervals_and_rowids_equal_truth_misc` | same comparison, own code | `misc_diffs` | no, equal |
| `t6_read::t_long2_decodes_...` | row 1 against a literal copy of `truth_long2.txt` line 1 | every line of the folder's truth | no; the literal is a duplicate of the truth (benign) |
| `t6_parts::t_part2_...` | rows by `14_part2.sql` | SP / ROW lines | no; the old test is stronger |
| `t6l_ru::r_clob_in_all_seven_files` | row 1 full text, row 2 length + 60 bytes | length + head | no, equal |
| `t6l_ru::r_lob2_...` | SQL-rule texts | lengths, head, tail, B against C | no |
| `t7_owner::stc_family_...` | rewritten (counts from truth; pair equality) | `stc_diffs` | no |

No duplicate hides a weaker check. Two family checks are narrower than "the truth file" suggests: `lob_diffs`
reads only `lob_exp_<v>` (T_LOB / T_LOB2 have 4 sections each; the two `lob_exp_direct_<v>` files are covered only
by pair equality in `t6l_lob::conventional_and_direct_files_give_equal_batches_and_t_frame_follows`, which is
transitive and acceptable), and `r_lob2_diff` compares B with the bytes of the reader's own C
(`truth_checks.rs:407-412`; T-10b note item 4): an expectation built from the output under test, harmless here
because C's length, head and tail come from the truth and `t6l_ru` builds C from `15_ru_lob2.sql`, but it should be
the SQL rule (N-6).

## 3. T-11: strict, per-column overflow, `max_digits_dropped`, survey, new codes

Done-when items against the tests:
- "one test per code of arch 1.4 and 7.1, each asserting code and offset": `t11_codes.rs` has one test per code of
  `ErrorCode::ALL` (24) and `WarningCode::ALL` (19), each through `is(e, code, offset)` or an equal assertion; codes
  without an offset assert `None`. The architecture's `W-LOB-CHARSET`, `W-LONG-DIRECT`, `W-ZERO-LENGTH` are the
  retired ones, asserted absent. Holds.
- "`strict=True` over the fixtures fails exactly on the documented list": `t11_strict.rs:717-750`. Provenance of
  `DOCUMENTED`: the entries are justified by rules and truths, not by the reader (W-TSLTZ of arch 5.1; the three
  failed exports, logs and ADR-39; T_NUM2 / T_NUM3 rounding and T_NUM / T_DATE / T_TS / T_INTERVAL2 errors from the
  truth texts, classified independently in `t10_truth.rs::the_classification_of_the_truth_numbers`), and the counts
  equal the catalog instance counts I took from `exp_catalog.tsv` (T_DATE 8, T_TS 8, T_NUM 8, T_NUM2 2, T_NUM3 2,
  T_INTERVAL2 2, T_TSTZ 4, `???_???` 2, `\xbf..` 1). But the table is a literal, and its "derivation" test
  (`strict_fails_exactly_where_the_plain_run_...`, `:756`) derives the expectation from the reader's own plain
  run and from `strict_error()`, the function under test, so it cannot catch a wrong strict table (mutation a1
  below passes it). Note N-3.
- "a test greps every error and warning text produced from the fixtures for bytes of row values and finds none":
  exists (`t11_options.rs:643`) but cannot fail on the fixtures' messages: S-1.
- Session 5 to 8 items (strict for every walker / resolution / reader warning, `round` -> `E-NUMBER-OVERFLOW`,
  per-column policy, `max_digits_dropped` from the truth text, survey core, the 178 copy): each has a test in
  `t11_strict.rs` / `t11_options.rs`; the survey's "years below 1 for the TIMESTAMP family" has none (S-2).

Mutations (source, scratch copy):

| id | mutation | tests that fail | did not fail but should |
|---|---|---|---|
| a1 | `strict_error()`: `CharsetMismatch => None` (`error.rs:388`) | `error::tests::strict_makes_an_error_of_every_warning_but_the_two_policies`, `t11_strict::strict_charset_mismatch_fails_read_table_itself`, `t9b_fixes::strict_stops_t_lob_before_any_row_...` | (the fixture-wide tests: no fixture has the warning; expected) |
| a2 | reader `warn()`: strict skipped for `W-LONG-LENGTH` (`reader.rs:390`) | `t11_strict::strict_long_length` | - |
| a3 | `strict_check` in the scan ignores `W-TSLTZ` (`dump.rs:449`) | `t11_codes::e_strict`, `t11_strict::strict_tsltz_...`, `strict_over_the_201_files_...`, `strict_fails_exactly_where_...` | - |
| b | overflow policy: entry ignored, global always (`reader.rs:1005`) | `t11_options::the_entry_policy_wins_...`, `the_entry_policy_stops_the_read_...` | - |
| b2 | the global wins whenever it is not `round` | the same two | - |
| c | `max_dropped` = the last value's figure, not the largest (`reader.rs:419`) | `t11_options::w_number_rounded_carries_the_largest_number_of_digits_dropped` | - |
| c2 | `max_dropped` + 1 | the same | - |
| d1 | the failing field's bytes (lossy UTF-8) appended to the value error's message (`reader.rs:497`) | **none (371 passed)** | `no_error_or_warning_text_from_the_fixtures_holds_a_value` |
| d2 | the failing field's bytes in hex appended (a probe printed `[E-NUMBER-OVERFLOW offset=0x230f] integer part does not fit the precision value d10b (...)` and `[E-BAD-DATETIME offset=0x22b6] ... value 6364060f0d0101 (...)`) | **none (371 passed)** | the same |
| e1 | survey: `years_below_1` counted for DATE (12) only, TIMESTAMP family dropped (`survey.rs:108`) | **none (371 passed)** | a survey test (S-2) |
| e2 | survey: NULLs of NUMBER columns not counted | `t11_options::the_counting_core_agrees_with_the_totals_of_the_201_files` | - |
| e3 | survey: refused short / long counts swapped | `t11_options::the_counting_core_counts_the_values_adr_25_refuses_by_form_on_the_178_copy` | - |

No row value in any text, by reading the builders (not the test): `Error.message` and `Warning.message` are
`&'static str` (`error.rs:164`, `:433`); the only constructors take `&'static str`; there is no `Box::leak` /
`String::leak` in `src/`; `render` (`error.rs:528-575`) adds only the code, the offset and the integers table, part,
column, row, length, type code and the warning code; `Warning`'s `Display` adds count, count_to_zero,
max_digits_dropped (integers). `length` carries a set id for `W-CHARSET-UNSUPPORTED` (metadata). The `source` of
`E-IO` is an OS error. So today no message can hold a row value; this is why S-1 is not blocking.

## 4. T-9b

- (a) `reader.rs:1123`: `matches!(c.type_code, 1 | 96 | 112)`, nothing else changed in the condition (diff of
  `e308d9a` read).
- Mutations: r1 (112 removed again) fails `t9b_fixes::form_1_clob_columns_warn_charset_mismatch` and
  `strict_stops_t_lob_before_any_row_and_db_charset_silences_it`; r2 (`W-LONG-LENGTH` at the last piece again,
  `value_offset` -> `offset`, `reader.rs:599`) fails `t9b_fixes::w_long_length_on_a_chain_reports_the_first_piece`;
  r3 (`E-LOB-TOO-LARGE` at the passing piece, `.at(offset)`) fails `t9b_fixes::e_lob_too_large_on_a_chain_...` and
  `reader::tests::max_lob_bytes_is_checked_piece_by_piece`. As the plan says, `t6l_ru::an_nclob_in_a_single_byte_header_is_unconfirmed`
  and the fixture-wide strict list do not respond to r1 (no fixture has a CLOB set different from its database set).
- The patched copies patch what the test says, checked against the fixture bytes:
  - `18c/lob_exp_18c.dmp` @0x30: `03 69 03 69 07 d0` (client 873, database 873, national 2000); the copy writes
    `00 ab` at 0x32-0x33 (database 171). Agrees with `t9b_fixes.rs:38-40`.
  - `18c/long_exp_direct_18c.dmp` @0x2288: `9c 80`; walking the u16 lengths from the bytes (own Python): pieces at
    0x2288 / 0x2326 / 0x4241 / 0x615c / 0x8077 of 156, 7961, 7961, 7961, 7961 (last without 0x8000), end at
    0x9f92; first data byte 0x228a. Agrees with `t9b_fixes.rs:129-156` and `t3c_walker.rs:103-107`.
  - For T-11 (same helpers): `11g-cp1251/ru_lob2_exp_cp1251_11g.dmp` @0x36 = `ab` (low byte of the database set
    `00 ab`), @0x24e3 = `ab` (CLOB descriptor set), @0x2790 = `84 00` (132): agree with `t11_codes.rs:8-9`.
- `form_1_clob_columns_warn_charset_mismatch` (`t9b_fixes.rs:80-86`) compares the short C values of the 171 copy
  with the reader's output of the unpatched file (`tx[..4] == ty[..4]`), with only row 4 against the SQL
  (`m` x 3000). The SQL gives all four (`hello lob`, NULL, empty, `m` x 3000); note N-5.
- The offset rule of ADR-41 item 6 ("the start of the value, the same byte a value error of that value reports")
  is not held by the other reports of the same chain: S-3.

## 5. Test files session 1 did not read line by line

| file | expected values from | findings |
|---|---|---|
| `t6l_lob.rs` (read fully) | `01b_optional_tables.sql` / `13_optional.sql` (literal values match the SQL: `hello lob`, `HEXTORAW('48656C6C6F20626C6F62')`, `RPAD('AB',2000,'AB')` = 1000 x `ab`, row 5 chunks, T_LOB2 `abc` / `x` / EMPTY_CLOB NCLOB / `00FF`), truth lengths, the Analyst's chunk sizes (labelled "observed, ADR-37") | `t_lob2_...` loops over the truth lines without asserting their number (covered by `t_lob2_diff`'s row count). No `unwrap_or` on a checked value, no ignore |
| `t9_charset.rs` (read fully) | DUMP bytes of the truth files decoded by the test (std UTF-8 / UTF-16, Oracle's own CL8MSWIN1251 map from `truth_csmap.txt`) | `a_descriptor_set_other_than_the_database_set_warns_...` (`:978`) passes on **any** mismatch warning; assert column 3 (C) and count 1 (N-4). `us7ascii_high_byte_...` (`:913-923`): the comment speaks of C's own count, the assertion checks V's (column 2) only. `own_exp_w1252_...`: ADR-40 asked this session to check the COMMENT-item claim: in `18c/own_exp_w1252_18c.dmp` the only `?`/0xbf byte is 0xbf @0x2ced, inside `COMMENT ON COLUMN "O_CHILD"."PARENT_ID" IS 'Café € ¿'` (the truth `truth_own_ddl.txt` COLCOMMENT `...d0,b4` = `д`); 11g @0x2cf2, the same item; no `?` at all. **Confirmed** |
| `t4_values.rs` (read fully) | truth DUMP bytes and texts, an own encoder, `value_parts.tsv` (tool) | `a_all_fields_of_the_exp_files_decode` (`:477`): `year_below_1 > 0 && % 2 == 0`; the truth gives exactly 2 per T_DATE section x 8 sections = 16; also `:484-489` only `> 0` for the other types (N-2). `b_float64_and_text` checks the text rendering only by round trip (`parse_decimal(v.to_text()) == v`), the truth text itself is not compared (acceptable: the value is) |
| `t3a_walker.rs` (test functions read) | `walk_parts.tsv`, `lob_expected.tsv` (tool), rules v3.3 / `exp.md` offsets | none; `unwrap_or_default` at `:185`, `:193` is on an optional partition name compared with the TSV's empty field, correct |
| `t3b_parts.rs` (read from `:395`; earlier functions skimmed) | `exp_raw.tsv` digest (tool), truth part counts | `h_column_info_long_and_lob_equal_the_statistics_of_the_walk` (`:547-565`) compares `ColumnInfo.lob` with the walker's own statistics (a count compared with itself through two layers of the reader) and ends with `lob_columns >= 3`; `lob_exp_18c` has exactly 5 LOB columns (C, B, A, N, B) (N-2) |
| `t3c_walker.rs` (`:100-400` read, rest skimmed) | `exp.md` 6.6 offsets, `truth_part2.txt` read by the test | none |
| `t6_patched.rs` (read fully) | truth / SQL, except `long_in_a_single_byte_database_...` (`:69-73`) whose expectation is computed with `encoding_rs` WINDOWS_1251, the library the reader uses (safe in practice: `t9_charset::map_test` shows it equals Oracle's map for all 256 bytes; better: `truth_checks::decode_cp1251`) and `a_broken_utf8_sequence_in_a_chain_is_a_value_error` (`:128`) asserts only `offset.is_some()` (see S-3) (N-4) |
| `t11_codes.rs`, `t11_strict.rs`, `t11_options.rs` (main tests read; helpers skimmed) | patched copies with offsets from `walk_path` / byte search / `field_offsets`; truth texts for rounding and digits; the plan's 178-copy numbers | S-1, S-2, N-3 |

No `#[ignore]`, no skipped test, no `unwrap_or` on a value under test in these files.

## 6. Numbers

Re-derived in the scratch copy with the Analyst's tools: `corpus_counts.py` (TOTAL files 201, sections 386, rows
41,212, row_fields 243,007, null_fields 19,900, text_fields 783, empty 24 / 17, codes 112: 21, 113: 12, problems
0; per folder 89/85/27, 170/164/52), `rowlen_stats.py` (parts 400, lob_sections 21, LOB values 126 = 80 short + 46
long, 12 empty, findings 0), `num_decode.py` (86,080 NUMBER fields decoded and re-encoded, PROBLEMS 0),
`collist.py` (386 `CREATE TABLE` lines, 1,907 column definitions, PROBLEMS 0).

| quantity | tools now | `common/mod.rs` | plan (top table, line 44-61) | notes (N-24, T-10b) | rules v3.5 |
|---|---|---|---|---|---|
| files | 201 | 201 | 201 | 201 | 201 |
| sections | 386 | 386 | 386 | 386 | 386 |
| parts | 400 | 400 | 400 | 400 | 400 |
| rows | 41,212 | 41,212 | 41,212 | 41,212 | 41,212 |
| fields / NULL | 243,007 / 19,900 | same | same | same | same |
| LOB values | 126 (80 + 46) | 126 | 126 | 126 | 126 |
| NUMBER fields | 86,080 | 86,080 | 86,080 | 86,080 | 86,080 |
| column definitions | 1,907 | 1,907 (`TOTALS.column_definitions`, used by `t5_types.rs:171-174`) | not in the top table | 1,907 | **1885 / 382 lines (T9, line 226)** |
| tests | 371 run (372 `#[test]` by grep, one is the string `"\n#[test]"` in `t10_truth.rs:1387`) | - | 347 (T-11 record, correct at `7db4667`) | **370** (T-10b note, lines 969 and 1012) | - |

Numbers that disagree: rules T9 (known, assigned to the Analyst by ADR-38 item 7); the T-10b note's 370 against 371
at HEAD (`#[test]` count: 347 at `7db4667`, 352 at `e308d9a`, 371 at `7941e40`). Review 01's stale numbers
(`gen_walk_expected.py:183-186` comment, `FolderTotals.lob_sections` doc, the `EXP-00008` comment, N-23's 257)
are corrected.

## 7. Notes and decisions

| note | deviation / choice | decided by |
|---|---|---|
| N-25 (T-11) | two new codes; where strict acts; value warnings fatal under `null`; `round` -> `error`; per-column map; `max_digits_dropped`; plan (b) wording (T_NUM cannot "stop at the first value with more than 10 decimals"; null on T_NUM, stop on T_NUM3); `extra_intro_lines`; survey core without the 5.2b profile; the T-9 CLOB bug; W-LONG-LENGTH offset; the fixture-wide list | **ADR-41** items 1 to 7 (all) |
| T-9b note (`implementer-notes.md:952`) | `E-LOB-TOO-LARGE` on a LONG chain now at the first piece; the unit test changed from 30 to 10 | pre-authorised by plan T-9b (c) ("follows the same rule and the note says so"); no Architect answer after the fact; plan T-9b and T-10b have no DONE line yet |
| T-10b note 1 | `DELEGATED` gets a 4th field; `delegated_tests_exist` checks the call | only noted (consistent with ADR-38 item 1 / plan (a2), (a3)) |
| T-10b note 2 | duplicates kept | only noted (open question; section 2 above: no duplicate hides a weaker check) |
| T-10b note 3 | extra comparisons (T_STC `COLUMN` statistics, R_LOB2 head / tail, R_CLOB, T_LOB3) | only noted |
| T-10b note 4 | R_LOB2.B compared with the reader's C; Oracle's map instead of `encoding_rs` | only noted (N-6) |
| T-10b note 5 | `TRUTH_NOT_OPENED` reasons for env / own_ddl / r045 rest on the Reviewer's reading | only noted; checked true here (section 1) |
| T-10b note 6 | the wrapper gap | only noted (N-1) |
| T-10b note 7, 8 | no disagreement found; (a5) done | facts; (a5) checked |
| T-10b note, first paragraph | "nothing is committed", "the uncommitted `reader.rs` / `t9b_fixes.rs`" | stale since `e308d9a` / `7941e40` (N-7) |

## Findings

### Blocking

None.

### Should fix

**S-1 The "no value in any message" test cannot fail on the messages the fixtures produce**
(`t11_options.rs:590-603`, `:643-697`, `:700-709`). `values_of` collects the **raw field bytes** of 8 bytes or more;
a message is a `String`. The fixtures' errors and warnings concern NUMBER, DATE, TIMESTAMP and INTERVAL values
(`E-NUMBER-OVERFLOW`, `E-BAD-DATETIME`, `E-BAD-INTERVAL`, `W-NUMBER-ROUNDED`, `W-VALUE-NULLED`), whose raw bytes
contain bytes above 0x7f (every positive NUMBER starts with 0xc1 or more; DATE starts with the century byte 0x64 to
0x78 and has others above 0x7f) and are shorter than 8 bytes in most rows; such bytes can never appear in a UTF-8
message, and no decoded rendering or hex form is searched. Shown: mutations d1 (lossy bytes) and d2 (hex) put the
failing value into every value error's message; the probe printed `... value d10b ...` and
`... value 6364060f0d0101 ...`; all 371 tests pass. `the_message_check_can_fail` proves only that a VARCHAR2 value
of T_FRAME would be found, a case no fixture message concerns. Today the property holds by construction (section 3),
so not blocking; it must hold after T-12 (`Cow<'static, str>`, ADR-40). Fix (T-12 group): search the messages also
for the reader's rendering of every cell of the fixture (the `cell()` text of a lenient read, at least 4 characters,
excluding pure digit strings shorter than, say, 6) and for the lowercase / uppercase hex of every raw field; and make
the self-test inject a value into a real fixture error (for example render the `E-NUMBER-OVERFLOW` of T_NUM row 19
with its value and assert the check finds it).

**S-2 The survey's "years below 1" is not tested for the TIMESTAMP family** (`survey.rs:108`,
`t11_options.rs:465-512`). The plan (T-11 session 5) asks for DATE / TIMESTAMP family; the test covers T_DATE only and
asserts "no other column" = 0. Mutation e1 (count DATE only) passes all tests. Fix: a patched copy of `t_ts_exp_18c`
(and of a TSTZ / TSLTZ file) with one century byte set below 100 (as `t4_values.rs` does in memory), expecting
`years_below_1` = 1 in that column and `E-BAD-DATETIME` from the reader at the same row.

**S-3 The offset of one LONG chain value differs by report** (`reader.rs:570`, `:603-615`; ADR-41 item 6). Probe on
`long_exp_direct_18c.dmp` T_LONG row 3 (first data byte 0x228a): `E-LOB-TOO-LARGE` with `max_lob_bytes=20000` and
`raise` reports 0x228a (T-9b), but the `W-VALUE-NULLED` of the same value under `null` reports **0x615e** (the data of
the 4th piece, where the limit was passed); `E-BAD-TEXT` for a broken sequence at the end of piece 1 reports
**0x2328** (piece 2). ADR-41 item 6 says the warning's offset is "the same byte a value error of that value
reports" and ADR-27 item 2 "offset of the first value". `t6_patched::a_broken_utf8_sequence_in_a_chain_is_a_value_error`
asserts only `offset.is_some()`, so nothing pins either. Fix: the Architect decides whether `E-BAD-TEXT` inside a
chain keeps the precise byte (useful) or the value start; the `W-VALUE-NULLED` of a chain should at least carry the
offset the `raise` error of the same value carries (`st.start`); pin both in `t9b_fixes.rs`.

**S-4 `DOCUMENTED` (strict list) and its derivation test do not use an independent source for the strict table**
(`t11_strict.rs:717-784`). The derivation test takes its expectation from `strict_error()` and the plain run, both
code under test, so a wrong row in the strict table (mutation a1) passes it; the literal list catches only codes that
occur in the fixtures. Fix: assert the strict outcome of every `WarningCode` against a table written from arch 7.1
in the test (the unit test `error::tests::strict_makes_an_error_of_every_warning_but_the_two_policies` does it at
the code level; the fixture-wide test should use that table, not `strict_error()`), and derive the `DOCUMENTED`
counts from `exp_catalog.tsv` (number of files with T_DATE, T_TS, ...), as T-10b did for the instance counts.
(Borderline note; listed as should-fix because the plan's Done-when names this list.)

### Notes

- N-1 Thin-wrapper gap (section 2): shown by e3 / e4; generic "delete an assertion"; fix suggested there.
- N-2 Loose assertions: `t4_values.rs:477` (`year_below_1 > 0 && % 2 == 0`, exact value 16), `:484-489` (`> 0`);
  `t3b_parts.rs:547-565` (catalog statistics compared with the walker's, `lob_columns >= 3` where 5 is exact).
- N-3 `DOCUMENTED`'s entries are right and justified by rules / truths / logs, and the counts equal the catalog's
  instance counts (re-derived here), but the test itself holds literals (see S-4).
- N-4 `t9_charset.rs:978` (`any` mismatch, assert column 3 and count 1), `t9_charset.rs:913-923` (comment and
  assertion disagree on the column), `t6_patched.rs:69-73` (expectation from `encoding_rs`, the reader's library;
  use `truth_checks::decode_cp1251`).
- N-5 `t9b_fixes.rs:80-86`: rows 1 to 3 of C on the 171 copy are compared with the reader's output of the unpatched
  file; compare them with `01b_optional_tables.sql` (`hello lob`, NULL, empty) as `t6l_lob.rs` does.
- N-6 `truth_checks.rs:407-412`: R_LOB2.B against the bytes of the reader's C; use the bytes of the SQL rule
  (`15_ru_lob2.sql`, as `t6l_ru.rs::lob2_block` builds them). `lob_diffs` (`:1011`) covers only the conventional
  `lob_exp_<v>` files; the direct ones are covered by pair equality (fine, say so in the doc comment).
- N-7 `implementer-notes.md:963-969`: "nothing is committed" and "uncommitted reader.rs / t9b_fixes.rs" are stale;
  "370 tests" (also `:1012`) is 371 at HEAD. Plan T-9b / T-10b headings have no DONE record (Architect).
- N-8 `truth_env.txt` (`NLS_CHARACTERSET=AL32UTF8`, `NLS_NCHAR_CHARACTERSET=AL16UTF16`) could be compared with the
  header set ids of every file of its folder, and the OBJECT TABLE lines of `truth_own_ddl.txt` with the catalog's
  table names of `own_exp_<v>`; both files would then leave `TRUTH_NOT_OPENED` with a real check. `truth_decl.txt`
  is compared only in `t5_types.rs`'s process; a `decl_diffs` in `truth_checks.rs` would let the coverage test see
  it (plan (a2) asks this for delegated families; T_DECL is not delegated, so optional).
- N-9 `t10_truth.rs::break_t_stc_...` uses a literal count `"50"` instead of the truth line, so b6 does not fail it;
  harmless (the family check does), but the break test then does not exercise the truth side of the count.

## What I did not check

- `t3a_walker.rs`, `t3b_parts.rs` (before line 395) and `t3c_walker.rs` (after line 400): read at the level of test
  functions and their expected-value sources, not every line; `t11_*` helper functions skimmed.
- `t1_walk.rs`, `t2_manifest.rs`, `t3_catalog.rs`, `t5_types.rs` (beyond the per-form table and the truth_decl
  test), `t5f_followup.rs`, `t6_read.rs` and `t6l_ru.rs` (beyond the delegated tests), `t6_parts.rs` (beyond T_PART2),
  `t6_value_parts.rs`, `t7_owner.rs` (beyond STC), `t8_direct.rs`: not reviewed this session.
- Golden files: only `gen_walk_expected.py` changed since review 01 (a comment, `7941e40`); I re-ran it in the
  scratch copy as its docstring says (`cd docs/formats/tools && python3 -B .../gen_walk_expected.py`; it printed
  201 / 386 / 400 / 41,212 / 243,007 / 19,900, LOB 126 = 80 + 46, 12 empty, 21 sections) and `walk_expected.tsv`,
  `walk_parts.tsv`, `lob_expected.tsv` are byte-identical to HEAD. The other generators (`gen_number_parts.py`,
  `gen_exp_raw.py`, `gen_exp_catalog.py`) were not re-run: their outputs and inputs did not change since review 01.
- Production code beyond the paths named (strict, overflow policy, `max_dropped`, survey, the T-9b lines, the message
  builders). The correctness of `exp_parse.py` and the other tools themselves.
- Mutations are single-shot and chosen by hand; I did not try every checker, every family check or every strict
  path. `damaged_*_never_panic` tests were not mutated.
- Python (`python/`), Data Pump files, `corpus/work/` (not read, by rule). `docs/design/decisions.md` read only for
  ADR-38 to ADR-42.
- The Cargo target directory outside the repository (`<home>/.cache/oradump-review-02/`, used because `/tmp` was
  full) was deleted at the end of the session.

## git status

Before (start of the session): `?? docs/prompts/review-02-prompt.md`. After: `?? docs/prompts/review-02-prompt.md`
and `?? docs/review/review-02-t9b-t11.md` (this file); nothing else. `docs/review/review-01-golden.md` is tracked and
unchanged. Every experiment ran in the scratchpad copy; nothing under `crates/`, `corpus/`, `tests/` or `docs/`
other than this file was written.
