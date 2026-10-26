#![no_main]
//! Target `read`: the input is written to one file in a per-process temporary folder, then `open`, `scan` and
//! every table read to its end (ADR-62 items 5 and 6).

use std::path::PathBuf;
use std::sync::OnceLock;

use libfuzzer_sys::fuzz_target;
use oradmp_core::dump::{open, CatalogEnd, Options, TableRef};
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_fuzz::{check_error, check_warning};

/// `<temp dir>/oradmp-fuzz-<pid>/in.dmp`, the folder created once per process.
fn input_path() -> &'static PathBuf {
    static PATH: OnceLock<PathBuf> = OnceLock::new();
    PATH.get_or_init(|| {
        let dir = std::env::temp_dir().join(format!("oradmp-fuzz-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("create the temporary folder");
        dir.join("in.dmp")
    })
}

fuzz_target!(|data: &[u8]| {
    let len = data.len() as u64;
    let path = input_path();
    std::fs::write(path, data).expect("write the input file");

    let mut dump = match open(path, Options::default()) {
        Ok(d) => d,
        Err(e) => {
            check_error(&e, len);
            return;
        }
    };
    for w in dump.warnings() {
        check_warning(w, len);
    }
    let ordinals: Vec<u32> = match dump.scan() {
        Ok(catalog) => {
            if let CatalogEnd::Stopped(e) = &catalog.end {
                check_error(e, len);
            }
            catalog.tables.iter().map(|t| t.ordinal).collect()
        }
        Err(e) => {
            check_error(&e, len);
            return;
        }
    };
    for w in dump.warnings() {
        check_warning(w, len);
    }
    let options = ReadOptions {
        batch_rows: 1024,
        batch_bytes: 4 << 20,
        max_lob_bytes: 1 << 20,
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    for ordinal in ordinals {
        let mut batches = match dump.read_table(TableRef::ByOrdinal(ordinal), &options) {
            Ok(b) => b,
            Err(e) => {
                check_error(&e, len);
                continue;
            }
        };
        loop {
            match batches.next_batch() {
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(e) => {
                    check_error(&e, len);
                    break;
                }
            }
        }
        for w in batches.warnings() {
            check_warning(w, len);
        }
    }
});
