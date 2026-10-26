# Review 06a: T-13j, T-15b, T-18, T-18b and the 0.1.0a3 candidate

Reviewer session 6a, 2026-10-24. Prompt: `docs/prompts/review-06a-t13j-t18b.md`.

At the start and at the end of the session:

    $ git rev-parse HEAD
    bc2804e3c74387f76c3d74982529f397b208b0b6
    $ git status --short
    (empty)

**Reviewed: `d4d20eff66d221e6d6f167b1bd2ac6e680353b43`** ("Version 0.1.0-alpha.3"). `HEAD` is `bc2804e` (Analyst
session 13, documents only): `git diff --stat d4d20ef HEAD -- . ':!docs'` is **empty**, so HEAD does not change the
candidate. Note that T-13j's code is split between `1c17633` ("Analyst 12": `io.rs`, `rows.rs`, `t14_damage.rs`,
`t15_memory.rs`, `lib.rs`, `check_dist.py` and the tests) and `460aef5` (README, CONTRIBUTING, note).

Method. Everything under `/var/tmp/rev06` (size at the end: **11 GB**, almost all Cargo target folders; it can be
deleted): `src` (clean `git clone --no-local`, checked out at `d4d20ef`: checks, benchmarks, memory test), `mut`
(second clone at `d4d20ef` for the mutations; each mutation applied alone by a script that asserts exactly one
occurrence of the original text, runs the tests, then `git checkout --` and asserts `git status` is clean),
`release` (`ORADMP_BUILD_ROOT` of the release build), own venvs. `CARGO_BUILD_JOBS=2`, `TMPDIR=/var/tmp/oradmp-tmp`,
`cargo nextest run -j 2`, one build at a time. No `uv run` in the repository; the repository's
`python/oradmp/_core.abi3.so` was not touched (mtime 08:46:38, before this session started). `ORADMP_WORK` was used
only by the memory and throughput tests (and `sha256sum` / `stat` of the three files); nothing from it is quoted
here beyond sizes, hashes and the `exp` log's row line.

Machine conditions: 8 cores, 11.7 GB RAM. **Not met in full: an Oracle XE container was running the whole session**
(54 processes of uid 54321, `ora_pmon_xe` etc., started 06:18, idle, about 0 % CPU), although the prompt says none
is running. VS Code server and this agent were also running. Each measurement started only after the 1-minute load
average was below 0.5 (seen: 0.10 to 0.64).

## Summary

- **No blocking findings for a3.** The shipped files are right: release build reproducible, `check_dist: ok`,
  version `0.1.0a3`, entry point present, no home paths, no probe; full checks green with the expected counts; the
  installed wheel works in three venvs and the `oradmp` command does what ADR-72 (as amended) says.
- **Should fix: 2.** SF-1: `tests/test_cli.py` cannot run against an installed wheel (4 failures from a git clone, 1
  from copied folders), so checklist step 7 cannot pass as written; the wheel itself passes the same tests once the
  test points at it. SF-2: seven CLI mutations survive `tests/test_cli.py` (suffix `.parquet`, `-n -1`,
  `--all -o -`, `--version`, `--trim-char`, `--number-type`, name escaping).
- Every mutation the T-13j note names fails the test it names (s1o, s1e, s1i, line_start, m10 to m21, README link,
  README version); every T-18 / T-18b mutation the prompt and the plan ask for fails a test in `tests/test_cli.py`.
- Benchmarks and memory: every figure within 9 % of the Implementer's (most within 4 %); I consider them
  **reproducible** and fit to be the new baseline (with the caveat about the idle Oracle container).

## Blocking

None.

## Should fix

### SF-1. `tests/test_cli.py` tests the source tree, not the installed wheel; step 7 cannot pass as written

`tests/test_cli.py:717, :732, :747, :780` run the tool in a subprocess with `PYTHONPATH=str(ROOT / "python")`, and
`:759` scans `ROOT / "python" / "oradmp" / *.py`. `PYTHONPATH` comes before `site-packages`, so a subprocess imports
`oradmp` from the checkout. A git clone has no `_core.abi3.so` there (it is git-ignored and built only by
`maturin develop`), so the import fails.

