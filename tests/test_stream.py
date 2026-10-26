"""Task T-13h (a), ADR-55: the Arrow C stream path keeps the read's row count and warnings.

All on ``18c/t_num_exp_18c.dmp`` T_NUM (33 rows) with ``on_value_error="null"`` (truth_num.txt, as
``test_warnings.py``): column V has 10 values that fail (row 19 first) and 4 that are rounded (row 25 first, one
becomes 0). The iterator path and the path through ``pyarrow.RecordBatchReader.from_stream`` must report the same.
"""

from __future__ import annotations

import warnings
from collections.abc import Callable

import oradmp
import pyarrow as pa
import pytest
from conftest import fixture

PATH = "18c/t_num_exp_18c.dmp"
LENIENT = {"on_value_error": "null"}
Patch = Callable[[str, str, Callable[[bytearray], None]], str]


def stream_text(e: BaseException) -> str:
    """The text of a stream error without the ``External error: `` prefix that arrow-rs puts in front of it (the
    ADR-55 text "starts with the code" holds after that prefix; recorded in the T-13h note)."""
    return str(e).removeprefix("External error: ")


def codes(rec: list[warnings.WarningMessage]) -> list[str]:
    return [str(w.message).split(" ", 1)[0].lstrip("[") for w in rec if issubclass(w.category, oradmp.OradmpWarning)]


@pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")
def test_the_stream_path_reports_what_the_iterator_path_reports() -> None:
    with oradmp.open(fixture(PATH), **LENIENT) as dump:  # type: ignore[arg-type]
        with dump.read_table("T_NUM") as it_reader:
            it_rows = sum(b.num_rows for b in it_reader)
        with dump.read_table("T_NUM") as reader:
            table = pa.RecordBatchReader.from_stream(reader).read_all()
            assert table.num_rows == 33
            assert reader.rows_read == 33
            by_code = {w.code: w for w in reader.warnings}
            # truth_num.txt: 10 values of V fail (row 19 first), 4 are rounded (row 25 first; 1 becomes 0)
            assert sorted(by_code) == ["W-NUMBER-ROUNDED", "W-VALUE-NULLED"]
            assert by_code["W-VALUE-NULLED"].count == 10
            assert by_code["W-NUMBER-ROUNDED"].count == 4
            assert by_code["W-NUMBER-ROUNDED"].count_to_zero == 1
            assert (by_code["W-VALUE-NULLED"].row, by_code["W-NUMBER-ROUNDED"].row) == (19, 25)
            # the same warnings (codes, counts, offsets, columns, rows) as the iterator path
            assert reader.warnings == it_reader.warnings
            assert all(w.offset is not None for w in reader.warnings)
        assert it_rows == 33 == it_reader.rows_read


def test_the_warnings_of_the_stream_path_are_issued_once_at_the_end_of_the_with() -> None:
    with warnings.catch_warnings(record=True) as rec, oradmp.open(fixture(PATH), **LENIENT) as dump:  # type: ignore[arg-type]
        warnings.simplefilter("always")
        with dump.read_table("T_NUM") as reader:
            pa.RecordBatchReader.from_stream(reader).read_all()
            assert codes(rec) == []  # nothing is issued from inside the stream
        assert sorted(codes(rec)) == ["W-NUMBER-ROUNDED", "W-VALUE-NULLED"]
        assert {w.filename for w in rec} == {__file__}  # the user's line, not the package's
        reader.close()  # issued once
        assert len(codes(rec)) == 2


def test_a_consumer_that_never_closes_the_reader_still_finds_the_warnings() -> None:
    with oradmp.open(fixture(PATH), **LENIENT) as dump:  # type: ignore[arg-type]
        reader = dump.read_table("T_NUM")
        with warnings.catch_warnings(record=True) as rec:
            warnings.simplefilter("always")
            pa.RecordBatchReader.from_stream(reader).read_all()
            assert codes(rec) == []
        assert sorted(w.code for w in reader.warnings) == ["W-NUMBER-ROUNDED", "W-VALUE-NULLED"]


@pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")
def test_rows_read_follows_the_consumer_one_batch_at_a_time() -> None:
    with (
        oradmp.open(fixture(PATH), **LENIENT) as dump,  # type: ignore[arg-type]
        dump.read_table("T_NUM", batch_rows=5) as reader,
    ):
        stream = pa.RecordBatchReader.from_stream(reader)
        assert reader.rows_read == 0
        first = stream.read_next_batch()
        assert first.num_rows == 5
        assert reader.rows_read == 5


def test_close_before_the_first_pull_makes_the_pull_fail_with_e_config() -> None:
    with oradmp.open(fixture(PATH), **LENIENT) as dump:  # type: ignore[arg-type]
        reader = dump.read_table("T_NUM")
        stream = pa.RecordBatchReader.from_stream(reader)
        reader.close()
        with pytest.raises(pa.ArrowInvalid) as ei:  # a silent end would be a short table
            stream.read_all()
        assert stream_text(ei.value).startswith("[E-CONFIG]")
        assert reader.rows_read == 0  # the counts stay readable


@pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")
def test_close_after_the_end_of_the_stream_is_not_an_error() -> None:
    with oradmp.open(fixture(PATH), **LENIENT) as dump:  # type: ignore[arg-type]
        reader = dump.read_table("T_NUM")
        stream = pa.RecordBatchReader.from_stream(reader)
        assert stream.read_all().num_rows == 33
        reader.close()
        assert reader.rows_read == 33


