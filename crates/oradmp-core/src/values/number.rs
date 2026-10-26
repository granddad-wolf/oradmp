//! NUMBER (type code 2) [Y1]: architecture 5.2 (decoder, renderings) and 5.2a (values that do not fit).
//!
//! [`OraNumber::decode`] validates the bytes exactly as [Y1] does and gives a lossless, fixed-size intermediate
//! (no heap): sign, up to 40 decimal digits and a power of ten. The renderings are pure functions of it:
//! Int64, Decimal128(P, S), Decimal256(P, S), Float64 and plain text. An `i128` or `i256` is built only after
//! the digit count has passed the fit test, so a 39- or 40-digit coefficient never enters integer arithmetic.

use arrow_buffer::i256;

use crate::error::{Error, ErrorCode, Result};

/// Largest number of significant decimal digits of a NUMBER (20 base-100 digit bytes) [Y1].
pub const MAX_DIGITS: usize = 40;

/// Largest precision of Decimal256 (`arrow_schema::DECIMAL256_MAX_PRECISION`).
pub const MAX_PRECISION_256: u8 = 76;

/// Largest precision of Decimal128.
pub const MAX_PRECISION_128: u8 = 38;

/// Longest plain-decimal text before the exponent form is used (architecture 5.2 step 4).
const MAX_PLAIN_TEXT: usize = 64;

fn bad_number(message: &'static str) -> Error {
    Error::new(ErrorCode::BadNumber, message)
}

fn overflow(message: &'static str) -> Error {
    Error::new(ErrorCode::NumberOverflow, message)
}

/// A decoded NUMBER: value = (-1)^negative x coefficient x 10^exp10. Zero has no digits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OraNumber {
    negative: bool,
    /// Decimal digits 0 to 9, most significant first; the first and the last digit are not 0.
    coeff: [u8; MAX_DIGITS],
    /// Number of digits in `coeff`; 0 only for the value zero.
    len: u8,
    exp10: i16,
}

impl OraNumber {
    /// Zero.
    pub const ZERO: Self = Self {
        negative: false,
        coeff: [0; MAX_DIGITS],
        len: 0,
        exp10: 0,
    };

    /// Builds a number from its parts; the digits must be 0 to 9 with a non-zero first and last digit and at
    /// most 40 of them. An empty digit string is zero. Returns `None` if a condition does not hold.
    pub fn from_parts(negative: bool, digits: &[u8], exp10: i16) -> Option<Self> {
        if digits.is_empty() {
            return Some(Self::ZERO);
        }
        if digits.len() > MAX_DIGITS
            || digits.iter().any(|&d| d > 9)
            || digits.first() == Some(&0)
            || digits.last() == Some(&0)
        {
            return None;
        }
        let mut coeff = [0u8; MAX_DIGITS];
        for (slot, &d) in coeff.iter_mut().zip(digits) {
            *slot = d;
        }
        Some(Self {
            negative,
            coeff,
            len: u8::try_from(digits.len()).ok()?,
            exp10,
        })
    }

    /// Decodes the value bytes `b` of one field (architecture 5.2 steps 1 to 3) and applies every validity check
    /// of [Y1]: length 1 to 21; a single byte must be `80`; positive digit bytes `01` to `64`, negative `02` to
    /// `65` with a closing `66` unless the length is 21; at least one digit; the first and the last base-100
    /// digit are not 0. The error is `E-BAD-NUMBER` without context.
    pub fn decode(b: &[u8]) -> Result<Self> {
        if b.is_empty() || b.len() > 21 {
            return Err(bad_number("NUMBER length is not 1 to 21"));
        }
        let Some((&first, rest)) = b.split_first() else {
            return Err(bad_number("NUMBER length is not 1 to 21"));
        };
        if rest.is_empty() {
            return if first == 0x80 {
                Ok(Self::ZERO)
            } else {
                Err(bad_number("single NUMBER byte other than 80"))
            };
        }
        let positive = first & 0x80 != 0;
        let (e, mantissa) = if positive {
            (i32::from(first).saturating_sub(0xC1), rest)
        } else {
            let m = match rest.split_last() {
                Some((&0x66, head)) => head,
                _ if b.len() == 21 => rest,
                _ => {
                    return Err(bad_number(
                        "negative NUMBER without closing 66 and shorter than 21",
                    ))
                }
            };
            (0x3E_i32.saturating_sub(i32::from(first)), m)
        };
        let n = mantissa.len();
        if n == 0 || n > 20 {
            return Err(bad_number(
                "NUMBER without a digit byte or with more than 20",
            ));
        }
        // Base-100 digits d[0..n) as two decimal digits each.
        let mut dec = [0u8; 2 * 20];
        let mut first_d = 0u8;
        let mut last_d = 0u8;
        let (pairs, _) = dec.as_chunks_mut::<2>();
        for (i, (&byte, pair)) in mantissa.iter().zip(pairs.iter_mut()).enumerate() {
            let d = if positive {
                if !(0x01..=0x64).contains(&byte) {
                    return Err(bad_number("positive NUMBER digit byte outside 01 to 64"));
                }
                byte.saturating_sub(1)
            } else {
                if !(0x02..=0x65).contains(&byte) {
                    return Err(bad_number("negative NUMBER digit byte outside 02 to 65"));
                }
                101u8.saturating_sub(byte)
            };
            if i == 0 {
                first_d = d;
            }
            last_d = d;
            *pair = [d / 10, d % 10];
        }
        if first_d == 0 || last_d == 0 {
            return Err(bad_number("NUMBER with a leading or trailing zero digit"));
        }
        let start = usize::from(first_d < 10);
        let trailing = last_d.is_multiple_of(10);
        let end = n.saturating_mul(2).saturating_sub(usize::from(trailing));
        let digits = dec.get(start..end).unwrap_or(&[]);
        let n_i32 = i32::try_from(n).unwrap_or(i32::MAX);
        let exp = e
            .saturating_sub(n_i32)
            .saturating_add(1)
            .saturating_mul(2)
            .saturating_add(i32::from(trailing));
        let exp10 = i16::try_from(exp).map_err(|_| bad_number("NUMBER exponent out of range"))?;
        let mut coeff = [0u8; MAX_DIGITS];
        for (slot, &d) in coeff.iter_mut().zip(digits) {
            *slot = d;
        }
        Ok(Self {
            negative: !positive,
            coeff,
            len: u8::try_from(digits.len()).unwrap_or(u8::MAX),
            exp10,
        })
    }

