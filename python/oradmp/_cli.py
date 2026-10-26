"""The command-line tool ``oradmp`` (architecture 3.5, ADR-72, task T-18).

A thin layer over the public API: ``tables``, ``columns``, ``head``, ``export`` and ``survey``. It calls only
``oradmp`` (never ``oradmp._core``), reads through the iterator path so that every error is typed, writes data to
stdout only and errors and warnings to stderr only. It never prints a value of the file in a message, and it never
imports pandas.

Exit codes (ADR-72 item 8): 0 done; 1 the dump cannot be read as asked (``FormatError``, ``UnsupportedError``,
``ValueDecodeError``, ``StrictError``); 2 usage (argparse, ``UsageError``, the tool's own ``[E-CONFIG]`` refusals);
3 ``E-IO`` (``OSError``); 4 ``E-INTERNAL``; 130 an interrupt.
"""

from __future__ import annotations

import argparse
import contextlib
import errno
import os
import re
import sys
import warnings
from collections.abc import Iterator, Sequence
from typing import TYPE_CHECKING, Any

from . import _api, _survey
from ._errors import (
    FormatError,
    InternalError,
    OradmpError,
    OradmpWarning,
    StrictError,
    UnsupportedError,
    UsageError,
    ValueDecodeError,
    make_error,
)

if TYPE_CHECKING:
    import pyarrow as pa

__all__ = ["EXIT_CODES", "exit_code_for_class", "main", "table_file_names"]

PARTIAL_SUFFIX = ".oradmp-partial"
DEFAULT_HEAD_ROWS = 10

# error class -> exit code (ADR-72 item 8); a subclass takes the code of its nearest class in this table
EXIT_CODES: dict[type[BaseException], int] = {
    FormatError: 1,
    UnsupportedError: 1,
    ValueDecodeError: 1,
    StrictError: 1,
    UsageError: 2,
    OSError: 3,
    InternalError: 4,
}
EXIT_INTERRUPT = 130


def exit_code_for_class(cls: type[BaseException]) -> int | None:
    """The exit code of an error class, or ``None`` if the table has no entry for it or a base class."""
    for base in cls.__mro__:
        if base in EXIT_CODES:
            return EXIT_CODES[base]
    return None


def _exit_code(e: BaseException) -> int:
    code = exit_code_for_class(type(e))
    return 1 if code is None else code


class _State:
    """Module state of one run: a closed stdout is remembered here because pyarrow may re-wrap the exception."""

    def __init__(self) -> None:
        self.stdout_closed = False
        self.quiet = False
        self.file: str | None = None  # the dump path as given; the file of every I/O error of the API


_state = _State()


def _config_error(message: str) -> BaseException:
    text = f"[E-CONFIG] {message}"
    return make_error("E-CONFIG", text, None, None, None, None, None, None, None, None, None, None)


# --- output -------------------------------------------------------------------------------------------------


def _out(text: str) -> None:
    try:
        sys.stdout.write(text)
    except BrokenPipeError:
        _state.stdout_closed = True
        raise


def _flush() -> None:
    try:
        sys.stdout.flush()
    except BrokenPipeError:
        _state.stdout_closed = True
        raise


def _err(text: str) -> None:
    try:
        sys.stderr.write(text + "\n")
        sys.stderr.flush()
    except (OSError, ValueError):  # a closed stderr has nobody to tell
        pass


def _warn(text: str) -> None:
    if not _state.quiet:
        _err(f"oradmp: warning: {text}")


def _warn_all(ws: Sequence[_api.DumpWarning]) -> None:
    for w in ws:
        _warn(w.text)


def _escape(s: str) -> str:
    """Tab, newline and backslash in a name (ADR-72 item 5)."""
    return s.replace("\\", "\\\\").replace("\t", "\\t").replace("\n", "\\n")


def _cell(v: object) -> str:
    if v is None:
        return ""
    if isinstance(v, bool):
        return "true" if v else "false"
    return _escape(str(v))


def _row(values: Sequence[object]) -> str:
    return "\t".join(_cell(v) for v in values) + "\n"


