# Review 06c: the 0.1.0a4 candidate (T-17b to T-20b, the cut, step 3b)

Reviewer session 6c, 2026-10-29. The prompt is the owner's private reviewer-6c prompt. Scope: the plan, "Release 0.1.0a4",
session 26 and 27 blocks, items 1 to 11; ADR-105 to ADR-108.

**Commit under review: C = `4b9883c52ebf709ccf0161883505844fdcc45345`** (T-20b, on top of `552eb19`, `e1ed10b` T-20,
`4aa33e6`, and S = `9950ef2`).

**Verdict: no blocking finding. 0.1.0a4 can be built from C as it stands.** My scratch-root build gives the sdist
byte for byte as C's CI run (`510f2e39...c0fd`). My wheel differs from the CI wheel only in the build root, which
appears in the SBOM member alone (as ADR-90 predicts). Every check of the definition of done passes with the counts
of ADR-108 (nextest 488 + 4 skipped, pytest 256). Every mutation I repeated or added fails a named test, except one
(N-4), and that one changes nothing a user sees. The cut leaves no residue I could find. Step 3b shows no regression.
The `T_BIG` scan watch showed once in three runs. The archive's a3 core (`460aef5`), measured in the same sitting, is
as slow or slower, so the watch comes from the machine, not the code. There is 1 should-fix and there are 7 notes.
None of them needs a change to C before the release.

### Where the plan and the prompt differ (I followed the prompt where it is stricter, as noted)

1. **Report name.** The plan says `review-06c-...md` and the prompt says `review-06c.md`. I used the prompt's name.
2. **The clone.** The plan says to clone from GitHub. The prompt says to clone `~/p/oradmp` and to use no network
   except the package registries. I cloned `~/p/oradmp`, whose HEAD is C.
3. **The pins.** The plan (ADR-93 item 3) says to resolve every action tag to its SHA with `git ls-remote`. That
   needs GitHub, which the prompt's network rule excludes, so I **did not check** the pins against their tags.
4. **The CI pastes.** The plan names the pastes of S, `e1ed10b`, the `t20b-proof` run and C's three runs as inputs.
   None of them is in the repository, and the prompt gives only C's two hashes. Everything I compared with CI is
   compared with those two hashes and with the figures that ADR-105 and ADR-108 record.

## Method and conditions

- At the start: load 0.20 and no container (`docker ps` printed only its header). `git status` of `~/p/oradmp` was
  clean at C before this report. Afterwards it shows only this file. I built nothing in `~/p/oradmp` and did not run
  `uv run` there. Its `python/oradmp/_core.abi3.so` (mtime 16:44:15) is from before this session.
- **Scratch.** Everything ran below `/var/tmp/oradmp-review-6c/`:
  - `oradmp/` is a clone of `~/p/oradmp`, detached at C. The archive's `master` and the three `v0.1.0a*` tags were
    fetched into this clone, which reads the archive without changing it.
  - `a3core/` is a second worktree of that clone, at `460aef5`.
  - The scratch also held its own target folders, a venv for `maturin develop`, three install venvs and the release
    root `release/`.
- Settings: `CARGO_BUILD_JOBS=2`, `TMPDIR=/var/tmp/oradmp-tmp`, and `ORADMP_WORK` set to the owner's `corpus/work`
  (the parent folder of `18c/`). One build at a time.
- **Mutations.** Each mutation script checks that the original text occurs exactly once, applies the change, runs
  the tests, restores with `git checkout --`, and prints `git status --porcelain`. The status was empty after every
  mutation.
- **Secrets.** No secret, home path, e-mail address or host name is printed in this report or in my terminal output.
  I counted them only.

## Findings

### Blocking

None.

### Should fix

**SF-1. Five command substitutions in `echo` arguments still hide a failure under `pipefail`.**

- Where: `.github/workflows/ci.yml:200`, and in the same form lines 57, 178, 318 and 363.
- Line 200 is `echo "NEWEST_PY=$(python -c '...')" >> "$GITHUB_ENV"`. `pipefail` and `-e` do not see the status of
  a substitution inside the arguments of another command.
- Shown: `bash --noprofile --norc -eo pipefail -c 'echo "X=$(false; echo)" >/dev/null; echo reached-echo; x="$(false)";
  echo not-reached'` prints `reached-echo` and exits 1 only at the assignment.
