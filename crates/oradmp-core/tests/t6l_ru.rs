//! Task T-6L (part 2) and T-6M: CLOB values of a single-byte database (the 7 `ru_*` files of `11g-cp1251`),
//! the two `ru_lob2_*` files, the confirmed and unconfirmed cases of ADR-25 on synthetic files, and a partitioned LOB table (ADR-32 item 5, synthetic).
//!
//! Expected values come from the plan, `truth_ru.txt` (R_CLOB, R_TEXT lines: DUMP of the database bytes), the
//! SQL of `corpus/sql/07_ru_objects.sql` (the Cyrillic table: `'абв'` and 1, NULL and 2) and
//! `corpus/sql/01b_optional_tables.sql` / `13_optional.sql`. Row 2 of R_CLOB is a short-form value of 6,000
//! bytes (`70 17`), which the walker reports as such (`tests/t3a_walker.rs`, Analyst's `tools/lob_check.py`).
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
use arrow_schema::DataType;
use common::*;
use oradmp_core::dump::{Dump, Options, TableRef};
use oradmp_core::reader::{OnValueError, ReadOptions, UnconfirmedLob};
use oradmp_core::{ErrorCode, WarningCode};

const RU_FILES: [&str; 7] = [
    "ru_exp_al32utf8_11g.dmp",
    "ru_exp_cp1251_11g.dmp",
    "ru_exp_direct_cp1251_11g.dmp",
    "ru_exp_direct_us7ascii_11g.dmp",
    "ru_exp_us7ascii_11g.dmp",
    "ru_exp_w1252_11g.dmp",
    "ru_tables_exp_us7ascii_11g.dmp",
];

fn ru(file: &str) -> Dump {
    open_fixture(&format!("11g-cp1251/{file}"))
}

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

/// The bytes of a `DUMP(x, 16)` truth field (`Typ=1 Len=6: cf,f0,...`); `None` for `NULL`.
fn dump_bytes(f: &str) -> Option<Vec<u8>> {
    if f == "NULL" {
        return None;
    }
    let (_, list) = f.split_once(": ")?;
    Some(
        list.split(',')
            .map(|h| u8::from_str_radix(h, 16).unwrap())
            .collect(),
    )
}

fn cp1251_bytes(s: &str) -> Vec<u8> {
    let (b, _, bad) = encoding_rs::WINDOWS_1251.encode(s);
    assert!(!bad);
    b.into_owned()
}

/// (b) R_CLOB in all 7 files: row 1 = the 6 database bytes of `truth_ru.txt` decoded with CL8MSWIN1251 (the
/// word `Привет`), row 2 = 6,000 characters whose first 60 bytes equal the truth, row 3 NULL; the same with
/// every client set (AL32UTF8, CL8MSWIN1251, US7ASCII, WE8MSWIN1252, conventional and `direct=y`). The
/// column is LargeUtf8; no warning.
#[test]
fn r_clob_in_all_seven_files() {
    // T-10b: the truth comparison is a function of `common/truth_checks.rs` (also called by t10_truth.rs)
    let diffs = common::truth_checks::r_clob_diffs();
    assert!(diffs.is_empty(), "{diffs:?}");
    let lines: Vec<_> = truth("11g-cp1251", "truth_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_CLOB")
        .collect();
    assert_eq!(lines.len(), 3);
    let first6 = dump_bytes(&lines[0][3]).unwrap();
    assert_eq!(first6, [0xcf, 0xf0, 0xe8, 0xe2, 0xe5, 0xf2]);
    let first60 = dump_bytes(&lines[1][3]).unwrap();
    assert_eq!(first60.len(), 60);
    for f in RU_FILES {
        let (b, w) = read(&mut ru(f), "R_CLOB", &ReadOptions::default());
        assert!(w.is_empty(), "{f}: {w:?}");
        assert_eq!(b[0].column(1).data_type(), &DataType::LargeUtf8, "{f}");
        let c = texts(&b, 1);
        assert_eq!(c.len(), 3, "{f}");
        let v1 = c[0].as_ref().unwrap();
        assert_eq!(v1, "\u{41f}\u{440}\u{438}\u{432}\u{435}\u{442}", "{f}");
        assert_eq!(cp1251_bytes(v1), first6, "{f}");
        let v2 = c[1].as_ref().unwrap();
        assert_eq!(v2.chars().count(), 6000, "{f}");
        assert_eq!(&cp1251_bytes(v2)[..60], &first60[..], "{f}");
        assert_eq!(c[2], None, "{f}");
        // GETLENGTH of the truth
        assert_eq!((lines[0][2].as_str(), lines[1][2].as_str()), ("6", "6000"));
    }
}

