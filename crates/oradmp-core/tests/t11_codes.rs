//! Task T-11: one test per error code and per warning code of architecture 1.4 and 7.1, each asserting the code
//! and the offset (a code that has no offset by nature, such as a bad option, asserts `None`).
//!
//! Every case is a fixture, a patched copy of one (written below `target/`, the fixtures are never touched) or
//! an option that makes the file fail. The expected offsets come from structure the tests read independently of
//! the code under test: the walker's report (`walk_path`: descriptor, `m`, entry and LOB value offsets), a byte
//! search in the patched copy, or `field_offsets`, which walks the u16 length fields of a conventional-path row.
//! Numbers from the plan: the 178 copy of T-6M (d) (`ru_lob2_exp_cp1251_11g.dmp`, header set @0x36, CLOB
//! descriptor set @0x24e3, N row 1 @0x2790 with stored length 132); row numbers from `truth_num.txt` (the first
//! value that needs more than 28 integer digits is row 19) and `truth_date.txt` (year below 1: rows 8, 9).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use common::*;
use oradmp_core::dump::{open, CatalogEnd, Dump, Options, TableRef};
use oradmp_core::exp::items::{walk_path, WalkOptions, WalkReport};
use oradmp_core::reader::{OnValueError, ReadOptions, UnconfirmedLob};
use oradmp_core::values::number::OverflowPolicy;
use oradmp_core::{Error, ErrorCode, Warning, WarningCode};

fn walked(path: &std::path::Path) -> WalkReport {
    walk_path(
        path,
        &WalkOptions {
            record_lob: true,
            ..WalkOptions::default()
        },
    )
    .unwrap()
}

fn walked_rel(rel: &str) -> WalkReport {
    walked(&fixtures().join(rel))
}

fn is(e: &Error, code: ErrorCode, offset: Option<u64>) {
    assert_eq!((e.code, e.offset), (code, offset), "{e}");
}

/// The scan of a patched copy fails; the error.
fn scan_err(p: &std::path::Path, o: Options) -> Error {
    open(p, o).unwrap().scan().unwrap_err()
}

/// The error the scan stopped with (`Catalog.end`).
fn stopped(p: &std::path::Path) -> Error {
    let mut d = open(p, Options::default()).unwrap();
    match d.scan().unwrap().end {
        CatalogEnd::Stopped(ref e) => Error {
            code: e.code,
            offset: e.offset,
            table: e.table,
            part: e.part,
            column: e.column,
            row: e.row,
            length: e.length,
            type_code: e.type_code,
            warning: e.warning,
            message: e.message.clone(),
            source: None,
        },
        CatalogEnd::Complete => panic!("the scan was not stopped"),
    }
}

fn put_u16(b: &mut [u8], at: u64, v: u16) {
    let at = usize::try_from(at).unwrap();
    b[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

// ---------------------------------------------------------------------------------------------
// error codes
// ---------------------------------------------------------------------------------------------

#[test]
fn e_not_exp_file() {
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "not_exp.dmp", |b| {
        b[0] = 0x04
    });
    let e = open(&p, Options::default()).unwrap_err();
    is(&e, ErrorCode::NotExpFile, Some(0));
}

#[test]
fn e_bad_header() {
    // the user line must start with `U`; it follows the version line
    let mut at = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "bad_header.dmp", |b| {
        at = find_first(b, b"\nUCORPUS") + 1;
        b[at] = b'X';
    });
    let e = open(&p, Options::default()).unwrap_err();
    is(&e, ErrorCode::BadHeader, Some(at as u64));
}

#[test]
fn e_bad_record_length() {
    let mut at = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "bad_rl.dmp", |b| {
        at = find_first(b, b"\n8192\n") + 1;
        b[at..at + 4].copy_from_slice(b"0000");
    });
    let e = open(&p, Options::default()).unwrap_err();
    is(&e, ErrorCode::BadRecordLength, Some(at as u64));
}

#[test]
fn e_missing_metrics_line() {
    let mut at = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "no_metrics.dmp", |b| {
        at = find_first(b, b"METRICST");
        b[at] = b'X';
    });
    let e = open(&p, Options::default()).unwrap_err();
    is(&e, ErrorCode::MissingMetricsLine, Some(at as u64));
}

