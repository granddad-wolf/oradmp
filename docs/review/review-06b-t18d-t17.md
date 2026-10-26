# Review 06b: T-18d (file names that are not UTF-8) and T-17 (synthetic dump files)

Reviewer session 6b, 2026-10-24. Prompt: `docs/prompts/review-06b-t18d-t17.md`.

**Commit under review: HEAD = `edc62a5c28ebff689e4ab377565805c8577d7ce3`** ("T-17: synthetic dump generator and tests
(18), fuzz seeds"), on top of `ba8a89c` (T-18d) and the released 0.1.0a3 `ff3d6b25c360f93eeb8c1305927dd6cf3550610b`.

**Verdict: no blocking findings.** T-18d does what ADR-82 asks, and every mutation the prompt names fails a named
test. In T-17 the generator writes correct bytes wherever I could compare it with a real fixture. A synthetic
T_FRAME file is byte-identical to `t_frame_exp_{18c,11g}` and `t_frame_exp_direct_{18c,11g}` from R + 0x104 to the
end of the file. 20,000 NUMBER values and 20,000 DATE / TIMESTAMP values agree with the Analyst's `num_decode.py` and
with Python's calendar. All 11 hand mutations of the reader that change behaviour are caught. The synthetic files
**can be trusted as test inputs for framing and value decoding**. There are five should-fix items. The generator
writes some header and table combinations that no fixture has (with the wrong status in the note in one case). One
guard in the generator does not work. One expected type is taken from reader code. The README sentence of T-18d
promises more than the code does. And three test files besides `t14_damage` can still write
`*.proptest-regressions` files.

## Method and conditions

- Scratch only under `/var/tmp/rev06b`: **6.8 GB** at the end. `repo` is 4.6 GB, a clean clone of HEAD with its
  Cargo target, the fuzz build and the seed folder. `work-target` is 1.5 GB (the mutation build). `work` (190 MB) is
  the mutation clone. There are two venvs (0.4 GB) and the generated files and seed copies (about 40 MB). None of it
  is needed after this report. Settings: `CARGO_BUILD_JOBS=2`, `TMPDIR=/var/tmp/oradmp-tmp`, nextest `-j 2`, one build
  at a time. Fuzz temporary folder: `ORADMP_FUZZ_TMP=/var/tmp/rev06b/fuzztmp`.
- `repo`: `git clone` of the repository at HEAD, used for the step-4 checks, the 20,000-case run, the fuzz run, and a
  probes build installed into its own venv (`maturin develop --features test-probes`). `work`: a second clone for
  the mutations. Its extension was built into a second venv, and it holds one scratch test file of mine,
  `crates/oradmp-core/tests/t17_review.rs` (it exists only in the scratch clone). That file writes generated files
  to `/var/tmp/rev06b/gen/` for the byte comparisons.
- Mutations were run by two scripts. Each script checks that the original text occurs exactly once, applies the
  change, runs the tests, restores with `git checkout --`, and prints `git status --porcelain`. The status was clean
  (apart from my scratch test file) after every run. The one exception was the deliberate `t14_damage` run in 3 (a),
  whose `.proptest-regressions` file I deleted.
- The repository was not touched. `git status --porcelain` showed only `?? docs/prompts/review-06b-t18d-t17.md`
  before and after (plus this report at the end). I did not run `uv run` in the repository root and did not build
  there. The repository's `python/oradmp/_core.abi3.so` was not written by me (mtime 14:45:44, from before this
  session's builds).
- `docker ps`: **no container running** (header line only). Load average 0.5 to 0.9.

## 1. What changed since `ff3d6b2` outside `docs/`

`git diff --stat ff3d6b2 HEAD -- . ':!docs'`: `CONTRIBUTING.md` (4), `README.md` (5),
`crates/oradmp-core/tests/common/mod.rs` (1), `crates/oradmp-core/tests/common/synth.rs` (1665, new),
`crates/oradmp-core/tests/t17_synth.rs` (1557, new), `crates/oradmp-py/src/lib.rs` (3), `python/oradmp/_cli.py`
(11), `python/oradmp/_survey.py` (9), `tests/test_cli.py` (45), `tests/test_paths.py` (102, new),
`tools/fuzz_seeds.sh` (8). Every file is accounted for by the T-18d note or the T-17 note. Nothing changed in
`crates/oradmp-core/src/`, `corpus/`, `tests/golden/`, `crates/oradmp-core/tests/data/`, the Cargo files or
`pyproject.toml`. The version is still `0.1.0-alpha.3` / `0.1.0a3`. `tests/test_survey.py`, which the T-18d plan
lists, is unchanged, which is allowed.

## 2. Findings

### Blocking

None.

### Should fix

**SF-1. The generator writes header and table combinations that no fixture has, and the note gives E3 the wrong
status.** ADR-80 item 1 says the generator "emits a construct as valid input only when its rule is confirmed". The
note says that E1, E2, E3 and E4 are confirmed (`implementer-notes.md` T-17 note, "Rules used").
- **E3 is "suggested (follows from E1, E2)"** (`exp_reader_rules.md` line 263). The construct that rests on it is
  `ff ff` (E1) after path flag 1 in an **18c** file. That construct is in no fixture. `exp.md` 5.4 lists all 18c
  direct-path empty tables, and every one is the E2 form; 11g writes E1. It is generated in
  `both_empty_table_forms_after_a_direct_path_flag` (`t17_synth.rs:588`, table D_E1 in an 18c file) and in about
  half of the random 18c direct-path empty tables (`synth.rs:1593-1597`). The plan's (d) itself asks for "[E1, E2,
  E3]", so the test is what the plan wanted. But the note's status is wrong, and the plan and ADR-80 item 1
  disagree.
