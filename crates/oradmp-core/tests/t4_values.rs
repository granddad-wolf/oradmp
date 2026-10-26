//! Task T-4: value decoders (`oradmp_core::values`).
//!
//! Expected values come from the truth files of `corpus/fixtures/{18c,11g}`, from the plan and architecture
//! (named in the comments), and from `tests/data/value_parts.tsv`, which `tests/data/gen_number_parts.py`
//! generated from the Analyst's `docs/formats/tools/exp_parse.py` over the 229 `*_exp_*.dmp` fixtures:
//! per binary part the offset of its rows, the path, the row count, the column type codes, the number of
//! non-NULL NUMBER fields (86,236 in total, rules v3.9 [Y1]; `common::TOTALS`) and a SHA-256 over those fields. The test
//! reads the fields from the files with its own small field reader (not the walker: the walker delivers no
//! field bytes before T-6), so a wrong offset or a wrong chain reading shows as a different hash.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use arrow_buffer::i256;
use oradmp_core::values::datetime::{
    decode_date, decode_date_fields, decode_timestamp, decode_tsltz, decode_tstz, DateTime,
    Timestamp, Zone,
};
use oradmp_core::values::interval::{IntervalDs, IntervalYm};
use oradmp_core::values::long::{self, LongBuffer};
use oradmp_core::values::number::{Fitted, OraNumber, OverflowPolicy};
use oradmp_core::values::raw;
use oradmp_core::values::rowid::{decode_rowid, decode_urowid, ROWID_PREFIX};
use oradmp_core::ErrorCode;
use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixtures() -> PathBuf {
    root().join("corpus/fixtures")
}

/// The lines of a truth file, split at `|`.
fn truth(folder: &str, name: &str) -> Vec<Vec<String>> {
    common::truth_text(folder, name)
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split('|').map(str::to_string).collect())
        .collect()
}

/// `Typ=2 Len=3: 3e,64,66` (hex without leading zeros) to bytes; `NULL` to None.
fn dump_bytes(s: &str) -> Option<Vec<u8>> {
    let (_, hex) = s.split_once(": ")?;
    Some(
        hex.split(',')
            .map(|h| u8::from_str_radix(h, 16).unwrap())
            .collect(),
    )
}

// ---------------------------------------------------------------------------------------------------------
// NUMBER helpers

/// Text of the forms `-.5`, `123.456`, `1E+125`, `.05` to the canonical `OraNumber`.
fn parse_decimal(t: &str) -> OraNumber {
    let (neg, rest) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.strip_prefix('+').unwrap_or(t)),
    };
    let (mant, exp) = match rest.split_once(['E', 'e']) {
        Some((m, e)) => (m, e.parse::<i32>().unwrap()),
        None => (rest, 0),
    };
    let (ip, fp) = mant.split_once('.').unwrap_or((mant, ""));
    let mut digits: Vec<u8> = ip.bytes().chain(fp.bytes()).map(|c| c - b'0').collect();
    let mut exp10 = exp - fp.len() as i32;
    while digits.first() == Some(&0) {
        digits.remove(0);
    }
    while digits.last() == Some(&0) {
        digits.pop();
        exp10 += 1;
    }
    if digits.is_empty() {
        return OraNumber::ZERO;
    }
    OraNumber::from_parts(neg, &digits, exp10 as i16).unwrap()
}

/// Test-only encoder, the inverse of `OraNumber::decode` (the Analyst's `num_decode.encode` idea, written
/// again here in integers): None if the value needs more than 20 base-100 digits or an exponent outside the
/// bytes of [Y1].
fn encode(n: &OraNumber) -> Option<Vec<u8>> {
    if n.is_zero() {
        return Some(vec![0x80]);
    }
    let len = n.digits().len() as i32;
    let lead_pos = n.exp10() as i32 + len - 1;
    let e = lead_pos.div_euclid(2);
    if !(-65..=62).contains(&e) {
        return None;
    }
    // the leading base-100 digit covers decimal positions 2e+1 and 2e
    let pad_front = (2 * e + 1 - lead_pos) as usize;
    let mut dec: Vec<u8> = vec![0; pad_front];
    dec.extend_from_slice(n.digits());
    if dec.len() % 2 == 1 {
        dec.push(0);
    }
    let b100: Vec<u8> = dec.chunks(2).map(|p| p[0] * 10 + p[1]).collect();
    if b100.len() > 20 {
        return None;
    }
    let mut out = Vec::new();
    if n.is_negative() {
        out.push((0x3E - e) as u8);
        out.extend(b100.iter().map(|&d| 101 - d));
        if out.len() < 21 {
            out.push(0x66);
        }
    } else {
        out.push((0xC1 + e) as u8);
        out.extend(b100.iter().map(|&d| d + 1));
    }
    Some(out)
}

/// The 52 non-NULL values of the truth files `truth_num` (32), `truth_num2` (14) and `truth_num3` (6) with
/// the file name and id: (file, id, truth text, bytes).
fn truth_values(folder: &str) -> Vec<(&'static str, u32, String, Vec<u8>)> {
    let mut v = Vec::new();
    for name in ["truth_num.txt", "truth_num2.txt", "truth_num3.txt"] {
        for f in truth(folder, name) {
            if let Some(b) = dump_bytes(&f[2]) {
                v.push((name, f[0].parse().unwrap(), f[1].clone(), b));
            }
        }
    }
    v
}

fn n(file: &str, id: u32, folder: &str) -> OraNumber {
    let all = truth_values(folder);
    let (_, _, _, b) = all
        .iter()
        .find(|(f, i, _, _)| *f == file && *i == id)
        .unwrap();
    OraNumber::decode(b).unwrap()
}

/// (a) Every non-NULL value of `truth_num` (32), `truth_num2` (14), `truth_num3` (6) and the two DUMP columns
/// of `truth_numtypes` (4 rows x 2), in both folders, decodes to the truth value as an exact decimal, and the
/// test-only encoder gives the bytes back. 120 values in all: the count of the Analyst's `num_decode.py`
/// ("1. text truth: 120 values compared", run in this session on the three folders).
#[test]
fn a_truth_values_decode_to_the_truth_and_encode_back() {
    let mut count = 0;
    for folder in ["18c", "11g"] {
        for (file, id, text, bytes) in truth_values(folder) {
            let got = OraNumber::decode(&bytes).unwrap();
            assert_eq!(got, parse_decimal(&text), "{folder}/{file} id {id}");
            assert_eq!(encode(&got).unwrap(), bytes, "{folder}/{file} id {id}");
            count += 1;
        }
        // truth_numtypes: id|n_p10|n_p10s2|n_neg|n_fl|DUMP(n_p10s2)|DUMP(n_neg)
        for f in truth(folder, "truth_numtypes.txt") {
            for (value_col, dump_col) in [(2, 5), (3, 6)] {
                if let Some(bytes) = dump_bytes(&f[dump_col]) {
                    let got = OraNumber::decode(&bytes).unwrap();
                    assert_eq!(
                        got,
                        parse_decimal(&f[value_col]),
                        "{folder} numtypes id {}",
                        f[0]
                    );
                    assert_eq!(encode(&got).unwrap(), bytes);
                    count += 1;
                }
            }
        }
    }
    assert_eq!(count, 120);
}

/// The 18c and 11g truth files of the NUMBER tests are identical (checked, so that one folder serves the
/// classification tests below).
#[test]
fn a_truth_files_of_both_folders_are_equal() {
    for name in [
        "truth_num.txt",
        "truth_num2.txt",
        "truth_num3.txt",
        "truth_numtypes.txt",
    ] {
        assert_eq!(truth("18c", name), truth("11g", name), "{name}");
    }
}

fn code(e: oradmp_core::Error) -> ErrorCode {
    e.code
}

