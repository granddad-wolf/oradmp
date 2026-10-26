//! Layer 8 (architecture 2.1, 3.2): the row decoder and [`TableBatches`], the first complete read (task T-6).
//!
//! For every part of a table the reader seeks to the part's `rows_offset`, reads the fields in the **file
//! order** of the part and places each one, by `PartInfo.file_order`, into the builder of its column in the
//! **declared order** (ADR-24, ADR-31). A value error is raised or, with `on_value_error="null"`, becomes
//! NULL with a counted `W-VALUE-NULLED`; the value-layer flags become warnings, once per column and table,
//! with part, row, column and offset of the first value and the count (ADR-27 item 2). A batch closes at
//! `batch_rows`, at `batch_bytes` (soft, counted on output bytes, architecture 5.6 step 8) and at the end of
//! a part; a row and a value are never split. Task T-6L adds the LOB values (CLOB, NCLOB, BLOB) of the value
//! area after a row [L3, L4, L5], read chunk by chunk through `RowSink::lob_start` / `lob_chunk` / `lob_end`
//! (architecture 5.6); a column that is not selected is passed by its lengths and never held.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;

use arrow_array::{RecordBatch, RecordBatchReader};
use arrow_schema::{ArrowError, DataType, Schema, SchemaRef};

use crate::arrow_out::{field, ColBuilder};
use crate::charset::{Charset, PieceDecoder, LOB_CONFIRMED_DB_SETS};
use crate::dump::{ColumnInfo, PartInfo, TableInfo};
use crate::error::{Error, ErrorCode, Result, Warning, WarningCode};
use crate::exp::intro::IntroList;
use crate::exp::rows::{Env, LobAction, LobForm, RowCursor, RowSink, Step};
use crate::exp::table::PathKind;
use crate::io::{ByteStream, MIN_BUFFER_BYTES};
use crate::types::declared::DeclaredType;
use crate::types::map::TypeSource;
use crate::values::datetime::{decode_date, decode_timestamp, decode_tsltz, decode_tstz};
use crate::values::interval::{IntervalDs, IntervalYm};
use crate::values::long::{above_confirmed_length, has_high_bytes};
use crate::values::number::{Fitted, OraNumber, OverflowPolicy};
use crate::values::raw::longer_than_declared;
use crate::values::rowid::{decode_rowid, decode_urowid};

/// What a value error does (`on_value_error`, architecture 3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnValueError {
    /// Stop with the error (default).
    #[default]
    Raise,
    /// Write NULL, count it in `W-VALUE-NULLED`.
    Null,
}

/// What happens to a CLOB / NCLOB value of a single-byte database whose decoding no fixture confirms
/// (`unconfirmed_lob`, ADR-25, request R-038).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnconfirmedLob {
    /// A value error `E-LOB-UNCONFIRMED` (default); `on_value_error` decides: stop, or NULL.
    #[default]
    Error,
    /// Decode as the rules [L5] state (CLOB: the descriptor set; NCLOB: UTF-16 BE) with `W-LOB-UNTESTED`.
    Decode,
}

/// Options of one read (architecture 3.2 `ReadOptions`) with the value options of `Options` that the row
/// decoder needs.
#[derive(Debug, Clone)]
pub struct ReadOptions {
    /// Names of the columns to read; `None` reads all. The result keeps the table order. A column that is not
    /// selected is skipped by its length and never decoded.
    pub columns: Option<Vec<String>>,
    /// Most rows in a batch (default 65,536).
    pub batch_rows: usize,
    /// Soft cap of a batch in output bytes (default 64 MiB).
    pub batch_bytes: u64,
    /// `on_value_error`.
    pub on_value_error: OnValueError,
    /// `on_number_overflow`: for a column whose type is not fixed by a declared precision (default `Round`,
    /// owner decision of 2026-10-16).
    pub on_number_overflow: OverflowPolicy,
    /// `max_lob_bytes`: bounds a LONG / LONG RAW value and the stored length of a LOB value (ADR-15, ADR-18);
    /// default 256 MiB.
    pub max_lob_bytes: u64,
    /// `unconfirmed_lob` (ADR-25): default `Error`.
    pub unconfirmed_lob: UnconfirmedLob,
    /// `trim_char` (architecture 5.1): remove the trailing U+0020 of CHAR and NCHAR values (default false:
    /// the padding the file holds is kept). VARCHAR2 and NVARCHAR2 values are never trimmed.
    pub trim_char: bool,
}

impl Default for ReadOptions {
    fn default() -> Self {
        Self {
            columns: None,
            batch_rows: 65_536,
            batch_bytes: 64 << 20,
            on_value_error: OnValueError::Raise,
            on_number_overflow: OverflowPolicy::Round,
            max_lob_bytes: 256 << 20,
            unconfirmed_lob: UnconfirmedLob::Error,
            trim_char: false,
        }
    }
}

/// How a column's bytes are decoded.
#[derive(Debug, Clone, Copy)]
enum Kind {
    NumberI64,
    NumberD128(u8, u8),
    NumberD256(u8, u8),
    NumberF64,
    NumberText,
    Date,
    Timestamp,
    Tstz,
    Tsltz,
    /// VARCHAR2, NVARCHAR2, CHAR, NCHAR; `trim`: remove the trailing U+0020 (CHAR / NCHAR with `trim_char`).
    Text(Charset, bool),
    Raw,
    LongText(Charset),
    LongRaw,
    Rowid,
    Urowid,
    IntervalYm,
    IntervalDs,
    /// `interval_as="string"`: the `TO_CHAR` text of rule Y10 with the leading precision of the declaration
    /// (`None`: the digits each value needs).
    IntervalYmText(Option<usize>),
    /// As above, with the leading and fractional precisions (`None`: digits needed, 9).
    IntervalDsText(Option<(usize, usize)>),
    /// CLOB or NCLOB decoded as text (LargeUtf8).
    LobText {
        set: Charset,
        unconfirmed: Unconfirmed,
    },
    /// BLOB, or a CLOB of a set the reader cannot decode (LargeBinary).
    LobBytes,
}

/// Which values of a CLOB / NCLOB column ADR-25 treats as unconfirmed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unconfirmed {
    /// None (the database set and the column set are confirmed, or the short form of a CLOB).
    Never,
    /// The long form only (a CLOB).
    LongOnly,
    /// Every value (an NCLOB of a database set outside the confirmed list).
    Always,
}

/// The state of the LONG / LONG RAW or LOB value in progress.
#[derive(Debug)]
struct LongState {
    /// Offset of the value (LOB: of its first u16), for an error found at its end.
    start: u64,
    decoder: PieceDecoder,
    /// Stored bytes so far.
    total: u64,
    /// Output bytes written for the value so far.
    out: u64,
    /// A value error happened (kept for `on_value_error="null"`): the rest of the value is passed over.
    failed: bool,
    /// Pieces (LONG) or chunks (LOB) fed so far, 1-based once the first one is in; a failure message names
    /// the one in which the failure was detected (ADR-43 item 3).
    pieces: u64,
}

/// The error `e` of a decoder, with the 1-based piece or chunk `n` of the value in its message (an integer:
/// no value ever enters a message). The offset stays whatever `e` has.
fn in_piece(e: Error, what: &str, n: u64) -> Error {
    Error {
        message: Cow::Owned(format!("{} ({what} {n} of the value)", e.message)),
        ..e
    }
}

#[derive(Debug)]
struct Col {
    kind: Kind,
    builder: ColBuilder,
    policy: OverflowPolicy,
    /// `strict` turned the policy `round` into `error` for this column: an overflow that `round` would have
    /// rounded is then an error that `strict` produced (`Error::warning` = `W-NUMBER-ROUNDED`, ADR-41 item 1).
    strict_round: bool,
    /// Types 1, 23, 96: the descriptor length is a true maximum [C8].
    max_bytes: Option<u16>,
    long: Option<LongState>,
}

