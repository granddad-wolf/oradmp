"""Exception classes and the factory the Rust binding calls (architecture 3.4, ADR-29, ADR-41).

The class of an error follows its stable code. Every exception carries ``.code`` (``"E-FRAMING"``), ``.offset``
(absolute file offset of the first byte that could not be accepted), ``.table`` (the table's name), ``.part``
(``(partition, subpartition)`` names or ``None``), ``.column`` (the column's name), ``.row`` (1-based within the
part), ``.length``, ``.type_code`` and ``.warning`` (the ``W-...`` code that ``strict=True`` turned into this
error, else ``None``). The text of the exception starts with the code and the offset
(``[E-FRAMING offset=0x1a2b] ...``) so that they survive a consumer that keeps only the message. No message
holds a value of the file. ``E-INTERNAL`` (``InternalError``) is a panic of the core caught by the binding; its message
is a fixed text without offset, table or row.
"""

from __future__ import annotations

from typing import Any

__all__ = [
    "ERROR_CLASSES",
    "WARNING_CLASSES",
    "FormatError",
    "InternalError",
    "OradmpError",
    "OradmpWarning",
    "StrictError",
    "UnsupportedCharset",
    "UnsupportedError",
    "UsageError",
    "ValueDecodeError",
    "make_error",
]


class OradmpError(Exception):
    """Base class of every error of the reader."""

    code: str
    offset: int | None
    table: str | None
    table_ordinal: int | None
    part: tuple[str | None, str | None] | None
    column: str | None
    row: int | None
    length: int | None
    type_code: int | None
    warning: str | None


class FormatError(OradmpError):
    """The file is not an exp file the reader can follow: framing, header or truncation."""


class UnsupportedError(OradmpError):
    """A type or layout the reader does not support (stops the file).

    Example (``E-UNSUPPORTED-LAYOUT``, [R8]): a conventional LONG or LONG RAW field that starts ``fb ff`` or
    ``fc ff`` (no test file has one).
    """


class UnsupportedCharset(UnsupportedError):
    """A character set the reader cannot decode (``W-CHARSET-UNSUPPORTED`` under ``strict``)."""


class ValueDecodeError(OradmpError):
    """The bytes of one value are not a value of the column type (``on_value_error`` decides)."""


class UsageError(OradmpError):
    """A wrong call: no such table or column, an ambiguous name, an invalid option, a changed file."""


class InternalError(OradmpError):
    """A panic in the Rust core was caught (``E-INTERNAL``, ADR-59). It is a bug of oradmp: report it with the
    version. The ``Dump`` or reader that raised it is unusable afterwards (every later call raises it again)."""


class StrictError(OradmpError):
    """``strict=True`` turned a warning outside the confirmed rules into an error (see ``.warning``)."""


class OradmpWarning(UserWarning):
    """A warning of the reader, issued through the :mod:`warnings` module; ``.detail`` is the ``DumpWarning``."""

    detail: Any


# error code -> class (a test walks ``_core.ERROR_CODES`` against this table); ``E-IO`` is ``OSError``
ERROR_CLASSES: dict[str, type[BaseException]] = {
    "E-NOT-EXP-FILE": FormatError,
    "E-BAD-HEADER": FormatError,
    "E-BAD-RECORD-LENGTH": FormatError,
    "E-MISSING-METRICS-LINE": FormatError,
    "E-FRAMING": FormatError,
    "E-TRUNCATED": FormatError,
    "E-UNSUPPORTED-TYPE": UnsupportedError,
    "E-UNSUPPORTED-LAYOUT": UnsupportedError,
    "E-UNSUPPORTED-CHARSET": UnsupportedCharset,
    "E-BAD-NUMBER": ValueDecodeError,
    "E-NUMBER-OVERFLOW": ValueDecodeError,
    "E-BAD-DATETIME": ValueDecodeError,
    "E-BAD-INTERVAL": ValueDecodeError,
    "E-BAD-ROWID": ValueDecodeError,
    "E-BAD-TEXT": ValueDecodeError,
    "E-LOB-TOO-LARGE": ValueDecodeError,
    "E-LOB-UNCONFIRMED": ValueDecodeError,
    "E-CONFIG": UsageError,
    "E-FILE-CHANGED": UsageError,
    "E-NO-SUCH-TABLE": UsageError,
    "E-AMBIGUOUS-TABLE-NAME": UsageError,
    "E-NO-SUCH-COLUMN": UsageError,
    "E-STRICT": StrictError,
    "E-INTERNAL": InternalError,
    "E-IO": OSError,
}


# warning code -> category (a test walks ``_core.WARNING_CODES`` against this table); every warning of the
# reader is an ``OradmpWarning`` when it is issued through the ``warnings`` module (``.detail.code`` tells them apart)
WARNING_CLASSES: dict[str, type[UserWarning]] = {
    code: OradmpWarning
    for code in (
        "W-UNKNOWN-INTRO",
        "W-TRAILING",
        "W-UNTESTED-VERSION",
        "W-UNTESTED-HEADER",
        "W-FILE-SIZE",
        "W-UNTESTED-LAYOUT",
        "W-DDL-FALLBACK",
        "W-CHARSET-UNSUPPORTED",
        "W-UNVERIFIED-TYPE",
        "W-LONG-LENGTH",
        "W-LONG-CHARSET",
        "W-LONGER-THAN-DECLARED",
        "W-NUMBER-ROUNDED",
        "W-NUMBER-NULLED",
        "W-VALUE-NULLED",
        "W-CHARSET-MISMATCH",
        "W-LOB-UNTESTED",
        "W-TSLTZ",
        "W-POSSIBLY-INCOMPLETE",
    )
}


def make_error(
    code: str,
    message: str,
    offset: int | None,
    table: str | None,
    table_ordinal: int | None,
    part: tuple[str | None, str | None] | None,
    column: str | None,
    row: int | None,
    length: int | None,
    type_code: int | None,
    warning: str | None,
    errno: int | None,
) -> BaseException:
    """Builds the exception for one core error (called by the binding; not raised here)."""
    cls = ERROR_CLASSES.get(code, OradmpError)
    err: BaseException = cls(message)
    if cls is OSError and errno is not None:
        err = OSError(errno, message)
    err.code = code  # type: ignore[attr-defined]
    err.offset = offset  # type: ignore[attr-defined]
    err.table = table  # type: ignore[attr-defined]
    err.table_ordinal = table_ordinal  # type: ignore[attr-defined]
    err.part = part  # type: ignore[attr-defined]
    err.column = column  # type: ignore[attr-defined]
    err.row = row  # type: ignore[attr-defined]
    err.length = length  # type: ignore[attr-defined]
    err.type_code = type_code  # type: ignore[attr-defined]
    err.warning = warning  # type: ignore[attr-defined]
    return err
