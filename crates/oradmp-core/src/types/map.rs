//! Declared type and descriptor to Arrow type (architecture 5.1, ADR-6, ADR-14, ADR-15, ADR-19).
//!
//! Pure functions. The descriptor gives the type code, the maximum length, the character set id and the form;
//! the `CREATE TABLE` line gives precision, scale and the interval precisions. Which Arrow type a column gets
//! never depends on the rows (ADR-6).

use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;

use arrow_schema::{DataType, IntervalUnit, TimeUnit};

use crate::charset::Charset;
use crate::error::{Error, ErrorCode};
use crate::exp::table::ColumnDesc;
use crate::types::declared::{DeclaredType, TimestampZone};
use crate::values::number::OverflowPolicy;

/// The Arrow type of a NUMBER column (`number_default`, `number_types`; ADR-14, owner decision Q-1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberType {
    /// `int64`.
    Int64,
    /// `decimal(p,s)`, p up to 38.
    Decimal128 {
        /// Precision.
        precision: u8,
        /// Scale.
        scale: i8,
    },
    /// `decimal256(p,s)`, p up to 76.
    Decimal256 {
        /// Precision.
        precision: u8,
        /// Scale.
        scale: i8,
    },
    /// `float64`.
    Float64,
    /// `string`.
    Utf8,
}

impl NumberType {
    /// The owner's default for a column declared `NUMBER`: Decimal128(38, 10) (Q-1).
    pub const DEFAULT: Self = Self::Decimal128 {
        precision: 38,
        scale: 10,
    };

    /// The Arrow type.
    pub fn arrow(self) -> DataType {
        match self {
            Self::Int64 => DataType::Int64,
            Self::Decimal128 { precision, scale } => DataType::Decimal128(precision, scale),
            Self::Decimal256 { precision, scale } => DataType::Decimal256(precision, scale),
            Self::Float64 => DataType::Float64,
            Self::Utf8 => DataType::Utf8,
        }
    }
}

fn decimal_args(s: &str, max_precision: u8) -> Option<(u8, i8)> {
    let (p, sc) = s.split_once(',')?;
    let p: u8 = p.trim().parse().ok()?;
    let sc: i8 = sc.trim().parse().ok()?;
    let valid = (1..=max_precision).contains(&p) && sc >= 0 && i16::from(sc) <= i16::from(p);
    valid.then_some((p, sc))
}

impl FromStr for NumberType {
    type Err = Error;

    /// `int64`, `float64`, `string`, `decimal(p,s)` (1 <= p <= 38) and `decimal256(p,s)` (1 <= p <= 76), with
    /// 0 <= s <= p. Anything else is `E-CONFIG`.
    fn from_str(text: &str) -> Result<Self, Error> {
        let t = text.trim().to_ascii_lowercase();
        let bad = || Error::new(ErrorCode::Config, "unknown NUMBER type option");
        match t.as_str() {
            "int64" => return Ok(Self::Int64),
            "float64" => return Ok(Self::Float64),
            "string" => return Ok(Self::Utf8),
            _ => {}
        }
        if let Some(inner) = t
            .strip_prefix("decimal256(")
            .and_then(|r| r.strip_suffix(')'))
        {
            let (precision, scale) = decimal_args(inner, 76).ok_or_else(bad)?;
            return Ok(Self::Decimal256 { precision, scale });
        }
        if let Some(inner) = t.strip_prefix("decimal(").and_then(|r| r.strip_suffix(')')) {
            let (precision, scale) = decimal_args(inner, 38).ok_or_else(bad)?;
            return Ok(Self::Decimal128 { precision, scale });
        }
        Err(bad())
    }
}

/// `timestamp_unit` (architecture 3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimestampUnit {
    /// Microsecond for f <= 6, Nanosecond for f 7 to 9 (and Microsecond when f is unknown).
    #[default]
    Auto,
    /// Always Microsecond.
    Micro,
    /// Always Nanosecond.
    Nano,
}

/// `interval_as` (ADR-47): the Arrow type of every INTERVAL column of the dump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntervalAs {
    /// Interval(MonthDayNano) / Duration (ADR-19).
    #[default]
    Native,
    /// Utf8 holding the `TO_CHAR` text of rule Y10.
    String,
}

