# Review 03: T-11b, T-12, T-13, T-13f, T-13g (Python layer, packaging, rename) and the older unread tests

Reviewer session 3, 2026-10-18. Prompt: `docs/prompts/review-03-prompt.md` (to be saved as
`review-03-python-packaging.md`; untracked at the start, not touched). Reviewed: **HEAD `3a1bb5f`** (T-13g), with
`3fe33e7` (T-13f), `e2eaae6` (T-13) and `a19d873` (T-11b and T-12, one commit) below it. All five tasks are
committed; the working tree had no change except the untracked prompt, so HEAD is what was reviewed.

Method. `git clone` of HEAD into `<home>/.cache/review-03/repo` (a clone, so that maturin honours
`.gitignore`; `/tmp` is a 3.9 GB tmpfs, 94 % full; no path contains the old name), Cargo target directory
`<home>/.cache/review-03/target`, `CARGO_BUILD_JOBS=2`, `cargo nextest run -j 2`, one build at a time. Own venv
(`uv venv`, Python 3.13.5, pyarrow 26.0.0, pandas 3.0.6, pytest 9.1.1, mypy 2.4.0, ruff 0.17.1, maturin 1.15.0,
ziglang 0.17.0). Baseline in the clone: `cargo fmt --check` clean, `cargo clippy --all-targets -- -D warnings`
clean, `cargo nextest run` **395 passed, 0 skipped**; `maturin develop`, `ruff check`, `ruff format --check`, `mypy`
clean, `pytest` **73 passed, 0 skipped** (pyarrow 26.0.0: the interval skip did **not** happen). Every mutation
was applied alone by a script (`mut.py`, exact text replaced, occurrence asserted), the full suite run with
`--no-fail-fast`, the clone restored with `git checkout -- .` (`git status` clean after each). Binding and Python
mutations: `maturin develop` (build failure detected and the run discarded), full `pytest`. The Analyst's tools
were run read-only in the clone (`python3 -I -B`). Nothing was installed under the old name; `import oradump` was
only attempted in a fresh venv that holds `oradmp` and `pyarrow` alone.

## Summary

- **Blocking: none.**
- **Should fix: 5.** S-1: a read through the Arrow C stream (`pyarrow.RecordBatchReader.from_stream(reader)`, the
  path the README advertises) delivers no warning at all: with the default `on_number_overflow="round"` T_NUM
  gives 4 rounded and 10 nulled values and `reader.warnings == []`, nothing issued. S-2: the release wheel
  carries the owner's home path and user name (SBOM, 6 places; and 161 strings in the `.so`, which the T-13g note
  calls clean). S-3: `check_dist.py` passes a wheel with a `.pyc`, with a home path, or a `corpus/` folder nested
  below `python/oradmp/`. S-4: the message check (d) cannot see the text of a value the lenient read nulled, i.e.
  exactly the NUMBER an `E-NUMBER-OVERFLOW` message would print; d4b was caught only by a substring coincidence,
  and d6 (ADR-46 item 3) is not caught. S-5: architecture 7.3 says the binding turns a panic into `OradmpError`;
  no code does.
- Review 02's S-1 to S-4, e3 / e4 and D3: **closed**; every mutation the review and ADR-43 / ADR-46 asked for was
  repeated (section 1). N-1 remains a note.
- Numbers: every corpus total re-derived with the tools agrees; 395 Rust tests, 73 pytest. Rule T9 still says
  **382 / 1885** (should be 386 / 1,907): not corrected by the Analyst.

## 1. Review 02's findings

