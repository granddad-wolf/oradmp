//! Task T-6L (part 1): LOB values end to end on the AL32UTF8 fixtures (`lob_exp*`, `lob3_exp`, `nchar2_exp`):
//! done-when items (a), (c), (d), (f) of the plan.
//!
//! Expected values come from the plan, the truth files `truth_lob.txt`, `truth_frame.txt`, `truth_lob3.txt`,
//! `truth_nchar2.txt` of `corpus/fixtures/{18c,11g}`, and the SQL of `corpus/sql/01b_optional_tables.sql`
//! (T_LOB values) and `13_optional.sql` (T_LOB2 values). Byte lengths of the long form are those of the
//! Analyst's tools (`tools/lob_check.py`, `tools/lob3_switch.py`): the rules file [L4] gives 18c CLOB / BLOB
//! chunks of 16,120 / 8,060 bytes, 11g 16,264 / 8,132 (observed in this AL32UTF8 fixture, not a rule (ADR-37)).
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
use oradmp_core::charset::{Charset, PieceDecoder};
use oradmp_core::dump::TableRef;
use oradmp_core::exp::rows::{LobAction, LobForm, RowSink};
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::{ErrorCode, WarningCode};

const VERSIONS: [&str; 2] = ["18c", "11g"];

fn opts() -> ReadOptions {
    ReadOptions::default()
}

