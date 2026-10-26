//! Task T-3a: the walker of rules v3.3: LONG and LONG RAW [Y9], the 16 type codes [T5], m, the path flag and
//! the LOB column entries [T6, L1], LOB values [L2 to L4, L6], partitions [T10], [R8] for column fields,
//! [E2a], the header checks of ADR-17, list K compared in all four fields (that last one is in `t1_walk.rs`).
//!
//! Expected values come from the Analyst's tools through `tests/data/gen_walk_expected.py`:
//! `walk_parts.tsv` (one line per binary part, 418 for the 211 files: offsets, rows, NULLs, end marker; from
//! `exp_parse.py`) and `lob_expected.tsv` (m, the column entries, k per row, every LOB value with offset, form,
//! stored length and chunk lengths; from `exp_parse.py`, the data `lob_check.py` checks). Numbers written into
//! the tests as constants are those of the rules file v3.3 (the corpus totals: `common::TOTALS`, rules v3.5) (`exp_reader_rules.md` L1 to L6, T10, T8) and of
//! `exp.md` 14.6, 6.7 and 4.8.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use oradmp_core::exp::items::{walk, walk_path, WalkEnd, WalkOptions, WalkReport};
use oradmp_core::exp::rows::{EndMarker, LobForm};
use oradmp_core::exp::table::PathKind;
use oradmp_core::{Error, ErrorCode, WarningCode};

mod common;
use common::TOTALS;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixtures() -> PathBuf {
    root().join("corpus/fixtures")
}

fn bytes_of(rel: &str) -> Vec<u8> {
    std::fs::read(fixtures().join(rel)).unwrap()
}

fn walk_bytes(b: Vec<u8>) -> oradmp_core::Result<WalkReport> {
    walk(Cursor::new(b), &WalkOptions::default())
}