- Scenario: if `python -c` failed, `NEWEST_PY` would be empty. Then `uv venv --python ""` does not fail. I ran it in
  the scratch folder: it exits 0 and creates a venv with uv's default interpreter (3.14.7 here). So the venv named
  "newest" would test whichever Python uv picks, and the job would stay green.
- Why it does not block: the same step has just run `python -m pip install uv` with the same `python`, and that
  command would fail the step first. On the runner, uv's default is the `python` that setup-python put first on
  `PATH`, which is the newest one. The four `JOB_START=$(date +%s)` lines cannot fail in a way that matters.
- The session 27 block lists substitutions swallowed by `if`, `case` or an assignment. This form is not on that
  list, but it belongs to the same class, so the Architect should rule on it.
- Fix (after a4, with the next `ci.yml` task): `newest="$(python -c ...)"` first, then `echo "NEWEST_PY=$newest"`.
  The same for `JOB_START` if one wants a single rule.

### Notes

**N-1. `ci.yml:402`: `if [ -n "$(find . -name '*.proptest-regressions' -print -quit)" ]` swallows an error of `find`.**
I repeated both constructs of T-20b under `bash -eo pipefail`:
- The old `find | grep -q` takes the wrong branch when there are many matches, because of SIGPIPE. With 20,000
  regression files it printed `MISSED` in 3 of 3 runs.
- The new form takes the right branch with and without a file.

What the new form can still miss is a regression file inside a folder that `find` cannot read. That does not happen
in a runner checkout, and ADR-108 item 5 prescribes this construct. Recorded, no action.

**N-2. Some counts in the documents disagree with the corpus.**
- The test names `t11_options.rs:375` `..._totals_of_the_201_files` and `t11_strict.rs:806` `strict_over_the_201_files_...`
  say 201 files. They assert `TOTALS` (229 files).
- The comments `common/synth.rs:145` ("211 files") and `t11_options.rs:400` ("all 211 files") say 211.
- The T-19 (m) note's result line says the `T_BIG` scan was "-5.0 % to -6.8 %". Its own table gives 726.1 / 736.7 /
  741.6 MB/s against 779.0, which is -6.8 / -5.4 / -4.8 %. The plan's "-4.8 to -6.8 %" is right.

The values that are asserted are right. I checked each against the Analyst's tools in the scratch clone:
- `corpus_counts.py` TOTAL: 229 files, 436 sections, 41,494 rows, 243,905 fields, 20,084 nulls, 783 text fields, 27
  empty conventional and 20 empty direct.
- `collist.py`: 2,075 column definitions.
- `gen_walk_expected.py`: 450 parts and 21 LOB sections; 126 LOB values, of which 80 short and 46 long.
- `gen_number_parts.py`: 86,236 NUMBER fields.

All of these equal `common/mod.rs` `TOTALS` and `FOLDER_TOTALS`.

**N-3. The docstrings of `crates/oradmp-core/tests/data/gen_walk_expected.py` and `gen_number_parts.py` still say
`uv run`.** ADR-105's answer covers only the first of the two (deviation 7). In the public tree a contributor who
follows either line syncs the root project. Both generators ran correctly for me with `python3 -I` from
`docs/formats/tools`.

**N-4. One mutation survived: the buffer raise in `Dump::open` (`dump.rs:381`).**
- `ByteStream::new(f, options.buffer_bytes)` without `.max(MIN_BUFFER_BYTES)` leaves 488 of 488 tests passing.
- The stream there reads only the header and the first item, so the raise has no effect and nothing can test it.
- `io.rs:16-18` says the raise is applied "at every `ByteStream::new` of `dump.rs` and `reader.rs`", which is true
  but includes this site where it does nothing.
- The two raises that matter are each caught: `dump.rs:912` (mutation R5) fails `a_small_buffer_is_raised_for_the_row_cursor`,
  and `reader.rs:1294` (R6) fails `read_table_with_a_small_buffer_reads_the_big_rows`.

No action needed.

**N-5. CONTRIBUTING, read as a stranger would.**
- Every command runs from the public tree, and none needs a private file.
- Two gaps remain:
  1. "Development setup" installs `ruff` and `mypy` unpinned, while CI pins ruff 0.17.1 and mypy 2.4.0. A newer ruff
     can report what CI does not.
  2. Nothing says how to get `cargo nextest` and `cargo deny`, which the checks list needs.
