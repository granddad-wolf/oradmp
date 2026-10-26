"""Task T-12: typed exceptions carry ``.code``, ``.offset``, ``.table``, ``.part``, ``.column``, ``.row``,
``.length`` and ``.warning`` (architecture 3.4, ADR-29, ADR-41); every code of the core has a Python class;
``strict`` errors; a dump stays usable after a ``StrictError``. Expected offsets are those of the Rust tests
(``t11_codes.rs``, ``t6_patched.rs``, ``t11_strict.rs``) and of the Analyst's ``exp_parse.py``."""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path

import oradmp
import pytest
from conftest import find_one, fixture
from oradmp import _core

Patch = Callable[[str, str, Callable[[bytearray], None]], str]


def golden_columns(rel: str, table: str) -> list[str]:
    """The column names of a table, in file order, from the golden catalog."""
    for line in (Path(__file__).parent / "golden" / "exp_catalog.tsv").read_text().splitlines():
        f = line.split("\t")
        if len(f) > 11 and f[0] == rel and f[2] == table:
            return f[11].split("|")
    raise AssertionError((rel, table))


def test_every_core_code_has_a_python_class_and_nothing_else_does() -> None:
    assert set(oradmp.ERROR_CLASSES) == set(_core.ERROR_CODES)
    assert set(oradmp.WARNING_CLASSES) == set(_core.WARNING_CODES)
    for code, cls in oradmp.ERROR_CLASSES.items():
        assert issubclass(cls, (oradmp.OradmpError, OSError)), code
    for cls in oradmp.WARNING_CLASSES.values():
        assert issubclass(cls, oradmp.OradmpWarning)
    # the hierarchy of architecture 3.4
    assert issubclass(oradmp.UnsupportedCharset, oradmp.UnsupportedError)
    assert issubclass(oradmp.StrictError, oradmp.OradmpError)
    assert not issubclass(oradmp.StrictError, oradmp.UnsupportedError)
    assert oradmp.ERROR_CLASSES["E-LOB-UNCONFIRMED"] is oradmp.ValueDecodeError
    assert oradmp.ERROR_CLASSES["E-STRICT"] is oradmp.StrictError
    assert oradmp.ERROR_CLASSES["E-UNSUPPORTED-CHARSET"] is oradmp.UnsupportedCharset
    assert issubclass(oradmp.OradmpWarning, UserWarning)


def test_a_value_error_carries_code_offset_table_column_and_row() -> None:
    # t_num_exp_18c: row 19 of T_NUM column V has an integer part above 28 digits (truth_num.txt), at 0x230f
    with (
        oradmp.open(fixture("18c/t_num_exp_18c.dmp")) as dump,
        pytest.raises(oradmp.ValueDecodeError) as ei,
    ):
        dump.read_table("T_NUM").read_all()
    e = ei.value
    assert (e.code, e.offset, e.table, e.column, e.row) == ("E-NUMBER-OVERFLOW", 0x230F, "T_NUM", "V", 19)
    assert e.part is None  # not partitioned
    assert e.warning is None
    assert e.table_ordinal == 1
    assert str(e).startswith("[E-NUMBER-OVERFLOW offset=0x230f]")


def test_part_is_the_partition_name_in_a_partitioned_table(patched: Patch) -> None:
    # part_exp_18c: T_PART, partition P2020 starts at 0x834e; row 1: ID `02 00 c1 1e`, SALE_DATE `07 00` +
    # `78 78 01 0d 01 01 01` (2020-01-13); the month byte at 0x8356 becomes 13
    def patch(b: bytearray) -> None:
        assert bytes(b[0x834E:0x8354]) == bytes.fromhex("0200c11e0700")
        assert bytes(b[0x8354:0x835B]) == bytes.fromhex("7878010d010101")
        b[0x8356] = 13

    path = patched("18c/part_exp_18c.dmp", "part_month13.dmp", patch)
    with oradmp.open(path) as dump, pytest.raises(oradmp.ValueDecodeError) as ei:
        dump.read_table("T_PART").read_all()
    e = ei.value
    assert (e.code, e.offset, e.table, e.part, e.column, e.row) == (
        "E-BAD-DATETIME",
        0x8354,
        "T_PART",
        ("P2020", None),
        "SALE_DATE",
        1,
    )


