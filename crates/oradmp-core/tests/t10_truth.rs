//! Task T-10 and T-10b: the value-level truth check over all 229 `*_exp_*.dmp` fixtures (architecture 8.2 (b)).
//!
//! Every table section of every file is read through `Dump::read_table` and handed to a checker that compares
//! the decoded values with the `truth_*.txt` of the file's folder (parsed directly, never through our own
//! decoders) or, where the truth gives only a prefix or a length, with the generating SQL of `corpus/sql/`.
//! A table whose checker lives in an earlier task's test is *delegated*: the registry names the test function
//! and `delegated_tests_exist` fails if that function is gone. A table with no entry in either list fails
//! `every_table_of_the_exp_files_is_classified_and_read`, so no file is skipped silently.
//!
//! T-10b (ADR-38): a truth file counts as compared only when a check *opened* it: `common::truth_text` is the
//! one function that opens a `truth_*.txt` and records `(folder, name)`. The truth comparison of every
//! delegated family is a function of `common/truth_checks.rs` that returns the list of differences; the
//! delegated test calls it and asserts the list is empty, and `every_truth_file_is_read_or_listed` calls every
//! checker of this file and every such function before it compares the recorded set (plus the explicit list,
//! with reasons) with the truth lines of the three `SHA256SUMS`. A source literal no longer counts. Each new or
//! changed checker has a break test: one line of the truth changed in memory gives exactly one difference.
//!
//! Numbers: the file and table counts (229 files, 436 sections; T-3d, T-19b) are those of the plan
//! (`common::TOTALS`: `tools/corpus_counts.py`, `problems 0`); the section count and the per-checker instance
//! counts (`expected_counts`) are derived from `tests/golden/exp_catalog.tsv` (the Analyst's `exp_parse.py`
//! through `gen_exp_catalog.py`) with the keying of `delegated()`, never from the reader. The expected value
//! errors are those of ADR-28 and the plan (T_DATE rows 8 and 9, T_TS row 5, T_INTERVAL2 IYM rows 2 and 3, the
//! T_NUM rows whose integer part does not fit Decimal128(38, 10)); the rows that `on_number_overflow` changes
//! are computed from the truth text by decimal string arithmetic in this file.
//!
//! T-11b (ADR-43 item 5, ADR-44 item 5): the checkers of this file assert nothing. Each comparison notes a
//! difference in the `Ctx` (`ceq!` / `cok!`), `check_table` returns the list, and `union_of` is the one
//! function that joins the lists; `every_table_of_the_exp_files_is_classified_and_read` asserts it empty and
//! `break_every_tag_of_check_table_...` expects it not empty for every tag when one reader cell is changed in
//! memory. `truth_env.txt` and `truth_own_ddl.txt` have checks (`env_diffs`, `own_ddl_diffs`).
//!
//! Truth files that no check of this binary opens are in `TRUTH_NOT_OPENED` with a reason: `truth_float.txt`
//! (T_FLOAT exists only in a Data Pump file), `truth_decl.txt` (T-5, `t5_types.rs`) and `truth_r045.txt`
//! (R-045, an SQL*Plus spool: ORA-14085 for `t_plong`, `t_plongraw` and `ALTER TABLE t_plong2 ADD (l LONG)`,
//! `Table created.` / `Table dropped.` for the rest; it describes no EXP table).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use common::truth_checks::{by_id, golden_sections};
use common::*;
use oradmp_core::dump::{CatalogEnd, Dump, TableRef};
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::values::number::OverflowPolicy;
use oradmp_core::{Error, ErrorCode, Warning, WarningCode};

const FOLDERS: [&str; 3] = ["18c", "11g", "11g-cp1251"];

fn exp_files() -> Vec<String> {
    let mut v = Vec::new();
    for f in FOLDERS {
        for e in std::fs::read_dir(fixtures().join(f)).unwrap() {
            let name = e.unwrap().file_name().to_string_lossy().into_owned();
            if name.contains("_exp_") && name.ends_with(".dmp") {
                v.push(format!("{f}/{name}"));
            }
        }
    }
    v.sort();
    v
}

type Rows = Vec<Vec<String>>;

fn rd(d: &mut Dump, table: &str, o: &ReadOptions) -> Result<(Rows, Vec<Warning>), Error> {
    let (b, w) = try_read(d, table, o)?;
    Ok((rows(&b), w))
}

/// Like `rd`, with the rows ordered by the numeric ID in column 0 (the truth is ordered by ID, the file by
/// insertion; rule R6 compares by id).
fn rds(d: &mut Dump, table: &str, o: &ReadOptions) -> Result<(Rows, Vec<Warning>), Error> {
    let (mut r, w) = rd(d, table, o)?;
    r.sort_by_key(|x| x[0].parse::<i64>().unwrap_or(i64::MIN));
    Ok((r, w))
}

fn lenient() -> ReadOptions {
    ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    }
}

fn count_of(w: &[Warning], code: WarningCode) -> Vec<&Warning> {
    w.iter().filter(|w| w.code == code).collect()
}

// ---------------------------------------------------------------------------------------------------------
// small parsers of the truth texts
// ---------------------------------------------------------------------------------------------------------

/// The bytes of `Typ=1 Len=3 [CharacterSet=X]: 61,62,63`; `None` for `NULL`.
fn dump_bytes(f: &str) -> Option<Vec<u8>> {
    if f.trim() == "NULL" {
        return None;
    }
    let (_, list) = f.split_once(": ").unwrap();
    Some(
        list.split(',')
            .map(|h| u8::from_str_radix(h, 16).unwrap())
            .collect(),
    )
}

fn utf8(b: &[u8]) -> String {
    String::from_utf8(b.to_vec()).unwrap()
}

fn utf16be(b: &[u8]) -> String {
    let u: Vec<u16> = b
        .chunks(2)
        .map(|p| u16::from_be_bytes([p[0], p[1]]))
        .collect();
    String::from_utf16(&u).unwrap()
}

fn opt(s: Option<String>) -> String {
    s.unwrap_or_else(|| "<NULL>".to_string())
}

/// A truth decimal (`.5`, `-.5`, `1E+125`, `1E-130`) as plain text in the form of `common::decimal_text`.
fn plain(t: &str) -> String {
    let (neg, rest) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t),
    };
    let (mant, exp) = match rest.split_once('E') {
        Some((m, e)) => (m, e.parse::<i64>().unwrap()),
        None => (rest, 0),
    };
    let (ip, fp) = mant.split_once('.').unwrap_or((mant, ""));
    let digits = format!("{ip}{fp}");
    let pt = ip.len() as i64 + exp;
    let s = if pt <= 0 {
        format!("0.{}{digits}", "0".repeat((-pt) as usize))
    } else if pt as usize >= digits.len() {
        format!("{digits}{}", "0".repeat(pt as usize - digits.len()))
    } else {
        format!("{}.{}", &digits[..pt as usize], &digits[pt as usize..])
    };
    let (i, f) = s.split_once('.').unwrap_or((&s, ""));
    let i = i.trim_start_matches('0');
    let i = if i.is_empty() { "0" } else { i };
    let f = f.trim_end_matches('0');
    let body = if f.is_empty() {
        i.to_string()
    } else {
        format!("{i}.{f}")
    };
    if neg && body != "0" {
        format!("-{body}")
    } else {
        body
    }
}

/// What Decimal128(38, 10) makes of a truth decimal under `round`.
#[derive(Debug, PartialEq, Eq)]
enum Fit {
    Exact(String),
    /// rounded text; the value was not zero and became zero
    Rounded(String, bool),
    /// the integer part does not fit (more than 28 digits): a value error
    Overflow,
}

/// Rounds the plain decimal `t` half away from zero to `scale` fractional digits.
fn round_plain(t: &str, scale: usize) -> String {
    let (neg, rest) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t),
    };
    let (ip, fp) = rest.split_once('.').unwrap_or((rest, ""));
    let mut digits: Vec<u8> = ip
        .bytes()
        .chain(fp.bytes().take(scale))
        .map(|c| c - b'0')
        .collect();
    digits.extend(std::iter::repeat_n(0, scale.saturating_sub(fp.len())));
    if fp.as_bytes().get(scale).is_some_and(|&c| c >= b'5') {
        let mut i = digits.len();
        loop {
            if i == 0 {
                digits.insert(0, 1);
                break;
            }
            i -= 1;
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                break;
            }
        }
    }
    let s: String = digits.iter().map(|d| char::from(b'0' + d)).collect();
    decimal_text(&format!("{}{s}", if neg { "-" } else { "" }), scale)
}

fn fit_10(truth: &str) -> Fit {
    let p = plain(truth);
    let body = p.trim_start_matches('-');
    let (ip, fp) = body.split_once('.').unwrap_or((body, ""));
    let int_digits = if ip == "0" { 0 } else { ip.len() };
    if int_digits > 28 {
        return Fit::Overflow;
    }
    if fp.len() <= 10 {
        return Fit::Exact(p);
    }
    let r = round_plain(&p, 10);
    let to_zero = r == "0" && p != "0";
    Fit::Rounded(r, to_zero)
}

/// `YYYY-MM-DD HH:MM:SS[.fff]` to (seconds since the epoch, nanoseconds).
fn parse_ts(t: &str) -> (i64, i64) {
    let (date, time) = t.split_once(' ').unwrap();
    let (hms, frac) = time.split_once('.').unwrap_or((time, ""));
    let d: Vec<i64> = date.split('-').map(|x| x.parse().unwrap()).collect();
    let h: Vec<i64> = hms.split(':').map(|x| x.parse().unwrap()).collect();
    let days = days_from_civil(d[0], d[1] as u32, d[2] as u32);
    let nanos = if frac.is_empty() {
        0
    } else {
        format!("{frac:0<9}").parse().unwrap()
    };
    (days * 86_400 + h[0] * 3600 + h[1] * 60 + h[2], nanos)
}

/// Microseconds (or nanoseconds, `unit` = 1) since the epoch of a truth timestamp text, as a decimal string.
fn ts_units(t: &str, unit: i64) -> String {
    let (secs, nanos) = parse_ts(t);
    ((i128::from(secs) * 1_000_000_000 + i128::from(nanos)) / i128::from(unit)).to_string()
}

fn is_null(t: &str) -> bool {
    t == "<NULL>" || t == "NULL"
}

fn unwrap_null(t: &str) -> String {
    if is_null(t) {
        "<NULL>".to_string()
    } else {
        t.to_string()
    }
}

// ---------------------------------------------------------------------------------------------------------
// the checkers; each reads the table itself (the options differ) and returns the warnings of its main read
// ---------------------------------------------------------------------------------------------------------

/// What one checker run is given and what it collects. The checkers never assert: every comparison goes
/// through `ceq!` / `cok!` (or `Ctx::merge`), which note a difference here; `check_table` hands the list to its
/// caller, and the one union function (`union_of`) is asserted empty (T-11b, ADR-43 item 5). `edit` is the
/// hook of the break test: the cell (row, column) of every successful read is replaced in memory.
struct Ctx<'a> {
    folder: &'a str,
    file: &'a str,
    diffs: RefCell<Vec<String>>,
    edit: Option<(usize, usize)>,
}

impl<'a> Ctx<'a> {
    fn new(folder: &'a str, file: &'a str) -> Self {
        Ctx {
            folder,
            file,
            diffs: RefCell::new(Vec::new()),
            edit: None,
        }
    }

    fn note(&self, msg: String) {
        self.diffs.borrow_mut().push(msg);
    }

    fn merge(&self, d: Vec<String>) {
        self.diffs.borrow_mut().extend(d);
    }

    fn take(&self) -> Vec<String> {
        std::mem::take(&mut *self.diffs.borrow_mut())
    }

    fn apply_edit(&self, mut r: Rows) -> Rows {
        if let Some((row, col)) = self.edit {
            // an empty table gets one invented row (the reader "delivers" a row it must not have)
            if r.is_empty() {
                r.push(vec!["BREAK".to_string()]);
            } else if let Some(cell) = r.get_mut(row).and_then(|x| x.get_mut(col)) {
                cell.push_str("_BREAK");
            }
        }
        r
    }

    fn rd(
        &self,
        d: &mut Dump,
        table: &str,
        o: &ReadOptions,
    ) -> Result<(Rows, Vec<Warning>), Error> {
        rd(d, table, o).map(|(r, w)| (self.apply_edit(r), w))
    }

    fn rds(
        &self,
        d: &mut Dump,
        table: &str,
        o: &ReadOptions,
    ) -> Result<(Rows, Vec<Warning>), Error> {
        rds(d, table, o).map(|(r, w)| (self.apply_edit(r), w))
    }
}

fn clip(s: String) -> String {
    s.chars().take(160).collect()
}

macro_rules! ceq {
    ($c:expr, $a:expr, $b:expr) => {{
        let (a, b) = (&$a, &$b);
        if a != b {
            $c.note(clip(format!("line {}: {:?} != {:?}", line!(), a, b)));
        }
    }};
    ($c:expr, $a:expr, $b:expr, $($arg:tt)+) => {{
        let (a, b) = (&$a, &$b);
        if a != b {
            $c.note(clip(format!("line {}: {} ({:?} != {:?})", line!(), format_args!($($arg)+), a, b)));
        }
    }};
}