- **An 18c header with database set 171** is generated: `random_spec` takes `v18` and `db_171` independently
  (`synth.rs:1471-1473`), and `every_generated_type_with_edge_values` uses `spec_of` (V18) with `db_set = 171`
  (`t17_synth.rs:843-929`). The rules file says "databases AL32UTF8 (both versions) and CL8MSWIN1251 (11g only)"
  and "Not covered by any fixture: ... a single-byte database on 18c" (lines 145, 162). I listed the set ids of all
  201 EXP headers (a small script in the scratch folder): no 18c file has database set 171.
- **Client set 171 with database set 873** (`client_pick` gives 171 with any database, `synth.rs:1472`) is in no
  fixture either. The same header listing shows clients {873, 1, 178} with database 873, and {873, 171, 1, 178} with
  database 171. Owner mode with client 1 is not in the fixtures.
- **TIMESTAMP(f) for f in 1 to 5, 7 and 8** is generated (`synth.rs:1495`, `rng.range(0, 9)`) with descriptor length 11
  (`synth.rs:505`). The fixtures have only (0) with length 7 (8 sections), and (6) and (9) with length 11 (10 and 8
  sections). I counted these with a script over every `CREATE TABLE` line and its descriptor. The note gives the
  source of "7 for (0), 11 otherwise" as "`t_ts_exp_18c` and `types/map.rs` `check_agreement`". The second source
  is **reader code**, and the reader checks this agreement. Mutation G2 below (descriptor always 11) fails two T-17
  tests. So for f outside {0, 6, 9} the generator copies the reader's assumption (common mode by construction, not
  only by reading the same rules).

Why it matters: none of these is wrong as far as anyone knows, but each one widens "confirmed" without evidence, and
the note says otherwise. Suggested fix: limit `v18 && db_171` to V11 and the client sets to the fixture pairs, and
generate TIMESTAMP precisions only from {0, 6, 9}. Or keep the combinations and list them in the note as
"unconfirmed combinations, accepted by rule". Correct the E3 status in the note. The Architect should say whether
plan (d) or ADR-80 item 1 wins for E1 after flag 1 on 18c.

**SF-2. The E2 guard in the generator does not work: 1.3 % of the E2 tables are followed by `ANALCOMPUTE`.**
`gen_noise_after` (`synth.rs:1456-1462`) drops an `ANALCOMPUTE` line only when `i == 0`. When the first drawn item
is skipped, the next one becomes the first item behind the E2 table, and it can be `ANALCOMPUTE` too. Over
`params_from_seed(0..200000)` I counted 10,404 E2 empty tables, and **131** of them have `ANALCOMPUTE` as their first
following item. `ANALCOMPUTE ` is not in the E2 (a) list of the rules (`exp_reader_rules.md` line 262). The tests
pass anyway, because the reader's E2 test (a) for a first column with non-zero length is "an upper-case letter
follows" (`rows.rs:277-286`, `lf_is_empty_table`), not the list. I showed this with a deterministic file (E2 table,
then `ANALCOMPUTE TABLE "E"`): the scan accepts it, with 0 rows. So the generator writes a construct its own comment
says it avoids, and nothing notices. Fix: test `v.is_empty()` instead of `i == 0`.