    /// True for the value zero.
    pub fn is_zero(&self) -> bool {
        self.len == 0
    }

    /// True for a negative value.
    pub fn is_negative(&self) -> bool {
        self.negative
    }

    /// The significant decimal digits, most significant first (empty for zero).
    pub fn digits(&self) -> &[u8] {
        self.coeff.get(..usize::from(self.len)).unwrap_or(&[])
    }

    /// The power of ten: value = coefficient x 10^exp10.
    pub fn exp10(&self) -> i16 {
        self.exp10
    }

    /// Significant digits (0 for zero).
    pub fn sig(&self) -> u32 {
        u32::from(self.len)
    }

    /// Fractional digits: max(0, -exp10).
    pub fn frac(&self) -> u32 {
        if self.is_zero() {
            0
        } else {
            u32::from(self.exp10.min(0).unsigned_abs())
        }
    }

    /// Integer digits: max(0, len + exp10).
    pub fn int_digits(&self) -> u32 {
        if self.is_zero() {
            return 0;
        }
        let v = i32::from(self.len).saturating_add(i32::from(self.exp10));
        u32::try_from(v).unwrap_or(0)
    }

    /// Position of the leading digit: exp10 + len - 1 (None for zero).
    pub fn leading_position(&self) -> Option<i32> {
        if self.is_zero() {
            None
        } else {
            Some(
                i32::from(self.exp10)
                    .saturating_add(i32::from(self.len))
                    .saturating_sub(1),
            )
        }
    }

    /// True if the value is an integer (zero included).
    pub fn is_integer(&self) -> bool {
        self.is_zero() || self.exp10 >= 0
    }

    /// Int64 rendering: `Some` iff the value is an integer inside the range of `i64` (compared on the digits).
    pub fn to_i64(&self) -> Option<i64> {
        if self.is_zero() {
            return Some(0);
        }
        if self.exp10 < 0 || self.int_digits() > 19 {
            return None;
        }
        let mut mag: u128 = 0;
        for &d in self.digits() {
            mag = mag.checked_mul(10)?.checked_add(u128::from(d))?;
        }
        let zeros = u32::try_from(self.exp10).ok()?;
        mag = mag.checked_mul(10u128.checked_pow(zeros)?)?;
        if self.negative {
            if mag <= 1u128 << 63 {
                // -2^63 included
                i64::try_from(i128::try_from(mag).ok()?.checked_neg()?).ok()
            } else {
                None
            }
        } else {
            i64::try_from(mag).ok()
        }
    }

