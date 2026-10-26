"""Task T-13 (a): ``interval_as`` (ADR-47). ``"string"`` makes every INTERVAL column Utf8 (the ``TO_CHAR`` text of
rule Y10, equal to ``truth_misc.txt`` / ``truth_misc2.txt``, checked cell by cell in ``t13_interval.rs``); that is
what lets ``pyarrow.parquet.write_table`` store the column. The assertions state what pyarrow 15.0.2 and 26.0.0 do; with
``"native"`` the Parquet writer refuses Interval(MonthDayNano)."""

from __future__ import annotations

import io

import oradmp
import pyarrow as pa
import pyarrow.parquet as pq
import pytest
from conftest import fixture

pytestmark = pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")


def read(rel: str, table: str, **options: str) -> pa.Table:
    with oradmp.open(fixture(rel), **options) as dump:  # type: ignore[arg-type]
        return dump.read_table(table).read_all()


def test_string_mode_writes_to_parquet_and_reads_back_equal() -> None:
    t = read("18c/misc_exp_18c.dmp", "T_INTERVAL", interval_as="string")
    assert t.schema.field("IYM").type == pa.string()
    assert t.schema.field("IDS").type == pa.string()
    assert t.schema.field("IYM").metadata[b"oradmp:conversion"] == b"to-char-text"
    # truth_misc.txt (18c): rows 1 to 4
    assert t.column("IYM").to_pylist() == ["+0002-06", "-0003-02", "+0000-00", None]
    assert t.column("IDS").to_pylist() == [
        "+0010 12:30:45.123456",
        "-0001 00:00:00.000000",
        "+0000 00:00:00.000000",
        None,
    ]
    buf = io.BytesIO()
    pq.write_table(t, buf)
    buf.seek(0)
    back = pq.read_table(buf)
    assert back.column("IYM").to_pylist() == t.column("IYM").to_pylist()
    assert back.column("IDS").to_pylist() == t.column("IDS").to_pylist()
    assert back.equals(t, check_metadata=False)


def test_native_mode_cannot_be_written_to_parquet() -> None:
    # unconditional, like test_consumers.py (review 03 N-5): the refusal was seen on pyarrow 15.0.2 and 26.0.0 and the
    # README states it without a version; if a later pyarrow writes the type, this test fails and the README and
    # ADR-47 are revisited
    t = read("18c/misc_exp_18c.dmp", "T_INTERVAL", interval_as="native")
    assert t.schema.field("IYM").type == pa.month_day_nano_interval()
    with pytest.raises(pa.ArrowNotImplementedError):
        pq.write_table(t, io.BytesIO())


def test_native_is_the_default() -> None:
    t = read("18c/misc_exp_18c.dmp", "T_INTERVAL")
    assert t.schema.field("IYM").type == pa.month_day_nano_interval()


def test_string_mode_reads_the_values_beyond_32_bit_months() -> None:
    # truth_misc2.txt rows 2 and 3: YEAR(9) values +999999999-11 and -999999999-11 (E-BAD-INTERVAL when native)
    t = read("18c/misc2_exp_18c.dmp", "T_INTERVAL2", interval_as="string")
    assert t.column("IYM").to_pylist()[1:] == ["+999999999-11", "-999999999-11"]
    with oradmp.open(fixture("18c/misc2_exp_18c.dmp")) as dump, pytest.raises(oradmp.ValueDecodeError):
        dump.read_table("T_INTERVAL2").read_all()


@pytest.mark.parametrize("value", ["String", "text", "", "native "])
def test_another_value_is_an_invalid_option(value: str) -> None:
    with pytest.raises(oradmp.UsageError) as ei:
        oradmp.open(fixture("18c/misc_exp_18c.dmp"), interval_as=value)
    assert ei.value.code == "E-CONFIG"
