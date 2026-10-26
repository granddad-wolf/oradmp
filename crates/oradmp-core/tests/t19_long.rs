//! Task T-19b (ADR-99 to ADR-103) is at the end of this file: the 18 files of Analyst sessions 19 and 20.
//!
//! Task T-19 (ADR-94, ADR-95, ADR-97): the conventional LONG forms of R-040 and the ten files of Analyst session 17
//! (`long3_exp(_direct)_<v>`, `tsp_exp(_direct)_<v>`, `seg_exp(_direct)_18c`, five truth files).
//!
//! Expected numbers come from the Analyst's tools, run read-only in T-19: `long3_check.py` (the offsets of the
//! seven T_LONG3 values and their forms in the four `long3` files: 0x2271, 0xa280, 0x12290, 0x2228f, 0x3228f,
//! 0x422c2, 0x522f6 conventional; direct 0x2279, 0xa28e, 0x122a4, 0x222b1, 0x322bf, 0x422dc, 0x522fb), `tsp_check.py`
//! (T_TSP: descriptors 180 / 11, four rows), `rowlen_stats.py` (`long_conv_8000_up` 6, `long_conv_chunked` 6,
//! the largest LONG 100,000) and the truth files `truth_long3.txt`, `truth_tsp.txt`, `truth_seg.txt`. The value
//! of every L is the arithmetic of R-040 (`L` + the size as six digits + `y` x (size - 7)), built in
//! `common/truth_checks.rs` from the truth length, never from the reader.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use arrow_array::cast::AsArray;
use common::*;
use oradmp_core::dump::{open, CatalogEnd, Dump, Options, TableRef};
use oradmp_core::exp::items::{walk_path, WalkOptions};
use oradmp_core::exp::rows::EndMarker;
use oradmp_core::exp::rows::RowSink;
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::{ErrorCode, WarningCode};

const VERSIONS: [&str; 2] = ["18c", "11g"];

/// (name stem, direct)
const LONG3_FILES: [(&str, bool); 2] = [("long3_exp", false), ("long3_exp_direct", true)];

fn long3_rel(kind: &str, v: &str) -> String {
    format!("{v}/{kind}_{v}.dmp")
}

// ---------------------------------------------------------------------------------------------------------
// (b) the four long3 files
// ---------------------------------------------------------------------------------------------------------

