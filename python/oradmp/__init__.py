"""oradmp: read Oracle legacy ``exp`` dump files without an Oracle server; Apache Arrow out.

>>> import oradmp
>>> with oradmp.open("dump.dmp") as dump:          # doctest: +SKIP
...     for batch in dump.read_table("EMP"):        # pyarrow.RecordBatch
...         ...

See :func:`open`, :class:`Dump`, :class:`BatchReader`. ``python -m oradmp survey FILE`` prints counts about a
dump (never a value).
"""

from importlib import metadata as _metadata

from ._api import (
    BatchReader,
    Catalog,
    ColumnInfo,
    Dump,
    DumpInfo,
    DumpWarning,
    LobStats,
    LongStats,
    PartInfo,
    TableInfo,
    open,
)
from ._errors import (
    ERROR_CLASSES,
    WARNING_CLASSES,
    FormatError,
    InternalError,
    OradmpError,
    OradmpWarning,
    StrictError,
    UnsupportedCharset,
    UnsupportedError,
    UsageError,
    ValueDecodeError,
)

try:
    __version__ = _metadata.version("oradmp")
except _metadata.PackageNotFoundError:  # the folder was copied without installing (ADR-54 item 7)
    __version__ = "0+unknown"

__all__ = [
    "ERROR_CLASSES",
    "WARNING_CLASSES",
    "BatchReader",
    "Catalog",
    "ColumnInfo",
    "Dump",
    "DumpInfo",
    "DumpWarning",
    "FormatError",
    "InternalError",
    "LobStats",
    "LongStats",
    "OradmpError",
    "OradmpWarning",
    "PartInfo",
    "StrictError",
    "TableInfo",
    "UnsupportedCharset",
    "UnsupportedError",
    "UsageError",
    "ValueDecodeError",
    "__version__",
    "open",
]
