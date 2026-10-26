//! Task T-12 (e), ADR-43 item 3 (review S-3): one offset per value under every policy. A report about one
//! value - the value error under `raise`, the `W-VALUE-NULLED` that replaces it under `null` - carries the
//! start of the value: the first data byte of the first piece of a direct-path LONG chain, the offset that
//! `lob_start` receives for a LOB value. For a value of more than one piece or chunk the message names the
//! 1-based piece (chunk) where the failure was detected. The copies are written below `target/`; the bytes the
//! tests rely on are asserted before they are patched.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use common::*;
use oradmp_core::dump::Options;
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::{Error, ErrorCode, Warning, WarningCode};

fn null_opts(max_lob_bytes: u64) -> ReadOptions {
    ReadOptions {
        on_value_error: OnValueError::Null,
        max_lob_bytes,
        ..ReadOptions::default()
    }
}

fn raise_opts(max_lob_bytes: u64) -> ReadOptions {
    ReadOptions {
        max_lob_bytes,
        ..ReadOptions::default()
    }
}

fn nulled(w: &[Warning]) -> Vec<&Warning> {
    w.iter()
        .filter(|w| w.code == WarningCode::ValueNulled)
        .collect()
}

/// The fields of an error that must not differ between the two policies.
fn place(
    e: &Error,
) -> (
    ErrorCode,
    Option<u64>,
    Option<u32>,
    Option<u64>,
    Option<u32>,
) {
    (e.code, e.offset, e.table, e.row, e.column)
}

/// T_LONG row 3 of `18c/long_exp_direct_18c.dmp` is a chain of 5 pieces: the headers `9c 80` @0x2288 (156 bytes,
/// more follow), `19 9f` @0x2326 and @0x615c (7,961 bytes each), the first data byte @0x228a (ADR-43 probe, the
/// bytes asserted below). The value is 32,000 bytes long.
///
/// (i) `max_lob_bytes=20000`, `raise`: `E-LOB-TOO-LARGE` @0x228a. (ii) `null`: `W-VALUE-NULLED` @0x228a, row 3
/// (before T-12 it stood at 0x615e, the piece that passed the limit). (iii) byte 0x2325 (the last data byte of
/// piece 1) -> `c3`: the sequence is cut by the header of piece 2, `E-BAD-TEXT` @0x228a with "piece 2" in the
/// message (before T-12: 0x2328). (iv) the same with `null`: `W-VALUE-NULLED` @0x228a.
#[test]
fn one_chain_value_reports_its_start_under_every_policy() {
    const START: u64 = 0x228a;
    let rel = "18c/long_exp_direct_18c.dmp";
    let good = std::fs::read(fixtures().join(rel)).unwrap();
    assert_eq!(&good[0x2288..0x228a], &[0x9c, 0x80]);
    assert_eq!(&good[0x2326..0x2328], &[0x19, 0x9f]);
    assert_eq!(&good[0x615c..0x615e], &[0x19, 0x9f]);
    // the last data byte of piece 1 is ASCII in the fixture
    assert!(good[0x2325].is_ascii());

    // (i), (ii): the value is above the limit
    let mut d = open_fixture(rel);
    let e = try_read(&mut d, "T_LONG", &raise_opts(20_000)).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.row, e.column, e.length),
        (
            ErrorCode::LobTooLarge,
            Some(START),
            Some(3),
            Some(3),
            Some(24_039)
        )
    );
    let (_, w) = read(&mut d, "T_LONG", &null_opts(20_000));
    let n = nulled(&w);
    assert_eq!(n.len(), 1, "{w:?}");
    assert_eq!(
        (n[0].offset, n[0].row, n[0].column, n[0].count),
        (Some(START), Some(3), Some(3), Some(1))
    );

    // (iii), (iv): a sequence cut between piece 1 and piece 2
    let p = patched_file(rel, "t12", "chain_cut.dmp", |b| {
        b[0x2325] = 0xc3;
    });
    let mut d = open_path(&p, Options::default());
    let e = try_read(&mut d, "T_LONG", &raise_opts(256 << 20)).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.row, e.column),
        (ErrorCode::BadText, Some(START), Some(3), Some(3))
    );
    assert!(
        e.message.ends_with(" (piece 2 of the value)"),
        "{}",
        e.message
    );
    let (_, w) = read(&mut d, "T_LONG", &null_opts(256 << 20));
    let n = nulled(&w);
    assert_eq!(n.len(), 1, "{w:?}");
    assert_eq!(
        (n[0].offset, n[0].row, n[0].column, n[0].count),
        (Some(START), Some(3), Some(3), Some(1))
    );
}

