#!/usr/bin/env python3
"""Print the header fields of several legacy EXP fixtures side by side.

Checks: which header bytes change when the client character set (NLS_LANG), the RECORDLENGTH,
the export path or the file name changes. One line per fixture with: bytes 1-2, the three u16 BE
values after the text lines, the two u16 BE pairs at +0x3c of the two 102-byte blocks, the
record length text, the file-name length field, the offset where the date text starts and ends,
the file size, and the offset of the last non-zero byte. Then it asserts the invariants that
docs/formats/exp.md section 2 states (file name at 0x80, date ends at 0x7f, header record is
zero-filled up to RECORDLENGTH, file size is a multiple of RECORDLENGTH, both blocks and the
string list are byte-identical in all files apart from the two u16 at block 2 +0x3c).
Expects: any number of `*_exp_*.dmp` fixtures or fixture directories, e.g.
    uv run header_fields.py ../../../corpus/fixtures/18c ../../../corpus/fixtures/11g-cp1251
Uses exp_parse.py from the same directory. Exploration tool only; never writes the fixtures.

Changed in Analyst session 4:
- accepts directories (all `*_exp_*.dmp` in them); works on 11g and 11g-cp1251;
- prints the version, user and mode lines, the first item at RECORDLENGTH + 0x104 and whether the
  second record starts with the block signature `64 00 00 00` at the offset the header names;
- the reference for "blocks and string list are identical" is the first file of each directory,
  and a difference is printed byte by byte (block offset, reference byte, this file's byte)
  instead of only being counted. The directories are compared with each other at the end;
- block 1 +0x3c / +0x3e are compared with header csB / csC (invariant added: equal).
"""
import glob
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402


def blockdiff(a: bytes, b: bytes) -> str:
    return ", ".join(f"+0x{i:02x} {a[i]:02x}->{b[i]:02x}" for i in range(len(a)) if a[i] != b[i]) or "same"


def main() -> None:
    paths = []
    for a in sys.argv[1:]:
        paths += sorted(glob.glob(os.path.join(a, "*_exp_*.dmp"))) if os.path.isdir(a) else [a]
    ref_b1 = ref_b2 = ref_str = None
    refs: dict = {}
    cur_dir = None
    bad = []
    print(f"{'file':38s} {'size':>6s} {'reclen':>6s} b1-2  csA@off     csB  csC  blk1+3c   blk2+3c   "
          f"fnlen@off  date@   last-nonzero  version/user/mode  first item")
    for path in paths:
        name = os.path.basename(path)
        if os.path.dirname(path) != cur_dir:
            cur_dir = os.path.dirname(path)
            ref_b1 = ref_b2 = ref_str = None
        d = open(path, "rb").read()
        r = parse(path)
        h = r["header"]
        b1o, b1 = r["block1"]
        b2o, b2 = r["block2"]
        last = max(i for i in range(len(d)) if d[i])
        print(f"{name:38s} {len(d):6d} {h['reclen']:6d} {h['cs_at_1']:4d}  {h['cs_a']:4d}@0x{h['bin_off']:02x}  "
              f"{h['cs_b']:4d} {h['cs_c']:4d}  {b1[0x3c:0x40].hex()}  {b2[0x3c:0x40].hex()}  "
              f"{h['fnlen']:3d}@0x{h['fnlen_off']:02x}  0x{h['date'][0]:02x}    0x{last:04x}  "
              f"{h['version']} {h['user']} {h['mode']}  {r['items'][0]['text']!r}@R+0x{r['first_item_off'] - h['reclen']:x}")
        if b1[:4] != bytes.fromhex("64000000"):
            bad.append(f"{name}: the second record at 0x{b1o:x} does not start with 64 00 00 00")
        if r["first_item_off"] != h["reclen"] + 0x104:
            bad.append(f"{name}: first item is not at R + 0x104")
        if (int.from_bytes(b1[0x3c:0x3e], "big"), int.from_bytes(b1[0x3e:0x40], "big")) != (h["cs_b"], h["cs_c"]):
            bad.append(f"{name}: block 1 +0x3c pair is not (csB, csC)")
        if h["fname"][1] != "/fixtures/" + name:
            bad.append(f"{name}: file name text at 0x80 is {h['fname'][1]!r}")
        if h["date"][0] + len(h["date"][1]) != 0x80:
            bad.append(f"{name}: date does not end at 0x7f")
        if len(d) % h["reclen"]:
            bad.append(f"{name}: size is not a multiple of reclen")
        if (last // h["reclen"] + 1) * h["reclen"] != len(d):
            bad.append(f"{name}: size is not the smallest multiple of reclen that holds the data")
        if h["cs_at_1"] != h["cs_a"]:
            bad.append(f"{name}: bytes 1-2 differ from csA")
        k1 = b1
        k2 = b2[:0x3c] + b2[0x40:]
        ks = [s for _o, s in r["strings"]]
        if ref_b1 is None:
            ref_b1, ref_b2, ref_str = k1, k2, ks
            refs[cur_dir] = (name, b1, b2, ks)
        if (k1, k2, ks) != (ref_b1, ref_b2, ref_str):
            bad.append(f"{name}: blocks or string list differ from the first file of its directory: "
                       f"block1 {blockdiff(ref_b1, k1)}; block2 (without +0x3c..0x3f) {blockdiff(ref_b2, k2)}; "
                       f"strings {ks}")
        if b2[0x3c:0x3e] != b2[0x3e:0x40] or int.from_bytes(b2[0x3c:0x3e], "big") != h["cs_a"]:
            bad.append(f"{name}: block 2 +0x3c pair is not (csA, csA)")
        if r["problems"]:
            bad.append(f"{name}: {r['problems']}")
    print("invariant violations:", bad or "none")
    keys = list(refs)
    for k in keys[1:]:
        n0, a1, a2, s0 = refs[keys[0]]
        n1, c1, c2, s1 = refs[k]
        print(f"{n0} -> {n1}: block1 {blockdiff(a1, c1)}; block2 {blockdiff(a2, c2)}; "
              f"strings {'same' if s0 == s1 else str(s0) + ' -> ' + str(s1)}")


if __name__ == "__main__":
    main()