- Suggested fix: one line each, at the next CONTRIBUTING task. Not an a4 matter.

**N-6. The canonical root `/var/tmp/oradmp-release` exists on the VM** (created 2026-10-27; I did not read it beyond
`ls`). `release_build.sh --release` needs it absent. This is already the owner's step (plan, session 27 order, row
4). I only checked that my run with `--release` and another root is refused: exit 1, with both paths named.

**N-7. Step 3b, one unconfirmed "watch" on `T_ROWS` direct.** In run 6 of 6 on C the direct-path scan was 753.8 MB/s
(-15.0 %) and the read 2.15 M rows/s (-12.2 %). Runs 1 to 5 give -0.7 to -8.9 % and -7.9 % to +3.3 %. Rule 3 needs a
second run to confirm a figure, and none of the other five does. This is the noisy 11 ms scan (benchmarks.md 4.2).

## What I verified by running

### Item 1: the diffs

- `git diff --stat edc62a5 C -- . ':!docs'` (170 files): every file belongs to a commit of T-17b, T-16a, T-16a2,
  T-19, T-19b, the T-19 (m) fix (`fuzz/fuzz_targets/values.rs`), the four scrub commits, `HISTORY.md`, T-20 or
  T-20b. The only other commits in range are the owner's corpus commits `cca4c65`, `bbf921b` and `b75d60d` (files
  under `corpus/` only) and `3dc7a37` (`CLAUDE.md`, now removed).
  - Wording: the plan's list for item 1 does not name the corpus commits. They are the owner's and expected.
- `git diff --stat 460aef5 C -- crates/oradmp-core/src`: 8 files. Each is touched only by `921971f` (T-19) and/or
  `1bb7785` (T-19b).
- `git diff --stat 1bb7785 C -- crates python tests`: `tests/check_dist.py` and `tests/test_check_dist.py` only.
- `git diff --stat e1ed10b C -- . ':!docs'`: `.github/workflows/ci.yml` only.
- `git diff --stat 9950ef2 C -- . ':!docs'`: exactly the 9 files of T-20 (g).
- `git ls-tree -r` of P (`b55eb10`) and S differ only by `HISTORY.md`.

### Item 6: the checks in the scratch clone at C

| check | result |
|---|---|
| `sha256sum -c` in `18c/`, `11g/`, `11g-cp1251/` | ok, ok, ok |
| `cargo fmt --all --check` | ok |
| `cargo clippy -- -D warnings`; `--all-targets`; `--all-targets --all-features` | ok, ok, ok |
| `cargo nextest run -j 2` | **488 passed, 4 skipped**: the four names of `ci.yml:106-109`, nothing else |
| `cargo deny check` | advisories ok, bans ok, licenses ok, sources ok (one "unmatched license allowance: Zlib" note, not an error) |
| `maturin develop --features test-probes` (scratch venv) | `oradmp 0.1.0a4` |
| `ruff check`, `ruff format --check`, `mypy` | ok, 29 files formatted, no issues in 24 files |
| `pytest -rs -q` | **256 passed**, 0 skipped |

### Item 4: the candidate built twice and installed

- `ORADMP_BUILD_ROOT=/var/tmp/oradmp-review-6c/release tools/release_build.sh C --twice` (without `--release`)
  printed `epoch: 1793315613`, which equals `%ct` of C. It then printed `check_dist: ok` for both builds, `identical`
  for both files, and `reproducible`.
  - Wheel `79f1ea3164c2e31c40cf8444073bc83a03771db932aa24ad5755a5f458853381`: differs from CI's `2e4c69a5...`, as
    expected at another root.
  - Sdist `510f2e39f8ae7eb9660fa4f2313062d2245c3c6d16ebd10740fa0e529c40c0fd`: **equals CI's**.
- Members of my wheel:
  - The build root appears only in `dist-info/sboms/oradmp-py.cyclonedx.json` (6 times). `RECORD` then differs
    through that member's hash.
  - The `.so` holds no root path.
  - I do not have the CI wheel or its member listing, so the member-by-member comparison with CI was not done.
