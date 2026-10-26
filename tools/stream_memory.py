"""Task T-15 (e): does RSS grow when a table is read many times through the iterator path and through the Arrow C
stream path (review 03's unreached check, ADR-55)? Not shipped.

    python tools/stream_memory.py [FIXTURES_DIR]

Run it with the python of a venv in which a release wheel is installed (see the T-15 note). It reads
``18c/part_exp_18c.dmp`` table T_PART 500 times through ``BatchReader`` iteration and 500 times through
``pyarrow.RecordBatchReader.from_stream(reader).read_all()``, drops each table, prints ``VmRSS`` (from
``/proc/self/status``) after reads 50, 250 and 500 per path, and ``PASS`` when RSS at 500 is at most RSS at 50 plus
10 MiB for both paths, else ``FAIL`` (exit status 1). The fixture is opened read-only.
"""

from __future__ import annotations

import gc
import sys
import warnings
from collections.abc import Callable
from pathlib import Path

import oradmp
import pyarrow as pa

READS = 500
MARKS = (50, 250, 500)
LIMIT_KIB = 10 * 1024


def rss_kib() -> int:
    for line in Path("/proc/self/status").read_text().splitlines():
        if line.startswith("VmRSS:"):
            return int(line.split()[1])
    raise RuntimeError("no VmRSS in /proc/self/status")


def via_iterator(path: str) -> int:
    with oradmp.open(path) as dump, dump.read_table("T_PART") as reader:
        return int(sum(batch.num_rows for batch in reader))


def via_stream(path: str) -> int:
    with oradmp.open(path) as dump, dump.read_table("T_PART") as reader:
        table = pa.RecordBatchReader.from_stream(reader).read_all()
        rows = int(table.num_rows)
        del table
        return rows


def run(name: str, read: Callable[[str], int], path: str) -> tuple[int, int]:
    marks: dict[int, int] = {}
    rows = 0
    for i in range(1, READS + 1):
        rows = read(path)
        if i in MARKS:
            gc.collect()
            marks[i] = rss_kib()
            print(f"{name}: after read {i:3d} ({rows} rows) VmRSS {marks[i]} kB")
    return marks[MARKS[0]], marks[MARKS[-1]]


def main() -> int:
    fixtures = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent / "corpus/fixtures"
    path = str(fixtures / "18c/part_exp_18c.dmp")
    warnings.simplefilter("ignore")
    ok = True
    for name, read in (("iterator", via_iterator), ("stream", via_stream)):
        first, last = run(name, read, path)
        grown = last - first
        print(f"{name}: RSS at 500 minus RSS at 50 = {grown} kB (limit {LIMIT_KIB} kB)")
        ok = ok and grown <= LIMIT_KIB
    print("PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
