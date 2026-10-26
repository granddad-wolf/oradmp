//! Task T-3b: matching and checks for every part (ADR-31), `part` in errors and warnings (ADR-29), the
//! digests over the assembled LONG values and over the LOB values (ADR-29 item 6), `ColumnInfo.long` and
//! `ColumnInfo.lob`. The golden files and the log check are in `t2_manifest.rs` and `t3_catalog.rs`.
//!
//! Expected numbers: `truth_part.txt` / `truth_part2.txt` (T_PART 1095 + 1098 + 807 rows; the six subpartitions of
//! T_PART2) and the Analyst's `exp_parse.py` through `tests/golden/gen_exp_raw.py` (digests of the assembled
//! values: `exp_raw.tsv`, column `long`). Patched copies are made in memory and written below `target/t3b/`.
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

use oradmp_core::dump::{open, CatalogEnd, Dump, Options, TableRef};
use oradmp_core::exp::items::{walk, walk_path, WalkOptions, WalkReport};
use oradmp_core::{ErrorCode, WarningCode};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixtures() -> PathBuf {
    root().join("corpus/fixtures")
}

fn bytes_of(rel: &str) -> Vec<u8> {
    std::fs::read(fixtures().join(rel)).unwrap()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut s, b| {
        write!(s, "{b:02x}").unwrap();
        s
    })
}

fn walk_bytes(b: Vec<u8>, hash: bool) -> oradmp_core::Result<WalkReport> {
    walk(
        Cursor::new(b),
        &WalkOptions {
            hash_fields: hash,
            ..WalkOptions::default()
        },
    )
}

/// Writes the bytes below `target/t3b/<name>/` and opens the file.
fn open_bytes(name: &str, bytes: &[u8]) -> Dump {
    let dir = root().join("target/t3b").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join("p.dmp");
    std::fs::write(&p, bytes).unwrap();
    open(&p, Options::default()).unwrap()
}

/// Replaces the n-th (0-based) occurrence of `needle` by `with` (same length).
fn replace_nth(b: &mut [u8], needle: &[u8], with: &[u8], n: usize) {
    assert_eq!(needle.len(), with.len());
    let mut from = 0;
    let mut seen = 0;
    while let Some(i) = b[from..].windows(needle.len()).position(|w| w == needle) {
        if seen == n {
            b[from + i..from + i + needle.len()].copy_from_slice(with);
            return;
        }
        seen += 1;
        from += i + 1;
    }
    panic!("occurrence {n} of the needle not found");
}

fn put_u16(b: &mut [u8], at: u64, v: u16) {
    let at = usize::try_from(at).unwrap();
    b[at..at + 2].copy_from_slice(&v.to_le_bytes());
}

const INS: &[u8] = b"INSERT INTO \"T_PART\" (\"ID\", \"SALE_DATE\", \"AMOUNT\") VALUES";
const INS_SWAPPED: &[u8] = b"INSERT INTO \"T_PART\" (\"AMOUNT\", \"SALE_DATE\", \"ID\") VALUES";

fn order_warnings(d: &Dump) -> Vec<(Option<u32>, Option<u32>, Option<u64>)> {
    d.warnings()
        .iter()
        .filter(|w| w.code == WarningCode::UntestedLayout)
        .map(|w| (w.table, w.part, w.offset))
        .collect()
}

// ---------------------------------------------------------------------------------------------
// (e) by-name matching for every part
// ---------------------------------------------------------------------------------------------

#[test]
fn e_the_nine_parts_of_t_part_and_t_part2_have_the_identity_order() {
    let mut seen = 0usize;
    for (file, table, parts) in [
        ("18c/part_exp_18c.dmp", "T_PART", 3),
        ("18c/part_exp_direct_18c.dmp", "T_PART", 3),
        ("18c/part2_exp_18c.dmp", "T_PART2", 6),
        ("18c/part2_exp_direct_18c.dmp", "T_PART2", 6),
    ] {
        let mut d = open(fixtures().join(file), Options::default()).unwrap();
        let t = d.table(TableRef::ByName(table)).unwrap().clone();
        assert_eq!(t.parts.len(), parts, "{file}");
        for p in &t.parts {
            assert_eq!(p.file_order, [0, 1, 2], "{file}");
            seen += 1;
        }
        assert!(d.warnings().is_empty(), "{file}: {:?}", d.warnings());
    }
    assert_eq!(seen, 18, "9 parts in each of the two paths");
}

