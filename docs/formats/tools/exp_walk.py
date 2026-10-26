#!/usr/bin/env python3
"""Walk a legacy EXP table-mode dump and print every region with its offsets.

Checks: the region layout described in docs/formats/exp.md (session 1), by walking the file
from offset 0 and printing each piece with start/end offsets. Decodes the row data with the
rule "u16 LE length + bytes, 0xFFFE = NULL, 0xFFFF = end of data; conventional path has a
00 00 after every row, direct path has none" and, if a truth file is given, compares the rows.
Every byte that the walk cannot name is printed as UNKNOWN; nothing is skipped silently.
Expects: t_frame_exp_18c.dmp or t_frame_exp_direct_18c.dmp (single table, VARCHAR2 columns), e.g.
    uv run exp_walk.py ../../../corpus/fixtures/18c/t_frame_exp_18c.dmp \
                       ../../../corpus/fixtures/18c/truth_frame.txt
It is written against exactly these two fixtures and stops with an assertion on anything else.
Exploration tool only, not the product parser. Reads the fixtures, never writes them.
"""
import sys


def u16le(d: bytes, o: int) -> int:
    return d[o] | (d[o + 1] << 8)


def region(start: int, end: int, what: str, detail: str = "") -> None:
    """end is exclusive; printed inclusive."""
    print(f"0x{start:04x}-0x{end - 1:04x} {end - start:5d}  {what}  {detail}")


def brief(b: bytes, n: int = 32) -> str:
    return b.hex(" ") if len(b) <= n else b[:n].hex(" ") + " ..."


def line(d: bytes, o: int) -> tuple[bytes, int]:
    e = d.index(b"\n", o)
    return d[o:e], e + 1


def main() -> None:
    d = open(sys.argv[1], "rb").read()
    truth = None
    if len(sys.argv) > 2:
        truth = [[None if v == "<NULL>" else v for v in ln.rstrip("\n").split("|")]
                 for ln in open(sys.argv[2], encoding="utf-8")]
    print(f"{sys.argv[1]}: {len(d)} bytes")

    # ---- header -------------------------------------------------------------------------
    o = 0
    region(o, o + 3, "3 binary bytes", brief(d[0:3]))
    o = 3
    direct_marker = d[o:o + 2] == b"D\n"
    if direct_marker:
        region(o, o + 2, "text line", "'D' + LF  (present only in the direct-path file)")
        o += 2
    for name in ("version line", "line starting with U", "line starting with R",
                 "numeric line 1", "numeric line 2", "numeric line 3", "numeric line 4"):
        s, n = line(d, o)
        region(o, n, name, repr(s.decode("ascii")) + " + LF")
        o = n
    region(o, o + 16, "binary", brief(d[o:o + 16]))
    o += 16
    region(o, o + 3, "binary, holds file-name length", brief(d[o:o + 3]))
    name_len_be = (d[o] << 8) | d[o + 1]
    name_len_le = u16le(d, o + 1)
    o += 3
    s = o
    while d[o] == 0x20:
        o += 1
    region(s, o, "spaces (0x20)", f"count={o - s}")
    s = o
    # date text: runs up to the '/' that starts the file name in these fixtures
    o = d.index(b"/", s)
    region(s, o, "export date text", repr(d[s:o].decode("ascii")))
    s = o
    o = d.index(b"\x00", s)
    region(s, o, "dump file name text", repr(d[s:o].decode("ascii")) +
           f"  len={o - s}; the 3 bytes above read {name_len_be} as u16 BE (first 2) and {name_len_le} as u16 LE (last 2)")
    s = o
    while d[o] == 0:
        o += 1
    region(s, o, "zero fill", "")
    assert o == 0x2000, hex(o)

    # ---- second 8192-byte unit ----------------------------------------------------------
    blk = 0x66
    b1, b2 = d[o:o + blk], d[o + blk:o + 2 * blk]
    region(o, o + blk, "binary block 1", brief(b1, 64))
    region(o + blk, o + 2 * blk, "binary block 2", brief(b2, 64))
    dif = [i for i in range(blk) if b1[i] != b2[i]]
    print("        block 1 vs block 2 differ at block offsets "
          + ", ".join(f"+0x{i:02x} ({b1[i]:02x}/{b2[i]:02x})" for i in dif))
    nz1 = max(i for i in range(blk) if b1[i])
    print(f"        last non-zero byte of block 1 at +0x{nz1:02x}; rest of block is zero")
    o += 2 * blk
    # u16-LE-length-prefixed strings until the LF that precedes METRICST
    while d[o] != 0x0A:
        n = u16le(d, o)
        region(o, o + 2 + n, "u16 LE length + text", f"len={n} {d[o + 2:o + 2 + n]!r}")
        o += 2 + n
    region(o, o + 1, "LF", "")
    o += 1
    for name in ("text line", "text line (table)", "text line (DDL)", "text line (INSERT)"):
        s, n = line(d, o)
        txt = s.decode("ascii")
        region(o, n, name, repr(txt if len(txt) < 70 else txt[:67] + "...") + " + LF")
        o = n

    # ---- column descriptors -------------------------------------------------------------
    ncol = u16le(d, o)
    region(o, o + 2, "u16 LE column count", str(ncol))
    o += 2
    for c in range(ncol):
        f = [u16le(d, o + 2 * i) for i in range(4)]
        region(o, o + 8, f"column {c + 1} descriptor", brief(d[o:o + 8]) +
               f"  as 4 x u16 LE: {f}")
        o += 8
    region(o, o + 2, "u16 LE (0 in both fixtures)", brief(d[o:o + 2]))
    o += 2
    flag = u16le(d, o)
    region(o, o + 2, "u16 LE (0 conventional / 1 direct)", brief(d[o:o + 2]))
    o += 2

    # ---- rows ---------------------------------------------------------------------------
    rows = []
    while True:
        if u16le(d, o) == 0xFFFF:
            region(o, o + 2, "end-of-data marker", "ff ff")
            o += 2
            break
        rs = o
        row = []
        parts = []
        for c in range(ncol):
            n = u16le(d, o)
            if n == 0xFFFE:
                row.append(None)
                parts.append(f"@0x{o:04x} fe ff=NULL")
                o += 2
            else:
                assert 0 < n < 0xFFFE, (hex(o), n)
                row.append(d[o + 2:o + 2 + n].decode("utf-8"))
                parts.append(f"@0x{o:04x} len={n} data@0x{o + 2:04x} {row[-1]!r}")
                o += 2 + n
        if flag == 0:
            assert d[o:o + 2] == b"\x00\x00", hex(o)
            parts.append(f"@0x{o:04x} 00 00=row end")
            o += 2
        rows.append(row)
        region(rs, o, f"row {len(rows)}", "; ".join(parts))
    if truth is not None:
        print("        rows equal truth file, same order:", rows == truth)

    # ---- trailer ------------------------------------------------------------------------
    region(o, o + 1, "LF", brief(d[o:o + 1]))
    assert d[o] == 0x0A
    o += 1
    while d[o] != 0:
        s, n = line(d, o)
        region(o, n, "text line", repr(s.decode("ascii")) + " + LF")
        o = n
    s = o
    while o < len(d) and d[o] == 0:
        o += 1
    region(s, o, "zero fill", "")
    assert o == len(d)


if __name__ == "__main__":
    main()
