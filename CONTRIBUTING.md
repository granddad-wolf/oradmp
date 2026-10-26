# Contributing to oradmp

## Licence of contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
oradmp by you, as defined in the Apache-2.0 license, shall be dual licensed under the MIT license
and the Apache License, Version 2.0, without any additional terms or conditions.

## Clean room

Facts about the dump format come only from `docs/formats/` and the fixtures in `corpus/fixtures/`. Do not read,
search for or copy the source code, or documentation derived from it, of any third-party Oracle dump parser or
exporter. If you are unsure whether a source is allowed, ask in the pull request before you read it.

## Development setup

```
uv venv .venv
uv pip install --python .venv/bin/python maturin pytest pyarrow ruff mypy pandas     # pandas is optional
.venv/bin/maturin develop --features test-probes    # builds the extension and installs the package (editable)
```

`--features test-probes` adds `oradmp._core._probe_panic`, which `tests/test_internal.py` needs (ADR-59); without
it those three tests skip with that reason. A release build never has the feature (`tests/check_dist.py` rejects a
wheel that does).

The checks of the definition of done:

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo nextest run
cargo deny check licenses
.venv/bin/ruff check
.venv/bin/ruff format --check
.venv/bin/mypy
.venv/bin/pytest
```

Fixtures in `corpus/fixtures/` are read-only: tests open them and write only below `target/` or a temporary
directory. Real data never enters the repository.

## Fuzzing

The three fuzz targets (`walk`, `read`, `values`) live in `fuzz/`, a crate of its own outside the workspace.

Prerequisites: the dated nightly named in `fuzz/rust-toolchain.toml`
(`rustup toolchain install nightly-2026-10-15 --profile minimal`), `cargo install cargo-fuzz --locked --version
0.13.2`, and a C++11 compiler. The root `rust-toolchain.toml` (1.99.0) stays for everything else; rustup takes the
fuzz pin only when the command runs inside `fuzz/`, which is why the script changes into that folder.

    tools/fuzz_run.sh <walk|read|values> [seconds]      # default 600

The script copies the fixtures to `target/fuzz-seeds/` when that folder is missing (`tools/fuzz_seeds.sh` does it,
and also writes `SYNTH_SEEDS` (default 200) synthetic files `synth__NNNN.dmp` there with the generator of the tests,
`crates/oradmp-core/tests/common/synth.rs`; `SYNTH_SEEDS=500 tools/fuzz_seeds.sh` writes 500). The seed step compiles
the `oradmp-core` test binary with the root toolchain (the script changes to the repository root first, so the result
does not depend on the caller's directory), which takes a build of its own the first time. The script then creates
`fuzz/corpus/<target>/`, puts `TMPDIR` into `${ORADMP_FUZZ_TMP:-/var/tmp/oradmp-fuzz}` (it stops when less than
2 GiB are free there; `/tmp` can be a small tmpfs), runs libFuzzer with the limits of the plan (`-max_len=1048576
-timeout=10 -rss_limit_mb=2048 -malloc_limit_mb=64 -jobs=1 -workers=1`), removes the `oradmp-fuzz-*` folders and moves
`fuzz-0.log` into the tmp folder afterwards, and exits non-zero when libFuzzer fails or `fuzz/artifacts/<target>/` is
not empty.

Machine rule: one build at a time (`CARGO_BUILD_JOBS=2`), and a fuzz run alone, with no other build, benchmark or
test running.

A finding (a file in `fuzz/artifacts/<target>/`) is never fixed by changing a test: reproduce it with `cargo fuzz run
<target> <artifact>` in `fuzz/`, minimise it (`cargo fuzz tmin`), put a regression test with a fixture patch or a
minimised file below 4 KiB into `crates/oradmp-core/tests/t14_fuzz.rs` (and `crates/oradmp-core/tests/fuzz/`), and
check that the test fails on the code before the fix. A fix that would change an error code, a warning, a fixture's rows
or a rule goes to `docs/design/implementer-notes.md` for the Architect. `fuzz/corpus/`, `fuzz/artifacts/` and the logs
are not committed.

## Release wheel

    tools/release_build.sh [COMMIT] [--twice] [--release]

The script builds the wheel and the sdist of a commit (default `HEAD`) in a fresh clone below
`/var/tmp/oradmp-release` (`ORADMP_BUILD_ROOT`), with its own venv, remapped paths and a fixed
`SOURCE_DATE_EPOCH`, runs `tests/check_dist.py` on both files and writes `dist/SHA256SUMS` (ADR-56). A wheel built
by hand in the working copy is for tests only.

A release build uses `--release` (ADR-90): the script then refuses any build root but `/var/tmp/oradmp-release` (the
canonical root, which must not exist yet) and prints `release root: canonical`; `check_dist.py --build-root` checks on
every build that the build paths in the wheel's SBOM lie below the root. At the canonical root, `d081109` gave the same
wheel and the same sdist on the build VM and on GitHub's `ubuntu-24.04` runner; a wheel built at another root differs
in the SBOM member and `RECORD` only. Nothing is claimed for any other machine.

The version comes from the root `Cargo.toml` only. Publishing follows the release checklist
(`docs/design/implementation-plan.md`, "First release"); only the project owner tags and uploads.

## CI

`.github/workflows/ci.yml` (GitHub Actions, Linux `ubuntu-24.04`) runs on every push to `master` and on
`workflow_dispatch`. A push runs two jobs. `checks`: the three `SHA256SUMS`, `cargo fmt --all --check`, `cargo clippy`
(all targets, and all targets with all features), `cargo nextest run -j 2` (exactly the four ignored tests are
skipped: the three `t15_*` tests and `write_fuzz_seeds`), `cargo deny check`, a probes build, `ruff`, `mypy` and
`pytest` (0 skipped). `wheel`: `tools/release_build.sh` on the pushed commit (`check_dist: ok`), then the wheel in
three fresh venvs (Python 3.11 with pyarrow 15.0.2 and pandas 2.2.3; the newest CPython with the newest pyarrow and
pandas; Python 3.11 with pyarrow only), `pytest` from the checkout against the installed package, and `oradmp
--version`. Two more jobs run only on `workflow_dispatch`: `extended` (fuzz seeds, the three fuzz targets for 60 s
each, `PROPTEST_CASES=10000` for `t14_damage` and `2000` for `t17_synth`, nightly clippy of `fuzz/`, a benchmark
report that is never compared with `benchmarks.md`) and `manylinux2014` (a `--zig` wheel, `auditwheel show` and the
abi3 tag; the wheel is not shipped). CI never publishes, never uses a secret, never touches `corpus/work/` and never
runs the memory and throughput tests (they stay `#[ignore]`). The one upload is the `fuzz-artifacts` artifact of the
`extended` job when it fails: the files of `fuzz/artifacts/` (crash inputs made from the fixtures and the synthetic
generator), kept 14 days. The `wheel` job calls the release script with `--release`.

To read a run, open its page, then the page of each job and its **Summary**: it lists the commit, the runner image,
the toolchain, Python and pyarrow versions, the nextest and pytest counts with every skipped name and reason, the
`check_dist` line, the SHA-256 of every built file and the job's duration. Paste the summaries (and, for a failed job,
the log of the failed step) to whoever asked for the run.