- `METADATA` and `PKG-INFO` carry `Version: 0.1.0a4` and the two `Project-URL` lines (`Source`, `Issues`), and
  `WHEEL` carries `Tag: cp311-abi3-manylinux_2_28_x86_64`.
- The wheel in three venvs, with `pytest` from the clone against the installed package (`oradmp.__file__` is in
  `site-packages`, checked from inside a pytest run):

  | venv | result |
  |---|---|
  | 3.11.16 + pyarrow 15.0.2 + pandas 2.2.3 | 253 passed, 3 skipped (the three probes) |
  | 3.14.7 + pyarrow 26.0.0 + pandas | 253 passed, 3 skipped (the three probes) |
  | 3.11.16 + pyarrow only | 246 passed, 4 skipped (the probes, and `tests/test_consumers.py:18` `importorskip("pandas")`) |

  All three print `oradmp --version` = `oradmp 0.1.0a4`. These counts equal ADR-108 item 1 (253 + 3 / 246 + 4 /
  253 + 3).
- The CLI from the wheel, run by hand with a dump named `d2 e5 f1 f2 .dmp`:
  - `tables` lists `T_FRAME`.
  - A missing name gives `[E-IO] No such file or directory: \xd2\xe5\xf1\xf2-missing.dmp`, exit 3.
  - `export -o` onto an existing `\xd2\xe5\xf1\xf2.csv` gives `[E-CONFIG] the target \xd2\xe5\xf1\xf2.csv exists`,
    exit 2.
  - Exporting into a read-only folder gives `[E-IO] Permission denied: ro/\xd2\xe5\xf1\xf2.csv.oradmp-partial`,
    exit 3, and the folder stays empty.
  - `survey` prints `file: \xd2\xe5\xf1\xf2.dmp`.
  - No message contains `\udc`. These runs were as uid 1000.

### Item 9: T-20 and T-20b

- (a) **Version.** `git grep '0\.1\.0a3\|0\.1\.0-alpha\.3' -- . ':!docs' ':!corpus' ':!HISTORY.md'` finds nothing.
  `0.1.0-alpha.4` appears in `Cargo.toml` and in three lock lines, and `0.1.0a4` in four README lines and one
  CONTRIBUTING line. The diff of the locks has only these three version lines.
- (b) **The `Project-URL` rule**, three mutations in `tests/check_dist.py`, each restored:
  1. Comparing labels only fails `test_t20_a_changed_project_url_fails_with_two_lines` and three others.
  2. Removing the call from `check_metadata` fails the missing, changed and extra cases.
  3. Ignoring extra lines fails the changed and extra cases.
  - `test_t20_pyproject_declares_the_two_urls` pins both URLs as literals.
- (c) **README.**
  - The LONG / LONG RAW sentence of ADR-104 item 4 stands verbatim in "What it reads" (lines 22-25).
  - I read "Limits" against `rows.rs` and found no false line. The `W-TSLTZ` line is still true:
    `types/resolve.rs:148-160` still emits the warning. T-19b removed only the 7-byte TSLTZ event.
  - The clone line names the public address.
- (d), (e) **CONTRIBUTING and `HISTORY.md`.** The CONTRIBUTING changes are as T-20 (d) lists them. `HISTORY.md` is
  not in any diff after S.
- (f) **`fuzz-check`.**
  - With the pinned nightly (`cargo 1.101.0-nightly 2026-10-12`), `cd fuzz && cargo check --locked` passes.
  - With `values.rs:51` reverted to `Some((t, _short))` it fails with `error[E0308]: mismatched types`, exit 101.
  - The same mutation inside `cargo check --locked 2>&1 | tee` exits 101 under `bash --noprofile --norc -eo pipefail`
    (the C shell). Under `bash -e` (the `e1ed10b` shell without its per-step line) it prints `reached` and exits 0.
  - The file was restored and `git status` was clean.
- T-20b (a), (b): I read the diff `e1ed10b..C` line by line.
  - It has a workflow-level `defaults.run.shell: bash` with its comment, the header line, and the removed
    `set -o pipefail`. No step has `shell:`.
  - The two constructs are rewritten as ADR-108 item 5 says.
  - PyYAML parse: `defaults` is `{'run': {'shell': 'bash'}}`. `bash -n` passes on all 37 `run` blocks. No step and
    no job has `continue-on-error`.