#[test]
fn e_framing() {
    // the first column field of the first row has a length of 0 [R8]
    let rows_offset = walked_rel("18c/t_num_exp_18c.dmp").sections[0].parts[0]
        .header
        .rows_offset;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "framing.dmp", |b| {
        put_u16(b, rows_offset, 0)
    });
    let e = scan_err(&p, Options::default());
    is(&e, ErrorCode::Framing, Some(rows_offset));
    assert_eq!((e.table, e.row, e.column), (Some(1), Some(1), Some(1)));
}

#[test]
fn e_unsupported_type() {
    let r = walked_rel("18c/multi_types_exp_18c.dmp");
    let at = r.sections[2].parts[0].header.columns[0].offset;
    let p = patched_file("18c/multi_types_exp_18c.dmp", "t11", "type114.dmp", |b| {
        put_u16(b, at, 114)
    });
    let e = stopped(&p);
    is(&e, ErrorCode::UnsupportedType, Some(at));
    assert_eq!(e.type_code, Some(114));
}

#[test]
fn e_unsupported_layout() {
    let m_at = walked_rel("18c/lob_exp_18c.dmp").sections[0].parts[0]
        .header
        .m_offset;
    let p = patched_file("18c/lob_exp_18c.dmp", "t11", "m2.dmp", |b| {
        put_u16(b, m_at, 2)
    });
    is(&stopped(&p), ErrorCode::UnsupportedLayout, Some(m_at));
}

#[test]
fn e_truncated() {
    // the second EXIT line is replaced by zero fill (E4): the error stands where the zeros begin
    let end = walked_rel("18c/t_num_exp_18c.dmp").end_offset.unwrap();
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "truncated.dmp", |b| {
        let end = usize::try_from(end).unwrap();
        for x in &mut b[end - 5..end] {
            *x = 0;
        }
    });
    let e = scan_err(&p, Options::default());
    is(&e, ErrorCode::Truncated, Some(end - 5));
}

#[test]
fn e_config() {
    let bad = Options {
        extra_intro_lines: vec!["prefix:3:plain:ab".to_string()],
        ..Options::default()
    };
    let e = open(fixtures().join("18c/t_num_exp_18c.dmp"), bad).unwrap_err();
    is(&e, ErrorCode::Config, None);
}

#[test]
fn e_internal() {
    // ADR-59: the only constructor is the panic guard; the error has no offset, table or row
    let e = oradmp_core::guard::catch_panic::<()>(|| panic!("probe")).unwrap_err();
    is(&e, ErrorCode::Internal, None);
    assert_eq!((e.table, e.column, e.row), (None, None, None));
}

#[test]
fn e_file_changed() {
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "changed.dmp", |_| {});
    let mut d = open(&p, Options::default()).unwrap();
    d.scan().unwrap();
    let mut f = std::fs::OpenOptions::new().append(true).open(&p).unwrap();
    std::io::Write::write_all(&mut f, &[0u8; 7]).unwrap();
    let e = d
        .read_table(TableRef::ByName("T_NUM"), &ReadOptions::default())
        .unwrap_err();
    is(&e, ErrorCode::FileChanged, Some(0));
}

#[test]
fn e_no_such_table() {
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let e = d
        .read_table(TableRef::ByName("NOPE"), &ReadOptions::default())
        .unwrap_err();
    is(&e, ErrorCode::NoSuchTable, None);
}

#[test]
fn e_ambiguous_table_name() {
    // the second TABLE line of `stat_exp_18c.dmp` is renamed (same length) to the name of the first
    let p = patched_file("18c/stat_exp_18c.dmp", "t11", "ambiguous.dmp", |b| {
        let at = find_first(b, b"\nTABLE \"T_STAT2\"\n") + 8;
        b[at..at + 7].copy_from_slice(b"T_STAT1");
    });
    let mut d = open(&p, Options::default()).unwrap();
    let e = d
        .read_table(TableRef::ByName("T_STAT1"), &ReadOptions::default())
        .unwrap_err();
    is(&e, ErrorCode::AmbiguousTableName, None);
    // the ordinal still names one of them
    assert!(d
        .read_table(TableRef::ByOrdinal(2), &ReadOptions::default())
        .is_ok());
}