| finding | what T-11b / T-12 did | shown here (scratch clone) |
|---|---|---|
| S-1 no value in any message | `t12_messages.rs`: corpus per table of raw fields (4 bytes or more), 4-char windows of the lossy text, hex of fields of 2 bytes or more as whole hex runs, cells of a lenient read; text-valued bad field `misc_exp_18c` T_FRAME row 7 A (`NNNNNNNNNN`, byte 4 -> `ff`, length `0a 00` asserted); 178 / 1 / 31 copies; self-test per form | d1 (lossy bytes in every non-text value error): fails **`no_message_of_the_fixtures_holds_a_value`** ({Text}, `months do not fit 32 bits ����G`); d2 (hex): the same test ({Hex}, `... bb9ac9ff47`); d3 (lossy text in `E-BAD-TEXT`): the same and `the_check_reports_a_value_in_each_form`; d4a (the rendered cell `1234567.0000000000` appended to `E-NUMBER-OVERFLOW`): fails ({Cell}); d4b (the failing value's own `to_text()` appended): fails, **but only by coincidence** (S-4); d5 (first 4 bytes of the patched field in `E-BAD-TEXT`): fails ({Text}, `NNNN`); **d6 (hex of the first 8 bytes of a field longer than 8 bytes): 395 passed, not caught** (S-4) |
| S-2 survey, TIMESTAMP family | `t11_options::the_counting_core_counts_years_below_1_in_the_timestamp_family_on_patched_copies` | e1 (`years_below_1` for type 12 only): fails **that test only**. Offsets checked against the fixture bytes by hand: `18c/t_ts_exp_18c.dmp` row 2 = `02 00 c1 03` @0x22bf, TS0 / TS6 / TS9 values `78 74 03 1a 11 0a 1c` @**0x22c5**, 0x22ce, 0x22d7 (the test patches these three and asserts `E-BAD-DATETIME` @0x22c5); `18c/t_tstz_exp_18c.dmp` row 2 TSTZ `0d 00` + `78 74 03 1a 14 23 38 07 5b ca 00 14 3c` @**0x22bf**; first non-NULL TSLTZ `0b 00` + `78 74 03 1a 14 23 38 07 5b ca 00` @0x233f (row ID `c2 02 02` = 101). Agrees |
| S-3 one offset per value | `t12_offsets.rs` (3 tests); `t6_patched` asserts 0x228a | Probe on the built wheel, `max_lob_bytes=20000`: `long_exp_direct_18c` T_LONG `E-LOB-TOO-LARGE` and `W-VALUE-NULLED` both @**0x228a** row 3, length 24,039; the test asserts the same for the `E-BAD-TEXT` / `W-VALUE-NULLED` pair of the cut sequence. Second chain: **T_LONGRAW** row 2 @0xa0ec (18c) and @0xa0ee (11g), T_LONG 11g @0x228b, raise and null equal; I walked the u16 headers myself (5 pieces 156 + 4 x 7,961 = 32,000; first headers @0xa0ea / 0xa0ec / 0x2289). Mutations: s3a (`W-VALUE-NULLED` of a chain at the piece offset) fails **`one_chain_value_reports_its_start_under_every_policy`**; s3b (`E-BAD-TEXT` at the failing piece) fails that test and `t6_patched::a_broken_utf8_sequence_in_a_chain_is_a_value_error`; LOB: s3d (raise at `value_offset + 2`) and s3e (null at `value_offset + 2`) each fail **`one_lob_value_reports_its_start_under_every_policy`**. s3c (drop `.at(st.start)` in `lob_piece`) passes: an equivalent mutant, the decoder's error has no offset and `ctx` fills `value_offset` (`reader.rs:377-386`); `lob_piece` has no chunk offset to misreport |
| S-4 strict table | `ARCH_7_1_STRICT` (19 lines) and `DOCUMENTED` without counts | a1 (`W-CHARSET-UNSUPPORTED` -> `E-STRICT` in `strict_error()`): fails **`arch_7_1_strict_covers_every_warning_code_and_equals_strict_error`**, `strict_charset_unsupported_is_unsupported_charset`, `t11_codes::e_unsupported_charset`; a1t (table line `W-LONG-LENGTH` -> `None`): fails `arch_7_1_strict_...` only. The 19 lines equal the session 11 table of arch 7.1 (`exp-reader-architecture.md`, section 7.1): 3 named errors, 2 `None`, 14 `E-STRICT`; each line cites 7.1 or an ADR, none the reader. Counts: `documented_count` from `exp_catalog.tsv`; I recounted T_DATE / T_TS / T_NUM 4 per version folder, T_INTERVAL2 1, total 13 + 13 = 26 |
| e3 / e4, N-1 | checkers return lists, `union_of`, per-tag break test | e4 (`11g/t_wide_exp_11g.dmp` @0x3804 `5`->`6`): fails `every_table_of_the_exp_files_is_classified_and_read`, `break_every_tag_...`, `t8_direct::every_direct_file_...`, `t2_manifest` (2). n1e4 (e4 plus the one `assert!(u.is_empty())` deleted): still fails `break_every_tag_...` (its control run reads the first T_WIDE section, which is this file). n1w18 (assert deleted, both 18c T_WIDE files patched alike): fails `break_t_wide_and_t_stat_...` (reads the 18c file) besides the checksum guards. So every T_WIDE instance is still covered by some value test; the gap is a change visible in only a file no break test reads. Rated **note** (N-1 stays) |
| D3 `Error.warning` | `t11_strict.rs` `DOCUMENTED` T_NUM2 / T_NUM3 now name `W-NUMBER-ROUNDED`; `t11_codes.rs:66` `.clone()` | Read the diff of `a19d873`: the old literal had `""` (absence allowed), the new one requires `W-NUMBER-ROUNDED`, as arch 7.1 session 11 says ("exactly when the error would have been that warning without strict"); T_NUM (integer part) stays `""`; `strict_does_not_change_the_null_policies` now uses the table, not `strict_error()`. Nothing weakened. Mutation b (`.with_warning` removed): fails `t12_warning_field::a_strict_round_error_...` and `strict_over_the_201_files_...` |

Can a row value reach a message? Core: `Error.message` / `Warning.message` are `Cow<'static, str>`; the only owned
texts are `in_piece` (`reader.rs:165`, an integer piece / chunk number) and `lob_unconfirmed_message`
(`reader.rs:1051`, the header's set id); every other constructor takes a literal; `ArrowError` never becomes a
message (`reader.rs:1509` wraps our error the other way). Binding: `to_pyerr` passes `e.to_string()` (code,
offset, integers); the Python layer adds only literals (`_usage`). So today no value can reach a message; the
check is the guard for the future (S-4 for its gaps). D5's thresholds are sound for the 16 static texts; the
gaps ADR-46 accepted (values of 1 to 3 bytes, hex with separators: probe `bytes 4e 4e 4e 4e ff` not caught) are
real and documented.

## 2. T-12: the binding (`crates/oradmp-py`, read in full, 1,271 lines) and the Python package

- No `unsafe` (workspace `unsafe_code = "forbid"`); no `unwrap` / `expect` / indexing (workspace lints, clippy
  clean); mutex poisoning handled (`lib.rs:31-33`). A panic still reaches Python as PyO3's `PanicException`
  (a `BaseException`), not `OradmpError` (S-5).
- GIL: `open`, the scan, `read_table`, `next_batch`, `digest`, `survey` run in `py.detach`; `CoreDump` and
  `TableBatches` are `Send` (the `frozen` pyclasses with `Mutex` compile). Lock order is always `inner` before
  `last`. No deadlock found; but `info()`, `warnings()` and the second lock in `scan_inner` take the dump's mutex
  while holding the GIL, so a thread that asks `dump.info` while another reads the same `Dump` blocks the whole
  interpreter until that batch ends (N-2).
- Arrow hand-over: `PyRecordBatch::new(b).into_pyarrow` per batch, `PyRecordBatchReader` for the capsule; no copy
  in our code. Release of memory was **not measured** (not checked). The capsule path takes the reader and
  stashes rows / warnings **before** the export (`lib.rs:373`), so `rows_read` stays 0 (asserted as such by
  `test_handover.py::test_the_arrow_c_stream_spends_the_reader`) and every warning of that read is lost (S-1).
- Error mapping: `ERROR_CLASSES` covers the 24 codes and `WARNING_CLASSES` the 19 (walked against `_core` by
  `test_errors.py`); `StrictError(OradmpError)`, `UnsupportedCharset(UnsupportedError)`; `E-IO` is `OSError`
  (with errno: `FileNotFoundError`), not an `OradmpError` (ADR-46 item 8, accepted; the README's "Errors are
  `oradmp.OradmpError` subclasses" omits it, N-7).
- Metadata keys: the reader writes `oradmp:type_code`, `oracle_type`, `declared`, `max_bytes`, `charset`,
  `number_overflow`, `number_scale`, `conversion` (`reader.rs:1136-1172`); **`oradmp:lob_charset`**, which arch 5.1
  (`exp-reader-architecture.md:595`) and ADR-54 item 1 list, is written nowhere (N-3).
- Warnings: issued once per entry at the end of the iteration; `stacklevel=4` (`_api.py:231`) points at the user's
  loop for `__next__` but into `_api.py:289` for `read_all()` (seen in the fresh venv) (N-2).

T-12 breaks re-run (8 of the 12, each alone): d1, d2, d3 (section 1); S-3 a = s3a and S-3 f = s3f (`in_piece`
removed: fails `one_chain_value_...` and `t6_patched::a_broken_utf8_...`); b (section 1); e (message without the set
id: fails `t12_messages::the_lob_unconfirmed_message_names_the_database_set_id` and
`t6l_ru::another_single_byte_database_set_keeps_the_stop`); h (`next_batch` without `Python::detach`, through a
helper that calls the closure): fails **`test_threads.py::test_two_threads_run_in_parallel`**. (My first h attempt
did not compile and was discarded.) Not re-run: S-3 "old reader.rs", b2, c, g.

Own Python-side breaks:
| id | break | fails |
|---|---|---|
| p1 | `class StrictError(Exception)` | `test_errors.py::test_every_core_code_has_a_python_class_and_nothing_else_does` |
| p2 | `_issue` issues nothing | 8 tests of `test_warnings.py` / `test_errors.py` (e.g. `test_counted_warnings_come_at_the_end_...`, `test_header_warnings_are_issued_at_open`) |
| p3 | binding ignores `interval_as` (`options.rs`) | `test_interval.py::test_string_mode_writes_to_parquet_and_reads_back_equal`, `..._beyond_32_bit_months` |
| p4 | binding ignores the per-read `on_number_overflow` | `test_warnings.py::test_per_read_overrides_...` only (test_truth cannot see it: its Rust digest goes through the same option parser) |
| p7 | `read_all` drops the last batch | 24 tests (test_truth x 3, test_handover, test_interval, test_warnings) |

## 3. The pytest files (10 `test_*.py` now, not 7 + 1: T-13 added `test_interval`, `test_truth`; T-13g `test_version`, `test_project_name`)

| file | expected values from | remarks |
|---|---|---|
| `test_truth.py` | `exp_catalog.tsv` (generator over `exp_parse.py`) for the sections; the documented failure codes of `t11_strict.rs` | Proves **binding fidelity**: the pyarrow table equals the core's own batches (`_core.digest`, FNV-1a in Rust, recomputed in Python), every one of the 386 sections compared, a missing or extra table fails the section count. It proves **nothing about reader correctness** (both sides are the same core; that is `t10_truth.rs`'s job), and a shared bug of the option parser is invisible to it (p4). The 26 expected failures: T_DATE (truth_date rows 8, 9, years -100 / -4712, ADR-28), T_TS (truth_ts row 5 TS9, outside the nanosecond range), T_NUM (truth_num row 19, 34 integer digits > 28 of Decimal128(38,10)), T_INTERVAL2 (truth_misc2 IYM rows 2, 3, beyond 2^31 months): each code from a truth line and an ADR, counts from the catalog (13 + 13 + 0). Weak spot: `:77` `table.num_rows == t.num_rows or policy == "null"` skips the row count under `null` and compares with the reader's own catalog (N-4) |
| `test_handover.py` | the core digest; T_PART sizes 1095 / 1098 / 807 (truth_part) | three paths compared; the capsule test asserts `rows_read == 0` (documents S-1 rather than catching it) |
| `test_errors.py` | offsets of the Rust tests and `exp_parse.py` (0x230f, 0x8354, 0x223a, 0x2790), bytes asserted before patching | `:259-266`: `e.column` compared with itself inside the tuple, then only `is not None` (N-4) |
| `test_warnings.py` | truth_num / truth_num3 (counts 6 / 30 digits, 4 / 1 / 115, 10 nulled), truth_date | sound |
| `test_survey.py` | the plan's T-12 numbers and the Analyst's tools | `--no-names`: names shorter than 3 and non-text values are not searched (accepted by design) |
| `test_interval.py` | truth_misc / truth_misc2 texts (rows 1 to 4 literal, equal to the truth lines) | the skip `:47-48` (`pa.__version__ != "26.0.0"`) is honest (it names the observed version) but narrow (26.0.1 skips too), and inconsistent: `test_consumers.py::test_parquet_accepts_duration_but_not_interval_month_day_nano` asserts the same refusal for **every** pyarrow (N-5) |
| `test_consumers.py` | observations, pyarrow 26.0.0 / pandas 3.0.6 | whole module skipped without pandas (a wheel test without pandas silently runs 8 fewer tests) |
| `test_threads.py` | timing, threshold 1.6 (measured 1.0 to 1.1; h gives 2.0) | a timing test; passed here |
| `test_version.py` | metadata and `Cargo.toml` via `check_dist.pep440` | p5b (`__version__ = "0.1.0"`) fails it; p6 (root `Cargo.toml` `0.1.0-alpha.2`, not rebuilt) fails it; p5 (`__version__ = "0.1.0a1"`, the correct literal) passes, as it must |
| `test_project_name.py` | `git ls-files` of the Implementer's paths | skips outside a git clone (N-5) |

No `pytest.mark.skip`, no `xfail`, no tolerance except the thread ratio.

## 4. T-13: `interval_as`

- `reader.rs:303-315`: `Kind::IntervalYmText` / `IntervalDsText`; the decoders (and their opposite-sign check) are
  the native ones; p / f from [T9] or the fallback (digits needed, f = 9).
- Two values re-derived from the fixture bytes: `18c/misc_exp_18c.dmp` T_INTERVAL row 1 IDS @0x22b0
  `80 00 00 0a 48 5a 69 87 5b ca 00`: days 0x8000000a - 2^31 = 10, hours 0x48 - 60 = 12, minutes 0x5a - 60 = 30,
  seconds 0x69 - 60 = 45, nanoseconds 0x875bca00 - 2^31 = 123,456,000; p = 4, f = 6 ->
  `+0010 12:30:45.123456` = `18c/truth_misc.txt` line 1. `11g/misc2_exp_11g.dmp` T_INTERVAL2 row 3 IDS @0x2338
  `44 65 36 01 25 01 01 44 65 36 01`: days 0x44653601 - 2^31 = -999,999,999, hours 37 - 60 = -23, minutes and
  seconds -59, nanoseconds -999,999,999 -> `-999999999 23:59:59.999999999` = `11g/truth_misc2.txt` line 3; and row 2
  IYM @0x230a `bb 9a c9 ff 47` = `+999999999-11`. Agree.
- Opposite signs: rule Y10 (`exp_reader_rules.md:60`, `:282`) confirms that every field of a negative interval is
  negative; that a mixed value is an error is the decision of arch 7.1 (session 4) and ADR-47, not a rule. Consistent.
- Mutations: i_sign (DAY TO SECOND sign from the days only) fails `t13_interval::string_reads_equal_the_to_char_texts_of_the_truth_files`
  and `t4_values::e_intervals_of_truth_misc2` (the `-00 00:00:01` value); i_frac (one fraction digit fewer) fails
  4 tests (`string_reads_...`, `a_column_without_a_usable_declaration_...`, `t4_values::e_intervals_of_truth_misc`, `..._misc2`);
  YM opposite-sign check removed: fails `t13_interval::opposite_signs_stay_e_bad_interval_under_string` and
  `t4_values::e_interval_errors`; DS check removed: fails `t4_values::e_interval_errors` only (no `string`-mode
  test for DAY TO SECOND opposite signs; the native test covers the shared decoder).
- `t13_interval.rs:45`: "the same three rows" while the test asserts four (cosmetic).

## 5. Packaging and licence

- Built in the clone with the commands of `CONTRIBUTING.md` (activated venv): `oradmp-0.1.0a1-cp311-abi3-manylinux_2_28_x86_64.whl`
  (3,611,216 bytes, sha256 `fedad388...b360`, differs from the Implementer's because the path and the SBOM time
  differ) and `oradmp-0.1.0a1.tar.gz` (sha256 `5788717c...ff48`, **identical** to the T-13g note: the sdist is
  reproducible). `check_dist: ok`. Wheel members: `oradmp/{__init__,__main__,_api,_errors,_survey}.py`,
  `_core.abi3.so`, `_core.pyi`, `py.typed`, `.dist-info/{METADATA,WHEEL,RECORD,licenses/LICENSE-APACHE,licenses/LICENSE-MIT,sboms/oradmp-py.cyclonedx.json}`.
  `METADATA`: `Name: oradmp`, `Version: 0.1.0a1`, `Metadata-Version: 2.4`, `License-Expression: MIT OR Apache-2.0`,
  two `License-File`, `Requires-Python: >=3.11`, `Requires-Dist: pyarrow>=15`, README as description.
- **The leak (PROBLEM 1), precisely.** The owner's checkout is now `<home>/p/oradmp`, so the old-name part of
  PROBLEM 1 no longer occurs in it; what remains is the home path:
  1. `oradmp-0.1.0a1.dist-info/sboms/oradmp-py.cyclonedx.json`: 6 strings
     `path+file://<absolute checkout>/crates/oradmp-core` / `.../crates/oradmp-py` (the `bom-ref` of the root
     component, its bin target and the dependency entries), here `path+file://<home>/.cache/review-03/repo/...`,
     in the owner's build `path+file://<home>/p/oradmp/...`: user name `max` and folder layout. Also a
     `timestamp` (build time), so the wheel is not reproducible.
  2. `oradmp/_core.abi3.so`: **161** strings `<home>/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/<crate>-<version>/src/...`
     (panic locations of the dependencies: pyo3, arrow-*, ...). The workspace crates appear relative
     (`crates/oradmp-core/src/...`). The T-13g note's "The `.so` is clean" holds for the old name only.
  3. Nothing else: no path or user name in `METADATA`, `RECORD`, the Python files, or anywhere in the sdist
     (`grep -rlai /home/` over both unpacked archives); `LICENSE-MIT` carries the owner's full name by design.
  A release build with `RUSTFLAGS="--remap-path-prefix=$HOME/.cargo/registry/src=/cargo-registry --remap-path-prefix=<checkout>=/build"`
  (tried here) gives a `.so` with **0** `<home>` strings; the SBOM keeps its 6. Whether maturin 1.15.0 can omit or
  relativise the SBOM I did not check. Decision for the Architect / owner: S-2.
- `LICENSE-APACHE`: 202 lines, sha256 `cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30`, the value
  the T-13 note recorded for the apache.org download (I did not fetch it again). `LICENSE-MIT`: the SPDX MIT text
  with `Copyright (c) 2026 Maksim Tkachenko`. `license = "MIT OR Apache-2.0"` in the root `Cargo.toml`, `license.workspace = true`
  in both crates, `license` and `license-files` in `pyproject.toml`; both files in the wheel are byte-identical to
  the repository's.
- `cargo deny --offline check licenses`: `licenses ok` (warning: `Zlib` allowance unmatched); `check bans`: `bans ok`.
  `cargo tree -i tiny-keccak --offline --target T` prints nothing for each of the five targets of `deny.toml`;
  `--target all` shows `tiny-keccak <- const-random-macro <- const-random <- ahash <- arrow-array`. ADR-49 holds.
- `check_dist.py` mutations (archive copies): corpus / tests / docs member added to the sdist -> FAIL each; old
  name in `oradmp/_survey.py` -> FAIL with offset and line; wheel renamed to `0.1.0a2` -> FAIL (version against
  `Cargo.toml`, and the `.dist-info` mismatch); `METADATA` `Version` changed -> FAIL; `oradmp/exp_survey.py` added ->
  FAIL. **Not caught**: a `oradmp/__pycache__/_api.cpython-313.pyc` (with a neutral path and with
  `<home>/p/oradmp/...` inside), and `python/oradmp/corpus/x.txt` in the sdist (only the top folder is tested,
  `check_dist.py:39`) (S-3). It scans the SBOM, but only for the old name.
- README: the claims I could check hold (types, `interval_as`, limits, pandas / Parquet facts are the asserted
  ones; the example ran against `18c/t_num_exp_18c.dmp` in the fresh venv). It says "not on PyPI yet", shows no
  `pip install oradmp`, has the ADR-54 line ("oradmp (not oradump, ...)") right after the title, and "not affiliated
  with, endorsed by or sponsored by Oracle Corporation". ADR-50's verbatim sentence "`pip install oradump` does not
  install this project" was dropped in T-13g (necessary under ADR-54 item 2, but not mentioned in the note) (N-6).
- CONTRIBUTING: the checks of the definition of done run as written (fmt, clippy, nextest, deny, ruff, mypy,
  pytest: done above); the release block works in an activated venv. "Development setup" creates `.venv` with
  `python3 -m venv` and `.venv/bin/pip`, while the release block says ".venv has no pip" (the repository's `.venv`
  now has pip 25.1.1) (N-6). The `pip install` commands needing the network were not run as written.

## 6. The rename

- `grep -rIli oradump` (without `target/`, `.venv/`, `.git/`): `CLAUDE.md` (rule 1, owner), four `corpus/` files
  (owner), `docs/briefs` (2), `docs/design` (5), `docs/prompts` (5 finished + this session's prompt), `docs/research` (2),
  `docs/review` (2), `README.md` (the one line). All in ADR-54's kept or owner groups; `find -iname '*oradump*'`:
  only `./oradump.code-workspace`. Live Architect text that still names old paths: the plan's Reviewer 3 reading
  list (`implementation-plan.md:1379`, `:1405-1408`) and the Q-5 text in `open-questions.md:171-228` (N-8).
- Binary (`grep -a`): the owner's untracked `python/oradmp/_core.abi3.so` and `__pycache__/*.pyc`, and
  `tests/__pycache__/*.pyc`, contain the old name (built before the folder was renamed); harmless, not shipped
  from a clone, but stale (N-8).
- Wheel and sdist built here: no old name anywhere (`check_dist` scan, member names and contents, `.so` and SBOM
  included).
- Fresh venv (`uv venv`, wheel + pyarrow only): `import oradmp` -> `0.1.0a1`; `import oradump` ->
  `ModuleNotFoundError: No module named 'oradump'`; `python -m oradmp survey --no-names` prints `oradmp survey`.
- `git show -M --name-status 3a1bb5f`: every move is an `R` entry (31 `R100`, the rest renames with path edits);
  the four `tests/data/*.tsv` are `R100`; `tests/golden/` untouched. Both generators re-run in the clone from the
  new paths: outputs byte-identical (`git status` clean).

## 7. Who wrote where

`a19d873`, `e2eaae6`, `3fe33e7`, `3a1bb5f` touch only `crates/`, `python/oradmp|oradump/`, `tests/`, the root
files of the Implementer's row (`Cargo.toml`, `Cargo.lock`, `pyproject.toml`, `deny.toml`, `README.md`,
`CONTRIBUTING.md`, `LICENSE-*`, `.gitattributes`) and `docs/design/implementer-notes.md` (additions only, its own
sections). Nothing under `corpus/`, `docs/formats/`, `docs/review/`, `.claude/`, `CLAUDE.md` or `docs/prompts/`.
**No finding.** Outside the repository: `~/.cache/oradump-t12` (3.9 GB, with a `repo` and `target`) is still there,
although the T-12 note says the scratch copy was removed (N-8).

## 8. Test files not read line by line before

Read: `t3_catalog.rs` (the log test in full, the rest by test function), `t5_types.rs` (first three tests in
full), `t8_direct.rs` (pair test in full), `t2_manifest.rs` (manifest and truth_counts tests), `t1_walk.rs` (plan
numbers, i2), `t5f_followup.rs` (TSLTZ tests); the others (`t3a_walker`, `t3b_parts`, `t3c_walker`, `t6_read`,
`t6l_ru`, `t6_value_parts`) by test function, expected-value sources and a grep for loose assertions.

| file | expected values from | findings |
|---|---|---|
| `t3a_walker.rs` (19 tests) | `walk_parts.tsv`, `walk_expected.tsv`, `lob_expected.tsv` (generators), `exp.md` offsets, truth_lob / truth_part / truth_ru | none (the `unwrap_or_default` at `:185`, `:193` is on an optional partition name, as review 02 said) |
| `t5_types.rs` (24) | `collist.py` per-form counts (I re-ran the tool: the 28 forms and counts equal the literal table, sum 1,907), truth_decl, truth_probe11 | `list.columns.len() == t.columns.len()` compares two reader layers, but totals come from the tool: fine |
| `t3_catalog.rs` (15) | the 201 `.log` files, `exp_raw.tsv`, `exp_catalog.tsv`, truth files | `:960` `compared >= 300` and `:963` `failed_tables > 0` are loose (completeness is held by `remaining.is_empty()`; exact values would be cheap) (N-9) |
| `t3c_walker.rs` (11) | `exp.md` 6.6 offsets, truth_part2 | none |
| `t6_read.rs` (16) | truth files (stat, num3, misc, misc2, date, ts, tstz, long2), `11_types.sql`, `exp_raw.tsv` | none found |
| `t1_walk.rs` (16) | tool TSVs, `corpus_counts.py`, `intro_list.py`, rules block | `:808` `length > 200` (loose) |
| `t6l_ru.rs` (7) | truth_ru, truth_lob_ru, `15_ru_lob2.sql`, `07_ru_objects.sql` | none |
| `t3b_parts.rs` (10) | `exp_raw.tsv`, `lob_expected.tsv` (since T-11b), `exp.md` | none new |
| `t8_direct.rs` (8) | pair equality (reader against reader, transitive to the truth checks of the conventional files), truth_part / part2 | `:164-165` `tables >= 80`, `rows_total > 10_000` loose (N-9) |
| `t5f_followup.rs` (8) | truth_num3, truth_stat | `:240-273` the TSLTZ expectation is built from the reader's own catalog (columns with type 231) and ends with `columns > 0`; the catalog gives 4 sections with TSLTZ (collist: 4 `... WITH LOCAL TIME ZONE`), which could be asserted (N-9) |
| `t2_manifest.rs` (6) | `SHA256SUMS`, `exp_raw.tsv`, truth_counts | `:336-337` `>= 10`, `>= 6`, `:441` `compared >= 30` loose (N-9) |
| `t6_value_parts.rs` (1) | `value_parts.tsv` (generator) | none |

No `#[ignore]` in any file; no `unwrap_or` on a value under test found; no count compared with itself except
`test_errors.py:264` (section 3).

## 9. The Analyst's tools

- `num_decode.decode`: `d1 0b` = 10^33 (exponent 0xd1 - 193 = 16, digit 0x0b - 1 = 10; truth_num row 19);
  `2c 59 43 2d 17 0b ... 66` = -12345678901234567890123456789012345678 (exponent 62 - 0x2c = 18, digits 101 - b;
  row 22); `c1 02` = 1; `80` = 0; `3e 5b 66` = -10. All by hand, all agree.
- `collist.py`: 386 lines, 1,907 definitions, 6 moved sections, 0 problems; the 4 TSLTZ definitions equal the 4
  T_TSTZ sections of `exp_catalog.tsv`.
- `exp_parse.parse` on `18c/misc_exp_18c.dmp` T_INTERVAL: `rows_off` 0x22a3, row 1 fields at 0x22a3 (`c1 02`),
  0x22a7 (`80 00 00 02 42`), 0x22ae (11 bytes), end marker `ff ff` @0x22fb: equal to my hex dump.
- Not checked: `exp_walk.py`, `lob_check.py`, `check_truth.py`, `check_all.py`, `boundary.py`, `misc_types.py` and
  the other 20 tools (only run, not checked, were `corpus_counts.py`, `rowlen_stats.py`).

## 10. Numbers

| quantity | tools now | `common/mod.rs` `TOTALS` | plan / notes | rules |
|---|---|---|---|---|
| files / sections / parts | 201 / 386 / 400 (`corpus_counts`, `rowlen_stats`) | 201 / 386 / 400 | same | 201 / 386 |
| rows / fields / NULL | 41,212 / 243,007 / 19,900 | same | same | same |
| LOB values | 126 (80 + 46, 12 empty, 21 sections) | 126 | 126 | 126 |
| NUMBER fields | 86,080 (`num_decode`, PROBLEMS 0) | 86,080 | 86,080 | - |
| column definitions | 1,907 (`collist`) | 1,907 | 1,907 | **T9 (`exp_reader_rules.md:226`): "382 lines, 1885 column definitions" - not corrected** |
| Rust tests | 395 run (396 `#[test]` by grep; one is the string at `t10_truth.rs:1525`) | - | 395 (T-13, T-13f, T-13g) | - |
| pytest | 73 passed, 0 skipped (Python 3.13.5, pyarrow 26.0.0, pandas 3.0.6; the interval skip did not happen) | - | 73 (T-13g note) | - |

Generators re-run (`gen_walk_expected.py`: 201 / 386 / 400 / 41,212 / 243,007 / 19,900, LOB 126; `gen_number_parts.py`:
201 files, 400 parts, 86,080 NUMBER fields): outputs byte-identical. The only disagreement is T9.

## 11. Deviations: decided or only noted

| note | deviation | decided by |
|---|---|---|
| T-11b items 1 to 13 | checker macros, break test on reader cells, `ARCH_7_1_STRICT`, counts, N-2 to N-9 fixes, N-1 left | Architect session 11 answers; item 3 ADR-41 item 9 |
| T-12 D1 to D10 | arrow 59 / pyo3 0.30.1 / pyo3-arrow 0.20.0; lints; `Error.warning`; S-3 as built; D5 thresholds; names after a failed scan; survey core; digest; packaging files; warnings | ADR-3 (session 11), ADR-41 item 8, ADR-46 items 1 to 9 |
| T-13 P1 to P9 | cargo-deny CC0, `UsageError`, `TypeOptions.interval_as`, fallback text, sdist without tests, SBOM, version, ruff on README, disk | ADR-49, ADR-50, ADR-51 |
| T-13f problems 1, 2 | `__version__`, zig only with `.venv/bin` on `PATH` | ADR-54 items 7, 8 |
| T-13g PROBLEM 1 | SBOM path (and home path) | **only noted** (S-2 here) |
| T-13g Note 2 | `.pyc` in a wheel built outside a git work tree | **only noted** (S-3 here) |
| T-13g | `str(...)` around `get_payload()` in `check_dist.py`; mypy now checks `check_dist.py` | **only noted** (harmless) |
| T-13g (not in the note) | ADR-50's README sentence with the old name dropped | **not noted** (N-6) |

## Findings

### Blocking

None.

### Should fix

**S-1 Warnings are lost on the Arrow C stream path** (`crates/oradmp-py/src/lib.rs:358-378`, `python/oradmp/_api.py:291`).
`arrow_c_stream` stashes rows and warnings before handing the reader to `PyRecordBatchReader`, and nothing issues
or records what the consumer's reads produce. Shown in the fresh venv: `oradmp.open("t_num_exp_18c.dmp",
on_value_error="null").read_table("T_NUM")` through `pa.RecordBatchReader.from_stream(r).read_all()` gives 33 rows,
`r.warnings == []`, `r.rows_read == 0`, no warning issued under `simplefilter("always")`; the iterator path on the
same table gives `W-VALUE-NULLED` (10 values) and `W-NUMBER-ROUNDED` (4 values, 1 to 0). With the default
`on_number_overflow="round"` the C stream path changes values silently. ADR-33 item 3 / arch 3.4 ("warnings are
part of the result") and the README (which advertises `from_stream`) do not say this path is different. No test
covers it (`test_the_arrow_c_stream_spends_the_reader` asserts `rows_read == 0`). Fix (Architect to choose): keep
a shared handle (`Arc<Mutex<..>>`) to the `TableBatches` counters so `BatchReader.warnings` / `rows_read` reflect
the consumer's reads and are issued on the next `BatchReader` call or at `close()`; or document the path as
"no warnings" and make `strict` / an explicit opt-in a requirement there; plus a test that reads T_NUM through
`from_stream` and asserts the two warnings.

**S-2 The release wheel holds the builder's home path and user name** (the T-13g note's PROBLEM 1, restated in
section 5): the SBOM's 6 `path+file://<home>/...` references, and 161 `<home>/.cargo/registry/src/...`
strings in `_core.abi3.so`. The sdist is clean and reproducible. Shown: `grep -rlai /home/` over both unpacked
archives; `strings -a _core.abi3.so | grep -c home/max` = 161 before and 0 after a build with
`--remap-path-prefix` for the registry and the checkout (SBOM unchanged at 6). Fix: the Architect decides (a)
`RUSTFLAGS` remap prefixes in the release command of `CONTRIBUTING.md` and the checklist, and (b) for the SBOM a
neutral build path (for example a clone in `/build/oradmp`) or removing / rewriting the SBOM if maturin allows it
(not verified); and `check_dist.py` should then fail on `/home/`, `path+file://` and the builder's user name in any
member.

**S-3 `check_dist.py` lets through a `.pyc`, a home path and a nested `corpus/`** (`tests/check_dist.py:29-43`).
Shown on archive copies: a wheel with `oradmp/__pycache__/_api.cpython-313.pyc` (neutral path, and one compiled
with `<home>/p/oradmp/...` as file name) prints `check_dist: ok`; so does an sdist with
`oradmp-0.1.0a1/python/oradmp/corpus/x.txt`. The T-13g note's Note 2 shows the `.pyc` case happens for real (a
build from a plain copy) and is caught today only if the path holds the old name. Fix: reject `__pycache__/` and
`*.pyc` / `*.pyo` members; test every path component, not only the first, against `corpus` / `tests` / `docs`;
add the S-2 path checks.

