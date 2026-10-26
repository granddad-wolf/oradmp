"""Task T-12: batches reach pyarrow unchanged. Each table of five fixtures is read through ``BatchReader``
(``read_all`` and the iterator), through ``pyarrow.RecordBatchReader.from_stream`` (the Arrow PyCapsule
Interface) and compared with the digest the Rust core computes from its own batches (``_core.digest``; the
canonical form is in ``conftest.py`` and ``oradmp-py/src/digest.rs``)."""

from __future__ import annotations

import oradmp
import pyarrow as pa
import pytest
from conftest import fixture, table_digest
from oradmp import _core

pytestmark = pytest.mark.filterwarnings("ignore::oradmp.OradmpWarning")

FILES = [
    "18c/multi_types_exp_18c.dmp",  # every scalar type family
    "18c/lob_exp_18c.dmp",  # CLOB, NCLOB, BLOB, short and long values
    "18c/part_exp_18c.dmp",  # a partitioned table: three parts, three batches
    "11g-cp1251/ru_exp_cp1251_11g.dmp",  # CL8MSWIN1251 database
    "11g/misc2_exp_11g.dmp",  # intervals, rowids
    "11g-cp1251/ru_lob2_exp_cp1251_11g.dmp",  # R_LOB2: CLOB / NCLOB / BLOB values in the CL8MSWIN1251 database
]
LENIENT = {"on_value_error": "null"}


def rust_digest(path: str, ordinal: int) -> list[tuple[str, int, int, int]]:
    d = _core.open(path, {})
    opts = {"on_value_error": "null", "on_number_overflow": "round"}
    return d.digest(ordinal, opts)


@pytest.mark.parametrize("rel", FILES)
def test_three_paths_give_what_rust_read(rel: str) -> None:
    path = fixture(rel)
    with oradmp.open(path, **LENIENT) as dump:  # type: ignore[arg-type]
        tables = dump.tables
        assert tables
        for t in tables:
            want = rust_digest(path, t.ordinal)
            # 1. BatchReader.read_all
            with dump.read_table(t.ordinal) as reader:
                table = reader.read_all()
            assert table.num_rows == t.num_rows, t.name
            assert table_digest(table) == want, (rel, t.name, "BatchReader.read_all")
            # 2. the iterator: pyarrow.RecordBatch objects, same schema
            batches = list(dump.read_table(t.name if sum(x.name == t.name for x in tables) == 1 else t.ordinal))
            assert all(isinstance(b, pa.RecordBatch) for b in batches)
            assert table_digest(pa.Table.from_batches(batches, schema=table.schema)) == want
            # 3. pyarrow.RecordBatchReader.from_stream (Arrow PyCapsule Interface)
            stream = pa.RecordBatchReader.from_stream(dump.read_table(t.ordinal))
            via_stream = stream.read_all()
            assert via_stream.schema == table.schema
            assert table_digest(via_stream) == want, (rel, t.name, "from_stream")


def test_every_column_arrives_in_the_declared_order_with_the_core_schema() -> None:
    with oradmp.open(fixture("18c/multi_types_exp_18c.dmp"), **LENIENT) as dump:  # type: ignore[arg-type]
        for t in dump.tables:
            schema = dump.read_table(t.ordinal).schema
            assert schema.names == [c.name for c in t.columns]
            for field, c in zip(schema, t.columns, strict=True):
                assert field.metadata[b"oradmp:type_code"] == str(c.type_code).encode()


def test_partitioned_table_batches_never_mix_parts() -> None:
    with oradmp.open(fixture("18c/part_exp_18c.dmp")) as dump:
        t = next(x for x in dump.tables if x.name == "T_PART")
        assert [p.num_rows for p in t.parts] == [1095, 1098, 807]
        sizes = [b.num_rows for b in dump.read_table("T_PART")]
        assert sizes == [1095, 1098, 807]


def test_read_all_returns_every_table_by_name() -> None:
    with oradmp.open(fixture("18c/misc_exp_18c.dmp"), **LENIENT) as dump:  # type: ignore[arg-type]
        all_tables = dump.read_all()
        assert set(all_tables) == {t.name for t in dump.tables}
        for t in dump.tables:
            assert all_tables[t.name].num_rows == t.num_rows


def test_columns_selection_keeps_the_table_order() -> None:
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp"), **LENIENT) as dump:  # type: ignore[arg-type]
        table = dump.read_table("T_NUM", columns=["V"]).read_all()
        assert table.column_names == ["V"]
        assert table.num_rows == 33


def test_the_arrow_c_stream_spends_the_reader() -> None:
    with oradmp.open(fixture("18c/t_num_exp_18c.dmp"), **LENIENT) as dump:  # type: ignore[arg-type]
        reader = dump.read_table("T_NUM")
        capsule = reader.__arrow_c_stream__()
        assert type(capsule).__name__ == "PyCapsule"
        with pytest.raises(oradmp.UsageError):
            reader.__arrow_c_stream__()