#[test]
fn e_io() {
    let e = open(
        target_dir("t11").join("does_not_exist.dmp"),
        Options::default(),
    )
    .unwrap_err();
    is(&e, ErrorCode::Io, None);
    assert!(std::error::Error::source(&e).is_some());
}

#[test]
fn e_no_such_column() {
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let o = ReadOptions {
        columns: Some(vec!["NOPE".to_string()]),
        ..ReadOptions::default()
    };
    let e = d.read_table(TableRef::ByName("T_NUM"), &o).unwrap_err();
    is(&e, ErrorCode::NoSuchColumn, None);
    assert_eq!(e.table, Some(1));
}

/// The error of the read of `table` and the absolute offset of the value that the error names, found by walking
/// the length fields of the row (`row` and `column` of the error are 1-based; conventional path).
fn value_error_at(d: &mut Dump, table: &str, o: &ReadOptions) -> (Error, u64) {
    let e = try_read(d, table, o).unwrap_err();
    let data = std::fs::read(d.path()).unwrap();
    let ncols = d.table(TableRef::ByName(table)).unwrap().columns.len();
    let starts = row_starts(d, table);
    let fields = field_offsets(
        &data,
        starts[usize::try_from(e.row.unwrap() - 1).unwrap()],
        ncols,
    );
    let at = fields[usize::try_from(e.column.unwrap() - 1).unwrap()].unwrap();
    (e, at)
}

#[test]
fn e_bad_number() {
    // T_NUM row 1, column ID: `c1 02` (= 1); the digit byte becomes 0xff (positive digits are 01 to 64)
    let mut d0 = open_fixture("18c/t_num_exp_18c.dmp");
    let first = row_starts(&mut d0, "T_NUM")[0];
    let data = std::fs::read(d0.path()).unwrap();
    let at = field_offsets(&data, first, 2)[0].unwrap();
    assert_eq!(data[at as usize..at as usize + 2], [0xc1, 0x02]);
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "bad_number.dmp", |b| {
        b[at as usize + 1] = 0xff
    });
    let mut d = open(&p, Options::default()).unwrap();
    let e = try_read(&mut d, "T_NUM", &ReadOptions::default()).unwrap_err();
    is(&e, ErrorCode::BadNumber, Some(at));
    assert_eq!(
        (e.table, e.part, e.row, e.column),
        (Some(1), Some(1), Some(1), Some(1))
    );
}

#[test]
fn e_number_overflow() {
    // truth_num.txt: row 19 is the first value with an integer part that no Decimal128(38, 10) holds
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let (e, at) = value_error_at(&mut d, "T_NUM", &ReadOptions::default());
    assert_eq!(e.code, ErrorCode::NumberOverflow);
    assert_eq!((e.row, e.column), (Some(19), Some(2)));
    assert_eq!(e.offset, Some(at));
}

#[test]
fn e_bad_datetime() {
    // truth_date.txt: rows 8 and 9 have a year below 1
    let mut d = open_fixture("18c/t_date_exp_18c.dmp");
    let (e, at) = value_error_at(&mut d, "T_DATE", &ReadOptions::default());
    assert_eq!(e.code, ErrorCode::BadDatetime);
    assert_eq!((e.row, e.column), (Some(8), Some(2)));
    assert_eq!(e.offset, Some(at));
}

#[test]
fn e_bad_interval() {
    // truth_misc2.txt: T_INTERVAL2 row 2 has +999999999-11 months, more than 32 bits
    let mut d = open_fixture("18c/misc2_exp_18c.dmp");
    let (e, at) = value_error_at(&mut d, "T_INTERVAL2", &ReadOptions::default());
    assert_eq!(e.code, ErrorCode::BadInterval);
    assert_eq!((e.row, e.column), (Some(2), Some(2)));
    assert_eq!(e.offset, Some(at));
}

#[test]
fn e_bad_rowid() {
    // T_ROWID row 1, column RID: the first text byte becomes `!` (outside the base-64 alphabet)
    let mut d0 = open_fixture("18c/misc_exp_18c.dmp");
    let first = row_starts(&mut d0, "T_ROWID")[0];
    let data = std::fs::read(d0.path()).unwrap();
    let at = field_offsets(&data, first, 3)[1].unwrap();
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "bad_rowid.dmp", |b| {
        b[at as usize + 5] = b'!'
    });
    let mut d = open(&p, Options::default()).unwrap();
    let e = try_read(&mut d, "T_ROWID", &ReadOptions::default()).unwrap_err();
    is(&e, ErrorCode::BadRowid, Some(at));
    assert_eq!((e.row, e.column), (Some(1), Some(2)));
}