/// T_LONG3 of the four files: the length, the first and the last 20 bytes of L and the text of V from
/// `truth_long3.txt`, the whole L from R-040's arithmetic, no warning (the family check of `truth_checks`).
#[test]
fn t_long3_equals_the_truth_in_all_four_files() {
    let d = truth_checks::long3_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// The pure check fails on a wrong truth line (the break test of the family): a length, a first 20 bytes and a V.
#[test]
fn the_long3_check_notices_a_changed_truth() {
    let (_, rows, _) = truth_checks::read_named(
        &long3_rel("long3_exp", "18c"),
        "T_LONG3",
        &ReadOptions::default(),
    )
    .unwrap();
    let t = truth("18c", "truth_long3.txt");
    assert!(truth_checks::long3_diff("x", &t, &rows).is_empty());
    for (field, from, to) in [(2usize, "4C31", "4C32"), (4, "v", "w")] {
        let mut m = t.clone();
        assert!(m[6][field].contains(from));
        m[6][field] = m[6][field].replacen(from, to, 1);
        assert_eq!(truth_checks::long3_diff("x", &m, &rows).len(), 1, "{field}");
    }
    let mut m = t.clone();
    m[6][1] = "100001".to_string();
    assert!(!truth_checks::long3_diff("x", &m, &rows).is_empty());
}

/// T_FRAME behind T_LONG3 equals the T_FRAME truth the existing checker uses (`truth_frame.txt`), in all four files.
#[test]
fn t_frame_of_the_long3_files_equals_its_truth() {
    for v in VERSIONS {
        let want: Vec<Vec<String>> = truth(v, "truth_frame.txt")
            .into_iter()
            .map(|l| {
                l.into_iter()
                    .map(|x| if x == "NULL" { "<NULL>".to_string() } else { x })
                    .collect()
            })
            .collect();
        assert_eq!(want.len(), 7);
        for (kind, _) in LONG3_FILES {
            let (b, w) = read_default(&long3_rel(kind, v), "T_FRAME");
            assert!(w.is_empty(), "{v} {kind}");
            assert_eq!(rows(&b), want, "{v} {kind}");
        }
    }
}

/// The LongStats of column L: conventional `values 7, chained 0, chunked 3, largest 100000, above_32000 7`; direct
/// `values 7, chained 7, chunked 0, largest 100000, above_32000 7` (`rowlen_stats.py`: three `fd ff` values, rows 5 to
/// 7, per conventional file; the direct chains are all of more than one piece). No warning at all with the default
/// options and with `strict`, at open, scan and read.
#[test]
fn long_stats_and_no_warning_in_the_long3_files() {
    for v in VERSIONS {
        for (kind, direct) in LONG3_FILES {
            for strict in [false, true] {
                let rel = long3_rel(kind, v);
                let mut d = open(
                    fixtures().join(&rel),
                    Options {
                        strict,
                        ..Options::default()
                    },
                )
                .unwrap();
                let t = d.table(TableRef::ByName("T_LONG3")).unwrap().clone();
                let l = t
                    .columns
                    .iter()
                    .find(|c| c.name == "L")
                    .and_then(|c| c.long.clone())
                    .unwrap();
                let want = if direct { (7, 7, 0) } else { (7, 0, 3) };
                assert_eq!(
                    (l.values, l.chained, l.chunked),
                    want,
                    "{rel} strict={strict}"
                );
                assert_eq!((l.largest, l.above_32000), (100_000, 7), "{rel}");
                assert_eq!((l.zero_pieces, l.chunked_short), (0, 0), "{rel}");
                assert!(d.warnings().is_empty(), "{rel}: {:?}", d.warnings());
                let o = ReadOptions::default();
                for name in ["T_LONG3", "T_FRAME"] {
                    let (_, w) = try_read(&mut d, name, &o).unwrap();
                    assert!(w.is_empty(), "{rel} {name} strict={strict}: {w:?}");
                }
                assert!(d.warnings().is_empty(), "{rel}");
            }
        }
    }
}

/// The assembled digest of T_LONG3's L (`long_sha256`: the total as u64 and the bytes, pieces and chunks joined) is
/// the same in all four files, and differs from the digest of a column whose value differs.
#[test]
fn the_assembled_long_digest_is_the_same_in_all_four_files() {
    let mut digests = Vec::new();
    for v in VERSIONS {
        for (kind, _) in LONG3_FILES {
            let rel = long3_rel(kind, v);
            let r = walk_path(
                &fixtures().join(&rel),
                &WalkOptions {
                    hash_fields: true,
                    ..WalkOptions::default()
                },
            )
            .unwrap();
            let part = &r.sections[0].parts[0];
            assert_eq!(part.rows.long_sha256.len(), 1, "{rel}");
            digests.push((rel, part.rows.long_sha256[0], part.rows.raw_sha256.unwrap()));
        }
    }
    assert!(digests.iter().all(|d| d.1 == digests[0].1), "{digests:?}");
    // the raw digests differ: the forms differ (a plain field, `fd ff` chunks, a chain)
    assert_ne!(digests[0].2, digests[1].2);
    // the digest is the one of the assembled values: SHA-256 over, per row, the total (u64 LE) and L's bytes
    use sha2::{Digest, Sha256};
    let t = truth("18c", "truth_long3.txt");
    let mut h = Sha256::new();
    for l in &t {
        let size: usize = l[1].parse().unwrap();
        h.update((size as u64).to_le_bytes());
        h.update(format!("L{size:06}{}", "y".repeat(size - 7)).as_bytes());
    }
    let want: [u8; 32] = h.finalize().into();
    assert_eq!(digests[0].1, want);
}

/// The scan of the four files walks to the second EXIT with the ends the Analyst's tool reads (7 rows, then T_FRAME
/// with 7 rows), and the columns are L as LargeUtf8.
#[test]
fn the_long3_files_scan_to_the_end() {
    for v in VERSIONS {
        for (kind, _) in LONG3_FILES {
            let mut d = open_fixture(&long3_rel(kind, v));
            let c = d.scan().unwrap();
            assert!(matches!(c.end, CatalogEnd::Complete));
            let names: Vec<(&str, u64)> = c
                .tables
                .iter()
                .map(|t| (t.name.as_str(), t.num_rows))
                .collect();
            assert_eq!(names, [("T_LONG3", 7), ("T_FRAME", 7)]);
            let (b, _) = read_default(&long3_rel(kind, v), "T_LONG3");
            assert_eq!(
                b[0].column(1).data_type(),
                &arrow_schema::DataType::LargeUtf8
            );
            assert_eq!(b[0].column(1).as_string::<i64>().value(0).len(), 32_767);
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// (c) the stops and the framing errors of the conventional LONG / LONG RAW forms on patched copies (T-19b)
// ---------------------------------------------------------------------------------------------------------

fn put_u16(b: &mut [u8], at: usize, v: u16) {
    b[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

const STOP_MSG: &str =
    "conventional LONG or LONG RAW field starting 0xfffb or 0xfffc, not in the fixtures [R8]";
const N_MSG: &str = "conventional LONG or LONG RAW 4-byte length outside 65,530 to 65,533";
const Z_MSG: &str =
    "conventional LONG or LONG RAW 4-byte length whose upper two bytes are not 00 00";

/// `long4_exp_<v>.dmp` with the u16 at 0x22277 (`fa ff`, row 3: 65,530 bytes in form (b)) set to `fb ff` and to `fc ff`
/// (`long4_check.py`: the offset of the value): the scan ends `stopped` with `E-UNSUPPORTED-LAYOUT` at 0x22277, table
/// 1 (T_LONG4), row 3, column 2, `length` 0xfffb / 0xfffc and the exact message; no table is listed after it.
#[test]
fn a_field_starting_fb_ff_or_fc_ff_stops_the_file_by_name() {
    for v in VERSIONS {
        for (u, name) in [(0xfffbu16, "fb"), (0xfffc, "fc")] {
            let rel = format!("{v}/long4_exp_{v}.dmp");
            let p = patched_file(&rel, "t19b", &format!("long4_{v}_{name}.dmp"), |b| {
                assert_eq!(b[0x22277..0x22279], [0xfa, 0xff]);
                put_u16(b, 0x22277, u);
            });
            let mut d = open(&p, Options::default()).unwrap();
            let c = d.scan().unwrap();
            let CatalogEnd::Stopped(e) = &c.end else {
                panic!("{v} {name}: not stopped");
            };
            assert_eq!(
                (e.code, e.offset, e.table, e.row, e.column, e.length),
                (
                    ErrorCode::UnsupportedLayout,
                    Some(0x22277),
                    Some(1),
                    Some(3),
                    Some(2),
                    Some(u64::from(u))
                ),
                "{v} {name}"
            );
            assert_eq!(e.message, STOP_MSG);
            assert!(c.tables.is_empty(), "no table is listed after the stop");
            assert_eq!(
                c.stopped_table.as_ref().unwrap().name_raw,
                b"T_LONG4".to_vec()
            );
        }
    }
}

/// `long3_exp_<v>.dmp` with the u16 at 0x2228f (`f0 ff`, row 4: a plain field of 65,520 bytes) set to `fc ff` stops
/// with the same text (the stop does not depend on the file or the row).
#[test]
fn the_stop_is_the_same_in_the_long3_files() {
    for v in VERSIONS {
        let rel = long3_rel("long3_exp", v);
        let p = patched_file(&rel, "t19b", &format!("long3_{v}_fc.dmp"), |b| {
            assert_eq!(b[0x2228f..0x22291], [0xf0, 0xff]);
            put_u16(b, 0x2228f, 0xfffc);
        });
        let mut d = open(&p, Options::default()).unwrap();
        let c = d.scan().unwrap();
        let CatalogEnd::Stopped(e) = &c.end else {
            panic!("{v}: not stopped");
        };
        assert_eq!(
            (e.code, e.offset, e.table, e.row, e.column, e.length),
            (
                ErrorCode::UnsupportedLayout,
                Some(0x2228f),
                Some(1),
                Some(4),
                Some(2),
                Some(0xfffc)
            ),
            "{v}"
        );
        assert_eq!(e.message, STOP_MSG);
    }
}

/// `long_exp_<v>.dmp` with the u16 at 0xa0ea (`00 7d`, T_LONGRAW row 2) set to `fb ff`: stopped at 0xa0ea, table 2,
/// row 2, column 2; T_LONG (table 1) is listed and reads its 3 rows. (The LONG RAW lengths `00 80`, `f0 ff` and `fd ff`
/// that T-19 stopped on are readable forms now: see the longraw files.)
#[test]
fn a_long_raw_field_starting_fb_ff_stops_and_the_table_before_it_reads() {
    for v in VERSIONS {
        let rel = format!("{v}/long_exp_{v}.dmp");
        let p = patched_file(&rel, "t19b", &format!("long_{v}_fbff.dmp"), |b| {
            assert_eq!(b[0xa0ea..0xa0ec], [0x00, 0x7d]);
            put_u16(b, 0xa0ea, 0xfffb);
        });
        let mut d = open(&p, Options::default()).unwrap();
        let c = d.scan().unwrap();
        let CatalogEnd::Stopped(e) = &c.end else {
            panic!("{v}: not stopped");
        };
        assert_eq!(
            (e.code, e.offset, e.table, e.row, e.column, e.length),
            (
                ErrorCode::UnsupportedLayout,
                Some(0xa0ea),
                Some(2),
                Some(2),
                Some(2),
                Some(0xfffb)
            ),
            "{v}"
        );
        assert_eq!(e.message, STOP_MSG);
        assert_eq!(c.tables.len(), 1);
        assert_eq!(c.stopped_table.as_ref().unwrap().ordinal, 2);
        assert_eq!(c.tables[0].name, "T_LONG");
        let (b, _) = try_read(&mut d, "T_LONG", &ReadOptions::default()).unwrap();
        assert_eq!(row_count(&b), 3, "{v}");
    }
}

/// `long4_exp_<v>.dmp` row 3 (`fa ff`, n at 0x22279, z at 0x2227b): n set to `f9 ff` and to `fe ff` is `E-FRAMING` at
/// 0x22279 with `length` 0xfff9 / 0xfffe; z set to `01 00` is `E-FRAMING` at 0x2227b with `length` 1. Framing is a
/// hard error of the scan (the byte count of the value is unknown, ADR-12).
#[test]
fn the_four_byte_length_is_checked_at_n_and_at_z() {
    for v in VERSIONS {
        for (at, bytes, length, msg) in [
            (0x22279usize, [0xf9u8, 0xff], 0xfff9u64, N_MSG),
            (0x22279, [0xfe, 0xff], 0xfffe, N_MSG),
            (0x2227b, [0x01, 0x00], 1, Z_MSG),
        ] {
            let rel = format!("{v}/long4_exp_{v}.dmp");
            let p = patched_file(
                &rel,
                "t19b",
                &format!("long4_{v}_{at:x}_{length:x}.dmp"),
                |b| {
                    assert_eq!(b[0x22277..0x2227d], [0xfa, 0xff, 0xfa, 0xff, 0x00, 0x00]);
                    b[at..at + 2].copy_from_slice(&bytes);
                },
            );
            let mut d = open(&p, Options::default()).unwrap();
            let e = d.scan().map(|_| ()).unwrap_err();
            assert_eq!(
                (e.code, e.offset, e.table, e.row, e.column, e.length),
                (
                    ErrorCode::Framing,
                    Some(at as u64),
                    Some(1),
                    Some(3),
                    Some(2),
                    Some(length)
                ),
                "{v} {at:#x}"
            );
            assert_eq!(e.message, msg);
        }
    }
}

/// A chunk u16 `f0 ff` in a chunked value is `E-FRAMING` at that u16 (patched `long3_exp_18c.dmp`: the first chunk
/// of row 5 at 0x3228f + 2 = 0x32291, which holds `a0 0f` = 4000).
#[test]
fn a_chunk_length_from_0xfff0_is_framing_at_the_chunk() {
    let rel = long3_rel("long3_exp", "18c");
    let p = patched_file(&rel, "t19", "long3_chunk_f0ff.dmp", |b| {
        assert_eq!(b[0x3228f..0x32293], [0xfd, 0xff, 0xa0, 0x0f]);
        put_u16(b, 0x32291, 0xfff0);
    });
    // `Framing` is a hard error of the scan (it is not a stop of a table: the framing is lost, ADR-12)
    let mut d = open(&p, Options::default()).unwrap();
    let e = d.scan().map(|_| ()).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.table, e.row, e.column, e.length),
        (
            ErrorCode::Framing,
            Some(0x32291),
            Some(1),
            Some(5),
            Some(2),
            Some(0xfff0)
        )
    );
    assert_eq!(
        e.message,
        "conventional LONG or LONG RAW chunk length from 0xfff0"
    );
}

/// The largest plain length stays a plain length: `ef ff`/`f0 ff` in rows 3 and 4 of the fixture read (the file scans to
/// the end), and a chunk of `ef ff` (65,519 bytes) would be read as a chunk of that length (here it misframes what
/// follows, never a panic and never `E-UNSUPPORTED-LAYOUT`).
#[test]
fn the_largest_chunk_length_is_not_refused() {
    let rel = long3_rel("long3_exp", "18c");
    let p = patched_file(&rel, "t19", "long3_chunk_ffef.dmp", |b| {
        put_u16(b, 0x32291, 0xffef);
    });
    let mut d = open(&p, Options::default()).unwrap();
    if let Ok(c) = d.scan() {
        if let CatalogEnd::Stopped(e) = &c.end {
            assert_ne!(e.code, ErrorCode::UnsupportedLayout);
            assert_ne!(
                e.message,
                "conventional LONG or LONG RAW chunk length from 0xfff0"
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// (d) the memory guard on 100,000 bytes
// ---------------------------------------------------------------------------------------------------------

/// With `max_lob_bytes = 65,536`, T_LONG3 row 7 gives `E-LOB-TOO-LARGE` at the value's first data byte with `length`
/// the total at the piece that passes the bound: 68,000 (conventional: 17 chunks of 4,000) and 71,568 (direct: 9
/// pieces of 7,952); rows 1 to 6 are read. With 100,000 row 7 reads.
#[test]
fn the_bound_is_checked_as_the_chunks_and_pieces_accumulate() {
    // (file, offset of the first data byte of row 7, length at the error): 0x522f6 + 4; 0x522fb + 2
    for (kind, at, length) in [
        ("long3_exp", 0x522fau64, 68_000u64),
        ("long3_exp_direct", 0x522fd, 71_568),
    ] {
        for v in VERSIONS {
            let rel = long3_rel(kind, v);
            let mut d = open_fixture(&rel);
            let o = ReadOptions {
                max_lob_bytes: 65_536,
                batch_rows: 1,
                ..ReadOptions::default()
            };
            let mut r = d.read_table(TableRef::ByName("T_LONG3"), &o).unwrap();
            let mut n = 0;
            let e = loop {
                match r.next_batch() {
                    Ok(Some(b)) => n += b.num_rows(),
                    Ok(None) => panic!("{rel}: no error"),
                    Err(e) => break e,
                }
            };
            assert_eq!(n, 6, "{rel}: rows 1 to 6 are read");
            assert_eq!(e.code, ErrorCode::LobTooLarge, "{rel}");
            assert_eq!(
                (e.offset, e.row, e.length),
                (Some(at), Some(7), Some(length)),
                "{rel}"
            );
            assert!(e.table == Some(1), "{rel}");
            // the same under null: row 7's L is NULL, V is `v100000`, T_FRAME reads, one W-VALUE-NULLED
            let mut d = open_fixture(&rel);
            let o = ReadOptions {
                max_lob_bytes: 65_536,
                on_value_error: OnValueError::Null,
                ..ReadOptions::default()
            };
            let (b, w) = try_read(&mut d, "T_LONG3", &o).unwrap();
            let r = rows(&b);
            assert_eq!(r.len(), 7);
            assert_eq!(
                (r[6][1].as_str(), r[6][2].as_str()),
                ("<NULL>", "v100000"),
                "{rel}"
            );
            assert!(r[..6].iter().all(|x| x[1] != "<NULL>"), "{rel}");
            let nulled: Vec<_> = w
                .iter()
                .filter(|w| w.code == WarningCode::ValueNulled)
                .collect();
            assert_eq!(nulled.len(), 1, "{rel}: {w:?}");
            assert_eq!(w.len(), 1);
            let (_, fw) = try_read(&mut d, "T_FRAME", &o).unwrap();
            assert!(fw.is_empty());
            // 100,000 is the bound: row 7 reads
            let mut d = open_fixture(&rel);
            let o = ReadOptions {
                max_lob_bytes: 100_000,
                ..ReadOptions::default()
            };
            let (b, w) = try_read(&mut d, "T_LONG3", &o).unwrap();
            assert_eq!(row_count(&b), 7);
            assert!(w.is_empty(), "{rel}: {w:?}");
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// (e) tsp and seg
// ---------------------------------------------------------------------------------------------------------

/// T_TSP of the four files against the decimal DUMP of `truth_tsp.txt` (decoded by the test's own [Y2] / [Y5]
/// arithmetic) and the `TO_CHAR ... FF9` lines (truncated to the unit), row 4 NULL.
#[test]
fn t_tsp_equals_the_truth_in_all_four_files() {
    let d = truth_checks::tsp_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// The break test of the TSP check: a changed DUMP byte and a changed TO_CHAR line are each noticed.
#[test]
fn the_tsp_check_notices_a_changed_truth() {
    let (_, r, _) =
        truth_checks::read_named("18c/tsp_exp_18c.dmp", "T_TSP", &ReadOptions::default()).unwrap();
    let t = truth("18c", "truth_tsp.txt");
    assert!(truth_checks::tsp_diff("x", &t, &r).is_empty());
    let mut m = t.clone();
    let i = m.iter().position(|l| l[0] == "2" && l.len() == 8).unwrap();
    // row 2, T1: 900,000,000 ns (`53,164,233,0`) becomes 900,065,536 ns: the microseconds differ
    m[i][1] = m[i][1].replace(",53,164,233,0", ",53,165,233,0");
    assert_eq!(
        truth_checks::tsp_diff("x", &m, &r).len(),
        1,
        "the DUMP differs"
    );
    let mut m = t.clone();
    let i = m.iter().position(|l| l[0] == "2" && l.len() == 4).unwrap();
    m[i][2] = m[i][2].replace(".999000000", ".998000000");
    assert_eq!(
        truth_checks::tsp_diff("x", &m, &r).len(),
        1,
        "the TO_CHAR line differs"
    );
}

/// The descriptors of T_TSP: type 180, length 11 for T1 to T8 (the columns T1 to T5, T7 and T8), no `W-DDL-FALLBACK`,
/// Arrow Timestamp(Microsecond) for T1 to T5 and Timestamp(Nanosecond) for T7 and T8 (arch 5.1).
#[test]
fn the_tsp_descriptors_and_arrow_types() {
    use arrow_schema::{DataType, TimeUnit};
    let t = truth("18c", "truth_tsp.txt");
    let cols = truth_checks::tsp_columns(&t);
    assert_eq!(cols.len(), 8);
    for v in VERSIONS {
        for kind in ["tsp_exp", "tsp_exp_direct"] {
            let rel = format!("{v}/{kind}_{v}.dmp");
            let mut d = open_fixture(&rel);
            let info = d.table(TableRef::ByName("T_TSP")).unwrap().clone();
            assert_eq!(info.columns.len(), 8, "{rel}");
            assert_eq!(info.num_rows, 4);
            for (c, (name, ty, len, f)) in info.columns.iter().zip(&cols) {
                assert_eq!(&c.name, name, "{rel}");
                if name == "ID" {
                    continue;
                }
                assert!(ty.starts_with("TIMESTAMP"));
                assert_eq!(*len, 11, "{rel} {name}: the truth's data_length");
                assert_eq!((c.type_code, c.max_bytes), (180, *len), "{rel} {name}");
                let want = if *f <= 6 {
                    DataType::Timestamp(TimeUnit::Microsecond, None)
                } else {
                    DataType::Timestamp(TimeUnit::Nanosecond, None)
                };
                assert_eq!(c.arrow_type, want, "{rel} {name}");
            }
            let precisions: Vec<u8> = cols.iter().skip(1).map(|c| c.3).collect();
            assert_eq!(precisions, [1, 2, 3, 4, 5, 7, 8]);
            assert!(d
                .warnings()
                .iter()
                .all(|w| w.code != WarningCode::DdlFallback));
            let (_, w) = try_read(&mut d, "T_TSP", &ReadOptions::default()).unwrap();
            assert!(w.is_empty(), "{rel}: {w:?}");
        }
    }
}

/// `seg_exp(_direct)_18c`: three tables with 0 rows each (`truth_seg.txt`), the end markers E1, E1, E1
/// (conventional) and E1, E1, E2 (direct; T_SEG2 has no segment, so no `ff ff`: [E2]). The expectation comes from
/// the truth lines (`segment_created=YES` -> E1, `NO` -> E2 on the direct path) and the golden catalog.
#[test]
fn the_seg_files_hold_three_empty_tables_with_the_end_markers_of_the_truth() {
    let d = truth_checks::seg_diffs();
    assert!(d.is_empty(), "{d:?}");
    let t = truth("18c", "truth_seg.txt");
    assert_eq!(t.len(), 3);
    for (kind, direct) in [("seg_exp", false), ("seg_exp_direct", true)] {
        let rel = format!("18c/{kind}_18c.dmp");
        let mut d = open_fixture(&rel);
        let c = d.scan().unwrap();
        assert!(matches!(c.end, CatalogEnd::Complete), "{rel}");
        assert_eq!(c.tables.len(), 3);
        let golden = std::fs::read_to_string(root().join("tests/golden/exp_catalog.tsv")).unwrap();
        for (tab, l) in c.tables.iter().zip(&t) {
            assert_eq!(tab.name, l[0]);
            assert_eq!(tab.num_rows, 0, "{rel} {}", tab.name);
            let segment = l[1] == "segment_created=YES";
            let want = if direct && !segment {
                EndMarker::LfOnly
            } else {
                EndMarker::FfFf
            };
            assert_eq!(tab.parts[0].end_marker, want, "{rel} {}", tab.name);
            // the golden catalog (exp_parse.py) says the same
            let line = golden
                .lines()
                .find(|g| {
                    g.starts_with(&format!("18c/{kind}_18c.dmp\t"))
                        && g.split('\t').nth(2) == Some(tab.name.as_str())
                })
                .unwrap();
            let marker = line.rsplit('\t').next().unwrap().split(',').nth(3).unwrap();
            assert_eq!(
                marker,
                if want == EndMarker::LfOnly {
                    "lf"
                } else {
                    "ffff"
                }
            );
        }
        // E1, E1, E1 (conventional) and E1, E1, E2 (direct)
        let markers: Vec<EndMarker> = c.tables.iter().map(|t| t.parts[0].end_marker).collect();
        let want = if direct {
            [EndMarker::FfFf, EndMarker::FfFf, EndMarker::LfOnly]
        } else {
            [EndMarker::FfFf; 3]
        };
        assert_eq!(markers, want, "{rel}");
        assert!(d
            .warnings()
            .iter()
            .all(|w| w.code != WarningCode::UntestedLayout));
    }
}

// ---------------------------------------------------------------------------------------------------------
// ADR-95 item 2, ADR-99 item 2: a chunked total below 65,534 bytes
// ---------------------------------------------------------------------------------------------------------

/// T_LONG3 row 5 of a patched `long3_exp_18c.dmp` is chunked with 65,533 bytes (below the 65,534 of `long4_exp_<v>` row
/// 6, the smallest chunked size seen): it is decoded (the whole value, two bytes short of the truth's row 5), and
/// `W-UNTESTED-LAYOUT` stands once at the value's first data byte, table 1, row 5, column 2; with `strict` the scan
/// fails with `E-STRICT`. (M6: with the threshold back at 65,535 the unpatched row 5 of `long4_exp_<v>`, row 6, warns:
/// `t_long4_equals_the_truth_in_all_four_files`, `no_warning_in_the_new_long_files`.)
#[test]
fn a_patched_chunked_value_below_65534_bytes_warns_and_is_read() {
    let (p, at) = long3_chunked_short_copy("t19", "long3_short.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    let c = d.scan().unwrap();
    assert!(matches!(c.end, CatalogEnd::Complete));
    let ws: Vec<_> = d
        .warnings()
        .iter()
        .filter(|w| w.code == WarningCode::UntestedLayout)
        .collect();
    assert_eq!(ws.len(), 1, "{:?}", d.warnings());
    assert_eq!(
        (ws[0].offset, ws[0].table, ws[0].row, ws[0].column),
        (Some(at), Some(1), Some(5), Some(2))
    );
    assert_eq!(
        ws[0].message,
        "conventional LONG or LONG RAW in the chunked form with fewer than 65,534 bytes"
    );
    let l = d
        .table(TableRef::ByName("T_LONG3"))
        .unwrap()
        .columns
        .iter()
        .find(|c| c.name == "L")
        .and_then(|c| c.long.clone())
        .unwrap();
    assert_eq!((l.values, l.chunked, l.chunked_short), (7, 3, 1));
    let (b, w) = try_read(&mut d, "T_LONG3", &ReadOptions::default()).unwrap();
    assert!(w.is_empty(), "{w:?}");
    let r = rows(&b);
    assert_eq!(r[4][1].len(), 65_533);
    assert_eq!(
        r[4][1],
        format!("L065535{}", "y".repeat(65_535 - 7))[..65_533]
    );
    // strict: E-STRICT at the scan
    let e = open(
        &p,
        Options {
            strict: true,
            ..Options::default()
        },
    )
    .and_then(|mut d| d.scan().map(|_| ()))
    .unwrap_err();
    assert_eq!(e.code, ErrorCode::Strict);
}

/// The 100,001-byte copies give `W-LONG-LENGTH` once, at the value's first data byte, on both paths; 100,000 none.
#[test]
fn w_long_length_is_above_100000_bytes_on_both_paths() {
    let (p, at) = long3_100001_copy("t19", "long3_100001.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    let (_, w) = try_read(&mut d, "T_LONG3", &ReadOptions::default()).unwrap();
    assert_eq!(w.len(), 1, "{w:?}");
    assert_eq!(
        (w[0].code, w[0].offset, w[0].row, w[0].column),
        (WarningCode::LongLength, Some(at), Some(7), Some(2))
    );
    let (p, at) = long3_direct_100001_copy("t19", "long3_direct_100001.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    let (_, w) = try_read(&mut d, "T_LONG3", &ReadOptions::default()).unwrap();
    assert_eq!(w.len(), 1, "{w:?}");
    assert_eq!(
        (w[0].code, w[0].offset, w[0].row, w[0].column),
        (WarningCode::LongLength, Some(at), Some(7), Some(3))
    );
}

// =========================================================================================================
// T-19b: long4, long5, longraw4, longraw5 (R-048, R-051), tsltz2 (R-050)
// =========================================================================================================
//
// Expected numbers come from the Analyst's tools run read-only in T-19b: `long4_check.py` and `long5_check.py` (the
// offsets and forms of the 7 + 6 values: `rowlen_stats.py` lists `long4_exp_18c.dmp@0x2271` 65,521 one field,
// 0x12272 65,525 one field, 0x22277 / 0x32285 / 0x42295 form (b) with 65,530 / 65,532 / 65,533, 0x522a6 and 0x622d8
// chunked with 65,534 and 65,535), the truth files `truth_long4.txt`, `truth_long5.txt`, `truth_longraw4.txt`,
// `truth_longraw5.txt`, `truth_tsltz2.txt`, and `corpus_counts.py` (229 files, `problems 0`). The value of every L is the
// arithmetic of R-040, every LR is n bytes `ab` (R-048), both built in `common/truth_checks.rs` from the truth
// length, never from the reader.

/// What the scan counts for the LONG / LONG RAW column of one file: `(values, chained, chunked, largest,
/// above_32000, sizes_65521_to_65534)`.
type Stats = (u64, u64, u64, u64, u64, u64);

struct LongSet {
    /// `<table>`
    table: &'static str,
    /// the LONG / LONG RAW column
    column: &'static str,
    /// the file stems (conventional, direct) without the version
    stems: [&'static str; 2],
    /// per version: rows, conventional stats, direct stats
    per_version: &'static [(&'static str, Stats, Stats)],
}

const LONG_SETS: [LongSet; 4] = [
    // `long4_check.py`: conventional two `fd ff` values (rows 6, 7), direct all seven in pieces; six sizes in 65,521..=65,534
    LongSet {
        table: "T_LONG4",
        column: "L",
        stems: ["long4_exp", "long4_exp_direct"],
        per_version: &[
            ("18c", (7, 0, 2, 65_535, 7, 6), (7, 7, 0, 65_535, 7, 6)),
            ("11g", (7, 0, 2, 65_535, 7, 6), (7, 7, 0, 65_535, 7, 6)),
        ],
    },
    // `long5_check.py`: 65,526 to 65,531, no chunked value
    LongSet {
        table: "T_LONG5",
        column: "L",
        stems: ["long5_exp", "long5_exp_direct"],
        per_version: &[
            ("18c", (6, 0, 0, 65_531, 6, 6), (6, 6, 0, 65_531, 6, 6)),
            ("11g", (6, 0, 0, 65_531, 6, 6), (6, 6, 0, 65_531, 6, 6)),
        ],
    },
    // R-048 ran LONG RAW on 18c: 32,768, 65,520, 65,535, 100,000 (the last two chunked)
    LongSet {
        table: "T_LONGRAW4",
        column: "LR",
        stems: ["longraw4_exp", "longraw4_exp_direct"],
        per_version: &[("18c", (4, 0, 2, 100_000, 4, 0), (4, 4, 0, 100_000, 4, 0))],
    },
    // R-051: 18c seven sizes (65,534 chunked); 11g eleven (the same and 32,768, 65,520, 65,535, 100,000)
    LongSet {
        table: "T_LONGRAW5",
        column: "LR",
        stems: ["longraw5_exp", "longraw5_exp_direct"],
        per_version: &[
            ("18c", (7, 0, 1, 65_534, 7, 7), (7, 7, 0, 65_534, 7, 7)),
            (
                "11g",
                (11, 0, 3, 100_000, 11, 7),
                (11, 11, 0, 100_000, 11, 7),
            ),
        ],
    },
];

fn long_column(d: &mut Dump, set: &LongSet) -> oradmp_core::exp::rows::LongStats {
    d.table(TableRef::ByName(set.table))
        .unwrap()
        .columns
        .iter()
        .find(|c| c.name == set.column)
        .and_then(|c| c.long.clone())
        .unwrap()
}

/// T_LONG4 of the four files against `truth_long4.txt`: length, first and last 20 bytes, V and the whole L.
#[test]
fn t_long4_equals_the_truth_in_all_four_files() {
    let d = truth_checks::long4_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// T_LONG5 of the four files against `truth_long5.txt` (65,526 to 65,531 bytes).
#[test]
fn t_long5_equals_the_truth_in_all_four_files() {
    let d = truth_checks::long5_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// T_LONGRAW4 of the two 18c files against `truth_longraw4.txt` (32,768, 65,520, 65,535, 100,000 bytes of `ab`).
#[test]
fn t_longraw4_equals_the_truth_in_both_18c_files() {
    let d = truth_checks::longraw4_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// T_LONGRAW5 of the four files against `truth_longraw5.txt` of each folder (18c 7 rows, 11g 11 rows).
#[test]
fn t_longraw5_equals_the_truth_in_all_four_files() {
    let d = truth_checks::longraw5_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// The pure checks fail on a wrong truth line (the break test of the families): a length, a first 20 / 10 bytes, a V.
#[test]
fn the_new_long_checks_notice_a_changed_truth() {
    let (_, rows, _) =
        truth_checks::read_named("18c/long4_exp_18c.dmp", "T_LONG4", &ReadOptions::default())
            .unwrap();
    let t = truth("18c", "truth_long4.txt");
    assert!(truth_checks::long3_diff("x", &t, &rows).is_empty());
    let mut m = t.clone();
    m[4][1] = "65534".to_string();
    assert!(!truth_checks::long3_diff("x", &m, &rows).is_empty());
    let mut m = t.clone();
    m[0][2] = m[0][2].replacen("4C30", "4C31", 1);
    assert_eq!(truth_checks::long3_diff("x", &m, &rows).len(), 1);
    let mut m = t.clone();
    m[6][4] = "w65535".to_string();
    assert_eq!(truth_checks::long3_diff("x", &m, &rows).len(), 1);

    let (_, rows, _) = truth_checks::read_named(
        "18c/longraw4_exp_18c.dmp",
        "T_LONGRAW4",
        &ReadOptions::default(),
    )
    .unwrap();
    let t = truth("18c", "truth_longraw4.txt");
    assert!(truth_checks::longraw_diff("x", &t, &rows).is_empty());
    let mut m = t.clone();
    m[1][1] = "65521".to_string();
    assert!(!truth_checks::longraw_diff("x", &m, &rows).is_empty());
    let mut m = t.clone();
    m[2][3] = m[2][3].replacen("AB", "AC", 1);
    assert_eq!(truth_checks::longraw_diff("x", &m, &rows).len(), 1);
    // a value that differs in the middle only is found by the whole-value comparison
    let mut changed = rows.clone();
    changed[0][1].replace_range(30..32, "ac");
    assert_eq!(truth_checks::longraw_diff("x", &t, &changed).len(), 1);
}

/// T_FRAME behind the LONG table equals its truth in all 14 files (`truth_frame.txt`, 7 rows), with no warning.
#[test]
fn t_frame_of_the_new_long_files_equals_its_truth() {
    let mut files = 0;
    for set in &LONG_SETS {
        for (v, _, _) in set.per_version {
            let want: Vec<Vec<String>> = truth(v, "truth_frame.txt")
                .into_iter()
                .map(|l| {
                    l.into_iter()
                        .map(|x| if x == "NULL" { "<NULL>".to_string() } else { x })
                        .collect()
                })
                .collect();
            assert_eq!(want.len(), 7);
            for stem in set.stems {
                let (b, w) = read_default(&format!("{v}/{stem}_{v}.dmp"), "T_FRAME");
                assert!(w.is_empty(), "{v} {stem}");
                assert_eq!(rows(&b), want, "{v} {stem}");
                files += 1;
            }
        }
    }
    assert_eq!(files, 14);
}

/// The `LongStats` of the LONG / LONG RAW column of each of the 14 files (the numbers in `LONG_SETS`: row counts,
/// chunked values and the sizes from the truth files; `sizes_65521_to_65534` counts the values of 65,521 to 65,534
/// bytes in the truth), and no warning at all with the default options and with `strict`, at open, scan and read.
#[test]
fn long_stats_and_no_warning_in_the_new_long_files() {
    let mut files = 0;
    for set in &LONG_SETS {
        for (v, conv, direct) in set.per_version {
            // the band count from the truth file of the folder, not from the reader
            let truth_lines = truth(v, &format!("truth_{}.txt", set.table[2..].to_lowercase()));
            let band = truth_lines
                .iter()
                .filter(|l| (65_521..=65_534).contains(&l[1].parse::<u64>().unwrap()))
                .count() as u64;
            assert_eq!(band, conv.5, "{v} {}", set.table);
            for (stem, want) in set.stems.iter().zip([conv, direct]) {
                for strict in [false, true] {
                    let rel = format!("{v}/{stem}_{v}.dmp");
                    let mut d = open(
                        fixtures().join(&rel),
                        Options {
                            strict,
                            ..Options::default()
                        },
                    )
                    .unwrap();
                    let l = long_column(&mut d, set);
                    assert_eq!(
                        (
                            l.values,
                            l.chained,
                            l.chunked,
                            l.largest,
                            l.above_32000,
                            l.sizes_65521_to_65534
                        ),
                        *want,
                        "{rel} strict={strict}"
                    );
                    assert_eq!((l.zero_pieces, l.chunked_short), (0, 0), "{rel}");
                    assert!(d.warnings().is_empty(), "{rel}: {:?}", d.warnings());
                    let o = ReadOptions::default();
                    for name in [set.table, "T_FRAME"] {
                        let (_, w) = try_read(&mut d, name, &o).unwrap();
                        assert!(w.is_empty(), "{rel} {name} strict={strict}: {w:?}");
                    }
                    assert!(d.warnings().is_empty(), "{rel}");
                }
                files += 1;
            }
        }
    }
    assert_eq!(files, 14);
}

/// The assembled digest of the LONG / LONG RAW column (`long_sha256`) of the conventional file equals the direct
/// file's (same version); for `long4` and `long5` all four files agree; the raw digests differ (the forms differ);
/// and the digest is the one of the assembled values built from the truth (the total as u64 LE and the bytes).
#[test]
fn the_assembled_long_digest_agrees_between_the_paths_and_the_versions() {
    use sha2::{Digest, Sha256};
    let digest = |rel: &str| {
        let r = walk_path(
            &fixtures().join(rel),
            &WalkOptions {
                hash_fields: true,
                ..WalkOptions::default()
            },
        )
        .unwrap();
        let part = &r.sections[0].parts[0];
        assert_eq!(part.rows.long_sha256.len(), 1, "{rel}");
        (part.rows.long_sha256[0], part.rows.raw_sha256.unwrap())
    };
    for set in &LONG_SETS {
        let mut all = Vec::new();
        for (v, _, _) in set.per_version {
            let c = digest(&format!("{v}/{}_{v}.dmp", set.stems[0]));
            let d = digest(&format!("{v}/{}_{v}.dmp", set.stems[1]));
            assert_eq!(c.0, d.0, "{v} {}", set.table);
            assert_ne!(c.1, d.1, "{v} {}: the raw digests differ", set.table);
            // from the truth
            let t = truth(v, &format!("truth_{}.txt", set.table[2..].to_lowercase()));
            let mut h = Sha256::new();
            for l in &t {
                let size: usize = l[1].parse().unwrap();
                h.update((size as u64).to_le_bytes());
                if set.column == "L" {
                    h.update(format!("L{size:06}{}", "y".repeat(size - 7)).as_bytes());
                } else {
                    h.update(vec![0xab; size]);
                }
            }
            let want: [u8; 32] = h.finalize().into();
            assert_eq!(c.0, want, "{v} {}", set.table);
            all.push(c.0);
        }
        if set.table.starts_with("T_LONG") && !set.table.contains("RAW") {
            assert_eq!(all[0], all[1], "{}: 18c and 11g agree", set.table);
        }
    }
}

/// The raw digest tells the three forms apart: `long4_exp_18c` holds one-field values, form (b) values and chunked
/// values, and its raw digest is not the one of the same walk with form (b) serialised as a plain field (M9 changes
/// `gen_exp_raw.py` and the manifest comparison catches it); here the digest of the first four values is computed from
/// the file by the test: plain fields as `u32 n` + bytes, form (b) as `f9 ff ff ff` + `u32 n` + bytes.
#[test]
fn the_raw_digest_of_form_b_is_the_marker_the_length_and_the_bytes() {
    use sha2::{Digest, Sha256};
    let data = std::fs::read(fixtures().join("18c/long4_exp_18c.dmp")).unwrap();
    // row 3 at 0x22277 (`long4_check.py`): `fa ff`, n = 65,530, `00 00`, the bytes
    assert_eq!(data[0x22277..0x2227d], [0xfa, 0xff, 0xfa, 0xff, 0x00, 0x00]);
    let value = &data[0x2227d..0x2227d + 65_530];
    let mut h = Sha256::new();
    h.update([0xf9, 0xff, 0xff, 0xff]);
    h.update(65_530u32.to_le_bytes());
    h.update(value);
    let want_form_b: [u8; 32] = h.finalize().into();
    let mut p = Sha256::new();
    p.update(65_530u32.to_le_bytes());
    p.update(value);
    let as_plain: [u8; 32] = p.finalize().into();
    assert_ne!(want_form_b, as_plain);
    // the golden file's line for the table is the one the generator wrote from `long_fa` of exp_parse.py; the
    // manifest test compares the walker with it (`t2_manifest`). Here: the raw digest of a table is the SHA-256 of the
    // serialisation, which for a one-row table made by a patched copy can be built by hand.
    let ref_path = patched_file("18c/long4_exp_18c.dmp", "t19b", "long4_raw.dmp", |_| {});
    let r = walk_path(
        &ref_path,
        &WalkOptions {
            hash_fields: true,
            ..WalkOptions::default()
        },
    )
    .unwrap();
    // rebuild the whole serialisation of T_LONG4 (ID, L, V) from the file by the rules of `gen_exp_raw.py`
    let mut h = Sha256::new();
    let mut at = r.sections[0].parts[0].header.rows_offset as usize;
    let u16_at = |p: usize| usize::from(u16::from_le_bytes([data[p], data[p + 1]]));
    for _ in 0..7 {
        for col in 0..3 {
            let u = u16_at(at);
            if col == 1 && u == 0xfffa {
                let n = u16_at(at + 2);
                h.update([0xf9, 0xff, 0xff, 0xff]);
                h.update((n as u32).to_le_bytes());
                h.update(&data[at + 6..at + 6 + n]);
                at += 6 + n;
            } else if col == 1 && u == 0xfffd {
                h.update([0xfb, 0xff, 0xff, 0xff]);
                at += 2;
                loop {
                    let c = u16_at(at);
                    at += 2;
                    if c == 0 {
                        break;
                    }
                    h.update((c as u32).to_le_bytes());
                    h.update(&data[at..at + c]);
                    at += c;
                }
                h.update([0xfa, 0xff, 0xff, 0xff]);
            } else {
                h.update((u as u32).to_le_bytes());
                h.update(&data[at + 2..at + 2 + u]);
                at += 2 + u;
            }
        }
        assert_eq!(u16_at(at), 0, "row end");
        at += 2;
    }
    let want: [u8; 32] = h.finalize().into();
    assert_eq!(r.sections[0].parts[0].rows.raw_sha256.unwrap(), want);
}

/// ADR-102 item 3 (T-19b (c)): `Dump::rows` and `Dump::part_rows` with `buffer_bytes = 4096` read `long4_exp_18c` row 5
/// (65,533 bytes, form (b)) and `long3_exp_18c` row 4 (65,520 bytes, one field) equal to their truth: the buffer is
/// raised to `MIN_BUFFER_BYTES`. (M8: without the raise the sink path cannot lend 65,520 bytes: `E-TRUNCATED`.)
#[test]
fn a_small_buffer_is_raised_for_the_row_cursor() {
    #[derive(Default)]
    struct Collect {
        values: Vec<(u32, u64, Vec<u8>)>,
    }
    impl RowSink for Collect {
        fn row_start(&mut self, _row: u64) {}
        fn null(&mut self, _c: u32, _o: u64) -> oradmp_core::Result<()> {
            Ok(())
        }
        fn value(&mut self, c: u32, o: u64, b: &[u8]) -> oradmp_core::Result<()> {
            if b.len() > 1000 {
                self.values.push((c, o, b.to_vec()));
            }
            Ok(())
        }
        fn chain_piece(
            &mut self,
            _c: u32,
            _o: u64,
            _p: &[u8],
            _f: bool,
            _l: bool,
        ) -> oradmp_core::Result<()> {
            Ok(())
        }
        fn row_end(&mut self) -> oradmp_core::Result<()> {
            Ok(())
        }
    }
    for (rel, table, row, size) in [
        ("18c/long4_exp_18c.dmp", "T_LONG4", 5usize, 65_533usize),
        ("18c/long3_exp_18c.dmp", "T_LONG3", 4, 65_520),
    ] {
        for use_part in [false, true] {
            let mut d = open(
                fixtures().join(rel),
                Options {
                    buffer_bytes: 4096,
                    ..Options::default()
                },
            )
            .unwrap();
            d.scan().unwrap();
            let mut cur = if use_part {
                d.part_rows(TableRef::ByName(table), 0).unwrap()
            } else {
                d.rows(TableRef::ByName(table)).unwrap()
            };
            let mut got = None;
            for r in 1..=row {
                let mut c = Collect::default();
                assert!(cur.next_row_with(&mut c).unwrap().is_some(), "{rel} {r}");
                if r == row {
                    got = c.values.pop();
                }
            }
            let (col, _, bytes) = got.unwrap();
            // the file order is ID, L, V on the conventional path
            assert_eq!(col, 2, "{rel}");
            assert_eq!(bytes.len(), size, "{rel}");
            let t = truth("18c", &format!("truth_{}.txt", table[2..].to_lowercase()));
            let line = t.iter().find(|l| l[0] == row.to_string()).unwrap();
            assert_eq!(line[1], size.to_string());
            assert_eq!(
                bytes,
                format!("L{size:06}{}", "y".repeat(size - 7)).into_bytes(),
                "{rel} use_part={use_part}"
            );
        }
    }
}

/// The reader (`read_table`) also reads the two rows with `buffer_bytes = 4096`.
#[test]
fn read_table_with_a_small_buffer_reads_the_big_rows() {
    for (rel, table, row, size) in [
        ("18c/long4_exp_18c.dmp", "T_LONG4", 5usize, 65_533usize),
        ("18c/long3_exp_18c.dmp", "T_LONG3", 4, 65_520),
    ] {
        let mut d = open(
            fixtures().join(rel),
            Options {
                buffer_bytes: 4096,
                ..Options::default()
            },
        )
        .unwrap();
        let (b, w) = try_read(&mut d, table, &ReadOptions::default()).unwrap();
        assert!(w.is_empty());
        assert_eq!(rows(&b)[row - 1][1].len(), size, "{rel}");
    }
}

/// `max_lob_bytes = 65,531` on `long4_exp_<v>`: row 4 (65,532 bytes, form (b), the field at 0x32285) gives
/// `E-LOB-TOO-LARGE` at 0x3228b (field + 6) with `length` 65,532, table 1, column 2; rows 1 to 3 read. With
/// `on_value_error = null`, rows 4 to 7 have L NULL and V intact (`v65532` to `v65535`), T_FRAME reads, and the
/// warning is `W-VALUE-NULLED` counted by the existing rule (one warning for the column).
#[test]
fn the_memory_guard_applies_to_form_b() {
    for v in VERSIONS {
        let rel = format!("{v}/long4_exp_{v}.dmp");
        let mut d = open_fixture(&rel);
        let o = ReadOptions {
            max_lob_bytes: 65_531,
            batch_rows: 1,
            ..ReadOptions::default()
        };
        let mut r = d.read_table(TableRef::ByName("T_LONG4"), &o).unwrap();
        let mut n = 0;
        let e = loop {
            match r.next_batch() {
                Ok(Some(b)) => n += b.num_rows(),
                Ok(None) => panic!("{rel}: no error"),
                Err(e) => break e,
            }
        };
        assert_eq!(n, 3, "{rel}: rows 1 to 3 are read");
        assert_eq!(e.code, ErrorCode::LobTooLarge, "{rel}");
        assert_eq!(
            (e.offset, e.row, e.column, e.length, e.table),
            (Some(0x3228b), Some(4), Some(2), Some(65_532), Some(1)),
            "{rel}"
        );
        let mut d = open_fixture(&rel);
        let o = ReadOptions {
            max_lob_bytes: 65_531,
            on_value_error: OnValueError::Null,
            ..ReadOptions::default()
        };
        let (b, w) = try_read(&mut d, "T_LONG4", &o).unwrap();
        let r = rows(&b);
        assert_eq!(r.len(), 7);
        for (i, x) in r.iter().enumerate() {
            if i < 3 {
                assert_ne!(x[1], "<NULL>", "{rel} row {}", i + 1);
            } else {
                assert_eq!(x[1], "<NULL>", "{rel} row {}", i + 1);
                assert_eq!(x[2], format!("v{}", 65_529 + i), "{rel} row {}", i + 1);
            }
        }
        // rows 4 to 7 are 65,532 / 65,533 / 65,534 / 65,535 bytes: the V of row 4 is `v65532` (the ids are 1 to 7)
        assert_eq!(r[3][2], "v65532");
        let nulled: Vec<_> = w
            .iter()
            .filter(|w| w.code == WarningCode::ValueNulled)
            .collect();
        assert_eq!(nulled.len(), 1, "{rel}: {w:?}");
        assert_eq!(w.len(), 1, "{rel}: {w:?}");
        let (_, fw) = try_read(&mut d, "T_FRAME", &o).unwrap();
        assert!(fw.is_empty());
    }
}

/// A value of 65,522 bytes (a size no fixture shows, R-052; `long4_exp_18c` row 1 patched to `f2 ff` with one byte
/// added) is read as one field with `W-UNTESTED-LAYOUT` at its u16 (0x2271), table 1, row 1, column 2, once; the value
/// is whole; `strict` fails with `E-STRICT`; the second and later 0xfff2 to 0xfff4 values of the column add nothing.
#[test]
fn a_length_of_0xfff2_is_read_as_one_field_with_a_warning() {
    let (p, at) = long4_fff2_copy("t19b", "long4_fff2.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    let c = d.scan().unwrap();
    assert!(matches!(c.end, CatalogEnd::Complete));
    let ws: Vec<_> = d
        .warnings()
        .iter()
        .filter(|w| w.code == WarningCode::UntestedLayout)
        .collect();
    assert_eq!(ws.len(), 1, "{:?}", d.warnings());
    assert_eq!(
        (ws[0].offset, ws[0].table, ws[0].row, ws[0].column),
        (Some(at), Some(1), Some(1), Some(2))
    );
    assert_eq!(
        ws[0].message,
        "conventional LONG or LONG RAW length 0xfff2 to 0xfff4 read as one field, not in the fixtures (R-052)"
    );
    let (b, w) = try_read(&mut d, "T_LONG4", &ReadOptions::default()).unwrap();
    assert!(w.is_empty(), "{w:?}");
    let r = rows(&b);
    assert_eq!(r[0][1].len(), 65_522);
    assert_eq!(r[0][1], format!("L065521{}y", "y".repeat(65_521 - 7)));
    assert_eq!(r[1][1].len(), 65_525);
    let l = d
        .table(TableRef::ByName("T_LONG4"))
        .unwrap()
        .columns
        .iter()
        .find(|c| c.name == "L")
        .and_then(|c| c.long.clone())
        .unwrap();
    // the 65,522-byte value is in the band of 65,521 to 65,534 bytes as well
    assert_eq!(
        (l.values, l.fff2_to_fff4, l.sizes_65521_to_65534),
        (7, 1, 6)
    );
    let e = open(
        &p,
        Options {
            strict: true,
            ..Options::default()
        },
    )
    .and_then(|mut d| d.scan().map(|_| ()))
    .unwrap_err();
    assert_eq!(e.code, ErrorCode::Strict);
}

// ---------------------------------------------------------------------------------------------------------
// T-19b (b): TSLTZ, 7-byte values
// ---------------------------------------------------------------------------------------------------------

/// T_TSLTZ2 of the four files against `truth_tsltz2.txt`: every value equals the decimal `Typ=231` bytes decoded by
/// the test's own [Y2] / [Y5] arithmetic and the `TO_CHAR` lines (truncated to the unit); row 4 is all NULL.
#[test]
fn t_tsltz2_equals_the_truth_in_all_four_files() {
    let d = truth_checks::tsltz2_diffs();
    assert!(d.is_empty(), "{d:?}");
}

/// The break test of the TSLTZ check: a changed DUMP byte and a changed TO_CHAR line are each noticed.
#[test]
fn the_tsltz2_check_notices_a_changed_truth() {
    let (_, r, _) = truth_checks::read_named(
        "18c/tsltz2_exp_18c.dmp",
        "T_TSLTZ2",
        &ReadOptions::default(),
    )
    .unwrap();
    let t = truth("18c", "truth_tsltz2.txt");
    assert!(truth_checks::tsltz2_diff("x", &t, &r).is_empty());
    // row 3, L0 (a 7-byte value): the seconds byte 7 becomes 8
    let mut m = t.clone();
    let i = m
        .iter()
        .position(|l| l[0] == "3" && l[1].starts_with("Typ=231"))
        .unwrap();
    m[i][1] = m[i][1].replace(",5,7", ",5,8");
    assert_eq!(truth_checks::tsltz2_diff("x", &m, &r).len(), 1);
    // row 2, L3: 123 ms becomes 124 ms in the TO_CHAR line
    let mut m = t.clone();
    let i = m
        .iter()
        .position(|l| l[0] == "2" && l[2].contains("05.123000000"))
        .unwrap();
    m[i][2] = m[i][2].replace("05.123000000", "05.124000000");
    assert_eq!(truth_checks::tsltz2_diff("x", &m, &r).len(), 1);
}

/// `tsltz2_exp(_direct)_<v>` (`tsltz2_check.py`): the descriptors of T_TSLTZ2 are type 231 with lengths 7 / 11 / 11
/// (L0, L3, L9), no `W-DDL-FALLBACK`; the Arrow types are Timestamp(Microsecond) for L0 and L3 and
/// Timestamp(Nanosecond) for L9, no zone (arch 5.1); the warnings of the table are exactly three `W-TSLTZ` (one per
/// column) and nothing else, at open, scan and read, with `strict` off.
#[test]
fn the_tsltz2_descriptors_types_and_warnings() {
    use arrow_schema::{DataType, TimeUnit};
    let t = truth("18c", "truth_tsltz2.txt");
    let cols = truth_checks::tsltz2_columns(&t);
    assert_eq!(
        cols,
        [
            ("L0".to_string(), 7, 0),
            ("L3".to_string(), 11, 3),
            ("L9".to_string(), 11, 9)
        ]
    );
    for v in VERSIONS {
        for kind in ["tsltz2_exp", "tsltz2_exp_direct"] {
            let rel = format!("{v}/{kind}_{v}.dmp");
            let mut d = open_fixture(&rel);
            let info = d.table(TableRef::ByName("T_TSLTZ2")).unwrap().clone();
            assert_eq!(info.columns.len(), 4, "{rel}");
            assert_eq!(info.num_rows, 4);
            for (c, (name, len, f)) in info.columns.iter().skip(1).zip(&cols) {
                assert_eq!(&c.name, name, "{rel}");
                assert_eq!((c.type_code, c.max_bytes), (231, *len), "{rel} {name}");
                let want = if *f <= 6 {
                    DataType::Timestamp(TimeUnit::Microsecond, None)
                } else {
                    DataType::Timestamp(TimeUnit::Nanosecond, None)
                };
                assert_eq!(c.arrow_type, want, "{rel} {name}");
            }
            assert!(d
                .warnings()
                .iter()
                .all(|w| w.code != WarningCode::DdlFallback));
            let (b, w) = try_read(&mut d, "T_TSLTZ2", &ReadOptions::default()).unwrap();
            // the TSLTZ warnings: one per column, nothing else; the 7-byte values add no `W-UNVERIFIED-TYPE`
            let mut codes: Vec<(WarningCode, Option<u32>)> =
                d.warnings().iter().map(|w| (w.code, w.column)).collect();
            codes.extend(w.iter().map(|w| (w.code, w.column)));
            assert!(
                codes.iter().all(|c| c.0 == WarningCode::Tsltz),
                "{rel}: {codes:?}"
            );
            let mut cs: Vec<_> = codes.iter().map(|c| c.1).collect();
            cs.sort_unstable();
            cs.dedup();
            assert_eq!(codes.len(), 3, "{rel}: {codes:?}");
            assert_eq!(cs, [Some(2), Some(3), Some(4)], "{rel}");
            // row 4 is all NULL
            assert_eq!(rows(&b)[3][1..], ["<NULL>", "<NULL>", "<NULL>"], "{rel}");
        }
    }
}