Shown, wheel `oradmp-0.1.0a3-cp311-abi3-manylinux_2_28_x86_64.whl` of my release build, `pytest -rs` from the build
clone `$ORADMP_BUILD_ROOT/src` (git clone of `d4d20ef`, the layout checklist step 7 prescribes since ADR-70 item 3):

| venv | result | skips |
|---|---|---|
| CPython 3.11.16, pyarrow 15.0.2, pandas 2.2.3 (numpy 1.26.4) | **4 failed**, 191 passed, 3 skipped | the 3 probe tests |
| CPython 3.14.7, pyarrow 26.0.0, pandas 3.1.0 | **4 failed**, 191 passed, 3 skipped | the 3 probe tests |
| CPython 3.13.5, pyarrow 26.0.0, no pandas | **4 failed**, 184 passed, 4 skipped | the 3 probe tests + `test_consumers.py` (no pandas) |

The four: `test_a_closed_stdout_ends_head_quietly_with_exit_0`, `test_a_closed_stdout_of_export_to_stdout_is_exit_3`,
`test_the_module_entry_is_the_same_tool`, `test_the_cli_does_not_load_pandas_when_it_runs`. The child's stderr:
`ImportError: cannot import name '_core' from partially initialized module 'oradmp' ...
(/var/tmp/rev06/release/src/python/oradmp/__init__.py)`.

In the owner's Rocky Linux 8 layout (copied `tests/`, `corpus/fixtures`, root files, no `python/`) the subprocess
tests pass, but `test_no_file_of_the_package_imports_pandas` fails (`_cli.py` not found under `ROOT/python`): **1
failed**, 193 passed, 4 skipped (3 probes + "not a git clone") on 3.11 / pyarrow 15.0.2; the same on the pyarrow-only
venv plus the pandas skip. So no layout gives a green run against an installed wheel.

That the wheel is fine: in a scratch clone with `env = dict(os.environ)` (no `PYTHONPATH`) the four tests **pass in
all three venvs** (4 passed each; `tool_command()` then runs `<venv>/bin/oradmp`, the installed console script). In
the dev tree (`maturin develop`) the tests pass because `ROOT/python` is the editable package itself: there they
test the dev build, which is what the T-18 / T-18b counts (196 / 198) measured. The T-18 note (j) checked the wheel
by hand (`--help`, `tables`, `export`, `head | head -1`), not with `pytest`.

