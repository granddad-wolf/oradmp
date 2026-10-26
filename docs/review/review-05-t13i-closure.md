# Review 05: closure of T-13i and the 0.1.0a2 release build

Reviewer session 5, 2026-10-21. Prompt: `docs/prompts/review-05-t13i-closure.md` (untracked, not touched).

At the start (and unchanged at the end):

    $ git rev-parse HEAD
    dc7668e72e002cc54449659d69e580c2553dac33
    $ git status --short
     M docs/design/benchmarks.md
     M docs/design/decisions.md
     M docs/design/implementation-plan.md
     M docs/design/implementer-notes.md
     M docs/design/open-questions.md
    ?? docs/prompts/architect-17-update.md
    ?? docs/prompts/review-05-t13i-closure.md

**Reviewed: HEAD `dc7668e`** ("Version 0.1.0-alpha.2", on top of `469416d`, T-13i). The working tree holds
uncommitted Architect session 17 edits under `docs/design/` (ADR-69, amendments) and two untracked prompts; none
of them is in either archive. I read the session 17 text from the working tree; every build and test ran on
clean clones of `dc7668e`.

Method. All work under `/var/tmp/rev05`: `src` (clean `git clone --no-local` of HEAD: checks, release build
source), `mut` (second clone for the mutations, restored with `git checkout -- .` and `git clean` after each; `git
status` clean after every one), `fuzzclone` (fresh clone, no `fuzz/corpus/`), own Cargo target folders, own venvs,
`TMPDIR=/var/tmp/rev05/tmp` (`/tmp` was 98 % full). `CARGO_BUILD_JOBS=2`, `cargo nextest run -j 2`, one build at a
time; the fuzz runs alone. Each mutation was applied alone by a script (exact text, one occurrence asserted).
Binding mutations: `maturin develop --features test-probes` in `mut`, full `pytest`. Core mutations: full `cargo
nextest run -p oradmp-core`. `check_dist.py` mutations: `pytest tests/test_check_dist.py`. No `uv run` in the
repository root; `python/oradmp/_core.abi3.so` of the repository untouched (mtime 16:32, before this session).
Nothing read from `corpus/work/`.

## Summary

- **Blocking: none. No blocking findings, safe to release** (the build of the candidate commit, by the owner, per
  the checklist; see N-6 for why my hashes are not the release hashes).
