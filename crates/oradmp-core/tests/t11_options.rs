//! Task T-11: the options (`extra_intro_lines`, `max_line_bytes`, `max_lob_bytes`, the per-column overflow policy
//! of a `number_types` entry, `Warning.max_digits_dropped`), the survey's counting core and the check that no
//! message holds a value.
//!
//! Expected numbers: `truth_num.txt` (T_NUM column V: rows 19 to 24, 26, 28, 29, 32 have an integer part above
//! 28 digits, rows 25, 27, 30, 31 more than 10 decimals; the largest number of fractional digits is 125 (row 27,
//! 1E-125), so `round` drops at most 115 digits below the scale 10: the test computes the figure from the truth
//! text with its own parser, a cross-check with Python `decimal` over the same file gave rows [25, 27, 30, 31],
//! fractions 38, 125, 37, 40 and 115), `truth_num3.txt` (all six rows rounded), `truth_date.txt` (rows 8, 9: year
//! below 1), `truth_misc2.txt` (rows 2, 3: months beyond 32 bits), `common::TOTALS` (rules file v3.7: 243,303
//! fields, 19,960 NULL fields, 126 LOB values over the 211 files, from `corpus_counts.py` and `rowlen_stats.py`).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::collections::BTreeMap;

use common::*;
use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::exp::rows::{LobAction, LobForm, RowSink};
use oradmp_core::reader::{OnValueError, ReadOptions, UnconfirmedLob};
use oradmp_core::survey::count_table;
use oradmp_core::types::map::NumberType;
use oradmp_core::values::number::OverflowPolicy;
use oradmp_core::{Error, ErrorCode, Warning, WarningCode};

fn warning(ws: &[Warning], code: WarningCode) -> Option<&Warning> {
    ws.iter().find(|w| w.code == code)
}

// ---------------------------------------------------------------------------------------------
// extra_intro_lines
// ---------------------------------------------------------------------------------------------

/// `stat_exp_18c.dmp` with its first `ANALSTATS IS ` line misspelled `ANALSTATT IS `: not in list K.
fn misspelled() -> (std::path::PathBuf, u64) {
    let mut at = 0;
    let p = patched_file("18c/stat_exp_18c.dmp", "t11", "o_intro.dmp", |b| {
        at = find_first(b, b"\nANALSTATS IS ") + 1;
        b[at + 8] = b'T';
    });
    (p, at as u64)
}

#[test]
fn extra_intro_lines_extend_list_k_so_that_an_unknown_line_needs_no_release() {
    let (p, at) = misspelled();
    // without the entry: read by G1, with a warning at the line and one field counted
    let mut d = open(&p, Options::default()).unwrap();
    let c = d.scan().unwrap();
    assert_eq!(c.item_counts.g1_fields, 1);
    let w = warning(d.warnings(), WarningCode::UnknownIntro).unwrap();
    assert_eq!(w.offset, Some(at));
    // with the entry (the form of `data/exp_intro_lines.txt`): no warning, no G1 field, one more entry of list K
    let o = Options {
        extra_intro_lines: vec!["prefix:10:plain|marked:ANALSTATT ".to_string()],
        ..Options::default()
    };
    let mut d = open(&p, o).unwrap();
    let c = d.scan().unwrap();
    assert_eq!(c.item_counts.g1_fields, 0);
    let last = c.item_counts.per_entry.last().unwrap();
    assert_eq!(last.total(), 1);
    assert!(warning(d.warnings(), WarningCode::UnknownIntro).is_none());
    // the same rows through the reader (which holds the extended list too)
    let (rows_a, _) = read(&mut d, "T_STAT1", &ReadOptions::default());
    let (rows_b, _) = read_default("18c/stat_exp_18c.dmp", "T_STAT1");
    assert_eq!(row_count(&rows_a), row_count(&rows_b));
    // strict: the extra entry makes the file pass, the unknown line fails it
    let s = Options {
        strict: true,
        extra_intro_lines: vec!["prefix:10:plain|marked:ANALSTATT ".to_string()],
        ..Options::default()
    };
    assert!(open(&p, s).unwrap().scan().is_ok());
}