macro_rules! cok {
    ($c:expr, $cond:expr) => {{
        if !$cond {
            $c.note(format!("line {}: {}", line!(), stringify!($cond)));
        }
    }};
    ($c:expr, $cond:expr, $($arg:tt)+) => {{
        if !$cond {
            $c.note(clip(format!("line {}: {}", line!(), format_args!($($arg)+))));
        }
    }};
}

fn t(c: &Ctx, name: &str) -> Vec<Vec<String>> {
    truth(c.folder, name)
}

fn check_frame(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rd(d, "T_FRAME", &ReadOptions::default()).unwrap();
    let want: Rows = t(c, "truth_frame.txt")
        .into_iter()
        .map(|l| l.iter().map(|x| unwrap_null(x)).collect())
        .collect();
    ceq!(c, want.len(), 7);
    ceq!(c, r, want, "{}", c.file);
    w
}

/// The T_LEN value of `corpus/sql/01_corpus_tables.sql`: `RPAD(SUBSTR('L' || n || '_', 1, n), n, 'x')`.
fn len_rule(n: usize) -> String {
    let mut s: String = format!("L{n}_").chars().take(n).collect();
    while s.chars().count() < n {
        s.push('x');
    }
    s
}

/// T_LEN (ID, S) against `truth_len.txt` (`id|chars|DUMP of the first 12 bytes`) and the SQL rule: the whole
/// value is the rule's string (S-5), its number of characters is the second truth field, the DUMP bytes are
/// its beginning and the `Len=` of the DUMP is its byte length. The ID of the reader's row gives n.
fn len_diff(file: &str, tr: &[Vec<String>], r: &[Vec<String>]) -> Vec<String> {
    let mut out = Vec::new();
    if r.len() != tr.len() {
        out.push(format!("{file}: row count"));
        return out;
    }
    for (row, l) in r.iter().zip(tr) {
        let id = &row[0];
        if *id != l[0] {
            out.push(format!("{file}: id order at {id}"));
            continue;
        }
        match dump_bytes(&l[2]) {
            None => {
                if row[1] != "<NULL>" || l[1] != "<NULL>" {
                    out.push(format!("{file}: id {id} should be NULL"));
                }
            }
            Some(b) => {
                let n: usize = id.parse().unwrap();
                // DUMP shows the first 12 bytes only; `Len=` is the whole byte length
                if !row[1].as_bytes().starts_with(&b) {
                    out.push(format!("{file}: id {id} first bytes"));
                }
                if row[1].chars().count().to_string() != l[1] {
                    out.push(format!("{file}: id {id} character count"));
                }
                if !l[2].contains(&format!("Len={}:", row[1].len())) {
                    out.push(format!("{file}: id {id} byte length of the DUMP"));
                }
                if row[1] != len_rule(n) {
                    out.push(format!("{file}: id {id} value against the SQL rule"));
                }
            }
        }
    }
    out
}

fn check_len(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rds(d, "T_LEN", &ReadOptions::default()).unwrap();
    let tr = t(c, "truth_len.txt");
    ceq!(c, tr.len(), 22);
    let diffs = len_diff(c.file, &tr, &r);
    c.merge(diffs);
    w
}

fn check_nullmix(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rds(d, "T_NULLMIX", &ReadOptions::default()).unwrap();
    let want: Rows = t(c, "truth_nullmix.txt")
        .into_iter()
        .map(|l| {
            vec![
                l[0].clone(),
                if is_null(&l[1]) {
                    "<NULL>".into()
                } else {
                    plain(&l[1])
                },
                unwrap_null(&l[2]),
                unwrap_null(&l[3]),
            ]
        })
        .collect();
    ceq!(c, want.len(), 8);
    ceq!(c, r, want, "{}", c.file);
    w
}

/// T_NUM, T_NUM2 and T_NUM3: V is `NUMBER` = Decimal128(38, 10). Every truth value is classified by decimal
/// string arithmetic (exact, rounded, integer part too big); the four policies are run.
fn check_num(c: &Ctx, d: &mut Dump, table: &str, truth_file: &str, nrows: usize) -> Vec<Warning> {
    let tr = t(c, truth_file);
    ceq!(c, tr.len(), nrows);
    let fits: Vec<Option<Fit>> = tr
        .iter()
        .map(|l| (!is_null(&l[1])).then(|| fit_10(&l[1])))
        .collect();
    let overflow_rows: Vec<usize> = fits
        .iter()
        .enumerate()
        .filter(|(_, f)| matches!(f, Some(Fit::Overflow)))
        .map(|(i, _)| i + 1)
        .collect();
    let rounded_rows: Vec<usize> = fits
        .iter()
        .enumerate()
        .filter(|(_, f)| matches!(f, Some(Fit::Rounded(..))))
        .map(|(i, _)| i + 1)
        .collect();
    let to_zero = fits
        .iter()
        .filter(|f| matches!(f, Some(Fit::Rounded(_, true))))
        .count();
    for (i, l) in tr.iter().enumerate() {
        ceq!(c, l[0], (i + 1).to_string(), "truth ids are 1..n");
    }
    let want = |round: bool| -> Rows {
        tr.iter()
            .zip(&fits)
            .map(|(l, f)| {
                let v = match f {
                    None => "<NULL>".to_string(),
                    Some(Fit::Exact(s)) => s.clone(),
                    Some(Fit::Rounded(s, _)) if round => s.clone(),
                    Some(_) => "<NULL>".to_string(),
                };
                vec![l[0].clone(), v]
            })
            .collect()
    };

    // (1) defaults (round, raise): the first integer overflow is the error; without one the read succeeds
    let default = c.rds(d, table, &ReadOptions::default());
    let mut main_warnings = Vec::new();
    match (overflow_rows.first(), default) {
        (None, Ok((r, w))) => {
            ceq!(c, r, want(true), "{} {table}", c.file);
            main_warnings = w;
        }
        (Some(&first), Err(e)) => {
            ceq!(c, e.code, ErrorCode::NumberOverflow, "{} {table}", c.file);
            ceq!(c, e.row, Some(first as u64), "{} {table}", c.file);
            ceq!(c, e.column, Some(2));
        }
        (a, b) => c.note(format!(
            "{} {table}: {a:?} against {:?}",
            c.file,
            b.map(|x| x.0.len())
        )),
    }

    // (2) `error` policy: stops at the first value that is not exact (rounded or too big)
    let first_inexact = overflow_rows.iter().chain(&rounded_rows).min().copied();
    let err = c.rds(
        d,
        table,
        &ReadOptions {
            on_number_overflow: OverflowPolicy::Error,
            ..ReadOptions::default()
        },
    );
    match (first_inexact, err) {
        (None, Ok((r, _))) => ceq!(c, r, want(true)),
        (Some(first), Err(e)) => {
            ceq!(c, e.code, ErrorCode::NumberOverflow);
            ceq!(c, e.row, Some(first as u64), "{} {table}", c.file);
        }
        (a, b) => c.note(format!(
            "{} {table} error policy: {a:?} against {:?}",
            c.file,
            b.map(|x| x.0.len())
        )),
    }

    // (3) round, value errors become NULL: rows equal the computed rounding; counted warnings
    let (r, w) = c.rds(d, table, &lenient()).unwrap();
    ceq!(c, r, want(true), "{} {table} round", c.file);
    let rounded = count_of(&w, WarningCode::NumberRounded);
    if rounded_rows.is_empty() {
        cok!(c, rounded.is_empty());
    } else {
        ceq!(c, rounded.len(), 1);
        ceq!(c, rounded[0].count, Some(rounded_rows.len() as u64));
        ceq!(c, rounded[0].count_to_zero, Some(to_zero as u64));
        ceq!(c, rounded[0].row, Some(rounded_rows[0] as u64));
    }
    let nulled = count_of(&w, WarningCode::ValueNulled);
    if overflow_rows.is_empty() {
        cok!(c, nulled.is_empty());
    } else {
        ceq!(c, nulled.len(), 1);
        ceq!(c, nulled[0].count, Some(overflow_rows.len() as u64));
        ceq!(c, nulled[0].row, Some(overflow_rows[0] as u64));
    }

    // (4) null policy: the rounded values are NULL too (W-NUMBER-NULLED), exact ones stay
    let (r, w) = c
        .rds(
            d,
            table,
            &ReadOptions {
                on_number_overflow: OverflowPolicy::Null,
                ..lenient()
            },
        )
        .unwrap();
    ceq!(c, r, want(false), "{} {table} null", c.file);
    let n = count_of(&w, WarningCode::NumberNulled);
    if rounded_rows.is_empty() {
        cok!(c, n.is_empty());
    } else {
        ceq!(c, n.len(), 1);
        ceq!(c, n[0].count, Some(rounded_rows.len() as u64));
    }
    if overflow_rows.is_empty() {
        main_warnings
    } else {
        // the main read stopped: the warnings of the lenient read are the ones to inventory
        let (_, w) = c.rds(d, table, &lenient()).unwrap();
        w
    }
}

fn check_numtypes(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rds(d, "T_NUMTYPES", &ReadOptions::default()).unwrap();
    let tr = t(c, "truth_numtypes.txt");
    ceq!(c, tr.len(), 5);
    ceq!(c, r.len(), 5);
    for (row, l) in r.iter().zip(&tr) {
        ceq!(c, row[0], l[0]);
        // N_P10 and N_NEG are Int64, N_P10S2 Decimal128(10, 2), N_FL Float64
        for (cell_i, t_i) in [(1usize, 1usize), (2, 2), (3, 3)] {
            if is_null(&l[t_i]) {
                ceq!(c, row[cell_i], "<NULL>");
            } else {
                ceq!(c, row[cell_i], plain(&l[t_i]), "{} id {}", c.file, l[0]);
            }
        }
        if is_null(&l[4]) {
            ceq!(c, row[4], "<NULL>");
        } else {
            let a: f64 = row[4].parse().unwrap();
            let b: f64 = l[4].parse().unwrap();
            ceq!(c, a, b, "{} id {}", c.file, l[0]);
        }
    }
    w
}

fn check_date(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let e = c.rds(d, "T_DATE", &ReadOptions::default()).unwrap_err();
    ceq!(c, e.code, ErrorCode::BadDatetime, "{}", c.file);
    ceq!(c, (e.row, e.column, e.part), (Some(8), Some(2), Some(1)));
    cok!(c, e.offset.is_some());
    let (r, w) = c.rds(d, "T_DATE", &lenient()).unwrap();
    let tr = t(c, "truth_date.txt");
    ceq!(c, tr.len(), 11);
    ceq!(c, r.len(), 11);
    for l in &tr {
        let i: usize = l[0].parse().unwrap();
        let got = &r[i - 1][1];
        match i {
            // years -100 and -4712 (ADR-28): E-BAD-DATETIME, NULL under `null`
            8 | 9 => ceq!(c, got, "<NULL>", "{}", c.file),
            11 => ceq!(c, got, "<NULL>"),
            _ => ceq!(c, got, l[1].trim(), "{} row {i}", c.file),
        }
    }
    let n = count_of(&w, WarningCode::ValueNulled);
    ceq!(c, n.len(), 1);
    ceq!(
        c,
        (n[0].count, n[0].row, n[0].column),
        (Some(2), Some(8), Some(2))
    );
    w
}

fn check_ts(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    // row 5 column 4 (TIMESTAMP(9) 9999-12-31) is outside the 64-bit nanosecond range: E-BAD-DATETIME
    let e = c.rds(d, "T_TS", &ReadOptions::default()).unwrap_err();
    ceq!(
        c,
        (e.code, e.row, e.column),
        (ErrorCode::BadDatetime, Some(5), Some(4)),
        "{}",
        c.file
    );
    let (r, w) = c.rds(d, "T_TS", &lenient()).unwrap();
    let tr = t(c, "truth_ts.txt");
    ceq!(c, tr.len(), 6);
    for l in &tr {
        let i: usize = l[0].parse().unwrap();
        // truth: id|ts0|dump|ts6|dump|ts9|dump
        for (cell_i, t_i, unit) in [(1usize, 1usize, 1_000i64), (2, 3, 1_000), (3, 5, 1)] {
            let want = if is_null(&l[t_i]) || (i == 5 && cell_i == 3) {
                "<NULL>".to_string()
            } else {
                ts_units(&l[t_i], unit)
            };
            ceq!(c, r[i - 1][cell_i], want, "{} row {i} col {cell_i}", c.file);
        }
    }
    let n = count_of(&w, WarningCode::ValueNulled);
    ceq!(c, n.len(), 1);
    ceq!(c, n[0].count, Some(1));
    w
}