/// (b) R_TEXT (before R_CLOB in the file) and the Cyrillic-named table (after it) equal their truth in all 7
/// files: V, C, VC (database bytes), N (UTF-16), LENGTHB(LV) and the first 60 bytes of LV, D; the Cyrillic
/// table holds (`абв`, 1) and (NULL, 2) where `exp` exported it (3 files; in the other 3 it failed on the
/// table name, ORA-00942, and the table has no rows; `ru_tables_exp_us7ascii` does not list it).
#[test]
fn r_text_and_the_cyrillic_table_around_r_clob() {
    let lines: Vec<_> = truth("11g-cp1251", "truth_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_TEXT")
        .collect();
    assert_eq!(lines.len(), 309);
    for f in RU_FILES {
        let mut d = ru(f);
        let (b, w) = read(&mut d, "R_TEXT", &ReadOptions::default());
        assert!(
            w.iter().all(|w| w.code != WarningCode::ValueNulled),
            "{f}: {w:?}"
        );
        let schema = b[0].schema();
        let names: Vec<_> = schema.fields().iter().map(|x| x.name().clone()).collect();
        assert_eq!(names, ["ID", "V", "C", "VC", "N", "LV", "D"], "{f}");
        let mut r = rows(&b);
        assert_eq!(r.len(), 309, "{f}");
        // the file holds the rows in insertion order; the truth is ordered by ID
        r.sort_by_key(|x| x[0].parse::<u32>().unwrap());
        for (row, l) in r.iter().zip(&lines) {
            assert_eq!(row[0], l[1], "{f}");
            // l: R_TEXT|id|V|C|VC|N|LENGTHB(LV)|ORA_HASH|DUMP(LV,1..60)|D
            for (cell_i, t_i) in [(1usize, 2usize), (2, 3), (3, 4)] {
                let want = dump_bytes(&l[t_i]);
                let got = (row[cell_i] != "<NULL>").then(|| cp1251_bytes(&row[cell_i]));
                assert_eq!(got, want, "{f} id {} col {}", row[0], cell_i);
            }
            let n_want = dump_bytes(&l[5]);
            let n_got = (row[4] != "<NULL>").then(|| {
                row[4]
                    .encode_utf16()
                    .flat_map(u16::to_be_bytes)
                    .collect::<Vec<u8>>()
            });
            assert_eq!(n_got, n_want, "{f} id {} N", row[0]);
            let lv = (row[5] != "<NULL>").then(|| cp1251_bytes(&row[5]));
            assert_eq!(
                lv.as_ref()
                    .map(|x| x.len().to_string())
                    .unwrap_or("NULL".into()),
                l[6],
                "{f} id {} LENGTHB",
                row[0]
            );
            let head = dump_bytes(&l[8]);
            assert_eq!(lv.map(|x| x[..x.len().min(60)].to_vec()), head, "{f}");
            if l[9] == "NULL" {
                assert_eq!(row[6], "<NULL>");
            } else {
                assert!(row[6].starts_with(&l[9]), "{f}: {} {}", row[6], l[9]);
            }
        }
        // the Cyrillic-named table: the section that is neither R_TEXT nor R_CLOB (its name in the client set)
        let ord = d
            .tables()
            .unwrap()
            .iter()
            .filter(|t| t.name != "R_TEXT" && t.name != "R_CLOB")
            .map(|t| t.ordinal)
            .collect::<Vec<_>>();
        // ru_tables_exp_us7ascii (tables=R_TEXT,R_CLOB) has no such table
        if f.starts_with("ru_tables") {
            assert!(ord.is_empty());
            continue;
        }
        assert_eq!(ord.len(), 1, "{f}: the Cyrillic table");
        let ord = ord[0];
        let (b, _) = {
            let mut r = d
                .read_table(TableRef::ByOrdinal(ord), &ReadOptions::default())
                .unwrap();
            let mut out = Vec::new();
            while let Some(x) = r.next_batch().unwrap() {
                out.push(x);
            }
            (out, ())
        };
        let rr = rows(&b);
        // the logs: the Cyrillic table exported 2 rows with a CL8MSWIN1251 or AL32UTF8 client; with a
        // US7ASCII or WE8MSWIN1252 client `exp` failed on the table name (ORA-00942, name `???_???`): no rows
        let exported = ["ru_exp_al32utf8", "ru_exp_cp1251", "ru_exp_direct_cp1251"]
            .iter()
            .any(|p| f.starts_with(&format!("{p}_11g")));
        if exported {
            assert_eq!(
                rr,
                [
                    vec!["\u{430}\u{431}\u{432}".to_string(), "1".to_string()],
                    vec!["<NULL>".to_string(), "2".to_string()]
                ],
                "{f}"
            );
        } else {
            assert!(rr.is_empty(), "{f}: {rr:?}");
        }
    }
}