/// A warning counted per column and table.
#[derive(Debug)]
struct Counted {
    code: WarningCode,
    col: usize,
    message: &'static str,
    part: u32,
    file_column: u32,
    row: u64,
    offset: u64,
    count: u64,
    to_zero: u64,
    max_dropped: u32,
}

/// What a decoded value reports besides being appended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    None,
    Rounded { to_zero: bool, dropped: u16 },
    Nulled,
    Unverified,
}

fn overflow_err() -> Error {
    Error::new(
        ErrorCode::NumberOverflow,
        "NUMBER does not fit the column type",
    )
}

/// Decodes one field into the builder of `c`. On `Err` nothing was appended. Returns the event and the output
/// bytes the value adds to the batch count.
fn decode_value(c: &mut Col, bytes: &[u8]) -> Result<(Event, u64)> {
    let fixed = c.builder.width();
    let b = &mut c.builder;
    let mut event = Event::None;
    let mut out = fixed;
    match c.kind {
        Kind::NumberI64 => {
            let v = OraNumber::decode(bytes)?
                .to_i64()
                .ok_or_else(overflow_err)?;
            b.append_i64(v)?;
        }
        Kind::NumberD128(p, s) => match OraNumber::decode(bytes)?.to_decimal128(p, s, c.policy)? {
            Fitted::Exact(v) => b.append_d128(v)?,
            Fitted::Rounded {
                value,
                to_zero,
                dropped,
            } => {
                b.append_d128(value)?;
                event = Event::Rounded { to_zero, dropped };
            }
            Fitted::Nulled => {
                b.append_null();
                event = Event::Nulled;
            }
        },
        Kind::NumberD256(p, s) => match OraNumber::decode(bytes)?.to_decimal256(p, s, c.policy)? {
            Fitted::Exact(v) => b.append_d256(v)?,
            Fitted::Rounded {
                value,
                to_zero,
                dropped,
            } => {
                b.append_d256(value)?;
                event = Event::Rounded { to_zero, dropped };
            }
            Fitted::Nulled => {
                b.append_null();
                event = Event::Nulled;
            }
        },
        Kind::NumberF64 => b.append_f64(OraNumber::decode(bytes)?.to_f64()?)?,
        Kind::NumberText => {
            let t = OraNumber::decode(bytes)?.to_text();
            out = u64::try_from(t.len()).unwrap_or(u64::MAX);
            b.append_text(&t)?;
        }
        Kind::Date => b.append_i64(decode_date(bytes)?)?,
        Kind::Timestamp => b.append_i64(timestamp_value(b, &decode_timestamp(bytes)?)?)?,
        Kind::Tstz => b.append_i64(timestamp_value(b, &decode_tstz(bytes)?.instant)?)?,
        Kind::Tsltz => {
            let t = decode_tsltz(bytes)?;
            b.append_i64(timestamp_value(b, &t)?)?;
        }
        Kind::Text(set, trim) => {
            let t = set.decode(bytes)?;
            let t: &str = if trim { t.trim_end_matches(' ') } else { &t };
            out = u64::try_from(t.len()).unwrap_or(u64::MAX);
            b.append_text(t)?;
        }
        Kind::Raw => {
            out = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
            b.append_binary(bytes)?;
        }
        Kind::Rowid => {
            let r = decode_rowid(bytes)?;
            out = u64::try_from(r.text.len()).unwrap_or(u64::MAX);
            b.append_text(&r.text)?;
            if !r.prefix_known {
                event = Event::Unverified;
            }
        }
        Kind::Urowid => {
            let t = decode_urowid(bytes)?;
            out = u64::try_from(t.len()).unwrap_or(u64::MAX);
            b.append_text(&t)?;
        }
        Kind::IntervalYm => {
            let months = IntervalYm::decode(bytes)?.total_months()?;
            b.append_mdn(months, 0, 0)?;
        }
        Kind::IntervalYmText(p) => {
            let v = IntervalYm::decode(bytes)?;
            let t = v.to_text(p.unwrap_or_else(|| digits(v.years.unsigned_abs())));
            out = u64::try_from(t.len()).unwrap_or(u64::MAX);
            b.append_text(&t)?;
        }
        Kind::IntervalDsText(pf) => {
            let v = IntervalDs::decode(bytes)?;
            let (p, f) = pf.unwrap_or_else(|| (digits(v.days.unsigned_abs()), 9));
            let t = v.to_text(p, f);
            out = u64::try_from(t.len()).unwrap_or(u64::MAX);
            b.append_text(&t)?;
        }
        Kind::IntervalDs => {
            let v = IntervalDs::decode(bytes)?;
            match b {
                ColBuilder::DurU(_) => b.append_i64(v.to_duration_micros()?)?,
                ColBuilder::DurN(_) => b.append_i64(v.to_duration_nanos()?)?,
                _ => {
                    let (days, nanos) = v.to_month_day_nano()?;
                    b.append_mdn(0, days, nanos)?;
                }
            }
        }
        // LONG columns go through `long_piece`, LOB columns through `lob_*`
        Kind::LongText(_) | Kind::LongRaw | Kind::LobText { .. } | Kind::LobBytes => {
            return Err(Error::new(
                ErrorCode::Config,
                "LONG or LOB value outside its pieces",
            ))
        }
    }
    Ok((event, out))
}

/// The decimal digits of `n` (1 for 0).
fn digits(n: u64) -> usize {
    usize::try_from(n.checked_ilog10().unwrap_or(0).saturating_add(1)).unwrap_or(usize::MAX)
}

/// The epoch value of an instant in the unit of the timestamp builder.
fn timestamp_value(b: &ColBuilder, t: &crate::values::datetime::Timestamp) -> Result<i64> {
    match b {
        ColBuilder::TsS(_) => Ok(t.secs),
        ColBuilder::TsU(_) => t.to_micros(),
        _ => t.to_nanos(),
    }
}

/// Fills the builders of the selected columns while the cursor reads the rows of one part.
#[derive(Debug)]
struct TableSink {
    table: u32,
    part: u32,
    row: u64,
    cols: Vec<Col>,
    /// For each file position of the current part (index) the column it fills.
    map: Vec<Option<usize>>,
    on_value_error: OnValueError,
    max_lob_bytes: u64,
    decode_unconfirmed: bool,
    /// `strict` (T-11): the warnings outside the confirmed rules are errors.
    strict: bool,
    /// The database set id of the header (decides `E-LOB-UNCONFIRMED`'s text).
    db_charset_id: u16,
    /// Stored bytes of LOB values lent to a builder / LOB values passed without reading (diagnostics).
    lob_bytes_read: u64,
    lob_values_skipped: u64,
    rows_in_batch: u64,
    bytes_in_batch: u64,
    counted: Vec<Counted>,
}

impl TableSink {
    fn ctx(&self, e: Error, column: u32, offset: u64) -> Error {
        let mut e = e;
        if e.offset.is_none() {
            e = e.at(offset);
        }
        e.in_table(self.table)
            .in_part(self.part)
            .in_row(self.row)
            .in_column(column)
    }

    fn note(
        &mut self,
        code: WarningCode,
        message: &'static str,
        col: usize,
        file_column: u32,
        offset: u64,
        to_zero: bool,
    ) {
        if let Some(c) = self
            .counted
            .iter_mut()
            .find(|c| c.code == code && c.col == col)
        {
            c.count = c.count.saturating_add(1);
            c.to_zero = c.to_zero.saturating_add(u64::from(to_zero));
            return;
        }
        self.counted.push(Counted {
            code,
            col,
            message,
            part: self.part,
            file_column,
            row: self.row,
            offset,
            count: 1,
            to_zero: u64::from(to_zero),
            max_dropped: 0,
        });
    }

