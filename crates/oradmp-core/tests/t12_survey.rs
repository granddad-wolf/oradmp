//! Task T-12: the counting core of the survey (architecture 5.2b, 8.5): the NUMBER profile with its candidate
//! types, the length statistics, the LONG chain counts and the LOB counts, all counts, no value.
//!
//! Expected numbers: the Analyst's tools `docs/formats/tools/exp_parse.py` (`parse`) and `num_decode.py`
//! (`decode`) run read-only over the fixtures named below, each NUMBER value classified with Python `decimal`
//! (precision 500; `ROUND_HALF_UP` is half away from zero) for Decimal128(38, 10) and Decimal256(76, 38):
//! significant digits, fractional digits, integer digits, exp10, leading position, and the classes exact /
//! changed by half-away rounding at the scale (of which to 0) / integer part too large (unscaled digits above the
//! precision after rounding). `t_num_exp_18c.dmp` and `t_num_exp_11g.dmp` give identical profiles. Lengths of the
//! LONG values: `exp_parse.parse` of `long_exp_direct_<v>.dmp` and `long_exp_<v>.dmp`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use common::*;
use oradmp_core::dump::{Dump, Options, TableRef};
use oradmp_core::survey::{count_table, number_summary, ColumnCounts, TableCounts};

fn counts(d: &mut Dump, table: &str) -> TableCounts {
    count_table(d, TableRef::ByName(table)).unwrap()
}

/// (label, exact, rounded, rounded to 0, integer part too large) of the candidates of a column.
fn cands(c: &ColumnCounts) -> Vec<(&str, u64, u64, u64, u64)> {
    c.number
        .as_ref()
        .unwrap()
        .candidates
        .iter()
        .map(|k| {
            (
                k.label.as_str(),
                k.exact,
                k.rounded,
                k.rounded_to_zero,
                k.int_too_large,
            )
        })
        .collect()
}

/// `t_num_exp_<v>.dmp` T_NUM column V (32 non-NULL values, 1 NULL): the plan's numbers. Decimal128(38, 10): 18
/// exact, 4 changed by `round` (1 of them to 0: row 27), 10 whose integer part does not fit; Decimal256(76, 38):
/// 27 / 2 (1) / 3; largest `sig` 40, `frac` 125, `int` 126; exp10 from -125 to +125.
#[test]
fn the_number_profile_of_t_num_column_v() {
    for v in ["18c", "11g"] {
        let mut d = open_fixture(&format!("{v}/t_num_exp_{v}.dmp"));
        let t = counts(&mut d, "T_NUM");
        assert_eq!(t.rows, 33, "{v}");
        let id = &t.columns[0];
        assert_eq!((id.values, id.nulls), (33, 0), "{v}");
        let c = &t.columns[1];
        assert_eq!((c.values, c.nulls), (32, 1), "{v}");
        assert_eq!(
            cands(c),
            [
                ("decimal(38,10)", 18, 4, 1, 10),
                ("decimal256(76,38)", 27, 2, 1, 3)
            ],
            "{v}"
        );
        let p = c.number.as_ref().unwrap();
        assert_eq!(
            (
                p.invalid,
                p.integers,
                p.fit_int64,
                p.sig_39_or_40,
                p.max_sig,
                p.max_frac,
                p.max_int
            ),
            (0, 19, 11, 1, 40, 125, 126),
            "{v}"
        );
        assert_eq!((p.min_exp10, p.max_exp10), (Some(-125), Some(125)), "{v}");
        assert_eq!((p.min_lead, p.max_lead), (Some(-125), Some(125)), "{v}");
        // the ID column: 33 integers of one or two digits
        let q = id.number.as_ref().unwrap();
        assert_eq!(
            (q.integers, q.fit_int64, q.max_sig, q.max_int, q.max_frac),
            (33, 33, 2, 2, 0)
        );
        // the summary line: one column in which `round` changes values or an integer part does not fit, per candidate
        let s = number_summary(std::slice::from_ref(&t));
        let line: Vec<_> = s
            .iter()
            .map(|c| {
                (
                    c.label.as_str(),
                    c.columns,
                    c.exact,
                    c.rounded,
                    c.rounded_to_zero,
                    c.int_too_large,
                    c.columns_needing_override,
                )
            })
            .collect();
        assert_eq!(
            line,
            [
                ("decimal(38,10)", 2, 51, 4, 1, 10, 1),
                ("decimal256(76,38)", 2, 60, 2, 1, 3, 1)
            ],
            "{v}"
        );
    }
}

