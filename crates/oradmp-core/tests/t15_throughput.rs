//! Task T-15 (c): throughput of the scan and of a full read on the owner's `T_ROWS` exports (architecture 4.3).
//! `#[ignore]`: run alone, in a release build, with `ORADMP_WORK` set:
//! `cargo nextest run --release --run-ignored only -E 'binary(t15_throughput)' -j 1`.
//! It prints and asserts nothing about speed. The files are read only; nothing from them is committed. The
//! files are `ORADMP_WORK/18c/t_rows_exp_18c.dmp` and `t_rows_exp_direct_18c.dmp` (and `T_BIG`, if present).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use std::path::PathBuf;
use std::time::{Duration, Instant};

use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::reader::ReadOptions;

const RUNS: usize = 3;
const FILES: [(&str, &str); 3] = [
    ("18c/t_rows_exp_18c.dmp", "T_ROWS"),
    ("18c/t_rows_exp_direct_18c.dmp", "T_ROWS"),
    ("18c/t_big_exp_18c.dmp", "T_BIG"),
];

fn median(mut v: Vec<Duration>) -> Duration {
    v.sort();
    v[v.len() / 2]
}

#[test]
#[ignore = "needs ORADMP_WORK; run alone in a release build"]
fn throughput_of_scan_and_read() {
    let Some(work) = std::env::var_os("ORADMP_WORK") else {
        eprintln!("T-15 throughput: ORADMP_WORK is not set, skipped");
        return;
    };
    if cfg!(debug_assertions) {
        eprintln!("T-15 throughput: a debug build; the numbers mean nothing (use --release)");
    }
    for (rel, table) in FILES {
        let path = PathBuf::from(&work).join(rel);
        let Ok(meta) = std::fs::metadata(&path) else {
            eprintln!("T-15 throughput: {rel} is absent, skipped");
            continue;
        };
        let size = meta.len();
        let mb = size as f64 / 1e6;
        let mut scans = Vec::new();
        let mut reads = Vec::new();
        let (mut rows, mut row_bytes) = (0u64, 0u64);
        for _ in 0..RUNS {
            let mut d = open(&path, Options::default()).unwrap();
            let t = Instant::now();
            d.scan().unwrap();
            scans.push(t.elapsed());
            let info = d.table(TableRef::ByName(table)).unwrap().clone();
            row_bytes = info
                .parts
                .iter()
                .map(|p| p.rows_end_offset - p.rows_offset)
                .sum();
            let t = Instant::now();
            let mut r = d
                .read_table(TableRef::ByName(table), &ReadOptions::default())
                .unwrap();
            let mut n = 0u64;
            while let Some(b) = r.next_batch().unwrap() {
                n += b.num_rows() as u64;
            }
            reads.push(t.elapsed());
            rows = n;
        }
        let scan = median(scans.clone()).as_secs_f64();
        let read = median(reads.clone()).as_secs_f64();
        println!("T-15 {rel}: file {size} bytes ({mb:.1} MB), table {table}, {rows} rows, row area {row_bytes} bytes");
        println!(
            "T-15 {rel}: scan runs {:?} median {:.3} s = {:.1} MB/s",
            scans,
            scan,
            mb / scan
        );
        println!(
            "T-15 {rel}: read runs {:?} median {:.3} s = {:.0} rows/s, {:.1} MB/s of row area, {:.1} MB/s of file",
            reads,
            read,
            rows as f64 / read,
            row_bytes as f64 / 1e6 / read,
            mb / read
        );
    }
}
