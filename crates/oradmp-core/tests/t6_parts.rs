//! Task T-6, owner requirement: a partitioned table is read through all its parts.
//!
//! * T_PART (`part_exp_18c.dmp`, `part_exp_direct_18c.dmp`): 3 range partitions, 3,000 rows in total. Numbers:
//!   `truth_part.txt` (`P|P2019|1095`, `P|P2020|1098`, `P|PMAX|807`, `TOTAL|3000|1395465|01-JAN-19|26-SEP-21`)
//!   and the Analyst's `docs/formats/tools/part_check.py` run read-only on both files ("ids are exactly 1..3000:
//!   OK", every row equals the SQL of `corpus/sql/01b_optional_tables.sql`: id = lvl, sale_date = DATE
//!   '2019-01-01' + MOD(lvl * 13, 1000), amount = MOD(lvl * 31, 100000) / 100).
//! * T_PART2 (`part2_exp_18c.dmp`, `part2_exp_direct_18c.dmp`, the "part2" fixtures): 100 rows in 6 parts of
//!   21, 29, 0, 0, 21, 29 rows (`truth_part2.txt` `SP` lines, `part2_check.py`: "all 100 ROW lines found"),
//!   rows from `corpus/sql/14_part2.sql` (ids 1..50 and 300..349, K = MOD(LEVEL, 7), V = 'v'|'w' || LEVEL).
//!   There is no partition named `part2`: the name belongs to the fixture family and its table T_PART2.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::collections::BTreeMap;

use common::*;
use oradmp_core::dump::TableRef;
use oradmp_core::reader::ReadOptions;

const PART_FILES: [&str; 2] = ["18c/part_exp_18c.dmp", "18c/part_exp_direct_18c.dmp"];
const PART2_FILES: [&str; 2] = ["18c/part2_exp_18c.dmp", "18c/part2_exp_direct_18c.dmp"];

/// T_PART gives 3,000 rows in total through the Arrow path, in batches that follow the three parts (1095,
/// 1098, 807 rows of `truth_part.txt`), on both paths. Reading only the first part would give 1,095.
#[test]
fn t_part_gives_3000_rows_through_all_three_parts() {
    let truth_parts: Vec<(String, usize)> = truth("18c", "truth_part.txt")
        .into_iter()
        .filter(|l| l[0] == "P")
        .map(|l| (l[1].clone(), l[2].parse().unwrap()))
        .collect();
    assert_eq!(
        truth_parts,
        [
            ("P2019".to_string(), 1095),
            ("P2020".to_string(), 1098),
            ("PMAX".to_string(), 807)
        ]
    );
    for file in PART_FILES {
        let mut d = open_fixture(file);
        let info = d.table(TableRef::ByName("T_PART")).unwrap().clone();
        assert!(info.partitioned);
        assert_eq!(info.parts.len(), 3);
        let (b, w) = read(&mut d, "T_PART", &ReadOptions::default());
        assert!(w.is_empty(), "{file}: {w:?}");
        assert_eq!(row_count(&b), 3000, "{file}");
        // one batch per part, in file order, named as the truth
        let sizes: Vec<usize> = b.iter().map(|x| x.num_rows()).collect();
        assert_eq!(sizes, [1095, 1098, 807], "{file}");
        let names: Vec<_> = info
            .parts
            .iter()
            .map(|p| p.partition.clone().unwrap())
            .collect();
        assert_eq!(names, ["P2019", "P2020", "PMAX"]);
        // ids are exactly 1..3000; every row equals the SQL
        let r = rows(&b);
        let mut ids: Vec<i64> = r.iter().map(|x| x[0].parse().unwrap()).collect();
        ids.sort_unstable();
        assert_eq!(ids, (1..=3000).collect::<Vec<i64>>(), "{file}");
        let base = days_from_civil(2019, 1, 1);
        for row in &r {
            let lvl: i64 = row[0].parse().unwrap();
            let (y, m, dd) = civil_from_days(base + (lvl * 13) % 1000);
            assert_eq!(
                row[1],
                format!("{y:04}-{m:02}-{dd:02} 00:00:00"),
                "{file} id {lvl}"
            );
            assert_eq!(
                row[2],
                decimal_text(&((lvl * 31) % 100_000).to_string(), 2),
                "{file} id {lvl}"
            );
        }
        // TOTAL|3000|1395465|01-JAN-19|26-SEP-21
        let total = truth("18c", "truth_part.txt")
            .into_iter()
            .find(|l| l[0] == "TOTAL")
            .unwrap();
        assert_eq!(total[1], "3000");
        let sum_cents: i128 = b
            .iter()
            .map(|x| {
                let a = x
                    .column(2)
                    .as_any()
                    .downcast_ref::<arrow_array::Decimal128Array>()
                    .unwrap();
                a.iter().map(|v| v.unwrap()).sum::<i128>()
            })
            .sum();
        assert_eq!(sum_cents, 139_546_500, "sum(amount) = {} * 100", total[2]);
        assert_eq!(total[2], "1395465");
        let dates: Vec<&str> = r.iter().map(|x| &x[1][..10]).collect();
        assert_eq!(*dates.iter().min().unwrap(), "2019-01-01"); // 01-JAN-19
        assert_eq!(*dates.iter().max().unwrap(), "2021-09-26"); // 26-SEP-21
                                                                // every date lies in the bounds of its partition: P2019 < 2020-01-01 <= P2020 < 2021-01-01 <= PMAX
        let bounds = [
            ("0000-01-01", "2020-01-01"),
            ("2020-01-01", "2021-01-01"),
            ("2021-01-01", "~"),
        ];
        for (batch, (lo, hi)) in b.iter().zip(bounds) {
            for row in rows(std::slice::from_ref(batch)) {
                let date = &row[1][..10];
                assert!(date >= lo && date < hi, "{row:?}");
            }
        }
    }
}

