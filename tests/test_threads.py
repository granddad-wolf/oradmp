"""Task T-12: the GIL is released around ``open``, the scan and every batch. A Python thread that only counts must
make progress while another thread is inside a native read call. The test does not measure speed and does not need
a second CPU: it only asks in how many native calls the counter thread ran at all."""

from __future__ import annotations

import sys
import threading
import time

import oradmp
from conftest import fixture

PATH = "18c/part_exp_18c.dmp"  # T_PART: 3,000 rows in three parts
CALLS = 60  # native calls that return a batch
# Measured on the VM (debug build, also pinned to one CPU with taskset): the counter ran during every such call with
# the GIL released; with the GIL held through the native code (mutation) it still moved in up to 32 percent of the
# calls. The cause (perhaps pyarrow releasing the GIL during the batch import) is not confirmed. Half is far from both.
MIN_SHARE = 0.5


def test_counter_thread_progresses_during_read() -> None:
    ticks = 0
    stop = threading.Event()

    def counter() -> None:
        nonlocal ticks
        while not stop.is_set():
            ticks += 1
            time.sleep(0)  # give the GIL back at once

    old_interval = sys.getswitchinterval()
    # A huge switch interval means Python itself never takes the GIL from the reading thread: the counter can run
    # only when the extension releases the GIL (or the reading thread blocks).
    sys.setswitchinterval(1000.0)
    thread = threading.Thread(target=counter)
    calls = 0
    progressed = 0
    try:
        with oradmp.open(fixture(PATH)) as dump:
            thread.start()
            while ticks == 0:  # the counter is running before the first measurement
                time.sleep(0.001)
            while calls < CALLS:
                reader = dump.read_table("T_PART")
                while calls < CALLS:
                    before = ticks
                    # one native call (private handle on purpose): no pyarrow or other Python code in between
                    batch = reader._core.next_batch()
                    if batch is None:
                        break
                    calls += 1
                    progressed += ticks != before
    finally:
        stop.set()
        sys.setswitchinterval(old_interval)
        thread.join()
    assert progressed >= CALLS * MIN_SHARE, (
        f"the counter thread ran during only {progressed} of {CALLS} read calls: is the GIL held?"
    )
