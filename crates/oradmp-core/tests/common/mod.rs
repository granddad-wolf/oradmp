//! Helpers of the T-6 tests: fixture access, reading a table to batches, cells as text, a test-only NUMBER
//! encoder and the civil-date arithmetic. Test code only.
#![allow(
    dead_code,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

pub mod synth;
pub mod truth_checks;

use std::path::{Path, PathBuf};

use arrow_array::cast::AsArray;
use arrow_array::types::{
    Decimal128Type, Decimal256Type, DurationMicrosecondType, DurationNanosecondType, Int64Type,
    IntervalMonthDayNanoType, TimestampMicrosecondType, TimestampNanosecondType,
    TimestampSecondType,
};
use arrow_array::{Array, RecordBatch};
use arrow_schema::{DataType, IntervalUnit, TimeUnit};
use oradmp_core::dump::{open, Dump, Options, TableRef};
use oradmp_core::reader::{ReadOptions, TableBatches};
use oradmp_core::{Error, Warning};

/// The totals of the Analyst for the EXP fixtures, written down once (rules file v3.9;
/// `docs/formats/tools/corpus_counts.py` re-run read-only in T-19b, per folder and in total: 229 files = 105 / 97 /
/// 27, 436 sections, `problems 0`, 41,494 rows, 243,905 fields, 20,084 NULL, 27 + 20 empty tables;
/// `rowlen_stats.py` for the 450 parts and the LOB values, `num_decode.py` step 3 for the 86,236 NUMBER fields,
/// `collist.py` for the 2,075 column definitions). Every other test derives its counts from the golden files; the single test
/// `tool_table_reproduces_the_plan_numbers_for_the_exp_files` (t1_walk.rs) compares this table with them.
pub struct FolderTotals {
    pub folder: &'static str,
    pub files: usize,
    pub sections: usize,
    /// binary parts (`INSERT INTO` lines)
    pub parts: usize,
    pub rows: u64,
    pub fields: u64,
    pub nulls: u64,
    /// fields after an introducing line (`text_fields` of corpus_counts.py)
    pub text_fields: u64,
    /// sections with a CLOB column (type 112): the `codes` entry 112 of corpus_counts.py. Every section with
    /// a type 113 column also has one of type 112 in this corpus, so the count of sections with 112 or 113
    /// (21 over the three folders, `rowlen_stats.py` `lob_sections`) is the same number.
    pub lob_sections: usize,
}

pub const FOLDER_TOTALS: [FolderTotals; 3] = [
    FolderTotals {
        folder: "18c",
        files: 105,
        sections: 200,
        parts: 214,
        rows: 9_673,
        fields: 32_055,
        nulls: 2_356,
        text_fields: 263,
        lob_sections: 6,
    },
    FolderTotals {
        folder: "11g",
        files: 97,
        sections: 184,
        parts: 184,
        rows: 3_445,
        fields: 13_371,
        nulls: 2_324,
        text_fields: 223,
        lob_sections: 6,
    },
    FolderTotals {
        folder: "11g-cp1251",
        files: 27,
        sections: 52,
        parts: 52,
        rows: 28_376,
        fields: 198_479,
        nulls: 15_404,
        text_fields: 297,
        lob_sections: 9,
    },
];

pub struct CorpusTotals {
    pub files: usize,
    pub sections: usize,
    pub parts: usize,
    pub rows: u64,
    pub fields: u64,
    pub nulls: u64,
    pub text_fields: u64,
    /// empty tables, conventional / direct path (corpus_counts.py `empty_conventional`, `empty_direct`)
    pub empty_conv: u64,
    pub empty_direct: u64,
    pub lob_sections: usize,
    /// LOB values after the rows: 126 = 80 short + 46 long (`rowlen_stats.py`, session 10)
    pub lob_values: u64,
    pub lob_short: u64,
    pub lob_long: u64,
    /// NUMBER fields (`num_decode.py` step 3, session 10)
    pub number_fields: u64,
    /// column definitions of the 436 `CREATE TABLE` lines (`collist.py`: "column definitions checked against
    /// descriptors 2075"; review N-2)
    pub column_definitions: u64,
}

