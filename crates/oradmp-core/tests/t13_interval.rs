//! Task T-13 (a): `interval_as="string"` (ADR-47). Every INTERVAL column is Utf8 holding the `TO_CHAR` text of
//! rule Y10, cell by cell equal to the texts of `truth_misc.txt` (18c; T_INTERVAL) and `truth_misc2.txt`
//! (18c and 11g; T_INTERVAL2). The expected texts are read from those truth files (the `INTERVAL|id|IYM|dump|IDS|
//! dump[|IDS0|dump]` lines), none is typed here except the fallback and opposite-sign cases, which derive from
//! the dump bytes of the truth lines.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use arrow_schema::DataType;
use common::*;
use oradmp_core::dump::{open, Options};
use oradmp_core::reader::ReadOptions;
use oradmp_core::types::map::IntervalAs;
use oradmp_core::{ErrorCode, WarningCode};

fn string_options() -> Options {
    let mut o = Options::default();
    o.types.interval_as = IntervalAs::String;
    o
}

fn open_string(rel: &str) -> oradmp_core::dump::Dump {
    open(fixtures().join(rel), string_options()).unwrap()
}

/// The truth lines of `INTERVAL` as (id, texts of the interval columns).
fn interval_truth(folder: &str, file: &str) -> Vec<Vec<String>> {
    truth(folder, file)
        .into_iter()
        .filter(|l| l[0] == "INTERVAL")
        .map(|l| l.into_iter().skip(2).step_by(2).collect())
        .collect()
}

#[test]
fn string_reads_equal_the_to_char_texts_of_the_truth_files() {
    for v in ["18c", "11g"] {
        // truth_misc.txt exists in both versions; the dump carries the same four rows
        let mut d = open_string(&format!("{v}/misc_exp_{v}.dmp"));
        let (b, w) = read(&mut d, "T_INTERVAL", &ReadOptions::default());
        assert!(w.is_empty(), "{v}: {w:?}");
        let schema = b[0].schema();
        assert_eq!(schema.field(1).data_type(), &DataType::Utf8);
        assert_eq!(schema.field(2).data_type(), &DataType::Utf8);
        for f in [1, 2] {
            assert_eq!(
                schema
                    .field(f)
                    .metadata()
                    .get("oradmp:conversion")
                    .map(String::as_str),
                Some("to-char-text")
            );
        }
        let got = rows(&b);
        let want = interval_truth(v, "truth_misc.txt");
        assert_eq!(got.len(), 4, "{v}");
        assert_eq!(want.len(), 4, "{v}");
        // row 4 is NULL in both columns (the truth file writes NULL, the cell text is <NULL>)
        assert_eq!(want[3], vec!["NULL", "NULL"]);
        for (g, t) in got.iter().zip(&want) {
            let t: Vec<String> = t.iter().map(|x| x.replace("NULL", "<NULL>")).collect();
            assert_eq!((&g[1], &g[2]), (&t[0], &t[1]), "{v}");
        }

        let mut d = open_string(&format!("{v}/misc2_exp_{v}.dmp"));
        let (b, w) = read(&mut d, "T_INTERVAL2", &ReadOptions::default());
        assert!(w.is_empty(), "{v}: {w:?}");
        let got = rows(&b);
        let want = interval_truth(v, "truth_misc2.txt");
        assert_eq!(got.len(), 3);
        assert_eq!(want.len(), 3);
        for (g, t) in got.iter().zip(&want) {
            assert_eq!(&g[1..], &t[..], "{v}");
        }
        // rows 2 and 3 (YEAR(9) beyond 2^31 months) read without E-BAD-INTERVAL
        assert_eq!(got[1][1], "+999999999-11");
        assert_eq!(got[2][1], "-999999999-11");
    }
}

#[test]
fn native_is_the_default_and_keeps_the_types_and_the_error() {
    let mut d = open_fixture("18c/misc2_exp_18c.dmp");
    let e = try_read(&mut d, "T_INTERVAL2", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadInterval);
    let (b, _) = read_default("18c/misc_exp_18c.dmp", "T_INTERVAL");
    assert!(matches!(
        b[0].schema().field(1).data_type(),
        DataType::Interval(_)
    ));
    assert!(b[0]
        .schema()
        .field(1)
        .metadata()
        .get("oradmp:conversion")
        .is_none());
    assert_eq!("string".parse::<IntervalAs>().unwrap(), IntervalAs::String);
    assert_eq!("native".parse::<IntervalAs>().unwrap(), IntervalAs::Native);
    assert_eq!(
        "other".parse::<IntervalAs>().unwrap_err().code,
        ErrorCode::Config
    );
}

/// T_INTERVAL row 1 IYM is `Typ=182 Len=5: 80,0,0,2,42` (truth_misc.txt: +0002-06): years +2, months +6.
const IYM_ROW1: [u8; 5] = [0x80, 0, 0, 2, 0x42];

