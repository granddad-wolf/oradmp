//! Layer 4, rows: fields by their length, end of rows, both empty-table forms, the LOB value area
//! [R1 to R8, E1 to E3, E2a, L2 to L4]. Values are skipped by length; nothing is decoded (tasks T-1,
//! T-3a). Task T-3c adds the direct-path LONG / LONG RAW piece chains [R2, Y9, ADR-15]. Task T-19 (ADR-95, rules
//! v3.7) replaces the stop for a conventional LONG length from 0x8000 by the two forms R-040 found. Task T-19b
//! (ADR-99, rules v3.9) reads the three forms of [R2] for LONG and LONG RAW alike: one field up to 65,529 bytes
//! (0xfff2 to 0xfff4 with a warning), the 4-byte-length form `fa ff` for 65,530 to 65,533 bytes, the chunked `fd ff`
//! form from 65,534 bytes; a first u16 of 0xfffb or 0xfffc stops by name [R8]. With `hash` set (task T-2) the raw column fields
//! are also fed to SHA-256 in the serialisation of [`RawHasher`]. Task T-3b adds, in the same mode, a digest
//! over the LOB values and one over the assembled values of each LONG / LONG RAW column (ADR-29 item 6),
//! and counts the LOB values per column. Task T-6L gives the LOB value area to a [`RowSink`]
//! (`lob_start`, `lob_chunk`, `lob_end`).

use std::io::{Read, Seek};

use sha2::{Digest, Sha256};

use super::framing;
use super::intro::IntroList;
use super::table::{is_lob_type, is_long_type, PathKind, TableHeader};
use crate::error::{Error, ErrorCode, Result};
use crate::io::{ByteStream, LF, MIN_BUFFER};

/// How the rows of a table ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EndMarker {
    /// `ff ff` where a row would start [E1].
    FfFf,
    /// Direct path, no rows, no `ff ff`: an LF directly after the path flag [E2].
    LfOnly,
}

/// The form of a LOB value [L4].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LobForm {
    /// `u16 n` + n bytes; n = 0 is an empty LOB.
    Short,
    /// `fc ff`, 8 bytes total length, chunks.
    Long,
}

/// Counts over the LOB values of a table (no byte of a value is read into memory) [L3, L4].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct LobSummary {
    /// All LOB values: `short` + `long`.
    pub values: u64,
    /// Values in the short form, empty ones included.
    pub short: u64,
    /// Values of length 0 (a subset of `short`).
    pub empty: u64,
    /// Values in the long form.
    pub long: u64,
    /// Sum of the stored byte lengths (n of the short form, T of the long form).
    pub stored_bytes: u64,
    /// The largest stored byte length.
    pub largest: u64,
}

/// The LOB value counts of one LOB column of a part (`column` is the 1-based file position).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LobColumn {
    /// 1-based position of the column in the file order of the part.
    pub column: u32,
    /// Counts over the values of this column.
    pub stats: LobSummary,
}

/// One LOB value, recorded when the walk was asked to (`record_lob`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LobValueRecord {
    /// 1-based row number.
    pub row: u64,
    /// 1-based position of the column the value belongs to.
    pub column: u32,
    /// Offset of the value's first u16.
    pub offset: u64,
    /// Short or long.
    pub form: LobForm,
    /// Stored byte length (n or T).
    pub total: u64,
    /// The chunk lengths of a long value (empty for a short one).
    pub chunks: Vec<u16>,
}

/// What `record_lob` keeps: k of every row and every value.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LobDetail {
    /// The u16 after each row (k, or 0), with the offset where it stands.
    pub ks: Vec<(u64, u16)>,
    /// The values in file order.
    pub values: Vec<LobValueRecord>,
}

/// A value of a LONG / LONG RAW column above this many bytes is counted in [`LongStats::above_32000`] (the name and
/// the meaning are kept since T-6; `W-LONG-LENGTH` has its own thresholds since T-19: 100,000 bytes for LONG, this
/// limit for LONG RAW, ADR-95 item 9).
pub const LONG_LIMIT_BYTES: u64 = 32_000;

/// What the scan counts per LONG / LONG RAW column of a part (arch 3.2 `LongStats`; no byte of a value is
/// read into memory). NULL fields are not values. For the survey and for `ColumnInfo.long` (T-3b).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LongStats {
    /// 1-based position of the column in `INSERT INTO` order.
    pub column: u32,
    /// Non-NULL values.
    pub values: u64,
    /// Values made of more than one piece (direct path only).
    pub chained: u64,
    /// Values in the conventional `fd ff` chunked form (T-19, ADR-95 item 8); `chained` stays the direct-path count.
    pub chunked: u64,
    /// Chunked values with a total below 65,534 bytes ([`CHUNKED_MIN_TOTAL`], ADR-99 item 2): the walker warns.
    pub chunked_short: u64,
    /// Row (1-based) and offset of the first such value (its first data byte).
    pub first_chunked_short: Option<(u64, u64)>,
    /// The largest total length of a value (sum of the pieces).
    pub largest: u64,
    /// Values with a total above [`LONG_LIMIT_BYTES`].
    pub above_32000: u64,
    /// Pieces of length 0 seen in chains (`00 80` or a final `00 00`) [ADR-15]: the walker warns.
    pub zero_pieces: u64,
    /// Row (1-based) and offset of the u16 of the first such piece.
    pub first_zero_piece: Option<(u64, u64)>,
    /// Conventional fields whose first u16 is 0xfff2 to 0xfff4, read as one field (ADR-101 item 1): the walker warns.
    pub fff2_to_fff4: u64,
    /// Row (1-based) and offset of the u16 of the first such field.
    pub first_fff2_to_fff4: Option<(u64, u64)>,
    /// Non-NULL values with a total of 65,521 to 65,534 bytes, any form or path (ADR-101 item 4).
    pub sizes_65521_to_65534: u64,
}

impl LongStats {
    /// Counts one non-NULL value of `total` bytes (all paths and forms).
    fn count_value(&mut self, total: u64) {
        self.values = self.values.saturating_add(1);
        self.largest = self.largest.max(total);
        if total > LONG_LIMIT_BYTES {
            self.above_32000 = self.above_32000.saturating_add(1);
        }
        if (65_521..=65_534).contains(&total) {
            self.sizes_65521_to_65534 = self.sizes_65521_to_65534.saturating_add(1);
        }
    }
}

/// A long LOB value with total length 0 (`fc ff` + 8 zero bytes, no chunk): the first one of a part
/// [L4, ADR-23].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LobZeroTotal {
    /// Offset of the `fc ff`.
    pub offset: u64,
    /// 1-based row.
    pub row: u64,
    /// 1-based column position of the LOB column.
    pub column: u32,
}

/// What the row walk found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RowsSummary {
    /// Number of rows [R6: counted, never read from METRICET].
    pub rows: u64,
    /// Number of column fields: rows times columns.
    pub fields: u64,
    /// The end marker.
    pub end_marker: EndMarker,
    /// Offset of the end marker (of `ff ff`, or of the LF for `LfOnly`).
    pub end_marker_offset: u64,
    /// Offset of the first byte after the LF that closes the rows.
    pub rows_end_offset: u64,
    /// Number of NULL fields (`fe ff`) over all rows and columns.
    pub nulls: u64,
    /// SHA-256 of the raw fields, if hashing was requested (see [`RawHasher`]).
    pub raw_sha256: Option<[u8; 32]>,
    /// Counts of the LOB values (all 0 for a table without a LOB column).
    pub lob: LobSummary,
    /// k per row and the values, if `record_lob` was set.
    pub lob_detail: Option<LobDetail>,
    /// The "no rows" decision of [E2a] was taken (column 1 of length 0 on the direct path): the walker
    /// raises `W-UNTESTED-LAYOUT` (ADR-22).
    pub e2a_no_rows: bool,
    /// One entry per LONG / LONG RAW column, in column order (empty for a table without one).
    pub long: Vec<LongStats>,
    /// The first long LOB value of total 0, if any [ADR-23].
    pub lob_zero_total: Option<LobZeroTotal>,
    /// One entry per LOB column, in column order (empty for a table without one): the counts of `lob` split
    /// by column.
    pub lob_columns: Vec<LobColumn>,
    /// With hashing: SHA-256 over all LOB values of the part, in file order, each as its stored length
    /// (u64 little-endian) and its bytes (long values: the chunks joined). `None` without hashing or
    /// without a LOB column.
    pub lob_sha256: Option<[u8; 32]>,
    /// With hashing: per LONG / LONG RAW column (same order as `long`), SHA-256 over the **assembled**
    /// values: a NULL is 8 bytes `ff`, any other value its total length (u64 little-endian) and its bytes
    /// (the pieces of a chain joined). Empty without hashing.
    pub long_sha256: Vec<[u8; 32]>,
}

