//! Task T-8: the direct path (path flag 1 [T6, R4, R7]) and both empty-table forms [E1 to E3], tested through
//! `Dump::read_table`.
//!
//! Numbers: the pair comparison needs no external number (conventional against `direct=y`, same database, same
//! data). The part sizes come from `truth_part.txt` (`P|P2019|1095`, `P|P2020|1098`, `P|PMAX|807`) and
//! `truth_part2.txt` (`SP` lines: 21, 29, 0, 0, 21, 29; the empty subpartitions give no batch), as in
//! `t6_parts.rs`. The 18c / 11g difference of the empty-table form (no `ff ff` / `ff ff`) is that of
//! `docs/formats/exp_reader_rules.md` [E1, E2]; it is checked here against the fixtures `t_empty_exp_direct_<v>`
//! (bytes `0a 4d` / `ff ff` behind the path flag, `xxd`), and the catalog.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::path::PathBuf;

use arrow_array::RecordBatch;
use common::*;
use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::exp::rows::EndMarker;
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::values::number::OverflowPolicy;
use oradmp_core::{ErrorCode, WarningCode};

const ERRORS: usize = 0;
const FOLDERS: [&str; 3] = ["18c", "11g", "11g-cp1251"];

fn direct_files() -> Vec<(String, String)> {
    let mut out = Vec::new();
    for f in FOLDERS {
        let mut names: Vec<String> = std::fs::read_dir(fixtures().join(f))
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|n| n.contains("_exp_direct_") && n.ends_with(".dmp"))
            .collect();
        names.sort();
        for n in names {
            let conv = n.replace("_exp_direct_", "_exp_");
            out.push((format!("{f}/{conv}"), format!("{f}/{n}")));
        }
    }
    out
}

fn sorted(mut r: Vec<Vec<String>>) -> Vec<Vec<String>> {
    r.sort();
    r
}

fn batches_equal(a: &[RecordBatch], b: &[RecordBatch], what: &str) {
    assert_eq!(a.len(), b.len(), "{what}: number of batches");
    for (x, y) in a.iter().zip(b) {
        assert_eq!(x.schema(), y.schema(), "{what}");
        assert_eq!(x.num_rows(), y.num_rows(), "{what}");
        for (cx, cy) in x.columns().iter().zip(y.columns()) {
            assert_eq!(cx.to_data(), cy.to_data(), "{what}");
        }
    }
}

/// Families whose batches (not only the multisets of rows) must be equal: LOB (flag 0 in both files [L6]),
/// LONG, LONG2, t_probe11 and part2 (declared column order, ADR-24).
fn batch_family(file: &str) -> bool {
    [
        "/lob_",
        "/long_",
        "/long2_",
        "/t_probe11_",
        "/part2_",
        "/part_",
    ]
    .iter()
    .any(|p| file.contains(p))
}

