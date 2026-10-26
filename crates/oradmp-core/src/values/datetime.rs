//! DATE (12), TIMESTAMP (180), TIMESTAMP WITH TIME ZONE (181) and WITH LOCAL TIME ZONE (231):
//! rules [Y2, Y5, Y7, Y8], architecture 5.3 and 5.4.
//!
//! The civil fields are taken as the proleptic Gregorian calendar and converted to seconds since
//! 1970-01-01 00:00:00 ("what Oracle prints is what Arrow prints"). A year below 1 decodes to fields but has no
//! Arrow value (architecture 5.3): [`DateTime::epoch_seconds`] returns `E-BAD-DATETIME` for it, as it does for a
//! nanosecond value outside the 64-bit range.

use crate::error::{Error, ErrorCode, Result};

fn bad(message: &'static str) -> Error {
    Error::new(ErrorCode::BadDatetime, message)
}

/// Civil date and time of day as stored (no zone). `year` is negative before year 1 (there is no year 0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTime {
    /// Year, -4712 to 9999 (never 0).
    pub year: i32,
    /// Month 1 to 12.
    pub month: u8,
    /// Day 1 to 31 (within the month).
    pub day: u8,
    /// Hour 0 to 23.
    pub hour: u8,
    /// Minute 0 to 59.
    pub minute: u8,
    /// Second 0 to 59.
    pub second: u8,
}

fn is_leap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
    }
}

impl DateTime {
    /// A date and time from fields; the fields must be a valid civil date (year 1 to 9999 or a negative year;
    /// for a negative year the day is only checked against 31).
    pub fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Result<Self> {
        if year == 0 {
            return Err(bad("year 0 does not exist"));
        }
        if !(1..=12).contains(&month) {
            return Err(bad("month outside 1 to 12"));
        }
        let max_day = if year > 0 {
            days_in_month(year, month)
        } else {
            31
        };
        if day < 1 || day > max_day {
            return Err(bad("day outside the month"));
        }
        if hour > 23 || minute > 59 || second > 59 {
            return Err(bad("time of day outside 00:00:00 to 23:59:59"));
        }
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    /// Seconds since 1970-01-01 00:00:00 of these fields. A year below 1 is `E-BAD-DATETIME`: its Arrow value
    /// needs a statement about year numbering that no fixture gives (architecture 5.3).
    pub fn epoch_seconds(&self) -> Result<i64> {
        if self.year < 1 {
            return Err(bad("year below 1 has no Arrow value"));
        }
        // days from civil (proleptic Gregorian), years 1 to 9999
        let m = i64::from(self.month);
        let y = i64::from(self.year).saturating_sub(i64::from(self.month <= 2));
        let era = y / 400;
        let yoe = y.saturating_sub(era.saturating_mul(400));
        let mp = if m > 2 {
            m.saturating_sub(3)
        } else {
            m.saturating_add(9)
        };
        let doy = (153i64.saturating_mul(mp).saturating_add(2) / 5)
            .saturating_add(i64::from(self.day))
            .saturating_sub(1);
        let doe = yoe
            .saturating_mul(365)
            .saturating_add(yoe / 4)
            .saturating_sub(yoe / 100)
            .saturating_add(doy);
        let days = era
            .saturating_mul(146_097)
            .saturating_add(doe)
            .saturating_sub(719_468);
        let secs = i64::from(self.hour)
            .saturating_mul(3600)
            .saturating_add(i64::from(self.minute).saturating_mul(60))
            .saturating_add(i64::from(self.second));
        Ok(days.saturating_mul(86_400).saturating_add(secs))
    }
}

/// Decodes the first seven bytes of a DATE-layout value [Y2]: century + 100, year in century + 100, month,
/// day, hour + 1, minute + 1, second + 1; for years before 1 the first two bytes are 100 - century and
/// 100 - year. `b` must hold at least 7 bytes (the caller has checked the length).
fn date_fields(b: &[u8]) -> Result<DateTime> {
    let Some(&[c, y, month, day, h, mi, s]) = b.first_chunk::<7>() else {
        return Err(bad("date value shorter than 7 bytes"));
    };
    let year = if c >= 100 && y >= 100 {
        i32::from(c.saturating_sub(100))
            .saturating_mul(100)
            .saturating_add(i32::from(y.saturating_sub(100)))
    } else if (1..=100).contains(&c) && (1..=100).contains(&y) {
        0i32.saturating_sub(
            i32::from(100u8.saturating_sub(c))
                .saturating_mul(100)
                .saturating_add(i32::from(100u8.saturating_sub(y))),
        )
    } else {
        return Err(bad("century or year byte outside the DATE layout"));
    };
    if h == 0 || mi == 0 || s == 0 {
        return Err(bad(
            "time byte of 0 (the bytes are hour + 1, minute + 1, second + 1)",
        ));
    }
    DateTime::new(
        year,
        month,
        day,
        h.saturating_sub(1),
        mi.saturating_sub(1),
        s.saturating_sub(1),
    )
}

/// Decodes a DATE value (12): exactly 7 bytes [Y2]. Returns the civil fields (a year below 1 is returned as
/// such).
pub fn decode_date_fields(b: &[u8]) -> Result<DateTime> {
    if b.len() != 7 {
        return Err(bad("DATE length is not 7"));
    }
    date_fields(b)
}

/// Decodes a DATE value to seconds since the epoch (Arrow Timestamp(Second)); year below 1 is an error.
pub fn decode_date(b: &[u8]) -> Result<i64> {
    decode_date_fields(b)?.epoch_seconds()
}

/// An instant: seconds since the epoch and nanoseconds within the second.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp {
    /// Seconds since 1970-01-01 00:00:00.
    pub secs: i64,
    /// Nanoseconds, 0 to 999,999,999.
    pub nanos: u32,
}

