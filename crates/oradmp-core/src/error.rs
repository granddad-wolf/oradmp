//! Errors and warnings of the reader (architecture 1.4, 7.1 to 7.4).
//!
//! Both carry a stable code, the absolute file offset of the first byte they could not accept and, where
//! they apply, table ordinal, column position, row number and a length. The message is a `Cow<'static, str>`
//! (ADR-40: a text that names an integer such as a character set id or a piece number is built at run time):
//! no value read from the file ever enters an error or a warning (architecture 3.4, "No value in any
//! message"; `tests/t12_messages.rs` checks it). The same holds for names: the table ordinal stands instead
//! of the table name.

use std::borrow::Cow;
use std::fmt::Write as _;

/// Result type of the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Stable code of an [`Error`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    /// Not an `exp` file (byte 0 is not `03`, line 1 is not `EXPORT:V...`, or a compressed file).
    NotExpFile,
    /// The header lines or binary fields cannot be parsed [H3, H4].
    BadHeader,
    /// RECORDLENGTH is not a usable decimal number, or R + 0x104 lies beyond the file [H3, H5].
    BadRecordLength,
    /// The item at R + 0x104 is neither `METRICST` nor `METRICSU` [H6].
    MissingMetricsLine,
    /// A field or line breaks the framing [I2 to I5, T1, T3, R1 to R4, E1 to E3]. Fatal.
    Framing,
    /// A column type code outside [T5]: the file cannot be read beyond it [X1, ADR-12].
    UnsupportedType,
    /// A layout the fixtures do not show and the rules say to stop on: m or a LOB column entry that fits no
    /// confirmed form, a path flag other than 0 or 1, a LOB table on the direct path [T6, L1, X5, ADR-12].
    UnsupportedLayout,
    /// Zero fill or end of file where more data must stand [E4].
    Truncated,
    /// A bad configuration (for example an invalid list of introducing lines).
    Config,
    /// The file differs (size or modification time) from the one that was scanned.
    FileChanged,
    /// No table of that name or ordinal.
    NoSuchTable,
    /// The name belongs to more than one table; use the ordinal.
    AmbiguousTableName,
    /// An operating-system I/O error.
    Io,
    /// A selected column does not exist in the table.
    NoSuchColumn,
    /// Value error: NUMBER bytes that break the checks of [Y1] (architecture 5.2).
    BadNumber,
    /// Value error: a NUMBER does not fit the column type: an integer part that is too large under every
    /// policy, or a fraction beyond the scale under policy `error` (architecture 5.2a).
    NumberOverflow,
    /// Value error: DATE, TIMESTAMP or TSTZ bytes that are not a date, or a date outside the Arrow range
    /// (year below 1, nanoseconds beyond 64 bits; architecture 5.3, 5.4). Python: `ValueDecodeError`.
    BadDatetime,
    /// Value error: INTERVAL bytes outside the confirmed form or not exact in the Arrow type [Y10, ADR-19].
    BadInterval,
    /// Value error: ROWID or UROWID bytes outside the confirmed forms [Y11, ADR-20].
    BadRowid,
    /// Value error: text that is not valid in its character set (architecture 5.6, 5.4 LONG).
    BadText,
    /// Value error: a LOB or LONG value above `max_lob_bytes` (ADR-18, ADR-15).
    LobTooLarge,
    /// Value error: a CLOB value in the long form or an NCLOB value of a database with a single-byte character
    /// set, whose decoding no fixture confirms (R-038, ADR-25). `unconfirmed_lob="decode"` reads it.
    LobUnconfirmed,
    /// `strict=True` turned a warning into an error: the code of the warning is in [`Error::warning`]. The
    /// warnings that have a code of their own under `strict` (`W-NUMBER-ROUNDED`, `W-LOB-UNTESTED`,
    /// `W-CHARSET-UNSUPPORTED`) give that code instead (arch 7.1, ADR-14, ADR-34 item 5).
    Strict,
    /// A character set the reader cannot decode (`W-CHARSET-UNSUPPORTED` under `strict`, arch 6.4).
    UnsupportedCharset,
    /// A panic in the core was caught at the binding (ADR-59). The core never constructs it except through
    /// [`crate::guard::catch_panic`]; it carries no offset, table or row.
    Internal,
}