const LOB2_FILES: [&str; 2] = ["ru_lob2_exp_cp1251_11g.dmp", "ru_lob2_exp_us7ascii_11g.dmp"];

/// The 66-character block of `corpus/sql/15_ru_lob2.sql`: CHR(192) to CHR(255), CHR(168), CHR(184) read as
/// CL8MSWIN1251 (U+0410 to U+044F, U+0401, U+0451). Built from the SQL rule, not from the reader.
fn lob2_block() -> String {
    let mut bytes: Vec<u8> = (192u8..=255).collect();
    bytes.push(168);
    bytes.push(184);
    let (s, _, bad) = encoding_rs::WINDOWS_1251.decode(&bytes);
    assert!(!bad);
    s.into_owned()
}

/// The two texts of R_LOB2 rows 1 and 2 (66 and 20,000 characters; 303 x 66 + 2).
fn lob2_texts() -> (String, String) {
    let blk = lob2_block();
    let long = format!(
        "{}{}",
        blk.repeat(303),
        blk.chars().take(2).collect::<String>()
    );
    (blk, long)
}

fn binaries(b: &[RecordBatch], c: usize) -> Vec<Option<Vec<u8>>> {
    let mut out = Vec::new();
    for b in b {
        let a = b.column(c).as_binary::<i64>();
        for i in 0..a.len() {
            out.push((!a.is_null(i)).then(|| a.value(i).to_vec()));
        }
    }
    out
}

/// Rows of R_LOB2 ordered by ID: (ID, C, N, B).
type Lob2 = (
    Vec<String>,
    Vec<Option<String>>,
    Vec<Option<String>>,
    Vec<Option<Vec<u8>>>,
);

fn lob2_by_id(b: &[RecordBatch]) -> Lob2 {
    let ids: Vec<String> = rows(b).into_iter().map(|r| r[0].clone()).collect();
    (ids, texts(b, 1), texts(b, 2), binaries(b, 3))
}

