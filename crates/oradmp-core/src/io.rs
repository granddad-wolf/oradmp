//! Layer 1: a buffered byte stream with an absolute file offset over `Read + Seek`.
//!
//! Nothing is inserted at multiples of RECORDLENGTH inside the text region or the table data [R5, I7],
//! so this layer is a plain byte stream. Every read that cannot be satisfied returns `Truncated` with the
//! offset where the read began; no function here panics on any input.

use std::io::{ErrorKind, Read, Seek, SeekFrom};

use crate::error::{Error, ErrorCode, Result};

/// Default buffer size (architecture 4.1: 1 MiB).
pub const DEFAULT_BUFFER: usize = 1 << 20;
/// Smallest buffer the stream accepts; `peek` and `read_bytes` are limited to this size.
pub const MIN_BUFFER: usize = 64;

/// The smallest buffer a reader of rows uses (ADR-102 item 3): a conventional LONG field is at most 65,535 bytes and
/// the sink path of the cursor lends it whole, so a caller's smaller `buffer_bytes` is raised to this at every
/// `ByteStream::new` of `dump.rs` and `reader.rs`.
pub const MIN_BUFFER_BYTES: usize = 1 << 17;

/// The line feed byte.
pub const LF: u8 = 0x0a;

/// A buffered stream that knows its absolute offset and the size of the file.
#[derive(Debug)]
pub struct ByteStream<R> {
    inner: R,
    buf: Vec<u8>,
    /// Index of the next unread byte in `buf`.
    pos: usize,
    /// Number of valid bytes in `buf`.
    end: usize,
    /// File offset of `buf[0]`.
    start: u64,
    size: u64,
}

fn to_u64(n: usize) -> u64 {
    u64::try_from(n).unwrap_or(u64::MAX)
}

impl<R: Read + Seek> ByteStream<R> {
    /// Wraps `inner`, positioned at offset 0, with a buffer of `capacity` bytes (at least [`MIN_BUFFER`]).
    pub fn new(mut inner: R, capacity: usize) -> Result<Self> {
        let size = inner.seek(SeekFrom::End(0)).map_err(Error::io)?;
        inner.seek(SeekFrom::Start(0)).map_err(Error::io)?;
        Ok(Self {
            inner,
            buf: vec![0; capacity.max(MIN_BUFFER)],
            pos: 0,
            end: 0,
            start: 0,
            size,
        })
    }

    /// Size of the underlying file in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Absolute offset of the next unread byte.
    pub fn offset(&self) -> u64 {
        self.start.saturating_add(to_u64(self.pos))
    }

    fn available(&self) -> usize {
        self.end.saturating_sub(self.pos)
    }

    fn truncated(&self, message: &'static str) -> Error {
        Error::new(ErrorCode::Truncated, message).at(self.offset())
    }

    /// Makes at least `want` bytes available (fewer only at the end of the file).
    fn fill(&mut self, want: usize) -> Result<()> {
        let want = want.min(self.buf.len());
        if self.available() >= want {
            return Ok(());
        }
        if self.pos > 0 {
            if let Some(range_ok) = self.buf.get(self.pos..self.end).map(<[u8]>::len) {
                self.buf.copy_within(self.pos..self.end, 0);
                self.start = self.start.saturating_add(to_u64(self.pos));
                self.end = range_ok;
                self.pos = 0;
            }
        }
        while self.end < want {
            let Some(dst) = self.buf.get_mut(self.end..) else {
                break;
            };
            match self.inner.read(dst) {
                Ok(0) => break,
                Ok(n) => self.end = self.end.saturating_add(n),
                Err(e) if e.kind() == ErrorKind::Interrupted => {}
                Err(e) => return Err(Error::io(e).at(self.offset())),
            }
        }
        Ok(())
    }

    /// Up to `n` bytes (`n` at most the buffer size) without consuming them; fewer at the end of the file.
    pub fn peek(&mut self, n: usize) -> Result<&[u8]> {
        self.fill(n)?;
        let stop = self.pos.saturating_add(n).min(self.end);
        Ok(self.buf.get(self.pos..stop).unwrap_or(&[]))
    }

    /// Consumes up to `n` bytes that are already in the buffer.
    fn consume(&mut self, n: usize) {
        self.pos = self.pos.saturating_add(n).min(self.end);
    }