pub const TOTALS: CorpusTotals = CorpusTotals {
    files: 229,
    sections: 436,
    parts: 450,
    rows: 41_494,
    fields: 243_905,
    nulls: 20_084,
    text_fields: 783,
    empty_conv: 27,
    empty_direct: 20,
    lob_sections: 21,
    lob_values: 126,
    lob_short: 80,
    lob_long: 46,
    number_fields: 86_236,
    column_definitions: 2_075,
};

/// Data lines of a TSV below the repository root: no `#` comment line, no header line.
pub fn golden_data_lines(rel: &str) -> usize {
    std::fs::read_to_string(root().join(rel))
        .unwrap()
        .lines()
        .filter(|l| !l.starts_with('#') && !l.is_empty())
        .count()
        - 1
}

pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub fn fixtures() -> PathBuf {
    root().join("corpus/fixtures")
}

/// A scratch directory below `target/`.
pub fn target_dir(name: &str) -> PathBuf {
    let d = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/t6-tests")
        .join(name);
    std::fs::create_dir_all(&d).unwrap();
    d
}

pub fn open_fixture(rel: &str) -> Dump {
    open(fixtures().join(rel), Options::default()).unwrap()
}

pub fn open_path(p: &Path, options: Options) -> Dump {
    open(p, options).unwrap()
}

/// Reads a table to the end; the typed error of the first failure is returned.
pub fn try_read(
    d: &mut Dump,
    table: &str,
    o: &ReadOptions,
) -> Result<(Vec<RecordBatch>, Vec<Warning>), Error> {
    let mut r: TableBatches = d.read_table(TableRef::ByName(table), o)?;
    let mut out = Vec::new();
    while let Some(b) = r.next_batch()? {
        out.push(b);
    }
    Ok((out, r.warnings().to_vec()))
}

pub fn read(d: &mut Dump, table: &str, o: &ReadOptions) -> (Vec<RecordBatch>, Vec<Warning>) {
    try_read(d, table, o).unwrap()
}

pub fn read_default(rel: &str, table: &str) -> (Vec<RecordBatch>, Vec<Warning>) {
    read(&mut open_fixture(rel), table, &ReadOptions::default())
}

/// Days since 1970-01-01 to (year, month, day), proleptic Gregorian (the usual civil-from-days arithmetic).
pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let mp = i64::from(if m > 2 { m - 3 } else { m + 9 });
    let doy = (153 * mp + 2) / 5 + i64::from(d) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// `YYYY-MM-DD HH:MM:SS` of seconds since the epoch.
pub fn ts_text(secs: i64) -> String {
    let (y, m, d) = civil_from_days(secs.div_euclid(86_400));
    let s = secs.rem_euclid(86_400);
    format!(
        "{y:04}-{m:02}-{d:02} {:02}:{:02}:{:02}",
        s / 3600,
        s / 60 % 60,
        s % 60
    )
}

/// An unscaled decimal as plain text with the scale applied and no trailing zeros (`1`, `-12.5`, `0.05`).
pub fn decimal_text(unscaled: &str, scale: usize) -> String {
    let (neg, digits) = match unscaled.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, unscaled),
    };
    let padded = format!("{:0>width$}", digits, width = scale + 1);
    let (ip, fp) = padded.split_at(padded.len() - scale);
    let fp = fp.trim_end_matches('0');
    let mut t = String::new();
    if neg && (ip != "0" || !fp.is_empty()) {
        t.push('-');
    }
    t.push_str(ip);
    if !fp.is_empty() {
        t.push('.');
        t.push_str(fp);
    }
    t
}