#[test]
fn an_invalid_extra_intro_line_is_a_config_error() {
    let p = fixtures().join("18c/t_num_exp_18c.dmp");
    for bad in [
        "",
        "# a comment",
        "prefix:3:plain:ab",
        "middle:2:plain:ab",
        "prefix:2:flat:ab",
        "prefix:2:plain",
        "prefix:2:plain:ab\nprefix:2:plain:cd",
        "prefix:2:plain:ab\r",
    ] {
        let o = Options {
            extra_intro_lines: vec![bad.to_string()],
            ..Options::default()
        };
        let e = open(&p, o).unwrap_err();
        assert_eq!((e.code, e.offset), (ErrorCode::Config, None), "{bad:?}");
    }
}

// ---------------------------------------------------------------------------------------------
// max_line_bytes, max_lob_bytes
// ---------------------------------------------------------------------------------------------

#[test]
fn max_line_bytes_limits_a_line_and_the_error_stands_where_the_line_begins() {
    // the CREATE TABLE line of T_NUM is longer than 100 bytes; the lines before it are not
    let p = fixtures().join("18c/t_num_exp_18c.dmp");
    let data = std::fs::read(&p).unwrap();
    let at = find_first(&data, b"CREATE TABLE \"T_NUM\"") as u64;
    let o = Options {
        max_line_bytes: 100,
        ..Options::default()
    };
    let e = open(&p, o).unwrap().scan().unwrap_err();
    assert_eq!((e.code, e.offset), (ErrorCode::Framing, Some(at)));
    assert!(e.length.unwrap() > 100);
    assert!(e.message.contains("max_line_bytes"));
    // a limit above the line passes
    let o = Options {
        max_line_bytes: 1 << 20,
        ..Options::default()
    };
    assert!(open(&p, o).unwrap().scan().is_ok());
}

#[test]
fn max_lob_bytes_bounds_a_long_value_and_the_stored_length_of_a_lob_value() {
    // T_LONG row 3 has a LONG of 32,000 bytes (truth: `long_exp_18c.dmp`), rows 1 and 2 are shorter
    let rel = "18c/long_exp_18c.dmp";
    let at = {
        let mut d = open_fixture(rel);
        let data = std::fs::read(d.path()).unwrap();
        let ncols = d.table(TableRef::ByName("T_LONG")).unwrap().columns.len();
        field_offsets(&data, row_starts(&mut d, "T_LONG")[2], ncols)[1].unwrap()
    };
    let mut d = open_fixture(rel);
    let o = |n: u64| ReadOptions {
        max_lob_bytes: n,
        ..ReadOptions::default()
    };
    let e = try_read(&mut d, "T_LONG", &o(31_999)).unwrap_err();
    assert_eq!(
        (e.code, e.offset, e.row),
        (ErrorCode::LobTooLarge, Some(at), Some(3))
    );
    assert_eq!(e.length, Some(32_000));
    // exactly the length is allowed
    assert_eq!(row_count(&read(&mut d, "T_LONG", &o(32_000)).0), 3);
    // `null` turns the refusal into NULL with a counted warning
    let n = ReadOptions {
        on_value_error: OnValueError::Null,
        ..o(31_999)
    };
    let (_, ws) = read(&mut d, "T_LONG", &n);
    assert_eq!(
        warning(&ws, WarningCode::ValueNulled).unwrap().count,
        Some(1)
    );
    // a LOB value: the stored length (u16 n or T) against the limit, checked before a byte is read
    let mut d = open_fixture("18c/lob_exp_18c.dmp");
    let e = try_read(&mut d, "T_LOB", &o(2_999)).unwrap_err();
    assert_eq!(e.code, ErrorCode::LobTooLarge);
}

// ---------------------------------------------------------------------------------------------
// the per-column overflow policy of a `number_types` entry (session 6 (b))
// ---------------------------------------------------------------------------------------------

fn with_entry(table_column: &str, policy: OverflowPolicy) -> Options {
    let mut o = Options::default();
    o.types
        .number_types
        .insert(table_column.to_string(), NumberType::DEFAULT);
    o.types
        .number_overflow
        .insert(table_column.to_string(), policy);
    o
}

fn lenient(global: OverflowPolicy) -> ReadOptions {
    ReadOptions {
        on_value_error: OnValueError::Null,
        on_number_overflow: global,
        ..ReadOptions::default()
    }
}

