#!/usr/bin/env python3
"""List and decode the optimizer-statistics items (`ANALSTATS xx`, `ANALCOMPUTE xx`) of legacy EXP fixtures.

Checks (exp.md section 4.6, Analyst session 5, fixture request R-016):
- where every `ANALSTATS` / `ANALCOMPUTE` line stands (inside a table section after the rows, or
  in a `TABLE ... ENDTABLE` group of a trailer section), its two letters, whether a field follows,
  the form of that field (plain / pieces after `fd ff`), its payload length and piece lengths;
- that the payload of every field is one of three PL/SQL texts and nothing else:
      table   BEGIN  DBMS_STATS.SET_TABLE_STATS(NULL,'"T"',NULL,NULL,NULL,<4 numbers>); END;
      index   BEGIN  DBMS_STATS.SET_INDEX_STATS(NULL,'"I"',NULL,NULL,NULL,<8 numbers>); END;
      column  DECLARE  SREC DBMS_STATS.STATREC; BEGIN SREC.MINVAL := ..; SREC.MAXVAL := ..;
              SREC.EAVS := ..; SREC.CHVALS := ..; SREC.NOVALS := DBMS_STATS.NUMARRAY(..);
              SREC.BKVALS := DBMS_STATS.NUMARRAY(..); SREC.EPC := ..;
              DBMS_STATS.SET_COLUMN_STATS(NULL,'"T"','"C"', NULL ,NULL,NULL,<3 numbers>,srec,<2 numbers>); END;
  (a payload that matches none of them is printed as a PROBLEM);
- every number of these texts against truth_stat.txt of the fixture's directory (lines TAB|, COL|,
  HIST|, IND|): which argument is num_rows, blocks, avg_row_len, num_distinct, density, num_nulls,
  avg_col_len, low / high value, endpoint values and endpoint numbers. Each compared value is
  counted as equal or differing; with -v every item is printed with its comparison;
- the second letter (`S` / `R`) against the export log next to the dump: `R` in every line or in
  none, `R` exactly when the log has `EXP-00091`, and the number of `EXP-00091` lines against (a)
  the number of lines with `R` and (b) the number of `ANALSTATS ?R` lines followed by a PLAIN field
  (= one per table and one per index);
- that every byte of every statistics payload is ASCII (below 0x80), whatever the client set;
- that the column items of a table stand in the column order of its INSERT line;
- one line per table: number of columns (from the INSERT line), number of table-level items,
  column-level items, columns without an item, ANALCOMPUTE lines.
Expects: `*_exp_*.dmp` fixtures or fixture directories; written for the R-016 files
    uv run stat_items.py ../../../corpus/fixtures/18c/stat_exp_18c.dmp -v
    uv run stat_items.py ../../../corpus/fixtures/11g-cp1251 --only ru_stat
Options: -v (one block per item), --only <substring> (file name filter), --md (the item list as a
Markdown table, used for exp.md 4.6).
Importable: `stat_items(path)` returns the list of items with their decoded arguments.
Uses exp_parse.py from the same directory. Exploration tool only. Reads the fixtures, their logs
and truth files, never writes them. New in Analyst session 5.
"""
import glob
import os
import re
import sys
from decimal import Decimal

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from exp_parse import parse  # noqa: E402

NUM = r"-?(?:\d+\.?\d*|\.\d+)(?:E[+-]?\d+)?"
RX_TAB = re.compile(
    rb"^BEGIN  DBMS_STATS\.SET_TABLE_STATS\(NULL,'\"(?P<t>[^']*)\"',NULL,NULL,NULL,"
    rb"(?P<a>[^,]*),(?P<b>[^,]*),(?P<c>[^,]*),(?P<d>[^,)]*)\); END;$", re.S)
RX_IND = re.compile(
    rb"^BEGIN  DBMS_STATS\.SET_INDEX_STATS\(NULL,'\"(?P<i>[^']*)\"',NULL,NULL,NULL,"
    rb"(?P<args>[^)]*)\); END;$", re.S)
