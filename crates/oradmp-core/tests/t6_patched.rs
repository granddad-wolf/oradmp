//! Task T-6: the checks that need a patched copy of a fixture (the copies are written below `target/`, the
//! fixtures are never touched). Each test says which bytes it changes and why the expectation holds.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::path::PathBuf;

use common::*;
use oradmp_core::dump::Options;
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::{ErrorCode, WarningCode};

/// Copies a fixture to `target/t6-tests/<dir>/<name>` after `patch` changed the bytes.
fn patched(rel: &str, name: &str, patch: impl FnOnce(&mut Vec<u8>)) -> PathBuf {
    let mut data = std::fs::read(fixtures().join(rel)).unwrap();
    patch(&mut data);
    let p = target_dir("patched").join(name);
    std::fs::write(&p, data).unwrap();
    p
}

/// The first position of `pattern` in `data` (T_LONG stands before T_LONGRAW, whose value has the same piece
/// lengths).
fn find_first(data: &[u8], pattern: &[u8]) -> usize {
    data.windows(pattern.len())
        .position(|w| w == pattern)
        .unwrap()
}

/// The only position of `pattern` in `data`.
fn find_one(data: &[u8], pattern: &[u8]) -> usize {
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

/// Session 4 (ADR-15 item 4): `W-LONG-CHARSET` only in a database whose set is not AL32UTF8. The database set
/// of the header (client id, database id, national id: `03 69 03 69 07 d0`) is changed from 873 to 171
/// (`00 ab`): the LONG bytes `d0 b4 d0 b6 20 43 61 66 c3 a9` (truth_long2.txt line 1) have bytes above 0x7f, are
/// decoded as CL8MSWIN1251 and give one warning; the VARCHAR2 descriptor (873) now differs from the header set
/// (`W-CHARSET-MISMATCH`, the descriptor wins). The unpatched file gives neither.
#[test]
fn long_in_a_single_byte_database_decodes_with_w_long_charset() {
    let p = patched("18c/long2_exp_18c.dmp", "long2_db1251.dmp", |d| {
        let at = find_one(&d[..0x200], &[0x03, 0x69, 0x03, 0x69, 0x07, 0xd0]);
        d[at + 2] = 0x00;
        d[at + 3] = 0xab;
    });
    let mut d = open_path(&p, Options::default());
    assert_eq!(d.info().db_charset_id, 171);
    let (b, w) = read(&mut d, "T_LONG2", &ReadOptions::default());
    // the expectation comes from Oracle's own map of CL8MSWIN1251 (`truth_checks`, the file
    // `truth_charsets.txt`), not from the library the reader uses (review N-4); the bytes are those of
    // truth_long2.txt line 1
    let want =
        truth_checks::decode_cp1251(&[0xd0, 0xb4, 0xd0, 0xb6, 0x20, 0x43, 0x61, 0x66, 0xc3, 0xa9]);
    assert_eq!(rows(&b)[0][1], want);
    let l: Vec<_> = w
        .iter()
        .filter(|w| w.code == WarningCode::LongCharset)
        .collect();
    assert_eq!(l.len(), 1, "{w:?}");
    assert_eq!(
        (l[0].row, l[0].column, l[0].part, l[0].table),
        (Some(1), Some(2), Some(1), Some(1))
    );
    assert!(l[0].offset.is_some());
    assert!(
        w.iter()
            .any(|w| w.code == WarningCode::CharsetMismatch && w.column == Some(3)),
        "{w:?}"
    );
    let (_, w) = read_default("18c/long2_exp_18c.dmp", "T_LONG2");
    assert!(w.is_empty());
}

/// The chain of T_LONG row 3 in `long_exp_direct_18c.dmp` is 5 pieces (156 + 4 x 7,961 bytes: `9c 80`, then
/// `19 9f`). Bytes 155 and 156 of the value are changed to `d0 b4` (the UTF-8 of U+0434), so the character
/// is split between piece 1 and piece 2: it decodes as one character at the same place.
#[test]
fn a_utf8_sequence_split_between_two_pieces_decodes() {
    let p = patched("18c/long_exp_direct_18c.dmp", "chain_split.dmp", |d| {
        let start = find_first(d, &[0x9c, 0x80]);
        assert_eq!(&d[start + 2 + 156..start + 2 + 158], &[0x19, 0x9f]);
        d[start + 2 + 155] = 0xd0;
        d[start + 2 + 156 + 2] = 0xb4;
    });
    let mut d = open_path(&p, Options::default());
    let (b, w) = read(&mut d, "T_LONG", &ReadOptions::default());
    assert!(w.is_empty(), "{w:?}");
    let want = format!("L{}\u{434}{}", "x".repeat(154), "x".repeat(32_000 - 157));
    assert_eq!(rows(&b)[2][1], want);
    assert_eq!(rows(&b)[2][2], "c");
}

/// The same bytes with only `d0` (piece 1 ends inside a sequence, piece 2 goes on with ASCII) are not UTF-8:
/// `E-BAD-TEXT` at row 3, column 3 (file position), or NULL with a counted `W-VALUE-NULLED`; the rest of the
/// chain is passed over and the next table reads.
#[test]
fn a_broken_utf8_sequence_in_a_chain_is_a_value_error() {
    let p = chain_cut_copy("patched", "chain_broken.dmp");
    let mut d = open_path(&p, Options::default());
    let e = try_read(&mut d, "T_LONG", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadText);
    assert_eq!(
        (e.row, e.column, e.part, e.table),
        (Some(3), Some(3), Some(1), Some(1))
    );
    // ADR-43 item 3: the start of the value (the first data byte of piece 1, `9c 80` is at 0x2288), not the
    // piece where the decoder noticed; the message names the piece
    assert_eq!(e.offset, Some(0x228a));
    // the exact suffix: `contains("piece 2")` would pass "piece 20" (review 04 S-5)
    assert!(
        e.message.ends_with(" (piece 2 of the value)"),
        "{}",
        e.message
    );
    let (b, w) = read(
        &mut d,
        "T_LONG",
        &ReadOptions {
            on_value_error: OnValueError::Null,
            ..ReadOptions::default()
        },
    );
    assert_eq!(col_of(&b, 1), ["short", "<NULL>", "<NULL>"]);
    assert_eq!(col_of(&b, 2), ["a", "b", "c"]);
    assert_eq!(
        w.iter()
            .filter(|w| w.code == WarningCode::ValueNulled && w.count == Some(1))
            .count(),
        1
    );
    assert_eq!(
        row_count(&read(&mut d, "T_FRAME", &ReadOptions::default()).0),
        7
    );
}

fn col_of(b: &[arrow_array::RecordBatch], c: usize) -> Vec<String> {
    rows(b).into_iter().map(|r| r[c].clone()).collect()
}

/// ROWID prefix flag (architecture 5.7 item 3, ADR-27 item 2): the 5 bytes before the 18 characters of the
/// 23-byte field are `01 00 00 00 17` in every fixture; with `01 00 00 00 18` the text is still delivered and
/// one `W-UNVERIFIED-TYPE` is raised for the column (row 1, file position 2).
#[test]
fn a_rowid_with_another_prefix_gives_one_unverified_type_warning() {
    let p = patched("18c/misc_exp_18c.dmp", "rowid_prefix.dmp", |d| {
        let at = find_one(d, &[0x17, 0x00, 0x01, 0x00, 0x00, 0x00, 0x17]);
        d[at + 6] = 0x18;
    });
    let mut d = open_path(&p, Options::default());
    let (b, w) = read(&mut d, "T_ROWID", &ReadOptions::default());
    assert_eq!(rows(&b)[0][1], "AAAR8lAAMAAAACHAAA");
    let u: Vec<_> = w
        .iter()
        .filter(|w| w.code == WarningCode::UnverifiedType)
        .collect();
    assert_eq!(u.len(), 1, "{w:?}");
    assert_eq!(
        (u[0].row, u[0].column, u[0].part, u[0].count),
        (Some(1), Some(2), Some(1), Some(1))
    );
    let (_, w) = read_default("18c/misc_exp_18c.dmp", "T_ROWID");
    assert!(w.is_empty());
}

/// `W-LONGER-THAN-DECLARED` (ADR-26): the descriptor length of T_FRAME.A (`01 00 0a 00 69 03 01 00`, 10) is
/// changed to 3 in `misc_exp_18c.dmp`; the values `AAAA`, `DDDD`, `GGGG` and `NNNNNNNNNN` (rows 1, 2, 3, 7) are
/// longer: one warning with count 4, first at row 1, column 1; the values are delivered as read
/// (`truth_frame.txt`).
#[test]
fn a_field_longer_than_its_descriptor_is_delivered_with_a_warning() {
    let desc = [
        0x03, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69, 0x03, 0x01, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69,
        0x03, 0x01, 0x00, 0x01, 0x00, 0x0a, 0x00, 0x69, 0x03, 0x01, 0x00,
    ];
    let p = patched("18c/misc_exp_18c.dmp", "frame_short_desc.dmp", |d| {
        let at = find_one(d, &desc);
        d[at + 4] = 0x03;
    });
    let mut d = open_path(&p, Options::default());
    let (b, w) = read(&mut d, "T_FRAME", &ReadOptions::default());
    let got: Vec<String> = rows(&b).iter().map(|r| r.join("|")).collect();
    let want: Vec<String> = truth("18c", "truth_frame.txt")
        .iter()
        .map(|l| l.join("|"))
        .collect();
    assert_eq!(got, want);
    let l: Vec<_> = w
        .iter()
        .filter(|w| w.code == WarningCode::LongerThanDeclared)
        .collect();
    assert_eq!(l.len(), 1, "{w:?}");
    assert_eq!(
        (l[0].count, l[0].row, l[0].column),
        (Some(4), Some(1), Some(1))
    );
}

/// Hostile input: one changed byte at many offsets of three fixtures (a partitioned, a LONG and a stat file)
/// never panics the open, the scan or the read; the result is data or a typed error.
#[test]
fn damaged_files_never_panic_the_reader() {
    for rel in [
        "18c/stat_exp_18c.dmp",
        "18c/long_exp_direct_18c.dmp",
        "18c/part2_exp_18c.dmp",
    ] {
        let len = std::fs::metadata(fixtures().join(rel)).unwrap().len() as usize;
        let step = (len / 150).max(1);
        for at in (0..len).step_by(step) {
            for value in [0x00u8, 0xff, 0x80] {
                let p = patched(rel, "damaged.dmp", |d| d[at] = value);
                let Ok(mut d) = oradmp_core::dump::open(&p, Options::default()) else {
                    continue;
                };
                let Ok(tables) = d.tables().map(<[_]>::to_vec) else {
                    continue;
                };
                for t in tables {
                    for o in [
                        ReadOptions::default(),
                        ReadOptions {
                            on_value_error: OnValueError::Null,
                            ..ReadOptions::default()
                        },
                    ] {
                        let _ = try_read(&mut d, &t.name, &o);
                    }
                }
            }
        }
    }
}