/// [Y1] validity checks, each on a patched byte string (architecture 5.2 step 2) gives `E-BAD-NUMBER`.
#[test]
fn a_every_validity_check_of_y1_has_a_failing_input() {
    let bad: &[(&str, &[u8])] = &[
        ("empty", &[]),
        ("22 bytes", &[0xc1; 22]),
        ("single byte 81", &[0x81]),
        ("single byte 00", &[0x00]),
        ("positive digit 00", &[0xc1, 0x02, 0x00]),
        ("positive digit 65", &[0xc1, 0x65]),
        ("negative digit 01", &[0x3e, 0x01, 0x66]),
        ("negative digit 66 inside", &[0x3e, 0x66, 0x40, 0x66]),
        (
            "negative digit 02..65 upper bound 66 not last",
            &[0x3e, 0x66, 0x02, 0x66],
        ),
        ("leading zero digit", &[0xc2, 0x01, 0x02]),
        ("trailing zero digit", &[0xc1, 0x02, 0x01]),
        ("no digit byte after sign byte", &[0xc1]),
        (
            "negative without closing 66, length 20",
            &[0x3e, 0x64, 0x4e, 0x38],
        ),
        ("negative 66 only", &[0x3e, 0x66]),
    ];
    for (what, b) in bad {
        let e = OraNumber::decode(b).expect_err(what);
        assert_eq!(e.code, ErrorCode::BadNumber, "{what}");
        assert!(e.offset.is_none(), "{what}: the caller adds the offset");
    }
    // a negative of length 20 without 66 (19 digit bytes would be 21 with it): all digit bytes valid
    let mut neg20 = vec![0x3e];
    neg20.extend(std::iter::repeat_n(0x40, 18));
    neg20.push(0x41); // 19 bytes
    assert_eq!(neg20.len(), 20);
    assert_eq!(
        OraNumber::decode(&neg20).unwrap_err().code,
        ErrorCode::BadNumber
    );
    // ... and the same bytes with the closing 66 are valid
    neg20.push(0x66);
    assert!(OraNumber::decode(&neg20).is_ok());
    // zero is exactly 80
    assert!(OraNumber::decode(&[0x80]).unwrap().is_zero());
}

/// Architecture 5.2 step 3: the intermediates of the three examples.
#[test]
fn a_intermediate_examples() {
    let x = OraNumber::decode(&[0x80, 0x02]).unwrap(); // E = -65, d = 1
    assert_eq!((x.digits(), x.exp10()), (&[1u8][..], -130));
    let x = OraNumber::decode(&[0xff, 0x0b]).unwrap(); // E = 62, d = 10
    assert_eq!((x.digits(), x.exp10()), (&[1u8][..], 125));
    let x = OraNumber::decode(&[0xc0, 0x33]).unwrap();
    assert_eq!((x.digits(), x.exp10()), (&[5u8][..], -1));
    assert_eq!((x.sig(), x.frac(), x.int_digits()), (1, 1, 0));
}

/// Walks the rows of a part with a field reader written for the test: rows from `rows_off`, per column a
/// u16 LE length (`fe ff` NULL), a direct-path LONG / LONG RAW (types 8, 24) as a piece chain [R2], after
/// each conventional row `00 00` or the LOB value area [R4, L3, L4] skipped by lengths. Calls `f(type, bytes)`
/// for every non-NULL field in file order.
fn read_fields(
    data: &[u8],
    rows_off: usize,
    direct: bool,
    rows: usize,
    types: &[u16],
    mut f: impl FnMut(u16, &[u8]),
) {
    let mut p = rows_off;
    let u16_at = |p: usize| u16::from_le_bytes([data[p], data[p + 1]]);
    for _ in 0..rows {
        let mut locators = 0;
        for &ty in types {
            let len = u16_at(p);
            p += 2;
            if len == 0xfffe {
                continue;
            }
            let is_long = ty == 8 || ty == 24;
            if direct && is_long {
                let mut value = Vec::new();
                let mut n = len;
                loop {
                    let l = usize::from(n & 0x7fff);
                    value.extend_from_slice(&data[p..p + l]);
                    p += l;
                    if n & 0x8000 == 0 {
                        break;
                    }
                    n = u16_at(p);
                    p += 2;
                }
                f(ty, &value);
            } else if is_long && len == 0xfffa {
                // the 4-byte length form [R2 v3.9]: `u16 n`, `00 00`, n bytes
                let n = usize::from(u16_at(p));
                assert_eq!(u16_at(p + 2), 0, "upper half of the 4-byte length at {p}");
                p += 4;
                f(ty, &data[p..p + n]);
                p += n;
            } else if is_long && len == 0xfffd {
                // the conventional chunked form [R2 v3.7]: chunks `u16 n + n bytes`, ended by `00 00`
                let mut value = Vec::new();
                loop {
                    let c = usize::from(u16_at(p));
                    p += 2;
                    if c == 0 {
                        break;
                    }
                    value.extend_from_slice(&data[p..p + c]);
                    p += c;
                }
                f(ty, &value);
            } else {
                let l = usize::from(len);
                f(ty, &data[p..p + l]);
                if ty == 112 || ty == 113 {
                    locators += 1;
                }
                p += l;
            }
        }
        if !direct {
            let k = usize::from(u16_at(p));
            p += 2;
            assert_eq!(k, locators, "k at {p}");
            for _ in 0..k {
                let h = u16_at(p);
                p += 2;
                if h == 0xfffc {
                    let total = u64::from_le_bytes(data[p..p + 8].try_into().unwrap());
                    p += 8;
                    let mut sum = 0u64;
                    while sum < total {
                        let c = u64::from(u16_at(p));
                        p += 2 + c as usize;
                        sum += c;
                    }
                } else {
                    p += usize::from(h);
                }
            }
        }
    }
}

struct Part {
    file: String,
    rows_off: usize,
    direct: bool,
    rows: usize,
    types: Vec<u16>,
    number_fields: usize,
    sha256: String,
}

fn parts() -> Vec<Part> {
    let text = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/value_parts.tsv"),
    )
    .unwrap();
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("file\t"))
        .map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            Part {
                file: c[0].to_string(),
                rows_off: c[1].parse().unwrap(),
                direct: c[2] == "1",
                rows: c[3].parse().unwrap(),
                types: c[4].split(',').map(|t| t.parse().unwrap()).collect(),
                number_fields: c[5].parse().unwrap(),
                sha256: c[6].to_string(),
            }
        })
        .collect()
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// Counters of the all-files walk.
#[derive(Default, Debug)]
struct FieldCounts {
    number: usize,
    date: usize,
    timestamp: usize,
    tstz: usize,
    tsltz: usize,
    iym: usize,
    ids: usize,
    rowid: usize,
    urowid: usize,
    /// DATE / TIMESTAMP fields with a year below 1 (decode to fields, no Arrow value)
    year_below_1: usize,
    /// INTERVAL YEAR TO MONTH fields whose months do not fit 32 bits
    iym_too_large: usize,
}