fn walk_rel(rel: &str, options: &WalkOptions) -> WalkReport {
    walk_path(&fixtures().join(rel), options).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn lob_options() -> WalkOptions {
    WalkOptions {
        record_lob: true,
        hash_fields: true,
        ..WalkOptions::default()
    }
}

fn escape(name: &[u8]) -> String {
    let mut s = String::new();
    for &b in name {
        if (0x20..=0x7e).contains(&b) && b != b'\\' {
            s.push(char::from(b));
        } else {
            write!(s, "\\x{b:02x}").unwrap();
        }
    }
    s
}

fn put_u16(b: &mut [u8], at: u64, v: u16) {
    let at = usize::try_from(at).unwrap();
    b[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

fn stopped(r: &WalkReport) -> &Error {
    match &r.end {
        WalkEnd::Stopped(e) => e,
        WalkEnd::Complete => panic!("the walk was not stopped"),
    }
}

fn warning_codes(r: &WalkReport) -> Vec<WarningCode> {
    r.warnings.iter().map(|w| w.code).collect()
}

fn find_bytes(hay: &[u8], needle: &[u8]) -> usize {
    hay.windows(needle.len()).position(|w| w == needle).unwrap()
}

// ---------------------------------------------------------------------------------------------
// the expected tables
// ---------------------------------------------------------------------------------------------

struct Tsv {
    head: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Tsv {
    fn load(name: &str) -> Self {
        let text = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/data")
                .join(name),
        )
        .unwrap();
        let mut lines = text.lines().filter(|l| !l.starts_with('#'));
        let head = lines
            .next()
            .unwrap()
            .split('\t')
            .map(str::to_string)
            .collect();
        let rows = lines
            .map(|l| l.split('\t').map(str::to_string).collect())
            .collect();
        Self { head, rows }
    }

    fn get<'a>(&self, row: &'a [String], col: &str) -> &'a str {
        &row[self.head.iter().position(|h| h == col).unwrap()]
    }

    fn num(&self, row: &[String], col: &str) -> u64 {
        self.get(row, col).parse().unwrap()
    }
}

/// The same run-length text as `rle()` of the generator: `16120x12+6560`, or `-` for no chunks.
fn rle(chunks: &[u16]) -> String {
    if chunks.is_empty() {
        return "-".to_string();
    }
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < chunks.len() {
        let mut j = i;
        while j < chunks.len() && chunks[j] == chunks[i] {
            j += 1;
        }
        out.push(if j - i == 1 {
            chunks[i].to_string()
        } else {
            format!("{}x{}", chunks[i], j - i)
        });
        i = j;
    }
    out.join("+")
}

// ---------------------------------------------------------------------------------------------
// every part of the 211 files at the offsets of the tool
// ---------------------------------------------------------------------------------------------

#[test]
fn x_every_binary_part_of_the_exp_files_has_the_offsets_rows_and_end_marker_of_the_tool() {
    let tsv = Tsv::load("walk_parts.tsv");
    assert_eq!(tsv.rows.len(), TOTALS.parts, "one line per binary part");
    let mut by_file: BTreeMap<&str, Vec<&Vec<String>>> = BTreeMap::new();
    for r in &tsv.rows {
        by_file.entry(tsv.get(r, "file")).or_default().push(r);
    }
    assert_eq!(by_file.len(), TOTALS.files);
    let mut checked = 0;
    for (file, lines) in by_file {
        let report = walk_rel(file, &WalkOptions::default());
        let mut parts = Vec::new();
        for s in &report.sections {
            for (pi, p) in s.parts.iter().enumerate() {
                parts.push((s, pi, p));
            }
        }
        assert_eq!(parts.len(), lines.len(), "{file}: parts");
        for ((s, pi, p), l) in parts.iter().zip(lines) {
            let ctx = format!("{file} section {} part {pi}", s.ordinal);
            assert_eq!(u64::from(s.ordinal), tsv.num(l, "section"), "{ctx}");
            assert_eq!(*pi as u64, tsv.num(l, "part"), "{ctx}");
            assert_eq!(escape(&s.name_raw), tsv.get(l, "table"), "{ctx}");
            assert_eq!(
                p.partition
                    .as_ref()
                    .map(|x| escape(&x.name_raw))
                    .unwrap_or_default(),
                tsv.get(l, "partition"),
                "{ctx}"
            );
            assert_eq!(
                p.subpartition
                    .as_ref()
                    .map(|x| escape(&x.name_raw))
                    .unwrap_or_default(),
                tsv.get(l, "subpartition"),
                "{ctx}"
            );
            // LONG statistics: `column:values:chained:largest:above32000:chunked:band`, `;` between columns, `-` for none
            let long: Vec<String> = p
                .rows
                .long
                .iter()
                .map(|x| {
                    format!(
                        "{}:{}:{}:{}:{}:{}:{}",
                        x.column,
                        x.values,
                        x.chained,
                        x.largest,
                        x.above_32000,
                        x.chunked,
                        x.sizes_65521_to_65534
                    )
                })
                .collect();
            let long = if long.is_empty() {
                "-".to_string()
            } else {
                long.join(";")
            };
            assert_eq!(long, tsv.get(l, "long"), "{ctx}");
            assert_eq!(p.header.columns.len() as u64, tsv.num(l, "ncols"), "{ctx}");
            assert_eq!(p.insert_offset, tsv.num(l, "insert_off"), "{ctx}");
            assert_eq!(p.header.descriptors_offset, tsv.num(l, "desc_off"), "{ctx}");
            assert_eq!(p.header.flag_offset, tsv.num(l, "flag_off"), "{ctx}");
            assert_eq!(p.header.rows_offset, tsv.num(l, "rows_off"), "{ctx}");
            assert_eq!(
                p.header.path == PathKind::Direct,
                tsv.get(l, "path") == "direct",
                "{ctx}"
            );
            assert_eq!(u64::from(p.header.m), tsv.num(l, "m"), "{ctx}");
            assert_eq!(p.rows.rows, tsv.num(l, "rows"), "{ctx}");
            assert_eq!(p.rows.nulls, tsv.num(l, "nulls"), "{ctx}");
            let marker = match p.rows.end_marker {
                EndMarker::FfFf => "ffff",
                EndMarker::LfOnly => "lf",
            };
            assert_eq!(marker, tsv.get(l, "end_marker"), "{ctx}");
            assert_eq!(
                p.rows.end_marker_offset,
                tsv.num(l, "end_marker_off"),
                "{ctx}"
            );
            // the tool's `rows_end` is the offset of the LF; the walker reports the offset behind it
            assert_eq!(p.rows.rows_end_offset, tsv.num(l, "rows_end") + 1, "{ctx}");
            checked += 1;
        }
    }
    assert_eq!(checked, tsv.rows.len());
}

// ---------------------------------------------------------------------------------------------
// (a) Y9: LONG and LONG RAW
// ---------------------------------------------------------------------------------------------

#[test]
fn y9_long_and_long_raw_are_read() {
    // Tool numbers (`exp_parse.py` on both files, `check_all.py`): T_LONG 3 rows (ID NUMBER, L LONG, V
    // VARCHAR2(10)), T_LONGRAW 2 rows (ID, LR LONG RAW), T_FRAME 7 rows; the descriptors of types 8 and 24 are
    // 4 bytes with length 0 [T5].
    for v in ["18c", "11g"] {
        let r = walk_rel(&format!("{v}/long_exp_{v}.dmp"), &lob_options());
        assert!(matches!(r.end, WalkEnd::Complete), "{v}");
        assert!(r.warnings.is_empty(), "{v}: {:?}", r.warnings);
        let names: Vec<(String, u64)> = r
            .sections
            .iter()
            .map(|s| (escape(&s.name_raw), s.parts[0].rows.rows))
            .collect();
        assert_eq!(
            names,
            [
                ("T_LONG".to_string(), 3),
                ("T_LONGRAW".to_string(), 2),
                ("T_FRAME".to_string(), 7)
            ],
            "{v}"
        );
        let long = &r.sections[0].parts[0].header;
        let codes: Vec<(u16, u16)> = long
            .columns
            .iter()
            .map(|c| (c.type_code, c.max_len))
            .collect();
        assert_eq!(codes, [(2, 22), (8, 0), (1, 10)], "{v}");
        assert_eq!(long.m, 0);
        let raw = &r.sections[1].parts[0].header;
        let codes: Vec<(u16, u16)> = raw
            .columns
            .iter()
            .map(|c| (c.type_code, c.max_len))
            .collect();
        assert_eq!(codes, [(2, 22), (24, 0)], "{v}");
        // T_FRAME behind the LONG tables is byte-identical to `t_frame_exp_<v>` (rule T8): same raw fields
        // and the same section bytes
        let frame = &r.sections[2];
        let reference = walk_rel(&format!("{v}/t_frame_exp_{v}.dmp"), &lob_options());
        assert_eq!(
            frame.parts[0].rows.raw_sha256, reference.sections[0].parts[0].rows.raw_sha256,
            "{v}"
        );
        assert_eq!(
            frame.parts[0].rows.nulls,
            reference.sections[0].parts[0].rows.nulls
        );
        let a = bytes_of(&format!("{v}/long_exp_{v}.dmp"));
        let b = bytes_of(&format!("{v}/t_frame_exp_{v}.dmp"));
        let range = |rep: &WalkReport, i: usize| {
            let s = &rep.sections[i];
            let end = s.parts.last().unwrap().rows.rows_end_offset;
            usize::try_from(s.table_line_offset).unwrap()..usize::try_from(end).unwrap()
        };
        assert_eq!(a[range(&r, 2)], b[range(&reference, 0)], "{v}");
    }
    // exp.md 6.6 and 11 (T8): long_exp_11g.dmp@0x11df1-0x11fd9 = t_frame_exp_11g.dmp@0x210d-0x22f5
    let r = walk_rel("11g/long_exp_11g.dmp", &WalkOptions::default());
    let f = &r.sections[2];
    assert_eq!(f.table_line_offset, 0x11df1);
    assert_eq!(f.parts[0].rows.rows_end_offset, 0x11fd9 + 1);
    let r = walk_rel("11g/t_frame_exp_11g.dmp", &WalkOptions::default());
    assert_eq!(r.sections[0].table_line_offset, 0x210d);
    assert_eq!(r.sections[0].parts[0].rows.rows_end_offset, 0x22f5 + 1);
}

// ---------------------------------------------------------------------------------------------
// (c) L1 to L6: LOB columns
// ---------------------------------------------------------------------------------------------

#[test]
fn l1_to_l4_every_lob_table_has_the_m_entries_k_per_row_and_values_of_the_tool() {
    let tsv = Tsv::load("lob_expected.tsv");
    // one line per binary part of a LOB table (sections with a type 112 / 113 column; R_LOB2 of the two
    // `ru_lob2_*` files included)
    assert_eq!(tsv.rows.len(), TOTALS.lob_sections);
    let mut values = 0usize;
    for l in &tsv.rows {
        let file = tsv.get(l, "file");
        let report = walk_rel(file, &lob_options());
        let section = &report.sections[usize::try_from(tsv.num(l, "section")).unwrap() - 1];
        let part = &section.parts[usize::try_from(tsv.num(l, "part")).unwrap()];
        let ctx = format!("{file} {}", tsv.get(l, "table"));
        assert_eq!(escape(&section.name_raw), tsv.get(l, "table"), "{ctx}");
        assert_eq!(u64::from(part.header.m), tsv.num(l, "m"), "{ctx}");
        assert_eq!(
            part.header.columns.len() as u64,
            tsv.num(l, "ncols"),
            "{ctx}"
        );
        // L1: m, the path flag, then m entries `u8 n, n name bytes, 2 bytes`
        let mut entries = String::new();
        for e in &part.header.lob_entries {
            write!(entries, "{:02x}", e.name.len()).unwrap();
            for b in &e.name {
                write!(entries, "{b:02x}").unwrap();
            }
            write!(entries, "{:02x}{:02x}", e.tail[0], e.tail[1]).unwrap();
        }
        assert_eq!(entries, tsv.get(l, "entries"), "{ctx}");
        assert!(part.header.odd_entry_tail.is_none(), "{ctx}");
        // L6: every LOB table has flag 0, whatever the header says
        assert_eq!(part.header.path, PathKind::Conventional, "{ctx}");
        let d = part.rows.lob_detail.as_ref().unwrap();
        let ks: Vec<String> = d.ks.iter().map(|(_, k)| k.to_string()).collect();
        assert_eq!(ks.join(","), tsv.get(l, "ks"), "{ctx}");
        let vs: Vec<String> = d
            .values
            .iter()
            .map(|v| {
                let form = match v.form {
                    LobForm::Short => "short",
                    LobForm::Long => "long",
                };
                format!(
                    "{}:{}:{}:{form}:{}:{}",
                    v.row,
                    v.column,
                    v.offset,
                    v.total,
                    rle(&v.chunks)
                )
            })
            .collect();
        assert_eq!(vs.join(";"), tsv.get(l, "values"), "{ctx}");
        values += d.values.len();
        assert_eq!(part.rows.lob.values as usize, d.values.len(), "{ctx}");
        // k says how many values follow the row: the sum of k is the number of values
        assert_eq!(
            d.ks.iter().map(|(_, k)| usize::from(*k)).sum::<usize>(),
            d.values.len(),
            "{ctx}"
        );
    }
    assert_eq!(values as u64, TOTALS.lob_values);
}

/// Rules L3, L4 and arch 8.1: T_LOB 5 rows with k = 2, 0, 2, 2, 2; T_LOB2 2 rows with k = 3, 1; R_CLOB 3 rows
/// with k = 1, 1, 0; the two long values of T_LOB have 13 chunks each, with the sizes of the version (18c
/// 12 x 16,120 + 6,560 and 12 x 8,060 + 3,280; 11g 12 x 16,264 + 4,832 and 12 x 8,132 + 2,416) and the stored
/// lengths 200,000 and 100,000 (the chunk sizes are
/// "observed in this AL32UTF8 fixture, not a rule (ADR-37)"); the short values have 18, 10, 0, 0, 6,000, 1,000
/// bytes (T_LOB) and 6, 6, 2, 0 (T_LOB2) (`truth_lob.txt` in characters x 2, `tools/lob_check.py`).
#[test]
fn l3_l4_k_per_row_value_forms_and_chunk_counts_per_version() {
    for (v, big, small) in [
        ("18c", (16_120u16, 6_560u16), (8_060u16, 3_280u16)),
        ("11g", (16_264, 4_832), (8_132, 2_416)),
    ] {
        for file in [
            format!("{v}/lob_exp_{v}.dmp"),
            format!("{v}/lob_exp_direct_{v}.dmp"),
        ] {
            let r = walk_rel(&file, &lob_options());
            assert!(r.warnings.is_empty(), "{file}");
            let lob = &r.sections[0].parts[0];
            let lob2 = &r.sections[1].parts[0];
            assert_eq!(escape(&r.sections[0].name_raw), "T_LOB");
            assert_eq!(escape(&r.sections[1].name_raw), "T_LOB2");
            let ks = |p: &oradmp_core::exp::items::Part| -> Vec<u16> {
                p.rows
                    .lob_detail
                    .as_ref()
                    .unwrap()
                    .ks
                    .iter()
                    .map(|k| k.1)
                    .collect()
            };
            assert_eq!(ks(lob), [2, 0, 2, 2, 2], "{file}");
            assert_eq!(ks(lob2), [3, 1], "{file}");
            let totals: Vec<(LobForm, u64)> = lob
                .rows
                .lob_detail
                .as_ref()
                .unwrap()
                .values
                .iter()
                .map(|v| (v.form, v.total))
                .collect();
            use LobForm::{Long, Short};
            assert_eq!(
                totals,
                [
                    (Short, 18),
                    (Short, 10),
                    (Short, 0),
                    (Short, 0),
                    (Short, 6000),
                    (Short, 1000),
                    (Long, 200_000),
                    (Long, 100_000)
                ],
                "{file}"
            );
            let longs: Vec<&Vec<u16>> = lob
                .rows
                .lob_detail
                .as_ref()
                .unwrap()
                .values
                .iter()
                .filter(|v| v.form == Long)
                .map(|v| &v.chunks)
                .collect();
            assert_eq!(longs.len(), 2);
            // observed in this AL32UTF8 fixture, not a rule (ADR-37)
            for (chunks, (size, last)) in longs.iter().zip([big, small]) {
                assert_eq!(chunks.len(), 13, "{file}: 13 chunks");
                assert!(chunks[..12].iter().all(|c| *c == size), "{file}");
                assert_eq!(chunks[12], last, "{file}");
            }
            // short values, empty and non-empty, of T_LOB2
            let t2: Vec<u64> = lob2
                .rows
                .lob_detail
                .as_ref()
                .unwrap()
                .values
                .iter()
                .map(|v| v.total)
                .collect();
            assert_eq!(t2, [6, 6, 2, 0], "{file}");
            // NULL against empty (L4): T_LOB row 2 has no value at all (k = 0); row 3 has two empty ones
            assert_eq!(lob.rows.lob.empty, 2);
            assert_eq!(lob2.rows.lob.empty, 1);
        }
    }
    // R_CLOB of the six `ru_exp_*` files and of `ru_tables_exp_us7ascii_11g` (3 rows, k 1, 1, 0; values of 6
    // and 6,000 bytes, short form, `truth_ru.txt`)
    for f in [
        "ru_exp_al32utf8_11g",
        "ru_exp_cp1251_11g",
        "ru_exp_direct_cp1251_11g",
        "ru_exp_direct_us7ascii_11g",
        "ru_exp_us7ascii_11g",
        "ru_exp_w1252_11g",
        "ru_tables_exp_us7ascii_11g",
    ] {
        let r = walk_rel(&format!("11g-cp1251/{f}.dmp"), &lob_options());
        let clob = r
            .sections
            .iter()
            .find(|s| s.name_raw == b"R_CLOB")
            .unwrap_or_else(|| panic!("{f}: no R_CLOB"));
        let p = &clob.parts[0];
        assert_eq!(p.rows.rows, 3, "{f}");
        let d = p.rows.lob_detail.as_ref().unwrap();
        assert_eq!(
            d.ks.iter().map(|k| k.1).collect::<Vec<_>>(),
            [1, 1, 0],
            "{f}"
        );
        assert_eq!(
            d.values.iter().map(|v| v.total).collect::<Vec<_>>(),
            [6, 6000],
            "{f}"
        );
        assert!(d.values.iter().all(|v| v.form == LobForm::Short));
        assert_eq!(p.header.m, 2, "{f}");
        assert_eq!(p.header.lob_entries.len(), 2);
        assert_eq!(p.header.lob_entries[1].name, b"CL", "{f}");
    }
}

/// The offsets of `exp.md` 14.6 and rule T6: `lob_exp_11g.dmp`@0x2401 holds `03 00` (m) and `00 00` (the flag),
/// the entries of T_LOB follow (11 bytes), the rows start at 0x2410; in `ru_tables_exp_us7ascii_11g.dmp` the
/// 12 bytes between descriptors and rows of R_CLOB are at 0x11ae4 to 0x11aef (m = 2, the flag, 8 bytes of
/// entries) and the first descriptor that stopped the walk in session 7 stood at 0x11adc.
#[test]
fn t6_l1_the_bytes_between_descriptors_and_rows_of_a_lob_table() {
    let b = bytes_of("11g/lob_exp_11g.dmp");
    let r = walk_rel("11g/lob_exp_11g.dmp", &WalkOptions::default());
    let h = &r.sections[0].parts[0].header;
    assert_eq!(h.m_offset, 0x2401);
    assert_eq!(&b[0x2401..0x2405], [3, 0, 0, 0]);
    assert_eq!(h.flag_offset, 0x2403);
    assert_eq!(h.rows_offset, 0x2410);
    assert_eq!(
        &b[0x2405..0x2410],
        [0x00, 0x00, 0x00, 0x01, 0x43, 0x00, 0x00, 0x01, 0x42, 0x00, 0x00]
    );
    let r = walk_rel(
        "11g-cp1251/ru_tables_exp_us7ascii_11g.dmp",
        &WalkOptions::default(),
    );
    let clob = r.sections.iter().find(|s| s.name_raw == b"R_CLOB").unwrap();
    let h = &clob.parts[0].header;
    assert_eq!(h.columns[1].offset, 0x11adc);
    assert_eq!(h.columns[1].type_code, 112);
    assert_eq!(h.m_offset, 0x11ae4);
    assert_eq!(h.rows_offset, 0x11af0);
    let b = bytes_of("11g-cp1251/ru_tables_exp_us7ascii_11g.dmp");
    assert_eq!(
        &b[0x11ae4..0x11af0],
        [2, 0, 0, 0, 0x00, 0x00, 0x00, 0x02, 0x43, 0x4c, 0x00, 0x00]
    );
}

/// T8 and L6: T_FRAME behind LOB, INTERVAL and ROWID tables is byte-identical to `t_frame_exp_<v>` (`exp.md`
/// 11 / T8: `lob_exp_11g.dmp`@0x4e181-0x4e369 behind two LOB tables, `misc_exp_11g.dmp`@0x24a0-0x2688 behind
/// INTERVAL and ROWID tables; `tools/lob_check.py --frame`, `tools/frame_t8.py`), on both paths and in both
/// versions; a LOB table of a `direct=y` file has flag 0 and T_FRAME flag 1.
#[test]
fn t8_l6_t_frame_after_lob_interval_and_rowid_tables_equals_t_frame() {
    for v in ["18c", "11g"] {
        for (file, reference) in [
            (
                format!("{v}/lob_exp_{v}.dmp"),
                format!("{v}/t_frame_exp_{v}.dmp"),
            ),
            (
                format!("{v}/lob_exp_direct_{v}.dmp"),
                format!("{v}/t_frame_exp_direct_{v}.dmp"),
            ),
            (
                format!("{v}/misc_exp_{v}.dmp"),
                format!("{v}/t_frame_exp_{v}.dmp"),
            ),
        ] {
            let r = walk_rel(&file, &lob_options());
            let t = walk_rel(&reference, &lob_options());
            let frame = r.sections.last().unwrap();
            assert_eq!(frame.name_raw, b"T_FRAME", "{file}");
            let (fp, tp) = (&frame.parts[0], &t.sections[0].parts[0]);
            assert_eq!(fp.rows.rows, 7);
            assert_eq!(fp.rows.raw_sha256, tp.rows.raw_sha256, "{file}");
            assert_eq!(fp.header.path, tp.header.path, "{file}");
            let a = bytes_of(&file);
            let b = bytes_of(&reference);
            let range = |s: &oradmp_core::exp::items::Section| {
                usize::try_from(s.table_line_offset).unwrap()
                    ..usize::try_from(s.parts.last().unwrap().rows.rows_end_offset).unwrap()
            };
            assert_eq!(a[range(frame)], b[range(&t.sections[0])], "{file}");
            if file.contains("lob_exp_direct") {
                // L6: the LOB tables are written on the conventional path, T_FRAME on the direct one
                for s in &r.sections[..2] {
                    assert_eq!(s.parts[0].header.path, PathKind::Conventional);
                }
                assert_eq!(fp.header.path, PathKind::Direct);
            }
        }
    }
    let r = walk_rel("11g/lob_exp_11g.dmp", &WalkOptions::default());
    assert_eq!(r.sections[1].parts[0].rows.rows_end_offset, 0x4e181);
    assert_eq!(r.sections[2].parts[0].rows.rows_end_offset, 0x4e369 + 1);
    let r = walk_rel("11g/misc_exp_11g.dmp", &WalkOptions::default());
    assert_eq!(r.sections[1].parts[0].rows.rows_end_offset, 0x24a0);
    assert_eq!(r.sections[2].parts[0].rows.rows_end_offset, 0x2688 + 1);
    // misc_exp: INTERVAL and ROWID descriptors are 4 bytes (lengths 5, 11, 23 and the declared 4000)
    let codes: Vec<(u16, u16)> = r.sections[0].parts[0]
        .header
        .columns
        .iter()
        .chain(r.sections[1].parts[0].header.columns.iter())
        .map(|c| (c.type_code, c.max_len))
        .collect();
    assert_eq!(
        codes,
        [(2, 22), (182, 5), (183, 11), (2, 22), (69, 23), (208, 4000)]
    );
}

// ---------------------------------------------------------------------------------------------
// (d) T10: partitions
// ---------------------------------------------------------------------------------------------

#[test]
fn t10_t_part_is_one_section_with_three_parts_on_both_paths() {
    for (file, direct) in [
        ("18c/part_exp_18c.dmp", false),
        ("18c/part_exp_direct_18c.dmp", true),
    ] {
        let r = walk_rel(file, &WalkOptions::default());
        assert!(r.warnings.is_empty(), "{file}: {:?}", r.warnings);
        assert_eq!(r.sections.len(), 1, "{file}");
        let s = &r.sections[0];
        assert_eq!(s.name_raw, b"T_PART");
        assert!(s.partitioned);
        assert_eq!(s.parts.len(), 3);
        let names: Vec<String> = s
            .parts
            .iter()
            .map(|p| escape(&p.partition.as_ref().unwrap().name_raw))
            .collect();
        assert_eq!(names, ["P2019", "P2020", "PMAX"]);
        // rows per partition: `truth_part.txt` = the log (1,095 / 1,098 / 807), total 3,000
        let rows: Vec<u64> = s.parts.iter().map(|p| p.rows.rows).collect();
        assert_eq!(rows, [1095, 1098, 807], "{file}");
        assert_eq!(rows.iter().sum::<u64>(), 3000);
        // each part has its own path flag; the same descriptors
        for p in &s.parts {
            assert_eq!(p.header.path == PathKind::Direct, direct, "{file}");
            assert!(p.header.same_layout(&s.parts[0].header));
            assert_eq!(p.insert.table, b"T_PART");
        }
        // the PARTITION line stands directly before each INSERT INTO line and ENDPARTITION closes the list
        let b = bytes_of(file);
        for p in &s.parts {
            let at = usize::try_from(p.insert_offset).unwrap();
            let line = &b[..at - 1];
            let start = line.iter().rposition(|c| *c == b'\n').unwrap() + 1;
            assert!(b[start..at - 1].starts_with(b"PARTITION \""), "{file}");
        }
        assert!(b.windows(13).any(|w| w == b"\nENDPARTITION"));
    }
}

// ---------------------------------------------------------------------------------------------
// (e) patched copies in memory
// ---------------------------------------------------------------------------------------------

#[test]
fn x1_a_type_code_outside_the_16_stops_the_file_and_the_earlier_tables_stay() {
    let good = walk_rel("18c/multi_types_exp_18c.dmp", &WalkOptions::default());
    assert_eq!(good.sections.len(), 4);
    let third = &good.sections[2].parts[0].header.columns[0];
    for code in [114u16, 9, 0, 0xffff] {
        let mut b = bytes_of("18c/multi_types_exp_18c.dmp");
        put_u16(&mut b, third.offset, code);
        let r = walk_bytes(b).unwrap();
        let e = stopped(&r);
        assert_eq!(e.code, ErrorCode::UnsupportedType, "{code}");
        assert_eq!(e.type_code, Some(code));
        assert_eq!(e.offset, Some(third.offset));
        assert_eq!((e.table, e.column), (Some(3), Some(1)));
        assert_eq!(
            r.sections.len(),
            2,
            "the two tables before the stop are listed"
        );
        assert!(r.end_offset.is_none());
        // the stopped section is named from its TABLE line (the error carries only the ordinal)
        let st = r.stopped_section.as_ref().unwrap();
        assert_eq!(st.ordinal, 3);
        assert_eq!(st.name_raw, good.sections[2].name_raw);
        assert_eq!(st.table_line_offset, good.sections[2].table_line_offset);
    }
}

#[test]
fn t6_x5_m_flag_and_column_entries_that_the_rules_do_not_allow_stop_with_unsupported_layout() {
    let file = "18c/lob_exp_18c.dmp";
    let r = walk_rel(file, &WalkOptions::default());
    let h = &r.sections[0].parts[0].header;
    let patched = |f: &dyn Fn(&mut Vec<u8>)| {
        let mut b = bytes_of(file);
        f(&mut b);
        walk_bytes(b).unwrap()
    };
    // m = 2 for 3 columns
    let p = patched(&|b| put_u16(b, h.m_offset, 2));
    let e = stopped(&p);
    assert_eq!(
        (e.code, e.offset),
        (ErrorCode::UnsupportedLayout, Some(h.m_offset))
    );
    assert_eq!(e.table, Some(1));
    // m = 0 although the descriptors have LOB columns, and m = 4
    for m in [0u16, 4, 0xffff] {
        let p = patched(&|b| put_u16(b, h.m_offset, m));
        assert_eq!(stopped(&p).code, ErrorCode::UnsupportedLayout, "m = {m}");
    }
    // path flag 1 with m > 0
    let p = patched(&|b| put_u16(b, h.flag_offset, 1));
    let e = stopped(&p);
    assert_eq!(
        (e.code, e.offset),
        (ErrorCode::UnsupportedLayout, Some(h.flag_offset))
    );
    // a flag that is neither 0 nor 1
    let p = patched(&|b| put_u16(b, h.flag_offset, 2));
    assert_eq!(stopped(&p).code, ErrorCode::UnsupportedLayout);
    // an entry with n > 0 on the NUMBER column
    let e0 = &h.lob_entries[0];
    let p = patched(&|b| b[usize::try_from(e0.offset).unwrap()] = 1);
    let e = stopped(&p);
    assert_eq!(
        (e.code, e.offset),
        (ErrorCode::UnsupportedLayout, Some(e0.offset))
    );
    assert_eq!(e.column, Some(1));
    // an entry with n = 0 on a CLOB column
    let e1 = &h.lob_entries[1];
    let p = patched(&|b| b[usize::try_from(e1.offset).unwrap()] = 0);
    let e = stopped(&p);
    assert_eq!(
        (e.code, e.offset),
        (ErrorCode::UnsupportedLayout, Some(e1.offset))
    );
    assert_eq!(e.column, Some(2));
    // m > 0 for a table without a LOB column
    let mut b = bytes_of("18c/t_num_exp_18c.dmp");
    let t = walk_bytes(b.clone()).unwrap();
    let th = &t.sections[0].parts[0].header;
    put_u16(&mut b, th.m_offset, th.columns.len() as u16);
    let p = walk_bytes(b).unwrap();
    assert_eq!(stopped(&p).code, ErrorCode::UnsupportedLayout);
    // the other tables of the file are unaffected where the layout is fine
    assert!(matches!(r.end, WalkEnd::Complete));
}

#[test]
fn l3_l4_r8_broken_lob_framing_and_lengths_that_never_occur_are_framing_errors() {
    let file = "18c/lob_exp_18c.dmp";
    let r = walk_rel(file, &lob_options());
    let part = &r.sections[0].parts[0];
    let d = part.rows.lob_detail.as_ref().unwrap();
    let framing_at = |mut b: Vec<u8>, patch: &dyn Fn(&mut Vec<u8>)| -> Error {
        patch(&mut b);
        match walk_bytes(b) {
            Err(e) => e,
            Ok(r) => panic!("no error; end {:?}, warnings {:?}", r.end, r.warnings),
        }
    };
    let good = bytes_of(file);
    // k off by one, either way: Framing at the k field
    let (k_at, k) = d.ks[0];
    assert_eq!(k, 2);
    for wrong in [1u16, 3, 0] {
        let e = framing_at(good.clone(), &|b| put_u16(b, k_at, wrong));
        assert_eq!(
            (e.code, e.offset, e.row),
            (ErrorCode::Framing, Some(k_at), Some(1)),
            "k {wrong}"
        );
        assert_eq!(e.table, Some(1));
    }
    // a LOB value that the row does not announce (k = 0 for a row that has locators)
    // chunk length 0, from 0xfff0 and a chunk that passes T, and a long value without chunks
    let long = d.values.iter().find(|v| v.form == LobForm::Long).unwrap();
    let first_chunk = long.offset + 10;
    for c in [0u16, 0xfff0, 0xfffd, 0xffff] {
        let e = framing_at(good.clone(), &|b| put_u16(b, first_chunk, c));
        assert_eq!(
            (e.code, e.offset),
            (ErrorCode::Framing, Some(first_chunk)),
            "chunk {c:#x}"
        );
        assert_eq!(e.column, Some(long.column));
    }
    // T replaced by 100: the first chunk (16,120 bytes) passes it
    let e = framing_at(good.clone(), &|b| {
        let at = usize::try_from(long.offset + 2).unwrap();
        b[at..at + 8].copy_from_slice(&100u64.to_le_bytes());
    });
    assert_eq!((e.code, e.offset), (ErrorCode::Framing, Some(first_chunk)));
    // T larger than the chunks that follow: the last chunk of the value is taken for a chunk and the next
    // u16 is not a chunk length either (an error somewhere behind, never a panic)
    let e = framing_at(good.clone(), &|b| {
        let at = usize::try_from(long.offset + 2).unwrap();
        b[at..at + 8].copy_from_slice(&(long.total + 1000).to_le_bytes());
    });
    assert!(
        matches!(e.code, ErrorCode::Framing | ErrorCode::Truncated),
        "{e}"
    );
    // `fe ff` (and 0xfff0 to 0xfffb, 0xfffd, 0xffff) in the value area
    let first_value = d.values[0].offset;
    for v in [0xfffeu16, 0xffff, 0xfff0, 0xfffd] {
        let e = framing_at(good.clone(), &|b| put_u16(b, first_value, v));
        assert_eq!(
            (e.code, e.offset),
            (ErrorCode::Framing, Some(first_value)),
            "value {v:#x}"
        );
    }
}

#[test]
fn r8_column_lengths_of_zero_and_fd_ff_are_framing_in_any_column_even_a_long_one() {
    let file = "18c/t_num_exp_18c.dmp";
    let r = walk_rel(file, &WalkOptions::default());
    let at = r.sections[0].parts[0].header.rows_offset;
    for len in [0u16, 0xfff0, 0xfffd] {
        let mut b = bytes_of(file);
        put_u16(&mut b, at, len);
        let e = walk_bytes(b).unwrap_err();
        assert_eq!(
            (e.code, e.offset),
            (ErrorCode::Framing, Some(at)),
            "{len:#x}"
        );
        assert_eq!((e.table, e.row, e.column), (Some(1), Some(1), Some(1)));
        assert_eq!(e.length, Some(u64::from(len)));
    }
    // LONG: 0 is the same error (ADR-15), at the column's length field; 0xfff1 to 0xfffc in a conventional LONG
    // column are no longer Framing but, for 0xfffb / 0xfffc, the named stop of [R8] (T-19b, ADR-99 item 1); 0xfff1 is a field
    let file = "18c/long_exp_18c.dmp";
    let r = walk_rel(file, &WalkOptions::default());
    let rows = r.sections[0].parts[0].header.rows_offset;
    let b0 = bytes_of(file);
    let id_len = u64::from(u16::from_le_bytes([
        b0[usize::try_from(rows).unwrap()],
        b0[usize::try_from(rows).unwrap() + 1],
    ]));
    let l_at = rows + 2 + id_len;
    for (len, code) in [
        (0xfffcu16, ErrorCode::UnsupportedLayout),
        (0xfffb, ErrorCode::UnsupportedLayout),
        (0, ErrorCode::Framing),
    ] {
        let mut b = b0.clone();
        put_u16(&mut b, l_at, len);
        // `Framing` is an error of the walk; a stop of the table ends the report (ADR-12)
        let e = match walk_bytes(b) {
            Err(e) => e,
            Ok(r) => match r.end {
                WalkEnd::Stopped(e) => e,
                WalkEnd::Complete => panic!("{len:#x}: the file was accepted"),
            },
        };
        assert_eq!(
            (e.code, e.offset, e.column),
            (code, Some(l_at), Some(2)),
            "{len:#x}"
        );
    }
    // lengths above 32,000 are not errors [R8]: a 40,000-byte value is read like any length
    assert!(r.warnings.is_empty());
}

#[test]
fn t10_a_second_insert_into_needs_a_partition_line_and_equal_descriptors() {
    let file = "18c/part_exp_18c.dmp";
    let good = bytes_of(file);
    let r = walk_bytes(good.clone()).unwrap();
    let s = &r.sections[0];
    // no PARTITION line at all: the second INSERT INTO is Framing
    let mut b = good.clone();
    let mut from = 0;
    while let Some(i) = b[from..].windows(12).position(|w| w == b"\nPARTITION \"") {
        let at = from + i + 1;
        b[at] = b'X';
        from = at;
    }
    let e = walk_bytes(b).unwrap_err();
    assert_eq!(
        (e.code, e.offset),
        (ErrorCode::Framing, Some(s.parts[1].insert_offset))
    );
    assert_eq!(e.table, Some(1));
    // a changed descriptor in part 2 (the maximum length of column 1)
    let c1 = &s.parts[1].header.columns[0];
    let mut b = good.clone();
    put_u16(&mut b, c1.offset + 2, c1.max_len + 1);
    let e = walk_bytes(b).unwrap_err();
    assert_eq!(
        (e.code, e.offset),
        (ErrorCode::Framing, Some(s.parts[1].insert_offset))
    );
    // a changed type code in part 3
    let c1 = &s.parts[2].header.columns[0];
    let mut b = good.clone();
    put_u16(&mut b, c1.offset, 12);
    let e = walk_bytes(b).unwrap_err();
    assert_eq!(
        (e.code, e.offset),
        (ErrorCode::Framing, Some(s.parts[2].insert_offset))
    );
    // a changed column name in the INSERT INTO line of part 2 (same length)
    let mut b = good.clone();
    let at = usize::try_from(s.parts[1].insert_offset).unwrap();
    let rel = find_bytes(&b[at..], b"(\"") + 2;
    b[at + rel] = if b[at + rel] == b'Z' { b'Y' } else { b'Z' };
    let e = walk_bytes(b).unwrap_err();
    assert_eq!(
        (e.code, e.offset),
        (ErrorCode::Framing, Some(s.parts[1].insert_offset))
    );
    // a second INSERT INTO in an ordinary section
    let t = bytes_of("18c/multi_types_exp_18c.dmp");
    let rep = walk_bytes(t.clone()).unwrap();
    assert_eq!(rep.sections.len(), 4);
}

#[test]
fn t10_untested_partition_layouts_are_warnings_and_the_parts_are_read() {
    let file = "18c/part_exp_18c.dmp";
    let good = bytes_of(file);
    let r = walk_bytes(good.clone()).unwrap();
    assert!(r.warnings.is_empty());
    let s = &r.sections[0];
    // the second part is not directly preceded by a PARTITION line (a SUBPARTITION line, say): read, warned
    let mut b = good.clone();
    let at = usize::try_from(s.parts[1].insert_offset).unwrap();
    let line = b[..at]
        .windows(11)
        .rposition(|w| w == b"PARTITION \"")
        .unwrap();
    b[line] = b'X';
    let r = walk_bytes(b).unwrap();
    assert_eq!(warning_codes(&r), [WarningCode::UntestedLayout]);
    assert_eq!(r.warnings[0].offset, Some(s.parts[1].insert_offset));
    let rows: Vec<u64> = r.sections[0].parts.iter().map(|p| p.rows.rows).collect();
    assert_eq!(rows, [1095, 1098, 807]);
    assert_eq!(
        r.sections[0].parts[1].partition.as_ref().unwrap().name_raw,
        b"P2019",
        "named after the last PARTITION line"
    );
    // no ENDPARTITION: the section ends [T2] without it
    let mut b = good.clone();
    let at = find_bytes(&b, b"\nENDPARTITION") + 1;
    b[at] = b'X';
    let r = walk_bytes(b).unwrap();
    assert_eq!(warning_codes(&r), [WarningCode::UntestedLayout]);
    assert_eq!(r.warnings[0].table, Some(1));
    // PARTITION and ENDPARTITION lines outside a section: inserted behind the first ENDTABLE of a file
    let t = bytes_of("18c/own_exp_18c.dmp");
    let at = find_bytes(&t, b"\nENDTABLE\n") + 10;
    for line in [&b"ENDPARTITION\n"[..], b"PARTITION \"P1\"\n"] {
        let mut b = t.clone();
        b.splice(at..at, line.iter().copied());
        let r = walk_bytes(b).unwrap();
        let codes = warning_codes(&r);
        assert!(codes.contains(&WarningCode::UntestedLayout), "{codes:?}");
        let w = r
            .warnings
            .iter()
            .find(|w| w.code == WarningCode::UntestedLayout)
            .unwrap();
        assert_eq!(w.offset, Some(at as u64));
        // never a G1 candidate: no unknown-introducing-line warning, and no field was taken
        assert_eq!(r.item_counts.g1_fields, 0);
    }
}

#[test]
fn l1_the_two_bytes_after_an_entry_and_a_changed_entry_name_are_untested_layout_warnings() {
    let file = "18c/lob_exp_18c.dmp";
    let r = walk_rel(file, &WalkOptions::default());
    let h = &r.sections[0].parts[0].header;
    let e1 = &h.lob_entries[1];
    let tail_at = e1.offset + 1 + e1.name.len() as u64;
    let mut b = bytes_of(file);
    b[usize::try_from(tail_at).unwrap()] = 1;
    let r = walk_bytes(b).unwrap();
    assert_eq!(warning_codes(&r), [WarningCode::UntestedLayout]);
    assert_eq!(r.warnings[0].offset, Some(tail_at));
    assert_eq!(r.warnings[0].table, Some(1));
    // the walk is unaffected: same rows, same LOB values
    let g = walk_rel(file, &WalkOptions::default());
    assert_eq!(
        r.parts().map(|p| p.rows.rows).collect::<Vec<_>>(),
        g.parts().map(|p| p.rows.rows).collect::<Vec<_>>()
    );
    assert_eq!(r.lob_totals(), g.lob_totals());
    // all of `01 00` in the two bytes
    let mut b = bytes_of(file);
    b[usize::try_from(tail_at).unwrap()] = 1;
    b[usize::try_from(tail_at).unwrap() + 1] = 0;
    assert_eq!(
        warning_codes(&walk_bytes(b).unwrap()),
        [WarningCode::UntestedLayout]
    );
    // the name in an entry differs from the INSERT INTO name of that column: warned once per table
    let mut b = bytes_of(file);
    b[usize::try_from(e1.offset).unwrap() + 1] = b'Q';
    let r = walk_bytes(b).unwrap();
    assert_eq!(warning_codes(&r), [WarningCode::UntestedLayout]);
    assert_eq!(r.warnings[0].offset, Some(e1.offset));
    // both at once in one table: still one warning (once per table)
    let mut b = bytes_of(file);
    b[usize::try_from(e1.offset).unwrap() + 1] = b'Q';
    b[usize::try_from(tail_at).unwrap()] = 1;
    assert_eq!(warning_codes(&walk_bytes(b).unwrap()).len(), 1);
}

#[test]
fn h3_h4_h6_header_patches_give_untested_header_warnings_with_the_offset() {
    let file = "18c/t_num_exp_18c.dmp";
    let good = bytes_of(file);
    assert!(walk_bytes(good.clone()).unwrap().warnings.is_empty());
    // the three lines after RECORDLENGTH: `0`, `72`, `0` [H3]
    let lines = find_bytes(&good, b"\n0\n72\n0\n") + 1;
    for (patch, at) in [(&b"1"[..], lines), (b"73", lines + 2), (b"9", lines + 5)] {
        let mut b = good.clone();
        b[at..at + patch.len()].copy_from_slice(patch);
        let r = walk_bytes(b).unwrap();
        assert_eq!(
            warning_codes(&r),
            [WarningCode::UntestedHeader],
            "{patch:?}"
        );
        // the offset is that of the first line that differs
        assert_eq!(r.warnings[0].offset, Some(at as u64), "{patch:?}");
    }
    // the byte after the file-name length [H4]: 3 sets (6 bytes), 10 bytes, u16 file-name length, `00`
    let zero = lines + 7 + 6 + 10 + 2;
    assert_eq!(good[zero], 0);
    let mut b = good.clone();
    b[zero] = 1;
    let r = walk_bytes(b).unwrap();
    assert_eq!(warning_codes(&r), [WarningCode::UntestedHeader]);
    assert_eq!(r.warnings[0].offset, Some(zero as u64));
    // the mode line against the first item [H6]: RTABLES with METRICSU, and RUSERS with METRICST
    let first = walk_bytes(good.clone()).unwrap().header.first_item_offset as usize;
    assert_eq!(&good[first..first + 8], b"METRICST");
    let mut b = good.clone();
    b[first + 7] = b'U';
    let r = walk_bytes(b).unwrap();
    assert_eq!(warning_codes(&r), [WarningCode::UntestedHeader]);
    assert_eq!(r.warnings[0].offset, Some(first as u64));
    let own = bytes_of("18c/own_exp_18c.dmp");
    let r0 = walk_bytes(own.clone()).unwrap();
    assert!(r0.warnings.is_empty());
    let first = r0.header.first_item_offset as usize;
    assert_eq!(&own[first..first + 8], b"METRICSU");
    let mut b = own.clone();
    b[first + 7] = b'T';
    let r = walk_bytes(b).unwrap();
    assert_eq!(warning_codes(&r), [WarningCode::UntestedHeader]);
    // a missing line stays MissingMetricsLine
    let mut b = own;
    b[first + 7] = b'X';
    assert_eq!(
        walk_bytes(b).unwrap_err().code,
        ErrorCode::MissingMetricsLine
    );
}

// ---------------------------------------------------------------------------------------------
// (e) E2a on synthetic files
// ---------------------------------------------------------------------------------------------

/// `stat_exp_direct_18c.dmp` T_STAT4 is an empty direct-path table (no `ff ff`, an `ANALSTATS` line follows
/// the LF); its first column is NUMBER (type 2, length 22). Patching that descriptor to type 8, length 0 makes
/// a LONG-first table without rows: the three outcomes of ADR-22.
#[test]
fn e2a_a_long_first_column_without_rows_is_no_rows_with_a_warning_by_default_and_framing_when_strict(
) {
    let file = "18c/stat_exp_direct_18c.dmp";
    let good = bytes_of(file);
    let r = walk_bytes(good.clone()).unwrap();
    assert!(r.warnings.is_empty());
    let t4 = &r.sections[3].parts[0];
    assert_eq!(t4.rows.end_marker, EndMarker::LfOnly);
    let c = t4.header.columns[0];
    assert_eq!((c.type_code, c.max_len), (2, 22));
    let mut b = good.clone();
    put_u16(&mut b, c.offset, 8);
    put_u16(&mut b, c.offset + 2, 0);

    // outcome 1: text of a known kind follows (ANALSTATS and a well-formed field): no rows, W-UNTESTED-LAYOUT
    // at the LF (T-3c: no W-LONG-DIRECT any more, ADR-15)
    let p = walk_bytes(b.clone()).unwrap();
    let t = &p.sections[3].parts[0];
    assert_eq!((t.rows.rows, t.rows.end_marker), (0, EndMarker::LfOnly));
    assert!(t.rows.e2a_no_rows);
    assert_eq!(warning_codes(&p), [WarningCode::UntestedLayout]);
    assert_eq!(p.warnings[0].offset, Some(t.rows.end_marker_offset));
    assert_eq!(p.warnings[0].table, Some(4));
    assert_eq!(t.rows.rows_end_offset, t4.rows.rows_end_offset);
    // the same file, strict: "text follows" alone is Framing (session 2 rule)
    let strict = WalkOptions {
        strict_e2a: true,
        ..WalkOptions::default()
    };
    let e = walk(Cursor::new(b.clone()), &strict).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.table),
        (ErrorCode::Framing, Some(t4.rows.end_marker_offset), Some(4))
    );
}

