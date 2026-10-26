//! Task T-15 (d): the memory test of architecture 4.3 on the owner's `T_ROWS` export. `#[ignore]`: run alone,
//! in a release build, with `ORADMP_WORK` set:
//! `cargo nextest run --release --run-ignored only -E 'binary(t15_memory)' -j 1`.
//!
//! The test re-runs its own binary (`std::env::current_exe`) as a child for the helper test
//! `t15_child_read`; the child reads the table with `batch_rows = 4096`, once stopping after 10,000 rows (at
//! the end of the batch that reaches them: exactly 12,288) and once to the end (100,000 rows: a little more than
//! eight times the data), and prints its `VmHWM` (`/proc/self/status`). The parent asserts `peak(100,000) <= 1.10
//! x peak(12,288)`. With `T_BIG` (5,000,000 rows, `18c/t_big_exp_18c.dmp`) in `ORADMP_WORK` the same test runs on
//! it: stop after 100,000 rows (the batch that reaches them: exactly 102,400), then to the end, with the same
//! limit of 1.10 (ADR-65 item 6: 49 times the data resolves about one byte per row).
//! T-15b measured that resolution: one byte retained per row raised the `T_BIG` peak ratio to 1.78 to 1.96 (a
//! `Vec` doubles its capacity, so the rise is larger than the 4.9 MB of arithmetic); the correct reader gives
//! 1.00 to 1.05. Every file present is measured before the test asserts, and a failure lists each file with
//! both peaks and its ratio. The files are read only.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use std::path::PathBuf;
use std::process::Command;

use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::reader::ReadOptions;

const MODE: &str = "ORADMP_T15_MODE";
const FILE: &str = "ORADMP_T15_FILE";
const TABLE: &str = "ORADMP_T15_TABLE";

fn vm_hwm_kb() -> u64 {
    let s = std::fs::read_to_string("/proc/self/status").unwrap();
    let l = s.lines().find(|l| l.starts_with("VmHWM:")).unwrap();
    l.split_whitespace().nth(1).unwrap().parse().unwrap()
}

/// The child. Without the environment of the parent it does nothing.
#[test]
#[ignore = "helper of t15_memory; run by the parent test"]
fn t15_child_read() {
    let (Some(mode), Some(file), Some(table)) = (
        std::env::var_os(MODE),
        std::env::var_os(FILE),
        std::env::var(TABLE).ok(),
    ) else {
        return;
    };
    // the mode is "all" or "stop=<rows>"
    let stop_after = mode
        .to_str()
        .and_then(|m| m.strip_prefix("stop="))
        .map_or(u64::MAX, |n| n.parse().unwrap());
    let mut d = open(PathBuf::from(file), Options::default()).unwrap();
    let opts = ReadOptions {
        batch_rows: 4096,
        ..ReadOptions::default()
    };
    let mut r = d.read_table(TableRef::ByName(&table), &opts).unwrap();
    let mut rows = 0u64;
    while let Some(b) = r.next_batch().unwrap() {
        rows += b.num_rows() as u64;
        if rows >= stop_after {
            break;
        }
    }
    println!("T15_CHILD rows={rows} vmhwm_kb={}", vm_hwm_kb());
}

fn child(file: &PathBuf, table: &str, mode: &str) -> (u64, u64) {
    let out = Command::new(std::env::current_exe().unwrap())
        .args([
            "t15_child_read",
            "--exact",
            "--ignored",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(MODE, mode)
        .env(FILE, file)
        .env(TABLE, table)
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    assert!(out.status.success(), "child failed: {text}");
    let line = text.lines().find(|l| l.contains("T15_CHILD")).unwrap();
    let field = |k: &str| -> u64 {
        line.split_whitespace()
            .find_map(|w| w.strip_prefix(k))
            .unwrap()
            .parse()
            .unwrap()
    };
    (field("rows="), field("vmhwm_kb="))
}

fn compare(file: &PathBuf, table: &str, label: &str, stop: u64) -> (u64, u64, u64, u64) {
    let (r10, p10) = child(file, table, &format!("stop={stop}"));
    let (rall, pall) = child(file, table, "all");
    println!(
        "T-15 memory {label}: stop after {r10} rows peak {p10} kB; to the end {rall} rows peak {pall} kB; ratio {:.4}",
        pall as f64 / p10 as f64
    );
    (r10, p10, rall, pall)
}

/// One measured file: label, rows at the stop, rows to the end, both peaks (kB), the expected rows.
struct Measured {
    label: String,
    stopped_at: (u64, u64),
    total: u64,
    r_stop: u64,
    p_stop: u64,
    r_all: u64,
    p_all: u64,
}

impl Measured {
    fn ratio(&self) -> f64 {
        self.p_all as f64 / self.p_stop as f64
    }
    fn ok(&self) -> bool {
        self.r_stop == self.stopped_at.1
            && self.r_all == self.total
            && self.p_all * 100 <= self.p_stop * 110
    }
}

/// Runs the pair of reads on one file (no assertion: the test asserts after every file is measured).
fn measure(
    file: &PathBuf,
    table: &str,
    label: &str,
    (stop, stopped_at): (u64, u64),
    total: u64,
) -> Measured {
    let (r_stop, p_stop, r_all, p_all) = compare(file, table, label, stop);
    Measured {
        label: label.to_string(),
        stopped_at: (stop, stopped_at),
        total,
        r_stop,
        p_stop,
        r_all,
        p_all,
    }
}

#[test]
#[ignore = "needs ORADMP_WORK; run alone in a release build"]
fn peak_rss_of_eight_times_the_data_stays_within_ten_percent() {
    let Some(work) = std::env::var_os("ORADMP_WORK") else {
        eprintln!("T-15 memory: ORADMP_WORK is not set, skipped");
        return;
    };
    let work = PathBuf::from(work);
    let mut all: Vec<Measured> = Vec::new();
    for rel in ["18c/t_rows_exp_18c.dmp", "18c/t_rows_exp_direct_18c.dmp"] {
        let file = work.join(rel);
        if !file.exists() {
            eprintln!("T-15 memory: {rel} is absent, skipped");
            continue;
        }
        // three batches of 4096 reach 10,000 rows: the stop is exactly 12,288
        all.push(measure(&file, "T_ROWS", rel, (10_000, 12_288), 100_000));
    }
    // T_BIG (5,000,000 rows), when present: the same test, stop at 100,000 rows (25 batches: 102,400), same limit
    let big = work.join("18c/t_big_exp_18c.dmp");
    if big.exists() {
        all.push(measure(
            &big,
            "T_BIG",
            "T_BIG",
            (100_000, 102_400),
            5_000_000,
        ));
    } else {
        eprintln!("T-15 memory: 18c/t_big_exp_18c.dmp is absent, T_BIG skipped");
    }
    if all.is_empty() {
        eprintln!("T-15 memory: no file, nothing measured");
    }
    let report: Vec<String> = all
        .iter()
        .map(|m| {
            format!(
                "{}: stop {} rows (expected {}) peak {} kB; end {} rows (expected {}) peak {} kB; ratio {:.4}; {}",
                m.label,
                m.r_stop,
                m.stopped_at.1,
                m.p_stop,
                m.r_all,
                m.total,
                m.p_all,
                m.ratio(),
                if m.ok() { "ok" } else { "FAILS (limit 1.10)" }
            )
        })
        .collect();
    assert!(
        all.iter().all(Measured::ok),
        "memory test failed:\n{}",
        report.join("\n")
    );
}
