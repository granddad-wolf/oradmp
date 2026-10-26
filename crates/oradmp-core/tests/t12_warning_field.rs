//! Task T-12 (a) / (b), ADR-41 item 1: `Error::warning` is set on **every** error a strict conversion produced, so
//! that "did strict cause this?" is one test for every error class. `error.rs` unit-tests `Error::from_warning`
//! for every code; these tests cover the two places where the reader makes the error itself: a NUMBER that
//! `round` would have rounded (`E-NUMBER-OVERFLOW` from `W-NUMBER-ROUNDED`) and a CLOB / NCLOB value decoded
//! under `unconfirmed_lob="decode"` (`E-LOB-UNCONFIRMED` from `W-LOB-UNTESTED`). An error that strict did not cause
//! (an integer part that does not fit, the refusal of ADR-25 without `decode`) has no warning.
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
use oradmp_core::reader::{ReadOptions, UnconfirmedLob};
use oradmp_core::{ErrorCode, WarningCode};

fn strict() -> Options {
    Options {
        strict: true,
        ..Options::default()
    }
}

/// `t_num3_exp_18c.dmp`: six values with more than 10 decimals (truth_num3.txt) are rounded without `strict`;
/// with it row 1 is `E-NUMBER-OVERFLOW` and the error names `W-NUMBER-ROUNDED`. `t_num_exp_18c.dmp` row 19 has an
/// integer part above 28 digits: an overflow under every policy, nothing to do with `strict`, no warning.
#[test]
fn a_strict_round_error_names_the_warning_and_an_integer_overflow_does_not() {
    let mut d = open_path(&fixtures().join("18c/t_num3_exp_18c.dmp"), strict());
    let e = try_read(&mut d, "T_NUM3", &ReadOptions::default()).unwrap_err();
    assert_eq!(
        (e.code, e.row, e.column, e.warning),
        (
            ErrorCode::NumberOverflow,
            Some(1),
            Some(2),
            Some(WarningCode::NumberRounded)
        )
    );
    let mut d = open_path(&fixtures().join("18c/t_num_exp_18c.dmp"), strict());
    let e = try_read(&mut d, "T_NUM", &ReadOptions::default()).unwrap_err();
    assert_eq!(
        (e.code, e.row, e.column, e.warning),
        (ErrorCode::NumberOverflow, Some(19), Some(2), None)
    );
    // without strict the same overflow has no warning either
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let e = try_read(&mut d, "T_NUM", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.warning, None);
}

/// The 178 copy of T-6M (d): with `unconfirmed_lob="decode"` the value would be read with `W-LOB-UNTESTED`; under
/// `strict` it is `E-LOB-UNCONFIRMED` and the error names that warning. Without `decode` the refusal is the
/// ordinary one of ADR-25, strict or not: no warning.
#[test]
fn a_strict_lob_error_names_the_warning_only_when_decode_was_asked_for() {
    let p = patched_file(
        "11g-cp1251/ru_lob2_exp_cp1251_11g.dmp",
        "t12",
        "warning_field_178.dmp",
        |b| {
            assert_eq!(b[0x35..0x37], [0x00, 0xab]);
            assert_eq!(b[0x24e3..0x24e5], [0xab, 0x00]);
            b[0x36] = 0xb2;
            b[0x24e3] = 0xb2;
        },
    );
    let decode = ReadOptions {
        unconfirmed_lob: UnconfirmedLob::Decode,
        ..ReadOptions::default()
    };
    let mut d = open_path(&p, strict());
    let e = try_read(&mut d, "R_LOB2", &decode).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.warning),
        (
            ErrorCode::LobUnconfirmed,
            Some(0x2790),
            Some(WarningCode::LobUntested)
        )
    );
    let e = try_read(&mut d, "R_LOB2", &ReadOptions::default()).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.warning),
        (ErrorCode::LobUnconfirmed, Some(0x2790), None)
    );
    let mut d = open_path(&p, Options::default());
    let e = try_read(&mut d, "R_LOB2", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.warning, None);
}
