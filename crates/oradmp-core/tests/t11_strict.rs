//! Task T-11: `strict=True` (architecture 7.1, owner Q-2, ADR-14, ADR-23, ADR-33 item 3, ADR-34 item 5).
//!
//! Every warning outside the confirmed rules is an error under `strict`: `open` fails on the header's, `scan` on
//! the walker's and the resolution's, `read_table` and the iteration on the reader's. One test per case of the
//! plan (session 5, session 6 (a), session 7 (d), (e), session 8 (d)); each shows first that the same file
//! without `strict` gives that warning (code and offset) and then the error under `strict` (code, offset, the
//! warning it stands for). `W-NUMBER-NULLED` and `W-VALUE-NULLED` report a chosen policy and do not respond.
//! The last tests run all 229 EXP files under `strict` and compare the outcome with the documented list.
//!
//! Expected numbers: the plan (the 178 copy of T-6M (d): N row 1 @0x2790, C row 2 @0x29c4), `truth_num2.txt`
//! (rows 13, 14 are rounded), `truth_num3.txt` (all six rows are rounded: row 1 is the first), the offsets of
//! the walker's report for the structure of the patched copies.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::collections::BTreeMap;
use std::path::Path;

use common::truth_checks::golden_sections;
use common::*;
use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::exp::items::{walk_path, WalkOptions, WalkReport};
use oradmp_core::exp::rows::LobForm;
use oradmp_core::reader::{OnValueError, ReadOptions, UnconfirmedLob};
use oradmp_core::values::number::OverflowPolicy;
use oradmp_core::{Error, ErrorCode, Warning, WarningCode};

fn strict() -> Options {
    Options {
        strict: true,
        ..Options::default()
    }
}

fn walked_rel(rel: &str) -> WalkReport {
    walk_path(
        &fixtures().join(rel),
        &WalkOptions {
            record_lob: true,
            ..WalkOptions::default()
        },
    )
    .unwrap()
}

fn put_u16(b: &mut [u8], at: u64, v: u16) {
    let at = usize::try_from(at).unwrap();
    b[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

/// Inserts bytes and takes as many zero bytes off the end (the files end in zero fill), so that the size stays a
/// multiple of RECORDLENGTH and `W-FILE-SIZE` does not join the warning under test.
fn insert_keeping_size(b: &mut Vec<u8>, at: usize, ins: &[u8]) {
    b.splice(at..at, ins.iter().copied());
    let cut = b.len() - ins.len();
    assert!(b[cut..].iter().all(|&x| x == 0), "the fill is too short");
    b.truncate(cut);
}

/// Replaces `old` at `at` by `new` and keeps the size of the file (zero fill is added or taken off the end).
fn replace_keeping_size(b: &mut Vec<u8>, at: usize, old: usize, new: &[u8]) {
    b.splice(at..at + old, new.iter().copied());
    if new.len() >= old {
        let cut = b.len() - (new.len() - old);
        assert!(b[cut..].iter().all(|&x| x == 0), "the fill is too short");
        b.truncate(cut);
    } else {
        b.extend(std::iter::repeat_n(0u8, old - new.len()));
    }
}

/// The warnings of `open` and the scan without `strict`.
fn plain_warnings(p: &Path) -> Vec<Warning> {
    let mut d = open(p, Options::default()).unwrap();
    d.scan().unwrap();
    d.warnings().to_vec()
}

/// The patched file gives exactly one warning of `code` at `offset` without `strict`, and under `strict` the scan
/// (or, for a header warning, the `open`) fails with `error` at the same offset, naming the warning.
fn scan_case(p: &Path, code: WarningCode, offset: u64, error: ErrorCode) -> Error {
    let ws = plain_warnings(p);
    let hits: Vec<&Warning> = ws.iter().filter(|w| w.code == code).collect();
    assert_eq!(hits.len(), 1, "{code}: {ws:?}");
    assert_eq!(hits[0].offset, Some(offset), "{code}");
    let e = match open(p, strict()) {
        Err(e) => e,
        Ok(mut d) => d.scan().unwrap_err(),
    };
    assert_eq!(
        (e.code, e.offset, e.warning),
        (error, Some(offset), Some(code)),
        "{e}"
    );
    e
}

// ---------------------------------------------------------------------------------------------
// the header's warnings: `open` fails
// ---------------------------------------------------------------------------------------------

#[test]
fn strict_untested_version() {
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "s_version.dmp", |b| {
        let at = find_first(b, b"EXPORT:V18.00.00") + 10;
        b[at..at + 6].copy_from_slice(b"10.02.");
    });
    let e = scan_case(&p, WarningCode::UntestedVersion, 3, ErrorCode::Strict);
    assert_eq!(e.length, Some(16));
}

#[test]
fn strict_untested_header() {
    let mut at = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "s_header.dmp", |b| {
        at = find_first(b, b"\n0\n72\n0\n") + 3;
        b[at..at + 2].copy_from_slice(b"73");
    });
    scan_case(
        &p,
        WarningCode::UntestedHeader,
        at as u64,
        ErrorCode::Strict,
    );
}

#[test]
fn strict_file_size() {
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "s_size.dmp", |b| {
        b.extend_from_slice(&[0u8; 7])
    });
    scan_case(&p, WarningCode::FileSize, 0, ErrorCode::Strict);
}

