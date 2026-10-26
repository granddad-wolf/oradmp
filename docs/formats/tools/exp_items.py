#!/usr/bin/env python3
"""Print the item walk of several legacy EXP fixtures side by side as a Markdown table.

Checks (exp.md sections 4.3, 4.5, 13): that files which should hold the same objects (the same
export on the conventional and the direct path, with another client set, or on the other Oracle
version) consist of the same sequence of items (same form: line / plain field / pieces, same
label), and prints one table row per item: the offset in every file, the form, the length (and
the piece lengths), the kind, where the item stands (METRICS section, table section before the
INSERT or after the rows, TABLE ... ENDTABLE group) and the start of its text. Lengths or texts
that differ between the files are marked in the row. The binary part after an `INSERT INTO` line
is printed as one row (column count to the LF after the rows).
Expects: fixtures with the same item sequence, e.g. (written for the R-012 files)
    uv run exp_items.py ../../../corpus/fixtures/18c/own_exp_18c.dmp \
                        ../../../corpus/fixtures/18c/own_exp_direct_18c.dmp
If the sequences differ the tool says where and stops. Reads the fixtures, never writes them.
New in Analyst session 4.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

paths = [a for a in sys.argv[1:] if not a.startswith("-")]
rs = [parse(p) for p in paths]
seqs = [[(i["kind"], i["label"]) for i in r["items"]] for r in rs]
for k, s in enumerate(seqs[1:], 1):
    if s != seqs[0]:
        j = next((j for j, (x, y) in enumerate(zip(seqs[0], s)) if x != y), min(len(s), len(seqs[0])))
        print(f"item sequences differ: {paths[0]} / {paths[k]} at item {j}: "
              f"{seqs[0][j] if j < len(seqs[0]) else None} / {s[j] if j < len(s) else None}")
        sys.exit(1)
for p, r in zip(paths, rs):
    print(f"<!-- {os.path.basename(p)}: {len(r['items'])} items, problems: {r['problems'] or 'none'} -->")
print("| # | " + " | ".join(os.path.basename(p)[:-4] for p in paths) + " | form | length | kind | where | text (start) |")
print("|---|" + "---|" * len(paths) + "---|---|---|---|---|")


def where(r, it):
    for t in r["sections"]:
        if it is t.get("_create") or any(it is x for x in t["pre_items"]):
            return f"table section {t['name']}, before the rows"
        if any(it is x for x in t["post_items"]):
            return f"table section {t['name']}, after the rows"
    if it["label"] == "EXIT" or it["text"] in ("METRICETG0", "METRICEUG0"):
        return "end of the data"
    if it["label"] == "ENDTABLE" and any(t.get("sec_end") == it["off"] for t in r["sections"]):
        return "after the last table section (owner mode)"
    z = "before the first METRICS section" if it["zone"] is None else f"`{it['zone'].strip() or '(no name)'}`"
    if it["label"] == "TABLE" and any(t["table_off"] == it["off"] for t in r["sections"]):
        return "start of a table section"
    if it["group"]:
        return f"group TABLE {it['group']} in {z}"
    return z


def esc(s: str) -> str:
    s = s.encode("unicode_escape").decode("ascii")
    return s.replace("|", "\\|").replace("`", "'")


tabs = [{t["insert_off"]: t for t in r["tables"]} for r in rs]
for n in range(len(rs[0]["items"])):
    its = [r["items"][n] for r in rs]
    it = its[0]
    offs = " | ".join(f"0x{i['off']:04x}" for i in its)
    lens = [len(i["raw"]) for i in its]
    ltxt = str(lens[0]) if len(set(lens)) == 1 else " / ".join(map(str, lens))
    if it["kind"] == "line":
        form = "line"
    elif it["kind"] == "plain":
        form = "field, plain"
    else:
        pl = [[n_ for _o, n_ in i["pieces"]] for i in its]
        p0 = pl[0]
        ptxt = ", ".join(map(str, p0)) if len(p0) <= 12 else \
            ", ".join(map(str, p0[:4])) + f", ... ({len(p0)} pieces incl. the last of 0) ..., " + ", ".join(map(str, p0[-3:]))
        if any(p != p0 for p in pl):
            ptxt += " (piece lengths differ between the files)"
        form = ("field, `fd ff` pieces: " if it["kind"] == "pieces" else "field, pieces without marker: ") + ptxt
    same = all(i["raw"] == it["raw"] for i in its)
    txt = esc(it["text"][:70]) + ("..." if len(it["text"]) > 70 else "")
    print(f"| {n + 1} | {offs} | {form} | {ltxt} | {it['label']} | {where(rs[0], it)} | `{txt}`{'' if same else ' (text differs between the files)'} |")
    if it["off"] in tabs[0]:
        ts = [tb[i["off"]] for tb, i in zip(tabs, its)]
        offs = " | ".join(f"0x{t['bin_off']:04x}" for t in ts)
        desc = []
        for k, t in enumerate(ts):
            em = "none" if t["end_marker"] is None else f"0x{t['end_marker']:04x}"
            desc.append(f"{os.path.basename(paths[k])[:-4]}: flag `{t['post_desc'][2]:02x} 00` @0x{t['post_desc'][0] + 2:04x}, "
                        f"{len(t['rows'])} rows from 0x{t['rows_off']:04x}, `ff ff` {em}, LF 0x{t['rows_end']:04x}")
        print(f"| - | {offs} | binary | {' / '.join(str(t['rows_end'] + 1 - t['bin_off']) for t in ts)} | table data | "
              f"table section {ts[0]['name']} | column count {len(ts[0]['cols'])}, descriptors, `00 00`, path flag, rows. "
              + "; ".join(desc) + " |")
