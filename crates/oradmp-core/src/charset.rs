//! Layer 5 (architecture 2.1, section 6): character set ids to decoders. The five ids of [X3] (US7ASCII 1,
//! CL8MSWIN1251 171, WE8MSWIN1252 178, AL32UTF8 873, AL16UTF16 2000), the overrides `db_charset` /
//! `client_charset` (an Oracle name or any `encoding_rs` label, [`Charset::from_name`]) and the two ways to
//! decode: [`Charset::decode`] for row data and [`decode_text`] for names and DDL.
//!
//! One rule for row data (architecture 6.3): a byte sequence the set does not allow is `E-BAD-TEXT`; the
//! reader never delivers a U+FFFD that the file does not hold. One rule for metadata text: a byte the set does
//! not allow becomes U+FFFD and the item is flagged; the scan never stops for it. CL8MSWIN1251 and
//! WE8MSWIN1252 have no malformed byte; `encoding_rs` is used for them (the 768-line map test of
//! `tests/t9_charset.rs` compares both decoders with `truth_csmap.txt`).

use std::borrow::Cow;

use crate::error::{Error, ErrorCode, Result};

/// The database character set ids of the header whose CLOB / NCLOB storage a fixture confirms: AL32UTF8 (873)
/// and CL8MSWIN1251 (171) [L5 v3.5, request R-038, ADR-25 session 8]. The evidence for 171 is 11g only
/// (`ru_lob2_exp_*_11g`); every other database set keeps `E-LOB-UNCONFIRMED` (with `unconfirmed_lob="decode"`
/// the value is read with `W-LOB-UNTESTED`). This list is the only place that names these sets for the
/// decision; the `db_charset` override changes the decoder, not the decision.
pub(crate) const LOB_CONFIRMED_DB_SETS: &[u16] = &[873, 171];

/// A character set the reader can decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charset {
    /// US7ASCII (1): bytes below 0x80; a higher byte is a value error (ADR-16).
    Us7Ascii,
    /// CL8MSWIN1251 (171).
    Win1251,
    /// WE8MSWIN1252 (178).
    Win1252,
    /// AL32UTF8 (873).
    Utf8,
    /// AL16UTF16 (2000), big-endian code units with surrogate pairs.
    Utf16Be,
    /// Any other `encoding_rs` encoding, chosen by the caller's override (architecture 6.4); the caller's
    /// responsibility.
    Other(&'static encoding_rs::Encoding),
}

fn bad_text(message: &'static str) -> Error {
    Error::new(ErrorCode::BadText, message)
}

impl Charset {
    /// The set of an Oracle character set id; `None` for an id the reader does not decode [X3].
    pub fn from_id(id: u16) -> Option<Self> {
        match id {
            1 => Some(Self::Us7Ascii),
            171 => Some(Self::Win1251),
            178 => Some(Self::Win1252),
            873 => Some(Self::Utf8),
            2000 => Some(Self::Utf16Be),
            _ => None,
        }
    }