    /// Exactly `n` bytes (`n` at most the buffer size), consumed; `Truncated` if the file ends first.
    pub fn read_bytes(&mut self, n: usize) -> Result<&[u8]> {
        self.fill(n)?;
        if self.available() < n || n > self.buf.len() {
            return Err(self.truncated("file ends inside a field"));
        }
        let from = self.pos;
        self.consume(n);
        Ok(self.buf.get(from..self.pos).unwrap_or(&[]))
    }

    /// One byte.
    pub fn read_u8(&mut self) -> Result<u8> {
        self.read_bytes(1)?
            .first()
            .copied()
            .ok_or_else(|| self.truncated("file ends inside a field"))
    }

    /// Little-endian u16 (table data, field lengths).
    pub fn read_u16_le(&mut self) -> Result<u16> {
        let b = self.read_bytes(2)?;
        <[u8; 2]>::try_from(b)
            .map(u16::from_le_bytes)
            .map_err(|_| self.truncated("file ends inside a field"))
    }

    /// Little-endian u64 (the total of a long LOB value).
    pub fn read_u64_le(&mut self) -> Result<u64> {
        let b = self.read_bytes(8)?;
        <[u8; 8]>::try_from(b)
            .map(u64::from_le_bytes)
            .map_err(|_| self.truncated("file ends inside a field"))
    }

    /// Big-endian u16 (header).
    pub fn read_u16_be(&mut self) -> Result<u16> {
        let b = self.read_bytes(2)?;
        <[u8; 2]>::try_from(b)
            .map(u16::from_be_bytes)
            .map_err(|_| self.truncated("file ends inside a field"))
    }

    /// Skips `n` bytes without buffering them; `Truncated` (offset: where the skip began) if the file is
    /// shorter.
    pub fn skip(&mut self, n: u64) -> Result<()> {
        let here = self.offset();
        let target = match here.checked_add(n) {
            Some(t) if t <= self.size => t,
            _ => return Err(self.truncated("file ends inside a field")),
        };
        let in_buffer = to_u64(self.available());
        if n <= in_buffer {
            // n <= available, which is a usize, so the conversion cannot fail.
            self.consume(usize::try_from(n).unwrap_or(self.available()));
            return Ok(());
        }
        self.reposition(target)
    }

    /// Moves to an absolute offset (at most the file size).
    pub fn seek_to(&mut self, target: u64) -> Result<()> {
        if target > self.size {
            return Err(self.truncated("seek beyond the end of the file"));
        }
        let buf_end = self.start.saturating_add(to_u64(self.end));
        if target >= self.start && target <= buf_end {
            self.pos = usize::try_from(target.saturating_sub(self.start)).unwrap_or(self.end);
            return Ok(());
        }
        self.reposition(target)
    }

    fn reposition(&mut self, target: u64) -> Result<()> {
        self.inner
            .seek(SeekFrom::Start(target))
            .map_err(|e| Error::io(e).at(target))?;
        self.start = target;
        self.pos = 0;
        self.end = 0;
        Ok(())
    }

