"""The public API (architecture 3.4): ``open`` -> ``Dump`` -> ``BatchReader``.

``oradmp._core`` (Rust) does the work and returns plain data; this module wraps it in classes and dataclasses,
issues the warnings of the reader through the :mod:`warnings` module and keeps the per-read defaults given to
:func:`open`.
"""

from __future__ import annotations

import os
import warnings
from collections.abc import Iterator, Mapping
from dataclasses import dataclass, field
from types import TracebackType
from typing import TYPE_CHECKING, Any

from . import _core
from ._errors import OradmpWarning, make_error

if TYPE_CHECKING:
    import pyarrow as pa

__all__ = [
    "BatchReader",
    "Catalog",
    "ColumnInfo",
    "Dump",
    "DumpInfo",
    "DumpWarning",
    "LobStats",
    "LongStats",
    "PartInfo",
    "TableInfo",
    "open",
]

DEFAULT_BATCH_ROWS = 65_536
DEFAULT_BATCH_BYTES = 64 * 2**20
DEFAULT_MAX_LOB_BYTES = 256 * 2**20


@dataclass(frozen=True, slots=True)
class DumpWarning:
    """One warning of the reader. ``count`` and the two figures after it belong to the once-per-column ones."""

    code: str
    offset: int | None
    table: str | None
    table_ordinal: int | None
    part: tuple[str | None, str | None] | None
    column: str | None
    row: int | None
    length: int | None
    message: str
    count: int | None
    count_to_zero: int | None
    max_digits_dropped: int | None
    text: str

    def __str__(self) -> str:
        return self.text


@dataclass(frozen=True, slots=True)
class DumpInfo:
    """What the first record says."""

    format: str
    version: str
    user: str
    user_possibly_damaged: bool
    mode: str
    record_length: int
    direct_flag: bool
    client_charset_id: int
    client_charset: str | None
    db_charset_id: int
    db_charset: str | None
    national_charset_id: int
    national_charset: str | None
    text_converted_at_export: bool
    db_charset_override: str | None
    client_charset_override: str | None
    file_size: int


@dataclass(frozen=True, slots=True)
class LobStats:
    """Counts of the LOB values of one column (counts only)."""

    values: int
    short: int
    empty: int
    long: int
    stored_bytes: int
    largest: int


@dataclass(frozen=True, slots=True)
class LongStats:
    """Counts of the LONG / LONG RAW values of one column.

    ``chained``: direct-path values in more than one piece. ``chunked``: values in the conventional
    ``fd ff`` chunked form. ``sizes_65521_to_65534``: values of 65,521 to 65,534 bytes, any form or path.
    """

    values: int
    chained: int
    largest: int
    above_32000: int
    zero_pieces: int
    chunked: int
    sizes_65521_to_65534: int


@dataclass(frozen=True, slots=True)
class ColumnInfo:
    """One column: descriptor, declared type, Arrow type (as text) and where the type came from."""

    position: int
    file_position: int
    name: str
    name_possibly_damaged: bool
    type_code: int
    oracle_type: str
    max_bytes: int
    charset_id: int | None
    form: int | None
    declared: str | None
    declared_known: bool
    not_null: bool
    arrow_type: str
    type_source: str
    lob: LobStats | None
    long: LongStats | None


@dataclass(frozen=True, slots=True)
class PartInfo:
    """One binary part of a table (the whole table, or one partition or subpartition)."""

    partition: str | None
    subpartition: str | None
    path: str
    num_rows: int
    rows_offset: int
    rows_end_offset: int
    end_marker: str
    file_order: list[int]


@dataclass(frozen=True, slots=True)
class TableInfo:
    """One table with data."""

    ordinal: int
    name: str
    name_raw: bytes
    name_possibly_damaged: bool
    columns: list[ColumnInfo]
    ddl_fallback: bool
    path: str
    parts: list[PartInfo]
    partitioned: bool
    subpartitioned: bool
    num_rows: int
    possibly_incomplete: bool
    ddl: str
    ddl_lossy: bool


@dataclass(frozen=True, slots=True)
class StoppedTable:
    """The table in which the scan stopped."""

    ordinal: int
    name: str
    table_line_offset: int


@dataclass(frozen=True, slots=True)
class ItemCounts:
    """Counts of items of the scan; ``g1_fields`` are the fields read after an unknown introducing line."""

    lines: int
    g1_fields: int
    per_entry: list[tuple[str, int, int, int]]