Fix (test file only, not shipped): take the package's location from the import, e.g. `PKG =
Path(oradmp.__file__).resolve().parent`; subprocess `PYTHONPATH=str(PKG.parent)` (or no override when `oradmp` is
importable), and scan `PKG.glob("*.py")` in the pandas test, which then checks the shipped files. For a3 the
Architect and owner choose: (a) fix before the build (a test-only commit after this review; checklist step 1 allows
it as the owner's decision on a review finding; the wheel's content is unchanged, its hashes change with the commit
time), recommended, so step 7 is green as written; or (b) build `d4d20ef` (or a docs-only `HEAD`) and record in step
7 the four failures as explained by this finding.

### SF-2. Seven CLI mutations survive `tests/test_cli.py`

Each applied alone to `python/oradmp/_cli.py` of the mutation clone (dev build with probes), `pytest
tests/test_cli.py`: **44 passed** every time.

| mutation | line | what a user would see |
|---|---|---|
| suffix `.parquet` gives `"csv"` | `:458` | `export ... -o x.parquet` writes CSV into `x.parquet` |
| `-n` negative check off | `:498` | `head -n -1` not refused (today: `[E-CONFIG] -n must not be negative`, exit 2, checked by hand) |
| `--all -o -` refusal off | `:567` | `--all` writing into a folder named `-` |
| `--version` text `oradmp x` | `:225` | wrong version printed (today `oradmp 0.1.0a3`, checked by hand in all venvs) |
| `trim_char` always `False` | `:266` | `--trim-char` ignored |
| `number_types` never passed | `:287` | `--number-type` ignored |
| `_escape` returns `s` unchanged | `:127` | tab / newline / backslash in a name printed raw (ADR-72 item 5); no fixture has such a name, so the `esc()` comparison of `:54` cannot see it |

Only `--strict`, `--on-value-error`, `--on-number-overflow`, `--interval-as`, `--columns` and `--ordinal` are
exercised; `--number-default`, `--timestamp-unit`, `--max-lob-bytes`, `--unconfirmed-lob`, `--db-charset`,
`--client-charset` were not mutated by me but have no test either (by reading). Fix: unit tests of `_escape` / `_row`
(`"a\tb"`, `"a\\b"`, `"a\nb"`) and `_format_for` (`.csv`, `.parquet`, `.PARQUET`, `-`, `--all`); `--version` equals
`oradmp {oradmp.__version__}`; `head -n -1` and `export --all -o -` give `E-CONFIG` / 2; one test per pass-through
option that compares the tool's output with the API read under the same option (e.g. `T_TEXT` CHAR column with and
without `--trim-char`). Not blocking: the behaviour is right today (suffix, `-n`, `--version` checked by hand).

## 1. T-13j: mutations repeated alone

Core mutations: `cargo nextest run -p oradmp-core -j 2` (412 tests, 3 skipped). Binding: `maturin develop --features
test-probes`, full `pytest` (198). `check_dist.py`: `pytest tests/test_check_dist.py` (59). README: `pytest
tests/test_version.py tests/test_project_name.py`.

| id | mutation | failing (all others pass) |
|---|---|---|
| lob_be | `read_u64_le`: `from_le_bytes` -> `from_be_bytes` | the `read_u64_le` unit test and 78 fixture tests (79 failed) |
| lob_len7 | `read_u64_le`: `read_bytes(8)` -> `read_bytes(7)` | the same 79 |
| line_start | `.at(line_start)` on "file ends inside a line" | `t14_damage each_truncation_site_names_its_offset` only |
| row17_at (own) | rows.rs "row or end of rows" site `.at(at + 1)` | `each_truncation_site_names_its_offset` only |
| row17_row (own) | the same site `.in_row(self.rows)` (row 16 instead of 17) | `each_truncation_site_names_its_offset` only |
| s1o | review 05 SF-1 text in `pull_raw` | `test_the_warnings_are_complete_after_an_error_on_the_iterator_path` only |
| s1e | warnings copied only on `Ok` | both S-2 tests of `test_stream.py` + `test_cli.py::test_warnings_go_to_stderr_once...` |
| s1i | warnings not copied on an iterator error | the iterator S-2 test + `test_cli.py::test_warnings_go_to_stderr_once...` |
| m10 | sdist file-name rule `if False:` | `test_m10_...` only |
| m11a / m11b | sdist member old-name scan off / `allow = True` | `test_m11_...` only (each) |
| m12, m13, m14 | Requires-Dist / License-Expression / Requires-Python rule off | its own case only |
| m15 | required-files rule off | the **11** parametrised cases (ten of T-13j + `python/oradmp/_cli.py` of T-18) |
| m15 (`_cli.py` entry deleted) | | `test_m15_...[python/oradmp/_cli.py]` only |
| m16 | licence-file rule off | the 2 cases |
| m17, m18, m19, m20 | Metadata-Version / License-File / description / `License ::` rule off | its own case only |
| m21a / m21b | wheel METADATA missing / sdist PKG-INFO missing rule off | `test_m21_...` only (each) |
| m22, m23a, m23b, m24 | entry_points missing / items / sections / parse-error rule off | `test_m22`, `test_m23` (2 / 3 params), `test_m24` |
| outside rule | `if top not in (...)` off | `test_m7_...` and `test_m25_...` |
| README link | `[LICENSE-MIT](LICENSE-MIT)` added | `test_the_readme_has_no_relative_link` only |
| README version | first `oradmp==0.1.0a3` -> `0.1.0a2`; or the wheel name's version | `test_every_version_named_in_the_readme_...` only |
| README old name (own) | the old name added to a heading | `test_readme_has_the_old_name_only_in_the_one_line` only |
| **README prose version** (own) | "Alpha (version 0.1.0a2): ..." | **none** (N-1) |

The damage-test offsets were checked against the Analyst's tool, not the reader: `exp_parse.py
18c/t_num_exp_18c.dmp --rows`: `row 17 0x22ee: @22ee [2] c1 12 | @22f2 [5] c2 02 18 2e 3d`, `rows: 33; end marker ff
ff at 0x2455`, `0x254d line [4+LF] 'EXIT'`, `0x2552 ... 'EXIT'`; `18c/lob_exp_18c.dmp`: `LOB value of row 5 col 2
@0x4358 long [200000]`; `xxd -s 0x4356`: `fcff` at 0x4358, `400d0300 00000000` (= 200,000) at 0x435a. Each equals
the case table of `t14_damage.rs:270-400`. `grep -rn "inside a LOB length" crates/` is empty; the `lib.rs` module
comment names all five exempt calls; CONTRIBUTING and `tools/fuzz_run.sh:46-47` agree; the `t15_memory.rs` comment
gives the measured 1.78 to 1.96.

## 2. T-18 and T-18b against ADR-72 as amended (ADR-75)

Read: `_cli.py` (668 lines), `__main__.py`, `tests/test_cli.py` (44 tests), `check_dist.py` / `test_check_dist.py`
(m22 to m26), README "Command line", arch 3.5. Expected values in `test_cli.py` come from the API (`Dump.tables`,
`read_table`), from `truth_text.txt` / `truth_num.txt` (first three rows, row counts) and from the patched copies of
T-13i; none from the tool's output. The CSV expectation is `pyarrow.csv.write_csv` of the API table with hex
re-implemented in the test; the Parquet expectation is the API table after `timestamp[s] -> timestamp[ms]` with
metadata checked, and the read-back cast back to the API schema must equal the API table.

Mutations (each alone, `pytest tests/test_cli.py`, 44 tests):

| mutation | failing tests |
|---|---|
| **usage error exit 2 -> 1** (`UsageError: 1`) | 9 (every exit-2 test, refused type, stopped-scan, existing target, ...) |
| **data error line to stdout** (exit-1 errors written with `_out`) | 6 (`test_exit_1_*` x4, `--all` first-error, the (g) warning test) |
| **stopped-scan rule removed** (`_read_table`) | `test_exit_1_a_table_not_found_in_a_scan_that_stopped_gives_the_stop_error` |
| stop error of `tables` / `--all` not reported (`_report_stop`) | `test_exit_1_a_scan_that_stopped_prints_the_stop_error_after_the_tables` |
| **DATE conversion removed**, test side (`parquet_form` keeps `s`) | `test_export_parquet_and_csv_equal_the_api_table[18c]`, `[11g]` |
| **DATE conversion changed**, tool side (`ParquetWriter(..., coerce_timestamps="us")`) | the same two |
| **`-q` ignored** | 3 (scan warnings, (g), T-18b (d)) |
| `OSError` out of `EXIT_CODES` | 5 (walk, three exit-3, closed stdout of `export -o -`) |
| `UsageError` out of `EXIT_CODES` | 10 (walk + the exit-2 tests) |
| partial file not removed | 7 |
| reader warnings not printed | (g) and T-18b (d) |
| closed-pipe handling off | both closed-pipe subprocess tests |
| warnings to stdout (own) | 9 |
| hex lower case (own) | both export comparisons |
| `head` one row too many (own) | `test_head_defaults_to_ten_rows...` |
| `--all` existing-file check off (own) | `test_export_all_refuses_an_existing_file...` |
| `StrictError` -> 2, `InternalError` -> 1, interrupt -> 1 (own) | the strict test; the walk; both interrupt tests |
| CSV preflight off (own) | the refused-type test `[csv]` |
| `warnings` filter off (own) | the (g) `recwarn` test |
| scan warnings not printed (own) | `test_the_scan_warnings_print_once_after_the_scan` |
| `survey --no-names` ignored (own) | `test_survey_through_the_new_entry...` |
| survivors | SF-2 |

The prompt's "DATE conversion removed" has no direct target: the tool converts nothing (pyarrow's writer stores
seconds as milliseconds, ADR-75 item 3). I mutated both sides, as above; both are caught.

Checked by hand on the installed wheel (copied fixtures, all three venvs; the pyarrow-only one is 3.13 / 26.0.0):
`oradmp --version` = `oradmp 0.1.0a3`; `--help` exit 0; `tables`, `columns`, `head -n 1`, `head --ordinal 1`,
`export TABLE -o x.csv --format csv`, `-o x.parquet` (suffix), `-o -`, `export --all -o DIR --format parquet`
(manifest on stdout), `survey FILE --no-names --no-read`, `python -m oradmp ...` all exit 0 with nothing on stderr;
`oradmp head ... T_TEXT | head -1` exit 0, stderr empty. 18c `T_DATE` export: exit 1 with
`[E-BAD-DATETIME offset=0x22b6] ...` and no file left (the year-below-1 row); with `--on-value-error null`: exit 0,
one `oradmp: warning: [W-VALUE-NULLED offset=0x22b6] ...` on stderr, the Parquet file reads back as `timestamp[ms]`
and, cast to the API's `timestamp[s]` schema, equals the API table. 18c `T_NUM` `--on-value-error null`: exit 0,
stderr exactly the two warnings `W-VALUE-NULLED offset=0x230f ... row 19 [10 values]` and `W-NUMBER-ROUNDED
offset=0x23a5 ... [4 values ...]` (ADR-75 item 4 expected both), `-q` silent, 34 CSV lines. `survey missing.dmp`:
`ERROR: ...` on stdout, exit 1; `tables missing.dmp`: `oradmp: error: [E-IO] I/O error`, exit 3. On 3.13
`head FILE -n 1 TABLE` works too; the README's "table right after the file" is the documented contract.

README "Command line" against arch 3.5 and ADR-72 as amended: commands, the `--interval-as string` default,
hex for RAW / LONG RAW / BLOB, DATE as milliseconds, partial file and `--overwrite`, the `--all` file-name rule,
stdout / stderr, exit codes (with the stopped-scan rule and survey's 0 / 1), the widened `head` sentence and
`--version` all agree. Two small gaps: N-2 (the format from the suffix is not stated) and the option list of `README.md:169-171`
omits `--interval-as` from the list (it is named in the next sentence).

`--help` lists every option ADR-72 item 3 names; Data Pump, filters, SQL and pandas are absent; no `_core` import in
`_cli.py`; no `pandas` import anywhere in `python/oradmp/` (and the pandas-less venv ran CSV and Parquet exports).

## 3. Benchmarks and memory test (alone; page cache; release build of `d4d20ef`)

T_BIG input: 453,304,320 bytes, SHA-256 `9fe3d960...a6bda1` = `SHA256SUMS_big`; `exp` log: "T_BIG 5000000 rows
exported", "Export terminated successfully without warnings". `T_ROWS`: 10,035,200 / 9,838,592 bytes.

### Memory test, correct reader (all three files; peaks `VmHWM` kB, stop / end)

| run | T_ROWS conv | T_ROWS direct | T_BIG | result |
|---|---|---|---|---|
| 1 | 7592 / 7748, 1.0205 | 7512 / 7844, 1.0442 | 7556 / 7644, **1.0116** | pass |
| 2 | 7428 / 7920, 1.0662 | 7596 / 7708, 1.0147 | 7588 / 7812, **1.0295** | pass |
| 3 | 7556 / 7784, 1.0302 | 7528 / 7748, 1.0292 | 7580 / 7724, **1.0190** | pass |

Implementer: T_BIG 1.0016 / 1.0252 / 1.0137 (T-15b), 1.0246 (T-18b); peaks 7,472 to 7,684 kB. Mine: peaks 7,556
to 7,812 kB (end peaks at most +1.7 % above T-15b's highest, +0.9 % above the 4.3 baseline's 7,848 / 7,820 for
`T_ROWS`). Within 15 %.

### Memory test, mutation b = 1 (`TableSink::row_end` keeps 1 byte per row in a `static Mutex<Vec<u8>>`; reverted)

| setting | T_BIG stop / end peak, ratio | result |
|---|---|---|
| `ORADMP_WORK` holding only a symlink to T_BIG, run 1 | 7756 / 13520, **1.7432** | FAILS |
| the same, run 2 | 7712 / 14880, **1.9295** | FAILS |
| the same, run 3 | 7752 / 13560, **1.7492** | FAILS |
| all three files (T_ROWS 1.0110 and 1.0425 "ok") | 7708 / 13708, **1.7784** | FAILS; the message lists all three files |

Implementer: 1.78 to 1.96. My lowest (1.743) is 2 % below theirs: within 15 %. One byte per row is resolved on
T_BIG; T_ROWS does not see it, as expected.

### Throughput (`t15_throughput`, one run; no figure disagreed, so no extra runs)

| file | quantity | mine (runs) | Implementer (T-15b) | diff | 4.2 baseline |
|---|---|---|---|---|---|
| T_ROWS conv | scan | 846.1 MB/s (12.9, 11.9, 11.8 ms) | 809.1 | +4.6 % | 803 (+5.4 %) |
| | read | 2,359,224 rows/s, 236.4 MB/s | 2.17 M | +8.7 % | 2.15 M (+9.7 %) |
| T_ROWS direct | scan | 888.6 MB/s | 886.7 | +0.2 % | 830 (+7.1 %) |
| | read | 2,456,653 rows/s, 241.3 MB/s | 2.45 M | +0.3 % | 2.32 M (+5.9 %) |
| T_BIG | scan | 779.0 MB/s (567.0, 593.7, 581.9 ms) | 785.3 | -0.8 % | none |
| | read | 2,232,222 rows/s, 202.4 MB/s (2.224, 2.240, 2.255 s) | 2,195,749, 199.1 MB/s | +1.7 % | none |

(T-15b's two unnamed T_ROWS figures read as conventional / direct, as `benchmarks.md` does; my run agrees with that
reading.)

### Micro-benchmarks (`cargo bench -p oradmp-core --bench decode`, once; mean)

| benchmark | mine | T-18b | diff | 4.1 baseline ("note") |
|---|---|---|---|---|
| `number/OraNumber::decode` | 1.4645 us | 1.4626 us | +0.1 % | 1.528 (4.2 % faster) |
| `datetime/decode_date` | 141.31 ns | 145.18 ns | -2.7 % | 151.1 |
| `datetime/decode_timestamp` | 254.71 ns | 262.61 ns | -3.0 % | 260.2 |
| `text_cp1251/Charset::decode (Win1251)` | 170.28 us | 173.67 us | -2.0 % | 188.5 |
| `read_table/R_STAT` | 970.26 us | 1.0034 ms | -3.3 % | 1.0306 ms |
| `read_table/T_NUM` | 37.911 us | 36.669 us | +3.4 % | 39.24 |
| `read_table/T_TEXT` | 37.606 us | 39.685 us | -5.2 % | 41.86 |

Verdict on the numbers: every repeated figure lies within 9 % of the Implementer's (rule 1 asks 15 %), and every one
is at or above the current baseline (no regression under rule 3 or rule 4). **I consider them reproducible** and fit
to become the new baseline (code of `460aef5` = the core sources of `d4d20ef`), with the conditions line saying that
an idle Oracle XE container was running during this repeat. Not repeated: `tools/stream_memory.py`.

## 4. The a3 candidate

### Release build (`ORADMP_BUILD_ROOT=/var/tmp/rev06/release tools/release_build.sh d4d20eff... --twice`)

- `check_dist: ok` twice; **`identical:` wheel and sdist; "reproducible"**. Commit `d4d20eff66d2...`; rustc 1.99.0
  (b940084d7 2026-09-28); maturin 1.15.0; zig 0.17.0; epoch 1792847341.
- `e905e103eb5d73e836c13f008aeff41d0d698375691182eb561cf6a51d342f86  oradmp-0.1.0a3-cp311-abi3-manylinux_2_28_x86_64.whl`
  `bf480eaf0818ac3828f5114e4623071878f48136d661d78b3f8d3d1f416ace66  oradmp-0.1.0a3.tar.gz`
  (my build root; the SBOM holds the root path, so the owner's step 6 hashes will differ: compare uploads only with
  the owner's own `SHA256SUMS`).
- My own scans (outside `check_dist.py`): `/home/`, `/root/`, `/Users/`, `C:\Users` (any case) in the unpacked
  wheel, the unpacked sdist, the raw `.whl` and the decompressed `.tar.gz`: 0 hits each; `_probe_panic` in the
  release `.so`: 0; build-root string (`rev06`, `/var/tmp`) in the `.so`: 0 (remapped to `/build`, `/cargo`); the
  SBOM holds `path+file:///var/tmp/rev06/release/src/crates/...` (expected, ADR-70 item 5).
