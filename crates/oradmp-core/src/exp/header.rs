//! Layer 2, first and second record: the header [H1 to H5], and the first item at R + 0x104 [H6].
//!
//! Everything after the seven text lines has no fixed offset [H3]. The date text and the file name
//! [H4a] are not needed by a reader of data and are not parsed.

use std::io::{Read, Seek};

use crate::error::{Error, ErrorCode, Result, Warning, WarningCode};
use crate::io::ByteStream;

/// Longest header text line accepted (the lines of the fixtures are under 20 bytes).
const MAX_HEADER_LINE: usize = 1024;
/// Offset, from the start of the second record, of its first item [H6].
pub const FIRST_ITEM_DELTA: u64 = 0x104;
/// Versions of the fixtures [H3].
const TESTED_VERSIONS: [&[u8]; 2] = [b"EXPORT:V18.00.00", b"EXPORT:V11.02.00"];
/// RECORDLENGTH values of the fixtures [H5].
const TESTED_RECORD_LENGTHS: [u64; 4] = [2048, 4096, 8192, 16384];

/// Export mode of the `R` line [H3].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    /// `RTABLES`: table mode, the first item is `METRICST`.
    Tables,
    /// `RUSERS`: owner mode, the first item is `METRICSU`.
    Users,
    /// Any other text (unknown; reported as `W-UNTESTED-HEADER`).
    Other(Vec<u8>),
}

/// The fields of the first record that a reader needs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// Character set id of the export client, bytes 1-2 [H1].
    pub client_charset: u16,
    /// Client set id as written again after the text lines [H4]; equal to `client_charset` in every fixture.
    pub client_charset_again: u16,
    /// Database character set id [H4].
    pub db_charset: u16,
    /// National character set id [H4].
    pub national_charset: u16,
    /// `D` LF at bytes 3-4: the export was started with `direct=y` [H2]. Not the path of a table [T6].
    pub direct_flag: bool,
    /// Version line, for example `EXPORT:V11.02.00`.
    pub version: Vec<u8>,
    /// User, the `U` line without its letter (client character set).
    pub user: Vec<u8>,
    /// Mode, the `R` line without its letter.
    pub mode: Mode,
    /// RECORDLENGTH `R` [H3, H5].
    pub record_length: u64,
    /// Size of the file.
    pub file_size: u64,
    /// Offset of the first item: R + 0x104 [H6].
    pub first_item_offset: u64,
}

fn header_err(message: &'static str, offset: u64) -> Error {
    Error::new(ErrorCode::BadHeader, message).at(offset)
}

/// Reads one header line. A file that ends or a line that is too long is `BadHeader`.
fn header_line<R: Read + Seek>(s: &mut ByteStream<R>, line: &mut Vec<u8>) -> Result<()> {
    let at = s.offset();
    s.read_line(line, MAX_HEADER_LINE)
        .map_err(|_| header_err("header text line is missing or too long", at))
}

/// Known compressed formats, by their first bytes.
fn compression_magic(b: &[u8]) -> bool {
    b.starts_with(&[0x1f, 0x8b])
        || b.starts_with(b"PK\x03\x04")
        || b.starts_with(b"BZh")
        || b.starts_with(&[0x28, 0xb5, 0x2f, 0xfd])
        || b.starts_with(&[0xfd, b'7', b'z', b'X', b'Z', 0x00])
}

