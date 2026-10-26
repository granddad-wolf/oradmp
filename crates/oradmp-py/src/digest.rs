//! A canonical digest of the batches of a table, computed in Rust from the core's own batches (no pyarrow): the
//! pytest files compute the same digest from the pyarrow tables and compare, which proves that the hand-over
//! (schema, null bitmaps, buffers, offsets) kept every cell.
//!
//! Per column and cell: a byte 0 for NULL, else a byte 1 and the value: 8 bytes little-endian for Int64,
//! Timestamp and Duration; the IEEE bits for Float64; 16 / 32 bytes for Decimal128 / Decimal256; months (i32),
//! days (i32), nanoseconds (i64) for Interval(MonthDayNano); a u64 length and the bytes for text and binary. FNV-1a
//! over that stream, one digest per column.

use arrow_array::cast::AsArray;
use arrow_array::types::{
    Decimal128Type, Decimal256Type, DurationMicrosecondType, DurationNanosecondType, Float64Type,
    Int64Type, IntervalMonthDayNanoType, TimestampMicrosecondType, TimestampNanosecondType,
    TimestampSecondType,
};
use arrow_array::{Array, RecordBatch};
use arrow_schema::{DataType, IntervalUnit, TimeUnit};

const BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0000_0100_0000_01b3;

pub struct ColumnDigest {
    pub name: String,
    pub rows: u64,
    pub nulls: u64,
    pub digest: u64,
}

fn feed(h: &mut u64, bytes: &[u8]) {
    for b in bytes {
        *h ^= u64::from(*b);
        *h = h.wrapping_mul(PRIME);
    }
}

/// The digests of the columns over all batches.
pub fn digest(batches: &[RecordBatch], names: &[String]) -> Vec<ColumnDigest> {
    let mut out: Vec<ColumnDigest> = names
        .iter()
        .map(|n| ColumnDigest {
            name: n.clone(),
            rows: 0,
            nulls: 0,
            digest: BASIS,
        })
        .collect();
    for b in batches {
        for (c, col) in out.iter_mut().zip(b.columns()) {
            add(c, col.as_ref());
        }
    }
    out
}

fn add(c: &mut ColumnDigest, a: &dyn Array) {
    let mut h = c.digest;
    for i in 0..a.len() {
        c.rows = c.rows.saturating_add(1);
        if a.is_null(i) {
            c.nulls = c.nulls.saturating_add(1);
            feed(&mut h, &[0]);
            continue;
        }
        feed(&mut h, &[1]);
        match a.data_type() {
            DataType::Int64 => feed(
                &mut h,
                &a.as_primitive::<Int64Type>().value(i).to_le_bytes(),
            ),
            DataType::Float64 => feed(
                &mut h,
                &a.as_primitive::<Float64Type>()
                    .value(i)
                    .to_bits()
                    .to_le_bytes(),
            ),
            DataType::Decimal128(..) => feed(
                &mut h,
                &a.as_primitive::<Decimal128Type>().value(i).to_le_bytes(),
            ),
            DataType::Decimal256(..) => feed(
                &mut h,
                &a.as_primitive::<Decimal256Type>().value(i).to_le_bytes(),
            ),
            DataType::Timestamp(TimeUnit::Second, _) => feed(
                &mut h,
                &a.as_primitive::<TimestampSecondType>()
                    .value(i)
                    .to_le_bytes(),
            ),
            DataType::Timestamp(TimeUnit::Microsecond, _) => feed(
                &mut h,
                &a.as_primitive::<TimestampMicrosecondType>()
                    .value(i)
                    .to_le_bytes(),
            ),
            DataType::Timestamp(TimeUnit::Nanosecond, _) => feed(
                &mut h,
                &a.as_primitive::<TimestampNanosecondType>()
                    .value(i)
                    .to_le_bytes(),
            ),
            DataType::Duration(TimeUnit::Microsecond) => feed(
                &mut h,
                &a.as_primitive::<DurationMicrosecondType>()
                    .value(i)
                    .to_le_bytes(),
            ),
            DataType::Duration(TimeUnit::Nanosecond) => feed(
                &mut h,
                &a.as_primitive::<DurationNanosecondType>()
                    .value(i)
                    .to_le_bytes(),
            ),
            DataType::Interval(IntervalUnit::MonthDayNano) => {
                let v = a.as_primitive::<IntervalMonthDayNanoType>().value(i);
                feed(&mut h, &v.months.to_le_bytes());
                feed(&mut h, &v.days.to_le_bytes());
                feed(&mut h, &v.nanoseconds.to_le_bytes());
            }
            DataType::Utf8 => text(&mut h, a.as_string::<i32>().value(i).as_bytes()),
            DataType::LargeUtf8 => text(&mut h, a.as_string::<i64>().value(i).as_bytes()),
            DataType::Binary => text(&mut h, a.as_binary::<i32>().value(i)),
            DataType::LargeBinary => text(&mut h, a.as_binary::<i64>().value(i)),
            _ => feed(&mut h, b"?"),
        }
    }
    c.digest = h;
}

fn text(h: &mut u64, bytes: &[u8]) {
    feed(
        h,
        &u64::try_from(bytes.len()).unwrap_or(u64::MAX).to_le_bytes(),
    );
    feed(h, bytes);
}