RX_COL = re.compile(
    rb"^DECLARE  SREC DBMS_STATS\.STATREC; BEGIN SREC\.MINVAL := (?P<min>NULL|'[0-9A-F]*'); "
    rb"SREC\.MAXVAL := (?P<max>NULL|'[0-9A-F]*'); SREC\.EAVS := (?P<eavs>\d+); "
    rb"SREC\.CHVALS := (?P<chvals>NULL|DBMS_STATS\.CHARARRAY\((?P<chlist>.*?)\)); "
    rb"(?:# )?SREC\.NOVALS := DBMS_STATS\.NUMARRAY\((?P<novals>[^)]*)\); "
    rb"SREC\.BKVALS := DBMS_STATS\.NUMARRAY\((?P<bkvals>[^)]*)\); "
    rb"SREC\.EPC := (?P<epc>\d+); "
    rb"DBMS_STATS\.SET_COLUMN_STATS\(NULL,'\"(?P<t>[^']*)\"','\"(?P<c>[^']*)\"', NULL ,NULL,NULL,"
    rb"(?P<distcnt>[^,]*),(?P<density>[^,]*),(?P<nullcnt>[^,]*),srec,(?P<avgclen>[^,]*),(?P<flags>[^,)]*)\); END;$",
    re.S)


def truth_stat(tdir: str) -> dict:
    """TAB / COL / HIST / IND lines of truth_stat.txt, keyed by object name (ASCII names only)."""
    out: dict = {"TAB": {}, "COL": {}, "HIST": {}, "IND": {}}
    p = os.path.join(tdir, "truth_stat.txt")
    if not os.path.exists(p):
        return out
    for ln in open(p, "rb").read().split(b"\n"):
        try:
            f = ln.decode("ascii").rstrip("\r").split("|")
        except UnicodeDecodeError:
            continue                                   # rows of the Cyrillic-named objects: not in these dumps
        if f[0] == "TAB":
            out["TAB"][f[1]] = f[2:]
        elif f[0] == "COL":
            out["COL"][(f[1], f[2])] = f[3:]
        elif f[0] == "HIST":
            out["HIST"].setdefault((f[1], f[2]), []).append((f[3], f[4], f[5]))
        elif f[0] == "IND":
            out["IND"][f[1]] = f[2:]
    return out


def same_num(a: str, b: str) -> bool:
    try:
        return Decimal(a) == Decimal(b)
    except Exception:
        return a == b


def close_num(a: str, b: str, digits: int = 14) -> bool:
    """Equal after rounding both to `digits` significant digits (the truth prints density with TM9)."""
    try:
        x, y = Decimal(a), Decimal(b)
    except Exception:
        return a == b
    if x == y:
        return True
    if x == 0 or y == 0:
        return False
    return abs(x - y) / max(abs(x), abs(y)) < Decimal(10) ** (-digits)


def decode(payload: bytes) -> dict:
    m = RX_TAB.match(payload)
    if m:
        return {"what": "table", "table": m["t"].decode("latin-1"),
                "args": [m[k].decode() for k in ("a", "b", "c", "d")]}
    m = RX_IND.match(payload)
    if m:
        return {"what": "index", "index": m["i"], "args": m["args"].decode().split(",")}
    m = RX_COL.match(payload)
    if m:
        d = {k: m[k].decode("latin-1") for k in ("min", "max", "eavs", "chvals", "novals", "bkvals", "epc",
                                                    "distcnt", "density", "nullcnt", "avgclen", "flags")}
        d.update({"what": "column", "table": m["t"].decode("latin-1"), "column": m["c"].decode("latin-1"),
                  "chlist": m["chlist"], "has_hash": b"; # SREC.NOVALS" in payload})
        return d
    return {"what": "UNKNOWN"}


