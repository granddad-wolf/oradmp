# Review 06a-closure: T-18c and the real 0.1.0a3 candidate

Reviewer session 6a-closure, 2026-10-24. Prompt: `docs/prompts/review-06a-closure-t18c.md`.

**Candidate C = `ff3d6b25c360f93eeb8c1305927dd6cf3550610b`** ("T-18c: CLI tests find the installed package, mutation
tests, I/O error text, README"). HEAD at the start was `baa6b1befb05978c5e25e325ece974cd3aaa6db4` (the prompt
commit); `git diff --stat ff3d6b2 baa6b1b -- . ':!docs'` is empty, so HEAD and C build the same files.

**Verdict: no blocking findings for a3 at `ff3d6b25c360f93eeb8c1305927dd6cf3550610b`, safe to upload.** The two
should-fix items below concern a test that checks the wrong error shape (a case the command line cannot reach) and
checklist step 7 run as root in the Rocky Linux 8 container. Neither changes what the wheel does for a user of a3.

## Method and conditions

- Scratch only under `/var/tmp/rev06c` (**5.7 GB** at the end: `src` 4.1 GB is mostly the Cargo target folder,
  `release` 0.8 GB, five venvs; none of it is needed after this report). `CARGO_BUILD_JOBS=2`,
  `TMPDIR=/var/tmp/oradmp-tmp`, nextest `-j 2`, one build at a time.
- `src`: `git clone --no-local` of the repository, checked out at C (step 3 checks, dev build with
  `--features test-probes`, the mutations). `release`: `ORADMP_BUILD_ROOT` of `tools/release_build.sh C --twice`, run
  from the `src` clone. `copied`: the copied-folder layout (`tests/`, `corpus/fixtures/`, `Cargo.toml`,
  `Cargo.lock`, `README.md`, `pyproject.toml`, `deny.toml`, `CONTRIBUTING.md`, both licences, `tools/`,
  `fuzz/Cargo.lock`, `docs/design/implementer-notes.md`; no `python/`, no `.git`).
- Mutations were run by a script that checks the original text occurs exactly once, applies the change, runs
  `pytest tests/test_cli.py tests/test_version.py tests/test_survey.py`, restores with `git checkout --`, and checks
  that `git status --porcelain` is empty. It was clean after each of the 40 runs.
- The repository was not touched: `git status` was clean before and after. I did not run `uv run` in the repository.
  The repository's `python/oradmp/_core.abi3.so` has mtime 10:57:57, from before this session, and I did not change it.
- **`docker ps`: three containers were running the whole session**: `ora11ru`, `ora11` (gvenzl/oracle-xe:11) and
  `ora18` (gvenzl/oracle-xe:18), "Up 5 hours". The prompt expects none. This session took no measurements, so they
  do not affect any result here. The 1-minute load average was 0.1 to 1.3.

## 1. The diff (Done when (a) to (g), ADR-77 item 4, ADR-72 as amended)

`git diff --stat d4d20ef C -- . ':!docs'` shows:

    CLAUDE.md | 4 +-;  README.md | 19;  python/oradmp/_cli.py | 22;  tests/test_cli.py | 258;  tests/test_version.py | 14

`git diff --stat d4d20ef C -- crates/` is empty, so no Rust re-review and no benchmark are due. `CLAUDE.md` is the
one extra file (note N-1). The version stays `0.1.0-alpha.3` / `0.1.0a3`, and there is no version commit.

I read `_cli.py` in full and every new test:

- (a) SF-1: `PKG = Path(oradmp.__file__).resolve().parent` and `child_env()` with `PYTHONPATH=str(PKG.parent)`.
  `grep 'ROOT / "python"' tests/test_cli.py` finds nothing, and `ROOT` is no longer imported. The pandas scan uses
  `PKG.glob("*.py")` and asserts that `_cli.py` and `__init__.py` are in it. `test_the_subprocess_imports_the_same_package`
  is present. As asked.
- (b) SF-2: one test per survivor. `test_every_read_option_reaches_the_api` covers 13 cases (all of ADR-72 item 3,
  with `--number-type` once and repeated), and `test_columns_reaches_the_api` covers `head` and `export`. The
  `--trim-char` and `--number-type` tests first assert that the two API reads differ, as asked. Their expected values
  come from the API's own read of the fixture. That is right here: the option has to reach the reader, and the values
  themselves are checked by the truth tests.
- (c) N-3: `_path_text`, `_api_io_text`, `_error_text(e, file)` and `_state.file`, as ADR-77 item 4 says, with one
  exception (SF-1 below). The output side is unchanged and `survey` is unchanged.