/// (a) All NUMBER fields of the 229 files (86,236, `num_decode.py` step 3 and the generator's assertion) decode, the
/// test-only encoder gives the same bytes, and the SHA-256 over them equals the Analyst's tool; the other
/// value types of the fixtures decode too, with exactly the documented exceptions (a year below 1, an
/// interval beyond 32 bits of months).
#[test]
fn a_all_fields_of_the_exp_files_decode() {
    let parts = parts();
    let tsv_parts = parts.len();
    let tsv_numbers: usize = parts.iter().map(|p| p.number_fields).sum();
    let mut by_file: BTreeMap<&str, Vec<&Part>> = BTreeMap::new();
    for p in &parts {
        by_file.entry(&p.file).or_default().push(p);
    }
    assert!(by_file.len() > 1);
    let mut c = FieldCounts::default();
    for (file, ps) in &by_file {
        let data = std::fs::read(fixtures().join(file)).unwrap();
        for p in ps {
            let mut h = Sha256::new();
            let mut n = 0;
            read_fields(
                &data,
                p.rows_off,
                p.direct,
                p.rows,
                &p.types,
                |ty, b| match ty {
                    2 => {
                        let v = OraNumber::decode(b).unwrap_or_else(|e| panic!("{file}: {e}"));
                        assert_eq!(encode(&v).as_deref(), Some(b), "{file}");
                        h.update([b.len() as u8]);
                        h.update(b);
                        n += 1;
                    }
                    12 => {
                        c.date += 1;
                        let d = decode_date_fields(b).unwrap();
                        if d.year < 1 {
                            c.year_below_1 += 1;
                            assert_eq!(decode_date(b).unwrap_err().code, ErrorCode::BadDatetime);
                        } else {
                            assert!(decode_date(b).is_ok());
                        }
                    }
                    180 => {
                        c.timestamp += 1;
                        decode_timestamp(b).unwrap();
                    }
                    181 => {
                        c.tstz += 1;
                        decode_tstz(b).unwrap();
                    }
                    231 => {
                        c.tsltz += 1;
                        decode_tsltz(b).unwrap();
                    }
                    182 => {
                        c.iym += 1;
                        let v = IntervalYm::decode(b).unwrap();
                        if v.total_months().is_err() {
                            c.iym_too_large += 1;
                        }
                    }
                    183 => {
                        c.ids += 1;
                        let v = IntervalDs::decode(b).unwrap();
                        v.to_month_day_nano().unwrap();
                    }
                    69 => {
                        c.rowid += 1;
                        let r = decode_rowid(b).unwrap();
                        assert!(r.prefix_known);
                    }
                    208 => {
                        c.urowid += 1;
                        decode_urowid(b).unwrap();
                    }
                    _ => {}
                },
            );
            assert_eq!(n, p.number_fields, "{file}@{}", p.rows_off);
            assert_eq!(hex(&h.finalize()), p.sha256, "{file}@{}", p.rows_off);
            c.number += n;
        }
    }
    assert_eq!(c.number, tsv_numbers, "all parts of the TSV were decoded");
    assert_eq!(parts.len(), tsv_parts);
    // N-2 (review 2): exact counts. Year below 1: `truth_date.txt` has two such rows (the dates written with a
    // leading `-`: rows 8 and 9), and the golden catalog has one T_DATE section in each of 8 files.
    let below_per_section = truth("18c", "truth_date.txt")
        .iter()
        .filter(|l| l[1].starts_with('-'))
        .count();
    assert_eq!(below_per_section, 2);
    let t_date_sections = common::truth_checks::golden_sections()
        .iter()
        .filter(|(_, t)| t == "T_DATE")
        .count();
    assert_eq!(t_date_sections, 8);
    assert_eq!(c.year_below_1, below_per_section * t_date_sections, "{c:?}");
    // the interval rows 2 and 3 of misc2 (999,999,999 years) in the two misc2 files x 2 columns of INTERVAL YEAR(9)
    assert_eq!(
        c.iym_too_large,
        2 * 2,
        "misc2: rows 2 and 3 in each of the two files"
    );
    // The number of non-NULL fields of each type over the 229 files (T-19b: the same script re-run, 206 TIMESTAMP fields; the tsp files add 84; the tsltz2 files add 36 TSLTZ fields: 4 files x 3 rows x 3 columns, row 4 NULL): a scratch script over
    // `docs/formats/tools/exp_parse.py::parse` (every row of every table, `row[i][1] is not None`, grouped by
    // the type code of the column), the same walk as `gen_number_parts.py`; its NUMBER (type 2) total is the
    // 86,236 of rules v3.9, asserted above through `tsv_numbers`.
    // type: 12 DATE 37,942; 180 TIMESTAMP 206; 181 TSTZ 24; 231 TSLTZ 44; 182 IYM 12; 183 IDS 18; 69 ROWID 8;
    // 208 UROWID 10.
    assert_eq!(
        (
            c.date,
            c.timestamp,
            c.tstz,
            c.tsltz,
            c.iym,
            c.ids,
            c.rowid,
            c.urowid
        ),
        (37_942, 206, 24, 44, 12, 18, 8, 10),
        "{c:?}"
    );
}

// ---------------------------------------------------------------------------------------------------------
// (b) renderings

fn d128(n: &OraNumber, policy: OverflowPolicy) -> oradmp_core::Result<Fitted<i128>> {
    n.to_decimal128(38, 10, policy)
}

/// Unscaled digits of a truth text at `scale` computed from the text alone (None if the text has an
/// exponent or more fractional digits than the scale).
fn unscaled_from_text(t: &str, scale: usize) -> Option<String> {
    if t.contains('E') {
        return None;
    }
    let (neg, rest) = match t.strip_prefix('-') {
        Some(r) => ("-", r),
        None => ("", t),
    };
    let (ip, fp) = rest.split_once('.').unwrap_or((rest, ""));
    let fp = fp.trim_end_matches('0');
    if fp.len() > scale {
        return None;
    }
    let s = format!("{ip}{fp}{}", "0".repeat(scale - fp.len()));
    let s = s.trim_start_matches('0');
    Some(if s.is_empty() {
        "0".to_string()
    } else {
        format!("{neg}{s}")
    })
}

/// Architecture 5.2 step 4 / plan (b): Int64 for `truth_num2` rows 10 and 12, "does not fit" for 11, and for
/// all 52 truth values `to_i64` agrees with `str::parse::<i64>` of the truth text.
#[test]
fn b_int64() {
    assert_eq!(n("truth_num2.txt", 10, "18c").to_i64(), Some(i64::MAX));
    assert_eq!(n("truth_num2.txt", 12, "18c").to_i64(), Some(i64::MIN));
    assert_eq!(n("truth_num2.txt", 11, "18c").to_i64(), None);
    let mut fits = 0;
    for (file, id, text, bytes) in truth_values("18c") {
        let want = text.parse::<i64>().ok();
        assert_eq!(
            OraNumber::decode(&bytes).unwrap().to_i64(),
            want,
            "{file} {id}"
        );
        fits += usize::from(want.is_some());
    }
    assert!(fits >= 15);
    assert_eq!(OraNumber::ZERO.to_i64(), Some(0));
}

/// Plan (b): Decimal128(38, 10) is exact for `truth_num` rows 1 to 18 (unscaled value from the truth text),
/// under every policy.
#[test]
fn b_decimal128_exact_rows_1_to_18() {
    for id in 1..=18 {
        let v = n("truth_num.txt", id, "18c");
        let text = truth("18c", "truth_num.txt")[id as usize - 1][1].clone();
        let want: i128 = unscaled_from_text(&text, 10).unwrap().parse().unwrap();
        for policy in [
            OverflowPolicy::Error,
            OverflowPolicy::Round,
            OverflowPolicy::Null,
        ] {
            assert_eq!(d128(&v, policy).unwrap(), Fitted::Exact(want), "row {id}");
        }
    }
}

/// Plan (b): with `round`, rows 25, 27, 30, 31 and `truth_num2` rows 13, 14 round (27, 13, 14 to 0); the
/// rounded values are written out by hand (half away from zero on the 11th decimal); under `error` they are
/// value errors and under `null` they give NULL; integer overflow (rows 19 to 24, 26, 28, 29, 32) is an error
/// under every policy.
#[test]
fn b_decimal128_round_error_null() {
    // (file, id, unscaled value at scale 10, became zero)
    let rounded: [(&str, u32, i128, bool); 6] = [
        ("truth_num.txt", 25, 1_234_567_890, false), // .12345678901|2345...: digit 11 is 1, down
        ("truth_num.txt", 27, 0, true),              // 1E-125
        ("truth_num.txt", 30, 31_415_926_536, false), // 3.1415926535|897...: up
        ("truth_num.txt", 31, 3_333_333_333, false), // .3333... down
        ("truth_num2.txt", 13, 0, true),             // 1E-130
        ("truth_num2.txt", 14, 0, true),             // .00000000001 = 1E-11 < 5E-11
    ];
    for (file, id, want, zero) in rounded {
        let v = n(file, id, "18c");
        match d128(&v, OverflowPolicy::Round).unwrap() {
            Fitted::Rounded {
                value,
                to_zero,
                dropped,
            } => {
                assert_eq!((value, to_zero), (want, zero), "{file} {id}");
                assert!(dropped > 0);
            }
            other => panic!("{file} {id}: {other:?}"),
        }
        assert_eq!(
            d128(&v, OverflowPolicy::Error).unwrap_err().code,
            ErrorCode::NumberOverflow,
            "{file} {id}"
        );
        assert_eq!(
            d128(&v, OverflowPolicy::Null).unwrap(),
            Fitted::Nulled,
            "{file} {id}"
        );
    }
    for id in [19, 20, 21, 22, 23, 24, 26, 28, 29, 32] {
        let v = n("truth_num.txt", id, "18c");
        for policy in [
            OverflowPolicy::Error,
            OverflowPolicy::Round,
            OverflowPolicy::Null,
        ] {
            assert_eq!(
                d128(&v, policy).unwrap_err().code,
                ErrorCode::NumberOverflow,
                "row {id}"
            );
        }
    }
}