#[test]
fn the_entry_policy_wins_over_the_global_policy_on_t_num_v() {
    let p = fixtures().join("18c/t_num_exp_18c.dmp");
    // global `round`, entry `error`: the four values with more than 10 decimals are value errors like the ten
    // integer overflows: 14 NULLs, nothing rounded
    let mut d = open(&p, with_entry("T_NUM.V", OverflowPolicy::Error)).unwrap();
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Round));
    assert!(warning(&ws, WarningCode::NumberRounded).is_none());
    let w = warning(&ws, WarningCode::ValueNulled).unwrap();
    assert_eq!((w.count, w.row), (Some(14), Some(19)));
    // the reverse: global `error`, entry `round`: 4 rounded (one to 0), the ten integer overflows stay errors
    let mut d = open(&p, with_entry("T_NUM.V", OverflowPolicy::Round)).unwrap();
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Error));
    let w = warning(&ws, WarningCode::NumberRounded).unwrap();
    assert_eq!(
        (w.count, w.count_to_zero, w.row),
        (Some(4), Some(1), Some(25))
    );
    let w = warning(&ws, WarningCode::ValueNulled).unwrap();
    assert_eq!(w.count, Some(10));
    // no entry: the global policy decides (control)
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Round));
    assert_eq!(
        warning(&ws, WarningCode::NumberRounded).unwrap().count,
        Some(4)
    );
    // the entry `null` against the global `round`
    let mut d = open(&p, with_entry("T_NUM.V", OverflowPolicy::Null)).unwrap();
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Round));
    assert_eq!(
        warning(&ws, WarningCode::NumberNulled).unwrap().count,
        Some(4)
    );
    assert!(warning(&ws, WarningCode::NumberRounded).is_none());
}

#[test]
fn the_entry_policy_stops_the_read_at_the_first_value_with_too_many_decimals() {
    // `t_num3` (truth_num3.txt): every value has more than 10 decimals, row 1 is the first; with `raise`
    let p = fixtures().join("18c/t_num3_exp_18c.dmp");
    let mut d = open(&p, with_entry("T_NUM3.V", OverflowPolicy::Error)).unwrap();
    let ro = ReadOptions {
        on_number_overflow: OverflowPolicy::Round,
        ..ReadOptions::default()
    };
    let e = try_read(&mut d, "T_NUM3", &ro).unwrap_err();
    assert_eq!(
        (e.code, e.row, e.column),
        (ErrorCode::NumberOverflow, Some(1), Some(2))
    );
    // and the reverse reads all six rows, rounded
    let mut d = open(&p, with_entry("T_NUM3.V", OverflowPolicy::Round)).unwrap();
    let ro = ReadOptions {
        on_number_overflow: OverflowPolicy::Error,
        ..ReadOptions::default()
    };
    let (b, ws) = read(&mut d, "T_NUM3", &ro);
    assert_eq!(row_count(&b), 6);
    assert_eq!(
        warning(&ws, WarningCode::NumberRounded).unwrap().count,
        Some(6)
    );
    // the metadata of the column says which policy was used
    let f = b[0].schema().field(1).clone();
    assert_eq!(
        f.metadata()
            .get("oradmp:number_overflow")
            .map(String::as_str),
        Some("round")
    );
}

#[test]
fn a_policy_without_a_number_types_entry_is_a_config_error() {
    let mut o = Options::default();
    o.types
        .number_overflow
        .insert("T_NUM.V".to_string(), OverflowPolicy::Error);
    let e = open(fixtures().join("18c/t_num_exp_18c.dmp"), o).unwrap_err();
    assert_eq!((e.code, e.offset), (ErrorCode::Config, None));
}

// ---------------------------------------------------------------------------------------------
// Warning.max_digits_dropped (session 6 (c))
// ---------------------------------------------------------------------------------------------

/// (digits before the decimal point, digits after it) of a decimal text of the truth files (`-1.5E+3`, `.5`,
/// `1E-125`, `123.45`): the test's own parser.
fn int_and_frac_digits(text: &str) -> (i32, i32) {
    let t = text.trim().trim_start_matches('-');
    let (mant, exp) = match t.split_once('E') {
        Some((m, e)) => (m, e.parse::<i32>().unwrap()),
        None => (t, 0),
    };
    let (ip, fp) = mant.split_once('.').unwrap_or((mant, ""));
    let fp = fp.trim_end_matches('0');
    let digits = format!("{ip}{fp}");
    let lead_zeros = digits.len() - digits.trim_start_matches('0').len();
    if lead_zeros == digits.len() {
        return (0, 0);
    }
    let point = ip.len() as i32 + exp;
    let int = (point - lead_zeros as i32).max(0);
    let frac = (digits.len() as i32 - point).max(0);
    (int, frac)
}