@dataclass(frozen=True, slots=True)
class Catalog:
    """The result of the scan. ``end`` is ``"complete"`` or ``"stopped"``; ``stop_error`` is the (unraised) error."""

    tables: list[TableInfo]
    end: str
    stop_error: BaseException | None
    stopped_table: StoppedTable | None
    end_offset: int | None
    item_counts: ItemCounts


def _warning(d: dict[str, Any]) -> DumpWarning:
    return DumpWarning(**d)


def _column(d: dict[str, Any]) -> ColumnInfo:
    d = dict(d)
    d["lob"] = LobStats(**d["lob"]) if d["lob"] is not None else None
    d["long"] = LongStats(**d["long"]) if d["long"] is not None else None
    return ColumnInfo(**d)


def _table(d: dict[str, Any]) -> TableInfo:
    d = dict(d)
    d["columns"] = [_column(c) for c in d["columns"]]
    d["parts"] = [PartInfo(**p) for p in d["parts"]]
    return TableInfo(**d)


def _catalog(d: dict[str, Any]) -> Catalog:
    ic = d["item_counts"]
    return Catalog(
        tables=[_table(t) for t in d["tables"]],
        end=d["end"],
        stop_error=d["stop_error"],
        stopped_table=StoppedTable(**d["stopped_table"]) if d["stopped_table"] is not None else None,
        end_offset=d["end_offset"],
        item_counts=ItemCounts(ic["lines"], ic["g1_fields"], list(ic["per_entry"])),
    )


def _issue(ws: list[DumpWarning], up: int) -> None:
    """Issues each warning once through the :mod:`warnings` module. ``up`` counts the frames from the caller of
    ``_issue_new`` to the user's line (1: the caller of ``_issue_new`` was called by the user), so that the
    warning's ``filename`` and ``lineno`` are the user's (review 03 N-2)."""
    for w in ws:
        w_instance = OradmpWarning(w.text)
        w_instance.detail = w
        warnings.warn(w_instance, stacklevel=up + 3)


class BatchReader:
    """The batches of one table: an iterator of :class:`pyarrow.RecordBatch` and a context manager.

    The counted warnings (``W-NUMBER-ROUNDED``, ``W-VALUE-NULLED``, ...) exist once the iteration has ended
    (``StopIteration`` or a raised error); the reader issues the new warnings through :mod:`warnings` at that
    moment, once each. ``__arrow_c_stream__`` hands the whole reader to any Arrow consumer (ADR-55): the reader is
    spent for iteration afterwards (``next()`` and :meth:`read_all` raise :class:`UsageError`), ``rows_read`` and
    ``warnings`` follow the consumer's reads, the warnings are complete once the consumer has read to the end,
    and :meth:`close` (or leaving the ``with`` block) issues the ones not yet issued. An error inside the stream
    arrives as a message that contains its code and offset.
    """

    def __init__(self, core: Any) -> None:
        self._core = core
        self._issued = 0
        self._schema: pa.Schema | None = None

    @property
    def schema(self) -> pa.Schema:
        """The Arrow schema of the batches."""
        if self._schema is None:
            self._schema = self._core.schema()
        return self._schema

    @property
    def rows_read(self) -> int:
        """Rows read so far, all parts."""
        return int(self._core.rows_read())

    @property
    def warnings(self) -> list[DumpWarning]:
        """The warnings of this read so far; the counted ones are complete once the iteration has ended."""
        return [_warning(w) for w in self._core.warnings()]

    def _issue_new(self, up: int) -> None:
        ws = self.warnings
        new = ws[self._issued :]
        self._issued = len(ws)
        _issue(new, up)

    def __iter__(self) -> Iterator[pa.RecordBatch]:
        return self

    def __next__(self) -> pa.RecordBatch:
        try:
            batch = self._core.next_batch()
        except BaseException:
            self._issue_new(1)
            raise
        if batch is None:
            self._issue_new(1)
            raise StopIteration
        return batch

    def read_all(self) -> pa.Table:
        """All remaining batches as one :class:`pyarrow.Table` (materialises). Raises :class:`UsageError` after
        the reader was exported to an Arrow stream."""
        import pyarrow as pa

        batches: list[pa.RecordBatch] = []
        try:
            while (batch := self._core.next_batch()) is not None:
                batches.append(batch)
        except BaseException:
            self._issue_new(1)
            raise
        self._issue_new(1)
        return pa.Table.from_batches(batches, schema=self.schema)

    def __arrow_c_stream__(self, requested_schema: object = None) -> object:
        """The Arrow PyCapsule Interface: any Arrow consumer can read the table from this object."""
        return self._core.arrow_c_stream(requested_schema)

    def close(self) -> None:
        """Releases the file handle, issues the warnings not yet issued; the warnings and the row count stay
        readable. A stream exported earlier fails on its next pull with ``[E-CONFIG`` (unless it had ended)."""
        self._close()

    def _close(self) -> None:
        self._core.close()
        self._issue_new(2)

    def __enter__(self) -> BatchReader:
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc: BaseException | None,
        tb: TracebackType | None,
    ) -> None:
        self._close()


