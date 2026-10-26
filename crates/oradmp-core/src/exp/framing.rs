//! Layer 2, framing: line, plain field, field in pieces, pieces without marker [I2 to I5], and the
//! fallback G1. Skipped fields are never buffered (architecture 2.6).

use std::io::{Read, Seek};

use super::intro::FormSet;
use crate::error::{Error, ErrorCode, Result};
use crate::io::{ByteStream, LF};

/// The marker that opens a field in pieces [I4].
pub const MARKER: [u8; 2] = [0xfd, 0xff];

/// The form in which a field was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldForm {
    /// u16 LE length, bytes, LF [I3].
    Plain,
    /// `fd ff`, pieces up to a piece of length 0, LF [I4].
    Marked,
    /// Pieces up to a piece of length 0, LF, no marker [I5].
    Unmarked,
}

fn framing(message: &'static str, offset: u64) -> Error {
    Error::new(ErrorCode::Framing, message).at(offset)
}

/// Consumes the LF that ends a field.
fn expect_lf<R: Read + Seek>(s: &mut ByteStream<R>) -> Result<()> {
    let at = s.offset();
    if s.read_u8()? == LF {
        Ok(())
    } else {
        Err(framing("field is not followed by a line feed", at))
    }
}

/// Pieces `u16 LE n + n bytes` up to a piece with n = 0, then LF.
fn read_pieces<R: Read + Seek>(s: &mut ByteStream<R>) -> Result<()> {
    loop {
        let n = s.read_u16_le()?;
        if n == 0 {
            break;
        }
        s.skip(u64::from(n))?;
    }
    expect_lf(s)
}

/// Plain field: `u16 LE n`, n bytes, LF.
fn read_plain<R: Read + Seek>(s: &mut ByteStream<R>) -> Result<()> {
    let n = s.read_u16_le()?;
    s.skip(u64::from(n))?;
    expect_lf(s)
}

/// Does the next item look like a field (rule G1): its first two bytes are `fd ff`, or its second byte is
/// below 0x20? At the end of the file there is nothing to look at.
pub fn looks_like_field<R: Read + Seek>(s: &mut ByteStream<R>) -> Result<bool> {
    let two = s.peek(2)?;
    Ok(match two {
        [a, b] => (*a == MARKER[0] && *b == MARKER[1]) || *b < 0x20,
        _ => false,
    })
}

/// Reads the one field that follows a line of list K, in a form the entry allows [I6]. `ANALSTATS ` allows
/// plain and marked: the two bytes decide [S1]. A form the entry does not allow is `Framing`.
pub fn read_listed_field<R: Read + Seek>(
    s: &mut ByteStream<R>,
    forms: FormSet,
) -> Result<FieldForm> {
    let at = s.offset();
    let is_marker = s.peek(2)? == MARKER;
    if is_marker {
        if !forms.marked {
            return Err(framing("field form not allowed after this line", at));
        }
        s.skip(2)?;
        read_pieces(s)?;
        Ok(FieldForm::Marked)
    } else if forms.plain {
        read_plain(s)?;
        Ok(FieldForm::Plain)
    } else if forms.unmarked {
        read_pieces(s)?;
        Ok(FieldForm::Unmarked)
    } else {
        Err(framing("field form not allowed after this line", at))
    }
}

