//! Task T-9b: `W-CHARSET-MISMATCH` for form-1 CLOB columns (type 112; N-25, ADR-41 item 5) and the offset of
//! `W-LONG-LENGTH` on a direct-path chain (ADR-41 item 6).
//!
//! Offsets: `lob_exp_18c.dmp` header database set at 0x32..0x34 (`03 69`, as in `t6l_ru.rs`, T-6M (c));
//! `long_exp_direct_18c.dmp` T_LONG row 3: the chain starts at 0x2288 with `9c 80` (156 bytes), then four pieces
//! of 7,961 bytes (exp.md 6.6, `t3b_parts.rs`); 156 + 4 x 7961 = 32,000.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use arrow_array::cast::AsArray;
use arrow_array::{Array, RecordBatch};
use common::*;
use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::{ErrorCode, Warning, WarningCode};

fn texts(b: &[RecordBatch], c: usize) -> Vec<Option<String>> {
    let mut out = Vec::new();
    for b in b {
        let a = b.column(c).as_string::<i64>();
        for i in 0..a.len() {
            out.push((!a.is_null(i)).then(|| a.value(i).to_string()));
        }
    }
    out
}

/// `lob_exp_18c.dmp` with the database set of the header 873 -> 171 (the T-6M patch), offsets asserted first.
fn lob_171(name: &str) -> std::path::PathBuf {
    patched_file("18c/lob_exp_18c.dmp", "t9b", name, |b| {
        assert_eq!(b[0x30..0x36], [0x03, 0x69, 0x03, 0x69, 0x07, 0xd0]);
        b[0x32] = 0x00;
        b[0x33] = 0xab;
    })
}

/// The mismatch warnings the creation of the reader of `table` gives, as (column, table, offset-present).
fn mismatches(d: &mut oradmp_core::dump::Dump, table: &str, o: &ReadOptions) -> Vec<Warning> {
    let r = d.read_table(TableRef::ByName(table), o).unwrap();
    r.warnings()
        .iter()
        .filter(|w| w.code == WarningCode::CharsetMismatch)
        .cloned()
        .collect()
}

#[test]
fn form_1_clob_columns_warn_charset_mismatch() {
    let p = lob_171("lob_171_a.dmp");
    let mut d = open_path(&p, Options::default());
    assert_eq!(d.info().db_charset_id, 171);
    // T_LOB: ID (2), C (112, form 1, set 873, file position 2), B (113); T_LOB2: A (112, form 1, set 873,
    // position 2), V (VARCHAR2 set 873, position 3), N (112, form 2, set 2000, position 4)
    let w = mismatches(&mut d, "T_LOB", &ReadOptions::default());
    assert_eq!(w.iter().map(|w| w.column).collect::<Vec<_>>(), [Some(2)]);
    assert_eq!(w[0].table, Some(1));
    let w = mismatches(&mut d, "T_LOB2", &ReadOptions::default());
    // A and V; N is a national column (form 2) and does not warn
    assert_eq!(
        w.iter().map(|w| w.column).collect::<Vec<_>>(),
        [Some(2), Some(3)]
    );

    // short C values read as UTF-16 BE (L5); row 5 (long form, S != D) stays unconfirmed
    let e = try_read(&mut d, "T_LOB", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::LobUnconfirmed);
    assert_eq!((e.row, e.column), (Some(5), Some(2)));
    let null = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let (x, _) = try_read(&mut d, "T_LOB", &null).unwrap();
    let mut orig = open_fixture("18c/lob_exp_18c.dmp");
    let (y, _) = read(&mut orig, "T_LOB", &ReadOptions::default());
    let (tx, ty) = (texts(&x, 1), texts(&y, 1));
    assert_eq!(tx[3].as_deref(), Some("m".repeat(3000).as_str()));
    assert_eq!(tx[4], None);
    // N-5: rows 1 to 3 of C against `corpus/sql/01b_optional_tables.sql` (`TO_CLOB('hello lob')`, NULL,
    // `EMPTY_CLOB()`), as `t6l_lob.rs` does, and then the unpatched file's read
    assert_eq!(tx[0].as_deref(), Some("hello lob"));
    assert_eq!(tx[1], None);
    assert_eq!(tx[2].as_deref(), Some(""));
    assert_eq!(tx[..4], ty[..4]);
}