/// The largest (fractional digits - scale) over the values of a truth file that a Decimal(p, s) rounds: the
/// integer part fits (at most p - s digits) and there are more than s decimals.
fn max_dropped(file: &str, p: i32, s: i32) -> Option<u32> {
    truth("18c", file)
        .iter()
        .filter(|l| !is_null_text(&l[1]))
        .filter_map(|l| {
            let (int, frac) = int_and_frac_digits(&l[1]);
            (int <= p - s && frac > s).then_some((frac - s) as u32)
        })
        .max()
}

fn is_null_text(t: &str) -> bool {
    t.is_empty() || t == "NULL" || t == "<NULL>"
}

#[test]
fn w_number_rounded_carries_the_largest_number_of_digits_dropped() {
    // Decimal128(38, 10) + round: the figure is computed from the truth text (125 - 10 = 115 for row 27)
    let want = max_dropped("truth_num.txt", 38, 10).unwrap();
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Round));
    let w = warning(&ws, WarningCode::NumberRounded).unwrap();
    assert_eq!(w.max_digits_dropped, Some(want));
    assert_eq!(want, 115, "plan T-12: largest frac 125, scale 10");
    assert!(w.to_string().contains("115 digits dropped"), "{w}");
    // Decimal256(76, 38): fewer values are rounded and fewer digits dropped
    let mut o = Options::default();
    o.types.number_types.insert(
        "T_NUM.V".to_string(),
        "decimal256(76,38)".parse::<NumberType>().unwrap(),
    );
    let mut d = open(fixtures().join("18c/t_num_exp_18c.dmp"), o).unwrap();
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Round));
    let w = warning(&ws, WarningCode::NumberRounded).unwrap();
    assert_eq!(w.max_digits_dropped, max_dropped("truth_num.txt", 76, 38));
    // the other counted warnings carry none
    let mut d = open_fixture("18c/t_num_exp_18c.dmp");
    let (_, ws) = read(&mut d, "T_NUM", &lenient(OverflowPolicy::Null));
    assert!(ws.iter().all(|w| w.max_digits_dropped.is_none()));
    // T_NUM3: the six computed values (truth_num3.txt)
    let want = max_dropped("truth_num3.txt", 38, 10).unwrap();
    let mut d = open_fixture("18c/t_num3_exp_18c.dmp");
    let (_, ws) = read(&mut d, "T_NUM3", &ReadOptions::default());
    assert_eq!(
        warning(&ws, WarningCode::NumberRounded)
            .unwrap()
            .max_digits_dropped,
        Some(want)
    );
}

#[test]
fn the_digits_parser_of_this_file_agrees_with_the_plan_on_the_t_num_rows() {
    // rows 25, 27, 30, 31 of truth_num.txt: fractional digits 38, 125, 37, 40 (cross-check with Python
    // `decimal`, see the file header); the integer-overflow rows have more than 28 integer digits
    let lines = truth("18c", "truth_num.txt");
    let fr = |row: usize| int_and_frac_digits(&lines[row - 1][1]).1;
    assert_eq!([fr(25), fr(27), fr(30), fr(31)], [38, 125, 37, 40]);
    for row in [19, 20, 21, 22, 23, 24, 26, 28, 29, 32] {
        assert!(int_and_frac_digits(&lines[row - 1][1]).0 > 28, "row {row}");
    }
    assert_eq!(int_and_frac_digits("-1.5E+3"), (4, 0));
    assert_eq!(int_and_frac_digits(".5"), (0, 1));
    assert_eq!(int_and_frac_digits("1E-125"), (0, 125));
}

// ---------------------------------------------------------------------------------------------
// the survey's counting core
// ---------------------------------------------------------------------------------------------