impl FromStr for IntervalAs {
    type Err = Error;

    /// `native` or `string`; anything else is `E-CONFIG`.
    fn from_str(text: &str) -> Result<Self, Error> {
        match text.trim().to_ascii_lowercase().as_str() {
            "native" => Ok(Self::Native),
            "string" => Ok(Self::String),
            _ => Err(Error::new(
                ErrorCode::Config,
                "interval_as must be \"native\" or \"string\"",
            )),
        }
    }
}

/// The options that decide Arrow types (the part of architecture 3.2 `Options` that task T-5 needs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeOptions {
    /// Type of a column declared `NUMBER` (also of the type-2 fallback).
    pub number_default: NumberType,
    /// Per column, key `TABLE.COLUMN` (names as in the file, ASCII decoding): replaces the type of any
    /// type-2 column (NUMBER, FLOAT, declared precision included).
    pub number_types: BTreeMap<String, NumberType>,
    /// Per column, same key as `number_types` (every key must also be a `number_types` key; `open` refuses
    /// others with `E-CONFIG`): the overflow policy of that entry. It wins over
    /// `ReadOptions.on_number_overflow` (ADR-33 item 5).
    pub number_overflow: BTreeMap<String, OverflowPolicy>,
    /// Unit of TIMESTAMP columns.
    pub timestamp_unit: TimestampUnit,
    /// `interval_as` (ADR-47): fixed for the life of a `Dump`, it changes the schema.
    pub interval_as: IntervalAs,
    /// The `db_charset` override (architecture 6.4): the set of every VARCHAR2 / CHAR column, whatever its
    /// descriptor says. With it no column of type 1 / 96 (form 1) is an unsupported set.
    pub db_charset: Option<Charset>,
}

impl Default for TypeOptions {
    fn default() -> Self {
        Self {
            number_default: NumberType::DEFAULT,
            number_types: BTreeMap::new(),
            number_overflow: BTreeMap::new(),
            timestamp_unit: TimestampUnit::Auto,
            interval_as: IntervalAs::Native,
            db_charset: None,
        }
    }
}

/// Why a declared type does not fit its descriptor or its own limits (column fallback, `W-DDL-FALLBACK`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disagreement {
    /// The type family differs from the descriptor type code (or form).
    Family,
    /// The descriptor length differs from the one the declaration implies [T5, C6].
    Length,
    /// A precision, scale or length outside what the type allows.
    Range,
}

/// The character set widths of rule C6: 4 for AL32UTF8 (873), 1 for the single-byte sets seen (1, 171, 178);
/// `None` for others (not checked).
fn char_width(charset: Option<u16>) -> Option<u32> {
    match charset? {
        873 => Some(4),
        1 | 171 | 178 => Some(1),
        _ => None,
    }
}

fn expect_len(d: &ColumnDesc, want: u32) -> Result<(), Disagreement> {
    if u32::from(d.max_len) == want {
        Ok(())
    } else {
        Err(Disagreement::Length)
    }
}

fn expect_code(d: &ColumnDesc, code: u16, form: Option<u16>) -> Result<(), Disagreement> {
    let form_ok = form.is_none() || d.form == form;
    if d.type_code == code && form_ok {
        Ok(())
    } else {
        Err(Disagreement::Family)
    }
}

