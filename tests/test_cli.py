"""Task T-18 (ADR-72, architecture 3.5): the command-line tool ``oradmp``.

Most tests call ``oradmp._cli.main(argv)`` in this process with captured stdout and stderr. Expected values come from
the API (``Dump.tables``, ``TableInfo.columns``, ``read_table``), from the truth files ``truth_text.txt`` and
``truth_num.txt`` of ``corpus/fixtures/18c`` (first three rows, row counts) and from facts of earlier tasks (the
patched ``T_NUM`` copy of T-13i (b): row 17 rounds, row 19 overflows; offsets in ``test_stream.py``). Nothing is taken
from the tool's own output.
"""

from __future__ import annotations

import csv
import errno
import io
import os
import re
import subprocess
import sys
from collections.abc import Callable
from decimal import Decimal
from pathlib import Path

import oradmp
import pyarrow as pa
import pyarrow.csv as pacsv
import pyarrow.parquet as pq
import pytest
from conftest import FIXTURES, fixture
from oradmp import _api, _cli, _core

Patch = Callable[[str, str, Callable[[bytearray], None]], str]
FOLDERS = ["18c", "11g", "11g-cp1251"]
T_NUM = "18c/t_num_exp_18c.dmp"
T_TEXT = "18c/t_text_exp_18c.dmp"

# The package under test is the one that is imported (the dev tree or an installed wheel); the child processes get its
# parent folder on PYTHONPATH, so that they import the same package (review 06a SF-1).
PKG = Path(oradmp.__file__).resolve().parent

pytestmark = pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")


def run(capsys: pytest.CaptureFixture[str], *argv: object) -> tuple[int, str, str]:
    """(exit code, stdout, stderr) of ``main`` with these arguments."""
    capsys.readouterr()
    code = _cli.main([str(a) for a in argv])
    cap = capsys.readouterr()
    return code, cap.out, cap.err


def exp_files(folder: str) -> list[Path]:
    return sorted(p for p in (FIXTURES / folder).iterdir() if "_exp_" in p.name and p.suffix == ".dmp")


def tsv(text: str) -> list[list[str]]:
    return [line.split("\t") for line in text.split("\n") if line]


def esc(s: str) -> str:
    """ADR-72 item 5, written again here: backslash, tab and newline in a name."""
    return s.replace("\\", "\\\\").replace("\t", "\\t").replace("\n", "\\n")


def yes(b: bool) -> str:
    return "true" if b else "false"


def none(v: object) -> str:
    return "" if v is None else esc(str(v))


# --- (a) the commands and their help ----------------------------------------------------------------------------


def test_help_lists_the_six_commands_and_the_options(capsys: pytest.CaptureFixture[str]) -> None:
    code, out, _ = run(capsys, "--help")
    assert code == 0
    for name in ("tables", "columns", "head", "export", "survey"):
        assert re.search(rf"^\s+{name}\s", out, re.M), name
    options = {
        "tables": [],
        "columns": ["--ordinal"],
        "head": ["-n", "--ordinal", "--columns"],
        "export": ["-o", "--format", "--all", "--overwrite", "--ordinal", "--columns"],
        "survey": ["--no-names", "--no-read"],
    }
    common = [
        "--strict",
        "--on-value-error",
        "--on-number-overflow",
        "--number-default",
        "--number-type",
        "--interval-as",
        "--timestamp-unit",
        "--trim-char",
        "--max-lob-bytes",
        "--unconfirmed-lob",
        "--db-charset",
        "--client-charset",
        "-q",
    ]
    for command, own in options.items():
        code, out, _ = run(capsys, command, "--help")
        assert code == 0
        for opt in own + (common if command != "survey" else []):
            assert opt in out, (command, opt)
    assert "string" in run(capsys, "head", "--help")[1]  # the default of --interval-as is named


def test_survey_through_the_new_entry_is_the_same_text_and_code(capsys: pytest.CaptureFixture[str]) -> None:
    from oradmp._survey import write_survey

    for rel in (T_NUM, "18c/part_exp_18c.dmp", "11g/lob_exp_11g.dmp"):
        for flags in ([], ["--no-names"], ["--no-read"]):
            expected = io.StringIO()
            assert (
                write_survey(fixture(rel), expected, names="--no-names" not in flags, read="--no-read" not in flags)
                == 0
            )
            code, out, err = run(capsys, "survey", fixture(rel), *flags)
            assert (code, out, err) == (0, expected.getvalue(), "")