// ---------------------------------------------------------------------------------------------
// the walker's warnings: `scan` fails (session 5; session 6 (a))
// ---------------------------------------------------------------------------------------------

#[test]
fn strict_unknown_intro_and_trailing() {
    let mut at = 0;
    let p = patched_file("18c/stat_exp_18c.dmp", "t11", "s_intro.dmp", |b| {
        at = find_first(b, b"\nANALSTATS IS ") + 1;
        b[at + 8] = b'T';
    });
    scan_case(&p, WarningCode::UnknownIntro, at as u64, ErrorCode::Strict);
    let mut last = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "s_trailing.dmp", |b| {
        last = b.len() - 1;
        b[last] = 1;
    });
    scan_case(&p, WarningCode::Trailing, last as u64, ErrorCode::Strict);
}

#[test]
fn strict_untested_layout_of_a_lob_entry() {
    let r = walked_rel("18c/lob_exp_18c.dmp");
    let e1 = &r.sections[0].parts[0].header.lob_entries[1];
    let tail_at = e1.offset + 1 + e1.name.len() as u64;
    let p = patched_file("18c/lob_exp_18c.dmp", "t11", "s_l1.dmp", |b| {
        b[tail_at as usize] = 1
    });
    scan_case(&p, WarningCode::UntestedLayout, tail_at, ErrorCode::Strict);
}

#[test]
fn strict_a_piece_of_length_zero() {
    // `long_exp_direct_18c.dmp` (exp.md 6.6): the second piece of T_LONG row 3 stands at 0x2326; `00 80` (zero
    // bytes, more follows) is inserted before it
    let p = patched_file(
        "18c/long_exp_direct_18c.dmp",
        "t11",
        "s_zero_piece.dmp",
        |b| insert_keeping_size(b, 0x2326, &[0x00, 0x80]),
    );
    let e = scan_case(&p, WarningCode::UntestedLayout, 0x2326, ErrorCode::Strict);
    assert_eq!((e.table, e.row, e.column), (Some(1), Some(3), Some(3)));
}

#[test]
fn strict_a_long_lob_value_with_a_total_of_zero() {
    // ADR-23: the first long value of T_LOB becomes `fc ff` and 8 zero bytes
    let r = walked_rel("18c/lob_exp_18c.dmp");
    let d = r.sections[0].parts[0].rows.lob_detail.as_ref().unwrap();
    let v = d.values.iter().find(|v| v.form == LobForm::Long).unwrap();
    let start = usize::try_from(v.offset).unwrap();
    let len = 2 + 8 + v.chunks.iter().map(|c| 2 + usize::from(*c)).sum::<usize>();
    let p = patched_file("18c/lob_exp_18c.dmp", "t11", "s_t0.dmp", |b| {
        let tail = b[start + len..].to_vec();
        b.truncate(start);
        b.extend_from_slice(&[0xfc, 0xff, 0, 0, 0, 0, 0, 0, 0, 0]);
        b.extend_from_slice(&tail);
        b.extend(std::iter::repeat_n(0u8, len - 10));
    });
    let e = scan_case(&p, WarningCode::UntestedLayout, v.offset, ErrorCode::Strict);
    assert_eq!((e.row, e.column), (Some(v.row), Some(v.column)));
}

#[test]
fn strict_a_part_without_its_partition_line() {
    // ADR-21: the second part of `part_exp_18c.dmp` is not directly preceded by a PARTITION line
    let r = walked_rel("18c/part_exp_18c.dmp");
    let second = r.sections[0].parts[1].insert_offset;
    let p = patched_file("18c/part_exp_18c.dmp", "t11", "s_part.dmp", |b| {
        let at = usize::try_from(second).unwrap();
        let line = b[..at]
            .windows(11)
            .rposition(|w| w == b"PARTITION \"")
            .unwrap();
        b[line] = b'X';
    });
    scan_case(&p, WarningCode::UntestedLayout, second, ErrorCode::Strict);
}

#[test]
fn strict_composite_partition_lines_out_of_order() {
    // ADR-21 / ADR-29: a SUBPARTITION line with no open COMPARTITION, inserted before the first COMPARTITION
    let good = std::fs::read(fixtures().join("18c/part2_exp_18c.dmp")).unwrap();
    let com1 = find_first(&good, b"\nCOMPARTITION \"P1\"\n") + 1;
    let p = patched_file("18c/part2_exp_18c.dmp", "t11", "s_composite.dmp", |b| {
        insert_keeping_size(b, com1, b"SUBPARTITION \"ZZ\"\n")
    });
    scan_case(
        &p,
        WarningCode::UntestedLayout,
        com1 as u64,
        ErrorCode::Strict,
    );
    // a PARTITION line outside any section (ADR-29)
    let own = std::fs::read(fixtures().join("18c/own_exp_18c.dmp")).unwrap();
    let at = find_first(&own, b"\nENDTABLE\n") + 10;
    let p = patched_file("18c/own_exp_18c.dmp", "t11", "s_outside.dmp", |b| {
        insert_keeping_size(b, at, b"PARTITION \"P1\"\n")
    });
    scan_case(
        &p,
        WarningCode::UntestedLayout,
        at as u64,
        ErrorCode::Strict,
    );
}