/// Architecture 5.2a / plan (b'): counts over the 52 truth values, `round`, exact / changed (became 0) /
/// integer part too large: Decimal128(38, 10) 30 / 12 (3) / 10; Decimal256(76, 38) 42 / 7 (2) / 3;
/// Decimal256(76, 40) 42 / 2 (2) / 8. This is the classification the survey prints (5.2b).
#[test]
fn b_counts_of_arch_5_2a() {
    let values: Vec<OraNumber> = truth_values("18c")
        .iter()
        .map(|(_, _, _, b)| OraNumber::decode(b).unwrap())
        .collect();
    assert_eq!(values.len(), 52);
    let classify = |f: &dyn Fn(&OraNumber) -> oradmp_core::Result<(bool, bool)>| {
        let (mut exact, mut changed, mut zero, mut int) = (0, 0, 0, 0);
        for v in &values {
            match f(v) {
                Ok((false, _)) => exact += 1,
                Ok((true, z)) => {
                    changed += 1;
                    zero += usize::from(z);
                }
                Err(e) => {
                    assert_eq!(e.code, ErrorCode::NumberOverflow);
                    int += 1;
                }
            }
        }
        (exact, changed, zero, int)
    };
    let c128 = classify(&|v| {
        Ok(match v.to_decimal128(38, 10, OverflowPolicy::Round)? {
            Fitted::Exact(_) => (false, false),
            Fitted::Rounded { to_zero, .. } => (true, to_zero),
            Fitted::Nulled => unreachable!(),
        })
    });
    assert_eq!(c128, (30, 12, 3, 10));
    let c256_38 = classify(&|v| {
        Ok(match v.to_decimal256(76, 38, OverflowPolicy::Round)? {
            Fitted::Exact(_) => (false, false),
            Fitted::Rounded { to_zero, .. } => (true, to_zero),
            Fitted::Nulled => unreachable!(),
        })
    });
    assert_eq!(c256_38, (42, 7, 2, 3));
    let c256_40 = classify(&|v| {
        Ok(match v.to_decimal256(76, 40, OverflowPolicy::Round)? {
            Fitted::Exact(_) => (false, false),
            Fitted::Rounded { to_zero, .. } => (true, to_zero),
            Fitted::Nulled => unreachable!(),
        })
    });
    assert_eq!(c256_40, (42, 2, 2, 8));
}

/// Plan (b'), `truth_num3`: the six values are exact in Decimal256(76, 40); under Decimal128(38, 10) with
/// `round` all six change and none becomes 0 (rounded values written out); under Decimal256(76, 38) rows
/// 1, 2, 3, 5 change and rows 4, 6 are exact (architecture 8.1, "session 4: Y1 computed").
#[test]
fn b_num3_computed_values() {
    // unscaled values at scale 10 after rounding half away from zero
    let want10: [i128; 6] = [
        3_333_333_333,   // 1/3
        6_666_666_667,   // 2/3 = .66...67
        1_428_571_429,   // 1/7 = .1428571428|57...
        333_333_333_333, // 100/3 = 33.3333333333|33...
        -3_333_333_333,  // -1/3
        16_666_666_667,  // 1.66...67 -> 1.6666666667
    ];
    let texts: Vec<String> = truth("18c", "truth_num3.txt")
        .iter()
        .map(|f| f[1].clone())
        .collect();
    for id in 1..=6u32 {
        let v = n("truth_num3.txt", id, "18c");
        match d128(&v, OverflowPolicy::Round).unwrap() {
            Fitted::Rounded { value, to_zero, .. } => {
                assert_eq!(value, want10[id as usize - 1], "row {id}");
                assert!(!to_zero);
            }
            other => panic!("row {id}: {other:?}"),
        }
        // exact in Decimal256(76, 40): the unscaled digits are the digits of the text shifted to scale 40
        let want =
            i256::from_string(&unscaled_from_text(&texts[id as usize - 1], 40).unwrap()).unwrap();
        assert_eq!(
            v.to_decimal256(76, 40, OverflowPolicy::Error).unwrap(),
            Fitted::Exact(want),
            "row {id}"
        );
        let at38 = v.to_decimal256(76, 38, OverflowPolicy::Round).unwrap();
        let exact38 = matches!(at38, Fitted::Exact(_));
        assert_eq!(exact38, id == 4 || id == 6, "row {id}");
    }
}

/// Plan (b): Float64 equals `str::parse::<f64>` of the truth text for every row; the text rendering is the
/// truth value (compared as a decimal, not as text: `0.001`, not `.001`).
#[test]
fn b_float64_and_text() {
    for (file, id, text, bytes) in truth_values("18c") {
        let v = OraNumber::decode(&bytes).unwrap();
        let want: f64 = text.parse().unwrap();
        assert_eq!(v.to_f64().unwrap().to_bits(), want.to_bits(), "{file} {id}");
        assert_eq!(
            parse_decimal(&v.to_text()),
            v,
            "{file} {id}: {}",
            v.to_text()
        );
    }
    // text forms of architecture 5.2 step 4
    let t = |s: &str| parse_decimal(s).to_text();
    assert_eq!(t("0.001"), "0.001");
    assert_eq!(t("-1000000"), "-1000000");
    assert_eq!(t("123.456"), "123.456");
    assert_eq!(t("-0.5"), "-0.5");
    assert_eq!(t("1E+125"), "1E+125");
    assert_eq!(t("1E-130"), "1E-130");
    assert_eq!(
        t("9.9999999999999999999999999999999999999E+125"),
        "9.9999999999999999999999999999999999999E+125"
    );
    assert_eq!(OraNumber::ZERO.to_text(), "0");
    assert_eq!(OraNumber::ZERO.to_f64().unwrap(), 0.0);
    // a plain text of exactly 64 characters stays plain, 65 characters use the exponent form
    let big64 = parse_decimal(&format!("1{}", "0".repeat(63)));
    assert_eq!(big64.to_text().len(), 64);
    let big65 = parse_decimal(&format!("1{}", "0".repeat(64)));
    assert_eq!(big65.to_text(), "1E+64");
}