#[test]
fn e2a_text_that_is_not_an_item_is_a_row_and_a_long_first_column_with_rows_is_read() {
    // outcome 2: the LF is a length byte. `t_empty_exp_direct_18c.dmp` T_EMPTY (3 columns: NUMBER, VARCHAR2,
    // DATE), first column patched to LONG; the rows part is replaced by one row `0a 41` + 16,648 bytes (a
    // length with an LF as low byte and a capital letter as high byte), NULL, NULL, then `ff ff` LF.
    let file = "18c/t_empty_exp_direct_18c.dmp";
    let good = bytes_of(file);
    let r = walk_bytes(good.clone()).unwrap();
    let t = &r.sections[0].parts[0];
    let c = t.header.columns[0];
    let at = usize::try_from(t.header.rows_offset).unwrap();
    assert_eq!(good[at], b'\n');
    let mut row = Vec::new();
    row.extend_from_slice(&0x410au16.to_le_bytes());
    row.extend(std::iter::repeat_n(b'x', 0x410a));
    row.extend_from_slice(&[0xfe, 0xff, 0xfe, 0xff]);
    row.extend_from_slice(&[0xff, 0xff, b'\n']);
    let mut b = good[..at].to_vec();
    b.extend_from_slice(&row);
    b.extend_from_slice(&good[at + 1..]);
    put_u16(&mut b, c.offset, 8);
    put_u16(&mut b, c.offset + 2, 0);
    let p = walk_bytes(b.clone()).unwrap();
    let t = &p.sections[0].parts[0];
    assert_eq!((t.rows.rows, t.rows.end_marker), (1, EndMarker::FfFf));
    assert!(!t.rows.e2a_no_rows);
    assert_eq!(t.rows.nulls, 2);
    // (the file is longer than before: W-FILE-SIZE of the header comes first)
    assert_eq!(warning_codes(&p), [WarningCode::FileSize]);
    // strict: the session 2 rule says "text follows": Framing
    let strict = WalkOptions {
        strict_e2a: true,
        ..WalkOptions::default()
    };
    let e = walk(Cursor::new(b), &strict).unwrap_err();
    assert_eq!((e.code, e.offset), (ErrorCode::Framing, Some(at as u64)));

    // outcome 2b: a LONG value of 10 bytes (`0a 00`): the LF is a length byte, the next byte is no capital
    // letter, so "text follows" fails. The session 2 rule had no answer here (test (b) cannot fail for a
    // first column of length 0): the walker read it as an ambiguous empty table.
    let mut row = Vec::new();
    row.extend_from_slice(&10u16.to_le_bytes());
    row.extend_from_slice(b"0123456789");
    row.extend_from_slice(&[0xfe, 0xff, 0xfe, 0xff, 0xff, 0xff, b'\n']);
    let mut b = good[..at].to_vec();
    b.extend_from_slice(&row);
    b.extend_from_slice(&good[at + 1..]);
    put_u16(&mut b, c.offset, 8);
    put_u16(&mut b, c.offset + 2, 0);
    for options in [WalkOptions::default(), strict.clone()] {
        let p = walk(Cursor::new(b.clone()), &options).unwrap();
        let t = &p.sections[0].parts[0];
        assert_eq!((t.rows.rows, t.rows.end_marker), (1, EndMarker::FfFf));
        assert!(!t.rows.e2a_no_rows);
        assert_eq!(t.rows.nulls, 2);
    }

    // outcome 3: the byte at p is not an LF: a row is read, in both modes; same rows as with a NUMBER
    // first column (`t_nullmix_exp_direct_18c.dmp`, 8 rows, first column NUMBER)
    let file = "18c/t_nullmix_exp_direct_18c.dmp";
    let good = bytes_of(file);
    let g = walk_bytes(good.clone()).unwrap();
    let c = g.sections[0].parts[0].header.columns[0];
    let mut b = good;
    put_u16(&mut b, c.offset, 24);
    put_u16(&mut b, c.offset + 2, 0);
    for options in [WalkOptions::default(), strict] {
        let p = walk(Cursor::new(b.clone()), &options).unwrap();
        let t = &p.sections[0].parts[0];
        assert_eq!(t.rows.rows, 8);
        assert_eq!(t.rows.nulls, g.sections[0].parts[0].rows.nulls);
        assert!(p.warnings.is_empty(), "{:?}", p.warnings);
    }
}