#[test]
fn strict_a_later_part_in_another_column_order() {
    // ADR-32 item 4: part 2 of `part_exp_18c.dmp` names ID and AMOUNT in the other order (same descriptor)
    const INS: &[u8] = b"INSERT INTO \"T_PART\" (\"ID\", \"SALE_DATE\", \"AMOUNT\") VALUES";
    const SWAPPED: &[u8] = b"INSERT INTO \"T_PART\" (\"AMOUNT\", \"SALE_DATE\", \"ID\") VALUES";
    let r = walked_rel("18c/part_exp_18c.dmp");
    let second = r.sections[0].parts[1].insert_offset;
    let p = patched_file("18c/part_exp_18c.dmp", "t11", "s_order.dmp", |b| {
        let first = find_first(b, INS);
        let rel = find_first(&b[first + 1..], INS) + first + 1;
        b[rel..rel + INS.len()].copy_from_slice(SWAPPED);
    });
    let e = scan_case(&p, WarningCode::UntestedLayout, second, ErrorCode::Strict);
    assert_eq!((e.table, e.part), (Some(1), Some(2)));
}

#[test]
fn strict_e2a_a_long_first_column_without_rows_is_framing() {
    // ADR-22 / ADR-23: `stat_exp_direct_18c.dmp` T_STAT4 (an empty direct-path table) with its first column
    // patched to LONG of length 0: no rows and a warning without `strict`, `Framing` at the LF with it
    let r = walked_rel("18c/stat_exp_direct_18c.dmp");
    let t4 = &r.sections[3].parts[0];
    let c = t4.header.columns[0];
    let at = t4.rows.end_marker_offset;
    let p = patched_file("18c/stat_exp_direct_18c.dmp", "t11", "s_e2a.dmp", |b| {
        put_u16(b, c.offset, 8);
        put_u16(b, c.offset + 2, 0);
    });
    // (the patched descriptor no longer agrees with the declaration of the column: W-DDL-FALLBACK joins)
    let ws = plain_warnings(&p);
    let hit: Vec<&Warning> = ws
        .iter()
        .filter(|w| w.code == WarningCode::UntestedLayout)
        .collect();
    assert_eq!(hit.len(), 1, "{ws:?}");
    assert_eq!((hit[0].offset, hit[0].table), (Some(at), Some(4)));
    let e = open(&p, strict()).unwrap().scan().unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.table),
        (ErrorCode::Framing, Some(at), Some(4))
    );
}

// ---------------------------------------------------------------------------------------------
// the resolution's warnings: `scan` fails
// ---------------------------------------------------------------------------------------------

#[test]
fn strict_ddl_fallback_of_a_column_text() {
    let mut at = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "s_ddl.dmp", |b| {
        at = find_first(b, b"\"V\" NUMBER)");
        b[at + 8] = b'F';
    });
    let e = scan_case(&p, WarningCode::DdlFallback, at as u64, ErrorCode::Strict);
    assert_eq!((e.table, e.column), (Some(1), Some(2)));
}

#[test]
fn strict_ddl_fallback_for_a_declared_number_no_decimal128_holds() {
    // ADR-30 item 3: `N_STAR NUMBER(*,2)` of T_DECL becomes `NUMBER(*,40)`; no Decimal128 holds it
    let mut at = 0;
    let p = patched_file("18c/t_decl_exp_18c.dmp", "t11", "s_nodec.dmp", |b| {
        at = find_first(b, b"\"N_STAR\" NUMBER(*,2)");
        replace_keeping_size(b, at + 18, 1, b"40");
    });
    let e = scan_case(&p, WarningCode::DdlFallback, at as u64, ErrorCode::Strict);
    assert_eq!((e.table, e.column), (Some(1), Some(2)));
}

#[test]
fn strict_tsltz_is_a_scan_error_and_one_warning_per_column_without_it() {
    // session 7 (e): `t_tstz_exp_18c.dmp` has one TSLTZ column (type 231)
    let r = walked_rel("18c/t_tstz_exp_18c.dmp");
    let cols = &r.sections[0].parts[0].header.columns;
    let ltz: Vec<usize> = (0..cols.len())
        .filter(|&i| cols[i].type_code == 231)
        .collect();
    assert_eq!(ltz.len(), 1);
    let p = fixtures().join("18c/t_tstz_exp_18c.dmp");
    let ws = plain_warnings(&p);
    assert_eq!(
        ws.iter().filter(|w| w.code == WarningCode::Tsltz).count(),
        1,
        "one warning per TSLTZ column"
    );
    let e = scan_case(
        &p,
        WarningCode::Tsltz,
        cols[ltz[0]].offset,
        ErrorCode::Strict,
    );
    assert_eq!(e.column, Some(u32::try_from(ltz[0] + 1).unwrap()));
}

#[test]
fn strict_charset_unsupported_is_unsupported_charset() {
    let r = walked_rel("18c/misc_exp_18c.dmp");
    let t = r
        .sections
        .iter()
        .find(|s| s.name_raw == b"T_FRAME")
        .unwrap();
    let c0 = t.parts[0].header.columns[0];
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "s_set.dmp", |b| {
        put_u16(b, c0.offset + 4, 1000)
    });
    let e = scan_case(
        &p,
        WarningCode::CharsetUnsupported,
        c0.offset,
        ErrorCode::UnsupportedCharset,
    );
    assert_eq!((e.table, e.column), (Some(e.table.unwrap()), Some(1)));
}

