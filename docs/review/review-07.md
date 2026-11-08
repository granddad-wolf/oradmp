# Review 07: the wheel and CI line since 6c (T-21, T-16c, T-16b, T-16d)

Reviewer session 7, 2026-11-08. Prompt: `docs/prompts/reviewer-7.md`. Scope: the plan's "Reviewer 7" block (sessions 31
to 33), ADR-119 as amended, ADR-108, ADR-110, ADR-116 to ADR-118, ADR-120 to ADR-130.

**Candidate: `4ed52e5e185800380893acb7d46c056adf2ecfd9`** (Architect 33 documents), on top of `aae02c6` (T-16d
results) and `9a9bc84` (T-16d code). `git diff --stat 9a9bc84 4ed52e5` lists only five files under `docs/design/`. So the
code I read and built at 4ed52e5 is the code at `9a9bc84`, as ADR-129 item 4 asks. Range read: `4b9883c..4ed52e5`.

**Verdict: no blocking finding. Nothing in this report requires a change to the candidate before a5.**
- My scratch-root build of 4ed52e5 gives CI's sdist byte for byte (`eb66c661…`).
- Its wheel differs from CI's only where the build root is written (the SBOM and `RECORD`). `_core.abi3.so` is
  `d0efa2df…`.
- Every check of the definition of done passes with the expected counts.
- There are 3 should-fix findings and 11 notes. All of them go to T-16e, to the Architect's a5 checklist, or to the
  Q-15 amendment session.

### Where the plan and the prompt differ

1. **Q-15.** The prompt gives the owner's answer "yes" (2026-11-08). The documents at the candidate still treat Q-15 as
   open. I followed the prompt and list what the amendment needs (section "Q-15").
2. **The machine-state proof from one push run** (ADR-119 item 3 (d)) needs the `checks` and `wheel` summaries of one
   push run. I cannot read GitHub; I use the recorded figures and say so.
3. **The macOS expected counts** are written in two orders: 246/246/239 in the plan's Reviewer 7 block (i) and ADR-129
   item 4, and 246/239/246 in ADR-126 item 4 and ADR-129 item 4 (i). I give them by venv name (N-1).

## Method and conditions

- **Pre-checks.** `git status --short` was empty. `git log -1` = `4ed52e5e185800380893acb7d46c056adf2ecfd9`.
  `/var/tmp/oradmp-release` did not exist; I never used it. `ORADMP_WORK` was set to the owner's `corpus/work`.
- **Scratch.** Everything ran below `/var/tmp/oradmp-review-7/`, in a clone of the repository detached at 4ed52e5:
  - its own cargo target and venv;
  - six release-script build roots: `rb-4ed52e5`, then `rb` reused once per commit and moved aside after each build;
  - throwaway copies for the mutations and harnesses.

  Nothing ran in the repository except reads. `git status` there was empty before this file was written.
- **Settings.** `CARGO_BUILD_JOBS=2`, `TMPDIR=/var/tmp/oradmp-tmp`, one build at a time.
- **Network.** Only the package registries: crates, PyPI for maturin, ziglang and pyarrow, and `pip download`.
- **Secrets.** No token, home path or host name appears in this report.
- **Labels.** "Ran" marks what I executed. "Read" marks what I only read. "S" marks a figure that rests on the owner's
  pasted summaries as recorded in the notes or the plan.

## Findings

### Blocking

None.

### Should fix

**SF-1. The substitution form of Reviewer 6c's SF-1 is back, in the `platforms` job (`ci.yml:771`).**
- **What.** The `platforms` "Environment" step writes `echo "JOB_START=$(date +%s)"` inside `{ ... } >> "$GITHUB_ENV"`
  (lines 768 to 772). ADR-109 item 3 forbids this form: "never inside the arguments of another command". T-16b (g)
  required the substitution rule.
- **History.** The six sites 6c and ADR-112 named are fixed (T-21): `job_start=` at lines 59, 181, 323, 369 and 539;
  `newest=` at 204 and 562; `regs=` at 416. T-16b then added the platforms job with the old form.
- **Shown (ran).** I ran a copy of the step under `bash --noprofile --norc -eo pipefail`, with `date` made to fail. The
  step went on and wrote `JOB_START=` (empty). The plain assignment `job_start="$(date +%s)"` stopped the script at once
  (exit 1).
- **Consequence.** `JOB_START` empty makes the summary's duration line print the epoch as a duration. No check is
  hidden, and the job is dispatch-only. **Not an a5 blocker.**
