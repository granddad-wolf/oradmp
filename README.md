# oradmp

oradmp (not oradump, which is an unrelated project on PyPI): "dmp" as in Oracle dump files, legacy exp now and Data Pump later.

Read Oracle legacy `exp` dump files without an Oracle server. The reader is written in Rust; the Python package
delivers Apache Arrow (`pyarrow`).

Alpha: every release so far is a pre-release. Python 3.11 or later, pyarrow 15 or later. Install it with
`pip install "oradmp==0.1.0a4"` or `pip install --pre oradmp`.
Wheels: Linux x86_64 only (`manylinux_2_28`: RHEL / Oracle Linux 8 and later); see "Other platforms".

## What it reads

Table and owner mode, conventional and direct path, 11g and 18c files of same-version `exp`; the 16 column types
of the table below (NCHAR and `CHAR(n CHAR)` included, non-BMP text included) with the mapping of the table,
declared types matched by name and the schema in declared order; CLOB, NCLOB and BLOB (decoded in both forms in
AL32UTF8 and CL8MSWIN1251 databases; in a database of any other set a long-form CLOB value and any NCLOB value are
refused per value, `E-LOB-UNCONFIRMED`, or decoded with a warning on request); INTERVAL as
Interval(MonthDayNano) / Duration, or as text with `interval_as="string"`; ROWID and UROWID (physical and
logical) as text; partitioned and range-hash composite-partitioned tables as one table with its parts listed;
LONG and LONG RAW as LargeUtf8 / LargeBinary on both paths (direct-path chains; a LONG or LONG RAW above 100,000 bytes with a
warning). LONG and LONG RAW values on the conventional path are read in the three forms `exp` writes (one field up
to 65,529 bytes, a 4-byte length for 65,530 to 65,533 bytes, 4,000-byte chunks from 65,534 bytes; confirmed on 11g
and 18c up to 100,000 bytes), and a value of 65,522 to 65,524 bytes, a size no test file has, is read with a
`W-UNTESTED-LAYOUT` warning; the NUMBER overflow policy of your choice
and `decimal256(p,s)` overrides; the five character sets of the corpus with Oracle's map; scan and catalog with
exact row counts; `read_table` as a stream of batches into pyarrow; typed errors and warnings; the
structure-only `survey` with the per-candidate NUMBER profile and the LOB, LONG and partition counts.

It does **not** contain: BFILE, object, REF, collection or XML columns (a table with one stops the file),
`ReadOptions.partitions`, a partition-name column, a tested 10.2 client, parallel decoding, measured performance
figures or fuzzing results. Data Pump files, compression and encryption are not read.

## Install

From PyPI (Linux x86_64, glibc 2.28 or later, or a platform that can build the source distribution):

    pip install "oradmp==0.1.0a4"

Offline, from the wheel file of the release (Linux x86_64, glibc 2.28 or later):

    pip install oradmp-0.1.0a4-cp311-abi3-manylinux_2_28_x86_64.whl

From source (Python 3.11 or later, and Rust 1.99 or later on `PATH`; pip fetches maturin for the build):

    pip install oradmp-0.1.0a4.tar.gz     # the source distribution
    pip install .                          # in a clone of https://github.com/granddad-wolf/oradmp

Only pyarrow (15 or later) is required. pandas is optional (`Table.to_pandas()`).

The package installs the command `oradmp` (see "Command line").

## Other platforms

Wheels for Windows, macOS and other Linux systems are planned, not part of this release. Until then:

- Read the dump on a Linux x86_64 host and write Parquet there (`pyarrow.parquet.write_table`); pass
  `interval_as="string"` when a table has INTERVAL columns, because pyarrow's Parquet writer refuses the
  native interval type. The Parquet files can then be read anywhere.
- Or build from source as above. A source build on another platform is not tested; the test suite runs only
  from a clone of the repository (the source distribution contains neither the tests nor the test dumps).