#[test]
fn e_bad_text() {
    // T_FRAME row 1, column A (`AAAA`): the first byte becomes 0xff, which is not UTF-8
    let mut d0 = open_fixture("18c/misc_exp_18c.dmp");
    let first = row_starts(&mut d0, "T_FRAME")[0];
    let data = std::fs::read(d0.path()).unwrap();
    let at = field_offsets(&data, first, 3)[0].unwrap();
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "bad_text.dmp", |b| {
        b[at as usize] = 0xff
    });
    let mut d = open(&p, Options::default()).unwrap();
    let e = try_read(&mut d, "T_FRAME", &ReadOptions::default()).unwrap_err();
    is(&e, ErrorCode::BadText, Some(at));
    assert_eq!((e.row, e.column), (Some(1), Some(1)));
}

#[test]
fn e_lob_too_large() {
    // T_LOB: the first value whose stored length is above 1,000 bytes is refused before a byte is read; the
    // walker's list of values has its offset (the first u16 of the value) and its stored length
    let r = walked_rel("18c/lob_exp_18c.dmp");
    let d = r.sections[0].parts[0].rows.lob_detail.as_ref().unwrap();
    let v = d.values.iter().find(|v| v.total > 1000).unwrap();
    let mut dump = open_fixture("18c/lob_exp_18c.dmp");
    let o = ReadOptions {
        max_lob_bytes: 1000,
        ..ReadOptions::default()
    };
    let e = try_read(&mut dump, "T_LOB", &o).unwrap_err();
    is(&e, ErrorCode::LobTooLarge, Some(v.offset));
    assert_eq!(e.length, Some(v.total));
}

/// `ru_lob2_exp_cp1251_11g.dmp` with the database set 178 and the CLOB descriptor set 178 (plan T-6M (d)): the
/// copy whose N row 1 (@0x2790) and long C value (@0x29c4) are unconfirmed.
pub fn lob2_178(name: &str) -> std::path::PathBuf {
    patched_file("11g-cp1251/ru_lob2_exp_cp1251_11g.dmp", "t11", name, |b| {
        assert_eq!(b[0x35..0x37], [0x00, 0xab]);
        assert_eq!(b[0x24e3..0x24e5], [0xab, 0x00]);
        b[0x36] = 0xb2;
        b[0x24e3] = 0xb2;
    })
}

#[test]
fn e_lob_unconfirmed() {
    let p = lob2_178("lob2_178_codes.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    assert_eq!(d.info().db_charset_id, 178);
    let e = try_read(&mut d, "R_LOB2", &ReadOptions::default()).unwrap_err();
    is(&e, ErrorCode::LobUnconfirmed, Some(0x2790));
    assert_eq!((e.row, e.length), (Some(1), Some(132)));
}

#[test]
fn e_strict() {
    // `t_tstz`: W-TSLTZ for the TSLTZ column at its descriptor; strict turns it into E-STRICT
    let r = walked_rel("18c/t_tstz_exp_18c.dmp");
    let ltz = r.sections[0].parts[0]
        .header
        .columns
        .iter()
        .find(|c| c.type_code == 231)
        .unwrap()
        .offset;
    let o = Options {
        strict: true,
        ..Options::default()
    };
    let e = scan_err(&fixtures().join("18c/t_tstz_exp_18c.dmp"), o);
    is(&e, ErrorCode::Strict, Some(ltz));
    assert_eq!(e.warning, Some(WarningCode::Tsltz));
}

#[test]
fn e_unsupported_charset() {
    let o = Options {
        strict: true,
        ..Options::default()
    };
    // a VARCHAR2 descriptor with the set id 1000: W-CHARSET-UNSUPPORTED, `E-UNSUPPORTED-CHARSET` under strict
    let r = walked_rel("18c/misc_exp_18c.dmp");
    let t = r
        .sections
        .iter()
        .find(|s| s.name_raw == b"T_FRAME")
        .unwrap();
    let c0 = t.parts[0].header.columns[0];
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "set1000.dmp", |b| {
        put_u16(b, c0.offset + 4, 1000)
    });
    let mut d = open(&p, Options::default()).unwrap();
    d.scan().unwrap();
    let w: Vec<&Warning> = d
        .warnings()
        .iter()
        .filter(|w| w.code == WarningCode::CharsetUnsupported)
        .collect();
    assert_eq!(w.len(), 1);
    assert_eq!(w[0].offset, Some(c0.offset));
    let e = scan_err(&p, o);
    is(&e, ErrorCode::UnsupportedCharset, Some(c0.offset));
    assert_eq!(e.warning, Some(WarningCode::CharsetUnsupported));
}