- (d) The README suffix sentence is word for word the plan's text. `--interval-as` is in the option list. The
  exit-code bullet is split, and its "3" names the line form. The reflow changed the wording "a missing dump" to "a
  missing or unreadable dump"; that is fine. One reflowed line, README:173, is 128 characters long (cosmetic).
- (e) `ANY_PRERELEASE` and its self-test are as asked (note N-4).

## 2. Mutations (each alone, dev build with probes, at C)

| mutation (in `_cli.py` unless named) | failing tests |
|---|---|
| suffix `.parquet` returns `"csv"` | `test_the_format_comes_from_the_option_else_the_suffix_in_any_case`, `test_export_takes_parquet_from_the_suffix` |
| `-n < 0` guard off | **pytest aborts** (rc -6, i.e. SIGABRT); see N-2 |
| `--all -o -` guard off | `test_export_all_to_stdout_is_refused_and_creates_nothing` |
| `--version` text `oradmp x` | `test_version_prints_the_package_version` |
| `trim_char` always False | `test_trim_char_reaches_the_reader`, `...reaches_the_api[trim_char8]` |
| `number_types` not passed | `test_number_type_reaches_the_reader`, `...[number_types4]`, `...[number_types5]` |
| `--number-type` keeps only the last one | `...[number_types5]` |
| `_escape`: no tab / no newline / no backslash (three runs) | `test_the_escape_of_names` (each) |
| `.lower()` removed | `test_the_format_comes_from_the_option_else_the_suffix_in_any_case` |
| `.csv` branch off | 15 tests |
| explicit `--format` ignored | 3 tests, including the `_format_for` test |
| `-` not CSV / `--all` not CSV | 4 / 6 tests, including the `_format_for` test |
| each of `on_value_error`, `on_number_overflow`, `number_default`, `timestamp_unit`, `max_lob_bytes`, `unconfirmed_lob`, `db_charset`, `client_charset` dropped (8 runs) | its case of `test_every_read_option_reaches_the_api` (plus an older test for `on_value_error` and `on_number_overflow`) |
| `strict` always False / `interval_as` always `"string"` / `--columns` ignored | its pass-through case plus older tests (2 / 3 / 5 failures) |
| N-3: path left out | `test_exit_3_a_missing_dump_names_the_reason_and_the_path`, `..._an_unreadable_dump_...`, `test_the_io_error_text_of_the_api_error` |
| N-3: OS text replaced by `I/O error` | the same three |
| N-3: no-errno text lost / path not escaped / offset prefix lost | `test_the_io_error_text_of_the_api_error` |
| N-3: `_state.file` not set | the two exit-3 tests |
| N-3: the `[E-IO` branch off | the three tests |
| `EXIT_CODES[OSError]` = 1 | 7 tests |
| `_survey.py`: OSError branch returns 3 / re-raises (so it goes through the tool's error line) | `test_survey_of_a_missing_file_keeps_its_own_text` |
| README: "Alpha (version 0.1.0a2)" appended | `test_every_pre_release_version_in_the_readme_is_the_cargo_version` |
| README: suffix sentence removed / `--interval-as` removed from the list / exit-3 example removed | **none (survive)**; see N-3 |
| SF-1, git clone with the installed wheel (3.11): `child_env` back to `ROOT/python` | 5 fail: the two closed-stdout tests, `test_the_subprocess_imports_the_same_package`, `test_the_module_entry_is_the_same_tool`, `test_the_cli_does_not_load_pandas_when_it_runs` |
| SF-1, copied layout (3.11): pandas scan back to `ROOT/python/oradmp` | `test_no_file_of_the_package_imports_pandas` |

Every mutation in the T-18c note and in the plan's closure item 2 fails a named test, and `git status` was clean after
each run. The only mutations that survive are the README prose ones, which no test is meant to guard.

## 3. Installed wheel, both layouts (wheel of section 4)

`pip install --no-deps` of the release wheel into fresh venvs, then `python -m pytest -rs -p no:cacheprovider`:

| venv | git clone (`release/src`) | copied folder |
|---|---|---|
| CPython 3.11.16, pyarrow 15.0.2, pandas 2.2.3, numpy 1.26.4 | 224 passed, 3 skipped | 223 passed, 4 skipped |
| CPython 3.14.7, pyarrow 26.0.0, pandas 3.1.0 | 224 passed, 3 skipped | 223 passed, 4 skipped |
| CPython 3.13.5, pyarrow 26.0.0, no pandas | 217 passed, 4 skipped | 216 passed, 5 skipped |

There were 0 failures in all six runs. The skips are exactly the expected ones:
- the three `test_internal.py` probe tests ("built without --features test-probes"), everywhere;
- `test_project_name.py:34` "not a git clone", in the copied folders only;
- `test_consumers.py:18` (no pandas), in the 3.13 venv only.

Total 227 = 224 + 3, the same count as the dev build. SF-1 of review 06a is fixed.

## 4. Release build and checklist step 3

`tools/release_build.sh ff3d6b25... --twice` (root `/var/tmp/rev06c/release`, exit 0) printed `commit:` = C and
`check_dist: ok` twice, and `identical` for the wheel and the sdist ("reproducible"). Epoch 1792857590. Hashes
(void for the upload: the owner's build root differs, and the SBOM holds the root):

    0279e692230b387c6606448d35a07feeea135097b6c7775501801bd128edfbca  oradmp-0.1.0a3-cp311-abi3-manylinux_2_28_x86_64.whl
    36bc571d75d6b6bb66968e31ee4144ba46324aa9efa174888ea973ed6e9293e5  oradmp-0.1.0a3.tar.gz

- METADATA and PKG-INFO: `Version: 0.1.0a3`. The METADATA description is `README.md`, apart from one trailing
  newline. There is no `Project-URL`. `entry_points.txt` is exactly `[console_scripts]` /
  `oradmp=oradmp._cli:main`. The wheel holds only `oradmp/` and the dist-info, and its `_cli.py` is byte-equal to C's.
- `/home/`, `/root/`, `/Users/`, `C:\Users`: no hit in any member of the wheel or the sdist (`.so`, SBOM and
  METADATA included). The SBOM holds `path+file:///var/tmp/rev06c/release/...`, which is expected (ADR-70 item 5).
  `_probe_panic` does not occur in the release `.so`. `CLAUDE.md` is not in the sdist.
- **Step 3 in the clean clone at C** (the release's step 3):
  - fixture checksums OK (11g, 11g-cp1251, 18c); `cargo fmt --all --check` ok;
  - `cargo clippy --workspace --all-targets -- -D warnings` ok, also `--all-features`, also plain
    `cargo clippy -- -D warnings`;
  - `cargo nextest run -j 2`: **412 passed, 3 skipped**;
  - `cargo deny check`: advisories, bans, licenses and sources all ok (the known warning: `Zlib` is allowed but not
    used; I did not check the date of the advisory database);
  - `ruff check` (0.17.1) ok; `ruff format --check` ok (32 files); `mypy` (2.4.0) ok (22 files);
  - `pytest -rs` on the `--features test-probes` build: **227 passed, 0 skipped**.

  These are the T-18c note's counts: 198 + 29, and 47 to 76 tests collected in `test_cli.py` + `test_version.py`.
- Checklist 3b ("not due"): `git diff --stat 460aef5 C -- crates/oradmp-core/src rust-toolchain.toml` is empty, and
  `git diff 460aef5 C -- Cargo.toml` shows only the version line. Step 5's `cargo metadata` line prints only
  `"version":"0.1.0-alpha.3"`, and `grep -nE '0\.1\.0a[0-9]+' README.md` finds only `0.1.0a3` (4 lines).
- Source build once: `uv pip install` of the sdist into a fresh 3.11 venv gives `0.1.0a3`; `oradmp tables`,
  `python -m oradmp survey --no-names` and the missing-file line all work.

## 5. The command line from the installed wheel (copied fixtures, all three venvs)

- `oradmp --version` prints `oradmp 0.1.0a3` (exit 0). `--help` prints the five commands. `tables` lists `T_TEXT`,
  10 rows. `columns` shows the 4 columns. `head -n 3` gives CSV with the header, and `head ... | head -1` exits 0
  with an empty stderr.
- `export -o t.csv`, `-o t.parquet` and `-o t.PARQUET` (no `--format`) all exit 0. Both Parquet files start with
  `PAR1` and read back as (10, 4) in the pyarrow-only venv. `-o t.txt` gives `[E-CONFIG] the format is not known...`
  (exit 2).
- `tables`, `columns`, `head`, `export T -o out/m.csv`, `export --all -o out` and `export T -o -` on `missing.dmp`
  each give exit 3, empty stdout and `oradmp: error: [E-IO] No such file or directory: missing.dmp`, and `out/`
  stays empty. An absolute path is printed as given. A mode-000 copy gives
  `oradmp: error: [E-IO] Permission denied: locked.dmp` (exit 3; not run as root). `tables .` gives
  `[E-IO offset=0x0] Is a directory: .` (the offset prefix is kept). A tab in the name is printed as `a\tb.dmp`.
- `survey missing.dmp` prints `oradmp survey` / `ERROR: [Errno 2] [E-IO] I/O error` on stdout and exits 1
  (unchanged).
- Output side: a read-only folder gives `[E-IO] Permission denied: ro/t.csv.oradmp-partial` (exit 3).
- Usage errors give exit 2: `oradmp frob`, `head FILE` with no table, `head -n -1`, `export --all -o -`, and an
  existing target without `--overwrite`.
- Every example in the README's "Command line" and "Survey" sections works as written with `FILE`, `TABLE`, `PATH`
  and `DIR` filled in: `tables`, `columns`, `head -n`, `export -o PATH --format csv|parquet`, `export -o -`,
  `export --all -o DIR [--format parquet]` (the manifest, `T_TEXT.csv` / `T_TEXT.parquet`), `survey --no-names
  --no-read`, `python -m oradmp survey`, and `--ordinal 1`.

## Blocking

None.

## Should fix

### SF-1. The "no errno" case of the I/O text is tested with an error shape the API never produces

`tests/test_cli.py`, `test_the_io_error_text_of_the_api_error`, asserts:

    text(OSError(None, "[E-IO] I/O error"), "a.dmp") == "[E-IO] I/O error: a.dmp"

The API never builds that shape. `python/oradmp/_errors.py:157-159` (`make_error`) builds `OSError(message)` when
there is no errno, and a one-argument `OSError` has `strerror = None`. So `_cli.py:632`
(`isinstance(strerror, str) and strerror.startswith("[E-IO")`) is false for the real error, the old branch at `:636`
runs, and the line comes out with the prefix twice and no file.

Shown with the installed wheel:

    _cli._error_text(make_error("E-IO", "[E-IO] I/O error", None, ..., None), "a.dmp")
    -> '[E-IO] [E-IO] I/O error'
    (the same with an offset: '[E-IO] [E-IO offset=0x10] I/O error')

ADR-77 item 4 asks for `[E-IO] I/O error: FILE`. The only no-errno source I found is a path with a NUL byte
(`oradmp.open("a\x00b.dmp")` raises `OSError('[E-IO] I/O error')` with `errno` None), and argv cannot carry a NUL byte.
So for a3's command line the case is practically unreachable, and that is why this is not blocking. The test still
passes for the wrong reason: it claims coverage of the API's no-errno form and covers a different form.

Fix (tests and `_cli.py`, a4):
- In `_error_text`, take the text as `e.strerror if isinstance(e.strerror, str) else (e.args[0] if len(e.args) == 1
  and isinstance(e.args[0], str) else None)` before the `[E-IO` test, or key the branch on
  `getattr(e, "code", None) == "E-IO"`.
- Build the test's errors with `oradmp._errors.make_error("E-IO", ..., errno=None)` and `errno=2`, so they have the
  binding's real shape.

### SF-2. Checklist step 7 in the Rocky Linux 8 container as root: one more skip than the checklist expects

T-18c added `test_exit_3_an_unreadable_dump_names_the_reason_and_the_path`, skipped as root
(`reason="root can read a file of mode 000"`). a2's container run was as root (`/var/tmp/rhel-test/rhel_counts.txt`
is owned by root). If a3's step 7 runs the same way, both layouts show one skip more than step 7's "exactly the three
probe tests" (and "plus `test_project_name.py`" for copied folders).

The build clone `R/src` mounted read-only into a root container also has a problem with `test_project_name.py`. If
`git` is missing in the image, the test skips with "not a git clone". If `git` is present, it refuses a repository
owned by another uid ("dubious ownership"), and `tracked()` runs it with `check=True`, so the test fails rather than
skips. I could not run this (no Docker by rule); it is inference from the code and git's documented
`safe.directory` behaviour.

Fix (Architect, checklist text, before step 7): either run the container as the owner's uid (`--user "$(id -u):$(id -g)"`),
or list "as root also `test_exit_3_an_unreadable_dump_...` (root can read mode 000)" among the expected skips. For the
git-clone layout in a container, either `git config --global --add safe.directory <mount>` or accept the "not a git
clone" skip there. On the VM as a normal user, step 7 passes exactly as written (section 3).

## Notes

- **N-1** `git diff --stat d4d20ef C -- . ':!docs'` also lists `CLAUDE.md`. That is commit `9f1dc4d`, the owner's
  O-21 line for `.github/`. It is not shipped (not in the sdist or the wheel), and it is not a T-18c change. Closure
  item 1's list should allow for it. No action.
- **N-2** `head -n -1` with the guard off: confirmed. `python -m oradmp head ... -n -1` dies with
  `terminate called after throwing an instance of 'std::length_error'`, exit **134**, and pytest dies with it (rc -6).
  I accept it as "detected": the run is not green and the guard stays. But the failure names no test, and in CI it
  would look like a crash. Optional (later task): run that one case in a subprocess, so that the mutation gives a
  named failure (exit 134 against the expected 2).
- **N-3** README prose mutations survive: removing the suffix sentence, the `--interval-as` entry or the exit-3
  example fails nothing. As planned: T-18c (d) asks no test for the prose, and the Reviewer reads it (section 1: all
  three are present and correct).
- **N-4** `ANY_PRERELEASE` sees only PEP 440 pre-releases written `X.Y.Za|b|rcN`. A stale Cargo-form
  `0.1.0-alpha.2` or a stable `0.1.0` in prose would pass. That is what (e) specified, so the requirement is met.
- **N-5** Old behaviour, found here, not caused by T-18c. A dump whose **file name is not valid UTF-8**
  (`oradmp tables $'\xff.dmp'`, whether the file exists or not) ends with a Python traceback:
  `UnicodeEncodeError: ... surrogates not allowed while processing 'path'`, exit 1. Cause:
  `crates/oradmp-py/src/lib.rs:653` takes `path: String`. On Linux such names are plausible (CP1251 or Latin-1 file
  names copied from Windows). The API has the same problem (`oradmp.open` raises `UnicodeEncodeError`). ADR-72 item 8
  allows a traceback only for a bug of the tool, and this is one. Not for a3 (it is in a2 too, and fixing it touches
  `crates/`). Candidate for a4: take a `PathBuf` / `OsString` in the binding.
- **N-6** `export --all -o <read-only folder>` prints the manifest header on stdout before the exit-3 error. This is
  T-18 behaviour, unchanged and harmless; recorded only.
- **N-7** README:173 is 128 characters after the reflow, while the paragraph wraps at about 120 (cosmetic).
- **N-8** T-18c note: its counts (71 / 76 / 227; 412 + 3) agree with mine. Its ".parquet removed (6 fail)" used a
  different mutation (branch removed, so `E-CONFIG`) from mine (returns `"csv"`, 2 fail). Both fail named tests.

## Checklist "Release 0.1.0a3 checklist (session 20)", read step by step

- Step 1: holds today. C = `ff3d6b25c360f93eeb8c1305927dd6cf3550610b`; `git diff --stat C HEAD -- . ':!docs'` is
  empty at `baa6b1b`. This report adds only `docs/review/`.
- Step 3: counts for the record: 412 passed + 3 skipped (nextest); 227 passed, 0 skipped (pytest, probes build);
  fmt, clippy (×3), deny, ruff, ruff format and mypy clean. 3b "not due" is confirmed (section 4).
- Step 5: both commands work as written and print the expected output (section 4).
- Step 6: the command works. With `| tee`, the shell's exit status is tee's, so the owner should judge by the log
  (`check_dist: ok` twice, `identical` twice). Alternatively add `set -o pipefail` or check `${PIPESTATUS[0]}`. Minor.
  `/var/tmp/oradmp-a3` and `/var/tmp/a3-check` do not exist yet (good).
- Step 7: see SF-2 (root in the container). On the VM as a normal user it passes as written (section 3). Its CLI
  lines (`--version`, `tables`, `tables X/missing.dmp; echo $?` giving the `[E-IO] No such file or directory: X/missing.dmp`
  line and `3`, exports by suffix in the pyarrow-only venv, the sdist build) all behave as written.
- Steps 9 and 10: the commands name `0.1.0a3`, W and S correctly. I found no stale a2 literal.

## Not checked

- PyPI and TestPyPI: no upload, no index pages, no `pip download`, no `pip install --pre` and no plain
  `pip install oradmp` against an index. No tokens.
- macOS and Windows (no machine, no wheels), a real RHEL / Oracle Linux 8 host, and the Rocky Linux 8 container (no
  Docker by rule). SF-2's container behaviour is inferred, not run.
- Step 4's network checks as a separate run. `cargo deny check` did run advisories and sources here, but I did not
  check the date of the advisory database.
- A real `SIGINT`, a full disk, a closed stderr. The EACCES case as root.
- `test_cli.py`'s older (T-18 / T-18b) tests, beyond what the mutations exercised. Review 06a covered them.
- Benchmarks and memory: not due (no change under `crates/`).
- I did not re-read Analyst session 14 or Architect session 20 beyond ADR-72, ADR-77, the T-18c plan entry, the a3
  checklist and the closure "Done when".

Scratch: `/var/tmp/rev06c`, 5.7 GB; it can be deleted.