/// Rounding carries through all digits (half away from zero) and may push the integer part over the precision.
#[test]
fn b_rounding_carry() {
    let r = |t: &str, p: u8, s: u8| parse_decimal(t).to_decimal128(p, s, OverflowPolicy::Round);
    assert_eq!(
        r("0.5", 5, 0).unwrap(),
        Fitted::Rounded {
            value: 1,
            to_zero: false,
            dropped: 1
        }
    );
    assert_eq!(
        r("-0.5", 5, 0).unwrap(),
        Fitted::Rounded {
            value: -1,
            to_zero: false,
            dropped: 1
        }
    );
    assert_eq!(
        r("0.4999", 5, 0).unwrap(),
        Fitted::Rounded {
            value: 0,
            to_zero: true,
            dropped: 4
        }
    );
    assert_eq!(
        r("99.5", 5, 0).unwrap(),
        Fitted::Rounded {
            value: 100,
            to_zero: false,
            dropped: 1
        }
    );
    assert_eq!(
        r("99999.5", 5, 0).unwrap_err().code,
        ErrorCode::NumberOverflow
    ); // 100000 needs 6 digits
    assert_eq!(
        r("0.00001", 5, 3).unwrap(),
        Fitted::Rounded {
            value: 0,
            to_zero: true,
            dropped: 2
        }
    );
    assert_eq!(
        r("0.0005", 5, 3).unwrap(),
        Fitted::Rounded {
            value: 1,
            to_zero: false,
            dropped: 1
        }
    );
    // an exact value is exact whatever the policy
    assert_eq!(r("12.5", 5, 1).unwrap(), Fitted::Exact(125));
    // NUMBER(3, 5) is Decimal128(5, 5): .00123 fits
    assert_eq!(r("0.00123", 5, 5).unwrap(), Fitted::Exact(123));
    assert_eq!(r("0.1", 5, 5).unwrap(), Fitted::Exact(10000));
    // precision above 38 for Decimal128 is a configuration error, above 76 for Decimal256
    assert_eq!(
        code(
            parse_decimal("1")
                .to_decimal128(39, 0, OverflowPolicy::Round)
                .unwrap_err()
        ),
        ErrorCode::Config
    );
    assert_eq!(
        code(
            parse_decimal("1")
                .to_decimal256(77, 0, OverflowPolicy::Round)
                .unwrap_err()
        ),
        ErrorCode::Config
    );
    // zero is exact
    assert_eq!(
        OraNumber::ZERO
            .to_decimal128(38, 10, OverflowPolicy::Error)
            .unwrap(),
        Fitted::Exact(0)
    );
}

// ---------------------------------------------------------------------------------------------------------
// (c) dates and times

