//! The panic guard of the binding (ADR-59, architecture 7.3): a panic in the core becomes `E-INTERNAL`.
//!
//! The lints of the workspace deny every way to panic on input, so a panic here is a bug; the binding still
//! must not let it reach the host as a `BaseException`, or abort across the C boundary of an Arrow stream.

use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::error::{Error, ErrorCode, Result};

/// The fixed message of the error (no panic payload, no location, no value).
pub const INTERNAL_MESSAGE: &str =
    "internal error: a panic in oradmp was caught; please report it with the oradmp version";

/// The message of every later call on an object that caught a panic.
pub const INTERNAL_LATER_MESSAGE: &str = "an earlier call failed with an internal error";

/// The error of a caught panic: `E-INTERNAL`, the fixed message, no context.
pub fn internal_error() -> Error {
    Error::new(ErrorCode::Internal, INTERNAL_MESSAGE)
}

/// The error of a call on an object that is unusable after an earlier caught panic.
pub fn internal_error_later() -> Error {
    Error::new(ErrorCode::Internal, INTERNAL_LATER_MESSAGE)
}

/// Runs `f`; `Ok` and `Err` pass through unchanged, a panic gives [`internal_error`]. The payload is dropped
/// unread. The default panic hook still prints to stderr (the library does not replace the process's hook).
pub fn catch_panic<R>(f: impl FnOnce() -> Result<R>) -> Result<R> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(r) => r,
        Err(_payload) => Err(internal_error()),
    }
}

#[cfg(test)]
#[allow(clippy::panic, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn a_panic_becomes_the_fixed_internal_error() {
        let e = catch_panic::<u8>(|| panic!("secret value 4e4e4e4e from the file")).unwrap_err();
        assert_eq!(e.code, ErrorCode::Internal);
        assert_eq!(e.code.as_str(), "E-INTERNAL");
        assert_eq!(e.message, INTERNAL_MESSAGE);
        assert!(!e.to_string().contains("secret"), "{e}");
        assert!(!e.to_string().contains("4e4e"), "{e}");
        assert!(!e.to_string().contains("guard.rs"), "{e}");
        assert_eq!(e.offset, None);
        assert_eq!(e.table, None);
        assert_eq!(e.row, None);
    }

    #[test]
    fn a_panic_with_a_formatted_payload_is_dropped_too() {
        let n = 7;
        let e = catch_panic::<u8>(|| panic!("value {n}")).unwrap_err();
        assert_eq!(e.message, INTERNAL_MESSAGE);
    }

    #[test]
    fn ok_and_err_pass_through_unchanged() {
        assert_eq!(catch_panic(|| Ok(5_u8)).unwrap(), 5);
        let e = catch_panic::<u8>(|| Err(Error::new(ErrorCode::Framing, "x").at(9))).unwrap_err();
        assert_eq!(e.code, ErrorCode::Framing);
        assert_eq!(e.offset, Some(9));
        assert_eq!(e.message, "x");
    }

    #[test]
    fn the_later_error_has_its_own_literal() {
        let e = internal_error_later();
        assert_eq!(e.code, ErrorCode::Internal);
        assert_eq!(e.message, INTERNAL_LATER_MESSAGE);
    }
}