**S-4 The message check cannot see the value that failed, and misses a hex prefix** (`t12_messages.rs:85-90`,
`:197-222`). (d) collects cells from a lenient read, where the failing value is NULL, so the decimal text of a
NUMBER that overflows is in no corpus; raw bytes (non-ASCII), lossy grams and hex do not match its text. d4b
(the failing value's `to_text()` appended to `E-NUMBER-OVERFLOW`) failed only because the text happens to contain
`100000`, the cell of T_NUM row 10 (probe: the cells of `t_num_exp_18c` matching the message are exactly
`100000`); the same message with `1E+33` passes the check, and so would any overflowing value without such a
substring. d6 (hex of the first 8 bytes of a longer field): **395 passed**; probe on the patched copy: `bytes
4e4e4e4eff4e4e4e` gives no hit. ADR-46 item 3 makes d6 a should-fix with its rule (a hex run of 8 or more digits
that is a substring of a field's hex is a hit). Fix: that rule; and add to each table's corpus the reader's text of
every decodable raw field (for NUMBER `OraNumber::to_text()`, for DATE / TIMESTAMP / INTERVAL the decoder's text)
whether or not the column type can hold it; a self-test with T_NUM row 19's `1000000000000000000000000000000000`
replaced by `2E+33`-style and plain forms.

**S-5 Architecture 7.3 promises a panic -> `OradmpError` conversion that does not exist**
(`exp-reader-architecture.md:1109`; `crates/oradmp-py/src/*.rs` has no `catch_unwind`). A panic in the core reaches
Python as `pyo3_runtime.PanicException`, a `BaseException`: `except oradmp.OradmpError` and `except Exception` do
not catch it. Under the workspace lints no panic is expected, and T-14 fuzzes; so the risk is small, but the text is
false. Fix: either wrap the `py.detach` closures in `std::panic::catch_unwind` and map to an `OradmpError` (code to be
decided), or correct 7.3 and the README.

### Notes

- N-1 (from review 02, still open): deleting the single `assert!` of `every_table_of_the_exp_files_is_classified_and_read`
  is caught only where a break test reads the same file (n1e4, n1w18 in section 1); small.
- N-2 GIL and warnings placement: `PyDump.info()`, `warnings()` and `scan_inner`'s second lock (`lib.rs:63-70`,
  `:76-83`, `:96-106`) lock the dump's mutex while holding the GIL (no deadlock; the interpreter stalls while another
  thread reads the same `Dump`); `stacklevel=4` (`_api.py:231`) points into `_api.py` for `read_all()`.
- N-3 `oradmp:lob_charset` (arch 5.1, ADR-54 item 1) is not written; LOB columns carry `oradmp:charset` instead
  (`reader.rs:1145-1147`). Decide which text is right.
- N-4 `test_truth.py:77` (`or policy == "null"` skips the row count; and the count is the reader's own catalog,
  the golden catalog has the truth); `test_errors.py:259-266` (`e.column` compared with itself);
  `t12_offsets.rs:155` (`let _ = place(&e);`, a helper whose result is never asserted).
- N-5 Skips: `test_interval.py:47-48` skips for any pyarrow other than exactly 26.0.0 while
  `test_consumers.py` asserts the same refusal unconditionally (make both the same: assert for the versions
  observed, 15.0.2 and 26.0.0, skip otherwise, or assert always); `test_project_name.py:33` skips outside a git clone
  and `test_consumers.py:18` without pandas, so a run against an installed wheel can silently run fewer tests (print
  the skips in the release checklist step).
- N-6 README / CONTRIBUTING: ADR-50's verbatim Install sentence changed in T-13g without a note (needed by ADR-54
  item 2; the Architect should record the new text); "Errors are `oradmp.OradmpError` subclasses" omits `OSError` and
  panics; CONTRIBUTING's dev setup (`python3 -m venv`, `.venv/bin/pip`) contradicts the release block's ".venv has no
  pip"; `.gitattributes` last line lost the column alignment of ADR-52's text (cosmetic).