@dataclass(slots=True)
class _Defaults:
    """The per-read options given to :func:`open`; ``read_table`` overrides them per call."""

    on_value_error: str = "raise"
    on_number_overflow: str = "round"
    max_lob_bytes: int = DEFAULT_MAX_LOB_BYTES
    unconfirmed_lob: str = "error"
    trim_char: bool = False
    extra: dict[str, Any] = field(default_factory=dict)


class Dump:
    """An opened dump file (context manager). Open with :func:`open`."""

    def __init__(self, core: Any, path: str, defaults: _Defaults) -> None:
        self._core = core
        self._path = path
        self._defaults = defaults
        self._info: DumpInfo | None = None
        self._catalog: Catalog | None = None
        self._issued = 0
        self._issue_new(2)

    def _issue_new(self, up: int) -> None:
        ws = self.warnings
        new = ws[self._issued :]
        self._issued = len(ws)
        _issue(new, up)

    @property
    def path(self) -> str:
        """The path given to :func:`open`."""
        return self._path

    @property
    def info(self) -> DumpInfo:
        """The first record: format, version, user, mode, record length, character set ids and names, size."""
        if self._info is None:
            self._info = DumpInfo(**self._core.info())
        return self._info

    def scan(self) -> Catalog:
        """Walks the file once (GIL released) and caches the catalog; the scan's warnings are issued once."""
        if self._catalog is None:
            self._catalog = _catalog(self._core.scan())
            self._issue_new(1)
        return self._catalog

    @property
    def tables(self) -> list[TableInfo]:
        """The tables with data (runs the scan once)."""
        return self.scan().tables

    @property
    def warnings(self) -> list[DumpWarning]:
        """The warnings of the header and of the scan (not those of a read)."""
        return [_warning(w) for w in self._core.warnings()]

    def read_table(
        self,
        table: str | int,
        *,
        columns: list[str] | None = None,
        batch_rows: int = DEFAULT_BATCH_ROWS,
        batch_bytes: int = DEFAULT_BATCH_BYTES,
        on_value_error: str | None = None,
        on_number_overflow: str | None = None,
        max_lob_bytes: int | None = None,
        unconfirmed_lob: str | None = None,
        trim_char: bool | None = None,
    ) -> BatchReader:
        """A reader for one table, by name or by 1-based ordinal.

        ``on_value_error``, ``on_number_overflow``, ``max_lob_bytes``, ``unconfirmed_lob`` and ``trim_char`` are
        per-read overrides: ``None`` means the value given to :func:`open`.
        """
        d = self._defaults
        options: dict[str, Any] = {
            "columns": list(columns) if columns is not None else None,
            "batch_rows": batch_rows,
            "batch_bytes": batch_bytes,
            "on_value_error": d.on_value_error if on_value_error is None else on_value_error,
            "on_number_overflow": d.on_number_overflow if on_number_overflow is None else on_number_overflow,
            "max_lob_bytes": d.max_lob_bytes if max_lob_bytes is None else max_lob_bytes,
            "unconfirmed_lob": d.unconfirmed_lob if unconfirmed_lob is None else unconfirmed_lob,
            "trim_char": d.trim_char if trim_char is None else trim_char,
        }
        if isinstance(table, bool) or not isinstance(table, (str, int)):
            raise _usage("a table is named by a str or an int ordinal")
        name = table if isinstance(table, str) else None
        ordinal = None if isinstance(table, str) else table
        self.scan()
        return BatchReader(self._core.read_table(name, ordinal, options))

    def read_all(self, *, columns: list[str] | None = None) -> dict[str, pa.Table]:
        """Every table as a :class:`pyarrow.Table` (materialises; for small files). A name that two tables share
        is suffixed with ``#<ordinal>``."""
        names = [t.name for t in self.tables]
        out: dict[str, pa.Table] = {}
        for t in self.tables:
            key = t.name if names.count(t.name) == 1 else f"{t.name}#{t.ordinal}"
            with self.read_table(t.ordinal, columns=columns) as r:
                out[key] = r.read_all()
        return out

    def _survey(self, read: bool) -> dict[str, Any]:
        """The survey's data (see ``oradmp._survey``)."""
        return self._core.survey(read)  # type: ignore[no-any-return]

    def close(self) -> None:
        """Drops the catalog and marks the object closed; readers already handed out stay valid."""
        self._core.close()

    @property
    def closed(self) -> bool:
        """True after :meth:`close`."""
        return bool(self._core.is_closed())

    def __enter__(self) -> Dump:
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc: BaseException | None,
        tb: TracebackType | None,
    ) -> None:
        self.close()


