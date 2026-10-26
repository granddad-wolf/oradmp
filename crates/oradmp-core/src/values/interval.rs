//! INTERVAL YEAR TO MONTH (182) and INTERVAL DAY TO SECOND (183): rule [Y10], architecture 5.7, ADR-19.
//!
//! Both are ordinary fields whose bytes are the output of `DUMP`: signed values stored with an offset. Every
//! field of a negative interval is negative (zero fields may stand beside them) [Y10]; a value with a field
//! above 0 and another below 0 breaks the confirmed form and is `E-BAD-INTERVAL`.

use crate::error::{Error, ErrorCode, Result};

fn bad(message: &'static str) -> Error {
    Error::new(ErrorCode::BadInterval, message)
}

/// Largest absolute value of the years or days field (leading precision 9) [Y10].
const MAX_LEAD: i64 = 999_999_999;

fn lead_field(b: &[u8; 4]) -> Result<i64> {
    let v = i64::from(u32::from_be_bytes(*b)).saturating_sub(0x8000_0000);
    if v.abs() > MAX_LEAD {
        return Err(bad("years or days outside +-999,999,999"));
    }
    Ok(v)
}

fn small_field(byte: u8, lo: i32, hi: i32) -> Result<i32> {
    let v = i32::from(byte).saturating_sub(60);
    if v < lo || v > hi {
        return Err(bad("field of an interval outside its range"));
    }
    Ok(v)
}

fn same_sign(values: &[i64]) -> bool {
    !(values.iter().any(|&v| v > 0) && values.iter().any(|&v| v < 0))
}

/// INTERVAL YEAR TO MONTH: years and months, both of the sign of the interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalYm {
    /// Years, -999,999,999 to 999,999,999.
    pub years: i64,
    /// Months, -11 to 11.
    pub months: i32,
}

impl IntervalYm {
    /// Decodes a value of 5 bytes: years = u32 BE b[0..4] - 0x80000000, months = b[4] - 60.
    pub fn decode(b: &[u8]) -> Result<Self> {
        let Some(&[y0, y1, y2, y3, m]) = b.first_chunk::<5>().filter(|_| b.len() == 5) else {
            return Err(bad("INTERVAL YEAR TO MONTH length is not 5"));
        };
        let years = lead_field(&[y0, y1, y2, y3])?;
        let months = small_field(m, -11, 11)?;
        if !same_sign(&[years, i64::from(months)]) {
            return Err(bad("fields of opposite signs"));
        }
        Ok(Self { years, months })
    }

    /// 12 x years + months as an `i32` (the Arrow Interval(MonthDayNano) months field); a larger value is
    /// `E-BAD-INTERVAL` (possible only with a leading precision of 9, ADR-19).
    pub fn total_months(&self) -> Result<i32> {
        self.years
            .checked_mul(12)
            .and_then(|m| m.checked_add(i64::from(self.months)))
            .and_then(|m| i32::try_from(m).ok())
            .ok_or_else(|| bad("months do not fit 32 bits"))
    }

    /// The `TO_CHAR` text of [Y10] with the leading precision `p` (the digits of the years field, zero
    /// padded): `+0002-06`, `-0003-02`. Used by tests and by `interval_as="string"`.
    pub fn to_text(&self, p: usize) -> String {
        let neg = self.years < 0 || self.months < 0;
        format!(
            "{}{:0width$}-{:02}",
            if neg { '-' } else { '+' },
            self.years.unsigned_abs(),
            self.months.unsigned_abs(),
            width = p
        )
    }
}

/// INTERVAL DAY TO SECOND: five signed fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntervalDs {
    /// Days, -999,999,999 to 999,999,999.
    pub days: i64,
    /// Hours, -23 to 23.
    pub hours: i32,
    /// Minutes, -59 to 59.
    pub minutes: i32,
    /// Seconds, -59 to 59.
    pub seconds: i32,
    /// Nanoseconds, -999,999,999 to 999,999,999.
    pub nanos: i32,
}

