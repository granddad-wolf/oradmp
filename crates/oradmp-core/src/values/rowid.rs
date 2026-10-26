//! ROWID (69) and UROWID (208): rule [Y11], architecture 5.7, ADR-20.
//!
//! The text is that of `ROWIDTOCHAR`: base 64 with the alphabet `A-Z a-z 0-9 + /`, most significant first.

use crate::error::{Error, ErrorCode, Result};

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// The 5-byte prefix of a ROWID field in every confirmed value [Y11] (meaning unknown).
pub const ROWID_PREFIX: [u8; 5] = [0x01, 0x00, 0x00, 0x00, 0x17];

fn bad(message: &'static str) -> Error {
    Error::new(ErrorCode::BadRowid, message)
}

fn in_alphabet(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'+' || c == b'/'
}

/// A decoded ROWID.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rowid {
    /// The 18-character text.
    pub text: String,
    /// False if the 5-byte prefix is not `01 00 00 00 17`: the caller raises `W-UNVERIFIED-TYPE` [Y11].
    pub prefix_known: bool,
}

/// Decodes a ROWID field: 23 bytes, a 5-byte prefix and the 18-character text, every text byte in the
/// alphabet [Y11]. Another length or a byte outside the alphabet is `E-BAD-ROWID`.
pub fn decode_rowid(b: &[u8]) -> Result<Rowid> {
    if b.len() != 23 {
        return Err(bad("ROWID length is not 23"));
    }
    let (prefix, text) = b.split_at(5);
    if !text.iter().all(|&c| in_alphabet(c)) {
        return Err(bad("ROWID text byte outside the base-64 alphabet"));
    }
    Ok(Rowid {
        text: text.iter().map(|&c| char::from(c)).collect(),
        prefix_known: prefix == ROWID_PREFIX,
    })
}

/// Appends `n` base-64 characters of `value`, most significant first.
fn push_b64(out: &mut String, value: u64, n: u32) {
    for i in (0..n).rev() {
        let shift = i.saturating_mul(6);
        let idx = usize::try_from((value >> shift) & 63).unwrap_or(0);
        out.push(char::from(ALPHABET.get(idx).copied().unwrap_or(b'A')));
    }
}

/// Decodes a UROWID field (the bytes of `DUMP`). First byte `01` and 13 bytes: physical, object u32 BE, file
/// u16 BE, block u32 BE, row u16 BE, written with 6, 3, 6, 3 characters. First byte `02`: logical, the text is
/// `*` + base 64 of the bytes after the first, 3 bytes to 4 characters; the tail must be a non-empty multiple
/// of 3 bytes (no padding rule is known [Y11]). Anything else is `E-BAD-ROWID`.
pub fn decode_urowid(b: &[u8]) -> Result<String> {
    match b.split_first() {
        Some((&0x01, rest)) if b.len() == 13 => {
            let (obj, rest) = rest.split_at(4);
            let (file, rest) = rest.split_at(2);
            let (block, row) = rest.split_at(4);
            let be = |s: &[u8]| s.iter().fold(0u64, |a, &x| (a << 8) | u64::from(x));
            let mut t = String::with_capacity(18);
            push_b64(&mut t, be(obj), 6);
            push_b64(&mut t, be(file), 3);
            push_b64(&mut t, be(block), 6);
            push_b64(&mut t, be(row), 3);
            Ok(t)
        }
        Some((&0x02, tail)) if !tail.is_empty() && tail.len().is_multiple_of(3) => {
            let (groups, _) = tail.as_chunks::<3>();
            let mut t = String::with_capacity(groups.len().saturating_mul(4).saturating_add(1));
            t.push('*');
            for g in groups {
                let v = g.iter().fold(0u64, |a, &x| (a << 8) | u64::from(x));
                push_b64(&mut t, v, 4);
            }
            Ok(t)
        }
        _ => Err(bad("UROWID form outside the confirmed ones")),
    }
}