#[test]
fn strict_possibly_incomplete() {
    let rel = "11g-cp1251/ru_exp_us7ascii_11g.dmp";
    let mut d = open_fixture(rel);
    let t = d
        .tables()
        .unwrap()
        .iter()
        .find(|t| t.possibly_incomplete)
        .unwrap()
        .clone();
    let e = open(fixtures().join(rel), strict())
        .unwrap()
        .scan()
        .unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.table, e.warning),
        (
            ErrorCode::Strict,
            Some(t.create_offset),
            Some(t.ordinal),
            Some(WarningCode::PossiblyIncomplete)
        )
    );
}

// ---------------------------------------------------------------------------------------------
// the reader's warnings: `read_table` or the iteration fails (session 6 (a), session 7 (d), session 8 (d))
// ---------------------------------------------------------------------------------------------

fn read_strict(p: &Path, o: Options, table: &str, r: &ReadOptions) -> Result<Vec<Warning>, Error> {
    let mut d = open(p, o)?;
    try_read(&mut d, table, r).map(|(_, w)| w)
}

/// `read` without `strict` warns once with `code` at `offset`; with it the read fails with `error`.
fn read_case(
    p: &Path,
    o: Options,
    table: &str,
    r: &ReadOptions,
    code: WarningCode,
    offset: u64,
    error: ErrorCode,
) -> Error {
    let ws = read_strict(p, o.clone(), table, r).unwrap();
    let hits: Vec<&Warning> = ws.iter().filter(|w| w.code == code).collect();
    assert_eq!(hits.len(), 1, "{code}: {ws:?}");
    assert_eq!(hits[0].offset, Some(offset), "{code}");
    let s = Options { strict: true, ..o };
    let e = read_strict(p, s, table, r).unwrap_err();
    assert_eq!((e.code, e.offset), (error, Some(offset)), "{e}");
    e
}

fn first_value(rel: &str, table: &str, row: usize, column: usize) -> u64 {
    value_in(&fixtures().join(rel), table, row, column)
}

fn value_in(p: &Path, table: &str, row: usize, column: usize) -> u64 {
    let mut d = open(p, Options::default()).unwrap();
    let data = std::fs::read(d.path()).unwrap();
    let ncols = d.table(TableRef::ByName(table)).unwrap().columns.len();
    field_offsets(&data, row_starts(&mut d, table)[row - 1], ncols)[column - 1].unwrap()
}

#[test]
fn strict_unverified_type() {
    let at = first_value("18c/misc_exp_18c.dmp", "T_ROWID", 1, 2);
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "s_rowid.dmp", |b| {
        b[at as usize + 4] = 0x18
    });
    let e = read_case(
        &p,
        Options::default(),
        "T_ROWID",
        &ReadOptions::default(),
        WarningCode::UnverifiedType,
        at,
        ErrorCode::Strict,
    );
    assert_eq!(
        (e.table, e.part, e.row, e.column),
        (Some(e.table.unwrap()), Some(1), Some(1), Some(2))
    );
    assert_eq!(e.warning, Some(WarningCode::UnverifiedType));
}

#[test]
fn strict_long_length() {
    // T-19: T_LONG3 row 7 of `long3_exp_18c.dmp` with 100,001 bytes (see `long3_100001_copy`)
    let (p, at) = long3_100001_copy("t11", "s_long.dmp");
    let e = read_case(
        &p,
        Options::default(),
        "T_LONG3",
        &ReadOptions::default(),
        WarningCode::LongLength,
        at,
        ErrorCode::Strict,
    );
    assert_eq!((e.row, e.column), (Some(7), Some(2)));
}

#[test]
fn strict_long_charset() {
    // the override `db_charset=CL8MSWIN1251` makes the LONG bytes above 0x7f a single-byte text (no mismatch
    // warning with an override): W-LONG-CHARSET (ADR-15 item 4)
    let at = first_value("18c/long2_exp_18c.dmp", "T_LONG2", 1, 2);
    let o = Options {
        db_charset: Some("CL8MSWIN1251".to_string()),
        ..Options::default()
    };
    let p = fixtures().join("18c/long2_exp_18c.dmp");
    let e = read_case(
        &p,
        o,
        "T_LONG2",
        &ReadOptions::default(),
        WarningCode::LongCharset,
        at,
        ErrorCode::Strict,
    );
    assert_eq!((e.row, e.column), (Some(1), Some(2)));
}

#[test]
fn strict_longer_than_declared() {
    let desc = [
        0x03, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69, 0x03, 0x01, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69,
        0x03, 0x01, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69, 0x03, 0x01, 0x00,
    ];
    // the descriptor length of column A becomes 3 and the declaration `VARCHAR2(3)`: they agree, so the scan
    // is quiet and the reader's W-LONGER-THAN-DECLARED is the only warning
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "s_declared.dmp", |b| {
        let d = find_one(b, &desc);
        b[d + 4] = 3;
        let t = find_first(b, b"CREATE TABLE \"T_FRAME\" (\"A\" VARCHAR2(10)") + 37;
        replace_keeping_size(b, t, 2, b"3");
    });
    let at = value_in(&p, "T_FRAME", 1, 1);
    let e = read_case(
        &p,
        Options::default(),
        "T_FRAME",
        &ReadOptions::default(),
        WarningCode::LongerThanDeclared,
        at,
        ErrorCode::Strict,
    );
    assert_eq!((e.row, e.column), (Some(1), Some(1)));
}

