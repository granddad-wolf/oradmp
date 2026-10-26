The architect subagent finished session 1: four design documents are in [docs/design/](docs/design/), untracked and not committed, and no existing file was modified. I confirmed the files exist but have not reviewed their content; the rest is the architect's report.

| Document | Content |
|---|---|
| [exp-reader-architecture.md](docs/design/exp-reader-architecture.md) | Architecture, sections 1 to 9 |
| [decisions.md](docs/design/decisions.md) | ADR-1 to ADR-13: 10 decided, 3 waiting for you |
| [implementation-plan.md](docs/design/implementation-plan.md) | Tasks T-1 to T-15 and the alpha definition (cut after T-13) |
| [open-questions.md](docs/design/open-questions.md) | 5 owner questions, Analyst requests R-024 to R-030, 5 notes on the rules file, 9 ranked risks |

## Design in brief
- **Crates:** `oradump-core` (pure Rust, no `unsafe`) and `oradump-py` (PyO3 `cdylib` built by maturin).
- **Data Pump seam:** a single `RowSource` trait; everything above it is format-neutral.
- **Reading model:** `open` reads the header only, `scan` walks the file once and builds a catalog with exact row counts, and `read_table` seeks with its own file handle.
- **Output:** each table is an Arrow `RecordBatchReader`, bounded at 65,536 rows and 64 MiB per batch.
- **Types:** NUMBER and TIMESTAMP map by the declared type parsed from the `CREATE TABLE` column list; every conversion is exact or an error.
- **Errors:** fatal framing errors stop the read and never resynchronise; anything outside the confirmed rules is a warning, or an error under `strict`.
- **Python:** a `BatchReader` exposing `__arrow_c_stream__` through `pyo3-arrow`, plus `python -m oradump survey` as the structure-only mode for real dumps.

## Decisions for you
1. **`NUMBER` without precision:** the architect recommends Decimal128(38, 10), exact or an error, with a per-column override.
2. **Default strictness:** it recommends warn-and-continue outside the confirmed rules, with `strict=True` as opt-in, because the real dumps carry a 10.2 version line that no fixture has.
3. **Minimum Python:** it recommends 3.11, one `abi3-py311` wheel per platform, pyarrow 15 or later.
4. **Real column types:** it asks for the list of type codes from `python/exp_survey.py`, whether any tables are partitioned, and whether anyone needs the original TSTZ zone offset. Any code outside 1, 2, 12, 23, 96, 180, 181, 231 moves LOB, LONG and INTERVAL/ROWID work (R-027 to R-029) ahead of the alpha.
5. **Which documents to correct:** see the disagreements below; it recommends updating `CLAUDE.md` and `docs/scope.md`.

## Disagreements it found
- **Output directory:** `CLAUDE.md` gives the Architect `docs/decisions/`, while the prompt says `docs/design/`. It followed the prompt.
- **Character sets:** `docs/scope.md` lists only AL32UTF8, while the prompt asks for five. It followed the prompt.
- **Fixture count:** the prompt and rules file say 157 EXP files, but the folders hold 161. The four extra are the R-023 files, which are committed while `requests.md` still shows R-023 as open. The plan tests all 161.

## Top risks
1. **10.2 client:** the real dumps come from an `exp` 10.2 client against 11.2.0.4, and no rule is confirmed for that.
2. **Unsupported column types:** one LOB or LONG column stops the file and hides every later table. In the 6 `ru_exp_*` fixtures, R_CLOB sits before R_TEXT, so R_TEXT is unreachable in phase 1.
3. **Silent misread:** an unknown introducing line could in principle cause a table to be skipped without an error.

## Not yet established
- **NUMBER decoding:** the format document gives test vectors only, so the decoder is "suggested" until R-024.
- **Column-list grammar:** observed in fixtures only (R-025).
- **Oracle versus WHATWG code pages:** whether the 1251 and 1252 tables match byte for byte is not established (R-026b).
- **Crate facts:** versions of `pyo3`, `pyo3-arrow`, arrow-rs and `encoding_rs` were read from documentation, not built.

The Analyst requests R-024 to R-030 are only proposed in [open-questions.md](docs/design/open-questions.md); nothing was written into `docs/formats/`, and none of the SQL was run.

The architect also pointed out that two of the untracked files in the repo root are named `survey_real_v2.txt` and `survey_real_v3.txt`. It did not read them, but given the no-real-data rule you may want to check that they belong in the repository directory.