- Version `0.1.0a3` in both file names, `Version: 0.1.0a3` in METADATA and PKG-INFO; `Metadata-Version: 2.4`,
  `Requires-Dist: pyarrow>=15`, `Requires-Python: >=3.11`, `License-Expression: MIT OR Apache-2.0`, both
  `License-File`; METADATA description = `README.md` (diff: one trailing newline). No `Project-URL` (ADR-76 item 4).
- `oradmp-0.1.0a3.dist-info/entry_points.txt` = `[console_scripts]` / `oradmp=oradmp._cli:main`; the wheel holds
  `oradmp/` and the dist-info only (`_cli.py` and `__main__.py` included).
- Version files: `Cargo.toml` `0.1.0-alpha.3`; `Cargo.lock` `oradmp-core` and `oradmp-py` `0.1.0-alpha.3`;
  `fuzz/Cargo.lock` `oradmp-core` `0.1.0-alpha.3`; README `0.1.0a3` four times.

### Checklist step 3 in the clean clone

Fixture checksums OK (11g, 11g-cp1251, 18c); `cargo fmt --all --check` ok; `cargo clippy --workspace --all-targets
-- -D warnings` ok, also `--all-features`, also plain `cargo clippy -- -D warnings`; `cargo nextest run -j 2`:
**412 passed, 3 skipped**; `cargo deny check`: `advisories ok, bans ok, licenses ok, sources ok` (one warning:
`Zlib` allowed but not encountered; the advisory database is whatever the VM had or fetched, date not checked);
`ruff check` (0.17.2) ok, `ruff format --check` ok (32 files), `mypy` (2.4.0) ok (22 files); `pytest -rs` on a
`--features test-probes` build: **198 passed, 0 skipped**. Counts equal the T-18b note (412 + 3; 198 = T-13j 143
+ 42 CLI + 11 `check_dist` cases of T-18 + 2 of T-18b).