#[test]
fn strict_charset_mismatch_fails_read_table_itself() {
    // `long2_exp_18c.dmp` with the database set 171 and the option `db_charset` left out: the VARCHAR2 column C
    // has the descriptor set 873 (the descriptor wins, W-CHARSET-MISMATCH at the CREATE TABLE line); the LONG
    // value gives W-LONG-CHARSET in the same read, so the check is on `read_table` (before any row)
    let create = walked_rel("18c/long2_exp_18c.dmp").sections[0].create_offset;
    let p = patched_file("18c/long2_exp_18c.dmp", "t11", "s_mismatch.dmp", |b| {
        let h = find_first(&b[..0x200], &[0x03, 0x69, 0x03, 0x69, 0x07, 0xd0]);
        b[h + 2] = 0x00;
        b[h + 3] = 0xab;
    });
    let mut d = open(&p, Options::default()).unwrap();
    let (_, ws) = read(&mut d, "T_LONG2", &ReadOptions::default());
    let m = ws
        .iter()
        .find(|w| w.code == WarningCode::CharsetMismatch)
        .unwrap();
    assert_eq!((m.offset, m.column), (Some(create), Some(3)));
    let mut d = open(&p, strict()).unwrap();
    let e = d
        .read_table(TableRef::ByName("T_LONG2"), &ReadOptions::default())
        .unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.column, e.warning, e.row),
        (
            ErrorCode::Strict,
            Some(create),
            Some(3),
            Some(WarningCode::CharsetMismatch),
            None
        )
    );
}

/// The 178 copy of T-6M (d): `ru_lob2_exp_cp1251_11g.dmp` with the database set and the CLOB descriptor set 178.
fn lob2_178(name: &str) -> std::path::PathBuf {
    patched_file("11g-cp1251/ru_lob2_exp_cp1251_11g.dmp", "t11", name, |b| {
        assert_eq!(b[0x35..0x37], [0x00, 0xab]);
        assert_eq!(b[0x24e3..0x24e5], [0xab, 0x00]);
        b[0x36] = 0xb2;
        b[0x24e3] = 0xb2;
    })
}

#[test]
fn strict_lob_untested_is_lob_unconfirmed_whatever_unconfirmed_lob_says() {
    // session 8 (d): with `unconfirmed_lob="decode"` the copy gives W-LOB-UNTESTED for C (count 1) and N
    // (count 2) without `strict`; with it, E-LOB-UNCONFIRMED at R_LOB2 row 1 column N (@0x2790, 132 bytes)
    let p = lob2_178("s_lob178.dmp");
    let decode = ReadOptions {
        unconfirmed_lob: UnconfirmedLob::Decode,
        ..ReadOptions::default()
    };
    let ws = read_strict(&p, Options::default(), "R_LOB2", &decode).unwrap();
    let counts: Vec<(u32, u64)> = ws
        .iter()
        .filter(|w| w.code == WarningCode::LobUntested)
        .map(|w| (w.column.unwrap(), w.count.unwrap()))
        .collect();
    assert_eq!(counts, [(2, 1), (3, 2)]);
    let e = read_strict(&p, strict(), "R_LOB2", &decode).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.row, e.column, e.length),
        (
            ErrorCode::LobUnconfirmed,
            Some(0x2790),
            Some(1),
            Some(3),
            Some(132)
        )
    );
    // a value error: `on_value_error=null` turns the refused values into NULL with a counted W-VALUE-NULLED
    let null = ReadOptions {
        on_value_error: OnValueError::Null,
        ..decode
    };
    let ws = read_strict(&p, strict(), "R_LOB2", &null).unwrap();
    assert!(ws.iter().all(|w| w.code != WarningCode::LobUntested));
    let n: Vec<(u32, u64)> = ws
        .iter()
        .filter(|w| w.code == WarningCode::ValueNulled)
        .map(|w| (w.column.unwrap(), w.count.unwrap()))
        .collect();
    assert_eq!(n, [(2, 1), (3, 2)]);
}

#[test]
fn strict_a_value_round_would_change_is_e_number_overflow() {
    // `t_num3_exp_18c.dmp` (truth_num3.txt): all six values have more than 10 decimals: rounded without
    // `strict` (count 6, row 1 first), `E-NUMBER-OVERFLOW` at row 1, column V with it
    let p = fixtures().join("18c/t_num3_exp_18c.dmp");
    let ws = read_strict(&p, Options::default(), "T_NUM3", &ReadOptions::default()).unwrap();
    let w = ws
        .iter()
        .find(|w| w.code == WarningCode::NumberRounded)
        .unwrap();
    assert_eq!((w.count, w.row, w.column), (Some(6), Some(1), Some(2)));
    let at = first_value("18c/t_num3_exp_18c.dmp", "T_NUM3", 1, 2);
    let e = read_strict(&p, strict(), "T_NUM3", &ReadOptions::default()).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.row, e.column),
        (ErrorCode::NumberOverflow, Some(at), Some(1), Some(2))
    );
    // the error is a value error: `on_value_error=null` gives NULL and a counted W-VALUE-NULLED, no rounding
    let null = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let ws = read_strict(&p, strict(), "T_NUM3", &null).unwrap();
    assert!(ws.iter().all(|w| w.code != WarningCode::NumberRounded));
    let w = ws
        .iter()
        .find(|w| w.code == WarningCode::ValueNulled)
        .unwrap();
    assert_eq!((w.count, w.row, w.column), (Some(6), Some(1), Some(2)));
}

