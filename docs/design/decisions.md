# Legacy EXP reader: decision records

**The project was called `oradump` until ADR-54** (session 13): the records before ADR-54 keep that name; read it as
`oradmp` (crates `oradmp-core`, `oradmp-py`, import `oradmp`, classes `OradmpError` / `OradmpWarning`).

Architect session 1, 2026-10-07; session 2, 2026-10-14; sessions 3 and 4, 2026-10-15; sessions 5 and
6, 2026-10-16; sessions 7 to 13, 2026-10-17; sessions 14 and 15, 2026-10-18; sessions 16 and 17, 2026-10-21; session 18, 2026-10-22; sessions 19 to 23, 2026-10-24; sessions 24 and 25, 2026-10-25; sessions 26 and 27, 2026-10-29; session 28, 2026-10-30; session 29, 2026-10-31. Each record:
context, decision, consequences, rejected. Status is **decided** (the Implementer follows it; reopening
needs a new record) or **needs the project owner** (question in `open-questions.md`). Rule numbers are
those of `docs/formats/exp_reader_rules.md` (v3.4 from session 4 on, v3.5 from session 8 on, v3.7 from session 24 on, **v3.9 from session 25 on**); section
numbers without a file name are those of `exp-reader-architecture.md`.

**Reading these records in the public repository** (session 26, ADR-105): every commit hash cited before session 26
names a commit of the owner's private archive, not of this repository (`HISTORY.md`); the hashes are not rewritten.
Cites of `docs/prompts/...`, `.claude/` and `CLAUDE.md` name files the owner keeps privately; nothing a contributor must
do depends on them (the checks a contributor runs are in `CONTRIBUTING.md`).

## Changes in session 29

- Basis: the owner's facts of session 29 (0.1.0a4 released on 2026-10-31) and his raw evidence file (private, captured
  2026-10-31 11:55:33 UTC: the tag lines on `origin`, `dist/` and `dist2/` hashes, PyPI's hashes and sizes, the
  installed `_core.abi3.so` hash from PyPI, the three VM pytest last lines). Checked in this session (reading only):
  `git rev-parse v0.1.0a4` = `582045a1...`, `v0.1.0a4^{commit}` = C; `git cat-file -p v0.1.0a4` (tagger time, message);
  `%ct` of C = 1793315613; local `HEAD` = `origin/master` = `9cb6e84`, `git diff --stat C HEAD -- . ':!docs'` empty; the
  `SHA256SUMS` file rebuilt from the evidence's two lines: 206 bytes, sha256 `936bcd14...` (= the stated values);
  PyPI's JSON API for `oradmp` 0.1.0a4 (fetched once); `deny.toml` and `Cargo.lock` at C (the `syn` dependents);
  `.github/workflows/ci.yml` at C (the `JOB_START` lines per job); cargo-deny's `[licenses]` configuration page
  (`unused-allowed-license`). Nothing built.
- New: **ADR-111** (the a4 record; risk 31 closed; tokens and release page as practised), **ADR-112** (T-21 restated:
  the fifth `JOB_START` line, `deny.toml`, the duplicate `syn`, the check that nothing shipped changes, the sdist
  source build), **ADR-113** (after a4: scratch folders, `oradmp-private`, the order, what makes a5, the public face).
- Amended in place (an "Amendment (session 29)" line): ADR-67 item 4, ADR-109 item 6, ADR-110 items 4 and 5.

## Changes in session 28

- Basis: Reviewer 6c (`docs/review/review-06c.md`, C = `4b9883c52ebf709ccf0161883505844fdcc45345`: 0 blocking, SF-1,
  N-1 to N-7) and the owner's facts of session 28 (C's CI runs, the `t20b-proof` run, the release facts). Checked in this
  session (reading only): `git rev-parse 4b9883c`; `%ct` of C = 1793315613; `075a6bf` adds only the review;
  `.github/workflows/ci.yml` at C, lines 57, 178, 200, 318, 363, 402 and the five `uses:` pins with their version
  comments; `tools/release_build.sh` (what it clones, `SOURCE_DATE_EPOCH`, the root checks, `dist/`); the root-only
  skips at C (`tests/test_cli.py:594`, `:923`, `tests/test_paths.py:121`); the stale counts of N-2 and two more of the same
  kind (`t11_options.rs:11`, `:492`); the `uv run` lines of N-3; CONTRIBUTING's setup line against CI's pins. Nothing
  built.
- New: **ADR-109** (6c recorded; T-20b DONE; SF-1, N-1 to N-7 and the unchecked items ruled; task T-21), **ADR-110** (the
  release commit, the hashes it must give, the a4 checklist, the release page, tokens, records, the order after a4).
- Amended in place (an "Amendment (session 28)" line): ADR-106 item 3, ADR-108 item 4.

## Changes in session 27

- Basis: the owner's facts of session 27 (T-20 pushed: `4aa33e6`, `e1ed10b`; its push run and the two dispatches on
  `e1ed10b`; the cut date; the throwaway users; the T-20 note's `pipefail` finding). Checked in this session (reading
  only): `git log -1 --format=%ci` of S `9950ef2` (2026-10-26 15:50:21 -0500) and, in `~/p/oradmp-archive`, of P
  `b55eb10` (2026-10-26 15:25:06 -0500); `%ct` of `e1ed10b` and `4aa33e6` = 1793310726 (= the pasted
  `SOURCE_DATE_EPOCH`); `git show --stat e1ed10b` (the files of T-20 (g) plus the note); `.github/workflows/ci.yml` at
  HEAD read step by step; `tools/release_build.sh` and `tools/fuzz_run.sh` (exit paths); GitHub's "Workflow syntax for
  GitHub Actions" (the default shell table and `defaults.run.shell`). Nothing built.
- New: **ADR-108** (T-20 recorded; the cut date; the throwaway users; `pipefail` in CI and task T-20b; the candidate and
  Reviewer 6c's commit).
- Amended in place (an "Amendment (session 27)" line): ADR-105 item 1 (the cut date), ADR-107 item 3 (the order).

## Changes in session 26

- Basis: the owner's facts of session 26 (the cut done: P = `b55eb10376a9402aee36a1ee61779cf087c7bfb3`, S =
  `9950ef204fb20b4d7f5d9da31467ffbe9aeeff60`; the scrub commits; the public settings; the first public CI runs; T-19 (m)
  `0520cd5`). Checked in this session (reading only): `git log` of S (one commit, 925 files, author and committer the
  GitHub no-reply address, dated 2026-10-26 15:50 -0500); `git ls-tree -r` of P (archive) against S: the only difference
  is `HISTORY.md` (blob `197cfcf`); the archive tags `v0.1.0a1` to `v0.1.0a3` resolve to `b7304d56...`, `ff7d5255...`,
  `ff3d6b25...` as `HISTORY.md` says; the archive clone's GitHub remote names `oradmp-private`; the public clone has one
  remote, `origin`; `.github/workflows/ci.yml` uses only `actions/*` and the pinned `taiki-e/install-action@1e29709c...`;
  `CLAUDE.md`, `.claude/`, `docs/prompts/` are ignored by `.gitignore` and absent from S; no file outside `docs/`,
  `.gitignore` and `HISTORY.md` cites them; the T-19b note's counts (nextest 488 passed + 4 skipped, pytest 251) equal the
  first public run's. Nothing built.
- New: **ADR-105** (the cut recorded; Q-14 closed), **ADR-106** (four rulings: the fuzz compile step, the `T_BIG` scan
  watch, `manylinux2014`, the owner's open items), **ADR-107** (task T-20, the a4 version commit; Reviewer 6c's scope; the
  order to a4).
- Amended in place (an "Amendment (session 26)" line): ADR-90, ADR-93, ADR-104.

## Changes in session 25

- Basis: the owner's facts of session 25 (prompt `docs/prompts/architect-25-update.md`: T-19 committed `921971f`, CI green
  on `bbf921b`; R-048 to R-051 run on 11g and 18c; the new fixtures uncommitted with regenerated `SHA256SUMS`); Analyst
  sessions 19 and 20 (`5b9f249`, `6aaff82`, `588f658`: `exp.md` 5.6, 6.5, 6.6; rules **v3.9**: R2, R8, Y8, Y9, T5;
  `requests.md` R-048 to R-051 "generated and used", R-052 proposed); the T-19 note. Checked in this session (reading
  only): `sha256sum -c` in `11g/` and `18c/` (all OK; 159 and 171 lines); `git status` / `git ls-files` of `corpus/`;
  `tools/corpus_counts.py` (229 files, 105 / 97 / 27, 436 sections, 41,494 rows, 243,905 fields, 20,084 NULL, `problems
  0`); `tools/rowlen_stats.py` (450 parts; `long_conv_8000_up` 28, `long_conv_chunked` 16, `long_conv_fa` 16; `len0` 0,
  `high` 0); `tools/long4_check.py`, `long5_check.py`, `tsltz2_check.py` (`differences: 0` each); `xxd` of
  `long4_exp_18c`@0x22277 (`fa ff fa ff 00 00`), `long4_exp_11g`@0x32285 (`fa ff fc ff 00 00`), `long5_exp_18c`@0x32286
  (`f9 ff`), `long5_exp_11g`@0x5229d (`fa ff fb ff 00 00`); the truth files `truth_long4/5.txt`, `truth_longraw4/5.txt`,
  `truth_tsltz2.txt`; `exp/rows.rs` 670-800, 1010-1020 and its `RawHasher` comment, `reader.rs` 260-275, 515-535, 1297,
  `values/datetime.rs` 262-273, `error.rs` 305-320, `dump.rs` (`buffer_bytes` sites 381, 905), `python/oradmp/_survey.py`
  (the LONG line), `_api.py` (`LongStats`), `exp_parse.py` (`long_fa`), README 12-30 and 126-140. Nothing built.
- New: **ADR-99** (the three conventional LONG / LONG RAW forms; replaces ADR-95 items 1, 3 to 5 and the LONG RAW half of
  item 9), **ADR-100** (R-050 answered: TSLTZ; R-049 negative; R-052; counts), **ADR-101** (the five policy questions),
  **ADR-102** (the four T-19 deviations), **ADR-103** (task T-19b), **ADR-104** (the order to a4, step 3b after T-19b,
  Reviewer 6c's scope, the LONG sentence of the release notes, the public repository name at the cut).
- Amended in place (an "Amendment (session 25)" line): ADR-95, ADR-96, ADR-97 item 4, ADR-98 items 2, 3, 5.

## Changes in session 24

- Basis: the owner's facts of session 24 (prompt `docs/prompts/architect-24-update.md`: R-017, R-040, R-047 run on
  2026-10-25, `cca4c65`; T-16a2 done, CI and VM hashes identical at `af6a54a`; the Q-14 steps not done yet); Analyst
  session 17 (`8162eb0`: `exp.md` 5.4, 5.6, 6.4, 6.6, 13.2; rules **v3.7**: T5, Y5, R2, R8, Y9, E2, E3; `requests.md`
  R-017, R-040, R-047 "generated and used", new R-048, R-049). Checked in this session (reading only): `sha256sum -c` of
  `18c/` and `11g/` (all OK); `tools/long3_check.py` on the four `long3_*` files (`differences: 0`);
  `tools/corpus_counts.py` (211 files, 404 sections, 41,284 rows, 243,303 column fields, 19,960 NULL, `problems 0`);
  `xxd` of `long3_exp_<v>`@0x0 (client `03 69`), @0xa280 `00 80`, @0x2228f `f0 ff`, `long_exp_<v>`@0xa0ea `00 7d`,
  `t_tstz_exp_<v>`@0x229a `e7 00 0b 00`; `crates/oradmp-core/src/exp/rows.rs` 600-760 and 960-1045,
  `src/reader.rs` 566-690, `src/types/map.rs` 209-280, `src/types/resolve.rs` 148-165 and 331-395;
  `tests/common/mod.rs` (`TOTALS`), `t10_truth.rs` 1688-1720 and 1797-1830, `t3_catalog.rs` 60-72, `t14_damage.rs`
  195-224, `tests/common/synth.rs` 70-77, 925-950, 1040-1075, 1410-1490; the `201` assertions of `tests/golden/gen_*.py`
  and `crates/oradmp-core/tests/data/gen_number_parts.py`; `python/oradmp/_errors.py`, `_api.py` (`LongStats`),
  `_cli.py` 300-320; `README.md` 12-25, 128-136; `CLAUDE.md` (ownership table). Nothing built or run beyond the
  Analyst's tools.
- New: **ADR-94** (Analyst 17 recorded; the ten new fixtures and the red corpus tests), **ADR-95** (the conventional LONG
  and LONG RAW field; replaces ADR-12's session 4 case (b); rulings on the Analyst's questions 1 and 2), **ADR-96**
  (TIMESTAMP WITH LOCAL TIME ZONE length: question 3, R-050), **ADR-97** (task T-19; T-19 does not wait for R-048),
  **ADR-98** (the a4 order with T-19 first; Reviewer 6c's scope; the LONG sentence of the release notes; T-16a2 recorded).
- Amended in place (an "Amendment (session 24)" line): ADR-12 (session 4 case (b) replaced), ADR-15 (item 1's
  conventional sentence, item 4's threshold), ADR-86 item 1 (by ADR-94 items 1 and 2), ADR-93 item 1 (by ADR-98).

## Changes in session 23

- Basis: the owner's facts of session 23 (prompt `docs/prompts/architect-23-update.md`: T-16a's GitHub results at
  `d081109`, the reproducibility finding, Q-14's lean); the T-16a note; `.github/workflows/ci.yml` at `d081109`;
  `tools/release_build.sh`; `git show --stat` of `c1d1c58`, `7a2d028`, `d081109`. Checked in this session (reading
  only): the VM's build of `d081109` below `/var/tmp/oradmp-release-ci2` (its `src` is at `d081109b...`; `SHA256SUMS`;
  the SBOM member's `/var/tmp` paths counted with the owner's command); `ls -d /var/tmp/oradmp-*` (the default root
  `/var/tmp/oradmp-release` does not exist now); `git grep -l` for the home path, the e-mail address, `IDENTIFIED BY`
  and the word "password" (file names only, no value printed); PyPI's JSON API for pyarrow 15.0.2 and 26.0.0 (wheel
  file names); GitHub's Actions billing page (docs.github.com, "GitHub Actions billing"). Rules unchanged (v3.6);
  nothing researched about the formats.
- New: **ADR-89** (T-16a and T-17b recorded; O-22 closed; what is open), **ADR-90** (the canonical release root, the
  SBOM, which hash is authoritative, the reproducibility claim), **ADR-91** (a4 ships `manylinux_2_28` only;
  `manylinux2014` becomes task T-16c), **ADR-92** (CI follow-ups: the fuzz crash artifact, benchmarks, what the owner
  pastes, the minutes budget; task T-16a2), **ADR-93** (the cut's steps and its place before the a4 version commit;
  the a4 order; T-16b and visibility). ADR-88 item 2 is amended by ADR-93.

## Changes in session 22

- Basis: Reviewer 6b (`docs/review/review-06b-t18d-t17.md`, HEAD `edc62a5`: no blocking finding, SF-1 to SF-5, N-1 to
  N-8); the T-18d and T-17 notes; the owner's facts of session 22 (prompt `docs/prompts/architect-22-update.md`: O-21
  done, O-22 answered "the owner pastes the logs", the one-time cut as a question only). Rules unchanged (v3.6);
  nothing researched about the formats. Checked in this session (reading only): `README.md:159`;
  `crates/oradmp-core/tests/common/synth.rs` lines 1454-1463 (the `i == 0` guard), 1468-1475 (`db_171` and
  `client_pick` drawn independently of the version), 1495 (`rng.range(0, 9)` for TIMESTAMP);
  `crates/oradmp-core/tests/t17_synth.rs:97-100` (`nt.arrow()`); `grep proptest!|failure_persistence` over
  `crates/oradmp-core/tests/*.rs` (`t4_values.rs:1160`, `t5_types.rs:1172`, `t14_damage.rs:186` without
  `failure_persistence`); `tools/fuzz_run.sh` lines 33-40 (`../target/fuzz-seeds` for every target); `git diff ba8a89c
  edc62a5 -- CONTRIBUTING.md tools/fuzz_seeds.sh` (three added lines above 120 characters); `docs/formats/exp_reader_rules.md`
  H1 to H6, T5, Y5, E1 to E4, "Status and scope"; `docs/formats/requests.md` R-017 and the index (next free number
  R-047); `CLAUDE.md` rule 8 (no agent uses a GitHub token); `git grep -l` for `<home>`, the e-mail address and the
  word "password" in tracked files (counts in Q-14). No code read beyond these lines.
- New: **ADR-85** (Reviewer 6b recorded; what it closes), **ADR-86** (rulings on SF-1 to SF-5 and N-1 to N-8; task
  T-17b; the labelling of unconfirmed generator constructs), **ADR-87** (T-16 split into T-16a and T-16b; the logs
  route; T-17b first), **ADR-88** (0.1.0a4 restated: scope, order, Reviewer 6c).
- Amended in place (an "Amendment (session 22)" line at the end of each): ADR-79 (item 3 (c): the token route is
  withdrawn), ADR-80 (item 1: unconfirmed combinations; the seed folder), ADR-82 (item 3: every path in the tool's
  messages), ADR-83 (order superseded by ADR-87), ADR-84 (items 1 and 2 superseded by ADR-88).

## Changes in session 21

- Basis: the owner's facts of session 21 (prompt `docs/prompts/architect-21-update.md`: 0.1.0a3 released on
  2026-10-24, the Rocky Linux 8 run, the uploads, the tag); Reviewer 6a-closure
  (`docs/review/review-06a-closure-t18c.md`: no blocking finding at `ff3d6b2`, SF-1, SF-2, N-1 to N-8). Rules unchanged
  (v3.6); nothing researched about the formats. Checked in this session (reading only): `git rev-list -n1 v0.1.0a3`
  (`ff3d6b25...`, an annotated tag), `git ls-remote --tags github` (`v0.1.0a3` -> `ff3d6b25...`; `v0.1.0a2` ->
  `ff7d525...`), `git ls-remote --tags origin` (not possible: the Gogs server asks for credentials), `git log`
  (`9f1dc4d`: O-21 applied), `CLAUDE.md` (the `.github/workflows/` line and the Implementer's row), `tests/test_cli.py`
  lines 576 and 897 (the two root skips), `tests/test_project_name.py:33-34`, `python/oradmp/_cli.py:620-638`,
  `python/oradmp/_errors.py:157-159`, `python/oradmp/_survey.py:67`, `python/oradmp/_api.py:536-537`,
  `crates/oradmp-py/src/lib.rs:653` (`path: String`), `crates/oradmp-core/src/dump.rs:377` (`open(path: impl
  AsRef<Path>, ...)`), `crates/oradmp-core/tests/t15_*.rs` (`#[ignore]`, skip without `ORADMP_WORK`), `.gitattributes`,
  `du -sh corpus/fixtures` (16 MB, 556 tracked files), `docs/formats/requests.md` R-003 and `xxd` of
  `lob_exp_18c.dmp` at 0x24f0 to 0x253f (for the R-003 wording). External, general engineering only: PEP 440
  (pre-release handling), the Python documentation of `os.fsencode` / `os.fsdecode` and of the `backslashreplace` error
  handler, pyo3's conversion table (`PathBuf`).
- New: **ADR-81** (0.1.0a3 recorded; lessons), **ADR-82** (T-18d; file names that are not UTF-8), **ADR-83** (order
  after a3; what CI runs and what it does not), **ADR-84** (0.1.0a4: content, gate, steps).
- Amended in place: ADR-77 item 4 (the path text, by ADR-82 item 3), ADR-79 (O-21 done; an "Amendment (session 21)"
  line at its end).

## Changes in session 20

- Basis: Reviewer session 6a (`docs/review/review-06a-t13j-t18b.md`, candidate `d4d20ef`: no blocking finding,
  SF-1, SF-2, N-1 to N-8); the owner's facts of session 20 (prompt `docs/prompts/architect-20-update.md`: the lean
  towards T-18c, the private GitHub repository, no Project-URL in a3, R-003). Rules unchanged (v3.6); nothing
  researched about the formats. Checked in this session (reading only): `git diff --stat d4d20ef HEAD -- . ':!docs'`
  (empty), `git diff --stat 460aef5 d4d20ef -- crates/oradmp-core/src rust-toolchain.toml` (empty), `git diff 460aef5
  d4d20ef -- Cargo.toml` (version line only), `python/oradmp/_cli.py` (`_format_for`, `_escape`, `_error_text`),
  `crates/oradmp-py/src/errors.rs` and `crates/oradmp-core/src/error.rs` (how `E-IO` reaches Python),
  `tests/test_cli.py` (the `PYTHONPATH` lines), README "Command line", `tools/release_build.sh` (the build root must
  not exist), `CLAUDE.md` and the root `.gitignore` (O-16, O-19 (b)), `docs/formats/requests.md` R-003 and R-040.
  External, general engineering only: GitHub's Actions billing page; pip's `pip install` documentation (pre-releases).
- New: **ADR-77** (Reviewer 6a recorded; T-18c and Reviewer 6a-closure before the a3 upload; the I/O error text; a3
  keeps `0.1.0a3`, no version commit, tag pushed after PyPI), **ADR-78** (baseline 2; the regression rule), **ADR-79**
  (GitHub Actions; what T-16 needs; `.github/` is the Implementer's), **ADR-80** (T-17).
- Amended in place: ADR-66, ADR-72, ADR-76 (an "Amendment (session 20)" line at the end of each).

## Changes in session 19

- Basis: the owner's facts of session 19 (prompt `docs/prompts/architect-19-update.md`: the built commit of 0.1.0a2,
  the PyPI comparison, tokens, `T_BIG`, O-19, Q-11); the T-13j, T-15b and T-18 notes (`implementer-notes.md`;
  commits `1c17633` + `460aef5`, `0956df0`, `2ba62eb`); review 05. Rules unchanged (v3.6); nothing researched about
  the formats. Checked in this session (reading only): `git rev-parse v0.1.0a2^{}` / `v0.1.0a1^{}`, `git cat-file -p`
  of both tags, `git log -1 --format=%ct` of `4e8f212` and `ff7d525`, `git diff --stat dc7668e ff7d525 -- . ':!docs'`,
  `git diff --stat 4e8f212 ff7d525`, `git diff --stat 460aef5 HEAD -- crates/`, `git diff --stat 39cbb93 HEAD --
  crates/oradmp-core/src/`, `git ls-files uv.lock`, the root `.gitignore`, `CLAUDE.md` (Ownership), `corpus/sql/15_big.sql`,
  the `T_BIG` block of `corpus/sql/01b_optional_tables.sql`, `README.md`, the README tests (`tests/test_version.py`,
  `tests/test_project_name.py`), `docs/formats/requests.md` R-001 to R-011, `ls corpus/fixtures/*/`.
- New: **ADR-74** (the 0.1.0a2 record corrected: built commit `ff7d525`; O-18 closed; risk 23 closed), **ADR-75**
  (T-15b's results and the memory test; the T-18 rulings; new task **T-18b**), **ADR-76** (Reviewer session 6 split
  into 6a now and 6b with T-16; 0.1.0a3 without T-16; T-17 after the a3 tag; the Project-URL).
- Amended in place: ADR-65, ADR-69, ADR-70, ADR-71, ADR-72, ADR-73 (an "Amendment (session 19)" line at the end of
  each).

## Changes in session 18

- Basis: Reviewer session 5 (`docs/review/review-05-t13i-closure.md`, HEAD `dc7668e`: no blocking finding, SF-1 to
  SF-3, N-1 to N-10); the owner's record of release run B (prompt of session 18, "Owner facts"). Rules unchanged
  (v3.6); nothing researched about the formats. Checked in this session (reading only): `git log`, `git tag`, `git
  cat-file -p v0.1.0a2`, `git diff --stat dc7668e 4e8f212 -- . ':!docs'`, `git diff dc7668e 4e8f212 -- README.md`,
  `tools/release_build.sh` (how the epoch is taken), `tests/check_dist.py` (the rules SF-3 names), `pyproject.toml`,
  `python/oradmp/` (`__main__.py`, `_survey.py`, `_api.py`, `_errors.py`), the head of `crates/oradmp-core/src/lib.rs`
  and its `Cargo.toml`, `docs/formats/README.md`, the root `.gitignore` and `CLAUDE.md`.
- New: **ADR-70** (run B recorded; its deviations; the lessons; ADR-67 closed), **ADR-71** (review 05 -> T-13j; what
  goes into 0.1.0a3 and when), **ADR-72** (T-18, the command-line tool), **ADR-73** (order after run B; the DuckDB side
  project only as an open question, Q-13).
- Amended in place: ADR-59, ADR-64, ADR-65, ADR-67, ADR-68, ADR-69 (an "Amendment (session 18)" line at the end of
  each).

## Changes in session 17

- Basis: the T-13i note (`implementer-notes.md`, "T-13i note"; commits `469416d`, `dc7668e`; 411 Rust tests + 3
  ignored, 117 pytest, 0 skipped). Rules file now **v3.6** (Analyst session 11, in `469416d`); rule L4 and reader step 6
  re-read for ADR-69 item 2, nothing else of v3.6 re-planned here. Checked in this session (reading only):
  `exp/rows.rs` `lob_value` (the `fc ff` branch), `io.rs` `read_bytes` / `read_u16_le` / `truncated`, the doc comment
  of `t14_damage.rs::each_truncation_site_names_its_offset`, `t15_memory.rs` (the `T_BIG` branch), the README's
  stream example, arch 3.4 and ADR-55 item 3 (when warnings are issued), `corpus/sql/01b_optional_tables.sql` (`T_BIG`).
- New: **ADR-69** (the three T-13i deviations and the d9 / d10 rules; new tasks T-13j and T-15b; owner action O-17).
- Amended in place: ADR-64 item 2, ADR-65 items 3 and 6, ADR-68 (an "Amendment (session 17)" line at the end of each).

## Changes in session 16

- Basis: Reviewer session 4 (`docs/review/review-04-t13h-t15.md`, HEAD `b3a5a7f`: no blocking finding, S-1 to S-5,
  N-1 to N-10; 408 Rust tests + 3 ignored, 105 pytest); the T-14 and T-15 notes. Rules unchanged (v3.5); nothing
  researched about the formats. Checked in this session: `exp_parse.py --rows` and `num_decode.py` on
  `18c/t_num_exp_18c.dmp` (ADR-64 item 2; checksum equal to `SHA256SUMS`), `xxd` of the patched bytes;
  `crates/oradmp-py/src/lib.rs` and `options.rs` (the calls outside the guard, ADR-65 item 1), `reader.rs` `in_piece`,
  `framing.rs` `read_unlisted_field`, `tests/check_dist.py`; the root and `fuzz/` `.gitignore`; the README's version
  lines; `git tag` (`v0.1.0a1` on `b7304d5`); `uv publish --help` of uv 0.12.21.
- New: **ADR-64** (S-1 to S-5 -> T-13i), **ADR-65** (notes and surviving mutations, memory limit), **ADR-66**
  (benchmark baseline and regression rule; `benchmarks.md`), **ADR-67** (release run B: closure review, version bump,
  TestPyPI then PyPI, token handling), **ADR-68** (T-16 / T-17 revised; order).
- Amended in place: ADR-58, ADR-59, ADR-61, ADR-62, ADR-63 (an "Amendment (session 16)" line at the end of each).

## Changes in session 15

- Basis: the T-13h note (`implementer-notes.md`, "T-13h note"; commit `39cbb93`; 402 Rust tests, 105 pytest, 0 skipped);
  the owner's answer to Q-12 (internal hand-out now, PyPI after Reviewer 4). Rules unchanged (v3.5); nothing researched
  about the formats. Checked in this session: arrow-schema 59.3.1 `Display` of `ArrowError`, arrow-array 59.3.1
  `ffi_stream.rs` (`get_error_code`, the `CString` of the last error), pyo3 0.30.1 `Python::detach` bounds (`Ungil`),
  the rust-fuzz book ("Setup": nightly and a C++11 compiler), the README at `39cbb93`, the VM's toolchains and memory.
- New: **ADR-61** (T-13h deviations 1 and 2 and its smaller points), **ADR-62** (T-14 and T-15: one after the other,
  models, the machine's memory rule, fuzz layout and limits; arch 8.3 (b) moves to T-17), **ADR-63** (0.1.0a1 internal
  only; the PyPI upload is the reviewed build, 0.1.0a2).
- Amended in place: ADR-55 (items 5, 6, 7), ADR-58 (item 4), ADR-59 (items 2, 5 (b)): an "Amendment (session 15)" line
  at the end of each.

## Changes in session 14

- Basis: Reviewer session 3 (`docs/review/review-03-python-packaging.md`, HEAD `3a1bb5f`: no blocking finding, S-1 to
  S-5, N-1 to N-11; 395 Rust tests, 73 pytest). Rules unchanged (v3.5); nothing researched about the formats. Checked
  on the VM in this session: `crates/oradmp-py/src/lib.rs` (`arrow_c_stream`, the locks), `python/oradmp/_api.py`
  (`BatchReader`), `tests/check_dist.py`, `crates/oradmp-core/tests/t12_messages.rs`, the metadata keys of `reader.rs`,
  the workspace `Cargo.toml` (no `panic = "abort"`); the maturin user guide (maturin.rs, "Configuration" and "SBOM"),
  `maturin build --help` and the strings of the pinned maturin 1.15.0 binary (ADR-56).
- New: **ADR-55** (S-1: the C stream path shares the reader's state), **ADR-56** (S-2, N-11: release build script,
  neutral path, remaps, SBOM kept), **ADR-57** (S-3: `check_dist.py` rules), **ADR-58** (S-4: the message check sees
  decoded values and hex substrings), **ADR-59** (S-5: `E-INTERNAL`, `InternalError`, the panic guard), **ADR-60**
  (N-1 to N-11, the order, Q-12).
- ADR-50's exact README text: the Install sentence with the old name is superseded by ADR-54 item 2 (ADR-60, N-6). ADR-54 item 1's
  key list: `oradmp:lob_charset` is not a key (ADR-60, N-3).

## Changes in session 13

- Basis: the T-13f note (`implementer-notes.md`, "T-13f note (Implementer)"; commit `3fe33e7`; **395** Rust tests,
  **69** pytest, `licenses ok`); the owner's decision on the name (Q-10, name part: `oradump` is taken on PyPI,
  the new name is **`oradmp`**, free on PyPI, owner checked on 2026-10-18). Rules unchanged (v3.5); nothing researched
  about the formats. Checked on the VM in this session: `grep -rIci oradump` over the repository (without `target/`,
  `.venv/`, `.git/`), `grep -rlai oradump` over `corpus/` (no fixture, truth file or golden file holds the name),
  `strings` on the T-13f wheel's `_core.abi3.so` (only relative paths and the field-metadata keys), the `.venv`
  editable install (`oradump.pth`, `oradump-0.1.0.dist-info`).
- New: **ADR-54** (rename to `oradmp`: names, inventory, policy for old text, clean room, `__version__`, task T-13g).
- The line at the top of this file: the records before ADR-54 keep the old name.

## Changes in session 12

- Basis: the T-13 note (`implementer-notes.md`, "T-13 (Implementer, session 12)"; commit `e2eaae6`; **395** Rust tests,
  **69** pytest; `cargo deny check licenses` red); the owner's answers Q-8 and Q-9 (`owner-decisions-2026-10-16.md`,
  "2026-10-18"). Rules unchanged (v3.5); nothing researched about the formats. Checked on the VM in this session:
  `arrow-array 59.3.1`'s manifest and `cargo tree` (ADR-49), a scratch copy of `deny.toml` under cargo-deny 0.20.2
  (ADR-49), Cargo's and maturin 1.15.0's handling of a pre-release version on a scratch crate (ADR-50), a scratch
  attributes file with `git check-attr` (ADR-52).
- New: **ADR-49** (cargo-deny: `tiny-keccak` is a wasm32-only dependency; the check is limited to the targets we
  build; no CC0 allowance), **ADR-50** (version `0.1.0-alpha.1` / `0.1.0a1`, one source; release checklist; README
  install text; no crates.io), **ADR-51** (the T-13 choices P2 to P9), **ADR-52** (`.gitattributes`), **ADR-53**
  (T-16 "More platforms"; the workarounds until then; Q-9 closed).
- ADR-48 item 1: Q-9 answered (RHEL 8+, x86_64 only): `manylinux_2_28` stands; the status line "needs the project
  owner" no longer applies.

## Changes in session 11

- Basis: the T-11b and T-12 notes (`implementer-notes.md`; commit `a19d873`; **390** Rust tests, **58** pytest).
  Rules unchanged (v3.5); nothing researched about the formats. Crate facts read on docs.rs on 2026-10-17
  (`pyo3-arrow` 0.20.0, `arrow-array` 60.0.0 `ffi_stream`, `pyo3` 0.30.1 `PyCapsule`) and on maturin.rs
  (Distribution, Metadata pages).
- **ADR-3** amended: arrow **59** (59.3.1), pyo3 0.30.1, pyo3-arrow 0.20.0 accepted (T-12 D1); rule for future bumps.
- **ADR-19** amended: Parquet refuses Interval(MonthDayNano) (T-12 finding 6); `interval_as` moves into the alpha
  (ADR-47). **ADR-28** (b): the results of checks 1 to 6 recorded.
- **ADR-41** amended (items 8, 9): `Error.warning` as completed by T-12 (D3) confirmed; the `E-STRICT` default for
  every warning without an error of its own (the T-11b table) confirmed; review N-1 stays open for Reviewer 3.
- New: **ADR-46** (the T-12 choices D2, D4 to D10 and the pandas / pyarrow findings; the survey's default second
  read kept), **ADR-47** (`interval_as` in the alpha, T-13), **ADR-48** (first-release wheel: `manylinux_2_28`
  baseline, maturin pinned, metadata and contents checked; Q-9 to the owner).

## Changes in session 10

- Basis: Reviewer session 2 (`docs/review/review-02-t9b-t11.md`, HEAD `7941e40`, **371** tests pass): no blocking
  finding, S-1 to S-4, notes N-1 to N-9, mutations d1, d2, e1 (survey), e3, e4 not caught; the T-10b note's
  items 1 to 6; the owner's answer to Q-6. Rules unchanged (v3.5); nothing researched about the formats.
- New: **ADR-43** (the second review's findings: S-1 to S-4, e3 / e4, who fixes what in tasks T-11b and T-12;
  the offset rule of one value replaces ADR-41 item 6), **ADR-44** (T-10b note items 1 to 6, the review's notes,
  the stale numbers), **ADR-45** (licence: MIT OR Apache-2.0, files, metadata, `deny.toml`, the corpus out of the
  packages).
- ADR-41 item 6 is widened by ADR-43 item 3 (every report about one value carries the start of the value, under
  every policy); arch 7.4 follows.

## Changes in session 9

- Basis: the Reviewer's first report `docs/review/review-01-golden.md` (HEAD `7211137`; no blocking finding,
  S-1 to S-6, notes N-1 to N-8), Implementer notes N-21 to N-25 (T-11 committed as `7db4667`, 347 `#[test]`
  functions counted with `grep`). Rules unchanged (v3.5); nothing researched.
- New: **ADR-38** (the review's findings: who fixes what, task T-10b widened with named mutations),
  **ADR-39** (the three `W-POSSIBLY-INCOMPLETE` entries of the warning inventory: approved), **ADR-40** (the
  open points of N-21 to N-24, among them `Error.message` as `Cow<'static, str>` in T-12), **ADR-41** (the
  T-11 choices of N-25: `E-STRICT`, `E-UNSUPPORTED-CHARSET`, strict and `null`, the `W-CHARSET-MISMATCH` bug of
  T-9 and the new task T-9b, the offset of `W-LONG-LENGTH`), **ADR-42** (roles: the Reviewer replaces the
  "Test engineer"; a review session after each group of tasks).

## Changes in session 8

- Basis: rules v3.5 (Analyst session 10: R-038 answered, `exp.md` 14.6 "Session 10", rules L4, L5, C6, X5;
  `tools/lob_ru_check.py` 77 checks, 0 differ); `ru_lob2_exp_cp1251_11g.dmp`, `ru_lob2_exp_us7ascii_11g.dmp`,
  `truth_lob_ru.txt`. Nothing researched outside `docs/formats/` and the fixtures.
- **ADR-25 item 2 lifted for the database set 171** (session 8 block in the record): CLOB and NCLOB of a
  CL8MSWIN1251 database decode in both forms without `E-LOB-UNCONFIRMED` and without a warning. Sets 1, 178 and
  every other database set except 873 keep the stop. "Confirmed" is one list in the code,
  `LOB_CONFIRMED_DB_SETS = [873, 171]`, compared with the header's database set [H4], never with `db_charset`.
- New **ADR-37**: LOB chunk lengths come from the stream only; the chunk sizes named in this file (ADR-18,
  ADR-32 item 1) are observations per version and database set, not rules.
- ADR-34 item 9 carries a session 8 line: the three places it names change in task T-6M.

## Changes in session 7

- Basis: Implementer notes N-19 (T-6L built), N-20 (T-5f built), the T-8 notes and the T-7 / T-8 test files
  (no production change in either). Rules unchanged (v3.4); nothing researched. 232 tests pass.
- New: **ADR-34** (the T-6L choices: `lob_start` with the form, the two LOB counters `#[doc(hidden)]`, the
  order of the two LOB refusals, `E-LOB-UNCONFIRMED` / `W-LOB-UNTESTED` in the code tables and in Python,
  `W-LOB-UNTESTED` under `strict`), **ADR-35** (the T-5f choices: `NUMBER(*,s)` is a usable declaration,
  `NUMBER(*,-2)` takes `number_default`, the warning order of one column, `W-TSLTZ` once per column in
  Python), **ADR-36** (what T-7 and T-8 established without a code change).
- ADR-25 item 3 and ADR-33 item 4 carry a one-line session 7 amendment pointing to ADR-34 / ADR-35.

## Changes in session 6

- Basis: Implementer notes N-17 (T-3b built) and N-18 (T-6 built). Rules unchanged (v3.4); nothing new
  researched about the formats. One general Oracle fact is used (ADR-32 item 4: a table with a LONG
  column cannot be partitioned, ORA-14085, Oracle Database error documentation; LONG RAW by the Oracle9i
  partitioning text, to be observed by R-045).
- New: **ADR-32** (the T-3b choices: LOB and assembled-LONG digests, `Dump::rows` refuses a table of more
  than one part, the order warning of a later part, the fixture gap), **ADR-33** (the T-6 choices: the
  `RowSink` seam, value options in `ReadOptions`, counted warnings in `TableBatches::warnings()`, which
  NUMBER columns `on_number_overflow` governs (`ColumnInfo.type_source`), the per-column policy and the
  "largest number of digits dropped" built in T-11 for the alpha, smaller points).
- ADR-18: the incremental-write behaviour of the arrow-array builders is **verified** on arrow-array
  60.0.0 (T-6, N-18).

## Changes in session 5

- Basis: Implementer notes N-12 to N-16 (T-3c, T-4, T-5 built); the owner's decisions of 2026-10-16
  (`owner-decisions.md`: Q-1 `round`, Q-7 accepted, Q-3 Python 3.11). Rules unchanged (v3.4); nothing new
  researched.
- Owner decisions folded in: ADR-13 **decided** (Python 3.11, abi3-py311, pyarrow 15); ADR-14 **decided**
  (`round` by default; with `strict=True` a rounded value is an error); ADR-25 accepted by the owner (Q-7).
- New: ADR-27 (the T-4 codes and checks: `E-NUMBER-OVERFLOW`, `E-BAD-DATETIME`, `W-UNVERIFIED-TYPE` as a
  `WarningCode`, TSTZ offset bounds, interval range, empty logical UROWID tail, DATE byte pairs), ADR-28
  (values the Arrow type cannot hold: INTERVAL YEAR(9) beyond 32-bit months stays an error; DATE /
  TIMESTAMP years below 1 stay an error; what T-12 checks in pandas / pyarrow), ADR-29 (the T-3c choices:
  `Warning.column` / `row` / `part`, chain start `00 80`, two more partition-order warnings, the raw hash of
  chains, `strict` from T-11), ADR-30 (the T-5 choices: where an unknown type text ends, out-of-range
  parameters, a declared `NUMBER` that no Decimal128 holds now warns and takes `number_default`, the form
  check, the order warning, `number_types` against `number_default`), ADR-31 (partitioned tables: by-name
  matching and checks per part).
- ADR-21 item 3 is restated by name in ADR-31 (a later part may differ from the first in order, not in
  names or descriptors); item 4 gains the two cases of ADR-29.

## Changes in session 4

- Basis: rules v3.4 (Analyst session 9: 199 files; T3 / T9 narrowed, R2 / R8 / Y9 piece chains, C8,
  Y1 computed values, Y10, Y11, L4, L5, T10 composite partitions, E2, E2a); Implementer notes N-9 to
  N-11 (T-3a built); the owner's answers on the T-3a choices (given with the session 4 prompt).
- ADR-12: a conventional LONG length from 0x8000 is `UnsupportedLayout` [R8, R-040]; a path flag other
  than 0 / 1 is `UnsupportedLayout` (owner, accepted).
- ADR-14 rewritten with [Y1] v3.4 (computed quotients have 39 / 40 digits) and a Decimal256 analysis;
  still **needs the project owner** (Q-1). `decimal256(p,s)` becomes a value of `number_default` and
  `number_types` (decided, whatever the default).
- ADR-15: direct-path LONG is a piece chain [R2, Y9]; LONG / LONG RAW become LargeUtf8 / LargeBinary;
  `W-LONG-DIRECT` retired, `W-LONG-CHARSET` narrowed to single-byte databases.
- ADR-18: `W-LOB-CHARSET` retired ([L5] confirmed for non-ASCII and non-BMP text); the single-byte
  case moves to ADR-25.
- ADR-19: [Y10] confirmed for negative intervals; precisions 9 make the Duration range too small:
  INTERVAL DAY TO SECOND with a leading precision beyond the Duration range becomes Interval(MonthDayNano).
- ADR-20: logical UROWID is read [Y11]; physical UROWID text without warning.
- ADR-21: composite partitions [T10]; the walker's part rules; `PartInfo.subpartition`.
- ADR-22: narrowed to single-column LONG tables [E2a].
- New: ADR-23 (the T-3a choices the owner accepted, and the one he changed), ADR-24 (column order:
  `INSERT INTO` order for reading, `CREATE TABLE` order for the schema), ADR-25 (CLOB / NCLOB of a
  single-byte database until R-038), ADR-26 (what is checked against the descriptor length [C8]).

## Changes in session 3

- Basis: rules v3.3 (Analyst session 8: L1 to L6, X5, Y10, Y11, T10, E2a, R8 reworded, notes A-6 to
  A-10 answered); 177 fixtures.
- ADR-12 rewritten: LOB, INTERVAL, ROWID, UROWID no longer stop the file; what still stops is a code
  outside the 16 of [T5] and the layouts of [X5]; no file of the corpus stops.
- ADR-6: declared-type texts of session 8 [T9]; interval precisions come only from the `CREATE TABLE` line.
- ADR-14: the "40 digits" fact rests on one fixture value (`t_num` row 31) [Y1]; wording corrected.
- ADR-15: character set of LONG now stated by [Y9] (note A-8), still suggested; LONG may become
  LargeUtf8 if R-031 shows the long form.
- ADR-17: the mode pairing is confirmed [H6] (A-10); a mismatch stays a warning (outside the confirmed
  rules), made in T-3a.
- New: ADR-18 LOB columns (read, LargeUtf8 / LargeBinary, `max_lob_bytes`), ADR-19 INTERVAL types,
  ADR-20 ROWID and UROWID as text, ADR-21 partitioned tables in the catalog, ADR-22 the empty-table
  decision with a length-0 first column [E2a].

## Changes in session 2

- Owner decisions of 2026-10-08 (`owner-decisions.md`) folded in: ADR-6 (Q-1 accepted, then reopened
  by ADR-14), ADR-11 (Q-2 accepted with refinements), ADR-13 (Q-3 still open).
- ADR-5: [C7] and what `encoding_rs` returns for the undefined bytes; the deciding test.
- ADR-6: grammar of the column list confirmed [T9]; `NUMBER(*,0)` gives Decimal128(38, 0); fallback per column.
- ADR-12: the stop now applies to type 112 and unknown codes only; consequences recomputed.
- New: ADR-14 NUMBER overflow policy (needs the project owner), ADR-15 LONG and LONG RAW, ADR-16 one
  rule for US7ASCII high bytes, ADR-17 header checks without a known meaning are warnings.

## ADR-1 Two crates: `oradump-core` and `oradump-py` - decided
- Context: eight internal layers (2.1), one consumer (the Python package), tests that must run without
  Python.
- Decision: `oradump-core` (pure Rust, layers as modules) and `oradump-py` (PyO3 `cdylib`).
- Consequences: one version, one `Cargo.lock`; layer direction is kept by visibility and review, not by
  the compiler; `cargo nextest` needs no interpreter.
- Rejected: one crate per layer (compiler-checked direction, but eight manifests and public APIs nobody
  uses); a single crate with a `python` feature (PyO3 linking then leaks into every test run).

## ADR-2 One seam for a second format: trait `RowSource` - decided
- Context: Data Pump comes later; its layout is not part of this session.
- Decision: the format-specific part ends at `RowSource` (table headers and raw fields, 2.3). Value
  decoding, Arrow and the API are above it and shared.
- Consequences: a Data Pump reader is a second implementation plus, if its value bytes differ (unknown),
  a second decoder set in `values`. `TableHeader` must not grow `exp`-only fields without need: path and
  end marker live in an `exp` extension of it.
- Rejected: a seam at the Arrow level (each format builds its own batches: all decoders written twice); a
  plug-in registry (a framework for two formats).

## ADR-3 Arrow hand-over: `pyo3-arrow` and the Arrow PyCapsule Interface - decided
- Context: batches must reach pyarrow without copies. Candidates: `pyo3-arrow`; the `pyarrow` module of
  arrow-rs (crate `arrow-pyarrow`); hand-written FFI with `FFI_ArrowArrayStream` (arrow-rs `ffi_stream`).
- Decision: `pyo3-arrow`. `PyRecordBatchReader::new` takes a boxed `RecordBatchReader + Send` and exports
  `__arrow_c_stream__`; `into_pyarrow` needs pyarrow 15 or later. Our `BatchReader` class sits in front of
  it for typed exceptions (3.4).
- Consequences: the versions of `pyo3` and the arrow crates follow `pyo3-arrow`; any consumer of the
  PyCapsule Interface works, not only pyarrow; the core stays free of `unsafe`.
- Rejected: `arrow-pyarrow` (imports pyarrow for every conversion and ties us to its conversion traits;
  no gain over the capsule protocol); own FFI (`unsafe` code we would have to maintain and fuzz).
- To verify in T-12 (not read in this session): the export method of `pyo3_arrow::PyRecordBatch` and
  `PySchema` for single batches.
- **Session 11 (T-12 D1): versions accepted as built.** `pyo3-arrow` 0.20.0 (newest on docs.rs, 2026-10-15) needs
  `arrow-array` / `arrow-schema` `^59` and `pyo3` `^0.30`; a `RecordBatch` of arrow 60 is another type. The core is on
  **arrow-array / -buffer / -schema 59** (lock: 59.3.1), the binding on **pyo3 0.30.1** with `abi3-py311`; all 376
  core tests passed unchanged on 59. T-12 verified: `PyRecordBatch::into_pyarrow` and `PySchema::into_pyarrow` work for
  single batches; `PyRecordBatchReader::__arrow_c_stream__` is private, so the binding calls it through Python.
  - Alternative checked: return to arrow 60 and export with arrow's own C stream. It needs **no `unsafe`**:
    `arrow_array::ffi_stream::FFI_ArrowArrayStream::new(Box<dyn RecordBatchReader + Send>)` is a safe fn (feature
    `ffi`; only `from_raw`, `set_release`, `set_private_data` are unsafe), the struct is `#[repr(C)]` and its `Drop`
    calls `release` only if set; `pyo3::types::PyCapsule::new_with_value(py, value, name)` (0.30) is safe and boxes the
    value, so the capsule pointer is the struct; single batches would use the safe `arrow_array::ffi::to_ffi`. Cost:
    about 100 lines of capsule code of our own (stream, array and schema capsules, their names, `requested_schema`,
    the consumed-stream and release-once tests), which `pyo3-arrow` maintains and tests today.
  - Cost of keeping (a): the core trails arrow's newest major by as much as `pyo3-arrow` does (today one major,
    released the same month); arrow 60 brings nothing the reader uses.
  - **Decision: (a).** **Bump rule**: arrow-*, `pyo3` and `pyo3-arrow` move **together**, only as their own small
    Implementer task (never inside a feature task), only after a `pyo3-arrow` release that requires the new arrow;
    checks: `cargo tree -d` lists one version of each arrow crate; fmt, clippy, all nextest tests (ADR-18's builder
    tests included), `maturin develop` + `ruff`, `mypy`, `pytest`; the abi3 wheel check of ADR-48; versions written
    into arch 3.4 / 9 and this record. If `pyo3-arrow` falls **two arrow majors** behind, or a needed arrow fix lands
    only in a newer major, the Architect revisits with the capsule route above (safe, as checked).

## ADR-4 Reading model: scan, then independent table readers; buffered file I/O - decided
- Context: no directory and no row count in the file [R6]; target files of gigabytes; one table is often
  wanted, not all.
- Decision: `scan()` walks the file once and skips values; `read_table` seeks to `rows_offset` with its
  own handle and streams batches. I/O is a 1 MiB buffer over `std::fs::File`, generic over `Read + Seek`
  inside the core (tests use `std::io::Cursor`).
- Consequences: reading everything costs two sequential passes (scan + decode); table readers are
  independent and `'static`, which `pyo3-arrow` needs; memory is bounded (4.1); a stop by [X1] still
  leaves the earlier tables readable.
- Rejected: a single-pass event stream "table, batch, batch, table" (one pass, but readers that borrow
  the walker, no access by table, and no row count before the end; can be added later on the same
  walker); `mmap` (no gain for sequential reads, `SIGBUS` on a file that shrinks, poor on network file
  systems); reading the whole file.

## ADR-5 Character sets: `encoding_rs` plus two decoders of our own - decided
- Context: five sets in the fixtures [X3]; three roles (6.1).
- Decision: `encoding_rs` for CL8MSWIN1251, WE8MSWIN1252 and AL16UTF16 (as UTF-16BE); `std::str::from_utf8`
  for AL32UTF8; an own strict decoder for US7ASCII. Strict decoding in row data (6.3). Unknown id: bytes
  and a warning [X3].
- Consequences: one small, widely used dependency that implements the WHATWG Encoding Standard; an
  override may name any of its labels.
- Session 2: R-026b is answered [C7]: Oracle maps the undefined bytes of 1251 (0x98) and 1252 (0x81,
  0x8D, 0x8F, 0x90, 0x9D) to the C1 control of the same number. The WHATWG indexes that `encoding_rs`
  implements give the same code points for these six pointers and have all 128 entries (6.2), so the
  crate is kept and is expected to match Oracle in all 256 bytes of both sets, with no decode error.
  Deciding test: T-9 compares all 768 lines of `truth_csmap.txt`. If a byte differs, that set gets an
  own 256-entry table built from `truth_csmap.txt`, with no change to the API.
- Rejected: crate `encoding` (unmaintained); own tables for 1251 / 1252 now (`truth_csmap.txt` would
  allow them, but nothing shows the crate differs; one dependency fewer is not worth 512 hand-made
  entries); ICU bindings (heavy, system dependency).

## ADR-6 NUMBER and TIMESTAMP map by the declared type, read from the `CREATE TABLE` column list - decided (session 2: grammar of [T9]); overflow policy in ADR-14
- Context: the descriptor has no precision or scale (exp.md 4.1). An ETL schema must be stable: the same
  table must give the same Arrow schema whatever its rows hold.
- Decision: parse the column list of the `CREATE TABLE` line by the grammar of [T9] (5.5) and map as in
  5.1. Agreement with the descriptors (count, names, type family, confirmed lengths) is checked; a line
  that does not parse or disagrees in count or names falls back for the table, a single unknown or
  disagreeing type text falls back for that column, both with `W-DDL-FALLBACK`. Conversions of declared
  columns are exact or a value error.
- Session 2 forms: `NUMBER(*,0)` (= INTEGER) gives **Decimal128(38, 0)**, not Int64: `*` is precision
  38 (`truth_decl.txt`), and Int64 would fail on 19-digit keys; `int64` by override. `NUMBER(*,s)`:
  Decimal128(38, s). `FLOAT(b)` including `FLOAT(126)` and `FLOAT(63)`: Float64 (nearest double, written
  in the field metadata). `TIMESTAMP (6)` (plain TIMESTAMP): Microsecond.
- Consequences: IDs declared `NUMBER(10)` arrive as Int64, amounts as Decimal128(p, s); the schema does not
  depend on the data; a type text the fixtures do not show costs one column its declared type, not the
  table.
- Rejected: one Arrow type for every NUMBER (Decimal128(38, 10) for an id column, or Float64 for money);
  choosing the type from the values seen in the scan (schema changes from dump to dump); Decimal256
  (little support in consumers); whole-table fallback for one unknown type text (session 1; too coarse
  now that the grammar is confirmed).
- Owner decision Q-1 (2026-10-08): NUMBER without precision is Decimal128(38, 10) with a per-column
  override; `number_default` may also be `float64` or `string`. Reopened for values that do not fit:
  ADR-14.
- Session 3 forms [T9]: `CLOB`, `NCLOB`, `BLOB`, `ROWID`, `UROWID(n)`, `INTERVAL YEAR (p) TO MONTH `
  (a space follows `MONTH` before `, ` or `)`), `INTERVAL DAY (p) TO SECOND (f)`. The interval
  precisions p and f exist **only** in this line (the descriptor has length 5 or 11 whatever the
  precision [T5, Y10]); f chooses the Arrow unit of INTERVAL DAY TO SECOND (ADR-19) and p, f are needed
  for the text form. On a fallback (line or column) an interval column takes f unknown: Duration
  (Microsecond), exact or value error, as TIMESTAMP does. Agreement lengths added (5.5): CLOB, NCLOB,
  BLOB 72; ROWID 23; `UROWID(n)` n; INTERVAL YEAR 5, INTERVAL DAY 11. A `NOT NULL ENABLE` after an
  interval type text is not in the fixtures (R-039): until then such a column falls back with
  `W-DDL-FALLBACK`.

## ADR-7 Dates and time zones - decided
- Decision: DATE as Timestamp(Second); TIMESTAMP by declared fraction (Microsecond up to 6 digits,
  Nanosecond for 7 to 9); TSTZ as the UTC instant with zone "UTC", the zone of the value is not delivered;
  TSLTZ as stored, without zone, with `W-TSLTZ`; years below 1 and nanosecond overflow are value errors
  (5.3).
- Context and consequences: the instant of a TSTZ value is confirmed for offset and region zones, the two
  zone bytes of a region are unknown [Y7]; whether TSLTZ is UTC is unknown [Y8, R-019]. Nothing unknown
  is turned into a claim in the schema. Losing the original offset of TSTZ is documented in the field
  metadata.
- Rejected: a struct column (instant, offset minutes) for TSTZ now (half of the zones cannot be filled);
  Date32 for DATE (loses the time of day); always Nanosecond (cannot hold years 1 to 1677, which
  `truth_ts.txt` contains).

## ADR-8 Row counts come from the scan only - decided
- Decision: `num_rows` is counted by walking the length fields; no cheaper estimate is offered; the
  catalog is kept in memory, not in a side file.
- Context: `METRICET` / `METRICEU` and statistics `num_rows` are confirmed **not** to be row counts
  [R6, chapter 10].
- Consequences: listing tables of a large file costs one sequential read; its speed is a benchmark
  figure (4.3).
- Rejected: an index file next to the dump (writes into directories of corporate dumps; stale-index
  bugs); "approximate rows" from statistics (wrong by rule).

## ADR-9 List K is a data file checked against the rules file - decided
- Decision: 2.4. Fallback G1 with a warning for a line the list lacks; `strict` turns it into an error.
- Consequences: a new entry from the Analyst is a one-line change that a test forces; the project owner
  can add entries at run time for a real dump.
- Rejected: `match` arms in code (the list would be edited by reading code, and could not be extended at
  run time); G1 alone without a list (G1 is "suggested" and has known failure cases [G2]).

## ADR-10 No threads inside the core in phase 1 - decided
- Decision and consequences: 4.6. Readers are `Send` and independent, so callers may parallelise by table.
- Rejected: a decoder thread pool now (the first measurement does not exist yet; a design for it is
  sketched in 4.6).

## ADR-11 Error policy: three classes, typed, with offsets, without values - decided, except the default of Q-2
- Decision: 7.1 to 7.4. Fatal framing errors stop the file; "outside the confirmed rules" is a warning
  that `strict` turns into an error; value errors follow `on_value_error` (default `raise`).
- Consequences: a damaged value never becomes a silent NULL unless asked for; a file is never
  resynchronised by searching (chapter 10 of the rules file); logs hold no data.
- Rejected: skip a damaged row and continue by default (silent data loss in ETL); a single `strict`
  switch that also covers value errors (the two questions are independent).
- Owner decision Q-2 (2026-10-08), now decided: non-strict by default, `strict=True` opt-in. Refinements:
  an unsupported type is always an error (ADR-12); warnings are returned in the result and printed by
  `survey`, never only logged; an introducing line read by G1 stays a warning and is counted in
  `Catalog.item_counts`.

## ADR-12 What stops the file: a type code outside [T5] and the LOB layouts of [X5] - decided (session 3: rewritten for rules v3.3)
- Context: rules v3.3 give 16 type codes with a known descriptor size [T5]: 1, 2, 8, 12, 23, 24, 69,
  96, 112, 113, 180, 181, 182, 183, 208, 231. For LOB tables the bytes between descriptors and rows (m,
  path flag, m column entries [T6, L1]) and the bytes after each row (k, then k values in a short or a
  long form [R4, L3, L4]) are confirmed (3 tables, 2 versions, 2 database sets, values up to 200,000
  bytes), so the end of a LOB table can be found exactly and every later table is reachable. For any
  other code the descriptor size is unknown [X1]: nothing after it can be located. [X5] lists the LOB
  layouts the fixtures do not show.
- Decision, three outcomes:
  1. **Stops the file** (`Catalog.end = Stopped(error)`, tables before it listed and readable; the error
     names the stopped table from its `TABLE` line, the offset, and the column position and type code
     where there is one):
     - `UnsupportedType` (`E-UNSUPPORTED-TYPE`) at a descriptor whose code is not among the 16 [X1]:
       BFILE (114), object, REF, collection and XML types, anything else;
     - `UnsupportedLayout` (`E-UNSUPPORTED-LAYOUT`, new) for the cases of [X5] and [T6]: m neither 0 nor
       the column count; m > 0 with path flag 1; a column entry [L1] with n > 0 for a column that is
       not 112 / 113, or n = 0 for one that is; a path flag other than 0 or 1.
  2. **Read** (values decoded, ADR-18 to ADR-20): CLOB, NCLOB (112), BLOB (113), INTERVAL YEAR TO MONTH
     (182), INTERVAL DAY TO SECOND (183), ROWID (69), UROWID (208); LONG, LONG RAW as before (ADR-15).
  3. **A LOB value the reader cannot decode** (the walk is unaffected; only the value is in question;
     ADR-18 and arch 5.6): a value error under `on_value_error` (raise, or NULL + `W-VALUE-NULLED`) for
     bytes that break the decoding (odd UTF-16 length, unpaired surrogate, US7ASCII high byte, a value
     above `max_lob_bytes`); a warning (strict: error) where the decoding rests on a rule that is only
     confirmed for part of the data: `W-LOB-CHARSET` for a CLOB of an AL32UTF8 database with a
     character above U+007F [L5, R-035], `W-LOB-UNTESTED` for a CLOB or NCLOB value in the long form in
     a single-byte database and for an NCLOB of a single-byte database [L5, R-038]; a LargeBinary column
     with `W-CHARSET-UNSUPPORTED` for a CLOB whose descriptor set is none of 1, 171, 178, 873 [L5, X3].
- The LOB framing checks are errors, not warnings (rules L3, L4 call them errors): k different from the
  number of LOB columns of the row that are not NULL; a chunk length of 0 or from 0xfff0; chunks whose
  sum passes the total T; in the LOB value area a u16 that is neither a length 0 to 0xffef nor `fc ff`.
  All are `Framing` at the offset (fatal, 7.1).
- Consequences (177 files, `tools/check_all.py` and `tools/corpus_counts.py` run read-only on
  2026-10-15: `problems 0`): **no file of the corpus stops**; all 177 walk to the second `EXIT`; all 334
  table sections (338 binary parts) are reachable; the 6 `ru_exp_*` owner-mode files give R_CLOB, R_TEXT
  and the Cyrillic-named table. The manifest grows to 338 parts (T-3b). In a real schema a BFILE,
  object or XML column still blocks every later table; a LOB, interval, ROWID or LONG column does not.
  Which codes the real tables hold is Q-4.
- Session 4 (rules v3.4, owner answers on T-3a): two more `UnsupportedLayout` cases, both stopping the
  file with the table named and the offset: (a) a **path flag other than 0 or 1** (was `Framing` in T-1;
  T-3a made it `UnsupportedLayout`, owner accepted, ADR-23); (b) on the **conventional** path a LONG /
  LONG RAW field whose u16 is 0x8000 to 0xffef: [R8] says "unknown (stop or warn), not a plain length"
  until R-040; the two readings R-040 names (one field of that length, or a piece chain) consume
  different byte counts, so reading on would misframe every later value: stop, message "conventional
  LONG length from 0x8000, R-040". 0xfff0 to 0xfffd stay `Framing` [R8]. With v3.4 the 199 files still
  contain no stop: all walk to the second `EXIT` (after T-3c; under T-3a `long_exp_direct_<v>` and
  `part2_exp_<v>` fail, N-10). Item 3 changes: `W-LOB-CHARSET` is retired (ADR-18, session 4);
  `W-LOB-UNTESTED` is replaced by ADR-25 (a value error by default for the unconfirmed single-byte
  cases).
- Rejected: keep stopping at 112 / 113 (contradicts confirmed rules and hides every later table);
  **skip** LOB tables (walk L1 to L4, deliver no values) as the alpha's behaviour: it would cost nearly
  the same code as reading (the walk is the hard part) and give the owner nothing for the columns most
  likely to matter; skipping stays available per column through `columns` (unselected LOB values are
  passed over by their lengths, never decoded or held); a single "stop on anything suggested" policy
  (one non-ASCII CLOB value would stop a nightly job on a rule that is confirmed in its framing).
- Amendment (session 24, ADR-95): the session 4 case (b) (a conventional LONG / LONG RAW u16 from 0x8000 to 0xffef stops
  the file, "R-040") is **replaced** by ADR-95. R-040 answered it: for LONG the u16 is a plain length up to 0xfff0 and
  `fd ff` starts a chunked value; `UnsupportedLayout` stays only for a conventional LONG u16 0xfff1 to 0xfffc and for a
  conventional LONG RAW u16 from 0x8000 to 0xfffd (R-048). Everything else in this record stands.

## ADR-13 Minimum Python and wheel form - decided (owner, 2026-10-16: Q-3 accepted)
- Session 5: the owner accepted the recommendation below: Python 3.11 or later, one `abi3-py311` wheel per
  platform, pyarrow 15 or later. T-12 and T-13 no longer wait for Q-3.
- Recommendation: Python 3.11 or later, one `abi3-py311` wheel per platform, pyarrow 15 or later.
- Rejected: per-version wheels (five builds per platform for no speed gain here); Python 3.10 (end of
  life this month, and `pyo3-arrow`'s buffer protocol feature needs `abi3-py311` or non-abi3 builds).
- Owner (2026-10-08): open; T-12 and T-13 wait for the Python version of the ETL hosts.
- Session 11: the platform tag of the first release (`manylinux_2_28`, Linux x86_64 only) is ADR-48.

## ADR-14 NUMBER values that do not fit the column type: `on_number_overflow` - decided (owner, 2026-10-16: Q-1 `round`)

Session 5 (owner decision of 2026-10-16, `owner-decisions.md`): the proposal below is **accepted**. A
column declared `NUMBER` is Decimal128(38, 10); `on_number_overflow` defaults to `round` (only below the
10th decimal, half away from zero); an integer part that does not fit is `E-NUMBER-OVERFLOW` under every
policy (ADR-27); every rounded value is counted per column (`W-NUMBER-ROUNDED`, with how many became 0)
and shown by the survey; `error` and `null` stay available per column and globally; `decimal256(p,s)` and
`string` are per-column or per-dump options, not the default. **New with the owner's decision: with
`strict=True`, `round` acts as `error`**: a value that would be rounded is `E-NUMBER-OVERFLOW` (value
class: `on_value_error` decides stop or NULL + `W-VALUE-NULLED`). `null` is an explicit choice and is not
changed by `strict`. This replaces "strictness does not change the number policy" of arch 7.1. Which
columns the policy covers: ADR-30 item 6.

Session 4 text (replaces the session 3 context and keeps its decision proposal; the session 3 text
follows as history).
- Context, rules v3.4: [Y1] (R-032, `t_num3_exp_<v>`, 18c and 11g byte-identical): a quotient or an
  average that does not terminate is stored with the **full 20 digit bytes**: 40 significant decimal
  digits, 39 when the first digit byte is below 10 (5/3). A column declared `NUMBER` therefore holds
  such values **as ordinary data**, not as a rarity. Values of 38 significant digits (integers and
  fractions) and the exponents +125 / -130 are in the fixtures too.
- Facts computed in session 4 (Analyst tools `num_decode.decode`, `exp_parse`, `collist`, run read-only;
  arch 5.2a has the table): of the **52** non-NULL truth values (`truth_num` 32, `truth_num2` 14,
  `truth_num3` 6), Decimal128(38, 10) holds 30 exactly; `round` changes 12 (3 of them to 0) and all six
  computed values of `t_num3`; 10 have an integer part above 28 digits (an error under every policy).
  Decimal256(76, 38) holds 42 exactly, `round` changes 7 (2 to 0; 4 of the 6 computed values lose digits
  39 and 40 only), 3 do not fit (|v| >= 10^38: 1E+125, 9.99...E+125, -1E+125). Utf8 text holds all 52.
  Over the 199 files, the 2,230 non-NULL fields of the 11 columns declared `NUMBER` give 2,102 / 48 / 80
  (exact / rounded / integer overflow) under Decimal128(38, 10) and 2,180 / 26 / 24 under
  Decimal256(76, 38) (the fixtures repeat the same tables; the truth counts are the meaningful ones).
- Consumers of Decimal256 (documentation read 2026-10-15): Parquet through Arrow C++ / pyarrow maps the
  Parquet DECIMAL logical type to and from Decimal32 / 64 / 128 / **256** (Arrow C++ "Reading and writing
  Parquet files", logical types table); Polars `Decimal` is a 128-bit type with precision at most 38
  (docs.pola.rs, `polars.datatypes.Decimal`); DuckDB `DECIMAL` has width at most 38 (DuckDB
  documentation, numeric types). **Not verified**: what Polars and DuckDB do when they import an Arrow
  Decimal256 column (error, cast to a float, or narrow), and what `pyarrow.Table.to_pandas` makes of a
  decimal column (expected: an object column of `decimal.Decimal`, for Decimal128 as well). T-12
  records all three on the pinned versions.
- Decision proposed (unchanged in substance, needs the owner): default type of `NUMBER` stays
  Decimal128(38, 10) (owner, 2026-10-08) and `on_number_overflow` defaults to **`round`**, with the
  guards below; **new, decided**: `decimal256(p,s)` (p up to 76, `arrow_schema::DECIMAL256_MAX_PRECISION`
  = 76, arrow-schema 60.0.0 on docs.rs) is accepted by `number_default` and `number_types`, so that a
  column the survey shows to hold long fractions can be read with digits 11 to 38 kept, where the
  consumer accepts Decimal256. Why not Decimal256 as the default: it fixes the type of **every**
  unconstrained NUMBER column, ids and amounts included, and two of the four named consumers have a
  native decimal of at most 38 digits, with import behaviour not verified; a type the consumer refuses
  stops the whole column, a rounded 11th decimal changes counted values. Why not `error`: every
  computed quotient stops the job. Why not text: every id and amount becomes a string.
- The survey (arch 5.2b) prints per column, counts only, the outcome under each candidate
  (Decimal128(38, 10), Decimal256(76, 38), text), so the owner decides on the real dump.

Session 3 text (history):
- Context: rule [Y1] shows that a column declared `NUMBER` holds values of up to 40 significant digits
  and exponents of +125 / -130; 14 of 32 values of `truth_num.txt` and 2 of 14 of `truth_num2.txt` do
  not fit Decimal128(38, 10). Session 3 (note A-6, [Y1] v3.3): the **40 digits rest on one value**,
  `t_num` row 31 (1/3 written as a 40-digit literal); row 30 (pi) and seven more have 38; no value has
  39. Whether a value **computed** by SQL (a quotient, an average) is stored with 40 digits is not in
  the fixtures (R-032). What the 16 non-fitting truth values show does not depend on it: 13 of them are
  integer overflow (10) or tiny values (1E-11, 1E-125, 1E-130); only rows 25, 30 (38 digits) and 31
  (40 digits) of `truth_num.txt` are long fractions, so even a 38-digit stored quotient does not fit
  Decimal128(38, 10). With "exact or an error" any of them stops the read.
- Decision proposed: option `on_number_overflow` = `round` | `error` | `null`, global and per column,
  for columns whose type is not fixed by a declared precision (`NUMBER`, fallback, overrides).
  Recommended default `round`: fractional digits beyond the scale are rounded half away from zero; an
  integer part that does not fit is a value error under every policy; every rounded value is counted
  per column (`W-NUMBER-ROUNDED`, with how many became 0); the policy is written into the field
  metadata. Full option table with costs: 5.2a.
- Consequences: a nightly job is not stopped by a quotient; values change below the 10th decimal and
  values under 5E-11 become 0, both counted and reported; CI and the truth tests run with `error`.
  The survey (5.2b) gives the owner the counts per column before the first run.
- Rejected: `error` as default (stops on ordinary data; the fix is known only after a failed run);
  `null` as default (loses whole values); `string` as an overflow policy (a column's Arrow type is fixed
  before its first row; it exists as a column type); a different default type (Float64 loses ids
  silently, Utf8 is text everywhere, Decimal256(76, 38) still does not hold 40 fractional digits or
  1E+125).

## ADR-15 LONG and LONG RAW are read by rule Y9 - decided
- Context: rules v3.2 add codes 8 and 24 to [T5] (4-byte descriptor, length 0) and [Y9]: each value is
  one ordinary field; confirmed up to 32,000 bytes on the conventional path in an AL32UTF8 database;
  unknown: values above 65,533 bytes, the direct path, the character set of non-ASCII LONG (R-031).
- Decision: LONG is Utf8, decoded with the database set of the header [H4] (the descriptor has no set
  id); LONG RAW is Binary. Warnings, strict turns them into errors: `W-LONG-CHARSET` (a byte above 0x7f
  in a LONG value), `W-LONG-LENGTH` (a value above 32,000 bytes), `W-LONG-DIRECT` (path flag 1). With a
  LONG first column on the direct path, the empty-table decision never guesses "empty" (5.4).
- Consequences: the two `long_exp` fixtures become readable; the walker's test that expected a stop is
  inverted (T-3a); a real table with a LONG column no longer blocks the file. A LONG above 65,519 bytes,
  if `exp` writes it in a form [Y9] does not know, will most likely end in `Framing` later in the table,
  and in the worst case misread values until then: the warning above 32,000 bytes marks where.
- Rejected: keep stopping at code 8 / 24 (contradicts the confirmed rule and blocks every later table);
  decode LONG with the client set (C4 forbids the client set for row data); LargeUtf8 / LargeBinary
  (a field is at most 65,519 bytes [R8], and `batch_bytes` bounds a batch).
- Session 3: [Y9] v3.3 now states the set (note A-8): the database set of [H4], never header bytes 1-2,
  status suggested; L5 (a CLOB of the single-byte database is database bytes with every client) is
  further support. Design unchanged: database set of the header, `W-LONG-CHARSET` on a byte above 0x7f.
  [Y9] also names a hypothesis for values above 65,533 bytes: the long form of [L4] (`fc ff`, 8-byte
  total, chunks). The reader does **not** read that form for LONG until R-031 confirms it: `fc ff` as a
  LONG length is 0xfffc, which [R8] makes an error in a column field (`Framing` at the offset, message
  "LONG value in an unconfirmed form, R-031"). If R-031 confirms it, LONG and LONG RAW become LargeUtf8
  / LargeBinary (a schema change, to be made before the beta, by a new record).
- Session 4 (rules v3.4; this paragraph is that record). Facts: `exp` writes LONG tables on the direct
  path (flag 1) and there the LONG / LONG RAW column stands **last** in `INSERT INTO` and the descriptors
  [T3, T9]; its value is a **piece chain**: u16 n, (n & 0x7fff) bytes, another piece while bit 0x8000
  is set; NULL `fe ff` [R2, Y9] (4 chains of 32,000 bytes = 156 + 4 x 7961, 6 one-piece values, LONG
  and LONG RAW, 2 versions). The character set is the database set, **confirmed** for AL32UTF8 with
  two client sets [Y9]. The long form of [L4] for LONG (the session 3 hypothesis) is not what the
  direct path writes; on the conventional path a length from 0x8000 is unknown (R-040).
- Decision:
  1. Walker (task T-3c): with path flag 1, a field of a type 8 / 24 column is read as a chain. First
     u16: `fe ff` NULL [R3]; 0, 0xfff0 to 0xfffd, `ff ff`: `Framing` [R8]; otherwise a piece. Later
     u16: a piece; a u16 from 0xfff0 to 0xffff (a piece of 0x7ff0 bytes or more with the flag, which
     would collide with the markers, exp.md 6.6 "unknown") is `Framing`; a piece of length 0 (0x8000 or a
     final 0x0000) is read by the formula with `W-UNTESTED-LAYOUT` once per column. The scan skips
     pieces by length and records per column the number of values, the largest total and the number
     above 32,000 bytes (for the survey). With path flag 0 a field is one ordinary field [Y9]; a u16
     from 0x8000 to 0xffef in a type 8 / 24 column is `UnsupportedLayout` (ADR-12, R-040).
  2. Types: LONG **LargeUtf8**, LONG RAW **LargeBinary** (64-bit offsets): a chain has no total and no
     bound in the format, like a LOB value (ADR-18). The value is bounded by `max_lob_bytes` (the
     option now covers LONG chains too; checked as pieces accumulate, value error `E-LOB-TOO-LARGE`
     at the piece that passes it; under `on_value_error="null"` the remaining pieces are passed over by
     length).
  3. Decoding: pieces are appended to the builder as they come; LONG text through one streaming
     decoder per value (AL32UTF8: `encoding_rs::UTF_8` decoder without replacement, since a piece can end
     inside a UTF-8 sequence; single-byte sets: stateless as 6.2).
  4. Warnings: `W-LONG-DIRECT` is **retired** once T-3c reads chains (its condition, "direct path LONG
     unconfirmed", is answered by [R2, Y9]); until T-3c the T-3a walker keeps raising it, as the owner
     accepted (ADR-23). `W-LONG-CHARSET` now only in a database whose set [H4] is not AL32UTF8 (873) and
     only for a value with a byte above 0x7f (single-byte database: "assumed the same" [Y9]).
     `W-LONG-LENGTH` stays for a value above 32,000 bytes on either path (largest confirmed [Y9]; a
     longer chain is the hypothesis of R-040 (c)).
  5. Column order: ADR-24.
- Consequences: the two `long_exp_direct_<v>` files read completely (T-3a stops them with
  `E-TRUNCATED`, N-10); a LONG column costs 4 more offset bytes per row; a real LONG table on the direct
  path reads whatever its value sizes; a conventional LONG value of 32,768 bytes or more stops the file
  at that table until R-040.
- Rejected: Utf8 / Binary with a value error above 2 GiB - 1 (as ADR-18); reading the conventional
  0x8000 to 0xffef lengths as plain lengths with a warning (if R-040 shows a chain, every later value
  of the file is misread until a `Framing`); a separate size option for LONG (one bound for every
  unbounded value is simpler to explain).
- Amendment (session 24, ADR-95): item 1's last sentence ("with path flag 0 ... a u16 from 0x8000 to 0xffef in a type 8 /
  24 column is `UnsupportedLayout`") and the session 3 paragraph's `fc ff` hypothesis are replaced by ADR-95 (R-040: one
  field up to 0xfff0, `fd ff` + chunks + `00 00` from 65,535 bytes, LONG only). Item 4's `W-LONG-LENGTH` threshold
  becomes 100,000 bytes for LONG (32,000 stays for LONG RAW). Items 2, 3 and 5 stand; the consequence "a conventional
  LONG value of 32,768 bytes or more stops the file" is withdrawn for LONG.

## ADR-16 US7ASCII high bytes: an error in row data, U+FFFD with a flag in metadata - decided
- Context: Oracle turns every byte above 0x7f of US7ASCII into U+FFFD [C7]; the dump keeps the bytes of
  the database [C1]. No row data of a US7ASCII database are in the fixtures.
- Decision: one rule, "row data never get a U+FFFD the file does not hold; metadata never stops the
  scan" (6.3). Row data: value error (`on_value_error`), message pointing to `db_charset`. Names, DDL and
  text items: U+FFFD, raw bytes kept, `name_possibly_damaged` / `ddl_lossy` set.
- Consequences: a US7ASCII database that holds bytes of another set (a common pass-through situation)
  is read correctly with `db_charset`, never silently damaged; the catalog of such a file is always
  available.
- Rejected: U+FFFD in row data like Oracle (destroys bytes the file holds, and a downstream job cannot
  see it); an error for metadata (one damaged comment would stop the scan).

## ADR-17 Header checks whose meaning is unknown are warnings - decided
- Context: N-5: the walker does not check the `00` after the file-name length [H4], the lines `0`, `72`,
  `0` [H3], or that the mode line agrees with `METRICST` / `METRICSU` [H6]. The values are constant in
  169 files; their meaning is unknown.
- Decision: T-3a adds the three checks; a different value is `W-UNTESTED-HEADER` with the offset
  (strict: error). Nothing else depends on them.
- Consequences: a real dump from a 10.2 client (R-022) that differs here is read and reported, not
  refused.
- Rejected: errors (a refusal for bytes whose meaning nobody knows); no check (a difference would go
  unseen, and it is the cheapest sign of an untested writer).
- Session 3 (note A-10 answered): [H6] confirms the pairing `RTABLES` - `METRICST`, `RUSERS` - `METRICSU`
  in all 177 files. A mismatch is therefore outside the confirmed rules and stays `W-UNTESTED-HEADER`
  (strict: error), as class 2 of 7.1 prescribes; the missing line itself stays `MissingMetricsLine`.
  The check is made in **T-3a** (not done yet: `implementer-notes.md` has no T-3a entry), together with
  the `00` after the file-name length and the lines `0`, `72`, `0`.

## ADR-18 LOB columns are read: CLOB, NCLOB as LargeUtf8, BLOB as LargeBinary, bounded by `max_lob_bytes` - decided
- Context: [L1 to L6]: the LOB data follow the row as k values, each short (`u16 n` + n bytes, 0 to
  6,000 seen) or long (`fc ff`, 8-byte LE total T, chunks `u16 c` + c bytes summing to T; 100,000 and
  200,000 seen; chunk sizes 16,120 / 8,060 on 18c, 16,264 / 8,132 on 11g in AL32UTF8 databases, 8,132 /
  16,264 / 8,132 for CLOB / NCLOB / BLOB in the 11g CL8MSWIN1251 database: observations, never to be
  assumed, ADR-37). NULL
  (`fe ff` in the column, not counted in k) and empty (locator, value of length 0) differ. Value bytes
  [L5]: BLOB the bytes; NCLOB UTF-16 BE; CLOB by the descriptor set: 873 (AL32UTF8 database) UTF-16 BE,
  confirmed for ASCII only; 171 (CL8MSWIN1251) the database bytes, confirmed for Cyrillic in the short
  form (6 and 6,000 bytes).
- Decision:
  1. Arrow types: CLOB and NCLOB **LargeUtf8**; BLOB **LargeBinary** (64-bit offsets, Arrow columnar
     format). A CLOB whose descriptor set is not supported (6.4): LargeBinary with
     `W-CHARSET-UNSUPPORTED`.
  2. Decoding by [L5] (arch 5.6): UTF-16 BE through one streaming `encoding_rs` decoder per value fed
     chunk by chunk (a chunk may end inside a code unit or a surrogate pair; a chunk is never decoded
     alone); single-byte sets as for VARCHAR2 (6.2); US7ASCII high bytes as ADR-16.
  3. Memory: a value is materialised whole in its builder (Arrow has no partial value). New option
     `max_lob_bytes` (default **256 MiB** of stored bytes, per value): a value whose T (long form) or n
     exceeds it is a value error `E-LOB-TOO-LARGE` before any byte is read; under
     `on_value_error="null"` its chunks are passed over by their lengths.
  4. `batch_bytes` stays a soft cap counted on output bytes: a row is never split, a value never
     split; the batch closes after the row that reaches the cap (arch 4.1, 5.6).
  5. Warnings and errors while statuses are suggested: ADR-12 item 3.
- Consequences: no per-value size limit below memory and `max_lob_bytes`; peak memory is one batch plus
  at most one row's LOB values plus the decoder scratch (arch 4.1); the schema of a LOB column does not
  depend on its values. LargeUtf8 offsets cost 4 more bytes per row than Utf8. LONG (Utf8) and CLOB
  (LargeUtf8) differ in offset width: documented in the type table.
- Rejected: Utf8 / Binary with a value error above 2 GiB - 1 (an Arrow array with 32-bit offsets holds
  at most 2^31 - 1 bytes in total, so a batch would also have to close by offset overflow, and the
  type would carry a limit the data do not have); `Utf8View` (arch 4.5: not in phase 1, and less widely
  consumed); a LOB as a struct (locator, bytes) (the locator is not needed [L2] and differs between two
  exports of the same data); streaming one LOB value in pieces across batches (no Arrow type for it;
  the consumer would have to reassemble).
- Session 4 (rules v3.4): [L5] now confirms non-ASCII and non-BMP CLOB and NCLOB text in AL32UTF8 as
  UTF-16 BE with surrogate pairs (`lob3`, `nchar2`, 2 versions), so **`W-LOB-CHARSET` is retired**.
  [L4] confirms the switch (long iff the byte length exceeds one chunk); the reader still accepts both
  forms at any length. `DBMS_LOB.GETLENGTH` counts UTF-16 code units [L5]: the survey reports stored
  bytes, never a character count, so no number of the reader is compared with `GETLENGTH`. Surrogate
  pairs across chunks: arch 5.6 step 6 (test vectors there). A long value with total T = 0: ADR-23.
  The single-byte database: **ADR-25** replaces `W-LOB-UNTESTED`.
- Session 6 (T-6, N-18): **verified** on the pinned arrow-array **60.0.0** (unit test in `arrow_out::tests`):
  `GenericStringBuilder` implements `std::fmt::Write` and `GenericBinaryBuilder` implements `std::io::Write`;
  written bytes accumulate into the next value; `append_value("")` / `append_value(b"")` ends that value;
  `append_null()` after a partial write ends it as null and keeps the partial bytes under the null slot
  (offsets `[0, 6, 7, 9]` in the test). Item 3 and arch 4.1 (e), 5.6 step 6 therefore hold as written. A
  change of the pinned arrow version re-runs that test before anything else.

## ADR-19 INTERVAL: YEAR TO MONTH as Interval(MonthDayNano), DAY TO SECOND as Duration - decided; `interval_as="string"` after the alpha (session 11: in the alpha, ADR-47)
- Context: [Y10]: 182 is 5 bytes (years, months), 183 is 11 bytes (days, hours, minutes, seconds,
  nanoseconds), all signed, confirmed for 3 values per column (positive, negative, zero); a negative
  value with non-zero lower fields and other precisions are R-036. Precisions only in the `CREATE TABLE`
  line [T9]. Arrow (columnar format) offers Interval(YearMonth) (i32 months), Interval(DayTime),
  Interval(MonthDayNano) (i32 months, i32 days, i64 nanoseconds) and Duration(unit) (i64).
- Decision:
  - INTERVAL YEAR (p) TO MONTH: **Interval(MonthDayNano)** with months = 12 x years + months, days 0,
    nanoseconds 0. Exact for |months| <= 2^31 - 1 (about 178.9 million years); beyond: value error
    `E-BAD-INTERVAL` (possible only with p >= 9).
  - INTERVAL DAY (p) TO SECOND (f): **Duration(Microsecond)** for f <= 6, **Duration(Nanosecond)** for
    f 7 to 9 (as TIMESTAMP, ADR-7); f unknown (fallback): Microsecond. Value = the signed sum of the five
    fields in the unit; exact or value error (a non-zero sub-microsecond part under Microsecond; i64
    overflow: Nanosecond covers about +-106,751 days, so every p <= 5; Microsecond about
    +-106,751,991 days, every p <= 8).
  - Checks (value error `E-BAD-INTERVAL`): length 5 / 11; months -11 to 11; hours -23 to 23; minutes and
    seconds -59 to 59; nanoseconds -999,999,999 to 999,999,999. Fields of different signs: not in the
    fixtures (R-036): the sum is still defined, so the value is delivered with `W-UNVERIFIED-TYPE` once
    per column (strict: error).
  - After the alpha: option `interval_as="string"` (whole dump) gives Utf8 in the `TO_CHAR` form of
    [Y10], with p and f from [T9] (fallback: p = the digits needed, f = 9, with `W-DDL-FALLBACK`).
- Costs: Interval(MonthDayNano) is the only Arrow interval type pyarrow exposes as a factory
  (`pyarrow.month_day_nano_interval`); pandas turns it into objects, not a native dtype; whether pyarrow
  writes it to Parquet is **not verified** (T-6 records the result; if it cannot, the owner's job uses
  `interval_as="string"` or casts). Duration maps to `timedelta64` in pandas and is widely consumed.
  The two interval kinds get different Arrow types: a year-month interval has no fixed length, so a
  Duration would be wrong for it.
- Rejected: Interval(YearMonth) for 182 (exact for the same range, but no pyarrow factory and weaker
  consumer support); Interval(MonthDayNano) for 183 too (exact for every precision, but pandas objects
  instead of timedelta, and a day-time interval does not need months); Utf8 text as the default (loses
  ordering and arithmetic; needs the DDL precision for every value); Interval(DayTime) (milliseconds:
  loses fractions).
- Session 4 (rules v3.4, `misc2_exp_<v>`, 2 versions). [Y10] confirms: **every field of a negative
  interval is negative** (zero fields may stand beside them: `80 00 00 00 3c 3c 3b 80 00 00 00` is 0 d,
  -1 s); descriptor lengths 5 / 11 for every precision; extremes +-999,999,999 years or days. Changes:
  1. Signs: a value with one field above 0 and another below 0 is no longer delivered with a warning;
     it breaks the confirmed form and is a value error `E-BAD-INTERVAL` ("opposite signs"). The
     `W-UNVERIFIED-TYPE` of session 3 for intervals is dropped.
  2. Range. Duration(Microsecond) holds +-106,751,991 days, Duration(Nanosecond) +-106,751 days (i64);
     a leading precision of 9 passes both (`INTERVAL DAY(9) TO SECOND(9)`: 999,999,999 days in the
     fixture). INTERVAL DAY (p) TO SECOND (f) is therefore: **Duration(Microsecond)** if f <= 6 and
     p <= 8; **Duration(Nanosecond)** if 7 <= f <= 9 and p <= 5; otherwise **Interval(MonthDayNano)**
     with months 0, days = the day field (|days| <= 999,999,999 < 2^31), nanoseconds = the signed sum of
     hours, minutes, seconds and nanoseconds (below 86,400 x 10^9 in magnitude, fits i64): exact for
     every value of every precision. The type follows the declared p and f, so the schema is stable;
     p, f unknown (fallback): Duration(Microsecond), exact or value error, as before.
     INTERVAL YEAR (p) TO MONTH stays Interval(MonthDayNano) with months only; it is exact for p <= 8
     (99,999,999 x 12 + 11 < 2^31); with p = 9 a value above 178,956,970 years is `E-BAD-INTERVAL` (no
     Arrow interval type has more than 32 bits of months; `misc2` rows 2 and 3 are such values). The
     lossless route is `interval_as="string"`; it stays after the alpha unless the survey shows p = 9.
  3. Text (`interval_as="string"`, test renderer of T-4): sign `-` when any field is negative, else `+`;
     then the absolute fields: years / days with p digits, `-MM` or ` HH:MI:SS`, and `.` + the first f of
     the 9 nanosecond digits only when f > 0 [Y10]; `7f ff ff ff 3a 39 38 62 32 9b 00` with p = f = 9 is
     `-000000001 02:03:04.500000000`; `80 00 00 00 3c 3c 3b 80 00 00 00` with p = 2, f = 0 is
     `-00 00:00:01`.
- Rejected (session 4): Interval(MonthDayNano) for every DAY TO SECOND column (exact, but pandas
  objects for the common p = 2); a value error for DAY(9) values beyond the Duration range (they are
  legal values that the fixture holds); Int64 months for YEAR(9) (an invented encoding).
- **Session 11 (T-12 finding 6, verified with pyarrow 26.0.0 and 15.0.2):** `pyarrow.parquet.write_table` refuses
  Interval(MonthDayNano) (`ArrowNotImplementedError: Unhandled type for Arrow to Parquet schema conversion:
  month_day_nano_interval`, `misc_exp_18c.dmp` T_INTERVAL column IYM); Duration(Microsecond) writes and reads back.
  So every YEAR TO MONTH column, and a DAY TO SECOND column with p > 8 (or f >= 7 and p > 5), cannot go to Parquet
  natively. **`interval_as="string"` moves into the alpha** (ADR-47, task T-13); the default stays `native`.

## ADR-20 ROWID and UROWID as Utf8 text - decided
- Context: [Y11]: a ROWID field is 23 bytes, 5 bytes (`01 00 00 00 17`, meaning unknown) + the
  18-character `ROWIDTOCHAR` text (confirmed, 1 value per version). A UROWID field is the `DUMP` bytes;
  first byte `01` = physical (object u32 BE, file u16 BE, block u32 BE, row u16 BE), whose base-64
  rendering equals the ROWID text of the same row (consistent, 2 values); other first bytes (logical
  UROWID) unknown.
- Decision: both **Utf8**. ROWID: the last 18 bytes, checked against the base-64 alphabet of [Y11];
  length other than 23 or a byte outside the alphabet: value error `E-BAD-ROWID`; the 5-byte prefix
  other than `01 00 00 00 17`: the text is delivered with `W-UNVERIFIED-TYPE` once per column (the rule
  says "report it"). UROWID: first byte `01` and length 13: the 18-character text of [Y11], with
  `W-UNVERIFIED-TYPE` once per column (the conversion is "consistent", not checked against a truth
  text); any other first byte or length: value error `E-BAD-ROWID` (logical UROWID, R-036).
- Consequences: an ETL job gets the same text Oracle prints for a ROWID; ROWID and physical UROWID of
  the same row compare equal as text. A UROWID column of an index-organised table fails per value until
  R-036 (with `on_value_error="null"`: NULL and a counted warning).
- Rejected: Binary (lossless but useless to a consumer; the ROWID field holds text, not the 10 `DUMP`
  bytes); FixedSizeBinary(10) built from the text (an invented encoding); delivering the logical UROWID
  bytes as hexadecimal text in the same column (two formats in one column).
- Session 4 (rules v3.4, `misc2_exp_<v>`): [Y11] confirms the ROWID prefix `01 00 00 00 17` in 8 values,
  the physical UROWID text in 6 values (= `CAST(urid AS VARCHAR2)`), and states the **logical** UROWID
  (first byte `02`): text = `*` + base 64 (`A-Z a-z 0-9 + /`, 3 bytes to 4 characters, most significant
  first) of the bytes after the first (`02 04 03 00 15 b3 02 c1 02 fe` = `*BAMAFbMCwQL+`; 1 value per
  version). Decision: physical UROWID (first byte `01`, 13 bytes) is delivered **without** warning;
  logical UROWID is **read**: first byte `02` and (length - 1) a multiple of 3 gives the text of [Y11]
  without warning; a tail that is not a multiple of 3 bytes (unknown [Y11]: no padding rule) or any
  other first byte is `E-BAD-ROWID`; the ROWID prefix other than `01 00 00 00 17` keeps
  `W-UNVERIFIED-TYPE` (the rule says "report it"). A UROWID column of an index-organised table no longer
  fails.

## ADR-21 Partitioned tables: one table, its partitions listed in `TableInfo`, rows of all parts in file order - decided
- Context: [T10]: one table section, one `CREATE TABLE` line, and per partition a plain line
  `PARTITION "p"`, an `INSERT INTO` line and a complete binary part (T4 to T6, rows, end); one
  `ENDPARTITION` after the last; same descriptors, own path flag per part. Confirmed on 1
  range-partitioned table, 3 partitions, 18c, both paths. Unknown: subpartitions, an empty partition,
  statistics (R-037); partitioned tables on 11g (11g XE has no partitioning, so the corpus cannot show
  them; the owner's database is 11.2.0.4).
- Decision:
  - The catalog has **one** `TableInfo` per section; `TableInfo.parts` lists the binary parts in file
    order, each with `name` (from the `PARTITION` line, client set [C2], raw bytes kept; None for an
    unpartitioned table, which has one part), path, `num_rows`, `rows_offset`, `rows_end_offset`,
    `end_marker`. `TableInfo.num_rows` is the **sum** of the parts. `TableInfo.partitioned` is true when
    a `PARTITION` line was seen.
  - `read_table` streams the parts in file order into one schema; batches never mix parts (a batch
    closes at the end of a part), so a consumer can map batches to parts.
  - Checks: every part's column count, descriptors, column entries and `INSERT INTO` names equal those
    of the first part, else `Framing` (Oracle partitions share their table's columns, so a difference
    means a misreading). A second `INSERT INTO` in a section with no `PARTITION` line before it:
    `Framing`. A binary part whose preceding line is not a `PARTITION` line (subpartitions may add their
    own lines, R-037): read, named after the last `PARTITION` line, `W-UNTESTED-LAYOUT` (strict: error).
    A `PARTITION` or `ENDPARTITION` line outside a section, or a section with `PARTITION` parts that
    ends [T2] without `ENDPARTITION`: `W-UNTESTED-LAYOUT`. Items between parts are skipped as in [T7].
  - After the alpha: `ReadOptions.partitions` (names) reads a subset of parts by seeking to their
    offsets.
- Consequences: a partitioned table is read like any other; `AmbiguousTableName` cannot arise from
  partitions; the empty-table decision on the direct path [E2] runs per part (an empty partition on 18c
  direct is R-037).
- Rejected: one `TableInfo` per partition (several entries with the same name; `ByName` would fail);
  a partition-name column in the batches (changes the schema of every partitioned table; a consumer can
  derive it from `parts` and the batch boundaries; can be an option later).
- Session 4 (rules v3.4 [T10], `part2_exp(_direct)_18c`; N-9, N-11, ADR-23). The record above stands;
  composite partitions and the walker's exact rules are added.
  1. **Lines.** Five plain lines, recognised in a section and never G1 candidates or list-K lines:
     `PARTITION "p"`, `COMPARTITION "p"`, `SUBPARTITION "s"`, `ENDCOMPARTITION`, `ENDPARTITION`. A
     **naming line** is `PARTITION "p"`, or `SUBPARTITION "s"` while a `COMPARTITION` is open.
  2. **Confirmed shapes** (no warning): simple: (`PARTITION`, part)+, items, `ENDPARTITION`; composite:
     (`COMPARTITION`, (`SUBPARTITION`, part)+, `ENDCOMPARTITION`)+, items, `ENDPARTITION`. "items" may be
     none (session 8) or the `ANALSTATS` items of the table and its partitions (session 9: 16 items,
     after the last `ENDCOMPARTITION`, before `ENDPARTITION`). Items are skipped wherever they stand
     [T7]; the reader never counts or expects them [S7].
  3. **Parts** (resolves N-11: arch 2.5 step 4 and this record say the same thing; T-3a's reading is
     the design): the first `INSERT INTO` of a section is always read. A further one is `Framing` if no
     naming line has been seen in the section (owner, accepted); if one has been seen but the line
     directly before the `INSERT INTO` is not a naming line, the part is read, named after the last
     naming line, with `W-UNTESTED-LAYOUT`. Every part's column count, descriptors, m, column entries
     and `INSERT INTO` names equal the first part's, else `Framing`.
  4. **Order of lines outside the confirmed shapes**: `W-UNTESTED-LAYOUT` (once per section, offset of
     the first), reading continues: `SUBPARTITION` with no open `COMPARTITION`; `COMPARTITION` while one
     is open; `ENDCOMPARTITION` with none open, or with no part since its `COMPARTITION`; `PARTITION` and
     `COMPARTITION` in one section; `ENDPARTITION` while a `COMPARTITION` is open; a section that has
     naming lines and ends [T2] without `ENDPARTITION`; any of the five lines outside a section. Status
     "provisional": only 2 tables, 18c, are behind the shapes (rules T10, R-030, R-037).
  5. **Empty parts**: [E1] or [E2] per part; the item starts of test (a) include all five lines (rules
     E2: `SUBPARTITION "` and `ENDCOMPARTITION` seen after the two empty 18c direct-path subpartitions).
  6. **Catalog**: `PartInfo.partition` (from `PARTITION` or the enclosing `COMPARTITION` line) and
     `PartInfo.subpartition` (from `SUBPARTITION`, else None), both in the client set [C2] with raw
     bytes kept; `TableInfo.partitioned` (a naming line seen) and `TableInfo.subpartitioned` (a
     `COMPARTITION` seen); `TableInfo.num_rows` = the sum of the parts (T_PART 3,000; T_PART2 100, with
     two parts of 0 rows). A composite partition has no part of its own [T10]; the catalog does not list
     partitions without parts. Statistics items give the catalog nothing but their count in
     `Catalog.item_counts` [S7, R6]; the `num_rows` inside them is never used.
  7. `ReadOptions.partitions` (after the alpha) matches a name against `partition` and `subpartition`.
  Consequences: `part2_exp(_direct)_18c` read completely (T-3a stops them with `Framing`, N-10); until
  T-3b, `TableInfo` describes the first part only (N-11).
- Session 5: item 3's "every part's `INSERT INTO` names equal the first part's" is restated by name in
  ADR-31 (same names and, per name, the same descriptor; the order may differ as ADR-24 allows). Item 4
  gains two cases (ADR-29 item 4): a section that ends with a `COMPARTITION` still open, and a section
  that has `COMPARTITION` lines but no naming line and ends without `ENDPARTITION`.

## ADR-22 Empty-table decision with a length-0 first column follows [E2a], with a warning - decided
- Context: arch 5.4 (session 2) never decided "empty" for a LONG / LONG RAW first column on the direct
  path: it stopped with `Framing` when test (a) of [E2] held. Rules v3.3 add [E2a] (suggested, note
  A-7): "no rows" when test (a) holds **and** the line from p + 1 to the next LF is a complete item of a
  known kind whose field, if list K requires one, is well-formed; otherwise read a row. Not in the
  fixtures (R-031); it may never arise if `exp` writes LONG tables on the conventional path in a
  `direct=y` file, as it does LOB tables [L6].
- Decision: default: follow [E2a]; the table already carries `W-LONG-DIRECT` (path flag 1 with a LONG
  column), and the decision "no rows" adds `W-UNTESTED-LAYOUT` with offset p. The known kinds are those
  of [E2a] plus `PARTITION "` and `ENDPARTITION` (an empty partition, R-037). `strict=True`: the
  session 2 rule (test (a) holds: `Framing` "ambiguous empty table"; fails: read rows).
- When it fails: a wrong "no rows" makes the item walker read row bytes as items, a wrong "read a row"
  makes the row walker read item text as a LONG value; both most likely end in `Framing` within the
  next few items or rows; a silent misreading needs a LONG value that is itself a complete item
  (rules E2a). The warning marks the place.
- Rejected: always `Framing` when (a) holds (session 2; refuses a plausible empty LONG table on every
  run); the version line (forbidden, [E3]).
- Session 4 (rules v3.4 [E2a], [T3]): on the direct path the LONG column stands last, so a first column
  of descriptor length 0 occurs only in a table whose **only** column is LONG / LONG RAW (not in the
  fixtures; R-041 asks for it). The empty two-column LONG table of `long_exp_direct_18c` is decided by
  [E2] (`0a 54` > 10). The decision above is unchanged; the known kinds now also hold `COMPARTITION "`,
  `SUBPARTITION "`, `ENDCOMPARTITION` (ADR-21). `W-LONG-DIRECT` no longer accompanies it (ADR-15); the
  "no rows" decision keeps `W-UNTESTED-LAYOUT`. `strict_e2a` is a placeholder until T-11 wires `strict`
  (owner, accepted, ADR-23).

## ADR-23 The T-3a choices put to the owner - decided (owner, 2026-10-15, through the session 4 prompt)
- Context: N-9 lists choices T-3a made where the design was silent or self-contradictory; the owner
  answered them before this session (accepted all but one).
- Decided, as built:
  1. A binary part not directly preceded by a naming line: `Framing` when no naming line was seen in
     the section, `W-UNTESTED-LAYOUT` otherwise. Provisional; extended to `SUBPARTITION` in ADR-21
     (session 4).
  2. A path flag other than 0 or 1 is `UnsupportedLayout` (stops the file, ADR-12).
  3. `W-LONG-DIRECT` is raised by the walker, per part with flag 1 and a LONG / LONG RAW column. Session
     4: retired by T-3c (ADR-15), because rules v3.4 confirm the direct-path form.
  4. `WalkOptions::strict_e2a` is a placeholder until T-11 connects it to `strict`.
- **Changed by the owner**: a long LOB value with total T = 0 (`fc ff` + 8 zero bytes, no chunk) is
  accepted silently **only if a fixture contains one**; otherwise it is read (it frames correctly: no
  chunk follows) with `W-UNTESTED-LAYOUT` at the `fc ff` (strict: error). The 177 files of the walker's
  expected values contain no such value (`tests/data/lob_expected.tsv`: the 8 long values have T = 100,000
  or 200,000); for the 199 files the Implementer answers question AQ-1 of `implementer-notes.md`. Until
  the answer, the warning form applies.
- Rejected: `Framing` for T = 0 (the bytes frame; nothing is misread), silent acceptance without
  evidence (the owner's point: the reader says when it reads beyond the fixtures).

## ADR-24 Column order: `INSERT INTO` order to read, `CREATE TABLE` order for the schema, matched by name - decided
- Context: [T3, T9] v3.4: the descriptors and the row fields follow the `INSERT INTO` line, and on the
  direct path a LONG / LONG RAW column stands last there while `CREATE TABLE` keeps the declared order
  (6 sections, 2 versions); elsewhere the two orders are equal. [R7] says the path changes nothing else.
- Decision:
  1. The walker and the row decoder use the `INSERT INTO` order (file position) only.
  2. The declared column list [T9] is matched to the `INSERT INTO` names **by name** (byte-equal,
     client set), not by position. Agreement (arch 5.5 step 3) is checked per matched pair. Each
     `INSERT INTO` name must match exactly one coldef and the counts must be equal, else the table falls
     back [W-DDL-FALLBACK]. An order that differs in any way other than "the one LONG / LONG RAW column
     moved last in a part with flag 1" is read the same way (by name) with `W-UNTESTED-LAYOUT` once per
     table.
  3. The **Arrow schema and `TableInfo.columns` follow the `CREATE TABLE` order**, so the conventional and
     the direct export of one table give the same schema; `ColumnInfo.position` = declared position,
     `ColumnInfo.file_position` = position in the part. On a table fallback (no usable `CREATE TABLE`
     list) both are the `INSERT INTO` order. `ReadOptions.columns` keeps schema order.
- Consequences: one schema per table whatever the path; batches are assembled from builders in schema
  order (a permutation of column arrays, no copy).
- Rejected: schema in `INSERT INTO` order (a job reading a conventional and a direct export of the same
  table would see two schemas); matching by position with a special case for LONG (breaks on any other
  reordering the fixtures do not show).

## ADR-25 CLOB / NCLOB of a single-byte database until R-038: a value error by default, opt-in decoding - decided (it narrows the owner's Q-2 default for one case; owner accepted, Q-7, 2026-10-16); session 8: item 2 lifted for database set 171
- **Session 8 (R-038 answered, rules v3.5).** Item 2 is lifted **for the database set 171 (CL8MSWIN1251)
  only**; the rest of the record stands for every other database set. Wording (the Analyst's, rules file
  "Changes in session 10", tightened): *a CLOB of a dump whose database set [H4] is 171 and whose
  descriptor set is 171 is decoded with set 171 in both forms, and an NCLOB of such a dump as UTF-16 BE in
  both forms, without `E-LOB-UNCONFIRMED` and without a warning.* What each part rests on:
  - CLOB, long form = the database bytes, T counts bytes: [L4, L5 v3.5]; `ru_lob2_exp_cp1251_11g.dmp`@0x29c4
    (`fc ff`, T = `20 4e 00 00 00 00 00 00` = 20,000 = GETLENGTH of `truth_lob_ru.txt` row 2, not 40,000),
    chunks @0x29ce, 0x4994, 0x695a (8,132 + 8,132 + 3,736), decoded with `cp1251` = the SQL text (`exp.md` 14.6
    "Session 10"); descriptor set 171 @0x24e3 (`ab 00`), header database set 171 @0x36 (`00 ab`).
  - CLOB, short form (already item 1): @0x274c, 66 bytes `c0 c1 .. a8 b8` [L5]; R_CLOB (session 4).
  - NCLOB short = UTF-16 BE: @0x2790, 132 bytes `04 10 04 11 ..` [L5 v3.5]; NCLOB long = UTF-16 BE: @0x77f4
    (T = 40,000), chunks @0x77fe, 0xb788, 0xf712 (16,264 + 16,264 + 7,472) [L4 v3.5].
  - BLOB (item 4, unchanged): @0x2816 and @0x11444 [L5].
  - The client set does not matter: `ru_lob2_exp_us7ascii_11g.dmp` has the same bytes at the same offsets
    from 0x2104 to 0x16282 except one byte per locator [L5 v3.5; `exp.md` 14.6].
  - The table after the LOB table reads unchanged: R_TEXT @0x16283 is byte-identical to
    `ru_tbl_exp_cp1251_11g.dmp`@0x90d [T8, L6].
- **Session 8, what stays refused.** Database sets 1 (US7ASCII) and 178 (WE8MSWIN1252), and any database set
  other than 873 and 171: item 2 stays as written (`E-LOB-UNCONFIRMED` by default, `unconfirmed_lob="decode"`
  with `W-LOB-UNTESTED`, `strict` as ADR-34 item 5). Reason: the fixtures hold no such database ([L5 v3.5]:
  "for 1 and 178 as database sets ... assumed"; `exp.md` 14.6 "Limit of confirmed in session 10"); the
  Analyst's alternative (a counted warning instead of the stop) is rejected because a wrong reading is
  still silent and Q-7 (owner) chose the stop for unconfirmed LOBs. Q-7 is unchanged; only the set of
  confirmed cases grows.
- **Session 8, how the reader decides "confirmed"** (one list in one place, T-6M):
  1. `charset.rs` holds `pub(crate) const LOB_CONFIRMED_DB_SETS: &[u16] = &[873, 171];` with a doc comment
     naming [L5 v3.5] and R-038. No other code names 171 or 873 for this decision.
  2. D = the database set id of the header [H4] (`DumpInfo.db_charset_id`). The `db_charset` override (T-9)
     chooses the **decoder** only; it never changes D for this decision, because the storage form is decided
     by the database that wrote the file, which the override does not change. (This amends T-9's "ADR-25
     follows the override", N-21: an override naming a multi-byte set must not lift a stop silently, and one
     naming 1251 on a US7ASCII header must not either.)
  3. NCLOB (type 112, form 2, set 2000): confirmed in both forms iff D is in the list.
  4. CLOB (type 112, form 1) with a decodable descriptor set S (873, 171, 178, 1): the short form is
     confirmed (item 1, as before); the long form is confirmed iff D is in the list **and S = D**. So a CLOB
     with S = 873 in a D = 171 header (a patched file only; `W-CHARSET-MISMATCH`, ADR-34 item 6) keeps its
     long form unconfirmed: no fixture shows that combination.
  5. CLOB of any other S: LargeBinary with `W-CHARSET-UNSUPPORTED`, never unconfirmed (ADR-34 item 6).
  6. Version: the list is not split by version. The 171 evidence is 11g (XE 11.2.0.2) only; 18c with a
     single-byte database is not in the fixtures [L4 v3.5]. The design already applied the 11g short-form
     evidence to 18c (item 1); every LOB encoding fact seen on both versions agreed (AL32UTF8, [L5]), and
     chunk sizes, the only fact that differs, are not used (ADR-37). This is an assumption, written into the
     doc comment of the list and into the README limits; Q-4 asks the owner's version.
  7. The message of `E-LOB-UNCONFIRMED` names D, the confirmed list and `unconfirmed_lob`, no longer R-038.
     The survey's count of refused values (ADR-25 item 5, T-11) uses the same predicate: 0 on all 201 fixtures.
- Rejected (session 8): lifting for every single-byte set (the evidence is one set); restricting the lift to
  11g headers (the short form is not restricted either; the result would refuse the long form and accept
  the short one on the same assumption); deciding by the effective set after `db_charset` (the override is
  a decoding choice, not evidence about storage); a counted warning instead of the stop for 1 / 178 (above).
- Session 5: the owner accepted this record as written (Q-7), and adds that the default of
  `on_value_error` for this case stops with a clear message rather than writing NULL (it is `raise`, the
  global default; the message names R-038 and `unconfirmed_lob="decode"`). R-038 is being generated.
- Context: [L5]: in the CL8MSWIN1251 database a CLOB is the database bytes, confirmed for **short**
  values only (6 and 6,000 bytes, 7 files); a long-form CLOB, any NCLOB and any BLOB of a single-byte
  database are not in the fixtures (R-038, which the owner is preparing). In AL32UTF8 both forms are
  UTF-16 BE. Neither reading can fail loudly: every byte is a valid CL8MSWIN1251 character [C7], and
  most byte pairs are valid UTF-16, so a wrong choice gives **plausible-looking wrong text** with no
  error. The owner's data are Cyrillic; this is the top value risk (open-questions (c)).
- Decision, for a dump whose database set [H4] is single-byte (1, 171, 178):
  1. CLOB, short form: decoded with the descriptor set (confirmed), no warning.
  2. CLOB in the long form, and NCLOB in either form: **value error `E-LOB-UNCONFIRMED`** by default
     (value class: `on_value_error="raise"` stops the read of that table with the column, row and offset;
     `"null"` gives NULL and a counted `W-VALUE-NULLED`). The walk is unaffected (the framing [L3, L4] is
     confirmed), so every other table and column reads.
  3. New option `unconfirmed_lob="error" | "decode"` (default `"error"`; dump-wide in the alpha, no
     per-column form). `"decode"` reads them by [L5] as stated (CLOB: the descriptor set; NCLOB: UTF-16 BE) with `W-LOB-UNTESTED`
     once per column, for an owner who wants to compare the output with the database.
     Session 7 (ADR-34 items 4, 5): `W-LOB-UNTESTED` is a counted warning (once per column and table,
     `count`); under `strict=True` the value is `E-LOB-UNCONFIRMED` again, so `strict` overrides `"decode"`.
  4. BLOB: the bytes, no warning (no character set; the framing is the confirmed one).
  5. The survey counts, per LOB column, the values that hit item 2 (short / long, by form), so the owner
     knows the exposure before a run.
  6. When R-038 confirms a reading, item 2 becomes the confirmed decoding and the error disappears; no
     API change.
- Reason: the same rule as ADR-16 ("row data never get text the file does not hold"): a value whose
  bytes have two plausible meanings is not decoded by a guess. The owner's Q-2 (warn by default) is
  about framing and metadata that fail loudly if wrong; here a wrong guess is silent, so the value
  class applies. A refused value costs a NULL or a stop with a precise message; a misread value costs
  wrong Cyrillic text in the warehouse.
- Rejected: decode by [L5] with a warning by default (session 3: silent wrong text if the long form is
  UTF-16); decode the long form as UTF-16 BE with a warning (the opposite guess, equally unproven, and
  inconsistent with the confirmed short form); LargeBinary for every CLOB of a single-byte database
  (changes the schema of confirmed short values); a content heuristic (for example "many 0x00 / 0x04
  bytes means UTF-16"; a guess presented as a rule).

## ADR-26 What is checked against the descriptor length [C8] - decided
- Context: [C8] v3.4: for types 1, 23 and 96 the descriptor length is a true maximum (no field of
  238,657 is longer); recommendation: warn on a longer value, and never check types 8 / 24 (descriptor
  0) or 112 / 113 (descriptor 72, the field is a locator). Equality is not expected (character
  semantics, NVARCHAR2, non-BMP NCHAR are shorter).
- Decision: `W-LONGER-THAN-DECLARED` (once per column, offset of the first, strict: error) for types 1,
  23 and 96 only; **RAW moves from value error to this warning** (arch 5.4 session 2 had a value error).
  The value is delivered as read; the length field alone decides how many bytes are read [R2]. Types
  with fixed value lengths keep their own value checks (DATE 7, TIMESTAMP 7 / 11, TSTZ 13, TSLTZ 11,
  NUMBER 1 to 21, INTERVAL 5 / 11, ROWID 23, UROWID by [Y11]); nothing else is compared with the
  descriptor. No check that CHAR / NCHAR values are padded to n (C5, C6: padding counts code points for
  `CHAR(n CHAR)` and UTF-16 code units for NCHAR; no rule needs it).
- Rejected: a value error for types 1 / 96 / 23 (the rule's recommendation is a warning; the bytes are
  intact); checking the declared n (the descriptor is the reliable maximum, [C6] cap included).

## ADR-27 The value layer's codes and checks (T-4, N-12) - decided (session 5)
- Context: T-4 built the pure decoders of arch 5.2 to 5.7 and made choices the design left open (N-12).
  Decoders return values or a code; table, column, row and offset are added by the caller (T-6).
- Decision, as built unless stated:
  1. **Value codes** (all value class, `on_value_error` applies; Python class `ValueDecodeError` for every
     one of them, `.code` tells them apart): `E-BAD-NUMBER` (bytes break [Y1]); **`E-NUMBER-OVERFLOW`**
     (new name: an integer part that does not fit the column type, under every policy, the carry of a
     rounding included, 99999.5 at `NUMBER(5,0)`; a fraction beyond the scale under `error`, and under
     `round` with `strict=True`, ADR-14); **`E-BAD-DATETIME`** (new name for what arch 5.3 / 5.4 called
     `ValueDecodeError`: wrong length, impossible fields, year below 1 (ADR-28), nanoseconds beyond 64 bits,
     TSTZ offset bytes out of bounds); `E-BAD-INTERVAL`; `E-BAD-ROWID`; `E-BAD-TEXT` (malformed UTF-8 or
     UTF-16, LONG included); `E-LOB-TOO-LARGE` (LOB value or LONG chain above `max_lob_bytes`);
     `E-LOB-UNCONFIRMED` (ADR-25).
  2. **`W-UNVERIFIED-TYPE` becomes a `WarningCode`** (`UnverifiedType`). The decoders stay pure and return a
     flag (`Rowid::prefix_known`, the `bool` of `decode_tsltz` for a 7-byte TSLTZ); the row decoder (T-6)
     turns a flag into one warning per column and table, with the offset, part, row and column of the first
     value; T-11 makes it an error under `strict`. Every value-layer warning follows this pattern, and T-6
     adds the variants it raises: `W-UNVERIFIED-TYPE`, `W-LONG-LENGTH`, `W-LONG-CHARSET`,
     `W-LONGER-THAN-DECLARED`, `W-NUMBER-ROUNDED`, `W-NUMBER-NULLED`, `W-VALUE-NULLED`, `W-LOB-UNTESTED`;
     `W-TSLTZ` depends on the type only and is raised by the type resolution during the scan (as
     `W-DDL-FALLBACK`), once per column.
  3. **TSTZ offset** [Y7]: an offset zone (bit 0x80 of the first zone byte clear) is accepted when hour -
     20 lies in -23..23 and minute - 60 in -59..59; otherwise `E-BAD-DATETIME`. The bound is wide on
     purpose: it catches a misread byte, it does not validate time zones; the offset is not delivered
     (ADR-7). A region zone: the instant is delivered, the two bytes are returned raw (`Zone::Region`) and
     not interpreted [Y7 unknown]. R-044 asks for offsets -12:00 and +14:00 to put two more vectors behind
     the formula.
  4. **Intervals**: years and days within -999,999,999..999,999,999 (the largest leading precision is 9;
     the extremes are in `misc2` [Y10]), besides the checks of ADR-19; outside: `E-BAD-INTERVAL`. The sign
     check covers all fields; a zero beside negative fields is allowed [Y10].
  5. **Logical UROWID with an empty tail** (`02` alone): `E-BAD-ROWID`. [Y11] gives the text rule for
     tails of 3k bytes seen with 9 bytes; a text `*` with nothing after it would be our invention.
  6. **DATE / TIMESTAMP century and year bytes** [Y2]: c = byte 0, y = byte 1.
     - c >= 100 and y >= 100: year = (c - 100) x 100 + (y - 100), with c - 100 and y - 100 at most 99;
       (100, 100) would be year 0: `E-BAD-DATETIME`.
     - 1 <= c <= 100 and 1 <= y <= 100, not both 100: year = -((100 - c) x 100 + (100 - y)) ([Y2]: "100 -
       century, 100 - year"; `63 64` = -100, `35 58` = -4712, `truth_date` rows 8, 9). A pair (100, y < 100)
       is a year of century 0 BC (-50 = `64 32`): this follows the formula but no fixture shows it (R-044).
     - anything else (one byte below 100 and the other above, a 0 byte): `E-BAD-DATETIME`.
     Time bytes are hour + 1, minute + 1, second + 1; 0 or a field beyond 23 / 59 / 59 is
     `E-BAD-DATETIME`. For a year below 1 the fields are decoded (the survey and the test renderer use
     them) and the day is checked against 1..31 only; the Arrow value is an error (ADR-28).
  7. `OraNumber` marks zero by `len = 0` instead of a separate flag (arch 5.2 step 3 allows either).
- Consequences: eight value codes under one Python class; a caller can filter them by `.code`. Every
  warning of a value carries a row and a column (ADR-29).
- Rejected: one generic value code (an ETL job could not tell a rounding policy stop from a damaged
  byte); warnings raised inside the decoders (they would need table, row and a sink, and lose their
  purity); Oracle's documented offset range -12:00..+14:00 as the TSTZ bound (not in the rules, and a
  too-narrow bound refuses legal data).

## ADR-28 Values the Arrow type cannot hold: INTERVAL YEAR(9) beyond 32-bit months, years below 1 - decided (session 5)
- (a) **INTERVAL YEAR (9) TO MONTH.** Context: the Arrow columnar format has three interval types, all
  with at most 32 bits of months (YearMonth: i32 months; DayTime: i32 days, i32 milliseconds;
  MonthDayNano: i32 months, i32 days, i64 nanoseconds). |months| <= 2^31 - 1 = 2,147,483,647 is 178,956,970
  years 7 months. `INTERVAL YEAR(9)` allows 999,999,999 years 11 months = 11,999,999,999 months [Y10]. Four
  values of the fixtures exceed it: `misc2` rows 2 and 3 in both versions (`bb 9a c9 ff 47` =
  +999999999-11, `44 65 36 01 31` = -999999999-11, `truth_misc2.txt`).
  Decision: **keep the error.** INTERVAL YEAR (p) TO MONTH stays Interval(MonthDayNano) for every p; a
  value beyond +-(2^31 - 1) months is `E-BAD-INTERVAL` (value class). The survey counts, per YEAR TO MONTH
  column, the declared p and the values beyond the limit. `interval_as="string"` (the lossless text of
  [Y10], renderer built in T-4) stays after the alpha; it moves into the alpha if the owner's survey shows
  any such value.
  Consequences: the 4 fixture values are listed as expected errors in T-10 (run once with `raise`: the
  read of that table stops at row 2; once with `null`: 4 NULLs and `W-VALUE-NULLED`). Values of 178 million
  years are not plausible ETL data; ordinary YEAR(9) columns read exactly.
  Rejected: Int64 months (an integer column whose meaning is in metadata only: an invented encoding, ADR-19
  session 4); Utf8 for every column declared with p = 9 (the schema of every value changes because the
  type permits extremes); a struct (years Int64, months Int8) or a decimal (no consumer reads either as an
  interval); years carried in the days field of MonthDayNano (a year has no fixed number of days).
- (b) **Years below 1** (DATE, TIMESTAMP, TSTZ, TSLTZ). Context: [Y2] decodes them (`truth_date` rows 8, 9:
  -0100-06-15, -4712-01-01), and an Arrow Timestamp is an i64 count of units since 1970-01-01 (Arrow
  columnar format), so Timestamp(Second) could hold the number. What no rule gives is the **mapping**:
  arch 5.3 carries Oracle's printed fields into the proleptic Gregorian calendar so that "what Oracle
  prints is what Arrow prints"; for a year below 1 that needs to know whether Oracle's year -1 directly
  precedes year 1 (no year 0) or not, and the two readings differ by one year for every such date (R-044).
  On the Python side, `datetime.datetime` holds years 1 to 9999 only (`datetime.MINYEAR` = 1, Python
  standard library), so `to_pylist()`, `as_py()` and `to_pandas(timestamp_as_object=True)` could not
  return such a value anyway.
  Decision: **keep the error**: a year below 1 is `E-BAD-DATETIME` for DATE, TIMESTAMP, TSTZ and TSLTZ
  (`on_value_error`: stop, or NULL + `W-VALUE-NULLED`). The survey counts such values per column. R-044
  (low priority) asks for the year numbering; even with an answer the value would come only as an opt-in,
  because of the Python limit above. Dates from year 1 to 1582 are delivered by their printed fields
  (unchanged, arch 5.3); whether Oracle computes them in another calendar is not stated by the rules
  (R-044 asks); the README says that such dates are "as printed", not a claim about the instant.
  Consequences: `t_date_exp*` give two expected `E-BAD-DATETIME` per file (rows 8, 9) in T-10, read once
  with `raise` and once with `null`.
  **What T-12 checks** (pinned pyarrow and pandas; results recorded under this record; README states the
  versions): (1) `pyarrow.Table.to_pandas()` of a Timestamp(Second) column with `truth_date` rows 4 and 5
  (0001-01-01 00:00:00, 9999-12-31 23:59:59): the pyarrow documentation says timestamps become datetime64
  and calls `coerce_temporal_nanoseconds` (default False) "a legacy option to coerce ... timestamp time
  units to nanoseconds"; record whether the column arrives as `datetime64[s]` with both values intact, and
  from which pandas version; (2) the same with `coerce_temporal_nanoseconds=True`: record the error or the
  values (both lie outside the nanosecond range, about 1677-09-21 to 2262-04-11); (3)
  `timestamp_as_object=True` and `to_pylist()`: `datetime.datetime` for both; (4) the Microsecond and
  Nanosecond TIMESTAMP columns of `truth_ts.txt` row 5 (9999-12-31) the same way; (5) a value NULLed under
  `on_value_error="null"` arrives as `NaT` / `None`; (6) ADR-19's Parquet check of Interval(MonthDayNano).
  **Results (session 11, T-12 note; asserted by `tests/test_consumers.py` for pyarrow 26.0.0 / pandas 3.0.6, also
  seen with pyarrow 15.0.2 / pandas 2.2.3):** (1) `to_pandas()` gives dtype `datetime64[s]` with
  `0001-01-01 00:00:00` and `9999-12-31 23:59:59` intact, on pandas 2.2.3 and 3.0.6; (2)
  `coerce_temporal_nanoseconds=True` raises `ArrowInvalid` ("would result in out of bounds timestamp"), also for the
  Microsecond column; (3) `timestamp_as_object=True` and `to_pylist()` give `datetime.datetime(1, 1, 1)` and
  `datetime.datetime(9999, 12, 31, 23, 59, 59)`; (4) `truth_ts` row 5: TS0 and TS6 arrive as `datetime64[us]`
  (`...59` and `...59.999999`); TS9 (Nanosecond) is outside the range and is `E-BAD-DATETIME`, NULL under `null`
  (as in T-10); (5) a NULLed value is `NaT` in pandas and `None` in `to_pylist()`; (6) refused, see ADR-19 session 11.
  The README states these, with the versions (T-13). Decided: no change to the types; the README warns against
  `coerce_temporal_nanoseconds=True`.
  Rejected: BC values with an assumed year numbering (a claim no rule makes, and unrepresentable in Python
  `datetime`); NULL by default (silent loss); Utf8 for DATE columns (every date becomes text to keep two
  fixture rows).

## ADR-29 The walker's choices of T-3c (N-15) - decided (session 5)
- Context: T-3c built the walker of rules v3.4; all 199 files walk with no stop and no warning. N-15
  lists the choices the design did not fix.
- Decision:
  1. **`Warning` (and the error type) carry `column` and `row`**, and T-3b adds **`part`**. Convention, the
     same for errors and warnings: `table` = the 1-based ordinal of the section; `part` = the 1-based index
     of the binary part in the table (1 for an unpartitioned table), set whenever `row` is; `row` = 1-based
     within the part; `column` = the 1-based **file position** (order of the part's `INSERT INTO` line),
     never the declared position (the walker does not know it; ADR-31 makes file positions comparable
     across parts by name). DDL warnings of a matched column use its file position too. Python's
     `OradumpError` and `DumpWarning` expose `.table` (name), `.part` (partition / subpartition names or
     None), `.row`, and `.column` as the **column name** resolved through the catalog.
  2. `WarningCode::LongDirect` is removed (ADR-15): accepted.
  3. **Chain start** (ADR-15 item 1): a first u16 of `00 80` (0x8000) is a piece of length 0 with more to
     follow, read with `W-UNTESTED-LAYOUT`; this is the intended reading of "otherwise a piece". A first u16
     of 0, 0xfff0 to 0xfffd or `ff ff` stays `Framing` [R8]; a later u16 from 0xfff0 stays `Framing`.
  4. **Partition lines**, two more `W-UNTESTED-LAYOUT` cases (ADR-21 item 4): a section that ends [T2] with
     a `COMPARTITION` still open; a section with `COMPARTITION` lines (no naming line) that ends without
     `ENDPARTITION`. A `SUBPARTITION` outside a `COMPARTITION` is a warning case already and does not name
     the part. Accepted: nothing outside the confirmed shapes passes silently.
  5. **Frequency**: a piece of length 0 once per LONG column and section (offset, row, column of the
     first); a long LOB value of total 0 once per section; a part not directly after a naming line once per
     part. Accepted.
  6. **Raw hash of a chain**: the serialisation built in T-3c (`fd ff ff ff`, per piece u32 LE length
     without bit 0x8000 and the bytes, `fc ff ff ff`) is **final** for `exp_raw.tsv`: the raw manifest
     records the bytes as they stand, piece boundaries included. T-3b adds, per LONG / LONG RAW column of a
     part, a second digest over the **assembled** values (NULL = 8 bytes `ff`; otherwise the total as u64
     LE and the bytes), equal for the conventional and the direct export of the same table; T-6 compares
     its LONG RAW output, and its LONG output in an AL32UTF8 database, with that digest.
  7. The new warnings (piece of length 0, T = 0, partition order, [E2a]) do **not** respond to `strict`
     until T-11 wires it (as `strict_e2a`, ADR-23).
  8. AQ-1 answered: no long LOB value with T = 0 in the 199 files (walker and tool agree). ADR-23's warning
     stands as built.
  9. Plan T-3c (c): the six parts of `part2` compared as a set of (partition, subpartition) pairs, each with
     the rows of its subpartition in `truth_part2.txt` (`SP` lines are sorted by name, the file is in
     partition order): accepted.
- Rejected: `Framing` for a first piece `00 80` (the bytes frame, ADR-15 reads a zero piece elsewhere);
  the declared position in warnings (unknown to the walker; two numbering schemes in one field); hashing
  only the assembled value in the raw manifest (piece boundaries would go untested).

## ADR-30 The declared-type choices of T-5 (N-16) - decided (session 5)
- Context: T-5 built the [T9] parser, the by-name matching (ADR-24) and the mapping of arch 5.1. N-16
  lists six choices.
- Decision:
  1. **Where an unknown type text ends**: at the first `, "` or the first `)  ` (closing parenthesis, two
     spaces), whichever comes first; a trailing ` NOT NULL ENABLE` is removed and sets `not_null`. Safe
     because of the agreement check: every `INSERT INTO` name must match exactly one definition and the
     counts must be equal (ADR-24), so a wrong cut either keeps the exact text or makes the table fall back
     (`W-DDL-FALLBACK`); it cannot give one column another column's declared type, and values never depend
     on it (they are decoded from the descriptors). Forms a real dump may hold:

     | form | what happens |
     |---|---|
     | `TIMESTAMP (6) WITH LOCAL TIME ZONE`, `INTERVAL YEAR (p) TO MONTH `, `INTERVAL DAY (p) TO SECOND (f)` as [T9] writes them | known forms, no cut |
     | the same with another spacing (`TIMESTAMP(6) ...`), or an interval followed by ` NOT NULL ENABLE` (R-039) | unknown text, ends correctly at `, "` / `)  `; column fallback: TSLTZ / TIMESTAMP unit Microsecond, INTERVAL DAY Duration(Microsecond), INTERVAL YEAR Interval(MonthDayNano); `not_null` kept |
     | `XMLTYPE`, an object type `"OWNER"."TYPE"`, VARRAY, nested table, `REF`, `BFILE`, `SDO_GEOMETRY` | the descriptor code is outside [T5]: the file stops at that descriptor (ADR-12) whatever the cut; quoted names contain `"."`, not `, "` |
     | a text with an expression (a virtual column, if `exp` writes one; [T9] says DEFAULT never stands in the list) | may contain `, "` (`CONCAT("A", "B")`) or `)  `: the definition count changes, table fallback with `W-DDL-FALLBACK` |
     | an encryption clause (`ENCRYPT USING ...`) | out of scope (no encryption); if met: unknown text, column fallback |

     Test vectors (T-5 follow-up): `"X" FOO(3))  ` + storage keeps `FOO(3)`; `"X" FOO(3), "Y" DATE)  `
     keeps `FOO(3)` and parses Y; `"X" F("A", "B"), "Y" DATE)  ` gives the table fallback. (N-16 says the
     first keeps `FOO(3`; by the code it keeps `FOO(3)`; the test settles it.)
  2. **Out-of-range parameters are a column fallback with `W-DDL-FALLBACK`**: confirmed. `NUMBER(p, s)` with
     p outside 1..38 or s outside -84..127; `FLOAT(b)` outside 1..126; `TIMESTAMP (f)` and interval
     precisions outside 0..9. Such a text cannot be what Oracle declared, so it is treated as unknown.
  3. **A declared NUMBER that no Decimal128 holds warns and takes `number_default`** (changes the build):
     `NUMBER(p, s)` with s > 38; `NUMBER(p, s)` with s < 0 and p - s > 38; `NUMBER(*,s)` with s > 38 or s < 0
     (`*` is precision 38, so `NUMBER(*,-2)` needs 40 digits; this replaces the session 2 row "s < 0:
     Decimal128(38, 0)"). The column gets the `number_default` type (Decimal128(38, 10) unless set),
     `on_number_overflow` applies, `declared` keeps p and s, and `W-DDL-FALLBACK` (table, column, reason "no
     Decimal128 holds NUMBER(p,s)") is raised in the scan. A `number_types` override (for example
     `decimal256(76,50)` for `NUMBER(3,50)`) reads it exactly. New row in arch 5.1.
  4. **Form check**: VARCHAR2, CHAR, CLOB need descriptor form 1; NVARCHAR2, NCHAR, NCLOB form 2:
     confirmed. The form's meaning is "suggested" [T5], but a disagreement only costs a column fallback.
     TSTZ length 13 and TSLTZ 7 / 11 (`tools/collist.py`; every fixture agrees): accepted on the same
     ground.
  5. **Order warning**: `W-UNTESTED-LAYOUT` once per table unless the file order is the declared order or,
     in a part with path flag 1, the declared order with the LONG / LONG RAW column moved last [T3]: confirmed.
     A conventional part with the LONG column moved last therefore warns ([T3] shows the move only on the
     direct path). Two equal `INSERT INTO` names, or a name that matches no or two definitions: table
     fallback. Per part: ADR-31.
  6. **`number_types` against `number_default`**: confirmed, this is what Q-1 and ADR-14 mean.
     `number_types` is the caller's explicit choice for a named column and wins over every type-2 column:
     plain `NUMBER`, declared `NUMBER(p, s)`, INTEGER (`NUMBER(*,0)`) and FLOAT. `number_default` is the type
     of the type-2 columns whose type no usable declaration fixes: plain `NUMBER`, the fallbacks, and item 3.
     `on_number_overflow` (global, or the policy given with the `number_types` entry) applies to the columns
     under `number_default` and under `number_types`; a declared column without override stays "exact or
     value error" (`E-NUMBER-OVERFLOW`).
  7. The CLOB set check in the mapping (form 1, set not 873 / 171 / 178 / 1: LargeBinary +
     `W-CHARSET-UNSUPPORTED`; NCLOB LargeUtf8 whatever the id) and the warnings of the resolution in
     `Dump::warnings()` during the scan: confirmed.
- Rejected: item 1, a full expression parser (no rule describes the texts it would parse); item 3, "as
  `NUMBER` without a warning" (as built: a declaration silently not honoured), the nearest Decimal128
  ((38, 38) or (38, 0); keeps more digits, but a third mapping rule for columns no fixture has), automatic
  Decimal256(s, s) (a type the owner did not choose, ADR-14); item 6, `number_default` also overriding
  declared columns (it would turn every `NUMBER(10)` id into the default type).

## ADR-31 Partitioned tables: by-name matching and checks per part - decided (session 5)
- Context: T-5 resolves the types on the first part (N-16). ADR-21 item 3 requires every part's
  descriptors and `INSERT INTO` names to equal the first part's; ADR-24 matches by name. Each part has its
  own path flag [T10], and on the direct path the LONG / LONG RAW column moves last [T3], so two parts of
  one table may differ in order (not in the fixtures: `T_PART`, `T_PART2` have no LONG column).
- Decision:
  1. T-3b runs the by-name matching of ADR-24 **for every part**: each `PartInfo` gets `file_order` (for
     each file position the declared position, or on a table fallback the position in the first part's
     `INSERT INTO` order).
  2. ADR-21 item 3 by name: every part has the same column names (each once) and, per name, the same
     descriptor (code, length, set id, form) and the same LOB column entry presence as the first part, and
     the same column count; otherwise `Framing` at that part. An order that differs from the first part's
     follows the order rule of ADR-30 item 5 per part (no warning for the LONG-last move on a flag-1 part,
     `W-UNTESTED-LAYOUT` once per table otherwise).
  3. Types are resolved once per table (declaration + first part's descriptors); item 2 makes them valid
     for every part. `W-DDL-FALLBACK` is raised once per table, not per part.
  4. T-6 decodes each part's fields in its file order and places them by that part's `file_order`; the
     schema is the table's (declared order).
- Consequences: a conventional and a direct part of one table read into one schema; a part that really
  differs still stops with `Framing`.
- Rejected: resolving and ordering on the first part only (a later part in another order would put
  values in the wrong columns, silently when the types agree); one schema per part (breaks "one table,
  one schema", ADR-21).

## ADR-32 The catalog choices of T-3b (N-17) - decided (session 6)
- Context: T-3b built `TableInfo.parts`, the per-part matching of ADR-31 and the golden files of 199 files
  (396 part lines, 382 table lines; 158 tests). N-17 lists the choices the design did not fix.
- Decision:
  1. **LOB digest** (`RowsSummary.lob_sha256`, column `lob_sha256` of `exp_raw.tsv`), **final**: one SHA-256
     per binary part over the LOB values of the part in file order (rows in order; within a row the value
     order of [L3], which is column order); per value its **stored length** (h of the short form, T of the
     long form [L4]) as u64 little-endian, then the stored bytes, chunks joined, chunk length fields
     excluded. A NULL LOB has no value in the value area [L3, L4] and adds nothing; an empty LOB adds 8 zero
     bytes. Chunk boundaries are not hashed: they are checked by the framing ([L4]: sum = T, `Framing`
     otherwise) and they differ by version for the same data (16,120 / 8,060 on 18c, 16,264 / 8,132 on
     11g; observations, ADR-37), so equal data give equal digests across versions and paths.
  2. **Assembled-LONG digest** (`RowsSummary.long_sha256`): as ADR-29 item 6, confirmed as built: per LONG /
     LONG RAW column of a part, each value hashed whole after its pieces are joined (NULL = 8 bytes `ff`;
     otherwise the total as u64 LE, then the bytes). The second pass over a chain (seek back to the first
     piece, hash, seek forward) is accepted because it happens only in hashing mode (tests and golden
     files); the reader never seeks back.
  3. **`Dump::rows` on a table of more than one part is an error**, not the first part. `Dump::rows(which)`
     returns the cursor when `TableInfo.parts` has exactly one element (every unpartitioned table, and a
     partitioned table with one part); otherwise `E-CONFIG` ("the table has n parts; use `part_rows` or
     `read_table`", `length` = n, `table` set). `Dump::part_rows(which, index)` with an index outside the
     parts is `E-CONFIG` too (it is `E-NO-SUCH-TABLE` as built: the table exists). Both are Rust-level
     diagnostic cursors (they skip values, decode nothing); the reading API is `read_table`, which already
     reads every part in file order (ADR-21); the Python binding exposes neither cursor.
  4. **Order of a later part**: a later part whose file order differs from the first part's (other than the
     LONG-last move on a flag-1 part, ADR-30 item 5) gives `W-UNTESTED-LAYOUT` **once per table**, at the
     first occurrence (first part: at the `CREATE TABLE` line with `part` 1; a later part: at its `INSERT
     INTO` line with its `part`); values are placed by name, so they stay right. Confirmed as built. Oracle
     cannot create a partitioned table with a LONG column (ORA-14085 "partitioned table cannot have column
     with LONG datatype", Oracle Database error documentation, docs.oracle.com/en/error-help/db/ora-14085);
     for LONG RAW only an Oracle9i Concepts sentence is known, quoted in a search result and **not read**
     in this session ("Any table can be partitioned except those tables containing columns with LONG or
     LONG RAW datatypes"); R-045 observes both on 18c. So the LONG-last case of a
     later part cannot come from Oracle: the per-part exemption stays (it costs nothing and is unit-tested),
     and any order difference between parts is outside every observed layout, hence the warning.
  5. **The fixture gap**: no partitioned table with a LONG column can exist (item 4); R-045 asks the owner
     to record ORA-14085 on 18c with R-042's run (low priority; turns a documentation fact into an observed
     one). A partitioned table with LOB columns is **R-042** (session 4): priority stays **medium**, becomes
     **high** if Q-4 shows partitioned tables with CLOB / NCLOB / BLOB columns; T-6L tests the combination
     on a synthetic two-part section meanwhile (as T-3b did for `ColumnInfo.lob`).
  6. Accepted as built: the per-part check in the walker (same count, same m, the sorted set of (name, type
     code, length, set id, form, LOB entry name and tail); the path flag is **not** compared, [T10]); equal
     names in the first part (a table fallback) give later parts the identity order; `part` on every error
     and warning raised while a part is read, none on section-level ones (except the second `INSERT INTO`
     without a naming line, which names the part it would have been); `column` of the DDL warnings = file
     position in the first part; `TableInfo.path` = `TablePath { Conventional, Direct, Mixed }`;
     `PartInfo.partition` / `subpartition` as `Option<String>` (ASCII) with `*_raw` (client-set decoding and
     a damaged flag are T-9's, arch 6.5); `ColumnInfo.long` summed over the parts through each part's
     `file_order`; the log check over 199 logs (leaves of a partitioned table equal the parts in name,
     order and rows).
- Consequences: a caller who uses `Dump::rows` on `T_PART` gets an error naming 3 parts instead of 1,095 of
  3,000 rows; tests that walk every part use `part_rows`. The golden files are stable across versions for
  LOB data.
- Rejected: `rows` reads the first part (silent loss: 1,095 of 3,000 rows on `T_PART`); `rows` crosses all
  parts (the cursor's column order and end marker are per part, so a crossing cursor would change its
  column order in mid-stream; `read_table` already gives all parts in one schema); chunk lengths in the LOB
  digest (would make 18c and 11g digests of equal data differ, and boundaries are framing-checked);
  one LOB digest per column (one per part suffices for a golden file; T-6L compares values per column).

## ADR-33 The reader choices of T-6 (N-18) - decided (session 6)
- Context: T-6 built `arrow_out`, the reader (`reader::TableBatches`) and the first end-to-end reads
  (194 tests; `T_PART` 3 batches of 1,095 / 1,098 / 807 rows, `T_PART2` 4 batches, empty parts give no
  batch). N-18 lists the choices.
- Decision:
  1. **Seam**: `RowSink` (`exp::rows`, format-neutral; `row_start`, `null`, `value`, `chain_piece(first,
     last)`, `row_end`) driven by `RowCursor::step_with` is accepted as the push form of `RowSource::
     next_field` (arch 2.3); a sink and a hasher are exclusive. T-6L extends it with three methods for the
     LOB value area (arch 2.3, plan T-6L); `column` is the file position everywhere (ADR-29).
  2. **Where the value options live: `ReadOptions`**, confirmed. `Options` (dump-wide: used by the scan or
     by the type resolution, fixed for the life of a `Dump`): `strict`, `db_charset`, `client_charset`,
     `types` (`number_default`, `number_types` with the optional per-column policy, `timestamp_unit`),
     `interval_as`, `extra_intro_lines`, `max_line_bytes`, `buffer_bytes`. `ReadOptions` (per read):
     `columns`, `batch_rows`, `batch_bytes`, `on_value_error`, `on_number_overflow` (the global policy),
     `max_lob_bytes`, `unconfirmed_lob`, `trim_char`. Python keeps arch 3.4: `oradump.open()` takes every
     option and stores the per-read ones as defaults; `read_table()` accepts the per-read ones as keyword
     overrides (`None` = the `open()` value); the binding builds one `ReadOptions` per read.
  3. **Counted warnings** (once per column and table, `count`, `count_to_zero`; ADR-27 item 2) live in
     `TableBatches::warnings()`, **not** in `Dump::warnings()` (which holds the scan's warnings only).
     They are flushed when the reader returns `None` **and** when it returns an error (counts up to the
     failing row); a reader dropped early reports nothing for unread rows. `warnings()` before the end
     holds the warnings emitted so far, not the running counts. Python (T-12): `BatchReader.warnings` is the
     list (each `DumpWarning` with `.code`, `.offset`, `.table`, `.part`, `.column` as a name, `.row`,
     `.count`, `.count_to_zero`, `.max_digits_dropped`); when the iteration ends (StopIteration or a raised
     error) the binding issues each new warning once through the `warnings` module as `OradumpWarning`;
     `Dump.read_all` and the survey do the same per table. Arch 5.2a guard (3) is amended accordingly.
  4. **Which NUMBER columns `on_number_overflow` governs** (fixes the build): by the **origin of the type**,
     never by comparing Arrow types. `types::resolve` sets `ColumnInfo.type_source` = `Declared` (a usable
     declaration fixed the type: `NUMBER(p, s)` that a Decimal128 holds, `NUMBER(*,0)` (session 7: every
     `NUMBER(*,s)` with 0 <= s <= 38, ADR-35 item 1), FLOAT), `Default`
     (plain `NUMBER`, a column or table fallback, ADR-30 item 3) or `Override` (a `number_types` entry).
     The policy applies to `Default` and `Override`; `Declared` is exact or `E-NUMBER-OVERFLOW`. FLOAT read
     as Float64 is a conversion (`oradump:conversion=nearest-double`), never an overflow. As built, a
     declared `NUMBER(38,10)` counts as unconstrained because its type equals the default: that is wrong
     and is corrected in the T-5 follow-up (field) and T-11 (use).
  5. **Per-column overflow policy and "the largest number of digits dropped"** (arch 5.2a guards (3), (5)):
     both **in the alpha**, built in **T-11**, because the owner's decision of 2026-10-16 accepted `round`
     with these guards ("`error` and `null` stay available per column or globally"; every rounded value
     counted). The per-column policy is an optional field of a `number_types` entry; precedence: entry
     policy, then `ReadOptions.on_number_overflow`; `strict` turns a value `round` would change into
     `E-NUMBER-OVERFLOW` under either. `Warning` gains `max_digits_dropped: Option<u32>`, set on
     `W-NUMBER-ROUNDED` only: the largest (fractional digits of the value - scale of the column) over the
     rounded values of the column.
  6. **`batch_bytes`**: the rule of arch 5.6 step 8 stands (a batch closes after the row that brings the
     count to the cap or beyond). The plan's sentence "16 KiB forces one row per batch" on T_LONG was wrong:
     the rows add 14, 9 and 32,009 bytes (N-18), so 16 KiB gives one batch of 3 rows, a cap of 14 gives
     batches of 1 and 2 rows, a cap of 1 gives 3 batches. The plan is corrected.
  7. Accepted as built: `Warning.count` / `count_to_zero`; the new codes `W-UNVERIFIED-TYPE`,
     `W-LONG-LENGTH`, `W-LONG-CHARSET`, `W-LONGER-THAN-DECLARED`, `W-NUMBER-ROUNDED`, `W-NUMBER-NULLED`,
     `W-VALUE-NULLED`, `W-CHARSET-MISMATCH` (arch 6.1, once per column at the start of a read) and
     `E-NO-SUCH-COLUMN`; a part whose row count differs from the scan is `E-FILE-CHANGED`; a LONG value that
     fails under `on_value_error="null"` passes its remaining pieces by length and becomes one NULL with its
     partial bytes under the slot (arch 5.6 step 6); the I/O buffer is at least 128 KiB whatever
     `buffer_bytes` says (a field is up to 65,519 bytes, [R8]); the field metadata keys listed in N-18; the
     reading of the owner's "part2 must give 100 rows" as table `T_PART2`; the UROWID comparison with the
     ROWID text of the same `truth_misc.txt` line.
  8. **Interim only**: an unsupported set id of a type 1 / 96 column is `E-CONFIG` until T-9 builds arch 6.4
     (Binary column with `W-CHARSET-UNSUPPORTED`; strict: `UnsupportedCharset`). Not built in T-6 and
     assigned: LOB values (T-6L), `W-TSLTZ` (T-5 follow-up), `strict` (T-11), `trim_char` and the set
     overrides (T-9), `interval_as` (after the alpha).
- Consequences: one `Dump` can serve reads with different policies without re-opening; the scan's warning
  list never changes after the scan, so it needs no lock when readers run in threads (ADR-10, arch 4.6).
- Rejected: value options in `Options` (a policy change would re-open and re-scan the file); counted
  warnings also in `Dump::warnings()` (a list shared with readers in other threads needs a lock and its
  order depends on scheduling); the policy test by comparing a column's Arrow type with `number_default`
  (misclassifies a declared column whose type equals the default); dropping the digits figure (the owner's
  acceptance of `round` rests on the counts).

## ADR-34 The LOB reader choices of T-6L (N-19) - decided (session 7)
- Context: T-6L built LOB values end to end (17 tests in `t6l_lob.rs` and `t6l_ru.rs`; 232 in all). N-19
  lists the choices; five need a decision, the rest are accepted below.
- Decision:
  1. **`RowSink::lob_start` takes the form**: confirmed. As built: `lob_start(column, offset, form: LobForm,
     total) -> Result<LobAction>` (`LobForm::Short | Long`, `LobAction::Read | Skip`), `lob_chunk(column,
     offset, bytes)`, `lob_end(column)` (called only after `Read`); default bodies skip, so sinks without
     LOB interest are unchanged. Reason: ADR-25 item 2 refuses the **long** form of a CLOB and accepts the
     short one, and `total` cannot tell them apart: [L4] says "long iff the byte length exceeds one chunk"
     with chunk sizes that differ by version and type ("never assume a size"), and the reader never uses
     the switch to decide anything (arch 5.6 step 4). The cursor knows the form from the bytes it read
     (`fc ff` or not, [L4]) and passes it on.
  2. **The counters `TableBatches::lob_bytes_read()` and `lob_values_skipped()`**: keep them, public but
     **`#[doc(hidden)]`**, with a doc comment "diagnostic for tests; not part of the API, may change
     without notice"; not bound in Python (T-12). They are outside any semver promise of `oradump-core`.
  3. **Order of the two refusals**: confirmed. A value that is unconfirmed (ADR-25) and above
     `max_lob_bytes` gives `E-LOB-UNCONFIRMED`; with `unconfirmed_lob="decode"` it gives `E-LOB-TOO-LARGE`.
     `max_lob_bytes` is compared with the **stored** length (u16 n of the short form or T of the long form,
     [L4]), before the first byte is read: a 200,000-byte UTF-16 CLOB of 100,000 characters is refused at
     150,000. Reason: the unconfirmed refusal is about meaning and no larger limit removes it, so its
     message (which names R-038 and `unconfirmed_lob`; session 8: the database set, the confirmed list and
     `unconfirmed_lob`, ADR-25 session 8 item 7) is the useful one; the stored length is what is
     read and held, which is what the limit bounds (ADR-18). Under `on_value_error="null"` both give
     NULL + `W-VALUE-NULLED`, so the order changes only the message.
  4. **Code tables and Python**: `E-LOB-UNCONFIRMED` is a value-class code (arch 7.1): offset = first u16
     of the value, `length` = the stored length, `table`, `part`, `row`, `column` (file position, ADR-29);
     `on_value_error` decides stop or NULL. Python (T-12): `ValueDecodeError` with `.code ==
     "E-LOB-UNCONFIRMED"`; the option is `unconfirmed_lob="error" | "decode"` (Rust `ReadOptions`, Python
     `open()` default with a `read_table()` override, ADR-33 item 2). `W-LOB-UNTESTED` is an
     `OradumpWarning`, raised only under `"decode"`, **counted** once per column and table (`count` = the
     values decoded so), returned in `TableBatches::warnings()` / `BatchReader.warnings` (ADR-33 item 3).
     Both rows join the tables of arch 7.1 and 3.4.
  5. **`strict` (T-11)**: `W-LOB-UNTESTED` is outside the confirmed rules, so under `strict=True` each such
     value is `E-LOB-UNCONFIRMED` (value class; `on_value_error` decides), i.e. `strict` overrides
     `"decode"`. Same pattern as `round` under `strict` (ADR-14).
  6. **Which values are unconfirmed**: confirmed as built. Header database set [H4] 1, 171 or 178: every
     NCLOB value; a CLOB (form 1) of descriptor set 171, 178, 1 or 873 in its long form only. A CLOB of
     descriptor set 873 in such a header (not in the fixtures) is read as UTF-16 BE by type code and set
     id together [L5], its long form unconfirmed; because its descriptor set differs from the header's
     database set, it also gets `W-CHARSET-MISMATCH` (arch 6.1 states this for every descriptor id; built
     for types 1 / 96 only): **T-9 extends the warning to form-1 CLOB columns**. A CLOB of any other set:
     LargeBinary + `W-CHARSET-UNSUPPORTED` (scan), bytes, never unconfirmed [L5 "report it and return
     bytes"].
  7. Accepted as built: selecting a LOB column is no longer `E-CONFIG`; an unselected LOB value is passed
     by its lengths (`lob_start` answers `Skip`); error offsets of N-19; a value failing in a later chunk
     under `null` keeps the text already written under the null slot (arch 5.6 step 6); no new code in
     `charset.rs` (the T-6 `PieceDecoder` with `Charset::Utf16Be` serves, vectors added); the R_TEXT /
     Cyrillic-table facts taken from the export logs (the Cyrillic table has rows in three `ru_exp_*`
     files, ORA-00942 in three, absent from `ru_tables_exp_us7ascii`); the synthetic two-part LOB section
     (ADR-32 item 5); the 720 damaged variants run through the reader.
  8. Assigned: the scan's per-column count of the values ADR-25 item 2 refuses (ADR-25 item 5, arch 5.6
     step 9) goes to **T-11** (survey counting core, Rust; T-12 prints it); `strict` for `W-LOB-UNTESTED`
     to T-11; `W-CHARSET-MISMATCH` for CLOB to T-9.
  9. **After R-038** only three things change: `lob_text_kind` in `reader.rs` (which values are marked
     unconfirmed), the unconfirmed check in the sink's `lob_start`, and the tests in `t6l_ru.rs`. No API,
     option or code changes (ADR-25 item 6); `unconfirmed_lob` stays, for whatever R-038 leaves
     unconfirmed.
     Session 8: R-038 is answered; the change is task **T-6M** (ADR-25 session 8 blocks). Besides the three
     places, the list `LOB_CONFIRMED_DB_SETS` is added to `charset.rs` and the error message changes (it
     named R-038); no API, option or code is added or removed.
- Consequences: the reader distinguishes the forms without a chunk-size table; Rust users see no
  test-only methods in the documentation; T-11 has one more `strict` case.
- Rejected: item 1, deriving the form from `total` and a chunk-size table ([L4]: never assume a size);
  item 2, a `test-counters` cargo feature: integration tests in `tests/` would have to declare it as
  `required-features`, and Cargo skips a target whose required features are not enabled (The Cargo
  Book, "Cargo Targets", the `required-features` field), so the plain `cargo nextest run` of the definition
  of done would silently skip the "no LOB byte held" test unless CI adds a flag; removing the counters
  (without `unsafe`, forbidden in the workspace, no counting allocator exists to replace them); item 3,
  comparing `max_lob_bytes` with the decoded length (known only after reading); item 5, leaving
  `W-LOB-UNTESTED` a warning under `strict` (every other warning outside the confirmed rules is an error
  there).

## ADR-35 The declared-type choices of T-5f (N-20) - decided (session 7)
- Context: T-5f built `ColumnInfo.type_source`, `W-TSLTZ`, `Dump::rows` / `part_rows` errors and the flush
  of counted warnings on error (8 tests in `t5f_followup.rs`). N-20 lists the choices.
- Decision:
  1. **`NUMBER(*,s)` with 0 <= s <= 38 is a usable declaration**: confirmed. [T9] gives `NUMBER(*,s)` as
     "precision none, scale s" and `*` is precision 38 (arch 5.1); Decimal128(38, s) holds every such
     value, so `type_source = Declared` (exact or `E-NUMBER-OVERFLOW`, ADR-33 item 4). A negative s, or
     s > 38, is a declaration no Decimal128 holds (ADR-30 item 3): the column takes `number_default`,
     `type_source = Default`, one `W-DDL-FALLBACK` per column in the scan, `declared` keeps the scale.
     Example **`NUMBER(*,-2)`**: its values are integers, multiples of 100, of up to 40 digits (38 + 2); it
     maps to `number_default`, **Decimal128(38, 10)** unless set. Such a value has no fractional digits,
     so `round` never changes it; a value of more than 28 integer digits (|v| >= 10^28, the integer range
     of Decimal128(38, 10)) is `E-NUMBER-OVERFLOW` under **every** `on_number_overflow` policy (arch 5.2a
     guard 1, ADR-14) and then `on_value_error` decides stop or NULL. (The session 7 prompt says these
     values "follow the overflow policy": they follow the integer-overflow rule, which no policy relaxes.)
     Exact reading: `number_types` `decimal256(40,0)` or `string`.
  2. **A `number_types` entry on a declaration no Decimal128 holds gives no `W-DDL-FALLBACK`**: confirmed
     (ADR-30 item 3: the override reads it exactly; no fallback happened). `type_source = Override`.
  3. **Warning order in `Dump::warnings()` for one table**, as built and now fixed: table-level warnings
     first (the order warning `W-UNTESTED-LAYOUT` of ADR-30 item 5, or a table fallback's
     `W-DDL-FALLBACK`), then per column: `W-CHARSET-UNSUPPORTED`, **`W-TSLTZ`, then `W-DDL-FALLBACK`**. The
     order within one column is part of the contract (tested); the order across columns follows the
     resolution (declared order; file order after a table fallback) and is not promised.
  4. **`W-TSLTZ` once per type-231 column**: confirmed, also for Python. It is raised by the resolution
     of the first part (ADR-31 item 3), so once per column and table, not per part and not per read; it
     stands in `Dump::warnings()` (the scan's list), also on a table or column fallback, with `table`,
     `column` (file position) and the descriptor's offset. Python (T-12): `Dump.warnings` holds one
     `DumpWarning` per such column (`.column` as a name); the binding issues each entry once through the
     `warnings` module when the scan completes; `BatchReader.warnings` never repeats it. Arch 3.4's old
     phrase "once per code and file" is replaced by "once per entry of the list" (as ADR-33 item 3 already
     says for the read's warnings). Under `strict` it is an error (T-11, ADR-33 item 5 (a) list).
  5. Accepted as built: `t_num3_exp_18c` as the ADR-33 item 4 test table (`t_num` rows 19 and later
     overflow under every policy); the [T9] forms with comma and space (`NUMBER(3, 50)`, `NUMBER(38, -2)`)
     and without (`NUMBER(*,s)`) as in real lines; `part_rows` with an index outside the parts is
     `E-CONFIG` with `length` = number of parts (ADR-32 item 3); `TableBatches::next_batch` flushes the
     counted warnings on error (ADR-33 item 3); `type_source` of a non-type-2 column is set but carries no
     meaning (arch 3.2 says "type 2 only"); the dead `ReaderInit.types` and `plan_column` parameters
     removed.
- Consequences: no `NUMBER(*,s)` column silently rounds; the warning list is deterministic for tests and
  for the survey's output.
- Rejected: item 1, mapping `NUMBER(*,-2)` to Decimal128(38, 0) (the session 2 row: it holds 38 of the 40
  possible digits, so a declared type would fail on legal values without any warning, ADR-30 item 3); item 4, issuing `W-TSLTZ` again at every read (a scan fact, not a read
  fact).

## ADR-36 What T-7 and T-8 established without a code change - decided (session 7)
- Context: T-7 (5 tests, `t7_owner.rs`) and T-8 (8 tests, `t8_direct.rs`) were planned as tests that touch
  `exp/items.rs` / `exp/rows.rs` only if a test failed. None failed; no production file changed.
- Decision (recorded facts about **our reader**, not new format rules):
  1. **Owner mode reads through the same code as table mode**: O_PARENT, O_CHILD, O_EMPTY equal
     `truth_own.txt` in `own` and `own_tables` files, owner-mode and table-mode batches are equal, the
     `stc` and `ddl` families and the `ru_exp_*` owner files read completely. This is what [T8]
     (confirmed) predicts; no mode branch exists or is needed below the catalog.
  2. **Direct path**: all **80** conventional / direct pairs (every `*_exp_direct_*` file of `18c`, `11g`,
     `11g-cp1251`; 80 by `ls`) give equal rows [R7]; T_PART 1,095 / 1,098 / 807 and `part2` 21, 29, 21, 29
     per part on both paths; the LOB pairs agree with flag 0 in the direct files [L6]. The pair test reads
     with `on_value_error="null"`, `on_number_overflow="null"` so that `T_DATE` rows 8 and 9 and the
     `T_NUM` extremes compare row by row; under the defaults both paths fail at the same place.
  3. **The empty-table decision does not use the version line** [E3]: a test gives each
     `t_empty_exp_direct_<v>` file the other version's line and gets the same end marker ([E2] `LfOnly`
     on 18c, [E1] `FfFf` on 11g) and 0 rows; a synthetic ambiguous case
     (one test of [E2] holds, the other not) is `Framing` at the LF ([E3] recommendation); a
     single-column direct LONG table follows ADR-22 by default.
  4. **Fixture fact**: `T_LONG_EMPTY` (0 rows) exists only in `long_exp_direct_<v>`, not in `long_exp_<v>`
     (the export logs `long_exp_direct_18c.log` / `long_exp_direct_11g.log` name it, `long_exp_18c.log`
     / `long_exp_11g.log` do not). It is a property of how the corpus was exported, not of the format;
     [E2] already uses it ("`TABLE "` after the empty LONG table"). Consequence: pair comparisons of the
     `long` family compare table lists without it and read it alone; T-10's expected list counts it for
     the direct files only. No request.
  5. **Citations**: arch 8.1 now names the tests for rows T1 / T2 / T8 (`t7_owner.rs`), R6 / R7, E1 to E4
     and E2a (`t8_direct.rs`) and L6 (`t8_direct.rs`, LOB pairs); arch 5.4 cites the version-line test for
     [E3]. The status of the rules is the rules file's: [E3] and [E2a] stay "suggested" there; the tests
     show the reader follows them, not that they are true of other dumps.
- Rejected: a separate owner-mode code path (nothing in [T8] calls for one).

## ADR-37 LOB chunk lengths come from the stream only - decided (session 8)
- Context: [L4 v3.5]: the chunk of the long form is 16,120 / 8,060 bytes (CLOB / BLOB) on 18c and 16,264 /
  8,132 on 11g in AL32UTF8 databases, but **8,132** for a CLOB and **16,264** for an NCLOB in the 11g
  CL8MSWIN1251 database (`ru_lob2_exp_cp1251_11g.dmp`@0x29ce, 0x77fe; `exp.md` 14.6 "Session 10"); the session 8
  sentence "the CLOB chunk is twice the BLOB chunk" holds for a UTF-16 CLOB only (`exp.md` 9.8). "8,132 storage
  units" is a hypothesis; 18c in a single-byte database is not in the fixtures. A fixed chunk size is
  therefore wrong in at least one confirmed case.
- Decision:
  1. The reader takes every chunk length from its u16 [L4] and compares it with **nothing fixed**: no table
     of chunk sizes by version, type or set exists in the code, and the form (short / long) is taken from
     the bytes (`fc ff` or not), never from the length (ADR-34 item 1).
  2. A chunk of an unexpected size (any c from 1 to 0xffef, full or not, last or not) is **accepted
     silently**: the lengths in the stream are authoritative, and "every chunk but the last is full" is an
     observation [L4], not a check.
  3. What stays an error is what [L4] calls one: c = 0 or c >= 0xfff0, or a chunk that passes T
     (`Framing` at that u16, arch 5.6 step 4). A chunk that cannot add up to T is not a warning: after it the
     reader cannot find the next value, so it stops as every framing error does. (The session 8 prompt asked
     for "a counted warning only if it cannot add up to the total"; [L4] makes that case an error, and the
     design follows the rule.)
  4. Every chunk size named in the design (ADR-18, ADR-32 item 1, arch 5.6 step 4, arch 8.1) and in a test
     (`t3a_walker.rs` and `t6l_lob.rs` assert 16,120 / 6,560 / 8,060 / 3,280 and 16,264 / 4,832 / 8,132 / 2,416
     for `lob_exp_<v>`; `lob_expected.tsv`) is an **observation of a named fixture**, per version and
     database set, and is labelled so; no test asserts a chunk size of a file it does not name.
- Consequences: an 18c or other single-byte database with another chunk size reads without change; tests
  that build long-form values (T-6L, T-6M) may use any chunk size.
- Rejected: a warning when a chunk differs from the size seen for that version (it would fire on the
  confirmed 171 CLOB, 8,132 against 16,264); deriving the form from T and a chunk size (ADR-34 item 1).

## ADR-38 The first review's findings: who fixes what - decided (session 9)
- Context: Reviewer session 1 (`docs/review/review-01-golden.md`) found no blocking problem: the six golden /
  expected files regenerate byte-identically from the Analyst's tools. It found six should-fix points (S-1 to
  S-6) and stale numbers. Its mutations a3, b4, b5, b6 (a changed byte of T_ROWID2 row 1 in
  `11g/misc2_exp_11g.dmp`; a changed value in `18c/truth_long2.txt`, in `11g-cp1251/truth_stat.txt`, in
  `18c/truth_stc.txt`) failed only the checksum guard, and a2 (an R_TEXT byte of a `ru_stat_*` file), a4 (a
  digit in the T_LEN padding) and e (two checkers emptied) failed no value test.
- Decision (one line each; the work is task **T-10b**, widened, unless named otherwise):
  1. **S-1** (loose "read" evidence): T-10b. The truth-file test keys on **(folder, file name)**, never on a bare
     name, and counts a file as read only when a test **opened** it: `common::truth` (the one function that
     opens a truth file) records `(folder, name)` in a process-wide set, and the test runs every checker and
     every delegated test function it credits before comparing that set with the three `SHA256SUMS`. A literal
     in the source no longer counts. T_PART2's registry entry points to
     `t6_parts.rs::t_part2_gives_100_rows_and_each_part_holds_its_own_rows`.
     Rejected: a static registry `(folder, file, checker)` alone (it records an intention, not an opening).
  2. **S-2**: T-10b. `truth_long2.txt` and `truth_misc2.txt` leave the explicit list. T_LONG2 is compared with
     the length and hex of each `<id>|<len>|<hex>` line of the truth file of its folder (the reason "compared
     with `11_types.sql`" was wrong: the table comes from `14_long2.sql`); the `INSERT REFUSED` line means "no
     such row". A new checker in `t10_truth.rs` compares T_INTERVAL2 and T_ROWID2 of **every** `misc2_exp_<v>`
     file, every row, with `truth_misc2.txt` of its folder; the 18c-only literal test stays as it is.
  3. **S-3**: T-10b. The T_STC test asserts each table's count from `truth_stc.txt` of its folder; T_STC4
     (`NULL` there: no statistics) takes its count from the `TAB|T_STC4|50` line of `truth_stc_extra.txt`
     (which leaves the explicit list for that line); values by the rule of `corpus/sql/10_stc.sql` (ID =
     LEVEL, A = `'a' || MOD(LEVEL, 7)`, B = `'b' || LEVEL`, C = 2020-01-01 + LEVEL, ALLNULL NULL; T_STC2 also
     ADDED_LATER NULL, ADDED_NUM = MOD(ID, 3)).
  4. **S-4**: T-10b. R_TEXT is checked in `t10_truth.rs` against `truth_ru.txt` by ID in **all 27** sections
     (the 10 of the `ru_stat_*` files included; expectation as in T-9's R_TEXT test, which finds the truth in
     every client set; a file that disagrees is reported, never adapted). R_STAT (10 sections) gets a
     checker: row count = the `TAB|R_STAT|` count of `11g-cp1251/truth_stat.txt`; per column the `COL` line's
     NUM_NULLS and NUM_DISTINCT (gathered with `estimate_percent => 100`, `08_ru_stats_objects.sql`, so exact);
     values by the rule of `08_ru_stats_objects.sql` by ID. T_LONG_EMPTY stays delegated with the stated
     expectation "0 rows, direct files only" (ADR-36 item 4). Rejected: the manifest hash as R_STAT's check
     (it checks the walker's bytes, not the reader's values).
  5. **S-5**: T-10b. `check_len` compares the whole value with `RPAD(SUBSTR('L' || n || '_', 1, n), n, 'x')`
     (`01_corpus_tables.sql`) and the 12 DUMP bytes of the truth file.
  6. **S-6**: T-10b. `EXPECTED_CHECKED` / `EXPECTED_DELEGATED` are computed in the test from
     `tests/golden/exp_catalog.tsv` with the keying of `delegated()`; the literal tables go.
  7. **Stale numbers**: rules T9 ("382 lines, 1885 definitions"; `collist.py` now gives 386 / 1,907) and X2
     (the evidence lacks the third failed-export file `ru_exp_direct_us7ascii_11g`, `EXP-00008` / `ORA-00604` /
     `ORA-00942`) belong to the **Analyst** (input for the next Analyst session; open-questions (b)). The comment of
     `gen_walk_expected.py:183-185` (74 / 40 / 19), the doc of `FolderTotals.lob_sections` (name the key that is
     used, the 112 count), the comment at `t10_truth.rs:1107` (name `EXP-00008` too), `column_definitions` =
     1,907 in `TOTALS` (review N-2) and the N-23 count (257, not 256) belong to **T-10b**; the generator comment
     is a comment change, the outputs stay byte-identical.
  8. **Proof**: T-10b is done only when the Implementer applies each mutation of review section 5 that T-10b
     targets (a2, a3, a4, b4, b5, b6, e) to a **scratch copy** (outside the repository or below `target/`) and
     names the test that fails; the list goes into the T-10b note. The second Reviewer session repeats them.
- Consequences: the truth-file test can no longer be satisfied by a name; each truth file of each folder is
  compared through the reader or listed with a checked reason.
- Rejected: leaving the delegated families as they are with a note (the review showed four undetected
  mutations; a test that cannot fail is not a check).

## ADR-39 The `W-POSSIBLY-INCOMPLETE` entries of the warning inventory - decided (session 9)
- Context: T-3d (N-24) added to T-10's `EXPECTED_WARNINGS` `("???_???", W-POSSIBLY-INCOMPLETE, 2)` and
  `("\u{bf}\u{bf}\u{bf}_\u{bf}\u{bf}\u{bf}", W-POSSIBLY-INCOMPLETE, 1)`, although the plan said "inventory
  unchanged". The Reviewer checked the logs (`EXP-00056` / `ORA-00942` for `ru_exp_us7ascii_11g` and
  `ru_exp_w1252_11g`, `EXP-00008` / `ORA-00604` / `ORA-00942` for `ru_exp_direct_us7ascii_11g`), the catalog
  (0 rows each) and `exp.md` 14.3: the three tables are cut-short exports.
- Decision: **approved**. The flag is arch 6.5's (0 rows and a damaged name; N-21), it is right on exactly
  these three tables, and T-9 added it after T-10 was written. The plan's "inventory unchanged" referred to
  the two `ru_lob2_*` files, which add none (asserted). T-10b fixes the comment (`EXP-00008` too).
- Consequences: the inventory has 3 entries more than T-10 listed; T-11's fixture-wide strict list contains
  the same three scans (N-25), as it must.
- Rejected: a separate list for "known failed exports" (the warning is the reader's way of saying so).

## ADR-40 The open points of N-21 to N-24 - decided (session 9)
- Context: the Reviewer's table (section 6) lists choices of T-9, T-10, T-6M and T-3d that have no answer yet.
- Decision, one line each:
  - **N-21 `ddl_lossy` of `own_exp_w1252`**: the Implementer is right; the plan sentence is dropped. The
    replaced character is in a COMMENT item, which the reader does not expose; the patched copy and the
    `ru_exp_us7ascii` / `ru_exp_w1252` files are the positive cases. (The COMMENT-item claim is checked by
    Reviewer session 2.)
  - **N-21 flag definition** (U+FFFD from the client set, or converted at export and `?`, also 0xbf for
    WE8MSWIN1252): accepted; it is a hint, documented as such (a real `?` in a converted file is flagged).
  - **N-21 `possibly_incomplete`** (0 rows in every part and a damaged table or column name): accepted (ADR-39).
  - **N-21 override names** (Oracle names, `ascii` / `us-ascii` = US7ASCII, `encoding_rs` labels as
    `Charset::Other`): accepted; an `Other` set is never counted as confirmed or single-byte (ADR-25 session
    8), and the README of T-13 lists it as untested.
  - **N-21 LONG in a header of an unsupported database set**: stays `E-CONFIG` (message names `db_charset`)
    for the alpha. LargeBinary + `W-CHARSET-UNSUPPORTED` is done only if Q-4 shows a database set outside
    the five. Rejected for now: the change without a case that needs it.
  - **N-21 `trim_char`, the US7ASCII patched copy, R_TEXT in storage order**: accepted as built.
  - **N-22 registry and delegation**: accepted with the T-10b changes of ADR-38; **partial truth texts**:
    accepted only where the rest is rebuilt exactly from the SQL (BN, T_WIDE trimmed on the right; T_LEN
    exact by ADR-38 item 5); **warning inventory**: accepted (with ADR-39); **T_NUM stops by default**:
    accepted (integer overflow, ADR-14).
  - **N-23 (1) the database set in the `E-LOB-UNCONFIRMED` message**: `Error.message` (and
    `Warning.message`) become **`Cow<'static, str>`** in **T-12**, whose first Rust step it is; the four
    static texts of `lob_unconfirmed_message` become one formatted text naming the set id. T-11 is done and
    did not touch it; T-12 builds the Python messages from these fields anyway. The rule "no value in a
    message" stands (a set id is metadata).
    Rejected: an extra numeric field for the set id (one more field for one message).
  - **N-23 (2) to (4)** (`W-LOB-UNTESTED` text, test name kept, header offsets of the 178 copy): accepted.
    **Correction**: the sentence "its T_LOB2 reads carry T-9's `W-CHARSET-MISMATCH` for the form-1 CLOB A"
    is wrong: those warnings are column V's, because the reader raises the warning for types 1 / 96 only
    (N-25; ADR-41 item 5). The test passes on any list of mismatch warnings; T-9b makes it assert per column.
  - **N-23 (5) refusal beyond 171**: accepted; it is ADR-25 session 8 as decided. Test count at `a937f2f`:
    257 (review N-8; T-10b corrects the note).
  - **N-24 generator constants edited by hand**: accepted (the generators are code; their outputs are
    never edited and regenerate byte-identically, review section 1). **`t4_values.rs` outside the list**:
    accepted. **Loose literal match** and **`truth_long2` / `truth_misc2` in the explicit list**: replaced
    by ADR-38 items 1 and 2. **Counts by derivation**: accepted, with `column_definitions` added (ADR-38 item 7).
    **Ownership of `tests/golden/`**: ADR-42.
- Consequences: N-21 to N-24 are closed except the work assigned to T-9b, T-10b and T-12.

## ADR-41 The T-11 choices (N-25) - decided (session 9)
- Context: T-11 (commit `7db4667`) built `strict`, the per-column overflow policy, `max_digits_dropped`,
  `extra_intro_lines` and the survey's counting core; `t11_codes.rs`, `t11_strict.rs`, `t11_options.rs`.
- Decision:
  1. **Two new error codes, both accepted**: `E-STRICT` (fatal for the read or scan that raised it; the
     warning that caused it in `Error.warning`) and `E-UNSUPPORTED-CHARSET` (`W-CHARSET-UNSUPPORTED` under
     `strict`). Python (T-12): `E-UNSUPPORTED-CHARSET` -> `UnsupportedCharset(UnsupportedError)` (arch 3.4 already
     names it); `E-STRICT` -> **`StrictError(OradumpError)`**, a direct subclass of the base error, with
     `.warning` (the `W-...` code as a string), so a caller can catch it apart from format and value errors.
     `Error.warning` is set on **every** error a strict conversion produced (also `E-UNSUPPORTED-CHARSET`, the
     `E-NUMBER-OVERFLOW` of `round`, the `E-LOB-UNCONFIRMED` of `W-LOB-UNTESTED`) and is `None` otherwise;
     Python exposes `.warning` on `OradumpError`, so "did strict cause this?" is one test for every class. T-12
     builds the missing settings if T-11 sets it on `E-STRICT` only. Rejected: mapping `E-STRICT` to
     `UnsupportedError` (an unconfirmed layout is not an unsupported one; the caller would lose the difference).
  2. **`strict` with `on_value_error="null"`**: confirmed. The value-layer warnings (`W-UNVERIFIED-TYPE`,
     `W-LONG-LENGTH`, `W-LONG-CHARSET`, `W-LONGER-THAN-DECLARED`) become `E-STRICT` at the first such value and
     stop the read of that table; `null` does not turn them into NULL, because the value is not bad but
     unconfirmed, and NULLing it would lose data that the non-strict read delivers. Only `E-NUMBER-OVERFLOW`
     (`round`) and `E-LOB-UNCONFIRMED` (`W-LOB-UNTESTED`) are value class under `strict` (arch 7.1). The framing
     is intact: the `Dump` stays usable and another table can be read (T-12 tests it through Python).
  3. **`round` under `strict` counts as `error`** and `oradump:number_overflow` says `error`: confirmed
     (ADR-14; the metadata states what the read did).
  4. **The NUMBER profile of arch 5.2b stays in T-12**: confirmed (the plan's T-12 Done-when has its numbers).
  5. **Bug from T-9: `W-CHARSET-MISMATCH` is not raised for form-1 CLOB columns**, contrary to ADR-34 item 6 and
     plan T-9 session 7 (a). Fix: type 112 joins 1 and 96 in the condition of `reader.rs` (form not 2, no
     `db_charset`, descriptor id != the header's database id), decodable or not, as for types 1 / 96. New task
     **T-9b** (plan). No fixture is affected: in all 201 files every CLOB's set equals the database set, so the
     T-11 fixture-wide strict list does not change.
  6. **Offset of `W-LONG-LENGTH`** (rule): the offset of the **start of the value**, the same byte a value
     error of that value reports: for a conventional field its first data byte (as built, `t11_codes.rs`
     `w_long_length`); for a direct-path chain the first data byte of the **first** piece, not the last.
     `row`, `part`, `column` are those of the value. Reason: ADR-27 item 2 ("offset of the first value"), and
     a user finds a value from its start. T-9b changes the chain case and tests it on a copy of
     `long_exp_direct_18c.dmp` with a chain above 32,000 bytes.
  7. Accepted as built: the per-column policy map `TypeOptions.number_overflow` (precedence entry, then
     `ReadOptions`, then strict), `max_digits_dropped`, `extra_intro_lines` (`E-CONFIG` on a bad entry),
     `scan` committing nothing on a strict error, the counting core `survey::count_table`, the fixture-wide
     strict list of N-25 (4 TSLTZ scans, 3 failed-export scans, `T_NUM2` / `T_NUM3` strict-only, 26
     reads that fail in every mode).
- Consequences: two codes join arch 7.1 and the Python mapping; T-9b is a short production fix.
- **Session 11 changes (T-11b note item 3, T-12 note D3):**
  8. **`Error.warning` as completed by T-12: confirmed.** It is set (a) on `E-STRICT` and `E-UNSUPPORTED-CHARSET`
     (T-11); (b) on an `E-NUMBER-OVERFLOW` only when `strict` turned the column's `round` into `error` **and** `round`
     would have rounded that value: `W-NUMBER-ROUNDED`; an integer part that does not fit is an error under every
     policy and carries no warning (strict did not cause it); (c) on `E-LOB-UNCONFIRMED` only under `strict` with
     `unconfirmed_lob="decode"`: `W-LOB-UNTESTED`; without `decode` it is the ordinary refusal of ADR-25 and carries
     none. Rule in one line: **`warning` is set exactly when the error would have been that warning without
     `strict`.** The changed expectations in `t11_strict.rs` (`DOCUMENTED`: T_NUM2 / T_NUM3 now name
     `W-NUMBER-ROUNDED`) and the `.clone()` in `t11_codes.rs` line 66 are accepted; Reviewer 3 reads both edits.
  9. **The strict table of T-11b: confirmed as built.** Every warning code that arch 7.1 does not give an error of its
     own is `E-STRICT` under `strict` (the default rule of the session 9 paragraph of 7.1 and item 1): `W-UNKNOWN-INTRO`,
     `W-TRAILING`, `W-UNTESTED-VERSION`, `W-UNTESTED-HEADER`, `W-FILE-SIZE`, `W-UNTESTED-LAYOUT`, `W-DDL-FALLBACK`,
     `W-UNVERIFIED-TYPE`, `W-LONG-LENGTH`, `W-LONG-CHARSET`, `W-LONGER-THAN-DECLARED`, `W-CHARSET-MISMATCH`, `W-TSLTZ`,
     `W-POSSIBLY-INCOMPLETE`; own codes: `W-CHARSET-UNSUPPORTED` -> `E-UNSUPPORTED-CHARSET`, `W-NUMBER-ROUNDED` ->
     `E-NUMBER-OVERFLOW`, `W-LOB-UNTESTED` -> `E-LOB-UNCONFIRMED`; none: `W-NUMBER-NULLED`, `W-VALUE-NULLED` (a policy
     the caller chose). No code needs another error: `W-TRAILING` (non-zero bytes after the second `EXIT`
     line, arch 2.5 step 6) is outside the confirmed rules like the others, and `strict` means "nothing unconfirmed passes". Arch 7.1 now
     lists this table. **Review N-1** (deleting the one `assert!` on the union in
     `every_table_of_the_exp_files_is_classified_and_read` is caught by no test) **stays open, accepted by ADR-43
     item 5**; Reviewer session 3 reads that line.

## ADR-42 Roles for tests and golden files; a review after each group of tasks - decided (session 9; the `CLAUDE.md` text needs the project owner)
- Context: `CLAUDE.md` gives `tests/` to a "Test engineer", a role that never ran. The Reviewer
  (`.claude/agents/reviewer.md`) now does that job, writes only `docs/review/` and edits no test.
- Decision:
  1. `tests/golden/` and `crates/oradump-core/tests/data/` are **regenerated by the Implementer with the
     project's generators only** (which import the Analyst's tools in `docs/formats/tools/`), never from the
     reader's output and never edited by hand; the **Reviewer checks** them (generator, input, byte-identical
     rerun) in the next review session.
  2. **Review gate**: after every group of tasks that adds or changes tests over the corpus, a Reviewer
     session runs before the next group starts. Groups from here: (T-11, T-9b, T-10b) -> Reviewer session
     2; (T-12, T-13) -> Reviewer session 3; (T-14, T-15) -> Reviewer session 4. A blocking finding stops the
     next group; should-fix findings become a task of the next group.
  3. The `CLAUDE.md` text is the owner's to change; the exact replacement is in `open-questions.md` (a).
- Rejected: a review after every task (the groups share files; one review sees the joint effect at lower
  cost); letting the Reviewer edit tests (then nobody checks the Reviewer's tests).

## ADR-43 The second review's findings: who fixes what - decided (session 10)
- Context: Reviewer session 2 (`docs/review/review-02-t9b-t11.md`, HEAD `7941e40`, 371 tests) found no blocking
  problem and four should-fix points. Mutations that no test caught: d1 / d2 (the failing field's bytes, as text
  or hex, in a value error's message), e1 (the survey counts years below 1 for DATE only), e3 / e4 (the `assert!`
  of `check_wide` deleted, then a T_WIDE byte of `11g/t_wide_exp_11g.dmp` changed). Probe of S-3 on
  `18c/long_exp_direct_18c.dmp`, T_LONG row 3, a chain of 5 pieces (headers 0x2288 / 0x2326 / 0x4241 / 0x615c /
  0x8077; first data byte 0x228a; bytes checked in this session: `9c 80` at 0x2288, `19 9f` at 0x2326 and 0x615c):
  `E-LOB-TOO-LARGE` 0x228a, its `W-VALUE-NULLED` 0x615e, `E-BAD-TEXT` 0x2328.
- Decision (task **T-11b** is tests only; **T-12** takes what needs production code or the `Cow` messages):
  1. **S-1 (no value in any message): T-12**, confirmed; it is the task that makes `message` a
     `Cow<'static, str>` (ADR-40), so the test must hold before the first formatted message lands. The check
     searches each `Error.message` / `Warning.message` (the `message` field, not `Display`, whose `offset=0x...`
     and integer fields would match hex by chance) of every error and warning the fixture runs produce, under
     `raise` and `null`, for: (a) the raw bytes of every field of the table read (as now); (b) every run of at
     least 4 characters of the lossy UTF-8 rendering of each raw field; (c) **the hex form** of every raw field of
     2 bytes or more, lower and upper case, as a token bounded by non-hex characters; (d) the reader's rendering
     of every cell of a lenient read (`on_value_error="null"`) of the same table, at least 4 characters, pure digit
     strings shorter than 6 excluded (messages hold row numbers and set ids). Hex is covered because d2 shows it is
     the form a careless builder writes. The corpus of messages gets a **text-valued bad field**: a patched copy of
     an AL32UTF8 fixture (below `target/`, bytes asserted before patching) in which one byte inside a VARCHAR2 value
     of 8 ASCII bytes or more becomes 0xff (`E-BAD-TEXT` under `raise`, `W-VALUE-NULLED` under `null`). Proof: the
     check function, fed a real fixture message with the value appended in memory in each form (a) to (d), reports
     it (a permanent test); and mutations d1, d2 and d3 (the patched copy's lossy text appended to `E-BAD-TEXT`'s
     message) applied to a scratch copy each fail the check test (listed in the T-12 note).
     Rejected: keeping `&'static str` (ADR-40 needs the set id in a message); checking `Display` (false hits).
  2. **S-2 (years below 1 for the TIMESTAMP family): T-11b, patched copies**, because no fixture holds such a
     value: the year-below-1 values of the corpus are T_DATE's (`truth_date.txt` rows 8, 9: `63,64,...` = -0100,
     `35,58,...` = -4712), `truth_ts.txt` and `truth_tstz.txt` have none, and R-044 (which has them) is not
     generated. The copies write the **confirmed** pair `63 64` [Y2] over the first two bytes of a value: in
     `18c/t_ts_exp_18c.dmp` row 2 of each of the three TIMESTAMP columns (7-byte values `78,74,3,1a,11,a,1c`,
     [Y5] gives the DATE layout); in `18c/t_tstz_exp_18c.dmp` row 2 of the TSTZ column (`78,74,3,1a,14,23,38,...`,
     zone +00:00) and the first non-NULL TSLTZ value. Expected: `years_below_1` = 1 in each patched column, 0 in
     every other; the reader's first error under `raise` is `E-BAD-DATETIME` at row 2 of the first patched column.
     When R-044 is generated, its files replace the copies (no design change).
  3. **S-3 (offsets of one value): the rule changes in the code, not in the text. T-12** (production; `reader.rs`
     is touched by T-12 anyway). Rule, replacing ADR-41 item 6: **every report about one value** - a value error
     under `raise`, the `W-VALUE-NULLED` / `W-NUMBER-NULLED` that replaces it under `null`, and every value-level
     warning (`W-LONG-LENGTH`, `W-LONG-CHARSET`, `W-UNVERIFIED-TYPE`, `W-LONGER-THAN-DECLARED`, `W-LOB-UNTESTED`,
     `W-NUMBER-ROUNDED`'s first value) - carries `offset` = **the start of the value**: the first data byte of a
     conventional field; the first data byte of the **first** piece of a direct-path LONG chain (`st.start`); the
     offset `lob_start` receives for a LOB value (as `E-LOB-UNCONFIRMED`). So `E-LOB-TOO-LARGE` keeps 0x228a, its
     `W-VALUE-NULLED` becomes 0x228a, and `E-BAD-TEXT` inside a chain or a LOB becomes the value start too (0x228a;
     a conventional field already reports its start). The position where decoding failed is not lost: for a
     value of more than one piece or chunk, the message names the 1-based piece (chunk) in which the failure was
     detected, an integer (metadata, allowed by S-1's check through the digit exclusion). Framing errors of the
     chain itself (a piece length, a missing last piece) stay layer-2 errors at their own offset (arch 7.4, "first
     byte it could not accept"). Reason: one value, one offset - a caller who switches `raise` to `null` finds the
     same byte, and a user finds a value from its start; the old `E-BAD-TEXT` offset was the piece where the
     decoder noticed, not the bad byte either. Test: new `t12_offsets.rs::one_chain_value_reports_its_start_under_every_policy`
     on copies of `long_exp_direct_18c.dmp` (offsets asserted before patching): (i) `max_lob_bytes=20000`, `raise`
     -> `E-LOB-TOO-LARGE` 0x228a; (ii) the same with `null` -> `W-VALUE-NULLED` 0x228a, row 3; (iii) byte 0x2325
     (the last data byte of piece 1) set to 0xc3 -> `E-BAD-TEXT` 0x228a with "piece 2" in the message; (iv) (iii)
     with `null` -> `W-VALUE-NULLED` 0x228a; plus the same two checks (`raise`, `null`) for a long-form CLOB value of
     `18c/lob_exp_18c.dmp` whose first chunk ends in an unpaired high surrogate (`d8 00` as its last UTF-16 code
     unit [L5]; chunk offsets from the walker helpers, asserted before patching): `E-BAD-TEXT` and
     `W-VALUE-NULLED` at the offset `lob_start` receives, "chunk 2" in the message.
     `t6_patched::a_broken_utf8_sequence_in_a_chain_is_a_value_error` asserts the value start instead of
     `offset.is_some()` (T-12 owns `t6_patched.rs` in this group, so review N-4's line there is T-12's too).
     Rejected: keeping the precise byte for `E-BAD-TEXT` only (two offsets for one value under two policies,
     which is the reported defect; and the chain decoder knows only the piece).
  4. **S-4 (strict list): T-11b; the expected list lives in the test, not in a golden file** (it is not produced
     by an Analyst tool, so a generator would only copy a literal). Two tables in `t11_strict.rs`:
     (a) `ARCH_7_1_STRICT`: one line per `WarningCode::ALL` entry with its outcome under `strict` (the error code
     or "none" for `W-NUMBER-NULLED` / `W-VALUE-NULLED`), each line citing arch 7.1 or the ADR; a test asserts that
     the table has exactly the codes of `WarningCode::ALL` and that `strict_error()` equals it for every code
     (catches a1 without a fixture); the derivation test uses this table, **never `strict_error()`**;
     (b) `DOCUMENTED`: per entry (stage, table key, code, warning, **reason**) with no count; the reason names the
     rule, truth file or log (W-TSLTZ: arch 5.1 and the `tsltz=` field of `truth_tstz.txt`; the failed exports:
     ADR-39 and the three logs; T_NUM2 / T_NUM3: the truth texts classified by
     `t10_truth.rs::the_classification_of_the_truth_numbers`; T_NUM, T_DATE, T_TS, T_INTERVAL2: the same
     classification and ADR-28). The **count** of each entry is the number of `exp_catalog.tsv` sections with that
     table key, under the keying of `delegated()` (as T-10b derived the instance counts); no count is typed in.
  5. **e3 / e4 (thin wrappers): yes, a second guard, T-11b.** Every checker of `t10_truth.rs` returns its list
     of differences instead of asserting; `every_table_of_the_exp_files_is_classified_and_read` asserts the
     **union** once, per tag; and one break test runs `check_table` for **every** tag of `checker_tag` with one
     truth line or one reader cell changed in memory and expects a non-empty list for that tag. Proof on a
     scratch copy: e3 cannot be expressed any more (no per-checker `assert!` is left); e4 (the 11g T_WIDE byte
     @0x3804) fails `every_table_of_the_exp_files_is_classified_and_read`. The union is one named function that
     both tests call. Deleting its single `assert!` remains an edit to a test that no test can catch (review N-1),
     but it is one line instead of one per checker, and review reads it.
     Rejected: a counter of executed comparisons (it counts work, not equality).
  6. **The review's notes in T-11b** (tests only): N-2 (`t4_values.rs:477` exact 16 = 2 per T_DATE section x 8,
     from the truth file; `:484-489` exact counts; `t3b_parts.rs:547-565` `lob_columns == 5` for `lob_exp_18c`
     and the statistics compared with `lob_expected.tsv`, not with the walker), N-4 (`t9_charset.rs:978` column 3
     and count 1; `:913-923` comment and assertion agree; the `t6_patched.rs:69-73` part is T-12's, item 3), N-5 (`t9b_fixes.rs:80-86` rows 1 to 3 against `01b_optional_tables.sql`),
     N-6 (`truth_checks.rs:407-412` R_LOB2.B from the rule of `15_ru_lob2.sql`; doc comment of `lob_diffs` names
     the pair-equality coverage of the direct files), N-8 (below, ADR-44 item 5), N-9 (the T_STC break test takes
     its count from the truth line). N-1 is item 5; N-3 is item 4; N-7 is ADR-44 item 7.
- Consequences: T-11b is a short test session that can run beside T-12 (plan); T-12 starts with the offset rule
  and the message check before any formatted message. Reviewer session 3 repeats d1, d2, d3, e1, e3 / e4, a1.
- Rejected: one task for all of it (T-12 would then hold every test fix and grow past one session).

## ADR-44 The T-10b note's items 1 to 6, the review's notes, the numbers - decided (session 10)
- Context: the T-10b note (`implementer-notes.md`, "T-10b (Implementer, session 9)") asked for confirmation of six
  choices; review 02 section 7 lists them as "only noted". N-25 is answered by ADR-41.
- Decision, one line each:
  1. **`DELEGATED` with a fourth field** (the family check the test calls) and `delegated_tests_exist`: accepted.
  2. **Duplicates kept**: accepted; none hides a weaker check (review 02 section 2), and three of the old tests are
     stronger (SQL values). They are not removed.
  3. **Extra comparisons** (T_STC `COLUMN` statistics, R_LOB2 head / tail, R_CLOB, T_LOB3 DUMP): accepted; each is
     from a truth file.
  4. **R_LOB2.B against the reader's C**: changed: B is compared with the CL8MSWIN1251 bytes of the rule of
     `15_ru_lob2.sql` (N-6; T-11b). Oracle's map from `truth_csmap.txt` instead of `encoding_rs`: accepted.
  5. **`TRUTH_NOT_OPENED` reasons**: accepted (checked true by Reviewer session 2). T-11b turns two entries into
     checks (review N-8): `truth_env.txt` (18c, 11g) against the header database and national set ids of every
     file of its folder, by the ids `truth_charsets.txt` gives for the two names; the OBJECT TABLE lines of
     `truth_own_ddl.txt` against the catalog's table names of `own_exp_<v>`. A disagreement is a note, never an
     adapted expectation. `truth_float`, `truth_r045`, `truth_decl` stay listed (T_DECL: optional `decl_diffs`).
  6. **The wrapper gap**: closed by ADR-43 item 5.
  7. **Numbers** (review N-7): the T-10b note's "370 tests" is **371** at `7941e40` (`#[test]` count: 347 at
     `7db4667`, 352 at `e308d9a`, 371 at `7941e40`); "nothing is committed" and "the uncommitted `reader.rs` /
     `t9b_fixes.rs`" are stale since `e308d9a` / `7941e40`. Corrected in the note by an Architect line; the plan
     gets DONE lines for T-9b and T-10b. Rules T9 (386 lines, 1,907 definitions) and X2 (the third failed export,
     `ru_exp_direct_us7ascii_11g`, `EXP-00008` / `ORA-00604` / `ORA-00942`) stay with the **Analyst**
     (open-questions (b)).
  8. **T-9b note** (`E-LOB-TOO-LARGE` on a chain at the first piece; unit test 30 -> 10): accepted; it was
     pre-authorised by plan T-9b (c), and ADR-43 item 3 makes it the general rule.
- Consequences: the T-10b note and N-25 are closed; the remaining work is T-11b's.

## ADR-45 Licence: MIT OR Apache-2.0 - decided (owner, 2026-10-17: Q-6)
- Context: no licence is set in the manifests (N-5); the wheel metadata and the README of T-13 need one. The owner
  decided Q-6: dual licence.
- Decision:
  1. **Licence**: `MIT OR Apache-2.0` (an SPDX licence expression) for every Rust crate of the workspace and for
     the Python package. Cargo: `license = "MIT OR Apache-2.0"` in `[workspace.package]` of the root `Cargo.toml`
     and `license.workspace = true` in each crate's `[package]` (`oradump-core`, `oradump-py`, any later crate;
     `fuzz/` too, though it is not published). Python: `[project] license = "MIT OR Apache-2.0"` and
     `license-files = ["LICENSE-MIT", "LICENSE-APACHE"]` in `pyproject.toml` (PEP 639, Python Packaging User
     Guide); the maturin version in `[build-system] requires` must support PEP 639 (its changelog places the
     support and `license-files` in the 1.9 series; T-13 pins a version and checks the wheel's `METADATA` has
     `License-Expression: MIT OR Apache-2.0` and the two files under `.dist-info/licenses/`). No
     `License ::` classifier (PEP 639 deprecates them beside an expression).
  2. **Files**: `LICENSE-MIT` and `LICENSE-APACHE` at the repository root. **The Implementer writes them in
     T-13**: `LICENSE-APACHE` is the verbatim text of https://www.apache.org/licenses/LICENSE-2.0.txt (no edit, the
     appendix left as it is); `LICENSE-MIT` the SPDX `MIT` text with the copyright line the owner confirms (Q-8;
     recommended `Copyright (c) 2026 Maksim Tkachenko`). Reason: the texts are fixed, the task that checks the
     wheel also creates its inputs, and nothing waits on the owner but one line. `README.md` gets the "License"
     section and the trademark line below (T-13); `CONTRIBUTING.md` gets the contribution sentence (T-13).
  3. **README text** (exact, T-13 copies it):

         ## License

         Licensed under either of

         - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
           <https://www.apache.org/licenses/LICENSE-2.0>)
         - MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

         at your option.

         Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
         this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
         any additional terms or conditions.

         The test dumps in `corpus/fixtures/` are generated from synthetic tables by the SQL in `corpus/sql/`.
         They contain no real data and are not part of the wheel or the source distribution.

         oradump is an independent project. It is not affiliated with, endorsed by or sponsored by Oracle
         Corporation. Oracle is a trademark or registered trademark of Oracle Corporation and/or its
         affiliates.

  4. **Contributing sentence** (exact, `CONTRIBUTING.md`, section "Licence of contributions"):

         Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
         oradump by you, as defined in the Apache-2.0 license, shall be dual licensed under the MIT license
         and the Apache License, Version 2.0, without any additional terms or conditions.

  5. **Dependency check**: `cargo deny check licenses` (cargo-deny) passes before T-13 is done, and in CI from
     then on. T-13 writes `deny.toml` at the root with `[licenses] allow = ["MIT", "Apache-2.0",
     "Apache-2.0 WITH LLVM-exception", "BSD-2-Clause", "BSD-3-Clause", "ISC", "Unicode-3.0", "Zlib"]`
     (`Unicode-DFS-2016` too if a pinned dependency still declares it; cargo-deny denies every licence not in
     `allow`); `confidence-threshold` left at its default 0.8; no `exceptions` without a decision record. A
     dependency with another licence (MPL-2.0, any GPL family, unknown) is a note to the Architect, not an
     added `allow` line. `cargo about` is not used (one tool is enough; it produces attribution files, which
     the wheel does not need for these licences beyond the crates' own files).
  6. **The corpus**: `corpus/` (fixtures, truth files, SQL, scripts), `tests/` and `docs/` stay **in the
     repository only**; neither the wheel nor the sdist contains a `.dmp`, a `truth_*.txt` or `corpus/` (T-13:
     maturin's `[tool.maturin] exclude` / the sdist's include list, checked by listing both archives; the crates'
     `package.exclude` if a crate is ever packaged). The fixtures are data from our own synthetic tables and are
     covered by the repository's licence like every other file. **The owner's real dumps never enter the
     repository** (`CLAUDE.md` rule 4): `.gitignore` keeps `corpus/work/` out, and no test reads outside
     `corpus/fixtures/` except the optional T-15 runs on `corpus/work/`.
  7. The clean-room record (`docs/research/sources.md`) stays the basis for the claim that no third-party parser
     code is in the project; the README's licence section does not repeat it.
- Consequences: T-13 adds `LICENSE-MIT`, `LICENSE-APACHE`, `deny.toml`, the `license` fields, `README.md` and
  `CONTRIBUTING.md` (root files: the Q-5 owner action adds them to the Implementer's row); the release no longer
  waits for Q-6. Q-8 (the copyright line) blocks only `LICENSE-MIT`.
- Rejected: Apache-2.0 alone (the Rust ecosystem's habit is the pair, and MIT eases reuse in other licences);
  shipping the fixtures in the sdist (they are test data of this repository, the tests that read them are not
  shipped either, and every byte shipped is a byte to justify); the owner writing the files himself (a fixed
  text; the task that checks the wheel should create its inputs).

## ADR-46 The T-12 choices (D2, D4 to D10) and the survey's default read - decided (session 11)
- Context: the T-12 note (`implementer-notes.md`, "T-12 (Implementer, session 10)") asks for confirmation of D1 to
  D10 and records the pandas / pyarrow findings. D1 is ADR-3 (session 11), D3 is ADR-41 item 8, the findings are
  ADR-28 (b) and ADR-19 (session 11).
- Decision, one line each:
  1. **D2** (`oradump-py` under the workspace lints, `unsafe_code = "forbid"`, no `unwrap`, no indexing, no unchecked
     arithmetic): accepted; it is the rule from now on for every crate of the workspace.
  2. **D4** (S-3 as built): accepted. A LONG chain's message names "(piece n of the value)" only for a value of more
     than one piece; a LOB message names "(chunk n of the value)" always, also for one chunk: one fixed form per kind
     is simpler to grep and holds no value; arch 7.4 now says so. The extra test
     `the_long_charset_warning_of_a_chain_is_at_the_value_start` is accepted.
  3. **D5** (thresholds of the S-1 check): accepted: raw fields of 4 bytes or more, 4-character windows of the
     lossy text (digit-only and white-space-only windows left out), hex of fields of 2 bytes or more as whole
     hex-digit runs, cells of 4 characters or more (digit-only below 6 and `<NULL>` left out), per table corpus. Known
     and accepted gaps: values of 1 to 3 bytes; hex written with separators (`ff 41 42`); since the corpus has only 16
     distinct static texts, the check guards future formatted messages. **Reviewer 3 mutates, each alone on a scratch
     copy:** d1, d2, d3 again; **d4** the rendered cell of a `t_num_exp_18c` value of 6 or more digits appended to the
     `E-NUMBER-OVERFLOW` message (must fail via (d)); **d5** the first 4 bytes only of the patched text field appended
     to `E-BAD-TEXT` (must fail via (b)); **d6** the hex of the first 8 bytes only of a field longer than 8 bytes
     appended (a prefix of the hex, not the whole token): the Reviewer records whether it is caught; if not, it is a
     should-fix with this rule: a hex-digit run of 8 or more digits in a message that is a substring of a field's hex
     is a hit.
  4. **D6** (names after a failed scan): accepted: `.table` is `None` when the scan failed and `.table_ordinal` holds
     the section number; `.part` is `(partition, subpartition)` names or `None`; `.column` the declared name resolved
     through `PartInfo.file_order` (ADR-29); extra attributes `.table_ordinal` and `.type_code` join arch 3.4.
  5. **D7** (survey counting core): accepted as built: `len_min` / `len_max` / `len_total` per column, the
     `NumberProfile` classified by the reader's own `to_decimal128` / `to_decimal256` with `Round` (the profile and
     the read cannot disagree), zero values left out of the exp10 / leading-position ranges, `number_summary()`, and
     `Catalog.item_counts.g1_fields` printed (owner Q-2 (3)).
  6. **D8** (`_core.digest`, FNV-1a over a canonical byte form, test support): accepted; it stays in the private
     module `_core`, is not exported by `oradump/__init__.py` and is not documented in the README.
  7. **D9** (packaging files): accepted; `readme`, `license`, `license-files` are T-13's (ADR-45, ADR-48); the ruff
     exclusions of `tests/golden` and `crates/oradump-core/tests/data` (generators, not package code), line length
     120, `A001` / `A004` ignored for `oradump.open`, mypy strict with `ignore_missing_imports` for pyarrow: accepted.
  8. **D10** (warnings in Python: header warnings at `Dump.__init__`, scan warnings at the first scan, read warnings
     when the iteration ends; `.detail`; `E-IO` as `OSError` with errno; capsule-path errors as `[CODE offset=0x..]`
     inside `ArrowInvalid`): accepted; matches arch 3.4.
  9. **Survey default**: **keep** the lenient read pass by default (`--no-read` skips it). Owner Q-2 (2) requires
     that warnings are printed by `survey`; the value-level warnings (`W-VALUE-NULLED`, `W-LONG-CHARSET`,
     `W-LONGER-THAN-DECLARED`, `W-LOB-UNTESTED`, ...) exist only after a read, and the survey is run once per dump
     before a job, where time matters less than completeness. The survey prints, in its first lines, which passes it
     runs and how to skip the read. T-15 measures the survey's time on a large `corpus/work/` file; if the read pass
     is more than half of it, the Architect considers merging the counting pass into the read (one pass).
     Rejected: read off by default (the pasted-back output would lack the warnings the owner asked for).
- Consequences: arch 3.4 (attributes), 7.4 (piece / chunk wording), 8.5 (passes) updated; Reviewer 3's list gains d4
  to d6.

## ADR-47 `interval_as` in the alpha - decided (session 11)
- Context: ADR-19 kept `interval_as="string"` for after the alpha. T-12 showed that `pyarrow.parquet` refuses
  Interval(MonthDayNano), the type of every INTERVAL YEAR TO MONTH column (and of DAY TO SECOND with p > 8, or
  f >= 7 and p > 5). The owner's job is ETL; Parquet is a likely target. A documented cast was considered: no pyarrow
  cast from `month_day_nano_interval` to an integer or string type was verified in this project, so the README
  could not promise one. The renderers already exist in production code (`values/interval.rs`:
  `IntervalYm::to_text(p)`, `IntervalDs::to_text(p, f)`, checked against the `TO_CHAR` text of the truth files in T-4).
- Decision: **`interval_as` ships in the alpha, built in T-13.** `open(..., interval_as="native")`, values
  `"native"` (default, ADR-19 types unchanged) and `"string"`; an `Options` field (fixed for the life of a `Dump`,
  arch 3.2: it changes the schema), not a `read_table` override; any other value is `E-CONFIG`
  (`InvalidOption`). With `"string"` every type-182 and type-183 column is **Utf8** holding the `TO_CHAR` text of
  [Y10] (arch 5.7 item 5) with p and f from [T9]; when [T9] gives none (fallback), p = the digits the value needs and
  f = 9, with `W-DDL-FALLBACK` once per column in the scan. Field metadata `oradump:conversion` = `to-char-text`.
  Decoding checks are unchanged (length, field ranges, opposite signs stay `E-BAD-INTERVAL`); only the 32-bit-months
  limit of ADR-28 (a) disappears, since text holds every value. Whole dump, both kinds: one switch, predictable
  schema.
- Consequences: `misc2` rows 2 and 3 (YEAR(9) beyond 2^31 months) read without error under `"string"`; the README's
  type table gets one line per mode and the Parquet note. Survey unchanged.
- Rejected: documenting a cast only (not verified; every job would carry its own conversion code); per-column
  `interval_as` (no evidence anyone needs it; can come later without breaking the dump-wide option); Interval(YearMonth)
  for Parquet (pyarrow's Parquet writer support for it is not verified either, and it would change the default type).

## ADR-48 First-release wheel: `manylinux_2_28`, pinned maturin, checked metadata and contents - decided (session 11; session 12: baseline confirmed by the owner, Q-9 closed)
- Context (T-12 note): `maturin build` on the VM (Debian 13, glibc 2.41) tags the wheel `cp311-abi3-manylinux_2_34_x86_64`
  (maturin assigns the lowest manylinux tag the built library satisfies, maturin.rs "Distribution"). glibc 2.34
  excludes RHEL / Oracle Linux 8 (glibc 2.28), still common on corporate ETL hosts. maturin's documentation offers
  `--zig` "for manylinux targets, use zig to ensure compliance for the chosen manylinux version" without Docker
  (zig from PyPI, `ziglang`, or `maturin[zig]`), and `--compatibility` to choose the tag. maturin's Metadata page
  does **not** mention PEP 639 (`license-files`, `License-Expression`); ADR-45's claim rests on the changelog only.
  `pyproject.toml` has `maturin>=1.15,<2`; the VM venv (`.venv` at the root) has maturin 1.15.0.
- Decision:
  1. **Baseline `manylinux_2_28` x86_64** for the first release (RHEL / Oracle Linux 8 and later, Debian 10, Ubuntu
     20.04 and later), built with `maturin build --release --zig --compatibility manylinux_2_28` (exact flag spelling
     from `maturin build --help` of the pinned version), `ziglang` pinned in the build instructions. Not lower:
     manylinux2014 (glibc 2.17) serves only RHEL 7, which is out of support. Other platforms (aarch64, Windows,
     macOS) are not in the first release. **Q-9** asks the owner for the glibc of the ETL hosts; if it is below 2.28,
     T-13 builds `manylinux2014` instead (same command, other tag), nothing else changes.
  2. **maturin pinned exactly** in `[build-system] requires` (`maturin==<version>`, the version the metadata check of
     item 3 passed with; 1.15.0 if it passes); a bump repeats item 3.
  3. **The wheel and the sdist are checked, not trusted**: the wheel's `.dist-info/METADATA` has `Metadata-Version`
     2.4 or later, `License-Expression: MIT OR Apache-2.0`, `License-File: LICENSE-MIT`, `License-File:
     LICENSE-APACHE`, `Requires-Python: >=3.11`, `Requires-Dist: pyarrow>=15` and the README as description; both
     licence files are under `.dist-info/licenses/`. If the pinned maturin does not write `License-Expression`, the
     Implementer tries the newest maturin 1.x; if that fails too, it stops and notes it (no hand-edited metadata).
  4. **Contents**: the wheel holds only `oradump/` (the Python files, `_core.pyi`, `py.typed`, the extension module)
     and `oradump-<v>.dist-info/`; never `exp_survey.py`, `corpus/`, `tests/`, `docs/`, a `.dmp` or a `truth_*.txt`.
     The sdist holds what the build needs (`Cargo.toml`, `Cargo.lock`, `crates/oradump-core/src`, its `data/`,
     `crates/oradump-py`, `python/oradump`, `pyproject.toml`, `README.md`, the licence files) and none of the above;
     both listings go into the T-13 note.
- Consequences: T-13's "Done when" (plan) is this list plus ADR-45 and ADR-47; the wheel tag is checked by
  installing it into a fresh venv on the VM (the VM's newer glibc runs it; a run on an actual glibc-2.28 host is
  the owner's acceptance step, not an agent's).
- Rejected: keeping `manylinux_2_34` (refuses RHEL 8 hosts with "no matching distribution" and nothing tells the
  owner why); a manylinux Docker image (agents do not run Docker, `CLAUDE.md` rule 3 and the charter); an unpinned
  maturin range (the PEP 639 output is the very thing that may change between releases).

## ADR-49 cargo-deny: the CC0 crate is a wasm32-only dependency; the check covers the targets we build - decided (session 12)
- Context (T-13 P1): `cargo deny check licenses` (cargo-deny 0.20.2) rejects `tiny-keccak 2.0.2` (`CC0-1.0`), reached
  through `arrow-array -> ahash 0.8.12 -> const-random -> const-random-macro -> tiny-keccak`. Checked in session 12:
  `arrow-array 59.3.1`'s `Cargo.toml` declares `ahash` per target: `cfg(not(target_arch = "wasm32"))` with
  `default-features = false, features = ["runtime-rng"]`, and `cfg(target_arch = "wasm32")` with
  `features = ["compile-time-rng"]`; only the second pulls `const-random`. `cargo tree -i tiny-keccak` prints nothing for
  the host, `x86_64-pc-windows-msvc` and `aarch64-apple-darwin`; only `--target all` shows the path. cargo-deny looks at
  every target unless `[graph] targets` is set (cargo-deny book, configuration page: "if none of the listed targets
  satisfy the target constraint, the dependency link is ignored"). So `tiny-keccak` is never compiled into anything we
  build, and there is **no arrow feature** to switch off (the prompt's option (b) does not exist as a feature).
- Cost of each, one line:
  (a) per-crate exception `{ name = "tiny-keccak", allow = ["CC0-1.0"] }`: 3 lines and this record; it allows a licence
  for a crate we never compile, and would stay silent if that crate one day reached a real target.
  (b) feature off: not available; only a `[patch]` of `arrow-array` would do it, a fork of a dependency.
  (c) `[graph] targets`: 9 lines; the list must follow the build matrix (a new target is a new line).
- Decision: **(c)**. `deny.toml` gets, below its header comment:

      [graph]
      # the targets we build now or in T-16 (ADR-49); a target added to the build is added here first
      targets = [
          "x86_64-unknown-linux-gnu",
          "aarch64-unknown-linux-gnu",
          "x86_64-pc-windows-msvc",
          "x86_64-apple-darwin",
          "aarch64-apple-darwin",
      ]

  The allow-list and the "no `exceptions` without a decision record" rule of ADR-45 item 5 stay. Verified before
  deciding: a scratch copy of `deny.toml` with this block passes (`cargo deny --config <copy> check licenses`:
  `licenses ok`); so does a copy with the exception of (a). Task: **T-13f** (Implementer, small). If a dependency ever
  brings a CC0 crate into one of these targets, the check fails again and the Architect decides; (a) is the
  prepared answer for a public-domain dedication like CC0.
- Consequences: the check describes the binaries we ship; T-16 needs no change to `deny.toml` (its five triples are
  listed already); `cargo deny check licenses` runs once per CI run, not per platform.
- Rejected: (a) as the default (the owner's suggestion; it whitelists something that is not in the product); (b)
  (no feature exists); listing only the host target (T-16's targets would go unchecked).

## ADR-50 Version `0.1.0a1`, release checklist, README install text - decided (session 12; publication on PyPI needs the project owner, Q-10)
- Context (T-13 P7): the version is `0.1.0` in `Cargo.toml` (`[workspace.package]`) and, separately, in
  `pyproject.toml` (`[project] version`); the README says "alpha (0.1.0a1 when released)" and shows `pip install
  oradump`, but nothing is published, and nobody has checked that the name is free. Checked in session 12 on a
  scratch crate: Cargo refuses `version = "0.1.0a1"` ("unexpected character 'a' after patch version number") and
  accepts `0.1.0-alpha.1`; maturin 1.15.0 with `dynamic = ["version"]` turns `0.1.0-alpha.1` into
  `vtest-0.1.0a1.tar.gz`, a wheel `vtest-0.1.0a1-...whl` and `Version: 0.1.0a1` in `METADATA` (maturin.rs "Metadata":
  the version "converted from SemVer to PEP 440 format" when `version` is in `project.dynamic`).
- Decision:
  1. **One source of the version**: `[workspace.package] version = "0.1.0-alpha.1"` in the root `Cargo.toml`
     (SemVer; the crates inherit it); `pyproject.toml` drops `version` and lists `dynamic = ["version"]`; the
     Python package is `0.1.0a1` (PEP 440). `Cargo.lock` follows. Set now, in **T-13f**, so every build until the
     release carries the release's version. "Set `0.1.0a1` in all `Cargo.toml` files" as the prompt words it is not
     possible (Cargo refuses it); this is its equivalent.
  2. **`tests/check_dist.py` loses the literal `0.1.0`**: it takes the version from the wheel's file name and asserts
     that it equals the wheel's `Version:`, the sdist's `PKG-INFO` `Version:`, the `.dist-info` folder name, the
     sdist's root folder name, and the PEP 440 form of the root `Cargo.toml` version. Conversion in the script:
     `X.Y.Z` -> `X.Y.Z`, `X.Y.Z-alpha.N` -> `X.Y.ZaN` (the one form verified above); any other pre-release suffix
     fails with "conversion not verified" until a record verifies it. The wheel name must match
     `oradump-<v>-cp311-abi3-<platform>.whl` (T-16 reuses the script on every platform).
  3. **No crates.io for now**: `oradump-core` gets `publish = false` (as `oradump-py` has); a crate release needs
     its own decision (API review, `repository`, docs).
  4. **README** (T-13f, exact text below): the status line and the "Install" section no longer show `pip install
     oradump`; they name the wheel file and the source build; a section "Other platforms" holds ADR-53 item 6.
     `pip install "oradump==0.1.0a1"` replaces it only after an upload to PyPI (Q-10), in the commit after the
     upload.
  5. **Tag**: annotated `v0.1.0a1` (the PEP 440 spelling users see) on the commit that was built and checked, made by
     the owner (step 8 of the checklist); no agent tags.
  6. **Release checklist**: `implementation-plan.md`, "First release", "Release checklist (session 12)". It includes
     the name check on PyPI and crates.io as an owner step before the tag.
  7. After the release, the task that prepares the next release sets the next version (`0.1.0-alpha.2` or a
     beta); between releases the version is not touched.
- README text (exact; T-13f replaces the status line and the "Install" section with it and adds "Other platforms"
  after "Install"):

      Status: alpha, version 0.1.0a1, not yet published on PyPI. Python 3.11 or later, pyarrow 15 or later.
      Wheels: Linux x86_64 only (`manylinux_2_28`: RHEL / Oracle Linux 8 and later); see "Other platforms".

      ## Install

      oradump is not on PyPI yet: `pip install oradump` does not install this project. Install the wheel file of a
      release, or build from source.

      From the wheel file (Linux x86_64, glibc 2.28 or later):

          pip install oradump-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl

      From source (Python 3.11 or later, and Rust 1.99 or later on `PATH`; pip fetches maturin for the build):

          pip install oradump-0.1.0a1.tar.gz     # the source distribution
          pip install .                          # or in a clone of the repository

      Only pyarrow (15 or later) is required. pandas is optional (`Table.to_pandas()`).

      ## Other platforms

      Wheels for Windows, macOS and other Linux systems are planned, not part of this release. Until then:

      - Read the dump on a Linux x86_64 host and write Parquet there (`pyarrow.parquet.write_table`); pass
        `interval_as="string"` when a table has INTERVAL columns, because pyarrow's Parquet writer refuses the
        native interval type. The Parquet files can then be read anywhere.
      - Or build from source as above. A source build on another platform is not tested; the test suite runs only
        from a clone of the repository (the source distribution contains neither the tests nor the test dumps).

      Python 3.11 or later is required on every platform.

- Consequences: T-13f changes `Cargo.toml`, `Cargo.lock`, `crates/oradump-core/Cargo.toml`, `pyproject.toml`,
  `tests/check_dist.py`, `README.md`, `CONTRIBUTING.md` (its release commands use `target/wheels/oradump-*.whl` /
  `oradump-*.tar.gz`, not a fixed version). The wheel becomes `oradump-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl`.
- Rejected: two version fields kept in step by hand (they drift; the check would catch it, but one source needs no
  check); `0.1.0` final with an "alpha" label only in the README (PEP 440 has a spelling for it; a final `0.1.0` on
  PyPI would be installed by a plain `pip install` before it deserves to be); tagging `v0.1.0-alpha.1` (users see
  `0.1.0a1`).

## ADR-51 The T-13 choices (P2 to P9) - decided (session 12)
- Context: the T-13 note's "Problems and deviations". P1 is ADR-49, P7 is ADR-50.
- Decision, one line each:
  1. **P2** an invalid option value raises the existing `UsageError` (`E-CONFIG`), no new `InvalidOption` class:
     **accepted**. The names in arch 3.4's `UsageError` parenthesis are kinds (codes and messages), not Python
     classes; Python accepts exactly `"native"` / `"string"`; the Rust `IntervalAs::from_str` that trims and
     lowercases is accepted for Rust callers (Python stays exact).
  2. **P3** `interval_as` is a field of `TypeOptions` (`Options.types.interval_as`), not of `Options` directly:
     **accepted**; it decides the Arrow type like `timestamp_unit`; Python's `open(..., interval_as=)` is unchanged.
  3. **P4** fallback text p = the digits the value needs (1 for zero, no minimum of 2), f = 9; on a table fallback one
     `W-DDL-FALLBACK` per INTERVAL column beside the table-level one: **accepted**; it is ADR-47's wording.
  4. **P5** the sdist omits `crates/*/tests/**`, so `cargo test` cannot run from it while the build does:
     **accepted**; the sdist is for building, the tests need `corpus/`, which is never shipped (ADR-45 item 6); the
     README says so (ADR-50). Cargo's "no documentation, homepage or repository" warning stays until the owner names a
     public repository URL (checklist step 2).
  5. **P6** maturin's default SBOM `oradump-<v>.dist-info/sboms/oradump-py.cyclonedx.json` in the wheel:
     **accepted**; it lists the Rust dependencies, sits inside `.dist-info` (ADR-48 item 4 holds) and costs nothing.
  6. **P8** `ruff format` also formats the README's Python block: **accepted**; the README example stays as ruff
     writes it, so `ruff format --check` stays green.
  7. **P9** (tmpfs `/tmp` nearly full, zig's linker warning): noted, no action; the zig warning is harmless.
- Consequences: arch 3.4 says "`UsageError` (`E-CONFIG` for an invalid option value)"; the plan's T-13 text is
  history.
- Rejected: a new `InvalidOption` class now (one more class to bind and document for no caller that needs it).

## ADR-52 `.gitattributes`: Git never converts the test data - decided (session 12)
- Context: the fixtures' `SHA256SUMS` cover the `.dmp` and `truth_*.txt` files; tests compare `.log` files, golden
  `.tsv` files and truth files byte for byte or line for line. There is no `.gitattributes`. `git ls-files --eol`
  (session 12): 235 files (the `.dmp`) are detected as binary, 331 text files are stored and checked out with LF.
  On a Windows checkout with line-ending conversion enabled (Git's `core.autocrlf`), Git would write CRLF into the
  text files and the checksums and golden comparisons would fail (T-16).
- Decision: a `.gitattributes` at the repository root, exact text:

      # Test data is compared byte for byte (SHA256SUMS, golden files): Git never converts it (ADR-52).
      *.dmp                                  binary
      corpus/fixtures/**/*.txt               -text
      corpus/fixtures/**/*.log               -text
      corpus/fixtures/**/SHA256SUMS          -text
      tests/golden/*.tsv                     -text
      crates/oradump-core/tests/data/*.tsv   -text

  `-text` (no conversion in either direction; gitattributes(5)) rather than `eol=lf` (which normalises on commit and
  so could change bytes when a file is re-added); `binary` (`-text -diff`) for dumps. Checked in session 12 with this
  text as a scratch attributes file: `git check-attr text` gives `unset` for a `.dmp`, a truth file, a `.log`, a
  `SHA256SUMS` and both `.tsv` folders, `unspecified` for `README.md`.
- Owner and time: the **Implementer**, in **T-13f** (now: cheap and without effect on Linux). It is a root file like
  those of ADR-45; the Q-5 text (`open-questions.md`) adds it to the Implementer's row. It changes no file under
  `corpus/`. Done when: `git status` is clean after adding it (no renormalisation diff), `git ls-files --eol` shows
  `attr/-text` for the patterns, `sha256sum -c` passes in the three fixture folders.
- Rejected: `eol=lf` (see above); `* -text` for the whole repository (source files may well be normalised; only
  test data must not be).

## ADR-53 T-16 "More platforms"; the workarounds until then - decided (session 12; the CI service needs the project owner, Q-11)
- Context: Q-9 is answered (owner): the first release is **RHEL 8+ / x86_64 only**, `manylinux_2_28` (ADR-48). Other
  users need other wheels later. The repository has no CI configuration; its remote is a self-hosted Git server.
  The VM is Linux x86_64 and cannot run Windows or macOS tests.
- Decision:
  1. **T-16 is not needed for 0.1.0a1.** It comes after T-15 and Reviewer session 4.
  2. **Matrix** (one `abi3-py311` wheel per platform, built natively on each runner unless stated):
     Linux x86_64 `manylinux2014` (glibc 2.17) with `maturin build --release --zig --compatibility manylinux2014`;
     Windows x86_64; macOS arm64; macOS x86_64; optionally Linux aarch64 `manylinux_2_28`. Python 3.11 and the newest
     CPython release, each with pyarrow 15 where a pyarrow 15 wheel exists for it and with the newest pyarrow.
  3. **Every platform runs the full suites**, not only the build: fixture checksums first (a Python step, since
     `sha256sum` is not on every runner), then `cargo nextest run`, then the wheel built, `tests/check_dist.py` on it
     and the sdist, the wheel installed into a fresh venv and the full `pytest` against the installed wheel.
     `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo deny check licenses` (ADR-49), `ruff`,
     `mypy` run once, on Linux.
  4. **Acceptance rule**: a platform is **supported** (named in the README, its wheel released) only when on that
     platform the checksums pass and the full `cargo nextest run` and full `pytest` (against the installed wheel)
     pass, with no test skipped because of the platform unless a decision record allows that one test. A platform
     that only builds is "builds, not supported" and its wheel is not released.
  5. The Linux x86_64 `manylinux2014` wheel **replaces** the `manylinux_2_28` one once accepted (it installs on both
     kinds of host); if it fails, `manylinux_2_28` stays.
  6. **Workarounds until T-16** (README "Other platforms", ADR-50): read the dump on a supported host and write
     Parquet (with `interval_as="string"` for INTERVAL columns); or build the sdist with a Rust toolchain of the
     crates' `rust-version` (1.99) or later, untested; Python 3.11 or later in every case.
  7. **Risks to check first, in this order** (T-16's first step is a run that only checks them):
     (a) **line endings and paths**: the checksum step on a Windows checkout (ADR-52); tests that build paths as
     strings with `/` or compare message texts that contain a path; Rust tests that read a text file and split on
     `\n` (a `\r` would survive); Windows file locks (a file open in a `Dump` cannot be replaced or deleted: the
     `FileChanged` tests and temporary copies);
     (b) **glibc** for `manylinux2014`: the Rust toolchain's own minimum glibc for `x86_64-unknown-linux-gnu`
     (Rust's platform-support page, read for the pinned toolchain) and any dependency that links a newer symbol;
     the tag is confirmed with `auditwheel show` (PyPA) on the built wheel, not taken from the file name;
     (c) **the abi3 tag per platform**: every wheel must be `cp311-abi3-<platform>`; the extension module's file
     name differs per platform (`.abi3.so`, `.pyd`), so `check_dist.py` accepts exactly one extension module file
     per wheel, whatever its suffix.
  8. The CI configuration is a root file of the **Implementer** (after the Q-5 edit); the service is the owner's
     choice (Q-11). A Reviewer session reads T-16 before its wheels are released.
- Consequences: plan section T-16; `deny.toml` already lists the five triples (ADR-49); `check_dist.py` is made
  version-agnostic in T-13f and platform-agnostic in T-16.
- Rejected: cross-building Windows / macOS wheels on the VM and calling them supported (nothing would run their
  tests); `universal2` macOS wheels (one wheel, but two architectures to test in one artefact; separate wheels keep
  one test run per wheel); T-16 before the alpha (the owner's hosts are RHEL 8+ x86_64).

## ADR-54 The project is renamed `oradmp` - decided (owner, Q-10 name part; session 13)
- Context: the PyPI name `oradump` belongs to another project (owner, Q-10). The owner chose **`oradmp`** ("dmp"
  covers the legacy EXP format and, later, Data Pump) and checked that it is free on PyPI (2026-10-18). Nothing is
  released or uploaded yet (ADR-50), so no user, script or published name depends on the old one. Whether 0.1.0a1 is
  uploaded to PyPI is still open (Q-10, upload part).
- Decision:
  1. **One name everywhere** (the owner's default, confirmed; no reason found to keep any part different):
     distribution `oradmp` (wheel `oradmp-<v>-cp311-abi3-<platform>.whl`, sdist `oradmp-<v>.tar.gz`, `.dist-info`
     `oradmp-<v>.dist-info`); import package `oradmp` (folder `python/oradmp/`, extension `oradmp._core`, maturin
     `module-name = "oradmp._core"`; the Rust `[lib] name = "_core"` is unchanged); command `python -m oradmp survey`;
     crates `oradmp-core` / `oradmp-py` in `crates/oradmp-core/` / `crates/oradmp-py/` (Rust paths `oradmp_core::`;
     `publish = false` on both, the crates.io names are neither checked nor needed, ADR-50 item 3); exception and
     warning base classes **`OradmpError`** and **`OradmpWarning`** (every subclass name is unchanged: `FormatError`,
     `UnsupportedError`, ...); Arrow field-metadata keys **`oradmp:type_code`**, `oradmp:oracle_type`,
     `oradmp:declared`, `oradmp:max_bytes`, `oradmp:charset`, `oradmp:lob_charset`, `oradmp:number_overflow`,
     `oradmp:number_scale`, `oradmp:conversion` (same values); the survey's header line `oradmp survey`; the SBOM that
     maturin writes becomes `oradmp-<v>.dist-info/sboms/oradmp-py.cyclonedx.json` by itself. **No alias**: no
     `oradump` module, no `OradumpError = OradmpError`, no old metadata key beside the new one.
  2. **README and PyPI description**: the README is the PyPI long description (`readme = "README.md"`), so one text
     serves both. Its title is `# oradmp`; the next paragraph is this one line (exact):

         oradmp (not oradump, which is an unrelated project on PyPI): "dmp" as in Oracle dump files, legacy exp now and Data Pump later.

     This is the **only** line of the README, the wheel's `METADATA` and the sdist's `PKG-INFO` that may contain the
     old name. The affiliation paragraph stays, with the name changed: "oradmp is an independent project. It is not
     affiliated with, endorsed by or sponsored by Oracle Corporation. ..." The short `description` in
     `pyproject.toml` (PyPI's summary) is unchanged; it holds no name.
  3. **Clean room**: no agent reads the source code or the documentation of the project `oradump` by `elessarelfstone`
     (github.com/elessarelfstone/oradump, PyPI `oradump`); nobody fetches its pages, **installs it** (`pip install
     oradump` would download it: the "old import fails" check of T-13g is done without installing anything named
     `oradump`) or copies from it. It may be a dump parser, so `CLAUDE.md` rule 1 applies to it in full. The owner adds
     one sentence to rule 1 (text in item 6); the next Scout session records it in `docs/research/sources.md` as
     "excluded, not read".
  4. **The repository folder** on the owner's machine (`~/p/oradump`) is the owner's business; nothing in the task
     depends on it. Path names inside the repository change; the checkout folder does not.
  5. **Old text** (policy):
     - **renamed** (the Implementer, T-13g): all code, tests, generators' docstrings, packaging and root files
       (`Cargo.toml`, `Cargo.lock` by Cargo, `pyproject.toml`, `.gitattributes`, `README.md`, `CONTRIBUTING.md`),
       folders by `git mv` so history follows;
     - **updated** (live documents): `CLAUDE.md`, `.claude/agents/*.md` and the live prompt template
       `docs/prompts/implementer-task.md` by the **owner**; `exp-reader-architecture.md` (body), the live parts of
       `implementation-plan.md` (T-16, "First release" and its checklist, the new order) and `open-questions.md` (Q-10)
       by the **Architect** (done in session 13);
     - **kept as written** (historical): the records before ADR-54 in this file, the "Changes in session n" lists and
       the done tasks of the plan, `implementer-notes.md` (session reports), `docs/review/`, `docs/briefs/`,
       `docs/research/` (the Scout's history; its "OraDump Export Kit" / "OraDump-to-CSV" rows name other, commercial
       products), the prompts of finished sessions in `docs/prompts/`, git history. The line at the top of this file
       says so.
     - **owner's choice, not required**: `corpus/scripts/config.sh` (`ORADUMP_HOME`, a variable of the fixture
       scripts only), `corpus/scripts/make_ru.sh`, `corpus/scripts/order_of_use.sh`, `corpus/sql/01_corpus_tables.sql`
       (comments and the default checkout path), the editor file `oradump.code-workspace`. None of them is shipped or
       read by a test; no fixture, truth file or golden file contains the name (`grep -rlai oradump corpus/ tests/golden
       crates/oradump-core/tests/data`, session 13: only the four corpus files above and the two generator scripts).
  6. **Owner action before T-13g** (O-13): in `CLAUDE.md` replace `oradump` by `oradmp` in the title, the layout lines
     (`crates/`, `python/`), rule 7 (`crates/oradmp-core/tests/data/`) and the Implementer's row
     (`oradmp-core`, `oradmp-py`, `python/oradmp/`); add to rule 1: "This includes the project `oradump` by
     `elessarelfstone` (PyPI `oradump`): nobody reads its source or documentation, installs it or copies from it."; the
     same replacement in the five `.claude/agents/*.md` and in `docs/prompts/implementer-task.md`. Rule 1's sentence
     and the README line are then the only live mentions of the old name.
  7. **`__version__`** (T-13f note, problem 1): `oradmp.__version__` is `importlib.metadata.version("oradmp")`
     (Python standard library); if the distribution metadata is missing (package folder copied without installing)
     it is `"0+unknown"` (a valid PEP 440 local version that no release has). A pytest asserts it equals the installed
     metadata and the PEP 440 form of the root `Cargo.toml` version (ADR-50 item 2's conversion, imported from
     `tests/check_dist.py`) and is not `"0+unknown"`.
  8. **CONTRIBUTING** (T-13f note, problem 2): the release commands run in an activated `.venv`; `zig` was found only
     with `.venv/bin` on `PATH`, and `.venv` (made by `uv`) has no `pip`, so packages are installed with `uv pip`.
  9. **The work is task T-13g** (plan), one Sonnet session, alone; the release name step of the checklist becomes
     "`oradmp` is free on PyPI (owner checked on 2026-10-18); it is secured only by the first upload".
- Inventory (session 13: `grep -rIci oradump . --exclude-dir={target,.venv,.git}`, 90 files with the name, plus
  `find -iname '*oradump*'`; I = Implementer in T-13g, O = owner, A = Architect):

  | kind of place | files | action | who |
  |---|---|---|---|
  | crate folders `crates/oradump-core/`, `crates/oradump-py/` | 2 folders | `git mv` to `crates/oradmp-core/`, `crates/oradmp-py/` | I |
  | `Cargo.toml` (members), `crates/*/Cargo.toml` (`name`, `description`, the path dependency), `Cargo.lock` | 4 | rename; lock regenerated by Cargo (diff: two package names only) | I |
  | Rust sources `src/` (`oradump_core::` paths, doc comments, `#[pyclass(module = "oradump._core")]`, `py.import("oradump._errors")`, field-metadata keys `oradump:` in `reader.rs`) | 8 (core 3, py 5) | rename; keys to `oradmp:` | I |
  | Rust tests (`tests/*.rs`, `tests/common/*.rs`: crate paths, expected metadata keys and messages) | 30 | rename | I |
  | generator scripts `tests/data/gen_walk_expected.py`, `gen_number_parts.py` (docstring paths) | 2 | rename the paths; re-run both, outputs byte-identical | I |
  | golden / test data (`tests/golden/*.tsv`, `crates/*/tests/data/*.tsv`) | 0 with the name | moved with their folder only; never edited | I |
  | `pyproject.toml` (`name`, `manifest-path`, `module-name`, comment, ruff `extend-exclude`, mypy `files`); no entry points (`python -m` only) | 1 | rename | I |
  | Python package `python/oradump/` (6 files with the name, plus `py.typed`) | 1 folder | `git mv` to `python/oradmp/`; delete the untracked rest (`_core.abi3.so`, `__pycache__`) | I |
  | exception / warning classes `OradumpError`, `OradumpWarning` (`_errors.py`, `__init__.py`, `_api.py`, stubs, tests, README) | in the rows above and below | `OradmpError`, `OradmpWarning`; no alias | I |
  | pytest files (`conftest.py`, 8 `test_*.py`) | 9 | rename imports, class names, keys, survey header | I |
  | `tests/check_dist.py` (wheel / sdist / `.dist-info` names, required sdist paths, the README title check) | 1 | rename; add the old-name scan (T-13g) | I |
  | built artefacts (`.so` inside the wheel, wheel / sdist names, SBOM) | built | follow from the rows above; old `target/wheels/oradump-*` deleted | I |
  | dev install in `.venv` (`oradump.pth`, `oradump-0.1.0.dist-info`) | not in the repository | `uv pip uninstall oradump`, then `maturin develop` | I |
  | environment variables | 0 in code and tests; `ORADUMP_HOME` in `corpus/scripts/config.sh` | owner's choice | O |
  | messages (survey header `oradump survey`, argparse `prog`) | in `_survey.py` | rename | I |
  | `README.md` | 1 (14 lines) | rename; title and the one line of item 2; install lines `oradmp-0.1.0a1-...` | I |
  | `CONTRIBUTING.md` | 1 (3 lines) | rename; activated `.venv` (item 8) | I |
  | `deny.toml`, `LICENSE-*` | 0 | none | - |
  | `.gitattributes` (`crates/oradump-core/tests/data/*.tsv`) | 1 | path renamed | I |
  | `CLAUDE.md`, `.claude/agents/*.md`, `docs/prompts/implementer-task.md` | 1 + 5 + 1 | item 6 | O |
  | prompts of finished sessions (`docs/prompts/`) | 5 | keep | - |
  | `docs/review/`, `docs/briefs/`, `docs/research/` | 2 + 2 + 2 | keep (historical; other products' names in `sources.md`) | - |
  | `docs/design/`: `decisions.md` (records before ADR-54), `implementer-notes.md` | 2 | keep; top line of this file | A |
  | `docs/design/`: `exp-reader-architecture.md` (body), `implementation-plan.md` (live parts), `open-questions.md` (Q-10) | 3 | updated in session 13 | A |
  | `corpus/scripts/` (3), `corpus/sql/01_corpus_tables.sql`, `oradump.code-workspace` | 5 | owner's choice | O |

- Consequences: T-13g before Reviewer session 3, so the review reads the renamed tree and the release is built once,
  under the new name. Every later document uses `oradmp`. The old name stays in history and in exactly two live lines
  (`CLAUDE.md` rule 1, the README line). The name is free but **not secured** until the first upload (risk 18).
- Rejected: keeping the crate names `oradump-*` (nothing published; mixed names cost every reader a mapping and keep
  the old name in the wheel's SBOM and in panic paths inside the `.so`); keeping the import name `oradump` with
  distribution `oradmp` (the name users type would differ from the name they import, and another distribution may
  use the same import name; not checked, by item 3); aliases for the
  old class names or metadata keys (no user depends on them; they keep the old name in the wheel); renaming the
  historical documents (it would falsify what each session saw and wrote).

## ADR-55 The Arrow C stream path keeps the read's warnings and row count (review 03 S-1) - decided (session 14; amended in session 15, ADR-61)
- Context: `BatchReader.__arrow_c_stream__` takes the core reader out of the binding, stashes `rows_read` and the
  warnings **before** the export and hands the reader to `pyo3_arrow::PyRecordBatchReader` (`crates/oradmp-py/src/lib.rs`
  `arrow_c_stream`). Everything the consumer then reads is invisible: review 03 read `18c/t_num_exp_18c.dmp` T_NUM with
  `on_value_error="null"` through `pyarrow.RecordBatchReader.from_stream(r).read_all()`: 33 rows, `r.warnings == []`,
  `r.rows_read == 0`, nothing issued, while the iterator path gives `W-VALUE-NULLED` (10 values) and
  `W-NUMBER-ROUNDED` (4 values). With the default `on_number_overflow="round"` this path changes values silently,
  against arch 3.4 / ADR-33 item 3 ("warnings are part of the result"); the README advertises the path.
- Decision:
  1. **One shared state.** The binding's `_Reader` keeps the core `TableBatches` behind an `Arc<Mutex<..>>` (std).
     `arrow_c_stream` hands `PyRecordBatchReader::new` a small binding type (`SharedBatches`) that holds a clone of
     that `Arc` and implements `Iterator<Item = Result<RecordBatch, ArrowError>>` and `arrow_array::RecordBatchReader`
     (+ `Send`) by calling the same `TableBatches::next_batch` under the lock, inside the panic guard of ADR-59.
     `BatchReader.rows_read` and `.warnings` read the shared state, so they follow the consumer's reads. No stash.
  2. **Values while the stream is partly consumed**: `rows_read` = rows of the batches the core has handed to the
     stream so far (a consumer may hold some of them unprocessed); `warnings` = the warnings raised so far. The
     counted warnings (`W-NUMBER-ROUNDED`, `W-NUMBER-NULLED`, `W-VALUE-NULLED`, ...) are **complete only once the
     stream has returned its end or an error**: the rule of the iterator path (arch 3.4), unchanged.
  3. **Issue through `warnings`**: `BatchReader.close()` and `__exit__` issue every warning not yet issued (on both
     paths; the existing `_issued` counter makes each one issued once). The iterator path keeps issuing at its end.
     Nothing is issued from inside the stream: the consumer calls it from C, maybe from another thread, and a
     warnings filter set to `"error"` could not raise there.
  4. **After the export** the reader is spent for iteration: `__next__` and `read_all()` raise `UsageError`
     (`E-CONFIG`, "the reader was exported to an Arrow stream"); a second export stays `E-CONFIG`. Today `__next__`
     after an export silently stops: that is a short table without an error and goes.
  5. **`close()` while the stream is alive** drops the core reader (the file handle) and keeps the counts; the
     stream's next pull returns an error whose message starts `[E-CONFIG` ("the reader is closed"), never a silent
     end. `Dump.close()` does not affect a reader (arch 3.4, unchanged).
  6. **An error inside the stream** reaches the consumer as the `ArrowError` text that starts with the code and the
     offset (arch 3.4); `BatchReader.warnings` is then complete. A typed exception object for a stream error
     (`BatchReader.error`) is not in the alpha.
  7. **Locks**: no code holds the shared mutex while it calls Python. The Python-facing methods (`warnings`,
     `rows_read`, and `Dump.info`, `Dump.warnings`, the second lock of `scan_inner`, N-2 of review 03) take the lock
     inside `py.detach`, copy the data out, release the lock, then build Python objects. So a thread that asks for
     the warnings while another thread pulls a batch waits without holding the GIL.
  8. **README**: the stream example shows the order: `with dump.read_table("EMP") as r: t =
     pyarrow.RecordBatchReader.from_stream(r).read_all()`, then `r.warnings` (issued at the end of the `with`).
- Tests (T-13h (a)): through `from_stream(...).read_all()` on 18c T_NUM, `on_value_error="null"`: `rows_read == 33`,
  the warning list equal (codes, counts, offsets) to the iterator path's on the same table **and** to the literal
  counts of `test_warnings.py` (10 nulled, 4 rounded; truth_num); issued exactly once at the end of the `with`, with
  the test file as `filename`; with `batch_rows=5` and one `read_next_batch()`: `rows_read == 5`; `close()` before the
  first pull: the pull raises a pyarrow exception whose text starts `[E-CONFIG`; `next(r)` after the export raises
  `oradmp.UsageError`. `test_the_arrow_c_stream_spends_the_reader` no longer asserts `rows_read == 0`. The README
  names only `from_stream` as a consumer; any consumer the README adds later gets the first test too.
- Consequences: the C stream path and the iterator path report the same; a consumer that never closes the reader
  still finds the warnings in `r.warnings`. The GIL behaviour of pyarrow while it pulls from the capsule stays
  unassumed (arch 3.4); T-13h records a two-thread measurement on this path, Reviewer 4 checks it.
- Rejected: documenting the path as "no warnings" and requiring `strict` there (a silent value change on the
  advertised path, with the default options); issuing warnings from inside the stream (thread and filter problems
  above); a `pyarrow.RecordBatchReader` wrapper in Python that pulls batch by batch (copies the stream protocol
  in Python and loses zero-copy for other consumers).
- **Amendment (session 15, ADR-61)**: items 5 and 6: the stream's error text **contains** `[E-CONFIG` / the code and
  the offset; arrow-rs puts `External error: ` before it (pyarrow raises `pyarrow.ArrowInvalid`), so it does not start
  with `[E-`; a consumer finds the code by the pattern `\[(E-[A-Z0-9-]+)`. Item 5 extends to the iterator: after
  `close()`, `next(r)` and `read_all()` raise `UsageError` (`E-CONFIG`, "the reader is closed") unless the source had
  already returned its end (then the end again). Item 7's rule is checked by code review (ADR-61 item 7), not by a test.

## ADR-56 Release build: a clean clone at a neutral path, remapped paths, the SBOM kept (review 03 S-2, N-11) - decided (session 14; the `tools/` row needs the owner, O-14)
- Context: the T-13g wheel holds the builder's home path and user name: 6 `path+file://<home>/...` references in
  `oradmp-<v>.dist-info/sboms/oradmp-py.cyclonedx.json` and 161 `<home>/.cargo/registry/src/...` strings (panic
  locations of dependencies) in `_core.abi3.so` (review 03, section 5). `RUSTFLAGS` with `--remap-path-prefix` for the
  registry and the checkout gave 0 such strings in the `.so` (review 03, tried) but leaves the SBOM. The SBOM
  `timestamp` makes every wheel build differ (N-11); the sdist is reproducible. A build from a plain copy (no `.git`)
  puts `__pycache__/*.pyc` into the wheel (T-13g note, Note 2).
- What maturin 1.15.0 offers (checked this session): the maturin user guide (maturin.rs, "Configuration" and "SBOM"
  pages, current version) documents `[tool.maturin.sbom]` with `rust` (CycloneDX SBOM of the Rust dependency tree,
  default on), `auditwheel` and `include`; `rust = false` turns the Rust SBOM off. `maturin build --help` of the
  pinned 1.15.0 shows `--sbom-include`, `-o/--out`, `--locked`; the 1.15.0 binary contains the `SbomConfig` keys and
  the strings `SOURCE_DATE_EPOCH` and "Invalid reproducible build timestamp" (its bundled `cargo-cyclonedx` 0.5.9).
  **Not verified**: that `rust = false` works in 1.15.0, and whether the SBOM `timestamp` follows `SOURCE_DATE_EPOCH`.
- Options and their cost: (a) drop the SBOM (`[tool.maturin.sbom] rust = false`): one line, but the wheel loses
  its machine-readable dependency list, which corporate scanners read (PEP 770); (b) rewrite the SBOM after the
  build: unzip, edit JSON, recompute `RECORD` hashes, re-zip: a hand-made archive step no tool vouches for; (c) build
  from a clean clone in a neutral folder outside the home directory, with path remaps: one clone and one full build
  per release (minutes), no root, no container; (d) a container: the cleanest, but `docker` is outside what the agents
  may run, and the owner's step would grow.
- Decision:
  1. **(c)**. The release is built by **`tools/release_build.sh`** (written by the Implementer in T-13h; the owner or
     the Implementer runs it): a fresh `git clone --no-local` of the commit to release into
     `$ORADMP_BUILD_ROOT/src` (default `ORADMP_BUILD_ROOT=/var/tmp/oradmp-release`; the script refuses a root that
     contains `$HOME` or the user name as a path component, or that exists already), its own venv
     (`uv venv`; `uv pip install "maturin==1.15.0" ziglang`, `PATH` with the venv's `bin`),
     `CARGO_TARGET_DIR=$ROOT/target`, `RUSTFLAGS="--remap-path-prefix=$CARGO_HOME=/cargo
     --remap-path-prefix=$ROOT=/build"` (`CARGO_HOME` defaulting to `$HOME/.cargo`), `SOURCE_DATE_EPOCH` = the commit
     time (`git log -1 --format=%ct`), then `maturin build --release --locked --zig --compatibility manylinux_2_28 --out
     $ROOT/dist` and `maturin sdist --out $ROOT/dist`, then `tests/check_dist.py` of the clone on both files; it writes
     `$ROOT/dist/SHA256SUMS` and prints the commit, the versions of rustc, maturin and zig, and the two hashes. Exit
     status non-zero on any failure (`set -euo pipefail`). The SBOM then names `/var/tmp/oradmp-release/src/...`: no
     user, no home.
  2. **The SBOM stays** (option (a) is the fallback: if T-13h finds the SBOM still carries a home path from the
     neutral build, it sets `rust = false`, says so in its note, and this record is amended).
  3. **Reproducibility (N-11)**: not a release gate for 0.1.0a1. The sdist must stay byte-identical across builds
     (it is, review 03); for the wheel the script's option `--twice` builds a second time from scratch into
     `$ROOT/dist2` and prints whether the hashes agree and, if not, which members differ. T-13h records the result
     once. The released files are the ones whose SHA-256 the checklist records; nothing is rebuilt for the upload.
  4. **`check_dist.py`** is the safety net for anything the remaps miss: ADR-57.
  5. **Who owns `tools/`**: the Implementer (scripts for building and releasing; not shipped: ADR-57 rejects it in
     both archives). `CLAUDE.md` names no such folder, so the owner adds it to the Implementer's row (owner action
     **O-14**, before T-13h; the same edit adds `fuzz/` for T-14, arch 9). `CONTRIBUTING.md`'s "Release wheel" block
     becomes the one command `tools/release_build.sh` plus a sentence on what it does; the dev build stays as it is.
- Consequences: a release needs about 1 GB free below `/var/tmp` and one full build; the output never depends on
  the checkout's folder name or its untracked files. The wheel built in the working copy (`maturin build` by hand) is
  for tests only.
- Rejected: options (a), (b), (d) above; scanning for the user name in `check_dist` (the owner's user `max` occurs in
  `max_lob_bytes`: false hits; the path patterns catch the known leaks).

## ADR-57 `check_dist.py`: what the archives may not contain (review 03 S-3) - decided (session 14)
- Context: `tests/check_dist.py` passes a wheel with `oradmp/__pycache__/*.pyc` (with a neutral path and with a home
  path inside) and an sdist with `python/oradmp/corpus/x.txt`: only the first path component is tested against
  `corpus` / `tests` / `docs` (`check_dist.py:39`), `.pyc` is not tested, and the content scan looks only for the old
  name.
- Decision (T-13h (c)):
  1. **Paths**, wheel and sdist (after the sdist's root folder): a member fails if **any** component is `corpus`,
     `tests`, `docs`, `tools`, `__pycache__`, `target`, `.venv` or `.git`, or its name ends in `.pyc` or `.pyo`; the
     existing rules stay (`.dmp`, `truth_*.txt`, `exp_survey.py`, wheel members only under `oradmp/` and the
     `.dist-info`).
  2. **Contents**, every member of both archives, text and binary (`.so`, SBOM, `RECORD` included): a FAIL line per
     hit, with member, byte offset and line, for the byte strings `/home/`, `/Users/`, `/root/`, and,
     case-insensitive, `C:\Users\` and its JSON-escaped form `C:\\Users\\`. No exception list. A hit that the
     Implementer believes harmless is reported in the note, not allowed in the script.
  3. **A pytest** `tests/test_check_dist.py` builds small synthetic archives in `tmp_path` (names from the root
     `Cargo.toml` version through `check_dist.pep440`; the real archives are not needed) and asserts one FAIL per rule:
     a wheel with `oradmp/__pycache__/_api.cpython-313.pyc`; a wheel with `oradmp/x.pyo`; an sdist with
     `<root>/python/oradmp/corpus/x.txt`; an sdist with `<root>/tools/release_build.sh`; an SBOM member containing
     `path+file:///home/u/p/oradmp/crates/oradmp-py`; a binary member with `/Users/u/` inside; a JSON member with
     `C:\\Users\\u`; and a clean synthetic pair that passes. So each rule is a mutation that a test catches, not a
     one-off run.
- Consequences: a build from a plain copy (`.pyc`) or a working tree with stray folders fails before release; the
  remaps of ADR-56 are checked on every release.
- Rejected: an allow-list of members only (the wheel's member list changes with features; the deny rules name the
  failures we have seen and the ones that cost most); the user name (ADR-56).

## ADR-58 The message check sees the failing value: decoded forms and hex substrings (review 03 S-4) - decided (session 14; amended in session 15, ADR-61)
- Context: `t12_messages.rs` collects, per table, raw fields (a), 4-character windows of their lossy text (b), whole
  hex tokens (c) and the cells of a lenient read (d). The value that fails is NULL in a lenient read, so its decimal
  text is in no corpus: mutation d4b (the failing NUMBER's `to_text()` appended to `E-NUMBER-OVERFLOW`) failed only
  because the text contains `100000`, a cell of row 10; with `1E+33` it passes. d6 (hex of the first 8 bytes of a
  longer field) passed all 395 tests, against ADR-46 item 3. Today no message holds a value (review 03 section 1:
  messages are literals, piece numbers and set ids); the check guards the future.
- Decision (T-13h (d)):
  1. **(c) hex, stronger**: before the tests, a run of 4 or more two-digit hex pairs separated by single ` `, `:` or
     `-` is joined into one token. A token is a hit when it equals the hex of a field of 2 bytes or more (as now) **or**
     has 8 or more hex digits and is a substring of the hex of any field of the table (ADR-46 item 3's rule).
  2. **(e) decoded values, new**: for every non-NULL field of the table, by the column's type code, whatever the
     Arrow type, the read policy or the declared type: NUMBER / FLOAT (2) through `OraNumber::decode` and `to_text()`;
     DATE (12) through `decode_date_fields`; TIMESTAMP family (180, 181, 231) through the core's timestamp decoders;
     INTERVAL (182, 183) through the decoders' `to_text` (p = 9, f = 9, and the declared p / f when known). A field the
     decoder refuses adds nothing. Stored per table: the digit strings of the numbers (sign and point removed), their
     canonical form (digits without leading or trailing zeros, decimal exponent, sign), and the date, time and interval
     texts (`YYYY-MM-DD` with a sign for years below 1, `HH:MM:SS`, the interval text).
     A message hits (e) when it has (e1) a run of **6 or more** decimal digits that is a substring of a number's digit
     string; (e2) a number token with a point or an exponent (`1E+33`, `1e33`, `1.0E+33`, `-0.5`) whose canonical form
     equals a stored number's; or (e3) a stored date, time or interval text of 8 characters or more.
  3. **Thresholds and why**: 6 digits because messages may carry integers of at most 5 digits (piece and chunk
     numbers, set ids such as 873 / 2000, the literals 32000 / 32768); the test asserts this inventory: the longest
     digit run over all messages it sees is at most 5, so a message that ever needs a longer integer fails here
     first and needs a decision. 8 hex digits = 4 bytes, the threshold of the raw form (a). `message` carries no
     offset (offsets are in `Display` only, arch 7.4), so hex offsets do not collide. Accepted gaps (unchanged from
     ADR-46): values of 1 to 3 bytes, numbers of 1 to 5 digits without a point or exponent, a value transformed in a
     way none of the forms covers (e.g. reversed).
  4. **Self-test** (`the_check_reports_a_value_in_each_form`, extended): the real `E-NUMBER-OVERFLOW` message of 18c
     T_NUM with the value of truth_num row 19 (10^33) appended as `1000000000000000000000000000000000`, `1E+33`, `1e33`
     and `1.0E+33` reports form (e); a 6-digit window of row 22's digits (-12345678901234567890123456789012345678)
     reports (e), a 5-digit window does not; the hex of the first 8 bytes of the patched T_FRAME value
     (`4e4e4e4eff4e4e4e`) and `4e 4e 4e 4e ff` report (c); the date text of truth_date row 8 (year below 1, from the
     truth file, not retyped) appended to the real `E-BAD-DATETIME` message reports (e).
  5. **Mutations** (each alone on a scratch copy, T-13h note): d1, d2, d3, d4a, d5 still fail
     `no_message_of_the_fixtures_holds_a_value`; **d4b** (the failing value's `to_text()` in `E-NUMBER-OVERFLOW`) fails
     it with form (e) among the forms reported; **d4c** (new: the value as `1E+33`-style text, `format!("{:E}")` of its
     decimal or an equivalent) fails it with form (e); **d6** (hex of the first 8 bytes of a field longer than 8 bytes)
     fails it with form (c); **d7** (new: that hex with spaces between the pairs) fails it with form (c); **d8** (new:
     the decoded date text in `E-BAD-DATETIME`) fails it with form (e). The assertion message prints the forms found,
     so the note can show which form caught each mutation.
- Consequences: the check now sees the value that the lenient read hides; the corpus grows by one decode per field
  (a few seconds over the 201 files). The test needs the core's decoders public (they are: `values::number`,
  `values::datetime`, `values::interval`).
- Rejected: substrings of 3 characters of every form (messages hold 3- and 4-digit set ids and piece numbers: false
  hits); the known bad values of the truth files only (they cover the fixtures' failures, not a value a future message
  might print for another file).
- **Amendment (session 15, ADR-61 item 6)**: item 4's self-test is three tests in `t12_messages.rs`:
  `the_check_reports_a_value_in_each_form` (forms (a) to (d), and the hex forms of (c)),
  `the_check_reports_a_decoded_value` (form (e): row 19's four spellings, the 6- and 5-digit windows of row 22, the
  date text of truth_date row 8) and `the_helpers_of_the_decoded_forms`. A number token with an exponent inside a
  hex-looking text (`04e4`) is a form (e) hit; a token inside a word (`0x1e5`) is skipped.
- **Amendment (session 16, ADR-64 item 5, ADR-65 item 4)**: item 3's 5-digit inventory applies to the message without
  the exact `in_piece` suffix " (piece N of the value)" / " (chunk N of the value)" (a counter, pinned by a unit
  test); item 5's list gains **d9** (date as `DD.MM.YYYY`) and **d10** (number in 3-digit groups), caught by the
  widened (e3) date tokens and (e1) group joining.

## ADR-59 A panic in the core becomes `E-INTERNAL` (`InternalError`) at the binding (review 03 S-5) - decided (session 14; amended in session 15, ADR-61)
- Context: arch 7.3 says the binding converts a caught panic into `OradmpError`; the binding has no `catch_unwind`, so a
  panic reaches Python as PyO3's `PanicException`, a `BaseException` that `except Exception` does not catch. The
  workspace lints deny `unwrap`, `expect`, indexing, unchecked arithmetic and `panic!`, and T-14 fuzzes, so a panic is
  unlikely; but an ETL job must be able to catch every reader failure with `except oradmp.OradmpError`. Panics unwind
  (no `panic = "abort"` in the workspace). A panic that unwinds out of the Arrow C stream callback would cross an
  `extern "C"` boundary.
- Decision:
  1. New error code **`E-INTERNAL`** (`ErrorCode::Internal` in the core enum, so `ERROR_CODES` and the Python class
     table stay one list; the core never constructs it). Python class **`InternalError(OradmpError)`**. Message (fixed
     literal, no value, no panic payload, no location): "internal error: a panic in oradmp was caught; please report
     it with the oradmp version". No offset, table or row. Not affected by `strict` or `on_value_error`. Arch 7.1's
     code list and 7.3 say so.
  2. **Where**: one helper in the core, `oradmp_core::guard::catch_panic(f) -> Result<R>` (`std::panic::catch_unwind`
     with `AssertUnwindSafe`; no `unsafe`), used by the binding around every call into the core: the closure of
     `PyDump::with`, the reader's `next_batch`, `open`, `survey`, `digest`, and inside `SharedBatches::next` (ADR-55), so
     a panic in a stream pull becomes an `ArrowError` whose text starts `[E-INTERNAL` and never unwinds into C.
  3. **After a caught panic** the object is unusable: the binding drops the core `Dump` or reader it guarded and marks
     it; every later call on it raises `E-INTERNAL` again ("an earlier call failed with an internal error"). Its
     mutex may be poisoned; `lock()` already ignores poisoning, the mark decides.
  4. A panic in the binding's own Python-object code (outside the guard) stays PyO3's `PanicException`; 7.3 says so.
     The default panic hook still prints to stderr; the library does not replace the process's hook.
  5. **Tests without a panic path in production**: (a) core unit tests of `catch_panic` under `#[cfg(test)]` (a
     closure that panics gives `E-INTERNAL` with the literal; `Ok` and `Err` pass through unchanged); (b) a cargo
     feature **`test-probes`** of `oradmp-py`, off by default, adds `_core._probe_panic(kind)` for `kind` in `dump`,
     `reader`, `stream`: it runs a panicking closure through `PyDump::with`, through the reader's batch path, and
     through `SharedBatches` (made generic over a small batch-source trait whose panicking implementation exists only
     under the feature). `tests/test_internal.py` asserts `oradmp.InternalError`, `.code == "E-INTERNAL"`, the
     literal message, `isinstance(e, Exception)`, and `E-INTERNAL` again on the next call; the `stream` case runs in a
     subprocess (`sys.executable -c ...`) and asserts exit status 0 and a pyarrow error text starting `[E-INTERNAL`, so
     an abort fails the test instead of killing pytest. The tests skip, with that reason, when `_core` has no
     `_probe_panic` (a release build). The dev build is `maturin develop --features test-probes` (CONTRIBUTING).
     (c) `check_dist.py` fails when the wheel's `.so` contains the string `_probe_panic` (a release built with the
     feature).
  6. **Mutations** (T-13h note): the guard removed from `PyDump::with` fails `test_internal.py` (`dump`); removed from
     the batch path fails (`reader`); removed from `SharedBatches::next` fails (`stream`, the subprocess); `catch_panic`
     returning the payload text in the message fails its core unit test; a release build with `--features
     test-probes` fails `check_dist.py`.
- Consequences: `except oradmp.OradmpError` catches everything the reader raises except `E-IO` (an `OSError`, ADR-46
  item 8) and a panic in the binding's own glue. The README says so (N-6). A run of the installed release wheel
  skips the three probe tests (listed in the release note).
- Rejected: correcting 7.3 and keeping `PanicException` (an ETL job would need `except BaseException`, which also
  swallows `KeyboardInterrupt`); a global panic hook (changes the host process); `panic = "abort"` (kills the Python
  process).
- **Amendment (session 15, ADR-61 items 1, 5)**: item 2: the stream pull's error text **contains** `[E-INTERNAL` (after
  arrow-rs's `External error: `). Item 5 (b): the probe is `_core._probe_panic(kind, path=None)`; it needs a fixture
  path for `dump` and returns the object (a `_Dump`, whose feature-gated `_probe_panic()` panics through
  `PyDump::with`, or a `_Reader` over a panicking `BatchSource` for `reader` and `stream`); `SharedBatches` holds a
  `Box<dyn BatchSource>` instead of a generic parameter. Only with the feature `test-probes`.
- **Amendment (session 16, ADR-65 item 1)**: item 2's "every call into the core" reads: every call that reads or
  decodes the file or parses user input, i.e. the list of item 2 plus `NumberType::from_str` and `number_summary`.
  Exempt, by name: `Dump::info()`, `Dump::catalog()`, `Dump::warnings()`, `TableBatches::schema()`, `Error`'s
  `Display` in `to_pyerr`, the drops in both `close()`, and the `core_open` of the feature-gated `_probe_panic`.
- **Amendment (session 18, ADR-71 item 4; review 05 N-1)**: the exempt list also names, all panic-free by reading:
  `Warning`'s `Display` in `warning_dict` and `Error`'s `Display` in `error_instance` (`errors.rs`), `Charset::from_id`
  / `Charset::name` (`conv.rs`, a `match` on a `u16`), the `as_str` constants and `ALL` of `ErrorCode` /
  `WarningCode` (module init), and the implicit drops of `PyReader` / `PyDump` at garbage collection. No wrap; the
  `lib.rs` module comment is completed in T-13j.


## ADR-60 Review 03's notes N-1 to N-11; order after review 03 - decided (session 14)
- N-1 (deleting the one `assert!` of `every_table_of_the_exp_files_is_classified_and_read`): **closed**, accepted: a
  test cannot guard its own deletion; a reviewer reads the diff.
- N-2 (dump mutex taken under the GIL; `stacklevel` of `read_all`): **T-13h** (ADR-55 item 7; `_issue` gets the stack
  level from its caller so that `__next__`, `read_all()` and `close()` all point at the user's line; a test asserts
  `w.filename == __file__` for the three).
- N-3 (`oradmp:lob_charset` never written): **closed by a document change**: arch 5.1 no longer names
  `oradmp:lob_charset`; LOB text columns carry **`oradmp:charset`** with the set the values are decoded with
  (`AL16UTF16` for a CLOB with id 873 and for every NCLOB), which is what `reader.rs` writes. ADR-54 item 1's key list
  is read without it.
- N-4 (assertions that cannot fail): **T-13h**: `test_errors.py:259-266` compares `e.column` with the column name of
  T_ROWID taken from the golden catalog or `collist.py` (not with itself); `test_truth.py:77` compares
  `table.num_rows` with the golden catalog's row count (`exp_catalog.tsv`) under **every** policy (no `or policy ==
  "null"`); `t12_offsets.rs:155` asserts the result of `place(&e)` or drops the call. Each is shown to fail by a
  one-line mutation on a scratch copy.
- N-5 (skips): **T-13h**: `test_interval.py`'s refusal test asserts unconditionally, like
  `test_consumers.py` (the refusal was seen on pyarrow 15.0.2 and 26.0.0; the README states it without a version; if
  a later pyarrow writes the type, the test fails and the README and ADR-47 are revisited). The other skips stay; the
  release checklist runs pytest with `-rs` and lists every skip (step 6).
- N-6 (README / CONTRIBUTING): ADR-50's README sentence "`pip install oradump` does not install this project" is
  **superseded by ADR-54 item 2's line** (recorded here; no text change). **T-13h**: README "Errors" sentence:
  "Errors are `oradmp.OradmpError` subclasses (an internal error is `oradmp.InternalError`, `E-INTERNAL`), except an
  operating-system error reading the file, which is an `OSError` with `.code == "E-IO"`."; CONTRIBUTING's development
  setup uses `uv venv .venv` and `uv pip install`, like the release part, and `maturin develop --features
  test-probes`. `.gitattributes` alignment: closed (cosmetic).
- N-7 (`E-IO` as `OSError`): **closed** (ADR-46 item 8); the README text is N-6.
- N-8 (old names): the plan's Reviewer 3 list and the Q-5 text in `open-questions.md` are corrected in session 14
  (Architect). **T-13h**: the untracked `python/oradmp/_core.abi3.so` and every `__pycache__` under `python/` and
  `tests/` are rebuilt / deleted (`maturin develop` after `find python tests -name __pycache__ -prune -exec rm -rf {}
  +`), and the scratch folder `~/.cache/oradump-t12` left by T-12 is deleted.
- N-9 (loose counts in older tests): **T-14** item (exact values from the tools or the golden catalog; T-14 adds tests
  anyway), together with the missing `string`-mode test of DAY TO SECOND opposite signs (review 03 section 4) and the
  "three rows" comment of `t13_interval.rs:45`.
- N-10 (rule T9's 382 / 1,885): **Analyst session 11** (with X2): `open-questions.md` (b).
- N-11 (SBOM timestamp): ADR-56 item 3.
- **Order**: T-13h (alone) -> release 0.1.0a1 -> T-14, T-15 -> Reviewer session 4 -> T-16. T-14 does **not** start
  before T-13h is committed: both change `oradmp-core`'s `error.rs` (the `E-INTERNAL` code), `lib.rs` (the `guard`
  module) and `Cargo.lock`, and T-14's fuzz targets assert the no-panic rule that ADR-59 builds on; a session of delay
  is cheaper than a merge of the error enum. For rule 7 of `CLAUDE.md`, T-13h, T-14 and T-15 are one group: Reviewer 4
  reads all three. The alpha is therefore released with T-13h not yet reviewed; whether that is acceptable for a PyPI
  upload is **Q-12** (needs the project owner).

## ADR-61 The T-13h deviations and choices (implementer-notes "T-13h note") - decided (session 15)
- Context: T-13h is committed (`39cbb93`; 402 Rust tests, 105 pytest, 0 skipped). Its note lists two deviations from
  ADR-55 and four smaller points. Checked in this session in the crate sources of the lock file (arrow-schema 59.3.1
  `src/error.rs`, arrow-array 59.3.1 `src/ffi_stream.rs`, pyo3 0.30.1 `src/marker.rs`), not in other code.
- Decision:
  1. **Deviation 1, the `External error: ` prefix: accepted; the documents change, no wrapper.** arrow-schema's
     `Display` for `ArrowError` puts a prefix before every variant (`ExternalError` prints `External error: {source}`;
     no variant prints the bare text), and arrow-array's C stream export stores `err.to_string()` as the stream's last
     error and returns `EINVAL` for every variant except `NotYetImplemented`, `MemoryError`, `IoError` (`get_error_code`),
     which pyarrow raises as `pyarrow.ArrowInvalid`. The contract on the stream path is now: **the message contains the
     code in brackets, with the offset when there is one (`[E-FRAMING offset=0x1a2b] ...`); a consumer finds the code
     by the pattern `\[(E-[A-Z0-9-]+)` (first match), not by a prefix.** The core's own `Display` (and so every
     `OradmpError` message on the iterator path) still starts with `[E-`. Arch 3.4 and ADR-55 items 5, 6 and ADR-59 item
     2 read "contains" from now on (amendments under those records). The tests' `removeprefix("External error: ")` is
     correct and stays.
     Rejected: a wrapper that removes the prefix. In Rust it would mean our own `FFI_ArrowArrayStream` export (raw
     pointers and `extern "C"` callbacks: `unsafe`, against arch 7.3 and the workspace lint `unsafe_code = "forbid"`);
     in Python it is the batch-by-batch wrapper ADR-55 already rejected (copies the protocol, loses zero-copy for other
     consumers). A pattern is as stable as a prefix and survives a later arrow-rs wording change of the prefix.
     Rejected for the alpha: a helper `oradmp.stream_error_code(exc)` (new public API for one regular expression); the
     typed `BatchReader.error` stays "later" (ADR-55 item 6).
  2. **README on the stream path** (the Implementer's file, done in T-14 (i)): today's README says only "An error has
     `.code`, ...", which a reader takes for the stream path too. After the stream paragraph it gets this sentence,
     exactly: "On the stream path an error reaches the consumer as a pyarrow exception (`pyarrow.ArrowInvalid`), not as
     an `oradmp.OradmpError`: its text contains the code and the offset, for example `External error: [E-FRAMING
     offset=0x1a2b] ...`; find the code with the pattern `\[(E-[A-Z0-9-]+)`. `r.warnings` is complete after such an
     error." No other README text about stream errors exists (checked at `39cbb93`); nothing there contradicts the
     contract, so the internal hand-out of 0.1.0a1 is not held for it.
  3. **NUL in a message** (found while checking item 1): arrow-array's export turns the message into a `CString` with
     `expect(...)`, inside the `extern "C"` callback; a message with a NUL byte would panic there, outside the binding's
     guard, and abort. Today no message can hold one (the messages are literals and integers; names never enter a
     message, arch 3.4 "No value in any message"). **Rule**: no error or warning text contains U+0000; T-14's fuzz
     targets assert it for every error they see, and `t12_messages.rs` asserts it over the 201 files (T-14 (c), (h)).
  4. **Deviation 2, the iterator after `close()`: confirmed.** After `close()`, a pull on either path is `E-CONFIG`
     ("the reader is closed"), unless the source had already returned its end, in which case it is the end again
     (`StopIteration` stays `StopIteration`, as the Python iterator protocol expects). This is ADR-55's reason ("never a
     silent short table") applied to the iterator; recorded as ADR-55 item 5, amended.
  5. **`_probe_panic(kind, path=None)` returning a `_Dump` or `_Reader`** (feature `test-probes` only): accepted; ADR-59
     item 5 (b) amended. `SharedBatches` holding a `Box<dyn BatchSource>` instead of being generic: accepted (same
     effect, no generic parameter in a `#[pyclass]`).
  6. **The ADR-58 self-test as three tests** (`the_check_reports_a_value_in_each_form`,
     `the_check_reports_a_decoded_value`, `the_helpers_of_the_decoded_forms`): accepted; the values come from the truth
     files through `common::truth`, which is what item 4 asked. ADR-58 item 4 amended. The `04e4` / `0x1e5` rule of note
     item 5 is accepted as written.
  7. **"No lock held while Python code runs": no runtime test; a code-review rule for Reviewer 4.** A deterministic test
     would need a probe that blocks inside the core while holding the mutex and a second thread that would deadlock:
     slow, timing-dependent, and it shows one path, not the absence of all others. The property is structural, so it is
     checked by reading the code. The rule Reviewer 4 applies to `crates/oradmp-py/src/*.rs`:
     (a) every `.lock()` of the reader's or the dump's mutex is inside a `py.detach(...)` closure, or inside
     `SharedBatches::next` (called by the consumer, GIL state unknown);
     (b) no such closure calls `Python::attach` (or any API that takes the GIL) and none touches a `Py<...>` object;
     `Bound<'py>` and `Python<'py>` cannot enter it anyway, since pyo3 0.30.1 requires the closure to be `Ungil`, which
     without pyo3's nightly feature means `Send` (`marker.rs`), and both are not `Send`;
     (c) the guard is released before any Python object is built (copy out, then build).
     With (a) to (c) a mutex holder never waits for the GIL, so a thread that waits for the mutex while holding the GIL
     (a pull from pyarrow) cannot deadlock. A one-line `grep -n "lock()"` over the binding lists the places to read.
     The T-13h two-thread record stays a record (Reviewer 4 repeats it).
  8. **N-5 on pyarrow 15.0.2**: run in release checklist **step 7**, in the Python 3.11 venv with pyarrow 15.0.2 and
     pandas 2.2.3, as part of the full `pytest -rs` against the installed wheel; the release note names the outcome of
     `test_interval.py` and `test_consumers.py` on that venv explicitly. Not a separate task.
  9. `~/.cache/oradump-t13` (note item 10): the owner may delete it; nothing reads it.
- Consequences: arch 3.4 and the README describe what pyarrow actually shows; no `unsafe` and no new API; the lock rule
  is a review item with a precise checklist instead of a flaky test.
- Rejected: see items 1 and 7.
- **Amendment (session 16, ADR-65 item 2)**: item 7 (a) covers the `inner` and `shared` mutexes. The `names` mutex
  may be taken with the GIL held: its critical sections only clone or store an `Arc<Names>`, call no Python and take
  no other lock; anything more under it moves into `detach`.

## ADR-62 T-14 and T-15: scope, order, models, the machine's memory - decided (session 15)
- Context: T-14 and T-15 were planned (sessions 12 to 14) to run in parallel after T-13h. Facts since: T-13h added
  `oradmp_core::guard` and `E-INTERNAL` (`error.rs`, `lib.rs`; `ErrorCode::ALL` has 25 codes), the `test-probes` feature
  of `oradmp-py`, and `tools/` and `fuzz/` are the Implementer's (`CLAUDE.md`, O-14 applied). The VM has 8 CPUs and
  11 GiB of memory with 6 GiB swap, of which 4 GiB were in use when this session looked (`free -g`); a full build with
  default parallelism, a nextest run, a fuzz run with AddressSanitizer and a benchmark compete for it. Arch 8.3 (c)
  (NUMBER round trip) already exists (`t4_values.rs`, `number_roundtrip`). `cargo-fuzz` needs a nightly compiler and a
  C++11 compiler (rust-fuzz book, "Setup"); the VM has only the pinned stable 1.99.0 and `c++` 14.2, no `cargo fuzz`.
  `oradmp_core::dump::open` takes a path (no reader-based open); `exp::items::walk` takes any `Read + Seek`.
- Decision:
  1. **One after the other: T-14, then T-15**, each alone on the machine, in the main checkout (no worktrees). Reasons:
     (a) the memory rule below allows one build at a time, and fuzzing and benchmarks must run alone, so two sessions
     would wait for each other anyway; (b) both change `Cargo.lock` (T-15 adds `criterion`), and T-14 may change core
     files that T-15 benchmarks (a fix in the row loop changes the baseline); (c) the baseline of T-15 should be taken
     on the code T-14 hardened.
  2. **Memory rule for the machine (12 GB)**, in every Implementer and Reviewer prompt from now on: one build at a time
     (no two `cargo` or `maturin` commands at once, no second session building); `CARGO_BUILD_JOBS=2` in the
     environment of every `cargo` / `maturin` command; `cargo nextest run -j 2`; fuzzing (`cargo fuzz run`, one target at
     a time, `-jobs=1 -workers=1`) and benchmarks (`cargo bench`, the throughput and memory runs) alone on the machine:
     no build, test or second session at the same time. A run that hits the OOM killer is repeated alone, not with
     larger limits.
  3. **Models**: T-14 **Opus** (a fuzz finding means a fix in framing or decoding code where a wrong fix changes rows
     silently, plus the judgement whether a fix changes behaviour and must stop for the Architect); T-15 **Sonnet**
     (measurement and recording, no design choice).
  4. **T-14 scope**: arch 8.3 (a) (damage properties), the three `cargo-fuzz` targets `walk`, `read`, `values`, review
     03 N-9, the DAY TO SECOND `string`-mode test, the README stream sentence of ADR-61 item 2, the NUL rule of ADR-61
     item 3. Arch 8.3 (b) (a generator of synthetic files from the rules) **moves out** to a new **T-17** after T-16, not
     a beta gate: it is large, its expectation and the reader come from the same reading of the rules (common mode), and
     what it would add for scale (large files) the owner's `corpus/work/` files give T-15.
  5. **Fuzz layout**: `fuzz/` is its own crate outside the workspace (its own `[workspace]`, as cargo-fuzz generates it;
     its own `fuzz/Cargo.lock`, committed; the root `Cargo.lock` does not change for it), with `fuzz/rust-toolchain.toml`
     pinning one **dated** nightly (rustup takes the nearest `rust-toolchain.toml`, so the root's stable pin stays for
     everything else); `cargo-fuzz` installed with `cargo install cargo-fuzz --locked --version <x>`; both versions in
     the T-14 note. `fuzz/corpus/` and `fuzz/artifacts/` are not committed (`fuzz/.gitignore`). Seeds: the `.dmp` files
     of the three fixture folders, copied (read only from `corpus/fixtures/`) into `target/fuzz-seeds/` by
     `tools/fuzz_seeds.sh`; libFuzzer writes new inputs only into its first corpus folder. The fuzz targets call the
     core directly: **a panic there is a crash, not `E-INTERNAL`**; ADR-59's guard is the binding's and is not used in
     the targets.
  6. **Fuzz limits**, every run: `-max_total_time=600 -max_len=1048576 -timeout=10 -rss_limit_mb=2048
     -malloc_limit_mb=64`; the `read` target reads with `batch_rows=1024`, `batch_bytes=4 MiB`, `max_lob_bytes=1 MiB`,
     `on_value_error=Null`, so 64 MiB for one allocation is a bound no correct read of a 1 MiB input needs. A crash, a
     timeout (hang) or an out-of-memory report is a finding.
  7. **A finding** gets a regression test that fails before the fix (shown in the note) and passes after: in
     `crates/oradmp-core/tests/t14_fuzz.rs`, as "fixture X with these bytes at these offsets" when the input is a mutated
     fixture, else as a minimised file below 4 KiB in `crates/oradmp-core/tests/fuzz/` (not `tests/data/`: no expected
     output, so not a golden file under rule 7). A fix that changes an error code, a warning, the rows or values of any
     fixture, or a rule stops the task and goes to the Architect as a note; a fix that only turns a panic, hang or
     unbounded allocation into an existing typed error is the Implementer's.
  8. **T-15 inputs**: micro-benchmarks on the committed fixtures (arch 4.4); throughput and the memory test of arch 4.3
     on the owner's exports of `T_ROWS` (100,000 rows) and `T_BIG` (5,000,000 rows) in `corpus/work/` (owner action
     **O-15**: say which files exist and their paths; agents other than the Implementer in T-15 do not read that
     folder). Without them T-15 is not done (the micro-benchmarks alone are not the baseline of arch 4.3).
- Consequences: two sessions in a row instead of one in parallel; no merge of `Cargo.lock` or core files; the machine
  does not swap during measurements. The plan's T-14 and T-15 "Done when" are rewritten (session 15).
- Rejected: parallel worktrees (two builds at once on 12 GB, shared `Cargo.lock`, a baseline taken on code that changes
  next door); `rustup default nightly` (changes the toolchain for the whole machine; the pin is per folder);
  reader-based `open` in the core for the `read` target (a new public API for a test; the target writes its input to a
  file in a per-process temporary folder, which costs microseconds); an allocation counter in the core tests (a
  `GlobalAlloc` needs `unsafe`, forbidden by the workspace lints; libFuzzer's limits measure it in `fuzz/`).
- **Amendment (session 16, ADR-64 item 3, ADR-65 item 9)**: item 5: fuzz runs use `tools/fuzz_run.sh <target>
  [seconds]`, which runs from `fuzz/` (the nearest `rust-toolchain.toml` is the nightly pin) and creates
  `fuzz/corpus/<target>`. Item 8: T-15 is done with `T_ROWS` alone; `T_BIG` is optional (it would sharpen the memory
  test, ADR-65 item 6).

## ADR-63 Release after the owner's Q-12 answer: 0.1.0a1 handed out, the PyPI upload is the reviewed build - decided (session 15; the upload itself stays the owner's, Q-10)
- Context: the owner answered Q-12: 0.1.0a1 is handed out internally now, for the install test on RHEL 8; the PyPI
  upload comes only after Reviewer session 4. The owner ran `tools/release_build.sh` once on the committed tree; the
  Architect does not know the result. T-14 will change shipped files (the README, ADR-61 item 2; core fixes for fuzz
  findings); a wheel's `METADATA` and the sdist carry the README. An uploaded version cannot be replaced.
- Decision:
  1. **0.1.0a1 is internal only**: built from the T-13h state (the commit the owner built, or a later commit that changes
     no shipped file), tagged `v0.1.0a1`, handed out as the two files with their SHA-256; **never uploaded** to PyPI.
  2. The tag goes on **exactly the commit the build printed** (`SOURCE_DATE_EPOCH` is the commit time, so a later
     docs-only commit gives different archive bytes); if the owner's run was on another commit, step 6 is run again on
     the commit to tag.
  3. **The PyPI upload** (after Reviewer session 4 with no blocking finding) uploads the build of the reviewed commit. If
     any shipped file differs from `v0.1.0a1` (it will: the README of T-14), the version becomes **`0.1.0-alpha.2`**
     (Cargo) / **`0.1.0a2`** (Python), set by the Implementer in checklist step 5 of that run (root `Cargo.toml`,
     `Cargo.lock`: two lines), and the whole checklist runs again for a2, the RHEL 8 install included. Uploading a1 after
     T-14 would publish code without its fuzz fixes.
- Consequences: the PyPI version starts at a2; a1 exists only as handed-out files and a tag. Q-12 is closed; Q-10's
  upload part is now "upload a2 after Reviewer 4?" (recommendation: yes).
- Rejected: uploading the a1 files after Reviewer 4 (outdated README, no fuzz fixes); rebuilding a1 from a later commit
  (two different files with one version).
- **Amendment (session 16, ADR-67)**: item 3's "two lines" is wrong: the README names the version three times, so
  the a2 bump changes `Cargo.toml`, the two `Cargo.lock` entries, the `oradmp-core` entry of `fuzz/Cargo.lock` and
  the README (ADR-67 item 2). Run A: tag
  `v0.1.0a1` on `b7304d5` (seen in git); its hashes and RHEL 8 result are the owner's to record.

## ADR-64 Review 04's S-1 to S-5: one task T-13i - decided (session 16)
- Context: Reviewer session 4 (`docs/review/review-04-t13h-t15.md`, HEAD `b3a5a7f`; 408 Rust tests + 3 ignored, 105
  pytest): no blocking finding, five should-fix items, each shown by a mutation that no test caught. All five are test
  or tooling gaps; none changes a rule, a code, a warning or any fixture's rows. They go into **one task T-13i**
  (Implementer, **Opus**, alone; plan), together with the notes of ADR-65 marked "T-13i".
- Decision:
  1. **S-1 (iterator after `close()`, ADR-61 item 4)**: a pytest in `tests/test_stream.py`. (a) A reader of 18c
     `part_exp_18c.dmp` T_PART with `batch_rows=1000`: one `next(r)`, `r.close()`, then `next(r)` raises
     `oradmp.UsageError` with `.code == "E-CONFIG"` and "the reader is closed" in the text, `r.read_all()` raises the
     same, `r.rows_read` stays 1000. (b) A second reader read to its end, then `close()`: `next(r)` raises
     `StopIteration`, `r.read_all()` returns a table with 0 rows and the reader's schema. Done when mutation **s1f**
     (`finished || !from_stream` gives a silent end) fails (a).
  2. **S-2 (`r.warnings` complete after an error)**: a test, not a weaker README sentence (the sentence is true; the
     shared state copies the warnings before it returns the error). **The patched copy**: the test copies
     `corpus/fixtures/18c/t_num_exp_18c.dmp` into `tmp_path`, asserts the bytes before patching, and changes **one
     byte**: file offset **0x22F4**, `c2` -> `bc`. Derivation (checked in this session with the Analyst's tools,
     fixture SHA-256 `3df4a6c5...8bb275` equal to `SHA256SUMS`): `exp_parse.py --rows` lists T_NUM row 17 as
     `@22ee [2] c1 12 | @22f2 [5] c2 02 18 2e 3d`, i.e. the V field's length `05 00` at 0x22F2 and its bytes
     `c2 02 18 2e 3d` (123.456, truth_num row 17) at 0x22F4 to 0x22F8 (`xxd` agrees). By rule [Y1] the first byte is
     the exponent: `bc` gives E = 0xBC - 0xC1 = -5, digits 1, 23, 45, 60, value 1.23456E-10 (`num_decode.decode`; its
     `encode` returns the same five bytes, so the patched value is canonical and valid). Column V is `NUMBER`
     (Decimal128(38, 10), `round`, ADR-14): the value has 15 fractional digits and rounds to 1E-10, so row 17 gives
     **`W-NUMBER-ROUNDED`** (first offset 0x22F4 = 8948, row 17, column `V`, count 1), and row 19 (`d1 0b` =
     1E+33 at 0x230F) still gives **`E-NUMBER-OVERFLOW` offset 0x230f** under the default `raise`, as unpatched.
     Asserted on **both paths**, default options: the iterator raises `E-NUMBER-OVERFLOW` (offset 0x230F, row 19) and
     the stream raises `pyarrow.ArrowInvalid` containing `[E-NUMBER-OVERFLOW offset=0x230f]`; on each, `r.warnings`
     is exactly one `W-NUMBER-ROUNDED` with count 1, offset 8948, row 17, column `V`; `r.rows_read` equals what
     `test_warnings.py::test_an_error_ends_the_iteration_and_issues_the_counts_up_to_it` asserts for the unpatched
     file; the warning is issued once at the end of the `with`. Done when mutation **s1e** (warnings copied only on a
     successful pull) fails it on the stream path.
  3. **S-3 (the fuzz command)**: the plan's T-14 (d) command was wrong (the root pin 1.99.0 wins from the root; the
     corpus folder must exist). **`tools/fuzz_run.sh <target> [seconds]`** (Implementer, T-13i; default 600): runs
     `tools/fuzz_seeds.sh` when `target/fuzz-seeds/` is missing; creates `fuzz/corpus/<target>` (**the run script
     creates the corpus folders, not `fuzz_seeds.sh`**: the corpus belongs to a run, the seeds to the fixtures); sets
     `TMPDIR` to `${ORADMP_FUZZ_TMP:-/var/tmp/oradmp-fuzz}` (created; the script stops when it has less than 2 GiB
     free, since `/tmp` is a 3.9 GiB tmpfs that was full in T-14); runs from `fuzz/`, so rustup takes
     `fuzz/rust-toolchain.toml` (no `RUSTUP_TOOLCHAIN` needed): `cargo fuzz run <t> corpus/<t> ../target/fuzz-seeds
     -- -max_total_time=<s> -max_len=1048576 -timeout=10 -rss_limit_mb=2048 -malloc_limit_mb=64 -jobs=1 -workers=1`;
     afterwards removes `$TMPDIR/oradmp-fuzz-*` and moves libFuzzer's `fuzz-0.log` into `$TMPDIR`; exit status
     non-zero when libFuzzer fails or `fuzz/artifacts/<t>/` is not empty. The equivalent manual command, for the
     plan: `tools/fuzz_seeds.sh && mkdir -p fuzz/corpus/<t> && cd fuzz && TMPDIR=/var/tmp/oradmp-fuzz cargo fuzz run
     <t> corpus/<t> ../target/fuzz-seeds -- <limits>`. **CONTRIBUTING** gets a "Fuzzing" section (Implementer,
     T-13i): prerequisites (the dated nightly of `fuzz/rust-toolchain.toml` with `--profile minimal`, `cargo install
     cargo-fuzz --locked --version 0.13.2`, a C++11 compiler), the script, the machine rule, where findings go
     (ADR-62 item 7). Done when `tools/fuzz_run.sh walk 30` runs from a fresh clone (no `fuzz/corpus/`) and the note
     shows its output.
  4. **S-4 (older `check_dist.py` rules)**: one synthetic case each in `tests/test_check_dist.py`, in the style of
     ADR-57 item 3 (builders `make_wheel` / `make_sdist`; `make_sdist` gains a root-name parameter), each asserting
     exactly one FAIL line naming its rule and nothing else: (m1) a wheel named with a version other than
     `pep440(cargo_version())` -> "from Cargo.toml"; (m2) a correctly named wheel whose METADATA says `Version: 0.0.0`
     -> "Version is"; (m3) a wheel member `oradmp/_api.py` holding the old name (built from parts, as `check_dist.py`
     does) -> "the old project name", and the same pair with only the allowed line in METADATA passes; (m4) sdist
     `<root>/python/oradmp/x.dmp` -> "a dump file"; (m5) sdist `<root>/python/oradmp/truth_x.txt` -> "a truth file";
     (m6) sdist `<root>/python/exp_survey.py` -> "the survey tool"; (m7) wheel member `other/x.py` -> "outside
     oradmp/"; (m8) an sdist whose root folder is `oradmp-0.0.0` under the right file name -> "root folder"; (m9)
     wheel member `oradmp/__pycache__/x.txt` (not `.pyc`) -> "under a __pycache__/ folder", so the component rule is
     caught without the suffix rule. Done when each of the nine rules, removed alone, fails its test (mutation table in
     the note).
  5. **S-5 (`in_piece` messages)**: (a) `t12_messages.rs` checks two more copies: the chain copy of
     `t6_patched::a_broken_utf8_sequence_in_a_chain_is_a_value_error` and the chunk copy of `t12_offsets.rs` (the
     high surrogate at the end of chunk 1, "chunk 2"); both made by one shared helper in `tests/common/` (the patch is
     not written twice); the test asserts that `seen` holds a message ending exactly "(piece 2 of the value)" and one
     ending "(chunk 2 of the value)". (b) The two existing tests assert the exact message suffix instead of
     `contains("piece 2")` / `contains("chunk 2")` (which pass "piece 20"). (c) **The premise gets a new bound**: a
     piece or chunk number is a counter that can exceed 5 digits (a long chain or a large LOB has that many pieces), so
     ADR-58 item 3's "at most 5 digits" covers the message **without** the `in_piece` suffix: the check removes an
     exact trailing `" (piece N of the value)"` / `" (chunk N of the value)"` (N decimal digits) before the digit-run
     test (e1) and the inventory assertion, and checks everything else as before. (d) A unit test of `in_piece`
     (`reader.rs` tests) asserts the exact text for N = 2, 100000 and `u64::MAX`. Done when the reviewer's
     **premise** mutation (`n + 100000`) fails (b) and (d), and a value appended inside the message before the suffix
     still fails `no_message_of_the_fixtures_holds_a_value`.
- Consequences: every ADR-55 / ADR-61 stream behaviour and every `check_dist.py` rule has a test that a one-line
  mutation fails; the fuzz runs are one command; the message check sees both owned runtime-integer messages.
- Rejected: a weaker README sentence for S-2 (the property holds and is cheap to test); patching a value after row 25
  so that the error follows the rounding of row 25 (several bytes and a length change instead of one byte); a 6-digit
  inventory bound (the piece counter has no 6-digit bound either); making `fuzz_seeds.sh` create the corpus folders
  (a seed refresh would then touch run state).
- Amendment (session 17, ADR-69 item 1): in item 2, "the warning is issued once at the end of the `with`" holds for
  the **stream path** only. Read it as: "the warning is issued **once**: on the iterator path when the error ends the
  iteration (arch 3.4, ADR-55 item 3), and the end of the `with` issues nothing more; on the stream path at the end
  of the `with`, nothing before it". No code change.
- Amendment (session 18, ADR-71 items 1, 2; review 05 SF-1, SF-3): the "Consequences" sentence "every `check_dist.py`
  rule has a test that a one-line mutation fails" is **not yet true** (seven rules survive removal, five more by
  reading); item 2's "on each [path]" is not met by the iterator test (mutation s1o survives). Both are closed by
  T-13j; the sentence stands as the target.

## ADR-65 Review 04's notes and the mutations no test caught - decided (session 16)
- Context: review 04 notes N-1 to N-10 and section "Tests that did not fail when mutated". One line each below; "T-13i"
  means a "Done when" item of T-13i (plan).
- Decision:
  1. **N-1, panic guard coverage**: checked against `crates/oradmp-py/src/lib.rs` and `options.rs` at `b3a5a7f`.
     **Wrap** in `catch_panic` the two calls that compute on input: `NumberType::from_str` (`options.rs`, the two
     parses of user strings) and `number_summary` (`lib.rs`, survey). **Exempt**, ADR-59 item 2 amended with this
     list: `Dump::info()`, `Dump::catalog()`, `Dump::warnings()` and `TableBatches::schema()` (accessors returning or
     cloning data that was built under the guard), `Error`'s `Display` in `to_pyerr` (formats literals and integers),
     the drops in both `close()` (frees buffers, closes a file), and the `core_open` of `_probe_panic` (feature
     `test-probes` only). T-13i; a test is not needed for the wrap (no panic path exists), the note lists the diff.
  2. **N-2, `names` mutex under the GIL**: no code change; ADR-61 item 7 (a) amended: it covers the `inner` and
     `shared` mutexes; the `names` mutex may be taken with the GIL held because its critical sections only clone or
     store an `Arc<Names>`, call no Python and take no other lock. A later change that does more under `names` must
     move it into `detach`. Closed.
  3. **N-3, damage-test mutations a1, a2, c2**: sampling cannot pin a site that one cut in millions reaches. T-13i adds
     to `t14_damage.rs` a table-driven test `each_truncation_site_names_its_offset`: one case per site that `grep -n "file ends"
     crates/oradmp-core/src` lists (at `b3a5a7f`: `io.rs` "inside a field" / "inside a line", `exp/rows.rs` "where a
     row or the end of rows should stand" and "inside a LOB length", `exp/items.rs` "before the second EXIT line",
     `exp/framing.rs:109`), each a named fixture cut at an offset taken from `exp_parse.py` output (the note shows the
     line), asserting the code the site returns and the **exact** offset the rule names; for `exp/framing.rs:109` (`read_unlisted_field`; no fixture has an unlisted introducing line) a
     unit test in `framing.rs` over a `Cursor` (`03 00 61 62 63`, end of input) asserting `E-TRUNCATED` at offset 5.
     A panic on any of these paths fails its case. The properties also bound **warning** offsets (<= t, <= file
     length). Done when a1, a2 and c2 each fail at the default 256 cases.
  4. **N-5, d11 and d13**: added to ADR-58 item 5's list as **d9** (a date as `DD.MM.YYYY`, e.g. `15.06.-100`) and
     **d10** (an overflowing number in digit groups with `,`). The check changes in `t12_messages.rs`: (e3) also
     matches a date token of three integer groups separated by `.`, `/` or `-` in the orders Y-M-D, D.M.Y and M/D/Y
     whose (year, month, day) equals a stored date; (e1) first joins digit groups separated by one of `,` `.` `_` `'`
     or a space when every group after the first has exactly 3 digits. The self-tests gain one case each. T-13i; done
     when d9 and d10 fail `no_message_of_the_fixtures_holds_a_value` and d1 to d8 still fail it.
  5. **N-6, the root `.gitignore`**: T-14 edited a file outside the Implementer's row (three lines `fuzz/corpus/`,
     `fuzz/artifacts/`, `fuzz/target/`, redundant with `fuzz/.gitignore`). The file also holds the real-data rules
     (`real_dumps/`, `*.real.dmp`, `corpus/work/`), so it should stay the owner's: owner action **O-16**
     (`open-questions.md`) names it in the Human row; the three lines stay unless the owner removes them; agents put
     ignore rules into the `.gitignore` of their own folder.
  6. **N-4, the memory test's limit: stays 1.10**, no absolute assertion. Correct runs gave 1.023 to 1.049 (six
     runs); a limit below about 1.06 would sit one run of VM noise away from a false failure, and a flaky gate gets
     ignored; an absolute bound on growth (correct 172 to about 370 kB, 4 bytes per row 464 to 592 kB) separates
     the cases by less than the noise. 1.10 catches retained batches and 8 bytes per row (shown); growth below about
     6 bytes per row is below what a 10 MB file resolves, recorded in `benchmarks.md` 4.3. With `T_BIG` the same test
     (stop at 100,000 rows vs the end, 50 times the rows) resolves 1 byte per row: T-13i makes the test take `T_BIG`
     when present, with the same 1.10. T-13i also renames the test (eight times, not ten) and asserts the stop at
     exactly 12,288 rows.
  7. **N-7, `_probe_panic` in the shipped stub**: T-13i removes it from `python/oradmp/_core.pyi`; the probe tests
     reach it with `getattr` and skip when absent, as now.
  8. **N-8, `release_build.sh`**: T-13i pins `ziglang` to the version run A used (printed by the script) and resolves
     the build root with `realpath -m` before the home and user checks; the SBOM wording ("6 occurrences of 2 paths")
     is noted.
  9. **T-15 deviations** (not decided before): `T_BIG` absent: T-15 is done with `T_ROWS` (ADR-62 item 8 amended;
     O-15's `T_BIG` stays optional); cold cache not measured (not asked; `benchmarks.md` 1 says so); 12,288 instead of
     10,000 rows (a batch is not cut): accepted; criterion without default features: accepted (no HTML reports
     needed); `VmHWM` Linux-only: the memory test and `stream_memory.py` are Linux-only, T-16 gates them by
     `target_os`.
  10. **T-14 deviations**: debug assertions and overflow checks in the fuzz profile: accepted; warning offsets checked
     in the targets: accepted; 34 Data Pump seeds: accepted; `/tmp` and the leftover folders: ADR-64 item 3; no
     clippy for `fuzz/`: T-16 (a nightly clippy step in CI); `-malloc_limit_mb` bounds one allocation only: accepted as
     the bound for "no allocation sized from a length field" (arch 8.3 (a) reworded in session 16).
  11. **N-9, fuzz scope** (one option set in `read`, no charset decoders in `values`, no dictionary): deferred to
     T-17's time; not a release gate. **N-10** (rule T9): Analyst session 11 (`open-questions.md` (b)).
- Consequences: T-13i is tests, two tool scripts, the binding's two wraps, the stub and CONTRIBUTING; no core change
  except the `in_piece` unit test and the `framing.rs` unit test.
- Rejected: a memory limit of 1.05 or 1.06 (inside the noise); wrapping the accessors and drops (no input-derived
  work in them; the guard would only add marks to objects that cannot fail); taking the `names` lock inside `detach`
  (a GIL release per `Arc` clone).
- Amendment (session 17, ADR-69 items 2 and 3): item 3's site list named `exp/rows.rs` "inside a LOB length"; that
  site **cannot be reached** (`read_bytes(8)` returns 8 bytes or fails first with `io.rs` "inside a field"); the two
  LOB cases of `each_truncation_site_names_its_offset` (cuts 0x4359, 0x435d of 18c `lob_exp_18c.dmp`) stand for it,
  and T-13j removes the branch. Item 6's sentence "1.10 catches retained batches and 8 bytes per row (shown)" is
  **wrong** and is replaced by: "on `T_ROWS`, 1.10 catches retained batches and 16 bytes per row (Reviewer 4: 2.37,
  1.40); 8 bytes per row sits **at the limit** (Reviewer 4: 1.16 in one run; T-13i: 1.073 to 1.111, caught in 1 of 3
  runs); 4 bytes per row passes. Correct-reader ratios: 1.016 to 1.059 (six runs per path: T-15 note, Reviewer 4, T-13i)".
  "`T_BIG` resolves 1 byte per row" is **arithmetic, not measured** (ADR-69 item 3); T-15b measures it.
- Amendment (session 18, ADR-71 item 4): item 1's exempt list is completed as ADR-59's session 18 amendment says.
- Amendment (session 19, ADR-75 item 1): item 6's wording stands (not wrong). "`T_BIG` resolves 1 byte per row" is
  now **measured** (T-15b, code of `460aef5`, three runs each): correct reader 1.0016 to 1.0252 on `T_BIG`; 1 byte
  per row 1.78 to 1.96, 4 bytes about 3.7, 8 bytes about 5.9 on `T_BIG` alone, each failing 3 of 3. On `T_ROWS` 8
  bytes per row gave 1.1014 to 1.1033 (3 of 3 just above 1.10): still "at the limit", as the session 17 wording says.

## ADR-66 Benchmark baseline and regression rule - decided (session 16)
- Context: T-15 measured, Reviewer 4 reproduced within noise (0 to 13 % apart). Arch 4.3 promised a CI gate of 20 %
  "after the first measurement"; CI does not exist yet (T-16), and a hosted runner is not this VM.
- Decision: `docs/design/benchmarks.md` holds the baseline (both columns, spreads), the machine and conditions, the
  exact commands, and the rules: a new baseline only after a Reviewer-checked run (repeated figures within 15 %); a
  regression is a figure more than **20 %** below the baseline on the VM, confirmed by a second run alone (10 to 20 %
  is "watch"); a memory regression is a failed ratio (1.10), a `stream_memory.py` FAIL, or a peak more than **15 %**
  above the baseline. Measurements are due before a release that changes core sources, the release profile or the
  toolchain (checklist step 3b). A CI runner gets its own baseline table and gates only after five runs of the
  unchanged commit stay within +-10 % on that runner class (T-16); until then the VM procedure is the gate.
- Consequences: the gate is a manual VM step for now; arch 4.3 points to `benchmarks.md`.
- Rejected: 10 % (inside the session-to-session difference); a CI gate on shared hosted runners from day one (their
  noise is unknown).
- **Amendment (session 20, ADR-78)**: baseline 2 (code of `460aef5`) replaces the first; the compared figure is the
  slower of the two columns; `T_BIG` throughput regresses at more than 10 % (5 to 10 % "watch"); the memory peak limits
  are per file, 15 % above baseline 2's highest end peak.

## ADR-67 Release run B: 0.1.0a2, closure review, TestPyPI first, then PyPI - decided (session 16; the upload stays the owner's, Q-10)
- Context: Reviewer 4 found nothing blocking; ADR-63 makes the PyPI upload the build of the reviewed commit as
  0.1.0a2. Run A is done as far as the repository shows: tag `v0.1.0a1` on `b7304d5` (a docs-only commit after
  T-13h); its hashes, the install test on RHEL 8 and the hand-out are not recorded in the repository. The README names
  the version in three places (status line, wheel and sdist install lines), so the version bump is not two lines. An
  upload cannot be replaced: PyPI refuses a file name once used, even after deletion.
- Decision:
  1. **Closure review: yes**, a short Reviewer session 5 (Opus) after T-13i: the T-13i diff; each mutation of ADR-64
     and ADR-65 (s1e, s1f, m1 to m9, premise, d9, d10, a1, a2, c2) repeated alone; `tools/fuzz_run.sh` run once; the
     release build of the candidate commit with `--twice`. No new reading beyond that.
  2. **Version bump** (Implementer, the last commit of T-13i, after the owner's Q-10 answer): `[workspace.package]
     version = "0.1.0-alpha.2"` in the root `Cargo.toml`; `Cargo.lock` (the two entries `oradmp-core`, `oradmp-py`);
     `fuzz/Cargo.lock` (its `oradmp-core` entry, by `cd fuzz && cargo fuzz build`; not shipped, kept consistent);
     `README.md`: the status line and the install text. With Q-10 = upload: install text `pip install
     "oradmp==0.1.0a2"`, the file install kept as the offline alternative with the a2 file names. With Q-10 = no
     upload: the a1 text with a2 file names. `check_dist.py` (version rules) and the README test run green.
  3. **Steps of run B** (checklist, plan): gate (Reviewer 5 without a blocking finding; Analyst session 11 done;
     `git diff --stat <T-13i commit>..<candidate>` touches only `docs/`); checks; network checks; build with
     `tools/release_build.sh <candidate> --twice` on the committed tree; install tests (3.11 + pyarrow 15.0.2 /
     pandas 2.2.3; newest CPython; sdist build); RHEL 8 install of the a2 wheel; tag `v0.1.0a2` on the built commit;
     **TestPyPI upload**, download and compare; **PyPI upload**, download and compare; record.
  4. **Upload tool and token**: `uv publish` (uv 0.12.21 on the VM: `--publish-url`, `UV_PUBLISH_TOKEN`,
     `--dry-run`), run by the owner in his own shell, **never in an agent session**; TestPyPI with `--publish-url
     https://test.pypi.org/legacy/`, PyPI with the default endpoint. The token lives in the owner's password manager,
     is typed with `read -rs UV_PUBLISH_TOKEN; export UV_PUBLISH_TOKEN` (not on a command line, not in shell history,
     not in any file under the repository or `$HOME/.pypirc`), and is unset after the upload. TestPyPI and PyPI are
     separate accounts with separate tokens. The first upload needs an account-wide token (a project-scoped token can
     only be made once the project exists); right after it the owner makes a token scoped to `oradmp` and deletes
     the account-wide one.
  5. **Comparison after each upload**: in a scratch folder, the index page `<index>/simple/oradmp/` gives each file
     with its `#sha256=` fragment (PEP 503), compared with `$ORADMP_BUILD_ROOT/dist/SHA256SUMS`; the wheel is also
     fetched with `pip download --no-deps --only-binary :all:` and hashed (not the sdist: `pip download` would build
     its metadata); then an install from the index into a fresh venv
     (pyarrow from PyPI first, then `oradmp` with `--no-deps` from the index) and `python -m oradmp survey` on a
     copied fixture. A mismatch on TestPyPI stops before PyPI.
- Consequences: the name is secured by run B's PyPI upload; a mistake shows on TestPyPI, where a file name is cheap.
- Rejected: uploading without TestPyPI (an irreversible first step); twine (one more pinned tool; `uv` is already
  used); a token in `~/.pypirc` or a CI secret (outside the owner's direct control; CI does not upload in phase 1);
  skipping the closure review (T-13i changes the binding and the checks that guard the archives).
- Amendment (session 18, ADR-70): **closed**. Items 1 to 5 were carried out in run B (record in ADR-70 and the plan,
  "First release"). Corrections from the run: item 3's gate condition did not hold (the README commit `4e8f212`
  after Reviewer 5; accepted, ADR-70 item 2); item 4's `--dry-run` is **not** a credential-free check (it asks for the
  token), so the checklist no longer presents it as one; the files are named one by one, not by a glob (`dist/` also
  holds `SHA256SUMS`).
- Amendment (session 29, ADR-111 item 5): item 4's "the token lives in the owner's password manager" is replaced from
  a4 on by ADR-110 item 5 as practised in a4: one project-scoped token per index **made for the release and deleted
  after its upload**; nothing is kept between releases. Typed with `read -rs`, never on a command line, in a file or in
  an agent session: unchanged. A GitHub CLI token, if one is used for a release step, follows the same rule (ADR-111
  item 5).

## ADR-68 T-16 and T-17 after T-13h to T-15; order after review 04 - decided (session 16; the CI service needs the owner, Q-11)
- Context: T-13h to T-15 taught: `VmHWM` is Linux-only; `/tmp` can be a small tmpfs; the release build is
  byte-identical twice on the VM; the Linux release is built by `tools/release_build.sh` (bash, remaps); fuzzing
  needs a dated nightly and room in `TMPDIR`.
- Decision: T-16's "Done when" is rewritten (plan): the Linux release wheel stays the script's build on the VM; CI
  builds every platform with path remaps of its own and `check_dist.py` (home-path scan) must pass on every wheel;
  CI's Linux build by the script is compared with the VM's hashes and the result recorded (not a gate: zig and
  runner images may differ); `TMPDIR` is set on every runner; the Linux-only tests are gated by `target_os`; a
  nightly job runs `tools/fuzz_run.sh` for 60 s per target, `PROPTEST_CASES=10000`, nightly clippy of `fuzz/` and the
  micro-benchmarks as a report (ADR-66). T-17 is unchanged in scope, after T-16, not a gate. **Order**: T-13i;
  Reviewer session 5 (closure); release run B; T-16; T-17. Analyst session 11 runs before run B (any time, it builds
  nothing).
- Rejected: T-16 before the upload (the alpha is Linux-only by Q-9; CI waits for Q-11).
- Amendment (session 17, ADR-69 item 5): **Order**: T-13i (done); Reviewer session 5 (closure); release run B;
  **T-13j** (the dead LOB-length branch, after run B); T-16; T-17. **T-15b** (memory test on `T_BIG`) whenever owner
  action O-17 is done, after run B, alone on the machine; it gates nothing. Run B's gate and steps are unchanged
  (`git diff --stat` from the T-13i version commit still lists only `docs/`; step 3b stays "not due").
- Amendment (session 18, ADR-73): order after run B: T-13j; T-15b when O-17 is done; T-16; T-18; Reviewer session 6
  (T-13j, T-16, T-18); release 0.1.0a3; T-17; Data Pump (phase 2); the DuckDB side project last (Q-13).

## ADR-69 The T-13i deviations: warning timing, the unreachable LOB-length site, the memory claim, d9 and d10 - decided (session 17; the `T_BIG` file needs the owner, O-17)
- Context: the T-13i note (commits `469416d`, `dc7668e`) reports three points where ADR-64 / ADR-65 do not match what
  was built or measured, and the detection rules it chose for d9 and d10. None changes a code, a warning, a rule or
  a fixture's rows.
- Decision:
  1. **Warnings on the iterator path: amend the ADR, no code change.** The code does what arch 3.4 and ADR-55 item 3
     decided ("the iterator path keeps issuing at its end"; counted warnings are issued when the iteration ends by
     `StopIteration` or a raised error); only ADR-64 item 2's sentence and the plan's T-13i (b) said "at the end of
     the `with`" for both paths. Both are amended (ADR-64 amendment; plan). **README: no change.** Checked: its only
     timing claim is in the stream example ("Read it through the `with` block, so that the warnings are issued when
     the block ends", line 74, and the comment on line 82), which is the stream path and true; the iterator path's
     timing is not stated in the README. The test as built (issued when the error ends the iteration, not again at
     the `with` end) is the correct test.
  2. **The unreachable "file ends inside a LOB length": remove the branch** (task **T-13j**, after run B). Rule L4
     (v3.6) makes the long form `fc ff`, then **8 bytes LE** = T; nothing in the rules gives a cut inside those 8
     bytes its own meaning; reader step 6 makes any file cut short an error. So the reader's general truncation rule
     applies: `E-TRUNCATED` at the offset where the failed primitive began, which is what happens now (T-13i:
     0x4358 for the cut inside `fc ff`, 0x435a for the cut inside the 8 bytes). The dead branch, if it were ever
     reached, would say something else: **`E-FRAMING`** (via `framing_err`) at the **start of the LOB value**
     (`at`), not `E-TRUNCATED` at the 8 bytes. Defensive code that would report a different code and offset than the
     rule is worse than none. T-13j adds `ByteStream::read_u64_le` to `io.rs`, written like `read_u16_le` (its
     `try_from` error maps to the same `self.truncated("file ends inside a field")`, so primitive and fallback
     cannot diverge), and `lob_value` calls it. Not before run B: the change is behaviour-neutral, and before run B
     it would break the gate (`git diff` only `docs/`), make step 3b due and reopen Reviewer 5.
  3. **Memory test: correct the claim (ADR-65 amendment); keep 1.10 on `T_ROWS`; get the larger input (O-17,
     T-15b).** Why the 8-byte case is at the edge, by arithmetic: between the stop (12,288 rows) and the end
     (100,000) a retention of b bytes per row adds 87,712 x b bytes; for b = 8 that is 702 kB = **9.1 %** of the
     about 7,700 kB peak, i.e. below 1.10 before allocator rounding and noise, so 1 of 3 is what to expect, not a
     defect of the test. Tightening: the highest correct ratio is 1.059 and the lowest 8-byte ratio 1.073; a limit
     of 1.065 to 1.07 sits 0.006 to 0.011 above a correct run, inside the run-to-run spread (0.043 within one path,
     T-13i) -> rejected (a flaky gate gets ignored, ADR-65 item 6). An absolute-growth assertion: the correct growth
     (172 to about 370 kB) and the 8-byte growth (about 700 kB plus the `Vec`'s growth) are closer than the peak noise
     (about 230 kB between correct runs) -> rejected again. **`T_BIG`**: stop at 100,000 rows, end at 5,000,000:
     b bytes per row add 4,900,000 x b bytes, i.e. **4.9 MB per byte per row**, more than half of the expected peak
     (arithmetic; T_BIG's peak is not measured); 1 byte per row would then sit far above 1.10. The SQL exists
     (`corpus/sql/01b_optional_tables.sql`, commented out); producing the file is the owner's (O-17); the test's
     `T_BIG` branch exists (T-13i (h)); **T-15b** runs and records it. **Does not block run B** (agreed with the
     owner's view): it is the strength of a regression test, not a product defect: the correct reader's ratios are
     1.016 to 1.059 on both paths, `stream_memory.py` shows no growth over 500 reads (benchmarks.md 4.3), and no
     change since the baseline touches `crates/oradmp-core/src/`.
  4. **d9, d10 detection rules: accepted as built.** (e3) tries all three orders (Y-M-D, D.M.Y, M/D/Y) for each of
     `.` `/` `-`: a superset of ADR-65 item 4; a superset can only add false hits, and a false hit fails the test
     loudly (never a silent pass), so the wider net is the safe direction. A `-` before the first or last group is a
     sign (d9's `15.06.-100` is a year below 1), month and day are never negative. (e1)
     joins groups after a separator from `,` `.` `_` `'` space only when every later group has exactly 3 digits,
     and compares the **canonical** digit string of the joined token with the stored numbers: needed, since without
     it d10 would be found only through the accident of a `000000` run (the note's argument holds). The negatives in
     the self-tests (2-digit groups not joined; `15.06.-101`, `16.06.-100`, `15.06/-100`, `13.13.-100` not hits)
     are the right ones. If a later owned message legitimately contains a joined or dated token, the test fails and
     the Architect decides then; no allow-list now.
  5. **Release run B and Reviewer 5**: order unchanged: Reviewer session 5 (closure) -> run B. Reviewer 5 reads, in
     addition to the plan's list: (7) the ADR-64 amendment against `test_stream.py`'s two S-2 tests (iterator: issued
     at the error, not again; stream: at the `with` end); (8) that `469416d` also adds a root **`uv.lock`** and
     comments out the three `fuzz/` lines of the root `.gitignore`, neither named in the T-13i note: whether
     `uv.lock` is in the sdist (`check_dist.py` output) and who owns it (the root `.gitignore` is the owner's,
     O-16). A finding there is reported, not fixed by the Reviewer. The checklist steps are unchanged.
- Consequences: no core change before run B; one small core task after it; the memory test's limits stay and its
  stated strength is now what was measured; the strong check waits for the owner's file.
- Rejected: keeping the dead branch "as defence" (it would report `E-FRAMING` at another offset); documenting it as
  unreachable only (leaves a third truncation text in the error inventory for no case); a limit of 1.07 (inside the
  noise); a code change on the iterator path to issue at the `with` end (it would delay warnings for every plain
  `for` loop without a `with`, against arch 3.4); a narrower (e3) (fewer false hits are worth nothing when false hits
  are loud).
- Amendment (session 19, ADR-75 item 1): item 3's "The SQL exists (`corpus/sql/01b_optional_tables.sql`, commented
  out)" is wrong for the file that was made: `T_BIG` was made by `corpus/scripts/make_big.sh` with
  `corpus/sql/15_big.sql` (6 columns, 5,000,000 distinct ids), not from the 01b block (4 columns, repeating ids).
  The arithmetic stands (stop at 102,400 rows; the measured figures are in ADR-65's session 19 amendment).
- Amendment (session 18, ADR-71): T-13j is widened by review 05's SF-1, SF-3 and accepted notes; item 2 stands. Item
  3's "4,900,000 x b" is 4,897,600 x b with the test's actual stop at 102,400 rows (review 05 N-5; immaterial).

## ADR-70 Release run B (0.1.0a2) recorded: deviations, lessons, ADR-67 closed - decided (session 18; three confirmations need the owner, O-18)
- Context: the owner's record of run B (prompt of session 18, "Owner facts"): candidate `4e8f2122445cf089e183071c40c4e547c613c59c` [session 19: wrong; the built commit is `ff7d525`, ADR-74]
  (the README commit after Reviewer 5, who reviewed `dc7668e`); `tools/release_build.sh <commit> --twice`: wheel and
  sdist identical, "reproducible"; wheel `oradmp-0.1.0a2-cp311-abi3-manylinux_2_28_x86_64.whl` sha256
  `255cb122dcca3979e171984f89a719a293770709baf09a47936335f80b9b0f1c`, sdist `oradmp-0.1.0a2.tar.gz` sha256
  `cae95420b83d29866f8b939d8427cd857902a0055abe466baa66e9753c88702d`; install test in a Rocky Linux 8 container
  (python3.11, pyarrow 26.0.0): 113 passed, 4 skipped (the 3 probe tests and 1 "not a git clone"), with
  `Cargo.toml`, `Cargo.lock`, `README.md`, `pyproject.toml`, `tools/`, `docs/design/implementer-notes.md`,
  `fuzz/Cargo.lock` staged next to the copied `tests/` and `corpus/fixtures`; a real RHEL 8 host: not done; TestPyPI
  upload 2026-10-22 by `uv publish --publish-url https://test.pypi.org/legacy/ --check-url https://test.pypi.org/simple/
  <wheel> <sdist>`, index sha256 equal to the two above, `pip install --no-deps --index-url https://test.pypi.org/simple/
  "oradmp==0.1.0a2"` works, `import oradmp` gives 0.1.0a2, `python -m oradmp survey --help` works, the page shows the
  licence and "Requires Python >=3.11", the account-wide token replaced by a project-scoped one; PyPI upload done
  2026-10-22; tags `v0.1.0a2` and `v0.1.0a1` pushed; Q-10 answered "upload". Checked in the repository in this
  session: the commits, the README diff, the tag objects, the epoch rule of `tools/release_build.sh`.
- Decision:
  1. **Run B is done**; the record goes into the plan ("First release", "Run B record"). Q-10 is closed (upload, done).
     What is not done: the install on a real RHEL 8 host (checklist step 8; still open, as under Q-4), and the items
     of O-18.
  2. **Deviation: a shipped file changed after the closure review.** `4e8f212` changes `README.md` (one line replaced
     by one; the README is the wheel's METADATA description and is in the sdist) after Reviewer 5 read `dc7668e`, so
     the gate's "`git diff --stat <T-13i version commit> <candidate>` lists only `docs/`" (ADR-67 item 3) did not
     hold. It was the owner's decision on review 05 SF-2 (option (b)), and the diff was checked: `git diff --stat
     dc7668e 4e8f212 -- . ':!docs'` lists only `README.md` (1 insertion, 2 deletions; repeated in session 18), and the
     build ran `check_dist.py`, which reads the README (old name, description), with `check_dist: ok`. **Accepted, no
     re-review** (text only; SF-2 itself asked the Architect only whether a short re-check is needed: not for one
     sentence that `check_dist.py` reads). Rule from now on (checklist step 1): a shipped-file change after the
     closure review is allowed only as the owner's decision on a review finding, with the `git diff --stat` in the
     release record and `check_dist.py` and `tests/test_project_name.py` green on the candidate in a git clone. Left
     for a3: the new line dropped "Python 3.11 or later, pyarrow 15 or later" from the top of the README and gives the
     two commands without code marks (T-13j (g)).
  3. **Deviation: the fourth skip.** "not a git clone" is `tests/test_project_name.py` (`tracked()` skips without a
     `.git`): the container held copied folders, not a clone. Explained, so step 7's "any other skip stops the release
     until explained" is met. The test read the same README line 3 green on `dc7668e` (Reviewer 5) and the commit
     did not touch that line. Whether checklist step 3 ran on `4e8f212` is not in the record (O-18 (c)). From now on the
     container test runs from the build clone `$ORADMP_BUILD_ROOT/src` (a git clone with every file the tests read),
     mounted read-only, so the expected skips are again exactly the three probe tests (lesson 6).
  4. **Found in session 18: the tag is not on the built commit.** [Session 19: it is; the build was of `ff7d525`, ADR-74.] In this repository `v0.1.0a2` is an annotated tag
     (tagger date 2026-10-22 14:23:28 -0500) whose object is **`ff7d525`**, not `4e8f212`. `ff7d525` differs from
     `4e8f212` only under `docs/` (Architect 17 and Reviewer 5 documents), which neither archive contains, but
     `tools/release_build.sh` takes `SOURCE_DATE_EPOCH` from the commit time (`git log -1 --format=%ct`): 1792691228
     for `4e8f212`, 1792692632 for `ff7d525`. So a build of the tag gives other bytes than the uploaded files (by the
     script's rule; not rebuilt here), and ADR-63 item 2 (the tag on exactly the commit the build printed) is not met.
     Decision: the owner reads the `commit:` line of the run B build log; if it is `4e8f212`, he moves the tag (`git tag
     -f -a v0.1.0a2 -m 0.1.0a2 4e8f212`, then `git push --force origin v0.1.0a2`); if the log says `ff7d525`, the
     record's candidate is corrected instead and the tag stays (O-18 (a)). The remote could not be read from the VM
     (no credentials), so the pushed tag's target is the owner's to confirm.
  5. **Lessons** go into the checklist (plan): (1) `uv publish --dry-run` asks for the token: it is not a
     credential-free check, so the step does not rely on it; (2) the two files are named one by one, never by a glob
     (`dist/` also holds `SHA256SUMS`); (3) a project-scoped token can be made only after the first upload; the
     owner types the token with `read -rs` and `unset`s it after the upload; (4) the install check uses
     `--index-url` (not `--extra-index-url`) and `--no-deps`, with pyarrow installed from PyPI first; (5) step 6
     says `--twice`; (6) the container install test runs on the whole build clone (item 3).
  6. **ADR-67 is closed** (amendment there). PyPI: nothing remains to upload. Not in the owner's record: the PyPI
     comparison lines of step 10 (c) (index sha256 of both files against the two above; the install from
     `https://pypi.org/simple/` and `python -m oradmp survey`), and whether the PyPI token too is now project-scoped
     (the record says so for one token): O-18 (b). Risk 22 is closed for 0.1.0a2 and stays as a standing check for
     every later upload.
- Consequences: the a3 run uses the corrected checklist; the release is reproducible from its tag once O-18 (a) is
  done.
- **Amendment (session 19, ADR-74)**: the built commit was **`ff7d525`**, not `4e8f212` (owner: `git -C
  /var/tmp/oradmp-release/src rev-parse HEAD` of the build clone). Read every `4e8f212` of items 2 to 4 as "the
  candidate `ff7d525`" (same shipped files: `4e8f212` and `ff7d525` differ only under `docs/`); item 4's tag is on the
  built commit, so no tag move; `v0.1.0a1` was **not** pushed (local only). O-18 is closed (ADR-74).
- Rejected: a re-review of `4e8f212` (one README sentence, read by `check_dist.py`); leaving the tag on `ff7d525`
  without a correction (the release could not be rebuilt from its tag); a second tag for the same release (two
  names for one set of files).

## ADR-71 Review 05 into T-13j; what goes into 0.1.0a3 and when - decided (session 18; the Project-URL needs the owner, O-19)
- Context: Reviewer session 5 (`docs/review/review-05-t13i-closure.md`, HEAD `dc7668e`): no blocking finding; SF-1,
  SF-2, SF-3; N-1 to N-10. T-13j (ADR-69 item 2) already removes the unreachable LOB-length branch after run B.
  Run B is done (ADR-70), so shipped files can change again; they ship with 0.1.0a3.
- Decision:
  1. **SF-1 accepted**: the iterator S-2 test calls `check_row_17_rounded_before_row_19(reader)` after its
     `pytest.raises` block. Done when mutation **s1o** (review 05, SF-1, exact text) fails it. T-13j (d).
  2. **SF-3 accepted, fixed, not reworded**: `check_dist.py` guards every upload, and a3 is an upload. One synthetic
     case per rule, in the style of m1 to m9: the seven of review 05's table (sdist file name; old-name scan of sdist
     member contents with its allowed line; `Requires-Dist`; `License-Expression`; `Requires-Python`; the required
     sdist files; both licence files in the wheel) and the five review 05 lists as "same kind, by reading"
     (`Metadata-Version >= 2.4`; the `License-File` lines; "the description is the README"; no `License ::`
     classifier; METADATA or PKG-INFO missing). Cases **m10 to m21**; each rule removed alone fails exactly its case.
     If a case shows that a rule crashes instead of printing a `FAIL` line (for example a missing `PKG-INFO` read with
     `extractfile`), making it print the `FAIL` line is in scope. T-13j (e). ADR-64's sentence then holds.
  3. **SF-2 closed** by `4e8f212` (ADR-70 item 2); its leftover (the dropped minimum versions, code marks) is
     T-13j (g).
  4. **N-1 accepted**: the exempt list is completed (ADR-59 amendment, session 18); no wrap. T-13j (f) completes the
     `lib.rs` module comment.
  5. **N-2**: the owner keeps the root `uv.lock` (owner fact); it needs an owner in `CLAUDE.md` (O-19 (b):
     recommendation, the Implementer's row, beside `Cargo.lock`). The root `.gitignore`: O-16 (see its status).
  6. **N-3 accepted**: `t14_damage.rs`'s doc comment says the offset is where the failed read stood (`io.rs`
     `truncated()`, the current position), not where the primitive began. T-13j (c), which touches the file anyway.
  7. **N-4 accepted**: CONTRIBUTING says the script removes the `oradmp-fuzz-*` folders and **moves** `fuzz-0.log`
     into the tmp folder. T-13j (i).
  8. **N-5 accepted**: `t15_memory.rs`'s comment calls `T_BIG`'s one-byte resolution arithmetic until T-15b has
     measured it; ADR-69's figure corrected by amendment. T-13j (j).
  9. **N-6 accepted**: step 6 says `--twice` (checklist, now); a test that every version in the README equals the
     Cargo version (T-13j (h)); the checklist says the hashes depend on the commit (epoch) and the build root (SBOM).
  10. **N-7**: moot for TestPyPI's name (run B uploaded `oradmp` there); `--check-url` is in the checklist; the index
     lag note too (a missing file right after an upload is retried, not a mismatch).
  11. **N-8 accepted**: the README's two licence links become absolute URLs (T-13j (g), with a test that no README
     link has a relative target). **Project-URL**: only with a URL that a PyPI visitor can open (O-19 (a)); if the
     owner names one before a3, T-13j (g) adds `[project.urls]` and a `check_dist.py` rule with its case; if not, a3
     ships without one (the page then has no source link; not a defect).
  12. N-9, N-10: no action.
  13. **Model and review of T-13j**: **Sonnet** (tests that make checks stricter, comments, the README, one
     behaviour-neutral core change with its unit test; a wrong test there fails loudly, it cannot weaken a check).
     **No separate closure review**: T-13j, T-16 and T-18 form one group under rule 7 of `CLAUDE.md` (as T-13h, T-14
     and T-15 did, ADR-60); **Reviewer session 6** reads the group before 0.1.0a3 is built and before T-17 starts.
     T-13j's note carries its mutation table so that the Reviewer repeats it. T-15b changes no file and needs no
     review of its own (its numbers are read by Reviewer 6 if it is done by then).
  14. **0.1.0a3**: built from `master` after Reviewer session 6 reports no blocking finding, by the checklist
     (step 3b due: T-13j changes `crates/oradmp-core/src/`). Content: T-13j (the branch removal, README, Project-URL
     if any, the stricter tests), T-16's accepted platform wheels, T-18's command-line tool. **Not right after T-13j**:
     a2's page defects are cosmetic (the absolute licence URLs stand next to the broken relative links), and a
     release run costs the owner about a day; a release with the command-line tool and more platforms is worth that.
     Exception: a defect of 0.1.0a2 that harms a user (a wrong value, a crash, an install failure) makes a3 a fix
     release from `master` after T-13j and a short closure review. If T-16 is still blocked by Q-11 when T-18 is done,
     a3 may go out Linux-only (the owner decides then).
- Consequences: T-13j grows from one core change to a test-and-text task of about twelve items; nothing ships until
  a3.
- **Amendment (session 19, ADR-76)**: item 13's group is split: Reviewer session **6a** reads T-13j, T-15b, T-18 and
  T-18b now; **6b** reads T-16 when it exists. Item 14: 0.1.0a3 is built after 6a, **without T-16** (Linux x86_64
  only; the owner does not want to wait for the GitHub account); T-16's wheels go into a later release. Item 5 (the
  `uv.lock` owner): see O-19 (b) in `open-questions.md` (owner says done, not yet visible in the tree). Item 11: no
  Project-URL in a3 (ADR-76 item 4).
- Rejected: rewording ADR-64's claim instead of testing the rules (the checks guard an irreversible upload); a
  closure review right after T-13j (nothing ships before a3, and Reviewer 6 reads the whole group); releasing a3
  right after T-13j (cosmetic gain, a day of owner time, a benchmark run).

## ADR-72 T-18: the command-line tool `oradmp` - decided (session 18)
- Context: the owner wants a thin tool on the Python API: list the tables of a dump, show a table's columns, print
  the first N rows, export a table (and all tables) to CSV or Parquet; `survey` stays. The API already has
  everything (`open`, `Dump.tables`, `TableInfo.columns`, `read_table` with `columns` and `batch_rows`, typed errors
  with stable codes, `DumpWarning.text`).
- Decision:
  1. **Scope**: a new module `python/oradmp/_cli.py` that calls only the public API of `oradmp` (no `_core`, no new
     reading logic, no change in `crates/`). `python -m oradmp` calls the same `main`. It ships **in the same wheel**,
     with no new dependency.
  2. **Console script** `oradmp` = `oradmp._cli:main`, declared in `pyproject.toml` `[project.scripts]` (PEP 621; the
     wheel's `.dist-info/entry_points.txt` holds it).
  3. **Commands** (arch 3.5 has the full text):
     `oradmp tables FILE`; `oradmp columns FILE TABLE`; `oradmp head FILE TABLE [-n N]` (default 10); `oradmp export
     FILE TABLE -o PATH [--format csv|parquet]`; `oradmp export FILE --all -o DIR [--format csv|parquet]`; `oradmp
     survey FILE [--no-names] [--no-read]` (unchanged: same output and exit codes 0 / 1). A table is named by its
     name, or by `--ordinal N` instead of the name. Read options mirror `open` / `read_table`: `--strict`,
     `--on-value-error`, `--on-number-overflow`, `--number-default`, `--number-type TABLE.COLUMN=TYPE` (repeatable),
     `--interval-as`, `--timestamp-unit`, `--trim-char`, `--max-lob-bytes`, `--unconfirmed-lob`, `--db-charset`,
     `--client-charset`, `--columns A,B,...`. Not exposed: `extra_intro_lines`, `max_line_bytes`, batch sizes.
  4. **Defaults that differ from the API**: `--interval-as string` (the tool writes only text and Parquet, and
     pyarrow's Parquet writer refuses Interval(MonthDayNano), README "pandas and Parquet"); everything else is the
     API's default.
  5. **Output**: `tables` and `columns` print tab-separated text with a header line (tab, newline and backslash in a
     name written as `\t`, `\n`, `\\`); `head` writes CSV to stdout; `export` writes CSV (`pyarrow.csv.CSVWriter`,
     batch by batch, header line, pyarrow's text of each type) or Parquet (`pyarrow.parquet.ParquetWriter`, batch by
     batch, pyarrow's defaults). Binary columns (RAW, LONG RAW, BLOB) are written to CSV as upper-case hex (as Oracle's
     `RAWTOHEX`). A column type that pyarrow's CSV writer refuses stops before any row is written (`E-CONFIG`, naming
     the column and the Arrow type); the task lists such types, if any, on pyarrow 15.0.2 and the newest, and the
     Architect decides a text form for them later. Data go to stdout only; errors and warnings to stderr only.
  6. **Reading path**: the iterator (`for batch in reader`), never `__arrow_c_stream__`, so every error arrives typed.
     Warnings are printed once each, as `oradmp: warning: <DumpWarning.text>`, after the scan (`Dump.warnings`) and
     after each table's read (`BatchReader.warnings`); the `warnings` module's own output of `OradmpWarning` is
     suppressed inside the tool, so nothing is printed twice. `-q` hides warnings.
  7. **Files**: an export writes `<target>.oradmp-partial` in the target's folder and renames it (`os.replace`) only
     after the last batch; on any error or interrupt the partial file is removed. An existing target is refused
     (`E-CONFIG`) before reading unless `--overwrite`. `--all`: one file per table, `<NAME>.<ext>` when the name
     matches `[A-Za-z0-9_$#]+` and is unique among the dump's tables ignoring case, otherwise `table_<ordinal>.<ext>`;
     a tab-separated manifest on stdout (`ordinal`, `name`, `file`, `rows`); the first error stops the run (files
     already written stay complete; the failing table leaves none). `-o -` writes CSV to stdout (Parquet to stdout
     is refused).
  8. **Exit codes**, by the exception's class from `ERROR_CLASSES` (the code-to-class table of `_errors.py`, so every
     code of `_core.ERROR_CODES` has one): **0** done (warnings allowed); **1** the dump cannot be read as asked:
     `FormatError`, `UnsupportedError` (`UnsupportedCharset` included), `ValueDecodeError`, `StrictError`; **2** usage:
     argparse's own errors, `UsageError` (`E-CONFIG`, `E-NO-SUCH-TABLE`, `E-AMBIGUOUS-TABLE-NAME`,
     `E-NO-SUCH-COLUMN`, `E-FILE-CHANGED`) and the tool's own refusals, printed as `[E-CONFIG] ...`; **3** `E-IO`
     (`OSError` on the dump or the output; a closed stdout during `export -o -`); **4** `E-INTERNAL`; **130** an
     interrupt. The error line on stderr is `oradmp: error: ` + `str(e)`, which starts with `[<code> offset=0x...]` and
     holds no value (arch 3.4). No traceback, except for a Python exception that is not an `OradmpError` or `OSError`
     (a bug of the tool: traceback, exit 1 by Python's default). A closed stdout during `head` ends quietly with 0. A
     scan that stops (`Catalog.end == "stopped"`) prints the tables before the stop, then the stop error, and exits
     with that error's code.
  9. **Not in phase 1**: Data Pump; row filters, `WHERE`, row ranges other than `head -n`; SQL; joins; JSON or Excel
     output; Parquet or CSV writer options beyond the defaults; parallel export; reading from stdin or URLs; pandas.
  10. **pandas**: never imported by the tool (pyarrow's CSV and Parquet writers do not need it). A pyarrow build
     without `pyarrow.parquet` gives `E-CONFIG` ("this pyarrow has no Parquet support") when Parquet is asked for; the
     import is lazy.
  11. **Wheel checks that change**: `check_dist.py` gains a rule that the wheel has `<dist-info>/entry_points.txt` with
     exactly one section `[console_scripts]` and exactly `oradmp = oradmp._cli:main` (no `oradmp-<v>.data/` folder:
     the existing "outside `oradmp/` and the dist-info" rule already refuses one), with its mutation case; the sdist's
     required files gain `python/oradmp/_cli.py`; checklist step 7 runs `oradmp --help` and `oradmp tables` on a
     copied fixture from the venv's `bin/`, and one install venv **without pandas** runs `oradmp export` to CSV and to
     Parquet; step 8 runs `oradmp tables` on the RHEL 8 host.
  12. **Order**: after T-16 (the owner's order), **before T-17**: it is user-visible and part of a3, T-17 is coverage
     and no release gate; and after T-16 the CI runs the tool's tests on every platform from its first commit (paths,
     `os.replace` on Windows). Model: **Sonnet**; reviewed in Reviewer session 6.
- Consequences: one new public surface (command names, options, exit codes) that later versions keep or change by a
  record; the README gains a "Command line" section in T-18.
- Rejected: a second package or wheel for the tool (one more release artefact for a thin layer); pandas for CSV
  (a heavy dependency for what pyarrow does); `--interval-as native` as the tool's default (every Parquet export of an
  INTERVAL table would fail); the Arrow C stream path in the tool (untyped errors); one exit code for every error
  (an ETL scheduler cannot tell a bad dump from a bad command line); click or typer (a dependency; argparse is
  already used by `survey`); writing the target in place (a failed export would leave a file that looks complete).
- **Amendment (session 19, ADR-75 items 2 to 6)**: T-18 is done (`2ba62eb`). Item 5: Parquet stores a DATE column
  (Timestamp(second)) as milliseconds and reads it back as Timestamp(millisecond) (the Parquet TIMESTAMP logical type
  has the units MILLIS, MICROS, NANOS only; Apache Parquet format, `LogicalTypes.md`); same instants; the tool keeps
  pyarrow's defaults and the round-trip test compares after `timestamp[s]` -> `timestamp[ms]`. Item 6: `head` prints
  no counted warning (they exist only at the end of a read, ADR-55 item 2); documented, no core change. Item 7:
  `--all` needs an existing folder; `--all` and `-o -` default to CSV. Item 8: a single-table command whose TABLE is
  not found in a **stopped** scan prints the stop error and exits with its code (T-18b). Synopsis: the table comes
  right after the file (Python 3.11's argparse rejects `head FILE -n 3 TABLE`). Accepted additions: `--version`; `-q`
  on every command but `survey`; `true` / `false` and an empty field in `tables` / `columns`; a closed stdout of
  `tables` / `columns` ends quietly with 0 like `head`; `survey` keeps its own error text on stdout and exit 1 for a
  missing file (item 3's "unchanged" wins over item 8's exit 3).
- **Amendment (session 20, ADR-77 item 4)**: item 8's error line for an `OSError` of the API names the reason and the
  file: `oradmp: error: [E-IO] <os.strerror(errno)>: FILE` (the API's prefix kept, offset included when present);
  exit 3 unchanged; output-side `OSError`s keep `[E-IO] <strerror>: <filename>`. T-18c.

## ADR-73 Order after run B; the DuckDB side project as a question only - decided (session 18; Q-11 and Q-13 need the owner)
- Context: the owner's order: T-13j, (T-15b when `T_BIG` exists), a3 if decided, T-16, T-18, T-17, Data Pump as
  phase 2, the DuckDB side project last. ADR-71 places a3 after Reviewer session 6.
- Decision: the order table of the plan (session 18): T-13j; T-15b whenever O-17 is done (alone, after T-13j); T-16
  (Q-11); T-18; Reviewer session 6 (T-13j, T-16, T-18); release 0.1.0a3; T-17; Data Pump (phase 2, planned when
  it starts); the DuckDB extension last, not planned (**Q-13**: what must be checked first, and the conditions the
  core keeps meanwhile). If Q-11 is still open when T-13j is done, the recommendation is to run T-18 before T-16
  (it needs nothing from the owner); that swap is the owner's call (asked under Q-11).
- Consequences: no task waits for the DuckDB question; the core keeps the conditions of Q-13 from now on.
- Rejected: planning the extension now (DuckDB's extension options are not checked, and the core's API is not yet
  declared stable); a3 before T-16 and T-18 (ADR-71 item 14).
- **Amendment (session 19, ADR-76)**: the swap happened (T-18 ran before T-16). New order: T-18b (with the a3 version
  commit); Reviewer session 6a; release 0.1.0a3 (Linux only); T-17; T-16 when the GitHub account and mirror exist
  (Q-11); Reviewer session 6b; the next release with T-16's wheels; Data Pump; DuckDB last.

## ADR-74 The 0.1.0a2 record corrected: built commit `ff7d525`; O-18 closed - decided (session 19)
- Context: the owner's facts of session 19: the uploaded files were built from
  `ff7d52559551a973621c68154e03dba1a157731a` (`git -C /var/tmp/oradmp-release/src rev-parse HEAD`); session 18 and its
  prompt said `4e8f212`. The tag `v0.1.0a2` points to `ff7d525` and is on the server; `v0.1.0a1` (on `b7304d5`) is
  local only. PyPI (2026-10-22): the `#sha256=` values of `https://pypi.org/simple/oradmp/` equal `dist/SHA256SUMS`
  (wheel `255cb122...0f1c`, sdist `cae95420...702d`); `oradmp==0.1.0a2` installed from pypi.org in a fresh venv and
  ran. Both indexes now have tokens scoped to the project `oradmp`; the account-wide tokens are deleted. A stray file
  `UV_PUBLISH_TOKEN` in the build root's `dist/`, made during the upload, was found and removed. Checked here:
  `git rev-parse v0.1.0a2^{}` = `ff7d525...`; `v0.1.0a1^{}` = `b7304d5...`; `ff7d525`'s commit time 1792692632 is the
  epoch of the uploaded files; `git diff --stat 4e8f212 ff7d525` lists only `docs/` files; `git diff --stat dc7668e
  ff7d525 -- . ':!docs'` lists only `README.md` (1 insertion, 2 deletions).
- Decision:
  1. The record's candidate is **`ff7d525`** everywhere (plan "Run B record", ADR-70 amendment). The deviation of
     ADR-70 item 2 stands unchanged in substance (the shipped files differ from the reviewed `dc7668e` by the one
     README line).
  2. **O-18 (a) closed**: the tag is on the built commit; nothing to move; the release can be rebuilt from its tag
     (same build root: the SBOM holds the path, ADR-70 item 5). **Risk 23 closed.**
  3. **O-18 (b) closed**: PyPI comparison equal; install from pypi.org works; both tokens project-scoped. The stray
     file could hold only a token used for the uploads, and the uploads used the account-wide tokens (a project
     token can be made only after the first upload), which are deleted: nothing more to revoke. If the owner knows
     the file held a project-scoped token, he revokes that token.
  4. **O-18 (c) closed without a run**: step 3 was not reported on `ff7d525`. Accepted for a2: its shipped tree is
     `dc7668e` plus one README line; Reviewer 5 ran the full checks on `dc7668e` (411 + 3 ignored, 117 pytest, 0
     skipped) and the build ran `check_dist.py` (`check_dist: ok`). Nothing remains for a2. For a3, step 3 runs on
     the candidate itself (T-18b (h) and Reviewer 6a).
  5. `v0.1.0a1` stays local unless the owner pushes it (optional; recommendation: push, so the hand-out's commit is
     recorded on the server; nothing on an index refers to it).
  6. Checklist lesson (step 10 (5)): after the uploads `ls -A <dist>` lists exactly the wheel, the sdist and
     `SHA256SUMS`; anything else is removed, and if it can hold a token, that token is revoked.
- Consequences: the run B record is complete except step 8 (a real RHEL 8 host), which stays open under Q-4.
- Rejected: moving the tag to `4e8f212` (the uploaded bytes come from `ff7d525`); a re-run of step 3 on `ff7d525` now
  (the code is superseded on `master`; a3 runs it on its own candidate).

## ADR-75 T-15b's results; the T-18 rulings; task T-18b - decided (session 19)
- Context: T-15b note (code of `460aef5`; `T_BIG` 453,304,320 bytes, SHA-256 equal to `SHA256SUMS_big`): correct
  reader 3 of 3 pass (T_BIG ratios 1.0016, 1.0252, 1.0137); retained b bytes per row in `TableSink::row_end`: b = 8
  fails on the `T_ROWS` files first (1.1014 to 1.1033) and, with `T_ROWS` out of the way, on `T_BIG` (5.88 to 5.98);
  b = 4 (3.69 to 3.74) and b = 1 (1.78 to 1.96) fail 3 of 3 on `T_BIG`; one `t15_throughput` run (page cache): T_BIG
  scan 785.3 MB/s, read 2,195,749 rows/s (199.1 MB/s of row area); the T_ROWS lines of the same run 809.1 / 886.7
  MB/s scan, 2.17 / 2.45 M rows/s read. The test stops at the first failing file. T-18 note: findings 1 to 7.
- Decision:
  1. **Memory test**: ADR-65 item 6 holds (amendment there). The test is changed to **measure every file present,
     then assert**, with a failure message that lists each file's ratio (the Implementer's suggestion; T-18b (a)).
     Reason: a leak that fails `T_ROWS` should still show whether `T_BIG` fails; diagnosis, not strength. The limit
     1.10 and the stop points stay. Who: the Implementer in T-18b; Reviewer 6a repeats it.
  2. **Benchmarks**: T-15b's numbers are **not a baseline** (benchmarks.md rule 1: a Reviewer repeat is missing, and
     the micro-benchmarks were not run). They satisfy a3's step 3b **for throughput and memory only**, as a "no
     regression" check (no core source change between `460aef5` and `HEAD`): the T_ROWS figures are 0 to +7 % from
     the baseline (reading the note's two figures per quantity as conventional / direct, the baseline's order).
     The micro-benchmarks (`cargo bench --bench decode`) are still due for a3: T-18b (b). Reviewer 6a repeats
     throughput, memory and the micro-benchmarks; if they agree under rule 1, the next Architect session makes them
     the new baseline (code of `460aef5`), with `T_BIG` rows.
  3. **Parquet and DATE** (T-18 finding 1): **accepted**: the tool keeps pyarrow's defaults; a DATE column comes back
     as `timestamp[ms]`; the README says so (it does). Rejected: casting DATE to `timestamp[ms]` before writing (same
     file, more code), int64 seconds (loses the type), `coerce_timestamps` options (writer options are not in phase 1,
     ADR-72 item 9).
  4. **"Exactly one warning line"** (finding 2): the test as built is **accepted** (patched `T_NUM`: one
     `W-NUMBER-ROUNDED` line and the error line; it is the same patched copy as the S-2 test of T-13i (b)). It does
     not show a warning on a run that **succeeds**; T-18b (d) adds that case on an unpatched fixture: 18c `T_NUM` with
     `--on-value-error null` reads to the end (33 rows; `W-VALUE-NULLED` and `W-NUMBER-ROUNDED`, ADR-55's test data),
     so `export` exits 0 and stderr holds exactly the lines `oradmp: warning: ` + each text of
     `BatchReader.warnings` of the same read through the API, in order; none with `-q`. (Expected from ADR-55's
     test data; if the run does not exit 0, the Implementer stops and reports.)
  5. **`head` and the counted warnings** (finding 3): **left as documented; no core change.** ADR-55 item 2 makes
     counted warnings complete only at the end or at an error, on both paths; making `close()` report counts so far
     would put partial counts into `BatchReader.warnings` and issue them at `__exit__` for every reader closed early
     (with a warnings filter set to `"error"`, an exception at `with` exit), and the counts would cover whole decoded
     batches, not the N rows `head` printed. The README sentence is widened in T-18b (e): `head` prints no warning of
     the table's read; a value it shows may be rounded or NULL without a warning; `export` reports them. A "warnings
     so far, marked partial" API is a later candidate, its own record when a user needs it (with the typed
     `BatchReader.error` of ADR-55 item 6).
  6. **Open choices of ADR-72** (finding 6), one line each:
     - flags `true` / `false`, "none" an empty field: accepted (tab-separated text for scripts).
     - `--all` needs an existing folder: accepted (no surprise folders); `--all` and `-o -` without `--format` are
       CSV: accepted (the only format that can go to stdout; one default).
     - only `tables` and `export --all` report a stopped scan: accepted for a table that was found (it was read in
       full); **changed** for a name not found in a stopped scan: print the scan's stop error and exit with its code
       (1 for a format stop), not `E-NO-SUCH-TABLE` / 2, because the table may lie beyond the stop (T-18b (c)).
     - `--version`: accepted (`oradmp <version>` from `oradmp.__version__`).
     - `-q` on every command but `survey`: accepted (`survey` has its own output contract).
     - `--columns` applies to every table of `--all`: accepted (a table without the column stops the run with
       `E-NO-SUCH-COLUMN`, as the first-error rule says).
     - closed stdout of `tables` / `columns` ends quietly with 0: accepted (same reason as `head`).
     - `survey`: its own `ERROR: ...` on stdout and exit 1 for a missing file: accepted (ADR-72 item 3, unchanged
       output); the README's exit-code list names the exception (it does: "`survey` returns 0 or 1").
  7. **Python 3.11 argparse** (finding 4): accepted; the synopsis order (table right after the file) is the
     contract; the README says it.
  8. **T-18 (l)** ("green on every CI platform") moves to T-16 (its (c) runs the full `pytest`, `tests/test_cli.py`
     included). T-18 (j)'s dry build on a scratch commit is accepted; the real build of the a3 candidate is Reviewer
     6a's.
  9. **New task T-18b** (plan): items 1, 2, 4, 5, 6 above, then the a3 version commit. Sonnet; reviewed in 6a.
- Consequences: one small task before the review; the tool's surface changes in one case (a missing table in a
  stopped scan).
- Rejected: the core change for `head` (item 5); another fixture for the warning case (an unpatched fixture already
  gives it); leaving the memory test as it is (a hidden second failure costs a diagnosis run, as T-15b needed).

## ADR-76 Reviewer session 6 split; 0.1.0a3 without T-16; T-17 after the a3 tag; the Project-URL - decided (session 19; the Project-URL timing needs the owner, O-19 (a))
- Context: T-16 waits for a GitHub account (Q-11: GitHub Actions through a push mirror from Gogs; the account comes
  later; public or private undecided). The owner does not want to wait. T-13j, T-15b and T-18 are done and unreviewed
  (rule 7). The owner: Project-URL deferred until the GitHub repository exists, "it goes into 0.1.0a3".
- Decision:
  1. **Reviewer session 6a now** (after T-18b and the a3 version commit): T-13j, T-15b, T-18, T-18b; the repeated
     benchmarks (so they can become the baseline); the a3 candidate built with `--twice`; the CLI on the installed
     wheel. "Done when" in the plan. **Reviewer session 6b later**: T-16 (and T-17 if done by then), before T-16's
     wheels are released.
  2. **0.1.0a3 goes out without T-16**: Linux x86_64 `manylinux_2_28` only, as a2 (ADR-71 item 14 left this to the
     owner; his wish not to wait is the answer). The owner confirms it at checklist step 1. T-16's accepted wheels
     ship in the release after Reviewer 6b (0.1.0a4 or later).
  3. **T-17** can run without the owner (synthetic files in a temporary folder; no fixture, no data, no account).
     It starts **after the a3 tag** (step 9): it adds files under `crates/oradmp-core/tests/`, which would move
     `master` away from the reviewed candidate if it ran earlier. Its "Done when" is written by the next Architect
     session (after Reviewer 6a), which re-reads the rules for it. Reviewed in 6b.
  4. **Project-URL**: a3 ships **without** one, unless the owner prefers a3 to wait for a public GitHub repository
     (O-19 (a), asked again). A private repository gives no Project-URL at all (a PyPI visitor could not open it).
     When the public URL exists, `[project.urls]`, the `check_dist.py` rule and its case go into the first task after
     it (T-16 is the natural place).
- Consequences: a3 brings T-13j's fixes and the command-line tool now; the platform wheels wait for the account.
- Rejected: one Reviewer 6 after T-16 (holds a3 for an account with no date); a3 without any review of T-13j / T-18
  (rule 7); T-17 before the review (it would join the reviewed group late and move the candidate).
- **Amendment (session 20, ADR-77, ADR-79)**: Reviewer 6a found nothing blocking; a3's candidate becomes the commit of
  **T-18c**, read by a short **Reviewer 6a-closure**; `d4d20ef` is not released. Item 4: the owner confirms a3
  without a Project-URL (it needs a public repository; expected in a4). T-17 starts after the a3 tag is pushed.

## ADR-77 Reviewer 6a recorded; T-18c and a closure review before the a3 upload; the a3 candidate - decided (session 20)
- Context: Reviewer 6a (`docs/review/review-06a-t13j-t18b.md`) reviewed `d4d20ef` ("Version 0.1.0-alpha.3"; HEAD
  differs from it only under `docs/`): **no blocking finding**; release build reproducible, `check_dist: ok`, version
  `0.1.0a3`, entry point present, full checks green (412 + 3 skipped; 198 pytest, 0 skipped); every T-13j, T-18 and
  T-18b mutation it repeated fails its test; benchmarks and memory within 9 % of the Implementer's. **SF-1**: four
  subprocess tests of `tests/test_cli.py` force `PYTHONPATH=ROOT/python`, so against an installed wheel they fail in a
  git clone (4 failures in all three venvs) and the pandas scan fails in a copied folder (1 failure): checklist step 7
  cannot pass as written. **SF-2**: seven mutations of `_cli.py` survive (`.parquet` suffix, `-n -1`, `--all -o -`,
  `--version`, `--trim-char`, `--number-type`, `_escape`); six more options have no test. Notes: N-1 (a README version
  in prose is not checked), N-2 (README: format from the suffix not stated; `--interval-as` missing from the option
  list), N-3 (`oradmp tables missing.dmp` prints `oradmp: error: [E-IO] I/O error`, no reason, no path), N-4 to N-7
  (bisect noise, an unexplained `t14_damage.proptest-regressions` file, counts, ADR-75 rulings), N-8 (an idle Oracle
  container during the measurements); checklist steps 5, 9, 10 still name a2. Nothing of a3 is uploaded or tagged.
- Decision:
  1. **Recorded and closed**: T-13j, T-15b, T-18 and T-18b are reviewed (review 06a); the memory test and the
     benchmarks of T-15b / T-18b are closed and become baseline 2 (ADR-78). N-4, N-6, N-7: nothing to do (only the
     version commit is a candidate; counts agree; ADR-75 as built). N-5: every mutation procedure checks `git status`
     after each run (it already did); T-17 (j) sets the persistence of its own property tests and says whether the
     file can come from `t14_damage`.
  2. **T-18c now, then a short closure review (Reviewer 6a-closure), then the a3 upload** (the owner's lean).
     Costs, compared:
     - T-18c path: one Sonnet session (tests, about 40 lines of `_cli.py`, README) and one short Reviewer session
       (one release build `--twice`, three venvs, the mutations): about a day of machine time, two owner commits.
       A user of a3 gets an error line with the reason and the path, a README that says where the format comes
       from; the release record has no deviation (step 7 green as written; step 3 on the candidate itself, ADR-74
       item 4).
     - Release `d4d20ef` as it is: the upload can start now. A user of a3 sees `[E-IO] I/O error` for a missing
       file and for a permission problem alike (an ETL operator cannot tell them apart without `strace`); the README
       does not say that `-o x.parquet` chooses Parquet; step 7 is recorded with four explained failures. The fixes
       ride in a4 (which comes anyway with T-16), so a second release costs no extra upload; but the a3 files with
       the poor error text stay on PyPI for good (an upload cannot be replaced, only yanked), and a4's review must
       then also cover T-18c.
     Reason for the choice: the difference is a day against a permanent artefact; the project's rule since a2 is
     "step 3 on the candidate itself" and a green step 7; the fix touches no file of `crates/`, so the baseline,
     the Rust checks and step 3b stay valid.
  3. **T-18c scope** (plan, "Done when"): SF-1 (the tests find the package through `oradmp.__file__`); SF-2 (a test
     for each survivor and for every pass-through option); N-3 (the error text below); N-2 (two README sentences);
     N-1 (the README version check widened). Nothing in `crates/`, `pyproject.toml`, the Cargo files or
     `check_dist.py`. The checklist strings of steps 5, 9, 10 are the Architect's and are fixed in this session (the
     a3 checklist names `0.1.0a3`).
  4. **The I/O error text of the tool** (ADR-72 item 8 amended): for an `OSError` raised by the `oradmp` API (its
     `strerror` starts with `[E-IO`; the API opens nothing but the dump, so the file is always FILE) the stderr line
     is `oradmp: error: ` + the API's bracketed prefix (`[E-IO]` or `[E-IO offset=0x...]`) + a space + the operating
     system's text for the error number (`os.strerror(e.errno)`; the API's `I/O error` when there is no number) +
     `: ` + FILE as given on the command line (tab, CR and newline written `\t`, `\r`, `\n`; nothing else changed;
     **session 21, ADR-82 item 3**: a byte of the name that is not valid UTF-8 is written `\xHH` as well).
     Example: `oradmp: error: [E-IO] No such file or directory: missing.dmp`. Exit code **3** unchanged (ADR-72 item
     8, README). An `OSError` of the output side keeps today's form (`[E-IO] <strerror>: <filename>`). Commands:
     `tables`, `columns`, `head`, `export` (also `--all`); `survey` keeps its own text (ADR-75 item 6). A path and an
     OS message are not values of the file (arch 3.4). The Python API is **not** changed in a3
     (`oradmp.open("missing.dmp")` still raises `OSError` with `.errno` 2, `.code` `E-IO`, no `.filename`); filling
     `.filename` and the OS text there is a candidate for a4, its own record when scheduled.
  5. **Version and tag**: a3 keeps **`0.1.0a3`** (no index holds any a3 file; PyPI refuses only a file name that was
     uploaded). The version files are already a3 since `d4d20ef`, so there is **no version commit**: the new
     candidate is T-18c's last commit, which must be the last commit outside `docs/` when the build runs. `d4d20ef` is
     never tagged; Reviewer 6a's hashes (`e905e103...` / `bf480eaf...`) are void. The tag `v0.1.0a3` is made at step 9
     on the new candidate and **pushed only after the PyPI upload succeeded** (step 10 (g)): a local tag can still
     move, a pushed one should not.
  6. **Reviewer 6a-closure** ("Done when" in the plan): the T-18c diff, each mutation, the build twice, the
     installed-wheel suite from a git clone and from a copied folder, the CLI from the wheel, checklist step 3 in a
     clean clone. Its step 3 run is the release's step 3 (same commit).
  7. **Checklist lessons** (plan, "Release 0.1.0a3 checklist (session 20)"): expected skips per layout (git clone:
     the three probe tests; copied folders: also `test_project_name.py` "not a git clone"; a venv without pandas:
     also `test_consumers.py`); a new build root per release (`release_build.sh` refuses an existing one; a2's
     `/var/tmp/oradmp-release` stays, ADR-74 item 2); the build log kept with `tee`; the tag pushed last; the
     plain `pip install oradmp` behaviour with only pre-releases on the index is recorded, not assumed (pip's
     documentation says it finds only stable versions by default and does not state the outcome when only
     pre-releases exist).
- Consequences: one more small task and review before a3; no change to the core, the baseline or the wheel's
  content apart from `_cli.py`; the a3 record will name the T-18c commit.
- Rejected: release `d4d20ef` as it is and fix in a4 (item 2); a new version number `0.1.0a4` for the new candidate
  (nothing of a3 was published; a gap in the version list for no user benefit); fixing the text in the binding or
  the core (touches `crates/`, widens the review, changes the API without a record); a one-off step 7 deviation
  instead of SF-1 (the next release would hit it again).

## ADR-78 Baseline 2 and the regression rule - decided (session 20)
- Context: benchmarks.md rule 1 needs a Reviewer repeat within 15 %. Reviewer 6a repeated every T-15b / T-18b figure:
  micro-benchmarks -5.2 % to +3.4 %, `T_ROWS` throughput +0.2 % to +8.7 %, `T_BIG` -0.8 % / +1.7 %, memory ratios
  1.0116 to 1.0662 (all pass), b = 1 on `T_BIG` 1.743 to 1.930 (all fail); every figure at or above baseline 1. An
  idle Oracle XE container ran during the repeat (N-8); each run started at a load below 0.5.
- Decision:
  1. **Baseline 2** = code of `460aef5` (the core sources of `d4d20ef`), both columns, in benchmarks.md section 4;
     baseline 1 moves to section 6 (history). The figure rules compare against is the **slower** of the two columns.
  2. N-8: the repeat stands; a run with an **idle** container is acceptable when its note names it; a load above
     0.5 at the start is not. No clean repeat is asked.
  3. Rule 3: `T_BIG` throughput regresses at more than **10 %** below its baseline (5 to 10 % "watch"), because its
     honest spread is at most 6.0 % (single samples) and 1.7 % between sessions; `T_ROWS` and the micro-benchmarks
     keep 20 % (honest differences up to 13 %). Rule 4: per-file end-peak limits 15 % above baseline 2's highest end
     peak (`T_ROWS` conventional 9,108 kB, direct 9,021 kB, `T_BIG` 8,984 kB); the ratio limit 1.10 unchanged.
  4. `tools/stream_memory.py` was not repeated: its baseline 1 figures stay the reference.
- Consequences: step 3b of the next release that changes core sources compares with baseline 2; T-18c changes no core
  source, so a3's 3b is "not due".
- Rejected: one 20 % limit for every figure (it would let a 15 % slowdown of the 5,000,000-row read pass unseen); 10 %
  for `T_ROWS` (inside its honest spread); asking for a repeat without the container (the container was idle and the
  figures agree with the Implementer's runs without it).

## ADR-79 CI on GitHub Actions; what T-16 needs; `.github/` belongs to the Implementer - decided (session 20; the visibility and two settings need the owner, O-19 (a), O-22)
- Context: the owner has a **private** GitHub repository `granddad-wolf/oradmp` (remote `github` in his clone; `master`
  and `v0.1.0a2` pushed there and to the Gogs `origin`). Public or private is undecided. Q-11 recommended a hosted CI
  through a mirror. GitHub's Actions billing documentation (docs.github.com, "GitHub Actions billing", read
  2026-10-24): standard hosted runners are free for public repositories; a private repository on GitHub Free has
  2,000 included minutes a month, and the listed per-minute rates make a macOS minute about ten times a Linux 2-core
  minute ($0.062 against $0.006) and a Windows minute about 1.7 times. GitHub Actions reads workflows from
  `.github/workflows/*.yml` in the repository; no row of `CLAUDE.md` names `.github/`.
- Decision:
  1. **Q-11 closed**: GitHub Actions on `granddad-wolf/oradmp`. A push mirror from Gogs or pushes by hand (`git push
     github master`) both work; CI runs on a push to GitHub and on a manual `workflow_dispatch`.
  2. **`.github/` is the Implementer's** (it is "the CI configuration" of the Implementer's row, T-16). The owner
     adds it to `CLAUDE.md` (O-21; text there). Branch protection, Actions permissions and repository settings stay
     the owner's (GitHub web settings, not files).
  3. **T-16 needs now** (O-22): (a) Actions enabled for the repository; (b) `master` pushed to `github` after each
     commit that should be tested (mirror or by hand); (c) how the Implementer sees the results: recommended, the
     owner logs `gh` in on the VM with a fine-grained token limited to this repository, **read-only** for Actions and
     contents (the Implementer runs `gh run list` / `gh run view --log-failed`; it never pushes); alternative, the
     owner pastes the failed logs; (d) while the repository is **private**, the macOS and Windows jobs run only on
     `workflow_dispatch` (the owner starts them), Linux on every push; when public, every job on every push. No
     secret: CI never uploads in phase 1.
  4. Workflow rules for T-16: `permissions: contents: read` at the top; no `pull_request_target`; third-party
     actions pinned to a full commit SHA; the Rust toolchain from `rust-toolchain.toml`; `CARGO_BUILD_JOBS=2` and a
     `TMPDIR` on the runner's work disk (ADR-68).
  5. **Project-URL**: not in a3 (O-19 (a) answered by the owner: it needs a public address). When the owner makes
     the repository public, the first task after that adds `[project.urls]` with the `check_dist.py` rule and its
     case (T-16 (j)); expected in a4.
- Consequences: T-16 can start after the a3 tag once O-21 and O-22 are done; private runs cost minutes only when the
  owner starts the macOS / Windows jobs.
- Rejected: a CI on the Gogs server (no Windows or macOS runners); the macOS / Windows jobs on every push of a private
  repository (minutes); the CI configuration as the owner's file (it changes with the tests, which are the
  Implementer's).
- Amendment (session 21): item 2 is done (`9f1dc4d`: `CLAUDE.md` names `.github/workflows/` in the layout and in the
  Implementer's row; O-21 closed). Item 3 (O-22) is still open. What CI runs and does not run: ADR-83.
- Amendment (session 22): item 3 (c) is answered by the owner: **the owner pastes the CI logs**. The token route is
  withdrawn: `CLAUDE.md` rule 8 (since `3dc7a37`) says no agent ever sees or uses a GitHub token, so no `gh` login on the
  VM is asked for. What the Implementer asks for and what the jobs print for pasting: ADR-87 item 4.

## ADR-80 T-17: synthetic files from the rules - decided (session 20)
- Context: arch 8.3 (b), ADR-62 item 4, ADR-76 item 3: a generator that writes small exp files from the rules and a
  test that the reader returns what was written; after the a3 tag; reviewed in 6b. T-14 to T-18 taught: tests must
  write only below `target/` or a temporary folder; mutations must name the failing test; a property test can leave a
  `*.proptest-regressions` file (proptest's default failure persistence writes one beside the test source; proptest
  documentation, "Failure persistence"); review 04 N-9 found option sets no test drives; fuzz seeds come from
  `tools/fuzz_seeds.sh`.
- Decision:
  1. The generator is test code in `crates/oradmp-core/tests/` and follows `docs/formats/exp_reader_rules.md` only:
     it emits a construct as valid input only when its rule is **confirmed** there, and names the rule id beside the
     code that writes it. Constructs marked suggested or unknown appear only as negative cases whose expected result
     is the typed error the design gives them.
  2. Expected values are the generator's own input, never the reader's output and never reader code (the generator
     does not call `oradmp_core`'s decoders; NUMBER, DATE and TIMESTAMP bytes come from a test-side encoder written
     from [Y1], [Y2], [Y5]). It is common-mode with the reader's reading of the rules, so it adds coverage of
     combinations, not evidence; a disagreement stops the task for the Architect (the fixtures decide).
  3. No change to `crates/oradmp-core/src/`. Model **Opus** (faithfulness to rule statuses is the point). Needs
     nothing from the owner except the start; alone on the machine for the long property run and the fuzz run.
- Consequences: "Done when" in the plan; the synthetic files also seed the fuzz targets.
- Rejected: generating from the fixtures by patching (that is T-13i / T-14's patched-copy method; it cannot vary
  structure); expectations from the reader (circular); INTERVAL, ROWID, TIMESTAMP WITH (LOCAL) TIME ZONE and the
  CL8MSWIN1251 LOB forms in the first version (one table each in the fixtures; their text forms are the design's,
  not the rules').
- Amendment (session 22, ADR-86 item 1): item 1 reads: a construct is valid input when its bytes are confirmed **and**
  a fixture shows it in that combination (version, database set, client set, mode, path). A combination no fixture
  shows is generated only when every construct in it is confirmed on its own, a confirmed rule or a decided design rule
  says the reader treats it alike whatever the varying field ([E3] with arch 5.4: the version line never decides E1
  against E2; [C4]: the client set never decodes row data), and it is a variant of the generator's `Unconfirmed`
  list (named in every failure message and in the note, with its evidence route). Bytes confirmed for no combination
  (the TIMESTAMP(f) descriptor length for f outside {0, 6, 9}) and combinations the rules file names as not covered
  (a single-byte database on 18c) are not generated. So the plan's T-17 (d) "[E1, E2, E3]" stands, with the 18c E1 case
  as `Unconfirmed`. The plan's T-17 (g) "the ignored seed folders of the three targets" reads "the one ignored seed
  folder `target/fuzz-seeds/`, which `tools/fuzz_run.sh` passes to every target" (review 06b 5 (b)).

## ADR-81 Release 0.1.0a3 recorded; its lessons; a3 closed - decided (session 21)
- Context: the owner released 0.1.0a3 on 2026-10-24 from `ff3d6b25c360f93eeb8c1305927dd6cf3550610b` (T-18c), the
  candidate Reviewer 6a-closure named without a blocking finding. `tools/release_build.sh ff3d6b25... --twice`:
  `check_dist: ok`, wheel and sdist identical in both builds. Wheel sha256 `15db873f...2aa16`, sdist sha256
  `36bc571d...293e5` (full values in the plan's a3 record). The sdist hash equals the closure review's sdist hash (other
  build root); the wheel hash differs (the SBOM holds the build root, ADR-70 item 5). Step 7 ran **once**: a Rocky
  Linux 8 container, python3.11, pyarrow 26.0.0, pandas present, **as root**: 221 passed, 6 skipped = 227. TestPyPI and
  PyPI uploads with project-scoped tokens typed with `read -rs`; index hashes equal; a plain `pip install oradmp` in a
  fresh venv installs 0.1.0a3. The tag `v0.1.0a3` (annotated) is on `ff3d6b2` and on `github` (checked with `git
  ls-remote`); `origin` could not be checked here. Real RHEL 8 host: not done.
- Decision:
  1. **a3 is done**; the record is the plan's "Release 0.1.0a3 record (session 21)". Closed by it: T-18c (reviewed:
     6a-closure), Reviewer 6a-closure, ADR-77 items 2, 5, 6, risk 22 for a3 (it stays the standing check for every
     upload). **a4 is the next release** (ADR-84).
  2. **Deviations, accepted for a3 only**: step 7 ran in one venv (3.11, pyarrow 26.0.0, pandas) instead of three
     (3.11 with pyarrow 15.0.2 and pandas 2.2.3; the newest CPython; pyarrow without pandas), and the sdist source build
     and the pyarrow-only export are not in the owner's facts. They stand on the closure review's run of the same
     commit (three venvs, both layouts, 0 failures; the CLI and the sdist build checked). The wheels of the two builds
     differ only in the build-root path of the SBOM (ADR-70 item 5), so the review's runs test the same code. Not a
     precedent: a4's step 7 runs all three venvs on the owner's own wheel. Step 4 (network `cargo deny`), `pip install
     --pre` and the pip version are not in the facts: "not recorded" (the closure review's `cargo deny check` was ok).
  3. **Lessons, for every later release** (the plan's checklist, session 21 block):
     (a) the tag names the full hash: `git tag -a v<v> -m "oradmp <v>" <full hash>`, then `git rev-parse
     'v<v>^{commit}'` equals the build log's `commit:` line; never a tag on an implicit `HEAD`;
     (b) tokens: `read -rs UV_PUBLISH_TOKEN; export UV_PUBLISH_TOKEN`, `unset UV_PUBLISH_TOKEN` right after each upload
     (worked as written; kept);
     (c) the two files named one by one, never a glob (kept);
     (d) the build line runs with `set -o pipefail` (or `echo "${PIPESTATUS[0]}"` right after it must print `0`):
     with `| tee` the shell's status is otherwise tee's (review 06a-closure, checklist step 6);
     (e) step 7 as **root** in a container: the expected skips are exactly the three probe tests of
     `tests/test_internal.py`, `tests/test_cli.py:576` (`test_exit_3_a_read_only_output_folder`, "root can write to a
     read-only folder"), `tests/test_cli.py:897` (`test_exit_3_an_unreadable_dump_names_the_reason_and_the_path`, "root
     can read a file of mode 000") and `tests/test_project_name.py:34` ("not a git clone": no `git` in the image), i.e.
     **6**; plus `tests/test_consumers.py` in a venv without pandas (**7**). If the image has `git`, run `git config
     --global --add safe.directory <mount point of R/src>` before pytest, so that `test_project_name.py` runs (then 5 /
     6 skips); without it that test **fails** on git's "dubious ownership" (SF-2), a checklist error, not a product
     fault. As a normal user on the VM: the three probe tests (git clone), plus `test_project_name.py` in copied
     folders, plus `test_consumers.py` without pandas;
     (f) a plain `pip install oradmp` installed the pre-release while only pre-releases exist on PyPI. This agrees with
     PEP 440 ("Handling of pre-releases": pre-releases are accepted when the only available versions that satisfy the
     specifier are pre-releases). It ends with the first stable release; the README keeps `oradmp==<v>`. The pip
     version is recorded next time;
     (g) the sdist hash does not depend on the build root: the owner's sdist hash is compared with the closure
     review's (a mismatch stops before TestPyPI); the wheel hash is compared only with the owner's own `SHA256SUMS`.
- Consequences: nothing to undo; the a3 files stay on PyPI. The checklist grows by (d), (e), (g).
- Rejected: re-running step 7 for a3 now in the missing venvs (the files are published; a failure could only lead to
  a4, which runs the full step anyway).

## ADR-82 T-18d: the 6a-closure findings and file names that are not UTF-8 - decided (session 21)
- Context: review 06a-closure. **SF-1**: `_cli.py:632` recognises an API error by `e.strerror` starting with `[E-IO`;
  `make_error` (`_errors.py:157-159`) builds `OSError(message)` when there is no errno, whose `strerror` is `None`, so
  the line would read `[E-IO] [E-IO] I/O error` without the file; the unit test uses `OSError(None, ...)`, a shape the
  API never builds. Reachable only by a NUL byte in the path (not from argv). **N-5**: a dump file name that is not
  valid UTF-8 ends with a Python traceback (`UnicodeEncodeError ... surrogates not allowed`) in the tool and in
  `oradmp.open`, because the binding takes `path: String` (`crates/oradmp-py/src/lib.rs:653`). On POSIX Python decodes
  such argv bytes and file names with the `surrogateescape` handler (Python documentation, "file system encoding and
  error handler", `os.fsdecode`); a lone surrogate cannot be encoded to UTF-8, so the string conversion fails. The core
  already takes any path (`crates/oradmp-core/src/dump.rs:377`, `impl AsRef<Path>`); `_api.open` passes
  `os.fspath(path)`. `_survey.py:67` prints `os.path.basename(path)` on stdout, which would fail the same way after the
  binding is fixed. Such names are plausible on the owner's hosts (CP1251 names copied from Windows). Notes N-2 (the
  `-n -1` guard mutation aborts pytest instead of failing a named test), N-6 (`export --all` prints the manifest header
  before an exit-3 error), N-7 (README line 173, 128 characters), N-3, N-4, N-8 (as specified, nothing to do).
- Decision:
  1. **T-18d now, before T-17 and T-16** (not a later a4 task): it is small, the binding change is one parameter, and
     T-16's first CI run should test the binding a4 ships. Model Sonnet.
  2. **The binding takes a path**: `open(path: PathBuf, ...)` in `crates/oradmp-py/src/lib.rs`, through pyo3's
     `FromPyObject` for `std::path::PathBuf` (pyo3's conversion table: `str` and `os.PathLike` to `PathBuf`). The
     task's tests prove that a name made by `os.fsdecode` of non-UTF-8 bytes reaches the file with its original
     bytes; if pyo3 0.30 does not do that, the task stops and reports (no workaround chosen in the task). The Python
     signature and type hint stay (`str | os.PathLike[str]`); `bytes` paths are not promised.
  3. **The path text in messages** (ADR-77 item 4 amended): the tool writes FILE with each byte that is not valid
     UTF-8 as `\xHH` (two lower-case hex digits; `os.fsencode(path).decode("utf-8", "backslashreplace")`: Python's
     codecs documentation, `backslashreplace` on decoding gives `\xhh`), then tab, CR, newline as `\t`, `\r`, `\n`.
     The same rule for `survey`'s `file:` line. Neither stdout nor stderr ever receives a lone surrogate.
  4. **SF-1**: `_error_text` takes an error of the API by its attribute `code == "E-IO"` (set by `make_error` on every
     API error), and the API text from `e.strerror` if it is a string, else from the single string argument; the test
     builds its errors with `oradmp._errors.make_error` (errno `None` and `ENOENT`).
  5. **N-2**: the `-n -1` case also runs as a subprocess, so the guard mutation fails a named test (exit 134 against
     2); the in-process case may stay. **N-7**: the README line rewrapped. **N-6**: no change: the exit code is the
     signal, and the manifest header carries no data; arch 3.5 says so.
  6. **Not in T-18d or a4**: `.filename` and the OS text on the API's `OSError` (ADR-77 item 4's candidate). It
     changes the exception the ETL code catches; its own record when the owner asks for it.
  7. **Review**: T-18d, T-17 and T-16 are one group, reviewed in **Reviewer 6b** (rule 7 of `CLAUDE.md`); no closure
     review of T-18d alone, because nothing is released before 6b and T-17 touches neither the binding nor the tool.
     Exception: if the owner wants a fix release for the non-UTF-8 crash before T-16 and T-17, a short closure review
     of T-18d alone runs first (recommendation: no; the crash is a traceback, not wrong data, and renaming the file is
     a workaround).
- Consequences: T-18d changes `crates/oradmp-py/src/lib.rs`, so the full Rust checks run; step 3b is not due (no file
  of `crates/oradmp-core/src/`). The checklist strings (SF-2, the `tee` status) are the Architect's and are fixed in
  this session (ADR-81 item 3).
- Rejected: an a4 task after T-17 (CI would first test the old binding; the same review either way); encoding to
  `bytes` in `_api.py` (the binding takes `String`: same failure); leaving Python's default `\udcff` form on stderr (it
  shows a surrogate, not the file's byte, and stdout would still fail); escaping every control character in FILE (no
  case needs it; ADR-77 item 4 stays narrow).
- Amendment (session 22, ADR-86 SF-4): item 3 covers **every path the tool writes into a message**, not only FILE: the
  output target of `export -o` (the "exists" refusal) and the `filename` of an output-side `OSError` too. The README
  sentence of T-18d then holds as written. Done in T-17b.

## ADR-83 Order after a3: T-18d, T-17, T-16; what CI runs and what it does not - decided (session 21; O-22 and the visibility need the owner)
- Context: O-21 is done (`9f1dc4d`); O-22 (Actions enabled, pushes to `github`, read access for the Implementer) is
  not. T-17 needs nothing from the owner. The repository holds `corpus/fixtures` (556 tracked files, 16 MB;
  `.gitattributes` keeps dumps binary and the logs, truth files and `SHA256SUMS` unconverted, ADR-52). The memory and
  throughput tests (`crates/oradmp-core/tests/t15_memory.rs`, `t15_throughput.rs`) are `#[ignore]` and skip without
  `ORADMP_WORK`; their files (`corpus/work/`, `T_BIG` 5,000,000 rows) are not in git.
- Decision:
  1. **Order**: T-18d now; **T-17** after T-18d is committed; **T-16** when O-22 is done and T-17 is committed. The
     session 20 rule stands (T-16 first when both can start), and the VM runs one of them at a time. With O-22 open
     today the expected order is T-18d, T-17, T-16. Reason: T-17 needs nothing, and T-16's first CI runs then cover
     T-17's tests on every platform from the start (temporary folders, paths: T-16 risk check (a)).
  2. **CI uses the committed fixtures**: the checkout brings them (no LFS, no download); the first step of every job
     checks the three `SHA256SUMS` (with `sha256sum`, or a few lines of Python `hashlib` where the runner lacks it).
  3. **CI does not run the memory and throughput tests**, and no file of `corpus/work/` goes to GitHub (no artifact,
     cache or release asset): they need the owner's large exports and an idle machine, and their baselines are
     VM-only (benchmarks.md rules 1, 5). They stay `#[ignore]` (a CI run shows them as skipped, with the reason).
     `cargo bench --bench decode` in the manual job is a report, never compared with the VM baseline (ADR-66).
  4. **Private repository**: Linux jobs on every push; macOS and Windows only on `workflow_dispatch`, started by the
     owner when T-16 asks. The T-16 note records the billed minutes of each dispatched run (from `gh run view`), so
     the owner sees the cost of the 2,000 included minutes a month (ADR-79). If the minutes run out, the jobs wait;
     the visibility does not change because of it.
  5. Inside T-16: the Linux job green on a push first; then one dispatched run for the three risk checks (T-16 (b));
     then the rest of the matrix.
- Consequences: T-16 needs O-22 only; the VM stays the only place for memory, throughput and the release build.
- Rejected: uploading the `T_BIG` export as a CI artifact (owner's file, 5,000,000 rows, and the measurement needs a
  quiet, known machine); T-16 before T-17 while O-22 is open (the VM would wait).
- Amendment (session 22): items 1 and 5 are superseded by ADR-87 (T-17b, then T-16a on Linux, a4, then T-16b for the
  other platforms). Items 2 to 4 stand; item 4's "started by the owner" now also covers pasting the logs (ADR-79
  amendment).

## ADR-84 0.1.0a4: content, gate, steps - decided (session 21; the Project-URL depends on the visibility, O-19 (a))
- Context: a3 is out (ADR-81). ADR-76 and ADR-79 planned a4 with T-16's wheels and the Project-URL if public.
- Decision:
  1. **Content**: T-18d (SF-1; file names that are not UTF-8 in the API, the tool and `survey`; the `-n -1`
     subprocess case; README); T-17's tests (they change no reader code; if T-17 finds a disagreement, the reader fix
     is its own task and record, step 3b becomes due if `crates/oradmp-core/src/` changes, and it goes into a4 only
     after 6b read it); T-16's accepted platform wheels (ADR-53 item 4; none accepted means Linux x86_64 only, as a3;
     an accepted `manylinux2014` wheel replaces `manylinux_2_28`, T-16 (g)); `[project.urls]` with its `check_dist.py`
     rule if the repository is public before T-16 ends (T-16 (j)); version `0.1.0a4`.
  2. **Gate**: the version commit `0.1.0-alpha.4` (Implementer, small: root `Cargo.toml`, both `Cargo.lock` entries,
     the `oradmp-core` entry of `fuzz/Cargo.lock`, the README's version lines; `tests/test_version.py` green) is the
     last commit outside `docs/` before **Reviewer 6b**. 6b reads T-18d, T-17, T-16 and the candidate (build twice,
     checklist step 3 in a clean clone, the installed wheel in three venvs and both layouts, the CLI, the non-UTF-8
     case). A closure review follows if 6b's fixes change a shipped file. The a4 checklist is written by the Architect
     after 6b: a3's checklist with ADR-81 item 3, plus, if T-16 adds platform wheels, where each comes from (the CI
     run on the tagged commit, its log's SHA-256 compared with the downloaded file before any upload).
  3. **Not in a4**: the API `OSError.filename` (ADR-82 item 6); conventional LONG values of 32 KB or more (R-040 has no
     fixture yet); Data Pump.
- Consequences: a4 has no date; it waits for 6b. The RHEL 8 host test stays outside the gate, as since a1.
- Rejected: a fix release a4 now for the non-UTF-8 crash alone (ADR-82 item 7); a4 waiting for the visibility
  decision (the Project-URL can come in a5).
- Amendment (session 22): items 1 and 2 are superseded by ADR-88 (T-17b added; T-16b's wheels move to a5; the
  review before a4 is **Reviewer 6c**, because 6b ran before T-16). Item 3 stands.

## ADR-85 Reviewer 6b recorded; what it closes - decided (session 22)
- Context: Reviewer 6b (`docs/review/review-06b-t18d-t17.md`) read T-18d (`ba8a89c`) and T-17 (`edc62a5`) on top of the
  released a3 (`ff3d6b2`). No blocking finding. A synthetic T_FRAME file is byte-identical with `t_frame_exp_{18c,11g}`
  and `t_frame_exp_direct_{18c,11g}` from R + 0x103 to the end, apart from the header name and date (H4a) and the H6
  blocks; 20,000 NUMBER values agree with the Analyst's `num_decode.py` and 20,000 DATE / TIMESTAMP values with Python's
  calendar; every reader mutation that changed behaviour failed a named T-17 test; every T-18d mutation failed a named
  test. Clean clone: `cargo nextest run -j 2` 429 passed and 4 skipped, `pytest -rs` 233 passed and 0 skipped, fmt,
  clippy (three forms), `cargo deny check`, ruff, ruff format, mypy ok; `PROPTEST_CASES=20000` on `t17_synth` 17 passed,
  1 skipped, no regression file; `tools/fuzz_run.sh read 60` with 435 seeds: findings 0. T-16 had not run, so 6b did not
  read the a4 candidate that ADR-84 item 2 gave it.
- Decision:
  1. **T-18d DONE and reviewed**; **T-17 DONE and reviewed**. The synthetic files are accepted as test inputs for
     framing and value decoding; they stay common-mode coverage, not format evidence (ADR-80 item 2).
  2. **Closed**: ADR-82 items 2, 4, 5 (the binding takes `PathBuf` and passes the bytes; SF-1 of 6a-closure; the `-n -1`
     subprocess case), each shown by a named failing mutation in 6b section 4; ADR-80 item 3 (no file of
     `crates/oradmp-core/src/` changed); ADR-80 item 2 except SF-3 (ADR-86). **Review 06a N-5** (the untracked
     `t14_damage.proptest-regressions`): answered from 6b 3 (a) / 5 (a) and the T-17 note (j): `t14_damage`'s own
     property tests write it under a mutation that moves an error offset beyond the cut (`ByteStream::truncated` at
     `offset + 100000`: three tests fail and the file appears); the fix is SF-5. **Do the fuzz targets read the
     synthetic seeds?** Yes, from 6b 5 (b): `tools/fuzz_run.sh` passes `../target/fuzz-seeds` to every target, the run
     printed `INFO: 435 files found in ../target/fuzz-seeds`, and with `-runs=0` the seeds give INITED cov 4209 / ft
     12792 (fixtures only), 3301 / 13501 (synthetic only), 4368 / 16277 (both): about 160 more edges and 3,500 more
     features than the fixtures alone.
  3. **The two T-17 deviations are accepted**: the seeds go to `target/fuzz-seeds/` (the plan's wording is corrected,
     ADR-80 amendment); `tools/fuzz_seeds.sh` needs a working `cargo` (it runs the generator as an ignored test); N-8
     pins which toolchain (ADR-86).
  4. Accepted as not checked, not asked again before a4: the third T-17 reader mutation (two bytes at each multiple of
     R; O4 and the boundary test cover part of it); a code-coverage run; the 600-second runs of `walk` and `values`
     (the T-16a manual job runs 60 seconds each). Off-Linux behaviour is T-16b's.
  5. Inside the report two numbers disagree with its own tables: the verdict says "R + 0x104", section 3 item 1 says
     R + 0x103 (section 3 is right: the LF at R + 0x103 is identical too); "All 11 hand mutations" and "Six of my
     mutations touch rules outside the Implementer's four" against the table, which lists 13 reader mutations that
     changed behaviour (K1, K2, K4, O1b, O1c, O2 to O9) and 9 outside the four (the table is right). No finding
     depends on either.
- Consequences: SF-1 to SF-5 and N-1 to N-8 go to T-17b (ADR-86). The review before a4 is a new session, Reviewer 6c
  (ADR-88).
- Rejected: a closure review of T-17 alone now (no blocking finding; T-17b carries a named failing mutation or check
  per item, and 6c repeats them).

## ADR-86 Review 6b's SF-1 to SF-5 and N-1 to N-8; task T-17b - decided (session 22)
- Context: ADR-85. ADR-80 item 1 says the generator emits a construct as valid input only when its rule is confirmed;
  the plan's T-17 (d) asks for "[E1, E2, E3]", and E3 is "suggested (follows from E1, E2)" (`exp_reader_rules.md`
  line 263). The rules file names as covered "databases AL32UTF8 (both versions) and CL8MSWIN1251 (11g only)" and as
  not covered "a single-byte database on 18c". [Y5] and [T5] give the TIMESTAMP value bytes and code, not the
  descriptor length per precision; the fixtures show 7 for (0) and 11 for (6) and (9) only (6b SF-1). [C4]: header
  bytes 1-2 (client set) never decode row data (confirmed). R-017 (open in `requests.md`) asks exactly for an empty
  18c table with a segment on the direct path, the only known route to an 18c file with E1 after flag 1.
- Decision (one line each; the checks are T-17b's "Done when"):
  1. **SF-1, the rule**: ADR-80 item 1 amended (its amendment line): fixture-shown combinations, plus `Unconfirmed`
     combinations under a named rule, nothing else. Plan T-17 (d) wins for E1 after flag 1 in an 18c file, as an
     `Unconfirmed` construct.
     - **E1 after path flag 1 in an 18c file: stays**, as `Unconfirmed::E1AfterFlag1In18c` (generator-only). The bytes
       are confirmed (every 11g direct-path empty table); the design decided not to choose E1 / E2 by the version line
       ([E3], arch 5.4), and this case is what tests that decision. Evidence route: **R-017** (existing; no new number).
     - **A client set no fixture pairs with that version, mode and database** (client 171 with database 873; owner
       mode with client 1 and database 873): **stays**, as `Unconfirmed::ClientSetNotInFixtures`. [C4] (confirmed)
       makes the client set irrelevant to row data and the generator's identifiers are ASCII, so only the header fields
       change. No request: no reader path depends on it.
     - **An 18c header with database set 171: removed** (database 171 only with an 11g header). The rules file names a
       single-byte 18c database as not covered, and the 171 LOB forms rest on 11g only (ADR-25 session 8). No request:
       it needs a new 18c database; the owner's dumps are 11.2 (risk 1).
     - **TIMESTAMP(f) for f in 1 to 5, 7, 8: removed** (f only from {0, 6, 9}). The descriptor length the generator
       wrote for them copied the reader's agreement check (6b mutation G2), i.e. reader code. New request **R-047**
       (open-questions (b)): TIMESTAMP(1) to (8) on both versions and paths, with the expected bytes. When R-047 is
       generated and the Analyst confirms the lengths, a later task may widen the generator.
     - **Labelling in the code**: an enum `Unconfirmed` in `synth.rs` with exactly the two variants above; every place
       that writes such a construct carries the comment `// UNCONFIRMED(<Variant>): <rule ids>; accepted by <rule>;
       evidence: <R-n or none>`; the `Layout` of each file carries the set of variants it contains; every failure
       message of `t17_synth.rs` prints that set; a new test reads the header of every EXP fixture with its own byte
       reading ([H1] to [H4], no `oradmp_core`) and fails when a header combination the generator draws without the
       `ClientSetNotInFixtures` label is absent from the fixtures, or one with the label is present. A construct the
       fixtures show is never labelled, and one they do not show is never unlabelled.
     - **The T-17 note's wording**: corrected by a dated "Corrections (T-17b)" block under the note (the original text
       stays): E3 suggested; the 18c E1 case and the client pairs `Unconfirmed`; the TIMESTAMP length source is [Y5] with
       `t_ts` and `t_decl` for (0), (6), (9), not `types/map.rs`; H4's 10 bytes constant in 201 of 201 files (6b),
       meaning unknown; R + 0x000 to R + 0x102 are zeros (not "to 0x103").
  2. **SF-2**: the guard tests `v.is_empty()`; a test over many seeds asserts that every E2 table is followed by an
     item of the E2 (a) list, written as literals from the rules file.
  3. **SF-3**: the three expected types of `number_types_override_the_arrow_type` are literals (`DataType::Int64`,
     `DataType::Utf8`, `DataType::Decimal128(20, 4)`); `t17_synth.rs` calls no reader function for an expected value.
  4. **SF-4**: **escape the paths**, not narrow the README: every path the tool writes into a message goes through
     `_path_text` (the `export -o` target in the "exists" refusal, the `filename` of an output-side `OSError`, and any
     other path the Implementer finds by grep). Reason: CP1251 output names follow CP1251 input names on the owner's
     hosts, one form in all messages is what ADR-82 item 3 meant, and the change is two calls. Rejected: narrowing the
     README to "a dump file name" (leaves two forms, `\xHH` and `\udcHH`, in one tool).
  5. **SF-5**: `failure_persistence: None` in `t14_damage.rs`, `t4_values.rs`, `t5_types.rs` (proptest's
     `ProptestConfig`, other fields from `ProptestConfig::default()`, so `PROPTEST_CASES` still applies); the
     `t14_damage.rs` doc line "proptest's regression file is committed" goes. Rejected: a `.gitignore` entry (the
     owner's file; it hides the file but keeps writing it).
  6. **N-1** (structure that differs from the fixtures; the reader does not depend on it): no generator change; the note
     lists (a) to (e); the zero bytes at R + 0x000 to R + 0x102 get the comment `// CONTRADICTS FIXTURES (harmless:
     [H6] skip)`. The fuzz seed set already holds the 235 real fixtures with the real blocks.
  7. **N-2**: one deterministic case with the fixture's exact chain shape (`9c 80` + 156 bytes, three times `19 9f` +
     7,961, `19 1f` + 7,961 = 32,000 bytes) on the direct path; the random shapes stay (R2: "never assume").
  8. **N-3**: no code change; a note claims "a property test catches X" only with the case count and "in this run";
     mutations are carried by deterministic tests.
  9. **N-4**: one distribution: the proptest strategy becomes `any::<u64>()` mapped through `params_from_seed`, so the
     coverage test measures what the property test draws; failure messages print the seed (shrinking a seed means
     nothing; the message is the replay).
  10. **N-5**: the note's "not generated" list completed with 6b's N-5 items, each with the fixture test that covers it
      (`t1_walk.rs`, `t5_types.rs`, `t5f_followup.rs`, `t11_*`, as 6b found by grep).
  11. **N-6**: corrected in the "Corrections (T-17b)" block: three own mistakes, not four; K1 fails 12 of 17 tests in a
      `--no-fail-fast` run (6b), not "at least 8"; K2's firm failures are the two deterministic tests.
  12. **N-7**: the three lines T-17 added above 120 characters are wrapped: `CONTRIBUTING.md:56` (127),
      `tools/fuzz_seeds.sh:4` (207) and `tools/fuzz_seeds.sh:19` (123; 6b names only the first two).
  13. **N-8**: `tools/fuzz_seeds.sh` changes to the repository root before `cargo test` (rustup then takes the root
      `rust-toolchain.toml` from any caller directory) and prints `cargo --version` once; CONTRIBUTING says in one
      sentence that the seed step compiles the `oradmp-core` test binary with the root toolchain.
  14. **Model and review**: T-17b is one Sonnet session; no closure review of its own: T-17b, T-16a and the a4
      version commit are one group (rule 7 of `CLAUDE.md`), reviewed by **Reviewer 6c** before a4 (ADR-88). Nothing
      is released between T-17b and 6c, and each T-17b item has a named failing mutation or check that 6c repeats.
- Consequences: T-17b changes test files, `tools/fuzz_seeds.sh`, `CONTRIBUTING.md` and `python/oradmp/_cli.py` (a
  shipped file: 6c reads it with the candidate); no file of `crates/oradmp-core/src/` (step 3b not due). Arch 3.5 and
  8.3 get one line each.
- Rejected: removing the 18c E1 case (it is the only test of the decided "never by version" rule); removing the
  client pairs (no reader path depends on the client set, so removal buys no safety); keeping TIMESTAMP(1-5, 7, 8)
  as `Unconfirmed` (its bytes are not confirmed for any combination, and its length came from reader code); labels only
  in the note (a later reader of the code cannot see them).
- Amendment (session 24, ADR-94 items 1, 2): R-047 confirmed [T5] v3.7, so "TIMESTAMP(f) for f in 1 to 5, 7, 8: removed" is
  lifted (T-19 draws f from 0 to 9, length from [T5] v3.7 as a literal); R-017 showed E1 after flag 1 in an 18c table-mode
  file, so `E1AfterFlag1In18c` is narrowed to owner mode (`E1AfterFlag1In18cOwnerMode`).

## ADR-87 T-16 in two parts: T-16a (Linux CI) now, T-16b (other platforms) after a4; the logs route; T-17b first - decided (session 22; the one-time cut needs the owner, Q-14)
- Context: O-21 done (`9f1dc4d`). O-22: the owner chose to **paste the CI logs** (session 22); `CLAUDE.md` rule 8 forbids
  any agent to use a GitHub token, so the Implementer never reads GitHub itself. The repository `granddad-wolf/oradmp`
  is private: Linux minutes are cheap, a macOS minute costs about ten times as much (ADR-79). The owner's hosts are RHEL
  8+ x86_64 only (Q-9), so the platform matrix does not serve the owner's own use. The owner leans to a one-time cut to
  a public repository after T-16's first green Linux run (Q-14), which would make the macOS / Windows minutes free.
  T-17 facts: 17 new tests that write only below `target/t17-synth/` and need no `corpus/` file, `git` or network;
  `tests/test_paths.py` skips off Linux ("POSIX byte file names"); `tools/fuzz_seeds.sh` needs `cargo`.
- Decision:
  1. **T-17b before T-16a.** T-17b changes the tests, `_cli.py` and the seed script that T-16a's jobs run; doing it
     first means T-16a's counts and skip lists are final on the first green run, and SF-5 is fixed before any runner
     executes the property tests. T-17b is one session; the VM runs one build at a time.
  2. **T-16a (Linux, private repository)**: one workflow, Linux jobs on every push to `master` and on
     `workflow_dispatch`; a manual "extended" job (fuzz, long property runs, benchmark report) on `workflow_dispatch`
     only while private (no schedule: a nightly job would bill minutes every day). Done when one run on the final
     T-16a commit is green and the extended job ran once on it with findings 0 ("Done when" in the plan).
  3. **T-16b (macOS arm64 and x86_64, Windows x86_64; Linux aarch64 optional)** after the a4 release: the risk check
     of line endings, paths and file locks, the `target_os` gates, the acceptance rule of ADR-53 item 4, the README's
     platform list. While private: `workflow_dispatch` only, started by the owner, at most three dispatched runs without
     asking the owner again, billed minutes in the note; if public by then: every push. Accepted wheels ship in a5.
  4. **The logs route**: the Implementer commits; the owner pushes to `github` and pastes, for each run asked for, the
     run's web address, the **job summary** of every job and, for a failed job, the failed step's log. So every job
     ends with a summary step that writes to GitHub's job summary (`GITHUB_STEP_SUMMARY`, GitHub Actions documentation,
     "Adding a job summary"): commit, runner image, toolchain and Python versions, nextest passed / skipped with the
     skipped names, pytest passed / skipped with each skip reason, the `check_dist` line, the SHA-256 of each built
     file, the job's duration. A request in the T-16a note names the commit and what to paste. The Implementer never
     pushes; no token anywhere.
  5. **Portable workflow**: nothing in `.github/` names the repository, the account or a host (GitHub's own context
     values instead), so the same files run unchanged in a repository made by the cut (Q-14).
  6. **What CI never does** (ADR-83 items 2 and 3 stand): no upload, no secret, no `corpus/work/` file, no memory or
     throughput test (they stay `#[ignore]` and show as skipped), no benchmark gate.
- Consequences: a4 waits for T-16a, not for T-16b (ADR-88). The cut, if the owner decides it, fits after T-16a's first
  green run and before the a4 version commit, or after the a4 tag (Q-14).
- Rejected: T-16 before T-17b (its first green run would test tests about to change, and the counts would be redone);
  one T-16 that holds a4 until macOS and Windows are accepted (the owner's hosts are Linux; billed minutes or the cut's
  timing would hold the non-UTF-8 fix); a scheduled nightly job on the private repository (daily minutes); a `gh`
  token on the VM (rule 8).

## ADR-88 0.1.0a4 restated: content, order, Reviewer 6c - decided (session 22; the Project-URL depends on the cut, Q-14)
- Context: ADR-84 planned a4 with T-18d, T-17, T-16 and Reviewer 6b on the candidate. 6b ran before T-16 (ADR-85);
  T-17b is new (ADR-86); T-16 is split (ADR-87).
- Decision:
  1. **Content**: T-18d; T-17 and T-17b (tests; SF-4 changes `_cli.py`); T-16a (CI files only; no product change
     except what a CI failure proves, each such change named in its note); the Linux x86_64 `manylinux_2_28` wheel and
     the sdist built on the VM with `tools/release_build.sh`, as a3; version `0.1.0a4`. `[project.urls]` with its
     `check_dist.py` rule and case (T-16 (j)) only if the cut (Q-14) is done before the a4 version commit; then the
     version task adds them. Not in a4: T-16b's wheels (a5), the API `OSError.filename` (ADR-82 item 6), R-040, Data
     Pump.
  2. **Order**: T-17b, T-16a, (the owner's cut, optional, Q-14), the a4 version commit (Implementer, small: as ADR-84
     item 2), **Reviewer 6c**, an Architect session (records 6c, writes the a4 checklist), the release. Never a cut
     between the version commit and the pushed tag.
  3. **Reviewer 6c is the review before a4**, and it runs after T-16a and the version commit. It reads T-17b (every
     mutation and check of its "Done when"), T-16a (the workflow against ADR-79 item 4, ADR-83, ADR-87; the pasted
     summaries; whether the CI counts equal the VM's), and the candidate as ADR-84 item 2 lists it (build twice,
     checklist step 3 in a clean clone, the installed wheel in three venvs and both layouts, the CLI, the non-UTF-8
     case including SF-4's output paths). If the cut came first, 6c also checks the cut (Q-14 checks 6 to 9). A closure
     review follows if 6c's fixes change a shipped file.
  4. Step 3b (benchmarks) is due only if `crates/oradmp-core/src/` changed since `460aef5`; none of T-18d, T-17,
     T-17b, T-16a is planned to change it.
- Consequences: a4 depends on T-17b, T-16a, 6c and the owner; not on macOS / Windows, not on the cut. ADR-84 item 3
  stands.
- Rejected: a4 without a review after T-16a (rule 7; the CI files and T-17b's `_cli.py` change are unread); waiting for
  T-16b (ADR-87).

## ADR-89 T-16a and T-17b recorded; O-22 closed; what T-16a leaves open - decided (session 23; the billing figure needs the owner)
- Context (owner's facts, session 23; checked against `ci.yml` and `git log`): T-16a is `c1d1c58` plus two follow-ups,
  `7a2d028` (the GIL test without timing; `nproc` in the summaries) and `d081109` (per-member listing of wheel and sdist
  in the summary). Last full run at `d081109b708b08f08cc6c965994b6c094b649b65`: `checks (Linux)` nextest 432 passed, 4
  skipped, `cargo deny` ok, pytest 235 passed (about 3 min); `wheel (Linux)` wheel `8405b25b...`, sdist `ed4dbf0f...`,
  pytest 232/3, 225/4, 232/3, `check_dist` ok (about 4.3 min); `extended` (dispatched) 435 seeds, fuzz `read` 58,010,
  `walk` 173,513, `values` 8,100,212 runs, about 61 s each, 0 findings, `t14_damage` 10,000 cases 5 passed, `t17_synth`
  2,000 cases 20 passed 1 skipped, fuzz clippy ok, bench report (about 10.6 min); `manylinux2014` (dispatched) tag
  `cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64`, auditwheel consistent with `manylinux_2_17_x86_64`, newest
  glibc symbol `GLIBC_2.14` (about 3.5 min). The old `test_two_threads_run_in_parallel` failed once on the 2-vCPU
  runner (1.67 against 1.6) and was replaced in `7a2d028` by `test_counter_thread_progresses_during_read`, green in three
  re-runs. The three pytest figures are in the summary's order, which is the sorted log names (`py311-pa15-pd223`,
  `py311-pyarrow-only`, `pynewest-latest`), not the run order of `ci.yml`: 232/3, 225/4, 232/3 is what the VM had
  (T-16a note, VM table). The T-16a note was written before the first run and was not updated by the follow-ups (it
  still says "has NOT run on GitHub yet").
- Decision:
  1. **T-16a is done** (2026-10-24, `d081109`): "Done when" (a), (b), (d), (e), (f), (g), (h) are met; the counts equal
     the VM's at `4b4e732` (the follow-ups replace one test by one). Still open, carried by **T-16a2** (ADR-92 item 6):
     (c) the comparison of the wheel at the same build root (ADR-90 item 4; the sdist is equal: CI `ed4dbf0f...` = the
     VM's `ed4dbf0f8651a149956712be352f24b99c39339ea104e94be30471f284490c0d`); (i) the line for the one CI-proven change
     outside `.github/` and `CONTRIBUTING.md` (`tests/test_threads.py` in `7a2d028`: failing step, cause, fix); (j) the
     note's run record (runs, pasted summaries with full hashes, durations, the billed minutes).
  2. **The billed-minutes figure** is the owner's (O-23 (b)); until he gives it, the budget uses "2,000 a month" as his
     understanding (GitHub's billing page lists 2,000 included minutes for GitHub Free; fetched in session 23). It does
     not hold any task.
  3. **T-17b is done** (`4b4e732`; its counts are the ones CI reproduced); reviewed in Reviewer 6c with T-16a (ADR-88).
  4. **O-22 is closed**: Actions are enabled, the pushes happened, the summaries were pasted.
  5. **Risk checks (b) and (c) of ADR-53 item 7 pass as build checks**: `auditwheel show` (PyPA) reports
     `manylinux_2_17_x86_64` and the wheel is `cp311-abi3`. Nothing was installed on a glibc older than 2.28: that is
     T-16c (ADR-91).
- Consequences: T-16a2 before the cut and the a4 version commit (ADR-93). The next counts to compare are T-16a2's.
- Rejected: holding T-16a open until the billing figure arrives (it changes no file); counting the follow-ups as a new
  task (they are CI-proven fixes, which T-16a (i) allows).

## ADR-90 The canonical release root; the SBOM; which hash is authoritative; the reproducibility claim - decided (session 23; the confirming rebuild needs the owner or T-16a2, O-23 (a))
- Context: ADR-56 kept maturin's CycloneDX SBOM; `RUSTFLAGS="--remap-path-prefix=..."` changes what rustc writes, not
  the SBOM, which holds `path+file://` references to the workspace crates (review 03: 6 references). Owner's finding
  (session 23): the sdist and `_core.abi3.so` of `d081109` are byte-identical on the VM and on the CI runner; only
  `oradmp-0.1.0a3.dist-info/sboms/oradmp-py.cyclonedx.json` differs, and `RECORD` and the wheel hash follow it.
  **Checked in this session** (reading the VM's files only): the VM built `d081109` at `/var/tmp/oradmp-release-ci2`
  (wheel sha256 `01dd86601aaf81a9718c748fbf1762360b4ab623b2637413249baa175906b08f`); its SBOM member (134,581 bytes)
  holds exactly **6** `/var/tmp` paths, all below `/var/tmp/oradmp-release-ci2/src/crates/` (3 `oradmp-core`, 2
  `oradmp-py`, 1 `oradmp-py ... bin-target-0`). The CI job runs `tools/release_build.sh "$GITHUB_SHA"` without
  `ORADMP_BUILD_ROOT`, so its root is the default `/var/tmp/oradmp-release` (`ci.yml`, `wheel` job). 6 x 4 characters
  (`-ci2`) = 24 bytes, the owner's difference. **Not in the facts**: the owner's rebuild of `d081109` at the default
  root compared with CI's wheel. So the hypothesis "the build root in the SBOM is the only difference" is **consistent,
  not confirmed**. The default root is free on the VM today (`/var/tmp/oradmp-release` absent; `-a2`, `-a3`, `-ci`,
  `-ci2` exist).
- Options: (a) **a path-independent SBOM**. maturin 1.15.0's documented SBOM settings are `rust`, `auditwheel`,
  `include` (ADR-56); none that we checked rewrites or relativises the paths. What is left: drop it (`rust = false`;
  rejected in ADR-56: corporate scanners read it, PEP 770), rewrite it after the build (rejected in ADR-56 (b): a
  hand-made archive step), build at a fixed path through a container, chroot or bind mount (Docker is outside what
  agents run; the owner's step grows), or a newer maturin with such an option (unverified, and a pin change). High cost;
  the only gain is "any root". (b) **one fixed root for every release build, CI included**: no build change, two
  checks.
- Decision:
  1. **(b)** from a4 on. The **canonical release root** is `/var/tmp/oradmp-release`. A release build is
     `tools/release_build.sh <C> --twice --release` with `ORADMP_BUILD_ROOT` unset. The new option `--release`
     (T-16a2) refuses any root other than the canonical one after `realpath -m` and prints `release root: canonical`;
     the CI `wheel` job calls the script with `--release` too. Reviewer and test builds keep their own roots, without
     `--release`.
  2. **`check_dist.py --build-root <path>`** (T-16a2): every `/var/tmp/`, `/tmp/` or `path+file://` path in the wheel's
     SBOM member must begin with `<path>/src/`; else one FAIL line per hit with the byte offset. `release_build.sh`
     passes `--build-root "$ROOT"` on every build (release or not), so a remap or SBOM change shows at once. Test cases
     in `tests/test_check_dist.py`: an SBOM with another root fails; the right root passes; the option absent changes
     nothing.
  3. **Before every release** the canonical root must not exist; the owner moves the previous one aside (`mv
     /var/tmp/oradmp-release /var/tmp/oradmp-release-<what it holds>`; the script refuses an existing root). The a4
     checklist names the command.
  4. **The confirming check** (O-23 (a); the owner, or T-16a2's first step, whichever comes first; one build at a time):
     `tools/release_build.sh d081109b708b08f08cc6c965994b6c094b649b65` at the canonical root (no `--release` yet: it does
     not exist at that commit), then compare the wheel's **full** SHA-256 with the full value in the CI summary of
     `d081109` (`8405b25b...`); also the SBOM line of that summary's member listing should read 134,557 bytes (134,581 -
     24). Equal: the hypothesis is confirmed and recorded with both full hashes. Not equal: the member listings of both
     go to the Architect; nothing else changes. Then the root is moved aside (item 3).
  5. **The authoritative hash** of a release is the `SHA256SUMS` of the owner's VM build at checklist step 6: those files
     are uploaded, nothing is rebuilt. The CI run on the candidate is a cross-check: its **sdist** hash must equal the
     owner's (a gate, as ADR-81 item 3 (g)); its **wheel** hash must equal the owner's **only if item 4 confirmed the
     hypothesis** (then a difference stops the release until explained); otherwise the difference is recorded with the
     differing members.
  6. **The claim** (README, CONTRIBUTING, the checklist): until item 4 confirms, exactly "the sdist is byte-identical
     across machines (the build VM and the CI runner); the wheel is byte-identical when built twice on the same machine
     (`--twice`); a wheel built at another root differs in the SBOM member and `RECORD`". The README says nothing about
     reproducibility (it says nothing today). After item 4 confirms: "built from the same commit with
     `tools/release_build.sh --release` (the canonical root and the pinned toolchain, maturin and zig), the wheel and the
     sdist were byte-identical on the build VM and on GitHub's `ubuntu-24.04` runner; each release's CI run on the
     tagged commit shows the SHA-256 of the uploaded files". The README may then carry that sentence; never "on any
     machine".
- Consequences: the owner's release step changes by one flag and one `mv`; a Reviewer's wheel hash still differs from
  the owner's (other root), so 6c compares the sdist and the list of differing members, as before. The archive releases
  a2 and a3 (built at other roots) are not rebuilt.
- Rejected: option (a) in each of its forms above; making the CI wheel the release file (CI never uploads, ADR-87 item
  6; the runner is not the owner's machine); asserting the hypothesis from the byte count alone.
- Amendment (session 26, ADR-105 item 6): the evidence at S adds two CI runs with the same wheel and sdist at the canonical
  root and the owner's scratch-root build with the same sdist and another wheel; the claim of item 6 is unchanged.

## ADR-91 a4 ships `manylinux_2_28` only; `manylinux2014` becomes T-16c - decided (session 23)
- Context: ADR-53 item 5: an **accepted** `manylinux2014` wheel replaces `manylinux_2_28`; item 4: accepted means the
  full suites pass on that platform. T-16a's job built the wheel without the release script (no remaps, no `check_dist`)
  and installed it nowhere. The owner's hosts are RHEL 8+ (Q-9, glibc 2.28). PyPI's JSON API (session 23): pyarrow
  **26.0.0** has, for cp311 Linux x86_64, only `manylinux_2_28` and `musllinux_1_2` wheels; pyarrow **15.0.2** has a
  `manylinux_2_17_x86_64.manylinux2014_x86_64` wheel. So a glibc-2.17 host gets at best an older pyarrow from a wheel;
  which is the newest is not known here.
- Decision:
  1. **a4 ships the `manylinux_2_28` wheel and the sdist only**, Linux x86_64, as a3.
  2. **New task T-16c** (after the a4 tag; with or after T-16b; Sonnet): "Done when": (a) `tools/release_build.sh`
     takes `--manylinux 2014` (default `2_28`) and builds the 2014 wheel with the same remaps, `check_dist.py` and
     `--build-root` rule; (b) an install test in a **glibc 2.17** environment: a container image pinned by digest, run
     with `docker run` inside a CI step on the Ubuntu runner (JavaScript actions stay outside the container; no agent
     runs Docker on the VM); the note records `ldd --version`, which CPython 3.11+ interpreters the image has (none:
     the test cannot run, T-16c ends "rejected"), the newest pyarrow with a cp311 `manylinux2014` wheel (`pip download
     --only-binary :all: --platform manylinux2014_x86_64 --python-version 3.11 pyarrow` and the same for its numpy),
     full `pytest -rs` against the installed wheel with that pyarrow (0 failed; skips: the probe tests only); (c) the
     same wheel passes the three venvs on the runner and the Rocky Linux 8 container (owner); (d) the acceptance rule of
     ADR-53 item 4; if accepted, a5 ships the 2014 wheel **instead of** 2_28 (item 5), the README names glibc 2.17 and
     the newest pyarrow found; if pyarrow 15 or later cannot be installed there on Python 3.11, T-16c ends "rejected:
     no supported pyarrow" and 2_28 stays.
  3. The dispatched `manylinux2014` job stays a risk check until T-16c replaces it; it never publishes.
- Rejected: 2014 in a4 (untested, built without remaps); shipping both Linux wheels (two wheels for one platform double
  the install tests and the upload checks for no named user); dropping 2014 now (cheap to test once on CI; corporate
  hosts older than RHEL 8 exist).

## ADR-92 CI follow-ups: the fuzz crash artifact, benchmarks, what the owner pastes, the minutes budget; T-16a2 - decided (session 23)
- Context: T-16a (h) forbids uploads; ADR-62 item 7 allows a crash file as a workflow artifact the owner downloads; the
  T-16a note asks for a ruling. Bench numbers came back (report only). The owner pasted hashes shortened to 8 digits.
  GitHub's billing page (session 23): 2,000 included minutes a month for private repositories on GitHub Free; standard
  hosted runners free in public repositories; per-minute prices Linux 2-core $0.006, Windows $0.010, macOS $0.062 (about
  1.7 and 10 times Linux); without a payment method, usage is blocked at the quota.
- Decision:
  1. **Fuzz crash files: yes, as an artifact, on failure only.** One step at the end of `extended`, `if: failure()`,
     `actions/upload-artifact` pinned by full commit SHA (resolved in T-16a2 by `git ls-remote`, version in a comment),
     `path: fuzz/artifacts/` only (never `fuzz/corpus/`, never a wheel), `if-no-files-found: ignore`, `retention-days:
     14`. **Secrecy**: the files are libFuzzer mutations of seeds made by `tools/fuzz_seeds.sh` from the committed
     fixtures and the synthetic generator; CI has no `corpus/work/` file (ADR-83), so a crash file holds nothing that is
     not already in the repository. Private: readable by those with read access; after the cut by anyone with read
     access to the public repository; acceptable for generated input. T-16a (h)'s "no upload" is narrowed to: no
     publish to an index or a release, and no artifact except this one.
  2. **Benchmarks stay report-only** on CI: a shared runner fails ADR-66's conditions (own machine, five runs within
     +-10 %). `benchmarks.md` section 7 lists CI reports for orientation; never a baseline, never compared.
  3. **What the owner pastes after a run**: the run's web address and event (push / dispatch); the full commit hash; for
     every job its conclusion and its whole Summary block **verbatim** (hashes in full, 64 hex digits; no shortening);
     for a failed job the failed step's log from its first line (or its last 200 lines); the run's billed minutes if
     the usage page shows them, else the job durations. It goes into the prompt of the session that asked; that
     session's note copies the counts and hashes.
  4. **The minutes budget** is written in the plan ("CI minutes budget (session 23)"): the quota (the owner's figure,
     O-23 (b)), the cost of a push (about 7 to 8 minutes) and of a full dispatch (about 21), the dispatch rule while
     private, T-16b's cap. Not in `CONTRIBUTING.md` (a contributor's run is billed to their own account).
  5. `auditwheel` is installed unpinned in the `manylinux2014` job (`ci.yml`); T-16a2 pins it and prints its version.
  6. **Task T-16a2** (small, Sonnet, before the cut and the a4 version commit): ADR-89 item 1's open items, ADR-90 items
     1, 2, 4, items 1 and 5 here, `CONTRIBUTING.md` lines for `--release` and the artifact. Reviewed in 6c.
- Rejected: no artifact (a CI-only finding could not be reproduced without a rerun on the VM); uploading the whole fuzz
  folder (the corpus grows and is not needed); benchmark gates on CI; shortened hashes (they cannot prove equality).

## ADR-93 The cut before the a4 version commit; the a4 order; T-16b and visibility - decided (session 23; the cut itself and its settings need the owner, Q-14)
- Context: Q-14 stays as written; the owner leans to the cut after the first green Linux run (met) and before the a4
  version commit. ADR-88 item 2 put the cut as optional between T-16a and the version commit and forbade it between the
  version commit and the pushed tag. ADR-90 and ADR-92 add T-16a2. The public repository's name is not decided: the
  private `granddad-wolf/oradmp` holds the name today.
- Decision:
  1. **Order**: T-16a2 (and O-23 (a)) on the private repository, its CI run green; then **the cut** (the owner, steps in
     `open-questions.md`, "Q-14 steps (session 23)"); the first CI run on the public repository green **before any other
     commit, tag, release or setting there** (only a fix of that run may follow); Q-14 checks; a short Architect session
     records S and P and closes Q-14; then the **a4 version commit**, which then adds `[project.urls]` (`Source`, the
     public address) with its `check_dist.py` rule (METADATA has `Project-URL: Source, <url>` equal to
     `pyproject.toml`) and its mutation case; its CI run green; **Reviewer 6c**; an Architect session (records 6c, writes
     the a4 checklist); the release. If the owner does not cut first, the same order without the cut steps and without
     `[project.urls]`, and the cut comes after the a4 tag is pushed. Never between the version commit and the pushed tag
     (ADR-88 item 2 stands).
  2. **a4's release order**: gate (6c, closure if any); checks (6c's run on C); network `cargo deny`; version check; the
     old canonical root moved aside; `tools/release_build.sh C --twice --release`; hash cross-check with the CI summary
     of C (ADR-90 item 5); the Rocky Linux 8 install test (three venvs, as root); the RHEL 8 host if at hand; the tag
     made locally with the explicit hash; TestPyPI upload, index-hash comparison, install; PyPI upload, index-hash
     comparison, installs; the tag pushed last (to the public repository if cut, and to the Gogs remote); `dist/`
     checked; the record.
  3. **Reviewer 6c's CI checks** (added to its "Done when"): every `uses:` is a 40-hex SHA with a version comment, and
     the Reviewer resolves each tag to that SHA itself (`git ls-remote`, tags only, no action source); `permissions:
     contents: read` at the top and no job widens it; `persist-credentials: false` on every checkout; triggers only
     `push` (master) and `workflow_dispatch`; no `secrets.`, no token in any step; no publish step (no `uv publish`,
     `twine`, publish action, `gh release`) and no `upload-artifact` except ADR-92 item 1's; the `manylinux2014` job
     dispatch-only, its wheel kept under `$RUNNER_TEMP`, no artifact; every installed tool pinned except the named
     "newest" venv; no `--run-ignored`, no `corpus/work/`, no bench gate; the `wheel` job uses `--release`; nothing names
     the account, repository or host.
  4. **T-16b does not wait for the cut**, but with the cut before a4 (item 1) it runs on the public repository, where
     standard runners are free and every job may run on every push. If the repository is still private then, T-16b
     runs dispatch-only with ADR-87 item 3's cap (three dispatched runs) and the budget of the plan; macOS costs about
     ten Linux minutes per minute. Which macOS x86_64 runner image still exists is T-16b's first check (GitHub's
     runner-images list).
- Consequences: a4 waits for T-16a2 and, if the owner cuts first, for the cut and one green public run. ADR-88 item 1's
  `[project.urls]` condition stands.
- Rejected: the version commit before the cut (a4 would be reviewed and built from a commit that does not exist in the
  public repository, or the cut would fall inside the release); waiting for T-16b's minutes as a reason to cut (the cut
  is the owner's decision for its own reasons).
- Amendment (session 24, ADR-98 item 2): T-19 comes after T-16a2 and before the cut; the rest of item 1 stands.
- Amendment (session 26, ADR-105, ADR-107): the cut is done; item 1 continues at the a4 version commit (task T-20). Item
  2's "the tag pushed last (to the public repository if cut, and to the Gogs remote)" reads "the tag pushed last to
  `origin` (the public repository)"; the Gogs archive is frozen at P and gets no new tag.

## ADR-94 Analyst 17 recorded: R-017, R-040, R-047; rules v3.7; the ten new fixtures and the red corpus tests - decided (session 24)
- Context: the owner ran R-017 (18c), R-040 and R-047 (18c and 11g) on 2026-10-25 (`cca4c65`); Analyst 17 (`8162eb0`)
  analysed them and raised the rules to v3.7. New EXP files: 18c six (`tsp_exp(_direct)_18c`, `long3_exp(_direct)_18c`,
  `seg_exp(_direct)_18c`), 11g four (`tsp_*`, `long3_*`); new truth files five (`truth_tsp.txt` x 2, `truth_long3.txt`
  x 2, `18c/truth_seg.txt`; 87 truth files in the three `SHA256SUMS`, was 82). Corpus: **211** EXP files (95 / 89 / 27),
  404 sections, 418 binary parts (Analyst), 41,284 rows, 243,303 column fields, 19,960 NULL (`corpus_counts.py`, rerun
  in this session).
- Decision:
  1. **R-047 answered**: [T5] v3.7: TIMESTAMP descriptor length 7 for f = 0, 11 for f = 1 to 9; [Y5] v3.7: the fraction is
     what the database stored (rounded at INSERT). The reader's check (`types/map.rs`, `D::Timestamp`) already agrees: no
     reader change. ADR-86 item 1's "TIMESTAMP(f) for f in 1 to 5, 7, 8: removed" is lifted: T-19 lets the generator draw
     f from 0 to 9 again, the length taken from [T5] v3.7 and written as a literal table in `synth.rs` with the rule id
     (never from `types/map.rs`).
  2. **R-017 answered for 18c**: an empty table with a segment gets `ff ff` (E1) after the direct-path flag, one without
     gets none (E2), both in one file (`seg_exp_direct_18c`@0x226a, 0x23ca, 0x24c7). [E3]'s "never by the version line"
     now rests on a fixture. No reader change. Generator: `Unconfirmed::E1AfterFlag1In18c` no longer applies to the
     combination the fixture shows (18c, table mode, AL32UTF8, direct path): there the label goes; an 18c **owner-mode**
     file with E1 after flag 1 keeps a label, renamed `E1AfterFlag1In18cOwnerMode` (evidence: none; accepted by [E3]). 11g
     without a segment: R-049 (closes the record only; no reader path depends on it).
  3. **R-040 answered** for LONG on both paths: the conventional field is ADR-95. The direct path needs no change (a
     longer chain, 100,000 bytes = 12 x 7952 + 4576; 18c and 11g cut values differently; the reader uses only the flag).
  4. **Where the reader contradicts v3.7** (the Analyst's locations, read in this session): `exp/rows.rs:718` (stop on a
     conventional LONG length from 0x8000) and `exp/rows.rs:671-681` (`f0 ff` and `fd ff` are `Framing` in every column).
     Both are T-19 (ADR-97).
  5. **The tests over the corpus are red at `cca4c65` and later** (by reading; nothing was run in this session): every
     test that lists the fixture folders picks up the ten files: `t3_catalog.rs` (`exp_files`), `t10_truth.rs`
     (`the_exp_files_are_the_listed_ones` against `TOTALS.files = 201`; the truth listing asserts 82), `t14_damage.rs`
     (`0usize..201`, `assert_eq!(files.len(), 201)`), `t17_synth.rs` (`header_sets_match_the_fixtures` asserts 201), the
     generators (`assert files == 201` in `gen_number_parts.py`; the golden generators name 201), and the reader stops
     `long3_exp_<v>.dmp` with `E-UNSUPPORTED-LAYOUT` at 0xa280 (row 2), against the "no file stops" tests. A push of
     `cca4c65` or later turns CI's `checks` job red. So T-19 takes in all ten files and five truth files, not only
     `long3`, and nothing is cut or released before T-19 (ADR-98).
  6. **Statuses**: R-017, R-040, R-047 closed in `open-questions.md`; Q-4's "R-040 blocks the alpha if ..." becomes "R-048
     matters if a conventional LONG between 65,521 and 65,534 bytes or a LONG RAW above 32,767 bytes exists".
  7. **`docs/research/hypotheses.md`**: `CLAUDE.md`'s ownership table already gives the Analyst "the `Result` lines in
     `docs/research/hypotheses.md`"; the changes to the Result lines of H-013 and H-020 are inside that row. **No
     `CLAUDE.md` change is needed**; nothing for the owner.
- Consequences: the design documents cite rules v3.7 from this session; risk 4 rewritten, risk 26 updated, new risk 29;
  T-19.
- Rejected: a test-only task for the ten files before T-19 (the golden files would be regenerated twice and `long3_exp`
  recorded as "stops" for one task); keeping the 18c table-mode E1 label (ADR-86 item 1: a construct the fixtures show is
  never labelled).

## ADR-95 The conventional LONG and LONG RAW field (R-040); the Analyst's questions 1 and 2 - decided (session 24; replaces ADR-12 session 4 case (b))
- Context: [R2], [R8], [Y9] v3.7; `exp.md` 6.6 "Session 17". Conventional path (flag 0), LONG: a u16 from 1 to 0xfff0 is
  the length of one field, bit 0x8000 included (`ff 7f`, `00 80`, `ef ff`, `f0 ff` seen); `fd ff` starts a value of
  chunks `u16 n + n bytes` ended by a chunk `00 00` (65,535, 65,536, 100,000 bytes seen; 4000 each, the remainder last;
  100,000 = 25 x 4000, then `00 00`). Where the switch lies between 65,521 and 65,534 bytes is unknown (R-048); [R8]: 0 and
  0xfff1 to 0xfffc in such a field "stop". LONG RAW above 32,000 bytes: not in the fixtures on either path, "assumed the
  same" (status: assumed). Today the reader stops at any conventional LONG u16 from 0x8000 and treats 0xfff0 to 0xfffd as
  `Framing` (ADR-94 item 4).
- Decision:
  1. **The first u16 of a field of a type 8 / 24 column with path flag 0:**

     | u16 | LONG (8) | LONG RAW (24) | basis |
     |---|---|---|---|
     | `fe ff` | NULL | NULL | [R3] |
     | 1 to 0x7fff | one field | one field | [R2], [R8] (confirmed to 32,767 for LONG; for LONG RAW a length above 32,000 is "read like any length", [R8]) |
     | 0x8000 to 0xfff0 | one field; bit 0x8000 is part of the length | **stop**: `E-UNSUPPORTED-LAYOUT` (R-048) | [R2], [R8] v3.7 |
     | 0xfff1 to 0xfffc | **stop**: `E-UNSUPPORTED-LAYOUT` (R-048) | **stop** (as above) | [R8] v3.7 |
     | `fd ff` | chunked value (item 2) | **stop** (as above) | [R2] v3.7 |
     | 0 | `Framing` (unchanged) | `Framing` (unchanged) | [R8] |
     | `ff ff` | `Framing` "ff ff inside a row" (unchanged; at a row start it is [E1]) | same | [R8], [E1] |

     Every other column is unchanged: 0 and 0xfff0 to 0xfffd stay `Framing` there [R8]. The direct path is unchanged
     (ADR-15 item 1).
  2. **The chunked form (LONG only)**: after `fd ff`, repeat: read u16 c; c = 0 ends the value; 1 <= c <= 0xffef: c
     value bytes follow; c >= 0xfff0: `Framing` "conventional LONG chunk length from 0xfff0" at the chunk's u16 (as the
     LOB chunk rule of ADR-12). No chunk size is assumed or checked (4000 is observed only, [R2] "never assume"). A
     chunked value whose total is below 65,535 bytes (none in the fixtures; it covers R-048's range if `exp` writes it
     chunked, and `fd ff 00 00`, an empty value) is **decoded** with `W-UNTESTED-LAYOUT` "conventional LONG in the chunked
     form with fewer than 65,535 bytes", once per column and table part at the first such value's offset (as the
     zero-piece warning of ADR-15); `strict`: `E-STRICT`. The chunk framing is self-delimiting, so this case has one
     reading; the warning marks a combination no fixture shows.
  3. **Errors** (fatal; the file stops at that table, `Catalog.end = stopped`, the tables before it are listed and
     readable, ADR-12): code `E-UNSUPPORTED-LAYOUT` (`ErrorCode::UnsupportedLayout`), offset = the field's u16, with table,
     row, column and `length` = the u16. Messages, exact (no value bytes, ADR-58):
     - LONG, 0xfff1 to 0xfffc: `conventional LONG length 0xfff1 to 0xfffc, not in the fixtures (R-048)`;
     - LONG RAW, 0x8000 to 0xfffd: `conventional LONG RAW length from 0x8000 or chunked form, not in the fixtures (R-048)`;
     - the chunk case of item 2 is `E-FRAMING` with `conventional LONG chunk length from 0xfff0`.
     Python: `oradmp.UnsupportedError` (`ERROR_CLASSES` unchanged; `.code`, `.offset`, `.table`, `.row`, `.column` as for
     every error); the CLI prints it as every stop (`_report_stop` / `_report_error`, form unchanged) and exits **1**
     (`EXIT_CODES[UnsupportedError]`). The `UnsupportedError` docstring names the two R-048 cases as examples.
  4. **Ruling on question 1** (0xfff1 to 0xfffc in a conventional LONG column): **stop**, not warn. Two readings consume
     different byte counts (a plain length of 65,521 to 65,532, or some other form at the very boundary where `exp`
     changes form; 0xfffd is already a marker there), so any guess misframes every later field of the file and can
     misread values silently until a later `Framing`. The stop names the table, row, column and offset; R-048 decides.
  5. **Ruling on question 2** (does `fd ff` apply to LONG RAW before R-048): **no**. A conventional LONG RAW u16 from 0x8000
     (`f0 ff`, `fd ff` included) stops as item 3, as before R-040. No LONG RAW value above 32,000 bytes is in any fixture on
     either path; the shared framing up to 32,000 (session 9) does not show the new form; a wrong guess on binary data has
     no character check to expose it, only a later `Framing`. The stop costs one named table. R-048's LONG RAW half
     (python-oracledb; 18c alone is enough at first) is the cheaper half and goes first if Q-4 shows LONG RAW columns.
  6. **Delivery**: chunks go to the sink as chain pieces: `chain_piece(column, offset, bytes, first = (i == 0), last =
     false)` per data chunk, then one empty piece with `last = true` at the `00 00` (the sink takes empty pieces; this
     empty final piece is not a "zero piece" and raises no warning: that warning belongs to the direct-path chain). The
     LONG text decoder (ADR-15 item 3, `encoding_rs` streaming) carries a UTF-8 sequence across chunks. The value's offset
     is its first data byte (ADR-41 item 6). The Implementer may rename the sink method if the name misleads; behaviour
     as stated.
  7. **Bounds**: `max_lob_bytes` is checked as chunks accumulate (ADR-15 item 2): `E-LOB-TOO-LARGE` at the value's offset
     with `length` = the total at the chunk that passes the bound; under `on_value_error="null"` the remaining chunks are
     passed by length and the row goes on. The scan passes chunks by length and holds none.
  8. **Counts**: `LongStats` gains `chunked` (values in the `fd ff` form), in Rust and in Python (`oradmp.LongStats.chunked`,
     added as the last field); the survey's LONG line prints it after `chained`; `largest` and `above_32000` count chunked
     totals as any other; `chained` stays the direct-path count.
  9. **`W-LONG-LENGTH`**: LONG above **100,000** bytes (the largest confirmed, both paths and versions, R-040), message
     `LONG value above 100,000 bytes, the largest confirmed`; LONG RAW above 32,000 bytes, unchanged threshold, message
     `LONG RAW value above 32,000 bytes, the largest confirmed`. `LONG_LIMIT_BYTES` splits by type; the count
     `above_32000` keeps its name and meaning.
  10. **Raw digest** (`RawHasher`, `tests/golden/gen_exp_raw.py`): a chunked value is `fb ff ff ff`, per data chunk its
      length as u32 LE and its bytes, then `fa ff ff ff`; a plain field from 0x8000 is serialised as any field. The doc
      comment's "at most 65,519" becomes 65,520. The assembled digest (`long_sha256`: total u64 + bytes) is unchanged, so
      one value has the same assembled digest in all three forms.
- Consequences: the seven T_LONG3 values read on both paths and versions, without a warning even under `strict`; a real
  conventional LONG of any size reads unless its length field is 0xfff1 to 0xfffc; a conventional LONG RAW of 32,768
  bytes or more still stops the file; risk 4 lowered, risk 29 new (a value of 65,533 or 65,534 bytes written as one field
  would collide with `fd ff` / `fe ff`).
- Rejected: reading 0xfff1 to 0xfffc as a plain length with a warning (item 4); LONG RAW like LONG with
  `W-UNTESTED-LAYOUT` (item 5); checking that chunks are 4000 bytes ([R2]: never assume); a new error code for the R-048
  cases (`E-UNSUPPORTED-LAYOUT` already means "a layout the fixtures do not show; the file stops"; a new code adds a
  class row and an exit-code row for no caller's benefit); the chain markers for chunks in the raw digest (a golden line
  could not tell the two forms apart).
- Amendment (session 25, ADR-99, ADR-101, ADR-102): items 1, 3, 4, 5 and the LONG RAW half of item 9 are replaced by ADR-99
  (R-048 and R-051 answered them: three forms, LONG RAW = LONG). Item 2 stands for both types with the threshold 65,534
  (was 65,535) and the message of ADR-99 item 3; its "once per column and table part" reads "once per column and table
  section" (ADR-102 item 1). Items 6, 7, 8 stand and apply to LONG RAW too (a LONG RAW chunk goes to the binary sink, no
  decoder); item 8 gains `sizes_65521_to_65534` (ADR-101 item 4). Item 10 gains the `fa ff` form (ADR-99 item 5).

## ADR-96 TIMESTAMP WITH LOCAL TIME ZONE: the descriptor length per precision (question 3); R-050 - decided (session 24)
- Context: `types/map.rs` (`D::Timestamp`, `WithLocalTimeZone`) expects length 7 for f = 0 and 11 otherwise for type 231.
  The fixtures show one TSLTZ column, `TIMESTAMP(6) WITH LOCAL TIME ZONE` = `e7 00 0b 00` (`t_tstz_exp_<v>`@0x229a, 2
  versions); [Y8]: TIMESTAMP layout, no zone bytes; a zero-fraction value (7 bytes?) "not in the fixtures". A wrong length
  rule gives the column fallback of arch 5.5 step 4 (`W-DDL-FALLBACK`, values still decoded, the type from the
  descriptor), never a wrong value or a stop; every 231 column already carries `W-TSLTZ`.
- Decision: **write the request** (R-050 in `open-questions.md` (b): TSLTZ(0), (3), (9), zero and non-zero fractions, a
  rounding case, both versions and paths), **priority low**: it can wait unless Q-4 shows type 231 in the owner's tables;
  cheapest when run with R-048 in the same containers. No code change until it is answered; the rule stays a hypothesis
  drawn from [T5] v3.7 for type 180. The generator keeps TSLTZ out (ADR-80).
- Rejected: no request (the rule would stay unconfirmed with no route); a code change now (nothing to change it to).
- Amendment (session 25, ADR-100 item 1): R-050 is answered; the length rule of `types/map.rs` is confirmed for f = 0, 3, 6,
  9 and the 7-byte value is confirmed; the `W-UNVERIFIED-TYPE` for a 7-byte TSLTZ value goes (T-19b). `W-TSLTZ` stays.

## ADR-97 Task T-19: the conventional LONG forms and the session 17 fixtures; T-19 does not wait for R-048 - decided (session 24)
- Context: ADR-94, ADR-95. The owner's lean: do not wait for R-048; the 65,521 to 65,534 range stops with a named error and
  R-048 runs when convenient. Unchanged rules for tests: ADR-80 (amended), ADR-86, `CLAUDE.md` rule 7 (golden files by
  generators only).
- Decision:
  1. **T-19 now, without R-048.** A reader that decodes all seven known sizes and stops by name on the unknown ones is
     strictly better than one that stops at 32,768; R-048 can only widen what reads (ADR-95 items 1 and 5), never turn a
     read value into a wrong one, because every unknown case stops or warns today.
  2. **Scope** ("Done when" in the plan, T-19): the reader of ADR-95; the ten files and five truth files in every corpus
     test, with new totals and regenerated golden files; the generator widened (new LONG sizes and forms, TIMESTAMP f 0
     to 9, the E1 label narrowed); negative cases for the R-048 stops on patched copies; damage tests and fuzz seeds over
     211 files; the memory guard on the 100,000-byte value; named mutations; README, Python docstrings and the survey
     line; the architecture sections. Step 3b (benchmarks) is due (the core changes) and is part of T-19.
  3. **Model**: Sonnet. One build at a time; benchmarks and the memory test alone, no Oracle container running.
  4. **R-048, R-049 (and R-050)**: the owner runs them when convenient; **the owner's assistant writes the runners** from
     the text in `docs/formats/requests.md` (as for R-040); R-050 first goes into `requests.md` through Analyst session 18
     (open-questions "Inputs for Analyst session 18"). After the fixtures: an Analyst session, then an Architect session
     and a small task **T-19b** that moves the boundary of ADR-95 item 1 (and LONG RAW, item 5) to what R-048 shows, with
     the fixtures in the tests. T-19b is not in a4 unless it is done before the version commit.
     Amendment (session 25, ADR-103, ADR-104): R-048 to R-051 are answered; T-19b is defined by ADR-103 and comes **before**
     the cut and the a4 version commit, so it is in a4.
- Consequences: a4 carries T-19 (ADR-98); R-048 is not on the a4 path.
- Rejected: waiting for R-048 (a4 would wait on an Oracle run for a range no fixture or survey has shown to matter); T-19
  without the generator and damage work (rule 7 and ADR-80 ask that every new reader path is covered by both).

## ADR-98 The a4 order with T-19 first; Reviewer 6c's scope; the LONG sentence of the release notes; T-16a2 recorded - decided (session 24; the cut and its settings need the owner, Q-14)
- Context: ADR-93 item 1: T-16a2, the cut, the a4 version commit, Reviewer 6c, the Architect, the release. Owner's fact:
  T-16a2 done, CI and VM hashes identical at `af6a54a` (the T-16a2 note: the confirming rebuild of `d081109` gave equal
  wheel and sdist, ADR-90 item 4 confirmed). The owner's lean: T-19 first, then the cut, then the version commit; "the cut
  must not carry a known reader error". ADR-94 item 5: master is red on the corpus tests since `cca4c65`.
- Decision:
  1. **T-16a2 is done** (`af6a54a`). ADR-90 item 4's hypothesis is confirmed (the build root in the SBOM was the only
     difference); the claim of ADR-90 item 6 in its "after item 4 confirms" form applies, limited as CONTRIBUTING states
     it. Open from its note: the owner's `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-d081109` before any release
     build (ADR-90 item 3).
  2. **Order** (amends ADR-93 item 1): **T-19** committed, its push run green (CI green again) -> [OWNER] **the cut** (Q-14
     steps), its first public CI run green before anything else there -> short Architect session (close Q-14) -> **a4
     version commit** (+ `[project.urls]` if cut) and its CI run -> **Reviewer 6c** -> Architect (record 6c, a4 checklist)
     -> release. Reason: the cut's first public run must be green (ADR-93), which master is not before T-19; and a public
     snapshot should not hold a reader that stops a confirmed LONG form. Without the cut first: the same order without it;
     the cut after the a4 tag.
  3. **Reviewer 6c covers** T-17b, T-16a, T-16a2, **T-19** and the version commit (and the cut, if first), all as one group
     (rule 7). New in its "Done when": T-19's mutations and checks repeated; the golden files regenerated by their
     generators and compared; ADR-95's table read against `rows.rs`; **step 3b** (the core changed since `460aef5`): the
     Reviewer repeats T-19's benchmark and memory figures under `benchmarks.md` section 5; item 1's "the core diff is
     empty" is replaced by "the core diff is T-19's files only".
  4. **Content of a4**: as ADR-88 item 1 and ADR-93, plus T-19 (a reader change). "Not in a4: R-040" is withdrawn; R-048's
     result (T-19b) is not in a4 unless done before the version commit.
  5. **The release notes' LONG sentence** (tag message, the a4 record, the README's "What it reads" line): "A LONG value on
     the conventional path no longer stops the file at 32,768 bytes: it is read in the two forms `exp` writes (one field
     up to 65,520 bytes, 4,000-byte chunks from 65,535 bytes; confirmed on 11g and 18c up to 100,000 bytes). A LONG of
     65,521 to 65,534 bytes whose form is not yet known, and a conventional LONG RAW value of 32,768 bytes or more, still
     stop the file with `E-UNSUPPORTED-LAYOUT`."
- Consequences: the order table of session 24 in the plan; a4 waits for T-19 and its review.
- Rejected: the cut before T-19 (its first public run would be red, and ADR-93 allows only a fix of that run before
  anything else); a4 without T-19 (the release would ship a reader that the shipped tests fail on the committed corpus).
- Amendment (session 25, ADR-104): item 2's order gains T-19b and T-19's step 3b before the cut; item 3's Reviewer 6c also
  covers T-19b and the step 3b figures; item 5's sentence is replaced by ADR-104 item 4.

## ADR-99 The three conventional LONG / LONG RAW forms (R-048, R-051); replaces ADR-95 items 1, 3 to 5 - decided (session 25)
- Context: rules v3.9 [R2] (session 20 text), [R8], [Y9]; `exp.md` 6.6 and 5.6, sessions 19 and 20; R-048 and R-051
  "generated and used". Conventional path (flag 0), first u16 u of a type 8 or 24 field: one field for u = 1 to 0xfff9
  (seen above 0x7fff: 0x8000, 0xffef, 0xfff0, 0xfff1, 0xfff5 to 0xfff9; 0xfff2 to 0xfff4 in no fixture, "hypothesis");
  `fa ff` + u16 n + `00 00` + n bytes for 65,530 to 65,533 (n = 0xfffa to 0xfffd, all four seen; "u32 LE" a hypothesis;
  "stop if the last two bytes are not `00 00`"); `fd ff` chunks from 65,534. 0, 0xfffb, 0xfffc: "not in the fixtures,
  stop". **LONG RAW = LONG** in every form, 18c and 11g (11 sizes, 11g = 18c byte for byte from 0x2000). Direct path: a
  chain for every size, unchanged. Checked: `long4_exp_18c`@0x22277 `fa ff fa ff 00 00`, `long5_exp_11g`@0x5229d `fa ff
  fb ff 00 00`, `long5_exp_18c`@0x32286 `f9 ff`; `rowlen_stats.py`: 28 one-field values from 0x8000, 16 `fa ff`, 16
  chunked, all in LONG / LONG RAW columns; 0 elsewhere.
- Decision:
  1. **The first u16 u of a field of a type 8 or type 24 column with path flag 0** (one table for both types):

     | u | reading | basis |
     |---|---|---|
     | `fe ff` | NULL | [R3] |
     | 0 | `Framing` (unchanged) | [R8] |
     | 1 to 0xfff1, 0xfff5 to 0xfff9 | one field of u bytes (bit 0x8000 is part of the length) | [R2] (a), confirmed |
     | 0xfff2 to 0xfff4 | one field of u bytes **and** `W-UNTESTED-LAYOUT` (ADR-101 item 1) | [R2] (a), hypothesis; R-052 |
     | 0xfffa (`fa ff`) | form (b): u16 n, then u16 z; z = 0 and 0xfffa <= n <= 0xfffd, else `Framing` (ADR-101 item 2); then n value bytes | [R2] (b) |
     | 0xfffb, 0xfffc | **stop**: `E-UNSUPPORTED-LAYOUT` | [R8] v3.9 ("stop") |
     | 0xfffd (`fd ff`) | form (c), chunked (ADR-95 item 2, both types) | [R2] (c) |
     | `ff ff` | `Framing` "ff ff inside a row" (unchanged; at a row start [E1]) | [R8], [E1] |

     Every other column and the direct path are unchanged (0 and 0xfff0 to 0xfffd `Framing` in any other column [R8]).
  2. **Chunked form**: as ADR-95 item 2 for both types; a chunk u16 from 0xfff0 stays `Framing`; the threshold of the
     "short" warning is **65,534** (`CHUNKED_MIN_TOTAL`), the smallest chunked size seen (`long4_exp_<v>` row 6, 17
     chunks, 16 x 4000 + 1534).
  3. **Messages** (exact; no value bytes, ADR-58). The two R-048 texts of ADR-95 item 3 retire. New:
     - `E-UNSUPPORTED-LAYOUT` at the field's u16, `length` = u: `conventional LONG or LONG RAW field starting 0xfffb or
       0xfffc, not in the fixtures [R8]`;
     - `E-FRAMING` at the u16 n (field + 2), `length` = n: `conventional LONG or LONG RAW 4-byte length outside 65,530 to
       65,533`;
     - `E-FRAMING` at the u16 z (field + 4), `length` = z: `conventional LONG or LONG RAW 4-byte length whose upper two bytes
       are not 00 00`;
     - `E-FRAMING` at the chunk's u16 (text widened to both types): `conventional LONG or LONG RAW chunk length from
       0xfff0`;
     - `W-UNTESTED-LAYOUT` at the field's u16 of the first such value, once per column and table section:
       `conventional LONG or LONG RAW length 0xfff2 to 0xfff4 read as one field, not in the fixtures (R-052)`;
     - `W-UNTESTED-LAYOUT` (ADR-95 item 2): `conventional LONG or LONG RAW in the chunked form with fewer than 65,534
       bytes`.
     The stop behaves as every stop (ADR-12; Python `UnsupportedError` through `Catalog.stop_error`, CLI exit 1).
  4. **Value, sink, bounds**: a form (b) value is one field: its offset is its first data byte (field + 6, ADR-41 item 6),
     it goes to the sink as a plain value, `max_lob_bytes` is compared with n before the bytes are taken (`E-LOB-TOO-LARGE`
     at field + 6 with `length` = n; under `on_value_error="null"` the n bytes are passed by length). LONG RAW chunks go to
     the binary sink as pieces (ADR-95 item 6, no decoder).
  5. **Raw digest** (`RawHasher`, `gen_exp_raw.py`): a form (b) value is the four bytes `f9 ff ff ff`, n as u32 LE and the
     n bytes (the 6 framing bytes are not hashed), so a golden line tells the three forms apart; the doc comment's "at most
     65,520" becomes "plain lengths at most 65,529". `f9 ff ff ff` cannot be a u32 length (all are below 0x10000) nor
     another marker (`ff`, `fd`, `fc`, `fb`, `fa` in use). `long_sha256` (assembled) is unchanged: one value has the same
     assembled digest in all forms and on both paths.
  6. **`W-LONG-LENGTH`**: LONG and LONG RAW above **100,000** bytes (the largest confirmed for both types, both paths, both
     versions: `longraw4_exp(_direct)_18c`, `longraw5_exp(_direct)_11g` id 11); one `LONG_LIMIT_BYTES`; messages `LONG
     value above 100,000 bytes, the largest confirmed` and `LONG RAW value above 100,000 bytes, the largest confirmed`.
     `above_32000` keeps its name and meaning.
  7. **Risk 29 closed**: 65,533 is `fa ff` + n (`long4_exp_<v>`@0x42295), 65,534 chunked (@0x522a6); no size collides
     with `fd ff` or `fe ff`.
- Consequences: every conventional LONG / LONG RAW size reads except a field starting `fb ff` / `fc ff` (stop) and 0
  (`Framing`); 65,522 to 65,524 bytes read with a warning; T-19b (ADR-103).
- Rejected: keeping LONG RAW apart (R-051 shows LONG RAW = LONG at 11 sizes on both versions, byte for byte); reading
  the 4 bytes after `fa ff` as a u32 without the range check (a hypothesis, and a misread length misframes the file);
  reusing the plain-field serialisation for form (b) in the raw digest (a golden line could not tell the forms apart).

## ADR-100 R-050 answered (TSLTZ); R-049 negative; R-052; the corpus after sessions 19 and 20 - decided (session 25)
- Context: rules v3.9 [T5], [Y8] (session 19), [E2], [E3]; `requests.md` R-049 to R-052. `tsltz2_exp(_direct)_<v>`
  (TSLTZ(0), (3), (9), 4 rows; `tsltz2_check.py`: the field bytes identical in all four files). The reader marks a 7-byte
  TSLTZ value with `W-UNVERIFIED-TYPE` ("value form not confirmed by a fixture", `values/datetime.rs` `decode_tsltz`,
  `reader.rs` `Kind::Tsltz`); the owner's facts call it "the untested-layout warning": the code is `W-UNVERIFIED-TYPE`.
- Decision:
  1. **R-050 answered**: descriptor length 7 for f = 0 and 11 for f = 3, 6, 9 (`types/map.rs` agrees: no change); a value is
     7 bytes when the stored fraction is zero (in every precision, also under descriptor 11), else 11 bytes [Y8] = [Y5].
     T-19b removes the `W-UNVERIFIED-TYPE` for a 7-byte TSLTZ value (`decode_tsltz` returns the timestamp only; the doc of
     `WarningCode::UnverifiedType` names the ROWID prefix alone; arch 5.4). `W-TSLTZ` stays on every type-231 column
     (UTC or the database zone: unknown, R-019; every fixture is at +00:00). The generator keeps TSLTZ out (ADR-80's
     reason stands: the meaning is the design's, and the four files are compared value by value).
  2. **R-049 negative**: 11g XE refuses `SEGMENT CREATION DEFERRED` (ORA-00439, `11g/truth_r049.txt`; no dump). [E1] /
     [E2] / [E3] already accept an empty table with or without the end marker on any version; no reader change. A real
     11g Enterprise Edition dump can hold such tables: a README limitation line (T-19b) says the 11g case is untested.
  3. **R-052** (sizes 65,522 to 65,524; LONG RAW 65,526 to 65,528 and 65,532): stays **proposed, very low**; run only on
     the condition of ADR-101 item 5. No reader change waits for it.
  4. **Corpus** (Analyst's tools, rerun in this session): **229** EXP files (105 / 97 / 27), 436 table sections, **450**
     binary parts, 41,494 rows, 243,905 column fields, 20,084 NULL, `problems 0`; truth files **97** (18c 49, 11g 45,
     11g-cp1251 3; `11g/truth_r049.txt` and `18c/truth_r045.txt` are records without a table); 263 `.dmp` files (229 EXP +
     34 Data Pump). `longraw4_*` exists for 18c only (R-048 ran LONG RAW on 18c; the 11g LONG RAW sizes of R-048 are ids 8
     to 11 of `longraw5_exp(_direct)_11g`).
- Consequences: T-19b (ADR-103); R-050 closed in `open-questions.md`; risk 29 closed (ADR-99 item 7).
- Rejected: keeping the TSLTZ warning until the time-zone question (R-019) is answered (`W-TSLTZ` covers that question;
  `W-UNVERIFIED-TYPE` was about the byte form, now confirmed).

## ADR-101 The five policy questions of Analyst 19 and 20 - decided (session 25)
- Context: the Analyst's questions and the owner's assistant's leans (prompt of session 25). The charter: what the rules
  mark unknown or hypothesis gets a typed error or a warning with the offset, never a silent guess.
- Decision:
  1. **First u16 0xfff2 to 0xfff4**: read as one field **with** `W-UNTESTED-LAYOUT` (ADR-99 item 3); `strict`: `E-STRICT`.
     Reason: both neighbours are confirmed one-field sizes (65,521; 65,525 to 65,529) and the forms after them are fully
     mapped (`fa ff` exactly for 0xfffa to 0xfffd, chunks from 0xfffe), so the only competing reading would be a fourth
     form for three sizes; a stop would block a legitimate file for that; the warning keeps it visible with the offset,
     and a misread would meet the next field's framing and the [R4] row end at once. (ADR-95 item 4 stopped because a
     whole range of 14 sizes had no form; that reason no longer holds.)
  2. **0xfffb, 0xfffc as a first u16**: `E-UNSUPPORTED-LAYOUT`, message naming [R8]. They lie in the marker band (`fa`,
     `fd`, `fe`, `ff` are markers) and could be a marker the fixtures do not show, i.e. a layout, not damage; the sizes
     they would stand for are written `fa ff` + n. **Out-of-range n after `fa ff`, or non-zero upper bytes**: `E-FRAMING`.
     Inside a known form, every size from 65,525 to 65,535 is in a fixture on both types and versions, so an n below
     0xfffa (written as one field) or above 0xfffd (chunked), or a length above 0xffff (chunked, `long3`), is not what `exp`
     writes for a valid value: the bytes are damaged.
  3. **A chunked total below 65,534**: keep `W-UNTESTED-LAYOUT` (no fixture shows it; the framing is self-delimiting, so
     the value has one reading).
  4. **`python/exp_survey.py`** (owner-owned, stops on type codes 8 and 24): left as it is; Q-4's LONG part uses `oradmp
     survey` from a T-19b build (the a4 wheel). Check of `_survey.py` at `921971f`: the column line prints the type code
     (231 visible), the LONG line prints `values`, `chained`, `chunked`, `largest`, `above 32,000 bytes`; the dump's
     warnings and a stop are printed. **Missing**: a count of values between 65,521 and 65,534 bytes. T-19b adds
     `LongStats.sizes_65521_to_65534` (Rust and Python, last field; every non-NULL value whose total is in that range, any
     path) and prints it in the LONG line as `65,521 to 65,534 bytes N`. "LONG RAW above 32,767" needs no new count after
     T-19b (such values read; the open LONG RAW sizes all lie in the new count's range). The limit of `exp_survey.py` goes
     into `docs/formats/README.md` through the Analyst ("Inputs for Analyst session 21").
  5. **R-052**: stays `proposed`, very low; run only if a survey of a real dump shows the warning of item 1 (a conventional
     value of 65,522 to 65,524 bytes) or a stop of item 2.
- Consequences: ADR-99's table; T-19b; Q-4's LONG list restated.
- Rejected: stopping on 0xfff2 to 0xfff4 until R-052 (blocks a file for an interpolation every neighbour supports);
  `Framing` for 0xfffb / 0xfffc (would call a possibly valid unknown marker "damage"); `UnsupportedLayout` for an
  out-of-range n (the form is known, its values are mapped); changing `exp_survey.py` (owner-owned).

## ADR-102 The four T-19 deviations - decided (session 25)
- Context: the T-19 note, "Behaviour as built" and "Problems and observations".
- Decision:
  1. **Once per column and table section** (not "part") for the short-chunked warning: accepted; it matches the
     zero-piece warning that ADR-95 item 2 named as its model. ADR-95 item 2 is amended; the new warning of ADR-99 item 3
     follows the same rule.
  2. **`LongStats.chunked_short` and `first_chunked_short`** (Rust only): accepted; they are the walker's input for the
     warning, not a reported count; Python's `LongStats` stays as ADR-95 item 8 plus ADR-101 item 4.
  3. **`Dump::rows` / `part_rows` without the 128 KiB minimum**: a defect (a valid file gives `E-TRUNCATED` for a caller
     with a small buffer); **fixed in T-19b**: one constant (`MIN_BUFFER_BYTES = 1 << 17`) applied at every
     `ByteStream::new` of `dump.rs` and `reader.rs`, with a test (`buffer_bytes = 4096` reads `long4_exp_18c` row 5,
     65,533 bytes in form (b), and `long3_exp_18c` row 4). Reason: form (b) adds fields of 65,539 bytes on the same path;
     the fix is one line and testable.
  4. **Offsets and the Python stop**: a chunked value's offset is its first data byte: accepted (ADR-95 item 6, ADR-41
     item 6); the empty value's offset `field + 4` accepted. The scan stop's `UnsupportedError.column` is `None` (column and
     row in the message): accepted as the existing catalog-stop behaviour (ADR-12); ADR-95 item 3's "as for every error"
     reads "`.column` may be `None` for a scan stop". Property cases with a LONG above 100,000 bytes run without `strict`,
     the expected warning count from the Spec: accepted.
- Rejected: changing the warning to once per part (a second rule for one kind of warning); exposing the Rust-only fields
  in Python (no caller needs them).

## ADR-103 Task T-19b: the three forms, the session 19 and 20 fixtures in the tests, the TSLTZ warning - decided (session 25)
- Context: ADR-99 to ADR-102; the Analyst's list for `exp/rows.rs` (prompt of session 25); the T-19 note "For the next
  tasks"; `CLAUDE.md` rule 7; ADR-80 as amended.
- Decision:
  1. **Scope** ("Done when" in the plan, T-19b): the reader of ADR-99 (the four `rows.rs` changes: the LONG RAW stop from
     0x8000 removed; the `0xfff1..=0xfffc` arm narrowed to 0xfffb, 0xfffc; the `fa ff` arm; `CHUNKED_MIN_TOTAL` 65,534;
     plus the 0xfff2 to 0xfff4 warning, the texts, `W-LONG-LENGTH` for LONG RAW); the TSLTZ warning removed (ADR-100 item
     1); the `dump.rs` buffer fix (ADR-102 item 3); `sizes_65521_to_65534` (ADR-101 item 4); the 18 new EXP files and 10
     truth files in every corpus test, every value of every row compared with the truth files; generator, golden files,
     damage, fuzz seeds; named mutations; README; architecture sections already written by the Architect.
  2. **Model**: Sonnet. Works in the owner's tree (the new fixtures are there, uncommitted; `corpus/` is not touched).
  3. **Commits**: the Implementer commits nothing. The owner commits, in this order, in one push: (a) a corpus commit with
     the 18 + 10 fixture files and their logs, the two `SHA256SUMS`, `corpus/scripts/make_pending3.sh` and
     `corpus/sql/18_*.sql` (4 files); (b) the T-19b commit. Pushed together, CI runs once, on (b); (a) alone would make
     master red (the counts). The owner pastes the summaries (ADR-92 item 3).
  4. **Step 3b** is not part of T-19b: it runs once, after T-19b, on T-19b's commit (ADR-104 item 1, row 2), so the
     figures cover T-19 and T-19b together.
- Consequences: a4 carries T-19b; Reviewer 6c reviews it with T-19.
- Rejected: the fixtures committed first in a separate push (a red master, and the cut must start green); T-19b after
  a4 (a4 would ship a stop on LONG RAW from 32,768 bytes that the corpus already shows to be readable).

## ADR-104 The order to a4; step 3b after T-19b; Reviewer 6c's scope; the LONG sentence; the public name at the cut - decided (session 25; the cut and its settings need the owner, Q-14)
- Context: ADR-98 (order), ADR-93 (cut); owner's facts of session 25: T-19 done and green; T-19's item (m) not run (needs
  the machine quiet, `ora11` and `ora18` stopped, `ORADMP_WORK` = the folder that holds `t_big_exp_18c.dmp`); the owner
  agreed that `docs/prompts`, `.claude` and `CLAUDE.md` stay private (own small Gogs repositories, ignored in the public
  tree) and that `docs/design`, `docs/formats`, `docs/review` are public; the Q-14 checklist is the owner's assistant's job.
- Decision:
  1. **Order** (the plan's table, session 25): T-19b (with the owner's two commits) and its push run green -> **T-19
     (m)**, step 3b, one Implementer session alone on the machine, on T-19b's commit, note only -> [OWNER] the cut (Q-14),
     first public run green -> Architect (short): record S and P, close Q-14 -> **a4 version commit** (+ `[project.urls]`)
     and its run -> **Reviewer 6c** -> Architect: record 6c, the a4 checklist -> the release.
  2. **Reviewer 6c covers** T-17b, T-16a, T-16a2, T-19, T-19b, the step 3b figures and the version commit (and the cut).
     Item 1's core diff (`460aef5..C -- crates/oradmp-core/src`) lists T-19's and T-19b's files only; item 8 adds T-19b's
     mutations and checks, ADR-99's table read against `rows.rs`, and step 3b repeated by the Reviewer.
  3. **The public repository's name**: the public repository is `oradmp`; the private GitHub repository is **renamed at
     the cut**, as the cut's first step, before the public one is created (GitHub keeps a redirect from a renamed
     repository's old name until a repository of that name is created, GitHub Docs "Renaming a repository"; creating the
     public one ends that redirect, which is wanted). **Hazard**: after the public repository takes the old name, a clone
     whose remote still names the old address pushes to the **public** repository; so every remote of the private
     history (the VM clone, any other) is pointed at the renamed address (`git remote set-url`) and checked with `git
     remote -v` before the public repository is created. The step goes into the owner's assistant's Q-14 checklist; the short
     Architect session after the cut records the public address for `[project.urls]`. Recorded as the owner's decision;
     nothing else of Q-14 changes here.
  4. **The LONG sentence** (tag message, the a4 record, the README's "What it reads"; replaces ADR-98 item 5): "LONG and
     LONG RAW values on the conventional path are read in the three forms `exp` writes (one field up to 65,529 bytes, a
     4-byte length for 65,530 to 65,533 bytes, 4,000-byte chunks from 65,534 bytes; confirmed on 11g and 18c up to 100,000
     bytes), and a value of 65,522 to 65,524 bytes, a size no test file has, is read with a `W-UNTESTED-LAYOUT` warning."
- Consequences: the order table of session 25; a4 waits for T-19b, step 3b, the cut (if first), 6c.
- Rejected: step 3b before T-19b (it would be repeated, the core changes again); T-19b after the cut (the public snapshot
  would carry a stop the corpus refutes).
- Amendment (session 26, ADR-105, ADR-107): item 1's order is done up to "close Q-14" (T-19b `1bb7785`, T-19 (m)
  `0520cd5`, the cut); the rest continues in ADR-107 item 3. Item 3 is done (archive remote `github` names
  `oradmp-private`). Item 4's sentence is in the README since T-19b.

## ADR-105 The cut recorded; Q-14 closed; what is public and what is private - decided (session 26)
- Context: Q-14 (session 22), its steps (session 23), ADR-93, ADR-104 item 3. The owner reports the cut done (session 26
  facts); this record states what exists, so that later records and a stranger reading the public tree agree.
- Decision (a record of facts; the checked ones are marked "checked", the rest are the owner's report):
  1. **P and S.** P = `b55eb10376a9402aee36a1ee61779cf087c7bfb3`, the last commit of the private archive (2026-10-26). S =
     `9950ef204fb20b4d7f5d9da31467ffbe9aeeff60`, the first commit of the public repository
     `https://github.com/granddad-wolf/oradmp`, 925 files, no tags, author and committer GitHub's no-reply address
     (checked). S's tree is P's tree plus `HISTORY.md` (checked: `git ls-tree -r` differs in that one line). **The cut
     is 2026-10-26** (P 15:25:06 -0500, S 15:50:21 -0500, both checked in session 27), as `HISTORY.md` says. The
     public settings, the first public runs and the two manual dispatches on S followed over the next days; they carry
     no recorded date and no "cut complete" date is stated. `HISTORY.md` is the owner's file; nothing in it is changed by
     an agent.
     Amendment (session 27): the date "2026-10-29" given here for the cut in session 26 was wrong (ADR-108 item 2).
  2. **Scrub** (Q-14 step 1, option (i): in place, four archive commits before P): `0cbd165` (home path -> `<home>`, owner
     e-mail -> `<owner e-mail>` in `docs/design/{decisions,implementer-notes,open-questions}.md` and reviews 02, 03; one
     check line in `open-questions.md` reworded); `6a5869c` (`CONTRIBUTING.md` no longer points to `CLAUDE.md`);
     `ac58e08` (the `corpus` user's password leaves the repository: `corpus/scripts/config.sh` reads `CORPUS_PW` from the
     environment or `~/.config/oradmp/corpus.env`, `run_sql` passes it to `corpus/sql/00_setup.sql` as `&1`; the
     passwords of the two throwaway users `corpus_own`, `corpus_ru` stay literal, owner's decision: local test
     containers only); `b55eb10` = P (`docs/prompts/`, `.claude/`, `CLAUDE.md` removed and ignored). The handle
     `elessarelfstone` stays in the clean-room ADR (owner's decision: keep). The owner states nothing in `corpus/` came
     from real data (the Russian files are generated with `UNISTR`).
  3. **Q-14's checks on S** (owner): no hit for the home path, the e-mail, the machine name, the Gogs host, the password
     (text or binary); token patterns hit only the Architect's description lines in `open-questions.md` (accepted); the
     three `SHA256SUMS` OK; `diff -r` of P and S: only `HISTORY.md`; the three private paths absent.
  4. **Private, kept by the owner**: `docs/prompts/`, `.claude/` (two small private Gogs repositories, `oradmp-prompts`,
     `oradmp-claude`), `CLAUDE.md` (lives as `.claude/CLAUDE.md`, the root `CLAUDE.md` a symlink); ignored by the public
     `.gitignore`. **Public**: everything else, including `docs/design/`, `docs/formats/`, `docs/review/`,
     `docs/research/`, `docs/briefs/`, `corpus/`. Cites of the private paths in public documents stay (`HISTORY.md`
     explains them); no public document sends a contributor to them as a step (checked: no file outside `docs/`,
     `.gitignore` and `HISTORY.md` names them; `CONTRIBUTING.md` holds the checks of the definition of done).
  5. **Old history.** The archive is the owner's `~/p/oradmp-archive` (read-only for agents: `git -C ... log / show /
     diff`); every hash cited in `docs/` before session 26 names a commit there. Gogs `oradump.git`: `master` and
     `archive/master` at P, tags `v0.1.0a1` to `v0.1.0a3`; the old GitHub repository is `granddad-wolf/oradmp-private`
     (kept until after the a4 release; the archive clone's `github` remote names it, checked). The public repository has
     no tags. The owner's working clone has one remote, `origin` = the public repository; commits carry the no-reply
     address; GitHub's "block pushes that expose my e-mail" is on.
  6. **Reproducibility at S** (amends nothing in ADR-90 item 6, adds evidence): two CI runs on S gave the same wheel
     `4dca095c5aa951dae8c104d8eb69bb924d1c7527e9b60c93916222002b479262` and sdist
     `582ee53a3b42ca81a9e033c392474aca1178f86e72810699f6824bfbb89e900e` (canonical root, `release root: canonical`,
     `check_dist: ok`); the owner's test build of S at a scratch root gave the same sdist and another wheel
     (`a204e54f...`), as ADR-90 predicts (the SBOM holds the root). VM-against-CI wheel equality at the canonical root was
     shown at archive commits `d081109` and `af6a54a` (ADR-98 item 1) and is not repeated for S. The claim stays: the
     sdist is byte-identical across the machines tried; the wheel only when built at the canonical root. S's sdist
     differs from P's (another commit time, so another `SOURCE_DATE_EPOCH`), as Q-14 step 9 expected.
  7. **The release rule from a4 on**: every release is built from a commit of the public repository, with
     `tools/release_build.sh <commit> --twice --release` at the canonical root `/var/tmp/oradmp-release` (ADR-90), and its
     tag is pushed to `origin` only. a1 to a3 are rebuilt only from the archive (`HISTORY.md`).
  8. **Public repository settings** (owner, before the first push): Actions allow-list = actions by GitHub plus
     `taiki-e/install-action@1e29709c0ec63babbe8f266397a465f29a825f91`, full-SHA pinning required; approval for all
     outside collaborators; read-only workflow permissions; ruleset `master-protection` (no deletion, no force push);
     Issues on; Discussions, Wikis, Projects off; private vulnerability reporting on. Consequence: a task that changes the
     `taiki-e` pin, or adds any non-GitHub action, needs the owner to change the allow-list before its push (the task's
     "Needs from the owner" says so).
  9. **First public CI** (owner's paste): push run on S twice: `checks` nextest 488 passed + 4 skipped, pytest 251,
     `cargo deny` ok; `wheel` `release root: canonical`, `check_dist: ok`, the three venvs 248 / 241 / 248; hashes as item
     6. These equal the T-19b note's VM counts (488 + 4, 251). Dispatched on S: `extended` (fuzz 60 s each: `read`
     58,555 runs, `walk` 123,318, `values` 9,033,953, 0 findings; `t14_damage` 10,000 cases 5 passed; `t17_synth` 2,000
     cases 30 passed + 1 skipped; fuzz clippy ok; bench report only, a 4-CPU runner, not comparable) and `manylinux2014`
     (ADR-106 item 3). The whole `extended` job took 535 s.
  10. **Closed by this record**: Q-14; O-19 (a) (superseded in session 22, now done: the Project-URL comes with T-20);
     O-23 (b) (the billing figure: standard runners are free on a public repository; ADR-92 item 4's budget lapses for
     them); risk 27. O-16 stays closed (the root `.gitignore` is still the owner's; it now also ignores the three private
     paths). New owner item **O-24** (the private `CLAUDE.md` text for the public layout; `open-questions.md`).
- Consequences: the plan's order continues at T-20 (ADR-107); T-16b runs on the public repository with every job free.
- Rejected: rewriting the archive hashes in the documents (they are history; `HISTORY.md` explains them); a
  `docs/design/` copy of the private files' contents (they are private by the owner's decision).

## ADR-106 Four rulings after the cut: the fuzz compile step, the `T_BIG` scan watch, `manylinux2014`, the owner's open items - decided (session 26)
- Context: T-19 (m) addendum (the T-19b break of `fuzz/fuzz_targets/values.rs` was found only by a manual run, because the
  push jobs never compile `fuzz/`); the `T_BIG` scan "watch" of T-19 (m) (-4.8 to -6.8 % against baseline 2 in three
  runs, not bisected); the `manylinux2014` dispatch on S; the owner's open items. ADR-92 item 4's minutes budget does not
  bind a public repository; its runner has 4 CPUs.
- Decision:
  1. **Fuzz: a compile step on every push, and the `extended` run on each release candidate.** A new push job
     `fuzz-check` (parallel to `checks`, so their wall times do not add): checkout, the nightly of
     `fuzz/rust-toolchain.toml` installed as the `extended` job does, `cd fuzz && cargo check --locked`, a summary line
     with the toolchain and the job's duration. Reason: a core API change then breaks the push run of the commit that
     made it, not a later manual run; `cargo check` needs no `cargo-fuzz` install (its compile errors are the ones the
     T-19b break produced, a type mismatch). Unverified: that `cargo check --locked` in `fuzz/` passes without the flags
     `cargo fuzz build` adds, and its cold wall time; T-20 shows both on the VM first and records them. The `extended`
     dispatch on the candidate stays a release step (it is the only CI run of the targets). Done in T-20.
  2. **The `T_BIG` scan watch: nothing now; Reviewer 6c repeats it in step 3b on C; a bisect only if it shows again.**
     I agree, with one addition: if 6c's three runs again fall 5 to 10 % below baseline 2, 6c runs the `T_BIG` scan of
     the a3 core (`460aef5`, from the archive) in the same sitting, alone, before anyone bisects; equal figures mean the
     machine, a gap means the code. Above 10 % is a regression under rule 3 and stops the release. Reason: the read of
     the same file is within 1.2 %, the scan is the noisiest figure (`benchmarks.md` 4.2), and the scan's code changed
     only by one branch per LONG field.
  3. **`manylinux2014`: risk checks (b) of ADR-53 item 7 pass at S; acceptance (T-16c) remains.** Proven at S: the wheel
     builds with `maturin --zig --compatibility manylinux2014` and is tagged `manylinux_2_17` / `manylinux2014`;
     `auditwheel` 6.8.2 (pinned) reports it consistent with `manylinux_2_17`, highest symbol `GLIBC_2.14`. Not proven:
     the abi3 tag of that wheel (risk check (c): read from the summary in 6c); a build through `tools/release_build.sh`
     with the remaps and `check_dist.py` (T-16c (a)); any **install** on a glibc 2.17 system: a CPython 3.11+ there, a
     `pyarrow` 15 or later with a cp311 `manylinux2014` wheel and its numpy, full `pytest` against the installed wheel
     (T-16c (b)); the three venvs and Rocky Linux 8 (T-16c (c)); acceptance (d). a4 ships `manylinux_2_28` only (ADR-91
     unchanged); T-16c stays after the a4 tag.
     Amendment (session 28): risk check (c) is shown: the `manylinux2014` summary of C's dispatch carries both `Tag:`
     lines (owner's paste, ADR-109 item 2). T-16c (a) to (d) remain.
  4. **The owner's open items block nothing of a4.** Q-4 (the real-dump counts) is run with `oradmp survey` from the a4
     wheel, so it follows a4 by design; R-052 runs only on Q-4's condition (ADR-101 item 5); R-046 is low; the RHEL 8 host
     test is done with the a4 wheel and is no gate (as for a1 to a3); deleting scratch folders matters only for the
     canonical root, which the release step moves aside anyway; `oradmp-private` and the snapshot folder: keep until the
     a4 tag is pushed, then the owner's choice (recommendation: archive `oradmp-private` read-only rather than delete it,
     because the a3 record cites its runs).
- Consequences: T-20 gains item (f); CONTRIBUTING's CI section names three push jobs; Reviewer 6c's step 3b may add one
  run of `460aef5`.
- Rejected: `cargo fuzz build` on every push (installs `cargo-fuzz` from source on each run for no extra check of the
  kind that failed); only an `extended` run before each release (a break would again surface late, at the release);
  bisecting the `T_BIG` scan now (one watch figure, within limits, on a shared-load machine).

## ADR-107 Task T-20 (the a4 version commit); Reviewer 6c's scope after the cut; the order to a4 - decided (session 26)
- Context: ADR-93 item 1, ADR-104, ADR-105, ADR-106. The cut is done, so the version commit adds `[project.urls]`.
- Decision:
  1. **Task T-20** (plan; Implementer; Sonnet; small): version `0.1.0a4` everywhere a test checks it; `[project.urls]`
     `Source` and `Issues` = the public repository (labels from PyPA's "Well-known Project URLs in Metadata", PEP 753;
     Issues are on) with a
     `check_dist.py` rule and its cases; README and CONTRIBUTING made true for the public tree; ADR-106 item 1's
     `fuzz-check` job. `HISTORY.md` is not touched. The owner commits, pushes once, pastes; no tag.
  2. **Reviewer 6c** (Opus) covers T-17b, T-16a, T-16a2, T-19, T-19b, T-19 (m)'s figures, T-20 (= C), and the cut
     (scrub residue, `HISTORY.md`, the settings as far as the tree shows them, the public CI). It reads archive-only
     commits from `~/p/oradmp-archive` (read-only) by fetching that clone's `master` into **its own scratch clone** of the
     public repository below `/var/tmp`, so tree diffs across the cut work (`git diff` needs trees, not shared history);
     the owner's clone and the archive are not changed. It builds only in its scratch folder; step 3b on C alone on the
     machine. Items in the plan ("Release 0.1.0a4", session 26 block).
  3. **Order**: T-20 and its run -> [OWNER] one `extended` and one `manylinux2014` dispatch on C, pasted -> Reviewer 6c ->
     Architect (record 6c, the a4 checklist) -> a closure task if 6c needs one -> [OWNER] the release (old canonical root
     moved aside first; Linux `manylinux_2_28` wheel + sdist; upload and tag the owner's) -> T-16b, T-16c.
     Amendment (session 27): T-20b (ADR-108 item 5) comes between the `e1ed10b` dispatches and Reviewer 6c; C is T-20b's
     commit, and the two dispatches are repeated on it.
- Consequences: the plan's order table (session 26); the a4 checklist is written after 6c.
- Rejected: splitting the fuzz job into its own task before T-20 (one more push and review for one small workflow job);
  a README section on the cut (the README says nothing about provenance; `HISTORY.md` does).

## ADR-108 T-20 recorded; the cut date; the throwaway users; `pipefail` in CI (task T-20b); the candidate and Reviewer 6c's commit - decided (session 27)
- Context: the owner's session 27 facts; the T-20 note's deviation (`set -o pipefail` in the `fuzz-check` step) and its
  remark that other steps pipe into `tee` without it. Read in this session: `.github/workflows/ci.yml` at `e1ed10b` has
  **no `defaults:` block and no step sets `shell:`** (`grep -n 'defaults\|shell:'` finds nothing). GitHub's "Workflow
  syntax for GitHub Actions" (default shell table): an unspecified shell on Linux runs `bash -e {0}`; `shell: bash` runs
  `bash --noprofile --norc -eo pipefail {0}`; `defaults.run.shell` sets the shell of every `run` step of the workflow.
  Bash (GNU Bash manual, "Pipelines", "The Set Builtin"): without `pipefail` a pipeline's status is that of its last
  command, so `X | tee f` is 0 whenever `tee` succeeds; `-e` stops a script only on a non-zero status it sees.
- Decision:
  1. **T-20 DONE**: `4aa33e6` (Architect session 26 documents) and `e1ed10bff1587c28861bcf0b1c6e9f950c788cce` (T-20), one
     push `9950ef2..e1ed10b`. Push run on `e1ed10b`: `checks` nextest 488 passed + 4 skipped, `cargo deny` advisories /
     bans / licenses / sources ok, pytest 256 passed (251 + T-20's five cases); `wheel` `release root: canonical`,
     `check_dist: ok` (with the `Project-URL` rule), the three venvs 253 + 3 skipped, 246 + 4 skipped (pyarrow only:
     `tests/test_consumers.py`'s seven tests are one module skip, `importorskip("pandas")`), 253 + 3 skipped (the three
     probes), `SOURCE_DATE_EPOCH` 1793310726 (= `%ct` of `e1ed10b`, checked), wheel
     `a3fb19c3cf43c830c1628806b44a86f09238dc211c74b2e012c82dc464903807`, sdist
     `7a2ff91c6b96ffe864463b4605a60b718f00e28acdc7fef3f088d6573cffd83f`; `fuzz-check` green, 35 s to the end of its step,
     `cargo check` 26.39 s (pinned nightly, cold runner; the VM: 23.1 s cold). Dispatched on `e1ed10b`: `extended` (fuzz
     60 s each: `read` 68,805 runs, `walk` 221,819, `values` 14,799,481, 0 findings; 463 seed files; `t14_damage` 5
     passed; `t17_synth` 30 passed + 1 skipped; fuzz clippy ok; bench report only; 435 s) and `manylinux2014` (wheel
     `oradmp-0.1.0a4-cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.whl`, sha256
     `be6c78e5763cd3313f4e4ab5f2d4cb2f5e8487748486f56ae7b1fd213cc5160a`, not shipped; `auditwheel` 6.8.2 "consistent
     with manylinux_2_17_x86_64", symbols up to `GLIBC_2.14`; 119 s; the abi3 tag in the file name, so ADR-106 item 3's
     risk check (c) is shown by the name; 6c reads the `Tag:` line). The note's `set -o pipefail` deviation is accepted
     (item 4 makes it redundant).
  2. **The cut date is 2026-10-26** (P and S, checked); the session 26 text "2026-10-29" for the cut was wrong (ADR-105
     item 1 amended). The settings, first public runs and manual dispatches followed on later days and carry no date in
     the documents. The session dates in the file headers ("session 26, 2026-10-29") are the Architect sessions' dates
     and stay.
  3. **Throwaway users**: the literal passwords of the Docker corpus container's users `corpus_own` and `corpus_ru` stay
     in the public tree (owner's decision: a local, throwaway container); decided.
  4. **`pipefail`: a real hole, in four of the five jobs.** Every `run` step except `fuzz-check`'s check step runs
     without `pipefail`. Each step that pipes into `tee` (or another command), and whether a failure of the piped
     command is hidden (the step and the job stay green):

     | job | step (line at `e1ed10b`) | pipeline | failure hidden? |
     |---|---|---|---|
     | checks | nextest (95) | `cargo nextest run ... \| tee` | **yes** for failing tests: the step's status is the skip-list `diff` that follows; a build failure shows (no `SKIP` lines, the `diff` fails). Whether nextest's fail-fast still prints the four `SKIP` lines is unverified; the check must not depend on it |
     | checks | cargo deny (105) | `cargo deny check \| tee` | **yes**, completely: the pipeline is the step's only command |
     | checks | pytest (128) | `pytest -rs -q \| tee` | **yes**: only a `SKIPPED` line after it can fail the step; failed or errored tests do not |
     | wheel | release build (206) | `bash tools/release_build.sh ... \| tee` | partly: the `grep '^check_dist: ok'` and the `cp` of `SHA256SUMS` after it catch a failure up to and including `check_dist`; a failure after it (the script's report lines) is hidden |
     | wheel | venvs (223, 229) | `python -c ... \| tee`; `pytest ... \| tee` | the first is guarded (the `case` on `oradmp.__file__` fails on an import error); the second **yes**: the skip, probe and `--version` checks after it do not see failed tests |
     | fuzz-check | check (322) | `cargo check --locked \| tee` | no (`set -o pipefail`) |
     | extended | seeds (380) | `tools/fuzz_seeds.sh \| tee` | **yes**, completely |
     | extended | fuzz `read`, `walk`, `values` (383, 386, 389) | `tools/fuzz_run.sh <t> 60 \| tee` | **yes**, completely: `fuzz_run.sh` exits 1 on a finding (its last lines), the step stays green, so the crash-file upload (`if: failure()`) never runs; only the summary's findings count shows it. The worst of the list |
     | extended | long property runs (393, 394) | two `cargo nextest ... \| tee` | **yes**: only the `.proptest-regressions` check after them can fail the step |
     | extended | fuzz clippy (403) | `cargo clippy -- -D warnings \| tee` | **yes**, completely |
     | extended | bench (406) | `cargo bench ... \| tee` | **yes** (a report, but a bench that fails to build or panics shows green) |
     | manylinux2014 | build (487 to 491) | `basename`, `auditwheel --version`, `auditwheel show`, `unzip \| grep`, `sha256sum`, each `\| tee` | `auditwheel show` **yes** (the two tag `grep`s after it do not read its output); `unzip \| grep` is guarded by the `grep -q` of `tags.txt`; the other three cannot fail in a way that matters |
     | all | Job summary (`if: always()`) | pipes inside `$( )` or ending in `\|\| true` | by design (a report) |

     No past run is in doubt as far as the pastes show: every pasted summary of S and `e1ed10b` gives passed counts,
     `ok` for the four `cargo deny` checks and 0 fuzz findings, read from the logs, not from the step status.
     Amendment (session 28): "a hidden failure" (the blocking class of the plan's session 27 block, item 9) means a
     failed command that leaves a check reported as passed (a red result shown green). A command substitution inside
     the arguments of another command (`echo "X=$(...)"`) belongs to the same shell class but is ruled case by case;
     the rule for `ci.yml` from T-21 on: a substitution whose status matters stands alone as an assignment, then its value
     is used (ADR-109 item 3).
  5. **Task T-20b** (plan; Implementer; Sonnet; one small commit, `ci.yml` only): a workflow-level `defaults: run: shell:
     bash`, so every `run` step runs `bash --noprofile --norc -eo pipefail {0}`; the `set -o pipefail` line of
     `fuzz-check` removed (one mechanism); the two constructs that `pipefail` would turn wrong rewritten (the `probes=`
     count in `run_one`, and the `find | grep -q` test of the property step, which under `pipefail` can take the wrong
     branch when `find` gets SIGPIPE); the proof on the VM and on a throwaway branch (plan T-20b). **Before Reviewer
     6c**: the release is then built from the commit 6c reviews, and that commit's CI can be trusted to go red. T-20b's
     commit is **C**, the a4 candidate (the version stays `0.1.0a4`, set by T-20). T-20's hashes (wheel `a3fb19c3...`,
     sdist `7a2ff91c...`, the `manylinux2014` `be6c78e5...`) and the `e1ed10b` dispatches stay in the documents as
     T-20's evidence only; they name no candidate and no release (C has another commit time, so another
     `SOURCE_DATE_EPOCH`, sdist and wheel). The release cross-check uses C's CI sdist hash.
  6. **Reviewer 6c reviews C = T-20b's commit** on the public `master` (its full hash goes into the 6c prompt after the
     owner's commit and green push run); scope and builds in the plan ("Release 0.1.0a4", session 27 block).
- Consequences: one more push, one throwaway-branch dispatch and two dispatches on C before 6c (free on the public
  repository). CONTRIBUTING's CI paragraph stays true (it names no shell). Every new `run` step gets `pipefail` without
  anyone remembering it.
- Rejected: `set -o pipefail` in each affected step (thirteen steps to edit, and the next new step can forget it, as
  these did); `shell: bash` on each step (same); `PIPESTATUS` checks (more code for the same effect); T-20b after a4 (a4 would
  ship from a commit whose CI shows green on a failed `cargo deny` or a fuzz finding, and 6c would review a commit that
  is not the release's); leaving `extended` and `manylinux2014` alone because they are dispatch-only (the `extended` run
  on C is a release step, ADR-106 item 1).

## ADR-109 Reviewer 6c recorded; T-20b DONE; SF-1, N-1 to N-7 and the unchecked items ruled; task T-21 - decided (session 28)
- Context: Reviewer 6c (`docs/review/review-06c.md`) on **C = `4b9883c52ebf709ccf0161883505844fdcc45345`** (T-20b):
  0 blocking, 1 should-fix, 7 notes; "0.1.0a4 can be built from C as it stands". The owner's session 28 facts: C's push
  run and its two dispatches; the `t20b-proof` run. Read in this session: `ci.yml` at C (the five `echo "...$(...)"`
  lines 57, 178, 200, 318, 363; the `find` test at 402; line 199 `python -m pip install --quiet uv==0.12.21` right before
  line 200 in the same step; the venv info line that prints each venv's Python version into the summary); `%ct` of C
  = 1793315613 = the epoch 6c printed.
- Decision:
  1. **Reviewer 6c DONE; no closure task; C stays the candidate.** Verified by 6c in its scratch clone at C: fixture
     checksums; fmt; clippy (three forms); `cargo deny check` (four ok); nextest **488 passed + 4 skipped** (the four
     names of `ci.yml`); `maturin develop --features test-probes`; ruff, ruff format, mypy; pytest **256**; the
     installed wheel in three venvs 253 + 3 / 253 + 3 / 246 + 4; two scratch-root builds byte-identical, `check_dist:
     ok`; **sdist `510f2e39f8ae7eb9660fa4f2313062d2245c3c6d16ebd10740fa0e529c40c0fd` = CI's**; its wheel `79f1ea31...`
     differs from CI's by the build root in the SBOM member only (ADR-90); the cut: 0 hits for home path, e-mail, host
     name, Gogs host, password (text, binary, UTF-16) and the token forms, tree and public history; `HISTORY.md` agrees
     with the archive; step 3b: no regression (the `T_BIG` scan watch showed once, -7.4 %, and the a3 core `460aef5`
     measured in the same sitting was as slow or slower: the machine, ADR-106 item 2 satisfied); T-19 deviation 3
     (`walk` uses the caller's `buffer_bytes`) right, shown for all 229 fixtures with a 64-byte buffer. Every repeated
     mutation fails a named test except N-4.
  2. **T-20b DONE** (`4b9883c`). Push run on C green with `e1ed10b`'s figures (nextest 488 + 4, `cargo deny` four ok,
     pytest 256; `release root: canonical`, `check_dist: ok`, venvs 253 + 3 / 246 + 4 / 253 + 3; `fuzz-check` green);
     `SOURCE_DATE_EPOCH` 1793315613; **wheel `2e4c69a57baafe84e6c5ec0b4b9bef1248c93ea2eabdb57adb17d49d58949dbd`**, **sdist
     `510f2e39f8ae7eb9660fa4f2313062d2245c3c6d16ebd10740fa0e529c40c0fd`** at the canonical root. Dispatches on C:
     `extended` green, 0 findings on 463 seeds (fewer runs than on `e1ed10b`: the runner; `checks` not slower);
     `manylinux2014` green, wheel `1df8ed44bcae37444de00c72b54866d8bfe0c3fbd7794a2668e9632cbe87a1e9` (not shipped),
     `auditwheel` consistent with `manylinux_2_17`, both `Tag:` lines present (ADR-106 item 3 (c) shown). Proof run
     **40974068275** on `t20b-proof` (`75ad263`): `checks`, `wheel`, `extended`, `manylinux2014` red with exactly 3, 2,
     7 and 1 exit-code-7 annotations (= the ids of T-20b (f): `nextest deny pytest` / `build venvs` / `seeds fread fwalk
     fvalues props fclippy bench` / `audit`), `fuzz-check` green, the crash-artifact step ran; the branch deleted locally
     and on `origin`. T-20b (a) to (g) met. **Risk 30 closed.**
  3. **SF-1: not blocking; fixed after a4 in T-21.** The blocking class of session 27 is a failure that leaves a check
     reported as passed (ADR-108 item 4, amended). Line 200 cannot do that: the `python` it calls has just run `python -m
     pip install uv` in the same step, which fails the step first; if `python -c` still failed alone, `NEWEST_PY` would be
     empty, uv would make the "newest" venv with its default interpreter, every check in that venv would still run, and
     the summary's info line for that venv (`Python x.y.z, pyarrow ...`) shows which Python was tested: a visible wrong
     input, not a hidden failed check. The four `JOB_START` lines feed only the summary's duration. Checklist step 1
     reads that info line in C's pasted summary, which excludes the scenario for the release. Rule for `ci.yml` from now
     on: a command substitution whose status matters stands alone as an assignment (`x="$(...)"`), then its value is
     used; never inside the arguments of another command.
  4. **The notes** (one line each):
     - **N-1**: fix after a4 in **T-21** by the same rule (`regs="$(find ... -print -quit)"`, then `[ -n "$regs" ]`, so
       a `find` error fails the step). Not before a4: a runner checkout has no unreadable folder.
     - **N-2**: fix in **T-21**: the two test names lose the count, the comments cite `TOTALS` instead of a file count
       (6c's four sites plus `t11_options.rs:11`, whose figures are those of 211 files, and `:492`). The T-19 (m) note's
       "-5.0 to -6.8 %" reads **-4.8 to -6.8 %** (its own table and the plan are right); the note is history and stays,
       this line corrects it.
     - **N-3**: fix in **T-21**: both generator docstrings run them with `python3 -I` from `docs/formats/tools` (as 6c
       did); regenerated outputs byte-identical.
     - **N-4**: no action. The raise in `Dump::open` has no effect and cannot be tested; the two raises that matter are
       each caught by a named test (R5, R6). A comment edit in `io.rs` would make step 3b due for a5 for no behaviour.
     - **N-5**: fix in **T-21**: CONTRIBUTING pins what CI pins (`maturin==1.15.0 pytest==9.1.1 ruff==0.17.1
       mypy==2.4.0`; `cargo install --locked cargo-nextest@0.9.146 cargo-deny@0.20.2`), and a pytest keeps the two
       files equal.
     - **N-6**: owner, checklist step 2 (the `mv` before the build); no task.
     - **N-7**: no action; recorded in `benchmarks.md` (session 28). Rule 3 needs a confirming run and five of six did
       not confirm; the 11 ms scan is the noisiest figure. The next step 3b reads `T_ROWS` direct first.
  5. **What 6c did not check** (who, when):
     - **The CI pastes**: given as owner's facts in session 28 and recorded in item 2; nothing more.
     - **The `manylinux2014` `Tag:` line**: seen by the owner (item 2); done.
     - **The action pins against their tags**: **before a4, the owner** (checklist step 4, five `git ls-remote` lines,
       read-only, no token). A pin that is not its tag's commit stops the release until the Architect rules: CI would
       then have run code nobody chose. The release files themselves do not depend on CI (built on the VM, checked
       against CI by hash only).
     - **The member-by-member comparison with the CI wheel**: replaced by the release build at the canonical root,
       whose wheel must equal CI's `2e4c69a5...` byte for byte (a gate, ADR-90, ADR-98 item 1); on a difference the CI
       summary's member list (sha256, size, name) is the comparison.
     - **The remaining mutations** (T-19 M1 to M12, T-19b M2, M5, M7, M9 to M12, T-17b (b), (d) to (j), T-16a2 (a), (d) to
       (g)): no action. Each was shown by its task's note; no file they guard changed since (`git diff --stat 1bb7785 C
       -- crates python tests` lists T-20's `check_dist` files only, 6c item 1); every mutation 6c repeated or added
       (R1 to R7, B1 to B3, T16a2-c, T-20 (b) and (f)) behaved as its task recorded, except N-4.
     - **Fuzz runs**: no action; CI's `extended` on C (0 findings, 463 seeds) and T-19 (m)'s 600 s runs on the same core.
     - **`tools/stream_memory.py`**: **before a4, the owner**, once on the release wheel (checklist step 8; seconds;
       `PASS` required, `benchmarks.md` rule 4). Last run at T-15; the binding changed since (T-18d).
     - **A Rocky Linux 8 install, runs as root**: **before a4, the owner**, in a `rockylinux:8` Docker container on the VM,
       as root, three venvs (checklist step 9; a gate, as for a2 and a3). It proves: pip on a RHEL 8-family userland
       (glibc 2.28) accepts the `manylinux_2_28` tag; the extension loads there; pyarrow and pandas resolve there; every
       test passes against the installed wheel as root (with the root-only skips named). It does not prove: a RHEL 8
       kernel, SELinux or FIPS settings, a corporate package mirror or proxy. That is the **RHEL 8 host test**: not done,
       no host; not a gate (as for a1 to a3); done with the a4 wheel when a host is at hand (Q-4 time).
     - **The README beyond four sections**: no action (its other sections are unchanged since 6a / 6b read them).
  6. **Task T-21** (plan; Implementer; Sonnet; small; the first commit after the a4 tag): SF-1, N-1, N-2, N-3, N-5.
     Reviewed with T-16b and T-16c as one group (rule 7: T-21 renames tests and changes comments, no assertion).
     Amendment (session 29, ADR-112 item 1): `ci.yml` at C has **five** `JOB_START` lines, one per job (57 `checks`,
     178 `wheel`, 318 `fuzz-check`, 363 `extended`, 468 `manylinux2014`), not four; T-21 changes all five and needs a
     `manylinux2014` dispatch as well as the `extended` one.
- Consequences: no change to C; the release follows the a4 checklist (ADR-110). Risk 30 closed. T-21 precedes T-16b
  (both edit `ci.yml`).
- Rejected: SF-1 as blocking with a T-21 before a4 (a new candidate, a new push, two dispatches and a closure review for
  a failure that cannot hide a check, and the release files do not come from CI); fixing the notes before a4 (each is a
  document, a comment or a contributor's setup line; none touches the shipped files); a `PIPESTATUS`-style check in
  each `echo` line (the assignment rule is shorter and also covers N-1).

## ADR-110 The a4 release: tag on C, the hashes it must give, the checklist, the release page, tokens, records; the order after a4 - decided (session 28)
- Context: ADR-109. After this session the owner pushes two docs commits (`075a6bf` with the review, and this session's),
  so the public `master` head is no longer C. `tools/release_build.sh <commit>` clones the local repository, checks out
  the given commit and sets `SOURCE_DATE_EPOCH` to that commit's `%ct` (read in this session); every archive's member
  times come from it, so another commit gives other hashes even with an identical tree.
- Decision:
  1. **Tag `v0.1.0a4` goes on C = `4b9883c52ebf709ccf0161883505844fdcc45345`**, not on a later docs commit. C is the
     commit 6c reviewed, whose CI is green and whose hashes are recorded; a docs commit has another `%ct`, so its
     wheel and sdist differ from the reviewed ones although no shipped file changed, and the cross-check with CI would
     need another run. The later docs commits are descendants of C; `git diff --stat C origin/master -- . ':!docs'` must
     print nothing (checklist step 1).
  2. **The hashes the owner's build must give** (`tools/release_build.sh C --twice --release` at the canonical root
     `/var/tmp/oradmp-release`): epoch `1793315613`; wheel `oradmp-0.1.0a4-cp311-abi3-manylinux_2_28_x86_64.whl` =
     `2e4c69a57baafe84e6c5ec0b4b9bef1248c93ea2eabdb57adb17d49d58949dbd`; sdist `oradmp-0.1.0a4.tar.gz` =
     `510f2e39f8ae7eb9660fa4f2313062d2245c3c6d16ebd10740fa0e529c40c0fd` (C's CI push run; the sdist also = 6c's). **Both
     are gates**: the sdist does not depend on the root; the wheel equals CI's at the canonical root since ADR-98 item 1.
     Compare with C's run only: the CI runs of the docs commits give other hashes (other epoch).
  3. **The a4 checklist** is in the plan ("Release 0.1.0a4", session 28 block), numbered for the owner. Changes against
     the a3 checklist: the old canonical root moved aside; the `--release` build at the canonical root; both hashes
     known in advance; the five action pins resolved; `stream_memory.py` on the release wheel; the Rocky Linux 8 counts
     at C with three root-only skips; one remote (`origin`) for the tag; a GitHub release page; the tokens deleted
     after use; the build root kept under a versioned name.
  4. **GitHub release page: yes**, made by the owner from the pushed tag, marked pre-release, body = the tag message,
     the two SHA-256 and the PyPI link; **no files attached** (PyPI is the one place to download; the hashes let anyone
     check). Rejected: attaching the files (a second copy to keep equal); no page (the tag message is not shown on
     GitHub's front page, and users look for notes there).
     Amendment (session 29, ADR-111 item 5): the a4 page exists (pre-release, no files attached); its body as recorded
     holds the SHA-256 lines and the PyPI link; that it holds the tag message is not shown by the evidence and is not
     claimed. The owner may add the tag message to the body (an edit of the page, no new release; ADR-113 item 6).
  5. **Tokens**: the owner creates a project-scoped token on TestPyPI and one on PyPI for this release, types each with
     `read -rs`, and **deletes both after the uploads** (amends ADR-67's "keep the scoped one in the password manager"
     from a4 on). Reason: releases are weeks apart; no upload credential then exists between them; a new scoped token
     takes a minute. A token never appears on a command line, in a file, in an environment that outlives the upload, or
     in an agent session (CLAUDE.md rule 8).
     Amendment (session 29, ADR-111 item 5): carried out in a4 as written (both tokens deleted, typed with `read -rs` in
     a plain shell, no agent session open). Not in this item but used: a **GitHub CLI token**, revoked afterwards. From
     a5 on the same rule covers it: made for the release, limited to this repository and to what the step needs,
     revoked after use, and `gh auth logout` run, because the GitHub CLI keeps a logged-in token in its own storage
     until then.
  6. **Records**: the owner pastes the results (checklist step 17); the Architect writes "Release 0.1.0a4 record" in the
     plan, as for a3. `HISTORY.md` is not changed (it explains the archive and a1 to a3; a4's provenance is its public
     tag). No `CHANGELOG.md` now: the tag message and the release page carry the notes (a file is a later Implementer
     task if users ask; one more place to keep equal is not worth it for an alpha).
  7. **Release notes** (tag message and release page; the LONG sentence is ADR-104's of session 25):
     "oradmp 0.1.0a4. LONG and LONG RAW values on the conventional path are read in the three forms `exp` writes (one
     field up to 65,529 bytes, a 4-byte length for 65,530 to 65,533 bytes, 4,000-byte chunks from 65,534 bytes;
     confirmed on 11g and 18c up to 100,000 bytes), and a value of 65,522 to 65,524 bytes, a size no test file has, is
     read with a `W-UNTESTED-LAYOUT` warning. Dump and output file names that are not valid UTF-8 are accepted by
     `oradmp.open`, the `oradmp` command and `survey`; messages write such bytes as `\xHH`. PyPI now links the source
     repository and the issue tracker. Linux x86_64 wheel (`manylinux_2_28`) and sdist; Python 3.11 or later, pyarrow
     15 or later."
  8. **After a4** (plan, order table of session 28): the owner's record paste and a short Architect session (the a4
     record); **T-21** (first commit after the tag); **T-16b** and **T-16c**, then Reviewer 7 on T-21, T-16b, T-16c;
     **Q-4** (the owner's `oradmp survey` counts from the a4 wheel on the real dumps, on his side) any time after the
     tag; **R-052** only if Q-4 shows a LONG of 65,522 to 65,524 bytes or a field starting `fb ff` / `fc ff` (ADR-101
     item 5); **R-046** low, any time; Data Pump planning after a4.
- Consequences: the release needs no new commit; the plan's a4 record is written after the owner's paste.
- Rejected: tagging the docs head (other hashes than the reviewed ones, item 1); rebuilding C's hashes from a CI run of
  the docs head (other epoch); keeping project-scoped tokens between releases (item 5).

## ADR-111 Release 0.1.0a4 recorded; risk 31 closed; tokens and the release page as practised - decided (session 29)
- Context: the owner released 0.1.0a4 on 2026-10-31 by the a4 checklist (plan, "Release 0.1.0a4 checklist (session
  28)") and gave his facts and a raw evidence file (private, captured 2026-10-31 11:55:33 UTC). The evidence file
  shows: `git ls-remote` of `origin` (tag object `582045a129b24e51c3b585083e463290da1a1039`, `^{}` = C =
  `4b9883c52ebf709ccf0161883505844fdcc45345`); `SHA256SUMS` of `dist/` and the hashes of `dist2/`; PyPI's SHA-256 and
  sizes of both files; `oradmp 0.1.0a4` and the installed `_core.abi3.so` hash from a PyPI install; `oradmp 0.1.0a4`
  from a TestPyPI install; the last pytest line of three VM venvs. Everything else in the facts (pins, toolchain,
  `cargo deny`, `stream_memory.py`, the Rocky Linux 8 counts, `twine check`, the TestPyPI library hash, the tokens) is
  the owner's statement, recorded as such. Checked in this session: see "Changes in session 29".
- Decision:
  1. **a4 is done**; the record is the plan's "Release 0.1.0a4 record (session 29)". Closed by it: ADR-110 items 1 to 3,
     6 and 7; the "Release 0.1.0a4" step; **risk 31** (it did not happen: the owner's build at the canonical root gave
     exactly C's CI files, and those are the files on PyPI). **a5 is the next release** (ADR-113 item 4).
  2. **What was verified against what** (E = in the evidence file or checked in this session; S = owner's statement):
     (a) the owner's canonical-root build of C (`dist/`) = the gates of ADR-110 item 2, i.e. C's CI push run: wheel
     `2e4c69a5...` and sdist `510f2e39...` (E); (b) the second build (`dist2/`) = the first (E: same two lines;
     `reproducible`, S); (c) PyPI's published files = (a), with sizes 3,637,834 and 174,352 bytes (E; and PyPI's JSON
     API fetched in this session: uploaded 2026-10-31 11:43:34Z and 11:43:36Z, not yanked, `requires_python` `>=3.11`,
     `project_urls` Source and Issues, licence expression `MIT OR Apache-2.0`); (d) the sdist = Reviewer 6c's
     scratch-root sdist (ADR-109 item 1), so three independent builds (6c, CI, the owner) agree on it; (e) the library
     installed from PyPI has sha256 `d0efa2dff42b57f5203429ace3b7789d4d1cfbbb7fd09ce986b5badf51b4789c` (E), the one from
     TestPyPI the same (S); (f) the tag object is on C (E, and the local tag in this session); (g) `SHA256SUMS` (206
     bytes, sha256 `936bcd14...`) is exactly the two lines of (a) (rebuilt in this session).
  3. **Deviations from the checklist, accepted** (none touches the shipped files, which are fixed by 2 (a) to (c)):
     - Step 2: the old root went to `/var/tmp/oradmp-release-20261027`, not `...-old-2026-10-27`: a name only.
     - Step 8: three `uv` venvs on the VM (3.11.16 with pyarrow 15.0.2 and pandas 2.2.3; 3.14 with the newest packages;
       3.11 with pyarrow only) instead of one: more than asked; counts 253 + 3, 253 + 3, 246 + 4 = CI's and 6c's.
     - Step 9: the Rocky Linux 8.9 container ran on a **`git archive` export of C**, not on the build clone, with `uv`,
       the image's Python 3.11.13 and a downloaded CPython 3.14.10. Hence `tests/test_project_name.py:34` skips ("not a
       git clone"), the case the checklist names ("each count has one more skip"): 249 + 7, 249 + 7, 242 + 8 are the
       expected 250 + 6, 250 + 6, 243 + 7 with that one skip; the test passes in the VM runs, which are git clones. The
       three root-only skips are those of the checklist (`test_cli.py:594`, `:923`, `test_paths.py:121`).
     - **The three tools outside the checklist** (as this record reads the facts): (i) `uv` for every venv, including
       the container (the checklist used `pip` there except for 3.14); (ii) `twine check` (both files PASSED; a metadata
       check, read-only, in its own venv `/var/tmp/a4-twine`; ADR-67 rejected twine as the *upload* tool, not as a
       check); (iii) the GitHub CLI with a token (item 5). Also the owner's own scripts `/var/tmp/a4-install-test.sh`
       (which also ran `sha256sum -c SHA256SUMS`) and `/var/tmp/a4-rocky-test.sh`: their text is the only exact record
       of what steps 8 and 9 ran, so they are kept as evidence (ADR-113 item 1). The upload tool is not named in the
       facts (the checklist's is `uv publish`).
     - Step 11: the tag message is ADR-110 item 7's text as **one paragraph** (its first line is the whole text), not
       "`oradmp 0.1.0a4`, a blank line, the text". The tag object is dated 2026-10-30 22:39:36 UTC, about 13 hours
       before the uploads; the checklist has no time limit between steps 11 and 12. No action.
  4. **Not done or not recorded** (who, when):
     - The **RHEL 8 host test**: not done, not a gate (as for a1 to a3; ADR-109 item 5); with the a4 wheel when a host
       is at hand (the owner, Q-4 time). The only one of 6c's unchecked items ruled "after a4"; the others were done
       before the upload (pins: step 4; `stream_memory.py`: step 8, PASS, iterator growth 0 kB, stream 20 kB, limit
       10,240 kB; Rocky Linux 8: step 9).
     - **The sdist built from source** (step 8's `X/vsrc`): not in the facts. No build of C's sdist from source is
       recorded anywhere (6c and CI compare its bytes only). Done once in T-21 (ADR-112 item 5); risk 32.
     - Not in the facts: step 1's check of the `pytest-pynewest-latest` info line (moot for the files: the owner's own
       3.14 venvs passed on the VM and in the container); step 3's README grep (`tests/test_version.py` passed in every
       install run, which checks the same); step 8's `oradmp tables` and the exit-3 case; step 9 (b)'s `export` to CSV
       and Parquet; the TestPyPI and PyPI `simple/` index lines (PyPI's JSON stands for the PyPI one; TestPyPI's index
       hash is not recorded, the install from it gave the same library hash, S); step 13's plain `pip install oradmp`
       with `pip --version` (an `oradmp==0.1.0a4` install with `uv` was done instead); step 16's three count checks
       (in the clean-up commands, ADR-113 item 1); the build log L (not in the scratch list). Accepted for a4: each is
       either covered by the hashes of item 2 or by a test that ran.
     - The plain `pip install oradmp` observation is **dropped** from later checklists: the a3 record has it, PEP 440
       explains it, and the README tells users `oradmp==<v>` or `--pre`.
  5. **Tokens and the release page as practised**: both project-scoped index tokens were made for a4 and deleted after
     it, typed with `read -rs` in a plain shell with no agent session open (ADR-110 item 5 as written). A **GitHub CLI
     token** was also used (its purpose is not in the facts) and revoked afterwards. Rule from a5 on (amends ADR-110
     item 5 and ADR-67 item 4): any token a release step needs, the GitHub one included, is made for that release,
     limited to this repository and the permission the step needs, typed hidden, revoked after use; for the GitHub CLI
     also `gh auth logout`, because it keeps the token in its own storage until then. The release page exists
     (pre-release, no files attached, GitHub's own source archives only); its body holds the SHA-256 lines and the PyPI
     link; the tag message in the body is not shown by the evidence and not claimed (ADR-110 item 4 amended).
  6. **For the a5 checklist** (written when a5 is due): the lessons of item 3 and 4 (the owner's scripts pasted with the
     record; Rocky from a `git archive` is fine with the one extra skip named in the expected counts; the TestPyPI
     index line recorded; the tag message from a file with a separate first line; the sdist source build recorded;
     `cargo deny`'s expected warnings named, ADR-112 item 2; the GitHub token rule of item 5).
- Consequences: nothing to undo; the a4 files stay on PyPI. Risk 31 closed; risk 32 opened (until T-21 (k)).
- Rejected: re-running the unrecorded steps on the a4 files now as gates (they are published; each is covered by item 2
  or by a test that ran; the one real gap, the source build, is T-21 (k)); yanking or re-tagging anything (nothing is
  wrong); attributing the three unlabelled VM pytest lines from the evidence alone (they are in the order 253, 246, 253;
  246 + 4 is necessarily the venv without pandas; the other two are taken from the facts).

## ADR-112 T-21 restated: five `JOB_START` lines, `deny.toml`, the duplicate `syn`, the check that nothing shipped changes, the sdist source build - decided (session 29)
- Context: T-21 (session 28, ADR-109 items 3, 4, 6). `ci.yml` at C read in this session: `JOB_START=$(date +%s)` stands
  in **every** job (57 `checks`, 178 `wheel`, 318 `fuzz-check`, 363 `extended`, 468 `manylinux2014`); line 200 is
  `NEWEST_PY`; line 402 the `find` test of N-1. The owner's `cargo deny check` at the release root: four ok, and two
  kinds of warning: the duplicate `syn` (2.0.119 and 3.0.6) and the unused licence allowances `ISC` (`deny.toml` line 20)
  and `Zlib` (line 22). `Cargo.lock` at C: `syn` 2.0.119 is used by `windows-implement`, `windows-interface` (Windows
  targets of `[graph]`) and `zerocopy-derive`; `syn` 3.0.6 by `pyo3-macros`, `pyo3-macros-backend`, `serde_derive`,
  `thiserror-impl`, `borsh-derive` and `wasm-bindgen-macro-support`: proc-macro crates, compile-time only. cargo-deny's
  `[licenses]` documentation: `unused-allowed-license` (`warn` by default, also `allow`, `deny`) decides what happens
  when an allowed licence is not encountered in the dependency graph; the licence configuration applies to the whole
  crate graph.
- Decision:
  1. **SF-1 covers all five `JOB_START` lines and line 200**; N-1 line 402. Since 363 is in `extended` and 468 in
     `manylinux2014`, T-21 needs **one dispatch of each** besides the push run (ADR-109 item 6 amended).
  2. **`deny.toml` unchanged; the `ISC` and `Zlib` allowances stay.** Dropping them cannot break `cargo deny` on C's
     lock (the warning says no crate of the graph of the five targets uses them), but it would break `cargo deny check
     licenses` on any later lock that brings in an ISC- or Zlib-licensed crate (a `cargo update`, T-16b's platform
     crates, a pyo3 or arrow upgrade): a licence ADR-45 item 5 already approved would turn into a red CI run and a note.
     Keeping them costs two warning lines. The warnings are **expected**: T-21's note and the a5 checklist list them
     (the two unused allowances and the duplicate `syn`); any other warning is reported to the Architect. Rejected:
     dropping them (above); `unused-allowed-license = "allow"` (would hide that the graph changed; one more key for two
     lines of output).
  3. **The duplicate `syn`: no action.** Both versions come from upstream proc-macro crates; nothing of `syn` is in the
     wheel's library; removing it needs either upstream releases or a `[bans]` skip entry (a decision for no gain).
     Revisited when a dependency upgrade is made for another reason.
  4. **"Nothing the wheel contains changes", and how it is shown.** Whole-file hashes cannot show it: every archive's
     member times come from the commit's `SOURCE_DATE_EPOCH` (ADR-110 context), so T-21's wheel and sdist hashes
     differ from a4's although no shipped file changes. What is compared instead is the **member list** (sha256, size,
     name) that the `wheel` job's summary prints for the canonical-root build of the pushed commit, against the same
     listing of a4's released files at `/var/tmp/oradmp-release-0.1.0a4/dist/` (read only, after `sha256sum -c
     SHA256SUMS`), computed with the summary step's own Python lines. T-21 keeps the version `0.1.0a4`, so no member
     name or version-dependent member differs. Required: (a) every wheel member equal in all three fields, in
     particular `oradmp/_core.abi3.so` (a4's member hash is expected to equal the installed file's
     `d0efa2dff42b...`); (b) **allowed exception**: `oradmp-0.1.0a4.dist-info/sboms/oradmp-py.cyclonedx.json` and
     `RECORD` may differ in hash **with equal size** (the SBOM's `timestamp`, if maturin's bundled `cargo-cyclonedx` takes
     it from `SOURCE_DATE_EPOCH`, which ADR-56 left "not verified"; `RECORD` holds the SBOM's hash); the note says which
     case occurred, which settles ADR-56's open point; (c) every sdist member equal, except members whose path is in
     T-21's diff list (e.g. `CONTRIBUTING.md` if the sdist holds it); `PKG-INFO` equal. Any other difference stops;
     the Architect rules.
  5. **The sdist source build** (not recorded for a4, ADR-111 item 4): T-21 does it once on the VM, outside the
     repository (rule 9): a scratch folder below `/var/tmp`, a fresh `uv` venv with Python 3.11, `uv pip install` of
     the released `oradmp-0.1.0a4.tar.gz` (Rust and network; `TMPDIR=/var/tmp/oradmp-tmp`, `CARGO_BUILD_JOBS=2`), then
     `oradmp --version` and `python -m oradmp survey --no-names` on a copied fixture exit 0. A failure is a defect of the
     published sdist: stop, note, the Architect rules (a5 content). A permanent CI check of the sdist build goes to
     T-16b (it adds build jobs anyway).
  6. **One public-face line** in T-21 (ADR-113 item 6): `CONTRIBUTING.md` "Release wheel" points to the newest release
     checklist instead of "First release". No change to `README.md`, `pyproject.toml`, `Cargo.toml` or the locks in
     T-21: each is part of the wheel's metadata or of the build.
- Consequences: T-21's "Done when" (plan) gains (h) to (k); the owner's part is one push and two dispatches.
- Rejected: a T-21 wheel compared by whole-file hash (impossible, item 4); building T-21 at a scratch root on the VM for
  the comparison (the SBOM then names that root and differs anyway, ADR-90); moving the README or classifier edits into
  T-21 (they change `METADATA`, so the "nothing shipped changes" check could not hold; they go to a5's version commit).

## ADR-113 After a4: scratch folders, `oradmp-private`, the order, what makes a5, the public face - decided (session 29; the GitHub settings of item 6 need the owner)
- Context: ADR-111, ADR-112; the owner's scratch list; ADR-106 item 4 (`oradmp-private` after the tag); ADR-110 item 8.
- Decision:
  1. **Scratch folders on the VM** (commands: plan, "Clean-up after a4 (session 29; for the owner)"). Keep, with the
     reason: `/var/tmp/oradmp-release` **renamed `/var/tmp/oradmp-release-0.1.0a4`** (the shipped files, `dist2/`, the
     build clone; T-21's member comparison reads it; the canonical root is then free for a5), with an `evidence/` folder
     holding the two test scripts, the build log L if it exists, and any pytest log found in the venv folders; the
     `dist/` of `/var/tmp/oradmp-release-a2` and `-a3` (the shipped a2 and a3 files, a few MB, an offline copy whose
     hashes the plan records; the rest of those roots goes). Delete, with the reason: `/var/tmp/oradmp-release-20261027`
     (predates C, ships nothing; its commit and `SHA256SUMS` are printed first for the record);
     `-af6a54a`, `-ci`, `-ci2`, `-d081109` (the CI-against-VM comparisons of T-16a and T-16a2, whose hashes are
     recorded in ADR-90, ADR-98 and the plan); `/var/tmp/a4-src` (a `git archive` of C; `git archive v0.1.0a4` makes it
     again); `/var/tmp/a4-venvs`, `a4-twine`, `a4-testpypi`, `a4-pypi` (venvs; their results are in the record);
     `/var/tmp/a4-tag-message.txt` (the message is in the tag object, read in this session); `/var/tmp/a4-check` and
     `/var/tmp/oradmp-review-6c` if present. Keep `/var/tmp/oradmp-tmp` (the machine's `TMPDIR`).
  2. **`oradmp-private`: archive it read-only now** (GitHub, its Settings, "Archive this repository"; it stays
     private). The condition of ADR-106 item 4 (after the a4 tag is pushed) is met. Reason: the a3 and earlier records
     cite its runs, an archived repository keeps what its retention keeps, and nobody can push to it by mistake.
     Deleting it is a later owner choice, not before a5 is out; the local archive `~/p/oradmp-archive` stays as
     `HISTORY.md` describes. The snapshot folder: the owner's choice (no gate).
  3. **Order after a4** (plan, "Order of the next tasks (session 29)"): the owner's docs push and clean-up; **T-21**;
     **T-16c** (Linux only; its first step a quick `pip download` check of whether a pyarrow 15 or later wheel exists
     for `manylinux2014` and cp311, which can end the task as "rejected" without any CI work); **T-16b** (macOS,
     Windows, with the sdist build check); **Reviewer 7** on T-21, T-16c, T-16b; then a5. In parallel, the owner's
     items: Q-4 with `oradmp survey` from the a4 wheel (counts only); R-052 only on Q-4's condition; R-046 (low); the
     RHEL 8 host test with the a4 wheel; O-24, Q-13, O-20. Data Pump planning: an Architect session after T-21, scope
     with the owner.
  4. **What makes a5**: at least one change a user can install or read, reviewed: an accepted T-16b platform wheel, an
     accepted T-16c `manylinux2014` wheel, or a reader change that Q-4 (R-052, R-046) or a user's defect report calls
     for. With it: T-21 (already on `master`), the a5 version commit (version, locks, README lines, the classifier of
     item 6), Reviewer 7 (or a closure review) on the exact candidate, step 3b only if `crates/oradmp-core/src/`
     changed, and the a5 checklist with ADR-111 item 6. **The smallest release worth making** is one accepted platform
     wheel or one reader fix; T-21, README wording, classifiers or CI changes alone never make a release. Exception: a
     defect of a4 that harms a user makes a5 a fix release from `master` after a short closure review.
  5. **No new request for the Analyst**: nothing in a4 raised a format question. Next free number stays R-053.
  6. **The public face** (PyPI now shows the project):
     - **Owner, GitHub settings (no commit)**: the repository's "About": description as `pyproject.toml`'s, website
       `https://pypi.org/project/oradmp/`, topics (for example `oracle`, `oracle-database`, `exp`, `dump`,
       `apache-arrow`, `pyarrow`, `parquet`, `etl`, `rust`, `pyo3`); check that Issues are on (PyPI's "Issues" link
       points there) and that private vulnerability reporting is on (`CONTRIBUTING.md` "Security" sends reporters to
       "Report a vulnerability"); optionally add the tag message to the a4 release page's body (ADR-110 item 4).
     - **T-21** (files the wheel does not hold): the `CONTRIBUTING.md` pointer of ADR-112 item 6.
     - **a5's version commit** (changes `METADATA`): the classifier `Development Status :: 3 - Alpha`; the README's
       "Wheels:" line and "Other platforms" as T-16b and T-16c decide; the install lines stay `oradmp==<v>` and `--pre`
       (ADR-81 item 3 (f)). An `Operating System` classifier only when T-16b has decided the platforms.
     - **No badges**: the README is PyPI's project description; a badge loads an image from another service on every
       view and repeats what PyPI already shows. No new section, no feature.
- Consequences: the canonical root is free for a5; T-21 can start after the docs push; nothing in this record needs a
  build on the VM except T-21's one sdist build (ADR-112 item 5).
- Rejected: deleting `oradmp-private` now (the records cite its runs; nothing is gained); keeping every old build root
  (each holds a full clone and target; their hashes are recorded); a5 made of T-21 alone (nothing changes for a user);
  T-16b before T-16c (T-16c is Linux only and may end at its first check; T-16b is the larger task and both edit
  `ci.yml`, so they run one after the other).