// ---------------------------------------------------------------------------------------------
// warning codes
// ---------------------------------------------------------------------------------------------

fn warning(ws: &[Warning], code: WarningCode) -> &Warning {
    let hits: Vec<&Warning> = ws.iter().filter(|w| w.code == code).collect();
    assert_eq!(hits.len(), 1, "{code}: {ws:?}");
    hits[0]
}

/// The warnings of `open` and the scan.
fn scan_warnings(p: &std::path::Path) -> Vec<Warning> {
    let mut d = open(p, Options::default()).unwrap();
    d.scan().unwrap();
    d.warnings().to_vec()
}

#[test]
fn w_unknown_intro() {
    // the first `ANALSTATS IS` line of `stat_exp_18c.dmp` is misspelled: not in list K, read by G1
    let mut at = 0;
    let p = patched_file("18c/stat_exp_18c.dmp", "t11", "unknown_intro.dmp", |b| {
        at = find_first(b, b"\nANALSTATS IS ") + 1;
        b[at + 8] = b'T';
    });
    let ws = scan_warnings(&p);
    let w = warning(&ws, WarningCode::UnknownIntro);
    assert_eq!(w.offset, Some(at as u64));
    let mut d = open(&p, Options::default()).unwrap();
    assert_eq!(d.scan().unwrap().item_counts.g1_fields, 1);
}

#[test]
fn w_trailing() {
    let mut last = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "trailing.dmp", |b| {
        last = b.len() - 1;
        b[last] = 1;
    });
    let ws = scan_warnings(&p);
    assert_eq!(
        warning(&ws, WarningCode::Trailing).offset,
        Some(last as u64)
    );
}

#[test]
fn w_untested_version() {
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "version.dmp", |b| {
        let at = find_first(b, b"EXPORT:V18.00.00") + 10;
        b[at..at + 6].copy_from_slice(b"10.02.");
    });
    // `open` already knows it (no scan needed)
    let d = open(&p, Options::default()).unwrap();
    assert_eq!(
        warning(d.warnings(), WarningCode::UntestedVersion).offset,
        Some(3)
    );
}

#[test]
fn w_untested_header() {
    // the lines after RECORDLENGTH are `0`, `72`, `0` [H3]: the second is changed
    let mut at = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "header.dmp", |b| {
        at = find_first(b, b"\n0\n72\n0\n") + 3;
        b[at..at + 2].copy_from_slice(b"73");
    });
    let d = open(&p, Options::default()).unwrap();
    assert_eq!(
        warning(d.warnings(), WarningCode::UntestedHeader).offset,
        Some(at as u64)
    );
}

#[test]
fn w_file_size() {
    let size = std::fs::metadata(fixtures().join("18c/t_num_exp_18c.dmp"))
        .unwrap()
        .len();
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "size.dmp", |b| {
        b.extend_from_slice(&[0u8; 7])
    });
    let d = open(&p, Options::default()).unwrap();
    let w = warning(d.warnings(), WarningCode::FileSize);
    assert_eq!((w.offset, w.length), (Some(0), Some(size + 7)));
}

#[test]
fn w_untested_layout() {
    // `lob_exp_18c.dmp`: the first of the two bytes after the entry of the second LOB column is 1 [L1]
    let r = walked_rel("18c/lob_exp_18c.dmp");
    let e1 = &r.sections[0].parts[0].header.lob_entries[1];
    let tail_at = e1.offset + 1 + e1.name.len() as u64;
    let p = patched_file("18c/lob_exp_18c.dmp", "t11", "layout.dmp", |b| {
        b[tail_at as usize] = 1
    });
    let ws = scan_warnings(&p);
    let w = warning(&ws, WarningCode::UntestedLayout);
    assert_eq!((w.offset, w.table), (Some(tail_at), Some(1)));
}