    /// A warning of the reader outside the confirmed rules: counted, or under `strict` the error of
    /// [`Error::from_warning`] at the value (arch 7.1).
    fn warn(
        &mut self,
        code: WarningCode,
        message: &'static str,
        ci: usize,
        column: u32,
        offset: u64,
    ) -> Result<()> {
        if self.strict && code.strict_error().is_some() {
            let w = Warning::new(code, message, offset)
                .in_table(self.table)
                .in_part(self.part)
                .in_row(self.row)
                .in_column(column);
            if let Some(e) = Error::from_warning(&w) {
                return Err(e);
            }
        }
        self.note(code, message, ci, column, offset, false);
        Ok(())
    }

    /// `W-NUMBER-ROUNDED` for one value: counted with the digits dropped (the largest is kept).
    fn note_rounded(&mut self, ci: usize, column: u32, offset: u64, to_zero: bool, dropped: u16) {
        self.note(
            WarningCode::NumberRounded,
            "NUMBER rounded below the scale (on_number_overflow=round)",
            ci,
            column,
            offset,
            to_zero,
        );
        if let Some(c) = self
            .counted
            .iter_mut()
            .find(|c| c.code == WarningCode::NumberRounded && c.col == ci)
        {
            c.max_dropped = c.max_dropped.max(u32::from(dropped));
        }
    }

    /// A value error: raised, or NULL with a counted warning.
    fn value_error(&mut self, ci: usize, column: u32, offset: u64, e: Error) -> Result<()> {
        match self.on_value_error {
            OnValueError::Raise => Err(self.ctx(e, column, offset)),
            OnValueError::Null => {
                let mut width = 0;
                if let Some(c) = self.cols.get_mut(ci) {
                    c.builder.append_null();
                    width = c.builder.width();
                }
                self.bytes_in_batch = self.bytes_in_batch.saturating_add(width);
                self.note(
                    WarningCode::ValueNulled,
                    "value error: the value was set to NULL (on_value_error=null)",
                    ci,
                    column,
                    offset,
                    false,
                );
                Ok(())
            }
        }
    }

    fn field(&mut self, ci: usize, column: u32, offset: u64, bytes: &[u8]) -> Result<()> {
        let Some(c) = self.cols.get_mut(ci) else {
            return Ok(());
        };
        if matches!(c.kind, Kind::LongText(_) | Kind::LongRaw) {
            return self.long_piece(ci, column, offset, bytes, true, true);
        }
        if matches!(c.kind, Kind::LobText { .. } | Kind::LobBytes) {
            // the locator of a LOB value: skipped, the value follows the row [L2]
            return Ok(());
        }
        let too_long = c
            .max_bytes
            .is_some_and(|m| longer_than_declared(bytes.len(), m));
        let r = decode_value(c, bytes);
        if too_long {
            self.warn(
                WarningCode::LongerThanDeclared,
                "field longer than the descriptor length; delivered as read",
                ci,
                column,
                offset,
            )?;
        }
        match r {
            Ok((event, out)) => {
                self.bytes_in_batch = self.bytes_in_batch.saturating_add(out);
                match event {
                    Event::None => {}
                    Event::Rounded { to_zero, dropped } => {
                        self.note_rounded(ci, column, offset, to_zero, dropped);
                    }
                    Event::Nulled => self.note(
                        WarningCode::NumberNulled,
                        "NUMBER set to NULL (on_number_overflow=null)",
                        ci,
                        column,
                        offset,
                        false,
                    ),
                    Event::Unverified => self.warn(
                        WarningCode::UnverifiedType,
                        "value form not confirmed by a fixture",
                        ci,
                        column,
                        offset,
                    )?,
                }
                Ok(())
            }
            Err(e) => {
                let e = if e.code == ErrorCode::NumberOverflow && self.round_would_change(ci, bytes)
                {
                    e.with_warning(WarningCode::NumberRounded)
                } else {
                    e
                };
                self.value_error(ci, column, offset, e)
            }
        }
    }

    /// Does the column of `ci` have `strict_round` and would `round` have accepted `bytes` (a fraction beyond
    /// the scale, not an integer part that does not fit)? Then the error of `strict` is the conversion of
    /// `W-NUMBER-ROUNDED` (ADR-41 item 1).
    fn round_would_change(&self, ci: usize, bytes: &[u8]) -> bool {
        let Some(c) = self.cols.get(ci).filter(|c| c.strict_round) else {
            return false;
        };
        let Ok(n) = OraNumber::decode(bytes) else {
            return false;
        };
        match c.kind {
            Kind::NumberD128(p, s) => n.to_decimal128(p, s, OverflowPolicy::Round).is_ok(),
            Kind::NumberD256(p, s) => n.to_decimal256(p, s, OverflowPolicy::Round).is_ok(),
            _ => false,
        }
    }

    /// One piece of a LONG / LONG RAW value (a conventional value is one piece) [R2, Y9, ADR-15].
    fn long_piece(
        &mut self,
        ci: usize,
        column: u32,
        offset: u64,
        piece: &[u8],
        first: bool,
        last: bool,
    ) -> Result<()> {
        let max = self.max_lob_bytes;
        let piece_len = u64::try_from(piece.len()).unwrap_or(u64::MAX);
        let mut charset_warning = false;
        let mut failure: Option<Error> = None;
        let mut finished: Option<(u64, u64)> = None;
        let is_raw: bool;
        let value_offset: u64;
        {
            let Some(c) = self.cols.get_mut(ci) else {
                return Ok(());
            };
            let Col {
                kind,
                builder,
                long,
                ..
            } = c;
            let Some(st) = long.as_mut() else {
                return Ok(());
            };
            is_raw = matches!(kind, Kind::LongRaw);
            if first {
                st.decoder.reset();
                st.total = 0;
                st.out = 0;
                st.failed = false;
                st.pieces = 0;
                // the value is reported at its first data byte (ADR-41 item 6)
                st.start = offset;
            }
            st.pieces = st.pieces.saturating_add(1);
            value_offset = st.start;
            let new_total = st.total.saturating_add(piece_len);
            if st.failed {
                st.total = new_total;
            } else if new_total > max {
                st.total = new_total;
                st.failed = true;
                failure = Some(
                    Error::new(ErrorCode::LobTooLarge, "LONG value above max_lob_bytes")
                        .at(st.start)
                        .with_length(new_total),
                );
            } else {
                st.total = new_total;
                let r = match kind {
                    Kind::LongText(set) => {
                        if *set != Charset::Utf8 && has_high_bytes(piece) {
                            charset_warning = true;
                        }
                        let out = &mut st.out;
                        st.decoder.feed(piece, last, &mut |t| {
                            *out = out.saturating_add(u64::try_from(t.len()).unwrap_or(u64::MAX));
                            builder.write_text(t)
                        })
                    }
                    _ => {
                        st.out = st.out.saturating_add(piece_len);
                        builder.write_bytes(piece)
                    }
                };
                if let Err(e) = r {
                    st.failed = true;
                    // one offset per value under every policy: the start of the value; a value of more
                    // than one piece names the piece where the decoder noticed (ADR-43 item 3)
                    let e = e.at(st.start);
                    failure = Some(if first && last {
                        e
                    } else {
                        in_piece(e, "piece", st.pieces)
                    });
                }
            }
            if last {
                if st.failed {
                    builder.append_null();
                } else {
                    builder.finish_value()?;
                    finished = Some((st.total, st.out));
                }
            }
        }
        if charset_warning {
            self.warn(
                WarningCode::LongCharset,
                "LONG byte above 0x7f in a database whose set is not AL32UTF8 (assumed the same)",
                ci,
                column,
                value_offset,
            )?;
        }
        if let Some((total, out)) = finished {
            self.bytes_in_batch = self.bytes_in_batch.saturating_add(out);
            if above_confirmed_length(total) {
                self.warn(
                    WarningCode::LongLength,
                    if is_raw {
                        "LONG RAW value above 100,000 bytes, the largest confirmed"
                    } else {
                        "LONG value above 100,000 bytes, the largest confirmed"
                    },
                    ci,
                    column,
                    value_offset,
                )?;
            }
        }
        match failure {
            Some(e) => match self.on_value_error {
                OnValueError::Raise => Err(self.ctx(e, column, value_offset)),
                OnValueError::Null => {
                    self.note(
                        WarningCode::ValueNulled,
                        "value error: the value was set to NULL (on_value_error=null)",
                        ci,
                        column,
                        value_offset,
                        false,
                    );
                    Ok(())
                }
            },
            None => Ok(()),
        }
    }
}