/// Architecture 5.5 step 3: type family and, where the fixtures confirm it, the descriptor length.
pub fn check_agreement(decl: &DeclaredType, d: &ColumnDesc) -> Result<(), Disagreement> {
    use DeclaredType as D;
    match *decl {
        D::Number { .. } | D::Float { .. } => {
            expect_code(d, 2, None)?;
            expect_len(d, 22)
        }
        D::Varchar2 { n, char_semantics } => {
            expect_code(d, 1, Some(1))?;
            char_length(d, n, char_semantics, 4000)
        }
        D::Char { n, char_semantics } => {
            expect_code(d, 96, Some(1))?;
            char_length(d, n, char_semantics, 2000)
        }
        D::NVarchar2 { n } => {
            expect_code(d, 1, Some(2))?;
            expect_len(d, n.saturating_mul(2))
        }
        D::NChar { n } => {
            expect_code(d, 96, Some(2))?;
            expect_len(d, n.saturating_mul(2))
        }
        D::Raw { n } => {
            expect_code(d, 23, None)?;
            expect_len(d, n)
        }
        D::Date => {
            expect_code(d, 12, None)?;
            expect_len(d, 7)
        }
        D::Timestamp { fractional, zone } => {
            let (code, len) = match zone {
                TimestampZone::None => (180, if fractional == 0 { 7 } else { 11 }),
                TimestampZone::WithTimeZone => (181, 13),
                TimestampZone::WithLocalTimeZone => (231, if fractional == 0 { 7 } else { 11 }),
            };
            expect_code(d, code, None)?;
            expect_len(d, len)
        }
        D::Clob => {
            expect_code(d, 112, Some(1))?;
            expect_len(d, 72)
        }
        D::NClob => {
            expect_code(d, 112, Some(2))?;
            expect_len(d, 72)
        }
        D::Blob => {
            expect_code(d, 113, None)?;
            expect_len(d, 72)
        }
        D::Long => {
            expect_code(d, 8, None)?;
            expect_len(d, 0)
        }
        D::LongRaw => {
            expect_code(d, 24, None)?;
            expect_len(d, 0)
        }
        D::Rowid => {
            expect_code(d, 69, None)?;
            expect_len(d, 23)
        }
        D::Urowid { n } => {
            expect_code(d, 208, None)?;
            expect_len(d, n)
        }
        D::IntervalYearToMonth { .. } => {
            expect_code(d, 182, None)?;
            expect_len(d, 5)
        }
        D::IntervalDayToSecond { .. } => {
            expect_code(d, 183, None)?;
            expect_len(d, 11)
        }
    }
}

/// `n` bytes, or with character semantics min(w x n, cap) [C6]; other sets than 873 / 1 / 171 / 178: nothing
/// is checked.
fn char_length(d: &ColumnDesc, n: u32, chars: bool, cap: u32) -> Result<(), Disagreement> {
    if !chars {
        return expect_len(d, n);
    }
    match char_width(d.charset_id) {
        Some(w) => expect_len(d, n.saturating_mul(w).min(cap)),
        None => Ok(()),
    }
}

fn timestamp_unit(fractional: Option<u32>, opt: TimestampUnit) -> TimeUnit {
    match opt {
        TimestampUnit::Micro => TimeUnit::Microsecond,
        TimestampUnit::Nano => TimeUnit::Nanosecond,
        TimestampUnit::Auto => match fractional {
            Some(f) if f >= 7 => TimeUnit::Nanosecond,
            _ => TimeUnit::Microsecond,
        },
    }
}

/// Where the type of a column came from (ADR-33 item 4). `on_number_overflow` governs `Default` and
/// `Override` columns; a `Declared` column is exact or `E-NUMBER-OVERFLOW`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TypeSource {
    /// A usable declaration fixed the type (`NUMBER(p,s)` that a Decimal128 holds, `NUMBER(*,s)`, FLOAT, or
    /// any non-NUMBER column with a usable declaration).
    #[default]
    Declared,
    /// `number_default`: plain `NUMBER`, a column or table fallback, a declaration no Decimal128 holds.
    Default,
    /// A `number_types` entry.
    Override,
}

/// The result of [`declared_number`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberFit {
    /// Plain `NUMBER`: the type is `number_default`.
    Unconstrained(NumberType),
    /// The declaration fixes this type.
    Exact(NumberType),
    /// Scale above 38, or precision minus a negative scale above 38: no Decimal128 holds the declaration
    /// (ADR-30 item 3); the type is `number_default`.
    NoDecimal128(NumberType),
}