/// (b) T-6M: R_LOB2 of both client files decodes to the truth (`truth_lob_ru.txt`: GETLENGTH 66 / 20,000 / NULL
/// for C, N and B; head and tail DUMPs of C (database bytes) and N (UTF-16 code units)) with the text rule
/// of `15_ru_lob2.sql`, with default options, without any warning. The expected texts are built from the
/// SQL rule and compared with the truth DUMPs first; the reader's output is never the source.
#[test]
fn r_lob2_decodes_to_the_truth_in_both_client_files() {
    // T-10b: the truth comparison is a function of `common/truth_checks.rs` (also called by t10_truth.rs)
    let diffs = common::truth_checks::r_lob2_diffs();
    assert!(diffs.is_empty(), "{diffs:?}");
    let lines: Vec<_> = truth("11g-cp1251", "truth_lob_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_LOB2")
        .collect();
    assert_eq!(lines.len(), 3);
    let (short, long) = lob2_texts();
    assert_eq!(short.chars().count(), 66);
    assert_eq!(long.chars().count(), 20_000);
    // the expectations agree with the truth file: lengths, heads and tails of C (cp1251 bytes) and N (UTF-16)
    for (l, t) in lines.iter().zip([&short, &long]) {
        let c = cp1251_bytes(t);
        assert_eq!(l[2], t.chars().count().to_string());
        assert_eq!(dump_bytes(&l[5]).unwrap(), c[..4]);
        assert_eq!(dump_bytes(&l[7]).unwrap(), c[c.len() - 4..]);
        let u: Vec<u16> = t.encode_utf16().collect();
        let be = |w: &[u16]| w.iter().flat_map(|x| x.to_be_bytes()).collect::<Vec<u8>>();
        // the truth DUMP of N prints the code units as `hi,lo` decimal-free hex pairs, high byte first
        assert_eq!(dump_bytes(&l[6]).unwrap(), be(&u[..4]));
        assert_eq!(dump_bytes(&l[8]).unwrap(), be(&u[u.len() - 4..]));
    }
    assert_eq!(lines[2][2..5], ["NULL", "NULL", "NULL"]);

    let want = (
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
        vec![Some(short.clone()), Some(long.clone()), None],
        vec![Some(short.clone()), Some(long.clone()), None],
        vec![Some(cp1251_bytes(&short)), Some(cp1251_bytes(&long)), None],
    );
    let r_text_truth: Vec<_> = truth("11g-cp1251", "truth_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_TEXT")
        .collect();
    let mut first = None;
    for f in LOB2_FILES {
        let mut d = ru(f);
        let (b, w) = read(&mut d, "R_LOB2", &ReadOptions::default());
        assert!(w.is_empty(), "{f}: {w:?}");
        assert_eq!(b[0].column(1).data_type(), &DataType::LargeUtf8, "{f}");
        assert_eq!(b[0].column(2).data_type(), &DataType::LargeUtf8, "{f}");
        let mut got = lob2_by_id(&b);
        // the file order is the insertion order; sort by ID all the same
        let mut idx: Vec<usize> = (0..got.0.len()).collect();
        idx.sort_by_key(|i| got.0[*i].parse::<u32>().unwrap());
        got = (
            idx.iter().map(|i| got.0[*i].clone()).collect(),
            idx.iter().map(|i| got.1[*i].clone()).collect(),
            idx.iter().map(|i| got.2[*i].clone()).collect(),
            idx.iter().map(|i| got.3[*i].clone()).collect(),
        );
        assert_eq!(got, want, "{f}");
        let lob2_ordinal = d.table(TableRef::ByName("R_LOB2")).unwrap().ordinal;
        assert!(
            d.warnings().iter().all(|w| w.table != Some(lob2_ordinal)),
            "{f}: {:?}",
            d.warnings()
        );

        // `decode` gives the same batches and still no warning
        let dec = ReadOptions {
            unconfirmed_lob: UnconfirmedLob::Decode,
            ..ReadOptions::default()
        };
        let (b2, w2) = read(&mut d, "R_LOB2", &dec);
        assert!(w2.is_empty(), "{f}: {w2:?}");
        assert_eq!(b.len(), b2.len());
        for (x, y) in b.iter().zip(&b2) {
            assert_eq!(x, y, "{f}");
        }
        if let Some(prev) = &first {
            let prev: &Vec<RecordBatch> = prev;
            assert_eq!(prev.len(), b.len());
            for (x, y) in prev.iter().zip(&b) {
                assert_eq!(x, y, "the two files give equal batches");
            }
        }
        first = Some(b);

        // R_TEXT after R_LOB2: 309 rows equal to truth_ru.txt by ID
        let (b, w) = read(&mut d, "R_TEXT", &ReadOptions::default());
        assert!(
            w.iter().all(|w| w.code != WarningCode::ValueNulled),
            "{f}: {w:?}"
        );
        let mut r = rows(&b);
        assert_eq!(r.len(), 309, "{f}");
        r.sort_by_key(|x| x[0].parse::<u32>().unwrap());
        for (row, l) in r.iter().zip(&r_text_truth) {
            assert_eq!(row[0], l[1], "{f}");
            for (cell_i, t_i) in [(1usize, 2usize), (2, 3), (3, 4)] {
                let got = (row[cell_i] != "<NULL>").then(|| cp1251_bytes(&row[cell_i]));
                assert_eq!(got, dump_bytes(&l[t_i]), "{f} id {} col {}", row[0], cell_i);
            }
        }
    }
}

fn find(hay: &[u8], needle: &[u8]) -> Vec<usize> {
    hay.windows(needle.len())
        .enumerate()
        .filter(|(_, w)| *w == needle)
        .map(|(i, _)| i)
        .collect()
}