/// One cell as text; NULL is `<NULL>`.
pub fn cell(a: &dyn Array, i: usize) -> String {
    if a.is_null(i) {
        return "<NULL>".to_string();
    }
    match a.data_type() {
        DataType::Int64 => a.as_primitive::<Int64Type>().value(i).to_string(),
        DataType::Decimal128(_, s) => decimal_text(
            &a.as_primitive::<Decimal128Type>().value(i).to_string(),
            *s as usize,
        ),
        DataType::Decimal256(_, s) => decimal_text(
            &a.as_primitive::<Decimal256Type>().value(i).to_string(),
            *s as usize,
        ),
        DataType::Float64 => a
            .as_primitive::<arrow_array::types::Float64Type>()
            .value(i)
            .to_string(),
        DataType::Utf8 => a.as_string::<i32>().value(i).to_string(),
        DataType::LargeUtf8 => a.as_string::<i64>().value(i).to_string(),
        DataType::Binary => hex(a.as_binary::<i32>().value(i)),
        DataType::LargeBinary => hex(a.as_binary::<i64>().value(i)),
        DataType::Timestamp(TimeUnit::Second, _) => {
            ts_text(a.as_primitive::<TimestampSecondType>().value(i))
        }
        DataType::Timestamp(TimeUnit::Microsecond, _) => a
            .as_primitive::<TimestampMicrosecondType>()
            .value(i)
            .to_string(),
        DataType::Timestamp(TimeUnit::Nanosecond, _) => a
            .as_primitive::<TimestampNanosecondType>()
            .value(i)
            .to_string(),
        DataType::Duration(TimeUnit::Microsecond) => a
            .as_primitive::<DurationMicrosecondType>()
            .value(i)
            .to_string(),
        DataType::Duration(TimeUnit::Nanosecond) => a
            .as_primitive::<DurationNanosecondType>()
            .value(i)
            .to_string(),
        DataType::Interval(IntervalUnit::MonthDayNano) => {
            let v = a.as_primitive::<IntervalMonthDayNanoType>().value(i);
            format!("{}m{}d{}ns", v.months, v.days, v.nanoseconds)
        }
        t => panic!("cell: type {t:?}"),
    }
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// All rows of all batches as vectors of cell texts.
pub fn rows(batches: &[RecordBatch]) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    for b in batches {
        for i in 0..b.num_rows() {
            out.push(
                b.columns()
                    .iter()
                    .map(|c| cell(c.as_ref(), i))
                    .collect::<Vec<_>>(),
            );
        }
    }
    out
}

pub fn row_count(batches: &[RecordBatch]) -> usize {
    batches.iter().map(RecordBatch::num_rows).sum()
}

/// The `(folder, name)` of every truth file this process has opened through `truth_text` (T-10b, ADR-38
/// item 1). Each `tests/*.rs` is its own binary and nextest runs each test in its own process, so the set
/// holds only what the calling test opened itself.
static OPENED_TRUTH: std::sync::Mutex<std::collections::BTreeSet<(String, String)>> =
    std::sync::Mutex::new(std::collections::BTreeSet::new());

/// The text of a truth file. The one function that opens a `truth_*.txt` of `corpus/fixtures/`: it records
/// `(folder, name)` in the process-wide set of `opened_truth_files`.
pub fn truth_text(folder: &str, name: &str) -> String {
    let text = std::fs::read_to_string(fixtures().join(folder).join(name)).unwrap();
    OPENED_TRUTH
        .lock()
        .unwrap()
        .insert((folder.to_string(), name.to_string()));
    text
}

/// The lines of a truth file, split at `|`.
pub fn truth(folder: &str, name: &str) -> Vec<Vec<String>> {
    truth_text(folder, name)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split('|').map(str::to_string).collect())
        .collect()
}

/// The truth files opened so far in this process, as `(folder, name)`.
pub fn opened_truth_files() -> std::collections::BTreeSet<(String, String)> {
    OPENED_TRUTH.lock().unwrap().clone()
}

