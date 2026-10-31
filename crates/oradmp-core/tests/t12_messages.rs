//! Task T-12 (f), ADR-43 item 1 (review S-1): no value in any message. Over the `message` field (not `Display`,
//! whose `offset=0x...` and integers would match hex by chance) of every error and warning that the 211 EXP
//! fixtures produce under `raise` and `null`, plus the copies below, the check searches
//!
//! * (a) the raw bytes of every field of the table read (fields of 4 bytes or more; a shorter one would match
//!   any word),
//! * (b) every run of 4 characters of the lossy UTF-8 rendering of every raw field (a run of digits only, or of
//!   white space only, is not counted: messages hold row numbers and set ids),
//! * (c) the hex form of every raw field of 2 bytes or more, lower and upper case, as a token bounded by non-hex
//!   characters; and (ADR-58 item 1) a run of 4 or more hex pairs separated by single ` `, `:` or `-` is joined into
//!   one token, and a token of 8 or more hex digits that is a substring of the hex of any field of the table is a
//!   hit too,
//! * (d) the reader's rendering of every cell of a lenient read (`on_value_error="null"`) of the table, 4
//!   characters or more, digit-only strings shorter than 6 and the NULL marker excluded,
//! * (e) (ADR-58 item 2) the decoded values: for every non-NULL field, by the column's type code, the NUMBER digit
//!   strings (a run of 6 or more digits of a message that is a substring) and canonical forms (a message token with
//!   a point or an exponent: `1E+33`, `1e33`, `1.0E+33`, `-0.5`), the date, time and interval texts of 8 characters
//!   or more. The value that fails is NULL in a lenient read, so (d) never sees it; (e) does.
//!
//! and finds none. The copies: `18c/misc_exp_18c.dmp` with one byte inside the VARCHAR2 value `NNNNNNNNNN`
//! (T_FRAME row 7, column A) set to 0xff (`E-BAD-TEXT` under `raise`, `W-VALUE-NULLED` under `null`), and the
//! 178 / 1 / 31 copies of `ru_lob2_exp_cp1251_11g.dmp` of T-6M (d) (`E-LOB-UNCONFIRMED`, whose message names the
//! database set id: an integer, allowed).
//!
//! The inventory assertion of ADR-58 item 3: the longest run of decimal digits in any message is at most 5, which
//! is why (e) counts runs of 6 or more.
//!
//! Proof that the check can fail: `the_check_reports_a_value_in_each_form` feeds it a real message with the
//! value appended in each form; the mutations d1 to d8 of the T-12 and T-13h notes in `implementer-notes.md`
//! each fail `no_message_of_the_fixtures_holds_a_value`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};

use common::*;
use oradmp_core::dump::{open, Dump, Options, TableRef};
use oradmp_core::exp::rows::{LobAction, LobForm, RowSink};
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::types::declared::DeclaredType;
use oradmp_core::values::datetime::decode_date_fields;
use oradmp_core::values::interval::{IntervalDs, IntervalYm};
use oradmp_core::values::number::{OraNumber, OverflowPolicy};
use oradmp_core::{Error, ErrorCode};

/// A form in which a value can reach a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Form {
    /// (a) raw bytes.
    Raw,
    /// (b) a run of the lossy UTF-8 rendering.
    Text,
    /// (c) hex.
    Hex,
    /// (d) a rendered cell.
    Cell,
    /// (e) a decoded value: a number's digits or canonical form, a date, time or interval text.
    Decoded,
}

/// What a message must not repeat: the values of one table.
#[derive(Default)]
struct Corpus {
    raw: BTreeSet<Vec<u8>>,
    grams: HashSet<String>,
    hex: HashSet<String>,
    /// The hex of every field of 2 bytes or more, for the substring test of form (c).
    hex_fields: BTreeSet<String>,
    cells: BTreeSet<String>,
    /// (e1) the digit strings of the numbers (sign, point and exponent removed).
    digit_strings: BTreeSet<String>,
    /// (e2) the canonical forms of the numbers: (negative, significant digits, exponent of the first one).
    canonical: BTreeSet<(bool, String, i64)>,
    /// (e3) the date, time and interval texts of 8 characters or more.
    stamps: BTreeSet<String>,
    /// (e3, ADR-65 item 4) the (year, month, day) of every stored date, for the date tokens of three groups.
    ymd: BTreeSet<(i64, u32, u32)>,
}

/// The canonical form of a decimal text (`-1.50E+3`, `1e33`, `1000`, `0.5`): sign, the digits without leading
/// and trailing zeros, and the decimal exponent of the first of them. `None` for a zero or a text that is not a
/// number.
fn canonical(t: &str) -> Option<(bool, String, i64)> {
    let (neg, t) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let (mant, exp) = match t.split_once(['e', 'E']) {
        Some((m, e)) => (m, e.parse::<i64>().ok()?),
        None => (t, 0),
    };
    let (int, frac) = mant.split_once('.').unwrap_or((mant, ""));
    if (int.is_empty() && frac.is_empty())
        || !(int.chars().chain(frac.chars()).all(|c| c.is_ascii_digit()))
    {
        return None;
    }
    let all = format!("{int}{frac}");
    let trimmed = all.trim_start_matches('0');
    let lead = (all.len() - trimmed.len()) as i64;
    let sig = trimmed.trim_end_matches('0');
    if sig.is_empty() {
        return None;
    }
    Some((neg, sig.to_string(), int.len() as i64 + exp - lead - 1))
}

/// The number tokens of a message that have a point or an exponent (`1E+33`, `1e33`, `1.0E+33`, `-0.5`).
fn number_tokens(msg: &str) -> Vec<String> {
    let b: Vec<char> = msg.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if !b[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        if i > 0 && b[i - 1].is_ascii_alphabetic() {
            // inside a word (`0x1e5`, `v2`): not a number token
            while i < b.len() && b[i].is_ascii_alphanumeric() {
                i += 1;
            }
            continue;
        }
        let start = if i > 0 && b[i - 1] == '-' { i - 1 } else { i };
        let mut j = i;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        let mut special = false;
        if j + 1 < b.len() && b[j] == '.' && b[j + 1].is_ascii_digit() {
            special = true;
            j += 1;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
        }
        if j < b.len() && (b[j] == 'e' || b[j] == 'E') {
            let mut k = j + 1;
            if k < b.len() && (b[k] == '+' || b[k] == '-') {
                k += 1;
            }
            if k < b.len() && b[k].is_ascii_digit() {
                while k < b.len() && b[k].is_ascii_digit() {
                    k += 1;
                }
                special = true;
                j = k;
            }
        }
        if special {
            out.push(b[start..j].iter().collect());
        }
        i = j.max(i + 1);
    }
    out
}