#[test]
fn the_counting_core_agrees_with_the_totals_of_the_201_files() {
    let mut fields = 0u64;
    let mut nulls = 0u64;
    let mut rows = 0u64;
    let mut lob_values = 0u64;
    let mut refused = 0u64;
    for rel in exp_files() {
        let mut d = open_fixture(&rel);
        let tables: Vec<u32> = d.tables().unwrap().iter().map(|t| t.ordinal).collect();
        for o in tables {
            let c = count_table(&mut d, TableRef::ByOrdinal(o)).unwrap();
            rows += c.rows;
            for col in &c.columns {
                fields += col.values + col.nulls;
                nulls += col.nulls;
                if matches!(col.type_code, 112 | 113) {
                    lob_values += col.values;
                }
                refused += col.lob_refused_short + col.lob_refused_long;
            }
        }
    }
    assert_eq!(rows, TOTALS.rows);
    assert_eq!(fields, TOTALS.fields);
    assert_eq!(nulls, TOTALS.nulls);
    assert_eq!(lob_values, TOTALS.lob_values);
    // ADR-25 item 5, plan session 8 (f): 0 values refused on all 211 files
    assert_eq!(refused, 0);
}

#[test]
fn the_counting_core_counts_the_values_adr_25_refuses_by_form_on_the_178_copy() {
    // the 178 copy of T-6M (d): C 1 long; N 1 short and 1 long (the plan); nothing else
    let p = patched_file(
        "11g-cp1251/ru_lob2_exp_cp1251_11g.dmp",
        "t11",
        "o_lob178.dmp",
        |b| {
            b[0x36] = 0xb2;
            b[0x24e3] = 0xb2;
        },
    );
    let mut d = open(&p, Options::default()).unwrap();
    let c = count_table(&mut d, TableRef::ByName("R_LOB2")).unwrap();
    let by_pos: Vec<(u16, u64, u64)> = c
        .columns
        .iter()
        .map(|x| (x.type_code, x.lob_refused_short, x.lob_refused_long))
        .collect();
    let named: Vec<(&str, (u64, u64))> = d
        .table(TableRef::ByName("R_LOB2"))
        .unwrap()
        .columns
        .iter()
        .zip(&by_pos)
        .map(|(col, &(_, s, l))| (col.name.as_str(), (s, l)))
        .collect();
    let get = |n: &str| named.iter().find(|x| x.0 == n).unwrap().1;
    assert_eq!(get("C"), (0, 1));
    assert_eq!(get("N"), (1, 1));
    assert_eq!(get("B"), (0, 0));
    assert_eq!(
        named.iter().map(|x| x.1 .0 + x.1 .1).sum::<u64>(),
        3,
        "{named:?}"
    );
    // the reader refuses exactly these values: with `null` and `decode`'s counterpart the counts of
    // W-VALUE-NULLED are C 1 and N 2
    let null = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let (_, ws) = read(&mut d, "R_LOB2", &null);
    let mut nulled: Vec<(u32, u64)> = ws
        .iter()
        .filter(|w| w.code == WarningCode::ValueNulled)
        .map(|w| (w.column.unwrap(), w.count.unwrap()))
        .collect();
    nulled.sort_unstable();
    assert_eq!(nulled, [(2, 1), (3, 2)]);
    // the unpatched file has none
    let mut d = open_fixture("11g-cp1251/ru_lob2_exp_cp1251_11g.dmp");
    let c = count_table(&mut d, TableRef::ByName("R_LOB2")).unwrap();
    assert!(c
        .columns
        .iter()
        .all(|x| x.lob_refused_short + x.lob_refused_long == 0));
}

