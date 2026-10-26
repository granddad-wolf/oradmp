"""``python -m oradmp survey FILE [--no-names] [--no-read]`` (architecture 5.2b, 8.5).

Prints counts about a dump: the header, the tables with their parts and columns, null counts and length figures,
the NUMBER profile of every NUMBER column (per candidate type), the LONG and LOB counts, the warnings and the
errors with offsets. It prints **no value**, no minimum and no maximum *value*, and no byte of the file; with
``--no-names`` also no table, column, partition or user name, so that the output can be pasted back.
"""

from __future__ import annotations

import argparse
import os
import sys
from typing import Any, TextIO

from . import _api
from ._errors import OradmpError


def path_text(path: str) -> str:
    """A path in a message (ADR-82 item 3): each byte that is not valid UTF-8 written ``\\xHH``; tab, CR and
    newline written ``\\t``, ``\\r``, ``\\n``; nothing else changed. Never a lone surrogate."""
    text = os.fsencode(path).decode("utf-8", "backslashreplace")
    return text.replace("\t", "\\t").replace("\r", "\\r").replace("\n", "\\n")


class _Names:
    """Shows a name, or a stand-in when names are hidden."""

    def __init__(self, show: bool) -> None:
        self.show = show

    def table(self, ordinal: int, name: str) -> str:
        return f"table {ordinal} {name}" if self.show else f"table {ordinal}"

    def column(self, position: int, name: str) -> str:
        return f"{name}" if self.show else f"C{position}"

    def part(self, index: int, partition: str | None, subpartition: str | None) -> str:
        if not self.show:
            return f"part {index}"
        names = [n for n in (partition, subpartition) if n is not None]
        return f"part {index} ({'/'.join(names)})" if names else f"part {index}"


def _mean(total: int, values: int) -> str:
    return f"{total / values:.1f}" if values else "-"


def _fmt(v: object) -> str:
    return "-" if v is None else str(v)


def _error_line(e: BaseException) -> str:
    """The error as text: code, offset and numbers (the message of an error holds no value)."""
    return str(e)


