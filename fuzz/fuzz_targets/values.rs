#![no_main]
//! Target `values`: the first byte selects a value decoder, the rest is the field (NUMBER, DATE, TIMESTAMP
//! 180 / 181 / 231, INTERVAL 182 / 183, ROWID). Every conversion of a decoded value is called too.

use libfuzzer_sys::fuzz_target;
use oradmp_core::values::datetime::{decode_date, decode_timestamp, decode_tsltz, decode_tstz};
use oradmp_core::values::interval::{IntervalDs, IntervalYm};
use oradmp_core::values::number::{OraNumber, OverflowPolicy};
use oradmp_core::values::rowid::{decode_rowid, decode_urowid};
use oradmp_fuzz::checked;

fuzz_target!(|data: &[u8]| {
    let Some((&which, field)) = data.split_first() else {
        return;
    };
    // A decoder sees no file: there is no offset, the length bound only keeps the check meaningful.
    let len = u64::MAX;
    match which % 9 {
        0 => {
            if let Some(n) = checked(OraNumber::decode(field), len) {
                let _ = n.to_text();
                let _ = n.to_i64();
                checked(n.to_f64(), len);
                for policy in [
                    OverflowPolicy::Error,
                    OverflowPolicy::Round,
                    OverflowPolicy::Null,
                ] {
                    checked(n.to_decimal128(38, 10, policy), len);
                    checked(n.to_decimal128(10, 0, policy), len);
                    checked(n.to_decimal256(76, 38, policy), len);
                }
            }
        }
        1 => {
            checked(decode_date(field), len);
        }
        2 => {
            if let Some(t) = checked(decode_timestamp(field), len) {
                checked(t.to_micros(), len);
                checked(t.to_nanos(), len);
            }
        }
        3 => {
            if let Some(t) = checked(decode_tstz(field), len) {
                checked(t.instant.to_micros(), len);
                checked(t.instant.to_nanos(), len);
            }
        }
        4 => {
            if let Some(t) = checked(decode_tsltz(field), len) {
                checked(t.to_micros(), len);
                checked(t.to_nanos(), len);
            }
        }
        5 => {
            if let Some(v) = checked(IntervalYm::decode(field), len) {
                checked(v.total_months(), len);
                for p in [1usize, 2, 9] {
                    let _ = v.to_text(p);
                }
            }
        }
        6 => {
            if let Some(v) = checked(IntervalDs::decode(field), len) {
                let _ = v.time_nanos();
                let _ = v.total_nanos();
                checked(v.to_duration_micros(), len);
                checked(v.to_duration_nanos(), len);
                checked(v.to_month_day_nano(), len);
                for (p, f) in [(1usize, 0usize), (2, 6), (9, 9)] {
                    let _ = v.to_text(p, f);
                }
            }
        }
        7 => {
            checked(decode_rowid(field), len);
        }
        _ => {
            checked(decode_urowid(field), len);
        }
    }
});