impl TableSink {
    /// A value of a LOB column starts: refuse it (ADR-25, `max_lob_bytes`) or prepare to read it.
    fn lob_begin(
        &mut self,
        ci: usize,
        column: u32,
        offset: u64,
        form: LobForm,
        total: u64,
    ) -> Result<LobAction> {
        let Some(c) = self.cols.get(ci) else {
            return Ok(LobAction::Skip);
        };
        let unconfirmed = match c.kind {
            Kind::LobText { unconfirmed, .. } => match unconfirmed {
                Unconfirmed::Never => false,
                Unconfirmed::LongOnly => form == LobForm::Long,
                Unconfirmed::Always => true,
            },
            _ => false,
        };
        // under `strict` an unconfirmed value is refused whatever `unconfirmed_lob` says (ADR-34 item 5)
        let refusal = if unconfirmed && (!self.decode_unconfirmed || self.strict) {
            let e = Error::new(
                ErrorCode::LobUnconfirmed,
                lob_unconfirmed_message(self.db_charset_id),
            );
            // `strict` with `unconfirmed_lob="decode"`: the error is the conversion of `W-LOB-UNTESTED`
            Some(if self.strict && self.decode_unconfirmed {
                e.with_warning(WarningCode::LobUntested)
            } else {
                e
            })
        } else if total > self.max_lob_bytes {
            Some(Error::new(
                ErrorCode::LobTooLarge,
                "LOB value above max_lob_bytes",
            ))
        } else {
            None
        };
        if let Some(e) = refusal {
            self.lob_values_skipped = self.lob_values_skipped.saturating_add(1);
            // under `raise` the error; under `null` a NULL and a counted warning
            return self
                .value_error(ci, column, offset, e.with_length(total))
                .map(|()| LobAction::Skip);
        }
        if let Some(st) = self.cols.get_mut(ci).and_then(|c| c.long.as_mut()) {
            st.decoder.reset();
            st.start = offset;
            st.total = total;
            st.out = 0;
            st.failed = false;
            st.pieces = 0;
        }
        if unconfirmed {
            self.note(
                WarningCode::LobUntested,
                "CLOB / NCLOB value decoded by the rules, not confirmed by a fixture (unconfirmed_lob=decode)",
                ci,
                column,
                offset,
                false,
            );
        }
        Ok(LobAction::Read)
    }

    /// One chunk of the LOB value in progress; `last` ends the value (the decoder is told, a builder
    /// finishes the value or, after an earlier failure, takes a NULL).
    fn lob_piece(&mut self, ci: usize, column: u32, bytes: &[u8], last: bool) -> Result<()> {
        let len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        self.lob_bytes_read = self.lob_bytes_read.saturating_add(len);
        let mut failure: Option<Error> = None;
        let mut finished_out: Option<u64> = None;
        let value_offset: u64;
        {
            let Some(c) = self.cols.get_mut(ci) else {
                return Ok(());
            };
            let Col {
                kind,
                builder,
                long,
                ..
            } = c;
            let Some(st) = long.as_mut() else {
                return Ok(());
            };
            value_offset = st.start;
            // the closing call of `lob_end` carries no chunk
            if !(last && bytes.is_empty()) {
                st.pieces = st.pieces.saturating_add(1);
            }
            if !st.failed {
                let r = match kind {
                    Kind::LobText { .. } => {
                        let out = &mut st.out;
                        st.decoder.feed(bytes, last, &mut |t| {
                            *out = out.saturating_add(u64::try_from(t.len()).unwrap_or(u64::MAX));
                            builder.write_text(t)
                        })
                    }
                    _ => {
                        st.out = st.out.saturating_add(len);
                        builder.write_bytes(bytes)
                    }
                };
                if let Err(e) = r {
                    st.failed = true;
                    // the start of the value (the offset `lob_start` received), the chunk in the message
                    failure = Some(in_piece(e.at(st.start), "chunk", st.pieces));
                }
            }
            if last {
                if st.failed {
                    builder.append_null();
                } else {
                    builder.finish_value()?;
                    finished_out = Some(st.out);
                }
            }
        }
        if let Some(out) = finished_out {
            self.bytes_in_batch = self.bytes_in_batch.saturating_add(out);
        }
        match failure {
            None => Ok(()),
            Some(e) => match self.on_value_error {
                OnValueError::Raise => Err(self.ctx(e, column, value_offset)),
                OnValueError::Null => {
                    self.note(
                        WarningCode::ValueNulled,
                        "value error: the value was set to NULL (on_value_error=null)",
                        ci,
                        column,
                        value_offset,
                        false,
                    );
                    Ok(())
                }
            },
        }
    }
}

impl RowSink for TableSink {
    fn row_start(&mut self, row: u64) {
        self.row = row;
    }

    fn null(&mut self, column: u32, _offset: u64) -> Result<()> {
        let index = usize::try_from(column.saturating_sub(1)).unwrap_or(usize::MAX);
        if let Some(Some(ci)) = self.map.get(index).copied() {
            if let Some(c) = self.cols.get_mut(ci) {
                c.builder.append_null();
                self.bytes_in_batch = self.bytes_in_batch.saturating_add(c.builder.width());
            }
        }
        Ok(())
    }

    fn value(&mut self, column: u32, offset: u64, bytes: &[u8]) -> Result<()> {
        let index = usize::try_from(column.saturating_sub(1)).unwrap_or(usize::MAX);
        match self.map.get(index).copied() {
            Some(Some(ci)) => self.field(ci, column, offset, bytes),
            _ => Ok(()),
        }
    }

    fn chain_piece(
        &mut self,
        column: u32,
        offset: u64,
        piece: &[u8],
        first: bool,
        last: bool,
    ) -> Result<()> {
        let index = usize::try_from(column.saturating_sub(1)).unwrap_or(usize::MAX);
        match self.map.get(index).copied() {
            Some(Some(ci)) => self.long_piece(ci, column, offset, piece, first, last),
            _ => Ok(()),
        }
    }

    fn lob_start(
        &mut self,
        column: u32,
        offset: u64,
        form: LobForm,
        total: u64,
    ) -> Result<LobAction> {
        let index = usize::try_from(column.saturating_sub(1)).unwrap_or(usize::MAX);
        match self.map.get(index).copied() {
            Some(Some(ci)) => self.lob_begin(ci, column, offset, form, total),
            _ => {
                self.lob_values_skipped = self.lob_values_skipped.saturating_add(1);
                Ok(LobAction::Skip)
            }
        }
    }

    fn lob_chunk(&mut self, column: u32, _offset: u64, bytes: &[u8]) -> Result<()> {
        let index = usize::try_from(column.saturating_sub(1)).unwrap_or(usize::MAX);
        match self.map.get(index).copied() {
            Some(Some(ci)) => self.lob_piece(ci, column, bytes, false),
            _ => Ok(()),
        }
    }