/// The serialisation of the raw fields of a table, in file order, row after row, column after column:
/// a NULL field is the four bytes `ff ff ff ff`; any other field is its length as u32 little-endian
/// followed by its bytes exactly as they stand in the file (the locator of a LOB column is such a field;
/// LOB values are not part of it). Nothing stands between rows and nothing after the last one. The digest
/// is SHA-256 of that byte string. (Plain lengths are at most 65,529, so a NULL cannot be mistaken for a length.)
/// The same serialisation is written by `tests/golden/gen_exp_raw.py`. Task T-3c: a direct-path LONG /
/// LONG RAW chain is the four bytes `fd ff ff ff`, then per piece its length (without bit 0x8000) as u32
/// little-endian and its bytes, then `fc ff ff ff` (neither marker can be a length or a NULL). Task T-19
/// (ADR-95 item 10): a conventional LONG in the chunked `fd ff` form is the four bytes `fb ff ff ff`, per data
/// chunk its length as u32 little-endian and its bytes, then `fa ff ff ff` (the final `00 00` is not hashed);
/// a plain field from 0x8000 is serialised as any field. Task T-19b (ADR-99 item 5, [R2] v3.9): a conventional
/// value in the 4-byte-length form (b) (`fa ff`, u16 n, u16 z, n bytes) is the four bytes `f9 ff ff ff`, n as u32
/// little-endian and the n bytes (the six framing bytes are not hashed), so that a golden line tells the three forms
/// apart (`f9 ff ff ff` is neither a u32 length below 0x10000 nor another marker).
pub struct RawHasher {
    raw: Sha256,
    /// One digest per LONG / LONG RAW column of the part (created when first used).
    longs: Vec<Sha256>,
    lob: Sha256,
}

/// Which second digest the bytes of a field also go to.
#[derive(Debug, Clone, Copy)]
enum Also {
    /// None: only the raw digest.
    Nothing,
    /// The assembled digest of the LONG column with this index (among the LONG columns of the part).
    Long(usize),
    /// The digest over the LOB values.
    Lob,
}

impl RawHasher {
    fn new() -> Self {
        Self {
            raw: Sha256::new(),
            longs: Vec::new(),
            lob: Sha256::new(),
        }
    }

    fn long_update(&mut self, i: usize, bytes: &[u8]) {
        if self.longs.len() <= i {
            self.longs.resize_with(i.saturating_add(1), Sha256::new);
        }
        if let Some(h) = self.longs.get_mut(i) {
            h.update(bytes);
        }
    }

    fn null(&mut self) {
        self.raw.update([0xff; 4]);
    }

    fn chain_start(&mut self) {
        self.raw.update([0xfd, 0xff, 0xff, 0xff]);
    }

    fn chain_end(&mut self) {
        self.raw.update([0xfc, 0xff, 0xff, 0xff]);
    }

    fn chunked_start(&mut self) {
        self.raw.update([0xfb, 0xff, 0xff, 0xff]);
    }

    fn chunked_end(&mut self) {
        self.raw.update([0xfa, 0xff, 0xff, 0xff]);
    }

    fn four_byte_start(&mut self) {
        self.raw.update([0xf9, 0xff, 0xff, 0xff]);
    }

    fn length(&mut self, n: u16) {
        self.raw.update(u32::from(n).to_le_bytes());
    }

    /// Feeds bytes to the raw digest (if `raw`) and to the second digest `also` names.
    fn update(&mut self, bytes: &[u8], raw: bool, also: Also) {
        if raw {
            self.raw.update(bytes);
        }
        match also {
            Also::Nothing => {}
            Also::Long(i) => self.long_update(i, bytes),
            Also::Lob => self.lob.update(bytes),
        }
    }

    fn finish(self, nlong: usize) -> ([u8; 32], Vec<[u8; 32]>, [u8; 32]) {
        let mut longs = self.longs;
        while longs.len() < nlong {
            longs.push(Sha256::new());
        }
        (
            self.raw.finalize().into(),
            longs.into_iter().map(|h| h.finalize().into()).collect(),
            self.lob.finalize().into(),
        )
    }
}

/// Feeds the next `n` bytes of the stream to the hasher and consumes them.
fn hash_bytes<R: Read + Seek>(
    s: &mut ByteStream<R>,
    mut n: usize,
    h: &mut RawHasher,
    raw: bool,
    also: Also,
) -> Result<()> {
    while n > 0 {
        let step = n.min(MIN_BUFFER);
        h.update(s.read_bytes(step)?, raw, also);
        n = n.saturating_sub(step);
    }
    Ok(())
}

fn framing_err(message: &'static str, offset: u64, table: u32) -> Error {
    Error::new(ErrorCode::Framing, message)
        .at(offset)
        .in_table(table)
}

/// The test of architecture 5.4 for the first byte LF after a direct-path flag, for a first column of
/// non-zero descriptor length: both tests must hold for "empty table without `ff ff`", neither for "a row
/// starts here"; one of them is ambiguous.
fn lf_is_empty_table(second: u8, first_col_max: u16) -> Option<bool> {
    let upper = second.is_ascii_uppercase();
    let as_len = u32::from(LF) | (u32::from(second) << 8);
    let exceeds = as_len > u32::from(first_col_max);
    match (upper, exceeds) {
        (true, true) => Some(true),
        (false, false) => Some(false),
        _ => None,
    }
}

/// Lines of the kinds [E2a] names, plus the five partition lines (ADR-22, ADR-21): `TABLE "`, `ENDTABLE`,
/// `METRICS`, `CREATE `, `ALTER `, `COMMENT ON `, `GRANT `, `ANALSTATS `, `CALTER TABLE`, `PARTITION "`,
/// `COMPARTITION "`, `SUBPARTITION "`, `ENDCOMPARTITION`, `ENDPARTITION`.
fn is_known_item_line(line: &[u8]) -> bool {
    const PREFIXES: [&[u8]; 10] = [
        b"TABLE \"",
        b"METRICS",
        b"CREATE ",
        b"ALTER ",
        b"COMMENT ON ",
        b"GRANT ",
        b"ANALSTATS ",
        b"PARTITION \"",
        b"COMPARTITION \"",
        b"SUBPARTITION \"",
    ];
    const EXACT: [&[u8]; 4] = [
        b"ENDTABLE",
        b"CALTER TABLE",
        b"ENDPARTITION",
        b"ENDCOMPARTITION",
    ];
    PREFIXES.iter().any(|p| line.starts_with(p)) || EXACT.contains(&line)
}

/// What the row walk needs besides the stream: list K and limits, for the decision of [E2a].
#[derive(Debug, Clone, Copy)]
pub struct Env<'a> {
    /// List K: a line of a known kind may be followed by a field that must be well-formed.
    pub intro: &'a IntroList,
    /// Longest line read by [E2a].
    pub max_line: usize,
    /// ADR-22, `strict`: with a first column of length 0, "text follows" alone is `Framing` (session 2).
    pub strict_e2a: bool,
}

/// Where the rows of a table ended (the part of [`RowsSummary`] that does not depend on counting).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowsEnd {
    /// The end marker.
    pub end_marker: EndMarker,
    /// Offset of the end marker (of `ff ff`, or of the LF for `LfOnly`).
    pub end_marker_offset: u64,
    /// Offset of the first byte after the LF that closes the rows.
    pub rows_end_offset: u64,
    /// The decision of [E2a] said "no rows".
    pub e2a_no_rows: bool,
}

/// Receives the column fields of the rows a [`RowCursor`] reads (task T-6): the format-neutral side of the
/// seam of architecture 2.3. The cursor gives the bytes lent by the I/O buffer; the sink decodes them (or
/// ignores them: a column that is not selected, a LOB locator). `column` is the 1-based file position and
/// `offset` the file offset of the first byte of the value. A sink never sees the LOB value area. An error
/// of the sink ends the step; the cursor adds nothing to it (the sink knows table, part, row and column).
pub trait RowSink {
    /// A row starts; `row` is its 1-based number within the part.
    fn row_start(&mut self, row: u64);
    /// A NULL field (`fe ff`).
    fn null(&mut self, column: u32, offset: u64) -> Result<()>;
    /// One ordinary field (a conventional LONG / LONG RAW and a LOB locator included).
    fn value(&mut self, column: u32, offset: u64, bytes: &[u8]) -> Result<()>;
    /// One piece of a direct-path LONG / LONG RAW chain [R2]; `first` for the first piece, `last` for the
    /// piece after which no other follows. The piece may be empty (ADR-15).
    fn chain_piece(
        &mut self,
        column: u32,
        offset: u64,
        piece: &[u8],
        first: bool,
        last: bool,
    ) -> Result<()>;
    /// A LOB value of the value area after the row starts [L3, L4]: `column` is the file position of the LOB
    /// column, `offset` the offset of the value's first u16, `form` its form and `total` its stored byte length
    /// (`n` of the short form, `T` of the long form; 0 for an empty value). It is called after the u16 (and
    /// the 8 bytes of `T`) passed the framing checks of [L4]; the chunks are checked as they are passed. The
    /// answer says whether the cursor lends the bytes ([`LobAction::Read`]: `lob_chunk` for each chunk, an
    /// empty value has none, then `lob_end`) or passes them by their lengths ([`LobAction::Skip`]: neither
    /// follows). A sink that records a NULL or an error for the value answers `Skip` or returns the error.
    /// The default skips every value.
    fn lob_start(
        &mut self,
        column: u32,
        offset: u64,
        form: LobForm,
        total: u64,
    ) -> Result<LobAction> {
        let _ = (column, offset, form, total);
        Ok(LobAction::Skip)
    }
    /// One chunk of the LOB value after `lob_start` answered `Read` (a short value is one chunk); `offset` is
    /// the offset of the chunk's first byte.
    fn lob_chunk(&mut self, column: u32, offset: u64, bytes: &[u8]) -> Result<()> {
        let _ = (column, offset, bytes);
        Ok(())
    }
    /// The LOB value was passed completely (only after `Read`).
    fn lob_end(&mut self, column: u32) -> Result<()> {
        let _ = column;
        Ok(())
    }
    /// The row was read completely (LOB values included).
    fn row_end(&mut self) -> Result<()>;
}