/// The `YYYY-MM-DD` text of a date (a sign for a year below 1, as in the truth files) and its `HH:MM:SS` text.
fn date_texts(b: &[u8]) -> Vec<String> {
    match decode_date_fields(b) {
        Ok(d) => vec![
            format!(
                "{}{:04}-{:02}-{:02}",
                if d.year < 1 { "-" } else { "" },
                d.year.abs(),
                d.month,
                d.day
            ),
            format!("{:02}:{:02}:{:02}", d.hour, d.minute, d.second),
        ],
        Err(_) => Vec::new(),
    }
}

/// The (year, month, day) of a DATE field, if the core decodes it (a year below 1 included).
fn date_ymd(b: &[u8]) -> Option<(i64, u32, u32)> {
    let d = decode_date_fields(b).ok()?;
    Some((i64::from(d.year), u32::from(d.month), u32::from(d.day)))
}

fn digits_only(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

impl Corpus {
    /// A field of a column with the type code `code` (the declared type, if known, fixes the interval precisions).
    fn add_typed_field(&mut self, code: u16, declared: Option<&DeclaredType>, f: &[u8]) {
        self.add_field(f);
        self.add_decoded(code, declared, f);
    }

    /// Form (e), ADR-58 item 2: what the decoders of the core make of the field. A field the decoder refuses
    /// adds nothing. The TIMESTAMP family shares the 7 date bytes of DATE, which `decode_date_fields` reads
    /// (it also reads a year below 1, which the timestamp decoders refuse and which is the value to catch).
    fn add_decoded(&mut self, code: u16, declared: Option<&DeclaredType>, f: &[u8]) {
        match code {
            2 => {
                if let Ok(n) = OraNumber::decode(f) {
                    let text = n.to_text();
                    let mantissa = text.split(['E', 'e']).next().unwrap_or("");
                    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
                    if !digits.is_empty() {
                        self.digit_strings.insert(digits);
                    }
                    if let Some(c) = canonical(&text) {
                        self.canonical.insert(c);
                    }
                }
            }
            12 => {
                self.add_stamps(date_texts(f));
                self.ymd.extend(date_ymd(f));
            }
            180 | 181 | 231 => {
                if let Some(head) = f.get(..7) {
                    self.add_stamps(date_texts(head));
                    self.ymd.extend(date_ymd(head));
                }
            }
            182 => {
                if let Ok(i) = IntervalYm::decode(f) {
                    let mut texts = vec![i.to_text(9)];
                    if let Some(DeclaredType::IntervalYearToMonth { precision }) = declared {
                        texts.push(i.to_text(*precision as usize));
                    }
                    self.add_stamps(texts);
                }
            }
            183 => {
                if let Ok(i) = IntervalDs::decode(f) {
                    let mut texts = vec![i.to_text(9, 9)];
                    if let Some(DeclaredType::IntervalDayToSecond {
                        precision,
                        fractional,
                    }) = declared
                    {
                        texts.push(i.to_text(*precision as usize, *fractional as usize));
                    }
                    self.add_stamps(texts);
                }
            }
            _ => {}
        }
    }

    fn add_stamps(&mut self, texts: Vec<String>) {
        for t in texts {
            if t.chars().count() >= 8 {
                self.stamps.insert(t);
            }
        }
    }

    fn add_field(&mut self, f: &[u8]) {
        if f.len() >= 4 {
            self.raw.insert(f.to_vec());
        }
        if f.len() >= 2 {
            self.hex.insert(hex(f));
            self.hex_fields.insert(hex(f));
        }
        let lossy: Vec<char> = String::from_utf8_lossy(f).chars().collect();
        for w in lossy.windows(4) {
            let g: String = w.iter().collect();
            if !digits_only(&g) && !g.chars().all(char::is_whitespace) {
                self.grams.insert(g);
            }
        }
    }

    fn add_cell(&mut self, c: &str) {
        if c.chars().count() >= 4 && c != "<NULL>" && !(digits_only(c) && c.len() < 6) {
            self.cells.insert(c.to_string());
        }
    }

    fn merge(&mut self, o: &Self) {
        self.raw.extend(o.raw.iter().cloned());
        self.grams.extend(o.grams.iter().cloned());
        self.hex.extend(o.hex.iter().cloned());
        self.hex_fields.extend(o.hex_fields.iter().cloned());
        self.cells.extend(o.cells.iter().cloned());
        self.digit_strings.extend(o.digit_strings.iter().cloned());
        self.canonical.extend(o.canonical.iter().cloned());
        self.stamps.extend(o.stamps.iter().cloned());
        self.ymd.extend(o.ymd.iter().copied());
    }
}

/// ADR-64 item 5 (c): the message without an exact trailing ` (piece N of the value)` / ` (chunk N of the value)`
/// (N decimal digits). The piece or chunk number is a counter that can exceed 5 digits, so the "at most 5 digits"
/// premise covers the rest of the message; everything else is checked as before. A suffix anywhere else, or with
/// text after it, is not removed.
fn without_position_suffix(msg: &str) -> &str {
    for what in ["piece", "chunk"] {
        let Some(head) = msg.strip_suffix(" of the value)") else {
            return msg;
        };
        let Some(open) = head.rfind(" (") else {
            return msg;
        };
        let Some(inner) = head.get(open + 2..) else {
            return msg;
        };
        if let Some(n) = inner.strip_prefix(what).and_then(|r| r.strip_prefix(' ')) {
            if digits_only(n) {
                return head.get(..open).unwrap_or(msg);
            }
        }
    }
    msg
}

/// The forms in which `msg` repeats a value of `c`. Empty for a clean message.
fn leaks(msg: &str, c: &Corpus) -> BTreeSet<Form> {
    let msg = without_position_suffix(msg);
    let mut found = BTreeSet::new();
    let bytes = msg.as_bytes();
    if c.raw
        .iter()
        .any(|f| bytes.windows(f.len()).any(|w| w == f.as_slice()))
    {
        found.insert(Form::Raw);
    }
    let chars: Vec<char> = msg.chars().collect();
    if chars
        .windows(4)
        .any(|w| c.grams.contains(&w.iter().collect::<String>()))
    {
        found.insert(Form::Text);
    }
    // maximal runs of hex digits
    let mut run = String::new();
    let mut tokens = Vec::new();
    for ch in msg.chars().chain(std::iter::once(' ')) {
        if ch.is_ascii_hexdigit() {
            run.push(ch.to_ascii_lowercase());
        } else {
            if !run.is_empty() {
                tokens.push(std::mem::take(&mut run));
            }
        }
    }
    // ADR-58 item 1: a run of 4 or more hex pairs separated by single ` `, `:` or `-` is one token
    tokens.extend(joined_pairs(msg));
    if tokens.iter().any(|t| {
        c.hex.contains(t) || (t.len() >= 8 && c.hex_fields.iter().any(|h| h.contains(t.as_str())))
    }) {
        found.insert(Form::Hex);
    }
    if c.cells.iter().any(|x| msg.contains(x.as_str())) {
        found.insert(Form::Cell);
    }
    if decoded_hit(msg, c) {
        found.insert(Form::Decoded);
    }
    found
}

/// The runs of 4 or more two-digit hex pairs separated by single ` `, `:` or `-`, joined, lower case.
fn joined_pairs(msg: &str) -> Vec<String> {
    let b: Vec<char> = msg.chars().collect();
    let pair =
        |i: usize| i + 1 < b.len() && b[i].is_ascii_hexdigit() && b[i + 1].is_ascii_hexdigit();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let bounded = i == 0 || !b[i - 1].is_ascii_hexdigit();
        if !(bounded && pair(i)) {
            i += 1;
            continue;
        }
        let mut joined = String::new();
        let mut n = 0;
        let mut j = i;
        loop {
            if !pair(j) || (j + 2 < b.len() && b[j + 2].is_ascii_hexdigit()) {
                break;
            }
            joined.extend(b[j..j + 2].iter().map(char::to_ascii_lowercase));
            n += 1;
            j += 2;
            if j + 2 < b.len() && matches!(b[j], ' ' | ':' | '-') && pair(j + 1) {
                j += 1;
            } else {
                break;
            }
        }
        if n >= 4 {
            out.push(joined);
        }
        i = j.max(i + 1);
    }
    out
}