    fn lob_end(&mut self, column: u32) -> Result<()> {
        let index = usize::try_from(column.saturating_sub(1)).unwrap_or(usize::MAX);
        match self.map.get(index).copied() {
            Some(Some(ci)) => self.lob_piece(ci, column, &[], true),
            _ => Ok(()),
        }
    }

    fn row_end(&mut self) -> Result<()> {
        self.rows_in_batch = self.rows_in_batch.saturating_add(1);
        Ok(())
    }
}

/// What the reader needs to know about one part.
#[derive(Debug, Clone)]
struct PartPlan {
    rows_offset: u64,
    path: PathKind,
    /// For each file position the declared position (0-based).
    file_order: Vec<u32>,
    num_rows: u64,
}

/// What `Dump::read_table` hands to [`TableBatches::new`].
pub(crate) struct ReaderInit<'a> {
    pub path: &'a std::path::Path,
    pub buffer_bytes: usize,
    pub max_line: usize,
    pub table: &'a TableInfo,
    pub db_charset_id: u16,
    /// The `db_charset` override, already parsed.
    pub db_override: Option<Charset>,
    /// List K of the dump (built-in entries and `extra_intro_lines`).
    pub intro: &'a IntroList,
    pub strict: bool,
    pub options: &'a ReadOptions,
}

/// The batches of one table: all parts, in file order, one schema (ADR-21). Implements
/// [`RecordBatchReader`]; [`Self::next_batch`] gives the typed [`Error`] instead of an `ArrowError`.
pub struct TableBatches {
    schema: SchemaRef,
    stream: ByteStream<File>,
    intro: IntroList,
    strict_e2a: bool,
    max_line: usize,
    parts: Vec<PartPlan>,
    part_index: usize,
    cursor: Option<RowCursor>,
    sink: TableSink,
    /// For each part and each file position of it: (type code, maximum length).
    descriptor_cols: Vec<Vec<(u16, u16)>>,
    /// For each declared column the index of its builder, if it is selected.
    declared_to_col: Vec<Option<usize>>,
    table: u32,
    batch_rows: u64,
    batch_bytes: u64,
    rows_read: u64,
    warnings: Vec<Warning>,
    finished: bool,
}

impl std::fmt::Debug for TableBatches {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableBatches")
            .field("rows_read", &self.rows_read)
            .finish_non_exhaustive()
    }
}

/// The set of a VARCHAR2 / CHAR / NVARCHAR2 / NCHAR column: the national set for form 2, else the
/// `db_charset` override if given, else the set of the descriptor [C1, C3, architecture 6.1].
fn text_set_for(c: &ColumnInfo, db_override: Option<Charset>) -> Result<Charset> {
    let id = if c.form == Some(2) {
        2000
    } else {
        if let Some(o) = db_override {
            return Ok(o);
        }
        c.charset_id.unwrap_or(0)
    };
    Charset::from_id(id).ok_or_else(|| {
        Error::new(
            ErrorCode::Config,
            "character set of the column is not supported",
        )
        .with_length(u64::from(id))
        .in_column(c.file_position)
    })
}

/// The decoding of a CLOB / NCLOB column (112) by the type code, the form and the set id together [L5]: an
/// NCLOB and a CLOB of set 873 are UTF-16 BE, a CLOB of a single-byte set is that set, a CLOB of any other set
/// is returned as bytes (the Arrow type of the column is LargeBinary then). ADR-25 (session 8) decides what is
/// unconfirmed by the database set id `db_id` of the header and the list [`LOB_CONFIRMED_DB_SETS`]: an NCLOB
/// value is unconfirmed iff `db_id` is not in the list; the long form of a CLOB of set S iff `db_id` is not in
/// the list or S differs from `db_id`. The `db_charset` override changes the decoder, not this decision.
fn lob_text_kind(c: &ColumnInfo, db_id: u16, db_override: Option<Charset>) -> Kind {
    if c.arrow_type == DataType::LargeBinary {
        return Kind::LobBytes;
    }
    let confirmed = LOB_CONFIRMED_DB_SETS.contains(&db_id);
    let (set, unconfirmed) = if c.form == Some(2) {
        let u = if confirmed {
            Unconfirmed::Never
        } else {
            Unconfirmed::Always
        };
        (Charset::Utf16Be, u)
    } else {
        let long_only = if confirmed && c.charset_id == Some(db_id) {
            Unconfirmed::Never
        } else {
            Unconfirmed::LongOnly
        };
        match c.charset_id {
            Some(873) => (Charset::Utf16Be, long_only),
            Some(id @ (1 | 171 | 178)) => match db_override.or_else(|| Charset::from_id(id)) {
                Some(set) => (set, long_only),
                None => return Kind::LobBytes,
            },
            _ => return Kind::LobBytes,
        }
    };
    Kind::LobText { set, unconfirmed }
}

/// Does ADR-25 refuse a value of this form in this CLOB / NCLOB column (the predicate of `lob_begin`, without
/// the options)? The survey counts with it (task T-11).
pub(crate) fn lob_value_unconfirmed(c: &ColumnInfo, db_id: u16, form: LobForm) -> bool {
    match lob_text_kind(c, db_id, None) {
        Kind::LobText { unconfirmed, .. } => match unconfirmed {
            Unconfirmed::Never => false,
            Unconfirmed::LongOnly => form == LobForm::Long,
            Unconfirmed::Always => true,
        },
        _ => false,
    }
}

/// The text of `E-LOB-UNCONFIRMED`: one text that names the database set id of the header (an integer, ADR-40
/// N-23 (1); never a value).
fn lob_unconfirmed_message(db_id: u16) -> Cow<'static, str> {
    Cow::Owned(format!(
        "CLOB / NCLOB value whose decoding no fixture confirms: database character set id {db_id} of the header (confirmed database sets: 873, 171; a long-form CLOB needs its own set to equal the database set); unconfirmed_lob=\"decode\" reads it"
    ))
}

/// Kind of a column and the policy that applies to it.
fn plan_column(
    c: &ColumnInfo,
    db_set: Option<Charset>,
    db_override: Option<Charset>,
    db_id: u16,
    options: &ReadOptions,
    strict: bool,
) -> Result<(Kind, OverflowPolicy)> {
    let mut policy = OverflowPolicy::Error;
    let kind = match c.type_code {
        2 => {
            // by the origin of the type, never by comparing Arrow types (ADR-33 item 4)
            let constrained = c.type_source == TypeSource::Declared;
            if !constrained {
                // the entry of `number_types` first, then the global policy (ADR-33 item 5); under `strict`
                // a value `round` would change is `E-NUMBER-OVERFLOW` (owner, ADR-14); `null` stays
                policy = c.overflow_policy.unwrap_or(options.on_number_overflow);
                if strict && policy == OverflowPolicy::Round {
                    policy = OverflowPolicy::Error;
                }
            }
            let scale = |s: i8| u8::try_from(s).unwrap_or(0);
            match c.arrow_type {
                DataType::Int64 => Kind::NumberI64,
                DataType::Decimal128(p, s) => Kind::NumberD128(p, scale(s)),
                DataType::Decimal256(p, s) => Kind::NumberD256(p, scale(s)),
                DataType::Float64 => Kind::NumberF64,
                _ => Kind::NumberText,
            }
        }
        // a column in a set the reader cannot decode is Binary (architecture 6.4, `W-CHARSET-UNSUPPORTED` from
        // the scan)
        1 | 96 if c.arrow_type == DataType::Binary => Kind::Raw,
        1 | 96 => Kind::Text(
            text_set_for(c, db_override)?,
            c.type_code == 96 && options.trim_char,
        ),
        8 => Kind::LongText(db_set.ok_or_else(|| {
            Error::new(
                ErrorCode::Config,
                "database character set of the header is not supported: LONG values cannot be decoded (use db_charset)",
            )
            .in_column(c.file_position)
        })?),
        12 => Kind::Date,
        23 => Kind::Raw,
        24 => Kind::LongRaw,
        69 => Kind::Rowid,
        180 => Kind::Timestamp,
        181 => Kind::Tstz,
        182 if c.arrow_type == DataType::Utf8 => Kind::IntervalYmText(match c.declared {
            Some(DeclaredType::IntervalYearToMonth { precision }) => usize::try_from(precision).ok(),
            _ => None,
        }),
        183 if c.arrow_type == DataType::Utf8 => Kind::IntervalDsText(match c.declared {
            Some(DeclaredType::IntervalDayToSecond {
                precision,
                fractional,
            }) => usize::try_from(precision)
                .ok()
                .zip(usize::try_from(fractional).ok()),
            _ => None,
        }),
        182 => Kind::IntervalYm,
        183 => Kind::IntervalDs,
        208 => Kind::Urowid,
        231 => Kind::Tsltz,
        112 => lob_text_kind(c, db_id, db_override),
        113 => Kind::LobBytes,
        _ => {
            return Err(
                Error::new(ErrorCode::Config, "column type the reader does not decode")
                    .in_column(c.file_position),
            )
        }
    };
    Ok((kind, policy))
}