/// Oracle NUMBER bytes of the decimal `unscaled / 10^scale` ([Y1]; test-only encoder, written from the rule).
pub fn encode_number(unscaled: i128, scale: u32) -> Vec<u8> {
    if unscaled == 0 {
        return vec![0x80];
    }
    let neg = unscaled < 0;
    let mut digits: Vec<u8> = unscaled
        .unsigned_abs()
        .to_string()
        .bytes()
        .map(|c| c - b'0')
        .collect();
    // number of digits before the decimal point (may be <= 0)
    let int_digits = digits.len() as i32 - scale as i32;
    // base 100 groups are aligned at the decimal point
    let lead = int_digits.rem_euclid(2);
    if lead == 1 {
        digits.insert(0, 0);
    }
    if digits.len() % 2 == 1 {
        digits.push(0);
    }
    let e = (int_digits + lead) / 2 - 1;
    let mut groups: Vec<u8> = digits.chunks(2).map(|p| p[0] * 10 + p[1]).collect();
    while groups.last() == Some(&0) {
        groups.pop();
    }
    let mut out = Vec::new();
    if neg {
        out.push((62 - e) as u8);
        out.extend(groups.iter().map(|g| 101 - g));
        if groups.len() < 20 {
            out.push(0x66);
        }
    } else {
        out.push((193 + e) as u8);
        out.extend(groups.iter().map(|g| g + 1));
    }
    out
}

/// The 7 bytes of an Oracle DATE from seconds since the epoch ([Y2], year 1 to 9999).
pub fn encode_date(secs: i64) -> Vec<u8> {
    let (y, m, d) = civil_from_days(secs.div_euclid(86_400));
    let s = secs.rem_euclid(86_400);
    vec![
        (y / 100 + 100) as u8,
        (y % 100 + 100) as u8,
        m as u8,
        d as u8,
        (s / 3600 + 1) as u8,
        (s / 60 % 60 + 1) as u8,
        (s % 60 + 1) as u8,
    ]
}

// ---------------------------------------------------------------------------------------------
// T-11 helpers: patched copies below `target/` and the offsets of row fields
// ---------------------------------------------------------------------------------------------

/// Copies a fixture to `target/t6-tests/<dir>/<name>` after `patch` changed its bytes; the fixture is never
/// touched. Returns the path of the copy.
pub fn patched_file(rel: &str, dir: &str, name: &str, patch: impl FnOnce(&mut Vec<u8>)) -> PathBuf {
    let mut data = std::fs::read(fixtures().join(rel)).unwrap();
    patch(&mut data);
    let p = target_dir(dir).join(name);
    std::fs::write(&p, data).unwrap();
    p
}

/// ADR-64 item 5: the copy of `18c/long_exp_direct_18c.dmp` whose T_LONG row 3 (a chain of 5 pieces) has a UTF-8
/// lead byte `d0` as the last data byte of piece 1 (`9c 80` @0x2288, 156 data bytes from 0x228a, so the byte is
/// 0x2325), and piece 2 goes on with ASCII: `E-BAD-TEXT` with "(piece 2 of the value)" at 0x228a, row 3, column 3.
/// Used by `t6_patched.rs` and `t12_messages.rs`, so that the patch is written once.
pub fn chain_cut_copy(dir: &str, name: &str) -> PathBuf {
    patched_file("18c/long_exp_direct_18c.dmp", dir, name, |d| {
        let start = find_first(d, &[0x9c, 0x80]);
        assert_eq!(start, 0x2288);
        d[start + 2 + 155] = 0xd0;
    })
}