#[test]
fn opposite_signs_stay_e_bad_interval_under_string() {
    let p = patched_file("18c/misc_exp_18c.dmp", "t13", "opposite.dmp", |b| {
        let at = find_one(b, &[5, 0, 0x80, 0, 0, 2, 0x42]) + 2;
        assert_eq!(b[at..at + 5], IYM_ROW1);
        b[at + 4] = 60 - 6; // months -6 beside years +2
    });
    let mut d = open(&p, string_options()).unwrap();
    let e = try_read(&mut d, "T_INTERVAL", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadInterval);
    assert_eq!((e.row, e.column), (Some(1), Some(2)));
}

/// IDS of T_INTERVAL row 1 (`18c/misc_exp_18c.dmp`@0x22b0, `docs/review/review-03-python-packaging.md` section 4:
/// days 0x8000000a - 2^31 = 10, hours 0x48 - 60 = 12, minutes 30, seconds 45, nanoseconds 123,456,000).
const IDS_ROW1: [u8; 11] = [0x80, 0, 0, 0x0a, 0x48, 0x5a, 0x69, 0x87, 0x5b, 0xca, 0];

/// T-14 (h), review 03 section 4: the DAY TO SECOND opposite-sign check also holds under `interval_as =
/// "string"`. The hours byte of row 1 becomes -12 beside +10 days. Shown to fail with the DS check removed
/// (`IntervalDs::decode`, `same_sign`): the cell is then read as text.
#[test]
fn day_to_second_opposite_signs_stay_e_bad_interval_under_string() {
    let p = patched_file("18c/misc_exp_18c.dmp", "t13", "opposite_ds.dmp", |b| {
        let at = find_one(b, &[11, 0, 0x80, 0, 0, 0x0a, 0x48]) + 2;
        assert_eq!(b[at..at + 11], IDS_ROW1);
        b[at + 4] = 60 - 12; // hours -12 beside days +10
    });
    let mut d = open(&p, string_options()).unwrap();
    let e = try_read(&mut d, "T_INTERVAL", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadInterval);
    // the IDS column is the third column of T_INTERVAL (file position 3), row 1
    assert_eq!((e.row, e.column), (Some(1), Some(3)));
}

#[test]
fn a_column_without_a_usable_declaration_takes_the_digits_it_needs_and_f_9() {
    // `SECOND (6)` of T_INTERVAL.IDS becomes `SECONX (6)` (same length): the definition is not one of the forms of
    // rule T9, the column falls back (W-DDL-FALLBACK, column 3) and its text has p = digits needed, f = 9.
    let p = patched_file("18c/misc_exp_18c.dmp", "t13", "fallback.dmp", |b| {
        let at = find_one(b, b"INTERVAL DAY (4) TO SECOND (6)") + 20;
        assert_eq!(&b[at..at + 6], b"SECOND");
        b[at + 5] = b'X';
    });
    let mut d = open(&p, string_options()).unwrap();
    d.scan().unwrap();
    let ws = d.warnings().to_vec();
    let fb: Vec<_> = ws
        .iter()
        .filter(|w| w.code == WarningCode::DdlFallback && w.column == Some(3))
        .collect();
    assert_eq!(fb.len(), 1, "{ws:?}");
    let (b, _) = read(&mut d, "T_INTERVAL", &ReadOptions::default());
    let got = rows(&b);
    // truth_misc.txt rows 1 and 2: +0010 12:30:45.123456 and -0001 00:00:00.000000
    assert_eq!(got[0][2], "+10 12:30:45.123456000");
    assert_eq!(got[1][2], "-1 00:00:00.000000000");
    // the column with its declaration keeps p = 4
    assert_eq!(got[0][1], "+0002-06");
}

#[test]
fn a_table_fallback_warns_once_per_interval_column() {
    // the CREATE TABLE name `"IYM"` becomes `"IYX"`: the names do not match the INSERT INTO names, the table
    // is mapped from its descriptors (one table-level W-DDL-FALLBACK, no column) and under `string` each of
    // the two INTERVAL columns (file positions 2 and 3) gets its own.
    let p = patched_file("18c/misc_exp_18c.dmp", "t13", "table_fallback.dmp", |b| {
        let at = find_one(b, b"\"IYM\" INTERVAL") + 3;
        b[at - 1] = b'X';
    });
    let mut d = open(&p, string_options()).unwrap();
    d.scan().unwrap();
    let ord = d
        .catalog()
        .unwrap()
        .tables
        .iter()
        .find(|t| t.name == "T_INTERVAL")
        .unwrap()
        .ordinal;
    let cols: Vec<Option<u32>> = d
        .warnings()
        .iter()
        .filter(|w| w.code == WarningCode::DdlFallback && w.table == Some(ord))
        .map(|w| w.column)
        .collect();
    let (b, _) = read(&mut d, "T_INTERVAL", &ReadOptions::default());
    assert_eq!(rows(&b)[0][1], "+2-06", "p = the digits the value needs");
    assert_eq!(
        cols.iter().filter(|c| **c == Some(2)).count(),
        1,
        "{cols:?}"
    );
    assert_eq!(
        cols.iter().filter(|c| **c == Some(3)).count(),
        1,
        "{cols:?}"
    );
    assert_eq!(cols.iter().filter(|c| c.is_none()).count(), 1, "{cols:?}");
}