#[test]
fn e_a_later_part_in_another_order_is_matched_by_name_with_one_warning_per_table() {
    let good = bytes_of("18c/part_exp_18c.dmp");
    let report = walk_path(
        &fixtures().join("18c/part_exp_18c.dmp"),
        &Default::default(),
    )
    .unwrap();
    let inserts: Vec<u64> = report.sections[0]
        .parts
        .iter()
        .map(|p| p.insert_offset)
        .collect();
    // ID and AMOUNT have the same descriptor (NUMBER, 22), so swapping their names in the INSERT INTO line
    // of a part is a legal change of order

    // part 2 only: file order [AMOUNT, SALE_DATE, ID] = declared positions [2, 1, 0]
    let mut b = good.clone();
    replace_nth(&mut b, INS, INS_SWAPPED, 1);
    let mut d = open_bytes("order_p2", &b);
    let t = d.table(TableRef::ByOrdinal(1)).unwrap().clone();
    let orders: Vec<&[u32]> = t.parts.iter().map(|p| p.file_order.as_slice()).collect();
    assert_eq!(orders, [&[0, 1, 2][..], &[2, 1, 0][..], &[0, 1, 2][..]]);
    assert_eq!(order_warnings(&d), [(Some(1), Some(2), Some(inserts[1]))]);
    // the columns keep the declared order and the first part's file positions
    let names: Vec<&str> = t.columns.iter().map(|c| c.name.as_str()).collect();
    assert_eq!(names, ["ID", "SALE_DATE", "AMOUNT"]);
    // the reader of part 2 reads all 1,098 rows
    let mut r = d.part_rows(TableRef::ByOrdinal(1), 1).unwrap();
    let mut n = 0;
    while r.next_row().unwrap().is_some() {
        n += 1;
    }
    assert_eq!(n, 1098);

    // parts 2 and 3: still one warning for the table, at part 2
    let mut b = good.clone();
    replace_nth(&mut b, INS, INS_SWAPPED, 1);
    replace_nth(&mut b, INS, INS_SWAPPED, 1); // the next unpatched occurrence is part 3
    let mut d = open_bytes("order_p23", &b);
    let t = d.table(TableRef::ByOrdinal(1)).unwrap().clone();
    assert_eq!(t.parts[1].file_order, [2, 1, 0]);
    assert_eq!(t.parts[2].file_order, [2, 1, 0]);
    assert_eq!(order_warnings(&d), [(Some(1), Some(2), Some(inserts[1]))]);

    // parts 1 and 2: the first part's order is the table's "other order", warned once (part 1); the
    // declared order is that of CREATE TABLE
    let mut b = good.clone();
    replace_nth(&mut b, INS, INS_SWAPPED, 0);
    replace_nth(&mut b, INS, INS_SWAPPED, 0);
    let mut d = open_bytes("order_p12", &b);
    let t = d.table(TableRef::ByOrdinal(1)).unwrap().clone();
    let orders: Vec<&[u32]> = t.parts.iter().map(|p| p.file_order.as_slice()).collect();
    assert_eq!(orders, [&[2, 1, 0][..], &[2, 1, 0][..], &[0, 1, 2][..]]);
    // the first part's warning stands at the CREATE TABLE line (the order is that of its columns)
    assert_eq!(
        order_warnings(&d),
        [(Some(1), Some(1), Some(t.create_offset))]
    );
    let positions: Vec<u32> = t.columns.iter().map(|c| c.file_position).collect();
    assert_eq!(positions, [3, 2, 1]);

    // control: the unpatched file has none
    let d = open_bytes("order_none", &good);
    assert!(order_warnings(&d).is_empty());
}