impl Timestamp {
    /// Microseconds since the epoch, exact: a non-zero sub-microsecond part is `E-BAD-DATETIME`.
    pub fn to_micros(&self) -> Result<i64> {
        if !self.nanos.is_multiple_of(1000) {
            return Err(bad("fraction finer than a microsecond"));
        }
        self.secs
            .checked_mul(1_000_000)
            .and_then(|s| s.checked_add(i64::from(self.nanos / 1000)))
            .ok_or_else(|| bad("timestamp outside the 64-bit microsecond range"))
    }

    /// Nanoseconds since the epoch; outside the 64-bit range (about 1677 to 2262) is `E-BAD-DATETIME`.
    pub fn to_nanos(&self) -> Result<i64> {
        self.secs
            .checked_mul(1_000_000_000)
            .and_then(|s| s.checked_add(i64::from(self.nanos)))
            .ok_or_else(|| bad("timestamp outside the 64-bit nanosecond range"))
    }
}

fn nanos_at(b: &[u8], at: usize) -> Result<u32> {
    let chunk = b
        .get(at..)
        .and_then(|t| t.first_chunk::<4>())
        .ok_or_else(|| bad("fraction bytes missing"))?;
    let n = u32::from_be_bytes(*chunk);
    if n > 999_999_999 {
        return Err(bad("fraction above 999,999,999 ns"));
    }
    Ok(n)
}

/// Decodes a TIMESTAMP value (180): 7 bytes (fraction zero) or 11 bytes (DATE layout + u32 BE nanoseconds)
/// [Y5].
pub fn decode_timestamp(b: &[u8]) -> Result<Timestamp> {
    let nanos = match b.len() {
        7 => 0,
        11 => nanos_at(b, 7)?,
        _ => return Err(bad("TIMESTAMP length is not 7 or 11")),
    };
    Ok(Timestamp {
        secs: date_fields(b)?.epoch_seconds()?,
        nanos,
    })
}

/// The zone of a TIMESTAMP WITH TIME ZONE value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    /// A fixed offset from UTC in minutes (hour byte - 20, minute byte - 60) [Y7].
    Offset(i16),
    /// A region (bit 0x80 of the first zone byte set): the two bytes, meaning unknown [Y7].
    Region(u16),
}

/// A TIMESTAMP WITH TIME ZONE value: the instant (the stored fields are UTC) and the zone bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimestampTz {
    /// The instant.
    pub instant: Timestamp,
    /// The zone of the value; the Arrow value does not deliver it (ADR-7).
    pub zone: Zone,
}

/// Decodes a TIMESTAMP WITH TIME ZONE value (181): 13 bytes, DATE layout in UTC, u32 BE nanoseconds and two
/// zone bytes [Y7].
pub fn decode_tstz(b: &[u8]) -> Result<TimestampTz> {
    if b.len() != 13 {
        return Err(bad("TIMESTAMP WITH TIME ZONE length is not 13"));
    }
    let instant = Timestamp {
        secs: date_fields(b)?.epoch_seconds()?,
        nanos: nanos_at(b, 7)?,
    };
    let &[z0, z1] = b
        .get(11..)
        .and_then(|t| t.first_chunk::<2>())
        .ok_or_else(|| bad("zone bytes missing"))?;
    let zone = if z0 & 0x80 != 0 {
        Zone::Region(u16::from_be_bytes([z0, z1]))
    } else {
        let hour = i16::from(z0).saturating_sub(20);
        let minute = i16::from(z1).saturating_sub(60);
        if !(-23..=23).contains(&hour) || !(-59..=59).contains(&minute) {
            return Err(bad("zone offset outside the plausible range"));
        }
        Zone::Offset(hour.saturating_mul(60).saturating_add(minute))
    };
    Ok(TimestampTz { instant, zone })
}

/// Decodes a TIMESTAMP WITH LOCAL TIME ZONE value (231): the TIMESTAMP layout [Y8, Y5]; 7 bytes (a stored fraction
/// of zero, in any precision) and 11 bytes are both confirmed (`tsltz2_exp_<v>`, ADR-100 item 1), so no event.
/// Returns the stored local time as written (whether it is UTC or the database zone is unknown, R-019; `W-TSLTZ`
/// says so on the column).
pub fn decode_tsltz(b: &[u8]) -> Result<Timestamp> {
    decode_timestamp(b)
}