    /// The unscaled digits of the value at scale `scale` under `policy` (architecture 5.2 step 4, 5.2a).
    /// `Ok(None)` means: not exact, policy `Null`.
    fn scaled(&self, precision: u8, scale: u8, policy: OverflowPolicy) -> Result<Option<Scaled>> {
        if precision > MAX_PRECISION_256 {
            return Err(Error::new(ErrorCode::Config, "decimal precision above 76"));
        }
        let mut out = Scaled {
            negative: self.negative,
            ..Scaled::default()
        };
        if self.is_zero() {
            return Ok(Some(out));
        }
        let len = i32::from(self.len);
        let exp = i32::from(self.exp10);
        let p = i32::from(precision);
        let s = i32::from(scale);
        let int_digits = len.saturating_add(exp).max(0);
        if int_digits > p.saturating_sub(s) {
            return Err(overflow("integer part does not fit the precision"));
        }
        let k = exp.saturating_add(s);
        if k >= 0 {
            let n = usize::try_from(len.saturating_add(k)).unwrap_or(usize::MAX);
            if n > usize::from(precision) {
                return Err(overflow("integer part does not fit the precision"));
            }
            // digits, then k zeros (the buffer is zero-filled)
            for (slot, &d) in out.digits.iter_mut().zip(self.digits()) {
                *slot = d;
            }
            out.n = n;
            return Ok(Some(out));
        }
        let drop = k.saturating_neg();
        match policy {
            OverflowPolicy::Error => {
                return Err(overflow("fraction digits beyond the scale (policy error)"))
            }
            OverflowPolicy::Null => return Ok(None),
            OverflowPolicy::Round => {}
        }
        let keep = len.saturating_sub(drop);
        let dropped = u16::try_from(drop).unwrap_or(u16::MAX);
        let round_up = match usize::try_from(keep) {
            Ok(i) => self.coeff.get(i).copied().unwrap_or(0) >= 5,
            Err(_) => false,
        };
        let keep_n = usize::try_from(keep).unwrap_or(0);
        for (slot, &d) in out.digits.iter_mut().zip(self.digits().iter().take(keep_n)) {
            *slot = d;
        }
        out.n = keep_n;
        if round_up {
            // half away from zero on the magnitude; the carry may add a digit
            let mut carry = true;
            for slot in out.digits.iter_mut().take(keep_n).rev() {
                if *slot == 9 {
                    *slot = 0;
                } else {
                    *slot = slot.saturating_add(1);
                    carry = false;
                    break;
                }
            }
            if carry {
                // all kept digits were 9 (or none were kept): the result is 10^keep_n
                out.digits.fill(0);
                if let Some(first) = out.digits.first_mut() {
                    *first = 1;
                }
                out.n = keep_n.saturating_add(1);
            }
            if out.n > usize::from(precision) {
                return Err(overflow(
                    "integer part does not fit the precision after rounding",
                ));
            }
        }
        out.rounded = Some(Rounding {
            to_zero: out.n == 0,
            dropped,
        });
        Ok(Some(out))
    }

    /// Decimal128(`precision`, `scale`) rendering: the unscaled value, exact, rounded or nulled by `policy`.
    /// `precision` is 1 to 38. An integer part that does not fit is `E-NUMBER-OVERFLOW` under every policy.
    pub fn to_decimal128(
        &self,
        precision: u8,
        scale: u8,
        policy: OverflowPolicy,
    ) -> Result<Fitted<i128>> {
        if precision > MAX_PRECISION_128 {
            return Err(Error::new(
                ErrorCode::Config,
                "Decimal128 precision above 38",
            ));
        }
        let Some(sc) = self.scaled(precision, scale, policy)? else {
            return Ok(Fitted::Nulled);
        };
        let mut v: i128 = 0;
        for &d in sc.digits.iter().take(sc.n) {
            v = v
                .checked_mul(10)
                .and_then(|x| x.checked_add(i128::from(d)))
                .ok_or_else(|| overflow("Decimal128 value out of range"))?;
        }
        if sc.negative {
            v = v
                .checked_neg()
                .ok_or_else(|| overflow("Decimal128 value out of range"))?;
        }
        Ok(sc.fitted(v))
    }

    /// Decimal256(`precision`, `scale`) rendering, `precision` 1 to 76 (architecture 5.2 step 4).
    pub fn to_decimal256(
        &self,
        precision: u8,
        scale: u8,
        policy: OverflowPolicy,
    ) -> Result<Fitted<i256>> {
        let Some(sc) = self.scaled(precision, scale, policy)? else {
            return Ok(Fitted::Nulled);
        };
        let ten = i256::from_i128(10);
        let mut v = i256::ZERO;
        for &d in sc.digits.iter().take(sc.n) {
            v = v
                .checked_mul(ten)
                .and_then(|x| x.checked_add(i256::from_i128(i128::from(d))))
                .ok_or_else(|| overflow("Decimal256 value out of range"))?;
        }
        if sc.negative {
            v = v
                .checked_neg()
                .ok_or_else(|| overflow("Decimal256 value out of range"))?;
        }
        Ok(sc.fitted(v))
    }