def write_survey(path: str, out: TextIO, *, names: bool = True, read: bool = True) -> int:
    """Writes the survey of ``path`` to ``out``; returns 0, or 1 if the file could not be opened or scanned."""
    n = _Names(names)
    p = out.write
    try:
        dump = _api.open(path)
        cat = dump.scan()
    except OradmpError as e:
        p(f"oradmp survey\nERROR: {_error_line(e)}\n")
        return 1
    except OSError as e:
        p(f"oradmp survey\nERROR: {_error_line(e)}\n")
        return 1
    info = dump.info
    p("oradmp survey (counts only: no value, no byte of the file)\n")
    p(f"file: {path_text(os.path.basename(path)) if names else '(hidden)'}  size: {info.file_size} bytes\n")
    p(f"format: {info.format}  version line: {info.version}  mode: {info.mode}  record length: {info.record_length}\n")
    p(f"user: {info.user if names else '(hidden)'}  direct-path flag: {info.direct_flag}\n")
    p(
        f"character sets: client {info.client_charset_id} ({_fmt(info.client_charset)}), "
        f"database {info.db_charset_id} ({_fmt(info.db_charset)}), "
        f"national {info.national_charset_id} ({_fmt(info.national_charset)})"
        f"{'; names converted at export (may hold ?)' if info.text_converted_at_export else ''}\n"
    )
    p(f"tables with data: {len(cat.tables)}  scan: {cat.end}")
    if cat.stop_error is not None:
        p(f"  stopped by: {_error_line(cat.stop_error)}")
        if cat.stopped_table is not None:
            p(f"  in {n.table(cat.stopped_table.ordinal, cat.stopped_table.name)}")
    p("\n")
    p(f"lines: {cat.item_counts.lines}  fields after an unknown introducing line (G1): {cat.item_counts.g1_fields}\n")
    ws = dump.warnings
    p(f"warnings of the dump: {len(ws)}\n")
    for w in ws:
        p(f"  {w.text}\n")

    data = dump._survey(read)
    by_ordinal: dict[int, dict[str, Any]] = {t["ordinal"]: t for t in data["tables"]}
    for t in cat.tables:
        entry = by_ordinal.get(t.ordinal, {})
        counts = entry.get("counts")
        p(f"\n== {n.table(t.ordinal, t.name)}\n")
        flags = []
        if t.possibly_incomplete:
            flags.append("possibly incomplete (no rows and a damaged name)")
        if t.ddl_fallback:
            flags.append("column list not understood: mapped from the descriptors")
        if t.name_possibly_damaged:
            flags.append("name possibly damaged")
        p(f"rows: {t.num_rows}  columns: {len(t.columns)}  path: {t.path}  parts: {len(t.parts)}")
        p(f"  {'; '.join(flags)}\n" if flags else "\n")
        if t.partitioned:
            for i, part in enumerate(t.parts, 1):
                p(f"  {n.part(i, part.partition, part.subpartition)}: rows {part.num_rows}, {part.path}\n")
        cc = {c["position"]: c for c in counts["columns"]} if counts else {}
        for c in t.columns:
            k = cc.get(c.position)
            label = n.column(c.position, c.name)
            decl = c.declared if (c.declared_known or names) and c.declared else "?"
            p(f"  {c.position:>3} {label}: type {c.type_code} {c.oracle_type}, declared {decl}, arrow {c.arrow_type}")
            if c.name_possibly_damaged and names:
                p(" (name possibly damaged)")
            p("\n")
            if k is None:
                continue
            p(
                f"      values {k['values']}, nulls {k['nulls']}, bytes min/mean/max "
                f"{_fmt(k['len_min'])}/{_mean(k['len_total'], k['values'])}/{_fmt(k['len_max'])}\n"
            )
            if c.long is not None:
                lg = c.long
                p(
                    f"      LONG: values {lg.values}, chained {lg.chained}, chunked {lg.chunked}, "
                    f"largest {lg.largest}, above 32,000 bytes {lg.above_32000}, zero-length pieces {lg.zero_pieces}, "
                    f"65,521 to 65,534 bytes {lg.sizes_65521_to_65534}\n"
                )
            if c.lob is not None:
                lb = c.lob
                p(
                    f"      LOB: values {lb.values}, short {lb.short}, empty {lb.empty}, long {lb.long}, "
                    f"largest stored {lb.largest} bytes; would raise E-LOB-UNCONFIRMED: "
                    f"short {k['lob_refused_short']}, long {k['lob_refused_long']}\n"
                )
            if k["years_below_1"] or k["months_beyond_32_bits"]:
                p(
                    f"      values the Arrow type cannot hold: year below 1: {k['years_below_1']}, "
                    f"interval months beyond 32 bits: {k['months_beyond_32_bits']}\n"
                )
            if k["no_decimal128_holds"]:
                p("      declared NUMBER that no Decimal128 holds: number_default applies (W-DDL-FALLBACK)\n")
            num = k["number"]
            if num is not None:
                _number(p, num, k["values"])
        if entry.get("count_error") is not None:
            p(f"  counting failed: {_error_line(entry['count_error'])}\n")
        r = entry.get("read")
        if r is not None:
            p(f"  lenient read (on_value_error=null): rows {r['rows_read']}, warnings {len(r['warnings'])}\n")
            for w in r["warnings"]:
                p(f"    {w['text']}\n")
            if r["error"] is not None:
                p(f"    read stopped: {_error_line(r['error'])}\n")

    summary = data["number_summary"]
    if summary:
        p("\n== NUMBER columns, all tables\n")
        for s in summary:
            p(
                f"  {s['label']}: columns {s['columns']}, exact {s['exact']}, changed by round {s['rounded']} "
                f"(to 0: {s['rounded_to_zero']}), integer part too large {s['int_too_large']}; "
                f"columns that would need an override: {s['columns_needing_override']}\n"
            )
    dump.close()
    return 0


def _number(p: Any, num: dict[str, Any], values: int) -> None:
    p(
        f"      NUMBER: integers {num['integers']}, fit Int64 {num['fit_int64']}, "
        f"39-40 significant digits {num['sig_39_or_40']}, invalid {num['invalid']}\n"
    )
    p(
        f"      digits: largest sig {num['max_sig']}, frac {num['max_frac']}, int {num['max_int']}; "
        f"exp10 {_fmt(num['min_exp10'])}..{_fmt(num['max_exp10'])}; "
        f"leading digit position {_fmt(num['min_lead'])}..{_fmt(num['max_lead'])}\n"
    )
    for k in num["candidates"]:
        p(
            f"      as {k['label']}: exact {k['exact']}, changed by round {k['rounded']} "
            f"(to 0: {k['rounded_to_zero']}), integer part too large {k['int_too_large']}\n"
        )


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(prog="python -m oradmp")
    sub = parser.add_subparsers(dest="command", required=True)
    s = sub.add_parser("survey", help="print counts about a dump (never a value)")
    s.add_argument("file")
    s.add_argument("--no-names", action="store_true", help="hide table, column, partition and user names")
    s.add_argument("--no-read", action="store_true", help="count only; do not read every table leniently")
    args = parser.parse_args(argv)
    return write_survey(args.file, sys.stdout, names=not args.no_names, read=not args.no_read)