/// The maximal runs of ASCII digits of `chars` as (start, end) index pairs.
fn digit_runs(chars: &[char]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        if !chars[i].is_ascii_digit() {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < chars.len() && chars[j].is_ascii_digit() {
            j += 1;
        }
        out.push((i, j));
        i = j;
    }
    out
}

/// ADR-65 item 4 (e1): the digit strings of the message. Every maximal run, and every run of digit groups joined:
/// groups separated by exactly one of `,` `.` `_` `'` or a space, every group after the first of exactly 3 digits
/// (`1,000,000`, `12 345.678`). The joined strings of two or more groups are returned second.
fn digit_strings_of(chars: &[char]) -> (Vec<String>, Vec<String>) {
    let runs = digit_runs(chars);
    let plain = runs
        .iter()
        .map(|&(a, b)| chars[a..b].iter().collect())
        .collect();
    let mut joined = Vec::new();
    let mut k = 0;
    while k < runs.len() {
        let mut end = k;
        while end + 1 < runs.len() {
            let (_, prev_end) = runs[end];
            let (next_start, next_end) = runs[end + 1];
            if next_start == prev_end + 1
                && matches!(chars[prev_end], ',' | '.' | '_' | '\'' | ' ')
                && next_end - next_start == 3
            {
                end += 1;
            } else {
                break;
            }
        }
        if end > k {
            joined.push(
                runs[k..=end]
                    .iter()
                    .flat_map(|&(a, b)| chars[a..b].iter())
                    .collect(),
            );
        }
        k = end + 1;
    }
    (plain, joined)
}

/// ADR-65 item 4 (e3): is there a token of three integer groups separated by the same `.`, `/` or `-` whose
/// (year, month, day) is a stored date, in the order Y-M-D, D.M.Y or M/D/Y? A group may carry a `-` sign (a year
/// below 1: `15.06.-100`). All three orders are tried for every separator.
fn date_token_hit(chars: &[char], c: &Corpus) -> bool {
    let runs = digit_runs(chars);
    let value = |(a, b): (usize, usize)| -> Option<i64> {
        if b - a > 9 {
            return None;
        }
        let n: i64 = chars[a..b].iter().collect::<String>().parse().ok()?;
        let negative =
            a > 0 && chars[a - 1] == '-' && (a < 2 || !chars[a - 2].is_ascii_alphanumeric());
        Some(if negative { -n } else { n })
    };
    for w in runs.windows(3) {
        let (r0, r1, r2) = (w[0], w[1], w[2]);
        // the separators are one character each, equal, and the third group may have a sign in front
        let sep = |from: (usize, usize), to: (usize, usize)| -> Option<char> {
            let (s0, s1) = (from.1, to.0);
            let sign_gap = s1 == s0 + 2 && chars[s0 + 1] == '-';
            let gap = s1 == s0 + 1 || sign_gap;
            let ch = *chars.get(s0)?;
            (gap && matches!(ch, '.' | '/' | '-')).then_some(ch)
        };
        let (Some(s1), Some(s2)) = (sep(r0, r1), sep(r1, r2)) else {
            continue;
        };
        if s1 != s2 {
            continue;
        }
        let (Some(a), Some(b), Some(cc)) = (value(r0), value(r1), value(r2)) else {
            continue;
        };
        // the month and day groups are never negative
        let (a_abs, b_abs, c_abs) = (a, b.abs(), cc);
        let orders = [
            (a_abs, b_abs, c_abs.abs()), // Y-M-D (the year is the first group)
            (c_abs, b_abs, a.abs()),     // D.M.Y: year last
            (c_abs, a.abs(), b_abs),     // M/D/Y: year last
        ];
        for (y, m, d) in orders {
            if let (Ok(m), Ok(d)) = (u32::try_from(m), u32::try_from(d)) {
                if c.ymd.contains(&(y, m, d)) {
                    return true;
                }
            }
        }
    }
    false
}

