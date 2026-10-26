//! Task T-7: owner mode (`RUSERS`, sections behind the `METRICSU` / `METRICEU` pairs, trailer groups).
//!
//! Expected values come from the truth files of `corpus/fixtures/{18c,11g}`: `truth_own.txt` (O_PARENT, O_CHILD,
//! O_EMPTY), `truth_ddl.txt` (T_DDL) and the row counts of the logs / `truth_stc.txt` (`TAB|T_STCn|50`).
//! Rule T8: a table section is the same bytes in owner mode and in table mode.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use arrow_array::RecordBatch;
use common::*;
use oradmp_core::dump::{open, Options};
use oradmp_core::reader::ReadOptions;

const VERSIONS: [&str; 2] = ["18c", "11g"];

fn read_t(rel: &str, table: &str) -> Vec<RecordBatch> {
    let (b, w) = read_default(rel, table);
    assert!(w.is_empty(), "{rel} {table}: {w:?}");
    b
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

fn truth_rows(folder: &str, table: &str) -> Vec<Vec<String>> {
    truth(folder, "truth_own.txt")
        .into_iter()
        .filter(|l| l[0] == table)
        .map(|l| l[1..].to_vec())
        .collect()
}

/// O_PARENT, O_CHILD, O_EMPTY of `truth_own.txt` in every `own` and `own_tables` file, conventional and
/// direct, both versions; the batches of owner mode and table mode are equal [T8].
#[test]
fn own_tables_equal_truth_and_owner_equals_table_mode() {
    for v in VERSIONS {
        for fam in ["own_exp", "own_exp_direct", "own_exp_w1252"] {
            let own = format!("{v}/{fam}_{v}.dmp");
            for t in ["O_PARENT", "O_CHILD"] {
                let b = read_t(&own, t);
                assert_eq!(rows(&b), truth_rows(v, t), "{own} {t}");
            }
            let e = read_t(&own, "O_EMPTY");
            assert_eq!(row_count(&e), 0, "{own}");
            assert_eq!(truth(v, "truth_own.txt").last().unwrap()[1], "count=0");
        }
        for (own, tbl) in [
            ("own_exp", "own_tables_exp"),
            ("own_exp_direct", "own_tables_exp_direct"),
        ] {
            for t in ["O_PARENT", "O_CHILD", "O_EMPTY"] {
                let a = read_t(&format!("{v}/{own}_{v}.dmp"), t);
                let b = read_t(&format!("{v}/{tbl}_{v}.dmp"), t);
                assert_eq!(rows(&b), rows(&a), "{v} {tbl} {t}");
                data_equal(&a, &b);
                if t != "O_EMPTY" {
                    assert_eq!(rows(&b), truth_rows(v, t));
                }
            }
        }
    }
}

/// The three tables of `own` are all found, in a file made for a user (not a table list).
#[test]
fn own_catalog_lists_the_three_tables() {
    for v in VERSIONS {
        let mut d = open_fixture(&format!("{v}/own_exp_{v}.dmp"));
        d.scan().unwrap();
        let mut names: Vec<_> = d
            .catalog()
            .unwrap()
            .tables
            .iter()
            .map(|t| t.name.clone())
            .collect();
        names.sort();
        assert_eq!(names, ["O_CHILD", "O_EMPTY", "O_PARENT"], "{v}");
    }
}

/// `ddl`: T_DDL = `truth_ddl.txt` (3 rows), T_PARENT 2 rows; the same in the direct file.
#[test]
fn ddl_family_reads_through_the_same_code() {
    for v in VERSIONS {
        let expect: Vec<Vec<String>> = truth(v, "truth_ddl.txt");
        let base = read_t(&format!("{v}/ddl_exp_{v}.dmp"), "T_DDL");
        assert_eq!(rows(&base), expect, "{v}");
        let direct = read_t(&format!("{v}/ddl_exp_direct_{v}.dmp"), "T_DDL");
        assert_eq!(rows(&direct), expect, "{v} direct");
        let p = read_t(&format!("{v}/ddl_exp_{v}.dmp"), "T_PARENT");
        assert_eq!(row_count(&p), 2);
        let pd = read_t(&format!("{v}/ddl_exp_direct_{v}.dmp"), "T_PARENT");
        data_equal(&p, &pd);
    }
}

/// `stc`: each table has the row count of `truth_stc.txt` of its folder (`TAB|T_STCn|50`; T_STC4, `NULL`
/// there, from `TAB|T_STC4|50` of `truth_stc_extra.txt`), the column statistics of the `COLUMN` lines and the
/// values of the rule of `10_stc.sql` (all through `truth_checks::stc_diffs`, in the four files of each
/// version); the four files and the direct file also equal each other.
#[test]
fn stc_family_reads_through_the_same_code() {
    // T-10b: the truth comparison is a function of `common/truth_checks.rs` (also called by t10_truth.rs)
    let diffs = common::truth_checks::stc_diffs();
    assert!(diffs.is_empty(), "{diffs:?}");
    for v in VERSIONS {
        let counts = common::truth_checks::stc_counts(v);
        assert_eq!(counts.len(), 5);
        for (t, n) in &counts {
            let base = read_t(&format!("{v}/stc_exp_{v}.dmp"), t);
            assert_eq!(row_count(&base).to_string(), *n, "{v} {t}");
            for f in ["stc_exp_none", "stc_exp_us7ascii", "stc_exp_direct"] {
                let o = read_t(&format!("{v}/{f}_{v}.dmp"), t);
                assert_eq!(rows(&o), rows(&base), "{v} {f} {t}");
            }
        }
    }
}

/// `ru_exp_*` (owner mode): every table reads completely, no error and no stop (the R_CLOB values are T-6L's).
#[test]
fn ru_owner_files_read_completely() {
    let dir = fixtures().join("11g-cp1251");
    let mut n = 0;
    for e in std::fs::read_dir(&dir).unwrap() {
        let name = e.unwrap().file_name().to_string_lossy().into_owned();
        if !(name.starts_with("ru_exp_") && name.ends_with(".dmp")) {
            continue;
        }
        let mut d = open(dir.join(&name), Options::default()).unwrap();
        d.scan().unwrap();
        let tables: Vec<_> = d
            .catalog()
            .unwrap()
            .tables
            .iter()
            .map(|t| t.name.clone())
            .collect();
        assert!(tables.len() >= 2, "{name}: {tables:?}");
        for t in &tables {
            let (_b, _) = try_read(&mut d, t, &ReadOptions::default())
                .unwrap_or_else(|e| panic!("{name} {t}: {e:?}"));
        }
        n += 1;
    }
    assert_eq!(n, 6);
}
