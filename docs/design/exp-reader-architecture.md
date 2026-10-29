# Legacy EXP reader: architecture (phase 1)

Architect session 1, 2026-10-07; revised in session 2, 2026-10-14, sessions 3 and 4, 2026-10-15,
sessions 5 and 6, 2026-10-16, sessions 7 to 13, 2026-10-17, sessions 14 and 15, 2026-10-18, session 16, 2026-10-21, session 18, 2026-10-22, and sessions 19 to 22, 2026-10-24, and sessions 24 and 25, 2026-10-25, and session 26, 2026-10-29. Status: sessions 1 to 6 accepted; the
session 7 and 8 changes below are a proposal for review. Rules: `exp_reader_rules.md` v3.5 from session 8, v3.7 from session 24, v3.9 from session 25. Companion
files: `decisions.md` (ADR-n), `implementation-plan.md` (T-n), `open-questions.md` (Q-n, R-024 and
later, risks), `implementer-notes.md` (N-n with the Architect's answers).

## Changes in session 26

Basis: the cut (ADR-105). No change to the reading design.

- 9: the public repository's layout: `HISTORY.md` (owner's), the private paths, the archive, the remote, the release
  rule (paragraph "Session 26" at the end of section 9); the `tools/` line no longer sends the reader to `CLAUDE.md`.

## Changes in session 25

Basis: Analyst 19 and 20 (R-048 to R-051; rules v3.9), ADR-99 to ADR-104. One change to the reading design: the three
conventional LONG / LONG RAW forms (ADR-99); task T-19b.

- 1.4: the session 24 rows are replaced: a conventional LONG or LONG RAW field starting `fb ff` / `fc ff` is
  `UnsupportedLayout`; inside the `fa ff` form an n outside 0xfffa to 0xfffd or non-zero upper bytes are `Framing`; a
  chunk u16 from 0xfff0 stays `Framing`.
- 1.5: the non-goal reads "a conventional LONG / LONG RAW field starting `fb ff` or `fc ff`".
- 2.5 step 5, 5.1 (types 8 and 24), 5.5 ("LONG and LONG RAW"): one field for 1 to 0xfff9 (0xfff2 to 0xfff4 with
  `W-UNTESTED-LAYOUT`), `fa ff` + n + `00 00` for 65,530 to 65,533, chunks from 65,534; LONG RAW as LONG; `W-LONG-LENGTH`
  above 100,000 for both.
- 5.4: a 7-byte TSLTZ value is confirmed [Y8] v3.9: no `W-UNVERIFIED-TYPE` (ADR-100).
- 7.x: the session 24 messages are replaced by those of ADR-99 item 3.

## Changes in session 24

Basis: Analyst 17 (R-017, R-040, R-047; rules v3.7), ADR-94 to ADR-98. One change to the reading design: the conventional
LONG field (ADR-95); task T-19.

- 1.4: the conventional LONG / LONG RAW rows: LONG reads one field up to 0xfff0 and the `fd ff` chunked form; the stop
  remains for a LONG u16 0xfff1 to 0xfffc and a LONG RAW u16 from 0x8000 (R-048); a chunk u16 from 0xfff0 is `Framing`.
- 1.5: the non-goal "conventional LONG of 32,768 bytes or more" narrowed to the R-048 cases.
- 2.5 step 5, 5.1 (types 8 and 24), 5.5 ("LONG and LONG RAW", "Lengths that never occur"): as 1.4; `W-LONG-LENGTH` from
  100,000 bytes for LONG; `W-UNTESTED-LAYOUT` for a chunked value below 65,535 bytes; `LongStats.chunked`.
- 5.4 (empty tables): [E2] / [E3] unchanged; the cause (no segment) is confirmed on 18c (R-017).
- 7.x: no new code; `E-UNSUPPORTED-LAYOUT` gains the two R-048 messages.

## Changes in session 22

Basis: Reviewer 6b (ADR-85, ADR-86). Nothing in the reading design changes.

- 3.5: every path the tool writes into a message (the dump, the `export -o` target, the file of an output-side
  `OSError`) is written with `\xHH` for each byte that is not UTF-8 (ADR-82 item 3 as amended; T-17b).
- 8.3: the synthetic generator writes fixture-shown combinations, plus two labelled `Unconfirmed` ones (ADR-80 as
  amended); its files are coverage, not evidence.

## Changes in session 21

Basis: Reviewer 6a-closure and the a3 release (ADR-81, ADR-82). Nothing in the reading design changes.

- 3.4: `oradmp.open` accepts any file name the operating system gives, also one that is not valid UTF-8 (the binding
  takes a path; T-18d, in a4).
- 3.5: such a name is written with `\xHH` for each byte that is not UTF-8 in the tool's messages and in `survey`; the
  manifest header that `export --all` prints before an exit-3 error stays (the exit code is the signal).

## Changes in session 20

Basis: Reviewer session 6a (ADR-77, ADR-78). Nothing in the reading design changes.

- 3.5: the tool's line for an I/O error of the dump names the OS reason and the file (`oradmp: error: [E-IO] No such
  file or directory: FILE`, exit 3; T-18c); the export format from the suffix is case-insensitive.
- 4.3: the benchmark baseline is baseline 2 of `benchmarks.md` (code of `460aef5`), with its regression rule.

## Changes in session 19

Basis: the T-15b and T-18 notes (ADR-75). Nothing in the reading design changes.

- 3.5: Parquet stores DATE as milliseconds; `head` shows no counted warning; a missing table in a stopped scan exits with
  the stop error's code (T-18b); the synopsis order; `--version` and `-q`.

## Changes in session 18

Basis: Reviewer session 5 (`dc7668e`, no blocking finding) and release run B (0.1.0a2 on PyPI). Rules v3.6; nothing
in the reading design changes.

- New 3.5: the command-line tool `oradmp` (T-18, ADR-72): commands, output, exit codes, what it does not do.
- 7.3: the panic guard's exempt list is complete (ADR-59 amendment, session 18).
- 9: `python/oradmp/_cli.py` and the console script.

## Changes in session 16

Basis: Reviewer session 4 (`b3a5a7f`, no blocking finding). Rules unchanged (v3.5).

- 4.3: the baseline and the regression rule live in `benchmarks.md` (ADR-66); the memory test compares 12,288 with
  100,000 rows (about eight times), limit 1.10 (ADR-65 item 6).
- 7.3: the guard covers every call that reads or decodes the file or parses user input; the exempt calls are named
  (ADR-59, amended).
- 8.3: the allocation bound is "no single allocation sized from a file field beyond 64 MiB" (libFuzzer); fuzz runs use
  `tools/fuzz_run.sh` (ADR-64 item 3). 8.6: the nightly job of T-16 (ADR-68).

## Changes in session 15

Basis: the T-13h note (`39cbb93`; 402 Rust tests, 105 pytest). Rules unchanged (v3.5).

- 3.4: on the Arrow C stream path the error text **contains** the code and offset after arrow-rs's `External error: `
  (pyarrow `ArrowInvalid`); the code is found by a pattern, not a prefix; no message holds U+0000; after `close()` the
  iterator too is `E-CONFIG` (ADR-61).
- 8.3: (b) moves to T-17; the three fuzz targets, their limits and assertions (ADR-62).
- 9: `fuzz/` has its own lock file and a dated nightly pin (ADR-62).

## Changes in session 14

Basis: Reviewer session 3 (`docs/review/review-03-python-packaging.md`; 395 Rust tests, 73 pytest at `3a1bb5f`).
Rules unchanged (v3.5).

- 3.4: the Arrow C stream path shares the reader's state: `rows_read` and `warnings` follow the consumer's reads,
  `close()` issues the warnings not yet issued, iteration after the export is `E-CONFIG` (ADR-55).
- 5.1: no `oradmp:lob_charset` key; LOB text columns carry `oradmp:charset` (ADR-60, N-3).
- 7.1, 7.3: new code `E-INTERNAL` (`InternalError`): the binding catches a panic of the core (ADR-59); 7.3 said so
  before, now it is specified.
- 9: `tools/release_build.sh` (ADR-56); `tools/` and `fuzz/` need a row in `CLAUDE.md` (O-14).

## Changes in session 13

- The project is renamed **`oradmp`** (ADR-54): in sections 1 to 9 every `oradump` is now `oradmp` (39 places):
  crates `oradmp-core` / `oradmp-py` (2.2, 9), `oradmp_core` (3.2), `import oradmp` and `oradmp.open` (3.4),
  `OradmpError` / `OradmpWarning` (3.4, 7), the field-metadata keys `oradmp:...` (5), `python -m oradmp survey` (8.5),
  the layout of 9 (`crates/oradmp-core/`, `python/oradmp/`, `module-name = "oradmp._core"`). Meaning unchanged.
  The "Changes in session n" lists below keep the old name (history). Until T-13g is done the code still has the old
  names.

## Changes in session 12

Basis: the T-13 note (395 Rust tests, 69 pytest at `e2eaae6`). Rules unchanged (v3.5).

- 3.4: an invalid option value is `UsageError` with `E-CONFIG` (no `InvalidOption` class); on the Rust side
  `interval_as` is `Options.types.interval_as` (ADR-51 items 1, 2).
- 9: version `0.1.0-alpha.1` (Cargo) / `0.1.0a1` (Python) from one source; `deny.toml` checks the five build targets;
  `.gitattributes` keeps Git off the test data; more platforms are T-16 (ADR-49, ADR-50, ADR-52, ADR-53).

## Changes in session 11

Basis: the T-11b and T-12 notes (390 Rust tests, 58 pytest at `a19d873`). Rules unchanged (v3.5).

- 3.4: `open(..., interval_as="native")` (ADR-47); exception attributes `.table_ordinal`, `.type_code`, `.table`
  `None` after a failed scan (ADR-46 item 4); hand-over versions: arrow 59, pyo3 0.30, pyo3-arrow 0.20 (ADR-3).
- 5.7 item 5: `interval_as="string"` is in the alpha (ADR-47).
- 7.1: the outcome of every warning code under `strict`, as one table (ADR-41 item 9).
- 7.4: a LOB message always names the chunk, a LONG chain's only when it has more than one piece (ADR-46 item 2).
- 8.5: the survey's three passes and `--no-read` (ADR-46 item 9).
- 9: versions and the bump rule (ADR-3); wheel `cp311-abi3-manylinux_2_28_x86_64`, pinned maturin (ADR-48).

## Changes in session 10

Basis: Reviewer session 2 (`docs/review/review-02-t9b-t11.md`, 371 tests, no blocking finding); the owner's Q-6.
Rules unchanged (v3.5).

- 7.4: **one offset per value** under every policy (ADR-43 item 3, replacing ADR-41 item 6): value errors, the
  `W-VALUE-NULLED` / `W-NUMBER-NULLED` that replaces them under `null`, and value-level warnings carry the start
  of the value; a multi-piece value's message names the piece or chunk where the failure was detected.
- 3.4 "No value in any message": checked on the `message` field for raw bytes, text runs, hex and rendered cells
  (ADR-43 item 1, T-12).
- 9: licence `MIT OR Apache-2.0`, root files `LICENSE-MIT`, `LICENSE-APACHE`, `deny.toml`, `README.md`,
  `CONTRIBUTING.md`; the corpus is not in the wheel or the sdist (ADR-45).

## Changes in session 9

Basis: Reviewer session 1 (`docs/review/review-01-golden.md`), notes N-21 to N-25 (T-11 done, 347 tests).
Rules unchanged (v3.5).

- 3.4: `StrictError(OradumpError)` for `E-STRICT`, `UnsupportedCharset` for `E-UNSUPPORTED-CHARSET`, `.warning`
  on every exception; `message` becomes `Cow<'static, str>` in T-12 (ADR-40, ADR-41 item 1).
- 6.1: `W-CHARSET-MISMATCH` for form-1 CLOB columns is not built yet (bug from T-9; task T-9b, ADR-41 item 5).
- 7.1: the session 9 codes paragraph (`E-STRICT`, `E-UNSUPPORTED-CHARSET`; strict and `null`). 7.4: the offset of
  a value-level warning is the start of the value, also for a LONG chain (ADR-41 item 6).
- 8: the review gate after each group of tasks (ADR-42).

## Changes in session 8

Basis: rules v3.5 (R-038 answered: [L4, L5] in the CL8MSWIN1251 database, 11g). 255 tests on master after
T-9 and T-10, 11 of them failing on the corpus growth from 199 to 201 EXP files (task T-3d).

- 5.1 CLOB / NCLOB rows and 6.1 table: a CLOB or NCLOB of database set 171 decodes in both forms; the stop
  stays for every database set except 873 and 171 (ADR-25 session 8).
- 5.6 step 4: chunk lengths come from the stream; the sizes quoted are observations per version and
  database set (ADR-37). Step 6: the single-byte CLOB line. Step 7: the confirmed list
  `LOB_CONFIRMED_DB_SETS = [873, 171]`, compared with the header's database set, not with `db_charset`.
- 8.1: L rows cite `ru_lob2_*`; 8.2: 201 files, 386 sections, 400 parts after T-3d; `truth_lob_ru`,
  `truth_r045` join the truth-file list.

## Changes in session 7

Basis: notes N-19 (T-6L built), N-20 (T-5f built), the T-7 / T-8 tests (no production change). Rules
unchanged (v3.4). 232 tests.

- 2.3: `lob_start` takes the form of the value [L4] (ADR-34 item 1).
- 3.2: `ColumnInfo.type_source` covers `NUMBER(*,s)` (ADR-35 item 1) and means something for type 2 only;
  `max_lob_bytes` counts stored bytes; `TableBatches::lob_bytes_read()` / `lob_values_skipped()` are
  `#[doc(hidden)]` diagnostics, not API (ADR-34 item 2).
- 3.4: `E-LOB-UNCONFIRMED` is a `ValueDecodeError`; warnings are issued once per entry of the list, not
  once per code and file (ADR-35 item 4).
- 5.1: `NUMBER(*,s)` rows: `type_source`, the `NUMBER(*,-2)` example (ADR-35 item 1).
- 5.4: [E3] cites the T-8 test. 5.6 step 4 / 7: the order of the two LOB refusals; `W-LOB-UNTESTED`
  counted and `E-LOB-UNCONFIRMED` under `strict` (ADR-34 items 3 to 5).
- 6.1: `W-CHARSET-MISMATCH` also for form-1 CLOB columns (T-9, ADR-34 item 6).
- 7.1: the session 7 codes paragraph (LOB codes, `W-TSLTZ`, warning order). 8.1: rows cite the T-7 / T-8
  test files (ADR-36).

## Changes in session 6

Basis: notes N-17 (T-3b built) and N-18 (T-6 built). Rules unchanged (v3.4).

- 2.3: the `RowSink` push interface (T-6) is the implemented form of the seam; T-6L adds the LOB methods
  (ADR-33 item 1).
- 3.2: `Dump::rows` refuses a table of more than one part, `part_rows` reads one part (ADR-32 item 3);
  the option split between `Options` and `ReadOptions` as built (ADR-33 item 2); `ColumnInfo.type_source`
  (ADR-33 item 4); `Warning.count`, `count_to_zero`, `max_digits_dropped`; counted warnings in
  `TableBatches::warnings()` only (ADR-33 item 3).
- 3.4: `read_table()` takes the per-read options as overrides; when warnings are issued in Python.
- 4.1 (e): the builder behaviour is verified on arrow-array 60.0.0 (ADR-18).
- 5.2a guard (3): where `W-NUMBER-ROUNDED` appears; which columns the policy governs.

## Changes in session 5

Basis: notes N-12 to N-16 (T-3c, T-4, T-5 built), the owner's decisions of 2026-10-16 (Q-1 `round`, Q-7,
Q-3). Rules unchanged (v3.4).

- 3.2, 3.4: `on_number_overflow` defaults to `round` (owner, ADR-14); errors and warnings carry `part`,
  `row`, `column` by the convention of ADR-29; `PartInfo.file_order` (ADR-31).
- 5.1: a declared `NUMBER` that no Decimal128 holds takes `number_default` with `W-DDL-FALLBACK` (ADR-30
  item 3; `NUMBER(*,s)` with s < 0 included); DATE and INTERVAL YEAR rows point to ADR-28.
- 5.2a: decided (owner). 5.3: years below 1 stay `E-BAD-DATETIME` (ADR-28); dates before 1583 "as
  printed". 5.5 step 4: where an unknown type text ends (ADR-30 item 1); step 3: per part (ADR-31).
- 5.7 step 1: YEAR(9) values beyond 32-bit months stay `E-BAD-INTERVAL` (ADR-28).
- 7.1: `E-NUMBER-OVERFLOW`, `E-BAD-DATETIME`; `W-UNVERIFIED-TYPE` a warning code raised by the row
  decoder (ADR-27); with `strict=True` a value that `round` would change is an error (owner). 7.4: the
  part / row / column convention.

## Changes in session 4

Basis: rules file v3.4 ("Changes in session 9"), notes N-9 to N-11 (T-3a built), the owner's answers on
the T-3a choices (ADR-23).

- Header and 8.2: 199 fixtures (89 / 85 / 25), 382 sections, 396 binary parts, 238,657 row fields.
- 1.1, 1.4, 1.5: composite partitions read [T10]; direct-path LONG read as piece chains [R2, Y9]; a
  conventional LONG length from 0x8000 is `UnsupportedLayout` (R-040).
- 2.3: chunked column values (direct-path LONG) in the seam. 2.5 steps 3 to 5: the five partition lines
  and the part rules (ADR-21), the chain.
- 3.2, 3.4: `PartInfo.partition` / `subpartition`, `TableInfo.subpartitioned`, `ColumnInfo.file_position`
  (ADR-24); options `unconfirmed_lob` (ADR-25), `number_default="decimal256(p,s)"` (ADR-14);
  `max_lob_bytes` also bounds LONG chains.
- 4.1: a direct-path LONG value is unbounded like a LOB value.
- 5.1: LONG / LONG RAW LargeUtf8 / LargeBinary; `CHAR(n CHAR)`, `VARCHAR2(n CHAR)`, NCHAR, CLOB, NCLOB,
  INTERVAL DAY TO SECOND, UROWID rows updated. 5.2, 5.2a, 5.2b: [Y1] v3.4, Decimal256, the survey's
  candidate counts. 5.4: [C8] (ADR-26), LONG checks, [E2] item starts, [E2a] narrowed. 5.5 step 3: by
  name; `n CHAR` lengths [C6]. 5.6 steps 4, 6, 7: T = 0, surrogate pairs and test vectors, ADR-25. 5.7:
  [Y10], [Y11] v3.4.
- 6.1: LONG and CLOB rows. 7.1: codes. 8.1, 8.2, 8.5: tests and survey for the 22 new files.

## Changes in session 3

Basis: rules file v3.3 ("Changes in session 8"), `implementer-notes.md` (no note since session 2),
`owner-decisions.md` (unchanged since 2026-10-08).

