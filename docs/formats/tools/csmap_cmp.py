#!/usr/bin/env python3
"""Compare Oracle's byte-to-Unicode map of three single-byte character sets with Python's codecs.

Checks (exp.md section 14.11, Analyst session 7, request R-026(b)): truth_csmap.txt holds, for
the sets US7ASCII, WE8MSWIN1252 and CL8MSWIN1251 and every byte 0x00 to 0xFF, the DUMP(...,16) of
UTL_I18N.RAW_TO_NCHAR(byte, set), i.e. the AL16UTF16 (UTF-16 BE) code units Oracle produces. The
tool decodes those code units and compares each byte with
  - Python `cp1251` / `cp1252` (strict; an undefined byte is reported as "undefined"),
  - the WHATWG reading of windows-1252 / windows-1251 as Python can express it: the strict codec,
    and for the bytes Python leaves undefined the C1 control U+0080 + (byte - 0x80) (this is how
    the WHATWG index fills them; stated here as the assumption of the comparison),
  - for US7ASCII: the byte itself for 0x00..0x7F; for 0x80..0xFF every distinct answer is listed.
It prints every byte where Oracle and the codec differ, and per set the number of equal bytes.
Expects: a truth_csmap.txt, default corpus/fixtures/18c/truth_csmap.txt:
    uv run csmap_cmp.py ../../../corpus/fixtures/18c/truth_csmap.txt
Reads the truth file, never writes it. New in Analyst session 7.
"""
import collections
import sys


def oracle_map(path: str) -> dict:
    m: dict = {}
    for ln in open(path, encoding="ascii").read().split("\n"):
        if not ln.strip():
            continue
        cs, b, dump = ln.split("|")
        if dump.strip() == "NULL":
            m[(cs, int(b, 16))] = None
            continue
        raw = bytes(int(x, 16) for x in dump.split(": ", 1)[1].split(","))
        m[(cs, int(b, 16))] = raw.decode("utf-16-be")
    return m


def py(cs: str, b: int):
    try:
        return bytes([b]).decode(cs)
    except UnicodeDecodeError:
        return None


def fmt(s) -> str:
    if s is None:
        return "undefined/NULL"
    return " ".join(f"U+{ord(c):04X}" for c in s)


def main() -> None:
    path = sys.argv[1] if len(sys.argv) > 1 else "../../../corpus/fixtures/18c/truth_csmap.txt"
    m = oracle_map(path)
    sets = sorted({cs for cs, _ in m})
    print(f"{path}: {len(m)} lines, sets {sets}")
    for cs, codec in (("CL8MSWIN1251", "cp1251"), ("WE8MSWIN1252", "cp1252")):
        eq = eq_whatwg = 0
        for b in range(256):
            o = m[(cs, b)]
            p = py(codec, b)
            w = p if p is not None else chr(b)          # WHATWG: undefined bytes -> C1 control U+0080..U+009F
            eq += o == p
            eq_whatwg += o == w
            if o != p or o != w:
                print(f"  {cs} byte {b:02X}: oracle {fmt(o)}  python {codec} {fmt(p)}  whatwg {fmt(w)}")
        print(f"{cs}: equal to Python {codec} in {eq}/256 bytes, to the WHATWG reading in {eq_whatwg}/256")
    cs = "US7ASCII"
    low = sum(m[(cs, b)] == chr(b) for b in range(128))
    hi = collections.Counter(fmt(m[(cs, b)]) for b in range(128, 256))
    same_as_low7 = sum(m[(cs, b)] == chr(b & 0x7F) for b in range(128, 256))
    print(f"US7ASCII: bytes 00..7F equal to the byte itself: {low}/128")
    print(f"US7ASCII: bytes 80..FF, answers: {dict(hi)}; equal to (byte & 0x7F): {same_as_low7}/128")
    for b in (0x80, 0x81, 0xBF, 0xC0, 0xFF):
        print(f"  US7ASCII byte {b:02X}: oracle {fmt(m[(cs, b)])}")


if __name__ == "__main__":
    main()