fn check_tstz(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rds(d, "T_TSTZ", &ReadOptions::default()).unwrap();
    let tr = t(c, "truth_tstz.txt");
    ceq!(c, tr.len(), 10);
    ceq!(c, r.len(), 10);
    for l in &tr {
        let row = r.iter().find(|x| x[0] == l[0]).unwrap();
        // "tstz=2016-03-26 19:34:55.123456 +05:30 +05:30": instant = local time minus the offset
        match l[1].strip_prefix("tstz=").filter(|t| *t != "<NULL>") {
            Some(v) => {
                let p: Vec<&str> = v.split(' ').collect();
                let (secs, nanos) = parse_ts(&format!("{} {}", p[0], p[1]));
                let sign = if p[2].starts_with('-') { -1 } else { 1 };
                let (oh, om) = p[2][1..].split_once(':').unwrap();
                let off =
                    sign * (oh.parse::<i64>().unwrap() * 3600 + om.parse::<i64>().unwrap() * 60);
                ceq!(
                    c,
                    row[1],
                    ((secs - off) * 1_000_000 + nanos / 1000).to_string(),
                    "{} id {}",
                    c.file,
                    l[0]
                );
            }
            None => ceq!(c, row[1], "<NULL>"),
        }
        // TSLTZ is delivered as stored (W-TSLTZ)
        match l[3].strip_prefix("tsltz=").filter(|t| *t != "<NULL>") {
            Some(v) => {
                let (secs, nanos) = parse_ts(v);
                ceq!(c, row[2], (secs * 1_000_000 + nanos / 1000).to_string());
            }
            None => ceq!(c, row[2], "<NULL>"),
        }
    }
    w
}

/// T_TEXT: V and C are database bytes (the client set does not matter, exp.md 6.3), NV is UTF-16 BE.
fn check_text(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rds(d, "T_TEXT", &ReadOptions::default()).unwrap();
    let tr = t(c, "truth_text.txt");
    ceq!(c, tr.len(), 10);
    ceq!(c, r.len(), 10);
    for (row, l) in r.iter().zip(&tr) {
        ceq!(c, row[0], l[0]);
        // truth: id|v_chars=n|v_bytes=n|DUMP V|DUMP C|DUMP NV
        for (cell_i, t_i, utf16) in [(1usize, 3usize, false), (2, 4, false), (3, 5, true)] {
            let want = dump_bytes(&l[t_i]).map(|b| if utf16 { utf16be(&b) } else { utf8(&b) });
            ceq!(
                c,
                row[cell_i],
                opt(want.clone()),
                "{} id {} col {cell_i}",
                c.file,
                l[0]
            );
            if cell_i == 1 {
                if let Some(v) = want {
                    ceq!(c, format!("v_chars={}", v.chars().count()), l[1]);
                    ceq!(c, format!("v_bytes={}", v.len()), l[2]);
                }
            }
        }
    }
    w
}

fn check_raw(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rd(d, "T_RAW", &ReadOptions::default()).unwrap();
    // truth lines: id|len=n|part=1|HEX; a value above 1000 bytes continues in a `part=2` line
    let mut want: BTreeMap<String, (String, String)> = BTreeMap::new();
    for l in t(c, "truth_raw.txt") {
        let e = want.entry(l[0].clone()).or_default();
        e.0 = l[1].clone();
        if l[3] != "<NULL>" {
            e.1.push_str(&l[3].to_lowercase());
        }
    }
    ceq!(c, want.len(), 7);
    ceq!(c, r.len(), 7);
    for row in &r {
        let (len, hex) = &want[&row[0]];
        if len == "len=<NULL>" {
            ceq!(c, row[1], "<NULL>");
        } else {
            ceq!(c, &row[1], hex, "{} id {}", c.file, row[0]);
            ceq!(c, format!("len={}", row[1].len() / 2), *len);
        }
    }
    w
}

/// T_WIDE (120 columns) against `truth_wide.txt`: the spool pads every field to the column width (8), a NULL is
/// shown as `<NULL>`.
fn wide_diff(file: &str, tr: &[Vec<String>], r: &[Vec<String>]) -> Vec<String> {
    let mut out = Vec::new();
    if r.len() != tr.len() {
        out.push(format!("{file}: row count"));
        return out;
    }
    for (ri, (row, l)) in r.iter().zip(tr).enumerate() {
        if row.len() != 120 || l.len() != 120 {
            out.push(format!("{file}: row {} width", ri + 1));
            continue;
        }
        for (ci, (got, want)) in row.iter().zip(l).enumerate() {
            let want = want.trim_end();
            let want = if want == "<NULL>" { "<NULL>" } else { want };
            if got != want {
                out.push(format!("{file}: row {} column {}", ri + 1, ci + 1));
            }
        }
    }
    out
}

fn check_wide(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rd(d, "T_WIDE", &ReadOptions::default()).unwrap();
    let tr = t(c, "truth_wide.txt");
    ceq!(c, (tr.len(), r.len()), (4, 4));
    let diffs = wide_diff(c.file, &tr, &r);
    c.merge(diffs);
    w
}

fn check_probe6(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rd(d, "T_PROBE6", &ReadOptions::default()).unwrap();
    let tr = t(c, "truth_probe6.txt");
    ceq!(c, tr.len(), 300);
    ceq!(c, r.len(), 300);
    let mut got = r.clone();
    got.sort();
    let mut want = tr.clone();
    want.sort();
    ceq!(c, got, want, "{}", c.file);
    w
}

/// T_BN1 to T_BN4: the truth gives the length and the first bytes; the rest is the RPAD of the SQL of
/// `corpus/sql/06_bn.sql` (the same two hex digits, or the same pattern, repeated).
fn check_bn(c: &Ctx, d: &mut Dump, n: u8) -> Vec<Warning> {
    let table = format!("T_BN{n}");
    let (r, w) = c.rd(d, &table, &ReadOptions::default()).unwrap();
    let tr = t(c, &format!("truth_bn{n}.txt"));
    ceq!(c, r.len(), tr.len());
    ceq!(c, tr.len(), if n <= 2 { 8 } else { 32 });
    let mut got = r.clone();
    got.sort_by(|a, b| a[1].cmp(&b[1]));
    for (row, l) in got.iter().zip(&tr) {
        let len: usize = l[1].strip_prefix("len=").unwrap().parse().unwrap();
        if n <= 2 {
            // RAW: HEXTORAW(RPAD('A' || level, 2 * len, 'A' || level)): the byte `A<level>` len times
            ceq!(c, row[0], l[0]);
            let b = u8::from_str_radix(&l[2][..2], 16).unwrap();
            ceq!(c, l[2], format!("{b:02X}").repeat(4));
            ceq!(c, row[1], format!("{b:02x}").repeat(len), "{}", c.file);
        } else {
            // VARCHAR2: RPAD('N' || LPAD(level, 3, '0') || '_', len, 'x'), ID NULL
            ceq!(c, row[0], "<NULL>");
            ceq!(c, l[0], "<NULL>");
            ceq!(c, row[1].len(), len);
            cok!(c, row[1].starts_with(&l[2]), "{}", c.file);
            ceq!(c, &row[1][l[2].len()..], "x".repeat(len - l[2].len()));
        }
    }
    w
}

fn check_ddl(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rds(d, "T_DDL", &ReadOptions::default()).unwrap();
    let want: Rows = t(c, "truth_ddl.txt")
        .into_iter()
        .map(|l| {
            l.iter()
                .enumerate()
                .map(|(i, x)| {
                    if i == 3 && !is_null(x) {
                        plain(x)
                    } else {
                        unwrap_null(x)
                    }
                })
                .collect()
        })
        .collect();
    ceq!(c, want.len(), 3);
    ceq!(c, r, want, "{}", c.file);
    w
}

/// T_PARENT of `corpus/sql/01_corpus_tables.sql`: (1, 'p1'), (2, 'p2'); 2 rows in `truth_counts.txt`.
fn check_parent(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rd(d, "T_PARENT", &ReadOptions::default()).unwrap();
    ceq!(
        c,
        r,
        vec![
            vec!["1".to_string(), "p1".into()],
            vec!["2".into(), "p2".into()]
        ],
        "{}",
        c.file
    );
    w
}

fn check_own(c: &Ctx, d: &mut Dump, table: &str) -> Vec<Warning> {
    let (r, w) = c.rd(d, table, &ReadOptions::default()).unwrap();
    let want: Rows = t(c, "truth_own.txt")
        .into_iter()
        .filter(|l| l[0] == table)
        .map(|l| l[1..].iter().map(|x| unwrap_null(x)).collect())
        .collect();
    if table == "O_EMPTY" {
        ceq!(c, want, vec![vec!["count=0".to_string()]]);
        cok!(c, r.is_empty(), "{}", c.file);
    } else {
        cok!(c, !want.is_empty());
        ceq!(c, r, want, "{} {table}", c.file);
    }
    w
}

/// T_STAT1 = the 50 `ROW` lines of `truth_stat.txt` (ID, V, N, D); T_STAT2 and T_STAT3 are `CREATE TABLE ... AS
/// SELECT * FROM t_stat1` (`corpus/sql/09_stat.sql`): the same rows; T_STAT4 has none.
fn stat_diff(file: &str, table: &str, tr: &[Vec<String>], r: &[Vec<String>]) -> Vec<String> {
    let mut out = Vec::new();
    if table == "T_STAT4" {
        if !r.is_empty() {
            out.push(format!("{file}: {table} should be empty"));
        }
        return out;
    }
    let want: Vec<Vec<String>> = tr
        .iter()
        .filter(|l| l[0] == "ROW")
        .map(|l| {
            vec![
                l[1].clone(),
                unwrap_null(&l[2]),
                if is_null(&l[3]) {
                    "<NULL>".into()
                } else {
                    plain(&l[3])
                },
                if is_null(&l[4]) {
                    "<NULL>".into()
                } else {
                    format!("{} 00:00:00", l[4])
                },
            ]
        })
        .collect();
    if want.len() != 50 || r.len() != want.len() {
        out.push(format!("{file}: {table} row count"));
        return out;
    }
    for (row, w) in r.iter().zip(&want) {
        for (ci, (g, x)) in row.iter().zip(w).enumerate() {
            if g != x {
                out.push(format!("{file}: {table} id {} column {}", w[0], ci + 1));
            }
        }
    }
    out
}

fn check_stat(c: &Ctx, d: &mut Dump, table: &str) -> Vec<Warning> {
    let (r, w) = c.rds(d, table, &ReadOptions::default()).unwrap();
    let diffs = stat_diff(c.file, table, &t(c, "truth_stat.txt"), &r);
    c.merge(diffs);
    w
}

fn check_empty(c: &Ctx, d: &mut Dump, table: &str) -> Vec<Warning> {
    let (r, w) = c.rd(d, table, &ReadOptions::default()).unwrap();
    cok!(c, r.is_empty(), "{} {table}", c.file);
    w
}

/// T_NCHAR (ID, NC, CC, NV) against `truth_nchar.txt` (`id|DUMP nc (UTF-16, padded to 5)|DUMP cc (UTF-8,
/// padded to 4 characters)|DUMP nv (UTF-16)`). With `trimmed` (`trim_char`) the trailing U+0020 of NC and CC
/// are removed; NV (NVARCHAR2) is never trimmed.
fn nchar_diff(file: &str, tr: &[Vec<String>], r: &[Vec<String>], trimmed: bool) -> Vec<String> {
    let mut out = Vec::new();
    if r.len() != tr.len() {
        out.push(format!("{file}: row count"));
        return out;
    }
    let fix = |s: String| {
        if trimmed {
            s.trim_end_matches(' ').to_string()
        } else {
            s
        }
    };
    for (row, l) in r.iter().zip(tr) {
        if row[0] != l[0] {
            out.push(format!("{file}: id order at {}", l[0]));
            continue;
        }
        let nc = dump_bytes(&l[1]).map(|b| fix(utf16be(&b)));
        let cc = dump_bytes(&l[2]).map(|b| fix(utf8(&b)));
        let nv = dump_bytes(&l[3]).map(|b| utf16be(&b));
        for (i, want, name) in [(1usize, nc, "NC"), (2, cc, "CC"), (3, nv, "NV")] {
            if row[i] != opt(want) {
                out.push(format!("{file}: id {} column {name}", l[0]));
            }
        }
    }
    out
}

fn check_nchar(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rds(d, "T_NCHAR", &ReadOptions::default()).unwrap();
    let tr = t(c, "truth_nchar.txt");
    ceq!(c, (tr.len(), r.len()), (3, 3));
    // the padding is kept
    let diffs = nchar_diff(c.file, &tr, &r, false);
    c.merge(diffs);
    // `trim_char` removes the padding of NC and CC (once, T-10b (b7)); NV is unchanged
    let o = ReadOptions {
        trim_char: true,
        ..ReadOptions::default()
    };
    let (rt, _) = c.rds(d, "T_NCHAR", &o).unwrap();
    let diffs = nchar_diff(c.file, &tr, &rt, true);
    c.merge(diffs);
    w
}

fn check_nchar2(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rd(d, "T_NCHAR2", &ReadOptions::default()).unwrap();
    let tr = t(c, "truth_nchar2.txt");
    ceq!(c, (tr.len(), r.len()), (1, 1));
    let l = &tr[0];
    // truth: id|NC|NV|CC|VC (DUMPs)|CL length|NCL length|DUMP SUBSTR(CL)|DUMP SUBSTR(NCL)
    let u16s = |i: usize| utf16be(&dump_bytes(&l[i]).unwrap());
    let u8s = |i: usize| utf8(&dump_bytes(&l[i]).unwrap());
    let want = vec![
        l[0].clone(),
        u16s(1),
        u16s(2),
        u8s(3),
        u8s(4),
        u8s(7),
        u16s(8),
    ];
    ceq!(c, r[0], want, "{}", c.file);
    // the astral character is one scalar value (a surrogate pair in UTF-16)
    cok!(c, r[0][1].contains('\u{1F600}'));
    w
}