/// `T_LOB` row 5, column C of `18c/lob_exp_18c.dmp` is a long-form CLOB: `fc ff` + the total 200,000 as u64 LE
/// (`40 0d 03 00 00 00 00 00`), then chunks, the first one 16,120 bytes (UTF-16 BE; the characters at the 0-based positions 8,059 and 8,060 of the text are dots, the padding
/// of the block `chunk009`, see `t6l_lob.rs::text_100k`).
/// The last code unit of chunk 1 becomes `d8 00` (a high surrogate, [L5]); chunk 2 begins with `00 2e`, not a
/// low surrogate: `E-BAD-TEXT` (`raise`) or `W-VALUE-NULLED` (`null`), both at the offset `lob_start`
/// receives (the `fc ff`), "chunk 2" in the message.
#[test]
fn one_lob_value_reports_its_start_under_every_policy() {
    let (p, start) = lob_chunk_cut_copy("t12", "lob_cut.dmp");
    let mut d = open_path(&p, Options::default());
    let e = try_read(&mut d, "T_LOB", &raise_opts(256 << 20)).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadText);
    assert_eq!(e.offset, Some(start), "{e}");
    assert_eq!((e.row, e.table), (Some(5), Some(1)));
    // the exact suffix: `contains("chunk 2")` would pass "chunk 20" (review 04 S-5)
    assert!(
        e.message.ends_with(" (chunk 2 of the value)"),
        "{}",
        e.message
    );
    let (_, w) = read(&mut d, "T_LOB", &null_opts(256 << 20));
    let n = nulled(&w);
    assert_eq!(n.len(), 1, "{w:?}");
    assert_eq!((n[0].offset, n[0].row), (Some(start), Some(5)));
    assert_eq!(n[0].column, e.column);
    // the same place under both policies
    assert_eq!(n[0].offset, e.offset);
    // the error and the warning name the same table (review 03 N-4: the result of `place` is asserted)
    assert_eq!(place(&e).2, n[0].table);
}

/// `W-LONG-CHARSET` of a chain (ADR-43 item 3): in a database whose set is not AL32UTF8 a LONG byte above 0x7f
/// is "assumed the same". The header of `18c/long_exp_direct_18c.dmp` gets the database set 171 (`00 ab` after
/// `03 69 03 69 07 d0`, as `t6_patched.rs`) and the byte 0x2330, inside piece 2 of the chain of T_LONG row 3
/// (data 0x2328 to 0x4241), becomes `e9`: one warning at the start of the value (0x228a, row 3), not at piece 2;
/// under `strict` (with the `db_charset` override, so that the mismatch of the VARCHAR2 descriptors does not come
/// first) the `E-STRICT` is at the same place.
#[test]
fn the_long_charset_warning_of_a_chain_is_at_the_value_start() {
    let rel = "18c/long_exp_direct_18c.dmp";
    let p = patched_file(rel, "t12", "chain_charset.dmp", |b| {
        let h = find_first(&b[..0x200], &[0x03, 0x69, 0x03, 0x69, 0x07, 0xd0]);
        b[h + 2] = 0x00;
        b[h + 3] = 0xab;
        assert_eq!(&b[0x2326..0x2328], &[0x19, 0x9f]);
        assert!(b[0x2330].is_ascii());
        b[0x2330] = 0xe9;
    });
    let mut d = open_path(&p, Options::default());
    assert_eq!(d.info().db_charset_id, 171);
    let (_, w) = read(&mut d, "T_LONG", &ReadOptions::default());
    let l: Vec<_> = w
        .iter()
        .filter(|w| w.code == WarningCode::LongCharset)
        .collect();
    assert_eq!(l.len(), 1, "{w:?}");
    assert_eq!(
        (l[0].offset, l[0].row, l[0].column, l[0].count),
        (Some(0x228a), Some(3), Some(3), Some(1))
    );
    // strict: with the descriptors (873) differing from the patched header the mismatch warning would come first, so
    // the `db_charset` override names the set and the mismatch is not raised (arch 6.1)
    let o = Options {
        strict: true,
        db_charset: Some("CL8MSWIN1251".to_string()),
        ..Options::default()
    };
    let mut d = open_path(&p, o);
    let e = try_read(&mut d, "T_LONG", &ReadOptions::default()).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.row, e.warning),
        (
            ErrorCode::Strict,
            Some(0x228a),
            Some(3),
            Some(WarningCode::LongCharset)
        )
    );
}