/// Reads the header [H1 to H5] from offset 0. Returns the header and the warnings of the header.
pub fn read_header<R: Read + Seek>(s: &mut ByteStream<R>) -> Result<(Header, Vec<Warning>)> {
    s.seek_to(0)?;
    let file_size = s.size();
    let first = s.peek(8)?;
    if compression_magic(first) {
        return Err(Error::new(
            ErrorCode::NotExpFile,
            "compressed input is not supported (gzip, zip, bzip2, xz or zstd magic bytes)",
        )
        .at(0));
    }
    if first.first() != Some(&0x03) {
        return Err(Error::new(ErrorCode::NotExpFile, "byte 0 is not 03").at(0));
    }
    s.skip(1)?;
    let client_charset = s
        .read_u16_be()
        .map_err(|_| header_err("header ends early", 1))?;
    let direct_flag = s.peek(2)? == b"D\n";
    if direct_flag {
        s.skip(2)?;
    }
    let line1_at = s.offset();
    if !s.peek(8)?.starts_with(b"EXPORT:V") {
        return Err(
            Error::new(ErrorCode::NotExpFile, "line 1 does not start with EXPORT:V").at(line1_at),
        );
    }
    let mut line = Vec::new();
    header_line(s, &mut line)?;
    let version = line.clone();

    let at = s.offset();
    header_line(s, &mut line)?;
    let user = line
        .strip_prefix(b"U")
        .ok_or_else(|| header_err("user line does not start with U", at))?
        .to_vec();

    let at = s.offset();
    header_line(s, &mut line)?;
    let mode_text = line
        .strip_prefix(b"R")
        .ok_or_else(|| header_err("mode line does not start with R", at))?
        .to_vec();
    let mode = match mode_text.as_slice() {
        b"TABLES" => Mode::Tables,
        b"USERS" => Mode::Users,
        _ => Mode::Other(mode_text),
    };

    let rl_at = s.offset();
    header_line(s, &mut line)?;
    let record_length = parse_record_length(&line, rl_at)?;
    // Three more text lines: 0, 72, 0 (meaning unknown) [H3]; another text is `W-UNTESTED-HEADER` (ADR-17).
    let mut odd_lines = None;
    for expected in [&b"0"[..], b"72", b"0"] {
        let at = s.offset();
        header_line(s, &mut line)?;
        if line != expected && odd_lines.is_none() {
            odd_lines = Some(at);
        }
    }

    let bin_at = s.offset();
    let client_charset_again = s
        .read_u16_be()
        .map_err(|_| header_err("header ends early", bin_at))?;
    let db_charset = s
        .read_u16_be()
        .map_err(|_| header_err("header ends early", bin_at))?;
    let national_charset = s
        .read_u16_be()
        .map_err(|_| header_err("header ends early", bin_at))?;
    // 10 bytes of unknown meaning, the u16 BE length of the file name and one 00 [H4]; another byte than
    // `00` is `W-UNTESTED-HEADER` (ADR-17).
    s.skip(12)
        .map_err(|_| header_err("header ends early", bin_at))?;
    let zero_at = s.offset();
    let after_name_length = s
        .read_u8()
        .map_err(|_| header_err("header ends early", bin_at))?;
    let header_end = s.offset();

    if record_length < header_end {
        return Err(Error::new(
            ErrorCode::BadRecordLength,
            "RECORDLENGTH is smaller than the header",
        )
        .at(rl_at)
        .with_length(record_length));
    }
    let first_item_offset = record_length.saturating_add(FIRST_ITEM_DELTA);
    if first_item_offset >= file_size {
        return Err(Error::new(
            ErrorCode::BadRecordLength,
            "the second record lies beyond the end of the file",
        )
        .at(rl_at)
        .with_length(record_length));
    }

    let mut warnings = Vec::new();
    if !TESTED_VERSIONS.contains(&version.as_slice()) {
        warnings.push(
            Warning::new(
                WarningCode::UntestedVersion,
                "version line is not one of the tested versions",
                line1_at,
            )
            .with_length(u64::try_from(version.len()).unwrap_or(u64::MAX)),
        );
    }
    if matches!(mode, Mode::Other(_)) || !TESTED_RECORD_LENGTHS.contains(&record_length) {
        warnings.push(
            Warning::new(
                WarningCode::UntestedHeader,
                "mode line or RECORDLENGTH is outside the tested values",
                rl_at,
            )
            .with_length(record_length),
        );
    }
    if let Some(at) = odd_lines {
        warnings.push(Warning::new(
            WarningCode::UntestedHeader,
            "the lines after RECORDLENGTH are not 0, 72, 0",
            at,
        ));
    }
    if after_name_length != 0 {
        warnings.push(Warning::new(
            WarningCode::UntestedHeader,
            "the byte after the file-name length is not 00",
            zero_at,
        ));
    }
    if file_size.checked_rem(record_length) != Some(0) {
        warnings.push(
            Warning::new(
                WarningCode::FileSize,
                "file size is not a multiple of RECORDLENGTH",
                0,
            )
            .with_length(file_size),
        );
    }

    Ok((
        Header {
            client_charset,
            client_charset_again,
            db_charset,
            national_charset,
            direct_flag,
            version,
            user,
            mode,
            record_length,
            file_size,
            first_item_offset,
        },
        warnings,
    ))
}