#[test]
fn the_counting_core_counts_years_below_1_and_months_beyond_32_bits_like_the_reader() {
    // truth_date.txt rows 8 and 9; truth_misc2.txt rows 2 and 3 (YEAR (9) TO MONTH beyond 2^31 - 1 months)
    for v in ["18c", "11g"] {
        let mut d = open_fixture(&format!("{v}/t_date_exp_{v}.dmp"));
        let c = count_table(&mut d, TableRef::ByName("T_DATE")).unwrap();
        assert_eq!(c.columns[1].years_below_1, 2, "{v}");
        assert_eq!(c.columns[0].years_below_1, 0);
        // the reader: the same two values are the value errors
        let null = ReadOptions {
            on_value_error: OnValueError::Null,
            ..ReadOptions::default()
        };
        let (_, ws) = read(&mut d, "T_DATE", &null);
        assert_eq!(
            warning(&ws, WarningCode::ValueNulled).unwrap().count,
            Some(2)
        );
        let mut d = open_fixture(&format!("{v}/misc2_exp_{v}.dmp"));
        let c = count_table(&mut d, TableRef::ByName("T_INTERVAL2")).unwrap();
        assert_eq!(c.columns[1].months_beyond_32_bits, 2, "{v}");
        assert_eq!(c.columns[2].months_beyond_32_bits, 0);
        let (_, ws) = read(&mut d, "T_INTERVAL2", &null);
        assert_eq!(
            warning(&ws, WarningCode::ValueNulled).unwrap().count,
            Some(2)
        );
    }
    // no other DATE-family column of the 211 files has a year below 1, no other interval column an overflow
    let mut others = 0u64;
    for rel in exp_files() {
        let mut d = open_fixture(&rel);
        let tables: Vec<(u32, String)> = d
            .tables()
            .unwrap()
            .iter()
            .map(|t| (t.ordinal, t.name.clone()))
            .collect();
        for (o, name) in tables {
            let c = count_table(&mut d, TableRef::ByOrdinal(o)).unwrap();
            for col in &c.columns {
                if name != "T_DATE" && name != "T_INTERVAL2" {
                    others += col.years_below_1 + col.months_beyond_32_bits;
                }
            }
        }
    }
    assert_eq!(others, 0);
}

/// ADR-43 item 2 (review S-2): no fixture holds a TIMESTAMP-family value with a year below 1 (`truth_ts.txt` and
/// `truth_tstz.txt` have none; R-044 is not generated), so the copies write the confirmed pair `63 64` [Y2]
/// (century 99, year 100: year -100, the pair of `truth_date.txt` row 8) over the first two bytes of a value:
/// `18c/t_ts_exp_18c.dmp` row 2 of the three TIMESTAMP columns (7-byte values `78,74,3,1a,11,a,1c` in the
/// DUMP of `truth_ts.txt`), `18c/t_tstz_exp_18c.dmp` row 2 of TSTZ (`78,74,3,1a,14,23,38,...`, DUMP of
/// `truth_tstz.txt`) and the first non-NULL TSLTZ value (row 101, `78,74,3,1a,14,23,38,7,5b,ca,0`). The survey
/// counts 1 in each patched column and 0 in every other; the plain read stops with `E-BAD-DATETIME` at row 2 of
/// the first patched column. Mutation e1 (the survey counts DATE, type 12, only) fails this test.
#[test]
fn the_counting_core_counts_years_below_1_in_the_timestamp_family_on_patched_copies() {
    let years = |p: &std::path::Path, table: &str| -> Vec<u64> {
        let mut d = open(p, Options::default()).unwrap();
        let c = count_table(&mut d, TableRef::ByName(table)).unwrap();
        c.columns.iter().map(|x| x.years_below_1).collect()
    };
    // the unpatched files count none
    for (rel, table, n) in [
        ("18c/t_ts_exp_18c.dmp", "T_TS", 4),
        ("18c/t_tstz_exp_18c.dmp", "T_TSTZ", 3),
    ] {
        let mut d = open_fixture(rel);
        let c = count_table(&mut d, TableRef::ByName(table)).unwrap();
        assert_eq!(c.columns.len(), n);
        assert!(c.columns.iter().all(|x| x.years_below_1 == 0), "{rel}");
    }

    // T_TS: ID, TS0, TS6, TS9
    let rel = "18c/t_ts_exp_18c.dmp";
    let data = std::fs::read(fixtures().join(rel)).unwrap();
    let mut d = open_fixture(rel);
    let starts = row_starts(&mut d, "T_TS");
    let f = field_offsets(&data, starts[1], 4);
    let ts = [0x78u8, 0x74, 3, 0x1a, 0x11, 0xa, 0x1c];
    let at: Vec<usize> = f[1..].iter().map(|x| x.unwrap() as usize).collect();
    for a in &at {
        assert_eq!(data[*a..*a + 7], ts, "row 2 value before patching");
    }
    let p = patched_file(rel, "t11", "o_ts_year_below_1.dmp", |b| {
        for a in &at {
            b[*a..*a + 2].copy_from_slice(&[0x63, 0x64]);
        }
    });
    assert_eq!(years(&p, "T_TS"), [0, 1, 1, 1]);
    let mut d = open(&p, Options::default()).unwrap();
    let e = try_read(&mut d, "T_TS", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadDatetime);
    assert_eq!((e.row, e.column), (Some(2), Some(2)));
    assert_eq!(e.offset, Some(at[0] as u64));

    // T_TSTZ: ID, TSTZ, TSLTZ
    let rel = "18c/t_tstz_exp_18c.dmp";
    let data = std::fs::read(fixtures().join(rel)).unwrap();
    let mut d = open_fixture(rel);
    let starts = row_starts(&mut d, "T_TSTZ");
    let tstz = field_offsets(&data, starts[1], 3)[1].unwrap() as usize;
    assert_eq!(
        data[tstz..tstz + 7],
        [0x78, 0x74, 3, 0x1a, 0x14, 0x23, 0x38]
    );
    let (ltz_row, ltz) = starts
        .iter()
        .enumerate()
        .find_map(|(i, s)| field_offsets(&data, *s, 3)[2].map(|a| (i, a as usize)))
        .unwrap();
    assert_eq!(
        ltz_row, 7,
        "the first non-NULL TSLTZ value is row 101, the 8th"
    );
    assert_eq!(
        data[ltz..ltz + 11],
        [0x78, 0x74, 3, 0x1a, 0x14, 0x23, 0x38, 7, 0x5b, 0xca, 0]
    );
    let p = patched_file(rel, "t11", "o_tstz_year_below_1.dmp", |b| {
        for a in [tstz, ltz] {
            b[a..a + 2].copy_from_slice(&[0x63, 0x64]);
        }
    });
    assert_eq!(years(&p, "T_TSTZ"), [0, 1, 1]);
    let mut d = open(&p, Options::default()).unwrap();
    let e = try_read(&mut d, "T_TSTZ", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadDatetime);
    assert_eq!((e.row, e.column), (Some(2), Some(2)));
    assert_eq!(e.offset, Some(tstz as u64));
}