/// A LOB value in the long form: `fc ff`, T as 8 bytes, chunks of `chunk` bytes.
fn long_form(value: &[u8], chunk: usize) -> Vec<u8> {
    let mut v = vec![0xfc, 0xff];
    v.extend_from_slice(&(value.len() as u64).to_le_bytes());
    for c in value.chunks(chunk) {
        v.extend_from_slice(&(c.len() as u16).to_le_bytes());
        v.extend_from_slice(c);
    }
    v
}

/// `ru_exp_cp1251_11g.dmp` with the 6,000-byte value of R_CLOB row 2 rewritten in the long form (two chunks
/// of 3,000 bytes). Returns the path and the offset of the value's `fc ff`.
fn synthetic_long_clob() -> (std::path::PathBuf, u64) {
    let mut b = std::fs::read(fixtures().join("11g-cp1251/ru_exp_cp1251_11g.dmp")).unwrap();
    // the u16 6000 = `70 17` and the first bytes of the value (`c7 ce d5 dc e3 ea f1 f8`, truth_ru.txt R_CLOB 2)
    let at = find(
        &b,
        &[0x70, 0x17, 0xc7, 0xce, 0xd5, 0xdc, 0xe3, 0xea, 0xf1, 0xf8],
    );
    assert_eq!(at.len(), 1);
    let at = at[0];
    let value = b[at + 2..at + 2 + 6000].to_vec();
    b.splice(at..at + 2 + 6000, long_form(&value, 3000));
    let p = target_dir("t6l").join("ru_long_clob.dmp");
    std::fs::write(&p, &b).unwrap();
    (p, at as u64)
}

/// T-6M (c): a synthetic long-form value in an R_CLOB column of a CL8MSWIN1251 database (set 171, S = D, both
/// in `LOB_CONFIRMED_DB_SETS`) decodes: the rewritten row 2 (two chunks of 3,000 bytes) reads as the text of
/// the short form without any warning, with the default options and with `unconfirmed_lob="decode"`; the
/// tables after it read.
#[test]
fn a_long_form_clob_of_database_set_171_decodes() {
    let (p, at) = synthetic_long_clob();
    let mut d = open_path(&p, Options::default());
    assert_eq!(d.info().db_charset_id, 171);
    let original = texts(
        &read(
            &mut ru("ru_exp_cp1251_11g.dmp"),
            "R_CLOB",
            &ReadOptions::default(),
        )
        .0,
        1,
    );
    // the walk sees a long value of 6,000 bytes in two chunks at `at`
    let t = d.table(TableRef::ByName("R_CLOB")).unwrap().clone();
    let lob = t.columns[1].lob.as_ref().unwrap();
    assert_eq!((lob.long, lob.largest), (1, 6000));
    assert!(at > 0);

    for o in [
        ReadOptions::default(),
        ReadOptions {
            unconfirmed_lob: UnconfirmedLob::Decode,
            ..ReadOptions::default()
        },
    ] {
        let (b, w) = read(&mut d, "R_CLOB", &o);
        assert_eq!(texts(&b, 1), original);
        assert!(w.is_empty(), "{w:?}");
    }
    assert_eq!(
        row_count(&read(&mut d, "R_TEXT", &ReadOptions::default()).0),
        309
    );
}