#[test]
fn w_ddl_fallback() {
    // T_NUM: the type word of column V is misspelled; the definition is not one of the forms of [T9]
    let mut at = 0;
    let p = patched_file("18c/t_num_exp_18c.dmp", "t11", "ddl.dmp", |b| {
        at = find_first(b, b"\"V\" NUMBER)") + 5;
        b[at + 4] = b'F';
        at -= 5;
    });
    let ws = scan_warnings(&p);
    let w = warning(&ws, WarningCode::DdlFallback);
    // the offset of the definition in the file: its opening quote; the column is V, file position 2
    assert_eq!(
        (w.offset, w.table, w.column),
        (Some(at as u64), Some(1), Some(2))
    );
}

#[test]
fn w_charset_unsupported() {
    let r = walked_rel("18c/misc_exp_18c.dmp");
    let t = r
        .sections
        .iter()
        .find(|s| s.name_raw == b"T_FRAME")
        .unwrap();
    let c0 = t.parts[0].header.columns[0];
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "set1000b.dmp", |b| {
        put_u16(b, c0.offset + 4, 1000)
    });
    let ws = scan_warnings(&p);
    let w = warning(&ws, WarningCode::CharsetUnsupported);
    assert_eq!(
        (w.offset, w.column, w.length),
        (Some(c0.offset), Some(1), Some(1000))
    );
}

#[test]
fn w_unverified_type() {
    // T_ROWID row 1: the last prefix byte `17` becomes `18` (architecture 5.7)
    let mut d0 = open_fixture("18c/misc_exp_18c.dmp");
    let first = row_starts(&mut d0, "T_ROWID")[0];
    let data = std::fs::read(d0.path()).unwrap();
    let at = field_offsets(&data, first, 3)[1].unwrap();
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "rowid_prefix.dmp", |b| {
        b[at as usize + 4] = 0x18
    });
    let mut d = open(&p, Options::default()).unwrap();
    let (_, ws) = read(&mut d, "T_ROWID", &ReadOptions::default());
    let w = warning(&ws, WarningCode::UnverifiedType);
    assert_eq!((w.offset, w.row, w.column), (Some(at), Some(1), Some(2)));
}