    /// Reads one line into `out` (cleared first), without the LF. A line longer than `max` bytes is
    /// `Framing` at the offset where the line began; the end of the file before the LF is `Truncated`.
    pub fn read_line(&mut self, out: &mut Vec<u8>, max: usize) -> Result<()> {
        out.clear();
        let line_start = self.offset();
        loop {
            self.fill(1)?;
            let avail = self.buf.get(self.pos..self.end).unwrap_or(&[]);
            if avail.is_empty() {
                return Err(self.truncated("file ends inside a line"));
            }
            let lf = avail.iter().position(|&b| b == LF);
            let take = lf.unwrap_or(avail.len());
            let new_len = out.len().saturating_add(take);
            if new_len > max {
                return Err(
                    Error::new(ErrorCode::Framing, "line longer than max_line_bytes")
                        .at(line_start)
                        .with_length(to_u64(new_len)),
                );
            }
            out.extend_from_slice(avail.get(..take).unwrap_or(&[]));
            self.consume(take);
            if lf.is_some() {
                self.consume(1);
                return Ok(());
            }
        }
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn stream(data: &[u8], cap: usize) -> ByteStream<Cursor<Vec<u8>>> {
        ByteStream::new(Cursor::new(data.to_vec()), cap).unwrap()
    }

    #[test]
    fn offset_follows_reads_across_buffer_refills() {
        let data: Vec<u8> = (0..=255u8).cycle().take(1000).collect();
        let mut s = stream(&data, MIN_BUFFER);
        let mut seen = Vec::new();
        for _ in 0..1000 {
            let o = s.offset();
            let b = s.read_u8().unwrap();
            assert_eq!(b, data[usize::try_from(o).unwrap()]);
            seen.push(b);
        }
        assert_eq!(seen, data);
        assert_eq!(s.offset(), 1000);
        assert_eq!(s.read_u8().unwrap_err().code, ErrorCode::Truncated);
    }

    #[test]
    fn u16_values_cross_a_refill_boundary() {
        let mut data = vec![0u8; 63];
        data.extend_from_slice(&[0x34, 0x12, 0x12, 0x34]);
        let mut s = stream(&data, MIN_BUFFER);
        s.skip(63).unwrap();
        assert_eq!(s.read_u16_le().unwrap(), 0x1234);
        assert_eq!(s.read_u16_be().unwrap(), 0x1234);
    }

    #[test]
    fn read_u64_le_reads_eight_bytes_and_a_short_read_is_truncated_at_its_start() {
        let mut s = stream(&[1, 0, 0, 0, 0, 0, 0, 0], MIN_BUFFER);
        assert_eq!(s.read_u64_le().unwrap(), 1);
        let mut s = stream(&[1, 0, 0, 0, 0, 0, 0], MIN_BUFFER);
        let e = s.read_u64_le().unwrap_err();
        assert_eq!(e.code, ErrorCode::Truncated);
        assert_eq!(e.message, "file ends inside a field");
        assert_eq!(e.offset, Some(0));
    }

    #[test]
    fn skip_beyond_the_end_is_truncated_with_the_start_offset() {
        let mut s = stream(&[1, 2, 3], MIN_BUFFER);
        s.read_u8().unwrap();
        let e = s.skip(5).unwrap_err();
        assert_eq!(e.code, ErrorCode::Truncated);
        assert_eq!(e.offset, Some(1));
    }

    #[test]
    fn skip_of_more_than_the_buffer_repositions() {
        let data: Vec<u8> = (0..=255u8).cycle().take(5000).collect();
        let mut s = stream(&data, MIN_BUFFER);
        s.read_u8().unwrap();
        s.skip(3000).unwrap();
        assert_eq!(s.offset(), 3001);
        assert_eq!(s.read_u8().unwrap(), data[3001]);
    }

    #[test]
    fn read_line_longer_than_buffer_and_limit() {
        let mut data = vec![b'a'; 300];
        data.push(LF);
        data.extend_from_slice(b"next\n");
        let mut s = stream(&data, MIN_BUFFER);
        let mut line = Vec::new();
        s.read_line(&mut line, 300).unwrap();
        assert_eq!(line.len(), 300);
        s.read_line(&mut line, 300).unwrap();
        assert_eq!(line, b"next");
        let mut s = stream(&data, MIN_BUFFER);
        let e = s.read_line(&mut line, 299).unwrap_err();
        assert_eq!(e.code, ErrorCode::Framing);
        assert_eq!(e.offset, Some(0));
    }

    #[test]
    fn read_line_without_lf_is_truncated() {
        let mut s = stream(b"abc", MIN_BUFFER);
        let mut line = Vec::new();
        assert_eq!(
            s.read_line(&mut line, 10).unwrap_err().code,
            ErrorCode::Truncated
        );
    }

    #[test]
    fn seek_back_inside_and_outside_the_buffer() {
        let data: Vec<u8> = (0..=255u8).cycle().take(2000).collect();
        let mut s = stream(&data, MIN_BUFFER);
        s.skip(1500).unwrap();
        s.seek_to(1499).unwrap();
        assert_eq!(s.read_u8().unwrap(), data[1499]);
        s.seek_to(10).unwrap();
        assert_eq!(s.read_u8().unwrap(), data[10]);
        assert_eq!(s.seek_to(2001).unwrap_err().code, ErrorCode::Truncated);
        s.seek_to(2000).unwrap();
        assert!(s.peek(4).unwrap().is_empty());
    }
}
