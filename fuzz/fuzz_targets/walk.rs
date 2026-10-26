#![no_main]
//! Target `walk`: the structural walker over the bytes (`oradmp_core::exp::items::walk`).

use std::io::Cursor;

use libfuzzer_sys::fuzz_target;
use oradmp_core::exp::items::{walk, WalkEnd, WalkOptions};
use oradmp_fuzz::{check_error, check_warning};

fuzz_target!(|data: &[u8]| {
    let len = data.len() as u64;
    let options = WalkOptions {
        hash_fields: true,
        ..WalkOptions::default()
    };
    match walk(Cursor::new(data), &options) {
        Ok(report) => {
            for w in &report.warnings {
                check_warning(w, len);
            }
            if let WalkEnd::Stopped(e) = &report.end {
                check_error(e, len);
            }
        }
        Err(e) => check_error(&e, len),
    }
});