def stat_items(path: str) -> list:
    r = parse(path, "keyword")
    post = {}
    for s in r["sections"]:
        for it in s["post_items"]:
            post[it["off"]] = s["name"]
    its = r["items"]
    out = []
    for i, it in enumerate(its):
        if it["kind"] != "line":
            continue
        m = re.match(r'^(ANALSTATS|ANALCOMPUTE) ([A-Z])([A-Z]) "([^"]*)"(.*)$', it["text"], re.S)
        if not m:
            continue
        e = {"off": it["off"], "line": it["text"], "verb": m.group(1), "l1": m.group(2), "l2": m.group(3),
             "target": m.group(4), "rest": m.group(5),
             "where": ("table section " + post[it["off"]] + ", after the rows") if it["off"] in post
             else f"trailer '{it['zone']}', group TABLE \"{it['group']}\"",
             "field": None}
        if i + 1 < len(its) and its[i + 1]["kind"] != "line":
            f = its[i + 1]
            e["field"] = {"off": f["off"], "form": f["kind"], "len": len(f["raw"]), "raw": f["raw"],
                          "pieces": [n for _o, n in f["pieces"]], "piece_offs": [o for o, _n in f["pieces"]],
                          "end": f["end"]}
            e["dec"] = decode(f["raw"])
        out.append(e)
    return out, r


def compare(e: dict, tr: dict) -> list:
    """List of (name, dump value, truth value, equal?) for one decoded item."""
    d = e.get("dec")
    res = []
    if not d:
        return res
    if d["what"] == "table" and d["table"] in tr["TAB"]:
        t = tr["TAB"][d["table"]]
        res += [("num_rows", d["args"][0], t[0], same_num(d["args"][0], t[0])),
                ("blocks", d["args"][1], t[1], same_num(d["args"][1], t[1])),
                ("avg_row_len", d["args"][2], t[5], same_num(d["args"][2], t[5]))]
    if d["what"] == "index":
        name = d["index"].decode("latin-1").strip()
        t = tr["IND"].get(name)
        if t is None and name.startswith("SYS_C"):
            cand = [v for k, v in tr["IND"].items() if k.startswith("SYS_C") and v[0] == e["target"]]
            t = cand[0] if len(cand) == 1 else None
        if t is not None:
            a = d["args"]
            # IND|name|table|uniqueness|blevel|leaf_blocks|distinct_keys|avg_leaf|avg_data|clustering|num_rows|sample|date
            for nm, got, want in (("num_rows", a[0], t[8]), ("leaf_blocks", a[1], t[3]), ("distinct_keys", a[2], t[4]),
                                  ("avg_leaf_blocks_per_key", a[3], t[5]), ("avg_data_blocks_per_key", a[4], t[6]),
                                  ("clustering_factor", a[5], t[7]), ("blevel", a[6], t[2])):
                res.append((nm, got, want, same_num(got, want)))
    if d["what"] == "column" and (d["table"], d["column"]) in tr["COL"]:
        t = tr["COL"][(d["table"], d["column"])]
        # COL|t|c|num_distinct|num_nulls|num_buckets|density|histogram|avg_col_len|sample|low|high|date
        res += [("num_distinct", d["distcnt"], t[0], same_num(d["distcnt"], t[0])),
                ("num_nulls", d["nullcnt"], t[1], same_num(d["nullcnt"], t[1])),
                ("density", d["density"], t[3], close_num(d["density"], t[3])),
                ("avg_col_len", d["avgclen"], t[5], same_num(d["avgclen"], t[5])),
                ("low_value", d["min"].strip("'"), t[7], d["min"].strip("'") == t[7]),
                ("high_value", d["max"].strip("'"), t[8], d["max"].strip("'") == t[8])]
        h = tr["HIST"].get((d["table"], d["column"]), [])
        nov = [x for x in d["novals"].split(",") if x]
        bkv = [x for x in d["bkvals"].split(",") if x]
        res.append(("EPC = histogram rows", d["epc"], str(len(h)), int(d["epc"]) == len(h)))
        res.append(("len(NOVALS) = EPC", str(len(nov)), d["epc"], len(nov) == int(d["epc"])))
        res.append(("len(BKVALS) = EPC", str(len(bkv)), d["epc"], len(bkv) == int(d["epc"])))
        if len(h) == len(nov) == len(bkv):
            res.append(("NOVALS = endpoint_value", f"{len(nov)} values", "", all(close_num(a, b[1], 15) for a, b in zip(nov, h))))
            res.append(("BKVALS = endpoint_number", f"{len(bkv)} values", "", all(same_num(a, b[0]) for a, b in zip(bkv, h))))
    return res


