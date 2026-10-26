//! Layer 7 (architecture 2.1): column builders, the schema and its field metadata (architecture 5.1).
//!
//! A [`ColBuilder`] is the Arrow builder of one column. Variable-size values can be written in pieces
//! (`write_text`, `write_bytes`) and are finished by `finish_value`: `GenericStringBuilder` implements
//! `std::fmt::Write` and `GenericBinaryBuilder` implements `std::io::Write`, the written bytes become part of
//! the next value and `append_value("")` ends it (arrow-array 60.0.0, tested in this module; architecture
//! 4.1 (e)). The builders count nothing; [`ColBuilder::width`] says how many output bytes a value adds to
//! the running count of `batch_bytes` (architecture 5.6 step 8).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::io::Write as _;
use std::sync::Arc;

use arrow_array::builder::{
    BinaryBuilder, Decimal128Builder, Decimal256Builder, DurationMicrosecondBuilder,
    DurationNanosecondBuilder, Float64Builder, Int64Builder, IntervalMonthDayNanoBuilder,
    LargeBinaryBuilder, LargeStringBuilder, StringBuilder, TimestampMicrosecondBuilder,
    TimestampNanosecondBuilder, TimestampSecondBuilder,
};
use arrow_array::types::IntervalMonthDayNanoType;
use arrow_array::ArrayRef;
use arrow_buffer::i256;
use arrow_schema::{DataType, Field, IntervalUnit, TimeUnit};

use crate::error::{Error, ErrorCode, Result};

/// The Arrow builder of one column.
pub enum ColBuilder {
    /// `Int64`.
    I64(Int64Builder),
    /// `Decimal128(p, s)`.
    D128(Decimal128Builder),
    /// `Decimal256(p, s)`.
    D256(Decimal256Builder),
    /// `Float64`.
    F64(Float64Builder),
    /// `Utf8`.
    Str(StringBuilder),
    /// `Binary`.
    Bin(BinaryBuilder),
    /// `LargeUtf8`.
    LStr(LargeStringBuilder),
    /// `LargeBinary`.
    LBin(LargeBinaryBuilder),
    /// `Timestamp(Second)`.
    TsS(TimestampSecondBuilder),
    /// `Timestamp(Microsecond)`.
    TsU(TimestampMicrosecondBuilder),
    /// `Timestamp(Nanosecond)`.
    TsN(TimestampNanosecondBuilder),
    /// `Interval(MonthDayNano)`.
    Mdn(IntervalMonthDayNanoBuilder),
    /// `Duration(Microsecond)`.
    DurU(DurationMicrosecondBuilder),
    /// `Duration(Nanosecond)`.
    DurN(DurationNanosecondBuilder),
}

impl std::fmt::Debug for ColBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ColBuilder")
    }
}

fn config(message: &'static str) -> Error {
    Error::new(ErrorCode::Config, message)
}

impl ColBuilder {
    /// A builder for the Arrow type `t`; a type the reader never produces is `E-CONFIG`.
    pub fn new(t: &DataType) -> Result<Self> {
        Ok(match t {
            DataType::Int64 => Self::I64(Int64Builder::new()),
            DataType::Decimal128(..) => {
                Self::D128(Decimal128Builder::new().with_data_type(t.clone()))
            }
            DataType::Decimal256(..) => {
                Self::D256(Decimal256Builder::new().with_data_type(t.clone()))
            }
            DataType::Float64 => Self::F64(Float64Builder::new()),
            DataType::Utf8 => Self::Str(StringBuilder::new()),
            DataType::Binary => Self::Bin(BinaryBuilder::new()),
            DataType::LargeUtf8 => Self::LStr(LargeStringBuilder::new()),
            DataType::LargeBinary => Self::LBin(LargeBinaryBuilder::new()),
            DataType::Timestamp(TimeUnit::Second, _) => {
                Self::TsS(TimestampSecondBuilder::new().with_data_type(t.clone()))
            }
            DataType::Timestamp(TimeUnit::Microsecond, _) => {
                Self::TsU(TimestampMicrosecondBuilder::new().with_data_type(t.clone()))
            }
            DataType::Timestamp(TimeUnit::Nanosecond, _) => {
                Self::TsN(TimestampNanosecondBuilder::new().with_data_type(t.clone()))
            }
            DataType::Interval(IntervalUnit::MonthDayNano) => {
                Self::Mdn(IntervalMonthDayNanoBuilder::new())
            }
            DataType::Duration(TimeUnit::Microsecond) => {
                Self::DurU(DurationMicrosecondBuilder::new())
            }
            DataType::Duration(TimeUnit::Nanosecond) => {
                Self::DurN(DurationNanosecondBuilder::new())
            }
            _ => return Err(config("Arrow type outside the types of architecture 5.1")),
        })
    }