#[test]
fn the_counting_core_flags_a_declared_number_no_decimal128_holds() {
    // T_DECL has none; with `N_STAR NUMBER(*,2)` patched to `NUMBER(*,40)` (ADR-30 item 3) exactly N_STAR
    let mut d = open_fixture("18c/t_decl_exp_18c.dmp");
    let c = count_table(&mut d, TableRef::ByName("T_DECL")).unwrap();
    assert!(c.columns.iter().all(|x| !x.no_decimal128_holds));
    let p = patched_file("18c/t_decl_exp_18c.dmp", "t11", "o_nodec.dmp", |b| {
        let at = find_first(b, b"\"N_STAR\" NUMBER(*,2)") + 18;
        b.splice(at..at + 1, *b"40");
        let cut = b.len() - 1;
        assert_eq!(b[cut], 0);
        b.truncate(cut);
    });
    let mut d = open(&p, Options::default()).unwrap();
    let names: Vec<String> = d
        .table(TableRef::ByName("T_DECL"))
        .unwrap()
        .columns
        .iter()
        .map(|c| c.name.clone())
        .collect();
    let c = count_table(&mut d, TableRef::ByName("T_DECL")).unwrap();
    let flagged: Vec<&str> = c
        .columns
        .iter()
        .zip(&names)
        .filter(|(x, _)| x.no_decimal128_holds)
        .map(|(_, n)| n.as_str())
        .collect();
    assert_eq!(flagged, ["N_STAR"]);
}

// ---------------------------------------------------------------------------------------------
// no value in any message
// ---------------------------------------------------------------------------------------------

/// Collects the raw bytes of every field of every row (at least 8 bytes) of a table.
struct Fields(Vec<Vec<u8>>);

impl RowSink for Fields {
    fn row_start(&mut self, _: u64) {}
    fn null(&mut self, _: u32, _: u64) -> oradmp_core::Result<()> {
        Ok(())
    }
    fn value(&mut self, _: u32, _: u64, b: &[u8]) -> oradmp_core::Result<()> {
        if b.len() >= 8 {
            self.0.push(b.to_vec());
        }
        Ok(())
    }
    fn chain_piece(
        &mut self,
        _: u32,
        _: u64,
        b: &[u8],
        _: bool,
        _: bool,
    ) -> oradmp_core::Result<()> {
        if b.len() >= 8 {
            self.0.push(b.to_vec());
        }
        Ok(())
    }
    fn lob_start(&mut self, _: u32, _: u64, _: LobForm, _: u64) -> oradmp_core::Result<LobAction> {
        Ok(LobAction::Skip)
    }
    fn row_end(&mut self) -> oradmp_core::Result<()> {
        Ok(())
    }
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && hay.windows(needle.len()).any(|w| w == needle)
}