class _StdoutSink:
    """A binary sink for pyarrow's writers on stdout; it remembers a closed pipe."""

    def __init__(self) -> None:
        _flush()
        self._raw = getattr(sys.stdout, "buffer", None)

    @property
    def closed(self) -> bool:
        return False

    def writable(self) -> bool:
        return True

    def write(self, data: bytes) -> int:
        try:
            if self._raw is None:
                sys.stdout.write(bytes(data).decode("utf-8"))
            else:
                self._raw.write(data)
        except BrokenPipeError:
            _state.stdout_closed = True
            raise
        return len(data)

    def flush(self) -> None:
        try:
            if self._raw is not None:
                self._raw.flush()
            sys.stdout.flush()
        except BrokenPipeError:
            _state.stdout_closed = True
            raise

    def close(self) -> None:
        self.flush()


# --- arguments ----------------------------------------------------------------------------------------------


def _common_parent() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser(add_help=False)
    g = p.add_argument_group("read options (as oradmp.open)")
    g.add_argument("--strict", action="store_true", help="a warning outside the confirmed rules becomes an error")
    g.add_argument("--on-value-error", metavar="raise|null", help="what a bad value does (default: raise)")
    g.add_argument(
        "--on-number-overflow", metavar="round|error|null", help="a NUMBER that does not fit (default: round)"
    )
    g.add_argument("--number-default", metavar="TYPE", help="type of NUMBER without precision (decimal(38,10), ...)")
    g.add_argument(
        "--number-type",
        action="append",
        default=[],
        metavar="TABLE.COLUMN=TYPE",
        help="type of one NUMBER column (repeatable)",
    )
    g.add_argument(
        "--interval-as",
        default="string",
        metavar="string|native",
        help="INTERVAL columns as text (default here; Parquet and CSV cannot hold the native type)",
    )
    g.add_argument("--timestamp-unit", metavar="UNIT", help="auto, s, ms, us or ns (default: auto)")
    g.add_argument("--trim-char", action="store_true", help="trim the padding of CHAR values")
    g.add_argument("--max-lob-bytes", type=int, metavar="N", help="refuse a LOB value larger than N stored bytes")
    g.add_argument("--unconfirmed-lob", metavar="error|decode", help="LOB values of an unconfirmed layout")
    g.add_argument("--db-charset", metavar="NAME", help="override the database character set")
    g.add_argument("--client-charset", metavar="NAME", help="override the client character set")
    p.add_argument("-q", "--quiet", action="store_true", help="do not print warnings")
    return p


def _table_args(p: argparse.ArgumentParser, *, optional: bool) -> None:
    p.add_argument("table", nargs="?" if optional else None, help="table name (or use --ordinal)")
    p.add_argument("--ordinal", type=int, metavar="N", help="1-based ordinal of the table instead of its name")


def _build_parser() -> argparse.ArgumentParser:
    common = _common_parent()
    parser = argparse.ArgumentParser(
        prog="oradmp", description="Read Oracle legacy exp dump files; Arrow, CSV, Parquet."
    )
    parser.add_argument("--version", action="version", version=f"oradmp {_version()}")
    sub = parser.add_subparsers(dest="command", required=True, metavar="COMMAND")

    s = sub.add_parser("tables", parents=[common], help="list the tables with data")
    s.add_argument("file")

    s = sub.add_parser("columns", parents=[common], help="show the columns of one table")
    s.add_argument("file")
    _table_args(s, optional=True)

    s = sub.add_parser("head", parents=[common], help="print the first rows of a table as CSV")
    s.add_argument("file")
    _table_args(s, optional=True)
    s.add_argument("-n", type=int, default=DEFAULT_HEAD_ROWS, metavar="N", help="rows (default 10)")
    s.add_argument("--columns", metavar="A,B,...", help="only these columns")

    s = sub.add_parser("export", parents=[common], help="write a table, or every table, to CSV or Parquet")
    s.add_argument("file")
    _table_args(s, optional=True)
    s.add_argument("--all", action="store_true", help="every table, one file each, into the folder given by -o")
    s.add_argument(
        "-o", "--output", required=True, metavar="PATH", help="target file, or folder with --all; - is stdout"
    )
    s.add_argument("--format", choices=["csv", "parquet"], help="default: from the suffix of -o (.csv, .parquet)")
    s.add_argument("--overwrite", action="store_true", help="replace an existing target")
    s.add_argument("--columns", metavar="A,B,...", help="only these columns")

    s = sub.add_parser("survey", help="print counts about a dump (never a value)")
    s.add_argument("file")
    s.add_argument("--no-names", action="store_true", help="hide table, column, partition and user names")
    s.add_argument("--no-read", action="store_true", help="count only; do not read every table leniently")
    return parser


