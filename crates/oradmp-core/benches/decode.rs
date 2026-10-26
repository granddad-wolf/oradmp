//! Task T-15: micro-benchmarks of the value decoders and of the row loop on committed fixtures
//! (architecture 4.4). The field bytes are taken once, outside the timing, with a `RowSink` from the
//! fixtures; nothing in this file reads `ORADMP_WORK`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use std::hint::black_box;
use std::path::{Path, PathBuf};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use oradmp_core::charset::Charset;
use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::exp::rows::RowSink;
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::values::datetime::{decode_date, decode_timestamp};
use oradmp_core::values::number::OraNumber;

fn fixture(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../corpus/fixtures")
        .join(rel)
}

/// Collects the non-NULL field bytes of the wanted file columns (1-based).
struct Collect {
    want: Vec<u32>,
    fields: Vec<Vec<u8>>,
}

impl RowSink for Collect {
    fn row_start(&mut self, _row: u64) {}
    fn null(&mut self, _column: u32, _offset: u64) -> oradmp_core::Result<()> {
        Ok(())
    }
    fn value(&mut self, column: u32, _offset: u64, bytes: &[u8]) -> oradmp_core::Result<()> {
        if self.want.contains(&column) {
            self.fields.push(bytes.to_vec());
        }
        Ok(())
    }
    fn chain_piece(
        &mut self,
        _column: u32,
        _offset: u64,
        _piece: &[u8],
        _first: bool,
        _last: bool,
    ) -> oradmp_core::Result<()> {
        Ok(())
    }
    fn row_end(&mut self) -> oradmp_core::Result<()> {
        Ok(())
    }
}

fn fields(rel: &str, table: &str, want: &[u32]) -> Vec<Vec<u8>> {
    let mut d = open(fixture(rel), Options::default()).unwrap();
    let mut rows = d.rows(TableRef::ByName(table)).unwrap();
    let mut sink = Collect {
        want: want.to_vec(),
        fields: Vec::new(),
    };
    while rows.next_row_with(&mut sink).unwrap().is_some() {}
    sink.fields
}

fn throughput(f: &[Vec<u8>]) -> Throughput {
    Throughput::ElementsAndBytes {
        elements: f.len() as u64,
        bytes: f.iter().map(|x| x.len() as u64).sum(),
    }
}

fn number(c: &mut Criterion) {
    // T_NUM: ID (file column 1) and V (2): every NUMBER field
    let f = fields("18c/t_num_exp_18c.dmp", "T_NUM", &[1, 2]);
    let mut g = c.benchmark_group("number");
    g.throughput(throughput(&f));
    g.bench_function("OraNumber::decode", |b| {
        b.iter(|| {
            for x in &f {
                black_box(OraNumber::decode(black_box(x)).ok());
            }
        })
    });
    g.finish();
}

fn datetime(c: &mut Criterion) {
    let dates = fields("18c/t_date_exp_18c.dmp", "T_DATE", &[2]);
    let ts = fields("18c/t_ts_exp_18c.dmp", "T_TS", &[2, 3, 4]);
    let mut g = c.benchmark_group("datetime");
    g.throughput(throughput(&dates));
    g.bench_function("decode_date (T_DATE.D)", |b| {
        b.iter(|| {
            for x in &dates {
                black_box(decode_date(black_box(x)).ok());
            }
        })
    });
    g.throughput(throughput(&ts));
    g.bench_function("decode_timestamp (T_TS.TS0/TS6/TS9)", |b| {
        b.iter(|| {
            for x in &ts {
                black_box(decode_timestamp(black_box(x)).ok());
            }
        })
    });
    g.finish();
}

fn text_cp1251(c: &mut Criterion) {
    // R_STAT: TXT (3), CODE (4), NOTE (7), CL8MSWIN1251
    let f = fields(
        "11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp",
        "R_STAT",
        &[3, 4, 7],
    );
    let mut g = c.benchmark_group("text_cp1251");
    g.throughput(throughput(&f));
    g.bench_function("Charset::decode (Win1251)", |b| {
        b.iter(|| {
            for x in &f {
                black_box(Charset::Win1251.decode(black_box(x)).ok());
            }
        })
    });
    g.finish();
}

fn read_table(c: &mut Criterion) {
    let cases = [
        (
            "R_STAT",
            "11g-cp1251/ru_stat_exp_cp1251_est_11g.dmp",
            "R_STAT",
        ),
        ("T_NUM", "18c/t_num_exp_18c.dmp", "T_NUM"),
        ("T_TEXT", "18c/t_text_exp_18c.dmp", "T_TEXT"),
    ];
    let mut g = c.benchmark_group("read_table");
    for (label, rel, table) in cases {
        let mut d = open(fixture(rel), Options::default()).unwrap();
        let info = d.table(TableRef::ByName(table)).unwrap().clone();
        let bytes: u64 = info
            .parts
            .iter()
            .map(|p| p.rows_end_offset - p.rows_offset)
            .sum();
        eprintln!("{label}: {} rows, {bytes} row bytes", info.num_rows);
        g.throughput(Throughput::ElementsAndBytes {
            elements: info.num_rows,
            bytes,
        });
        // T_NUM row 19 holds a value that does not fit the plain NUMBER default (E-NUMBER-OVERFLOW): NULL it
        let opts = ReadOptions {
            on_value_error: OnValueError::Null,
            ..ReadOptions::default()
        };
        g.bench_function(label, |b| {
            b.iter(|| {
                let mut r = d.read_table(TableRef::ByName(table), &opts).unwrap();
                let mut n = 0usize;
                while let Some(batch) = r.next_batch().unwrap() {
                    n += batch.num_rows();
                }
                black_box(n)
            })
        });
    }
    g.finish();
}

criterion_group!(benches, number, datetime, text_cp1251, read_table);
criterion_main!(benches);