// ---------------------------------------------------------------------------------------------
// the fixtures give none of these errors or warnings (211 files); damaged files never panic
// ---------------------------------------------------------------------------------------------

/// T-3c (a): all 211 files walk to the second `EXIT`, none stops, none warns (the rules v3.4 cover every
/// layout the fixtures hold, `W-LONG-DIRECT` is retired), and AQ-1: no long LOB value has a total of 0
/// (walker and tool agree: the `lob_zero_total` column of `walk_expected.tsv` is 0 in every file).
#[test]
fn x_the_exp_fixtures_give_no_stop_no_framing_error_and_no_warning() {
    let tsv = Tsv::load("walk_expected.tsv");
    assert_eq!(tsv.rows.len(), TOTALS.files);
    let mut zero_tool = 0;
    let mut zero_walker = 0;
    for l in &tsv.rows {
        let file = tsv.get(l, "file");
        let r = walk_rel(file, &WalkOptions::default());
        assert!(matches!(r.end, WalkEnd::Complete), "{file}");
        assert!(r.end_offset.is_some(), "{file}");
        assert!(r.warnings.is_empty(), "{file}: {:?}", r.warnings);
        zero_tool += tsv.num(l, "lob_zero_total");
        zero_walker += r
            .parts()
            .filter(|p| p.rows.lob_zero_total.is_some())
            .count();
    }
    assert_eq!((zero_tool, zero_walker), (0, 0), "AQ-1");
}