def _version() -> str:
    from . import __version__

    return __version__


def _open_options(args: argparse.Namespace) -> dict[str, Any]:
    kw: dict[str, Any] = {"strict": args.strict, "interval_as": args.interval_as, "trim_char": args.trim_char}
    for name in (
        "on_value_error",
        "on_number_overflow",
        "number_default",
        "timestamp_unit",
        "max_lob_bytes",
        "unconfirmed_lob",
        "db_charset",
        "client_charset",
    ):
        value = getattr(args, name)
        if value is not None:
            kw[name] = value
    types: dict[str, str] = {}
    for spec in args.number_type:
        key, eq, value = spec.partition("=")
        if not eq or not key or not value:
            raise _config_error("--number-type is TABLE.COLUMN=TYPE")
        types[key] = value
    if types:
        kw["number_types"] = types
    return kw


def _table_spec(args: argparse.Namespace) -> str | int:
    if (args.table is None) == (args.ordinal is None):
        raise _config_error("name the table, or give --ordinal N (not both)")
    return args.table if args.table is not None else int(args.ordinal)


def _column_list(args: argparse.Namespace) -> list[str] | None:
    text = getattr(args, "columns", None)
    return None if text is None else text.split(",")


def _open_dump(args: argparse.Namespace) -> _api.Dump:
    dump = _api.open(args.file, **_open_options(args))
    try:
        dump.scan()
    except BaseException:
        _warn_all(dump.warnings)
        dump.close()
        raise
    _warn_all(dump.warnings)
    return dump


def _report_stop(cat: _api.Catalog) -> int:
    """A scan that stopped: the error after the tables before it, and its exit code."""
    if cat.end == "stopped" and cat.stop_error is not None:
        _report_error(cat.stop_error)
        return _exit_code(cat.stop_error)
    return 0


# --- commands -----------------------------------------------------------------------------------------------


def _cmd_tables(args: argparse.Namespace) -> int:
    dump = _open_dump(args)
    with contextlib.closing(dump):
        cat = dump.scan()
        _out(_row(["ordinal", "name", "rows", "columns", "parts", "path", "partitioned", "possibly_incomplete"]))
        for t in cat.tables:
            cells = [t.ordinal, t.name, t.num_rows, len(t.columns), len(t.parts), t.path, t.partitioned]
            _out(_row([*cells, t.possibly_incomplete]))
        _flush()
        return _report_stop(cat)


def _read_table(dump: _api.Dump, spec: str | int, **kwargs: Any) -> _api.BatchReader:
    """``dump.read_table``; a table not found in a stopped scan gives the stop error (it may lie behind the stop)."""
    try:
        return dump.read_table(spec, **kwargs)
    except OradmpError as e:
        cat = dump.scan()
        if e.code == "E-NO-SUCH-TABLE" and cat.end == "stopped" and cat.stop_error is not None:
            raise cat.stop_error from None
        raise


def _find_table(dump: _api.Dump, spec: str | int) -> _api.TableInfo:
    """The table the API would read for ``spec``; the API's own error (no such table, ambiguous) if there is none."""
    with _read_table(dump, spec):
        pass
    for t in dump.tables:
        if (t.ordinal == spec) if isinstance(spec, int) else (t.name == spec):
            return t
    raise _config_error("the table named by the API is not in the catalog")  # unreachable unless the API changes