- Header: 177 fixtures (77 / 75 / 25), 334 table sections, 338 binary parts, 237,347 row fields.
- 1.1, 1.4, 1.5: 16 type codes [T5]; LOB, INTERVAL, ROWID, UROWID and partitioned tables are read; what
  still stops the file: [X1], [X5] (ADR-12); `UnsupportedLayout`.
- 2.1, 2.3, 2.5: LOB values after the row in the seam and the walker [L1 to L4]; `PARTITION` /
  `ENDPARTITION` lines and several binary parts per section [T10]; [E2a] (ADR-22).
- 3.2, 3.4: `TableInfo.parts`, `partitioned`; LOB statistics per column; options `max_lob_bytes`,
  `interval_as` (after the alpha).
- 4.1: memory with LOB values.
- 5.1: type table: CLOB, NCLOB, BLOB, INTERVAL, ROWID, UROWID rows. 5.2: NUMBER field count 84,968 and the
  40-digit fact (A-6). 5.4: [R8] for column fields only; [E2a]. 5.5: [T9] texts of session 8. New 5.6
  (LOB reading) and 5.7 (INTERVAL, ROWID, UROWID).
- 6.1: CLOB and NCLOB rows. 7.1: new codes. 8.1, 8.2, 8.5: tests for L, Y10, Y11, T10; 177 files; the
  survey's LOB and partition counts.

## Changes in session 2

Basis: rules file v3.2 ("Changes in session 7"), Implementer notes N-1 to N-8, owner decisions of
2026-10-08.

- Header: 169 fixtures (72 / 72 / 25), all written up by the Analyst.
- 1.1, 1.4, 1.5: LONG (8) and LONG RAW (24) are read [T5, Y9]; the stop [X1] is for 112 and unknown codes
  only; length 0 and 0xfff0 to 0xfffd in row data are fatal [R8].
- 1.3 and new 5.5: the column list is parsed by the confirmed grammar [T9]; the fallback is per column.
- 3.2, 3.4: new option `on_number_overflow`; `number_default` also takes `float64` and `string` (owner, Q-1).
- 4.1: largest field 65,519 bytes [R8].
- 5.1: type table rewritten: declared forms of [T9], NCHAR confirmed [C5], `CHAR(n CHAR)` [C6], LONG, LONG RAW.
- 5.2: NUMBER decoder specified from [Y1]; new 5.2a, the overflow policy (Q-1 reopened).
- 5.4: [R8]; `W-LONGER-THAN-DECLARED` and `CHAR(n CHAR)`; LONG checks; [E2] with a first column of length 0.
- 6.1 to 6.3: character set of LONG; [C7] against `encoding_rs`; one rule for US7ASCII high bytes.
- 7.1: new warning codes; the header checks of N-5.
- 8.1, 8.2, 8.5: 169 files; rows for T9, Y9, C5 to C7; the survey's NUMBER profile.
- 9: `sha2` is a normal dependency (N-6).

Sources. Format facts: `docs/formats/exp_reader_rules.md` v3.4 (cited as [H1], [T5], ...; "rules file"),
`docs/formats/exp.md` (cited as exp.md and section), `docs/formats/requests.md`, and the fixtures. Crate
and standard facts name the crate or standard; versions were read on docs.rs / pyo3.rs on 2026-10-07,
2026-10-14 and 2026-10-15 and are pinned by the workspace. Nothing here comes from a third-party dump
parser.

Fixture count: **199** EXP files (89 / 85 / 25), 170 / 164 / 48 table sections (382; **396** binary
parts: T_PART 3, T_PART2 6 per file), 40,588 rows, 238,657 row fields (19,186 NULL), 777 fields after an
introducing line (rules file "Changes in session 9"; `tools/corpus_counts.py` run read-only on
2026-10-15: `problems 0`, empty parts 24 conventional / 17 direct). Session 3 counted 177 files.

## 1. Scope and non-goals

1.1 **Reads**: the rows of every table section [T1 to T8] of a legacy `exp` file in table mode
(`RTABLES`) or owner mode (`RUSERS`) [H3], conventional or direct path [T6, R4, R7], version lines
`EXPORT:V18.00.00` and `EXPORT:V11.02.00` [H3], any RECORDLENGTH the header states [H5], the 16 column
types of [T5]: 1, 2, 8, 12, 23, 24, 69, 96, 112, 113, 180, 181, 182, 183, 208, 231 (8 and 24 with the
limits of [Y9]: one field on the conventional path, a piece chain on the direct path [R2]; LOB columns by
[L1 to L6]; 5.1), partitioned and composite-partitioned tables [T10] (ADR-21).

1.2 **Skips by framing, never interprets**: statistics items [S1, S2, S7], grants, comments, constraints,
indexes, triggers, views, procedures, sequences, synonyms, the trailer groups [I1 to I7, T7]. They are
counted by kind for diagnostics only.

1.3 **Reports, does not interpret**: the `CREATE TABLE` text of each table, decoded with the client set
[C2], as `TableInfo.ddl`. One narrow exception: its column list is parsed by the grammar of [T9]
(confirmed for 334 lines) for declared precision, scale, fraction digits and length semantics (ADR-6,
5.5); a column whose type text is not in [T9], or a line that does not parse, falls back to the
descriptors alone with `W-DDL-FALLBACK`.

1.4 **Refuses** (typed error, file offset where one exists; codes in 7.2):

| condition | rule | error |
|---|---|---|
| byte 0 is not `03`, or line 1 does not start with `EXPORT:V` | H1, H3 | `NotExpFile` (message names gzip / zip / bzip2 / zstd magic bytes when it sees them: "compressed input is not supported") |
| header lines or binary fields cannot be parsed | H3, H4 | `BadHeader` |
| RECORDLENGTH is not a decimal number, is 0, or R + 0x104 lies beyond the file | H3, H5 | `BadRecordLength` |
| the line at R + 0x104 is neither `METRICST` nor `METRICSU` | H6 | `MissingMetricsLine` |
| a field or line that breaks the framing; `INSERT INTO` where no table section is open; column count of the `INSERT INTO` line differs from the descriptor count (N-4); a second `INSERT INTO` in a section in which no naming line (`PARTITION "p"`, or `SUBPARTITION "s"` inside `COMPARTITION`) was seen; parts of one table with different descriptors | I2 to I5, T1, T3, T4, T10 | `Framing` |
| a length field of 0, or 0xfff0 to 0xfffd, in a **column field**; in a direct-path LONG chain a later piece u16 from 0xfff0 to 0xffff | R8 (recommendation), R2 | `Framing` at the length field |
| (session 25, ADR-99; replaces the session 24 row) a conventional-path (flag 0) **LONG or LONG RAW** field starting `fb ff` or `fc ff`. Its other first u16 values: 1 to 0xfff9 a length (0xfff2 to 0xfff4 with `W-UNTESTED-LAYOUT`), `fa ff` the 4-byte-length form, `fd ff` the chunked form [R2 v3.9] | R8 v3.9 ("stop") | `UnsupportedLayout` at the field's u16: stops the file (ADR-12 as amended, ADR-99) |
| (session 25) in the `fa ff` form, an n outside 0xfffa to 0xfffd or upper bytes not `00 00`; in the chunked form, a chunk u16 from 0xfff0 (both types) | R2 v3.9, ADR-99, ADR-101 item 2 | `Framing` at that u16 |
| LOB value area: k differs from the non-NULL LOB columns of the row; a u16 that is neither 0 to 0xffef nor `fc ff`; a chunk length 0 or from 0xfff0; chunks passing the total T | L3, L4 | `Framing` at the offset |
| type code not among the 16 of T5 (BFILE, object, XML, ...) | X1 | `UnsupportedType`: stops the file; tables before it stay readable (ADR-12) |
| m neither 0 nor the column count; m > 0 with path flag 1; a column entry with n > 0 for a non-LOB column or n = 0 for a LOB column; path flag not 0 / 1 | T6, L1, X5 | `UnsupportedLayout`: stops the file like `UnsupportedType` (ADR-12) |
| zero fill or end of file before the two `EXIT` lines | E4, procedure step 6 | `Truncated` |
| encryption | - | not a property of this format as far as `docs/formats/` knows; such a file fails as `NotExpFile` |

1.5 **Non-goals of phase 1**: Data Pump; BFILE, object, REF, collection and XML columns [X1];
conventional-path LONG / LONG RAW fields starting `fb ff` or `fc ff`, a form no fixture shows (they stop the file,
ADR-99; session 25: R-048 and R-051 answered every size, ADR-99); the LOB locator's content [L2]; partitioning methods other than range and range-hash as
confirmed cases (read with the rules of ADR-21, warnings where the line order differs); full database mode
(no fixture, not requested); multi-file dumps; decoding of statistics or comments; writing dumps;
resynchronising after a framing error (the rules forbid searching the byte stream, chapter 10).
Session 3 moved LOB, INTERVAL, ROWID, UROWID and partitioned tables from here to 1.1.

## 2. Layers

2.1 Dependency direction: each layer uses only the layers below it. `error` is used by all.

| # | layer (module) | does | rules |
|---|---|---|---|
| 9 | `oradmp-py` (separate crate) | Python classes, exceptions, pyarrow hand-over | - |
| 8 | `api` | `Dump`, `Options`, `Catalog`, `TableInfo`, `TableBatches` | - |
| 7 | `arrow_out` | column builders, batch assembly, schema and metadata | - |
| 6 | `values` | value bytes to typed value: number, datetime, interval, rowid, text, raw, LOB assembly; declared types | Y1 to Y11, C1, C3, C5, C6, L5, T9 |
| 5 | `charset` | Oracle character set id to decoder (streaming for UTF-16 LOB values) | C1 to C4, C7, X3, L5 |
| - | **seam: trait `RowSource`** | format-neutral: table headers, raw fields, LOB values as chunk streams | - |
| 4 | `exp::table`, `exp::rows` | `INSERT INTO` line, descriptors, m, path flag, LOB column entries, rows, direct-path LONG chains, LOB value area, end of rows | T3 to T6, T10, L1 to L4, L6, X5, R1 to R8, E1 to E3, E2a, Y9 |
| 3 | `exp::items` | item walker; table sections; the five partition lines (ADR-21); list K as data; fallback G1 | I1, I6, G1, T1, T2, T7, T10, S1, S2, E4 |
| 2 | `exp::framing`, `exp::header` | line, plain field, pieces, pieces without marker; first and second record | I2 to I5, I7, H1 to H6 |
| 1 | `io` | buffered byte stream with absolute offset over `Read + Seek` | R5, I7 |

