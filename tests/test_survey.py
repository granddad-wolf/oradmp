"""Task T-12: ``python -m oradmp survey`` (architecture 5.2b, 8.5). The NUMBER profile, the LONG and LOB counts
and the parts are counts; ``--no-names`` prints no table, column, partition or user name and no value. Expected
numbers: the plan (T-12 "Done when") and the Analyst's tools ``exp_parse.py`` / ``num_decode.py`` run read-only
over the fixtures named (Python ``decimal``, half away from zero), as in ``t12_survey.rs``."""

from __future__ import annotations

import io
import re
import subprocess
import sys
import warnings

import oradmp
import pytest
from conftest import FIXTURES, fixture
from oradmp._survey import write_survey

pytestmark = pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")


def survey(rel: str, **kw: bool) -> str:
    out = io.StringIO()
    assert write_survey(fixture(rel), out, **kw) == 0
    return out.getvalue()


def test_the_number_profile_of_t_num_column_v() -> None:
    text = survey("18c/t_num_exp_18c.dmp")
    v = text.split("2 V: type 2")[1].split("lenient read")[0]
    assert "values 32, nulls 1" in v
    assert "integers 19, fit Int64 11, 39-40 significant digits 1, invalid 0" in v
    assert "largest sig 40, frac 125, int 126; exp10 -125..125; leading digit position -125..125" in v
    # Decimal128(38, 10): 18 fit, 4 changed by round (1 to 0: row 27), 10 whose integer part does not fit
    assert "as decimal(38,10): exact 18, changed by round 4 (to 0: 1), integer part too large 10" in v
    # session 4, arch 5.2b: Decimal256(76, 38): 27 exact, 2 changed (1 to 0), 3 integer parts too large
    assert "as decimal256(76,38): exact 27, changed by round 2 (to 0: 1), integer part too large 3" in v
    summary = "decimal(38,10): columns 2, exact 51, changed by round 4 (to 0: 1), integer part too large 10; "
    assert summary + "columns that would need an override: 1" in text


def test_the_t_num3_column_is_flagged_with_six_values_of_39_or_40_digits() -> None:
    text = survey("18c/t_num3_exp_18c.dmp")
    v = text.split("2 V: type 2")[1].split("lenient read")[0]
    assert "39-40 significant digits 6" in v
    assert "largest sig 40, frac 40, int 2" in v


def long_line(values: int, chained: int, chunked: int, largest: int, above: int, band: int) -> str:
    return (
        f"LONG: values {values}, chained {chained}, chunked {chunked}, largest {largest}, "
        f"above 32,000 bytes {above}, zero-length pieces 0, 65,521 to 65,534 bytes {band}"
    )


def test_long_chain_and_lob_counts() -> None:
    assert long_line(2, 1, 0, 32000, 0, 0) in survey("18c/long_exp_direct_18c.dmp")
    # T-19 (ADR-95 item 8; `rowlen_stats.py`: three `fd ff` values per conventional long3 file, `long3_check.py`): the
    # survey prints `chunked`
    assert long_line(7, 0, 3, 100000, 7, 0) in survey("18c/long3_exp_18c.dmp")
    assert long_line(7, 7, 0, 100000, 7, 0) in survey("11g/long3_exp_direct_11g.dmp")
    # T-19b (ADR-101 item 4; `rowlen_stats.py`, `long4_check.py`, truth_long4.txt / truth_longraw4.txt): long4_exp_18c
    # has 7 values, two chunked (65,534 and 65,535), six of 65,521 to 65,534 bytes; longraw4_exp_18c has 4 values, two
    # chunked (65,535 and 100,000) and none in the band
    assert long_line(7, 0, 2, 65535, 7, 6) in survey("18c/long4_exp_18c.dmp")
    assert long_line(4, 0, 2, 100000, 4, 0) in survey("18c/longraw4_exp_18c.dmp")
    text = survey("18c/lob_exp_18c.dmp")
    assert "would raise E-LOB-UNCONFIRMED: short 0, long 0" in text
    assert "LOB: values" in text


def test_parts_of_a_partitioned_table() -> None:
    text = survey("18c/part_exp_18c.dmp")
    assert "part 1 (P2019): rows 1095, conventional" in text
    assert "part 2 (P2020): rows 1098, conventional" in text
    assert "part 3 (PMAX): rows 807, conventional" in text
    hidden = survey("18c/part_exp_18c.dmp", names=False)
    assert "part 2: rows 1098" in hidden
    assert "P2020" not in hidden


def names_and_values(rel: str) -> tuple[set[str], set[str]]:
    """Names (user, tables, columns, partitions) and text values of at least 6 characters of a fixture."""
    names: set[str] = set()
    values: set[str] = set()
    with oradmp.open(fixture(rel), on_value_error="null") as dump:
        names.add(dump.info.user)
        for t in dump.tables:
            names.add(t.name)
            names.update(c.name for c in t.columns)
            for p in t.parts:
                names.update(n for n in (p.partition, p.subpartition) if n)
            with warnings.catch_warnings():
                warnings.simplefilter("ignore")
                table = dump.read_table(t.ordinal).read_all()
            for col in table.itercolumns():
                if str(col.type) in ("string", "large_string"):
                    values.update(v for v in col.to_pylist() if isinstance(v, str) and len(v) >= 6)
    return {n for n in names if len(n) >= 3}, values


@pytest.mark.parametrize(
    "rel",
    [
        "18c/misc_exp_18c.dmp",
        "18c/lob_exp_18c.dmp",
        "18c/part_exp_18c.dmp",
        "18c/long_exp_direct_18c.dmp",
        "18c/own_exp_18c.dmp",
        "11g-cp1251/ru_exp_cp1251_11g.dmp",
        "11g-cp1251/ru_lob2_exp_cp1251_11g.dmp",
        "11g/multi_types_exp_11g.dmp",
    ],
)
def test_no_names_output_has_no_name_and_no_value(rel: str) -> None:
    names, values = names_and_values(rel)
    shown = survey(rel)
    hidden = survey(rel, names=False)
    # the control: with names the names are there
    assert any(n in shown for n in names), rel
    assert "file: " + rel.rsplit("/", 1)[1] in shown
    for n in names:
        assert not re.search(re.escape(n), hidden), (rel, n)
    for v in values:
        assert v not in hidden, (rel, "a value")
        assert v not in shown, (rel, "a value")
    assert rel.rsplit("/", 1)[1] not in hidden
    # no byte of the file as hex either
    assert "(hidden)" in hidden


def test_command_line_exit_code_and_output() -> None:
    p = subprocess.run(
        [sys.executable, "-m", "oradmp", "survey", "--no-names", str(FIXTURES / "18c/t_num_exp_18c.dmp")],
        capture_output=True,
        text=True,
        check=False,
    )
    assert p.returncode == 0, p.stderr
    assert "NUMBER: integers 19, fit Int64 11" in p.stdout
    assert "T_NUM" not in p.stdout
    bad = subprocess.run(
        [sys.executable, "-m", "oradmp", "survey", str(FIXTURES / "18c" / "SHA256SUMS")],
        capture_output=True,
        text=True,
        check=False,
    )
    assert bad.returncode == 1
    assert "E-NOT-EXP-FILE" in bad.stdout


def test_survey_prints_warnings_with_offsets_and_the_stop() -> None:
    # the failed export of the Russian set: W-POSSIBLY-INCOMPLETE with its offset (ADR-39)
    text = survey("11g-cp1251/ru_exp_us7ascii_11g.dmp")
    assert "[W-POSSIBLY-INCOMPLETE offset=0x" in text
    assert "warnings of the dump: " in text