def _cmd_columns(args: argparse.Namespace) -> int:
    spec = _table_spec(args)
    dump = _open_dump(args)
    with contextlib.closing(dump):
        table = _find_table(dump, spec)
        _out(_row(["position", "name", "oracle_type", "declared", "arrow_type", "not_null", "type_source"]))
        for c in table.columns:
            _out(_row([c.position, c.name, c.oracle_type, c.declared, c.arrow_type, c.not_null, c.type_source]))
        _flush()
    return 0


def _is_binary(t: Any) -> bool:
    import pyarrow as pa

    return bool(pa.types.is_binary(t) or pa.types.is_large_binary(t))


def _csv_schema(schema: pa.Schema) -> pa.Schema:
    """The schema of the CSV: binary columns become text (upper-case hex, as Oracle's ``RAWTOHEX``)."""
    import pyarrow as pa

    fields = []
    for f in schema:
        if _is_binary(f.type):
            t = pa.large_string() if pa.types.is_large_binary(f.type) else pa.string()
            fields.append(pa.field(f.name, t, f.nullable))
        else:
            fields.append(f)
    return pa.schema(fields)


def _csv_batch(batch: pa.RecordBatch, schema: pa.Schema) -> pa.RecordBatch:
    import pyarrow as pa

    if schema == batch.schema:
        return batch
    arrays = []
    for i, field in enumerate(schema):
        col = batch.column(i)
        if _is_binary(col.type):
            col = pa.array([None if v is None else v.hex().upper() for v in col.to_pylist()], type=field.type)
        arrays.append(col)
    return pa.RecordBatch.from_arrays(arrays, schema=schema)


def _probe_columns(schema: pa.Schema, make_writer: Any, what: str) -> None:
    """Stops before any row (``E-CONFIG``) if the writer refuses a column type: one null row per column is tried."""
    import io

    import pyarrow as pa

    for field in schema:
        one = pa.schema([field])
        sink = io.BytesIO()
        try:
            writer = make_writer(sink, one)
            writer.write_table(pa.table({field.name: pa.array([None], type=field.type)}, schema=one))
            writer.close()
        except (pa.ArrowNotImplementedError, pa.ArrowInvalid, pa.ArrowTypeError) as e:
            hint = " (try --interval-as string)" if "interval" in str(field.type).lower() else ""
            raise _config_error(
                f"the {what} writer of pyarrow {pa.__version__} does not support column {field.name} "
                f"of Arrow type {field.type}{hint}"
            ) from e


def _csv_writer(sink: Any, schema: pa.Schema) -> Any:
    import pyarrow.csv as pacsv

    return pacsv.CSVWriter(sink, schema)


def _parquet_module() -> Any:
    try:
        import pyarrow.parquet as pq
    except ImportError as e:
        raise _config_error("this pyarrow has no Parquet support") from e
    return pq


def _parquet_writer(sink: Any, schema: pa.Schema) -> Any:
    return _parquet_module().ParquetWriter(sink, schema)


def _write_batch(writer: Any, batch: pa.RecordBatch) -> None:
    """Writes one batch (the one place a test can interrupt)."""
    import pyarrow as pa

    writer.write_table(pa.Table.from_batches([batch]))


def _format_for(output: str, fmt: str | None, *, everything: bool) -> str:
    if fmt is not None:
        return fmt
    if everything or output == "-":
        return "csv"
    suffix = os.path.splitext(output)[1].lower()
    if suffix == ".csv":
        return "csv"
    if suffix == ".parquet":
        return "parquet"
    raise _config_error(
        "the format is not known: give --format csv or --format parquet, or end -o with .csv or .parquet"
    )


def _copy_table(reader: _api.BatchReader, sink: Any, fmt: str, *, limit: int | None = None) -> None:
    """Streams the batches of ``reader`` into ``sink``; stops after ``limit`` rows. Raises before the first row if the
    writer refuses a column type. The reader's warnings are printed whatever happens."""
    schema = reader.schema
    try:
        if fmt == "csv":
            out_schema = _csv_schema(schema)
            _probe_columns(out_schema, _csv_writer, "CSV")
            writer = _csv_writer(sink, out_schema)
        else:
            out_schema = schema
            _probe_columns(out_schema, _parquet_writer, "Parquet")
            writer = _parquet_writer(sink, out_schema)
        try:
            seen = 0
            for batch in reader:
                if limit is not None and seen + batch.num_rows > limit:
                    batch = batch.slice(0, limit - seen)
                if fmt == "csv":
                    batch = _csv_batch(batch, out_schema)
                _write_batch(writer, batch)
                seen += batch.num_rows
                if limit is not None and seen >= limit:
                    break
        finally:
            writer.close()
    finally:
        reader.close()
        _warn_all(reader.warnings)


