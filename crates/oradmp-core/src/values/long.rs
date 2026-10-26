//! LONG (8) and LONG RAW (24): rules [Y9, R2], ADR-15, architecture 5.1 and 5.4.
//!
//! The value of a LONG column is one field (conventional path) or a chain of pieces (direct path); the walker
//! delivers the pieces and [`LongBuffer`] assembles them, so that a UTF-8 sequence split between two pieces is
//! decoded whole. LONG RAW is the assembled bytes. The character set of LONG is the **database** set of the
//! header, never the client set [Y9].

use crate::error::{Error, ErrorCode, Result};

/// The largest LONG and LONG RAW value confirmed on both paths and versions (R-040, R-048, R-051; ADR-99 item 6); a
/// larger value gives `W-LONG-LENGTH` once per column.
pub const CONFIRMED_MAX_LEN: u64 = 100_000;

/// True if a value of `total` bytes is above the largest confirmed length (`W-LONG-LENGTH`): 100,000 bytes, for LONG
/// and LONG RAW alike.
pub fn above_confirmed_length(total: u64) -> bool {
    total > CONFIRMED_MAX_LEN
}

/// Assembles the pieces of one value under a limit (`max_lob_bytes`, applies to LONG and LONG RAW values).
#[derive(Debug, Clone)]
pub struct LongBuffer {
    bytes: Vec<u8>,
    max: u64,
}

impl LongBuffer {
    /// An empty buffer that refuses more than `max_bytes` bytes.
    pub fn new(max_bytes: u64) -> Self {
        Self {
            bytes: Vec::new(),
            max: max_bytes,
        }
    }

    /// Appends one piece. A value that would pass the limit is `E-LOB-TOO-LARGE` before any byte is copied.
    pub fn push_piece(&mut self, piece: &[u8]) -> Result<()> {
        let new_len = u64::try_from(self.bytes.len())
            .unwrap_or(u64::MAX)
            .saturating_add(u64::try_from(piece.len()).unwrap_or(u64::MAX));
        if new_len > self.max {
            return Err(
                Error::new(ErrorCode::LobTooLarge, "LONG value above max_lob_bytes")
                    .with_length(new_len),
            );
        }
        self.bytes.extend_from_slice(piece);
        Ok(())
    }

    /// The bytes so far (a LONG RAW value when the chain is complete).
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Takes the bytes and empties the buffer.
    pub fn take(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.bytes)
    }
}

/// True if any byte is above 0x7f: LONG text of a database set other than AL32UTF8 then gets
/// `W-LONG-CHARSET` (architecture 5.1, ADR-15).
pub fn has_high_bytes(bytes: &[u8]) -> bool {
    bytes.iter().any(|&b| b > 0x7f)
}

/// Decodes assembled LONG bytes as UTF-8 (database set AL32UTF8, confirmed [Y9]); invalid UTF-8 is
/// `E-BAD-TEXT`.
pub fn decode_utf8(bytes: &[u8]) -> Result<&str> {
    std::str::from_utf8(bytes)
        .map_err(|_| Error::new(ErrorCode::BadText, "LONG text is not valid UTF-8"))
}