fn check_probe11(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rds(d, "T_PROBE11", &ReadOptions::default()).unwrap();
    let tr: Vec<Vec<String>> = t(c, "truth_probe11.txt")
        .into_iter()
        .filter(|l| l[0] == "ROW")
        .collect();
    ceq!(c, (tr.len(), r.len()), (2, 2));
    for (row, l) in r.iter().zip(&tr) {
        // truth: ROW|id|B|C|K|W|X|LENGTHB(Y)|LENGTHB(Z); Y and Z are CHAR(500 CHAR) / CHAR(501 CHAR) holding
        // the text of C padded with spaces to 500 / 501 characters
        ceq!(c, row[0], l[1]);
        for (cell_i, t_i) in [(1usize, 2usize), (2, 3), (3, 4), (4, 5), (5, 6)] {
            ceq!(
                c,
                row[cell_i],
                opt(dump_bytes(&l[t_i]).map(|b| utf8(&b))),
                "{} id {} col {cell_i}",
                c.file,
                l[1]
            );
        }
        let text = row[2].trim_end().to_string();
        for (cell_i, chars, bytes_i) in [(6usize, 500usize, 7usize), (7, 501, 8)] {
            let v = &row[cell_i];
            ceq!(c, v.chars().count(), chars);
            ceq!(c, v.len().to_string(), l[bytes_i]);
            ceq!(c, v.trim_end(), text);
        }
    }
    w
}

fn check_decl(c: &Ctx, d: &mut Dump) -> Vec<Warning> {
    let (r, w) = c.rd(d, "T_DECL", &ReadOptions::default()).unwrap();
    // the row of corpus/sql/11_types.sql
    let want: Vec<String> = [
        "1",
        "1.25",
        "1",
        "100",
        "0.00123",
        "1",
        "1.25",
        "1.5",
        "1.5",
        "1.5",
        &ts_units("2020-01-02 03:04:05.123456", 1000),
        "2020-01-02 00:00:00",
        "a,b)",
        "x",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect();
    ceq!(c, r, vec![want], "{}", c.file);
    w
}

/// Tables with a row count in `truth_counts.txt` also get their count compared (all instances).
fn counts_truth(c: &Ctx, table: &str) -> Option<usize> {
    t(c, "truth_counts.txt")
        .into_iter()
        .find(|l| l[0] == table)
        .map(|l| l[1].parse().unwrap())
}

/// The family checks of `common/truth_checks.rs` (each returns the list of differences) by name.
type FamilyCheck = fn() -> Vec<String>;

const FAMILIES: &[(&str, FamilyCheck)] = &[
    ("lob_diffs", truth_checks::lob_diffs),
    ("lob3_diffs", truth_checks::lob3_diffs),
    ("misc_diffs", truth_checks::misc_diffs),
    ("misc2_diffs", truth_checks::misc2_diffs),
    ("long2_diffs", truth_checks::long2_diffs),
    ("part2_diffs", truth_checks::part2_diffs),
    ("r_clob_diffs", truth_checks::r_clob_diffs),
    ("r_lob2_diffs", truth_checks::r_lob2_diffs),
    ("r_text_diffs", truth_checks::r_text_diffs),
    ("r_stat_diffs", truth_checks::r_stat_diffs),
    ("stc_diffs", truth_checks::stc_diffs),
    ("charsets_diffs", truth_checks::charsets_diffs),
    ("csmap_diffs", truth_checks::csmap_diffs),
    ("env_diffs", truth_checks::env_diffs),
    ("own_ddl_diffs", truth_checks::own_ddl_diffs),
    // T-19 (ADR-97): the session 17 fixtures; their tests are in `t19_long.rs`
    ("long3_diffs", truth_checks::long3_diffs),
    ("tsp_diffs", truth_checks::tsp_diffs),
    ("seg_diffs", truth_checks::seg_diffs),
    // T-19b: the session 19 and 20 fixtures; their tests are in `t19_long.rs`
    ("long4_diffs", truth_checks::long4_diffs),
    ("long5_diffs", truth_checks::long5_diffs),
    ("longraw4_diffs", truth_checks::longraw4_diffs),
    ("longraw5_diffs", truth_checks::longraw5_diffs),
    ("tsltz2_diffs", truth_checks::tsltz2_diffs),
];

/// Tables whose checker lives in a test: (table, test file, test function, family check the test calls or
/// `""`). `delegated_tests_exist` fails if the function is gone or no longer calls its family check; the
/// truth-file test runs every family check, so a truth file counts as compared only when a check opened it.
const DELEGATED: &[(&str, &str, &str, &str)] = &[
    (
        "T_LOB",
        "t6l_lob.rs",
        "t_lob_equals_the_sql_and_the_truth_in_both_versions",
        "lob_diffs",
    ),
    (
        "T_LOB2",
        "t6l_lob.rs",
        "t_lob2_equals_the_sql_and_the_truth",
        "lob_diffs",
    ),
    (
        "T_LOB3",
        "t6l_lob.rs",
        "lob3_values_around_the_switch_and_the_surrogate_pair",
        "lob3_diffs",
    ),
    (
        "T_LONG",
        "t6_read.rs",
        "long_and_long_raw_values_conventional_and_direct",
        "",
    ),
    (
        "T_LONGRAW",
        "t6_read.rs",
        "long_and_long_raw_values_conventional_and_direct",
        "",
    ),
    (
        "T_LONG2",
        "t6_read.rs",
        "t_long2_decodes_cyrillic_and_accents_without_a_warning",
        "long2_diffs",
    ),
    (
        "T_LONG_EMPTY",
        "t10_truth.rs",
        "t_long_empty_has_no_rows_and_exists_in_the_direct_files_only",
        "",
    ),
    (
        "T_INTERVAL",
        "t6_read.rs",
        "intervals_and_rowids_equal_truth_misc",
        "misc_diffs",
    ),
    (
        "T_ROWID",
        "t6_read.rs",
        "intervals_and_rowids_equal_truth_misc",
        "misc_diffs",
    ),
    (
        "T_INTERVAL2",
        "t10_truth.rs",
        "t_interval2_and_t_rowid2_equal_truth_misc2",
        "misc2_diffs",
    ),
    (
        "T_ROWID2",
        "t10_truth.rs",
        "t_interval2_and_t_rowid2_equal_truth_misc2",
        "misc2_diffs",
    ),
    (
        "T_PART",
        "t10_truth.rs",
        "t_part_matches_the_p_lines_and_the_total_line",
        "",
    ),
    (
        "T_PART2",
        "t6_parts.rs",
        "t_part2_gives_100_rows_and_each_part_holds_its_own_rows",
        "part2_diffs",
    ),
    (
        "R_TEXT",
        "t10_truth.rs",
        "r_text_equals_truth_ru_in_all_27_sections",
        "r_text_diffs",
    ),
    (
        "R_CLOB",
        "t6l_ru.rs",
        "r_clob_in_all_seven_files",
        "r_clob_diffs",
    ),
    (
        "R_LOB2",
        "t6l_ru.rs",
        "r_lob2_decodes_to_the_truth_in_both_client_files",
        "r_lob2_diffs",
    ),
    (
        "R_STAT",
        "t10_truth.rs",
        "r_stat_equals_truth_stat_and_the_sql_rule",
        "r_stat_diffs",
    ),
    (
        "CYRILLIC",
        "t6l_ru.rs",
        "r_text_and_the_cyrillic_table_around_r_clob",
        "",
    ),
    (
        "T_STC",
        "t7_owner.rs",
        "stc_family_reads_through_the_same_code",
        "stc_diffs",
    ),
    // T-19: T_FRAME of the long3 files has its checker above (`check_frame`)
    (
        "T_LONG3",
        "t19_long.rs",
        "t_long3_equals_the_truth_in_all_four_files",
        "long3_diffs",
    ),
    (
        "T_TSP",
        "t19_long.rs",
        "t_tsp_equals_the_truth_in_all_four_files",
        "tsp_diffs",
    ),
    (
        "T_LONG4",
        "t19_long.rs",
        "t_long4_equals_the_truth_in_all_four_files",
        "long4_diffs",
    ),
    (
        "T_LONG5",
        "t19_long.rs",
        "t_long5_equals_the_truth_in_all_four_files",
        "long5_diffs",
    ),
    (
        "T_LONGRAW4",
        "t19_long.rs",
        "t_longraw4_equals_the_truth_in_both_18c_files",
        "longraw4_diffs",
    ),
    (
        "T_LONGRAW5",
        "t19_long.rs",
        "t_longraw5_equals_the_truth_in_all_four_files",
        "longraw5_diffs",
    ),
    (
        "T_TSLTZ2",
        "t19_long.rs",
        "t_tsltz2_equals_the_truth_in_all_four_files",
        "tsltz2_diffs",
    ),
    (
        "T_SEG0",
        "t19_long.rs",
        "the_seg_files_hold_three_empty_tables_with_the_end_markers_of_the_truth",
        "seg_diffs",
    ),
    (
        "T_SEG1",
        "t19_long.rs",
        "the_seg_files_hold_three_empty_tables_with_the_end_markers_of_the_truth",
        "seg_diffs",
    ),
    (
        "T_SEG2",
        "t19_long.rs",
        "the_seg_files_hold_three_empty_tables_with_the_end_markers_of_the_truth",
        "seg_diffs",
    ),
];

fn delegated(table: &str) -> Option<&'static str> {
    let key = if table.starts_with("T_STC") && table.len() == 6 {
        "T_STC"
    } else if !table.is_ascii() || table.contains('?') || table.contains("\\x") {
        // a damaged or non-ASCII name; the golden catalog writes such bytes as `\xNN`
        "CYRILLIC"
    } else {
        table
    };
    DELEGATED.iter().find(|d| d.0 == key).map(|d| d.0)
}

/// Runs the checker of `table`; `None` when the table is delegated. Returns the checker's tag, the warnings
/// and the list of differences (empty = the reader's values equal the truth). Nothing here asserts.
fn check_table(
    c: &Ctx,
    d: &mut Dump,
    table: &str,
) -> Option<(&'static str, Vec<Warning>, Vec<String>)> {
    let (tag, w) = match table {
        "T_FRAME" => ("T_FRAME", check_frame(c, d)),
        "T_LEN" => ("T_LEN", check_len(c, d)),
        "T_NULLMIX" => ("T_NULLMIX", check_nullmix(c, d)),
        "T_NUM" => ("T_NUM", check_num(c, d, "T_NUM", "truth_num.txt", 33)),
        "T_NUM2" => ("T_NUM2", check_num(c, d, "T_NUM2", "truth_num2.txt", 14)),
        "T_NUM3" => ("T_NUM3", check_num(c, d, "T_NUM3", "truth_num3.txt", 6)),
        "T_NUMTYPES" => ("T_NUMTYPES", check_numtypes(c, d)),
        "T_DATE" => ("T_DATE", check_date(c, d)),
        "T_TS" => ("T_TS", check_ts(c, d)),
        "T_TSTZ" => ("T_TSTZ", check_tstz(c, d)),
        "T_TEXT" => ("T_TEXT", check_text(c, d)),
        "T_RAW" => ("T_RAW", check_raw(c, d)),
        "T_WIDE" => ("T_WIDE", check_wide(c, d)),
        "T_PROBE6" => ("T_PROBE6", check_probe6(c, d)),
        "T_BN1" => ("T_BN1", check_bn(c, d, 1)),
        "T_BN2" => ("T_BN2", check_bn(c, d, 2)),
        "T_BN3" => ("T_BN3", check_bn(c, d, 3)),
        "T_BN4" => ("T_BN4", check_bn(c, d, 4)),
        "T_DDL" => ("T_DDL", check_ddl(c, d)),
        "T_PARENT" => ("T_PARENT", check_parent(c, d)),
        "O_PARENT" => ("O_PARENT", check_own(c, d, "O_PARENT")),
        "O_CHILD" => ("O_CHILD", check_own(c, d, "O_CHILD")),
        "O_EMPTY" => ("O_EMPTY", check_own(c, d, "O_EMPTY")),
        "T_STAT1" => ("T_STAT", check_stat(c, d, table)),
        "T_STAT2" => ("T_STAT", check_stat(c, d, table)),
        "T_STAT3" => ("T_STAT", check_stat(c, d, table)),
        "T_STAT4" => ("T_STAT", check_stat(c, d, table)),
        "T_EMPTY" => ("T_EMPTY", check_empty(c, d, table)),
        "T_NCHAR" => ("T_NCHAR", check_nchar(c, d)),
        "T_NCHAR2" => ("T_NCHAR2", check_nchar2(c, d)),
        "T_PROBE11" => ("T_PROBE11", check_probe11(c, d)),
        "T_DECL" => ("T_DECL", check_decl(c, d)),
        _ => return None,
    };
    if let Some(n) = counts_truth(c, table) {
        // `truth_counts.txt` lists the table: the row count of the (lenient) read is that number
        let (r, _) = c.rds(d, table, &lenient()).unwrap();
        ceq!(c, r.len(), n, "{} {table} against truth_counts", c.file);
    }
    Some((tag, w, c.take()))
}