impl ErrorCode {
    /// Every code, in the order of the enum (the tests of task T-11 take their list from here).
    pub const ALL: [Self; 25] = [
        Self::NotExpFile,
        Self::BadHeader,
        Self::BadRecordLength,
        Self::MissingMetricsLine,
        Self::Framing,
        Self::UnsupportedType,
        Self::UnsupportedLayout,
        Self::Truncated,
        Self::Config,
        Self::FileChanged,
        Self::NoSuchTable,
        Self::AmbiguousTableName,
        Self::Io,
        Self::NoSuchColumn,
        Self::BadNumber,
        Self::NumberOverflow,
        Self::BadDatetime,
        Self::BadInterval,
        Self::BadRowid,
        Self::BadText,
        Self::LobTooLarge,
        Self::LobUnconfirmed,
        Self::Strict,
        Self::UnsupportedCharset,
        Self::Internal,
    ];

    /// The stable text of the code, for example `E-FRAMING`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NotExpFile => "E-NOT-EXP-FILE",
            Self::BadHeader => "E-BAD-HEADER",
            Self::BadRecordLength => "E-BAD-RECORD-LENGTH",
            Self::MissingMetricsLine => "E-MISSING-METRICS-LINE",
            Self::Framing => "E-FRAMING",
            Self::UnsupportedType => "E-UNSUPPORTED-TYPE",
            Self::UnsupportedLayout => "E-UNSUPPORTED-LAYOUT",
            Self::Truncated => "E-TRUNCATED",
            Self::Config => "E-CONFIG",
            Self::FileChanged => "E-FILE-CHANGED",
            Self::NoSuchTable => "E-NO-SUCH-TABLE",
            Self::AmbiguousTableName => "E-AMBIGUOUS-TABLE-NAME",
            Self::Io => "E-IO",
            Self::NoSuchColumn => "E-NO-SUCH-COLUMN",
            Self::BadNumber => "E-BAD-NUMBER",
            Self::NumberOverflow => "E-NUMBER-OVERFLOW",
            Self::BadDatetime => "E-BAD-DATETIME",
            Self::BadInterval => "E-BAD-INTERVAL",
            Self::BadRowid => "E-BAD-ROWID",
            Self::BadText => "E-BAD-TEXT",
            Self::LobTooLarge => "E-LOB-TOO-LARGE",
            Self::LobUnconfirmed => "E-LOB-UNCONFIRMED",
            Self::Strict => "E-STRICT",
            Self::UnsupportedCharset => "E-UNSUPPORTED-CHARSET",
            Self::Internal => "E-INTERNAL",
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A typed error with the context of architecture 7.4.
#[derive(Debug, thiserror::Error)]
#[error("{}", render(.code.as_str(), .offset, .table, .part, .column, .row, .length, .type_code, .warning.map(WarningCode::as_str), .message))]
pub struct Error {
    /// Stable code.
    pub code: ErrorCode,
    /// Absolute file offset of the first byte that could not be accepted.
    pub offset: Option<u64>,
    /// Ordinal (1-based) of the table section.
    pub table: Option<u32>,
    /// Index (1-based) of the binary part in the table (1 for an unpartitioned table); set whenever the
    /// error belongs to a part, always when `row` is set (ADR-29).
    pub part: Option<u32>,
    /// Position (1-based) of the column in the **file order** of its part (ADR-29), never the declared
    /// position.
    pub column: Option<u32>,
    /// Row number (1-based) within the part.
    pub row: Option<u64>,
    /// A length that took part in the decision (bytes).
    pub length: Option<u64>,
    /// The column type code of an `UnsupportedType` error (structure, not a value).
    pub type_code: Option<u16>,
    /// The warning that `strict=True` turned into this error (`Strict` and the three codes of
    /// [`WarningCode::strict_error`]); structure, not a value.
    pub warning: Option<WarningCode>,
    /// Explanation: a static text, or one that names integers (set id, piece number), never a value.
    pub message: Cow<'static, str>,
    /// The operating-system error of an `Io` error.
    #[source]
    pub source: Option<std::io::Error>,
}

impl Error {
    /// A new error without context.
    pub fn new(code: ErrorCode, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            code,
            offset: None,
            table: None,
            part: None,
            column: None,
            row: None,
            length: None,
            type_code: None,
            warning: None,
            message: message.into(),
            source: None,
        }
    }

