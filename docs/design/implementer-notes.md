# Implementer notes

Questions and problems the Implementer found while building. Each entry: task, file and rule or section,
what was seen, what is proposed. The Architect and the Analyst answer; the Implementer does not work around
a point silently.

## Changes in session 31 (Architect)

- "T-16c note": "Answers (Architect, session 31)" added at its end (ADR-116 to ADR-119). T-16c is DONE and accepted.
  The next task is T-16b (plan, its "Session 31" block).

## Changes in session 30 (Architect)

- "T-20b" note, the session 28 answer on the T-19 (m) figure: "The note stays as written" replaced (T-21 corrected the
  result line to "-4.8 % to -6.8 %"; ADR-114 item 5). No other place in this file holds the old figure.
- New section "T-21 (no note committed; Architect, session 30)" with "Answers (Architect, session 30)" at the end of the
  file (ADR-114, ADR-115). T-21 is DONE; the next task is T-16c (plan, its "Session 30" block).

## Changes in session 28 (Architect)

- "T-20b" note: "Answers (Architect, session 28)" added at its end (ADR-109, ADR-110). T-20b is DONE and reviewed (6c);
  the next Implementer task is T-21, after the a4 tag (plan).

## Changes in session 27 (Architect)

- "T-20 note": "Answers (Architect, session 27)" added at its end (ADR-108). The next task is T-20b (plan).

## Changes in session 26 (Architect)

- The repository is public since the cut (ADR-105): hashes cited in this file before session 26 name commits of the
  private archive (`HISTORY.md`); cites of `CLAUDE.md` and `docs/prompts/` name private files.
- "T-19 (m)" note: "Answers (Architect, session 26)" added at its end (ADR-106, ADR-107). The next task is T-20 (plan).

## Changes in session 25 (Architect)

- "T-19 note": "Answers (Architect, session 25)" added at its end (ADR-99 to ADR-104). The next task is T-19b (plan),
  then T-19 (m).

## Changes in session 23 (Architect)

- "T-16a note": "Answers (Architect, session 23)" added at its end (ADR-89 to ADR-93). The next task is T-16a2 (plan).

## Changes in session 22 (Architect)

- "T-18d note" and "T-17 note": "Answers (Architect, session 22)" added at the end of each (ADR-85, ADR-86). The next
  task is T-17b (plan), then T-16a.

## Changes in session 21 (Architect)

- "T-18c note": "Answers (Architect, session 21)" added at its end (ADR-81, ADR-82). The next task is T-18d (plan).

## Changes in session 20 (Architect)

- "T-18b note": "Answers (Architect, session 20)" added at its end (ADR-77, ADR-78).

## Changes in session 19 (Architect)

- "T-15b note" and "T-18 note": "Answers (Architect, session 19)" added at the end of each (ADR-75, ADR-76). The T-13j
  note needs no answer (nothing unforeseen; its mutation table goes to Reviewer 6a).

## Changes in session 17 (Architect)

- "T-13i note": "Answers (Architect, session 17)" added at its end (ADR-69).

## Changes in session 16 (Architect)

- "T-15 note": "Answers (Architect, session 16)" added at its end; they answer the T-14 note's problems too (ADR-64 to
  ADR-68).

## Changes in session 15 (Architect)

- "T-13h note": "Answers (Architect, session 15)" added at its end (ADR-61, ADR-62).

## T-13g note (Implementer, 2026-10-18): rename to `oradmp`

Done as ADR-54 and the plan say. Facts: folders moved by `git mv` (`crates/oradmp-core`, `crates/oradmp-py`,
`python/oradmp`); stale `_core.abi3.so` and `__pycache__` of the old folder deleted; every tsv under
`crates/oradmp-core/tests/data/` is a pure rename (`git diff HEAD -M --numstat`: 0 / 0); both generators re-run from
the new paths (`gen_walk_expected.py` via `uv run` in `docs/formats/tools`, `gen_number_parts.py`), the tsv files are
byte-identical, only the docstring path lines of the two scripts changed. `Cargo.lock` diff: the two package names
and the one dependency line. `git check-attr text` on a tsv prints `unset`.

(c) outputs: the first grep is empty; `grep -ci oradump README.md` is 1; `find` prints only `./oradump.code-workspace`;
the full `grep -rIli` list is `CLAUDE.md`, four `corpus/` files, `docs/briefs`, `docs/design`, `docs/prompts` (finished
sessions), `docs/research`, `docs/review` and `README.md` (the one line): all in ADR-54 item 5's kept or owner groups.

(d) `tests/check_dist.py` scans every member name and content, text and binary, for the old name (built from parts),
blanking only the exact README line in the sdist `README.md` / `PKG-INFO` and the wheel `METADATA`. One `FAIL` line per
hit with byte offset and line. Scratch copies (outside the repository): wheel with `oradmp/_errors.py` renamed to
`ora`+`dump/x.py` gives two FAIL lines and `check_dist: FAILED`; sdist whose `README.md` got a second line with the name
gives `FAIL: sdist member oradmp-0.1.0a1/README.md: the old project name at byte offset 9390 (line 161)`; the untouched
archives print `check_dist: ok`. `tests/test_project_name.py` (3 tests) scans `git ls-files` of the (c) paths and checks
the README has the one line right after the title; on a scratch git copy with the name planted it fails.
(e) `__version__` is `importlib.metadata.version("oradmp")`, `"0+unknown"` on `PackageNotFoundError`;
`tests/test_version.py` (1 test) fails on a scratch copy with the literal `"0.1.0"`. pytest: 69 -> 73 (3 + 1).
`mypy` now also type-checks `check_dist.py` (imported by the new tests); one `str(...)` was added to its
`get_payload()` call to satisfy strict mode, no behaviour change.

(g) wheel `oradmp-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl`, sdist `oradmp-0.1.0a1.tar.gz`; `METADATA` has
`Name: oradmp`, `Version: 0.1.0a1`; SBOM `oradmp-0.1.0a1.dist-info/sboms/oradmp-py.cyclonedx.json`.
(h) fresh venv (`uv venv` outside the repository, wheel installed): `import oradmp` -> `0.1.0a1`; `import oradump` ->
`ModuleNotFoundError` (nothing installed under that name); `python -m oradmp survey FILE` prints `oradmp survey`. In the
repo `.venv`: `uv pip uninstall oradump`, `maturin develop`; `oradump.pth` and `oradump-0.1.0.dist-info` are gone,
`import oradmp` works, `import oradump` fails.
(i) `sha256sum -c` clean in the three fixture folders; fmt, clippy `--all-targets`, nextest **395**, `cargo deny check
licenses`, ruff check, ruff format --check, mypy, pytest **73**: all green.

### PROBLEM 1 (blocks `check_dist: ok` in this checkout; owner action)
Task T-13g (g), `tests/check_dist.py` (d) vs ADR-54 item 4. The CycloneDX SBOM that maturin writes holds the absolute
checkout path (`path+file://<home>/p/oradump/crates/oradmp-py#...`, six places). In the checkout folder
`~/p/oradump`, the new scan therefore fails on the wheel: `check_dist: FAILED`, six FAIL lines, all in
`oradmp-0.1.0a1.dist-info/sboms/oradmp-py.cyclonedx.json`. The `.so` is clean. Built from a copy at a path without the
old name (`/dev/shm/.../oradmp`, outside the repository), the same commands give `check_dist: ok`; wheel SHA-256
`14a150fd9ac4fa8f6564ce2c2ff509b14fa7310bf060e3f98c304351074df3bc`, sdist
`5788717ca4dc636494406931a0150617211a06e157db27c8beb6ad3ee362ff48` (the wheel hash depends on the build path; the
sdist does not). The same SBOM also carries `<home>` (the user name) in a published file. Proposal: the owner renames
the checkout folder to `~/p/oradmp` before the release build (then ADR-54 item 4 is no longer "the owner's business"
only), and the Architect decides whether the release build should also remap or avoid the home path (e.g. build in a
neutral folder). I did not weaken the scan or add an exception for the SBOM. The wheel and sdist now in
`target/wheels/` (built in `~/p/oradump`) have the SBOM hit and must not be released.

### Note 2 (information)
`maturin build` includes `python/oradmp/__pycache__/*.pyc` when the folder is not in a git work tree (my scratch copy had
no `.git`; the pyc files carry the source path). In the repository `.gitignore` keeps them out, so the real wheel has
none. A release build from a plain copy of the tree (not a clone) would fail the new scan for this reason; build from the
clone.

### Answers (Architect, session 14)

- **PROBLEM 1** (SBOM with the checkout path; also the home path and user name, and 161 home paths in the `.so`,
  review 03 S-2): ADR-56. The release is built by `tools/release_build.sh` from a clean clone at a neutral path
  (`/var/tmp/oradmp-release/src` by default) with `--remap-path-prefix` for `CARGO_HOME` and the build root; the SBOM is
  kept; `check_dist.py` fails on `/home/`, `/Users/`, `/root/`, `C:\Users\` in any member (ADR-57). Work in T-13h (b),
  (c). The owner's checkout folder name no longer matters for the release.
- **Note 2** (`.pyc` in a wheel built from a plain copy): ADR-57 item 1 (`__pycache__`, `.pyc`, `.pyo` fail); the
  script builds from a clone, so `.gitignore` applies. T-13h (c).
- The scratch folder `~/.cache/oradump-t12` of T-12 is still there (review 03 N-8): T-13h (f) deletes it.

## Changes in session 14 (Architect, 2026-10-18)

The T-13g note got an "Answers (Architect, session 14)" block (ADR-56, ADR-57). Review 03's findings are answered by
ADR-55 to ADR-60; the work is T-13h (`implementation-plan.md`). No correction to the note's facts.

## Changes in session 13 (Architect, 2026-10-17)

The T-13f note got an "Answers (Architect, session 13)" block: problem 1 is ADR-54 item 7, problem 2 is ADR-54 item 8;
both are done in T-13g. The project is renamed `oradmp` (ADR-54); the notes above and below keep the old name
(session reports, history). No correction to the note's facts.

## Changes in session 12 (Architect, 2026-10-17)

The T-13 note got an "Answers (Architect, session 12)" block: P1 is ADR-49, P7 is ADR-50, P2 to P6, P8, P9 are
ADR-51; `.gitattributes` is ADR-52. No correction to the note's facts. New work: T-13f (`implementation-plan.md`).

## Changes in session 11 (Architect, 2026-10-17)

The T-11b and T-12 notes got "Answers (Architect, session 11)" blocks: T-11b items 1 to 13 and T-12 D1 to D10 and
the pandas / pyarrow findings are answered by ADR-3 (session 11), ADR-19 / ADR-28 (session 11), ADR-41 items 8, 9,
ADR-46, ADR-47, ADR-48. No correction to the notes' facts. New work: T-13 as rewritten in session 11.

## Changes in session 10 (Architect, 2026-10-17)

The T-9b and T-10b notes got "Answer (session 10)" lines (ADR-44; the review's findings in ADR-43). Stale facts in the
T-10b note are corrected in place, marked "[session 10: ...]": the tests are **371** at `7941e40`, not 370, and
T-9b / T-10b are committed (`e308d9a`, `7941e40`). New work: T-11b and the session 10 items of T-12 and T-13
(`implementation-plan.md`).

## Changes in session 9 (Architect, 2026-10-17)

N-21 (rest), N-22, N-23, N-24 and N-25 got "Answer (session 9)" lines; the decisions are ADR-38 (the first
review's findings, T-10b widened), ADR-39 (the `W-POSSIBLY-INCOMPLETE` entries), ADR-40 (the open points of
N-21 to N-24), ADR-41 (the T-11 choices, task T-9b) and ADR-42 (roles, review gate). N-23's sentence on the CLOB A
is corrected in its answer, not edited. Work: T-9b, T-10b, T-12 in `implementation-plan.md`, "Order of the next
tasks (session 9)".

## Changes in session 8 (Architect, 2026-10-17)

The merge of `t9` and `t10` had left six git conflict-marker lines in this file (after the T-8 notes, around
the T-9 and T-10 sections); all three sides were additions, so the marker lines are removed and no text is
changed. N-21 got an "Answer" line on the `db_charset` / ADR-25 point only (ADR-25 session 8 changes it); N-22
got one on the leftovers (task T-10b). The other N-21 / N-22 points are not reviewed in this session. Work:
T-6M, T-3d, T-10b in `implementation-plan.md`, "Order of the next tasks (session 8)".

## Changes in session 7 (Architect, 2026-10-17)

N-19, N-20 and the T-8 notes got "Answer" lines; the decisions are ADR-34 (T-6L), ADR-35 (T-5f) and ADR-36
(T-7 / T-8) of `decisions.md`. The work the answers create: T-9 (`#[doc(hidden)]` on the two LOB counters,
`W-CHARSET-MISMATCH` for form-1 CLOB), T-10 (`T_LONG_EMPTY` in the direct LONG files only), T-11 (`strict`
for `W-LOB-UNTESTED`, the survey's count of refused LOB values), T-12 (Python mapping); see
`implementation-plan.md`, "Order of the next tasks (session 7)".

## Changes in session 6 (Architect, 2026-10-16)

N-17 and N-18 got "Answer" lines; the decisions are ADR-32 (T-3b) and ADR-33 (T-6) of `decisions.md`, and
ADR-18 records the arrow-array 60.0.0 check. The work the answers create: the T-5 follow-up (now with
`ColumnInfo.type_source`), small fixes at the start of T-6L (`Dump::rows`, `part_rows`, counted warnings
flushed on error), T-11 (per-column policy, `max_digits_dropped`); see `implementation-plan.md`, "Order of
the next tasks (session 6)".

## Changes in session 5 (Architect, 2026-10-16)

N-12 to N-16 and the T-3c "Points for the Architect" got "Answer" lines; the decisions are ADR-27 to ADR-31
of `decisions.md`. AQ-1 is closed (answered 0; ADR-23 stands). The work the answers create: T-3b (per-part
matching, `part`, assembled-LONG digest), a short T-5 follow-up, T-6 (value warnings, file order to
declared order; may run in parallel with T-3b), T-11 (`strict` for the new warnings); see
`implementation-plan.md`, "Order of the next tasks (session 5)".

## Changes in session 4 (Architect, 2026-10-15)

N-9 to N-11 got "Answer" lines. The owner's answers on the T-3a choices are ADR-23 of `decisions.md`.
New: question **AQ-1** to the Implementer (end of file). The work the answers create is task **T-3c**
(new, before T-3b) of `implementation-plan.md`.

## Changes in session 2 (Architect, 2026-10-14)

Each note N-1 to N-8 got an "Answer" line (accept, change, or sent to the Analyst). The work the answers
create is task T-3a of `implementation-plan.md`, except where an answer names another task.

## T-1

### N-1 Eight fixtures that the plan does not count (169 files, not 161)
- Where: `implementation-plan.md` (top), `architecture.md` (fixture count); folders `corpus/fixtures/18c`,
  `11g`, listed in the modified `SHA256SUMS`.
- Seen: `*_exp_*.dmp` gives 72 / 72 / 25 = **169** files. The new ones are `long_exp_<v>`, `nchar_exp_<v>`,
  `t_decl_exp_<v>`, `t_num2_exp_<v>` for both versions (untracked in git, listed in `SHA256SUMS`). The plan's
  numbers (68 / 68 / 25 files, 127 / 127 / 48 sections, 222 / 222 / 291 fields, empty tables 6 / 6 / 1 and
  10 / 10 / 2) hold exactly for the 161 files without them (tested: `tool_table_reproduces_the_plan_numbers_for_the_161_files`).
  The eight files add 6 sections and 1 field per version.
- Done: the T-1 tests cover all 169 files; `tests/data/walk_expected.tsv` has a `plan` column (1 for the 161,
  0 for the eight). Nothing is skipped.
- Proposal: the Architect updates the fixture count of the plan and of arch; the Analyst writes up the eight files.
- **Answer (Architect, session 2): accepted.** The Analyst wrote the eight files up (rules v3.2); plan and
  arch now count 169 files, 133 / 133 / 48 sections, 223 / 223 / 291 fields (737). T-3a drops the `plan`
  column and replaces the 161-file test by one test of the 169-file numbers at the top of the plan.

### N-2 LONG (type 8) and LONG RAW (type 24) columns in `long_exp_<v>.dmp`: rule T5 / X1 versus the tools
- Where: rules T5, X1; `corpus/fixtures/{18c,11g}/long_exp_<v>.dmp`; `tools/exp_parse.py`.
- Seen: `T_LONG` has columns of type 8 (descriptor max length 0) and `T_LONGRAW` type 24. T5 lists 8 codes and X1
  says "any other type code: stop". `exp_parse.py` reads every unknown code as a 4-byte descriptor and walks
  these files to the second `EXIT` without a problem (3 sections, `T_LONG` 3 rows, `T_LONGRAW` 2 rows, `T_FRAME`
  7 rows), which is evidence for a 4-byte descriptor, but no rule says so.
- Done: the walker follows the rules file: `UnsupportedType` at the descriptor (type code 8, offset from the
  tool table, 3 sections found by the tool but none closed before the stop). Test:
  `e_long_columns_stop_like_any_unknown_type_code`. The descriptor offsets in the TSV come from the tool.
- Proposal: the Analyst decides whether types 8 and 24 get a rule (descriptor size, row bytes) or stay in X1.
  Phase 1 has no LONG support (ADR-12), so this only matters if the owner's tables have such columns (Q-4).
- **Answer (Architect, session 2): changed by the rules.** Following the rules file was right for v3.1. Rules
  v3.2 add 8 and 24 to T5 and add Y9 (4-byte descriptor, length 0, values are ordinary fields). LONG and LONG RAW
  are now in scope (ADR-15, arch 5.1): T-3a reads them, inverts `e_long_columns_stop_like_any_unknown_type_code`
  into `y9_long_and_long_raw_are_read`, moves the two `long_exp` files off the "expected stop" list and
  regenerates `walk_expected.tsv`, `exp_raw.tsv` and `exp_catalog.tsv` (295 tables in 163 files).

### N-3 The list block of the rules file has no field forms
- Where: rules file, rule I6 and the block "Lines that introduce a field"; ADR-9 and arch 2.4.
- Seen: the block gives `kind:length:text`; the allowed forms stand in the prose paragraph below it. The data file
  `data/exp_intro_lines.txt` adds the forms (`plain`, `marked`, `unmarked`; names chosen here, the architecture
  only shows `plain|marked`). Test (d) compares kind, length and text with the block; it cannot compare the forms.
- Proposal: the Analyst adds the forms to the block (for example `kind:length:forms:text` as in the data file), so
  that the whole entry is checked. Until then a change of a form in the prose is not detected by a test.
- **Answer (Architect, session 2): answered by the Analyst.** The block is now `kind:length:forms:text` with the
  names `plain`, `marked`, `unmarked` (rules v3.2, A-3), equal to the data file as it stands. T-3a extends test
  (d) to all four fields.

### N-4 Check "column count of the `INSERT INTO` line equals the descriptor count" not in T-1
- Where: arch 7.1 (fatal class), rules T3, T4; plan T-1 does not list it, T-3 reads the column names.
- Seen: counting the columns of the `INSERT INTO "t" ("c1", ...) VALUES (:1, ...)` line needs the identifier
  grammar (quoted names can contain `,` and `:`), which T-3 needs anyway.
- Proposal: T-3 does the check (`Framing` at the `INSERT INTO` line). Not done in T-1.
- **Answer (Architect, session 2): accepted**, and done in T-3 (N-7). Arch 1.4 lists it under `Framing`.

### N-5 Choices not fixed by the design, made in T-1 (please confirm or correct)
- Warning code `W-FILE-SIZE` (file size not a multiple of R [H5], arch 7.1 names the condition, not a code) and error
  code `E-CONFIG` (invalid list K or option) are new names. `W-UNTESTED-VERSION`, `W-UNTESTED-HEADER`,
  `W-UNKNOWN-INTRO`, `W-TRAILING` are implemented now because the header and the walker need them; strictness
  (turning them into errors) is T-11.
- `WalkEnd::Stopped` is used only for `UnsupportedType` (ADR-12). Every other fatal error is `Err` and gives no partial
  report. A section stopped by X1 is not in `WalkReport::sections`; the error carries its ordinal (`table`) and the
  column position. T-3 may want the partial section (name, descriptors before the unknown one): not built.
- A section ends (field `end_offset`) at the line that ends it by T2, or at the first `EXIT` line of the final pair.
- A `TABLE "name"` line followed by `CREATE TABLE` opens a section; a second `INSERT INTO` in one section, or one
  outside a section, is `Framing`; a table with 0 columns is `Framing` (otherwise the direct path would loop on a
  row of no bytes).
- The empty-table test of arch 5.4 (both conditions, else `Framing`) is already in `exp::rows`, because the walk
  needs it to find the end of the table. T-8 therefore has only the synthetic ambiguous case and the pair
  comparison left; the unit test `exp::rows::tests::empty_table_decision_needs_both_tests` covers the decision.
- The `00` byte after the file-name length [H4] and the three header numbers `0`, `72`, `0` [H3] are not checked
  (meaning unknown). The mode line is not compared with the first item (`RTABLES` / `METRICST`).
- The workspace `Cargo.toml` states `rust-version = "1.99"`; `rust-toolchain.toml` pins `1.99.0` (rustup installed
  it on first use). No licence is set in the manifests: the owner decides.
- **Answer (Architect, session 2), point by point:**
  - `W-FILE-SIZE`, `E-CONFIG`: accepted; arch 7.1 lists them.
  - `WalkEnd::Stopped` only for `UnsupportedType`, every other fatal error `Err`: accepted. The stopped section
    need not be in `WalkReport::sections`, but `Catalog.end` must name the stopped table (from its `TABLE` line),
    the column position and the type code, so that `survey` can print it (arch 3.2); do it in T-3a if T-3 does not
    already. No partial `TableInfo` for the stopped table: its rows cannot be located [X1].
  - Section end at the T2 line or the first `EXIT`: accepted.
  - Second `INSERT INTO`, `INSERT INTO` outside a section, 0 columns: `Framing`: accepted (fixtures have 2 to 120
    columns).
  - Empty-table decision already in `exp::rows`: accepted; T-8 keeps the pair comparison, the synthetic ambiguous
    case and the LONG-first case of arch 5.4.
  - **Checks not made** (`00` after the file-name length [H4]; lines `0`, `72`, `0` [H3]; mode line against
    `METRICST` / `METRICSU` [H6]): **T-3a** adds all three as `W-UNTESTED-HEADER` with the offset, not as errors,
    because their meaning is unknown (ADR-17); T-11 makes them errors under `strict`. Asked the Analyst whether
    the mode pairing is confirmed (A-10).
  - Toolchain 1.99.0: accepted. Licence: question Q-6 to the owner.

## T-2

### N-6 Choices made in T-2 (please confirm or correct)
- `sha2` is a normal dependency of `oradump-core` (the walker hashes, option `WalkOptions::hash_fields`), not only a
  dev-dependency; the row walk reads field bytes in 64-byte steps when hashing (no effect on the offsets).
- The manifest `tests/golden/exp_raw.tsv` lists only tables before the first X1 stop of a file (the `T_LONG*`, CLOB
  files): 289 lines for 169 files. It carries `type:maxlen` per column, so T-3 can compare lengths too. The Test
  engineer's review of the manifest is still to be done.
- `truth_counts.txt` lists `T_FLOAT` and `T_ROWS`; neither is in any exp fixture (`T_FLOAT` has types outside T5 or
  is only in Data Pump files; `T_ROWS` is in `corpus/work/`). The test asserts exactly this. There is no
  `truth_counts.txt` in `11g-cp1251`.
- **Answer (Architect, session 2): accepted, all three.** `sha2` as a normal dependency is in arch 9 now (small,
  pure Rust; the hashing is opt-in through `WalkOptions::hash_fields`). The manifest rule "tables before the first
  X1 stop" stays; after T-3a it lists 295 tables (the 6 of the `long_exp` files added). `T_FLOAT` and `T_ROWS`:
  correct as asserted. The Test engineer's review is done on the regenerated manifest.

## T-3

### N-7 Choices made in T-3 (please confirm or correct)
- Public entry is `oradump_core::dump` (`open`, `Dump`, `DumpInfo`, `Catalog`, `CatalogEnd`, `TableInfo`,
  `ColumnInfo`, `TableRef`, `TableRows`). `Dump::scan` / `tables` / `table` / `rows` take `&mut self` (the catalog is
  cached in the `Dump`). `read_table` and `TableBatches` are T-6; `TableRows` is the T-3 row cursor (rows skipped by
  length, no value decoded); it shares the code of the scan (`exp::rows::RowCursor`).
- `Options` has only `max_line_bytes` and `buffer_bytes` so far. `ColumnInfo.declared` and `arrow_type` do not exist
  yet (T-5, T-6). `TableInfo.possibly_incomplete` is always `false` (X2 has no sign in the file). Sections without an
  `INSERT INTO` line are not in `Catalog.tables`; the ordinal keeps counting them.
- Names and DDL: ASCII decoding, every byte above 0x7f becomes U+FFFD and sets `name_possibly_damaged` /
  `ddl_lossy`; the raw bytes are kept (`name_raw`, `ddl_raw`, `ColumnInfo::name_raw`). `TableInfo.name` comes from the
  `TABLE "name"` line; the golden file shows it equals the `INSERT INTO` name in all 289 tables, but nothing checks
  or relies on that in the reader.
- `oracle_type` for form 2 of type 1 / 96 is `NVARCHAR2` / `NCHAR` (meaning of the form value is "suggested" in T5).
- New error codes: `E-FILE-CHANGED`, `E-NO-SUCH-TABLE`, `E-AMBIGUOUS-TABLE-NAME`. N-4 is done: an `INSERT INTO` line
  that does not parse (rule T3 grammar) or whose column or placeholder count differs from the descriptor count is
  `Framing` at the line's offset (the check runs for every table that is not stopped by X1).
- `FileChanged` compares size and modification time, as designed. A file that is replaced within the clock tick of
  the original (same size, same mtime, for example a copy made in the same few milliseconds) is not detected; the test
  gives the replacement its own mtime. Proposal: also compare device and inode on Unix (`MetadataExt`, no `unsafe`).
  Not done, because the design says size and time.
- The snapshot is taken at `open`, not at `scan`: a file that changes between `open` and `scan` is `FileChanged` at
  the scan.
- **Answer (Architect, session 2; outside the N-1 to N-6 asked for, short):** public entry `oradump_core::dump`,
  `&mut self` for `scan` / `tables` / `table` / `rows`, `TableRows` as the T-3 cursor, ASCII-only names with
  U+FFFD and flags, raw bytes kept, `oracle_type` NVARCHAR2 / NCHAR for form 2 (now confirmed by C5), new error
  codes, snapshot at `open`: accepted. Device and inode on Unix (`std::os::unix::fs::MetadataExt::dev` / `ino`)
  in addition to size and mtime: accepted, do it in T-3a. `possibly_incomplete` always `false`: change in T-9,
  when names are decoded with the client set: set it as arch 6.5 defines it (name possibly damaged and 0 rows),
  with `W-POSSIBLY-INCOMPLETE`; it is a "possibly", not a claim. Sections without `INSERT INTO` outside
  `Catalog.tables`, ordinal counting them: accepted.

### N-8 `.log` files against the golden file (task request of the project owner)
- No mismatch of a row count. All 169 fixtures have a `.log`. Every table of `exp_raw.tsv` (289) has a `rows exported`
  line with the same name and count; the 22 further `rows exported` lines of the logs all belong to files that stop at
  an unsupported column type (CLOB or LONG, X1), after the stop, so the golden file cannot list them.
- Tables in the logs without a count: `???_???` (us7ascii) and `\xbf\xbf\xbf_\xbf\xbf\xbf` (w1252) in `ru_exp_*`: the
  line is followed by `EXP-00056` (conventional) or `EXP-00008` (direct; error 604) and no count. All lie behind the
  CLOB stop of those files, so the file side of X2 was not seen in this task.
- The log shows the name in the client character set; the test compares escaped bytes with the escaped
  `name_raw`, which only works because the fixtures' log and dump were written by the same client.
- **Answer (Architect, session 2): accepted.** After T-3a the 6 LONG-file tables join the golden file; the
  extra `rows exported` lines should drop from 22 to 16, all in CLOB files (assert it).

## T-3a (Implementer, 2026-10-15)

Built: the walker of rules v3.3. All 177 files of the rules file walk to the second `EXIT` with no warning and no stop;
the numbers of the plan (334 sections, 338 parts, 40,172 rows, 237,347 row fields, 18,994 NULL fields, 737 text fields,
22 / 13 empty tables, 15 LOB sections, 62 LOB values) come out of the walker itself. Tests: `t1_walk.rs` (numbers, list K
in four fields), `t3a_walker.rs` (new), adapted `t2_manifest.rs` and `t3_catalog.rs`.

### N-9 Choices made in T-3a (please confirm or correct)
- **API changes.** `Section.data: Option<TableData>` is now `Section.parts: Vec<Part>` plus `partitioned`; `Part` holds the
  `INSERT INTO` line, `TableHeader` (now with `m`, `m_offset`, `lob_entries`, `odd_entry_tail`), `RowsSummary` and
  the `PARTITION` line (`PartitionLine`). `RowsSummary` has `fields`, `lob: LobSummary`, `lob_detail`
  (k per row and every value with offset, form, length and chunk lengths; only with `WalkOptions::record_lob`) and
  `e2a_no_rows`; it is no longer `Copy`. `RowCursor::step` takes an `Env` (list K, `max_line`, `strict_e2a`);
  `RowCursor::from_parts` is `from_columns(path, (type code, max length) per column, table)`. `read_first_item`
  returns the warnings of the mode pairing. New: `WalkReport::parts()`, `lob_totals()`, `stopped_section`.
- **`UnsupportedLayout` stops the file** like `UnsupportedType` (`WalkEnd::Stopped`, ADR-12). A path flag other than 0 / 1
  is now `UnsupportedLayout` (was `Framing` in T-1; arch 1.4 lists it there). Order of the checks: flag value, m neither 0
  nor the column count, m and the LOB descriptors disagree, flag 1 with m > 0, each entry (n > 0 exactly for 112 / 113).
- **[R8] in column fields:** length 0 and 0xfff0 to 0xfffd are `Framing` at the length field (with row, column and the
  length); `fc ff` in a LONG column is that error. Not touched: 0xfffe (NULL), `ff ff` inside a row (was `Framing`).
- **LOB value area [L3, L4]:** k must equal the number of non-NULL LOB columns of the row (else `Framing` at the k field);
  a short value is any h in 0 to 0xffef (an empty value is a short one: the counts are 54 short of which 12 empty, 8 long);
  `fc ff` + 8 bytes LE total + chunks; a long value with T = 0 (no chunks) is accepted (not covered by the rules; harmless).
  A u16 of 0xfff0 to 0xfffb, 0xfffd to 0xffff in the value area is `Framing`. LOB values are not hashed (T-3b adds the hash).
- **Partitions [T10, ADR-21].** arch 2.5 step 4 says both "a further INSERT INTO only after a PARTITION line has been seen in
  the section (else Framing)" and "a part not directly preceded by a PARTITION line: W-UNTESTED-LAYOUT". Implemented as:
  a second `INSERT INTO` in a section in which no `PARTITION` line was seen: `Framing`; a `PARTITION` line seen in the
  section but not the line directly before this `INSERT INTO`: read, named after the last `PARTITION` line,
  `W-UNTESTED-LAYOUT`. Descriptors, m, entries, table and column names of every part must equal the first part's (else
  `Framing` at the `INSERT INTO` line). A section with `PARTITION` lines that ends without `ENDPARTITION`, and either line
  outside a section: `W-UNTESTED-LAYOUT`. Both lines never reach G1.
- **L1 warnings** (`W-UNTESTED-LAYOUT`, once per section): the 2 bytes after an entry other than `00 00`; an entry name
  that differs from the `INSERT INTO` name of that column.
- **[E2a].** Implemented as ADR-22 says, with test (a) kept as the second byte being an upper-case letter (arch 5.4, N-5):
  first column of descriptor length 0, LF at p: second byte not a capital: a row is read; else the line from p + 1 must be
  a complete line of a known kind (`TABLE "`, `ENDTABLE`, `METRICS`, `CREATE `, `ALTER `, `COMMENT ON `, `GRANT `,
  `ANALSTATS `, `CALTER TABLE`, `PARTITION "`, `ENDPARTITION`) whose list-K field, if any, is well-formed; the check reads
  ahead and seeks back to p (one reposition, only in this case). "No rows" gives `W-UNTESTED-LAYOUT` at p. `strict`
  (T-11 will wire it) is `WalkOptions::strict_e2a`: "text follows" is `Framing`. `W-LONG-DIRECT` (new code) is raised by
  the walker per part with path flag 1 and a LONG / LONG RAW column; `W-LONG-CHARSET` and `W-LONG-LENGTH` are values and
  belong to T-6.
- **ADR-17 checks** done: the lines after RECORDLENGTH (`0`, `72`, `0`), the byte after the file-name length, and the
  mode line against `METRICST` / `METRICSU` are `W-UNTESTED-HEADER` with the offset (the first differing line / the byte /
  the first item). `Mode::Other` raises no pairing warning (it already has one).
- **Catalog (until T-3b).** `TableInfo` still describes the **first** binary part (`num_rows`, `rows_offset`, ... of part 1);
  for T_PART that is 1,095 rows, not 3,000, until `TableInfo.parts` arrives. `oracle_type` names the new codes. `Catalog`
  has `stopped_table` (ordinal, name from the `TABLE` line, offset): the answer to N-5 "Catalog.end must name the stopped
  table". The error keeps only the ordinal (no name from the file in an error). The device / inode comparison of N-7 is done.
- **`Dump::warnings`** listed the header warnings twice after `scan` (the walk reads the header again); fixed.
- **Golden files not regenerated** (that is T-3b): `t2_manifest.rs` and `t3_catalog.rs` compare the old 289 lines as a
  prefix of each file's tables (the tables before the old stop) and assert that no file stops; the log test now compares
  every catalog table with its `rows exported` line for all 177 logs, with no log line left over (T-3b adds the partition
  lines). The failed tables of the `ru_exp_*` files (`EXP-00056` / `EXP-00008`) are in the catalog with 0 rows, as X2 says.
  The checksum test counts 313 listed files, 233 dumps (the 22 files of session 9 and their logs are listed).
- **Expected values** (`tests/data/gen_walk_expected.py`, run from `docs/formats/tools` with `uv run`): three TSV files from
  `exp_parse.py` (`walk_expected.tsv` per file, `walk_parts.tsv` per binary part with offsets, `lob_expected.tsv` per LOB
  table); the generator asserts the plan totals, so it fails if the tools change. The `plan`, `stop_*` and `pre_*`
  columns are gone.
- **Answer (Architect, session 4), point by point** (owner's answers: ADR-23):
  - API changes (`Section.parts`, `Part`, `TableHeader` with m and entries, `RowsSummary`, `RowCursor::step` with `Env`,
    `from_columns`, `WalkReport::parts` / `lob_totals` / `stopped_section`): accepted. T-3c adds the partition and
    subpartition lines to `Part` and `LongStats` per LONG column (arch 3.2).
  - `UnsupportedLayout` stops the file like `UnsupportedType`; a path flag other than 0 / 1 is `UnsupportedLayout`:
    accepted (owner). The order of the checks: accepted.
  - [R8] in column fields, `fc ff` in a LONG column, `ff ff` inside a row as `Framing`: accepted. T-3c adds: a
    conventional LONG / LONG RAW u16 from 0x8000 to 0xffef is `UnsupportedLayout` (R-040); with flag 1 a LONG field
    is a piece chain (ADR-15 session 4 item 1).
  - LOB value area: the k check, short values 0 to 0xffef, the `Framing` cases: accepted (whether L4 should list the
    first-u16 cases is note A-13 to the Analyst, still open). **T = 0**: the owner changed it: silent only if a
    fixture holds one, else `W-UNTESTED-LAYOUT` at the `fc ff` (strict: error); see AQ-1. LOB hashes: T-3b.
  - Partitions: your reading is the design (N-11 below; ADR-21 session 4 item 3); T-3c extends it to
    `COMPARTITION` / `SUBPARTITION` / `ENDCOMPARTITION` (ADR-21 items 1 to 5).
  - L1 warnings once per section: accepted.
  - [E2a] as built, `strict_e2a` a placeholder until T-11: accepted (owner). Session 4: the case is narrowed to a
    single-column LONG table (ADR-22); add `COMPARTITION "`, `SUBPARTITION "`, `ENDCOMPARTITION` to the known kinds.
  - `W-LONG-DIRECT` raised by the walker: accepted (owner) for T-3a; **removed in T-3c**, because rules v3.4 confirm
    the direct-path form [R2, Y9] (ADR-15 session 4 item 4). `W-LONG-CHARSET` and `W-LONG-LENGTH` are T-6's; the first
    only for a database set other than AL32UTF8.
  - ADR-17 checks, no pairing warning for `Mode::Other`: accepted.
  - `TableInfo` = first part until T-3b; `Catalog.stopped_table` with the name; the error with the ordinal only:
    accepted (the survey prints the name from the catalog).
  - `Dump::warnings` duplicate: noted, thank you.
  - Golden files: T-3b, now after T-3c and for 199 files.
  - The generator of expected values asserting the totals: accepted; T-3c regenerates it for 199 files.

### N-10 The 22 files of the Analyst's session 9 (not written up): what the walker makes of them
Walked, nothing compared (`x_files_not_yet_written_up_are_walked_without_a_panic`). Outcomes on 2026-10-15:
- Complete, no warning: `lob3`, `long2` (conventional and `us7ascii`), `misc2`, `nchar2`, `t_num3`, `t_probe11` (both
  paths), both versions; `long2_exp_direct_<v>`: complete with `W-LONG-DIRECT` only.
- `long_exp_direct_<v>`: `E-TRUNCATED` (18c at 0xa328, 11g at 0xa329). The session 9 version of `exp_parse.py` says a LONG
  value on the direct path is a chain of pieces `u16 n + bytes` with bit 0x8000 of n meaning "another piece follows"
  (R-031). The walker reads each value as one ordinary field [Y9], which is right only for values below 0x8000 bytes
  (`long2_exp_direct` happens to hold such values). Proposal: when the Analyst writes the rule (v3.4), the Architect adds
  "path flag 1 and a LONG / LONG RAW column: pieces" to arch 5.4 / ADR-15 and T-3a's follow-up; until then the `W-LONG-DIRECT`
  warning marks every such table.
- `part2_exp_<v>` (18c): `E-FRAMING` "second INSERT INTO in one table section without a PARTITION line": the file has
  composite partitions (`COMPARTITION "p"`, `SUBPARTITION "s"` lines, R-037), which rules v3.3 do not know. Proposal:
  after the Analyst's write-up, accept these two line kinds like `PARTITION "p"` (name = subpartition name) in the
  walker and in ADR-21.
- **Answer (Architect, session 4): both written up in rules v3.4 and designed.** `long_exp_direct_<v>`: [R2, Y9] confirm
  the piece chain; ADR-15 session 4 item 1 and arch 2.5 step 5 give the reading, including the u16 cases the fixtures
  do not show; T-3c item 1. `part2_exp_<v>`: [T10] confirms `COMPARTITION "p"`, `SUBPARTITION "s"` + `INSERT INTO` +
  part, `ENDCOMPARTITION`, statistics items before `ENDPARTITION`; your proposal is adopted with one difference: a part
  carries both names (`PartInfo.partition` from the enclosing `COMPARTITION`, `PartInfo.subpartition` from the
  `SUBPARTITION` line); ADR-21 session 4; T-3c item 4. `long2_exp_direct_<v>` loses its `W-LONG-DIRECT` in T-3c.

### N-11 Points for the Architect
- arch 2.5 step 4 and ADR-21 contradict each other as quoted above; my reading is in N-9. Please confirm.
- `TableInfo` has no `parts` yet; a caller that reads T_PART through `Dump::rows` sees the first partition only until T-3b.
- Rule L4 allows a long form with T = 0 (no chunk); nothing in the rules says whether `exp` can write it. The walker
  accepts it.
- **Answer (Architect, session 4):**
  - arch 2.5 step 4 and ADR-21 do not contradict each other once both sentences are read as one rule, which is
    your reading: a further `INSERT INTO` is `Framing` when no naming line was seen in the section; when one was
    seen but does not directly precede it, the part is read, named after the last one, with `W-UNTESTED-LAYOUT`.
    The owner accepted it (ADR-23 item 1); arch 2.5 step 4 and ADR-21 session 4 item 3 now say it in these words,
    with `SUBPARTITION` inside `COMPARTITION` as a naming line too.
  - `TableInfo.parts`: T-3b (after T-3c).
  - T = 0: ADR-23 (owner): `W-UNTESTED-LAYOUT` unless a fixture holds one; AQ-1 asks you which.

## T-4 (Implementer, 2026-10-16)

### N-12 Choices made in T-4 (please confirm or correct)
- **New error codes** (value class, no context; the caller adds offset / table / column / row): `E-BAD-NUMBER`,
  `E-NUMBER-OVERFLOW` (new name: an integer part that does not fit under every policy, or a fraction beyond the
  scale under policy `error`), `E-BAD-DATETIME` (new name for what arch 5.3 / 5.4 call `ValueDecodeError`: year
  below 1, nanoseconds beyond 64 bits, impossible fields, wrong length; Python maps it to `ValueDecodeError`),
  `E-BAD-INTERVAL`, `E-BAD-ROWID`, `E-BAD-TEXT` (LONG text not UTF-8), `E-LOB-TOO-LARGE` (`LongBuffer`). No new
  warning code: `W-UNVERIFIED-TYPE` (ROWID prefix, 7-byte TSLTZ) is a flag in the return value
  (`Rowid::prefix_known`, the `bool` of `decode_tsltz`); T-6 / T-11 raise the warning.
- **NUMBER**: `OraNumber` has `len = 0` for zero instead of a separate flag (same information). Decimal types take
  `precision: u8` and `scale: u8` (scale >= 0; negative-scale columns are Int64 / Decimal128(p - s, 0) by arch 5.1,
  decided in T-5). `Fitted::Rounded` carries `to_zero` and `dropped` (digits beyond the scale) for
  `W-NUMBER-ROUNDED`; policy `Null` gives `Fitted::Nulled`. Integer overflow is checked first, so it is an error
  under all three policies, also when the carry of a rounding adds a digit (99999.5 at NUMBER(5,0)).
  Float64 and text allocate a `String` per value; a faster path is a T-15 matter.
- **DATE** [Y2]: the century / year bytes are read as positive when both are >= 100, as BC when both are in 1..=100
  (so century 0 BC years such as -50 would read `64 32`; no fixture), anything else is `E-BAD-DATETIME`. For a
  negative year the day is only checked against 31 (Oracle's pre-1582 calendar is not modelled; arch 5.3 makes the
  Arrow value an error anyway). Time bytes are hour + 1, minute + 1, second + 1; a byte of 0 is an error.
- **TSTZ** [Y7]: an offset zone (bit 0x80 clear) is accepted when hour is -23..23 and minute -59..59 (my bound; the rule
  gives none); a region zone is returned as the two raw bytes (`Zone::Region`), the instant is the stored UTC.
- **INTERVAL** [Y10], ADR-19: years / days must be within +-999,999,999 (the leading precision 9) besides the
  checks of ADR-19; the sign check covers all fields (a zero beside a negative is allowed).
- **UROWID** logical: an empty tail (`02` alone) is `E-BAD-ROWID` (the rule says the tail is a multiple of 3 bytes;
  zero bytes would give the text `*`, which no fixture shows).
- `values::long` holds only what is pure: `LongBuffer` (assembly of pieces under `max_lob_bytes`), the 32,000
  limit, `has_high_bytes`, `decode_utf8`. The piece-header rules (first u16 `fe ff`, 0, 0xfff0 ..., pieces of length 0)
  are the walker's (T-3c); the charset of LONG is T-6 / T-9 (`charset` does not exist yet).
- Dependencies added: `arrow-buffer = "60"` (for `i256`; `from_i128`, `checked_mul`, `checked_add`, `checked_neg`,
  `from_string` exist on 60.0.0), dev `proptest`. `Decimal256Builder` is not used in T-4 (T-6).

- **Answer (Architect, session 5): accepted, all points** (ADR-27), with one addition: `W-UNVERIFIED-TYPE`
  becomes a `WarningCode` that the row decoder of T-6 raises from your flags, once per column and table;
  the decoders stay pure. The DATE pair rule is written out in ADR-27 item 6 ((100, 100) = year 0 is an
  error; (100, y < 100) is a year of century 0 BC). The two type consequences are ADR-28: the 4 YEAR(9)
  values beyond 32-bit months **stay** `E-BAD-INTERVAL`; years below 1 **stay** `E-BAD-DATETIME` (R-044
  asks for the year numbering; Python `datetime` cannot hold them anyway).

### N-13 The walker delivers no field bytes; T-4 reads fields with its own reader
- Seen: `RowCursor::step` skips every value by length; nothing in the public API returns field bytes before the
  `RowSource` seam of T-6. The plan's check "all 85,456 NUMBER fields of 199 files decode and re-encode" needs bytes.
- Done: `tests/t4_values.rs` has a 40-line test-only field reader (rows from the tool's `rows_off`, u16 lengths, direct-path
  LONG chains, the LOB value area skipped by lengths) driven by `tests/data/value_parts.tsv`, which
  `tests/data/gen_number_parts.py` writes from `tools/exp_parse.py` (396 parts, 199 files, 85,456 NUMBER fields, a SHA-256
  over them per part). The reader does not depend on T-3a / T-3c / T-3b state (offsets come from the tool), so it works on
  all 199 files already. When T-6 lands, the walker should be tested against the same TSV.
- Side result: the same pass decodes every DATE, TIMESTAMP, TSTZ, TSLTZ, INTERVAL and ROWID / UROWID field of the 199
  files; the only value errors are the documented ones (year below 1 in DATE; 4 INTERVAL YEAR(9) values beyond 32 bits of
  months in the two `misc2` files).

- **Answer (Architect, session 5): accepted.** T-6's done-when now says the row decoder reproduces the
  per-part digests of `value_parts.tsv`, so your test-only reader is checked against the real one.

### N-14 The Analyst's `tools/num_decode.py` stops at its step 3
- Seen: `uv run docs/formats/tools/num_decode.py <three folders>` prints steps 1 and 2 (120 values, as the plan says) and then
  stops with `KeyError: 0` at `w = want[ci]` (line 195), before "3. NUMBER fields in ... files". Cause not investigated (`want` has no
  entry 0 for some table). Not my file; I counted the 85,456 NUMBER
  fields with `exp_parse.parse` directly (generator above). Proposal: the Analyst fixes step 3.

- **Answer (Architect, session 5):** sent to the Analyst as note A-24 (`open-questions.md`). Your count from
  `exp_parse.parse` stands as the plan's figure meanwhile.

## T-3c (Implementer, 2026-10-16)

Built: the walker of rules v3.4. All 199 files walk to the second `EXIT`, no stop, no warning at all; the numbers of the
199-file table (382 sections, 396 parts, 40,588 rows, 238,657 fields, 19,186 NULL fields, 777 text fields, 24 / 17 empty
parts, 19 LOB sections, 114 LOB values: 74 short, 40 long) come out of the walker (`t1_walk.rs`, `t3a_walker.rs`).
Tests: `t3c_walker.rs` (new, 11 tests), adapted `t1_walk.rs`, `t3a_walker.rs`.

### N-15 Choices made in T-3c (please confirm or correct)
- **`Warning` has `column` and `row`** (new `in_column`, `in_row`; they are in the rendered text). The piece-of-length-0 and the
  T = 0 warnings need them (task rule: warnings carry column and row). `WarningCode::LongDirect` is removed (ADR-15), not just
  unused.
- **`LongStats`** (`exp::rows`): per LONG / LONG RAW column of a part, in `RowsSummary.long` (empty for a table without one):
  `column` (1-based, INSERT INTO order), `values` (non-NULL), `chained` (values of more than one piece), `largest`, `above_32000`
  (the limit is `LONG_LIMIT_BYTES`), `zero_pieces` (pieces of length 0, counted as pieces) and `first_zero_piece` (row, offset).
  Kept for both paths (a conventional value is one field: `chained` 0). T-3b adds the per-column sum over parts for
  `ColumnInfo.long`.
- **Chain reading** (`read_chain`): first u16 by [R8] before the chain is entered (`fe ff` NULL; 0, 0xfff0 to 0xfffd, `ff ff`
  `Framing` with row, column, length); later u16 from 0xfff0: `Framing` at that u16; a piece of length 0 (`00 80`, or a final
  `00 00`) read by the formula. A first u16 of `00 80` is a piece of length 0 with more to follow (warned), not `Framing`;
  that is my reading of "otherwise a piece" in ADR-15 item 1.
- **Hash of a chain** (`hash_fields`, T-2 serialisation extended): `fd ff ff ff`, then per piece its length without bit 0x8000 as
  u32 LE and the bytes, then `fc ff ff ff`. Nothing in the old golden file has a direct-path LONG, so no line changes; T-3b's
  generator may replace this by the assembled value, then `RawHasher` follows it.
- **Warnings once**: a piece of length 0 once per LONG column and section (offset of the first); a long LOB value of total 0 once
  per section (offset of the first `fc ff`, with row, column and length 0); a part not directly preceded by a naming line stays
  per part (as in T-3a).
- **Partition lines** (ADR-21 items 1 to 5, built as written): the five lines are recognised before list K and G1. `Part` has
  `partition` (the `PARTITION` line, or the enclosing `COMPARTITION` line) and `subpartition`; `Section.partitioned` = a naming
  line or a `COMPARTITION` was seen, new `Section.subpartitioned` = a `COMPARTITION` was seen. The once-per-section
  `W-UNTESTED-LAYOUT` covers all of item 4 plus two cases I added so that nothing is silent: a section that ends with a
  `COMPARTITION` still open, and a section with only `COMPARTITION` lines that ends without `ENDPARTITION` (item 4 says "naming
  lines"; `COMPARTITION` alone is not one). `PARTITION` while a `COMPARTITION` is open is covered by "PARTITION and COMPARTITION
  in one section". A `SUBPARTITION` outside a `COMPARTITION` is no naming line and does not change the part's names.
- **Conventional LONG from 0x8000**: `UnsupportedLayout` at the field with table, row, column and length; any LONG / LONG RAW
  column on flag 0, u16 0x8000 to 0xffef (0x7fff and below are plain lengths; 0xfff0 to 0xfffd stay `Framing`). The check is on
  the u16 only, it does not look at the neighbours (as ADR-12 says).
- **[E2a] known kinds** now include `COMPARTITION "`, `SUBPARTITION "` and `ENDCOMPARTITION` (unit test `known_item_lines`).
- **Expected values**: `gen_walk_expected.py` covers all 199 files, asserts the 199-file totals, and has new columns:
  `subpartition`, `long` (per LONG column `column:values:chained:largest:above32000`) in `walk_parts.tsv`, `lob_zero_total` in
  `walk_expected.tsv`. `t2_manifest.rs` and `t3_catalog.rs` still compare the golden file of 177 files (the files they
  cannot find in it are not compared); they are T-3b's to move to 199 / 396.

- **Answer (Architect, session 5): accepted, all points** (ADR-29), with these precisions: (1) `column` is the
  1-based file position everywhere, `row` is 1-based within the part, and T-3b adds `part` (1-based, set
  with `row`); the DDL warnings of `types::resolve` follow the same convention; (2) the chain hash is
  **final** for the raw manifest; T-3b adds a second digest per LONG column over the assembled values
  (ADR-29 item 6); (3) the two extra partition cases join ADR-21 item 4; (4) `strict` for all of them is
  T-11.

### Answer to AQ-1 (T = 0)
0. No long LOB value with a total of 0 in the 199 files: the walker (`RowsSummary.lob_zero_total`, 0 parts) and the tool
(`lob_zero_total` column of `walk_expected.tsv`, computed from `exp_parse.py`'s `lob_values`, 0 in every file) agree
(`x_the_199_fixtures_give_no_stop_no_framing_error_and_no_warning`). ADR-23 stands: the walker warns `W-UNTESTED-LAYOUT` at the
`fc ff` (test: a synthetic value made from the first long value of T_LOB in `lob_exp_18c.dmp`).

### Points for the Architect
- Plan T-3c (c) says the 6 parts have "21, 29, 0, 0, 21, 29" rows "named as the `SP` lines". The `SP` lines of `truth_part2.txt`
  are sorted by name (P1, P1, P3, P3, P_EMPTY, P_EMPTY), the file is in partition order (P1, P_EMPTY, P3). The test compares the
  six (partition, subpartition) pairs as sets, and the rows of each part with the `ROW|` lines of its subpartition
  (21, 29, 21, 29, 0, 0 in `SP` order = 21, 29, 0, 0, 21, 29 in file order).
- `strict` for T-3c warnings (a zero-length piece, T = 0): not wired, like `strict_e2a`; T-11.
- Nothing in the walker bounds a chain's total (it skips by length, nothing is buffered); `max_lob_bytes` is T-6's.

- **Answer (Architect, session 5):** the set comparison of the `part2` parts is accepted (ADR-29 item 9); the
  rest as above.

## T-5 (Implementer, 2026-10-16)

### N-16 Choices made in T-5 (please confirm or correct)
- Module `oradump_core::types` (`declared`, `map`, `resolve`); new dependency `arrow-schema` 60 (`DataType`); `Options.types:
  TypeOptions` (`number_default`, `number_types`, `timestamp_unit`); `ColumnInfo` gains `file_position`, `declared`,
  `declared_text`, `not_null`, `arrow_type`; `TableInfo` gains `ddl_fallback`; `TableInfo.columns` is in declared order
  (ADR-24) and `Dump::rows` reads the fields in `file_position` order. New warning codes `W-DDL-FALLBACK` and
  `W-CHARSET-UNSUPPORTED` (the CLOB of architecture 5.1 is the only user until T-9). Warnings of the resolution go
  to `Dump::warnings()` during `scan`.
- **Where an unknown type text ends** (architecture 5.5 step 4 says "the text is kept", rule T9 does not say where such a
  text ends). The parser takes it up to the first `, "` or `)  `. A type text that itself contains `, "` or `)  `
  would be cut wrongly; for `FOO(3)  ` as the last definition the kept text is `FOO(3` (the closing parenthesis is
  taken as the end of the list). Only the kept text is affected; the other columns and the column count stay right
  unless the text holds `, "`. Proposal: accept (not in the fixtures); the Analyst may add the rule if the owner's
  dump shows such a type.
- **Out-of-range parameters are a column fallback**: `NUMBER(p, s)` with p outside 1..=38 or s outside -84..=127,
  `FLOAT(b)` outside 1..=126, `TIMESTAMP (f)` and interval precisions above 9. Not in rules or ADR; chosen because
  Oracle does not allow them, so the text is wrong for this file.
- **`NUMBER` that no Decimal128 can hold is read "as `NUMBER`"** without a warning, as the table of 5.1 says for
  `p - s > 38`; I applied the same to a scale above 38 (`NUMBER(*,40)`, `NUMBER(3,50)`), which the table does not
  list. Proposal: the Architect adds the row, or asks for a warning.
- **Form check in the agreement**: VARCHAR2 and CHAR need descriptor form 1, NVARCHAR2 and NCHAR form 2, CLOB form 1, NCLOB
  form 2 (arch 5.5 step 3 names the type code only; the form is in T5 and C3/C5). No fixture disagrees (199 files).
  TSTZ length 13 and TSLTZ 7/11 are the lengths of `collist.py` (arch 5.5 gives only DATE, TIMESTAMP (0)/(f>0)).
- **Order warning**: `W-UNTESTED-LAYOUT` (once per table) unless the file order is the declared order or, on the
  direct path, the declared order with the LONG / LONG RAW columns moved last. A LONG-last order on the conventional
  path therefore warns. Two `INSERT INTO` names that are equal, or a name that matches no or two definitions, is a
  table fallback.
- **`number_types` applies to every type-2 column** (declared precision included, FLOAT included), because the
  architecture lists `int64` for `NUMBER(*,0)`. `number_default` applies only to a plain `NUMBER`, a `NUMBER` that no
  Decimal128 holds, and the fallbacks. The overflow policy (`round` / `error` / `null`) is not part of T-5 (T-4 renders,
  T-11 wires); the metadata key `oradump:number_overflow` is T-6's.
- **CLOB set check** in the type mapping: descriptor form 1 and a set id other than 873, 171, 178, 1 gives LargeBinary and
  `W-CHARSET-UNSUPPORTED` (arch 5.1). NCLOB and form 2 are LargeUtf8 whatever the id.
- Tests of the six "moved" tables, of `Dump::rows` order and of the patched files are in `tests/t5_types.rs`; T-3b still
  has to give `TableInfo.parts`: until then the types are resolved on the first part.

- **Answer (Architect, session 5), point by point** (ADR-30, ADR-31):
  - Unknown type text: accepted (first `, "` or `)  `), because the agreement check turns a wrong cut into a
    table fallback; the forms a real dump may hold are listed in ADR-30 item 1. Please add its three test
    vectors; by your code `"X" FOO(3))  ` keeps `FOO(3)`, not `FOO(3` as the note says.
  - Out-of-range parameters as a column fallback: confirmed.
  - **Changed**: a declared `NUMBER` that no Decimal128 holds (s > 38; s < 0 with p - s > 38; `NUMBER(*,s)`
    with s < 0) takes `number_default` **with** `W-DDL-FALLBACK`; arch 5.1 has the row (ADR-30 item 3). This
    replaces the 5.1 text "s < 0: Decimal128(38, 0)" for `NUMBER(*,s)`.
  - Form check, TSTZ / TSLTZ lengths, order warning, CLOB set check: confirmed.
  - `number_types` over every type-2 column, `number_default` for plain `NUMBER`, fallbacks and the new
    case: confirmed; that is what Q-1 and ADR-14 mean.
  - Types resolved on the first part: T-3b runs the matching per part and fills `PartInfo.file_order`
    (ADR-31); the types stay resolved once per table.

### N-17 Choices made in T-3b (please confirm or correct)
- **Where the per-part check lives.** The walker (`exp::items`, `TableHeader::same_columns_by_name`) does the check of ADR-31
  item 2 at the `INSERT INTO` of each later part: same column count, same `m`, and the sorted set of (name, type code, length, set id,
  form, LOB column entry name and tail) equal to the first part's; else `Framing` with `part`. The **path flag is not compared** any more
  ([T10]: each part has its own flag); `TableHeader::same_layout` (path and order included) is kept for the tests that use it, the walker
  no longer calls it. `resolve::part_order` then finds `file_order` by name for each later part and raises the order warning.
- **Offsets of the order warning.** The first part's warning (ADR-24, raised in `resolve_table`) stands at the `CREATE TABLE` line and has
  `part` 1; a later part's stands at its `INSERT INTO` line with its `part`. Once per table.
- **Equal names in the first part** (table fallback, ADR-24 item 2): a name cannot identify a field, so later parts get the identity
  order (position by position). The walker's check still compares them as sets. No fixture has this; a part with equal names is already a
  `W-DDL-FALLBACK` table.
- **LONG-last exemption of a later part** is computed from the first part's descriptors (type codes 8 and 24), so it works on a table
  fallback as well; on a conventional part it warns (ADR-30 item 5). Unit tests in `types/resolve.rs` (`part_order_tests`), because no
  fixture has a partitioned table with a LONG column.
- **`part` is set on every error and warning raised while a part is read** (header, rows), not only when `row` is set; errors of
  `TableRows::next_row` carry the part of the cursor. Errors and warnings about the section as a whole (partition lines, DDL fallback of
  the table, the `INSERT INTO outside a section` case) have no part, except the second `INSERT INTO` without a naming line, which names the
  part it would have been.
- **`column` of the DDL warnings** (`W-DDL-FALLBACK` of a column, `W-CHARSET-UNSUPPORTED`) is now the file position of the first part.
- **API of the catalog.** `TableInfo.path` is a new `TablePath { Conventional, Direct, Mixed }`; `rows_offset`, `rows_end_offset`,
  `end_marker`, `insert_offset` moved to `PartInfo` (arch 3.2 session 3). `Dump::rows(which)` is the cursor of the **first part**,
  `Dump::part_rows(which, index)` the cursor of any part; the cursor lists its columns in the part's file order (`file_order`).
  `PartInfo.partition` / `subpartition` are `Option<String>` (ASCII decoding) with `*_raw` bytes; there is no damaged flag yet (T-9).
- **`ColumnInfo.lob`** needed per-column counts, which the walker did not keep: `RowsSummary.lob_columns` (new). `LobStats` is an alias of
  `LobSummary`. **`ColumnInfo.long`** is the sum of the `LongStats` of the parts found through each part's `file_order`; its `column` is the
  declared position and `first_zero_piece` that of the first part that has one. Unit test with a synthetic two-part section in
  `dump.rs`, because no fixture has a partitioned table with a LONG or LOB column.
- **The new digests** (hashing mode only, `WalkOptions.hash_fields`): `RowsSummary.long_sha256` (assembled values, serialisation of ADR-29
  item 6) and `RowsSummary.lob_sha256`. The serialisation of the LOB digest is not fixed by the ADR; chosen: per LOB value in file order
  its stored length as u64 little-endian, then its bytes (chunks joined). A direct-path chain is read twice in hashing mode (the total
  comes first in the digest and is known only at the end): the walker seeks back to the first piece, hashes the pieces, and seeks forward.
- **Golden files.** `exp_raw.tsv` has 16 columns (`part`, `partition`, `subpart`, `lob_values`, `lob_bytes`, `lob_sha256`, `long` added);
  `exp_catalog.tsv` has one line per section (382) with a `parts` column that holds the offsets of every part; the X1 filter of the
  generators is gone (they assert that every type code is one of the 16 of T5). The generators are run with `python3 -I` from
  `docs/formats/tools`. The chain serialisation of the raw digest is reproduced in `gen_exp_raw.py` from `t["long_pieces"]`.
- **The log check** reads `exporting partition`, `exporting composite partition` (no count) and `exporting subpartition` lines; for a
  partitioned table the leaves (partitions, or the subpartitions of a composite partition) must equal the catalog's parts in name, order
  and rows. T_PART: 3 lines in each of 2 files, T_PART2: 6 in each of 2 files; 199 logs, none left over.

- **Answer (Architect, session 6), point by point** (ADR-32):
  - LOB digest: accepted and **final** (per part, values in file order, stored length u64 LE + stored bytes, chunk
    length fields excluded; NULL adds nothing, empty adds 8 zero bytes). Assembled-LONG digest: as ADR-29 item 6,
    confirmed; the seek-back in hashing mode is accepted (tests only).
  - **Changed**: `Dump::rows` on a table with more than one part is `E-CONFIG` (`length` = number of parts), not the
    first part; `part_rows` with an index outside the parts is `E-CONFIG`, not `E-NO-SUCH-TABLE`. Both stay Rust-only
    diagnostic cursors; `read_table` is the reading API. Small fix, first step of T-6L (or of the T-5 follow-up).
  - Order warning of a later part (once per table, offsets as you built them): confirmed.
  - Fixture gap: Oracle refuses a partitioned table with a LONG column (ORA-14085), so no fixture can have one; your
    unit tests stay the test; R-045 (low) records the refusal. Partitioned LOB tables: R-042, medium (high if Q-4
    shows them); T-6L adds a synthetic two-part LOB section to its tests.
  - Everything else (walker check without the path flag, identity order on equal names, `part` placement, DDL
    warning column, `TablePath::Mixed`, partition names ASCII + raw until T-9, `ColumnInfo.long` sum, log check):
    accepted.

### N-18 Choices made in T-6 (please confirm or correct)
- **The seam for decoding is `RowSink`** (`exp::rows`, format-neutral): `RowCursor::step_with(stream, hasher, sink, env)` lends every
  column field (`row_start`, `null`, `value`, `chain_piece(first, last)`, `row_end`) to a sink; the scan still uses `step` (no sink).
  With a sink the bytes are not hashed (sink and hasher are for different jobs). The LOB value area is not given to the sink (T-6L).
  `TableRows::next_row_with(sink)` exposes the same for tests; `tests/t6_value_parts.rs` uses it for the N-13 check (396 parts,
  85,456 NUMBER fields, hashes equal `value_parts.tsv`). `column` in the sink and in every error and warning is the **file position**
  (ADR-29); the offset of a value is the offset of its first byte (of a piece for a chain).
- **Where the value options live.** Architecture 3.2 puts `on_value_error`, `on_number_overflow`, `max_lob_bytes` into `Options`; I put
  them into `reader::ReadOptions` (with `columns`, `batch_rows`, `batch_bytes`) so that one `read_table` call carries everything the
  reader needs and the options can differ per read without re-opening. `Options` keeps `types` (Arrow types). Proposal: keep, and say so
  in 3.2; moving them is mechanical. Defaults as designed: `Raise`, `Round`, 256 MiB, 65,536 rows, 64 MiB.
- **`Warning` has two new fields**, `count` and `count_to_zero`, for the once-per-column-and-table warnings of ADR-27 item 2
  (`W-NUMBER-ROUNDED`, `W-NUMBER-NULLED`, `W-VALUE-NULLED`, `W-UNVERIFIED-TYPE`, `W-LONG-LENGTH`, `W-LONG-CHARSET`,
  `W-LONGER-THAN-DECLARED`): part, row, column and offset are those of the first value, `count` is the number of values of the column in
  the table. Not carried: "the largest number of digits dropped" of arch 5.2a (3) (the decoder returns it; the `Warning` has no field
  for it). They are emitted when the reader returns `None` and live in `TableBatches::warnings()`, not in `Dump::warnings()`. New
  codes: `UnverifiedType`, `LongLength`, `LongCharset`, `LongerThanDeclared`, `NumberRounded`, `NumberNulled`, `ValueNulled`,
  `CharsetMismatch` (`W-CHARSET-MISMATCH`, arch 6.1, raised once per column at the start of the read) and the error code `NoSuchColumn`.
- **Which overflow policy applies.** The option applies to a type-2 column that is not a declared `NUMBER(p,s)` whose Arrow type differs from
  `number_default`, or that has a `number_types` entry (key `TABLE.COLUMN`); every other NUMBER column is `error` ("a declared column
  stays exact or value error", 5.2a). A declared `NUMBER(38,10)` therefore counts as unconstrained (its type equals the default); I did not
  find a cleaner test with the data `ColumnInfo` carries. The per-column policy of `number_types` ("optionally an overflow policy") does
  not exist in `TypeOptions` (T-5 stores only the type); not built.
- **`batch_bytes` in the plan.** T-6 says "`batch_bytes` 16 KiB forces one row per batch without splitting a value" on T_LONG. By the rule of
  5.6 step 8 (the batch closes after the row that reaches the cap) T_LONG rows add 14, 9 and 32,009 bytes, so 16 KiB gives **one** batch of
  three rows; the cap 14 gives batches of 1 and 2 rows; the cap 1 gives three batches. The test records the rule, not the sentence.
- **Not built in T-6** (not in its "Done when", or owned by another task): LOB columns (selecting one is `E-CONFIG`, T-6L; the other columns
  of such a table read, the locators are skipped); `W-TSLTZ` (type resolution, the scan); `strict`; `trim_char`; `db_charset` /
  `client_charset` overrides; an unsupported set id of a type 1 / 96 column is `E-CONFIG` ("task T-9"), not `W-CHARSET-UNSUPPORTED` + bytes
  (that is for CLOB only in 5.1); `interval_as`; `timestamp_unit` is T-5's. The metadata keys written: `oradump:type_code`, `oracle_type`,
  `declared`, `max_bytes`, `charset` (text columns), `number_overflow` and `number_scale` (NUMBER columns that are not Float64),
  `conversion` = `nearest-double` (Float64 of a NUMBER / FLOAT column).
- **A part whose row count differs from the scan** (the cursor reached the end marker after another number of rows than `PartInfo.num_rows`)
  is `E-FILE-CHANGED` with part and length; the scan and the read use the same cursor, so this can only mean a changed file.
- **LONG under `on_value_error="null"`**: a failure in a piece (a value above `max_lob_bytes`, a broken UTF-8 sequence) marks the value
  failed, the remaining pieces are passed over by length, one NULL is appended at the last piece and `W-VALUE-NULLED` counts one; bytes
  already written stay under the null slot (arch 5.6 step 6). A LONG in a database whose set is single-byte and a US7ASCII database go
  through `Charset::decode` per piece (stateless); AL32UTF8 goes through the streaming `encoding_rs` decoder.
- **For ADR-18 (arch 4.1 (e)), the check the plan asks T-6 to record**: on arrow-array 60.0.0 `GenericStringBuilder` implements
  `std::fmt::Write` and `GenericBinaryBuilder` `std::io::Write`; written bytes accumulate into the next value, `append_value("")` /
  `append_value(b"")` ends it, and `append_null()` after a partial write keeps the partial bytes under the null slot (offsets `[0,6,7,9]`
  in `arrow_out::tests`). I cannot edit `decisions.md`; please add this to ADR-18.
- **The I/O buffer** of a reader is at least 128 KiB whatever `Options.buffer_bytes` says: a field is up to 65,519 bytes and a piece 32,767
  and `ByteStream::read_bytes` needs the whole value in the buffer.
- **Partition requirement (owner, T-6).** "part2 must give 100 rows" is read as the table `T_PART2` of `part2_exp_18c.dmp` /
  `part2_exp_direct_18c.dmp` (`truth_part2.txt`, `part2_check.py`: 100 rows, 6 parts of 21, 29, 0, 0, 21, 29). `part2` is the name of the
  fixture family: no partition or subpartition has that name (the partitions are `P1`, `P_EMPTY`, `P3`, subpartitions `SYS_SUBP368` to
  `373`). The format documents support the requirement cleanly: partition boundaries are not needed to read (the parts are delimited by
  their `INSERT INTO` lines and `ff ff`), only to check the data, and T_PART's boundaries are in the DDL / SQL
  (`corpus/sql/01b_optional_tables.sql`), not in the dump's binary parts; `tests/t6_parts.rs` checks every date against them. An empty
  part yields no batch (a batch never mixes parts, and an empty part has no rows).
- **UROWID text in `truth_misc.txt`**: the line holds the text only for the ROWID column; the UROWID column is the DUMP of its 13 bytes
  (the physical UROWID equals the ROWID of the row, [Y11]), so the test compares the UROWID text with the ROWID text of the same line;
  `truth_misc2.txt` has both texts.

- **Answer (Architect, session 6), point by point** (ADR-33; ADR-18 for the arrow check):
  - `RowSink` seam: accepted as the push form of `RowSource::next_field`; T-6L adds `lob_start` / `lob_chunk` /
    `lob_end` (plan T-6L).
  - Value options in `ReadOptions`: **confirmed**; arch 3.2 now lists the split (`unconfirmed_lob` and `trim_char`
    join `ReadOptions`; `strict`, set overrides, `types`, `interval_as` stay in `Options`). Python keeps `open()`
    kwargs as defaults and lets `read_table()` override the per-read ones.
  - Counted warnings in `TableBatches::warnings()`, not in `Dump::warnings()`: **confirmed**, with one addition:
    they are also flushed when the read ends with an error. Python issues them at the end of iteration (ADR-33
    item 3).
  - Overflow policy selection: **changed**. Your type comparison misclassifies a declared `NUMBER(38,10)`. The T-5
    follow-up adds `ColumnInfo.type_source` (`Declared` / `Default` / `Override`); the policy applies to `Default`
    and `Override` only (ADR-33 item 4).
  - Per-column policy and "largest number of digits dropped": both in the alpha, built in **T-11**
    (`number_types` entry policy; `Warning.max_digits_dropped`).
  - `batch_bytes`: you are right; the plan sentence is corrected (one batch of 3 rows at 16 KiB).
  - ADR-18 check recorded under ADR-18 (arrow-array 60.0.0).
  - `E-FILE-CHANGED` on a row-count mismatch, LONG under `null`, 128 KiB buffer floor, new codes, metadata keys,
    `T_PART2` = 100 rows, UROWID comparison: accepted. Unsupported set id of type 1 / 96 as `E-CONFIG`: accepted as
    interim until T-9 (arch 6.4).

## T-6L (Implementer, 2026-10-16)

### N-19 Choices made in T-6L (please confirm or correct)
- **`RowSink::lob_start` has a fourth parameter, the form.** The plan names `lob_start(column, offset, total)`. ADR-25 treats the
  long form of a CLOB differently from the short one, and "the reader never uses the switch to decide anything" (arch 5.6 step 4),
  so `total` alone cannot say which form it is. Built: `lob_start(column, offset, form: LobForm, total) -> Result<LobAction>`
  (`LobAction::Read | Skip`), `lob_chunk(column, offset, bytes)`, `lob_end(column)`; the three have default bodies (skip every
  value) so the other sinks (the scan has none; `tests/t6_value_parts.rs`) are unchanged. `lob_end` is called only after `Read`.
  A sink that refuses a value (`E-LOB-UNCONFIRMED`, `E-LOB-TOO-LARGE`) returns the error, or under `on_value_error="null"` appends its
  NULL itself and answers `Skip`; the cursor then passes the chunks by their lengths, still checking each.
- **New codes**: `ErrorCode::LobUnconfirmed` (`E-LOB-UNCONFIRMED`) and `WarningCode::LobUntested` (`W-LOB-UNTESTED`, counted once per
  column and table with `count`, like the other value-layer warnings). New `ReadOptions.unconfirmed_lob: UnconfirmedLob`
  (`Error` default, `Decode`), as ADR-33 item 2 lists. The Python layer must map the new error code to `ValueDecodeError` (T-12).
- **Order of the two refusals**: a value that is both unconfirmed and above `max_lob_bytes` gives `E-LOB-UNCONFIRMED` (the more useful
  message) unless `unconfirmed_lob="decode"`, then `E-LOB-TOO-LARGE`. `max_lob_bytes` is compared with the **stored** length
  (h or T, bytes), before the first byte is read: a 200,000-byte UTF-16 CLOB of 100,000 characters is refused at 150,000.
- **Which values are "unconfirmed"** (header set id of [H4] in 1, 171, 178): NCLOB (form 2), every value; CLOB (form 1, descriptor set
  873, 171, 178 or 1), the long form only. A CLOB of descriptor set 873 in such a header (not seen in any file) is read as UTF-16 BE
  (type code and set id together, [L5]) and its long form is unconfirmed too. A CLOB of any other descriptor set stays LargeBinary
  (the type fixed at the scan, `W-CHARSET-UNSUPPORTED`) and is read as bytes, never unconfirmed.
- **Selecting a LOB column no longer is `E-CONFIG`.** `plan_column` returns `E-CONFIG` only for a type code it does not decode.
- **A LOB column that is not selected** is passed by its lengths: `lob_start` answers `Skip` and `RowCursor` calls `skip`. To make
  "no LOB byte is held" testable without `unsafe` (the workspace forbids it, so no counting allocator exists), `TableBatches` gained
  two read-only diagnostics, `lob_bytes_read()` (stored bytes lent to a builder) and `lob_values_skipped()`. The test asserts 0 and 8
  for `columns=["ID"]` on T_LOB and that the batches are below 64 KiB. Please confirm these two small public methods, or tell me to
  hide them behind a test-only feature.
- **Error offsets of a LOB value**: `E-LOB-UNCONFIRMED` / `E-LOB-TOO-LARGE` / `E-BAD-TEXT` at the first u16 of the value (chunk
  offset for a malformed chunk; the value's offset for an odd total or an unpaired surrogate found at the end), `length` = the stored
  length for the two refusals. Under `null` a value that fails in a later chunk keeps the text already written under the null slot
  (arch 5.6 step 6), and the remaining chunks are passed over.
- **The UTF-16 BE streaming decoder already existed** (`PieceDecoder` with `Charset::Utf16Be`, T-6, `encoding_rs`
  `decode_to_string_without_replacement`); T-6L added no code to `charset.rs`, only vectors (tests). Verified on the pinned
  encoding_rs: odd total, lone high or low surrogate, split pairs.
- **T-5f was not built** when this task ran (`type_source`, `W-TSLTZ`, flushing counted warnings on error are absent). T-6L did not need
  them; note that without the flush of ADR-33 item 3 a read that stops at `E-LOB-UNCONFIRMED` returns no counted warnings.
- **R_TEXT and the Cyrillic table around R_CLOB.** In the `ru_exp_*` files the order is R_TEXT, R_CLOB, then the Cyrillic table; the
  plan says "R_TEXT and the Cyrillic-named table after R_CLOB" and the test reads both (R_TEXT is compared field by field with the
  DUMP lines of `truth_ru.txt`, 309 rows, the file holds them in insertion order). The Cyrillic table has rows (`абв`, 1), (NULL, 2) in
  three files only (`ru_exp_al32utf8`, `ru_exp_cp1251`, `ru_exp_direct_cp1251`); in `ru_exp_direct_us7ascii`, `ru_exp_us7ascii`,
  `ru_exp_w1252` the export logged ORA-00942 on the table name (`???_???`) and the table has no rows; `ru_tables_exp_us7ascii` does not
  hold it (tables=R_TEXT,R_CLOB). The test encodes this from the logs.
- **Partitioned LOB table** (ADR-32 item 5): the synthetic section is built in memory from `lob_exp_18c` as the plan says (a `PARTITION
  "P1"` line, the T_LOB part, `PARTITION "P2"`, the part again, `ENDPARTITION`); the scan gives no warning, two batches, equal values,
  `ColumnInfo.lob` doubles every count.
- **Not built (outside T-6L)**: the scan's per-column count of the values that ADR-25 item 5 / arch 5.6 step 9 would refuse (survey,
  T-12); `strict` for `W-LOB-UNTESTED` (T-11); `W-CHARSET-UNSUPPORTED` stays the scan's.
- **Framing tests through the reader** cannot separate the scan from the read for a damaged chunk (the scan refuses the file first);
  the cursor code is the same, and `damaged_lob_files_never_panic_the_reader` runs 720 damaged variants of two LOB files through the
  reader with `raise`/`null`/`decode`.

- **Answer (Architect, session 7; ADR-34)**:
  - `lob_start` with `form`: **confirmed** (ADR-25 needs the form; `total` cannot tell it without a chunk-size
    table, [L4]).
  - `lob_bytes_read()` / `lob_values_skipped()`: **keep, public, `#[doc(hidden)]`**, doc comment "diagnostic, no
    API promise"; not bound in Python. Not a cargo feature: `required-features` would make the plain `cargo
    nextest run` skip the test. Add the attribute in T-9 (it touches `reader.rs`).
  - `E-LOB-UNCONFIRMED` before `E-LOB-TOO-LARGE`; `max_lob_bytes` on the stored length: **confirmed**.
  - New codes and `unconfirmed_lob`: **confirmed**; T-12 maps `E-LOB-UNCONFIRMED` to `ValueDecodeError`.
    `W-LOB-UNTESTED` is counted; under `strict` (T-11) the value becomes `E-LOB-UNCONFIRMED`.
  - Which values are unconfirmed: **confirmed**; a CLOB of set 873 in a single-byte header also gets
    `W-CHARSET-MISMATCH` (arch 6.1), built in T-9 (today only types 1 / 96 get it).
  - The scan's count of refused values: **T-11** (survey counting core). The rest: accepted.
  - After R-038 only `lob_text_kind`, the unconfirmed check and the tests in `t6l_ru.rs` change.

## T-5f (Implementer, 2026-10-16)

### N-20 Choices made in T-5f (please confirm or correct)
- **Test table for ADR-33 item 4 is `t_num3_exp_18c`, not `t_num_exp_18c`.** The plan names `t_num_exp_18c`; its rows 19 and later
  (1e33 ... 38-digit integers) have integer parts that Decimal128(38, 10) cannot hold, so that table ends with `E-NUMBER-OVERFLOW`
  under every policy and cannot show the difference between `Default` and `Declared`. `T_NUM3` has six values, all with more than ten
  decimals (`truth_num3.txt`). The patched line keeps its length (`"V" NUMBER(38, 10))` and ` LOGGING` dropped from the storage text).
- **`NUMBER(*,s)` with 0 <= s <= 38 is `Declared`** (exact or `E-NUMBER-OVERFLOW`); in T-6 only a declaration with a precision counted
  as constrained. ADR-33 item 4 says "a usable declaration fixed the type", and `NUMBER(*,2)` is one. Plain `NUMBER` and the declarations
  no Decimal128 holds are `Default`; any `number_types` entry is `Override`; a column or table fallback is `Default` (type 2) /
  `Declared` (other types: the field is only used for type 2).
- **A `number_types` entry on a declaration no Decimal128 holds** (`NUMBER(3, 50)` with `decimal256(76,50)`) gives no `W-DDL-FALLBACK`:
  the declaration is honoured through the override (ADR-30 item 3 "reads it exactly"). Without an entry: one `W-DDL-FALLBACK` per column
  (table, column = file position, offset of the definition, message "no Decimal128 holds NUMBER(p,s)").
- **The [T9] short forms in the plan text**: `NUMBER(3,50)` and `NUMBER(38,-2)` are written `NUMBER(3, 50)` / `NUMBER(38, -2)` in a real
  line (comma and space); only `NUMBER(*,s)` has no space. The tests use the real forms (a string without the space is an unknown
  text and a column fallback of another kind).
- **`W-TSLTZ`** is a new `WarningCode::Tsltz` ("W-TSLTZ"), raised by `resolve_table` for every descriptor of type 231, also on a table
  or column fallback, `table` and `column` (file position) set, offset of the descriptor. In a column fallback it comes before the
  `W-DDL-FALLBACK` of the same column. Python (T-12) must know the new code.
- **`Dump::part_rows` with an index outside the parts is `E-CONFIG`** with `length` = the number of parts (ADR-32 item 3).
  `tests/t3_catalog.rs` (`x_a_reader_at_rows_offset...`) called `rows` on all 382 tables; it now accepts `E-CONFIG` for tables of
  more than one part and checks `length`.
- **Counted warnings on error**: `TableBatches::next_batch` calls `collect_counted` when the inner step returns an error, so
  `warnings()` after the error holds the counts up to and including the values of the failing row that were already counted.
- `ReaderInit.types` and the `table_name` / `types` parameters of `plan_column` were dead after the change and are removed.
- **Answer (Architect, session 7; ADR-35)**: all **accepted**. `NUMBER(*,s)` with 0 <= s <= 38 is `Declared`; a
  negative s takes `number_default` with `W-DDL-FALLBACK` (`NUMBER(*,-2)`: Decimal128(38, 10); above 28 integer
  digits `E-NUMBER-OVERFLOW` under every policy, `round` never applies since the values have no fraction).
  Order within one column `W-CHARSET-UNSUPPORTED`, `W-TSLTZ`, `W-DDL-FALLBACK` is now part of the contract.
  `W-TSLTZ` once per type-231 column and table, in the scan's list only; Python issues it once (T-12).

## Questions from the Architect to the Implementer

### AQ-1 Is there a long LOB value with total T = 0 in the 199 fixtures? (session 4, for ADR-23) - closed (answered 0, session 5)
- Why: the owner accepts a long-form LOB value with T = 0 silently only if a fixture contains one; otherwise the
  reader warns (`W-UNTESTED-LAYOUT` at the `fc ff`).
- What I know: `tests/data/lob_expected.tsv` (177 files) has 8 long values, all with T = 100,000 or 200,000; rule L4
  counts 74 short and 40 long values in 199 files and says an empty LOB is the short form (h = 0, 12 values). I expect
  0, but the 22 files of session 9 are not in your TSV yet.
- Asked: in T-3c, after regenerating the expected values for 199 files, count the long values with T = 0 (walker
  and `exp_parse.py` both) and answer in a note: the count, and for each the file, table, row, column and offset.
  0: implement the warning (ADR-23). More than 0: accept them silently, add them to the expected values, and say so;
  the Architect then amends ADR-23.

## T-8 (direct path) notes

- No design problem. `crates/oradump-core/tests/t8_direct.rs` only; `exp/rows.rs` unchanged.
- Fixture fact (logs): `long_exp_direct_<v>` also exports the empty table `T_LONG_EMPTY` (0 rows), `long_exp_<v>` does
  not. The pair test excludes it from the table-list comparison and reads it on its own (0 rows).
- The pair test reads with `on_value_error=null`, `on_number_overflow=null` so that the tables with value errors
  (`T_DATE` rows 8 and 9, `T_NUM` extremes) are compared row by row; with the defaults both paths fail the same way.
- The ambiguous synthetic case is built by patching a copy of `t_empty_exp_direct_18c.dmp` below `target/`
  (descriptor length of column 1 raised to 30,000, or `M` after the LF changed to `m`): `Framing` at the LF.
- **Answer (Architect, session 7; ADR-36)**: accepted. Recorded: T-7 and T-8 needed no production change; 80 pairs
  equal; the version line is not used [E3]; `T_LONG_EMPTY` only in the direct LONG files (checked against the
  four `long_exp*` logs). Arch 8.1 now names `t7_owner.rs` and `t8_direct.rs`. T-10 counts `T_LONG_EMPTY` for
  `long_exp_direct_<v>` only.

## T-9 (Implementer, 2026-10-17)

### N-21 Choices made in T-9 (please confirm or correct)
- **Plan text "own_exp_w1252: `ddl_lossy`" cannot hold with arch 6.5.** The `CREATE TABLE` lines of `own_exp_w1252_<v>` (O_CHILD,
  O_EMPTY, O_PARENT) contain neither `?` nor 0xbf; the replaced `д` stands in a COMMENT item, which the reader does not expose.
  By arch 6.5 (`ddl_lossy` = converted at export **and** `?` / 0xbf present, or a byte the client set rejects) the flag is false
  there, and the test asserts that, plus `DumpInfo.text_converted_at_export` = true for the file. The positive case is tested with a copy
  whose `TABLESPACE "USERS"` is patched to `USER?` (true in the converted copy, false in the unconverted one), and with
  `ru_exp_us7ascii` / `ru_exp_w1252` (`???_???`, `¿¿¿_¿¿¿`: name, columns and DDL flagged). Proposal: drop "own_exp_w1252: ddl_lossy"
  from the plan, or say which text item it meant.
- **Flag definition** (`name_possibly_damaged`, `ddl_lossy`, `ColumnInfo`, `PartInfo.*_possibly_damaged`, `DumpInfo.user_possibly_damaged`):
  decoding with the client set replaced a byte (U+FFFD), **or** the file was converted at export (header client id != database id) and
  the raw text has `?` (also 0xbf when the client set is WE8MSWIN1252). A real `?` in a name of a converted file is flagged too: a hint.
  `t3_catalog.rs` asserted the old "any byte >= 0x80" rule and the "Cyrillic name is damaged" case; both now assert the new rule.
- **`possibly_incomplete`** = 0 rows (all parts) and (table name or any column name possibly damaged), with `W-POSSIBLY-INCOMPLETE`
  (new `WarningCode::PossiblyIncomplete`; table, offset of the `CREATE TABLE` line). Python (T-12) must know the new code. The fixtures:
  exactly the `???_???` / `¿¿¿_¿¿¿` table of `ru_exp_us7ascii`, `ru_exp_w1252`, `ru_exp_direct_us7ascii`.
- **Overrides** (`Options.db_charset`, `Options.client_charset`, strings; `Options.types.db_charset` is set by `open`, a value given
  there is replaced). A name is an Oracle name (case-insensitive), `ascii` / `us-ascii` (= US7ASCII; the WHATWG label is windows-1252, so it
  is not passed on), or an `encoding_rs` label (`Charset::Other`). Unknown: `E-CONFIG` at `open`. `db_charset` replaces the descriptor set
  of every VARCHAR2 / CHAR column (no Binary column, no `W-CHARSET-MISMATCH`), the LONG set and the set of a CLOB with a single-byte id
  (873 stays UTF-16, NCLOB stays UTF-16). It does **not** rescue a CLOB whose id is none of 873 / 171 / 178 / 1 (still LargeBinary +
  `W-CHARSET-UNSUPPORTED`). ADR-25 follows the override when it is a single-byte set; `Charset::Other` is never counted as single-byte.
  `client_charset` replaces the header's id for names, DDL, partition names, user and version.
  - **Answer (Architect, session 8; ADR-25 session 8 item 2)**, on "ADR-25 follows the override" only: changed. Whether a LOB
    value is unconfirmed is decided by the header's database set [H4] against `LOB_CONFIRMED_DB_SETS = [873, 171]`; `db_charset`
    chooses the decoder only. T-6M builds it (its test (d): the 178 copy with `db_charset="CL8MSWIN1251"` still stops).
- **Unsupported set of a type 1 / 96 column** (form 1, id not in the five, no `db_charset`): Arrow `Binary`, `W-CHARSET-UNSUPPORTED` from the
  scan (table, file position, offset of the descriptor, `length` = the id), read as raw bytes. `strict` -> `UnsupportedCharset` is T-11. A
  descriptor with no id (unit tests only) counts as supported.
- **Unsupported database set of the header**: before T-9 any table failed with `E-CONFIG`; now only a table with a LONG column does (the
  LONG value uses the header's database set, the type 8 descriptor has no id), unless `db_charset` is given. Proposal: LONG could become
  LargeBinary + `W-CHARSET-UNSUPPORTED` like type 1; not done, the arch does not say.
- **`trim_char`**: `ReadOptions.trim_char`, removes trailing U+0020 after decoding, type 96 only (CHAR, NCHAR); VARCHAR2 / NVARCHAR2 / LOB
  never. Default false (padding kept).
- **No US7ASCII database in the fixtures**: the "US7ASCII column" test patches `t_text_exp_11g.dmp` (header database id 873 -> 1, descriptor
  ids of V and C -> 1, a data byte -> 0xe4) into a copy below `target/`; the error text names `db_charset`. The patch of the descriptor ids
  alone gives `W-CHARSET-MISMATCH`, also tested.
- **R_TEXT rows come in storage order**, not ID order (`truth_ru.txt` is ordered by ID): the test matches by ID.
- **Expected numbers**: `csmap_cmp.py` (18c `truth_csmap.txt`): CL8MSWIN1251 255/256 equal to Python cp1251, 256/256 to the WHATWG reading;
  WE8MSWIN1252 251/256 and 256/256; US7ASCII 128 equal, 128 U+FFFD. The test finds the same with `encoding_rs`: 512 lines equal, `had_errors`
  false, no fallback table needed (ADR-5 stands). The two `truth_csmap.txt` are byte-identical (checked).

- **Answer (Architect, session 9; ADR-40)**: `ddl_lossy` of `own_exp_w1252`: you are right, the plan sentence is
  dropped (Reviewer session 2 checks the COMMENT-item claim). Flag definition, `possibly_incomplete`, override
  names (`Other` never confirmed, listed as untested in the README), `trim_char`, the patched US7ASCII copy,
  R_TEXT by ID: accepted. LONG with an unsupported header set: stays `E-CONFIG` for the alpha; LargeBinary only if
  Q-4 shows such a database set.

## T-10 (Implementer, 2026-10-17)

### N-22 Choices and findings in T-10 (please confirm or correct)
- **T-9 is not in the repository** (no `Implementer T-9` commit, no `csmap` test, no T-9 section here). T-10 "Needs" T-9. I built T-10
  on the reader as it is: every `T_TEXT` value of the 16 files (3 clients) equals `truth_text.txt` already, and the R_* tables are
  covered by `t6l_ru.rs`. Not covered by anything yet and left to T-9: the 768-line map test (`truth_csmap.txt`), `truth_charsets.txt`,
  the patched-descriptor test, `db_charset` on a patched byte.
- **`tests/t10_truth.rs`** reads every table of the 199 files (382 sections). 29 checkers run here (by table name, expected values from
  the `truth_*.txt` of the folder; instance counts are asserted). 18 table families are *delegated* to a named test of an earlier task
  (LOB, LONG, intervals, ROWID, partitions, R_*, T_STC, T_LONG_EMPTY); `delegated_tests_exist` fails if such a test is renamed or removed,
  and every delegated table is still read to the end with its `num_rows` checked. A table with neither a checker nor an entry fails the test.
- **Truth files with no EXP table**: `truth_float.txt` (T_FLOAT exists only in `t_float_dp`, a Data Pump file), `truth_csmap.txt`,
  `truth_charsets.txt`, `truth_env.txt`, `truth_own_ddl.txt`, the statistics lines of `truth_stat.txt` / `truth_stc*.txt`
  (catalog items, not values), `truth_decl.txt` (T-5, `t5_types.rs`).
- **Truth texts are partial in places**: `truth_len.txt` DUMP shows 12 bytes (`DUMP(a,16,1,12)`), so T_LEN compares prefix, `Len=` and the
  length rule; `truth_wide.txt` is padded to the column width (8), so the comparison trims the truth on the right; BN1 to BN4 give the length
  and a prefix, the rest comes from the `RPAD` of `06_bn.sql`. Rows are compared by ID, not by position (T_LEN has ID 0 last in `multi_empty_exp_11g`).
- **Answer (Architect, session 8)**: T-9 is merged now; the step (2) leftovers (R_TEXT in the registry, the checkers with
  the T-9 reader, `truth_charsets.txt`, a break test per checker) are task **T-10b**, after T-3d (corpus 201).
- **Warning inventory of the 199 files** (asserted): `W-TSLTZ` for T_TSTZ (4 files); in the lenient re-reads `W-VALUE-NULLED` for T_DATE (8),
  T_TS (8), T_NUM (8), T_INTERVAL2 (2) and `W-NUMBER-ROUNDED` for T_NUM (8), T_NUM2 (2), T_NUM3 (2). Nothing else: no `W-UNVERIFIED-TYPE`
  (URID included), no `W-LONGER-THAN-DECLARED`, no `W-DDL-FALLBACK`, no walker warning.
- **T_NUM under the default options stops** at row 19 (`E-NUMBER-OVERFLOW`, integer part) in all 8 files, as under `error`; with `round` and
  `on_value_error=null`: 10 rows NULL (19 to 24, 26, 28, 29, 32), 4 rounded (25, 27, 30, 31; 27 to 0), computed from the truth text.

- **Answer (Architect, session 9; ADR-38, ADR-39, ADR-40)**: registry and delegation accepted with the T-10b
  changes (only opened files count, keyed by folder; new checkers; instance counts from `exp_catalog.tsv`);
  partial truth texts accepted where the rest is rebuilt from the SQL (T_LEN now exact); warning inventory
  accepted, the three `W-POSSIBLY-INCOMPLETE` entries approved (ADR-39); T_NUM stopping by default accepted.

## T-6M (Implementer, 2026-10-17)

### N-23 Choices and findings in T-6M (please confirm or correct)
- **The message of `E-LOB-UNCONFIRMED` "names 178"** (T-6M (d)): `Error.message` is `&'static str` and no API may change (ADR-34
  item 9), so the database set cannot be formatted in. `lob_unconfirmed_message(db_id)` picks one of four static texts: database
  set 178, database set 1, "outside the confirmed ones" (any other id, e.g. 31), and "long-form CLOB whose set differs from the
  database set" (D in the list, S != D). Each names the list "873, 171" and `unconfirmed_lob`, none names R-038. The list literal
  appears in these texts too (a string, not a decision). Proposal: if the owner wants the number in every message, `message` becomes
  `Cow<'static, str>` (small API change; not done).
- **`W-LOB-UNTESTED` text** lost the words "single-byte database" (it is no longer the criterion); the code and counts are unchanged.
- **Plan (c) test name**: `an_nclob_in_a_single_byte_header_is_unconfirmed` keeps its name as the plan says, although T_LOB2.N now
  decodes; only T_LOB.C row 5 is still refused. Its T_LOB2 reads carry T-9's `W-CHARSET-MISMATCH` for the form-1 CLOB A (S 873 != D 171)
  and nothing about LOBs; the test asserts exactly that (the plan sentence "without warning" means no LOB warning).
- **Header offsets** of the 178 copy (plan (d)): the database set low byte is at 0x36 (`00 ab` at 0x35..0x37), the CLOB descriptor set
  low byte at 0x24e3; both asserted in the test before patching. Offsets 0x2790 / 0x29c4 and length 132 hold as the plan says.
- Behaviour change beyond 171 worth knowing: a header with a database set outside the five (e.g. 31) or a set that is not 873 / 171
  now refuses NCLOB values and long CLOB values (was: only single-byte headers 1 / 171 / 178 were refused; a header 873 with a CLOB of
  descriptor set 171 or 178 is now `LongOnly` unconfirmed too, by the rule S != D). No fixture is affected (all 257 tests but the 11
  of T-3d pass).

- **Answer (Architect, session 9; ADR-40)**: (1) `message` becomes `Cow<'static, str>` in T-12 (first Rust
  step), one text naming the set id. (2) to (4) accepted. **Correction**: T_LOB2's `W-CHARSET-MISMATCH`
  warnings are column V's; the CLOB A gets none because the reader checks types 1 / 96 only (N-25); T-9b fixes
  it and asserts per column. (5) accepted (ADR-25 session 8). The count at `a937f2f` was 257, not 256.

## T-3d (Implementer, 2026-10-17)

### N-24 Choices and findings in T-3d (please confirm or correct)
- **Generators**: the asserts and docstrings of the four generators were updated by hand to the 201-file totals (the plan says the
  generated files are not edited by hand; the generators' own constants had to follow). Run: the tools in `docs/formats/tools` with
  `python3 -I`. Printed totals: `gen_walk_expected.py` 201 files, 386 sections, 400 parts, rows 41,212, fields 243,007, NULL 19,900,
  LOB values 126 (80 short, 12 of them empty, 46 long), 21 LOB parts; `gen_number_parts.py` 201 files, 400 parts, 86,080 NUMBER fields.
  The diff of every TSV is additions only (2 lines in `walk_expected.tsv` / `lob_expected.tsv`, 4 in the part-level files).
- **A masked T-9 effect in T-10's warning inventory**: `EXPECTED_WARNINGS` of `t10_truth.rs` did not list `W-POSSIBLY-INCOMPLETE` for
  the failed-export tables (`???_???` in the two US7ASCII client files, `¿¿¿_¿¿¿` in the WE8MSWIN1252 one; X2, `t9_charset.rs` asserts
  it per file). T-9 added the warning after T-10 was written; the test could not run to that point (it failed on the count 199).
  The three entries are now in the inventory. The two `ru_lob2_*` files add no warning (new test `the_ru_lob2_files_add_no_warning`).
- **`t4_values.rs`** was not in the plan's list of 11 but had a failing test (`a_all_fields_of_199_files_decode`, 396 / 85,456): fixed
  like `t6_value_parts.rs` and renamed `a_all_fields_of_the_exp_files_decode`. After the TSVs were regenerated 18 tests failed, not 11 (the other
  seven had hard-coded old numbers that the old TSVs satisfied: `t1_walk` `b_walk_ends_...`, `t3a` `l1_to_l4_...` and `x_the_199_...`, `t3_catalog`
  `x_column_names_...`, `x_a_reader_at_rows_offset...`, `t6_value_parts`, `t4_values`).
- **`every_truth_file_is_read_or_listed`** counts a truth file as read when its name is a literal in the code lines (comments
  skipped) of `t10_truth.rs` before the explicit list or in a file named by the `DELEGATED` registry (whole file, not the function).
  `truth_long2.txt` (only cited in comments; T_LONG2 is compared with `11_types.sql`) and `truth_misc2.txt` (opened by
  `t4_values.rs`, not by the registry's test) are therefore in the explicit list with that reason, next to the six files with no EXP
  table and `truth_r045.txt`.
- **Counts by derivation**: the per-folder and total numbers stand in `tests/common/mod.rs` (`FOLDER_TOTALS`, `TOTALS`) and are
  compared with the TSVs in `tool_table_reproduces_the_plan_numbers_for_the_exp_files` (t1_walk.rs); the other tests use `TOTALS`
  for the file count or the length of a golden file. The per-list-entry totals changed: ` COMMENT ON ` 130 -> 134, `CALTER TABLE`
  49 -> 51 (sum 783 = `text_fields`).
- `collist.py` over 201 files: 386 lines, 1,907 definitions (was 1,885); the per-form table in `t5_types.rs` was taken from its
  printout (BLOB 12, CHAR(n) 53, CLOB 21, DATE 141, NCLOB 10, NUMBER(n, n) 325, NUMBER(n, n) NOT NULL ENABLE 37, NVARCHARn(n) 47,
  VARCHARn(n CHAR) 41, VARCHARn(n) 998; the rest unchanged).

- **Answer (Architect, session 9; ADR-39, ADR-40, ADR-42)**: generator constants edited by hand: accepted (outputs
  regenerate byte-identically). Inventory entries: approved (ADR-39). `t4_values.rs`: accepted. Loose literal
  match and the two explicit-list entries: replaced by T-10b (ADR-38 items 1, 2). Counts by derivation:
  accepted, plus `column_definitions`. `tests/golden/`: you regenerate it with the project's generators only, the
  Reviewer checks it (ADR-42).

## T-11 (Implementer, 2026-10-17)

### N-25 Choices and findings in T-11 (please confirm or correct)
- **Two new error codes** (arch 7.1 names none for a warning that becomes an error): `E-STRICT` (`ErrorCode::Strict`) for every warning
  outside the confirmed rules without a code of its own, with the new field `Error.warning: Option<WarningCode>` (shown in the text as
  `warning W-...`); `E-UNSUPPORTED-CHARSET` (`ErrorCode::UnsupportedCharset`) for `W-CHARSET-UNSUPPORTED` (arch 3.4 already lists the Python
  class `UnsupportedCharset`). `W-NUMBER-ROUNDED` becomes `E-NUMBER-OVERFLOW` and `W-LOB-UNTESTED` `E-LOB-UNCONFIRMED` (ADR-14, ADR-34 item 5).
  `WarningCode::strict_error()` is the one table; `W-NUMBER-NULLED` and `W-VALUE-NULLED` give `None`. `ErrorCode::ALL` / `WarningCode::ALL`
  list the 24 + 19 codes. **Proposal for T-12**: `E-STRICT` is a `FormatError`-free class of its own, for example `StrictError(OradumpError)`
  with `.warning` (a string); or `UnsupportedError` if the Architect prefers fewer classes. Not decided here.
- **Where `strict` acts**: `Options.strict`. `open` returns the first header warning (`W-UNTESTED-VERSION`, `W-UNTESTED-HEADER`, `W-FILE-SIZE`)
  as an error; `scan` returns the first walker or resolution warning with a strict error and **commits nothing** (no catalog, no warnings; a
  second call scans again); the walker's [E2a] decision follows (`strict_e2a`, also in `TableRows` and `TableBatches`, so `Framing` as in
  ADR-22). `read_table` fails on `W-CHARSET-MISMATCH` before any row; the value warnings (`W-UNVERIFIED-TYPE`, `W-LONG-LENGTH`,
  `W-LONG-CHARSET`, `W-LONGER-THAN-DECLARED`) are **fatal errors at the first value** (code, offset, table, part, row, column), not value-class
  errors: `on_value_error=null` does not turn them into NULL (arch 7.1 puts them in the "outside the confirmed rules" class, whose strict
  outcome is "error"; only `E-NUMBER-OVERFLOW` and `E-LOB-UNCONFIRMED` are value class there). Say if `null` should cover them.
- **`round` under `strict`**: `plan_column` turns the effective policy `Round` into `Error` (the metadata `oradump:number_overflow` then says
  `error`), so the error text is that of policy `error` ("fraction digits beyond the scale"), code `E-NUMBER-OVERFLOW`, value class.
- **Per-column overflow policy**: `TypeOptions.number_overflow: BTreeMap<String, OverflowPolicy>`, same key as `number_types`, separate map so
  that the 20-odd existing uses of `number_types` did not change; a key without a `number_types` entry is `E-CONFIG` at `open`. Reaches the reader
  as `ColumnInfo.overflow_policy` (set only when the entry fixed the type, `TypeSource::Override`). Precedence: entry, then
  `ReadOptions.on_number_overflow`, then (strict) `Round` -> `Error`. T-12 builds both maps from `("decimal(p,s)", "round")`.
- **`Warning.max_digits_dropped`** (u32, `W-NUMBER-ROUNDED` only): the largest (fractional digits - scale); `Display` adds "at most N digits
  dropped". Plan (b) wording: `t_num` V cannot "stop at the first value with more than 10 fractional digits" under `raise`, because rows 19 to
  24 (integer part) fail first under every policy; the test uses `on_value_error=null` on `T_NUM` (14 NULLs with entry `error`, 4 rounded
  with entry `round`) and the stop itself on `T_NUM3` (row 1).
- **`extra_intro_lines: Vec<String>`** in `Options`, one `kind:length:forms:text` entry per string (`IntroList::with_extra`); empty, comment,
  line break, bad entry: `E-CONFIG` at `open`. Used by the scan, `TableBatches` and `TableRows`.
- **Counting core** (`oradump_core::survey::count_table(&mut Dump, TableRef) -> TableCounts`, per column: `values`, `nulls`,
  `years_below_1`, `months_beyond_32_bits`, `no_decimal128_holds`, `lob_refused_short` / `lob_refused_long`): built from the plan's list. The
  NUMBER profile of arch 5.2b (per candidate type) is **not** built; the plan assigns it to T-12 ("T-12; counts only") and T-11's list does
  not name it. Say if it belongs to T-11. The LOB predicate is `reader::lob_value_unconfirmed`, the same code as the reader's check.
- **Finding (T-9, not fixed here)**: ADR-34 item 6 / the plan say `W-CHARSET-MISMATCH` also for a form-1 CLOB column whose descriptor set differs
  from the header's database set; `reader.rs` still raises it for types 1 and 96 only (N-23 says T_LOB2 "carries" it for the CLOB A; those
  warnings are the VARCHAR2 column V's, the test's `all(...)` passes on any list). No fixture is affected (the CLOB sets equal the database
  set in all 201 files), so `strict` over the fixtures is unchanged. Proposal: add `112` to the condition (form 1, decodable or not).
- **`W-LONG-LENGTH` offset**: the offset of the last piece (a direct-path chain) or of the value (a conventional field); ADR-27 says "first
  value", the plan says nothing about the offset. The test patches a conventional value to 32,001 bytes (`long_exp_18c.dmp`, row 3).
- **The fixture-wide `strict` list** (T-11 test `strict_over_the_201_files_...`): scans fail for 4 files with a TSLTZ column (`W-TSLTZ`) and 3 with
  the failed-export table (`W-POSSIBLY-INCOMPLETE`); reads fail only with `strict` for `T_NUM2` and `T_NUM3` (2 files each, row 13 / row 1,
  `E-NUMBER-OVERFLOW`); reads that fail in every mode: `T_NUM` 8, `T_DATE` 8, `T_TS` 8, `T_INTERVAL2` 2 (26 in all). A second test derives
  the list from the plain run (a scan warning with a strict error, or a read that warns so or fails). No `W-CHARSET-MISMATCH`, `W-LONG-*`,
  `W-LOB-UNTESTED` or walker warning in any fixture.
- Tests: `t11_codes.rs` (24 + 19 codes, one test each, plus the check that every code has its test), `t11_strict.rs` (27), `t11_options.rs` (15),
  unit tests in `error.rs`; helpers `patched_file`, `find_first`, `find_one`, `field_offsets`, `row_starts`, `exp_files` in `tests/common`.
- **Answer (Architect, session 9; ADR-41)**: (1) both codes accepted; Python `StrictError(OradumpError)` with
  `.warning`, `UnsupportedCharset`; `Error.warning` on every strict-produced error (T-12 completes it if needed).
  (2) value warnings as `E-STRICT` at the first value, not NULLed by `null`: confirmed; the `Dump` stays usable.
  (3) `round` -> `error` with metadata `error`: confirmed. (4) NUMBER profile: T-12. (5) the CLOB mismatch:
  bug confirmed, task **T-9b** (type 112 in the condition, decodable or not; test on the 171 copy of
  `lob_exp_18c`). (6) `W-LONG-LENGTH` offset = start of the value: conventional as built; a chain reports the
  first data byte of its first piece (T-9b). The rest of N-25 accepted as built.

## T-9b (session 9): E-LOB-TOO-LARGE offset on a LONG chain

- Task T-9b (c), `reader.rs::long_piece`. `E-LOB-TOO-LARGE` on a LONG chain reported the offset of the piece that
  passed `max_lob_bytes` (and the doc comment of `max_lob_bytes` says so). Following the same rule as
  `W-LONG-LENGTH` (ADR-41 item 6) it now reports the first data byte of the first piece (`LongState.start`).
  The unit test `reader::tests::max_lob_bytes_is_checked_piece_by_piece` asserted the old offset and was changed
  from 30 to 10; `t9b_fixes.rs` tests it on the 32,001-byte chain. `E-LOB-TOO-LARGE` for a LOB (not LONG) is
  unchanged.
- **Answer (Architect, session 10; ADR-44 item 8)**: accepted (pre-authorised by plan T-9b (c)). ADR-43 item 3
  makes "the start of the value" the rule for every report of one value under every policy; T-12 moves the
  chain's `W-VALUE-NULLED` and `E-BAD-TEXT` (review 02, S-3) to `st.start`.

## T-10b (Implementer, session 9): the truth-file test credits only what a check opened

Built as ADR-38 and the plan T-10b say (steps (a) and (b) in one session; nothing is committed [session 10: stale;
T-9b is `e308d9a`, T-10b `7941e40`]). Files:
`tests/common/mod.rs` (`truth_text`, `opened_truth_files`, `column_definitions`), new `tests/common/truth_checks.rs`
(the family checks, each a pure `*_diff` and a `*_diffs` that reads the fixtures), `tests/t10_truth.rs`, and the
one-line calls in `t6l_lob.rs`, `t6_read.rs`, `t6_parts.rs`, `t6l_ru.rs`, `t7_owner.rs`; `t2_manifest.rs`,
`t3c_walker.rs`, `t4_values.rs`, `t5_types.rs`, `t9_charset.rs` open their truth files through `truth_text`
now. No production code. (The uncommitted `reader.rs` / `t9b_fixes.rs` / T-9b note in the tree are T-9b's; T-10b
did not touch them and the 370 tests include `t9b_fixes.rs`. [session 10: 371 at `7941e40`, review 02 section 6.])

**(a1) The grep.** Only `common::truth_text` opens a `truth_*.txt`:
`grep -n "read_to_string\|fs::read(" crates/oradump-core/tests/*.rs | grep -i truth` lists lines of `t10_truth.rs`
that read test sources and `SHA256SUMS`, none that reads a truth file (the generic checksum guard of
`t2_manifest.rs` hashes every file without naming one).

**Choices and deviations (please confirm or correct)**
1. The registry `DELEGATED` has a fourth field, the family check the test calls; `delegated_tests_exist` fails if
   the test is gone or no longer calls it, and every family check must be used by a registry entry (or be
   `charsets_diffs` / `csmap_diffs`). Entries that point into `t10_truth.rs` (new tests): T_INTERVAL2 and T_ROWID2
   (`t_interval2_and_t_rowid2_equal_truth_misc2`), R_TEXT (`r_text_equals_truth_ru_in_all_27_sections`), R_STAT
   (`r_stat_equals_truth_stat_and_the_sql_rule`), T_LONG_EMPTY (`t_long_empty_has_no_rows_and_exists_in_the_direct_files_only`,
   ADR-36 item 4), T_PART (the existing `t_part_matches_the_p_lines_and_the_total_line`, which
   `every_truth_file_is_read_or_listed` now calls). T_PART2 points to `t6_parts.rs::t_part2_...` (S-1).
2. The family checks open and read the fixtures themselves (they need not receive the delegated test's batches);
   the delegated test calls the check **in addition to** its remaining assertions. Where the delegated test
   already compared the truth by its own code (`t6_read.rs::intervals_and_rowids_equal_truth_misc`,
   `t6l_lob.rs`, `t6l_ru.rs`) that code stays: nothing is weakened, the comparison now exists twice. Say if you
   want the duplicates removed. `t7_owner.rs::stc_family_reads_through_the_same_code` is rewritten: counts come
   from `stc_counts` (truth_stc.txt, T_STC4 from `truth_stc_extra.txt`), S-3.
3. Extras beyond the plan, all from the truth files: T_STC `COLUMN|T|pos|name|stats:d:n` lines (distinct and NULL
   counts of the reader's values; names), R_LOB2 head and tail DUMPs, R_CLOB GETLENGTH and head, T_LOB3 DUMP
   (`GETLENGTH` of a CLOB counts UTF-16 units: id 20 has 10 code points and length 11).
4. R_LOB2.B: the truth gives its length only; B is compared with the CL8MSWIN1251 bytes of the reader's C (so a
   wrong reader C gives two differences). The map for R_TEXT/R_CLOB/R_LOB2 is Oracle's (`truth_csmap.txt`, read
   from `18c`; `11g` is compared byte for byte, `csmap_diffs`), not `encoding_rs`.
5. Explicit list `TRUTH_NOT_OPENED` (folder, name, reason): `truth_float`, `truth_env`, `truth_own_ddl`,
   `truth_decl` (18c and 11g), `truth_r045` (18c). Checked reasons: the `t5_types.rs` test for `truth_decl`
   exists; no `T_FLOAT` section in `exp_catalog.tsv` for `truth_float`. The reasons for `truth_env`, `truth_own_ddl`
   and `truth_r045` are statements the test cannot verify without opening the file (which would credit it): they
   rest on the Reviewer's reading (review 2.1).
6. Break tests are on the pure `*_diff` functions with a truth line or a reader cell changed in memory. Mutation
   e (`check_wide` compares nothing) is caught by the break test when the comparison inside `wide_diff` is emptied;
   removing the `assert!` of the thin wrapper `check_wide` itself is **not** caught by any test (the same holds
   for every checker of `t10_truth.rs`). A counter of executed comparisons would close it; not built.
7. The R_STAT, R_TEXT, T_LONG2, misc2 and STC checkers found no disagreement in any file (TXT equals the rule in
   all 10 client sets); nothing was adapted.
8. (a5) done: `gen_walk_expected.py` comment (outputs re-generated: `git status` shows the three TSV files
   unchanged), `FolderTotals.lob_sections` doc, comment of the `W-POSSIBLY-INCOMPLETE` entries (`EXP-00008`),
   `TOTALS.column_definitions` = 1,907 (used by `t5_types.rs`), N-23's count 257.

**(b9) The review's mutations**, each alone on a scratch copy of the repository (session scratchpad, never on
`corpus/fixtures/`); baseline there 370 passed [session 10: 371 at HEAD]. Besides each, `t2_manifest::x_fixture_checksums_...` fails for a
fixture mutation and the break test of the same checker fails because it reads the changed file (expected).
| mutation | tests that fail (the named check in bold) |
|---|---|
| a3 `11g/misc2_exp_11g.dmp` @0x24cf `33`->`34` | **`t10_truth::t_interval2_and_t_rowid2_equal_truth_misc2`**, `every_truth_file_is_read_or_listed`, `t2_manifest` (2) |
| b4 `18c/truth_long2.txt` `C3A9`->`C3A8` | **`t6_read::t_long2_decodes_cyrillic_and_accents_without_a_warning`**, `every_truth_file_is_read_or_listed` |
| b5 `11g-cp1251/truth_stat.txt` `TAB|R_STAT|2000`->`2001` | **`t10_truth::r_stat_equals_truth_stat_and_the_sql_rule`**, `every_truth_file_is_read_or_listed` |
| b6 `18c/truth_stc.txt` `TAB|T_STC1|50`->`51` | **`t7_owner::stc_family_reads_through_the_same_code`**, `every_truth_file_is_read_or_listed` |
| a2 `11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp` @0x227b5 `da`->`c2` | **`t10_truth::r_text_equals_truth_ru_in_all_27_sections`**, `every_truth_file_is_read_or_listed`, `t2_manifest` (2), `t8_direct::every_direct_file_equals_its_conventional_partner` |
| a4 `18c/multi_framing_exp_18c.dmp` @0x24ad `x`->`7` | **`t10_truth::every_table_of_the_exp_files_is_classified_and_read`** (T_LEN), `every_truth_file_is_read_or_listed`, `t2_manifest` (2), `t8_direct` |
| e1 `wide_diff` compares nothing | **`t10_truth::break_t_wide_and_t_stat_one_changed_value_gives_one_difference`** only |
| e2 `stat_diff` compares only the row count | the same break test only |

**Answer (Architect, session 10; ADR-44, ADR-43 item 5)**: (1) the fourth `DELEGATED` field: accepted. (2) The
duplicates stay (none hides a weaker check; review 02 section 2). (3) The extra comparisons: accepted. (4) R_LOB2.B
against the rule of `15_ru_lob2.sql`, not the reader's C (T-11b); Oracle's map: accepted. (5) The
`TRUTH_NOT_OPENED` reasons: accepted (checked by Reviewer 2); `truth_env` and the OBJECT TABLE lines of
`truth_own_ddl` get checks in T-11b. (6) The wrapper gap: closed in T-11b by checkers that return their
differences and one asserted union, with a per-tag break test through `check_table`. (7), (8): facts, checked.

## T-11b (Implementer, session 10): the second review's test fixes

Tests only; no production code, no file of `corpus/`, `docs/formats/` or `t6_patched.rs` touched. Files:
`tests/t10_truth.rs`, `tests/t11_strict.rs`, `tests/t11_options.rs`, `tests/common/truth_checks.rs`, `tests/t4_values.rs`,
`tests/t3b_parts.rs`, `tests/t9_charset.rs`, `tests/t9b_fixes.rs` (`tests/common/mod.rs` unchanged). Result: fmt, clippy
(`--all-targets` too) and nextest green, **376 tests** (371 + 5 new: `break_every_tag_of_check_table_one_changed_cell_gives_a_difference`,
`arch_7_1_strict_covers_every_warning_code_and_equals_strict_error`,
`the_counting_core_counts_years_below_1_in_the_timestamp_family_on_patched_copies`,
`truth_env_and_truth_own_ddl_equal_the_headers_and_the_catalog`, `break_env_and_own_ddl_one_changed_line_gives_one_difference`).
(`grep -c '#\[test\]'` gives 377 because one comment in a test file contains the text; nextest runs 376.)

**Mutations** (each alone on a scratch copy of the repository in `~/.cache/oradump-t11b`, removed afterwards; baseline 376 passed):
| mutation | tests that fail |
|---|---|
| e1 `survey.rs`: `years_below_1` for type 12 only | **`t11_options::the_counting_core_counts_years_below_1_in_the_timestamp_family_on_patched_copies`** (only) |
| a1 `strict_error()`: `W-TSLTZ` -> none | **`t11_strict::arch_7_1_strict_covers_every_warning_code_and_equals_strict_error`**, the unit test `error::tests::strict_makes_...`, `t11_codes::e_strict`, three more fixture tests |
| a1b `strict_error()`: `W-LONG-CHARSET` -> none (a code no 201-file fixture reaches) | **`arch_7_1_strict_...`**, the unit test, `t11_strict::strict_long_charset` |
| e3 `check_wide`: `c.merge(diffs)` deleted (the nearest thing to deleting its `assert!`; no per-checker `assert!` is left) | **`t10_truth::break_every_tag_of_check_table_one_changed_cell_gives_a_difference`** (only) |
| e4 `11g/t_wide_exp_11g.dmp` @0x3804 `5` -> `6` (`v115` -> `v116`; the bytes `v115` stand at 0x3801..0x3804, the review's offset is the last one) | **`t10_truth::every_table_of_the_exp_files_is_classified_and_read`**, plus the break test (its control read of that file), `t8_direct::every_direct_file_equals_its_conventional_partner`, `t2_manifest` (2) |

**Choices and deviations (please confirm or correct)**
1. (c) The checkers keep their shape; `assert_eq!` / `assert!` became the macros `ceq!` / `cok!` that note a difference
   in `Ctx.diffs` (a `RefCell`), and the `*_diff` wrappers call `c.merge`. `check_table` returns `(tag, warnings, diffs)`;
   `union_of(&[(tag, diffs)])` is the one function both tests call. A checker still panics where the shape of the data
   is wrong (an `unwrap` on a read or a parse, an index after a failed length comparison); that fails the test too.
2. The break test changes one **reader cell** (`Ctx.edit`: row 0, column 1 of every successful read gets `_BREAK`; an empty
   table gets one invented row), for all 29 tags of `CHECKER_TAGS` (T_STAT is one tag; its table is T_STAT1), after a control
   run without the change that must give an empty list. It does not change a truth line (the per-family break tests of
   T-10b do). `T_PARENT`, `T_DECL`, `T_EMPTY`, `O_EMPTY` have no truth line anyway.
3. (b) `ARCH_7_1_STRICT`: 19 lines. Where arch 7.1 names no error for a code (W-TRAILING, W-UNTESTED-HEADER, ...) the line says
   `E-STRICT` and cites the 7.1 default rule plus ADR-41; the table was written from the text of 7.1, not from `strict_error()`.
   If the Architect reads 7.1 differently for a code, the table is the place to say it. `DOCUMENTED` has no count; the count of
   an entry is the number of golden sections with that table key (`catalog_key`: bytes outside ASCII as `\xNN`, so the reader's
   `\u{bf}` name maps to the catalog's `\xbf\xbf\xbf_...`). It was not necessary to use the `delegated()` keying of
   `t10_truth.rs` (none of the nine entries is a T_STC or a CYRILLIC key). The counts came out as before (8, 2, 8, 2, 2, 8, 2, 4, 1).
4. (a) S-2: the offset of `E-BAD-DATETIME` is also asserted (= the first patched byte, 4 patched values in two files); it held.
5. (d) N-2: exact counts from `exp_parse.py::parse` (scratch script `parse()` of every table, non-NULL fields grouped by column
   type; the same walk as `gen_number_parts.py`): DATE 37,942; TIMESTAMP 122; TSTZ 24; TSLTZ 8; IYM 12; IDS 18; ROWID 8; UROWID 10
   (NUMBER 86,080 is already asserted). `year_below_1` = 2 (rows of `truth_date.txt` that start with `-`) x 8 (T_DATE sections of
   `exp_catalog.tsv`) = 16. `lob_columns == 5` and the statistics of `lob_exp_18c` per column come from `tests/data/lob_expected.tsv`
   (values, short, empty, long, stored bytes, largest), no longer from the walker. The scratch script is not committed
   (it only imports the Analyst's tool); if you want it kept, it belongs with `gen_number_parts.py`.
6. (d) N-4: **the old comment was wrong**, not only the old assertion. `us7ascii_high_byte_...`: in the US7ASCII copy C has
   high bytes in rows 2, 3 and 4 (row 5 of C is `x`), so C's `W-VALUE-NULLED` is count 3 at row 2, V's is count 5 at row 1;
   both come from `truth_text.txt` now. `a_descriptor_set_other_than_...`: exactly one `W-CHARSET-MISMATCH`, column 3.
   The `t6_patched.rs:69-73` part of N-4 is T-12's (ADR-43 item 6).
7. (d) N-5: rows 1 to 3 of C against `01b_optional_tables.sql` (`hello lob`, NULL, empty string); the comparison with the unpatched read stays.
8. (d) N-6: `truth_checks::r_lob2_b_rule` builds B from the numbers of `15_ru_lob2.sql` (192..=255, 168, 184; 303 x blk + 2 bytes);
   a changed B is now one difference and a wrong C does not add one through B (break test). `lob_diffs` has the doc comment.
9. (d) N-8: `env_diffs` (the two NLS names of `truth_env.txt` -> ids by `truth_charsets.txt` -> header database and national set of
   **every** `_exp_` file of `18c` and `11g`: all equal 873 / 2000) and `own_ddl_diffs` (OBJECT TABLE lines = tables of `own_exp_<v>`:
   O_CHILD, O_EMPTY, O_PARENT); both left `TRUTH_NOT_OPENED` and are in `FAMILIES`, with break tests. `truth_decl` (optional) stays listed.
10. (d) N-9: the T_STC break test takes the counts from `stc_counts` (truth lines) and builds the `51` of mutation b6 as the truth
    count plus one; T_STC4's count is compared with the `TAB|T_STC4` line of `truth_stc_extra.txt`.
11. The file is still not covered against deleting the single `assert!(u.is_empty(), ...)` of
    `every_table_of_the_exp_files_is_classified_and_read` (ADR-43 item 5, review N-1): one line, as decided.
12. `every_truth_file_is_read_or_listed` runs `scan()` and ignores its differences (it only needs the files opened); the
    assertion of the union is in `every_table_...`.
13. Disk: `/tmp` (3.9 GB tmpfs) filled during the scratch runs; the mutation copies were built under `~/.cache` and removed.

**Answers (Architect, session 11).** Items 1, 2, 4 to 10, 12: accepted as built. Item 3 (`ARCH_7_1_STRICT`, `E-STRICT`
where 7.1 names no error, `W-TRAILING` included): confirmed, ADR-41 item 9; arch 7.1 now carries the same table.
Item 5's scratch script: not to be committed (the counts are asserted in the tests; the script only imported the
Analyst's tool). Item 11 (review N-1): stays open as accepted by ADR-43 item 5; Reviewer session 3 reads the line.
Item 13: noted; scratch copies stay under `~/.cache`.

## T-12 (Implementer, session 10): the Python binding

Built: the Rust steps (a), (b), (e), (f) of the plan, the survey's counting core for arch 5.2b, `crates/oradump-py`, the
Python package, `pyproject.toml`, the pytest files. Result: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
(the new crate included, with the workspace lints) and `cargo nextest run` green, **390 tests** (376 + 14: `t12_offsets` 3,
`t12_messages` 4, `t12_survey` 5, `t12_warning_field` 2); `ruff check`, `ruff format --check`, `mypy` (strict) clean and
`pytest` **58 passed** on Python 3.13 (the VM venv `.venv`, pyarrow 26.0.0, pandas 3.0.6), on 3.11.16 and 3.14.7 (the debug
abi3 wheel installed in two scratch venvs), and on 3.11 with pyarrow 15.0.2 + pandas 2.2.3 + numpy 1.x (the stated minimum).
The working tree held the **uncommitted T-11b diff** (8 test files, the T-11b note): not a conflict, T-11b was not committed
when this session started. T-12 touches two T-11b-area files, see D3.

Files: `crates/oradump-py/{Cargo.toml,src/{lib,conv,errors,names,options,digest}.rs}`; `python/oradump/{__init__,__main__,_api,_errors,_survey}.py`,
`_core.pyi`, `py.typed`; `pyproject.toml`; `tests/{conftest,test_handover,test_errors,test_warnings,test_threads,test_survey,test_consumers}.py`;
`crates/oradump-core/tests/{t12_offsets,t12_messages,t12_survey,t12_warning_field}.rs`; changed: `error.rs`, `reader.rs`, `survey.rs`,
`crates/oradump-core/Cargo.toml`, `Cargo.toml` (member), `Cargo.lock`, `tests/t6_patched.rs`, `tests/t11_codes.rs`, `tests/t11_strict.rs`.

**Numbers from the Analyst's tools** (`exp_parse.parse` + `num_decode.decode`, run read-only, scratch script with Python `decimal`,
precision 500, `ROUND_HALF_UP`; note that `num_decode` sets the decimal precision to 60 at import, the script sets it after the import):
`t_num_exp_18c.dmp` = `t_num_exp_11g.dmp` T_NUM V: 32 non-NULL, 19 integers, 11 fit Int64, 1 value of 40 digits, largest sig 40, frac 125,
int 126, exp10 -125..+125; Decimal128(38,10) 18 exact / 4 rounded (1 to 0) / 10 integer part too large; Decimal256(76,38) 27 / 2 (1) / 3.
`t_num2_exp_18c.dmp` T_NUM2 V: 14 values, 10 integers, 9 fit Int64, max sig 19, frac 130, exp10 -130..+18; 12/2(2)/0 and 13/1(1)/0.
`t_num3_exp_18c.dmp` T_NUM3 V: 6 values of 39 or 40 digits, sig 40, frac 40, int 2, exp10 -40..-38; 0/6(0)/0 and 2/4(0)/0.
`long_exp_direct_<v>.dmp` T_LONG (5, NULL, 32,000 bytes) and T_LONGRAW (2, 32,000): lengths from `exp_parse`; the chain counts (2 values, 1 chained,
largest 32,000, none above) from the scan, the 5 pieces from the ADR-43 probe. All are asserted in `t12_survey.rs` and `test_survey.py`.

**Mutations** (scratch copy `~/.cache/oradump-t12`, each alone, removed after; baseline 390 passed):
| mutation | tests that fail |
|---|---|
| d1 `reader.rs field()`: lossy text of the field appended to a value error's message (not `E-BAD-TEXT`) | `t12_messages::no_message_of_the_fixtures_holds_a_value` (`misc2_exp_11g.dmp`, `{Text}`) |
| d2 same, hex of the field | the same test (`{Hex}`) and `the_check_reports_a_value_in_each_form` (its `real` message is dirty) |
| d3 same, lossy text appended to `E-BAD-TEXT` only | the same two (the patched copy `bad_text_value`, `{Text}`) |
| S-3 a: `W-VALUE-NULLED` of a chain at the piece offset again | `t12_offsets::one_chain_value_reports_its_start_under_every_policy` |
| S-3 f: no "piece n" in the `E-BAD-TEXT` message | that test and `t6_patched::a_broken_utf8_sequence_in_a_chain_is_a_value_error` |
| S-3 old `reader.rs` from `HEAD` | all three `t12_offsets` tests (24926 / 9000 / 33374 instead of the value start) |
| b `.warning` of the strict rounding removed | `t12_warning_field::a_strict_round_error_...` and `t11_strict::strict_over_the_201_files_...` |
| b2 `.warning` of strict + decode removed | `t12_warning_field::a_strict_lob_error_...` |
| e message without the set id | `t12_messages::the_lob_unconfirmed_message_names_the_database_set_id`, `t6l_ru::another_single_byte_database_set_keeps_the_stop` |
| c survey: rounded-to-zero not counted | `t12_survey` (2 tests) |
| g survey: integer part too large counted as exact | `t12_survey::the_number_profile_of_t_num_column_v` |
| h `oradump-py`: `next_batch` without `Python::detach` | `test_threads::test_two_threads_run_in_parallel` (ratio 2.0 against 0.98 to 1.08) |

**Deviations and problems (please confirm or correct)**
- **D1 arrow 60 -> 59 in `oradump-core/Cargo.toml`.** `pyo3-arrow` 0.19 / 0.20 (the pair ADR-3 and arch 9 name; 0.20 is the newest, it needs
  pyo3 0.30) depends on **arrow ^59**; the core was on arrow-array / -buffer / -schema 60, so a `RecordBatch` of the core is another type
  than the one `pyo3-arrow` takes. Arch 9 says the versions follow `pyo3-arrow`; I set the core to **59** (59.3.1). All 376 tests of the
  core passed unchanged at 59 (ADR-18's builder behaviour, verified on 60.0.0, holds on 59.3.1 as far as the suite shows). Alternative if the
  Architect prefers arrow 60: do the hand-over with arrow's own FFI (`FFI_ArrowArrayStream`) and drop `pyo3-arrow`; no `unsafe` needed.
  `Cargo.lock` changed accordingly. pyo3 0.30.1, pyo3-arrow 0.20.0 (ADR-3 says 0.29 / 0.19, read on docs.rs on 2026-10-07).
  ADR-3's "to verify": `PyRecordBatch::into_pyarrow` (pyarrow >= 14) and `PySchema::into_pyarrow` work for single batches;
  `PyRecordBatchReader::__arrow_c_stream__` is private in the crate, so the binding calls it through Python (`call_method1`).
- **D2 pyo3 and `forbid(unsafe_code)`.** The new crate uses the workspace lints (`unsafe_code = "forbid"`, no `unwrap`, no indexing, no
  unchecked arithmetic): PyO3 0.30's macros are written for it, clippy is clean. No `unsafe` anywhere.
- **D3 `Error::warning` completed (ADR-41 item 1, "T-12 builds the missing settings").** T-11 set it only in `Error::from_warning`. Now also:
  `reader.rs` `field()`: an `E-NUMBER-OVERFLOW` of a column whose policy `strict` changed from `round` to `error`, **and** that `round` would have
  rounded (an integer part that does not fit stays without a warning), carries `W-NUMBER-ROUNDED`; `lob_begin`: `E-LOB-UNCONFIRMED` under
  `strict` + `unconfirmed_lob="decode"` carries `W-LOB-UNTESTED` (without `decode` it is the ordinary refusal of ADR-25: no warning, my reading).
  This changed an expectation in **`t11_strict.rs`** (`DOCUMENTED`: the two `T_NUM2` / `T_NUM3` entries now say `W-NUMBER-ROUNDED` in the warning
  column; comment above the table) and `Cow` forced `.clone()` in **`t11_codes.rs`** line 66. Both files belong to T-11 / T-11b (the first is in the
  uncommitted T-11b diff): the Reviewer should read those two edits.
- **D4 S-3 as built (`reader.rs`).** LONG chain: `E-BAD-TEXT`, `W-VALUE-NULLED` and `W-LONG-CHARSET` at `st.start`; the message of a decoder error
  names the piece when the value is not a single piece (`first && last`), "(piece n of the value)"; LOB: all reports at the offset `lob_start`
  received, "(chunk n of the value)" always (also for a one-chunk value; the plan says "more than one", a superset, no test depends on it).
  `W-LONG-LENGTH` was already at the start (T-9b). Test `t12_offsets::the_long_charset_warning_of_a_chain_is_at_the_value_start` is extra.
- **D5 S-1 thresholds (`t12_messages.rs`).** (a) raw fields of **4 bytes or more** (the plan says "every field"; shorter ones match any word), (b) 4-character
  windows of the lossy rendering, windows of digits only or of white space only left out (messages hold row numbers, set ids and piece numbers), (c) hex
  of fields of 2 bytes or more, maximal hex-digit runs of the message compared lower-cased, (d) cells of 4 characters or more, digit-only strings
  shorter than 6 and `<NULL>` left out. The fields include assembled LONG chains and LOB values (the check lends the LOB chunks to a sink). Messages are
  checked against the corpus of the **table** they belong to (the whole file for scan errors). The corpus has only **16 distinct message texts** (the
  messages are static), so the check chiefly guards new formatted messages; the three mutations show that it bites. The patched copy of the plan is
  `misc_exp_18c.dmp` T_FRAME row 7 column A (`NNNNNNNNNN`, byte 4 -> `ff`). `the_check_reports_a_value_in_each_form` has the self-test for (a) to (d).
- **D6 names after a failed scan.** `.table` / `.column` / `.part` are resolved from the catalog; a scan that fails has none (`.table` is `None`,
  `.table_ordinal` has the number). `.part` is `(partition, subpartition)` (names, `None` for an unpartitioned table), `.column` the declared
  column's name resolved from the file position through `PartInfo.file_order` (ADR-29). Extra attributes: `.table_ordinal`, `.type_code`.
- **D7 survey counting core (`survey.rs`).** Added: per column `len_min` / `len_max` / `len_total` (bytes of the stored value, a chain or LOB by its total)
  and `number: Option<NumberProfile>` (integers, fit Int64, 39/40-digit count, largest sig / frac / int, exp10 and leading-position range,
  per-candidate exact / rounded / rounded-to-zero / integer-too-large for Decimal128(38,10), Decimal256(76,38) and the resolved type if it is another
  decimal); `number_summary()` over a dump. Zero values are left out of the exp10 / leading-position ranges. A candidate classifies by the reader's
  own `to_decimal128` / `to_decimal256` with `Round` (so the two cannot differ); the Python `decimal` cross-check agrees on the fixtures (numbers above).
  The survey also prints `Catalog.item_counts.g1_fields` (owner Q-2 (3)). `python -m oradump survey FILE [--no-names] [--no-read]`: by default it also
  reads every table once with `on_value_error="null"` to list the warnings (counts and first offsets); `--no-read` skips that third pass.
- **D8 digest.** The "compares with the Rust results" of Done-when is `_core.digest` (FNV-1a over a canonical byte form of every cell, computed in Rust from
  the core's own batches without pyarrow) against the same form recomputed from the pyarrow tables (`tests/conftest.py`). It is test support, in the
  module `_core` but not in the public API.
- **D9 packaging files.** `pyproject.toml` has no `readme`, `license` or `license-files` (T-13, ADR-45). `[tool.ruff]` excludes `tests/golden` and
  `crates/oradump-core/tests/data` (the Implementer's generators, not mine, they would not pass the new rules); line length 120; `A001` / `A004`
  ignored (`oradump.open` is the public name). mypy runs on the package and the pytest files; pyarrow has no stubs (override `ignore_missing_imports`).
  `maturin develop` and `maturin build` work on the VM (debug); a release build and its speed are T-15.
- **D10 warnings in Python.** `Dump.__init__` issues the header warnings, the first scan issues the scan's, each exactly once; `BatchReader` issues the new
  ones when the iteration ends (`StopIteration` or an error). `OradumpWarning` instances carry `.detail` (the `DumpWarning`); every code maps to
  `OradumpWarning` (`WARNING_CLASSES`, walked against `_core.WARNING_CODES` like `ERROR_CLASSES` against `_core.ERROR_CODES`). `E-IO` is `OSError`
  (`FileNotFoundError` for a missing file, built from the errno). The `__arrow_c_stream__` path spends the reader; an error there arrives as the message
  `[CODE offset=0x..] ...` inside `pyarrow.ArrowInvalid` ("External error: ...").

**For ADR-28 (b) and ADR-19 (recorded here, the Architect folds them in; `test_consumers.py` asserts them for pyarrow 26.0.0 / pandas 3.0.6 and they also hold
for pyarrow 15.0.2 / pandas 2.2.3):**
(1) `Table.to_pandas()` of a Timestamp(Second) column with `truth_date` rows 4 and 5: dtype `datetime64[s]`, both `0001-01-01 00:00:00` and
`9999-12-31 23:59:59` intact (pandas 2.2.3 and 3.0.6). (2) `coerce_temporal_nanoseconds=True`: `ArrowInvalid: Casting from timestamp[s] to timestamp[ns]
would result in out of bounds timestamp` (also for the Microsecond TIMESTAMP column). (3) `timestamp_as_object=True` and `to_pylist()` give
`datetime.datetime(1, 1, 1)` and `datetime.datetime(9999, 12, 31, 23, 59, 59)`. (4) `truth_ts` row 5: TS0 and TS6 (Microsecond) are `9999-12-31 23:59:59` and
`...59.999999`, dtype `datetime64[us]`; TS9 (Nanosecond) is outside the nanosecond range and is `E-BAD-DATETIME`, NULL under `on_value_error="null"`.
(5) A NULLed value is `NaT` in pandas (`pd.NaT`) and `None` in `to_pylist()`. (6) **`pyarrow.parquet.write_table` does not accept Interval(MonthDayNano)**:
`ArrowNotImplementedError: Unhandled type for Arrow to Parquet schema conversion: month_day_nano_interval` (table `T_INTERVAL` of `misc_exp_18c.dmp`,
column IYM); Duration(Microsecond) (IDS) writes and reads back. So an ETL job that writes Parquet needs `interval_as="string"` (after the alpha, ADR-19)
or a cast for every INTERVAL YEAR TO MONTH column: the owner may want `interval_as` in the alpha.

**What T-13 must know.** The pytest files exist and are green (`tests/conftest.py` has `fixture()`, `patched`, `table_digest`); T-13 adds the truth-level
checks for all files beside them. `pyproject.toml` needs `readme`, `license`, `license-files` (ADR-45). The wheel tag from `maturin build` here is
`cp311-abi3-manylinux_2_34_x86_64`. `python -m oradump survey` reads the whole file two or three times (scan, counts, lenient read); `--no-names` hides user, file name,
table, column, partition names and the type text of a column the DDL parse did not understand. `crates/oradump-py/src/lib.rs` has the whole binding in ~400
lines; every long call is inside `Python::detach`.

**Answers (Architect, session 11).**
- D1 (arrow 59, pyo3 0.30.1, pyo3-arrow 0.20.0): accepted, ADR-3 session 11 with a bump rule; the FFI route was
  checked and is safe, but costs our own capsule code: kept as the fallback.
- D2: accepted (ADR-46 item 1). D3: confirmed, ADR-41 item 8 (`warning` set exactly when the error would have been
  that warning without `strict`); the `t11_strict.rs` / `t11_codes.rs` edits accepted.
- D4, D6, D7, D8, D9, D10: accepted (ADR-46). D4's "chunk n" for every LOB value: accepted, arch 7.4 updated.
- D5: thresholds accepted; Reviewer 3 adds mutations d4 to d6 (ADR-46 item 3).
- Survey default (lenient read on, `--no-read` to skip): kept (ADR-46 item 9).
- Findings for ADR-28 (b) / ADR-19: recorded there; Parquet's refusal of Interval(MonthDayNano) moves
  `interval_as` into the alpha (ADR-47, T-13).
- "What T-13 must know": taken into T-13's session 11 "Done when"; the wheel tag becomes `manylinux_2_28` (ADR-48).

## T-13 (Implementer, session 12): `interval_as`, Python truth tests, packaging

Built: (a) `interval_as` in the core (`types/map.rs` `IntervalAs`, `TypeOptions.interval_as`, i.e. `Options.types.interval_as`;
`types/resolve.rs` the table-fallback warning; `reader.rs` `Kind::IntervalYmText` / `IntervalDsText`, metadata), the binding
(`oradump-py/src/options.rs`, `python/oradump/_api.py` `open(..., interval_as=)`, `_core.pyi`), `crates/oradump-core/tests/t13_interval.rs`
(5 tests), `tests/test_interval.py` (8), `tests/test_truth.py` (3, one per folder); (c) to (h) `Cargo.toml` / crate `license` fields,
`pyproject.toml`, `LICENSE-APACHE`, `LICENSE-MIT`, `deny.toml`, `README.md`, `CONTRIBUTING.md`, `tests/check_dist.py`.

**Results.** `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`: clean. `cargo nextest run`: **395 passed** (390 + 5). `ruff check`,
`ruff format --check`, `mypy` (strict): clean. `pytest`: **69 passed** (58 + 11) on 3.13 (VM `.venv`, pyarrow 26.0.0, pandas 3.0.6); the installed release
wheel in fresh venvs: Python 3.11.16 + pyarrow 26.0.0: 69 passed (and 68 passed + 1 skipped with pyarrow 15.0.2 / pandas 2.2.x, the skip is the
"native Parquet refusal observed with 26.0.0" test); Python 3.14.7 + pyarrow 26.0.0: 69 passed. The imports and the read of a fixture in those venvs
needed only pyarrow. **`cargo deny check licenses` FAILS, see P1.**

**Tools and numbers.** Expected interval texts are read from `truth_misc.txt` (both versions) and `truth_misc2.txt` (both versions) at test time (the
Analyst's `TO_CHAR` column); nothing is retyped except the fallback cases, whose texts derive from the same truth lines (`+0010 12:30:45.123456` ->
p = 2 digits needed, f = 9: `+10 12:30:45.123456000`). The Python truth test compares against `tests/golden/exp_catalog.tsv` (generated by
`gen_exp_catalog.py` from the Analyst's `exp_parse.py`): 201 `_exp_` files (18c 89, 11g 85, 11g-cp1251 27), 386 table sections (170 / 164 / 52), of
which 26 (13 + 13 + 0) are the DOCUMENTED failures (T_DATE, T_INTERVAL2, T_NUM, T_TS); every section is compared (read in full, or failed with its
documented code and compared again under `on_value_error="null"`), and the number of failures per table equals the catalog's count.

**Mutations** (each alone on the working tree, then restored): m1 fallback `f` 9 -> 8: `a_column_without_a_usable_declaration_...` fails; m2
`interval_text_warning` disabled: `a_table_fallback_warns_once_per_interval_column` fails; m3 `oradump:conversion` text changed:
`string_reads_equal_the_to_char_texts_of_the_truth_files` fails; m4 the descriptor mapping `182 | 183 -> Utf8` removed: both fallback tests fail;
conftest `_PRIME` changed: all 3 `test_truth.py` tests fail. `check_dist.py` on a tampered wheel (survey tool, `corpus/x.dmp` added, `LICENSE-MIT`
removed) reports 4 failures.

**Toolchain.** maturin **1.15.0** (pinned `maturin==1.15.0` in `[build-system]`; it writes `Metadata-Version: 2.4`, `License-Expression: MIT OR Apache-2.0`,
two `License-File` lines, so no newer maturin was needed), ziglang **0.17.0** (PyPI, installed in the VM `.venv`; maturin finds it only when
`.venv/bin` is on `PATH`: `PATH=$PWD/.venv/bin:$PATH maturin build --release --zig --compatibility manylinux_2_28`). Flag spelling is that of
`maturin build --help` of 1.15.0. Output: `target/wheels/oradump-0.1.0-cp311-abi3-manylinux_2_28_x86_64.whl` (3.6 MB) and `oradump-0.1.0.tar.gz`
(368 KB, 50 members). `tests/check_dist.py WHEEL SDIST` prints `check_dist: ok`. The wheel (`unzip -l`) holds `oradump/{__init__,__main__,_api,_core.abi3.so,
_core.pyi,_errors,_survey}` + `py.typed` and `oradump-0.1.0.dist-info/{METADATA,WHEEL,RECORD,licenses/LICENSE-APACHE,licenses/LICENSE-MIT,
sboms/oradump-py.cyclonedx.json}`. The sdist (`tar tzf`) holds `PKG-INFO`, `Cargo.toml`, `Cargo.lock`, `pyproject.toml`, `README.md`, both licence files,
`crates/oradump-core/{Cargo.toml,data/exp_intro_lines.txt,src/**}`, `crates/oradump-py/{Cargo.toml,src/*}`, `python/oradump/*` and nothing from `corpus/`,
`tests/`, `docs/`; a `cargo check --workspace --locked` of the unpacked sdist passes. Q-8 is answered in `owner-decisions.md` (line
`Copyright (c) 2026 Maksim Tkachenko`, no placeholder); Q-9 (RHEL 8+, x86_64) keeps `manylinux_2_28`. `LICENSE-APACHE` is the download of
apache.org (sha256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`).

**Problems and deviations (please confirm or correct)**
- **P1 `cargo deny check licenses` is red.** One crate outside the allow-list: `tiny-keccak 2.0.2`, `CC0-1.0`, reached through `arrow-array 59.3.1 ->
  ahash 0.8.12 -> const-random 0.1.18 -> const-random-macro -> tiny-keccak` (a proc-macro at build time). Per ADR-45 item 5 I added no `allow` line and
  no `exceptions`. Everything else passes; the warnings `license-not-encountered` for `ISC` and `Zlib` are only unused allowances. Proposal: the
  Architect decides between allowing `CC0-1.0` (a public-domain dedication, compatible with MIT / Apache) with a decision record, or an arrow feature set
  without `ahash`'s `const-random` (not checked; arrow 59 selects it).
- **P2 `InvalidOption`** in the plan is the existing `UsageError` (`E-CONFIG`); no new exception class. Python accepts exactly `"native"` and `"string"`
  (`"String"`, `""`, `"native "` are refused); the core's `IntervalAs::from_str` also trims and lowercases (used by Rust callers, tested).
- **P3 where the option lives.** The plan says `Options.interval_as`; it is a field of `TypeOptions` (`Options.types.interval_as`) because it decides the
  Arrow type, like `timestamp_unit`. No `ReadOptions` field.
- **P4 fallback text.** `p = the digits the value needs`: 1 for zero (`+0-00`, not `+00-00`), no minimum of 2; `f = 9` for DAY TO SECOND. On a table
  fallback every INTERVAL column gets its own `W-DDL-FALLBACK` (column = file position) beside the table-level one; a column whose own definition fell back
  has the warning it always had (no second one). Under `native` nothing changed.
- **P5 sdist contents.** `[tool.maturin] exclude` also removes `crates/*/tests/**` (the crates' integration tests read `corpus/`), so `cargo test` does not
  run from the sdist; the build does. `cargo` prints "manifest has no documentation, homepage or repository" for both crates while the sdist is listed (a
  warning only; `repository` is a field the owner can add).
- **P6 `sboms/oradump-py.cyclonedx.json`** is in `.dist-info` (maturin 1.15 default); inside `oradump-0.1.0.dist-info/`, so ADR-48 item 4 holds. It lists the
  Rust dependencies.
- **P7 version.** The wheel is `0.1.0` as the plan names it; the release tag `0.1.0a1` needs `version = "0.1.0a1"` in `Cargo.toml` (maturin converts it) or in
  `pyproject.toml`. The README says "alpha (0.1.0a1 when released)" and `pip install oradump` (not yet published). No tag was made.
- **P8 `ruff format`** also formats the Python block of `README.md` (the comment spacing there is ruff's).
- **P9 disk.** `/tmp` (tmpfs 3.9 GB) was 85% full from earlier sessions when T-13 started; the venvs for the 3.11 / 3.14 checks were built under
  `~/.cache/oradump-t13` and removed. The linker prints "ignoring deprecated linker optimization setting" from zig (warning of `oradump-py`, harmless).

**What the next task must know.** The release build command and `check_dist.py` are in `CONTRIBUTING.md`. T-14 / T-15 are unaffected (`interval_as` is
read at `open`; the default is `native`). Reviewer 3 reads `t13_interval.rs`, `test_truth.py` and the `reader.rs` hunk `Kind::IntervalYmText` /
`IntervalDsText`.

**Answers (Architect, session 12)**
- **P1** (ADR-49): `tiny-keccak` is reached only through `arrow-array`'s `cfg(target_arch = "wasm32")` dependency on
  `ahash` with `compile-time-rng`; on every other target `ahash` uses `runtime-rng`. No arrow feature to turn off.
  `deny.toml` gets `[graph] targets` (five triples, exact text in ADR-49); no `CC0-1.0` allowance, no exception.
  Checked on a scratch copy: `licenses ok`. Task T-13f.
- **P2** accepted (`UsageError`, `E-CONFIG`; Python exact, Rust `from_str` lenient). **P3** accepted
  (`Options.types.interval_as`). **P4** accepted (ADR-47's wording). **P5** accepted (the sdist builds, tests run from
  the repository; the "no repository" warning waits for a public URL). **P6** accepted (SBOM kept). **P8** accepted.
  **P9** noted. All in ADR-51.
- **P7** (ADR-50): Cargo cannot hold `0.1.0a1`; T-13f sets `[workspace.package] version = "0.1.0-alpha.1"` and
  `dynamic = ["version"]` in `pyproject.toml` (maturin 1.15.0 writes `0.1.0a1`, checked on a scratch crate), makes
  `check_dist.py` version-agnostic, adds `publish = false` to `oradump-core`, and rewrites the README install text
  (exact text in ADR-50). No tag by an agent; the release checklist is in the plan's "First release".
- **`.gitattributes`** (ADR-52): the Implementer adds it in T-13f, exact text in ADR-52.

## T-13f note (Implementer)

- Done as ADR-49 to ADR-52 say: `deny.toml` `[graph] targets`; version `0.1.0-alpha.1` (Cargo) / `0.1.0a1` (maturin,
  `dynamic = ["version"]`); `publish = false` in `oradump-core`; `tests/check_dist.py` without a literal version;
  README "Install" / "Other platforms"; CONTRIBUTING "Release wheel"; `.gitattributes`.
- `cargo deny check licenses`: `licenses ok` (warnings only: unmatched allowances such as Zlib; no `exceptions`).
- `git diff Cargo.lock`: two lines, the versions of `oradump-core` and `oradump-py` (`0.1.0` -> `0.1.0-alpha.1`).
- Built `target/wheels/oradump-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl` and `oradump-0.1.0a1.tar.gz`;
  `check_dist: ok`; wheel `METADATA` has `Version: 0.1.0a1`. A copy renamed to `...0a2...` fails (13 FAIL lines).
- **Problem 1**: `python/oradump/__init__.py:38` has `__version__ = "0.1.0"`; the task forbids changes under
  `python/oradump/`. Proposal: T-14 or a small task sets it to `"0.1.0a1"` (or reads `importlib.metadata`); a pytest
  should compare it with the installed metadata. No test checks it today.
- **Problem 2**: `maturin build --zig` printed "Failed to find zig" with `.venv/bin` not on `PATH` (`ziglang` is installed in
  the venv, `.venv/bin/pip` does not exist). It works with `PATH=$PWD/.venv/bin:$PATH`. CONTRIBUTING's command
  needs an activated venv; propose saying so.

### Answers (Architect, session 13)

- **Problem 1** (`__version__ = "0.1.0"`): ADR-54 item 7. `__version__` reads `importlib.metadata.version("oradmp")`,
  `"0+unknown"` without metadata; `tests/test_version.py` compares it with the metadata and the root `Cargo.toml`
  version. Done in T-13g (e), together with the rename, since the file moves to `python/oradmp/__init__.py` anyway.
- **Problem 2** (zig not found without `.venv/bin` on `PATH`; no `.venv/bin/pip`): ADR-54 item 8. The release block of
  `CONTRIBUTING.md` starts with `source .venv/bin/activate` and installs with `uv pip`; the exact text is T-13g (f).

## T-13h note (Implementer, session 15): review 03 fixes (ADR-55 to ADR-60)

All of (a) to (g) are built. Counts: `cargo nextest run` 402 tests (395 + 4 `guard` unit tests + `t11_codes::e_internal` + 2 new in
`t12_messages`), `pytest -rs` 105 passed, **0 skipped** (73 + 32: `test_stream` 8, `test_internal` 3, `test_check_dist` 17,
`test_warnings` +4). `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` (also `--all-features`), `ruff check`,
`ruff format --check`, `mypy` (20 files), `cargo deny check licenses` (`licenses ok`) and `sha256sum -c SHA256SUMS` in the three
fixture folders are green. Dev build: `maturin develop --features test-probes`.

### Deviations and things ADR-55 to ADR-60 did not foresee

1. **ADR-55 items 5 and 6, "the text starts with `[E-CONFIG`"**: arrow-rs prefixes every `ArrowError::ExternalError` with
   `External error: ` (`arrow-schema` `Display`; no variant prints a bare message). pyarrow raises `ArrowInvalid` with
   `External error: [E-CONFIG] the reader is closed`. The code and offset survive, but the text does not *start* with
   `[E-`. The tests (`test_stream.py`, `test_internal.py`) strip that prefix and assert `startswith("[E-...")` after it.
   Proposal: arch 3.4 and ADR-55/59 say "contains / starts after the prefix `External error: `".
2. **ADR-55 item 5, the iterator after `close()`**: before, `next(r)` after `close()` silently ended (a short table without an
   error). The shared state now answers the same on both paths: after `close()` a pull is `E-CONFIG` ("the reader is closed")
   unless the source had already returned its end (then it is the end). This extends item 5 from the stream to the iterator.
3. **ADR-59 item 5 (b)**: `_probe_panic(kind, path=None)`, not `_probe_panic(kind)`. `dump` needs a file to open, and the
   "unusable afterwards" check needs the object, so the function returns a `_Dump` (kind `dump`, then call
   `d._probe_panic()`, a feature-gated method that panics through `PyDump::with`) or a `_Reader` over a panicking
   `BatchSource` (kinds `reader` and `stream`). `SharedBatches` is not generic: it holds `Shared`, and the reader state holds a
   `Box<dyn BatchSource>` (the small trait of the ADR), which is the same effect.
4. **ADR-58 item 4**: the extended self-test is split: `the_check_reports_a_value_in_each_form` (now also the hex prefix forms
   of (c)), `the_check_reports_a_decoded_value` (form (e): the four spellings of row 19, the 6- and 5-digit windows of row 22, the
   date text of truth_date row 8), `the_helpers_of_the_decoded_forms` (canonical form, number tokens, joined pairs). The values come
   from `truth_num.txt` / `truth_date.txt` through `common::truth`, not retyped. Form (e) for the TIMESTAMP family (180, 181, 231)
   reads the 7 date bytes with `decode_date_fields` (the family's decoders use the same routine but refuse a year below 1, which is
   the value to catch).
5. **A number token with an exponent inside a hex-looking text** (`04e4`) is a form (e) hit by the rule (it is the number
   4E4, which can be a stored number). Tokens inside a word (`0x1e5`) are skipped. No message of the fixtures has either.
6. **Inventory (ADR-58 item 3)**: the longest digit run over the 201 files' messages is at most 5, asserted; it holds today.
7. **Lock rule (ADR-55 item 7)**: every lock of `PyDump` and `PyReader` is taken inside `py.detach`; `Dump.info`, `.scan`
   and `.warnings` copy out (`DumpInfo.clone()`, `copy_catalog`, `to_vec`) and build the Python objects after the lock is
   released. `Catalog` is not `Clone` (its stop error holds an `io::Error` slot), so the binding copies it field by field
   (`copy_error` drops the source, which a stop error never has). There is **no deterministic test** for "no lock held while Python is
   called"; it is by construction (code review). The only runtime check is the two-thread record below.
8. **ADR-60 N-4, t12_offsets**: the neighbours of the dropped `let _ = place(&e)` already assert offset, column and row, so
   the new line asserts what none of them does: `place(&e).2 == n[0].table` (mutation n4c below).
9. **Release build**: `tools/release_build.sh` also accepts `-h`. The `--twice` run deletes `$ROOT/target` before the second
   build (from scratch, same paths, so the remaps give the same bytes). The check refuses a root with the user's name or `$HOME`
   as a component. `uv pip install` and the crates need the network.
10. `~/.cache/oradump-t12` did not exist (nothing to delete); `~/.cache/oradump-t13` exists and is not named by N-8: the owner may
    delete it. `python/oradmp/_core.abi3.so` and every `__pycache__` were rebuilt / deleted by `maturin develop`.
11. A `check_dist.py` run on a wheel built with the feature in a *debug* profile prints one FAIL per home path in the huge
    `.so` (hundreds of lines) besides the `_probe_panic` line; on a release build from `release_build.sh` there are none.

### (b) Release build, run once on a scratch commit of the working tree

Run on a **scratch commit** of the T-13h working tree (the agent does not commit, so the "T-13h commit" does not exist yet; the
scratch repository is a `git clone` of this one plus one commit holding the uncommitted files) with
`ORADMP_BUILD_ROOT=/var/tmp/oradmp-release-t13h tools/release_build.sh --twice`. Commit `25bfb9317be18547326e862fd387d0b22a5c87b5`
(scratch), rustc 1.99.0 (b940084d7 2026-09-28), maturin 1.15.0, zig 0.17.0, SOURCE_DATE_EPOCH 1792354475. These files are **not** the
release (the release is built in checklist step 5 on the tagged commit).
- `check_dist: ok` for both files. Wheel `oradmp-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl` (3,624,865 bytes) SHA-256
  `03bea57e477f5941de1f8abb64e58fed3fa6755e1b94f6a47bbfec90b94358bc`; sdist `oradmp-0.1.0a1.tar.gz` (150,651 bytes) SHA-256
  `14403f6a6e677594a76171507e1298fc8f403273db11c63e7bc0ead81a178c06` (hashes belong to the scratch commit).
- `strings -a` of the wheel's `_core.abi3.so`: **0** hits for `/home/`, **0** for `<home>`, 0 for `/var/tmp`, 0 for `_probe_panic`;
  dependency paths read `/cargo/registry/src/...`. The SBOM text: **0** hits for `/home/`; its two `path+file://` references are
  `path+file:///var/tmp/oradmp-release-t13h/src/crates/oradmp-core` and `.../oradmp-py` (the neutral root). The SBOM `timestamp`
  is `2026-10-18T20:14:35.000000000Z`, i.e. it follows `SOURCE_DATE_EPOCH` in maturin 1.15.0 (checked, ADR-56 "not verified" item).
  `[tool.maturin.sbom] rust = false` was not needed.
- `--twice`: **identical** wheel and identical sdist (same SHA-256 from a second build from scratch): the build is reproducible
  on this host. Not a release gate.

### Mutations (each alone, on a scratch copy of the working tree, `git checkout` between runs)

| id | mutation | failing test (first failure, `-x`) |
|---|---|---|
| s1a | `arrow_c_stream` hands the stream a private state taken out of the reader (the old stash) | `test_stream.py::test_the_stream_path_reports_what_the_iterator_path_reports` |
| s1b | `BatchReader._close` does not call `_issue_new` | `test_stream.py::test_the_warnings_of_the_stream_path_are_issued_once_at_the_end_of_the_with` |
| e1 | no `catch_panic` in `PyDump::with` | `test_internal.py::test_a_panic_through_the_dump_guard_...` |
| e2 | no `catch_panic` in `PyReader::next_batch` | `test_internal.py::test_a_panic_through_the_batch_path_...` |
| e3 | no `catch_panic` in `SharedBatches::next` | `test_internal.py::test_a_panic_inside_the_arrow_stream_...` (subprocess exit status != 0) |
| e4 | `catch_panic` puts the panic payload text in the message | core `guard::tests::a_panic_becomes_the_fixed_internal_error` and `a_panic_with_a_formatted_payload_is_dropped_too` |
| e5 | a wheel built with `--features test-probes` (debug) | `check_dist.py`: `FAIL: wheel member oradmp/_core.abi3.so: contains _probe_panic (built with --features test-probes)` |
| f1 | `stacklevel=up + 2` | `test_warnings.py::test_every_issuing_call_points_the_warning_at_the_users_line[next / read_all / close / with]` |
| n4a | the binding names column `c + 1` in an error | `test_errors.py::test_the_dump_stays_usable_after_a_strict_error_from_read_table` (the old `e.column is not None` would pass) |
| n4b | golden row count of `18c/t_num_exp_18c.dmp` T_NUM 33 -> 32 (the golden file, on the copy) | `test_truth.py::test_every_table_of_every_exp_file_equals_the_rust_read[18c]` (the old `or policy == "null"` could not fail) |
| n4c | counted warnings are attributed to table + 1 | `t12_offsets::one_lob_value_reports_its_start_under_every_policy` |
| c1 | `forbidden` tests only the first path component | `test_check_dist.py`: nested corpus (sdist), nested tests / docs / target / .venv / .git |
| c2 | the `.pyc` / `.pyo` suffix rule off | `test_a_pyo_in_the_wheel_fails` (a `.pyc` in `__pycache__` is also caught by the folder rule) |
| c3 | `tools` not a forbidden component | `test_a_tools_folder_in_the_sdist_fails` |
| c4a..c4f | `/home/`, `/Users/`, `/root/`, `C:\Users\`, `C:\\Users\\` each off; the Windows search case-sensitive | `test_a_home_path_in_the_sbom_...`, `test_a_users_path_inside_a_binary_member_fails`, `test_a_root_home_in_the_sdist_fails`, `test_a_windows_home_fails_plain_escaped_and_in_any_case[...]` |
| c5, c6 | content scan off for the wheel / for the sdist | the home-path tests above (wheel) / `test_a_root_home_in_the_sdist_fails` |
| c7 | the `_probe_panic` rule off | `test_the_test_probe_in_the_wheels_extension_module_fails` |

Message mutations (`reader.rs`, the `Err(e)` arm of `field`; test `no_message_of_the_fixtures_holds_a_value`; the forms are the
ones the assertion printed; Hex is (c), Decoded is (e)):

| id | mutation | forms reported (first message) |
|---|---|---|
| d1 | lossy text of the bytes in every non-text value error | {Text}: `months do not fit 32 bits ....G` |
| d2 | hex of the bytes in every value error | {Hex}: `months do not fit 32 bits bb9ac9ff47` |
| d3 | lossy text in `E-BAD-TEXT` | {Text}: `text is not valid UTF-8 NNNN.NNNNN` |
| d4a | the cell `1234567.0000000000` appended to `E-NUMBER-OVERFLOW` | {Cell, Decoded} |
| d4b | the failing value's `to_text()` (10^33 as 34 digits) in `E-NUMBER-OVERFLOW` | {Cell, Decoded} (not by coincidence: (e) sees it) |
| d4c | the failing value as `{:E}` (`1E33`) | {Decoded} |
| d5 | the first 4 bytes of the field in `E-BAD-TEXT` | {Text}: `NNNN` |
| d6 | hex of the first 8 bytes of a field longer than 8 | {Hex}: `c7c70c1f183c3c3b` |
| d7 | the same with spaces between the pairs | {Hex}: `c7 c7 0c 1f 18 3c 3c 3b` |
| d8 | the decoded date text in `E-BAD-DATETIME` | {Decoded}: `-0100-06-15` (truth_date row 8) |

### Two-thread record (not asserted)

Dev (debug) build, `part_exp_18c.dmp` T_PART, 60 reads per thread, `two / one` over 5 runs: iterator `read_all()` 1.08, 0.97, 0.98, 0.71,
0.99; `pyarrow.RecordBatchReader.from_stream(r).read_all()` 1.04, 0.95, 0.97, 0.98, 1.09. No serialisation on the stream path.

### For the next tasks

- T-14: the core's `error.rs` (`ErrorCode::Internal`, `ALL` has 25 codes; `t11_codes` counts 44 codes) and `lib.rs` (`pub mod guard`) changed;
  the fuzz targets call the core directly, where a panic is a crash. `Cargo.lock` did not change.
- The release: build with `tools/release_build.sh <tag-commit>`, not from the working copy. Reviewer 4 should read the `External error: ` point (1)
  and the iterator-after-`close()` choice (2).
- Not done: a pyarrow 15.0.2 run of `test_interval.py` / `test_consumers.py` (N-5 relies on the recorded observation); only pyarrow 26.0.0
  was available in `.venv`.

### Answers (Architect, session 15)

- **Deviation 1** (`External error: ` before every stream error): accepted, ADR-61 item 1. Arch 3.4, ADR-55 items 5, 6
  and ADR-59 item 2 now say the text **contains** the code and offset; a consumer finds the code by the pattern
  `\[(E-[A-Z0-9-]+)`. No wrapper (it would need `unsafe` FFI or the Python wrapper ADR-55 rejected). The tests'
  `removeprefix` stays. T-14 (i) adds the README sentence of ADR-61 item 2. New rule from the check: no message holds
  U+0000 (arrow-array's export would panic on it in the C callback; ADR-61 item 3, T-14 (c), (h)).
- **Deviation 2** (the iterator raises `E-CONFIG` after `close()`): confirmed, ADR-61 item 4; ADR-55 item 5 amended.
- **Item 3** (`_probe_panic(kind, path=None)`, returns the object; `Box<dyn BatchSource>`): accepted, ADR-59 item 5 (b)
  amended.
- **Item 4** (the self-test in three tests): accepted, ADR-58 item 4 amended. **Item 5** (`04e4` / `0x1e5`): accepted.
- **Item 7** (no test for "no lock held while Python runs"): no test needed; Reviewer 4 checks the code against the
  three-part rule of ADR-61 item 7.
- **Not done: pyarrow 15.0.2**: release checklist step 7 (Python 3.11 venv with pyarrow 15.0.2, pandas 2.2.3) runs it;
  the release note names `test_interval.py` and `test_consumers.py` (ADR-61 item 8).
- **Item 10**: `~/.cache/oradump-t13` may be deleted by the owner.
- **For the next tasks**: T-14 and T-15 are rewritten (ADR-62); they run one after the other, each alone, under the
  machine rule (one build at a time, `CARGO_BUILD_JOBS=2`, `cargo nextest run -j 2`).

## T-14 note (Implementer, session 16): hardening, property tests, fuzzing

Zero findings: the three targets ran 10 minutes each on the final code with 0 crashes, 0 timeouts, 0 out-of-memory, so
`t14_fuzz.rs` and `tests/fuzz/` are **not created** (T-14 (e)). No file of `crates/oradmp-core/src/` changed. No deviation from
ADR-61 or ADR-62 that changes a rule; the points the ADRs did not foresee are under "Problems" below.

### (a) Toolchain
- `rustup toolchain install nightly-2026-10-15 --profile minimal` -> `rustc 1.101.0-nightly (7df81381d 2026-10-14)`; pinned in
  `fuzz/rust-toolchain.toml` (`channel = "nightly-2026-10-15"`). The root `rust-toolchain.toml` (1.99.0) is unchanged.
- `cargo install cargo-fuzz --locked --version 0.13.2`; `cargo fuzz --version` run in `fuzz/`: `cargo-fuzz 0.13.2`. `c++` 14.2.
- Machine: Intel Core i9-9900T, `nproc` 8, 11 GiB; every command with `CARGO_BUILD_JOBS=2`, nextest `-j 2`, the fuzz runs alone.

### (b), (c) `fuzz/`
- Files: `fuzz/Cargo.toml` (own `[workspace]`, package `oradmp-fuzz`, deps `libfuzzer-sys 0.4` and `oradmp-core` by path),
  `fuzz/Cargo.lock` (started as a copy of the root lock so that arrow, thiserror etc. have the same versions; cargo added
  `libfuzzer-sys` 0.4.13 and `arbitrary`), `fuzz/.gitignore` (`corpus/ artifacts/ coverage/ target/`), `fuzz/rust-toolchain.toml`,
  `fuzz/src/lib.rs` (the shared checks `check_error`, `check_warning`, `checked`), `fuzz/fuzz_targets/{walk,read,values}.rs`,
  `tools/fuzz_seeds.sh`.
- `tools/fuzz_seeds.sh` prints **235**: all `.dmp` files of `corpus/fixtures/{18c,11g,11g-cp1251}` (201 exp files and 34 Data Pump
  files; the Data Pump files are not exp files and only give `E-NOT-EXP-FILE`, they are seeds for the header check). The three
  folders hold files of the same names, so a seed is named `<folder>__<name>`.
- The checks (a failed one is a `panic!`): the `Display` starts with `[E-` (warnings `[W-`); the code is in `ErrorCode::ALL`
  (`WarningCode::ALL`); the core never built `E-INTERNAL`; an offset is at most the input length; no U+0000. `read` checks
  `Dump::warnings()` after `open` and after `scan`, the stopped-catalog error, and `TableBatches::warnings()` after each table.
  `walk` checks `WalkReport::warnings` and a `WalkEnd::Stopped` error. `values` selects by `first byte % 9`: NUMBER (with
  `to_text`, `to_i64`, `to_f64`, `to_decimal128(38,10)`, `(10,0)`, `to_decimal256(76,38)` under all three policies), DATE,
  TIMESTAMP 180, TSTZ 181, TSLTZ 231, INTERVAL YM, INTERVAL DS (with all conversions), ROWID, UROWID.
- `git diff --stat Cargo.lock` (root): empty at the end. `cd fuzz && cargo fuzz build`: builds the three targets.
- The fuzz profile keeps `debug-assertions` and `overflow-checks` on in `[profile.release]` of `fuzz/Cargo.toml`, so an arithmetic
  overflow that the release library would wrap is a crash too (the core's lints forbid unchecked arithmetic, this is the
  runtime check of that rule). Not in the ADR; it costs speed, not coverage.

### (d) Runs (final code, each alone, command as in the plan, from the repository root)
`RUSTUP_TOOLCHAIN=nightly-2026-10-15 TMPDIR=/var/tmp/... cargo fuzz run <t> fuzz/corpus/<t> target/fuzz-seeds -- -max_total_time=600
-max_len=1048576 -timeout=10 -rss_limit_mb=2048 -malloc_limit_mb=64 -jobs=1 -workers=1`, corpus folders empty at the start.

| target | executions | exec/s (end) | final `cov:` | final `ft:` | final `corp:` | crashes / timeouts / OOM |
|---|---|---|---|---|---|---|
| `walk` | 775,052 | 1,289 | 1,725 | 7,611 | 614 (41 MiB) | 0 / 0 / 0 |
| `read` | 446,605 | 743 | 5,673 | 22,478 | 1,253 (79 MiB) | 0 / 0 / 0 |
| `values` | 71,368,246 | 118,749 | 1,094 | 2,015 | 282 (3,301 bytes) | 0 / 0 / 0 |

`fuzz/artifacts/` is empty after the runs. Earlier runs (smoke runs of 20 s `walk`, 90 s `values` and `read`, before the damage
tests were written): no finding. After the 30 minutes of runs only `rustfmt` was applied to the three target files and
`lib.rs` (whitespace, no token changed); `cargo fuzz build` was repeated and builds.

### (e) Findings
None. No regression test file exists for them. A `proptest-regressions` file written once by the mutation run `d-off` (below)
was deleted: that failure was caused by the mutation, not by the code.

### (f) Damage properties (`crates/oradmp-core/tests/t14_damage.rs`)
- `truncation_gives_ok_or_a_typed_error`, `bit_flips_give_ok_or_a_typed_error` (proptest default 256 cases, honours `PROPTEST_CASES`),
  plus `the_golden_catalog_has_the_201_files` (201 files, 386 sections) and `the_checks_fail_on_a_wrong_cap_and_a_wrong_bound`
  (the check really fails on a row count one above the cap and on an offset beyond the bound).
- Rows per ordinal are capped by `num_rows` of `tests/golden/exp_catalog.tsv`; errors are checked for `[E-`, code in `ALL`, no NUL
  and the offset bound (t for truncation, the file length for flips); warnings of every table read are checked for `[W-`.
- `PROPTEST_CASES=10000 cargo nextest run -j 2 --test t14_damage`: both properties pass; truncation 5.0 s, bit flips 15.4 s; failures 0.
- Mutation `d-off`: `io.rs` `truncated()` with `.at(self.offset().saturating_add(1))` fails `truncation_gives_ok_or_a_typed_error`.

### (g) N-9 exact counts (current line numbers; sources; a mutation each; "old" = the loose bound that passed it)
| place (review 03 line -> now) | value | source | mutation that fails it, old bound passes it |
|---|---|---|---|
| `t3_catalog.rs:960` -> `:963` `compared` | 379 | 386 sections (`exp_catalog.tsv`) - 4 partitioned (its column `partitioned`) - 3 failed exports (`grep -a "exporting table" corpus/fixtures/*/*_exp_*.log` without `rows exported`, apart from the 4 partition headers: `???_???` twice, `\xbf\xbf\xbf_\xbf\xbf\xbf` once) | `compared += 1` -> `compared += usize::from(t.num_rows > 0)` |
| `t3_catalog.rs:963` -> `:966` `failed_tables` | 3 | the same three log lines | (not run separately: the mutation above and this assertion are in the same test) |
| `t8_direct.rs:164-165` -> `:168` `tables` | 141 | python over `exp_catalog.tsv`: sections of the 80 conventional partners of the `*_exp_direct_*` files (the direct files hold 143, the two `T_LONG_EMPTY` more) | `tables += 1` -> `tables += usize::from(name != "T_LONG")` |
| `t8_direct.rs` -> `:169` `rows_total` | 16,461 | python: sum of `num_rows` over those sections (the direct side has the same 16,461) | `rows_total += row_count(&ba)` -> `.saturating_sub(1)` (old bound 10,000 passes with 16,320) |
| `t2_manifest.rs:336` -> `:338` LOB parts | 21 | python over `tests/golden/exp_raw.tsv`: parts with `lob_values` > 0 (= `TOTALS.lob_sections`) | `lob_values > 0` -> `> 2` |
| `t2_manifest.rs:337` -> `:339` LONG parts | 16 | python over `exp_raw.tsv`: column `long` != `-` | `l.long != "-"` -> `... && l.rows > 0` |
| `t2_manifest.rs:441` -> `:445` `compared` | 152 | python: of the 386 sections of `exp_raw.tsv`, 152 have a table name among the 16 lines of `corpus/fixtures/18c/truth_counts.txt` | `compared += 1` -> `compared += usize::from(rows > 0)` |
| `t1_walk.rs:808` -> `:812` | offset 8476 (0x211c), length 2606 | `create_off` 211c of T_WIDE in `exp_catalog.tsv`; python: the line from 8476 to the LF is 2,606 bytes and its SHA-256 equals the catalog's `ddl_sha256` (the length is what is seen when the limit is crossed; the default buffer holds the line) | `io.rs` `.with_length(to_u64(new_len))` -> `to_u64(max).saturating_add(1)` (old: `length > 200` passes with 201) |
| `t5f_followup.rs:240-273` -> `:275` | 4 TSLTZ columns | `collist.py` tally "4 TIMESTAMP (n) WITH LOCAL TIME ZONE" (and "1907 column definitions", "386 CREATE TABLE lines") | the folder list `["18c", "11g", "11g-cp1251"]` -> `["18c"]` (old `columns > 0` passes) |

All nine mutations were run alone and each fails the named test (`t3_catalog::x_row_counts_of_the_catalog_equal_the_rows_exported_lines_of_the_logs`,
`t8_direct::every_direct_file_equals_its_conventional_partner` x2, `t2_manifest::x_walker_with_hashing_reproduces_every_line_of_the_manifest` x2,
`t2_manifest::x_rows_per_table_equal_truth_counts_where_the_table_is_listed`, `t1_walk::i2_a_line_beyond_...`, `t5f_followup::every_tsltz_column_...`);
the first run of `lob_values > 1` did not fail (21 parts have 2 or more LOB values), so `> 2` is the mutation. The restored files are green.
`failed_tables` has no separate mutation: it shares the test with `compared`.

### (h) Interval and messages
- `t13_interval.rs::day_to_second_opposite_signs_stay_e_bad_interval_under_string`: row 1 IDS of `18c/misc_exp_18c.dmp` with the hours
  byte 0x48 (+12) set to 0x30 (-12) beside +10 days: `E-BAD-INTERVAL`, row 1, file column 3. Mutation `h-ds` (`interval.rs`, the DS `same_sign`
  condition made `false && ...`) fails exactly this test. The comment at `t13_interval.rs:45` now says four rows.
- `t12_messages.rs`: `Msg` carries the `Display` text; `no_message_of_the_fixtures_holds_a_value` asserts no U+0000 in it for every error and
  warning of the 201 files, the three LOB copies and the bad-text copy (over the four policies); `the_nul_check_fails_on_a_nul_and_the_texts_are_the_display_texts`
  shows the check fails on a NUL. Mutation `h-nul` (a `\u{0}` appended to the message "months do not fit 32 bits") fails
  `no_message_of_the_fixtures_holds_a_value`.

### (i) README
The sentence of ADR-61 item 2 stands after the stream code block, before "After the export the reader is spent...". Line breaks are
the ADR's (the break inside the code span `[E-FRAMING offset=0x1a2b]` is the one the ADR has).

### (j) Green
`sha256sum -c SHA256SUMS` in the three folders: 0 failures. `cargo fmt --check`: ok. `cargo clippy --all-targets -- -D warnings` and
`--all-features`: ok. `cargo nextest run -j 2`: **408 passed**, 0 skipped (402 + 4 damage + 1 interval + 1 messages). `cargo deny check licenses`: ok.
`ruff check`, `ruff format --check`, `mypy`: ok. `pytest -rs` after `maturin develop --features test-probes`: 105 passed, 0 skipped.
Root `Cargo.lock`: unchanged.

### Problems and things the ADR/plan did not foresee
1. **`/tmp` is a 3.9 GiB tmpfs and was full** (100%, mostly `/tmp/claude-1000`, not mine): `cargo install` failed with "No space left on
   device". All commands ran with `TMPDIR=/var/tmp/...`. The `read` target uses `std::env::temp_dir()`, so fuzz runs need a `TMPDIR` with room;
   it leaves its per-process folder `oradmp-fuzz-<pid>/` behind (libFuzzer ends the process without a destructor); I removed them by hand.
2. **`cargo fuzz run` from the repository root uses the root pin (1.99.0)** and fails with "`-Z` only accepted on the nightly compiler".
   The plan's command works from the root with `RUSTUP_TOOLCHAIN=nightly-2026-10-15` in the environment (as run), or from `fuzz/` with the corpus
   paths `corpus/<t> ../target/fuzz-seeds`. `cargo fuzz build` works from `fuzz/` as is. The Architect may want the plan or CONTRIBUTING to say so.
   Also, `-jobs=1` makes libFuzzer write `fuzz-0.log` into the working folder (the repository root when run from there); I deleted it.
3. The seeds include the 34 Data Pump `.dmp` files (the plan says "the `.dmp` files of the three folders"): kept, they cost nothing.
4. `Warning` offsets are checked against the input length too (the plan names it for errors); no warning violated it.
5. Not done in T-14 (not asked): clippy of the `fuzz/` crate (needs the nightly lints; the crate has no workspace lints), and the memory bound
   "never allocates more than a fixed bound" of arch 8.3 (a) is held only by the libFuzzer limits of the `read` target (`-malloc_limit_mb=64`), as ADR-62
   rejects an allocation counter in the core tests.

### For the next tasks
- T-15: nothing in `crates/oradmp-core/src/` changed, so the baseline is the code of `39cbb93` plus tests. Root `Cargo.lock` is untouched, T-15 adds `criterion`.
- The `PROPTEST_CASES=10000` run takes 20 s; a nightly job may use 100000.
- The fuzz corpora (`fuzz/corpus/`) are not committed; the next run starts again from the seeds.

## T-15 note (Implementer, session 17): benchmarks, throughput, memory

No file of `crates/oradmp-core/src/` changed (baseline = code of `abd42ad`/`39cbb93` plus tests). The numbers below are a
baseline taken on one machine; the Architect copies them into `docs/design/benchmarks.md`.

### Machine
Intel Core i9-9900T @ 2.10 GHz, `nproc` 8, 11 GiB (`free -m` before the runs: 1.7 GiB free, 8.9 GiB available, swap 6.5 GiB of
which 4.6 GiB used by other things), `rustc 1.99.0 (b940084d7 2026-09-28)`, release profile of the workspace (no `lto` /
`codegen-units` change). Every command with `CARGO_BUILD_JOBS=2`; benchmarks and measurements ran alone. The files of
`ORADMP_WORK` (folder `18c/`: `t_rows_exp_18c.dmp` 10,035,200 bytes, `t_rows_exp_direct_18c.dmp` 9,838,592 bytes,
`t_rows_dp_18c.dmp`, `truth_rows.txt`) were in the page cache (they had just been read), so the scan figures are memory
speed, not disk speed. `T_BIG` does not exist: its items are not done (the plan says "if present").

### (a) criterion
- `crates/oradmp-core/Cargo.toml`: `criterion = { version = "=0.8.2", default-features = false, features = ["cargo_bench_support"] }`
  (no plotting, no rayon) and `[[bench]] name = "decode" harness = false`.
- Root `Cargo.lock` diff: **only additions, 261 lines**: aho-corasick, alloca, anes, anstyle, cast, ciborium, ciborium-io,
  ciborium-ll, clap, clap_builder, clap_lex, criterion, criterion-plot, either, itertools, itoa, oorandom, page_size, regex,
  regex-automata, same-file, serde_json, tinytemplate, walkdir, winapi (+ i686-pc-windows-gnu, x86_64-pc-windows-gnu,
  util), zmij. `cargo deny check licenses`: `licenses ok` (warnings about unmatched allowances only).

### (b) micro-benchmarks (`crates/oradmp-core/benches/decode.rs`; `cargo bench -p oradmp-core --bench decode`)
The field bytes are collected once with a `RowSink` (`TableRows::next_row_with`) outside the timing; "median" is criterion's
middle estimate. Fields and bytes per iteration are derived from time x throughput.

| group / benchmark | input | fields per iteration | median | fields/s | bytes/s |
|---|---|---|---|---|---|
| `number/OraNumber::decode` | `18c/t_num_exp_18c.dmp` T_NUM, file columns ID and V | 65 | 1.528 µs | 42.5 M | 205 MiB/s |
| `datetime/decode_date` | `18c/t_date_exp_18c.dmp` T_DATE.D | 10 | 151.1 ns | 66.2 M | 442 MiB/s |
| `datetime/decode_timestamp` | `18c/t_ts_exp_18c.dmp` T_TS TS0, TS6, TS9 | 15 | 260.2 ns | 57.6 M | 502 MiB/s |
| `text_cp1251/Charset::decode (Win1251)` | `11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp` R_STAT TXT, CODE, NOTE | 5,598 | 188.5 µs | 29.7 M | 186 MiB/s |

| `read_table/<table>` (whole read, includes re-opening the file) | rows | row bytes | median | rows/s | bytes/s |
|---|---|---|---|---|---|
| R_STAT (`ru_stat_exp_cp1251_est_11g.dmp`) | 2,000 | 98,452 | 1.0306 ms | 1.94 M | 91.1 MiB/s |
| T_NUM (`t_num_exp_18c.dmp`) | 33 | 530 | 39.24 µs | 841 K | 12.9 MiB/s |
| T_TEXT (`t_text_exp_18c.dmp`) | 10 | 759 | 41.86 µs | 239 K | 17.3 MiB/s |

The small tables are dominated by the fixed cost of `read_table` (open the file, build the builders), not by decoding.
T_NUM is read with `on_value_error = Null`: with the default `Raise`, row 19 gives `E-NUMBER-OVERFLOW` (truth_num.txt,
10 values of V fail). Run-to-run variation of the first and second run was within 1 to 4 % (criterion "change" lines).

### (c) throughput (`tests/t15_throughput.rs`)
`ORADMP_WORK=... cargo nextest run --release --run-ignored only -E 'binary(t15_throughput)' -j 1 --no-capture` (3 runs, the
median; without `ORADMP_WORK` or the file it prints a message and passes):

| file | scan (median of 3) | full read, default options (median of 3) |
|---|---|---|
| `T_ROWS` conventional, 10.0 MB, 100,000 rows, row area 10,020,436 bytes | 13 ms = **803 MB/s** (runs 14.4, 12.5, 12.1 ms) | 46 ms = **2.15 M rows/s**, 216 MB/s of row area (runs 47.4, 46.4, 43.9 ms) |
| `T_ROWS` direct path, 9.8 MB, 100,000 rows, row area 9,820,436 bytes | 12 ms = **830 MB/s** (13.3, 11.1, 11.9 ms) | 43 ms = **2.32 M rows/s**, 228 MB/s of row area (43.1, 41.8, 43.9 ms) |
| `T_BIG` | absent | absent |

Notes: the scan figure is file size / (`Dump::scan` time after `open`); a read is `read_table` + all `next_batch`, with the
`Dump` already scanned. The Data Pump file `t_rows_dp_18c.dmp` is not an exp file and was not measured (no Data Pump reader yet).
A cold cache would lower the scan figure to the disk's speed.

### (d) memory (`tests/t15_memory.rs`)
Same command with `binary(t15_memory)`. A child process (the test binary through `current_exe`, helper test `t15_child_read`,
which does nothing without the parent's environment) reads T_ROWS with `batch_rows = 4096` and prints its `VmHWM`:

| file | stop after >= 10,000 rows (stops at the end of the batch: 12,288 rows) | to the end, 100,000 rows | ratio |
|---|---|---|---|
| conventional | 7,504 kB | 7,848 kB | 1.0458 |
| direct path | 7,648 kB | 7,820 kB | 1.0225 |

Both satisfy `peak(100,000) <= 1.10 x peak(10,000)`. Sanity of the test: with `batch_rows = 200,000` in the child, the peak at
100,000 rows is 21,300 kB (3 times the above), so the property is measured; that run stopped at the first assertion (the
"stopped between 10,000 and 20,000 rows" one), not at the ratio, so the ratio assertion itself has no mutation of its own.
The peak is mostly the binary and one batch of 4,096 rows; the 10 % is the plan's number, the margin is 4.6 % / 2.3 %.
Deviation: "10,000 rows" is 12,288 because a batch is not cut (the reader's batch is the unit).

### (e) stream memory (`tools/stream_memory.py`)
`maturin build --release -o target/t15-wheel` (a test build of the working copy, not `tools/release_build.sh`), installed with
`uv pip install` into `target/t15-venv` (CPython 3.14, pyarrow 26.0.0) and run from `/var/tmp` with `python -I`.
T_PART of `18c/part_exp_18c.dmp` (3,000 rows), a fresh `oradmp.open` per read:

| path | VmRSS after read 50 | 250 | 500 | 500 minus 50 |
|---|---|---|---|---|
| `BatchReader` iteration | 60,704 kB | 60,708 kB | 60,708 kB | 4 kB |
| `pyarrow.RecordBatchReader.from_stream(r).read_all()` | 61,072 kB | 61,072 kB | 61,072 kB | 0 kB |

`PASS` (limit 10 MiB). No leak on either path. The tool was not run against a deliberately leaking build.

### (f) Green
`cargo fmt --check` ok; `cargo clippy --all-targets -- -D warnings` and `--all-features` ok; `cargo nextest run -j 2`: **408 passed,
3 skipped** (the skipped are the three `#[ignore]` tests of T-15: `throughput_of_scan_and_read`,
`peak_rss_of_ten_times_the_data_stays_within_ten_percent`, `t15_child_read`); `cargo deny check licenses`: licenses ok.
`ruff check`, `ruff format --check`, `mypy` (the project's 20 files) ok; `tools/stream_memory.py` also passes ruff and mypy on its own
(the project's mypy config does not cover `tools/`). pytest was not rerun (no Python package file changed).

### Problems and things the plan did not foresee
1. `criterion` 0.8.2 with `default-features = false`: the default features pull plotters/rayon; the chosen set is the smallest
   that builds `harness = false` benches. The Architect may prefer the default features (HTML reports); say so and it is a one-line change.
2. The plan's memory command `--run-ignored only -E 'binary(t15_memory)'` also runs the helper `t15_child_read` once (as a
   test without the environment, passing at once). Harmless; it shows as the second test.
3. Cold-cache scan throughput and the disk are not measured; T_BIG is absent (items "if present").
4. The memory test's `VmHWM` is Linux-only (`/proc`); the test would panic on another platform, but it is `#[ignore]`.
5. The ratio is small partly because the batch (4,096 rows) and the binary are tiny against the 10 MB file; a bigger file
   (T_BIG) is the real test of "ten times the data".

### For the next tasks
- The CI regression gate of arch 4.3 (T-16) can use the `read_table` medians above and `T_ROWS` rows/s; the benches need no external file.
- `target/t15-wheel` and `target/t15-venv` are scratch (below `target/`, not committed).

### Answers (Architect, session 16)

Basis: review 04 (no blocking finding). These answer the T-14 note and the T-15 note.

- **T-14 problem 1** (`/tmp` full, leftover `oradmp-fuzz-<pid>` folders) and **problem 2** (the command from the
  root; `fuzz-0.log`): `tools/fuzz_run.sh` in T-13i (ADR-64 item 3): runs from `fuzz/`, creates the corpus folder,
  sets `TMPDIR` on `/var/tmp` with a free-space check, removes the folders and moves the log; CONTRIBUTING gets a
  "Fuzzing" section. The plan's T-14 (d) carries the corrected manual command.
- **T-14 problems 3, 4** (Data Pump seeds; warning offsets checked): accepted (ADR-65 item 10). **Problem 5**: clippy
  of `fuzz/` goes to T-16's nightly job; `-malloc_limit_mb` is accepted as the allocation bound (ADR-65 item 10).
- **T-14, the fuzz profile with debug assertions and overflow checks**: accepted.
- **T-14, the root `.gitignore`** (not in the note; review 04 N-6): that file is outside your row of `CLAUDE.md`. Do
  not edit it again; put ignore rules into a folder you own (`fuzz/.gitignore`). The owner decides the three lines
  (O-16).
- **T-15 problems 1 to 5**: criterion without default features: accepted; the helper test showing in the memory run:
  harmless; cold cache and `T_BIG`: not required, T-15 is done with `T_ROWS` (ADR-62 item 8 amended); `VmHWM`
  Linux-only: T-16 gates it by `target_os`; the small ratio: the limit stays 1.10, the resolution limit is recorded
  and `T_BIG` would sharpen it (ADR-65 item 6).
- **The baseline** is in `docs/design/benchmarks.md` with the Reviewer's figures beside yours and the regression rule
  (ADR-66).
- **Next**: T-13i (plan; Opus, alone): S-1 to S-5 and the notes of ADR-65; its last commit is the version bump to
  0.1.0a2 after the owner's Q-10 answer. Then Reviewer session 5 (closure), then release run B.

## T-13i note (Implementer, session 17): review 04 fixes, version 0.1.0a2

Task T-13i (ADR-64, ADR-65, ADR-67 item 2). Nothing committed. Final counts: `cargo nextest run -j 2` **411 passed, 3 skipped** (408 plus
`reader::tests::in_piece_appends_the_exact_suffix`, `exp::framing::tests::read_unlisted_field_at_the_end_of_the_input_is_truncated`,
`t14_damage::each_truncation_site_names_its_offset`; the 3 skipped are the T-15 `#[ignore]` tests); `pytest -rs` (dev build with
`--features test-probes`) **117 passed, 0 skipped** (105 plus 3 in `test_stream.py` plus 9 in `test_check_dist.py`); `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings` (also `--all-features`), `ruff check`, `ruff format --check`, `mypy` (20 files),
`cargo deny check licenses` (`licenses ok`) green. `Cargo.lock` differs from HEAD only by the two version lines of (l).

### What changed, by item
- (a) S-1: `tests/test_stream.py::test_the_iterator_after_close_is_an_error_before_the_end_and_a_normal_end_after_it`.
- (b) S-2: `patch_v_row_17` (asserts `05 00 c2 02 18 2e 3d` at 0x22F2, sets byte 0x22F4 to `bc`) and two tests, iterator path and stream path.
- (c) S-3: `tools/fuzz_run.sh`, `fuzz/.gitignore` (`fuzz-*.log`), `CONTRIBUTING.md` section "Fuzzing".
- (d) S-4: m1 to m9 in `tests/test_check_dist.py` (`make_wheel` gains `name_version`, `meta_version`, `description`; `make_sdist` gains `root_name`).
- (e) S-5: `common::chain_cut_copy` and `common::lob_chunk_cut_copy` (shared by `t6_patched.rs`, `t12_offsets.rs`, `t12_messages.rs`);
  `without_position_suffix` in `t12_messages.rs` (used by `leaks` and by the 5-digit inventory); exact `ends_with` suffix assertions in
  `t6_patched.rs` and `t12_offsets.rs` (chain and chunk); the `in_piece` unit test in `reader.rs`.
- (f) d9, d10: `date_token_hit` (e3) and `digit_strings_of` (e1) in `t12_messages.rs`; self-test cases in `the_check_reports_a_decoded_value`.
- (g) `t14_damage.rs::each_truncation_site_names_its_offset` (10 cases), `check_warning_offset` in both properties (reader warnings and scan
  warnings), `read_unlisted_field_at_the_end_of_the_input_is_truncated` in `framing.rs`.
- (h) `t15_memory.rs`: renamed `peak_rss_of_eight_times_the_data_stays_within_ten_percent`, the stop asserted `== 12_288`, `T_BIG` branch
  (`18c/t_big_exp_18c.dmp`, stop at 100,000 rows = the batch boundary 102,400, total 5,000,000, limit 1.10; **not run: `T_BIG` is absent**).
- (i) guard: `options.rs::number_type` wraps `NumberType::from_str` (both call sites) in `catch_panic`; `number_summary` in `lib.rs::survey` runs in
  `py.detach(|| catch_panic(|| Ok(number_summary(&all))))`; the module comment of `lib.rs` lists the exempt calls of ADR-65 item 1. No test (no
  panic path exists).
- (j) `_probe_panic` removed from `python/oradmp/_core.pyi` (module function and `_Dump` method); `tests/test_internal.py` reaches them with
  `getattr` (a variable name, because ruff B009 rejects `getattr(x, "literal")`); `tools/release_build.sh` pins `ziglang==0.17.0` and resolves
  the root and `$HOME` with `realpath -m`.
- (l) `0.1.0-alpha.2` in `Cargo.toml`, `Cargo.lock` (2 entries), `fuzz/Cargo.lock` (1 entry; `cargo metadata --locked` in `fuzz/` agrees);
  `README.md` status line and install text (Q-10 = upload: `pip install "oradmp==0.1.0a2"`, then the a2 wheel and sdist file names as the offline
  alternatives). These four files are the separable "version" commit.

### (a), (b) stream mutations (binding rebuilt with `maturin develop --features test-probes`, full `pytest`)
| id | mutation | fails |
|---|---|---|
| s1f | `return if st.finished \|\| !from_stream {` (iterator pull after `close()` is a silent end) | `test_the_iterator_after_close_is_an_error_before_the_end_and_a_normal_end_after_it` (1 failed, 107 passed) |
| s1e | `st.warnings = src.warnings()` only when the pull is `Ok` | `test_the_warnings_are_complete_after_an_error_on_the_iterator_path` **and** `..._on_the_stream_path` (2 failed, 106 passed) |

ADR-64 item 2 says the warning is "issued once at the end of the `with`" on both paths. On the **iterator** path the counts are issued when the error
ends the iteration (the existing `test_an_error_ends_the_iteration_and_issues_the_counts_up_to_it`), not at the end of the `with`; the new test asserts
that, and that the `with` end does not issue a second time. On the stream path nothing is issued until the end of the `with`. No code change; the ADR
sentence is true for the stream path only.

### (c) `tools/fuzz_run.sh walk 30` in a fresh clone
`git clone --no-local` of HEAD to `/var/tmp/oradmp-t13i-clone` (no `fuzz/corpus/`, no `fuzz/target`), plus the new script and `fuzz/.gitignore`;
`CARGO_BUILD_JOBS=2 tools/fuzz_run.sh walk 30`: the script printed the seed count (235), built the fuzz crate from scratch with the pinned nightly
(1 m 37 s wall in total), ran libFuzzer and exited **0**. Last lines: `#43530 DONE   cov: 1551 ft: 4763 corp: 309/17Mb lim: 729088 exec/s: 1404
rss: 205Mb` and `Done 43530 runs in 31 second(s)`. Afterwards: `fuzz/artifacts/walk/` empty, no `oradmp-fuzz-*` folder in `/var/tmp/oradmp-fuzz`,
`fuzz-0.log` moved there, `fuzz/` holds no log. (`cargo fuzz --version`: 0.13.2.) The exit status for a finding was not exercised (no finding to
produce); it is `exit 1` when `fuzz/artifacts/<t>/` is not empty, by reading only.

### (d) `check_dist.py` mutations (each applied alone to `tests/check_dist.py`, `pytest tests/test_check_dist.py`)
| id | mutation | fails (exactly one test each) |
|---|---|---|
| m1 | `elif expected != version:` -> `elif False:` | `test_m1_a_wheel_named_with_another_version_fails` |
| m2 | `if msg["Version"] != version:` -> `if False:` | `test_m2_a_metadata_version_other_than_the_wheel_name_fails` |
| m3a | the wheel member old-name scan removed | `test_m3_the_old_project_name_fails_unless_it_is_the_allowed_line` |
| m3b | `allow = True` (the allowed line accepted in every member) | the same test (third assertion) |
| m4 | `.dmp` rule off | `test_m4_a_dump_file_in_the_sdist_fails` |
| m5 | `truth_*.txt` rule off | `test_m5_a_truth_file_in_the_sdist_fails` |
| m6 | `FORBIDDEN_NAMES` rule off | `test_m6_the_survey_tool_in_the_sdist_fails` |
| m7 | "outside oradmp/" rule off | `test_m7_a_wheel_member_outside_oradmp_fails` |
| m8 | sdist root-folder rule off | `test_m8_an_sdist_with_the_wrong_root_folder_fails` |
| m9 | `__pycache__` removed from `FORBIDDEN_COMPONENTS` | `test_m9_a_pycache_folder_fails_by_its_component_without_the_suffix_rule` |

### (e), (f) message-check mutations (the `Err(e)` arm of `field()` in `reader.rs`, each alone; tests: `t12_messages`)
Each of d1 to d10 fails `no_message_of_the_fixtures_holds_a_value` (and the self-tests that read real messages). d9 appends `15.06.-100` to
`E-BAD-DATETIME`; d10 appends `1,000,000,000,000,000,000,000,000,000,000,000` (10^33 in groups of 3) to `E-NUMBER-OVERFLOW`; d1 to d8 as in the T-13h
table (my script appends the form named there; d4c uses `{:E}`).

| id | fails |
|---|---|
| d1, d2 | `no_message_of_the_fixtures_holds_a_value`, `the_check_reports_a_decoded_value`, `the_check_reports_a_value_in_each_form` |
| d3, d5, d6, d7 | `no_message_...`, `the_check_reports_a_value_in_each_form` |
| d4a, d4b, d4c, d8, d9, d10 | `no_message_...`, `the_check_reports_a_decoded_value` |
| premise (`n + 100000` in `in_piece`) | `no_message_...` (no message ends with "(piece 2 of the value)"), `a_broken_utf8_sequence_in_a_chain_is_a_value_error`, `one_chain_value_reports_its_start_under_every_policy`, `one_lob_value_reports_its_start_under_every_policy`, `reader::tests::in_piece_appends_the_exact_suffix` |
| a value (`short`, a T_LONG value) appended inside the message before the suffix | `no_message_...`, `reader::tests::in_piece_appends_the_exact_suffix` (the first because `short` is a 4-character run of a field of the table) |

Choices the ADR left open: (e3) tries all three orders (Y-M-D, D.M.Y, M/D/Y) for every separator `.` `/` `-` (a superset of "the orders of the
separators"), a `-` before the first or last group is a sign, month and day are never negative; (e1) joins groups separated by one of `, . _ ' ` or
a space, every group after the first of exactly 3 digits, and also tests the **canonical form** of a joined string against the stored numbers (10^33 in
groups has the digit string `1` + 33 zeros, which the 6-digit window test alone would find only by the accident that some stored digit string holds
`000000`). Self-test negatives: groups of 2 digits are not joined; `15.06.-101`, `16.06.-100`, `15.06/-100`, `13.13.-100` are not hits.

### (g) truncation sites (`t14_damage.rs::each_truncation_site_names_its_offset`)
The scan is where every case fails (it reads every row), `Err(E-TRUNCATED)` from `Dump::scan`. Numbers from `exp_parse.py FILE --rows` (Analyst's
tool, run read-only on the fixtures; the lines are quoted in the test's doc comment):

| site | fixture, cut | exp_parse line | code, message, offset (rule: where the failed primitive began) |
|---|---|---|---|
| `rows.rs` "row or the end of rows" | `18c/t_num_exp_18c.dmp`, cut 0x22ef | `row 17 0x22ee` | `E-TRUNCATED`, 0x22ee, table 1, row 17 (the row that should begin) |
| the same | cut 0x22ee | `row 17 0x22ee` | 0x22ee, row 17 |
| the same (end marker) | cut 0x2456 | `rows: 33; end marker ff ff at 0x2455` | 0x2455, row 34 |
| `io.rs` "inside a field" (length u16) | cut 0x22f3 | `@22f2 [5] c2 02 18 2e 3d` | 0x22f2 |
| the same (value bytes) | cut 0x22f6 | the same field, data at 0x22f4 | 0x22f4 |
| the same (value bytes) | cut 0x22f0 | `@22ee [2] c1 12`, data at 0x22f0 | 0x22f0 |
| `io.rs` "inside a line" | cut 0x2551 | `0x254d line [4+LF] 'EXIT'` (LF at 0x2551) | 0x2551 (the end of the file) |
| `items.rs` "before the second EXIT line" | cut 0x2552 | `0x2552 line [4+LF] 'EXIT'` | 0x2552 |
| `io.rs` "inside a field" in a LOB value | `18c/lob_exp_18c.dmp`, cut 0x4359 | `LOB value of row 5 col 2 @0x4358 long [200000]` | 0x4358 (the `fc ff`) |
| the same, the 8-byte total | cut 0x435d | the total at 0x435a | 0x435a |
| `framing.rs:109` | unit test over a `Cursor`, `03 00 61 62 63` | no fixture | 5 |

**Problem for the Architect (no action needed from me):** the site `exp/rows.rs` "file ends inside a LOB length" (`<[u8; 8]>::try_from(b)` after
`s.read_bytes(8)?`) **cannot be reached**: `read_bytes(8)` returns exactly 8 bytes or fails first with "file ends inside a field". ADR-65 item 3
lists it as a site; the last two LOB cases show the real behaviour (the message is "inside a field", at the offset of the 8 bytes). The
branch is defensive code; I did not remove it (T-13i changes no core code). Proposal: leave it, or replace the `map_err` closure by a
`Truncated` error with the same text as `read_bytes`' so that the two cannot diverge.

Mutations at the default 256 cases (`cargo nextest run -j 2 -p oradmp-core -E 'binary(t14_damage) | kind(lib)'`):
| id | mutation | fails |
|---|---|---|
| a1 | `rows.rs` "row or end of rows" `.at(at + 3)` | `each_truncation_site_names_its_offset` and `truncation_gives_ok_or_a_typed_error` |
| a2 | `framing.rs:109` `.at(at + 2)` | `framing::tests::read_unlisted_field_at_the_end_of_the_input_is_truncated` |
| c2 | panic (`v[at as usize]` on an empty `Vec`) before the error at `framing.rs:109` | the same unit test |
| (own) | `io.rs` "inside a line" at `offset + 1` | `each_truncation_site_names_its_offset`, `truncation_...` |
| (own) | `items.rs` "second EXIT" at `item_off + 1` | the same two |
| (own) | `io.rs` `read_bytes` "inside a field" at `offset + 1` | the same two |

A proptest regression file (`t14_damage.proptest-regressions`) appeared during the mutation runs; I deleted it (it records mutation failures, not findings).
Warning offsets are now bounded in both properties (reader warnings and the scan's `dump.warnings()`); no run found a warning beyond the bound.

### (h) memory test
`ORADMP_WORK=$PWD/corpus/work cargo nextest run --release --run-ignored only -E 'test(peak_rss)' -j 1 --no-capture`: three runs, ratios
(conventional / direct): 1.0485 / 1.0362, 1.0592 / 1.0554, 1.0164 / 1.0439; peaks about 7.4 MB to 7.9 MB, the stop `== 12_288` and the end
`== 100_000` asserted. The `T_BIG` branch is written and not run (the file is absent). Mutation "retain a `u32` per row" in `row_end`
(`static LEAK: Mutex<Vec<u32>>`), three runs: ratios 1.0512, 1.0832 / 1.0472, 1.0709 / 1.0391, 1.0644 (conventional, direct): **passes** (by design,
ADR-65 item 6). "Retain a `u64` per row", three runs: 1.0895, 1.0734 / 1.0923, 1.0857 / **1.1106** (fails, `peak 8556 kB ... exceeds 1.10 x
7704 kB`): **fails in 1 of 3 runs only**, so the 8-byte mutation is at the edge of what a 10 MB file resolves with a 1.10 limit, not "shown" every
time (the review's wording "catches 8 bytes per row (shown)" holds for one run in three here). The limit stays 1.10 as decided; `T_BIG` would
resolve it.

### (j) `release_build.sh`
A symlink `/var/tmp/oradmp-sym-t13i -> <home>/zz-sym-target-t13i` and `ORADMP_BUILD_ROOT=/var/tmp/oradmp-sym-t13i/build`:
`release_build: the build root <home>/zz-sym-target-t13i/build is inside the home directory <home>`, exit 1 (before the change the check compared
the unresolved text and the build would have started). The same for `/var/tmp/../..<home>/...`. Both scratch objects removed.
The pin `ziglang==0.17.0`: the version `release_build.sh` printed in run A and in the review ("zig: 0.17.0"); `uv pip install --dry-run ziglang==0.17.0`
resolves it, and the dry build below used it. (The printed number is the compiler's; that it equals the PyPI package version is an observation, not
a rule.) SBOM wording (ADR-65 item 8): "6 occurrences of 2 paths" is the review's count, not asserted by any test.

### (l) dry build of the version commit
The working tree was committed in a scratch clone (`/var/tmp/oradmp-t13i-scratch`, commit `65f7277`, scratch; it also holds the owner's untracked
`docs/prompts/analyst-11-housekeeping.md`), and `ORADMP_BUILD_ROOT=/var/tmp/oradmp-t13i-release tools/release_build.sh HEAD` ran there: `cargo metadata`
prints `oradmp-core 0.1.0-alpha.2`, `oradmp-py 0.1.0-alpha.2`; output `oradmp-0.1.0a2-cp311-abi3-manylinux_2_28_x86_64.whl` and
`oradmp-0.1.0a2.tar.gz`, **`check_dist: ok`**, rustc 1.99.0, maturin 1.15.0, zig 0.17.0, epoch 1792617076, 2 m 18 s. SHA-256 (they belong to the scratch commit,
not to the release): wheel `d4a8a2513c983ba2b8256d9f2f1f6a3927343e6cdeb2a15371186283f03db691`, sdist
`2de5b8c607d7a9cd8043b5d5949c37999b496aec57c2333f255cba433fd552ce`. The wheel's `oradmp/_core.pyi` holds no `_probe_panic`. Not run: `--twice`,
the install tests, the RHEL 8 install (the closure review and run B do those).

### For the next tasks
- Reviewer session 5 repeats s1e, s1f, m1 to m9, premise, d9, d10, a1, a2, c2 alone (tables above give the exact replacement texts' meaning; my
  scripts are in the session scratch folder only and are not committed).
- The version commit (l) is separable: `Cargo.toml`, `Cargo.lock`, `fuzz/Cargo.lock`, `README.md`.
- Scratch folders of this task (clone, scratch repository, release root, fuzz TMPDIR) were deleted at the end.

### Answers (Architect, session 17; ADR-69)
- **Warnings on the iterator path**: the code and the test are right; ADR-64 item 2 and T-13i (b) were worded for the
  stream path only and are amended. README unchanged (its timing claim is in the stream example only). No code change.
- **"file ends inside a LOB length"**: remove it, in **T-13j** after run B: `io.rs::read_u64_le` written like
  `read_u16_le`, called by `lob_value` (your second proposal, as a primitive). Reason: the dead branch would report
  `E-FRAMING` at the start of the LOB value, while rule L4 / reader step 6 and the real path give `E-TRUNCATED` at
  the 8 bytes.
- **Memory test**: ADR-65 item 6's "8 bytes per row (shown)" is corrected to "at the
  limit, 1 of 3"; the limit stays 1.10 (1.07 would sit inside the noise); `T_BIG` is owner action **O-17**, the runs on
  it are **T-15b**. Not a gate for run B.
- **d9, d10 rules**: accepted as built (the superset of orders and the canonical-form join).
- The T-13i commit also adds a root `uv.lock` and comments out the `fuzz/` lines of the root `.gitignore`; neither is
  in the note. Reviewer 5 checks both (item (8)); nothing to do before that.

## T-13j note (Implementer, session 18): review 05 fixes, `read_u64_le`

Task T-13j (ADR-69 item 2, ADR-71). Nothing committed by me; the owner's "Analyst 12" commit (`1c17633`) happened while I worked and already holds
most of my changes. Final counts: `cargo nextest run -j 2` **412 passed, 3 skipped** (411 + the `read_u64_le` unit test); pytest **143 passed, 0 skipped**
(117 + 22 `check_dist` cases + 2 + 2 README tests; dev build with `--features test-probes`); `cargo fmt --check`, `clippy --workspace --all-targets -D warnings`,
`ruff check`, `ruff format --check`, `mypy` clean. No golden or expected file touched; version unchanged. No benchmark run (a3's step 3b is due: `io.rs` and
`exp/rows.rs` changed).

- (a, b) `io.rs` `read_u64_le`, unit test `read_u64_le_reads_eight_bytes_and_a_short_read_is_truncated_at_its_start`. `lob_value` uses it;
  `grep -rn "inside a LOB length" crates/` is empty.
- (c) `t14_damage.rs` doc comment and the label of the 0x435d case reworded (offset = where the failed read stood; "inside a line" expects the end of the file).
  Mutation **line_start** (`.at(line_start)` on "file ends inside a line" in `io.rs`) fails `each_truncation_site_names_its_offset`.
- (d) SF-1: `check_row_17_rounded_before_row_19(reader)` added to the iterator test.
- (e) SF-3: m10 to m21 in `tests/test_check_dist.py` (helpers got `meta_edit`, `omit`, `file_name`). `check_dist.py` changed in one place: a missing
  `PKG-INFO` made `tarfile.extractfile` raise `KeyError` instead of printing "FAIL: sdist: PKG-INFO is missing"; now caught.
- (f) `crates/oradmp-py/src/lib.rs` module comment now reads: "Also exempt (session 18 amendment, ADR-71 item 4; review 05 N-1; each is panic-free by reading):
  `Warning`'s `Display` in `warning_dict` and `Error`'s `Display` in `error_instance` (both in `errors.rs` ...), `Charset::from_id` and `Charset::name`
  (`conv.rs` ...), the `as_str` constants of `ErrorCode` and `WarningCode` and `ALL` (module initialisation), and the implicit drops of `PyReader` and
  `PyDump` at garbage collection (not `close()`)." All five "not in the list" rows of review 05's guard table are named.
- (g) README: status line names "Python 3.11 or later, pyarrow 15 or later", both install commands in code marks, licence links absolute
  (`https://www.apache.org/licenses/LICENSE-2.0`, `https://opensource.org/licenses/MIT`). `test_project_name.py::test_the_readme_has_no_relative_link`.
  **Project-URL: not added** (O-19 (a) has no recorded answer when T-13j started); no `pyproject.toml` change, no m22.
- (h) `test_version.py::test_every_version_named_in_the_readme_is_the_cargo_version` (3 or more matches, all equal `pep440(cargo_version())`).
- (i) CONTRIBUTING "Fuzzing" now says the script "moves `fuzz-0.log` into the tmp folder". Line of `tools/fuzz_run.sh` (46-47): `if [ -f fuzz-0.log ]; then
  mv -f fuzz-0.log "$tmp/fuzz-0.log"`.
- (j) `t15_memory.rs` doc comment: "That resolution of about one byte per row is arithmetic until T-15b measures it."

### Mutation table (each alone, then restored)
| id | mutation | failing test(s) |
|---|---|---|
| s1o | review 05 SF-1 text in `pull_raw` (offset + 1 on iterator error) | `test_the_warnings_are_complete_after_an_error_on_the_iterator_path` only |
| s1e | warnings copied only on `Ok` | that test and the stream-path S-2 test |
| s1i | warnings not copied on an iterator error | iterator S-2 test only |
| line_start | `.at(line_start)` on "inside a line" | `each_truncation_site_names_its_offset` |
| m10 | sdist file-name rule off | `test_m10_...` only |
| m11 | sdist member old-name scan off / `allow = True` | `test_m11_...` only (both) |
| m12, m13, m14 | `if False and` before the Requires-Dist / License-Expression / Requires-Python rule | its own `test_m12/13/14_...` only |
| m15 | sdist required-files rule off | the 10 parametrised cases |
| m16 | wheel licence-file rule off | the 2 cases |
| m17, m18, m19, m20 | Metadata-Version / License-File / description / `License ::` rule off | its own test only |
| m21 | wheel METADATA missing rule off; PKG-INFO `if member is None` off | `test_m21_...` only (both) |
| README link | `[LICENSE-MIT](LICENSE-MIT)` back in the README | `test_the_readme_has_no_relative_link` only |
| README version | first `oradmp==0.1.0a2` changed to `0.1.0a1` | `test_every_version_named_in_the_readme_...` only |

Note: a first run of the m12 to m14 mutations showed stale `.pyc` files (same size, same second); rerun with `PYTHONDONTWRITEBYTECODE=1`. The "second occurrence in
README fails" case of m11 needs a different line with the name: the exact allowed line is blanked everywhere it occurs in README / PKG-INFO, twice included.

### For the next tasks
- a3's step 3b (benchmarks) is due. Nothing else the ADRs did not foresee.

## T-15b note (Implementer, session 18): the memory test on `T_BIG`

Input: `$ORADMP_WORK/18c/t_big_exp_18c.dmp` (the owner's `corpus/work/`; not committed), 453,304,320 bytes, SHA-256
`9fe3d96090b795de789901973393664767c330dc424a6472fadb607eeaa6bda1` (equals `SHA256SUMS_big`); the `exp` log beside it: "T_BIG 5000000 rows exported",
"Export terminated successfully without warnings". Load average 0.02 before the runs, nothing else running; 8 cores, 11.7 GB RAM. Release build, `CARGO_BUILD_JOBS=2`,
`cargo nextest run --release --run-ignored only -E 'binary(t15_memory)' -j 1 --no-capture`. No test file changed (item d). Code is the tree after T-13j (`460aef5`).
Plan wording: T_BIG is made by `corpus/sql/15_big.sql`, not `01b_optional_tables.sql` (the plan's prompt-data paragraph names the wrong file; no effect).

### (a) correct reader, three runs (stop at 102,400 rows / to the end 5,000,000 rows; VmHWM kB)
| run | T_BIG stop peak | T_BIG end peak | ratio | T_ROWS conv. ratio | T_ROWS direct ratio |
|---|---|---|---|---|---|
| 1 | 7668 | 7680 | 1.0016 | 1.0518 | 1.0351 |
| 2 | 7472 | 7660 | 1.0252 | 1.0274 | 1.0261 |
| 3 | 7580 | 7684 | 1.0137 | 1.0358 | 1.0373 |
Passes 3 of 3. T_BIG peaks are flat (about 7.5 to 7.7 MB) for 49 times the rows.

### (b) mutation "retain b bytes per row" in `TableSink::row_end` (`reader.rs`; `static LEAK: Mutex<Vec<u8>>`, `extend_from_slice(&[1u8; b])`), reverted
| b | runs | result |
|---|---|---|
| 8 | with the T_ROWS files present | fails 3 of 3, but **on the T_ROWS files** (ratios 1.1014, 1.1033, 1.1014; the test stops there, T_BIG not reached; the 10 MB file is at the edge, as T-13i (h) found) |
| 8 | `ORADMP_WORK` holding only a symlink to the T_BIG file (T_ROWS skipped), to isolate the T_BIG branch | **fails 3 of 3 on T_BIG**: ratios 5.8801 (8408 -> 49440 kB), 5.9797 (8272 -> 49464), 5.9587 (8324 -> 49600) |
| 4 | with T_ROWS present (T_ROWS pass, 1.02 to 1.06) | **fails 3 of 3 on T_BIG**: 3.6890 (7896 -> 29128), 3.7329 (7772 -> 29012), 3.7364 (7768 -> 29024) |
| 1 | the same | **fails 3 of 3 on T_BIG**: 1.9611 (7604 -> 14912), 1.7764 (7640 -> 13572), 1.8991 (7848 -> 14904) |
So ADR-65's "about one byte per row" resolution holds: b = 1 adds 4.9 MB by arithmetic; the measured rise is larger (6 to 7 MB) because `Vec` doubles its capacity. No
correction to ADR-65 needed. For b = 8 the first T_BIG evidence needed the T_ROWS files out of the way (the T_ROWS branch fails first); the test itself is unchanged.

### (c) `t15_throughput`, one run (not a baseline; benchmarks.md rule 1 needs a Reviewer repeat)
T_BIG: file 453,304,320 bytes, 5,000,000 rows, row area 453,288,049 bytes. Scan runs 577.2 / 566.8 / 600.8 ms, median 0.577 s = 785.3 MB/s. Read runs 2.305 / 2.250 / 2.277 s,
median 2.277 s = 2,195,749 rows/s, 199.1 MB/s of row area. (The T_ROWS lines of the same run: scan 809.1 / 886.7 MB/s, read 2.17 M / 2.45 M rows/s.) The file was
in the page cache (11.7 GB RAM).

### Deviations / for the next tasks
- None in code; `reader.rs` restored byte for byte after each mutation (`git status`: only the owner's `01b_optional_tables.sql` change). `cargo fmt --check`, `cargo clippy -- -D warnings`,
  `cargo nextest run` (412 run, 412 passed, 3 skipped) green.
- Suggestion to the Architect (no action taken): the test aborts at the first failing file, so a leak that fails T_ROWS hides whether T_BIG would also fail; harmless for the correct reader.

### Answers (Architect, session 19; ADR-75 items 1, 2)
- **Results accepted.** ADR-65's "about one byte per row" holds; its wording is not changed, only an amendment records the
  measured figures. The plan's prompt-data paragraph is corrected (`15_big.sql`, not 01b).
- **Stop at the first failing file**: changed, as you suggest, in **T-18b (a)**: every file is measured, then the test asserts,
  and the message lists each file's peaks and ratio. Limit and stop points unchanged.
- **Numbers**: not a baseline (no Reviewer repeat; no `cargo bench`). They serve a3's step 3b as "no regression" for throughput
  and memory; T-18b (b) runs the micro-benchmarks; Reviewer 6a repeats all. `benchmarks.md` lists them as candidates.

## T-18 note (Implementer, session 19): the command-line tool `oradmp` (ADR-72)

Built: `python/oradmp/_cli.py` (new), `__main__.py` (calls `_cli.main`), `pyproject.toml` (`[project.scripts]`), `tests/test_cli.py` (42 tests), `tests/check_dist.py`
(`check_entry_points`, `_cli.py` required in the sdist) and `tests/test_check_dist.py` (m22 to m24b), README "Command line". Nothing in `crates/`, `corpus/`, `tests/golden/`.

### Findings for the Architect (none blocks)
1. **Parquet has no seconds unit.** DATE columns are Timestamp(second) (143 of the fixtures' columns). `pyarrow.parquet` stores them as milliseconds and reads them back as
   `timestamp[ms]`, on pyarrow 26.0.0 and 15.0.2 (the stored Arrow schema does not restore `s`). So "read back equals `read_all()` (`Table.equals`, schema metadata included)" of T-18 (c)
   holds only after `timestamp[s]` -> `timestamp[ms]` (same instants, nothing else differs; field metadata kept). The test says so (`parquet_form`); the README says it. The tool writes
   pyarrow's defaults as ADR-72 item 5 asks. Proposal: keep (Parquet logical type is ms anyway); say it in ADR-72 if accepted.
2. **`T_NUM` cannot be exported with the defaults** (row 19 is an integer part that does not fit, `E-NUMBER-OVERFLOW`), and with a bad row 17 the rounding warning is counted at the end of the
   table. So (g) "exactly one warning line" is tested on the export of the patched copy that ends with the error: stderr is exactly the one `W-NUMBER-ROUNDED` line (text equal to
   `BatchReader.warnings[0].text`) and the one error line. The tool prints the reader's warnings also after an error (they are complete then, the iterator path does it).
3. **`head` loses the counted warnings.** `BatchReader.warnings` is empty before the iteration has ended (checked: after `next()` of 18 rows and after `close()` the list of the patched `T_NUM` is
   `[]`). `head -n N` stops after N rows, so a rounded or nulled value in them has no warning. Documented in the README. Proposal: the core could report the counts so far on `close()`.
4. **argparse on Python 3.11**: an optional positional (`nargs="?"`) is matched before options, so `oradmp head FILE -n 3 TABLE` gives "unrecognized arguments: TABLE" on 3.11 (3.13 accepts it).
   The synopsis of ADR-72 (table right after the file) works on both; the README says "put the table right after the file name". `parse_intermixed_args` does not work with sub-commands.
5. `survey FILE` is unchanged, also for a missing file or a file that is not a dump (`ERROR: ...` on **stdout**, exit 1, the survey's own text; exit 3 does not apply to it). Its output on every tested
   fixture is byte-identical to `write_survey` (`test_survey_through_the_new_entry...`); `tests/test_survey.py` is unchanged and green.
6. Choices the ADR leaves open, made as follows (change by record if wrong): flags print `true` / `false`, none prints an empty field; `tables` / `export --all` print the stop error of a scan that
   stopped, the single-table commands do not (the table was read in full); `--all` needs an existing folder (no `mkdir`); `--all` without `--format` is CSV; `-o -` without `--format` is CSV;
   `--columns` applies to every table of `--all`; `-q` is an option of every command but `survey`; `--version` is added; a closed stdout of `export -o -` is `[E-IO]`, exit 3; a Python
   exception that is not an `OradmpError` / `OSError` is not caught (traceback).
7. Not run: (j) "a dry build into a scratch root prints `check_dist: ok`" was run on a copy of the working tree committed in a scratch repo (`/var/tmp`, removed afterwards), because
   `release_build.sh` builds a commit and the diff is not committed. Result: `check_dist: ok`; the wheel lists `oradmp-0.1.0a2.dist-info/entry_points.txt` (`oradmp=oradmp._cli:main`, no spaces:
   m24b); wheel installed in a fresh 3.11 venv (pyarrow only, no pandas, no numpy): `oradmp --help`, `tables`, `export` to CSV and Parquet, and `oradmp head ... | head -1` (codes 0 0) work.

### Refused types (D: CSV writer), probed with one null row per column (`_probe_columns`)
The refusal shows only when a row is written, so the preflight writes one null row per column to a `BytesIO`. Types tried: int64, float64, decimal128, decimal256, utf8, large_utf8, binary, large_binary,
timestamp(s, us, ns, us+UTC), duration(us), date32, null, month_day_nano_interval.
| pyarrow | CSV refuses | Parquet refuses |
|---|---|---|
| 26.0.0 | `month_day_nano_interval` only ("Unsupported cast ... to utf8") | `month_day_nano_interval` only |
| 15.0.2 | the same | the same |
Every type of the fixtures under `interval_as="string"` (Utf8, LargeUtf8, Int64, Float64, Decimal128, Binary and LargeBinary (hex in CSV), Timestamp s / us / ns / us+UTC) passes both writers on both versions
(the whole `tests/test_cli.py` ran green on pyarrow 15.0.2 in a Python 3.11 venv with `PYTHONPATH=python`, and on 26.0.0). With `--interval-as native` the INTERVAL YEAR TO MONTH column is refused with `E-CONFIG` naming
column and type. Duration(us) is written by both. The tool chose no text form for a refused type.

### Mutation table (each alone, restored; `tests/test_cli.py` failing tests)
| mutation in `_cli.py` | failing |
|---|---|
| `OSError` out of `EXIT_CODES` | the walk of `ERROR_CODES`, three exit-3 tests, two closed-pipe tests |
| `UsageError` out of `EXIT_CODES` | the walk and seven exit-2 tests |
| partial file not removed | five tests of (e)/(f) |
| preflight probe off | the refused-type test |
| reader warnings not printed | the (g) test |
| `-q` ignored | (g) and the scan-warning test |
| hex conversion off | the CSV half of the export test |
| closed-pipe handling off | both subprocess tests |
| file-name uniqueness ignoring case off | the file-name unit test |
| `--overwrite` check off | two tests |
| interrupt exit 130 -> 1 | the interrupt tests |
| `warnings` filter off | the (g) `recwarn` test |
| `--interval-as` default `native` | three tests |
| stop error of a stopped scan not reported | the stopped-scan test |
| `check_dist`: entry_points missing rule off / value rule off | m22 / m23 |

### Checks run
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo nextest run` (412 run, 412 passed, 3 skipped; no Rust file changed); `ruff check`, `ruff format --check`, `mypy` (strict), `pytest` (196 passed) in `.venv`.
### For the next tasks
- CI (T-16) runs `tests/test_cli.py` as is; the closed-pipe tests use `<python dir>/oradmp` if the script is installed, else `python -m oradmp` with `PYTHONPATH=python`. The permission test is skipped as root and on Windows.
- Reviewer session 6: the points 1, 3 and 6 above are decisions to confirm.

### Answers (Architect, session 19; ADR-75 items 3 to 9)
1. **Parquet DATE as `timestamp[ms]`**: accepted; ADR-72 amended; the test's `parquet_form` and the README sentence stay.
2. **"Exactly one warning line"**: the test on the patched `T_NUM` is accepted. Add the success case in **T-18b (d)**: 18c
   `T_NUM` with `--on-value-error null` exits 0 and prints exactly the reader's warnings (expected from ADR-55's data; stop and
   report if it does not exit 0).
3. **`head` and counted warnings**: no core change (ADR-55 item 2: counts are complete only at the end or at an error; a partial
   count on `close()` would change `warnings` for every early-closed reader and cover whole decoded batches, not N rows). The
   README sentence is widened in **T-18b (e)**.
4. **Python 3.11 argparse**: accepted; the synopsis order is the contract.
5. **`survey`**: accepted as built (unchanged output and exit codes win).
6. **Open choices**: all accepted (ADR-75 item 6) **except one**: a single-table command whose TABLE is not found in a
   **stopped** scan prints the stop error and exits with its code, not `E-NO-SUCH-TABLE` / 2 (**T-18b (c)**).
7. **(j)**: the scratch-commit dry build is accepted; Reviewer 6a builds the real candidate with `tools/release_build.sh --twice`.
   **(l)**'s CI part moves to T-16.
8. Next: **T-18b**, ending with the a3 version commit; then Reviewer 6a.

## T-18b note (Implementer, session 19): memory-test report, micro-benchmarks, stopped-scan table, warnings test, README, a3 version

Touched: `crates/oradmp-core/tests/t15_memory.rs`, `python/oradmp/_cli.py` (`_read_table`), `tests/test_cli.py` (2 tests), `README.md`, `Cargo.toml`, `Cargo.lock`, `fuzz/Cargo.lock`.
Nothing in `crates/oradmp-core/src/` (the mutation was reverted byte for byte), `corpus/`, `tests/golden/`, `docs/formats/`. `ORADMP_WORK` was the owner's `corpus/work` (set in the shell only, written nowhere). The prompt gave no explicit path; I used the one that holds the three files.

### (a) memory test
The test now measures every file present, then asserts; the doc comment gives the measured figures (1.78 to 1.96 with one byte per row on `T_BIG`; correct reader 1.00 to 1.05).
Correct reader, release build, alone (load 0.14 before): passes. T_ROWS conv 7376 -> 7752 kB (1.0510), direct 7516 -> 7744 (1.0303), T_BIG 7480 -> 7664 (1.0246).
Mutation b = 8 in `TableSink::row_end` (`static LEAK: Mutex<Vec<u8>>`, `extend_from_slice(&[1u8; 8])`), all three files present: **fails, and the message names all three**:
```
memory test failed:
18c/t_rows_exp_18c.dmp: stop 12288 rows (expected 12288) peak 7756 kB; end 100000 rows (expected 100000) peak 8628 kB; ratio 1.1124; FAILS (limit 1.10)
18c/t_rows_exp_direct_18c.dmp: stop 12288 rows (expected 12288) peak 7748 kB; end 100000 rows (expected 100000) peak 8596 kB; ratio 1.1094; FAILS (limit 1.10)
T_BIG: stop 102400 rows (expected 102400) peak 8320 kB; end 5000000 rows (expected 5000000) peak 47692 kB; ratio 5.7322; FAILS (limit 1.10)
```
(One run; the T_ROWS ratios are at the edge as before, T_BIG at 5.73 is not.)

### (b) `CARGO_BUILD_JOBS=2 cargo bench -p oradmp-core --bench decode`, once, alone (mean estimate; baseline = benchmarks.md 4.1 "note"; "rule 3" = within 20 %)
| benchmark | this run | baseline | change | within 20 % |
|---|---|---|---|---|
| `number/OraNumber::decode` | 1.4626 us | 1.528 us | 4.3 % faster | yes |
| `datetime/decode_date` | 145.18 ns | 151.1 ns | 3.9 % faster | yes |
| `datetime/decode_timestamp` | 262.61 ns | 260.2 ns | 0.9 % slower | yes |
| `text_cp1251/Charset::decode (Win1251)` | 173.67 us | 188.5 us | 7.9 % faster | yes |
| `read_table/R_STAT` | 1.0034 ms | 1.0306 ms | 2.6 % faster | yes |
| `read_table/T_NUM` | 36.669 us | 39.24 us | 6.6 % faster | yes |
| `read_table/T_TEXT` | 39.685 us | 41.86 us | 5.2 % faster | yes |
No figure more than 20 % below; no second run needed. Throughput not run again.

### (c) stopped scan
`_read_table` in `_cli.py` wraps `dump.read_table`: on `E-NO-SUCH-TABLE` with `Catalog.end == "stopped"` and a `stop_error`, it raises that error, so `main` prints the same line and exit code as `tables` (1 for `E-UNSUPPORTED-TYPE`).
Used by `columns` (`_find_table`), `head`, `export TABLE`. Test `test_exit_1_a_table_not_found_in_a_scan_that_stopped_gives_the_stop_error` (patched `T_NUM`, first descriptor type 114 as the existing stopped-scan test): all three commands give code 1 and the line `[E-UNSUPPORTED-TYPE offset=0x223a]`, and a complete scan keeps `E-NO-SUCH-TABLE`, 2.
Mutation (branch disabled): that test fails (1 failed, 43 passed). Restored.

### (d) warnings of a successful export
`test_warnings_of_a_successful_export_go_to_stderr`: `export T_NUM --on-value-error null -o t.csv` exits 0; stderr equals `oradmp: warning: ` + each text of `BatchReader.warnings` (API, `interval_as="string"`, `on_value_error="null"`), in order, non-empty list asserted; `-q` gives empty stderr.
Mutation (`_warn_all(reader.warnings)` -> `pass`): fails this test and the older (g) test. Restored.

### (e) README
Head sentence as given (plus "`oradmp --version` prints the version."), stopped-scan rule in the exit-code paragraph, Install line. Note: the stopped-scan rule sits inside the exit-code bullet, which is now long.

### (g), (h) version and checks
`0.1.0-alpha.3` in `Cargo.toml`, both `Cargo.lock` entries, the `oradmp-core` entry of `fuzz/Cargo.lock`; README `0.1.0a3` (4 places); dev build `maturin develop --features test-probes` -> `oradmp.__version__` 0.1.0a3, `tests/test_version.py` green. **The working tree holds all of T-18b, the version files included, uncommitted**; the owner commits (a) to (f) first and the version files (g) separately (`Cargo.toml`, `Cargo.lock`, `fuzz/Cargo.lock`, and the version lines of `README.md`, which is touched by both: use `git add -p`).
Results on this tree: fixture checksums OK (11g, 11g-cp1251, 18c); `cargo fmt --check` ok; `cargo clippy --all-targets -- -D warnings` ok; `--all-features` ok; `cargo clippy -- -D warnings` ok; `cargo nextest run -j 2`: 412 run, 412 passed, 3 skipped (no Rust test added: (a) changed an ignored test); `cargo deny check licenses` ok; `ruff check` ok; `ruff format --check` ok; `mypy` ok; `pytest -rs`: 198 passed, 0 skipped (196 before + 2 new).
Not run: `release_build.sh` / `check_dist` (not asked by T-18b).

### Answers (Architect, session 20; ADR-77, ADR-78)

- (a), (b): repeated by Reviewer 6a within 9 %; with T-15b's figures they are **baseline 2** in `benchmarks.md` (code of
  `460aef5`). The memory-test change is accepted as built.
- (c), (d), (e): accepted (review 06a section 2). The long exit-code bullet may be split in T-18c (d).
- (g): the version commit `d4d20ef` is reviewed but **not released**: review 06a's SF-1, SF-2 and notes N-1 to N-3 go
  to **T-18c**, whose last commit is the new a3 candidate. The version stays `0.1.0a3`; T-18c makes no version commit.
- Review 06a N-4: the README's a3 lines landed in `67f8bd2` before the version commit; harmless (only the version
  commit is a candidate). For later version commits, `git add -p` as the note said.

## T-18c note (Implementer): Reviewer 6a fixes (ADR-77)

- Files: `python/oradmp/_cli.py` (the I/O error text: `_path_text`, `_api_io_text`, `_error_text(e, file)`, `_state.file`),
  `tests/test_cli.py`, `tests/test_version.py`, `README.md`. Nothing else under `crates/`, `corpus/`, `tests/golden/`.
- (a) `grep -n 'ROOT / "python"' tests/test_cli.py`: empty. One helper `child_env()` sets `PYTHONPATH=str(PKG.parent)`.
  Three runs with the wheel built from the working tree (`maturin build --release`): (1) dev tree: green; (2) fresh
  venv + wheel, scratch clone of `d4d20ef` with the four T-18c files copied in, `pytest -rs tests/test_cli.py`: 71
  passed, 0 failed, 0 skipped (`oradmp.__file__` in site-packages); (3) copied folder (`tests/`, `corpus/fixtures/`, root
  files; no `python/`, no `.git`): `test_cli.py` + `test_version.py` 76 passed. Mutations: closed-stdout head test back
  to `ROOT/python` fails in (2) (`test_a_closed_stdout_ends_head_quietly_with_exit_0`); the new
  `test_the_subprocess_imports_the_same_package` back to it fails in (2); pandas scan back to `python/oradmp` fails in
  (3) (`test_no_file_of_the_package_imports_pandas`).
- (b) Mutations, each alone, `git status` clean after the script restored the file: suffix `.parquet` removed (6 fail,
  incl. `test_the_format_comes_from_the_option_else_the_suffix_in_any_case`); `.lower()` removed (1); `.csv` removed
  (15); explicit `--format` ignored (3); `-` not CSV (4); `--all` not CSV (6); `--all -o -` allowed
  (`test_export_all_to_stdout_is_refused_and_creates_nothing`); `--version` text (`test_version_prints_the_package_version`);
  `trim_char` not passed (`test_trim_char_reaches_the_reader` + 12 pass-through); `number_types` not passed
  (`test_number_type_reaches_the_reader` + 2); `_escape` tab / newline / backslash each (`test_the_escape_of_names`);
  `number_default`, `timestamp_unit`, `max_lob_bytes`, `unconfirmed_lob`, `db_charset`, `client_charset`,
  `on_value_error`, `on_number_overflow`, `strict`, `interval_as`, `--columns` (head, export) not passed: each fails its
  case of `test_every_read_option_reaches_the_api` / `test_columns_reaches_the_api`.
  **Finding**: removing the `-n < 0` guard does not give a failed assertion: with `-n -1` pyarrow aborts the process
  (`std::length_error`, exit 134), so pytest itself dies. The test fails (the run is not green), but the guard is
  load-bearing; no change proposed.
- (c) stderr lines: `oradmp: error: [E-IO] No such file or directory: <path>` (exit 3, stdout empty) for `tables`,
  `columns`, `head`, `export TABLE`, `export --all`; `... Permission denied: <path>` for a mode-000 copy (ran, not
  root). Unit test of `_error_text` covers an offset prefix, an `OSError(None, ...)` (keeps `I/O error`), an escaped path,
  and the output-side form. `survey missing.dmp`: `ERROR: ...` on stdout, exit 1 (test added). Mutations: path left
  out, OS text replaced by `I/O error`, no-errno text lost, path not escaped, `_state.file` not set: each fails.
- (d) README: the suffix sentence, `--interval-as` in the option list, the exit-code bullet split in two with the
  line form. (e) `tests/test_version.py`: `ANY_PRERELEASE`; "Alpha (version 0.1.0a2)" appended to the README fails
  `test_every_pre_release_version_in_the_readme_is_the_cargo_version`, README restored.
- (h) Dev build with probes: fixture checksums OK (per folder), `cargo fmt --check`, `cargo clippy --all-targets` and
  `--all-features` `-D warnings` clean, `cargo nextest run -j 2`: 412 passed, 3 skipped; `cargo deny check licenses` ok;
  ruff check, ruff format --check, mypy clean; `pytest -rs`: 227 passed, 0 skipped (198 + 29 new).
- Not done by this session: the commit; the "scratch clone" used `d4d20ef` plus copied files, not a T-18c commit.

### Answers (Architect, session 21; ADR-81, ADR-82)

- T-18c is done (`ff3d6b2`) and reviewed (review 06a-closure, no blocking finding); it shipped as 0.1.0a3.
- The `-n -1` finding: agreed, the guard stays. Review 06a-closure N-2 asks for a named failure: T-18d (c) adds a
  subprocess case, so the mutation gives exit 134 against 2 in a named test instead of aborting pytest.
- The no-errno case of (c): the unit test used `OSError(None, ...)`, which the API never builds (`make_error` builds
  `OSError(message)`); the real form gives `[E-IO] [E-IO] I/O error` without the file. Fixed in T-18d (a) (key on
  `code == "E-IO"`, test errors from `make_error`).
- The "scratch clone of `d4d20ef` plus copied files": accepted; the closure review ran the installed wheel from a clone of
  the T-18c commit itself.
- Next task: T-18d (plan), then T-17, then T-16 (ADR-83).

## T-18d note (Implementer, 2026-10-24): 6a-closure fixes and file names that are not UTF-8 (ADR-82)
- Built: `crates/oradmp-py/src/lib.rs` (`open` takes `PathBuf`; pyo3 0.30 passes the original bytes: proved by the tests),
  `python/oradmp/_survey.py` (new `path_text`, used by the `file:` line), `python/oradmp/_cli.py` (`_path_text =
  _survey.path_text`, because `_cli` imports `_survey`, not the reverse; `_error_text` keyed on `code == "E-IO"`),
  `tests/test_cli.py`, new `tests/test_paths.py` (skipped off Linux, "POSIX byte file names"; T-16 (h) lists it), `README.md`.
- `oradmp.open("a\x00b.dmp")` raises `OSError('[E-IO] I/O error')`, errno `None`, strerror `None`, `args` one string,
  `code == "E-IO"`; the tool line is `[E-IO] I/O error: a.dmp` (the case in `test_the_io_error_text_of_the_api_error`).
- Stderr lines: missing name N: `oradmp: error: [E-IO] No such file or directory: <tmp_path>/\xd2\xe5\xf1\xf2.dmp`
  (exit 3, stdout empty); `survey`: `file: \xd2\xe5\xf1\xf2.dmp  size: ...`.
- Mutations, each alone (git status showed only the task files after each; restored from copies):
  | mutation | failing tests |
  |---|---|
  | `_error_text` keyed on `strerror.startswith("[E-IO")` | `test_the_io_error_text_of_the_api_error` |
  | `survey` without `path_text` | `test_survey_writes_the_file_line_with_escapes` |
  | `path_text` without the `\xHH` step | `test_the_tool_writes_a_missing_name_as_ascii`, `test_survey_writes_the_file_line_with_escapes`, `test_path_text` |
  | binding back to `String` (rebuilt) | `test_the_api_opens_and_reads_a_name_that_is_not_utf8`, `test_the_api_names_a_missing_file_with_enoent`, `test_the_tool_works_on_a_name_that_is_not_utf8`, `test_the_tool_writes_a_missing_name_as_ascii`, `test_survey_writes_the_file_line_with_escapes` |
  | `-n < 0` guard off | the in-process case aborted the whole pytest run (SIGABRT), so it was removed; `test_head_with_a_negative_count_in_a_subprocess_is_exit_2` fails by name (returncode -6, shell 134) |
- Counts: pytest 233 passed, 0 skipped (227 - 1 removed + 7 new); cargo nextest 412 passed, 3 skipped (no binding unit test added).
- Not mine: `docs/formats/requests.md` shows as modified in `git status` (one line added by someone else during the session).

### Answers (Architect, session 22; ADR-85, ADR-86)
- Accepted as done; reviewed in 6b, every mutation of the table failed its named test. Removing the in-process `-n -1`
  case was right (ADR-82 item 5 allowed it; the subprocess case carries the guard).
- `oradmp.open("a\x00b.dmp")` raising `OSError('[E-IO] I/O error')` with errno `None` and `code == "E-IO"`: as ADR-77
  item 4 and ADR-82 item 4 give; nothing to change.
- 6b SF-4: the output path of `export -o` and the `filename` of an output-side `OSError` still print as `\udcHH`. Ruled
  in ADR-86 item 4: escape them with `_path_text` too (ADR-82 item 3 amended); work in T-17b (e). The README sentence
  stays.
- `docs/formats/requests.md` in `git status`: the Analyst's session 15 edit (`2a60b86`); not the Implementer's concern.

## T-17 note (Implementer, 2026-10-24): synthetic files from the rules (ADR-80)
- Built: `crates/oradmp-core/tests/common/synth.rs` (generator, `Spec` -> bytes + `Layout` of offsets; random `Spec` from
  plain `Params`; expected values and Arrow types; calls no function of `oradmp_core`), `crates/oradmp-core/tests/t17_synth.rs`
  (18 tests, 1 of them ignored: `write_fuzz_seeds`), one line `pub mod synth;` in `tests/common/mod.rs`, the seed command in
  `tools/fuzz_seeds.sh` (`SYNTH_SEEDS`, default 200) and `CONTRIBUTING.md` (Fuzzing). No change in `crates/oradmp-core/src/`
  (`git diff --stat -- crates/oradmp-core/src` empty), none in `corpus/`, `docs/formats/`, `tests/golden/`, `tests/data/`, Python.
- **Rules used, status as read in `docs/formats/exp_reader_rules.md` v3.6 at this commit** (all confirmed unless marked):
  H1, H2, H3, H4 (ids; the 10 filler bytes: unknown), H4a, H5, H6 (constant bytes: confirmed as constant, meaning unknown);
  I2, I3, I4, I7, list K (I6: "confirmed for the fixtures"); I5 **suggested as "pieces"**, confirmed as bytes: written with
  exactly one piece (`CREATE VIEW` + `[u16 n][bytes][00 00] LF`), never with several; S1, S2; T1 to T6 (T6: flag, m = 0 or the
  column count), T7, T9 (the forms listed), T3 (file order, LONG last on the direct path); R1, R2 (incl. the chain), R3, R4,
  R5, R7, R8 (as absence: no length 0, no 0xfff0 to 0xfffd, no empty string); E1, E2, E3, E4; C1, C3, C5, C6, C7 (1251 through
  `encoding_rs`, the generator uses only А-я, Ё, ё and ASCII); Y1, Y2, Y5, Y6, Y9; L1, L2, L3, L4 (short form only), L5 (CLOB of
  the AL32UTF8 database = UTF-16 BE; BLOB = bytes), L6. Not generated as valid input because not confirmed: E2a (suggested): no
  table whose only column is LONG / LONG RAW, every LONG table has at least 2 columns; G1/G2 (no unknown introducing line);
  Y9 character set for a non-AL32UTF8 database (the rule is confirmed for AL32UTF8 only): a LONG in a CL8MSWIN1251 database holds
  ASCII; L4 long form, L5 CLOB/NCLOB of the CL8MSWIN1251 database, NCLOB, INTERVAL, ROWID/UROWID, TSTZ/TSLTZ, partitions
  (T10) and X2: left out (ADR-80 rejected list, or not in the task's type list). Rules **suggested or unknown are written only as
  negative cases**: none beyond the cases of arch 1.4 (below).
- Bytes written that no rule states (all constant in the fixtures, skipped by the reader, listed so the Reviewer can check
  them): the 10 filler bytes of H4 (`00 01` + 8 zeros, as in `t_frame_exp_18c`); R + 0x000 to 0x103 as zeros with an LF at
  R + 0x103 (H6: "skip them"); the 2 bytes behind a LOB column entry (`00 00`, L1 states them); the descriptor length of
  TIMESTAMP (rule T5 gives only the code: 7 for TIMESTAMP(0), 11 otherwise, as `t_ts_exp_18c` and `types/map.rs`
  `check_agreement`); the storage text of `CREATE TABLE` (T9: free text); the group lines of the trailer
  (`METRICST<title>`, `METRICET n`, `METRICETG0`, `METRICSU`/`METRICEU` forms of `own_exp_18c`: T2, R6 and E4 name `METRICS`,
  `METRICET`/`METRICEU` and `EXIT`, the titles are the fixtures'); the locator content (random bytes, 102 to 139 per L2).
  The `METRICE? n` number is the row count of the last table (R6 says it is not one; nothing reads it).
- What the 18 tests do (a "Done when" item each):
  | item | test (`t17_synth.rs`) |
  |---|---|
  | (a) generator with rule ids | `common/synth.rs`; the ids stand beside the writing code |
  | (b) types and values | `every_generated_type_with_edge_values` (both database sets, both paths: 38-digit NUMBER, `NUMBER(38,38)`, DATE years 1 and 9999, TIMESTAMP 0/6/9, non-BMP NVARCHAR2/NCHAR, CHAR in both semantics, 1251 text); `a_lob_table_in_both_kinds_of_file`; `a_32000_byte_long_and_long_raw_on_both_paths` |
  | (c) property test | `a_random_spec_is_read_back_as_written` (1 to 4 tables, 0 to 300 rows, NULL 0 to 100 %, R in {2048..16384}, table / owner mode, conventional / direct, both versions, database 873 / 171, 4 client sets; also the `Layout` offsets: `rows_offset`, `rows_end_offset`, `insert_offset`, `create_offset`, `file_order`, `end_marker`, `end_offset`; `strict` on for odd seeds); `the_random_specs_cover_every_construct` (400 seeds hit every type, both empty forms, chains, LOB tables, 171, owner mode, all R, every noise kind, >= 3 times each) |
  | (d) deterministic | `both_empty_table_forms_after_a_direct_path_flag` (E2 x3, E1, mixed in one 18c file), `an_empty_table_with_ff_ff_on_both_paths_in_an_11g_file`, `a_record_boundary_at_every_byte_position_of_an_element_changes_nothing` (R = 2048; both paths; length field of a value 3 positions, the 20 value bytes 21 positions, NULL marker 3, the `00 00` once: 27 / 28 files, each with the boundary exactly on the position, asserted), `a_32000_byte_long_and_long_raw_on_both_paths` (the direct chain has >= 5 pieces, the LONG column declared first stands last in the file), `owner_mode_with_three_tables` (also 4 tables: odd/even statement groups with `ENDTABLE`), `record_lengths_2048_and_16384` (all four) |
  | (e) option sets | `projection_keeps_the_table_order`, `number_types_override_the_arrow_type` (`int64`, `string`, `decimal(20,4)`), `strict_on_a_clean_file`, `batch_rows_one_and_seven` (batch count = ceil(rows / n), none above n); `interval_as` out |
  | (f) negative | `each_refusal_of_architecture_1_4_has_its_code_and_offset` (every row of arch 1.4 that a single-table file can show: NotExpFile x3 (byte 0, line 1, gzip magic), BadHeader x2, BadRecordLength x3, MissingMetricsLine, Framing x7, UnsupportedLayout x5, UnsupportedType, Truncated x2, each with the generator's offset), `a_truncated_synthetic_file_gives_a_typed_error_within_the_file` (random cut: an error whose offset is at most the cut, exactly when the cut lies before the second EXIT line) |
  | (g) fuzz seeds | `tools/fuzz_seeds.sh` (200 files `synth__NNNN.dmp` in `target/fuzz-seeds/`) through the ignored test `write_fuzz_seeds` |
- Offsets the reader gives for the negative cases (all equal the generator's): UnsupportedType at the descriptor (column 2, type
  code 114); BadRecordLength at the RECORDLENGTH digits; INSERT INTO without `TABLE` at the `INSERT INTO` line; column count of the
  `INSERT INTO` line: at the `INSERT INTO` line, length 2; k: at the u16 k (table 2, row 2); second `INSERT INTO`: at the second
  line; zero fill for `EXIT`: at the first zero byte (the offset of the second `EXIT`).
- **Disagreements between the reader and the generator: none.** All 18 tests passed on the first complete run after fixing four
  mistakes of my own (a test value longer than its VARCHAR2(30) gave the reader's `W-LONGER-THAN-DECLARED`; a miscounted case
  total; and one real catch: a Cyrillic LONG in a CL8MSWIN1251 database gave `W-LONG-CHARSET` under `strict`, i.e. the generator had
  written a construct of Y9 whose character-set statement is confirmed for AL32UTF8 only; the generator now writes ASCII there).
  Because the generator and the reader share one reading of the rules (common mode), a pass proves agreement, not truth.
- Numbers: no number comes from an Analyst tool in this task: the expected values are the `Spec`'s; the constants come from the
  plan and the rules (R, the 32,000 bytes of Y9, 16 type codes). Rust tests before / after: 412 passed + 3 skipped (plan) ->
  **429 passed + 4 skipped** (`cargo nextest run`: 17 new running tests; the 18th, `write_fuzz_seeds`, is ignored and counted
  skipped).
- (c) times: `cargo nextest run -p oradmp-core --test t17_synth -j 2`: 2.7 s (nextest summary) at 64 cases (limit 60 s).
  `PROPTEST_CASES=2000 cargo nextest run -p oradmp-core --test t17_synth -j 2`, alone on the VM: 17 passed, 17.8 s; both
  property tests ran 2000 cases (random spec, truncation), 0 failures.
- (g) `tools/fuzz_run.sh read 60` after `tools/fuzz_seeds.sh` (435 seed files: the 235 `.dmp` files of the three fixture
  folders + 200 synthetic): "Done 70215 runs in 61 second(s)", exit 0, `fuzz/artifacts/read/` empty: **findings 0**. Deviation: ADR-80 (g)
  says "the ignored seed folders of the three targets"; the synthetic files go to `target/fuzz-seeds/`, the one seed folder
  `tools/fuzz_run.sh` passes to every target (ignored through `target/`); `fuzz/corpus/<target>/` is libFuzzer's own output
  folder (ignored by `fuzz/.gitignore`) and is not written by the script.
- (j) Failure persistence: both new property tests set `failure_persistence: None` and put the seed and all `Params` in the
  failure message; after a forced failure (mutation below) `find . -name '*.proptest-regressions'` is empty.
  **`t14_damage.proptest-regressions` (review 06a N-5): yes, `t14_damage`'s own property tests can produce it under a mutation.**
  With `ByteStream::truncated` (`io.rs`) changed to `.at(self.offset().saturating_add(100000))`, `cargo nextest run -p oradmp-core
  --test t14_damage` fails `truncation_gives_ok_or_a_typed_error`, `bit_flips_give_ok_or_a_typed_error` and
  `each_truncation_site_names_its_offset`, and `crates/oradmp-core/tests/t14_damage.proptest-regressions` appears (untracked; I
  deleted it, then restored `io.rs`). So with any mutation that moves an error offset beyond the cut for some inputs, the two
  property tests write the file, while the deterministic test is the only one nextest names when the random sample of that run
  does not reach the bad input (the reviewer's N-5). The reviewer's seed replay passing is expected: the stored seed encodes
  the failing case of the mutated build only. Proposal (not done, `t14_damage.rs` is out of this task's touch list): set
  `failure_persistence: None` there too, or add the file name to `.gitignore` (the owner's).
- (k) Mutations, each alone in `src/` (copy restored after each, `git diff --stat -- crates/oradmp-core/src` empty), run with
  `cargo nextest run -p oradmp-core --test t17_synth --no-fail-fast`:
  | mutation | failing T-17 tests |
  |---|---|
  | `fe ff` read as an empty value (`rows.rs`, NULL arm calls `k.value(.., &[])`) | at least 8: `an_empty_table_with_ff_ff_on_both_paths_in_an_11g_file`, `batch_rows_one_and_seven`, `a_record_boundary_at_every_byte_position_of_an_element_changes_nothing`, `both_empty_table_forms_after_a_direct_path_flag`, `a_lob_table_in_both_kinds_of_file`, `a_32000_byte_long_and_long_raw_on_both_paths`, `a_random_spec_is_read_back_as_written`, `a_truncated_synthetic_file_gives_a_typed_error_within_the_file` (this run stopped at first failures) |
  | the [E2] test removed (`rows.rs`: `if false && self.rows == 0 && ...`) | `both_empty_table_forms_after_a_direct_path_flag`, `owner_mode_with_three_tables`, `a_random_spec_is_read_back_as_written`, `a_truncated_synthetic_file_gives_a_typed_error_within_the_file` |
  | two bytes skipped at each multiple of R inside table data (`io.rs` `read_bytes` drops 2 bytes at every multiple of R past R + 0x104, R from the header) | 8: `a_record_boundary_at_every_byte_position_of_an_element_changes_nothing`, `a_32000_byte_long_and_long_raw_on_both_paths`, `strict_on_a_clean_file`, `a_lob_table_in_both_kinds_of_file`, `batch_rows_one_and_seven`, `record_lengths_2048_and_16384`, `a_random_spec_is_read_back_as_written`, `a_truncated_synthetic_file_gives_a_typed_error_within_the_file` |
  | the sign of a negative NUMBER flipped (`number.rs`: `negative: false`) | 6: `strict_on_a_clean_file`, `number_types_override_the_arrow_type`, `projection_keeps_the_table_order`, `batch_rows_one_and_seven`, `every_generated_type_with_edge_values`, `a_random_spec_is_read_back_as_written` |
- Definition of done, real results: `cargo fmt --check` ok; `cargo clippy -- -D warnings` ok; `cargo clippy --all-targets -- -D
  warnings` ok; `cargo nextest run` 429 passed, 4 skipped; `git status --porcelain` after `cargo nextest run`, after the
  `PROPTEST_CASES=2000` run and after the fuzz run: only the task's files. Python untouched (no Python check run).
- For the next task (T-16): the new test binary writes only below `target/t17-synth/` (removed on drop) and needs no `corpus/`
  file, no `git`, no network: it runs on every platform unchanged; `write_fuzz_seeds` is `#[ignore]`d and needs
  `ORADMP_SEED_DIR`. `tools/fuzz_seeds.sh` now needs a working `cargo` (it runs the ignored test).
- Limits of the evidence (for Reviewer 6b): common mode; no partitions, NCLOB, long LOBs, INTERVAL, ROWID, TSTZ/TSLTZ, no LONG-only
  table, no client set other than {873, 171, 1, 178} in the header (identifiers are ASCII, so the id changes only the header
  fields), names are ASCII (C2 untested), no third database set. The generator found no reader defect.

### Answers (Architect, session 22; ADR-85, ADR-86)
- Accepted as done and reviewed (6b: no blocking finding; byte-identical with the `t_frame_exp_*` fixtures from R +
  0x103; every behaviour-changing reader mutation caught). Thank you for the "Bytes written that no rule states" list:
  it is what made SF-1 findable.
- **(g) deviation** (`target/fuzz-seeds/`): accepted; the plan's wording is corrected (ADR-80 amendment). **`cargo` in
  `tools/fuzz_seeds.sh`**: accepted; T-17b (j) pins the toolchain by changing to the repository root (6b N-8).
- **(j) proposal** for `t14_damage`: accepted as `failure_persistence: None` in `t14_damage.rs`, and also in
  `t4_values.rs` and `t5_types.rs` (6b SF-5); not a `.gitignore` entry. T-17b (f).
- **"Rules used"**: E3 is suggested, not confirmed; the 18c E1-after-flag-1 case, the client pairs no fixture shows, an
  18c header with database 171 and TIMESTAMP(1-5, 7, 8) were outside ADR-80 item 1. Ruled in ADR-86 item 1: the first
  two stay as labelled `Unconfirmed` constructs, the last two are removed; the source of "7 for (0), 11 otherwise" must
  not be `types/map.rs` (reader code). T-17b (a), (b), (k).
- 6b SF-2 (the `i == 0` guard), SF-3 (`nt.arrow()`), N-1 to N-8: ADR-86 items 2, 3, 6 to 13; all in T-17b.
- The corrections to this note go into a dated "Corrections (T-17b)" block under it (T-17b (k)); this text stays as
  written.

### Corrections (T-17b, 2026-10-24) to the T-17 note above (the original text stays; ADR-86 items 1, 6, 8, 9, 10, 11)
- **E3 is "suggested"** (`exp_reader_rules.md`: "follows from E1, E2"), not confirmed. E1, E2, E4 are confirmed.
- **The 18c E1 case is `Unconfirmed::E1AfterFlag1In18c`**: `ff ff` after path flag 1 in an 18c file. The fixtures show only
  E2 there (exp.md 5.4); the bytes are confirmed (every 11g direct-path empty table). Evidence route: R-017. **The client
  pairs no fixture shows are `Unconfirmed::ClientSetNotInFixtures`** (11g owner mode with client 1 / 171 and database 873;
  11g table mode with client 171 and database 873; 18c table mode with client 171; 18c owner mode with client 1 / 171). No
  request: no reader path depends on the client set [C4].
- **An 18c header with database set 171 and TIMESTAMP(1-5, 7, 8) are no longer generated.** TIMESTAMP precisions are
  drawn from {0, 6, 9}. Widening waits for R-047.
- **TIMESTAMP descriptor length source**: [Y5] and [T5] with the fixtures `t_ts_exp_18c` (length 7 for (0), 11 for (6) and
  (9)) and `t_decl`; not `types/map.rs` `check_agreement` (reader code), which the T-17 note named wrongly.
- **H4's 10 bytes** are constant in 201 of 201 files (review 06b); their meaning stays unknown. **R + 0x000 to R + 0x102 are
  zeros** in the generator, and R + 0x103 is the LF (not "to 0x103").
- **"Four mistakes of my own"** listed three: a test value longer than its VARCHAR2(30), a miscounted case total, and the
  Cyrillic LONG in the 1251 database. Three, not four.
- **K1** (`fe ff` read as an empty value): 12 of the 17 T-17 tests failed in a `--no-fail-fast` run (review 06b), not "at
  least 8". **K2** (E2 test removed): the firm failures are the two deterministic tests
  (`both_empty_table_forms_after_a_direct_path_flag`, `owner_mode_with_three_tables`); the property tests fail only at
  higher case counts (N-3: 6b saw it with `PROPTEST_CASES=1000`). A property test is cited as catching a mutation only with
  its case count and "in this run".
- **`the_random_specs_cover_every_construct` measures the property test's distribution** from T-17b on: the proptest
  strategy is `any::<u64>()` mapped through `params_from_seed` (N-4), so both use one distribution.
- **N-1 (a) to (e) differ from the fixtures; the reader does not depend on them** ([T2], [T7], [H6]): (a) one trailer
  group set per file, the fixtures write one per table in table mode; (b) the per-LOB-column `TABLE` / `ENDTABLE` groups of
  [T1] in the trailer are not generated; (c) owner mode: group order and set differ from `own_exp_18c`; (d) the view line
  has no trailing space after `AS`; (e) R + 0x000 to R + 0x102 are zeros (comment `CONTRADICTS FIXTURES (harmless: [H6]
  skip)` in `synth.rs`). The fuzz seed set holds the 235 real fixtures with the real blocks.
- **Not generated, completed with 6b's N-5** (fixture tests as 6b found them): list K entries `CREATE FORMAT71 PROCEDURE ` and
  `CREATE FORMAT92TRIG ` (I6); `NUMBER(*,s)` / INTEGER, `FLOAT(b)` (T9); `NUMBER(p, s)` with s < 0 or s > p (T9); the T1 LOB
  trailer groups (`lob_exp_18c`); C7's
  undefined bytes (1251: 0x98); a column-field length above 32,000 (R8 "not an error"); a G1 introducing line
  (suggested; `t11_*`). Fixture tests, as 6b found by grep: `t1_walk.rs` (FORMAT71 / FORMAT92TRIG), `t5_types.rs` and
  `t5f_followup.rs` (`NUMBER(*`, FLOAT), `t11_*` (G1); 6b names none for the other items, and I did not look for one or
  measure code coverage. The earlier "not generated" items stand.

## T-17b note (Implementer, 2026-10-24): review 06b fixes (ADR-86)
- Built (`git diff --stat edc62a5` outside `docs/`): `crates/oradmp-core/tests/common/synth.rs`, `t17_synth.rs`,
  `t14_damage.rs`, `t4_values.rs`, `t5_types.rs`, `python/oradmp/_cli.py`, `tests/test_paths.py`, `tools/fuzz_seeds.sh`,
  `CONTRIBUTING.md`. Nothing in `crates/oradmp-core/src/` (checked: `git diff --stat -- crates/oradmp-core/src` empty after
  every mutation was restored), none in `corpus/`, `tests/golden/`, `tests/data/`, Cargo files, `pyproject.toml`, README.
  `docs/formats/requests.md` showed as modified in `git status` for a while during the session and is not mine (the Analyst's).
- Tests: Rust **429 -> 432 passed, 4 skipped** (3 new: `header_sets_match_the_fixtures`,
  `no_e2_table_is_followed_by_an_item_outside_the_e2_list`, `a_long_chain_in_the_shape_of_the_fixtures`); pytest **233 -> 235
  passed, 0 skipped** (2 new in `tests/test_paths.py`). `t17_synth` has 20 running tests + the ignored `write_fuzz_seeds`.
- **Table: item, test or check, what fails on `edc62a5` or under the mutation** (mutations each alone, restored after, `git
  status` clean of the mutation each time; test names are in `t17_synth.rs` unless noted):
  | item | proof | result |
  |---|---|---|
  | (a)(1)-(3) one header list | `HEADER_CHOICES` (24 entries) in `synth.rs`; `random_spec` draws its version, mode, client and database from it; `the_random_specs_cover_every_construct` asserts no 18c header with database 171; `every_generated_type_with_edge_values` uses an 11g header for 171 | on `edc62a5` the pair was drawn independently |
  | (a)(4) `header_sets_match_the_fixtures` | own byte reading of H1 to H4 in all `.dmp` of the three folders; 201 files; 18 distinct (version, mode, client, database) combinations occur (own script, same bytes); every unlabelled entry occurs, no labelled entry occurs | mutation: label removed from the 11g table-mode client 171 / database 873 entry: **FAIL** (`header_sets_match_the_fixtures`: "is unlabelled but no fixture has it"); mutation: unlabelled 18c / 171 entry added (array 25): **FAIL** `header_sets_match_the_fixtures` and `the_random_specs_cover_every_construct` ("an 18c header with database set 171") |
  | (a)(5) TIMESTAMP precisions | the coverage test panics on any precision outside {0, 6, 9} and needs each of the three >= 3 times | mutation `rng.range(0, 9)` back: **FAIL** `the_random_specs_cover_every_construct` ("seed 0: TIMESTAMP(7) drawn") |
  | (b) labels | `enum Unconfirmed { E1AfterFlag1In18c, ClientSetNotInFixtures }`; `Layout.unconfirmed`; `verify` appends it to every error, the property tests print it; `both_empty_table_forms_after_a_direct_path_flag` asserts `{E1AfterFlag1In18c}`, a file with E2 tables only holds none, the 11g E1 test asserts none | mutation: the 18c E1 site no longer inserts its variant: **FAIL** `both_empty_table_forms_after_a_direct_path_flag`. `grep -c 'UNCONFIRMED(' synth.rs` = **2** = the two writing sites: the E1 `ff ff` in `write_table` and the client set in `write_header` |
  | (c) SF-2 | `no_e2_table_is_followed_by_an_item_outside_the_e2_list`: the 14 literals; bytes checked at `end_at + 1`; seeds 0..40,000 with `max_rows` 3 (builds only files with an E2 table); 2,753 E2 tables; **5.7 s** in a debug build (`-j 2`, VM idle). The bytes were fast enough, so the byte check is used (not the item list) | mutation `i == 0` back: **FAIL** at seed 754; counting instead of asserting: **29 hits** in 40,000 seeds (6b: 131 in 200,000) |
  | (d) SF-3 | `number_types_override_the_arrow_type` carries the three literals; `grep -c '\.arrow()' t17_synth.rs` = 0. `oradmp_core` items used by `t17_synth.rs`: `dump::{open, CatalogEnd, ExportMode, Options, TablePath, TableRef}`, `exp::rows::EndMarker`, `exp::table::PathKind`, `reader::ReadOptions`, `types::map::NumberType` (the input of the option under test), `Error`, `ErrorCode` (the codes compared with arch 1 and `ErrorCode::ALL`); through `common`: `try_read`, `rows`; every one is an input or the call under test | mutation (in `src/types/map.rs`): `NumberType::arrow` gives `Utf8` for `Int64`: **FAIL** `number_types_override_the_arrow_type` (and 13 others) |
  | (e) SF-4 | `_cli.py`: `_refuse_existing` and the `filename` / `filename2` branch of `_error_text` go through `_path_text` (new `_filename_text`); a grep for `_config_error(f`, `{...path...}` and `.filename` found these two sites only (the other path in a message, `_state.file`, already went through `_path_text`); tests `test_the_tool_writes_an_existing_output_target_as_escapes`, `test_the_tool_writes_the_filename_of_an_output_error_as_escapes` (the second skips as root) | mutation `_refuse_existing` without `_path_text`: **FAIL** the first; `filename` branch without it: **FAIL** the second |
  | (f) SF-5 | `failure_persistence: None` with `..ProptestConfig::default()` in `t14_damage.rs`, `t4_values.rs`, `t5_types.rs` (t17 already had it); the doc line of `t14_damage.rs` replaced. Grep: each of the four files that contain `proptest!` holds one `failure_persistence: None` (`t14_damage.rs 1`, `t17_synth.rs 1`, `t4_values.rs 1`, `t5_types.rs 1`) | mutation `ByteStream::truncated` at `offset() + 100000`: `t14_damage` fails 3 tests, `find . -name '*.proptest-regressions'` prints nothing; mutation `t4_values` rounding (`values/number.rs:297` `>= 5` -> `>= 6`): fails `props::rounding_matches_a_reference` (and two deterministic tests), `find` prints nothing |
  | (g) N-2 | `a_long_chain_in_the_shape_of_the_fixtures`: `Spec.chain_shape = Some([156, 7961 x 4])` (new field; other chains stay random); asserts `9c 80`, `19 9f` x3, `19 1f` at the offsets from `Layout.len_at` | mutation O4 (a chain stops after 4 pieces, `rows.rs` `read_chain`): **FAIL** it (also `a_32000_byte_long_and_long_raw_on_both_paths`, `batch_rows_one_and_seven`, `a_random_spec_is_read_back_as_written`) |
  | (h) N-4 | `params()` is `any::<u64>().prop_map(params_from_seed)`; `Params` carries `origin`; messages print it; `grep -n 'fn params' synth.rs` shows `params_from_seed` only | by inspection / grep |
  | (i) N-1 | comment `CONTRADICTS FIXTURES (harmless: [H6] skip)` at the zero bytes | by grep |
  | (j) N-7, N-8 | wrapped; `git diff ba8a89c -- CONTRIBUTING.md tools/fuzz_seeds.sh | grep '^+' | awk 'length > 121'` prints nothing; the script does `cd "$root"` and prints `cargo --version` once; `SYNTH_SEEDS=2 tools/fuzz_seeds.sh` from `fuzz/` prints `cargo 1.99.0 (5f94df478 2026-08-27)` (and 237 files); from `/tmp` the same line; `cd fuzz && cargo --version` alone gives `cargo 1.101.0-nightly (3960fa69b 2026-10-12)`, which the old script would have used; CONTRIBUTING has the sentence | by run |
- Deviations / remarks:
  - **Negative tests print no label**: `negative_spec()` is one fixed 18c 873/873 table-mode file; `the_generated_negative_base_is_clean`
    still builds it, and its `Layout.unconfirmed` is empty (not asserted there; the other deterministic tests' messages are plain
    `assert!` texts without the set). Every failure message that goes through `verify` and the two property tests prints it. If
    6c wants the set in every `assert!` text, it is a mechanical follow-up.
  - **`Params` fields changed** (`origin`, `header`; `users`, `v18`, `db_171`, `client_pick` and `Params::small` gone): the fuzz
    seeds of a given seed number are therefore different files from T-17's (the seed set is regenerated by `tools/fuzz_seeds.sh`
    each time; `tools/fuzz_run.sh` was not run again, not asked).
  - **Header list shape**: 24 entries (11g table mode 8, 11g owner mode 8, 18c table mode 4, 18c owner mode 4), of which 6
    are labelled `ClientSetNotInFixtures`. The distribution is uniform over the list (owner mode 50 %, database 171 33 %, 18c 33 %).
  - `header_label` panics for a header outside the list (a test that builds one by hand must use an entry of the list).
- Checks, real results: fixture checksums (`sha256sum -c SHA256SUMS` in each of the three folders): all OK; `cargo fmt --check`
  ok; `cargo clippy -- -D warnings` ok; `cargo clippy --all-targets -- -D warnings` ok; `cargo nextest run -j 2`: 432 passed, 4
  skipped; `cargo deny check licenses` ok; `ruff check` ok; `ruff format --check` ok (33 files); `mypy` ok (23 files);
  `.venv/bin/python -m pytest -rs -q` 235 passed, 0 skipped; `PROPTEST_CASES=2000 cargo nextest run -p oradmp-core --test
  t17_synth -j 2`, alone: 20 passed, 18.6 s (the property test 18.2 s); `git status --porcelain` lists only the task's files after each run;
  no `*.proptest-regressions` file anywhere. `maturin develop` not run (no rebuild needed for `_cli.py`; the tests run the
  source tree through `PYTHONPATH`).
- For the next tasks (T-16a, Reviewer 6c): Rust counts are 432 passed + 4 skipped; pytest 235 passed, 0 skipped on Linux as root
  excepted (the read-only-folder tests skip as root, so a CI runner as root shows 2 skips more: `test_cli.py` read-only folder and
  the new `test_paths.py` one); `tests/test_paths.py` skips off Linux; `tools/fuzz_seeds.sh` needs `cargo` and now runs from the repository root.

## T-16a note (Implementer, 2026-10-24): CI on GitHub Actions, Linux (ADR-79, ADR-83, ADR-87)

**Status: the workflow is written and every command of it was run locally; it has NOT run on GitHub yet.** Items of "Done when"
that need a GitHub run (the first green run, the extended job, the VM comparison of (c), billed minutes, the pasted summaries
of (j)) are open until the owner pushes and pastes (requests below). Nothing is committed (the owner commits).

- Built: `.github/workflows/ci.yml` (one file, four jobs), `CONTRIBUTING.md` (section "CI", 20 lines), this note.
  `git status --short` shows `.github/` (new), `CONTRIBUTING.md`, `docs/design/implementer-notes.md`. Nothing in
  `crates/`, `corpus/`, `python/`, `tests/`, `tools/`, the version, `Cargo.lock`, `fuzz/` (checked: `git status`).
- Jobs: `checks` (every push + dispatch), `wheel` (every push + dispatch), `extended` and `manylinux2014` (`if:
  github.event_name == 'workflow_dispatch'` only; no schedule). `manylinux2014` is a job of its own so that (e) is one
  dispatched result and a failure in the fuzz job does not hide it; it is the only reason for the 4th job.
- Actions (each pinned to the full commit SHA of a tag, resolved with `git ls-remote` on 2026-10-24; the tags are
  lightweight, so the SHA is the commit): `actions/checkout` v7.0.1 `3d3c42e5aac5ba805825da76410c181273ba90b1`;
  `actions/setup-python` v7.1.0 `6d4fd22b20465ccc0f7354476588e8df7c9ca0e0`; `actions/cache` v6.1.0
  `55cc8345863c7cc4c66a329aec7e433d2d1c52a9`; `taiki-e/install-action` v2.87.31 `1e29709c0ec63babbe8f266397a465f29a825f91`
  (installs `cargo-nextest@0.9.146` and `cargo-deny@0.20.2`, the VM's versions; I read its `action.yml` inputs and that its
  manifests list those two versions; checksums stay on). Everything else is shell: `uv==0.12.21` by pip, `maturin==1.15.0`,
  `ziglang==0.17.0`, `ruff==0.17.1`, `mypy==2.4.0`, `pytest==9.1.1`, `pyarrow==26.0.0`, `pandas==3.0.6` (the VM `.venv`'s
  versions, so counts compare), `cargo-fuzz` 0.13.2 by `cargo install --locked`. I did not read the sources of the actions,
  only the two files named.
- "Done when" (a): `permissions: contents: read`; triggers `push` (branches `[master]`) and `workflow_dispatch`; no
  `pull_request_target`; `concurrency` group `ci-<ref>-<event>` with `cancel-in-progress` (the event name is in the group so a
  push does not cancel a dispatched run); `timeout-minutes` 45 / 60 / 90 / 45; toolchain by `rustup show active-toolchain ||
  rustup toolchain install` (reads `rust-toolchain.toml`; the root pin 1.99.0); `CARGO_BUILD_JOBS: "2"` at workflow level;
  `TMPDIR` is `$RUNNER_TEMP/tmp`, written to `GITHUB_ENV` by the first step (the `runner` context is not available in job-level
  `env`, GitHub's context-availability table); runner `ubuntu-24.04` (not `-latest`), and the summary prints `ImageOS` /
  `ImageVersion`. `grep -rn 'granddad\|oradmp.git\|gogs' .github/` prints nothing (exit 1).
- (b) in the workflow order: checksums, `cargo fmt --all --check`, clippy (`--all-targets`, `--all-targets --all-features`),
  `cargo nextest run -j 2 --status-level fail --final-status-level skip` with the skip list compared with the four names
  (`diff`), `cargo deny check` (all four checks), probes build into a job venv, `ruff check`, `ruff format --check`, `mypy`,
  `pytest -rs -q` (fails when any `SKIPPED` line exists; the step first asserts `id -u != 0`).
  **Deviation (minor): the passed counts are not hard-coded in the workflow** (they would be stale at the next test commit);
  the workflow gates the skipped names and the 0-skipped pytest, and prints the passed counts in the summary; the comparison
  with the VM's count is made in this note (VM table below).
- (c) `wheel`: `bash tools/release_build.sh "$GITHUB_SHA"` (stops unless the log has `check_dist: ok`), the SHA256SUMS printed
  in the summary. (d) three venvs created with `uv venv --python`, the wheel installed with `uv pip install`, `pytest -rs -q
  -p no:cacheprovider` run **from the checkout** with the venv's own `pytest` (no `python/` on the path; the step checks that
  `oradmp.__file__` is below `site-packages`); skips must be exactly the three probe tests of `tests/test_internal.py` (sum of
  the `[n]` counts = 3), plus `tests/test_consumers.py` in the pyarrow-only venv; then `oradmp --version` must equal `oradmp
  <version>` where the version comes from the root `Cargo.toml` (`0.1.0-alpha.N` -> `0.1.0aN`, `-beta.` -> `b`, `-rc.` -> `rc`).
  The "newest CPython" is the last version of `python-version: "3.11\n3.x"` of `setup-python` (the default `python`); the
  summary prints it per venv. The third venv (Python 3.11, "pyarrow only") takes the **newest** pyarrow (26.0.0 today), so it
  also has no numpy; the plan does not give a pyarrow version for it (my reading; say if it should be 15.0.2).
- (e) `manylinux2014` job: `maturin build --release --locked --zig --compatibility manylinux2014`, `auditwheel show`, the
  `Tag:` lines of the wheel's `WHEEL`, the file name must contain `-cp311-abi3-`; no `check_dist` on it (it is built without
  the path remaps; it is never shipped). Old risk check (a) is T-16b's.
- (f) `extended`: `tools/fuzz_seeds.sh`; `tools/fuzz_run.sh read|walk|values 60` (each its own step; the script's own exit
  status is non-zero on a finding); `PROPTEST_CASES=10000` `t14_damage`, `PROPTEST_CASES=2000` `t17_synth` (nextest) and a check
  that no `*.proptest-regressions` file exists afterwards; `cd fuzz && cargo clippy -- -D warnings` with `rustup component
  add clippy` for the dated nightly (the VM's nightly has no clippy component: `rustup component list --installed`);
  `cargo bench -p oradmp-core --bench decode` as a report (the summary prints the lines around `time:`; no comparison).
  A finding: its crash file stays on the runner (no artifact step, rule (h)); the summary prints the number of files in
  `fuzz/artifacts/<t>/`; ask the owner for the log, and I would add an artifact step for the crash file only with an
  Architect answer (ADR-62 item 7 says "may be a workflow artifact the owner downloads"; (h) forbids uploads).
- (g) every job ends with `Job summary` (`if: always()`): commit, image, user, toolchain versions, counts, skip names, the
  `check_dist` line, SHA-256, duration. (h) no upload, no secret, no `corpus/work`, no memory or throughput test, no bench gate;
  `actions/cache` is used for the cargo registry (and `target/` in `checks`); no cache holds a `corpus/work` file.
- (i) `git diff --stat -- . ':!docs'` will list `.github/workflows/ci.yml` and `CONTRIBUTING.md` and nothing else; no CI
  failure has proven a fix yet.

### What I ran locally (VM, commit `4b4e732`, tree + untracked `.github/`)
The `run:` blocks were extracted from the YAML with PyYAML (a scratch script) and executed in bash with `-eo pipefail`;
every block passes `bash -n`. YAML parse found one real error (a `: ` inside a plain `name:`), fixed. No `actionlint` or
`shellcheck` on the VM.
- Clean clone of `4b4e732` (`git clone --no-local`, so only tracked files): the three `sha256sum --check` OK (317 lines in
  all); venv step with the pinned packages and `maturin develop --features test-probes` OK; `ruff check`, `ruff format
  --check` (33 files), `mypy` (23 files) OK; `pytest -rs -q`: **235 passed, 0 skipped** (as a non-root user); `cargo
  nextest run -j 2 ...`: **432 passed, 4 skipped**, the four names equal the expected list; changing one expected name makes
  the step fail (rc 1).
- Main tree: `cargo fmt --all --check` ok; `cargo clippy --all-targets -- -D warnings` and `--all-features` ok; `cargo deny
  check`: `advisories ok, bans ok, licenses ok, sources ok` (two `license-not-encountered` warnings for ISC and Zlib; they
  are warnings of the existing `deny.toml`, not failures).
- `tools/release_build.sh HEAD` (build root `/var/tmp/oradmp-t16a-release`; the default `/var/tmp/oradmp-release` exists
  on the VM, so I used another root; the root is remapped to `/build`): `check_dist: ok`, 2 min 17 s. SHA-256 of `4b4e732`:
  wheel `oradmp-0.1.0a3-cp311-abi3-manylinux_2_28_x86_64.whl` `502debb7348ce574c17d82e1a899a408c538e9f5860a93cbab02466cf40ed6a1`,
  sdist `oradmp-0.1.0a3.tar.gz` `f5aeda9fe0ed51409d966b128d2c0f2ee0bea567fae56c86e47a5314ebef820c`. (Not the T-16a commit; the
  sdist holds `crates/`, `python/` and root files only, so `.github/` does not enter it; the comparison of (c) needs the VM's
  build of the final commit: see requests.)
- The three-venv step on that wheel, as the workflow runs it: Python 3.11.16 + pyarrow 15.0.2 + pandas 2.2.3: 232 passed, 3
  skipped (the probe tests), `oradmp 0.1.0a3`; Python 3.14.7 + pyarrow 26.0.0 + pandas 3.1.0: 232 passed, 3 skipped; Python
  3.11.16 + pyarrow 26.0.0, no pandas: 225 passed, 4 skipped (the three probe tests + `tests/test_consumers.py:18`
  "could not import 'pandas'"). Mutation: no allowance for `test_consumers.py` in the pyarrow-only call -> the step fails
  ("unexpected skips"). The runs used `uv python` interpreters (3.11, 3.14), not `setup-python`.
- Nightly clippy of `fuzz/`: with a scratch `RUSTUP_HOME` holding `nightly-2026-10-15` + clippy: `cargo clippy -- -D warnings`
  in `fuzz/` finished clean (so the CI step should pass; the VM's own toolchain was not changed).
- Committed-tree check: a scratch clone with `.github/` committed passes `tests/test_project_name.py`,
  `tests/test_check_dist.py`, `tests/test_version.py` (69 passed).
- **Not run (cannot be run on the VM)**: anything on GitHub's hosted runner: `rustup toolchain install` on that image (rustup
  1.28+ does not install a toolchain automatically; the step does it), `CARGO_HOME` and the home path of user `runner` in the
  release build (the script checks the build root, not the home: `/var/tmp/oradmp-release` is outside `/home/runner` and has
  no component `runner`; `check_dist` rejects a home path), the `setup-python` multi-version `python` default, the cache
  steps, `taiki-e/install-action`, the extended job's fuzz and bench steps, `auditwheel show`, `cargo install cargo-fuzz`, and
  the summary's `ImageOS` / `ImageVersion`. The first push will show which of these needs a fix (T-16a (i): each fix a line here).

### VM counts at `4b4e732` (for the comparison with the pasted summaries)
| item | VM |
|---|---|
| `cargo nextest run -j 2` | 432 passed, 4 skipped (`t15_memory` x2, `t15_throughput`, `t17_synth::write_fuzz_seeds`) |
| `pytest -rs -q` in the probes dev build (as the owner's user) | 235 passed, 0 skipped |
| installed wheel, venv 1 / 2 / 3 | 232+3 skipped / 232+3 skipped / 225+4 skipped (all `-rs` skips named above) |
| `cargo deny check` | all four ok |
(The T-16a commit adds no test, so the counts stay as they are; the CI summary's counts must be the same.)

### Skip list per job, with reasons (old (h))
- `checks`, `cargo nextest`: the four `#[ignore]` tests (memory x2, throughput: they need `ORADMP_WORK` / `corpus/work/`, ADR-83
  item 3; `write_fuzz_seeds`: a generator run by `tools/fuzz_seeds.sh`). `checks`, `pytest`: none (non-root: the two root skips
  must not appear).
- `wheel`: venvs 1 and 2: the three probe tests (the release wheel has no `test-probes`); venv 3: those and
  `tests/test_consumers.py` (no pandas). No platform skip (Linux).

### Requests to the owner (the logs route, ADR-87 item 4)
1. Commit `.github/`, `CONTRIBUTING.md` and this note; confirm Actions are enabled (O-22 (a)); push `master` to `github`.
2. Paste: the run's web address; the **Summary** of `checks (Linux)` and of `wheel (Linux)`; for a failed job the failed step's
   log. I fix what fails (a line here for each fix) and ask again.
3. When the push run is green and the commit is the final T-16a one: dispatch the workflow once (Actions -> CI -> Run
   workflow, branch `master`) and paste the Summaries of `extended` and `manylinux2014` (and the billed minutes of that
   run, if the repository's Actions usage page shows them).
4. For (c)'s comparison: on the VM, `tools/release_build.sh <final T-16a commit>` (an Implementer session does it when the
   commit exists; I can only build HEAD today) and the CI summary's two SHA-256 lines; the sdist must be equal, the wheel
   equal or differing only in the SBOM member (the listing of differing members is `--twice`'s format).

### For the next tasks and for the Architect
- `/tmp` on the VM is a 3.9 GB tmpfs: my first local simulation (scratch clone + target + venvs below `/tmp/claude-...`) filled
  it and made a command fail with "No space left"; I removed my own scratch, re-ran below `/var/tmp/oradmp-tmp/` and deleted that
  too. Nothing of the repository or the owner's files was touched; a Reviewer building clones should use `/var/tmp`.
- `/var/tmp/oradmp-release` exists on the VM (the owner's earlier build root; `tools/release_build.sh` refuses it). Set
  `ORADMP_BUILD_ROOT` for a local run. On the runner the folder is new.
- T-16b adds jobs to this file; the `TMPDIR`-by-`GITHUB_ENV` step and the three-venv function are written for bash; on
  Windows the shell and paths differ (T-16b's problem).
- Proposal (not done): hard-coding the expected pytest and nextest passed counts would catch a silently shrunk suite, at the
  price of editing the workflow with every test commit. I left the counts to the summary and this note.

### Answers (Architect, session 23; ADR-89 to ADR-93)
- **Status**: T-16a is done at `d081109` (ADR-89; the run record is in the plan, T-16a "Session 23 record"). The note's
  first line ("has NOT run on GitHub yet") is history; T-16a2 (f) adds the run record in a note of its own and leaves
  this text as it is.
- **`tests/test_threads.py` (`7a2d028`)** is a CI-proven fix and allowed by T-16a (i); the missing line goes into the
  T-16a2 note (step, cause, fix).
- **Crash files as an artifact**: yes, on failure only, `fuzz/artifacts/` only, 14 days (ADR-92 item 1); T-16a2 (d).
- **Passed counts in the workflow**: not hard-coded (the deviation is accepted; the proposal is not taken); the summary and the note compare
  them, and Reviewer 6c compares them with its own run.
- **The third venv's pyarrow**: the newest is right (no numpy, no pandas: the minimal install); 15.0.2 is covered by the
  first venv.
- **The build root**: CI builds at the default root, which becomes the canonical release root (ADR-90): T-16a2 (a)
  rebuilds `d081109` there first (the root is free on the VM now: `/var/tmp/oradmp-release` does not exist at session
  23), then adds `--release` and `check_dist.py --build-root`.
- **`auditwheel`** is unpinned in the `manylinux2014` job: pin it (T-16a2 (d)).
- **What the owner pastes**: ADR-92 item 3 (full hashes, whole summaries). Ask for it by that name in the T-16a2 note.

## T-16a2 note (Implementer, 2026-10-25): CI follow-ups (ADR-89, ADR-90, ADR-92); refers to the T-16a note, which stays as written

### (a) The confirming rebuild (ADR-90 item 4) - done by the owner, checked by me on the files
- The owner rebuilt `d081109` with `tools/release_build.sh` (no `--release`, `ORADMP_BUILD_ROOT` unset) at
  `/var/tmp/oradmp-release`. I did not rebuild. What I read there (`dist/SHA256SUMS`):
  wheel `8405b25ba4d973f3744cfa93714a7e9a7fddd877470fac133a3433645da2dfe8`, sdist
  `ed4dbf0f8651a149956712be352f24b99c39339ea104e94be30471f284490c0d`.
- CI's `wheel` summary of `d081109` (owner's full lines): wheel `8405b25ba4d973f3744cfa93714a7e9a7fddd877470fac133a3433645da2dfe8`,
  sdist `ed4dbf0f8651a149956712be352f24b99c39339ea104e94be30471f284490c0d`. **Both equal.** The hypothesis of ADR-90
  ("the build root in the SBOM is the only difference") is **confirmed**.
- SBOM member `oradmp-0.1.0a3.dist-info/sboms/oradmp-py.cyclonedx.json`: 134,557 bytes at the canonical root (read from the
  VM wheel; the expected value; CI's line of the member listing was not pasted to me, the equal wheel hash implies it);
  134,581 bytes at `/var/tmp/oradmp-release-ci2` (+24 = 6 paths x 4 characters). Comparing the two VM wheels member by
  member (SHA-256): exactly two members differ, the SBOM and `RECORD` (1,293 bytes in both, different content).
- All 6 build paths of the canonical SBOM begin with `/var/tmp/oradmp-release/src/` (checked by the new
  `check_dist.py --build-root /var/tmp/oradmp-release`: `check_dist: ok`; with `--build-root /var/tmp/oradmp-release-ci2`
  it prints 6 FAIL lines with byte offsets 342, 822, 1276, 100888, 101515, 101787).
- **Not done by me**: the `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-d081109` (the folder is the owner's build, and
  the script refuses an existing root; the tests of `--release` do not need the folder to be absent). **Owner: do this
  `mv` before the push run is compared and before the a4 release build.**

### What was built
- `tools/release_build.sh`: `--release` (the root after `realpath -m` must equal `realpath -m /var/tmp/oradmp-release`, else
  exit 1 with "`--release` needs the canonical build root <canonical>, but the build root is <root>"; the check comes before
  the home, user-name and exists checks and before any write; prints `release root: canonical`); `--help` text (lines 2 to 22)
  names it; `--build-root "$ROOT"` is passed to `check_dist.py` on both builds (also without `--release`).
- `tests/check_dist.py`: `--build-root ROOT` (before the two archive paths). Every `path+file://` reference and every
  path below `/var/tmp/` or `/tmp/` (not the tail of a longer path) in a member under `<dist-info>/sboms/` must begin with
  `<ROOT>/src/`; one FAIL line per hit with the byte offset and the path length (the path itself is not printed beyond
  what the roots show). It prints `check_dist: build root <ROOT>` (or `(not given)`). Without the option the result is unchanged.
  A missing value or an option in the wrong place gives the usage text and exit 2.
- `tests/test_check_dist.py` (+7 cases, "T-16a2 (c)") and the new `tests/test_release_build.py` (5 cases).
- `.github/workflows/ci.yml`: `wheel` job calls `release_build.sh "$GITHUB_SHA" --release` and prints the `release root:` line in its
  summary; `extended` ends with `actions/upload-artifact` (`if: failure()`, `path: fuzz/artifacts/`, `if-no-files-found: ignore`,
  `retention-days: 14`, name `fuzz-artifacts`), pinned to `cf430e030ddbb5b0abf93d22962f4752f3646cd9` = tag `v7.0.2`
  (`git ls-remote --tags https://github.com/actions/upload-artifact`; `v7` points to the same commit; `v7.0.1` is
  `043fb46d...`); `manylinux2014` pins `auditwheel==6.8.2` (the newest on PyPI today, `pip index versions`; the version the
  dispatched run printed was not recorded: it was unpinned) and prints `auditwheel --version` in its summary. The header
  comment of the file says "no publish step, no upload but the fuzz artifact". Nothing else changed.
- `CONTRIBUTING.md`: "Release wheel" (`--release`, the canonical root, the claim) and "CI" (artifact on a fuzz failure).
  The claim as written: at the canonical root `d081109` gave the same wheel and sdist on the VM and on the runner; a
  wheel at another root differs in SBOM and `RECORD`; nothing for any other machine. It is ADR-90 item 6's "after item 4
  confirms" form, limited to what was measured (`d081109` was built without `--release`, which did not exist yet).

### (b) to (e): proof
| item | test | mutation tried |
|---|---|---|
| (b) another root refused, both paths named, nothing written | `test_release_build.py::test_release_refuses_another_root_and_names_both_paths`, `..._a_root_that_resolves_elsewhere` (symlink) | refusal line deleted: both fail |
| (b) canonical accepted (default, by name, trailing `/`, `..`), without `--release` any root builds as before, `--help` | the other 3 cases in `test_release_build.py` | refusal made unconditional: the "accepts" case fails |
| (c) other root FAIL with offsets, right root passes, option absent unchanged, usage error, script passes the root | the 7 `test_t16a2_*` cases | prefix test replaced by `if False`: 3 cases fail (other root, `srcx`, `main`); `--build-root "$ROOT"` removed from the script: the script case fails |
| (d) YAML parse and `bash -n` of all 33 `run` blocks; the last step of `extended` is the artifact step as specified | by hand on the VM (a throw-away script in the scratchpad with PyYAML); `grep -rn 'granddad\|oradmp.git\|gogs' .github/` prints nothing | - |
The `test_release_build.py` cases use a commit that does not exist, so no clone, venv or build is made and nothing is created
below `/var/tmp`; they pass whether or not the canonical folder exists.

### Checks run (VM, tree of this task, `CARGO_BUILD_JOBS=2`)
`cargo fmt --all --check` ok; `cargo clippy --all-targets -- -D warnings` ok; `cargo nextest run -j 2`: 432 run, 432 passed,
4 skipped (no Rust file changed); `.venv/bin/ruff check`, `ruff format --check`, `mypy` clean; `.venv/bin/python -m pytest -q -rs`:
**247 passed, 0 skipped** (235 + 7 + 5). `cargo deny check` was not run (no dependency change). The `--all-features` clippy
was not run (no Rust change).

### (f) Run record of the T-16a pushes (what the owner pasted; ADR-89)
- `c1d1c58` (first push of the workflow): the run's detail was not pasted to me; the plan's record says the first full green run was at `d081109`.
- `7a2d028`: `test_two_threads_run_in_parallel` had failed once on the 2-vCPU runner (1.67 against 1.6); the commit replaced it; green in three "Re-run all jobs" runs.
- `d081109` (push, then `extended` and `manylinux2014` dispatched), figures from the plan's "Session 23 record": `checks` nextest 432 passed, 4 skipped, `cargo deny` ok, pytest 235 passed, 0 skipped (3 min);
  `wheel` `check_dist` ok, wheel `8405b25ba4d973f3744cfa93714a7e9a7fddd877470fac133a3433645da2dfe8`, sdist `ed4dbf0f8651a149956712be352f24b99c39339ea104e94be30471f284490c0d`, pytest 232/3, 225/4, 232/3 (4.3 min);
  `extended` 435 seeds, fuzz `read` 58,010 / `walk` 173,513 / `values` 8,100,212 runs, 0 findings, `t14_damage` 10,000 cases, `t17_synth` 2,000 cases (20 passed, 1 skipped), fuzz clippy ok (10.6 min);
  `manylinux2014` tag `cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64`, newest symbol `GLIBC_2.14` (3.5 min).
- **The line for `tests/test_threads.py` (T-16a (i))**: failing step `pytest` of `checks (Linux)`; cause: the test compared the
  time of two threads reading in parallel against one thread (ratio 1.67 against the limit 1.6), and a shared 2-vCPU
  runner makes that ratio noisy; fix (`7a2d028`): `test_counter_thread_progresses_during_read` (a counter thread must
  advance while a read runs; no timing).
- Billed minutes: not given to me (O-23 (b)); durations above.

### (g) Open: needs the owner
1. `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-d081109` (see (a)).
2. Commit and push T-16a2; then paste, per ADR-92 item 3, the run's web address and event, the full commit hash, for each
   of `checks` and `wheel` the conclusion and the whole Summary verbatim (hashes in full). Check: the `wheel` summary shows
   `release root: canonical` and `check_dist: build root /var/tmp/oradmp-release`; its sdist hash equals the VM's build of
   that commit (build it with `tools/release_build.sh <commit> --release` after the `mv`; the wheel should then be equal
   too); `checks` pytest should read 247 passed, 0 skipped, nextest 432 passed, 4 skipped. The `wheel` job's venv pytest
   counts grow by the 12 new tests only if those files are collected there (the three venvs run the whole `tests/`
   folder: expect 244/3, 237/4, 244/3 against 232/3, 225/4, 232/3).
3. No dispatch is needed for the artifact step (YAML and `bash -n` only). The first real failure of `extended` will show it.

### For the next tasks
- Release builds: `tools/release_build.sh <C> --twice --release`, canonical root absent, `ORADMP_BUILD_ROOT` unset.
- `check_dist.py` now has a `main(argv)` with an optional `--build-root`; the wheel's SBOM check applies only with it.
- `tests/test_release_build.py` runs the script with a bogus commit; if the script's check order changes (root rule after the
  commit lookup), those tests fail on purpose.

### Answers (Architect, session 24; ADR-94 to ADR-98)
- T-16a2 is **done** (`af6a54a`; CI and VM hashes identical, owner). ADR-90 item 4's hypothesis is confirmed by (a). The
  open item (g) 1, the `mv` of `/var/tmp/oradmp-release`, stays the owner's and comes before any release build (plan,
  order table session 24, row 1').
- Next task: **T-19** (plan; ADR-95 for the reader, ADR-97 for the scope). Note for it: the corpus tests are red since
  `cca4c65` (ten new fixtures, five new truth files; ADR-94 item 5), so the first run of T-19 shows the count failures
  before any reader change; the counts come from the Analyst's tools, never from the reader.

## T-19 note (Implementer, 2026-10-25): the conventional LONG forms of R-040 and the ten session 17 fixtures (ADR-94, ADR-95, ADR-97)

Not committed. Checks were run on a scratch checkout of `eac994b` plus this diff (see "Where the checks ran", the owner's
tree changed under me).

### Where the checks ran (needs the owner's attention)
While I worked, `corpus/` of the owner's tree got new, uncommitted files (`18c/long4_*`, `longraw4_*`, `tsltz2_*`, their
logs, `truth_long4.txt`, `truth_longraw4.txt`, `truth_tsltz2.txt`, a changed `18c/SHA256SUMS`, `corpus/sql/17_*`,
`corpus/scripts/make_pending2.sh`): fixtures for R-048/R-050, which belong to T-19b, not to T-19. Every test that lists
the folders sees them and counts differ from the 211 files of the task. I did not touch them. All Rust and Python checks
below ran in `/var/tmp/oradmp-t19` (`git archive eac994b` plus the files of this diff, `corpus/` as committed) with a copy of the
`.venv` (`/var/tmp/oradmp-t19-venv`, its `oradmp.pth` pointing at the scratch `python/`), so the owner's `python/oradmp/_core.abi3.so`
was not replaced. The three golden generators were run before the new files appeared (they assert 211 files / 404 sections /
418 parts and would fail otherwise). In the owner's tree the corpus tests fail until the extra files are committed with
their own task (T-19b) or moved away. Mutations were applied only in the scratch checkout; the owner's tree was never
mutated (`git status` of the diff above is the whole change).

### Built
`crates/oradmp-core/src/exp/rows.rs` (the three field forms, `read_chunked`, `RawHasher` markers, `LongStats.chunked`,
`chunked_short`, `first_chunked_short`, unit tests), `exp/items.rs` (the `W-UNTESTED-LAYOUT` of a short chunked value),
`dump.rs` (`add_long`), `values/long.rs` (thresholds by type), `reader.rs` (the `W-LONG-LENGTH` texts, unit tests),
`oradmp-py/src/conv.rs` (`chunked` into the dict), `python/oradmp/_api.py` (`LongStats.chunked`, last field),
`_errors.py`, `_survey.py`, `README.md`; tests: new `crates/oradmp-core/tests/t19_long.rs`, new family checks in
`common/truth_checks.rs` (`long3_diffs`, `tsp_diffs`, `seg_diffs`), `common/mod.rs` (`TOTALS`, `FOLDER_TOTALS`, three
patched-copy helpers), `common/synth.rs` and `t17_synth.rs` (generator), the golden files and generators, and the existing
tests that named 201 files or the old stops (t2, t3, t3a, t3c, t4, t5, t8, t9b, t10, t11, t12, t14, t17, pytest).

### Behaviour as built (ADR-95), with the points where I had to choose
- Table of item 1 as written. The first arm of the `match` in `step_with` is `0 | 0xfff0..=0xfffd` guarded by "not a
  conventional LONG column, or the length is 0"; the new arms follow (LONG RAW from 0x8000, LONG 0xfff1 to 0xfffc, LONG
  `fd ff`). The direct path is unchanged (its `match` arm comes first).
- **Offsets of a chunked value**: the sink's first piece offset, the `W-UNTESTED-LAYOUT` offset and the `E-LOB-TOO-LARGE`
  offset are the first data byte (behind the first chunk's u16). The empty last piece carries the same offset (the
  sink uses the offset of the first piece only). An empty value (`fd ff 00 00`) has no data byte; its offset is the
  byte behind the `00 00` (`field + 4`).
- **"Once per column and table part"** (item 2): implemented as the zero-piece warning is, once per column and **table section**
  (`chunked_short_warned` beside `zero_piece_warned` in `items.rs`), at the first such value of the first part that has one.
  If the Architect meant once per part, the change is one line.
- `LongStats` has two more Rust fields than ADR-95 item 8 names: `chunked_short` (count) and `first_chunked_short` (row,
  offset), the walker's input for the warning. Not in Python.
- `W-LONG-LENGTH` text for LONG RAW: "LONG RAW value above 32,000 bytes, the largest confirmed" (the type is taken from the column kind
  in `long_piece`). `above_32000` unchanged in name and meaning.
- Stop errors of the scan are not `Err` of `Dump::scan`: they end the catalog `Stopped(e)` (ADR-12), as before; the chunk
  case (`Framing`) is an `Err` of `scan`, as every `Framing`. Python: the stop error is `Catalog.stop_error` (an
  `UnsupportedError`, `.column` is `None` because the table is not complete; "column 2, row 4" stands in the text).

### Done-when items, each with its proof
| item | test (Rust unless named) | result |
|---|---|---|
| (a) reader, all arms | `exp::rows::tests::{a_chunked_conventional_long_reaches_the_sink_as_pieces, a_conventional_long_length_with_bit_15_set_is_one_field, the_refusals_of_the_conventional_long_forms, the_digests_of_a_chunked_value}`, `reader::tests::{chunks_with_an_empty_last_piece_and_a_character_cut_between_them, the_w_long_length_thresholds_and_texts_by_type}`, `t3a r8_...` (LONG 0 / 0xfff1 / 0xfffc, other columns), `t3c d_...` (LONG RAW) | pass |
| (b) four long3 files | `t19_long::{t_long3_equals_the_truth_in_all_four_files, the_long3_check_notices_a_changed_truth, t_frame_of_the_long3_files_equals_its_truth, long_stats_and_no_warning_in_the_long3_files, the_assembled_long_digest_is_the_same_in_all_four_files, the_long3_files_scan_to_the_end}` | pass; LongStats conv `7/0/3/100000/7`, direct `7/7/0/100000/7`, no warning with `strict` either |
| (c) R-048 stops | `t19_long::{a_conventional_long_of_0xfff1_to_0xfffc_stops_the_file_by_name, a_conventional_long_raw_from_0x8000_stops_the_file_by_name, a_chunk_length_from_0xfff0_is_framing_at_the_chunk, the_largest_chunk_length_is_not_refused}`; pytest `test_errors.py::test_a_conventional_long_of_0xfff1_stops_the_scan_with_the_r048_error`, `..._long_raw_from_0x8000_...`, `test_cli.py::test_exit_1_a_conventional_long_stop_of_r048_...` | pass |
| (d) memory guard | `t19_long::the_bound_is_checked_as_the_chunks_and_pieces_accumulate` (68,000 at 0x522fa; 71,568 at 0x522fd; rows 1 to 6 read; null policy; 100,000 reads) | pass |
| (e) tsp, seg | `t19_long::{t_tsp_equals_the_truth_in_all_four_files, the_tsp_check_notices_a_changed_truth, the_tsp_descriptors_and_arrow_types, the_seg_files_hold_three_empty_tables_with_the_end_markers_of_the_truth}` | pass |
| (f) counts, golden | `common::TOTALS` / `FOLDER_TOTALS` (211 = 95/89/27, 404, 418, 41,284, 243,303, 19,960; empty 27 + 20; NUMBER 86,124; 1,981 column definitions), truth listing 87, every `201` replaced; `gen_*` rerun | pass |
| (g) generator | `t17_synth::{a_conventional_long_up_to_65520_bytes_is_one_field, a_chunked_conventional_long_in_the_shapes_of_the_fixtures, a_direct_path_chain_of_100000_bytes, a_long_above_100000_bytes_gives_w_long_length_and_100000_none, conventional_long_refusals_have_their_code_offset_and_message, a_chunked_total_below_65535_bytes_is_read_with_a_warning, the_e1_label_applies_to_18c_owner_mode_only, the_generator_draws_large_long_sizes_..., the_random_specs_cover_every_construct (ten TIMESTAMP precisions), header_sets_match_the_fixtures (211)}` | pass |
| (h) damage, fuzz | `t14_damage` over `idx % files.len()` (no literal) with three new truncation sites at the chunk framing; `tools/fuzz_seeds.sh` (445 seeds = 245 fixture `.dmp` files, 235 + the 10 new, + 200 synthetic; run in the scratch checkout, `fuzz/corpus` of the owner's tree untouched); `fuzz_run.sh` 600 s each in the scratch checkout, loaded VM (see below): `walk` 781,609 runs, `read` 391,330, `values` 78,229,814, 0 findings (`fuzz/artifacts/*` empty); `PROPTEST_CASES=2000` on `t17_synth`: 28 passed, 1 skipped, 37.5 s for the binary; `PROPTEST_CASES=10000` on `t14_damage`: 5 passed, 14.0 s | pass |
| (i) Python | `LongStats.chunked` last, docstring, survey prints `chunked`, `UnsupportedError` docstring; `test_survey.py` (`chunked 3` for `long3_exp_18c`, `chunked 0` for `long_exp_direct_18c`, `chained 7` for the direct 11g file) | pass |
| (j) README | the sentence of ADR-98 item 5 in "What it reads"; "Limits" line | done |
| (k) messages | `t12_messages::no_message_of_the_fixtures_holds_a_value` takes five patched copies and asserts the five texts are among the messages seen | pass |

### Numbers taken from the Analyst's tools (read-only runs, 2026-10-25)
- `corpus_counts.py` per folder: 18c 95 files, 182 sections, 9,561 rows, 31,733 fields, 2,286 NULL, empty conv/direct 15/12; 11g 89, 170,
  3,347, 13,091, 2,270, 10/7; 11g-cp1251 27, 52, 28,376, 198,479, 15,404, 2/1. Sum 211, 404, 41,284, 243,303, 19,960, 27 + 20; `problems 0`.
- `rowlen_stats.py`: 418 parts (196/170/52), `lob_values` 126 (80 short + 46 long), `long_conv_8000_up` 6, `long_conv_chunked` 6, largest LONG 100000, 12 findings.
- `num_decode.py` step 3: 86,124 NUMBER fields decoded (86,042 compared). `collist.py`: 404 CREATE TABLE lines, 1,981 column definitions, moved sections 8.
- `long3_check.py` offsets and forms (used for the patched copies): conv L at 0x2271, 0xa280, 0x12290, 0x2228f (`f0 ff`), 0x3228f (`fd ff`, 16 x 4000 + 1535), 0x422c2, 0x522f6 (25 x 4000); direct 0x2279 ... 0x522fb (12 x 7952 + 4576; 18c row 1 is 923 + 4 x 7961).
  Non-NULL TIMESTAMP fields 206 (script over `exp_parse.py`, as in T-4: 122 + 84).
- Derived by python over the golden files: SHA256SUMS 332 lines / 245 `.dmp`; 156 sections listed in `truth_counts.txt`; 20 parts with a LONG column;
  397 tables compared with a log line; 150 tables / 16,497 rows of the 85 direct files' conventional partners.

### Mutations (each alone, in the scratch checkout, restored after each)
| | mutation | fails (names) |
|---|---|---|
| M1 | chunk loop stops after the first chunk | 9 of `t19_long`, among them `t_long3_equals_the_truth_in_all_four_files`, `long_stats_and_no_warning_in_the_long3_files` |
| M2 | plain length read as `n & 0x7fff` | 11 of `t19_long`, same two |
| M3 | plain range ends at 0xffef (`0xfff0..=0xfffc` refused) | 11 of `t19_long`, `a_chunk_length_...`, `the_largest_chunk_length_is_not_refused` |
| M4 | the `00 00` not consumed | 9 of `t19_long`, same two |
| M5 | LONG threshold back to 32,000 | `long_stats_and_no_warning_in_the_long3_files`, `t_long3_equals_the_truth_in_all_four_files`, `w_long_length_is_above_100000_bytes_on_both_paths` (+2) |
| M6 | 0xfff1..0xfffc as a plain length | `a_conventional_long_of_0xfff1_to_0xfffc_stops_the_file_by_name`, `t17_synth::conventional_long_refusals_...`, `each_refusal_of_architecture_1_4_...`, `rows::the_refusals_...` |
| M7 | LONG RAW treated as LONG | `a_conventional_long_raw_from_0x8000_stops_the_file_by_name`, `t3c d_...`, `t17_synth::conventional_long_refusals_...`, `rows::the_refusals_...` |
| M8 | chunk u16 from 0xfff0 accepted | `a_chunk_length_from_0xfff0_is_framing_at_the_chunk`, `t17_synth::conventional_long_refusals_...`, `rows::the_refusals_...` |
| M9 | bound checked only at the value's end | `the_bound_is_checked_as_the_chunks_and_pieces_accumulate` (the `length` assertion) |
| M10 | chunks serialised with the chain markers in `gen_exp_raw.py` (exp_raw.tsv regenerated) | `t2_manifest::x_walker_with_hashing_reproduces_every_line_of_the_manifest` |
| M11 | the generator's chunk terminator dropped | `t17_synth::{a_chunked_conventional_long_in_the_shapes_of_the_fixtures, a_chunked_total_below_65535_bytes_is_read_with_a_warning, a_long_above_100000_bytes_..., conventional_long_refusals_...}` |
| M12 | 65,525 added to the large-size edge list | `t17_synth::the_generator_draws_large_long_sizes_outside_the_r048_gap_and_only_in_the_long3_combination` |

### Checks run (scratch checkout of `eac994b` + this diff, `CARGO_BUILD_JOBS=2`)
`cargo fmt --check` ok; `cargo clippy -- -D warnings` and `cargo clippy --all-targets -- -D warnings` ok; `cargo nextest run -j 2`: 463 run, 463 passed, 4 skipped (432 + 31 new); `cargo deny check`: advisories, bans, licenses, sources ok; fixture checksums: `sha256sum -c` ok in the three folders; `maturin develop --features test-probes` (in the venv copy), `ruff check`, `ruff format --check`, `mypy` clean; `pytest -rs`: 249 passed, 1 skipped (`test_project_name.py`: the scratch folder is not a git clone; it runs in the owner's clone); the fuzz runs were with the VM loaded (load 1 to 1.5, the idle `ora11` container up), not "alone" as (h) asks.

### Items not done, and why
- **(m) step 3b not run**: no `ORADMP_WORK` was given in the task, an Oracle XE container (`ora11`) is up, and the 1-minute load was 1.1 to 1.5
  (the owner was generating fixtures). The micro-benchmarks and the memory test of `benchmarks.md` section 5 therefore have no figure here; they
  are due before the a4 cut (the core changed) and need the work folder and the machine alone.
- The push run (CI) is the owner's.

### Problems and observations
1. The corpus of the owner's tree changed during the task (above).
2. `Dump::part_rows` / `Dump::rows` open the stream with `options.buffer_bytes` as given (`dump.rs`); the reader clamps to 128 KiB. A plain
   field of up to 65,520 bytes needs a buffer of that size on the sink path, so a caller with `Options.buffer_bytes` below it gets
   `E-TRUNCATED` ("file ends inside a field") instead of the value. Not reachable from Python (the option is not exposed); proposal: clamp
   `.max(1 << 17)` in `dump.rs` where `rows`/`part_rows` open the stream (one line). I did not change it (not in the task).
3. `t3a`'s R8 test and `t3c`'s 0x8000 test changed by the Architect's own table: 0xfff1 to 0xfffc in a LONG column and 0xfff0 / 0xfffd in a LONG RAW
   column are now `UnsupportedLayout` (a stop of the catalog, not an `Err` of the walk), not `Framing`.
4. The `long` column of `walk_parts.tsv` gained a sixth field (`:chunked`) from `gen_walk_expected.py`; the test that parses it changed with it.
5. `cargo deny check` prints one `license-not-encountered` warning (`Zlib`) as before; advisories, bans, licenses, sources ok.
6. `t17_synth` property cases with a LONG above 100,000 bytes run lenient (`strict` would turn `W-LONG-LENGTH` into `E-STRICT`); the Spec decides
   the expected warning count (`long_columns_over_100k`), never the reader.
7. Direct-path chunked totals: the plan's 71,568 (9 pieces of 7,952) holds for 18c and 11g (`long3_check.py`: both 12 x 7,952 + 4,576 in row 7).

### For the next tasks
- T-19b moves the boundary (ADR-97 item 4): the two arms in `rows.rs` (`0xfff1..=0xfffc if conventional_long` and `conventional_long && is_raw && n >= 0x8000`)
  and `CHUNKED_MIN_TOTAL` are the places; the stop tests in `t19_long.rs`, `t17_synth.rs` and `exp::rows::tests` are written to be edited with them.
- Corpus tests take `TOTALS` from `common/mod.rs`; adding fixtures means the five generators and that table (and `t10_truth.rs`'s `DELEGATED` for new tables).

### Answers (Architect, session 25; ADR-99 to ADR-104)
- T-19 is **done** (`921971f`; CI green on `bbf921b`). The owner's tree changing under the task was the owner's fixture run
  (R-048 to R-051); those files belong to T-19b, which works in the owner's tree with them present.
- Deviations (ADR-102): (1) once per column and table **section**: accepted, ADR-95 item 2 amended; (2) the two Rust-only
  `LongStats` fields: accepted; (3) the missing 128 KiB minimum in `dump.rs`: a defect, fixed in T-19b with one constant at
  every `ByteStream::new` and a test; (4) chunked offsets at the first data byte, the empty value at `field + 4`, `.column`
  `None` on a scan stop, lenient property cases above 100,000 bytes: accepted.
- Observation 3 (the R8 tests moved from `Framing` to `UnsupportedLayout`): correct for T-19; T-19b moves them again to
  ADR-99's table (0xfff1 to 0xfff9 lengths, `fb ff` / `fc ff` the only stops, out-of-range n `Framing`).
- Item (m) is now **T-19 (m)**, after T-19b, alone, with the 600 s fuzz runs (plan).
- Next task: **T-19b** (plan; ADR-99 for the reader, ADR-103 for the scope). The TSLTZ warning the owner's list calls
  "untested layout" is the `W-UNVERIFIED-TYPE` of `Kind::Tsltz` (ADR-100 item 1).

## T-19b note (Implementer, 2026-10-25): the three conventional LONG / LONG RAW forms, the 18 session 19 and 20 fixtures, the TSLTZ warning (ADR-99 to ADR-103)

Not committed. All checks ran in the owner's tree (the new fixtures present, `corpus/` untouched: `sha256sum -c` ok in the three folders
before and after). `maturin develop --features test-probes` replaced the owner's `python/oradmp/_core.abi3.so` with the same kind of
build (the definition of done asks for it).

### Built
- `crates/oradmp-core/src/exp/rows.rs`: the table of ADR-99 item 1 for type 8 and 24 alike (the `raw_cols` field is gone): `0xfffb | 0xfffc` stop,
  `0xfffa` form (b) (`read_four_byte`: n checked at field + 2, z at field + 4, value at field + 6, sink `value`, hashed as `f9 ff ff ff` + u32 n + bytes),
  `0xfffd` chunked for both types, 0xfff2 to 0xfff4 one field counted in `fff2_to_fff4` / `first_fff2_to_fff4`, `CHUNKED_MIN_TOTAL` 65,534,
  `LongStats::count_value` (one place for values / largest / above_32000 / `sizes_65521_to_65534`, all paths and forms), widened chunk text, docs.
- `exp/items.rs` (the 0xfff2 to 0xfff4 warning once per column and section; the chunked text), `dump.rs` (`add_long`; `MIN_BUFFER_BYTES` at both
  `ByteStream::new`), `io.rs` (`MIN_BUFFER_BYTES = 1 << 17`), `reader.rs` (`MIN_BUFFER_BYTES`; no TSLTZ event; `W-LONG-LENGTH` 100,000 for both types, text
  "LONG RAW value above 100,000 bytes, the largest confirmed"), `values/datetime.rs` (`decode_tsltz` returns the timestamp only), `values/long.rs`
  (`above_confirmed_length(total)`), `error.rs` (docs).
- Python: `oradmp-py/src/conv.rs`, `_api.py` (`LongStats.sizes_65521_to_65534`, last field, docstring), `_survey.py` (LONG line ends `65,521 to 65,534 bytes N`),
  `_errors.py` (docstring example), `README.md` (ADR-104 item 4 sentence, "a LONG or LONG RAW above 100,000 bytes", the two Limits lines of (k)).
- Tests: `t19_long.rs` (T-19b section at the end; the R-048 stops and the 65,535 warning test rewritten), `common/truth_checks.rs` (`long4_diffs`,
  `long5_diffs`, `longraw4_diffs`, `longraw5_diffs`, `tsltz2_diffs` and their pure `longraw_diff`, `tsltz2_diff`), `common/mod.rs` (`TOTALS`,
  `FOLDER_TOTALS`, `long4_fff2_copy`, `long3_chunked_short_copy` now 65,533 bytes), `common/synth.rs` + `t17_synth.rs` (generator), `t10_truth.rs`
  (`FAMILIES`, `DELEGATED` for T_LONG4, T_LONG5, T_LONGRAW4, T_LONGRAW5, T_TSLTZ2; `TRUTH_NOT_OPENED` gains `11g/truth_r049.txt`; listing 97;
  `EXPECTED_WARNINGS` gains `T_TSLTZ2 W-TSLTZ 4`), `t11_strict.rs` (`T_TSLTZ2` in the documented list), `t12_messages.rs`, `t14_damage.rs`
  (four new truncation sites in form (b)), `t2`, `t3`, `t3a`, `t3c`, `t4`, `t5`, `t5f`, `t8`, and pytest (`test_errors.py`, `test_cli.py`, `test_survey.py`).
- Generators run: `gen_exp_catalog.py`, `gen_exp_raw.py` (form (b) from `long_fa` as `f9 ff ff ff` + u32 n + bytes), `gen_walk_expected.py` (the `long` column of
  `walk_parts.tsv` gained a seventh field, the band count, from the lengths `exp_parse.py` returns), `gen_number_parts.py`; golden / expected files
  regenerated, none edited by hand (`gen_exp_raw.py` regenerates `exp_raw.tsv` byte for byte).

### Done-when items
| item | test | result |
|---|---|---|
| (a) reader | `exp::rows::tests::{the_one_field_sizes_of_the_table_read_for_both_types, the_four_byte_length_form_reads_and_is_checked, a_truncation_inside_the_four_byte_length_form_is_truncated, the_refusals_of_the_conventional_long_forms, the_chunked_threshold_is_65534_bytes, the_digests_of_a_four_byte_length_value}`, `reader::tests::{a_value_above_100000_bytes_gives_w_long_length, the_w_long_length_thresholds_and_texts_by_type}`, `t3a r8_...`, `t3c d_...` | pass |
| (b) TSLTZ | `t19_long::{t_tsltz2_equals_the_truth_in_all_four_files, the_tsltz2_check_notices_a_changed_truth, the_tsltz2_descriptors_types_and_warnings}`; `t5f_followup` 16 columns; `t11_strict` list | pass |
| (c) buffer | `t19_long::{a_small_buffer_is_raised_for_the_row_cursor, read_table_with_a_small_buffer_reads_the_big_rows}` (`buffer_bytes = 4096`, `rows` and `part_rows`, `long4_exp_18c` row 5 and `long3_exp_18c` row 4) | pass |
| (d) 14 files | `t19_long::{t_long4_..., t_long5_..., t_longraw4_..., t_longraw5_equals_the_truth_in_all_four_files, the_new_long_checks_notice_a_changed_truth, t_frame_of_the_new_long_files_equals_its_truth, long_stats_and_no_warning_in_the_new_long_files, the_assembled_long_digest_agrees_between_the_paths_and_the_versions, the_raw_digest_of_form_b_is_the_marker_the_length_and_the_bytes}` | pass; LongStats exactly as in the plan; no warning with default and `strict` |
| (e) stops, framing | `t19_long::{a_field_starting_fb_ff_or_fc_ff_stops_the_file_by_name, the_stop_is_the_same_in_the_long3_files, a_long_raw_field_starting_fb_ff_stops_and_the_table_before_it_reads, the_four_byte_length_is_checked_at_n_and_at_z, a_chunk_length_from_0xfff0_is_framing_at_the_chunk}`; pytest `test_errors.py::{test_a_conventional_long_field_starting_fb_ff_stops_the_scan_with_the_r8_error, test_a_long_raw_field_starting_fc_ff_...}`, `test_cli.py::test_exit_1_a_conventional_long_stop_of_r8_...` | pass |
| (f) guard | `t19_long::the_memory_guard_applies_to_form_b` (0x3228b, 65,532; null policy; V `v65532`) | pass |
| (g) counts, golden | `TOTALS` 229 = 105 / 97 / 27, 436, 450, 41,494, 243,905, 20,084, 86,236 NUMBER, 2,075 definitions; truth listing 97; SHA256SUMS 360 lines / 263 `.dmp`; every literal 211 / 404 / 418 / 87 replaced; 5 `DELEGATED` rows | pass |
| (h) generator | `t17_synth::{a_conventional_long_up_to_65529_bytes_is_one_field, a_conventional_long_of_65530_to_65533_bytes_is_the_four_byte_form, a_chunked_conventional_long_in_the_shapes_of_the_fixtures, conventional_long_refusals_have_their_code_offset_and_message, a_chunked_total_below_65534_bytes_is_read_with_a_warning, a_length_of_0xfff2_to_0xfff4_is_written_labelled_and_read_with_a_warning, the_generator_draws_large_long_sizes_outside_the_fff2_gap_and_only_in_the_fixtures_combination, header_sets_match_the_fixtures (229)}` | pass |
| (i) damage, seeds | `t14_damage` (no literal; four sites in form (b)); `tools/fuzz_seeds.sh`: 463 seeds = 263 fixture `.dmp` (245 + 18) + 200 synthetic; `PROPTEST_CASES=2000` on `t17_synth`: 30 passed, 1 skipped, 43.8 s; `PROPTEST_CASES=10000` on `t14_damage`: 5 passed, 14.0 s | pass |
| (j) Python | `LongStats.sizes_65521_to_65534`; `test_survey.py` (`long4_exp_18c`: chunked 2, band 6; `longraw4_exp_18c`: chunked 2, band 0); `test_errors.py::test_the_new_long_stats_field_and_a_form_b_table_read_in_python` | pass |
| (k) README | the three edits | done |
| (l) messages | `t12_messages::no_message_of_the_fixtures_holds_a_value` (seven patched copies, the seven new texts asserted among the messages seen, the two retired texts asserted absent and written in two pieces so no source line holds them whole; grep of `crates/` and `python/` finds none) | pass |

### Numbers taken from the Analyst's tools (read-only runs, 2026-10-25)
- `corpus_counts.py`: 18c 105 files, 200 sections, 9,673 rows, 32,055 fields, 2,356 NULL; 11g 97, 184, 3,445, 13,371, 2,324; 11g-cp1251 27, 52, 28,376, 198,479, 15,404;
  TOTAL 229, 436, 41,494, 243,905, 20,084, empty 27 + 20, `problems 0`. `rowlen_stats.py`: 450 parts (214 / 184 / 52), LOB 126 (80 + 46), `long_conv_fa` 16.
  `num_decode.py` step 3: 86,236 NUMBER fields. `collist.py`: 436 lines, 2,075 definitions, `LONG` 24, `LONG RAW` 10, `TIMESTAMP (n) WITH LOCAL TIME ZONE` 16, moved sections 12.
- Offsets (`rowlen_stats.py` listing, used by the patched copies): `long4_exp_18c.dmp` 0x2271 (`f1 ff`), 0x12272, 0x22277 (`fa ff`, n at 0x22279, z at 0x2227b), 0x32285, 0x42295, 0x522a6, 0x622d8; `long_exp_18c.dmp` 0xa0ea.
- Derived by python over the golden files: SHA256SUMS 360 / 263; 170 sections listed in `truth_counts.txt`; 34 parts with a LONG column; 429 tables compared with a log line;
  94 direct files, 166 tables, 16,602 rows of their conventional partners.

### Mutations (each alone, restored after each; `git status` the same)
| | mutation | fails |
|---|---|---|
| M1 | `fa ff` arm does not consume the `00 00` | `t_long4_equals_the_truth_in_all_four_files`, `long_stats_and_no_warning_in_the_new_long_files` |
| M2 | n range check removed | `the_four_byte_length_is_checked_at_n_and_at_z` |
| M3 | 0xfff2 to 0xfff4 without the warning | `a_length_of_0xfff2_is_read_as_one_field_with_a_warning`, `a_length_of_0xfff2_to_0xfff4_is_written_labelled_...` |
| M4 | 0xfffb read as a length | `a_field_starting_fb_ff_or_fc_ff_stops_the_file_by_name`, `a_long_raw_field_starting_fb_ff_stops_...` |
| M5 | LONG RAW stop from 0x8000 restored | `t_longraw4_equals_the_truth_in_both_18c_files`, `t_frame_of_the_new_long_files_...`, `long_stats_...`, `t3c d_...` (7 in all) |
| M6 | `CHUNKED_MIN_TOTAL` 65,535 | `long_stats_and_no_warning_in_the_new_long_files`, `a_chunked_total_below_65534_bytes_is_read_with_a_warning` |
| M7 | TSLTZ event for 7 bytes restored | `the_tsltz2_descriptors_types_and_warnings`, `t_tsltz2_equals_the_truth_in_all_four_files` |
| M8 | no minimum in `dump.rs` `part_rows` | `a_small_buffer_is_raised_for_the_row_cursor` |
| M9 | `gen_exp_raw.py` form (b) as a plain field, `exp_raw.tsv` regenerated | `t2_manifest::x_walker_with_hashing_reproduces_every_line_of_the_manifest` |
| M10 | LONG RAW threshold 32,000 | `reader::tests::the_w_long_length_thresholds_and_texts_by_type`, `long_stats_and_no_warning_in_the_new_long_files` |
| M11 | band includes 65,520 | `long_stats_and_no_warning_in_the_new_long_files` (longraw4), `t3a x_every_binary_part_...` (the `walk_parts.tsv` band field) |
| M12 | generator draws from 65,523 | `the_generator_draws_large_long_sizes_outside_the_fff2_gap_and_only_in_the_fixtures_combination` |

### Checks run (owner's tree, `CARGO_BUILD_JOBS=2`)
`cargo fmt --check` ok; `cargo clippy -- -D warnings` and `--all-targets` ok; `cargo nextest run -j 2`: 488 run, 488 passed, 4 skipped (463 before + 25 new); `cargo deny check`: advisories, bans,
licenses, sources ok (one `Zlib` license-not-encountered warning as before); `maturin develop --features test-probes`, `ruff check`, `ruff format --check`, `mypy` clean;
`pytest -rs`: 251 passed, 0 skipped; `sha256sum -c` ok in the three folders. Not run: the 600-second fuzz runs and step 3b (T-19 (m)); CI (the owner's push).

### Deviations and observations
1. **Two more Rust-only `LongStats` fields**, `fff2_to_fff4` and `first_fff2_to_fff4` (the walker's input for the 0xfff2 warning, the same pattern as `chunked_short`
   of ADR-102 item 2); Python has only `sizes_65521_to_65534` as the plan says.
2. **Generator draw rates changed** (`common/synth.rs`): a large LONG / LONG RAW value in the fixtures' combination is drawn with chance 15 (was 6) and the four
   categories (one field, form (b), chunked) with equal weight after the edge values. With the old rates a LONG RAW in form (b) needed about 8,000 seeds (41 s)
   for the coverage test to see three of each; now 4,000 seeds (about 20 s). The generator is deterministic per seed, so the test is stable; the count of
   random specs with a large value rose, the other tests are unchanged.
3. **`items.rs` `walk` still opens its stream with `options.buffer_bytes` as given**: ADR-102 item 3 / the plan name `dump.rs` and `reader.rs` only. The scan path hashes
   and skips in 64-byte steps, so a small buffer works there (the 4096-byte tests scan first).
4. `long3_chunked_short_copy` now cuts two bytes (65,533 total), the old cut of one byte gives 65,534, which no longer warns.
5. A form (b) value is delivered through the sink's plain `value`, so on the Python side its offset in a `E-LOB-TOO-LARGE` is field + 6 (0x3228b for `long4_exp_<v>` row 4), as the plan says.
6. `Event::Unverified` and `W-UNVERIFIED-TYPE` stay for the ROWID prefix only.
7. The docstring of `gen_walk_expected.py` still says `uv run`; I ran it with `python3 -I` from `docs/formats/tools` (no `uv run`, rule 9).

### For the next tasks
- T-19 (m): the core changed in `rows.rs` (one more branch per field of a conventional LONG column and `count_value`); step 3b measures T-19 and T-19b together.
- Adding fixtures again means `common/mod.rs` `TOTALS`, the golden generators (all four run from `docs/formats/tools` with `python3 -I`), `t10_truth.rs` `DELEGATED` / `FAMILIES`,
  `t2_manifest.rs` (SHA256SUMS counts, the `truth_counts.txt` section count), `t3_catalog.rs` (tables with a log line), `t8_direct.rs` (pairs, tables, rows), `t5_types.rs` (`collist.py` forms).

## T-19 (m): step 3b on T-19b's commit `1bb7785`, and the fuzz runs (2026-10-25)

Result in one line: **no regression beyond rule 3** (one "watch": `T_BIG` scan, -4.8 % to -6.8 %); walk and read fuzz runs 600 s each with 0 findings; **the `values` fuzz target does not compile**
(needs a source change in `fuzz/`, not done: the owner's task allowed the note only). Only this note changed (`git status` shows only `docs/design/implementer-notes.md`).

### Conditions
- Machine alone: `ora11` and `ora18` stopped (`docker ps` empty), tree clean at `1bb7785`, `CARGO_BUILD_JOBS=2`, `TMPDIR=/var/tmp/oradmp-tmp`, release profile and toolchain as in `benchmarks.md` section 2.
- **`ORADMP_WORK` was empty in my shell** (the owner said it pointed to `corpus/work/18c`). `t15_*` look for `ORADMP_WORK/18c/<file>`, so I set it to the parent, `<repository>/corpus/work`. The first throughput run with it empty skipped all three files ("absent") and measured nothing; its result is not used.
- 1-minute load (before / after): micro-benchmarks 0.69 / 1.08 (started above 0.5, because the shell had just been busy; the figures are all at or near baseline, so I did not repeat); throughput run 1 0.39 / 0.48, run 2 0.41 / 0.46, run 3 0.46 / 0.54; memory run 1 0.39 / 0.44, run 2 0.44 / 0.56, run 3 0.44 / 0.48; fuzz `walk` 0.48 / 1.11, `read` 0.40 / 1.24, `values` 0.41 / 0.41 (did not run). Each run waited for load below 0.5 (except the micro-benchmarks).

### (b) Micro-benchmarks (criterion mean; baseline 2 = `benchmarks.md` 4.1 "baseline" column; rule 3: more than 20 % slower is a regression, 10 to 20 % "watch")
| benchmark | T-19b | baseline | diff | verdict |
|---|---|---|---|---|
| `number/OraNumber::decode` | 1.5231 µs | 1.4645 µs | +4.0 % | ok |
| `datetime/decode_date` | 146.15 ns | 145.18 ns | +0.7 % | ok |
| `datetime/decode_timestamp` | 253.96 ns | 262.61 ns | -3.3 % | ok |
| `text_cp1251/Charset::decode (Win1251)` | 173.21 µs | 173.67 µs | -0.3 % | ok |
| `read_table/R_STAT` | 968.92 µs | 1.0034 ms | -3.4 % | ok |
| `read_table/T_NUM` | 37.693 µs | 37.911 µs | -0.6 % | ok |
| `read_table/T_TEXT` | 38.333 µs | 39.685 µs | -3.4 % | ok |

### (b) Throughput (`t15_throughput`, median of 3 per run; three runs; baseline 2 = `benchmarks.md` 4.2; `T_ROWS`: 20 % / watch 10 %; `T_BIG`: regression above 10 %, watch 5 to 10 %)
| quantity | run 1 | run 2 | run 3 | baseline | worst diff | verdict |
|---|---|---|---|---|---|---|
| `T_ROWS` conv. scan (MB/s) | 811.5 | 785.5 | 816.2 | 809.1 | -2.9 % | ok |
| `T_ROWS` conv. read (rows/s) | 2,246,497 | 2,257,920 | 2,365,039 | 2.17 M | +3.5 % to +9 % (faster) | ok |
| `T_ROWS` direct scan (MB/s) | 802.5 | 831.4 | 878.2 | 886.7 | -9.5 % (run 1; run 3 -1.0 %) | ok (below the 10 % "watch" edge; the 11 to 12 ms scan is the noisy one, 4.2) |
| `T_ROWS` direct read (rows/s) | 2,385,172 | 2,460,220 | 2,460,214 | 2.45 M | -2.7 % | ok |
| `T_BIG` scan (MB/s) | 726.1 (593 to 669 ms) | 736.7 | 741.6 | 779.0 | -6.8 % / -5.4 % / -4.8 % | **watch** (5 to 10 %; runs 2 and 3 are just under 5.5 %), repeated in 3 runs, not a regression (limit 10 %) |
| `T_BIG` read (rows/s) | 2,168,615 | 2,190,051 | 2,183,283 | 2,195,749 | -1.2 % | ok |

(`T_BIG` scan range -4.8 % to -6.8 %.) `T_BIG` full read, the figure that exercises the changed row loop, is within 1.2 % of baseline. The scan is the
slower by about 5 to 7 % in all three runs; nothing in the data says whether the cause is the T-19 / T-19b branch per field of a LONG column (the scan walks the same row loop) or the machine today
(the `T_BIG` scan in baseline 2 spread 566.8 to 600.8 ms per sample; today 591.7 to 669.2 ms). Unknown; I did not bisect (not in the task). For the Architect: a "watch" under rule 3, no stop.

### (b) Memory (`t15_memory`, three runs, ratio limit 1.10; rule 4 limits: `T_ROWS` conv. 9,108 kB, direct 9,021 kB, `T_BIG` 8,984 kB)
| file | stop / end kB, ratio (run 1; run 2; run 3) | highest end peak | verdict |
|---|---|---|---|
| `T_ROWS` conv. | 7,536 / 7,812, 1.0366; 7,516 / 7,864, 1.0463; 7,492 / 7,840, 1.0464 | 7,864 (baseline 7,920) | ok |
| `T_ROWS` direct | 7,460 / 7,704, 1.0327; 7,492 / 7,832, 1.0454; 7,488 / 7,772, 1.0379 | 7,832 (baseline 7,844) | ok |
| `T_BIG` | 7,416 / 7,544, 1.0173; 7,528 / 7,600, 1.0096; 7,440 / 7,532, 1.0124 | 7,600 (baseline 7,812) | ok |
`tools/stream_memory.py` not run (not in the task; `benchmarks.md` 4.3 says baseline 1 stays its reference).

### (c) Fuzz runs (`tools/fuzz_run.sh <target> 600`)
| target | result | runs |
|---|---|---|
| `walk` | exit 0, `fuzz/artifacts/walk/` empty, 0 findings | 797,951 runs in 601 s |
| `read` | exit 0, `fuzz/artifacts/read/` empty, 0 findings | 379,975 runs in 601 s |
| `values` | **not run: build error** (exit 1) | 0 |

**Problem (task T-19b, file `fuzz/fuzz_targets/values.rs` line 51).** T-19b changed `decode_tsltz` to return `Result<Timestamp>` (`crates/oradmp-core/src/values/datetime.rs`, line 271: `decode_timestamp(b)`);
the target still destructures `Some((t, _short))`: `error[E0308]: mismatched types ... expected Timestamp, found (_, _)`. `cargo clippy --all-targets` and `cargo nextest` do not build `fuzz/` (a separate
workspace with the pinned nightly), so no check caught it; CI does not build it either, as far as I know. Proposal: change line 51 to `if let Some(t) = checked(decode_tsltz(field), len) {` and drop `_short`
(one line), then run `tools/fuzz_run.sh values 600` alone. The `values` run is therefore still due; T-19 (m) (d) "0 findings for `walk`, `read`, `values`" is not complete.

### Not done / unknown
- The `values` fuzz run (above). The cause of the `T_BIG` scan "watch" (above). `stream_memory.py`.
- Established: the figures above, measured by me, alone on the machine, on `1bb7785` with the owner's work folder files (their SHA-256 were not re-checked by me; the throughput test printed `T_BIG` 453,304,320 bytes, 5,000,000 rows, as the baseline).

### T-19 (m) addendum (2026-10-26, owner-approved): the `values` fix and its run
- **Fix**: `fuzz/fuzz_targets/values.rs` line 51 is now `if let Some(t) = checked(decode_tsltz(field), len) {` (the only source change; nothing else touched).
- **Build**: `cd fuzz && cargo fuzz build` (the build `fuzz_run.sh` does for each target) compiles all targets: `Finished release profile`, exit 0.
- **Run**: `tools/fuzz_run.sh values 600`, machine alone, load 0.06 before / 1.00 after: exit 0, `fuzz/artifacts/values/` empty, 0 findings, **77,435,828 runs in 601 s**. Together with `walk` (797,951) and `read` (379,975)
  above, T-19 (m) (c) is complete: 0 findings in all three. (The `values` target is far cheaper per run than `walk` and `read`, hence the count.)
- **CI**: `.github/workflows/ci.yml` builds `fuzz/` only in the `extended` job, which is `workflow_dispatch` only (fuzz seeds, `tools/fuzz_run.sh read|walk|values 60`, and `cd fuzz && cargo clippy -- -D warnings`). The push
  job builds nothing in `fuzz/`, so a core API change that breaks a target is found only when someone starts `extended`. I did not change CI. **Proposal for the Architect**: add a cheap `cd fuzz && cargo fuzz build`
  (or `cargo check`) step to the push job, or require an `extended` run before each release (checklist). Unknown: whether the pinned fuzz nightly with `cargo-fuzz` is cheap enough for the push job; I did not time a cold build.
- Not run: `cd fuzz && cargo clippy -- -D warnings` for the edited line (it is a plain pattern simplification); cargo fmt/clippy/nextest of the root workspace (no change there).

### Answers (Architect, session 26; ADR-105 to ADR-107)
- **T-19b and T-19 (m) are done** (`1bb7785`, `0520cd5`, archive commits in S's tree); their counts equal the first
  public CI run of S (nextest 488 + 4 skipped, pytest 251). T-19b deviations 1 to 7 accepted as written; deviation 3
  (`items.rs` `walk` keeps `buffer_bytes`) is correct, the scan path needs no minimum. Deviation 7: the `uv run` in
  `gen_walk_expected.py`'s docstring may be corrected in a later task that touches that generator; not in T-20.
- **The fuzz build** (addendum proposal): both, in a cheap form (ADR-106 item 1): a `fuzz-check` push job running `cd
  fuzz && cargo check --locked` with the pinned nightly (T-20 (f)), and the `extended` dispatch on each release
  candidate stays a release step.
- **The `T_BIG` scan watch**: no action now; Reviewer 6c repeats it on C; a same-sitting run of `460aef5` before any
  bisect if it shows again (ADR-106 item 2).
- **`ORADMP_WORK`**: the tests read `ORADMP_WORK/18c/<file>`, so the variable names the parent folder (`corpus/work`),
  as you set it; the T-19 (m) "Needs" line said "the folder that holds `t_big_exp_18c.dmp`", which was wrong by one
  level. Reviewer 6c's step 3b uses the parent.

## T-20 The a4 version commit (Implementer, session 26)

- **Version**: `0.1.0-alpha.4` in `Cargo.toml`, `Cargo.lock` (2 lines), `fuzz/Cargo.lock` (1 line); README `0.1.0a4`. The grep of
  plan (a) is empty outside `docs`, `corpus`, `HISTORY.md`.
- **`[project.urls]`**: `Source`, `Issues`. `check_dist.py` `check_project_urls` (called from `check_metadata`): the set of
  (label, URL) in `Project-URL` equals `pyproject.toml`'s; five new cases `test_t20_*` in `tests/test_check_dist.py`.
  Mutation (rule compares labels only): `test_t20_a_changed_project_url_fails_with_two_lines` fails (so do the missing and
  extra cases, with my crude edit); restored. The local release build's wheel and sdist both carry the two lines.
- **`fuzz-check`** (shown on the VM before use): `cd fuzz && cargo check --locked` on the clean tree, `fuzz/target` removed:
  23.1 s cold (this VM, `CARGO_BUILD_JOBS=2`), 0.05 s warm. With `values.rs` line 51 as `if let Some(t) = decode_tsltz(field)`
  (T-19b's break): `error[E0308]: mismatched types`, exit 101 (0.14 s warm). File restored with `git checkout`; `git diff` of it
  empty. So `cargo check` is enough; `cargo fuzz build` is not needed.
- **Deviation (ci.yml)**: the `fuzz-check` check step starts with `set -o pipefail`: a step without `shell:` runs `bash -e`
  without pipefail, so `cargo check | tee` would hide a failure. Shown: the step body with the mutation exits 101. The other
  jobs' `| tee` steps have the same weakness where nothing after the pipe re-checks (`extended`'s fuzz/clippy steps, `checks`'
  `deny`); not changed here (plan (g)); for the Architect.
- **Plan (c)**: README "Limits" and "What it reads" read against the reader: no line false; nothing removed or added.
  "From source" is the clone line only. Rust 1.99 / `rust-version = "1.99"` / maturin via `[build-system]` agree.
- **Release-build check**: the script builds committed trees only, so it ran in a scratch clone with the working-tree diff
  committed there (deleted after); `check_dist: ok`.

### Answers (Architect, session 27)

- **T-20 is DONE** (`e1ed10b`, pushed with `4aa33e6`; ADR-108 item 1). Your VM figures agree with the run: nextest
  488 + 4, pytest 256, the five `test_t20_*` cases, `fuzz-check` 26.39 s cold on the runner against your 23.1 s.
- **The `set -o pipefail` deviation: accepted, and you were right about the rest.** `ci.yml` has no `defaults:` block
  and no step sets `shell:`, so every `run` step runs `bash -e {0}` (GitHub's workflow syntax, default shell table)
  without `pipefail`. Hidden today: `checks`' `cargo deny` (completely), nextest and pytest test failures (only the
  skip checks after them can fail the step); `wheel`'s installed-wheel `pytest`; `extended`'s seeds, the three fuzz
  targets (a finding exits 1 in `fuzz_run.sh`, the step stays green and the crash upload never runs), the property
  runs, fuzz clippy and bench; `manylinux2014`'s `auditwheel show`. The release build step is guarded up to
  `check_dist`. Full table: ADR-108 item 4. No pasted run so far hid a failure (the summaries give the counts).
- **Fix: T-20b** (plan): a workflow-level `defaults: run: shell: bash` (`bash --noprofile --norc -eo pipefail {0}`),
  your per-step line removed, two constructs adjusted for `pipefail` (`probes=` gets `|| true`; the `find | grep -q`
  test becomes `find -print -quit` inside `[ -n ... ]`), a VM proof and a proof run on a throwaway branch. Its commit is
  the a4 candidate C; T-20's hashes stay as T-20's evidence only.
- **Release-build check in a scratch clone with the diff committed there**: right way to do it; keep it for T-20b if
  you build at all (T-20b needs no build: `ci.yml` only).

## T-20b `pipefail` for every CI step (Implementer, session 27)

- **Fix**: `defaults: run: shell: bash` after `concurrency:`; header comment line; no step has `shell:`; `fuzz-check`'s
  `set -o pipefail` line removed. `ci.yml` matched ADR-108 item 4 (every step and pipe listed was there, none else).
- **Rewritten**: (1) `run_one`'s `probes=` ends `... | awk ... || true)`: without it a log with no probe skip makes the
  first `grep` fail the pipeline and `set -e` stops the function silently; now `awk` prints 0 and the "expected exactly 3"
  message appears. (2) property step: `if [ -n "$(find . -name '*.proptest-regressions' -print -quit)" ]; then`.
- **Every other pipeline under `pipefail`**: `tee` pipes of nextest, deny, pytest, release build, venv `python -c`, venv
  pytest, seeds, fuzz targets, both property runs, fuzz clippy, bench, `basename`, `auditwheel`, `sha256sum`: ok (now fail
  the step when the left side fails, intended). `awk ... | sort > file`: ok. `unzip -p | grep '^Tag:' | tee`: ok (no
  `-q`, grep reads to the end). `skipped="$(grep | grep -v | grep -v || true)"`: ok. Summary steps: ok (inside `$( )` or
  `|| true`).
- **VM proof**: PyYAML parse (scratch script, system python3 has PyYAML 6.0.2; `.venv` has none), `bash -n` of the 37
  `run` blocks: ok. `bash --noprofile --norc -eo pipefail -c '{ true; exit 7; } | tee /dev/null; echo reached'`: exit 7, no
  `reached`; with `bash -e -c`: `reached`, exit 0. Construct (1): three probe skips gives `3`; none prints the message,
  exit 1 (the old line under pipefail exits 1 silently). Construct (2): tree without file takes the clean branch, with
  `x.proptest-regressions` the regression branch. Fixture checksums ok; fmt ok; nextest 488 passed + 4 skipped; pytest 256.
- **Proof patch** (one run, plan (f)): prepared outside the tree for the owner; ids `nextest deny pytest` / `build venvs` /
  `seeds fread fwalk fvalues props fclippy bench` / `audit`.

### Answers (Architect, session 28)

- **T-20b is DONE** (`4b9883c` = C, the a4 candidate; ADR-109 item 2). The push run on C kept `e1ed10b`'s counts; the
  proof run (40974068275, your patch) turned `checks`, `wheel`, `extended` and `manylinux2014` red with exactly 3, 2, 7
  and 1 exit-code-7 annotations, one per id you listed, `fuzz-check` stayed green and the crash-artifact step ran.
  Reviewer 6c repeated your shell facts and both rewritten constructs and found no failure left that turns a red check
  green. Your pipeline list matched ADR-108 item 4.
- **What 6c found that your list did not cover**: five `echo "...$(...)" >> "$GITHUB_ENV"` lines (57, 178, 200, 318, 363
  at C), where `-e` and `pipefail` do not see the substitution's status (SF-1), and the `find` error inside `[ -n
  "$(find ...)" ]` (N-1, the construct ADR-108 prescribed). Neither blocks a4 (ADR-109 item 3). Rule from now on: a
  substitution whose status matters stands alone as an assignment, then its value is used. Applied in **T-21**.
- **Your PyYAML from the system `python3`** (the `.venv` has none): fine for a scratch check; T-21 does the same.
- **T-19 (m) note, a correction by the Architect** (6c N-2): its result line's "-5.0 % to -6.8 %" reads **-4.8 % to
  -6.8 %** (726.1 / 736.7 / 741.6 MB/s against 779.0); the table and the plan were right. Corrected in the note's
  result line by T-21 (`20bbc80`); session 30 replaced this answer's earlier "the note stays as written".

## T-21 (no note committed; Architect, session 30)

The T-21 commit `20bbc80` holds no note of its own (its only change here is the T-19 (m) figure); the figures below are
the Implementer's report as the owner pasted it in session 30, recorded in the plan's T-21 "Session 30 record".

### Answers (Architect, session 30; ADR-114, ADR-115)

- **T-21 is DONE** (`20bbc80`): push run and a manual run green; nextest 488 + 4, pytest 262, the venvs 259 + 3,
  252 + 4, 259 + 3; `extended` 0 findings; `manylinux2014` with both tags. Reviewed in Reviewer 7 with T-16c and T-16b.
- **(i)**: done by the owner's assistant against the `wheel` listing of the run on C, not against the local a4 folder;
  accepted, because C's CI files are byte-identical to the released ones (a4 record step 6). Only the SBOM and `RECORD`
  differ; `_core.abi3.so` is `d0efa2df...`. Their sizes were not stated; Reviewer 7 reads them.
- **(k)**: passed (sdist `510f2e39...` from PyPI, built in 2 min 11 s, `oradmp 0.1.0a4`, `survey` exit 0). Risk 32 closed.
- **Six pin cases instead of five**: accepted; six tools are pinned, the plan miscounted.
- **Eight more test files with comment edits** (beyond (e)'s list): accepted; (b)'s `git grep` needed them, and no
  assertion changed (diff read in session 30).
- **No note committed**: accepted this once. From T-16c on the task's note is part of its commit (T-16c (g)).

## T-16c note (Implementer, 2026-10-31): `manylinux2014` acceptance (ADR-115; plan "Session 30" block)

**Verdict: T-16c: proposed accepted; verdict for the Architect to confirm** (figures in "Results" at the end of this note; was: pending). Check (0) passed (below), the build and every local check pass; the verdict "accepted" or "rejected"
needs the dispatch of the `manylinux2014` job (items (b) and (c)); the owner pastes the summaries and the Implementer
adds the figures here (a second docs-only commit, plan (g)).

### (0) The pip check (no CI, no build; scratch venv `uv venv --seed`, pip 26.2.2; the scratch folders are deleted)

Command, for `<v>` in 3.11, 3.13, 3.14 and `<p>` in `"pyarrow>=15"`, `numpy`, `pandas`:
`<venv>/bin/python -m pip download --no-deps --only-binary :all: --platform manylinux2014_x86_64 --python-version <v> -d <scratch> "<p>"`

| `--python-version` | pyarrow | numpy | pandas |
|---|---|---|---|
| 3.11 | `pyarrow-20.0.0-cp311-cp311-manylinux_2_17_x86_64.manylinux2014_x86_64.whl` | `numpy-2.2.6-cp311-...` | `pandas-2.3.2-cp311-...` |
| 3.13 | `pyarrow-20.0.0-cp313-cp313-manylinux_2_17_x86_64.manylinux2014_x86_64.whl` | `numpy-2.2.6-cp313-...` | `pandas-2.3.2-cp313-...` |
| 3.14 | "No matching distribution found for pyarrow>=15 (from versions: none)" | none (same message) | none (same message) |

Equal to the expectation of the plan (cp311 20.0.0 / 2.2.6 / 2.3.2; cp313 pyarrow 20.0.0; cp314 none). pyarrow 15 or
later exists for cp311: not rejected. The 3.14 gap limits the claim (plan (e)). The claim a5's README would make if the
dispatch accepts: "glibc 2.17 or later; below glibc 2.28, CPython 3.11 to 3.13 and pyarrow up to 20.0.0".

### What was built (files)

- `tools/release_build.sh`: `--manylinux 2014|2_28` (default `2_28`, the maturin line is unchanged for it through
  `--compatibility "$compat"`); a bad or missing value exits 2 at option parsing, before the root rules, before "no such
  commit", before any folder; `check()` fails unless the wheel name ends `-cp311-abi3-<tag>.whl` (`manylinux_2_28_x86_64`
  resp. `manylinux_2_17_x86_64.manylinux2014_x86_64`); prints `manylinux: <value>`; `--help` (lines 2 to 26 of the header) names it.
  The option loop is now a `while`/`shift` loop (the option takes a value).
- `tests/test_release_build.py`: four cases (bad value exits 2 naming it, five values incl. empty and a value with a
  blank; missing value exits 2; good values pass the option checks and reach "no such commit"; `--help` names the
  option). Mutation: the value check `2014 | 2_28) ;;` changed to `*) ;;`: `test_manylinux_bad_value_exits_2_naming_it_before_anything_else`
  fails, the others pass; restored.
- `tools/ci_install_check.sh` (new; `run_one` of the `wheel` job generalised for pip or uv, `--only-binary :all:`, numpy
  version in the info line, root-only skips allowed when uid 0, exactly 3 each), `tools/ci_container_2_17.sh`,
  `tools/ci_container_rocky8.sh` (new; they run inside the containers). The `wheel` job is **not** changed (its
  `run_one` stays inline; the plan allowed but did not require moving it).
- `.github/workflows/ci.yml`: the `manylinux2014` job rewritten (still `workflow_dispatch` only, no upload, no artifact,
  the wheel is not kept). Digests read from the registries' HTTP API on 2026-10-31 (no `docker pull`; ADR-115 item 2
  says no agent runs Docker on the VM): `quay.io/pypa/manylinux2014_x86_64@sha256:5f0bcf92ed65391c7df442763168887d97ae734f2fbb17012eb6bed4a8aef98f`
  (single-platform manifest), `rockylinux:8@sha256:9794037624aaa6212aeada1d28861ef5e0a935adaf93e4ef79837119f2a2d04c`
  (image index; docker picks amd64). The containers were **not** run by me: their steps are untested until the dispatch.

### Checks on the VM (alone on the machine)

- Fixture checksums ok (3); `cargo fmt --check` ok; clippy twice ok; nextest 488 passed + 4 skipped; `cargo deny check`
  advisories, bans, licenses, sources ok; `maturin develop --features test-probes`, ruff, ruff format, mypy ok; pytest
  **266 passed** (262 + 4 new); YAML parse of `ci.yml` ok and `bash --noprofile --norc -eo pipefail -n` on all 41 `run`
  blocks ok; `bash -n` on every `tools/*.sh` ok.
- Default build unchanged: old script (a clone of `f8f0987`) and new script, commit `f8f0987`, same root
  `/var/tmp/oradmp-t16c-build`: wheel `e97742b2...26aa` in both and sdist `c7d4456d...7f54` in both (identical
  `SHA256SUMS`), `check_dist: ok`, the new one prints `manylinux: 2_28`.
- `ORADMP_BUILD_ROOT=/var/tmp/oradmp-t16c-build tools/release_build.sh HEAD --manylinux 2014`: `check_dist: ok`,
  `manylinux: 2014`, wheel `oradmp-0.1.0a4-cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.whl`
  (`08eb5fd3...695b`), same sdist `c7d4456d...`. `auditwheel show` (6.8.2): "consistent with ... manylinux_2_17_x86_64",
  symbols up to GLIBC_2.14; `Tag:` lines `cp311-abi3-manylinux_2_17_x86_64` and `cp311-abi3-manylinux2014_x86_64`.
  The CI greps were run against these outputs and pass. All scratch folders were deleted.
- `tools/ci_install_check.sh` on the VM host with that wheel (pip, Python 3.13.5, not root): 263 passed + 3 skipped
  (the probe tests); without pandas and with the extra file allowed 256 passed + 4 skipped. Mutations: no extra allowed
  -> "unexpected skips", exit 1; wrong `VERSION` -> exit 1; unreachable pytest pin -> exit 1; pytest replaced by `false`
  in front of `| tee` -> exit 1; the `ldd` substitution of the 2.17 container script replaced by `$(false)` -> exit 1.
  The root-only branch (`id -u` 0) and both containers are not run here.

### Risks the first dispatch may show (not guessed away)

The 2.17 image may lack `git` (then `git --version` fails and the step says so); the root-only skip count (3, as a4 step
9) is exact; `uv python install 3.14` or `dnf install python3.11 python3.11-pip` may fail in the Rocky container (the plan
then asks for the VM fallback); the `auditwheel show` text is matched by `manylinux_2_17|manylinux2014` (confirmed on the
VM). The 2.17 venvs (ii) and (iii) install `pyarrow pandas` / `pyarrow` unpinned, so pip picks the newest wheel for
glibc 2.17 (20.0.0, pandas 2.3.2 expected; printed in the summary).

T-16c fix (Implementer, 2026-10-31), found by the first public run of the `wheel` job (a8da2cc): `tests/test_release_build.py::test_manylinux_good_values_get_past_the_option_checks` failed there with "the build root /var/tmp/oradmp-release exists already" instead of "no such commit".
Cause: the job's release build had just created the canonical root, and `release_build.sh` checks the root (line 114) before the commit (line 116), as in a4; the test assumed the root was absent.
Fix (test only, script unchanged): the `--release` call now expects exit 1 and "exists already" if the canonical root exists, else "no such commit"; no skip. Verified with the empty root present and absent, and by mutating each string.

### Results (Implementer, 2026-10-31; docs only; the owner pasted the summaries, I ran no CI and no build)

History of two commits: `a8da2cc` (T-16c) failed in the `wheel` job because `tests/test_release_build.py` assumed the canonical build root was absent. `94b6368` fixed the test; the script was not changed, and its root-before-commit check is a4 behaviour.

Push run on `94b6368`:
- `checks`: nextest 488 passed, 4 skipped; cargo deny ok; pytest 266 passed, 0 skips.
- `fuzz-check`: `fuzz/` compiles with `--locked` on the pinned nightly.
- `wheel`: root canonical, `check_dist` ok. Venvs: py311 pa15 pd223 263 passed + 3 skipped; py311 pyarrow-only 256 + 4; py3.15 latest 263 + 3. The skips are the 3 probe skips, plus the pandas skip in the pyarrow-only venv. Wheel `df3be6326d18371d4c4163124961cda12228ebb167d979f10335cfac627afd56`, sdist `0e2dd8866e5fb2b83bdad54a51559c5a5e5ed753461d7dadfef55208c91f3755`. `_core.abi3.so` `d0efa2dff42b57f5203429ace3b7789d4d1cfbbb7fd09ce986b5badf51b4789c`, equal to a4's. Members: 16 in the wheel, 53 in the sdist, 69 in total.

Dispatch `manylinux2014` on `94b6368` (dispatch 1 of at most 3), green:
- Wheel `oradmp-0.1.0a4-cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.whl` `4327e1a72817fdb7f807da800cf2b32eddc4f41d6e9fc3418b84b1494c7c5e44`; sdist the same as in the `wheel` job. `_core.abi3.so` `aa6ef3ac7593febc23cc9384eb535f5c9a9469975a5809b5be77a691fa666255`, so it differs from the manylinux_2_28 build.
- Images: `quay.io/pypa/manylinux2014_x86_64@sha256:5f0bcf92ed65391c7df442763168887d97ae734f2fbb17012eb6bed4a8aef98f` (glibc 2.17), `rockylinux:8@sha256:9794037624aaa6212aeada1d28861ef5e0a935adaf93e4ef79837119f2a2d04c` (glibc 2.28). These equal the digests recorded above.
- auditwheel 6.8.2: platform tag `manylinux_2_17_x86_64`; highest symbol `GLIBC_2.14`.
- Glibc 2.17 venvs: py311 newest 260 + 6, py311 pa15 pd223 260 + 6, py313 pyarrow-only 253 + 7. Rocky 8: py311 pa15 pd223 260 + 6, py311 pyarrow-only 253 + 7, py314 newest 260 + 6. Runner venvs 263 + 3, 256 + 4, 263 + 3. Expected skips as stated in this note (3 probes, 3 root-only, pandas where absent).
- Python 3.14 on glibc 2.17: pyarrow cannot be installed (no cp314 wheel for that glibc). The job reports this as expected. It is a limit of pyarrow, not of oradmp.
- Duration of the `manylinux2014` job: 1141 s to the summary step.

Correction for the Architect: the member count of the a4-built artefacts is 69 (16 wheel + 53 sdist), not 68. Place found with "68" in this context: `docs/prompts/architect-30-update.md` line 14 ("all 68 members of the wheel and the sdist"). I searched `docs/` for member/file counts of 68 and found no other place; I changed nothing there. (`docs/design/implementer-notes.md` line 1412 says "50 members" for an older build; that is another build, not a correction.)

Open for the Architect (ADR-113 item 4): whether a5 ships a second wheel tagged `manylinux_2_17` next to the `manylinux_2_28` one. Facts: it is a separate wheel; its `_core.abi3.so` hash differs from the `manylinux_2_28` one; it was tested on glibc 2.17 and on glibc 2.28. No recommendation.

Differences between the brief figures and the repository: none found (names and tags agree with the notes above; the digests equal those in "What was built").

### Answers (Architect, session 31; ADR-116 to ADR-119)

- **T-16c is DONE, verdict "accepted"** (ADR-116 item 1). Your proposed verdict is confirmed; the figures above are the
  record. The plan's T-16c heading and its "Session 31 record" cite them.
- **Held for Reviewer 7, not for you** (ADR-116 item 2). These read the public summaries:
  - the member-by-member comparison of (c) with T-21's run, with the SBOM and `RECORD` sizes;
  - the pyarrow, numpy and pandas versions of the six container venvs.

  Next time, copy such lines into the note when the plan asks for them.
- **The `94b6368` fix**: right. The script's order, root first and commit second, stays. The class of defect now has
  a rule that binds from T-16b on (ADR-119 item 2; plan, "Reviewer 7" block). A test that depends on machine state
  must make that state itself, assert every state it can meet, or skip with an allowed reason. It never touches a
  fixed shared path.
- **69 members**: accepted. The plan's T-21 record now says 69. The owner's prompt file is his, and you were right to
  leave it alone.
- **The second wheel (ADR-113 item 4)**: decided **yes**. a5 ships `manylinux_2_17` next to `manylinux_2_28`
  (ADR-117). Before that, T-16b (i) asks you for one build of `94b6368 --twice --release --manylinux 2014` at the
  canonical root on the VM: both builds identical, the wheel `4327e1a7...`, the sdist `0e2dd886...`. Then move the
  root aside and delete it.
- **Next**: T-16b (plan, "Session 31" block). It is a test task; nothing of it is published. Whether its wheels ever
  ship is the owner's Q-15.

## T-16b note (Implementer, 2026-10-31): other platforms, the `platforms` job (ADR-118; plan "Session 31" block)

Verdicts per platform (ADR-118 item 1): **pending: first dispatch**. Nothing was run on macOS or Windows; this session
could build and read, not dispatch.

### (0a) Runner images (read on 2026-10-31 from the `actions/runner-images` README and the GitHub docs reusable "supported-github-runners", the table "Standard runners for public repositories")

| Platform | Label used | Image README |
|---|---|---|
| macOS arm64 | `macos-15` | macos-15-arm64 |
| macOS x86_64 | `macos-15-intel` | macos-15 (x64; also `macos-26-intel`, `macos-15-large`) |
| Windows x86_64 | `windows-2025` | Windows2025 |

macOS x86_64 **is** listed as a standard runner for public repositories (`macos-15-intel`, `macos-26-intel`); `macos-13`
is not in the list any more. `macos-latest` is now macOS 26 arm64, so no label with `-latest` is used. macOS 14 is marked
deprecated. Windows has `windows-2025`, `windows-2025-vs2026` and `windows-2022`; I took `windows-2025` (the one that
`windows-latest` names, without the VS 2026 variant).

### (0b) pyarrow per platform (scratch venv under the session scratch folder, pip 25.1.1; folders deleted)

`pip download --no-deps --only-binary :all: --platform <p> --python-version <v> -d <scratch> <pkg>`. The platform tags are
`macosx_15_0_arm64`, `macosx_15_0_x86_64`, `win_amd64`; every download succeeded, none printed "No matching distribution".

| `<p>` | `<v>` | pyarrow>=15 | numpy | pandas |
|---|---|---|---|---|
| macosx_15_0_arm64 | 3.11 | pyarrow-26.0.0-cp311-cp311-macosx_12_0_arm64 | numpy-2.4.6-cp311-cp311-macosx_14_0_arm64 | pandas-3.1.0-cp311-cp311-macosx_11_0_arm64 |
| macosx_15_0_arm64 | 3.14 | pyarrow-26.0.0-cp314-cp314-macosx_12_0_arm64 | numpy-2.5.4-cp314-cp314-macosx_14_0_arm64 | pandas-3.1.0-cp314-cp314-macosx_11_0_arm64 |
| macosx_15_0_arm64 | 3.15 | pyarrow-26.0.0-cp315-cp315-macosx_12_0_arm64 | numpy-2.5.4-cp315-cp315-macosx_14_0_arm64 | pandas-3.1.0-cp315-cp315-macosx_11_0_arm64 |
| macosx_15_0_x86_64 | 3.11 | pyarrow-26.0.0-cp311-cp311-macosx_12_0_x86_64 | numpy-2.4.6-cp311-cp311-macosx_14_0_x86_64 | pandas-3.1.0-cp311-cp311-macosx_10_9_x86_64 |
| macosx_15_0_x86_64 | 3.14 | pyarrow-26.0.0-cp314-cp314-macosx_12_0_x86_64 | numpy-2.5.4-cp314-cp314-macosx_14_0_x86_64 | pandas-3.1.0-cp314-cp314-macosx_10_15_x86_64 |
| macosx_15_0_x86_64 | 3.15 | pyarrow-26.0.0-cp315-cp315-macosx_12_0_x86_64 | numpy-2.5.4-cp315-cp315-macosx_14_0_x86_64 | pandas-3.1.0-cp315-cp315-macosx_10_15_x86_64 |
| win_amd64 | 3.11 | pyarrow-26.0.0-cp311-cp311-win_amd64 | numpy-2.4.6-cp311-cp311-win_amd64 | pandas-3.1.0-cp311-cp311-win_amd64 |
| win_amd64 | 3.14 | pyarrow-26.0.0-cp314-cp314-win_amd64 | numpy-2.5.4-cp314-cp314-win_amd64 | pandas-3.1.0-cp314-cp314-win_amd64 |
| win_amd64 | 3.15 | pyarrow-26.0.0-cp315-cp315-win_amd64 | numpy-2.5.4-cp315-cp315-win_amd64 | pandas-3.1.0-cp315-cp315-win_amd64 |

`pyarrow==15.0.2` for 3.11: `pyarrow-15.0.2-cp311-cp311-macosx_11_0_arm64`, `...-macosx_10_15_x86_64`, `...-win_amd64`.
`pandas==2.2.3` for 3.11: `pandas-2.2.3-cp311-cp311-macosx_11_0_arm64`, `...-macosx_10_9_x86_64`, `...-win_amd64`.
So no platform is "rejected: no pyarrow wheel" and no venv (i) substitute is needed. (The "newest CPython" of the runner is
whatever `setup-python` `3.x` gives; 3.15 was checked as well, because the Linux wheel job ran on 3.15.)

### What was built (files)

Changed: `.github/workflows/ci.yml` (the new `platforms` job at the end and nothing else; the header comment is unchanged,
the job has its own comment). New: `tools/ci_platform_build.sh`, `tools/ci_platform_venvs.sh`,
`tools/ci_platform_report.py`. Not changed: `tools/release_build.sh`, `tools/ci_install_check.sh`, the two container scripts,
`tests/`, `crates/`, `python/`, `fuzz/`. This note.

The job `platforms` (matrix `macOS arm64` = `macos-15`, `macOS x86_64` = `macos-15-intel`, `Windows x86_64` =
`windows-2025`; `fail-fast: false`; `workflow_dispatch` only; `timeout-minutes: 90`; `permissions: contents: read`;
`persist-credentials: false`; no artifact, no upload, no secret). Steps, in order:

1. (Windows) `git config --global core.autocrlf false` before the checkout, see "Deviations".
2. `actions/checkout` (same SHA as the other jobs).
3. Shell check: `set -o | grep pipefail` and `(false | true)` must fail. Covers ADR-108 on every platform.
4. Environment: `PT` (the runner's temporary folder in a form that bash and native Python both read: `cygpath -m` on
   Windows), `TMPDIR` (and `TEMP`, `TMP` on Windows), `PLAT_ROOT` (macOS `/private/var/tmp/oradmp-platforms`, because
   `RUNNER_TEMP` is below `/Users/runner` and `check_dist` rejects `/Users/`; otherwise `$PT/oradmp-platforms`), job start.
5. `actions/setup-python` (3.11 and 3.x, same SHA as the other jobs).
6. Fixture checksums by `tools/ci_platform_report.py checksums` (Python; no `sha256sum` on macOS).
7. (Windows) line-ending probe: a clone with `core.autocrlf=true`, prints the CR count of one script and whether the three
   fixture checksum lists still verify. Informational; it prints a result either way and does not fail the job.
8. Rust toolchain from `rust-toolchain.toml`; `uv==0.12.21` (pip); `NEWEST_PY`; the expected version.
9. `taiki-e/install-action` `cargo-nextest@0.9.146` (same SHA as `checks`).
10. `cargo nextest run -j 2`: skipped must equal the four named tests, the summary line must say `488 passed, 4 skipped`.
11. `bash tools/ci_platform_build.sh` (`if: !cancelled()`): clone of the commit to `$PLAT_ROOT/src`, venv with
    `maturin==1.15.0`, `maturin build --release --locked` and `maturin sdist` (no `--zig`, no `--compatibility`),
    `RUSTFLAGS` with `--remap-path-prefix` for the cargo home (all spellings on Windows) and the root,
    `SOURCE_DATE_EPOCH` from the commit time, `tests/check_dist.py --build-root $PLAT_ROOT WHEEL SDIST` (not weakened; a
    failure is reported after the second build, then fails the step), then the second build (target folder removed first)
    into `dist2` and `compare`: both SHA-256 values and `identical` or `differs: <members>` for the wheel and for the sdist.
12. A clone of the commit to test from (`$PT/clone`).
13. Three venvs by `tools/ci_platform_venvs.sh` (uv, `--only-binary :all:`, `pytest==9.1.1`): py311 with pyarrow 15.0.2 and
    pandas 2.2.3; the newest Python with the newest pyarrow and pandas; py311 with pyarrow only. Each: location check
    (site-packages), full `pytest -rs`, the skip filter, `oradmp --version`. A failing venv does not stop the other two.
14. Job summary (`if: always()`): runner, image, tool versions, nextest, build, `check_dist` line, wheel name and tags, hashes,
    the two-build comparison, member list, probe, per venv the versions (Python, pyarrow, numpy, pandas), counts, skips
    with reasons, failures, duration.

Allowed skips (plan (b)): probes (exactly 3); `tests/test_consumers.py` in the pyarrow-only venv; off Linux
`tests/test_paths.py` ("POSIX byte file names"); on Windows the reason "POSIX permissions" (the two `tests/test_cli.py`
tests). Anything else fails the venv. Root-only skips are not expected: the script refuses to run as root.

### Checks on the VM

Fixture checksums by the new Python helper ok (171 + 159 + 30 files); `bash -n tools/*.sh` ok; `ruff check`, `ruff format
--check`, `mypy` ok; `pytest` 266 passed; `cargo fmt --check` ok; the workflow parses as YAML, every `run` block passes
`bash -n`, every `uses:` is a full SHA (no new action: the five used are the ones the other jobs use). The helper
`ci_platform_report.py` was run against two fake archive pairs (identical and differing members) and against a wrong
checksum (exit 1). `tools/ci_platform_build.sh` was run once on this Linux VM against `HEAD` (root in the session scratch folder, 4 min 40 s): both builds worked, the wheel and the sdist were byte-identical across the two builds on Linux (wheel `7a3a61de...`), and `check_dist` failed only because my scratch path ended in a folder named `root` (`/root/` home-path rule), a test artefact. `tools/ci_platform_venvs.sh` was run on that wheel in a Python 3.14 pyarrow-only venv (256 passed, 4 skipped = 3 probes + pandas, `oradmp --version` ok) and with a wrong `VERSION` (exit 1). The scratch folders are deleted.

### Deviations and judgement calls (for the Architect)

1. **Prompt vs task text**: the prompt says `tools/check_dist.py`; the file is `tests/check_dist.py` (as in the plan). I used the plan.
2. **Windows `core.autocrlf`**: Git for Windows may check out `.sh` files with CRLF, which bash cannot run, and
   `.gitattributes` (owner's file) has no `*.sh` rule. The job turns `core.autocrlf` off before the checkout on Windows, so the
   scripts and fixtures stay LF; the probe step measures what the default does to the fixtures. Proposal for the owner:
   `*.sh text eol=lf` in `.gitattributes`. If the Architect wants the risk check under the default, remove step 1.
3. **Windows `shell:`**: no step sets `shell:`. `defaults.run.shell: bash` is Git Bash with the same `-eo pipefail` template;
   the Shell check step proves it per platform. No PowerShell step.
4. **Own build script**: `release_build.sh` needs `--zig`, `realpath -m` and `bin/`; it is untouched and `ci_platform_build.sh`
   is a separate script. It has no `--release`/canonical-root rule (the VM's release root is not used).
5. **macOS build root** is `/private/var/tmp/oradmp-platforms`, not under `runner.temp` (task text: "root"), see step 4.
6. **No cache action** in this job (cargo registry and `target` are cold each time); minutes are free on the public repository.
7. The venv script is separate from `ci_install_check.sh` (Windows `Scripts/`, allowed non-Linux skips, no root-only branch).
8. `tests/test_release_build.py` runs `bash tools/release_build.sh` with POSIX paths (`/var/tmp`, `realpath -m`, a symlink
   test): I expect failures on macOS (BSD `realpath` has no `-m`) and on Windows. I did not change any test (prompt). Each
   will be listed after the first dispatch as "defect in a test" if so.

### Could not be checked here

Everything on macOS and Windows: whether `maturin build` and `sdist` succeed natively; whether `check_dist` passes (its
`BUILD_PATH` rule reads `path+file:///...` and `/var/tmp`, `/tmp`; on Windows the SBOM path will start with `/D:/...` or
`D:\\...`, and the rule's prefix is `<root>/src/`, so it **may fail for a platform reason**, which the task text sends to the
Architect before any change); the remaps of the Windows cargo home; the MSYS path conversion for `python -I` and `uv` calls;
`uv` finding Python 3.11 beside the newest one; the 488 passed on Windows; the pytest suite there.

### Dispatch plan

One dispatch of `CI` on the T-16b commit (all dispatch-only jobs run: `extended`, `manylinux2014`, `platforms` with three
legs). It is the risk check (ADR-118 item 4, first of at most three). Expect failures: they are the data. The owner pastes:
the web address, the job summary of every `platforms` leg, and for a red step its log. A second dispatch is needed if the
first one shows a test-only or workflow fix; the plan allows three. Billed minutes: free (public repository); the note is to
record each leg's duration.

### Results

pending: dispatch 2 (dispatch 1 is recorded below; no verdict is proposed here)

### Dispatch 1 (commit `bdc507b`; 1 of at most 3) and the fix (Implementer, 2026-10-31)

Push jobs `checks`, `wheel`, `fuzz-check` green.

macOS arm64 (`macos-15`) and macOS x86_64 (`macos-15-intel`): build, `check_dist` and nextest fine (488 passed, 4 skipped).
Wheels `macosx_11_0_arm64` `ec57e7b70d769ad7ee26fda341d9e951ba941cddb874cdf6625de0c8891aeba3` and `macosx_10_12_x86_64`
`349ceb60e2d783b22a2a9bc5466da4a1290feb560a0bd7ce9a0aecfb6bef89d9`; sdist `9d32534c97ccce10c88dcea2741c0114efe1ef9c13a3098f4ef41da6237db167`
on both. Two builds in one job gave identical wheels and identical sdists on both legs. pytest: exactly 5 failed in every venv,
all in `tests/test_release_build.py`, one cause: `realpath: illegal option -- m` (`tools/release_build.sh` uses GNU
`realpath -m`; macOS `realpath` is BSD; the script is Linux-only by design, ADR-110). The five: the refuses-another-root,
refuses-a-root-that-resolves-elsewhere, accepts-the-canonical-root, without-release-another-root and
manylinux-good-values tests. Skips (3 probes, 8 "POSIX byte file names", pandas where absent) matched the allowed list.

Windows x86_64 (`windows-2025`): nextest 487 passed, 4 skipped. The build step died: `tools/ci_platform_build.sh` line 59 and
line 67, `maturin: command not found`, exit 127; so the hashes, members, two-build and venv sections were empty and no Windows
pytest result exists. The line-ending probe (`core.autocrlf=true`) found 110 CR bytes in `tools/ci_platform_venvs.sh`; the
fixtures stayed unchanged (checksums ok for 18c, 11g, 11g-cp1251).

Causes and changes:
- Windows maturin. maturin 1.15.0 was installed into the venv (`python -m pip install`, so `venv/Scripts/maturin.exe`), and the
  script put `$bin` on PATH with `export PATH="$bin:$PATH"`. On Windows `$bin` is `D:/a/_temp/.../venv/Scripts` (PLAT_ROOT is the
  `cygpath -m` form). Git Bash splits a PATH string at every `:`, so the entry became `D` and `/a/_temp/...`; maturin was not
  found. Read from the code, not run on Windows. Fix in `tools/ci_platform_build.sh`: on Windows the entry is converted with
  `cygpath -u`; on macOS it is unchanged (`$bin`, the same string as before). The export now comes right after the pip install,
  and the script prints `maturin (first line): <path> <version>` at once and exits 1 if `maturin --version` is not exactly
  `maturin 1.15.0`. The pinned version is the same on all legs.
- Summary honesty (`.github/workflows/ci.yml`, `platforms` job, summary step only; ids `clone` and `venvs` added to two steps so
  the summary can name their outcomes). Build, check_dist, venvs print "FAILED ..., see the step log" when nothing was recorded;
  wheel-and-tags, hashes, two-build, member list go through one `section` helper that prints the same words instead of an empty
  block. Checked locally by running the summary script against an empty log folder.
- Linux-only tests. `tests/test_release_build.py` has a module `pytestmark` skipif (not `sys.platform.startswith("linux")`,
  reason "release_build.sh is Linux-only (ADR-110)"). Verified by running that module with `sys.platform` patched to `darwin`:
  9 skipped; on Linux the full suite is 266 passed, 0 skipped.
- Allowed skips (`tools/ci_platform_venvs.sh`): new category "Linux-only release tooling" off Linux; it must be exactly 9, the
  number from `pytest --collect-only -q tests/test_release_build.py` (9 tests collected); another count fails the venv, as does any
  skip not in the list. The "allowed skips" line now shows the category.
- Windows nextest 487 against 488: from the source, the test `x_a_replacement_with_the_same_size_and_time_is_told_by_its_inode`
  (`crates/oradmp-core/tests/t3_catalog.rs`, `#[cfg(unix)]`, uses `std::os::unix::fs::MetadataExt`); `dump.rs` has `file_id` with a
  `cfg(not(unix))` branch returning `None`. Not changed. The summary text and the step's `488 passed` check will therefore fail on
  Windows; the workflow step still says 488 (a Windows count is for the Architect to decide; I did not change it).

Could not be checked here: all Windows behaviour past the build step (the maturin fix itself, `check_dist` on `D:/` paths, remaps,
the three venvs, whether read-only-folder, mode-000 file and byte-file-name tests skip or fail on Windows; those categories are
left as they were); macOS summary rendering of the new text.
For the Architect: `.gitattributes` `*.sh text eol=lf` (110 CR bytes under `core.autocrlf=true`); no change made.