- **Should fix: 3.** SF-1: the iterator half of the S-2 test does not check `r.warnings` / `r.rows_read` (a
  mutation survives). SF-2: the README's "pip installs it only when asked for by version" is false while 0.1.0a2 is
  the only release, and it goes into the PyPI page of a version that cannot be re-uploaded. SF-3: seven more
  `check_dist.py` rules have no test (ADR-64's "every `check_dist.py` rule" is not met).
- Every mutation of ADR-64 / ADR-65 that I repeated fails the named test: s1e, s1f, m1 to m9 (+ m3b), premise, d9,
  d10, a1, a2, c2; plus own variants. Survivors: s1o (SF-1), seven `check_dist.py` rules (SF-3).
- Release build `tools/release_build.sh dc7668e --twice`: `check_dist: ok` twice, **wheel and sdist byte-identical**,
  0 `/home/`, `/root/`, `/Users/`, `C:\Users` anywhere, version `0.1.0a2` in names and METADATA / PKG-INFO, no
  `_probe_panic` in the `.so` or the wheel.
- Installed wheel, full pytest: Python 3.11.16 + pyarrow 15.0.2 and Python 3.14.7 + pyarrow 26.0.0: **114 passed,
  3 skipped** each (the three probe tests).
- Full checks green; counts equal the T-13i note: **411 passed, 3 skipped** (nextest), **117 passed, 0 skipped**
  (pytest, dev build with probes).

## Blocking

None.

## Should fix

### SF-1. The iterator S-2 test does not assert `r.warnings` or `r.rows_read` (ADR-64 item 2: "on each")

`tests/test_stream.py:176-190` (`test_the_warnings_are_complete_after_an_error_on_the_iterator_path`) checks the
exception (code, offset, row) and the **codes** of the issued warnings only. The helper that checks the reader's
warning list and rows, `check_row_17_rounded_before_row_19` (`:169`), is called by the stream test only (`:202`).
ADR-64 item 2 asks for "on each [path], `r.warnings` is exactly one `W-NUMBER-ROUNDED` with count 1, offset 8948,
row 17, column `V`; `r.rows_read` equals ...".

Shown: mutation **s1o** in `crates/oradmp-py/src/lib.rs` `pull_raw`, after `st.warnings = src.warnings();`:
`if r.is_err() && !from_stream { for w in st.warnings.iter_mut() { w.offset = w.offset.map(|o| o + 1); } }` (the
iterator path's warnings carry a wrong offset after an error): **117 passed, nothing fails**. The same mutation on the
stream path (control) fails `test_the_warnings_are_complete_after_an_error_on_the_stream_path`. (A rows mutation,
`st.rows` copied only on `Ok`, is caught, by `test_warnings.py::test_an_error_ends_the_iteration_...` and two stream
tests, not by the new iterator test.)

Fix: call `check_row_17_rounded_before_row_19(reader)` in the iterator test after the `pytest.raises` block. A test
file only; does not change any shipped file, so it can wait until after run B.

### SF-2. README: "a pre-release: pip installs it only when asked for by version" is false for a first release

`README.md:8`. pip and uv install a pre-release without a version pin when no final release exists. Shown locally,
without network: fresh Python 3.11 venv with pip 26.2.2, `pip install --no-index --no-deps --find-links
<release dist> oradmp` (no version) installed **0.1.0a2**. (The same finder runs for an index; I did not test it
against PyPI.) The README is the wheel's METADATA description, so this sentence becomes the PyPI page of 0.1.0a2,
and that file cannot be replaced.

Not blocking (no user is harmed: a plain `pip install oradmp` gets the alpha, which is all there is). Owner's choice
before step 6 of run B: (a) accept it and fix in a3; or (b) change the sentence (e.g. "Status: alpha, version
0.1.0a2. Pin the version when you install it.") in a commit before the build; that commit touches a shipped file,
so the gate's "`git diff --stat <T-13i version commit> <candidate>` lists only `docs/`" no longer holds and the
Architect decides whether a short re-check is needed (the change is one README line; `check_dist.py` and
`test_project_name.py` read the README).

### SF-3. Seven more `check_dist.py` rules can be removed without a test failing

ADR-64 "Consequences" says "every `check_dist.py` rule has a test that a one-line mutation fails". m1 to m9 now do
(table below). These do not, each removed alone (`if False:` or line deleted), `pytest tests/test_check_dist.py`
**26 passed** every time:

| rule | `tests/check_dist.py` |
|---|---|
| sdist file name `oradmp-<v>.tar.gz` | `:186` |
| old-name scan of sdist member contents (and its allowed line in README / PKG-INFO) | `:209` |
| `Requires-Dist` holds `pyarrow>=15` | `:122` |
| `License-Expression` | `:116` |
| `Requires-Python` | `:120` |
| the sdist's required files (`Cargo.lock`, `crates/.../lib.rs`, ...) | `:200` |
| both licence files under `.dist-info/licenses/` | `:164` |

Not tried, same kind by reading: `Metadata-Version >= 2.4`, `License-File` lines, "description is the README",
`License ::` classifier, METADATA / PKG-INFO missing. For this release the real archives pass `check_dist` and I
read their METADATA and PKG-INFO by hand (`Version: 0.1.0a2`, `Requires-Dist: pyarrow>=15`, `Requires-Python:
>=3.11`, `License-Expression: MIT OR Apache-2.0`, both `License-File`, description = README), so nothing is wrong
in the build. Fix (after run B): one synthetic case each in the style of m1 to m9, or reword the ADR's claim.

## 1. T-13i items, mutations repeated alone

### S-1 (iterator after `close()`)

`test_stream.py:133`: T_PART of `18c/part_exp_18c.dmp`, `batch_rows=1000`; one batch, `close()`, `next()` and
`read_all()` raise `UsageError` `E-CONFIG` "the reader is closed", `rows_read == 1000`; a second reader read to
the end, `close()`: `StopIteration`, `read_all()` 0 rows with the schema. As ADR-64 item 1.

### S-2 (the patched `T_NUM` copy): bytes checked against the fixture and the rules

- Fixture SHA-256 `3df4a6c5...8bb275` = `corpus/fixtures/18c/SHA256SUMS`; `sha256sum -c` of all three folders ok.
- `xxd -s 0x22e8`: `...02 00 c1 12 | 05 00 c2 02 18 2e 3d...`: the V length `05 00` at 0x22F2, value at 0x22F4 to
  0x22F8. `exp_parse.py --rows` (Analyst's tool): `row 17 0x22ee: @22ee [2] c1 12 | @22f2 [5] c2 02 18 2e 3d`
  (`@` marks the length field); row 19 `@230d [2] d1 0b` (value at 0x230F). `truth_num.txt` line 17: `17|123.456|Typ=2
  Len=5: c2,2,18,2e,3d`.
- `num_decode.decode`: `c202182e3d` = 123.4560; patched `bc02182e3d` = 1.234560E-10 (rule Y1: exponent byte 0xBC,
  E = -5); `encode` of it gives back `bc02182e3d` (canonical); `d10b` = 10^33. 0x22F4 = 8948.
- The test (`:162-166`) asserts the 7 bytes before patching and changes one byte. Expected values come from the
  fixture, the tools and ADR-64, not from the reader.

ADR-69 item 1 / review item (7): the stream test asserts nothing issued before the end of the `with` and one
`W-NUMBER-ROUNDED` after it; the iterator test asserts it issued when the error ends the iteration and not again
at the end of the `with`. Both match the ADR-64 amendment. (Gap: SF-1.)

### Mutation table

| id | mutation | result |
|---|---|---|
| s1f | `return if st.finished \|\| !from_stream {` | fails `test_the_iterator_after_close_..._normal_end_after_it` (1 failed, 116 passed) |
| s1e | warnings copied only when the pull is `Ok` | fails both S-2 tests (2 failed) |
| s1r (own) | rows copied only when the pull is `Ok` | fails `test_an_error_inside_the_stream_...`, the stream S-2 test, `test_warnings.py::test_an_error_ends_the_iteration_...` |
| s1i (own) | warnings not copied on an iterator error (stream intact) | fails the iterator S-2 test |
| **s1o** (own) | iterator-path warnings with offset + 1 after an error | **none (117 passed)**: SF-1 |
| m1 | wheel version vs `Cargo.toml` off | fails `test_m1_...` only |
| m2 | METADATA `Version` check off | fails `test_m2_...` only |
| m3a | wheel member old-name scan removed | fails `test_m3_...` only |
| m3b | `allow = True` | fails `test_m3_...` only |
| m4 / m5 / m6 | `.dmp` / `truth_*.txt` / `FORBIDDEN_NAMES` off | fails `test_m4_...` / `test_m5_...` / `test_m6_...` only |
| m7 | "outside oradmp/" off | fails `test_m7_...` only |
| m8 | sdist root folder rule off | fails `test_m8_...` only |
| m9 | `__pycache__` removed from `FORBIDDEN_COMPONENTS` | fails `test_m9_...` only |
| 7 more rules | SF-3 table | **none** |
| premise | `in_piece` prints `n + 100000` | fails `in_piece_appends_the_exact_suffix`, `no_message_of_the_fixtures_holds_a_value`, `t6_patched::a_broken_utf8_...`, both `t12_offsets` tests (5) |
| piece20 (own) | `in_piece` prints `{n}0` ("piece 20") | the same 5 (the exact-suffix assertions of S-5 (b)) |
| value before suffix | `" Lxxxxxxxxxxxxxxx"` (start of a T_LONG value of the same file) before the suffix | fails `no_message_...` ({Text}) and the unit test |
| value before suffix, other file (own) | `" plain ascii"` (a T_TEXT value of another fixture) | only the unit test: correct, the check compares each message with the values of its own table (N-10) |
| d9 | `15.06.-100` appended to `E-BAD-DATETIME` | fails `no_message_...` ({Decoded}: "year below 1 has no Arrow value 15.06.-100"), `the_check_reports_a_decoded_value` |
| d9 own | DD.MM.YYYY computed from the failing field's own bytes, every 7-byte field | same 2 |
| d10 | `1,000,000,000,000,000,000,000,000,000,000,000` appended to `E-NUMBER-OVERFLOW` | same 2 |
| d10 own | the failing NUMBER's own `to_text()` grouped by 3 with `,` | same 2 |
| a1 | `rows.rs` "row or end of rows" `.at(at + 3)` | fails `each_truncation_site_names_its_offset`, `truncation_gives_ok_or_a_typed_error` (default 256 cases) |
| a2 | `framing.rs:109` `.at(at + 2)` | fails `framing::tests::read_unlisted_field_at_the_end_of_the_input_is_truncated` |
| c2 | panic (index into an empty `Vec`) before `framing.rs:109` | the same unit test |
| line_start (own) | `io.rs` "inside a line" at the line start | fails `each_truncation_site_names_its_offset` |

Damage-test offsets checked by hand against `exp_parse.py --rows` and `xxd`: end marker `ff ff` at 0x2455; EXIT
lines at 0x254d and 0x2552 (`xxd -s 0x2545`: `0a 45 58 49 54 0a 45 58 49 54 0a`); row 17 at 0x22ee, the ID value at
0x22f0, V length at 0x22f2, V value at 0x22f4; `lob_exp_18c.dmp` `fc ff` at 0x4358 and the 8-byte total `40 0d 03 00
00 00 00 00` (200,000) at 0x435a. All ten cases agree. The offsets are the reader's convention (the position where
the failed read stood), not a rule of the rules file, which has none for truncation offsets (N-3).

## 2. Panic guard (ADR-59 item 2 as amended by ADR-65 item 1)

`crates/oradmp-py/src/lib.rs` (688 lines), `options.rs`, `errors.rs`, `conv.rs`, `names.rs`, `digest.rs` read.

| call into the core | where | guarded |
|---|---|---|
| `dump::open` | `lib.rs:651` `open` | yes, `detach(catch_panic)` |
| `Dump::scan`, `catalog()` for the names | `:94-97` `scan_inner` -> `with` | yes |
| `Dump::read_table` | `:191-203` -> `with` | yes |
| `survey_data`: `tables()`, `count_table`, `read_table`, `next_batch`, `rows_read`, `warnings` | `:215` -> `with` | yes |
| `number_summary` | `:259-261` | **yes (new)**, `detach(catch_panic)` |
| `digest`: `read_table`, `schema`, `next_batch` | `:279-291` -> `with` | yes |
| `TableBatches::next_batch`, `rows_read`, `warnings` | `pull_raw` from `next_batch` `:513` and `SharedBatches::next` `:461` | yes |
| `NumberType::from_str` | `options.rs:21` (both call sites `:103`, `:143`) | **yes (new)** |
| `Dump::info()` + clone | `:141` | no: exempt by name |
| `Dump::catalog()` + `copy_catalog` | `:152-157` | no: exempt |
| `Dump::warnings()` | `:170` | no: exempt |
| `TableBatches::schema()` | `:205` | no: exempt |
| `Error`'s `Display` in `to_pyerr` | `errors.rs:35` | no: exempt |
| drops in both `close()` | `:300`, `:579` | no: exempt |
| `core_open` of `_probe_panic` | `:618` (feature `test-probes`) | no: exempt |
| `Warning`'s `Display` in `warning_dict` | `errors.rs:99` | **no, not in the list** (N-1) |
| `Error`'s `Display` in `error_instance` (survey and catalog reports) | `errors.rs:51` -> `:35` | **no, not in the list** (N-1) |
| `Charset::from_id` / `Charset::name` (header ids) | `conv.rs:20` | **no, not in the list** (N-1) |
| `ErrorCode::as_str`, `WarningCode::as_str`, `ALL` | errors, module init | no, not in the list (constants) |
| implicit drops of `PyReader` / `PyDump` (garbage collection, not `close()`) | - | no, not in the list |

Every listed exemption and both new wraps match ADR-65 item 1 and the `lib.rs` module comment. The unlisted ones are
panic-free by reading (a `match` on a `u16`; `render` of literals and integers, as `Error`'s); N-1.

## 3. Release build

`ORADMP_BUILD_ROOT=/var/tmp/rev05/release tools/release_build.sh dc7668e72e002cc54449659d69e580c2553dac33 --twice`
from the clean clone: exit 0, 4 min 32 s. Printed: commit `dc7668e7...`, rustc 1.99.0, maturin 1.15.0, zig 0.17.0
(`ziglang==0.17.0` installed, ADR-65 item 8), epoch 1792618501. `check_dist: ok` (both builds). `--twice`:
**"identical" wheel, "identical" sdist, "reproducible"**.

    9ed0ce75080753dd023649e75fa0bc141da3fd407005841103f30ad7f166945b  oradmp-0.1.0a2-cp311-abi3-manylinux_2_28_x86_64.whl  (3,625,341 bytes)
    4fd52cf74844c3562a81b10a21ed409320b468386c8e196595783867b55408c2  oradmp-0.1.0a2.tar.gz  (155,476 bytes)

These belong to `dc7668e` built under `/var/tmp/rev05/release`; they are **not** the hashes to upload (N-6).

| marker | `.so` (`strings -a`) | SBOM | METADATA | RECORD | every wheel member | every sdist member |
|---|---|---|---|---|---|---|
| `/home/` | 0 | 0 | 0 | 0 | 0 | 0 |
| `/root/` | 0 | 0 | 0 | 0 | 0 | 0 |
| `/Users/` | 0 | 0 | 0 | 0 | 0 | 0 |
| `C:\Users` (any case, also escaped) | 0 | 0 | 0 | 0 | 0 | 0 |
| `_probe_panic` | 0 | 0 | 0 | 0 | 0 (the stub no longer has it) | 6 (source: `oradmp-py/Cargo.toml` 1, `lib.rs` 5) |
| `/cargo/` (remapped) | 161 | 0 | 0 | 0 | 161 | 0 |

Version: file names `oradmp-0.1.0a2-...whl` / `oradmp-0.1.0a2.tar.gz`, `Version: 0.1.0a2` in METADATA and PKG-INFO,
sdist root `oradmp-0.1.0a2/`. Wheel tag `cp311-abi3-manylinux_2_28_x86_64`; highest GLIBC symbol of the `.so`
`GLIBC_2.28`. SBOM: 6 references to 2 paths `path+file:///var/tmp/rev05/release/src/crates/oradmp-core|oradmp-py`,
timestamp = the epoch. sdist: one mtime (the epoch), uid 0. Review item (8): **`uv.lock` is in neither archive**
(listings in `release_build.log`). Refusals of the script tried: a symlink into `$HOME` (resolved by `realpath -m`,
refused), a root with the user name as a component, a relative root: each refused before anything was created.

## 4. Install from the wheel

Fresh venvs with only the wheel and the listed packages; `pytest -rs -p no:cacheprovider` from the build clone
`$ORADMP_BUILD_ROOT/src`; `oradmp.__file__` in `site-packages` (not the source tree), `_core` without `_probe_panic`.

| Python | pyarrow | pandas | numpy | result |
|---|---|---|---|---|
| 3.11.16 | **15.0.2** (oldest supported) | 2.2.3 | 1.26.4 | **114 passed, 3 skipped** |
| 3.14.7 (newest available) | 26.0.0 | 3.0.6 | 2.5.4 | **114 passed, 3 skipped** |

The 3 skips: `tests/test_internal.py:37, :51, :64`, "_core has no _probe_panic" (expected, checklist step 7).
`test_interval.py` and `test_consumers.py` pass on pyarrow 15.0.2. `python -m oradmp survey` on a copied fixture
runs. Also the sdist: `pip install oradmp-0.1.0a2.tar.gz` in a fresh 3.13 venv built and installed (maturin fetched
by pip), `import oradmp` 0.1.0a2, `survey` runs.

## 5. Full checks (clean clone of HEAD)

| check | result |
|---|---|
| `cargo fmt --check` | ok |
| `cargo clippy --all-targets -- -D warnings`; `--all-features` | ok; ok |
| `cargo nextest run -j 2` | **411 run: 411 passed, 3 skipped** |
| `cargo deny check` (all four) | `advisories ok, bans ok, licenses ok, sources ok` (warnings: `ISC`, `Zlib` allowances unused, duplicate crates); advisory DB fetched in this run (its last commit 2026-10-21 08:20 +0200) |
| `ruff check` / `ruff format --check` | ok / 30 files formatted |
| `mypy` | no issues, 20 source files |
| `pytest -rs` (dev build `--features test-probes`, Python 3.13.5, pyarrow 26.0.0, pandas 3.0.6) | **117 passed, 0 skipped** |
| `sha256sum -c SHA256SUMS` | 18c (147), 11g (140), 11g-cp1251 (30): ok |

Counts agree with the T-13i note and the plan's T-13i heading: 408 + 3 = 411 Rust tests (+ 3 ignored); 105 + 3 + 9 =
117 pytest.

## 6. Docs and fuzz commands

**README** (`README.md:8, 31-44`): version 0.1.0a2 in the status line and in all three install lines; install text
"From PyPI ... `pip install "oradmp==0.1.0a2"`", then the wheel file "Offline" and the sdist: this is the Q-10 =
upload form of ADR-67 item 2. TestPyPI is a run B step and rightly not in the README. Problem: SF-2. Q-10 itself:
`open-questions.md` (session 17) says no owner answer is recorded under `docs/design/`; the gate (step 1) has the
owner confirm it.

**CONTRIBUTING "Fuzzing"** documents `tools/fuzz_run.sh <walk|read|values> [seconds]`. Run as written in the fresh
clone (no `fuzz/corpus/`, no `fuzz/target/`), with `ORADMP_FUZZ_TMP=/var/tmp/rev05/fuzztmp` (the documented variable;
the default `/var/tmp/oradmp-fuzz` is outside my scratch folder):

    tools/fuzz_run.sh walk 60      exit 0, 2 min 8 s (build included)
    #236    INITED cov: 1067 ft: 3014 corp: 63/4100Kb
    #93705  DONE   cov: 1569 ft: 5320 corp: 360/16Mb exec/s: 1536 rss: 198Mb
    Done 93705 runs in 61 second(s)

235 seeds copied; `fuzz/corpus/walk` created; `fuzz/artifacts/walk/` empty; `fuzz-0.log` moved to the tmp folder,
no `oradmp-fuzz-*` left; `git status` clean. Exit paths: `ORADMP_FUZZ_TMP=/tmp` (94 MB free): refused, exit 1;
`walk 1x`: exit 2; target `nope`: usage, exit 2; a file put into `fuzz/artifacts/walk/`: "FINDING", exit 1.

The plan's manual equivalent (ADR-64 item 3), on `read`, `TMPDIR` in my scratch folder instead of
`/var/tmp/oradmp-fuzz`: `tools/fuzz_seeds.sh && mkdir -p fuzz/corpus/read && cd fuzz && TMPDIR=... cargo fuzz run read
corpus/read ../target/fuzz-seeds -- -max_total_time=60 -max_len=1048576 -timeout=10 -rss_limit_mb=2048
-malloc_limit_mb=64 -jobs=1 -workers=1`: exit 0, `DONE cov: 5180 ft: 17569 corp: 632/40Mb exec/s: 912`, 55,656
runs, artifacts empty. As expected for the manual form it leaves `fuzz/fuzz-0.log` (ignored by `fuzz/.gitignore`)
and one `oradmp-fuzz-<pid>` folder in `TMPDIR`.

## 7. Release run B checklist (ADR-67; plan, "Release checklist", steps 1 to 11)

| step | what I found |
|---|---|
| 1 Gate | T-13i and its version commit are in; Analyst session 11 done (rules v3.6 in `469416d`, T9 now "386 lines, 1,907 column definitions"); this review has no blocking finding. The candidate will be a docs-only commit after `dc7668e` (session 17 docs, this report): `git diff --stat dc7668e <candidate>` must list only `docs/`; today it would. Q-10: owner to confirm (see 6). If SF-2 (b) is chosen, this condition does not hold as written. |
| 2 Name | Only PyPI was checked (2026-10-18). **TestPyPI is a separate namespace**; the checklist does not check that `oradmp` is free there. If it is taken, step 10 (a) fails before anything irreversible; harmless, but there is no fallback in the plan (N-7). |
| 3 Checks | All run, counts above. 3b "not due" confirmed: `git diff --stat 39cbb93 dc7668e -- crates/oradmp-core/src` = `framing.rs` +15, `reader.rs` +20, both inside `mod tests` (hunks at `:133` and `:1567`, after `#[cfg(test)]` at `:115` / `:1520`); root `Cargo.toml` changed by the version line only; no profile or toolchain change. |
| 4 Network checks | `cargo deny check` all ok on `dc7668e` (section 5). I did not run `cargo tree` for the target. |
| 5 Version | `0.1.0-alpha.2` in `Cargo.toml`, `Cargo.lock` (2), `fuzz/Cargo.lock` (1); README three places. There is **no test that the README names the Cargo version**; ADR-67 item 2's "the README test" exists only as step 5's manual `grep -n 0.1.0a2 README.md` (N-6). |
| 6 Build | Works, reproducible (section 3). Step 6 says `tools/release_build.sh <commit>`; ADR-67 item 3 says `--twice`; worth writing `--twice` into step 6 (N-6). The hashes depend on the commit (epoch) **and on the build root** (the SBOM holds `path+file://<root>/src/...`): the owner's build of the candidate gives other hashes than mine; compare TestPyPI / PyPI with the owner's own `SHA256SUMS`, as the checklist says. |
| 7 Install | Done on `dc7668e` (section 4); expected skips exactly the three probe tests. |
| 8 RHEL 8 | Not checkable here; only the `.so`'s highest symbol `GLIBC_2.28` checked. |
| 9 Tag | On the commit built in step 6: correct (ADR-63 item 2); a later docs commit gives different bytes. |
| 10 Publish | Read only; **PyPI / TestPyPI behaviour not verified** (no upload, no index access). `uv 0.12.21` has `--publish-url`, `--dry-run`, `UV_PUBLISH_TOKEN` (from `uv publish --help`); the glob `<dist>/oradmp-0.1.0a2*` matches exactly the two files. Suggestions (N-7): `--check-url <index>/simple/` makes a retry after a partial upload skip files already there instead of failing; the simple page can lag a few minutes behind an upload (a missing file is not a mismatch; retry before stopping); PyPI requires 2FA on the account before a token can be made. Unverified: that PyPI accepts Metadata-Version 2.4 with `License-Expression` and the `sboms/` folder (they are current standards; I could not test). |
| 11 Record | Nothing to check yet. |

## Notes

- **N-1 (panic guard list).** Unguarded and not named in ADR-65 item 1 / the `lib.rs` comment: `Warning`'s `Display`
  (`errors.rs:99`), `Error`'s `Display` in `error_instance` (`errors.rs:51`, survey / catalog reports),
  `Charset::from_id` / `name` (`conv.rs:20`), the `as_str` constants, and the implicit drops of `PyReader` / `PyDump`
  at garbage collection. All panic-free by reading. Suggest adding them to the exempt list so the list is complete.
- **N-2 (review item (8)).** `469416d` comments out the three `fuzz/` lines of the root `.gitignore` (the owner's
  file, O-16; plan T-13i "Touches: not ... the root `.gitignore`") and adds a root `uv.lock` (pins only pyarrow
  26.0.0 for an editable `.`; a by-product of `uv` in the root; in no ownership row). Neither is in the T-13i note.
  Effect today: none on the archives (`uv.lock` in neither; listings) and none on git (`fuzz/.gitignore` covers
  `corpus/`, `artifacts/`, `target/`, `fuzz-*.log`: `git status` clean after both fuzz runs in a fresh clone). The
  owner decides: restore or delete the three lines; delete `uv.lock` and ignore it, or name its owner.
- **N-3 (`t14_damage.rs:249`).** The doc comment says "The offset of every case is where the primitive that failed
  began"; the "inside a line" case expects 0x2551, the end of the file, while the line began at 0x254d (the case's own
  text says "the end of the file"). The expected offsets follow `io.rs`'s `truncated()` (the current position), not
  a rule; `exp_reader_rules.md` has no truncation-offset rule. The test pins the current behaviour (mutation
  line_start fails it); only the comment is imprecise.
- **N-4 (CONTRIBUTING).** "removes the `oradmp-fuzz-*` folders and `fuzz-0.log` afterwards": the script removes the
  folders and **moves** `fuzz-0.log` into the tmp folder.
- **N-5 (numbers).** ADR-69 item 3 computes `T_BIG` growth as 4,900,000 x b with the stop at 100,000; the test stops
  at 102,400 (25 batches), so 4,897,600 x b. Immaterial. `t15_memory.rs:11` still says "49 times the data resolves
  about one byte per row" as a fact; ADR-69 calls it arithmetic, not measured.
- **N-6 (checklist text).** Step 6 without `--twice` (ADR-67 item 3 has it); no automated README version test
  (step 5 grep only); hashes depend on the build root (SBOM) and the commit.
- **N-7 (run B, upload).** TestPyPI name not checked; `--check-url`; index lag; all unverified by me.
- **N-8 (PyPI page).** The README's relative links `[LICENSE-APACHE](LICENSE-APACHE)` and `[LICENSE-MIT](LICENSE-MIT)`
  (`README.md:164, :166`) will not resolve on PyPI (the URL alternatives next to them do). METADATA has no
  `Project-URL`, so the PyPI page will have no source link. Cosmetic; fixable in a3.
- **N-9.** The prompt says "the eight `check_dist.py` rules"; ADR-64 item 4 has nine (m1 to m9). I removed all nine
  (and m3b).
- **N-10.** The message check compares each message with the values of its own table (`check_file`, per-table
  `Corpus`), so a value of another fixture appended to a message is not a leak for it (only the `in_piece` unit test
  catches my "plain ascii" variant). That is the design of ADR-58, not a defect.

## What I did not check

- Anything on PyPI or TestPyPI: upload, `--dry-run` behaviour, simple-index `#sha256=` fragments, token scopes,
  acceptance of Metadata 2.4 / the SBOM folder, `pip` against a real index (only `--find-links`). Not verifiable here.
- The RHEL 8 install (checklist step 8); run A's records.
- The memory test `t15_memory.rs` and the throughput tests (need `ORADMP_WORK`, i.e. `corpus/work/`, which I may
  not read); benchmarks (step 3b not due).
- Fuzzing beyond 60 s on `walk` and on `read`; `values` not run; the exit path for a real crash (only a planted
  artifact file).
- a1 / a2 at `PROPTEST_CASES=10000`; c2 only through the unit test.
- Python 3.12 and 3.13 with pyarrow between 15 and 26 on the installed wheel (3.13 only via the dev build).
- The `docs/formats/` changes in `469416d` (`exp.md`, rules v3.6, `requests.md`) beyond the T9 line, and the
  committed prompt `docs/prompts/analyst-11-housekeeping.md`.
- The whole of `t12_messages.rs` beyond the T-13i diff; false-positive behaviour of the widened (e1) / (e3).
- Golden files and generators: T-13i touched none (`git show --stat`), so I did not re-run them.

Scratch: `/var/tmp/rev05`, **9.5 GB** at the end (Cargo targets 3.6 GB + 3.2 GB, release root 0.8 GB, fuzz clone
0.5 GB, five venvs about 1.3 GB, two clones); left in place for the owner to delete. Outside it: the shared caches
`~/.cargo` (registry, advisory DB refreshed by `cargo deny`) and `~/.cache/uv` were used; the one pip wheel-cache
entry my sdist build created was removed.