/// `t_num2_exp_18c.dmp` T_NUM2 column V (14 values: the Y1 vectors of R-024) and `t_num3_exp_18c.dmp` T_NUM3 column V
/// (6 computed values of 39 / 40 significant digits, R-032): the plan's flag for the computed-quotient signature.
#[test]
fn the_number_profile_flags_the_computed_quotients() {
    let mut d = open_fixture("18c/t_num3_exp_18c.dmp");
    let t = counts(&mut d, "T_NUM3");
    let c = &t.columns[1];
    assert_eq!((c.values, c.nulls), (6, 0));
    let p = c.number.as_ref().unwrap();
    assert_eq!(p.sig_39_or_40, 6);
    assert_eq!(
        (p.integers, p.fit_int64, p.max_sig, p.max_frac, p.max_int),
        (0, 0, 40, 40, 2)
    );
    assert_eq!((p.min_exp10, p.max_exp10), (Some(-40), Some(-38)));
    assert_eq!((p.min_lead, p.max_lead), (Some(-1), Some(1)));
    assert_eq!(
        cands(c),
        [
            ("decimal(38,10)", 0, 6, 0, 0),
            ("decimal256(76,38)", 2, 4, 0, 0)
        ]
    );
    // the other columns of the table are not flagged
    assert_eq!(
        t.columns[0].number.as_ref().unwrap().sig_39_or_40,
        0,
        "ID column"
    );

    let mut d = open_fixture("18c/t_num2_exp_18c.dmp");
    let t = counts(&mut d, "T_NUM2");
    let c = &t.columns[1];
    assert_eq!((c.values, c.nulls), (14, 0));
    let p = c.number.as_ref().unwrap();
    assert_eq!(
        (p.integers, p.fit_int64, p.max_sig, p.max_frac, p.max_int),
        (10, 9, 19, 130, 19)
    );
    assert_eq!((p.min_exp10, p.max_exp10), (Some(-130), Some(18)));
    assert_eq!(
        cands(c),
        [
            ("decimal(38,10)", 12, 2, 2, 0),
            ("decimal256(76,38)", 13, 1, 1, 0)
        ]
    );
}

/// A column that is not NUMBER has no profile, and a declared precision adds its own candidate.
#[test]
fn only_type_2_columns_have_a_profile_and_a_declared_type_is_a_candidate() {
    let mut d = open_fixture("18c/t_decl_exp_18c.dmp");
    let t = counts(&mut d, "T_DECL");
    let info = d.table(TableRef::ByName("T_DECL")).unwrap().clone();
    let mut with_profile = 0;
    for (c, i) in t.columns.iter().zip(&info.columns) {
        assert_eq!(c.number.is_some(), i.type_code == 2, "{}", i.name);
        with_profile += usize::from(c.number.is_some());
        if let Some(p) = &c.number {
            // the two standard candidates, then the resolved type if it is another decimal
            let labels: Vec<&str> = p.candidates.iter().map(|k| k.label.as_str()).collect();
            assert_eq!(&labels[..2], ["decimal(38,10)", "decimal256(76,38)"]);
            if let arrow_schema::DataType::Decimal128(pr, sc) = i.arrow_type {
                if (pr, sc) != (38, 10) {
                    assert_eq!(labels[2], format!("decimal({pr},{sc})"), "{}", i.name);
                }
            }
        }
    }
    assert!(with_profile >= 3, "{with_profile}");
    assert!(t
        .columns
        .iter()
        .zip(&info.columns)
        .any(|(c, i)| i.type_code == 2 && c.number.as_ref().unwrap().candidates.len() == 3));
}