/// T-6M (c): a synthetic CL8MSWIN1251 header: `lob_exp_18c.dmp` with the database set of the header (+2 of line
/// 8 of the header, [H4]) changed from 873 to 171. Set 171 is confirmed, so every NCLOB value of T_LOB2 (also
/// the empty one) now decodes to the text of the unpatched file without a warning; the long-form CLOB of T_LOB
/// (row 5, 200,000 bytes, set S = 873 differs from D = 171) stays `E-LOB-UNCONFIRMED`, the short CLOB values
/// and the BLOBs read. With `null` the value is NULL with a counted `W-VALUE-NULLED`; with `decode` the text
/// equals that of the AL32UTF8 file (UTF-16 BE), with `W-LOB-UNTESTED` for the long value only.
#[test]
fn an_nclob_in_a_single_byte_header_is_unconfirmed() {
    let mut b = std::fs::read(fixtures().join("18c/lob_exp_18c.dmp")).unwrap();
    // header bytes 0x30..0x36: client 03 69, database 03 69, national 07 d0
    assert_eq!(b[0x30..0x36], [0x03, 0x69, 0x03, 0x69, 0x07, 0xd0]);
    b[0x32] = 0x00;
    b[0x33] = 0xab;
    let p = target_dir("t6l").join("lob_single_byte_header.dmp");
    std::fs::write(&p, &b).unwrap();
    let mut d = open_path(&p, Options::default());
    assert_eq!(d.info().db_charset_id, 171);
    let mut orig = open_fixture("18c/lob_exp_18c.dmp");

    // T_LOB2: A (CLOB, short) and N (NCLOB) read, equal to the unpatched file, no warning
    let (x, w) = read(&mut d, "T_LOB2", &ReadOptions::default());
    let (y, _) = read(&mut orig, "T_LOB2", &ReadOptions::default());
    // only T-9's W-CHARSET-MISMATCH for the form-1 CLOB A (S = 873 != D = 171); nothing about the LOBs
    assert!(
        w.iter().all(|w| w.code == WarningCode::CharsetMismatch),
        "{w:?}"
    );
    assert_eq!(texts(&x, 3), texts(&y, 3));
    assert!(texts(&x, 3).iter().any(Option::is_some));
    for (a, b) in x.iter().zip(&y) {
        for (ca, cb) in a.columns().iter().zip(b.columns()) {
            assert_eq!(ca.to_data(), cb.to_data());
        }
    }
    // T_LOB: the long CLOB value of row 5 (C, column 2), S = 873 != D = 171
    let e = try_read(&mut d, "T_LOB", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::LobUnconfirmed);
    assert_eq!(
        (e.row, e.column, e.length),
        (Some(5), Some(2), Some(200_000))
    );
    let msg = e.to_string();
    assert!(!msg.contains("R-038"), "{msg}");

    let null = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let (bb, w) = read(&mut d, "T_LOB2", &null);
    assert_eq!(texts(&bb, 1), [Some("abc".to_string()), None]);
    assert_eq!(texts(&bb, 3), texts(&y, 3));
    assert!(
        w.iter().all(|w| w.code == WarningCode::CharsetMismatch),
        "{w:?}"
    );
    let (bb, w) = read(&mut d, "T_LOB", &null);
    let c = texts(&bb, 1);
    assert_eq!(c[3].as_deref(), Some("m".repeat(3000).as_str()));
    assert_eq!(c[4], None);
    assert!(w
        .iter()
        .any(|w| w.code == WarningCode::ValueNulled && w.count == Some(1)));

    let dec = ReadOptions {
        unconfirmed_lob: UnconfirmedLob::Decode,
        ..ReadOptions::default()
    };
    for t in ["T_LOB", "T_LOB2"] {
        let (x, w) = read(&mut d, t, &dec);
        let (y, _) = read(&mut orig, t, &ReadOptions::default());
        for (a, b) in x.iter().zip(&y) {
            for (ca, cb) in a.columns().iter().zip(b.columns()) {
                assert_eq!(ca.to_data(), cb.to_data(), "{t}");
            }
        }
        let u: Vec<_> = w
            .iter()
            .filter(|w| w.code == WarningCode::LobUntested)
            .collect();
        let cols: Vec<_> = u
            .iter()
            .map(|w| (w.column.unwrap(), w.count.unwrap()))
            .collect();
        // T_LOB: C has one long value; T_LOB2: nothing is unconfirmed any more
        assert_eq!(cols, if t == "T_LOB" { vec![(2, 1)] } else { vec![] });
    }
    // BLOBs and a table without a LOB column are not touched
    assert_eq!(
        row_count(&read(&mut d, "T_FRAME", &ReadOptions::default()).0),
        7
    );
}

/// `ru_lob2_exp_cp1251_11g.dmp` with the header database set @0x36 (`00 ab`) set to `db_low` and, if given, the
/// low byte of the CLOB descriptor set @0x24e3 (`ab 00`) set to `clob_low`; written below `target/`.
/// Offsets from `exp.md` 14.6 session 10 / the plan (T-6M (d)); asserted against the bytes first.
fn lob2_patched(name: &str, db_low: u8, clob_low: Option<u8>) -> std::path::PathBuf {
    let mut b = std::fs::read(fixtures().join("11g-cp1251/ru_lob2_exp_cp1251_11g.dmp")).unwrap();
    assert_eq!(b[0x35..0x37], [0x00, 0xab]);
    assert_eq!(b[0x24e3..0x24e5], [0xab, 0x00]);
    b[0x36] = db_low;
    if let Some(c) = clob_low {
        b[0x24e3] = c;
    }
    let p = target_dir("t6m").join(name);
    std::fs::write(&p, &b).unwrap();
    p
}