- **Fix (T-16e, which owns `platforms`).** `job_start="$(date +%s)"`, then `echo "JOB_START=$job_start"` in the block.
- **Ruling proposal on SF-1.** Close 6c's SF-1 as fixed by T-21 (`20bbc80`): every site it named is an assignment now.
  Record `ci.yml:771` as a new should-fix that T-16b reintroduced. It does not reopen T-21 or T-16c. No other
  `echo "X=$(...)"` form remains in `ci.yml` outside a summary step (`grep -nE 'echo "[A-Z_]+=\$\('`: line 771 only).

**SF-2. The `platforms` skip gate does not fail when an allowed skip goes missing, and it matches one category by
reason only (`tools/ci_platform_venvs.sh:198-223`).**
- **What the header says** (lines 130 and 131): `tests/test_paths.py` with the reason "POSIX byte file names".
- **What the code does:**
  - it counts and drops lines by the reason alone (lines 200 and 201);
  - it never checks the count. Only the probes (`!= 3`, line 220) and the release tooling (`!= 9`, line 216) are
    exact.
- **Shown (ran).** A harness ran the real script with stubbed `uv` and `uname` and fed it synthetic `pytest -rs` outputs
  (macOS form, kernel `Darwin`):

  | case | exit | expected |
  |---|---|---|
  | the macOS skips as recorded (3 + 8 + 9) | 0 | 0 |
  | one unknown skip added | 1 | 1 |
  | probes missing | 1 | 1 |
  | release tooling 8 instead of 9 | 1 | 1 |
  | "POSIX permissions" skip on macOS | 1 | 1 |
  | **"POSIX byte file names" missing entirely (0 of 8)** | **0** | 1 |
  | **"POSIX byte file names" 7 of 8** | **0** | 1 |
  | **that reason from another file (`tests/test_truth.py`)** | **0** | 1 |
  | pyarrow-only venv with the `tests/test_consumers.py` skip absent | 0 | 0 or 1 (not specified) |
  | Windows form `tests\test_internal.py` (kernel MINGW64) | 1 | known, ADR-125 item 2 |

- **Consequence.**
  - If those 8 tests started to run and pass on macOS, the gate would stay green. Only a human reading the counts
    (246 + 20) would notice.
  - It contradicts the script's own header and ADR-125 item 2 ("matches on the path or the reason consistently, as
    written in the script's header").
  - For T-16f and the a5 runs, where macOS wheels may ship, the gate should enforce what the release relies on.
- **Fix (T-16e item 7, the same gate).** Off Linux, require exactly 8 "POSIX byte file names" skips, all in
  `tests/test_paths.py`. Match each category on path **and** reason. The self-test gets the three bold cases.
- **Not an a5 blocker**, because the counts are also compared by a person. If Q-15 ships macOS, T-16f should not start
  without this fix (see the Q-15 list).

**SF-3. The order of T-16e, T-16f and a5 lets unreviewed T-16e code reach the a5 candidate.** (Plan, order table of
session 33: rows 3' and 4, lines 4312 and 4313; ADR-123 item 6, `decisions.md:5967-5970`.)
- **The rules as written.** T-16e "starts after Reviewer 7" and "joins a5 only if its commit is reviewed (Reviewer 8 …)
  before a5's version commit; otherwise a6". Row 3' (T-16f) "runs in parallel with" row 4 (T-16e).
- **The scenario.** `master` is linear, and the a5 version commit C is made on its head. T-16e changes
  `crates/oradmp-py/src/errors.rs`, `python/oradmp/_cli.py`, the release gate `tests/check_dist.py` and the macOS skip
  gate. If T-16e is pushed before Reviewer 8 has passed it, C contains it unreviewed. "Otherwise a6" then cannot hold
  unless C is made on an older base, which nothing in the plan describes.
- **Second conflict.** T-16f and T-16e both edit the `platforms` job and `tools/ci_platform_*` in parallel.
- **Fix (Architect, before T-16e starts).** Say where T-16e's commits live until Reviewer 8, for example "not pushed to
  `master` before the a5 tag unless Reviewer 8 has passed them", or that a5 waits for Reviewer 8. Order T-16f and T-16e
  one after the other.
- No change to the candidate.

### Notes

**N-1. The `manylinux2014` and macOS counts are recorded in mixed orders, without venv names.**
- T-16c's note (`implementer-notes.md:3262`) names each venv. Rocky 8 there is 260, 253, 260 (pa15, pyarrow-only,
  py314), which is the order the job summary prints (alphabetical glob).
- T-16d's record (`implementer-notes.md:3641`), the plan (`implementation-plan.md:2222`) and ADR-129 item 4
  (`decisions.md:6248`) give Rocky 8 as "260, 260, 253" and the runner as "263, 263, 256", without names. That is
  neither the printed order nor named.
- The multisets agree, and the pyarrow-only venv must be the +7 / +4 one. So I read this as a reordering, not a
  different result.
