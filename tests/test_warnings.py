"""Task T-12: warnings are returned in the result and issued once through the ``warnings`` module (ADR-33
item 3, ADR-35 item 4): ``Dump.warnings`` holds the scan's, ``BatchReader.warnings`` the read's (the counted ones
exist when the iteration has ended and are issued at that moment); per-read overrides of ``open()``'s options."""

from __future__ import annotations

import warnings
from collections.abc import Callable

import oradmp
import pytest
from conftest import fixture, table_digest

Patch = Callable[[str, str, Callable[[bytearray], None]], str]


def records(rec: list[warnings.WarningMessage]) -> list[str]:
    """The codes of the OradmpWarnings in a ``catch_warnings(record=True)`` list."""
    return [str(w.message).split(" ", 1)[0].lstrip("[") for w in rec if issubclass(w.category, oradmp.OradmpWarning)]


def test_counted_warnings_come_at_the_end_of_the_iteration_not_before() -> None:
    # t_num3_exp_18c (truth_num3.txt): six computed values, all rounded under Decimal128(38, 10): one batch
    with oradmp.open(fixture("18c/t_num3_exp_18c.dmp")) as dump:
        reader = dump.read_table("T_NUM3")
        it = iter(reader)
        with warnings.catch_warnings(record=True) as rec:
            warnings.simplefilter("always")
            first = next(it)
            assert first.num_rows == 6
            assert reader.warnings == []  # the running counts do not exist yet
            assert records(rec) == []
        with warnings.catch_warnings(record=True) as rec:
            warnings.simplefilter("always")
            with pytest.raises(StopIteration):
                next(it)
            assert records(rec) == ["W-NUMBER-ROUNDED"]
        (w,) = reader.warnings
        assert (w.code, w.table, w.column, w.row, w.part) == ("W-NUMBER-ROUNDED", "T_NUM3", "V", 1, None)
        assert (w.count, w.count_to_zero, w.max_digits_dropped) == (6, 0, 30)
        # issued once: asking again issues nothing
        with warnings.catch_warnings(record=True) as rec:
            warnings.simplefilter("always")
            with pytest.raises(StopIteration):
                next(it)
            assert records(rec) == []


@pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")
def test_pytest_warns_sees_the_rounding() -> None:
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp"), on_value_error="null") as dump:
        reader = dump.read_table("T_NUM")
        with pytest.warns(oradmp.OradmpWarning, match=r"W-NUMBER-ROUNDED") as rec:
            reader.read_all()
        details = {w.message.detail.code: w.message.detail for w in rec if isinstance(w.message, oradmp.OradmpWarning)}
        # T_NUM column V (truth_num.txt): 4 rounded values (1 became 0, at most 115 digits dropped), 10 nulled
        r = details["W-NUMBER-ROUNDED"]
        assert (r.count, r.count_to_zero, r.max_digits_dropped, r.row, r.column) == (4, 1, 115, 25, "V")
        v = details["W-VALUE-NULLED"]
        assert (v.count, v.row, v.column) == (10, 19, "V")


def test_an_error_ends_the_iteration_and_issues_the_counts_up_to_it() -> None:
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp")) as dump:
        reader = dump.read_table("T_NUM")
        with warnings.catch_warnings(record=True) as rec:
            warnings.simplefilter("always")
            with pytest.raises(oradmp.ValueDecodeError):
                reader.read_all()
            assert records(rec) == []  # no rounding happened before row 19
        assert reader.rows_read == 18


def test_tsltz_is_one_dump_warning_at_the_scan_and_never_repeated_by_a_read() -> None:
    with oradmp.open(fixture("18c/t_tstz_exp_18c.dmp")) as dump:
        with pytest.warns(oradmp.OradmpWarning, match="W-TSLTZ") as rec:
            tables = dump.tables
        assert len([w for w in rec if "W-TSLTZ" in str(w.message)]) == 1
        assert [w.code for w in dump.warnings] == ["W-TSLTZ"]
        assert dump.warnings[0].table == "T_TSTZ"
        # a second call of the scan issues nothing; neither does the read
        with warnings.catch_warnings(record=True) as rec2:
            warnings.simplefilter("always")
            assert dump.tables is tables
            dump.read_table("T_TSTZ").read_all()
            assert records(rec2) == []


def test_header_warnings_are_issued_at_open(patched: Patch) -> None:
    # t11_codes: the version line EXPORT:V18.00.00 -> V18.00.01 is W-UNTESTED-VERSION
    def patch(b: bytearray) -> None:
        at = b.find(b"EXPORT:V18.00.00")
        assert at >= 0
        b[at + 14 : at + 16] = b"01"

    path = patched("18c/t_num_exp_18c.dmp", "v1801.dmp", patch)
    with pytest.warns(oradmp.OradmpWarning, match="W-UNTESTED-VERSION"):
        dump = oradmp.open(path)
    assert [w.code for w in dump.warnings] == ["W-UNTESTED-VERSION"]
    dump.close()


@pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")
def test_per_read_overrides_of_the_number_policy_give_different_results_from_one_dump() -> None:
    # t_num3_exp_18c V: six values with more than 10 decimals
    with oradmp.open(fixture("18c/t_num3_exp_18c.dmp")) as dump:
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            rounded = dump.read_table("T_NUM3").read_all()
            nulled = dump.read_table("T_NUM3", on_number_overflow="null").read_all()
        assert rounded.column("V").null_count == 0
        assert nulled.column("V").null_count == 6
        assert table_digest(rounded) != table_digest(nulled)
        with pytest.raises(oradmp.ValueDecodeError) as ei:
            dump.read_table("T_NUM3", on_number_overflow="error").read_all()
        assert (ei.value.code, ei.value.row) == ("E-NUMBER-OVERFLOW", 1)
        # None means the value given to open(): the dump-wide default is unchanged by the overrides
        again = dump.read_table("T_NUM3")
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            assert table_digest(again.read_all()) == table_digest(rounded)
    with oradmp.open(fixture("18c/t_num3_exp_18c.dmp"), on_number_overflow="null") as dump:
        assert dump.read_table("T_NUM3").read_all().column("V").null_count == 6
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            assert dump.read_table("T_NUM3", on_number_overflow="round").read_all().column("V").null_count == 0


def test_on_value_error_override() -> None:
    with oradmp.open(fixture("18c/t_date_exp_18c.dmp")) as dump:
        with pytest.raises(oradmp.ValueDecodeError) as ei:
            dump.read_table("T_DATE").read_all()
        assert (ei.value.code, ei.value.row, ei.value.column) == ("E-BAD-DATETIME", 8, "D")
        with pytest.warns(oradmp.OradmpWarning, match="W-VALUE-NULLED"):
            t = dump.read_table("T_DATE", on_value_error="null").read_all()
        assert t.column("D").null_count == 3  # rows 8, 9 (years -100, -4712; truth_date.txt) and the NULL of row 11


def test_a_number_types_entry_fills_both_maps() -> None:
    # ("decimal(20,5)", "null") on T_NUM.V: the type is Decimal128(20, 5) and the entry's policy wins over the
    # read's `on_number_overflow="round"`: the 4 values with more than 5 decimals are NULL, not rounded
    with oradmp.open(
        fixture("18c/t_num_exp_18c.dmp"),
        on_value_error="null",
        number_types={"T_NUM.V": ("decimal(20,5)", "null")},
    ) as dump:
        reader = dump.read_table("T_NUM", on_number_overflow="round")
        assert str(reader.schema.field("V").type) == "decimal128(20, 5)"
        with warnings.catch_warnings():
            warnings.simplefilter("ignore")
            reader.read_all()
        codes = {w.code for w in reader.warnings}
        assert "W-NUMBER-NULLED" in codes
        assert "W-NUMBER-ROUNDED" not in codes
        assert reader.schema.field("V").metadata[b"oradmp:number_overflow"] == b"null"
    # a bare string entry takes the read's policy
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp"), number_types={"T_NUM.V": "string"}) as dump:
        assert str(dump.read_table("T_NUM").schema.field("V").type) == "string"


def test_extra_intro_lines_are_validated() -> None:
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp"), extra_intro_lines=["suffix:15:plain: MODIFY DEFAULT"]) as dump:
        assert dump.tables
    with pytest.raises(oradmp.UsageError) as ei:
        oradmp.open(fixture("18c/t_num_exp_18c.dmp"), extra_intro_lines=["not an entry"])
    assert ei.value.code == "E-CONFIG"


def test_dump_rows_is_not_exposed_and_the_lob_diagnostics_are_not_bound() -> None:
    with oradmp.open(fixture("18c/lob_exp_18c.dmp")) as dump:
        assert not hasattr(dump, "rows")
        assert not hasattr(dump, "part_rows")
        reader = dump.read_table("T_LOB")
        assert not hasattr(reader, "lob_bytes_read")
        assert not hasattr(reader, "lob_values_skipped")


def test_context_managers_and_closing() -> None:
    dump = oradmp.open(fixture("18c/t_num3_exp_18c.dmp"))
    with warnings.catch_warnings():
        warnings.simplefilter("ignore")
        reader = dump.read_table("T_NUM3")
        dump.close()
        assert dump.closed
        # a reader handed out before the close keeps its own handle
        assert reader.read_all().num_rows == 6
        with pytest.raises(oradmp.UsageError):
            dump.read_table("T_NUM3")
    with oradmp.open(fixture("18c/t_num3_exp_18c.dmp")) as dump2, dump2.read_table("T_NUM3") as r:
        assert r.rows_read == 0


@pytest.mark.parametrize("how", ["next", "read_all", "close", "with"])
def test_every_issuing_call_points_the_warning_at_the_users_line(how: str) -> None:
    # review 03 N-2 / ADR-60: __next__, read_all(), close() and the end of a `with` all attribute the warning to
    # the caller's file; T_NUM (truth_num.txt) gives W-VALUE-NULLED and W-NUMBER-ROUNDED
    import pyarrow as pa

    with oradmp.open(fixture("18c/t_num_exp_18c.dmp"), on_value_error="null") as dump:
        with warnings.catch_warnings(record=True) as rec:
            warnings.simplefilter("always")
            reader = dump.read_table("T_NUM")
            if how == "next":
                for _ in reader:
                    pass
            elif how == "read_all":
                reader.read_all()
            elif how == "close":
                pa.RecordBatchReader.from_stream(reader).read_all()
                assert records(rec) == []
                reader.close()
            else:
                with reader:
                    pa.RecordBatchReader.from_stream(reader).read_all()
            mine = [w for w in rec if issubclass(w.category, oradmp.OradmpWarning)]
        assert len(mine) == 2
        assert {w.filename for w in mine} == {__file__}
