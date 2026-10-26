"""Task T-12: what consumers do with the Arrow types of the reader (ADR-28 (b) items 1 to 6, ADR-19 Parquet
check). The assertions state what was observed with pyarrow 26.0.0 and pandas 3.0.6 on the VM (2026-10-17); the
results are written down in ``docs/design/implementer-notes.md`` (T-12 note), for the Architect to record under
ADR-28 and ADR-19. A different pyarrow / pandas may differ: the tests skip below pandas 2."""

from __future__ import annotations

import datetime as dt
import io
import warnings

import oradmp
import pyarrow as pa
import pyarrow.parquet as pq
import pytest
from conftest import fixture

pd = pytest.importorskip("pandas")
if int(pd.__version__.split(".")[0]) < 2:  # pragma: no cover
    pytest.skip("observations are for pandas 2 and later", allow_module_level=True)

pytestmark = pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")


def read(rel: str, table: str) -> pa.Table:
    with oradmp.open(fixture(rel), on_value_error="null") as dump:
        return dump.read_table(table).read_all()


def test_date_column_to_pandas_keeps_year_1_and_year_9999() -> None:
    # (1) truth_date.txt rows 4 and 5: 0001-01-01 00:00:00 and 9999-12-31 23:59:59 in Timestamp(Second)
    t = read("18c/t_date_exp_18c.dmp", "T_DATE")
    assert t.schema.field("D").type == pa.timestamp("s")
    df = t.to_pandas()
    assert str(df["D"].dtype) == "datetime64[s]"
    assert df["D"].iloc[3] == pd.Timestamp("0001-01-01 00:00:00")
    assert df["D"].iloc[4] == pd.Timestamp("9999-12-31 23:59:59")
    assert df["D"].iloc[3].year == 1 and df["D"].iloc[4].year == 9999


def test_date_column_with_coerce_temporal_nanoseconds_fails_for_both_extremes() -> None:
    # (2) the nanosecond range is about 1677-09-21 to 2262-04-11: the cast is refused, not wrapped
    t = read("18c/t_date_exp_18c.dmp", "T_DATE")
    with pytest.raises(pa.ArrowInvalid, match="out of bounds timestamp"):
        t.to_pandas(coerce_temporal_nanoseconds=True)


def test_date_column_as_python_objects_gives_datetime_for_both_extremes() -> None:
    # (3) timestamp_as_object=True and to_pylist() give datetime.datetime for year 1 and year 9999
    t = read("18c/t_date_exp_18c.dmp", "T_DATE")
    objs = t.to_pandas(timestamp_as_object=True)["D"].tolist()
    assert objs[3] == dt.datetime(1, 1, 1) and objs[4] == dt.datetime(9999, 12, 31, 23, 59, 59)
    lst = t.column("D").to_pylist()
    assert lst[3] == dt.datetime(1, 1, 1) and lst[4] == dt.datetime(9999, 12, 31, 23, 59, 59)


def test_timestamp_columns_micro_and_nano() -> None:
    # (4) truth_ts.txt row 5: TS0 and TS6 are 9999-12-31 (Microsecond); TS9 (Nanosecond) is outside the nanosecond
    # range, E-BAD-DATETIME, so it is NULL under on_value_error="null"
    t = read("18c/t_ts_exp_18c.dmp", "T_TS")
    df = t.to_pandas()
    assert [str(df[c].dtype) for c in ("TS0", "TS6", "TS9")] == [
        "datetime64[us]",
        "datetime64[us]",
        "datetime64[ns]",
    ]
    assert df["TS0"].iloc[4] == pd.Timestamp("9999-12-31 23:59:59")
    assert df["TS6"].iloc[4] == pd.Timestamp("9999-12-31 23:59:59.999999")
    assert pd.isna(df["TS9"].iloc[4])
    with pytest.raises(pa.ArrowInvalid):
        t.to_pandas(coerce_temporal_nanoseconds=True)
    objs = t.to_pandas(timestamp_as_object=True)
    assert objs["TS0"].iloc[4] == dt.datetime(9999, 12, 31, 23, 59, 59)
    assert objs["TS6"].iloc[4] == dt.datetime(9999, 12, 31, 23, 59, 59, 999999)
    # Oracle's own TS9 value of row 4 (2000-01-01 00:00:00.123456789) keeps its nanoseconds
    assert df["TS9"].iloc[3] == pd.Timestamp("2000-01-01 00:00:00.123456789")


def test_a_nulled_value_arrives_as_nat_and_none() -> None:
    # (5) T_DATE rows 8 and 9 are years below 1 (E-BAD-DATETIME): NULL under on_value_error="null"
    t = read("18c/t_date_exp_18c.dmp", "T_DATE")
    df = t.to_pandas()
    assert pd.isna(df["D"].iloc[7]) and pd.isna(df["D"].iloc[8])
    assert df["D"].iloc[7] is pd.NaT
    lst = t.column("D").to_pylist()
    assert lst[7] is None and lst[8] is None


def test_parquet_accepts_duration_but_not_interval_month_day_nano() -> None:
    # (6) ADR-19: pyarrow cannot write Interval(MonthDayNano) to Parquet; Duration(Microsecond) it can
    t = read("18c/misc_exp_18c.dmp", "T_INTERVAL")
    assert t.schema.field("IYM").type == pa.month_day_nano_interval()
    assert t.schema.field("IDS").type == pa.duration("us")
    buf = io.BytesIO()
    with pytest.raises(pa.ArrowNotImplementedError, match="month_day_nano_interval"):
        pq.write_table(t, buf)
    buf = io.BytesIO()
    pq.write_table(t.select(["ID", "IDS"]), buf)
    buf.seek(0)
    back = pq.read_table(buf)
    assert back.column("IDS").to_pylist() == t.column("IDS").to_pylist()
    # the interval column survives as Python objects and as a cast to a plain months column
    assert t.column("IYM").to_pylist()[0].months == 30
    months = pa.array([v.months if v is not None else None for v in t.column("IYM").to_pylist()], pa.int32())
    buf = io.BytesIO()
    pq.write_table(pa.table({"IYM_MONTHS": months}), buf)


def test_number_columns_to_pandas() -> None:
    # Decimal128 columns arrive as object columns of decimal.Decimal (arch 5.2a, "pandas: not verified")
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp"), on_value_error="null") as dump:
        df = dump.read_table("T_NUM").read_all().to_pandas()
    assert str(df["ID"].dtype) == "int64"
    assert str(df["V"].dtype) == "object"
    import decimal

    assert isinstance(df["V"].iloc[1], decimal.Decimal)
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        with oradmp.open(fixture("18c/t_num_exp_18c.dmp"), on_value_error="null", number_default="float64") as dump:
            df = dump.read_table("T_NUM").read_all().to_pandas()
    assert str(df["V"].dtype) == "float64"
