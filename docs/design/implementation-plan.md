# Legacy EXP reader: implementation plan

Architect session 1, 2026-10-07; session 2, 2026-10-14; sessions 3 and 4, 2026-10-15; sessions 5 and
6, 2026-10-16; sessions 7 to 13, 2026-10-17; sessions 14 and 15, 2026-10-18; sessions 16 and 17, 2026-10-21; session 18, 2026-10-22; sessions 19 to 23, 2026-10-24; sessions 24 and 25, 2026-10-25; sessions 26 and 27, 2026-10-29; session 28, 2026-10-30; sessions 29 to 31, 2026-10-31. Tasks for the
Implementer, in order; each is meant for one session. "Rules" are those of
`docs/formats/exp_reader_rules.md` (v3.4; v3.5 from session 8; v3.6 from session 17; v3.7 from session 24; v3.9 from session 25); "arch n" is a section of `exp-reader-architecture.md`. Every
task ends with `cargo fmt`, `cargo clippy -- -D warnings` and `cargo nextest run` green (`CLAUDE.md`,
definition of done; in the public repository the same checks are listed in `CONTRIBUTING.md`, "Development setup")
and adds no `unsafe` to the core. Commit hashes cited before session 26 name commits of the private archive
(`HISTORY.md`, ADR-105).

## Changes in session 31

- **T-16c: DONE, accepted** (`a8da2cc` + `94b6368`, results `0bfa70d`; ADR-116). Figures are in its heading and its
  "Session 31 record". Two checks of recorded summaries are held for Reviewer 7 (ADR-116 item 2).
- **a5 ships a `manylinux_2_17` wheel next to `manylinux_2_28`** (ADR-117). The pre-check rides on T-16b as its
  item (i). The a5 gates are in "Release 0.1.0a5 (session 31)".
- **T-16b** made exact: "Session 31" block (ADR-118). It is a test task; whether its wheels ship is **Q-15** (owner).
- **Reviewer 7** scope (new block after T-16c) and the **rule for tests that depend on machine state** (ADR-119).
- **When a5 is cut**: ADR-120 and "Release 0.1.0a5 (session 31)".
- T-21 record: the member count is 69 (16 + 53). "CI minutes budget": the T-16b line. New order table (session 31); the
  session 30 table becomes history.

## Changes in session 30

- **T-21: DONE** (`20bbc80`; push run and a manual run green; ADR-114): figures in its heading and its "Session 30
  record". The wheel-unchanged check (i) passed by the CI listing of C; risk 32 closed (the a4 sdist builds from
  source). Deviations accepted: six cases in `tests/test_contributing_pins.py`, eight more test files with comment
  changes, no T-21 note in the repository.
- **T-16c** made exact (ADR-115): "Session 30" block with the full "Done when" (it wins over the earlier blocks): check
  (0) and what "rejected" writes where; `--manylinux 2014` in the release script; the glibc 2.17 image
  `quay.io/pypa/manylinux2014_x86_64` and `rockylinux:8`, both in CI; nothing published.
- "Release 0.1.0a4 record": step 16 gains the token-history line; "What remains of a4" updated.
- New order table (session 30); the session 29 table is history.

## Changes in session 29

- **Release 0.1.0a4: DONE** (2026-10-31; tag `v0.1.0a4` on C = `4b9883c`; wheel `2e4c69a5...`, sdist `510f2e39...`
  = C's CI files = PyPI's). The record is "Release 0.1.0a4 record (session 29)" under "Release 0.1.0a4" (ADR-111),
  followed by the owner's clean-up commands (ADR-113 item 1).
- **T-21** restated (ADR-112): five `JOB_START` lines (one per job), so one `extended` and one `manylinux2014`
  dispatch; `deny.toml` unchanged (the warnings are expected); the member-list check that nothing shipped changes; the
  one-time source build of the a4 sdist; the CONTRIBUTING pointer.
- T-16c gains a first check (a pyarrow `manylinux2014` wheel for cp311 at all) and goes before T-16b; T-16b gains the
  sdist build check (ADR-113 item 3, ADR-112 item 5). What makes a5: ADR-113 item 4.
- New order table (session 29); the session 28 table is history.

## Changes in session 28

- **Reviewer 6c: DONE** (`docs/review/review-06c.md`, C = `4b9883c`: 0 blocking, SF-1, N-1 to N-7; ADR-109).
  **T-20b: DONE** (`4b9883c`; its push run, the proof run 40974068275 and the two dispatches on C; ADR-109 item 2).
- SF-1, N-1, N-2, N-3, N-5 go to the new **T-21** (after the a4 tag); N-4, N-7 no action; N-6 is checklist step 2
  (ADR-109 items 3, 4).
- "Release 0.1.0a4": session 28 block with the **a4 checklist** for the owner (tag on C; both hashes known; ADR-110).
  New order table (session 28); the session 27 table is history.

## Changes in session 27

- **T-20: DONE** (`e1ed10b`, pushed with `4aa33e6` as `9950ef2..e1ed10b`; ADR-108 item 1): the run figures and the
  `fuzz-check` times are in its heading and its "Result" line.
- The cut date is **2026-10-26** (ADR-108 item 2; the session 26 text "2026-10-29" for the cut was wrong).
- New **T-20b** (ADR-108 items 4, 5): `pipefail` for every `run` step of `ci.yml`, with its proof; Sonnet; before
  Reviewer 6c. Its commit is the candidate **C**.
- "Release 0.1.0a4": session 27 block (C = T-20b's commit; Reviewer 6c's exact scope and where it builds). New order
  table (session 27); the session 26 table is history.

## Changes in session 26

- **The cut is done** (ADR-105): P = archive `b55eb10`, S = `9950ef2` (public `https://github.com/granddad-wolf/oradmp`).
  **T-19b: DONE** (`1bb7785`); **T-19 (m): DONE** (`0520cd5`); both archive commits, in S's tree.
- New **T-20** (ADR-107): the a4 version commit with `[project.urls]`, the README / CONTRIBUTING edits for the public
  tree and the `fuzz-check` push job (ADR-106 item 1). Sonnet.
- "Release 0.1.0a4": session 26 block (Reviewer 6c's scope and where it reads the archive; the release rule from a4 on;
  the tag to `origin` only). "CI minutes budget": lapses for the public repository. New order table (session 26); the
  session 25 table is history.

## Changes in session 25

- **T-19: DONE** (`921971f`; CI green on `bbf921b`: nextest 463 passed + 4 skipped, pytest 250, wheel `release root:
  canonical`, `check_dist: ok`). Its item (m) (step 3b) was not run: it becomes **T-19 (m)**, after T-19b (ADR-104).
  Deviations ruled by ADR-102.
- New **T-19b** (ADR-99 to ADR-103): the three conventional forms for LONG and LONG RAW, the 0xfff2 to 0xfff4 warning, the
  `fb ff` / `fc ff` stop, the TSLTZ warning removed, the `dump.rs` buffer fix, the survey's band count, the 18 new EXP
  files and 10 truth files in every corpus test; Sonnet; the owner commits the fixtures and T-19b in one push.
- "Release 0.1.0a4": session 25 block (T-19b and step 3b before the cut; Reviewer 6c's scope; the LONG sentence; the
  public name at the cut). New order table (session 25); the session 24 table is history.

## Changes in session 24

- **T-16a2: DONE** (`af6a54a`; CI and VM hashes identical; ADR-98 item 1). Open from its note: the owner's `mv` of the
  canonical root before any release build.
- New **T-19** (ADR-94, ADR-95, ADR-97): the conventional LONG forms of R-040, the R-048 stops, the ten session 17
  fixtures in every corpus test, generator, damage, fuzz, step 3b; Sonnet; before the cut. Master's corpus tests are red
  since `cca4c65` (ADR-94 item 5) until T-19.
- "Release 0.1.0a4": session 24 block (T-19 first; Reviewer 6c gains item 8; step 3b due; the LONG sentence of the release
  notes). New order table (session 24); the session 23 table is history.

## Changes in session 23

- **T-17b: DONE** (`4b4e732`). **T-16a: DONE** (2026-10-24, `d081109`; record under T-16a, "Session 23 record";
  ADR-89). Open: the wheel comparison at the same build root, the note's run record and its line for
  `tests/test_threads.py`, the billing figure: in **T-16a2** and O-23.
- New **T-16a2** (ADR-90, ADR-92): `--release` and the canonical root, `check_dist.py --build-root`, the fuzz crash
  artifact, the `auditwheel` pin, the note. New **T-16c** (ADR-91): `manylinux2014` acceptance after a4. T-16b:
  session 23 line (ADR-93 item 4).
- Release checklist: session 23 block (canonical root, authoritative hash, the claim; ADR-90). "Release 0.1.0a4":
  session 23 block (the order with the cut; ADR-93); Reviewer 6c gains item 7 (CI checks).
- New "CI minutes budget (session 23)". New order table (session 23); the session 22 table is history.

## Changes in session 22

- **Reviewer 6b** (HEAD `edc62a5`; no blocking finding; ADR-85): **T-18d: DONE** (`ba8a89c`) and **T-17: DONE**
  (`edc62a5`), both reviewed. T-17 (g)'s wording corrected (one seed folder, `target/fuzz-seeds/`).
- New **T-17b** (ADR-86): SF-1 to SF-5 and N-1 to N-8 of review 06b, with the `Unconfirmed` labels of the generator;
  Sonnet; reviewed in Reviewer 6c with T-16a and the a4 candidate.
- **T-16 split** (ADR-87): **T-16a** Linux CI on the private repository after T-17b (the owner pastes the logs; no
  token); **T-16b** macOS / Windows after a4 (wheels in a5). "Done when" of both under T-16, "Session 22".
- **Release 0.1.0a4** restated (ADR-88): content, order, **Reviewer 6c** (6b ran before T-16). New order table (session
  22); the session 21 table is history.

## Changes in session 21

- **0.1.0a3 released** (2026-10-24, `ff3d6b2`): "Release 0.1.0a3 record (session 21)" written (ADR-81); a3 marked
  done; deviations accepted for a3 only (step 7 in one root container venv; the closure review's three venvs stand in).
  **T-18c: DONE** and reviewed (review-06a-closure, no blocking finding). O-21 done (`9f1dc4d`).
- General checklist: session 21 block (explicit-hash tag, `pipefail` / `PIPESTATUS`, the exact skip list as root in a
  container and the `safe.directory` line, the sdist hash compared with the review's, the plain `pip install`
  observation).
- New **T-18d** (ADR-82): SF-1 of 6a-closure (`_cli.py` and its test), file names that are not UTF-8 (binding
  `PathBuf`, `\xHH` in messages and in `survey`), the `-n -1` subprocess case, README line 173. Reviewed with T-17 and
  T-16 in Reviewer 6b; no closure review of its own.
- **T-17** after T-18d; **T-16** after T-17 when O-22 is done (ADR-83: what CI runs; no `corpus/work/` file on GitHub;
  the memory and throughput tests stay VM-only). New "Release 0.1.0a4 (session 21)" (ADR-84). New order table (session
  21); the session 20 table is history.

## Changes in session 20

- **Reviewer 6a** (candidate `d4d20ef`): no blocking finding; T-13j, T-15b, T-18, T-18b marked reviewed; the benchmarks
  become baseline 2 (`benchmarks.md`, ADR-78). **T-18b: DONE** (`d4d20ef`).
- New **T-18c** (ADR-77): SF-1 (tests against the installed wheel), SF-2 (CLI mutations), N-3 (the I/O error line
  with reason and path), N-2 (README suffix sentence), N-1; its last commit is the new a3 candidate; version stays
  `0.1.0a3`, no version commit. New **Reviewer 6a-closure** ("Done when" under the session 20 order table).
- **Release 0.1.0a3**: a full checklist for the owner ("Release 0.1.0a3 checklist (session 20)"): file names,
  `read -rs`, hash comparison, fresh-venv installs, pip's pre-release behaviour recorded, the tag pushed after PyPI.
  General checklist: session 20 corrections (`<v>` in steps 5, 9, 10; skips per layout; new build root per release).
- **T-16**: what it needs now that the private GitHub repository exists (ADR-79; owner actions O-21, O-22);
  `.github/` is the Implementer's. **T-17**: "Done when" written (ADR-80); starts after the a3 tag is pushed; Opus.
- New order table (session 20); the session 19 table is history.

## Changes in session 19

- **Run B record corrected** (ADR-74): the built commit is **`ff7d525`** (not `4e8f212`); the tag `v0.1.0a2` is on it
  and pushed; `v0.1.0a1` is local only; PyPI comparison equal, install from pypi.org works, both tokens
  project-scoped; O-18 closed; checklist step 10 gains item (5) (the `dist/` folder after the upload).
- **T-13j: DONE** (`1c17633` + `460aef5`; 412 Rust tests + 3 skipped, 143 pytest). **T-15b: DONE** (`0956df0`; note
  only; ADR-75 items 1, 2). **T-18: DONE** (`2ba62eb`; 196 pytest; its findings ruled by ADR-75 items 3 to 8).
  T-15b's prompt data corrected: `T_BIG` comes from `corpus/sql/15_big.sql` (`make_big.sh`), not from 01b.
- New **T-18b** (ADR-75 item 9): the memory test measures every file before it asserts; the micro-benchmarks for a3;
  a missing table in a stopped scan; a warning test on a successful export; the README `head` sentence; then the a3
  version commit.
- **Reviewer session 6 split** (ADR-76): **6a** now (T-13j, T-15b, T-18, T-18b, benchmarks repeated, a3 candidate);
  **6b** later with T-16. **0.1.0a3 without T-16** (Linux only), no Project-URL. T-17 after the a3 tag. T-16 waits for
  the GitHub account (Q-11).
- "Release 0.1.0a3" rewritten (README list, step 3b); new order table (session 19); the session 18 table is history.

## Changes in session 18

- **Release run B: done** (ADR-70): 0.1.0a2 built from `4e8f212` [session 19: wrong; built from `ff7d525`, ADR-74] with `--twice`, on TestPyPI and PyPI 2026-10-22;
  record under "First release" ("Run B record"), with the deviation (the README commit after Reviewer 5, the owner's
  decision on SF-2, diff checked) and the tag found on `ff7d525` instead of the built commit (O-18). Checklist
  corrected with the run's six lessons (steps 1, 6, 7, 9, 10).
- **T-13j** widened (ADR-71): review 05 SF-1, SF-3 and the accepted notes N-1, N-3 to N-6, N-8, beside the LOB-length
  branch; every item names a test, a mutation or a check. Sonnet; reviewed in Reviewer session 6 with T-16 and T-18.
- **T-15b** restated with the prompt data the Implementer needs (O-17 still open).
- New **T-18** the command-line tool `oradmp` (ADR-72), after T-16, before T-17.
- **0.1.0a3**: after T-13j, T-16, T-18 and Reviewer session 6 (ADR-71 item 14); new section "Release 0.1.0a3".
- New order table (session 18; ADR-73).

## Changes in session 17

- **T-13i: DONE** (`469416d`, version commit `dc7668e`; 411 Rust tests + 3 ignored, 117 pytest, 0 skipped); its three
  deviations and the d9 / d10 rules answered by **ADR-69**. T-13i (b) reworded (warning timing per path, ADR-64
  amendment); (h)'s claim corrected (ADR-65 amendment).
- New **T-13j** (after run B): the unreachable "file ends inside a LOB length" branch replaced by `read_u64_le`
  (ADR-69 item 2). New **T-15b** (after run B, when owner action **O-17** has made `T_BIG`): the memory test and
  its mutations on `T_BIG` (ADR-69 item 3). Neither gates run B.
- Reviewer session 5's list gains items (7) and (8) (ADR-69 item 5). Run B's checklist unchanged.
- Order: Reviewer session 5 (closure), release run B, T-13j, T-16, T-17; T-15b when O-17 is done.

## Changes in session 16

- **T-14: DONE** (`abd42ad`; 408 Rust tests; zero fuzz findings) and **T-15: DONE** (`b3a5a7f`; 408 + 3 ignored;
  baseline in the new `benchmarks.md`); both **reviewed** by Reviewer session 4 (no blocking finding).
- New **T-13i** (Opus, alone): review 04's S-1 to S-5 (ADR-64) and the notes marked T-13i in ADR-65; its last commit
  is the version bump to 0.1.0a2 (ADR-67 item 2).
- T-14 (d): the command as written did not run; the correct one is `tools/fuzz_run.sh` (ADR-64 item 3); the manual
  form is under T-14 (d).
- T-16 "Done when" rewritten with what T-13h to T-15 taught (ADR-68); T-17 unchanged in scope.
- "First release": run A recorded as far as git shows (tag `v0.1.0a1` on `b7304d5`); **run B** rewritten (ADR-67):
  closure review, version bump in three files, step 3b (benchmarks when core sources changed), TestPyPI before
  PyPI, token handling, download-and-compare.
- Order: T-13i, Reviewer session 5 (closure), release run B, T-16, T-17; Analyst session 11 before run B.

## Changes in session 15

- **T-13h: DONE** (`39cbb93`; 402 Rust tests, 105 pytest, 0 skipped); its deviations and choices are answered by
  **ADR-61** (the stream error text *contains* the code; `E-CONFIG` on the iterator after `close()`; the lock rule is a
  review item; the pyarrow 15.0.2 run is checklist step 7).
- **T-14 and T-15 rewritten** (ADR-62): every "Done when" item names a test, a command or a number; T-14 gains the README
  stream sentence and the NUL rule (ADR-61 items 2, 3) and is **Opus**; T-15 measures on the owner's `T_ROWS` (O-15)
  and records the stream path's memory; they run **one after the other**, each alone. Arch 8.3 (b) moves to the new
  **T-17** (after T-16).
- **Machine rule** (12 GB): one build at a time, `CARGO_BUILD_JOBS=2`, `cargo nextest run -j 2`, fuzzing and
  benchmarks alone ("Order of the next tasks (session 15)").
- "First release" and the checklist (ADR-63, the owner's Q-12 answer): run A = 0.1.0a1, internal hand-out and the RHEL 8
  install test, never uploaded; run B = 0.1.0a2 after Reviewer 4, the PyPI upload.
- Order: release run A, T-14, T-15, Reviewer session 4, release run B, T-16, T-17.

## Changes in session 14

- Reviewer session 3 (`docs/review/review-03-python-packaging.md`, HEAD `3a1bb5f`): **no blocking finding**; S-1 to S-5
  and N-1 to N-11 answered by ADR-55 to ADR-60. **T-13g: DONE** (`3a1bb5f`; 395 Rust tests, 73 pytest).
- New **T-13h** (Opus, alone): the C stream path keeps the warnings (ADR-55), `tools/release_build.sh` (ADR-56),
  `check_dist.py` rules with a pytest (ADR-57), the message check with decoded values and hex substrings (ADR-58),
  `E-INTERNAL` and the panic guard (ADR-59), and the notes N-2, N-4, N-5, N-6, N-8 (ADR-60).
- T-14 gains review 03's N-9 (exact counts) and waits for T-13h. "First release": built by the script after T-13h;
  the checklist gains the owner's network checks (`cargo deny` advisories / sources), the expected skips, a source
  build, and Q-12; steps renumbered 1 to 11.
- Order: O-14, T-13h, the release, T-14 / T-15, Reviewer session 4, T-16 ("Order of the next tasks (session 14)", with
  Reviewer 4's reading list). The Reviewer 3 reading lists below (sessions 11 to 13, history) now name the paths with
  `oradmp` (review 03 N-8).

## Changes in session 13

- **T-13f: DONE** (commit `3fe33e7`; **395** Rust tests, **69** pytest, `licenses ok`); its note's problems 1
  (`__version__`) and 2 (zig and the activated `.venv`) are answered by ADR-54 items 7, 8.
- The owner renamed the project: **`oradmp`** (Q-10 name part; ADR-54). New **T-13g** "Rename to `oradmp`"
  (Sonnet, one session, alone), after the owner's edit **O-13** of `CLAUDE.md` / `.claude/agents/` / the Implementer
  prompt template.
- T-16, "First release" and its checklist now use the new name (live parts); the done tasks keep the old name
  (history, ADR-54 item 5). Checklist step 2 is the name step of ADR-54 item 9.
- Order: O-13, T-13g, Reviewer session 3 (now also reading T-13g), the 0.1.0a1 release alongside T-14 / T-15,
  Reviewer session 4, T-16 ("Order of the next tasks (session 13)").

## Changes in session 12

- **T-13: DONE** (commit `e2eaae6`; **395** Rust tests, **69** pytest; choices answered by ADR-49 to ADR-51). One red
  check: `cargo deny check licenses` (ADR-49).
- New **T-13f** (small, Implementer): `deny.toml` `[graph] targets` (ADR-49), version `0.1.0-alpha.1` / `0.1.0a1`
  and a version-agnostic `check_dist.py` (ADR-50), `oradump-core` `publish = false`, README install text (ADR-50),
  `.gitattributes` (ADR-52).
- "First release": the **release checklist** (ADR-50); Q-9 closed (RHEL 8+, x86_64 only).
- New **T-16** "More platforms" (ADR-53), after T-15 and Reviewer session 4; not needed for 0.1.0a1.
- Order: T-13f, Reviewer session 3, the 0.1.0a1 release (owner, checklist), T-14 / T-15, Reviewer session 4, T-16
  ("Order of the next tasks (session 12)"); Reviewer session 3's reading list gains T-13's and T-13f's files.

## Changes in session 11

- **T-11b, T-12: DONE** (commit `a19d873`): **376** Rust tests after T-11b, **390** after T-12; **58** pytest (Python
  3.13; also 3.11 and 3.14, and 3.11 with pyarrow 15.0.2 / pandas 2.2.3). Choices answered: ADR-3 (arrow 59),
  ADR-41 items 8, 9, ADR-46.
- **T-13 rewritten** with an explicit "Done when" (session 11 block): `interval_as` (ADR-47), Python truth tests,
  licence files (placeholder for Q-8), `deny.toml`, README, `CONTRIBUTING.md`, pinned maturin, wheel
  `manylinux_2_28` (ADR-48), checked metadata and contents.
- "First release": `interval_as="string"` is in it now.
- Order unchanged: T-13, Reviewer session 3, T-14 / T-15, Reviewer session 4 ("Order of the next tasks (session
  11)"); Reviewer session 3's reading list gains the T-11b / T-12 files and the 12 T-12 breaks.

## Changes in session 10

- **T-9b, T-10b: DONE** (T-9b `e308d9a`, T-10b `7941e40`; **371** tests at `7941e40`: 347 at `7db4667`, 352 at
  `e308d9a`). Reviewer session 2 (`docs/review/review-02-t9b-t11.md`): no blocking finding; S-1 to S-4 and the
  notes are answered by ADR-43 and ADR-44; Q-6 (licence) decided by the owner, ADR-45.
- New **T-11b** (tests only): S-2, S-4, e3 / e4 and the review's notes N-2, N-4, N-5, N-6, N-8, N-9.
- **T-12** gains S-1 (the message check with hex and rendered forms, before the first formatted message) and S-3
  (one offset per value under every policy, ADR-43 item 3) and new test files only.
- **T-13** gains the licence work of ADR-45: `LICENSE-MIT`, `LICENSE-APACHE`, `license` fields, `deny.toml` with
  `cargo deny check licenses`, README and `CONTRIBUTING.md` texts, corpus out of the wheel and the sdist.
- Order: "Order of the next tasks (session 10)". For the next **Analyst** session: rules T9 (386 / 1,907) and X2
  (the third failed export), the R-045 record, the R-040 write-up.

## Changes in session 9

- T-6M, T-3d, **T-11**: **DONE** (T-11 commit `7db4667`; **347** `#[test]` functions by `grep`; choices in N-25,
  answered by ADR-41). Reviewer session 1 (`docs/review/review-01-golden.md`, HEAD `7211137`): no blocking
  finding; S-1 to S-6 are answered by ADR-38, the warning inventory by ADR-39, N-21 to N-24 by ADR-40.
- **T-10b widened** (ADR-38): the truth-file test credits only files a check actually opened, keyed on
  (folder, file); new checkers for T_LONG2, T_INTERVAL2 / T_ROWID2, R_TEXT (all 27), R_STAT, T_STC with its
  counts; exact T_LEN; instance counts from `exp_catalog.tsv`; each of the review's undetected mutations must
  fail a named test, shown on a scratch copy.
- New **T-9b** (small, ADR-41 items 5, 6): `W-CHARSET-MISMATCH` for form-1 CLOB columns (bug from T-9), and the
  offset of `W-LONG-LENGTH` on a direct-path chain.
- T-12 binds T-11's new public fields, maps `E-STRICT` (`StrictError`) and `E-UNSUPPORTED-CHARSET`
  (`UnsupportedCharset`), and makes `Error.message` a `Cow<'static, str>` (ADR-40, ADR-41).
- **Review gate** (ADR-42): after every group of tasks that adds or changes tests over the corpus, a Reviewer
  session runs before the next group starts. Order: "Order of the next tasks (session 9)".
- For the next **Analyst** session (not Implementer work; ADR-38 item 7): rules T9 (still "382 lines, 1885
  column definitions"; `collist.py` gives 386 / 1,907) and X2 (evidence lacks `ru_exp_direct_us7ascii_11g`,
  `EXP-00008` / `ORA-00604` / `ORA-00942`, `exp.md` 14.3) are stale; R-045 is still to be recorded; R-040 can be
  written up (the owner reports `sqlldr` in the 18c container).

## Changes in session 8

- T-9 and T-10: **DONE**, merged on master (2026-10-17): **255** tests, of which **11 fail** because the corpus
  grew from 199 to 201 EXP files (R-038: `ru_lob2_exp_cp1251_11g`, `ru_lob2_exp_us7ascii_11g`) and the failing
  tests compare it with lists made for 199. The reader is not at fault (task T-3d).
- R-038 answered (rules v3.5): ADR-25 item 2 lifted for database set 171; ADR-37 (chunk lengths from the
  stream only).
- New **T-6M** (small): `LOB_CONFIRMED_DB_SETS`, the confirmed check, `t6l_ru.rs` from "stop" to "decodes to
  the truth" for `ru_lob2_*`, the stop kept on a patched copy with database set 178.
- New **T-3d** ("corpus 201"): golden and expected files regenerated by the generators that import the
  Analyst's tools; counts derived from the golden files; `ru_lob2_*`, `truth_lob_ru.txt`, `truth_r045.txt` in
  the lists. New **T-10b**: the T-10 step (2) leftovers of N-22, separate so that T-3d stays mechanical.
- T-11 (d), (f) and T-12 use the 178 copy of T-6M instead of the synthetic R_CLOB value (now confirmed).
- Order: T-6M, then T-3d, then T-11 beside T-10b ("Order of the next tasks (session 8)").

Expected values of the **201** files (rules v3.5 "Changes in session 10"; `tools/corpus_counts.py` re-run
read-only in Architect session 8: TOTAL 201 files, `problems 0`). These are T-3d's targets.

| | `18c` | `11g` | `11g-cp1251` | total |
|---|---|---|---|---|
| files | 89 | 85 | 27 | 201 |
| table sections | 170 | 164 | 52 | 386 |
| binary parts (`INSERT INTO` lines) | 184 | 164 | 52 | 400 |
| rows | 9,525 | 3,311 | 28,376 | 41,212 |
| row (column) fields | 31,585 | 12,943 | 198,479 | 243,007 |
| NULL fields | 2,256 | 2,240 | 15,404 | 19,900 |
| fields after an introducing line | 263 | 223 | 297 | 783 |
| sections with type 112 / 113 | 6 / 5 | 6 / 5 | 9 / 2 | 21 / 12 |

LOB values after the rows: 126 (80 short, 46 long; `tools/rowlen_stats.py`, session 10); NUMBER fields 86,080
(`tools/num_decode.py` step 3, session 10); `SHA256SUMS` lines 147 / 140 / 30 = 317, of which 235 name a
`.dmp` (201 EXP); `sha256sum -c` OK in all three folders (Architect session 8).

## Changes in session 7

- T-5f, T-6L, T-7, T-8: **DONE**. Tests: **232** (194 after T-6; T-6L +17 in `t6l_lob.rs` 12 and
  `t6l_ru.rs` 5; T-5f +8 in `t5f_followup.rs`, with `t3_catalog.rs` / `t5_types.rs` adapted; T-7 +5 in
  `t7_owner.rs`; T-8 +8 in `t8_direct.rs`). fmt and clippy clean. N-19 / N-20 answered by ADR-34 / ADR-35;
  T-7 / T-8 recorded in ADR-36 (no production change).
- Work the answers create: T-9 gets `#[doc(hidden)]` on the two LOB counters and `W-CHARSET-MISMATCH` for
  form-1 CLOB columns; T-10 counts `T_LONG_EMPTY` for the direct LONG files only; T-11 gets `strict` for
  `W-LOB-UNTESTED` (the value becomes `E-LOB-UNCONFIRMED`) and the survey's per-column count of the
  values ADR-25 refuses; T-12 maps `E-LOB-UNCONFIRMED` to `ValueDecodeError`, issues warnings once per
  entry, `W-TSLTZ` once per column.
- Re-planned from T-9: **T-9 and T-10 overlap** (separate worktrees); T-11 after T-9, overlapping T-10's
  tail; its fixture-wide `strict` test waits for T-10. Order: "Order of the next tasks (session 7)".

## Changes in session 6

- T-3b and T-6: **DONE** (2026-10-16; N-17, N-18 answered by ADR-32, ADR-33). Tests reported: **158** with
  T-3b, **194** with T-6.
- T-6's sentence "`batch_bytes` 16 KiB forces one row per batch" was wrong (ADR-33 item 6): corrected in
  the record below; the rule of arch 5.6 step 8 stands.
- The T-5 follow-up is still open and grows into **T-5f** (one short session): ADR-30 items 1 and 3,
  `W-TSLTZ`, `ColumnInfo.type_source` and its use by the reader (ADR-33 item 4), `Dump::rows` /
  `part_rows` errors (ADR-32 item 3), counted warnings flushed on error (ADR-33 item 3).
- Re-planned: T-5f, then T-6L (hook named); T-7 and T-8 in parallel with both (tests); T-9 after T-6L
  (shared files); T-10; T-11 (strict for every new warning, per-column policy, `max_digits_dropped`);
  T-12 (warnings in Python, per-read overrides); T-13. Order: "Order of the next tasks (session 6)".

## Changes in session 5

- T-3c, T-4, T-5: **DONE** (2026-10-16; N-12 to N-16 answered by ADR-27 to ADR-31). Test count reported to
  the Architect: 117 with T-3c and T-4, 143 with T-5.
- Re-planned from T-3b on: T-3b matches and checks **every part** (ADR-31), adds `part` to errors and
  warnings (ADR-29) and the assembled-LONG digest; T-6 decodes in file order per part and emits in declared
  order, raises the value-layer warnings (ADR-27), and can start **in parallel with T-3b** (interface fixed
  below); T-5 has a short follow-up (ADR-30 items 1 and 3); T-10 lists the `E-BAD-DATETIME` /
  `E-BAD-INTERVAL` values of ADR-28; T-11 turns the walker's new warnings into errors under `strict` and
  `round` into an error under `strict` (owner); T-12 gets the pandas / pyarrow checks of ADR-28 and no
  longer waits for Q-3 (Python 3.11, owner).
- Order: "Order of the next tasks (session 5)" at the end.

## Changes in session 4

- Expected values: **199** files, 382 sections, 396 binary parts (table below); the 177-file numbers are
  kept as the T-3a record.
- T-3a: **DONE** (2026-10-15, 81 tests, N-9 to N-11 answered).
- New **T-3c**: the walker of rules v3.4 (direct-path LONG chains, conventional LONG from 0x8000,
  composite partitions, `W-LONG-DIRECT` retired, the T = 0 LOB value of ADR-23). It comes **before**
  T-3b, because T-3b's golden files cover all 199 files.
- T-3b: golden files for 199 files and 396 parts; `PartInfo.subpartition`.
- T-4: ready to start now, independent of the walker; adds `truth_num3`, Decimal256, the session 4
  interval and UROWID rules. T-5: matching by name and declared column order (ADR-24), `n CHAR` lengths.
  T-6: LONG as LargeUtf8 / LargeBinary from chains, schema order. T-6L: surrogate vectors, `lob3`,
  ADR-25 instead of `W-LOB-CHARSET` / `W-LOB-UNTESTED`. T-10: the 7 new truth files. T-11:
  `unconfirmed_lob`, new and retired codes. T-12: the survey's per-candidate NUMBER counts.
- Parallel sessions and the dependencies on Q-1: "Order of the next tasks (session 4)" at the end.

## Changes in session 3

- Expected values: 177 files, 334 sections, 338 binary parts, 237,347 row fields (below). During this
  session 22 more EXP files appeared in `18c` (12) and `11g` (10), listed in `SHA256SUMS` (checksums
  pass), not yet written up by the Analyst (R-031 to R-037 generated: `lob3`, `long2`, `long_exp_direct`,
  `misc2`, `nchar2`, `part2`, `t_num3`, `t_probe11`; none in `11g-cp1251`, so R-038 is still needed).
  Nothing in this design rests on them. The numbers below are those of the 177 files; T-3a's test takes
  its numbers from the Analyst's tools for whatever the rules file covers when T-3a starts, and lists
  any other file as "not yet written up", walked but not compared.
- T-3a: **not done** (`implementer-notes.md` has no T-3a entry; no commit since T-2 touches `crates/`).
  Its scope grows to the whole walker of rules v3.3: 16 type codes, LOB framing [L1 to L4], partitions
  [T10], [E2a], [X5]; the golden files move to the new T-3b.
- New T-3b: catalog with parts, golden files for 177 files (338 parts), log check.
- T-4: INTERVAL and ROWID decoders [Y10, Y11]. T-5: [T9] texts of session 8. T-6: Large types,
  interval and duration builders. New T-6L: LOB values end to end. T-10: `truth_lob`, `truth_misc`,
  `truth_part`. T-12: survey with LOB and partition counts. Alpha definition and order updated.

## Changes in session 2

- Expected values: 169 files and the counts of rules v3.2 (below); the "161 / plan column" split is gone.
- T-1, T-2: done, with their results. T-3: built, accepted after T-3a.
- New T-3a: LONG and LONG RAW in the walker [Y9], regeneration of the golden files, the header checks
  of N-5 (ADR-17), list K with forms (N-3), lengths of [R8].
- T-4: NUMBER decoder of arch 5.2 with `truth_num2.txt`; overflow renderings (arch 5.2a).
- T-5: grammar of [T9] (arch 5.5). T-6: LONG / LONG RAW mapping. T-9: the 768-line map test, the
  US7ASCII rule. T-10: `long_exp`, `nchar_exp`, `t_decl_exp`, `t_num2_exp`. T-11: `on_number_overflow`.
  T-12: survey with the NUMBER profile. Alpha definition updated.

Expected values for the walk, **199** files (rules v3.4 "Changes in session 9"; `tools/corpus_counts.py`
run read-only in Architect session 4, 2026-10-15, `problems 0`). These are T-3c's and T-3b's targets.

| | `18c` | `11g` | `11g-cp1251` | total |
|---|---|---|---|---|
| files | 89 | 85 | 25 | 199 |
| table sections | 170 | 164 | 48 | 382 |
| binary parts (`INSERT INTO` lines) | 184 | 164 | 48 | 396 |
| rows | 9,525 | 3,311 | 27,752 | 40,588 |
| row (column) fields | 31,585 | 12,943 | 194,129 | 238,657 |
| NULL fields | 2,256 | 2,240 | 14,690 | 19,186 |
| fields after an introducing line | 263 | 223 | 291 | 777 |
| empty binary parts, conventional / direct | 12 / 9 | 10 / 7 | 2 / 1 | 24 / 17 |
| sections with a LOB column (112 or 113) | 6 | 6 | 7 | 19 |

Fields after an introducing line: 340 plain, 425 in pieces, 12 without marker; `ANALSTATS ` 180 plain
and 342 in pieces; 122 `ANALCOMPUTE` lines; 2 to 120 columns. Sections per type code outside the ten of
rules v3.1: 112 19, 113 10, 8 12, 24 4, 182 / 183 / 69 / 208 4 each. Partitions: T_PART 3 parts and
T_PART2 6 parts per file, 18c only. LOB values: [L4] counts 74 short and 40 long; the per-file numbers
come from the Analyst's tools as in T-3a. LONG chains: 4 values of 32,000 bytes per version (T_LONG,
T_LONGRAW of `long_exp_direct_<v>`) plus one-piece values. No file stops.

Expected values of the **177** files (rules v3.3; T-3a's record; `tools/corpus_counts.py`
and `tools/check_all.py` run read-only on 2026-10-15, `problems 0`, `rule_diff 0`):

| | `18c` | `11g` | `11g-cp1251` | total |
|---|---|---|---|---|
| files | 77 | 75 | 25 | 177 |
| table sections | 144 | 142 | 48 | 334 |
| binary parts (`INSERT INTO` lines) | 148 | 142 | 48 | 338 |
| rows | 9,210 | 3,210 | 27,752 | 40,172 |
| row (column) fields | 30,609 | 12,609 | 194,129 | 237,347 |
| NULL fields | 2,152 | 2,152 | 14,690 | 18,994 |
| fields after an introducing line | 223 | 223 | 291 | 737 |
| empty tables, conventional / direct | 10 / 6 | 10 / 6 | 2 / 1 | 22 / 13 |
| sections with a LOB column (112 or 113) | 4 | 4 | 7 | 15 |

Fields after an introducing line: 330 plain, 395 in pieces after `fd ff`, 12 without marker; per list
entry ` MODIFY DEFAULT` 30, `ANALSTATS ` 170 plain and 312 in pieces, `COMMENT ON ` 130, `CALTER TABLE`
49, `PROCACTION` 12, `CREATE FORMAT71 PROCEDURE ` 12, `CREATE FORMAT92TRIG ` 10, `CREATE VIEW ` 12; 118
`ANALCOMPUTE` lines; 2 to 120 columns. Sections per type code outside the ten of rules v3.1: 112 15, 113
8, 8 / 24 / 182 / 183 / 69 / 208 2 each. LOB: 49 rows in 15 sections (k = 0 in 11), 62 LOB values (54
short of which 12 empty, 8 long of 100,000 and 200,000 bytes) [L3, L4]. Partitions: 1 section, 3 parts
per file, 2 files (18c). With rules v3.3 **no file stops**: all 177 reach the second `EXIT`.

## T-1 Workspace, byte stream, header, framing, structural walker - DONE (2026-10-12)
- Result (Implementer, commit `7bd945a`, `implementer-notes.md` N-1 to N-5): workspace with
  `oradump-core`, toolchain 1.99.0 pinned; header, framing, items, list K as data, G1, sections, rows,
  both end forms, the empty-table decision of arch 5.4 (already here, N-5); `tests/data/walk_expected.tsv`
  generated from the Analyst's tools covers all 169 files (the 161 of the plan reproduce the session 1
  numbers exactly); the 7 CLOB files stop at 112 as planned; the two `long_exp` files stop at type 8
  (rules v3.1 had no LONG; reversed by T-3a). Open items moved: N-3 (forms of list K) and the header
  checks of N-5 to T-3a; N-4 to T-3 (done there).
- Original goal: walk every fixture from byte 0 to the second `EXIT` and report its structure. No value decoding.
- Inputs: H1 to H6, I1 to I7, G1, list K, T1 to T8, R1 to R5, E1 to E4, S1, S2; arch 2, 5.4, 9; all 161
  files. Workspace as in arch 9 (`oradump-core` only), pinned toolchain, `data/exp_intro_lines.txt`.
- Type 112: the walker stops with `UnsupportedType` (ADR-12). The 7 CLOB files are expected to stop at
  the R_CLOB descriptor (`ru_tables_exp_us7ascii_11g.dmp` at 0x11adc, exp.md 14.6).
- (The rest of T-1 is the session 1 text, kept as the record of what was asked; numbers of 161 files.)
- Done when: (a) header fields of all 161 files equal a table generated from `tools/header_fields.py`
  output (client / database / national ids, R, direct flag, mode); (b) for the 154 files without a LOB
  table the walk ends at the second `EXIT` with 0 warnings, and files, sections, fields per list entry
  and empty tables per path equal the numbers above minus the 7 CLOB files (the exact reduced numbers
  are taken from `check_all.py` per-file lines and committed with the test); (c) with an empty list K
  the same items are read through G1; (d) the list file equals the block in the rules file; (e) the 7
  CLOB files give `UnsupportedType` with the documented offset.
- Needs: nothing.

## T-2 Fixture guard and raw manifest - DONE (2026-10-14)
- Result (commit `3283561`, N-6): checksum test over the three `SHA256SUMS`; `tests/golden/exp_raw.tsv`
  generated by `tests/golden/gen_exp_raw.py` from `tools/exp_parse.py`: 289 tables in 161 files (tables
  before the first X1 stop; the 6 `ru_exp_*` and 2 `long_exp` files list none); the walker reproduces
  every line; `type:maxlen` per column. T-1 and T-2 together: 45 tests green. Still open: the Test
  engineer's review of the manifest (do it on the regenerated file of T-3a).
- Original goal: arch 8.4 and 8.2 (a). A checksum test over the three `SHA256SUMS`; the manifest
  `tests/golden/exp_raw.tsv` (per file and table: columns, type codes, path, rows, NULLs, SHA-256 of the
  raw fields in a documented serialisation) generated once by a small script around the Analyst's
  `tools/exp_parse.py` and reviewed by the Test engineer.
- Done when: the checksum test fails on a changed byte (tested on a copy below `target/`); the walker of
  T-1, extended to hash raw fields, reproduces every line of the manifest; rows per table equal
  `truth_counts.txt` where the table is listed.
- Needs: T-1.

## T-3 Scan, catalog and table metadata - DONE (2026-10-16, its tests re-run on the T-3b golden files)
- State (N-7, N-8; code and `tests/golden/exp_catalog.tsv` committed with `639c614`): `oradump_core::dump`
  with `open`, `scan`, `Catalog`, `TableInfo`, `ColumnInfo`, `TableRows`; N-4 done; logs checked
  against the golden file. Not reviewed in Architect session 2. Its tests are re-run on the golden
  files regenerated by T-3b (session 3: 334 tables, 338 parts); then T-3 is done. Answers to N-7: `implementer-notes.md`.
- Goal: `open`, `DumpInfo`, `scan`, `Catalog`, `TableInfo`, `ColumnInfo` (arch 3.2); column names from
  the `INSERT INTO` line [T3]; names and DDL decoded only for ASCII at this stage; `Catalog.end`.
- Inputs: T1 to T6, X1, X2, R6, E1 to E3; `own`, `multi_*`, `t_wide` (120 columns), `t_empty`.
- Done when: names, column counts, type codes, lengths, path and `num_rows` of every table equal the
  manifest; a reader opened at `rows_offset` reads exactly `num_rows` rows and ends at
  `rows_end_offset`; `FileChanged` is raised for a file replaced after the scan; `METRICET` numbers are
  provably unused (a test patches them in memory and gets the same catalog).
- Needs: T-2.

## T-3a The walker of rules v3.3: LONG, LOB framing, new codes, partitions - DONE (2026-10-15)
- Result (Implementer, `implementer-notes.md` N-9 to N-11; 81 tests in `t1_walk.rs`, `t3a_walker.rs`,
  `t2_manifest.rs`, `t3_catalog.rs`): all 177 files of rules v3.3 walk to the second `EXIT` with no
  warning and no stop; the walker itself reproduces 334 sections, 338 parts, 40,172 rows, 237,347 row
  fields, 18,994 NULL fields, 737 text fields, 22 / 13 empty tables (conventional / direct), 15 LOB
  sections and 62 LOB values; expected values from `tests/data/gen_walk_expected.py` (three TSV files
  from the Analyst's tools; the generator asserts the totals). Header checks of ADR-17, list K in four
  fields, device / inode in `FileChanged`, `Catalog.stopped_table`. The owner accepted its choices except
  the T = 0 LOB value (ADR-23). Of the 22 files of Analyst session 9, `long_exp_direct_<v>` ends in
  `E-TRUNCATED` and `part2_exp_<v>` in `Framing` (N-10): both are T-3c.
- Not done here (by plan): golden files (T-3b); `TableInfo.parts` (T-3b; until then `TableInfo` is the
  first part, N-11).

## T-3c The walker of rules v3.4: LONG chains, composite partitions (session 4) - DONE (2026-10-16)
- Result (commit `38967f1`, N-15): all **199** files walk to the second `EXIT`, no stop, no warning; the
  walker reproduces 382 sections, 396 parts, 40,588 rows, 238,657 row fields, 19,186 NULL fields, 777 text
  fields, 24 / 17 empty parts, 19 LOB sections, 114 LOB values (74 short, 40 long); `t3c_walker.rs` (11
  tests) and the adapted `t1_walk.rs`, `t3a_walker.rs`; `LongStats` per LONG column; AQ-1 answered (no
  T = 0 value). Choices recorded in ADR-29. Left for T-3b: `t2_manifest.rs`, `t3_catalog.rs` still compare
  the 177-file golden files.
- Goal: the walk of all **199** files to the second `EXIT` by rules v3.4, values skipped by length.
  1. Direct-path LONG / LONG RAW fields as piece chains [R2, Y9] exactly as ADR-15 session 4 item 1 and
     arch 2.5 step 5 say (first u16, later u16, pieces of length 0); the scan records per LONG column the
     values, chained values, largest total and count above 32,000 bytes (`LongStats`, arch 3.2).
  2. Conventional LONG / LONG RAW u16 from 0x8000 to 0xffef: `UnsupportedLayout` at the field (ADR-12,
     R-040); 0xfff0 to 0xfffd stay `Framing`.
  3. `W-LONG-DIRECT` removed (ADR-15); `W-LONG-LENGTH` stays a value-layer warning (T-6).
  4. Composite partitions: `COMPARTITION "p"`, `SUBPARTITION "s"`, `ENDCOMPARTITION` and the rules of
     ADR-21 session 4 items 1 to 5 (naming lines, confirmed shapes, `W-UNTESTED-LAYOUT` list, items
     before `ENDPARTITION`); `Part` carries partition and subpartition lines.
  5. [E2] / [E2a] item starts include the five partition lines (they already pass test (a) as built;
     the [E2a] list of known kinds gets the three new ones).
  6. The long LOB value with T = 0: as ADR-23 (warning unless AQ-1 finds a fixture value).
  7. The `INSERT INTO` order is used as is; nothing in the walker reorders (ADR-24 is T-5's).
- Inputs: rules R2, R8, Y9, T3, T10, E2, E2a, L4; exp.md 4.8 (session 9 table), 6.6 (session 9 tables);
  `long_exp_direct_<v>`, `long2_exp_direct_<v>`, `part2_exp_18c`, `part2_exp_direct_18c`; ADR-12, ADR-15,
  ADR-21, ADR-22, ADR-23.
- Done when: (a) `tests/data/*.tsv` regenerated by `gen_walk_expected.py` for 199 files (the Analyst's
  `exp_parse.py` reads chains and composite partitions since session 9) and the numbers of the 199-file
  table at the top come out of the walker; the test "files not yet written up" is gone; no file stops,
  no file warns except as listed in the test (none expected); (b) `long_exp_direct_<v>`: T_LONG row 3
  and T_LONGRAW row 2 are chains of 5 pieces (156 + 4 x 7961) at the offsets of exp.md 6.6, T_LONG_EMPTY
  is empty by [E2] (18c) / [E1] (11g), T_FRAME follows; (c) `part2_exp(_direct)_18c`: 6 parts with 21,
  29, 0, 0, 21, 29 rows, named as `truth_part2.txt` `SP` lines, the 16 `ANALSTATS` items skipped before
  `ENDPARTITION`; (d) on patched copies in memory: a piece u16 `f0 ff` and a first u16 `00 00` in a chain
  give `Framing`; a piece `00 80` gives `W-UNTESTED-LAYOUT`; a conventional LONG length `00 80` gives
  `UnsupportedLayout` with the table named and earlier tables readable; `SUBPARTITION` with no open
  `COMPARTITION`, a missing `ENDCOMPARTITION`, `ENDPARTITION` inside a `COMPARTITION`, `PARTITION` and
  `COMPARTITION` in one section each give `W-UNTESTED-LAYOUT`; a second `INSERT INTO` in a section with no
  naming line still gives `Framing`; a synthetic long LOB value with T = 0 gives `W-UNTESTED-LAYOUT`
  (or no warning, by AQ-1); (e) a synthetic single-column direct-path LONG table, empty and with rows,
  gives the three outcomes of ADR-22.
- Needs: T-3a. Runs in parallel with T-4 and T-5 (different modules).

## T-3b Catalog with parts; golden files for 199 files (session 4: 177 old and 22 new files; session 5: per-part matching) - DONE (2026-10-16)
- Result (commit `41e95ef`, N-17; **158** tests, new `t3b_parts.rs`, regenerated `t2_manifest.rs`,
  `t3_catalog.rs`): `TableInfo.parts` (**396** parts in 199 files; 382 catalog lines), `TablePath` with
  `Mixed`, per-part matching with `PartInfo.file_order` (the 9 parts of T_PART / T_PART2 give the identity
  order), `part` on errors and warnings, `ColumnInfo.lob` / `long`, the LOB digest and the assembled-LONG
  digest (ADR-32 items 1, 2), the log check over 199 logs (T_PART 3 and T_PART2 6 leaf lines per file, none
  left over); T_PART `num_rows` 3,000, T_PART2 100. T-3 is done. Choices recorded in ADR-32. Left over:
  `Dump::rows` returns the first part of a partitioned table (changed by ADR-32 item 3: T-5f); the Test
  engineer's review of the two golden files.
- Session 5 additions (ADR-29, ADR-31), done when also:
  (e) the by-name matching of ADR-24 and the agreement of arch 5.5 step 3 run for **every part**;
  `PartInfo.file_order` is filled; a part whose names (as a set) or per-name descriptors differ from the
  first part's gives `Framing`; a patched later part with another order gives `W-UNTESTED-LAYOUT` once per
  table (in memory; no fixture has one); the 9 parts of T_PART / T_PART2 give the identity order;
  (f) errors and warnings get `part` (1-based, set with `row`); `column` is the file position everywhere,
  the DDL warnings of `types::resolve` included (ADR-29 item 1);
  (g) `exp_raw.tsv` keeps the chain serialisation of T-3c for raw fields and adds, per LONG / LONG RAW
  column of a part, the digest over the assembled values (ADR-29 item 6); the conventional and the direct
  file of `long_exp_<v>` and `long2_exp_<v>` give equal assembled digests;
  (h) `ColumnInfo.long` = the sum of `LongStats` over the parts.
- Goal: `TableInfo.parts`, `partitioned`, `subpartitioned`, `num_rows` as the sum, `PartInfo` with
  `partition` and `subpartition`, `ColumnInfo.lob` and `ColumnInfo.long` (arch 3.2, ADR-21, ADR-15);
  `Catalog.end` for `UnsupportedLayout`; the golden files regenerated for all 199 files.
- Inputs: T-3c; arch 3.2, 5.6 step 9; ADR-12, ADR-15, ADR-21; the generators `tests/golden/gen_exp_raw.py`,
  `gen_exp_catalog.py` (they call the Analyst's `tools/exp_parse.py`, which reads LOB tables, parts,
  composite partitions and LONG chains since session 9).
- Done when: (a) `exp_raw.tsv` has **396** part lines for 199 files (one per binary part, with partition
  and subpartition names; LOB value count, LOB byte total and a SHA-256 over the LOB values; for LONG
  columns the value count and the SHA-256 over the assembled chains; serialisation documented in the
  generator), and `exp_catalog.tsv` **382** table lines; walker and catalog reproduce every line; the
  Test engineer reviews both; (b) the log check of T-3 (N-8): every `rows exported` line of the 199 logs
  matches a table, a partition or a subpartition of the golden files, **none** is left over, T_PART's
  three lines and T_PART2's six subpartition lines match its parts; (c) a reader opened at each part's
  `rows_offset` reads `num_rows` rows and ends at `rows_end_offset`; T_PART `num_rows` = 3,000, T_PART2
  100; (d) the T-3 tests run on the new files; then T-3 is done.
- Needs: T-3c.

## T-4 Value decoders - DONE (2026-10-16)
- Result (commit `43d0f6b`, N-12 to N-14): `values::{number, datetime, raw, long, interval, rowid}`; all 52
  truth NUMBER values and the 85,456 NUMBER fields of 199 files decode and re-encode (through a test-only
  field reader driven by `tests/data/value_parts.tsv`, N-13); every DATE, TIMESTAMP, TSTZ, TSLTZ, INTERVAL,
  ROWID and UROWID field of the 199 files decodes, the only value errors being the documented ones (years
  below 1 in `t_date`, the 4 YEAR(9) values of `misc2`); Decimal256 rendering with `arrow_buffer::i256`.
  Choices recorded in ADR-27; the two type questions in ADR-28. `tools/num_decode.py` step 3 fails
  (N-14): sent to the Analyst (note A-24).
- Goal: `values::{number, datetime, raw, long}` (arch 5.1 to 5.4), pure functions from bytes to the
  lossless intermediates and to Int64 / i128 / f64 / text / epoch values.
- Inputs: Y1 (session 7: the decoding procedure), Y2, Y5 to Y9; arch 5.2 and 5.2a; `truth_num.txt` (33
  rows), `truth_num2.txt` (14), `truth_numtypes.txt`, `truth_date.txt`, `truth_ts.txt`, `truth_tstz.txt`,
  `truth_raw.txt`.
- Done when:
  (a) NUMBER, arch 5.2 step 5: all 32 + 14 non-NULL values and the DUMP columns of `truth_numtypes`,
  both folders, decode to the truth value (exact decimal comparison); all NUMBER fields of the 177 files
  (84,968 by [Y1] v3.3) decode, and a test-only encoder gives the same bytes; every validity check of [Y1] has
  a test with a patched byte string (single byte `81`, digit `00` / `65` positive, `01` / `66` inside a
  negative, leading or trailing zero digit, negative of length 20 without `66`) giving `E-BAD-NUMBER`;
  (b) renderings: Int64 for `truth_num2` rows 10 and 12, "does not fit" for 11; Decimal128(38, 10) exact
  for `truth_num` rows 1 to 18; with `round`: rows 25, 27, 30, 31 and `truth_num2` 13, 14 round (27, 13, 14
  to 0, counted as such); integer overflow under every policy for rows 19 to 24, 26, 28, 29, 32; Float64
  equals `str::parse::<f64>` of the truth text for every row; the text rendering is the truth value;
  (c) dates and times as before (TSTZ: instant equals local time minus offset for all 6 rows; years below
  1 and nanosecond overflow give `ValueDecodeError`);
  (d) proptest round trips of arch 8.3 (c) pass; no panic on arbitrary bytes of length 0 to 64;
  (e) session 3, `values::{interval, rowid}` (arch 5.7, ADR-19, ADR-20): the 3 non-NULL values of IYM and
  IDS in `truth_misc.txt` (both folders) give the months and microseconds of their `TO_CHAR` text (-38
  months for `-0003-02`; 909,045,123,456 microseconds for `+0010 12:30:45.123456`; zero) and, through the
  test-only text renderer of [Y10] with p = 4, f = 6, the truth text itself; RID gives
  `AAAR8lAAMAAAACHAAA` (18c) and `AAAE5IAAEAAAACGAAA` (11g); URID gives the same text; patched lengths,
  months 12, hours 24, a ROWID byte outside the alphabet, a UROWID starting `02` give `E-BAD-INTERVAL` /
  `E-BAD-ROWID`; R-036's arithmetic (mixed signs) gives the sum and `W-UNVERIFIED-TYPE`.
  Session 4 changes to (a), (b), (e) (rules v3.4):
  (a') `truth_num3.txt` (6 values, both folders) joins the exact-decode test; 85,456 NUMBER fields of 199
  files decode and re-encode;
  (b') the Decimal256(P, S) rendering (arch 5.2 step 4) with `arrow_buffer::i256`; the six `t_num3` values
  are exact in Decimal256(76, 40); counts of arch 5.2a for the 52 truth values under Decimal128(38, 10),
  Decimal256(76, 38), Decimal256(76, 40) reproduced by a test (30 / 12 (3) / 10; 42 / 7 (2) / 3; 42 / 2
  (2) / 8), so the survey's classification (5.2b) is tested before T-12;
  (e') intervals by ADR-19 session 4: opposite signs `E-BAD-INTERVAL` (replaces the `W-UNVERIFIED-TYPE`
  case); the Arrow value of each `misc2` interval as arch 5.7 (MonthDayNano for p = f = 9, IYM rows 2, 3
  `E-BAD-INTERVAL`); the test renderer gives every `TO_CHAR` text of `truth_misc2.txt`; UROWID: physical
  without warning, logical `02 ...` gives `*BAMAFbMCwQL+` (18c) and the 11g text of the truth, a logical
  tail of 8 bytes gives `E-BAD-ROWID`.
- Needs: T-1 (error type only). **Ready to start now**, in parallel with T-3c / T-3b. Does not wait for
  Q-1: it builds every rendering and every policy.

## T-5 Declared types and the type mapping - DONE (2026-10-16), follow-up open as T-5f (session 6)
- Result (commit `8e332ba`, N-16): `types::{declared, map, resolve}`; all **382** `CREATE TABLE` lines and
  **1,885** column definitions of the 199 files parse and agree; matching by name, declared order,
  `file_position`; `W-DDL-FALLBACK`, `W-CHARSET-UNSUPPORTED`; `tests/t5_types.rs`. Choices recorded in
  ADR-30.
- Follow-up (one short session, or the first step of T-6; touches `types` only): (1) the three test
  vectors of ADR-30 item 1; (2) ADR-30 item 3: a declared `NUMBER` no Decimal128 holds (s > 38; s < 0 with
  p - s > 38; `NUMBER(*,s)` with s < 0) takes `number_default` with `W-DDL-FALLBACK`, tested on patched
  lines `NUMBER(*,40)`, `NUMBER(3,50)`, `NUMBER(38,-2)`, `NUMBER(*,-2)`; (3) `W-TSLTZ` raised by the
  resolution, once per TSLTZ column (ADR-27 item 2).
- Goal: the column-list parser of arch 5.5 (grammar of [T9]) and the function (descriptor, declared
  type, options) to Arrow type of arch 5.1, with the fallback per table and per column.
- Inputs: `CREATE TABLE` lines of all fixtures; `truth_decl.txt`; arch 5.1, 5.5; ADR-6; owner decision
  Q-1 (Decimal128(38, 10)); ADR-14 (until the owner answers: `round`, with `error` in tests).
- Session 3: the [T9] texts `NCLOB`, `BLOB`, `ROWID`, `UROWID(n)`, `INTERVAL YEAR (p) TO MONTH ` (trailing
  space), `INTERVAL DAY (p) TO SECOND (f)` and their family and length checks (arch 5.5); the Arrow types
  of the new rows of arch 5.1 (T_INTERVAL: IYM Interval(MonthDayNano), IDS Duration(Microsecond) from f =
  6; T_ROWID: Utf8, Utf8; T_LOB: LargeUtf8, LargeBinary; T_LOB2 N: LargeUtf8; R_CLOB CL: LargeUtf8).
- Done when: every `CREATE TABLE` line the walker reaches parses (`tools/collist.py` reads all 334 with 0
  failures; the test takes its expected per-form counts from that tool for the same lines) and agrees
  with the descriptors and the `INSERT INTO` names (arch 5.5 step 3); T_DECL gives the 14 precision /
  scale pairs of `truth_decl.txt` and the Arrow types of arch 5.1 (`NUMBER(*,0)` Decimal128(38, 0),
  `NUMBER(*,2)` Decimal128(38, 2), `NUMBER(38, 0)` Decimal128(38, 0), `NUMBER(5, -2)` Int64, `NUMBER(3, 5)`
  Decimal128(5, 5), `NUMBER(10, 2)` Decimal128(10, 2), `FLOAT(126)` and `FLOAT(63)` Float64, `TIMESTAMP
  (6)` Microsecond, `D_NN` not-null flag set); a table-driven test covers each row of arch 5.1; a patched
  line (unknown type text; a name that differs; one coldef too few) gives the column or table fallback
  with `W-DDL-FALLBACK`, never an error.
- Session 4 (ADR-24, [C6], [T9] v3.4): coldefs matched to `INSERT INTO` names **by name**; schema and
  `TableInfo.columns` in declared order, `file_position` kept; T_LONG and T_LONG2 of the direct files give
  the declared order ID, L, V with file order ID, V, L and no warning; a patched
  `INSERT INTO` order other than LONG-last gives `W-UNTESTED-LAYOUT`; the `n CHAR` lengths of arch 5.5
  step 3 checked on `t_probe11` (7 columns, `truth_probe11.txt` `COL` lines); `number_default` and
  `number_types` accept `decimal256(p,s)`; INTERVAL DAY TO SECOND types by p and f as ADR-19 session 4
  (T_INTERVAL2: IDS Interval(MonthDayNano), IDS0 Duration(Microsecond)); all 382 `CREATE TABLE` lines
  parse.
- Needs: T-3 (catalog with `ddl`, exists). Can start in parallel with T-3c; its all-lines test includes
  the 22 new files once T-3c lands.

## T-6 Arrow output and the first end-to-end read - DONE (2026-10-16)
- Result (commit `112c11f`, N-18; **194** tests, new `t6_read.rs`, `t6_parts.rs`, `t6_patched.rs`,
  `t6_value_parts.rs`): `arrow_out`, `reader::{ReadOptions, TableBatches}`, minimal `charset`; T_STAT1,
  R_STAT, T_LONG / T_LONGRAW (both paths), T_LONG2, T_INTERVAL, T_ROWID end to end; the `RowSink` seam;
  `value_parts.tsv` reproduced from the reader's field bytes over **396** parts (85,456 NUMBER fields);
  partitioned reads: **T_PART 3 batches of 1,095 / 1,098 / 807 rows**, **T_PART2 4 batches** (21, 29, 21,
  29; the two empty parts give none), 100 rows. ADR-18 verified on arrow-array 60.0.0. Choices recorded in
  ADR-33. Not built (assigned): LOB values (T-6L), `W-TSLTZ` and `type_source` (T-5f), `strict`, the
  per-column overflow policy and `max_digits_dropped` (T-11), `trim_char`, set overrides and the
  unsupported set of type 1 / 96 (T-9; `E-CONFIG` meanwhile).
- **Correction (session 6, ADR-33 item 6)** of the session 2 text below: "`batch_bytes` 16 KiB forces one
  row per batch" is wrong. By arch 5.6 step 8 a batch closes after the row that brings the count to the
  cap or beyond; T_LONG's rows add 14, 9 and 32,009 bytes, so 16 KiB gives **one batch of 3 rows**, a cap
  of 14 gives batches of 1 and 2 rows, a cap of 1 gives 3 batches. The test records the rule.
- Goal: `arrow_out` (builders, `batch_rows`, `batch_bytes`), `TableBatches` as
  `arrow_array::RecordBatchReader`; minimal `charset` (AL32UTF8, CL8MSWIN1251). First complete read:
  T_STAT1 from `stat_exp_18c.dmp` and `stat_exp_11g.dmp`, R_STAT from `ru_stat_exp_cp1251_est_11g.dmp`.
- Inputs: T-4, T-5; C1; `truth_stat.txt` (`ROW|` lines), manifest for R_STAT.
- Done when: T_STAT1 equals the 50 `ROW|` lines of `truth_stat.txt` in both versions; R_STAT gives 2000
  rows whose re-encoded fields reproduce the manifest hash; batches respect both limits (tested with
  `batch_rows` 1, 7, 65,536); `stat_exp_none`, `stat_exp_compute` and `stat_exp_us7ascii` give the same
  batches as `stat_exp` (statistics items change nothing, S6, S7); session 2: T_LONG and T_LONGRAW of
  `long_exp_<v>` read as Utf8 and Binary (ADR-15) with the values of `corpus/sql/11_types.sql` (the
  32,000-byte values whole, `batch_bytes` 16 KiB forces one row per batch without splitting a value),
  database set of the header used for L, no warning; a patched L byte above 0x7f gives `W-LONG-CHARSET`.
  Session 3: builders for LargeUtf8, LargeBinary (incremental writes, arch 4.1 (e): verify the
  `std::fmt::Write` / `std::io::Write` behaviour of `GenericStringBuilder` / `GenericBinaryBuilder` on
  the pinned arrow version and record it in `decisions.md` under ADR-18), Interval(MonthDayNano),
  Duration; T_INTERVAL and T_ROWID of `misc_exp_<v>` end to end (= `truth_misc.txt`); a pyarrow check
  is T-12's: record whether `pyarrow.parquet.write_table` accepts Interval(MonthDayNano) (ADR-19).
  Session 4: LONG / LONG RAW are LargeUtf8 / LargeBinary (ADR-15): the session 2 checks of T_LONG and
  T_LONGRAW hold with the Large types; the same values from `long_exp_direct_<v>` (chains, assembled
  across 5 pieces and across record boundaries) equal those of `long_exp_<v>`; T_LONG2 row 1 decodes to
  `дж Café` from `long2_exp_<v>`, `long2_exp_us7ascii_<v>` and `long2_exp_direct_<v>` with no
  `W-LONG-CHARSET` (AL32UTF8 database); a synthetic chain whose UTF-8 sequence is split between two
  pieces decodes; `max_lob_bytes` 20,000 gives `E-LOB-TOO-LARGE` on the 32,000-byte chain; batches
  assembled in declared column order (ADR-24); Decimal256 builder for a `decimal256` override on
  `t_num3`.
- Session 5 (ADR-27, ADR-29, ADR-31): the row decoder reads each part's fields in **file order** and
  places them by `PartInfo.file_order` into builders in **declared order**; it turns the decoders' flags
  into warnings (once per column and table, with part, row, column and offset of the first value) and
  adds the `WarningCode` variants it raises (`W-UNVERIFIED-TYPE`, `W-LONG-LENGTH`, `W-LONG-CHARSET`,
  `W-LONGER-THAN-DECLARED`, `W-NUMBER-ROUNDED`, `W-NUMBER-NULLED`, `W-VALUE-NULLED`); `on_number_overflow`
  defaults to `round` (owner); the field metadata `oradump:number_overflow`. Done when also: the field
  bytes the row decoder sees reproduce the per-part SHA-256 of `tests/data/value_parts.tsv` (N-13: the
  walker replaces T-4's test-only reader); the ROWID prefix flag gives one `W-UNVERIFIED-TYPE` on a patched
  prefix; `t_date_exp_<v>` rows 8, 9 give `E-BAD-DATETIME` (raise) or NULL + `W-VALUE-NULLED` (null).
- **Parallel with T-3b** (session 5). T-6 can start now, on unpartitioned tables, with the one-part
  `TableInfo` that exists (rows offset of the first part, `ColumnInfo.file_position` of T-5 as the order).
  What T-6 needs from T-3b, fixed here so both can build against it: `TableInfo.parts: Vec<PartInfo>` with
  `rows_offset`, `rows_end_offset`, `path`, `num_rows`, `end_marker`, `partition`, `subpartition` and
  `file_order: Vec<u32>` (index = file position - 1, value = declared position - 1); the `part` field of
  `Warning` and of the error type (ADR-29). T-6 writes its reader as "for each part: seek, read rows in
  file order, place by `file_order`", using a one-element list until T-3b lands. What T-3b needs from T-6:
  nothing; neither session changes the `RowSource` seam of arch 2.3 without a note to the other (T-3b works
  in the scan, T-6 in the reader). Split of the done-when: items that need parts or the 199-file golden
  files (R_STAT against the manifest hash, `part2` / `T_PART` through `read_table`, the assembled-LONG
  digests, the `value_parts.tsv` check on partitioned files) are checked after T-3b is merged; everything
  else before.
- Needs: T-4, T-5 (done); T-3b for the split items only.

## T-5f T-5 follow-up and the session 6 fixes (new, session 6) - DONE (2026-10-17)
- Session 7 record: built as planned (N-20, ADR-35); 8 tests in `t5f_followup.rs`; 232 tests in all. The
  ADR-33 item 4 test uses `t_num3_exp_18c` instead of `t_num_exp_18c` (ADR-35 item 5); `NUMBER(*,s)` with
  0 <= s <= 38 is `Declared`.
- (1) to (3) as "T-5 ... follow-up" above: the three test vectors of ADR-30 item 1; ADR-30 item 3 on the
  four patched lines; `W-TSLTZ` once per TSLTZ column, raised by the resolution into `Dump::warnings()`.
- (4) ADR-33 item 4: `ColumnInfo.type_source` (`Declared` / `Default` / `Override`) set by
  `types::resolve`; the reader applies `on_number_overflow` to `Default` and `Override` only. Test: a copy
  of `t_num_exp_18c` in memory whose `CREATE TABLE` line declares V as `NUMBER(38, 10)` (its type equals
  the default) gives
  `E-NUMBER-OVERFLOW` for an 11-decimal value under the default `round`; plain `NUMBER` rounds it with
  `W-NUMBER-ROUNDED`; a `number_types` entry of `decimal(38,10)` on the declared column rounds it too.
- (5) ADR-32 item 3: `Dump::rows` on T_PART / T_PART2 is `E-CONFIG` (`length` 3 / 6); `part_rows(…, 3)` on
  T_PART is `E-CONFIG`; `rows` on every one-part table unchanged (existing tests).
- (6) ADR-33 item 3: the counted warnings are flushed into `TableBatches::warnings()` when the reader
  returns an error. Test (no fixture has both, so in memory): a copy of `t_num_exp_18c` with one V field
  after the first rounded row patched to break [Y1] (`E-BAD-NUMBER` under `raise`): the error comes with
  `W-NUMBER-ROUNDED` counting the rounded rows before it.
- Touches: `types::*`, `dump.rs`, `reader.rs`. Needs: nothing. Runs in parallel with T-7, T-8.

## T-6L LOB values end to end (session 3) - DONE (2026-10-16)
- Session 7 record: built (N-19, ADR-34); 17 tests (`t6l_lob.rs` 12, `t6l_ru.rs` 5). `lob_start` takes the
  form; the two LOB counters stay, `#[doc(hidden)]` (T-9 adds the attribute); done-when (a) to (f) met as
  N-19 reports (the scan's count of refused values moves to T-11).
- Session 6, **where it hooks in** (ADR-33 item 1): `RowCursor::step_with` in `exp/rows.rs` calls
  `lob_value` once per present LOB column after the row's fields [L3]; today `lob_value` only skips (and
  hashes in hashing mode). T-6L adds to `RowSink` `lob_start(column, offset, total) -> Read | Skip`,
  `lob_chunk(column, offset, bytes)` (one call per chunk; a short value is one chunk; an empty value no
  chunk) and `lob_end(column)`, called from `lob_value` **after** each u16 / total has passed the framing
  checks of [L4]. The sink answers `Skip` for an unselected column, for a value above `max_lob_bytes`
  (before any byte: `E-LOB-TOO-LARGE` or NULL + `W-VALUE-NULLED`) and for `E-LOB-UNCONFIRMED`; on `Skip` the
  cursor passes the chunks by their lengths, still checking each. The scan keeps calling `step` without a
  sink. The reader's LOB kinds go into `reader.rs` beside `LongText` / `LongRaw` (same builders,
  LargeUtf8 / LargeBinary, same `batch_bytes` count), the UTF-16 BE streaming decoder into `charset.rs`.
  `column` in every LOB warning and error is the file position (ADR-29).
- Session 6, **ADR-25 is decided by the header**: "single-byte database" means the database set id of
  [H4] is 1, 171 or 178 (`DumpInfo`); in such a dump a long-form CLOB and every NCLOB value is
  `E-LOB-UNCONFIRMED` (or decoded with `W-LOB-UNTESTED` under `unconfirmed_lob="decode"`, a `ReadOptions`
  field, ADR-33 item 2); AL32UTF8 dumps are not affected. The 199 fixtures give no such value (done-when
  (e) uses synthetic values). If R-038 is delivered during T-6L, (e) gets its real values in T-10, not here.
- Session 6, **partitioned LOB table** (ADR-32 item 5): until R-042, a synthetic two-part section built
  from `lob_exp_18c` (the T_LOB part copied behind a `PARTITION` line) reads both parts into one schema
  with equal values; `ColumnInfo.lob` sums the two.
- Touches: `exp/rows.rs` (sink methods, `lob_value`), `reader.rs`, `arrow_out.rs`, `charset.rs`, tests
  `t6l_*.rs`. Needs: T-5f (shares `reader.rs`). Runs in parallel with T-7, T-8; **not** with T-9.
- Goal: arch 5.6 complete in `values` and `arrow_out`: the `LobValue` stream of the seam (arch 2.3),
  UTF-16 BE decoding across chunks, single-byte CLOB, NULL against empty, `max_lob_bytes`,
  `batch_bytes` with large values, `W-LOB-CHARSET`, `W-LOB-UNTESTED`, `E-BAD-TEXT`, `E-LOB-TOO-LARGE`.
- Inputs: L1 to L6, C1, C7; ADR-12, ADR-18; `lob_exp_<v>`, `lob_exp_direct_<v>` (18c, 11g),
  `ru_tables_exp_us7ascii_11g` and the 6 `ru_exp_*` files (`cp1251`); `truth_lob.txt`, `truth_ru.txt`
  (R_CLOB lines); the SQL of `corpus/sql/01b_optional_tables.sql`, `13_optional.sql`, `07_ru_objects.sql`
  for the full text of the values.
- Done when:
  (a) T_LOB: C = `hello lob`, NULL, `""`, 3,000 x `m`, the 100,000-character text of the SQL
  (`chunk001` ...); B = the 10 bytes of the truth, NULL, `b""`, 1,000 x `ab`, 100,000 x `cd`; lengths =
  `truth_lob.txt` (characters for C, bytes for B) and the first 30 bytes of B = its hex; T_LOB2 A, V, N,
  B = the SQL (N of row 2 empty, not NULL); both versions, both files of each version give equal
  batches [L6, R7]; T_FRAME after them = `truth_frame.txt`;
  (b) R_CLOB in all 7 files: row 1 = the 6 database bytes of `truth_ru.txt` decoded with CL8MSWIN1251,
  row 2 = 6,000 characters whose first 60 bytes equal the truth, row 3 NULL; the same with every client
  set [L5, C1]; R_TEXT and the Cyrillic-named table after R_CLOB in `ru_exp_*` equal their truth;
  (c) chunk boundaries: a test feeds the UTF-16 decoder the 200,000-byte value of T_LOB row 5 in the
  file's chunks and in chunks of 1, 3 and 16,119 bytes and gets the same text; a synthetic surrogate
  pair split between two chunks decodes to one character; an odd total and an unpaired surrogate give
  `E-BAD-TEXT`;
  (d) `batch_bytes` 4,000 on T_LOB gives batches of 4 and 1 rows, `batch_bytes` 1 gives 5 batches of 1
  row, the default gives 1 batch (arch 5.6 step 8); `max_lob_bytes` 150,000 gives `E-LOB-TOO-LARGE` at
  row 5 of T_LOB (C, 200,000 stored bytes; B has 100,000 and is read), and with
  `on_value_error="null"` row 5 has C NULL with `W-VALUE-NULLED` and B intact, while T_LOB2 and T_FRAME
  still read; `columns=["ID"]` reads T_LOB without
  holding any LOB byte (peak allocation below 64 KiB beyond the I/O buffer);
  (e) (session 4, replaces the `W-LOB-CHARSET` test) ADR-25: a synthetic long-form value in an R_CLOB
  column gives `E-LOB-UNCONFIRMED` (row, column, offset; with `on_value_error="null"` NULL and
  `W-VALUE-NULLED`, R_TEXT after it still reads); with `unconfirmed_lob="decode"` it decodes as
  CL8MSWIN1251 with `W-LOB-UNTESTED`; a synthetic NCLOB in a CL8MSWIN1251 header gives the same pair;
  the 199 fixtures give none of these;
  (f) (session 4) surrogate pairs: the vectors of arch 5.6 step 6 (`nchar2_exp_<v>` CL and NCL,
  `lob3_exp_<v>` row 20 C and N = UTF-8 `d0 b4 d0 b6 20 43 61 66 c3 a9 20 61 f0 9f 98 80`), the split
  feeds and the negative vectors; `lob3_exp_<v>` rows 1 to 11: B = S bytes `5a`, C = S characters `x`
  for S of `truth_lob3.txt`, short up to one chunk and long beyond (arch 5.6 step 4), both versions.
- Needs: T-6 (done), T-5f. Can run in parallel with T-7 and T-8 (session 6: not with T-9).

## T-7 Owner mode - DONE (2026-10-17)
- Session 7 record: 5 tests in `t7_owner.rs`, no production change; owner mode reads through the table-mode
  code [T8] (ADR-36 item 1).
- Goal: all owner-mode files (`RUSERS`): sections behind the `METRICSU` / `METRICEU` pairs, trailer
  groups, `ENDTABLE` not paired [T1, T2, T8, chapter 10].
- Inputs: `own_exp_*`, `own_tables_exp_*`, `truth_own.txt`; `ru_exp_*` (session 3: read completely, no
  stop; their R_CLOB values are T-6L's).
- Done when: O_PARENT, O_CHILD, O_EMPTY equal `truth_own.txt` in `own` and `own_tables` files and the
  batches of the two are equal [T8]; the `stc` and `ddl` families pass through the same code.
- Session 6: all 199 files already walk (T-3c), so T-7 is mostly tests through `read_table`. Touches:
  new `tests/t7_owner.rs`; `exp/items.rs` only if a test fails (with a note). Needs: T-6 (done). Runs in
  parallel with T-5f, T-6L, T-8.

## T-8 Direct path - DONE (2026-10-17)
- Session 7 record: 8 tests in `t8_direct.rs`, no production change; 80 pairs equal [R7]; the empty-table
  decision ignores the version line [E3]; the synthetic ambiguous case is `Framing` at the LF;
  `T_LONG_EMPTY` is only in `long_exp_direct_<v>` (ADR-36). The `strict` side of [E2a] stays T-11's.
- Goal: flag 1 [T6, R4, R7] and both empty-table forms [E1 to E3] with the test of arch 5.4.
- Inputs: every `*_exp_direct_*` file, both versions; `t_empty`, `multi_empty*`, `own`, `stat` direct.
- Done when: every conventional / direct pair gives equal multisets of rows; empty tables give 0 rows on
  18c (no `ff ff`) and 11g (`ff ff`) with the same code path decision and no use of the version line; a
  synthetic ambiguous case gives `Framing`. Session 2: the decision itself is already in `exp::rows`
  (N-5); left here are the pair comparison, the synthetic case, and a synthetic direct-path table with a
  LONG first column (session 3: [E2a] as ADR-22, its three outcomes by default and the session 2 rule
  under `strict`; the walker part is T-3a, this task tests it through `read_table`). Session 3: T_PART
  conventional against direct give equal multisets per part; `lob_exp_direct_<v>` against `lob_exp_<v>`
  equal batches (flag 0 in both [L6]). Session 4: `long_exp_<v>` against `long_exp_direct_<v>`,
  `long2_exp_<v>` against `long2_exp_direct_<v>`, `t_probe11` and `part2` pairs give equal batches in
  declared column order (ADR-24; the direct files move L last and chain it); the synthetic LONG first
  column is now a **single-column** LONG table (ADR-22 session 4).
- Session 6: pair comparisons through `read_table`; `part2` pairs give 4 batches each (21, 29, 21, 29),
  T_PART pairs 3 (1,095, 1,098, 807). Touches: new `tests/t8_direct.rs`; `exp/rows.rs` only if a test
  fails (with a note). The `strict` side of [E2a] is T-11's. Needs: T-6 (done). Runs in parallel with
  T-5f, T-6L, T-7.

## T-9 Character sets - DONE (2026-10-17, merged)
- Session 8 record: commit `2f7f850`, merged `6128ff5`; `t9_charset.rs`; choices in N-21. ADR-25's use of the
  `db_charset` override is amended by ADR-25 session 8 (decision by the header's database set); T-6M builds it.
- Goal: complete `charset` (arch 6): three sets by field kind, five ids, unsupported id, overrides,
  `trim_char`, damaged-text flags.
- Inputs: C1 to C7, X3, X4; `t_text` x 3 clients x 2 paths, `ru_tbl_*` x 4 clients x 2 paths,
  `own_exp_w1252`, `nchar_exp_<v>`, `truth_text.txt`, `truth_ru.txt`, `truth_charsets.txt`,
  `truth_nchar.txt`, `truth_csmap.txt`; arch 6.2, 6.3; ADR-5, ADR-16.
- Done when: V, C, NV of `t_text` and all six columns of R_TEXT (309 rows) equal the truth for every
  client set; NC, CC, NV of T_NCHAR equal `truth_nchar.txt` (padding kept; with `trim_char` `ab` and
  U+0434 U+0436 for NC and CC; no `W-LONGER-THAN-DECLARED` for CC, descriptor 16); names and DDL are
  decoded with the client set and flagged (session 9: the words "`own_exp_w1252`: `ddl_lossy`" are dropped,
  ADR-40; the flag is false there by arch 6.5), and `possibly_incomplete` is
  set as arch 6.5 defines it (N-7); **the map test**: each of the 768 lines of `truth_csmap.txt` (both
  folders; identical) is compared with our decoder for its set: for CL8MSWIN1251 and WE8MSWIN1252 the
  `encoding_rs` result must equal the truth code unit in all 512 lines with `had_errors` false (one
  difference: ADR-5 fallback table); for US7ASCII the 128 lines 0x00 to 0x7F equal, and the 128 lines
  0x80 to 0xFF give a value error in row data and U+FFFD with the flag in a name (ADR-16); a patched
  descriptor id gives a Binary column and `W-CHARSET-UNSUPPORTED`; malformed bytes follow
  `on_value_error`; `db_charset` reads a patched high byte of a US7ASCII column. Session 4: NC, NV of
  `truth_nchar2.txt` (surrogate pair, `NCHAR(3)` not padded), CC with `trim_char` = `a` + U+1F600; the 7
  columns of `t_probe11` (both paths) = `truth_probe11.txt` (`ROW` lines: values; Y, Z byte lengths 500 /
  506 and 501 / 507); no `W-LONGER-THAN-DECLARED` anywhere (ADR-26).
- Session 6 (ADR-33 items 2, 8): `trim_char` is a `ReadOptions` field, `db_charset` / `client_charset`
  `Options` fields; an unsupported set id of a type 1 / 96 column becomes a Binary column with
  `W-CHARSET-UNSUPPORTED` (replaces T-6's interim `E-CONFIG`); partition and subpartition names decoded
  with the client set [C2] with a damaged flag (ADR-32 item 6). Touches: `charset.rs`, `reader.rs`,
  `dump.rs`, `types/map.rs`. Needs: T-6L (shares `charset.rs`, `reader.rs`). Runs in parallel with T-7,
  T-8 if they are still open.
- Session 7: (a) `W-CHARSET-MISMATCH` also for a form-1 CLOB (type 112) whose descriptor set differs from
  the database set of [H4] (arch 6.1; ADR-34 item 6); test: a patched copy of `lob_exp_18c` with the
  header database set changed to 171 gives the warning once for T_LOB.C, reads the short values as
  UTF-16 BE [L5] and refuses the long one (row 5) with `E-LOB-UNCONFIRMED`; (b) `#[doc(hidden)]` and the
  "diagnostic, no API promise" doc comment on `TableBatches::lob_bytes_read()` / `lob_values_skipped()`
  (ADR-34 item 2). Touches (session 7, complete): `charset.rs`, `reader.rs`, `dump.rs`, `types/map.rs`,
  `exp/header.rs` only if the set-id overrides need it, new `tests/t9_charset.rs`, `tests/common/`
  (helpers only). Needs: nothing open. **Runs in parallel with T-10** (separate worktree: both may add
  helpers to `tests/common/`).

## T-10 Remaining types and the full truth check - DONE (2026-10-17, merged; leftovers in T-10b)
- Session 8 record: commit `e08535e`, merged `0f0e4f7`; `t10_truth.rs` (29 checkers, 18 delegated families);
  choices in N-22. Two of its tests fail on the 201-file corpus (T-3d); the step (2) leftovers are T-10b.
- Goal: TSTZ, TSLTZ, RAW, the checks of arch 5.4, and arch 8.2 (b) for every truth file.
- Inputs: Y6 to Y9; `t_tstz`, `t_raw`, `t_len`, `t_nullmix`, `t_wide`, `t_frame`, `t_probe6`,
  `t_bn1` to `t_bn4`, `multi_*`, `ddl`; session 2: `long_exp_<v>` (values from `corpus/sql/11_types.sql`),
  `nchar_exp_<v>` (`truth_nchar.txt`), `t_decl_exp_<v>` (`truth_decl.txt` for the declared types, the
  row of `11_types.sql` for the values, the ` MODIFY DEFAULT` item skipped), `t_num2_exp_<v>`
  (`truth_num2.txt`); session 3: `lob_exp*` (`truth_lob.txt`, T-6L), `misc_exp_<v>` (`truth_misc.txt`),
  `part_exp*_18c` (`truth_part.txt`); session 4: `t_num3` (`truth_num3.txt`), `nchar2`
  (`truth_nchar2.txt`), `lob3` (`truth_lob3.txt`), `long2` x 3 and `long_exp_direct` (`truth_long2.txt`,
  `11_types.sql`), `misc2` (`truth_misc2.txt`), `part2*_18c` (`truth_part2.txt`: each `ROW` id in the part
  of its subpartition), `t_probe11` x 2 (`truth_probe11.txt`).
- Done when: the value-level truth test passes for all **199** files with the expected warnings and
  errors listed in the test (TSLTZ warning; session 4: no `W-UNVERIFIED-TYPE` for URID; `E-BAD-INTERVAL`
  for the two YEAR(9) extremes of `misc2`; **no stop**; the
  `truth_num` rows that cannot fit, run with `on_number_overflow="error"` and once with `round`); T_PART
  gives the `P|` lines of `truth_part.txt` per part and its `TOTAL` line (3,000 rows, the sum, minimum
  and maximum it states) over the whole table; no file is skipped without a line in that list.
- Session 5 (ADR-28): the list of expected value errors holds `E-BAD-DATETIME` for `t_date_exp*` rows 8
  and 9 (every `t_date` EXP file) and `E-BAD-INTERVAL` for `misc2_exp_<v>` IYM rows 2, 3; each such table is
  read once with `raise` (stops at the first) and once with `null` (NULLs counted as `W-VALUE-NULLED`).
- Needs: T-6L, T-7, T-8, T-9.
- Session 7: T-6L, T-7, T-8 are done, so T-10 **starts now, beside T-9**, in two steps: (1) every truth file
  whose columns do not depend on T-9 (among them `truth_num*`, `truth_date`, `truth_ts`, `truth_tstz`, `truth_raw`,
  `truth_len`, `truth_nullmix`, `truth_wide`, `truth_frame`, `truth_lob*`, `truth_misc*`, `truth_part*`,
  `truth_long2`, `truth_decl`, `truth_own`, `truth_stat`, the LONG tables against `11_types.sql`);
  (2) after T-9 is merged: `truth_text`, `truth_ru`, `truth_nchar`, `truth_nchar2`, `truth_probe11`,
  `truth_charsets` (the map test `truth_csmap` is T-9's). `T_LONG_EMPTY` (0 rows) is expected in
  `long_exp_direct_<v>` only (ADR-36 item 4). The expected-warning list includes one `W-TSLTZ` per TSLTZ
  column (`t_tstz`) in `Dump::warnings()` and none in the reads (ADR-35 item 4). The list of expected
  errors and warnings is written as data the T-11 `strict` test can reuse. Touches: new
  `tests/t10_truth.rs`, `tests/common/` (helpers); production code only if a test fails (with a note, and
  then coordinate with the open T-9 / T-11 worktree). Needs: T-9 for step (2) only.

## T-6M LOB storage of database set 171 confirmed (R-038; session 8, small) - DONE (2026-10-17, `a937f2f`; N-23, ADR-40)
- Goal: ADR-25 session 8 blocks and ADR-37: a CLOB / NCLOB of a CL8MSWIN1251 database decodes in both forms
  without error or warning; every other database set except 873 keeps the stop; one list decides.
- Inputs: [L4, L5 v3.5], `exp.md` 14.6 "Session 10" (offsets below are of `ru_lob2_exp_cp1251_11g.dmp`; the
  US7ASCII file has the same LOB bytes at the same offsets); `ru_lob2_exp_cp1251_11g`, `ru_lob2_exp_us7ascii_11g`;
  `truth_lob_ru.txt` (lengths, head / tail DUMPs), `corpus/sql/15_ru_lob2.sql` (the full values), `truth_ru.txt`.
- Done when:
  (a) `charset.rs` has `pub(crate) const LOB_CONFIRMED_DB_SETS: &[u16] = &[873, 171];` (doc comment: [L5 v3.5],
  R-038, the 11g-only evidence, ADR-25 session 8); `lob_text_kind` takes the header's database set id D
  instead of `db_single_byte` and marks: NCLOB unconfirmed iff D is not in the list; CLOB of set S (873, 171,
  178, 1) unconfirmed in the long form iff D is not in the list or S != D; the `db_charset` override changes the
  decoder, not this decision; no other code names 171 or 873 for it; `src/` contains no chunk-size literal
  (8060, 8132, 16120, 16264; none today) [ADR-37];
  (b) new test **`r_lob2_decodes_to_the_truth_in_both_client_files`** (`t6l_ru.rs`; T-3d delegates R_LOB2 to it):
  both files, default `ReadOptions`, R_LOB2 rows by ID: C row 1 = `blk` = the 66 characters U+0410 to U+044F,
  U+0401, U+0451 (the cp1251 reading of `c0`..`ff`, `a8`, `b8`, as the SQL builds it); row 2 = `blk` x 303 +
  U+0410 U+0411 = 20,000 characters (303 x 66 = 19,998 + 2), last four U+0401 U+0451 U+0410 U+0411 (truth tail
  `a8,b8,c0,c1`); row 3 NULL. N = the same two texts and NULL (truth GETLENGTH 66 / 20,000; head
  `4,10,4,11,4,12,4,13`, tail `4,1,4,51,4,10,4,11`). B = the cp1251 bytes of C: 66 and 20,000 bytes, NULL.
  Expectations are built from the truth file and the SQL rule, not from the reader. The read gives **no
  warning** (no `W-LOB-UNTESTED`, `W-CHARSET-MISMATCH`, `W-VALUE-NULLED`) and `Dump::warnings()` has none for
  R_LOB2; the two files give equal batches; `unconfirmed_lob="decode"` gives the same batches and still no
  warning; R_TEXT after R_LOB2 has 309 rows equal to `truth_ru.txt` by ID [T8];
  (c) `a_long_form_clob_of_a_single_byte_database_is_unconfirmed` becomes
  `a_long_form_clob_of_database_set_171_decodes`: the rewritten R_CLOB row 2 (two chunks of 3,000 bytes) reads
  as the short-form text with no warning, default and `decode` alike;
  `an_nclob_in_a_single_byte_header_is_unconfirmed` (`lob_exp_18c`, header D 873 -> 171) now expects T_LOB2.N
  to decode to the text of the unpatched file without warning, while T_LOB.C row 5 (S = 873 != D) stays
  `E-LOB-UNCONFIRMED` (T-9's `W-CHARSET-MISMATCH` test on the same patch passes unchanged);
  (d) new stop test **with another single-byte set**: a copy of `ru_lob2_exp_cp1251_11g.dmp` below `target/`
  with the header database set @0x36 `00 ab` -> `00 b2` (178) and the CLOB descriptor set @0x24e3 `ab 00` ->
  `b2 00`. Default: `E-LOB-UNCONFIRMED` at R_LOB2 row 1, column 3 (N), offset 0x2790, length 132 (C row 1, short,
  is read before it); the message names 178, the confirmed list and `unconfirmed_lob`, not R-038. With
  `on_value_error="null"`: C = [the windows-1252 text of the 66 bytes, NULL, NULL], N = [NULL, NULL, NULL], B
  intact, `W-VALUE-NULLED` C count 1 (row 2, @0x29c4) and N count 2; R_TEXT still reads 309 rows (its 171
  columns warn `W-CHARSET-MISMATCH` as T-9 defines; asserted, not compared). With `decode`: C row 2 = the
  windows-1252 text of the 20,000 bytes, N = the Cyrillic texts of (b), `W-LOB-UNTESTED` C count 1, N count 2.
  The same first error with the header patched to 1 (descriptor left at 171) and to 31 (an id outside the
  five); with `db_charset="CL8MSWIN1251"` on the 178 copy the error stays (the override does not lift);
  (e) the chunk sizes asserted in `t3a_walker.rs` and `t6l_lob.rs` for `lob_exp_<v>` carry a comment
  "observed in this AL32UTF8 fixture, not a rule (ADR-37)"; no assertion changes;
  (f) `cargo fmt`, `clippy -D warnings` clean; `cargo nextest run`: every test passes **except the 11 of
  T-3d's list**, which fail as before and for the same reason (this task is allowed to land on the red
  master; it must not add a failure).
- Touches: `charset.rs`, `reader.rs` (`lob_text_kind`, where `db_single_byte` is computed in
  `TableBatches::new`, the message of `E-LOB-UNCONFIRMED`), `tests/t6l_ru.rs`, comments in `tests/t3a_walker.rs`
  and `tests/t6l_lob.rs`. No API, option, error or warning code is added or removed (ADR-34 item 9).
- Needs: T-9 merged (done: T-9 and T-6M both touch `reader.rs` and `charset.rs`). Runs **before** T-3d (whose
  reads of `ru_lob2_*` must decode) and **not beside** T-11 (`reader.rs`).

## T-3d Corpus 201 (session 8) - DONE (2026-10-17, `7211137`; N-24, ADR-39, ADR-40; reviewed: review-01)
- Goal: every test that compares the corpus with a fixed list follows the 201 EXP files; master green again.
  The 11 failing tests: `t10_truth` `the_199_files_are_the_listed_ones`,
  `every_table_of_the_199_files_is_classified_and_read`; `t1_walk::table_covers_every_exp_fixture`;
  `t2_manifest` `x_walker_with_hashing_reproduces_every_line_of_the_manifest`,
  `x_fixture_checksums_match_the_three_sha256sums_files`; `t3_catalog` four `x_...` tests (the reader finds
  R_LOB2 and R_TEXT where the manifest has nothing); `t5_types` `every_table_of_199_files_...`,
  `all_382_create_table_lines_...`.
- Done when:
  (a) **regenerated, not edited by hand**, by the generators that import the Analyst's tools from
  `docs/formats/tools/` (each docstring then says 201 files, 386 sections, 400 parts):
  `crates/oradump-core/tests/data/gen_walk_expected.py` -> `walk_expected.tsv` (201 lines), `walk_parts.tsv`
  (400), `lob_expected.tsv` (+ the R_LOB2 parts of the two new files); `gen_number_parts.py` ->
  `value_parts.tsv` (400 parts, 86,080 NUMBER fields); `tests/golden/gen_exp_raw.py` -> `exp_raw.tsv` (the
  manifest, 400 parts); `tests/golden/gen_exp_catalog.py` -> `exp_catalog.tsv` (386 lines). The Analyst ran the
  tools in session 10 but committed no output file: the files to use are these generator outputs, and the
  Analyst's printed totals are the check (rules v3.5 "Changes in session 10"; `exp.md` 12): `corpus_counts.py`
  (the 201-file table above), `check_all.py` (parts 184 / 164 / 52; fields 22,979 / 10,537 / 170,109),
  `rowlen_stats.py` (126 LOB values, 80 short, 46 long), `num_decode.py` (86,080), `collist.py` (the per-form
  counts of the `CREATE TABLE` lines, 386 lines). The checksum expectation is the three `SHA256SUMS`: 317
  lines, 235 `.dmp`. The Test engineer reviews the regenerated files in `tests/golden/`, as in T-3b;
  (b) counts in tests are **derived from the golden files** (line counts and sums of the TSVs), and the
  Analyst's totals stand once, in `tests/common/` (one constant table citing rules v3.5), asserted against
  the TSVs in one test. Exception: the checksum guard keeps explicit numbers (317, 235), because its purpose is
  to notice a file that arrives without a design update. Test names lose their numbers:
  `the_exp_files_are_the_listed_ones`, `every_table_of_the_exp_files_is_classified_and_read`,
  `every_table_agrees_with_its_descriptors_and_insert_names_without_fallback`,
  `all_create_table_lines_parse_with_the_per_form_counts_of_collist_py`, and the same in `t1_walk.rs`
  (`tool_table_reproduces_the_plan_numbers...`), `t3a_walker.rs` (`x_every_binary_part_of_the_199_files...`,
  `x_the_199_fixtures_give_no_stop...`) and `t6_value_parts.rs` (396 / 199 / 85,456), which pass today only
  because they read the old TSVs. T-10's registry follows the renames (`delegated_tests_exist`). The T-10
  classification covers `ru_lob2_*`: R_LOB2 delegated to `t6l_ru.rs::r_lob2_decodes_to_the_truth_in_both_client_files`
  (T-6M; truth `truth_lob_ru.txt` + `15_ru_lob2.sql`), `EXPECTED_DELEGATED` R_LOB2 2, R_TEXT 25 -> 27; the
  warning inventory unchanged (the two files add none: asserted);
  (c) `truth_lob_ru.txt` (11g-cp1251) and `truth_r045.txt` (18c; R-045, the SQL*Plus spool: ORA-14085 for
  `t_plong`, `t_plongraw` and `ALTER TABLE t_plong2 ADD (l LONG)`, `Table created.` / `Table dropped.` for the
  rest) stand in every list of truth files: the module comment of `t10_truth.rs` ("truth files with no EXP
  table" gets `truth_r045.txt`), arch 8.2 (done), and a new test **`every_truth_file_is_read_or_listed`**: the
  set of `truth_*.txt` names in the three `SHA256SUMS` equals the files opened by a checker or a delegated
  test plus that explicit list;
  (d) the T-10 step (2) leftovers of N-22 are **not** here: task T-10b;
  (e) the log check (`x_row_counts_of_the_catalog_equal_the_rows_exported_lines_of_the_logs`) covers 201 logs:
  R_LOB2 3 and R_TEXT 309 in both new logs;
  (f) `cargo nextest run` green (0 failing), fmt and clippy clean.
- **The reads of `ru_lob2_*` decode**: T-3d runs after T-6M, so R_LOB2 reads to the truth. Before T-6M, the
  T-10 read would stop with `E-LOB-UNCONFIRMED` at R_LOB2 row 1 N (@0x2790); the walk, catalog and manifest
  tests (t1, t2, t3, t3a, t5, t6_value_parts) never decode a LOB value and do not depend on T-6M.
- Touches: `tests/golden/*` (regenerated), `crates/oradump-core/tests/data/*` (regenerated), `tests/t1_walk.rs`,
  `t2_manifest.rs`, `t3_catalog.rs`, `t3a_walker.rs`, `t5_types.rs`, `t6_value_parts.rs`, `t10_truth.rs`,
  `tests/common/`. No production code; a failing test that points at the reader is a note, not a fix here.
- Needs: T-6M merged. **Not beside T-11** (both touch the tests that read the corpus).

## T-10b T-10 step (2) leftovers and the first review's findings (N-22; session 8; widened in session 9, ADR-38) - DONE (2026-10-17, `7941e40`; reviewed: review-02; choices answered by ADR-44)
- Session 10 record: 371 tests at `7941e40`; every mutation of (b9) fails its named test (review 02 section 1);
  the thin-wrapper gap (e3 / e4) and the review's notes go to T-11b.
- Goal: every truth file of every folder is compared with the reader's values by a check that is shown to
  fail; the truth-file test credits only what a check opened.
- Inputs: `docs/review/review-01-golden.md` (S-1 to S-6, section 5 mutations, notes N-1 to N-4, N-8); ADR-38;
  `corpus/sql/01_corpus_tables.sql` (T_LEN), `08_ru_stats_objects.sql` (R_STAT), `10_stc.sql` (T_STC),
  `14_long2.sql`, `14_misc2.sql`; the truth files named below.
- May take two sessions: step (a) is committed on its own, then step (b).
- Done when, step (a) (mechanism):
  (a1) every truth file is opened through `common::truth` (or a raw-text sibling in `tests/common/`), which
  records `(folder, name)` in a process-wide set; no test opens a `truth_*.txt` another way (the T-10b note
  shows the grep);
  (a2) each truth comparison of a delegated family is a function in `tests/common/` that returns the list of
  differences (empty = equal); the delegated test calls it and asserts the list is empty. Reason: each
  `tests/*.rs` is its own binary and nextest runs each test in its own process, so the truth-file test can only
  see what it calls itself;
  (a3) `every_truth_file_is_read_or_listed` calls every checker of `t10_truth.rs` and every such `common`
  function, then compares the recorded `(folder, name)` set plus the explicit list (keyed by folder too, each
  entry with a reason) with the `truth_*.txt` lines of the three `SHA256SUMS`; a source literal no longer counts;
  T_PART2's entry points to `t6_parts.rs::t_part2_gives_100_rows_and_each_part_holds_its_own_rows` (S-1);
  (a4) `EXPECTED_CHECKED` / `EXPECTED_DELEGATED` are computed from `tests/golden/exp_catalog.tsv` with the keying
  of `delegated()` (S-6);
  (a5) comments and docs: `gen_walk_expected.py:183-185` (80 / 46 / 126, 21 sections; outputs byte-identical
  after the change), `FolderTotals.lob_sections` (the 112 count), `t10_truth.rs:1107` (`EXP-00056` and
  `EXP-00008`), `column_definitions` = 1,907 in `TOTALS` used by `t5_types.rs`, N-23's count 257.
- Done when, step (b) (checks):
  (b1) R_TEXT: own checker against `truth_ru.txt` by ID in **all 27** sections (the 10 of `ru_stat_*` included);
  (b2) **R_STAT** checker: in each of the 10 sections the row count equals `TAB|R_STAT|<n>` of
  `11g-cp1251/truth_stat.txt`, each column's NULL count and number of distinct values equal its `COL` line
  (gathered with `estimate_percent => 100`), and the values equal the rule of `08_ru_stats_objects.sql` by ID
  (TXT included in every client set, as R_TEXT equals `truth_ru.txt` in every set; a disagreement is a note,
  never an adapted expectation);
  (b3) **T_LONG2** checker in every file the T-6 test reads: length and hex of each `<id>|<len>|<hex>` line of
  `truth_long2.txt` of the folder (`INSERT REFUSED` = no such row); `truth_long2.txt` leaves the explicit list;
  (b4) **T_INTERVAL2 / T_ROWID2** checker for `misc2_exp_<v>` of both folders, every row and column, against
  `truth_misc2.txt` of the folder (IYM rows 2, 3 read with `on_value_error="null"` as T-10 does, ADR-28);
  `truth_misc2.txt` leaves the explicit list;
  (b5) **T_STC**: each count from `truth_stc.txt` of the folder, T_STC4 from `TAB|T_STC4|50` of
  `truth_stc_extra.txt`; values by the rule of `10_stc.sql` (ADR-38 item 3);
  (b6) `check_len` compares the full value with `RPAD(SUBSTR('L' || n || '_', 1, n), n, 'x')` (S-5);
  (b7) the session 8 items: T_TEXT, T_NCHAR, T_NCHAR2, T_PROBE11 with the T-9 reader (padding kept; `trim_char`
  once for T_NCHAR); a `truth_charsets.txt` checker (each `NAME=id` line of both folders equals the reader's
  id-to-name map; 171 = CL8MSWIN1251 from the header of every `11g-cp1251` file [X3, H4]);
  (b8) **one break test per new or changed checker** (on a patched copy below `target/` or a mutated truth line
  in memory: exactly one difference reported);
  (b9) **the review's mutations**, each applied alone to a scratch copy of the repository (never to
  `corpus/fixtures/`), make the named test fail, and the T-10b note lists the run:
  a3 (`11g/misc2_exp_11g.dmp` @0x24cf `33` -> `34`, T_ROWID2 row 1) -> the T_INTERVAL2 / T_ROWID2 checker;
  b4 (`18c/truth_long2.txt` `...C3A9` -> `...C3A8`) -> the T_LONG2 checker;
  b5 (`11g-cp1251/truth_stat.txt` `TAB|R_STAT|2000` -> `2001`) -> the R_STAT checker;
  b6 (`18c/truth_stc.txt` `TAB|T_STC1|50` -> `51`) -> the T_STC test;
  a2 (`11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp` @0x227b5 `da` -> `c2`) -> the R_TEXT checker;
  a4 (`18c/multi_framing_exp_18c.dmp` @0x24ad one `x` -> `7`) -> the T_LEN checker;
  e (`check_wide` compares nothing) -> `every_table_of_the_exp_files_is_classified_and_read` or the
  T_WIDE break test;
  besides each, the checksum guard fails as before (expected, not counted);
  (b10) green: fmt, clippy, nextest.
- Touches: `tests/t10_truth.rs`, `tests/common/` (new `truth_checks` module allowed), the delegated tests whose
  truth comparison moves into `common` (`t6_read.rs`, `t6_parts.rs`, `t6l_lob.rs`, `t6l_ru.rs`, `t7_owner.rs`,
  `t9_charset.rs`, `t5_types.rs`, `t4_values.rs`: assertions move, none is weakened), the comment of
  `tests/data/gen_walk_expected.py`. No production code; a reader bug found here is a note.
- Needs: T-3d, T-11 merged (both done). Runs beside **T-9b** (separate worktree; T-9b touches `reader.rs` and its
  own new test file only).

## T-9b `W-CHARSET-MISMATCH` for form-1 CLOB; `W-LONG-LENGTH` offset of a chain (session 9, small; ADR-41 items 5, 6) - DONE (2026-10-17, `e308d9a`; reviewed: review-02)
- Session 10 record: 352 tests at `e308d9a`; mutations r1 to r3 fail `t9b_fixes.rs` (review 02 section 4).
  `E-LOB-TOO-LARGE` on a chain moved to the first piece (accepted, ADR-44 item 8). The other reports of the
  same value (`W-VALUE-NULLED`, `E-BAD-TEXT`) still differ: T-12 (ADR-43 item 3).
- Goal: the bug from T-9 (N-25) and the offset rule of ADR-41 item 6.
- Done when: (a) `reader.rs`: type 112 joins 1 and 96 in the `W-CHARSET-MISMATCH` condition (form not 2, no
  `db_charset`, descriptor id != header database id; decodable or not), nothing else changes there;
  (b) test on a copy of `lob_exp_18c.dmp` below `target/` with the header database set 873 -> 171 (the T-6M
  patch, offsets asserted before patching): T_LOB.C gets **one** `W-CHARSET-MISMATCH` (column = its file
  position), T_LOB2's form-1 CLOB A gets one, and every other mismatch warning is asserted per column
  (V's); short C values read as UTF-16 BE [L5], row 5 stays `E-LOB-UNCONFIRMED`; with `strict` the read
  of T_LOB fails with `E-STRICT`, `warning` = `W-CHARSET-MISMATCH`, before any row; with `db_charset` given,
  no mismatch warning;
  (c) `W-LONG-LENGTH` on a direct-path chain reports the first data byte of the **first** piece (ADR-41 item
  6): a copy of `long_exp_direct_18c.dmp` whose 32,000-byte chain gets one more byte (piece length and data
  patched, offsets asserted before patching) gives the warning with that offset, row 3, column of the LONG;
  the conventional case (`t11_codes.rs::w_long_length`) is unchanged; if `E-LOB-TOO-LARGE` on a LONG chain
  reports another offset, it follows the same rule and the note says so;
  (d) the fixture-wide strict list of T-11 and the T-10 warning inventory pass unchanged (no fixture has a
  CLOB set different from its database set); (e) green.
- Touches: `reader.rs`; new `tests/t9b_fixes.rs` (existing helpers of `tests/common/` only, none added).
  `t6l_ru.rs::an_nclob_in_a_single_byte_header_is_unconfirmed` is not edited (it passes; (b) is the precise
  test). Needs: nothing open. Runs beside T-10b; merged before Reviewer session 2.

## T-11 Options, warnings, strictness - DONE (2026-10-17, `7db4667`; N-25, ADR-41)
- Session 9 record: `t11_codes.rs` (24 + 19 codes), `t11_strict.rs` (27), `t11_options.rs` (15), unit tests in
  `error.rs`; 347 tests. New codes `E-STRICT`, `E-UNSUPPORTED-CHARSET`; new public fields `Error.warning`,
  `Warning.max_digits_dropped`, `TypeOptions.number_overflow`, `Options.extra_intro_lines`; `survey::count_table`.
  Found the T-9 bug fixed by T-9b. The NUMBER profile of arch 5.2b is T-12's (ADR-41 item 4).
- Goal: arch 7 complete: warning codes, `strict`, `on_value_error`, `on_number_overflow` and
  `number_types` (ADR-14; default as the owner decides Q-1), `columns`, `extra_intro_lines`,
  `max_line_bytes`, `max_lob_bytes`; messages without values (no LOB byte either); `W-NUMBER-ROUNDED`
  counts as arch 5.2a; the session 3 codes of arch 7.1; session 4: `unconfirmed_lob` (ADR-25),
  `strict` connected to the walker's `strict_e2a` (ADR-23), the retired codes absent, `E-LOB-UNCONFIRMED`.
- Done when: one test per code of arch 1.4 and 7.1, each asserting code and offset; `strict=True` over
  the fixtures fails exactly on the documented list; a test greps every error and warning text produced
  from the fixtures for bytes of row values and finds none.
- Session 5: `strict` wired to every walker warning added in T-3c (piece of length 0, T = 0, the
  partition-order cases of ADR-21 / ADR-29, [E2a] via `strict_e2a`), to `W-UNVERIFIED-TYPE` and to
  `W-DDL-FALLBACK` (ADR-30 item 3 included); under `strict=True` a value `round` would change is
  `E-NUMBER-OVERFLOW` (owner, ADR-14), `null` unchanged; one test per case. The survey's counting core
  adds per column: years below 1 (DATE / TIMESTAMP family) and YEAR TO MONTH values beyond 32-bit months
  (ADR-28), declared `NUMBER` columns no Decimal128 holds (ADR-30).
- Session 6 (ADR-33 items 3, 5): (a) **`strict` for every warning outside the confirmed rules added since
  T-3c**, one test each: the walker's (piece of length 0, T = 0, the partition-order cases of ADR-21 /
  ADR-29, a later part's order of ADR-32 item 4, [E2a] through `strict_e2a`), the resolution's
  (`W-DDL-FALLBACK` incl. ADR-30 item 3, `W-TSLTZ`, `W-CHARSET-UNSUPPORTED` as `UnsupportedCharset`), the
  reader's (`W-UNVERIFIED-TYPE`, `W-LONG-LENGTH`, `W-LONG-CHARSET`, `W-LONGER-THAN-DECLARED`,
  `W-CHARSET-MISMATCH`, `W-LOB-UNTESTED`); `W-NUMBER-NULLED` and `W-VALUE-NULLED` do not respond to
  `strict` (they report a chosen policy, arch 7.1), and `round` becomes `E-NUMBER-OVERFLOW`; (b) the
  per-column overflow policy of a `number_types` entry (precedence: entry, then
  `ReadOptions.on_number_overflow`); test on `t_num_exp_18c` V with the global `round` and the entry
  `error` (stops at the first value with more than 10 fractional digits) and the reverse; (c)
  `Warning.max_digits_dropped` on `W-NUMBER-ROUNDED`: `t_num` V under Decimal128(38, 10) + `round` gives
  the largest fractional digits minus 10 over its rounded rows (the test computes the figure from the
  truth texts, never hard-codes it).
- Needs: T-10.
- Session 8 (ADR-25 session 8): (d) and (f) below use the **178 copy** of T-6M (d) (`ru_lob2_exp_cp1251_11g`
  with database set 178), because the synthetic long R_CLOB value of `t6l_ru.rs` is in a 171 database and is
  now confirmed: with `unconfirmed_lob="decode"` the copy gives `W-LOB-UNTESTED` for C (count 1) and N (count
  2) without `strict` and `E-LOB-UNCONFIRMED` at R_LOB2 row 1 N with it; the survey count is 0 on all 201
  fixtures and 3 on the copy (C: 1 long; N: 1 short, 1 long). The `ru_lob2_*` files give no `strict` error
  in R_LOB2. Needs: **T-3d merged** (not before; both touch the tests that read the corpus, and `reader.rs`
  with T-6M); runs beside T-10b.
- Session 7 (ADR-34 items 5, 8; ADR-35 item 4): (d) `W-LOB-UNTESTED` is a counted warning; under `strict=True`
  each such value is `E-LOB-UNCONFIRMED` (value class) whatever `unconfirmed_lob` says; test: the synthetic
  long-form R_CLOB value of `t6l_ru.rs` with `unconfirmed_lob="decode"` gives `W-LOB-UNTESTED` (count 1)
  without `strict` and `E-LOB-UNCONFIRMED` with it; (e) `W-TSLTZ` (already in list (a)) is tested on
  `t_tstz`: one warning per TSLTZ column without `strict`, a scan error with it; (f) the survey counting core counts per LOB column the values ADR-25 item 2
  refuses, by form (ADR-25 item 5, arch 5.6 step 9), 0 on every fixture, 1 on the synthetic value.
  Touches: `error.rs`, `reader.rs`, `dump.rs`, `exp::*` (walker warnings), `types/resolve.rs`, a new
  survey counting module, new `tests/t11_*.rs`. Needs: **T-9 merged** (shares `reader.rs`, `dump.rs`,
  `types`); may overlap T-10 in a separate worktree; the fixture-wide test "`strict=True` fails exactly on
  the documented list" waits for T-10's expected list (merged first).

## T-11b The second review's test fixes (session 10, small, tests only; ADR-43, ADR-44) - DONE (2026-10-17, `a19d873`; 376 tests; choices answered by ADR-41 item 9)
- Goal: the review-02 findings that need no production code: S-2, S-4, e3 / e4 and notes N-2, N-4 (in part), N-5,
  N-6, N-8, N-9.
- Inputs: `docs/review/review-02-t9b-t11.md`; ADR-43 items 2, 4, 5, 6; ADR-44 items 4, 5; arch 7.1;
  `tests/golden/exp_catalog.tsv`; `truth_date.txt`, `truth_ts.txt`, `truth_tstz.txt`, `truth_env.txt`,
  `truth_own_ddl.txt`, `truth_charsets.txt`; `corpus/sql/01b_optional_tables.sql`, `15_ru_lob2.sql`.
- Done when:
  (a) **S-2**: patched copies below `target/` (bytes asserted before patching): `18c/t_ts_exp_18c.dmp` row 2 of each
  of the three TIMESTAMP columns and `18c/t_tstz_exp_18c.dmp` row 2 of TSTZ and the first non-NULL TSLTZ value get
  `63 64` over their first two bytes (the -0100 pair of `truth_date.txt` row 8 [Y2, Y5]); `survey::count_table`
  gives `years_below_1` = 1 in each patched column and 0 in every other; the plain read stops with
  `E-BAD-DATETIME` at row 2 of the first patched column; mutation e1 fails this test (scratch copy);
  (b) **S-4**: `ARCH_7_1_STRICT` (one line per `WarningCode::ALL` entry, outcome under `strict`, citation) with a
  test that it covers `WarningCode::ALL` exactly and equals `strict_error()` for each code; the derivation test
  uses the table, never `strict_error()`; `DOCUMENTED` keeps (stage, table key, code, warning, reason) and takes
  each count from the `exp_catalog.tsv` sections of that key (keying of `delegated()`); no count typed in;
  mutation a1 fails the new table test;
  (c) **e3 / e4**: the checkers of `t10_truth.rs` return their difference lists; one named union function;
  `every_table_of_the_exp_files_is_classified_and_read` asserts it empty; a break test runs `check_table` for
  every tag of `checker_tag` with one truth line or reader cell changed in memory and expects a non-empty list for
  that tag; e4 (`11g/t_wide_exp_11g.dmp` @0x3804 `v115` -> `v116`, scratch copy) fails
  `every_table_of_the_exp_files_is_classified_and_read`;
  (d) **notes**: N-2 (`t4_values.rs:477` exactly 16 from the truth file, `:484-489` exact counts;
  `t3b_parts.rs:547-565` `lob_columns == 5` for `lob_exp_18c` and LOB statistics against `lob_expected.tsv`);
  N-4 (`t9_charset.rs:978` column 3, count 1; `:913-923` comment and assertion agree); N-5 (`t9b_fixes.rs:80-86`
  rows 1 to 3 against `01b_optional_tables.sql`); N-6 (R_LOB2.B from the rule of `15_ru_lob2.sql`; doc comment of
  `lob_diffs`); N-8 (`truth_env.txt` against the header set ids of every file of its folder, via
  `truth_charsets.txt`; the OBJECT TABLE lines of `truth_own_ddl.txt` against the catalog of `own_exp_<v>`; both
  leave `TRUTH_NOT_OPENED`); N-9 (T_STC break test takes its count from the truth line);
  (e) the T-11b note lists each mutation (e1, a1, e4) with the test that fails, and the new test count;
  (f) green: fmt, clippy, nextest.
- Touches: `tests/t10_truth.rs`, `tests/t11_strict.rs`, `tests/t11_options.rs`, `tests/common/truth_checks.rs`,
  `tests/common/mod.rs` (additive helpers only), `tests/t4_values.rs`, `tests/t3b_parts.rs`, `tests/t9_charset.rs`,
  `tests/t9b_fixes.rs`. **Not** `t6_patched.rs` (T-12's in this group). No production code: a reader bug found here
  is a note.
- Needs: nothing open. Runs beside T-12 (separate worktree; overlap only in `tests/common/mod.rs`, additive);
  merged first, T-12 rebases. Model: Sonnet.

## T-12 Python binding - DONE (2026-10-17, `a19d873`; 390 Rust tests, 58 pytest; choices answered by ADR-3, ADR-41 item 8, ADR-46, ADR-47)
- Goal: `oradump-py`, `pyproject.toml` (maturin), `python/oradump/` with stubs; API of arch 3.4;
  `python -m oradump survey` (arch 8.5) with the NUMBER profile of arch 5.2b, the LONG lengths, the LOB
  counts per column and the parts of partitioned tables; whether `pyarrow.parquet.write_table` accepts
  Interval(MonthDayNano), recorded under ADR-19. The
  survey's counting core (profile per column, no value) is Rust and can be built and tested in T-11
  before the binding; only its command line waits for Q-3.
- Inputs: ADR-3, ADR-13; PyO3 guide, `pyo3-arrow` documentation.
- Done when: `maturin develop` builds on the VM; `ruff`, `mypy`, `pytest` pass; `pytest` reads three
  fixtures through `BatchReader`, through `pyarrow.RecordBatchReader.from_stream`, and compares with the
  Rust results; typed exceptions carry `.code` and `.offset`; two threads reading two tables finish in
  clearly less than twice the time of one (GIL released) or the measured result is recorded in
  `decisions.md`; `survey --no-names` output of a fixture contains no table name, column name or value;
  the NUMBER profile of `t_num_exp_18c.dmp` column V gives, under Decimal128(38, 10): 18 values that fit,
  4 that `round` changes (1 of them to 0: row 27), 10 whose integer part does not fit, largest `sig` 40,
  largest `frac` 125, exp10 from -125 to +125; session 4: the per-candidate counts of arch 5.2b
  (`t_num` V under Decimal256(76, 38): 27 exact, 2 changed by `round` of which 1 to 0, 3 integer parts
  too large), the
  `t_num3` V column flagged with 6 values of 39 / 40 significant digits, the LONG chain counts of
  `long_exp_direct_<v>` and the `E-LOB-UNCONFIRMED` count (0 on the fixtures).
- Session 5: Q-3 decided (Python 3.11 or later, `abi3-py311`, pyarrow 15 or later; ADR-13): T-12 does not
  wait. Add the pandas / pyarrow checks of ADR-28 (b) items 1 to 6, results recorded under ADR-28; the
  exception attributes `.part` and `.column` as a name (ADR-29).
- Session 6 (ADR-33 items 2, 3; arch 3.4): `open()` keeps every option, `read_table()` takes the per-read
  ones as overrides (`None` = the `open()` value) and builds one `ReadOptions` per call; `Dump.warnings` =
  the scan's warnings, `BatchReader.warnings` = the read's, counted ones with `.count`, `.count_to_zero`,
  `.max_digits_dropped`, issued through `warnings` when the iteration ends (StopIteration or error). Tests:
  `t_num_exp_18c` read with `on_number_overflow` overridden per call gives different results from one
  `Dump`; `pytest.warns(OradumpWarning)` sees `W-NUMBER-ROUNDED` at the end of the iteration and not
  before; `Dump.rows` is not exposed.
- Session 7 (ADR-34 item 4, ADR-35 item 4): `E-LOB-UNCONFIRMED` raises `ValueDecodeError` with `.code`
  `"E-LOB-UNCONFIRMED"`; `unconfirmed_lob` is an `open()` keyword and a `read_table()` override;
  `W-LOB-UNTESTED` arrives in `BatchReader.warnings` with `.count`; `W-TSLTZ` is one `Dump.warnings` entry
  per TSLTZ column, issued once when the scan completes and never by a read; every list entry is issued
  through `warnings` once. `lob_bytes_read` / `lob_values_skipped` are not bound. Tests: `t_tstz` gives
  one `pytest.warns` hit at `Dump.tables` and none during `read_table`; the synthetic R_CLOB value raises
  `ValueDecodeError` with the code. Session 8: that value is now confirmed; the test uses the 178 copy of T-6M
  (d) (`ValueDecodeError` at R_LOB2 row 1 N), and `ru_lob2_exp_cp1251_11g` R_LOB2 through pyarrow equals the
  Rust batches.
- Session 9 (ADR-40, ADR-41): (a) first Rust step: `Error.message` and `Warning.message` become
  `Cow<'static, str>`; `lob_unconfirmed_message` gives one text that names the header's database set id (test:
  the 178, 1 and 31 copies of T-6M (d) each name their id); (b) `Error.warning` is set on every error a strict
  conversion produced (ADR-41 item 1), unit-tested; (c) Python: `StrictError(OradumpError)` for `E-STRICT`,
  `UnsupportedCharset(UnsupportedError)` for `E-UNSUPPORTED-CHARSET`, `.warning` on `OradumpError` (a `W-...`
  string or None); the binding exposes `strict`, `extra_intro_lines`, the per-column policy (a
  `number_types` entry `("decimal(p,s)", "round")` fills both Rust maps), `.max_digits_dropped`; every
  `ErrorCode::ALL` / `WarningCode::ALL` entry has a Python mapping (a test walks both lists); (d) after a
  `StrictError` from `read_table` on one table, another table of the same `Dump` reads (ADR-41 item 2); the
  survey uses `survey::count_table`.
- Needs: T-11, T-9b, T-10b, and Reviewer session 2 without a blocking finding (ADR-42).
- Session 10 (ADR-43 items 1, 3; before the first formatted message, i.e. with step (a) of session 9):
  (e) **S-3, one offset per value**: in `reader.rs` every report about one value carries its start (conventional
  field: first data byte; LONG chain: `st.start`; LOB: the offset `lob_start` receives) under `raise` and `null`:
  the `W-VALUE-NULLED` of a chain or LOB value, `E-BAD-TEXT` inside a chain or LOB, `W-LONG-CHARSET` of a chain;
  for a value of more than one piece or chunk the message names the 1-based piece (chunk) where the failure was
  detected. New `tests/t12_offsets.rs::one_chain_value_reports_its_start_under_every_policy` on copies of
  `18c/long_exp_direct_18c.dmp` (T_LONG row 3; `9c 80` @0x2288, `19 9f` @0x2326 and @0x615c asserted before
  patching): `max_lob_bytes=20000` + `raise` -> `E-LOB-TOO-LARGE` @0x228a; + `null` -> `W-VALUE-NULLED` @0x228a, row 3;
  byte 0x2325 -> `c3` + `raise` -> `E-BAD-TEXT` @0x228a, "piece 2" in the message; + `null` -> `W-VALUE-NULLED`
  @0x228a; the same `raise` / `null` pair for a long-form CLOB value of `18c/lob_exp_18c.dmp` whose first chunk ends
  in `d8 00` (unpaired high surrogate [L5]): both at the `lob_start` offset, "chunk 2" in the message.
  `t6_patched.rs`: `a_broken_utf8_sequence_in_a_chain_is_a_value_error` asserts the value start; `:69-73` takes its
  expectation from `truth_checks::decode_cp1251` (review N-4). Arch 7.4 states the rule.
  (f) **S-1, no value in any message**: new `tests/t12_messages.rs` replaces the reach of
  `t11_options::no_error_or_warning_text_from_the_fixtures_holds_a_value` (the old test stays, unedited): over the
  messages (`message` field, not `Display`) of every error and warning of every fixture read under `raise` and
  `null`, plus the patched copy with a text-valued bad field (one byte inside a VARCHAR2 value of 8 or more ASCII
  bytes of an AL32UTF8 fixture -> 0xff: `E-BAD-TEXT`, `W-VALUE-NULLED`), search (a) raw field bytes, (b) runs of 4
  or more characters of each field's lossy UTF-8, (c) hex of each field of 2 or more bytes, both cases, as a token
  bounded by non-hex characters, (d) the rendered cells of the lenient read, 4 or more characters, digit-only
  strings shorter than 6 excluded; finds none. Self-test: a real fixture message with the value appended in each
  form (a) to (d) is reported. Mutations on a scratch copy, listed in the T-12 note: d1 (lossy bytes in the value
  error's message), d2 (hex), d3 (the patched copy's text in `E-BAD-TEXT`'s message) each fail `t12_messages.rs`.
  (g) Model: **Opus**. Touches (session 10 additions): `reader.rs`, `t6_patched.rs`, new `t12_offsets.rs`,
  `t12_messages.rs`, `tests/common/mod.rs` (additive); never a T-11b file (if the `Cow` change forces an edit in
  one, it is made after rebasing on T-11b).

## T-13 Python truth tests, `interval_as`, packaging - DONE (2026-10-17, `e2eaae6`; 395 Rust tests, 69 pytest; choices answered by ADR-49 to ADR-51; leftovers in T-13f)
- Goal: the value-level truth check again through pyarrow for all files (`tests/`); `interval_as` (ADR-47); wheel
  build (`abi3-py311`, `manylinux_2_28`), `README` with the type table and the list of limits; the licence work of
  ADR-45.
- History: session 6 (Q-6 needed), session 10 (ADR-45 items (a) to (e)), session 9 (pytest ownership). **The
  session 11 list below replaces all earlier "Done when" items of T-13.**
- Inputs: ADR-19, ADR-28 (b), ADR-45, ADR-46, ADR-47, ADR-48; arch 3.4, 5.1, 5.7 item 5, 8.5, 9; the T-12 note;
  `truth_misc.txt`, `truth_misc2.txt` (18c, 11g); maturin documentation ("Distribution"); Python Packaging User Guide
  (PEP 639 fields).
- **Done when (session 11):**
  (a) **`interval_as`** (ADR-47): Rust `Options.interval_as` (`Native` default | `String`); under `String` every
  type-182 and 183 column is Utf8 with `IntervalYm::to_text(p)` / `IntervalDs::to_text(p, f)`, p and f from [T9],
  fallback p = digits needed, f = 9 with one `W-DDL-FALLBACK` per column; field metadata `oradump:conversion` =
  `to-char-text`. New `tests/t13_interval.rs`: `misc_exp_<v>` T_INTERVAL and `misc2_exp_<v>` T_INTERVAL2 (18c and
  11g) read with `String` equal the `TO_CHAR` texts of `truth_misc.txt` / `truth_misc2.txt` cell by cell, including
  `misc2` rows 2 and 3 (no `E-BAD-INTERVAL`); the `Native` reads are unchanged (existing tests stay green); a patched
  copy with opposite signs (bytes asserted before patching) is still `E-BAD-INTERVAL` under `String`. Python:
  `open(..., interval_as=...)`, in `_core.pyi`; another value raises `InvalidOption`; pytest:
  `pyarrow.parquet.write_table` of `misc_exp_18c` T_INTERVAL writes and reads back equal with `"string"`, and raises
  `ArrowNotImplementedError` with `"native"` (the test names the pyarrow version it asserts).
  (b) **Python truth tests**: for every table of every `_exp_` fixture that the Rust truth check reads, the pyarrow
  table equals the Rust batches (`_core.digest` against the digest recomputed from pyarrow, `tests/conftest.py`), the
  expected failures being those of `t11_strict.rs` `DOCUMENTED` and T-10; one test per version folder.
  (c) **Metadata**: root `Cargo.toml` `[workspace.package] license = "MIT OR Apache-2.0"`, `license.workspace = true`
  in `oradump-core` and `oradump-py`; `pyproject.toml` `readme = "README.md"`, `license = "MIT OR Apache-2.0"`,
  `license-files = ["LICENSE-MIT", "LICENSE-APACHE"]`, no `License ::` classifier, `[build-system] requires =
  ["maturin==<version>"]` pinned exactly (ADR-48 item 2).
  (d) **Licence files**: `LICENSE-APACHE` verbatim from https://www.apache.org/licenses/LICENSE-2.0.txt; `LICENSE-MIT`
  the SPDX `MIT` text. **Q-8**: unless `owner-decisions.md` holds the copyright line when T-13 starts, the line is
  exactly `Copyright (c) 2026 [COPYRIGHT HOLDER - PENDING OWNER DECISION Q-8]`, and the T-13 note reports it as open;
  no release (0.1.0a1) is tagged while the placeholder stands.
  (e) **`README.md`**: what it reads (the "First release" paragraph below, in its words), install, a short example
  (`open`, `tables`, `read_table`, pyarrow), the type table of arch 5.1 with both `interval_as` modes, the limits,
  the pandas / Parquet facts of ADR-28 (b) and ADR-19 session 11 with the versions, `python -m oradump survey` with
  `--no-names` / `--no-read`, the "License" section and trademark line verbatim from ADR-45 item 3.
  (f) **`CONTRIBUTING.md`**: section "Licence of contributions" verbatim from ADR-45 item 4; development setup (`.venv`
  at the root, `maturin develop`, the checks of `CLAUDE.md`'s definition of done); a pointer to `CLAUDE.md`'s
  clean-room rule.
  (g) **`deny.toml`** with the allow-list of ADR-45 item 5; `cargo deny check licenses` green; a dependency outside
  the list is a note, not a new `allow` line.
  (h) **Wheel**: `maturin build --release --zig --compatibility manylinux_2_28` (flag spelling from `maturin build
  --help`; `ziglang` version in the note) gives `oradump-0.1.0-cp311-abi3-manylinux_2_28_x86_64.whl` (another tag if
  Q-9 is answered below 2.28: `manylinux2014`); `maturin sdist` gives the sdist. A script `tests/check_dist.py` (not
  collected by pytest) takes both paths and asserts ADR-48 items 3 and 4: `METADATA` has `Metadata-Version` >= 2.4,
  `License-Expression: MIT OR Apache-2.0`, both `License-File` lines, `Requires-Python: >=3.11`, `Requires-Dist:
  pyarrow>=15`, the README as description; both licence files under `.dist-info/licenses/`; the wheel holds only
  `oradump/` and `oradump-0.1.0.dist-info/`; neither archive holds `exp_survey.py`, a `.dmp`, a `truth_*.txt`, or
  anything under `corpus/`, `tests/`, `docs/`. If the pinned maturin writes no `License-Expression`, try the newest
  maturin 1.x, else stop and note it (no hand-edited metadata).
  (i) **Install**: the release wheel installed into fresh venvs of Python 3.11 and 3.14 with only pyarrow (>= 15)
  imports and reads a fixture; `pytest` passes against the installed wheel on both.
  (j) Green: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo nextest run`, `ruff check`,
  `ruff format --check`, `mypy`, `pytest`.
  (k) The T-13 note: test counts, the maturin and ziglang versions, the `cargo deny` result, the output of
  `tests/check_dist.py`, the `unzip -l` / `tar tzf` listings, the Q-8 placeholder if left.
- Touches: `crates/oradump-core` (options, type resolution, reader: `interval_as` only), new `t13_interval.rs`,
  `crates/oradump-py`, `python/oradump/`, pytest files and `tests/check_dist.py`, root files (`Cargo.toml`,
  `pyproject.toml`, `deny.toml`, `README.md`, `CONTRIBUTING.md`, `LICENSE-MIT`, `LICENSE-APACHE`). Not `corpus/`,
  `docs/`.
- Needs: T-12 (done). Q-8 only for the final `LICENSE-MIT` line; Q-9 only if the owner's hosts are older than glibc
  2.28. Model: Sonnet.

## T-13f The T-13 fix-up: licence check, version, README install, `.gitattributes` (session 12, small) - DONE (2026-10-17, `3fe33e7`; 395 Rust tests, 69 pytest; problems 1 and 2 of its note answered by ADR-54 items 7, 8, work in T-13g)
- Goal: the red `cargo deny` check green, the release version in place, the README honest about installation, Git
  kept away from the test data's line endings. No change under `crates/*/src`, `python/oradump/`, `corpus/`, `docs/`.
- Inputs: ADR-49, ADR-50, ADR-51, ADR-52; the T-13 note.
- **Done when:**
  (a) `deny.toml` has the `[graph] targets` block of ADR-49 (exact text) and its header comment names ADR-49;
  `cargo deny check licenses` passes; no `CC0-1.0` in `allow`, no `exceptions`.
  (b) Root `Cargo.toml` `[workspace.package] version = "0.1.0-alpha.1"`; `pyproject.toml` without `version`, with
  `dynamic = ["version"]`; `Cargo.lock` updated by Cargo (no other lock change: `git diff Cargo.lock` shows only the
  two workspace crates' versions). `crates/oradump-core/Cargo.toml` gets `publish = false`.
  (c) `tests/check_dist.py` per ADR-50 item 2 (no literal version; the five equalities; the alpha conversion only;
  the `cp311-abi3-<platform>` name). Shown on scratch copies: a wheel renamed to another version fails; the
  untouched archives print `check_dist: ok`.
  (d) Built: `oradump-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl` and `oradump-0.1.0a1.tar.gz` (command of
  `CONTRIBUTING.md`); `check_dist.py` ok on both; `Version: 0.1.0a1` in the wheel's `METADATA`.
  (e) `README.md`: status line, "Install" and "Other platforms" verbatim from ADR-50; no `pip install oradump` line
  left. `CONTRIBUTING.md` "Release wheel": commands with `oradump-*` globs and a pointer to the release checklist
  (`docs/design/implementation-plan.md`, "First release").
  (f) `.gitattributes` verbatim from ADR-52 and its three checks (clean `git status`, `git ls-files --eol`,
  `sha256sum -c` in the three fixture folders).
  (g) Green: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo nextest run` (395), `cargo deny
  check licenses`, `ruff check`, `ruff format --check`, `mypy`, `pytest` (69).
  (h) A short T-13f note: the `cargo deny` output, `check_dist.py` output, the wheel name, the lock diff.
- Touches: `deny.toml`, `Cargo.toml`, `Cargo.lock`, `crates/oradump-core/Cargo.toml`, `pyproject.toml`,
  `tests/check_dist.py`, `README.md`, `CONTRIBUTING.md`, new `.gitattributes`. No tag (the owner tags, checklist).
- Needs: nothing from the owner. Model: Sonnet.

## T-13g Rename to `oradmp` (session 13, small; ADR-54) - DONE (2026-10-18, `3a1bb5f`; 395 Rust tests, 73 pytest; reviewed: review-03)
- Goal: the project, its crates, its Python package and its archives are named `oradmp`; nothing shipped or built
  carries the old name; `__version__` comes from the package metadata; CONTRIBUTING's release commands work as
  written. No behaviour change: the same tests pass with the same counts (plus the new ones).
- Inputs: ADR-54 (names, inventory table, policy, the README line, `__version__`, CONTRIBUTING), ADR-50 item 2 (the
  version conversion), the T-13f note (problems 1 and 2).
- Before: the owner's edit O-13 (ADR-54 item 6: `CLAUDE.md`, `.claude/agents/*.md`, `docs/prompts/implementer-task.md`)
  so that the ownership rows name the new paths. No `pip install oradump`, no visit to that project's pages (ADR-54
  item 3).
- **Done when:**
  (a) **Moves**: `git mv crates/oradump-core crates/oradmp-core`, `git mv crates/oradump-py crates/oradmp-py`, `git mv
  python/oradump python/oradmp`; the untracked leftovers of `python/oradump/` (`_core.abi3.so`, `__pycache__`) are
  deleted, so the folder is gone. `git diff -M --stat HEAD` shows every file under `tests/data/` and `tests/golden/`
  as a rename without content change (`{... => ...}` with `0` changed lines); `git diff --exit-code` on those files
  after re-running `gen_walk_expected.py` and `gen_number_parts.py` from their new paths (the generators, not the
  reader, ADR-42).
  (b) **Names** per ADR-54 item 1: crates, `oradmp_core::` paths, `module-name = "oradmp._core"`, the `pyclass`
  module strings, `OradmpError` / `OradmpWarning` (no alias), the `oradmp:` metadata keys, the survey header and
  `prog`; `.gitattributes` names `crates/oradmp-core/tests/data/*.tsv` and `git check-attr text` on one such file
  prints `unset`; `Cargo.lock` regenerated by Cargo, `git diff Cargo.lock` shows only the two package names.
  (c) **No old name in the tree**: the note shows the output of
  `grep -rIin --exclude-dir=target --exclude-dir=.venv --exclude-dir=.git oradump crates python tests Cargo.toml
  Cargo.lock pyproject.toml deny.toml CONTRIBUTING.md .gitattributes LICENSE-MIT LICENSE-APACHE` (**empty**),
  `grep -ci oradump README.md` (**1**: the exact line of ADR-54 item 2), `find . \( -path ./target -o -path ./.venv
  -o -path ./.git \) -prune -o -iname '*oradump*' -print` (only the owner's `./oradump.code-workspace`, unless the
  owner renamed it), and the full `grep -rIli` list of the repository, each file of which is in ADR-54 item 5's
  "kept" or "owner" groups. Tests that must name the old name build it from parts (`"ora" + "dump"`) so the first
  grep stays empty.
  (d) **Old-name test of the archives**: `tests/check_dist.py` fails (one `FAIL` line per hit, with member name and
  line or byte offset) when the old name, case-insensitive, appears in any member name or member content of the
  wheel or the sdist (text and binary members, the `.so` included), except the one exact README line of ADR-54 item
  2 in the sdist's `README.md` and `PKG-INFO` and the wheel's `METADATA`; its README title check reads `# oradmp`;
  its names are `oradmp-<v>-cp311-abi3-<platform>.whl`, `oradmp-<v>.dist-info`, `oradmp-<v>/`, the required sdist
  paths `crates/oradmp-*/...`, `python/oradmp/__init__.py`. Shown on scratch copies: a wheel with one member renamed
  to `oradump/x.py`, and a sdist whose `README.md` gets a second line with the old name, each fail; the untouched
  archives print `check_dist: ok`. A pytest `tests/test_project_name.py` scans the Implementer's tracked paths of (c)
  for the old name (built from parts) and fails on a hit, so a regression shows in `pytest`, not only at release.
  (e) **`__version__`** per ADR-54 item 7 in `python/oradmp/__init__.py`; a pytest (`tests/test_version.py`) asserts
  `oradmp.__version__ == importlib.metadata.version("oradmp")`, `!= "0+unknown"`, and equal to the PEP 440 form of
  the root `Cargo.toml` version using `check_dist.py`'s conversion (imported, not copied). Shown: with `__version__`
  set to the literal `"0.1.0"` on a scratch copy, the test fails.
  (f) **`CONTRIBUTING.md`** "Release wheel" (exact sentence before the block, then the block):

      Activate `.venv` before `maturin build --zig`: `zig` is found only through `.venv/bin` on `PATH`, and `.venv` has no `pip`.

          source .venv/bin/activate
          uv pip install "maturin==1.15.0" ziglang
          maturin build --release --zig --compatibility manylinux_2_28
          maturin sdist
          python tests/check_dist.py target/wheels/oradmp-*.whl target/wheels/oradmp-*.tar.gz

  and the `target/wheels/oradump-*` files of T-13f are deleted so the globs match one file each.
  (g) **Built**: in the activated `.venv`, the commands of (f) give `oradmp-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl`
  and `oradmp-0.1.0a1.tar.gz`; `check_dist: ok`; `METADATA` has `Name: oradmp` and `Version: 0.1.0a1`; the SBOM is
  `oradmp-0.1.0a1.dist-info/sboms/oradmp-py.cyclonedx.json`.
  (h) **Fresh venv**: a new venv outside the repository (`uv venv` in a scratch folder), the wheel installed, run from
  that folder: `python -c "import oradmp; print(oradmp.__version__)"` prints `0.1.0a1`; `python -c "import oradump"`
  exits with `ModuleNotFoundError`; `python -m oradmp survey` on a copied fixture prints the `oradmp survey` header.
  Also in the repository's `.venv` after `uv pip uninstall oradump` and `maturin develop`: `import oradump` fails,
  `import oradmp` works (no stale `oradump.pth` or `oradump-0.1.0.dist-info` left).
  (i) **Green**: `sha256sum -c SHA256SUMS` in the three fixture folders; `cargo fmt --check`, `cargo clippy
  --all-targets -- -D warnings`, `cargo nextest run` (**395**, unchanged: the rename adds no Rust test), `cargo deny
  check licenses`, `ruff check`, `ruff format --check`, `mypy`, `pytest` (**69** plus the tests of (d) and (e); the
  note gives the new count).
  (j) A short T-13g note: the outputs of (c), the scratch-copy failures of (d) and (e), the wheel and sdist names and
  SHA-256, the new pytest count, anything renamed that ADR-54's inventory did not list.
- Touches: `crates/` (moved and renamed), `python/oradmp/` (moved), `tests/` (pytest files, `check_dist.py`, two new
  test files), `Cargo.toml`, `Cargo.lock`, `pyproject.toml`, `.gitattributes`, `README.md`, `CONTRIBUTING.md`, its
  section of `implementer-notes.md`. Not `corpus/`, `docs/formats/`, `CLAUDE.md`, `.claude/`, `docs/prompts/`,
  `python/exp_survey.py`, `deny.toml` (no name in it). No tag, no upload.
- Needs: O-13 (the owner's edit) first. **Model: Sonnet** (mechanical work). **One session, alone**: nothing runs in
  parallel with it (it moves the folders every other task touches).

## T-13h Review 03 fixes: C stream warnings, release build, `check_dist`, message check, panic guard (session 14; ADR-55 to ADR-60) - DONE (2026-10-18, `39cbb93`; 402 Rust tests, 105 pytest, 0 skipped; choices answered by ADR-61)
- Goal: close review 03's S-1 to S-5 and the notes ADR-60 gives to T-13h, so that 0.1.0a1 can be built by
  `tools/release_build.sh` and released.
- Inputs: ADR-55 to ADR-60; `docs/review/review-03-python-packaging.md` (findings, mutation ids d1 to d6, the T-13g
  note's PROBLEM 1 and Note 2); arch 3.4, 5.1, 7.1, 7.3, 9; ADR-46 (the message check), ADR-48, ADR-50, ADR-54.
- Before: the owner's edit **O-14** (`tools/` and `fuzz/` in the Implementer's row of `CLAUDE.md`; ADR-56 item 5).
- **Done when:**
  (a) **S-1, C stream** (ADR-55): `SharedBatches` in the binding, the shared `Arc<Mutex<..>>`, no stash before the
  export; `close()` / `__exit__` issue the warnings not yet issued; iteration after the export is `UsageError`; a pull
  after `close()` fails with `[E-CONFIG`. New tests in `tests/test_handover.py` (or a new `tests/test_stream.py`), all
  on `18c/t_num_exp_18c.dmp` T_NUM with `on_value_error="null"`: through `pyarrow.RecordBatchReader.from_stream(r)
  .read_all()`, `r.rows_read == 33` and `r.warnings` equal to the iterator path's (codes, counts, offsets) and to the
  literal counts of `test_warnings.py` (10 nulled, 4 rounded); issued once at the end of the `with`; `batch_rows=5` and
  one `read_next_batch()` give `rows_read == 5`; `close()` before the first pull; `next(r)` after the export.
  Mutations on a scratch copy, each alone: **s1a** (restore the stash before the export) fails the `from_stream` test;
  **s1b** (`close()` issues nothing) fails the issue test. A two-thread run of the `test_threads.py` kind through
  `from_stream(...).read_all()` is **recorded** in the note (ratio), not asserted.
  (b) **S-2, release build** (ADR-56): `tools/release_build.sh` as ADR-56 item 1 says (options: the commit, default
  `HEAD`; `--twice`); CONTRIBUTING's "Release wheel" block is `tools/release_build.sh` plus one sentence. Run once on
  the T-13h commit: `check_dist: ok`; `strings -a` of the wheel's `_core.abi3.so` and the SBOM text have **0** hits for
  `/home/` and the user name's home path; the SBOM's `path+file://` references start with the neutral root; the
  `--twice` result (same hashes, or which members differ). The wheel and sdist of this run are **not** the release
  (the release is built in checklist step 5 on the tagged commit). If the SBOM still carries a home path, set
  `[tool.maturin.sbom] rust = false`, say so, and stop for the Architect (ADR-56 item 2).
  (c) **S-3, `check_dist.py`** (ADR-57 items 1, 2) and **`tests/test_check_dist.py`** (item 3): one FAIL per rule on
  the synthetic archives listed there, and a clean synthetic pair passes; plus the `_probe_panic` rule of ADR-59 item
  5 (c).
  (d) **S-4, message check** (ADR-58): forms (c) stronger and (e) new in `t12_messages.rs`; the inventory assertion
  (longest digit run in any message is at most 5); the self-test of ADR-58 item 4; the mutations d1, d2, d3, d4a, d4b,
  d4c, d5, d6, d7, d8 of ADR-58 item 5, each alone, each failing `no_message_of_the_fixtures_holds_a_value` with the
  forms printed in the note (d4b, d4c, d8 with (e); d6, d7 with (c)).
  (e) **S-5, panic guard** (ADR-59): `oradmp_core::guard::catch_panic` with its unit tests; `ErrorCode::Internal`
  (`E-INTERNAL`), `InternalError(OradmpError)` in `_errors.py` and `_core.pyi`; the guard at every core call of the
  binding and in `SharedBatches::next`; the "unusable afterwards" mark; feature `test-probes` and
  `tests/test_internal.py` (3 cases, the `stream` one in a subprocess); arch 7.1's code table walked by
  `test_errors.py` still matches `_core.ERROR_CODES`. Mutations of ADR-59 item 6, each alone, each failing the named
  test.
  (f) **Notes** (ADR-60): N-2 (locks inside `py.detach`; `stacklevel` test for `__next__`, `read_all()`, `close()`);
  N-4 (the three assertions, each shown to fail by a one-line mutation); N-5 (`test_interval.py` unconditional); N-6
  (README "Errors" sentence of ADR-60 exactly; README stream example of ADR-55 item 8; CONTRIBUTING dev setup with `uv`
  and `--features test-probes`); N-8 (stale `.so` and `__pycache__` rebuilt / deleted; `~/.cache/oradump-t12`
  deleted).
  (g) **Green**: `sha256sum -c SHA256SUMS` in the three fixture folders; `cargo fmt --check`; `cargo clippy
  --all-targets -- -D warnings` and the same with `--all-features`; `cargo nextest run` (395 plus the new tests; the note
  gives the count); `cargo deny check licenses`; `ruff check`, `ruff format --check`, `mypy`; `pytest -rs` in the dev
  build (`maturin develop --features test-probes`, pandas installed): 73 plus the new tests, **0 skipped**.
  (h) A T-13h note in `implementer-notes.md`: the outputs of (b), the mutation table of (a), (c), (d), (e), (f) with the
  failing test of each, the counts, anything ADR-55 to ADR-60 did not foresee.
- Touches: `crates/oradmp-py/` (`lib.rs`, `errors.rs`, `Cargo.toml` feature), `crates/oradmp-core/src/` (`error.rs`:
  one code; new `guard.rs`; `lib.rs`: the module), `crates/oradmp-core/tests/t12_messages.rs`, `t12_offsets.rs`,
  `python/oradmp/` (`_api.py`, `_errors.py`, `_core.pyi`, `__init__.py` export), `tests/` (`check_dist.py`, new
  `test_check_dist.py`, `test_internal.py`, the stream tests, `test_errors.py`, `test_truth.py`, `test_interval.py`,
  `test_warnings.py` for `stacklevel`), `tools/release_build.sh` (new), `README.md`, `CONTRIBUTING.md`,
  `pyproject.toml` (only if ADR-56 item 2's fallback is needed), `Cargo.lock` (no new dependency expected), its
  section of `implementer-notes.md`. Not `corpus/`, `docs/formats/`, `tests/golden/`, `CLAUDE.md`.
- Needs: O-14. **Model: Opus** (the shared state across the C stream, the lock rule and the panic guard are binding
  work where a mistake deadlocks or aborts; the rest is mechanical). **Alone**: nothing runs in parallel. If one
  session is not enough, part 1 = (a), (e), N-2 of (f), with (g) green and a commit; part 2 = (b), (c), (d), the rest
  of (f), (g), (h). Then the release (checklist), T-14.

## T-14 Hardening: property tests and fuzzing (rewritten in session 15, ADR-61, ADR-62) - DONE (2026-10-18, `abd42ad`; 408 Rust tests, 0 findings; reviewed: review-04; S-3 and N-3 fixed in T-13i)
- Goal: arch 8.3 (a) and the `cargo-fuzz` targets; review 03 N-9; the README sentence and the NUL rule of ADR-61.
  Arch 8.3 (b) is T-17 now; 8.3 (c) exists (`t4_values.rs`, `number_roundtrip`).
- Inputs: ADR-59, ADR-61 items 2, 3, ADR-62; arch 7.3, 8.3; `docs/review/review-03-python-packaging.md` N-9 and section
  4; the T-13h note ("For the next tasks"): `oradmp_core::guard`, `ErrorCode::Internal` (`ErrorCode::ALL` = 25 codes),
  `pub mod guard` in `lib.rs`. The fuzz targets call the core **directly**: a panic there is a **crash** (a finding),
  never `E-INTERNAL`; the guard is the binding's and is not used in `fuzz/`.
- Machine (ADR-62 item 2): one build at a time; `CARGO_BUILD_JOBS=2` for every `cargo` / `maturin` command; `cargo
  nextest run -j 2`; `cargo fuzz run` one target at a time with `-jobs=1 -workers=1`, nothing else running.
- **Done when:**
  (a) **Toolchain**: a dated nightly installed by `rustup toolchain install nightly-<date> --profile minimal` and pinned
  in `fuzz/rust-toolchain.toml`; `cargo install cargo-fuzz --locked --version <x>`; the note gives both, and
  `cargo fuzz --version` run in `fuzz/`. The root `rust-toolchain.toml` (1.99.0) is unchanged.
  (b) **`fuzz/`** is its own crate outside the workspace (own `[workspace]`), `fuzz/Cargo.lock` committed,
  `fuzz/.gitignore` with `corpus/`, `artifacts/`, `coverage/`, `target/`; `tools/fuzz_seeds.sh` copies the `.dmp`
  files of `corpus/fixtures/{18c,11g,11g-cp1251}/` into `target/fuzz-seeds/` (read only from the fixtures) and prints
  the count. `git diff --stat Cargo.lock` (root) is **empty** at the end of the task (if a fix needs a dependency, stop
  for the Architect). `cd fuzz && cargo fuzz build` builds the three targets.
  (c) **Three targets** (ADR-62 item 5, arch 8.3): `walk` (`oradmp_core::exp::items::walk` over `Cursor<&[u8]>`,
  `WalkOptions::default()` with `hash_fields = true`); `read` (the input written to one file in a per-process temporary
  folder, then `dump::open`, `scan`, and every table read to its end with `batch_rows = 1024`, `batch_bytes = 4 MiB`,
  `max_lob_bytes = 1 MiB`, `on_value_error = Null`); `values` (first byte selects the decoder: NUMBER, DATE, TIMESTAMP
  180 / 181 / 231, INTERVAL 182 / 183, ROWID; the rest is the field). Every `Err` the targets see is checked, and a
  failed check is a `panic!` (= a finding): its `Display` starts with `[E-`, its code is in `ErrorCode::ALL`, its
  offset (if any) is at most the input length, its text contains no U+0000 (ADR-61 item 3). Same checks for every
  warning's `Display` in `read`.
  (d) **Runs**: per target, alone, `cargo fuzz run <t> fuzz/corpus/<t> target/fuzz-seeds -- -max_total_time=600
  -max_len=1048576 -timeout=10 -rss_limit_mb=2048 -malloc_limit_mb=64 -jobs=1 -workers=1`. **Session 16 (review 04
  S-3, ADR-64 item 3): this command fails from the root (the root's 1.99.0 pin wins; `fuzz/corpus/<t>` must exist).
  From now on: `tools/fuzz_run.sh <t> [seconds]`; by hand: `tools/fuzz_seeds.sh && mkdir -p fuzz/corpus/<t> && cd
  fuzz && TMPDIR=/var/tmp/oradmp-fuzz cargo fuzz run <t> corpus/<t> ../target/fuzz-seeds -- <the limits above>`.** The last run of each target,
  on the final code, ends with **0 crashes, 0 timeouts, 0 out-of-memory**; the note gives per target the total
  executions, executions per second, the final `cov:` and `corpus:` numbers, and every finding of earlier runs.
  (e) **Findings** (ADR-62 item 7): each has a regression test in `crates/oradmp-core/tests/t14_fuzz.rs` (fixture plus
  patches, or a minimised file below 4 KiB in `crates/oradmp-core/tests/fuzz/`), shown to fail (panic, hang or the
  check of (c)) on the code before the fix and to pass after; the note lists finding, input, cause, fix, test. A fix
  that would change an error code, a warning, any fixture's rows or values, or a rule: stop, note for the Architect.
  Zero findings is a valid result (the note says so; `t14_fuzz.rs` is then not created).
  (f) **Damage properties** (arch 8.3 (a)), `crates/oradmp-core/tests/t14_damage.rs`, over the 201 exp files of
  `exp_catalog.tsv` (the strategy picks the file): `truncation_gives_ok_or_a_typed_error` (cut at a random offset t:
  `open` + `scan` + every table read with `on_value_error = Null` returns `Ok` or an error with a code of
  `ErrorCode::ALL` and an offset at most t; no table yields more rows than the golden catalog's count for it);
  `bit_flips_give_ok_or_a_typed_error` (1 to 8 random bit flips: `Ok` or a typed error whose offset is at most the file
  length). Default 256 cases each in `cargo nextest run`; one run with `PROPTEST_CASES=10000` recorded in the note
  (time, failures 0). A shrunk failure is a finding of (e); proptest's regression file is committed.
  (g) **N-9 exact counts** (review 03): `t3_catalog.rs:960` and `:963`, `t8_direct.rs:164-165`, `t2_manifest.rs:336-337`
  and `:441`, `t1_walk.rs:808`, `t5f_followup.rs:240-273` (4 TSLTZ sections, `collist.py`): each loose bound becomes an
  exact value; the note gives per place the value, its source (the Analyst's tool and command, or the golden catalog
  column) and a one-line mutation that fails it (a mutation table like T-13h's). Line numbers are those of review 03;
  the note gives the current ones.
  (h) **Interval and messages**: `t13_interval.rs` gains the DAY TO SECOND opposite-signs test under `interval_as =
  "string"` (`E-BAD-INTERVAL`, on the same kind of input as `opposite_signs_stay_e_bad_interval_under_string`), shown to
  fail when the DS opposite-sign check is removed (review 03 section 4); the comment at `t13_interval.rs:45` says four
  rows. `t12_messages.rs` asserts that no error or warning text of the 201 files contains U+0000 (ADR-61 item 3).
  (i) **README** (ADR-61 item 2): the stream-error sentence, exactly as in ADR-61 item 2, after the stream paragraph.
  (j) **Green**: `sha256sum -c SHA256SUMS` in the three fixture folders; `cargo fmt --check`; `cargo clippy --all-targets
  -- -D warnings` and the same with `--all-features`; `cargo nextest run -j 2` (402 plus the new tests; the note gives
  the count); `cargo deny check licenses`; `ruff check`, `ruff format --check`, `mypy`; `pytest -rs` in the dev build
  (`maturin develop --features test-probes`): 105 passed, **0 skipped**.
  (k) A T-14 note in `implementer-notes.md`: (a), (d), (e), (f), (g) as above, the counts, anything ADR-62 did not
  foresee.
- Touches: `fuzz/` (new), `tools/fuzz_seeds.sh` (new), `crates/oradmp-core/tests/` (`t14_damage.rs`, `t14_fuzz.rs` and
  `tests/fuzz/` if needed, the N-9 files, `t13_interval.rs`, `t12_messages.rs`, proptest regression files),
  `crates/oradmp-core/src/` only for fixes of findings, `README.md` (one sentence), its section of
  `implementer-notes.md`. Not `crates/oradmp-py/`, `python/`, the root `Cargo.toml` / `Cargo.lock`, `corpus/`,
  `docs/formats/`, `tests/golden/`, `crates/oradmp-core/tests/data/`.
- Needs: T-13h committed (done, `39cbb93`). **Model: Opus** (ADR-62 item 3). **Alone** on the machine; T-15 after it.
  If one session is not enough: part 1 = (a) to (e) with (j) green and a commit; part 2 = (f) to (k).

## T-15 Benchmarks (rewritten in session 15, ADR-62) - DONE (2026-10-19, `b3a5a7f`; `T_ROWS` only, `T_BIG` absent; reviewed: review-04; baseline in `benchmarks.md`; N-4 in T-13i)
- Goal: arch 4.3, 4.4: `criterion` micro-benchmarks; throughput and peak RSS on the owner's `T_ROWS` export; the memory
  test of arch 4.3; the memory of the Arrow stream path recorded. The Architect copies the baseline into
  `docs/design/benchmarks.md`.
- Inputs: arch 4.1, 4.3, 4.4, 4.6; ADR-55; ADR-62 items 2, 8; owner action **O-15** (`open-questions.md`): the path of
  the `T_ROWS` export (`corpus/sql/01_corpus_tables.sql`, 100,000 rows, 6 columns) and, if made, `T_BIG`. Only this
  task reads those files, through the environment variable `ORADMP_WORK`; nothing from them is committed.
- Machine (ADR-62 item 2): as T-14; every benchmark and measurement runs alone, release build; the note gives the CPU
  model (`lscpu`), `nproc`, free memory before the run, rustc version.
- **Done when:**
  (a) `criterion` is a dev-dependency of `oradmp-core` (one exact version, in the note) with `[[bench]] name = "decode"
  harness = false`; the root `Cargo.lock` diff adds only `criterion` and its dependencies (listed in the note); `cargo
  deny check licenses` stays `licenses ok` (a licence outside `deny.toml`'s list: stop for the Architect, ADR-45).
  (b) `crates/oradmp-core/benches/decode.rs`, groups on committed fixtures: `number` (every NUMBER field of
  `18c/t_num_exp_18c.dmp` T_NUM through `OraNumber::decode`), `datetime` (T_DATE, T_TS fields), `text_cp1251` (the text
  fields of R_STAT in `ru_stat_exp_cp1251_est_11g.dmp`), `read_table` (a full read of R_STAT, 2,000 rows; of T_NUM; of
  T_TEXT). `CARGO_BUILD_JOBS=2 cargo bench -p oradmp-core --bench decode` runs; the note gives per benchmark the median
  and the throughput (fields or rows per second, and bytes per second).
  (c) **Throughput** (arch 4.3): `crates/oradmp-core/tests/t15_throughput.rs`, `#[ignore]`, run by `cargo nextest run
  --release --run-ignored only -E 'binary(t15_throughput)' -j 1` with `ORADMP_WORK` set: per file, scan MB/s (file size
  / scan time) and per table rows/s and MB/s of a full read with default options, 3 runs, the median; prints, asserts
  nothing; skips with a message when `ORADMP_WORK` or the file is absent. Numbers for `T_ROWS` (and `T_BIG` if present)
  in the note.
  (d) **Memory test** (arch 4.3, 4.1 (a)): `crates/oradmp-core/tests/t15_memory.rs`, `#[ignore]`, same command: peak RSS
  (`VmHWM` of `/proc/self/status`) of a child process (the test binary re-run through `std::env::current_exe()` with a
  filter for a helper test) that reads `T_ROWS` with `batch_rows = 4096`, once stopping after 10,000 rows and once to
  the end (100,000 rows, ten times the data, same file and schema); asserts **peak(100,000) <= 1.10 x peak(10,000)**;
  the note gives both peaks. With `T_BIG` present, its peak with the same options is recorded (not asserted: other
  columns).
  (e) **Stream memory** (review 03's unreached check, ADR-55): `tools/stream_memory.py` (not shipped) reads
  `18c/part_exp_18c.dmp` T_PART 500 times through `BatchReader` iteration and 500 times through
  `pyarrow.RecordBatchReader.from_stream(r).read_all()`, dropping each table, and prints RSS (`/proc/self/status`
  `VmRSS`) after reads 50, 250 and 500 per path, and `PASS` when RSS at 500 is at most RSS at 50 plus 10 MiB, else
  `FAIL`; run on a release wheel installed in a scratch venv (`tools/release_build.sh` into a scratch root, or `maturin
  build --release` for tests only); output in the note.
  (f) **Green**: as T-14 (j), with the new counts (the ignored tests are not in the default run).
  (g) A T-15 note in `implementer-notes.md` with (a) to (e) and the machine facts; the Architect copies the baseline into
  `docs/design/benchmarks.md` (the CI regression gate of arch 4.3 comes with T-16).
- Touches: `crates/oradmp-core/Cargo.toml` (dev-dependency, `[[bench]]`), the root `Cargo.lock`,
  `crates/oradmp-core/benches/` (new), `crates/oradmp-core/tests/t15_throughput.rs`, `t15_memory.rs`,
  `tools/stream_memory.py`, its section of `implementer-notes.md`. Not `src/`, `crates/oradmp-py/`, `python/`, `corpus/`.
- Needs: T-14 committed; **O-15**. **Model: Sonnet** (ADR-62 item 3). **Alone** on the machine.

## T-13i Review 04 fixes: stream tests, `check_dist` rules, message check, damage sites, fuzz script, version 0.1.0a2 (session 16; ADR-64, ADR-65, ADR-67 item 2) - DONE (2026-10-21, `469416d` + `dc7668e`; 411 Rust tests + 3 ignored, 117 pytest, 0 skipped; deviations answered by ADR-69; to be reviewed: Reviewer 5)
- Goal: every should-fix of review 04 closed by a test or a script that a one-line mutation fails; the notes marked
  "T-13i" in ADR-65; the version bump for run B.
- Inputs: `docs/review/review-04-t13h-t15.md` (S-1 to S-5, N-1 to N-9, the mutation tables); ADR-64, ADR-65, ADR-67
  item 2; the T-14 and T-15 notes. Rules unchanged.
- Machine (ADR-62 item 2): one build at a time, `CARGO_BUILD_JOBS=2`, `cargo nextest run -j 2`; the fuzz smoke run and
  the memory test alone.
- **Done when:**
  (a) **S-1**: `tests/test_stream.py` test of ADR-64 item 1 ((a) close before the end: `UsageError` `E-CONFIG` "the
  reader is closed" from `next(r)` and `read_all()`, `rows_read` 1000; (b) after the end: `StopIteration`, `read_all()`
  0 rows with the schema). Mutation **s1f** fails (a).
  (b) **S-2**: the patched copy of ADR-64 item 2 (18c `t_num_exp_18c.dmp`, byte **0x22F4** `c2` -> `bc`, asserted
  before patching together with the length `05 00` at 0x22F2; made in `tmp_path`, the fixture untouched). Both paths,
  default options: `E-NUMBER-OVERFLOW` at 0x230F row 19 (iterator: the exception; stream: `pyarrow.ArrowInvalid`
  containing `[E-NUMBER-OVERFLOW offset=0x230f]`), and `r.warnings` == one `W-NUMBER-ROUNDED`, count 1, offset 8948,
  row 17, column `V`, issued once at the end of the `with`. Mutation **s1e** fails the stream case.
  Session 17 (ADR-69 item 1): "issued once" means on the iterator path when the error ends the iteration (the `with`
  end issues nothing more), on the stream path at the end of the `with`; the test as built checks exactly that.
  (c) **S-3**: `tools/fuzz_run.sh <target> [seconds]` as ADR-64 item 3 (runs from `fuzz/`, creates
  `fuzz/corpus/<t>`, `TMPDIR` with at least 2 GiB, cleans `oradmp-fuzz-*` and `fuzz-0.log`, non-zero exit on a finding);
  `fuzz/.gitignore` gains `fuzz-*.log`; `CONTRIBUTING.md` gets a "Fuzzing" section (prerequisites with the exact
  nightly and `cargo-fuzz 0.13.2`, the script, the machine rule, where findings go). `tools/fuzz_run.sh walk 30` run in
  a fresh clone (no `fuzz/corpus/`): output in the note.
  (d) **S-4**: the nine cases m1 to m9 of ADR-64 item 4 in `tests/test_check_dist.py`; each rule removed alone fails
  its case (table in the note).
  (e) **S-5**: ADR-64 item 5: the chain and chunk copies in the message check through one shared helper; `seen` holds
  "(piece 2 of the value)" and "(chunk 2 of the value)"; exact-suffix assertions in `t6_patched.rs` and
  `t12_offsets.rs`; the suffix removed before the 5-digit inventory; an `in_piece` unit test for 2, 100000,
  `u64::MAX`. The **premise** mutation (`n + 100000`) fails; a value appended before the suffix still fails
  `no_message_of_the_fixtures_holds_a_value`.
  (f) **d9, d10** (ADR-65 item 4): the widened (e3) date tokens and (e1) group joining, one self-test case each; d9
  and d10 fail `no_message_of_the_fixtures_holds_a_value`; d1 to d8 still fail it (repeated, table in the note).
  (g) **Damage sites** (ADR-65 item 3): `t14_damage.rs::each_truncation_site_names_its_offset` (one case per
  "file ends" site of ADR-65 item 3, a named fixture and cut taken from `exp_parse.py` output, exact code and offset; the note gives
  per case the fixture, the cut, the `exp_parse` line and the expected offset with its rule) and the
  `read_unlisted_field` unit test in `framing.rs` (`03 00 61 62 63`, `E-TRUNCATED` at 5); warning offsets bounded in
  both properties. Mutations **a1**, **a2**, **c2** fail at the default 256 cases.
  (h) **Memory test** (ADR-65 item 6): limit stays 1.10; the stop is asserted `== 12_288`; the name says eight times;
  with `T_BIG` in `ORADMP_WORK` the same test runs on it (stop at 100,000 rows) with 1.10; one run on `T_ROWS` in the
  note. Session 17 (ADR-65 amendment): on `T_ROWS` the 8-byte mutation is at the limit (caught 1 of 3); the
  measurement on `T_BIG` is T-15b.
  (i) **Guard** (ADR-65 item 1): `NumberType::from_str` (both places in `options.rs`) and `number_summary` inside
  `catch_panic`; the module comment of `lib.rs` names the exempt calls of ADR-59's session 16 amendment.
  (j) **Stub and script** (ADR-65 items 7, 8): `_probe_panic` removed from `python/oradmp/_core.pyi` (tests use
  `getattr`); `tools/release_build.sh` pins `ziglang==<the version run A printed>` and resolves the root with
  `realpath -m` before its checks (refusal of a symlink into `$HOME` shown in the note).
  (k) **Green**: as T-14 (j); `pytest -rs` in the dev build: 105 plus the new tests, **0 skipped**; `cargo nextest run
  -j 2`: 408 plus the new tests, 3 skipped (the T-15 ignored tests).
  (l) **Version** (ADR-67 item 2), the last commit, only after the owner's **Q-10** answer: `0.1.0-alpha.2` in
  `[workspace.package]`, `Cargo.lock` (two entries), `fuzz/Cargo.lock` (`oradmp-core` entry); `README.md` status line
  and install text per the answer; `cargo metadata` and a `tools/release_build.sh` dry build into a scratch root print
  `0.1.0a2` and `check_dist: ok`. Without an answer, T-13i ends before (l) and (l) is a separate small commit before
  run B.
  (m) A T-13i note in `implementer-notes.md`: the mutation tables (s1e, s1f, m1 to m9, premise, d1 to d10, a1, a2,
  c2, the memory mutation of 4 bytes per row on `T_ROWS` recorded as passing, by design), counts, anything the ADRs did
  not foresee.
- Touches: `tests/test_stream.py`, `tests/test_check_dist.py`, `tests/check_dist.py` only if a rule is found broken,
  `crates/oradmp-core/tests/` (`t12_messages.rs`, `t6_patched.rs`, `t12_offsets.rs`, `t14_damage.rs`,
  `t15_memory.rs`, `common/`), `crates/oradmp-core/src/reader.rs` and `exp/framing.rs` (unit tests only),
  `crates/oradmp-py/src/` (`lib.rs`, `options.rs`: the wraps), `python/oradmp/_core.pyi`, `tools/fuzz_run.sh` (new),
  `tools/release_build.sh`, `fuzz/.gitignore`, `CONTRIBUTING.md`, and for (l) `Cargo.toml`, `Cargo.lock`,
  `fuzz/Cargo.lock`, `README.md`. Not `corpus/`, `docs/formats/`, `tests/golden/`, `crates/oradmp-core/tests/data/`,
  the root `.gitignore` (O-16). A change that alters an error code, a warning, a rule or any fixture's rows stops for
  the Architect.
- Needs: nothing (Q-10 only for (l)). **Model: Opus** (it edits the checks that guard the archives and the message
  rule, where a wrong change weakens a safety net silently, and it precedes an irreversible upload). **Alone.** Then
  Reviewer session 5 (closure, ADR-67 item 1).

## T-13j Review 05 fixes and the unreachable LOB-length branch (session 17; widened in session 18, ADR-71) - DONE (2026-10-23, in `1c17633` and `460aef5`; 412 Rust tests + 3 skipped, 143 pytest, 0 skipped; no Project-URL (O-19 (a)); reviewed: review-06a, no blocking finding)
- Goal: one truncation behaviour for the 8-byte total of a long LOB value [L4] (ADR-69 item 2); every finding and
  accepted note of review 05 closed by a test, a mutation or a named check (ADR-71); the README fixed for a3.
- Inputs: `docs/review/review-05-t13i-closure.md` (SF-1, SF-3, N-1, N-3 to N-6, N-8; the mutation table); ADR-69 item
  2, ADR-71. Rules unchanged (v3.6).
- Machine (ADR-62 item 2): one build at a time, `CARGO_BUILD_JOBS=2`, `cargo nextest run -j 2`.
- **Done when:**
  (a) `crates/oradmp-core/src/io.rs` has `pub fn read_u64_le(&mut self) -> Result<u64>`, written like `read_u16_le`
  (`read_bytes(8)?`, `<[u8; 8]>::try_from`, `map_err` to `self.truncated("file ends inside a field")`); a unit test in
  `io.rs` over a `Cursor`: 8 bytes `01 00 00 00 00 00 00 00` give 1; 7 bytes give `E-TRUNCATED` "file ends inside a
  field" at offset 0 (the start of the read).
  (b) `exp/rows.rs` `lob_value` reads the total with `s.read_u64_le()?`; `grep -rn "inside a LOB length" crates/` is
  empty.
  (c) `t14_damage.rs`: the doc comment and the case label that name the removed site are reworded; the doc comment
  says the expected offset is where the failed read stood (`io.rs` `truncated()`: the current position), so the
  "inside a line" case expects the end of the file (review 05 N-3). The two LOB cases (18c `lob_exp_18c.dmp`, cuts
  0x4359 and 0x435d: `E-TRUNCATED` at 0x4358 and 0x435a) unchanged and green; review 05's mutation **line_start**
  still fails `each_truncation_site_names_its_offset`.
  (d) **SF-1**: `tests/test_stream.py::test_the_warnings_are_complete_after_an_error_on_the_iterator_path` calls
  `check_row_17_rounded_before_row_19(reader)` after its `pytest.raises` block. Mutation **s1o** (review 05 SF-1,
  exact text, in `crates/oradmp-py/src/lib.rs` `pull_raw`) fails it; **s1e** and **s1i** still fail it.
  (e) **SF-3**: `tests/test_check_dist.py` gains one synthetic case per rule, in the style of m1 to m9 (a copy of a
  good archive with one property broken; the exact `FAIL` line asserted): **m10** sdist file name; **m11** the old
  name in an sdist member's content (a second occurrence in `README.md` fails; the allowed line alone passes);
  **m12** `Requires-Dist` without `pyarrow>=15`; **m13** another `License-Expression`; **m14** another
  `Requires-Python`; **m15** each required sdist file missing (parametrised over the ten entries); **m16** each licence
  file missing from `.dist-info/licenses/` (two cases); **m17** `Metadata-Version: 2.3`; **m18** `License-File` lines
  wrong; **m19** the description not the README; **m20** a `License ::` classifier; **m21** METADATA missing from the
  wheel, PKG-INFO missing from the sdist. Each rule of `tests/check_dist.py` removed alone (`if False:` or the line
  deleted) fails exactly its case: table in the note. `check_dist.py` changes only where a case shows a rule raises
  instead of printing its `FAIL` line (then it prints the line; the note says which).
  (f) **N-1**: the module comment of `crates/oradmp-py/src/lib.rs` names the exempt calls of ADR-59's session 18
  amendment (`Warning`'s and `Error`'s `Display` in `errors.rs`, `Charset::from_id` / `name` in `conv.rs`, the
  `as_str` constants and `ALL`, the implicit drops of `PyReader` / `PyDump`). A comment only; check: each row of review
  05's guard table marked "not in the list" is named in the comment (the note quotes the comment).
  (g) **README** (ships in a3; review 05 N-8 and the SF-2 leftover, ADR-70 item 2): the status part names again
  "Python 3.11 or later, pyarrow 15 or later"; the two install commands in code marks; the two licence links
  point to absolute URLs. A new test in `tests/test_project_name.py`: no Markdown link in `README.md` has a target
  that does not start with `https://`, `http://` or `#`; mutation: adding `[LICENSE-MIT](LICENSE-MIT)` back fails it.
  Project-URL only if **O-19 (a)** has named a public URL when T-13j starts: `[project.urls]` `Source = "<url>"` in
  `pyproject.toml`, a `check_dist.py` rule that METADATA has `Project-URL: Source, <url>`, and its case **m22**;
  otherwise nothing (the note says so).
  (h) **N-6**: a test in `tests/test_version.py`: every `oradmp==<v>` and every `oradmp-<v>` file name in
  `README.md` equals the Python form of `[workspace.package] version` (the `pep440` helper of `check_dist.py`);
  mutation: one `0.1.0a2` of the README changed to `0.1.0a1` fails it.
  (i) **N-4**: `CONTRIBUTING.md` "Fuzzing" says the script removes the `oradmp-fuzz-*` folders and moves `fuzz-0.log`
  into the tmp folder; check: the note quotes the line of `tools/fuzz_run.sh` that moves it.
  (j) **N-5**: the doc comment of `t15_memory.rs` says that `T_BIG`'s resolution of about one byte per row is
  arithmetic until T-15b measures it.
  (k) Green as T-13i (k): Rust 411 + 1 (the `read_u64_le` unit test) passed, 3 skipped; pytest 117 + the new cases,
  **0 skipped** in the dev build; `cargo fmt`, `clippy -D warnings`, ruff, mypy. No golden or expected file changes
  (none is regenerated). No benchmark run in this task (a3's step 3b is **due**: a file of `crates/oradmp-core/src/`
  changed).
  (l) A T-13j note: the mutation table (s1o, s1e, s1i, m10 to m21 (m22), the README link and version mutations,
  line_start), the counts, anything the ADRs did not foresee.
- Touches: `io.rs`, `exp/rows.rs`, `tests/t14_damage.rs`, `tests/t15_memory.rs` (comment), `crates/oradmp-py/src/lib.rs`
  (comment), `tests/test_stream.py`, `tests/test_check_dist.py`, `tests/test_project_name.py`, `tests/test_version.py`,
  `README.md`, `CONTRIBUTING.md`; `tests/check_dist.py` and `pyproject.toml` only as (e) and (g) say. Not the version
  (stays `0.1.0-alpha.2`; the a3 bump is the release's own commit), `corpus/`, `docs/formats/`, `tests/golden/`,
  `crates/oradmp-core/tests/data/`, the root `.gitignore`. A change that alters an error code, a warning, a rule or
  any fixture's rows stops for the Architect.
- Needs: nothing (O-19 (a) only for the Project-URL). **Model: Sonnet.** Alone (machine rule). **Reviewed in Reviewer
  session 6** together with T-16 and T-18 (one group under rule 7, ADR-71 item 13), before a3 is built.

## T-15b The memory test on `T_BIG` (session 17; ADR-69 item 3; restated in session 18) - DONE (2026-10-24, `0956df0`; note only; results ruled by ADR-75 items 1, 2; repeated by Reviewer 6a, now baseline 2, ADR-78)
- Goal: the memory test resolves a retention of a few bytes per row, measured, not only computed.
- **Prompt data the owner gives the Implementer** (O-17): the work folder for `ORADMP_WORK` (the owner's path; never
  written into a committed file); that `$ORADMP_WORK/18c/t_big_exp_18c.dmp` exists (table `T_BIG` of
  `corpus/sql/15_big.sql`, made by `corpus/scripts/make_big.sh`; session 19: session 18 named the 01b block here, which
  was wrong; 5,000,000 rows, 6 columns, 18c, `exp`, conventional path, table mode); its size in bytes
  and SHA-256; the line of the `exp` log beside it that says 5,000,000 rows exported; that the VM runs nothing else
  during the task. Read only; nothing from it is committed.
- **Order**: after T-13j is committed (T-13j's comment change in `t15_memory.rs` comes first; the measured code then
  has the `read_u64_le` change, which is behaviour-neutral) and **alone on the machine**: no build, test, fuzz or
  benchmark of any other session at the same time (ADR-62 item 2). Before or after T-16, whichever comes first once
  O-17 is done.
- Commands (`benchmarks.md` section 3): `ORADMP_WORK=<work folder> CARGO_BUILD_JOBS=2 cargo nextest run --release
  --run-ignored only -E 'binary(t15_memory)' -j 1 --no-capture` (three times), and the same with
  `binary(t15_throughput)` once.
- **Done when:**
  (a) `peak_rss_of_eight_times_the_data_stays_within_ten_percent` runs its `T_BIG` branch (stop at exactly 102,400
  rows, then to the end), **three runs** alone in a release build: the correct reader passes 3 of 3; the note gives
  each ratio and both peaks.
  (b) The mutation "retain b bytes per row" in `row_end` (as T-13i (h)), three runs each, b = **8** and **4**: each
  fails **3 of 3** on `T_BIG`; b = **1**: recorded (by arithmetic it adds 4,897,600 bytes, so it should fail; if it
  does not, the note says so and the Architect corrects ADR-65's "1 byte per row").
  (c) `t15_throughput` (it already lists `18c/t_big_exp_18c.dmp`, `T_BIG`) runs once; its `T_BIG` figures are given
  in the note (not a baseline: benchmarks.md rule 1 needs a Reviewer repeat).
  (d) No test file changes unless (a) fails for the correct reader; then stop and report to the Architect (no new
  limit chosen in the task).
- Touches: the note only (and scratch mutations, reverted). **Model: Sonnet.** Alone on the machine. Gates nothing.

## T-16 More platforms (session 12, ADR-53; "Done when" rewritten in session 16, ADR-68) - split in session 22 (ADR-87): **T-16a** (Linux CI) after T-17b, **T-16b** (other platforms) after a4; see "Session 22" at the end of this task
- Goal: tested wheels beyond Linux x86_64 `manylinux_2_28`. **Not needed for 0.1.0a2.**
- Inputs: ADR-49, ADR-52, ADR-53, ADR-56, ADR-57, ADR-66, ADR-68; `tests/check_dist.py`; `tools/release_build.sh`;
  `tools/fuzz_run.sh`; `benchmarks.md`; maturin documentation of the pinned version.
- Needs from the owner (**Q-11**): the CI service; whether the mirror may be public; the mirror itself and the runner
  registration (the owner's accounts). No secret is needed: CI never uploads in phase 1.
- **Done when:**
  (a) A CI configuration runs the matrix of ADR-53 item 2: Linux x86_64 `manylinux2014` (`--zig`), Windows x86_64,
  macOS arm64, macOS x86_64; optionally Linux aarch64. Every job sets `CARGO_BUILD_JOBS=2` and a `TMPDIR` on the
  runner's work disk (T-14: `/tmp` can be a small tmpfs).
  (b) First run, before anything else: the three risk checks of ADR-53 item 7 ((a) line endings, paths, file locks;
  (b) glibc of the `manylinux2014` build, `auditwheel show`; (c) the abi3 tag), each result in the T-16 note.
  (c) On every platform: fixture checksums, full `cargo nextest run`, wheel and sdist built **with path remaps**
  (`--remap-path-prefix` of the cargo home and the checkout, as ADR-56 item 1 does on Linux), `check_dist.py` (one
  extension module of any suffix; the home-path scan unchanged: `/Users/`, `C:\Users\` included) passes on every wheel,
  wheel installed into a fresh venv, full `pytest -rs` against it, on Python 3.11 (pyarrow 15.0.2, pandas 2.2.3) and
  the newest CPython. Once on Linux: fmt, clippy, `cargo deny check licenses`, ruff, mypy.
  (d) The Linux release wheel stays the VM's `tools/release_build.sh` build. CI also runs the script on Linux for the
  same commit and records whether its hashes equal the VM's (byte-identical expected with the same pins; a difference
  is recorded with the differing members, not a gate).
  (e) Linux-only code is gated: `t15_memory.rs` (`VmHWM`) and `tools/stream_memory.py` are compiled / run only for
  `target_os = "linux"`, with a skip message elsewhere.
  (f) A nightly or manual job: `tools/fuzz_run.sh <t> 60` for the three targets, `PROPTEST_CASES=10000` for
  `t14_damage`, nightly clippy of `fuzz/`, `cargo bench --bench decode` as a report; benchmarks gate only under
  ADR-66 (own baseline table, five runs within +-10 %).
  (g) The acceptance rule of ADR-53 item 4 per platform; the README lists only accepted platforms and moves the
  others to "Other platforms"; an accepted `manylinux2014` wheel replaces `manylinux_2_28` (from the next version).
  (Session 31, ADR-117: it ships **next to** `manylinux_2_28`; it does not replace it.)
  (h) Every test skipped on a platform is listed in the note with its reason; none without a decision record.
- Touches: the CI configuration (root), `tests/check_dist.py`, tests where a platform needs a fix (paths, `\r`,
  file locks), the `target_os` gates, `README.md`. Not `corpus/`.
- Needs: release run B done; Q-11. A Reviewer session reads T-16 before its wheels are released. Model: Sonnet.
- **Session 19 (ADR-75 item 8, ADR-76)**: Q-11 is answered in part: **GitHub Actions through a push mirror from the
  Gogs server**; the account is made later; public or private undecided (a private repository costs runner minutes
  for macOS / Windows and gives no Project-URL). T-16 starts when the mirror exists. Added: (i) T-18 (l): `tests/test_cli.py`
  green on every platform of (a) (paths, `os.replace` on Windows; the permission test skipped as root and on Windows,
  listed under (h)); (j) if the repository is **public** by then: `[project.urls]` `Source = "<url>"`, a
  `check_dist.py` rule that METADATA has `Project-URL: Source, <url>`, and its mutation case (O-19 (a)). Reviewed in
  **Reviewer session 6b**; its wheels ship in the release after 6b.
- **Session 20 (ADR-79)**: the repository exists: **private** `granddad-wolf/oradmp` on GitHub (remote `github` in the
  owner's clone; `master` and `v0.1.0a2` pushed). Q-11 is closed (GitHub Actions). What T-16 needs now:
  - from the owner: **O-21** (`CLAUDE.md` names `.github/` in the Implementer's row) and **O-22**: Actions enabled
    for the repository; `master` pushed to `github` after each commit to be tested (push mirror or by hand); the way
    the Implementer reads results (recommended: `gh` logged in on the VM with a fine-grained, read-only token for
    this repository's Actions and contents; the Implementer runs `gh run list` / `gh run view --log-failed` and never
    pushes); the visibility decision (public / private) whenever the owner is ready (it decides (j) and the trigger
    rule below);
  - in the task: the configuration in **`.github/workflows/*.yml`** (the Implementer's, ADR-79 item 2), with
    `permissions: contents: read`, no `pull_request_target`, third-party actions pinned to a full commit SHA, the
    toolchain from `rust-toolchain.toml`; triggers `push` and `workflow_dispatch`; while the repository is private
    the macOS and Windows jobs run on `workflow_dispatch` only (GitHub bills private minutes, a macOS minute at about
    ten times a Linux one; GitHub's Actions billing page), Linux on every push; when public, all on every push;
  - starts after the a3 tag is pushed; not at the same time as T-17 on the VM; the first CI run is (b)'s three risk
    checks, as before.
- **Session 21 (ADR-83)**: the a3 tag is pushed; **O-21 is done** (`9f1dc4d`); **O-22 is not**. T-16 starts after
  T-17 is committed and O-22 (a) to (c) are done (if O-22 is done before T-17 starts, T-16 goes first). What it needs:
  - from the owner, concretely (O-22): (a) Actions enabled for `granddad-wolf/oradmp`; (b) `master` pushed to `github`
    after each commit to be tested; (c) `gh` on the VM logged in with a fine-grained token for this repository only,
    read-only for Actions, contents and metadata (typed with `read -rs`, never in a file of the repository or a prompt),
    or the owner pastes failed logs; (d) the owner starts the macOS and Windows runs (`workflow_dispatch`) when the task
    asks, while the repository is private;
  - **fixtures**: CI uses the committed `corpus/fixtures` (16 MB, in every checkout); each job first checks the three
    `SHA256SUMS`;
  - **`corpus/work/`**: not in git and never sent to GitHub; `t15_memory` and `t15_throughput` stay `#[ignore]` and
    show as skipped in CI; memory, throughput and the baselines stay on the VM (benchmarks.md);
  - **T-18d's tests**: the non-UTF-8 name cases skip off Linux (reason "POSIX byte file names"), listed under (h);
  - in the note: the billed minutes of each dispatched run; the order of runs as ADR-83 item 5;
  - (j) applies only if the owner makes the repository public before T-16 ends; otherwise the Project-URL waits for
    the first task after that.
- **Session 22 (ADR-87): T-16 is split.** O-21 is done; O-22 (c) is answered: **the owner pastes the logs** (no token,
  `CLAUDE.md` rule 8). The items (a) to (j) above are distributed below; where they differ, this block wins.

### T-16a CI on GitHub Actions, Linux (session 22; ADR-79, ADR-83, ADR-87) - after T-17b is committed - DONE (2026-10-24, `c1d1c58` + `7a2d028` + `d081109`; open items in T-16a2; ADR-89)

- Goal: every push to `master` of `granddad-wolf/oradmp` runs the definition of done on a Linux runner, plus the
  installed-wheel tests and a release-script build, with a summary the owner can paste.
- Inputs: ADR-53, ADR-56, ADR-57, ADR-66, ADR-68, ADR-79 (with its amendments), ADR-83, ADR-87; `tools/release_build.sh`,
  `tools/fuzz_run.sh`, `tools/fuzz_seeds.sh`, `tests/check_dist.py`; the T-17 and T-17b notes (counts, skips); GitHub
  Actions documentation (workflow syntax, `permissions`, `workflow_dispatch`, `concurrency`, job summaries); the
  documentation of each action used, at the pinned commit.
- Needs from the owner, before the first push: (1) confirm Actions are enabled for the repository (O-22 (a)); (2) push
  `master` to `github` when the note asks (O-22 (b)); (3) paste what the note asks for (O-22 (c)): the run's web
  address, every job summary, and the failed step's log of a failed job; (4) dispatch the extended job once on the final
  commit.
- **Done when:**
  (a) **One workflow file** in `.github/workflows/` (more only if the note says why): `permissions: contents: read` at
  the top; no `pull_request_target`; triggers `push` (branch `master`) and `workflow_dispatch`; `concurrency` so a newer
  push cancels the older run of the same branch; `timeout-minutes` on every job; every third-party action pinned to a
  full commit SHA with its name and version in a comment; the Rust toolchain from `rust-toolchain.toml` (rustup);
  `CARGO_BUILD_JOBS=2` and `TMPDIR` on the runner's temporary folder (`runner.temp`); a named runner image (not
  `-latest`), recorded in the summary. Nothing names the repository, the account or a host (ADR-87 item 5): `grep -rn
  'granddad\|oradmp.git\|gogs' .github/` prints nothing.
  (b) **Linux job, every push**, in this order: the three `SHA256SUMS` checked; `cargo fmt --all --check`; `cargo clippy
  --all-targets -- -D warnings` and `--all-targets --all-features`; `cargo nextest run -j 2` (passed = the VM's count at
  the same commit, skipped exactly the 4 of the T-17 note: the three `t15_*` tests and `write_fuzz_seeds`, by name);
  `cargo deny check` (all four checks; an advisories failure is reported to the Architect in the note, never silenced
  or skipped); a probes build into a job venv (`maturin develop --features test-probes`), then `ruff check`, `ruff
  format --check`, `mypy`, `pytest -rs` (passed = the VM's count, 0 skipped; the runner user is not root, so the two
  root skips must not appear; if they do, the note says why).
  (c) **Release-script build, every push**: `tools/release_build.sh <the pushed commit>` on the runner; `check_dist: ok`;
  the SHA-256 of wheel and sdist in the summary. Once, for the final T-16a commit, the note compares them with the VM's
  build of the same commit: the sdist equal (ADR-81 item 3 (g)); the wheel equal or differing only in the SBOM member
  (ADR-70 item 5), shown by listing the differing members. A difference elsewhere is recorded with the members, not a
  gate (old (d)).
  (d) **Installed wheel, every push**: the wheel of (c) installed into fresh venvs: Python 3.11 with pyarrow 15.0.2 and
  pandas 2.2.3; the newest CPython the runner offers with the newest pyarrow and pandas; Python 3.11 with pyarrow only.
  `pytest -rs` from the checkout against the installed package (no `python/` on the path, the T-18c mechanism): 0 failed;
  skipped exactly the three probe tests of `tests/test_internal.py`, plus `tests/test_consumers.py` in the venv without
  pandas. Then `oradmp --version` prints `oradmp <the version at that commit>`.
  (e) **Risk checks (b) and (c) of old (b), once**: a `manylinux2014` wheel built with `--zig` (not shipped):
  `auditwheel show` output and the abi3 tag in the note. Old risk check (a) (line endings, paths, file locks) is
  T-16b's.
  (f) **Extended job, `workflow_dispatch` only while private** (old (f)): `tools/fuzz_seeds.sh` (root toolchain; it needs
  `cargo`, which the job has); `tools/fuzz_run.sh <t> 60` for `read`, `walk`, `values` with the fuzz toolchain;
  `PROPTEST_CASES=10000` for `t14_damage` and `PROPTEST_CASES=2000` for `t17_synth`; nightly clippy of `fuzz/`;
  `cargo bench --bench decode` as a report only (never compared with benchmarks.md). Findings 0 on the final commit; a
  finding follows ADR-62 item 7 (its input is synthetic, so the crash file may be a workflow artifact the owner
  downloads).
  (g) **Summary**: every job ends with a step that writes to the job summary (ADR-87 item 4): commit, runner image,
  rustc / cargo / Python / pyarrow versions, nextest and pytest counts with each skipped name and reason, the
  `check_dist` line, the SHA-256 of each built file, the job's duration. The note shows that one pasted summary is
  enough to fill the counts without the full log.
  (h) **Never**: an upload or publish step, a secret, a `corpus/work/` file (no artifact, cache or download of one), the
  memory or throughput tests run (they stay `#[ignore]` and are listed as skipped), a benchmark gate (ADR-83).
  `actions/cache` (or an equivalent pinned action) for the cargo registry and `target/` is allowed.
  (i) **Nothing else changes**: `git diff --stat <T-17b commit> <T-16a commit> -- . ':!docs'` lists `.github/workflows/`,
  `CONTRIBUTING.md` (a short "CI" paragraph: what runs on a push, what is dispatched, how to read a summary) and
  otherwise only files a CI failure proved wrong, each with a line in the note (the failing step, the cause, the fix);
  not `crates/oradmp-core/src/` unless the Architect agreed in the note first; not `corpus/`, not the version.
  (j) **The T-16a note**: each requested run (commit, what was pasted, result), the pasted summaries of the final run,
  the billed minutes the owner reports (or the job durations), the comparison of (c), the result of (e), the skip list
  per job with reasons (old (h)), and the VM's counts at the same commit.
- Touches: `.github/workflows/`, `CONTRIBUTING.md`, the note; proven fixes only. **Model: Sonnet.** On the VM: one
  build at a time (the local checks before each commit). Reviewed in **Reviewer 6c** with T-17b and the a4 candidate.

**Session 23 record (ADR-89; the owner's facts, checked against `ci.yml` and `git log`; 2026-10-24).** Commits:
`c1d1c58` (workflow, CONTRIBUTING, note), `7a2d028` (GIL test without timing, `nproc` lines), `d081109` (member
listing in the `wheel` summary). Last full run at `d081109b708b08f08cc6c965994b6c094b649b65`:

| job | result | about |
|---|---|---|
| `checks (Linux)` (push) | nextest 432 passed, 4 skipped (the four names); `cargo deny` ok; pytest 235 passed, 0 skipped | 3 min |
| `wheel (Linux)` (push) | `check_dist` ok; wheel `8405b25b...`, sdist `ed4dbf0f...`; pytest 232/3 (py311-pa15-pd223), 225/4 (py311-pyarrow-only), 232/3 (pynewest-latest), in the summary's sorted order | 4.3 min |
| `extended` (dispatch) | 435 seeds; fuzz `read` 58,010, `walk` 173,513, `values` 8,100,212 runs (about 61 s each), 0 findings; `t14_damage` 10,000 cases 5 passed; `t17_synth` 2,000 cases 20 passed, 1 skipped; fuzz clippy ok; bench report (`benchmarks.md` 7) | 10.6 min |
| `manylinux2014` (dispatch) | `cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64`, sha256 `18ff0533...` (not shipped); auditwheel: `manylinux_2_17_x86_64`; newest symbol `GLIBC_2.14`; installed nowhere | 3.5 min |

The counter-thread GIL test was green in three "Re-run all jobs" runs; the old timing test failed once (1.67 against
1.6). Accepted: (a), (b), (d), (e), (f), (g), (h). Open, in T-16a2: (c) the wheel at the same root (the sdist is equal:
the VM's build of `d081109` at `/var/tmp/oradmp-release-ci2` has sdist `ed4dbf0f8651a149...`, wheel `01dd8660...`;
the wheels differ in the SBOM member and `RECORD` only, owner); (i) the line for `tests/test_threads.py`; (j) the run
record with full hashes. The billed-minutes figure: O-23 (b).

### T-16a2 CI follow-ups: the canonical release root, the SBOM check, the fuzz artifact, the note (session 23; ADR-89, ADR-90, ADR-92) - DONE (2026-10-25, `af6a54a`; CI and VM hashes identical; ADR-98 item 1)

- Goal: T-16a's open items closed; release builds tied to one root and checked; a CI fuzz finding downloadable.
- Inputs: ADR-56, ADR-57, ADR-89, ADR-90, ADR-92; `tools/release_build.sh`, `tests/check_dist.py`,
  `tests/test_check_dist.py`, `.github/workflows/ci.yml`, `CONTRIBUTING.md`; the T-16a note; GitHub's documentation of
  `actions/upload-artifact` at the pinned commit (inputs `path`, `if-no-files-found`, `retention-days`).
- Needs from the owner: the **full** SHA-256 lines of the `wheel` summary of the `d081109` run and the SBOM line of its
  member listing (ADR-92 item 3); afterwards a push of T-16a2 and the pasted summaries of that run (one push run; one
  dispatch only if the artifact step must be shown, see (d)).
- **Done when:**
  (a) **First, before any change** (unless the owner reports it done, O-23 (a)): with `ORADMP_BUILD_ROOT` unset,
  `tools/release_build.sh d081109b708b08f08cc6c965994b6c094b649b65` at `/var/tmp/oradmp-release`; the note gives the
  wheel's full SHA-256 next to CI's and the SBOM member size next to CI's (expected 134,557); equal or the differing
  members; then `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-d081109`.
  (b) `tools/release_build.sh --release`: refuses (exit non-zero, message names both paths) any root whose `realpath -m`
  is not `/var/tmp/oradmp-release`; prints `release root: canonical`; `--help` text updated. Shell test or pytest
  case for the refusal (another root with `--release` fails; without `--release` it builds as before).
  (c) `check_dist.py --build-root <path>` as ADR-90 item 2; `release_build.sh` passes `--build-root "$ROOT"` on every
  build; cases in `tests/test_check_dist.py` (other root: FAIL with the byte offset; right root: pass; option absent:
  unchanged result). Mutation: the prefix test accepts any root: the "other root" case fails. The script prints
  the root `check_dist` was given (`check_dist: build root <path>`), so a missing argument shows in every log.
  (d) `ci.yml`: the `wheel` job calls `tools/release_build.sh "$GITHUB_SHA" --release` and its summary prints the
  `release root:` line; the `extended` job ends with the artifact step of ADR-92 item 1 (`if: failure()`, the action
  pinned by full SHA with a version comment, `path: fuzz/artifacts/`, `if-no-files-found: ignore`, `retention-days:
  14`); `auditwheel` pinned in the `manylinux2014` job (the version that run printed, or the current one, recorded);
  nothing else in the workflow changes. The artifact step is shown once on the VM by the YAML parse and `bash -n` of the
  blocks; a dispatched run is not needed for it.
  (e) `CONTRIBUTING.md`: "Release wheel" names `--release` and the canonical root (one sentence, the claim of ADR-90
  item 6 as it stands at the commit); "CI" names the artifact on a fuzz failure (one sentence).
  (f) The T-16a note completed (in a "T-16a2 note" that refers to it; the T-16a text stays): the runs of `c1d1c58`,
  `7a2d028`, `d081109` with what was pasted; the line for `tests/test_threads.py` (step `pytest`, cause: a timing ratio
  on a shared 2-vCPU runner, fix: a counter thread that must advance during a read, no timing); the result of (a).
  (g) One push run of the T-16a2 commit green; its counts equal the VM's at that commit (pytest grows by the new
  `check_dist` cases only); the sdist hash of that run equals the VM's.
- Touches: `tools/release_build.sh`, `tests/check_dist.py`, `tests/test_check_dist.py`, `.github/workflows/ci.yml`,
  `CONTRIBUTING.md`, the note. Not `crates/`, `python/`, `corpus/`, the version. **Model: Sonnet.** One build at a
  time. Reviewed in **Reviewer 6c**.

### T-16b Other platforms: macOS, Windows (session 22; ADR-53, ADR-87; made exact in session 31, ADR-118) - next (after the session 31 docs push); the "Session 31" block below wins

- Goal: tested wheels for macOS arm64 and x86_64 and Windows x86_64 (Linux aarch64 optional); accepted ones ship in a5.
- Needs from the owner: while the repository is private, he dispatches the macOS and Windows jobs when the note asks; at
  most **three** dispatched runs without asking him again; if it is public by then (Q-14), they run on every push.
- **Done when:** old (a) for these platforms, as jobs of the T-16a workflow; old (b) risk check (a) (line endings, paths,
  file locks) as the first dispatched run, its result in the note; old (c) on each platform (checksums, `cargo nextest
  run`, wheel and sdist built with path remaps, `check_dist.py` passing on every wheel, the wheel in fresh venvs on
  Python 3.11 with pyarrow 15.0.2 and pandas 2.2.3 and on the newest CPython, `pytest -rs`); old (e) (`t15_memory.rs` and
  `tools/stream_memory.py` gated to `target_os = "linux"` with a skip message); old (g) (the acceptance rule of ADR-53 item
  4 per platform; the README lists accepted platforms only; the T-18d README sentence names Linux if a non-Linux
  platform is accepted); old (h) (every skip listed with its reason and record: `tests/test_paths.py` off Linux, "POSIX
  byte file names"; the permission tests on Windows; anything else needs a line in the note and an Architect answer);
  old (i) (`tests/test_cli.py` green on every platform; `os.replace` on Windows). The note records the billed minutes of
  every dispatched run. The extended job stays Linux only.
- Touches: `.github/workflows/`, `tests/check_dist.py`, the `target_os` gates, tests a platform proves wrong, `README.md`.
  **Model: Sonnet.** Reviewed before a5 by the Reviewer session that reads a5's candidate.
- **Session 23 (ADR-93 item 4)**: still after the a4 tag. It does not wait for the cut; with the cut before a4 it runs
  on the public repository (standard runners free; every job on every push). If still private: dispatch only, at most
  three dispatched runs, within "CI minutes budget (session 23)"; macOS costs about ten Linux minutes per minute. First
  check: which macOS x86_64 runner image GitHub still offers (runner-images list); none means macOS x86_64 moves to
  "builds, not supported" or out of the matrix, with a line in the note and an Architect answer. The paste rule of
  ADR-92 item 3 applies.
- **Session 29 (ADR-112 item 5, ADR-113 item 3)**: after T-21 and T-16c are pushed (all three edit `ci.yml`). Adds:
  the sdist built from source and installed in a fresh venv on one Linux runner (in `extended`, so a push stays short),
  with `oradmp --version` and `python -m oradmp survey --no-names` on a copied fixture. Accepted wheels ship in a5.
- **Session 31 (ADR-117 item 2, ADR-118, ADR-119; this block wins wherever the text above differs).** T-16b is a **test
  task**. Nothing it builds is published or kept. Whether an accepted platform's wheel ever ships is **Q-15**, the
  owner's question.
  - **Inputs** add:
    - ADR-116 to ADR-119;
    - `ci.yml` at `0bfa70d` (the `wheel` job's venv logic in `tools/ci_install_check.sh`);
    - `tests/check_dist.py`;
    - GitHub's runner-images list (README of `actions/runner-images`) and the "GitHub-hosted runners" reference (which
      labels are standard runners for public repositories);
    - pip's `pip download` documentation (`--platform`, `--python-version`, `--only-binary`);
    - maturin 1.15.0's `build` documentation.
  - **Needs from the owner**:
    - the commits and pushes;
    - at most **three** dispatches on T-16b commits before the Architect is asked again. Each dispatch also runs
      `extended` and `manylinux2014`. The first dispatch is the risk check;
    - the pasted summaries of the push run and of each dispatch;
    - the machine alone for (i).

    No tag, no release, no token, no Docker on the VM.
  - **Done when:**
    (0) **Check first, no CI.**
    - (0a) **Runner images.** From GitHub's runner-images list, record the image label used for macOS arm64, macOS
      x86_64 and Windows x86_64, each a named version, never `-latest`. macOS x86_64 counts only if it is listed as a
      standard runner for public repositories. Otherwise its verdict is "not tested: no runner", and the matrix leaves
      it out.
    - (0b) **pyarrow per platform.** In a scratch venv outside the repository (rule 9), run
      `pip download --no-deps --only-binary :all: --platform <p> --python-version <v> -d <scratch> "pyarrow>=15"`,
      and the same for `numpy` and `pandas`:
      - `<p>`: the platform tag that matches each runner image's macOS version (for example `macosx_15_0_arm64` and
        `macosx_15_0_x86_64`), and `win_amd64`;
      - `<v>`: 3.11 and the newest CPython on the runner;
      - also `pyarrow==15.0.2` and `pandas==2.2.3` for 3.11.

      The note records each downloaded file name, or pip's "No matching distribution" line.
      - **No pyarrow 15 or later for cp311** on a platform ends that platform "rejected: no pyarrow wheel".
      - **No pyarrow 15.0.2 there**: venv (i) uses the oldest pyarrow 15 or later that has a wheel, named in the note.

      The scratch folders are deleted afterwards.

    (a) **The `platforms` job**: one matrix over the platforms (0a) kept, `workflow_dispatch` only, `timeout-minutes`
    set, `permissions: contents: read`, `persist-credentials: false`, every action pinned by full SHA with a version
    comment, `CARGO_BUILD_JOBS=2`, `TMPDIR` on `runner.temp`. **No artifact, no upload.** In order:
    1. fixture checksums by a Python step;
    2. the toolchain from `rust-toolchain.toml`;
    3. `cargo nextest run -j 2` (the version pinned as in `checks`): passed = Linux's count, skipped exactly the four
       ignored tests by name;
    4. wheel and sdist built **natively** with `maturin==1.15.0 build --release --locked` (no `--zig`) and
       `RUSTFLAGS` `--remap-path-prefix` for the cargo home and the checkout, as `release_build.sh` does. The checkout is
       copied to `<root>/src` so that `check_dist.py --build-root <root>` applies its SBOM rule unchanged;
    5. `check_dist.py` passes on the wheel and the sdist with **no rule weakened or skipped**. A rule that fails for a
       platform reason is a note line and needs an Architect answer before any change;
    6. the wheel is built a **second time** into another folder, and the job prints both SHA-256 values and
       `identical` / `differs: <members>` (ADR-118 item 7; recorded, not a gate);
    7. three fresh venvs, every install `--only-binary :all:` with `pytest==9.1.1`:
       - (i) 3.11, pyarrow 15.0.2 (or (0b)'s substitute) and pandas 2.2.3;
       - (ii) the newest CPython the runner offers, with the newest pyarrow and pandas;
       - (iii) 3.11 with pyarrow only;
    8. in each venv, `oradmp --version` = the commit's version, then full `pytest -rs` from a copy of the checkout
       against the installed package: 0 failed;
    9. the summary: image, `rustc`, the Python, pyarrow, numpy and pandas versions per venv, the counts and every skip
       with its reason, the `check_dist` line, both wheel hashes, the sdist hash, and the duration.

    (b) **Allowed skips**:
    - the three probe tests;
    - `tests/test_consumers.py` where pandas is absent;
    - `tests/test_paths.py` off Linux ("POSIX byte file names");
    - permission tests on Windows, by name;
    - the Linux-only memory test and `tools/stream_memory.py` (gated by `target_os = "linux"` or `sys.platform`, each
      with a skip message).

    Any other skip fails the step, and its fix needs a note line with an Architect answer.

    (c) **First dispatch = the risk check** (ADR-53 item 7 (a)): line endings (`.gitattributes`, ADR-52), paths built as
    strings, `\r` in text reads, Windows file locks (the `FileChanged` tests, temporary copies), and
    `tests/test_cli.py`, which must be green on every platform (`os.replace` on Windows). Each failure goes in the note
    with its step, cause and fix:
    - only test code, `tests/check_dist.py` or the `target_os` gates may change without asking;
    - a change in `crates/*/src/` or `python/oradmp/` stops the task until the Architect answers in the note.

    (d) **Verdict per platform** (the note's first lines): "accepted" (ADR-53 item 4 met in every venv), "builds, not
    supported: <reason>", "rejected: <reason>" or "not tested: no runner".

    (e) **The sdist check in `extended`** (ADR-112 item 5):
    - build the pushed commit with `tools/release_build.sh "$GITHUB_SHA"` at a root under `runner.temp`, without
      `--release`; its sdist hash is printed and must equal the `wheel` job's;
    - `uv pip install` that `.tar.gz` into a fresh Python 3.11 venv (a source build);
    - `oradmp --version`; `python -m oradmp survey --no-names <copy of corpus/fixtures/18c/t_text_exp_18c.dmp>` exits 0.

    (f) **Nothing else changes**:
    - the `checks`, `wheel`, `fuzz-check` and `manylinux2014` jobs stay as they are;
    - the push run's `wheel` member list equals T-16c's push run on `94b6368`, except the SBOM and `RECORD` at equal
      size;
    - not `README.md`, `pyproject.toml`, `Cargo.*`, the version, `corpus/`.

    (g) **Checks** (definition of done):
    - fixture checksums; fmt; clippy (two forms); nextest 488 + 4 (unchanged unless (c) adds tests); `cargo deny` with
      the expected warnings of ADR-112 item 2;
    - `maturin develop --features test-probes`; ruff, ruff format, mypy; pytest 266 + any new cases, 0 skipped;
    - YAML parse and `bash -n` of every `run` block;
    - the substitution rule (ADR-109 item 3) and `pipefail` (`defaults.run.shell: bash` covers the Windows runner too;
      PowerShell steps are not used).

    (h) **The rule for tests that depend on machine state** (ADR-119 item 2) holds for every new or changed test.

    (i) **The 2014 pre-check** (ADR-117 item 2), on the VM, once, alone on the machine:
    1. Run `tools/release_build.sh 94b6368 --twice --release --manylinux 2014` at the canonical root (it must be absent
       first; if it exists, stop and ask the owner).
    2. Pass: the two builds are identical; the wheel = `4327e1a72817fdb7f807da800cf2b32eddc4f41d6e9fc3418b84b1494c7c5e44`;
       the sdist = `0e2dd8866e5fb2b83bdad54a51559c5a5e5ed753461d7dadfef55208c91f3755`.
    3. Then `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-t16c-2014`, put its `SHA256SUMS` in the note, and
       delete the folder.
    4. A difference: the member listing (sha256, size, name) goes into the note. Stop for the 2014 part only.

    (j) **The note** ("T-16b note") is part of the commit:
    - (0)'s output, the VM checks and (i);
    - after each dispatch, a docs-only commit with the summary figures, the verdicts and each job's duration.
  - **Not in scope**:
    - Linux aarch64, Windows arm64, musllinux, `universal2`;
    - a release route for these wheels (Q-15), artifacts, porting `release_build.sh`, cross-builds on the VM;
    - benchmarks, the memory test and fuzzing off Linux;
    - README, classifiers, the version.
  - **Touches**: `.github/workflows/ci.yml` (the new `platforms` job and the `extended` sdist step only);
    `tests/check_dist.py` only for a proven platform reason; tests a platform proves wrong; the `target_os` /
    `sys.platform` gates; a shared `tools/` script if needed; the note. **Model: Sonnet.** Reviewed in Reviewer 7.

### T-16c `manylinux2014` acceptance (session 23; ADR-53 items 4, 5; ADR-91; made exact in session 30, ADR-115) - DONE, **accepted** (2026-10-31, `a8da2cc` + `94b6368`, results `0bfa70d`; nextest 488 + 4, pytest 266; dispatch 1 of 3 green; ADR-116; to be reviewed in Reviewer 7)

- Goal: decide on evidence whether the Linux wheel can move from `manylinux_2_28` to `manylinux2014` (glibc 2.17).
- Inputs: ADR-53, ADR-56, ADR-90, ADR-91; `tools/release_build.sh`, `tests/check_dist.py`, `ci.yml`; PyPA's
  documentation of `auditwheel` and of the manylinux images; maturin's `--compatibility` documentation of the pin.
- Needs from the owner: dispatches while private; the Rocky Linux 8 run of (c) (as for a3).
- **Done when:** (a) to (d) of ADR-91 item 2; the note records the image digest, `ldd --version`, the interpreters
  found, the pyarrow and numpy versions installed, the counts, and the verdict "accepted" or "rejected: <reason>". A
  rejection is a valid end of the task. No release file changes in T-16c; an accepted 2014 wheel ships in a5.
- Touches: `tools/release_build.sh`, `.github/workflows/ci.yml`, the note; `README.md` only if accepted. **Model:
  Sonnet.** Reviewed with T-16b.
- **Session 29 (ADR-113 item 3)**: after T-21 is pushed, **before** T-16b. **(0) First check, no CI**: whether pip on a
  glibc 2.17 system can install pyarrow at all: `pip download --no-deps --only-binary :all: --platform
  manylinux2014_x86_64 --python-version 3.11 -d <scratch> "pyarrow>=15"` (and the same with `--python-version 3.14`);
  the note records the newest pyarrow version each finds, or that none is found. None for 3.11: T-16c ends "rejected:
  no pyarrow wheel for glibc 2.17" (a valid end; a 2014 wheel of oradmp is useless without one). Otherwise (a) to (d)
  as above, with the pyarrow version found in (0) in the 2.17 venv. The README is not touched in T-16c (an accepted
  wheel's README lines go into a5's version commit).
- **Session 30 (ADR-115; this block wins where the text above or ADR-91 item 2 differs)**.
  - **Inputs** add: ADR-114, ADR-115; `tests/test_release_build.py`; the `wheel` and `manylinux2014` jobs of `ci.yml`
    at `20bbc80`; PyPA's `manylinux` README (the `manylinux2014` image); pip's documentation of `pip download`
    (`--platform`, `--python-version`, `--only-binary`).
  - **Needs from the owner**: the commit and one push to `origin`; one `manylinux2014` dispatch on the pushed commit (at
    most three before asking the Architect); the pasted summaries of the push run's `wheel` job (with its member list)
    and of the dispatch. Nothing on the VM with Docker (the Rocky Linux 8 run is in CI now; the VM run as in a4 step 9 is
    the fallback only if (c) says so). No tag, no release, no token.
  - **Done when:**
    (0) **Check first, no CI, no build.** In a scratch venv outside the repository (`uv venv --seed
    /var/tmp/oradmp-t16c-pip`, rule 9), for `--python-version` 3.11, 3.13 and 3.14: `<venv>/bin/python -m pip download
    --no-deps --only-binary :all: --platform manylinux2014_x86_64 --python-version <v> -d <scratch> "pyarrow>=15"`, and
    the same for `numpy` and `pandas`. The note records the file each command downloads (or pip's "No matching
    distribution" line). Expected from PyPI's index as read in session 30: cp311 `pyarrow` 20.0.0, `numpy` 2.2.6,
    `pandas` 2.3.2; cp313 `pyarrow` 20.0.0; cp314 none of the three. A different result is a line in the note, not a
    stop, unless it is the rejection: **no pyarrow 15 or later for cp311** ends T-16c "rejected: no pyarrow wheel for
    glibc 2.17": the commit holds the note only (its first line that verdict, then the commands and their output); no
    change to `tools/`, `tests/` or `ci.yml`; `manylinux_2_28` stays; the Architect records it. No cp314 wheel is not a
    rejection (it limits the claim, (e)). The scratch folders are deleted afterwards.
    (a) **Release script**: `tools/release_build.sh [COMMIT] [--twice] [--release] [--manylinux 2014|2_28]`, default
    `2_28` (the line maturin gets is unchanged for it); `2014` gives `--compatibility manylinux2014`; any other value or
    a missing value exits 2 with a message naming the value, before any folder is made. After each build the script
    fails unless the wheel name holds `-cp311-abi3-` and the platform tag asked for (`manylinux_2_28_x86_64` resp.
    `manylinux_2_17_x86_64.manylinux2014_x86_64`), and it prints `manylinux: <value>`; `--help` names the option.
    `tests/test_release_build.py` gains cases (as its existing ones, with a commit that does not exist so nothing is
    built): a bad value and a missing value exit 2 before "no such commit"; `--help` names `--manylinux`. Mutation: the
    value check removed, the bad-value case fails. On the VM, once, alone on the machine:
    `ORADMP_BUILD_ROOT=/var/tmp/oradmp-t16c-build tools/release_build.sh HEAD --manylinux 2014` (no `--release`: the
    VM's canonical root is kept free for a5) gives `check_dist: ok` and the 2014 wheel name; the folder is deleted.
    (b) **CI, the `manylinux2014` job** (still `workflow_dispatch` only, still never publishing): builds with `bash
    tools/release_build.sh "$GITHUB_SHA" --release --manylinux 2014` (canonical root on the runner, as the `wheel`
    job); `auditwheel show` (6.8.2, pinned) consistent with `manylinux_2_17`; both `Tag:` lines `cp311-abi3-`; then:
    - **glibc 2.17**: `docker run` of **`quay.io/pypa/manylinux2014_x86_64@sha256:<digest>`** (the digest written in
      `ci.yml` and the note), the checkout and the `dist/` folder mounted read-only, the checkout copied inside to a
      work folder; `ldd --version` first line must say 2.17 (else the step fails); `ls /opt/python`. Four venvs from
      `/opt/python/cp3XX-cp3XX/bin/python`, every install `--only-binary :all:` with `pytest==9.1.1`: (i) 3.11, pyarrow
      15.0.2, pandas 2.2.3; (ii) 3.11, the newest cp311 pyarrow and pandas of (0); (iii) 3.13, the newest cp313 pyarrow
      of (0), no pandas; (iv) 3.14: only the pyarrow install is tried, its failure is printed and does not fail the step
      (if it succeeds, (iv) runs as (iii) and the note says so).
    - **glibc 2.28**: `docker run` of **`rockylinux:8@sha256:<digest>`**: `dnf install -y python3.11 git`, `uv` (pinned
      as in the `wheel` job) for a 3.14 interpreter; the `wheel` job's three venvs (3.11 + pyarrow 15.0.2 + pandas
      2.2.3; 3.14 + newest; 3.11 + pyarrow only) with the 2014 wheel. If `dnf` or `uv` cannot provide an interpreter in
      CI, the step says so and fails; the owner then runs the same three venvs on the VM as in a4 step 9 and pastes the
      counts (the only owner Docker run of the task).
    - **The runner**: the `wheel` job's three venvs with the 2014 wheel (`run_one` may move to a script under `tools/`
      used by both jobs; the `wheel` job's behaviour does not change).
    - In every venv: `oradmp --version` = the version of the commit; `oradmp` is the installed package; full `pytest -rs
      -q -p no:cacheprovider` from the copied checkout (a git clone; `git config --global --add safe.directory` when
      root), 0 failed. Allowed skips: the three probe tests; in the containers (root) the three root-only tests
      (`test_cli.py` two, `test_paths.py` one, as a4 step 9); `tests/test_consumers.py` where pandas is absent. Any other
      skip fails the step (the `wheel` job's skip check, extended by the root list).
    - The job summary: image digests, `ldd --version` of both containers, the interpreters found, per venv Python,
      pyarrow, numpy and pandas versions and the last pytest line, the 3.14 outcome on 2.17, the wheel's SHA-256 and
      member list. The 2014 wheel is **not** uploaded as an artifact.
    (c) **The default route unchanged**: the push run's `wheel` job still says `manylinux: 2_28`, `check_dist: ok`; its
    member list equals that of T-21's run on `20bbc80` except the SBOM and `RECORD` (equal size) and sdist members whose
    path T-16c changed (if any). Any other difference: stop, the Architect rules.
    (d) **Checks** (definition of done): fixture checksums; fmt; clippy (two forms); nextest 488 passed + 4 skipped
    (no Rust change); `cargo deny` (four ok, the expected warnings of ADR-112 item 2); `maturin develop --features
    test-probes`; ruff, ruff format, mypy; pytest 262 + the new cases, 0 skipped; YAML parse and `bash -n` of every
    `run` block of `ci.yml` (as T-21).
    (e) **Verdict** (first line of the note): "accepted" when (a) to (c) pass and every venv of (b) has 0 failed with
    only the allowed skips; the note then states the claim a5's README would make: "glibc 2.17 or later; below glibc
    2.28, CPython 3.11 to 3.13 and pyarrow up to <newest of (0)>". Otherwise "rejected: <reason>" (e.g. pyarrow does
    not import on 2.17, a test fails only there). Both are valid ends.
    (f) **Nothing published**: no tag, no version change, no README change, no upload to PyPI or TestPyPI, no GitHub
    release, no `gh`, no token. The Implementer prepares the change and the note; the owner commits and pushes.
    (g) **The note** ("T-16c note" in `implementer-notes.md`) is **part of the commit** (ADR-114 item 4 (c)): (0)'s
    output, the VM build line, the mutation, the counts of (d), and after the dispatch the summary figures of (b) and
    (c) (added by the Implementer in a second docs-only commit if the dispatch comes after the first commit).
  - **Touches**: `tools/release_build.sh`, `tests/test_release_build.py`, `.github/workflows/ci.yml` (the
    `manylinux2014` job; a shared `tools/` script if `run_one` moves), the note. Not `README.md`, `pyproject.toml`,
    `Cargo.*`, `crates/*/src`, `python/`. **Model: Sonnet.** Reviewed with T-21 and T-16b (Reviewer 7).
- **Session 31 record** (ADR-116). S marks the facts as pasted and the note's "Results"; E marks what was checked in
  session 31.
  - **Commits** (E, `git show --stat`):
    - `a8da2cc`: 7 files. `ci.yml`; `tests/test_release_build.py`; `tools/release_build.sh`; the new
      `tools/ci_install_check.sh`, `ci_container_2_17.sh` and `ci_container_rocky8.sh`; the note.
    - `94b6368`: the test and the note.
    - `0bfa70d`: the note only.
  - **The first `wheel` job on `a8da2cc` failed** (S). The test assumed that the canonical root was absent. `94b6368`
    accepts "exists already" or "no such commit", both with exit 1, and skips neither. The script is unchanged; it
    checks the root at line 114 before the commit at line 116 (E).
  - **Push run on `94b6368`**, green (S):
    - `checks`: nextest 488 + 4; `cargo deny` ok; pytest 266, 0 skipped.
    - `fuzz-check`: ok.
    - `wheel`: `check_dist: ok`; venvs 263 + 3, 256 + 4, 263 + 3.
      - wheel `df3be632...`, sdist `0e2dd886...`;
      - `_core.abi3.so` `d0efa2df...` (= a4's);
      - 69 members: 16 in the wheel, 53 in the sdist.
  - **Dispatch 1 of 3 (`manylinux2014`)**, green, 1141 s (S):
    - wheel `oradmp-0.1.0a4-cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.whl`
      `4327e1a72817fdb7f807da800cf2b32eddc4f41d6e9fc3418b84b1494c7c5e44`;
    - `_core.abi3.so` `aa6ef3ac7593febc23cc9384eb535f5c9a9469975a5809b5be77a691fa666255`;
    - sdist = the `wheel` job's;
    - `auditwheel` 6.8.2: `manylinux_2_17_x86_64`, up to `GLIBC_2.14`.

    | where | venvs (passed + skipped) |
    |---|---|
    | glibc 2.17 | 260 + 6, 260 + 6, 253 + 7 |
    | Rocky 8 | 260 + 6, 253 + 7, 260 + 6 |
    | runner | 263 + 3, 256 + 4, 263 + 3 |

    CPython 3.14 on glibc 2.17 has no pyarrow, as expected.
  - **Verdict "accepted"** (ADR-116 item 1).
    - Held for Reviewer 7 (ADR-116 item 2): (c)'s member-by-member comparison with T-21's run, including the SBOM and
      `RECORD` sizes; the package versions of the container venvs.
    - The VM-against-CI identity of the 2014 wheel is ADR-117 item 2, done in T-16b (i).

### Reviewer 7 on T-21, T-16c, T-16b (session 31; ADR-119) - after T-16b's results commit is pushed, before the a5 version commit

- **Scope**: T-21 (`20bbc80`); T-16c (`a8da2cc`, `94b6368`, `0bfa70d`); T-16b (its commits). It reads the code, the
  tests, `ci.yml`, the `tools/ci_*.sh` scripts and the notes, together with the public run summaries the owner pastes.
- **Done when**: `docs/review/review-07.md` covers ADR-119 item 3:
  - (a) T-21: the SBOM and `RECORD` sizes; the assignments in `ci.yml`.
  - (b) T-16c: ADR-116 item 2; the option parsing; the three container and install scripts (digests,
    `--only-binary :all:`, exact skip lists, `pipefail` and the substitution rule, no secret, no artifact).
  - (c) T-16b: the `platforms` job; the verdict per platform; the `extended` sdist check; the twice-built measurement;
    the 2014 pre-check.
  - (d) The **machine-state inventory**: the grep of ADR-119 item 3 (d), each hit classified (i), (ii) or (iii) of the
    rule below. Both states of `tests/test_release_build.py` are shown from one push run's `checks` and `wheel`
    summaries.
- **Rule for tests that depend on machine state** (ADR-119 item 2; it binds every task from T-16b on). A test whose
  result depends on state outside its own temporary folder must do one of three things:
  - (i) make that state itself (in `tmp_path`, or through an override such as `ORADMP_BUILD_ROOT`);
  - (ii) assert the expected result for each state it can meet;
  - (iii) skip with a named reason that a decision record allows.

  Such state includes a fixed path, the uid, a git clone, an optional package, an environment variable, the platform.
  A test never creates, writes or deletes a fixed shared path such as `/var/tmp/oradmp-release`.
- **Builds**: only in the Reviewer's own scratch folder under `/var/tmp`, never at the canonical root. **Model: Opus.**
  The owner starts it.
- If T-16b stalls, the owner may start it on T-21 and T-16c alone. T-16b is then read in the a5 closure review
  (ADR-119 item 1).

## T-19 The conventional LONG forms of R-040; the ten session 17 fixtures in the tests (session 24; ADR-94, ADR-95, ADR-97) - DONE (2026-10-25, `921971f`; nextest 463 passed + 4 skipped, pytest 250, CI green on `bbf921b`; deviations ruled by ADR-102; item (m) not run, now **T-19 (m)** after T-19b; to be reviewed in Reviewer 6c)

- Goal: the reader follows rules v3.7: every conventional LONG form the fixtures show reads; the cases R-048 asks about
  stop with a named error; every test over the corpus covers the 211 files; the generator and the damage tests cover the
  new forms.
- Inputs: ADR-15 and ADR-12 (amended), ADR-80 (amended), ADR-86 (amended), ADR-94, ADR-95, ADR-97; rules v3.7 (R2, R8, Y9,
  T5, Y5, E1 to E3); `exp.md` 6.4, 6.6 "Session 17", 5.4; `docs/formats/tools/exp_parse.py` (`long_chunks`),
  `long3_check.py`, `tsp_check.py`, `corpus_counts.py`, `rowlen_stats.py`; the five new truth files;
  `crates/oradmp-core/src/exp/rows.rs`, `src/reader.rs`, `src/dump.rs` (`LongStats`), `src/error.rs`; the tests named
  below; `tests/common/synth.rs`, `t17_synth.rs`; `tools/fuzz_seeds.sh`, `tools/fuzz_run.sh`; `python/oradmp/_api.py`,
  `_survey.py`, `_errors.py`; README; `benchmarks.md` section 5.
- Needs from the owner: the commit; one push and the pasted summaries (ADR-92 item 3); the machine alone for (m).
- Every item has a test or check that **fails at `8162eb0` (or under the named mutation) and passes on the T-19 commit**;
  the note's table names it. Mutations each alone, restored after each, `git status` clean after each.
- **Done when:**
  (a) **Reader** (`exp/rows.rs`, ADR-95 item 1 table): with path flag 0, a type 8 field: `fe ff` NULL; 1 to 0xfff0 one
  field; `fd ff` the chunked form (ADR-95 item 2: chunk 0 ends, 1 to 0xffef bytes, from 0xfff0 `E-FRAMING` "conventional
  LONG chunk length from 0xfff0"); 0xfff1 to 0xfffc `E-UNSUPPORTED-LAYOUT` "conventional LONG length 0xfff1 to 0xfffc, not
  in the fixtures (R-048)". A type 24 field: 0x8000 to 0xfffd `E-UNSUPPORTED-LAYOUT` "conventional LONG RAW length from
  0x8000 or chunked form, not in the fixtures (R-048)". 0, `ff ff` and every other column unchanged; the direct path
  unchanged. Offsets, table, row, column, `length` as ADR-95 item 3. The scan passes chunks by length; the sink gets
  chunks as ADR-95 item 6; `W-UNTESTED-LAYOUT` for a chunked total below 65,535 (item 2); `max_lob_bytes` per chunk
  (item 7); `LongStats.chunked` (item 8); `W-LONG-LENGTH` thresholds and messages (item 9); the raw digest (item 10).
  (b) **The four `long3` files** (new tests in a T-19 test file): for each of `long3_exp(_direct)_<v>.dmp`, T_LONG3's 7
  rows: the length, first 20 and last 20 bytes of L and the text of V equal `truth_long3.txt` (parsed from the file, the
  100,000-byte row included), and the whole L equals `L` + the 6-digit size + `y` x (size - 7), built by the test from
  the truth length (R-040's arithmetic, not reader output); T_FRAME's rows equal the T_FRAME truth the existing checker
  uses; `LongStats` of L: conventional `values 7, chained 0, chunked 3, largest 100000, above_32000 7`; direct `values 7,
  chained 7, chunked 0, largest 100000, above_32000 7`; no warning at all with default options and with `strict=True`; the
  assembled LONG digest (`long_sha256`) of T_LONG3 is the same in all four files. Mutations: (M1) the chunk loop stops
  after the first chunk; (M2) a plain length read as `n & 0x7fff`; (M3) the plain range ends at 0xffef; (M4) the `00 00`
  after the last chunk not consumed; (M5) the `W-LONG-LENGTH` threshold for LONG back to 32,000: each fails a test of
  (b) by name.
  (c) **The R-048 stops** on patched copies (the patched-copy helper of `t6_patched.rs`, copies below `target/` only):
  `long3_exp_<v>.dmp` with the u16 at 0x2228f (`f0 ff`, row 4) set to `f1 ff`, and to `fc ff`: the scan ends `stopped`
  with `E-UNSUPPORTED-LAYOUT` at 0x2228f, table 1, row 4, column 2, `length` 0xfff1 / 0xfffc, the exact message; no table
  listed after it. `long_exp_<v>.dmp` with the u16 at 0xa0ea (`00 7d`, T_LONGRAW row 2) set to `00 80`, `f0 ff` and `fd
  ff`: stopped with the LONG RAW message at 0xa0ea, table 2, row 2, column 2; T_LONG (table 1) reads its 3 rows. A
  synthetic or patched chunked value with a chunk u16 `f0 ff` gives `E-FRAMING` at that u16. Python (`tests/`): the same
  patched copy (written to `tmp_path`) raises `oradmp.UnsupportedError` with `.code == "E-UNSUPPORTED-LAYOUT"` and
  `.offset == 0x2228f`; `oradmp tables <copy>` exits 1 with that code, the message and the offset on stderr. Mutations:
  (M6) 0xfff1 to 0xfffc read as a plain length; (M7) LONG RAW treated as LONG; (M8) chunk u16 from 0xfff0 accepted:
  each fails its test.
  (d) **Memory guard on 100,000 bytes**: with `max_lob_bytes=65_536`, T_LONG3 row 7 gives `E-LOB-TOO-LARGE` at the
  value's first data byte with `length` 68,000 (conventional: 17 chunks of 4000) and 71,568 (direct: 9 pieces of 7952),
  rows 1 to 6 read; with `on_value_error="null"` as well, row 7's L is NULL, V is `v100000`, T_FRAME reads, one
  `W-VALUE-NULLED`. With `max_lob_bytes=100_000` row 7 reads. Mutation (M9) the bound checked only at the value's end:
  the `length` assertion fails.
  (e) **The other new fixtures**: `tsp_exp(_direct)_<v>`: the descriptors (type 180, length 11 for T1 to T8), no
  `W-DDL-FALLBACK`, Arrow Timestamp(Microsecond) for T1 to T5 and Timestamp(Nanosecond) for T7, T8 (arch 5.1), every value
  of the 4 rows equal to the decimal `DUMP` of `truth_tsp.txt` decoded by the test's own [Y2] / [Y5] arithmetic and to the
  `TO_CHAR ... FF9` lines (truncated to the unit), row 4 NULL. `seg_exp(_direct)_18c`: 3 tables with 0 rows each (=
  `truth_seg.txt`), the end markers E1, E1, E1 (conventional) and E1, E1, E2 (direct, T_SEG2 by [E2]).
  (f) **Counts and golden files**: `tests/common/mod.rs` `TOTALS` and `FOLDER_TOTALS` from the Analyst's tools
  (`corpus_counts.py`, `rowlen_stats.py`: 211 files, 95 / 89 / 27, 404 sections, 418 parts, 41,284 rows, 243,303 fields,
  19,960 NULL; the other fields as the tools print them), never from the reader; the truth listing 87; every literal
  `201` in tests and generators replaced by the catalog's own count or the new figure. The generators
  (`tests/golden/gen_exp_catalog.py`, `gen_exp_raw.py`, `crates/oradmp-core/tests/data/gen_walk_expected.py`,
  `gen_number_parts.py`) take the new files; `gen_exp_raw.py` serialises `long_chunks` of `exp_parse.py` as ADR-95 item
  10; all golden and expected files regenerated by them, none edited by hand. Every test that lists the fixtures passes
  with 211; `t3a`'s "all files walk to the second `EXIT`, none stops" and `t11_strict`'s documented list include the ten
  files. Mutation (M10) chunks serialised with the chain markers: the `exp_raw.tsv` comparison fails.
  (g) **Generator** (`synth.rs`, ADR-80 as amended, ADR-94 items 1, 2): (1) a conventional LONG value of 32,768 to 65,520
  bytes as one field, and of 65,535 to 150,000 bytes in the chunked form (chunks of 1 to 4,000 bytes chosen by the seed,
  the last followed by `00 00`), and a direct-path LONG chain of up to 150,000 bytes, only in the combination the `long3`
  fixtures show (18c or 11g, table mode, database and client 873); elsewhere LONG stays at most 32,000 bytes; every
  writing site names [R2] / [Y9] v3.7. One deterministic case each with the fixture's exact shapes: 16 x 4000 + 1535 then
  `00 00` (65,535) and 25 x 4000 then `00 00` (100,000). (2) Values from 65,521 to 65,534 bytes are never generated as
  valid input. (3) Negative cases with their expected result from ADR-95: a conventional LONG u16 0xfff1 to 0xfffc, a
  conventional LONG RAW u16 from 0x8000 and `fd ff` (`E-UNSUPPORTED-LAYOUT`); a chunk u16 from 0xfff0 (`E-FRAMING`); a
  chunked total below 65,535 (`W-UNTESTED-LAYOUT` and the value), the last as a new variant
  `Unconfirmed::LongChunkedBelow65535` (evidence: R-048). (4) LONG RAW stays at most 32,000 bytes on both paths. (5)
  TIMESTAMP f from 0 to 9, length 7 / 11 from [T5] v3.7 as a literal table; the coverage test asserts all ten occur. (6)
  `E1AfterFlag1In18c` becomes `E1AfterFlag1In18cOwnerMode` (owner-mode 18c files only); an 18c table-mode file with E1
  after flag 1 holds no variant (asserted). (7) `header_sets_match_the_fixtures` counts 211 files. Expected values come
  from the generator's input only. Mutations: (M11) the generator's chunk terminator dropped: a `t17_synth` test fails
  by name; (M12) a 65,525-byte value admitted as valid input: the coverage assertion of (2) fails.
  (h) **Damage and fuzz**: `t14_damage` ranges over the 211 files of the golden catalog (no literal); `tools/fuzz_seeds.sh`
  run (the seed count in the note: 245 fixture `.dmp` files, 235 + the 10 new, plus the synthetic ones); `tools/fuzz_run.sh` for `walk`, `read`, `values`,
  600 s each, alone, 0 findings; `PROPTEST_CASES=2000` on `t17_synth` and 10,000 on `t14_damage` once, times in the note.
  (i) **Python**: `LongStats.chunked` (docstring: "values in the conventional `fd ff` chunked form"; `chained`: "direct-path
  values in more than one piece"); the survey's LONG line prints `chunked`; the `UnsupportedError` docstring names the two
  R-048 cases as examples; `tests/test_survey.py` (or the existing survey test) shows `chunked 3` for `long3_exp_18c`.
  (j) **README**: "What it reads" and "Limits" carry the sentence of ADR-98 item 5 in place of "a conventional value of
  32,768 bytes or more stops the file" (lines 21-22 and 132 at `8162eb0`); nothing else changes there.
  (k) **Messages**: `t12_messages` takes the new texts; no message holds a value byte (ADR-58).
  (l) **Nothing else**: `git diff --stat 8162eb0 <T-19 commit> -- . ':!docs'` lists only `crates/oradmp-core/src/`,
  `crates/oradmp-core/tests/` (including `data/` by its generators), `tests/` (pytest files and `tests/golden/` by their
  generators), `python/oradmp/`, `README.md`, and the note. Not `corpus/`, `docs/formats/`, `.github/`, `tools/` (unless a
  literal 201 is found there), the version.
  (m) **Step 3b** (the core changed since `460aef5`): the benchmarks and the memory test of `benchmarks.md` section 5
  rerun, alone, no Oracle container; figures against baseline 2 in the note; a regression beyond rule 3 stops the task
  for the Architect.
  (n) **Checks**: the definition of done (fixture checksums, fmt, clippy twice, `cargo nextest run -j 2`, `cargo deny
  check`, `maturin develop --features test-probes`, ruff, ruff format, mypy, `pytest -rs` with 0 skipped on the VM);
  `git status --porcelain` empty after each run; one push run green with counts equal to the VM's.
- Touches: as (l). **Model: Sonnet.** Reviewed in **Reviewer 6c** with T-17b, T-16a, T-16a2 and the version commit
  (ADR-98 item 3).

## T-19b The three conventional LONG / LONG RAW forms; the session 19 and 20 fixtures in the tests; the TSLTZ warning (session 25; ADR-99 to ADR-103) - DONE (2026-10-25, archive `1bb7785`; nextest 488 passed + 4 skipped, pytest 251; equal on the first public CI run of S, ADR-105 item 9; to be reviewed in Reviewer 6c)

- Goal: the reader follows rules v3.9 [R2], [R8], [Y8], [Y9]: every conventional LONG and LONG RAW size the fixtures show
  reads, on both types; 65,522 to 65,524 bytes read with a warning; a field starting `fb ff` / `fc ff` stops by name; the
  18 new EXP files and 10 new truth files are in every corpus test, every value of every row compared with its truth.
- Inputs: ADR-95 (amended), ADR-99 to ADR-103, ADR-80 (amended), ADR-86; rules v3.9; `exp.md` 5.6, 6.5, 6.6 (sessions 19,
  20); `requests.md` R-048, R-050, R-051 (the value arithmetic); `docs/formats/tools/exp_parse.py` (`long_fa`,
  `long_chunks`), `long4_check.py`, `long5_check.py`, `tsltz2_check.py`, `corpus_counts.py`, `rowlen_stats.py`,
  `num_decode.py`, `collist.py`; the truth files `truth_long4.txt`, `truth_long5.txt` (both versions),
  `truth_longraw4.txt` (18c), `truth_longraw5.txt` (both), `truth_tsltz2.txt` (both), `11g/truth_r049.txt` (listed, no
  table); `crates/oradmp-core/src/exp/rows.rs` (the arms named in the T-19 note "For the next tasks", `CHUNKED_MIN_TOTAL`,
  `RawHasher`), `exp/items.rs`, `reader.rs` (`Kind::Tsltz`, `ByteStream::new`), `values/datetime.rs` (`decode_tsltz`),
  `values/long.rs`, `dump.rs`, `error.rs` (`UnverifiedType` doc); the tests the T-19 note lists; `tests/common/synth.rs`,
  `t17_synth.rs`; `tools/fuzz_seeds.sh`; `python/oradmp/_api.py`, `_survey.py`, `_errors.py`; `crates/oradmp-py/src/conv.rs`;
  README.
- Works in the owner's tree: the new fixtures are present there, uncommitted (`git status` shows them); `corpus/` is
  never modified. Fixture checksums: `sha256sum -c` in the three folders before and after.
- Needs from the owner: the two commits and one push (item (q)); the pasted summaries (ADR-92 item 3).
- Every item has a test or check that **fails at `921971f` with the new fixtures present (or under the named mutation) and
  passes on T-19b**; the note's table names it. Mutations each alone, restored after each, `git status` as before after
  each. Expected values come from the truth files, the requests' arithmetic and the Analyst's tools, never from the reader.
- **Done when:**
  (a) **Reader** (`exp/rows.rs`, ADR-99 item 1 table, path flag 0, type 8 **and** type 24 alike): `fe ff` NULL; 0
  `Framing`; 1 to 0xfff1 and 0xfff5 to 0xfff9 one field; 0xfff2 to 0xfff4 one field + `W-UNTESTED-LAYOUT` (ADR-99 item 3
  text, once per column and table section, at the field's u16; `strict`: `E-STRICT`); `fa ff`: u16 n, u16 z, then n bytes,
  `E-FRAMING` at field + 2 for n outside 0xfffa..=0xfffd and at field + 4 for z != 0 (texts of ADR-99 item 3); `fb ff`,
  `fc ff` `E-UNSUPPORTED-LAYOUT` (text of ADR-99 item 3, `length` = u); `fd ff` chunked for both types, chunk u16 from
  0xfff0 `E-FRAMING` (widened text); `CHUNKED_MIN_TOTAL` = 65,534 with the new warning text; the LONG RAW stop from 0x8000
  and the `0xfff1..=0xfffc` arm are gone. Form (b): value offset field + 6, a plain value to the sink, `max_lob_bytes`
  checked against n first (ADR-99 item 4). `W-LONG-LENGTH` above 100,000 for both types with the two texts of ADR-99 item
  6. `RawHasher`: form (b) as `f9 ff ff ff` + u32 n + bytes; its doc comment updated (ADR-99 item 5). `LongStats` gains
  `sizes_65521_to_65534` (every non-NULL value with a total of 65,521 to 65,534 bytes, any path). Direct path and every
  other column unchanged. Unit tests in `exp::rows::tests`, both type codes, one case per table row: u = 0xfff1, 0xfff2,
  0xfff4, 0xfff5, 0xfff9; `fa ff` with n = 0xfffa and 0xfffd (read), n = 0xfff9 and 0xfffe (`Framing` at + 2), z = 1
  (`Framing` at + 4); 0xfffb, 0xfffc (stop); `fd ff` with a total of 65,533 (warning) and 65,534 (none); a truncation
  inside each of form (b)'s three parts (`E-TRUNCATED`).
  (b) **TSLTZ** (ADR-100 item 1): `decode_tsltz` returns the timestamp only; `Kind::Tsltz` raises no event; the
  `UnverifiedType` doc names the ROWID prefix alone. `tsltz2_exp(_direct)_<v>` (4 files): T_TSLTZ2's descriptors are
  type 231 with lengths 7 / 11 / 11 (L0, L3, L9), no `W-DDL-FALLBACK`; Arrow types Timestamp(Microsecond) for L0 and L3,
  Timestamp(Nanosecond) for L9, no zone (arch 5.1); every value of the 4 rows equals the decimal `Typ=231` bytes of
  `truth_tsltz2.txt` decoded by the test's own [Y2] / [Y5] arithmetic and equals the `TO_CHAR` line (truncated to the
  unit); row 4 all NULL; the warnings of the table are exactly three `W-TSLTZ` (one per column) and nothing else.
  `t5f_followup`'s TSLTZ column count follows the listing (4 + 12 = 16), not a literal from the reader.
  (c) **Buffer** (ADR-102 item 3): one constant `MIN_BUFFER_BYTES = 1 << 17` at every `ByteStream::new` of `dump.rs` and
  `reader.rs`; test: `Dump::rows` (and `part_rows`) with `buffer_bytes = 4096` read `long4_exp_18c` row 5 (65,533 bytes,
  form (b)) and `long3_exp_18c` row 4 (65,520 bytes, one field) equal to their truth.
  (d) **The new LONG and LONG RAW files**, both paths, every row, each in a T-19b test file (or `t19_long.rs`):
  - `long4_exp(_direct)_<v>` (4 files), T_LONG4, 7 rows (65,521, 65,525, 65,530, 65,532, 65,533, 65,534, 65,535): length,
    first and last 20 bytes of L and the text of V equal `truth_long4.txt`; the whole L equals `L` + the 6-digit size +
    `y` x (size - 7), built by the test from the truth length (R-040's arithmetic). `LongStats` conventional `values 7,
    chained 0, chunked 2, largest 65535, above_32000 7, sizes_65521_to_65534 6`; direct `values 7, chained 7, chunked 0,
    largest 65535, above_32000 7, sizes_65521_to_65534 6`.
  - `long5_exp(_direct)_<v>` (4 files), T_LONG5, 6 rows (65,526 to 65,531), as above with `truth_long5.txt`; conventional
    `chunked 0, largest 65531, sizes_65521_to_65534 6`; direct `chained 6`.
  - `longraw4_exp(_direct)_18c` (2 files), T_LONGRAW4, 4 rows (32,768, 65,520, 65,535, 100,000): length and first / last
    10 bytes (hex) equal `truth_longraw4.txt`; the whole LR equals n bytes `ab` (R-048's arithmetic); conventional `values
    4, chunked 2, largest 100000, above_32000 4, sizes_65521_to_65534 0`; direct `chained 4`.
  - `longraw5_exp(_direct)_<v>` (4 files), T_LONGRAW5: 18c 7 rows (65,521, 65,525, 65,529, 65,530, 65,531, 65,533,
    65,534), 11g 11 rows (the same + 32,768, 65,520, 65,535, 100,000), as above with `truth_longraw5.txt`; conventional
    18c `chunked 1, largest 65534, sizes_65521_to_65534 7`, 11g `chunked 3, largest 100000, above_32000 11,
    sizes_65521_to_65534 7`; direct `chained` = the row count.
  - In all 14 files: T_FRAME equals its truth (the existing checker); no warning at all with default options **and** with
    `strict=True`; the assembled LONG digest (`long_sha256`) of the conventional file equals the direct file's (same
    version), and for `long4` / `long5` all four files agree.
  (e) **Stops and framing on patched copies** (the patched-copy helper; copies below `target/` or `tmp_path` only):
  `long4_exp_<v>.dmp` with the u16 at 0x22277 (`fa ff`, row 3) set to `fb ff` and to `fc ff`: the scan ends `stopped`
  with `E-UNSUPPORTED-LAYOUT` at 0x22277, table T_LONG4, row 3, column 2, `length` 0xfffb / 0xfffc, the exact text;
  `long_exp_<v>.dmp` with the u16 at 0xa0ea (T_LONGRAW row 2) set to `fb ff`: stopped at 0xa0ea, T_LONG reads its 3 rows.
  `long4_exp_<v>.dmp` with 0x22279 (n) set to `f9 ff` and to `fe ff`: `E-FRAMING` at 0x22279 with `length` 0xfff9 /
  0xfffe; with 0x2227b set to `01 00`: `E-FRAMING` at 0x2227b, `length` 1. The T-19 patched cases that no longer stop
  (long3 0x2228f = `f1 ff`; long_exp 0xa0ea = `00 80`, `f0 ff`, `fd ff`) are removed; long3 0x2228f = `fc ff` stays with
  the new text. Python (`tests/`): the `fb ff` copy gives `Catalog.stop_error` an `oradmp.UnsupportedError` with `.code
  == "E-UNSUPPORTED-LAYOUT"` and `.offset == 0x22277`; `oradmp tables <copy>` exits 1 with the code, the text and the
  offset on stderr (these replace the two R-048 cases of T-19).
  (f) **Memory guard on form (b)**: `long4_exp_<v>` with `max_lob_bytes = 65_531`: row 4 (65,532 bytes) gives
  `E-LOB-TOO-LARGE` at 0x3228b with `length` 65,532, rows 1 to 3 read; with `on_value_error="null"` rows 4 to 7 have L
  NULL and V intact (`v65532` ... `v65535`), T_FRAME reads, `W-VALUE-NULLED` as the existing rule counts it.
  (g) **Counts and golden files**: `TOTALS` / `FOLDER_TOTALS` from the Analyst's tools: **229** files (105 / 97 / 27),
  436 sections, 450 parts, 41,494 rows, 243,905 fields, 20,084 NULL, and the other fields as `corpus_counts.py`,
  `rowlen_stats.py`, `num_decode.py`, `collist.py` print them; truth listing **97**; every literal 211 / 87 / 404 / 418
  in tests, comments and generators replaced by the catalog's own count or the new figure; `t10_truth.rs` `DELEGATED`
  rows for T_LONG4, T_LONG5, T_LONGRAW4, T_LONGRAW5 and T_TSLTZ2 (the table names as the truth files and the catalog give
  them). The generators (`tests/golden/gen_exp_catalog.py`, `gen_exp_raw.py`,
  `crates/oradmp-core/tests/data/gen_walk_expected.py`, `gen_number_parts.py`) take the new files; `gen_exp_raw.py`
  serialises `long_fa` of `exp_parse.py` as ADR-99 item 5; if the walk-expected `long` field mirrors `LongStats`, it
  gains the band count, computed by the generator from `exp_parse.py`'s values. All golden and expected files
  regenerated by their generators, none edited by hand. `t3a`'s "all files walk to the second `EXIT`, none stops" and
  `t11_strict`'s documented list include the 18 files (the `tsltz2` files carry `W-TSLTZ` as `t_tstz` does).
  (h) **Generator** (`synth.rs`, ADR-80 as amended, ADR-99): one field for 32,768 to 65,521 and 65,525 to 65,529 bytes;
  form (b) for 65,530 to 65,533; chunked from 65,534; for **LONG and LONG RAW** alike, only in the combinations the
  fixtures show (`long3` / `long4` / `long5` / `longraw4` / `longraw5` headers: 18c or 11g, table mode, their sets); every
  writing site names [R2] v3.9. Deterministic cases with the fixtures' shapes: `fa ff fa ff 00 00` + 65,530 bytes; `fa
  ff fd ff 00 00` + 65,533; `fd ff` + 16 x 4000 + 1534 + `00 00` (65,534); the same for LONG RAW. 65,522 to 65,524 never as
  valid input: a new label `Unconfirmed::LongOneFieldFff2ToFff4` (evidence route R-052) writes them with the expected
  value **and** the warning; `LongChunkedBelow65535` becomes `LongChunkedBelow65534`. Negative cases with ADR-99's
  results: `fb ff` / `fc ff` (stop), form (b) with n out of range or z != 0 (`Framing`), a chunk u16 from 0xfff0
  (`Framing`). `header_sets_match_the_fixtures` counts 229. Coverage asserts both types reach all three forms.
  (i) **Damage and fuzz seeds**: `t14_damage` over the 229 files (no literal), new truncation sites after `fa ff`, after
  n and inside the bytes of form (b); `tools/fuzz_seeds.sh` run (seed count in the note: 263 fixture `.dmp` files, 245
  + 18, plus the synthetic ones); `PROPTEST_CASES=2000` on `t17_synth` and 10,000 on `t14_damage` once, times in the
  note. The 600-second fuzz runs go to T-19 (m) (the machine alone).
  (j) **Python**: `LongStats.sizes_65521_to_65534` (last field; docstring "values of 65,521 to 65,534 bytes, any form or
  path"; `conv.rs` passes it); the survey's LONG line ends `65,521 to 65,534 bytes N`; `tests/test_survey.py`:
  `long4_exp_18c` shows `chunked 2` and `65,521 to 65,534 bytes 6`, `longraw4_exp_18c` `chunked 2` and `65,521 to 65,534
  bytes 0`; the `UnsupportedError` docstring's example is the `fb ff` / `fc ff` case; `_errors.py` unchanged otherwise.
  (k) **README** (ADR-104 item 4, ADR-100 item 2): "What it reads": the sentence of ADR-104 item 4 replaces the T-19
  sentence, and "a LONG above 100,000 bytes or a LONG RAW above 32,000 bytes with a warning" reads "a LONG or LONG RAW
  above 100,000 bytes with a warning". "Limits": the LONG line reads "A conventional-path LONG or LONG RAW field that
  starts `fb ff` or `fc ff` (no test file has one) stops the file with `E-UNSUPPORTED-LAYOUT` (tables before it stay
  readable); a value of 65,522 to 65,524 bytes is read with `W-UNTESTED-LAYOUT`."; a new line "Empty tables created with
  deferred segment creation are tested on 18c only (11g XE cannot create them); in an 11g Enterprise Edition dump they
  are read by the same rule, untested." Nothing else changes there.
  (l) **Messages**: `t12_messages` takes the patched copies of (e) and asserts the new texts among the messages seen; no
  message holds a value byte (ADR-58); the two retired R-048 texts appear nowhere in `crates/` or `python/`.
  (m) **Mutations** (each fails a named test): (M1) the `fa ff` arm reads the value from field + 4 (the `00 00` not
  consumed; a `long4` truth test); (M2) the n range check removed; (M3) 0xfff2 to 0xfff4 without the
  warning; (M4) 0xfffb read as a length; (M5) the LONG RAW stop from 0x8000 restored; (M6) `CHUNKED_MIN_TOTAL` back to
  65,535 (the `long4` "no warning" test, row 6); (M7) the TSLTZ event restored (the `tsltz2` warning test); (M8) the
  buffer minimum removed in `dump.rs` (the test of (c)); (M9) `gen_exp_raw.py` writes form (b) as a plain field,
  `exp_raw.tsv` regenerated (the `t2_manifest` comparison); (M10) the LONG RAW `W-LONG-LENGTH` threshold back to 32,000
  (a `longraw` "no warning" test); (M11) the band count includes 65,520 (the `longraw4` band assertion); (M12) the
  generator admits 65,523 as valid input (the coverage assertion).
  (n) **Architecture**: no Implementer edit; arch 1.4, 1.5, 2.5, 5.1, 5.4, 5.5, 7.x are already written (session 25).
  (o) **Nothing else**: `git diff --stat 921971f <T-19b commit> -- . ':!docs' ':!corpus'` lists only
  `crates/oradmp-core/src/`, `crates/oradmp-core/tests/` (with `data/` by its generators), `crates/oradmp-py/src/`,
  `tests/` (pytest files and `tests/golden/` by their generators), `python/oradmp/`, `README.md`; and the note. Not
  `.github/`, `tools/`, the version.
  (p) **Checks**: the definition of done (fixture checksums in the three folders, fmt, clippy twice, `cargo nextest run -j
  2`, `cargo deny check`, `maturin develop --features test-probes`, ruff, ruff format, mypy, `pytest -rs` with 0 skipped
  on the VM); counts of tests in the note.
  (q) **Commits (the owner; the Implementer commits nothing)**: after the note, in one push: first a corpus commit with
  the new fixture files and logs in `11g/` and `18c/`, both `SHA256SUMS`, `corpus/scripts/make_pending3.sh` and the four
  `corpus/sql/18_*.sql` files; then the T-19b commit. The push run green with counts equal to the VM's; the owner pastes
  the summaries.
- Touches: as (o). **Model: Sonnet.** Reviewed in **Reviewer 6c** (ADR-104 item 2).

## T-19 (m) Step 3b on T-19b's commit; the fuzz runs (session 25; ADR-104 item 1) - DONE (2026-10-26, archive `0520cd5`; no regression, one watch: `T_BIG` scan -4.8 to -6.8 %, ADR-106 item 2; fuzz 600 s each, 0 findings, after the one-line `values.rs` fix)

- Goal: the benchmark and memory figures and the 600-second fuzz runs for the core of T-19 and T-19b, measured once.
- Needs from the owner: `ora11` and `ora18` stopped, no other build or session; `ORADMP_WORK` set to the folder that holds
  `t_big_exp_18c.dmp`; T-19b committed. One Implementer session (Sonnet), alone on the machine.
- **Done when**: (a) the 1-minute load before and after each run in the note; (b) the micro-benchmarks, the throughput
  test and the memory test of `benchmarks.md` sections 3 and 5 run on T-19b's commit, figures against baseline 2 in the
  note, each with its rule-3 verdict; a regression beyond rule 3 stops the task for the Architect; (c) `tools/fuzz_run.sh`
  600 s for `walk`, `read`, `values`, 0 findings, run counts in the note; (d) no file outside the note changes (`git
  status`); the owner commits the note.

## T-20 The a4 version commit: version, `[project.urls]`, README and CONTRIBUTING for the public tree, the fuzz compile job (session 26; ADR-105 to ADR-107) - DONE (2026-10-29, `e1ed10b`, pushed with `4aa33e6`; nextest 488 passed + 4 skipped, pytest 256; `fuzz-check` 35 s, `cargo check` 26.39 s cold; ADR-108 item 1; to be reviewed in Reviewer 6c)

- Goal: the exact a4 candidate C: version `0.1.0a4`, the source link on PyPI, documents a stranger can follow, and a push
  job that compiles `fuzz/`.
- Inputs: ADR-90, ADR-93, ADR-104 item 4, ADR-105 to ADR-107; `tests/test_version.py`, `tests/check_dist.py`,
  `tests/test_check_dist.py`, `tests/test_project_name.py` (no relative link in the README); `.github/workflows/ci.yml`
  (the `extended` job's toolchain step); `README.md`, `CONTRIBUTING.md`, `HISTORY.md` (read only); PyPA's core metadata
  specification (`Project-URL`) and "Well-known Project URLs in Metadata" (PEP 753); maturin's documentation of
  `[project.urls]` for the pinned 1.15.0.
- Needs from the owner: one commit and one push to `origin`, then the pasted summaries (ADR-92 item 3); afterwards one
  `extended` and one `manylinux2014` dispatch on C, pasted (input of Reviewer 6c). No tag. The new job uses only
  `actions/checkout` at the existing pin, so the Actions allow-list needs no change (ADR-105 item 8); if the Implementer
  needs any other action, it stops and asks.
- **Done when:**
  (a) **Version**: root `Cargo.toml` `version = "0.1.0-alpha.4"`; `Cargo.lock` (`oradmp-core`, `oradmp-py`) and
  `fuzz/Cargo.lock` (`oradmp-core`) entries the same (no other line of either lock changes: `git diff` shows the three
  version lines only); every version in the README `0.1.0a4` (the
  pip pins, the wheel and sdist file names); `git grep -n '0\.1\.0a3\|0\.1\.0-alpha\.3' -- . ':!docs' ':!corpus'
  ':!HISTORY.md'` empty; `tests/test_version.py` green.
  (b) **`[project.urls]`** in `pyproject.toml`: `Source = "https://github.com/granddad-wolf/oradmp"`, `Issues =
  "https://github.com/granddad-wolf/oradmp/issues"`. `check_dist.py` (in `check_metadata`, so for the wheel's `METADATA`
  and the sdist's `PKG-INFO` alike): exactly the `Project-URL:` lines `pyproject.toml` declares (label and URL, in any
  order), else one FAIL line each;
  cases in `tests/test_check_dist.py`: right lines pass; a missing line, a changed URL and an extra line each fail.
  Mutation: the rule compares labels only: the changed-URL case fails.
  (c) **README**: the LONG sentence of ADR-104 item 4 stands verbatim in "What it reads" (it does since T-19b; checked,
  not rewritten); "From source": the clone line names the repository with its absolute address (`pip install .   # in a
  clone of https://github.com/granddad-wolf/oradmp`); Rust 1.99 and "pip fetches maturin" checked against
  `rust-toolchain.toml` / `rust-version` and `pyproject.toml`; "Limits" checked line by line against the current reader
  (the T-19b lines present; nothing removed or added unless a line is false, then the note says which). No word on the
  cut or the archive (that is `HISTORY.md`'s). `tests/test_project_name.py` green (no relative link).
  (d) **CONTRIBUTING**: "Release wheel": "`d081109` gave the same wheel ..." names it as a commit of the private archive
  (see `HISTORY.md`) and adds one sentence: from 0.1.0a4 on a release is built from a commit of this repository with
  `--release`; "CI": a push runs **three** jobs (`fuzz-check` described in one sentence); the definition-of-done list
  says `cargo deny check` (as CI runs it); one sentence "Report a security problem privately with GitHub's 'Report a
  vulnerability' (Security tab), not in an issue." Nothing else; no mention of private files.
  (e) **`HISTORY.md`** unchanged (`git diff --stat` does not list it).
  (f) **`fuzz-check` job** (ADR-106 item 1) on `push` and `workflow_dispatch`: `actions/checkout` at the existing pin with
  `persist-credentials: false`; the nightly of `fuzz/rust-toolchain.toml` installed as in `extended`; `cd fuzz && cargo
  check --locked`; a Summary block (commit, runner image, fuzz toolchain, result, job duration). Workflow `permissions`
  unchanged; no secret; no cache that the other jobs do not already use. Shown first on the VM: `cd fuzz && cargo check
  --locked` passes (the cold and warm times in the note); with `fuzz/fuzz_targets/values.rs` line 51 reverted to the
  pre-`0520cd5` pattern it fails (mutation, restored after). YAML parse and `bash -n` of the new block.
  (g) **Nothing else**: `git diff --stat S C -- . ':!docs'` lists only `Cargo.toml`, `Cargo.lock`, `fuzz/Cargo.lock`,
  `pyproject.toml`, `README.md`, `CONTRIBUTING.md`, `tests/check_dist.py`, `tests/test_check_dist.py`,
  `.github/workflows/ci.yml`; and the note in `docs/design/implementer-notes.md`.
  (h) **Checks** (definition of done): `sha256sum -c` in `11g/`, `11g-cp1251/`, `18c/`; `cargo fmt --check`; `cargo clippy
  -- -D warnings` and `cargo clippy --all-targets -- -D warnings`; `cargo nextest run -j 2` (488 passed, 4 skipped:
  unchanged); `cargo deny check`; `maturin develop --features test-probes`; `ruff check`, `ruff format --check`, `mypy`;
  `pytest -rs` (251 + the new `check_dist` cases, 0 skipped); counts in the note.
  (i) **CI**: the owner's one push of C; the run green: `checks` counts equal (h)'s, `wheel` `release root: canonical`,
  `check_dist: ok` (now with the URL rule), the three venvs' counts as on S plus the new cases, the METADATA of the CI
  wheel shows both `Project-URL` lines (the summary or `check_dist`'s output), `fuzz-check` green with its duration; the
  CI sdist hash recorded (it differs from S's: another commit time).
- Touches: as (g). Not `crates/*/src`, `python/`, `corpus/`, `HISTORY.md`. **Model: Sonnet.** One build at a time.
  Reviewed in **Reviewer 6c**.
- The owner, after the green run: dispatches `extended` and `manylinux2014` on C once and pastes both; starts Reviewer 6c.
  No tag until the release step.
- **Result (session 27, ADR-108 item 1)**: commits `4aa33e6` (documents) and
  `e1ed10bff1587c28861bcf0b1c6e9f950c788cce` (T-20), one push `9950ef2..e1ed10b`. Push run: `checks` nextest 488 passed +
  4 skipped, `cargo deny` four checks ok, pytest 256 passed; `wheel` `release root: canonical`, `check_dist: ok` (with
  the `Project-URL` rule), venvs 253 + 3 / 246 + 4 / 253 + 3 skipped, `SOURCE_DATE_EPOCH` 1793310726, wheel
  `a3fb19c3cf43c830c1628806b44a86f09238dc211c74b2e012c82dc464903807`, sdist
  `7a2ff91c6b96ffe864463b4605a60b718f00e28acdc7fef3f088d6573cffd83f`; `fuzz-check` green, 35 s to the end of its step,
  `Finished ... in 26.39s` (pinned nightly, cold runner; VM 23.1 s cold, 0.05 s warm). Dispatches on `e1ed10b`:
  `extended` 435 s (fuzz 60 s each, `read` 68,805 / `walk` 221,819 / `values` 14,799,481 runs, 0 findings; 463 seeds;
  `t14_damage` 5 passed; `t17_synth` 30 + 1 skipped; fuzz clippy ok; bench report only); `manylinux2014` 119 s (wheel
  `...-cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.whl`, `be6c78e5...`, not shipped; `auditwheel` 6.8.2
  consistent with `manylinux_2_17_x86_64`, up to `GLIBC_2.14`). **These hashes are T-20's evidence, not the candidate's**:
  T-20b's commit becomes C (ADR-108 item 5). The note's `set -o pipefail` deviation: accepted, superseded by T-20b.

## T-20b `pipefail` for every CI step (session 27; ADR-108 items 4, 5) - DONE (2026-10-29, `4b9883c52ebf709ccf0161883505844fdcc45345` = C; push run green, proof run 40974068275 red as planned, `extended` and `manylinux2014` on C green; reviewed: review-06c, no blocking finding; ADR-109)

- Goal: a failure of any piped command in `.github/workflows/ci.yml` fails its step and its job; the commit is the a4
  candidate C.
- Inputs: ADR-108 item 4 (the step table) and item 5; `ci.yml` at `e1ed10b`; `tools/fuzz_run.sh` (its exit paths);
  GitHub's "Workflow syntax for GitHub Actions" (`defaults.run.shell`, the default shell table, `continue-on-error`,
  the `steps.<id>.outcome` context); GNU Bash manual ("Pipelines", `set -o pipefail`).
- Needs from the owner: (1) the commit C on `master`, one push, the pasted run; (2) a throwaway branch `t20b-proof` = C
  plus the Implementer's proof commit, pushed (`git push origin t20b-proof`), one `workflow_dispatch` of CI on that
  branch, the pasted summaries and the per-step results, then the branch deleted locally and on `origin` (`git push
  origin --delete t20b-proof`); (3) one `extended` and one `manylinux2014` dispatch on C (`master`), pasted. No tag. No
  new action (allow-list unchanged), `permissions` unchanged.
- **Done when:**
  (a) **Fix**: one workflow-level block after `concurrency:`, `defaults:` / `run:` / `shell: bash`, with a one-line
  comment (every `run` step runs `bash --noprofile --norc -eo pipefail {0}`), and the same line in the header comment.
  No step has its own `shell:`. `fuzz-check`'s `set -o pipefail` line removed (one mechanism).
  (b) **The constructs `pipefail` changes**: (1) `run_one`'s `probes="$(grep ... | awk ...)"` ends in `|| true` inside
  the substitution, so a log without probe skips still reaches the "expected exactly 3" message instead of a silent
  stop; (2) the property step's `if find . -name '*.proptest-regressions' | grep -q .; then` becomes `if [ -n "$(find .
  -name '*.proptest-regressions' -print -quit)" ]; then` (under `pipefail` a SIGPIPE of `find` after `grep -q` exits
  would make the test false and miss a regression file); (3) every other pipeline of the file read under `pipefail` and
  listed in the note, one word each (`ok` or the change); the summary steps stay as they are (every pipe is inside `$(
  )` or ends in `|| true`).
  (c) **VM proof, before the commit**: YAML parse with PyYAML in a scratch script (as T-16a did; `uv run --no-project
  --with pyyaml`, rule 9) and `bash -n` of every `run` block; the shell fact: `bash --noprofile --norc -eo pipefail -c '{
  true; exit 7; } | tee /dev/null; echo reached'` exits 7 without `reached`, and the same with `bash -e -c` prints
  `reached` and exits 0 (the hole, shown); the two rewritten constructs extracted and run under `bash -eo pipefail`
  against a temporary folder: (1) a log with three probe skips gives 3, a log with none prints the message and exits 1;
  (2) a tree with and one without a `x.proptest-regressions` file take the right branches. Outputs in the note.
  (d) **Nothing else**: `git diff --stat e1ed10b C -- . ':!docs'` lists `.github/workflows/ci.yml` only; plus the note.
  No build or test run is needed on the VM (no Rust, Python or test file changes); `sha256sum -c` in the three fixture
  folders.
  (e) **Push run on C green** with `e1ed10b`'s counts (nextest 488 + 4, `cargo deny` four ok, pytest 256; `release
  root: canonical`, `check_dist: ok`, venvs 253 + 3 / 246 + 4 / 253 + 3; `fuzz-check` green); C's wheel and sdist
  hashes recorded. A red step here is a failure `pipefail` now shows: stop, paste, the Architect rules.
  (f) **Proof run on `t20b-proof`** (never merged): in each affected step of ADR-108 item 4 the piped command `X` becomes
  `{ X; exit 7; }` (the real command runs, then its pipeline fails): `checks` nextest, deny, pytest; `wheel` release
  build, the venvs' `pytest` line; `extended` seeds, `read`, `walk`, `values`, the property step's second pipeline
  (`t17_synth`), fuzz clippy, bench; `manylinux2014` `auditwheel show`. Each such step gets an `id`; in each job all
  but the last affected step get `continue-on-error: true` and the last stays plain, so the job turns red; one step
  `if: always()` before each job's summary prints `T-20b proof:` and `<id>=${{ steps.<id>.outcome }}` for the job's
  ids. Expected in the paste: every outcome `failure`; `checks`, `wheel`, `extended`, `manylinux2014` red;
  `fuzz-check` green; `extended`'s crash-file upload step ran (now reachable; it finds no file). `git diff --stat C
  t20b-proof` lists `ci.yml` only (shown before the push). Restored: the branch is deleted (`git ls-remote origin
  t20b-proof` prints nothing); `master` never holds the proof commit.
  (g) **Dispatches on C**: `extended` and `manylinux2014` green, pasted; figures in line with `e1ed10b`'s (input of
  Reviewer 6c, and the release's `extended` run on the candidate, ADR-106 item 1).
- Touches: `.github/workflows/ci.yml`; the note in `docs/design/implementer-notes.md`. Not `CONTRIBUTING.md` (it names
  no shell), not `tools/`, `crates/`, `python/`, `tests/`. **Model: Sonnet.** Reviewed in **Reviewer 6c**.
- **Result (session 28, ADR-109 item 2)**: C = `4b9883c52ebf709ccf0161883505844fdcc45345`. Push run green with
  `e1ed10b`'s counts; `SOURCE_DATE_EPOCH` 1793315613 (= `%ct` of C, checked); wheel
  `2e4c69a57baafe84e6c5ec0b4b9bef1248c93ea2eabdb57adb17d49d58949dbd`, sdist
  `510f2e39f8ae7eb9660fa4f2313062d2245c3c6d16ebd10740fa0e529c40c0fd` (canonical root). Proof run 40974068275 on
  `t20b-proof` (`75ad263`): `checks` / `wheel` / `extended` / `manylinux2014` red with 3 / 2 / 7 / 1 exit-code-7
  annotations, `fuzz-check` green, the crash-artifact step ran; branch deleted locally and on `origin`. `extended` on C:
  0 findings, 463 seeds; `manylinux2014` on C: wheel `1df8ed44bcae37444de00c72b54866d8bfe0c3fbd7794a2668e9632cbe87a1e9`
  (not shipped), `auditwheel` consistent with `manylinux_2_17`, both `Tag:` lines. Reviewer 6c: no hidden failure that
  turns a red check green; SF-1 (five `echo "...$(...)"` lines) and N-1 go to T-21.

## T-21 The first commit after a4: substitutions in `ci.yml`, stale counts, generator docstrings, CONTRIBUTING pins (session 28; ADR-109 items 3, 4) - DONE (2026-10-31, `20bbc80`; nextest 488 + 4 skipped, pytest 262, CI green; risk 32 closed; ADR-114; to be reviewed in Reviewer 7)

- Goal: review 6c's SF-1, N-1, N-2, N-3 and N-5 fixed in one small commit; nothing shipped changes behaviour.
- Inputs: `docs/review/review-06c.md` (SF-1, N-1, N-2, N-3, N-5); ADR-108 item 4 (as amended), ADR-109; `ci.yml` at the
  a4 tag; GNU Bash manual ("Command Substitution", "The Set Builtin": `-e` and the status of a simple command with
  only assignments); `crates/oradmp-core/tests/common/mod.rs` (`TOTALS`).
- Needs from the owner: one commit and one push to `origin`, the pasted summaries; no dispatch needed unless (a) changes
  a line of `extended` (it does: line 363 and the `find` test), then one `extended` dispatch, pasted. No tag.
- **Done when:**
  (a) **SF-1 and N-1** (rule of ADR-109 item 3): in `.github/workflows/ci.yml` every command substitution whose status
  matters stands alone as an assignment, then its value is used: `newest="$(python -c '...')"` then `echo
  "NEWEST_PY=$newest" >> "$GITHUB_ENV"`; the same for the four `JOB_START` lines (57, 178, 318, 363 at C); the property
  step's test becomes `regs="$(find . -name '*.proptest-regressions' -print -quit)"` then `if [ -n "$regs" ]; then`.
  `grep -nE '(echo|printf)[^#]*\$\(' .github/workflows/ci.yml` outside the summary steps (`if: always()`) prints nothing,
  or each remaining line is listed in the note with why its status does not matter. Shown on the VM: under `bash
  --noprofile --norc -eo pipefail`, `x="$(false)"; echo reached` exits 1 without `reached`; the new `find` lines take the
  right branch with and without a `x.proptest-regressions` file; with a `find` over an unreadable folder (`chmod 000`
  in a temp folder) the step stops (exit non-zero). YAML parse and `bash -n` of every `run` block (as T-20b (c)).
  (b) **N-2**: `t11_options.rs` `the_counting_core_agrees_with_the_totals_of_the_201_files` and `t11_strict.rs`
  `strict_over_the_201_files_fails_exactly_on_the_documented_list` renamed without a count (e.g.
  `..._totals_of_all_fixture_files`, `strict_over_all_fixture_files_...`); the comments `t11_options.rs:11`, `:401`,
  `:492` and `common/synth.rs:145` name no file count, or cite `TOTALS`; no assertion changes. `git grep -nE
  '\b(201|211) files\b' -- crates tests` prints nothing. No other file names these tests (`ci.yml`'s skip list names
  only the four ignored tests).
  (c) **N-3**: the docstrings of `crates/oradmp-core/tests/data/gen_walk_expected.py` and `gen_number_parts.py` say
  `cd docs/formats/tools && python3 -I ../../../crates/oradmp-core/tests/data/<name>.py`; both run that way and the
  regenerated files are byte-identical (`git diff` empty). `git grep -n 'uv run' -- crates tests tools` prints only
  lines that say `uv run --no-project` (rule 9), each listed in the note.
  (d) **N-5**: `CONTRIBUTING.md` "Development setup": the install line pins `maturin==1.15.0 pytest==9.1.1
  ruff==0.17.1 mypy==2.4.0` (pandas and pyarrow stay unpinned, as CI's newest venv); one line `cargo install --locked
  cargo-nextest@0.9.146 cargo-deny@0.20.2` before the checks list. A new pytest `tests/test_contributing_pins.py` reads
  `ci.yml` (the `pip install` line of the `checks` job and the `tool:` line of `taiki-e/install-action`) and
  `CONTRIBUTING.md` and asserts the five versions are equal; mutation: a changed ruff version in either file fails it.
  (e) **Nothing else**: `git diff --stat <a4 tag> HEAD -- . ':!docs'` lists only `ci.yml`, the two `t11_*.rs`,
  `common/synth.rs`, the two generators, `CONTRIBUTING.md` and the new test; no file of `crates/*/src/`, `python/`,
  `tools/`, `fuzz/` (so step 3b is not due).
  (f) **Checks** (definition of done): fixture checksums; fmt; clippy (two forms); `cargo nextest run -j 2` 488 passed +
  4 skipped (same count, two names changed); `cargo deny check`; `maturin develop --features test-probes`; ruff, ruff
  format, mypy; pytest 256 + the new test's cases, 0 skipped. Counts in the note.
  (g) **CI**: the push run green with (f)'s counts; each venv's passed count grows by the new test's cases, skips
  unchanged; the `extended` dispatch green, 0 findings.
- Touches: as (e); the note. **Model: Sonnet.** Reviewed with T-16b and T-16c (Reviewer 7, rule 7).
- **Session 29 (ADR-112; this block wins where the text above differs)**. Starts after the owner's session 29 docs push
  and the clean-up rename (`/var/tmp/oradmp-release-0.1.0a4` must exist for (j)).
  - **Needs from the owner**: one commit and one push to `origin`; **one `extended` and one `manylinux2014` dispatch**
    on the pushed commit; the three summaries pasted (with the `wheel` job's member list). No tag, no release.
  - (a) reads: "the same for the **five** `JOB_START` lines (57 `checks`, 178 `wheel`, 318 `fuzz-check`, 363
    `extended`, 468 `manylinux2014` at C)", plus line 200's `NEWEST_PY` and line 402's `find`.
  - (d) adds: the last paragraph of `CONTRIBUTING.md` "Release wheel" names "the newest `Release 0.1.0a<n>
    checklist` in `docs/design/implementation-plan.md`" instead of "First release".
  - (e) adds: `git diff --stat v0.1.0a4 HEAD -- README.md pyproject.toml Cargo.toml Cargo.lock fuzz/Cargo.lock deny.toml
    python crates/oradmp-core/src crates/oradmp-py/src` prints nothing.
  - (g) reads: the push run green with (f)'s counts; the `extended` dispatch green, 0 findings; the `manylinux2014`
    dispatch green with both `Tag:` lines (its wheel hash is recorded, not compared).
  - (h) **`cargo deny check`**: four ok; the only warnings are the unused allowances `ISC` and `Zlib` and the duplicate
    `syn` (expected, ADR-112 items 2, 3; `deny.toml` unchanged). Any other warning: a line in the note.
  - (i) **No member of the wheel changes** (ADR-112 item 4): the note holds two listings (sha256, size, member) made by
    the `wheel` job summary's own Python lines: one from T-21's push-run summary, one from a4's released files at
    `/var/tmp/oradmp-release-0.1.0a4/dist/` (read only; `sha256sum -c SHA256SUMS` first), and the `diff` of the two.
    Pass: every wheel member equal in all three fields (`oradmp/_core.abi3.so` included; its a4 hash is expected to be
    `d0efa2dff42b57f5203429ace3b7789d4d1cfbbb7fd09ce986b5badf51b4789c`), except the SBOM member
    `oradmp-0.1.0a4.dist-info/sboms/oradmp-py.cyclonedx.json` and `RECORD`, which may differ in hash at **equal size**;
    the note says whether they did (ADR-56's open point on the SBOM timestamp). Every sdist member equal except members
    whose path is in (e)'s list; `PKG-INFO` equal. The whole-file hashes differ (another epoch) and are recorded only.
    Any other difference: stop, the Architect rules.
  - (j) The reference listing is made before any other build of the session touches `/var/tmp/oradmp-release*`; the
    a4 folder is never written to.
  - (k) **The a4 sdist from source, once** (ADR-112 item 5): in `/var/tmp/oradmp-t21-sdist/` (outside the repository,
    rule 9), `uv venv --python 3.11`, `TMPDIR=/var/tmp/oradmp-tmp CARGO_BUILD_JOBS=2 uv pip install --python <venv>/bin/python
    /var/tmp/oradmp-release-0.1.0a4/dist/oradmp-0.1.0a4.tar.gz`; `<venv>/bin/oradmp --version` prints `oradmp 0.1.0a4`;
    `<venv>/bin/python -m oradmp survey --no-names <copy of corpus/fixtures/18c/t_text_exp_18c.dmp>` exits 0. Alone on
    the machine (a release build). The folder is deleted afterwards. A failure: stop, the Architect rules (risk 32).
- **Session 30 record** (ADR-114; S = owner's facts or the Implementer's report as pasted, E = checked in session 30):
  - Commit `20bbc80b842cc5b280b920445aaf395742b10a50`, 17 files (E): `ci.yml`, `CONTRIBUTING.md`, the new
    `tests/test_contributing_pins.py`, `implementer-notes.md` (the T-19 (m) figure only), `common/synth.rs`, the two
    generators and ten `t*.rs` files under `crates/oradmp-core/tests/` (comments and the two renamed tests only).
  - Push run and a manual (dispatch) run on it green (S). `checks`: nextest 488 passed + 4 skipped, `cargo deny` four
    ok, pytest 262. `fuzz-check`: `Finished dev profile` in 17.28 s, 24 s to the end of the step. `wheel`: `release root:
    canonical`, `check_dist: ok`, epoch 1793458070, wheel `df5522bc96c30a573df3d63035dfccbcc096e166b0c3c8bea91aa0121abeaa06`,
    sdist `81cd7fe1876bf23a4a6ca8ba817f1f084d5bf4bf763bc9529a5093702a15a85f` (not release hashes); installed-wheel pytest
    259 + 3, 252 + 4, 259 + 3 (a4's 253, 246, 253 + the six new cases). `extended`: 0 findings on 463 seeds (`read`
    54,176 runs, `walk` 153,433, `values` 9,196,793), `t14_damage` 5 passed, `t17_synth` 30 passed + 1 skipped, fuzz
    clippy ok, bench report only (slower runner), 515 s. `manylinux2014`: wheel `084d9b8ff7c6717d3b36bf176bb239250cf05cfcc2cbdf389f0ae6a202d0e1de`
    (not shipped), `auditwheel` consistent with `manylinux_2_17`, symbols up to `GLIBC_2.14`.
  - (a): five `JOB_START` lines, `newest=` and `regs=` as assignments (E). (d): pins and the CONTRIBUTING pointer (E).
  - (h): `cargo deny` four ok (S); the expected warnings are not restated in the facts.
  - (i) **passed** (ADR-114 item 2): reference = the `wheel` listing of the run on C (= a4's released bytes, a4 record
    step 6); all wheel and sdist members (69: 16 in the wheel, 53 in the sdist; session 31, ADR-116 item 4) equal in
    SHA-256 except the SBOM and `RECORD`; `_core.abi3.so` `d0efa2df...`.
    The two members' sizes: not stated (Reviewer 7).
  - (k) **passed**, risk 32 closed: the a4 sdist `510f2e39...` from PyPI built from source into a fresh 3.11 venv in 2 min
    11 s; `oradmp 0.1.0a4`; `survey --no-names` on a copy of `t_text_exp_18c.dmp` exit 0 (S).
  - Deviations accepted (ADR-114 item 4): six pin cases, not five; eight test files beyond (e)'s list, comments only;
    no T-21 note committed (this record stands in).

## T-18 The command-line tool `oradmp` (session 18; ADR-72) - DONE (2026-10-24, `2ba62eb`; ran before T-16, ADR-73 swap; 412 Rust tests, 196 pytest; findings ruled by ADR-75; (l)'s CI part moved to T-16; reviewed: review-06a, SF-1 and SF-2 go to T-18c)
- Goal: `oradmp` as a thin console tool on the public Python API: list tables, show columns, print the first rows,
  export a table or all tables to CSV or Parquet; `survey` unchanged.
- Inputs: ADR-72 (the whole specification), arch 3.4 and 3.5, `python/oradmp/` (`_api.py`, `_errors.py`,
  `_survey.py`, `__main__.py`), `tests/check_dist.py`, the README; the pyarrow documentation of `pyarrow.csv` and
  `pyarrow.parquet` for **15.0.2** (the oldest supported) and the newest.
- **Scope**: `python/oradmp/_cli.py` (new; argparse), `__main__.py` calls `_cli.main`; `[project.scripts] oradmp =
  "oradmp._cli:main"`. Only the public API of `oradmp` is called (no `_core`, nothing in `crates/`); no new dependency;
  pandas never imported. Not in phase 1: ADR-72 item 9.
- **Done when:**
  (a) The six commands and the options of ADR-72 items 3 and 4 exist; `oradmp --help` and `<command> --help` list
  them; `python -m oradmp survey` output and exit codes are byte-identical to before on every fixture
  (`tests/test_survey.py` unchanged and green).
  (b) `tests/test_cli.py`, in-process `main(argv)` with captured stdout / stderr: `tables` and `columns` on every
  fixture of the three folders equal the API's `Dump.tables` / `TableInfo.columns` rendered by the rule of ADR-72
  item 5 (expected values from the API, plus the row counts of two truth tables from their truth files); `head -n 3`
  of `T_TEXT` and `T_NUM` (18c) parsed with Python's `csv` module gives, as values (text as is, NUMBER through
  `decimal.Decimal`), the truth files' first three rows (with the helpers of `tests/test_truth.py`).
  (c) Export: for every table of the 18c and 11g folders that the API reads without an error with the tool's
  defaults, `export --format parquet` read back with `pyarrow.parquet.read_table` equals
  `dump.read_table(t).read_all()` opened with the same options (`Table.equals`, schema metadata included), and
  `--format csv` equals `pyarrow.csv.write_csv` of the same table with binary columns as upper-case hex; `export
  --all` writes the manifest and one file per table; the file-name rule of ADR-72 item 7 has a unit test (a
  duplicate name ignoring case, a name with `/`, both give `table_<ordinal>`).
  (d) The CSV writer's type coverage: every Arrow type of arch 5.1 that the fixtures produce goes through
  `pyarrow.csv.CSVWriter` on pyarrow 15.0.2 and the newest; a refused type stops before any row with `E-CONFIG`
  naming column and type; the note lists every refused type per pyarrow version (none chosen by the task).
  (e) Exit codes, each from a real case: **1** a truncated copy in `tmp_path` (`E-TRUNCATED`) and the patched `T_NUM`
  copy of T-13i (b) with `--on-number-overflow error` (`E-NUMBER-OVERFLOW`); **2** an unknown table
  (`E-NO-SUCH-TABLE`), a wrong option value (`E-CONFIG`), an existing target without `--overwrite`, an argparse
  error; **3** a missing dump file and an output folder that cannot be written (skipped as root, with the reason);
  **130** a `KeyboardInterrupt` raised in the writer (monkeypatch). A unit test walks `_core.ERROR_CODES` through
  `ERROR_CLASSES` and asserts each code has exit code 1, 2, 3 or 4 (mutation: one class removed from the tool's map
  fails it). The stderr line starts `oradmp: error: [<code> offset=`; stdout holds no error text.
  (f) No partial file: after each failing export of (e) the target folder holds neither the target nor
  `*.oradmp-partial`.
  (g) Warnings: the patched `T_NUM` copy with the default options gives exactly one stderr line `oradmp: warning: `
  + the `W-NUMBER-ROUNDED` text of `BatchReader.warnings`, and none with `-q`; the `warnings` module prints nothing
  (pytest `recwarn` empty).
  (h) A subprocess test of the console script: `oradmp head <fixture> T_TEXT | head -1`-style closed pipe (stdout
  closed after one line) exits 0 with an empty stderr.
  (i) A test that no file of `python/oradmp/` imports `pandas` (static scan); the checklist step 7 venv without pandas
  is the run-time proof (release).
  (j) `tests/check_dist.py`: the `entry_points.txt` rule and the new required sdist file of ADR-72 item 11, each with
  its mutation case; a `tools/release_build.sh` dry build into a scratch root prints `check_dist: ok` and lists
  `oradmp-<v>.dist-info/entry_points.txt`.
  (k) README "Command line" section (commands, the `--interval-as string` default, exit codes, "data on stdout,
  errors and warnings on stderr"); arch 3.5 is the reference.
  (l) Green on the VM and on every CI platform of T-16 (paths, `os.replace` on Windows); ruff, mypy strict. A T-18
  note with the mutation table and the refused-type list.
- Touches: `python/oradmp/_cli.py` (new), `python/oradmp/__main__.py`, `pyproject.toml` (`[project.scripts]`),
  `tests/test_cli.py` (new), `tests/check_dist.py`, `tests/test_check_dist.py`, `README.md`. Not `crates/`, `corpus/`,
  `tests/golden/`.
- Needs: T-16 done (or the owner's swap, ADR-73). **Model: Sonnet.** Reviewed in Reviewer session 6.

## T-18b Before Reviewer 6a: memory-test report, micro-benchmarks, two CLI cases, README, the a3 version commit (session 19; ADR-75) - DONE (2026-10-24, version commit `d4d20ef`; 412 Rust tests + 3 skipped, 198 pytest; reviewed: review-06a; `d4d20ef` is not released, ADR-77)
- Goal: the leftovers of T-15b and T-18 closed, so that Reviewer 6a reads the final a3 candidate.
- Inputs: ADR-75, ADR-76; the T-15b and T-18 notes; `benchmarks.md` sections 2, 3, 5; ADR-72 as amended.
- **Prompt data the owner gives**: the work folder for `ORADMP_WORK` (holding `18c/t_rows_exp_18c.dmp`,
  `18c/t_rows_exp_direct_18c.dmp`, `18c/t_big_exp_18c.dmp`; never written into a committed file); that the VM runs
  nothing else during (a) and (b).
- **Done when:**
  (a) `crates/oradmp-core/tests/t15_memory.rs`: the test measures **every** file present (both `T_ROWS` files,
  `T_BIG`), then asserts; on failure the message lists each file with both peaks and its ratio. Limit 1.10 and the
  stop points unchanged. The doc comment's "arithmetic until T-15b measures it" becomes the measured figures (T-15b:
  1 byte per row 1.78 to 1.96 on `T_BIG`). Check, alone, release build: the correct reader passes once; the mutation
  b = 8 in `row_end` (as T-15b) with all three files present fails **and its message names both `T_ROWS` files and
  `T_BIG`** (note quotes it); reverted.
  (b) `CARGO_BUILD_JOBS=2 cargo bench -p oradmp-core --bench decode` once, alone; the note gives the seven figures of
  `benchmarks.md` 4.1 and says, per figure, whether it is within 20 % of the baseline (rule 3). Throughput is not run
  again (T-15b's run is on the same core sources). A figure more than 20 % below: run again alone; if confirmed, stop
  and report (step 3b).
  (c) A single-table command (`columns`, `head`, `export TABLE`) whose TABLE is not found in a scan that **stopped**
  (`Catalog.end == "stopped"`) prints the stop error line (as `tables` does) and exits with that error's code; a name
  not found in a complete scan stays `E-NO-SUCH-TABLE`, exit 2. Test on the fixture of the existing stopped-scan test:
  a name that is not in the catalog gives the stop error's code and line; mutation: the old behaviour fails it.
  (d) Test: 18c `T_NUM` with `export --on-value-error null -o <tmp>/t.csv` exits 0; stderr is exactly the lines
  `oradmp: warning: ` + each `DumpWarning.text` of `BatchReader.warnings` of the same read through the API (same
  options), in order; with `-q` stderr is empty. Mutation "reader warnings not printed on success" fails it.
  (e) README: the `head` sentence becomes: "`head` prints no warning of the table's read: the reader gives the
  counted warnings (for example `W-NUMBER-ROUNDED`) only at the end of a read, and `head` stops after N rows, so a
  value it shows may be rounded or NULL without a warning; `export` reports them." The stopped-scan rule of (c) in
  the exit-code paragraph; `oradmp --version` named once; the Install section gains one line: "The package installs
  the command `oradmp` (see "Command line")".
  (f) Arch 3.5 is the reference; nothing else of the tool changes.
  (g) **The a3 version commit**, last and separate: `0.1.0-alpha.3` in root `Cargo.toml`, both `Cargo.lock` entries,
  the `oradmp-core` entry of `fuzz/Cargo.lock`; the README's version lines `0.1.0a3`; `tests/test_version.py` green.
  (h) Checklist step 3 on the version commit, in the dev build (`--features test-probes`): fixture checksums, `cargo
  fmt --check`, `cargo clippy --all-targets -- -D warnings` (also `--all-features`), `cargo nextest run -j 2`, `cargo
  deny check licenses`, `ruff check`, `ruff format --check`, `mypy`, `pytest -rs` with **0 skipped**; counts in the
  note (412 + 3 skipped expected for Rust unless (a) adds a test).
  (i) A T-18b note: the (a) message, the (b) table, the mutations of (c), (d), the counts.
- Touches: `crates/oradmp-core/tests/t15_memory.rs`, `python/oradmp/_cli.py`, `tests/test_cli.py`, `README.md`, the
  version files of (g). Not `crates/oradmp-core/src/`, `corpus/`, `tests/golden/`, `docs/formats/`, the root
  `.gitignore`, `uv.lock`.
- Needs: the `ORADMP_WORK` path for (a). **Model: Sonnet.** Alone on the machine for (a) and (b). Reviewed in
  Reviewer 6a.

## T-18c Reviewer 6a fixes before the a3 upload: tests against the installed wheel, CLI mutations, the I/O error text, README (session 20; ADR-77) - DONE (2026-10-24, `ff3d6b2`; 412 Rust tests + 3 skipped, 227 pytest; reviewed: review-06a-closure, no blocking finding; released as 0.1.0a3; SF-1 and N-5 of that review go to T-18d)
- Goal: review 06a SF-1, SF-2, N-1, N-2 and N-3 fixed, so that a3's checklist step 7 passes as written and a user of
  a3 sees the reason and the path of an I/O error; its last commit is the new a3 candidate.
- Inputs: review 06a (SF-1, SF-2, notes N-1 to N-3), ADR-72 as amended (items 3, 5, 8), ADR-75, ADR-77, arch 3.5;
  `python/oradmp/_cli.py`, `tests/test_cli.py`, `tests/test_version.py`, `README.md`.
- **Done when:**
  (a) **SF-1.** `tests/test_cli.py` takes the package from the import: `PKG = Path(oradmp.__file__).resolve().parent`.
  Every subprocess sets `PYTHONPATH=str(PKG.parent)` (no `ROOT / "python"` left in the file; the note quotes `grep -n
  'ROOT / "python"' tests/test_cli.py`, empty). The pandas scan reads `PKG.glob("*.py")` and asserts that
  `PKG / "_cli.py"` and `PKG / "__init__.py"` are among the files (no empty pass). New test
  `test_the_subprocess_imports_the_same_package`: the child `python -c "import oradmp; print(oradmp.__file__)"` under
  the same environment prints a path whose resolved parent is `PKG`. Shown three ways, results in the note: (1) the dev
  tree (`maturin develop --features test-probes`): green; (2) a wheel built by `maturin build --release` from the
  working tree, installed in a fresh venv, `pytest -rs tests/test_cli.py` from a **scratch git clone** of `d4d20ef` with
  the T-18c files copied in: 0 failed; (3) the same wheel and venv, from a **copied-folder layout** (`tests/`,
  `corpus/fixtures/`, the root files; no `python/`, no `.git`): 0 failed. Mutations: one subprocess back to `ROOT /
  "python"` fails in (2) (the test named in the note, and the new test); the pandas scan back to `ROOT / "python" /
  "oradmp"` fails in (3).
  (b) **SF-2.** One test per survivor; each mutation of review 06a's SF-2 table, applied alone, fails the named test:
  1. `_format_for` unit test: `x.csv` -> csv, `x.parquet` -> parquet, `x.PARQUET` -> parquet, `x.Csv` -> csv, `x.txt`
     and no suffix -> `E-CONFIG`, `-` -> csv, `--all` -> csv, an explicit `--format` wins over the suffix; and one
     end-to-end `export T_TEXT -o <tmp>/t.parquet` without `--format` whose file `pyarrow.parquet.read_table` opens.
  2. `head FILE T_TEXT -n -1`: exit 2, stderr one line starting `oradmp: error: [E-CONFIG]`, stdout empty.
  3. `export FILE --all -o -` (run with `tmp_path` as the working folder): exit 2, `E-CONFIG`, nothing named `-` created.
  4. `oradmp --version`: stdout exactly `oradmp {oradmp.__version__}` and a newline, exit 0.
  5. `--trim-char`: `head` of 18c `T_TEXT` with and without it equals the API read with `trim_char=True` / `False`
     (CSV parsed as in T-18 (b)), and the test first asserts that the two API reads differ (so the case can see the
     option).
  6. `--number-type T_NUM.<column>=string` (a column of 18c `T_NUM` whose text differs from the default decimal
     rendering for at least one row, asserted first): equals the API read with the same `number_types`.
  7. `_escape` and the `tables` / `columns` row renderer: `"a\tb"` -> `a\tb` written as the two characters `\` `t`,
     `"a\nb"` -> `a\nb` likewise, `"a\\b"` -> `a\\b`, and a name with all three in one row.
  8. Pass-through, every read option of ADR-72 item 3 (`--strict`, `--on-value-error`, `--on-number-overflow`,
     `--number-default`, `--number-type` (repeated), `--interval-as`, `--timestamp-unit`, `--trim-char`,
     `--max-lob-bytes`, `--unconfirmed-lob`, `--db-charset`, `--client-charset`, `--columns`): a parametrised test
     that intercepts the tool's call into the public API (monkeypatch of the function `_cli.py` calls) and asserts
     the API keyword and value (e.g. `--number-type T.C=int64` -> `number_types={"T.C": "int64"}`). Mutation: each of
     `--number-default`, `--timestamp-unit`, `--max-lob-bytes`, `--unconfirmed-lob`, `--db-charset`,
     `--client-charset` not passed (each alone) fails its case.
  (c) **N-3, the I/O error text** (ADR-77 item 4, ADR-72 item 8 as amended). For `tables`, `columns`, `head`,
  `export TABLE` and `export --all` on a dump path that does not exist: exit **3**, stdout empty, stderr exactly one line
  `oradmp: error: [E-IO] {os.strerror(errno.ENOENT)}: {path}` (`path` as passed). On a copy with mode 000 (skipped as
  root and on Windows, with the reason): the same with `errno.EACCES`. If the API's text carries an offset
  (`[E-IO offset=0x...]`), the prefix is kept as it is (unit test of the formatting function with a constructed
  `OSError`, including one without `errno`, which keeps `I/O error`). Output-side `OSError`s unchanged (the existing
  exit-3 tests stay green). `survey` unchanged (`tests/test_survey.py` green; `survey missing.dmp` still `ERROR: ...` on
  stdout, exit 1). Mutations: the path left out; the OS text replaced by `I/O error`; each fails the test.
  (d) **N-2, README "Command line"**: one sentence: "`export` takes the format from `--format`, else from the suffix of
  `-o` (`.csv` or `.parquet`, in any case), else it stops with `E-CONFIG` (exit 2); `--all` writes CSV unless
  `--format parquet`; `-o -` writes CSV only."; `--interval-as` added to the option list; the exit-code bullet's
  **3** names the form of the line (`oradmp: error: [E-IO] No such file or directory: FILE`). The long exit-code bullet
  may be split in two (T-18b note); no other README change.
  (e) **N-1**: `tests/test_version.py` also asserts that every match of `\b\d+\.\d+\.\d+(?:a|b|rc)\d+\b` in `README.md`
  equals the Python form of the Cargo version; mutation: "Alpha (version 0.1.0a2)" added to the README fails it.
  (f) **Nothing else changes**: `git diff --stat d4d20ef <T-18c commit> -- . ':!docs'` lists only
  `python/oradmp/_cli.py`, `tests/test_cli.py`, `tests/test_version.py`, `README.md`. Not `crates/`, `pyproject.toml`,
  the Cargo files, `tests/check_dist.py`, `corpus/`, `tests/golden/`.
  (g) **Version**: stays `0.1.0-alpha.3` / `0.1.0a3`; **no version commit** (the version files are already a3 since
  `d4d20ef`). The owner commits T-18c (one commit, or code and README in two); its last commit is the **new a3
  candidate** and the last commit outside `docs/` when the closure review starts.
  (h) Checklist step 3 on the final tree, dev build with probes: fixture checksums, `cargo fmt --check`, `cargo clippy
  --all-targets -- -D warnings` (also `--all-features`), `cargo nextest run -j 2` (412 passed, 3 skipped: unchanged),
  `cargo deny check licenses`, `ruff check`, `ruff format --check`, `mypy`, `pytest -rs` with 0 skipped (198 + the new
  tests; the count in the note).
  (i) A T-18c note: the mutation table (each alone, failing test, all others pass, `git status` clean after each), the
  three runs of (a), the exact stderr lines of (c), the counts.
- Touches: `python/oradmp/_cli.py`, `tests/test_cli.py`, `tests/test_version.py`, `README.md`, the note. **Model:
  Sonnet.** Needs nothing from the owner but the commit. Reviewed in **Reviewer 6a-closure** (order table, session 20).

## T-18d Review 6a-closure fixes and file names that are not UTF-8 (session 21; ADR-82) - DONE (2026-10-24, `ba8a89c`; 412 Rust tests + 3 skipped, 233 pytest; reviewed: review-06b, no blocking finding; SF-4 of 6b goes to T-17b)
- Goal: the API error of the no-errno form gets the right line (SF-1); a dump whose name is not valid UTF-8 opens and
  reads through the API, the tool and `survey`, and every message shows such a name without a traceback (N-5); the
  `-n -1` guard has a named test (N-2); README line 173 (N-7).
- Inputs: review 06a-closure (SF-1, N-2, N-5, N-7), ADR-77 item 4 as amended, ADR-82; `python/oradmp/_cli.py`
  (`_path_text`, `_api_io_text`, `_error_text`), `_errors.py` (`make_error`), `_survey.py` (`write_survey`), `_api.py`
  (`open`), `crates/oradmp-py/src/lib.rs` (`open`); `tests/test_cli.py`, `tests/test_survey.py`; pyo3 0.30 documentation
  (conversion of `std::path::PathBuf`); Python documentation of `os.fsencode`, `os.fsdecode`, `backslashreplace`.
- The name used below: **N** = the bytes `d2 e5 f1 f2 2e 64 6d 70` (`Тест.dmp` in CP1251; not valid UTF-8: `d2` needs a
  continuation byte 0x80 to 0xbf, `e5` is not one), a copy of `corpus/fixtures/18c/t_text_exp_18c.dmp` under
  `tmp_path` (made with `os.fsencode(tmp_path) + b"/" + N`). The cases of (b) are skipped where the file system
  refuses such a name (`sys.platform != "linux"`, reason "POSIX byte file names"), and listed under T-16 (h).
- **Done when:**
  (a) **SF-1.** `_error_text` recognises an API error by `getattr(e, "code", None) == "E-IO"`; its text is `e.strerror`
  when that is a string, else `e.args[0]` when `e.args` is one string. `test_the_io_error_text_of_the_api_error` builds
  the API errors with `oradmp._errors.make_error("E-IO", <text>, ...)` (other fields `None`) and asserts, with file
  `a.dmp`: errno `None`, text `[E-IO] I/O error` -> `[E-IO] I/O error: a.dmp`; errno `None`, text `[E-IO offset=0x10]
  I/O error` -> `[E-IO offset=0x10] I/O error: a.dmp`; errno `ENOENT` -> `[E-IO] {os.strerror(errno.ENOENT)}: a.dmp`.
  An output-side `OSError(errno.EACCES, "Permission denied", "x.csv")` (no `code`) keeps `[E-IO] Permission denied:
  x.csv`. One more case uses the real exception of `oradmp.open("a\x00b.dmp")` with file `a.dmp`; the note says what
  that call raises after (b) (if it is no longer an `OSError` without errno, the case asserts what the code and ADR-77
  item 4 give, and the note names it). Mutation: the branch keyed on `strerror.startswith("[E-IO")` again fails the
  no-errno cases.
  (b) **N-5.** `crates/oradmp-py/src/lib.rs` `open` takes `path: PathBuf` (pyo3's conversion); nothing else in the
  binding changes; `_api.open` and its type hint unchanged. New tests (a new `tests/test_paths.py` or in
  `tests/test_cli.py`, the Implementer's choice):
  1. API: `oradmp.open(os.fsdecode(<path of N>))` and `oradmp.open(pathlib.Path(os.fsdecode(<path of N>)))`: `tables`
     names `T_TEXT`, and `read_table("T_TEXT").read_all()` equals the same read of the fixture itself (`Table.equals`,
     schema metadata included).
  2. API: N in `tmp_path` without the file: `OSError` with `errno == ENOENT` and `code == "E-IO"`; no
     `UnicodeEncodeError`.
  3. Tool, as a subprocess with the path given as **bytes** in argv (`[sys.executable, "-m", "oradmp", ..., <bytes
     path>]`, the environment of `child_env()`): `tables`, `columns ... T_TEXT`, `head ... T_TEXT -n 3`, `export ...
     T_TEXT -o <tmp>/t.csv` each exit 0 with an empty stderr; on the missing name `tables` exits 3, stdout empty,
     stderr exactly `oradmp: error: [E-IO] {os.strerror(errno.ENOENT)}: <dir>/\xd2\xe5\xf1\xf2.dmp` and a newline, where
     each `\xHH` is four ASCII characters (ADR-82 item 3) and `<dir>` is `tmp_path` as text.
  4. `survey`: `python -m oradmp survey <bytes path>` exits 0 and prints the line `file: \xd2\xe5\xf1\xf2.dmp  size: ...`;
     with `--no-names` the line says `(hidden)` as before; `tests/test_survey.py` otherwise unchanged and green.
  5. Unit: `_path_text(os.fsdecode(b"a\xffb\t.dmp")) == "a\\xffb\\t.dmp"`; a valid UTF-8 name (`"Тест.dmp"`) is
     unchanged.
  Mutations, each alone: the binding back to `String` (rebuild) fails 1 to 4 (named); `_path_text` without the `\xHH`
  step fails 3's missing-name case and 5; `survey` without the rule fails 4.
  (c) **N-2.** A subprocess test: `python -m oradmp head <18c t_text> T_TEXT -n -1` exits 2, stdout empty, stderr one
  line starting `oradmp: error: [E-CONFIG]`. Mutation: the `-n < 0` guard off fails this test by name (exit 134); the
  note says whether the in-process case still aborts pytest under the mutation (if so, it is removed: the subprocess
  case replaces it).
  (d) **README**: line 173 wrapped like its paragraph (at most 120 characters); one sentence in "Command line": "A
  file name that is not valid UTF-8 is accepted; messages write each such byte as `\xHH`." No other README change.
  (e) **Nothing else changes**: `git diff --stat ff3d6b2 <T-18d commit> -- . ':!docs'` lists only
  `crates/oradmp-py/src/lib.rs`, `python/oradmp/_cli.py`, `python/oradmp/_survey.py`, `tests/test_cli.py`, `tests/test_survey.py`,
  the new test file if any, `README.md`. Not `crates/oradmp-core/`, `pyproject.toml`, the Cargo files, `tests/check_dist.py`,
  `corpus/`, `tests/golden/`. **No version change** (stays `0.1.0-alpha.3` / `0.1.0a3`; the a4 version commit comes
  after T-16, ADR-84).
  (f) Checklist step 3 on the final tree, dev build with probes: fixture checksums, `cargo fmt --check`, `cargo clippy
  --all-targets -- -D warnings` (also `--all-features`), `cargo nextest run -j 2` (412 passed, 3 skipped, unless a
  binding unit test is added: the count in the note), `cargo deny check licenses`, `ruff check`, `ruff format --check`,
  `mypy`, `pytest -rs` with 0 skipped on the VM (227 + the new tests).
  (g) A T-18d note: the mutation table (each alone, failing test, `git status` clean after each), the exact stderr lines
  of (a) and (b) 3, what `oradmp.open("a\x00b.dmp")` raises, the counts.
- Touches: `crates/oradmp-py/src/lib.rs`, `python/oradmp/_cli.py`, `python/oradmp/_survey.py`, tests, `README.md`, the note.
  **Model: Sonnet.** Needs nothing from the owner but the commit. **Reviewed in Reviewer 6b** with T-17 and T-16 (one
  group, ADR-82 item 7); no closure review of its own unless the owner wants a fix release before T-16 and T-17.

## T-17 Synthetic files from the rules (arch 8.3 (b); session 15, ADR-62 item 4; "Done when" session 20, ADR-80) - DONE (2026-10-24, `edc62a5`; 429 Rust tests + 4 skipped; reviewed: review-06b, no blocking finding; SF-1 to SF-3, SF-5, N-1 to N-8 go to T-17b)
- Goal: a test-side generator that writes small synthetic exp files from the rules (lines, the three field forms, rows,
  both paths, both empty-table forms) and tests that the reader returns what was written.
- Inputs: `docs/formats/exp_reader_rules.md` (v3.6: sections 1 to 5, 7, 7a, 9, 10; the status column), arch 1 (the
  stop table), 2.5, 5.1, 8.3; ADR-80; `tools/fuzz_seeds.sh`, `tools/fuzz_run.sh`; review 04 N-9; review 06a N-5;
  `crates/oradmp-core/tests/common/` (existing helpers); the proptest and `tempfile` documentation of the pinned versions
  (if `tempfile` is not yet a dev-dependency, write below `target/` instead; no new dependency without a line in the note).
- **Done when:**
  (a) A generator module `crates/oradmp-core/tests/common/synth.rs` turns a `Spec` (header fields, mode, R, tables
  with columns, rows and path) into bytes. It writes only constructs whose rule is **confirmed** in the rules file
  and names the rule id beside the code that writes each (header [H1] to [H6]; lines and fields [I2] to [I7] with the
  list of rule K; table section [T1] to [T6], [T9] for the generated column types; rows [R1] to [R5], [R7], [R8];
  end [E1] to [E4]; LOB [L1] to [L4], [L6]). The note lists each rule id used with its status as read at the commit;
  a rule found "suggested" or "unknown" is not generated as valid input.
  (b) Types and values: NUMBER [Y1], DATE [Y2], TIMESTAMP [Y5], RAW [Y6], VARCHAR2 / CHAR in AL32UTF8 and
  CL8MSWIN1251 [C1, C6, C7], NVARCHAR2 / NCHAR [C3, C5], LONG / LONG RAW up to 32,000 bytes on both paths [Y9], CLOB /
  BLOB short form on the conventional path [L4, L5]. The bytes come from a test-side encoder written from those rules
  (`encoding_rs` allowed for the single-byte sets); the generator calls no function of `oradmp_core`. Expected values
  are the `Spec`'s own, never the reader's output.
  (c) Property test `t17_synth.rs`: a random `Spec` (1 to 4 tables, 0 to 300 rows, NULL density 0 to 100 %,
  conventional or direct path where the rules allow (a LOB table conventional only [L6]), R in {2048, 4096, 8192,
  16384}, table or owner mode) -> `open`, `scan`, `read_table` of every table: names, column count, Arrow types of
  arch 5.1, row counts and every value equal to the `Spec`, in file order. Default case count in `cargo nextest run`
  64, the whole binary under 60 s with `-j 2` (time in the note); once `PROPTEST_CASES=2000`, alone, result in the note.
  (d) Deterministic cases, one test each: both empty-table forms after a direct-path flag [E1, E2, E3]; a length field,
  a NULL marker and a value each split by a record boundary at every byte position [R5]; a 32,000-byte LONG on both
  paths; owner mode with three tables; R = 2048 and R = 16384.
  (e) Option sets of review 04 N-9 on generated files, each compared with the `Spec`: projection (`columns`, in a
  different order), `number_types` (`int64`, `string`, `decimal(p,s)`), `strict=true` on a clean file (no error, no
  warning), `batch_rows` 1 and 7 (batch boundaries); `interval_as` is out (no INTERVAL in the first version, ADR-80).
  (f) Negative cases with the generator's own offsets: an unknown type code [X1] -> `E-UNSUPPORTED-TYPE` at the
  descriptor; m not 0 and not the column count [X5] -> `E-UNSUPPORTED-LAYOUT`; truncation at a random offset -> a typed
  error whose offset is at most the file length (as `t14_damage`); each expected code from arch 1's table.
  (g) Fuzz seeds: a documented command (in `tools/fuzz_seeds.sh` or CONTRIBUTING) writes N synthetic files into the
  ignored seed folders of the three targets; `tools/fuzz_run.sh read 60` runs once with them, alone: findings 0 (a
  finding follows ADR-62 item 7).
  (h) No change to `crates/oradmp-core/src/`. A disagreement between reader and generator stops the task and goes to
  the Architect with the `Spec`, the bytes' offsets and the rule ids (the fixtures decide); no test is weakened or
  skipped to pass.
  (i) Files only below `target/` or a temporary folder; `git status --porcelain` is empty after `cargo nextest run` and
  after the `PROPTEST_CASES=2000` run.
  (j) The new property tests set proptest's failure persistence to none and print the failing seed in the message
  (proptest's `ProptestConfig::failure_persistence`); the note says whether `t14_damage.proptest-regressions` (review
  06a N-5) can be produced by `t14_damage`'s own property test under a mutation, and from which test.
  (k) Mutations, each alone in the reader, each fails at least one T-17 test (named in the note): `fe ff` read as an
  empty value; the [E2] test removed (always [E1]); two bytes skipped at each multiple of R inside table data; the sign
  of a negative NUMBER flipped.
  (l) Clean room: the generator follows the rules file only (no third-party writer or parser, no other description of
  the format). Checks of the definition of done green; Rust test count before / after in the note.
- Touches: `crates/oradmp-core/tests/common/synth.rs` (new), `crates/oradmp-core/tests/t17_synth.rs` (new),
  `tools/fuzz_seeds.sh` or CONTRIBUTING (one command), the note. Not `crates/oradmp-core/src/`, `corpus/`,
  `tests/golden/`, `crates/oradmp-core/tests/data/`, `docs/formats/`.
- Needs: the a3 tag pushed (release step 10 (g)); nothing from the owner but the start; alone on the VM for (c)'s long
  run and (g). Not with T-16 on the VM. **Model: Opus.** Reviewed in Reviewer 6b. Not a beta gate.
- (Session 15 / 16 / 19 text, history:) common mode: it adds coverage of combinations, not evidence; the evidence stays
  the fixtures and truth files; the synthetic files become fuzz seeds and give review 04 N-9's option sets a target;
  it writes only below `target/` or a temporary folder.
- **Session 21 (ADR-83)**: the a3 tag is pushed, so T-17 can start; it runs **after T-18d is committed** (one build at
  a time on the VM) and before T-16 while O-22 is open. "Done when" unchanged. It needs nothing from the owner but
  the start and a quiet VM for (c)'s `PROPTEST_CASES=2000` run and (g)'s fuzz run. It writes no file a CI runner could
  not write (temporary folders only), so T-16 can run it on every platform unchanged. Reviewed in 6b with T-18d and T-16.
- **Session 22 (ADR-85, ADR-86)**: done and reviewed (6b). (g) reads "the one ignored seed folder `target/fuzz-seeds/`,
  which `tools/fuzz_run.sh` passes to every target" (ADR-80 amendment). (a) and (d) read with ADR-80 item 1 as amended:
  the 18c E1 case of (d) is an `Unconfirmed` construct. Follow-up: T-17b.

## T-17b Review 06b fixes: unconfirmed generator constructs, the E2 guard, literals, output paths, regression files (session 22; ADR-86) - DONE (2026-10-24, `4b4e732`; nextest 432 passed + 4 skipped, pytest 235, as CI reproduced; to be reviewed in Reviewer 6c; ADR-89 item 3)

- Goal: the generator writes only fixture-shown combinations plus two labelled `Unconfirmed` ones; the E2 guard works;
  no expected value comes from reader code; every path in the tool's messages is written `\xHH`; no property test can
  leave a `*.proptest-regressions` file; review 06b's notes N-1 to N-8 settled.
- Inputs: review 06b (all of it), ADR-80 with its session 22 amendment, ADR-82 item 3 as amended, ADR-86;
  `docs/formats/exp_reader_rules.md` (H1 to H6, T5, Y5, E1 to E3, C4, "Status and scope"); the T-17 and T-18d notes;
  `crates/oradmp-core/tests/common/synth.rs`, `t17_synth.rs`, `t14_damage.rs`, `t4_values.rs`, `t5_types.rs`;
  `python/oradmp/_cli.py`; `tests/test_paths.py`; `tools/fuzz_seeds.sh`; `CONTRIBUTING.md`; proptest documentation of the
  pinned version (`ProptestConfig`, `failure_persistence`, `prop_map`).
- Every item below has a test, a mutation or a check that **fails on `edc62a5` (or under the mutation) and passes on the
  T-17b commit**; the note's table names it. Mutations each alone, restored after each, `git status` clean after each.
- **Done when:**
  (a) **SF-1, combinations.** (1) Database set 171 is drawn only with an 11g header (`random_spec` and
  `every_generated_type_with_edge_values`; the CP1251 part of the latter uses an 11g header). (2) TIMESTAMP precisions
  are drawn from {0, 6, 9} only. (3) The header choices the generator can draw are one list in `synth.rs` (version,
  mode, client set, database set, optional `Unconfirmed` label) that `random_spec` draws from. (4) New test
  `header_sets_match_the_fixtures`: it reads every `.dmp` in the three fixture folders whose byte 0 is `03` and whose
  first text line starts `EXPORT:V`, with its own byte reading by [H1] to [H4] (no `oradmp_core` call), asserts 201
  files, collects the (version, mode, client, database) of each, and asserts that every unlabelled entry of the list
  occurs, that every entry labelled `ClientSetNotInFixtures` does not, and that no entry pairs an 18c header with
  database 171. (5) The coverage test asserts that every TIMESTAMP precision drawn is in {0, 6, 9} and each of the three
  occurs. Mutations: the label removed from the client 171 / database 873 entry; `v18 && db_171` allowed again; an
  unlabelled 18c / 171 entry added; `rng.range(0, 9)` back: each fails (4) or (5) by name.
  (b) **SF-1, labels.** `enum Unconfirmed { E1AfterFlag1In18c, ClientSetNotInFixtures }` in `synth.rs`, no other
  variant. Every writing site of such a construct carries `// UNCONFIRMED(<Variant>): <rule ids>; accepted by <rule>;
  evidence: <R-017 | none>`. Each file's `Layout` holds the set of variants it contains; every failure message of
  `t17_synth.rs` prints it. `both_empty_table_forms_after_a_direct_path_flag` keeps its 18c E1 table and asserts that its
  `Layout` holds `E1AfterFlag1In18c`; a file with only E2 tables, or an 11g file with E1, holds no variant (asserted).
  Check: `grep -c 'UNCONFIRMED(' synth.rs` equals the number of writing sites the note lists. Mutation: the 18c E1
  writing site no longer records its variant: the assertion fails.
  (c) **SF-2.** The guard in `gen_noise_after` tests `v.is_empty()`. New test
  `no_e2_table_is_followed_by_an_item_outside_the_e2_list`: over `params_from_seed(0..S)`, every E2 table is followed
  (at p + 1) by bytes that start with one of the 14 entries of the [E2] (a) list (`TABLE "`, `ENDTABLE`, `METRICS`,
  `CREATE `, `ALTER `, `COMMENT ON `, `GRANT `, `ANALSTATS `, `CALTER TABLE`, `PARTITION "`, `COMPARTITION "`,
  `SUBPARTITION "`, `ENDCOMPARTITION`, `ENDPARTITION`), written as literals. S is chosen so that the old guard gives at
  least 5 hits (6b: 131 in 200,000 seeds, so about 40,000) and the test stays under 10 s; the note gives S, the time, and
  the hit count under the mutation. Checking the generator's item list instead of the bytes is allowed if the bytes
  take longer; the note says which. Mutation: `i == 0` back: the test fails.
  (d) **SF-3.** `number_types_override_the_arrow_type` writes `DataType::Int64`, `DataType::Utf8`,
  `DataType::Decimal128(20, 4)` as literals; `nt.arrow()` is gone from `t17_synth.rs`, and no expected value there comes
  from an `oradmp_core` function (the note lists every `oradmp_core` item `t17_synth.rs` uses, each an input or the call
  under test). Mutation (in `src/`, restored): `NumberType::arrow` gives `Utf8` for `Int64`: the test fails.
  (e) **SF-4.** In `python/oradmp/_cli.py` every path written into a message goes through `_path_text`: the target of
  the "exists" refusal (`_refuse_existing`), the `filename` (and `filename2`, if used) of an output-side `OSError`, and
  any other path the Implementer finds (the note lists each site from a grep). New tests in `tests/test_paths.py` (Linux
  only, as the module): `export ... T_TEXT -o <bytes path>` onto an existing file named `d2 e5 f1 f2 2e 63 73 76` exits as
  today with stderr containing `<dir>/\xd2\xe5\xf1\xf2.csv` and no `\udc`; into a read-only folder with such a name, stderr
  contains `\xd2\xe5\xf1\xf2.csv.oradmp-partial` (skipped as root, like `test_cli.py:576`). Mutations: `_refuse_existing`
  without `_path_text`; the `filename` branch without it: each fails its test. README unchanged.
  (f) **SF-5.** `t14_damage.rs`, `t4_values.rs`, `t5_types.rs` set `failure_persistence: None` (`..ProptestConfig::default()`
  for the rest, so `PROPTEST_CASES` still applies); the doc line of `t14_damage.rs:11-12` about a committed regression
  file is replaced. Checks: for every file under `crates/oradmp-core/tests/` that contains `proptest!`, it also contains
  `failure_persistence: None` (the note shows the grep); under the mutation `ByteStream::truncated` at `offset + 100000`
  `t14_damage` fails and `find . -name '*.proptest-regressions'` prints nothing; the same `find` after one mutation that
  fails a property test of `t4_values` (the Implementer's choice, named).
  (g) **N-2.** New deterministic test: a two-column LONG table on the direct path with one 32,000-byte value written as
  `9c 80` + 156 bytes, three times `19 9f` + 7,961 bytes, `19 1f` + 7,961 bytes; the test asserts these five piece headers
  at the `Layout` offsets and the value read back. Mutation O4 of 6b (a chain stops after 4 pieces): it fails.
  (h) **N-4.** The proptest strategy is `any::<u64>()` mapped through `params_from_seed`; the separate probabilities
  of `params()` are gone; failure messages print the seed. Check: `grep -n 'fn params' synth.rs` shows the mapping only.
  (i) **N-1.** The zero bytes at R + 0x000 to R + 0x102 carry `// CONTRADICTS FIXTURES (harmless: [H6] skip)`; no
  other generator change for N-1.
  (j) **N-7, N-8.** The three lines T-17 added above 120 characters are wrapped: `git diff ba8a89c HEAD --
  CONTRIBUTING.md tools/fuzz_seeds.sh | grep '^+' | awk 'length > 121'` prints nothing. `tools/fuzz_seeds.sh` changes to
  the repository root before `cargo test` and prints `cargo --version` once; run from inside `fuzz/` with `SYNTH_SEEDS=2`
  it prints the root toolchain's version (and the fuzz nightly without the `cd`: the note shows both lines).
  CONTRIBUTING gains one sentence: the seed step compiles the `oradmp-core` test binary with the root toolchain.
  (k) **The T-17 note** gets a dated "Corrections (T-17b)" block (the original text stays): E3 suggested; the 18c E1
  case and the client pairs `Unconfirmed` (with R-017 as the route for the first); the TIMESTAMP length source [Y5] with
  `t_ts` and `t_decl` for (0), (6), (9), not `types/map.rs`; H4's 10 bytes constant in 201 of 201 files (6b), meaning
  unknown; zeros at R + 0x000 to R + 0x102; three own mistakes, not four; K1: 12 of 17 tests in a `--no-fail-fast`
  run; K2: the two deterministic tests carry it, the property tests only at higher case counts (N-3); the coverage test
  measures the property test's distribution now (N-4); N-1 (a) to (e) as "differs from the fixtures, the reader does
  not depend on it ([T2], [T7], [H6])"; the "not generated" list completed with 6b's N-5 items and the fixture test of
  each.
  (l) **Nothing else changes**: `git diff --stat edc62a5 <T-17b commit> -- . ':!docs'` lists only
  `crates/oradmp-core/tests/common/synth.rs`, `crates/oradmp-core/tests/t17_synth.rs`, `t14_damage.rs`,
  `t4_values.rs`, `t5_types.rs`, `python/oradmp/_cli.py`, `tests/test_paths.py` (and `tests/test_cli.py` if a shared
  helper moves), `tools/fuzz_seeds.sh`, `CONTRIBUTING.md`. Not `crates/oradmp-core/src/`, `corpus/`, `tests/golden/`,
  `crates/oradmp-core/tests/data/`, the Cargo files, `pyproject.toml`; no version change.
  (m) **Checks**: the definition of done (fixture checksums, fmt, clippy twice, `cargo nextest run -j 2` with 429 +
  the new tests passed and 4 skipped, `cargo deny check licenses`, ruff, ruff format, mypy, `pytest -rs` with 233 + the
  new tests and 0 skipped on the VM, using `.venv/bin/python`; no rebuild is needed for `_cli.py`); once, alone,
  `PROPTEST_CASES=2000` on `t17_synth` (time in the note); `git status --porcelain` empty after each run.
- Touches: as (l), and the note. **Model: Sonnet.** Needs nothing from the owner but the commit. No closure review of
  its own: reviewed in **Reviewer 6c** with T-16a and the a4 candidate (ADR-86 item 14).

## First release (alpha, 0.1.0a1)

After T-13. It contains (session 4 wording): table and owner mode, both paths, 11g and 18c files of
same-version `exp`; the 16 types of [T5] (NCHAR and `CHAR(n CHAR)` included, non-BMP text included)
with the mapping of arch 5.1, declared types of [T9] matched by name and the schema in declared order
(ADR-24); CLOB, NCLOB, BLOB by [L1 to L6] (session 8: decoded in both forms in AL32UTF8 and CL8MSWIN1251
databases; in a database of any other set a long-form CLOB value and any NCLOB value are refused per value,
`E-LOB-UNCONFIRMED`, or decoded with a warning on request, ADR-25); INTERVAL as Interval(MonthDayNano) / Duration, ROWID and UROWID (physical and logical)
as text; partitioned and range-hash composite-partitioned tables as one table with its parts listed;
LONG and LONG RAW as LargeUtf8 / LargeBinary on both paths (direct-path chains [R2]; values above 32,000
bytes with a warning; a conventional value of 32,768 bytes or more stops the file until R-040); the
NUMBER overflow policy of the owner's choice (Q-1) and `decimal256(p,s)` overrides; the five character
sets with Oracle's map [C7]; scan and catalog with exact row counts; `read_table` as a stream of batches
into pyarrow; typed errors and warnings; the structure-only `survey` with the per-candidate NUMBER
profile, the LOB, LONG and partition counts. It does **not** contain: BFILE, object, REF, collection or
XML columns (a table with one stops the file), `ReadOptions.partitions`, a
partition-name column, a tested 10.2 client, parallel decoding, measured performance figures (T-15) or
fuzzing results (T-14). The README says so in these words. Beta: after T-14, T-15 and a `survey` of a
real dump that agrees with `python/exp_survey.py`.

Session 11 (ADR-47): `interval_as="string"` **is in the alpha** (Utf8 `TO_CHAR` text for every INTERVAL column; the
native Interval(MonthDayNano) cannot be written to Parquet by pyarrow). The wheel is `abi3-py311` for Linux x86_64,
`manylinux_2_28` (ADR-48); other platforms after the alpha.

Session 12: Q-9 is answered (RHEL 8+, x86_64 only); the wheel stays `manylinux_2_28`; other platforms are T-16.
The version is `0.1.0-alpha.1` in Cargo and `0.1.0a1` for Python (ADR-50).

Session 13 (ADR-54): the project is **`oradmp`** (PyPI `oradump` belongs to another project); the release is built
after T-13g, under the new name only: `oradmp-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl`, `oradmp-0.1.0a1.tar.gz`.

Session 14 (ADR-55 to ADR-60): the release is built after **T-13h**, by `tools/release_build.sh` (clean clone at a
neutral path, remapped paths, SBOM kept, ADR-56); the C stream path reports warnings (ADR-55); a panic is
`E-INTERNAL` (ADR-59).

Session 15 (ADR-63, owner's Q-12 answer): **0.1.0a1 is an internal hand-out** (for the install test on RHEL 8), never
uploaded. The PyPI upload comes after Reviewer session 4 and is the build of the reviewed commit, version
**0.1.0a2** (the README of T-14 and any fuzz fix change shipped files). The checklist below runs twice: run A for
0.1.0a1 now, run B for 0.1.0a2 after Reviewer 4.

Session 16 (ADR-67): **run A**, as far as the repository shows: tag `v0.1.0a1` on `b7304d5` (docs-only after
`39cbb93`); its step 6 output, the RHEL 8 result (step 8) and the hand-out are not recorded in the repository: the
owner records them (step 11) or says they are not done. **Run B** comes after T-13i (whose last commit is the version
bump) and a short Reviewer session 5 (closure); it uploads to **TestPyPI first**, then PyPI. The steps below are
revised for it.

Session 18 (ADR-70): **run B is done**; record below. 0.1.0a2 is on TestPyPI and PyPI.

Session 19 (ADR-74): the record is corrected: the built and tagged commit is **`ff7d525`** (session 18 wrote
`4e8f212`, the README commit with the same message; the two differ only under `docs/`). O-18 is closed.

#### Run B record (session 18; corrected in session 19, ADR-74; the owner's facts, checked against the repository where it can be)

| step | result |
|---|---|
| 1 Gate | Reviewer 5 (`dc7668e`): no blocking finding; Analyst session 11 done; Q-10 answered **upload**. **Deviation**: the candidate **`ff7d52559551a973621c68154e03dba1a157731a`** (session 19; it is `4e8f212`, the README change after the review, plus a docs-only commit) changes `README.md` after the review (owner's decision on review 05 SF-2, option (b)); `git diff --stat dc7668e ff7d525 -- . ':!docs'` lists only `README.md` (1 insertion, 2 deletions; repeated by the Architect in session 19). Accepted without re-review (ADR-70 item 2). |
| 2 Name | `oradmp` now exists on TestPyPI and PyPI (secured). |
| 3 Checks | **Not run on the candidate** (owner, session 19); accepted for a2 (ADR-74 item 4): Reviewer 5 ran them on `dc7668e` (411 + 3 ignored, 117 pytest, 0 skipped), the candidate differs by one README line, the build printed `check_dist: ok`. 3b not due (no core source change since the baseline). |
| 4 Network checks | Reviewer 5 on `dc7668e`: `cargo deny check` all ok; no owner run on `ff7d525` in the record (the lock files did not change). |
| 5 Version | `0.1.0-alpha.2` / `0.1.0a2` (T-13i (l), `dc7668e`). |
| 6 Build | `tools/release_build.sh ff7d525... --twice` (session 19: the build clone's `rev-parse HEAD`; epoch 1792692632 = `ff7d525`'s commit time): wheel and sdist identical in both builds, "reproducible". Wheel `oradmp-0.1.0a2-cp311-abi3-manylinux_2_28_x86_64.whl` sha256 `255cb122dcca3979e171984f89a719a293770709baf09a47936335f80b9b0f1c`; sdist `oradmp-0.1.0a2.tar.gz` sha256 `cae95420b83d29866f8b939d8427cd857902a0055abe466baa66e9753c88702d`. |
| 7 Install | Rocky Linux 8 container, python3.11, pyarrow 26.0.0: **113 passed, 4 skipped** (3 probe tests; 1 `test_project_name.py` "not a git clone", explained: copied folders, no `.git`; ADR-70 item 3). Staged for the release-tooling tests: `Cargo.toml`, `Cargo.lock`, `README.md`, `pyproject.toml`, `tools/`, `docs/design/implementer-notes.md`, `fuzz/Cargo.lock`. pyarrow 15.0.2 on the installed wheel: Reviewer 5 on `dc7668e` (114 passed, 3 skipped); not repeated on the candidate (README only). |
| 8 RHEL 8 host | **Not done** (open; also under Q-4). |
| 9 Tag | `v0.1.0a2` (annotated) on **`ff7d525`**, the built commit, and on the server (owner, session 19; `git rev-parse v0.1.0a2^{}` = `ff7d5255...`, checked). `v0.1.0a1` on `b7304d5`: **local only, not pushed** (owner, session 19; session 18's "pushed" was wrong). |
| 10 Publish | TestPyPI 2026-10-22: `uv publish --publish-url https://test.pypi.org/legacy/ --check-url https://test.pypi.org/simple/ <wheel> <sdist>`; index sha256 = step 6; `pip install --no-deps --index-url https://test.pypi.org/simple/ "oradmp==0.1.0a2"`, `import oradmp` 0.1.0a2, `python -m oradmp survey --help`; page shows the licence and "Requires Python >=3.11"; account-wide token replaced by a project-scoped one. **PyPI 2026-10-22: done**; `https://pypi.org/simple/oradmp/` sha256 of both files = `dist/SHA256SUMS`; `oradmp==0.1.0a2` installed from pypi.org in a fresh venv and runs (owner, session 19). Tokens of both indexes now scoped to `oradmp`; the account-wide ones deleted. A stray file `UV_PUBLISH_TOKEN` in `dist/`, made during the upload, found and removed (ADR-74 item 3). |
| 11 Record | This table (Architect, session 18; corrected in session 19). |

What remains of run B (session 19): only step 8 on a real RHEL 8 host (Q-4). O-18 is closed (ADR-74). Optional:
the owner pushes `v0.1.0a1` (recommended, ADR-74 item 5). Nothing remains to upload.

### Release checklist (session 12, ADR-50; revised in sessions 14, 15, 16, 18 and 19, ADR-56 to ADR-63, ADR-66, ADR-67, ADR-70, ADR-74)

**Session 23 (ADR-90), for 0.1.0a4 and later:**
- Step 6: first `ls -d /var/tmp/oradmp-release` must fail; if it exists, `mv /var/tmp/oradmp-release
  /var/tmp/oradmp-release-<what it holds>`. Then, with `ORADMP_BUILD_ROOT` unset, `tools/release_build.sh <C> --twice
  --release` (after T-16a2); the log shows `release root: canonical` and `root: /var/tmp/oradmp-release`.
- **Authoritative hashes**: this step's `SHA256SUMS`. Cross-check with the CI summary of the run on C (full hashes):
  the sdist must be equal (a gate); the wheel must be equal if O-23 (a) confirmed the build-root hypothesis (then a gate),
  otherwise the difference is recorded with the differing members.
- **The claim** the checklist, `CONTRIBUTING.md` and the README may make is ADR-90 item 6, in the form that holds at that
  release (before the confirmation: sdist across machines, wheel on one machine twice; after it: both across the VM and
  the CI runner at the canonical root). Never "on any machine".

**Session 21 (ADR-81 item 3, the a3 lessons), for 0.1.0a4 and later:**
- Step 6: run `set -o pipefail` in the shell first, or check `echo "${PIPESTATUS[0]}"` right after the `| tee` line
  (must print `0`); judge also by the log (`check_dist: ok` twice, `identical` twice, the `commit:` line). Then compare
  the sdist hash with the one in the closure (or 6b) review: the sdist does not depend on the build root, so a
  difference stops the release. The wheel hash is compared only with the owner's own `SHA256SUMS`.
- Step 7 as **root** in a container (as for a2 and a3): expected skips are exactly the three probe tests of
  `tests/test_internal.py`, `tests/test_cli.py:576` ("root can write to a read-only folder"), `tests/test_cli.py:897`
  ("root can read a file of mode 000") and `tests/test_project_name.py:34` ("not a git clone"; no `git` in the image):
  **6**; in the venv without pandas also `tests/test_consumers.py`: **7**. If the image has `git`, first `git config
  --global --add safe.directory <mount point of R/src>` (else `test_project_name.py` fails on "dubious ownership"); it
  then runs, and the counts are 5 / 6. As a normal user on the VM: the three probe tests (git clone), plus
  `test_project_name.py` (copied folders), plus `test_consumers.py` (no pandas). The line numbers are those of `ff3d6b2`;
  the checklist of each release names them again from its candidate (`grep -n geteuid tests/*.py`). All **three**
  venvs of step 7 run on the owner's wheel (a3's single venv was a one-off, ADR-81 item 2).
- Step 9: `git tag -a v<v> -m "oradmp <v>" <full hash of C>` (the hash, never an implicit `HEAD`), then `git rev-parse
  'v<v>^{commit}'` against the build log's `commit:` line.
- Step 10: unchanged and confirmed by a3: `read -rs` / `export` / `unset` around each upload, the two files named one by
  one, `--check-url`, the tag pushed to `origin` and `github` only after PyPI, then `git ls-remote --tags` on both.
  Record `pip --version` with the plain `pip install oradmp` observation (a3: it installed the pre-release, as PEP 440
  allows when only pre-releases exist).

**Session 20 (ADR-77 item 7), for 0.1.0a3 and later:** the a3 run follows "Release 0.1.0a3 checklist (session 20)"
below, which writes out every step with the file names. Corrections that hold for later releases too: steps 5, 9 and
10 name `<v>` (not a2's literal values); step 7's expected skips depend on the layout (git clone: the three probe
tests; copied folders: also `test_project_name.py` "not a git clone"; a venv without pandas: also
`test_consumers.py`); a new build root per release (`release_build.sh` refuses an existing one) and the step 6 log kept
with `tee`; the tag is made at step 9 but **pushed only after the PyPI upload** (10 (g)); the plain `pip install
oradmp` behaviour is recorded, not assumed.

**Session 19 (ADR-74 item 6, ADR-75 item 2), for 0.1.0a3 and later:**
- Step 3 runs **on the candidate itself** (a2's was not; ADR-74 item 4).
- Step 3b for a3: throughput and memory are covered as a "no regression" check by T-15b (code of `460aef5`, the same
  core sources as the candidate); the micro-benchmarks by T-18b (b); Reviewer 6a repeats all three. Neither is a new
  baseline until the Architect copies them into `benchmarks.md` after the repeat.
- Step 10 (5): after the uploads, `ls -A <dist>` lists exactly the wheel, the sdist and `SHA256SUMS`; anything else
  is removed, and a file that can hold a token means that token is revoked.

**Session 18 (ADR-70, the run B lessons), for 0.1.0a3 and later; read with the steps below:**
- Step 1: the candidate is `HEAD` of `master` after the closure review; `git diff --stat <reviewed commit>
  <candidate> -- . ':!docs'` is empty, or lists only files changed by the owner's decision on a review finding; then
  the diff goes into the record and `check_dist.py` and `pytest tests/test_project_name.py` run green on the candidate
  in a git clone.
- Step 6: always `tools/release_build.sh <commit> --twice`. The hashes depend on the commit (its time is
  `SOURCE_DATE_EPOCH`) and on the build root (the SBOM holds the path): compare uploads only with this run's
  `SHA256SUMS`.
- Step 7: a container install test runs on the whole build clone `$ORADMP_BUILD_ROOT/src` (a git clone), mounted
  read-only, not on copied folders; expected skips are exactly the three probe tests. From T-18 on: `oradmp --help`
  and `oradmp tables` on a copied fixture from the venv's `bin/`, and one venv **without pandas** runs `oradmp
  export` to CSV and to Parquet.
- Step 9: before pushing, `git rev-parse v<v>^{commit}` equals the `commit:` line of the step 6 log.
- Step 10: (1) `uv publish --dry-run` asks for the token; it is not a credential-free check: skip it, the TestPyPI
  upload is the rehearsal. (2) Name the two files one by one (`<dist>/oradmp-<v>-cp311-abi3-manylinux_2_28_x86_64.whl
  <dist>/oradmp-<v>.tar.gz`), never a glob (`dist/` also holds `SHA256SUMS`). (3) Tokens: from a3 on, the
  project-scoped tokens of both indexes (a project-scoped token can be made only after a project's first upload);
  typed with `read -rs UV_PUBLISH_TOKEN; export UV_PUBLISH_TOKEN`, `unset UV_PUBLISH_TOKEN` right after. (4) The
  install check: pyarrow from PyPI first, then `pip install --no-deps --index-url <index>/simple/ "oradmp==<v>"`
  (`--index-url`, not `--extra-index-url`). Use `--check-url <index>/simple/` on each upload (a retry then skips files
  already there). A file missing from the simple page right after an upload is index lag: retry for a few minutes
  before calling it a mismatch.


Who: **O** = the owner, **I** = the Implementer (a session of its own or the end of a task). Each step's result goes
into the release note (a short section in `implementer-notes.md`), with the commit hash. Machine rule of ADR-62 item 2
for every build and test step.

1. **Gate** (O). Run A: T-13h done and committed (`39cbb93`, done); Q-12 answered (done: internal hand-out); the
   working tree clean on `master` (the untracked prompt file under `docs/prompts/` is the owner's: commit it or leave it,
   it is in neither archive). Run B: Reviewer session 4 reported no blocking finding; T-14 and T-15 committed.
   **Run B (session 16)**: T-13i committed with its version commit (T-13i (l)); Reviewer session 5 (closure)
   reported no blocking finding; **Analyst session 11** done (rules T9 and X2; the rules file is not shipped, but
   the release should not go out with a known wrong number in the evidence); **Q-10** answered; the candidate commit
   is `HEAD` of `master`, and `git diff --stat <T-13i version commit> <candidate>` lists only files under `docs/`.
2. **Name** (O; ADR-54): `oradmp` free on PyPI (checked 2026-10-18); secured only by the first upload (run B, step 10).
   crates.io: nothing is published (ADR-50 item 3).
3. **Checks** (I): `sha256sum -c SHA256SUMS` in the three fixture folders; `cargo fmt --check`, `cargo clippy
   --all-targets -- -D warnings` (also with `--all-features`), `cargo nextest run -j 2`, `cargo deny check licenses`,
   `ruff check`, `ruff format --check`, `mypy`, `pytest -rs` in the dev build (`--features test-probes`): **0 skipped**;
   counts recorded (run A: 402 Rust tests, 105 pytest at `39cbb93`).
   **3b. Benchmarks** (I; session 16, ADR-66): only when a file of `crates/oradmp-core/src/`, the release profile or
   the toolchain changed since the baseline commit of `benchmarks.md`: the commands of `benchmarks.md` section 3, alone
   on the VM, compared under its section 5 rules; a confirmed regression stops the release. Run B: T-13i changes no
   core source file except unit tests inside `reader.rs` / `framing.rs` (test-only code), so step 3b records "not
   due" with the `git diff --stat` that shows it.
4. **Network checks** (O, once per run, from a clone of the commit, on a machine with network): `cargo deny check
   advisories`, `cargo deny check sources`, `cargo deny check bans`; output into the release note. An advisory against
   a crate in the wheel's graph (`cargo tree` for `x86_64-unknown-linux-gnu`) stops the release until the Architect
   decides.
5. **Version** (I): `cargo metadata` shows `0.1.0-alpha.1` (run A) / `0.1.0-alpha.2` (run B: the Implementer sets it
   in `[workspace.package]` and `Cargo.lock`, two lines, one commit) for both crates; `pyproject.toml` has `dynamic =
   ["version"]`. **Run B (session 16)**: done by T-13i (l) (root `Cargo.toml`, two `Cargo.lock` entries, the
   `oradmp-core` entry of `fuzz/Cargo.lock`, the README's status line and install text); this step only checks
   `cargo metadata` and `grep -n 0.1.0a2 README.md`.
6. **Build** (O or I): `tools/release_build.sh <commit> --twice` (session 18) on the commit to be tagged; it prints `check_dist: ok`, the
   commit, the rustc / maturin / zig versions and `$ORADMP_BUILD_ROOT/dist/SHA256SUMS` for the wheel
   `oradmp-<v>-cp311-abi3-manylinux_2_28_x86_64.whl` and the sdist `oradmp-<v>.tar.gz`. These two files are the
   release; nothing is rebuilt later. **Run A**: the owner has run the script once on the committed tree; the
   Architect does not know the result. The owner records it (commit printed, `check_dist`, hashes); if it failed, or
   if it was not run on the commit to be tagged, it is run again (ADR-63 item 2: the tag goes on exactly the commit the
   build printed).
7. **Install** (I): the wheel of step 6 in fresh venvs of Python 3.11 with **pyarrow 15.0.2 and pandas 2.2.3**, and of
   the newest CPython with the newest pyarrow and pandas; `pytest -rs` from the build clone `$ORADMP_BUILD_ROOT/src`
   against the installed wheel on both. Expected skips: exactly the three probe tests of `tests/test_internal.py` (no
   `test-probes` in a release); any other skip is listed with its reason and stops the release until explained. The
   note names the outcome of `test_interval.py` and `test_consumers.py` on pyarrow 15.0.2 (review 03 N-5, ADR-61 item
   8). Also a **source build** once: the sdist installed by `pip install` in a fresh venv (Rust and network for
   crates.io; O if the VM has no network), then `import oradmp` and `python -m oradmp survey` on a copied fixture.
8. **Acceptance** (O): the wheel installs on a real RHEL / Oracle Linux 8 host (glibc 2.28) and `python -m oradmp
   survey` runs there on a copied fixture (no real data leaves that host). Run A is this install test.
9. **Tag** (O): annotated tag `v0.1.0a1` (run A) / `v0.1.0a2` (run B) on the commit built in step 6; pushed.
10. **Publish** (O). Run A: hand out the two files with their SHA-256; **no upload** (ADR-63 item 1); the README stays.
    Run B (Q-10 upload part): upload the two files of step 6 unchanged to PyPI (SHA-256 compared with step 6's), then
    the README install line becomes `pip install "oradmp==0.1.0a2"` in a follow-up commit.
    **Run B, session 16 (ADR-67; replaces the sentence above)**, all by the owner in his own shell, no agent present:
    (a) `read -rs UV_PUBLISH_TOKEN; export UV_PUBLISH_TOKEN` with the **TestPyPI** token from the password manager;
    `uv publish --publish-url https://test.pypi.org/legacy/ --check-url https://test.pypi.org/simple/ <wheel> <sdist>`
    (session 18: no `--dry-run`, it asks for the token too; the two files named, no glob); `unset UV_PUBLISH_TOKEN`.
    (b) In a scratch folder outside the repository: `curl -s https://test.pypi.org/simple/oradmp/` lists both files
    with a `#sha256=` fragment (PEP 503); both equal step 6's `SHA256SUMS`; `pip download --no-deps --only-binary
    :all: --index-url https://test.pypi.org/simple/ "oradmp==0.1.0a2" -d dl` and `sha256sum dl/*` agree (the sdist is
    not fetched with `pip download`, which would build its metadata); a fresh venv with pyarrow from PyPI, then `pip install --no-deps --index-url
    https://test.pypi.org/simple/ "oradmp==0.1.0a2"`, `python -m oradmp survey` on a copied fixture. Any difference
    stops here (TestPyPI file names are cheap; PyPI's are not: a used file name is refused forever).
    (c) The same as (a) with the **PyPI** token and the default endpoint (no `--publish-url`); then (b) against
    `https://pypi.org/simple/`.
    (d) Tokens: the first PyPI upload needs an account-wide token; right after it the owner creates a token scoped to
    the project `oradmp`, deletes the account-wide one, and keeps the scoped one only in the password manager. No token
    is ever written to a file in the repository, `~/.pypirc`, a shell history or an agent's prompt. The README already
    names `pip install "oradmp==0.1.0a2"` (T-13i (l)); no follow-up commit.
11. **Record** (I or Architect): the plan's "First release" marks the run done with the tag, commit and hashes.
    Session 16: the owner gives the Architect (next session) for each run: commit built, the two SHA-256, the RHEL 8
    result, and for run B the TestPyPI and PyPI comparison lines; the Architect writes them here.

### Release 0.1.0a3 (session 18; ADR-71 item 14; rewritten in sessions 19 and 20, ADR-76, ADR-77) - DONE (2026-10-24, `ff3d6b2`, tag `v0.1.0a3`; record below, ADR-81)

#### Release 0.1.0a3 record (session 21; the owner's facts, checked against the repository where it can be)

| step | result |
|---|---|
| 1 Gate | Reviewer 6a-closure: no blocking finding at **C = `ff3d6b25c360f93eeb8c1305927dd6cf3550610b`** (T-18c). Linux x86_64 `manylinux_2_28` only, no Project-URL, version `0.1.0a3`. |
| 2 Name | nothing to do. |
| 3 Checks | the closure review's run on C in a clean clone: fmt, clippy (three forms), `cargo nextest run -j 2` 412 passed + 3 skipped, `cargo deny check` ok, ruff, ruff format, mypy, `pytest -rs` 227 passed, 0 skipped (probes build). **3b**: not due (`git diff --stat 460aef5 C -- crates/oradmp-core/src rust-toolchain.toml` empty; review section 4). |
| 4 Network checks | not in the owner's facts ("not recorded"); the closure review's `cargo deny check` (advisories, bans, licenses, sources) was ok. |
| 5 Version | `0.1.0-alpha.3` / `0.1.0a3` (review section 4). |
| 6 Build | `tools/release_build.sh ff3d6b25c360f93eeb8c1305927dd6cf3550610b --twice`: `check_dist: ok`, wheel and sdist identical in both builds ("reproducible"). Wheel `oradmp-0.1.0a3-cp311-abi3-manylinux_2_28_x86_64.whl` sha256 `15db873fd7b3b8700b0a1aa9ef65094cf89eae4a0b31f575e3f2c3a4dce2aa16`; sdist `oradmp-0.1.0a3.tar.gz` sha256 `36bc571d75d6b6bb66968e31ee4144ba46324aa9efa174888ea973ed6e9293e5` (= the closure review's sdist hash; its wheel hash `0279e692...` differs by the build root in the SBOM, as expected). `dist/` held exactly W, S and `SHA256SUMS`. |
| 7 Install | Rocky Linux 8 container, python3.11, pyarrow 26.0.0, pandas present, **as root**: `oradmp --version` and `oradmp.__version__` 0.1.0a3; **221 passed, 6 skipped = 227** (the clean-clone total). Skips: the three probe tests; `test_cli.py:576` and `:897` (root); `test_project_name.py` "not a git clone". **Deviation** (ADR-81 item 2): one venv only; the pyarrow 15.0.2 venv, the newest CPython, the pyarrow-only export and the sdist source build stand on the closure review's run of the same commit (three venvs, both layouts, 0 failed). |
| 8 RHEL 8 host | **not done** (open since a1; Q-4). |
| 9 Tag | `v0.1.0a3`, annotated, made with the explicit hash `ff3d6b25...`; `git rev-parse 'v0.1.0a3^{commit}'` = C (checked in session 21). |
| 10 Publish | TestPyPI: upload, hash comparison, install from it: done. PyPI: done; the sha256 values on `https://pypi.org/simple/oradmp/` equal step 6; a plain `pip install oradmp` in a fresh venv picks 0.1.0a3 and installs pyarrow; `oradmp --version` and `python -m oradmp survey --help` work (pip version not recorded; `pip install --pre` not reported). Both uploads with project-scoped tokens typed with `read -rs` and unset afterwards. Tag pushed: on `github` (checked in session 21: `git ls-remote --tags github` lists `v0.1.0a3` -> `ff3d6b25...`); on `origin` (Gogs) by the owner's report pending confirmation (the Architect cannot query it). `v0.1.0a1` stays local only. |
| 11 Record | this table (Architect, session 21). |

What remains of a3: the RHEL 8 host test (Q-4, not a gate); the owner's confirmation that `v0.1.0a3` is on `origin`.
The a2 record says `ff7d525` (Run B record, session 19; ADR-74); `git rev-list -n1 v0.1.0a2` = `ff7d5255...` (checked
again in session 21).

**Session 20 (ADR-77; replaces "When" and "Steps" below; the README list stands, plus T-18c's two sentences):**
- When: after **T-18c** is committed and **Reviewer 6a-closure** reports no blocking finding. `d4d20ef` is not
  released. Linux x86_64 `manylinux_2_28` only, no Project-URL (ADR-79 item 5). Version `0.1.0a3`, no version commit.
- Content: as session 19, plus T-18c (the I/O error line with reason and path; the README suffix sentence).
- Steps: the checklist below, which the owner follows as written. Step 3 is the closure review's run on the same
  commit; step 3b is "not due" (no core source change since baseline 2's commit `460aef5`).

#### Release 0.1.0a3 checklist (session 20; for the owner, in order)

Names used below: **C** = the full hash of the candidate (T-18c's last commit, named in the closure report);
**R** = `/var/tmp/oradmp-a3` (the build root: must not exist yet, not inside the home folder, no user name in it);
**W** = `oradmp-0.1.0a3-cp311-abi3-manylinux_2_28_x86_64.whl`; **S** = `oradmp-0.1.0a3.tar.gz`; **X** =
`/var/tmp/a3-check` (a scratch folder outside the repository, with a copied fixture, e.g.
`X/t_text_exp_18c.dmp` from `corpus/fixtures/18c/`). Steps 9 and 10 are done by the owner alone, with no agent
session running.

1. **Gate.** The closure report: no blocking finding; it names C. In the clone: `git status --short` lists nothing
   but untracked prompt files; `git diff --stat C HEAD -- . ':!docs'` prints nothing. Confirm for the record: Linux
   x86_64 only, no Project-URL, version 0.1.0a3.
2. **Name.** Nothing to do (`oradmp` is the project's on TestPyPI and PyPI).
3. **Checks.** Take the counts from the closure report (its step 3 run on C). If C is not the commit it names, stop.
   **3b.** Record "not due": `git diff --stat 460aef5 C -- crates/oradmp-core/src rust-toolchain.toml` prints nothing,
   and `git diff 460aef5 C -- Cargo.toml` shows only the version line.
4. **Network checks.** In a clone of C, on a machine with network: `cargo deny check advisories`, `cargo deny check
   sources`, `cargo deny check bans`; each "ok" into the record. An advisory against a crate in the wheel stops here.
5. **Version.** In the clone of C: `cargo metadata --no-deps --format-version 1 | grep -o '"version":"[^"]*"' | sort
   -u` prints only `"version":"0.1.0-alpha.3"`; `grep -nE '0\.1\.0a[0-9]+' README.md` shows `0.1.0a3` only.
6. **Build.** From the repository root: `ORADMP_BUILD_ROOT=/var/tmp/oradmp-a3 tools/release_build.sh C --twice 2>&1 |
   tee /var/tmp/oradmp-a3-build.log`. Expected: `commit:` = C; `check_dist: ok` for both builds; the `--twice` part
   says the wheel and the sdist are identical; `cat /var/tmp/oradmp-a3/dist/SHA256SUMS` lists W and S. These two files
   are the release; nothing is rebuilt later. Their hashes differ from the closure review's (another build root; the
   SBOM holds it). Leave a2's `/var/tmp/oradmp-release` as it is.
7. **Install test** (Rocky Linux 8 container as for a2, the build clone `R/src` mounted read-only; or on the VM):
   - Python 3.11: fresh venv; `pip install pyarrow==15.0.2 pandas==2.2.3 pytest`; `pip install --no-deps R/dist/W`;
     `cd R/src && python -m pytest -rs -p no:cacheprovider` -> 0 failed; skipped: exactly the three probe tests of
     `tests/test_internal.py`.
   - The newest CPython with the newest pyarrow and pandas: the same.
   - One venv with pyarrow only (no pandas): the same, plus `test_consumers.py` skipped (pandas); there `oradmp export
     X/t_text_exp_18c.dmp T_TEXT -o X/t.csv` and `-o X/t.parquet` exit 0 (format from the suffix).
   - From each venv's `bin/`: `oradmp --version` prints `oradmp 0.1.0a3`; `oradmp tables X/t_text_exp_18c.dmp` lists
     `T_TEXT`; `oradmp tables X/missing.dmp; echo $?` prints `oradmp: error: [E-IO] No such file or directory:
     X/missing.dmp` and `3`.
   - (Copied folders instead of the clone: also `test_project_name.py` "not a git clone" is skipped; nothing fails.)
   - Source build once: fresh venv, `pip install R/dist/S` (Rust and network needed), `python -m oradmp survey
     --no-names X/t_text_exp_18c.dmp`.
8. **RHEL 8 host** (open since a1, Q-4; not a gate, as for a2): if a real RHEL / Oracle Linux 8 host is at hand,
   install W in a venv there and run `oradmp tables` and `python -m oradmp survey --no-names` on a copied fixture.
   Otherwise record "not done".
9. **Tag, local only.** `git tag -a v0.1.0a3 -m "oradmp 0.1.0a3" C`; `git rev-parse 'v0.1.0a3^{commit}'` equals the
   `commit:` line of `/var/tmp/oradmp-a3-build.log`. **Do not push yet** (10 (g)).
10. **Publish.**
    (a) `cd /var/tmp/a3-check`; `ls -A /var/tmp/oradmp-a3/dist` lists exactly W, S and `SHA256SUMS`; `(cd
    /var/tmp/oradmp-a3/dist && sha256sum -c SHA256SUMS)` prints two `OK`.
    (b) **TestPyPI**: `read -rs UV_PUBLISH_TOKEN` (paste the TestPyPI token scoped to `oradmp` from the password
    manager; nothing is echoed), `export UV_PUBLISH_TOKEN`, then
    `uv publish --publish-url https://test.pypi.org/legacy/ --check-url https://test.pypi.org/simple/ /var/tmp/oradmp-a3/dist/oradmp-0.1.0a3-cp311-abi3-manylinux_2_28_x86_64.whl /var/tmp/oradmp-a3/dist/oradmp-0.1.0a3.tar.gz`,
    then `unset UV_PUBLISH_TOKEN`. No `--dry-run` (it asks for the token too), no glob.
    (c) **Compare**: `curl -s https://test.pypi.org/simple/oradmp/ | grep -o 'oradmp-0\.1\.0a3[^"#]*#sha256=[0-9a-f]*'`
    prints two lines whose hashes equal `SHA256SUMS`. A missing line right after the upload is index lag: retry for a
    few minutes before calling it a mismatch.
    (d) **Install from TestPyPI**: `python3.11 -m venv vt && . vt/bin/activate`; `pip install "pyarrow>=15"` (from
    PyPI); `pip download --no-deps --only-binary :all: --index-url https://test.pypi.org/simple/ "oradmp==0.1.0a3" -d
    dl && sha256sum dl/*` equals W's hash; `pip install --no-deps --index-url https://test.pypi.org/simple/
    "oradmp==0.1.0a3"` (`--index-url`, not `--extra-index-url`); `oradmp --version`; `oradmp tables
    X/t_text_exp_18c.dmp`; `deactivate`. Any difference stops here, before PyPI. (A TestPyPI file name, once used, is
    refused there for good: if a fix is needed after (b), the Architect decides the next version.)
    (e) **PyPI**: as (b) with the PyPI token scoped to `oradmp`, no `--publish-url`, and `--check-url
    https://pypi.org/simple/`; `unset UV_PUBLISH_TOKEN`.
    (f) As (c) and (d) against `https://pypi.org/simple/` (in a new venv `vp`: `pip install "pyarrow>=15"`, then `pip
    install --no-deps "oradmp==0.1.0a3"`). Pre-release behaviour, in two more fresh venvs: `pip install --pre oradmp`
    must install 0.1.0a3; `pip install oradmp` (no version, no `--pre`): record what pip does and its version
    (`pip --version`). pip's documentation says it finds only stable versions by default and does not state the
    outcome when only pre-releases exist; the README tells users `oradmp==0.1.0a3` or `--pre`.
    (g) **Push the tag**: `git push origin v0.1.0a3` and `git push github v0.1.0a3`; `git ls-remote --tags origin
    v0.1.0a3` and the same for `github` show it. (`master` with C is pushed to both remotes if it is not yet.)
    (h) `ls -A /var/tmp/oradmp-a3/dist` again lists exactly W, S and `SHA256SUMS`; anything else is removed, and a
    file that can hold a token means that token is revoked.
11. **Record**: give the Architect (next session): C; the two SHA-256 of step 6; the counts of the closure report;
    steps 4, 7, 8 results; the TestPyPI and PyPI comparison lines; the `pip --pre` and plain `pip install oradmp`
    observations with the pip version; that the tag is pushed to both remotes. The Architect writes the a3 record.

**Session 19 (ADR-76; replaces "When" and "Content" below):**
- When: after T-18b (with the version commit) and **Reviewer session 6a** with no blocking finding. **Without
  T-16**: Linux x86_64 `manylinux_2_28` only, as a2; the owner confirms "Linux only" at step 1. No Project-URL
  (O-19 (a)). T-16's wheels and a Project-URL (if the repository is public) come in the release after Reviewer 6b.
- Content: T-13j (LOB-length branch removed; README minimum versions, code marks, absolute licence links; the stricter
  tests), T-18 and T-18b (the `oradmp` command).
- Steps: the checklist with the session 18 and 19 corrections; 3b as the session 19 lines say; the candidate is the
  version commit of T-18b (g), which Reviewer 6a builds with `--twice` (the owner's step 6 build gives other hashes
  in another build root; compare uploads only with the owner's own `SHA256SUMS`).
- **README for a3** (what changes, ADR-75, ADR-76): (1) every `0.1.0a2` (status line, the PyPI line, the wheel and
  sdist file names) becomes `0.1.0a3` (T-18b (g)); (2) the `head` sentence and the stopped-scan exit rule (T-18b (e));
  (3) the Install section says that the package installs the command `oradmp` (one line, T-18b (e)); (4) "Wheels:
  Linux x86_64 only" and "Other platforms" stay (no T-16); (5) no source link (no public URL); (6) the "Command line"
  section otherwise as T-18 wrote it (the DATE-as-milliseconds sentence stays, ADR-75 item 3). **Rechecked by the
  tests that guard the README**: `tests/test_version.py` (every `oradmp==<v>` / `oradmp-<v>` equals the Cargo
  version), `tests/test_project_name.py` (the old name only in the one line after the title; no relative link
  target), `tests/check_dist.py` on the built archives (METADATA / PKG-INFO description = the README; old-name scan),
  all on the candidate in a git clone (step 1). No test reads the prose of "Command line": Reviewer 6a compares it
  with arch 3.5 and ADR-72 as amended.

(Session 18 text, history:)
- When: after T-13j, T-16, T-18 and **Reviewer session 6** (the group's review) with no blocking finding. No date;
  no obligation to release earlier. Exception: a defect of 0.1.0a2 that harms a user makes a3 a fix release from
  `master` after T-13j and a short closure review. If T-16 is still blocked by Q-11 when T-18 is reviewed, the
  owner decides whether a3 goes out Linux-only.
- Content: T-13j (LOB-length branch removed; README minimum versions, code marks, absolute licence links; Project-URL
  if O-19 (a) named one; the stricter tests), T-16's accepted platform wheels, T-18's `oradmp` command.
- Steps: the checklist with the session 18 corrections; **3b due** (T-13j changes `crates/oradmp-core/src/`); the
  version commit `0.1.0-alpha.3` (root `Cargo.toml`, two `Cargo.lock` entries, the `oradmp-core` entry of
  `fuzz/Cargo.lock`, the README's version lines; T-13j (h)'s test fails until all agree) is a small Implementer commit
  after T-18 and **before** Reviewer session 6, so that the Reviewer builds the real candidate (as T-13i (l) did for
  run B).

**Can wait until after the alpha**: ~~`interval_as="string"`~~ (session 11: in the alpha), `ReadOptions.partitions` (ADR-21), ~~the R-038 answer~~ (session 8:
in the alpha through T-6M; a further set only by a fixture of that set), the R-040 answer for conventional LONG values of 32 KB
or more.

### Release 0.1.0a4 (session 21; ADR-84; restated in session 22, ADR-88) - DONE (2026-10-31, C = `4b9883c`, tag `v0.1.0a4`; record below, ADR-111)

#### Release 0.1.0a4 record (session 29; the owner's facts and evidence file, checked against the repository where it can be)

"E" = shown by the owner's evidence file (captured 2026-10-31 11:55:33 UTC) or checked in session 29; "S" = the owner's
statement. Steps are those of the session 28 checklist below. C = `4b9883c52ebf709ccf0161883505844fdcc45345`.

| step | result |
|---|---|
| 1 Gate | `git fetch`; `master` level with `origin/master`; `git diff --stat C origin/master -- . ':!docs'` empty (S; re-checked in session 29 on the local refs: `HEAD` = `origin/master` = `9cb6e84`, diff empty, E). `master` was two docs commits ahead of C (`075a6bf`, session 28's). The `pytest-pynewest-latest` info-line check: not recorded. |
| 2 Old root | moved to `/var/tmp/oradmp-release-20261027` (the checklist's name was `...-old-2026-10-27`; S). |
| 3 Version | Cargo `0.1.0-alpha.4`; `pyproject.toml` `dynamic = ["version"]` (S). README grep not recorded; `tests/test_version.py` passed in every install run. |
| 4 Pins | the five `uses:` pins resolve to their tags: `actions/cache` v6.1.0, `actions/checkout` v7.0.1, `actions/setup-python` v7.1.0, `actions/upload-artifact` v7.0.2, `taiki-e/install-action` v2.87.31 (S). |
| 5 Build | `tools/release_build.sh C --twice --release` at `/var/tmp/oradmp-release`: exit 0, `check_dist: ok` twice, `reproducible`; rustc 1.99.0 (b940084d7 2026-09-28), maturin 1.15.0, zig 0.17.0, epoch 1793315613 = `%ct` of C (S; the epoch E). |
| 6 Hashes (gate) | **W** `oradmp-0.1.0a4-cp311-abi3-manylinux_2_28_x86_64.whl` = `2e4c69a57baafe84e6c5ec0b4b9bef1248c93ea2eabdb57adb17d49d58949dbd` (3,637,834 bytes); **S** `oradmp-0.1.0a4.tar.gz` = `510f2e39f8ae7eb9660fa4f2313062d2245c3c6d16ebd10740fa0e529c40c0fd` (174,352 bytes); `dist/` and `dist2/` give the same two lines (E) = ADR-110 item 2's gates = C's CI push run; S also = Reviewer 6c's sdist. `SHA256SUMS` 206 bytes, sha256 `936bcd1434ba0d63b5db35d41d7a10a63c0bcafc4f9935ea6b48a35baab716e1` (E: rebuilt from the two lines). Sizes: PyPI's (E). |
| 7 `cargo deny` | advisories, bans, licenses, sources ok; warnings only: duplicate `syn` (2.0.119, 3.0.6), unused allowances `ISC` (`deny.toml:20`) and `Zlib` (`:22`) (S). Ruled expected (ADR-112 items 2, 3). |
| 8 VM installs | normal user, fresh `uv` venvs against W, the owner's `/var/tmp/a4-install-test.sh` (which also ran `sha256sum -c SHA256SUMS`): Python 3.11.16 + pyarrow 15.0.2 + pandas 2.2.3 **253 passed, 3 skipped**; Python 3.14 + newest **253 + 3**; Python 3.11 + pyarrow only **246 + 4**; skips = the three probe tests (+ `test_consumers.py` without pandas); `oradmp --version` = `oradmp 0.1.0a4` in all three (S; the three last pytest lines E, unlabelled, in the order 253, 246, 253). `tools/stream_memory.py` **PASS**: iterator RSS growth 0 kB, stream 20 kB, limit 10,240 kB (S; `benchmarks.md` 4.3). Not recorded: `oradmp tables`, the exit-3 case, **the sdist source build** (now T-21 (k)). |
| 9 Rocky Linux 8 (gate) | Rocky Linux 8.9, glibc 2.28, **as root**, from a **`git archive` of C** (not a clone), `uv`, system Python 3.11.13 and a downloaded CPython 3.14.10: **249 + 7** (3.11, pyarrow 15.0.2, pandas 2.2.3), **249 + 7** (3.14), **242 + 8** (pyarrow only) (S). Skips: the three probe tests; the three root-only (`test_cli.py:594`, `:923`, `test_paths.py:121`); `test_project_name.py:34` "not a git clone" (the export; passes in a clone); `test_consumers.py` in the third. = the checklist's counts with its "one more skip" case. pyarrow for 3.14 on Rocky 8: no problem. `export` to CSV / Parquet: not recorded. |
| 10 RHEL 8 host | **not done**, not a gate (Q-4 time). |
| 11 Tag | annotated, tag object `582045a129b24e51c3b585083e463290da1a1039` on C (E, local `git rev-parse` in session 29); tagger time 2026-10-30 22:39:36 UTC; message = ADR-110 item 7 as one paragraph (no separate first line; accepted, ADR-111 item 3). |
| 12 TestPyPI | `twine check` PASSED for both files (S); W and S uploaded (tool not named); clean venv, pyarrow and pandas from PyPI, `oradmp==0.1.0a4 --no-deps` from TestPyPI: `oradmp 0.1.0a4` (E), `_core.abi3.so` `d0efa2df...` (S). The `simple/` index hash lines: not recorded. |
| 13 PyPI | W and S uploaded 2026-10-31 11:43:34Z / 11:43:36Z (PyPI JSON, fetched in session 29); PyPI's JSON lists the hashes and sizes of step 6 (E); `requires_python` `>=3.11`, `project_urls` Source and Issues, licence expression `MIT OR Apache-2.0` (PyPI JSON, session 29; the evidence file does not hold them). Clean venv `uv pip install oradmp==0.1.0a4`: pulled pyarrow 26.0.0; `oradmp 0.1.0a4`; `_core.abi3.so` sha256 `d0efa2dff42b57f5203429ace3b7789d4d1cfbbb7fd09ce986b5badf51b4789c` (E). Plain `pip install oradmp` with `pip --version`: not done; dropped from later checklists (ADR-111 item 4). |
| 14 Tag pushed | to `origin` only, about 11:48 UTC, after the PyPI upload; `git ls-remote`: `582045a1... refs/tags/v0.1.0a4`, `4b9883c5... refs/tags/v0.1.0a4^{}` (E). |
| 15 Release page | `https://github.com/granddad-wolf/oradmp/releases/tag/v0.1.0a4`: pre-release, no files attached; body = the SHA-256 lines and the PyPI link (S). The tag message in the body: not shown, not claimed. |
| 16 Afterwards | both project-scoped tokens (TestPyPI, PyPI) deleted; the GitHub CLI token revoked; tokens typed with `read -rs` in a plain shell, no agent session open (S). The three count checks and the folder steps: in the clean-up below. Session 30: during the clean-up the owner found an older shell-history line (an earlier session) with a token typed on the command line; all three tokens were already revoked, the lines removed; no value recorded (ADR-114 item 6). |
| 17 Record | this table (Architect, session 29). |

What remains of a4: the RHEL 8 host test (not a gate). Session 30: the sdist source build is done (T-21 (k), risk 32
closed); the clean-up is done (owner, session 30 facts).

#### Clean-up after a4 (session 29; ADR-113 item 1; for the owner, in order, no agent session needed)

Each deletion has its reason in ADR-113 item 1. Stop at the first command whose output differs from the comment.

```
# 0. look first (nothing changes)
du -sh /var/tmp/oradmp-release /var/tmp/oradmp-release-* /var/tmp/a4-* /var/tmp/oradmp-a4-build.log 2>/dev/null

# 1. a4: versioned name; evidence next to the shipped files
mv /var/tmp/oradmp-release /var/tmp/oradmp-release-0.1.0a4
test ! -e /var/tmp/oradmp-release && echo "canonical root free"            # canonical root free
(cd /var/tmp/oradmp-release-0.1.0a4/dist && sha256sum -c SHA256SUMS)         # two lines OK
mkdir /var/tmp/oradmp-release-0.1.0a4/evidence
mv /var/tmp/a4-install-test.sh /var/tmp/a4-rocky-test.sh /var/tmp/oradmp-release-0.1.0a4/evidence/
test -f /var/tmp/oradmp-a4-build.log && mv /var/tmp/oradmp-a4-build.log /var/tmp/oradmp-release-0.1.0a4/evidence/
find /var/tmp/a4-venvs /var/tmp/a4-twine /var/tmp/a4-testpypi /var/tmp/a4-pypi /var/tmp/a4-src -maxdepth 2 -name '*.log' -print
#    move every file the find prints into /var/tmp/oradmp-release-0.1.0a4/evidence/ before step 4

# 2. a2 and a3: keep dist/ only, after checking it holds the recorded release hashes (each grep prints 2)
grep -c -e 255cb122dcca3979e171984f89a719a293770709baf09a47936335f80b9b0f1c -e cae95420b83d29866f8b939d8427cd857902a0055abe466baa66e9753c88702d /var/tmp/oradmp-release-a2/dist/SHA256SUMS
grep -c -e 15db873fd7b3b8700b0a1aa9ef65094cf89eae4a0b31f575e3f2c3a4dce2aa16 -e 36bc571d75d6b6bb66968e31ee4144ba46324aa9efa174888ea973ed6e9293e5 /var/tmp/oradmp-release-a3/dist/SHA256SUMS
(cd /var/tmp/oradmp-release-a2/dist && sha256sum -c SHA256SUMS); (cd /var/tmp/oradmp-release-a3/dist && sha256sum -c SHA256SUMS)
find /var/tmp/oradmp-release-a2 /var/tmp/oradmp-release-a3 -mindepth 1 -maxdepth 1 ! -name dist -exec rm -rf -- {} +

# 3. the old canonical root: print for the record (paste with the next facts), then delete
git -C /var/tmp/oradmp-release-20261027/src log -1 --format='%H %ci'; cat /var/tmp/oradmp-release-20261027/dist/SHA256SUMS
rm -rf -- /var/tmp/oradmp-release-20261027

# 4. comparison roots, the export, the venvs, the tag message file, leftovers
rm -rf -- /var/tmp/oradmp-release-af6a54a /var/tmp/oradmp-release-ci /var/tmp/oradmp-release-ci2 /var/tmp/oradmp-release-d081109
rm -rf -- /var/tmp/a4-venvs /var/tmp/a4-twine /var/tmp/a4-testpypi /var/tmp/a4-pypi /var/tmp/a4-tag-message.txt
rm -rf -- /var/tmp/a4-src          # if "Permission denied" (made as root in the container): sudo rm -rf -- /var/tmp/a4-src
rm -rf -- /var/tmp/a4-check /var/tmp/oradmp-review-6c                 # if present
ls -d /var/tmp/oradmp-release* /var/tmp/a4-* 2>/dev/null               # oradmp-release-0.1.0a4, -a2, -a3 only

# 5. token checks of checklist step 16 (counts only, never a value)
ls -a ~ | grep -c pypirc                     # 0
grep -c 'pypi-' ~/.bash_history              # 0
env | grep -c UV_PUBLISH_TOKEN               # 0
gh auth status                               # not logged in; if it shows an account: gh auth logout
```

Then, on GitHub: archive `granddad-wolf/oradmp-private` (its Settings, "Archive this repository"; ADR-113 item 2), and
the "About" settings of ADR-113 item 6. Paste to the next Architect session: the output of step 3 and anything that
differed from a comment.


- **Content**: T-18d (the SF-1 fix; file names that are not UTF-8 in the API, the tool and `survey`; the `-n -1`
  subprocess case; README); T-17's tests (no reader change expected; a reader fix found by T-17 is its own task and
  record, with step 3b due if `crates/oradmp-core/src/` changes); T-16's accepted platform wheels (ADR-53 item 4; none
  accepted: Linux x86_64 only, as a3; an accepted `manylinux2014` wheel replaces `manylinux_2_28`); the Project-URL
  only if the repository is public before T-16 ends (T-16 (j)); version `0.1.0a4`.
- **Before it** (rule 7 of `CLAUDE.md`): T-18d, T-17, T-16 committed; the **a4 version commit** (Implementer, small:
  root `Cargo.toml`, both `Cargo.lock` entries, the `oradmp-core` entry of `fuzz/Cargo.lock`, the README's version
  lines `0.1.0a4`; `tests/test_version.py` green), the last commit outside `docs/`; **Reviewer 6b** on the group and
  the candidate, no blocking finding; a closure review if 6b's fixes change a shipped file; an **Architect session**
  that records 6b and writes the a4 checklist.
- **Steps**: the a3 checklist (session 20) with `<v>` = `0.1.0a4` and the session 21 block of the general checklist;
  if T-16 adds platform wheels, the a4 checklist adds where each comes from (the CI run on the tagged commit; its log's
  SHA-256 compared with the downloaded file before TestPyPI). No date.
- **Not in a4**: the API `OSError.filename` (ADR-82 item 6); conventional LONG values of 32 KB or more (R-040); Data
  Pump.
- **Session 22 (ADR-88; this block wins where the text above differs)**:
  - **Content**: T-18d; T-17 and T-17b (tests; SF-4 changes `python/oradmp/_cli.py`); T-16a (CI files); the Linux
    x86_64 `manylinux_2_28` wheel and the sdist from the VM's `tools/release_build.sh`, as a3; version `0.1.0a4`.
    `[project.urls]` with its `check_dist.py` rule and case only if the cut (Q-14) is done before the version commit (the
    version task then adds them, as old T-16 (j)). **Not in a4**: T-16b's platform wheels (a5); the items listed above.
  - **Order before it**: T-17b committed; T-16a done (one green run on its final commit, the extended job once,
    findings 0); optionally the owner's cut (Q-14), never later than this point; the **a4 version commit** (as above),
    pushed to GitHub so that T-16a's jobs run on the exact candidate (their summary is part of 6c's input); **Reviewer
    6c** on T-17b, T-16a and the candidate (ADR-88 item 3), no blocking finding; a closure review if 6c's fixes change a
    shipped file; an Architect session that records 6c and writes the a4 checklist.
  - **Steps**: as above, with Reviewer 6c in place of 6b; the CI run on the tagged commit is recorded (its sdist hash
    should equal the owner's; the shipped files are still the VM's).

**Reviewer 6c, "Done when"** (Opus; one build at a time; no benchmarks unless `crates/oradmp-core/src/` changed since
`460aef5`):
1. **Diff**: `git diff --stat edc62a5 C -- . ':!docs'` (C = the a4 candidate) lists only the files of T-17b (l), T-16a
   (i) and the version commit; each read against its "Done when" and ADR-86 to ADR-88. `git diff --stat 460aef5 C --
   crates/oradmp-core/src` is empty (else step 3b is due).
2. **T-17b**: every mutation and check of T-17b (a) to (j) repeated alone; each fails its named test or check, all
   others pass; `git status` clean after each. `header_sets_match_the_fixtures`: the Reviewer's own header listing (as
   in 6b) gives the same set. Every `UNCONFIRMED(` site read against ADR-80's amendment.
3. **T-16a**: the workflow read against ADR-79 item 4, ADR-83 items 2 and 3, ADR-87 items 4 to 6 (no repository name,
   pins by SHA, `permissions`, no upload, no secret, no `corpus/work/`); the pasted summary of the run on C: its counts
   equal the Reviewer's clean-clone counts; the sdist SHA-256 of the CI's `release_build.sh` equals the Reviewer's.
4. **Candidate**: build twice, checklist step 3 in a clean clone of C, the installed wheel in three venvs and both
   layouts, the CLI from the wheel (as 6a-closure items 3 to 6, with `0.1.0a4`); the non-UTF-8 cases of T-18d and
   SF-4 by hand once (dump name, `export -o` target, read-only folder).
5. **If the cut happened** (Q-14): checks 6 to 9 of Q-14 on the public tree.
6. **Verdict** for the a4 upload of C.
7. **(Session 23, ADR-93 item 3) CI and T-16a2**: the CI checks of ADR-93 item 3, one by one, each with the command or
   line that shows it; T-16a2's "Done when" (a) to (g), its mutation of (c) repeated; `git diff --stat edc62a5 C -- .
   ':!docs'` now also lists T-16a2's files. Item 1's list grows accordingly.

**Session 23 (ADR-93; this block wins where the text above differs):**
- **Content** unchanged (ADR-88 item 1), plus T-16a2 (release script, `check_dist.py`, CI files; no product change).
  Linux x86_64 `manylinux_2_28` and the sdist only (ADR-91). `[project.urls]` only if the cut is done first.
- **Order**: (1) T-16a2 committed, its push run green; O-23 (a) recorded. (2) [OWNER] the cut (Q-14, "Q-14 steps
  (session 23)" in `open-questions.md`), recommended now; the first CI run on the public repository green before
  anything else happens there; Q-14 checks; a short Architect session closes Q-14. (3) **a4 version commit** (Implementer,
  small: `0.1.0-alpha.4` in the root `Cargo.toml`, both `Cargo.lock` entries, the `oradmp-core` entry of
  `fuzz/Cargo.lock`, the README's version lines; if (2) is done: `[project.urls] Source = "<public address>"`, the
  `check_dist.py` rule and its case; `tests/test_version.py` green), pushed; its CI run green; the owner pastes the
  summaries (ADR-92 item 3). (4) **Reviewer 6c** (items 1 to 7). (5) Architect session: record 6c; the a4 checklist
  with names. (6) The release, in this order: gate; checks (6c's run on C); network `cargo deny`; version check; old
  canonical root moved aside; **build twice** (`--twice --release`); **hash cross-check** with the CI summary of C
  (ADR-90 item 5); **Rocky Linux 8 install test** (three venvs, as root; skips 6 / 7 as the session 21 block); RHEL 8
  host if at hand; tag made locally with the full hash; **TestPyPI** upload, index-hash comparison, install; **PyPI**
  upload, index-hash comparison, installs; the **tag pushed last** (public repository and Gogs, or `github` and `origin`
  without the cut); `dist/` checked; the record.
- Without the cut first: the same order without (2) and without `[project.urls]`; the cut comes after the a4 tag.

**Session 24 (ADR-98; this block wins where the text above differs):**
- **Order**: (0) **T-19** committed, its push run green (master's corpus tests are red until then, ADR-94 item 5); then
  (2) to (6) above unchanged. The cut never carries the reader of `8162eb0`.
- **Content**: plus T-19 (a reader change: the conventional LONG forms); "Not in a4: R-040" is withdrawn; R-048's result
  (T-19b) only if done before the version commit.
- **Reviewer 6c** covers T-17b, T-16a, T-16a2, T-19 and the version commit (and the cut if first). Item 1's "`git diff
  --stat 460aef5 C -- crates/oradmp-core/src` is empty" reads "lists only T-19's files". New **item 8**: T-19's mutations
  M1 to M12 and checks (a) to (n) repeated alone; the golden and expected files regenerated by their generators and
  compared with the committed ones; ADR-95's table read against `exp/rows.rs`; **step 3b**: T-19's benchmark and memory
  figures repeated under `benchmarks.md` section 5.
- **Release-notes sentence on LONG** (tag message, the a4 record, the README's "What it reads"): "A LONG value on the
  conventional path no longer stops the file at 32,768 bytes: it is read in the two forms `exp` writes (one field up to
  65,520 bytes, 4,000-byte chunks from 65,535 bytes; confirmed on 11g and 18c up to 100,000 bytes). A LONG of 65,521 to
  65,534 bytes whose form is not yet known, and a conventional LONG RAW value of 32,768 bytes or more, still stop the file
  with `E-UNSUPPORTED-LAYOUT`."

**Session 25 (ADR-104; this block wins where the text above differs):**
- **Order**: (0) T-19 done; **T-19b** with the owner's corpus commit, one push, run green; (0') **T-19 (m)** (step 3b and
  the fuzz runs, alone, on T-19b's commit); then the cut, whose **first step** is renaming the private GitHub repository
  and repointing every remote of the private history at the renamed address (`git remote -v` checked) before the public
  `oradmp` repository is created (ADR-104 item 3; in the owner's assistant's Q-14 checklist); then (2) to (6) of the
  session 23 block.
- **Content**: plus T-19b (a reader change: the three forms for LONG and LONG RAW, the TSLTZ warning, the buffer
  minimum, `LongStats.sizes_65521_to_65534`).
- **Reviewer 6c** covers T-17b, T-16a, T-16a2, T-19, **T-19b**, the **T-19 (m)** figures and the version commit (and the
  cut). Item 1's core diff lists T-19's and T-19b's files only. Item 8 adds: T-19b's mutations M1 to M12 and checks (a)
  to (p) repeated alone; ADR-99's table read against `exp/rows.rs`; the golden and expected files regenerated and
  compared; step 3b repeated on C against T-19 (m)'s figures.
- **Release-notes sentence on LONG** (replaces the session 24 one): "LONG and LONG RAW values on the conventional path are
  read in the three forms `exp` writes (one field up to 65,529 bytes, a 4-byte length for 65,530 to 65,533 bytes, 4,000-byte
  chunks from 65,534 bytes; confirmed on 11g and 18c up to 100,000 bytes), and a value of 65,522 to 65,524 bytes, a size
  no test file has, is read with a `W-UNTESTED-LAYOUT` warning."

**Session 26 (ADR-105 to ADR-107; this block wins where the text above differs):**
- **Done**: T-19b (`1bb7785`), T-19 (m) (`0520cd5`), the cut (P `b55eb10`, S `9950ef2`). C (the candidate) will be T-20's
  commit in the public repository. **Release rule**: from a4 on, every release is built from a commit of the public
  repository with `tools/release_build.sh C --twice --release` at `/var/tmp/oradmp-release`; the tag goes to `origin`
  only; the Gogs archive gets nothing new.
- **Content**: unchanged (T-17b, T-16a, T-16a2, T-18d, T-19, T-19b) plus T-20 (version, `[project.urls]`, README and
  CONTRIBUTING lines, `fuzz-check`). Linux x86_64 `manylinux_2_28` wheel and the sdist.
- **Reviewer 6c** (Opus; writes `docs/review/review-06c-...md` only). **Where it reads**: its own scratch clone of the
  public repository below `/var/tmp` (e.g. `/var/tmp/oradmp-review-6c/`); into that clone it fetches the archive
  (`git fetch ~/p/oradmp-archive +refs/heads/master:refs/remotes/archive/master` and the three `v0.1.0a*` tags), which
  reads `~/p/oradmp-archive` without changing it; the owner's clone is not touched. Tree diffs across the cut then work
  (`git diff <archive commit> C`). **What it builds where**: everything in its scratch folder, one build at a time; S's
  tree is P's tree plus `HISTORY.md`, so a build of C stands for the archive's code with T-20 on top; step 3b alone on the
  machine, Oracle containers stopped, `ORADMP_WORK` = the parent folder of `18c/t_big_exp_18c.dmp` (as T-19 (m) found). Items (replacing item 1's bases; items 2 to 8 as
  before):
  1. **Diff**: `git diff --stat edc62a5 C -- . ':!docs'` lists only the files of T-17b, T-16a, T-16a2, T-19, T-19b, the
     T-19 (m) fix (`fuzz/fuzz_targets/values.rs`), the scrub commits (`CONTRIBUTING.md`, `corpus/scripts/`,
     `corpus/sql/00_setup.sql`, `.gitignore`, the removed private paths), `HISTORY.md` and T-20; `git diff --stat 460aef5 C
     -- crates/oradmp-core/src` lists T-19's and T-19b's files only; `git diff --stat 1bb7785 C -- crates python tests`
     lists T-20's test files only.
  2. to 8. as the session 22 to 25 blocks (T-17b, T-16a / T-16a2 and the CI checks of ADR-93 item 3 now on `ci.yml` at
     C, with the new `fuzz-check` job; the candidate built twice and installed; T-19 and T-19b mutations and checks;
     golden files regenerated and compared; ADR-99's table against `rows.rs`; **step 3b** on C against baseline 2 and
     T-19 (m)'s figures, with ADR-106 item 2's same-sitting `460aef5` run of the `T_BIG` scan if the watch shows again).
  9. **T-20**: (a) to (i) read and repeated (version grep, the `Project-URL` cases and mutation, the `fuzz-check`
     mutation, the README and CONTRIBUTING lines read as a stranger would: every command runnable from the public tree,
     no step that needs a private file).
  10. **The cut**: (a) scrub residue on C, counts only, never a value printed: `git grep -c` for the Reviewer's `$HOME`,
     `$(hostname)`, the archive's author e-mail (`git -C ~/p/oradmp-archive log -1 --format=%ae 0520cd5`), the token forms
     of Q-14 check 2, and `IDENTIFIED BY` (expected: the two throwaway users only, ADR-105 item 2); `git log -p` of the
     public history for the same; (b) `HISTORY.md` against the archive: P, the a1 / a2 / a3 hashes and tags, the scrub
     list, the three private paths absent from C; (c) the settings of ADR-105 item 8 as far as the tree shows them (every
     `uses:` is GitHub's or the `taiki-e` pin of the allow-list; no workflow asks for write permission); the GitHub
     settings themselves are the owner's statement, recorded, not checked; (d) the public CI: the pasted summaries of S
     and C (push, `extended`, `manylinux2014`) against the Reviewer's clean-clone counts and sdist hash; (e) no public
     document sends a contributor to a private file as a step.
  11. **Verdict** for the a4 upload of C.
- **After 6c**: an Architect session records it and writes the a4 checklist (with the names: the `mv` of the old
  canonical root, the `extended` run on C, the tag to `origin`).

**Session 27 (ADR-108; this block wins where the text above differs):**
- **Done**: T-20 (`e1ed10b`, with `4aa33e6`; its push run and the `extended` and `manylinux2014` dispatches on it).
  **C = the commit of T-20b** on the public `master` (version `0.1.0a4` from T-20, `pipefail` from T-20b). The 6c prompt
  names C by its full hash, taken after the owner's commit, once C's push run and its two dispatches are green and
  pasted (T-20b (e), (g)). `e1ed10b`'s hashes and runs are T-20's evidence, not C's.
- **Reviewer 6c, the exact scope** (Opus; writes `docs/review/review-06c-...md` only): items 1 to 11 of the session 26
  block, on C, with these changes:
  - Item 1 adds `git diff --stat e1ed10b C -- . ':!docs'` = `.github/workflows/ci.yml` only, and `git diff --stat
    9950ef2 C -- . ':!docs'` = T-20's files of T-20 (g) only (T-20b adds no file).
  - Item 9 covers **T-20 and T-20b**: T-20 (a) to (i) as written; T-20b (a) to (g): the diff read line by line, (c)'s
    shell facts and the two rewritten constructs repeated in the scratch clone, the proof run's paste checked against
    the proof commit (the owner keeps its diff or the paste shows it; every listed outcome `failure`), and `ci.yml` at C
    read once more for any step whose failure is still hidden (a piped command, `|| true`, a command substitution that
    an `if`, `case` or assignment swallows) outside the summary steps; a hidden failure left is a blocking finding.
  - Item 10 (d): the public CI pastes are those of S, `e1ed10b` (push and both dispatches), the `t20b-proof` run, and
    C (push, `extended`, `manylinux2014`); the clean-clone counts and the sdist hash are compared with **C's** push run.
  - Item 10 (a) unchanged: `IDENTIFIED BY` expected for the two throwaway users only (ADR-108 item 3, decided).
- **Where 6c reads and builds**: its own scratch clone of the public repository, e.g. `git clone
  https://github.com/granddad-wolf/oradmp /var/tmp/oradmp-review-6c/src`, checked out at C; the archive is fetched into
  that clone from `~/p/oradmp-archive` as in the session 26 block (read-only, the archive and the owner's clone
  `~/p/oradmp` are not changed). All builds below `/var/tmp/oradmp-review-6c/`, one at a time: `cargo` with
  `CARGO_TARGET_DIR` there; its own venv there (`maturin develop --features test-probes` into that venv only); the
  candidate built twice with `ORADMP_BUILD_ROOT=/var/tmp/oradmp-review-6c/release tools/release_build.sh C --twice`
  **without** `--release` (the script refuses any other root with `--release`). **Never** at the canonical root
  `/var/tmp/oradmp-release` (the release's), never in `~/p/oradmp` (rule 9: no `uv run`, no `maturin develop` there).
  Expected: 6c's sdist equals C's CI sdist byte for byte; its wheel differs from the CI wheel by the build root in the
  SBOM only (ADR-90; a member-by-member comparison shows it). Step 3b alone on the machine, Oracle containers stopped,
  `ORADMP_WORK` = the parent folder of `18c/t_big_exp_18c.dmp`; the same-sitting `460aef5` scan run (ADR-106 item 2)
  builds from the archive commit in a second scratch worktree of the same clone.
- **After 6c**: unchanged (Architect session: record 6c, the a4 checklist; then the release from C).

**Session 28 (ADR-109, ADR-110; this block wins where the text above differs):**
- **Done**: Reviewer 6c (`docs/review/review-06c.md`): no blocking finding at C; "0.1.0a4 can be built from C as it
  stands". No closure task (SF-1 and the notes go to T-21 after the tag, or need no action; ADR-109).
- **The release commit is C**, the tag `v0.1.0a4` goes on it, not on a later docs commit (ADR-110 item 1).
- **Content**: unchanged (T-17b, T-16a, T-16a2, T-18d, T-19, T-19b, T-20, T-20b). Linux x86_64 `manylinux_2_28` wheel
  and the sdist. Release notes: ADR-110 item 7.

#### Release 0.1.0a4 checklist (session 28; for the owner, in order)

Names: **C** = `4b9883c52ebf709ccf0161883505844fdcc45345`; **R** = `/var/tmp/oradmp-release` (the canonical root);
**W** = `oradmp-0.1.0a4-cp311-abi3-manylinux_2_28_x86_64.whl`; **S** = `oradmp-0.1.0a4.tar.gz`; **X** =
`/var/tmp/a4-check` (scratch, outside the repository); **L** = `/var/tmp/oradmp-a4-build.log`. Expected: **epoch
1793315613**, **W = `2e4c69a57baafe84e6c5ec0b4b9bef1248c93ea2eabdb57adb17d49d58949dbd`**, **S =
`510f2e39f8ae7eb9660fa4f2313062d2245c3c6d16ebd10740fa0e529c40c0fd`** (C's CI push run; never a later run's).
**[O]** = the owner. Every step is the owner's; no agent session runs from step 11 to step 16. Oracle containers
stopped for steps 6 to 9; one build at a time; `TMPDIR=/var/tmp/oradmp-tmp`, `CARGO_BUILD_JOBS=2`. Any "stop" means:
nothing is uploaded or tagged; paste the output to the Architect.

1. **[O] Gate.** Push the two docs commits first if not yet (`075a6bf` and session 28's; their CI runs are not the
   release's). Then in `~/p/oradmp`: `git fetch origin`; `git rev-parse 'C^{commit}'` prints C;
   `git merge-base --is-ancestor C origin/master && echo on-master` prints `on-master`; `git diff --stat C origin/master
   -- . ':!docs'` prints nothing. In C's pasted `wheel` summary, the `pytest-pynewest-latest` line starts with the
   newest Python setup-python installed (3.14.x), not 3.11 (SF-1's only scenario, ADR-109 item 3). Else stop.
2. **[O] Old root aside** (6c N-6). `ls -ld /var/tmp/oradmp-release` (it exists, made 2026-10-27);
   `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-old-2026-10-27`; now `ls -d /var/tmp/oradmp-release` must fail.
3. **[O] Version.** `git show C:Cargo.toml | grep -m1 '^version'` prints `version = "0.1.0-alpha.4"`;
   `git show C:README.md | grep -oE '0\.1\.0a[0-9]+' | sort -u` prints only `0.1.0a4`.
4. **[O] Action pins** (6c did not check them; network, no token). For each of the five pins of `ci.yml` at C run
   `git ls-remote --tags <url> | grep -F 'refs/tags/<tag>'`:
   `https://github.com/actions/checkout` `v7.0.1` -> `3d3c42e5aac5ba805825da76410c181273ba90b1`;
   `https://github.com/actions/setup-python` `v7.1.0` -> `6d4fd22b20465ccc0f7354476588e8df7c9ca0e0`;
   `https://github.com/actions/cache` `v6.1.0` -> `55cc8345863c7cc4c66a329aec7e433d2d1c52a9`;
   `https://github.com/actions/upload-artifact` `v7.0.2` -> `cf430e030ddbb5b0abf93d22962f4752f3646cd9`;
   `https://github.com/taiki-e/install-action` `v2.87.31` -> `1e29709c0ec63babbe8f266397a465f29a825f91`.
   The pinned hash must equal the `refs/tags/<tag>^{}` line if one is printed (an annotated tag), else the
   `refs/tags/<tag>` line. Any other result: stop (ADR-109 item 5).
5. **[O] Build.** In `~/p/oradmp`:
   `env -u ORADMP_BUILD_ROOT TMPDIR=/var/tmp/oradmp-tmp CARGO_BUILD_JOBS=2 tools/release_build.sh 4b9883c52ebf709ccf0161883505844fdcc45345 --twice --release 2>&1 | tee /var/tmp/oradmp-a4-build.log; echo "status ${PIPESTATUS[0]}"`.
   Expected: `status 0`; in L: `release root: canonical`, `commit:   4b9883c52ebf...` (= C), `epoch:    1793315613
   (SOURCE_DATE_EPOCH)`, `root:     /var/tmp/oradmp-release`, `check_dist: ok` twice, `identical:` for W and for S,
   `reproducible`. Else stop. W and S in `R/dist` are the release; nothing is rebuilt later.
6. **[O] Hashes (gate).** `ls -A /var/tmp/oradmp-release/dist` lists exactly W, S, `SHA256SUMS`;
   `cat /var/tmp/oradmp-release/dist/SHA256SUMS` shows the two expected hashes above, all 64 characters equal. A
   difference stops the release. For a wheel difference, list its members and compare with the member list in C's
   `wheel` summary: `python3 -I -c 'import sys,zipfile,hashlib; z=zipfile.ZipFile(sys.argv[1]); [print(hashlib.sha256(z.read(i)).hexdigest(), i.file_size, i.filename) for i in z.infolist() if not i.is_dir()]' /var/tmp/oradmp-release/dist/W`.
7. **[O] Network check.** `cd /var/tmp/oradmp-release/src && cargo deny check`: advisories, bans, licenses, sources
   each `ok`. An advisory against a crate in the wheel stops the release until the Architect rules.
8. **[O] Install on the VM** (normal user). `mkdir /var/tmp/a4-check && cp ~/p/oradmp/corpus/fixtures/18c/t_text_exp_18c.dmp /var/tmp/a4-check/`.
   - `uv venv --python 3.11 X/v311`; `uv pip install --python X/v311/bin/python /var/tmp/oradmp-release/dist/W
     pytest==9.1.1 pyarrow==15.0.2 pandas==2.2.3`; `X/v311/bin/python -c 'import oradmp; print(oradmp.__version__,
     oradmp.__file__)'` prints `0.1.0a4` and a path in `X/v311/.../site-packages/oradmp/`.
   - `cd /var/tmp/oradmp-release/src && X/v311/bin/pytest -rs -q -p no:cacheprovider`: **253 passed, 3 skipped**, the
     three skips in `tests/test_internal.py` (no probes in a release).
   - `X/v311/bin/python tools/stream_memory.py corpus/fixtures` (still in `R/src`): prints `PASS`, exit 0; keep the
     printed RSS lines for the record.
   - `X/v311/bin/oradmp --version` prints `oradmp 0.1.0a4`; `X/v311/bin/oradmp tables X/t_text_exp_18c.dmp` lists
     `T_TEXT`; `X/v311/bin/oradmp tables X/missing.dmp; echo $?` prints `oradmp: error: [E-IO] No such file or
     directory: ...missing.dmp` and `3`.
   - Source build once: `uv venv --python 3.11 X/vsrc`; `TMPDIR=/var/tmp/oradmp-tmp CARGO_BUILD_JOBS=2 uv pip install
     --python X/vsrc/bin/python /var/tmp/oradmp-release/dist/S` (Rust and network); `X/vsrc/bin/python -m oradmp survey
     --no-names X/t_text_exp_18c.dmp` exits 0.
9. **[O] Rocky Linux 8 container, as root (gate; ADR-109 item 5).**
   `docker run --rm -it -v /var/tmp/oradmp-release:/var/tmp/oradmp-release:ro -v /var/tmp/a4-check:/x:ro rockylinux:8 bash`,
   then inside:
   - `dnf install -y python3.11 python3.11-pip git`; `git config --global --add safe.directory
     /var/tmp/oradmp-release/src`; `cd /var/tmp/oradmp-release/src`; `W=/var/tmp/oradmp-release/dist/W` (the full name).
   - (a) `python3.11 -m venv /v/a && /v/a/bin/pip install "$W" pytest==9.1.1 pyarrow==15.0.2 pandas==2.2.3`;
     `/v/a/bin/pytest -rs -q -p no:cacheprovider`: **250 passed, 6 skipped** = the three probe tests, `tests/test_cli.py:594`
     ("root can write to a read-only folder"), `tests/test_cli.py:923` ("root can read a file of mode 000"),
     `tests/test_paths.py:121` ("root can write to a read-only folder").
   - (b) `python3.11 -m venv /v/b && /v/b/bin/pip install "$W" pytest==9.1.1 pyarrow`; pytest as (a): **243 passed, 7
     skipped** (the six of (a) and `tests/test_consumers.py`, pandas); `/v/b/bin/oradmp export /x/t_text_exp_18c.dmp
     T_TEXT -o /tmp/t.csv` and `-o /tmp/t.parquet` each exit 0.
   - (c) The newest CPython: `/v/a/bin/pip install uv==0.12.21`; `/v/a/bin/uv python install 3.14`;
     `/v/a/bin/uv venv --python 3.14 /v/c`; `/v/a/bin/uv pip install --python /v/c/bin/python "$W" pytest==9.1.1 pyarrow
     pandas`; pytest as (a): **250 passed, 6 skipped**. If no pyarrow wheel exists for that Python on this image, record
     that and go on (6c ran 3.14.7 on the VM).
   - In each venv: `oradmp --version` prints `oradmp 0.1.0a4`; `oradmp tables /x/t_text_exp_18c.dmp` lists `T_TEXT`.
   - If `git` could not be installed, each count has one more skip (`tests/test_project_name.py:34`, "not a git
     clone"). Any failure, or any other skip: stop. `exit` (the container is removed).
10. **[O] RHEL 8 host**: not a gate; no host at hand: record "not done" (ADR-109 item 5).
11. **[O] Tag, local only.** Write the release notes of ADR-110 item 7 into `/var/tmp/a4-tag-message.txt` (first line
    `oradmp 0.1.0a4`, a blank line, then the text). `git tag -a v0.1.0a4 -F /var/tmp/a4-tag-message.txt
    4b9883c52ebf709ccf0161883505844fdcc45345`; `git rev-parse 'v0.1.0a4^{commit}'` equals the `commit:` line of L.
    Not pushed yet.
12. **[O] TestPyPI.** Close every agent session; a new terminal; `cd /var/tmp/a4-check`; `(cd /var/tmp/oradmp-release/dist
    && sha256sum -c SHA256SUMS)` prints two `OK`. On test.pypi.org create an API token scoped to the project `oradmp`
    and copy it from the page. Then:
    `read -rs UV_PUBLISH_TOKEN` (paste, Enter; nothing is echoed, and the history holds only this command line),
    `export UV_PUBLISH_TOKEN`,
    `uv publish --publish-url https://test.pypi.org/legacy/ --check-url https://test.pypi.org/simple/ /var/tmp/oradmp-release/dist/oradmp-0.1.0a4-cp311-abi3-manylinux_2_28_x86_64.whl /var/tmp/oradmp-release/dist/oradmp-0.1.0a4.tar.gz`,
    `unset UV_PUBLISH_TOKEN`. Never the token on a command line (`UV_PUBLISH_TOKEN=... uv publish`, `--token`), in a
    file, or in `~/.pypirc`. No glob, no `--dry-run`.
    Compare: `curl -s https://test.pypi.org/simple/oradmp/ | grep -o 'oradmp-0\.1\.0a4[^"#]*#sha256=[0-9a-f]*'` prints
    two lines with the hashes of step 6 (a missing line right after the upload is index lag: retry for a few minutes).
    Install: `python3 -m venv vt`; `vt/bin/pip install "pyarrow>=15"`; `vt/bin/pip download --no-deps --only-binary
    :all: --index-url https://test.pypi.org/simple/ "oradmp==0.1.0a4" -d dl && sha256sum dl/*` equals W's hash;
    `vt/bin/pip install --no-deps --index-url https://test.pypi.org/simple/ "oradmp==0.1.0a4"`; `vt/bin/oradmp --version`;
    `vt/bin/oradmp tables t_text_exp_18c.dmp`. Any difference: stop before PyPI.
13. **[O] PyPI.** As step 12 with a new token scoped to `oradmp` on pypi.org, no `--publish-url`, `--check-url
    https://pypi.org/simple/`; `unset UV_PUBLISH_TOKEN`. Compare against `https://pypi.org/simple/oradmp/` as in 12. In a
    new venv `vp`: `vp/bin/pip install "oradmp==0.1.0a4"` (dependencies from PyPI), `vp/bin/oradmp --version`,
    `vp/bin/oradmp tables t_text_exp_18c.dmp`. In one more fresh venv: `pip install oradmp` without a version; record
    the version it picks and `pip --version`.
14. **[O] Push the tag.** `cd ~/p/oradmp && git push origin refs/tags/v0.1.0a4`; `git ls-remote --tags origin v0.1.0a4`
    prints the tag object and a `^{}` line whose hash is C. Only `origin` (the public repository); nothing goes to the
    Gogs archive or `oradmp-private` (ADR-105 item 7). No agent pushes.
15. **[O] GitHub release page** (ADR-110 item 4): Releases, "Draft a new release", choose the existing tag
    `v0.1.0a4`, title `oradmp 0.1.0a4`, body = the tag message, the two SHA-256 of step 6 and
    `https://pypi.org/project/oradmp/0.1.0a4/`; tick "Set as a pre-release"; attach no file; publish.
16. **[O] Afterwards.**
    - Tokens: delete both API tokens (TestPyPI and PyPI, account settings, API tokens; ADR-110 item 5); log out of
      both web sites. Check, counts only: `ls -a ~ | grep -c pypirc` prints 0; `grep -c 'pypi-' ~/.bash_history` prints
      0; `env | grep -c UV_PUBLISH_TOKEN` prints 0.
    - `ls -A /var/tmp/oradmp-release/dist` lists exactly W, S, `SHA256SUMS` (anything else is removed; a file that could
      hold a token means that token is revoked, as it is anyway).
    - Folders: `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-0.1.0a4` (keeps the shipped files and the build clone;
      the canonical root is free for the next release); keep L next to it; delete `/var/tmp/a4-check` (with `vt`, `vp`,
      `dl`), `/var/tmp/a4-tag-message.txt`, and `/var/tmp/oradmp-release-old-2026-10-27` once you have looked at what it
      holds (it predates C); `/var/tmp/oradmp-review-6c` if anything is left of it.
    - `oradmp-private`: now the owner's choice (ADR-106 item 4); recommendation: archive it read-only on GitHub
      (Settings, "Archive this repository"), not delete it, because the a3 record cites its runs. The snapshot folder:
      the owner's choice. O-24 stays open (not a gate).
17. **[O] Record** (to the Architect, next session): the report lines of L (commit, epoch, root, `SHA256SUMS`,
    `check_dist`, `identical`); steps 4 and 7 results; step 8 and 9 counts with every skip line; the `stream_memory.py`
    lines; step 10; the TestPyPI and PyPI comparison lines; the plain `pip install oradmp` result with `pip --version`;
    the `git ls-remote` lines of step 14; the release page's URL. The Architect writes "Release 0.1.0a4 record" here.
    `HISTORY.md` is not changed; there is no changelog file (ADR-110 item 6).

## CI minutes budget (session 23; ADR-92 item 4)

**Session 26**: the repository is public; standard hosted runners are free there and this budget lapses (O-23 (b)
closed, ADR-105 item 10). The public runner reports 4 CPUs (benchmarks never compare CI figures, ADR-92 item 2).

| item | figure | source |
|---|---|---|
| included minutes, private repository | 2,000 a month (the owner's understanding; **[OWNER] the figure from Settings > Billing**, O-23 (b)) | owner; GitHub's billing page lists 2,000 for GitHub Free (fetched in session 23) |
| a push (`checks` + `wheel`) | about 7 to 8 runner minutes | `d081109` run |
| a full dispatch (all four jobs) | about 21 runner minutes | `d081109` run |
| Windows / macOS per minute | about 1.7 / 10 times a Linux 2-core minute (price ratio $0.010 / $0.062 against $0.006) | GitHub's billing page, session 23 |
| over the quota without a payment method | usage is blocked, no bill | GitHub's billing page, session 23 |
| public repository | standard hosted runners free | GitHub's billing page, session 23 |

Rules while private: a push runs only `checks` and `wheel`; a dispatch only when a task's note asks for it (T-16a2: none
needed; T-16b: at most three; T-16c: as its note asks, Linux only); the note records the minutes of each dispatched run.
After the cut the budget lapses for standard runners; the dispatch rule for `extended` stays (it is long, not costly).

**Session 31** (ADR-116 item 5, ADR-118 item 4). Dispatches:
- **T-16c**: used 1 of its 3 (`94b6368`, 1141 s); closed.
- **T-16b**: at most three before the Architect is asked. Each dispatch runs `extended`, `manylinux2014` and the new
  `platforms` job; runner minutes are free on the public repository, and the cap saves the owner's time.
- **a5**: one dispatch on the version commit (ADR-120 item 5).

## Release 0.1.0a5 (session 31; ADR-117, ADR-118, ADR-120) - conditions only; the checklist is written when they hold

- **Due**: T-16c was accepted (ADR-113 item 4, amended).
- **Content**:
  - `oradmp-0.1.0a5-cp311-abi3-manylinux_2_28_x86_64.whl` (as a4);
  - `oradmp-0.1.0a5-cp311-abi3-manylinux_2_17_x86_64.manylinux2014_x86_64.whl` (new);
  - the sdist;
  - non-Linux wheels only if Q-15 is answered "yes" before the version commit and T-16b accepted the platform with
    identical twice-built wheels.
- **Conditions**, in order (ADR-120):
  1. T-16b done;
  2. the 2014 pre-check passed (T-16b (i));
  3. Reviewer 7 with no blocking finding open;
  4. Q-15 answered, or a5 declared Linux-only;
  5. the version commit (version, locks, README "Wheels:" with both Linux wheels and ADR-115 item 5's claim,
     "Other platforms", the classifier), with its push run and one dispatch green;
  6. a closure review of the exact candidate C;
  7. the Architect session that writes the checklist.
- **Gates the checklist will hold for the second wheel** (ADR-117 item 3):
  - both wheels built `--twice --release` at the canonical root, one after the other, with the root moved aside between
    them;
  - each wheel equal to its CI job's wheel on C (`wheel`, `manylinux2014`);
  - one sdist, identical across the four builds;
  - `_core.abi3.so` hashes recorded;
  - installs and `stream_memory.py` on both;
  - TestPyPI, then PyPI, three files compared with the index;
  - the selection check: a glibc 2.28+ host gets 2_28, `--platform manylinux2014_x86_64` gets 2014;
  - `SHA256SUMS` with three lines; one upload, nothing added later.
- **Also from earlier records**: ADR-110 as amended; ADR-111 item 6 (lessons of a4); ADR-114 item 6 (the token-history
  check).

## Order of the next tasks (session 31)

Machine rule unchanged (session 16 below). CI runs on the public GitHub repository; it is not a build on the VM.
**OWNER** marks an owner action. Records: ADR-116 to ADR-120.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | **T-21** (`20bbc80`; ADR-114); **T-16c accepted** (`a8da2cc` + `94b6368`, results `0bfa70d`; ADR-116); this session | - | - | - | - | - |
| 1 | **OWNER**: commit and push this session's documents (docs only) | now | rows 6 to 10 | `docs/design/` | - | yes |
| 2 | **T-16b** (plan T-16b, "Session 31" block):<br>- (0) runner images and pyarrow per platform;<br>- the `platforms` job, dispatch only;<br>- the risk check as the first dispatch;<br>- a verdict per platform;<br>- the sdist check in `extended`;<br>- (i) the 2014 pre-check at the canonical root on the VM;<br>- nothing published | after row 1 | rows 6 to 10 (its VM builds alone on the machine) | `ci.yml` (`platforms`, `extended` sdist step), tests a platform proves wrong, gates, the note | Sonnet | **OWNER**: commits, pushes, at most three dispatches, pastes the summaries |
| 3 | **Q-15** (owner): may a release ship a wheel that `release_build.sh` cannot build? Recommendation: yes, built by CI on the tag in two dispatches, byte-identical, downloaded as an artifact; needs `CLAUDE.md` rule 8 amended | any time before row 5 | everything | `CLAUDE.md` (owner), then ADR amendments | - | yes |
| 4 | **Reviewer 7** on T-21, T-16c, T-16b (plan, "Reviewer 7" block; ADR-119) | after row 2's results commit is pushed | rows 6 to 10 | `docs/review/` | Opus | **OWNER** starts it |
| 5 | **a5** ("Release 0.1.0a5 (session 31)"; ADR-120):<br>- an Architect session records Reviewer 7 and Q-15;<br>- the version commit `0.1.0a5`;<br>- the closure review of C;<br>- an Architect session writes the checklist;<br>- the owner's release | after row 4 (and row 3, or Linux-only) | - | as named | - / Sonnet / Opus | **OWNER**: all release steps |
| 6 | **OWNER: Q-4**: the real-dump counts with `oradmp survey` from the a4 wheel, counts only | now | everything | nothing in the repository | - | yes |
| 6' | **R-052** only if Q-4 shows a LONG of 65,522 to 65,524 bytes or a field starting `fb ff` / `fc ff`; a reader change joins a5 if it is ready before the version commit | after row 6 | everything | `corpus/` (owner), then a task | - | yes |
| 7 | **R-046** (low) | any time | everything | `corpus/` (owner) | - | yes |
| 8 | **OWNER: RHEL 8 host test** with the a4 wheel (not a gate) | any time | everything | nothing in the repository | - | yes |
| 9 | **Data Pump** (phase 2): an Architect session plans it, scope with the owner | now | rows 2 to 8 | `docs/design/` | - | scope with the owner |
| 10 | **Analyst 21** (documents only), if not done | any time | everything | `docs/formats/` | - | **OWNER** starts it |

What the owner does, in order:
1. Push these documents.
2. Start the Implementer on T-16b. Push its commit, dispatch once (the risk check), and paste the summaries to the
   Implementer, up to three dispatches in all. Then paste them to the next Architect session.
3. Answer Q-15 whenever ready.
4. Start Reviewer 7 after T-16b's results commit.

Q-4, R-046 and the RHEL 8 host test can happen at any time. Standing owner items: O-24, Q-13, O-20.

## Order of the next tasks (session 30, history)

Machine rule unchanged (session 16 below). CI runs on the public GitHub repository and is not a build on the VM.
**OWNER** marks an owner action. ADR-113 items 3 and 4; ADR-114; ADR-115.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | **T-21** (2026-10-31, `20bbc80`; ADR-114; risk 32 closed); this session | - | - | - | - | - |
| 1 | **OWNER**: commit and push this session's documents (docs only) | now | rows 5 to 9 | `docs/design/` | - | yes |
| 2 | **T-16c** `manylinux2014` acceptance (plan T-16c, "Session 30" block): check (0) first (may end "rejected" with the note alone); then the release-script option, the CI job with the glibc 2.17 and Rocky Linux 8 containers, the default route unchanged; nothing published | after row 1 | rows 5 to 9 (its one VM build alone on the machine) | `tools/release_build.sh`, `tests/test_release_build.py`, `ci.yml` (`manylinux2014` job), the note | Sonnet | **OWNER** commits, pushes, one `manylinux2014` dispatch (at most three), pastes the `wheel` and `manylinux2014` summaries; Rocky 8 on the VM only as fallback |
| 3 | **T-16b** macOS / Windows, plus the sdist build check in `extended` | after row 2 is pushed (both edit `ci.yml`) | rows 5 to 9 | `.github/workflows/`, `tests/check_dist.py`, `target_os` gates, tests a platform proves wrong | Sonnet | **OWNER** pushes, dispatches, pastes |
| 4 | **Reviewer 7** on T-21, T-16c, T-16b (rule 7; also the SBOM / `RECORD` sizes of T-21 (i), ADR-114 item 2) and, if a5 is due by ADR-113 item 4, the a5 candidate after its version commit | after row 3 | - | `docs/review/` | Opus | **OWNER** starts it |
| 4' | **a5** (only if ADR-113 item 4 holds, e.g. T-16c "accepted"): Architect session (record Reviewer 7, the a5 checklist with ADR-111 item 6 and ADR-114 item 6), version commit `0.1.0a5` (README lines, classifier), the owner's release | after row 4 | - | as named | - / Sonnet | **OWNER** all release steps |
| 5 | **OWNER: Q-4**, the real-dump counts with `oradmp survey` from the a4 wheel, on the owner's side, counts only | now | everything | nothing in the repository | - | yes |
| 5' | **R-052** (exists) only if Q-4 shows a LONG of 65,522 to 65,524 bytes or a field starting `fb ff` / `fc ff` (ADR-101 item 5); a reader change from it makes a5 due | after row 5 | everything | `corpus/` (owner), then a task | - | yes |
| 6 | **R-046** (low): the CL8MSWIN1251 CLOB / NCLOB short-form end | any time | everything | `corpus/` (owner) | - | yes |
| 7 | **OWNER: RHEL 8 host test** with the a4 wheel (not a gate; when a host is at hand) | any time | everything | nothing in the repository | - | yes |
| 8 | **Data Pump** (phase 2): an Architect session plans it, scope with the owner | now (T-21 is done) | rows 2 to 7 | `docs/design/` | - | scope with the owner |
| 9 | **Analyst 21** (documents only; "Inputs for Analyst session 21"), if not done | any time | everything | `docs/formats/` | - | **OWNER** starts it |

What the owner does, in order: push these documents; start the Implementer on T-16c; after its commit push and one
`manylinux2014` dispatch, paste both summaries to the Implementer (for the note) and then to the next Architect session;
then T-16b the same way; then start Reviewer 7. Q-4, R-046, the RHEL 8 host test at any time. Standing owner items:
O-24; Q-13; O-20; the Gogs mirror stays "moved". **What makes a5** (ADR-113 item 4): unchanged.

## Order of the next tasks (session 29, history)

Machine rule unchanged (session 16 below). CI runs on the public GitHub repository and is not a build on the VM.
**OWNER** marks an owner action. ADR-113 items 3 and 4.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | **Release 0.1.0a4** (2026-10-31, tag on C `4b9883c`; record under "Release 0.1.0a4", ADR-111); this session | - | - | - | - | - |
| 1 | **OWNER**: commit and push this session's documents (docs only; their CI run is not compared with anything); the clean-up of "Clean-up after a4 (session 29)" (at least step 1, the rename); archive `oradmp-private`; the GitHub "About" settings (ADR-113 item 6) | now | rows 6 to 9 | `docs/`, the VM, GitHub settings | - | yes |
| 2 | **T-21** (ADR-109 items 3, 4; ADR-112; plan T-21 with its session 29 block): SF-1 (line 200, five `JOB_START` lines), N-1, N-2, N-3, N-5, the CONTRIBUTING pointer, `cargo deny`'s expected warnings, the member-list check, the a4 sdist built from source once | after row 1's rename | rows 6 to 9 (not with another VM build: (k) builds alone) | `ci.yml`, two `t11_*.rs`, `common/synth.rs`, two generators, `CONTRIBUTING.md`, one new test, the note | Sonnet | **OWNER** commits, pushes, one `extended` and one `manylinux2014` dispatch, pastes |
| 3 | **T-16c** `manylinux2014` acceptance: first the `pip download` check for a pyarrow 2014 wheel (may end "rejected" there), then (a) to (d) of ADR-91 item 2 | after row 2 is pushed (both edit `ci.yml`) | rows 6 to 9 | `tools/release_build.sh`, `ci.yml`, the note | Sonnet | **OWNER** for the glibc 2.17 and Rocky Linux 8 runs, dispatches |
| 4 | **T-16b** macOS / Windows, plus the sdist build check in `extended` | after row 3 is pushed | rows 6 to 9 | `.github/workflows/`, `tests/check_dist.py`, `target_os` gates, tests a platform proves wrong | Sonnet | **OWNER** pushes, pastes |
| 5 | **Reviewer 7** on T-21, T-16c, T-16b (rule 7) and, if a5 is due by ADR-113 item 4, the a5 candidate after its version commit | after row 4 | - | `docs/review/` | Opus | **OWNER** starts it |
| 5' | **a5** (only if ADR-113 item 4 holds): Architect session (record Reviewer 7, the a5 checklist with ADR-111 item 6), version commit `0.1.0a5` (Implementer; README lines, classifier), the owner's release | after row 5 | - | as named | - / Sonnet | **OWNER** all release steps |
| 6 | **OWNER: Q-4**, the real-dump counts with `oradmp survey` from the a4 wheel, on the owner's side, counts only | now | everything | nothing in the repository | - | yes |
| 6' | **R-052** (exists) only if Q-4 shows a LONG of 65,522 to 65,524 bytes or a field starting `fb ff` / `fc ff` (ADR-101 item 5); a reader change from it makes a5 due (ADR-113 item 4) | after row 6 | everything | `corpus/` (owner), then a task | - | yes |
| 7 | **R-046** (low): the CL8MSWIN1251 CLOB / NCLOB short-form end | any time | everything | `corpus/` (owner) | - | yes |
| 8 | **OWNER: RHEL 8 host test** with the a4 wheel (not a gate; when a host is at hand) | any time | everything | nothing in the repository | - | yes |
| 9 | **Data Pump** (phase 2): an Architect session plans it, scope with the owner | after row 2 | rows 3 to 8 | `docs/design/` | - | scope with the owner |
| 10 | **Analyst 21** (documents only; "Inputs for Analyst session 21"), if not done | any time | everything | `docs/formats/` | - | **OWNER** starts it |

Standing owner items: O-24; Q-13; O-20; the snapshot folder (owner's choice); the Gogs mirror stays "moved".
**What makes a5** (ADR-113 item 4): one accepted platform wheel (row 3 or 4) or one reader fix (row 6' or a user's
defect report), reviewed; T-21, README wording, classifiers or CI changes alone never make a release.

## Order of the next tasks (session 28, history)

Machine rule unchanged (session 16 below). CI runs on the public GitHub repository and is not a build on the VM.
**OWNER** marks an owner action.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-20b (C = `4b9883c`); its push run, proof run and the two dispatches on C; Reviewer 6c (no blocking finding); this session | - | - | - | - | - |
| 1 | **OWNER**: commit `docs/review/review-06c.md` (done, `075a6bf`) and this session's documents; one push | now | everything | `docs/` | - | yes |
| 2 | **OWNER: release 0.1.0a4** from C: "Release 0.1.0a4 checklist (session 28)", steps 1 to 16 (old root aside, action pins, build at the canonical root, both hashes, `cargo deny`, VM and Rocky Linux 8 installs, `stream_memory.py`, tag, TestPyPI, PyPI, tag pushed to `origin`, release page, tokens deleted) | after row 1 | **nothing that builds** (steps 5 to 9) | the VM, PyPI, a tag | - | **OWNER** all steps |
| 3 | **Architect**: the a4 record from the owner's paste (step 17) | after row 2 | - | `docs/design/` | - | no |
| 4 | **T-21** SF-1, N-1, N-2, N-3, N-5 (ADR-109) | after the a4 tag | row 7, row 8 | `ci.yml`, two `t11_*.rs`, `common/synth.rs`, two generators, `CONTRIBUTING.md`, one new test | Sonnet | **OWNER** commits, pushes, one `extended` dispatch, pastes |
| 5 | **T-16b** macOS / Windows; **T-16c** `manylinux2014` acceptance (ADR-106 item 3) | after row 4 is pushed (both edit `ci.yml`) | row 7, row 8 | `.github/workflows/`, `tools/`, `check_dist.py`, gates, README | Sonnet | **OWNER** for T-16c's glibc 2.17 run |
| 6 | **Reviewer 7** on T-21, T-16b, T-16c (rule 7) | after row 5 | - | `docs/review/` | Opus | **OWNER** starts it |
| 7 | **OWNER: Q-4**, the real-dump counts with `oradmp survey` from the a4 wheel, on the owner's side, counts only | after the a4 tag | everything | nothing in the repository | - | yes |
| 7' | **R-052** (Analyst request, exists) only if Q-4 shows a LONG of 65,522 to 65,524 bytes or a field starting `fb ff` / `fc ff` (ADR-101 item 5) | after row 7 | everything | `corpus/` (owner) | - | yes |
| 8 | **R-046** (low): the CL8MSWIN1251 CLOB / NCLOB short-form end | any time | everything | `corpus/` (owner) | - | yes |
| 9 | **Data Pump** (phase 2): an Architect session plans it | after the a4 tag | rows 4 to 8 | `docs/design/` | - | scope with the owner |
| 10 | **Analyst 21** (documents only; "Inputs for Analyst session 21"), if not done | any time | everything | `docs/formats/` | - | **OWNER** starts it |

Standing owner items (none blocks a4): O-24; the RHEL 8 host test (with the a4 wheel when a host is at hand);
`oradmp-private` and the snapshot folder after the tag (checklist step 16); Q-13; O-20; the Gogs mirror of the new
commits stays "moved".

## Order of the next tasks (session 27, history)

Machine rule unchanged (session 16 below). CI runs on the public GitHub repository and is not a build on the VM.
**OWNER** marks an owner action.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-20 (`e1ed10b`, with `4aa33e6`; push run green); row 1' of session 26 (`extended` and `manylinux2014` on `e1ed10b`, green); this session | - | - | - | - | - |
| 1 | **T-20b** `pipefail` in `ci.yml` (ADR-108; plan T-20b) | now | nothing that builds on the VM | `.github/workflows/ci.yml`, the note | Sonnet | **OWNER** commits C, one push, pastes; pushes `t20b-proof`, dispatches, pastes, deletes the branch |
| 1' | **OWNER**: one `extended` and one `manylinux2014` dispatch on C, pasted | after row 1's push run is green | - | GitHub | - | yes |
| 1'' | **OWNER**: O-24 (the private `CLAUDE.md` text) | any time | everything | private repository | - | yes |
| 2 | **Reviewer 6c** on C (session 27 block of "Release 0.1.0a4"; step 3b) | after rows 1 and 1' | **nothing** (it builds; step 3b alone) | `docs/review/` | Opus | **OWNER** starts it with C's full hash, containers stopped |
| 3 | **Architect**: record 6c; the a4 checklist; a closure task if needed | after row 2 | - | `docs/design/` | - | no |
| 3' | (closure task and its review, only if row 3 names one; then C moves to the closure commit) | after row 3 | - | as named | Sonnet / Opus | as named |
| 4 | **OWNER**: `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-<what it holds>` | before row 5 | everything | the VM | - | yes |
| 5 | **OWNER: release 0.1.0a4** from C: Linux x86_64 `manylinux_2_28` wheel + sdist, `tools/release_build.sh C --twice --release` at the canonical root; sdist hash cross-check with C's CI run; Rocky Linux 8 test; TestPyPI, then PyPI; the tag pushed last to `origin` | after row 3 (and 3') without a blocking finding | nothing that builds | a tag | - | **OWNER** all steps (upload and tag) |
| 6 | **T-16b** macOS / Windows; **T-16c** `manylinux2014` acceptance (ADR-106 item 3) | after the a4 tag | Data Pump planning | `.github/workflows/`, `tools/`, `check_dist.py`, gates, README | Sonnet | **OWNER** for T-16c's Rocky 8 run |
| 6' | **Data Pump** (phase 2): an Architect session plans it | after a4 | row 6 | `docs/design/` | - | scope with the owner |
| 7 | **Analyst 21** (documents only; "Inputs for Analyst session 21"), if not done | any time | everything | `docs/formats/` | - | **OWNER** starts it |

Standing owner items unchanged (session 26 list below; none blocks a4): Q-4, R-052 on Q-4's condition, R-046, the RHEL 8
host test, scratch folders, `oradmp-private` and the snapshot folder, Q-13, O-20; the Gogs mirror of the new commits
stays "moved" (no answer yet, no step needs it).

## Order of the next tasks (session 26, history)

Machine rule unchanged (session 16 below). CI runs on the public GitHub repository and is not a build on the VM.
**OWNER** marks an owner action.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-19b (`1bb7785`); T-19 (m) (`0520cd5`); the cut (P `b55eb10`, S `9950ef2`; first public runs green); this session | - | - | - | - | - |
| 1 | **T-20** a4 version commit (ADR-107; plan T-20) | now | nothing that builds on the VM | version files, `pyproject.toml`, README, CONTRIBUTING, `check_dist.py` + cases, `ci.yml` (`fuzz-check`), the note | Sonnet | **OWNER** commits, one push to `origin`, pastes the run; no tag |
| 1' | **OWNER**: one `extended` and one `manylinux2014` dispatch on C, pasted | after row 1's run is green | - | GitHub | - | yes |
| 1'' | **OWNER**: O-24 (the private `CLAUDE.md` text) | any time | everything | private repository | - | yes |
| 2 | **Reviewer 6c** (items 1 to 11 of the session 26 block; step 3b) | after row 1' | **nothing** (it builds; step 3b alone) | `docs/review/` | Opus | **OWNER** starts it, containers stopped |
| 3 | **Architect**: record 6c; the a4 checklist; a closure task if needed | after row 2 | - | `docs/design/` | - | no |
| 3' | (closure task and its review, only if row 3 names one) | after row 3 | - | as named | Sonnet / Opus | as named |
| 4 | **OWNER**: `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-<what it holds>` | before row 5 | everything | the VM | - | yes |
| 5 | **OWNER: release 0.1.0a4**: Linux x86_64 `manylinux_2_28` wheel + sdist, `tools/release_build.sh C --twice --release` at the canonical root; hash cross-check with C's CI; Rocky Linux 8 test; TestPyPI, then PyPI; the tag pushed last to `origin` | after row 3 (and 3') without a blocking finding | nothing that builds | a tag | - | **OWNER** all steps (upload and tag) |
| 6 | **T-16b** macOS / Windows; **T-16c** `manylinux2014` acceptance (ADR-106 item 3) | after the a4 tag | Data Pump planning | `.github/workflows/`, `tools/`, `check_dist.py`, gates, README | Sonnet | **OWNER** for T-16c's Rocky 8 run |
| 6' | **Data Pump** (phase 2): an Architect session plans it | after a4 | row 6 | `docs/design/` | - | scope with the owner |
| 7 | **Analyst 21** (documents only; "Inputs for Analyst session 21"), if not done | any time | everything | `docs/formats/` | - | **OWNER** starts it |

Standing owner items, none blocks a4 (ADR-106 item 4): Q-4 with `oradmp survey` from the a4 wheel; R-052 only on Q-4's
condition; R-046 (low); the RHEL 8 host test with the a4 wheel; scratch folders; `oradmp-private` and the snapshot folder
(keep until the a4 tag, then the owner's choice); Q-13; O-20 (optional).

## Order of the next tasks (session 25, history)

Machine rule unchanged (session 16 below). CI runs on GitHub and is not a build on the VM. **OWNER** marks an owner
action.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-19 (`921971f`, CI green on `bbf921b`); R-048 to R-051 run; Analyst 19 and 20 (`588f658`); this session | - | - | - | - | - |
| 1 | **T-19b** (ADR-99 to ADR-103) | now | nothing that builds on the VM | `crates/`, `tests/`, `python/oradmp/`, README, the note | Sonnet | yes: **OWNER** commits the corpus files, then T-19b, one push, pastes the run |
| 1' | **OWNER**: `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-d081109` (T-16a2 note (g) 1), if not done | any time before row 7 | everything | the VM | - | yes |
| 2 | **T-19 (m)**: step 3b (micro-benchmarks, throughput, memory test on `T_BIG`) and the 600 s fuzz runs, alone | after row 1's push run is green | **nothing** | the note only | Sonnet | yes: **OWNER** stops `ora11` / `ora18`, sets `ORADMP_WORK`, commits the note |
| 3 | **OWNER**: the cut (Q-14; checklist by the owner's assistant). First step: rename the private GitHub repository, repoint every remote, `git remote -v`; then create the public `oradmp` (ADR-104 item 3); `docs/prompts`, `.claude`, `CLAUDE.md` stay private | after row 2 | nothing (all agent sessions ended) | a new public repository; Gogs archive | - | yes: every [OWNER] item of Q-14 |
| 3' | **Architect** (short): record S and P, close Q-14, the public address | after row 3's first public run is green | - | `docs/design/` | - | no |
| 4 | **a4 version commit** `0.1.0-alpha.4` (+ `[project.urls]`, rule and case if row 3 is done) | after row 3' (or after row 2 without the cut) | - | version files, README, `pyproject.toml`, `check_dist.py` | Sonnet (small) | **OWNER** commits, pushes, pastes the run |
| 5 | **Reviewer 6c**: T-17b, T-16a, T-16a2, T-19, T-19b, T-19 (m)'s figures, the version commit (and the cut); items 1 to 8 with the session 25 additions; step 3b | after row 4's CI run | nothing (it builds) | `docs/review/` | Opus | no |
| 6 | **Architect**: record 6c; the a4 checklist; a closure task if needed | after 6c | - | `docs/design/` | - | no |
| 7 | **Release 0.1.0a4** (order in "Release 0.1.0a4", sessions 23 to 25) | after 6c without a blocking finding | nothing that builds | a tag | - | **OWNER** all steps |
| 7' | **OWNER**: the cut, if not done in row 3 | after the a4 tag is pushed | - | as row 3 | - | yes |
| 8 | **Analyst 21** (documents only): "Inputs for Analyst session 21" (`open-questions.md`) | any time | everything | `docs/formats/` | - | **OWNER** starts it |
| 9 | **T-16b** macOS / Windows; **T-16c** `manylinux2014` | after the a4 tag | Data Pump planning | `.github/workflows/`, `tools/`, `check_dist.py`, gates, README | Sonnet | **OWNER** dispatches if private; Rocky 8 run for T-16c |
| 9' | **Data Pump** (phase 2): an Architect session plans it | after a4 | row 9 | `docs/design/` | - | scope with the owner |
| 10 | **DuckDB extension** (side project) | not planned (Q-13) | - | - | - | Q-13 |

Standing owner items, no deadline: the real RHEL 8 host test (with a4); **Q-4** with `oradmp survey` from the a4 wheel
(or a T-19b build): type codes (231?), the LONG line's `largest`, `65,521 to 65,534 bytes`, any `W-UNTESTED-LAYOUT` or
stop on a LONG / LONG RAW column; R-052 only on ADR-101 item 5's condition; R-046 (low); Q-13; optional: push `v0.1.0a1`
(only to the archive after the cut), O-20.

## Order of the next tasks (session 24, history)

Machine rule unchanged (session 16 below). CI runs on GitHub and is not a build on the VM.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-16a2 (`af6a54a`); the owner's R-017, R-040, R-047 runs (`cca4c65`); Analyst 17 (`8162eb0`); this session | - | - | - | - | - |
| 1 | **T-19** (ADR-94, ADR-95, ADR-97) | now | nothing that builds on the VM | `crates/oradmp-core/`, `tests/`, `python/oradmp/`, README, the note | Sonnet | yes: **OWNER** commits, pushes, pastes the run |
| 1' | **OWNER**: `mv /var/tmp/oradmp-release /var/tmp/oradmp-release-d081109` (T-16a2 note (g) 1) | any time before row 6 | everything | the VM | - | yes |
| 1'' | **Analyst 18** (documents only): copy R-050 into `requests.md` (open-questions "Inputs for Analyst session 18") | any time | everything | `docs/formats/` | - | **OWNER** starts it |
| 1''' | **OWNER**: R-048, R-049 (and R-050 after row 1'') on Oracle when convenient; **the owner's assistant writes the runners** from `requests.md`; R-048's LONG RAW half first if Q-4 shows LONG RAW | any time | everything | `corpus/` (owner) | - | yes |
| 2 | **OWNER**: the cut (Q-14 steps, session 23), recommended | after row 1's push run is green | nothing (all agent sessions ended) | a new public repository; Gogs archive | - | yes: every [OWNER] item of Q-14 |
| 2' | **Architect** (short): record S and P, close Q-14, name the public address | after row 2's first public run is green | - | `docs/design/` | - | no |
| 3 | **a4 version commit** `0.1.0-alpha.4` (+ `[project.urls]`, rule and case if row 2 is done) | after row 2' (or after row 1 without the cut) | - | version files, README, `pyproject.toml`, `check_dist.py` | Sonnet (small) | **OWNER** commits, pushes, pastes the run |
| 4 | **Reviewer 6c** (items 1 to 8; step 3b) | after row 3's CI run | nothing (it builds) | `docs/review/` | Opus | no |
| 5 | **Architect**: record 6c; the a4 checklist; a closure task if needed | after 6c | - | `docs/design/` | - | no |
| 6 | **Release 0.1.0a4** (order in "Release 0.1.0a4", sessions 23 and 24) | after 6c without a blocking finding | nothing that builds | a tag | - | **OWNER** all steps |
| 6' | **OWNER**: the cut, if not done in row 2 | after the a4 tag is pushed | - | as row 2 | - | yes |
| 7 | **Analyst** on R-048 / R-049 / R-050, then Architect, then **T-19b** (the R-048 boundary, LONG RAW) | after row 1''' | rows 2 to 6 (documents); T-19b after row 6 unless before row 3 | `docs/formats/`, `docs/design/`, then as T-19 | Sonnet (T-19b) | the fixtures |
| 8 | **T-16b** macOS / Windows; **T-16c** `manylinux2014` | after the a4 tag | Data Pump planning | `.github/workflows/`, `tools/`, `check_dist.py`, gates, README | Sonnet | **OWNER** dispatches if private; Rocky 8 run for T-16c |
| 8' | **Data Pump** (phase 2): an Architect session plans it | after a4 | row 8 | `docs/design/` | - | scope with the owner |
| 9 | **DuckDB extension** (side project) | not planned (Q-13) | - | - | - | Q-13 |

Standing owner items, no deadline: the real RHEL 8 host test (with a4), Q-4 (real-dump survey; now also: LONG lengths
between 65,521 and 65,534, LONG RAW above 32,767, type 231), R-046 (low), Q-13; optional: push `v0.1.0a1` (only to the
archive after the cut), O-20.

## Order of the next tasks (session 23, history)

Machine rule unchanged (session 16 below). CI runs on GitHub and is not a build on the VM; T-16a2 (a) is one.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-17b (`4b4e732`); T-16a (`c1d1c58`, `7a2d028`, `d081109`; green); Analyst session 16 (`860ebd4`, R-047); O-22; this session | - | - | - | - | - |
| 1 | **T-16a2** (ADR-90, ADR-92): (a) the rebuild of `d081109` at the canonical root first, then `--release`, `--build-root`, the fuzz artifact, the `auditwheel` pin, the note | now | nothing that builds on the VM | `tools/`, `tests/check_dist.py`, `tests/test_check_dist.py`, `.github/workflows/`, CONTRIBUTING | Sonnet | yes: **OWNER** pastes the full `d081109` hashes; commits, pushes, pastes the run |
| 1' | **OWNER**: O-23 (b) the billing figure; R-047 and R-017 on Oracle when convenient | any time | everything | `corpus/` (owner) | - | yes |
| 2 | **OWNER**: the cut (Q-14 steps, session 23), recommended | after row 1's push run is green | nothing (all agent sessions ended) | a new public repository; Gogs archive | - | yes: every [OWNER] item of Q-14 |
| 2' | **Architect** (short): record S and P, close Q-14, name the public address for the version commit | after row 2's first public run is green | - | `docs/design/` | - | no |
| 3 | **a4 version commit** `0.1.0-alpha.4` (+ `[project.urls]`, rule and case if row 2 is done) | after row 2' (or after row 1 without the cut) | - | version files, README, `pyproject.toml`, `check_dist.py` | Sonnet (small) | **OWNER** commits, pushes, pastes the run |
| 4 | **Reviewer 6c** (items 1 to 7) | after row 3's CI run | nothing (it builds) | `docs/review/` | Opus | no |
| 5 | **Architect**: record 6c; the a4 checklist; a closure task if needed | after 6c | - | `docs/design/` | - | no |
| 6 | **Release 0.1.0a4** (order in "Release 0.1.0a4", session 23) | after 6c without a blocking finding | nothing that builds | a tag | - | **OWNER** all steps |
| 6' | **OWNER**: the cut, if not done in row 2 | after the a4 tag is pushed | - | as row 2 | - | yes |
| 7 | **T-16b** macOS / Windows; **T-16c** `manylinux2014` | after the a4 tag | Data Pump planning | `.github/workflows/`, `tools/`, `check_dist.py`, gates, README | Sonnet | **OWNER** dispatches if private; Rocky 8 run for T-16c |
| 7' | **Data Pump** (phase 2): an Architect session plans it | after a4 | row 7 | `docs/design/` | - | scope with the owner |
| 8 | **DuckDB extension** (side project) | not planned (Q-13) | - | - | - | Q-13 |

Standing owner items, no deadline: the real RHEL 8 host test (with a4), Q-4 (real-dump survey), R-040's run, R-046
(low), R-047 and R-017 (row 1'), Q-13; optional: push `v0.1.0a1` (only to the archive after the cut), O-20.

## Order of the next tasks (session 22, history)

Machine rule unchanged (session 16 below): one build at a time; `CARGO_BUILD_JOBS=2`; `cargo nextest run -j 2`;
fuzzing, benchmarks and memory runs alone; `TMPDIR` on `/var/tmp`. CI runs on GitHub and does not count as a build
on the VM.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-18d (`ba8a89c`); T-17 (`edc62a5`); Reviewer 6b (no blocking finding); Analyst session 15 (`2a60b86`); O-21; O-22 (c) answered (logs pasted); this session | - | - | - | - | - |
| 1 | **T-17b** review 06b fixes (ADR-86) | now | Analyst session 16 (documents only) | `crates/oradmp-core/tests/`, `_cli.py`, `tests/test_paths.py`, `tools/fuzz_seeds.sh`, CONTRIBUTING | Sonnet | the owner commits it |
| 1' | **Analyst session 16** (list in `open-questions.md`, "Inputs for Analyst session 16": R-047 into `requests.md`; R-017's new user) | now; the owner starts it | T-17b | `docs/formats/` | - | no |
| 1'' | Owner: run **R-047** and **R-017** in the 18c and 11g containers when convenient (the owner's assistant writes the runner); confirm O-22 (a) | any time | everything | `corpus/` (owner) | - | yes |
| 2 | **T-16a** Linux CI (ADR-87) | after T-17b is committed and pushed to `github` | nothing that builds on the VM | `.github/workflows/`, CONTRIBUTING | Sonnet | yes: the owner pushes, pastes summaries, dispatches the extended job |
| 2' | Owner: **the one-time cut** (Q-14), optional | after T-16a's first green Linux run; not after row 3 starts | rows 1', 1'' | a new GitHub repository; Gogs archive | - | yes |
| 3 | **a4 version commit** `0.1.0-alpha.4` (+ `[project.urls]` if 2' is done) | after T-16a is done | - | version files, README (`pyproject.toml`, `check_dist.py` if 2') | Sonnet (small) | the owner commits and pushes it |
| 4 | **Reviewer 6c** (T-17b, T-16a, the a4 candidate; "Done when" under "Release 0.1.0a4") | after 3 and its CI run | nothing (it builds) | `docs/review/` | Opus | no |
| 5 | **Architect session**: record 6c; the a4 checklist; a closure task if 6c finds something | after 6c | - | `docs/design/` | - | no |
| 6 | **Release 0.1.0a4**, Linux only (ADR-88) | after 6c (and its closure, if any) without a blocking finding | nothing that builds during steps 6 and 7 | a tag | - | yes |
| 7 | **T-16b** macOS / Windows (ADR-87) | after the a4 tag | the Data Pump planning (documents) | `.github/workflows/`, `check_dist.py`, gates, README | Sonnet | yes: dispatches while private |
| 7' | **Data Pump** (phase 2): an Architect session plans it | after a4 | T-16b | `docs/design/` | - | scope with the owner |
| 8 | **DuckDB extension** (side project) | after the main project; not planned (Q-13) | - | - | - | Q-13 |

Standing owner items, no deadline: the real RHEL 8 host test (with a4 when it is out), the real-dump survey (Q-4),
R-040's run (the owner's assistant writes the runner), R-046 (low), R-047 and R-017 (row 1''), Q-14 (row 2'); optional:
push `v0.1.0a1`.

## Order of the next tasks (session 21, history)

Machine rule unchanged (session 16 below): one build at a time; `CARGO_BUILD_JOBS=2`; `cargo nextest run -j 2`;
fuzzing, benchmarks and memory runs alone; `TMPDIR` on `/var/tmp`.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-18c (`ff3d6b2`); Reviewer 6a-closure (no blocking finding); Analyst session 14 (`27c0012`, R-003 rejected); **release 0.1.0a3** (2026-10-24); O-21 (`9f1dc4d`); this session | - | - | - | - | - |
| 1 | **T-18d** SF-1, non-UTF-8 file names, `-n -1` subprocess case, README (ADR-82) | now | Analyst session 15 (documents only) | `crates/oradmp-py/src/lib.rs`, `_cli.py`, `_survey.py`, tests, README | Sonnet | the owner commits it |
| 1' | **Analyst session 15** (list in `open-questions.md`, "Inputs for Analyst session 15": the R-003 wording) | now; the owner starts it | T-18d (it builds nothing) | `docs/formats/` | - | no |
| 1'' | Owner: **O-22** (GitHub for T-16); confirm `v0.1.0a3` on `origin`; **O-19 (a)** visibility when ready (recommended after T-16's first green Linux run) | any time | everything | GitHub settings, the VM's `gh` login | - | yes |
| 2 | **T-17** synthetic files from the rules (ADR-80) | after T-18d is committed | nothing that builds (its long runs alone) | `crates/oradmp-core/tests/`, `tools/fuzz_seeds.sh` or CONTRIBUTING | Opus | no |
| 3 | **T-16** CI on GitHub Actions and more platforms (ADR-68, ADR-79, ADR-83) | after T-17 is committed and O-22 is done (first, if O-22 is done before T-17 starts) | nothing on the VM (CI runs on GitHub) | `.github/workflows/`, `check_dist.py`, `target_os` gates, README; `[project.urls]` if public | Sonnet | O-22; the owner dispatches macOS / Windows runs |
| 4 | **a4 version commit** `0.1.0-alpha.4` | after T-16 and T-17 | - | version files, README | Sonnet (small) | the owner commits it |
| 5 | **Reviewer session 6b** (T-18d, T-17, T-16, the a4 candidate) | after 4 | nothing (it builds) | `docs/review/` | Opus | no |
| 6 | **Architect session**: record 6b; the a4 checklist; a closure task if 6b finds something | after 6b | - | `docs/design/` | - | no |
| 7 | **Release 0.1.0a4** (ADR-84) | after 6b (and its closure, if any) without a blocking finding | nothing that builds during steps 6 and 7 | a tag | - | yes |
| 8 | **Data Pump** (phase 2) | after a4; planned by the Architect when it starts | - | - | - | scope with the owner |
| 9 | **DuckDB extension** (side project) | after the main project; not planned (Q-13) | - | - | - | Q-13 |

Standing owner items, no deadline: the RHEL 8 host test (Q-4), the real-dump survey (Q-4), R-040's run (the owner's
assistant writes the runner), R-046 (low), optional: push `v0.1.0a1`.

## Order of the next tasks (session 20, history)

Machine rule unchanged (session 16 below): one build at a time; `CARGO_BUILD_JOBS=2`; `cargo nextest run -j 2`;
fuzzing, benchmarks and memory runs alone; `TMPDIR` on `/var/tmp`.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-18b (`d4d20ef`); Analyst session 13 (`bc2804e`); Reviewer session 6a (no blocking finding); this session | - | - | - | - | - |
| 1 | **T-18c** SF-1, SF-2, N-1 to N-3 (ADR-77) | now | Analyst session 14 (documents only) | `_cli.py`, `tests/test_cli.py`, `tests/test_version.py`, `README.md` | Sonnet | the owner commits it (the new a3 candidate) |
| 1' | **Analyst session 14** (list in `open-questions.md`, "Inputs for Analyst session 14": R-003 rejected, the R-040 runner) | now; the owner starts it | T-18c (it builds nothing) | `docs/formats/` | - | the R-040 text |
| 1'' | Owner actions **O-21** (`.github/` in `CLAUDE.md`) and **O-22** (GitHub settings for T-16); visibility when ready | any time before T-16 | everything | `CLAUDE.md`, GitHub settings | - | yes |
| 2 | **Reviewer 6a-closure** (below) | after T-18c is committed | nothing (it builds) | `docs/review/` | Opus | no |
| 3 | **Release 0.1.0a3**, Linux only ("Release 0.1.0a3 checklist (session 20)") | after 6a-closure without a blocking finding | nothing that builds during steps 6 and 7 | a tag (pushed at 10 (g)) | - | yes: steps 1, 4 to 11 |
| 4 | **Architect session 21**: the a3 record; answers to anything 6a-closure found | after the release (or at once if 6a-closure blocks) | - | `docs/design/` | - | the record facts (step 11) |
| 5 | **T-16** CI on GitHub Actions and more platforms (ADR-68, ADR-79) | after the a3 tag is pushed, when O-21 and O-22 are done | not with T-17 on the VM (CI runs on GitHub) | `.github/`, `check_dist.py`, `target_os` gates, README | Sonnet | O-21, O-22 |
| 5' | **T-17** synthetic files from the rules (ADR-80) | after the a3 tag is pushed; first if T-16 still waits for O-22 | not with T-16 on the VM | `crates/oradmp-core/tests/` | Opus | no |
| 6 | **Reviewer session 6b** (T-16, T-17) | after both | nothing | `docs/review/` | Opus | no |
| 7 | **Release 0.1.0a4** (T-16's accepted wheels; Project-URL if public; candidate: the API's `OSError` with `.filename` and the OS text, ADR-77 item 4, if scheduled) | after 6b without a blocking finding | as row 3 | version commit, a tag | - | yes |
| 8 | **Data Pump** (phase 2) | after a4; planned by the Architect when it starts | - | - | - | scope with the owner |
| 9 | **DuckDB extension** (side project) | after the main project; not planned (Q-13) | - | - | - | Q-13 |

T-16 before T-17 when both can start (it is user-visible, and CI then runs T-17's tests on every platform from their
first commit); T-17 first while T-16 waits for O-22.

**Reviewer 6a-closure, "Done when"** (report only, short; one build at a time; no benchmarks):
1. **Diff**: `git diff --stat d4d20ef C -- . ':!docs'` lists only `python/oradmp/_cli.py`, `tests/test_cli.py`,
   `tests/test_version.py`, `README.md`; the whole diff read against T-18c (a) to (g), ADR-77 item 4 and ADR-72 as
   amended. `git diff --stat d4d20ef C -- crates/` is empty (so no benchmark and no Rust re-review is due).
2. **Mutations**: every mutation of the T-18c note repeated alone: SF-1 (a subprocess back to `ROOT / "python"`, in
   the installed-wheel git clone; the pandas scan back to `ROOT / "python" / "oradmp"`, in the copied layout), the
   seven SF-2 survivors of review 06a, the six pass-through drops, the two N-3 mutations, the N-1 README mutation:
   each fails the named test, all others pass; `git status` clean after each.
3. **Build twice**: `tools/release_build.sh C --twice` in a fresh build root: `check_dist: ok` twice, wheel and sdist
   identical, `commit:` = C, `0.1.0a3` in both file names and in METADATA / PKG-INFO, `entry_points.txt` as ADR-72
   item 11.
4. **Checklist step 3** in a clean clone of C, dev build with probes: the full list (fixture checksums, fmt, clippy
   twice, `cargo nextest run -j 2` 412 + 3 skipped, `cargo deny check licenses`, ruff, ruff format, mypy, `pytest -rs`
   with 0 skipped and the T-18c count). This is the release's step 3.
5. **Installed wheel** (from 3), three fresh venvs (Python 3.11 with pyarrow 15.0.2 and pandas 2.2.3; the newest
   CPython, pyarrow and pandas; one with pyarrow only): full `pytest -rs` (a) from the build clone `$ORADMP_BUILD_ROOT/src`
   (a git clone): **0 failed**, skips exactly the three probe tests (plus `test_consumers.py` without pandas); (b) from
   a copied-folder layout (`tests/`, `corpus/fixtures/`, the root files; no `python/`, no `.git`): **0 failed**, skips the
   three probe tests and `test_project_name.py` "not a git clone" (plus `test_consumers.py` without pandas).
6. **CLI from the wheel** (each venv's `bin/`): `oradmp --version` = `oradmp 0.1.0a3`; `tables` on a copied fixture;
   `tables missing.dmp` prints `oradmp: error: [E-IO] No such file or directory: missing.dmp`, exit 3; a mode-000 copy
   prints the EACCES text, exit 3 (not as root); in the pyarrow-only venv `export` to `.csv` and to `.parquet` without
   `--format`, and `oradmp head ... T_TEXT | head -1` exit 0 with an empty stderr.
7. README "Command line" read against arch 3.5, ADR-72 as amended and ADR-77 (the suffix sentence, `--interval-as`
   in the list, the exit-3 line).
8. **Verdict**: blocking finding or not, for the a3 upload of C. Not repeated: what review 06a checked on T-13j, T-15b,
   T-18, T-18b and the benchmarks.

## Order of the next tasks (session 19, history)

Machine rule unchanged (session 16 below): one build at a time; `CARGO_BUILD_JOBS=2`; `cargo nextest run -j 2`;
fuzzing, benchmarks and memory runs alone; `TMPDIR` on `/var/tmp`.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-13j (`1c17633`, `460aef5`); Analyst session 12 (`1c17633`); O-17 `T_BIG` (`0dac62b`); T-15b (`0956df0`); T-18 (`2ba62eb`) | - | - | - | - | - |
| 1 | **T-18b** memory-test report, micro-benchmarks, two CLI cases, README; then the **a3 version commit** (ADR-75) | now | Analyst session 13 (documents only) | `t15_memory.rs`, `_cli.py`, `tests/test_cli.py`, README, version files | Sonnet | the `ORADMP_WORK` path in the prompt |
| 1' | **Analyst session 13** (list in `open-questions.md`, "Inputs for Analyst session 13") | now; the owner starts it | T-18b (it builds nothing) | `docs/formats/` | - | the owner decides R-001 to R-007 |
| 2 | **Reviewer session 6a** (T-13j, T-15b, T-18, T-18b; benchmarks repeated; the a3 candidate) | after T-18b's version commit | nothing (benchmarks alone) | `docs/review/` | Opus | the `ORADMP_WORK` path |
| 3 | **Architect session 20**: answers 6a; the new baseline into `benchmarks.md` if 6a's repeat agrees; T-17's "Done when" | after Reviewer 6a | - | `docs/design/` | - | no |
| 4 | **Release 0.1.0a3**, Linux only (checklist; 3b as the session 19 lines) | after 6a without a blocking finding (and session 20 if 6a found something that changes a shipped file) | nothing that builds during steps 3, 6, 7 | a tag | - | yes: steps 1, 4, 6, 8, 9, 10 |
| 5 | **T-17** synthetic files from the rules | after the a3 tag (step 9) | - | core tests | Sonnet | no |
| 6 | **T-16** more platforms (ADR-68; session 19 additions) | when the GitHub account and push mirror exist (Q-11); after the a3 tag | not with T-17 (machine rule for local builds; CI runs elsewhere) | CI configuration, `check_dist.py`, `target_os` gates, README | Sonnet | Q-11 |
| 7 | **Reviewer session 6b** (T-16; T-17 if done) | after T-16 | nothing | `docs/review/` | Opus | no |
| 8 | **Next release** (T-16's accepted wheels; Project-URL if public) | after 6b without a blocking finding | as row 4 | a tag | - | yes |
| 9 | **Data Pump** (phase 2) | after T-17; planned by the Architect when it starts | - | - | - | scope with the owner |
| 10 | **DuckDB extension** (side project) | after the main project; not planned (Q-13) | - | - | - | Q-13 |

**Reviewer session 6a, "Done when"** (report only; one build at a time; benchmarks and memory runs alone):
1. T-13j: its mutation table repeated alone (s1o, s1e, s1i, line_start, m10 to m21, the README link and version
   mutations); note that T-13j's code is split between `1c17633` ("Analyst 12") and `460aef5`.
2. T-18 and T-18b against ADR-72 as amended (ADR-75): the walk of `ERROR_CODES`, no partial file, stdout / stderr
   split, no pandas, the `entry_points.txt` rule and its cases (m22 to m24b), the refused-type preflight, the
   stopped-scan rule, the success-with-warnings case; at least these T-18 mutations repeated: `OSError` and
   `UsageError` out of `EXIT_CODES`, partial file not removed, reader warnings not printed, closed-pipe handling off.
   The README "Command line" section read against arch 3.5.
3. Benchmarks under `benchmarks.md` rule 1: `read_table/R_STAT`, `number/OraNumber::decode`, both `T_ROWS` throughput
   rows and `T_BIG`; `t15_memory` three runs with all files; the b = 1 mutation once on `T_BIG`. Each figure within 15 %
   of the Implementer's (T-15b, T-18b) or the difference named.
4. The a3 candidate (T-18b's version commit): `tools/release_build.sh <commit> --twice` reproducible, `check_dist:
   ok`, the wheel lists `oradmp-0.1.0a3.dist-info/entry_points.txt`; checklist step 3 in a clean clone (counts);
   the wheel in fresh venvs (Python 3.11 with pyarrow 15.0.2 and pandas 2.2.3; the newest CPython and pyarrow; one
   venv with pyarrow only): full `pytest -rs` from a git clone with exactly the three probe skips; from the venv's
   `bin/`: `oradmp --help`, `oradmp --version`, `oradmp tables` on a copied fixture; in the pyarrow-only venv
   `oradmp export` to CSV and to Parquet and `oradmp head ... | head -1` (exit 0, empty stderr).
5. Verdict: blocking finding or not, for a3.

## Order of the next tasks (session 18, history)

Machine rule unchanged (session 16 below): one build at a time; `CARGO_BUILD_JOBS=2`; `cargo nextest run -j 2`;
fuzzing, benchmarks and memory runs alone; `TMPDIR` on `/var/tmp`.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | Reviewer session 5 (`dc7668e`, no blocking finding); release run B (0.1.0a2 on TestPyPI and PyPI, 2026-10-22) | - | - | - | - | O-18 (record items) |
| 1 | **T-13j** review 05 fixes and the LOB-length branch (ADR-69 item 2, ADR-71) | now | Analyst session 12 (documents only) | `io.rs`, `exp/rows.rs`, core test comments, `lib.rs` comment, pytest files, `check_dist` cases, README, CONTRIBUTING | Sonnet | O-19 (a) for the Project-URL only |
| 1' | **Analyst session 12** (`docs/formats/README.md`; R-040 when the owner gives its text) | now; the owner starts it | T-13j (it builds nothing) | `docs/formats/` | - | the owner runs it |
| 2 | **T-15b** memory test on `T_BIG` (ADR-69 item 3) | after T-13j and **O-17** | nothing (alone) | the note | Sonnet | O-17 |
| 3 | **T-16** more platforms (ADR-68) | after T-13j and **Q-11** | - | CI configuration, `check_dist.py`, `target_os` gates, README | Sonnet | Q-11 |
| 4 | **T-18** the command-line tool (ADR-72) | after T-16 (or before it if the owner takes the swap of ADR-73) | - | `python/oradmp/_cli.py`, `pyproject.toml`, tests, README | Sonnet | no |
| 5 | **Reviewer session 6** (T-13j, T-16, T-18; the T-15b numbers if done) | after T-18 and the a3 version commit | nothing | `docs/review/` | Opus | no |
| 6 | **Release 0.1.0a3** (checklist with the session 18 corrections; 3b due) | after Reviewer 6 without a blocking finding | nothing that builds during steps 3, 6, 7 | the version commit, a tag | - | yes: steps 1, 4, 6, 8, 9, 10 |
| 7 | **T-17** synthetic files from the rules | after a3 (or after Reviewer 6) | - | core tests | Sonnet | no |
| 8 | **Data Pump** (phase 2) | after T-17; planned by the Architect when it starts | - | - | - | scope with the owner |
| 9 | **DuckDB extension** (side project) | after the main project; not planned (Q-13) | - | - | - | Q-13 |

**Reviewer session 6 reads** (session 18; replaced by 6a / 6b in session 19): the T-13j diff and its mutations repeated alone (s1o, m10 to m21 (m22), the README
link and version tests, line_start; the LOB cases); T-16 as its "Done when" says; T-18 against ADR-72 (exit codes,
no partial file, stdout / stderr split, no pandas, `entry_points.txt` rule and its case); the release build of the
a3 candidate with `--twice` and `check_dist: ok`. Report only.

## Order of the next tasks (session 17, history)

Machine rule unchanged (session 16 below).

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-13i (`469416d`, `dc7668e`); Analyst session 11 (rules v3.6, in `469416d`) | - | - | - | - | - |
| 1 | **Reviewer session 5, closure** (ADR-67 item 1; list below with items (7), (8) of ADR-69 item 5) | now | nothing | `docs/review/` | Opus | no |
| 2 | **Release 0.1.0a2, run B** (checklist, unchanged) | after Reviewer 5 without a blocking finding, Q-10 | nothing that builds during steps 3, 6, 7 | a tag | - | yes: steps 1, 4, 6, 8, 9, 10 |
| 3 | **T-13j** the LOB-length branch (ADR-69 item 2) | after run B | Analyst or owner work (no build) | `io.rs`, `exp/rows.rs`, `t14_damage.rs` | Sonnet | no |
| 4 | **T-16** More platforms (ADR-68) | after T-13j and Q-11 | - | as session 16 | Sonnet | Q-11 |
| 5 | **T-17** synthetic files from the rules | after T-16 | - | core tests | Sonnet | no |
| any | **T-15b** memory test on `T_BIG` (ADR-69 item 3) | after run B and **O-17** | nothing (alone) | the note | Sonnet | O-17 |

**Reviewer session 5 reads** the session 16 list (1) to (6) below, plus: (7) the ADR-64 amendment (session 17)
against the two S-2 tests of `tests/test_stream.py` (iterator: issued when the error ends the iteration, not again at
the `with` end; stream: at the `with` end only); (8) `469416d` also adds a root `uv.lock` and comments out the three
`fuzz/` lines of the root `.gitignore`, neither named in the T-13i note: is `uv.lock` in the sdist (`check_dist.py`
and `tar tzf` of the candidate's sdist), and is either change agreed with the owner (the root `.gitignore` is the
owner's, O-16). Report only.

## Order of the next tasks (session 16, history)

**Machine rule (12 GB; ADR-62 item 2)**, unchanged: one build at a time; `CARGO_BUILD_JOBS=2`; `cargo nextest run -j
2`; fuzzing, benchmarks and memory runs alone; `TMPDIR` on `/var/tmp` (the `/tmp` tmpfs is 3.9 GiB and fills up).

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-14 (`abd42ad`), T-15 (`b3a5a7f`), Reviewer session 4 (`2b40fe4`): no blocking finding; run A tagged (`v0.1.0a1` on `b7304d5`) | - | - | - | - | - |
| 1 | **T-13i** review 04 fixes and the version bump (ADR-64, ADR-65, ADR-67 item 2) | now | Analyst session 11 (documents only) | tests, `tools/`, binding wraps, stub, CONTRIBUTING; (l): `Cargo.toml`, lock files, README | **Opus** | Q-10 for item (l) only |
| 1' | **Analyst session 11** (rules T9, X2; R-045, R-040 records; `open-questions.md` (b)) | now; the owner starts it | T-13i (it builds nothing) | `docs/formats/` | - | the owner runs it |
| 2 | **Reviewer session 5, closure** (ADR-67 item 1): the T-13i diff, its mutations, `tools/fuzz_run.sh` once, the release build of the candidate with `--twice` | after T-13i is committed | nothing | `docs/review/` | Opus | no |
| 3 | **Release 0.1.0a2, run B** (checklist; TestPyPI, then PyPI) | after Reviewer 5 without a blocking finding, Analyst 11, Q-10 | nothing that builds during steps 3, 6, 7 | a tag | - | yes: steps 1, 4, 6, 8, 9, 10 |
| 4 | **T-16** More platforms (ADR-68) | after run B and Q-11 | - | CI configuration, `check_dist.py`, gates, README | Sonnet | Q-11 |
| 5 | **T-17** synthetic files from the rules | after T-16 | - | core tests | Sonnet | no |

**Reviewer session 5 reads** (short; rule 7 of `CLAUDE.md` for the T-13i group): (1) T-13i's diff against ADR-64,
ADR-65 and the plan's "Done when"; (2) each mutation repeated alone: s1e, s1f, m1 to m9, premise, d9, d10 (and d1 to d8
once), a1, a2, c2; (3) the S-2 patch bytes checked against `exp_parse.py --rows` and rule [Y1] (not against the
reader); (4) `tools/fuzz_run.sh walk 30` from a fresh clone; (5) `tools/release_build.sh <candidate> --twice` into a
scratch root: `check_dist: ok`, 0 home paths, identical hashes, version 0.1.0a2 in both names and METADATA; (6) the
README's version and install lines against Q-10's answer. Nothing else is re-read.

**Owner actions**: **Q-10** (upload 0.1.0a2? recommendation: yes, after Reviewer 5; needed before T-13i (l));
**Analyst session 11** (before run B); **O-16** (`.gitignore` in `CLAUDE.md`); run A's record (step 11: commit,
hashes, RHEL 8 result) and the **RHEL 8 install test** (run A, and again for a2 in run B step 8); run B's owner steps
(gate, `cargo deny` network checks, build or its check, tag, TestPyPI and PyPI uploads with the token handling of step
10); **R-040**; **R-046** (low); **Q-4** (the real-dump survey: set ids, type codes, partitioning; also the beta gate);
**Q-11** (CI service, for T-16); O-15's `T_BIG` (optional).

## Order of the next tasks (session 15, history)

**Machine rule (12 GB; ADR-62 item 2)**, for every session from now on, Implementer and Reviewer alike: one build at a
time (no two `cargo` / `maturin` commands at once, no second session building); `CARGO_BUILD_JOBS=2` for every `cargo`
and `maturin` command; `cargo nextest run -j 2`; fuzzing and benchmarks alone on the machine (no build, test or other
session at the same time). An out-of-memory kill is repeated alone, not with larger limits.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-13h (402 Rust tests, 105 pytest, 0 skipped), `39cbb93`; O-14 applied; Q-12 answered | - | - | - | - | - |
| 1 | **Release 0.1.0a1, run A** (checklist; internal hand-out, RHEL 8 install test; ADR-63) | now | nothing that builds while steps 3, 6, 7 run | none (builds, a tag) | - | yes: steps 1, 4, 6 (the owner's earlier build run), 8, 9, 10 |
| 2 | **T-14** hardening (fuzz, damage properties, N-9, README sentence; ADR-61, ADR-62) | now; ideally after run A's steps 3 to 7 (the machine rule) | **nothing** | `fuzz/`, `tools/fuzz_seeds.sh`, core tests, core fixes, `README.md` | **Opus** | no |
| 3 | **T-15** benchmarks | after T-14 is committed and **O-15** | **nothing** | `benches/`, core `Cargo.toml`, root `Cargo.lock`, `t15_*.rs`, `tools/stream_memory.py` | Sonnet | O-15 (the `T_ROWS` export) |
| 4 | **Reviewer session 4** (group: T-13h, T-14, T-15) | after T-15 | - | `docs/review/` | Opus | no |
| 5 | **Release 0.1.0a2, run B** (checklist; PyPI upload, Q-10) | after Reviewer 4 with no blocking finding (and its fixes, if any) | - | version (two lines), tag | - | yes: steps 1, 4, 8, 9, 10 |
| 6 | **T-16** More platforms | after Reviewer session 4 | - | CI configuration, `check_dist.py`, tests | Sonnet | Q-11 (the CI service) |
| 7 | **T-17** synthetic files from the rules | after T-16 | - | core tests | Sonnet | no |

**T-14 and T-15 run one after the other, not in two worktrees** (ADR-62 item 1): the memory rule allows one build at a
time and fuzzing and benchmarks alone, so parallel sessions would wait for each other; both change `Cargo.lock`
(T-15: `criterion`); a fix of T-14 in the row loop would move T-15's baseline. T-14's own files (`fuzz/` with its own
lock file, core tests, core fixes for findings, the README sentence) do not touch `error.rs` or `lib.rs` unless a
finding needs it.

**Reviewer session 4 reads**:
1. **T-13h**: the two deviations and their answers (ADR-61 items 1, 4: the `External error: ` prefix and the code
   pattern; `E-CONFIG` on the iterator after `close()`); the S-1 to S-5 closures against ADR-55 to ADR-59 and the T-13h
   note; every mutation of the note repeated, each alone (s1a, s1b, e1 to e5, f1, n4a to n4c, c1 to c7, d1 to d8); the
   **lock rule** of ADR-61 item 7 (a) to (c), read in `crates/oradmp-py/src/*.rs` from the list of `grep -n "lock()"`;
   the NUL rule (ADR-61 item 3); `tools/release_build.sh` read and run once into a scratch root (its refusal of a root
   under `$HOME` shown), the archives' `/home/` scan repeated by hand.
2. **The four T-12 breaks review 03 did not re-run**: S-3 "old reader.rs", b2, c, g (T-12 note, `implementer-notes.md`).
3. **The checks review 03 did not reach**: release of memory on the Arrow stream path (T-15 (e)'s `tools/stream_memory.py`
   repeated, plus a reading of where the shared state and the batches are dropped); the GIL while pyarrow pulls from the
   capsule (the T-13h two-thread record repeated on a release build); other Python / pyarrow versions (Python 3.11 with
   pyarrow 15.0.2 / pandas 2.2.3 and the newest CPython: the release note of run A, step 7, read, and one run repeated);
   a build and test from the sdist (run A step 7's source build read, or repeated); `cargo deny check advisories` and
   `sources` (the owner's output of step 4 in the release note; the Reviewer needs no network); `auditwheel show` on the
   wheel if available.
4. **T-14**: each target's last 10-minute run repeated for at least one target (the note's limits), every finding with
   its regression test failing on the unfixed code; the damage properties' assertions read for strength; the N-9 exact
   counts each shown to fail by its mutation; the DS `string` test; the README sentence equal to ADR-61 item 2.
5. **T-15**: the benchmark method of arch 4.3 (release build, alone, median of 3, machine facts), the memory test's
   assertion and its child-process measurement, the micro-benchmark inputs.

**Owner actions**: run A's owner steps (1, 4, the result of the earlier `tools/release_build.sh` run or a new one, 8, 9,
10); **O-15** (before T-15: the path of the `T_ROWS` export, and `T_BIG` if made); **Q-10** upload part (run B);
**R-040**; **R-046** (low); **Q-4** (the real dump's database character set id, type codes, partitioning); **Q-11**
(CI service, only for T-16); the install test on RHEL 8 (run A step 8). Next **Analyst session 11**: unchanged
(`open-questions.md` (b), "Inputs for Analyst session 11").

## Order of the next tasks (session 14, history)

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-13g (395 Rust tests, 73 pytest), `3a1bb5f`; Reviewer session 3 (no blocking finding) | - | - | - | - | - |
| 0 | **O-14** owner's edit: `tools/` and `fuzz/` in the Implementer's row of `CLAUDE.md` (ADR-56 item 5) | now | - | `CLAUDE.md` | - | yes (it is the owner's) |
| 1 | **T-13h** review 03 fixes (ADR-55 to ADR-60) | after O-14 | **nothing** | see T-13h "Touches" | **Opus** | no |
| 2 | **Release 0.1.0a1** (checklist, "First release") | after T-13h and Q-12 | T-14, T-15 (the release builds a fixed commit from a clone) | none (builds, a tag) | - | yes: steps 1, 2, 4, 8, 9, 10 (Q-10, Q-12) |
| 3 | **T-14** hardening (with review 03 N-9) | after T-13h is committed (not before: shared `error.rs`, core `lib.rs`, `Cargo.lock`; ADR-60) | T-15, the release | `fuzz/`, core tests, core fixes | Sonnet | no |
| 3 | **T-15** benchmarks | after T-13h is committed | T-14, the release | `benches/`, `docs/design/benchmarks.md` (the Architect copies the numbers in) | Sonnet | the owner's large files |
| 4 | **Reviewer session 4** (group: T-13h, T-14, T-15) | after T-14 and T-15 | - | `docs/review/` | Opus | no |
| 5 | **T-16** More platforms | after Reviewer session 4 | - | CI configuration, `check_dist.py`, tests | Sonnet | Q-11 (the CI service) |

**Can T-14 start before T-13h?** No. T-14's files (`fuzz/`, new core tests, fixes in the core where a target finds
a crash) and T-13h's (the binding, `python/oradmp/`, `tests/`, `tools/`, `t12_messages.rs`) are mostly disjoint, but
both change `oradmp-core`'s `error.rs` and `lib.rs` and `Cargo.lock`, and T-14's no-panic targets are the ground ADR-59's
guard stands on. One session of waiting is cheaper than merging the error enum. T-15 shares only `Cargo.lock`
(`criterion`) with T-14; they may run in parallel as planned since session 12.

**Reviewer session 4 reads**:
1. **T-13h**: the S-1 to S-5 closures against ADR-55 to ADR-59 and the T-13h note; every mutation of the note
   repeated, each alone (s1a, s1b; the `check_dist` rules through `tests/test_check_dist.py` and on a tampered copy of
   a real wheel; d1 to d8; the guard removals of ADR-59 item 6; the N-4 assertions); the `SharedBatches` lock rule
   (no Python call under the shared mutex) read in the code; `tools/release_build.sh` read and run once into a
   scratch root (its refusal of a root under `$HOME` shown), the archives' `/home/` scan repeated by hand.
2. **The T-12 breaks review 03 did not re-run**: S-3 "old reader.rs", b2, c, g (T-12 note, `implementer-notes.md`).
3. **The checks review 03 did not reach**: release of memory on the Arrow stream path (a batch and the stream dropped
   by pyarrow: RSS or an allocation counter, over a loop of reads of one table); the GIL while pyarrow pulls from the
   capsule (the T-13h two-thread record repeated); Python 3.11 with pyarrow 15 / pandas 2.2 and the newest CPython;
   a build and test from the sdist; `auditwheel show` on the wheel if available.
4. **T-14, T-15**: fuzz targets run for their time, every finding with a regression test; the N-9 exact counts each
   shown to fail by a mutation; the benchmark method of arch 4.3.
5. `cargo deny check advisories` / `sources` need the network: the **owner** runs them once, before the release
   (checklist step 4); the Reviewer reads that output in the release note and does not need the network.

**Owner actions**: **O-14** (before T-13h), **Q-12** (release before Reviewer 4 reads T-13h?), **Q-10** upload part,
the checklist steps (network checks, RHEL 8 install, tag), **R-040**, **R-046** (low), **Q-4** (the real dump's
database character set id, type codes, partitioning), **Q-11** (CI service, only for T-16). Next **Analyst session
11**: rules T9 and X2, the R-045 record, the R-040 write-up (`open-questions.md` (b), "Inputs for Analyst session 11").

## Order of the next tasks (session 13, history)

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-13f (395 Rust tests, 69 pytest, `licenses ok`), `3fe33e7` | - | - | - | - | - |
| 0 | **O-13** owner's edit of `CLAUDE.md`, `.claude/agents/*.md`, `docs/prompts/implementer-task.md` (ADR-54 item 6) | now | - | those files | - | yes (it is the owner's) |
| 1 | **T-13g** Rename to `oradmp` | after O-13 | **nothing** (one session, alone) | see T-13g "Touches" | Sonnet | no |
| 2 | **Reviewer session 3** (group: T-11b, T-12, T-13, T-13f, T-13g) | after T-13g | - | `docs/review/` | Opus | no |
| 3 | **Release 0.1.0a1** (checklist, "First release") | after Reviewer session 3 | T-14, T-15 | none (builds and a tag) | - | yes: steps 1, 2, 7, 8, 9 (Q-10 upload part) |
| 3 | T-14, T-15 | after Reviewer session 3 | each other, the release | fuzz targets, benches | - | T-15: the owner's large files |
| 4 | **Reviewer session 4** (group: T-14, T-15) | after both | - | `docs/review/` | - | no |
| 5 | **T-16** More platforms | after Reviewer session 4 | - | CI configuration, `check_dist.py`, tests | Sonnet | Q-11 (the CI service) |

**Reviewer session 3 reads** the session 12 list below (items 1 to 7; every path read with the new names, e.g.
`crates/oradmp-core/tests/t13_interval.rs`, `python/oradmp/`), plus:
8. **T-13g**: (a) no old name, case-insensitive, in the built wheel and sdist (run `tests/check_dist.py` on them and
   on two tampered copies: a member named with the old name, a second old-name line in the sdist's `README.md`; both
   must fail), in the code and tests (the grep of T-13g (c), and `tests/test_project_name.py` failing on a scratch
   file that holds the name), and in the live documents (`CLAUDE.md`: rule 1's sentence only; `.claude/agents/*.md`,
   `docs/prompts/implementer-task.md`: none; `README.md`: the one line of ADR-54 item 2; the body of
   `exp-reader-architecture.md` and the live parts of the plan: none); (b) the moves are renames only (`git log
   --follow` on one test file and one golden file; the golden files byte-identical, `git diff -M` of T-13g (a)); (c)
   **`__version__` is tested**: `tests/test_version.py` exists, fails with a literal `"0.1.0"` on a scratch copy, and
   the fresh-venv check of T-13g (h) is repeated (`import oradmp` works, `import oradump` fails, without installing
   anything named `oradump`); (d) `CONTRIBUTING.md`'s release block run as written in an activated `.venv`.

**Owner actions**: **O-13** (before T-13g), **R-040**, **R-046** (low), **Q-4** (the real dump's database character
set id, type codes, partitioning), **Q-11** (CI service, only for T-16), **Q-10** upload part (upload 0.1.0a1 to PyPI
or keep it internal; checklist step 9). Closed: Q-10 name part (`oradmp`, ADR-54), Q-5 (applied in `0a0fda5`).
Optional: the corpus scripts' `ORADUMP_HOME` and comments, `oradump.code-workspace` (ADR-54 item 5).

## Order of the next tasks (session 12, history)

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-13 (395 Rust tests, 69 pytest), `e2eaae6` | - | - | - | - | - |
| 1 | **T-13f** `deny.toml` targets, version `0.1.0a1`, `check_dist.py`, README install, `.gitattributes` | now | - | see T-13f "Touches" | Sonnet | no |
| 2 | **Reviewer session 3** (group: T-11b, T-12, T-13, T-13f) | after T-13f | - | `docs/review/` | Opus | no |
| 3 | **Release 0.1.0a1** (checklist, "First release") | after Reviewer session 3 | T-14, T-15 | none (builds and a tag) | - | yes: steps 1, 2, 7, 8, 9 (Q-10) |
| 4 | T-14, T-15 | after Reviewer session 3 | each other, the release | fuzz targets, benches | - | T-15: the owner's large files |
| 5 | **Reviewer session 4** (group: T-14, T-15) | after both | - | `docs/review/` | - | no |
| 6 | **T-16** More platforms | after Reviewer session 4 | - | CI configuration, `check_dist.py`, tests | Sonnet | Q-11 (the CI service) |

**Reviewer session 3 reads** (session 12: the session 11 list below, items 1 to 5, plus):
6. **T-13**: `crates/oradmp-core/tests/t13_interval.rs`; the `reader.rs` hunk `Kind::IntervalYmText` /
   `IntervalDsText` (the INTERVAL text path) and the `interval_as` parts of `types/map.rs` and `types/resolve.rs`;
   `tests/test_interval.py`; `tests/test_truth.py` with `conftest.py`: the **26 expected failures** (13 + 13 + 0 of
   T_DATE, T_INTERVAL2, T_NUM, T_TS) checked against `t11_strict.rs` `DOCUMENTED`, `exp_catalog.tsv` and the truth
   files, not only counted; the T-13 mutations m1 to m4 and the `_PRIME` change re-run, each alone on a scratch copy.
7. **Packaging**: `tests/check_dist.py` (run on the built archives and on tampered copies), `deny.toml` with ADR-49's
   `[graph] targets` (and `cargo tree -i tiny-keccak --target all` against the host target), `LICENSE-MIT`,
   `LICENSE-APACHE` (against the apache.org text), `README.md`, `CONTRIBUTING.md`, `.gitattributes` (ADR-52's
   checks), the version (ADR-50), `pyproject.toml`.

**Owner actions**: **R-040**, **R-046** (low), **Q-4** (the real dump's database character set id, type codes,
partitioning), **Q-10** (PyPI: name check and whether to upload 0.1.0a1), **Q-11** (CI service, only for T-16),
**Q-5** (the `CLAUDE.md` / `docs/scope.md` edit, still not applied; text in `open-questions.md`, now with
`.gitattributes`). Q-8 and Q-9 are closed.

## Order of the next tasks (session 11, history)

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-11b (376 tests), T-12 (390 Rust tests, 58 pytest), both in `a19d873` | - | - | - | - | - |
| 1 | **T-13** `interval_as`, Python truth tests, licence files, `deny.toml`, README, `CONTRIBUTING.md`, pinned maturin, `manylinux_2_28` wheel, dist check | now | - | see T-13 "Touches" | Sonnet | Q-8 (placeholder until answered); Q-9 (only if hosts are older than glibc 2.28) |
| 2 | **Reviewer session 3** (group: T-11b, T-12, T-13) | after T-13 | - | `docs/review/` | Opus | no |
| 3 | T-14, T-15 | after Reviewer session 3 | each other | fuzz targets, benches | - | T-15: the owner's large files |
| 4 | **Reviewer session 4** (group: T-14, T-15) | after both | - | `docs/review/` | - | no |

**Reviewer session 3 reads** (session 11 list, replacing the session 10 list):
1. T-11b, T-12, T-13 and their notes; the 8 T-11b test files; the four `crates/oradmp-core/tests/t12_*.rs`
   (`t12_offsets`, `t12_messages`, `t12_survey`, `t12_warning_field`) and `t13_interval.rs`; the production changes
   of T-12 in `error.rs`, `reader.rs`, `survey.rs`; **`crates/oradmp-py/`** (`lib.rs`, `conv.rs`, `errors.rs`,
   `names.rs`, `options.rs`, `digest.rs`); the Python package `python/oradmp/`; **the seven pytest files**
   (`conftest.py`, `test_handover.py`, `test_errors.py`, `test_warnings.py`, `test_threads.py`, `test_survey.py`,
   `test_consumers.py`) and T-13's new ones; `tests/check_dist.py`.
2. The files of ADR-41 items 8, 9: `t11_strict.rs` (`ARCH_7_1_STRICT` against arch 7.1's session 11 table; the
   `DOCUMENTED` T_NUM2 / T_NUM3 edit of T-12), `t11_codes.rs` line 66, and the single union `assert!` of
   `t10_truth.rs::every_table_of_the_exp_files_is_classified_and_read` (review N-1, open by ADR-43 item 5).
3. What sessions 1 and 2 did not read line by line: `t3a_walker.rs`, `t3b_parts.rs` (before line 395),
   `t3c_walker.rs` (after line 400), `t1_walk.rs`, `t2_manifest.rs`, `t3_catalog.rs`, `t5f_followup.rs`,
   `t6_value_parts.rs`, `t8_direct.rs`, `t6l_ru.rs`, `t5_types.rs`, most of `t6_read.rs`, `t6_parts.rs` beyond T_PART2,
   `t7_owner.rs` beyond STC; and **the Analyst's tools** (`docs/formats/tools/`: `exp_parse.py`, `corpus_counts.py`,
   `rowlen_stats.py`, `num_decode.py`, `collist.py` and those the generators import), checked for correctness
   against `exp.md`, the rules and the truth files, not only re-run.
4. **Re-runs**, each alone on a scratch copy: the **12 T-12 breaks** of the T-12 note (d1, d2, d3; S-3 a, S-3 f, S-3
   with the old `reader.rs`; b, b2; e; c; g; h); the T-11b mutations e1, a1, a1b, e3, e4; and the new D5 mutations
   d4, d5, d6 (ADR-46 item 3). Each must fail the test the note names (d6: record the result).
5. Packaging: the licence fields, `deny.toml` and its result, the `check_dist.py` output and both listings, the
   wheel tag (ADR-45, ADR-48); the Q-8 placeholder is reported, not a finding.

**Owner actions** (none blocks T-13 except the final `LICENSE-MIT` line): **R-040**, **R-046** (low), **Q-4**,
**Q-8** (copyright line), **Q-5** (the `CLAUDE.md` / `docs/scope.md` edit, not yet in the repository at `a19d873`;
full text in `open-questions.md` (a)), new **Q-9** (glibc of the ETL hosts; default `manylinux_2_28`).

## Order of the next tasks (session 10, history)

Rules as in session 9: parallel tasks use separate git worktrees whenever their file lists overlap; the one merged
second rebases. Review gate (ADR-42) unchanged.

| order | task | can start | runs in parallel with | touches | model | waits for a human? |
|---|---|---|---|---|---|---|
| done | T-9b (`e308d9a`), T-10b (`7941e40`; 371 tests), Reviewer session 2 (no blocking) | - | - | - | - | - |
| 1 | **T-11b** review-02 test fixes (small, tests only) | now | T-12 | `t10_truth`, `t11_strict`, `t11_options`, `t4_values`, `t3b_parts`, `t9_charset`, `t9b_fixes`, `common/truth_checks`, `common/mod` (additive) | Sonnet | no |
| 1 | **T-12** Python binding; S-1, S-3, `Cow` messages, `StrictError`, new public fields, NUMBER profile 5.2b, pytest files | now | T-11b | `crates/oradump-py`, `python/`, `pyproject.toml`, `error.rs`, `reader.rs`, `survey.rs`, `t6_patched`, new `t12_*.rs`, `common/mod` (additive), pytest files under `tests/` | **Opus** | no |
| 2 | T-13 packaging, licence files, `deny.toml`, README, `CONTRIBUTING.md`, Python truth tests | after T-12 (and T-11b) merged | - | packaging, root files, `tests/` (pytest) | Sonnet | Q-8 (one copyright line) for `LICENSE-MIT` only |
| 3 | **Reviewer session 3** (group: T-11b, T-12, T-13) | after T-13 | - | `docs/review/` | Opus | no |
| 4 | T-14, T-15 | after Reviewer session 3 | each other | fuzz targets, benches | - | T-15: the owner's large files |
| 5 | **Reviewer session 4** (group: T-14, T-15) | after both | - | `docs/review/` | - | no |

Overlap check (session 10): T-11b and T-12 share only `tests/common/mod.rs`, where both only add helpers;
`t6_patched.rs` is given to T-12 and `t9b_fixes.rs` to T-11b, so no test file is edited by both. T-11b merges first
(smaller); T-12 rebases and fixes any `Cow`-caused compile error in a T-11b file after the rebase. T-11b belongs to
the group of Reviewer session 3, not to a review of its own.

**Reviewer session 3 reads**, besides T-11b, T-12, T-13 and their notes, what sessions 1 and 2 did not read line by
line: `t3a_walker.rs`, `t3b_parts.rs` (before line 395), `t3c_walker.rs` (after line 400), `t1_walk.rs`,
`t2_manifest.rs`, `t3_catalog.rs`, `t5f_followup.rs`, `t6_value_parts.rs`, `t8_direct.rs`, `t6l_ru.rs`,
`t5_types.rs`, most of `t6_read.rs`, `t6_parts.rs` beyond T_PART2, `t7_owner.rs` beyond STC; the pytest files; and
**the Analyst's tools themselves** (`docs/formats/tools/`: `exp_parse.py`, `corpus_counts.py`, `rowlen_stats.py`,
`num_decode.py`, `collist.py` and the ones the generators import), checked for correctness against `exp.md`, the
rules and the truth files, not only re-run. It repeats the mutations d1, d2, d3, e1, a1, e3 / e4 and the S-3
offsets; it checks the licence fields, `deny.toml`, the sdist and wheel listings of T-13 (ADR-45).

**Owner actions** (none blocks T-11b or T-12): **R-040** (`sqlldr` exists in the 18c container; the Analyst writes
it up); **R-046** (low); **Q-4** (the real dump's database character set id, type codes, partitioning); **Q-5**
(the owner edits `CLAUDE.md` and `docs/scope.md`; text in `open-questions.md` (a), session 10 adds the root files);
**Q-8** (the copyright line of `LICENSE-MIT`). Q-6 is closed (ADR-45). Analyst session input: rules T9 (386 lines,
1,907 definitions) and X2 (third failed export), R-045 record, R-040 write-up.

## Order of the next tasks (session 9, history)

Rules: tasks that run in parallel use separate git worktrees whenever their file lists overlap; the one
merged second rebases. **Review gate (ADR-42)**: after every group of tasks that adds or changes tests over
the corpus, a Reviewer session runs before the next group starts; a blocking finding stops the next group,
should-fix findings become a task of the next group.

| order | task | can start | runs in parallel with | touches | waits for a human? |
|---|---|---|---|---|---|
| done | T-6M, T-3d, T-11 (347 tests) | - | - | - | - |
| 1 | **T-9b** CLOB mismatch, chain offset (small) | now | T-10b | `reader.rs`, new `tests/t9b_fixes.rs` | no |
| 1 | **T-10b** truth checks, review S-1 to S-6 (one or two sessions) | now | T-9b | `tests/t10_truth.rs`, `tests/common/`, truth parts of `t4`, `t5_types`, `t6_read`, `t6_parts`, `t6l_lob`, `t6l_ru`, `t7_owner`, `t9_charset`; a comment in `tests/data/gen_walk_expected.py` | no |
| 2 | **Reviewer session 2** (group 1: T-11, T-9b, T-10b) | after T-9b and T-10b merged | - | `docs/review/` | no |
| 3 | T-12 Python binding | after Reviewer session 2 | - | `crates/oradump-py`, `python/`, `Cow` in `error.rs` | no |
| 4 | T-13 Python truth tests and packaging | after T-12 | - | `tests/` (pytest), packaging | **yes: Q-6 (licence)** for the release, not for the tests |
| 5 | **Reviewer session 3** (group 2: T-12, T-13) | after T-13 | - | `docs/review/` | no |
| 6 | T-14, T-15 | after Reviewer session 3 | each other | fuzz targets, benches | T-15: the owner's large files |
| 7 | **Reviewer session 4** (group 3: T-14, T-15) | after both | - | `docs/review/` | no |

Overlap check (session 9): T-11 is merged, so the question "T-10b beside T-11" is closed. T-9b and T-10b share
no file: T-9b adds its own test file and uses existing helpers only; T-10b touches no production code. If T-9b
must add a helper, it waits for T-10b (both would touch `tests/common/`).

**Reviewer session 2 reads** (besides T-11's three test files, T-9b and T-10b, and the mutation list of T-10b
(b9), which it repeats): what session 1 did not: `t6l_lob.rs` (T_LOB, T_LOB2, T_LOB3 against `truth_lob*.txt`),
`t9_charset.rs` (whole), `t4_values.rs`, `t3a_walker.rs`, `t3b_parts.rs`, `t3c_walker.rs`, `t6_patched.rs`; the
header columns of `walk_expected.tsv` against `header_fields.py`; N-21's claim that the `?` of `own_exp_w1252` is
in a COMMENT item; whether every `ErrorCode` / `WarningCode` test of `t11_codes.rs` asserts code and offset.

**Owner actions** (none blocks group 1): **R-040** (`sqlldr` exists in the 18c container: ready to run; the
Analyst writes it up); **R-046** (low; `r_lob3` in `ora11ru`); **Q-4** (the real dump's database character set
id); **Q-5** (the owner edits `CLAUDE.md` and `docs/scope.md`; the ownership text of ADR-42 is in
`open-questions.md` (a)); **Q-6** (licence; first needed by the T-13 release). Analyst session input: rules T9 and
X2 (ADR-38 item 7), R-045 record, R-040 write-up.

## Order of the next tasks (session 8, history)

Rule kept: tasks that run in parallel use separate git worktrees whenever their file lists overlap; the
one merged second rebases.

| order | task | can start | runs in parallel with | touches | waits for a human? |
|---|---|---|---|---|---|
| done | T-9, T-10 (merged; 255 tests, 11 failing on the 201-file corpus) | - | - | - | - |
| 1 | **T-6M** set 171 confirmed (ADR-25 session 8) | now | nothing (T-3d needs it; T-11 shares `reader.rs`) | `charset.rs`, `reader.rs`, `tests/t6l_ru.rs`, comments in `t3a_walker.rs`, `t6l_lob.rs` | no |
| 2 | **T-3d** corpus 201 | after T-6M merged | nothing (T-11 shares the corpus tests) | regenerated `tests/golden/*`, `tests/data/*`; `t1`, `t2`, `t3`, `t3a`, `t5`, `t6_value_parts`, `t10`; `tests/common/` | no |
| 3 | **T-11** strict, per-column policy, `max_digits_dropped`, LOB / TSLTZ strict cases (178 copy), survey counting core | after T-3d merged | T-10b | `error.rs`, `reader.rs`, `dump.rs`, `exp::*`, `types/resolve.rs`, survey module, `tests/t11_*.rs` | no |
| 3 | **T-10b** T-10 leftovers (N-22) | after T-3d merged | T-11 | `tests/t10_truth.rs`, `tests/common/` | no |
| 4 | T-12 Python binding | after T-11 and T-10b | - | `crates/oradump-py`, `python/` (with the Python API agent) | no |
| 5 | T-13 Python truth tests and packaging | after T-12 | - | `tests/`, packaging | **yes: Q-6 (licence)** |
| 6 | T-14, T-15 | after T-11 | each other, T-12 | fuzz targets, benches | T-15: the owner's large files (data, not a decision) |

Overlaps: T-6M / T-11 share `reader.rs`; T-3d / T-11 share the tests that read the corpus (and T-11's
fixture-wide `strict` test needs T-3d's expected list): strictly sequential. T-11 / T-10b share only
`tests/common/` (worktrees).

**Owner actions** (not decisions; none blocks a task): **R-046** (proposed by the Analyst, low: the CLOB / NCLOB
short-form end in the CL8MSWIN1251 database; can share R-043's session); **R-040** (the owner reports whether
`sqlldr` exists in the 18c container); **R-045** delivered (`truth_r045.txt`, 18c, 2026-10-17: ORA-14085 for
LONG and LONG RAW in a partitioned table and for `ALTER TABLE ... ADD (l LONG)`): the owner's part is done,
the Analyst still records it (`requests.md` says "proposed"); the ADR-31 / ADR-32 exemption stays and stays
unit-tested. **Q-4** now decides whether the stop of ADR-25 matters at all (open-questions (a)). First task
that needs an owner decision: T-13 (Q-6).

## Order of the next tasks (session 7, history)

Rule kept: tasks that run in parallel use separate git worktrees whenever their file lists overlap; the
one merged second rebases.

| order | task | can start | runs in parallel with | touches | waits for a human? |
|---|---|---|---|---|---|
| done | T-5f, T-6L, T-7, T-8 | - | - | - | - (232 tests) |
| 1 | **T-9** character sets (+ CLOB `W-CHARSET-MISMATCH`, `#[doc(hidden)]` counters) | now | T-10 | `charset.rs`, `reader.rs`, `dump.rs`, `types/map.rs`, (`exp/header.rs`), `tests/t9_charset.rs`, `tests/common/` | no |
| 1 | **T-10** full truth check, step (1) now, step (2) after T-9 merges | now | T-9; later T-11 | `tests/t10_truth.rs`, `tests/common/`; production only on a failing test (note) | no (R-038 values join step (2) if they arrive) |
| 2 | **T-11** strict, per-column policy, `max_digits_dropped`, LOB / TSLTZ strict cases, survey counting core | after T-9 merged | T-10 (tail) | `error.rs`, `reader.rs`, `dump.rs`, `exp::*`, `types/resolve.rs`, survey module, `tests/t11_*.rs` | no |
| 3 | T-12 Python binding | after T-11 | - | `crates/oradump-py`, `python/` (with the Python API agent) | no |
| 4 | T-13 Python truth tests and packaging | after T-12 | - | `tests/`, packaging | **yes: Q-6 (licence)** |
| 5 | T-14, T-15 | after T-11 | each other, T-12 | fuzz targets, benches | T-15: the owner's large files (data, not a decision) |

Overlaps: T-9 / T-10 share only `tests/common/` (worktrees; T-9 merges first). T-10 / T-11 share
`tests/common/` and, if T-10 finds a bug, production files (worktrees; T-10 merges first, then T-11's
fixture-wide `strict` test is written against T-10's expected list). T-9 / T-11 share `reader.rs`,
`dump.rs`, `types`: **not** in parallel.

**First task whose test needs a human decision: T-13** (Q-6, the licence in the wheel metadata and README).
No earlier task waits for the owner: ADR-34 / ADR-35 decide everything N-19 / N-20 asked. Human actions
that change tests but are not decisions: R-038 (the fixture script exists and waits for the owner; when
delivered, only `lob_text_kind`, the unconfirmed check and the tests in `t6l_ru.rs` change, plus its
values in T-10 step (2)), R-042 / R-045, the owner's large files for T-15. Q-4 and Q-6 stay open.

## Order of the next tasks (session 6, history)

| order | task | can start | runs in parallel with | touches | waits for a human? |
|---|---|---|---|---|---|
| 1 | **T-5f** follow-up + session 6 fixes (`type_source`, `rows` / `part_rows`, flush on error, `W-TSLTZ`) | now | T-7, T-8 | `types::*`, `dump.rs`, `reader.rs` | no |
| 1 | T-7 owner mode (tests) | now | T-5f, T-6L, T-8 | `tests/t7_owner.rs` | no |
| 1 | T-8 direct path (tests) | now | T-5f, T-6L, T-7 | `tests/t8_direct.rs` | no |
| 2 | **T-6L** LOB values (hook at `step_with` / `lob_value`; ADR-25 by the header set id) | after T-5f | T-7, T-8 | `exp/rows.rs`, `reader.rs`, `arrow_out.rs`, `charset.rs` | no (R-038, if it lands, feeds T-10) |
| 3 | T-9 character sets | after T-6L | T-7, T-8 if open | `charset.rs`, `reader.rs`, `dump.rs`, `types/map.rs` | no |
| 4 | T-10 full truth check | after T-6L, T-7, T-8, T-9 | - | tests | no |
| 5 | T-11 strict for every new warning, per-column policy, `max_digits_dropped`, survey counting core | after T-10 | - | `error.rs`, `reader.rs`, `exp::*`, `types` | no |
| 6 | T-12 Python binding | after T-11 | - | `crates/oradump-py`, `python/` (with the Python API agent) | no |
| 7 | T-13 Python truth tests and packaging | after T-12 | - | `tests/`, packaging | **yes: Q-6 (licence)** for the wheel metadata and README |
| 8 | T-14, T-15 | after T-11 | each other | fuzz targets, benches | T-15: the owner's large files (data, not a decision) |

**First task whose test needs a human decision: T-13** (Q-6, the licence in the wheel metadata). No task
before it waits for the owner: Q-1, Q-3, Q-7 are decided; ADR-25 makes T-6L's single-byte LOB tests
deterministic without R-038. Human actions that change tests but are not decisions: R-038 (being prepared;
its values join T-10), R-042 / R-045 (a partitioned LOB table; the ORA-14085 record), the owner's large
files for T-15. Q-4 and Q-6 stay open; Q-4 orders the requests, it blocks no task.

## Order of the next tasks (session 5, history)

| order | task | can start | runs in parallel with | waits for an owner decision? |
|---|---|---|---|---|
| 1 | **T-3b** catalog with parts, per-part matching (ADR-31), `part` in errors / warnings, golden files for 199 files | now | T-6, T-5 follow-up | no |
| 1 | **T-6** Arrow output, file order to declared order, value warnings | now (split items after T-3b) | T-3b | no (Q-1 decided) |
| 1 | T-5 follow-up (ADR-30 items 1, 3; `W-TSLTZ`) | now; small, may be T-6's first step | T-3b | no |
| 2 | **T-6L** LOB values (ADR-25), T-7, T-8, T-9 | after T-6 and T-3b | each other | T-6L: no; R-038 (being generated) may turn `E-LOB-UNCONFIRMED` into a confirmed decoding |
| 3 | T-10, then T-11 (strict for the new warnings, `round` under `strict`, survey counting core) | after 2 | - | no |
| 4 | T-12, T-13 | after T-11 | - | no (Q-3 decided) |
| 5 | T-14, T-15 | after T-11 | each other | T-15: the owner's large files |

T-3b and T-6 meet in the interface written in T-6 ("Parallel with T-3b"): `PartInfo` with `file_order`,
and `part` in `Warning` and the error type. Whoever lands second adapts to the other's names; neither
changes the seam of arch 2.3. T-3b touches `exp::*` (scan), `dump` (catalog) and `tests/golden`; T-6
touches `arrow_out`, the value glue and the reader in `dump`, so the two meet in `dump.rs`: T-3b owns
`TableInfo` / `PartInfo`, T-6 owns `TableBatches`. Q-4 and Q-6 block nothing before T-13.

## Order of the next tasks (session 4, history)

| order | task | can start | runs in parallel with | waits for an owner decision? |
|---|---|---|---|---|
| 1 | **T-3c** walker of v3.4 (chains, composite partitions) | now | T-4, T-5 | no |
| 1 | **T-4** value decoders (NUMBER incl. Decimal256, intervals, ROWID / UROWID) | now | T-3c, T-5 | no: builds every policy and rendering |
| 1 | **T-5** declared types, by-name matching, declared order | now (all-lines test complete after T-3c) | T-3c, T-4 | no: the default type is a configuration value |
| 2 | **T-3b** catalog with parts, golden files for 199 files | after T-3c | T-4, T-5 | no |
| 3 | T-6 Arrow output (Large types for LONG, Decimal256 builder, schema order) | after T-3b, T-4, T-5 | - | no |
| 4 | **T-6L** LOB values (ADR-25), T-7, T-8, T-9 | after T-6 | each other (separate sessions) | T-6L: no, but R-038 decides whether ADR-25's error stays |
| 5 | T-10, then T-11 (with the survey's counting core) | after 4 | - | **T-11: Q-1** (default of `on_number_overflow`; Q-7 confirms the `unconfirmed_lob` default) |
| 6 | T-12, T-13 | after T-11 | - | **Q-3** (Python version) |
| 7 | T-14, T-15 | after T-11 | each other | T-15: the owner's large files |

Three sessions can run at once now (T-3c, T-4, T-5): they touch `exp::*`, `values::{number, interval,
rowid}` and `values::declared` respectively, and meet only in the error type (exists) and in T-5's
all-lines test, which takes its expected counts from `tools/collist.py` for the files the walker reaches.
Q-1 blocks nothing before T-11. R-038 should be generated before T-6L ends if Q-4 shows a single-byte
database with CLOB / NCLOB columns.

## Order of the next tasks (session 3, history)

By value for the alpha (a LOB or partitioned table blocks the rest of a real file; interval and ROWID
columns are rarer but cost little once the walker knows them):

1. **T-3a** (walker of v3.3: LONG, LOB framing, new codes, partitions, [E2a], header checks), then
   **T-3b** (catalog with parts, golden files for 177 files); then T-3 is accepted.
2. T-4 (with interval and ROWID decoders) and T-5 (with the session 8 type texts), independent.
3. T-6 (Large, interval, duration builders).
4. **T-6L** (LOB values) and T-7, T-8, T-9 (any order; T-6L first if Q-4 shows LOB columns).
5. T-10; T-11 (with the survey's counting core); T-12 and T-13 when Q-3 is answered; T-14, T-15.