/// The type of a NUMBER column by its declaration (architecture 5.1). `None`: precision or scale outside what
/// Oracle allows (column fallback).
pub fn declared_number(
    precision: Option<u32>,
    scale: Option<i32>,
    default: NumberType,
) -> Option<NumberFit> {
    if let Some(s) = scale {
        if !(-84..=127).contains(&s) {
            return None;
        }
    }
    if let Some(p) = precision {
        if !(1..=38).contains(&p) {
            return None;
        }
    }
    let dec = |p: i64, s: i64| -> NumberFit {
        match (u8::try_from(p), i8::try_from(s)) {
            (Ok(precision), Ok(scale)) => {
                NumberFit::Exact(NumberType::Decimal128 { precision, scale })
            }
            _ => NumberFit::NoDecimal128(default),
        }
    };
    Some(match (precision, scale) {
        (None, None) => NumberFit::Unconstrained(default),
        (None, Some(s)) => {
            let s = i64::from(s);
            if s == 0 {
                dec(38, 0)
            } else if (1..=38).contains(&s) {
                dec(38, s)
            } else {
                // s > 38, or s < 0 (precision 38 minus a negative scale needs 40 or more digits)
                NumberFit::NoDecimal128(default)
            }
        }
        (Some(p), s) => {
            let p = i64::from(p);
            let s = i64::from(s.unwrap_or(0));
            if s == 0 {
                if p <= 18 {
                    NumberFit::Exact(NumberType::Int64)
                } else {
                    dec(p, 0)
                }
            } else if s > 0 && s <= p {
                dec(p, s)
            } else if s > p {
                if s > 38 {
                    NumberFit::NoDecimal128(default)
                } else {
                    dec(s, s)
                }
            } else {
                let q = p.saturating_sub(s);
                if q <= 18 {
                    NumberFit::Exact(NumberType::Int64)
                } else if q <= 38 {
                    dec(q, 0)
                } else {
                    NumberFit::NoDecimal128(default)
                }
            }
        }
    })
}

/// The Arrow type of an INTERVAL DAY (p) TO SECOND (f) column (ADR-19 session 4).
fn interval_day(p: u32, f: u32) -> DataType {
    if f <= 6 && p <= 8 {
        DataType::Duration(TimeUnit::Microsecond)
    } else if (7..=9).contains(&f) && p <= 5 {
        DataType::Duration(TimeUnit::Nanosecond)
    } else {
        DataType::Interval(IntervalUnit::MonthDayNano)
    }
}

/// A column mapped from its declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredMapping {
    /// The Arrow type.
    pub arrow_type: DataType,
    /// Where the type came from.
    pub source: TypeSource,
    /// A NUMBER declaration that no Decimal128 holds took `number_default` (ADR-30 item 3); the caller raises
    /// `W-DDL-FALLBACK`. False if a `number_types` entry decided the type.
    pub no_decimal128: bool,
}

/// The Arrow type of a column by its declaration and its descriptor. `None`: a parameter outside its range
/// (column fallback). `number_override` is the per-column NUMBER type of `number_types`.
pub fn arrow_from_declared(
    decl: &DeclaredType,
    d: &ColumnDesc,
    opts: &TypeOptions,
    number_override: Option<NumberType>,
) -> Option<DeclaredMapping> {
    let plain = |arrow_type| DeclaredMapping {
        arrow_type,
        source: TypeSource::Declared,
        no_decimal128: false,
    };
    use DeclaredType as D;
    let arrow_type = match *decl {
        D::Number { precision, scale } => {
            let fit = declared_number(precision, scale, opts.number_default)?;
            if let Some(o) = number_override {
                return Some(DeclaredMapping {
                    arrow_type: o.arrow(),
                    source: TypeSource::Override,
                    no_decimal128: false,
                });
            }
            let (t, source, no_decimal128) = match fit {
                NumberFit::Exact(t) => (t, TypeSource::Declared, false),
                NumberFit::Unconstrained(t) => (t, TypeSource::Default, false),
                NumberFit::NoDecimal128(t) => (t, TypeSource::Default, true),
            };
            return Some(DeclaredMapping {
                arrow_type: t.arrow(),
                source,
                no_decimal128,
            });
        }
        D::Float { binary_precision } => {
            if !(1..=126).contains(&binary_precision) {
                return None;
            }
            return Some(match number_override {
                Some(o) => DeclaredMapping {
                    arrow_type: o.arrow(),
                    source: TypeSource::Override,
                    no_decimal128: false,
                },
                None => plain(DataType::Float64),
            });
        }
        D::Varchar2 { .. } | D::NVarchar2 { .. } | D::Char { .. } | D::NChar { .. } => {
            text_type(d, opts)
        }
        D::Raw { .. } => DataType::Binary,
        D::Date => DataType::Timestamp(TimeUnit::Second, None),
        D::Timestamp { fractional, zone } => {
            if fractional > 9 {
                return None;
            }
            let unit = timestamp_unit(Some(fractional), opts.timestamp_unit);
            match zone {
                TimestampZone::WithTimeZone => DataType::Timestamp(unit, Some(Arc::from("UTC"))),
                _ => DataType::Timestamp(unit, None),
            }
        }
        // the set of the descriptor decides (architecture 5.1, rule L5)
        D::Clob | D::NClob => arrow_from_descriptor(d, opts, None),
        D::Blob | D::LongRaw => DataType::LargeBinary,
        D::Long => DataType::LargeUtf8,
        D::Rowid | D::Urowid { .. } => DataType::Utf8,
        D::IntervalYearToMonth { precision } => {
            if precision > 9 {
                return None;
            }
            if opts.interval_as == IntervalAs::String {
                DataType::Utf8
            } else {
                DataType::Interval(IntervalUnit::MonthDayNano)
            }
        }
        D::IntervalDayToSecond {
            precision,
            fractional,
        } => {
            if precision > 9 || fractional > 9 {
                return None;
            }
            if opts.interval_as == IntervalAs::String {
                DataType::Utf8
            } else {
                interval_day(precision, fractional)
            }
        }
    };
    Some(plain(arrow_type))
}