fn metadata_for(c: &ColumnInfo, kind: Kind, policy: OverflowPolicy) -> HashMap<String, String> {
    let mut m = HashMap::new();
    m.insert("oradmp:type_code".to_string(), c.type_code.to_string());
    m.insert("oradmp:oracle_type".to_string(), c.oracle_type.to_string());
    if let Some(t) = &c.declared_text {
        m.insert("oradmp:declared".to_string(), t.clone());
    }
    m.insert("oradmp:max_bytes".to_string(), c.max_bytes.to_string());
    if let Kind::Text(set, _) | Kind::LobText { set, .. } = kind {
        m.insert("oradmp:charset".to_string(), set.name().to_string());
    }
    if matches!(
        kind,
        Kind::NumberI64 | Kind::NumberD128(..) | Kind::NumberD256(..) | Kind::NumberText
    ) {
        let name = match policy {
            OverflowPolicy::Error => "error",
            OverflowPolicy::Round => "round",
            OverflowPolicy::Null => "null",
        };
        m.insert("oradmp:number_overflow".to_string(), name.to_string());
        if let Kind::NumberD128(_, s) | Kind::NumberD256(_, s) = kind {
            m.insert("oradmp:number_scale".to_string(), s.to_string());
        }
    }
    if matches!(kind, Kind::IntervalYmText(_) | Kind::IntervalDsText(_)) {
        m.insert("oradmp:conversion".to_string(), "to-char-text".to_string());
    }
    if matches!(kind, Kind::NumberF64) && c.type_code == 2 {
        m.insert(
            "oradmp:conversion".to_string(),
            "nearest-double".to_string(),
        );
    }
    m
}

impl TableBatches {
    pub(crate) fn new(init: &ReaderInit<'_>) -> Result<Self> {
        let t = init.table;
        let o = init.options;
        let db_override = init.db_override;
        // LONG values use the database set of the header (architecture 6.1); an unsupported one is an error
        // only for a table that has a LONG column
        let db_set = db_override.or_else(|| Charset::from_id(init.db_charset_id));
        // the selection, in table order
        if let Some(names) = &o.columns {
            for n in names {
                if !t.columns.iter().any(|c| &c.name == n) {
                    return Err(
                        Error::new(ErrorCode::NoSuchColumn, "no such column").in_table(t.ordinal)
                    );
                }
            }
        }
        let mut fields = Vec::new();
        let mut cols = Vec::new();
        let mut declared_to_col: Vec<Option<usize>> = Vec::new();
        let mut warnings = Vec::new();
        for c in &t.columns {
            let selected = o
                .columns
                .as_ref()
                .is_none_or(|names| names.iter().any(|n| n == &c.name));
            if !selected {
                declared_to_col.push(None);
                continue;
            }
            let (kind, policy) =
                plan_column(c, db_set, db_override, init.db_charset_id, o, init.strict)
                    .map_err(|e| e.in_table(t.ordinal))?;
            if matches!(c.type_code, 1 | 96 | 112)
                && c.form != Some(2)
                && db_override.is_none()
                && c.charset_id.is_some_and(|id| id != init.db_charset_id)
            {
                let w = Warning::new(
                    WarningCode::CharsetMismatch,
                    "descriptor character set differs from the database set of the header; the descriptor wins",
                    t.create_offset,
                )
                .in_table(t.ordinal)
                .in_column(c.file_position);
                if init.strict {
                    if let Some(e) = Error::from_warning(&w) {
                        return Err(e);
                    }
                }
                warnings.push(w);
            }
            fields.push(field(
                &c.name,
                c.arrow_type.clone(),
                metadata_for(c, kind, policy),
            ));
            let long = matches!(
                kind,
                Kind::LongText(_) | Kind::LongRaw | Kind::LobText { .. } | Kind::LobBytes
            )
            .then(|| LongState {
                decoder: PieceDecoder::new(match kind {
                    Kind::LongText(set) | Kind::LobText { set, .. } => set,
                    _ => Charset::Utf8,
                }),
                start: 0,
                total: 0,
                out: 0,
                failed: false,
                pieces: 0,
            });
            declared_to_col.push(Some(cols.len()));
            let strict_round = init.strict
                && c.type_code == 2
                && c.type_source != TypeSource::Declared
                && c.overflow_policy.unwrap_or(o.on_number_overflow) == OverflowPolicy::Round;
            cols.push(Col {
                kind,
                builder: ColBuilder::new(&c.arrow_type)?,
                policy,
                strict_round,
                max_bytes: matches!(c.type_code, 1 | 23 | 96).then_some(c.max_bytes),
                long,
            });
        }
        let parts: Vec<PartPlan> = t
            .parts
            .iter()
            .map(|p: &PartInfo| PartPlan {
                rows_offset: p.rows_offset,
                path: p.path,
                file_order: p.file_order.clone(),
                num_rows: p.num_rows,
            })
            .collect();
        // (type code, max length) of every file position of every part, and the column each fills
        let mut descriptor_cols = Vec::with_capacity(parts.len());
        for p in &parts {
            let mut d = Vec::with_capacity(p.file_order.len());
            for &declared in &p.file_order {
                let c = t
                    .columns
                    .get(usize::try_from(declared).unwrap_or(usize::MAX))
                    .ok_or_else(|| {
                        Error::new(
                            ErrorCode::Config,
                            "file order names a column that is not there",
                        )
                    })?;
                d.push((c.type_code, c.max_bytes));
            }
            descriptor_cols.push(d);
        }
        let f = File::open(init.path).map_err(Error::io)?;
        // a field is at most 65,535 bytes (a piece 32,767): the buffer must hold one
        let stream = ByteStream::new(f, init.buffer_bytes.max(MIN_BUFFER_BYTES))?;
        let mut sink = TableSink {
            table: t.ordinal,
            part: 0,
            row: 0,
            cols,
            map: Vec::new(),
            on_value_error: o.on_value_error,
            max_lob_bytes: o.max_lob_bytes,
            decode_unconfirmed: o.unconfirmed_lob == UnconfirmedLob::Decode,
            strict: init.strict,
            db_charset_id: init.db_charset_id,
            lob_bytes_read: 0,
            lob_values_skipped: 0,
            rows_in_batch: 0,
            bytes_in_batch: 0,
            counted: Vec::new(),
        };
        sink.map = Vec::new();
        let schema: SchemaRef = Arc::new(Schema::new(fields));
        Ok(Self {
            schema,
            stream,
            intro: init.intro.clone(),
            strict_e2a: init.strict,
            max_line: init.max_line,
            parts,
            part_index: 0,
            cursor: None,
            sink,
            descriptor_cols,
            declared_to_col,
            table: t.ordinal,
            batch_rows: u64::try_from(o.batch_rows.max(1)).unwrap_or(u64::MAX),
            batch_bytes: o.batch_bytes.max(1),
            rows_read: 0,
            warnings,
            finished: false,
        })
    }