def test_a_framing_error_is_a_format_error(patched: Patch) -> None:
    # t_num_exp_18c: the first column field of the first row gets a length of 0 [R8] (t11_codes::e_framing)
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp")) as dump:
        rows_offset = dump.tables[0].parts[0].rows_offset

    def patch(b: bytearray) -> None:
        b[rows_offset : rows_offset + 2] = b"\x00\x00"

    path = patched("18c/t_num_exp_18c.dmp", "framing.dmp", patch)
    with oradmp.open(path) as dump, pytest.raises(oradmp.FormatError) as ei:
        dump.scan()
    e = ei.value
    # a failed scan has no catalog: the table is known by its ordinal only
    assert (e.code, e.offset, e.table, e.table_ordinal, e.row) == ("E-FRAMING", rows_offset, None, 1, 1)
    assert isinstance(e, oradmp.OradmpError)


def test_not_an_exp_file_is_a_format_error(tmp_path: Path) -> None:
    p = tmp_path / "junk.dmp"
    p.write_bytes(b"this is not a dump file" * 100)
    with pytest.raises(oradmp.FormatError) as ei:
        oradmp.open(p)
    assert ei.value.code == "E-NOT-EXP-FILE"
    assert ei.value.offset is not None


def test_an_unsupported_type_stops_the_scan_and_is_an_unsupported_error(patched: Patch) -> None:
    # t_num_exp_18c: the type of the first descriptor (0x223a, `exp_parse.py`) becomes 114 [X1]
    def patch(b: bytearray) -> None:
        assert bytes(b[0x223A:0x223C]) == b"\x02\x00"
        b[0x223A:0x223C] = (114).to_bytes(2, "little")

    path = patched("18c/t_num_exp_18c.dmp", "type114.dmp", patch)
    with oradmp.open(path) as dump:
        cat = dump.scan()
    assert cat.end == "stopped"
    e = cat.stop_error
    assert isinstance(e, oradmp.UnsupportedError)
    assert (e.code, e.offset, e.type_code) == ("E-UNSUPPORTED-TYPE", 0x223A, 114)
    assert cat.stopped_table is not None
    assert cat.stopped_table.name == "T_NUM"


LONG_STOP = "conventional LONG or LONG RAW field starting 0xfffb or 0xfffc, not in the fixtures [R8]"


def test_a_conventional_long_field_starting_fb_ff_stops_the_scan_with_the_r8_error(patched: Patch) -> None:
    # T-19b (ADR-99 item 1): long4_exp_18c.dmp, T_LONG4 row 3, the u16 at 0x22277 (`fa ff`, form (b), 65,530 bytes;
    # `long4_check.py`) becomes `fb ff`: the scan stops, the error is an UnsupportedError with the offset of the u16
    def patch(b: bytearray) -> None:
        assert bytes(b[0x22277:0x22279]) == b"\xfa\xff"
        b[0x22277:0x22279] = b"\xfb\xff"

    path = patched("18c/long4_exp_18c.dmp", "long4_fbff.dmp", patch)
    with oradmp.open(path) as dump:
        cat = dump.scan()
    assert cat.end == "stopped"
    e = cat.stop_error
    assert isinstance(e, oradmp.UnsupportedError)
    assert (e.code, e.offset, e.table_ordinal, e.row, e.length) == ("E-UNSUPPORTED-LAYOUT", 0x22277, 1, 3, 0xFFFB)
    # the table is not complete, so the column has no name; the position stands in the text
    assert e.column is None
    assert "column 2, row 3" in str(e)
    assert LONG_STOP in str(e)
    assert cat.tables == []
    assert cat.stopped_table is not None
    assert cat.stopped_table.name == "T_LONG4"
    # raised, it is the same class
    with pytest.raises(oradmp.UnsupportedError) as ei:
        raise e
    assert ei.value.code == "E-UNSUPPORTED-LAYOUT"


def test_a_long_raw_field_starting_fc_ff_stops_the_scan_after_the_tables_before_it(patched: Patch) -> None:
    # long_exp_18c.dmp, T_LONGRAW row 2, the u16 at 0xa0ea (`00 7d`, 32,000 bytes) becomes `fc ff`; T_LONG stays
    def patch(b: bytearray) -> None:
        assert bytes(b[0xA0EA:0xA0EC]) == b"\x00\x7d"
        b[0xA0EA:0xA0EC] = b"\xfc\xff"

    path = patched("18c/long_exp_18c.dmp", "longraw_fcff.dmp", patch)
    with oradmp.open(path) as dump:
        cat = dump.scan()
        assert [t.name for t in cat.tables] == ["T_LONG"]
        assert dump.read_table("T_LONG").read_all().num_rows == 3
    e = cat.stop_error
    assert isinstance(e, oradmp.UnsupportedError)
    assert (e.code, e.offset, e.table_ordinal, e.row, e.length) == ("E-UNSUPPORTED-LAYOUT", 0xA0EA, 2, 2, 0xFFFC)
    assert "column 2, row 2" in str(e)
    assert LONG_STOP in str(e)