- The a5 expected figures below are by venv name.

**N-2. ADR-116 item 2 (the checks held for this review) can only be closed by substitutes from the repository.**
- (a) **The SBOM and `RECORD` sizes of T-21's run against C's, and T-16c's member-by-member comparison.** Neither is
  recorded, and I cannot read the public summaries.
  - Substitute (ran): I built 4b9883c (C), 20bbc80 (T-21), 94b6368 (T-16c), 9a9bc84 and 4ed52e5 with the current
    script, all at one scratch root.
  - Between each neighbouring pair, all 69 members (16 + 53) are identical except
    `oradmp-0.1.0a4.dist-info/sboms/oradmp-py.cyclonedx.json` (134,581 bytes in all five) and `RECORD` (1,293 bytes in
    all five). Both differ in content only.
  - `_core.abi3.so` is `d0efa2df…` (12,988,104 bytes) in all five.
  - So the sizes are equal across commits at one root, which is what ADR-112 item 4 (b) requires. CI's own sizes at the
    canonical root stay unread.
- (b) **The container venvs' pyarrow, numpy and pandas versions.** These are not in the notes either.
  - Substitute (ran): `pip download --only-binary :all: --platform manylinux2014_x86_64` gives today:
    - cp311: pyarrow 20.0.0, numpy 2.2.6, pandas 2.3.2;
    - cp313: the same three;
    - cp314: "No matching distribution" for all three.
  - That is what the T-16c note expects (lines 3188 to 3190, 3242 and 3243), and it supports the README ceiling "pyarrow
    up to 20.0.0" (ADR-117 item 4) from the index, not from a run.
  - **For the a5 record:** copy the container venvs' info lines (Python, pyarrow, numpy, pandas) of the dispatch on C,
    so the README claim has run evidence.

  Neither substitute shows a defect in T-16c. Nothing is reopened.

**N-3. `tests/test_release_build.py` assumes a fixed shared path is absent** (`OTHER =
"/var/tmp/oradmp-release-t16a2-not-created"`, line 17; used at lines 53 to 58, 73 to 98).
- If that folder existed, five tests would fail with "exists already".
- This fits none of ADR-119 item 2's (i) to (iii). The T-16c cases (lines 71 to 98) were added before the rule bound
  (it binds from T-16b on).
- Nothing creates the folder, so the risk is low.
- Fix when the file is next touched: a root below `tmp_path` that is never created, which is form (i). The non-`--release`
  cases accept any root.

