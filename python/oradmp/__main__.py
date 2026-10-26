"""``python -m oradmp COMMAND ...``: the same tool as the ``oradmp`` console script (ADR-72)."""

from __future__ import annotations

import sys

from ._cli import main

if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