/// The 100,000-character text of T_LOB row 5: `chunk001` ... `chunk100`, each padded with `.` to 1,000
/// characters (`01b_optional_tables.sql`, `RPAD('chunk' || LPAD(i, 3, '0'), 1000, '.')`).
fn text_100k() -> String {
    (1..=100)
        .map(|i| format!("{:.<1000}", format!("chunk{i:03}")))
        .collect()
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

fn blobs(b: &[RecordBatch], c: usize) -> Vec<Option<Vec<u8>>> {
    let mut out = Vec::new();
    for b in b {
        let a = b.column(c).as_binary::<i64>();
        for i in 0..a.len() {
            out.push((!a.is_null(i)).then(|| a.value(i).to_vec()));
        }
    }
    out
}

fn data_equal(a: &[RecordBatch], b: &[RecordBatch]) {
    assert_eq!(a.len(), b.len(), "number of batches");
    for (x, y) in a.iter().zip(b) {
        assert_eq!(x.schema(), y.schema());
        for (cx, cy) in x.columns().iter().zip(y.columns()) {
            assert_eq!(cx.to_data(), cy.to_data());
        }
    }
}

/// (a) T_LOB: C and B against the SQL and `truth_lob.txt` (characters for C, bytes for B, the first 30
/// bytes of B in hex), both versions; the schema is LargeUtf8 / LargeBinary.
#[test]
fn t_lob_equals_the_sql_and_the_truth_in_both_versions() {
    // T-10b: the truth comparison is a function of `common/truth_checks.rs` (also called by t10_truth.rs)
    let diffs = common::truth_checks::lob_diffs();
    assert!(diffs.is_empty(), "{diffs:?}");
    let text = text_100k();
    for v in VERSIONS {
        let (b, w) = read_default(&format!("{v}/lob_exp_{v}.dmp"), "T_LOB");
        assert!(w.is_empty(), "{v}: {w:?}");
        let schema = b[0].schema();
        let types: Vec<_> = schema
            .fields()
            .iter()
            .map(|f| f.data_type().clone())
            .collect();
        assert_eq!(
            types,
            [DataType::Int64, DataType::LargeUtf8, DataType::LargeBinary]
        );
        let c = texts(&b, 1);
        assert_eq!(c.len(), 5);
        assert_eq!(c[0].as_deref(), Some("hello lob"));
        assert_eq!(c[1], None);
        assert_eq!(c[2].as_deref(), Some(""));
        assert_eq!(c[3].as_deref(), Some("m".repeat(3000).as_str()));
        assert_eq!(c[4].as_deref(), Some(text.as_str()));
        let bl = blobs(&b, 2);
        assert_eq!(bl[0].as_deref(), Some(&b"Hello blob"[..]));
        assert_eq!(bl[1], None);
        assert_eq!(bl[2].as_deref(), Some(&b""[..]));
        assert_eq!(bl[3].as_deref(), Some(&[0xab; 1000][..]));
        assert_eq!(bl[4].as_deref(), Some(&vec![0xcd; 100_000][..]));

        // the truth: |id|GETLENGTH(c)|GETLENGTH(b)|hex of the first 30 bytes of b
        let lines: Vec<_> = truth(v, "truth_lob.txt")
            .into_iter()
            .filter(|l| l[0] == "T_LOB")
            .collect();
        assert_eq!(lines.len(), 5);
        for (i, l) in lines.iter().enumerate() {
            let clen = c[i]
                .as_ref()
                .map_or("NULL".to_string(), |s| s.chars().count().to_string());
            let blen = bl[i]
                .as_ref()
                .map_or("NULL".to_string(), |s| s.len().to_string());
            assert_eq!(
                (l[2].clone(), l[3].clone()),
                (clen, blen),
                "{v} row {}",
                i + 1
            );
            let head = match &bl[i] {
                Some(x) if !x.is_empty() => hex(&x[..x.len().min(30)]).to_uppercase(),
                _ => "NULL".to_string(),
            };
            assert_eq!(l[4], head, "{v} row {}", i + 1);
        }
    }
}

/// (a) T_LOB2 (A CLOB, V, N NCLOB, B BLOB) against `13_optional.sql` and `truth_lob.txt`; the NCLOB of row 2
/// is empty, not NULL.
#[test]
fn t_lob2_equals_the_sql_and_the_truth() {
    // T-10b: the truth comparison is a function of `common/truth_checks.rs` (also called by t10_truth.rs)
    let diffs = common::truth_checks::lob_diffs();
    assert!(diffs.is_empty(), "{diffs:?}");
    for v in VERSIONS {
        let (b, w) = read_default(&format!("{v}/lob_exp_{v}.dmp"), "T_LOB2");
        assert!(w.is_empty(), "{v}: {w:?}");
        let a = texts(&b, 1);
        let n = texts(&b, 3);
        let bl = blobs(&b, 4);
        assert_eq!(col_strings(&b, 2), ["x", "y"]);
        assert_eq!(a, [Some("abc".to_string()), None]);
        assert_eq!(n, [Some("abc".to_string()), Some(String::new())]);
        assert_eq!(bl, [Some(vec![0x00, 0xff]), None]);
        // truth: |id|len(a)|v|len(n)|len(b)
        let lines: Vec<_> = truth(v, "truth_lob.txt")
            .into_iter()
            .filter(|l| l[0] == "T_LOB2")
            .collect();
        for (i, l) in lines.iter().enumerate() {
            let len = |o: &Option<String>| {
                o.as_ref()
                    .map_or("NULL".to_string(), |s| s.chars().count().to_string())
            };
            assert_eq!(l[2], len(&a[i]));
            assert_eq!(l[4], len(&n[i]));
            assert_eq!(
                l[5],
                bl[i]
                    .as_ref()
                    .map_or("NULL".to_string(), |x| x.len().to_string())
            );
        }
    }
}

fn col_strings(b: &[RecordBatch], c: usize) -> Vec<String> {
    rows(b).into_iter().map(|r| r[c].clone()).collect()
}

/// (a) Both files of each version (conventional and `direct=y`; LOB tables stay on the conventional path
/// [L6]) give equal batches for T_LOB, T_LOB2 and the T_FRAME after them; T_FRAME equals `truth_frame.txt`.
#[test]
fn conventional_and_direct_files_give_equal_batches_and_t_frame_follows() {
    for v in VERSIONS {
        let mut conv = open_fixture(&format!("{v}/lob_exp_{v}.dmp"));
        let mut direct = open_fixture(&format!("{v}/lob_exp_direct_{v}.dmp"));
        for t in ["T_LOB", "T_LOB2", "T_FRAME"] {
            let (a, wa) = read(&mut conv, t, &opts());
            let (b, wb) = read(&mut direct, t, &opts());
            data_equal(&a, &b);
            assert_eq!((wa.len(), wb.len()), (0, 0), "{v} {t}");
        }
        let (f, _) = read(&mut conv, "T_FRAME", &opts());
        let expected = truth(v, "truth_frame.txt");
        assert_eq!(rows(&f), expected, "{v}: T_FRAME");
    }
}

/// A sink that keeps the raw bytes of the LOB values it is given (and the chunk lengths).
type Seen = (u32, LobForm, u64, Vec<usize>, Vec<u8>);

#[derive(Default)]
struct Collect {
    values: Vec<Seen>,
    ends: usize,
}

impl RowSink for Collect {
    fn row_start(&mut self, _row: u64) {}
    fn null(&mut self, _c: u32, _o: u64) -> oradmp_core::Result<()> {
        Ok(())
    }
    fn value(&mut self, _c: u32, _o: u64, _b: &[u8]) -> oradmp_core::Result<()> {
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
    fn lob_start(
        &mut self,
        column: u32,
        _offset: u64,
        form: LobForm,
        total: u64,
    ) -> oradmp_core::Result<LobAction> {
        self.values
            .push((column, form, total, Vec::new(), Vec::new()));
        Ok(LobAction::Read)
    }
    fn lob_chunk(&mut self, _c: u32, _o: u64, bytes: &[u8]) -> oradmp_core::Result<()> {
        let v = self.values.last_mut().unwrap();
        v.3.push(bytes.len());
        v.4.extend_from_slice(bytes);
        Ok(())
    }
    fn lob_end(&mut self, _c: u32) -> oradmp_core::Result<()> {
        self.ends += 1;
        Ok(())
    }
    fn row_end(&mut self) -> oradmp_core::Result<()> {
        Ok(())
    }
}

fn decode_in_chunks(set: Charset, bytes: &[u8], size: usize) -> oradmp_core::Result<String> {
    let mut d = PieceDecoder::new(set);
    let mut out = String::new();
    for c in bytes.chunks(size) {
        d.feed(c, false, &mut |t| {
            out.push_str(t);
            Ok(())
        })?;
    }
    d.feed(&[], true, &mut |t| {
        out.push_str(t);
        Ok(())
    })?;
    Ok(out)
}

/// (c) The 200,000-byte value of T_LOB row 5 (column C) is fed to the UTF-16 BE decoder in the chunks of the
/// file (18c 12 x 16,120 + 6,560 bytes ... as the file stores them; 11g 12 x 16,264 + 4,832 [L4]) and in
/// chunks of 1, 3 and 16,119 bytes, and gives the same text; the file's chunk lengths are checked.
#[test]
fn the_long_value_decodes_the_same_in_any_chunking() {
    let text = text_100k();
    for (v, first, last) in [("18c", 16_120usize, 6_560usize), ("11g", 16_264, 4_832)] {
        let mut d = open_fixture(&format!("{v}/lob_exp_{v}.dmp"));
        let mut rows_ = d.rows(TableRef::ByName("T_LOB")).unwrap();
        let mut sink = Collect::default();
        while rows_.next_row_with(&mut sink).unwrap().is_some() {}
        // T_LOB: 5 rows, k = 2, 0, 2, 2, 2: C and B of rows 1, 3, 4, 5
        // (the chunk counts and sizes below are observed in this AL32UTF8 fixture, not a rule (ADR-37))
        assert_eq!(sink.values.len(), 8);
        assert_eq!(sink.ends, 8);
        let (col, form, total, chunks, bytes) = &sink.values[6];
        assert_eq!((*col, *form, *total), (2, LobForm::Long, 200_000));
        assert_eq!(chunks.len(), 13);
        assert!(chunks[..12].iter().all(|&c| c == first), "{v}");
        assert_eq!(chunks[12], last, "{v}");
        assert_eq!(bytes.len(), 200_000);
        // the file's own chunking, then 1, 3 and 16,119 bytes
        let mut dec = PieceDecoder::new(Charset::Utf16Be);
        let mut out = String::new();
        let mut at = 0;
        for &c in chunks {
            dec.feed(&bytes[at..at + c], false, &mut |t| {
                out.push_str(t);
                Ok(())
            })
            .unwrap();
            at += c;
        }
        dec.feed(&[], true, &mut |t| {
            out.push_str(t);
            Ok(())
        })
        .unwrap();
        assert_eq!(out, text);
        for size in [1, 3, 16_119] {
            assert_eq!(
                decode_in_chunks(Charset::Utf16Be, bytes, size).unwrap(),
                text,
                "{v} chunks of {size}"
            );
        }
        // the BLOB of row 5 is the next value: 100,000 bytes in chunks of the BLOB size
        let (col, form, total, chunks, _) = &sink.values[7];
        assert_eq!((*col, *form, *total), (3, LobForm::Long, 100_000));
        assert_eq!(chunks.len(), 13);
    }
}

/// (c) A surrogate pair split between two chunks decodes to one character; an odd total and an unpaired
/// surrogate are `E-BAD-TEXT` (the vectors of architecture 5.6 step 6).
#[test]
fn surrogate_pairs_split_between_chunks_and_malformed_utf16() {
    // `truth_lob3.txt` row 20 C, 22 bytes -> 16 UTF-8 bytes `d0 b4 d0 b6 20 43 61 66 c3 a9 20 61 f0 9f 98 80`
    let v: Vec<u8> = vec![
        0x04, 0x34, 0x04, 0x36, 0x00, 0x20, 0x00, 0x43, 0x00, 0x61, 0x00, 0x66, 0x00, 0xe9, 0x00,
        0x20, 0x00, 0x61, 0xd8, 0x3d, 0xde, 0x00,
    ];
    let want = "\u{434}\u{436} Caf\u{e9} a\u{1f600}";
    assert_eq!(want.len(), 16);
    assert_eq!(decode_in_chunks(Charset::Utf16Be, &v, 22).unwrap(), want);
    // split between `d8 3d` | `de 00`
    let mut d = PieceDecoder::new(Charset::Utf16Be);
    let mut out = String::new();
    for p in [&v[..20], &v[20..]] {
        d.feed(p, false, &mut |t| {
            out.push_str(t);
            Ok(())
        })
        .unwrap();
    }
    d.feed(&[], true, &mut |t| {
        out.push_str(t);
        Ok(())
    })
    .unwrap();
    assert_eq!(out, want);
    // split between `d8` | `3d de 00`
    assert_eq!(
        {
            let mut d = PieceDecoder::new(Charset::Utf16Be);
            let mut out = String::new();
            for p in [&v[..19], &v[19..]] {
                d.feed(p, false, &mut |t| {
                    out.push_str(t);
                    Ok(())
                })
                .unwrap();
            }
            d.feed(&[], true, &mut |t| {
                out.push_str(t);
                Ok(())
            })
            .unwrap();
            out
        },
        want
    );
    // negative vectors: `d8 3d` alone, `de 00 d8 3d`, `00 61 d8` (odd), in every chunking
    for bad in [
        &[0xd8u8, 0x3d][..],
        &[0xde, 0x00, 0xd8, 0x3d],
        &[0x00, 0x61, 0xd8],
    ] {
        for size in [1, 2, 3, 8] {
            let e = decode_in_chunks(Charset::Utf16Be, bad, size).unwrap_err();
            assert_eq!(e.code, ErrorCode::BadText, "{bad:?} in chunks of {size}");
        }
    }
}

/// (d) `batch_bytes` 4,000 on T_LOB gives batches of 4 and 1 rows, 1 gives 5 batches of 1 row, the default
/// one batch (architecture 5.6 step 8: rows add 27, 8, 8, 4,008 and 200,008 bytes).
#[test]
fn batch_bytes_with_large_values() {
    for v in VERSIONS {
        let mut d = open_fixture(&format!("{v}/lob_exp_{v}.dmp"));
        let sizes = |d: &mut oradmp_core::dump::Dump, cap: u64| {
            let o = ReadOptions {
                batch_bytes: cap,
                ..opts()
            };
            read(d, "T_LOB", &o)
                .0
                .iter()
                .map(RecordBatch::num_rows)
                .collect::<Vec<_>>()
        };
        assert_eq!(sizes(&mut d, 4_000), [4, 1], "{v}");
        assert_eq!(sizes(&mut d, 1), [1, 1, 1, 1, 1], "{v}");
        assert_eq!(sizes(&mut d, 64 << 20), [5], "{v}");
        // the same values in any batching
        let (one, _) = read(&mut d, "T_LOB", &opts());
        let (many, _) = read(
            &mut d,
            "T_LOB",
            &ReadOptions {
                batch_bytes: 4_000,
                ..opts()
            },
        );
        assert_eq!(texts(&one, 1), texts(&many, 1));
        assert_eq!(blobs(&one, 2), blobs(&many, 2));
    }
}

/// (d) `max_lob_bytes` 150,000 gives `E-LOB-TOO-LARGE` at row 5 of T_LOB (C has 200,000 stored bytes, B has
/// 100,000 and is read); with `on_value_error="null"` row 5 has C NULL with `W-VALUE-NULLED` and B intact,
/// while T_LOB2 and T_FRAME still read.
#[test]
fn max_lob_bytes_refuses_the_big_clob_only() {
    for v in VERSIONS {
        let mut d = open_fixture(&format!("{v}/lob_exp_{v}.dmp"));
        let o = ReadOptions {
            max_lob_bytes: 150_000,
            ..opts()
        };
        let e = try_read(&mut d, "T_LOB", &o).unwrap_err();
        assert_eq!(e.code, ErrorCode::LobTooLarge);
        assert_eq!(
            (e.row, e.column, e.part, e.length),
            (Some(5), Some(2), Some(1), Some(200_000))
        );
        assert!(e.offset.is_some());
        let msg = e.to_string();
        assert!(
            msg.contains("E-LOB-TOO-LARGE") && msg.contains("row 5"),
            "{msg}"
        );

        let o = ReadOptions {
            on_value_error: OnValueError::Null,
            ..o
        };
        let (b, w) = read(&mut d, "T_LOB", &o);
        let c = texts(&b, 1);
        assert_eq!(c[3].as_deref(), Some("m".repeat(3000).as_str()));
        assert_eq!(c[4], None);
        assert_eq!(blobs(&b, 2)[4].as_deref(), Some(&vec![0xcd; 100_000][..]));
        let n: Vec<_> = w
            .iter()
            .filter(|w| w.code == WarningCode::ValueNulled)
            .collect();
        assert_eq!(n.len(), 1);
        assert_eq!(
            (n[0].count, n[0].row, n[0].column, n[0].part),
            (Some(1), Some(5), Some(2), Some(1))
        );
        // the tables after it read
        assert_eq!(row_count(&read(&mut d, "T_LOB2", &o).0), 2);
        assert_eq!(row_count(&read(&mut d, "T_FRAME", &o).0), 7);
    }
}

/// (d) `columns=["ID"]` reads T_LOB without holding any LOB byte: no LOB value reaches a builder (the reader
/// counts the stored bytes it lends; every present value, 8 of them, is passed by its lengths), and the
/// batch holds only the 5 IDs. (The workspace forbids `unsafe`, so a counting allocator is not available;
/// the counter is the proof that nothing was copied.)
#[test]
fn an_unselected_lob_column_is_never_held() {
    for v in VERSIONS {
        let mut d = open_fixture(&format!("{v}/lob_exp_{v}.dmp"));
        let o = ReadOptions {
            columns: Some(vec!["ID".to_string()]),
            ..opts()
        };
        let mut r = d.read_table(TableRef::ByName("T_LOB"), &o).unwrap();
        let mut n = 0;
        let mut mem = 0;
        while let Some(b) = r.next_batch().unwrap() {
            n += b.num_rows();
            mem += b.get_array_memory_size();
        }
        assert_eq!(n, 5);
        assert!(mem < 64 * 1024, "{mem}");
        assert_eq!(r.lob_bytes_read(), 0);
        assert_eq!(r.lob_values_skipped(), 8);
        // with both columns the same reader lends every stored byte of the present values
        let mut r = d.read_table(TableRef::ByName("T_LOB"), &opts()).unwrap();
        while r.next_batch().unwrap().is_some() {}
        // C: 18 + 0 + 6,000 + 200,000 bytes; B: 10 + 0 + 1,000 + 100,000 bytes
        assert_eq!(
            r.lob_bytes_read(),
            18 + 6_000 + 200_000 + 10 + 1_000 + 100_000
        );
        assert_eq!(r.lob_values_skipped(), 0);
    }
}

/// (f) `lob3_exp_<v>` rows 1 to 11: B = S bytes of `5a`, C = S characters `x`, for S of `truth_lob3.txt`,
/// short up to one chunk and long beyond (architecture 5.6 step 4); row 20: C and N = the 16 UTF-8 bytes
/// `d0 b4 d0 b6 20 43 61 66 c3 a9 20 61 f0 9f 98 80` (a non-BMP character as a surrogate pair).
#[test]
fn lob3_values_around_the_switch_and_the_surrogate_pair() {
    // T-10b: the truth comparison is a function of `common/truth_checks.rs` (also called by t10_truth.rs)
    let diffs = common::truth_checks::lob3_diffs();
    assert!(diffs.is_empty(), "{diffs:?}");
    for v in VERSIONS {
        let (b, w) = read_default(&format!("{v}/lob3_exp_{v}.dmp"), "T_LOB3");
        assert!(w.is_empty(), "{v}: {w:?}");
        let ids = col_strings(&b, 0);
        let lines: Vec<_> = truth(v, "truth_lob3.txt")
            .into_iter()
            .filter(|l| l[0] == "T_LOB3")
            .collect();
        assert_eq!(lines.len(), 12);
        let schema = b[0].schema();
        let pos = |n: &str| schema.index_of(n).unwrap();
        let (bc, cc, nc) = (pos("B"), pos("C"), pos("N"));
        let bl = blobs(&b, bc);
        let cl = texts(&b, cc);
        let nl = texts(&b, nc);
        assert_eq!(ids.len(), 12);
        // truth: T_LOB3|id|GETLENGTH(c)|GETLENGTH(b) or NULL|GETLENGTH(n) or NULL|DUMP
        for l in lines.iter().filter(|l| l[1] != "20") {
            let id = &l[1];
            let i = ids.iter().position(|x| x == id).unwrap();
            let s: usize = l[2].parse().unwrap();
            assert_eq!(l[3].parse::<usize>().unwrap(), s);
            assert_eq!(bl[i].as_deref(), Some(&vec![0x5a; s][..]), "{v} row {id} B");
            assert_eq!(
                cl[i].as_deref(),
                Some("x".repeat(s).as_str()),
                "{v} row {id} C"
            );
            assert_eq!(nl[i], None, "{v} row {id} N");
        }
        let i = ids.iter().position(|x| x == "20").unwrap();
        let want = "\u{434}\u{436} Caf\u{e9} a\u{1f600}";
        assert_eq!(bl[i], None);
        assert_eq!(cl[i].as_deref(), Some(want));
        assert_eq!(nl[i].as_deref(), Some(want));
        assert_eq!(hex(want.as_bytes()), "d0b4d0b620436166c3a92061f09f9880");
    }
}

/// (f) `nchar2_exp_<v>` row 1 (`truth_nchar2.txt`: NC `00 61 d8 3d de 00`, CL `a` + U+1F600 stored in the
/// database set, NCL UTF-16 `00 61 d8 3d de 00`): CL and NCL give `a` + U+1F600, UTF-8 `61 f0 9f 98 80`,
/// the same as the NVARCHAR2 column; both versions.
#[test]
fn nchar2_clob_and_nclob_give_the_non_bmp_character() {
    for v in VERSIONS {
        let (b, w) = read_default(&format!("{v}/nchar2_exp_{v}.dmp"), "T_NCHAR2");
        assert!(w.is_empty(), "{v}: {w:?}");
        let schema = b[0].schema();
        let pos = |n: &str| schema.index_of(n).unwrap();
        let cl = texts(&b, pos("CL"));
        let ncl = texts(&b, pos("NCL"));
        let nv = col_strings(&b, pos("NV"));
        let want = "a\u{1f600}";
        assert_eq!(hex(want.as_bytes()), "61f09f9880");
        assert_eq!(cl[0].as_deref(), Some(want), "{v}");
        assert_eq!(ncl[0].as_deref(), Some(want), "{v}");
        assert_eq!(nv[0], want, "{v}");
    }
}

/// Hostile input: a chunk length of 0 inside the long value of T_LOB row 5 (C) is `Framing` at that u16 whether
/// the column is read or passed by its lengths (`columns=["ID"]`), with row and column; a chunk that passes
/// the total is `Framing` too. (`fc ff` + T = 200,000 = `40 0d 03 00 00 00 00 00`, then the first chunk's u16.)
#[test]
fn broken_chunks_are_framing_errors_when_read_and_when_skipped() {
    let good = std::fs::read(fixtures().join("18c/lob_exp_18c.dmp")).unwrap();
    let key = [0xfc, 0xff, 0x40, 0x0d, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00];
    let at: Vec<_> = good
        .windows(10)
        .enumerate()
        .filter(|(_, w)| *w == key)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(at.len(), 1);
    let u16_at = at[0] + 10;
    assert_eq!(u16::from_le_bytes([good[u16_at], good[u16_at + 1]]), 16_120);
    for (name, patch) in [("zero", 0u16), ("over", 0xffef)] {
        let mut b = good.clone();
        b[u16_at..u16_at + 2].copy_from_slice(&patch.to_le_bytes());
        // `over`: 0xffef passes nothing by itself (the total is 200,000) but the chunk stream no longer
        // lines up, so the walk meets a bad length later; `zero` fails at the u16 itself
        let p = target_dir("t6l").join(format!("lob_chunk_{name}.dmp"));
        std::fs::write(&p, &b).unwrap();
        for cols in [None, Some(vec!["ID".to_string()])] {
            let o = ReadOptions {
                columns: cols.clone(),
                ..opts()
            };
            // the scan already refuses a file with a broken value area
            let r = oradmp_core::dump::open(&p, oradmp_core::dump::Options::default())
                .and_then(|mut d| try_read(&mut d, "T_LOB", &o).map(|_| ()));
            let e = r.unwrap_err();
            assert_eq!(e.code, ErrorCode::Framing, "{name} {cols:?}");
            if name == "zero" {
                assert_eq!(e.offset, Some(u16_at as u64));
            }
        }
    }
}

/// Hostile input: bytes damaged in the LOB tables of `lob_exp_18c` and `lob3_exp_18c` (a deterministic spread of
/// positions, three values each) never panic the reader, whatever the options; every outcome is a value
/// or a typed error.
#[test]
fn damaged_lob_files_never_panic_the_reader() {
    for file in ["18c/lob_exp_18c.dmp", "18c/lob3_exp_18c.dmp"] {
        let good = std::fs::read(fixtures().join(file)).unwrap();
        let p = target_dir("t6l").join("damaged.dmp");
        let step = good.len() / 120;
        for i in 0..120usize {
            let at = 8_000 + i * (step - 66);
            for v in [0x00u8, 0xff, 0xfc] {
                let mut b = good.clone();
                b[at] = v;
                std::fs::write(&p, &b).unwrap();
                let Ok(mut d) = oradmp_core::dump::open(&p, oradmp_core::dump::Options::default())
                else {
                    continue;
                };
                for o in [
                    opts(),
                    ReadOptions {
                        on_value_error: OnValueError::Null,
                        max_lob_bytes: 5_000,
                        unconfirmed_lob: oradmp_core::reader::UnconfirmedLob::Decode,
                        ..opts()
                    },
                ] {
                    let names: Vec<String> = match d.tables() {
                        Ok(t) => t.iter().map(|t| t.name.clone()).collect(),
                        Err(_) => continue,
                    };
                    for n in names {
                        let _ = try_read(&mut d, &n, &o);
                    }
                }
            }
        }
    }
}