/// T-19: the copy of `18c/long3_exp_18c.dmp` whose T_LONG3 row 7 (`fd ff`, 25 chunks of 4,000 bytes, `00 00`; the
/// u16 at 0x522f6, `truth_long3.txt` line 7: 100,000 bytes) gets one more chunk of one byte before the `00 00`:
/// 100,001 bytes, above the largest confirmed LONG (`W-LONG-LENGTH`, ADR-95 item 9). Returns the path and the
/// offset of the value's first data byte (0x522fa). The offsets are those of `long3_check.py` (Analyst 17).
pub fn long3_100001_copy(dir: &str, name: &str) -> (PathBuf, u64) {
    let field = 0x522f6usize;
    let p = patched_file("18c/long3_exp_18c.dmp", dir, name, |b| {
        assert_eq!(b[field..field + 2], [0xfd, 0xff]);
        let end = field + 2 + 25 * 4002;
        assert_eq!(b[end..end + 2], [0x00, 0x00]);
        for (i, x) in [0x01u8, 0x00, b'y'].into_iter().enumerate() {
            b.insert(end + i, x);
        }
        // the file keeps its size (`W-FILE-SIZE` under strict): the zero fill at the end absorbs the 3 bytes
        let n = b.len();
        assert!(b[n - 3..].iter().all(|x| *x == 0));
        b.truncate(n - 3);
    });
    (p, (field + 4) as u64)
}

/// T-19: the copy of `18c/long3_exp_direct_18c.dmp` whose T_LONG3 row 7 (a chain of 12 x 7,952 + 4,576 bytes from
/// 0x522fb, `long3_check.py`) gets one more byte in the last piece: 100,001 bytes. Returns the path and the offset
/// of the first data byte (0x522fd).
pub fn long3_direct_100001_copy(dir: &str, name: &str) -> (PathBuf, u64) {
    let field = 0x522fbusize;
    let p = patched_file("18c/long3_exp_direct_18c.dmp", dir, name, |b| {
        let last = field + 12 * (2 + 7952);
        assert_eq!(b[last..last + 2], 4576u16.to_le_bytes());
        b[last..last + 2].copy_from_slice(&4577u16.to_le_bytes());
        b.insert(last + 2 + 4576, b'y');
        let n = b.len();
        assert_eq!(b[n - 1], 0);
        b.truncate(n - 1);
    });
    (p, (field + 2) as u64)
}

/// T-19, T-19b: the copy of `18c/long3_exp_18c.dmp` whose T_LONG3 row 5 (`fd ff`, 16 chunks of 4,000 + 1,535 bytes, the
/// u16 at 0x3228f, 65,535 bytes) has its first chunk shortened by two bytes (`a0 0f` becomes `9e 0f`, the 3,999th and
/// 4,000th data bytes are cut out and two zeros are appended to keep the file size): a chunked total of 65,533 bytes,
/// below the 65,534 that the fixtures show (`W-UNTESTED-LAYOUT`, ADR-95 item 2, ADR-99 item 2). Returns the path and
/// the offset of the value's first data byte (0x32293).
pub fn long3_chunked_short_copy(dir: &str, name: &str) -> (PathBuf, u64) {
    let field = 0x3228fusize;
    let p = patched_file("18c/long3_exp_18c.dmp", dir, name, |b| {
        assert_eq!(b[field..field + 4], [0xfd, 0xff, 0xa0, 0x0f]);
        b[field + 2] = 0x9e;
        b.remove(field + 4 + 3998);
        b.remove(field + 4 + 3998);
        b.extend_from_slice(&[0, 0]);
    });
    (p, (field + 4) as u64)
}

/// T-19b: the copy of `18c/long4_exp_18c.dmp` whose T_LONG4 row 1 (one field, `f1 ff`, the u16 at 0x2271, 65,521 bytes,
/// `long4_check.py`) is a field of 0xfff2 = 65,522 bytes: the u16 becomes `f2 ff`, one `y` is inserted after the
/// value and a zero of the fill at the end is dropped to keep the file size. The size 65,522 is one that no
/// fixture shows (R-052): the reader reads it as one field with `W-UNTESTED-LAYOUT`. Returns the path and the offset
/// of the field's u16 (0x2271, where the warning stands).
pub fn long4_fff2_copy(dir: &str, name: &str) -> (PathBuf, u64) {
    let field = 0x2271usize;
    let p = patched_file("18c/long4_exp_18c.dmp", dir, name, |b| {
        assert_eq!(b[field..field + 2], [0xf1, 0xff]);
        b[field] = 0xf2;
        let end = field + 2 + 65_521;
        // V (`v65521`, 6 bytes) follows L in the file order of the conventional path
        assert_eq!(b[end..end + 2], [0x06, 0x00]);
        b.insert(end, b'y');
        let n = b.len();
        assert_eq!(b[n - 1], 0);
        b.truncate(n - 1);
    });
    (p, field as u64)
}