#[test]
fn e_a_part_whose_names_or_descriptors_differ_from_the_first_part_is_framing_at_that_part() {
    let file = "18c/part_exp_18c.dmp";
    let good = bytes_of(file);
    let report = walk_path(&fixtures().join(file), &Default::default()).unwrap();
    let part2 = &report.sections[0].parts[1];
    let check = |name: &str, b: Vec<u8>| {
        let mut d = open_bytes(name, &b);
        let e = d.scan().unwrap_err();
        assert_eq!(e.code, ErrorCode::Framing, "{name}");
        assert_eq!(
            (e.table, e.part, e.offset),
            (Some(1), Some(2), Some(part2.insert_offset)),
            "{name}"
        );
        assert!(e.to_string().contains("part 2"), "{e}");
        assert!(!e.to_string().contains("SALE"), "no name in a message");
    };
    // a name that the first part does not have
    let mut b = good.clone();
    replace_nth(
        &mut b,
        INS,
        b"INSERT INTO \"T_PART\" (\"ID\", \"SALE_DATX\", \"AMOUNT\") VALUES",
        1,
    );
    check("names", b);
    // the same names, but ID and SALE_DATE swapped: per name the descriptor differs (NUMBER against DATE)
    let mut b = good.clone();
    replace_nth(
        &mut b,
        INS,
        b"INSERT INTO \"T_PART\" (\"SALE_DATE\", \"ID\", \"AMOUNT\") VALUES",
        1,
    );
    check("swapped_unequal", b);
    // the descriptor of one column: length 22 -> 21
    let mut b = good.clone();
    put_u16(&mut b, part2.header.columns[2].offset + 2, 21);
    check("descriptor", b);
    // the type code of one column: NUMBER -> VARCHAR2 would change the row layout; use RAW (23)
    let mut b = good.clone();
    put_u16(&mut b, part2.header.columns[2].offset, 23);
    check("type", b);
    // control: nothing patched scans
    let mut d = open_bytes("framing_control", &good);
    assert!(matches!(d.scan().unwrap().end, CatalogEnd::Complete));
}

#[test]
fn e_parts_may_differ_in_path_flag() {
    // T10: each part has its own flag. Part 2 of T_PART conventional gets flag 1 in memory; rows are then
    // read as a direct-path part, so the walk breaks later, but the check of the columns does not refuse the
    // part for its flag (the error, if any, is not Framing at the INSERT INTO line).
    let file = "18c/part_exp_18c.dmp";
    let mut b = bytes_of(file);
    let report = walk_path(&fixtures().join(file), &Default::default()).unwrap();
    let flag = report.sections[0].parts[1].header.flag_offset;
    put_u16(&mut b, flag, 1);
    match walk_bytes(b, false) {
        Ok(_) => panic!("rows read with the wrong path flag walk"),
        Err(e) => assert_ne!(
            e.offset,
            Some(report.sections[0].parts[1].insert_offset),
            "{e}"
        ),
    }
}

// ---------------------------------------------------------------------------------------------
// (f) `part` on errors and warnings; `column` is the file position
// ---------------------------------------------------------------------------------------------

#[test]
fn f_an_error_in_the_rows_of_a_part_names_table_part_row_and_column() {
    let file = "18c/part_exp_18c.dmp";
    let report = walk_path(&fixtures().join(file), &Default::default()).unwrap();
    for k in 0..3usize {
        // the first u16 of the first row of part k+1 becomes 0, a length that never occurs [R8]
        let mut b = bytes_of(file);
        put_u16(&mut b, report.sections[0].parts[k].header.rows_offset, 0);
        let e = walk_bytes(b, false).unwrap_err();
        assert_eq!(e.code, ErrorCode::Framing);
        assert_eq!(
            (e.table, e.part, e.row, e.column),
            (
                Some(1),
                Some(u32::try_from(k + 1).unwrap()),
                Some(1),
                Some(1)
            )
        );
        assert!(e.to_string().contains(&format!("part {}", k + 1)), "{e}");
    }
    // an unpartitioned table: part 1
    let file = "18c/t_nullmix_exp_18c.dmp";
    let report = walk_path(&fixtures().join(file), &Default::default()).unwrap();
    let mut b = bytes_of(file);
    put_u16(&mut b, report.sections[0].parts[0].header.rows_offset, 0);
    let e = walk_bytes(b, false).unwrap_err();
    assert_eq!((e.table, e.part, e.row), (Some(1), Some(1), Some(1)));
}