- T-20b (c), repeated:
  - The shell fact: `-eo pipefail` exits 7 without `reached`, and `-e` prints `reached` and exits 0.
  - Construct (1), new form: a log with three probe skips gives 3, and a log with one skip of 1 plus one of 2 gives 3.
    A log with none prints `expected exactly 3 probe tests skipped in x, got 0` and exits 1. The old form exits 1
    without the message.
  - Construct (2): see N-1.
- T-20b, the rest of `ci.yml` read once more for hidden failures outside the summary steps:
  - `case "$(...)"` (line 232) falls to `*) exit 1`.
  - The `skipped=` and `probes=` substitutions are followed by checks of their values.
  - The assignments at lines 221 to 223 and 251 pass on their status.
  - The `(cd fuzz && { ... } && rustup component add clippy)` subshell at line 375 fails the step.
  - The `rustup show active-toolchain || rustup toolchain install` lines are fallbacks.
  - `release_build.sh`, `fuzz_run.sh` and `fuzz_seeds.sh` each `set -euo pipefail`. `fuzz_run.sh:49-54` exits 1 on
    any file in `artifacts/<target>/`.
  - What is left is SF-1 (the `echo "...$(...)"` lines) and N-1.

### Item 10: the cut (counts only)

- (a) **Scrub residue at C.** `git grep -c` both as text and with `--text` (binary files included), a UTF-16LE hex
  scan of every tracked file, and `git log -p --text` of the whole public history (S to C), for each of these:
  - my `$HOME`;
  - `hostname`;
  - the archive's author e-mail (`git log -1 --format=%ae 0520cd5`);
  - the Gogs host (from the archive's `origin` URL);
  - the `corpus` password as `0520cd5:corpus/scripts/config.sh` held it.

  Result: **0 hits each**. In the token forms check:
  - `pypi-…{16,}`, `ghp_…{20,}`, `github_pat_…{20,}`, `gh[ousr]_…{20,}`, `AKIA…` and `BEGIN … PRIVATE KEY` give 0
    in the tree and in the history.
  - The bare prefixes hit only `docs/design/open-questions.md`, 3 lines each: the Architect's description lines,
    accepted in ADR-105 item 3.

  Other checks:
  - `IDENTIFIED BY` with a literal password: only `corpus_own` and `corpus_ru` (`05_own_setup.sql`,
    `07_ru_setup.sql`, and the `requests.md:250` example of `corpus_own`), as ADR-108 item 3 decided.
  - `corpus` is `IDENTIFIED BY "&1"`.
  - No dump holds `IDENTIFIED BY VALUES`.
  - The scripts hold literals only for `OWN_PW` and `RU_PW`. The `ORACLE_PASSWORD` lines say `'...'`.
  - All 5 public commits have the no-reply address as author and as committer.
- (b) **`HISTORY.md`.**
  - P and the a1, a2 and a3 hashes equal `b55eb10…` and the fetched tags `v0.1.0a1` to `v0.1.0a3` (full hashes
    compared).
  - P's date is 2026-10-26 15:25:06 -0500 and S's is 15:50:21 -0500.
  - The scrub list matches what the four scrub commits changed (`0cbd165` 5 docs files, `6a5869c` CONTRIBUTING,
    `ac58e08` `config.sh` and `00_setup.sql`, `b55eb10` the private paths).
  - `docs/prompts/`, `.claude/` and `CLAUDE.md` are absent from C and ignored by `.gitignore`.
- (c) **Settings as the tree shows them.**
  - Every `uses:` is GitHub's or `taiki-e/install-action@1e29709c…`, the pin in ADR-105 item 8, and each has a
    version comment.
  - `permissions: contents: read` is set at the top and no job sets its own `permissions`.
  - There are 5 checkouts, each with `persist-credentials: false`.
  - The triggers are `push` (master) and `workflow_dispatch`.
  - `secret`, `token`, `publish`, `twine`, `gh release`, `--run-ignored` and the account name occur nowhere in the
    file.
  - `upload-artifact` appears only in `extended` with `if: failure()`.
  - `manylinux2014` writes to `$RUNNER_TEMP` and makes no artifact.
  - The `wheel` job passes `--release`.
- (d) **Public CI.** My counts (nextest 488 + 4, pytest 256, the venv counts) equal ADR-108 item 1. My sdist equals
  C's CI sdist. The pastes themselves are not available to me (see "Where the plan and the prompt differ" item 4).
- (e) **No step points a contributor at a private file.** Outside `docs/`, `.gitignore` and `HISTORY.md`, no file
  names `CLAUDE.md`, `docs/prompts` or `.claude/`.

### Items 2 and 8: T-17b, T-19, T-19b

- **Golden and expected files**, regenerated in the scratch clone with `python3 -I` from `docs/formats/tools`:
  `exp_catalog.tsv` and `exp_raw.tsv` are byte-identical (`cmp`). `walk_expected.tsv`, `walk_parts.tsv`,
  `lob_expected.tsv` and `value_parts.tsv` were rewritten in place with `git diff` empty. All four generators import
  only `exp_parse` and `intro_list` from `docs/formats/tools`, never `oradmp`.
- **`header_sets_match_the_fixtures`.** My own reading by [H1] to [H4] (a Python script, no oradmp code) gives **229**
  EXP files and **18** distinct (version, mode, client, database) sets. That is exactly the 18 unlabelled entries of
  `HEADER_CHOICES`. None of the 6 labelled entries occurs, and no 18c file has database 171.
- **`UNCONFIRMED(`.** There are 4 sites in `synth.rs` (lines 860, 1130, 1175, 1206), one per variant of the
  4-variant `enum Unconfirmed`. `CONTRADICTS FIXTURES` is at line 909. The enum grew from T-17b's 2 variants by
  ADR-94 item 2 and T-19 / T-19b. The TIMESTAMP precision draw is 0 to 9 (`synth.rs:1765-1766`), by ADR-94 item 1
  (R-047, `tsp_exp_<v>`), which replaces T-17b (a)(2)'s {0, 6, 9}.
- **ADR-99's table against `rows.rs:704-830`**: `fe ff` is NULL; 0 is `Framing`; 1 to 0xfff1 and 0xfff5 to 0xfff9
  are one field; 0xfff2 to 0xfff4 are one field and counted for the warning (823-829); `fa ff` is form (b) with the n
  and z checks (`read_four_byte`); `fb ff` and `fc ff` stop with `E-UNSUPPORTED-LAYOUT` (764-776); `fd ff` is chunked
  with `CHUNKED_MIN_TOTAL = 65_534` (1032); `ff ff` is `Framing`. The guard `0 | 0xfff0..=0xfffd if !conventional_long || len == 0`
  keeps every other column as before. Each of the six message texts of ADR-99 item 3 occurs verbatim in `src/` and in
  the tests.
- **Mutations, each alone, full nextest run:**

  | id | mutation | result |
  |---|---|---|
  | R1 (= T-19b M1) | `fa ff`: z not read (`let z = 0u16`) | 47 fail, among them `t_long4_equals_the_truth_in_all_four_files`, `the_four_byte_length_is_checked_at_n_and_at_z` |
  | R2 (= T-19b M3) | no 0xfff2..0xfff4 count | 4 fail: `a_length_of_0xfff2_is_read_as_one_field_with_a_warning`, the t17 0xfff2 case, `rows::the_one_field_sizes...`, `t12 no_message...` |
  | R3 (= T-19b M4) | 0xfffb read as a length | 7 fail, among them `a_field_starting_fb_ff_or_fc_ff_stops_the_file_by_name`, `t3c d_...` |
  | R4 (= T-19b M6) | `CHUNKED_MIN_TOTAL` 65,535 | 10 fail, among them `long_stats_and_no_warning_in_the_new_long_files`, `a_chunked_total_below_65534_bytes_is_read_with_a_warning` |
  | R5 (= T-19b M8) | no raise in `dump.rs:912` | 1 fails: `a_small_buffer_is_raised_for_the_row_cursor` |
  | R6 (new) | no raise in `reader.rs:1294` | 1 fails: `read_table_with_a_small_buffer_reads_the_big_rows` |
  | R7 (new) | no raise in `dump.rs:381` (`open`) | **0 fail**: N-4 |
  | B1 (= T-17b (c)) | E2 guard disabled | `no_e2_table_is_followed_by_an_item_outside_the_e2_list` fails |
  | B2 (= T-17b (a)) | label removed from 11g RTABLES 171/873 | `header_sets_match_the_fixtures` fails |
  | B3 (= T-17b (a)) | unlabelled 18c / 171 entry added | `header_sets_match_the_fixtures` and `the_random_specs_cover_every_construct` fail |
  | T16a2-c | SBOM prefix test accepts any root | 3 `test_t16a2_*` fail |

### Item 5: deviation 3 (`walk` uses the caller's `buffer_bytes`)

**It is right.** The reasons, with the lines:

- `exp/items.rs:870-873` opens the stream with `options.buffer_bytes` as given (the floor is `io::MIN_BUFFER` = 64,
  `io.rs:49`). `walk_rows` (`rows.rs:1326-1341`) drives the cursor through `RowCursor::step` (`rows.rs:633-640`),
  which passes **no sink**.
- Every `read_bytes(n)` with a field-sized `n` stands in a `(Some(k), _)` sink arm (`rows.rs:835, 938, 982, 1089,
  1170, 1234`). Without a sink the cursor hashes in `MIN_BUFFER` steps (`hash_bytes`, `rows.rs:300-313`) or skips.
  `table.rs:169-178` (`read_vec`) reads in 64-byte steps, and `items.rs:819` `peek(4096)` handles a short peek.
- Shown by running: a throwaway test in the scratch clone walked all 229 fixtures with `hash_fields`, `record_lob`
  and `record_items`, once with `buffer_bytes: 64` and once with the default. The two `{:?}` reports were equal for
  every file, the 32 `long*` files included. As a check of the test itself, I mutated `hash_bytes` to read the whole
  field at once, and the test then failed.

### Step 3b: alone on the machine, no container, C against baseline 2 and T-19 (m)

- Every run started at a 1-minute load below 0.5 (0.35 to 0.45) and ended at 0.31 to 0.83.
- The files were read into the page cache before each run.
- `T_BIG` is 453,304,320 bytes and 5,000,000 rows, as in the baseline.
- **Micro-benchmarks** (criterion point estimate, against the baseline column):

  | benchmark | figure | diff |
  |---|---|---|
  | `number/OraNumber::decode` | 1.4322 µs | -2.2 % |
  | `decode_date` | 142.15 ns | -2.1 % |
  | `decode_timestamp` | 246.01 ns | -6.3 % |
  | `Charset::decode (Win1251)` | 171.36 µs | -1.3 % |
  | `read_table/R_STAT` | 1.0001 ms | -0.3 % |
  | `T_NUM` | 37.604 µs | -0.8 % |
  | `T_TEXT` | 37.202 µs | -6.3 % |

  All are at the baseline or faster.
- **Throughput**: six runs of C (runs 4 to 6 interleaved with the three `460aef5` runs), each a median of 3 samples.

  | quantity | baseline | C runs 1 to 6 | worst | `460aef5` (same sitting) |
  |---|---|---|---|---|
  | `T_ROWS` conv. scan (MB/s) | 809.1 | 816.6, 808.6, 774.6, 853.0, 806.7, 837.9 | -4.3 % | 801.0, 791.9, 764.0 |
  | `T_ROWS` conv. read (rows/s) | 2.17 M | 2.33, 2.30, 2.18, 2.38, 2.19, 2.36 M | +0.3 % | 2.38, 2.35, 2.32 M |
  | `T_ROWS` direct scan (MB/s) | 886.7 | 880.7, 856.2, 807.7, 868.3, 850.4, 753.8 | -15.0 % (run 6 only; N-7) | 876.9, 847.4, 831.7 |
  | `T_ROWS` direct read (rows/s) | 2.45 M | 2.53, 2.33, 2.26, 2.47, 2.33, 2.15 M | -12.2 % (run 6 only; N-7) | 2.49, 2.38, 2.39 M |
  | **`T_BIG` scan (MB/s)** | 779.0 | 721.0, 758.6, 745.5, 758.1, 772.8, 759.0 | -7.4 % (run 1; the others -0.8 to -4.3 %) | **747.2, 725.8, 744.5** (-4.1 to -6.8 %) |
  | `T_BIG` read (rows/s) | 2,195,749 | 2,127,868, 2,173,855, 2,097,403, 2,177,962, 2,171,203, 2,162,928 | -4.5 % | 2,189,109, 2,178,888, 2,204,884 |

  **The `T_BIG` scan watch, explained.**
  - The watch band (5 to 10 % below) showed once in three runs (run 1, -7.4 %), so I ran the `460aef5` scan in the
    same sitting, alone, interleaved with three more runs of C, as ADR-106 item 2 asks.
  - The a3 core gives 725.8 to 747.2 MB/s (mean 739.2, -5.1 %). The interleaved runs of C give 758.1 to 772.8 MB/s
    (mean 763.3, -2.0 %).
  - So the a3 core is not faster than C on today's machine: the difference against baseline 2 is the machine, not
    T-19 / T-19b.
  - The `T_BIG` full read of C is 0.3 to 2 % below `460aef5` in the same sitting, which agrees with T-19 (m)'s -1.2 %.
  - No bisect was needed. Nothing is above 10 %.
- **Memory** (`t15_memory`, three runs, ratio limit 1.10):

  | file | run 1 | run 2 | run 3 | highest end peak | limit |
  |---|---|---|---|---|---|
  | `T_ROWS` conv., stop / end kB, ratio | 7,492 / 7,688, 1.0262 | 7,440 / 7,780, 1.0457 | 7,540 / 7,792, 1.0334 | 7,792 | 9,108 |
  | `T_ROWS` direct, stop / end kB, ratio | 7,520 / 7,784, 1.0351 | 7,428 / 7,848, 1.0565 | 7,432 / 7,844, 1.0554 | 7,848 | 9,021 |
  | `T_BIG`, stop / end kB, ratio | 7,560 / 7,480, 0.9894 | 7,576 / 7,500, 0.9900 | 7,612 / 7,608, 0.9995 | 7,608 | 8,984 |

  All within the limits.

## Tests that did not fail when mutated

- R7 (`dump.rs:381`, the raise in `open`): no test fails, and none can. See N-4.

Every other mutation listed above failed at least one named test.

## Numbers that disagree

- The test names that say 201 files and the comments that say 211, against the asserted 229 (N-2).
- The T-19 (m) note's "-5.0 % to -6.8 %", against its own table's -4.8 to -6.8 % (N-2).

Otherwise, the figures of ADR-105 item 9, ADR-108 item 1, the plan's T-20 and T-20b, the note and my runs agree:
- nextest 488 + 4 skipped and pytest 256;
- the venvs 253 + 3, 246 + 4 and 253 + 3;
- 229 files, 436 sections, 450 parts, 41,494 rows, 243,905 fields and 86,236 NUMBER fields;
- `SOURCE_DATE_EPOCH` = `%ct` of the commit;
- C's sdist hash.

## What I did not check

- **The CI pastes**: C's push run, its `extended` and `manylinux2014` dispatches, and the `t20b-proof` run with its
  per-step outcomes (T-20b (e) to (g)). They are not in the repository. I compared only the two hashes the prompt
  gives, and only the sdist can be compared from a scratch root.
- **The `manylinux2014` `Tag:` line** (ADR-106 item 3 (c)) and any `manylinux2014` build.
- **The action pins against their tags** (`git ls-remote`; no network allowed). I also did not check the GitHub
  settings themselves (ADR-105 item 8 records them as the owner's statement).
- **The member-by-member comparison of my wheel with the CI wheel.** I have no CI wheel. I showed only that the
  build root is in the SBOM alone.
- **The rest of the earlier mutation lists.**
  - T-19 M1 to M12 and T-19b M2, M5, M7 and M9 to M12: I repeated only R1 to R5 above.
  - T-17b (b) and (d) to (j): only (a) and (c) were repeated. I did not repeat the SF-4 Python mutations, but did
    the non-UTF-8 cases by hand.
  - T-16a2 (a), (d) to (g): only (b)'s refusal and (c)'s mutation were repeated.
- **The fuzz runs, the long property runs, `stream_memory.py`, and a Rocky Linux 8 or RHEL 8 install.**
- **The probes and their skip reasons as root**: all runs were as uid 1000.
- **The README beyond** "What it reads", "Install", "Limits" and "Command line".

## Scratch

`/var/tmp/oradmp-review-6c` (7.9 GB at the end: clone, worktree, three target folders, venvs, the release root) is
deleted after this report; nothing in it is needed by a finding.