**SF-3. One expected type comes from reader code.** `verify` builds the expected Arrow type of each `number_types`
override with `nt.arrow()` (`t17_synth.rs:99`). That function is `oradmp_core::types::map::NumberType::arrow`
(`src/types/map.rs:52`). ADR-80 item 2 says expected values never come from reader code. If `arrow()` mapped `Utf8`
or `Int64` wrongly, the type checks at `t17_synth.rs:222` and `:297` would still pass. Fix: write the three expected
types as literals (`DataType::Int64`, `DataType::Utf8`, `DataType::Decimal128(20, 4)`) in
`number_types_override_the_arrow_type`.

**SF-4. The README sentence of T-18d is broader than the code.** `README.md:159`: "A file name that is not valid
UTF-8 is accepted; messages write each such byte as `\xHH`." This holds for the dump file (FILE) and for `survey`.
It does not hold for the **output** path of `export -o`. I ran it with an output name made of the same bytes:
- an existing target gives `oradmp: error: [E-CONFIG] the target .../\udcd2\udce5\udcf1\udcf2.csv exists; ...`
  (`_cli.py:546`, `_refuse_existing`);
- a read-only folder gives `oradmp: error: [E-IO] Permission denied: .../\udcd2\udce52.csv.oradmp-partial`
  (`_cli.py:637`, `e.filename` unescaped).

Python's `backslashreplace` on stderr keeps this from crashing, but the form is `\udcHH`, not `\xHH`. ADR-82 item 3
names FILE only, so the code meets the ADR and the README overstates it. Fix: either pass the output path and
`e.filename` through `_path_text`, or narrow the README to "a dump file name".

**SF-5. Three other property-test files keep proptest's default failure persistence.** This is open point (a); the
details are in 3 (a). I confirmed the note's claim: under the mutation `ByteStream::truncated` at
`offset + 100000`, `t14_damage` fails 3 tests and writes `crates/oradmp-core/tests/t14_damage.proptest-regressions`
(untracked; seed line "shrinks to idx = 4, cut = 84899"). The same default is in `t4_values.rs:1160` and
`t5_types.rs:1172` (`proptest!` without a config), which the note does not mention. Fix: in Architect terms,
`failure_persistence: None` in all three, the failing input in the message (proptest prints the shrunk input
anyway), and remove `t14_damage.rs:12` "proptest's regression file is committed".

### Notes