/// Every `*_exp_direct_*` file of the three folders against its conventional partner: the same tables and
/// the same multiset of rows (and for the families above the same batches), or the same error. The number
/// of pairs is that of the directory listing (checked against 94 = the `*_exp_direct_*` files).
#[test]
fn every_direct_file_equals_its_conventional_partner() {
    let pairs = direct_files();
    // T-19b: 85 + 9 (long4, long5 x 2 versions, longraw4 x 1, longraw5 x 2, tsltz2 x 2)
    assert_eq!(pairs.len(), 94, "number of direct files");
    let mut tables = 0;
    let mut errors = 0;
    let mut rows_total = 0;
    for (conv, direct) in &pairs {
        let mut dc = open_fixture(conv);
        let mut dd = open_fixture(direct);
        let tc: Vec<(String, usize)> = dc
            .tables()
            .unwrap()
            .iter()
            .map(|t| (t.name.clone(), t.parts.len()))
            .collect();
        let td: Vec<(String, usize)> = dd
            .tables()
            .unwrap()
            .iter()
            .map(|t| (t.name.clone(), t.parts.len()))
            .collect();
        // the one difference between the families: `long_exp_direct_<v>` also holds the empty table
        // T_LONG_EMPTY (its log lists it, the log of `long_exp_<v>` does not)
        let td: Vec<_> = td
            .into_iter()
            .filter(|(n, _)| {
                if n == "T_LONG_EMPTY" {
                    assert!(direct.contains("/long_exp_direct_"), "{direct}");
                    let b = read_default(direct, n).0;
                    assert_eq!(row_count(&b), 0, "{direct}");
                    false
                } else {
                    true
                }
            })
            .collect();
        assert_eq!(tc, td, "{direct}: tables");
        // the path of the direct file is 1 for every part that is not a LOB table (flag 0 [L6])
        for (name, _) in &tc {
            // the value errors of the fixtures (T_DATE rows 8, 9, NUMBER extremes, ...) become NULL, so that the
            // rows around them are compared as well
            let o = ReadOptions {
                on_value_error: OnValueError::Null,
                on_number_overflow: OverflowPolicy::Null,
                ..ReadOptions::default()
            };
            let a = try_read(&mut dc, name, &o);
            let b = try_read(&mut dd, name, &o);
            match (a, b) {
                (Ok((ba, wa)), Ok((bb, wb))) => {
                    assert_eq!(sorted(rows(&ba)), sorted(rows(&bb)), "{direct} {name}");
                    if batch_family(direct) {
                        batches_equal(&ba, &bb, &format!("{direct} {name}"));
                        assert_eq!(rows(&ba), rows(&bb), "{direct} {name}");
                    } else {
                        assert_eq!(row_count(&ba), row_count(&bb));
                    }
                    let ca: Vec<_> = wa.iter().map(|w| w.code).collect();
                    let cb: Vec<_> = wb.iter().map(|w| w.code).collect();
                    assert_eq!(ca, cb, "{direct} {name}: warning codes");
                    rows_total += row_count(&ba);
                }
                (Err(ea), Err(eb)) => {
                    errors += 1;
                    assert_eq!(ea.code, eb.code, "{direct} {name}");
                }
                (a, b) => panic!(
                    "{direct} {name}: one side failed: {:?} / {:?}",
                    a.err().map(|e| e.code),
                    b.err().map(|e| e.code)
                ),
            }
            tables += 1;
        }
    }
    assert_eq!(
        errors, ERRORS,
        "tables that fail the same way on both paths"
    );
    // N-9 (T-14 g): exact. python over `tests/golden/exp_catalog.tsv` (the Analyst's exp_parse.py through
    // gen_exp_catalog.py): the 85 conventional partners of the direct files hold 150 table sections and the sum of
    // their `num_rows` is 16,497 (the 85 direct files hold 152 sections, the two T_LONG_EMPTY of `long_exp_direct_*`
    // more, and the same 16,497 rows). T-19: the long3, tsp and seg files add 9 sections and 36 rows. T-19b: the 9
    // new direct files (long4, long5, longraw4, longraw5, tsltz2) add 16 sections and 105 rows (python over the same
    // golden file: 94 partners, 166 sections, 16,602 rows).
    assert_eq!(tables, 166, "tables compared");
    assert_eq!(rows_total, 16_602, "rows compared");
}

/// The path of each part: `direct=y` files give flag 1 for ordinary tables, conventional files flag 0; LOB
/// tables stay on flag 0 in both [L6]. Without this the comparison above could compare a file with itself.
#[test]
fn the_direct_files_really_use_path_flag_1() {
    use oradmp_core::dump::TablePath;
    for v in ["18c", "11g"] {
        let mut c = open_fixture(&format!("{v}/t_num_exp_{v}.dmp"));
        let mut d = open_fixture(&format!("{v}/t_num_exp_direct_{v}.dmp"));
        assert_eq!(c.tables().unwrap()[0].path, TablePath::Conventional);
        assert_eq!(d.tables().unwrap()[0].path, TablePath::Direct);
        let mut l = open_fixture(&format!("{v}/lob_exp_direct_{v}.dmp"));
        for t in l.tables().unwrap() {
            let want = if t.name == "T_FRAME" {
                TablePath::Direct
            } else {
                TablePath::Conventional
            };
            assert_eq!(t.path, want, "{v} {}", t.name);
        }
    }
}