    /// The error that `strict=True` makes of `w` (arch 7.1): code `Strict`, except `W-NUMBER-ROUNDED`
    /// (`NumberOverflow`), `W-LOB-UNTESTED` (`LobUnconfirmed`) and `W-CHARSET-UNSUPPORTED`
    /// (`UnsupportedCharset`). `None` for a warning that reports a chosen policy (`W-NUMBER-NULLED`,
    /// `W-VALUE-NULLED`): `strict` does not change those. Offset, table, part, column, row and length are the
    /// warning's.
    pub fn from_warning(w: &Warning) -> Option<Self> {
        let code = w.code.strict_error()?;
        let message = match code {
            ErrorCode::NumberOverflow => {
                "strict: a NUMBER that rounding would change is an error (on_number_overflow=round)"
            }
            ErrorCode::LobUnconfirmed => {
                "strict: a CLOB / NCLOB value whose decoding no fixture confirms is an error"
            }
            ErrorCode::UnsupportedCharset => {
                "strict: a character set the reader cannot decode is an error"
            }
            _ => "strict: a warning outside the confirmed rules is an error (warning code in the context)",
        };
        let mut e = Self::new(code, message);
        e.offset = w.offset;
        e.table = w.table;
        e.part = w.part;
        e.column = w.column;
        e.row = w.row;
        e.length = w.length;
        e.warning = Some(w.code);
        Some(e)
    }

    /// An I/O error.
    pub fn io(source: std::io::Error) -> Self {
        let mut e = Self::new(ErrorCode::Io, "I/O error");
        e.source = Some(source);
        e
    }

    /// Sets the file offset.
    #[must_use]
    pub fn at(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Sets the table ordinal.
    #[must_use]
    pub fn in_table(mut self, table: u32) -> Self {
        self.table = Some(table);
        self
    }

    /// Sets the part index (1-based).
    #[must_use]
    pub fn in_part(mut self, part: u32) -> Self {
        self.part = Some(part);
        self
    }

    /// Sets the column position.
    #[must_use]
    pub fn in_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    /// Sets the row number.
    #[must_use]
    pub fn in_row(mut self, row: u64) -> Self {
        self.row = Some(row);
        self
    }

    /// Sets a length.
    #[must_use]
    pub fn with_length(mut self, length: u64) -> Self {
        self.length = Some(length);
        self
    }

    /// Sets the warning that `strict` turned into this error.
    #[must_use]
    pub fn with_warning(mut self, warning: WarningCode) -> Self {
        self.warning = Some(warning);
        self
    }

    /// Sets the type code.
    #[must_use]
    pub fn with_type_code(mut self, type_code: u16) -> Self {
        self.type_code = Some(type_code);
        self
    }
}

/// Stable code of a [`Warning`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WarningCode {
    /// A line not in list K was followed by something that looks like a field (G1).
    UnknownIntro,
    /// Non-zero bytes after the second `EXIT` line.
    Trailing,
    /// The version line is neither `EXPORT:V18.00.00` nor `EXPORT:V11.02.00`.
    UntestedVersion,
    /// The mode line or RECORDLENGTH is outside the values of the fixtures.
    UntestedHeader,
    /// The file size is not a multiple of RECORDLENGTH [H5].
    FileSize,
    /// A layout that no fixture shows but the reader can still follow: the 2 bytes after a LOB column entry,
    /// an entry name that differs from the `INSERT INTO` name [L1], partition lines out of the confirmed
    /// order [T10, ADR-21], the "no rows" decision of [E2a] (ADR-22), a piece of length 0 in a direct-path
    /// LONG chain [R2, ADR-15], a long LOB value of total length 0 [L4, ADR-23].
    UntestedLayout,
    /// The `CREATE TABLE` line or one column definition cannot be used: the table or the column is mapped from
    /// its descriptor alone [T9, ADR-6, architecture 5.5 step 4].
    DdlFallback,
    /// A CLOB whose character set the reader cannot decode: the column is returned as bytes [L5].
    CharsetUnsupported,
    /// A value the reader could decode but whose form no fixture shows: a ROWID prefix other than the
    /// confirmed one (ADR-27 item 2; the 7-byte TIMESTAMP WITH LOCAL TIME ZONE is confirmed, ADR-100 item 1).
    UnverifiedType,
    /// A LONG / LONG RAW value above 100,000 bytes, the largest confirmed one (ADR-15, ADR-99 item 6).
    LongLength,
    /// A LONG value with a byte above 0x7f in a database whose set is not AL32UTF8 ("assumed the same",
    /// ADR-15 item 4).
    LongCharset,
    /// A field longer than the maximum of its descriptor [C8, ADR-26].
    LongerThanDeclared,
    /// `on_number_overflow="round"` changed values of a column (counted, ADR-14).
    NumberRounded,
    /// `on_number_overflow="null"` set values of a column to NULL (counted, ADR-14).
    NumberNulled,
    /// `on_value_error="null"` set values of a column to NULL after a value error (counted, ADR-11).
    ValueNulled,
    /// The character set id of a descriptor differs from the database set of the header [H4]; the
    /// descriptor wins (architecture 6.1).
    CharsetMismatch,
    /// A CLOB / NCLOB value of a single-byte database was decoded although no fixture confirms the reading
    /// (`unconfirmed_lob="decode"`, counted per column; ADR-25).
    LobUntested,
    /// A TIMESTAMP WITH LOCAL TIME ZONE column: values are returned as stored, without a zone; whether they
    /// are UTC or in the database time zone is unknown (ADR-27 item 2, once per column).
    Tsltz,
    /// A table with no rows and a possibly damaged name: the export may have failed, the file cannot tell
    /// [X2, architecture 6.5].
    PossiblyIncomplete,
}

