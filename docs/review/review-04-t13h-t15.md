# Review 04: T-13h (review 03 fixes), T-14 (fuzzing, damage tests), T-15 (benchmarks), the release build

Reviewer session 4, 2026-10-19. Prompt: `docs/prompts/review-04-t13h-t15.md` (untracked, not touched).

At the start:

    $ git log --oneline -12
    b3a5a7f T-15: criterion benches, throughput and memory tests, stream memory tool
    abd42ad T-14: fuzz targets, damage tests, exact counts
    b7304d5 Architect 15: ADR-61..63, T-14/T-15 order, release runs A and B
    39cbb93 T-13h: panic guard, shared stream state, release build script, dist and message checks
    c1bfeba Architect 14: ADR-55..60, T-13h, release build procedure; O-14
    60ff0df Reviewer 3 report and prompt
    3a1bb5f T-13g: rename to oradmp, version from metadata, dist checks
    0964683 Architect 13: ADR-54 rename to oradmp, T-13g; O-13 edits
    3fe33e7 T-13f: deny targets, version 0.1.0-alpha.1, README, .gitattributes
    0a0fda5 Q-5: ownership table, rule 7, scope updates
    e23a24e Architect 12: ADR-49..53, T-13f, T-16, release checklist
    e2eaae6 T-13: interval_as, Python truth tests, licence files, packaging
    $ git status --porcelain
    ?? docs/prompts/review-04-t13h-t15.md

T-13h, T-14 and T-15 are committed; nothing else was uncommitted, so **HEAD `b3a5a7f`** is what was reviewed.

Method. `git clone` of HEAD into a scratch folder under `/var/tmp` (no path holds the old name), Cargo target
directory, `TMPDIR`, venvs and release root all below it. `CARGO_BUILD_JOBS=2`, `cargo nextest run -j 2`, one build at a
time; fuzzing, benchmarks and memory runs alone. Every mutation was applied alone by a script (exact text replaced,
occurrence asserted), the suite run, the clone restored with `git checkout -- .` (`git status` clean after each).
Binding / Python mutations: `maturin develop --features test-probes`, full `pytest`. Core mutations: full `cargo nextest
run` or the named binary. `ORADMP_WORK` pointed read-only at the repository's work folder; only the T-15 tests and my
two measurement scripts read the two `t_rows_exp*_18c.dmp` files through it.

Baseline in the clone: `cargo fmt --check` ok; `cargo clippy --all-targets -- -D warnings` and `--all-features` ok;
`cargo nextest run -j 2` **408 passed, 3 skipped** (the three `#[ignore]` T-15 tests); dev build with probes (Python
3.13.5, pyarrow 26.0.0, pandas 3.0.6): `ruff check`, `ruff format --check`, `mypy` (20 files) ok, `pytest -rs` **105
passed, 0 skipped**.

## Summary

- **Blocking: none.**
- **Should fix: 5** (S-1 to S-5 below): two ADR-55/61 behaviours of the stream path that no test can fail (the
  iterator after `close()`; the warnings after a stream error); the T-14 fuzz command does not run as written; the
  older `check_dist.py` rules have no test; the message check never sees the one owned message with a runtime integer
  (`in_piece`), so its "at most 5 digits" premise is untested for it.
- Review 03's S-1 to S-5: **closed** (section 1); every mutation of the T-13h note that I repeated fails the named test.
- Release build (`tools/release_build.sh b3a5a7f --twice`): `check_dist: ok`; **0** `/home/`, `/root/`, `/Users/`,
  `C:\Users\` in the wheel (`.so` strings, SBOM, METADATA, RECORD, every member) and in the sdist; `_probe_panic` not
  in the `.so`; wheel and sdist **byte-identical** across the two builds.
- Fuzzing: three targets, 2 minutes each from the 235 seeds: 0 crashes, 0 timeouts, 0 out-of-memory.
- Numbers agree everywhere except rule T9 (still 382 / 1885, should be 386 / 1,907).

## 1. Review 03's S-1 to S-5

### S-1 (warnings lost on the Arrow C stream path): closed

`crates/oradmp-py/src/lib.rs` read in full (680 lines). `ReaderState` behind `Arc<Mutex<..>>`; `pull_raw` (`:405-431`)
is the one pull of both paths; `SharedBatches::next` (`:452-460`) calls it inside `catch_panic`; `rows` and `warnings`
are copied from the source after every pull (`:425-426`); no stash.

Behaviour on the release-like dev build (scratch script, 18c `t_num_exp_18c.dmp` T_NUM):
- `on_value_error="null"`, full `from_stream(r).read_all()`: 33 rows, `rows_read == 33`, warnings
  `[(W-VALUE-NULLED, count 10, offset 8975, row 19, V), (W-NUMBER-ROUNDED, 4, 9125, row 25, V)]`, **equal** to the
  iterator path (codes, counts, offsets, rows, columns).
- `batch_rows=5`, one `read_next_batch()`: `rows_read == 5`, warnings `[]`.
- `close()` before the first pull: `ArrowInvalid: External error: [E-CONFIG] the reader is closed`, `rows_read` stays.
- `next(r)` after the export: `UsageError` `E-CONFIG` "the reader was exported to an Arrow stream".
- Default policy (raise): `ArrowInvalid: External error: [E-NUMBER-OVERFLOW offset=0x230f] ...`, `rows_read == 18`.
All as ADR-55, amended by ADR-61.

| id | mutation (binding / Python) | fails |
|---|---|---|
| s1a | the stream gets a private state with the source taken out (the old stash) | 7 of `test_stream.py` + `test_warnings.py::test_every_issuing_call_points_the_warning_at_the_users_line[close]`, `[with]` |
| s1b | `_close` does not call `_issue_new` | `test_stream.py::test_the_warnings_of_the_stream_path_are_issued_once_at_the_end_of_the_with`, `test_warnings.py::...[close]`, `[with]` |
| s1c (own) | `rows` copied **before** the pull (lags one batch) | `test_stream.py::test_rows_read_follows_the_consumer_one_batch_at_a_time`, `..._error_inside_the_stream_...`, `test_warnings.py::test_an_error_ends_the_iteration_and_issues_the_counts_up_to_it` |
| s1d (own) | warnings not copied on stream pulls | 3 of `test_stream.py`, `test_warnings.py::...[close]`, `[with]` |
| s1g (own) | stream pull after `close()` is a silent end | `test_close_before_the_first_pull_makes_the_pull_fail_with_e_config` |
| s1h (own) | `close()` keeps the source | the same test |
| **s1e** (own) | warnings not copied when the pull **errs** | **none (105 passed)**: S-2 |
| **s1f** (own) | iterator pull after `close()` is a silent end (`finished \|\| !from_stream`) | **none (105 passed)**: S-1 |

### S-2 (home path in the wheel): closed

`tools/release_build.sh` read in full (156 lines): maturin pinned `==1.15.0` (`ziglang` is not pinned; zig 0.17.0 was
installed); `SOURCE_DATE_EPOCH` = commit time (`git log -1 --format=%ct`); `RUSTFLAGS="--remap-path-prefix=$CARGO_HOME=/cargo
--remap-path-prefix=$ROOT=/build"`; `git clone --no-local` of the commit; `--locked`; refusals tried: a root inside
`$HOME`, a root with the user name as a component, an existing root, a relative root: each refused with its message
(the check does not resolve symlinks). No `--features` in the maturin call and none in `pyproject.toml`, so
`test-probes` is off.