/// Rule G1: the next item is a field. `fd ff` opens pieces; otherwise read `u16 n + n bytes` and, if the
/// next byte is not LF, read on as pieces up to a piece of length 0.
pub fn read_unlisted_field<R: Read + Seek>(s: &mut ByteStream<R>) -> Result<FieldForm> {
    if s.peek(2)? == MARKER {
        s.skip(2)?;
        read_pieces(s)?;
        return Ok(FieldForm::Marked);
    }
    let n = s.read_u16_le()?;
    s.skip(u64::from(n))?;
    let at = s.offset();
    if s.peek(1)?.first() == Some(&LF) {
        s.skip(1)?;
        return Ok(FieldForm::Plain);
    }
    if s.peek(1)?.is_empty() {
        return Err(Error::new(ErrorCode::Truncated, "file ends inside a field").at(at));
    }
    read_pieces(s)?;
    Ok(FieldForm::Unmarked)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;
    use crate::io::MIN_BUFFER;
    use std::io::Cursor;

    fn stream(data: &[u8]) -> ByteStream<Cursor<Vec<u8>>> {
        ByteStream::new(Cursor::new(data.to_vec()), MIN_BUFFER).unwrap()
    }

    const ALL: FormSet = FormSet {
        plain: true,
        marked: true,
        unmarked: true,
    };

    /// ADR-65 item 3: no fixture has an unlisted introducing line, so the one "file ends" site of
    /// `read_unlisted_field` is reached here: a field `03 00 61 62 63` whose end is the end of the input is
    /// `E-TRUNCATED` at offset 5 (after the 3 value bytes), "file ends inside a field".
    #[test]
    fn read_unlisted_field_at_the_end_of_the_input_is_truncated() {
        let mut s = stream(&[3, 0, b'a', b'b', b'c']);
        let e = read_unlisted_field(&mut s).unwrap_err();
        assert_eq!(e.code, ErrorCode::Truncated);
        assert_eq!(e.offset, Some(5));
        assert_eq!(e.message, "file ends inside a field");
        // with an LF after the field it is a plain field; without one the pieces follow (a length 0 ends them)
        let mut s = stream(&[3, 0, b'a', b'b', b'c', LF]);
        assert_eq!(read_unlisted_field(&mut s).unwrap(), FieldForm::Plain);
    }

    #[test]
    fn plain_field_may_contain_lf_and_is_followed_by_lf() {
        let mut s = stream(&[3, 0, b'a', LF, b'b', LF, b'X']);
        assert_eq!(read_listed_field(&mut s, ALL).unwrap(), FieldForm::Plain);
        assert_eq!(s.offset(), 7 - 1);
    }

    #[test]
    fn plain_field_without_lf_is_framing_at_the_missing_lf() {
        let mut s = stream(&[1, 0, b'a', b'x', LF]);
        let e = read_listed_field(&mut s, ALL).unwrap_err();
        assert_eq!(e.code, ErrorCode::Framing);
        assert_eq!(e.offset, Some(3));
    }

    #[test]
    fn marked_pieces_end_at_a_zero_piece_then_lf() {
        let mut s = stream(&[0xfd, 0xff, 2, 0, b'a', b'b', 1, 0, b'c', 0, 0, LF, b'Z']);
        assert_eq!(read_listed_field(&mut s, ALL).unwrap(), FieldForm::Marked);
        assert_eq!(s.offset(), 12);
    }

    #[test]
    fn marker_after_a_plain_only_entry_is_framing() {
        let plain_only = FormSet {
            plain: true,
            ..FormSet::default()
        };
        let mut s = stream(&[0xfd, 0xff, 0, 0, LF]);
        assert_eq!(
            read_listed_field(&mut s, plain_only).unwrap_err().code,
            ErrorCode::Framing
        );
    }

    #[test]
    fn unmarked_pieces_for_views() {
        let only = FormSet {
            unmarked: true,
            ..FormSet::default()
        };
        let mut s = stream(&[2, 0, b'a', b'b', 0, 0, LF]);
        assert_eq!(
            read_listed_field(&mut s, only).unwrap(),
            FieldForm::Unmarked
        );
        assert_eq!(s.offset(), 7);
    }

    #[test]
    fn g1_reads_the_three_forms() {
        let mut s = stream(&[1, 0, b'a', LF]);
        assert_eq!(read_unlisted_field(&mut s).unwrap(), FieldForm::Plain);
        let mut s = stream(&[0xfd, 0xff, 1, 0, b'a', 0, 0, LF]);
        assert_eq!(read_unlisted_field(&mut s).unwrap(), FieldForm::Marked);
        let mut s = stream(&[1, 0, b'a', 1, 0, b'b', 0, 0, LF]);
        assert_eq!(read_unlisted_field(&mut s).unwrap(), FieldForm::Unmarked);
        assert_eq!(s.offset(), 9);
    }

    #[test]
    fn g1_detection() {
        assert!(looks_like_field(&mut stream(&[0xfd, 0xff, 0])).unwrap());
        assert!(looks_like_field(&mut stream(&[9, 0x10, 0])).unwrap());
        assert!(!looks_like_field(&mut stream(b"EXIT\n")).unwrap());
        assert!(!looks_like_field(&mut stream(b"E")).unwrap());
    }
}