def test_the_reader_is_spent_for_iteration_after_the_export() -> None:
    with oradmp.open(fixture(PATH), **LENIENT) as dump:  # type: ignore[arg-type]
        reader = dump.read_table("T_NUM")
        reader.__arrow_c_stream__()
        with pytest.raises(oradmp.UsageError) as ei:
            next(reader)
        assert ei.value.code == "E-CONFIG"
        with pytest.raises(oradmp.UsageError):
            reader.read_all()
        with pytest.raises(oradmp.UsageError):  # a second export stays E-CONFIG
            reader.__arrow_c_stream__()


def test_an_error_inside_the_stream_carries_its_code_and_offset_and_the_warnings_are_complete() -> None:
    # T_NUM row 19 of V has an integer part above 28 digits (truth_num.txt): E-NUMBER-OVERFLOW at 0x230f
    with oradmp.open(fixture(PATH)) as dump, dump.read_table("T_NUM") as reader:
        with pytest.raises(pa.ArrowInvalid) as ei:
            pa.RecordBatchReader.from_stream(reader).read_all()
        assert stream_text(ei.value).startswith("[E-NUMBER-OVERFLOW offset=0x230f]")
        assert reader.rows_read == 18


def test_the_iterator_after_close_is_an_error_before_the_end_and_a_normal_end_after_it() -> None:
    # T-13i (a), ADR-64 item 1 / ADR-61 item 4: 18c part_exp_18c.dmp T_PART has 3,000 rows (three batches of 1000
    # and an empty end); a close before the end must never look like the end of the table.
    path = fixture("18c/part_exp_18c.dmp")
    with oradmp.open(path) as dump:
        reader = dump.read_table("T_PART", batch_rows=1000)
        assert next(reader).num_rows == 1000
        reader.close()
        with pytest.raises(oradmp.UsageError) as ei:
            next(reader)
        assert ei.value.code == "E-CONFIG"
        assert "the reader is closed" in str(ei.value)
        with pytest.raises(oradmp.UsageError) as ei2:
            reader.read_all()
        assert ei2.value.code == "E-CONFIG"
        assert "the reader is closed" in str(ei2.value)
        assert reader.rows_read == 1000

        done = dump.read_table("T_PART", batch_rows=1000)
        assert sum(b.num_rows for b in done) == 3000
        done.close()
        with pytest.raises(StopIteration):
            next(done)
        empty = done.read_all()
        assert empty.num_rows == 0
        assert empty.schema == done.schema
        assert done.rows_read == 3000


def patch_v_row_17(b: bytearray) -> None:
    # T-13i (b), ADR-64 item 2. Facts from `exp_parse.py --rows` on 18c/t_num_exp_18c.dmp (row 17 of T_NUM):
    # `@22ee [2] c1 12 | @22f2 [5] c2 02 18 2e 3d`; the V length `05 00` is at 0x22F2, its bytes at 0x22F4.
    assert bytes(b[0x22F2:0x22F9]) == bytes.fromhex("0500c202182e3d")
    b[0x22F4] = 0xBC  # exponent byte c2 -> bc: 1.23456E-10, which rounds to 1E-10 in Decimal128(38, 10)


def check_row_17_rounded_before_row_19(reader: oradmp.BatchReader) -> None:
    (w,) = reader.warnings
    assert w.code == "W-NUMBER-ROUNDED"
    assert (w.count, w.offset, w.row, w.column) == (1, 8948, 17, "V")
    assert reader.rows_read == 18  # as test_warnings.py::test_an_error_ends_the_iteration_...


def test_the_warnings_are_complete_after_an_error_on_the_iterator_path(patched: Patch) -> None:
    path = patched(PATH, "t_num_v17.dmp", patch_v_row_17)
    with warnings.catch_warnings(record=True) as rec:
        warnings.simplefilter("always")
        with oradmp.open(path) as dump:
            with dump.read_table("T_NUM") as reader:
                with pytest.raises(oradmp.ValueDecodeError) as ei:
                    reader.read_all()
                assert ei.value.code == "E-NUMBER-OVERFLOW"
                assert ei.value.offset == 0x230F
                assert ei.value.row == 19
                assert codes(rec) == ["W-NUMBER-ROUNDED"]  # the iterator path issues them when the error ends the read
                check_row_17_rounded_before_row_19(reader)  # review 05 SF-1: the reader's own list, offset and rows
            assert codes(rec) == ["W-NUMBER-ROUNDED"]  # not a second time at the end of the with
            assert codes(rec) == ["W-NUMBER-ROUNDED"]
        assert codes(rec) == ["W-NUMBER-ROUNDED"]  # once


def test_the_warnings_are_complete_after_an_error_on_the_stream_path(patched: Patch) -> None:
    path = patched(PATH, "t_num_v17.dmp", patch_v_row_17)
    with warnings.catch_warnings(record=True) as rec:
        warnings.simplefilter("always")
        with oradmp.open(path) as dump:
            with dump.read_table("T_NUM") as reader:
                with pytest.raises(pa.ArrowInvalid) as ei:
                    pa.RecordBatchReader.from_stream(reader).read_all()
                assert "[E-NUMBER-OVERFLOW offset=0x230f]" in stream_text(ei.value)
                check_row_17_rounded_before_row_19(reader)
                assert codes(rec) == []
            assert codes(rec) == ["W-NUMBER-ROUNDED"]
        assert codes(rec) == ["W-NUMBER-ROUNDED"]