    /// Output bytes that a fixed-width value adds to the `batch_bytes` count (0 for variable-size types,
    /// which count the bytes they write).
    pub fn width(&self) -> u64 {
        match self {
            Self::I64(_)
            | Self::F64(_)
            | Self::TsS(_)
            | Self::TsU(_)
            | Self::TsN(_)
            | Self::DurU(_)
            | Self::DurN(_) => 8,
            Self::D128(_) | Self::Mdn(_) => 16,
            Self::D256(_) => 32,
            Self::Str(_) | Self::Bin(_) | Self::LStr(_) | Self::LBin(_) => 0,
        }
    }

    /// Appends a NULL. Bytes already written to a variable-size value stay under the null slot (the Arrow
    /// columnar format allows a null slot of positive length; architecture 5.6 step 6).
    pub fn append_null(&mut self) {
        match self {
            Self::I64(b) => b.append_null(),
            Self::D128(b) => b.append_null(),
            Self::D256(b) => b.append_null(),
            Self::F64(b) => b.append_null(),
            Self::Str(b) => b.append_null(),
            Self::Bin(b) => b.append_null(),
            Self::LStr(b) => b.append_null(),
            Self::LBin(b) => b.append_null(),
            Self::TsS(b) => b.append_null(),
            Self::TsU(b) => b.append_null(),
            Self::TsN(b) => b.append_null(),
            Self::Mdn(b) => b.append_null(),
            Self::DurU(b) => b.append_null(),
            Self::DurN(b) => b.append_null(),
        }
    }

    /// Appends an `i64` to an Int64, Timestamp or Duration column.
    pub fn append_i64(&mut self, v: i64) -> Result<()> {
        match self {
            Self::I64(b) => b.append_value(v),
            Self::TsS(b) => b.append_value(v),
            Self::TsU(b) => b.append_value(v),
            Self::TsN(b) => b.append_value(v),
            Self::DurU(b) => b.append_value(v),
            Self::DurN(b) => b.append_value(v),
            _ => return Err(config("builder does not take an i64")),
        }
        Ok(())
    }

    /// Appends an unscaled Decimal128 value.
    pub fn append_d128(&mut self, v: i128) -> Result<()> {
        match self {
            Self::D128(b) => b.append_value(v),
            _ => return Err(config("builder does not take a Decimal128")),
        }
        Ok(())
    }

    /// Appends an unscaled Decimal256 value.
    pub fn append_d256(&mut self, v: i256) -> Result<()> {
        match self {
            Self::D256(b) => b.append_value(v),
            _ => return Err(config("builder does not take a Decimal256")),
        }
        Ok(())
    }

    /// Appends an `f64`.
    pub fn append_f64(&mut self, v: f64) -> Result<()> {
        match self {
            Self::F64(b) => b.append_value(v),
            _ => return Err(config("builder does not take an f64")),
        }
        Ok(())
    }

    /// Appends an interval (months, days, nanoseconds).
    pub fn append_mdn(&mut self, months: i32, days: i32, nanos: i64) -> Result<()> {
        match self {
            Self::Mdn(b) => {
                b.append_value(IntervalMonthDayNanoType::make_value(months, days, nanos))
            }
            _ => return Err(config("builder does not take an interval")),
        }
        Ok(())
    }