#[test]
fn strict_does_not_change_the_null_policies() {
    // `on_number_overflow=null` is a chosen policy: W-NUMBER-NULLED stays a warning under `strict`; so does
    // W-VALUE-NULLED (`on_value_error=null`, T_DATE rows 8 and 9)
    let p = fixtures().join("18c/t_num3_exp_18c.dmp");
    let o = ReadOptions {
        on_number_overflow: OverflowPolicy::Null,
        ..ReadOptions::default()
    };
    let ws = read_strict(&p, strict(), "T_NUM3", &o).unwrap();
    let w = ws
        .iter()
        .find(|w| w.code == WarningCode::NumberNulled)
        .unwrap();
    assert_eq!(w.count, Some(6));
    let p = fixtures().join("18c/t_date_exp_18c.dmp");
    let o = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let ws = read_strict(&p, strict(), "T_DATE", &o).unwrap();
    let w = ws
        .iter()
        .find(|w| w.code == WarningCode::ValueNulled)
        .unwrap();
    assert_eq!((w.count, w.row), (Some(2), Some(8)));
    assert_eq!(arch_strict(WarningCode::NumberNulled), None);
    assert_eq!(arch_strict(WarningCode::ValueNulled), None);
}

// ---------------------------------------------------------------------------------------------
// all 229 EXP files under `strict`
// ---------------------------------------------------------------------------------------------

/// The outcome of one file under the options: the failure of the scan, or the failures of the table reads, as
/// (stage, table name, error code, warning) with the name taken from a non-strict scan of the same file.
type Outcome = Vec<(String, String, &'static str, &'static str)>;

fn outcome(rel: &str, o: Options) -> Outcome {
    let mut names: BTreeMap<u32, String> = BTreeMap::new();
    let mut plain = open_fixture(rel);
    for t in plain.tables().unwrap() {
        names.insert(t.ordinal, t.name.clone());
    }
    let mut d = open(fixtures().join(rel), o).unwrap();
    let mut out = Vec::new();
    let w = |e: &Error| e.warning.map_or("", WarningCode::as_str);
    match d.scan() {
        Err(e) => {
            let t = e
                .table
                .and_then(|o| names.get(&o))
                .cloned()
                .unwrap_or_default();
            out.push(("scan".to_string(), t, e.code.as_str(), w(&e)));
        }
        Ok(_) => {
            let tables: Vec<(u32, String)> = d
                .tables()
                .unwrap()
                .iter()
                .map(|t| (t.ordinal, t.name.clone()))
                .collect();
            for (_, name) in tables {
                if let Err(e) = try_read(&mut d, &name, &ReadOptions::default()) {
                    out.push(("read".to_string(), name, e.code.as_str(), w(&e)));
                }
            }
        }
    }
    out
}