Python 3.11 or later is required on every platform.

## Example

```python
import oradmp

with oradmp.open("scott.dmp") as dump:
    for t in dump.tables:  # TableInfo: name, columns, num_rows, parts
        print(t.ordinal, t.name, t.num_rows)

    reader = dump.read_table("EMP")  # a stream of pyarrow.RecordBatch
    table = reader.read_all()  # pyarrow.Table
    df = table.to_pandas()
```

`dump.read_table(...)` also supports the Arrow PyCapsule stream interface. Read it through the `with` block, so
that the warnings are issued when the block ends:

```python
import pyarrow as pa

with dump.read_table("EMP") as r:
    t = pa.RecordBatchReader.from_stream(r).read_all()
r.warnings  # complete; also issued through the warnings module at the end of the with
```

On the stream path an error reaches the consumer as a pyarrow exception (`pyarrow.ArrowInvalid`), not as
an `oradmp.OradmpError`: its text contains the code and the offset, for example `External error: [E-FRAMING
offset=0x1a2b] ...`; find the code with the pattern `\[(E-[A-Z0-9-]+)`. `r.warnings` is complete after such an
error.

After the export the reader is spent for iteration (`next(r)` raises `oradmp.UsageError`); `r.rows_read` and
`r.warnings` follow what the consumer has read. An error has `.code`, `.offset`, `.table`, `.column` and `.row`;
warnings are `oradmp.OradmpWarning`. Errors are `oradmp.OradmpError` subclasses (an
internal error is `oradmp.InternalError`, `E-INTERNAL`), except an operating-system error reading the file, which is an
`OSError` with `.code == "E-IO"`. No message holds a value from the file. Options of `oradmp.open`: `strict`,
`on_value_error`, `on_number_overflow`, `number_default`, `number_types`, `timestamp_unit`, `interval_as`,
`trim_char`, `max_lob_bytes`, `unconfirmed_lob`, `db_charset`, `client_charset`.

## Types

`interval_as` is `"native"` (default) or `"string"` and is fixed when the dump is opened.

| Oracle type | Arrow type | notes |
|---|---|---|
| `NUMBER(p, 0)`, p <= 18 | Int64 | exact or a value error |
| `NUMBER(p, 0)`, 19 <= p <= 38; `NUMBER(p, s)`; `NUMBER(*, s)` | Decimal128(p, s) | exact or a value error |
| `NUMBER` (no precision) | Decimal128(38, 10) by default; `decimal(p,s)`, `decimal256(p,s)`, `float64`, `string` by option | a value that does not fit is rounded below the 10th decimal (`on_number_overflow="round"`, counted in `W-NUMBER-ROUNDED`); an integer part that does not fit is an error |
| `FLOAT(b)` | Float64 | nearest double |
| `DATE` | Timestamp(second) | year below 1: `E-BAD-DATETIME` |
| `TIMESTAMP(f)` | Timestamp(microsecond), f <= 6; Timestamp(nanosecond), f 7 to 9 | |
| `TIMESTAMP(f) WITH TIME ZONE` | Timestamp(unit, "UTC") | the instant; the zone of the value is not delivered |
| `TIMESTAMP(f) WITH LOCAL TIME ZONE` | Timestamp(unit), no zone | as stored, `W-TSLTZ`: UTC or the database time zone is unknown |
| `VARCHAR2`, `CHAR`, `NVARCHAR2`, `NCHAR` | Utf8 | `CHAR` padding kept; `trim_char=True` removes it |
| `RAW(n)` | Binary | |
| `LONG` | LargeUtf8 | decoded with the database character set |
| `LONG RAW` | LargeBinary | |
| `CLOB`, `NCLOB` | LargeUtf8 | |
| `BLOB` | LargeBinary | |
| `INTERVAL YEAR TO MONTH`, `interval_as="native"` | Interval(MonthDayNano), months only | beyond 2^31 - 1 months (`YEAR(9)` only): `E-BAD-INTERVAL` |
| `INTERVAL DAY TO SECOND`, `interval_as="native"` | Duration(microsecond) for f <= 6 and p <= 8; Duration(nanosecond) for f 7 to 9 and p <= 5; otherwise Interval(MonthDayNano) | |
| every INTERVAL column, `interval_as="string"` | Utf8 | the `TO_CHAR` text, e.g. `+0002-06`, `-0001 02:03:04.500000000`; every value, none beyond a limit; field metadata `oradmp:conversion` = `to-char-text` |
| `ROWID`, `UROWID` | Utf8 | the 18-character text; a logical UROWID is `*` + base 64 |