/// T-6M (d): the stop with another single-byte set. A copy of `ru_lob2_exp_cp1251_11g.dmp` with the database
/// set 178 and the CLOB descriptor set 178: C row 1 (short) reads, N row 1 (@0x2790, stored length 132) is
/// refused with `E-LOB-UNCONFIRMED`; `null` gives NULLs with counted `W-VALUE-NULLED` (C: row 2 @0x29c4, one;
/// N: two), B intact, R_TEXT still reads 309 rows; `decode` reads C row 2 as windows-1252 and N as the Cyrillic
/// texts, with `W-LOB-UNTESTED` (C one, N two). The same first error with the header patched to 1 (descriptor
/// left at 171) and to 31; the `db_charset` override does not lift it.
#[test]
fn another_single_byte_database_set_keeps_the_stop() {
    let (short, long) = lob2_texts();
    let w1252 = |s: &str| {
        let bytes = cp1251_bytes(s);
        let (t, _, bad) = encoding_rs::WINDOWS_1252.decode(&bytes);
        assert!(!bad);
        t.into_owned()
    };
    let b_short = Some(cp1251_bytes(&short));
    let b_long = Some(cp1251_bytes(&long));

    let p = lob2_patched("lob2_178.dmp", 0xb2, Some(0xb2));
    let mut d = open_path(&p, Options::default());
    assert_eq!(d.info().db_charset_id, 178);

    let e = try_read(&mut d, "R_LOB2", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::LobUnconfirmed);
    assert_eq!(
        (e.row, e.column, e.part, e.offset, e.length),
        (Some(1), Some(3), Some(1), Some(0x2790), Some(132))
    );
    let msg = e.to_string();
    assert!(
        msg.contains("178") && msg.contains("873, 171") && msg.contains("unconfirmed_lob"),
        "{msg}"
    );
    assert!(!msg.contains("R-038"), "{msg}");

    let null = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let (b, w) = read(&mut d, "R_LOB2", &null);
    assert_eq!(
        texts(&b, 1),
        [Some(w1252(&short)), None, None],
        "C: the windows-1252 reading of the 66 bytes"
    );
    assert_eq!(texts(&b, 2), [None, None, None]);
    assert_eq!(binaries(&b, 3), [b_short.clone(), b_long.clone(), None]);
    let nulled = |code: WarningCode, col: u32| {
        w.iter()
            .filter(|w| w.code == code && w.column == Some(col))
            .map(|w| (w.count, w.row, w.offset))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        nulled(WarningCode::ValueNulled, 2),
        [(Some(1), Some(2), Some(0x29c4))]
    );
    let n = nulled(WarningCode::ValueNulled, 3);
    assert_eq!(n.len(), 1);
    assert_eq!(n[0].0, Some(2));
    let (t, w) = read(&mut d, "R_TEXT", &null);
    assert_eq!(row_count(&t), 309);
    // the 171 columns of R_TEXT warn W-CHARSET-MISMATCH (T-9); asserted, not compared
    assert!(w.iter().any(|w| w.code == WarningCode::CharsetMismatch));

    let dec = ReadOptions {
        unconfirmed_lob: UnconfirmedLob::Decode,
        ..ReadOptions::default()
    };
    let (b, w) = read(&mut d, "R_LOB2", &dec);
    assert_eq!(
        texts(&b, 1),
        [Some(w1252(&short)), Some(w1252(&long)), None]
    );
    assert_eq!(
        texts(&b, 2),
        [Some(short.clone()), Some(long.clone()), None]
    );
    assert_eq!(binaries(&b, 3), [b_short, b_long, None]);
    for (col, count) in [(2u32, 1u64), (3, 2)] {
        let u: Vec<_> = w
            .iter()
            .filter(|w| w.code == WarningCode::LobUntested && w.column == Some(col))
            .collect();
        assert_eq!(u.len(), 1, "column {col}");
        assert_eq!(u[0].count, Some(count), "column {col}");
    }
    assert!(w.iter().all(|w| w.code != WarningCode::ValueNulled));

    // the override changes the decoder, not the decision
    let o = Options {
        db_charset: Some("CL8MSWIN1251".to_string()),
        ..Options::default()
    };
    let mut d = open_path(&p, o);
    let e = try_read(&mut d, "R_LOB2", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::LobUnconfirmed);
    assert_eq!(
        (e.row, e.column, e.offset, e.length),
        (Some(1), Some(3), Some(0x2790), Some(132))
    );

    // header 1 (descriptor left at 171) and header 31 (outside the five): the same first error
    for (name, low) in [("lob2_h1.dmp", 0x01u8), ("lob2_h31.dmp", 0x1f)] {
        let p = lob2_patched(name, low, None);
        let mut d = open_path(&p, Options::default());
        assert_eq!(d.info().db_charset_id, u16::from(low), "{name}");
        let e = try_read(&mut d, "R_LOB2", &ReadOptions::default()).unwrap_err();
        assert_eq!(e.code, ErrorCode::LobUnconfirmed, "{name}");
        assert_eq!(
            (e.row, e.column, e.offset, e.length),
            (Some(1), Some(3), Some(0x2790), Some(132)),
            "{name}"
        );
    }
}

