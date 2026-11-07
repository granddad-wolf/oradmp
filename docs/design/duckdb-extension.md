# DuckDB extension `read_oradmp`: design

Architect session 33, 2026-11-04. Authority: ADR-130 (decided); Q-16, the release channel, needs the project owner.
Tasks: T-22 to T-27 in `implementation-plan.md`, "DuckDB line". Rule numbers are those of
`docs/formats/exp_reader_rules.md` v3.9.

## 1. What the owner asked

A DuckDB table function `read_oradmp(file, table := ...)` for `exp` dump files above 1 GB, so that DuckDB's engine
processes the rows directly: no Python, no intermediate Parquet. The function is a separate binary, not part of the
Python wheel. It does not block a5.

## 2. What the format allows (facts, not choices)

1. **No table index**:
   - A row has no header, length or number (R1).
   - No row count is stored (R6).
   - Table k's DDL lines and descriptors come after the rows of tables 1 to k-1 (T1 to T6, E1).
   - So the schema of a table is known only after walking every earlier row, field by field. The header [H1 to H5]
     lists no tables.
2. **The walk is cheap compared with decoding**: on the VM, `T_BIG` (453 MB, page cache) walks at 779 MB/s and decodes at
   199 MB/s of row area on one thread (`benchmarks.md` 4.2). A 1 GB file therefore walks in about 1.3 s (cached; a cold
   disk is slower) and decodes in about 5 s on one thread.
3. **Rows are independent**: a row needs only its part's descriptors and path flag (R1, R4, R7). A LONG chain (R2) and
   a LOB area (R4, L3) belong to the row. So decoding can start at any row start that the walk recorded.
4. **Rows have no order** (R6). File order is still the order every test and every user sees today.

## 3. Layers

```
DuckDB 2.0.x  --C extension API-->  crates/oradmp-duckdb (own workspace; FFI, cache, SQL surface)
                                         |  Rust calls only
                                    crates/oradmp-core (no FFI, no cache, no global state)
                                         |  std::fs::File
                                    the dump file (local path)
```

1. **`oradmp-core`** (T-23) gives:
   - `SharedDump` (`Send + Sync`): the header info, `Arc<Catalog>`, options and file identity, made by
     `Dump::into_shared`, which walks if needed.
   - `SharedDump::read_table(&self, which, &ReadOptions)` and `read_split(&self, which, part, split, &ReadOptions)`:
     both return `TableBatches`, each with its own file handle.
   - Split points in `PartInfo`: `(offset, first row ordinal)` about every `Options.split_bytes` (default 8 MiB; 0 =
     none).
   - It keeps the conditions of Q-13: no PyO3, no unsafe code, stable error and warning codes, plain arrow-rs
     `RecordBatch`, defaults in the core.
2. **`oradmp-duckdb`** (T-24) gives:
   - the three SQL functions (section 4);
   - a cache of at most 16 `SharedDump`s, keyed by file identity;
   - the mapping from Arrow types to DuckDB types (section 6);
   - the warning policy (section 5);
   - one `unsafe` module, the entry point;
   - a panic guard on every callback.

## 4. SQL surface (first version)

| function | returns | notes |
|---|---|---|
| `read_oradmp(file, table := 'T', table_ordinal := n, <options>)` | the rows of one table, all parts in file order | exactly one of `table` / `table_ordinal`, or neither if the file has one table with data; projection pushdown; exact cardinality; `parallel := false` by default |
| `oradmp_tables(file)` | `ordinal, name, name_possibly_damaged, columns, parts, partitioned, path, rows, ddl_fallback, warnings` | one walk, which is cached |
| `oradmp_columns(file)` | `table_ordinal, table_name, position, name, oracle_type, declared_text, duckdb_type` | from the catalog |

Options of `read_oradmp`:
- These keep the names, types and defaults of the Python API: `strict`, `on_value_error`, `db_charset`,
  `client_charset`, `number_default`, `on_number_overflow`, `timestamp_unit`, `interval_as`, `trim_char`,
  `max_lob_bytes`, `unconfirmed_lob`.
- New for the extension:
  - `allow_warnings` (a list of codes);
  - `parallel` (boolean).

Not in the first version:
- `number_types`;
- partition selection;
- globs and several files;
- remote paths (a URL scheme is `E-CONFIG`).

## 5. Errors and warnings

1. An error becomes the DuckDB error text `[<code>] <message>` with offset, table, part, column and row, the format
   the CLI prints. The text holds no value.
2. Bind errors (examples):
   - `E-NO-SUCH-TABLE`;
   - `E-AMBIGUOUS-TABLE-NAME` (the text suggests `table_ordinal`);
   - `E-CONFIG` (bad option, URL scheme, both or neither of `table` / `table_ordinal` with several tables);
   - a type that cannot be mapped without loss (section 6).