/// Architecture 7.1 as a table (ADR-43 item 4a), written here and never taken from the code under test: one
/// line per `WarningCode::ALL` entry with its outcome under `strict` (the error code, or `None`: `strict`
/// leaves the warning alone) and the place that says so. The default rule of 7.1 ("outside the confirmed
/// rules": `strict` makes an error) gives `E-STRICT` where no other error is named (ADR-41, the session 9
/// paragraph of 7.1).
const ARCH_7_1_STRICT: &[(&str, Option<&str>, &str)] = &[
    ("W-UNKNOWN-INTRO", Some("E-STRICT"), "7.1 outside the confirmed rules: unknown introducing line read by G1; arch 2.5"),
    ("W-TRAILING", Some("E-STRICT"), "7.1 outside the confirmed rules (every warning without an error of its own is E-STRICT, ADR-41)"),
    ("W-UNTESTED-VERSION", Some("E-STRICT"), "7.1 outside the confirmed rules: version line not V18.00.00 / V11.02.00"),
    ("W-UNTESTED-HEADER", Some("E-STRICT"), "7.1 outside the confirmed rules: W-UNTESTED-HEADER"),
    ("W-FILE-SIZE", Some("E-STRICT"), "7.1 outside the confirmed rules: file size not a multiple of R [H5]"),
    ("W-UNTESTED-LAYOUT", Some("E-STRICT"), "7.1 outside the confirmed rules: W-UNTESTED-LAYOUT; arch 7.1 session 3 and 4 list"),
    ("W-DDL-FALLBACK", Some("E-STRICT"), "7.1 outside the confirmed rules: DDL fallback"),
    ("W-CHARSET-UNSUPPORTED", Some("E-UNSUPPORTED-CHARSET"), "7.1 session 9 paragraph (ADR-41): W-CHARSET-UNSUPPORTED under strict is E-UNSUPPORTED-CHARSET"),
    ("W-UNVERIFIED-TYPE", Some("E-STRICT"), "7.1 session 9 paragraph (ADR-41): value-layer warnings are E-STRICT"),
    ("W-LONG-LENGTH", Some("E-STRICT"), "7.1 session 9 paragraph (ADR-41): value-layer warnings are E-STRICT"),
    ("W-LONG-CHARSET", Some("E-STRICT"), "7.1 session 9 paragraph (ADR-41): value-layer warnings are E-STRICT"),
    ("W-LONGER-THAN-DECLARED", Some("E-STRICT"), "7.1 session 9 paragraph (ADR-41): value-layer warnings are E-STRICT"),
    ("W-NUMBER-ROUNDED", Some("E-NUMBER-OVERFLOW"), "7.1 counted change, strict column (owner, session 5): a value round would change is E-NUMBER-OVERFLOW"),
    ("W-NUMBER-NULLED", None, "7.1 counted change: null is unchanged under strict (a policy the caller chose)"),
    ("W-VALUE-NULLED", None, "7.1 value class: on_value_error=null is a policy the caller chose; strict leaves it"),
    ("W-CHARSET-MISMATCH", Some("E-STRICT"), "7.1 outside the confirmed rules; arch 6.1 (descriptor id other than the database set), ADR-41 item 5"),
    ("W-LOB-UNTESTED", Some("E-LOB-UNCONFIRMED"), "7.1 session 7 (ADR-34, ADR-35): strict turns the value into E-LOB-UNCONFIRMED"),
    ("W-TSLTZ", Some("E-STRICT"), "7.1 outside the confirmed rules: TSLTZ; arch 5.1"),
    ("W-POSSIBLY-INCOMPLETE", Some("E-STRICT"), "7.1 outside the confirmed rules: possibly incomplete table"),
];

/// The outcome of 7.1 for a warning code, from the table above.
fn arch_strict(code: WarningCode) -> Option<&'static str> {
    let row = ARCH_7_1_STRICT
        .iter()
        .find(|r| r.0 == code.as_str())
        .unwrap_or_else(|| panic!("{code} is not in ARCH_7_1_STRICT"));
    row.1
}

/// ADR-43 item 4a: the table has exactly the codes of `WarningCode::ALL` (once each, in that order) and
/// `strict_error()` gives the table's outcome for every one. A wrong row of `strict_error()` (mutation a1)
/// fails here without any fixture.
#[test]
fn arch_7_1_strict_covers_every_warning_code_and_equals_strict_error() {
    let table: Vec<&str> = ARCH_7_1_STRICT.iter().map(|r| r.0).collect();
    let all: Vec<&str> = WarningCode::ALL.iter().map(|c| c.as_str()).collect();
    assert_eq!(table, all);
    for r in ARCH_7_1_STRICT {
        assert!(!r.2.is_empty(), "{} has no citation", r.0);
    }
    for c in WarningCode::ALL {
        assert_eq!(
            c.strict_error().map(ErrorCode::as_str),
            arch_strict(c),
            "{c}"
        );
    }
}

/// The documented list as (stage, table key, error code, warning, reason). Under `strict` the scan fails for
/// the files with a TSLTZ column (`W-TSLTZ` of T_TSTZ) and the files with the table whose export failed
/// (`W-POSSIBLY-INCOMPLETE`, X2); a read fails strictly (and only with `strict`) for T_NUM2 and T_NUM3
/// (`W-NUMBER-ROUNDED` becomes `E-NUMBER-OVERFLOW`, and since T-12 the error says so in `Error::warning`, ADR-41
/// item 1). The reads that fail with or without `strict` stay as they
/// are. The number of files of each entry is the number of sections with that table key in
/// `tests/golden/exp_catalog.tsv` (`documented_count`; no count is typed in).
const DOCUMENTED: &[(&str, &str, &str, &str, &str)] = &[
    ("read", "T_DATE", "E-BAD-DATETIME", "", "years -100 and -4712 are E-BAD-DATETIME under every policy (ADR-28; truth_date.txt rows 8, 9, classified by t10_truth::the_classification_of_the_truth_numbers)"),
    ("read", "T_INTERVAL2", "E-BAD-INTERVAL", "", "IYM rows 2, 3 of truth_misc2.txt are E-BAD-INTERVAL (ADR-28)"),
    ("read", "T_NUM", "E-NUMBER-OVERFLOW", "", "integer part too large, row 19 (truth_num.txt, classified by t10_truth::the_classification_of_the_truth_numbers; ADR-28)"),
    ("read", "T_NUM2", "E-NUMBER-OVERFLOW", "W-NUMBER-ROUNDED", "rows 13, 14 of truth_num2.txt are rounded: W-NUMBER-ROUNDED is E-NUMBER-OVERFLOW under strict (arch 7.1, owner)"),
    ("read", "T_NUM3", "E-NUMBER-OVERFLOW", "W-NUMBER-ROUNDED", "all six rows of truth_num3.txt are rounded: W-NUMBER-ROUNDED is E-NUMBER-OVERFLOW under strict (arch 7.1, owner)"),
    ("read", "T_TS", "E-BAD-DATETIME", "", "row 5 column 4 (9999-12-31, TIMESTAMP(9)) is outside the nanosecond range, truth_ts.txt (ADR-28)"),
    ("scan", "???_???", "E-STRICT", "W-POSSIBLY-INCOMPLETE", "failed export (EXP-00008 / ORA-00604 / ORA-00942 in ru_exp_direct_us7ascii_11g.log, EXP-00056 / ORA-00942 in ru_exp_us7ascii_11g.log; ADR-39)"),
    ("scan", "T_TSLTZ2", "E-STRICT", "W-TSLTZ", "TSLTZ column: arch 5.1; T-19b: tsltz2_exp(_direct)_<v>, four files (truth_tsltz2.txt)"),
    ("scan", "T_TSTZ", "E-STRICT", "W-TSLTZ", "TSLTZ column: arch 5.1 and the tsltz= field of truth_tstz.txt"),
    ("scan", "\u{bf}\u{bf}\u{bf}_\u{bf}\u{bf}\u{bf}", "E-STRICT", "W-POSSIBLY-INCOMPLETE", "failed export (EXP-00056 / ORA-00942 in ru_exp_w1252_11g.log; ADR-39)"),
];

