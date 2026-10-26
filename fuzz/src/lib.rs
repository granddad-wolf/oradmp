//! Checks shared by the three fuzz targets (T-14, ADR-61 item 3, ADR-62 item 5).
//!
//! A failed check is a `panic!`, which libFuzzer reports as a crash: a finding. The messages name only
//! structure (code, offset, length), never bytes of the input.

use oradmp_core::{Error, ErrorCode, Warning, WarningCode};

/// Checks one error the core returned for an input of `input_len` bytes.
pub fn check_error(e: &Error, input_len: u64) {
    let text = e.to_string();
    assert!(
        text.starts_with("[E-"),
        "error text does not start with [E- (code {:?})",
        e.code
    );
    assert!(
        ErrorCode::ALL.contains(&e.code),
        "error code {:?} is not in ErrorCode::ALL",
        e.code
    );
    assert!(
        e.code != ErrorCode::Internal,
        "the core constructed E-INTERNAL (code {:?})",
        e.code
    );
    if let Some(o) = e.offset {
        assert!(
            o <= input_len,
            "error offset {o} is beyond the input length {input_len} (code {:?})",
            e.code
        );
    }
    assert!(
        !text.contains('\u{0}'),
        "error text holds U+0000 (code {:?})",
        e.code
    );
}

/// Checks one warning (its `Display`, code and offset).
pub fn check_warning(w: &Warning, input_len: u64) {
    let text = w.to_string();
    assert!(
        text.starts_with("[W-"),
        "warning text does not start with [W- (code {:?})",
        w.code
    );
    assert!(
        WarningCode::ALL.contains(&w.code),
        "warning code {:?} is not in WarningCode::ALL",
        w.code
    );
    if let Some(o) = w.offset {
        assert!(
            o <= input_len,
            "warning offset {o} is beyond the input length {input_len} (code {:?})",
            w.code
        );
    }
    assert!(
        !text.contains('\u{0}'),
        "warning text holds U+0000 (code {:?})",
        w.code
    );
}

/// `Ok` passes, an `Err` is checked; returns the value if there is one.
pub fn checked<T>(r: Result<T, Error>, input_len: u64) -> Option<T> {
    match r {
        Ok(v) => Some(v),
        Err(e) => {
            check_error(&e, input_len);
            None
        }
    }
}