/// The answer of [`RowSink::lob_start`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LobAction {
    /// Lend the value's chunks to the sink.
    Read,
    /// Pass the value by its lengths (each chunk is still checked).
    Skip,
}

/// Reborrows an optional sink for a shorter time (the trait object's lifetime shrinks with the borrow).
fn reborrow<'a>(sink: &'a mut Option<&mut dyn RowSink>) -> Option<&'a mut dyn RowSink> {
    match sink {
        Some(k) => Some(&mut **k),
        None => None,
    }
}

/// One step of a [`RowCursor`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// A row was read; it started at this offset.
    Row(u64),
    /// The end of the rows; the stream stands behind the LF.
    End(RowsEnd),
}

/// Reads the rows of one table from a stream positioned at `rows_offset`: row by row, every value skipped
/// by its length [R1 to R4, E1 to E3, L2 to L4]. The same code walks the file at the scan and reads rows
/// later.
#[derive(Debug, Clone)]
pub struct RowCursor {
    path: PathKind,
    /// One flag per column: is it a LOB column (112, 113)?
    lob_cols: Vec<bool>,
    /// One flag per column: is it a LONG / LONG RAW column (8, 24)?
    long_cols: Vec<bool>,
    long: Vec<LongStats>,
    lob_zero_total: Option<LobZeroTotal>,
    lob_table: bool,
    first_max: u16,
    table: u32,
    rows: u64,
    nulls: u64,
    lob: LobSummary,
    /// The same counts per column (index = column - 1; only LOB columns get values).
    lob_per_column: Vec<LobSummary>,
    /// The columns whose field in the current row is a locator, in column order.
    row_locators: Vec<u32>,
    detail: Option<LobDetail>,
}

impl RowCursor {
    /// A cursor for the table with this header; `table` is the 1-based ordinal, for error context.
    pub fn new(header: &TableHeader, table: u32) -> Self {
        Self::from_columns(
            header.path,
            header.columns.iter().map(|c| (c.type_code, c.max_len)),
            table,
        )
    }

    /// A cursor from the facts of a catalog entry: path and (type code, maximum length) per column.
    pub fn from_columns(
        path: PathKind,
        columns: impl IntoIterator<Item = (u16, u16)>,
        table: u32,
    ) -> Self {
        let mut lob_cols = Vec::new();
        let mut long_cols = Vec::new();
        let mut long = Vec::new();
        let mut first_max = 0;
        for (i, (code, max)) in columns.into_iter().enumerate() {
            if i == 0 {
                first_max = max;
            }
            lob_cols.push(is_lob_type(code));
            long_cols.push(is_long_type(code));
            if is_long_type(code) {
                long.push(LongStats {
                    column: u32::try_from(i.saturating_add(1)).unwrap_or(u32::MAX),
                    ..LongStats::default()
                });
            }
        }
        let lob_table = lob_cols.iter().any(|&l| l);
        Self {
            path,
            lob_cols,
            long_cols,
            long,
            lob_zero_total: None,
            lob_table,
            first_max,
            table,
            rows: 0,
            nulls: 0,
            lob: LobSummary::default(),
            lob_per_column: Vec::new(),
            row_locators: Vec::new(),
            detail: None,
        }
    }

    /// Keeps k of every row and every LOB value (memory grows with the table).
    #[must_use]
    pub fn recording_lob(mut self) -> Self {
        self.detail = Some(LobDetail::default());
        self
    }

    /// Rows read so far.
    pub fn rows(&self) -> u64 {
        self.rows
    }

    /// Column fields read so far (rows times columns).
    pub fn fields(&self) -> u64 {
        self.rows
            .saturating_mul(u64::try_from(self.lob_cols.len()).unwrap_or(u64::MAX))
    }

    /// NULL fields read so far.
    pub fn nulls(&self) -> u64 {
        self.nulls
    }

    /// The LOB value counts so far.
    pub fn lob(&self) -> LobSummary {
        self.lob
    }

    /// The LOB value counts per LOB column so far, in column order.
    pub fn lob_columns(&self) -> Vec<LobColumn> {
        self.lob_cols
            .iter()
            .enumerate()
            .filter(|(_, is_lob)| **is_lob)
            .map(|(i, _)| LobColumn {
                column: u32::try_from(i.saturating_add(1)).unwrap_or(u32::MAX),
                stats: self.lob_per_column.get(i).copied().unwrap_or_default(),
            })
            .collect()
    }

    /// The LONG / LONG RAW statistics so far, one entry per such column.
    pub fn long_stats(&self) -> &[LongStats] {
        &self.long
    }

    /// The first long LOB value of total 0 so far.
    pub fn lob_zero_total(&self) -> Option<LobZeroTotal> {
        self.lob_zero_total
    }

    /// Takes the recorded detail (None unless [`Self::recording_lob`] was called).
    pub fn take_detail(&mut self) -> Option<LobDetail> {
        self.detail.take()
    }

    /// The decision for an LF at the first byte of a direct-path table: no rows (`Some(true)`, with the
    /// [E2a] flag), a row (`Some(false)`), or an error for the ambiguous case [E3, E2a, ADR-22].
    fn lf_means_no_rows<R: Read + Seek>(
        &self,
        s: &mut ByteStream<R>,
        second: u8,
        at: u64,
        env: &Env<'_>,
    ) -> Result<(bool, bool)> {
        if self.first_max != 0 {
            return match lf_is_empty_table(second, self.first_max) {
                Some(b) => Ok((b, false)),
                None => Err(framing_err(
                    "ambiguous empty table on the direct path",
                    at,
                    self.table,
                )),
            };
        }
        // Column 1 has descriptor length 0 (LONG, LONG RAW): test (b) says nothing [E2a].
        if !second.is_ascii_uppercase() {
            return Ok((false, false));
        }
        if env.strict_e2a {
            return Err(framing_err(
                "ambiguous empty table on the direct path (first column of length 0)",
                at,
                self.table,
            ));
        }
        Ok((known_item_follows(s, at, env)?, true))
    }

    /// Reads the next row, or the end of the rows. After `End` the cursor must not be used again.
    pub fn step<R: Read + Seek>(
        &mut self,
        s: &mut ByteStream<R>,
        hasher: Option<&mut RawHasher>,
        env: &Env<'_>,
    ) -> Result<Step> {
        self.step_with(s, hasher, None, env)
    }