/// The key of a reader's table name in the golden catalog, which writes the bytes of a name outside ASCII as
/// `\xNN` (the reader's Latin-1 `\u{bf}` is the byte `bf`).
fn catalog_key(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii() {
                c.to_string()
            } else {
                format!("\\x{:02x}", u32::from(c))
            }
        })
        .collect()
}

/// The number of files of a `DOCUMENTED` entry: the sections of the golden catalog with that table key.
fn documented_count(table: &str) -> usize {
    let key = catalog_key(table);
    golden_sections().iter().filter(|(_, t)| *t == key).count()
}

#[test]
fn strict_over_the_201_files_fails_exactly_on_the_documented_list() {
    let files = exp_files();
    assert_eq!(files.len(), TOTALS.files);
    let mut tally: BTreeMap<(String, String, &str, &str), usize> = BTreeMap::new();
    for rel in &files {
        for (stage, table, code, warning) in outcome(rel, strict()) {
            *tally.entry((stage, table, code, warning)).or_default() += 1;
        }
    }
    let got: Vec<(&str, &str, &str, &str, usize)> = tally
        .iter()
        .map(|((s, t, c, w), n)| (s.as_str(), t.as_str(), *c, *w, *n))
        .collect();
    let mut want: Vec<(&str, &str, &str, &str, usize)> = DOCUMENTED
        .iter()
        .map(|(s, t, c, w, _)| (*s, *t, *c, *w, documented_count(t)))
        .collect();
    want.sort();
    assert_eq!(got, want, "{got:?}");
    // every reason is written down, every count is a real number of sections
    for d in DOCUMENTED {
        assert!(!d.4.is_empty(), "{d:?}");
        assert!(documented_count(d.1) > 0, "{}", d.1);
    }
}

/// The list is not a snapshot of the code under test: it is derived from the non-strict runs. A file or table
/// fails under `strict` exactly if the non-strict scan has a warning that responds to `strict`, or the non-strict
/// read has one or fails. (This is what makes "strict" mean what arch 7.1 says on real data.)
#[test]
fn strict_fails_exactly_where_the_plain_run_has_a_warning_that_responds_to_it_or_fails() {
    for rel in exp_files() {
        let mut d = open_fixture(&rel);
        let scan_hit = {
            d.scan().unwrap();
            d.warnings().iter().any(|w| arch_strict(w.code).is_some())
        };
        let strict_out = outcome(&rel, strict());
        if scan_hit {
            assert_eq!(strict_out.len(), 1, "{rel}");
            assert_eq!(strict_out[0].0, "scan", "{rel}");
            continue;
        }
        let mut expected_tables = Vec::new();
        let names: Vec<String> = d.tables().unwrap().iter().map(|t| t.name.clone()).collect();
        for n in names {
            let fails = match try_read(&mut d, &n, &ReadOptions::default()) {
                Err(_) => true,
                Ok((_, ws)) => ws.iter().any(|w| arch_strict(w.code).is_some()),
            };
            if fails {
                expected_tables.push(n);
            }
        }
        let got: Vec<String> = strict_out.iter().map(|o| o.1.clone()).collect();
        assert!(strict_out.iter().all(|o| o.0 == "read"), "{rel}");
        assert_eq!(got, expected_tables, "{rel}");
    }
}

#[test]
fn the_plain_options_fail_on_none_of_the_scans_and_on_the_same_reads_as_strict_less_the_rounded_ones(
) {
    // without `strict` no scan fails; the reads that fail are those of the documented list that need no
    // `strict` (T_DATE, T_INTERVAL2, T_NUM, T_TS): 26 of the 30 failures under `strict`
    let mut n = 0;
    for rel in exp_files() {
        let o = outcome(&rel, Options::default());
        assert!(o.iter().all(|x| x.0 == "read"), "{rel}");
        for x in &o {
            assert!(
                ["T_DATE", "T_INTERVAL2", "T_NUM", "T_TS"].contains(&x.1.as_str()),
                "{rel}: {x:?}"
            );
        }
        n += o.len();
    }
    assert_eq!(n, 26);
}