def main() -> None:
    args = sys.argv[1:]
    verbose, md = "-v" in args, "--md" in args
    only = args[args.index("--only") + 1] if "--only" in args else ""
    skip = {args.index("--only") + 1} if "--only" in args else set()
    paths = []
    for k, a in enumerate(args):
        if a.startswith("-") or k in skip:
            continue
        paths += sorted(glob.glob(os.path.join(a, "*_exp_*.dmp"))) if os.path.isdir(a) else [a]
    tot = {"files": 0, "items": 0, "fields": 0, "unknown": 0, "eq": 0, "ne": 0, "letter_fit": 0, "letter_bad": 0,
           "exp91_eq_R_lines": 0, "exp91_eq_R_plain_fields": 0, "non_ascii_payloads": 0, "col_order_ok": 0,
           "col_order_bad": 0, "payload_bytes": 0}
    forms: dict = {}
    for p in paths:
        if only not in os.path.basename(p):
            continue
        its, r = stat_items(p)
        if not its:
            continue
        tot["files"] += 1
        # truth_stat.txt describes the state of the R-016 exports only (older dumps of the same folder
        # were written before the statistics were gathered)
        is_r016 = os.path.basename(p).startswith(("stat_exp", "ru_stat_exp"))
        tr = truth_stat(os.path.dirname(os.path.abspath(p))) if is_r016 else truth_stat("/nonexistent")
        log = p[:-4] + ".log"
        n91 = open(log, "rb").read().count(b"EXP-00091") if os.path.exists(log) else -1
        n_r = sum(1 for e in its if e["l2"] == "R")
        n_r_stats = sum(1 for e in its if e["l2"] == "R" and e["verb"] == "ANALSTATS")
        n_r_plain = sum(1 for e in its if e["l2"] == "R" and e["field"] is not None and e["field"]["form"] == "plain")
        tot["exp91_eq_R_lines"] += n91 == n_r
        tot["exp91_eq_R_plain_fields"] += n91 == n_r_plain
        codes: dict = {}
        ne_here = []
        for e in its:
            key = f"{e['verb']} {e['l1']}{e['l2']}" + ("" if e["field"] is None else " + " + e["field"]["form"])
            codes[key] = codes.get(key, 0) + 1
            tot["items"] += 1
            if e["field"] is not None:
                tot["fields"] += 1
                tot["payload_bytes"] += e["field"]["len"]
                if any(x >= 0x80 for x in e["field"]["raw"]):
                    tot["non_ascii_payloads"] += 1
                    print(f"    non-ASCII payload: {os.path.basename(p)} 0x{e['field']['off']:x} {e['field']['raw'][:70]!r}")
                w = e["dec"]["what"]
                forms[(e["l1"], w, e["field"]["form"])] = forms.get((e["l1"], w, e["field"]["form"]), 0) + 1
                if w == "UNKNOWN":
                    tot["unknown"] += 1
                    print(f"PROBLEM {os.path.basename(p)} 0x{e['field']['off']:x}: payload matches no pattern: "
                          f"{e['field']['raw'][:80]!r}")
            e["cmp"] = compare(e, tr)
            for nm, got, want, ok in e["cmp"]:
                tot["eq" if ok else "ne"] += 1
                if not ok:
                    ne_here.append((e, nm, got, want))
        fit = (n_r == 0) == (n91 == 0) and n_r in (0, len(its))
        tot["letter_fit" if fit else "letter_bad"] += 1
        h = r["header"]
        print(f"{os.path.basename(p)}: client {h['cs_at_1']} database {h['cs_b']}; {len(its)} items {codes}; "
              f"lines with R: {n_r} (ANALSTATS {n_r_stats}, of these with a plain field {n_r_plain}); "
              f"EXP-00091 in log: {n91}; "
              f"{'letter fits log' if fit else 'LETTER CONTRADICTS LOG'}")
        # per table
        for s in r["sections"]:
            name = s["name"]
            ncols = len(s.get("cols", [])) if "cols" in s else None
            names = re.findall(r'"([^"]*)"', s.get("insert", "").split(" VALUES ")[0])[1:]
            tl = [e for e in its if e["target"] == name and e.get("dec", {}).get("what") == "table"]
            cl = [e for e in its if e.get("dec", {}).get("what") == "column" and e["dec"]["table"] == name]
            il = [e for e in its if e.get("dec", {}).get("what") == "index" and e["target"] == name]
            ac = [e for e in its if e["verb"] == "ANALCOMPUTE" and e["target"] == name]
            if cl:
                in_order = [e["dec"]["column"] for e in cl] == [n for n in names if n in {e["dec"]["column"] for e in cl}]
                tot["col_order_ok" if in_order else "col_order_bad"] += 1
                if not in_order:
                    print(f"    column items of {name} NOT in column order: {[e['dec']['column'] for e in cl]}")
            print(f"    table {name}: columns {ncols}; ANALSTATS table items {len(tl)}, column items {len(cl)}, "
                  f"index items {len(il)}; ANALCOMPUTE lines {len(ac)} ({', '.join(e['l1'] + e['l2'] for e in ac)})")
        for e, nm, got, want in ne_here:
            print(f"    DIFFERS from truth: 0x{e['off']:x} {e['line'][:40]!r} {nm}: dump {got} truth {want}")
        if md:
            print("| offset | line | where | field: form, payload bytes (pieces) | content |")
            print("|---|---|---|---|---|")
        for e in its:
            f = e["field"]
            d = e.get("dec", {})
            if d.get("what") == "table":
                c = "SET_TABLE_STATS " + ",".join(d["args"])
            elif d.get("what") == "index":
                c = "SET_INDEX_STATS " + d["index"].decode("latin-1").strip()[:20] + " " + ",".join(d["args"])
            elif d.get("what") == "column":
                c = (f"SET_COLUMN_STATS {d['column']}: MINVAL {d['min']} MAXVAL {d['max']} EAVS {d['eavs']} "
                     f"CHVALS {d['chvals'][:40]} EPC {d['epc']} args {d['distcnt']},{d['density']},{d['nullcnt']},"
                     f"srec,{d['avgclen']},{d['flags']}")
            else:
                c = "(line only)" if f is None else "UNKNOWN"
            ftxt = "-" if f is None else (f"{f['form']} @0x{f['off']:x}, {f['len']}"
                                          + ("" if f["form"] == "plain" else " (" + ", ".join(map(str, f["pieces"])) + ")"))
            if md:
                print(f"| 0x{e['off']:x} | `{e['line'][:48]}` | {e['where']} | {ftxt} | {c} |")
            elif verbose:
                print(f"  0x{e['off']:x} {e['line']!r}\n      where: {e['where']}\n      field: {ftxt}\n      {c}")
                if d.get("what") == "column":
                    print(f"      NOVALS {d['novals'][:200]}\n      BKVALS {d['bkvals'][:200]}")
                for nm, got, want, ok in e.get("cmp", []):
                    print(f"      {'ok  ' if ok else 'DIFF'} {nm}: dump {got} truth {want}")
    print(tot)
    print("field forms by first letter and content:", {f"{k[0]}/{k[1]}/{k[2]}": v for k, v in sorted(forms.items())})


if __name__ == "__main__":
    main()
