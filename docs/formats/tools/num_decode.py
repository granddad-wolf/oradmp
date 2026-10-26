#!/usr/bin/env python3
"""Test the NUMBER decoding procedure of exp.md 6.1 (session 7) on every NUMBER value of the fixtures.

Checks (exp.md section 6.1 "Bytes to a decimal number", rule Y1 of exp_reader_rules.md, request R-024):
1. Text truth: for every line of truth_num.txt, truth_num2.txt (`id|TO_CHAR(v,'TM9')|DUMP(v,16)`) and
   truth_numtypes.txt (`id|n_p10|n_p10s2|n_neg|n_fl|DUMP(n_p10s2)|DUMP(n_neg)`), that
   decode(DUMP bytes) equals the decimal text exactly (Python `decimal`, 60 digits of context), and that
   encode(decimal text) gives back exactly the DUMP bytes (the representation is canonical).
2. The Architect's hypothesis of R-024 (14 byte strings) against truth_num2.txt.
3. Every NUMBER field (type code 2) of every table section of every `*_exp_*.dmp` file (walked with
   exp_parse.py): that decode() accepts it (validity checks below) and that encode(decode(b)) == b. Where
   check_truth.py has a truth for the field (DUMP bytes, an integer, or a decimal text), that decode()
   equals the truth value; the ID columns are compared with the key of the truth row.
Validity checks of decode(): length 1 to 21; the single byte 80 is zero; otherwise the first byte gives
the sign (bit 0x80 set = positive) and the base-100 exponent; positive digit bytes 0x01..0x64, negative
0x02..0x65, a negative mantissa ends with 0x66 unless the value is 21 bytes long; the first and the last
digit are not 0. Anything else raises ValueError (and is printed as PROBLEM).
Expects: fixture directories (default: the three folders of corpus/fixtures):
    uv run num_decode.py ../../../corpus/fixtures/18c ../../../corpus/fixtures/11g \
                         ../../../corpus/fixtures/11g-cp1251
Reads the fixtures, never writes them. New in Analyst session 7. Not product code.

Changed in Analyst session 9: truth_num3.txt (R-032, computed quotients and an average) is part of the text
truth; for each of its lines the number of digit bytes, the first digit byte and the number of significant
decimal digits are printed.

Changed in Analyst session 10 (note A-24, Implementer N-14): step 3 stopped with `KeyError: 0` because
check_truth.expected() returns, since session 9, the expectation of the LONG tables (T_LONG, T_LONG2) as a dict
keyed by column name (the direct path moves the LONG column last, rule T3); the first field hit was the ID of
T_LONG2 row 1, long2_exp_11g.dmp@0x226d. Step 3 now maps such a dict to the `INSERT INTO` column order, as
check_truth.py does, and passes the subpartition name to expected(). No rule is affected (a tool defect).
"""
import glob
import os
import re
import sys
from decimal import Decimal, getcontext

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from check_truth import dump_bytes, expected  # noqa: E402
from exp_parse import parse  # noqa: E402

getcontext().prec = 60


def decode(b: bytes) -> Decimal:
    if not 1 <= len(b) <= 21:
        raise ValueError(f"length {len(b)}")
    if b == b"\x80":
        return Decimal(0)
    if len(b) == 1:
        raise ValueError(f"single byte {b.hex()} is not zero")
    if b[0] & 0x80:                                   # positive
        e = b[0] - 0xC1
        m = b[1:]
        if not all(0x01 <= x <= 0x64 for x in m):
            raise ValueError("positive digit byte out of 01..64")
        digits = [x - 1 for x in m]
        sign = 1
    else:                                             # negative
        e = 0x3E - b[0]
        m = b[1:]
        if m[-1] == 0x66:
            m = m[:-1]
        elif len(b) != 21:
            raise ValueError("negative number without closing 66 and shorter than 21 bytes")
        if not m or not all(0x02 <= x <= 0x65 for x in m):
            raise ValueError("negative digit byte out of 02..65")
        digits = [101 - x for x in m]
        sign = -1
    if digits[0] == 0 or digits[-1] == 0:
        raise ValueError("leading or trailing base-100 digit 0")
    v = Decimal(0)
    for i, dg in enumerate(digits):
        v += Decimal(dg) * (Decimal(100) ** (e - i))
    return v * sign