    /// The schema (declared column order).
    pub fn schema(&self) -> SchemaRef {
        Arc::clone(&self.schema)
    }

    /// Rows read so far, all parts.
    pub fn rows_read(&self) -> u64 {
        self.rows_read
    }

    /// Stored bytes of LOB values that were lent to a builder so far. A column that is not selected adds
    /// nothing (its values are passed by their lengths).
    pub fn lob_bytes_read(&self) -> u64 {
        self.sink.lob_bytes_read
    }

    /// LOB values passed without reading so far: columns not selected and refused values.
    pub fn lob_values_skipped(&self) -> u64 {
        self.sink.lob_values_skipped
    }

    /// The warnings of the read so far; the counted ones (`W-NUMBER-ROUNDED`, `W-VALUE-NULLED`, ...) are
    /// complete once the reader has returned `None`.
    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }

    /// Starts the part `part_index`: seeks to its rows and sets the file-position map of the sink.
    fn start_part(&mut self) -> Result<bool> {
        let (Some(plan), Some(descriptors)) = (
            self.parts.get(self.part_index),
            self.descriptor_cols.get(self.part_index),
        ) else {
            return Ok(false);
        };
        let part_no = u32::try_from(self.part_index.saturating_add(1)).unwrap_or(u32::MAX);
        self.sink.part = part_no;
        self.sink.row = 0;
        self.sink.map = plan
            .file_order
            .iter()
            .map(|&d| {
                self.declared_to_col
                    .get(usize::try_from(d).unwrap_or(usize::MAX))
                    .copied()
                    .flatten()
            })
            .collect();
        self.cursor = Some(RowCursor::from_columns(
            plan.path,
            descriptors.iter().copied(),
            self.table,
        ));
        self.stream.seek_to(plan.rows_offset)?;
        Ok(true)
    }

    fn finish_batch(&mut self) -> Result<RecordBatch> {
        let arrays = self
            .sink
            .cols
            .iter_mut()
            .map(|c| c.builder.finish())
            .collect();
        let rows = usize::try_from(self.sink.rows_in_batch).unwrap_or(usize::MAX);
        self.sink.rows_in_batch = 0;
        self.sink.bytes_in_batch = 0;
        RecordBatch::try_new_with_options(
            Arc::clone(&self.schema),
            arrays,
            &arrow_array::RecordBatchOptions::new().with_row_count(Some(rows)),
        )
        .map_err(|_| Error::new(ErrorCode::Config, "the columns do not make a batch"))
    }

    /// The next batch, or `None` after the last one. Errors are typed; after an error the reader is done.
    pub fn next_batch(&mut self) -> Result<Option<RecordBatch>> {
        if self.finished {
            return Ok(None);
        }
        let r = self.next_batch_inner();
        if r.is_err() {
            self.finished = true;
            // the counts up to the failing row are reported with the error (ADR-33 item 3)
            self.collect_counted();
        }
        r
    }

    fn next_batch_inner(&mut self) -> Result<Option<RecordBatch>> {
        loop {
            if self.cursor.is_none() && !self.start_part()? {
                self.finished = true;
                self.collect_counted();
                return Ok(None);
            }
            let Some(cursor) = self.cursor.as_mut() else {
                continue;
            };
            let env = Env {
                intro: &self.intro,
                max_line: self.max_line,
                strict_e2a: self.strict_e2a,
            };
            let step = cursor
                .step_with(&mut self.stream, None, Some(&mut self.sink), &env)
                .map_err(|e| {
                    let e = if e.table.is_none() {
                        e.in_table(self.table)
                    } else {
                        e
                    };
                    if e.part.is_none() {
                        e.in_part(self.sink.part)
                    } else {
                        e
                    }
                })?;
            match step {
                Step::Row(_) => {
                    self.rows_read = self.rows_read.saturating_add(1);
                    if self.sink.rows_in_batch >= self.batch_rows
                        || self.sink.bytes_in_batch >= self.batch_bytes
                    {
                        return self.finish_batch().map(Some);
                    }
                }
                Step::End(_) => {
                    let expected = self.parts.get(self.part_index).map(|p| p.num_rows);
                    let seen = cursor.rows();
                    self.cursor = None;
                    self.part_index = self.part_index.saturating_add(1);
                    if expected.is_some_and(|n| n != seen) {
                        return Err(Error::new(
                            ErrorCode::FileChanged,
                            "the part holds another number of rows than the scan counted",
                        )
                        .in_table(self.table)
                        .in_part(self.sink.part)
                        .with_length(seen));
                    }
                    // batches never mix parts (ADR-21)
                    if self.sink.rows_in_batch > 0 {
                        return self.finish_batch().map(Some);
                    }
                }
            }
        }
    }

    /// Turns the counted events into warnings, one per code and column.
    fn collect_counted(&mut self) {
        let mut counted = std::mem::take(&mut self.sink.counted);
        counted.sort_by_key(|c| (c.col, c.file_column));
        for c in counted {
            let mut w = Warning::new(c.code, c.message, c.offset)
                .in_table(self.table)
                .in_part(c.part)
                .in_column(c.file_column)
                .in_row(c.row)
                .with_count(c.count);
            if c.code == WarningCode::NumberRounded {
                w.count_to_zero = Some(c.to_zero);
                w.max_digits_dropped = Some(c.max_dropped);
            }
            self.warnings.push(w);
        }
    }
}

impl Iterator for TableBatches {
    type Item = std::result::Result<RecordBatch, ArrowError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_batch() {
            Ok(Some(b)) => Some(Ok(b)),
            Ok(None) => None,
            Err(e) => Some(Err(ArrowError::ExternalError(Box::new(e)))),
        }
    }
}