/// Length statistics (bytes of the stored value; a chain by its total): `long_exp_direct_<v>.dmp` T_LONG has the
/// values 5 and 32,000 bytes and a NULL (`exp_parse.py`), T_LONGRAW 2 and 32,000. The LONG chain counts of the
/// scan: 2 values, 1 chained (the 5 pieces of 32,000 bytes), largest 32,000, none above 32,000; the conventional
/// export has no chain.
#[test]
fn length_statistics_and_the_long_chain_counts() {
    for (v, rel, chained) in [
        ("18c", "long_exp_direct_18c.dmp", 1u64),
        ("11g", "long_exp_direct_11g.dmp", 1),
        ("18c", "long_exp_18c.dmp", 0),
    ] {
        let mut d = open_fixture(&format!("{v}/{rel}"));
        let t = counts(&mut d, "T_LONG");
        let info = d.table(TableRef::ByName("T_LONG")).unwrap().clone();
        let li = info.columns.iter().position(|c| c.type_code == 8).unwrap();
        let c = &t.columns[li];
        assert_eq!((c.values, c.nulls), (2, 1), "{rel}");
        assert_eq!(
            (c.len_min, c.len_max, c.len_total),
            (Some(5), Some(32_000), 32_005),
            "{rel}"
        );
        let l = info.columns[li].long.as_ref().unwrap();
        assert_eq!(
            (l.values, l.chained, l.largest, l.above_32000),
            (2, chained, 32_000, 0),
            "{rel}"
        );
        let t = counts(&mut d, "T_LONGRAW");
        let info = d.table(TableRef::ByName("T_LONGRAW")).unwrap().clone();
        let li = info.columns.iter().position(|c| c.type_code == 24).unwrap();
        let c = &t.columns[li];
        assert_eq!(
            (c.values, c.len_min, c.len_max, c.len_total),
            (2, Some(2), Some(32_000), 32_002),
            "{rel}"
        );
        let l = info.columns[li].long.as_ref().unwrap();
        assert_eq!(
            (l.values, l.chained, l.largest, l.above_32000),
            (2, chained, 32_000, 0),
            "{rel}"
        );
    }
}

/// The `E-LOB-UNCONFIRMED` count of the survey is 0 over the fixtures (every CLOB / NCLOB value is of a
/// confirmed database set) and equals the reader's nulled values on the 178 copy of T-6M (d): C one long value
/// (row 2), N two values.
#[test]
fn the_unconfirmed_lob_count_is_zero_on_the_fixtures_and_matches_the_reader_on_the_178_copy() {
    let mut refused = 0u64;
    let mut lob_values = 0u64;
    for rel in exp_files() {
        let mut d = open_fixture(&rel);
        let ordinals: Vec<u32> = d.tables().unwrap().iter().map(|t| t.ordinal).collect();
        for o in ordinals {
            let t = count_table(&mut d, TableRef::ByOrdinal(o)).unwrap();
            for c in &t.columns {
                refused += c.lob_refused_short + c.lob_refused_long;
                if matches!(c.type_code, 112 | 113) {
                    lob_values += c.values;
                }
            }
        }
    }
    assert_eq!(refused, 0);
    // the LOB values were looked at (126 over the fixture files, rules v3.5 via `common::TOTALS`)
    assert_eq!(lob_values, TOTALS.lob_values);

    let p = patched_file(
        "11g-cp1251/ru_lob2_exp_cp1251_11g.dmp",
        "t12",
        "survey_178.dmp",
        |b| {
            assert_eq!(b[0x35..0x37], [0x00, 0xab]);
            assert_eq!(b[0x24e3..0x24e5], [0xab, 0x00]);
            b[0x36] = 0xb2;
            b[0x24e3] = 0xb2;
        },
    );
    let mut d = open_path(&p, Options::default());
    let t = counts(&mut d, "R_LOB2");
    let by_col: Vec<(u64, u64)> = t
        .columns
        .iter()
        .filter(|c| c.type_code == 112)
        .map(|c| (c.lob_refused_short, c.lob_refused_long))
        .collect();
    // C (CLOB, set 178 in the copy): the long value of row 2; N (NCLOB of a database set outside the list): the
    // short value of row 1 and the long value of row 2
    assert_eq!(by_col, [(0, 1), (1, 1)]);
}