### Installed wheel in three fresh venvs

See SF-1 for `pytest` (191 / 191 / 184 passed; exactly the three probe skips, plus the pandas skip in the pandas-less
venv; the four SF-1 failures in each). On pyarrow 15.0.2 `test_interval.py` and `test_consumers.py` pass (checklist
step 7's note). The `oradmp` command from each venv's `bin/`: section 2. Source build once: the sdist installed by
`uv pip install` in a fresh 3.11 venv (pyarrow 26), `oradmp.__version__` `0.1.0a3`, `oradmp tables` and `python -m
oradmp survey --no-names` on a copied fixture work.

## 5. Release steps for a3 (ADR-67 as amended, checklist with the session 18 and 19 lines)

Wrong or missing for a3 (none blocks; the owner substitutes as he goes):

1. Step 7 cannot pass as written: SF-1 (decide (a) or (b) before step 6).
2. The plan's 6a item 4 and step 7 expect "exactly the three probe skips" also in the **pyarrow-only** venv; that
   venv necessarily skips `test_consumers.py` (pandas). Say so in step 7.
3. Literal a2 values remain in steps that a3 repeats: step 5 (`grep -n 0.1.0a2 README.md`), step 9 (`v0.1.0a2`),
   step 10 (b) to (d) (`"oradmp==0.1.0a2"` in the `pip download` / `pip install` commands). Write `<v>`.
4. Step 8 (a real RHEL / Oracle Linux 8 host, now also `oradmp tables` per ADR-72 item 11) is still open since a1
   (Q-4).
5. Step 1: `HEAD` will move with the Architect session 20 commit; building `HEAD` is fine while `git diff --stat
   d4d20ef HEAD -- . ':!docs'` stays empty (it is now), but its epoch and hashes differ from a build of `d4d20ef`.
   Tag the commit the step 6 log prints.
6. Linux x86_64 only, no CI, no Project-URL: consistent across ADR-76, the plan's a3 section, the README ("Wheels:
   Linux x86_64 only", "Other platforms") and METADATA. Nothing missing that I could find.

## Notes

- **N-1** `tests/test_version.py:23`: the README check sees only `oradmp==<v>` and `oradmp-<v>`; a version written
  otherwise ("Alpha (version 0.1.0a2)") survives. Plan T-13j (h) asks exactly those two forms, so it is met; the
  prompt's "every version in the README" is wider. Optional: also match `\b\d+\.\d+\.\d+(?:a|b|rc)\d+\b` anywhere.
- **N-2** README "Command line" does not say that `export` takes the format from the suffix (`.csv`, `.parquet`),
  else `E-CONFIG` (arch 3.5 does; `--help` does).
- **N-3** `oradmp tables missing.dmp` and `oradmp tables /etc/shadow` both print `oradmp: error: [E-IO] I/O error`
  (exit 3): no reason and no path. The text is the core's `OSError` (`survey` shows `[Errno 2] [E-IO] I/O error`).
  A user cannot tell "missing" from "permission denied". A path or `strerror` is not a value of the file (arch 3.4).
- **N-4** `67f8bd2` (T-18b) already carries the README's `0.1.0a3` lines while `Cargo.toml` says `0.1.0-alpha.2`, so
  `test_every_version_named_in_the_readme_...` fails on that commit (the note asked for `git add -p`). Only the
  version commit is a candidate; noise for a bisect only.
- **N-5** Twice during full-suite mutation runs (line_start, row17_at) an untracked
  `crates/oradmp-core/tests/t14_damage.proptest-regressions` appeared while nextest reported only
  `each_truncation_site_names_its_offset` failing. Replaying its seed passes with and without the mutation;
  `PROPTEST_CASES=20000` on the candidate passes. Not a candidate defect; I could not explain it. Mutation procedures
  should check `git status` after each run (mine did, which is how it was seen).
- **N-6** Numbers: T-13j's "m15, the 10 parametrised cases" is now 11 (T-18 added `python/oradmp/_cli.py`); the T-18
  note's "42 tests" + T-18b's 2 = the 44 of `test_cli.py`; consistent.
- **N-7** The T-18 note's points 1, 3, 6 (Parquet DATE as ms, `head` without counted warnings, the open choices) are
  ruled by ADR-75; the code and README behave as ruled (section 2).
- **N-8** Measurement conditions: the idle Oracle XE container (see the top). Record it in `benchmarks.md` 2 with the
  new baseline, or repeat once without it if the owner prefers clean conditions.

## Not checked

- PyPI and TestPyPI behaviour (upload, index pages, `pip download`, index lag): no network uploads, no tokens.
- macOS and Windows (no wheels, no machine; `os.replace` and paths on Windows untested); a real RHEL 8 host and the
  Rocky Linux 8 container (no Docker by rule); I emulated the copied-folder layout on the VM only.
- `tools/stream_memory.py`; `tools/fuzz_run.sh`; the b = 4 and b = 8 memory mutations (T-15b / T-18b ran them).
- A real `SIGINT` to the tool (only the monkeypatched `KeyboardInterrupt` of the tests); a full disk; a closed
  stderr.
- The CSV writer's refused-type list on pyarrow 15.0.2 and 26.0.0 beyond what the tests probe (I did not redo the
  T-18 note's type probe by hand).
- The `cargo deny` advisory database date; `cargo deny` against the network-dependent checks of step 4.
- Analyst session 13's commit `bc2804e` (documents only, outside the candidate's shipped files) and T-16 (6b).
- I did not re-read the whole T-13j / T-18 diffs line by line beyond `_cli.py`, the tests named above, `check_dist.py`,
  `io.rs` / `rows.rs` changes, `t15_memory.rs`, README and the version files.

Scratch: `/var/tmp/rev06`, **11 GB** at the end (Cargo target folders of `src`, `mut`, `release`; venvs). Nothing
under it is needed after this report.