**N-4. The `ci.yml` header is stale.**
- Line 1 still says "Linux only".
- The job list (lines 3 to 10) omits `platforms`.
- Line 15 ("an upload except the fuzz crash artifact") and the `platforms` comment (lines 706 to 708, "never uploaded,
  not even as an artifact") must change with T-16f.

**N-5. `tools/ci_platform_build.sh:105` passes `"$(ls "$ROOT"/dist/oradmp-*.tar.gz)"` as an argument of `check_dist.py`.**
If `ls` fails, `check_dist.py` gets an empty path and fails, and the step still turns red. So this is the rule's form
without its harm. Fix with SF-1 in T-16e (`sdist="$(ls …)"` first).

**N-6. `tools/ci_container_2_17.sh:45` puts `python -m venv` and the pyarrow probe in one `elif`.**
A failing 3.14 venv creation would be reported as "the pyarrow install failed (expected …)". It is informational only;
fix it when the file is next touched.

**N-7. The line-ending probe (`ci.yml:806`) prints "0" for a missing file** (`tr … < file | wc -c` inside an `echo`).
When T-16e item 9 turns the probe into a gate (`.gitattributes` now has `*.sh text eol=lf`, `bd40f54`; all 33 scripts
are `i/lf w/lf attr/text`), the gate must fail on a missing file.

**N-8. Mutations of `tools/release_build.sh` that `tests/test_release_build.py` does not catch.** I ran each alone, in a
copy.

| mutation | pytest | what catches it in CI |
|---|---|---|
| value check removed | fails 1 | none needed |
| missing-value check removed | fails 1 | none needed |
| wheel-name check removed | passes 9 | nothing |
| 2014 maps to the `manylinux_2_28` compatibility | passes 9 | the script's wheel-name check, `ci.yml:595-598` |
| root-exists check removed | passes 9 with the root absent | the `wheel` job's root-present state |
| `manylinux:` line not printed | passes 9 | `ci.yml:584` |

These are build-time checks, which T-16c (a) did not ask to unit-test. Recorded, no action.

**N-9. The arm64 verdict rests on dispatch 3 alone.**
- ADR-121 item 2 made "accepted" conditional on a per-venv record. For arm64 that record never existed.
- ADR-126 item 5 keeps the verdict and defers the record to T-16f.
- a5's README line for arm64 ("the test suite passed …", ADR-122 item 3) therefore rests on the owner's pasted logs of
  `a726271` only. The README wording should say no more than that.

**N-10. The DuckDB documents against ADR-110 and ADR-90** (read only).
- **Release page.** ADR-110 item 4 says the release page has "no files attached (PyPI is the one place to download)".
  Q-16's recommendation (`decisions.md:6399-6406`, `duckdb-extension.md:150-151`) is a GitHub release asset. ADR-110
  item 4 needs an amendment when T-27 is planned.
- **The CI comparison.** "Equal to the `duckdb` job's build on C" needs that job to build through
  `release_build.sh --duckdb` at the canonical root, the ADR-90 lesson. ADR-130 item 8 does not say so.
- **Test collection.** `tests/duckdb/` must be kept out of the root pytest collection explicitly.
- **The tree.** There is no code: `crates/oradmp-duckdb/` and `tests/duckdb/` do not exist, and the root workspace
  lists two members.
- **The archives.** Wheel and sdist member lists are unchanged from `9a9bc84` (N-2: identical except the SBOM and
  `RECORD`).
- No contradiction with ADR-90 or with the `release_build.sh` root rules.

**N-11. Commit subject.** `9a9bc84`'s subject is "ci flow and implementer-notes.md" (no task name). It is a record
point only; no rewrite.

## The items of the prompt

### 1. Tasks since 6c (read, plus the builds of item 10)

- **T-21 (`20bbc80`).** Under `crates/` only comments, two generator docstrings (`uv run` became `python3 -I`) and two
  test names changed (no assertion).
  - `tests/test_contributing_pins.py`: a mutation (ruff 0.17.1 to 0.17.2 in a copy of `CONTRIBUTING.md`) fails 1 of 6
    cases (ran).
  - The `ci.yml` assignments: see SF-1.
  - Its sdist rebuilt at my root = the recorded `81cd7fe1…` (ran).
- **T-16c (`a8da2cc`, `94b6368`, `0bfa70d`).**
  - Option parsing comes before any folder (`release_build.sh:34-65`, before the root rules at 88 to 114).
  - The wheel-name check is at lines 145 to 148.
  - The three scripts:
    - every install `--only-binary :all:`;
    - digests in `ci.yml:524` and `:526` = the note;
    - `set -eo pipefail`;
    - assignments only;
    - root-only skips counted exactly (3) and only when uid 0;
    - probes exactly 3;
    - nothing written outside `/work` and `/out`;
    - no secret.
  - The `manylinux2014` job keeps no artifact.
  - `94b6368`'s test asserts both root states. The second state is shown in CI only (recorded S, push run on `94b6368`:
    `checks` 266 with 0 skips, `wheel` green).
- **T-16b (`bdc507b`, `89b7857`, `a726271`).** See items 3, 4 and 9, SF-2 and N-9.
- **T-16d (`9a9bc84`).** See item 5.

### 2. The two T-16c checks and the member count

See N-2. "69 members (16 + 53)": I counted them in my build of 4ed52e5 (ran). No document says 68 for the member count
except `docs/prompts/architect-30-update.md` and the places that quote the correction (`decisions.md:57`, `:5566`,
`open-questions.md:46`, `implementer-notes.md:3266`).

### 3. `ci.yml`, end to end (read; some steps ran as copies)

- **Default shell.** `defaults.run.shell: bash` (line 35). No step sets `shell:`. The `platforms` "Shell check" proves
  pipefail by `(false | true)` (the `set -o | grep` line succeeds either way; the subshell test is the real one).
- **Actions.** Every `uses:` is a 40-hex SHA with a version comment. The five SHAs are the ones of `4b9883c`, and
  T-16b/T-16d added no new action. I could not resolve the pins against their tags (no GitHub).
- **No secret.** No `secrets.` and no `GITHUB_TOKEN`. `permissions: contents: read`. `persist-credentials: false` on
  all six checkouts.
- **Uploads.** One upload, the fuzz crash files on `failure()` (line 507).
- **Dispatch-only jobs.** `extended`, `manylinux2014` and `platforms` each have
  `if: github.event_name == 'workflow_dispatch'`. The concurrency group includes the event name, so a push does not
  cancel a dispatch.
- **`timeout-minutes`** on every job: 45, 60, 30, 90, 90, 90.
- **Pipelines.** Every `| tee` of a command whose status matters runs under pipefail.
- **`$(...)` in arguments.** All are inside summary steps or are informational, except SF-1 (line 771) and N-7
  (line 806). Line 447 (`test "$(ls … | wc -l)" = 1`) and line 596 fail the step through the test.
