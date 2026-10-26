"""Shared helpers of the pytest files (task T-12): fixture paths, patched copies below ``tmp_path``, and the
canonical digest that ``oradmp._core`` computes in Rust (``oradmp-py/src/digest.rs``) recomputed from pyarrow.

Fixtures in ``corpus/fixtures`` are read-only: tests copy a file and patch the copy.
"""

from __future__ import annotations

import struct
from collections.abc import Callable
from pathlib import Path

import pyarrow as pa
import pytest

ROOT = Path(__file__).resolve().parent.parent
FIXTURES = ROOT / "corpus" / "fixtures"

_BASIS = 0xCBF29CE484222325
_PRIME = 0x100000001B3
_MASK = (1 << 64) - 1


def fixture(rel: str) -> str:
    """The path of a fixture (``"18c/t_num_exp_18c.dmp"``) as a string."""
    p = FIXTURES / rel
    assert p.is_file(), p
    return str(p)


def make_patched(tmp_path: Path, rel: str, name: str, patch: Callable[[bytearray], None]) -> str:
    """A copy of a fixture below ``tmp_path`` after ``patch`` changed its bytes."""
    data = bytearray((FIXTURES / rel).read_bytes())
    patch(data)
    p = tmp_path / name
    p.write_bytes(bytes(data))
    return str(p)


def find_one(data: bytes | bytearray, pattern: bytes) -> int:
    """The only position of ``pattern`` in ``data``."""
    first = data.find(pattern)
    assert first >= 0, pattern
    assert data.find(pattern, first + 1) < 0, pattern
    return first


@pytest.fixture
def patched(tmp_path: Path) -> Callable[[str, str, Callable[[bytearray], None]], str]:
    def make(rel: str, name: str, patch: Callable[[bytearray], None]) -> str:
        return make_patched(tmp_path, rel, name, patch)

    return make


def _feed(h: int, data: bytes) -> int:
    for b in data:
        h = ((h ^ b) * _PRIME) & _MASK
    return h


def _unscaled(d: object, scale: int) -> int:
    sign, digits, exponent = d.as_tuple()  # type: ignore[attr-defined]
    n = int("".join(map(str, digits)) or "0")
    shift = int(exponent) + scale
    n = n * 10**shift if shift >= 0 else n // 10 ** (-shift)
    return int(-n if sign else n)


def column_digest(col: pa.ChunkedArray) -> tuple[int, int, int]:
    """(rows, nulls, digest) of one column, the canonical form of ``digest.rs``."""
    h = _BASIS
    rows = nulls = 0
    t = col.type
    for chunk in col.chunks:
        if pa.types.is_timestamp(t) or pa.types.is_duration(t):
            values = chunk.cast(pa.int64()).to_pylist()
        else:
            values = chunk.to_pylist()
        for v in values:
            rows += 1
            if v is None:
                nulls += 1
                h = _feed(h, b"\x00")
                continue
            h = _feed(h, b"\x01")
            if pa.types.is_int64(t) or pa.types.is_timestamp(t) or pa.types.is_duration(t):
                h = _feed(h, struct.pack("<q", v))
            elif pa.types.is_float64(t):
                h = _feed(h, struct.pack("<d", v))
            elif pa.types.is_decimal128(t):
                h = _feed(h, _unscaled(v, t.scale).to_bytes(16, "little", signed=True))
            elif pa.types.is_decimal256(t):
                h = _feed(h, _unscaled(v, t.scale).to_bytes(32, "little", signed=True))
            elif pa.types.is_interval(t):
                h = _feed(h, struct.pack("<iiq", v.months, v.days, v.nanoseconds))
            elif pa.types.is_string(t) or pa.types.is_large_string(t):
                b = v.encode("utf-8")
                h = _feed(_feed(h, struct.pack("<Q", len(b))), b)
            elif pa.types.is_binary(t) or pa.types.is_large_binary(t):
                h = _feed(_feed(h, struct.pack("<Q", len(v))), v)
            else:  # pragma: no cover
                raise AssertionError(f"type {t}")
    return rows, nulls, h


def table_digest(table: pa.Table) -> list[tuple[str, int, int, int]]:
    """``(name, rows, nulls, digest)`` per column of a pyarrow table."""
    return [(name, *column_digest(table.column(i))) for i, name in enumerate(table.column_names)]
