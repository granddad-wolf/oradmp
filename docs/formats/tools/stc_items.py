#!/usr/bin/env python3
"""Which columns of a table get a statistics item: the R-020 files against truth_stc.txt.

Checks (exp.md section 4.6 "Columns without statistics", Analyst session 6, fixture request R-020):
- per table section, the items after the rows in file order with their offsets: kind letter
  (G = GRANT line, T = `ANALSTATS T?` + plain field with SET_TABLE_STATS, C = `ANALSTATS T?` + field
  in pieces with SET_COLUMN_STATS, M = COMMENT ON + field, I = index statistics, ? = anything else),
  framing, payload length, piece lengths, and for a column item the column named INSIDE the field
  and its 1-based position in the column list of the `INSERT INTO` line;
- the rule "a column has an item iff truth_stc.txt has `COLUMN|table|n|name|stats:d:n` for it"
  (and no item iff `NO STATISTICS`), column by column; `num_distinct` and `num_nulls` of the item
  against the truth line; `num_rows` of the table item against `TAB|table|num_rows|date`; "a table
  has a table item iff the TAB line has a date"; "a table has an `ANALCOMPUTE T?` line iff it has a
  table item"; the keyword of the `ANALCOMPUTE` line;
- whether the column items are in ascending column position, and whether a gap in the positions
  leaves any trace in the file (it prints the bytes between two neighbouring items);
- with two or more files: whether every statistics item (line + field) is byte-identical between
  the files (conventional / direct, 18c / 11g), and prints the ones that differ.
Every mismatch is printed as a line that starts with PROBLEM.
Expects: the R-020 files, each with truth_stc.txt in its own directory:
    uv run stc_items.py ../../../corpus/fixtures/18c/stc_exp_18c.dmp \
        ../../../corpus/fixtures/18c/stc_exp_direct_18c.dmp \
        ../../../corpus/fixtures/11g/stc_exp_11g.dmp ../../../corpus/fixtures/11g/stc_exp_direct_11g.dmp [-v]
-v prints the full text of every statistics field.
Uses exp_parse.py and stat_items.py from the same directory. Exploration tool only. Reads the
fixtures, never writes them. New in Analyst session 6.
Changed in Analyst session 7: the R-023 files `stc_exp_us7ascii_<v>.dmp` and `stc_exp_none_<v>.dmp` are
compared with truth_stc_extra.txt (written immediately before those exports, corpus/sql/10b_stc_truth.sql)
instead of truth_stc.txt; the file name decides.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from stat_items import stat_items  # noqa: E402


def truth_stc(tdir: str, name: str = "truth_stc.txt") -> tuple:
    tab, col = {}, {}
    for ln in open(os.path.join(tdir, name), encoding="ascii").read().split("\n"):
        f = ln.rstrip("\r").split("|")
        if f[0] == "TAB":
            tab[f[1]] = (f[2], f[3])                       # num_rows, last_analyzed (or NULL)
        elif f[0] == "COLUMN":
            col[(f[1], f[3])] = (int(f[2]), f[4])          # column_id, "stats:d:n" or "NO STATISTICS"
    return tab, col


def main() -> None:
    verbose = "-v" in sys.argv
    paths = [a for a in sys.argv[1:] if not a.startswith("-")]
    problems = 0
    per_file = {}
    for p in paths:
        base = os.path.basename(p)
        data = open(p, "rb").read()
        its, r = stat_items(p)
        tname = "truth_stc_extra.txt" if ("_us7ascii_" in base or "_none_" in base) else "truth_stc.txt"
        tab, col = truth_stc(os.path.dirname(os.path.abspath(p)), tname)
        none_mode = "_none_" in base
        if none_mode:
            print(f"   ({base}: statistics=none, every statistics item is expected to be absent)")
        by_off = {e["off"]: e for e in its}
        print(f"== {base}  ({r['header']['version']}, path flag of the first table: "
              f"{'direct' if r['sections'][0]['direct'] else 'conventional'})")
        seen = []
        tot = {"cols": 0, "cols_with_stats": 0, "col_items": 0, "tab_items": 0, "analcompute": 0}
        for s in r["sections"]:
            name = s["name"]
            names = re.findall(r'"([^"]*)"', s["insert"].split(" VALUES ")[0])[1:]
            tot["cols"] += len(names)
            order, t_items, c_items = "", [], []
            print(f"  {name}: {len(names)} columns {names}; rows {len(s['rows'])}; rows end 0x{s['rows_end']:x}; "
                  f"section end 0x{s['sec_end']:x}")
            post = s["post_items"]
            k = 0
            while k < len(post):
                it = post[k]
                e = by_off.get(it["off"])
                nxt = post[k + 1] if k + 1 < len(post) and post[k + 1]["kind"] != "line" else None
                if e is not None and e["field"] is not None:
                    f, d = e["field"], e["dec"]
                    if d["what"] == "table":
                        letter, what = "T", "table: SET_TABLE_STATS " + ",".join(d["args"])
                        t_items.append(e)
                    elif d["what"] == "column":
                        pos = names.index(d["column"]) + 1 if d["column"] in names else None
                        letter = "C"
                        what = (f"column {d['column']} (position {pos}): MINVAL {d['min']} MAXVAL {d['max']} EAVS {d['eavs']} "
                                f"CHVALS {d['chvals'][:20]} NOVALS({d['novals'][:30]}) BKVALS({d['bkvals']}) EPC {d['epc']} "
                                f"distcnt {d['distcnt']} density {d['density']} nullcnt {d['nullcnt']} avgclen {d['avgclen']} "
                                f"flags {d['flags']}")
                        c_items.append((e, pos))
                    elif d["what"] == "index":
                        letter, what = "I", "index"
                    else:
                        letter, what = "?", "UNKNOWN BLOCK"
                        problems += 1
                        print(f"PROBLEM {base} 0x{f['off']:x}: field matches no block")
                    form = "plain" if f["form"] == "plain" else "fd ff pieces (" + ", ".join(map(str, f["pieces"])) + ")"
                    print(f"    {letter} line 0x{e['off']:x} `{e['line']}` field 0x{f['off']:x} {form} {f['len']} bytes, "
                          f"ends 0x{f['end']:x}: {what}")
                    if verbose:
                        print(f"        {f['raw'].decode('latin-1')}")
                    seen.append((name, letter, e["line"], data[e["off"]:f["end"]]))
                    k += 2
                elif it["text"].startswith("GRANT "):
                    letter = "G"
                    print(f"    G line 0x{it['off']:x} `{it['text']}`")
                    k += 1
                elif it["text"].startswith("COMMENT ON ") and nxt is not None:
                    letter = "M"
                    print(f"    M line 0x{it['off']:x} `{it['text']}` field 0x{nxt['off']:x} {len(nxt['raw'])} bytes")
                    k += 2
                else:
                    letter = "?"
                    print(f"    ? 0x{it['off']:x} {it['kind']} `{it['text'][:60]}`")
                    k += 1 + (nxt is not None)
                order += letter
            ac = [e for e in its if e["verb"] == "ANALCOMPUTE" and e["target"] == name]
            for e in ac:
                print(f"    ANALCOMPUTE line 0x{e['off']:x} `{e['line']}` ({e['where']})")
                seen.append((name, "A", e["line"], e["line"].encode("latin-1")))
            print(f"    order after the rows: '{order}'; first item after the rows at 0x{s['rows_end']:x}"
                  f"{' (= section end: no item)' if not post else ''}")
            tot["tab_items"] += len(t_items)
            tot["col_items"] += len(c_items)
            tot["analcompute"] += len(ac)
            # --- statistics=none: no item at all, whatever the truth says
            if none_mode:
                if t_items or c_items or ac:
                    problems += 1
                    print(f"PROBLEM {base} {name}: statistics=none but {len(t_items)} table, {len(c_items)} column "
                          f"items and {len(ac)} ANALCOMPUTE lines")
                print(f"    truth: TAB {'|'.join(tab[name])}; statistics=none: table items {len(t_items)}, "
                      f"column items {len(c_items)}, ANALCOMPUTE lines {len(ac)}")
                continue
            # --- against truth_stc.txt
            rows_t, date_t = tab[name]
            has_tab = date_t != "NULL"
            if has_tab != (len(t_items) == 1) or len(t_items) > 1:
                problems += 1
                print(f"PROBLEM {base} {name}: truth TAB last_analyzed {date_t}, table items {len(t_items)}")
            if t_items and t_items[0]["dec"]["args"][0] != rows_t:
                problems += 1
                print(f"PROBLEM {base} {name}: num_rows {t_items[0]['dec']['args'][0]} truth {rows_t}")
            if (len(ac) == 1) != (len(t_items) == 1):
                problems += 1
                print(f"PROBLEM {base} {name}: ANALCOMPUTE lines {len(ac)}, table items {len(t_items)}")
            if order.replace("G", "").replace("M", "")[:1] not in ("", "T") or "T" in order[order.find("T") + 1:]:
                problems += 1
                print(f"PROBLEM {base} {name}: the table item is not the first statistics item: {order}")
            have = {e["dec"]["column"]: e for e, _pos in c_items}
            verdict = []
            for cn in names:
                cid, st = col[(name, cn)]
                tot["cols_with_stats"] += st != "NO STATISTICS"
                if cid != names.index(cn) + 1:
                    problems += 1
                    print(f"PROBLEM {base} {name}.{cn}: column_id {cid} is not its position in the INSERT line")
                if (st != "NO STATISTICS") != (cn in have):
                    problems += 1
                    print(f"PROBLEM {base} {name}.{cn}: truth '{st}', item {'present' if cn in have else 'absent'}")
                elif cn in have:
                    d = have[cn]["dec"]
                    _x, dist, nulls = st.split(":")
                    ok = d["distcnt"] == dist and d["nullcnt"] == nulls
                    if not ok:
                        problems += 1
                        print(f"PROBLEM {base} {name}.{cn}: item distcnt {d['distcnt']} nullcnt {d['nullcnt']}, truth {st}")
                    verdict.append(f"{cn}=item({d['distcnt']}:{d['nullcnt']})")
                else:
                    verdict.append(f"{cn}=none")
            poss = [pos for _e, pos in c_items]
            if poss != sorted(poss) or None in poss:
                problems += 1
                print(f"PROBLEM {base} {name}: column items not in ascending column position: {poss}")
            print(f"    truth: TAB {rows_t}|{date_t}; columns: {' '.join(verdict)}; item positions {poss}; "
                  f"positions without an item {[i + 1 for i, cn in enumerate(names) if cn not in have]}")
            for (a, _pa), (b, pb) in zip(c_items, c_items[1:]):
                gap = data[a["field"]["end"]:b["off"]]     # the field ends before its LF
                if gap != b"\n":
                    problems += 1
                    print(f"PROBLEM {base} {name}: {len(gap)} bytes between the items before position {pb}: {gap[:20].hex()}")
        print(f"  totals: {tot}")
        per_file[base] = seen
    if len(paths) > 1:
        ref_name = os.path.basename(paths[0])
        ref = per_file[ref_name]
        for p in paths[1:]:
            other = per_file[os.path.basename(p)]
            same = sum(1 for a, b in zip(ref, other) if a == b)
            print(f"== statistics items of {os.path.basename(p)} against {ref_name}: {len(other)} / {len(ref)} items, "
                  f"{same} byte-identical (line + field)")
            for a, b in zip(ref, other):
                if a != b:
                    x, y = a[3], b[3]
                    at = next((i for i in range(min(len(x), len(y))) if x[i] != y[i]), min(len(x), len(y)))
                    print(f"    differs: {a[0]} {a[1]} `{a[2][:30]}` lengths {len(x)} / {len(y)}, first difference at +{at}: "
                          f"...{x[max(0, at - 40):at + 60].decode('latin-1')!r} / ...{y[max(0, at - 40):at + 60].decode('latin-1')!r}")
    print(f"problems: {problems}")


if __name__ == "__main__":
    main()