/// Form (e), ADR-58 item 2 and ADR-65 item 4: (e1) a digit string of 6 or more digits of the message (a run, or
/// digit groups joined) of which a window of 6 digits is a substring of a number's digit string, or a joined string
/// whose canonical form is a stored number's; (e2) a number token with a point or an exponent whose canonical form
/// is a stored number's; (e3) a stored date, time or interval text, or a date token of three integer groups.
fn decoded_hit(msg: &str, c: &Corpus) -> bool {
    let chars: Vec<char> = msg.chars().collect();
    let (plain, joined) = digit_strings_of(&chars);
    for run in plain.iter().chain(joined.iter()) {
        if run.len() >= 6 {
            let run: Vec<char> = run.chars().collect();
            for w in run.windows(6) {
                let w: String = w.iter().collect();
                if c.digit_strings.iter().any(|d| d.contains(w.as_str())) {
                    return true;
                }
            }
        }
    }
    if joined
        .iter()
        .filter_map(|j| canonical(j))
        .any(|k| c.canonical.contains(&k))
    {
        return true;
    }
    if number_tokens(msg)
        .iter()
        .filter_map(|t| canonical(t))
        .any(|k| c.canonical.contains(&k))
    {
        return true;
    }
    if date_token_hit(&chars, c) {
        return true;
    }
    c.stamps.iter().any(|t| msg.contains(t.as_str()))
}

/// Collects every non-NULL value of every row, chains and LOB values assembled.
#[derive(Default)]
struct Fields {
    /// (file position of the column, bytes)
    values: Vec<(u32, Vec<u8>)>,
    open: Vec<u8>,
    open_column: u32,
}

impl RowSink for Fields {
    fn row_start(&mut self, _: u64) {}
    fn null(&mut self, _: u32, _: u64) -> oradmp_core::Result<()> {
        Ok(())
    }
    fn value(&mut self, column: u32, _: u64, b: &[u8]) -> oradmp_core::Result<()> {
        self.values.push((column, b.to_vec()));
        Ok(())
    }
    fn chain_piece(
        &mut self,
        column: u32,
        _: u64,
        b: &[u8],
        first: bool,
        last: bool,
    ) -> oradmp_core::Result<()> {
        if first {
            self.open.clear();
        }
        self.open_column = column;
        self.open.extend_from_slice(b);
        if last {
            self.values.push((column, std::mem::take(&mut self.open)));
        }
        Ok(())
    }
    fn lob_start(
        &mut self,
        column: u32,
        _: u64,
        _: LobForm,
        _: u64,
    ) -> oradmp_core::Result<LobAction> {
        self.open.clear();
        self.open_column = column;
        Ok(LobAction::Read)
    }
    fn lob_chunk(&mut self, _: u32, _: u64, b: &[u8]) -> oradmp_core::Result<()> {
        self.open.extend_from_slice(b);
        Ok(())
    }
    fn lob_end(&mut self, _: u32) -> oradmp_core::Result<()> {
        self.values
            .push((self.open_column, std::mem::take(&mut self.open)));
        Ok(())
    }
    fn row_end(&mut self) -> oradmp_core::Result<()> {
        Ok(())
    }
}

/// The corpus of each table (by ordinal) of a file: the raw fields and the cells of a lenient read.
fn corpus_of(p: &Path) -> BTreeMap<u32, Corpus> {
    let mut d = open(p, Options::default()).unwrap();
    // per table: the type code and the declared type of each column, by file position
    type Cols = BTreeMap<u32, (u16, Option<DeclaredType>)>;
    let tables: Vec<(u32, String, usize, Cols)> = d
        .tables()
        .unwrap()
        .iter()
        .map(|t| {
            let cols = t
                .columns
                .iter()
                .map(|c| (c.file_position, (c.type_code, c.declared)))
                .collect();
            (t.ordinal, t.name.clone(), t.parts.len(), cols)
        })
        .collect();
    let lenient = ReadOptions {
        on_value_error: OnValueError::Null,
        on_number_overflow: OverflowPolicy::Round,
        ..ReadOptions::default()
    };
    let mut out = BTreeMap::new();
    for (ordinal, name, parts, cols) in tables {
        let mut f = Fields::default();
        for k in 0..parts {
            let mut cur = d.part_rows(TableRef::ByOrdinal(ordinal), k).unwrap();
            while cur.next_row_with(&mut f).unwrap().is_some() {}
        }
        let mut c = Corpus::default();
        for (column, v) in &f.values {
            match cols.get(column) {
                Some((code, declared)) => c.add_typed_field(*code, declared.as_ref(), v),
                None => c.add_field(v),
            }
        }
        // the cells of the lenient read; a table that cannot be read at all adds none
        if let Ok((batches, _)) = try_read_ordinal(&mut d, ordinal, &lenient) {
            for row in rows(&batches) {
                for cell in &row {
                    c.add_cell(cell);
                }
            }
        }
        let _ = name;
        out.insert(ordinal, c);
    }
    out
}

fn try_read_ordinal(
    d: &mut Dump,
    ordinal: u32,
    o: &ReadOptions,
) -> Result<(Vec<arrow_array::RecordBatch>, Vec<oradmp_core::Warning>), Error> {
    let mut r = d.read_table(TableRef::ByOrdinal(ordinal), o)?;
    let mut out = Vec::new();
    while let Some(b) = r.next_batch()? {
        out.push(b);
    }
    Ok((out, r.warnings().to_vec()))
}

/// One message with the table it belongs to (`None`: the scan or the header).
struct Msg {
    table: Option<u32>,
    text: String,
    /// The whole `Display` text (code, offset, message, context), for the NUL rule of ADR-61 item 3.
    shown: String,
}