/// Independent day count: counts the days from 1970-01-01 year by year and month by month.
fn epoch_seconds(y: i32, mo: u32, d: u32, h: i64, mi: i64, s: i64) -> i64 {
    let leap = |y: i32| (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let mut days: i64 = 0;
    for yy in 1970..y {
        days += if leap(yy) { 366 } else { 365 };
    }
    for yy in y..1970 {
        days -= if leap(yy) { 366 } else { 365 };
    }
    let months = [
        31,
        if leap(y) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    days += months.iter().take((mo - 1) as usize).sum::<i64>();
    days += i64::from(d) - 1;
    days * 86_400 + h * 3600 + mi * 60 + s
}

/// ` 2012-09-17 13:31:01` / `-0100-06-15 12:00:00` to (year, month, day, h, m, s).
fn parse_ts(t: &str) -> (i32, u32, u32, i64, i64, i64, u32) {
    let (neg, t) = match t.strip_prefix('-') {
        Some(r) => (true, r),
        None => (false, t.trim_start()),
    };
    let (date, time) = t.split_once(' ').unwrap();
    let mut dp = date.split('-');
    let y: i32 = dp.next().unwrap().parse().unwrap();
    let mo = dp.next().unwrap().parse().unwrap();
    let d = dp.next().unwrap().parse().unwrap();
    let (time, frac) = time.split_once('.').unwrap_or((time, ""));
    let mut tp = time.split(':');
    let h = tp.next().unwrap().parse().unwrap();
    let mi = tp.next().unwrap().parse().unwrap();
    let s = tp.next().unwrap().parse().unwrap();
    let nanos = if frac.is_empty() {
        0
    } else {
        format!("{frac:0<9}").parse().unwrap()
    };
    (if neg { -y } else { y }, mo, d, h, mi, s, nanos)
}

/// Plan (c), `truth_date.txt` (both folders): ten non-NULL rows decode to the fields of the truth text and to
/// the epoch seconds (independent day count); rows 8 and 9 (years -100 and -4712) decode to fields and give
/// `E-BAD-DATETIME` as an Arrow value.
#[test]
fn c_dates() {
    for folder in ["18c", "11g"] {
        let mut n = 0;
        for f in truth(folder, "truth_date.txt") {
            let Some(b) = dump_bytes(&f[2]) else { continue };
            n += 1;
            let (y, mo, d, h, mi, s, _) = parse_ts(&f[1]);
            let got = decode_date_fields(&b).unwrap();
            assert_eq!(
                (
                    got.year,
                    u32::from(got.month),
                    u32::from(got.day),
                    i64::from(got.hour),
                    i64::from(got.minute),
                    i64::from(got.second)
                ),
                (y, mo, d, h, mi, s),
                "{folder} id {}",
                f[0]
            );
            if y < 1 {
                assert_eq!(decode_date(&b).unwrap_err().code, ErrorCode::BadDatetime);
            } else {
                assert_eq!(
                    decode_date(&b).unwrap(),
                    epoch_seconds(y, mo, d, h, mi, s),
                    "{folder} id {}",
                    f[0]
                );
            }
        }
        assert_eq!(n, 10);
    }
}

/// Plan (c), `truth_ts.txt`: TIMESTAMP(0), (6) and (9) of five rows: exact fields and nanoseconds; Microsecond
/// is exact or an error; the nanosecond value of 9999-12-31 is outside 64 bits (`E-BAD-DATETIME`).
#[test]
fn c_timestamps() {
    let mut micro_errors = 0;
    let mut nano_errors = 0;
    for folder in ["18c", "11g"] {
        for f in truth(folder, "truth_ts.txt") {
            for (tc, dc) in [(1, 2), (3, 4), (5, 6)] {
                let Some(b) = dump_bytes(&f[dc]) else {
                    continue;
                };
                let (y, mo, d, h, mi, s, ns) = parse_ts(&f[tc]);
                let ts = decode_timestamp(&b).unwrap();
                assert_eq!(
                    ts,
                    Timestamp {
                        secs: epoch_seconds(y, mo, d, h, mi, s),
                        nanos: ns
                    },
                    "{folder} {} col {tc}",
                    f[0]
                );
                match ts.to_micros() {
                    Ok(v) => assert_eq!(v, ts.secs * 1_000_000 + i64::from(ns / 1000)),
                    Err(e) => {
                        assert_eq!(e.code, ErrorCode::BadDatetime);
                        assert!(ns % 1000 != 0);
                        micro_errors += 1;
                    }
                }
                match ts.to_nanos() {
                    Ok(v) => assert_eq!(v, ts.secs * 1_000_000_000 + i64::from(ns)),
                    Err(e) => {
                        assert_eq!(e.code, ErrorCode::BadDatetime);
                        assert_eq!(y, 9999);
                        nano_errors += 1;
                    }
                }
            }
        }
    }
    // rows 3 (.000000001), 4 (.123456789) and 5 (.999999999) of the nanosecond column do not fit Microsecond:
    // 3 rows x 2 versions
    assert_eq!(micro_errors, 6);
    // row 5 of each of the three columns is 9999-12-31: 3 columns x 2 versions
    assert_eq!(nano_errors, 6);
    // lengths other than 7 and 11
    assert_eq!(
        decode_timestamp(&[0x78; 9]).unwrap_err().code,
        ErrorCode::BadDatetime
    );
}

/// Plan (c), `truth_tstz.txt`: for all 6 rows instant = local time minus offset (the text of the truth);
/// the offset of the first three rows is read from the zone bytes; the four region rows keep their two bytes.
#[test]
fn c_tstz_and_tsltz() {
    for folder in ["18c", "11g"] {
        let mut rows = 0;
        for f in truth(folder, "truth_tstz.txt") {
            // tstz=2016-03-26 19:34:55.123456 +05:30 +05:30
            if let Some(b) = dump_bytes(&f[2]) {
                rows += 1;
                let text = f[1].strip_prefix("tstz=").unwrap();
                let parts: Vec<&str> = text.split(' ').collect();
                let (y, mo, d, h, mi, s, ns) = parse_ts(&format!("{} {}", parts[0], parts[1]));
                let off = parts[2];
                let sign = if off.starts_with('-') { -1 } else { 1 };
                let (oh, om) = off[1..].split_once(':').unwrap();
                let off_min = sign * (oh.parse::<i64>().unwrap() * 60 + om.parse::<i64>().unwrap());
                let v = decode_tstz(&b).unwrap();
                assert_eq!(
                    v.instant.secs,
                    epoch_seconds(y, mo, d, h, mi, s) - off_min * 60,
                    "{folder} id {}",
                    f[0]
                );
                assert_eq!(v.instant.nanos, ns);
                match v.zone {
                    Zone::Offset(m) => {
                        assert!(["1", "2", "3"].contains(&f[0].as_str()), "id {}", f[0]);
                        assert_eq!(i64::from(m), off_min);
                    }
                    Zone::Region(r) => {
                        assert!(["4", "5", "6"].contains(&f[0].as_str()));
                        assert_eq!(r, u16::from_be_bytes([b[11], b[12]]));
                    }
                }
            }
            if let Some(b) = dump_bytes(&f[4]) {
                // tsltz=2016-03-26 19:34:55.123456: stored as written (R-019)
                let (y, mo, d, h, mi, s, ns) = parse_ts(f[3].strip_prefix("tsltz=").unwrap());
                let ts = decode_tsltz(&b).unwrap();
                assert_eq!(
                    ts,
                    Timestamp {
                        secs: epoch_seconds(y, mo, d, h, mi, s),
                        nanos: ns
                    }
                );
            }
        }
        assert_eq!(rows, 6);
    }
    // a TSLTZ of 7 bytes is read as a zero fraction (confirmed by tsltz2_exp_<v>, ADR-100 item 1)
    let ts = decode_tsltz(&[0x78, 0x74, 3, 0x1a, 0x10, 0x0a, 0x1c]).unwrap();
    assert_eq!(ts.nanos, 0);
}

/// Patched bytes: lengths, month 13, day 30 of February, an hour byte of 0, a century byte outside the layout,
/// nanoseconds above 999,999,999, year 0 and BC are all value errors, never a panic.
#[test]
fn c_datetime_errors() {
    let ok = [0x78u8, 0x70, 9, 0x11, 0x0e, 0x20, 0x02];
    assert!(decode_date(&ok).is_ok());
    let patch = |i: usize, v: u8| {
        let mut b = ok;
        b[i] = v;
        b
    };
    for (what, b) in [
        ("month 13", patch(2, 13)),
        ("month 0", patch(2, 0)),
        ("day 0", patch(3, 0)),
        ("day 32", patch(3, 32)),
        ("hour byte 0", patch(4, 0)),
        ("hour 24", patch(4, 25)),
        ("minute 60", patch(5, 61)),
        ("second 60", patch(6, 61)),
        ("century byte 0", patch(0, 0)),
        ("year byte 0", patch(1, 0)),
        ("year 0", {
            let mut b = ok;
            b[0] = 100;
            b[1] = 100;
            b
        }),
    ] {
        assert_eq!(
            decode_date(&b).unwrap_err().code,
            ErrorCode::BadDatetime,
            "{what}"
        );
    }
    assert!(
        decode_date(&[0x78, 0x70, 2, 30, 1, 1, 1]).is_err(),
        "30 February"
    );
    assert!(
        decode_date(&[0x78, 0x78, 2, 29, 1, 1, 1]).is_ok(),
        "29 February 2020"
    );
    assert!(
        decode_date(&[0x78, 0x77, 2, 29, 1, 1, 1]).is_err(),
        "29 February 2019"
    );
    assert!(decode_date(&ok[..6]).is_err());
    let mut ts = ok.to_vec();
    ts.extend_from_slice(&1_000_000_000u32.to_be_bytes());
    assert_eq!(
        decode_timestamp(&ts).unwrap_err().code,
        ErrorCode::BadDatetime
    );
    ts.truncate(7);
    ts.extend_from_slice(&999_999_999u32.to_be_bytes());
    assert_eq!(decode_timestamp(&ts).unwrap().nanos, 999_999_999);
    assert!(decode_tstz(&[0u8; 12]).is_err());
    let mut tz = ok.to_vec();
    tz.extend_from_slice(&[0, 0, 0, 0, 0x14, 0x3c]);
    assert_eq!(decode_tstz(&tz).unwrap().zone, Zone::Offset(0));
    tz[11] = 0x14 + 30; // +30 hours
    assert!(decode_tstz(&tz).is_err());
    assert_eq!(
        DateTime::new(2020, 2, 30, 0, 0, 0).unwrap_err().code,
        ErrorCode::BadDatetime
    );
}

// ---------------------------------------------------------------------------------------------------------
// (d) property tests

mod props {
    use super::*;
    use proptest::prelude::*;

    fn digits_strategy() -> impl Strategy<Value = Vec<u8>> {
        (1usize..=40).prop_flat_map(|len| {
            (
                1u8..=9,
                proptest::collection::vec(0u8..=9, len.saturating_sub(2)),
                1u8..=9,
            )
                .prop_map(move |(first, mid, last)| {
                    let mut v = vec![first];
                    if len >= 2 {
                        v.extend(mid);
                        v.push(last);
                    }
                    v
                })
        })
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: None,
            ..ProptestConfig::default()
        })]

        /// Architecture 8.3 (c): digits and exponent, to bytes, back to the same number.
        #[test]
        fn number_roundtrip(neg in any::<bool>(), digits in digits_strategy(), exp10 in -170i16..=130) {
            let num = OraNumber::from_parts(neg, &digits, exp10).unwrap();
            let Some(bytes) = encode(&num) else { return Ok(()) };
            prop_assert!(bytes.len() <= 21);
            let back = OraNumber::decode(&bytes).unwrap();
            prop_assert_eq!(back, num);
            // text round trip
            prop_assert_eq!(parse_decimal(&num.to_text()), num);
            // Float64 is the nearest double of the exact text
            let f = num.to_f64().unwrap();
            let want: f64 = format!("{}{}e{}", if neg { "-" } else { "" }, digits.iter().map(|d| char::from(b'0' + d)).collect::<String>(), exp10).parse().unwrap();
            prop_assert_eq!(f.to_bits(), want.to_bits());
        }

        /// Digit strings to Decimal128 and back: an exact value is the digits of the text.
        #[test]
        fn decimal128_matches_the_digit_string(neg in any::<bool>(), digits in digits_strategy(), scale in 0u8..=20) {
            let len = digits.len() as i32;
            // choose exp10 so that the value has exactly `scale` fractional digits when it fits
            let exp10 = -(i32::from(scale));
            let num = OraNumber::from_parts(neg, &digits, exp10 as i16).unwrap();
            match num.to_decimal128(38, scale, OverflowPolicy::Error) {
                Ok(Fitted::Exact(v)) => {
                    prop_assert!(len <= 38);
                    let s: String = digits.iter().map(|d| char::from(b'0' + d)).collect();
                    let want: i128 = s.parse().unwrap();
                    prop_assert_eq!(v, if neg { -want } else { want });
                }
                Ok(other) => prop_assert!(false, "{other:?}"),
                Err(e) => {
                    prop_assert_eq!(e.code, ErrorCode::NumberOverflow);
                    prop_assert!(len > 38);
                }
            }
        }

        /// Rounding half away from zero equals the digit-string reading: dropping `d` digits and adding 1
        /// when the first dropped digit is 5 or more.
        #[test]
        fn rounding_matches_a_reference(neg in any::<bool>(), digits in digits_strategy(), drop in 1usize..=12) {
            prop_assume!(digits.len() >= drop && digits.len() - drop <= 36);
            let scale = 10u8;
            let num = OraNumber::from_parts(neg, &digits, -(drop as i16) - i16::from(scale)).unwrap();
            let kept = &digits[..digits.len() - drop];
            let first_dropped = digits[digits.len() - drop];
            let mut v: i128 = kept.iter().fold(0i128, |a, &d| a * 10 + i128::from(d));
            if first_dropped >= 5 { v += 1; }
            let want = if neg { -v } else { v };
            match num.to_decimal128(38, scale, OverflowPolicy::Round) {
                Ok(Fitted::Rounded { value, .. }) => prop_assert_eq!(value, want),
                other => prop_assert!(false, "{other:?}"),
            }
        }

        /// No panic on arbitrary bytes of length 0 to 64, in every decoder; whatever decodes, renders.
        #[test]
        fn arbitrary_bytes_never_panic(b in proptest::collection::vec(any::<u8>(), 0..=64)) {
            if let Ok(v) = OraNumber::decode(&b) {
                let _ = v.to_i64();
                let _ = v.to_f64();
                let _ = v.to_text();
                let _ = v.to_decimal128(38, 10, OverflowPolicy::Round);
                let _ = v.to_decimal128(10, 2, OverflowPolicy::Null);
                let _ = v.to_decimal256(76, 38, OverflowPolicy::Round);
                let _ = v.to_decimal256(76, 0, OverflowPolicy::Error);
                // a valid byte string is canonical: the encoder returns it
                let enc = encode(&v);
                prop_assert_eq!(enc.as_deref(), Some(&b[..]));
            }
            let _ = decode_date(&b);
            let _ = decode_timestamp(&b);
            let _ = decode_tstz(&b);
            let _ = decode_tsltz(&b);
            let _ = IntervalYm::decode(&b).map(|v| (v.total_months(), v.to_text(4)));
            let _ = IntervalDs::decode(&b).map(|v| (v.to_duration_micros(), v.to_duration_nanos(), v.to_month_day_nano(), v.to_text(9, 9)));
            let _ = decode_rowid(&b);
            let _ = decode_urowid(&b);
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// (e) intervals, ROWID, UROWID

fn misc_lines(folder: &str, name: &str, kind: &str) -> Vec<Vec<String>> {
    truth(folder, name)
        .into_iter()
        .filter(|f| f[0] == kind)
        .collect()
}

/// Plan (e), `truth_misc.txt` (both folders): the three non-NULL values of IYM and IDS decode to -38 months
/// for `-0003-02`, 909,045,123,456 microseconds for `+0010 12:30:45.123456`, zero; and, through the text
/// renderer of [Y10] with p = 4, f = 6, to the `TO_CHAR` text of the truth itself.
#[test]
fn e_intervals_of_truth_misc() {
    let months = [30, -38, 0];
    let micros = [909_045_123_456_i64, -86_400_000_000, 0];
    for folder in ["18c", "11g"] {
        let lines = misc_lines(folder, "truth_misc.txt", "INTERVAL");
        assert_eq!(lines.len(), 4);
        for (i, f) in lines.iter().take(3).enumerate() {
            let ym = IntervalYm::decode(&dump_bytes(&f[3]).unwrap()).unwrap();
            assert_eq!(ym.total_months().unwrap(), months[i], "{folder} IYM {i}");
            assert_eq!(ym.to_text(4), f[2]);
            let ds = IntervalDs::decode(&dump_bytes(&f[5]).unwrap()).unwrap();
            assert_eq!(
                ds.to_duration_micros().unwrap(),
                micros[i],
                "{folder} IDS {i}"
            );
            assert_eq!(ds.to_text(4, 6), f[4]);
        }
        assert_eq!(&lines[3][2..], ["NULL", "NULL", "NULL", "NULL"]);
    }
}

/// Plan (e'), `truth_misc2.txt` (both folders): the Arrow value and the `TO_CHAR` text of each interval.
/// Column 1 is INTERVAL YEAR(9) TO MONTH, column 2 DAY(9) TO SECOND(9) (MonthDayNano), column 3 DAY(2) TO
/// SECOND(0) (Duration(Microsecond)). IYM rows 2 and 3 are `E-BAD-INTERVAL` (months beyond 32 bits); the
/// MonthDayNano values and the microsecond values are those written in architecture 5.7 and ADR-19.
#[test]
fn e_intervals_of_truth_misc2() {
    for folder in ["18c", "11g"] {
        let lines = misc_lines(folder, "truth_misc2.txt", "INTERVAL");
        assert_eq!(lines.len(), 3);
        // (months or error, days, nanos of the day, micros of the p = 2 column)
        let want_ds9: [(i32, i64); 3] = [
            (-1, -7_384_500_000_000),
            (999_999_999, 86_399_999_999_999),
            (-999_999_999, -86_399_999_999_999),
        ];
        let want_micros: [i64; 3] = [
            -1_000_000,
            (99 * 86_400 + 86_399) * 1_000_000,
            -(99 * 86_400 + 86_399) * 1_000_000,
        ];
        for (i, f) in lines.iter().enumerate() {
            let ym = IntervalYm::decode(&dump_bytes(&f[3]).unwrap()).unwrap();
            assert_eq!(ym.to_text(9), f[2], "{folder} row {}", i + 1);
            if i == 0 {
                assert_eq!(ym.total_months().unwrap(), -23);
            } else {
                assert_eq!(
                    ym.total_months().unwrap_err().code,
                    ErrorCode::BadInterval,
                    "row {}",
                    i + 1
                );
            }
            let ds9 = IntervalDs::decode(&dump_bytes(&f[5]).unwrap()).unwrap();
            assert_eq!(ds9.to_text(9, 9), f[4]);
            assert_eq!(
                ds9.to_month_day_nano().unwrap(),
                want_ds9[i],
                "row {}",
                i + 1
            );
            let ds2 = IntervalDs::decode(&dump_bytes(&f[7]).unwrap()).unwrap();
            assert_eq!(ds2.to_text(2, 0), f[6]);
            assert_eq!(
                ds2.to_duration_micros().unwrap(),
                want_micros[i],
                "row {}",
                i + 1
            );
        }
    }
}

/// ADR-19 checks: lengths, month 12, hour 24, minute 60, nanoseconds 10^9, opposite signs (session 4:
/// `E-BAD-INTERVAL` instead of a warning), the sub-microsecond part under Microsecond, the Nanosecond range.
#[test]
fn e_interval_errors() {
    let ym = |b: &[u8]| IntervalYm::decode(b).unwrap_err().code;
    assert_eq!(ym(&[0x80, 0, 0, 2]), ErrorCode::BadInterval); // length 4
    assert_eq!(ym(&[0x80, 0, 0, 2, 0x42, 0]), ErrorCode::BadInterval);
    assert_eq!(ym(&[0x80, 0, 0, 0, 60 + 12]), ErrorCode::BadInterval); // months 12
    assert_eq!(ym(&[0x80, 0, 0, 1, 60 - 1]), ErrorCode::BadInterval); // +1 year -1 month
    assert_eq!(
        ym(&[0x7f, 0xff, 0xff, 0xff, 60 + 1]),
        ErrorCode::BadInterval
    ); // -1 year +1 month
    assert_eq!(ym(&[0xff, 0xff, 0xff, 0xff, 60]), ErrorCode::BadInterval); // years beyond 999,999,999
    let ok = [0x80u8, 0, 0, 0x0a, 0x48, 0x5a, 0x69, 0x87, 0x5b, 0xca, 0x00];
    assert!(IntervalDs::decode(&ok).is_ok());
    let patch = |i: usize, v: u8| {
        let mut b = ok;
        b[i] = v;
        IntervalDs::decode(&b).unwrap_err().code
    };
    assert_eq!(patch(4, 60 + 24), ErrorCode::BadInterval); // hours 24
    assert_eq!(patch(5, 60 + 60), ErrorCode::BadInterval); // minutes 60
    assert_eq!(patch(6, 60 + 60), ErrorCode::BadInterval); // seconds 60
    assert_eq!(patch(5, 60 - 1), ErrorCode::BadInterval); // minutes -1 beside positive days
    assert_eq!(
        IntervalDs::decode(&ok[..10]).unwrap_err().code,
        ErrorCode::BadInterval
    );
    // nanoseconds 1,000,000,000
    let mut b = ok;
    b[7..11].copy_from_slice(&(0x8000_0000u32 + 1_000_000_000).to_be_bytes());
    assert_eq!(
        IntervalDs::decode(&b).unwrap_err().code,
        ErrorCode::BadInterval
    );
    // the sub-microsecond part under Duration(Microsecond), exact under Nanosecond
    let mut b = ok;
    b[7..11].copy_from_slice(&(0x8000_0000u32 + 1).to_be_bytes());
    let v = IntervalDs::decode(&b).unwrap();
    assert_eq!(
        v.to_duration_micros().unwrap_err().code,
        ErrorCode::BadInterval
    );
    assert_eq!(
        v.to_duration_nanos().unwrap(),
        10 * 86_400_000_000_000 + (12 * 3600 + 30 * 60 + 45) * 1_000_000_000 + 1
    );
    // 999,999,999 days: outside Duration(Nanosecond), inside Duration(Microsecond)? 8.6e22 ns / 1000 = 8.6e19 > i64
    let mut b = ok;
    b[0..4].copy_from_slice(&(0x8000_0000u32 + 999_999_999).to_be_bytes());
    let v = IntervalDs::decode(&b).unwrap();
    assert_eq!(
        v.to_duration_nanos().unwrap_err().code,
        ErrorCode::BadInterval
    );
    assert_eq!(
        v.to_duration_micros().unwrap_err().code,
        ErrorCode::BadInterval
    );
    assert_eq!(v.to_month_day_nano().unwrap().0, 999_999_999);
}

/// Plan (e), `truth_misc.txt` ROWID lines: RID = `AAAR8lAAMAAAACHAAA` (18c), `AAAE5IAAEAAAACGAAA` (11g) from the
/// 23-byte field (prefix `01 00 00 00 17` + text); URID gives the same text from the 13 `DUMP` bytes.
/// `truth_misc2.txt`: 3 ROWIDs and 3 UROWIDs equal; the logical UROWID `*BAMAFbMCwQL+` (18c) and the 11g
/// text of the truth.
#[test]
fn e_rowids() {
    let want = [("18c", "AAAR8lAAMAAAACHAAA"), ("11g", "AAAE5IAAEAAAACGAAA")];
    for (folder, text) in want {
        let lines = misc_lines(folder, "truth_misc.txt", "ROWID");
        assert_eq!(lines[0][2], text);
        let mut field = ROWID_PREFIX.to_vec();
        field.extend_from_slice(text.as_bytes());
        let r = decode_rowid(&field).unwrap();
        assert_eq!((r.text.as_str(), r.prefix_known), (text, true));
        assert_eq!(
            decode_urowid(&dump_bytes(&lines[0][4]).unwrap()).unwrap(),
            text
        );
        assert_eq!(&lines[1][2..], ["NULL", "NULL", "NULL"]);

        let lines = misc_lines(folder, "truth_misc2.txt", "ROWID");
        assert_eq!(lines.len(), 4);
        for f in &lines[..3] {
            let mut field = ROWID_PREFIX.to_vec();
            field.extend_from_slice(f[2].as_bytes());
            assert_eq!(decode_rowid(&field).unwrap().text, f[2]);
            assert_eq!(decode_urowid(&dump_bytes(&f[5]).unwrap()).unwrap(), f[2]);
            assert_eq!(f[4], f[2]);
        }
        // the logical UROWID of row 10
        let logical = &lines[3][4];
        assert_eq!(
            decode_urowid(&dump_bytes(&lines[3][5]).unwrap()).unwrap(),
            *logical
        );
    }
    assert_eq!(
        misc_lines("18c", "truth_misc2.txt", "ROWID")[3][4],
        "*BAMAFbMCwQL+"
    );
    assert_eq!(
        misc_lines("11g", "truth_misc2.txt", "ROWID")[3][4],
        "*BAEACtMCwQL+"
    );
    // the example of architecture 5.7: 13 bytes, 18c
    let b = [
        0x01, 0x00, 0x01, 0x1f, 0x25, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x87, 0x00, 0x00,
    ];
    assert_eq!(decode_urowid(&b).unwrap(), "AAAR8lAAMAAAACHAAA");
}

/// Patched ROWID and UROWID: lengths, a byte outside the alphabet, a prefix other than the known one (text
/// delivered, flag false), a UROWID starting `02` with a tail that is not a multiple of 3, other first bytes.
#[test]
fn e_rowid_errors() {
    let mut field = ROWID_PREFIX.to_vec();
    field.extend_from_slice(b"AAAR8lAAMAAAACHAAA");
    assert!(decode_rowid(&field[..22]).is_err());
    let mut longer = field.clone();
    longer.push(b'A');
    assert_eq!(decode_rowid(&longer).unwrap_err().code, ErrorCode::BadRowid);
    for bad in [b'-', b' ', b'=', 0x80, 0] {
        let mut f = field.clone();
        f[10] = bad;
        assert_eq!(decode_rowid(&f).unwrap_err().code, ErrorCode::BadRowid);
    }
    let mut f = field.clone();
    f[0] = 0x02;
    let r = decode_rowid(&f).unwrap();
    assert!(!r.prefix_known);
    assert_eq!(r.text, "AAAR8lAAMAAAACHAAA");
    // the prefix is part of the 5 bytes only: a bad byte in the prefix does not make an error
    assert_eq!(decode_urowid(&[]).unwrap_err().code, ErrorCode::BadRowid);
    assert_eq!(
        decode_urowid(&[0x01; 12]).unwrap_err().code,
        ErrorCode::BadRowid
    );
    assert_eq!(
        decode_urowid(&[0x01; 14]).unwrap_err().code,
        ErrorCode::BadRowid
    );
    // logical: tail of 8 bytes (not a multiple of 3), no tail, first byte 03
    assert_eq!(
        decode_urowid(&[0x02, 1, 2, 3, 4, 5, 6, 7, 8])
            .unwrap_err()
            .code,
        ErrorCode::BadRowid
    );
    assert_eq!(
        decode_urowid(&[0x02]).unwrap_err().code,
        ErrorCode::BadRowid
    );
    assert_eq!(
        decode_urowid(&[0x03, 1, 2, 3]).unwrap_err().code,
        ErrorCode::BadRowid
    );
    assert_eq!(decode_urowid(&[0x02, 0, 0, 0]).unwrap(), "*AAAA");
    assert_eq!(decode_urowid(&[0x02, 0xff, 0xff, 0xff]).unwrap(), "*////");
}

// ---------------------------------------------------------------------------------------------------------
// RAW and LONG

/// RAW: the bytes unchanged; the "longer than declared" test compares byte length and descriptor length [C8].
#[test]
fn raw_and_long_helpers() {
    let b = [0u8, 0xff, 0x80, 0x7f];
    assert_eq!(raw::decode_raw(&b), &b);
    assert!(!raw::longer_than_declared(4, 4));
    assert!(raw::longer_than_declared(5, 4));
    // LONG: a UTF-8 sequence split between two pieces decodes after assembly (ADR-15)
    let text = "дж Café".as_bytes();
    let mut buf = LongBuffer::new(100);
    buf.push_piece(&text[..1]).unwrap(); // half of the first letter
    buf.push_piece(&text[1..]).unwrap();
    assert_eq!(long::decode_utf8(buf.as_bytes()).unwrap(), "дж Café");
    assert!(long::has_high_bytes(buf.as_bytes()));
    assert!(!long::has_high_bytes(b"ascii"));
    assert_eq!(
        long::decode_utf8(&[0xd0]).unwrap_err().code,
        ErrorCode::BadText
    );
    // max_lob_bytes: the limit is checked before the piece is copied
    let mut small = LongBuffer::new(10);
    small.push_piece(&[1; 6]).unwrap();
    let e = small.push_piece(&[1; 5]).unwrap_err();
    assert_eq!(e.code, ErrorCode::LobTooLarge);
    assert_eq!(small.as_bytes().len(), 6);
    assert_eq!(small.take().len(), 6);
    // T-19b (ADR-99 item 6): LONG and LONG RAW are confirmed to 100,000 bytes (R-040, R-048, R-051)
    assert!(!long::above_confirmed_length(100_000) && long::above_confirmed_length(100_001));
    assert!(!long::above_confirmed_length(32_001));
}