/// The one union of the differences of all checkers, each prefixed with its tag. Both
/// `every_table_of_the_exp_files_is_classified_and_read` (which asserts it empty) and the per-tag break test
/// (which expects it not empty) call this function and nothing else (ADR-43 item 5).
fn union_of(per_table: &[(&'static str, Vec<String>)]) -> Vec<String> {
    per_table
        .iter()
        .flat_map(|(tag, d)| d.iter().map(move |x| format!("{tag}: {x}")))
        .collect()
}

/// Every tag `check_table` can give (T_STAT stands for T_STAT1 to T_STAT4).
const CHECKER_TAGS: &[&str] = &[
    "T_FRAME",
    "T_LEN",
    "T_NULLMIX",
    "T_NUM",
    "T_NUM2",
    "T_NUM3",
    "T_NUMTYPES",
    "T_DATE",
    "T_TS",
    "T_TSTZ",
    "T_TEXT",
    "T_RAW",
    "T_WIDE",
    "T_PROBE6",
    "T_BN1",
    "T_BN2",
    "T_BN3",
    "T_BN4",
    "T_DDL",
    "T_PARENT",
    "O_PARENT",
    "O_CHILD",
    "O_EMPTY",
    "T_EMPTY",
    "T_NCHAR",
    "T_NCHAR2",
    "T_PROBE11",
    "T_DECL",
    "T_STAT",
];

/// The tag `check_table` gives a table that has a checker here (T_STAT1 to T_STAT4 share one); `None` for a
/// table without one.
fn checker_tag(table: &str) -> Option<&'static str> {
    if matches!(table, "T_STAT1" | "T_STAT2" | "T_STAT3" | "T_STAT4") {
        return Some("T_STAT");
    }
    CHECKER_TAGS.iter().find(|t| **t == table).copied()
}

/// Instances (sections) per checker tag and per delegated key over the 229 files, derived from the golden
/// catalog `tests/golden/exp_catalog.tsv` (the Analyst's `exp_parse.py` through `gen_exp_catalog.py`) with the
/// keying of `delegated()` (T-10b, S-6). A checker that stops running, or a table that is neither checked nor
/// delegated, changes these numbers or panics here.
fn expected_counts() -> (BTreeMap<&'static str, usize>, BTreeMap<&'static str, usize>) {
    let mut checked = BTreeMap::new();
    let mut deleg = BTreeMap::new();
    for (file, table) in golden_sections() {
        if let Some(key) = delegated(&table) {
            *deleg.entry(key).or_default() += 1;
        } else if let Some(tag) = checker_tag(&table) {
            *checked.entry(tag).or_default() += 1;
        } else {
            panic!("{file}: the catalog table {table:?} has no checker and is not delegated");
        }
    }
    (checked, deleg)
}

/// The warnings the reads of the 211 files produce (the two `ru_lob2_*` files add none), by (table, code), with the number of files: the
/// inventory the plan asks for. `W-TSLTZ` is the one of arch 5.1 (T_TSTZ.TSLTZ); `W-NUMBER-ROUNDED` and
/// `W-VALUE-NULLED` are the counted warnings of the lenient re-reads of T_NUM, T_NUM2, T_NUM3, T_DATE, T_TS.
const EXPECTED_WARNINGS: &[(&str, &str, usize)] = &[
    // T-9 (not the new files, see `the_ru_lob2_files_add_no_warning`): the table whose export failed (X2;
    // `EXP-00056` / `ORA-00942` in `ru_exp_us7ascii_11g.log` and `ru_exp_w1252_11g.log`, `EXP-00008` /
    // `ORA-00604` / `ORA-00942` in `ru_exp_direct_us7ascii_11g.log`; exp.md 14.3, ADR-39) has no rows and a damaged name: the name is
    // `???_???` in the two US7ASCII client files and `\u{bf}\u{bf}\u{bf}_\u{bf}\u{bf}\u{bf}` in the WE8MSWIN1252 one
    // (`t9_charset.rs` asserts exactly this warning per file). The T-10 inventory of the 199 files, written
    // before T-9 was merged, did not have these three; T-3d found them when the test could run again.
    ("???_???", "W-POSSIBLY-INCOMPLETE", 2),
    ("T_DATE", "W-VALUE-NULLED", 8),
    ("T_INTERVAL2", "W-VALUE-NULLED", 2),
    ("T_NUM", "W-NUMBER-ROUNDED", 8),
    ("T_NUM", "W-VALUE-NULLED", 8),
    ("T_NUM2", "W-NUMBER-ROUNDED", 2),
    ("T_NUM3", "W-NUMBER-ROUNDED", 2),
    ("T_TS", "W-VALUE-NULLED", 8),
    // T-19b: the three TSLTZ columns of T_TSLTZ2 in each of the four files of ADR-100 item 1 (nothing else)
    ("T_TSLTZ2", "W-TSLTZ", 4),
    ("T_TSTZ", "W-TSLTZ", 4),
    (
        "\u{bf}\u{bf}\u{bf}_\u{bf}\u{bf}\u{bf}",
        "W-POSSIBLY-INCOMPLETE",
        1,
    ),
];

/// What one pass over the 229 files found.
struct Scan {
    checked: BTreeMap<&'static str, usize>,
    deleg: BTreeMap<&'static str, usize>,
    warnings: BTreeMap<(String, String), usize>,
    sections: usize,
    /// (tag, differences) of every checked table, input of `union_of`
    diffs: Vec<(&'static str, Vec<String>)>,
}

/// Reads every table of every `_exp_` file: a table with a checker here is checked, a delegated one is read
/// to the end and must deliver `num_rows` rows. Every `truth_*.txt` a checker needs is opened through
/// `common::truth_text`.
fn scan() -> Scan {
    let files = exp_files();
    assert_eq!(files.len(), TOTALS.files);
    let mut checked: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut deleg: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut warnings: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut sections = 0;
    let mut diffs: Vec<(&'static str, Vec<String>)> = Vec::new();
    for rel in &files {
        let (folder, file) = rel.split_once('/').unwrap();
        let mut d = open_fixture(rel);
        let tables: Vec<(String, u32, u64)> = d
            .tables()
            .unwrap()
            .iter()
            .map(|t| (t.name.clone(), t.ordinal, t.num_rows))
            .collect();
        // no file stops (ADR-12): every file walks to the second EXIT
        assert!(
            matches!(d.catalog().unwrap().end, CatalogEnd::Complete),
            "{rel} stops"
        );
        let names: BTreeMap<u32, String> = tables.iter().map(|t| (t.1, t.0.clone())).collect();
        let mut seen: BTreeSet<(String, String)> = BTreeSet::new();
        for w in d.warnings() {
            let tn = w
                .table
                .and_then(|o| names.get(&o))
                .cloned()
                .unwrap_or_default();
            seen.insert((tn, w.code.as_str().to_string()));
        }
        let c = Ctx::new(folder, file);
        for (name, _, num_rows) in &tables {
            sections += 1;
            if let Some(key) = delegated(name) {
                *deleg.entry(key).or_default() += 1;
                // a delegated table still reads to the end and delivers num_rows rows
                let (r, w) = rd(&mut d, name, &lenient()).unwrap();
                assert_eq!(r.len() as u64, *num_rows, "{rel} {name}");
                for w in w {
                    seen.insert((name.clone(), w.code.as_str().to_string()));
                }
                continue;
            }
            let (tag, w, df) = check_table(&c, &mut d, name).unwrap_or_else(|| {
                panic!("{rel}: table {name:?} has no checker and is not delegated")
            });
            *checked.entry(tag).or_default() += 1;
            diffs.push((tag, df));
            for w in w {
                seen.insert((name.clone(), w.code.as_str().to_string()));
            }
        }
        for k in seen {
            *warnings.entry(k).or_default() += 1;
        }
    }
    Scan {
        checked,
        deleg,
        warnings,
        sections,
        diffs,
    }
}

#[test]
fn every_table_of_the_exp_files_is_classified_and_read() {
    let s = scan();
    // the checkers assert nothing themselves (ADR-43 item 5): this is the one place that fails on a
    // difference between the reader's values and the truth, for every table of every file
    let u = union_of(&s.diffs);
    assert!(
        u.is_empty(),
        "{} differences, first: {:?}",
        u.len(),
        &u[..u.len().min(5)]
    );
    // the sections of the golden catalog (436 lines) and the Analyst's total agree with what was read
    assert_eq!(
        s.sections,
        golden_data_lines("tests/golden/exp_catalog.tsv")
    );
    assert_eq!(s.sections, TOTALS.sections);
    let (checked, deleg) = expected_counts();
    assert_eq!(s.checked, checked);
    assert_eq!(s.deleg, deleg);
    assert_eq!(
        checked.values().sum::<usize>() + deleg.values().sum::<usize>(),
        TOTALS.sections
    );
    let got: Vec<(&str, &str, usize)> = s
        .warnings
        .iter()
        .map(|((t, c), n)| (t.as_str(), c.as_str(), *n))
        .collect();
    assert_eq!(got, EXPECTED_WARNINGS, "{got:?}");
}

/// Every registry entry names a test that exists, and a test that has a family check still calls it.
#[test]
fn delegated_tests_exist() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut used = BTreeSet::new();
    for (table, file, func, family) in DELEGATED {
        let src = std::fs::read_to_string(dir.join(file)).unwrap();
        let at = src
            .find(&format!("fn {func}("))
            .unwrap_or_else(|| panic!("{table}: {file} has no test {func}"));
        if !family.is_empty() {
            used.insert(*family);
            let body = &src[at..];
            let body = &body[..body.find("\n#[test]").unwrap_or(body.len())];
            assert!(
                body.contains(&format!("{family}()")),
                "{table}: {file}::{func} does not call {family}"
            );
            assert!(
                FAMILIES.iter().any(|f| f.0 == *family),
                "{family} is not in FAMILIES"
            );
        }
    }
    // charsets_diffs and csmap_diffs have their own tests below
    for (name, _) in FAMILIES {
        assert!(
            used.contains(name)
                || [
                    "charsets_diffs",
                    "csmap_diffs",
                    "env_diffs",
                    "own_ddl_diffs"
                ]
                .contains(name),
            "{name} is called by no delegated test"
        );
    }
}

/// The rows of T_NUM that no Decimal128(38, 10) holds exactly, computed from the truth text: the integer part
/// of rows 19 to 24, 26, 28, 29 and 32 has more than 28 digits (value error under every policy); rows 25, 27,
/// 30 and 31 have more than 10 decimals (rounded; row 27, 1E-125, becomes 0); `truth_num2` rows 13 and 14 and
/// all six `truth_num3` rows are rounded (13 and 14 to 0; none of `truth_num3` to 0). Numbers from the same
/// arithmetic as `tests/t4_values.rs` (plan T-4 (b)); `fit_10` is checked here against them.
#[test]
fn the_classification_of_the_truth_numbers() {
    let classify = |file: &str| -> (Vec<usize>, Vec<usize>, usize) {
        let mut over = Vec::new();
        let mut rounded = Vec::new();
        let mut zero = 0;
        for (i, l) in truth("18c", file).iter().enumerate() {
            if is_null(&l[1]) {
                continue;
            }
            match fit_10(&l[1]) {
                Fit::Overflow => over.push(i + 1),
                Fit::Rounded(_, z) => {
                    rounded.push(i + 1);
                    zero += usize::from(z);
                }
                Fit::Exact(_) => {}
            }
        }
        (over, rounded, zero)
    };
    assert_eq!(
        classify("truth_num.txt"),
        (
            vec![19, 20, 21, 22, 23, 24, 26, 28, 29, 32],
            vec![25, 27, 30, 31],
            1
        )
    );
    assert_eq!(classify("truth_num2.txt"), (vec![], vec![13, 14], 2));
    assert_eq!(
        classify("truth_num3.txt"),
        (vec![], vec![1, 2, 3, 4, 5, 6], 0)
    );
    assert_eq!(plain(".5"), "0.5");
    assert_eq!(plain("-.5"), "-0.5");
    assert_eq!(plain("1E-3"), "0.001");
    assert_eq!(plain("1.5E+2"), "150");
    assert_eq!(plain("100"), "100");
    assert_eq!(round_plain("0.12345678905", 10), "0.1234567891");
    assert_eq!(round_plain("-0.00000000005", 10), "-0.0000000001");
    assert_eq!(round_plain("0.99999999995", 10), "1");
}

/// T_PART (both paths): the `P|` lines of `truth_part.txt` give the rows of each part (one batch per part, in
/// file order), the `TOTAL` line gives the rows over the whole table, the sum of AMOUNT and the first and last
/// SALE_DATE (`DD-MON-YY`); every number is read from the truth file.
#[test]
fn t_part_matches_the_p_lines_and_the_total_line() {
    check_t_part();
}

fn check_t_part() {
    let tr = truth("18c", "truth_part.txt");
    let parts: Vec<(String, usize)> = tr
        .iter()
        .filter(|l| l[0] == "P")
        .map(|l| (l[1].clone(), l[2].parse().unwrap()))
        .collect();
    let total = tr.iter().find(|l| l[0] == "TOTAL").unwrap();
    assert_eq!(parts.len(), 3);
    let month = |m: &str| -> u32 {
        [
            "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
        ]
        .iter()
        .position(|x| *x == m)
        .unwrap() as u32
            + 1
    };
    let date = |t: &str| -> String {
        let p: Vec<&str> = t.split('-').collect();
        format!("20{}-{:02}-{}", p[2], month(p[1]), p[0])
    };
    for file in ["18c/part_exp_18c.dmp", "18c/part_exp_direct_18c.dmp"] {
        let mut d = open_fixture(file);
        let info = d.table(TableRef::ByName("T_PART")).unwrap().clone();
        let names: Vec<String> = info
            .parts
            .iter()
            .map(|p| p.partition.clone().unwrap())
            .collect();
        assert_eq!(names, parts.iter().map(|p| p.0.clone()).collect::<Vec<_>>());
        let (b, _) = read(&mut d, "T_PART", &ReadOptions::default());
        let sizes: Vec<usize> = b.iter().map(|x| x.num_rows()).collect();
        assert_eq!(
            sizes,
            parts.iter().map(|p| p.1).collect::<Vec<_>>(),
            "{file}"
        );
        let r = rows(&b);
        assert_eq!(r.len().to_string(), total[1], "{file}");
        // AMOUNT is Decimal128(10, 2): the sum in cents is the truth sum times 100
        let cents: i128 = r
            .iter()
            .map(|x| {
                let v = &x[2];
                let (i, f) = v.split_once('.').unwrap_or((v, ""));
                format!("{i}{f:0<2}").parse::<i128>().unwrap()
            })
            .sum();
        assert_eq!(cents, total[2].parse::<i128>().unwrap() * 100, "{file}");
        let dates: Vec<&str> = r.iter().map(|x| &x[1][..10]).collect();
        assert_eq!(*dates.iter().min().unwrap(), date(&total[3]));
        assert_eq!(*dates.iter().max().unwrap(), date(&total[4]));
    }
}

/// `misc2_exp_<v>`: the two YEAR(9) extremes (IYM rows 2 and 3, +-999999999 years) exceed 32-bit months:
/// `E-BAD-INTERVAL` with `raise` (row 2, column 2), NULL with `null` and one `W-VALUE-NULLED` (count 2);
/// the other rows are unchanged. Plan T-10, session 5 (ADR-28).
#[test]
fn misc2_year_nine_extremes_are_bad_intervals() {
    for file in ["18c/misc2_exp_18c.dmp", "11g/misc2_exp_11g.dmp"] {
        let mut d = open_fixture(file);
        let e = try_read(&mut d, "T_INTERVAL2", &ReadOptions::default()).unwrap_err();
        assert_eq!(e.code, ErrorCode::BadInterval, "{file}");
        assert_eq!((e.row, e.column, e.part), (Some(2), Some(2), Some(1)));
        assert!(e.offset.is_some());
        let (b, w) = read(&mut d, "T_INTERVAL2", &lenient());
        let r = rows(&b);
        assert_eq!(r.len(), 3);
        assert_eq!(r[0][1], "-23m0d0ns", "{file}: -000000001-11 is -23 months");
        assert_eq!(r[1][1], "<NULL>");
        assert_eq!(r[2][1], "<NULL>");
        let n = count_of(&w, WarningCode::ValueNulled);
        assert_eq!(n.len(), 1);
        assert_eq!(
            (n[0].count, n[0].row, n[0].column),
            (Some(2), Some(2), Some(2))
        );
    }
}

/// No file is skipped: the names are the three folders' `_exp_` files and the three `SHA256SUMS` list
/// every one of them (the fixture guard of T-2 checks the sums themselves).
#[test]
fn the_exp_files_are_the_listed_ones() {
    let files = exp_files();
    assert_eq!(files.len(), TOTALS.files);
    let per: Vec<usize> = FOLDERS
        .iter()
        .map(|f| {
            files
                .iter()
                .filter(|x| x.starts_with(&format!("{f}/")))
                .count()
        })
        .collect();
    let want: Vec<usize> = FOLDER_TOTALS.iter().map(|t| t.files).collect();
    assert_eq!(per, want);
    for f in FOLDERS {
        let sums = std::fs::read_to_string(fixtures().join(f).join("SHA256SUMS")).unwrap();
        for x in files.iter().filter(|x| x.starts_with(&format!("{f}/"))) {
            let name = x.split_once('/').unwrap().1;
            assert!(sums.contains(name), "{name} is not in {f}/SHA256SUMS");
        }
    }
}

/// Truth files that no check of this binary opens, each with its reason (ADR-38 item 1): `(folder, name, reason,
/// test file, test function)`; the last two are empty when the reason is a fact about the corpus that
/// `explicit_reasons_hold` checks. A file that gets a check must leave this list: a file both opened and
/// listed fails.
const TRUTH_NOT_OPENED: &[(&str, &str, &str, &str, &str)] = &[
    (
        "18c",
        "truth_float.txt",
        "T_FLOAT exists only in a Data Pump file (no T_FLOAT section in exp_catalog.tsv)",
        "",
        "",
    ),
    (
        "11g",
        "truth_float.txt",
        "T_FLOAT exists only in a Data Pump file (no T_FLOAT section in exp_catalog.tsv)",
        "",
        "",
    ),
    (
        "18c",
        "truth_decl.txt",
        "declared types, compared by t5_types.rs (T-5)",
        "t5_types.rs",
        "t_decl_gives_the_14_precision_scale_pairs_of_truth_decl_and_the_arrow_types_of_5_1",
    ),
    (
        "11g",
        "truth_decl.txt",
        "declared types, compared by t5_types.rs (T-5)",
        "t5_types.rs",
        "t_decl_gives_the_14_precision_scale_pairs_of_truth_decl_and_the_arrow_types_of_5_1",
    ),
    (
        "18c",
        "truth_r045.txt",
        "R-045, an SQL*Plus spool, no EXP table",
        "",
        "",
    ),
    (
        "11g",
        "truth_r049.txt",
        "R-049, an SQL*Plus spool (ORA-00439: 11g XE refuses deferred segment creation), no EXP table, no dump",
        "",
        "",
    ),
];

/// The reasons of `TRUTH_NOT_OPENED` that can be checked: the named test exists, and no EXP table is T_FLOAT.
#[test]
fn explicit_reasons_hold() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    for (folder, name, _, file, func) in TRUTH_NOT_OPENED {
        if !file.is_empty() {
            let src = std::fs::read_to_string(dir.join(file)).unwrap();
            assert!(
                src.contains(&format!("fn {func}(")),
                "{folder}/{name}: {file} has no test {func}"
            );
        }
        if *name == "truth_float.txt" {
            assert!(
                golden_sections().iter().all(|(_, t)| t != "T_FLOAT"),
                "{folder}/{name}: an EXP file has a T_FLOAT section"
            );
        }
    }
}

/// The `(folder, name)` of every `truth_*.txt` in the three `SHA256SUMS`.
fn listed_truth_files() -> BTreeSet<(String, String)> {
    let mut listed = BTreeSet::new();
    for f in FOLDERS {
        let sums = std::fs::read_to_string(fixtures().join(f).join("SHA256SUMS")).unwrap();
        for l in sums.lines() {
            let name = l.rsplit(['/', ' ']).next().unwrap();
            if name.starts_with("truth_") && name.ends_with(".txt") {
                listed.insert((f.to_string(), name.to_string()));
            }
        }
    }
    listed
}

/// Runs every checker of this file and every family check, then compares the `(folder, name)` pairs that
/// `common::truth_text` recorded, plus `TRUTH_NOT_OPENED`, with the `truth_*.txt` lines of the three
/// `SHA256SUMS` (97 files since T-19b; 87 after T-19; 82 before). A literal name in a source file counts for nothing; a file both opened and
/// listed, a listed file not in the sums and a sum line neither opened nor listed all fail.
#[test]
fn every_truth_file_is_read_or_listed() {
    // the checkers of this file, over all 229 files (they open their truth files through `truth_text`)
    let _ = scan();
    check_t_part();
    for (name, f) in FAMILIES {
        let d = f();
        assert!(d.is_empty(), "{name}: {d:?}");
    }
    let opened = opened_truth_files();
    let listed = listed_truth_files();
    assert_eq!(listed.len(), 97, "truth files in the three SHA256SUMS");
    let explicit: BTreeSet<(String, String)> = TRUTH_NOT_OPENED
        .iter()
        .map(|(f, n, ..)| (f.to_string(), n.to_string()))
        .collect();
    assert_eq!(explicit.len(), TRUTH_NOT_OPENED.len(), "an entry twice");
    for (_, _, reason, _, _) in TRUTH_NOT_OPENED {
        assert!(!reason.is_empty());
    }
    let both: Vec<_> = explicit.intersection(&opened).collect();
    assert!(both.is_empty(), "opened by a check and listed: {both:?}");
    let union: BTreeSet<(String, String)> = opened.union(&explicit).cloned().collect();
    assert_eq!(
        listed.difference(&union).collect::<Vec<_>>(),
        Vec::<&(String, String)>::new(),
        "truth files neither opened nor listed"
    );
    assert_eq!(
        union.difference(&listed).collect::<Vec<_>>(),
        Vec::<&(String, String)>::new(),
        "files opened or listed that no SHA256SUMS has"
    );
}

// ---------------------------------------------------------------------------------------------------------
// the delegated families that live in this file, and the break tests of T-10b (b8)
// ---------------------------------------------------------------------------------------------------------

/// R_TEXT in all 27 sections (the 10 of the `ru_stat_*` files included) against `truth_ru.txt` by ID.
#[test]
fn r_text_equals_truth_ru_in_all_27_sections() {
    let d = truth_checks::r_text_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// R_STAT in its 10 sections: row count, NULL and distinct counts per column from `truth_stat.txt` of
/// `11g-cp1251`, values by the rule of `08_ru_stats_objects.sql`.
#[test]
fn r_stat_equals_truth_stat_and_the_sql_rule() {
    let d = truth_checks::r_stat_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// T_INTERVAL2 and T_ROWID2 of both `misc2_exp_<v>` files, every row and column, against `truth_misc2.txt` of
/// the folder (IYM rows 2 and 3 are NULL under `on_value_error="null"`, ADR-28).
#[test]
fn t_interval2_and_t_rowid2_equal_truth_misc2() {
    let d = truth_checks::misc2_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// T_LONG_EMPTY (ADR-36 item 4): the expectation is 0 rows, and the table exists in the two direct-path files
/// only; its "truth" is the empty table (no truth file).
#[test]
fn t_long_empty_has_no_rows_and_exists_in_the_direct_files_only() {
    let files = truth_checks::files_with_table("T_LONG_EMPTY");
    assert_eq!(
        files,
        ["11g/long_exp_direct_11g.dmp", "18c/long_exp_direct_18c.dmp"]
    );
    for f in files {
        let (_, r, _) =
            truth_checks::read_named(&f, "T_LONG_EMPTY", &ReadOptions::default()).unwrap();
        assert!(r.is_empty(), "{f}");
    }
}

/// No difference.
#[track_caller]
fn clean(d: Vec<String>) {
    assert!(d.is_empty(), "{d:?}");
}

/// Exactly one difference.
#[track_caller]
fn one(d: Vec<String>) {
    assert_eq!(d.len(), 1, "{d:?}");
}

fn mutated(
    lines: &[Vec<String>],
    at: usize,
    field: usize,
    from: &str,
    to: &str,
) -> Vec<Vec<String>> {
    let mut v = lines.to_vec();
    assert!(v[at][field].contains(from), "{:?}", v[at][field]);
    v[at][field] = v[at][field].replacen(from, to, 1);
    v
}

/// Reads a table to rows (default options) for the break tests.
fn rows_of(rel: &str, table: &str) -> Rows {
    truth_checks::read_named(rel, table, &ReadOptions::default())
        .unwrap()
        .1
}

#[test]
fn break_t_len_one_changed_truth_line_or_value_gives_one_difference() {
    let tr = truth("18c", "truth_len.txt");
    let r = rows_of("18c/multi_framing_exp_18c.dmp", "T_LEN");
    let r = by_id(r);
    clean(len_diff("f", &tr, &r));
    // the value: one `x` of the padding becomes a digit (review mutation a4), the character count stays
    let mut bad = r.clone();
    let k = bad.iter().position(|x| x[0] == "126").unwrap();
    let at = bad[k][1].rfind('x').unwrap();
    bad[k][1].replace_range(at..=at, "7");
    one(len_diff("f", &tr, &bad));
    // the truth: the character count, the DUMP bytes, the byte length of the DUMP
    let k = tr.iter().position(|l| l[0] == "126").unwrap();
    one(len_diff("f", &mutated(&tr, k, 1, "126", "125"), &r));
    one(len_diff("f", &mutated(&tr, k, 2, "Len=126", "Len=125"), &r));
    one(len_diff("f", &mutated(&tr, k, 2, ": 4c", ": 4d"), &r));
}

#[test]
fn break_t_nchar_one_changed_truth_line_gives_one_difference_with_and_without_trim() {
    let tr = truth("18c", "truth_nchar.txt");
    let o = ReadOptions {
        trim_char: true,
        ..ReadOptions::default()
    };
    let plain_rows = rows_of("18c/nchar_exp_18c.dmp", "T_NCHAR");
    let trimmed = truth_checks::read_named("18c/nchar_exp_18c.dmp", "T_NCHAR", &o)
        .unwrap()
        .1;
    let (plain_rows, trimmed) = (by_id(plain_rows), by_id(trimmed));
    clean(nchar_diff("f", &tr, &plain_rows, false));
    clean(nchar_diff("f", &tr, &trimmed, true));
    // trimmed rows against the padded truth: NC and CC of rows 1 and 2 differ
    assert_eq!(nchar_diff("f", &tr, &trimmed, false).len(), 4);
    // NV (NVARCHAR2) is never trimmed: one changed NV byte is one difference in both modes
    let bad = mutated(&tr, 0, 3, "0,61,0,62", "0,61,0,63");
    one(nchar_diff("f", &bad, &plain_rows, false));
    one(nchar_diff("f", &bad, &trimmed, true));
    let bad = mutated(&tr, 0, 1, "0,61,0,62", "0,61,0,63");
    one(nchar_diff("f", &bad, &plain_rows, false));
    one(nchar_diff("f", &bad, &trimmed, true));
}

#[test]
fn break_t_wide_and_t_stat_one_changed_value_gives_one_difference() {
    let tr = truth("18c", "truth_wide.txt");
    let r = rows_of("18c/t_wide_exp_18c.dmp", "T_WIDE");
    clean(wide_diff("f", &tr, &r));
    // review mutation a1 (`v115` -> `v116`) on the reader side, and the same on the truth side
    let mut bad = r.clone();
    let k = bad[0].iter().position(|c| c == "v115").unwrap();
    bad[0][k] = "v116".to_string();
    one(wide_diff("f", &tr, &bad));
    let mut bad = tr.clone();
    let k = bad[0].iter().position(|c| c.trim_end() == "v115").unwrap();
    bad[0][k] = "v116    ".to_string();
    one(wide_diff("f", &bad, &r));

    let tr = truth("18c", "truth_stat.txt");
    let r = by_id(rows_of("18c/stat_exp_18c.dmp", "T_STAT1"));
    clean(stat_diff("f", "T_STAT1", &tr, &r));
    let mut bad = tr.clone();
    let k = bad
        .iter()
        .position(|l| l[0] == "ROW" && l[1] == "7")
        .unwrap();
    bad[k][2] = format!("{}!", bad[k][2]);
    one(stat_diff("f", "T_STAT1", &bad, &r));
    one(stat_diff("f", "T_STAT4", &tr, &r));
}

#[test]
fn break_r_text_one_changed_truth_line_gives_one_difference() {
    let want = truth_checks::r_text_truth();
    let rel = "11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp";
    let r = rows_of(rel, "R_TEXT");
    clean(truth_checks::r_text_diff(rel, &want, &r));
    // V (database bytes), N (UTF-16), the LV length, its head, and D of row 2
    one(truth_checks::r_text_diff(
        rel,
        &mutated(&want, 1, 2, "cf,f0", "cf,f1"),
        &r,
    ));
    one(truth_checks::r_text_diff(
        rel,
        &mutated(&want, 1, 5, "4,1f", "4,20"),
        &r,
    ));
    one(truth_checks::r_text_diff(
        rel,
        &mutated(&want, 1, 6, "6", "7"),
        &r,
    ));
    one(truth_checks::r_text_diff(
        rel,
        &mutated(&want, 1, 8, "cf,f0", "cf,f1"),
        &r,
    ));
    one(truth_checks::r_text_diff(
        rel,
        &mutated(&want, 1, 9, "2026-01-02", "2026-01-03"),
        &r,
    ));
    // review mutation a2: a Cyrillic byte of the file (`da` -> `c2`) changes one cell on the reader side
    let mut bad = r.clone();
    let k = bad
        .iter()
        .position(|x| x[1].contains('\u{42a}'))
        .expect("a hard sign in V");
    bad[k][1] = bad[k][1].replacen('\u{42a}', "\u{412}", 1);
    one(truth_checks::r_text_diff(rel, &want, &bad));
}

#[test]
fn break_r_clob_and_r_lob2_one_changed_truth_line_gives_one_difference() {
    let want: Vec<Vec<String>> = truth("11g-cp1251", "truth_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_CLOB")
        .collect();
    let rel = "11g-cp1251/ru_exp_cp1251_11g.dmp";
    let r = rows_of(rel, "R_CLOB");
    clean(truth_checks::r_clob_diff(rel, &want, &r));
    one(truth_checks::r_clob_diff(
        rel,
        &mutated(&want, 0, 2, "6", "7"),
        &r,
    ));
    one(truth_checks::r_clob_diff(
        rel,
        &mutated(&want, 1, 3, "c7,ce", "c7,cf"),
        &r,
    ));

    let want: Vec<Vec<String>> = truth("11g-cp1251", "truth_lob_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_LOB2")
        .collect();
    let rel = truth_checks::R_LOB2_FILES[0];
    let r = rows_of(rel, "R_LOB2");
    clean(truth_checks::r_lob2_diff(rel, &want, &r));
    for (field, from, to) in [
        (2usize, "66", "67"),
        (3, "66", "67"),
        (4, "66", "67"),
        (5, "c0,c1", "c0,c2"),
        (6, "4,10", "4,11"),
        (7, "fe,ff", "fe,fe"),
        (8, "4,4e", "4,4d"),
    ] {
        one(truth_checks::r_lob2_diff(
            rel,
            &mutated(&want, 0, field, from, to),
            &r,
        ));
    }
    // N-6: B is compared with the bytes of the SQL rule, so a changed B (same length) is one difference, and a
    // changed C alone does not make a second one through B
    assert_eq!(r[0][3].len(), 132);
    let mut bad = r.clone();
    let k = bad.iter().position(|x| x[0] == "1").unwrap();
    bad[k][3].replace_range(0..2, "c1");
    one(truth_checks::r_lob2_diff(rel, &want, &bad));
    let mut bad = r.clone();
    let k = bad.iter().position(|x| x[0] == "2").unwrap();
    let n = bad[k][3].len();
    bad[k][3].replace_range(n - 2.., "c2");
    one(truth_checks::r_lob2_diff(rel, &want, &bad));
    assert_eq!(truth_checks::r_lob2_b_rule("3"), None);
}

#[test]
fn break_r_stat_one_changed_truth_line_or_value_gives_one_difference() {
    let stat = truth("11g-cp1251", "truth_stat.txt");
    let rel = "11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp";
    let (names, r, _) = truth_checks::read_named(rel, "R_STAT", &ReadOptions::default()).unwrap();
    clean(truth_checks::r_stat_diff(rel, &stat, &names, &r));
    // review mutation b5: `TAB|R_STAT|2000` -> `2001`
    let tab = stat
        .iter()
        .position(|l| l[0] == "TAB" && l[1] == "R_STAT")
        .unwrap();
    one(truth_checks::r_stat_diff(
        rel,
        &mutated(&stat, tab, 2, "2000", "2001"),
        &names,
        &r,
    ));
    // a COL line: the NULL count, the distinct count
    let col = stat
        .iter()
        .position(|l| l[0] == "COL" && l[1] == "R_STAT" && l[2] == "TXT")
        .unwrap();
    one(truth_checks::r_stat_diff(
        rel,
        &mutated(&stat, col, 4, "117", "118"),
        &names,
        &r,
    ));
    one(truth_checks::r_stat_diff(
        rel,
        &mutated(&stat, col, 3, "150", "151"),
        &names,
        &r,
    ));
    // a value of the reader against the rule of the SQL: TXT of ID 3 (not a multiple of 17)
    let mut bad = r.clone();
    let k = bad.iter().position(|x| x[0] == "3").unwrap();
    bad[k][2].push('x');
    // one more value is a new distinct value too: the rule and the distinct count both report
    assert_eq!(truth_checks::r_stat_diff(rel, &stat, &names, &bad).len(), 2);
    let mut bad = r.clone();
    let k = bad.iter().position(|x| x[0] == "3").unwrap();
    bad[k][1] = "4".to_string();
    // GRP of id 3 (rule: 6) is another existing group: NULL and distinct counts stay, the rule reports
    one(truth_checks::r_stat_diff(rel, &stat, &names, &bad));
}

#[test]
fn break_t_stc_one_changed_truth_line_or_value_gives_one_difference() {
    let rel = "18c/stc_exp_18c.dmp";
    let t = truth("18c", "truth_stc.txt");
    let cols = |table: &str| -> Vec<Vec<String>> {
        t.iter()
            .filter(|l| l[0] == "COLUMN" && l[1] == table)
            .cloned()
            .collect()
    };
    // the counts come from the truth lines (N-9), the `51` of mutation b6 is the truth count plus one
    let counts = truth_checks::stc_counts("18c");
    let n1 = counts["T_STC1"].clone();
    let n2 = counts["T_STC2"].clone();
    let n1_plus_one = (n1.parse::<u32>().unwrap() + 1).to_string();
    let (names, r, _) = truth_checks::read_named(rel, "T_STC1", &ReadOptions::default()).unwrap();
    clean(truth_checks::stc_diff(
        rel,
        "T_STC1",
        &n1,
        &cols("T_STC1"),
        &names,
        &r,
    ));
    // review mutation b6: `TAB|T_STC1|50` -> `51`
    one(truth_checks::stc_diff(
        rel,
        "T_STC1",
        &n1_plus_one,
        &cols("T_STC1"),
        &names,
        &r,
    ));
    // the statistics of a column: `stats:50:0` -> `stats:49:0`
    let bad = mutated(&cols("T_STC1"), 0, 4, "stats:50:0", "stats:49:0");
    one(truth_checks::stc_diff(rel, "T_STC1", &n1, &bad, &names, &r));
    // a value against the rule of 10_stc.sql
    let mut bad = r.clone();
    let k = bad.iter().position(|x| x[0] == "9").unwrap();
    bad[k][1] = "a9".to_string();
    // A has 7 distinct values; "a9" is a new one: the rule and the statistics both report
    assert_eq!(
        truth_checks::stc_diff(rel, "T_STC1", &n1, &cols("T_STC1"), &names, &bad).len(),
        2
    );
    // T_STC2 (ADDED_NUM = MOD(id, 3)) and T_STC4 (count from truth_stc_extra.txt)
    let (names2, r2, _) = truth_checks::read_named(rel, "T_STC2", &ReadOptions::default()).unwrap();
    clean(truth_checks::stc_diff(
        rel,
        "T_STC2",
        &n2,
        &cols("T_STC2"),
        &names2,
        &r2,
    ));
    let mut bad = r2.clone();
    let k = bad.iter().position(|x| x[0] == "4").unwrap();
    bad[k][5] = "0".to_string();
    one(truth_checks::stc_diff(
        rel,
        "T_STC2",
        &n2,
        &cols("T_STC2"),
        &names2,
        &bad,
    ));
    let extra: Vec<Vec<String>> = truth("18c", "truth_stc_extra.txt")
        .into_iter()
        .filter(|l| l[0] == "TAB" && l[1] == "T_STC4")
        .collect();
    assert_eq!(extra.len(), 1);
    assert_eq!(counts["T_STC4"], extra[0][2]);
}

#[test]
fn break_t_long2_one_changed_truth_line_gives_one_difference() {
    let want = truth("18c", "truth_long2.txt");
    let rel = "18c/long2_exp_18c.dmp";
    let r = rows_of(rel, "T_LONG2");
    clean(truth_checks::long2_diff(rel, &want, &r));
    // review mutation b4: `...C3A9` -> `...C3A8`
    let k = want.iter().position(|l| l[0] == "1").unwrap();
    one(truth_checks::long2_diff(
        rel,
        &mutated(&want, k, 2, "C3A9", "C3A8"),
        &r,
    ));
    one(truth_checks::long2_diff(
        rel,
        &mutated(&want, k, 1, "10", "11"),
        &r,
    ));
    // `INSERT REFUSED` means no such row: a row for it is a difference
    let k = want.iter().position(|l| l[1] == "INSERT REFUSED").unwrap();
    let mut extra = r.clone();
    extra.push(vec![want[k][0].clone(), "x".to_string()]);
    assert_eq!(truth_checks::long2_diff(rel, &want, &extra).len(), 2);
}

#[test]
fn break_misc_and_misc2_one_changed_line_or_value_gives_one_difference() {
    for v in ["18c", "11g"] {
        let t = truth(v, "truth_misc2.txt");
        let rel = format!("{v}/misc2_exp_{v}.dmp");
        let i = truth_checks::read_named(&rel, "T_INTERVAL2", &truth_checks::lenient())
            .unwrap()
            .1;
        let r = rows_of(&rel, "T_ROWID2");
        clean(truth_checks::misc2_diff(&rel, &t, &i, &r));
        // review mutation a3: the ROWID of row 1 changes by one character (reader side), and in the truth
        let mut bad = r.clone();
        bad[0][1] = bad[0][1].replacen("AAA", "AAB", 1);
        one(truth_checks::misc2_diff(&rel, &t, &i, &bad));
        let k = t
            .iter()
            .position(|l| l[0] == "ROWID" && l[1] == "1")
            .unwrap();
        one(truth_checks::misc2_diff(
            &rel,
            &mutated(&t, k, 2, "AAA", "AAX"),
            &i,
            &r,
        ));
        one(truth_checks::misc2_diff(
            &rel,
            &mutated(&t, k, 4, "AAA", "AAX"),
            &i,
            &r,
        ));
        let k = t
            .iter()
            .position(|l| l[0] == "ROWID" && l[1] == "10")
            .unwrap();
        one(truth_checks::misc2_diff(
            &rel,
            &mutated(&t, k, 4, "*BA", "*BB"),
            &i,
            &r,
        ));
        // the interval columns: IYM, IDS and IDS0 of row 1
        for (field, from, to) in [
            (2usize, "-000000001-11", "-000000001-10"),
            (4, "02:03:04.5", "02:03:04.6"),
            (6, "-00 00:00:01", "-00 00:00:02"),
        ] {
            let k = t
                .iter()
                .position(|l| l[0] == "INTERVAL" && l[1] == "1")
                .unwrap();
            one(truth_checks::misc2_diff(
                &rel,
                &mutated(&t, k, field, from, to),
                &i,
                &r,
            ));
        }
        // IYM row 2 is NULL under `null` because its months do not fit 32 bits: a reader value is a difference
        let mut bad = i.clone();
        bad[1][1] = "1m0d0ns".to_string();
        one(truth_checks::misc2_diff(&rel, &t, &bad, &r));
    }
    // truth_misc.txt (T_INTERVAL, T_ROWID)
    let t = truth("18c", "truth_misc.txt");
    let rel = "18c/misc_exp_18c.dmp";
    let i = rows_of(rel, "T_INTERVAL");
    let r = rows_of(rel, "T_ROWID");
    clean(truth_checks::misc_diff(rel, &t, &i, &r));
    one(truth_checks::misc_diff(
        rel,
        &mutated(&t, 0, 2, "+0002-06", "+0002-07"),
        &i,
        &r,
    ));
    one(truth_checks::misc_diff(
        rel,
        &mutated(&t, 0, 4, "12:30:45.123456", "12:30:45.123457"),
        &i,
        &r,
    ));
    let k = t
        .iter()
        .position(|l| l[0] == "ROWID" && l[1] == "1")
        .unwrap();
    one(truth_checks::misc_diff(
        rel,
        &mutated(&t, k, 2, "AAAR8l", "AAAR8m"),
        &i,
        &r,
    ));
}

#[test]
fn break_lob_checks_one_changed_truth_line_gives_one_difference() {
    let t = truth("18c", "truth_lob.txt");
    let lines = |n: &str| -> Vec<Vec<String>> { t.iter().filter(|l| l[0] == n).cloned().collect() };
    let rel = "18c/lob_exp_18c.dmp";
    let r = by_id(rows_of(rel, "T_LOB"));
    let l = lines("T_LOB");
    clean(truth_checks::t_lob_diff(rel, &l, &r));
    one(truth_checks::t_lob_diff(
        rel,
        &mutated(&l, 3, 2, "3000", "3001"),
        &r,
    ));
    one(truth_checks::t_lob_diff(
        rel,
        &mutated(&l, 3, 3, "1000", "1001"),
        &r,
    ));
    one(truth_checks::t_lob_diff(
        rel,
        &mutated(&l, 0, 4, "48656C6C", "48656C6D"),
        &r,
    ));
    let r2 = by_id(rows_of(rel, "T_LOB2"));
    let l2 = lines("T_LOB2");
    clean(truth_checks::t_lob2_diff(rel, &l2, &r2));
    one(truth_checks::t_lob2_diff(
        rel,
        &mutated(&l2, 0, 3, "x", "z"),
        &r2,
    ));
    one(truth_checks::t_lob2_diff(
        rel,
        &mutated(&l2, 0, 5, "2", "3"),
        &r2,
    ));

    let t3 = truth("18c", "truth_lob3.txt");
    let rel = "18c/lob3_exp_18c.dmp";
    let (names, r3, _) = truth_checks::read_named(rel, "T_LOB3", &ReadOptions::default()).unwrap();
    let pos = |n: &str| names.iter().position(|x| x == n).unwrap();
    let proj: Rows = by_id(r3)
        .into_iter()
        .map(|x| {
            vec![
                x[0].clone(),
                x[pos("C")].clone(),
                x[pos("B")].clone(),
                x[pos("N")].clone(),
            ]
        })
        .collect();
    clean(truth_checks::t_lob3_diff(rel, &t3, &proj));
    let k = t3.iter().position(|l| l[1] == "5").unwrap();
    one(truth_checks::t_lob3_diff(
        rel,
        &mutated(&t3, k, 2, "8133", "8134"),
        &proj,
    ));
    let k = t3.iter().position(|l| l[1] == "20").unwrap();
    one(truth_checks::t_lob3_diff(
        rel,
        &mutated(&t3, k, 5, "98,80", "98,81"),
        &proj,
    ));
}

#[test]
fn break_t_part2_one_changed_truth_line_gives_one_difference() {
    let want = truth("18c", "truth_part2.txt");
    let rel = truth_checks::PART2_FILES[0];
    let mut d = open_fixture(rel);
    let info = d.table(TableRef::ByName("T_PART2")).unwrap().clone();
    let parts: Vec<(String, String, u64)> = info
        .parts
        .iter()
        .map(|p| {
            (
                p.partition.clone().unwrap(),
                p.subpartition.clone().unwrap(),
                p.num_rows,
            )
        })
        .collect();
    let (b, _) = try_read(&mut d, "T_PART2", &ReadOptions::default()).unwrap();
    let batches: Vec<Rows> = b.iter().map(|x| rows(std::slice::from_ref(x))).collect();
    clean(truth_checks::part2_diff(rel, &want, &parts, &batches));
    // a ROW line puts id 1 into another subpartition
    let k = want
        .iter()
        .position(|l| l[0] == "ROW" && l[1] == "1")
        .unwrap();
    let other = want
        .iter()
        .find(|l| l[0] == "SP" && l[2] != want[k][2])
        .unwrap()[2]
        .clone();
    let mut bad = want.clone();
    bad[k][2] = other;
    one(truth_checks::part2_diff(rel, &bad, &parts, &batches));
    // an SP line names another subpartition than the catalog
    let k = want.iter().position(|l| l[0] == "SP").unwrap();
    let mut bad = want.clone();
    bad[k][2] = format!("{}X", bad[k][2]);
    let d = truth_checks::part2_diff(rel, &bad, &parts, &batches);
    assert!(!d.is_empty(), "{d:?}");
}

#[test]
fn break_charsets_one_changed_line_gives_one_difference() {
    for v in ["18c", "11g"] {
        let l = truth(v, "truth_charsets.txt");
        clean(truth_checks::charsets_diff(v, &l));
        let mut bad = l.clone();
        bad[0][0] = bad[0][0].replace('=', "=1000");
        one(truth_checks::charsets_diff(v, &bad));
        let mut bad = l.clone();
        bad[2][0] = bad[2][0].replace("AL32UTF8", "AL32UTF9");
        one(truth_checks::charsets_diff(v, &bad));
    }
    let ru = truth("11g-cp1251", "truth_ru.txt");
    let sets = vec![("f".to_string(), 171u16)];
    clean(truth_checks::charsets_cp1251_diff(&ru, &sets));
    one(truth_checks::charsets_cp1251_diff(
        &ru,
        &[("f".to_string(), 873)],
    ));
    let k = ru
        .iter()
        .position(|l| l[0] == "ENV" && l[1].starts_with("NLS_CHARSET_ID"))
        .unwrap();
    one(truth_checks::charsets_cp1251_diff(
        &mutated(&ru, k, 2, "171", "172"),
        &sets,
    ));
}

#[test]
fn break_csmap_one_changed_line_gives_one_difference() {
    let a = truth_text("18c", "truth_csmap.txt");
    let b = truth_text("11g", "truth_csmap.txt");
    clean(truth_checks::csmap_diff(&a, &b));
    one(truth_checks::csmap_diff(
        &a,
        &b.replacen("Len=2: 0,1", "Len=2: 0,2", 1),
    ));
}

/// N-8: `truth_env.txt` against the header sets of every `_exp_` file of its folder (ids from
/// `truth_charsets.txt`), and the OBJECT TABLE lines of `truth_own_ddl.txt` against the catalog of `own_exp_<v>`.
#[test]
fn truth_env_and_truth_own_ddl_equal_the_headers_and_the_catalog() {
    let d = truth_checks::env_diffs();
    assert!(d.is_empty(), "{d:?}");
    let d = truth_checks::own_ddl_diffs();
    assert!(d.is_empty(), "{d:?}");
}

#[test]
fn break_env_and_own_ddl_one_changed_line_gives_one_difference() {
    for v in ["18c", "11g"] {
        let env = truth(v, "truth_env.txt");
        let cs = truth(v, "truth_charsets.txt");
        let sets = vec![("f".to_string(), 873u16, 2000u16)];
        clean(truth_checks::env_diff(v, &env, &cs, &sets));
        // a header with another database set, another national set
        one(truth_checks::env_diff(
            v,
            &env,
            &cs,
            &[("f".to_string(), 178, 2000)],
        ));
        one(truth_checks::env_diff(
            v,
            &env,
            &cs,
            &[("f".to_string(), 873, 2001)],
        ));
        // the env line names another set
        let k = env
            .iter()
            .position(|l| l[0].starts_with("NLS_CHARACTERSET="))
            .unwrap();
        let mut bad = env.clone();
        bad[k][0] = "NLS_CHARACTERSET=WE8MSWIN1252".to_string();
        one(truth_checks::env_diff(v, &bad, &cs, &sets));
        // a charsets line with another id
        let k = cs
            .iter()
            .position(|l| l[0].starts_with("AL32UTF8="))
            .unwrap();
        one(truth_checks::env_diff(
            v,
            &env,
            &mutated(&cs, k, 0, "=873", "=874"),
            &sets,
        ));

        let ddl = truth(v, "truth_own_ddl.txt");
        let tables: Vec<String> = ["O_CHILD", "O_EMPTY", "O_PARENT"]
            .map(String::from)
            .to_vec();
        clean(truth_checks::own_ddl_diff(v, &ddl, &tables));
        let k = ddl
            .iter()
            .position(|l| l[0] == "OBJECT" && l[1] == "TABLE")
            .unwrap();
        one(truth_checks::own_ddl_diff(
            v,
            &mutated(&ddl, k, 2, "O_CHILD", "O_CHILE"),
            &tables,
        ));
        one(truth_checks::own_ddl_diff(v, &ddl, &tables[..2]));
    }
}

#[test]
fn charsets_and_the_oracle_map_equal_the_readers_tables() {
    let d = truth_checks::charsets_diffs();
    assert!(d.is_empty(), "{d:?}");
    let d = truth_checks::csmap_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// T-3d (b): the two `ru_lob2_*` files add no warning to the inventory: the dump has none and R_LOB2 / R_TEXT
/// read with the default options give none (R_TEXT: `unconfirmed_lob` and every LOB warning absent; the files
/// have no table with a damaged name).
#[test]
fn the_ru_lob2_files_add_no_warning() {
    for rel in [
        "11g-cp1251/ru_lob2_exp_cp1251_11g.dmp",
        "11g-cp1251/ru_lob2_exp_us7ascii_11g.dmp",
    ] {
        let mut d = open_fixture(rel);
        assert!(d.warnings().is_empty(), "{rel}: {:?}", d.warnings());
        for table in ["R_LOB2", "R_TEXT"] {
            let (_, w) = rd(&mut d, table, &ReadOptions::default()).unwrap();
            assert!(w.is_empty(), "{rel} {table}: {w:?}");
        }
    }
}

/// ADR-43 item 5 (e3 / e4): for every tag of `checker_tag`, a first control (the file as it is gives an empty
/// list) and then `check_table` with one cell of the reader's rows changed in memory must give a non-empty
/// list through `union_of`. Deleting a comparison from a checker, or an `assert!` around it (there is none
/// left), makes this test fail for that tag.
#[test]
fn break_every_tag_of_check_table_one_changed_cell_gives_a_difference() {
    let sections = golden_sections();
    let mut tested = BTreeSet::new();
    for tag in CHECKER_TAGS {
        // T_STAT stands for four tables; the first one with rows is T_STAT1
        let (rel, table) = sections
            .iter()
            .find(|(_, t)| {
                if *tag == "T_STAT" {
                    t == "T_STAT1"
                } else {
                    t == tag
                }
            })
            .unwrap_or_else(|| panic!("no section for the tag {tag}"));
        let (folder, file) = rel.split_once('/').unwrap();
        for edit in [None, Some((0usize, 1usize))] {
            let mut d = open_fixture(rel);
            let mut c = Ctx::new(folder, file);
            c.edit = edit;
            let (got_tag, _, df) = check_table(&c, &mut d, table).unwrap();
            assert_eq!(got_tag, *tag);
            let u = union_of(&[(got_tag, df)]);
            if edit.is_none() {
                assert!(u.is_empty(), "{rel} {table}: {u:?}");
            } else {
                assert!(
                    !u.is_empty(),
                    "{rel} {table}: a changed cell gave no difference"
                );
                assert!(u.iter().all(|x| x.starts_with(&format!("{tag}: "))));
            }
        }
        tested.insert(*tag);
    }
    // every table name that has a checker maps to a tag that was tested
    for (_, table) in &sections {
        if let Some(tag) = checker_tag(table) {
            assert!(tested.contains(tag), "{table}");
        }
    }
}