impl IntervalDs {
    /// Decodes a value of 11 bytes: days = u32 BE b[0..4] - 0x80000000, hours, minutes, seconds = b[4], b[5],
    /// b[6] - 60, nanoseconds = u32 BE b[7..11] - 0x80000000.
    pub fn decode(b: &[u8]) -> Result<Self> {
        let Some(&[d0, d1, d2, d3, h, mi, s, n0, n1, n2, n3]) =
            b.first_chunk::<11>().filter(|_| b.len() == 11)
        else {
            return Err(bad("INTERVAL DAY TO SECOND length is not 11"));
        };
        let days = lead_field(&[d0, d1, d2, d3])?;
        let hours = small_field(h, -23, 23)?;
        let minutes = small_field(mi, -59, 59)?;
        let seconds = small_field(s, -59, 59)?;
        let ns = i64::from(u32::from_be_bytes([n0, n1, n2, n3])).saturating_sub(0x8000_0000);
        if ns.abs() > 999_999_999 {
            return Err(bad("nanoseconds outside +-999,999,999"));
        }
        let nanos = i32::try_from(ns).map_err(|_| bad("nanoseconds outside +-999,999,999"))?;
        if !same_sign(&[
            days,
            i64::from(hours),
            i64::from(minutes),
            i64::from(seconds),
            ns,
        ]) {
            return Err(bad("fields of opposite signs"));
        }
        Ok(Self {
            days,
            hours,
            minutes,
            seconds,
            nanos,
        })
    }

    /// The signed sum of the hours, minutes, seconds and nanoseconds, in nanoseconds (below 86,400 x 10^9 in
    /// magnitude).
    pub fn time_nanos(&self) -> i64 {
        i64::from(self.hours)
            .saturating_mul(3600)
            .saturating_add(i64::from(self.minutes).saturating_mul(60))
            .saturating_add(i64::from(self.seconds))
            .saturating_mul(1_000_000_000)
            .saturating_add(i64::from(self.nanos))
    }

    /// The whole interval in nanoseconds (`i128`: 999,999,999 days are 8.6 x 10^22 ns).
    pub fn total_nanos(&self) -> i128 {
        i128::from(self.days)
            .saturating_mul(86_400_000_000_000)
            .saturating_add(i128::from(self.time_nanos()))
    }

    /// Duration(Microsecond): exact, else `E-BAD-INTERVAL` (a non-zero sub-microsecond part, or `i64`
    /// overflow; about +-106,751,991 days).
    pub fn to_duration_micros(&self) -> Result<i64> {
        let n = self.total_nanos();
        if n % 1000 != 0 {
            return Err(bad("sub-microsecond part under Duration(Microsecond)"));
        }
        i64::try_from(n / 1000).map_err(|_| bad("interval outside the Duration(Microsecond) range"))
    }

    /// Duration(Nanosecond): exact, else `E-BAD-INTERVAL` (about +-106,751 days).
    pub fn to_duration_nanos(&self) -> Result<i64> {
        i64::try_from(self.total_nanos())
            .map_err(|_| bad("interval outside the Duration(Nanosecond) range"))
    }

    /// Interval(MonthDayNano) with months 0: (days, nanoseconds of the day) (ADR-19, session 4).
    pub fn to_month_day_nano(&self) -> Result<(i32, i64)> {
        let days = i32::try_from(self.days).map_err(|_| bad("days do not fit 32 bits"))?;
        Ok((days, self.time_nanos()))
    }

    /// The `TO_CHAR` text of [Y10]: sign `-` if any field is negative, days with `p` digits, ` HH:MI:SS`, and
    /// `.` + the first `f` of the 9 nanosecond digits when `f > 0`.
    pub fn to_text(&self, p: usize, f: usize) -> String {
        let neg = self.days < 0
            || self.hours < 0
            || self.minutes < 0
            || self.seconds < 0
            || self.nanos < 0;
        let mut t = format!(
            "{}{:0width$} {:02}:{:02}:{:02}",
            if neg { '-' } else { '+' },
            self.days.unsigned_abs(),
            self.hours.unsigned_abs(),
            self.minutes.unsigned_abs(),
            self.seconds.unsigned_abs(),
            width = p
        );
        if f > 0 {
            let digits = format!("{:09}", self.nanos.unsigned_abs());
            t.push('.');
            t.push_str(digits.get(..f.min(9)).unwrap_or(""));
        }
        t
    }
}