/// The `message` of every error and warning of a file under the options: the scan, and every table read to its
/// end or its first error.
fn messages_of(p: &Path, strict: bool, ro: &ReadOptions) -> Vec<Msg> {
    let mut out = Vec::new();
    let o = Options {
        strict,
        ..Options::default()
    };
    let mut d = match open(p, o) {
        Ok(d) => d,
        Err(e) => {
            out.push(Msg {
                table: e.table,
                text: e.message.to_string(),
                shown: e.to_string(),
            });
            return out;
        }
    };
    match d.scan() {
        Err(e) => {
            out.push(Msg {
                table: e.table,
                text: e.message.to_string(),
                shown: e.to_string(),
            });
            return out;
        }
        // a stop of the scan (T-19: the R-048 cases) is an error of the catalog; the tables before it still read
        Ok(c) => {
            if let oradmp_core::dump::CatalogEnd::Stopped(e) = &c.end {
                out.push(Msg {
                    table: e.table,
                    text: e.message.to_string(),
                    shown: e.to_string(),
                });
            }
        }
    }
    for w in d.warnings() {
        out.push(Msg {
            table: w.table,
            text: w.message.to_string(),
            shown: w.to_string(),
        });
    }
    let ordinals: Vec<u32> = d.tables().unwrap().iter().map(|t| t.ordinal).collect();
    for ordinal in ordinals {
        let mut r = match d.read_table(TableRef::ByOrdinal(ordinal), ro) {
            Ok(r) => r,
            Err(e) => {
                out.push(Msg {
                    table: Some(ordinal),
                    text: e.message.to_string(),
                    shown: e.to_string(),
                });
                continue;
            }
        };
        loop {
            match r.next_batch() {
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(e) => {
                    out.push(Msg {
                        table: Some(ordinal),
                        text: e.message.to_string(),
                        shown: e.to_string(),
                    });
                    break;
                }
            }
        }
        for w in r.warnings() {
            out.push(Msg {
                table: Some(ordinal),
                text: w.message.to_string(),
                shown: w.to_string(),
            });
        }
    }
    out
}

/// `18c/misc_exp_18c.dmp` with one byte of the VARCHAR2 value `NNNNNNNNNN` (T_FRAME row 7, column A) set to
/// 0xff: not UTF-8. Bytes asserted before patching.
fn text_valued_bad_field(name: &str) -> PathBuf {
    patched_file("18c/misc_exp_18c.dmp", "t12", name, |b| {
        let at = find_one(b, b"NNNNNNNNNN");
        assert_eq!(&b[at - 2..at], &[10, 0], "the length field of the value");
        b[at + 4] = 0xff;
    })
}

/// The 178 / 1 / 31 copies of T-6M (d): `ru_lob2_exp_cp1251_11g.dmp` with the header database set @0x36 changed
/// (and for 178 the CLOB descriptor set @0x24e3). Bytes asserted before patching.
fn lob2_copy(name: &str, db_low: u8, clob_low: Option<u8>) -> PathBuf {
    patched_file("11g-cp1251/ru_lob2_exp_cp1251_11g.dmp", "t12", name, |b| {
        assert_eq!(b[0x35..0x37], [0x00, 0xab]);
        assert_eq!(b[0x24e3..0x24e5], [0xab, 0x00]);
        b[0x36] = db_low;
        if let Some(c) = clob_low {
            b[0x24e3] = c;
        }
    })
}

fn policies() -> Vec<(bool, ReadOptions)> {
    let raise = ReadOptions::default();
    let null = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let null_numbers = ReadOptions {
        on_value_error: OnValueError::Null,
        on_number_overflow: OverflowPolicy::Null,
        ..ReadOptions::default()
    };
    vec![
        (false, raise.clone()),
        (true, raise),
        (false, null),
        (false, null_numbers),
    ]
}

/// ADR-61 item 3: arrow-array's C stream export turns the message into a `CString` with `expect(...)`, so a NUL in
/// any error or warning text would abort the process.
fn assert_no_nul(shown: &str, label: &str) {
    assert!(
        !shown.contains('\u{0}'),
        "{label}: an error or warning text holds U+0000: {shown:?}"
    );
}

/// The messages of one file against the corpus of its tables; returns the number of messages looked at and the
/// codes seen at the start of the code field (not part of the message, so the kinds are counted by the
/// message text).
fn check_file(p: &Path, label: &str, seen: &mut BTreeSet<String>) -> usize {
    check_file_against(p, p, label, seen)
}

/// As [`check_file`], with the values to look for taken from `values_of`: for a copy whose scan fails in `Framing`
/// (T-19, the chunk length from 0xfff0) the fields are those of the unpatched file.
fn check_file_against(
    p: &Path,
    values_of: &Path,
    label: &str,
    seen: &mut BTreeSet<String>,
) -> usize {
    let corpus = corpus_of(values_of);
    let mut all = Corpus::default();
    for c in corpus.values() {
        all.merge(c);
    }
    let mut n = 0;
    for (strict, ro) in policies() {
        for m in messages_of(p, strict, &ro) {
            let c = m.table.and_then(|t| corpus.get(&t)).unwrap_or(&all);
            let found = leaks(&m.text, c);
            assert!(
                found.is_empty(),
                "{label}: a message repeats a value ({found:?}): {}",
                m.text
            );
            seen.insert(m.text);
            // ADR-61 item 3 (T-14 h): the text the Arrow C stream turns into a CString holds no U+0000
            assert_no_nul(&m.shown, label);
            n += 1;
        }
    }
    n
}