#[test]
fn w_long_length() {
    // T-19 (ADR-95 item 9): the threshold of a LONG is 100,000 bytes. T_LONG3 row 7 of `long3_exp_18c.dmp` (a
    // conventional value of 100,000 bytes in the chunked form) gets one more chunk: 100,001 bytes. The warning
    // stands at the value's first data byte (behind the first chunk's u16); the unpatched file gives none.
    let (p, at) = long3_100001_copy("t11", "long100001.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    let (_, ws) = read(&mut d, "T_LONG3", &ReadOptions::default());
    let w = warning(&ws, WarningCode::LongLength);
    assert_eq!((w.offset, w.row, w.column), (Some(at), Some(7), Some(2)));
    assert_eq!(
        w.message,
        "LONG value above 100,000 bytes, the largest confirmed"
    );
    let mut d = open_fixture("18c/long3_exp_18c.dmp");
    let (_, ws) = read(&mut d, "T_LONG3", &ReadOptions::default());
    assert!(ws.is_empty(), "{ws:?}");
}

#[test]
fn w_long_charset() {
    // `long2_exp_18c.dmp` with the database set of the header changed to 171: the LONG value of row 1 has bytes
    // above 0x7f in a database whose set is not AL32UTF8 (ADR-15 item 4)
    let mut d0 = open_fixture("18c/long2_exp_18c.dmp");
    let first = row_starts(&mut d0, "T_LONG2")[0];
    let data = std::fs::read(d0.path()).unwrap();
    let ncols = d0.table(TableRef::ByName("T_LONG2")).unwrap().columns.len();
    let at = field_offsets(&data, first, ncols)[1].unwrap();
    let p = patched_file("18c/long2_exp_18c.dmp", "t11", "long2_171.dmp", |b| {
        let h = find_first(&b[..0x200], &[0x03, 0x69, 0x03, 0x69, 0x07, 0xd0]);
        b[h + 2] = 0x00;
        b[h + 3] = 0xab;
    });
    let mut d = open(&p, Options::default()).unwrap();
    let (_, ws) = read(&mut d, "T_LONG2", &ReadOptions::default());
    let w = warning(&ws, WarningCode::LongCharset);
    assert_eq!((w.offset, w.row, w.column), (Some(at), Some(1), Some(2)));
}

#[test]
fn w_longer_than_declared() {
    // T_FRAME of `misc_exp_18c.dmp`: the descriptor length of column A (10) becomes 3 (ADR-26)
    let desc = [
        0x03, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69, 0x03, 0x01, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69,
        0x03, 0x01, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69, 0x03, 0x01, 0x00,
    ];
    let mut d0 = open_fixture("18c/misc_exp_18c.dmp");
    let first = row_starts(&mut d0, "T_FRAME")[0];
    let data = std::fs::read(d0.path()).unwrap();
    let at = field_offsets(&data, first, 3)[0].unwrap();
    let p = patched_file("18c/misc_exp_18c.dmp", "t11", "short_desc.dmp", |b| {
        let d = find_one(b, &desc);
        b[d + 4] = 3;
    });
    let mut d = open(&p, Options::default()).unwrap();
    let (_, ws) = read(&mut d, "T_FRAME", &ReadOptions::default());
    let w = warning(&ws, WarningCode::LongerThanDeclared);
    assert_eq!(
        (w.offset, w.row, w.column, w.count),
        (Some(at), Some(1), Some(1), Some(4))
    );
}

/// The offset of the value of `row` (1-based), `column` (1-based) of a conventional-path table.
fn value_offset(d: &mut Dump, table: &str, row: usize, column: usize) -> u64 {
    let data = std::fs::read(d.path()).unwrap();
    let ncols = d.table(TableRef::ByName(table)).unwrap().columns.len();
    let starts = row_starts(d, table);
    field_offsets(&data, starts[row - 1], ncols)[column - 1].unwrap()
}

fn lenient(o: OverflowPolicy) -> ReadOptions {
    ReadOptions {
        on_value_error: OnValueError::Null,
        on_number_overflow: o,
        ..ReadOptions::default()
    }
}

#[test]
fn w_number_rounded() {
    // truth_num.txt: rows 25, 27, 30, 31 have more than 10 decimals (the integer-part rows 19 to 24, 26, 28, 29,
    // 32 become NULL under `on_value_error=null`); the first is row 25, column V
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let at = value_offset(&mut d, "T_NUM", 25, 2);
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Round));
    let w = warning(&ws, WarningCode::NumberRounded);
    assert_eq!(
        (w.offset, w.row, w.column, w.count, w.count_to_zero),
        (Some(at), Some(25), Some(2), Some(4), Some(1))
    );
}

#[test]
fn w_number_nulled() {
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let at = value_offset(&mut d, "T_NUM", 25, 2);
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Null));
    let w = warning(&ws, WarningCode::NumberNulled);
    assert_eq!((w.offset, w.row, w.column), (Some(at), Some(25), Some(2)));
}

#[test]
fn w_value_nulled() {
    // truth_date.txt: rows 8 and 9 have a year below 1
    let mut d = open_fixture("18c/t_date_exp_18c.dmp");
    let at = value_offset(&mut d, "T_DATE", 8, 2);
    let (_, ws) = read(&mut d, "T_DATE", &lenient(OverflowPolicy::Round));
    let w = warning(&ws, WarningCode::ValueNulled);
    assert_eq!(
        (w.offset, w.row, w.column, w.count),
        (Some(at), Some(8), Some(2), Some(2))
    );
}

#[test]
fn w_charset_mismatch() {
    // `long2_exp_18c.dmp` with the database set 171 (as in `w_long_charset`): the VARCHAR2 column C has the
    // descriptor set 873: the warning stands at the CREATE TABLE line of the table
    let create = walked_rel("18c/long2_exp_18c.dmp").sections[0].create_offset;
    let p = patched_file("18c/long2_exp_18c.dmp", "t11", "long2_171b.dmp", |b| {
        let h = find_first(&b[..0x200], &[0x03, 0x69, 0x03, 0x69, 0x07, 0xd0]);
        b[h + 2] = 0x00;
        b[h + 3] = 0xab;
    });
    let mut d = open(&p, Options::default()).unwrap();
    let (_, ws) = read(&mut d, "T_LONG2", &ReadOptions::default());
    let w = warning(&ws, WarningCode::CharsetMismatch);
    assert_eq!((w.offset, w.column), (Some(create), Some(3)));
}