- **Windows handling.**
  - `core.autocrlf=false` runs before the checkout, only `if: runner.os == 'Windows'`.
  - `MSYS2_ENV_CONV_EXCL` is set only in the `cygpath` branch of `ci_platform_build.sh:63-70`.
- **SF-1 ruling:** see SF-1.

### 4. The scripts (read; the gate harness and the mutations ran)

- **`release_build.sh`.**
  - The diff since a4 is only the option loop (`for` became `while`/`shift`), `--manylinux`, `compat`/`platform_tag`,
    the wheel-name check, the `manylinux:` line and the help range (2 to 26; it still cuts the last header line, as
    2 to 22 did at a4).
  - The canonical-root rule (lines 94 to 99), the root checks before the commit (114, 116), `--twice` (166 to 208) and
    `--release` are unchanged.
  - **Default route unchanged, shown (ran):** the current script on a4's `4b9883c` gives the released a4 sdist
    `510f2e39…` and `_core.abi3.so` `d0efa2df…`.
- **`ci_platform_build.sh`.** Root absent first. maturin pinned and verified. `check_dist` is not weakened: its exit
  status is kept and returned. The two-build comparison never gates (by design). See N-5.
- **`ci_platform_venvs.sh`.** SF-2. Separator assumptions:
  - `tests/test_internal.py` (line 187) and the extra file `tests/test_consumers.py` (line 190) both assume `/`;
    T-16e's ADR-125 item 2 normalisation covers both;
  - the `location` check normalises `\`;
  - `ci_platform_report.py` uses `pathlib` and `glob`, and I found no separator assumption there;
  - `ci_install_check.sh` runs on Linux only.
- **`ci_platform_report.py`.** `checksums` reports "ok (0 files)" for an empty `SHA256SUMS`. That is harmless here (the
  fixture lists are committed). Otherwise it is sound.
- **`tests/check_dist.py`.** It has not changed since `4b9883c` (`git log 4b9883c..4ed52e5 -- tests/check_dist.py` is
  empty), so nothing was weakened. Its build-root rule (lines 74 to 89) is as at a4.

### 5. The `extended` sdist check (ran as a harness, read)

- **What it does.**
  1. It builds `$GITHUB_SHA` with `release_build.sh` (no `--release`) at `/var/tmp/oradmp-sdist`, after `test ! -e`.
  2. It requires `check_dist: ok` and exactly one `.tar.gz`, and prints that file's sha256.
  3. It installs that file into a fresh uv 3.11 venv. This is a source build: `pyproject.toml` pins `maturin==1.15.0`
     as the build backend.
  4. It checks that the package is in site-packages, that `--version` equals Cargo's version, and that
     `survey --no-names` on a copied fixture exits 0 with output.
- **What fails it.** I ran both step bodies under the file's shell template with stubs. Each of these turns a step red:
  release build failing in front of `tee`, no `check_dist` line, two `.tar.gz`, `uv pip install` failing in front of
  `tee`, a wrong version, survey exit 3, empty survey output.
- **The one silent part** is the cross-job comparison with the `wheel` job's sdist hash. It is made by a person, as
  ADR-126 item 2 accepted. The a5 checklist must name it.
- **Root and hash (record point (f)).** The root is right: the script refuses `RUNNER_TEMP` (inside `$HOME`). The sdist
  does not depend on the root: my two roots for 4ed52e5 gave the same sdist.

### 6. The evidence rule (ADR-126 item 7) and the figures

- **Shown (ran).** At one root, 9a9bc84 and 4ed52e5 give other wheel and sdist hashes:
  - wheels `29d7aa13…` and `b9eb6962…`;
  - sdists `bd274adc…` and `eb66c661…`.

  Yet every sdist member is identical, and the wheel members differ only in the SBOM and `RECORD`. The SBOM differs only
  in `"timestamp"` (`2026-11-04T12:57:09Z` against `2026-11-07T17:32:32Z`, which is each commit's `%ct`). The ZIP entry
  times are the epoch, rounded to even seconds.
  - This settles ADR-56's SBOM point and the "not checked byte by byte" line of ADR-126 item 7, for the wheel.
  - The tar and gzip header times I did not dissect.
- **Cross-commit hash comparisons.** None of `ci.yml`, the plan or the notes compares archive hashes across commits in a
  live check. The only line that does is T-16d's pre-rule template (`implementer-notes.md:3560-3561`, "dispatch 1 had
  `ec57e7b7…`" as the expected value on another commit). That template is superseded by the results section, so no
  action is needed. T-16b (f) and T-16e item 6 compare member lists, and T-16e item 13 states the rule.
- **Hash consistency (ran a script over `docs/design/` and `docs/review/`).**
  - No 8-hex prefix maps to two different full hashes.
  - Every abbreviated hash of this line (`4327e1a7`, `0e2dd886`, `f8517e34`, `bd274adc`, `d837ac77`, `d0efa2df`,
    `aa6ef3ac`, `df3be632`) matches its full form.
  - The CI hashes for 4ed52e5 (`eb66c661…`, `022adedc…`) are not in the repository yet.
- **Recorded sdists I rebuilt byte for byte at a scratch root** (ran):

  | commit | sdist |
  |---|---|
  | `4b9883c` | `510f2e39…` |
  | `20bbc80` | `81cd7fe1…` |
  | `94b6368` | `0e2dd886…` |
  | `9a9bc84` | `bd274adc…` |
  | `4ed52e5` | `eb66c661…` |

### 7. Q-15: what the rule 8 amendment needs

ADR-128 gives the wording and most of the evidence. **It is enough to write the rule if these points are added** (the
missing ones are marked new):

1. **Scope.** macOS arm64 and x86_64 wheels only, from the `platforms` job of the workflow at the tagged commit C.
   Linux files remain `release_build.sh`-only. Windows is excluded (ADR-122, ADR-124).
2. **Two separate runs on C.** A dispatch or a re-run attempt on a fresh runner counts (ADR-126 item 6). Each run is
   recorded with its run id and attempt. Both are green, the wheel SHA-256 and the member lists are identical, and the
   in-job double build is identical in each.
3. **Where the upload happens (new).** In the `platforms` job, on the two macOS legs only, as the last step before the
   summary:
   - only the first build's wheel (`$PLAT_ROOT/dist/oradmp-*.whl`);
   - `actions/upload-artifact` pinned by SHA, `retention-days: 7`, `if-no-files-found: error`;
   - **with the default `success()` condition, never `always()` or `!cancelled()`**. The build, clone and venvs steps of
     this job use `!cancelled()` (`ci.yml:848-861`), so an artifact could otherwise come from a leg whose nextest step
     was red;
   - no sdist, no log, nothing on Windows.

   Today the job uploads nothing, and its comment (lines 706 to 708) and the file header (line 15) forbid it.
   ADR-87 item 6 and ADR-92 item 1 are amended first.
4. **The hash compared (new).** The SHA-256 of the **wheel inside** the downloaded artifact ZIP, not of the ZIP, equals
   the wheel line of both run summaries.
5. **Who downloads and how (new).** Only the human, through the web page or with a token made for this step and revoked
   afterwards (ADR-111 item 5). Artifacts of a public repository still need a signed-in account. No agent ever sees it.
6. **Expiry (new).** The artifact must come from one of the two compared runs. If it has expired (7 days), new runs on
   C are needed, and their hash must equal the recorded one.
7. **Evidence a macOS wheel must carry** (ADR-128 list):
   - two summaries on C;
   - identical wheel hash and member list;
   - in-job identity;
   - nextest 488 + 4;
   - `check_dist: ok`;
   - the three venvs at their counts with the allowed skips only;
   - the sdist equal to the Linux sdist of C;
   - the downloaded wheel equal to both;
   - the tag `cp311-abi3-macosx_*`;
   - one upload with the Linux files;
   - **new**: the platform tag's minimum macOS version recorded (`macosx_10_12_x86_64` and `macosx_11_0_arm64` claim
     older systems than the macOS 15 runners tested, and pyarrow 15.0.2's own floor is 10.15). Either the README states
     the tested version or T-16f sets `MACOSX_DEPLOYMENT_TARGET` deliberately (an Architect choice).
8. **The skip gate must be exact before T-16f** (SF-2).
9. **Consequential edits to the a5 gates (new).**
   - ADR-117 item 3 (g): `SHA256SUMS` has three lines, which becomes three plus the macOS wheels.
   - Item 3 (f) gains a selection check for each macOS file (`pip download --platform macosx_…`).
   - Item 3 (h): one upload, nothing added later.
10. **Rule 8's first sentence** ("A release is built only … with `tools/release_build.sh`") must carve out the macOS
    wheel explicitly. The DuckDB asset (Q-16 yes) stays under `release_build.sh --duckdb` and needs no rule 8 change,
    but ADR-110 item 4 does (N-10).

### 8. DuckDB documents

See N-10.

### 9. Windows

- Every Windows branch is guarded:
  - `if: runner.os == 'Windows'` on two steps;
  - the `Windows)` arm of the Environment `case`;
  - `command -v cygpath` in the Environment step and in `ci_platform_build.sh`;
  - `MINGW*|MSYS*|CYGWIN*` in the gate;
  - `python.exe` only if the file exists.
- The rest are no-ops on POSIX: `tr -d '\r'`, `${…//\\//}`, `.replace(chr(92), "/")`.
- The Linux jobs use none of the platform scripts. The two `sys.platform == "win32"` skipifs in `tests/test_cli.py` do
  not change Linux or macOS. Nothing for Linux or macOS depends on a Windows branch.
- No other Windows defect beyond those known (T-16e).

### 10. Build and checks in my clone (ran)

**Checks.**
- Fixture checksums: 3 of 3 ok (clone and repository).
- `cargo fmt --check` ok.
- clippy `--all-targets` and `--all-targets --all-features`: both ok.
- nextest **488 passed, 4 skipped**.
- `cargo deny check`:
  - advisories, bans, licenses, sources ok;
  - warnings: the duplicate `syn` and two "license was not encountered" (ISC, Zlib), the expected three of ADR-112
    item 2.
- `maturin develop --features test-probes` ok.
- `ruff check` ok. `ruff format --check`: 31 files ok.
- mypy: 25 files, no issues.
- pytest **266 passed, 0 skipped**.

**Release build.** `ORADMP_BUILD_ROOT=/var/tmp/oradmp-review-7/rb-4ed52e5 tools/release_build.sh 4ed52e5`:
- `check_dist: ok`, `manylinux: 2_28`, epoch 1794072752 (= `%ct`);
- **sdist `eb66c661dd0965917124f35ad25cbe1bd9b569fea561ad45763ececc2e003a8f` = CI's**;
- wheel `8840a852…`, not CI's `022adedc…`, as expected.

**The wheel members.**
- `oradmp/_core.abi3.so` = **`d0efa2dff42b57f5203429ace3b7789d4d1cfbbb7fd09ce986b5badf51b4789c`**.
- Without CI's member list I cannot diff against it directly. But a second build of 4ed52e5 at another root differs
  from the first **only** in the SBOM (134,629 against 134,581 bytes: the root is 8 characters longer, 6 times) and
  `RECORD` (same size).
- So the root reaches no other member, and the only members that can differ from CI's are those two.

### 11. Record points (a) to (i)

- **(e) The four points.**
  - `tools/check_dist.py` against `tests/check_dist.py`: the Implementer used the plan's name (note "Deviations" item 1);
    no consequence.
  - Path separators: see item 4; only the known `\` cases, for T-16e.
  - The dispatch fix pushed before green: green afterwards (S); no consequence found.
  - `a726271`'s message is its subject plus the co-author line only. **The tree is not affected**: at `a726271`,
    `ci.yml` has `${{ !cancelled() }}` intact at all six `if:` lines (550 to 804 there). The T-16b note's
    "Dispatch 2 … and fix 2" section (`implementer-notes.md:3468-3506`) carries what a body would have said.
- **(f) The sdist root.** See item 5.
- **(g) The 2014 pre-check.** Deviations (a) to (c) of ADR-126 item 3 are as recorded.
  - The VM wheel `f8517e34…` and sdist `bd274adc…` equal both dispatches (S).
  - Read-only, I found the pre-check root kept as `/var/tmp/oradmp-release.9a9bc84`, not the name ADR-126 item 3 (c)
    gives, and not deleted. Its `SHA256SUMS` holds exactly those two lines. It has no `dist2` (fits "no `--twice`"),
    and its `src` is at `9a9bc84`.
  - The canonical root is absent today, so the open part of a5 condition 2 holds now. The owner still has to choose
    whether to rename or delete that folder.
- **(h)** See item 6.
- **(i) macOS x86_64 on `9a9bc84`** (S):
  - py311-pa15-pd223 246 + 20, py311-pyarrow-only 239 + 21, pynewest-latest (3.14.7) 246 + 20;
  - only the allowed skips: 3 + 8 + 9, plus the pandas module;
  - this matches 246/246/239 by name.
  - arm64: no record exists (N-9).

### Machine-state inventory (ADR-119 item 3 (d); the grep gives 35 lines, `crates/*/tests/` none)

| hit | class |
|---|---|
| `test_paths.py:22` (off Linux), `:121` (root); `test_cli.py:594, 923` (root), `:595, 924` (Windows) | (iii), allowed (ADR-119, ADR-118 (b)) |
| `test_paths.py:32`, `test_cli.py:976` (`dict(os.environ, PYTHONPATH=…)`) | (i) |
| `test_consumers.py:18` `importorskip("pandas")` | (iii) |
| `test_project_name.py:33` (`shutil.which("git")`, not a git clone) | (iii) |
| `test_release_build.py:16`, `:88-98` (canonical root present or absent) | (ii) |
| `test_release_build.py:21` (Linux-only) | (iii), ADR-121 item 1 |
| `test_release_build.py:25` (strips `ORADMP_BUILD_ROOT`) | (i) |
| `test_release_build.py:17` `OTHER` assumed absent | **none of (i) to (iii)**: N-3 |
| `test_check_dist.py` `/var/tmp/...` strings (data inside archives built in `tmp_path`) | no machine state |
| `check_dist.py:72-74` | the tool itself, not a test |

Beyond the grep, the Rust tests:
- `t3_catalog.rs:1054` `#[cfg(unix)]`: (iii), ADR-123 item 5;
- the `t15_*` tests and `write_fuzz_seeds` (`ORADMP_WORK`, `ORADMP_SEED_DIR`): the four ignored tests;
- `t17_synth.rs:365` (`PROPTEST_CASES` with a default): (ii);
- scratch folders under `target/` (`t3_catalog.rs:606`, `t2_manifest.rs:103`): (i).

### Expected figures for the a5 dispatch on C (ADR-129 item 4), by venv name and in summary order

These assume no test is added between 4ed52e5 and C. If T-16e or T-16f join, add their new tests. The version reads
`oradmp 0.1.0a5`. `_core` hashes are **not** fixed, because the version is compiled into the library.

| job / where | expected |
|---|---|
| `checks` | nextest 488 passed + 4 skipped (the four names); `cargo deny` four ok with the three warnings above; pytest 266 passed, 0 skipped |
| `wheel` | `release root: canonical`, `manylinux: 2_28`, `check_dist: ok`; 69 members (16 + 53) unless the version commit adds files; py311-pa15-pd223 263 + 3; py311-pyarrow-only 256 + 4; pynewest-latest 263 + 3 |
| `extended` | green; fuzz read, walk, values 0 findings; t14_damage 5 passed; t17_synth 30 passed + 1 skipped; sdist sha256 = the `wheel` job's of C; version as above; survey ran |
| `manylinux2014`, build | `manylinux: 2014`, `check_dist: ok`, `auditwheel` `manylinux_2_17_x86_64` with symbols up to `GLIBC_2.14`, both `Tag:` lines; sdist = the `wheel` job's |
| glibc 2.17 | py311-newest 260 + 6 (pyarrow 20.0.0, numpy 2.2.6, pandas 2.3.2 expected); py311-pa15-pd223 260 + 6; py313-newest 253 + 7 (pyarrow 20.0.0); 3.14: the pyarrow install fails, as expected |
| Rocky 8 | py311-pa15-pd223 260 + 6; py311-pyarrow-only 253 + 7; py314-newest 260 + 6 |
| runner | py311-pa15-pd223 263 + 3; py311-pyarrow-only 256 + 4; pynewest-latest 263 + 3 |
| `platforms` macOS, each leg (when macOS ships: on both runs) | nextest 488 + 4; `check_dist: ok`; two builds identical; py311-pa15-pd223 246 + 20; py311-pyarrow-only 239 + 21; pynewest-latest 246 + 20 (skips 3 probes, 8 POSIX byte file names, 9 Linux-only release tooling, plus the pandas module in pyarrow-only); sdist = the Linux sdist of C |
| `platforms` Windows | red as known (nextest 487 + 4 and the T-16e failures), unless T-16e joins |

## What I did not check

- **CI itself.**
  - Every figure marked S: the public run summaries (no GitHub access). This includes CI's SBOM and `RECORD` sizes,
    the container venvs' printed versions, the macOS summaries, the T-16d dispatch conclusions, and both states of
    `test_release_build.py` in one push run.
  - The action pins against their tags (no `git ls-remote`).
  - Anything on macOS or Windows, and the containers, which need Docker. The scripts were read, and only harnessed
    with stubs.
- **Byte-level and data checks.**
  - Byte-level identity of my wheel with CI's: I had no CI member list; I showed root confinement instead.
  - The tar and gzip header times of the sdist.
  - Whether `/var/tmp/oradmp-release.9a9bc84`'s wheel bytes equal its `SHA256SUMS` (I read the file, I did not hash
    the wheel).
  - Golden files and corpus tests: no task in this range changed them. The fixture checksums only.
- **Code and tests outside this range.**
  - The reader code: unchanged since 6c (`git diff --stat 4b9883c 4ed52e5 -- crates/*/src python` is empty).
  - Mutations of the Rust or Python reader.
  - `ci_install_check.sh`'s root branch (needs uid 0).
  - `fuzz/`.
- **Documents.**
  - The DuckDB design beyond contradictions with ADR-90, ADR-110 and the release script: no review of its API or of
    DuckDB facts.
  - The README and CONTRIBUTING beyond the pin test.
  - `docs/briefs/`.
- **Not run.** Benchmarks and the memory test.