/// T_PART (3 parts: 1,095, 1,098, 807 rows) and T_PART2 (4 batches 21, 29, 21, 29; the two empty
/// subpartitions give no batch) read the same on both paths, per part.
#[test]
fn partitioned_pairs_give_equal_batches_per_part() {
    let a = read_default("18c/part_exp_18c.dmp", "T_PART").0;
    let b = read_default("18c/part_exp_direct_18c.dmp", "T_PART").0;
    let sizes: Vec<_> = a.iter().map(RecordBatch::num_rows).collect();
    assert_eq!(sizes, [1095, 1098, 807]);
    batches_equal(&a, &b, "T_PART");
    let a = read_default("18c/part2_exp_18c.dmp", "T_PART2").0;
    let b = read_default("18c/part2_exp_direct_18c.dmp", "T_PART2").0;
    let sizes: Vec<_> = a.iter().map(RecordBatch::num_rows).collect();
    assert_eq!(sizes, [21, 29, 21, 29]);
    batches_equal(&a, &b, "T_PART2");
}

/// `lob_exp_direct_<v>` against `lob_exp_<v>`: equal batches (flag 0 in both [L6]); `long` and `long2` pairs
/// in declared column order (the direct files move L last, ADR-24); `t_probe11`.
#[test]
fn lob_long_and_probe_pairs_give_equal_batches_in_declared_order() {
    for v in ["18c", "11g"] {
        for fam in ["lob", "long", "long2", "t_probe11"] {
            let c = format!("{v}/{fam}_exp_{v}.dmp");
            let d = format!("{v}/{fam}_exp_direct_{v}.dmp");
            let names: Vec<String> = open_fixture(&c)
                .tables()
                .unwrap()
                .iter()
                .map(|t| t.name.clone())
                .collect();
            assert!(!names.is_empty());
            for t in names {
                let a = read_default(&c, &t).0;
                let b = read_default(&d, &t).0;
                batches_equal(&a, &b, &format!("{d} {t}"));
                assert!(
                    row_count(&a) > 0 || fam == "lob" || fam == "long",
                    "{d} {t}"
                );
            }
        }
    }
    // the LONG column of T_LONG stands where it was declared, in both files
    let b = read_default("18c/long_exp_direct_18c.dmp", "T_LONG").0;
    let c = read_default("18c/long_exp_18c.dmp", "T_LONG").0;
    let names = |x: &[RecordBatch]| -> Vec<String> {
        x[0].schema()
            .fields()
            .iter()
            .map(|f| f.name().clone())
            .collect()
    };
    assert_eq!(names(&b), names(&c));
}

// ---------------------------------------------------------------------------------------------
// empty tables [E1 to E3]
// ---------------------------------------------------------------------------------------------

/// Empty tables give no rows on both versions and both paths; the direct form of 18c has no `ff ff`
/// (`LfOnly`), that of 11g has (`FfFf`), and the conventional form of both has. Several empty tables of
/// `multi_empty*` stand in one file.
#[test]
fn empty_tables_give_no_rows_in_both_forms() {
    let mut seen_lf_only = 0;
    let mut seen_ff = 0;
    for v in ["18c", "11g"] {
        for fam in ["t_empty", "multi_empty", "multi_empty2"] {
            for direct in [false, true] {
                let file = if direct {
                    format!("{v}/{fam}_exp_direct_{v}.dmp")
                } else {
                    format!("{v}/{fam}_exp_{v}.dmp")
                };
                let mut d = open_fixture(&file);
                let tables = d.tables().unwrap().to_vec();
                let mut empty = 0;
                for t in &tables {
                    if t.num_rows != 0 {
                        continue;
                    }
                    empty += 1;
                    for p in &t.parts {
                        match p.end_marker {
                            EndMarker::LfOnly => {
                                assert!(direct && v == "18c", "{file} {}", t.name);
                                seen_lf_only += 1;
                            }
                            EndMarker::FfFf => {
                                assert!(!(direct && v == "18c"), "{file} {}", t.name);
                                seen_ff += 1;
                            }
                        }
                    }
                    let (b, w) = read(&mut d, &t.name, &ReadOptions::default());
                    assert_eq!(row_count(&b), 0, "{file} {}", t.name);
                    assert!(w.is_empty(), "{file} {}: {w:?}", t.name);
                }
                assert!(empty >= 1, "{file}: no empty table");
                assert!(d.warnings().is_empty(), "{file}: {:?}", d.warnings());
            }
        }
    }
    assert!(
        seen_lf_only >= 3 && seen_ff >= 3,
        "{seen_lf_only} {seen_ff}"
    );
}

fn patched(rel: &str, name: &str, patch: impl FnOnce(&mut Vec<u8>)) -> PathBuf {
    let mut data = std::fs::read(fixtures().join(rel)).unwrap();
    patch(&mut data);
    let p = target_dir("t8").join(name);
    std::fs::write(&p, data).unwrap();
    p
}