    /// As [`Self::step`], and gives every column field of the row to `sink`. With a sink the bytes of the
    /// fields are not hashed (the two are for different jobs: the scan hashes, the reader decodes).
    pub fn step_with<R: Read + Seek>(
        &mut self,
        s: &mut ByteStream<R>,
        mut hasher: Option<&mut RawHasher>,
        mut sink: Option<&mut dyn RowSink>,
        env: &Env<'_>,
    ) -> Result<Step> {
        let table = self.table;
        let at = s.offset();
        let two: [u8; 2] = {
            let p = s.peek(2)?;
            match <[u8; 2]>::try_from(p) {
                Ok(a) => a,
                Err(_) => {
                    return Err(Error::new(
                        ErrorCode::Truncated,
                        "file ends where a row or the end of rows should stand",
                    )
                    .at(at)
                    .in_table(table)
                    .in_row(self.rows.saturating_add(1)));
                }
            }
        };
        if two == [0xff, 0xff] {
            s.skip(2)?;
            expect_lf(s, table)?;
            return Ok(Step::End(RowsEnd {
                end_marker: EndMarker::FfFf,
                end_marker_offset: at,
                rows_end_offset: s.offset(),
                e2a_no_rows: false,
            }));
        }
        if self.rows == 0 && self.path == PathKind::Direct && two[0] == LF {
            let (empty, e2a) = self.lf_means_no_rows(s, two[1], at, env)?;
            if empty {
                s.skip(1)?;
                return Ok(Step::End(RowsEnd {
                    end_marker: EndMarker::LfOnly,
                    end_marker_offset: at,
                    rows_end_offset: s.offset(),
                    e2a_no_rows: e2a,
                }));
            }
        }
        self.rows = self.rows.saturating_add(1);
        let rows = self.rows;
        if let Some(k) = reborrow(&mut sink) {
            k.row_start(rows);
        }
        self.row_locators.clear();
        let mut long_index = 0usize;
        for (index, (&is_lob, &is_long)) in self.lob_cols.iter().zip(&self.long_cols).enumerate() {
            let conventional_long = is_long && self.path == PathKind::Conventional;
            let column = u32::try_from(index.saturating_add(1)).unwrap_or(u32::MAX);
            let field_at = s.offset();
            let len = s.read_u16_le()?;
            let stats_at = long_index;
            if is_long {
                long_index = long_index.saturating_add(1);
            }
            match len {
                0xfffe => {
                    // NULL [R3]
                    self.nulls = self.nulls.saturating_add(1);
                    if let Some(k) = reborrow(&mut sink) {
                        k.null(column, field_at)?;
                    }
                    if let Some(h) = hasher.as_mut() {
                        h.null();
                        if is_long {
                            h.update(&[0xff; 8], false, Also::Long(stats_at));
                        }
                    }
                }
                // [R8]: 0 is an error in every column; 0xfff0 to 0xfffd are errors in every column except a
                // conventional LONG / LONG RAW one, where the arms below read them (ADR-99 item 1);
                // `ff ff` is the end marker, never inside a row. This holds for the first u16 of a chain too.
                0 | 0xfff0..=0xfffd if !conventional_long || len == 0 => {
                    return Err(framing_err(
                        "column field with a length that never occurs (0 or 0xfff0 to 0xfffd)",
                        field_at,
                        table,
                    )
                    .in_row(rows)
                    .in_column(column)
                    .with_length(u64::from(len)));
                }
                0xffff => {
                    return Err(framing_err("ff ff inside a row", field_at, table)
                        .in_row(rows)
                        .in_column(column));
                }
                n if is_long && self.path == PathKind::Direct => {
                    // [R2, Y9, ADR-15]: a chain of pieces
                    let chain = read_chain(
                        s,
                        hasher.as_deref_mut(),
                        reborrow(&mut sink),
                        stats_at,
                        n,
                        field_at,
                        table,
                        rows,
                        column,
                    )?;
                    if let Some(st) = self.long.get_mut(stats_at) {
                        st.count_value(chain.total);
                        if chain.pieces > 1 {
                            st.chained = st.chained.saturating_add(1);
                        }
                        if let Some(at) = chain.zero_piece_at {
                            st.zero_pieces = st.zero_pieces.saturating_add(chain.zero_pieces);
                            if st.first_zero_piece.is_none() {
                                st.first_zero_piece = Some((rows, at));
                            }
                        }
                    }
                }
                0xfffb | 0xfffc if conventional_long => {
                    // [R8] v3.9 ("stop"): in the marker band, not in the fixtures; the readings consume different
                    // byte counts, so nothing after it can be located [ADR-12, ADR-99 item 1]
                    return Err(Error::new(
                        ErrorCode::UnsupportedLayout,
                        "conventional LONG or LONG RAW field starting 0xfffb or 0xfffc, not in the fixtures [R8]",
                    )
                    .at(field_at)
                    .in_table(table)
                    .in_row(rows)
                    .in_column(column)
                    .with_length(u64::from(len)));
                }
                0xfffa if conventional_long => {
                    // the 4-byte length form (b) [R2 v3.9, ADR-99]
                    let n = read_four_byte(
                        s,
                        hasher.as_deref_mut(),
                        reborrow(&mut sink),
                        stats_at,
                        field_at,
                        table,
                        rows,
                        column,
                    )?;
                    if let Some(st) = self.long.get_mut(stats_at) {
                        st.count_value(n);
                    }
                }
                0xfffd if conventional_long => {
                    // the chunked form [R2, ADR-95 item 2]
                    let ch = read_chunked(
                        s,
                        hasher.as_deref_mut(),
                        reborrow(&mut sink),
                        stats_at,
                        field_at,
                        table,
                        rows,
                        column,
                    )?;
                    if let Some(st) = self.long.get_mut(stats_at) {
                        st.count_value(ch.total);
                        st.chunked = st.chunked.saturating_add(1);
                        if ch.total < CHUNKED_MIN_TOTAL {
                            st.chunked_short = st.chunked_short.saturating_add(1);
                            if st.first_chunked_short.is_none() {
                                st.first_chunked_short = Some((rows, ch.value_at));
                            }
                        }
                    }
                }
                n => {
                    if is_lob {
                        self.row_locators.push(column);
                    }
                    if is_long {
                        if let Some(st) = self.long.get_mut(stats_at) {
                            st.count_value(u64::from(n));
                            if conventional_long && (0xfff2..=0xfff4).contains(&n) {
                                // one field, but no fixture has the size (R-052, ADR-101 item 1)
                                st.fff2_to_fff4 = st.fff2_to_fff4.saturating_add(1);
                                if st.first_fff2_to_fff4.is_none() {
                                    st.first_fff2_to_fff4 = Some((rows, field_at));
                                }
                            }
                        }
                    }
                    match (reborrow(&mut sink), hasher.as_mut()) {
                        (Some(k), _) => {
                            let at = s.offset();
                            let bytes = s.read_bytes(usize::from(n))?;
                            k.value(column, at, bytes)?;
                        }
                        (None, Some(h)) => {
                            h.length(n);
                            let also = if is_long {
                                h.update(&u64::from(n).to_le_bytes(), false, Also::Long(stats_at));
                                Also::Long(stats_at)
                            } else {
                                Also::Nothing
                            };
                            hash_bytes(s, usize::from(n), h, true, also)?;
                        }
                        (None, None) => s.skip(u64::from(n))?,
                    }
                }
            }
        }
        if self.path == PathKind::Conventional {
            let k_at = s.offset();
            let k = s.read_u16_le()?;
            if let Some(d) = self.detail.as_mut() {
                if self.lob_table {
                    d.ks.push((k_at, k));
                }
            }
            if !self.lob_table {
                if k != 0 {
                    return Err(
                        framing_err("00 00 does not follow the row", k_at, table).in_row(rows)
                    );
                }
            } else {
                let expected = u64::try_from(self.row_locators.len()).unwrap_or(u64::MAX);
                if u64::from(k) != expected {
                    return Err(framing_err(
                        "k differs from the number of LOB columns of the row that are not NULL",
                        k_at,
                        table,
                    )
                    .in_row(rows)
                    .with_length(u64::from(k)));
                }
                for i in 0..usize::from(k) {
                    let column = self.row_locators.get(i).copied().unwrap_or(0);
                    self.lob_value(s, hasher.as_deref_mut(), reborrow(&mut sink), rows, column)?;
                }
            }
        }
        if let Some(k) = sink {
            k.row_end()?;
        }
        Ok(Step::Row(at))
    }

    /// One LOB value of the value area after a row [L4]: short, empty or long. With a sink the value is
    /// offered to it (`lob_start`) and, if it answers `Read`, lent chunk by chunk; otherwise it is hashed or
    /// passed by its lengths.
    fn lob_value<R: Read + Seek>(
        &mut self,
        s: &mut ByteStream<R>,
        mut hasher: Option<&mut RawHasher>,
        mut sink: Option<&mut dyn RowSink>,
        row: u64,
        column: u32,
    ) -> Result<()> {
        let table = self.table;
        let at = s.offset();
        let bad = |message: &'static str, offset: u64| {
            framing_err(message, offset, table)
                .in_row(row)
                .in_column(column)
        };
        let h = s.read_u16_le()?;
        let (form, total, chunks) = match h {
            0xfffc => {
                let total = s.read_u64_le()?;
                if let Some(h) = hasher.as_deref_mut() {
                    h.update(&total.to_le_bytes(), false, Also::Lob);
                }
                let mut reading = false;
                if let Some(k) = reborrow(&mut sink) {
                    reading = k.lob_start(column, at, LobForm::Long, total)? == LobAction::Read;
                }
                let mut sum = 0u64;
                let mut chunks = Vec::new();
                let record = self.detail.is_some();
                while sum < total {
                    let c_at = s.offset();
                    let c = s.read_u16_le()?;
                    if c == 0 || c >= 0xfff0 {
                        return Err(bad("LOB chunk length is 0 or from 0xfff0", c_at)
                            .with_length(u64::from(c)));
                    }
                    let next = sum.saturating_add(u64::from(c));
                    if next > total {
                        return Err(
                            bad("LOB chunks pass the total length", c_at).with_length(total)
                        );
                    }
                    match (reborrow(&mut sink), hasher.as_deref_mut()) {
                        (Some(k), _) if reading => {
                            let b_at = s.offset();
                            let bytes = s.read_bytes(usize::from(c))?;
                            k.lob_chunk(column, b_at, bytes)?;
                        }
                        (Some(_), _) => s.skip(u64::from(c))?,
                        (None, Some(h)) => hash_bytes(s, usize::from(c), h, false, Also::Lob)?,
                        (None, None) => s.skip(u64::from(c))?,
                    }
                    sum = next;
                    if record {
                        chunks.push(c);
                    }
                }
                if reading {
                    if let Some(k) = reborrow(&mut sink) {
                        k.lob_end(column)?;
                    }
                }
                if total == 0 && self.lob_zero_total.is_none() {
                    self.lob_zero_total = Some(LobZeroTotal {
                        offset: at,
                        row,
                        column,
                    });
                }
                (LobForm::Long, total, chunks)
            }
            // `fe ff` (NULL), `ff ff` and the other values from 0xfff0 never stand here [L4]
            0xfff0..=0xfffb | 0xfffd..=0xffff => {
                return Err(bad(
                    "LOB value area holds a u16 that is neither a length nor fc ff",
                    at,
                )
                .with_length(u64::from(h)));
            }
            n => {
                let mut reading = false;
                if let Some(k) = reborrow(&mut sink) {
                    reading =
                        k.lob_start(column, at, LobForm::Short, u64::from(n))? == LobAction::Read;
                }
                match (reborrow(&mut sink), hasher) {
                    (Some(k), _) if reading => {
                        if n > 0 {
                            let b_at = s.offset();
                            let bytes = s.read_bytes(usize::from(n))?;
                            k.lob_chunk(column, b_at, bytes)?;
                        }
                        k.lob_end(column)?;
                    }
                    (Some(_), _) => s.skip(u64::from(n))?,
                    (None, Some(h)) => {
                        h.update(&u64::from(n).to_le_bytes(), false, Also::Lob);
                        hash_bytes(s, usize::from(n), h, false, Also::Lob)?;
                    }
                    (None, None) => s.skip(u64::from(n))?,
                }
                (LobForm::Short, u64::from(n), Vec::new())
            }
        };
        let index = usize::try_from(column.saturating_sub(1)).unwrap_or(usize::MAX);
        if self.lob_per_column.len() < self.lob_cols.len() {
            self.lob_per_column
                .resize_with(self.lob_cols.len(), LobSummary::default);
        }
        for l in std::iter::once(&mut self.lob).chain(self.lob_per_column.get_mut(index)) {
            l.values = l.values.saturating_add(1);
            match form {
                LobForm::Short => {
                    l.short = l.short.saturating_add(1);
                    if total == 0 {
                        l.empty = l.empty.saturating_add(1);
                    }
                }
                LobForm::Long => l.long = l.long.saturating_add(1),
            }
            l.stored_bytes = l.stored_bytes.saturating_add(total);
            l.largest = l.largest.max(total);
        }
        if let Some(d) = self.detail.as_mut() {
            d.values.push(LobValueRecord {
                row,
                column,
                offset: at,
                form,
                total,
                chunks,
            });
        }
        Ok(())
    }
}