/// RECORDLENGTH as decimal text [H3]: digits only, 1 to 10 of them, not zero.
fn parse_record_length(text: &[u8], offset: u64) -> Result<u64> {
    let bad = |m: &'static str| Error::new(ErrorCode::BadRecordLength, m).at(offset);
    if text.is_empty() || text.len() > 10 || !text.iter().all(u8::is_ascii_digit) {
        return Err(bad("RECORDLENGTH is not a decimal number"));
    }
    let n = text.iter().fold(0u64, |acc, d| {
        acc.saturating_mul(10)
            .saturating_add(u64::from(d.saturating_sub(b'0')))
    });
    if n == 0 {
        return Err(bad("RECORDLENGTH is 0"));
    }
    Ok(n)
}

/// Moves to R + 0x104 and verifies that the first item is the line `METRICST` or `METRICSU` [H6]. On
/// success the stream stands behind that line. The pairing with the mode line (`RTABLES` - `METRICST`,
/// `RUSERS` - `METRICSU`, confirmed in 177 files) is checked: another pairing is `W-UNTESTED-HEADER`
/// (ADR-17); the missing line itself is `MissingMetricsLine`.
pub fn read_first_item<R: Read + Seek>(
    s: &mut ByteStream<R>,
    header: &Header,
) -> Result<Vec<Warning>> {
    let at = header.first_item_offset;
    let missing = |m: &'static str| Error::new(ErrorCode::MissingMetricsLine, m).at(at);
    s.seek_to(at)?;
    let mut line = Vec::new();
    s.read_line(&mut line, 64)
        .map_err(|_| missing("the first item is not a METRICST or METRICSU line"))?;
    if line != b"METRICST" && line != b"METRICSU" {
        return Err(missing("the first item is not a METRICST or METRICSU line"));
    }
    let paired = match header.mode {
        Mode::Tables => line == b"METRICST",
        Mode::Users => line == b"METRICSU",
        Mode::Other(_) => true,
    };
    let mut warnings = Vec::new();
    if !paired {
        warnings.push(Warning::new(
            WarningCode::UntestedHeader,
            "the mode line and the first item (METRICST / METRICSU) are not a confirmed pair",
            at,
        ));
    }
    Ok(warnings)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;

    #[test]
    fn record_length_text() {
        assert_eq!(parse_record_length(b"2048", 0).unwrap(), 2048);
        for bad in [&b""[..], b"0", b"00", b"20x8", b"-1", b"99999999999"] {
            assert_eq!(
                parse_record_length(bad, 0).unwrap_err().code,
                ErrorCode::BadRecordLength
            );
        }
    }

    #[test]
    fn compressed_magic_bytes() {
        assert!(compression_magic(&[0x1f, 0x8b, 8]));
        assert!(compression_magic(b"PK\x03\x04zz"));
        assert!(compression_magic(b"BZh9"));
        assert!(!compression_magic(&[3, 0, 0x33]));
    }
}