fn find(data: &[u8], pattern: &[u8]) -> usize {
    data.windows(pattern.len())
        .position(|w| w == pattern)
        .unwrap()
}

/// The decision does not look at the version line [E3]: the two versions' files with the version line of the
/// other one give the same end marker and rows.
#[test]
fn the_empty_table_decision_does_not_use_the_version_line() {
    for (v, from, to) in [
        ("18c", &b"EXPORT:V18.00.00"[..], &b"EXPORT:V11.02.00"[..]),
        ("11g", &b"EXPORT:V11.02.00"[..], &b"EXPORT:V18.00.00"[..]),
    ] {
        let rel = format!("{v}/t_empty_exp_direct_{v}.dmp");
        let mut orig = open_fixture(&rel);
        let want = orig.table(TableRef::ByName("T_EMPTY")).unwrap().parts[0].end_marker;
        let p = patched(&rel, &format!("swapped_version_{v}.dmp"), |d| {
            let at = find(d, from);
            d[at..at + to.len()].copy_from_slice(to);
        });
        let mut d = open(&p, Options::default()).unwrap();
        let t = d.table(TableRef::ByName("T_EMPTY")).unwrap().clone();
        assert_eq!(t.parts[0].end_marker, want, "{v}");
        assert_eq!(t.num_rows, 0);
        let (b, _) = read(&mut d, "T_EMPTY", &ReadOptions::default());
        assert_eq!(row_count(&b), 0);
    }
    // the markers differ between the versions, so the test above can fail
    let m = |v: &str| {
        open_fixture(&format!("{v}/t_empty_exp_direct_{v}.dmp"))
            .table(TableRef::ByName("T_EMPTY"))
            .unwrap()
            .parts[0]
            .end_marker
    };
    assert_eq!(m("18c"), EndMarker::LfOnly);
    assert_eq!(m("11g"), EndMarker::FfFf);
}

/// A synthetic ambiguous case [E3]: the bytes behind the path flag of `t_empty_exp_direct_18c.dmp` are
/// `0a 4d` (LF, `M` of `METRICST`). Test (a) "LF and an item follows" holds; test (b) "the u16 is larger than
/// the maximum of column 1" fails when the descriptor length of column 1 (22, `16 00`, at `INSERT INTO ...
/// (:1, :2, :3)`, `03 00 02 00 16 00`) is raised to 30,000. Or (b) holds and (a) fails when `M` becomes `m`.
/// Exactly one test holds: `Framing` at the LF, through `read_table`.
#[test]
fn a_synthetic_ambiguous_empty_table_is_framing() {
    let rel = "18c/t_empty_exp_direct_18c.dmp";
    let marker_pos = |d: &[u8]| find(d, b"\n\x03\x00\x02\x00\x16\x00");
    let lf_pos = |d: &[u8]| find(d, b"\x00\x00\x01\x00\n").checked_add(4).unwrap();
    for (name, patch) in [
        (
            "ambiguous_a_only.dmp",
            Box::new(|d: &mut Vec<u8>| {
                let at = marker_pos(d) + 5;
                d[at..at + 2].copy_from_slice(&30_000u16.to_le_bytes());
            }) as Box<dyn Fn(&mut Vec<u8>)>,
        ),
        (
            "ambiguous_b_only.dmp",
            Box::new(|d: &mut Vec<u8>| {
                let at = lf_pos(d);
                assert_eq!(d[at + 1], b'M');
                d[at + 1] = b'm';
            }),
        ),
    ] {
        let lf = {
            let d = std::fs::read(fixtures().join(rel)).unwrap();
            lf_pos(&d) as u64
        };
        let p = patched(rel, name, |d| patch(d));
        let mut d = open(&p, Options::default()).unwrap();
        let outcome = match d.scan() {
            Err(e) => Err(e),
            Ok(_) => d
                .read_table(TableRef::ByName("T_EMPTY"), &ReadOptions::default())
                .map(|_| ()),
        };
        let e = outcome.unwrap_err();
        assert_eq!(e.code, ErrorCode::Framing, "{name}");
        assert_eq!(e.offset, Some(lf), "{name}");
        assert_eq!(e.table, Some(1), "{name}");
        // the message names no value from the file
        assert!(!e.to_string().contains("METRIC"), "{name}");
    }
}