/// A chunked conventional LONG / LONG RAW value of fewer bytes than this is a combination no fixture shows (ADR-99
/// item 2: the smallest chunked size seen is 65,534, `long4_exp_<v>` row 6).
const CHUNKED_MIN_TOTAL: u64 = 65_534;

/// What one chunked conventional LONG value was.
struct Chunked {
    /// Sum of the chunk lengths.
    total: u64,
    /// Offset where the value's data starts: behind the first chunk's u16.
    value_at: u64,
}

/// A conventional LONG value in the chunked form [R2 v3.7, ADR-95 item 2]: the `fd ff` at `field_at` was
/// read; then `u16 c` + c bytes, ended by a chunk `00 00`. A chunk length from 0xfff0 is `Framing`. The sink
/// gets the chunks as pieces of a chain, then one empty last piece; nothing is held here.
#[allow(clippy::too_many_arguments)]
fn read_chunked<R: Read + Seek>(
    s: &mut ByteStream<R>,
    mut hasher: Option<&mut RawHasher>,
    mut sink: Option<&mut dyn RowSink>,
    long_index: usize,
    field_at: u64,
    table: u32,
    row: u64,
    column: u32,
) -> Result<Chunked> {
    let mut out = Chunked {
        total: 0,
        value_at: 0,
    };
    let mut chunks = 0u64;
    if let Some(h) = hasher.as_deref_mut() {
        h.chunked_start();
    }
    loop {
        let c_at = s.offset();
        let c = s.read_u16_le()?;
        if chunks == 0 {
            out.value_at = s.offset();
        }
        if c == 0 {
            if let Some(k) = reborrow(&mut sink) {
                k.chain_piece(column, out.value_at, &[], chunks == 0, true)?;
            }
            break;
        }
        if c >= 0xfff0 {
            return Err(framing_err(
                "conventional LONG or LONG RAW chunk length from 0xfff0",
                c_at,
                table,
            )
            .in_row(row)
            .in_column(column)
            .with_length(u64::from(c)));
        }
        match (reborrow(&mut sink), hasher.as_deref_mut()) {
            (Some(k), _) => {
                let b_at = s.offset();
                let bytes = s.read_bytes(usize::from(c))?;
                k.chain_piece(column, b_at, bytes, chunks == 0, false)?;
            }
            (None, Some(h)) => {
                h.length(c);
                hash_bytes(s, usize::from(c), h, true, Also::Nothing)?;
            }
            (None, None) => s.skip(u64::from(c))?,
        }
        out.total = out.total.saturating_add(u64::from(c));
        chunks = chunks.saturating_add(1);
    }
    if let Some(h) = hasher {
        h.chunked_end();
        // the assembled digest starts with the total, known only now: read the chunks again
        let end = s.offset();
        h.update(&out.total.to_le_bytes(), false, Also::Long(long_index));
        s.seek_to(field_at.saturating_add(2))?;
        loop {
            let c = s.read_u16_le()?;
            if c == 0 {
                break;
            }
            hash_bytes(s, usize::from(c), h, false, Also::Long(long_index))?;
        }
        if s.offset() != end {
            return Err(framing_err(
                "conventional LONG chunks read twice with different lengths",
                end,
                table,
            )
            .in_row(row)
            .in_column(column));
        }
    }
    Ok(out)
}

/// A conventional LONG / LONG RAW value in the 4-byte length form (b) [R2 v3.9, ADR-99 items 1, 4]: the `fa ff` at
/// `field_at` was read; then `u16 n` (0xfffa to 0xfffd, else `Framing` at field + 2), `u16 z` (must be 0, else
/// `Framing` at field + 4) and n value bytes. The sink gets them as one plain value at field + 6 (the first data
/// byte). Returns n.
#[allow(clippy::too_many_arguments)]
fn read_four_byte<R: Read + Seek>(
    s: &mut ByteStream<R>,
    hasher: Option<&mut RawHasher>,
    sink: Option<&mut dyn RowSink>,
    long_index: usize,
    field_at: u64,
    table: u32,
    row: u64,
    column: u32,
) -> Result<u64> {
    let n_at = s.offset();
    let n = s.read_u16_le()?;
    if !(0xfffa..=0xfffd).contains(&n) {
        return Err(framing_err(
            "conventional LONG or LONG RAW 4-byte length outside 65,530 to 65,533",
            n_at,
            table,
        )
        .in_row(row)
        .in_column(column)
        .with_length(u64::from(n)));
    }
    let z_at = s.offset();
    let z = s.read_u16_le()?;
    if z != 0 {
        return Err(framing_err(
            "conventional LONG or LONG RAW 4-byte length whose upper two bytes are not 00 00",
            z_at,
            table,
        )
        .in_row(row)
        .in_column(column)
        .with_length(u64::from(z)));
    }
    debug_assert_eq!(n_at, field_at.saturating_add(2));
    match (sink, hasher) {
        (Some(k), _) => {
            let at = s.offset();
            let bytes = s.read_bytes(usize::from(n))?;
            k.value(column, at, bytes)?;
        }
        (None, Some(h)) => {
            h.four_byte_start();
            h.raw.update(u32::from(n).to_le_bytes());
            h.update(&u64::from(n).to_le_bytes(), false, Also::Long(long_index));
            hash_bytes(s, usize::from(n), h, true, Also::Long(long_index))?;
        }
        (None, None) => s.skip(u64::from(n))?,
    }
    Ok(u64::from(n))
}

/// What one piece chain was.
struct Chain {
    /// Sum of the piece lengths.
    total: u64,
    /// Number of pieces.
    pieces: u64,
    /// Offset of the u16 of the first piece of length 0.
    zero_piece_at: Option<u64>,
    /// Number of pieces of length 0.
    zero_pieces: u64,
}

/// A direct-path LONG / LONG RAW value [R2, Y9, ADR-15]: `first` is the u16 already read at `field_at`
/// (neither a NULL nor a value of [R8]); each u16 is a piece length in its low 15 bits and, with bit
/// 0x8000, "another piece follows". A later u16 from 0xfff0 is `Framing`; a piece of length 0 is read by
/// the formula and reported in `zero_piece_at`.
#[allow(clippy::too_many_arguments)]
fn read_chain<R: Read + Seek>(
    s: &mut ByteStream<R>,
    mut hasher: Option<&mut RawHasher>,
    mut sink: Option<&mut dyn RowSink>,
    long_index: usize,
    first: u16,
    field_at: u64,
    table: u32,
    row: u64,
    column: u32,
) -> Result<Chain> {
    let mut n = first;
    let mut at = field_at;
    let mut chain = Chain {
        total: 0,
        pieces: 0,
        zero_piece_at: None,
        zero_pieces: 0,
    };
    if let Some(h) = hasher.as_deref_mut() {
        h.chain_start();
    }
    loop {
        let len = n & 0x7fff;
        if len == 0 {
            chain.zero_pieces = chain.zero_pieces.saturating_add(1);
            if chain.zero_piece_at.is_none() {
                chain.zero_piece_at = Some(at);
            }
        }
        match (reborrow(&mut sink), hasher.as_deref_mut()) {
            (Some(k), _) => {
                let piece_at = s.offset();
                let bytes = s.read_bytes(usize::from(len))?;
                k.chain_piece(column, piece_at, bytes, chain.pieces == 0, n & 0x8000 == 0)?;
            }
            (None, Some(h)) => {
                h.length(len);
                hash_bytes(s, usize::from(len), h, true, Also::Nothing)?;
            }
            (None, None) => s.skip(u64::from(len))?,
        }
        chain.total = chain.total.saturating_add(u64::from(len));
        chain.pieces = chain.pieces.saturating_add(1);
        if n & 0x8000 == 0 {
            break;
        }
        at = s.offset();
        n = s.read_u16_le()?;
        if n >= 0xfff0 {
            return Err(framing_err(
                "direct-path LONG chain: a piece length from 0xfff0",
                at,
                table,
            )
            .in_row(row)
            .in_column(column)
            .with_length(u64::from(n)));
        }
    }
    if let Some(h) = hasher {
        h.chain_end();
        // the assembled digest starts with the total, which is known only now: read the pieces again
        let end = s.offset();
        h.update(&chain.total.to_le_bytes(), false, Also::Long(long_index));
        s.seek_to(field_at.saturating_add(2))?;
        let mut n = first;
        loop {
            hash_bytes(s, usize::from(n & 0x7fff), h, false, Also::Long(long_index))?;
            if n & 0x8000 == 0 {
                break;
            }
            n = s.read_u16_le()?;
        }
        if s.offset() != end {
            return Err(framing_err(
                "direct-path LONG chain read twice with different lengths",
                end,
                table,
            )
            .in_row(row)
            .in_column(column));
        }
    }
    Ok(chain)
}

