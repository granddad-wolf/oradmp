"""Task T-13h (e), ADR-59: a panic in the core becomes ``E-INTERNAL`` (``oradmp.InternalError``) at the binding.

The probes exist only in a build with the cargo feature ``test-probes`` (``maturin develop --features test-probes``);
the tests skip, with that reason, on a release build."""

from __future__ import annotations

import subprocess
import sys
import textwrap
from typing import Any

import oradmp
import pytest
from conftest import fixture
from oradmp import _core

pytestmark = pytest.mark.skipif(
    not hasattr(_core, "_probe_panic"), reason="_core has no _probe_panic: built without --features test-probes"
)

# The probes are not in the stub `_core.pyi` (ADR-65 item 7): they are reached with `getattr`.
PROBE = "_probe_panic"
probe_panic: Any = getattr(_core, PROBE, None)

MESSAGE = "[E-INTERNAL] internal error: a panic in oradmp was caught; please report it with the oradmp version"


def check(e: BaseException) -> None:
    assert isinstance(e, oradmp.InternalError)
    assert isinstance(e, oradmp.OradmpError)
    assert isinstance(e, Exception)  # `except Exception` and `except OradmpError` both catch it
    assert e.code == "E-INTERNAL"
    assert (e.offset, e.table, e.column, e.row) == (None, None, None, None)


def test_a_panic_through_the_dump_guard_is_an_internal_error_and_the_dump_is_unusable_after() -> None:
    d = _core.open(fixture("18c/t_num_exp_18c.dmp"), {})
    with pytest.raises(oradmp.InternalError) as ei:
        getattr(d, PROBE)()
    check(ei.value)
    assert str(ei.value) == MESSAGE  # no payload ("probe"), no location
    with pytest.raises(oradmp.InternalError) as again:  # the next call: E-INTERNAL again
        d.info()
    check(again.value)
    assert "earlier call" in str(again.value)
    with pytest.raises(oradmp.InternalError):
        d.scan()


def test_a_panic_through_the_batch_path_is_an_internal_error_and_the_reader_is_unusable_after() -> None:
    r = probe_panic("reader")
    with pytest.raises(oradmp.InternalError) as ei:
        r.next_batch()
    check(ei.value)
    assert str(ei.value) == MESSAGE
    with pytest.raises(oradmp.InternalError) as again:
        r.next_batch()
    check(again.value)
    with pytest.raises(oradmp.InternalError):
        r.arrow_c_stream()


def test_a_panic_inside_the_arrow_stream_is_an_error_text_and_never_an_abort() -> None:
    # in a subprocess: an unwinding panic across the C boundary would abort the process (exit status != 0)
    code = textwrap.dedent(
        """
        import sys
        import pyarrow as pa
        from oradmp import _core

        class Source:
            def __init__(self):
                self.r = _core._probe_panic("stream")
            def __arrow_c_stream__(self, requested_schema=None):
                return self.r.arrow_c_stream(requested_schema)

        src = Source()
        stream = pa.RecordBatchReader.from_stream(src)
        try:
            stream.read_all()
        except Exception as e:
            print("TEXT:" + str(e).removeprefix("External error: "))
        else:
            sys.exit(3)
        """
    )
    p = subprocess.run([sys.executable, "-c", code], capture_output=True, text=True, timeout=120, check=False)
    assert p.returncode == 0, (p.returncode, p.stderr[-2000:])
    lines = p.stdout.splitlines()
    assert lines[0] == "TEXT:" + MESSAGE, lines
    assert lines[0].startswith("TEXT:[E-INTERNAL")