- N-7 Binding error classes: `E-IO` as `OSError` is accepted (ADR-46 item 8) and carries `.code` too (`make_error`);
  only the README sentence (N-6) is incomplete.
- N-8 Stale old-name artefacts: the plan's Reviewer 3 list (`implementation-plan.md:1379`, `:1405-1408`) and
  `open-questions.md:171-228` (Architect); the owner's untracked `python/oradmp/_core.abi3.so` and `__pycache__`
  (rebuild before a release); `~/.cache/oradump-t12` (3.9 GB) left by T-12.
- N-9 Loose counts in older tests: `t3_catalog.rs:960`, `:963`; `t8_direct.rs:164-165`; `t2_manifest.rs:336-337`,
  `:441`; `t1_walk.rs:808`; `t5f_followup.rs:240-273` (expectation from the reader's catalog, `columns > 0`; 4 is
  exact by `collist.py`). Each could be an exact value from a tool or the catalog.
- N-10 Rule T9 (`exp_reader_rules.md:226`) still reads 382 / 1885 (Analyst; ADR-38 item 7, ADR-44 item 7).
- N-11 The SBOM's `timestamp` makes every wheel build different (only the sdist is reproducible); relevant if the
  release checklist compares hashes.

## Tests that did not fail when mutated

- d6 (hex prefix of a long field): none failed (S-4).
- s3c (`.at(st.start)` removed in `lob_piece`): none failed; equivalent mutant (section 1), not a gap.
- p5 (`__version__` = the correct literal `"0.1.0a1"`): none failed; cannot be caught, expected.
- `check_dist.py`: `.pyc` added (two variants) and nested `corpus/` in the sdist: `check_dist: ok` (S-3).
- Not a mutation: the C stream path loses the warnings with every test green (S-1).
- d4b failed only by coincidence (S-4).