3. Warnings are never dropped:
   - catalog warnings go into `oradmp_tables().warnings`;
   - a read-time warning fails the query with its code and offset, unless one of two things holds:
     - its code is in `allow_warnings`;
     - the caller asked for it with an option. The crate keeps that list per code and names the option for each; for
       example, `W-VALUE-NULLED` follows from `on_value_error := 'null'`.
   - `strict := true` makes every warning an error, as in the core.

## 6. Types

Rule: lossless or a bind error that names the column and the option that avoids the loss. The extension never rounds
or truncates. T-24 fills this table from the fixtures of each type (arch 5.1); until then every row is **unverified**.

| core Arrow type (arch 5.1) | DuckDB type | lossless? |
|---|---|---|
| Int64 | BIGINT | unverified |
| Decimal128(p, s), p <= 38 | DECIMAL(p, s) | unverified |
| Decimal256 | none (DuckDB DECIMAL stops at 38) | bind error; use `number_types` / `number_default` string or float |
| Float64 | DOUBLE | unverified |
| Utf8 / LargeUtf8 | VARCHAR | unverified |
| Binary / LargeBinary | BLOB | unverified |
| Timestamp(s / ms / us) | TIMESTAMP_S / _MS / TIMESTAMP | unverified |
| Timestamp(ns) | TIMESTAMP_NS | unverified |
| Timestamp with zone | TIMESTAMPTZ (microseconds) | a value with sub-microsecond digits is a value error |
| Interval(MonthDayNano) | INTERVAL (microseconds) | as above |
| Duration | to be read from DuckDB's Arrow import documentation in T-24 | unverified |
| Date32 | DATE | unverified |

## 7. Parallelism

1. **Across tables and parts**: each `read_oradmp` call and each part has its own reader and file handle. DuckDB runs
   separate calls in separate pipelines.
2. **Within a table** (T-26, opt-in `parallel := true`):
   - Work units are the split points of section 3.
   - `duckdb_init_set_max_threads` is set to the number of splits, capped by DuckDB's thread count.
   - Each local state takes the next unit from a shared counter.
   - Errors keep exact row numbers through the split's first-row ordinal.
3. **Row order**:
   - With one thread: file order.
   - With `parallel := true`: whatever the C API allows. T-26 reads DuckDB's documentation for a way to keep insertion
     order. Until a record says otherwise, the order is undefined with `parallel := true`, and the README says so.

## 8. Memory

1. Per thread: one batch of `duckdb_vector_size()` rows (DuckDB's vector size), the read buffer (`buffer_bytes`), and
   the largest single value (bounded by `max_lob_bytes`, default 256 MiB, for LONG and LOB only).
2. Per process: the cache of at most 16 catalogs. A catalog is proportional to tables x columns plus 16 bytes per split.
3. **Gate (T-25)** on the VM, alone, with `T_HUGE` (O-25, about 1.09 GB):
   - with `SET threads = 1`, the peak RSS of the DuckDB process is at most the baseline + 64 MiB. The baseline is the
     same binary with the extension loaded, running the same aggregate over `range(12000000)`;
   - within 1.10 of the same figure on `T_BIG`;
   - with `parallel := true` and n threads (T-26): at most the baseline + n x 64 MiB.

## 9. Versions, platforms, build, release

1. DuckDB 2.0.x LTS only (2.0.0 released 2026-10-21; end of life 2027-10-22, per DuckDB's release calendar).
2. Linux x86_64 only, with a glibc floor of 2.28.
3. One binary per exact DuckDB version, until T-22 shows otherwise.
4. The `duckdb` crate with `loadable-extension` + `vtab`. It must use the arrow major of the core; any difference is
   settled by ADR-3's bump rule in the core.
5. The crate is its own workspace in `crates/oradmp-duckdb/`, so the root workspace, `pyproject.toml`, the wheel and the
   sdist do not change.
6. CI: a job `duckdb` on push (Linux). It runs fmt, clippy, `cargo deny`, a release build and the SQL tests
   (`tests/duckdb/`, outside the default pytest collection, 0 skipped in this job).
7. Release: Q-16. The recommendation is a GitHub release asset built by `tools/release_build.sh <commit> --duckdb
   <version>` at the canonical root and loaded unsigned. This leaves `CLAUDE.md` rule 8 unchanged.

## 10. Open, with the task that settles each

| item | settled by |
|---|---|
| does one binary load into DuckDB 2.0.0 and 2.0.1; which footer the build writes | T-22 |
| the `duckdb` crate's arrow major against the core's 59 | T-22 |
| the licences of everything the extension pulls in | T-22 (`cargo deny`), T-24 |
| how `vtab-arrow` (or our own code) converts each type of section 6 | T-24 |
| the walk slowdown from recording split points (target under 2 %, gate: the 10 % watch band) | T-23 |
| ordered parallel reads through the C API | T-26 |
| release channel and signing | Q-16, then T-27 |