Every column is nullable. Column metadata carries `oradmp:type_code`, `oradmp:oracle_type`, `oradmp:declared`
and `oradmp:max_bytes`.

## Limits

- Legacy `exp` only; Oracle 11g and 18c; no compression, no encryption.
- A table with a BFILE, object, REF, collection or XML column stops the file (tables before it stay readable).
- A conventional-path LONG or LONG RAW field that starts `fb ff` or `fc ff` (no test file has one) stops the file with
  `E-UNSUPPORTED-LAYOUT` (tables before it stay readable); a value of 65,522 to 65,524 bytes is read with
  `W-UNTESTED-LAYOUT`.
- Empty tables created with deferred segment creation are tested on 18c only (11g XE cannot create them); in an 11g
  Enterprise Edition dump they are read by the same rule, untested.
- A year below 1 in a DATE or TIMESTAMP is an error (or NULL with `on_value_error="null"`); dates from year 1 to
  1582 are the printed fields, not a claim about the instant.
- `TIMESTAMP WITH LOCAL TIME ZONE` has no zone and a warning.
- Only Linux x86_64 wheels (glibc 2.28 or later).

## pandas and Parquet

Observed with pyarrow 26.0.0 and pandas 3.0.6 (and with pyarrow 15.0.2 and pandas 2.2.3):

- `Table.to_pandas()` of a DATE column gives `datetime64[s]` with year 1 and year 9999 intact.
- `to_pandas(coerce_temporal_nanoseconds=True)` fails with `ArrowInvalid` ("out of bounds timestamp") for such
  values (the nanosecond range is about 1677 to 2262). Do not use it on these columns.
- `to_pandas(timestamp_as_object=True)` and `to_pylist()` give `datetime.datetime`.
- A TIMESTAMP(7 to 9) column is Timestamp(nanosecond); a 9999-12-31 value in it is outside the range and is
  `E-BAD-DATETIME` (NULL with `on_value_error="null"`).
- A value turned to NULL arrives as `NaT` in pandas and `None` in `to_pylist()`.
- `pyarrow.parquet.write_table` does **not** accept Interval(MonthDayNano)
  (`ArrowNotImplementedError: Unhandled type for Arrow to Parquet schema conversion: month_day_nano_interval`).
  Every INTERVAL YEAR TO MONTH column has that type, and so has an INTERVAL DAY TO SECOND column with p > 8 or with
  f >= 7 and p > 5. Open the dump with `interval_as="string"` to write them to Parquet. Duration columns
  (the usual `INTERVAL DAY(2) TO SECOND`) write and read back unchanged.

## Command line

`oradmp` (also `python -m oradmp`) is a thin tool over the Python API. A table is named by its name or by
`--ordinal N`; put the table right after the file name.
A file name that is not valid UTF-8 is accepted; messages write each such byte as `\xHH`.

    oradmp tables FILE                              # the tables with data
    oradmp columns FILE TABLE                       # the columns of one table
    oradmp head FILE TABLE [-n N]                   # the first N rows (default 10) as CSV
    oradmp export FILE TABLE -o PATH [--format csv|parquet]   # one table; -o - writes CSV to stdout
    oradmp export FILE --all -o DIR [--format csv|parquet]    # every table, one file each
    oradmp survey FILE [--no-names] [--no-read]     # counts about a dump (see Survey)

