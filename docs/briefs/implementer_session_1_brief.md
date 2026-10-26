T-1 is built and all five "Done when" items pass. Your message still had the placeholder `T-n`; I ran T-1 because no `crates/` existed and it is the only task with no prerequisites.

I re-ran the checks myself after the implementer finished: `cargo fmt --check` is clean, `cargo clippy --all-targets -- -D warnings` is clean, and `cargo nextest run` passes 39 of 39. Nothing is committed, and nothing in `corpus/`, `docs/formats/` or `python/` was touched by this session.

## Two things that need your decision

- **169 fixtures, not 161 (N-1).** The plan counts 161 files, but the folders hold 169: `long_exp`, `nchar_exp`, `t_decl_exp` and `t_num2_exp` for both versions are untracked in git but listed in `SHA256SUMS`. The plan's numbers hold exactly for the other 161. The tests cover all 169.
- **`long_exp` files stop the walker (N-2).** They have LONG (type 8) and LONG RAW (type 24) columns. Rule X1 says to stop on them, so the walker returns `UnsupportedType`, while the Analyst's tool reads them and walks on. The Analyst needs to decide whether 8 and 24 get a rule or stay in X1.

## Done-when items

All tests are in [t1_walk.rs](crates/oradump-core/tests/t1_walk.rs).

| Item | Test | Result |
|---|---|---|
| (a) header fields equal the tool table | `a_header_fields_of_every_file_equal_the_tool_table` (all 169 files, 0 header warnings) | pass |
| (b) walk ends at second `EXIT`, 0 warnings, counts match | `b_walk_ends_at_the_second_exit_with_the_numbers_of_the_tool` (160 files that do not stop), `b_reduced_totals_of_the_154_files_without_a_lob_table` | pass |
| (c) empty list K reads the same items through G1 | `c_empty_list_k_reads_the_same_items_through_g1` | pass |
| (d) list file equals the rules block | `d_intro_list_file_equals_the_block_in_the_rules_file` | pass |
| (e) 7 CLOB files give `UnsupportedType` | `e_the_seven_clob_files_give_unsupported_type_at_the_descriptor`, plus one confirming `ru_tables_exp_us7ascii_11g.dmp` stops at 0x11adc | pass |

Reduced totals for the 154 files without a LOB table, committed with the test:

| | 18c | 11g | 11g-cp1251 |
|---|---|---|---|
| files | 68 | 68 | 18 |
| sections | 127 | 127 | 28 |
| fields | 222 | 222 | 242 |
| empty tables (direct, conventional) | 6, 10 | 6, 10 | 0, 0 |

A separate test confirms the tool table reproduces the plan's full numbers for the 161 files (735 fields, 127 / 127 / 48 sections). The implementer reports mutation checks for (c), (d) and (e): removing the behaviour failed the test each time. I did not repeat those.

## Numbers from the Analyst's tools

Expected values come from `exp_parse.py` and `intro_list.py`, run read-only on 2026-10-07 over all three fixture folders. They are in [walk_expected.tsv](crates/oradump-core/tests/data/walk_expected.tsv), produced by [gen_walk_expected.py](crates/oradump-core/tests/data/gen_walk_expected.py), which imports the tools rather than copying them. `check_all.py` and `header_fields.py` were also run.

## Smaller deviations

These are recorded with N-1 and N-2 in [implementer-notes.md](docs/design/implementer-notes.md):

- **N-3:** the rules block has no field forms, so (d) does not compare them. The data file adds them under the names `plain`, `marked` and `unmarked`, which are the implementer's own.
- **N-4:** the check that the `INSERT INTO` column count equals the descriptor count is not in T-1; it needs the identifier grammar, so it is left for T-3.
- **N-5:** choices to confirm:
  - Two new code names, `W-FILE-SIZE` and `E-CONFIG`.
  - `WalkEnd::Stopped` is used only for X1; every other fatal error is `Err`.
  - A section stopped by X1 is not in `sections`.
  - The `00` byte after the file-name length and the `0`/`72`/`0` header lines are not checked.
  - The mode line is not compared with `METRICST`/`METRICSU`.

The toolchain is pinned to 1.99.0 in `rust-toolchain.toml`, and rustup installed it on first use. The crate has no licence set.

## For T-2

- **Entry points:** `exp::walk`, `walk_path` and `walk_stream`, with `WalkOptions` (`max_line_bytes`, `intro_list`, `record_items`, `buffer_bytes`).
- **Where to hash raw fields:** rows are counted in `exp::rows::walk_rows`, which skips values by length; that loop is the place to extend.
- **Not decoded yet:** names, DDL and column names are left for T-3.