- **N-1. Structure that differs from the fixtures but that the reader does not depend on** (from the byte
  comparisons in 3):
  - (a) table mode with several tables: the fixtures write **one trailer group set (5 `METRICST...` groups) per
    table** (`multi_framing_exp_18c` 3 sets, `multi_empty_exp_direct_18c` 3 sets); the generator writes one set per
    file (`synth.rs:1016-1052`);
  - (b) the per-LOB-column `TABLE "t"` / `ENDTABLE` groups of rule T1 in the trailer (`lob_exp_18c` @0x4dfe3) are
    not generated;
  - (c) owner mode: the group order differs (the fixture has views before the tables' constraints and "Enable
    enforce constraints " last, `own_exp_18c`), and many groups are missing;
  - (d) the view line is `CREATE VIEW "V1" ("A") AS` without the trailing space of the fixture (`own_exp_18c` @0x3473
    `... AS ` + LF);
  - (e) R + 0x000 to R + 0x102 are zeros, while every fixture holds the two blocks and the strings (`64 00 00 00 00 60
    01 24 ...` at R).

  T2, T7 and H6 make the reader independent of all five. Copying the 0x104 bytes of `t_frame_exp_18c` (with the set
  ids patched) and one trailer set per table would make the fuzz seeds closer to real files.
- **N-2. Direct-path LONG chains have random piece sizes** (`chain_pieces`, `synth.rs:817`): a short value can be
  split into several pieces, and a middle piece can be shorter than 7,961. The fixtures show only 156 + 4 x 7,961 and
  one-piece values. R2 says "piece sizes: never assume", so the framing is confirmed but the shapes are not.
  Suggest one deterministic case with the fixture's exact shape (`9c 80` + 156, 3 x `19 9f` + 7,961, `19 1f` + 7,961).
- **N-3. The property tests are random from run to run** (no persistence, no fixed RNG seed). Mutation K2 (E2 test
  removed) failed only the two deterministic tests at the default 64 cases. With `PROPTEST_CASES=1000` it also
  failed both property tests. The note's K2 row lists all four, which depends on the run. The deterministic tests
  carry this mutation, so no change is needed. But "the property test catches X" should not be claimed from one run.
- **N-4. `the_random_specs_cover_every_construct` measures `params_from_seed`**, not the proptest strategy `params()`
  (`t17_synth.rs:338` versus `synth.rs:1106`). The two have different probabilities (owner mode 30 % against 50 %,
  set 171 25 % against 50 %). The coverage claim holds for the seeds and the fuzz seeds, not exactly for the
  property test.
- **N-5. The note's list of what is not generated is incomplete** against the rules. Also not generated: the list-K
  entries `CREATE FORMAT71 PROCEDURE ` and `CREATE FORMAT92TRIG ` (I6). From T9: `NUMBER(*,s)` / INTEGER,
  `FLOAT(b)`, `NUMBER(p, s)` with s < 0 or s > p. Also the T1 LOB trailer groups (N-1 (b)), C7's undefined bytes
  (1251: 0x98), a column-field length above 32,000 (R8, "not an error"), and a G1 introducing line (correctly left
  out: suggested). By grep, fixture tests exist for each of these: `t1_walk.rs` (FORMAT71 / FORMAT92TRIG),
  `t5_types.rs` and `t5f_followup.rs` (`NUMBER(*`, FLOAT), `t11_*` (G1). I did not measure code coverage.
- **N-6. Note text:** "after fixing four mistakes of my own" lists three. The K1 row says "at least 8": in my run 12
  T-17 tests failed.
- **N-7. Line length:** `CONTRIBUTING.md:56` (127 characters) is new in T-17. The comment line 4 of
  `tools/fuzz_seeds.sh` is about 230 characters. README line 173 (N-7 of 06a-closure) is now within 120.
- **N-8. `tools/fuzz_seeds.sh` runs `cargo test` from the caller's working directory** (`--manifest-path` does not
  change rustup's toolchain choice). From inside `fuzz/` it builds the workspace with the fuzz nightly, and from
  outside the repository with the default toolchain. `tools/fuzz_run.sh` calls it before `cd fuzz`, so the normal
  use (from the repository root) takes 1.99.0. CONTRIBUTING does not say that the seed step compiles the test
  binary with the root toolchain (time, and a second target build if the toolchain differs). One sentence would do.

## 3. T-17: the generator against the rules and the fixtures

### Byte patterns the generator writes, with their evidence

| what (`synth.rs`) | rule / evidence | status |
|---|---|---|
| `03`, u16 BE client, `D` LF if direct, the 7 lines (`write_header`, 721-747) | H1, H2, H3. Byte-identical with `t_frame_exp_18c` / `_direct_` / `_11g` from 0 to 0x35 (0x37 direct) | confirmed |
| 3 x u16 BE sets, then `00 01` + 8 zeros, u16 BE name length, `00` (750-756) | H4. The 10 bytes are `00010000000000000000` in **all 201** EXP files (my count) | confirmed as constant (meaning unknown) |
| spaces, date ending at 0x7f, name at 0x80 (758-764) | H4a; same place as `t_frame` @0x69 / @0x80 | confirmed (informative) |
| zero fill to R; R + 0x000 to 0x102 zeros, LF at R + 0x103 (766-769) | H5 confirmed. LF at R + 0x103 in all 201 files. **The zeros contradict the fixtures** (non-zero constant blocks); H6 says skip | LF confirmed; content contradicted, harmless (N-1 (e)) |
| `METRICST` / `METRICSU` first, pairing with the mode (992) | H6 | confirmed |
| plain, marked, unmarked fields (667-688) | I3, I4; I5 with one piece only | confirmed (I5 as bytes) |
| list-K lines used: ` MODIFY DEFAULT`, `COMMENT ON `, `ANALSTATS ` (plain and marked), `CALTER TABLE`, `PROCACTION`, `CREATE VIEW ` | I6, list K, S1 | confirmed |
| `ANALCOMPUTE`, `GRANT`, `CREATE INDEX`, `ALTER TABLE ... ADD`, `CREATE SEQUENCE` lines | I2, S2, T7 | confirmed |
| `TABLE "t"` + `CREATE TABLE` + storage text (836-839, 773-792) | T1, T9; storage text = `t_frame`'s | confirmed |
| type texts (550-569): `NUMBER`, `NUMBER(p, 0)`, `NUMBER(p, s)`, `DATE`, `TIMESTAMP (f)`, `RAW(n)`, `VARCHAR2(n[ CHAR])`, `CHAR(n[ CHAR])`, `NVARCHAR2(n)`, `NCHAR(n)`, `LONG`, `LONG RAW`, `CLOB`, `BLOB` | T9 forms; `NUMBER(3, 0)` as in `lob_exp_18c` | confirmed |
| `INSERT INTO` in file order, LONG last on the direct path (844-855, 205-211) | T3 | confirmed |
| u16 count + descriptors (857-862, 494-547) | T5, C3, C5, C6 (w = 4 / 1). Descriptors equal to `t_frame` (`01 00 0a 00 69 03 01 00`), `lob_exp_18c` (`70 00 48 00 69 03 01 00`, `71 00 48 00`), `t_empty` | confirmed; **TIMESTAMP length 11 for f not in {0, 6, 9}: unconfirmed** (SF-1) |
| m, flag; L1 entries `n`, name, `00 00` (864-881) | T6, L1, L6. Identical with `lob_exp_18c` (`03 00 00 00 00 00 00 01 43 00 00 01 42 00 00`) | confirmed; the `00 00` after an entry is constant, meaning unknown |
| fields u16 + bytes, `fe ff`, `00 00` / k after a conventional row (915-955) | R1 to R4, R7, L3 | confirmed |
| direct LONG chain with random piece sizes (921-936, 817-826) | R2 | framing confirmed; shapes not seen (N-2) |
| LOB locator: random 102 to 139 bytes (709-712) | L2 (content not needed) | framing confirmed; content invented |
| short LOB value u16 + bytes, length 0 for empty, CLOB as UTF-16 BE (904-908, 809) | L4 (short form; all values below one chunk), L5 (873) | confirmed |
| `ff ff` LF / LF only (959-965) | E1, E2; **E1 after flag 1 in an 18c file rests on E3 (suggested)** | E1 / E2 confirmed; the 18c-E1 case unconfirmed (SF-1) |
| E2 followed by `ANALCOMPUTE` | not in the E2 (a) list | not intended, generated (SF-2) |
| trailer groups, `METRICE? n`, `METRICE?G0`, `EXIT` LF x2, zero fill (973-1061) | T2, R6, E4. Titles as in `t_frame` | confirmed as text; one set per file, unlike the fixtures (N-1 (a)) |
| owner-mode extras, statement groups with `ENDTABLE` (993-1043) | T1, I4, I5 | confirmed as constructs; order differs (N-1 (c)) |
| NUMBER bytes (392-430) | Y1. 20,000 random values (1 to 38 digits, scale 0 to 38, both signs) equal `num_decode.encode` and round-trip through `num_decode.decode`, 0 differences (Analyst tool, run with `python3 -I`) | confirmed |
| DATE / TIMESTAMP bytes (458-479) | Y2, Y5. 20,000 random instants (years 1 to 9999, half with nanoseconds) equal bytes computed in Python from Y2 / Y5 with `datetime` | confirmed |
| UTF-16 BE, CP1251 through `encoding_rs` | C3, C5, L5, C7 (only А-я, Ё, ё, ASCII) | confirmed |

The unconfirmed bytes the note lists: the 10 filler bytes of H4 are **confirmed as constant** (201 / 201). The zeros
at R to R + 0x102 are **contradicted** by the fixtures (harmless). The LF at R + 0x103 is confirmed. The 2 bytes
behind a LOB entry are confirmed as constant (L1). The TIMESTAMP descriptor lengths are confirmed for (0), (6) and
(9), and unconfirmed for the other precisions. The storage text and the trailer titles are confirmed as fixture
text. The locator content is invented (allowed by L2). The `METRICE? n` number is unconfirmed and harmless (R6).

### Byte-level structure comparison (generated file against a real fixture of the same shape)

1. **T_FRAME** (3 x `VARCHAR2(10)`, the 7 rows of `truth_frame.txt`, R = 8192, user CORPUS). I compared four
   generated files with `t_frame_exp_18c`, `t_frame_exp_direct_18c`, `t_frame_exp_11g` and
   `t_frame_exp_direct_11g`. Each pair differs only in the header file name and date (H4a, plus the name-length byte
   at 0x41 / 0x43) and in R + 0x000 to R + 0x102 (the H6 blocks). From **R + 0x103 to the end of the file the bytes
   are identical**: the first item, the table section, the descriptors, m and flag, every row, the `00 00` row ends,
   `ff ff` LF, the 5 trailer groups, `EXIT` x2 and the zero fill.
2. **T_LOB** (ID, C CLOB, B BLOB) against `lob_exp_18c`. Descriptors, m = 3, flag 0, the three L1 entries, the
   locator framing (u16 + bytes), k = 2 after row 1, `12 00` + UTF-16 BE, `0a 00` + BLOB bytes: same structure. The
   locator lengths differ (random 102 to 139 against 114). The storage text differs, and the fixture has `LOB (...)
   STORE AS` clauses.
3. **T_EMPTY**, 18c direct, E2, against `t_empty_exp_direct_18c`. The `INSERT INTO` line, descriptors, `00 00 01 00`
   and the LF before `METRICST...` are identical. The storage text differs (the 18c fixture has no `STORAGE(...)`,
   which is R-017's hypothesis).
4. **Owner mode, 3 tables**, against `own_exp_18c`: the group order and set differ (N-1 (c)). **Table mode, 3
   tables**, against `multi_framing_exp_18c`: one trailer set against three (N-1 (a)).

### Where the expected values come from (ADR-80 item 2)

- Row values: `expected_rows` (`synth.rs:327`) maps the Spec's own cells `t.rows` (made by `gen_cell`,
  `synth.rs:1262`, or written literally in the deterministic tests) through `expected_cell` (`synth.rs:309`). That
  function uses `padded_text` (the generator's padding), `decimal_text` (`synth.rs:284`) and `common::ts_text`
  (`common/mod.rs:210`, a test helper). `verify` compares these with `common::rows` (`t17_synth.rs:247-268`), which
  only turns Arrow arrays into text.
- Arrow types: `arrow_type` (`synth.rs:264`), a literal table that matches arch 5.1 row by row. The one exception is
  the override types (SF-3).
- Header, names, counts, NOT NULL: Spec fields. Offsets: `Layout`, recorded with `W::at()` while writing. The
  generator imports nothing from `oradmp_core` (`use` lines of `synth.rs`: `arrow_schema`, `encoding_rs`). It has
  its own NUMBER encoder and does not use `common::encode_number`.

### Mutations (each alone; `cargo nextest run -p oradmp-core --test t17_synth -j 2 --no-fail-fast`, 64 cases)

| # | mutation | failing T-17 tests |
|---|---|---|
| K1 | `fe ff` read as an empty value (`rows.rs` NULL arm calls `k.value(.., &[])`) | 12 of 17 (all but `every_generated_type...`, `each_refusal...`, `the_generated_negative_base_is_clean`, `the_random_specs_cover...`, `record_lengths...`) |
| K2 | [E2] test removed (`if false && self.rows == 0 ...`) | `both_empty_table_forms_after_a_direct_path_flag`, `owner_mode_with_three_tables`; with `PROPTEST_CASES=1000` also both property tests (N-3) |
| K4 | sign of a negative NUMBER flipped (`negative: false`) | 6: `number_types_override...`, `projection...`, `strict_on_a_clean_file`, `a_random_spec...`, `batch_rows...`, `every_generated_type...` |
| O1b | [C3, C5] UTF-16 row text decoded little-endian (`Charset::decode`) | `every_generated_type...`, `a_random_spec...`, `a_truncated...` |
| O1c | [L5] UTF-16 piece decoder little-endian (CLOB) | `a_lob_table_in_both_kinds_of_file`, `a_random_spec...` |
| O2 | [Y2] hour byte not minus 1 | 7 tests |
| O3 | [Y5] TIMESTAMP nanoseconds little-endian | `every_generated_type...`, `a_random_spec...`, `a_truncated...` |
| O4 | [R2] a chain stops after 4 pieces | `a_32000_byte_long...`, `batch_rows...`, `a_truncated...` |
| O5 | [Y1] negative exponent `0x3F - b0` | 7 tests |
| O6 | [list K] `CALTER TABLE` removed | `a_random_spec...` (through `g1_fields != 0`) |
| O7 | [I5] `CREATE VIEW ` removed | `owner_mode...`, `strict_on_a_clean_file`, `a_random_spec...` |
| O8 | [L1] 1 byte behind a column entry instead of 2 | `a_lob_table...`, `the_generated_negative_base_is_clean`, `a_random_spec...`, `each_refusal...` |
| O9 | [C7] 1251 decoded as 1252 | `every_generated_type...`, `a_random_spec...` |
| O1 | UTF-16 → LE in `decode_text` | none. **Equivalent mutation, my error**: `decode_text` decodes names and DDL only, and no row data passes through its UTF-16 arm. Redone as O1b / O1c |
| G1 | generator: expected CHAR / NCHAR text without padding | `every_generated_type...`, `a_random_spec...` |
| G2 | generator: TIMESTAMP descriptor always 11 | `every_generated_type...`, `a_random_spec...` (the reader checks the agreement: SF-1) |
| G3 | generator: `VARCHAR2(n CHAR)` descriptor n instead of min(w n, 4000) | `every_generated_type...`, `a_random_spec...` |
| G4 | generator: the 10 filler bytes of H4 set to `aa` | none: expected, the reader skips them (H4) |

I did not redo the note's third mutation (two bytes dropped at each multiple of R) because of the build time. O4 and
the boundary test cover R-related framing in part.

Six of my mutations touch rules outside the Implementer's four: C3 / C5 (O1b), L5 (O1c), Y2 (O2), Y5 (O3), R2 (O4),
list K (O6), I5 (O7), L1 (O8), C7 (O9).

### Property tests with more cases

`PROPTEST_CASES=20000 cargo nextest run -p oradmp-core --test t17_synth -j 2 --no-fail-fast` on the clean clone:
**17 passed, 1 skipped**, 168.8 s (both property tests SLOW above 120 s / 60 s). No `*.proptest-regressions` file
was written; `git status --porcelain` was empty after the run. At the default 64 cases the binary takes 2.4 s, and at
2,000 cases 18.7 s (the note says 2.7 s and 17.8 s). A failure would **not** write a regression file:
`config()` sets `failure_persistence: None` (`t17_synth.rs:325-336`). I checked this under mutations K1 to O9: no
`t17_synth.proptest-regressions` appeared (`find`). The only regression file I ever saw is `t14_damage`'s (3 (a)).

### What is not generated

The note's list (E2a, partitions, NCLOB, long LOBs, INTERVAL, ROWID, TSTZ / TSLTZ, non-ASCII names, LONG in a 1251
database as non-ASCII, a third database set) is correct as far as it goes. N-5 lists the further gaps. Each of them
has fixture-based tests by grep. I cannot say that no reader construct is untested without a coverage run, which I
did not do.

## 4. T-18d: mutations and tests

Probes build in its own venv (Python 3.13, pyarrow 26.0.0, pandas 3.1.0); `pytest tests/test_paths.py
tests/test_cli.py tests/test_survey.py` (91 tests) per mutation, restored after each.

| mutation | failing tests |
|---|---|
| `_error_text` keyed on `strerror.startswith("[E-IO")` again | `test_cli.py::test_the_io_error_text_of_the_api_error` (`'[E-IO] [E-IO] I/O error'`) |
| `open(path: String)` in `lib.rs` (rebuilt; full pytest) | `test_paths.py`: `..._opens_and_reads_a_name_that_is_not_utf8`, `..._names_a_missing_file_with_enoent`, `..._tool_works_on_a_name_that_is_not_utf8`, `..._tool_writes_a_missing_name_as_ascii`, `test_survey_writes_the_file_line_with_escapes` (5 failed, 228 passed) |
| `path_text` without the `backslashreplace` step | `test_the_tool_writes_a_missing_name_as_ascii`, `test_survey_writes_the_file_line_with_escapes`, `test_path_text` |
| `path_text` with `"replace"` instead of `"backslashreplace"` | the same 3 |
| no `\t` escape | `test_path_text`, `test_the_io_error_text_of_the_api_error` |
| no `\r` escape / no `\n` escape (each alone) | `test_the_io_error_text_of_the_api_error` (line 936 of `test_cli.py`) |
| `survey` without `path_text` | `test_survey_writes_the_file_line_with_escapes` |
| `_cli._path_text` = identity | `test_the_tool_writes_a_missing_name_as_ascii`, `test_the_io_error_text_of_the_api_error` |
| `-n < 0` guard off | `test_head_with_a_negative_count_in_a_subprocess_is_exit_2` fails by name (returncode -6, `std::length_error`); pytest finished normally |
| README sentence removed | no test (none expected: documentation; checked by reading, SF-4) |

`tests/test_paths.py` on Linux: 7 passed, 0 skipped. **Not checked off Linux.** On macOS (APFS) such a name cannot be
created, and the module skips (`sys.platform != "linux"`). On Windows, file names are UTF-16, and how pyo3's
`PathBuf` conversion and `os.fsencode(...).decode(..., "backslashreplace")` behave with lone surrogates there is not
known to me. The skip reason "POSIX byte file names" is right for the tests as written. T-16 (h) has to show the
skips.

## 5. The three open points of the T-17 note

**(a) `t14_damage.proptest-regressions`:** confirmed (SF-5). With `ByteStream::truncated` at `offset + 100000`,
`each_truncation_site_names_its_offset`, `truncation_gives_ok_or_a_typed_error` and
`bit_flips_give_ok_or_a_typed_error` fail, and the file appears untracked next to the source. Is
`failure_persistence: None` the right fix? Yes, for this project. The stored seed only reproduces a failure of the
mutated build (the note explains why, and N-5 of review 06a saw it). An untracked file in `tests/` is a commit
hazard. Proptest prints the shrunk input in the failure message anyway, and for `t14_damage` that input (`idx`,
`cut`, `flips`) is enough to replay by hand. The same change belongs in `t4_values.rs` and `t5_types.rs`. The doc
comment at `t14_damage.rs:11-12` must change with it. A `.gitignore` entry is the weaker option (owner's file; it
hides the file but keeps writing it).

**(b) Seed folder:** `tools/fuzz_run.sh` passes `corpus/<target>` (first, written by libFuzzer) and
`../target/fuzz-seeds` (read only) to `cargo fuzz run` for every target, so one folder serves all three. ADR-80 (g)
"the seed folders of the three targets" is inaccurate, and the implementation is reasonable. The Architect should
fix the wording. Run on the clean clone, alone, after `tools/fuzz_seeds.sh` (435 files = 235 fixtures + 200
`synth__NNNN.dmp`): `tools/fuzz_run.sh read 60` printed `INFO: 435 files found in ../target/fuzz-seeds`, `seed
corpus: files: 435 ... max: 729088b`, `#436 INITED cov: 4368 ft: 16278 corp: 221/6998Kb`, `Done 71848 runs in 61
second(s)`, exit 0, `fuzz/artifacts/read/` empty: **findings 0**. To show the synthetic seeds matter, I ran
`cargo fuzz run read <empty dir> <seeds> -- -runs=0` three times:

| seeds | INITED |
|---|---|
| fixtures only | cov 4209, ft 12792, corp 87 |
| synthetic only | cov 3301, ft 13501, corp 162 |
| both | cov 4368, ft 16277, corp 223 |

So the synthetic seeds add about 160 edges and 3,500 features. `fuzz_run.sh` runs `fuzz_seeds.sh` only when
`target/fuzz-seeds/` is missing, so an old seed folder without synthetic files is reused until someone runs
`tools/fuzz_seeds.sh` (which empties it first). The CONTRIBUTING text says so.

**(c) `cargo` needed by `tools/fuzz_seeds.sh`:** the CONTRIBUTING text matches what the script does. It says the
script writes `SYNTH_SEEDS` synthetic files "with the generator of the tests". It does not say this compiles and runs
`cargo test` with the root toolchain picked by the caller's directory (N-8).

## 6. Full checks (clean clone at HEAD)

| check | result |
|---|---|
| fixture `SHA256SUMS` (three folders) | ok |
| `cargo fmt --all --check` | ok |
| `cargo clippy -- -D warnings`; `--all-targets`; `--all-targets --all-features` | ok, ok, ok |
| `cargo nextest run -j 2` | **429 passed, 4 skipped** (as the note says) |
| `cargo deny check` | `advisories ok, bans ok, licenses ok, sources ok` (warning: unmatched licence allowance `Zlib`, as before) |
| `pytest -rs` (probes build, 3.13, pyarrow 26.0.0, pandas 3.1.0) | **233 passed, 0 skipped** (as the note says) |
| `ruff check`, `ruff format --check`, `mypy` | ok (33 files formatted; mypy: no issues in 23 files) |
| `git status --porcelain` after nextest, the 20,000-case run and the fuzz run | empty |

## 7. Numbers that disagree

- The note's K1 row says "at least 8" (12 in my run); the K2 row lists 4 tests (2 at 64 cases in my run; N-3).
- The note says "four mistakes of my own" and lists three (N-6).
- The note says E3 is confirmed; the rules file says suggested (SF-1).
- ADR-80 (g) "seed folders of the three targets" against the one shared `target/fuzz-seeds/` (5 (b)).
- Counts that agree: Rust 412 + 3 → 429 + 4; pytest 227 - 1 + 7 = 233; 435 seed files (235 + 200); 2,000-case time
  17.8 s (note) against 18.7 s (mine).

## 8. What I did not check

- Anything off Linux (the `test_paths.py` cases; pyo3 `PathBuf` with lone surrogates on Windows; `path_text` on
  Windows, where `os.fsencode` uses `surrogatepass`).
- The note's third reader mutation (two bytes dropped at each multiple of R): not redone.
- Code coverage (no `cargo llvm-cov` run), so "no construct of the reader is untested" is not established; only the
  grep in N-5.
- The release build, PyPI, benchmarks and CI (out of scope for this session).
- The 600-second fuzz runs of `walk` and `values`: only `read`, for 60 s.
- Whether the H6 blocks, the locator content or the trailer order could matter to a future reader: the current
  reader skips them (shown for the H4 filler by G4; not shown for the others).
- The CP1251 map beyond the characters the generator uses (C7's undefined byte 0x98 is not generated).
- I read `exp.md` only in sections 5.4 and 13.2 (the E2 evidence). The rest of the comparison is against
  `exp_reader_rules.md` and the fixture bytes.