def encode(v: Decimal) -> bytes:
    """Inverse of decode, used only to show that the bytes are canonical."""
    if v == 0:
        return b"\x80"
    neg = v < 0
    v = abs(v)
    # base-100 exponent e: 100^e <= v < 100^(e+1)
    e = 0
    while v >= Decimal(100) ** (e + 1):
        e += 1
    while v < Decimal(100) ** e:
        e -= 1
    digits = []
    r = v / (Decimal(100) ** e)
    while r != 0 and len(digits) < 21:
        dg = int(r)
        digits.append(dg)
        r = (r - dg) * 100
    if r != 0:
        raise ValueError("more than 20 base-100 digits")
    while digits and digits[-1] == 0:
        digits.pop()
    if neg:
        out = bytes([0x3E - e]) + bytes(101 - d for d in digits)
        return out + (b"\x66" if len(out) < 21 else b"")
    return bytes([0xC1 + e]) + bytes(d + 1 for d in digits)


ARCH = {1: "c4 02", 2: "3b 64 66", 3: "bf 0b", 4: "40 5b 66", 5: "c1 64", 6: "3e 02 66", 7: "3d 64 66",
        8: "c9" + " 64" * 9, 9: "ca 02", 10: "ca 0a 17 22 49 04 45 37 4e 3b 08",
        11: "ca 0a 17 22 49 04 45 37 4e 3b 09", 12: "35 5c 4f 44 1d 62 21 2f 18 2b 5d 66", 13: "80 02",
        14: "bb 0b"}