    /// Float64 rendering: the nearest double of the text `[-]<digits>e<exp10>` (`str::parse::<f64>`).
    /// Every NUMBER of [Y1] lies inside the normal range of `f64`; a failure is `E-NUMBER-OVERFLOW`.
    pub fn to_f64(&self) -> Result<f64> {
        if self.is_zero() {
            return Ok(0.0);
        }
        let mut t = String::with_capacity(MAX_DIGITS + 8);
        if self.negative {
            t.push('-');
        }
        for &d in self.digits() {
            t.push(char::from(b'0'.saturating_add(d)));
        }
        t.push('e');
        t.push_str(&self.exp10.to_string());
        match t.parse::<f64>() {
            Ok(v) if v.is_finite() => Ok(v),
            _ => Err(overflow("NUMBER outside the range of Float64")),
        }
    }

    /// Text rendering: plain decimal notation (`0.001`, `-1000000`) when it has at most 64 characters, else
    /// `<d>.<ddd>E<+|-><n>`. Lossless (architecture 5.2 step 4).
    pub fn to_text(&self) -> String {
        if self.is_zero() {
            return "0".to_string();
        }
        let len = i64::from(self.len);
        let exp = i64::from(self.exp10);
        let sign = usize::from(self.negative);
        let plain_len = if exp >= 0 {
            len.saturating_add(exp)
        } else if len.saturating_add(exp) > 0 {
            len.saturating_add(1)
        } else {
            2i64.saturating_sub(exp)
        };
        let mut t = String::new();
        if self.negative {
            t.push('-');
        }
        let digit = |d: u8| char::from(b'0'.saturating_add(d));
        let plain_ok = usize::try_from(plain_len)
            .map(|l| l.saturating_add(sign) <= MAX_PLAIN_TEXT)
            .unwrap_or(false);
        if plain_ok {
            if exp >= 0 {
                t.extend(self.digits().iter().map(|&d| digit(d)));
                for _ in 0..exp {
                    t.push('0');
                }
            } else {
                let point = len.saturating_add(exp);
                if point > 0 {
                    let at = usize::try_from(point).unwrap_or(0);
                    for (i, &d) in self.digits().iter().enumerate() {
                        if i == at {
                            t.push('.');
                        }
                        t.push(digit(d));
                    }
                } else {
                    t.push_str("0.");
                    for _ in 0..point.unsigned_abs() {
                        t.push('0');
                    }
                    t.extend(self.digits().iter().map(|&d| digit(d)));
                }
            }
        } else {
            let mut it = self.digits().iter();
            if let Some(&d) = it.next() {
                t.push(digit(d));
            }
            if self.len > 1 {
                t.push('.');
                t.extend(it.map(|&d| digit(d)));
            }
            let e = exp.saturating_add(len).saturating_sub(1);
            t.push('E');
            t.push(if e < 0 { '-' } else { '+' });
            t.push_str(&e.unsigned_abs().to_string());
        }
        t
    }
}

/// What to do with a NUMBER that cannot be stored exactly in a decimal type (architecture 5.2a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowPolicy {
    /// A fraction beyond the scale is a value error.
    Error,
    /// A fraction beyond the scale is rounded half away from zero (owner decision Q-1, 2026-10-16).
    Round,
    /// A fraction beyond the scale makes the value NULL (counted by the caller).
    Null,
}

/// The result of a decimal rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fitted<T> {
    /// The unscaled value, exact.
    Exact(T),
    /// The unscaled value after rounding; `to_zero`: a non-zero value became 0; `dropped`: digits below the
    /// scale (the number of fractional digits that did not fit).
    Rounded {
        /// The unscaled value.
        value: T,
        /// The value was not zero and became zero.
        to_zero: bool,
        /// Digits beyond the scale.
        dropped: u16,
    },
    /// Not exact and the policy is `Null`.
    Nulled,
}

impl<T: Copy> Fitted<T> {
    /// The unscaled value, if any.
    pub fn value(&self) -> Option<T> {
        match *self {
            Self::Exact(v) | Self::Rounded { value: v, .. } => Some(v),
            Self::Nulled => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rounding {
    to_zero: bool,
    dropped: u16,
}

#[derive(Debug, Clone, Copy)]
struct Scaled {
    /// Unscaled digits, `n` of them are used; `digits` beyond the used part are 0.
    digits: [u8; 80],
    n: usize,
    negative: bool,
    rounded: Option<Rounding>,
}

impl Default for Scaled {
    fn default() -> Self {
        Self {
            digits: [0; 80],
            n: 0,
            negative: false,
            rounded: None,
        }
    }
}

impl Scaled {
    fn fitted<T>(&self, value: T) -> Fitted<T> {
        match self.rounded {
            None => Fitted::Exact(value),
            Some(r) => Fitted::Rounded {
                value,
                to_zero: r.to_zero,
                dropped: r.dropped,
            },
        }
    }
}