/// A small `batch_rows` never makes a batch that spans two parts (ADR-21): with 500 rows per batch the parts
/// give 3 + 3 + 2 batches, 3,000 rows in all.
#[test]
fn batches_never_mix_parts() {
    for file in PART_FILES {
        let (b, _) = read(
            &mut open_fixture(file),
            "T_PART",
            &ReadOptions {
                batch_rows: 500,
                ..ReadOptions::default()
            },
        );
        let sizes: Vec<usize> = b.iter().map(|x| x.num_rows()).collect();
        assert_eq!(sizes, [500, 500, 95, 500, 500, 98, 500, 307], "{file}");
        assert_eq!(row_count(&b), 3000);
    }
}

/// T_PART2 ("part2") gives 100 rows in total through the Arrow path, on both paths: 4 batches of 21, 29, 21,
/// 29 rows (the two empty parts of P_EMPTY give none), each batch holding exactly the rows that
/// `truth_part2.txt` puts into the subpartition of that part, and every row equals `14_part2.sql`.
#[test]
fn t_part2_gives_100_rows_and_each_part_holds_its_own_rows() {
    // T-10b: the truth comparison is a function of `common/truth_checks.rs` (also called by t10_truth.rs)
    let diffs = common::truth_checks::part2_diffs();
    assert!(diffs.is_empty(), "{diffs:?}");
    let t = truth("18c", "truth_part2.txt");
    let sp: Vec<(String, String)> = t
        .iter()
        .filter(|l| l[0] == "SP")
        .map(|l| (l[1].clone(), l[2].clone()))
        .collect();
    assert_eq!(sp.len(), 6);
    let sub_of: BTreeMap<i64, String> = t
        .iter()
        .filter(|l| l[0] == "ROW")
        .map(|l| (l[1].parse().unwrap(), l[2].clone()))
        .collect();
    assert_eq!(sub_of.len(), 100);
    for file in PART2_FILES {
        let mut d = open_fixture(file);
        let info = d.table(TableRef::ByName("T_PART2")).unwrap().clone();
        assert_eq!(info.num_rows, 100);
        let parts: Vec<_> = info
            .parts
            .iter()
            .map(|p| {
                (
                    p.partition.clone().unwrap(),
                    p.subpartition.clone().unwrap(),
                    p.num_rows,
                )
            })
            .collect();
        assert_eq!(
            parts.iter().map(|p| p.2).collect::<Vec<_>>(),
            [21, 29, 0, 0, 21, 29]
        );
        let (b, w) = read(&mut d, "T_PART2", &ReadOptions::default());
        assert!(w.is_empty(), "{file}: {w:?}");
        assert_eq!(row_count(&b), 100, "{file}");
        assert_eq!(
            b.iter().map(|x| x.num_rows()).collect::<Vec<_>>(),
            [21, 29, 21, 29],
            "{file}"
        );
        let non_empty: Vec<_> = parts.iter().filter(|p| p.2 > 0).collect();
        for (batch, part) in b.iter().zip(non_empty) {
            for row in rows(std::slice::from_ref(batch)) {
                let id: i64 = row[0].parse().unwrap();
                assert_eq!(
                    sub_of[&id], part.1,
                    "{file}: id {id} is in the subpartition of another part"
                );
                let level = if id <= 50 { id } else { id - 299 };
                assert_eq!(row[1], (level % 7).to_string(), "{file} id {id}");
                assert_eq!(
                    row[2],
                    format!("{}{level}", if id <= 50 { 'v' } else { 'w' }),
                    "{file} id {id}"
                );
            }
        }
        let mut ids: Vec<i64> = rows(&b).iter().map(|x| x[0].parse().unwrap()).collect();
        ids.sort_unstable();
        let want: Vec<i64> = (1..=50).chain(300..=349).collect();
        assert_eq!(ids, want, "{file}");
    }
}

/// The partitioned tables are listed once and `num_rows` is the sum of the parts: the numbers the readers
/// above rely on (T_PART 3,000; T_PART2 100, two parts of 0 rows).
#[test]
fn catalog_row_counts_are_the_sums_of_the_parts() {
    let mut d = open_fixture(PART_FILES[0]);
    assert_eq!(d.table(TableRef::ByName("T_PART")).unwrap().num_rows, 3000);
    let mut d = open_fixture(PART2_FILES[0]);
    let t = d.table(TableRef::ByName("T_PART2")).unwrap();
    assert_eq!(t.num_rows, 100);
    assert_eq!(t.parts.iter().filter(|p| p.num_rows == 0).count(), 2);
}