- `tables` and `columns` print tab-separated text with a header line; `true` / `false` for flags, an empty field
  for "none"; a tab, newline or backslash in a name is written `\t`, `\n`, `\\`.
- The read options of `oradmp.open` are options of the tool: `--strict`, `--on-value-error`,
  `--on-number-overflow`, `--number-default`, `--number-type TABLE.COLUMN=TYPE` (repeatable), `--interval-as`,
  `--timestamp-unit`, `--trim-char`, `--max-lob-bytes`, `--unconfirmed-lob`, `--db-charset`, `--client-charset`,
  and `--columns A,B`. One default differs from the API: **`--interval-as string`**, because Parquet and CSV cannot
  hold the native interval type (a column type that the CSV or Parquet writer refuses stops the export before the
  first row with `E-CONFIG`).
- `export` takes the format from `--format`, else from the suffix of `-o` (`.csv` or `.parquet`, in any case), else it
  stops with `E-CONFIG` (exit 2); `--all` writes CSV unless `--format parquet`; `-o -` writes CSV only.
- CSV is written by `pyarrow.csv`, with RAW, LONG RAW and BLOB values as upper-case hex. Parquet is written by
  `pyarrow.parquet` with its defaults; Parquet has no seconds unit, so a DATE column (Timestamp(second)) is stored as
  milliseconds (the same instants).
- An export writes `<target>.oradmp-partial` and renames it when the last batch is written; after an error or an
  interrupt nothing is left. An existing target needs `--overwrite`. With `--all`, a table is written to
  `<NAME>.csv` / `<NAME>.parquet` when its name has only letters, digits, `_`, `$`, `#` and no other table has
  the same name ignoring case, else to `table_<ordinal>.<ext>`; the folder must exist; the first error stops the
  run and files already written stay.
- **Data go to stdout, errors and warnings to stderr**: `oradmp: error: [E-TRUNCATED offset=0x...] ...` and
  `oradmp: warning: ...` (hidden by `-q`); no value of the file is ever in either.
- Exit codes: **0** done (warnings allowed); **1** the dump cannot be read as asked (format, unsupported, value or
  strict error; also a table that is not found in a scan that stopped: the stop error is printed, as `tables` does,
  because the table may lie behind the stop; in a complete scan it is a usage error, **2**); **2** usage (wrong option
  or value, unknown table or column, an existing target).
- Exit codes, continued: **3** I/O (a missing or unreadable dump, a folder that cannot be written, a closed stdout of
  `export -o -`); the line names the reason and the file, for example
  `oradmp: error: [E-IO] No such file or directory: FILE`; **4** internal error; **130** interrupted. A closed stdout
  of `head`, `tables` or `columns` ends quietly with 0. `survey` returns 0 or 1.
- `head` prints no warning of the table's read: the reader gives the counted warnings (for example
  `W-NUMBER-ROUNDED`) only at the end of a read, and `head` stops after N rows, so a value it shows may be rounded or
  NULL without a warning; `export` reports them. `oradmp --version` prints the version.

## Survey

`oradmp survey FILE` (or `python -m oradmp survey FILE`) prints counts about a dump and never a value: header, tables, types, the NUMBER
profile per column, LOB, LONG and partition counts, warnings. By default it reads every table once to list the
warnings; `--no-read` skips that pass. `--no-names` hides the user, the file name and the names of tables, columns
and partitions, so the output can be pasted into a report.

## License

Licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](https://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

The test dumps in `corpus/fixtures/` are generated from synthetic tables by the SQL in `corpus/sql/`.
They contain no real data and are not part of the wheel or the source distribution.

oradmp is an independent project. It is not affiliated with, endorsed by or sponsored by Oracle
Corporation. Oracle is a trademark or registered trademark of Oracle Corporation and/or its
affiliates.