def _usage(message: str) -> BaseException:
    return make_error("E-CONFIG", f"[E-CONFIG] {message}", None, None, None, None, None, None, None, None, None, None)


def _number_types(spec: Mapping[str, object] | None) -> dict[str, tuple[str, str | None]] | None:
    """``{"TABLE.COLUMN": "int64" | ... | ("decimal(p,s)", "round" | "error" | "null")}`` as (type, policy) pairs."""
    if spec is None:
        return None
    out: dict[str, tuple[str, str | None]] = {}
    for key, value in spec.items():
        if isinstance(value, str):
            out[key] = (value, None)
        elif isinstance(value, tuple) and len(value) == 2 and all(isinstance(v, str) for v in value):
            out[key] = (value[0], value[1])
        else:
            raise _usage("a number_types entry is a type string or a (type, policy) pair of strings")
    return out


def open(
    path: str | os.PathLike[str],
    *,
    strict: bool = False,
    on_value_error: str = "raise",
    db_charset: str | None = None,
    client_charset: str | None = None,
    number_default: str = "decimal(38,10)",
    on_number_overflow: str = "round",
    number_types: Mapping[str, object] | None = None,
    timestamp_unit: str = "auto",
    interval_as: str = "native",
    trim_char: bool = False,
    max_lob_bytes: int = DEFAULT_MAX_LOB_BYTES,
    unconfirmed_lob: str = "error",
    extra_intro_lines: list[str] | None = None,
    max_line_bytes: int | None = None,
) -> Dump:
    """Opens a legacy ``exp`` dump file (reads the header only).

    ``strict=True`` makes every warning outside the confirmed rules an error (:class:`StrictError`).
    ``on_value_error`` (``"raise"`` or ``"null"``), ``on_number_overflow`` (``"round"``, ``"error"``, ``"null"``),
    ``max_lob_bytes``, ``unconfirmed_lob`` (``"error"`` or ``"decode"``) and ``trim_char`` are the defaults of
    :meth:`Dump.read_table`, which accepts them as per-read overrides.

    ``interval_as`` is ``"native"`` (Interval(MonthDayNano) / Duration) or ``"string"`` (every INTERVAL column is
    Utf8 holding the ``TO_CHAR`` text; Parquet cannot store the native interval type). It is fixed for the life
    of the dump because it changes the schema.
    """
    defaults = _Defaults(
        on_value_error=on_value_error,
        on_number_overflow=on_number_overflow,
        max_lob_bytes=max_lob_bytes,
        unconfirmed_lob=unconfirmed_lob,
        trim_char=trim_char,
    )
    _core.check_read_options(
        {
            "on_value_error": on_value_error,
            "on_number_overflow": on_number_overflow,
            "max_lob_bytes": max_lob_bytes,
            "unconfirmed_lob": unconfirmed_lob,
            "trim_char": trim_char,
        }
    )
    options: dict[str, Any] = {
        "strict": strict,
        "db_charset": db_charset,
        "client_charset": client_charset,
        "number_default": number_default,
        "number_types": _number_types(number_types),
        "timestamp_unit": timestamp_unit,
        "interval_as": interval_as,
        "extra_intro_lines": extra_intro_lines,
        "max_line_bytes": max_line_bytes,
    }
    p = os.fspath(path)
    return Dump(_core.open(p, options), p, defaults)