def _cmd_head(args: argparse.Namespace) -> int:
    spec = _table_spec(args)
    if args.n < 0:
        raise _config_error("-n must not be negative")
    dump = _open_dump(args)
    with contextlib.closing(dump):
        reader = _read_table(dump, spec, columns=_column_list(args), batch_rows=max(args.n, 1))
        sink = _StdoutSink()
        _copy_table(reader, sink, "csv", limit=args.n)
        sink.flush()
    return 0


@contextlib.contextmanager
def _partial_file(target: str) -> Iterator[Any]:
    """A file ``<target>.oradmp-partial`` in the target's folder; renamed to ``target`` only if the block ends well,
    removed on any error or interrupt."""
    partial = target + PARTIAL_SUFFIX
    f = open(partial, "wb")  # noqa: SIM115 - closed below, on every path
    try:
        yield f
        f.close()
        os.replace(partial, target)
    except BaseException:
        with contextlib.suppress(OSError):
            f.close()
        with contextlib.suppress(OSError):
            os.unlink(partial)
        raise


_PLAIN_NAME = re.compile(r"[A-Za-z0-9_$#]+", re.ASCII)


def table_file_names(tables: Sequence[tuple[int, str]], ext: str) -> list[str]:
    """File names of ``--all`` (ADR-72 item 7): ``<NAME>.<ext>`` when the name matches ``[A-Za-z0-9_$#]+`` and is
    unique ignoring case among ``tables`` (pairs of ordinal and name), else ``table_<ordinal>.<ext>``."""
    lowered = [name.casefold() for _, name in tables]
    out = []
    for (ordinal, name), low in zip(tables, lowered, strict=True):
        if _PLAIN_NAME.fullmatch(name) and lowered.count(low) == 1:
            out.append(f"{name}.{ext}")
        else:
            out.append(f"table_{ordinal}.{ext}")
    return out


def _refuse_existing(path: str, overwrite: bool) -> None:
    if not overwrite and os.path.lexists(path):
        raise _config_error(f"the target {_path_text(path)} exists; give --overwrite to replace it")


def _export_one(dump: _api.Dump, spec: str | int, target: str, fmt: str, args: argparse.Namespace) -> int:
    reader = _read_table(dump, spec, columns=_column_list(args))
    if target == "-":
        if fmt != "csv":
            reader.close()
            raise _config_error("Parquet cannot be written to stdout")
        sink = _StdoutSink()
        _copy_table(reader, sink, "csv")
        sink.flush()
        return reader.rows_read
    with _partial_file(target) as f:
        _copy_table(reader, f, fmt)
    return reader.rows_read


def _cmd_export(args: argparse.Namespace) -> int:
    if args.all:
        if args.table is not None or args.ordinal is not None:
            raise _config_error("--all and a table (or --ordinal) exclude each other")
        if args.output == "-":
            raise _config_error("--all writes into a folder: give -o DIR")
    else:
        spec = _table_spec(args)
    fmt = _format_for(args.output, args.format, everything=args.all)
    if fmt == "parquet":
        _parquet_module()
    if not args.all and args.output != "-":
        _refuse_existing(args.output, args.overwrite)
    dump = _open_dump(args)
    with contextlib.closing(dump):
        if not args.all:
            _export_one(dump, spec, args.output, fmt, args)
            return 0
        cat = dump.scan()
        ext = "csv" if fmt == "csv" else "parquet"
        names = table_file_names([(t.ordinal, t.name) for t in cat.tables], ext)
        if len({n.casefold() for n in names}) != len(names):
            raise _config_error("two tables would get the same file name")
        targets = [os.path.join(args.output, n) for n in names]
        for target in targets:
            _refuse_existing(target, args.overwrite)
        _out(_row(["ordinal", "name", "file", "rows"]))
        _flush()
        for t, name, target in zip(cat.tables, names, targets, strict=True):
            rows = _export_one(dump, t.ordinal, target, fmt, args)
            _out(_row([t.ordinal, t.name, name, rows]))
            _flush()
        return _report_stop(cat)