impl WarningCode {
    /// The stable text of the code, for example `W-UNKNOWN-INTRO`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnknownIntro => "W-UNKNOWN-INTRO",
            Self::Trailing => "W-TRAILING",
            Self::UntestedVersion => "W-UNTESTED-VERSION",
            Self::UntestedHeader => "W-UNTESTED-HEADER",
            Self::FileSize => "W-FILE-SIZE",
            Self::UntestedLayout => "W-UNTESTED-LAYOUT",
            Self::DdlFallback => "W-DDL-FALLBACK",
            Self::CharsetUnsupported => "W-CHARSET-UNSUPPORTED",
            Self::UnverifiedType => "W-UNVERIFIED-TYPE",
            Self::LongLength => "W-LONG-LENGTH",
            Self::LongCharset => "W-LONG-CHARSET",
            Self::LongerThanDeclared => "W-LONGER-THAN-DECLARED",
            Self::NumberRounded => "W-NUMBER-ROUNDED",
            Self::NumberNulled => "W-NUMBER-NULLED",
            Self::ValueNulled => "W-VALUE-NULLED",
            Self::CharsetMismatch => "W-CHARSET-MISMATCH",
            Self::LobUntested => "W-LOB-UNTESTED",
            Self::Tsltz => "W-TSLTZ",
            Self::PossiblyIncomplete => "W-POSSIBLY-INCOMPLETE",
        }
    }
}

impl WarningCode {
    /// Every code, in the order of the enum (the tests of task T-11 take their list from here).
    pub const ALL: [Self; 19] = [
        Self::UnknownIntro,
        Self::Trailing,
        Self::UntestedVersion,
        Self::UntestedHeader,
        Self::FileSize,
        Self::UntestedLayout,
        Self::DdlFallback,
        Self::CharsetUnsupported,
        Self::UnverifiedType,
        Self::LongLength,
        Self::LongCharset,
        Self::LongerThanDeclared,
        Self::NumberRounded,
        Self::NumberNulled,
        Self::ValueNulled,
        Self::CharsetMismatch,
        Self::LobUntested,
        Self::Tsltz,
        Self::PossiblyIncomplete,
    ];

    /// The error code this warning has under `strict=True` (arch 7.1), or `None` if `strict` leaves it alone:
    /// `W-NUMBER-NULLED` and `W-VALUE-NULLED` report a policy the caller chose.
    pub fn strict_error(self) -> Option<ErrorCode> {
        match self {
            Self::NumberNulled | Self::ValueNulled => None,
            Self::NumberRounded => Some(ErrorCode::NumberOverflow),
            Self::LobUntested => Some(ErrorCode::LobUnconfirmed),
            Self::CharsetUnsupported => Some(ErrorCode::UnsupportedCharset),
            _ => Some(ErrorCode::Strict),
        }
    }
}