#[test]
fn f_warnings_of_a_row_and_of_a_part_carry_the_part() {
    // a piece of length 0 in the LONG chain of T_LONG row 3 (the technique of t3c_walker.rs (d)): `00 80`
    // before the second piece. Offsets of exp.md 6.6: second piece at 0x2288 + 0x9e on 18c.
    let file = "18c/long_exp_direct_18c.dmp";
    let good = bytes_of(file);
    let report = walk_path(&fixtures().join(file), &Default::default()).unwrap();
    let p = &report.sections[0].parts[0];
    // the offset of the first piece of row 3 is the chain's start: take the second piece from the bytes
    let first = p.rows.long[0].first_zero_piece.map_or_else(
        || {
            // no zero piece: find row 3's chain by its first u16 0x809c
            let needle = 0x809cu16.to_le_bytes();
            good.windows(2).position(|w| w == needle).unwrap() as u64
        },
        |x| x.1,
    );
    let second = first + 2 + 156;
    let mut b = good.clone();
    b.splice(
        usize::try_from(second).unwrap()..usize::try_from(second).unwrap(),
        [0x00u8, 0x80],
    );
    let cut = b.len() - 2;
    assert!(b[cut..].iter().all(|&x| x == 0));
    b.truncate(cut);
    let r = walk_bytes(b, false).unwrap();
    assert_eq!(r.warnings.len(), 1, "{:?}", r.warnings);
    let w = &r.warnings[0];
    assert_eq!(w.code, WarningCode::UntestedLayout);
    assert_eq!(
        (w.table, w.part, w.row, w.column),
        (Some(1), Some(1), Some(3), Some(3))
    );
    assert!(w.to_string().contains("part 1"));

    // a part that does not directly follow a naming line: the warning belongs to part 2
    let good = bytes_of("18c/part2_exp_18c.dmp");
    let r0 = walk_bytes(good.clone(), false).unwrap();
    let mut b = good.clone();
    replace_nth(
        &mut b,
        b"\nSUBPARTITION \"SYS_SUBP369\"\n",
        b"\nXUBPARTITION \"SYS_SUBP369\"\n",
        0,
    );
    let r = walk_bytes(b, false).unwrap();
    assert_eq!(r.warnings.len(), 1);
    assert_eq!(
        (r.warnings[0].part, r.warnings[0].offset),
        (Some(2), Some(r0.sections[0].parts[1].insert_offset))
    );
}

#[test]
fn f_the_column_of_a_ddl_warning_is_the_file_position() {
    // T_LONG of the direct files: CREATE TABLE has (ID, L LONG, V VARCHAR2(10)); the file has (ID, V, L).
    // `V VARCHAR2(10)` -> `VARCHAR2(11)` disagrees with the descriptor length: W-DDL-FALLBACK for V, which is
    // declared position 3 and file position 2 on the direct path, 3 and 3 on the conventional one.
    for (file, want) in [
        ("18c/long_exp_direct_18c.dmp", 2u32),
        ("18c/long_exp_18c.dmp", 3u32),
        ("11g/long_exp_direct_11g.dmp", 2u32),
    ] {
        let mut b = bytes_of(file);
        replace_nth(&mut b, b"\"V\" VARCHAR2(10)", b"\"V\" VARCHAR2(11)", 0);
        let mut d = open_bytes(&format!("ddl_col_{want}_{}", &file[..3]), &b);
        d.scan().unwrap();
        let w: Vec<_> = d
            .warnings()
            .iter()
            .filter(|w| w.code == WarningCode::DdlFallback)
            .collect();
        assert_eq!(w.len(), 1, "{file}: {w:?}");
        assert_eq!((w[0].table, w[0].column), (Some(1), Some(want)), "{file}");
        let t = d.table(TableRef::ByOrdinal(1)).unwrap();
        let v = t.columns.iter().find(|c| c.name == "V").unwrap();
        assert_eq!((v.position, v.file_position), (3, want), "{file}");
    }
}

// ---------------------------------------------------------------------------------------------
// (g) digests: raw chains stay, assembled LONG values, LOB values
// ---------------------------------------------------------------------------------------------

fn long_digests(rel: &str) -> BTreeMap<String, Vec<(u32, u64, String)>> {
    let r = walk_path(
        &fixtures().join(rel),
        &WalkOptions {
            hash_fields: true,
            ..WalkOptions::default()
        },
    )
    .unwrap();
    let mut out = BTreeMap::new();
    for s in &r.sections {
        for p in &s.parts {
            if p.rows.long.is_empty() {
                continue;
            }
            let v: Vec<(u32, u64, String)> = p
                .rows
                .long
                .iter()
                .zip(&p.rows.long_sha256)
                .map(|(st, h)| (st.column, st.values, hex(h)))
                .collect();
            out.insert(String::from_utf8(s.name_raw.clone()).unwrap(), v);
        }
    }
    out
}