def _cmd_survey(args: argparse.Namespace) -> int:
    return _survey.write_survey(args.file, sys.stdout, names=not args.no_names, read=not args.no_read)


_COMMANDS = {
    "tables": _cmd_tables,
    "columns": _cmd_columns,
    "head": _cmd_head,
    "export": _cmd_export,
    "survey": _cmd_survey,
}


# --- errors and main ----------------------------------------------------------------------------------------


_path_text = _survey.path_text


def _api_io_text(e: OSError, strerror: str, file: str | None) -> str:
    """ADR-77 item 4: the API's bracketed prefix, the operating system's text for the error number, the file."""
    prefix, bracket, rest = strerror.partition("]")
    if not bracket or file is None:
        return strerror
    reason = os.strerror(e.errno) if e.errno is not None else rest.strip() or "I/O error"
    return f"{prefix}] {reason}: {_path_text(file)}"


def _filename_text(name: str | bytes | int) -> str:
    """A ``filename`` of an ``OSError`` in a message (ADR-82 item 3 as amended, review 06b SF-4): as a path."""
    return _path_text(os.fsdecode(name) if isinstance(name, bytes) else str(name))


def _error_text(e: BaseException, file: str | None = None) -> str:
    if isinstance(e, OSError):
        strerror = e.strerror
        if getattr(e, "code", None) == "E-IO":  # an error of the API (make_error sets code on every one)
            if isinstance(strerror, str):
                return _api_io_text(e, strerror, file)
            if len(e.args) == 1 and isinstance(e.args[0], str):
                return _api_io_text(e, e.args[0], file)
        if isinstance(strerror, str) and strerror.startswith("["):
            return strerror
        detail = strerror or str(e)
        names = [_filename_text(n) for n in (e.filename, getattr(e, "filename2", None)) if n]
        return f"[E-IO] {detail}" + (": " + " -> ".join(names) if names else "")
    return str(e)


def _report_error(e: BaseException) -> None:
    _err(f"oradmp: error: {_error_text(e, _state.file)}")


def _is_closed_pipe(e: BaseException) -> bool:
    return isinstance(e, BrokenPipeError) or getattr(e, "errno", None) == errno.EPIPE or _state.stdout_closed


def _silence_stdout() -> None:
    """After a closed pipe: point fd 1 at the null device so that the flush at exit cannot fail (Python docs)."""
    try:
        fd = sys.stdout.fileno()
        os.dup2(os.open(os.devnull, os.O_WRONLY), fd)
    except (AttributeError, ValueError, OSError):  # stdout without a descriptor (captured)
        pass


def main(argv: Sequence[str] | None = None) -> int:
    """Runs the tool; returns the exit code (the console script passes it to ``sys.exit``)."""
    _state.stdout_closed = False
    parser = _build_parser()
    try:
        args = parser.parse_args(list(sys.argv[1:] if argv is None else argv))
    except SystemExit as e:  # argparse: 2 for a usage error, 0 for --help
        return e.code if isinstance(e.code, int) else 2
    _state.quiet = bool(getattr(args, "quiet", False))
    _state.file = getattr(args, "file", None)
    command = _COMMANDS[args.command]
    try:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore", OradmpWarning)
            return command(args)
    except KeyboardInterrupt:
        _err("oradmp: interrupted")
        return EXIT_INTERRUPT
    except Exception as e:
        if _is_closed_pipe(e):
            _silence_stdout()
            if args.command == "export" and args.output == "-":
                _err("oradmp: error: [E-IO] stdout is closed")
                return EXIT_CODES[OSError]
            return 0
        if isinstance(e, (OradmpError, OSError)):
            _report_error(e)
            return _exit_code(e)
        raise  # a bug of the tool: Python's traceback