#[test]
fn no_message_of_the_fixtures_holds_a_value() {
    let mut seen = BTreeSet::new();
    let mut n = 0;
    for rel in exp_files() {
        n += check_file(&fixtures().join(&rel), &rel, &mut seen);
    }
    for (name, db, clob) in [
        ("m_lob2_178.dmp", 0xb2u8, Some(0xb2u8)),
        ("m_lob2_h1.dmp", 0x01, None),
        ("m_lob2_h31.dmp", 0x1f, None),
    ] {
        n += check_file(&lob2_copy(name, db, clob), name, &mut seen);
    }
    n += check_file(
        &text_valued_bad_field("bad_text_all.dmp"),
        "bad_text_value",
        &mut seen,
    );
    // ADR-64 item 5 (a): the two owned messages with a runtime integer, the chain copy (piece 2) and the chunk copy
    // (chunk 2), made by the helpers that the offset tests use
    n += check_file(
        &chain_cut_copy("t12", "m_chain_cut.dmp"),
        "chain_cut",
        &mut seen,
    );
    n += check_file(
        &lob_chunk_cut_copy("t12", "m_lob_cut.dmp").0,
        "lob_chunk_cut",
        &mut seen,
    );
    // T-19b (ADR-99 item 3, ADR-103; the retired R-048 texts are gone): the texts of the stops and the framing errors
    // of the conventional LONG / LONG RAW forms, from patched copies of the `long3` / `long4` / `long` fixtures:
    // `fb ff` and `fc ff` (stop), the n and z of the 4-byte length, the chunk length from 0xfff0, and 0xfff2
    for (name, file, at, bytes) in [
        (
            "m_long4_fbff.dmp",
            "18c/long4_exp_18c.dmp",
            0x22277usize,
            [0xfb, 0xff],
        ),
        (
            "m_long4_n_f9ff.dmp",
            "18c/long4_exp_18c.dmp",
            0x22279,
            [0xf9, 0xff],
        ),
        (
            "m_long4_z_0001.dmp",
            "18c/long4_exp_18c.dmp",
            0x2227b,
            [0x01, 0x00],
        ),
        (
            "m_long3_fcff.dmp",
            "18c/long3_exp_18c.dmp",
            0x2228f,
            [0xfc, 0xff],
        ),
        (
            "m_longraw_fbff.dmp",
            "18c/long_exp_18c.dmp",
            0xa0ea,
            [0xfb, 0xff],
        ),
        (
            "m_chunk_f0ff.dmp",
            "18c/long3_exp_18c.dmp",
            0x32291,
            [0xf0, 0xff],
        ),
    ] {
        let p = patched_file(file, "t12", name, |b| {
            b[at..at + 2].copy_from_slice(&bytes);
        });
        n += check_file_against(&p, &fixtures().join(file), name, &mut seen);
    }
    n += check_file(
        &long4_fff2_copy("t12", "m_long4_fff2.dmp").0,
        "long4_fff2",
        &mut seen,
    );
    n += check_file(
        &long3_100001_copy("t12", "m_long_100001.dmp").0,
        "long_100001",
        &mut seen,
    );
    n += check_file(
        &long3_direct_100001_copy("t12", "m_long_direct_100001.dmp").0,
        "long_direct_100001",
        &mut seen,
    );
    n += check_file(
        &long3_chunked_short_copy("t12", "m_chunked_short.dmp").0,
        "chunked_short",
        &mut seen,
    );
    for part in [
        "conventional LONG or LONG RAW field starting 0xfffb or 0xfffc, not in the fixtures [R8]",
        "conventional LONG or LONG RAW 4-byte length outside 65,530 to 65,533",
        "conventional LONG or LONG RAW 4-byte length whose upper two bytes are not 00 00",
        "conventional LONG or LONG RAW chunk length from 0xfff0",
        "conventional LONG or LONG RAW length 0xfff2 to 0xfff4 read as one field, not in the fixtures (R-052)",
        "conventional LONG or LONG RAW in the chunked form with fewer than 65,534 bytes",
        "LONG value above 100,000 bytes, the largest confirmed",
    ] {
        assert!(seen.contains(part), "no message `{part}`: {seen:#?}");
    }
    // the two retired R-048 texts are gone (ADR-99 item 3)
    for retired in [
        // written in two pieces so that no source line holds a retired text whole
        [
            "conventional LONG length 0xfff1 to 0xfffc,",
            " not in the fixtures (R-048)",
        ]
        .concat(),
        [
            "conventional LONG RAW length from 0x8000 or chunked form,",
            " not in the fixtures (R-048)",
        ]
        .concat(),
    ] {
        assert!(!seen.contains(&retired), "retired text seen: {retired}");
    }
    assert!(
        seen.iter().any(|m| m.ends_with(" (piece 2 of the value)")),
        "no message ends with the piece suffix"
    );
    assert!(
        seen.iter().any(|m| m.ends_with(" (chunk 2 of the value)")),
        "no message ends with the chunk suffix"
    );
    // ADR-58 item 3 (amended by ADR-64 item 5 (c)): the longest run of decimal digits in any message, without its
    // piece / chunk suffix, is at most 5; a message that needs a longer integer fails here first and needs a
    // decision (form (e) counts runs of 6 or more)
    for m in &seen {
        let longest = without_position_suffix(m)
            .split(|ch: char| !ch.is_ascii_digit())
            .map(str::len)
            .max()
            .unwrap_or(0);
        assert!(longest <= 5, "a run of {longest} digits in a message: {m}");
    }
    // the corpus of messages is not empty and has the codes that carry values' neighbours
    assert!(n > 150, "{n} messages");
    assert!(
        seen.len() >= 14,
        "{} distinct messages: {seen:#?}",
        seen.len()
    );
    for part in [
        "integer part does not fit",
        "no fixture confirms",
        "set to NULL",
        "rounded",
        "year below 1",
        "text is not valid UTF-8",
        "months do not fit",
    ] {
        assert!(
            seen.iter().any(|m| m.contains(part)),
            "no message with `{part}`"
        );
    }
}

/// The patched copy produces the text-valued errors the corpus needs: `E-BAD-TEXT` under `raise` at the value,
/// `W-VALUE-NULLED` under `null`, and the field is in the corpus of its table.
#[test]
fn the_text_valued_copy_gives_a_bad_text_error_and_a_nulled_warning() {
    let p = text_valued_bad_field("bad_text_copy.dmp");
    let mut d = open_path(&p, Options::default());
    let e = try_read(&mut d, "T_FRAME", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadText);
    assert_eq!((e.row, e.column), (Some(7), Some(1)));
    let null = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let (_, w) = read(&mut d, "T_FRAME", &null);
    assert!(w
        .iter()
        .any(|w| w.code == oradmp_core::WarningCode::ValueNulled && w.row == Some(7)));
    let corpus = corpus_of(&p);
    let bad = b"NNNN\xffNNNNN";
    assert!(corpus.values().any(|c| c.raw.contains(&bad[..])));
}