/// ADR-32 item 5: until R-042 there is no partitioned LOB fixture. A synthetic two-part section built from
/// `lob_exp_18c`: the part of T_LOB (its `INSERT INTO` line to the `ff ff` LF) is copied behind a `PARTITION`
/// line and closed by `ENDPARTITION`. Both parts read into one schema with equal values (two batches, a
/// batch never mixes parts) and `ColumnInfo.lob` sums the two.
#[test]
fn a_partitioned_lob_table_reads_both_parts() {
    let b = std::fs::read(fixtures().join("18c/lob_exp_18c.dmp")).unwrap();
    let ins = find(&b, b"INSERT INTO \"T_LOB\" ")[0];
    let end = find(&b, b"\nTABLE \"T_LOB2\"\n")[0] + 1;
    let part = b[ins..end].to_vec();
    assert_eq!(&part[part.len() - 3..], [0xff, 0xff, 0x0a]);
    let mut out = b[..ins].to_vec();
    out.extend_from_slice(b"PARTITION \"P1\"\n");
    out.extend_from_slice(&part);
    out.extend_from_slice(b"PARTITION \"P2\"\n");
    out.extend_from_slice(&part);
    out.extend_from_slice(b"ENDPARTITION\n");
    out.extend_from_slice(&b[end..]);
    let p = target_dir("t6l").join("lob_partitioned.dmp");
    std::fs::write(&p, &out).unwrap();

    let mut d = open_path(&p, Options::default());
    let mut single = open_fixture("18c/lob_exp_18c.dmp");
    let t = d.table(TableRef::ByName("T_LOB")).unwrap().clone();
    assert_eq!(t.parts.len(), 2);
    let s = single.table(TableRef::ByName("T_LOB")).unwrap().clone();
    for (c2, c1) in t.columns.iter().zip(&s.columns) {
        match (&c2.lob, &c1.lob) {
            (Some(a), Some(b)) => {
                assert_eq!(a.values, 2 * b.values);
                assert_eq!(a.long, 2 * b.long);
                assert_eq!(a.short, 2 * b.short);
                assert_eq!(a.empty, 2 * b.empty);
                assert_eq!(a.stored_bytes, 2 * b.stored_bytes);
                assert_eq!(a.largest, b.largest);
            }
            (None, None) => {}
            other => panic!("{other:?}"),
        }
    }
    let (two, w) = read(&mut d, "T_LOB", &ReadOptions::default());
    let (one, _) = read(&mut single, "T_LOB", &ReadOptions::default());
    assert!(w.is_empty(), "{w:?}");
    assert_eq!(two.len(), 2);
    assert_eq!(one.len(), 1);
    for part in &two {
        assert_eq!(part.schema(), one[0].schema());
        for (a, b) in part.columns().iter().zip(one[0].columns()) {
            assert_eq!(a.to_data(), b.to_data());
        }
    }
}
