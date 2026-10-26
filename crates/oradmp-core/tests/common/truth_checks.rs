//! T-10b (ADR-38): the truth comparisons of the delegated families as functions that return the list of
//! differences (empty = equal). A delegated test calls its function and asserts the list is empty;
//! `t10_truth.rs::every_truth_file_is_read_or_listed` calls every one of them, so that the process-wide set of
//! `common::truth_text` knows which `(folder, name)` a check opened.
//!
//! Each check has two layers: a pure function `*_diff` on the truth lines and the rows the reader delivered
//! (the break tests of `t10_truth.rs` change a truth line in memory and expect exactly one difference), and a
//! family function `*_diffs` that opens the fixtures, reads the tables with the reader and calls the pure one.
//! Expected values come from the truth files and from the generating SQL of `corpus/sql/` named at each
//! function; the reader's output is never the source of an expectation. A difference text names the file, the
//! row and the column, never a value.
#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use std::collections::{BTreeMap, BTreeSet, HashMap};

use oradmp_core::charset::Charset;
use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::{Error, Warning};

use super::{
    days_from_civil, decimal_text, fixtures, root, rows, truth, truth_text, try_read, ts_text,
};

pub type Rows = Vec<Vec<String>>;
pub type Diffs = Vec<String>;

// ---------------------------------------------------------------------------------------------------------
// reading helpers
// ---------------------------------------------------------------------------------------------------------

/// The bytes of `Typ=1 Len=3 [CharacterSet=X]: 61,62,63`; `None` for `NULL`.
pub fn dump_bytes(f: &str) -> Option<Vec<u8>> {
    if f.trim() == "NULL" {
        return None;
    }
    let (_, list) = f.split_once(": ")?;
    Some(
        list.split(',')
            .map(|h| u8::from_str_radix(h, 16).unwrap())
            .collect(),
    )
}

pub fn utf16be_string(b: &[u8]) -> String {
    let u: Vec<u16> = b
        .chunks(2)
        .map(|p| u16::from_be_bytes([p[0], *p.get(1).unwrap_or(&0)]))
        .collect();
    String::from_utf16(&u).unwrap()
}

fn cell_or_null(s: Option<String>) -> String {
    s.unwrap_or_else(|| "<NULL>".to_string())
}

fn truth_null(s: &str) -> String {
    if s == "NULL" || s == "<NULL>" {
        "<NULL>".to_string()
    } else {
        s.to_string()
    }
}

/// A table of a fixture read with `o`: the column names, the rows as cell texts, the warnings.
pub fn read_named(
    rel: &str,
    table: &str,
    o: &ReadOptions,
) -> Result<(Vec<String>, Rows, Vec<Warning>), Error> {
    let mut d = open(fixtures().join(rel), Options::default())?;
    let mut r = d.read_table(TableRef::ByName(table), o)?;
    let mut batches = Vec::new();
    while let Some(b) = r.next_batch()? {
        batches.push(b);
    }
    let names = match batches.first() {
        Some(b) => b
            .schema()
            .fields()
            .iter()
            .map(|f| f.name().clone())
            .collect(),
        None => Vec::new(),
    };
    Ok((names, rows(&batches), r.warnings().to_vec()))
}

pub fn lenient() -> ReadOptions {
    ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    }
}

/// Rows ordered by the numeric ID of column 0 (the truth is ordered by ID, the file by insertion).
pub fn by_id(mut r: Rows) -> Rows {
    r.sort_by_key(|x| x[0].parse::<i64>().unwrap_or(i64::MIN));
    r
}

/// The data lines of `tests/golden/exp_catalog.tsv` (tool output of `gen_exp_catalog.py`): (file, table).
pub fn golden_sections() -> Vec<(String, String)> {
    std::fs::read_to_string(root().join("tests/golden/exp_catalog.tsv"))
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("file\t") && !l.is_empty())
        .map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            (c[0].to_string(), c[2].to_string())
        })
        .collect()
}

/// The files (`folder/name`) of the golden catalog that hold a section of `table`.
pub fn files_with_table(table: &str) -> Vec<String> {
    golden_sections()
        .into_iter()
        .filter(|(_, t)| t == table)
        .map(|(f, _)| f)
        .collect()
}

fn diff(out: &mut Diffs, ctx: &str, what: &str) {
    out.push(format!("{ctx}: {what}"));
}

// ---------------------------------------------------------------------------------------------------------
// the Oracle map of CL8MSWIN1251 (truth_csmap.txt)
// ---------------------------------------------------------------------------------------------------------

/// The UTF-16 text Oracle produced for each of the 256 bytes of CL8MSWIN1251 (`truth_csmap.txt` of `18c`).
pub fn cp1251_map() -> &'static Vec<String> {
    static MAP: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = vec![String::new(); 256];
        let mut n = 0;
        for l in truth("18c", "truth_csmap.txt") {
            if l[0] == "CL8MSWIN1251" {
                let b = usize::from_str_radix(&l[1], 16).unwrap();
                map[b] = utf16be_string(&dump_bytes(&l[2]).unwrap());
                n += 1;
            }
        }
        assert_eq!(n, 256);
        map
    })
}

pub fn decode_cp1251(b: &[u8]) -> String {
    let m = cp1251_map();
    b.iter().map(|x| m[usize::from(*x)].as_str()).collect()
}

/// The database bytes of a text by the inverse of Oracle's map; `None` when a character has no byte.
pub fn encode_cp1251(s: &str) -> Option<Vec<u8>> {
    static INVERSE: std::sync::OnceLock<HashMap<String, u8>> = std::sync::OnceLock::new();
    let inverse = INVERSE.get_or_init(|| {
        cp1251_map()
            .iter()
            .enumerate()
            .map(|(i, c)| (c.clone(), u8::try_from(i).unwrap()))
            .collect()
    });
    s.chars()
        .map(|c| inverse.get(&c.to_string()).copied())
        .collect()
}

/// The two texts of `truth_csmap.txt` are equal.
pub fn csmap_diff(a: &str, b: &str) -> Diffs {
    let mut out = Vec::new();
    if a != b {
        diff(&mut out, "truth_csmap", "the two folders differ");
    }
    out
}