/// The check can fail (ADR-43 item 1): a real fixture message with the value appended in each form is
/// reported in that form. The value is `NNNN\xffNNNNN` (the patched T_FRAME value, 10 bytes); for (d) the
/// NUMBER `1234567` rendered by the reader, which is not any raw field of the table.
#[test]
fn the_check_reports_a_value_in_each_form() {
    let p = text_valued_bad_field("bad_text_forms.dmp");
    let mut c = Corpus::default();
    // the copy (the damaged value) and the fixture it was made from (the intact one, for form (a))
    for q in [p.clone(), fixtures().join("18c/misc_exp_18c.dmp")] {
        for t in corpus_of(&q).values() {
            c.merge(t);
        }
    }
    // a real message: the text-valued error of the copy
    let mut d = open_path(&p, Options::default());
    let real = try_read(&mut d, "T_FRAME", &ReadOptions::default())
        .unwrap_err()
        .message
        .to_string();
    assert!(leaks(&real, &c).is_empty(), "{real}");

    let value = b"NNNN\xffNNNNN";
    // (a) the raw bytes: not valid UTF-8, so the message is built from bytes with the lossy rendering of
    // everything but a pure-ASCII 5-byte field; use a pure-ASCII field of the table for the raw form
    let ascii = b"NNNNNNNNNN";
    let m = format!("{real} {}", std::str::from_utf8(ascii).unwrap());
    let f = leaks(&m, &c);
    assert!(f.contains(&Form::Raw), "{f:?}");
    // (b) a run of the lossy rendering of the damaged value
    let lossy = String::from_utf8_lossy(value).into_owned();
    let m = format!("{real} {lossy}");
    let f = leaks(&m, &c);
    assert!(f.contains(&Form::Text), "{f:?}");
    assert!(!f.contains(&Form::Raw), "{f:?}");
    // (c) hex of the damaged value, both cases
    for h in [hex(value), hex(value).to_uppercase()] {
        let m = format!("{real} bytes {h}.");
        let f = leaks(&m, &c);
        assert_eq!(f, BTreeSet::from([Form::Hex]), "{h}");
    }
    // a hex string inside a longer hex run is not a token of its own (bounded by non-hex characters)
    // (the digits `04e4` read as a number token with an exponent, so form (e) may report it; (c) must not)
    let m = format!("{real} 0{}0", hex(value));
    assert!(!leaks(&m, &c).contains(&Form::Hex));
    // (d) a rendered cell that is no raw field: the NUMBER of `t_num_exp_18c.dmp` row 1 renders as text
    let q = fixtures().join("18c/t_num_exp_18c.dmp");
    let cn = corpus_of(&q);
    let mut cc = Corpus::default();
    for t in cn.values() {
        cc.merge(t);
    }
    let cell = cc
        .cells
        .iter()
        .find(|s| digits_only(s) && s.len() >= 6 && !cc.hex.contains(*s))
        .cloned()
        .expect("a NUMBER cell of 6 digits or more");
    let m = format!("{real} value {cell} here");
    // (the cell is a NUMBER: form (e) sees its digits too)
    assert!(leaks(&m, &cc).contains(&Form::Cell), "{cell}");
    // ADR-64 item 5 (c): a value inside the message before a position suffix is still found, and so is one after it
    let m = format!("{real} NNNNNNNNNN (piece 2 of the value)");
    assert!(leaks(&m, &c).contains(&Form::Raw), "{m}");
    let m = format!("{real} (piece 2 of the value) NNNNNNNNNN");
    assert!(leaks(&m, &c).contains(&Form::Raw), "{m}");
    // short digit strings are row numbers and ids: not a leak
    let m = format!("{real} row 1234 table 12 id 873");
    assert!(leaks(&m, &c).is_empty());
    // (c) stronger, ADR-58 item 1: the first 8 bytes of the damaged value (a prefix of a longer field: not the
    // hex of a whole field), plain and with pairs separated by a space, a colon or a dash; fewer than 4 pairs or
    // fewer than 8 digits do not count
    let first8 = hex(&value[..8]);
    assert_eq!(first8, "4e4e4e4eff4e4e4e");
    for m in [
        format!("{real} {first8}"),
        format!("{real} 4e 4e 4e 4e ff"),
        format!("{real} 4e:4e:4e:4e:ff:4e"),
        format!("{real} 4E-4E-4E-4E-FF"),
    ] {
        assert_eq!(leaks(&m, &c), BTreeSet::from([Form::Hex]), "{m}");
    }
    for m in [
        format!("{real} 4e 4e 4e"),          // 3 pairs: below the 4 pairs of a run
        format!("{real} {}", &first8[..6]),  // 6 digits: below 8
        format!("{real} 4e  4e  4e  4e ff"), // two spaces do not join
    ] {
        assert!(leaks(&m, &c).is_empty(), "{m}");
    }
}

/// The text of column `column` (0-based) of row `row` (1-based) of a truth file.
fn truth_cell(folder: &str, name: &str, row: usize, column: usize) -> String {
    let rows = truth(folder, name);
    let r = rows
        .iter()
        .find(|r| r[0] == row.to_string())
        .unwrap_or_else(|| panic!("row {row} of {name}"));
    r[column].clone()
}