impl std::fmt::Display for WarningCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A warning: reading goes on, `strict` mode (task T-11) turns it into an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warning {
    /// Stable code.
    pub code: WarningCode,
    /// Absolute file offset.
    pub offset: Option<u64>,
    /// Ordinal (1-based) of the table section.
    pub table: Option<u32>,
    /// Index (1-based) of the binary part in the table, where the warning belongs to one; always set when
    /// `row` is (ADR-29).
    pub part: Option<u32>,
    /// Position (1-based) of the column in the file order of its part, where the warning belongs to one.
    pub column: Option<u32>,
    /// Number (1-based) of the row within the part, where the warning belongs to one.
    pub row: Option<u64>,
    /// A length that took part in the decision (bytes).
    pub length: Option<u64>,
    /// For the counted warnings (`W-NUMBER-ROUNDED`, `W-NUMBER-NULLED`, `W-VALUE-NULLED`, and the other
    /// once-per-column ones): the number of values concerned in the table. `row`, `column` and `offset`
    /// are those of the first one.
    pub count: Option<u64>,
    /// `W-NUMBER-ROUNDED`: how many of the rounded values became 0 (owner decision Q-1).
    pub count_to_zero: Option<u64>,
    /// `W-NUMBER-ROUNDED`: the largest number of digits dropped below the scale over the rounded values of the
    /// column (fractional digits of the value minus the scale of the column; ADR-33 item 5).
    pub max_digits_dropped: Option<u32>,
    /// Explanation: a static text, or one that names integers (set id, piece number), never a value.
    pub message: Cow<'static, str>,
}

impl Warning {
    /// A new warning at an offset.
    pub fn new(code: WarningCode, message: impl Into<Cow<'static, str>>, offset: u64) -> Self {
        Self {
            code,
            offset: Some(offset),
            table: None,
            part: None,
            column: None,
            row: None,
            length: None,
            count: None,
            count_to_zero: None,
            max_digits_dropped: None,
            message: message.into(),
        }
    }

    /// Sets the length.
    #[must_use]
    pub fn with_length(mut self, length: u64) -> Self {
        self.length = Some(length);
        self
    }

    /// Sets the count of values.
    #[must_use]
    pub fn with_count(mut self, count: u64) -> Self {
        self.count = Some(count);
        self
    }

    /// Sets the column position.
    #[must_use]
    pub fn in_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    /// Sets the row number.
    #[must_use]
    pub fn in_row(mut self, row: u64) -> Self {
        self.row = Some(row);
        self
    }

    /// Sets the table ordinal.
    #[must_use]
    pub fn in_table(mut self, table: u32) -> Self {
        self.table = Some(table);
        self
    }

    /// Sets the part index (1-based).
    #[must_use]
    pub fn in_part(mut self, part: u32) -> Self {
        self.part = Some(part);
        self
    }
}

impl std::fmt::Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&render(
            self.code.as_str(),
            &self.offset,
            &self.table,
            &self.part,
            &self.column,
            &self.row,
            &self.length,
            &None,
            None,
            &self.message,
        ))?;
        if let Some(n) = self.count {
            write!(f, " [{n} values")?;
            if let Some(z) = self.count_to_zero {
                write!(f, ", {z} became 0")?;
            }
            if let Some(d) = self.max_digits_dropped {
                write!(f, ", at most {d} digits dropped")?;
            }
            f.write_str("]")?;
        }
        Ok(())
    }
}