/// ADR-64 item 5: the copy of `18c/lob_exp_18c.dmp` whose T_LOB row 5, column C (a long-form CLOB, UTF-16 BE,
/// first chunk 16,120 bytes) ends chunk 1 with the high surrogate `d8 00`; chunk 2 begins with `00 2e`, not a low
/// surrogate: `E-BAD-TEXT` with "(chunk 2 of the value)". Returns the path and the offset of the value (the
/// `fc ff` that `lob_start` receives). Used by `t12_offsets.rs` and `t12_messages.rs`.
pub fn lob_chunk_cut_copy(dir: &str, name: &str) -> (PathBuf, u64) {
    let rel = "18c/lob_exp_18c.dmp";
    let good = std::fs::read(fixtures().join(rel)).unwrap();
    let key = [0xfc, 0xff, 0x40, 0x0d, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00];
    let at = find_one(&good, &key);
    let first_len = usize::from(u16::from_le_bytes([good[at + 10], good[at + 11]]));
    assert_eq!(first_len, 16_120);
    let chunk1 = at + 12;
    let last_unit = chunk1 + first_len - 2;
    let chunk2 = chunk1 + first_len;
    assert_eq!(&good[last_unit..last_unit + 2], &[0x00, 0x2e]);
    assert_eq!(&good[chunk2 + 2..chunk2 + 4], &[0x00, 0x2e]);
    let p = patched_file(rel, dir, name, |b| {
        b[last_unit] = 0xd8;
        b[last_unit + 1] = 0x00;
    });
    (p, at as u64)
}

/// The first position of `pattern` in `data`.
pub fn find_first(data: &[u8], pattern: &[u8]) -> usize {
    data.windows(pattern.len())
        .position(|w| w == pattern)
        .unwrap_or_else(|| panic!("pattern {pattern:02x?} not found"))
}

/// The only position of `pattern` in `data`.
pub fn find_one(data: &[u8], pattern: &[u8]) -> usize {
    let hits: Vec<usize> = data
        .windows(pattern.len())
        .enumerate()
        .filter(|(_, w)| *w == pattern)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "pattern {pattern:02x?} found {} times",
        hits.len()
    );
    hits[0]
}

/// The file offset of the value of every column of the conventional-path row that starts at `row_start`
/// (plain fields only: a u16 LE length and the bytes; `fe ff` is NULL and gives `None`).
pub fn field_offsets(data: &[u8], row_start: u64, columns: usize) -> Vec<Option<u64>> {
    let mut at = usize::try_from(row_start).unwrap();
    let mut out = Vec::new();
    for _ in 0..columns {
        let len = u16::from_le_bytes([data[at], data[at + 1]]);
        if len == 0xfffe {
            out.push(None);
            at += 2;
        } else {
            out.push(Some((at + 2) as u64));
            at += 2 + usize::from(len);
        }
    }
    out
}

/// The start offsets of the rows of the first part of a table (conventional path, no LOB value area).
pub fn row_starts(d: &mut Dump, table: &str) -> Vec<u64> {
    let mut cur = d.rows(TableRef::ByName(table)).unwrap();
    let mut out = Vec::new();
    while let Some(at) = cur.next_row().unwrap() {
        out.push(at);
    }
    out
}

/// The folders of the EXP fixtures.
pub const FOLDERS: [&str; 3] = ["18c", "11g", "11g-cp1251"];

/// The EXP files (229 since T-19b) as `folder/name`, sorted.
pub fn exp_files() -> Vec<String> {
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
