//! Task T-6, N-13: the field bytes that the row decoder's seam (`RowSink`, `TableRows::next_row_with`) delivers
//! reproduce the per-part SHA-256 of `tests/data/value_parts.tsv` over the NUMBER fields of the binary
//! parts of the fixture files (counts derived from the TSV; the Analyst's totals 418 / 86,124 of rules v3.5 are compared with it in
//! `t1_walk.rs`) (the walker replaces T-4's test-only field reader). The expectation is the
//! Analyst's `docs/formats/tools/exp_parse.py` through `tests/data/gen_number_parts.py`; each field is hashed as
//! one length byte followed by its bytes.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::collections::HashMap;

use common::*;
use oradmp_core::dump::TableRef;
use oradmp_core::exp::rows::RowSink;
use oradmp_core::Result;
use sha2::{Digest, Sha256};

/// Hashes the fields of the NUMBER columns (by file position) and counts the rows.
struct NumberHash {
    number_columns: Vec<bool>,
    h: Sha256,
    fields: u64,
    rows: u64,
    pieces: u64,
}

impl RowSink for NumberHash {
    fn row_start(&mut self, _row: u64) {
        self.rows += 1;
    }
    fn null(&mut self, _column: u32, _offset: u64) -> Result<()> {
        Ok(())
    }
    fn value(&mut self, column: u32, _offset: u64, bytes: &[u8]) -> Result<()> {
        if self.number_columns[column as usize - 1] {
            self.h.update([bytes.len() as u8]);
            self.h.update(bytes);
            self.fields += 1;
        }
        Ok(())
    }
    fn chain_piece(&mut self, _c: u32, _o: u64, _p: &[u8], _f: bool, _l: bool) -> Result<()> {
        self.pieces += 1;
        Ok(())
    }
    fn row_end(&mut self) -> Result<()> {
        Ok(())
    }
}

#[test]
fn field_bytes_of_the_row_seam_reproduce_the_per_part_number_hashes() {
    let text =
        std::fs::read_to_string(root().join("crates/oradmp-core/tests/data/value_parts.tsv"))
            .unwrap();
    let mut expected: HashMap<(String, u64), (u64, u64, String)> = HashMap::new();
    let mut files: Vec<String> = Vec::new();
    for l in text
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with("file\t"))
    {
        let c: Vec<&str> = l.split('\t').collect();
        if !files.iter().any(|f| f == c[0]) {
            files.push(c[0].to_string());
        }
        expected.insert(
            (c[0].to_string(), c[1].parse().unwrap()),
            (
                c[3].parse().unwrap(),
                c[5].parse().unwrap(),
                c[6].to_string(),
            ),
        );
    }
    let tsv_parts = expected.len();
    let tsv_numbers: u64 = expected.values().map(|v| v.1).sum();
    assert!(tsv_parts > 0 && !files.is_empty());
    let (mut parts, mut numbers, mut chain_pieces) = (0, 0, 0);
    for f in &files {
        let mut d = open_fixture(f);
        let tables = d.tables().unwrap().to_vec();
        for t in &tables {
            let mut by_file_pos = vec![false; t.columns.len()];
            for c in &t.columns {
                by_file_pos[c.file_position as usize - 1] = c.type_code == 2;
            }
            for (pi, p) in t.parts.iter().enumerate() {
                let want = expected
                    .remove(&(f.clone(), p.rows_offset))
                    .unwrap_or_else(|| panic!("{f}@{}", p.rows_offset));
                let mut rows = d.part_rows(TableRef::ByOrdinal(t.ordinal), pi).unwrap();
                let mut sink = NumberHash {
                    number_columns: by_file_pos.clone(),
                    h: Sha256::new(),
                    fields: 0,
                    rows: 0,
                    pieces: 0,
                };
                while rows.next_row_with(&mut sink).unwrap().is_some() {}
                assert_eq!(sink.rows, want.0, "{f}@{} rows", p.rows_offset);
                assert_eq!(sink.fields, want.1, "{f}@{} NUMBER fields", p.rows_offset);
                assert_eq!(
                    hex(&sink.h.finalize()),
                    want.2,
                    "{f}@{} hash",
                    p.rows_offset
                );
                parts += 1;
                numbers += sink.fields;
                chain_pieces += sink.pieces;
            }
        }
    }
    assert!(
        expected.is_empty(),
        "parts of the TSV that the catalog does not have: {}",
        expected.len()
    );
    assert_eq!((parts, numbers), (tsv_parts, tsv_numbers));
    assert!(
        chain_pieces > 0,
        "the direct-path LONG chains reach the sink piece by piece"
    );
}