// ---------------------------------------------------------------------------------------------
// ADR-22: a single-column direct-path LONG table, through read_table
// ---------------------------------------------------------------------------------------------

/// A copy of `t_empty_exp_direct_18c.dmp` whose table section is replaced by `T_ONE` with the single column
/// `L` of type `code` (8 LONG, 24 LONG RAW) on the direct path; `rows` stands between the path flag and the
/// next item. The structure follows `tests/t3c_walker.rs` (e).
fn single_column_file(code: u16, rows: &[u8]) -> PathBuf {
    let d = std::fs::read(fixtures().join("18c/t_empty_exp_direct_18c.dmp")).unwrap();
    let from = find(&d, b"TABLE \"T_EMPTY\"\n");
    let to = find(&d, b"METRICSTreferential");
    let ty = if code == 8 { "LONG" } else { "LONG RAW" };
    let mut b = d[..from].to_vec();
    b.extend_from_slice(b"TABLE \"T_ONE\"\n");
    b.extend_from_slice(
        format!(
            "CREATE TABLE \"T_ONE\" (\"L\" {ty})  PCTFREE 10 PCTUSED 40 INITRANS 1 MAXTRANS 255 \
             TABLESPACE \"USERS\" LOGGING NOCOMPRESS\n"
        )
        .as_bytes(),
    );
    b.extend_from_slice(b"INSERT INTO \"T_ONE\" (\"L\") VALUES (:1)\n");
    b.extend_from_slice(&1u16.to_le_bytes());
    b.extend_from_slice(&code.to_le_bytes());
    b.extend_from_slice(&0u16.to_le_bytes());
    b.extend_from_slice(&[0, 0, 1, 0]);
    b.extend_from_slice(rows);
    let tail_end = d.iter().rposition(|&x| x != 0).unwrap() + 1;
    b.extend_from_slice(&d[to..tail_end]);
    while !b.len().is_multiple_of(8192) {
        b.push(0);
    }
    let p = target_dir("t8").join(format!("single_{code}_{}.dmp", rows.len()));
    std::fs::write(&p, b).unwrap();
    p
}

/// ADR-22 by default: "no rows" with `W-UNTESTED-LAYOUT` when an item line follows the LF; rows when it does
/// not (a value `0a 41 ...` is read, the first byte is a length byte). Both through `read_table`.
#[test]
fn a_single_column_direct_long_table_follows_adr_22_by_default() {
    for code in [8u16, 24] {
        // "no rows"
        let p = single_column_file(code, b"\n");
        let mut d = open(&p, Options::default()).unwrap();
        d.scan().unwrap();
        let (b, w) = {
            let mut r = d
                .read_table(TableRef::ByName("T_ONE"), &ReadOptions::default())
                .unwrap();
            let mut n = 0;
            while let Some(x) = r.next_batch().unwrap() {
                n += x.num_rows();
            }
            (n, r.warnings().to_vec())
        };
        assert_eq!(b, 0, "{code}");
        let all: Vec<_> = d
            .warnings()
            .iter()
            .chain(w.iter())
            .map(|x| x.code)
            .collect();
        assert!(
            all.contains(&WarningCode::UntestedLayout),
            "{code}: {all:?}"
        );
        assert_eq!(
            d.table(TableRef::ByName("T_ONE")).unwrap().parts[0].end_marker,
            EndMarker::LfOnly
        );

        // rows: a value of 0x410a bytes starts with `0a 41`; its second byte is a capital letter but the
        // line is no item of a known kind
        let mut rows = vec![0x0a, 0x41];
        rows.extend(std::iter::repeat_n(b'z', 0x410a));
        rows.extend_from_slice(&[0xff, 0xff, b'\n']);
        let p = single_column_file(code, &rows);
        let mut d = open(&p, Options::default()).unwrap();
        let (b, w) = read(&mut d, "T_ONE", &ReadOptions::default());
        assert_eq!(row_count(&b), 1, "{code}");
        assert!(w.is_empty(), "{code}: {w:?}");
        assert!(d.warnings().is_empty(), "{code}: {:?}", d.warnings());
        assert_ne!(
            d.table(TableRef::ByName("T_ONE")).unwrap().parts[0].end_marker,
            EndMarker::LfOnly
        );
    }
}