/// Whether the reader can decode the values of a type 1 / 96 column: the national set (form 2) always, the
/// database set when the descriptor id is one of the five of rule X3 or the `db_charset` override is given
/// (architecture 6.4). A descriptor without an id (never read from a file; unit tests only) counts as
/// supported.
pub fn text_charset_supported(d: &ColumnDesc, opts: &TypeOptions) -> bool {
    d.form == Some(2)
        || opts.db_charset.is_some()
        || d.charset_id.is_none_or(|id| Charset::from_id(id).is_some())
}

/// Utf8 for a type 1 / 96 column the reader decodes, Binary for one in an unsupported set (architecture 6.4,
/// reported as `W-CHARSET-UNSUPPORTED`).
fn text_type(d: &ColumnDesc, opts: &TypeOptions) -> DataType {
    if text_charset_supported(d, opts) {
        DataType::Utf8
    } else {
        DataType::Binary
    }
}

/// The character set ids of a CLOB that the reader can decode (architecture 5.1, rule L5): 873, 171, 178, 1.
pub fn clob_charset_supported(charset_id: Option<u16>) -> bool {
    matches!(charset_id, Some(873 | 171 | 178 | 1))
}

/// The Arrow type from the descriptor alone: the fallback of architecture 5.5 step 4. A CLOB (112, form 1)
/// whose set is none of 873, 171, 178, 1 is LargeBinary; the caller reports it (`W-CHARSET-UNSUPPORTED`, see
/// [`clob_charset_supported`]).
pub fn arrow_from_descriptor(
    d: &ColumnDesc,
    opts: &TypeOptions,
    number_override: Option<NumberType>,
) -> DataType {
    match d.type_code {
        1 | 96 => text_type(d, opts),
        69 | 208 => DataType::Utf8,
        2 => number_override.unwrap_or(opts.number_default).arrow(),
        8 => DataType::LargeUtf8,
        12 => DataType::Timestamp(TimeUnit::Second, None),
        23 => DataType::Binary,
        24 | 113 => DataType::LargeBinary,
        112 => {
            if d.form == Some(2) || clob_charset_supported(d.charset_id) {
                DataType::LargeUtf8
            } else {
                DataType::LargeBinary
            }
        }
        180 | 231 => DataType::Timestamp(timestamp_unit(None, opts.timestamp_unit), None),
        181 => DataType::Timestamp(
            timestamp_unit(None, opts.timestamp_unit),
            Some(Arc::from("UTC")),
        ),
        182 | 183 if opts.interval_as == IntervalAs::String => DataType::Utf8,
        182 => DataType::Interval(IntervalUnit::MonthDayNano),
        183 => DataType::Duration(TimeUnit::Microsecond),
        // The walker stops at every other code (ADR-12); never reached through `scan`.
        _ => DataType::Binary,
    }
}