def test_the_new_long_stats_field_and_a_form_b_table_read_in_python() -> None:
    # T-19b (ADR-101 item 4): LongStats.sizes_65521_to_65534; long4_exp_18c: six values of 65,521 to 65,534 bytes
    # (truth_long4.txt), two chunked; the 65,533-byte value of row 5 reads whole
    with oradmp.open(fixture("18c/long4_exp_18c.dmp")) as dump:
        info = next(t for t in dump.tables if t.name == "T_LONG4")
        col = next(c for c in info.columns if c.name == "L")
        assert col.long is not None
        assert (col.long.values, col.long.chunked, col.long.largest, col.long.sizes_65521_to_65534) == (7, 2, 65535, 6)
        t = dump.read_table("T_LONG4").read_all()
        assert t.num_rows == 7
        assert len(t.column("L")[4].as_py()) == 65533


def test_usage_errors() -> None:
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp")) as dump:
        with pytest.raises(oradmp.UsageError) as ei:
            dump.read_table("NOPE")
        assert ei.value.code == "E-NO-SUCH-TABLE"
        with pytest.raises(oradmp.UsageError) as ei:
            dump.read_table("T_NUM", columns=["NOPE"])
        assert ei.value.code == "E-NO-SUCH-COLUMN"
        with pytest.raises(oradmp.UsageError) as ei:
            dump.read_table("T_NUM", on_value_error="skip")
        assert ei.value.code == "E-CONFIG"
        with pytest.raises(oradmp.UsageError):
            dump.read_table(1.5)  # type: ignore[arg-type]
    with pytest.raises(oradmp.UsageError) as ei:
        oradmp.open(fixture("18c/t_num_exp_18c.dmp"), number_default="decimal(99,2)")
    assert ei.value.code == "E-CONFIG"
    with pytest.raises(oradmp.UsageError):
        oradmp.open(fixture("18c/t_num_exp_18c.dmp"), on_number_overflow="maybe")
    with pytest.raises(oradmp.UsageError):
        oradmp.open(fixture("18c/t_num_exp_18c.dmp"), number_types={"T_NUM.V": 3})


def test_a_missing_file_is_an_oserror() -> None:
    with pytest.raises(FileNotFoundError):
        oradmp.open("/nonexistent/dump.dmp")


def test_a_changed_file_is_refused(tmp_path: Path) -> None:
    p = tmp_path / "t.dmp"
    p.write_bytes((Path(fixture("18c/t_num_exp_18c.dmp"))).read_bytes())
    with oradmp.open(p, on_value_error="null") as dump:
        dump.scan()
        with p.open("ab") as f:
            f.write(b"\x00")
        with pytest.raises(oradmp.UsageError) as ei:
            dump.read_table("T_NUM")
    assert ei.value.code == "E-FILE-CHANGED"


def test_lob_unconfirmed_is_a_value_decode_error_naming_the_database_set(patched: Patch) -> None:
    # the 178 copy of T-6M (d): R_LOB2 row 1 column N (NCLOB), @0x2790, stored length 132
    def patch(b: bytearray) -> None:
        assert bytes(b[0x35:0x37]) == b"\x00\xab"
        assert bytes(b[0x24E3:0x24E5]) == b"\xab\x00"
        b[0x36] = 0xB2
        b[0x24E3] = 0xB2

    path = patched("11g-cp1251/ru_lob2_exp_cp1251_11g.dmp", "lob178.dmp", patch)
    with oradmp.open(path) as dump, pytest.raises(oradmp.ValueDecodeError) as ei:
        dump.read_table("R_LOB2").read_all()
    e = ei.value
    assert (e.code, e.offset, e.table, e.column, e.row, e.length) == (
        "E-LOB-UNCONFIRMED",
        0x2790,
        "R_LOB2",
        "N",
        1,
        132,
    )
    assert "database character set id 178 of the header" in str(e)
    # `unconfirmed_lob="decode"` reads it, with the counted W-LOB-UNTESTED
    with oradmp.open(path, unconfirmed_lob="decode") as dump:
        r = dump.read_table("R_LOB2")
        with pytest.warns(oradmp.OradmpWarning):
            r.read_all()
        assert {(w.code, w.column, w.count) for w in r.warnings if w.code == "W-LOB-UNTESTED"} == {
            ("W-LOB-UNTESTED", "C", 1),
            ("W-LOB-UNTESTED", "N", 2),
        }