/// `truth_csmap.txt` of `11g` is the file of `18c` (both are opened, so both count as read), and the map of
/// CL8MSWIN1251 has its 256 entries.
pub fn csmap_diffs() -> Diffs {
    let mut out = csmap_diff(
        &truth_text("18c", "truth_csmap.txt"),
        &truth_text("11g", "truth_csmap.txt"),
    );
    if cp1251_map().len() != 256 {
        diff(&mut out, "truth_csmap", "map size");
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// R_TEXT in all 27 sections (b1)
// ---------------------------------------------------------------------------------------------------------

/// `rows`: ID, V, C, VC, N, LV, D of one R_TEXT section; `want`: the `R_TEXT` lines of `truth_ru.txt`:
/// `R_TEXT|id|DUMP v|DUMP c|DUMP vc|DUMP n|LENGTHB(lv)|ORA_HASH|DUMP(lv, 16, 1, 60)|d`. V, C, VC are database
/// bytes of CL8MSWIN1251 (decoded with Oracle's map), N is UTF-16 BE.
pub fn r_text_diff(ctx: &str, want: &[Vec<String>], got: &Rows) -> Diffs {
    let mut out = Vec::new();
    let got = by_id(got.clone());
    if got.len() != want.len() {
        diff(&mut out, ctx, "row count");
        return out;
    }
    for (row, l) in got.iter().zip(want) {
        let id = &l[1];
        if row[0] != *id {
            diff(&mut out, ctx, &format!("ID order at id {id}"));
            continue;
        }
        for (cell_i, t_i, name) in [(1usize, 2usize, "V"), (2, 3, "C"), (3, 4, "VC")] {
            let w = cell_or_null(dump_bytes(&l[t_i]).map(|b| decode_cp1251(&b)));
            if row[cell_i] != w {
                diff(&mut out, ctx, &format!("id {id} column {name}"));
            }
        }
        let n = cell_or_null(dump_bytes(&l[5]).map(|b| utf16be_string(&b)));
        if row[4] != n {
            diff(&mut out, ctx, &format!("id {id} column N"));
        }
        // LV: the truth has LENGTHB and the first 60 bytes (the hash is Oracle's, not recomputed)
        if row[5] == "<NULL>" {
            if l[6] != "NULL" || l[8] != "NULL" {
                diff(&mut out, ctx, &format!("id {id} column LV is NULL"));
            }
        } else {
            match encode_cp1251(&row[5]) {
                None => diff(&mut out, ctx, &format!("id {id} column LV has no byte")),
                Some(b) => {
                    if l[6] != b.len().to_string() {
                        diff(&mut out, ctx, &format!("id {id} column LV length"));
                    }
                    match dump_bytes(&l[8]) {
                        Some(head) if b.starts_with(&head) => {}
                        _ => diff(&mut out, ctx, &format!("id {id} column LV head")),
                    }
                }
            }
        }
        let d = if l[9] == "NULL" {
            "<NULL>".to_string()
        } else {
            format!("{} 00:00:00", l[9])
        };
        if row[6] != d {
            diff(&mut out, ctx, &format!("id {id} column D"));
        }
    }
    out
}

pub fn r_text_truth() -> Vec<Vec<String>> {
    truth("11g-cp1251", "truth_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_TEXT")
        .collect()
}

/// R_TEXT of every file that has one (27 sections: `exp_catalog.tsv`), read with the default options.
pub fn r_text_diffs() -> Diffs {
    let want = r_text_truth();
    let mut out = Vec::new();
    let files = files_with_table("R_TEXT");
    if files.len() != 27 {
        diff(&mut out, "R_TEXT", "section count of the catalog");
    }
    for f in files {
        match read_named(&f, "R_TEXT", &ReadOptions::default()) {
            Err(e) => diff(&mut out, &f, &format!("read failed: {:?}", e.code)),
            Ok((names, r, w)) => {
                if names != ["ID", "V", "C", "VC", "N", "LV", "D"] {
                    diff(&mut out, &f, "column names");
                }
                if !w.is_empty() {
                    diff(&mut out, &f, &format!("{} warnings", w.len()));
                }
                out.extend(r_text_diff(&f, &want, &r));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// R_CLOB and R_LOB2 (truth_ru.txt, truth_lob_ru.txt)
// ---------------------------------------------------------------------------------------------------------

pub const R_CLOB_FILES: [&str; 7] = [
    "ru_exp_al32utf8_11g.dmp",
    "ru_exp_cp1251_11g.dmp",
    "ru_exp_direct_cp1251_11g.dmp",
    "ru_exp_direct_us7ascii_11g.dmp",
    "ru_exp_us7ascii_11g.dmp",
    "ru_exp_w1252_11g.dmp",
    "ru_tables_exp_us7ascii_11g.dmp",
];

/// `rows`: ID, CL of R_CLOB; `want`: `R_CLOB|id|GETLENGTH|DUMP(first 60 bytes)` (NULL line for a NULL value).
pub fn r_clob_diff(ctx: &str, want: &[Vec<String>], got: &Rows) -> Diffs {
    let mut out = Vec::new();
    let got = by_id(got.clone());
    if got.len() != want.len() {
        diff(&mut out, ctx, "row count");
        return out;
    }
    for (row, l) in got.iter().zip(want) {
        if row[0] != l[1] {
            diff(&mut out, ctx, "ID order");
            continue;
        }
        if l[2] == "NULL" {
            if row[1] != "<NULL>" {
                diff(&mut out, ctx, &format!("id {} is not NULL", l[1]));
            }
            continue;
        }
        if row[1].chars().count().to_string() != l[2] {
            diff(&mut out, ctx, &format!("id {} length", l[1]));
        }
        let head = dump_bytes(&l[3]).unwrap();
        let text = decode_cp1251(&head);
        if !row[1].starts_with(&text) {
            diff(&mut out, ctx, &format!("id {} head", l[1]));
        }
    }
    out
}

pub fn r_clob_diffs() -> Diffs {
    let want: Vec<Vec<String>> = truth("11g-cp1251", "truth_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_CLOB")
        .collect();
    let mut out = Vec::new();
    if want.len() != 3 {
        diff(&mut out, "R_CLOB", "truth line count");
    }
    for f in R_CLOB_FILES {
        let rel = format!("11g-cp1251/{f}");
        match read_named(&rel, "R_CLOB", &ReadOptions::default()) {
            Err(e) => diff(&mut out, &rel, &format!("read failed: {:?}", e.code)),
            Ok((_, r, w)) => {
                if !w.is_empty() {
                    diff(&mut out, &rel, &format!("{} warnings", w.len()));
                }
                out.extend(r_clob_diff(&rel, &want, &r));
            }
        }
    }
    out
}

pub const R_LOB2_FILES: [&str; 2] = [
    "11g-cp1251/ru_lob2_exp_cp1251_11g.dmp",
    "11g-cp1251/ru_lob2_exp_us7ascii_11g.dmp",
];

/// Column B of R_LOB2 by the rule of `corpus/sql/15_ru_lob2.sql`, as lower-case hex: `blk` is
/// `CHR(192) .. CHR(255) || CHR(168) || CHR(184)` (66 bytes in the database set), row 1 is `blk`, row 2 is
/// `blk` 303 times and `SUBSTR(blk, 1, 2)` (the bytes `c0 c1`), `UTL_RAW.CAST_TO_RAW` keeps the bytes; row 3
/// is NULL. Built from the numbers of the SQL, not from the reader's C and not from any character map.
pub fn r_lob2_b_rule(id: &str) -> Option<String> {
    let mut blk: Vec<u8> = (192u8..=255).collect();
    blk.push(168);
    blk.push(184);
    let bytes: Vec<u8> = match id {
        "1" => blk,
        "2" => {
            let mut v = blk.repeat(303);
            v.extend_from_slice(&blk[..2]);
            v
        }
        _ => return None,
    };
    Some(bytes.iter().map(|x| format!("{x:02x}")).collect())
}

/// `rows`: ID, C, N, B of R_LOB2; `want`: `R_LOB2|id|len c|len n|len b|DUMP head c|DUMP head n|DUMP tail c|
/// DUMP tail n` (`truth_lob_ru.txt`; C is database bytes, N is UTF-16 code units, B is the bytes of the rule `r_lob2_b_rule`).
pub fn r_lob2_diff(ctx: &str, want: &[Vec<String>], got: &Rows) -> Diffs {
    let mut out = Vec::new();
    let got = by_id(got.clone());
    if got.len() != want.len() {
        diff(&mut out, ctx, "row count");
        return out;
    }
    for (row, l) in got.iter().zip(want) {
        let id = &l[1];
        if row[0] != *id {
            diff(&mut out, ctx, "ID order");
            continue;
        }
        if l[2] == "NULL" {
            if row[1..4].iter().any(|c| c != "<NULL>") {
                diff(&mut out, ctx, &format!("id {id} is not NULL"));
            }
            continue;
        }
        let (c, n, b) = (&row[1], &row[2], &row[3]);
        if c.chars().count().to_string() != l[2] {
            diff(&mut out, ctx, &format!("id {id} column C length"));
        }
        if n.encode_utf16().count().to_string() != l[3] {
            diff(&mut out, ctx, &format!("id {id} column N length"));
        }
        if (b.len() / 2).to_string() != l[4] {
            diff(&mut out, ctx, &format!("id {id} column B length"));
        }
        let head_c = decode_cp1251(&dump_bytes(&l[5]).unwrap());
        let tail_c = decode_cp1251(&dump_bytes(&l[7]).unwrap());
        if !c.starts_with(&head_c) || !c.ends_with(&tail_c) {
            diff(&mut out, ctx, &format!("id {id} column C head or tail"));
        }
        let head_n = utf16be_string(&dump_bytes(&l[6]).unwrap());
        let tail_n = utf16be_string(&dump_bytes(&l[8]).unwrap());
        if !n.starts_with(&head_n) || !n.ends_with(&tail_n) {
            diff(&mut out, ctx, &format!("id {id} column N head or tail"));
        }
        // B is `UTL_RAW.CAST_TO_RAW` of the rule's text; the truth gives its length only (N-6, ADR-44 item 4)
        if r_lob2_b_rule(id).as_deref() != Some(b.as_str()) {
            diff(
                &mut out,
                ctx,
                &format!("id {id} column B against the SQL rule"),
            );
        }
    }
    out
}

pub fn r_lob2_diffs() -> Diffs {
    let want: Vec<Vec<String>> = truth("11g-cp1251", "truth_lob_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_LOB2")
        .collect();
    let mut out = Vec::new();
    if want.len() != 3 {
        diff(&mut out, "R_LOB2", "truth line count");
    }
    for rel in R_LOB2_FILES {
        match read_named(rel, "R_LOB2", &ReadOptions::default()) {
            Err(e) => diff(&mut out, rel, &format!("read failed: {:?}", e.code)),
            Ok((_, r, w)) => {
                if !w.is_empty() {
                    diff(&mut out, rel, &format!("{} warnings", w.len()));
                }
                out.extend(r_lob2_diff(rel, &want, &r));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// R_STAT (b2): 08_ru_stats_objects.sql, truth_stat.txt of 11g-cp1251
// ---------------------------------------------------------------------------------------------------------

/// The text of `word(i)` of `corpus/sql/08_ru_stats_objects.sql`: `3 + MOD(i, 6)` code points from
/// `1040 + MOD(i * 13, 50)` (all in CL8MSWIN1251).
fn r_stat_word(i: i64) -> String {
    let start = 1040 + (i * 13) % 50;
    (0..3 + i % 6)
        .map(|k| char::from_u32(u32::try_from(start + k).unwrap()).unwrap())
        .collect()
}

/// The row of `08_ru_stats_objects.sql` for ID `i`: ID, GRP, TXT, CODE, DT, AMT, NOTE as cell texts.
pub fn r_stat_rule(i: i64) -> Vec<String> {
    let grp = if i % 2 == 0 {
        1
    } else if i % 5 == 0 {
        2
    } else {
        i % 7 + 3
    };
    let txt = if i % 17 == 0 {
        "<NULL>".to_string()
    } else {
        r_stat_word(i)
    };
    let code = format!("K{:03}  ", i % 50);
    let secs = (days_from_civil(2026, 1, 1) + (i * 3) % 200) * 86_400;
    // ROUND(MOD(i * 37, 1000) / 7, 2) in cents, half away from zero: floor((200 m + 7) / 14)
    let amt = if i % 11 == 0 {
        "<NULL>".to_string()
    } else {
        let m = (i * 37) % 1000;
        decimal_text(&((200 * m + 7) / 14).to_string(), 2)
    };
    let note = if i % 7 == 0 {
        "<NULL>".to_string()
    } else {
        format!("note {i}")
    };
    vec![
        i.to_string(),
        grp.to_string(),
        txt,
        code,
        ts_text(secs),
        amt,
        note,
    ]
}

/// One R_STAT section against `truth_stat.txt` of `11g-cp1251` (`stat`: all its lines) and the rule of the SQL:
/// the row count is `TAB|R_STAT|n`; per column the NULL count and the number of distinct values are those of
/// its `COL|R_STAT|name|num_distinct|num_nulls|...` line (statistics gathered with `estimate_percent => 100`,
/// so exact); the values are the rule by ID.
pub fn r_stat_diff(ctx: &str, stat: &[Vec<String>], names: &[String], got: &Rows) -> Diffs {
    let mut out = Vec::new();
    let tab = stat
        .iter()
        .find(|l| l[0] == "TAB" && l[1] == "R_STAT")
        .unwrap();
    if got.len().to_string() != tab[2] {
        diff(&mut out, ctx, "row count against TAB");
    }
    for l in stat.iter().filter(|l| l[0] == "COL" && l[1] == "R_STAT") {
        let Some(ci) = names.iter().position(|n| *n == l[2]) else {
            diff(&mut out, ctx, &format!("column {} is missing", l[2]));
            continue;
        };
        let nulls = got.iter().filter(|r| r[ci] == "<NULL>").count();
        let distinct: BTreeSet<&str> = got
            .iter()
            .filter(|r| r[ci] != "<NULL>")
            .map(|r| r[ci].as_str())
            .collect();
        if nulls.to_string() != l[4] {
            diff(&mut out, ctx, &format!("column {} NULL count", l[2]));
        }
        if distinct.len().to_string() != l[3] {
            diff(&mut out, ctx, &format!("column {} distinct count", l[2]));
        }
    }
    for r in got {
        let Ok(i) = r[0].parse::<i64>() else {
            diff(&mut out, ctx, "ID is not a number");
            continue;
        };
        let want = r_stat_rule(i);
        for (ci, name) in ["ID", "GRP", "TXT", "CODE", "DT", "AMT", "NOTE"]
            .iter()
            .enumerate()
        {
            if r[ci] != want[ci] {
                diff(&mut out, ctx, &format!("id {i} column {name}"));
            }
        }
    }
    out
}

pub fn r_stat_diffs() -> Diffs {
    let stat = truth("11g-cp1251", "truth_stat.txt");
    let mut out = Vec::new();
    let files = files_with_table("R_STAT");
    if files.len() != 10 {
        diff(&mut out, "R_STAT", "section count of the catalog");
    }
    for f in files {
        match read_named(&f, "R_STAT", &ReadOptions::default()) {
            Err(e) => diff(&mut out, &f, &format!("read failed: {:?}", e.code)),
            Ok((names, r, w)) => {
                if names != ["ID", "GRP", "TXT", "CODE", "DT", "AMT", "NOTE"] {
                    diff(&mut out, &f, "column names");
                    continue;
                }
                if !w.is_empty() {
                    diff(&mut out, &f, &format!("{} warnings", w.len()));
                }
                out.extend(r_stat_diff(&f, &stat, &names, &r));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// T_STC (b5): 10_stc.sql, truth_stc.txt, truth_stc_extra.txt
// ---------------------------------------------------------------------------------------------------------

/// The row of `corpus/sql/10_stc.sql` for ID `i` (1 to 50) of table `table`: ID, A, B, C, then ALLNULL (all
/// tables but T_STC2) or ADDED_LATER and ADDED_NUM (T_STC2).
pub fn stc_rule(table: &str, i: i64) -> Vec<String> {
    let mut v = vec![
        i.to_string(),
        format!("a{}", i % 7),
        format!("b{i}"),
        ts_text((days_from_civil(2020, 1, 1) + i) * 86_400),
    ];
    if table == "T_STC2" {
        v.push("<NULL>".to_string());
        v.push((i % 3).to_string());
    } else {
        v.push("<NULL>".to_string());
    }
    v
}

/// One T_STC section: `count` is the row count of the truth (`TAB|T_STCn|50`), `cols` the `COLUMN|T|pos|name|
/// stats:distinct:nulls` (or `NO STATISTICS`) lines of the table; the values are the rule of the SQL.
pub fn stc_diff(
    ctx: &str,
    table: &str,
    count: &str,
    cols: &[Vec<String>],
    names: &[String],
    got: &Rows,
) -> Diffs {
    let mut out = Vec::new();
    if got.len().to_string() != count {
        diff(&mut out, ctx, &format!("{table} row count"));
    }
    if cols.len() != names.len() {
        diff(&mut out, ctx, &format!("{table} column count"));
        return out;
    }
    for (pos, l) in cols.iter().enumerate() {
        if names[pos] != l[3] {
            diff(&mut out, ctx, &format!("{table} column {} name", l[2]));
            continue;
        }
        if let Some(s) = l[4].strip_prefix("stats:") {
            let (d, n) = s.split_once(':').unwrap();
            let nulls = got.iter().filter(|r| r[pos] == "<NULL>").count();
            let distinct: BTreeSet<&str> = got
                .iter()
                .filter(|r| r[pos] != "<NULL>")
                .map(|r| r[pos].as_str())
                .collect();
            if d != distinct.len().to_string() || n != nulls.to_string() {
                diff(
                    &mut out,
                    ctx,
                    &format!("{table} column {} statistics", l[3]),
                );
            }
        }
    }
    for r in got {
        let Ok(i) = r[0].parse::<i64>() else {
            diff(&mut out, ctx, &format!("{table} ID is not a number"));
            continue;
        };
        if !(1..=50).contains(&i) {
            diff(&mut out, ctx, &format!("{table} ID {i} outside 1 to 50"));
            continue;
        }
        let want = stc_rule(table, i);
        if r.len() != want.len() {
            diff(&mut out, ctx, &format!("{table} id {i} width"));
            continue;
        }
        for (ci, (g, w)) in r.iter().zip(&want).enumerate() {
            if g != w {
                diff(
                    &mut out,
                    ctx,
                    &format!("{table} id {i} column {}", names[ci]),
                );
            }
        }
    }
    out
}

pub const STC_FILES: [&str; 4] = [
    "stc_exp",
    "stc_exp_none",
    "stc_exp_us7ascii",
    "stc_exp_direct",
];

/// Counts from `truth_stc.txt` of the folder (T_STC4: `NULL` there, no statistics; its count is the
/// `TAB|T_STC4|50` line of `truth_stc_extra.txt`).
pub fn stc_counts(folder: &str) -> BTreeMap<String, String> {
    let mut counts: BTreeMap<String, String> = truth(folder, "truth_stc.txt")
        .into_iter()
        .filter(|l| l[0] == "TAB")
        .map(|l| (l[1].clone(), l[2].clone()))
        .collect();
    let extra: BTreeMap<String, String> = truth(folder, "truth_stc_extra.txt")
        .into_iter()
        .filter(|l| l[0] == "TAB")
        .map(|l| (l[1].clone(), l[2].clone()))
        .collect();
    for (t, n) in counts.iter_mut() {
        if n == "NULL" {
            *n = extra.get(t).cloned().unwrap_or_else(|| "NULL".to_string());
        }
    }
    counts
}

pub fn stc_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let counts = stc_counts(v);
        if counts.len() != 5 {
            diff(&mut out, v, "TAB line count");
        }
        let columns: Vec<Vec<String>> = truth(v, "truth_stc.txt")
            .into_iter()
            .filter(|l| l[0] == "COLUMN")
            .collect();
        for f in STC_FILES {
            let rel = format!("{v}/{f}_{v}.dmp");
            for (t, n) in &counts {
                let cols: Vec<Vec<String>> =
                    columns.iter().filter(|c| c[1] == *t).cloned().collect();
                match read_named(&rel, t, &ReadOptions::default()) {
                    Err(e) => diff(&mut out, &rel, &format!("{t} read failed: {:?}", e.code)),
                    Ok((names, r, _)) => out.extend(stc_diff(&rel, t, n, &cols, &names, &r)),
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// T_LONG2 (b3): truth_long2.txt
// ---------------------------------------------------------------------------------------------------------

/// `rows`: ID, the LONG text of one T_LONG2 section; `want`: the lines `<id>|<len>|<hex>` of `truth_long2.txt`
/// (UTF-8 bytes of the value) or `<id>|INSERT REFUSED|...` (no such row).
pub fn long2_diff(ctx: &str, want: &[Vec<String>], got: &Rows) -> Diffs {
    let mut out = Vec::new();
    let present: Vec<&Vec<String>> = want.iter().filter(|l| l[1] != "INSERT REFUSED").collect();
    if got.len() != present.len() {
        diff(&mut out, ctx, "row count");
    }
    for l in want {
        let row = got.iter().find(|r| r[0] == l[0]);
        if l[1] == "INSERT REFUSED" {
            if row.is_some() {
                diff(&mut out, ctx, &format!("id {} should not exist", l[0]));
            }
            continue;
        }
        let Some(row) = row else {
            diff(&mut out, ctx, &format!("id {} is missing", l[0]));
            continue;
        };
        let bytes = row[1].as_bytes();
        if bytes.len().to_string() != l[1] {
            diff(&mut out, ctx, &format!("id {} length", l[0]));
        }
        let hex: String = bytes.iter().map(|b| format!("{b:02X}")).collect();
        if hex != l[2].to_uppercase() {
            diff(&mut out, ctx, &format!("id {} bytes", l[0]));
        }
    }
    out
}

/// The files the T-6 test reads: `long2_exp`, `long2_exp_us7ascii`, `long2_exp_direct` of both versions.
pub fn long2_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let want = truth(v, "truth_long2.txt");
        for kind in ["long2_exp", "long2_exp_us7ascii", "long2_exp_direct"] {
            let rel = format!("{v}/{kind}_{v}.dmp");
            match read_named(&rel, "T_LONG2", &ReadOptions::default()) {
                Err(e) => diff(&mut out, &rel, &format!("read failed: {:?}", e.code)),
                Ok((_, r, w)) => {
                    if !w.is_empty() {
                        diff(&mut out, &rel, &format!("{} warnings", w.len()));
                    }
                    out.extend(long2_diff(&rel, &want, &r));
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// T_INTERVAL / T_ROWID (truth_misc.txt) and T_INTERVAL2 / T_ROWID2 (b4, truth_misc2.txt)
// ---------------------------------------------------------------------------------------------------------

/// `[+-]YYYY-MM` or `[+-]YYYYYYYYY-MM` to months.
fn ym_months(t: &str) -> i128 {
    let sign = if t.starts_with('-') { -1 } else { 1 };
    let (y, m) = t[1..].split_once('-').unwrap();
    sign * (y.parse::<i128>().unwrap() * 12 + m.parse::<i128>().unwrap())
}

/// `[+-]D HH:MM:SS[.fff]` to (sign, days, nanoseconds of the day).
fn ds_parts(t: &str) -> (i128, i128, i128) {
    let sign = if t.starts_with('-') { -1 } else { 1 };
    let (days, rest) = t[1..].split_once(' ').unwrap();
    let (hms, frac) = rest.split_once('.').unwrap_or((rest, ""));
    let p: Vec<i128> = hms.split(':').map(|x| x.parse().unwrap()).collect();
    let nanos = if frac.is_empty() {
        0
    } else {
        format!("{frac:0<9}").parse::<i128>().unwrap()
    };
    (
        sign,
        days.parse().unwrap(),
        (p[0] * 3600 + p[1] * 60 + p[2]) * 1_000_000_000 + nanos,
    )
}

/// T_INTERVAL (ID, YM as months/days/nanoseconds, DS as microseconds) and T_ROWID (ID, ROWID, UROWID) of
/// `misc_exp_<v>` against the `INTERVAL` and `ROWID` lines of `truth_misc.txt`. The UROWID of the row is the
/// ROWID of the same row (rule Y11): the truth holds its text only as the DUMP of the 13 bytes, so the text of
/// the ROWID column stands for both.
pub fn misc_diff(ctx: &str, t: &[Vec<String>], interval: &Rows, rowid: &Rows) -> Diffs {
    let mut out = Vec::new();
    let lines: Vec<&Vec<String>> = t.iter().filter(|l| l[0] == "INTERVAL").collect();
    if interval.len() != lines.len() {
        diff(&mut out, ctx, "T_INTERVAL row count");
    }
    for (g, l) in interval.iter().zip(&lines) {
        if g[0] != l[1] {
            diff(&mut out, ctx, "T_INTERVAL ID order");
            continue;
        }
        if l[2] == "NULL" {
            if g[1] != "<NULL>" || g[2] != "<NULL>" {
                diff(
                    &mut out,
                    ctx,
                    &format!("T_INTERVAL id {} is not NULL", l[1]),
                );
            }
            continue;
        }
        if g[1] != format!("{}m0d0ns", ym_months(&l[2])) {
            diff(&mut out, ctx, &format!("T_INTERVAL id {} YM", l[1]));
        }
        let (s, d, n) = ds_parts(&l[4]);
        if g[2] != (s * (d * 86_400_000_000 + n / 1000)).to_string() {
            diff(&mut out, ctx, &format!("T_INTERVAL id {} DS", l[1]));
        }
    }
    let lines: Vec<&Vec<String>> = t.iter().filter(|l| l[0] == "ROWID").collect();
    if rowid.len() != lines.len() {
        diff(&mut out, ctx, "T_ROWID row count");
    }
    for (g, l) in rowid.iter().zip(&lines) {
        let w = truth_null(&l[2]);
        if g[0] != l[1] || g[1] != w || g[2] != w {
            diff(&mut out, ctx, &format!("T_ROWID id {}", l[1]));
        }
    }
    out
}

pub fn misc_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let t = truth(v, "truth_misc.txt");
        let rel = format!("{v}/misc_exp_{v}.dmp");
        let a = read_named(&rel, "T_INTERVAL", &ReadOptions::default());
        let b = read_named(&rel, "T_ROWID", &ReadOptions::default());
        match (a, b) {
            (Ok((_, a, wa)), Ok((_, b, wb))) => {
                if !wa.is_empty() || !wb.is_empty() {
                    diff(&mut out, &rel, "warnings");
                }
                out.extend(misc_diff(&rel, &t, &a, &b));
            }
            _ => diff(&mut out, &rel, "read failed"),
        }
    }
    out
}

/// T_INTERVAL2 (ID, IYM, IDS, IDS0) and T_ROWID2 (ID, RID, URID) of `misc2_exp_<v>` against `truth_misc2.txt`,
/// every row and column. IYM is Interval(MonthDayNano) months: a value beyond 32-bit months (the YEAR(9)
/// extremes, rows 2 and 3) is NULL under `on_value_error="null"` (ADR-28); IDS is months 0, days and the
/// nanoseconds of the day with the sign of the value; IDS0 is a Duration in microseconds.
pub fn misc2_diff(ctx: &str, t: &[Vec<String>], interval: &Rows, rowid: &Rows) -> Diffs {
    let mut out = Vec::new();
    let lines: Vec<&Vec<String>> = t.iter().filter(|l| l[0] == "INTERVAL").collect();
    if interval.len() != lines.len() {
        diff(&mut out, ctx, "T_INTERVAL2 row count");
    }
    for (g, l) in interval.iter().zip(&lines) {
        let id = &l[1];
        if g[0] != *id {
            diff(&mut out, ctx, "T_INTERVAL2 ID order");
            continue;
        }
        // INTERVAL|id|IYM|dump|IDS|dump|IDS0|dump
        let months = ym_months(&l[2]);
        let iym = if i32::try_from(months).is_ok() {
            format!("{months}m0d0ns")
        } else {
            "<NULL>".to_string()
        };
        if g[1] != iym {
            diff(&mut out, ctx, &format!("T_INTERVAL2 id {id} IYM"));
        }
        let (s, d, n) = ds_parts(&l[4]);
        if g[2] != format!("0m{}d{}ns", s * d, s * n) {
            diff(&mut out, ctx, &format!("T_INTERVAL2 id {id} IDS"));
        }
        let (s, d, n) = ds_parts(&l[6]);
        if g[3] != (s * (d * 86_400_000_000 + n / 1000)).to_string() {
            diff(&mut out, ctx, &format!("T_INTERVAL2 id {id} IDS0"));
        }
    }
    let lines: Vec<&Vec<String>> = t.iter().filter(|l| l[0] == "ROWID").collect();
    if rowid.len() != lines.len() {
        diff(&mut out, ctx, "T_ROWID2 row count");
    }
    for (g, l) in rowid.iter().zip(&lines) {
        // ROWID|id|RID text|dump|URID text|dump
        if g[0] != l[1] {
            diff(&mut out, ctx, "T_ROWID2 ID order");
            continue;
        }
        if g[1] != truth_null(&l[2]) {
            diff(&mut out, ctx, &format!("T_ROWID2 id {} RID", l[1]));
        }
        if g[2] != truth_null(&l[4]) {
            diff(&mut out, ctx, &format!("T_ROWID2 id {} URID", l[1]));
        }
    }
    out
}

pub fn misc2_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let t = truth(v, "truth_misc2.txt");
        let rel = format!("{v}/misc2_exp_{v}.dmp");
        let a = read_named(&rel, "T_INTERVAL2", &lenient());
        let b = read_named(&rel, "T_ROWID2", &ReadOptions::default());
        match (a, b) {
            (Ok((_, a, _)), Ok((_, b, _))) => out.extend(misc2_diff(&rel, &t, &a, &b)),
            _ => diff(&mut out, &rel, "read failed"),
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// T_LOB, T_LOB2, T_LOB3 (truth_lob.txt, truth_lob3.txt)
// ---------------------------------------------------------------------------------------------------------

fn text_len(cell: &str) -> String {
    if cell == "<NULL>" {
        "NULL".to_string()
    } else {
        // GETLENGTH of a CLOB counts UTF-16 code units (a non-BMP character counts 2)
        cell.encode_utf16().count().to_string()
    }
}

/// A LargeBinary cell is hex: its byte length, `NULL` for a NULL.
fn hex_len(cell: &str) -> String {
    if cell == "<NULL>" {
        "NULL".to_string()
    } else {
        (cell.len() / 2).to_string()
    }
}

/// `T_LOB` rows (ID, C, B) against `T_LOB|id|GETLENGTH(c)|GETLENGTH(b)|hex of the first 30 bytes of b`.
pub fn t_lob_diff(ctx: &str, want: &[Vec<String>], got: &Rows) -> Diffs {
    let mut out = Vec::new();
    if got.len() != want.len() {
        diff(&mut out, ctx, "T_LOB row count");
        return out;
    }
    for (row, l) in got.iter().zip(want) {
        if row[0] != l[1] {
            diff(&mut out, ctx, "T_LOB ID order");
            continue;
        }
        if text_len(&row[1]) != l[2] {
            diff(&mut out, ctx, &format!("T_LOB id {} C length", l[1]));
        }
        if hex_len(&row[2]) != l[3] {
            diff(&mut out, ctx, &format!("T_LOB id {} B length", l[1]));
        }
        let head = if row[2] == "<NULL>" || row[2].is_empty() {
            "NULL".to_string()
        } else {
            row[2][..row[2].len().min(60)].to_uppercase()
        };
        if head != l[4] {
            diff(&mut out, ctx, &format!("T_LOB id {} B head", l[1]));
        }
    }
    out
}

/// `T_LOB2` rows (ID, A, V, N, B) against `T_LOB2|id|len(a)|v|len(n)|len(b)`.
pub fn t_lob2_diff(ctx: &str, want: &[Vec<String>], got: &Rows) -> Diffs {
    let mut out = Vec::new();
    if got.len() != want.len() {
        diff(&mut out, ctx, "T_LOB2 row count");
        return out;
    }
    for (row, l) in got.iter().zip(want) {
        if row[0] != l[1] {
            diff(&mut out, ctx, "T_LOB2 ID order");
            continue;
        }
        if text_len(&row[1]) != l[2] {
            diff(&mut out, ctx, &format!("T_LOB2 id {} A length", l[1]));
        }
        if row[2] != l[3] {
            diff(&mut out, ctx, &format!("T_LOB2 id {} V", l[1]));
        }
        if text_len(&row[3]) != l[4] {
            diff(&mut out, ctx, &format!("T_LOB2 id {} N length", l[1]));
        }
        if hex_len(&row[4]) != l[5] {
            diff(&mut out, ctx, &format!("T_LOB2 id {} B length", l[1]));
        }
    }
    out
}

/// T_LOB and T_LOB2 of the conventional `lob_exp_<v>` files against `truth_lob.txt`. The direct-path files
/// `lob_exp_direct_<v>` are not read here: `t8_direct.rs::every_direct_file_equals_its_conventional_partner`
/// covers them by pair equality with these files (review 2, N-6).
pub fn lob_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let rel = format!("{v}/lob_exp_{v}.dmp");
        let t = truth(v, "truth_lob.txt");
        let lines = |name: &str| -> Vec<Vec<String>> {
            t.iter().filter(|l| l[0] == name).cloned().collect()
        };
        match read_named(&rel, "T_LOB", &ReadOptions::default()) {
            Ok((_, r, _)) => out.extend(t_lob_diff(&rel, &lines("T_LOB"), &by_id(r))),
            Err(e) => diff(&mut out, &rel, &format!("T_LOB read failed: {:?}", e.code)),
        }
        match read_named(&rel, "T_LOB2", &ReadOptions::default()) {
            Ok((_, r, _)) => out.extend(t_lob2_diff(&rel, &lines("T_LOB2"), &by_id(r))),
            Err(e) => diff(&mut out, &rel, &format!("T_LOB2 read failed: {:?}", e.code)),
        }
    }
    out
}

/// `T_LOB3` rows (ID, C, B, N) against `T_LOB3|id|GETLENGTH(c)|GETLENGTH(b) or NULL|GETLENGTH(n) or NULL|
/// DUMP(first bytes of c)`. Ids 1 to 11: C is S characters `x` and B S bytes of `5a` (`13_optional.sql`), N
/// is NULL; id 20: C and N are the text of the DUMP bytes (UTF-8), B is NULL.
pub fn t_lob3_diff(ctx: &str, want: &[Vec<String>], got: &Rows) -> Diffs {
    let mut out = Vec::new();
    if got.len() != want.len() {
        diff(&mut out, ctx, "T_LOB3 row count");
        return out;
    }
    for (row, l) in got.iter().zip(want) {
        let id = &l[1];
        if row[0] != *id {
            diff(&mut out, ctx, "T_LOB3 ID order");
            continue;
        }
        if text_len(&row[1]) != l[2] {
            diff(&mut out, ctx, &format!("T_LOB3 id {id} C length"));
        }
        if hex_len(&row[2]) != l[3] {
            diff(&mut out, ctx, &format!("T_LOB3 id {id} B length"));
        }
        if text_len(&row[3]) != l[4] {
            diff(&mut out, ctx, &format!("T_LOB3 id {id} N length"));
        }
        let head = dump_bytes(&l[5]).unwrap();
        if !row[1].as_bytes().starts_with(&head) {
            diff(&mut out, ctx, &format!("T_LOB3 id {id} C head"));
        }
        if id == "20" {
            if row[3] != row[1] {
                diff(&mut out, ctx, "T_LOB3 id 20 N is not C");
            }
            // the DUMP shows the whole value (16 bytes)
            if row[1].len() != head.len() {
                diff(&mut out, ctx, "T_LOB3 id 20 C byte length");
            }
        } else {
            if row[1] != "x".repeat(row[1].len()) {
                diff(&mut out, ctx, &format!("T_LOB3 id {id} C is not x"));
            }
            let s: usize = l[3].parse().unwrap();
            if row[2] != "5a".repeat(s) {
                diff(&mut out, ctx, &format!("T_LOB3 id {id} B is not 5a"));
            }
        }
    }
    out
}

pub fn lob3_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let rel = format!("{v}/lob3_exp_{v}.dmp");
        let want: Vec<Vec<String>> = truth(v, "truth_lob3.txt")
            .into_iter()
            .filter(|l| l[0] == "T_LOB3")
            .collect();
        match read_named(&rel, "T_LOB3", &ReadOptions::default()) {
            Ok((names, r, w)) => {
                if !w.is_empty() {
                    diff(&mut out, &rel, "warnings");
                }
                let pos = |n: &str| names.iter().position(|x| x == n).unwrap();
                let (c, b, nn) = (pos("C"), pos("B"), pos("N"));
                let proj: Rows = by_id(r)
                    .into_iter()
                    .map(|x| vec![x[0].clone(), x[c].clone(), x[b].clone(), x[nn].clone()])
                    .collect();
                out.extend(t_lob3_diff(&rel, &want, &proj));
            }
            Err(e) => diff(&mut out, &rel, &format!("read failed: {:?}", e.code)),
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// T_PART2 (truth_part2.txt)
// ---------------------------------------------------------------------------------------------------------

pub const PART2_FILES: [&str; 2] = ["18c/part2_exp_18c.dmp", "18c/part2_exp_direct_18c.dmp"];

/// `want`: the `SP|partition|subpartition` and `ROW|id|subpartition` lines of `truth_part2.txt`; `parts`: the
/// (partition, subpartition, rows) of the catalog in file order; `batches`: the rows of each batch (one batch
/// per non-empty part, in file order). Every row must be in the subpartition of its part.
pub fn part2_diff(
    ctx: &str,
    want: &[Vec<String>],
    parts: &[(String, String, u64)],
    batches: &[Rows],
) -> Diffs {
    let mut out = Vec::new();
    let sp: Vec<(&str, &str)> = want
        .iter()
        .filter(|l| l[0] == "SP")
        .map(|l| (l[1].as_str(), l[2].as_str()))
        .collect();
    let have: Vec<(&str, &str)> = parts.iter().map(|p| (p.0.as_str(), p.1.as_str())).collect();
    // the SP lines are in the order of the subpartition names, the catalog in file order
    let (mut sp, mut have) = (sp, have);
    sp.sort_unstable();
    have.sort_unstable();
    if sp != have {
        diff(&mut out, ctx, "SP lines against the parts of the catalog");
    }
    let sub_of: BTreeMap<i64, &str> = want
        .iter()
        .filter(|l| l[0] == "ROW")
        .map(|l| (l[1].parse().unwrap(), l[2].as_str()))
        .collect();
    let non_empty: Vec<&(String, String, u64)> = parts.iter().filter(|p| p.2 > 0).collect();
    if non_empty.len() != batches.len() {
        diff(&mut out, ctx, "batches against non-empty parts");
        return out;
    }
    let mut ids = Vec::new();
    for (b, p) in batches.iter().zip(non_empty) {
        if b.len() as u64 != p.2 {
            diff(&mut out, ctx, &format!("rows of part {}", p.1));
        }
        for r in b {
            let id: i64 = r[0].parse().unwrap();
            ids.push(id);
            if sub_of.get(&id).copied() != Some(p.1.as_str()) {
                diff(
                    &mut out,
                    ctx,
                    &format!("id {id} is in another subpartition"),
                );
            }
        }
    }
    ids.sort_unstable();
    let all: Vec<i64> = sub_of.keys().copied().collect();
    if ids != all {
        diff(&mut out, ctx, "ids against the ROW lines");
    }
    out
}

pub fn part2_diffs() -> Diffs {
    let want = truth("18c", "truth_part2.txt");
    let mut out = Vec::new();
    for rel in PART2_FILES {
        let mut d = open(fixtures().join(rel), Options::default()).unwrap();
        let info = d.table(TableRef::ByName("T_PART2")).unwrap().clone();
        let parts: Vec<(String, String, u64)> = info
            .parts
            .iter()
            .map(|p| {
                (
                    p.partition.clone().unwrap_or_default(),
                    p.subpartition.clone().unwrap_or_default(),
                    p.num_rows,
                )
            })
            .collect();
        match try_read(&mut d, "T_PART2", &ReadOptions::default()) {
            Err(e) => diff(&mut out, rel, &format!("read failed: {:?}", e.code)),
            Ok((b, _)) => {
                let batches: Vec<Rows> = b.iter().map(|x| rows(std::slice::from_ref(x))).collect();
                out.extend(part2_diff(rel, &want, &parts, &batches));
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// truth_charsets.txt (b7)
// ---------------------------------------------------------------------------------------------------------

/// Each `NAME=id` line of `want` equals the reader's id-to-name map (`Charset::from_id(id).name()`).
pub fn charsets_diff(folder: &str, want: &[Vec<String>]) -> Diffs {
    let mut out = Vec::new();
    for l in want {
        let Some((name, id)) = l[0].split_once('=') else {
            diff(&mut out, folder, "a line without `=`");
            continue;
        };
        let got = id
            .parse::<u16>()
            .ok()
            .and_then(Charset::from_id)
            .map(Charset::name);
        if got != Some(name) {
            diff(&mut out, folder, &format!("set id {id}"));
        }
    }
    out
}

/// N-8: `truth_env.txt` names the database and the national character set (`NLS_CHARACTERSET=`,
/// `NLS_NCHAR_CHARACTERSET=`); `truth_charsets.txt` of the same folder gives the id of each name; `header_sets`
/// are `(file, database set id, national set id)` of the header of every `_exp_` file of the folder. Each file's
/// ids equal the ids of the two names.
pub fn env_diff(
    folder: &str,
    env: &[Vec<String>],
    charsets: &[Vec<String>],
    header_sets: &[(String, u16, u16)],
) -> Diffs {
    let mut out = Vec::new();
    let value = |key: &str| {
        let mut it = env.iter().filter_map(|l| l[0].strip_prefix(key));
        let v = it.next().map(str::to_string);
        if it.next().is_some() {
            return None;
        }
        v
    };
    let id_of = |name: &Option<String>| -> Option<u16> {
        let name = name.as_deref()?;
        let mut it = charsets
            .iter()
            .filter_map(|l| l[0].split_once('=').filter(|(n, _)| *n == name))
            .map(|(_, id)| id.parse::<u16>().ok());
        let id = it.next()?;
        if it.next().is_some() {
            return None;
        }
        id
    };
    let db = value("NLS_CHARACTERSET=");
    let nat = value("NLS_NCHAR_CHARACTERSET=");
    let (db_id, nat_id) = (id_of(&db), id_of(&nat));
    if db_id.is_none() {
        diff(
            &mut out,
            folder,
            "the database set of truth_env has no id in truth_charsets",
        );
    }
    if nat_id.is_none() {
        diff(
            &mut out,
            folder,
            "the national set of truth_env has no id in truth_charsets",
        );
    }
    if header_sets.is_empty() {
        diff(&mut out, folder, "no file");
    }
    for (f, d, n) in header_sets {
        if Some(*d) != db_id {
            diff(&mut out, f, "database set of the header against truth_env");
        }
        if Some(*n) != nat_id {
            diff(&mut out, f, "national set of the header against truth_env");
        }
    }
    out
}

/// N-8: the `OBJECT|TABLE|<name>` lines of `truth_own_ddl.txt` are the table names of the catalog of
/// `own_exp_<v>` (as sets; each name once).
pub fn own_ddl_diff(folder: &str, ddl: &[Vec<String>], catalog_tables: &[String]) -> Diffs {
    let mut out = Vec::new();
    let mut want: Vec<&str> = ddl
        .iter()
        .filter(|l| l[0] == "OBJECT" && l.get(1).map(String::as_str) == Some("TABLE"))
        .filter_map(|l| l.get(2).map(String::as_str))
        .collect();
    want.sort_unstable();
    let mut got: Vec<&str> = catalog_tables.iter().map(String::as_str).collect();
    got.sort_unstable();
    if want.is_empty() {
        diff(&mut out, folder, "no OBJECT|TABLE line");
    }
    if want != got {
        diff(
            &mut out,
            folder,
            "OBJECT TABLE lines against the catalog tables",
        );
    }
    out
}

pub fn env_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let sets: Vec<(String, u16, u16)> = super::exp_files()
            .into_iter()
            .filter(|f| f.starts_with(&format!("{v}/")))
            .map(|rel| {
                let d = open(fixtures().join(&rel), Options::default()).unwrap();
                let i = d.info();
                (rel, i.db_charset_id, i.national_charset_id)
            })
            .collect();
        out.extend(env_diff(
            v,
            &truth(v, "truth_env.txt"),
            &truth(v, "truth_charsets.txt"),
            &sets,
        ));
    }
    out
}

pub fn own_ddl_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let rel = format!("{v}/own_exp_{v}.dmp");
        let mut d = open(fixtures().join(&rel), Options::default()).unwrap();
        let names: Vec<String> = d.tables().unwrap().iter().map(|t| t.name.clone()).collect();
        out.extend(own_ddl_diff(v, &truth(v, "truth_own_ddl.txt"), &names));
    }
    out
}

/// 171 is CL8MSWIN1251: `truth_ru.txt` of `11g-cp1251` has `ENV|NLS_CHARSET_ID(CL8MSWIN1251)|171` and the
/// header of every `11g-cp1251` file carries database set 171 [X3, H4].
pub fn charsets_cp1251_diff(ru: &[Vec<String>], header_sets: &[(String, u16)]) -> Diffs {
    let mut out = Vec::new();
    let id = ru
        .iter()
        .find(|l| l[0] == "ENV" && l[1] == "NLS_CHARSET_ID(CL8MSWIN1251)")
        .map(|l| l[2].clone());
    if id.as_deref() != Some("171") {
        diff(&mut out, "truth_ru", "the ENV line of CL8MSWIN1251");
    }
    if Charset::from_id(171).map(Charset::name) != Some("CL8MSWIN1251") {
        diff(&mut out, "reader", "id 171");
    }
    for (f, set) in header_sets {
        if *set != 171 {
            diff(&mut out, f, "database set of the header");
        }
    }
    out
}

pub fn charsets_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let lines = truth(v, "truth_charsets.txt");
        if lines.len() != 4 {
            diff(&mut out, v, "line count");
        }
        out.extend(charsets_diff(v, &lines));
    }
    let mut sets = Vec::new();
    for rel in super::exp_files()
        .into_iter()
        .filter(|f| f.starts_with("11g-cp1251/"))
    {
        let d = open(fixtures().join(&rel), Options::default()).unwrap();
        sets.push((rel, d.info().db_charset_id));
    }
    if sets.len() != 27 {
        diff(&mut out, "11g-cp1251", "file count");
    }
    out.extend(charsets_cp1251_diff(
        &truth("11g-cp1251", "truth_ru.txt"),
        &sets,
    ));
    out
}

// ---------------------------------------------------------------------------------------------------------
// T-19: T_LONG3 (truth_long3.txt, R-040) and T_TSP (truth_tsp.txt, R-047)
// ---------------------------------------------------------------------------------------------------------

/// Upper-case hex, the form of `truth_long3.txt`.
fn hex_upper(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02X}")).collect()
}

/// T_LONG3 (ID, L LONG, V) against the lines of `truth_long3.txt` (`id|length|first 20 bytes|last 20 bytes|V`):
/// the length, the first and the last 20 bytes of L and the text of V from the truth, and the whole L from R-040's
/// arithmetic (Analyst session 17, `long3_check.py`): `L` + the length as six digits + `y` x (length - 7). The
/// text is ASCII, so the cell text is the bytes. `rows` are in file order, which is the order of the IDs.
pub fn long3_diff(ctx: &str, t: &[Vec<String>], rows: &Rows) -> Diffs {
    let mut out = Vec::new();
    if rows.len() != t.len() {
        diff(&mut out, ctx, "T_LONG3 row count");
        return out;
    }
    for (g, l) in rows.iter().zip(t) {
        let id = &l[0];
        let size: usize = l[1].parse().unwrap();
        if g[0] != *id {
            diff(&mut out, ctx, &format!("id {id} order"));
            continue;
        }
        let b = g[1].as_bytes();
        if b.len() != size {
            diff(&mut out, ctx, &format!("id {id} length"));
            continue;
        }
        if hex_upper(&b[..20]) != l[2] {
            diff(&mut out, ctx, &format!("id {id} first 20 bytes"));
        }
        if hex_upper(&b[b.len() - 20..]) != l[3] {
            diff(&mut out, ctx, &format!("id {id} last 20 bytes"));
        }
        let whole = format!("L{size:06}{}", "y".repeat(size - 7));
        if g[1] != whole {
            diff(&mut out, ctx, &format!("id {id} the whole value"));
        }
        if g[2] != l[4] {
            diff(&mut out, ctx, &format!("id {id} V"));
        }
    }
    out
}

/// The four files of `long3_exp(_direct)_<v>.dmp`, T_LONG3 against `truth_long3.txt` of the folder; no warning.
pub fn long3_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let want = truth(v, "truth_long3.txt");
        for kind in ["long3_exp", "long3_exp_direct"] {
            let rel = format!("{v}/{kind}_{v}.dmp");
            match read_named(&rel, "T_LONG3", &ReadOptions::default()) {
                Err(e) => diff(&mut out, &rel, &format!("read failed: {:?}", e.code)),
                Ok((_, r, w)) => {
                    if !w.is_empty() {
                        diff(&mut out, &rel, &format!("{} warnings", w.len()));
                    }
                    out.extend(long3_diff(&rel, &want, &r));
                }
            }
        }
    }
    out
}

/// The decimal bytes of `Typ=180 Len=11: 120,120,1,...`; `None` for `NULL`.
pub fn dump_decimal(f: &str) -> Option<Vec<u8>> {
    if f.trim() == "NULL" {
        return None;
    }
    let (_, list) = f.split_once(": ")?;
    Some(list.split(',').map(|n| n.parse().unwrap()).collect())
}

/// The seconds since 1970-01-01 and the nanoseconds of the bytes of a TIMESTAMP [Y2], [Y5]: century and year
/// stored plus 100, month, day, hour, minute and second stored plus 1 (hour to second), then four bytes
/// big-endian for the nanoseconds when the value has 11 bytes (7 bytes: fraction 0). Written from the rules, not
/// from the reader.
pub fn timestamp_parts(b: &[u8]) -> (i64, u32) {
    assert!(b.len() == 7 || b.len() == 11, "{} bytes", b.len());
    let year = (i64::from(b[0]) - 100) * 100 + (i64::from(b[1]) - 100);
    let days = days_from_civil(year, u32::from(b[2]), u32::from(b[3]));
    let secs = days * 86_400
        + (i64::from(b[4]) - 1) * 3600
        + (i64::from(b[5]) - 1) * 60
        + (i64::from(b[6]) - 1);
    let ns = if b.len() == 11 {
        u32::from_be_bytes([b[7], b[8], b[9], b[10]])
    } else {
        0
    };
    (secs, ns)
}

/// The integer a TIMESTAMP(f) cell shows: microseconds for f <= 6, nanoseconds above (arch 5.1); the fraction
/// below the unit is dropped (truncated).
pub fn timestamp_cell(secs: i64, ns: u32, f: u8) -> String {
    if f <= 6 {
        (secs * 1_000_000 + i64::from(ns) / 1000).to_string()
    } else {
        (secs * 1_000_000_000 + i64::from(ns)).to_string()
    }
}

/// `YYYY-MM-DD HH:MM:SS.fffffffff` (the `TO_CHAR ... FF9` lines of `truth_tsp.txt`) to seconds and nanoseconds.
fn fractional_text(s: &str) -> (i64, u32) {
    let (date, time) = s.split_once(' ').unwrap();
    let d: Vec<i64> = date.split('-').map(|x| x.parse().unwrap()).collect();
    let (hms, frac) = time.split_once('.').unwrap();
    let t: Vec<i64> = hms.split(':').map(|x| x.parse().unwrap()).collect();
    assert_eq!(frac.len(), 9);
    let secs =
        days_from_civil(d[0], d[1] as u32, d[2] as u32) * 86_400 + t[0] * 3600 + t[1] * 60 + t[2];
    (secs, frac.parse().unwrap())
}

/// The columns of `truth_tsp.txt`: `(name, data_type text, data_length, precision)` of the lines that follow
/// `ID|NUMBER|22|0`, and the precision of the TIMESTAMP columns.
pub fn tsp_columns(t: &[Vec<String>]) -> Vec<(String, String, u16, u8)> {
    t.iter()
        .filter(|l| l.len() == 4 && !l[0].chars().all(|c| c.is_ascii_digit()))
        .map(|l| {
            (
                l[0].clone(),
                l[1].clone(),
                l[2].parse().unwrap(),
                l[3].parse().unwrap(),
            )
        })
        .collect()
}

/// T_TSP (ID, T1..T5, T7, T8) of `tsp_exp(_direct)_<v>.dmp` against `truth_tsp.txt`: every value of the four rows
/// equals the decimal `DUMP` of the truth decoded by [`timestamp_parts`] (row 4 NULL), and the `TO_CHAR ... FF9`
/// lines of rows 1 to 3 (columns T1, T3, T8) equal it truncated to the unit of the column.
pub fn tsp_diff(ctx: &str, t: &[Vec<String>], rows: &Rows) -> Diffs {
    let mut out = Vec::new();
    let cols = tsp_columns(t);
    let precision: Vec<u8> = cols.iter().skip(1).map(|c| c.3).collect();
    let dumps: Vec<&Vec<String>> = t.iter().filter(|l| l.len() == 8).collect();
    if rows.len() != 4 || dumps.len() != 4 || precision.len() != 7 {
        diff(&mut out, ctx, "T_TSP shape");
        return out;
    }
    for (r, (g, l)) in rows.iter().zip(&dumps).enumerate() {
        if g[0] != l[0] {
            diff(&mut out, ctx, &format!("row {} ID", r + 1));
            continue;
        }
        for (c, f) in precision.iter().enumerate() {
            let want = match dump_decimal(&l[c + 1]) {
                None => "<NULL>".to_string(),
                Some(b) => {
                    let (secs, ns) = timestamp_parts(&b);
                    timestamp_cell(secs, ns, *f)
                }
            };
            if g[c + 1] != want {
                diff(&mut out, ctx, &format!("row {} T{f}", r + 1));
            }
        }
    }
    // the TO_CHAR lines: `id|t1|t3|t8` (4 fields, the first all digits); T1 is column 1, T3 column 3, T8 column 7
    for l in t
        .iter()
        .filter(|l| l.len() == 4 && l[0].chars().all(|c| c.is_ascii_digit()))
    {
        let r: usize = l[0].parse().unwrap();
        for (text, c) in l[1..].iter().zip([1usize, 3, 7]) {
            let want = if text.is_empty() {
                "<NULL>".to_string()
            } else {
                let (secs, ns) = fractional_text(text);
                timestamp_cell(secs, ns, precision[c - 1])
            };
            if rows[r - 1][c] != want {
                diff(
                    &mut out,
                    ctx,
                    &format!("row {r} TO_CHAR of T{}", precision[c - 1]),
                );
            }
        }
    }
    out
}

/// The four files `tsp_exp(_direct)_<v>.dmp`: T_TSP against `truth_tsp.txt` of the folder, no warning.
pub fn tsp_diffs() -> Diffs {
    let mut out = Vec::new();
    for v in ["18c", "11g"] {
        let want = truth(v, "truth_tsp.txt");
        for kind in ["tsp_exp", "tsp_exp_direct"] {
            let rel = format!("{v}/{kind}_{v}.dmp");
            match read_named(&rel, "T_TSP", &ReadOptions::default()) {
                Err(e) => diff(&mut out, &rel, &format!("read failed: {:?}", e.code)),
                Ok((_, r, w)) => {
                    if !w.is_empty() {
                        diff(&mut out, &rel, &format!("{} warnings", w.len()));
                    }
                    out.extend(tsp_diff(&rel, &want, &r));
                }
            }
        }
    }
    out
}

/// `truth_seg.txt` of `18c`: `(table, segment_created, segments, rows)`; the three tables read with 0 rows.
pub fn seg_diffs() -> Diffs {
    let mut out = Vec::new();
    let t = truth("18c", "truth_seg.txt");
    if t.len() != 3 {
        diff(&mut out, "truth_seg", "line count");
    }
    for kind in ["seg_exp", "seg_exp_direct"] {
        let rel = format!("18c/{kind}_18c.dmp");
        for l in &t {
            let seg = l[1].strip_prefix("segment_created=").unwrap_or("?");
            let want_rows = l[3].strip_prefix("rows=").unwrap_or("?");
            match read_named(&rel, &l[0], &ReadOptions::default()) {
                Err(e) => diff(
                    &mut out,
                    &rel,
                    &format!("{} read failed: {:?}", l[0], e.code),
                ),
                Ok((_, r, w)) => {
                    if want_rows != "0"
                        || !r.is_empty()
                        || !w.is_empty()
                        || !["YES", "NO"].contains(&seg)
                    {
                        diff(&mut out, &rel, &format!("{} rows or warnings", l[0]));
                    }
                }
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------------------------
// T-19b: T_LONG4, T_LONG5, T_LONGRAW4, T_LONGRAW5 (R-048, R-051) and T_TSLTZ2 (R-050)
// ---------------------------------------------------------------------------------------------------------

/// The reads of one table in the files `<v>/<stem>_<v>.dmp` for the stems and versions given, with the default
/// options: `check` gets the file name, the truth of the folder and the rows; a read error or a warning is a
/// difference.
fn family_diffs(
    table: &str,
    truth_name: &str,
    versions: &[&str],
    stems: &[&str],
    check: fn(&str, &[Vec<String>], &Rows) -> Diffs,
) -> Diffs {
    let mut out = Vec::new();
    for v in versions {
        let want = truth(v, truth_name);
        for kind in stems {
            let rel = format!("{v}/{kind}_{v}.dmp");
            match read_named(&rel, table, &ReadOptions::default()) {
                Err(e) => diff(&mut out, &rel, &format!("read failed: {:?}", e.code)),
                Ok((_, r, w)) => {
                    // the table read raises no warning (the `W-TSLTZ` of the three columns stand in the catalog)
                    if !w.is_empty() {
                        diff(&mut out, &rel, &format!("{} warnings", w.len()));
                    }
                    out.extend(check(&rel, &want, &r));
                }
            }
        }
    }
    out
}

/// T_LONG4 / T_LONG5 (ID, L LONG, V) against `truth_long4.txt` / `truth_long5.txt`: the same lines and the same
/// arithmetic as T_LONG3 (R-040; Analyst sessions 19 and 20, `long4_check.py`, `long5_check.py`).
pub fn long4_diffs() -> Diffs {
    family_diffs(
        "T_LONG4",
        "truth_long4.txt",
        &["18c", "11g"],
        &["long4_exp", "long4_exp_direct"],
        long3_diff,
    )
}

/// See [`long4_diffs`]: `truth_long5.txt`, sizes 65,526 to 65,531.
pub fn long5_diffs() -> Diffs {
    family_diffs(
        "T_LONG5",
        "truth_long5.txt",
        &["18c", "11g"],
        &["long5_exp", "long5_exp_direct"],
        long3_diff,
    )
}

/// T_LONGRAW4 / T_LONGRAW5 (ID, LR LONG RAW) against `id|length|first 10 bytes|last 10 bytes` (upper-case hex): the
/// length and both ends from the truth, the whole value from R-048's arithmetic (`length` bytes `ab`). The cell
/// text of a binary value is lower-case hex.
pub fn longraw_diff(ctx: &str, t: &[Vec<String>], rows: &Rows) -> Diffs {
    let mut out = Vec::new();
    if rows.len() != t.len() {
        diff(&mut out, ctx, "LONG RAW row count");
        return out;
    }
    for (g, l) in rows.iter().zip(t) {
        let id = &l[0];
        let size: usize = l[1].parse().unwrap();
        if g[0] != *id {
            diff(&mut out, ctx, &format!("id {id} order"));
            continue;
        }
        if g[1].len() != size * 2 {
            diff(&mut out, ctx, &format!("id {id} length"));
            continue;
        }
        if g[1][..20].to_uppercase() != l[2] {
            diff(&mut out, ctx, &format!("id {id} first 10 bytes"));
        }
        if g[1][g[1].len() - 20..].to_uppercase() != l[3] {
            diff(&mut out, ctx, &format!("id {id} last 10 bytes"));
        }
        if g[1] != "ab".repeat(size) {
            diff(&mut out, ctx, &format!("id {id} the whole value"));
        }
    }
    out
}

/// The two 18c files of T_LONGRAW4 against `18c/truth_longraw4.txt` (R-048 ran LONG RAW on 18c only).
pub fn longraw4_diffs() -> Diffs {
    family_diffs(
        "T_LONGRAW4",
        "truth_longraw4.txt",
        &["18c"],
        &["longraw4_exp", "longraw4_exp_direct"],
        longraw_diff,
    )
}

/// The four files of T_LONGRAW5 (18c 7 rows, 11g 11 rows) against `truth_longraw5.txt` of each folder (R-051).
pub fn longraw5_diffs() -> Diffs {
    family_diffs(
        "T_LONGRAW5",
        "truth_longraw5.txt",
        &["18c", "11g"],
        &["longraw5_exp", "longraw5_exp_direct"],
        longraw_diff,
    )
}

/// The precisions of the TSLTZ columns of `truth_tsltz2.txt`: the lines `name|type|data_length|precision` after
/// `ID|NUMBER|22|0` (the first line, `+00:00|+00:00`, is the zone line).
pub fn tsltz2_columns(t: &[Vec<String>]) -> Vec<(String, u16, u8)> {
    t.iter()
        .filter(|l| l.len() == 4 && l[1].starts_with("TIMESTAMP("))
        .map(|l| (l[0].clone(), l[2].parse().unwrap(), l[3].parse().unwrap()))
        .collect()
}

/// T_TSLTZ2 (ID, L0, L3, L9) of `tsltz2_exp(_direct)_<v>.dmp` against `truth_tsltz2.txt`: every value of the four
/// rows equals the decimal `DUMP` bytes of the truth (`Typ=231`, 7 or 11 bytes) decoded by [`timestamp_parts`]
/// (row 4 NULL), and the `TO_CHAR ... FF9` lines equal it truncated to the unit of the column (microseconds for L0
/// and L3, nanoseconds for L9, arch 5.1).
pub fn tsltz2_diff(ctx: &str, t: &[Vec<String>], rows: &Rows) -> Diffs {
    let mut out = Vec::new();
    let cols = tsltz2_columns(t);
    let dumps: Vec<&Vec<String>> = t
        .iter()
        .filter(|l| l.len() == 4 && (l[1].starts_with("Typ=231") || l[1] == "NULL"))
        .collect();
    if rows.len() != 4 || dumps.len() != 4 || cols.len() != 3 {
        diff(&mut out, ctx, "T_TSLTZ2 shape");
        return out;
    }
    for (r, (g, l)) in rows.iter().zip(&dumps).enumerate() {
        if g[0] != l[0] {
            diff(&mut out, ctx, &format!("row {} ID", r + 1));
            continue;
        }
        for (c, (name, len, f)) in cols.iter().enumerate() {
            let want = match dump_decimal(&l[c + 1]) {
                None => "<NULL>".to_string(),
                Some(b) => {
                    // the length in the DUMP is the stored length: 7 when the fraction is zero, else 11 [Y8]
                    assert!(b.len() == 7 || b.len() == *len as usize, "{ctx} {name}");
                    let (secs, ns) = timestamp_parts(&b);
                    timestamp_cell(secs, ns, *f)
                }
            };
            if g[c + 1] != want {
                diff(&mut out, ctx, &format!("row {} {name}", r + 1));
            }
        }
    }
    // the TO_CHAR lines: `id|l0|l3|l9`, four fields, no `Typ=`, no `NULL`
    for l in t.iter().filter(|l| {
        l.len() == 4
            && l[0].chars().all(|c| c.is_ascii_digit())
            && !l[1].starts_with("Typ=")
            && l[1] != "NULL"
    }) {
        let r: usize = l[0].parse().unwrap();
        for (c, text) in l[1..].iter().enumerate() {
            let want = if text.is_empty() {
                "<NULL>".to_string()
            } else {
                let (secs, ns) = fractional_text(text);
                timestamp_cell(secs, ns, cols[c].2)
            };
            if rows[r - 1][c + 1] != want {
                diff(&mut out, ctx, &format!("row {r} TO_CHAR of {}", cols[c].0));
            }
        }
    }
    out
}

/// The four files `tsltz2_exp(_direct)_<v>.dmp`: T_TSLTZ2 against `truth_tsltz2.txt` of the folder.
pub fn tsltz2_diffs() -> Diffs {
    family_diffs(
        "T_TSLTZ2",
        "truth_tsltz2.txt",
        &["18c", "11g"],
        &["tsltz2_exp", "tsltz2_exp_direct"],
        tsltz2_diff,
    )
}