#[test]
fn g_the_conventional_and_the_direct_export_give_equal_assembled_digests() {
    // gen_exp_raw.py (exp_parse.py): T_LONG 2 values, T_LONGRAW 2 values on both paths; the long2 files
    // have a LONG column too
    let mut compared = 0usize;
    for v in ["18c", "11g"] {
        for base in ["long", "long2"] {
            let conv = long_digests(&format!("{v}/{base}_exp_{v}.dmp"));
            let direct = long_digests(&format!("{v}/{base}_exp_direct_{v}.dmp"));
            assert!(!conv.is_empty(), "{v} {base}");
            // the direct export of `long` has one more table, T_LONG_EMPTY (exp.md 6.6)
            for (table, c) in &conv {
                let d = &direct[table];
                assert_eq!(c.len(), d.len(), "{v} {base} {table}");
                for (x, y) in c.iter().zip(d) {
                    // the file position of the LONG column differs (it stands last on the direct path)
                    assert_eq!((x.1, &x.2), (y.1, &y.2), "{v} {base} {table}");
                    compared += 1;
                }
            }
        }
    }
    // T_LONG, T_LONGRAW (long) and T_LONG2 (long2) in two versions, as exp_raw.tsv lists them
    assert_eq!(compared, 6);
    // the digest of T_LONG in long_exp_18c, from exp_raw.tsv (gen_exp_raw.py, exp_parse.py): 2 values
    assert_eq!(
        long_digests("18c/long_exp_18c.dmp")["T_LONG"],
        [(
            2,
            2,
            "809fe03848945c0d23e08582c18cb28ea1546a6617e6b8b8c461e4df686c92fa".to_string()
        )]
    );
}

#[test]
fn g_a_changed_byte_in_a_long_value_changes_its_digests_and_a_changed_lob_byte_only_the_lob_digest()
{
    let opts = |h| WalkOptions {
        hash_fields: h,
        ..WalkOptions::default()
    };
    // LONG: the last piece's last byte of the 5-piece chain, direct path; the raw digest changes too
    let file = "18c/long_exp_direct_18c.dmp";
    let good = bytes_of(file);
    let r0 = walk(Cursor::new(good.clone()), &opts(true)).unwrap();
    let p0 = &r0.sections[0].parts[0];
    // the end marker of the rows stands behind the last value: the byte before it is a LONG RAW or LONG byte
    // of the last row of T_LONG; flip it
    let mut b = good.clone();
    let at = usize::try_from(p0.rows.end_marker_offset).unwrap() - 1;
    b[at] ^= 0x01;
    let r1 = walk(Cursor::new(b), &opts(true)).unwrap();
    let p1 = &r1.sections[0].parts[0];
    assert_ne!(p1.rows.long_sha256, p0.rows.long_sha256);
    assert_ne!(p1.rows.raw_sha256, p0.rows.raw_sha256);
    assert_eq!(
        p1.rows.long[0], p0.rows.long[0],
        "the statistics do not change"
    );
    // without hashing there are no digests
    let r2 = walk(Cursor::new(good), &opts(false)).unwrap();
    assert!(r2.sections[0].parts[0].rows.long_sha256.is_empty());

    // LOB: the first value of the first LOB table of lob_exp_18c
    let file = "18c/lob_exp_18c.dmp";
    let good = bytes_of(file);
    let r0 = walk(
        Cursor::new(good.clone()),
        &WalkOptions {
            hash_fields: true,
            record_lob: true,
            ..WalkOptions::default()
        },
    )
    .unwrap();
    let (s, p) = r0
        .sections
        .iter()
        .flat_map(|s| s.parts.iter().map(move |p| (s, p)))
        .find(|(_, p)| p.rows.lob.short > 0 && p.rows.lob.stored_bytes > 0)
        .unwrap();
    let v = p
        .rows
        .lob_detail
        .as_ref()
        .unwrap()
        .values
        .iter()
        .find(|v| v.total > 0 && v.chunks.is_empty())
        .unwrap();
    let mut b = good.clone();
    b[usize::try_from(v.offset).unwrap() + 2] ^= 0x01;
    let r1 = walk(
        Cursor::new(b),
        &WalkOptions {
            hash_fields: true,
            ..WalkOptions::default()
        },
    )
    .unwrap();
    let q = r1
        .sections
        .iter()
        .find(|x| x.ordinal == s.ordinal)
        .unwrap()
        .parts
        .first()
        .unwrap();
    assert_ne!(q.rows.lob_sha256, p.rows.lob_sha256);
    assert_eq!(
        q.rows.raw_sha256, p.rows.raw_sha256,
        "LOB values are not in the raw digest"
    );
    assert_eq!(q.rows.lob, p.rows.lob);
}