# ---------------------------------------------------------------------------------------------
# strict
# ---------------------------------------------------------------------------------------------


def test_strict_tsltz_is_a_strict_error_at_the_scan() -> None:
    with (
        oradmp.open(fixture("18c/t_tstz_exp_18c.dmp"), strict=True) as dump,
        pytest.raises(oradmp.StrictError) as ei,
    ):
        dump.scan()
    e = ei.value
    assert (e.code, e.warning) == ("E-STRICT", "W-TSLTZ")
    assert e.offset is not None
    assert not isinstance(e, oradmp.UnsupportedError)


def test_strict_makes_a_rounded_number_an_overflow_error_that_names_the_warning() -> None:
    # t_num3_exp_18c: six rounded values; strict: E-NUMBER-OVERFLOW at row 1 of V, caused by W-NUMBER-ROUNDED
    with (
        oradmp.open(fixture("18c/t_num3_exp_18c.dmp"), strict=True) as dump,
        pytest.raises(oradmp.ValueDecodeError) as ei,
    ):
        dump.read_table("T_NUM3").read_all()
    e = ei.value
    assert (e.code, e.row, e.column, e.warning) == ("E-NUMBER-OVERFLOW", 1, "V", "W-NUMBER-ROUNDED")
    # an integer part that does not fit is an overflow whatever strict says: no warning behind it
    with (
        oradmp.open(fixture("18c/t_num_exp_18c.dmp"), strict=True) as dump,
        pytest.raises(oradmp.ValueDecodeError) as ei,
    ):
        dump.read_table("T_NUM").read_all()
    assert (ei.value.code, ei.value.row, ei.value.warning) == ("E-NUMBER-OVERFLOW", 19, None)


def test_strict_unsupported_charset(patched: Patch) -> None:
    # misc_exp_18c T_FRAME: the VARCHAR2 descriptors get the set id 1000 (t11_codes::e_unsupported_charset)
    desc = bytes.fromhex("030001000a006903010001000a006903010001000a0069030100")

    def patch(b: bytearray) -> None:
        at = find_one(b, desc)
        assert bytes(b[at + 6 : at + 8]) == b"\x69\x03"
        b[at + 6 : at + 8] = (1000).to_bytes(2, "little")

    path = patched("18c/misc_exp_18c.dmp", "set1000.dmp", patch)
    with oradmp.open(path) as dump:
        with pytest.warns(oradmp.OradmpWarning, match="W-CHARSET-UNSUPPORTED"):
            dump.scan()
        assert any(w.code == "W-CHARSET-UNSUPPORTED" for w in dump.warnings)
    with oradmp.open(path, strict=True) as dump, pytest.raises(oradmp.UnsupportedCharset) as ei:
        dump.scan()
    e = ei.value
    assert (e.code, e.warning) == ("E-UNSUPPORTED-CHARSET", "W-CHARSET-UNSUPPORTED")
    assert isinstance(e, oradmp.UnsupportedError)


def test_the_dump_stays_usable_after_a_strict_error_from_read_table(patched: Patch) -> None:
    # ADR-41 item 2: misc_exp_18c with the ROWID prefix `01 00 00 00 17` -> `18` (t6_patched): W-UNVERIFIED-TYPE
    # at T_ROWID row 1, E-STRICT under strict; the framing is intact and T_FRAME (7 rows) reads
    def patch(b: bytearray) -> None:
        at = find_one(b, bytes.fromhex("17000100000017"))
        b[at + 6] = 0x18

    path = patched("18c/misc_exp_18c.dmp", "rowid_prefix.dmp", patch)
    with oradmp.open(path) as dump:
        r = dump.read_table("T_ROWID")
        with pytest.warns(oradmp.OradmpWarning, match="W-UNVERIFIED-TYPE"):
            r.read_all()
    with oradmp.open(path, strict=True) as dump:
        with pytest.raises(oradmp.StrictError) as ei:
            dump.read_table("T_ROWID").read_all()
        e = ei.value
        # the patched value is in the ROWID column RID: the column names of T_ROWID come from the golden catalog
        # (tests/golden/exp_catalog.tsv, the Analyst's exp_parse.py over misc_exp_18c.dmp: ID|RID|URID)
        assert golden_columns("18c/misc_exp_18c.dmp", "T_ROWID") == ["ID", "RID", "URID"]
        assert (e.code, e.warning, e.table, e.row, e.column) == (
            "E-STRICT",
            "W-UNVERIFIED-TYPE",
            "T_ROWID",
            1,
            "RID",
        )
        # the same Dump reads another table
        assert dump.read_table("T_FRAME").read_all().num_rows == 7