2.2 **Two crates** (ADR-1). `oradmp-core`: layers 1 to 8, pure Rust, no Python. `oradmp-py`: layer 9,
a `cdylib` built by maturin. Layers 1 to 8 are modules of one crate because nobody consumes them
separately; the direction of 2.1 is kept by `pub(crate)` visibility and a review rule ("`exp::*` never
imports `values`, `charset` or `arrow_out`; `values` never imports `exp`"). The core is a separate crate
so that `cargo nextest` runs without a Python interpreter.

2.3 **The seam** (ADR-2). `RowSource` is the only thing a second format implements:

    trait RowSource:
        next_table()  -> Result<Option<TableHeader>>   // name, columns (name, type code, max bytes,
                                                        // charset id, form, is_lob), parts (name, path,
                                                        // offsets), ddl text
        next_field()  -> Result<Field>                 // Null | Value(&[u8]) | LobPresent | LobValue(col,
                                                        // total, chunks) | EndOfRow | EndOfPart | EndOfTable
        skip_rows()   -> Result<u64>                    // count rows by lengths only (pre-scan)
        position()    -> u64

Session 3 (LOB): in a row, a LOB column's field is `LobPresent` (a locator was skipped [L2]) or `Null`;
after the last column field the source yields one `LobValue` per present LOB column, in column order
[L3], then `EndOfRow`. `LobValue` carries the column position, the total length (n of the short form or
T of the long form [L4]) and a chunk iterator that lends `&[u8]` slices of the I/O buffer (a short value
is one chunk); the consumer must drain or skip it before the next call, and skipping seeks over the
chunks by their lengths. The seam stays format-neutral: a Data Pump source may deliver a LOB value
inline as one chunk. `EndOfPart` separates the binary parts of a partitioned table [T10].

Session 4 (direct-path LONG [R2, Y9]): a LONG / LONG RAW field of a part with flag 1 is yielded as
`Chunked(col, chunks)`: the same lending chunk iterator as `LobValue`, with no total known in advance
(one piece = one chunk; the iterator ends after the piece whose u16 has bit 0x8000 clear). It stands in
the field sequence at its file position (last on the direct path [T3]). Every other column field,
including a conventional LONG, stays `Value(&[u8])`. `TableHeader.columns` carries both the file order
and, after T-5, the declared position (ADR-24); the seam never reorders fields.

Session 6 (T-6, ADR-33 item 1): the seam is built as a **push** interface: `RowCursor::step_with(stream,
hasher, sink, env)` lends each field to a `RowSink` (`row_start`, `null`, `value`, `chain_piece(first,
last)`, `row_end`; `column` = file position, offset = first byte of the value or piece); the scan uses
`step` without a sink. T-6L adds `lob_start(column, offset, total) -> Read | Skip`, `lob_chunk(column,
offset, bytes)`, `lob_end(column)` for the value area [L3, L4]; on `Skip` the cursor passes the chunks by
their lengths (still checking them, [L4]). Session 7 (as built, ADR-34 item 1): `lob_start(column, offset,
form, total) -> Result<LobAction>` with `form` = `Short | Long` as read from the bytes [L4] (ADR-25 needs
it; `total` cannot tell the forms apart without a chunk-size table); `lob_end` follows only a `Read`; the
three methods have default bodies that skip.

Everything above it (value decoding, Arrow, options, Python) is format-neutral; everything below is
`exp`. `Dump::open` chooses the implementation by the first bytes. Whether Data Pump value bytes equal
the `exp` ones is **unknown** to this design; if they differ, `values` gets a second decoder set selected
by a tag in `TableHeader`. No plug-in registry, no generic "format framework".

2.4 **List K is data** (ADR-9). `crates/oradmp-core/data/exp_intro_lines.txt`, one entry per line in the
form of the rules file plus the allowed field forms: `kind:length:forms:text`, for example
`suffix:15:plain: MODIFY DEFAULT` and `prefix:10:plain|marked:ANALSTATS ` (the length makes trailing
spaces visible [I6]). It is embedded with `include_str!` and parsed once. A test parses the list block of
`docs/formats/exp_reader_rules.md` and fails when the two differ. `Options.extra_intro_lines` adds
entries at run time, so an unknown line in a real dump does not need a release.

2.5 **Walker behaviour** (layer 3), exactly the procedure of chapter 9 of the rules file:

1. Line matches list K: read its one field in an allowed form [I6]; `ANALSTATS `: the two bytes decide
   [S1]. A form the entry does not allow is `Framing`.
2. Line does not match: apply G1 (next two bytes `fd ff`, or second byte below 0x20: a field). Then warn
   `W-UNKNOWN-INTRO` with the offset. G1 is "suggested", so `strict` makes it an error (7.1).
3. `TABLE "name"` followed by `CREATE TABLE`: section [T1]; it ends by [T2]. `TABLE` / `ENDTABLE` are
   never paired (chapter 10). Items in a section are skipped one by one, never counted [T7, S7].
4. `INSERT INTO` inside a section: binary part [T3 to T6; L1 when m > 0]; its column names and the
   order of descriptors and fields are those of this line [T3], which can differ from `CREATE TABLE`
   (ADR-24). Partition lines (session 4, ADR-21): `PARTITION "p"`, `COMPARTITION "p"`,
   `SUBPARTITION "s"`, `ENDCOMPARTITION`, `ENDPARTITION` are recognised plain lines inside a section,
   never G1 candidates or list-K lines; a **naming line** is `PARTITION "p"`, or `SUBPARTITION "s"` while
   a `COMPARTITION` is open. The first `INSERT INTO` of a section is always read. A further one is
   `Framing` if no naming line was seen in the section; if one was seen but the line directly before is
   not a naming line, the part is read, named after the last naming line, with `W-UNTESTED-LAYOUT`
   (owner, ADR-23). Every part's descriptors, m, column entries and names equal the first part's (else
   `Framing`). The confirmed shapes are (`PARTITION`, part)+ and (`COMPARTITION`, (`SUBPARTITION`,
   part)+, `ENDCOMPARTITION`)+, each followed by items (none, or the `ANALSTATS` items of the table and
   its partitions [T10]) and one `ENDPARTITION`; `ENDPARTITION` does not end the section, [T2] does. Any
   other order of these lines, or one of them outside a section: `W-UNTESTED-LAYOUT` once per section
   (the list is in ADR-21 item 4). Outside a section, `INSERT INTO`: `Framing`.
5. Rows by [R1 to R4]; in a table with m > 0, the u16 after the last field of a row is k, followed by
   k LOB values [L3, L4] (5.6); with path flag 1 a LONG / LONG RAW field is a piece chain [R2] (ADR-15:
   first u16 `fe ff` NULL; 0, 0xfff0 to 0xfffd, `ff ff` `Framing`; later u16 from 0xfff0 `Framing`;
   pieces of length 0 `W-UNTESTED-LAYOUT`); with flag 0 a LONG or LONG RAW field is one field (u16 1 to 0xfff9; 0xfff2
   to 0xfff4 with `W-UNTESTED-LAYOUT`), or `fa ff` + u16 n + `00 00` + n bytes (n 0xfffa to 0xfffd), or, after `fd ff`,
   chunks `u16 n + n bytes` up to a chunk `00 00`; `fb ff` / `fc ff` are `UnsupportedLayout` (session 25, ADR-99). End by [E1]; with path flag 1 also [E2] (5.4 says how the
   two are told apart without the version line [E3]; a length-0 first column, i.e. a single-column LONG
   table: [E2a], ADR-22). A LOB table never has path flag 1 [L6]; if it does: `UnsupportedLayout` [X5].
6. Second of two consecutive `EXIT` lines: done [E4]. Bytes after it that are not zero: `W-TRAILING`.

2.6 **Limits against garbage input**: a line longer than `max_line_bytes` (default 1 MiB; the longest
line in the fixtures is a `CREATE TABLE` of 120 columns) is `Framing`; skipped fields are never
buffered; a field kept in memory (DDL, names) is capped by the same limit.

## 3. Public API

Signatures are prose, not compiled code.

3.1 **Model** (ADR-4, ADR-8): open (header only), **scan** (one sequential pass that reads every length
field and skips the values; builds the catalog), then **independent table readers** that seek to a
table's row offset. There is no directory in the file and no row count [R6], so a table list needs the
scan; it runs once, lazily, and is cached in memory.

3.2 **Rust** (`oradmp_core`):

    open(path, Options) -> Result<Dump>                  // reads the first record and R + 0..0x10d only
    Dump::info() -> &DumpInfo        // format, version line, user, mode, record length, direct flag [H2],
                                     // client / database / national set ids and names [H1, H4], file size
    Dump::scan() -> Result<&Catalog>                     // idempotent; Catalog { tables, end, item_counts }
    Dump::tables() -> Result<&[TableInfo]>               // = scan().tables
    Dump::read_table(TableRef, ReadOptions) -> Result<TableBatches>
    Dump::warnings() -> &[Warning]                    // the scan's warnings only (session 6, ADR-33)
    Dump::rows(TableRef) -> Result<TableRows>         // Rust-only diagnostic cursor, values skipped; a table
                                                      // of more than one part: E-CONFIG (ADR-32 item 3)
    Dump::part_rows(TableRef, index) -> Result<TableRows>  // one part (0-based); bad index: E-CONFIG

    TableInfo:  ordinal, name, name_raw, name_possibly_damaged, columns: [ColumnInfo], partitioned,
                parts: [PartInfo] (one for an unpartitioned table; ADR-21), num_rows (sum of the parts),
                possibly_incomplete [X2], ddl (text), ddl_lossy
    PartInfo:   partition, partition_raw (None unless a PARTITION or COMPARTITION line names it [T10]),
                subpartition, subpartition_raw (None unless a SUBPARTITION line names it; session 4),
                path (Conventional | Direct) [T6], num_rows, rows_offset, rows_end_offset,
                end_marker (FfFf | LfOnly) [E1, E2],
                file_order (session 5, ADR-31: for each file position of this part the declared position)
    TableInfo.subpartitioned: a COMPARTITION line was seen (session 4, ADR-21)
    ColumnInfo: position (declared order, ADR-24), file_position (order in the part [T3]), name,
                type_code, oracle_type (text), max_bytes, charset_id, form [T5],
                declared (precision, scale, fractional digits, leading precision of an interval) if the
                DDL parse gave them, arrow_type; for LOB columns lob: LobStats (from the scan: values,
                empty, short, long, largest stored length; counts only); for LONG / LONG RAW columns
                long: LongStats (values, chained values, largest total, count above 32,000 bytes);
                type_source (session 6, ADR-33 item 4): Declared | Default | Override, for type-2
                columns: which rule fixed the Arrow type, and so whether on_number_overflow applies;
                session 7 (ADR-35): NUMBER(*,s) with 0 <= s <= 38 is Declared; set but meaningless for
                other type codes
    Warning:    code, offset, table, part, column (file position), row, length, message; for the
                once-per-column value warnings count, count_to_zero (W-NUMBER-ROUNDED) and
                max_digits_dropped (W-NUMBER-ROUNDED, T-11); session 6
    TableRef:   ByName(str) | ByOrdinal(u32)    // ByName fails with AmbiguousTableName on duplicates
    TableBatches: implements arrow_array::RecordBatchReader (crate arrow-array) and Send; owns its own
                file handle; rows_read(); warnings(): the warnings of this read; the counted ones
                (count) are added when the reader returns None or an error (session 6, ADR-33 item 3);
                lob_bytes_read(), lob_values_skipped(): #[doc(hidden)] test diagnostics, no API
                promise, not in Python (session 7, ADR-34 item 2)
    Catalog.end: Complete | Stopped(Error)      // X1, X5: tables before the stop are listed and
                                                // readable; the error names the stopped table (from its
                                                // TABLE line), offset, column position and type code

Session 3: `TableInfo.path`, `rows_offset`, `rows_end_offset`, `end_marker` of T-3 move to `PartInfo`
(T-3b); `TableInfo.path` stays as a convenience: the path of the parts if they agree, else `Mixed`.

`Options`: `strict` (bool, default false, Q-2 accepted), `on_value_error` (Raise | Null, default
Raise), `db_charset`, `client_charset` (override by name, 6.4), `number_default` (`decimal(38,10)`
default, `decimal(p,s)`, `float64`, `string`: owner decision Q-1; session 4: also `decimal256(p,s)`,
p up to 76, ADR-14), `on_number_overflow` (Round | Error |
Null; default **Round**, owner 2026-10-16, ADR-14; with `strict` a value Round would change is
`E-NUMBER-OVERFLOW`), `number_types` (per column: a type of `number_default` or `int64`, and optionally an
overflow policy; it wins over every type-2 column, declared ones and FLOAT included, ADR-30 item 6), `timestamp_unit` (Auto | Micro | Nano),
`trim_char` (default false), `extra_intro_lines`, `max_line_bytes`, `max_lob_bytes` (default 256 MiB,
stored bytes, session 7: u16 n or T of [L4], so a UTF-16 CLOB counts 2 bytes per BMP character; per LOB value and, session 4, per LONG / LONG RAW value, ADR-18, ADR-15), `interval_as` (`native`
default; `string`, session 11: in the alpha, ADR-47), `unconfirmed_lob` (`error` default | `decode`; session 4,
ADR-25).
`ReadOptions`: `columns` (names; unselected fields and unselected LOB values are skipped by length,
never decoded or held; result keeps table order), `batch_rows` (default 65,536), `batch_bytes` (soft
cap, default 64 MiB; a batch closes at whichever comes first, and at the end of a part, 5.6),
`partitions` (after the alpha: names of the parts to read, ADR-21).

Session 6 (ADR-33 item 2), **the split as built and decided**: `Options` holds what the scan or the type
resolution uses and is fixed for the life of a `Dump`: `strict`, `db_charset`, `client_charset`, `types`
(`number_default`, `number_types` with the optional per-column overflow policy, `timestamp_unit`),
`interval_as`, `extra_intro_lines`, `max_line_bytes`, `buffer_bytes` (at least 128 KiB is used whatever
is set). `ReadOptions` holds what one read uses: `columns`, `batch_rows`, `batch_bytes`,
`on_value_error`, `on_number_overflow` (the global policy; an entry policy of `number_types` wins over
it), `max_lob_bytes`, `unconfirmed_lob`, `trim_char`, and later `partitions`. The `Options` list of the
paragraph above is the Python `open()` signature (3.4), not the Rust struct.

3.3 **Row counts.** `TableInfo.num_rows` is exact after the scan and is the only row count the reader
offers. `METRICET` / `METRICEU` numbers and `num_rows` of statistics are never used [R6, chapter 10].
The scan is the "cheap pre-scan": it reads the whole file once but decodes nothing; its speed is measured
(4.3). A reader re-opens the path and refuses with `FileChanged` if size or modification time differ
from the scan.

3.4 **Python** (`import oradmp`; ADR-3):

    oradmp.open(path, *, strict=False, on_value_error="raise", db_charset=None, client_charset=None,
                 number_default="decimal(38,10)", on_number_overflow="round", number_types=None,
                 timestamp_unit="auto", trim_char=False, max_lob_bytes=256 * 2**20,
                 unconfirmed_lob="error", interval_as="native") -> Dump
    # session 11 (ADR-47): interval_as "native" | "string", fixed for the Dump (it changes the schema)
    # session 21 (ADR-82): path is str | os.PathLike[str]; a name that is not valid UTF-8 (os.fsdecode form) opens
    # the file with its original bytes (binding parameter PathBuf; T-18d); bytes paths are not promised
    # TableInfo.parts -> list[PartInfo] (partition, subpartition, path, num_rows);
    # ColumnInfo.lob -> LobStats | None; ColumnInfo.long -> LongStats | None
    # number_types: {"TABLE.COLUMN": "int64" | "float64" | "string" | "decimal(p,s)" | "decimal256(p,s)"
    #                 or ("decimal(p,s)", "round" | "error" | "null")}
    Dump: context manager (close() drops the catalog and marks the object closed; readers already
          handed out keep their own handle and stay valid)
      .info -> DumpInfo            .tables -> list[TableInfo]   (runs the scan once, GIL released)
      .scan() -> Catalog           .warnings -> list[DumpWarning]
      .read_table(name_or_ordinal, *, columns=None, batch_rows=65536, batch_bytes=...,
                  on_value_error=None, on_number_overflow=None, max_lob_bytes=None,
                  unconfirmed_lob=None, trim_char=None) -> BatchReader
                  # session 6: None = the value given to open(); one Rust ReadOptions per call
      .read_all(*, columns=None) -> dict[str, pyarrow.Table]   (convenience; materialises; small files)
    BatchReader: iterator of pyarrow.RecordBatch and context manager
      .schema -> pyarrow.Schema    .read_all() -> pyarrow.Table    .rows_read    .warnings
      .__arrow_c_stream__(requested_schema=None)   (Arrow PyCapsule Interface: any Arrow consumer)
      # session 14 (ADR-55): the exported stream shares the reader's state. rows_read / warnings follow the
      # consumer's pulls (counted warnings complete at the stream's end or error); close() / __exit__ issue the
      # warnings not yet issued; after the export __next__ / read_all() raise UsageError (E-CONFIG); close()
      # while the stream is alive makes its next pull fail with a text containing "[E-CONFIG ..." (never a silent end)
      # session 15 (ADR-61): after close() the iterator too raises UsageError (E-CONFIG) unless it had ended already

- **Hand-over to pyarrow**: crate `pyo3-arrow` (0.19 on docs.rs, depends on pyo3 0.29 and arrow-rs 59;
  it fixes the pair of versions; session 11, as built: **pyo3-arrow 0.20.0, pyo3 0.30.1, arrow 59.3.1**, bump rule
  in ADR-3). `PyRecordBatchReader::new(Box<dyn RecordBatchReader + Send>)` wraps the
  core reader and exports `__arrow_c_stream__`; `into_pyarrow` gives a `pyarrow.RecordBatchReader` and
  needs pyarrow 15 or later (pyo3-arrow documentation). Data cross as Arrow C Data Interface structs: no
  copy, no pickling. pyarrow is a runtime dependency of the package and is imported at first read.
- **Why our own `BatchReader` class** and not a bare `pyarrow.RecordBatchReader`: an error that crosses
  the C stream interface arrives as a message only (the Arrow C stream interface carries an errno and a
  string). `BatchReader.__next__` calls the core directly and raises the typed exception; the capsule
  path stays for consumers that want the stream, and every message starts with the stable error code and
  offset (`[E-FRAMING offset=0x1a2b] ...`) so that the code survives there too. Session 15 (ADR-61): on the stream
  path arrow-rs puts `External error: ` before the text and pyarrow raises `pyarrow.ArrowInvalid`, so there the
  message **contains** the code and offset; a consumer finds the code by the pattern `\[(E-[A-Z0-9-]+)` (first match),
  never by a prefix. No message contains U+0000 (arrow-array's export would panic on it inside the C callback).
- **GIL**: `Python::detach` (PyO3 0.26 and later; before that `allow_threads`) around `open`, `scan` and
  every `__next__`. The core touches no Python object. Whether pyarrow holds the GIL while it pulls from
  the capsule is not assumed: T-12 has a two-thread test that measures it.
- **Exceptions**: `OradmpError(Exception)` with `.code`, `.offset`, `.table`, `.part`, `.column`, `.row`
  (session 5, ADR-29: `.table` the name, `.part` the partition / subpartition names or None, `.row`
  1-based within the part, `.column` the column name resolved from the core's file position; session 11, ADR-46
  item 4: also `.table_ordinal` and `.type_code`; after a failed scan `.table` is None and `.table_ordinal` is set);
  subclasses `FormatError` (NotExpFile, BadHeader, BadRecordLength, MissingMetricsLine, Framing,
  Truncated), `UnsupportedError` (UnsupportedType, UnsupportedLayout, UnsupportedCharset), `ValueDecodeError`,
  `UsageError` (kinds, not classes: NoSuchTable, AmbiguousTableName, NoSuchColumn, FileChanged, and an invalid option
  value as `E-CONFIG`, session 12, ADR-51), and (session 9,
  ADR-41 item 1) `StrictError` for `E-STRICT` (a direct subclass of `OradmpError`); `E-UNSUPPORTED-CHARSET` is
  `UnsupportedCharset`; every exception has `.warning` (the `W-...` code a `strict` conversion turned into the
  error, else None). Session 7:
  every value-class code of 7.1 is a `ValueDecodeError`, `E-LOB-UNCONFIRMED` included (`.code` tells them
  apart; ADR-34 item 4); `W-LOB-UNTESTED` and `W-TSLTZ` are `OradmpWarning`s (`W-TSLTZ` once per type-231
  column in `Dump.warnings`, never repeated by a read, ADR-35 item 4). I/O errors are
  `OSError`. `OradmpWarning(UserWarning)` is issued through the `warnings` module once per **entry** of the list
  (session 7, ADR-35 item 4; was "once per code and file", which contradicted ADR-33 item 3); the full list
  is in `.warnings`. Session 6 (ADR-33 item 3): `Dump.warnings` holds the scan's warnings;
  `BatchReader.warnings` holds the read's, including the counted ones (`.count`, `.count_to_zero`,
  `.max_digits_dropped`), which exist only once the iteration has ended (StopIteration or a raised
  error); the binding issues the new ones through `warnings` at that moment. `read_all` and the survey
  do the same per table.
- **No value in any message.** Errors and warnings carry code, offset, table, column, row number and
  lengths; never value bytes. Logs of a corporate ETL job stay free of data. Session 10 (ADR-43 item 1): the
  test searches the `message` field for raw bytes, text runs of the lossy rendering, hex tokens and the
  reader's rendered cells; integers (row, piece, set id) are allowed.

3.5 **Command-line tool** (session 18, ADR-72; task T-18). `oradmp` (console script `oradmp._cli:main`; `python -m
oradmp` runs the same) is a thin layer over 3.4: it calls only the public Python API, adds no reading logic and no
dependency, and ships in the same wheel.

| command | does | stdout |
|---|---|---|
| `oradmp tables FILE` | the scan | tab-separated: `ordinal`, `name`, `rows`, `columns`, `parts`, `path`, `partitioned`, `possibly_incomplete` |
| `oradmp columns FILE TABLE` | one table's `ColumnInfo` | tab-separated: `position`, `name`, `oracle_type`, `declared`, `arrow_type`, `not_null`, `type_source` |
| `oradmp head FILE TABLE [-n N]` | the first N rows (default 10) | CSV with a header line |
| `oradmp export FILE TABLE -o PATH [--format csv\|parquet]` | one table to a file (`-o -`: CSV to stdout) | nothing (CSV with `-o -`) |
| `oradmp export FILE --all -o DIR [--format csv\|parquet]` | every table, one file each | manifest: `ordinal`, `name`, `file`, `rows` |
| `oradmp survey FILE [--no-names] [--no-read]` | unchanged (8.5) | the survey |

- A table is named by name or by `--ordinal N`. Read options mirror `open` / `read_table` (ADR-72 item 3); the one
  default that differs is `--interval-as string`. The format of `export` comes from `--format` or the suffix
  (`.csv`, `.parquet`), else `E-CONFIG`.
- Writers: `pyarrow.csv.CSVWriter` and `pyarrow.parquet.ParquetWriter`, batch by batch (memory as 4.1); binary
  columns as upper-case hex in CSV; a type the CSV writer refuses stops before the first row (`E-CONFIG`). No pandas.
- Reading uses the iterator path (typed errors), never the Arrow C stream. Data on stdout only; errors and warnings
  on stderr only (`oradmp: error: [<code> offset=...] ...`, `oradmp: warning: <text>`), never a value in either.
- An export writes `<target>.oradmp-partial` and renames it after the last batch; on any error or interrupt nothing
  is left; an existing target needs `--overwrite`.
- Exit codes by error class: 0 done; 1 `FormatError`, `UnsupportedError`, `ValueDecodeError`, `StrictError`; 2
  usage (argparse, `UsageError`, the tool's own `[E-CONFIG]` refusals); 3 `E-IO`; 4 `E-INTERNAL`; 130 interrupt.
  `survey` keeps 0 / 1. Session 20 (ADR-77 item 4): an `OSError` of the API (the dump) is printed as `oradmp: error: `
  + its `[E-IO ...]` prefix + the OS text of its error number + `: FILE` (e.g. `[E-IO] No such file or directory:
  missing.dmp`); an output-side `OSError` as `[E-IO] <strerror>: <filename>`. The suffix (`.csv`, `.parquet`) is
  matched in any case. Session 21 (ADR-82): in FILE (and in `survey`'s `file:` line) each byte that is not valid UTF-8
  is written `\xHH`, before the `\t`, `\r`, `\n` rule; `export --all` may have printed the manifest header on stdout
  before an exit-3 error (unchanged; the exit code decides). Session 22 (ADR-86 item 4): the same rule for every other
  path in a message: the `export -o` target ("exists" refusal) and the file of an output-side `OSError`.
- Not in phase 1: Data Pump, row filters, SQL, JSON or Excel output, writer options, parallel export, stdin or URLs.
- Session 19 (ADR-75): Parquet has no seconds unit, so a DATE column (Timestamp(second)) is stored and read back as
  Timestamp(millisecond), same instants (pyarrow's defaults kept). `head` prints no counted warning: they exist only at
  the end of a read (3.4), and `head` stops after N rows. A TABLE not found in a **stopped** scan gives the stop error
  and its exit code, not `E-NO-SUCH-TABLE` (T-18b). The table comes right after the file (Python 3.11's argparse).
  `--version`; `-q` on every command but `survey`; `tables` / `columns` print `true` / `false` and an empty field for
  "none".

## 4. Memory and speed

4.1 **Requirements.** (a) Memory does not grow with the file: peak = I/O buffer (1 MiB) + builders of one
batch (bounded by `batch_rows` and `batch_bytes`) + one finished batch + the catalog (a few hundred bytes
per table). (b) No whole-file read, no `mmap` (ADR-4). (c) I/O is sequential inside a pass; the only
seeks are to R [H5] and to a table's `rows_offset`. (d) A value is decoded from the I/O buffer straight
into its Arrow builder (crate `arrow-array`, `builder` module); a value that spans two buffer fills goes
through one reusable scratch buffer (a field is at most 65,519 bytes = 0xffef: 0xfff0 to 0xfffd are
errors [R8], `fe ff` and `ff ff` are markers [R3, E1]; the largest field seen is 32,000 bytes [R2]). No per-row or per-value heap allocation, no intermediate row objects.
(e) Session 3, LOB values [L4] have no such bound (T is 8 bytes; 200,000 seen). A LOB value is decoded
chunk by chunk straight into its builder (`GenericStringBuilder` implements `std::fmt::Write` and
`GenericBinaryBuilder` implements `std::io::Write`; written bytes become part of the next value, which
`append_value("")` finishes: arrow-array documentation, 60.0.0 on docs.rs, 2026-10-15; **verified** by T-6
on the pinned 60.0.0, including `append_null()` after a partial write, ADR-18 session 6), so no copy of the whole value is kept besides the builder. Peak memory therefore
becomes: I/O buffer + one batch (`batch_rows`, `batch_bytes`) + at most one row's LOB values beyond
`batch_bytes` (each at most `max_lob_bytes` stored bytes, at most 1.5 times that as UTF-8 for UTF-16
input of the Basic Multilingual Plane) + one finished batch + the catalog. A value above `max_lob_bytes`
is refused before any of its bytes is read (ADR-18).
(f) Session 4: a direct-path LONG / LONG RAW value [R2] is unbounded like a LOB value but has no total in
advance; its pieces go straight into the LargeUtf8 / LargeBinary builder (LONG text through a streaming
decoder, ADR-15), and `max_lob_bytes` is checked as the pieces accumulate. The bound of (d) (65,519
bytes) holds for every other column field, the conventional LONG included.

4.2 **Record boundaries cost nothing**: nothing is inserted at multiples of R [R5, I7], so layer 1 is a
plain byte stream and R is used only for the offset of the second record and the size check [H5].

4.3 **Measured** (T-15), single thread, release build, on the project VM: scan MB/s; decode rows/s and
MB/s per table; peak RSS. Reported, not promised: no number is a target before the first measurement;
after it, CI fails on a regression of more than 20 % against the recorded baseline, and the memory test
requires the peak RSS of a file ten times larger to stay within 10 %.
Session 16 (ADR-65 item 6, ADR-66): the baseline, the commands and the regression rule (20 % on the VM, confirmed by a
second run; memory: the 1.10 ratio, `stream_memory.py`, a peak 15 % above the baseline) are in `benchmarks.md`; until
a CI runner has its own stable baseline (T-16) the gate is the VM run of release checklist step 3b. The memory test
compares a read stopped at 12,288 rows with the full 100,000 rows of `T_ROWS` (about eight times the data); on a 10 MB
file it resolves a retention of about 6 bytes per row or more; `T_BIG`, when present, sharpens it.
Session 20 (ADR-78): baseline 2 (code of `460aef5`, with `T_BIG`) is the reference; `T_BIG` resolves 1 byte per row
(measured); `T_BIG` throughput regresses at 10 %, the rest at 20 %; per-file peak limits (`benchmarks.md` section 5).
`T_BIG` is made by `corpus/sql/15_big.sql` (O-17), not by the 01b block named in 4.4.

4.4 **Benchmark inputs.** Micro-benchmarks (crate `criterion`) of the value decoders and the row loop on
committed fixtures: `ru_stat_exp_cp1251_est_11g.dmp` (R_STAT, 2000 rows, 7 columns, Cyrillic text) and
`t_num` / `t_text`. Throughput and memory need large files that are **not** committed: the `T_ROWS`
export (100,000 rows, `corpus/sql/01_corpus_tables.sql`) and the optional `T_BIG` (5,000,000 rows,
`corpus/sql/01b_optional_tables.sql`), produced by the project owner into `corpus/work/` (this session
did not read that folder). The benchmark skips with a message when they are absent.

4.5 **Not optimised in phase 1**: SIMD validation, dictionary encoding, `Utf8View`, parallel decoding
inside a table, a persistent index, reading from pipes or object stores.

4.6 **Parallelism** (ADR-10). None inside the core in phase 1. Tables are sequential in the file and a
row can only be found by walking the lengths before it [R1, R6], so one table cannot be split. What the
design allows without change: after the scan every `TableBatches` has its own file handle and is `Send`,
so the caller may read different tables in different threads (Python: threads, the GIL is released).
Later, if profiling asks for it: one walker thread that hands blocks of raw rows to decoder threads.

## 5. Types

5.1 The descriptor gives type code, maximum byte length and, for types 1, 96 and 112, character set id and
form [T5]. It gives **no** precision, scale or NOT NULL: all ten numeric forms of `t_decl` give `02 00
16 00` (exp.md 4.1). The declared type stands only in the column list of the `CREATE TABLE` line,
whose grammar is **confirmed** for the fixtures [T9] (exp.md 4.7; 5.5 below). Arrow type by declared
type (session 2; "decl." = the type text of [T9]):

| Oracle type: descriptor code; decl. text | Arrow type | notes | status |
|---|---|---|---|
| 2; `NUMBER(p, 0)`, p <= 18 | Int64 | exact or value error | decoding confirmed [Y1]; decl. confirmed [T9] |
| 2; `NUMBER(p, 0)`, 19 <= p <= 38 | Decimal128(p, 0) | exact or value error | same |
| 2; `NUMBER(p, s)`, 0 < s <= p | Decimal128(p, s) | exact or value error | same |
| 2; `NUMBER(p, s)`, p < s <= 38 (`NUMBER(3, 5)`) | Decimal128(s, s) | exact or value error | same |
| 2; `NUMBER(p, s)`, s < 0 (`NUMBER(5, -2)`) | Int64 if p - s <= 18; Decimal128(p - s, 0) if p - s <= 38; else the row "no Decimal128" below | exact or value error | same |
| 2; `NUMBER(*,0)` (= INTEGER [T9]) | **Decimal128(38, 0)** | `*` means precision 38 (`truth_decl.txt`: precision NULL, scale 0); Int64 would fail on ordinary 19-digit keys; `int64` by override (ADR-6) | same |
| 2; `NUMBER(*,s)`, 0 < s <= 38 (`NUMBER(*,2)`) | Decimal128(38, s) | `type_source` Declared: exact or value error (session 7, ADR-35 item 1) | same |
| 2; no Decimal128 holds the declaration (session 5, ADR-30 item 3): `NUMBER(p, s)` or `NUMBER(*,s)` with s > 38; s < 0 with p - s > 38; `NUMBER(*,s)` with s < 0 (`*` = 38) | `number_default` | `W-DDL-FALLBACK` (table, column, "no Decimal128 holds NUMBER(p,s)") in the scan; `on_number_overflow` applies; `declared` keeps p, s; exact by `number_types` (`decimal256(76,s)`, `string`), then no `W-DDL-FALLBACK` (ADR-35 item 2). Session 7 example `NUMBER(*,-2)`: Decimal128(38, 10) by default, `type_source` Default; its values have no fraction, so `round` never changes one; above 28 integer digits `E-NUMBER-OVERFLOW` under every policy (5.2a guard 1); exact by `decimal256(40,0)` | not in the fixtures |
| 2; `NUMBER` | `number_default`, default Decimal128(38, 10); `decimal256(p,s)`, `float64`, `string` by option | values that do not fit: `on_number_overflow`, default `round` (5.2a, owner 2026-10-16); computed quotients have 39 / 40 digits [Y1 v3.4] | same |
| 2; `FLOAT(b)` (FLOAT = `FLOAT(126)`, REAL = `FLOAT(63)` [T9]) | Float64 | nearest double (5.2); the rounding is the declared conversion, written into the field metadata (`oradmp:conversion` = `nearest-double`), never counted as overflow; exact by override `decimal(p,s)` or `string` | same |
| 2; decl. unknown (5.5 fallback) | as `NUMBER` | `W-DDL-FALLBACK` | same |
| 12; `DATE` | Timestamp(Second, no zone) | year below 1: `E-BAD-DATETIME` (5.3, ADR-28) | confirmed [Y2] |
| 180; `TIMESTAMP (f)` | Timestamp(Microsecond) for f <= 6, Timestamp(Nanosecond) for f 7 to 9; f unknown: Microsecond | exact or value error; 7 or 11 bytes | confirmed [Y5] |
| 181; `TIMESTAMP (f) WITH TIME ZONE` | Timestamp(unit as above, "UTC") | the instant; the zone of the value is **not** delivered (ADR-7) | instant confirmed, region zone bytes unknown [Y7] |
| 231; `TIMESTAMP (f) WITH LOCAL TIME ZONE` | Timestamp(unit as above, no zone), "as stored" | `W-TSLTZ`: UTC or database time zone is unknown (R-019); not labelled UTC | bytes confirmed, meaning unknown [Y8] |
| 1, form 1; `VARCHAR2(n)`, `VARCHAR2(n CHAR)` | Utf8 | database set of the descriptor [C1]; descriptor length of `n CHAR` = min(4n, 4000) in AL32UTF8 [C6] | confirmed |
| 96, form 1; `CHAR(n)` | Utf8 | padded with 0x20 to n bytes [C1]; kept, `trim_char` removes trailing U+0020 | confirmed |
| 96, form 1; `CHAR(n CHAR)` | Utf8 | padded with 0x20 to n **code points** (U+1F600 counts 1): n up to the cap of bytes; descriptor length min(4n, 2000) in AL32UTF8 [C6]; padding kept, `trim_char` as for CHAR | confirmed [C6] (w for sets other than 873 / 171: assumed) |
| 1, set 2000, form 2; `NVARCHAR2(n)` | Utf8 | UTF-16 big-endian with surrogate pairs, no padding [C3, C5] | confirmed |
| 96, set 2000, form 2; `NCHAR(n)` | Utf8 | UTF-16 big-endian with surrogate pairs, padded with `00 20` to n UTF-16 **code units** (a non-BMP character counts 2; `NCHAR(3)` holding `a` + U+1F600 has no padding) [C5]; kept, `trim_char` as for CHAR | confirmed (one non-BMP character, 2 versions) |
| 23; `RAW(n)` | Binary | unchanged bytes [Y6]; longer than the descriptor: `W-LONGER-THAN-DECLARED` (ADR-26) | confirmed |
| 8; `LONG` | **LargeUtf8** (session 4) | conventional path: one field [Y9]; direct path: a piece chain [R2], the column last in the part [T3]; decoded with the **database set of the header** [H4] (confirmed for AL32UTF8 [Y9]); `W-LONG-CHARSET` once per column for a byte above 0x7f only when the database set is not AL32UTF8; values above 100,000 bytes: `W-LONG-LENGTH` (session 24); conventional path (session 25, ADR-99): one field up to 65,529 bytes (65,522 to 65,524 with `W-UNTESTED-LAYOUT`), `fa ff` + 4-byte length for 65,530 to 65,533, the `fd ff` chunked form from 65,534; `fb ff` / `fc ff` `UnsupportedLayout` | confirmed up to 100,000 bytes on both paths, 2 versions, every size from 65,525 to 65,535 [Y9, R2 v3.9]; 65,522 to 65,524: hypothesis (R-052) |
| 24; `LONG RAW` | **LargeBinary** (session 4) | unchanged bytes [Y9]; direct path a chain as LONG; conventional path the three forms of LONG (session 25, ADR-99); `W-LONG-LENGTH` above 100,000 bytes | confirmed up to 100,000 bytes on both paths, 2 versions (LONG RAW = LONG, R-051) |
| 112, form 1; `CLOB` | **LargeUtf8** | LOB value after the row [L3, L4]; set by the descriptor id [L5]: 873 = **UTF-16 BE** with surrogate pairs (not UTF-8); 171 / 178 / 1 = that single-byte set for the **short** form; session 8: in a database of set 171 with descriptor set 171 also the **long** form [L5 v3.5]; a long-form value of any other single-byte database: `E-LOB-UNCONFIRMED` (ADR-25); other id: LargeBinary + `W-CHARSET-UNSUPPORTED` (5.6, ADR-18) | framing confirmed [L1 to L4]; text confirmed for ASCII, non-ASCII and non-BMP text in AL32UTF8 and Cyrillic values in CL8MSWIN1251, short and (session 8) long, 11g [L5 v3.5] |
| 112, set 2000, form 2; `NCLOB` | **LargeUtf8** | UTF-16 BE with surrogate pairs [L5]; in a database of set 1, 178 or any set other than 873 / 171: `E-LOB-UNCONFIRMED` (ADR-25 session 8) | confirmed (AL32UTF8 database, short form; CL8MSWIN1251 database, 11g, short and long [L4, L5 v3.5]) |
| 113; `BLOB` | **LargeBinary** | the bytes [L5] | confirmed |
| 182; `INTERVAL YEAR (p) TO MONTH ` | Interval(MonthDayNano), months only | 12 x years + months; exact or `E-BAD-INTERVAL` (5.7, ADR-19); beyond 2^31 - 1 months (p = 9 only) `E-BAD-INTERVAL`, kept by ADR-28 | confirmed for 3 values [Y10] |
| 183; `INTERVAL DAY (p) TO SECOND (f)` | Duration(Microsecond) for f <= 6 and p <= 8; Duration(Nanosecond) for f 7 to 9 and p <= 5; otherwise Interval(MonthDayNano) (days, nanoseconds of the day); p, f unknown: Microsecond | signed sum of the fields; exact (or `E-BAD-INTERVAL` on a fallback column); opposite signs: `E-BAD-INTERVAL` (5.7, ADR-19 session 4) | confirmed [Y10]: 12 values, precisions 9 / 9 and 2 / 0 |
| 69; `ROWID` | Utf8 | the last 18 bytes of the 23-byte field [Y11]; other length: `E-BAD-ROWID`; other 5-byte prefix: `W-UNVERIFIED-TYPE` (5.7, ADR-20) | confirmed (8 values, 2 databases) |
| 208; `UROWID(n)` | Utf8 | first byte `01`, 13 bytes: the 18-character text; first byte `02` (logical, index-organised table): `*` + base 64 of the rest when its length is a multiple of 3; other: `E-BAD-ROWID` (ADR-20 session 4) | physical text confirmed (6 values); logical text confirmed (1 value per version) [Y11] |
| every code outside the 16 of [T5] (BFILE 114, object, XML, ...) | none: `UnsupportedType` | stops the file [X1], ADR-12 | unknown |
| BINARY_FLOAT, BINARY_DOUBLE | cannot occur | `exp` refuses such tables (rules file, chapter 7) | - |

Every field is nullable (`fe ff` is NULL for every type [R3]); `NOT NULL` of the DDL is reported in
`ColumnInfo.declared`, not used for the schema. Field metadata (Arrow schema key / value strings):
`oradmp:type_code`, `oradmp:oracle_type`, `oradmp:declared` (the [T9] type text), `oradmp:max_bytes`,
`oradmp:charset`, and for NUMBER columns `oradmp:number_overflow` (the policy in force). `Utf8` and
`Binary` have 32-bit offsets (Arrow columnar format): `batch_bytes` keeps a batch far below 2 GiB per
column; every such column field is at most 65,519 bytes (4.1). LOB and (session 4) LONG / LONG RAW
columns use the 64-bit-offset types (`LargeUtf8`, `LargeBinary`) because a single value is not bounded
by the format (ADR-18, ADR-15); interval
columns write `oradmp:declared` with p and f; LOB text columns carry `oradmp:charset` with the set actually
used (`AL16UTF16` for a CLOB with id 873 and for every NCLOB). Session 14 (ADR-60, N-3): there is no
`oradmp:lob_charset` key; the earlier text named one that no code wrote.

5.2 **NUMBER decoder** (rule [Y1], confirmed: 85,456 NUMBER fields of 199 files decode and re-encode to
the same bytes, 120 truth values, the 14 vectors of R-024, the 6 computed values of R-032 on 2 versions).

1. **Input**: the value bytes b[0..L) of one field, L from the length field. L outside 1 to 21 is a
   value error (`E-BAD-NUMBER`, offset of the length field).
2. **Validation**, exactly the checks of [Y1], each a value error at the offset of the value: L = 1
   and b[0] != `80`; positive (bit 0x80 of b[0] set): a digit byte outside `01` to `64`; negative: if
   the last byte is `66` drop it, otherwise L must be 21; a digit byte outside `02` to `65`; no digit
   byte left; first or last base-100 digit 0.
3. **Intermediate** (lossless, fixed size, no heap): `OraNumber { negative: bool, coeff: [u8; 40]
   (decimal digits 0 to 9), len: u8 (1 to 40), exp10: i16 }`, value = (-1)^negative x coeff x
   10^exp10, plus a zero flag. From the base-100 digits d[0..n) and E of [Y1]: write each d[i] as two
   decimal digits, remove the leading 0 of d[0] (if d[0] < 10) and the trailing 0 of d[n-1] (if
   d[n-1] is a multiple of 10); exp10 = 2 x (E - n + 1), plus 1 if a trailing 0 was removed. Derived
   counts used below: significant digits `sig` = len (1 to 40), fractional digits `frac` = max(0,
   -exp10), integer digits `int` = max(0, len + exp10). Examples: `80 02` (E = -65, d = 1) is coeff 1,
   exp10 -130; `ff 0b` (E = 62, d = 10) is coeff 1, exp10 +125; `c0 33` is coeff 5, exp10 -1. The
   bytes allow exp10 from -168 (E = -65 with 20 digits) to +125; i16 holds every value.
4. **Renderings** (pure functions of the intermediate):
   - *Int64*: fits iff exp10 >= 0 and int <= 19 and the value lies in i64 (compared digit-wise
     against 9223372036854775807 / 9223372036854775808); `truth_num2.txt` rows 10 and 12 fit, row 11
     does not.
   - *Decimal128(P, S)*: let k = exp10 + S. If k >= 0, the unscaled value is coeff x 10^k and fits iff
     len + k <= P. If k < 0, -k digits lie below the scale: the value is **not exact**; what happens is
     the overflow policy (5.2a). Rounding, when the policy allows it, works on the digit array (half
     away from zero, carry included), then the same fit test. An i128 is built only after the fit test
     (at most 38 digits); a 39- or 40-digit coefficient never enters integer arithmetic.
   - *Decimal256(P, S)* (session 4, `decimal256(p,s)`, ADR-14): the same test with P up to 76; the
     unscaled value (at most 76 digits, below 2^255) is built as `arrow_buffer::i256` only after the fit
     test; the column builder is `arrow_array::builder::Decimal256Builder` (crate documentation; T-4
     confirms both names on the pinned version).
   - *Float64*: the text `[-]<coeff>e<exp10>` parsed with `str::parse::<f64>` (Rust std, `impl FromStr
     for f64`: "the closest representable floating-point number"). Every NUMBER of [Y1] (up to about
     1E+126, down to 1E-130) lies inside the normal range of f64: no infinity, no subnormal.
   - *Text*: plain decimal notation with `.` and a leading `0` for |v| < 1 (`0.001`, `-1000000`) when
     that text has at most 64 characters, otherwise `<d>.<ddd>E<+|-><n>` (`1E+125`, `1E-130`). It is
     lossless. It is not Oracle's `TO_CHAR(v, 'TM9')` (which writes `.001`): tests compare the decimal
     value, not the text.
5. **Tests** (T-4): every non-NULL value of `truth_num.txt` (32), `truth_num2.txt` (14), `truth_num3.txt`
   (6, session 4) and the two DUMP
   columns of `truth_numtypes.txt`, both folders, decodes to the truth value (compared as exact
   decimals); every NUMBER field of the 199 files decodes without a validity error, and a test-only
   encoder gives the same bytes back (the Analyst's `tools/num_decode.py` check, repeated in Rust);
   proptest round trips on generated digit strings (8.3 c).

5.2a **Values that do not fit the column type** (session 4: rewritten with [Y1] v3.4; session 5:
**decided** by the owner on 2026-10-16: Decimal128(38, 10) with `round`, the recommendation below, and
with `strict=True` a value `round` would change is `E-NUMBER-OVERFLOW`, ADR-14).

*Facts.* [Y1] v3.4 (R-032, `t_num3_exp_<v>`, 18c and 11g byte-identical): a quotient or an average that
does not terminate is stored with the full 20 digit bytes, 40 significant decimal digits (39 when the
first digit byte is below 10: 5/3 = `c1 02` + 18 x `43` + `44`). A column declared `NUMBER` holds such
values as ordinary data; it also holds 38-digit integers and fractions and the exponents +125 / -130.
Decimal128(38, 10) holds |v| < 10^28 with at most 10 fractional digits; Decimal256(76, s) holds
|v| < 10^(76 - s) with at most s fractional digits (`arrow_schema::DECIMAL256_MAX_PRECISION` = 76); no
decimal type holds both a 38-digit integer and a 40-digit fraction (38 + 40 = 78 > 76), and none holds
1E+125 or 1E-130 exactly. Counts below: session 4 arithmetic on the 52 non-NULL truth values
(`truth_num` 32, `truth_num2` 14, `truth_num3` 6), each decoded by [Y1] and classified by significant
digits, fractional digits and integer digits (Analyst tools `num_decode.decode`, `exp_parse`, `collist`,
run read-only; the 18c and 11g truth files are identical).

*Options* for a column declared `NUMBER` (the policy applies only where the column type is **not** fixed
by a declared precision: `NUMBER`, the type-2 fallback, and per-column overrides; a declared
`NUMBER(p, s)` column stays exact or value error; session 6: decided by `ColumnInfo.type_source`
`Default` / `Override`, never by comparing the column's Arrow type with `number_default`, ADR-33 item 4):

| type + policy | exact | changed by `round` (of which to 0) | integer part too large: value error under every policy | bytes per value | consumers (documentation, 2026-10-15) |
|---|---|---|---|---|---|
| Decimal128(38, 10) + `error` | 30 | none; the 12 values with more than 10 fractional digits are value errors: **all 6 computed values** stop the read under `on_value_error="raise"` | 10 (rows 19 to 24, 26, 28, 29, 32 of `truth_num`) | 16 | Parquet via pyarrow (DECIMAL <-> Decimal128, Arrow C++ Parquet docs); DuckDB `DECIMAL` up to width 38 (DuckDB docs); Polars `Decimal`, 128-bit, precision up to 38 (Polars docs); pandas: object column of `decimal.Decimal` expected, **not verified** |
| Decimal128(38, 10) + `round` | 30 | 12 (3): every computed value loses digits 11 to 40 (change below 5E-11); 1E-11, 1E-125, 1E-130 become 0 | 10 | 16 | as above |
| Decimal128(38, 10) + `null` | 30 | none; the 12 become NULL (counted) | 10 | 16 | as above |
| Decimal256(76, 38) + `round` | 42 | 7 (2): the 40-digit values (4 computed, 1 literal) lose digits 39 and 40 (change below 5E-39); 1E-125, 1E-130 become 0; 5/3, 100/3, pi and every 38-digit integer are exact | 3 (1E+125, 9.99...E+125, -1E+125) | 32 | Parquet via pyarrow: DECIMAL <-> Decimal256 (Arrow C++ Parquet docs); **Polars and DuckDB: native decimals stop at 38 digits; what they do on import of an Arrow Decimal256 (error, float, narrowing) is not verified**; pandas: not verified |
| Decimal256(76, 40) + `round` | 42 | 2 (2): only 1E-125, 1E-130 | 8 (the five 37- and 38-digit integers join) | 32 | as above |
| Utf8 text (`string`) | 52 | none | 0 | text bytes + 4 offset bytes (46 for 1/3) | every consumer; no arithmetic until the consumer casts |
| Float64 | - | nearest double: the 20 values with more than 15 significant digits change, and so does every decimal fraction that is not a binary fraction (0.1) | 0 | 8 | every consumer; ids above 2^53 change silently |

Over the 199 files the 2,230 non-NULL fields of the 11 columns declared `NUMBER` give, exact / changed by
`round` (to 0) / integer part too large: Decimal128(38, 10) 2,102 / 48 (12) / 80; Decimal256(76, 38)
2,180 / 26 (10) / 24; Decimal256(76, 40) 2,156 / 10 (10) / 64. The files repeat the same tables, so the
truth counts above are the meaningful ones; real data differ, which is why the survey (5.2b) prints the
same classification per column.

*Recommendation* (accepted by the owner, 2026-10-16): keep **Decimal128(38, 10)** as the default type (owner's
decision of 2026-10-08) with **`on_number_overflow="round"`** as the default policy, and offer
`decimal256(p,s)` and `string` per column (`number_types`) and dump-wide (`number_default`). Guards: (1)
only fractional digits are ever rounded; an integer part that does not fit is a value error under every
policy; (2) rounding is half away from zero, which is what Oracle's `ROUND(n, 10)` gives for positive n
(Oracle 18c SQL Language Reference, ROUND (number)) and its mirror for negative n, so `ROUND(v, 10)` in
the source database gives equal values; (3) every rounded value is counted per column:
`W-NUMBER-ROUNDED` (table, column, count, how many became 0, the largest number of digits dropped, row
number and offset of the first one), one warning per column at the end of the table, in
`TableBatches.warnings()` and the survey (session 6, ADR-33 item 3: not in `Dump.warnings`, which holds
the scan's warnings; the per-column policy and the digits figure are built in T-11); (4) the field metadata
`oradmp:number_overflow` says `round` and the scale; (5) `error` and `null` remain, globally and per
column. CI and the truth tests run with `error`. Why this default, given that computed values are now
known to be ordinary: (a) `error` stops a nightly job on the first quotient; (b) Decimal256 as the
default would fix the type of **every** unconstrained NUMBER column, ids and amounts included, while
two of the four named consumers have no native decimal above 38 digits and their import of Decimal256 is
unverified: a type the consumer refuses fails the whole column, a rounded 11th decimal changes counted
values; (c) text as the default turns every id and amount into a string. The cost of the
recommendation, stated plainly: every computed quotient in a `NUMBER` column arrives rounded to 10
decimals unless the owner overrides that column; the survey names those columns before the first run,
and `decimal256(76,38)` keeps 38 decimals for them where the consumer accepts it.

5.2b **What the survey prints for NUMBER columns** (T-12; counts only, never a value, a minimum or a
maximum value): per column of type 2 (name hidden with `--no-names`): the [T9] type text and the
resolved Arrow type; non-NULL and NULL counts; count of integer values and count that fit Int64; then,
for **each candidate** Decimal128(38, 10), Decimal256(76, 38) and the resolved type if it is another:
count exact, count that `round` would change, of those how many become 0, and count whose integer part
does not fit; count with 39 or 40 significant digits (the computed-quotient signature [Y1]); largest
`sig`, largest `frac`, largest `int` (5.2 step 3); smallest and largest exp10 and smallest and largest
position of the leading digit (exp10 + len - 1). A summary line over all `NUMBER` columns gives the same
counts in total and the number of columns that would need an override under each candidate. From these
the owner chooses `number_default`, `on_number_overflow` and the per-column overrides before the first
run on a real dump.

5.3 **Calendar.** The fields year, month, day, hour, minute, second of [Y2] are taken as civil fields of
the proleptic Gregorian calendar and converted to the Arrow epoch value: what Oracle prints is what
Arrow prints. A year below 1 (`truth_date.txt` rows 8, 9) decodes to fields (confirmed) but its Arrow
value needs a statement about year numbering that no fixture gives: `ValueDecodeError`
(`on_value_error` applies). A nanosecond timestamp outside the 64-bit range (about 1677 to 2262;
`truth_ts.txt` row 5) is the same error. Session 5 (ADR-27, ADR-28): the code is `E-BAD-DATETIME`
(Python class `ValueDecodeError`); years below 1 **stay** an error for DATE, TIMESTAMP, TSTZ and TSLTZ
(the year numbering is unknown, R-044, and Python `datetime` holds years 1 to 9999 only); the byte-pair
rule of [Y2] is ADR-27 item 6. Dates before 1583 are delivered by their printed fields; whether Oracle
counts them in another calendar is not in the rules (R-044), so the README calls them "as printed", not
an instant. T-12's pandas / pyarrow checks: ADR-28 (b).

5.4 **Sanity checks that cost nothing**: DATE is 7 bytes; TIMESTAMP 7 or 11; TSTZ 13; TSLTZ 11; NUMBER 1
to 21: any other length is a value error (session 25: a TSLTZ of 7 bytes is the zero-fraction form, confirmed [Y8]
v3.9, R-050; read as such without a warning, ADR-100; before: `W-UNVERIFIED-TYPE`).

- **Longer than declared** (session 4: rule [C8], ADR-26). For types 1, 23 and 96 the descriptor length
  is a true maximum; a field longer than it gives `W-LONGER-THAN-DECLARED` (once per column, offset of
  the first; strict: error) and the value is delivered as read (the length field alone decides how many
  bytes are read [R2]). RAW is in this group now (was a value error). It compares the **byte** length
  with the **descriptor** length, never with the declared n: the descriptor is min(4n, 4000) for
  `VARCHAR2(n CHAR)` and min(4n, 2000) for `CHAR(n CHAR)` in AL32UTF8 [C6], so correct data never fire
  it. Not checked: types 8 / 24 (descriptor 0), 112 / 113 (descriptor 72; the field is a locator of 102
  to 1,102 bytes), and padding to exactly n (`CHAR(n CHAR)` pads to n code points [C6], NCHAR to n UTF-16
  code units [C5]; no rule needs the check). The fixed-length checks of the first paragraph stay value
  errors.
- **LONG and LONG RAW** (descriptor length 0 [T5]; session 4 [R2, Y9], ADR-15; session 24, ADR-95; **session 25,
  ADR-99**): conventional path, LONG and LONG RAW alike: one field for a u16 1 to 0xfff9 (bit 0x8000 part of the length;
  0xfff2 to 0xfff4 with `W-UNTESTED-LAYOUT`, R-052); `fa ff` + u16 n + `00 00` + n bytes for n 0xfffa to 0xfffd (else
  `Framing`); the chunked form after `fd ff` (chunks `u16 n + n bytes` up to `00 00`, delivered to the sink as pieces; a
  chunked total below 65,534 decodes with `W-UNTESTED-LAYOUT`; a chunk u16 from 0xfff0 `Framing`); `fb ff` / `fc ff`
  `UnsupportedLayout`. Direct path: a piece chain (2.5 step 5). `W-LONG-LENGTH` once per column for a LONG or LONG RAW
  above 100,000 bytes (largest confirmed); `LongStats.sizes_65521_to_65534` counts the values the survey reports for Q-4;
  `W-LONG-DIRECT` is retired by T-3c. `max_lob_bytes` bounds every form as it accumulates; `LongStats.chunked` counts the
  chunked values.
- **Lengths that never occur** [R8]: a length field of 0 or of 0xfff0 to 0xfffd in a **column field**
  (LOB locators included) is `Framing` at its offset. Lengths from 32,001 to 0xffef are not errors in
  themselves [R2, R8]; in a conventional LONG column 0xfff0 is a length and `fd ff` the chunked form, and a conventional
  LONG RAW column stops from 0x8000 (above; session 24). In a direct-path LONG chain
  a u16 with bit 0x8000 is a piece length [R8]. The LOB value area after a row is not a column field:
  there 0 is an empty LOB and `fc ff` starts a long value (5.6).
- **Empty table on the direct path**: after flag 1, `ff ff` is [E1]; otherwise the rows are empty only if
  **both** tests of [E2] hold: (a) the byte at p is LF and an item starts at p + 1; (b) the u16 at p is
  larger than the maximum length of column 1. If exactly one holds, `Framing` ("ambiguous empty
  table") at p [E3] (both hold for every empty direct-path part of the 199 files, 17 of them; the
  Implementer built this in T-1, N-5, with test (a) as "the second byte is an upper-case letter", which
  accepts the five partition lines, ADR-21; [E2] saw `SUBPARTITION "` and `ENDCOMPARTITION` there). If
  column 1 has descriptor length 0, test (b) says nothing; by [T3] v3.4 this happens only in a table
  whose **only** column is LONG / LONG RAW (the LONG column moves last on the direct path; not in the
  fixtures, R-041). Then [E2a] (ADR-22): "no rows" when test (a) holds **and** the line from p + 1 to the
  next LF is a complete item of a known kind (`TABLE "`, `ENDTABLE`, `METRICS`, `CREATE `, `ALTER `,
  `COMMENT ON `, `GRANT `, `ANALSTATS `, `CALTER TABLE`, and the five partition lines) whose field, if
  list K requires one, is well-formed; then `W-UNTESTED-LAYOUT` at p. Otherwise rows are read.
  `strict=True` keeps the session 2 rule (test (a) holds: `Framing` "ambiguous empty table"). A LOB
  table never reaches this test: its path flag is 0 [L6], and flag 1 is `UnsupportedLayout` [X5]. The
  version line is never consulted [E3] (session 7: tested, `t8_direct.rs`
  `the_empty_table_decision_does_not_use_the_version_line`; the ambiguous case by
  `a_synthetic_ambiguous_empty_table_is_framing`, ADR-36). Every part of a partitioned table takes this decision on its
  own [T10].

5.5 **Declared types** (rule [T9], confirmed for 382 lines and 1,885 column definitions, `tools/collist.py`; ADR-6, ADR-24).

1. Input: the `CREATE TABLE` line of the section [T1], bytes in the client set [C2]. Grammar exactly as
   [T9] / exp.md 4.7: `CREATE TABLE ` qname ` (` coldef (`, ` coldef)* `)` + two spaces + storage text;
   coldef = qname + one space + type + optional ` NOT NULL ENABLE`; a qname runs from `"` to the next
   `"`. The parser never splits on commas or parentheses outside this grammar (chapter 10 of the rules).
2. Type texts and what they give (`ColumnInfo.declared`): `NUMBER` (none, none); `NUMBER(p, s)` (p, s;
   s may be negative or above p; `NUMBER(p)` is written `NUMBER(p, 0)`); `NUMBER(*,s)` (none, s; no space
   after the comma); `FLOAT(b)` (binary precision b); `VARCHAR2(n)`, `VARCHAR2(n CHAR)`, `CHAR(n)`, `CHAR(n
   CHAR)` (n, byte or char semantics); `NVARCHAR2(n)`, `NCHAR(n)` (n characters); `RAW(n)`; `DATE`;
   `TIMESTAMP (f)` with optional ` WITH TIME ZONE` / ` WITH LOCAL TIME ZONE` (f; space before the
   parenthesis); `CLOB`, `LONG`, `LONG RAW`; session 3 [T9 v3.3]: `NCLOB`, `BLOB`, `ROWID`, `UROWID(n)`
   (n), `INTERVAL YEAR (p) TO MONTH ` (p; the type text **ends with a space**, so the separator after it
   reads ` , ` and a last column ends ` )`), `INTERVAL DAY (p) TO SECOND (f)` (p, f). Plus `not_null`.
   ` NOT NULL ENABLE` after an interval text is not in the fixtures (R-039): such a coldef is a column
   fallback until R-039 shows its form.
3. Agreement, checked per table: number of coldefs = descriptor count; each `INSERT INTO` name matches
   exactly one qname **by name**, byte for byte (session 4, ADR-24: on the direct path a LONG / LONG RAW
   column stands last in `INSERT INTO` but not in `CREATE TABLE` [T3, T9]; any other difference of order
   is read the same way with `W-UNTESTED-LAYOUT` once per table); for each matched pair the type family = the descriptor code (NUMBER and
   FLOAT: 2; VARCHAR2, NVARCHAR2: 1; CHAR, NCHAR: 96; RAW: 23; DATE: 12; TIMESTAMP: 180, 181, 231 by
   suffix; LONG: 8; LONG RAW: 24; CLOB, NCLOB: 112 (form 1, form 2); BLOB: 113; ROWID: 69; UROWID: 208;
   INTERVAL YEAR: 182; INTERVAL DAY: 183); and the length where the fixtures confirm it:
   `VARCHAR2(n)` and `CHAR(n)` = n, `NVARCHAR2(n)` and `NCHAR(n)` = 2n, `RAW(n)` = n, NUMBER and FLOAT
   = 22, DATE = 7, `TIMESTAMP (0)` = 7, `TIMESTAMP (f > 0)` = 11, CLOB / NCLOB / BLOB = 72, ROWID = 23,
   `UROWID(n)` = n, INTERVAL YEAR = 5, INTERVAL DAY = 11 [T5]; session 4 [C6]: `VARCHAR2(n CHAR)` =
   min(4n, 4000) and `CHAR(n CHAR)` = min(4n, 2000) when the descriptor set is 873, n (capped the same
   way) when it is 171; not checked for other sets (w assumed [C6]). The precisions of an interval are not in the descriptor and
   cannot be checked.
4. Fallback, never an error: the line does not parse, or the count or a name disagrees: the whole table
   is mapped from the descriptors alone (type 2 as `NUMBER`, TIMESTAMP unit Microsecond) with
   `W-DDL-FALLBACK` (table). A type text outside the list of step 2, or one whose family or length
   disagrees: that column alone is mapped from its descriptor, `declared` = unknown, the text is kept,
   `W-DDL-FALLBACK` (table, column). Session 5 (ADR-30 item 1): an unknown text ends at the first `, "` or
   the first `)  `, whichever comes first; a trailing ` NOT NULL ENABLE` sets `not_null`; a wrong cut
   changes the definition count or names and so gives the table fallback, never another column's type.
   Out-of-range parameters (`NUMBER` p outside 1..38 or s outside -84..127, `FLOAT(b)` outside 1..126,
   TIMESTAMP and interval precisions outside 0..9) are a column fallback; the descriptor form is part of
   the family check (form 1 for VARCHAR2, CHAR, CLOB; form 2 for NVARCHAR2, NCHAR, NCLOB). The values are decoded the same way in every case; only the Arrow
   type of NUMBER, TIMESTAMP and INTERVAL DAY TO SECOND columns depends on the declaration.
5. Column order (ADR-24): the schema and `TableInfo.columns` follow the coldef order; the decoder reads
   in file order and places each column by its match. On a table fallback both are the `INSERT INTO`
   order. Session 5 (ADR-31): the matching and the order rule run for **every part** of a table
   (`PartInfo.file_order`); the types are resolved once per table.

5.6 **LOB columns** (rules [L1 to L6], [T6], [R4]; ADR-12, ADR-18).

1. **Before the rows** [T6, L1]: after the descriptors, u16 m and the u16 path flag. m = 0: no LOB
   column, rows follow. m = column count: m column entries follow, one per column in order: u8 n, n
   name bytes, 2 bytes. Checks: m is 0 or the column count, and m > 0 exactly when a descriptor has code
   112 or 113; n > 0 exactly for the 112 / 113 columns; the flag is 0 when m > 0 [L6]. Any failure:
   `UnsupportedLayout` (stops the file, [X5]). Not errors but `W-UNTESTED-LAYOUT` (strict: error), once
   per table: the 2 bytes after an entry other than `00 00` (meaning unknown [L1]); entry name bytes
   different from the `INSERT INTO` name of that column (equal in the fixtures).
2. **In the row** [L2]: the field of a LOB column is `fe ff` (NULL) or a locator, an ordinary column
   field (102 to 1,102 bytes seen; [R8] applies) that is skipped by its length and never decoded; it
   differs between two exports of the same data.
3. **After the row** [R4, L3]: u16 k = the number of LOB columns of this row that are not NULL (k = 0 is
   the `00 00` row end). k must equal the count of locators in the row, else `Framing` at the k field.
   Then k values, the i-th belonging to the i-th LOB column with a locator, in column order.
4. **One value** [L4]: u16 h. h = 0: an **empty** LOB (non-NULL, length 0). 1 <= h <= 0xffef: short
   form, h bytes. h = 0xfffc (`fc ff`): long form: 8 bytes LE = total T (read as u64; the width is
   unknown [L4], so a T above 2^32 is read as stated but meets `max_lob_bytes` first), then chunks
   `u16 c` + c bytes until their sum is T, no terminator. Any other h, a chunk c of 0 or from 0xfff0,
   or a chunk that passes T: `Framing` at that u16. **Never assume a chunk size** (16,120 / 8,060 on 18c,
   16,264 / 8,132 on 11g, the last one the rest [L4]). Session 8 (ADR-37): these are observations of
   AL32UTF8 databases; the 11g CL8MSWIN1251 database has 8,132 for a CLOB and 16,264 for an NCLOB [L4 v3.5].
   The reader takes each c from the stream, checks it against nothing fixed and accepts any c from 1 to
   0xffef silently; only the [L4] errors above stop. Session 4: [L4] confirms the switch (long iff the
   byte length exceeds one chunk: 18c 16,120 / 8,060, 11g 16,264 / 8,132 bytes for CLOB / BLOB); the
   reader still accepts both forms for any length and never uses the switch to decide anything; tests
   use it (T-6L). The first two bytes of T can be `fc ff` (T = 65,532): T is read by position [L4]. A long
   value with T = 0 (no chunk): read, with `W-UNTESTED-LAYOUT` unless a fixture shows one (ADR-23, AQ-1).
   The stored length (h or T) is checked against `max_lob_bytes` before the first byte
   is read: above it, `E-LOB-TOO-LARGE` (value error). Session 7 (ADR-34 item 3): a value that is also
   unconfirmed (step 7) gives `E-LOB-UNCONFIRMED` instead, unless `unconfirmed_lob="decode"`.
5. **NULL against empty**: NULL = `fe ff` in the column and no value after the row = Arrow null; empty
   = a locator and a value with h = 0 = `""` (CLOB, NCLOB) or `b""` (BLOB), non-null [L4].
6. **Bytes to value** [L5], by the descriptor of the column:
   - BLOB (113): the bytes, appended chunk by chunk to the LargeBinary builder.
   - NCLOB (112, set 2000, form 2) and CLOB with set 873: **UTF-16 BE**. One decoder per value,
     `encoding_rs::UTF_16BE.new_decoder_without_bom_handling()`, fed every chunk in order with
     `last = false` and an empty input with `last = true` at the end, through the
     `_without_replacement` decode methods of `encoding_rs::Decoder` (verify the exact method names on
     the pinned version in T-6L), so that a code unit or a surrogate pair split between two chunks is
     decoded correctly. An odd total, an unpaired surrogate or any other malformed sequence: value error
     `E-BAD-TEXT`. The CLOB with set 873 is UTF-16 although the descriptor names AL32UTF8: the reader
     decides by the type code and the set id together, never by the set id alone.
     Session 4, **surrogate pairs** [C5, L5]: UTF-16 BE text of the corpus contains non-BMP characters
     as surrogate pairs; the WHATWG UTF-16BE decoder that `encoding_rs` implements combines a high and a
     following low surrogate into one scalar value and reports a lone surrogate as malformed (Encoding
     Standard, "shared UTF-16 decoder"). The same decoder serves NCHAR and NVARCHAR2 column values (one
     call per value, `Encoding::decode_without_bom_handling_and_without_replacement`, which returns None
     on malformed input; name to be confirmed on the pinned version). Test vectors (T-6L, T-9):
     `truth_nchar2.txt` row 1, NC `NCHAR(3)` = NV `NVARCHAR2(3)` = `00 61 d8 3d de 00` -> `a` + U+1F600,
     whose UTF-8 is `61 f0 9f 98 80` (the same bytes as the VC column's DUMP in that row); the CLOB and
     NCLOB of `nchar2_exp_<v>` give the same text; `truth_lob3.txt` row 20, C = `04 34 04 36 00 20 00 43
     00 61 00 66 00 e9 00 20 00 61 d8 3d de 00` (22 bytes) -> UTF-8 `d0 b4 d0 b6 20 43 61 66 c3 a9 20 61 f0
     9f 98 80` (16 bytes, = the DUMP(..., 1016) of the truth line; `GETLENGTH` 11 counts code units, the
     text has 10 characters). Negative vectors: `d8 3d` alone, `de 00 d8 3d`, `00 61 d8` (odd length):
     `E-BAD-TEXT`; the 22-byte value fed as chunks split between `d8 3d` | `de 00` and between `d8` |
     `3d de 00`: the same 16 UTF-8 bytes.
   - CLOB with a single-byte set (171, 178, 1): the decoder of 6.2, chunk by chunk (stateless); US7ASCII
     high bytes as ADR-16. Session 4: only the **short** form; a long-form value is ADR-25 (step 7).
     Session 8: the long form too when the header's database set is 171 and the descriptor set is 171
     [L5 v3.5].
   - CLOB with any other set id: LargeBinary column, `W-CHARSET-UNSUPPORTED` (6.4; [L5] "report it and
     return bytes"). The column type is fixed at the descriptor, before any value.
   - A value error under `on_value_error="null"` after part of a value was written: the builder appends
     a null; the bytes already written stay under that null slot, which the Arrow columnar format
     permits ("a null value may have a positive slot length"); they are valid UTF-8 because the decoder
     only emits complete characters.
7. **Values whose decoding is not confirmed** (session 4; ADR-25; counted in the survey):
   - `W-LOB-CHARSET` is **retired**: [L5] v3.4 confirms non-ASCII and non-BMP CLOB text in AL32UTF8.
   - Database set [H4] single-byte (1, 171, 178): a CLOB value in the **long** form, and any NCLOB value,
     is the value error **`E-LOB-UNCONFIRMED`** (column, row, offset of the value; `on_value_error`
     decides: stop, or NULL + `W-VALUE-NULLED`), because the fixtures show neither and both plausible
     readings (database bytes, UTF-16 BE) decode without error to different text. The chunks are passed
     over by their lengths; the framing is confirmed, so the next value, row and table read normally.
     With `unconfirmed_lob="decode"` the value is decoded by [L5] as stated (CLOB: the descriptor set;
     NCLOB: UTF-16 BE) with `W-LOB-UNTESTED` once per column (strict: error). Session 7 (ADR-34 items
     4, 5): `W-LOB-UNTESTED` is counted (once per column and table, `count`, in `TableBatches::warnings()`);
     under `strict=True` the value is `E-LOB-UNCONFIRMED` whatever `unconfirmed_lob` says. After R-038
     only `lob_text_kind`, this check and the tests of `t6l_ru.rs` change (ADR-34 item 9).
   - **Session 8 (ADR-25 session 8 blocks, task T-6M)**: the database sets whose LOB storage is confirmed are
     one list, `charset::LOB_CONFIRMED_DB_SETS = [873, 171]`, compared with the header's database set D [H4]
     (never with `db_charset`). An NCLOB is unconfirmed iff D is not in the list; a CLOB of a decodable
     single-byte or 873 descriptor set S is unconfirmed in its long form iff D is not in the list or S != D.
     Hence the refusal above now applies to database sets 1, 178 and any set outside the list; a
     CL8MSWIN1251 dump decodes all its CLOB and NCLOB values without warning.
   - Not affected: BLOB anywhere; short CLOB values in a single-byte database (confirmed [L5]); every
     CLOB and NCLOB of an AL32UTF8 database (both forms; the NCLOB long form is not in the fixtures, but
     its framing is that of [L4] and its text that of the short form).
8. **Batches** (`batch_bytes`, soft cap): the running count is the bytes appended to the builders' value
   buffers (the fixed width for primitive types, NULL included; the value bytes for variable-size types,
   output bytes after decoding). A batch closes after the row that brings the count to `batch_bytes` or
   beyond, or at `batch_rows`, or at the end of a part; a row and a value are never split. Example,
   T_LOB of `lob_exp_18c` (ID `NUMBER(3)` = Int64): rows 1 to 5 add 8 + 9 + 10 = 27, 8, 8, 8 + 3,000 +
   1,000 = 4,008 and 8 + 100,000 + 100,000 = 200,008 bytes (CLOB 100,000 ASCII characters = 200,000
   stored bytes = 100,000 UTF-8 bytes); with `batch_bytes` 4,000 the batches have 4 and 1 rows (after
   row 4 the count is 4,051); with the default 64 MiB, one batch of 5 rows; the 200,000-byte CLOB
   never needs more than its own value in memory beyond the batch.
9. **Scan** (ADR-8): walks L1 to L4 by lengths only, seeking over long values; it counts per LOB column
   the values, empty values, short and long forms and the largest stored length (`ColumnInfo.lob`), and
   (session 4) the values that step 7 would refuse; no LOB byte is read into memory.
10. **Not in phase 1**: the locator's content; LOB values on the direct path (never written so [L6]);
   BFILE [X1]; a partial read of a value.

5.7 **INTERVAL, ROWID, UROWID** (rules [Y10], [Y11]; ADR-19, ADR-20). All are ordinary column fields
[R2, R3]; NULL is `fe ff`.

1. INTERVAL YEAR TO MONTH (182): length must be 5. years = u32 BE b[0..4] - 0x80000000 (as i64),
   months = b[4] - 60; months in -11 to 11. Arrow: Interval(MonthDayNano) with months = 12 x years +
   months (must fit i32), days = 0, nanoseconds = 0. Example `7f ff ff fd 3a` = -3 years, -2 months =
   -38 months (`-0003-02`). Session 4: exact for p <= 8; with p = 9 a value beyond +-(2^31 - 1) months
   is `E-BAD-INTERVAL` (`misc2` rows 2 and 3: `bb 9a c9 ff 47` = 999,999,999 years 11 months). Session 5:
   kept (ADR-28 (a)); the survey counts such values; years and days beyond +-999,999,999 are
   `E-BAD-INTERVAL` too (ADR-27 item 4).
2. INTERVAL DAY TO SECOND (183): length must be 11. days = u32 BE b[0..4] - 0x80000000; hours, minutes,
   seconds = b[4], b[5], b[6] - 60; ns = u32 BE b[7..11] - 0x80000000; ranges as ADR-19. total ns =
   ((days x 24 + hours) x 60 + minutes) x 60 + seconds) x 10^9 + ns, in i128, then to the unit (exact or
   value error). Example `80 00 00 0a 48 5a 69 87 5b ca 00` = 10 d 12:30:45.123456 = 909,045,123,456
   microseconds. Session 4 [Y10]: every field of a negative interval is negative (zeros allowed);
   a value with one field above 0 and another below 0 is `E-BAD-INTERVAL`. The Arrow type follows p and
   f (ADR-19 session 4): Duration(Microsecond) if f <= 6 and p <= 8, Duration(Nanosecond) if f >= 7 and
   p <= 5, else Interval(MonthDayNano) with days = the day field and nanoseconds = the sum of the other
   four fields. Examples (`misc2`, p = f = 9, MonthDayNano): `7f ff ff ff 3a 39 38 62 32 9b 00` = days -1,
   nanoseconds -(2 x 3600 + 3 x 60 + 4.5) x 10^9 = -7,384,500,000,000; `bb 9a c9 ff 53 77 77 bb 9a c9 ff`
   = days 999,999,999, nanoseconds 86,399,999,999,999. (p = 2, f = 0, Microsecond): `80 00 00 00 3c 3c
   3b 80 00 00 00` = -1,000,000 microseconds.
3. ROWID (69): length 23; bytes 5 to 22 must be in `A-Z a-z 0-9 + /`; the text is those 18 bytes.
   Bytes 0 to 4 other than `01 00 00 00 17`: `W-UNVERIFIED-TYPE`.
4. UROWID (208): first byte `01` and length 13: object (u32 BE b[1..5]), file (u16 BE b[5..7]), block (u32
   BE b[7..11]), row (u16 BE b[11..13]), written in base 64 with 6, 3, 6, 3 characters, most significant
   first [Y11] (every u16 fits 3 and every u32 fits 6 base-64 characters). Example (18c): `01 00 01 1f 25
   00 0c 00 00 00 87 00 00` = `AAAR8lAAMAAAACHAAA` = the ROWID of the same row; no warning (session 4,
   6 values confirmed [Y11]). First byte `02` (logical UROWID, session 4): if (length - 1) is a multiple
   of 3, the text is `*` + base 64 of bytes 1 to the end, 3 bytes to 4 characters, most significant
   first: `02 04 03 00 15 b3 02 c1 02 fe` = `*BAMAFbMCwQL+`. Any other first byte or length, or a logical
   tail that is not a multiple of 3 bytes (unknown [Y11]): `E-BAD-ROWID`.
5. `interval_as="string"` (session 11: in the alpha, ADR-47; Utf8 for every type-182 and 183 column, metadata
   `oradmp:conversion` = `to-char-text`; ADR-28 (a)'s 32-bit-months limit does not apply): the `TO_CHAR` text of [Y10] with p and f of [T9]: `-` if
   any field is negative, else `+`; absolute fields; no `.` and no fraction when f = 0 (ADR-19 session 4
   item 3 has the vectors).

## 6. Character sets

6.1 **Three sets, chosen by field kind** [C1 to C4, exp.md 14.8]:

| what | set | where the reader finds it |
|---|---|---|
| VARCHAR2 and CHAR values (`CHAR(n CHAR)` included) | database | id at +4 of the column descriptor [C1, C6] |
| NVARCHAR2 and NCHAR values | national (AL16UTF16) | id at +4 of the descriptor (2000) [C3, C5] |
| LONG values | database ([Y9], confirmed for AL32UTF8 with 2 client sets) | **database id of the header** [H4]: the type 8 descriptor has no id. In a database whose set is not AL32UTF8 a byte above 0x7f gives `W-LONG-CHARSET` (once per column; strict: error; "assumed the same" [Y9]) |
| CLOB values | by the descriptor id, with a twist [L5] | id 873: **UTF-16 BE** with surrogate pairs, not AL32UTF8; single-byte id: that set, as C1, for the short form, and (session 8) for the long form when the header's database set is 171 = the descriptor id [L5 v3.5]; the long form in another single-byte database: ADR-25; 5.6 |
| NCLOB values | national (AL16UTF16) | id 2000 of the descriptor: UTF-16 BE [L5]; in a database of set 1, 178 or outside 873 / 171: ADR-25 (session 8) |
| table and column names, `CREATE TABLE` text, every other line and field | client | header bytes 1-2 [H1, C2] |
| partition and subpartition names (`PARTITION`, `COMPARTITION`, `SUBPARTITION` lines [T10]) | client | header bytes 1-2, like table names [C2] |
| RAW, LONG RAW, BLOB, NUMBER, dates, intervals | none | - |
| ROWID, UROWID | none (ASCII base-64 text [Y11]) | - |

The header's **client** id (bytes 1-2) is never used for row data and the descriptor never for text
items [C4]; the header's database id [H4] is used for LONG only, because nothing else names a set
for it. If a descriptor id
differs from the header's database set [H4] (equal in every fixture; exp.md 9, question 1c) the
descriptor wins and `W-CHARSET-MISMATCH` is raised. The form value [T5] is checked (1 or 2), not used.
Session 7: built for types 1 / 96 only; T-9 adds form-1 CLOB columns (type 112), e.g. a CLOB of set 873
in a single-byte header (ADR-34 item 6); NCHAR forms (set 2000) are never compared with [H4]. Session 9:
T-9 did not build the CLOB case (N-25); task T-9b adds type 112 to the same condition (ADR-41 item 5).

6.2 **Supported** (ids of [X3]) and the decoder of each (ADR-5):

| id | Oracle name | decoder |
|---|---|---|
| 1 | US7ASCII | own: bytes below 0x80 as themselves; a higher byte: 6.3 (`encoding_rs` has no ASCII-only decoder: US-ASCII is not in the WHATWG Encoding Standard it implements) |
| 171 | CL8MSWIN1251 | `encoding_rs::WINDOWS_1251` |
| 178 | WE8MSWIN1252 | `encoding_rs::WINDOWS_1252` |
| 873 | AL32UTF8 | validation by `std::str::from_utf8`, no copy through a converter |
| 2000 | AL16UTF16 | `encoding_rs::UTF_16BE` |

**Oracle's map against `encoding_rs`** (session 2). Oracle maps each of the 256 bytes of CL8MSWIN1251
and WE8MSWIN1252 to the Windows code page, and the bytes the code page leaves undefined (1251: 0x98;
1252: 0x81, 0x8D, 0x8F, 0x90, 0x9D) to the C1 control of the same number [C7, `truth_csmap.txt`].
What `encoding_rs` returns, from its documentation and the standard it names: the crate "is a ...
implementation of the Encoding Standard" (crate root, encoding_rs 0.8.42 on docs.rs), and the pages of
`WINDOWS_1251` / `WINDOWS_1252` link the WHATWG index of each. The WHATWG indexes
(`index-windows-1251.txt`, `index-windows-1252.txt`, encoding.spec.whatwg.org, read 2026-10-14) have an
entry for all 128 pointers; pointer 24 (byte 0x98) of windows-1251 is `0x0098`, pointers 1, 13, 15, 16,
29 (bytes 0x81, 0x8D, 0x8F, 0x90, 0x9D) of windows-1252 are `0x0081`, `0x008D`, `0x008F`, `0x0090`,
`0x009D`. So the crate is **expected** to give exactly Oracle's C1 controls and never to report an
error (`had_errors`) for these two sets. Expected, not assumed: the test of T-9 decodes every byte 0x00 to
0xFF with both decoders and compares the result with the code unit of each of the 512 lines of
`truth_csmap.txt` for the two sets, and the US7ASCII decoder with the 256 others (768 lines). One
differing byte for 171 or 178 replaces that crate decoder by an own 256-entry table (ADR-5); the API
does not change.

6.3 **Malformed text and US7ASCII high bytes.** One rule: **row data never get a U+FFFD the file does
not hold; metadata text never stops the scan.**

- Row data: a byte sequence the set does not allow (UTF-8, UTF-16, and every byte above 0x7f in a
  US7ASCII column) is a value error; `on_value_error="raise"` stops, `"null"` writes NULL and records
  `W-VALUE-NULLED` with table, column, row, offset. Reason for US7ASCII: Oracle itself turns such a byte
  into U+FFFD [C7], but the file still holds the original byte [C1]; delivering U+FFFD would destroy
  information the file has. A database declared US7ASCII that holds high bytes usually holds text of
  another set: the error message names `db_charset` (6.4), with which the caller reads the bytes as,
  for example, CL8MSWIN1251. Row data in a US7ASCII database are not in the fixtures [C7].
- Names, `CREATE TABLE` text and other text items (client set): a byte the set does not allow becomes
  U+FFFD, the raw bytes are kept (`name_raw`, `ddl_raw`) and the item is flagged
  (`name_possibly_damaged`, `ddl_lossy`), as in 6.5. For US7ASCII this is what Oracle does [C7].
- CL8MSWIN1251 and WE8MSWIN1252 have no malformed byte (above).

6.4 **A set that is not supported** [X3: "report it and return bytes"]: a column in that set becomes
`Binary` with `W-CHARSET-UNSUPPORTED` (strict: `UnsupportedCharset`). Names and DDL in an unsupported
client set are returned with every byte above 0x7f as U+FFFD, `name_raw` holds the bytes. Overrides:
`db_charset="..."` / `client_charset="..."` take one of the five Oracle names above or any label that
`encoding_rs::Encoding::for_label` knows; an override is recorded in `DumpInfo` and is the caller's
responsibility.

6.5 **Text lost in the dump.** When the client set lacks a character, `exp` wrote `?` (or 0xbf with
WE8MSWIN1252) into names, comments and DDL; the original is not in the file [C2, exp.md 14.2, 14.10]. The
reader returns the text as it stands and marks it: `DumpInfo.text_converted_at_export` (client id differs
from database id), `TableInfo.name_possibly_damaged` and `ddl_lossy` (that, and a `?` or 0xbf is present).
It never repairs or blanks. A table with such a name and 0 rows gets `possibly_incomplete` and
`W-POSSIBLY-INCOMPLETE`: the file cannot tell a failed export from an empty table [X2, exp.md 14.3]. Row
data are not affected [C1].

## 7. Errors and strictness

7.1 **Three classes.**

| class | examples | default | `strict=True` |
|---|---|---|---|
| fatal | everything in 1.4; a missing `00 00` after a conventional row [R4]; column count that differs from the `INSERT INTO` line [T3, T4] | error; the file position is lost, the reader stops and never resynchronises | same |
| outside the confirmed rules | version line not V18.00.00 / V11.02.00 (`W-UNTESTED-VERSION`, chapter 10, R-022); `W-UNTESTED-HEADER`: mode line other than `RTABLES` / `RUSERS`, RECORDLENGTH not 2048 / 4096 / 8192 / 16384, and (session 2, N-5) the byte after the file-name length not `00` [H4], the three lines after RECORDLENGTH not `0`, `72`, `0` [H3], first item `METRICST` with mode `RUSERS` or `METRICSU` with `RTABLES` [H6]; unknown introducing line read by G1 (counted in `Catalog.item_counts`, owner Q-2); TSLTZ; LONG: `W-LONG-CHARSET`, `W-LONG-LENGTH`, `W-LONG-DIRECT` (5.4); unsupported set; DDL fallback; file size not a multiple of R (`W-FILE-SIZE`) [H5]; possibly incomplete table; session 3: `W-LOB-CHARSET`, `W-LOB-UNTESTED` (5.6); `W-UNTESTED-LAYOUT` (L1 entry bytes or names, partition lines without the confirmed order, the [E2a] "no rows" decision); `W-UNVERIFIED-TYPE` (ROWID prefix, 5.7); session 4: `W-LOB-CHARSET` and `W-LONG-DIRECT` retired, `W-LOB-UNTESTED` only with `unconfirmed_lob="decode"`, `W-LONG-CHARSET` only outside AL32UTF8, `W-UNTESTED-LAYOUT` also for a LONG piece of length 0, a long LOB value with T = 0 (ADR-23), a column order other than [T3]'s (ADR-24); `W-LONGER-THAN-DECLARED` for types 1, 23, 96 (ADR-26) | warning, reading continues; every warning is returned in the result and printed by `survey` (owner Q-2) | error |
| value | bytes that are not a valid value of the column type; a value that does not fit the Arrow type of a declared column; a NUMBER that does not fit under `on_number_overflow="error"` or with an integer part too large (5.2a); session 3: `E-BAD-TEXT` (malformed UTF-16 in a LOB), `E-LOB-TOO-LARGE`, `E-BAD-INTERVAL`, `E-BAD-ROWID`; session 4: `E-LOB-UNCONFIRMED` (ADR-25), `E-BAD-INTERVAL` for opposite signs, `E-LOB-TOO-LARGE` for LONG chains | `on_value_error`: `raise` (default) or `null` + warning. The framing is intact (lengths are independent of content [R2]), so reading can continue | same option |
| counted change | a NUMBER rounded by `on_number_overflow="round"` (`W-NUMBER-ROUNDED`), set NULL by `"null"` (`W-NUMBER-NULLED`) | one warning per column with counts (5.2a) | session 5 (owner): a value `round` would change is `E-NUMBER-OVERFLOW` (value class); `null` unchanged |

Default non-strict: owner decision Q-2 (2026-10-08), with its refinements: a type that is not
supported is always an error (ADR-12); warnings are part of the result.

Codes added by the Implementer and accepted (N-5, N-7): `W-FILE-SIZE`, `E-CONFIG`, `E-FILE-CHANGED`,
`E-NO-SUCH-TABLE`, `E-AMBIGUOUS-TABLE-NAME`. Codes added in session 2: `E-BAD-NUMBER` (value error
class), `W-LONG-CHARSET`, `W-LONG-LENGTH`, `W-LONG-DIRECT`, `W-NUMBER-ROUNDED`, `W-NUMBER-NULLED`.
Removed: `W-ZERO-LENGTH` (now `Framing`, [R8]); `W-UNVERIFIED-TYPE` no longer applies to NCHAR [C5].
Codes added in session 3: `E-UNSUPPORTED-LAYOUT` (fatal, stops the file like `E-UNSUPPORTED-TYPE`),
`E-BAD-TEXT`, `E-LOB-TOO-LARGE`, `E-BAD-INTERVAL`, `E-BAD-ROWID` (value class), `W-LOB-CHARSET`,
`W-LOB-UNTESTED`, `W-UNTESTED-LAYOUT`; `W-UNVERIFIED-TYPE` reused for [Y10] mixed signs and [Y11].
The LOB framing checks of 1.4 (k, the value-area u16, chunks) are fatal `Framing`, never warnings.
Session 5 (ADR-27): value codes `E-NUMBER-OVERFLOW` (integer part too large under every policy; a
fraction beyond the scale under `error`, or under `round` with `strict`) and `E-BAD-DATETIME` (replaces the
name `ValueDecodeError` used as a code in 5.3 / 5.4); all value codes map to Python `ValueDecodeError`.
`W-UNVERIFIED-TYPE` is a `WarningCode` raised by the row decoder from the decoders' flags; T-6 adds the
value-layer warning codes it raises (ADR-27 item 2). New `W-UNTESTED-LAYOUT` cases: a section ending with
a `COMPARTITION` open, `COMPARTITION` lines without `ENDPARTITION` (ADR-29). `W-DDL-FALLBACK` also for a
declared `NUMBER` no Decimal128 holds (ADR-30). The walker's warnings respond to `strict` from T-11 on.
Session 4: new value code `E-LOB-UNCONFIRMED`; `UnsupportedLayout` also for a conventional LONG length
from 0x8000 (R-040; **session 24**: narrowed by ADR-95 to a LONG u16 0xfff1 to 0xfffc and a LONG RAW u16 from 0x8000,
R-048, with the messages of ADR-95 item 3; **session 25**: replaced by a field starting `fb ff` / `fc ff`, the messages of
ADR-99 item 3); `Framing` also for a LONG piece u16 from 0xfff0. Retired: `W-LOB-CHARSET` (L5
confirmed), `W-LONG-DIRECT` (from T-3c; R2 / Y9 confirmed). Narrowed: `W-LONG-CHARSET` (non-AL32UTF8
databases), `W-LOB-UNTESTED` (only with `unconfirmed_lob="decode"`), `W-UNVERIFIED-TYPE` (ROWID prefix
only; physical UROWID text and intervals no longer use it).

Session 7 (ADR-34, ADR-35): **`E-LOB-UNCONFIRMED`** (value class; offset = first u16 of the value,
`length` = stored length; Python `ValueDecodeError`; option `unconfirmed_lob`) and **`W-LOB-UNTESTED`**
(outside the confirmed rules; counted once per column and table; only under `unconfirmed_lob="decode"`;
`strict`: the value becomes `E-LOB-UNCONFIRMED`). A value both unconfirmed and above `max_lob_bytes`
reports `E-LOB-UNCONFIRMED`. **`W-TSLTZ`** (outside the confirmed rules, [Y8]): once per type-231 column
and table, in the scan's list, also after a fallback. **Order** of the resolution's warnings for one
table: table-level first, then per column `W-CHARSET-UNSUPPORTED`, `W-TSLTZ`, `W-DDL-FALLBACK`; only the
order within one column is promised.

Session 9 (ADR-41): **`E-STRICT`** (a warning outside the confirmed rules that has no error code of its own,
under `strict`; `Error.warning` names it; it stops the scan or the read of that table, the `Dump` stays
usable) and **`E-UNSUPPORTED-CHARSET`** (`W-CHARSET-UNSUPPORTED` under `strict`). Under `strict` the value-layer
warnings (`W-UNVERIFIED-TYPE`, `W-LONG-LENGTH`, `W-LONG-CHARSET`, `W-LONGER-THAN-DECLARED`) are `E-STRICT` at the
first such value whatever `on_value_error` says; only `E-NUMBER-OVERFLOW` (from `round`) and
`E-LOB-UNCONFIRMED` (from `W-LOB-UNTESTED`) stay value class. Python mapping in 3.4.

Session 14 (ADR-59): **`E-INTERNAL`** (a caught panic, 7.3; Python `InternalError(OradmpError)`); the core enum
has it so that one code list serves both languages, but only the binding constructs it. Not affected by `strict`
or `on_value_error`.

Session 11 (ADR-41 items 8, 9): **the outcome of every warning code under `strict`** (the reference the test table
`ARCH_7_1_STRICT` of `t11_strict.rs` follows):

| warning | under `strict` |
|---|---|
| `W-CHARSET-UNSUPPORTED` | `E-UNSUPPORTED-CHARSET` |
| `W-NUMBER-ROUNDED` | `E-NUMBER-OVERFLOW` (value class; only for a value `round` would change) |
| `W-LOB-UNTESTED` | `E-LOB-UNCONFIRMED` (value class) |
| `W-NUMBER-NULLED`, `W-VALUE-NULLED` | unchanged (a policy the caller chose) |
| every other code: `W-UNKNOWN-INTRO`, `W-TRAILING`, `W-UNTESTED-VERSION`, `W-UNTESTED-HEADER`, `W-FILE-SIZE`, `W-UNTESTED-LAYOUT`, `W-DDL-FALLBACK`, `W-UNVERIFIED-TYPE`, `W-LONG-LENGTH`, `W-LONG-CHARSET`, `W-LONGER-THAN-DECLARED`, `W-CHARSET-MISMATCH`, `W-TSLTZ`, `W-POSSIBLY-INCOMPLETE` | `E-STRICT` (the default rule) |

`Error.warning` names the warning exactly when the error would have been that warning without `strict` (so not on
an integer part that does not fit, and not on `E-LOB-UNCONFIRMED` without `unconfirmed_lob="decode"`). A new warning
code is `E-STRICT` under `strict` unless its decision record says otherwise.

7.2 **Carried over from the rules file.** [S7]: the reader never counts, expects or pairs statistics
items; it skips each by [S1, S2] until the section ends [T2]. A test asserts that the `stc` files (0, 2,
4 column items for 5 and 6 columns) and the `stat_exp_none` files give the same batches as their
siblings. Unknown lines: tested with G1 and reported with their offset (2.5). `Catalog.item_counts` lists
items by kind for diagnostics; nothing in the reader depends on them.

7.3 **Never a panic.** `#![forbid(unsafe_code)]` in the core; no `unwrap`, `expect`, slice indexing or
unchecked arithmetic on input-derived values outside tests (clippy lints `unwrap_used`, `expect_used`,
`indexing_slicing`, `arithmetic_side_effects` denied in `oradmp-core`); every read goes through layer 1,
which returns `Truncated` with the offset. Fuzzing checks it (8.3).
Session 14 (ADR-59): should a panic still happen, `oradmp-py` calls the core only through
`oradmp_core::guard::catch_panic` (`std::panic::catch_unwind`), also inside the Arrow C stream's pull, and turns
the panic into **`E-INTERNAL`** (Python `InternalError(OradmpError)`; fixed message, no value, no payload, no
offset); the guarded `Dump` or reader is unusable afterwards (`E-INTERNAL` on every later call). A panic in the
binding's own Python glue, outside the guard, stays PyO3's `PanicException` (a `BaseException`). The library does
not change the process's panic hook.
Session 16 (ADR-59 amended, ADR-65 item 1): "every call" means every call that reads or decodes the file or parses
user input (`NumberType::from_str` and `number_summary` included). Exempt by name: the accessors `info()`,
`catalog()`, `warnings()`, `TableBatches::schema()`, `Error`'s `Display`, the drops in `close()`, and the
feature-gated probe's `open`.
Session 18 (ADR-59 amended; review 05 N-1): also exempt, panic-free by reading: `Warning`'s and `Error`'s `Display`
in `warning_dict` / `error_instance`, `Charset::from_id` / `Charset::name`, the `as_str` constants and `ALL` of the
code enums, and the implicit drops of the Python objects at garbage collection.

7.4 **Offsets.** Every error and warning from layers 1 to 4 carries the absolute file offset of the first
byte it could not accept; value errors add table, column position and row number. Session 5 (ADR-29):
`table` = section ordinal, `part` = 1-based part index (set with `row`), `row` = 1-based within the part,
`column` = 1-based **file position** in the part (never the declared position); Python resolves names.
Session 9 (ADR-41 item 6): a warning about one value (`W-LONG-LENGTH`, `W-UNVERIFIED-TYPE`, ...) carries the
offset of the **start of that value**, the byte a value error of it would report: the first data byte of a
conventional field, the first data byte of the **first** piece of a direct-path LONG chain.
Session 10 (ADR-43 item 3): the same offset for **every** report about one value, under every policy: the value
error under `raise`, the `W-VALUE-NULLED` / `W-NUMBER-NULLED` under `null` and every value-level warning; for a
LOB value the offset `lob_start` receives. `E-BAD-TEXT` inside a chain or a LOB value reports the value start
too, and its message names the 1-based piece (chunk) where the failure was detected (session 11, ADR-46 item 2: a
LONG chain's message names the piece only for a value of more than one piece; a LOB message always names the
chunk, also for a one-chunk value). Errors of the chain's or
the LOB area's framing (layer 2) keep "the first byte it could not accept".

## 8. Testing

8.1 **A test per rule.** Rust integration tests in `crates/oradmp-core/tests/`, named after the rule.

| rules | fixtures | asserts |
|---|---|---|
| H1 to H5 | `t_frame` with 3 clients, `ru_tbl_*`, `*_rl4096`, `*_rl16384`, direct files | ids, `D` flag, seven lines, R from the text, size multiple of R |
| H6 | all 177 | first item at R + 0x104 is `METRICST` / `METRICSU` and agrees with the mode line (confirmed pairing, A-10); a patched copy in memory gives `MissingMetricsLine`, a swapped pair `W-UNTESTED-HEADER` |
| I2 to I7, I6 list | `ddl`, `own`, `stat`, `stc`, `ru_stat_*`, `t_decl` | items per file and fields per list entry equal the Analyst's counts (T-1); the data file equals the block of the rules file in all four fields `kind:length:forms:text` (N-3) |
| G1 | all 177 with an **empty** list K | same items as with the list, plus one warning per field |
| T1, T2, T8 | `own` / `own_tables`, `multi_*`, `long`, `lob`, `misc` (T_FRAME after LONG, LOB, INTERVAL, ROWID tables, both paths) | sections found; byte-identical sections give equal batches; session 7: owner mode `t7_owner.rs` (ADR-36 item 1) |
| T3 to T6, X1, X5 | all; `ru_tables_exp_us7ascii_11g`, `ru_exp_*`, `nchar`, `long`, `t_decl`, `lob`, `misc` | names, counts, descriptors (16 codes, 8-byte descriptors for 1, 96, 112), m, path; a patched code 114 gives `UnsupportedType`, patched m (2 for 3 columns), m > 0 with flag 1, an entry with n > 0 on the NUMBER column each give `UnsupportedLayout`; earlier tables readable |
| L1 to L6 | `lob_exp_<v>`, `lob_exp_direct_<v>`, the 7 R_CLOB files (`cp1251`); session 8: `ru_lob2_exp_cp1251_11g`, `ru_lob2_exp_us7ascii_11g` (R_LOB2 C / N / B short and long = `truth_lob_ru.txt` and `15_ru_lob2.sql`, R_TEXT after it = `truth_ru.txt`; T-6M) | m and entries (`03 00`, `00 00 00 01 43 00 00 01 42 00 00` for T_LOB); k per row (T_LOB 2, 0, 2, 2, 2; T_LOB2 3, 1; R_CLOB 1, 1, 0); short, empty and long values; 13 chunks for each long value with the version's sizes (18c 12 x 16,120 + 6,560 and 12 x 8,060 + 3,280; 11g 12 x 16,264 + 4,832 and 12 x 8,132 + 2,416); LOB tables on flag 0 in `direct=y` files; T_FRAME after them; patched k, chunk length 0, chunk sum past T, value-area `fe ff` give `Framing` |
| T10 | `part_exp_18c`, `part_exp_direct_18c` | one table, 3 parts P2019 / P2020 / PMAX with 1,095 / 1,098 / 807 rows, `num_rows` 3,000, flags 0 / 1 per part; a patched second part without its `PARTITION` line gives `Framing`; a patched descriptor in part 2 gives `Framing` |
| Y10, Y11 | `misc_exp_<v>` | T_INTERVAL 3 values + NULL per column, T_ROWID 1 value + NULL per column = `truth_misc.txt`; patched lengths give `E-BAD-INTERVAL` / `E-BAD-ROWID` |
| E2a | synthetic only (no fixture, R-031) | the three outcomes of ADR-22, default and strict; session 7: default side through `read_table` in `t8_direct.rs`, strict side T-11 |
| T9 | every `CREATE TABLE` line reached; `t_decl` | 5.5: agreement with descriptors and `INSERT INTO` names; T_DECL precision / scale = `truth_decl.txt` |
| T7, S1, S2, S6, S7 | `stat`, `stat_exp_none`, `stc`, `stc_exp_none`, `stc_exp_us7ascii` | 7.2 |
| R1 to R5, R8 | `t_len`, `t_nullmix`, `t_wide`, `t_bn1` to `t_bn4`, `t_probe6`, `long` | lengths 255 / 256 / 257 and 32,000, NULL patterns, 120 columns, values across multiples of R; patched lengths 0 and 0xfffd give `Framing` |
| Y9 | `long` | T_LONG 3 rows (L 5 bytes, NULL, 32,000 bytes; V after L), T_LONGRAW 2 rows (2, 32,000 bytes), values equal to `corpus/sql/11_types.sql` (no truth file) |
| C5, C6 | `nchar` | NC, CC, NV of `truth_nchar.txt`; `trim_char` gives `ab`, U+0434 U+0436; no `W-LONGER-THAN-DECLARED` |
| C7 | `truth_csmap.txt` (both folders) | 768 lines (6.2) |
| R6, R7 | every conventional / direct pair | equal multisets of rows; no use of `METRICET`; session 7: `t8_direct.rs`, 80 pairs, T_PART / `part2` per part, LOB pairs on flag 0 [L6] (ADR-36 item 2) |
| E1 to E4 | `t_empty`, `multi_empty*`, `own`, `stat` direct (18c and 11g) | 0 rows with and without `ff ff`; walk ends at the second `EXIT`; session 7: `t8_direct.rs` (both forms, version line swapped, ambiguous case `Framing`; ADR-36 item 3) |
| X2 | the cut-short section of `ru_exp_us7ascii_11g` (exp.md 14.3, from 0x135ef), spliced in memory behind a header: in the file it stands after R_CLOB (ADR-12) | `possibly_incomplete` set |
| C1 to C4, X3 | `t_text` x 3 clients, `ru_tbl_*` x 4, `own_exp_w1252` | values equal truth for every client; names in the client set; unknown id gives Binary |
| Y1 to Y8 | `t_num`, `t_num2`, `t_numtypes`, `t_date`, `t_ts`, `t_raw`, `t_tstz`, `t_decl` | 8.2; NUMBER as 5.2 step 5 |
| session 4: T3, T9, ADR-24 | `long2_exp_direct_<v>`, `long_exp_direct_<v>` (6 sections) | `INSERT INTO` order ID, V, L against `CREATE TABLE` ID, L, V; schema in declared order; conventional and direct exports give the same schema and equal batches |
| session 4: R2, Y9 chains | `long_exp_direct_<v>`, `long2_exp_direct_<v>` | T_LONG row 3 = `9c 80` + 156, 3 x `19 9f` + 7961, `19 1f` + 7961 = 32,000 bytes at 0x2288 (18c) / 0x2289 (11g); T_LONGRAW the same chain; one-piece values; NULL `fe ff`; T_FRAME after them; patched piece u16 `f0 ff` gives `Framing`, a patched conventional LONG length `00 80` gives `UnsupportedLayout` |
| session 4: T10 composite | `part2_exp_18c`, `part2_exp_direct_18c` | one table, 6 parts named P1 / SYS_SUBP368, P1 / SYS_SUBP369, P_EMPTY / SYS_SUBP370, SYS_SUBP371, P3 / SYS_SUBP372, SYS_SUBP373 with 21, 29, 0, 0, 21, 29 rows (= the `ROW` lines of `truth_part2.txt` and the log); empty parts by [E1] (conventional) and [E2] (direct, followed by `SUBPARTITION "` and `ENDCOMPARTITION`); the 16 `ANALSTATS` items before `ENDPARTITION` skipped; `num_rows` 100; patched line orders of ADR-21 item 4 give `W-UNTESTED-LAYOUT` |
| session 4: C5, C6, L5 non-BMP | `nchar2_exp_<v>`, `lob3_exp_<v>` row 20, `t_probe11` | the vectors of 5.6 step 6; `CHAR(3 CHAR)` `61 f0 9f 98 80 20` with `trim_char` gives `a` + U+1F600; descriptor lengths of `t_probe11` = `truth_probe11.txt` `COL` lines (min(4n, cap)) |
| session 4: L4 switch | `lob3_exp_<v>` | 11 sizes per column, short up to one chunk and long beyond (18c 8,060 / 16,120 bytes, 11g 8,132 / 16,264), lengths = `truth_lob3.txt`; a value split by a record boundary inside `fc ff` (`lob3_exp_11g`@0x47fff) reads |
| session 4: Y1 computed | `t_num3_exp_<v>` | the 6 values of `truth_num3.txt` exactly as text and as Decimal256(76, 40); under Decimal128(38, 10) with `round` all 6 change and none becomes 0; under Decimal256(76, 38) rows 1, 2, 3, 5 change, 4 and 6 are exact |
| session 4: Y10, Y11 | `misc2_exp_<v>` | the 9 interval values of `truth_misc2.txt` (texts by the test renderer; Arrow values as 5.7, IYM rows 2 and 3 `E-BAD-INTERVAL`); 3 ROWIDs; the logical UROWID `*BAMAFbMCwQL+` (18c; the 11g text from the truth) |
| session 4: C8 | all 199 | no `W-LONGER-THAN-DECLARED`; a patched RAW length one above the descriptor gives the warning, not an error |

8.2 **Truth check, all 199 files** (session 8: **201** files, 386 sections, 400 parts after T-3d [rules v3.5,
"Changes in session 10"]; new truth files `truth_lob_ru` (R_LOB2) and `truth_r045` (an SQL*Plus spool, no EXP
table: listed, not compared); session 4: 396 parts for 382 sections; the new truth files are
`truth_num3`, `truth_nchar2`, `truth_lob3`, `truth_long2`, `truth_misc2`, `truth_part2`,
`truth_probe11`; T_LONG and T_LONGRAW of `long_exp_direct_<v>` against `11_types.sql` as for
`long_exp_<v>`), in two steps (session 3: the manifest lists **binary parts**, 338
lines for 334 sections, and adds per part the LOB value count, LOB byte total and a SHA-256 over the
LOB values in a documented serialisation; no file stops; the statements below about the 7 CLOB stops
are history). (a) *Raw level*: a committed manifest
`tests/golden/exp_raw.tsv` with, per file and table, the column count, type codes, path, row count, NULL
count and a SHA-256 over the raw fields; it is produced once from the Analyst's `tools/exp_parse.py`
(whose rows equal the truth with 0 differences in `tools/check_all.py`) and reviewed; the Rust walker
must reproduce it. This covers the tables whose expectation the Analyst derives from SQL (T_STAT2,
T_STAT3, R_STAT, T_STC1 to T_STC5, T_BN1 to T_BN4) without re-implementing that SQL. (b) *Value level*:
tests parse the `truth_*.txt` files directly and compare decoded values: `truth_counts.txt` and the logs
for row counts; `truth_num`, `truth_num2`, `truth_numtypes` (decimal value of the `string` rendering, and
Int64 / Decimal128 where they fit, with `on_number_overflow="error"`); `truth_date`, `truth_ts`,
`truth_tstz` (instant = local time minus offset); `truth_text`, `truth_ru` (text after decoding against
the truth's bytes decoded the same way, and lengths); `truth_nchar`; `truth_decl` (declared types);
`truth_raw`; `truth_frame`, `truth_len`, `truth_nullmix`, `truth_wide`, `truth_own`, `truth_stat`
(`ROW|` lines); T_LONG, T_LONGRAW and the T_DECL row against the SQL of `corpus/sql/11_types.sql` (no
truth file exists for them). Rows are compared as multisets or by id column [R6]. Session 3: every
one of the 177 files walks to the second `EXIT` (ADR-12); `truth_lob.txt` (lengths in characters for C,
A, N and in bytes for B, the first 30 bytes of B in hex, the values of the SQL of
`corpus/sql/01b_optional_tables.sql` and `13_optional.sql` for the text), `truth_ru.txt` R_CLOB lines
(length and the first 60 bytes), `truth_misc.txt` (Y10, Y11), `truth_part.txt` (rows per partition, the
`TOTAL` line) join the value-level check.
Session 9 (ADR-38, ADR-42): a truth file counts as compared only when a check **opened** it through
`common::truth`, keyed by (folder, file); R_STAT, T_STC (counts), T_LONG2, T_INTERVAL2 / T_ROWID2 of both
folders and all 27 R_TEXT sections are compared value by value; every check has a break test. After every
group of tasks that changes tests over the corpus, a Reviewer session checks the tests, golden files and
expected values before the next group starts; golden files come only from the generators over the Analyst's
tools.

8.3 **Property and fuzz tests.** `proptest`: (a) for every fixture, truncation at a random offset and 1
to 8 random bit flips: the walker returns `Ok` or a typed error with an offset not beyond the damage
horizon, never panics, never allocates more than a fixed bound; (b) a generator that writes synthetic
files from the rules (lines, three field forms, rows, both paths, both empty-table forms) and checks
that the reader returns what was written; (c) NUMBER: bytes to text to Decimal128 round trips on
generated digit strings. `cargo-fuzz` (libFuzzer) targets `walk` and `decode_table`, seeded with the
fixtures; nightly, not on every push.
Session 15 (ADR-62): (b) moves to T-17 (after T-16, not a beta gate); (c) exists (`t4_values.rs`). The targets are
`walk` (`exp::items::walk` over the bytes), `read` (the bytes written to a temporary file, then `open`, `scan` and every
table read with `on_value_error=Null`, `batch_rows=1024`, `max_lob_bytes=1 MiB`) and `values` (the value decoders on
raw fields); limits `-max_len=1048576 -timeout=10 -rss_limit_mb=2048 -malloc_limit_mb=64`, 10 minutes per target. They
call the core directly, so a panic is a crash (the binding's guard of 7.3 is not in the way). Every error a target sees
starts with `[E-`, has a code of `ErrorCode::ALL`, an offset not beyond the input and no U+0000. "Never allocates more
than a fixed bound" is measured there (libFuzzer's limits), not in the proptest of (a).
Session 16 (review 04 N-9, ADR-65 items 3, 10): "a fixed bound" reads: no single allocation above 64 MiB for a 1 MiB
input (`-malloc_limit_mb`), and the process below 2 GiB (`-rss_limit_mb`); slow growth is the memory test's (4.3).
The truncation property is backed by one deterministic case per "file ends" site with its exact offset. Runs:
`tools/fuzz_run.sh <target> [seconds]` (ADR-64 item 3).
Session 22 (ADR-80 amendment, ADR-86): (b) exists (T-17, `crates/oradmp-core/tests/common/synth.rs`). It writes
confirmed bytes in combinations the fixtures show; a combination no fixture shows only under a named rule and as a
labelled `Unconfirmed` construct (today: E1 after a direct-path flag in an 18c file, [E3]; a client set the fixtures
do not pair with that header, [C4]). Its files also seed the fuzz targets (`target/fuzz-seeds/`).

8.4 **Fixtures are read-only.** A test recomputes the SHA-256 (crate `sha2`) of every file listed in the
three `SHA256SUMS` and fails on a difference or on an unlisted `.dmp`; tests open fixtures read-only and
write only below `target/`. CI also runs `sha256sum -c`.

8.5 **What the fixtures cannot test**, and what replaces it: an `exp` client older than the database
(R-022), the real schema's types and sizes, gigabyte files. The project owner runs, on his side only:
(1) `python/exp_survey.py` (structure-only, exists); (2) from the alpha on, `python -m oradmp survey
FILE [--no-names]`: the production reader in a mode that prints header fields, per table the column type
codes and [T9] type texts, row count, NULL counts, length minimum / mean / maximum (for LONG and LONG
RAW also the count above 32,000 bytes and, session 4, the count of chained values and the path of
each part), the NUMBER profile of 5.2b (session 4: per candidate type), for LOB columns
`ColumnInfo.lob` (values, empty, short, long, largest stored length) and the count of values that
would raise `E-LOB-UNCONFIRMED` (ADR-25), the database, client and national set ids [H1, H4], for
partitioned tables the parts with partition, subpartition and row counts (names hidden with
`--no-names`), warnings and errors with offsets, `Catalog.end` with the stopped table and type
code, and **never a value**; its output can be pasted back. Session 11 (ADR-46 item 9): `python -m oradmp survey
FILE [--no-names] [--no-read]` runs the scan, the counting pass and, by default, a lenient read of every table
(`on_value_error="null"`) that lists the value-level warnings with counts and first offsets; `--no-read` skips that
pass; the output's first lines name the passes run. Agreement of (1) and (2) on counts is the acceptance test for a real dump.

8.6 **CI.** Every push: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo nextest run` (rule
tests, raw manifest, value truth, proptest with a small case count), fixture checksums, `maturin
develop` + `ruff`, `mypy`, `pytest`. Nightly or manual: `cargo-fuzz` (10 minutes per target), proptest
with a large case count, benchmarks when `corpus/work/` exists, wheels for the release platforms.
Session 16 (ADR-68): the nightly job runs `tools/fuzz_run.sh <t> 60` per target, `PROPTEST_CASES=10000`, nightly
clippy of `fuzz/` and the micro-benchmarks as a report; CI has no `corpus/work/`, and the benchmarks gate only under
ADR-66.

## 9. Repository layout and toolchain

    Cargo.toml                       workspace: members crates/oradmp-core, crates/oradmp-py
    rust-toolchain.toml              one pinned stable release (the VM has rustc 1.99.0)
    pyproject.toml                   build-backend maturin; [tool.maturin] manifest-path =
                                     "crates/oradmp-py/Cargo.toml", python-source = "python",
                                     module-name = "oradmp._core"
    crates/oradmp-core/             src/{lib,error,io,api,charset,arrow_out}.rs, src/exp/, src/values/,
                                     data/exp_intro_lines.txt, tests/ (rule and truth tests), benches/
    crates/oradmp-py/               src/lib.rs (PyO3 classes only)
    fuzz/                            cargo-fuzz targets (outside the workspace, as cargo-fuzz expects); session 15
                                     (ADR-62): own `fuzz/Cargo.lock` and `fuzz/rust-toolchain.toml` (a dated
                                     nightly); `fuzz/corpus/`, `fuzz/artifacts/` not committed
    python/oradmp/                  __init__.py, __main__.py (survey; from T-18 the command-line tool),
                                     _cli.py (T-18, 3.5; console script `oradmp`), _core.pyi, py.typed
    python/exp_survey.py             stays where it is (owner's tool, not part of the package)
    tests/                           pytest for the Python API; tests/golden/exp_raw.tsv and
                                     exp_catalog.tsv with their generators
    docs/design/                     these documents
    LICENSE-MIT, LICENSE-APACHE      session 10 (ADR-45): licence MIT OR Apache-2.0; `license` in
                                     [workspace.package] and pyproject.toml (PEP 639 expression)
    deny.toml                        cargo-deny licence allow-list (ADR-45 item 5)
    README.md, CONTRIBUTING.md       licence section, trademark line, contribution sentence (ADR-45)
    tools/release_build.sh           session 14 (ADR-56): the release build from a clean clone at a neutral
                                     path; Implementer's; in neither archive (ADR-57)
    HISTORY.md                       session 26 (ADR-105): the owner's note on the old hashes and a1 to a3

Session 11 (ADR-3, ADR-48): arrow-array / -buffer / -schema **59** (lock 59.3.1), pyo3 **0.30** (`abi3-py311`),
pyo3-arrow **0.20**; they move together only in a task of their own (bump rule, ADR-3). Wheel:
`cp311-abi3-manylinux_2_28_x86_64` built with `maturin --zig`, maturin pinned exactly in `pyproject.toml`
(ADR-48; baseline pending Q-9). The VM venv is `.venv` at the root.

Session 12 (ADR-49, ADR-50, ADR-52, ADR-53): version `0.1.0-alpha.1` in `[workspace.package]` only; `pyproject.toml`
takes it through `dynamic = ["version"]` (Python `0.1.0a1`); both crates `publish = false`. `deny.toml` limits the
dependency graph to five targets (Linux x86_64 / aarch64, Windows x86_64, macOS x86_64 / arm64). `.gitattributes`
marks the fixtures and golden files `-text` (dumps `binary`). The first release is Linux x86_64 only (Q-9 closed);
other platforms come with T-16 and its acceptance rule.

Dependencies of `oradmp-core`: `arrow-array`, `arrow-schema`, `arrow-buffer` (the sub-crates, not the
`arrow` umbrella: no CSV / JSON / IPC code is compiled), `encoding_rs`, `thiserror`, `sha2` (normal
dependency since T-2: the walker hashes raw fields for the manifest, N-6). Dev: `proptest`, `criterion`. `oradmp-py`: `pyo3`, `pyo3-arrow`, `oradmp-core`. The versions of `pyo3` and the
arrow crates are the pair that the chosen `pyo3-arrow` release requires. `Cargo.lock` is committed.

Python: the VM develops on 3.13 / 3.14; **minimum supported 3.11** (Q-3 decided by the owner, ADR-13), built as one `abi3-py311` wheel per
platform (PyO3 feature; `pyo3-arrow` documents that its buffer-protocol feature needs `abi3-py311` or a
non-abi3 build). pyarrow 15 or later. Tools: `uv`, `ruff`, `mypy`, `pytest`, `cargo-nextest`.

Ownership of these directories is that of `CLAUDE.md` (Q-5 settled by the owner: the Architect writes
`docs/design/`). Session 10: the root files of ADR-45 and `pyproject.toml` are the Implementer's (proposed
`CLAUDE.md` text in `open-questions.md` (a)). Session 11: the Implementer also owns `python/oradmp/` and the pytest
files; the `CLAUDE.md` text is still to be applied by the owner (`open-questions.md`, Q-5). Neither the wheel nor the sdist contains `corpus/`, `tests/`,
`docs/` or any `.dmp` / `truth_*.txt` (ADR-45 item 6).

Session 26 (ADR-105): the repository is public, `https://github.com/granddad-wolf/oradmp` (remote `origin`), from
commit S `9950ef2`, whose tree is the private archive's last commit P `b55eb10` plus `HISTORY.md`. Commit hashes in
`docs/` before session 26 name archive commits; `HISTORY.md` says so. `docs/prompts/`, `.claude/` and `CLAUDE.md` (the
agents' prompts, definitions and rules, including the ownership table cited above) are private and ignored by
`.gitignore`; a contributor follows `CONTRIBUTING.md`. From 0.1.0a4 on every release is built from a commit of this
repository with `tools/release_build.sh <commit> --twice --release` at the canonical root (ADR-90) and tagged here only.