/// Form (e), ADR-58 item 4: the value that fails is NULL in a lenient read, so only the decoded forms see it.
/// `E-NUMBER-OVERFLOW` of `18c/t_num_exp_18c.dmp` T_NUM with the value of truth_num row 19 (10^33) appended in
/// four spellings, a 6-digit and a 5-digit window of row 22; `E-BAD-DATETIME` of `18c/t_date_exp_18c.dmp` T_DATE
/// with the date text of truth_date row 8 (year -100).
#[test]
fn the_check_reports_a_decoded_value() {
    let q = fixtures().join("18c/t_num_exp_18c.dmp");
    let mut cc = Corpus::default();
    for t in corpus_of(&q).values() {
        cc.merge(t);
    }
    let mut d = open_path(&q, Options::default());
    let real = try_read(&mut d, "T_NUM", &ReadOptions::default())
        .unwrap_err()
        .message
        .to_string();
    assert!(leaks(&real, &cc).is_empty(), "{real}");
    let v19 = truth_cell("18c", "truth_num.txt", 19, 1);
    assert_eq!(v19.len(), 34, "10^33 in decimal"); // truth_num row 19: `1` and 33 zeros
    let exp = v19.len() - 1;
    for spelling in [
        v19.clone(),
        format!("1E+{exp}"),
        format!("1e{exp}"),
        format!("1.0E+{exp}"),
    ] {
        let m = format!("{real} value {spelling} here");
        let f = leaks(&m, &cc);
        assert!(f.contains(&Form::Decoded), "{spelling}: {f:?}");
    }
    // row 22 is -12345678901234567890123456789012345678: a 6-digit window reports (e), a 5-digit one does not
    let v22 = truth_cell("18c", "truth_num.txt", 22, 1);
    let digits = v22.trim_start_matches('-');
    let m = format!("{real} {}", &digits[3..9]);
    assert!(leaks(&m, &cc).contains(&Form::Decoded), "{m}");
    let m = format!("{real} {}", &digits[3..8]);
    assert!(!leaks(&m, &cc).contains(&Form::Decoded), "{m}");
    // d10 (ADR-65 item 4): the failing value 10^33 in digit groups of 3 with each of the separators; the groups
    // of 5 are not a grouping of the rule (and 6-digit windows of 10^33 are not in any stored digit string)
    let grouped = |sep: &str| {
        let head = &v19[..1];
        let tail: Vec<&str> = (0..11).map(|_| "000").collect();
        format!("{head}{sep}{}", tail.join(sep))
    };
    for sep in [",", ".", "_", "'", " "] {
        let g = grouped(sep);
        assert_eq!(g.matches(sep).count(), 11, "{g}");
        let m = format!("{real} value {g} here");
        assert!(leaks(&m, &cc).contains(&Form::Decoded), "{g}");
    }
    // groups of 2 digits are not joined: the digit runs alone (at most 2 digits) do not name the number
    let m = format!("{real} value 1,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00,00 here");
    assert!(!leaks(&m, &cc).contains(&Form::Decoded), "{m}");
    // a negative number with a point or an exponent: its canonical form
    let m = format!(
        "{real} {}",
        format_args!("-1.2345678901234567890123456789012345678E+{}", 37)
    );
    assert!(leaks(&m, &cc).contains(&Form::Decoded), "{m}");

    // the date text of truth_date row 8 (year below 1) appended to the real E-BAD-DATETIME message
    let p = fixtures().join("18c/t_date_exp_18c.dmp");
    let mut cd = Corpus::default();
    for t in corpus_of(&p).values() {
        cd.merge(t);
    }
    let mut d = open_path(&p, Options::default());
    let e = try_read(&mut d, "T_DATE", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadDatetime);
    let real = e.message.to_string();
    assert!(leaks(&real, &cd).is_empty(), "{real}");
    let row8 = truth_cell("18c", "truth_date.txt", 8, 1);
    let date = row8.split(' ').next().unwrap().to_string();
    assert_eq!(date.len(), 11, "{row8}"); // `-0100-06-15`: a sign, 4 digits of year
    let m = format!("{real} ({date})");
    assert_eq!(leaks(&m, &cd), BTreeSet::from([Form::Decoded]), "{m}");
    // d9 (ADR-65 item 4): the same date as three integer groups, in the three orders (year -100, 15 June)
    for token in [
        "15.06.-100",
        "15.6.-100",
        "06/15/-100",
        "-100-06-15",
        "15/06/-100",
    ] {
        let m = format!("{real} ({token})");
        assert_eq!(leaks(&m, &cd), BTreeSet::from([Form::Decoded]), "{m}");
    }
    // not the stored date: another year, another day, a month above 12 in no order, different separators
    for token in ["15.06.-101", "16.06.-100", "15.06/-100", "13.13.-100"] {
        let m = format!("{real} ({token})");
        assert!(leaks(&m, &cd).is_empty(), "{m}: {:?}", leaks(&m, &cd));
    }
}

#[test]
fn the_helpers_of_the_decoded_forms() {
    assert_eq!(
        canonical("1E+33"),
        canonical("1000000000000000000000000000000000")
    );
    assert_eq!(canonical("1e33"), canonical("1.0E+33"));
    assert_ne!(canonical("-0.5"), canonical("0.5"));
    assert_eq!(canonical("0.5"), canonical("5E-1"));
    assert_eq!(canonical("0.0"), None);
    assert_eq!(
        number_tokens("a 1E+33, b 1e33; c -0.5 d 12 e 0x1e5 f 1.0E+33"),
        vec!["1E+33", "1e33", "-0.5", "1.0E+33"]
    );
    // ADR-64 item 5 (c): only an exact trailing suffix is removed
    assert_eq!(
        without_position_suffix("a message (piece 2 of the value)"),
        "a message"
    );
    assert_eq!(
        without_position_suffix("a message (chunk 123456 of the value)"),
        "a message"
    );
    for kept in [
        "a message (piece of the value)",
        "a message (piece x of the value)",
        "a message (piece 2 of the value) more",
        "a message (block 2 of the value)",
        "a message piece 2 of the value)",
        "(piece 2 of the value)x",
    ] {
        assert_eq!(without_position_suffix(kept), kept);
    }
    assert_eq!(joined_pairs("x 4e 4e 4e 4e ff y"), vec!["4e4e4e4eff"]);
    assert!(joined_pairs("4e 4e 4e").is_empty());
}

/// ADR-40 N-23 (1): `E-LOB-UNCONFIRMED` names the database set id of the header, for the 178, 1 and 31 copies
/// of T-6M (d), and no other id.
#[test]
fn the_lob_unconfirmed_message_names_the_database_set_id() {
    for (name, db, clob, id) in [
        ("n_lob2_178.dmp", 0xb2u8, Some(0xb2u8), 178u16),
        ("n_lob2_h1.dmp", 0x01, None, 1),
        ("n_lob2_h31.dmp", 0x1f, None, 31),
    ] {
        let p = lob2_copy(name, db, clob);
        let mut d = open_path(&p, Options::default());
        assert_eq!(d.info().db_charset_id, id);
        let e = try_read(&mut d, "R_LOB2", &ReadOptions::default()).unwrap_err();
        assert_eq!(e.code, ErrorCode::LobUnconfirmed, "{name}");
        let want = format!("database character set id {id} of the header");
        assert!(e.message.contains(&want), "{name}: {}", e.message);
        for other in [178u16, 1, 31] {
            if other != id {
                assert!(
                    !e.message.contains(&format!("set id {other} ")),
                    "{name}: {}",
                    e.message
                );
            }
        }
        assert!(e.to_string().contains("unconfirmed_lob"), "{name}");
    }
}

/// T-14 (h), ADR-61 item 3: the rule is checked over the `Display` text (what the stream carries) of every error
/// and warning of the fixture files (`no_message_of_the_fixtures_holds_a_value` runs `assert_no_nul` on each of
/// them); this test shows that the check itself fails on a NUL, and that the texts it sees start with the code.
#[test]
fn the_nul_check_fails_on_a_nul_and_the_texts_are_the_display_texts() {
    let r =
        std::panic::catch_unwind(|| assert_no_nul("[E-FRAMING offset=0x1] a\u{0}b", "selftest"));
    assert!(r.is_err());
    assert_no_nul("[E-FRAMING offset=0x1] ab", "selftest");
    let p = text_valued_bad_field("nul_selftest.dmp");
    let ms = messages_of(&p, false, &ReadOptions::default());
    assert!(!ms.is_empty());
    for m in &ms {
        assert!(
            m.shown.starts_with("[E-") || m.shown.starts_with("[W-"),
            "{}",
            m.shown
        );
        assert!(
            m.shown.contains(&m.text),
            "the display text holds the message"
        );
    }
}