/// [E2a]: the stream stands at the LF at `at` (first byte of a direct-path table whose column 1 has
/// descriptor length 0). True if the line from `at + 1` to the next LF is a complete item of a known kind
/// whose field, if list K says one follows, is well-formed. The stream is put back at `at`.
fn known_item_follows<R: Read + Seek>(
    s: &mut ByteStream<R>,
    at: u64,
    env: &Env<'_>,
) -> Result<bool> {
    s.skip(1)?;
    let mut line = Vec::new();
    let found = match s.read_line(&mut line, env.max_line) {
        Ok(()) => {
            if !is_known_item_line(&line) {
                false
            } else if let Some(k) = env.intro.find(&line) {
                let forms = env
                    .intro
                    .entries()
                    .get(k)
                    .map(|e| e.forms)
                    .unwrap_or_default();
                match framing::read_listed_field(s, forms) {
                    Ok(_) => true,
                    Err(e) if e.code == ErrorCode::Io => return Err(e),
                    Err(_) => false,
                }
            } else {
                true
            }
        }
        Err(e) if e.code == ErrorCode::Io => return Err(e),
        Err(_) => false,
    };
    s.seek_to(at)?;
    Ok(found)
}

/// Walks the rows of one table, from `header.rows_offset` to behind the LF that follows the end marker.
pub fn walk_rows<R: Read + Seek>(
    s: &mut ByteStream<R>,
    header: &TableHeader,
    table: u32,
    hash: bool,
    record_lob: bool,
    env: &Env<'_>,
) -> Result<RowsSummary> {
    let mut hasher = hash.then(RawHasher::new);
    let has_lob = header.has_lob();
    let mut cursor = RowCursor::new(header, table);
    if record_lob {
        cursor = cursor.recording_lob();
    }
    loop {
        if let Step::End(end) = cursor.step(s, hasher.as_mut(), env)? {
            let nlong = cursor.long_stats().len();
            let (raw_sha256, long_sha256, lob_sha256) = match hasher.map(|h| h.finish(nlong)) {
                Some((raw, longs, lob)) => (Some(raw), longs, has_lob.then_some(lob)),
                None => (None, Vec::new(), None),
            };
            return Ok(RowsSummary {
                rows: cursor.rows(),
                fields: cursor.fields(),
                end_marker: end.end_marker,
                end_marker_offset: end.end_marker_offset,
                rows_end_offset: end.rows_end_offset,
                nulls: cursor.nulls(),
                raw_sha256,
                lob: cursor.lob(),
                lob_detail: cursor.take_detail(),
                e2a_no_rows: end.e2a_no_rows,
                long: cursor.long_stats().to_vec(),
                lob_zero_total: cursor.lob_zero_total(),
                lob_columns: cursor.lob_columns(),
                lob_sha256,
                long_sha256,
            });
        }
    }
}