def main() -> None:
    here = os.path.dirname(os.path.abspath(__file__))
    dirs = sys.argv[1:] or [os.path.join(here, "../../../corpus/fixtures", v) for v in ("18c", "11g", "11g-cp1251")]
    problems = []
    # ---- 1. text truth ------------------------------------------------------------------------
    n_text = 0
    stats = {"maxlen": 0, "emin": 999, "emax": -999, "maxdigits": 0}
    for d in dirs:
        for name, cols in (("truth_num.txt", [(1, 2)]), ("truth_num2.txt", [(1, 2)]),
                           ("truth_num3.txt", [(1, 2)]),          # session 9 (R-032)
                           ("truth_numtypes.txt", [(2, 5), (3, 6)])):
            p = os.path.join(d, name)
            if not os.path.exists(p):
                continue
            for ln in open(p, encoding="utf-8").read().split("\n"):
                if not ln:
                    continue
                f = ln.split("|")
                for ti, di in cols:
                    b = dump_bytes(f[di])
                    if b is None:
                        continue
                    n_text += 1
                    want = Decimal(f[ti])
                    try:
                        got = decode(b)
                    except ValueError as ex:
                        problems.append(f"{name} id {f[0]}: {ex}")
                        continue
                    enc = encode(want)
                    ok = got == want and enc == b
                    sig = len(format(abs(got).normalize(), "e").split("e")[0].replace(".", "").lstrip("0"))
                    stats["maxlen"] = max(stats["maxlen"], len(b))
                    stats["maxdigits"] = max(stats["maxdigits"], sig)
                    if b != b"\x80":
                        e = b[0] - 0xC1 if b[0] & 0x80 else 0x3E - b[0]
                        stats["emin"], stats["emax"] = min(stats["emin"], e), max(stats["emax"], e)
                    if name == "truth_num3.txt":
                        print(f"   {os.path.basename(d)} {name} id {f[0]}: {len(b)} bytes, "
                              f"{len(b) - 1 - (b[-1] == 0x66 and not b[0] & 0x80)} digit bytes, first digit byte "
                              f"{b[1]:02x}, {sig} significant decimal digits, decode == TM9 text: {got == want}")
                    if not ok:
                        problems.append(f"{d}/{name} id {f[0]}: bytes {b.hex(' ')} decode {got} truth {want} "
                                        f"encode {enc.hex(' ')}")
    print(f"1. text truth: {n_text} values compared (decode and encode); "
          f"longest {stats['maxlen']} bytes, most significant digits {stats['maxdigits']}, "
          f"base-100 exponent {stats['emin']} .. {stats['emax']}")
    # ---- 2. the Architect's hypothesis --------------------------------------------------------
    for d in dirs:
        p = os.path.join(d, "truth_num2.txt")
        if not os.path.exists(p):
            continue
        res = []
        for ln in open(p, encoding="utf-8").read().split("\n"):
            if ln:
                f = ln.split("|")
                res.append((int(f[0]), dump_bytes(f[2]) == bytes.fromhex(ARCH[int(f[0])].replace(" ", ""))))
        print(f"2. {os.path.basename(d)} truth_num2.txt: hypothesis confirmed for ids "
              f"{[i for i, ok in res if ok]}, refuted for {[i for i, ok in res if not ok]}")
    # ---- 3. every NUMBER field of the fixtures ------------------------------------------------
    files = sorted(f for d in dirs for f in glob.glob(os.path.join(d, "*_exp_*.dmp")))
    n_all = n_truth = 0
    for f in files:
        r = parse(f)
        tdir = os.path.dirname(os.path.abspath(f))
        for t in r["tables"]:
            numcols = [i for i, c in enumerate(t["cols"]) if c["type"] == 2]
            if not numcols:
                continue
            keyf, exp, _note = expected(t["name"], tdir, t.get("partition"), t.get("subpartition"))
            names = re.findall(r'"([^"]*)"', t["insert"].split(") VALUES (")[0].split(" (", 1)[1]) if "insert" in t else []
            for _ro, row in t["rows"]:
                vals = [v for _o, v in row]
                want = None
                if exp is not None and keyf is not None:
                    want = exp.get(keyf(vals))
                if isinstance(want, dict):       # session 10 (A-24): expectation by column name (check_truth, session 9)
                    want = [want[n] for n in names]
                for ci in numcols:
                    fo, b = row[ci]
                    if b is None:
                        continue
                    n_all += 1
                    try:
                        v = decode(b)
                    except ValueError as ex:
                        problems.append(f"{os.path.basename(f)}@0x{fo:x}: {ex}")
                        continue
                    if encode(v) != b:
                        problems.append(f"{os.path.basename(f)}@0x{fo:x}: encode(decode) {encode(v).hex(' ')} != {b.hex(' ')}")
                    if want is None:
                        continue
                    w = want[ci]
                    if ci == 0 and w is ...:
                        k = keyf(vals)
                        if isinstance(k, int):
                            n_truth += 1
                            if v != k:
                                problems.append(f"{os.path.basename(f)}@0x{fo:x}: id {v} != key {k}")
                        continue
                    if w is ... or w is None:
                        continue
                    n_truth += 1
                    if isinstance(w, bytes):
                        good = w == b
                    elif isinstance(w, tuple) and w[0] == "int":
                        good = v == w[1]
                    elif isinstance(w, tuple) and w[0] == "num":
                        good = v == Decimal(w[1])
                    else:
                        good = False
                    if not good:
                        problems.append(f"{os.path.basename(f)}@0x{fo:x}: {b.hex(' ')} = {v}, truth {w}")
    print(f"3. NUMBER fields in {len(files)} files: {n_all} decoded and re-encoded; {n_truth} compared with a "
          f"truth or SQL value")
    print(f"PROBLEMS: {len(problems)}")
    for x in problems:
        print("  " + x)


if __name__ == "__main__":
    main()