def test_survey_of_a_file_that_is_not_a_dump_is_code_1(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    bad = tmp_path / "x.dmp"
    bad.write_bytes(b"not a dump" * 100)
    code, out, _ = run(capsys, "survey", bad)
    assert code == 1 and out.startswith("oradmp survey\nERROR: [E-")


# --- (b) tables, columns, head -----------------------------------------------------------------------------------


@pytest.mark.parametrize("folder", FOLDERS)
def test_tables_and_columns_equal_the_api_on_every_fixture(folder: str, capsys: pytest.CaptureFixture[str]) -> None:
    files = exp_files(folder)
    assert files
    for path in files:
        with oradmp.open(str(path), interval_as="string") as dump:
            want_tables = [
                [
                    str(t.ordinal),
                    esc(t.name),
                    str(t.num_rows),
                    str(len(t.columns)),
                    str(len(t.parts)),
                    t.path,
                    yes(t.partitioned),
                    yes(t.possibly_incomplete),
                ]
                for t in dump.tables
            ]
            code, out, err = run(capsys, "tables", path)
            assert code == 0, (path.name, err)
            rows = tsv(out)
            assert rows[0] == [
                "ordinal",
                "name",
                "rows",
                "columns",
                "parts",
                "path",
                "partitioned",
                "possibly_incomplete",
            ]
            assert rows[1:] == want_tables, path.name
            for t in dump.tables:
                want_columns = [
                    [
                        str(c.position),
                        esc(c.name),
                        c.oracle_type,
                        none(c.declared),
                        c.arrow_type,
                        yes(c.not_null),
                        c.type_source,
                    ]
                    for c in t.columns
                ]
                by_name = sum(1 for u in dump.tables if u.name == t.name) == 1
                args = [t.name] if by_name else ["--ordinal", str(t.ordinal)]
                code, out, err = run(capsys, "columns", path, *args)
                assert code == 0, (path.name, t.name, err)
                rows = tsv(out)
                assert rows[0] == [
                    "position",
                    "name",
                    "oracle_type",
                    "declared",
                    "arrow_type",
                    "not_null",
                    "type_source",
                ]
                assert rows[1:] == want_columns, (path.name, t.name)


def truth_lines(name: str) -> list[list[str]]:
    return [
        line.split("|")
        for line in (FIXTURES / "18c" / name).read_text().splitlines()
        if line.strip() and not line.startswith("#")
    ]


def test_row_counts_of_two_truth_tables(capsys: pytest.CaptureFixture[str]) -> None:
    # truth_text.txt and truth_num.txt hold one line per row
    for rel, truth, table in ((T_TEXT, "truth_text.txt", "T_TEXT"), (T_NUM, "truth_num.txt", "T_NUM")):
        code, out, _ = run(capsys, "tables", fixture(rel))
        assert code == 0
        (row,) = [r for r in tsv(out)[1:] if r[1] == table]
        assert int(row[2]) == len(truth_lines(truth))


def decode_dump(field: str) -> str | None:
    """``Typ=1 Len=11 CharacterSet=AL32UTF8: 70,6c`` (truth_text.txt) to the text; ``NULL`` to None."""
    if field == "NULL":
        return None
    head, _, hexes = field.partition(": ")
    codec = {"AL32UTF8": "utf-8", "AL16UTF16": "utf-16-be"}[re.search(r"CharacterSet=(\w+)", head).group(1)]  # type: ignore[union-attr]
    return bytes(int(h, 16) for h in hexes.split(",")).decode(codec)


def test_head_n_3_of_t_text_equals_the_truth_file(capsys: pytest.CaptureFixture[str]) -> None:
    code, out, err = run(capsys, "head", fixture(T_TEXT), "T_TEXT", "-n", "3")
    assert (code, err) == (0, "")
    rows = list(csv.reader(io.StringIO(out)))
    assert rows[0] == ["ID", "V", "C", "NV"]
    want = [[f[0], *(decode_dump(x) for x in f[3:6])] for f in truth_lines("truth_text.txt")[:3]]
    assert rows[1:] == want
    assert len(want) == 3 and want[1][1] == "Привет, мир"  # not a vacuous comparison


def test_head_n_3_of_t_num_equals_the_truth_file(capsys: pytest.CaptureFixture[str]) -> None:
    code, out, err = run(capsys, "head", fixture(T_NUM), "T_NUM", "-n", "3")
    assert (code, err) == (0, "")
    rows = list(csv.reader(io.StringIO(out)))
    assert rows[0] == ["ID", "V"]
    want = [(f[0], Decimal(f[1])) for f in truth_lines("truth_num.txt")[:3]]
    got = [(r[0], Decimal(r[1])) for r in rows[1:]]
    assert got == want and [w[1] for w in want] == [0, 1, -1]


def test_head_defaults_to_ten_rows_and_takes_columns_and_ordinal(capsys: pytest.CaptureFixture[str]) -> None:
    code, out, _ = run(capsys, "head", fixture(T_NUM), "--ordinal", "1")
    assert code == 0 and len(out.splitlines()) == 11  # header + 10 of 33
    code, out, _ = run(capsys, "head", fixture(T_NUM), "T_NUM", "-n", "2", "--columns", "V")
    assert code == 0 and out.splitlines()[0] == '"V"' and len(out.splitlines()) == 3
    code, out, _ = run(capsys, "head", fixture(T_NUM), "T_NUM", "-n", "0")
    assert code == 0 and out.splitlines() == ['"ID","V"']


def test_table_name_or_ordinal_but_not_both_and_not_none(capsys: pytest.CaptureFixture[str]) -> None:
    for extra in (["T_NUM", "--ordinal", "1"], []):
        code, out, err = run(capsys, "head", fixture(T_NUM), *extra)
        assert (code, out) == (2, "") and err.startswith("oradmp: error: [E-CONFIG] ")


# --- (c) export ---------------------------------------------------------------------------------------------------


def hex_table(table: pa.Table) -> pa.Table:
    """The table with binary columns as upper-case hex text, written again here from the rule of ADR-72 item 5."""
    cols = []
    for i, field in enumerate(table.schema):
        col = table.column(i)
        if pa.types.is_binary(field.type) or pa.types.is_large_binary(field.type):
            t = pa.large_string() if pa.types.is_large_binary(field.type) else pa.string()
            col = pa.chunked_array([pa.array([None if v is None else v.hex().upper() for v in col.to_pylist()], t)])
        cols.append(col)
    return pa.Table.from_arrays(cols, names=table.column_names)


def parquet_form(table: pa.Table) -> pa.Table:
    """What Parquet can hold of ``table``: it has no seconds unit, so pyarrow's writer stores ``timestamp[s]`` as
    milliseconds (same instants; found by this test on pyarrow 26.0.0 and 15.0.2, see the T-18 note). Nothing else
    differs."""
    fields = [
        f.with_type(pa.timestamp("ms", f.type.tz)) if pa.types.is_timestamp(f.type) and f.type.unit == "s" else f
        for f in table.schema
    ]
    return table.cast(pa.schema(fields, metadata=table.schema.metadata))


def readable_tables(path: Path) -> list[tuple[int, str, pa.Table]]:
    """The tables the API reads without an error with the tool's defaults (``interval_as="string"``)."""
    out = []
    with oradmp.open(str(path), interval_as="string") as dump:
        for t in dump.tables:
            try:
                with dump.read_table(t.ordinal) as r:
                    out.append((t.ordinal, t.name, r.read_all()))
            except oradmp.OradmpError:
                continue
    return out


@pytest.mark.parametrize("folder", ["18c", "11g"])
def test_export_parquet_and_csv_equal_the_api_table(
    folder: str, tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    compared = 0
    types: set[str] = set()
    for path in exp_files(folder):
        for ordinal, name, want in readable_tables(path):
            types.update(str(f.type) for f in want.schema)
            target = tmp_path / "t.parquet"
            code, out, err = run(capsys, "export", path, "--ordinal", ordinal, "-o", target, "--format", "parquet")
            assert (code, out) == (0, ""), (path.name, name)
            assert all(x.startswith("oradmp: warning: ") for x in err.splitlines()), err
            got = pq.read_table(target)
            expected_parquet = parquet_form(want)
            assert got.equals(expected_parquet, check_metadata=True), (path.name, name)
            assert got.schema.equals(expected_parquet.schema, check_metadata=True), (path.name, name)
            assert got.cast(want.schema).equals(want), (path.name, name)  # the same instants as the API's table
            target.unlink()

            target = tmp_path / "t.csv"
            code, out, err = run(capsys, "export", path, "--ordinal", ordinal, "-o", target)
            assert (code, out) == (0, ""), (path.name, name)
            assert all(x.startswith("oradmp: warning: ") for x in err.splitlines()), err
            expected = io.BytesIO()
            pacsv.write_csv(hex_table(want), expected)
            assert target.read_bytes() == expected.getvalue(), (path.name, name)
            target.unlink()
            compared += 1
    assert compared > 50
    # the Arrow types of the fixtures that went through both writers (the note lists them with the pyarrow version)
    assert {"binary", "large_binary", "int64", "string", "large_string"} <= types, types


def test_export_to_stdout_is_csv_and_parquet_to_stdout_is_refused(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    code, out, err = run(capsys, "export", fixture(T_NUM), "T_NUM", "-o", "-", "--columns", "ID")
    assert (code, err) == (0, "")
    assert out.splitlines()[0] == '"ID"' and len(out.splitlines()) == 34
    code, out, err = run(capsys, "export", fixture(T_NUM), "T_NUM", "-o", "-", "--format", "parquet")
    assert (code, out) == (2, "") and err.startswith("oradmp: error: [E-CONFIG] ") and "stdout" in err


def test_export_all_writes_the_manifest_and_one_file_per_table(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    path = FIXTURES / "18c" / "own_tables_exp_18c.dmp"
    for fmt, ext in (("csv", "csv"), ("parquet", "parquet")):
        folder = tmp_path / fmt
        folder.mkdir()
        code, out, err = run(capsys, "export", path, "--all", "-o", folder, "--format", fmt)
        assert code == 0, err
        with oradmp.open(str(path), interval_as="string") as dump:
            tables = dump.tables
            assert len(tables) > 1
            rows = tsv(out)
            assert rows[0] == ["ordinal", "name", "file", "rows"]
            assert rows[1:] == [[str(t.ordinal), t.name, f"{t.name}.{ext}", str(t.num_rows)] for t in tables]
            assert sorted(p.name for p in folder.iterdir()) == sorted(f"{t.name}.{ext}" for t in tables)
            for t in tables:
                with dump.read_table(t.ordinal) as r:
                    want = r.read_all()
                if fmt == "parquet":
                    assert pq.read_table(folder / f"{t.name}.parquet").equals(parquet_form(want), check_metadata=True)
                else:
                    expected = io.BytesIO()
                    pacsv.write_csv(hex_table(want), expected)
                    assert (folder / f"{t.name}.csv").read_bytes() == expected.getvalue()


def test_the_file_name_rule_of_export_all() -> None:
    names = _cli.table_file_names([(1, "EMP"), (2, "emp"), (3, "A/B"), (4, "Dept_1$#"), (5, "x y"), (6, "")], "csv")
    # EMP and emp are the same name ignoring case; a slash, a space and an empty name are not in [A-Za-z0-9_$#]
    assert names == ["table_1.csv", "table_2.csv", "table_3.csv", "Dept_1$#.csv", "table_5.csv", "table_6.csv"]
    assert _cli.table_file_names([(1, "EMP"), (2, "DEPT")], "parquet") == ["EMP.parquet", "DEPT.parquet"]
    assert _cli.table_file_names([(7, "a\nb"), (8, "é")], "csv") == ["table_7.csv", "table_8.csv"]


def test_export_all_refuses_an_existing_file_before_writing_any(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    path = FIXTURES / "18c" / "own_tables_exp_18c.dmp"
    with oradmp.open(str(path)) as dump:
        names = [t.name for t in dump.tables]
    (tmp_path / f"{names[-1]}.csv").write_text("mine")
    code, out, err = run(capsys, "export", path, "--all", "-o", tmp_path)
    assert (code, out) == (2, "") and err.startswith("oradmp: error: [E-CONFIG] ")
    assert sorted(p.name for p in tmp_path.iterdir()) == [f"{names[-1]}.csv"]
    code, out, err = run(capsys, "export", path, "--all", "-o", tmp_path, "--overwrite")
    assert code == 0 and (tmp_path / f"{names[-1]}.csv").read_text() != "mine"


# --- (d) the CSV writer's type coverage ---------------------------------------------------------------------------


def interval_fixture() -> tuple[str, str, str]:
    """(path, table, column) of an INTERVAL column, found through the API with ``interval_as="native"``."""
    for path in exp_files("18c"):
        with oradmp.open(str(path)) as dump:
            for t in dump.tables:
                for c in t.columns:
                    if c.arrow_type == "Interval(MonthDayNano)":
                        return str(path), t.name, c.name
    raise AssertionError("no INTERVAL YEAR TO MONTH column in the 18c fixtures")


@pytest.mark.parametrize("fmt", ["csv", "parquet"])
def test_a_refused_column_type_stops_before_any_row_naming_column_and_type(
    fmt: str, tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    path, table, column = interval_fixture()
    target = tmp_path / f"out.{fmt}"
    code, out, err = run(capsys, "export", path, table, "-o", target, "--interval-as", "native")
    assert (code, out) == (2, "")
    assert err.startswith("oradmp: error: [E-CONFIG] ")
    assert f"column {column} of Arrow type month_day_nano_interval" in err
    assert list(tmp_path.iterdir()) == []  # not even a partial file: refused before the file was opened
    # the same table with the tool's default (text) works
    code, _, err = run(capsys, "export", path, table, "-o", target)
    assert (code, err) == (0, "") and target.exists()


def test_every_arrow_type_of_the_fixtures_goes_through_the_csv_writer(tmp_path: Path) -> None:
    """The types the fixtures produce under every option of the tool that changes a type (native intervals are the
    one refused type, see above), probed with the tool's own preflight; the note lists the result per pyarrow."""
    seen: set[str] = set()
    for folder in FOLDERS:
        for path in exp_files(folder):
            with oradmp.open(str(path), interval_as="string", number_default="float64", timestamp_unit="ns") as dump:
                for t in dump.tables:
                    with dump.read_table(t.ordinal) as r:
                        schema = _cli._csv_schema(r.schema)
                    key = str(schema)
                    if key in seen:
                        continue
                    seen.add(key)
                    _cli._probe_columns(schema, _cli._csv_writer, "CSV")
    assert seen


# --- (e) exit codes -----------------------------------------------------------------------------------------------


def patch_v_row_17(b: bytearray) -> None:
    # as tests/test_stream.py patch_v_row_17 (T-13i (b)): `exp_parse.py --rows` on 18c/t_num_exp_18c.dmp, row 17 of
    # T_NUM; the V bytes are at 0x22F4 and the exponent byte c2 -> bc makes 1.23456E-10, which rounds to 1E-10
    assert bytes(b[0x22F2:0x22F9]) == bytes.fromhex("0500c202182e3d")
    b[0x22F4] = 0xBC


def only_error_line(err: str, prefix: str) -> None:
    lines = [line for line in err.splitlines() if line.startswith("oradmp: error: ")]
    assert len(lines) == 1 and lines[0].startswith(prefix), err


def test_exit_1_a_truncated_copy(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    data = (FIXTURES / T_NUM).read_bytes()
    bad = tmp_path / "trunc.dmp"
    bad.write_bytes(data[:9000])
    out_dir = tmp_path / "out"
    out_dir.mkdir()
    for argv in (["tables", bad], ["head", bad, "T_NUM"], ["export", bad, "T_NUM", "-o", out_dir / "x.csv"]):
        code, out, err = run(capsys, *argv)
        assert (code, out) == (1, ""), argv
        only_error_line(err, "oradmp: error: [E-TRUNCATED offset=0x")
    assert list(out_dir.iterdir()) == []  # (f)


def test_exit_1_the_patched_t_num_with_overflow_error(
    patched: Patch, tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    path = patched(T_NUM, "t_num_v17.dmp", patch_v_row_17)
    out_dir = tmp_path / "out"
    out_dir.mkdir()
    code, out, err = run(
        capsys, "export", path, "T_NUM", "-o", out_dir / "x.parquet", "--on-number-overflow", "error", "-q"
    )
    assert (code, out) == (1, "")
    only_error_line(err, "oradmp: error: [E-NUMBER-OVERFLOW offset=0x")
    assert list(out_dir.iterdir()) == []  # (f): the partial file was opened and removed


def test_exit_1_a_scan_that_stopped_prints_the_stop_error_after_the_tables(
    patched: Patch, tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    # as test_errors.py: the type of the first descriptor of T_NUM (0x223a, exp_parse.py) becomes 114
    def patch(b: bytearray) -> None:
        assert bytes(b[0x223A:0x223C]) == b"\x02\x00"
        b[0x223A:0x223C] = (114).to_bytes(2, "little")

    path = patched(T_NUM, "type114.dmp", patch)
    code, out, err = run(capsys, "tables", path)
    assert code == 1 and out.splitlines() == [
        "ordinal\tname\trows\tcolumns\tparts\tpath\tpartitioned\tpossibly_incomplete"
    ]
    only_error_line(err, "oradmp: error: [E-UNSUPPORTED-TYPE offset=0x223a]")
    code, out, err = run(capsys, "export", path, "--all", "-o", tmp_path)
    assert code == 1 and out.splitlines() == ["ordinal\tname\tfile\trows"]
    only_error_line(err, "oradmp: error: [E-UNSUPPORTED-TYPE offset=0x223a]")
    assert [p.name for p in tmp_path.iterdir()] == ["type114.dmp"]


def test_exit_1_a_conventional_long_stop_of_r8_prints_the_code_the_message_and_the_offset(
    patched: Patch, capsys: pytest.CaptureFixture[str]
) -> None:
    # T-19b (ADR-99 item 1): the copy of test_errors.py (long4_exp_18c.dmp, u16 at 0x22277 becomes `fb ff`) stops the
    # scan with E-UNSUPPORTED-LAYOUT; `oradmp tables` prints the (empty) table list and the stop error, exit 1
    def patch(b: bytearray) -> None:
        assert bytes(b[0x22277:0x22279]) == b"\xfa\xff"
        b[0x22277:0x22279] = b"\xfb\xff"

    path = patched("18c/long4_exp_18c.dmp", "long4_fbff.dmp", patch)
    code, out, err = run(capsys, "tables", path)
    assert code == 1 and out.splitlines() == [
        "ordinal\tname\trows\tcolumns\tparts\tpath\tpartitioned\tpossibly_incomplete"
    ]
    only_error_line(err, "oradmp: error: [E-UNSUPPORTED-LAYOUT offset=0x22277]")
    assert "conventional LONG or LONG RAW field starting 0xfffb or 0xfffc, not in the fixtures [R8]" in err


def test_exit_1_a_table_not_found_in_a_scan_that_stopped_gives_the_stop_error(
    patched: Patch, tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    # T-18b (c): same patched copy as above (the scan stops at the first descriptor of T_NUM); a name that is not in the
    # catalog may be behind the stop, so the stop error (code 1) is reported, not E-NO-SUCH-TABLE (code 2)
    def patch(b: bytearray) -> None:
        assert bytes(b[0x223A:0x223C]) == b"\x02\x00"
        b[0x223A:0x223C] = (114).to_bytes(2, "little")

    path = patched(T_NUM, "type114b.dmp", patch)
    for argv in (
        ["columns", path, "NO_SUCH"],
        ["head", path, "NO_SUCH"],
        ["export", path, "NO_SUCH", "-o", tmp_path / "n.csv"],
    ):
        code, out, err = run(capsys, *argv)
        assert (code, out) == (1, ""), argv
        only_error_line(err, "oradmp: error: [E-UNSUPPORTED-TYPE offset=0x223a]")
    assert not (tmp_path / "n.csv").exists()
    # a complete scan keeps E-NO-SUCH-TABLE, exit 2
    code, _, err = run(capsys, "columns", fixture(T_NUM), "NO_SUCH")
    assert code == 2 and err.startswith("oradmp: error: [E-NO-SUCH-TABLE")


def test_exit_1_a_strict_error(capsys: pytest.CaptureFixture[str]) -> None:
    code, out, err = run(capsys, "tables", fixture("18c/t_tstz_exp_18c.dmp"), "--strict")
    assert (code, out) == (1, "") and err.startswith("oradmp: error: [E-")


def test_exit_2_usage(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    f = fixture(T_NUM)
    cases: list[tuple[list[str | Path], str | None]] = [
        (["head", f, "NO_SUCH"], "E-NO-SUCH-TABLE"),
        (["columns", f, "NO_SUCH"], "E-NO-SUCH-TABLE"),
        (["head", f, "T_NUM", "--columns", "ZZ"], "E-NO-SUCH-COLUMN"),
        (["tables", f, "--on-number-overflow", "sometimes"], "E-CONFIG"),
        (["tables", f, "--number-type", "no-equal-sign"], "E-CONFIG"),
        (["export", f, "T_NUM", "-o", tmp_path / "x.txt"], "E-CONFIG"),  # no format from the suffix
        (["export", f, "T_NUM", "--all", "-o", tmp_path], "E-CONFIG"),
        (["export", f], None),  # argparse: -o is required
        (["frobnicate"], None),
        ([], None),
    ]
    for argv, code_name in cases:
        code, out, err = run(capsys, *argv)
        assert (code, out) == (2, ""), argv
        if code_name:
            only_error_line(err, f"oradmp: error: [{code_name}")
    assert list(tmp_path.iterdir()) == []


def test_exit_2_an_existing_target_without_overwrite(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    target = tmp_path / "x.csv"
    target.write_text("mine")
    code, out, err = run(capsys, "export", fixture(T_TEXT), "T_TEXT", "-o", target)
    assert (code, out) == (2, "")
    only_error_line(err, "oradmp: error: [E-CONFIG] ")
    assert target.read_text() == "mine" and [p.name for p in tmp_path.iterdir()] == ["x.csv"]
    code, _, _ = run(capsys, "export", fixture(T_TEXT), "T_TEXT", "-o", target, "--overwrite")
    assert code == 0 and target.read_text() != "mine"


def test_exit_3_a_missing_dump_file(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    for argv in (["tables", tmp_path / "gone.dmp"], ["head", tmp_path / "gone.dmp", "T_TEXT"]):
        code, out, err = run(capsys, *argv)
        assert (code, out) == (3, ""), argv
        only_error_line(err, "oradmp: error: [E-IO] ")


def test_exit_3_an_output_folder_that_cannot_be_written(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    code, out, err = run(capsys, "export", fixture(T_TEXT), "T_TEXT", "-o", tmp_path / "no_such_folder" / "x.csv")
    assert (code, out) == (3, "")
    only_error_line(err, "oradmp: error: [E-IO] ")
    assert list(tmp_path.iterdir()) == []


@pytest.mark.skipif(hasattr(os, "geteuid") and os.geteuid() == 0, reason="root can write to a read-only folder")
@pytest.mark.skipif(sys.platform == "win32", reason="POSIX permissions")
def test_exit_3_a_read_only_output_folder(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    ro = tmp_path / "ro"
    ro.mkdir()
    ro.chmod(0o500)
    try:
        code, out, err = run(capsys, "export", fixture(T_TEXT), "T_TEXT", "-o", ro / "x.csv")
    finally:
        ro.chmod(0o700)
    assert (code, out) == (3, "")
    only_error_line(err, "oradmp: error: [E-IO] ")
    assert list(ro.iterdir()) == []


@pytest.mark.parametrize("fmt", ["csv", "parquet"])
def test_exit_130_an_interrupt_in_the_writer_leaves_nothing(
    fmt: str, tmp_path: Path, capsys: pytest.CaptureFixture[str], monkeypatch: pytest.MonkeyPatch
) -> None:
    def interrupt(writer: object, batch: object) -> None:
        raise KeyboardInterrupt

    monkeypatch.setattr(_cli, "_write_batch", interrupt)
    code, out, err = run(capsys, "export", fixture(T_TEXT), "T_TEXT", "-o", tmp_path / f"x.{fmt}")
    assert (code, out) == (130, "") and err.startswith("oradmp: interrupted")
    assert list(tmp_path.iterdir()) == []


def test_every_core_error_code_has_an_exit_code() -> None:
    from oradmp import ERROR_CLASSES

    assert set(_core.ERROR_CODES) == set(ERROR_CLASSES)
    for code in _core.ERROR_CODES:
        cls = ERROR_CLASSES[code]
        assert _cli.exit_code_for_class(cls) in (1, 2, 3, 4), code
    # the map itself: a class taken out is a code without an exit code
    assert _cli.exit_code_for_class(oradmp.UnsupportedCharset) == 1
    assert _cli.exit_code_for_class(oradmp.InternalError) == 4
    assert _cli.exit_code_for_class(OSError) == 3
    assert _cli.exit_code_for_class(KeyError) is None


# --- (f) no partial file ------------------------------------------------------------------------------------------


def test_a_failing_export_to_an_existing_target_with_overwrite_keeps_the_old_file(
    patched: Patch, tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    path = patched(T_NUM, "t_num_v17.dmp", patch_v_row_17)
    target = tmp_path / "x.csv"
    target.write_text("old")
    code, _, _ = run(capsys, "export", path, "T_NUM", "-o", target, "--overwrite", "-q")
    assert code == 1
    assert target.read_text() == "old" and [p.name for p in tmp_path.iterdir() if p.name != "t_num_v17.dmp"] == [
        "x.csv"
    ]


def test_export_all_stops_at_the_first_error_and_keeps_the_complete_files(
    patched: Patch, tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    # 18c/t_num_exp_18c.dmp holds one table; a patched copy with a bad row stops it: no file at all for it
    path = patched(T_NUM, "t_num_v17.dmp", patch_v_row_17)
    out_dir = tmp_path / "out"
    out_dir.mkdir()
    code, out, _ = run(capsys, "export", path, "--all", "-o", out_dir, "-q")
    assert code == 1 and out.splitlines() == ["ordinal\tname\tfile\trows"]
    assert list(out_dir.iterdir()) == []


def test_export_all_keeps_the_files_written_before_the_first_error(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    # multi_empty2: T_EMPTY (ID, NAME, D) then T_FRAME (A, B, C); --columns ID is fine for the first, not the second
    path = FIXTURES / "18c" / "multi_empty2_exp_18c.dmp"
    code, out, err = run(capsys, "export", path, "--all", "-o", tmp_path, "--columns", "ID")
    assert code == 2 and err.startswith("oradmp: error: [E-NO-SUCH-COLUMN")
    assert out.splitlines() == ["ordinal\tname\tfile\trows", "1\tT_EMPTY\tT_EMPTY.csv\t0"]
    assert [p.name for p in tmp_path.iterdir()] == ["T_EMPTY.csv"]
    assert (tmp_path / "T_EMPTY.csv").read_text() == '"ID"\n'


# --- (g) warnings -------------------------------------------------------------------------------------------------


def test_warnings_go_to_stderr_once_and_not_through_the_warnings_module(
    patched: Patch, tmp_path: Path, capsys: pytest.CaptureFixture[str], recwarn: pytest.WarningsRecorder
) -> None:
    path = patched(T_NUM, "t_num_v17.dmp", patch_v_row_17)
    # the text of the warning, from the API under the same options as the tool's defaults
    with oradmp.open(path, interval_as="string") as dump, dump.read_table("T_NUM") as reader:
        with pytest.raises(oradmp.ValueDecodeError):
            reader.read_all()
        (w,) = reader.warnings
    assert w.code == "W-NUMBER-ROUNDED"
    recwarn.clear()
    code, out, err = run(capsys, "export", path, "T_NUM", "-o", tmp_path / "x.csv")
    assert (code, out) == (1, "")  # row 19 of T_NUM does not fit in any case (test_stream.py)
    lines = err.splitlines()
    assert [x for x in lines if x.startswith("oradmp: warning: ")] == [f"oradmp: warning: {w.text}"]
    assert len([x for x in lines if x.startswith("oradmp: error: [E-NUMBER-OVERFLOW offset=0x")]) == 1
    assert len(lines) == 2
    code, out, err = run(capsys, "export", path, "T_NUM", "-o", tmp_path / "y.csv", "-q")
    assert code == 1 and "warning" not in err and err.startswith("oradmp: error: [E-NUMBER-OVERFLOW")
    assert len(recwarn) == 0, [str(x.message) for x in recwarn]


def test_warnings_of_a_successful_export_go_to_stderr(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    # T-18b (d): 18c T_NUM with --on-value-error null exits 0; stderr is the reader's warnings, in order
    with (
        oradmp.open(fixture(T_NUM), interval_as="string") as dump,
        dump.read_table("T_NUM", on_value_error="null") as reader,
    ):
        reader.read_all()
        texts = [w.text for w in reader.warnings]
    assert texts, "the fixture must give at least one warning"
    code, _, err = run(capsys, "export", fixture(T_NUM), "T_NUM", "--on-value-error", "null", "-o", tmp_path / "t.csv")
    assert code == 0
    assert err.splitlines() == [f"oradmp: warning: {t}" for t in texts]
    code, _, err = run(
        capsys, "export", fixture(T_NUM), "T_NUM", "--on-value-error", "null", "-o", tmp_path / "t2.csv", "-q"
    )
    assert (code, err) == (0, "")


def test_the_scan_warnings_print_once_after_the_scan(capsys: pytest.CaptureFixture[str]) -> None:
    path = fixture("18c/t_tstz_exp_18c.dmp")
    with oradmp.open(path) as dump:
        dump.scan()
        texts = [w.text for w in dump.warnings]
    assert texts
    code, _, err = run(capsys, "tables", path)
    assert code == 0 and err.splitlines() == [f"oradmp: warning: {t}" for t in texts]
    code, _, err = run(capsys, "tables", path, "-q")
    assert (code, err) == (0, "")


# --- T-18c (b) SF-2: one test per survivor of review 06a ----------------------------------------------------------


def test_the_format_comes_from_the_option_else_the_suffix_in_any_case() -> None:
    for output, want in (("x.csv", "csv"), ("x.parquet", "parquet"), ("x.PARQUET", "parquet"), ("x.Csv", "csv")):
        assert _cli._format_for(output, None, everything=False) == want, output
    assert _cli._format_for("-", None, everything=False) == "csv"
    assert _cli._format_for("some_folder", None, everything=True) == "csv"
    assert _cli._format_for("x.csv", "parquet", everything=False) == "parquet"
    assert _cli._format_for("x.parquet", "csv", everything=False) == "csv"
    for bad in ("x.txt", "noext"):
        with pytest.raises(oradmp.UsageError, match=r"^\[E-CONFIG\]"):
            _cli._format_for(bad, None, everything=False)


def test_export_takes_parquet_from_the_suffix(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    target = tmp_path / "t.parquet"
    code, _, _ = run(capsys, "export", fixture(T_TEXT), "T_TEXT", "-o", target)
    assert code == 0
    table = pq.read_table(target)  # a CSV file would not open
    assert table.num_rows == 10 and table.column_names == ["ID", "V", "C", "NV"]


def test_head_with_a_negative_count_in_a_subprocess_is_exit_2() -> None:
    """T-18d (c), N-2: without the guard the core aborts the process (exit 134); here that fails by name."""
    r = subprocess.run(
        [sys.executable, "-m", "oradmp", "head", fixture(T_TEXT), "T_TEXT", "-n", "-1"],
        capture_output=True,
        env=child_env(),
        check=False,
    )
    assert r.returncode == 2, (r.returncode, r.stderr[-300:])
    assert r.stdout == b""
    lines = r.stderr.decode().splitlines()
    assert len(lines) == 1 and lines[0].startswith("oradmp: error: [E-CONFIG]"), lines


def test_export_all_to_stdout_is_refused_and_creates_nothing(
    tmp_path: Path, capsys: pytest.CaptureFixture[str], monkeypatch: pytest.MonkeyPatch
) -> None:
    monkeypatch.chdir(tmp_path)
    code, out, err = run(capsys, "export", fixture(T_TEXT), "--all", "-o", "-")
    assert (code, out) == (2, "")
    only_error_line(err, "oradmp: error: [E-CONFIG]")
    assert list(tmp_path.iterdir()) == []


def test_version_prints_the_package_version(capsys: pytest.CaptureFixture[str]) -> None:
    code, out, err = run(capsys, "--version")
    assert (code, out, err) == (0, f"oradmp {oradmp.__version__}\n", "")
    assert oradmp.__version__ != "0+unknown"


def api_csv(rel: str, table: str, rows: int, **options: object) -> str:
    """The first ``rows`` rows of the API read of ``table``, written by pyarrow's CSV writer."""
    with oradmp.open(fixture(rel), **options) as dump:  # type: ignore[arg-type]
        reader = dump.read_table(table, batch_rows=rows)
        t = pa.Table.from_batches([next(iter(reader))]).slice(0, rows)  # the first batch only (row 19 overflows)
        reader.close()
    sink = io.BytesIO()
    pacsv.write_csv(t, sink)
    return sink.getvalue().decode()


def test_trim_char_reaches_the_reader(capsys: pytest.CaptureFixture[str]) -> None:
    on = api_csv(T_TEXT, "T_TEXT", 3, trim_char=True)
    off = api_csv(T_TEXT, "T_TEXT", 3, trim_char=False)
    assert on != off  # the case can see the option
    assert run(capsys, "head", fixture(T_TEXT), "T_TEXT", "-n", "3", "--trim-char") == (0, on, "")
    assert run(capsys, "head", fixture(T_TEXT), "T_TEXT", "-n", "3") == (0, off, "")


def test_number_type_reaches_the_reader(capsys: pytest.CaptureFixture[str]) -> None:
    # T_NUM.V rows 1 to 3 are 0, 1, -1: decimal(38,10) writes 0.0000000000, string writes 0
    default = api_csv(T_NUM, "T_NUM", 3)
    as_text = api_csv(T_NUM, "T_NUM", 3, number_types={"T_NUM.V": "string"})
    assert default != as_text
    assert run(capsys, "head", fixture(T_NUM), "T_NUM", "-n", "3") == (0, default, "")
    code, out, err = run(capsys, "head", fixture(T_NUM), "T_NUM", "-n", "3", "--number-type", "T_NUM.V=string")
    assert (code, out, err) == (0, as_text, "")


def test_the_escape_of_names() -> None:
    assert _cli._escape("a\tb") == "a\\tb"
    assert _cli._escape("a\nb") == "a\\nb"
    assert _cli._escape("a\\b") == "a\\\\b"
    assert _cli._row([7, "x\ty\nz\\w", None, True]) == "7\tx\\ty\\nz\\\\w\t\ttrue\n"


class Stop(Exception):
    pass


class FakeDump:
    """Stands in for ``oradmp.Dump``: records the keyword arguments of ``read_table`` and stops there."""

    def __init__(self, calls: list[dict[str, object]]) -> None:
        self.calls = calls
        self.warnings: list[object] = []

    def scan(self) -> None:
        return None

    def close(self) -> None:
        return None

    def read_table(self, spec: object, **kwargs: object) -> None:
        self.calls.append(kwargs)
        raise Stop


PASS_THROUGH: list[tuple[list[str], str, object]] = [
    (["--strict"], "strict", True),
    (["--on-value-error", "null"], "on_value_error", "null"),
    (["--on-number-overflow", "error"], "on_number_overflow", "error"),
    (["--number-default", "decimal(30,5)"], "number_default", "decimal(30,5)"),
    (["--number-type", "T.C=int64"], "number_types", {"T.C": "int64"}),
    (["--number-type", "T.C=int64", "--number-type", "T.D=string"], "number_types", {"T.C": "int64", "T.D": "string"}),
    (["--interval-as", "native"], "interval_as", "native"),
    (["--timestamp-unit", "ms"], "timestamp_unit", "ms"),
    (["--trim-char"], "trim_char", True),
    (["--max-lob-bytes", "1234"], "max_lob_bytes", 1234),
    (["--unconfirmed-lob", "decode"], "unconfirmed_lob", "decode"),
    (["--db-charset", "WE8MSWIN1252"], "db_charset", "WE8MSWIN1252"),
    (["--client-charset", "AL32UTF8"], "client_charset", "AL32UTF8"),
]


@pytest.mark.parametrize(
    ("extra", "key", "value"), PASS_THROUGH, ids=[c[1] + str(i) for i, c in enumerate(PASS_THROUGH)]
)
def test_every_read_option_reaches_the_api(
    extra: list[str], key: str, value: object, monkeypatch: pytest.MonkeyPatch
) -> None:
    opened: list[dict[str, object]] = []

    def fake_open(path: str, **kwargs: object) -> FakeDump:
        opened.append(kwargs)
        return FakeDump([])

    monkeypatch.setattr(_api, "open", fake_open)
    with pytest.raises(Stop):
        _cli.main(["head", "any.dmp", "T", *extra])
    (kwargs,) = opened
    assert kwargs[key] == value
    assert kwargs["interval_as"] == ("native" if key == "interval_as" else "string")
    if key not in ("strict", "trim_char"):
        assert kwargs["strict"] is False and kwargs["trim_char"] is False


def test_columns_reaches_the_api(monkeypatch: pytest.MonkeyPatch) -> None:
    reads: list[dict[str, object]] = []

    def fake_open(path: str, **kwargs: object) -> FakeDump:
        return FakeDump(reads)

    monkeypatch.setattr(_api, "open", fake_open)
    with pytest.raises(Stop):
        _cli.main(["head", "any.dmp", "T", "--columns", "A,B"])
    assert reads[0]["columns"] == ["A", "B"]
    reads.clear()
    with pytest.raises(Stop):
        _cli.main(["export", "any.dmp", "T", "-o", "-", "--columns", "A"])
    assert reads[0]["columns"] == ["A"]


# --- T-18c (c) N-3: the I/O error line names the reason and the file -----------------------------------------------


def io_line(path: object, errno_: int) -> str:
    return f"oradmp: error: [E-IO] {os.strerror(errno_)}: {path}\n"


def test_exit_3_a_missing_dump_names_the_reason_and_the_path(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    gone = str(tmp_path / "gone.dmp")
    folder = tmp_path / "out"
    folder.mkdir()
    for argv in (
        ["tables", gone],
        ["columns", gone, "T_TEXT"],
        ["head", gone, "T_TEXT"],
        ["export", gone, "T_TEXT", "-o", folder / "x.csv"],
        ["export", gone, "--all", "-o", folder],
    ):
        code, out, err = run(capsys, *argv)
        assert (code, out) == (3, ""), argv
        assert err == io_line(gone, errno.ENOENT), argv
    assert list(folder.iterdir()) == []


@pytest.mark.skipif(hasattr(os, "geteuid") and os.geteuid() == 0, reason="root can read a file of mode 000")
@pytest.mark.skipif(sys.platform == "win32", reason="POSIX permissions")
def test_exit_3_an_unreadable_dump_names_the_reason_and_the_path(
    tmp_path: Path, capsys: pytest.CaptureFixture[str]
) -> None:
    locked = tmp_path / "locked.dmp"
    locked.write_bytes((FIXTURES / T_TEXT).read_bytes())
    locked.chmod(0)
    try:
        code, out, err = run(capsys, "tables", locked)
    finally:
        locked.chmod(0o600)
    assert (code, out) == (3, "")
    assert err == io_line(locked, errno.EACCES)


def test_the_io_error_text_of_the_api_error() -> None:
    """T-18d (a), SF-1: the errors the API builds (``make_error``), with and without an error number."""
    from oradmp._errors import make_error

    def api_error(message: str, errno_: int | None) -> BaseException:
        return make_error("E-IO", message, None, None, None, None, None, None, None, None, None, errno_)

    text = _cli._error_text
    assert text(api_error("[E-IO] I/O error", None), "a.dmp") == "[E-IO] I/O error: a.dmp"
    assert text(api_error("[E-IO offset=0x10] I/O error", None), "a.dmp") == "[E-IO offset=0x10] I/O error: a.dmp"
    assert text(api_error("[E-IO] I/O error", errno.ENOENT), "a.dmp") == f"[E-IO] {os.strerror(errno.ENOENT)}: a.dmp"
    assert text(api_error("[E-IO] I/O error", errno.EACCES), "a.dmp") == "[E-IO] Permission denied: a.dmp"
    assert text(api_error("[E-IO offset=0x1f] I/O error", errno.EIO), "a.dmp") == (
        f"[E-IO offset=0x1f] {os.strerror(errno.EIO)}: a.dmp"
    )
    assert text(api_error("[E-IO] I/O error", errno.ENOENT), "a\tb\r\nc\\d") == (
        "[E-IO] No such file or directory: a\\tb\\r\\nc\\d"
    )
    # an output-side error has no code: it keeps its form
    assert text(OSError(errno.EACCES, "Permission denied", "x.csv"), "a.dmp") == "[E-IO] Permission denied: x.csv"
    # the real exception of a NUL byte in the path (an OSError without errno, code E-IO)
    with pytest.raises(OSError) as info:
        oradmp.open("a\x00b.dmp")
    assert getattr(info.value, "code", None) == "E-IO"
    got = text(info.value, "a.dmp")
    assert got.startswith("[E-IO") and got.endswith(": a.dmp") and "[E-IO] [E-IO" not in got, got


def test_survey_of_a_missing_file_keeps_its_own_text(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    code, out, err = run(capsys, "survey", tmp_path / "missing.dmp")
    assert code == 1 and err == "" and out.startswith("oradmp survey\nERROR: ")


# --- (h) a closed pipe --------------------------------------------------------------------------------------------


def child_env() -> dict[str, str]:
    return dict(os.environ, PYTHONPATH=str(PKG.parent))


def tool_command() -> list[str]:
    script = Path(sys.executable).parent / "oradmp"
    return [str(script)] if script.exists() else [sys.executable, "-m", "oradmp"]


def test_a_closed_stdout_ends_head_quietly_with_exit_0() -> None:
    env = child_env()
    proc = subprocess.Popen(
        [*tool_command(), "head", fixture(T_TEXT), "T_TEXT", "-n", "5"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env=env,
    )
    assert proc.stdout is not None and proc.stderr is not None
    proc.stdout.close()  # nobody reads: the first write meets a closed pipe
    err = proc.stderr.read()
    assert proc.wait(timeout=60) == 0
    assert err == b""


def test_a_closed_stdout_of_export_to_stdout_is_exit_3() -> None:
    env = child_env()
    proc = subprocess.Popen(
        [*tool_command(), "export", fixture(T_TEXT), "T_TEXT", "-o", "-"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        env=env,
    )
    assert proc.stdout is not None and proc.stderr is not None
    proc.stdout.close()
    err = proc.stderr.read()
    assert proc.wait(timeout=60) == 3
    assert err.startswith(b"oradmp: error: [E-IO]")


def test_the_subprocess_imports_the_same_package() -> None:
    r = subprocess.run(
        [sys.executable, "-c", "import oradmp; print(oradmp.__file__)"],
        capture_output=True,
        env=child_env(),
        check=False,
    )
    assert r.returncode == 0, r.stderr.decode()[-500:]
    assert Path(r.stdout.decode().strip()).resolve().parent == PKG


def test_the_module_entry_is_the_same_tool(tmp_path: Path) -> None:
    env = child_env()
    r = subprocess.run(
        [sys.executable, "-m", "oradmp", "tables", fixture(T_TEXT)], capture_output=True, env=env, check=False
    )
    assert r.returncode == 0 and r.stdout.decode().splitlines()[1].startswith("1\tT_TEXT\t10\t4\t1\t")


# --- (i) no pandas ------------------------------------------------------------------------------------------------


def test_no_file_of_the_package_imports_pandas() -> None:
    pattern = re.compile(r"^\s*(import|from)\s+pandas\b|__import__\(\s*[\"']pandas|import_module\(\s*[\"']pandas", re.M)
    files = sorted(PKG.glob("*.py"))
    assert PKG / "_cli.py" in files and PKG / "__init__.py" in files
    for f in files:
        assert not pattern.search(f.read_text()), f
    # the scan itself finds what it looks for
    assert pattern.search("import pandas as pd\n") and pattern.search("    from pandas import x\n")


def test_the_cli_does_not_load_pandas_when_it_runs(tmp_path: Path) -> None:
    code = (
        "import sys\n"
        "class Block:\n"
        "    def find_spec(self, name, path=None, target=None):\n"
        "        if name == 'pandas' or name.startswith('pandas.'):\n"
        "            raise ModuleNotFoundError('pandas is blocked: ' + name)\n"
        "sys.meta_path.insert(0, Block())  # any `import pandas` now fails, as in a venv without pandas\n"
        "import oradmp._cli as c\n"
        f"assert c.main(['export', {fixture(T_TEXT)!r}, 'T_TEXT', '-o', {str(tmp_path / 'x.parquet')!r}]) == 0\n"
        f"assert c.main(['export', {fixture(T_TEXT)!r}, 'T_TEXT', '-o', {str(tmp_path / 'x.csv')!r}]) == 0\n"
        "assert 'pandas' not in sys.modules\n"
    )
    env = child_env()
    r = subprocess.run([sys.executable, "-c", code], capture_output=True, env=env, check=False)
    assert r.returncode == 0, r.stderr.decode()[-500:]