## What I did not check

- Line-by-line reading of `t3a_walker.rs`, `t3b_parts.rs`, `t3c_walker.rs`, `t6_read.rs`, `t6l_ru.rs`,
  `t6_value_parts.rs` and most of `t3_catalog.rs`, `t5_types.rs`, `t1_walk.rs`, `t2_manifest.rs`, `t5f_followup.rs`:
  read by test function, expected-value sources and a grep for loose patterns only (section 8).
- T-12 breaks S-3 "old reader.rs", b2, c, g: not re-run. `damaged_*_never_panic` not mutated.
- Memory release of the Arrow hand-over (no leak or RSS measurement); whether pyarrow holds the GIL while pulling
  from the capsule; behaviour with pyarrow 15 / pandas 2.2 and with Python 3.11 / 3.14 (only 3.13.5 + pyarrow 26.0.0
  here); `auditwheel` / an actual glibc 2.28 host; `pip install` of the sdist (source build) and the CONTRIBUTING
  commands that need the network.
- `LICENSE-APACHE` compared by SHA-256 with the value recorded in the T-13 note, not fetched again from apache.org.
- Whether maturin 1.15.0 can omit or relativise the SBOM; `--remap-path-prefix` was tried only for the strings count,
  not for reproducibility or debug quality.
- `cargo deny check advisories` / `sources` (need the network).
- The Analyst's tools other than `num_decode.py`, `collist.py`, `exp_parse.py` (section 9); the generators
  `gen_exp_raw.py`, `gen_exp_catalog.py` were not re-run (unchanged since review 02 except moved paths, which do not
  apply to `tests/golden/`).
- The survey's full output and its `--no-read` mode; the `names.rs` file-position mapping beyond reading.
- Data Pump files, `corpus/work/` (not read, by rule); nothing named `oradump` was read, fetched, searched or
  installed.

## git status

Before (start of the session): `?? docs/prompts/review-03-prompt.md`. After: `?? docs/prompts/review-03-prompt.md`
and `?? docs/review/review-03-python-packaging.md` (this file); nothing else. Every experiment ran in
`<home>/.cache/review-03/` (clone, target, venvs, archives); nothing under `crates/`, `python/`, `corpus/`,
`tests/` or `docs/` other than this file was written.