    /// The Oracle name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Us7Ascii => "US7ASCII",
            Self::Win1251 => "CL8MSWIN1251",
            Self::Win1252 => "WE8MSWIN1252",
            Self::Utf8 => "AL32UTF8",
            Self::Utf16Be => "AL16UTF16",
            Self::Other(e) => e.name(),
        }
    }

    /// The set of an override text: an Oracle name (case does not matter), `ascii` / `us-ascii` (the
    /// WHATWG label of US-ASCII is windows-1252, which is not what the caller means), or any label that
    /// `encoding_rs::Encoding::for_label` knows. `None` for anything else.
    pub fn from_name(name: &str) -> Option<Self> {
        let t = name.trim();
        match t.to_ascii_uppercase().as_str() {
            "US7ASCII" | "ASCII" | "US-ASCII" => return Some(Self::Us7Ascii),
            "CL8MSWIN1251" => return Some(Self::Win1251),
            "WE8MSWIN1252" => return Some(Self::Win1252),
            "AL32UTF8" => return Some(Self::Utf8),
            "AL16UTF16" => return Some(Self::Utf16Be),
            _ => {}
        }
        let e = encoding_rs::Encoding::for_label(t.as_bytes())?;
        Some(if e == encoding_rs::UTF_8 {
            Self::Utf8
        } else if e == encoding_rs::WINDOWS_1251 {
            Self::Win1251
        } else if e == encoding_rs::WINDOWS_1252 {
            Self::Win1252
        } else if e == encoding_rs::UTF_16BE {
            Self::Utf16Be
        } else {
            Self::Other(e)
        })
    }

    /// True for the sets in which one byte is one character (`Other` is never counted as one).
    pub fn is_single_byte(self) -> bool {
        matches!(self, Self::Us7Ascii | Self::Win1251 | Self::Win1252)
    }

    /// Decodes the bytes of one value. A malformed sequence is `E-BAD-TEXT` (no replacement character).
    pub fn decode(self, bytes: &[u8]) -> Result<Cow<'_, str>> {
        match self {
            Self::Us7Ascii => std::str::from_utf8(bytes)
                .ok()
                .filter(|_| bytes.is_ascii())
                .map(Cow::Borrowed)
                .ok_or_else(|| bad_text("byte above 0x7f in a US7ASCII column (the text may be in another set: read it with db_charset)")),
            Self::Utf8 => std::str::from_utf8(bytes)
                .map(Cow::Borrowed)
                .map_err(|_| bad_text("text is not valid UTF-8")),
            Self::Win1251 => encoding_rs::WINDOWS_1251
                .decode_without_bom_handling_and_without_replacement(bytes)
                .ok_or_else(|| bad_text("text is not valid CL8MSWIN1251")),
            Self::Win1252 => encoding_rs::WINDOWS_1252
                .decode_without_bom_handling_and_without_replacement(bytes)
                .ok_or_else(|| bad_text("text is not valid WE8MSWIN1252")),
            Self::Utf16Be => encoding_rs::UTF_16BE
                .decode_without_bom_handling_and_without_replacement(bytes)
                .ok_or_else(|| bad_text("text is not valid UTF-16")),
            Self::Other(e) => e
                .decode_without_bom_handling_and_without_replacement(bytes)
                .ok_or_else(|| bad_text("text is not valid in the character set of the override")),
        }
    }
}

/// Decodes a name or a DDL text of the file (architecture 6.3, 6.5). A byte the set does not allow becomes
/// U+FFFD; the flag says whether that happened. `set` is `None` for a client set the reader cannot decode
/// (architecture 6.4): every byte above 0x7f becomes U+FFFD.
pub fn decode_text(set: Option<Charset>, raw: &[u8]) -> (String, bool) {
    let Some(set) = set else {
        return ascii_lossy(raw);
    };
    match set {
        Charset::Us7Ascii => ascii_lossy(raw),
        Charset::Utf8 => match std::str::from_utf8(raw) {
            Ok(s) => (s.to_string(), false),
            Err(_) => (String::from_utf8_lossy(raw).into_owned(), true),
        },
        Charset::Win1251 => lossy(encoding_rs::WINDOWS_1251, raw),
        Charset::Win1252 => lossy(encoding_rs::WINDOWS_1252, raw),
        Charset::Utf16Be => lossy(encoding_rs::UTF_16BE, raw),
        Charset::Other(e) => lossy(e, raw),
    }
}

fn lossy(e: &'static encoding_rs::Encoding, raw: &[u8]) -> (String, bool) {
    let (text, had_errors) = e.decode_without_bom_handling(raw);
    (text.into_owned(), had_errors)
}

fn ascii_lossy(raw: &[u8]) -> (String, bool) {
    let mut damaged = false;
    let s = raw
        .iter()
        .map(|&b| {
            if b.is_ascii() {
                char::from(b)
            } else {
                damaged = true;
                '\u{fffd}'
            }
        })
        .collect();
    (s, damaged)
}

/// Whether the client set wrote a replacement character in `raw` when the database set had a character it
/// lacks: `?`, and 0xbf with WE8MSWIN1252 [C2, exp.md 14.2]. Only a hint ("possibly damaged"): a `?` can be
/// a real question mark.
pub fn has_replacement_char(set: Option<Charset>, raw: &[u8]) -> bool {
    raw.iter()
        .any(|&b| b == b'?' || (b == 0xbf && set == Some(Charset::Win1252)))
}

/// A streaming decoder for a value that arrives in pieces (a direct-path LONG chain): a UTF-8 sequence may
/// end inside a piece, so a piece is never decoded alone for AL32UTF8 (ADR-15 item 3). Single-byte sets are
/// stateless.
pub struct PieceDecoder {
    set: Charset,
    decoder: Option<encoding_rs::Decoder>,
    scratch: String,
}