// ---------------------------------------------------------------------------------------------
// (h) ColumnInfo.long and ColumnInfo.lob
// ---------------------------------------------------------------------------------------------

#[test]
fn h_column_info_long_and_lob_equal_the_statistics_of_the_walk() {
    // long_exp_direct (exp.md 6.6): T_LONG L has 2 values, one chained (5 pieces, 32,000 bytes);
    // T_LONGRAW LR the same; LOB values: lob_exp
    let mut d = open(
        fixtures().join("18c/long_exp_direct_18c.dmp"),
        Options::default(),
    )
    .unwrap();
    let t = d.table(TableRef::ByName("T_LONG")).unwrap().clone();
    let l = t.columns.iter().find(|c| c.name == "L").unwrap();
    let st = l.long.as_ref().unwrap();
    assert_eq!(st.column, l.position);
    assert_eq!((st.values, st.chained, st.largest), (2, 1, 32_000));
    assert_eq!(st.above_32000, 0);
    assert!(t
        .columns
        .iter()
        .filter(|c| c.name != "L")
        .all(|c| c.long.is_none()));
    assert!(t.columns.iter().all(|c| c.lob.is_none()));

    // N-2 (review 2): the statistics of `ColumnInfo.lob` against `tests/data/lob_expected.tsv` (the Analyst's
    // `exp_parse.py` through `gen_walk_expected.py`: one `row:col:offset:form:len:chunks` entry per LOB
    // value, `col` = the file position), not against the walker of the reader. lob_exp_18c has exactly 5
    // LOB columns: T_LOB C and B, T_LOB2 A, N and B.
    let mut d = open(fixtures().join("18c/lob_exp_18c.dmp"), Options::default()).unwrap();
    let text = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/lob_expected.tsv"),
    )
    .unwrap();
    let mut lines = text.lines().filter(|l| !l.starts_with('#'));
    let head: Vec<&str> = lines.next().unwrap().split('\t').collect();
    let col = |name: &str| head.iter().position(|h| *h == name).unwrap();
    let (c_file, c_table, c_values) = (col("file"), col("table"), col("values"));
    let tsv_rows: Vec<Vec<&str>> = lines.map(|l| l.split('\t').collect()).collect();
    let mut lob_columns = 0usize;
    let mut seen_tables = Vec::new();
    let tables = d.tables().unwrap().to_vec();
    for l in tsv_rows
        .iter()
        .filter(|l| l[c_file] == "18c/lob_exp_18c.dmp")
    {
        let name = l[c_table];
        seen_tables.push(name.to_string());
        let t = tables.iter().find(|t| t.name == name).unwrap();
        // (column, values, short, empty, long, stored bytes, largest) from the tool's entries
        let mut want: BTreeMap<u32, [u64; 6]> = BTreeMap::new();
        for e in l[c_values].split(';') {
            let f: Vec<&str> = e.split(':').collect();
            let (col, form, len): (u32, &str, u64) =
                (f[1].parse().unwrap(), f[3], f[4].parse().unwrap());
            let w = want.entry(col).or_default();
            w[0] += 1;
            match form {
                "short" => {
                    w[1] += 1;
                    w[2] += u64::from(len == 0);
                }
                "long" => w[3] += 1,
                other => panic!("form {other}"),
            }
            w[4] += len;
            w[5] = w[5].max(len);
        }
        for c in &t.columns {
            assert_eq!(c.lob.is_some(), c.type_code == 112 || c.type_code == 113);
            let w = want.get(&c.file_position);
            assert_eq!(c.lob.is_some(), w.is_some(), "{name} {}", c.name);
            if let (Some(s), Some(w)) = (&c.lob, w) {
                lob_columns += 1;
                assert_eq!(
                    [
                        s.values,
                        s.short,
                        s.empty,
                        s.long,
                        s.stored_bytes,
                        s.largest
                    ],
                    *w,
                    "{name} {}",
                    c.name
                );
            }
        }
    }
    assert_eq!(seen_tables, ["T_LOB", "T_LOB2"]);
    assert_eq!(lob_columns, 5, "T_LOB C and B, T_LOB2 A, N and B");
}