#[test]
fn strict_stops_t_lob_before_any_row_and_db_charset_silences_it() {
    let p = lob_171("lob_171_b.dmp");
    let mut d = open(
        &p,
        Options {
            strict: true,
            ..Options::default()
        },
    )
    .unwrap();
    let e = d
        .read_table(TableRef::ByName("T_LOB"), &ReadOptions::default())
        .map(|_| ())
        .unwrap_err();
    assert_eq!(e.code, ErrorCode::Strict);
    assert_eq!(e.warning, Some(WarningCode::CharsetMismatch));
    assert_eq!(e.column, Some(2));
    assert_eq!(e.row, None);

    let mut d = open(
        &p,
        Options {
            db_charset: Some("windows-1251".to_string()),
            ..Options::default()
        },
    )
    .unwrap();
    assert!(mismatches(&mut d, "T_LOB", &ReadOptions::default()).is_empty());
    assert!(mismatches(&mut d, "T_LOB2", &ReadOptions::default()).is_empty());
}

#[test]
fn the_unpatched_file_has_no_mismatch() {
    let mut d = open_fixture("18c/lob_exp_18c.dmp");
    assert!(mismatches(&mut d, "T_LOB", &ReadOptions::default()).is_empty());
    assert!(mismatches(&mut d, "T_LOB2", &ReadOptions::default()).is_empty());
}

/// Row 3 of T_LONG: the chain gets one more byte (last piece 7,961 -> 7,962 bytes): 32,001 bytes, above the
/// 20,000 of `max_lob_bytes` below, but not above the confirmed 100,000 bytes of a LONG (T-19).
fn chain_32001(name: &str) -> (std::path::PathBuf, u64) {
    let rel = "18c/long_exp_direct_18c.dmp";
    let data = std::fs::read(fixtures().join(rel)).unwrap();
    let start = 0x2288usize;
    assert_eq!(data[start..start + 2], [0x9c, 0x80]);
    // walk the pieces from the bytes
    let (mut at, mut lens) = (start, Vec::new());
    loop {
        let n = u16::from_le_bytes([data[at], data[at + 1]]);
        lens.push((at, usize::from(n & 0x7fff)));
        at += 2 + usize::from(n & 0x7fff);
        if n & 0x8000 == 0 {
            break;
        }
    }
    assert_eq!(
        lens.iter().map(|l| l.1).collect::<Vec<_>>(),
        [156, 7961, 7961, 7961, 7961]
    );
    let (last_at, last_len) = lens[4];
    assert_eq!(at, last_at + 2 + last_len);
    let p = patched_file(rel, "t9b", name, |b| {
        b[last_at..last_at + 2]
            .copy_from_slice(&u16::try_from(last_len + 1).unwrap().to_le_bytes());
        b.insert(at, b'x');
    });
    (p, (start + 2) as u64)
}

#[test]
fn w_long_length_on_a_chain_reports_the_first_piece() {
    // T-19: the chain of `long3_exp_direct_18c.dmp` row 7 (12 x 7,952 + 4,576 = 100,000 bytes) with one more byte
    let (p, first_data) = long3_direct_100001_copy("t9b", "chain_a.dmp");
    let mut d = open_path(&p, Options::default());
    let (_, ws) = read(&mut d, "T_LONG3", &ReadOptions::default());
    let w: Vec<_> = ws
        .iter()
        .filter(|w| w.code == WarningCode::LongLength)
        .collect();
    assert_eq!(w.len(), 1);
    // row 7, column of the LONG in the file order (3: ID, V, L)
    assert_eq!(
        (w[0].offset, w[0].row, w[0].column),
        (Some(first_data), Some(7), Some(3))
    );
    // 32,001 bytes are no longer above the confirmed length of a LONG
    let (p, _) = chain_32001("chain_c.dmp");
    let mut d = open_path(&p, Options::default());
    let (_, ws) = read(&mut d, "T_LONG", &ReadOptions::default());
    assert!(
        ws.iter().all(|w| w.code != WarningCode::LongLength),
        "{ws:?}"
    );
}

#[test]
fn e_lob_too_large_on_a_chain_reports_the_first_piece_too() {
    let (p, first_data) = chain_32001("chain_b.dmp");
    let mut d = open_path(&p, Options::default());
    let e = try_read(
        &mut d,
        "T_LONG",
        &ReadOptions {
            max_lob_bytes: 20_000,
            ..ReadOptions::default()
        },
    )
    .unwrap_err();
    assert_eq!(e.code, ErrorCode::LobTooLarge);
    assert_eq!(
        (e.offset, e.row, e.column),
        (Some(first_data), Some(3), Some(3))
    );
}