    /// Writes a piece of text into the value in progress (`Utf8` and `LargeUtf8`).
    pub fn write_text(&mut self, text: &str) -> Result<()> {
        let r = match self {
            Self::Str(b) => b.write_str(text),
            Self::LStr(b) => b.write_str(text),
            _ => return Err(config("builder does not take text")),
        };
        r.map_err(|_| config("text builder refused a write"))
    }

    /// Writes a piece of bytes into the value in progress (`Binary` and `LargeBinary`).
    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let r = match self {
            Self::Bin(b) => b.write_all(bytes),
            Self::LBin(b) => b.write_all(bytes),
            _ => return Err(config("builder does not take bytes")),
        };
        r.map_err(|_| config("binary builder refused a write"))
    }

    /// Ends the value in progress (everything written since the last value or null).
    pub fn finish_value(&mut self) -> Result<()> {
        match self {
            Self::Str(b) => b.append_value(""),
            Self::LStr(b) => b.append_value(""),
            Self::Bin(b) => b.append_value(b""),
            Self::LBin(b) => b.append_value(b""),
            _ => return Err(config("builder has no value in pieces")),
        }
        Ok(())
    }

    /// Appends one complete text value.
    pub fn append_text(&mut self, text: &str) -> Result<()> {
        self.write_text(text)?;
        self.finish_value()
    }

    /// Appends one complete binary value.
    pub fn append_binary(&mut self, bytes: &[u8]) -> Result<()> {
        self.write_bytes(bytes)?;
        self.finish_value()
    }

    /// The finished array; the builder is empty again.
    pub fn finish(&mut self) -> ArrayRef {
        match self {
            Self::I64(b) => Arc::new(b.finish()),
            Self::D128(b) => Arc::new(b.finish()),
            Self::D256(b) => Arc::new(b.finish()),
            Self::F64(b) => Arc::new(b.finish()),
            Self::Str(b) => Arc::new(b.finish()),
            Self::Bin(b) => Arc::new(b.finish()),
            Self::LStr(b) => Arc::new(b.finish()),
            Self::LBin(b) => Arc::new(b.finish()),
            Self::TsS(b) => Arc::new(b.finish()),
            Self::TsU(b) => Arc::new(b.finish()),
            Self::TsN(b) => Arc::new(b.finish()),
            Self::Mdn(b) => Arc::new(b.finish()),
            Self::DurU(b) => Arc::new(b.finish()),
            Self::DurN(b) => Arc::new(b.finish()),
        }
    }
}

/// A schema field: every field is nullable (`fe ff` is NULL for every type [R3]); the metadata keys are
/// those of architecture 5.1.
pub fn field(name: &str, t: DataType, metadata: HashMap<String, String>) -> Field {
    Field::new(name, t, true).with_metadata(metadata)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::indexing_slicing)]
mod tests {
    use super::*;
    use arrow_array::cast::AsArray;
    use arrow_array::Array;

    /// ADR-18 / architecture 4.1 (e): written pieces form one value, `append_value("")` ends it, a null after
    /// a partial write keeps the bytes under the null slot.
    #[test]
    fn large_utf8_and_large_binary_take_pieces_on_the_pinned_arrow() {
        let mut s = ColBuilder::new(&DataType::LargeUtf8).unwrap();
        s.write_text("foo").unwrap();
        s.write_text("bar").unwrap();
        s.finish_value().unwrap();
        s.write_text("x").unwrap();
        s.append_null();
        s.append_text("v2").unwrap();
        let a = s.finish();
        let a = a.as_string::<i64>();
        assert_eq!(a.len(), 3);
        assert_eq!(a.value(0), "foobar");
        assert!(a.is_null(1));
        assert_eq!(a.value(2), "v2");
        // the partial byte stays under the null slot
        assert_eq!(a.value_offsets(), &[0, 6, 7, 9]);

        let mut b = ColBuilder::new(&DataType::LargeBinary).unwrap();
        b.write_bytes(&[1, 2]).unwrap();
        b.write_bytes(&[3]).unwrap();
        b.finish_value().unwrap();
        let a = b.finish();
        assert_eq!(a.as_binary::<i64>().value(0), &[1, 2, 3]);
    }
}
