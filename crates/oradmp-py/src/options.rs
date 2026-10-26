//! The options of `oradmp.open()` and `read_table()` as plain Python values (strings, numbers, a dict) turned into
//! the core's `Options` and `ReadOptions`. A bad value is `E-CONFIG` (Python `UsageError`).

use std::str::FromStr;

use oradmp_core::dump::Options;
use oradmp_core::guard::catch_panic;
use oradmp_core::reader::{OnValueError, ReadOptions, UnconfirmedLob};
use oradmp_core::types::map::{IntervalAs, NumberType, TimestampUnit};
use oradmp_core::values::number::OverflowPolicy;
use oradmp_core::{Error, ErrorCode};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn config(message: &'static str) -> Error {
    Error::new(ErrorCode::Config, message)
}

/// `NumberType::from_str` under the panic guard (ADR-65 item 1): it parses a user string.
fn number_type(s: &str) -> Result<NumberType, Error> {
    catch_panic(|| NumberType::from_str(s))
}

pub fn on_value_error(s: &str) -> Result<OnValueError, Error> {
    match s {
        "raise" => Ok(OnValueError::Raise),
        "null" => Ok(OnValueError::Null),
        _ => Err(config("on_value_error must be \"raise\" or \"null\"")),
    }
}

pub fn overflow_policy(s: &str) -> Result<OverflowPolicy, Error> {
    match s {
        "round" => Ok(OverflowPolicy::Round),
        "error" => Ok(OverflowPolicy::Error),
        "null" => Ok(OverflowPolicy::Null),
        _ => Err(config(
            "on_number_overflow must be \"round\", \"error\" or \"null\"",
        )),
    }
}

fn unconfirmed_lob(s: &str) -> Result<UnconfirmedLob, Error> {
    match s {
        "error" => Ok(UnconfirmedLob::Error),
        "decode" => Ok(UnconfirmedLob::Decode),
        _ => Err(config("unconfirmed_lob must be \"error\" or \"decode\"")),
    }
}

fn timestamp_unit(s: &str) -> Result<TimestampUnit, Error> {
    match s {
        "auto" => Ok(TimestampUnit::Auto),
        "micro" | "us" | "microsecond" => Ok(TimestampUnit::Micro),
        "nano" | "ns" | "nanosecond" => Ok(TimestampUnit::Nano),
        _ => Err(config(
            "timestamp_unit must be \"auto\", \"micro\" or \"nano\"",
        )),
    }
}

fn interval_as(s: &str) -> Result<IntervalAs, Error> {
    match s {
        "native" => Ok(IntervalAs::Native),
        "string" => Ok(IntervalAs::String),
        _ => Err(config("interval_as must be \"native\" or \"string\"")),
    }
}

macro_rules! getter {
    ($name:ident, $t:ty) => {
        /// A value of the dict, or `None` if the key is missing or the value is `None`.
        fn $name(d: &Bound<'_, PyDict>, key: &str) -> Result<Option<$t>, Error> {
            match d.get_item(key) {
                Ok(Some(v)) if !v.is_none() => v
                    .extract::<$t>()
                    .map(Some)
                    .map_err(|_| config("an option has a value of the wrong type")),
                Ok(_) => Ok(None),
                Err(_) => Err(config("an option could not be read")),
            }
        }
    };
}

getter!(get_bool, bool);
getter!(get_string, String);
getter!(get_usize, usize);
getter!(get_u64, u64);

/// The dump-wide options of `open()` (architecture 3.2 `Options`).
pub fn open_options(d: &Bound<'_, PyDict>) -> Result<Options, Error> {
    let mut o = Options::default();
    if let Some(v) = get_bool(d, "strict")? {
        o.strict = v;
    }
    o.db_charset = get_string(d, "db_charset")?;
    o.client_charset = get_string(d, "client_charset")?;
    if let Some(v) = get_usize(d, "max_line_bytes")? {
        o.max_line_bytes = v;
    }
    if let Some(v) = get_string(d, "number_default")? {
        o.types.number_default = number_type(&v)?;
    }
    if let Some(v) = get_string(d, "timestamp_unit")? {
        o.types.timestamp_unit = timestamp_unit(&v)?;
    }
    if let Some(v) = get_string(d, "interval_as")? {
        o.types.interval_as = interval_as(&v)?;
    }
    if let Some(lines) = d
        .get_item("extra_intro_lines")
        .map_err(|_| config("an option could not be read"))?
    {
        if !lines.is_none() {
            let list = lines
                .cast::<PyList>()
                .map_err(|_| config("extra_intro_lines must be a list of strings"))?;
            for l in list.iter() {
                o.extra_intro_lines.push(
                    l.extract::<String>()
                        .map_err(|_| config("extra_intro_lines must be a list of strings"))?,
                );
            }
        }
    }
    // number_types: {"TABLE.COLUMN": (type, policy or None)}, normalised by the Python layer
    if let Some(m) = d
        .get_item("number_types")
        .map_err(|_| config("an option could not be read"))?
    {
        if !m.is_none() {
            let m = m
                .cast::<PyDict>()
                .map_err(|_| config("number_types must be a dict"))?;
            for (k, v) in m.iter() {
                let key: String = k
                    .extract()
                    .map_err(|_| config("number_types keys must be strings"))?;
                let (ty, policy): (String, Option<String>) = v
                    .extract()
                    .map_err(|_| config("number_types values must be (type, policy)"))?;
                o.types.number_types.insert(key.clone(), number_type(&ty)?);
                if let Some(p) = policy {
                    o.types.number_overflow.insert(key, overflow_policy(&p)?);
                }
            }
        }
    }
    Ok(o)
}

/// The options of one read (architecture 3.2 `ReadOptions`); the Python layer has merged the per-call overrides
/// with the values given to `open()`.
pub fn read_options(d: &Bound<'_, PyDict>) -> Result<ReadOptions, Error> {
    let mut o = ReadOptions::default();
    if let Some(cols) = d
        .get_item("columns")
        .map_err(|_| config("an option could not be read"))?
    {
        if !cols.is_none() {
            let list = cols
                .cast::<PyList>()
                .map_err(|_| config("columns must be a list of names"))?;
            let mut names = Vec::new();
            for c in list.iter() {
                names.push(
                    c.extract::<String>()
                        .map_err(|_| config("columns must be a list of names"))?,
                );
            }
            o.columns = Some(names);
        }
    }
    if let Some(v) = get_usize(d, "batch_rows")? {
        o.batch_rows = v;
    }
    if let Some(v) = get_u64(d, "batch_bytes")? {
        o.batch_bytes = v;
    }
    if let Some(v) = get_string(d, "on_value_error")? {
        o.on_value_error = on_value_error(&v)?;
    }
    if let Some(v) = get_string(d, "on_number_overflow")? {
        o.on_number_overflow = overflow_policy(&v)?;
    }
    if let Some(v) = get_u64(d, "max_lob_bytes")? {
        o.max_lob_bytes = v;
    }
    if let Some(v) = get_string(d, "unconfirmed_lob")? {
        o.unconfirmed_lob = unconfirmed_lob(&v)?;
    }
    if let Some(v) = get_bool(d, "trim_char")? {
        o.trim_char = v;
    }
    Ok(o)
}