#[test]
fn w_lob_untested() {
    // the 178 copy with `unconfirmed_lob="decode"`: N row 1 @0x2790 is the first refused value of column N
    let p = lob2_178("lob2_178_w.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    let o = ReadOptions {
        unconfirmed_lob: UnconfirmedLob::Decode,
        ..ReadOptions::default()
    };
    let (_, ws) = read(&mut d, "R_LOB2", &o);
    let n: Vec<&Warning> = ws
        .iter()
        .filter(|w| w.code == WarningCode::LobUntested)
        .collect();
    let by_column: Vec<(Option<u32>, Option<u64>, Option<u64>)> =
        n.iter().map(|w| (w.column, w.count, w.offset)).collect();
    // C (column 2): the long value @0x29c4; N (column 3): @0x2790 and one more (plan T-6M (d): counts 1 and 2)
    assert!(
        by_column.contains(&(Some(2), Some(1), Some(0x29c4))),
        "{by_column:?}"
    );
    assert!(
        by_column
            .iter()
            .any(|&(c, n, o)| (c, n) == (Some(3), Some(2)) && o == Some(0x2790)),
        "{by_column:?}"
    );
}

#[test]
fn w_tsltz() {
    let r = walked_rel("18c/t_tstz_exp_18c.dmp");
    let ltz = r.sections[0].parts[0]
        .header
        .columns
        .iter()
        .position(|c| c.type_code == 231)
        .unwrap();
    let off = r.sections[0].parts[0].header.columns[ltz].offset;
    let ws = scan_warnings(&fixtures().join("18c/t_tstz_exp_18c.dmp"));
    let w = warning(&ws, WarningCode::Tsltz);
    assert_eq!(
        (w.offset, w.table, w.column),
        (Some(off), Some(1), Some(u32::try_from(ltz + 1).unwrap()))
    );
}

#[test]
fn w_possibly_incomplete() {
    // `ru_exp_us7ascii_11g.dmp`: the table whose export failed has no rows and the name `???_???` (X2)
    let rel = "11g-cp1251/ru_exp_us7ascii_11g.dmp";
    let r = walked_rel(rel);
    let mut d = open_fixture(rel);
    let t = d
        .tables()
        .unwrap()
        .iter()
        .find(|t| t.possibly_incomplete)
        .unwrap()
        .clone();
    let create = r
        .sections
        .iter()
        .find(|s| s.ordinal == t.ordinal)
        .unwrap()
        .create_offset;
    let w = warning(d.warnings(), WarningCode::PossiblyIncomplete);
    assert_eq!((w.offset, w.table), (Some(create), Some(t.ordinal)));
}

// ---------------------------------------------------------------------------------------------
// every code has its test; the retired codes are gone
// ---------------------------------------------------------------------------------------------

#[test]
fn every_code_has_a_test_in_this_file_and_the_retired_codes_are_absent() {
    let src = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/t11_codes.rs"),
    )
    .unwrap();
    for c in ErrorCode::ALL {
        let f = format!("fn {}(", c.as_str().to_lowercase().replace('-', "_"));
        assert!(src.contains(&f), "{c}: no test {f}");
    }
    for c in WarningCode::ALL {
        let f = format!("fn {}(", c.as_str().to_lowercase().replace('-', "_"));
        assert!(src.contains(&f), "{c}: no test {f}");
    }
    // retired in sessions 2 to 4 (architecture 7.1): `W-ZERO-LENGTH` (now Framing), `W-LOB-CHARSET`,
    // `W-LONG-DIRECT`
    let all: Vec<&str> = ErrorCode::ALL
        .iter()
        .map(|c| c.as_str())
        .chain(WarningCode::ALL.iter().map(|c| c.as_str()))
        .collect();
    for retired in ["W-ZERO-LENGTH", "W-LOB-CHARSET", "W-LONG-DIRECT"] {
        assert!(!all.contains(&retired), "{retired}");
    }
    assert_eq!(ErrorCode::ALL.len() + WarningCode::ALL.len(), 44);
}