/// `[E-FRAMING offset=0x1a2b] message (table 2, part 1, column 3, row 4, length 5, type code 6)`: the code and
/// the offset first, so that they survive a consumer that keeps only the message (architecture 3.4).
#[allow(clippy::too_many_arguments)]
fn render(
    code: &str,
    offset: &Option<u64>,
    table: &Option<u32>,
    part: &Option<u32>,
    column: &Option<u32>,
    row: &Option<u64>,
    length: &Option<u64>,
    type_code: &Option<u16>,
    warning: Option<&str>,
    message: &str,
) -> String {
    let mut s = String::new();
    s.push('[');
    s.push_str(code);
    if let Some(o) = offset {
        let _ = write!(s, " offset={o:#x}");
    }
    s.push_str("] ");
    s.push_str(message);
    let mut parts: Vec<String> = Vec::new();
    if let Some(v) = table {
        parts.push(format!("table {v}"));
    }
    if let Some(v) = part {
        parts.push(format!("part {v}"));
    }
    if let Some(v) = column {
        parts.push(format!("column {v}"));
    }
    if let Some(v) = row {
        parts.push(format!("row {v}"));
    }
    if let Some(v) = length {
        parts.push(format!("length {v}"));
    }
    if let Some(v) = type_code {
        parts.push(format!("type code {v}"));
    }
    if let Some(v) = warning {
        parts.push(format!("warning {v}"));
    }
    if !parts.is_empty() {
        s.push_str(" (");
        s.push_str(&parts.join(", "));
        s.push(')');
    }
    s
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;

    /// A new variant does not compile here until it is named, and then it must be in `ALL`.
    fn error_listed(c: ErrorCode) -> bool {
        match c {
            ErrorCode::NotExpFile
            | ErrorCode::BadHeader
            | ErrorCode::BadRecordLength
            | ErrorCode::MissingMetricsLine
            | ErrorCode::Framing
            | ErrorCode::UnsupportedType
            | ErrorCode::UnsupportedLayout
            | ErrorCode::Truncated
            | ErrorCode::Config
            | ErrorCode::FileChanged
            | ErrorCode::NoSuchTable
            | ErrorCode::AmbiguousTableName
            | ErrorCode::Io
            | ErrorCode::NoSuchColumn
            | ErrorCode::BadNumber
            | ErrorCode::NumberOverflow
            | ErrorCode::BadDatetime
            | ErrorCode::BadInterval
            | ErrorCode::BadRowid
            | ErrorCode::BadText
            | ErrorCode::LobTooLarge
            | ErrorCode::LobUnconfirmed
            | ErrorCode::Strict
            | ErrorCode::Internal
            | ErrorCode::UnsupportedCharset => ErrorCode::ALL.contains(&c),
        }
    }

    fn warning_listed(c: WarningCode) -> bool {
        match c {
            WarningCode::UnknownIntro
            | WarningCode::Trailing
            | WarningCode::UntestedVersion
            | WarningCode::UntestedHeader
            | WarningCode::FileSize
            | WarningCode::UntestedLayout
            | WarningCode::DdlFallback
            | WarningCode::CharsetUnsupported
            | WarningCode::UnverifiedType
            | WarningCode::LongLength
            | WarningCode::LongCharset
            | WarningCode::LongerThanDeclared
            | WarningCode::NumberRounded
            | WarningCode::NumberNulled
            | WarningCode::ValueNulled
            | WarningCode::CharsetMismatch
            | WarningCode::LobUntested
            | WarningCode::Tsltz
            | WarningCode::PossiblyIncomplete => WarningCode::ALL.contains(&c),
        }
    }

    #[test]
    fn all_lists_every_code_once() {
        assert!(ErrorCode::ALL.iter().all(|&c| error_listed(c)));
        assert!(WarningCode::ALL.iter().all(|&c| warning_listed(c)));
        let mut e: Vec<&str> = ErrorCode::ALL.iter().map(|c| c.as_str()).collect();
        e.sort_unstable();
        e.dedup();
        assert_eq!(e.len(), ErrorCode::ALL.len());
        let mut w: Vec<&str> = WarningCode::ALL.iter().map(|c| c.as_str()).collect();
        w.sort_unstable();
        w.dedup();
        assert_eq!(w.len(), WarningCode::ALL.len());
        assert!(e.iter().all(|c| c.starts_with("E-")));
        assert!(w.iter().all(|c| c.starts_with("W-")));
    }

    #[test]
    fn strict_makes_an_error_of_every_warning_but_the_two_policies() {
        for c in WarningCode::ALL {
            let w = Warning::new(c, "m", 7).in_table(2).in_column(3);
            match c {
                WarningCode::NumberNulled | WarningCode::ValueNulled => {
                    assert!(Error::from_warning(&w).is_none(), "{c}");
                }
                _ => {
                    let e = Error::from_warning(&w).unwrap();
                    assert_eq!(
                        (e.offset, e.table, e.column, e.warning),
                        (Some(7), Some(2), Some(3), Some(c)),
                        "{c}"
                    );
                    assert_eq!(Some(e.code), c.strict_error());
                }
            }
        }
    }
}