impl RecordBatchReader for TableBatches {
    fn schema(&self) -> SchemaRef {
        Arc::clone(&self.schema)
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
    use arrow_array::cast::AsArray;
    use arrow_array::Array;

    fn long_sink(set: Charset, binary: bool, on: OnValueError, max: u64) -> TableSink {
        let (kind, t) = if binary {
            (Kind::LongRaw, DataType::LargeBinary)
        } else {
            (Kind::LongText(set), DataType::LargeUtf8)
        };
        TableSink {
            table: 1,
            part: 1,
            row: 1,
            cols: vec![Col {
                kind,
                builder: ColBuilder::new(&t).unwrap(),
                policy: OverflowPolicy::Error,
                strict_round: false,
                max_bytes: None,
                long: Some(LongState {
                    decoder: PieceDecoder::new(set),
                    start: 0,
                    total: 0,
                    out: 0,
                    failed: false,
                    pieces: 0,
                }),
            }],
            map: vec![Some(0)],
            on_value_error: on,
            max_lob_bytes: max,
            decode_unconfirmed: false,
            strict: false,
            db_charset_id: 873,
            lob_bytes_read: 0,
            lob_values_skipped: 0,
            rows_in_batch: 0,
            bytes_in_batch: 0,
            counted: Vec::new(),
        }
    }

    /// ADR-64 item 5 (d): `in_piece` appends exactly ` (piece N of the value)` / ` (chunk N of the value)` with the
    /// whole decimal number, also above 5 digits, and keeps the code and the offset of the error.
    #[test]
    fn in_piece_appends_the_exact_suffix() {
        for (what, n, text) in [
            ("piece", 2u64, "piece 2"),
            ("chunk", 2, "chunk 2"),
            ("piece", 100_000, "piece 100000"),
            ("chunk", u64::MAX, "chunk 18446744073709551615"),
        ] {
            let e = Error::new(ErrorCode::BadText, "the text is not valid UTF-8").at(77);
            let e = in_piece(e, what, n);
            assert_eq!(
                e.message,
                format!("the text is not valid UTF-8 ({text} of the value)")
            );
            assert_eq!((e.code, e.offset), (ErrorCode::BadText, Some(77)));
        }
    }

    /// A value above 100,000 bytes (the largest confirmed, ADR-99 item 6) gives `W-LONG-LENGTH` once per column and
    /// is delivered whole; 100,000 bytes give none, and 32,001 bytes none either. Pieces of 10,000 bytes (a chain, [R2]).
    #[test]
    fn a_value_above_100000_bytes_gives_w_long_length() {
        for (total, warn) in [(32_001usize, false), (100_000, false), (100_001, true)] {
            let mut s = long_sink(Charset::Utf8, true, OnValueError::Raise, u64::MAX);
            let data = vec![7u8; total];
            let chunks: Vec<&[u8]> = data.chunks(10_000).collect();
            for (i, c) in chunks.iter().enumerate() {
                s.chain_piece(1, 100 + i as u64, c, i == 0, i + 1 == chunks.len())
                    .unwrap();
            }
            s.row_end().unwrap();
            let a = s.cols[0].builder.finish();
            assert_eq!(a.as_binary::<i64>().value(0).len(), total);
            assert_eq!(
                s.counted
                    .iter()
                    .filter(|c| c.code == WarningCode::LongLength)
                    .count(),
                usize::from(warn),
                "{total}"
            );
        }
    }

    /// `max_lob_bytes` is checked as the pieces accumulate: the piece that passes it is `E-LOB-TOO-LARGE`
    /// with the offset of the first piece (ADR-41 item 6) and the running length; under `null` the rest of the chain is passed over and one NULL
    /// is written.
    #[test]
    fn max_lob_bytes_is_checked_piece_by_piece() {
        let mut s = long_sink(Charset::Utf8, false, OnValueError::Raise, 25);
        s.chain_piece(1, 10, &[b'a'; 10], true, false).unwrap();
        s.chain_piece(1, 20, &[b'a'; 10], false, false).unwrap();
        let e = s.chain_piece(1, 30, &[b'a'; 10], false, true).unwrap_err();
        assert_eq!(
            (e.code, e.offset, e.length, e.row, e.column, e.part),
            (
                ErrorCode::LobTooLarge,
                // the first piece (ADR-41 item 6), not the piece that passes the limit
                Some(10),
                Some(30),
                Some(1),
                Some(1),
                Some(1)
            )
        );

        let mut s = long_sink(Charset::Utf8, false, OnValueError::Null, 25);
        s.chain_piece(1, 10, &[b'a'; 10], true, false).unwrap();
        s.chain_piece(1, 20, &[b'a'; 10], false, false).unwrap();
        s.chain_piece(1, 30, &[b'a'; 10], false, false).unwrap();
        s.chain_piece(1, 40, &[b'a'; 10], false, true).unwrap();
        // the next value is intact
        s.chain_piece(1, 50, b"ok", true, true).unwrap();
        let a = s.cols[0].builder.finish();
        let a = a.as_string::<i64>();
        assert_eq!(a.len(), 2);
        assert!(a.is_null(0));
        assert_eq!(a.value(1), "ok");
        assert_eq!(
            s.counted
                .iter()
                .filter(|c| c.code == WarningCode::ValueNulled && c.count == 1)
                .count(),
            1
        );
    }

    /// A zero-length piece (ADR-15: read by the formula) and a character cut between pieces.
    #[test]
    fn empty_pieces_and_split_characters() {
        let mut s = long_sink(Charset::Utf8, false, OnValueError::Raise, u64::MAX);
        let bytes = "a\u{434}\u{1f600}".as_bytes();
        s.chain_piece(1, 10, &bytes[..2], true, false).unwrap();
        s.chain_piece(1, 20, &[], false, false).unwrap();
        s.chain_piece(1, 30, &bytes[2..4], false, false).unwrap();
        s.chain_piece(1, 40, &bytes[4..], false, true).unwrap();
        let a = s.cols[0].builder.finish();
        assert_eq!(a.as_string::<i64>().value(0), "a\u{434}\u{1f600}");
    }

    /// T-19 (ADR-95 items 6, 9): a chunked conventional LONG reaches the sink as pieces, the last one empty; a UTF-8
    /// sequence cut between chunks is decoded whole, and the empty last piece ends the value.
    #[test]
    fn chunks_with_an_empty_last_piece_and_a_character_cut_between_them() {
        let mut s = long_sink(Charset::Utf8, false, OnValueError::Raise, u64::MAX);
        let bytes = "a\u{434}\u{1f600}".as_bytes();
        s.chain_piece(1, 14, &bytes[..2], true, false).unwrap();
        s.chain_piece(1, 20, &bytes[2..5], false, false).unwrap();
        s.chain_piece(1, 30, &bytes[5..], false, false).unwrap();
        s.chain_piece(1, 40, &[], false, true).unwrap();
        // an empty value (`fd ff 00 00`): the first piece is the last and empty
        s.chain_piece(1, 50, &[], true, true).unwrap();
        let a = s.cols[0].builder.finish();
        let a = a.as_string::<i64>();
        assert_eq!(a.value(0), "a\u{434}\u{1f600}");
        assert_eq!((a.len(), a.value(1)), (2, ""));
    }

    /// ADR-99 item 6: the threshold of `W-LONG-LENGTH` is 100,000 bytes for LONG and for LONG RAW, each with its own
    /// text (M10: with the LONG RAW threshold back at 32,000 the first LONG RAW case fails).
    #[test]
    fn the_w_long_length_thresholds_and_texts_by_type() {
        for (binary, total, warn, text) in [
            (false, 32_001usize, false, ""),
            (false, 100_000, false, ""),
            (
                false,
                100_001,
                true,
                "LONG value above 100,000 bytes, the largest confirmed",
            ),
            (true, 32_001, false, ""),
            (true, 100_000, false, ""),
            (
                true,
                100_001,
                true,
                "LONG RAW value above 100,000 bytes, the largest confirmed",
            ),
        ] {
            let mut s = long_sink(Charset::Utf8, binary, OnValueError::Raise, u64::MAX);
            let data = vec![b'a'; total];
            let chunks: Vec<&[u8]> = data.chunks(4000).collect();
            for (i, c) in chunks.iter().enumerate() {
                s.chain_piece(1, 100 + i as u64, c, i == 0, false).unwrap();
            }
            s.chain_piece(1, 9, &[], false, true).unwrap();
            let got: Vec<&str> = s
                .counted
                .iter()
                .filter(|c| c.code == WarningCode::LongLength)
                .map(|c| c.message)
                .collect();
            assert_eq!(got.len(), usize::from(warn), "{binary} {total}");
            if warn {
                assert_eq!(got[0], text);
            }
        }
    }

    /// The batch-byte count of a NUMBER-free row: fixed widths for primitives, bytes for text.
    #[test]
    fn widths_of_the_builders() {
        assert_eq!(
            ColBuilder::new(&DataType::Decimal128(38, 10))
                .unwrap()
                .width(),
            16
        );
        assert_eq!(
            ColBuilder::new(&DataType::Decimal256(76, 38))
                .unwrap()
                .width(),
            32
        );
        assert_eq!(ColBuilder::new(&DataType::Utf8).unwrap().width(), 0);
    }
}