fn expect_lf<R: Read + Seek>(s: &mut ByteStream<R>, table: u32) -> Result<()> {
    let at = s.offset();
    if s.read_u8()? == LF {
        Ok(())
    } else {
        Err(framing_err(
            "end of rows is not followed by a line feed",
            at,
            table,
        ))
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

    /// The pieces a sink got from the cursor, as (column, offset, bytes, first, last).
    #[derive(Default)]
    struct Recorder {
        pieces: Vec<(u32, u64, Vec<u8>, bool, bool)>,
        values: Vec<(u32, u64, usize)>,
    }

    impl RowSink for Recorder {
        fn row_start(&mut self, _row: u64) {}
        fn null(&mut self, _column: u32, _offset: u64) -> Result<()> {
            Ok(())
        }
        fn value(&mut self, column: u32, offset: u64, bytes: &[u8]) -> Result<()> {
            self.values.push((column, offset, bytes.len()));
            Ok(())
        }
        fn chain_piece(
            &mut self,
            column: u32,
            offset: u64,
            piece: &[u8],
            first: bool,
            last: bool,
        ) -> Result<()> {
            self.pieces
                .push((column, offset, piece.to_vec(), first, last));
            Ok(())
        }
        fn row_end(&mut self) -> Result<()> {
            Ok(())
        }
    }

    /// One conventional row `(ID NUMBER, L)` with the given L bytes after the ID `01 00 c1`, the row end `00 00`, the
    /// end marker `ff ff` and an LF; the LONG column has type code `code` (8 or 24).
    fn long_row(code: u16, l: &[u8]) -> (RowCursor, Vec<u8>) {
        let mut data = vec![0x02, 0x00, 0xc1, 0x02];
        data.extend_from_slice(l);
        data.extend_from_slice(&[0x00, 0x00, 0xff, 0xff, LF]);
        (
            RowCursor::from_columns(PathKind::Conventional, [(2, 22), (code, 0)], 1),
            data,
        )
    }

    fn step_row(c: &mut RowCursor, data: &[u8], sink: Option<&mut dyn RowSink>) -> Result<Step> {
        let intro = IntroList::builtin().unwrap();
        let env = Env {
            intro: &intro,
            max_line: 100,
            strict_e2a: false,
        };
        let mut s = stream(data);
        c.step_with(&mut s, None, sink, &env)
    }

    /// ADR-95 item 2, 6, 8: `fd ff`, chunks, `00 00`: the sink gets one piece per chunk (the value's offset is the first
    /// data byte) and an empty last piece; the cursor counts one chunked value of 5 bytes (below 65,534: counted as
    /// short, with the offset of the first data byte); `fd ff 00 00` is an empty chunked value.
    #[test]
    fn a_chunked_conventional_long_reaches_the_sink_as_pieces() {
        let l = [0xfd, 0xff, 3, 0, b'a', b'b', b'c', 2, 0, b'd', b'e', 0, 0];
        let (mut c, data) = long_row(8, &l);
        let mut r = Recorder::default();
        assert!(matches!(
            step_row(&mut c, &data, Some(&mut r)),
            Ok(Step::Row(0))
        ));
        // ID at 0, L's `fd ff` at 4: the first chunk u16 at 6, its data at 8, the second chunk u16 at 11, data at 13
        assert_eq!(
            r.pieces,
            [
                (2, 8, b"abc".to_vec(), true, false),
                (2, 13, b"de".to_vec(), false, false),
                // the empty last piece carries the value's offset, like every piece after the first
                (2, 8, Vec::new(), false, true),
            ]
        );
        let st = &c.long_stats()[0];
        assert_eq!(
            (
                st.values,
                st.chunked,
                st.chained,
                st.largest,
                st.chunked_short
            ),
            (1, 1, 0, 5, 1)
        );
        assert_eq!(st.first_chunked_short, Some((1, 8)));
        // an empty value: the only piece is first and last
        let (mut c, data) = long_row(8, &[0xfd, 0xff, 0, 0]);
        let mut r = Recorder::default();
        step_row(&mut c, &data, Some(&mut r)).unwrap();
        assert_eq!(r.pieces, [(2, 8, Vec::new(), true, true)]);
        assert_eq!(c.long_stats()[0].largest, 0);
        // the scan (no sink, no hasher) passes the chunks by length and reads the end marker next
        let (mut c, data) = long_row(8, &l);
        let intro = IntroList::builtin().unwrap();
        let env = Env {
            intro: &intro,
            max_line: 100,
            strict_e2a: false,
        };
        let mut s = stream(&data);
        assert!(matches!(c.step(&mut s, None, &env), Ok(Step::Row(0))));
        assert!(matches!(c.step(&mut s, None, &env), Ok(Step::End(_))));
    }

    /// ADR-95 item 1: a plain length from 0x8000 to 0xfff0 is one field of that length in a LONG column (the sink
    /// gets it whole through `value`); `fe ff` stays NULL.
    #[test]
    fn a_conventional_long_length_with_bit_15_set_is_one_field() {
        for n in [0x8000usize, 0xffef, 0xfff0] {
            let mut l = (n as u16).to_le_bytes().to_vec();
            l.extend(std::iter::repeat_n(b'y', n));
            let (mut c, data) = long_row(8, &l);
            let mut r = Recorder::default();
            // the sink path reads the field whole: the buffer must hold it
            let intro = IntroList::builtin().unwrap();
            let env = Env {
                intro: &intro,
                max_line: 100,
                strict_e2a: false,
            };
            let mut s = ByteStream::new(std::io::Cursor::new(data.clone()), 1 << 17).unwrap();
            assert!(
                matches!(
                    c.step_with(&mut s, None, Some(&mut r), &env),
                    Ok(Step::Row(0))
                ),
                "{n:#x}"
            );
            assert_eq!(
                r.values,
                [(1, 2, 2), (2, 6, n)].map(|(a, b, c)| (a, b, c)),
                "{n:#x}"
            );
            let st = &c.long_stats()[0];
            assert_eq!(
                (st.values, st.chained, st.chunked, st.largest),
                (1, 0, 0, n as u64)
            );
        }
    }

    fn env() -> (IntroList, usize) {
        (IntroList::builtin().unwrap(), 100)
    }

    /// Steps one row of `(ID, L)` over `l` (a LONG or LONG RAW column by `code`) with a scan stream (no sink).
    fn run(code: u16, l: &[u8]) -> (RowCursor, Result<Step>) {
        let (intro, max_line) = env();
        let env = Env {
            intro: &intro,
            max_line,
            strict_e2a: false,
        };
        let (mut c, data) = long_row(code, l);
        let mut s = stream(&data);
        let r = c.step(&mut s, None, &env);
        (c, r)
    }

    /// A field of u bytes `u16 u` + u bytes.
    fn plain(u: usize) -> Vec<u8> {
        let mut v = (u as u16).to_le_bytes().to_vec();
        v.extend(std::iter::repeat_n(b'y', u));
        v
    }

    /// ADR-99 item 1, the table, both type codes: u = 0xfff1, 0xfff5 and 0xfff9 are one field (the cursor counts it, no
    /// warning field); 0xfff2 and 0xfff4 are one field and count in `fff2_to_fff4` with the field's offset (4: after the
    /// ID); 0xfffa is form (b); and only 0xfff2 to 0xfff4 count in the band 65,521 to 65,534 beside 65,521 itself.
    #[test]
    fn the_one_field_sizes_of_the_table_read_for_both_types() {
        for code in [8u16, 24] {
            for (u, warn) in [
                (0xfff1usize, false),
                (0xfff2, true),
                (0xfff4, true),
                (0xfff5, false),
                (0xfff9, false),
            ] {
                let (c, r) = run(code, &plain(u));
                assert!(matches!(r, Ok(Step::Row(0))), "{code} {u:#x}: {r:?}");
                let st = &c.long_stats()[0];
                assert_eq!(
                    (st.values, st.largest, st.sizes_65521_to_65534),
                    (1, u as u64, 1)
                );
                assert_eq!(
                    (st.fff2_to_fff4, st.first_fff2_to_fff4),
                    if warn { (1, Some((1, 4))) } else { (0, None) },
                    "{code} {u:#x}"
                );
            }
        }
    }

    /// ADR-99 item 1: `fa ff`, `u16 n`, `u16 z`, n bytes: n = 0xfffa and 0xfffd read as one value whose offset is
    /// field + 6, the sink gets a plain value (`value`), the stats count it once in the band; n = 0xfff9 and 0xfffe
    /// are `Framing` at field + 2 with `length` n; z = 1 is `Framing` at field + 4 with `length` 1; both texts exact.
    #[test]
    fn the_four_byte_length_form_reads_and_is_checked() {
        for code in [8u16, 24] {
            for n in [0xfffausize, 0xfffd] {
                let mut l = vec![0xfa, 0xff];
                l.extend_from_slice(&(n as u16).to_le_bytes());
                l.extend_from_slice(&[0, 0]);
                l.extend(std::iter::repeat_n(b'y', n));
                let (mut c, data) = long_row(code, &l);
                let (intro, max_line) = env();
                let env = Env {
                    intro: &intro,
                    max_line,
                    strict_e2a: false,
                };
                let mut r = Recorder::default();
                let mut s = ByteStream::new(std::io::Cursor::new(data), 1 << 17).unwrap();
                assert!(matches!(
                    c.step_with(&mut s, None, Some(&mut r), &env),
                    Ok(Step::Row(0))
                ));
                // ID at 0 (4 bytes), the field's `fa ff` at 4, its data at 4 + 6
                assert_eq!(r.values, [(1, 2, 2), (2, 10, n)], "{code} {n:#x}");
                assert!(r.pieces.is_empty());
                let st = &c.long_stats()[0];
                assert_eq!(
                    (
                        st.values,
                        st.chunked,
                        st.chained,
                        st.largest,
                        st.sizes_65521_to_65534
                    ),
                    (1, 0, 0, n as u64, 1)
                );
                assert_eq!(st.fff2_to_fff4, 0);
            }
            for (n, z, at, msg) in [
                (
                    0xfff9u16,
                    0u16,
                    6u64,
                    "conventional LONG or LONG RAW 4-byte length outside 65,530 to 65,533",
                ),
                (
                    0xfffe,
                    0,
                    6,
                    "conventional LONG or LONG RAW 4-byte length outside 65,530 to 65,533",
                ),
                (
                    0xfffa,
                    1,
                    8,
                    "conventional LONG or LONG RAW 4-byte length whose upper two bytes are not 00 00",
                ),
            ] {
                let mut l = vec![0xfa, 0xff];
                l.extend_from_slice(&n.to_le_bytes());
                l.extend_from_slice(&z.to_le_bytes());
                let (_, r) = run(code, &l);
                let e = r.unwrap_err();
                assert_eq!(
                    (e.code, e.offset, e.row, e.column, e.length, e.message.as_ref()),
                    (
                        ErrorCode::Framing,
                        Some(at),
                        Some(1),
                        Some(2),
                        Some(u64::from(if at == 6 { n } else { z })),
                        msg
                    ),
                    "{code} {n:#x} {z}"
                );
            }
        }
    }

    /// ADR-99 item 1: a truncation inside each of form (b)'s three parts (n, z, the value) is `E-TRUNCATED`, never a
    /// panic and never a short value.
    #[test]
    fn a_truncation_inside_the_four_byte_length_form_is_truncated() {
        let intro = IntroList::builtin().unwrap();
        let env = Env {
            intro: &intro,
            max_line: 100,
            strict_e2a: false,
        };
        for code in [8u16, 24] {
            // the full field: ID (4 bytes), `fa ff`, n = 0xfffa, z = 0, 0xfffa bytes
            let mut full = vec![0x02, 0x00, 0xc1, 0x02, 0xfa, 0xff, 0xfa, 0xff, 0, 0];
            full.extend(std::iter::repeat_n(b'y', 0xfffa));
            // after `fa ff` (1 byte of n), after n (1 byte of z), after z (the value one byte short)
            for cut in [7usize, 9, full.len() - 1] {
                let mut c =
                    RowCursor::from_columns(PathKind::Conventional, [(2, 22), (code, 0)], 1);
                let mut s = stream(&full[..cut]);
                let e = c.step(&mut s, None, &env).unwrap_err();
                assert_eq!(e.code, ErrorCode::Truncated, "{code} cut {cut}");
            }
        }
    }

    /// ADR-99 item 1: 0xfffb and 0xfffc as the first u16 of a conventional LONG / LONG RAW field are
    /// `UnsupportedLayout` at the u16 with `length` u and the exact text; 0 is `Framing`; `ff ff` is `Framing`; in
    /// another column 0xfff0 to 0xfffd stay `Framing` [R8]; a chunk length from 0xfff0 is `Framing` at the chunk.
    #[test]
    fn the_refusals_of_the_conventional_long_forms() {
        for code in [8u16, 24] {
            for u in [0xfffbu16, 0xfffc] {
                let (_, r) = run(code, &u.to_le_bytes());
                let e = r.unwrap_err();
                assert_eq!(
                    (e.code, e.offset, e.row, e.column, e.length),
                    (
                        ErrorCode::UnsupportedLayout,
                        Some(4),
                        Some(1),
                        Some(2),
                        Some(u64::from(u))
                    )
                );
                assert_eq!(
                    e.message,
                    "conventional LONG or LONG RAW field starting 0xfffb or 0xfffc, not in the fixtures [R8]"
                );
            }
            // 0 is Framing in a LONG column; `ff ff` inside a row too
            assert_eq!(run(code, &[0, 0]).1.unwrap_err().code, ErrorCode::Framing);
            assert_eq!(
                run(code, &[0xff, 0xff]).1.unwrap_err().code,
                ErrorCode::Framing
            );
            // a chunk length from 0xfff0: Framing at the chunk u16, in the third byte pair after `fd ff`
            let e = run(code, &[0xfd, 0xff, 1, 0, b'a', 0xf0, 0xff])
                .1
                .unwrap_err();
            assert_eq!(
                (e.code, e.offset, e.length, e.message.as_ref()),
                (
                    ErrorCode::Framing,
                    Some(9),
                    Some(0xfff0),
                    "conventional LONG or LONG RAW chunk length from 0xfff0"
                )
            );
            // 0xffef is the largest chunk (it runs past the data here: Truncated, never Framing)
            assert_eq!(
                run(code, &[0xfd, 0xff, 0xef, 0xff]).1.unwrap_err().code,
                ErrorCode::Truncated
            );
        }
        // another column: 0xfff0 and 0xfffd stay Framing [R8]
        let (intro, max_line) = env();
        let env = Env {
            intro: &intro,
            max_line,
            strict_e2a: false,
        };
        for u in [0xfff0u16, 0xfffa, 0xfffd] {
            let mut data = u.to_le_bytes().to_vec();
            data.extend_from_slice(&[0, 0, 0xff, 0xff, LF]);
            let mut c = RowCursor::from_columns(PathKind::Conventional, [(1, 10)], 1);
            let mut s = stream(&data);
            let e = c.step(&mut s, None, &env).unwrap_err();
            assert_eq!((e.code, e.offset), (ErrorCode::Framing, Some(0)));
        }
    }

    /// ADR-99 item 2: a chunked value of 65,533 bytes counts as short (the walker warns) and one of 65,534 does not,
    /// for both types; both count in the band.
    #[test]
    fn the_chunked_threshold_is_65534_bytes() {
        for code in [8u16, 24] {
            for (total, short) in [(65_533usize, true), (65_534, false)] {
                let mut l = vec![0xfd, 0xff];
                let mut left = total;
                while left > 0 {
                    let c = left.min(4000);
                    l.extend_from_slice(&(c as u16).to_le_bytes());
                    l.extend(std::iter::repeat_n(b'y', c));
                    left -= c;
                }
                l.extend_from_slice(&[0, 0]);
                let (c, r) = run(code, &l);
                assert!(matches!(r, Ok(Step::Row(0))), "{code} {total}");
                let st = &c.long_stats()[0];
                assert_eq!(
                    (st.chunked, st.chunked_short, st.sizes_65521_to_65534),
                    (1, u64::from(short), 1),
                    "{code} {total}"
                );
            }
        }
    }

    /// ADR-99 item 5: the raw digest of a form (b) value is `f9 ff ff ff`, n as u32 LE and the bytes (the six framing
    /// bytes are not hashed); the assembled digest is that of the same value as a plain field.
    #[test]
    fn the_digests_of_a_four_byte_length_value() {
        let (intro, max_line) = env();
        let env = Env {
            intro: &intro,
            max_line,
            strict_e2a: false,
        };
        let hash_of = |l: &[u8]| {
            let (mut c, data) = long_row(24, l);
            let mut s = stream(&data);
            let mut h = RawHasher::new();
            c.step(&mut s, Some(&mut h), &env).unwrap();
            h.finish(1)
        };
        let mut l = vec![0xfa, 0xff, 0xfa, 0xff, 0, 0];
        l.extend(std::iter::repeat_n(0xab, 0xfffa));
        let (raw, long, _) = hash_of(&l);
        let mut want = Sha256::new();
        want.update([2, 0, 0, 0, 0xc1, 0x02]);
        want.update([0xf9, 0xff, 0xff, 0xff]);
        want.update(0xfffau32.to_le_bytes());
        want.update(vec![0xab; 0xfffa]);
        let want: [u8; 32] = want.finalize().into();
        assert_eq!(raw, want);
        let mut a = Sha256::new();
        a.update(0xfffau64.to_le_bytes());
        a.update(vec![0xab; 0xfffa]);
        let a: [u8; 32] = a.finalize().into();
        assert_eq!(long[0], a);
        // the same bytes as a plain field of the same length are another raw digest, the same assembled one
        // (a plain field of 0xfff9 bytes is the largest: 0xfffa bytes cannot be a plain field)
        let mut p = vec![0xf9, 0xff];
        p.extend(std::iter::repeat_n(0xab, 0xfff9));
        let (raw_plain, _, _) = hash_of(&p);
        assert_ne!(raw_plain, raw);
    }

    /// ADR-95 item 10: the raw digest of a chunked value is `fb ff ff ff`, per data chunk its length as u32 LE and the
    /// bytes, `fa ff ff ff`; the assembled digest is the one of the same value as a plain field.
    #[test]
    fn the_digests_of_a_chunked_value() {
        let intro = IntroList::builtin().unwrap();
        let env = Env {
            intro: &intro,
            max_line: 100,
            strict_e2a: false,
        };
        let hash_of = |l: &[u8]| {
            let (mut c, data) = long_row(8, l);
            let mut s = stream(&data);
            let mut h = RawHasher::new();
            c.step(&mut s, Some(&mut h), &env).unwrap();
            h.finish(1)
        };
        let (raw, long, _) = hash_of(&[0xfd, 0xff, 3, 0, b'a', b'b', b'c', 2, 0, b'd', b'e', 0, 0]);
        let mut want = Sha256::new();
        want.update([2, 0, 0, 0, 0xc1, 0x02]);
        want.update([0xfb, 0xff, 0xff, 0xff]);
        want.update([3, 0, 0, 0]);
        want.update(b"abc");
        want.update([2, 0, 0, 0]);
        want.update(b"de");
        want.update([0xfa, 0xff, 0xff, 0xff]);
        let want: [u8; 32] = want.finalize().into();
        assert_eq!(raw, want);
        // the assembled digest: the total (u64 LE) and the bytes, as for a plain field of 5 bytes
        let (_, plain, _) = hash_of(&[5, 0, b'a', b'b', b'c', b'd', b'e']);
        assert_eq!(long, plain);
        let mut a = Sha256::new();
        a.update(5u64.to_le_bytes());
        a.update(b"abcde");
        let a: [u8; 32] = a.finalize().into();
        assert_eq!(long[0], a);
    }

    #[test]
    fn empty_table_decision_needs_both_tests() {
        // both tests hold: an upper-case letter and a length above the column maximum
        assert_eq!(lf_is_empty_table(b'C', 10), Some(true));
        // neither holds: a row starts (length 0x000a + 256 * 0)
        assert_eq!(lf_is_empty_table(0, 10), Some(false));
        // only one holds: ambiguous
        assert_eq!(lf_is_empty_table(b'C', 20000), None);
        assert_eq!(lf_is_empty_table(0x30, 10), None);
    }

    #[test]
    fn known_item_lines() {
        for ok in [
            &b"TABLE \"T\""[..],
            b"ENDTABLE",
            b"METRICSU",
            b"CREATE INDEX x",
            b"ALTER TABLE x",
            b"COMMENT ON TABLE x IS ",
            b"GRANT SELECT",
            b"ANALSTATS TS \"T\"",
            b"CALTER TABLE",
            b"PARTITION \"P1\"",
            b"COMPARTITION \"P1\"",
            b"SUBPARTITION \"S1\"",
            b"ENDCOMPARTITION",
            b"ENDPARTITION",
        ] {
            assert!(is_known_item_line(ok), "{ok:?}");
        }
        for bad in [
            &b"ENDTABLES"[..],
            b"CALTER TABLE ",
            b"ENDCOMPARTITIONS",
            b"Table \"",
            b"XYZ",
            b"",
        ] {
            assert!(!is_known_item_line(bad), "{bad:?}");
        }
    }

    fn stream(data: &[u8]) -> ByteStream<std::io::Cursor<Vec<u8>>> {
        ByteStream::new(std::io::Cursor::new(data.to_vec()), MIN_BUFFER).unwrap()
    }

    fn follows(data: &[u8], max_line: usize) -> (bool, u64) {
        let intro = IntroList::builtin().unwrap();
        let env = Env {
            intro: &intro,
            max_line,
            strict_e2a: false,
        };
        let mut s = stream(data);
        let r = known_item_follows(&mut s, 0, &env).unwrap();
        (r, s.offset())
    }

    /// [E2a]: the LF at `at`, then a line of a known kind and, where list K says a field follows, a
    /// well-formed field. The stream stands at `at` again afterwards, whatever the answer.
    #[test]
    fn e2a_known_item_with_a_well_formed_field() {
        // a line without a field
        assert_eq!(follows(b"\nENDTABLE\nrest", 100), (true, 0));
        assert_eq!(follows(b"\nTABLE \"T\"\nCREATE", 100), (true, 0));
        // ANALSTATS: plain field `u16 n`, n bytes, LF
        assert_eq!(
            follows(b"\nANALSTATS TS \"T\"\n\x03\x00abc\nTABLE", 100),
            (true, 0)
        );
        // ... in pieces after `fd ff`
        assert_eq!(
            follows(
                b"\nANALSTATS TS \"T\"\n\xfd\xff\x02\x00ab\x00\x00\nTABLE",
                100
            ),
            (true, 0)
        );
        // a field without its LF, with a piece that runs past the end of the file, absent altogether
        assert_eq!(
            follows(b"\nANALSTATS TS \"T\"\n\x03\x00abcZ\n", 100),
            (false, 0)
        );
        assert_eq!(
            follows(b"\nANALSTATS TS \"T\"\n\xfd\xff\x09\x00ab", 100),
            (false, 0)
        );
        assert_eq!(follows(b"\nANALSTATS TS \"T\"\n", 100), (false, 0));
    }

    #[test]
    fn e2a_text_that_is_no_item_or_no_complete_line_is_not_an_empty_table() {
        // a capital letter is not enough
        assert_eq!(follows(b"\nAxxxx\nTABLE", 100), (false, 0));
        assert_eq!(follows(b"\nENDTABLES\n", 100), (false, 0));
        // no LF before the end of the file, a line longer than the limit
        assert_eq!(follows(b"\nENDTABLE", 100), (false, 0));
        assert_eq!(
            follows(b"\nCREATE TABLE xxxxxxxxxxxxxxxxxxxx\n", 10),
            (false, 0)
        );
        // an empty file after the LF
        assert_eq!(follows(b"\n", 100), (false, 0));
    }
}