Run: `ORADMP_BUILD_ROOT=<scratch>/release tools/release_build.sh b3a5a7f --twice`, rustc 1.99.0, maturin 1.15.0,
zig 0.17.0, epoch 1792452909. `check_dist: ok` twice. Wheel `oradmp-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl`
(3,625,248 bytes) sha256 `3907eb2f...1ead1`; sdist `oradmp-0.1.0a1.tar.gz` (154,734 bytes) sha256 `ac9e2310...e521f`.
**`--twice`: identical wheel, identical sdist ("reproducible").**

| marker | `strings -a _core.abi3.so` | SBOM | METADATA | RECORD | wheel members (`grep -rla`) | sdist members |
|---|---|---|---|---|---|---|
| `/home/` | 0 | 0 | 0 | 0 | 0 | 0 |
| `/root/` | 0 | 0 | 0 | 0 | 0 | 0 |
| `/Users/` | 0 | 0 | 0 | 0 | 0 | 0 |
| `C:\Users` | 0 | 0 | 0 | 0 | 0 | 0 |
| `_probe_panic` | 0 | 0 | 0 | 0 | 1 (`oradmp/_core.pyi`, the stub) | 3 (stub, `oradmp-py/Cargo.toml`, `lib.rs`: source) |
| `/cargo/` (remapped registry) | 161 | 0 | 0 | 0 | 1 | 0 |

The SBOM's references are 6 occurrences of 2 paths `path+file://<build root>/src/crates/oradmp-core|oradmp-py`; its
`timestamp` is `2026-10-19T23:35:09Z` = the epoch. The string `max` occurs (`max_lob_bytes`, ...) as ADR-56 expected; no
path component.

### S-3 (`check_dist.py` rules): closed for ADR-57's rules; older rules untested (S-4)

Mutations of `tests/check_dist.py`, each alone, full pytest:

| mutation | fails |
|---|---|
| only the first path component tested | `test_a_corpus_folder_nested_in_the_sdist_fails`, `test_every_forbidden_folder_fails_at_any_depth[tests, docs, target, .venv, .git]` |
| `.pyc` / `.pyo` suffix rule off | `test_a_pyo_in_the_wheel_fails` |
| `tools` not forbidden | `test_a_tools_folder_in_the_sdist_fails` |
| `__pycache__` not forbidden | none (the `.pyc` of the test is caught by the suffix rule: near-equivalent) |
| `/home/` / `/Users/` / `/root/` off | `test_a_home_path_in_the_sbom_...` / `test_a_users_path_inside_a_binary_member_fails` / `test_a_root_home_in_the_sdist_fails` |
| `C:\Users\` plain off / escaped off / case-sensitive | `test_a_windows_home_...[C:\Users\u]`, `[c:\USERS\u]` / `["C:\\Users\\u"]` / `[c:\USERS\u]` |
| content scan off (wheel) / (sdist) | 5 home tests / `test_a_root_home_in_the_sdist_fails` |
| `_probe_panic` rule off | `test_the_test_probe_in_the_wheels_extension_module_fails` |
| **wheel version vs `Cargo.toml` off** | **none** |
| **METADATA `Version` check off** | **none** |
| **old-name scan off** | **none** |
| **`.dmp` rule / `truth_*.txt` rule / `exp_survey.py` rule / "outside `oradmp/` and `.dist-info`" / sdist root name** off | **none** (5 mutations) |

### S-4 (message check): closed; two gaps noted

`t12_messages.rs` read in full (989 lines). Message mutations in the `Err(e)` arm of `reader.rs` `field()`, test
`no_message_of_the_fixtures_holds_a_value` (forms printed by the assertion):

| id | mutation | forms |
|---|---|---|
| d1 | lossy text of the field in every value error | {Text} |
| d2 | hex of the field | {Hex} |
| d4b | failing NUMBER's `to_text()` in `E-NUMBER-OVERFLOW` | {Cell, Decoded} |
| d4c | the same as `{:E}` of its `to_f64` (`1E33`) | {Decoded} |
| d6 | hex of the first 8 bytes of a field longer than 8 | {Hex} (`timestamp outside the 64-bit ...`) |
| d7 | the same with spaces | {Hex} |
| d8 | decoded date text in `E-BAD-DATETIME` | {Decoded} |
| d12 (own) | `0x` + upper-case hex of the first 8 bytes | {Hex} |
| h-nul | `\u{0}` appended to "months do not fit 32 bits" (`interval.rs`) | `no_message_...`: "holds U+0000: [E-BAD-INTERVAL offset=0x230a] ..." |
| **d11** (own) | date as `DD.MM.YYYY` (`15.06.-100`) | **none** |
| **d13** (own) | the overflowing number in 5-digit groups with commas | **none** |
| **premise** (own) | `in_piece` prints `n + 100000` (a 6-digit piece number) | **none** (S-5) |

(d3 and d5 not repeated separately: d1 covers the lossy text of `E-BAD-TEXT` too.)

Thresholds: hex 8+ digits (= the 4 bytes of form (a)), digit runs 6+, stored text 8+ characters are sound for the
messages that exist; d11 / d13 are in ADR-58's accepted class ("a value transformed in a way none of the forms covers")
but both are ordinary formats (N-5). The premise "messages hold integers of at most 5 digits" is asserted over the
messages the fixtures produce; `in_piece` (`reader.rs:165`, a `u64` piece / chunk number) is never among them (S-5),
and structurally a piece number of a long chain can exceed 5 digits.

Can a row value reach a message? Core: `message` is `Cow<'static, str>`; the only owned texts are `in_piece`
(integer) and `lob_unconfirmed_message` (`reader.rs:1052`, the header's set id); `Display` (`error.rs:536-583`) adds
the code, offset and integer context. Binding: `to_pyerr` passes `e.to_string()`; names only go into attributes;
`_api._usage` uses literals. **No value can reach a message today.**

### S-5 (panic guard): closed; coverage narrower than "every call" (N-1)

`guard.rs` (77 lines): `catch_unwind(AssertUnwindSafe(f))`, payload dropped, fixed `INTERNAL_MESSAGE`; 4 unit tests.
Guarded: `PyDump::with` (scan, `read_table`, `survey` body, `digest`, probe), `open`, `next_batch`
(`pull_raw` inside `catch_panic`), `SharedBatches::next`. A caught panic marks the dump (`broken` + `inner = None`) or
the reader (`broken` + `source = None`) and every later call is `E-INTERNAL` "an earlier call failed ..."
(`test_internal.py` asserts both, and `info()` / `scan()` / `arrow_c_stream()` after it).

| id | mutation | fails |
|---|---|---|
| e1 | no `catch_panic` in `PyDump::with` | `test_internal.py::test_a_panic_through_the_dump_guard_...` |
| e2 | none in `next_batch` | `test_internal.py::test_a_panic_through_the_batch_path_...` |
| e3 | none in `SharedBatches::next` | `test_internal.py::test_a_panic_inside_the_arrow_stream_...` (subprocess) |
| e5 (own) | reader not marked `broken` after a panic | `..._batch_path_..._unusable_after` |
| e6 (own) | dump not marked after a panic | `..._dump_guard_..._unusable_after` |

Calls into the core **outside** the guard: `NumberType::from_str` (`options.rs:97`, `:139`, user strings),
`number_summary` (`lib.rs:254`, with the GIL held), the accessors `info()`, `catalog()`, `warnings()`
(`lib.rs:137-166`), `TableBatches::schema()` (`lib.rs:201`), `Error`'s `Display` in `to_pyerr`, and the drops in
both `close()`. All panic-free under the workspace lints; but ADR-59 item 2 says "every call into the core" (N-1).

## 2. ADR-61 deviations 1, 2 and the NUL rule

1. README (`README.md`, after the stream example): the sentence of ADR-61 item 2, verbatim (the line break inside the
   code span renders as a space). The pattern `\[(E-[A-Z0-9-]+)` returns the right code for **all 25**
   `_core.ERROR_CODES`, with and without an offset, with a `warning W-...` context after it (script); the real texts
   seen: `[E-CONFIG]`, `[E-NUMBER-OVERFLOW offset=0x230f]`, `[E-INTERNAL]` (probe). Every stream error is
   `ArrowError::ExternalError`, so pyarrow raises `ArrowInvalid` for each.
2. After `close()`: iterator `next(r)` and `read_all()` raise `UsageError` `[E-CONFIG] the reader is closed`; stream
   pull `ArrowInvalid ... [E-CONFIG] the reader is closed`; after the end both give the end again (`StopIteration`).
   Shown by script. **The iterator half has no test** (s1f survives): S-1.
3. NUL: `assert_no_nul` over the `Display` text of every error and warning of the 201 files + copies under four
   policies; `the_nul_check_fails_on_a_nul_...` is the self-test; h-nul fails `no_message_...`. The fuzz targets check
   the same (`fuzz/src/lib.rs`).

## 3. The lock rule (ADR-61 item 7)

Every `lock(` in `crates/oradmp-py/src/lib.rs` (the binding's only mutexes; `conv.rs`, `options.rs`, `errors.rs`,
`names.rs`, `digest.rs` have none):

| line | mutex | GIL | held across |
|---|---|---|---|
| 57 `names()` | dump `names` | **held** | an `Arc` clone |
| 76, 82 `with` | dump `inner` | released (`detach`) | the closure `f` (core scan / read / survey / digest); then `= None` after a panic |
| 96 `scan_inner` | dump `names` | **held** | store of a new `Arc<Names>` (drop of the old one) |
| 137, 149, 164 `info` / `scan` / `warnings` | dump `inner` | released | `clone()`, `copy_catalog`, `to_vec()`; Python objects built after |
| 292, 296 `close` / `is_closed` | dump `inner` | released | `= None` / `is_none()` |
| 406 `pull_raw`, 436 `after_pull` | reader `shared` | released (`next_batch` in `detach`) or unknown (`SharedBatches::next`, from the consumer) | `TableBatches::next_batch` (core), copies of rows / warnings |
| 520, 529, 541, 571 | reader `shared` | released | clone of warnings, rows, export flags, `source = None` |

(a) holds for every lock of `inner` and `shared`; it does **not** hold literally for the `names` mutex (`:57`, `:96`),
which is taken with the GIL held. Harmless: its critical sections are an `Arc` clone or store, call no Python and never
wait for the GIL, so no cycle can form (N-2). (b) No closure calls `Python::attach` (none in the binding) or touches a
`Py<...>`; the closures compile as `Ungil`. (c) Every Python object is built after the guard is dropped. No deadlock
path found.

## 4. What review 3 did not reach

(a) T-12 breaks: re-run **b2** (`.with_warning(LobUntested)` removed: fails
`t12_warning_field::a_strict_lob_error_names_the_warning_only_when_decode_was_asked_for`), **c** (rounded-to-zero not
counted: fails `t12_survey::the_number_profile_flags_the_computed_quotients`, `..._of_t_num_column_v`), **g** (integer
part too large counted as exact: fails `t12_survey::the_number_profile_of_t_num_column_v`): all as the T-12 note says.
**Not re-run: "S-3 old `reader.rs` from HEAD"** (the pre-T-12 `reader.rs` no longer compiles against today's core).

(b) Arrow stream path on the release wheel (Python 3.13.5, pyarrow 26.0.0), `T_ROWS` of both 18c exports, `batch_rows
=1000`, 100 batches pulled one by one through `from_stream(...).read_next_batch()` and dropped: VmRSS 57,568 kB at batch
5 and 57,644 kB at batch 100 (conventional), 57,256 -> 57,352 kB (direct): flat. GIL: a Python counter thread ran at
0.84 / 0.95 of its idle rate during ten stream reads of `T_ROWS`, 0.91 / 0.96 during iterator reads, and 0.45 / 0.49
while the main thread ran pure Python (GIL held): **the GIL is released on the stream path** (pyarrow releases it while
pulling; our pull never takes it).

(c) The release wheel in fresh venvs, full `pytest -rs` from the clone: **Python 3.11.16 + pyarrow 15.0.2 + pandas
2.2.3 + numpy 1.26.4: 102 passed, 3 skipped**; **Python 3.14.7 + pyarrow 26.0.0 + pandas 3.0.6: 102 passed, 3
skipped**. The 3 skips are the probe tests (`_core has no _probe_panic`); `test_interval.py` and `test_consumers.py`
pass on pyarrow 15.0.2 (the refusal of Interval(MonthDayNano) by Parquet holds there too).

(d) `uv pip install` of the sdist into a fresh 3.13 venv (network allowed: maturin and crates fetched): built in 2 min
16 s, installed; pytest 102 passed, 3 skipped. (A local source build is not remapped: its `.so` holds 179 `/home/`
strings; expected, only the release script remaps.)

(e) `cargo deny check advisories sources bans`: **`advisories ok, bans ok, sources ok`** (one `duplicate` warning for
`syn`); `licenses ok`. Whether the advisory database was refreshed by this run I did not verify.

## 5. T-14

**Targets** (`fuzz/` read in full). `walk` calls `exp::items::walk(Cursor<&[u8]>, hash_fields = true)`; `read`
writes the input to `<TMPDIR>/oradmp-fuzz-<pid>/in.dmp`, then `dump::open`, `scan`, every table to its end with
`batch_rows 1024`, `batch_bytes 4 MiB`, `max_lob_bytes 1 MiB`, `on_value_error Null`; `values` selects one of 9
decoders by `byte % 9` and calls every conversion. These are the entry points of ADR-62 item 5 / T-14 (c). Checks
(`fuzz/src/lib.rs`): `[E-` / `[W-` prefix, code in `ALL`, never `E-INTERNAL`, offset <= input length, no U+0000.
What keeps early rejection from hiding later code: only the seeds (the 201 real exp files pass the header) and
coverage guidance; there is no dictionary of introducing lines. Coverage at the end of my runs: `read` cov 5,356, `walk`
1,670 (the note: 5,673 / 1,725 after 10 minutes). Not fuzzed: `strict`, `interval_as="string"`, `number_types`,
column projection, `trim_char`, the charset decoders outside a read (N-9).

**The plan's command** from the repository root fails twice: without `RUSTUP_TOOLCHAIN=nightly-2026-10-15`
("1 nightly option were parsed", as the note says), and, with it, `ERROR: The required directory "fuzz/corpus/walk"
does not exist` until `mkdir -p fuzz/corpus/<t>` (not in the note). S-3.

**Runs** (`RUSTUP_TOOLCHAIN=nightly-2026-10-15`, seeds by `tools/fuzz_seeds.sh`: 235 = 201 exp + 34 Data Pump; no
seed above 1 MiB, so `-max_len` cuts none; plan limits, `-max_total_time=120`, one at a time):

| target | runs | exec/s | cov | ft | corp | crashes / timeouts / OOM |
|---|---|---|---|---|---|---|
| `walk` | 263,082 | 2,174 | 1,670 | 6,127 | 460 / 19 MB | 0 / 0 / 0 |
| `read` | 100,687 | 832 | 5,356 | 18,566 | 738 / 45 MB | 0 / 0 / 0 |
| `values` | 15,577,739 | 128,741 | 1,094 | 2,012 | 270 / 3,210 B | 0 / 0 / 0 |

`fuzz/artifacts/` empty; `read` left its `oradmp-fuzz-<pid>` folder in `TMPDIR`, as the note says.

**The two choices not in an ADR.** Debug assertions and overflow checks in the fuzz profile: sound (stricter than the
release build; a `debug_assert` firing in a dependency is a real finding too). Warning offsets against the input
length: sound (a warning beyond the file is a bug). **`-malloc_limit_mb=64`** bounds a single allocation only;
the cumulative bound is `-rss_limit_mb=2048`, about 2,000 times a 1 MiB input. It catches an allocation sized from a
file field (the risky kind: `Vec::with_capacity` from a count), not a slow growth; enough for "no unbounded
allocation from a length field", not for arch 8.3 (a)'s "never allocates more than a fixed bound" (N-9).

**`t14_damage.rs`** (227 lines, read in full). Each error is checked for `[E-`, code in `ErrorCode::ALL`, not
`E-INTERNAL`, no NUL, offset <= t (truncation) or <= file length (flips); rows per ordinal capped by
`exp_catalog.tsv` on truncation; warnings checked for `[W-`, NUL and code, **not** for their offset. So it checks code
and offset, not only "no panic". Mutations:

| id | mutation (core) | 256 cases (full suite) | `PROPTEST_CASES=10000` |
|---|---|---|---|
| a1 | `rows.rs:611` "file ends where a row ... should stand" at `at + 3` | none | `truncation_gives_ok_or_a_typed_error` ("offset 12211 beyond 12208", `idx = 97, cut = 4775291877934326139`) |
| a2 | `framing.rs:109` "file ends inside a field" at `at + 2` | none | **none** |
| b1 | `rows.rs:611` returns `E-INTERNAL` | `truncation_gives_ok_or_a_typed_error` | - |
| b2 | `E-TRUNCATED` dropped from `ErrorCode::ALL` (a code not in the table) | `error::tests::all_lists_every_code_once`, both damage properties | - |
| c1 | panic in the `rows.rs:611` path | `truncation_...`, `t3a_walker::x_damaged_lob_partition_and_long_files_never_panic`, `t1_walk::damaged_files_never_panic` | - |
| c2 | panic in the `framing.rs:109` path | **none** | **none**; a 90 s `walk` fuzz run did not reach it either |

The `framing.rs:109` path needs an unknown introducing line whose field ends exactly at the end of the file: neither
sampling reaches it (N-3). Reproducibility: on failure proptest prints the shrunk input (`idx`, `cut`) and writes
`crates/oradmp-core/tests/t14_damage.proptest-regressions` (it appeared in my clone, deleted); the seed itself is
not printed, the regression file holds it.

**N-9 exact counts**, re-derived with `exp_parse.parse` over the 201 files (my script, not the golden files):
386 sections, 400 parts, 4 partitioned sections; 80 direct files, their conventional partners **141 sections, 16,461
rows** (the direct files 143 / 16,461); **21** parts with LOB values, **16** parts with a LONG / LONG RAW column;
**152** sections whose table is one of the 16 names of `18c/truth_counts.txt`; failed exports from the logs:
`grep -a "exporting table"` without `rows exported` = 3 (`???_???` twice, `\xbf\xbf\xbf_\xbf\xbf\xbf` once) plus 4
partition headers, so **379** = 386 - 4 - 3; `collist.py`: **4** `TIMESTAMP (n) WITH LOCAL TIME ZONE`. All equal the
test literals. "3 failed exports": mutation "failed table not counted" fails `t3_catalog::x_row_counts_...` at the
`failed_tables` assertion alone (`:965`); "failed branch off" fails it at "no equal 'rows exported' line" (`:940`).
Since `compared + partitioned + failed` = the sections of each file, the 3 is implied by 379 + 4 + the 386 sections;
the assertion is redundant but can fail. Rated **fine**.

**Interval test** `day_to_second_opposite_signs_stay_e_bad_interval_under_string`: bytes asserted before patching
(`IDS_ROW1`, review 03's derivation at 0x22b0), hours `0x48` (+12) -> `0x30` (-12); expected `E-BAD-INTERVAL` row 1
column 3 comes from rule Y10 (all fields of a negative interval negative) and the decision of arch 7.1 / ADR-47
(mixed signs are an error), not from the reader.

## 6. T-15

Read in full: `benches/decode.rs`, `t15_throughput.rs`, `t15_memory.rs`, `tools/stream_memory.py`. Machine: i9-9900T,
8 CPUs, 11.5 GiB, about 9 GiB available; files of `ORADMP_WORK` in the page cache.

| quantity | note | mine (runs) | spread |
|---|---|---|---|
| `number/OraNumber::decode` | 42.5 M/s | 45.4 M/s | criterion CI +-1.5 % |
| `datetime/decode_date` / `decode_timestamp` | 66.2 / 57.6 M/s | 71.4 / 61.6 M/s | +-1.5 % |
| `text_cp1251` | 29.7 M/s | 33.5 M/s | +-2 % |
| `read_table/R_STAT` / `T_NUM` / `T_TEXT` | 1.94 M / 841 K / 239 K rows/s | 2.02 M / 846 K / 259 K | +-2 % |
| T_ROWS conventional scan / read | 803 MB/s / 2.15 M rows/s | 804 to 893 MB/s / 2.15 to 2.40 M rows/s (4 runs) | about +-6 % |
| T_ROWS direct scan / read | 830 MB/s / 2.32 M rows/s | 899 to 929 MB/s / 2.35 to 2.52 M rows/s (4 runs) | about +-5 % |

Mine are 0 to 13 % faster; within one session the spread of a 3-run median of an 11 ms scan is about 10 %, and
between sessions the VM's load differs, so they agree within the noise. What they mean: warm-cache, single-thread
decode speed of a 10 MB file on this VM; scan MB/s is memory speed. What they do not: disk or cold-cache speed, files
larger than 10 MB, other column mixes, or Python overhead.

**Memory test.** The two measurements run in **separate child processes** (`current_exe` + `t15_child_read`), so
`VmHWM` of the first cannot hide the second. My runs: ratios 1.028 / 1.037 and 1.049 / 1.035 (note 1.046 / 1.023):
noise up to about 1.05. Mutations in `TableBatches::next_batch` (core):

| mutation | stop / all peak kB | ratio | result |
|---|---|---|---|
| keep every batch until the reader is dropped | 8,292 / 19,640 | 2.369 | **fails at the 1.10 assertion** |
| keep 16 bytes per row | 7,740 / 10,800 | 1.395 | fails |
| keep 8 bytes per row | 7,652 / 8,900 | 1.163 | fails |
| keep 4 bytes per row | 7,740 / 8,332 (direct 7,716 / 8,180) | 1.077 / 1.060 | **passes** |

So the 1.10 limit does catch a reader that keeps its batches (the gap the note named is closed by this mutation). It
misses growth below about 6 bytes per row because the peak is mostly the 7.5 MB process; 4 bytes per row is 20 MB on
`T_BIG`. A limit of 1.05 is inside the noise; an absolute bound on `pall - p10` or the `T_BIG` file would be the fix
(N-4). Wording: the module comment says "stopping after 10,000 rows (at the end of the batch that reaches them:
12,288)" (honest); the test name says "ten times the data" while 100,000 / 12,288 = 8.1, and the assertion
`(10_000..20_000).contains(&r10)` could be `== 12_288`.

**`tools/stream_memory.py`.** On my release build (Python 3.13.5, pyarrow 26.0.0): iterator 57,348 -> 57,348 kB,
stream 57,720 -> 57,732 kB, **PASS**. FAIL path triggered (release `maturin develop` of mutated clones):
- a static list in `SharedBatches::next` keeps every streamed batch: stream 76,792 -> 151,540 kB, "RSS at 500 minus
  RSS at 50 = 74748 kB (limit 10240 kB)", **FAIL**, exit 1;
- `_api.py` keeps every batch of `__next__` in a module list: iterator 76,624 -> 155,448 kB (78,824 kB), **FAIL**;
- the reader state keeps its own batches (freed with the reader): PASS, correctly (no leak across reads; the tool
  does not measure the peak inside one read).

**T_BIG / cold cache.** Not met for lack of `T_BIG`: (c) its throughput, (d) its peak; both are "if present" in the
plan, so the plan accepts their absence. ADR-62 item 8 says "Without them T-15 is not done", O-15 narrows that to
`T_ROWS`; arch 4.3's "a file ten times larger" is replaced by the plan's stop-early design (8.1x). Cold-cache reads
are in neither the plan nor arch 4.3; the note only records them. No ADR decides these (the Architect has not met
since T-15).

**README**: no statement about speed or memory; "It does not contain: ... measured performance figures or fuzzing
results" is true for the README. Nothing to correct.

## 7. Who wrote where

`git show --stat` of `39cbb93`, `abd42ad`, `b3a5a7f`: `crates/`, `python/oradmp/`, `tests/`, `tools/`, `fuzz/`,
`Cargo.lock`, `README.md`, `CONTRIBUTING.md`, and `docs/design/implementer-notes.md` (one hunk each, additions at the
end: its own sections). Nothing under `corpus/`, `docs/formats/`, `docs/review/`, `.claude/`, `CLAUDE.md`,
`docs/prompts/`, `tests/golden/` or `crates/oradmp-core/tests/data/`. **One file outside the list**: the root
`.gitignore` (T-14 added `fuzz/corpus/`, `fuzz/artifacts/`, `fuzz/target/`), which `CLAUDE.md`'s Implementer row does
not name; not mentioned in the note; `fuzz/.gitignore` already holds the same lines (N-6). The T-15 diff holds no
work-folder path and no `/home/` (its note writes `ORADMP_WORK=...`). Committed files with the owner's home path: only
`docs/` (decisions, notes, a prompt, and reviews 02 and 03, the latter mine): no code, test or tool file.

## 8. Numbers

| quantity | tools now | `common/mod.rs` `TOTALS` | plan / notes | rules |
|---|---|---|---|---|
| files / sections / parts | 201 / 386 / 400 (`corpus_counts.py`, `rowlen_stats.py`, `exp_parse`) | 201 / 386 / 400 | same | 201 / 386 |
| rows / fields / NULL | 41,212 / 243,007 / 19,900 | same | same | same |
| LOB values | 126 (80 short + 46 long, 12 empty, 21 sections) | 126 | 126 | 126 |
| NUMBER fields | 86,080 (`gen_number_parts.py`) | 86,080 | 86,080 | - |
| column definitions | 1,907 (`collist.py`: 386 lines, 6 moved, 4 TSLTZ) | 1,907 | 1,907 | **T9 (`exp_reader_rules.md:226`): "382 lines, 1885 column definitions": not corrected** |
| Rust tests | 408 passed, 3 skipped | - | 408 / 3 (T-15 note) | - |
| pytest | 105 passed, 0 skipped (dev build with probes, Python 3.13.5, pyarrow 26.0.0, pandas 3.0.6) | - | 105 | - |

Generators re-run in the clone: `gen_exp_catalog.py`, `gen_exp_raw.py` (byte-identical to `tests/golden/`),
`gen_walk_expected.py` (201 / 386 / 400 / 41,212 / 243,007 / 19,900, LOB 126) and `gen_number_parts.py` (201 files,
400 parts, 86,080): `git status` clean afterwards. The only disagreement is T9.

## 9. Deviations: decided or only noted

| note | deviation | decided by |
|---|---|---|
| T-13h 1, 2 | `External error: ` prefix; iterator `E-CONFIG` after `close()` | ADR-61 items 1, 4 |
| T-13h 3, 4, 5 | `_probe_panic(kind, path)`; self-test in three tests; `04e4` / `0x1e5` | ADR-61 items 5, 6 |
| T-13h 7 | no runtime lock test | ADR-61 item 7 |
| T-13h 10, "Not done: pyarrow 15" | `~/.cache` folder; pyarrow 15.0.2 run | ADR-61 items 9, 8 |
| T-13h 6, 8, 9, 11 | inventory holds; n4c; script `-h` and `--twice` details; debug-wheel FAIL lines | only noted (descriptive) |
| T-14 1 | `/tmp` full, `TMPDIR`, leftover `oradmp-fuzz-<pid>` folders | **only noted** |
| T-14 2 | fuzz command needs `RUSTUP_TOOLCHAIN` (and, not noted, the corpus folders) | **only noted** (plan text unchanged) |
| T-14 (b) | debug assertions and overflow checks in the fuzz profile | **only noted** |
| T-14 3, 4 | 34 Data Pump seeds; warning offsets checked | **only noted** |
| T-14 5 | memory bound only from libFuzzer; `fuzz/` not under clippy | **only noted** (ADR-62 rejects the allocation counter, not the rest) |
| T-14 (not in the note) | root `.gitignore` edited | **not noted** |
| T-15 1 to 5 | 12,288 rows; criterion `default-features = false`; `VmHWM` Linux-only; no `T_BIG`; no cold cache | **only noted** (`T_BIG` items are "if present" in the plan) |

## 10. Still unread

Now read line by line: `crates/oradmp-py/src/lib.rs`, `guard.rs`, `t12_messages.rs`, `t14_damage.rs`, `t15_*.rs`,
`benches/decode.rs`, `fuzz/`, `tools/*`, `tests/check_dist.py`, `test_check_dist.py`, `test_stream.py`,
`test_internal.py`, `_api.py`. **Still not read line by line** (from reviews 1 to 3): `t3a_walker.rs`, `t3b_parts.rs`,
`t3c_walker.rs`, `t6_read.rs`, `t6l_ru.rs`, `t6_value_parts.rs`, most of `t3_catalog.rs` (the log test is now read),
`t5_types.rs`, `t1_walk.rs`, `t2_manifest.rs`, `t5f_followup.rs`; the binding's `conv.rs`, `digest.rs`, `names.rs`
(read in T-12's review, not again); the Analyst's tools other than `exp_parse.py`, `collist.py`, `num_decode.py`
(the others only run: `corpus_counts.py`, `rowlen_stats.py`).

## Findings

### Blocking

None.

### Should fix

**S-1 The iterator after `close()` (ADR-61 item 4) has no test** (`crates/oradmp-py/src/lib.rs:417-423`,
`tests/test_stream.py`). Shown: mutation s1f (`return if st.finished || !from_stream { Ok(None) } ...`, a silent short
table on the iterator path) passes all 105 tests; the behaviour itself is right today (script). Fix: a test that
calls `next(r)` and `r.read_all()` after `close()` before the end (`UsageError`, `E-CONFIG`, "the reader is closed")
and after the end (`StopIteration`, `read_all()` returning the empty rest).

**S-2 "`r.warnings` is complete after such an error" (README; ADR-55 item 6) is untested with a non-empty warning
list** (`lib.rs:425-426`; `test_stream.py::test_an_error_inside_the_stream_...`, `test_warnings.py::test_an_error_ends_
the_iteration_...` both have no warning before the error). Shown: s1e (warnings copied only when the pull succeeds)
passes all 105 tests; no read of the 201 fixtures raises after a warning (scanned under the default options and
`on_number_overflow="error"`). Fix: a patched copy of 18c T_NUM where a value after row 25 (rounded first) overflows
the integer part, read on both paths with the default policy: the error, and `W-NUMBER-ROUNDED` with the count up to it.

**S-3 The T-14 fuzz command does not run as the plan writes it** (`implementation-plan.md` T-14 (d); no fuzz
section in `CONTRIBUTING.md`). Shown: from the repository root it fails without `RUSTUP_TOOLCHAIN=nightly-2026-10-15`
(noted), and with it fails with `The required directory "fuzz/corpus/walk" does not exist` until the folder is made
(not noted); `read` needs a `TMPDIR` with room and leaves a folder per process. Fix: a `tools/fuzz_run.sh <target>
[seconds]` that sets the toolchain, makes the folders, runs from `fuzz/` and cleans up, and the plan / CONTRIBUTING
naming it (Architect for the plan text).

**S-4 The older `check_dist.py` rules have no test** (`tests/test_check_dist.py`). Shown: eight mutations pass all 105
tests: wheel version vs `Cargo.toml` off, METADATA `Version` check off, old-name scan off, `.dmp` rule off,
`truth_*.txt` rule off, `exp_survey.py` rule off, "outside `oradmp/` and `.dist-info`" off, sdist root name off. The
prompt's S-3 list counts the version and the old name among the rules that must fail a named test. Fix: one synthetic
case each (the builders `make_wheel` / `make_sdist` already exist).

**S-5 The message check never sees an `in_piece` message** (`reader.rs:165`; `t12_messages.rs` `check_file` list).
Shown: `in_piece` printing `n + 100000` (a 6-digit integer in a message) passes all 7 tests, so the inventory premise
of ADR-58 item 3 ("at most 5 digits") is not checked for the one owned message with a runtime integer besides the set
id, and that message is not leak-checked at all. Fix: add to `no_message_of_the_fixtures_holds_a_value` the patched
chain copy of `t6_patched::a_broken_utf8_sequence_in_a_chain_is_a_value_error` (and a LOB chunk error copy), so an
`in_piece` message is in `seen`.

### Notes

- **N-1** ADR-59 item 2 says "every call into the core"; outside `catch_panic` are `NumberType::from_str`
  (`options.rs:97`, `:139`), `number_summary` (`lib.rs:254`, GIL held), the accessors of `info` / `scan` / `warnings`,
  `TableBatches::schema()` (`lib.rs:201`), `Display` in `to_pyerr`, and the drops in `close()`. Panic-free under the
  lints; wrap `options::*` and `number_summary`, or word the ADR as "every call that reads the file".
- **N-2** Lock rule (a) is broken literally by the `names` mutex (`lib.rs:57`, `:96`, GIL held); harmless (section 3).
  Take it inside `detach` or say in ADR-61 item 7 that it covers `inner` and `shared`.
- **N-3** The damage properties are samples: the truncation offset at `framing.rs:109` and a panic there survive 10,000
  cases and a 90 s `walk` run; `rows.rs:611` needs 10,000 cases. Warning offsets are not bounded in `t14_damage.rs`
  (the fuzz targets bound them). A unit test per truncation site (cut the file at the field end) would pin them.
- **N-4** Memory test: catches retained batches (2.37) and 8 bytes per row (1.16), not 4 bytes per row (1.06 to
  1.08); noise reaches 1.05. Consider an absolute bound on the growth, or `T_BIG`. Test name "ten times" vs 8.1;
  `r10` could be asserted `== 12_288`.
- **N-5** Message forms not covered (ADR-58's accepted class, but common formats): a date as `15.06.-100` (d11) and a
  number with group separators (d13). Joining digit groups separated by `,` `.` `_` `'` or a space before (e1) would
  catch the second.
- **N-6** Root `.gitignore` edited by T-14 (not in the Implementer's row of `CLAUDE.md`, not in the note; duplicates
  `fuzz/.gitignore`). Owner: add `.gitignore` to the row, or the Implementer reverts the three lines.
- **N-7** The wheel's `oradmp/_core.pyi` declares `_probe_panic`, which the release `.so` lacks (a type checker
  believes it exists). Mark it or move it to a test stub.
- **N-8** `release_build.sh`: `ziglang` is unpinned (reproducibility across time); the root check does not resolve
  symlinks; the T-13h note's "two `path+file://` references" are 6 occurrences of 2 paths.
- **N-9** Fuzz scope: `read` uses one option set (no `strict`, `interval_as="string"`, `number_types`, projection,
  `trim_char`); `values` has no charset decoder; no dictionary of introducing lines; `-malloc_limit_mb=64` bounds one
  allocation, the cumulative bound is `-rss_limit_mb=2048`.
- **N-10** Rule T9 still reads 382 / 1885 (Analyst; ADR-60 N-10).

## Tests that did not fail when mutated

- s1e (warnings not copied on an erring pull) and s1f (iterator silent end after `close()`): all 105 pytest pass (S-2, S-1).
- `check_dist.py`: version vs Cargo, METADATA version, old name, `.dmp`, `truth_*`, `exp_survey.py`, outside members,
  sdist root: all pass (S-4); `__pycache__` component off: passes (the suffix rule catches the test's `.pyc`).
- `in_piece` with a 6-digit number: all 7 `t12_messages` tests pass (S-5); d11 (date `DD.MM.YYYY`), d13 (grouped
  number): pass (N-5).
- `framing.rs:109` offset + 2 and a panic there: pass at 256 and 10,000 cases; `rows.rs:611` offset + 3 passes at 256
  cases (fails at 10,000) (N-3).
- Memory: 4 bytes kept per row: `t15_memory` passes (N-4).
- `stream_memory.py` with batches kept inside the reader state: PASS, correctly (no cross-read leak).

## What I did not check

- The T-12 break "S-3 old `reader.rs` from HEAD" (does not compile against today's core).
- d3 and d5 as separate mutations (d1 covers them); N-4's three T-13h assertions and N-5 / N-6 / N-8 items of ADR-60
  were not mutated again (they were not in this prompt's list).
- Fuzz runs longer than 2 minutes, `PROPTEST_CASES=10000` on unmutated code (only with mutations), coverage reports.
- Clippy of `fuzz/`; whether the advisory database was refreshed by `cargo deny`.
- Cold-cache throughput, `T_BIG` (absent), any platform other than Linux x86_64 on this VM, an actual glibc 2.28 host.
- The install test on RHEL 8 and the checklist steps of ADR-63 (owner's).
- Line-by-line reading of the files listed in section 10; the Analyst's tools other than `exp_parse.py`, `collist.py`,
  `num_decode.py`.
- Data Pump files and the work folder beyond the two `t_rows_exp*_18c.dmp` files read through `ORADMP_WORK` by the
  T-15 tests and my two scripts. Nothing named `oradump` was read, fetched, searched for or installed.

## git status

Before (start of the session): `?? docs/prompts/review-04-t13h-t15.md`. After: `?? docs/prompts/review-04-t13h-t15.md`
and `?? docs/review/review-04-t13h-t15.md` (this file); nothing else. Every experiment ran in a scratch folder under
`/var/tmp` (clone, targets, venvs, release root, fuzz corpora); nothing under `crates/`, `python/`, `corpus/`, `tests/`,
`fuzz/`, `tools/` or `docs/` other than this file was written.