impl std::fmt::Debug for PieceDecoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PieceDecoder")
            .field("set", &self.set)
            .finish_non_exhaustive()
    }
}

impl PieceDecoder {
    /// A decoder for one value in `set`.
    pub fn new(set: Charset) -> Self {
        let decoder = match set {
            Charset::Utf8 => Some(encoding_rs::UTF_8.new_decoder_without_bom_handling()),
            Charset::Utf16Be => Some(encoding_rs::UTF_16BE.new_decoder_without_bom_handling()),
            Charset::Other(e) => Some(e.new_decoder_without_bom_handling()),
            _ => None,
        };
        Self {
            set,
            decoder,
            scratch: String::new(),
        }
    }

    /// Starts the next value.
    pub fn reset(&mut self) {
        *self = Self::new(self.set);
    }

    /// Feeds the next piece; calls `sink` with the text that is complete so far (possibly none). With
    /// `last` the end of the value is declared: an incomplete sequence is `E-BAD-TEXT`.
    pub fn feed(
        &mut self,
        piece: &[u8],
        last: bool,
        sink: &mut dyn FnMut(&str) -> Result<()>,
    ) -> Result<()> {
        let Some(dec) = self.decoder.as_mut() else {
            let text = self.set.decode(piece)?;
            return sink(&text);
        };
        let need = dec
            .max_utf8_buffer_length_without_replacement(piece.len())
            .unwrap_or(piece.len().saturating_mul(3).saturating_add(8));
        self.scratch.clear();
        self.scratch.reserve(need);
        let (result, read) =
            dec.decode_to_string_without_replacement(piece, &mut self.scratch, last);
        match result {
            encoding_rs::DecoderResult::InputEmpty if read == piece.len() => sink(&self.scratch),
            _ => Err(bad_text("text is not valid in its character set")),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    fn run(set: Charset, pieces: &[&[u8]]) -> Result<String> {
        let mut d = PieceDecoder::new(set);
        let mut out = String::new();
        for p in pieces {
            d.feed(p, false, &mut |t| {
                out.push_str(t);
                Ok(())
            })?;
        }
        d.feed(&[], true, &mut |t| {
            out.push_str(t);
            Ok(())
        })?;
        Ok(out)
    }

    #[test]
    fn a_utf8_sequence_split_between_pieces_decodes() {
        let s = run(Charset::Utf8, &[b"a\xd0", b"\xb4b"]).unwrap();
        assert_eq!(s, "a\u{434}b");
        // the same bytes cut into single bytes
        let one: Vec<&[u8]> = "a\u{434}\u{1f600}b".as_bytes().chunks(1).collect();
        assert_eq!(run(Charset::Utf8, &one).unwrap(), "a\u{434}\u{1f600}b");
    }

    #[test]
    fn malformed_text_is_a_value_error_never_a_replacement() {
        assert_eq!(
            run(Charset::Utf8, &[b"ab\xff"]).unwrap_err().code,
            ErrorCode::BadText
        );
        // incomplete at the end
        assert_eq!(
            run(Charset::Utf8, &[b"ab\xd0"]).unwrap_err().code,
            ErrorCode::BadText
        );
        assert_eq!(
            Charset::Us7Ascii.decode(b"a\xc3\xa9").unwrap_err().code,
            ErrorCode::BadText
        );
        assert_eq!(Charset::Us7Ascii.decode(b"abc").unwrap(), "abc");
    }

    #[test]
    fn single_byte_sets_map_every_byte() {
        // 0xe4 is U+0434 in windows-1251 and U+00E4 in windows-1252
        assert_eq!(Charset::Win1251.decode(&[0xe4]).unwrap(), "\u{434}");
        assert_eq!(Charset::Win1252.decode(&[0xe4]).unwrap(), "\u{e4}");
        assert_eq!(Charset::from_id(171), Some(Charset::Win1251));
        assert_eq!(Charset::from_id(4), None);
    }

    #[test]
    fn utf16_pairs_split_between_pieces() {
        let s = run(
            Charset::Utf16Be,
            &[&[0x00, 0x61, 0xd8], &[0x3d, 0xde, 0x00]],
        )
        .unwrap();
        assert_eq!(s, "a\u{1f600}");
        assert!(run(Charset::Utf16Be, &[&[0xd8, 0x3d]]).is_err());
    }
}