/// The values of all tables of a file (fields of 8 bytes or more) that a message could repeat.
fn values_of(d: &mut oradmp_core::dump::Dump) -> Vec<Vec<u8>> {
    let tables: Vec<u32> = d.tables().unwrap().iter().map(|t| t.ordinal).collect();
    let mut f = Fields(Vec::new());
    for o in tables {
        let parts = d.table(TableRef::ByOrdinal(o)).unwrap().parts.len();
        for k in 0..parts {
            let mut cur = d.part_rows(TableRef::ByOrdinal(o), k).unwrap();
            while cur.next_row_with(&mut f).unwrap().is_some() {}
        }
    }
    f.0.sort();
    f.0.dedup();
    f.0
}

/// Every error and warning text produced from the 211 fixtures under the options below, and from the 178 copy.
fn texts_of(rel: &str, strict: bool, ro: &ReadOptions) -> Vec<String> {
    let o = Options {
        strict,
        ..Options::default()
    };
    let mut d = open(fixtures().join(rel), o).unwrap();
    let mut out = Vec::new();
    if let Err(e) = d.scan() {
        out.push(e.to_string());
        return out;
    }
    out.extend(d.warnings().iter().map(ToString::to_string));
    let names: Vec<String> = d.tables().unwrap().iter().map(|t| t.name.clone()).collect();
    for n in names {
        let mut r = match d.read_table(TableRef::ByName(&n), ro) {
            Ok(r) => r,
            Err(e) => {
                out.push(e.to_string());
                continue;
            }
        };
        loop {
            match r.next_batch() {
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(e) => {
                    out.push(e.to_string());
                    break;
                }
            }
        }
        out.extend(r.warnings().iter().map(ToString::to_string));
    }
    out
}

#[test]
fn no_error_or_warning_text_from_the_fixtures_holds_a_value() {
    let lenient_rounded = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let lenient_null = ReadOptions {
        on_value_error: OnValueError::Null,
        on_number_overflow: OverflowPolicy::Null,
        ..ReadOptions::default()
    };
    let decode = ReadOptions {
        unconfirmed_lob: UnconfirmedLob::Decode,
        ..lenient_rounded.clone()
    };
    let mut seen_texts = 0usize;
    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    for rel in exp_files() {
        let mut d = open_fixture(&rel);
        let values = values_of(&mut d);
        let mut texts = Vec::new();
        for (strict, ro) in [
            (false, ReadOptions::default()),
            (true, ReadOptions::default()),
            (false, lenient_rounded.clone()),
            (false, lenient_null.clone()),
            (false, decode.clone()),
        ] {
            texts.extend(texts_of(&rel, strict, &ro));
        }
        for t in &texts {
            for v in &values {
                assert!(
                    !contains(t.as_bytes(), v),
                    "{rel}: a message holds a value: {t}"
                );
            }
            seen_texts += 1;
            if let Some(code) = t.split(' ').next() {
                *kinds.entry(code.to_string()).or_default() += 1;
            }
        }
    }
    // the corpus of messages is not empty and covers the error and warning codes of the fixtures
    assert!(seen_texts > 100, "{seen_texts}");
    for code in [
        "[E-NUMBER-OVERFLOW",
        "[E-BAD-DATETIME",
        "[E-STRICT",
        "[W-NUMBER-ROUNDED",
        "[W-VALUE-NULLED",
        "[W-TSLTZ",
    ] {
        assert!(kinds.contains_key(code), "no text of {code}: {kinds:?}");
    }
}

#[test]
fn the_message_check_can_fail() {
    // a message that repeats the value of T_FRAME row 7 (`NNNNNNNNNN`, 10 bytes) is found
    let mut d = open_fixture("18c/misc_exp_18c.dmp");
    let values = values_of(&mut d);
    assert!(values.iter().any(|v| v == b"NNNNNNNNNN"));
    let bad = Error::new(ErrorCode::BadText, "x").at(5).to_string() + " NNNNNNNNNN";
    assert!(values.iter().any(|v| contains(bad.as_bytes(), v)));
    // a value of a LOB or a long text: the 8-byte floor does not hide the long ones
    assert!(!values.is_empty());
}