/// A cheap stand-in for the property tests of task T-14, on the files that exercise the new framing (LOB
/// values, partitions, LONG): truncation and a flipped byte at many offsets, small and default buffer. Every
/// outcome is `Ok` or a typed error; nothing panics and nothing runs away.
#[test]
fn x_damaged_lob_partition_and_long_files_never_panic() {
    for name in [
        "18c/lob_exp_18c.dmp",
        "11g/lob_exp_11g.dmp",
        "18c/part_exp_direct_18c.dmp",
        "18c/part2_exp_direct_18c.dmp",
        "11g/long_exp_direct_11g.dmp",
        "11g/long_exp_11g.dmp",
        // T-19: the conventional `fd ff` chunks and the plain lengths from 0x8000, and the longer direct chains
        "11g/long3_exp_11g.dmp",
        "18c/long3_exp_direct_18c.dmp",
        "11g-cp1251/ru_exp_cp1251_11g.dmp",
        "18c/misc_exp_18c.dmp",
    ] {
        let bytes = bytes_of(name);
        let step = (bytes.len() / 150).max(37);
        for buffer in [64, oradmp_core::io::DEFAULT_BUFFER] {
            let options = WalkOptions {
                buffer_bytes: buffer,
                ..WalkOptions::default()
            };
            for cut in (0..bytes.len()).step_by(step) {
                let _ = walk(Cursor::new(bytes[..cut].to_vec()), &options);
            }
            for at in (0..bytes.len()).step_by(step) {
                for flip in [0xffu8, 0x01] {
                    let mut b = bytes.clone();
                    b[at] ^= flip;
                    let _ = walk(Cursor::new(b), &options);
                }
            }
        }
    }
}
