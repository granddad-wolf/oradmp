//! Task T-5: declared types and the type mapping (architecture 5.1, 5.5; ADR-6, ADR-14, ADR-19, ADR-24).
//!
//! Expected numbers come from the Analyst's `docs/formats/tools/collist.py` (run read-only over the 211 fixtures:
//! 404 `CREATE TABLE` lines, 1,981 column definitions, 0 problems, 8 sections "moved") and from the `truth_decl.txt`
//! and `truth_probe11.txt` files; the Arrow types come from architecture 5.1 and ADR-19 (session 4).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;
use common::TOTALS;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use arrow_schema::{DataType, IntervalUnit, TimeUnit};
use oradmp_core::dump::{open, Dump, Options, TableInfo, TableRef};
use oradmp_core::exp::table::{ColumnDesc, PathKind};
use oradmp_core::types::declared::{parse_column_list, DeclaredType, TimestampZone};
use oradmp_core::types::map::{NumberType, TimestampUnit, TypeOptions};
use oradmp_core::types::resolve::{resolve_table, ResolveInput, ResolvedTable};
use oradmp_core::{ErrorCode, Warning, WarningCode};

const FOLDERS: [&str; 3] = ["18c", "11g", "11g-cp1251"];

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixtures() -> PathBuf {
    root().join("corpus/fixtures")
}

fn exp_files() -> Vec<String> {
    let mut v = Vec::new();
    for f in FOLDERS {
        for e in std::fs::read_dir(fixtures().join(f)).unwrap() {
            let name = e.unwrap().file_name().to_string_lossy().into_owned();
            if name.contains("_exp_") && name.ends_with(".dmp") {
                v.push(format!("{f}/{name}"));
            }
        }
    }
    v.sort();
    v
}

/// All 211 fixtures opened and scanned once.
fn scanned() -> &'static BTreeMap<String, Dump> {
    static CELL: OnceLock<BTreeMap<String, Dump>> = OnceLock::new();
    CELL.get_or_init(|| {
        exp_files()
            .into_iter()
            .map(|f| {
                let mut d = open(fixtures().join(&f), Options::default()).unwrap();
                d.scan().unwrap();
                (f, d)
            })
            .collect()
    })
}

fn scan_file(rel: &str, options: Options) -> Dump {
    let mut d = open(fixtures().join(rel), options).unwrap();
    d.scan().unwrap();
    d
}

fn table(d: &Dump, name: &str) -> TableInfo {
    d.catalog()
        .unwrap()
        .tables
        .iter()
        .find(|t| t.name == name)
        .unwrap_or_else(|| panic!("no table {name}"))
        .clone()
}

fn col<'a>(t: &'a TableInfo, name: &str) -> &'a oradmp_core::dump::ColumnInfo {
    t.columns.iter().find(|c| c.name == name).unwrap()
}

fn ddl_warnings(d: &Dump, code: WarningCode) -> Vec<&Warning> {
    d.warnings().iter().filter(|w| w.code == code).collect()
}

fn normalise(text: &[u8], not_null: bool) -> String {
    let mut s = String::new();
    let mut in_digits = false;
    for &b in text {
        if b.is_ascii_digit() {
            if !in_digits {
                s.push('n');
            }
            in_digits = true;
        } else {
            in_digits = false;
            s.push(char::from(b));
        }
    }
    if not_null {
        s.push_str(" NOT NULL ENABLE");
    }
    s
}

// ---------------------------------------------------------------------------------------------
// Done when 1: every CREATE TABLE line parses, in the counts of tools/collist.py, and agrees
// ---------------------------------------------------------------------------------------------

#[test]
fn all_create_table_lines_parse_with_the_per_form_counts_of_collist_py() {
    // collist.py over the 229 files (T-3d; the numbers of T-19b are the tool's printout of that run, "type forms seen" (digit runs replaced by n; the interval text keeps its
    // trailing space, which the tool's printout hides).
    let want: BTreeMap<&str, usize> = [
        ("BLOB", 12),
        ("CHAR(n CHAR)", 16),
        ("CHAR(n)", 53),
        ("CLOB", 21),
        ("DATE", 147),
        ("DATE NOT NULL ENABLE", 2),
        ("FLOAT(n)", 10),
        ("INTERVAL DAY (n) TO SECOND (n)", 6),
        ("INTERVAL YEAR (n) TO MONTH ", 4),
        ("LONG", 24),
        ("LONG RAW", 10),
        ("NCHAR(n)", 4),
        ("NCLOB", 10),
        ("NUMBER", 78),
        ("NUMBER(*,n)", 4),
        ("NUMBER(n, -n)", 6),
        ("NUMBER(n, n)", 351),
        ("NUMBER(n, n) NOT NULL ENABLE", 37),
        ("NVARCHARn(n)", 47),
        ("RAW(n)", 12),
        ("ROWID", 4),
        ("TIMESTAMP (n)", 54),
        ("TIMESTAMP (n) WITH LOCAL TIME ZONE", 16),
        ("TIMESTAMP (n) WITH TIME ZONE", 4),
        ("UROWID(n)", 4),
        ("VARCHARn(n CHAR)", 41),
        ("VARCHARn(n)", 1070),
        ("VARCHARn(n) NOT NULL ENABLE", 28),
    ]
    .into_iter()
    .collect();
    let mut forms: BTreeMap<String, usize> = BTreeMap::new();
    let (mut lines, mut defs) = (0usize, 0usize);
    for (file, d) in scanned() {
        for t in &d.catalog().unwrap().tables {
            let list = parse_column_list(&t.ddl_raw)
                .unwrap_or_else(|e| panic!("{file} {}: {} at +{}", t.name, e.reason, e.offset));
            lines += 1;
            assert_eq!(list.table, t.name_raw, "{file}");
            assert_eq!(list.columns.len(), t.columns.len(), "{file} {}", t.name);
            for c in &list.columns {
                defs += 1;
                assert!(c.kind.is_some(), "{file} {}: type not understood", t.name);
                *forms
                    .entry(normalise(&c.type_text, c.not_null))
                    .or_default() += 1;
            }
        }
    }
    // collist.py: "CREATE TABLE lines 436, column definitions checked against descriptors 2075"
    assert_eq!(
        (lines, defs),
        (TOTALS.sections, TOTALS.column_definitions as usize)
    );
    let got: BTreeMap<&str, usize> = forms.iter().map(|(k, v)| (k.as_str(), *v)).collect();
    assert_eq!(got, want);
}

#[test]
fn every_table_agrees_with_its_descriptors_and_insert_names_without_fallback() {
    let mut moved = 0usize;
    let mut tables = 0usize;
    for (file, d) in scanned() {
        assert!(
            ddl_warnings(d, WarningCode::DdlFallback).is_empty(),
            "{file}: W-DDL-FALLBACK"
        );
        // W-UNTESTED-LAYOUT from the walker has other messages; the one of ADR-24 mentions INSERT INTO
        for w in ddl_warnings(d, WarningCode::UntestedLayout) {
            assert!(!w.message.contains("INSERT INTO order"), "{file}: {w}");
        }
        for t in &d.catalog().unwrap().tables {
            tables += 1;
            assert!(!t.ddl_fallback, "{file} {}", t.name);
            let n = t.columns.len();
            let mut file_positions: Vec<u32> = t.columns.iter().map(|c| c.file_position).collect();
            file_positions.sort_unstable();
            let want: Vec<u32> = (1..=u32::try_from(n).unwrap()).collect();
            assert_eq!(file_positions, want, "{file} {}", t.name);
            for (i, c) in t.columns.iter().enumerate() {
                assert_eq!(usize::try_from(c.position).unwrap(), i + 1);
                assert!(c.declared.is_some(), "{file} {}.{}", t.name, c.name);
            }
            if t.columns.iter().any(|c| c.position != c.file_position) {
                moved += 1;
            }
        }
    }
    assert_eq!(tables, TOTALS.sections);
    // collist.py: "direct path, LONG column moved last in INSERT INTO and descriptors: 12 sections" (T-19: the two
    // long3_exp_direct files add two; T-19b: the long4_exp_direct and long5_exp_direct files of both versions add four)
    assert_eq!(moved, 12);
}

// ---------------------------------------------------------------------------------------------
// Done when 2: T_DECL
// ---------------------------------------------------------------------------------------------

#[test]
fn t_decl_gives_the_14_precision_scale_pairs_of_truth_decl_and_the_arrow_types_of_5_1() {
    let dec = |p, s| DataType::Decimal128(p, s);
    // architecture 5.1 / plan T-5, in the order of truth_decl.txt
    let arrow: [DataType; 14] = [
        dec(38, 10),
        dec(38, 2),
        dec(38, 0),
        DataType::Int64,
        dec(5, 5),
        dec(38, 0),
        dec(10, 2),
        DataType::Float64,
        DataType::Float64,
        DataType::Float64,
        DataType::Timestamp(TimeUnit::Microsecond, None),
        DataType::Timestamp(TimeUnit::Second, None),
        DataType::Utf8,
        DataType::Utf8,
    ];
    for v in ["18c", "11g"] {
        let d = scan_file(&format!("{v}/t_decl_exp_{v}.dmp"), Options::default());
        let t = table(&d, "T_DECL");
        let truth = common::truth_text(v, "truth_decl.txt");
        let rows: Vec<Vec<&str>> = truth.lines().map(|l| l.split('|').collect()).collect();
        assert_eq!((rows.len(), t.columns.len()), (14, 14));
        for (i, r) in rows.iter().enumerate() {
            let c = &t.columns[i];
            assert_eq!(c.name, r[1], "{v}");
            let num = |s: &str| (s != "NULL").then(|| s.parse::<i32>().unwrap());
            let (p, s) = (num(r[3]), num(r[4]));
            let want = match r[2] {
                "NUMBER" => DeclaredType::Number {
                    precision: p.map(|x| u32::try_from(x).unwrap()),
                    scale: s,
                },
                "FLOAT" => DeclaredType::Float {
                    binary_precision: u32::try_from(p.unwrap()).unwrap(),
                },
                "TIMESTAMP(6)" => DeclaredType::Timestamp {
                    fractional: u32::try_from(s.unwrap()).unwrap(),
                    zone: TimestampZone::None,
                },
                "DATE" => DeclaredType::Date,
                "VARCHAR2" => DeclaredType::Varchar2 {
                    n: r[5].parse().unwrap(),
                    char_semantics: false,
                },
                other => panic!("{other}"),
            };
            assert_eq!(c.declared, Some(want), "{v} {}", c.name);
            assert_eq!(c.not_null, r[6] == "N", "{v} {}", c.name);
            assert_eq!(c.arrow_type, arrow[i], "{v} {}", c.name);
            assert_eq!(c.position, c.file_position);
        }
        assert!(col(&t, "D_NN").not_null);
        assert!(!col(&t, "N_PLAIN").not_null);
        assert!(d.warnings().is_empty(), "{v}: {:?}", d.warnings());
    }
}

// ---------------------------------------------------------------------------------------------
// Done when 3: a table-driven test of every row of architecture 5.1
// ---------------------------------------------------------------------------------------------

fn desc(type_code: u16, max_len: u16, charset_id: Option<u16>, form: Option<u16>) -> ColumnDesc {
    ColumnDesc {
        offset: 0x300,
        type_code,
        max_len,
        charset_id,
        form,
    }
}

fn ddl_line(cols: &[(&str, &str)]) -> Vec<u8> {
    let defs: Vec<String> = cols.iter().map(|(n, t)| format!("\"{n}\" {t}")).collect();
    format!(
        "CREATE TABLE \"T\" ({})  PCTFREE 10 INITRANS 1 MAXTRANS 255 STORAGE(INITIAL 65536)",
        defs.join(", ")
    )
    .into_bytes()
}

fn run(
    cols: &[(&str, &str)],
    descs: &[ColumnDesc],
    insert_names: &[&str],
    path: PathKind,
    opts: &TypeOptions,
) -> ResolvedTable {
    let ddl = ddl_line(cols);
    let names: Vec<Vec<u8>> = insert_names.iter().map(|n| n.as_bytes().to_vec()).collect();
    resolve_table(
        &ResolveInput {
            table_ordinal: 3,
            table_name: "T",
            ddl_raw: &ddl,
            ddl_offset: 0x2000,
            descriptors: descs,
            insert_names: &names,
            path,
            client: None,
        },
        opts,
    )
}

fn one(text: &str, d: ColumnDesc, opts: &TypeOptions) -> ResolvedTable {
    run(&[("A", text)], &[d], &["A"], PathKind::Conventional, opts)
}

const U: Option<u16> = Some(873);

#[test]
fn every_row_of_the_architecture_5_1_table_maps_to_its_arrow_type() {
    use DataType as T;
    let dec = |p, s| T::Decimal128(p, s);
    let ts = |u, z: Option<&str>| T::Timestamp(u, z.map(Arc::from));
    let us = TimeUnit::Microsecond;
    let ns = TimeUnit::Nanosecond;
    let n22 = desc(2, 22, None, None);
    let rows: Vec<(&str, ColumnDesc, DataType)> = vec![
        ("NUMBER(5, 0)", n22, T::Int64),
        ("NUMBER(18, 0)", n22, T::Int64),
        ("NUMBER(19, 0)", n22, dec(19, 0)),
        ("NUMBER(38, 0)", n22, dec(38, 0)),
        ("NUMBER(10, 2)", n22, dec(10, 2)),
        ("NUMBER(3, 5)", n22, dec(5, 5)),
        ("NUMBER(5, -2)", n22, T::Int64),
        ("NUMBER(16, -2)", n22, T::Int64),
        ("NUMBER(17, -2)", n22, dec(19, 0)),
        ("NUMBER(36, -2)", n22, dec(38, 0)),
        // p - s = 40 does not fit Decimal128: "as NUMBER" with W-DDL-FALLBACK (ADR-30 item 3)
        ("NUMBER(38, -2)", n22, dec(38, 10)),
        ("NUMBER(30, -10)", n22, dec(38, 10)),
        ("NUMBER(*,0)", n22, dec(38, 0)),
        ("NUMBER(*,2)", n22, dec(38, 2)),
        ("NUMBER(*,-2)", n22, dec(38, 10)),
        ("NUMBER", n22, dec(38, 10)),
        ("FLOAT(126)", n22, T::Float64),
        ("FLOAT(63)", n22, T::Float64),
        ("DATE", desc(12, 7, None, None), ts(TimeUnit::Second, None)),
        ("TIMESTAMP (0)", desc(180, 7, None, None), ts(us, None)),
        ("TIMESTAMP (6)", desc(180, 11, None, None), ts(us, None)),
        ("TIMESTAMP (7)", desc(180, 11, None, None), ts(ns, None)),
        ("TIMESTAMP (9)", desc(180, 11, None, None), ts(ns, None)),
        (
            "TIMESTAMP (6) WITH TIME ZONE",
            desc(181, 13, None, None),
            ts(us, Some("UTC")),
        ),
        (
            "TIMESTAMP (9) WITH TIME ZONE",
            desc(181, 13, None, None),
            ts(ns, Some("UTC")),
        ),
        (
            "TIMESTAMP (6) WITH LOCAL TIME ZONE",
            desc(231, 11, None, None),
            ts(us, None),
        ),
        (
            "TIMESTAMP (8) WITH LOCAL TIME ZONE",
            desc(231, 11, None, None),
            ts(ns, None),
        ),
        ("VARCHAR2(10)", desc(1, 10, U, Some(1)), T::Utf8),
        ("VARCHAR2(10 CHAR)", desc(1, 40, U, Some(1)), T::Utf8),
        ("VARCHAR2(1001 CHAR)", desc(1, 4000, U, Some(1)), T::Utf8),
        (
            "VARCHAR2(20 CHAR)",
            desc(1, 20, Some(171), Some(1)),
            T::Utf8,
        ),
        ("CHAR(3)", desc(96, 3, U, Some(1)), T::Utf8),
        ("CHAR(3 CHAR)", desc(96, 12, U, Some(1)), T::Utf8),
        ("CHAR(501 CHAR)", desc(96, 2000, U, Some(1)), T::Utf8),
        ("NVARCHAR2(5)", desc(1, 10, Some(2000), Some(2)), T::Utf8),
        ("NCHAR(5)", desc(96, 10, Some(2000), Some(2)), T::Utf8),
        ("RAW(16)", desc(23, 16, None, None), T::Binary),
        ("CLOB", desc(112, 72, U, Some(1)), T::LargeUtf8),
        ("CLOB", desc(112, 72, Some(171), Some(1)), T::LargeUtf8),
        ("NCLOB", desc(112, 72, Some(2000), Some(2)), T::LargeUtf8),
        ("BLOB", desc(113, 72, None, None), T::LargeBinary),
        ("LONG", desc(8, 0, None, None), T::LargeUtf8),
        ("LONG RAW", desc(24, 0, None, None), T::LargeBinary),
        (
            "INTERVAL YEAR (4) TO MONTH ",
            desc(182, 5, None, None),
            T::Interval(IntervalUnit::MonthDayNano),
        ),
        ("ROWID", desc(69, 23, None, None), T::Utf8),
        ("UROWID(4000)", desc(208, 4000, None, None), T::Utf8),
    ];
    for (text, d, want) in rows {
        let r = one(text, d, &TypeOptions::default());
        // ADR-30 item 3 and ADR-27 item 2: the two warnings this table can raise
        let want_codes: &[WarningCode] = if text.ends_with("WITH LOCAL TIME ZONE") {
            &[WarningCode::Tsltz]
        } else if ["NUMBER(38, -2)", "NUMBER(30, -10)", "NUMBER(*,-2)"].contains(&text) {
            &[WarningCode::DdlFallback]
        } else {
            &[]
        };
        let codes: Vec<WarningCode> = r.warnings.iter().map(|w| w.code).collect();
        assert_eq!(codes, want_codes, "{text}: {:?}", r.warnings);
        assert!(!r.fallback, "{text}");
        assert_eq!(r.columns.len(), 1);
        assert_eq!(r.columns[0].arrow_type, want, "{text}");
        assert!(r.columns[0].declared.is_some(), "{text}");
    }
}

#[test]
fn interval_day_to_second_follows_p_and_f_as_adr_19_session_4() {
    use DataType as T;
    // (p, f, type): ADR-19 session 4. misc: (4, 6) microseconds; misc2: (9, 9) and (2, 0).
    let cases = [
        (4, 6, T::Duration(TimeUnit::Microsecond)),
        (2, 0, T::Duration(TimeUnit::Microsecond)),
        (8, 6, T::Duration(TimeUnit::Microsecond)),
        (9, 6, T::Interval(IntervalUnit::MonthDayNano)),
        (9, 9, T::Interval(IntervalUnit::MonthDayNano)),
        (5, 7, T::Duration(TimeUnit::Nanosecond)),
        (5, 9, T::Duration(TimeUnit::Nanosecond)),
        (6, 9, T::Interval(IntervalUnit::MonthDayNano)),
        (6, 7, T::Interval(IntervalUnit::MonthDayNano)),
    ];
    for (p, f, want) in cases {
        let text = format!("INTERVAL DAY ({p}) TO SECOND ({f})");
        let r = one(&text, desc(183, 11, None, None), &TypeOptions::default());
        assert!(r.warnings.is_empty(), "{text}");
        assert_eq!(r.columns[0].arrow_type, want, "{text}");
    }
}

#[test]
fn interval_and_rowid_tables_of_misc_and_misc2_in_both_versions() {
    for v in ["18c", "11g"] {
        let d = scan_file(&format!("{v}/misc_exp_{v}.dmp"), Options::default());
        let t = table(&d, "T_INTERVAL");
        assert_eq!(
            col(&t, "IYM").arrow_type,
            DataType::Interval(IntervalUnit::MonthDayNano)
        );
        assert_eq!(
            col(&t, "IDS").arrow_type,
            DataType::Duration(TimeUnit::Microsecond)
        );
        assert_eq!(
            col(&t, "IDS").declared,
            Some(DeclaredType::IntervalDayToSecond {
                precision: 4,
                fractional: 6
            })
        );
        let t = table(&d, "T_ROWID");
        assert_eq!(col(&t, "RID").arrow_type, DataType::Utf8);
        assert_eq!(col(&t, "URID").arrow_type, DataType::Utf8);
        assert!(d.warnings().is_empty());

        let d = scan_file(&format!("{v}/misc2_exp_{v}.dmp"), Options::default());
        let t = table(&d, "T_INTERVAL2");
        assert_eq!(
            col(&t, "IDS").arrow_type,
            DataType::Interval(IntervalUnit::MonthDayNano)
        );
        assert_eq!(
            col(&t, "IDS0").arrow_type,
            DataType::Duration(TimeUnit::Microsecond)
        );
        assert!(d.warnings().is_empty());
    }
}

#[test]
fn lob_tables_of_lob_exp_map_clob_blob_nclob() {
    for v in ["18c", "11g"] {
        let d = scan_file(&format!("{v}/lob_exp_{v}.dmp"), Options::default());
        let t = table(&d, "T_LOB");
        assert_eq!(col(&t, "C").arrow_type, DataType::LargeUtf8);
        assert_eq!(col(&t, "B").arrow_type, DataType::LargeBinary);
        let t = table(&d, "T_LOB2");
        assert_eq!(col(&t, "A").arrow_type, DataType::LargeUtf8);
        assert_eq!(col(&t, "N").arrow_type, DataType::LargeUtf8);
        assert_eq!(col(&t, "V").arrow_type, DataType::Utf8);
        assert!(d.warnings().is_empty());
    }
}

#[test]
fn a_clob_in_an_unsupported_set_is_binary_with_a_warning() {
    let r = one(
        "CLOB",
        desc(112, 72, Some(1000), Some(1)),
        &TypeOptions::default(),
    );
    assert_eq!(r.columns[0].arrow_type, DataType::LargeBinary);
    assert_eq!(r.warnings.len(), 1);
    let w = &r.warnings[0];
    assert_eq!(w.code, WarningCode::CharsetUnsupported);
    assert_eq!(
        (w.table, w.column, w.offset),
        (Some(3), Some(1), Some(0x300))
    );
}

// ---------------------------------------------------------------------------------------------
// Options: number_default, number_types, timestamp_unit
// ---------------------------------------------------------------------------------------------

#[test]
fn number_default_changes_only_columns_without_a_declared_precision() {
    let cols = [
        ("A", "NUMBER"),
        ("B", "NUMBER(10, 2)"),
        ("C", "NUMBER(5, 0)"),
        ("D", "FLOAT(126)"),
    ];
    let d = [desc(2, 22, None, None); 4];
    for (text, want) in [
        ("float64", DataType::Float64),
        ("string", DataType::Utf8),
        ("decimal(20,4)", DataType::Decimal128(20, 4)),
        ("decimal256(76,38)", DataType::Decimal256(76, 38)),
        ("decimal256(76, 40)", DataType::Decimal256(76, 40)),
    ] {
        let opts = TypeOptions {
            number_default: text.parse().unwrap(),
            ..TypeOptions::default()
        };
        let r = run(
            &cols,
            &d,
            &["A", "B", "C", "D"],
            PathKind::Conventional,
            &opts,
        );
        let got: Vec<DataType> = r.columns.iter().map(|c| c.arrow_type.clone()).collect();
        assert_eq!(
            got,
            [
                want.clone(),
                DataType::Decimal128(10, 2),
                DataType::Int64,
                DataType::Float64
            ],
            "{text}"
        );
    }
}

#[test]
fn number_types_override_any_type_2_column_by_table_and_column_name() {
    let mut opts = TypeOptions::default();
    opts.number_types
        .insert("T.B".into(), "decimal256(76,38)".parse().unwrap());
    opts.number_types
        .insert("T.C".into(), "int64".parse().unwrap());
    opts.number_types
        .insert("T.D".into(), "string".parse().unwrap());
    opts.number_types
        .insert("OTHER.A".into(), "float64".parse().unwrap());
    let cols = [
        ("A", "NUMBER"),
        ("B", "NUMBER"),
        ("C", "NUMBER(*,0)"),
        ("D", "FLOAT(126)"),
    ];
    let r = run(
        &cols,
        &[desc(2, 22, None, None); 4],
        &["A", "B", "C", "D"],
        PathKind::Conventional,
        &opts,
    );
    let got: Vec<DataType> = r.columns.iter().map(|c| c.arrow_type.clone()).collect();
    assert_eq!(
        got,
        [
            DataType::Decimal128(38, 10),
            DataType::Decimal256(76, 38),
            DataType::Int64,
            DataType::Utf8
        ]
    );
    // the override also reaches a column that fell back
    let r = run(
        &[("A", "NUMBER"), ("B", "ZZZ")],
        &[desc(2, 22, None, None); 2],
        &["A", "B"],
        PathKind::Conventional,
        &opts,
    );
    assert_eq!(r.columns[1].arrow_type, DataType::Decimal256(76, 38));
}

#[test]
fn number_type_option_strings() {
    use std::str::FromStr;
    assert_eq!(
        NumberType::from_str("decimal(38,10)").unwrap(),
        NumberType::DEFAULT
    );
    assert_eq!(
        NumberType::from_str(" DECIMAL256(76,38) ").unwrap(),
        NumberType::Decimal256 {
            precision: 76,
            scale: 38
        }
    );
    for bad in [
        "decimal(39,0)",
        "decimal(0,0)",
        "decimal(5,6)",
        "decimal256(77,0)",
        "decimal(5)",
        "float32",
        "",
        "decimal(5,-1)",
    ] {
        let e = NumberType::from_str(bad).unwrap_err();
        assert_eq!(e.code, ErrorCode::Config, "{bad}");
    }
}

#[test]
fn timestamp_unit_option() {
    let d = desc(180, 11, None, None);
    for (unit, f, want) in [
        (TimestampUnit::Auto, 6, TimeUnit::Microsecond),
        (TimestampUnit::Auto, 9, TimeUnit::Nanosecond),
        (TimestampUnit::Micro, 9, TimeUnit::Microsecond),
        (TimestampUnit::Nano, 3, TimeUnit::Nanosecond),
    ] {
        let opts = TypeOptions {
            timestamp_unit: unit,
            ..TypeOptions::default()
        };
        let r = one(&format!("TIMESTAMP ({f})"), d, &opts);
        assert_eq!(r.columns[0].arrow_type, DataType::Timestamp(want, None));
    }
}

// ---------------------------------------------------------------------------------------------
// Done when 4: fallbacks, never an error
// ---------------------------------------------------------------------------------------------

#[test]
fn an_unknown_type_text_falls_back_for_that_column_only() {
    let r = run(
        &[("A", "NUMBER(5, 0)"), ("B", "BFILE"), ("C", "DATE")],
        &[
            desc(2, 22, None, None),
            desc(2, 22, None, None),
            desc(12, 7, None, None),
        ],
        &["A", "B", "C"],
        PathKind::Conventional,
        &TypeOptions::default(),
    );
    assert!(!r.fallback);
    assert_eq!(r.columns[0].arrow_type, DataType::Int64);
    assert_eq!(r.columns[1].declared, None);
    assert_eq!(r.columns[1].type_text.as_deref(), Some(&b"BFILE"[..]));
    assert_eq!(r.columns[1].arrow_type, DataType::Decimal128(38, 10));
    assert_eq!(
        r.columns[2].arrow_type,
        DataType::Timestamp(TimeUnit::Second, None)
    );
    assert_eq!(r.warnings.len(), 1);
    let w = &r.warnings[0];
    assert_eq!(w.code, WarningCode::DdlFallback);
    assert_eq!((w.table, w.column), (Some(3), Some(2)));
    // offset of the opening quote of "B": CREATE TABLE "T" ("A" NUMBER(5, 0), "B" ...
    let line = ddl_line(&[("A", "NUMBER(5, 0)"), ("B", "BFILE"), ("C", "DATE")]);
    let at = line.windows(4).position(|w| w == b"\"B\" ").unwrap();
    assert_eq!(w.offset, Some(0x2000 + at as u64));
}

#[test]
fn a_name_that_differs_or_a_missing_definition_falls_back_for_the_table() {
    let d = [desc(2, 22, None, None), desc(12, 7, None, None)];
    let cols = [("A", "NUMBER(5, 0)"), ("B", "DATE")];
    // name differs
    let r = run(
        &cols,
        &d,
        &["A", "C"],
        PathKind::Conventional,
        &TypeOptions::default(),
    );
    // one definition too few
    let r2 = run(
        &cols[..1],
        &d,
        &["A", "B"],
        PathKind::Conventional,
        &TypeOptions::default(),
    );
    // the line does not parse
    let mut line = ddl_line(&cols);
    line.truncate(30);
    let names = vec![b"A".to_vec(), b"B".to_vec()];
    let r3 = resolve_table(
        &ResolveInput {
            table_ordinal: 3,
            table_name: "T",
            ddl_raw: &line,
            ddl_offset: 0x2000,
            descriptors: &d,
            insert_names: &names,
            path: PathKind::Conventional,
            client: None,
        },
        &TypeOptions::default(),
    );
    // two INSERT INTO names equal
    let r4 = run(
        &cols,
        &d,
        &["A", "A"],
        PathKind::Conventional,
        &TypeOptions::default(),
    );
    for (i, r) in [r, r2, r3, r4].iter().enumerate() {
        assert!(r.fallback, "case {i}");
        assert_eq!(r.warnings.len(), 1, "case {i}");
        assert_eq!(r.warnings[0].code, WarningCode::DdlFallback);
        assert_eq!(r.warnings[0].table, Some(3));
        assert_eq!(r.columns.len(), 2);
        // file order, mapped from the descriptors
        assert_eq!(
            r.columns[0].arrow_type,
            DataType::Decimal128(38, 10),
            "case {i}"
        );
        assert_eq!(
            r.columns[1].arrow_type,
            DataType::Timestamp(TimeUnit::Second, None)
        );
        assert!(r.columns.iter().all(|c| c.declared.is_none()));
        assert_eq!(
            r.columns.iter().map(|c| c.file_index).collect::<Vec<_>>(),
            [0, 1]
        );
    }
}

#[test]
fn a_family_or_length_that_disagrees_with_the_descriptor_falls_back_for_the_column() {
    let cases = [
        ("VARCHAR2(10)", desc(1, 11, U, Some(1))),
        ("VARCHAR2(10)", desc(2, 22, None, None)),
        ("NCHAR(5)", desc(96, 5, Some(2000), Some(2))),
        ("RAW(16)", desc(23, 15, None, None)),
        ("TIMESTAMP (6)", desc(180, 7, None, None)),
        ("NUMBER(5, 0)", desc(2, 21, None, None)),
        ("CLOB", desc(112, 71, U, Some(1))),
        ("INTERVAL YEAR (4) TO MONTH ", desc(182, 11, None, None)),
        // out of range
        ("NUMBER(39, 0)", desc(2, 22, None, None)),
        ("NUMBER(0, 0)", desc(2, 22, None, None)),
        ("FLOAT(127)", desc(2, 22, None, None)),
        ("TIMESTAMP (10)", desc(180, 11, None, None)),
        ("INTERVAL DAY (10) TO SECOND (6)", desc(183, 11, None, None)),
        // character semantics with a wrong length
        ("VARCHAR2(10 CHAR)", desc(1, 10, U, Some(1))),
        ("CHAR(501 CHAR)", desc(96, 2004, U, Some(1))),
    ];
    for (text, d) in cases {
        let r = one(text, d, &TypeOptions::default());
        assert!(!r.fallback, "{text}");
        assert_eq!(r.warnings.len(), 1, "{text}");
        assert_eq!(r.warnings[0].code, WarningCode::DdlFallback, "{text}");
        assert_eq!(r.warnings[0].column, Some(1));
        assert_eq!(r.columns[0].declared, None, "{text}");
        assert_eq!(r.columns[0].type_text, Some(text.as_bytes().to_vec()));
    }
}

#[test]
fn not_null_enable_after_an_interval_text_is_a_column_fallback_r_039() {
    for (text, d) in [
        ("INTERVAL YEAR (4) TO MONTH ", desc(182, 5, None, None)),
        ("INTERVAL DAY (4) TO SECOND (6)", desc(183, 11, None, None)),
    ] {
        let line =
            format!("CREATE TABLE \"T\" (\"A\" {text}NOT NULL ENABLE, \"B\" DATE)  PCTFREE 10");
        let line = if text.ends_with(' ') {
            line
        } else {
            line.replace("(6)NOT", "(6) NOT")
        };
        let names = vec![b"A".to_vec(), b"B".to_vec()];
        let descs = [d, desc(12, 7, None, None)];
        let r = resolve_table(
            &ResolveInput {
                table_ordinal: 1,
                table_name: "T",
                ddl_raw: line.as_bytes(),
                ddl_offset: 0,
                descriptors: &descs,
                insert_names: &names,
                path: PathKind::Conventional,
                client: None,
            },
            &TypeOptions::default(),
        );
        assert!(!r.fallback, "{text}");
        assert_eq!(r.columns[0].declared, None, "{text}");
        assert!(r.columns[0].not_null);
        assert_eq!(r.columns[1].declared, Some(DeclaredType::Date), "{text}");
        assert_eq!(r.warnings.len(), 1);
        assert_eq!(r.warnings[0].column, Some(1));
    }
}

// ---------------------------------------------------------------------------------------------
// Done when 5: matching by name, declared order (ADR-24)
// ---------------------------------------------------------------------------------------------

#[test]
fn t_long_tables_of_the_direct_files_have_the_declared_order_id_l_v_without_warning() {
    for v in ["18c", "11g"] {
        // T_LONG: ID, L (LONG), V. The direct path writes ID, V, L (rules T3, T9; collist.py "moved").
        let d = scan_file(&format!("{v}/long_exp_direct_{v}.dmp"), Options::default());
        let t = table(&d, "T_LONG");
        let order: Vec<(&str, u32, u32)> = t
            .columns
            .iter()
            .map(|c| (c.name.as_str(), c.position, c.file_position))
            .collect();
        assert_eq!(order, [("ID", 1, 1), ("L", 2, 3), ("V", 3, 2)], "{v}");
        assert_eq!(col(&t, "L").arrow_type, DataType::LargeUtf8);
        assert_eq!(col(&t, "L").type_code, 8);
        assert_eq!(col(&t, "V").arrow_type, DataType::Utf8);
        let t = table(&d, "T_LONG_EMPTY");
        let order: Vec<(&str, u32, u32)> = t
            .columns
            .iter()
            .map(|c| (c.name.as_str(), c.position, c.file_position))
            .collect();
        assert_eq!(order, [("L", 1, 2), ("V", 2, 1)], "{v}");
        let t = table(&d, "T_LONGRAW");
        assert_eq!(col(&t, "LR").arrow_type, DataType::LargeBinary);
        assert_eq!(
            t.columns
                .iter()
                .map(|c| c.position == c.file_position)
                .collect::<Vec<_>>(),
            [true, true]
        );
        assert!(d.warnings().is_empty(), "{v}: {:?}", d.warnings());

        let d = scan_file(&format!("{v}/long2_exp_direct_{v}.dmp"), Options::default());
        let t = table(&d, "T_LONG2");
        let order: Vec<(&str, u32, u32)> = t
            .columns
            .iter()
            .map(|c| (c.name.as_str(), c.position, c.file_position))
            .collect();
        assert_eq!(order, [("ID", 1, 1), ("L", 2, 3), ("V", 3, 2)], "{v}");
        assert!(d.warnings().is_empty(), "{v}: {:?}", d.warnings());

        // the conventional export of the same table: one schema, the same declared order
        let d = scan_file(&format!("{v}/long_exp_{v}.dmp"), Options::default());
        let t = table(&d, "T_LONG");
        let names: Vec<&str> = t.columns.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["ID", "L", "V"]);
        assert!(t.columns.iter().all(|c| c.position == c.file_position));
    }
}

#[test]
fn a_cursor_on_a_moved_table_reads_its_rows_in_file_order() {
    // Without the sort by file_position the LONG column would be read as the second field and the
    // framing would break on the first row.
    for v in ["18c", "11g"] {
        let mut d = scan_file(&format!("{v}/long_exp_direct_{v}.dmp"), Options::default());
        let t = table(&d, "T_LONG");
        let mut rows = d.rows(TableRef::ByName("T_LONG")).unwrap();
        while rows.next_row().unwrap().is_some() {}
        assert_eq!(rows.rows_read(), t.num_rows);
        assert_eq!(rows.offset(), t.parts[0].rows_end_offset);
        assert_eq!(t.num_rows, 3);
    }
}

#[test]
fn any_other_insert_order_is_read_by_name_with_one_untested_layout_warning() {
    let cols = [
        ("A", "VARCHAR2(10)"),
        ("B", "NUMBER(5, 0)"),
        ("C", "VARCHAR2(10)"),
    ];
    let v10 = desc(1, 10, U, Some(1));
    let n = desc(2, 22, None, None);
    // INSERT INTO order B, A, C with descriptors in that order
    let r = run(
        &cols,
        &[n, v10, v10],
        &["B", "A", "C"],
        PathKind::Conventional,
        &TypeOptions::default(),
    );
    assert!(!r.fallback);
    let order: Vec<(usize, DataType)> = r
        .columns
        .iter()
        .map(|c| (c.file_index, c.arrow_type.clone()))
        .collect();
    assert_eq!(
        order,
        [
            (1, DataType::Utf8),
            (0, DataType::Int64),
            (2, DataType::Utf8)
        ]
    );
    assert_eq!(r.warnings.len(), 1);
    assert_eq!(r.warnings[0].code, WarningCode::UntestedLayout);
    assert_eq!(r.warnings[0].table, Some(3));
    assert!(r.warnings[0].message.contains("INSERT INTO order"));

    // a LONG column last on the direct path: the confirmed shape, no warning
    let cols = [("ID", "NUMBER(3, 0)"), ("L", "LONG"), ("V", "VARCHAR2(10)")];
    let long = desc(8, 0, None, None);
    let id = desc(2, 22, None, None);
    let args = |path| {
        run(
            &cols,
            &[id, v10, long],
            &["ID", "V", "L"],
            path,
            &TypeOptions::default(),
        )
    };
    let r = args(PathKind::Direct);
    assert!(r.warnings.is_empty(), "{:?}", r.warnings);
    assert_eq!(
        r.columns.iter().map(|c| c.file_index).collect::<Vec<_>>(),
        [0, 2, 1]
    );
    // the same order on the conventional path, or any other permutation on the direct path
    assert_eq!(args(PathKind::Conventional).warnings.len(), 1);
    let r = run(
        &cols,
        &[v10, id, long],
        &["V", "ID", "L"],
        PathKind::Direct,
        &TypeOptions::default(),
    );
    assert_eq!(r.warnings.len(), 1);
    assert_eq!(r.warnings[0].code, WarningCode::UntestedLayout);
}

// ---------------------------------------------------------------------------------------------
// Done when 6: n CHAR lengths on t_probe11
// ---------------------------------------------------------------------------------------------

#[test]
fn t_probe11_char_semantics_equal_truth_probe11_col_lines() {
    let mut files = 0usize;
    for v in ["18c", "11g"] {
        let truth = common::truth_text(v, "truth_probe11.txt");
        let cols: Vec<Vec<&str>> = truth
            .lines()
            .filter(|l| l.starts_with("COL|"))
            .map(|l| l.split('|').collect())
            .collect();
        assert_eq!(cols.len(), 8, "ID plus the 7 columns of the plan");
        for name in [
            format!("t_probe11_exp_{v}.dmp"),
            format!("t_probe11_exp_direct_{v}.dmp"),
        ] {
            let d = scan_file(&format!("{v}/{name}"), Options::default());
            let t = table(&d, "T_PROBE11");
            assert_eq!(t.columns.len(), 8);
            for r in &cols {
                // COL|name|data_type|data_length|char_length|char_used
                let c = col(&t, r[1]);
                assert_eq!(
                    u32::from(c.max_bytes),
                    r[3].parse::<u32>().unwrap(),
                    "{name} {}",
                    r[1]
                );
                match (r[2], c.declared.unwrap()) {
                    ("NUMBER", DeclaredType::Number { .. }) => {}
                    ("VARCHAR2", DeclaredType::Varchar2 { n, char_semantics }) => {
                        assert_eq!(n, r[4].parse::<u32>().unwrap(), "{name} {}", r[1]);
                        assert_eq!(char_semantics, r[5] == "C", "{name} {}", r[1]);
                    }
                    ("CHAR", DeclaredType::Char { n, char_semantics }) => {
                        assert_eq!(n, r[4].parse::<u32>().unwrap(), "{name} {}", r[1]);
                        assert_eq!(char_semantics, r[5] == "C", "{name} {}", r[1]);
                    }
                    other => panic!("{name} {}: {other:?}", r[1]),
                }
            }
            assert!(d.warnings().is_empty(), "{name}: {:?}", d.warnings());
            files += 1;
        }
    }
    assert_eq!(files, 4);
}

// ---------------------------------------------------------------------------------------------
// Patched files: warnings reach the dump, offsets point at the definition
// ---------------------------------------------------------------------------------------------

fn patched_copy(rel: &str, name: &str, from: &[u8], to: &[u8]) -> PathBuf {
    assert_eq!(from.len(), to.len());
    let mut bytes = std::fs::read(fixtures().join(rel)).unwrap();
    let at = bytes.windows(from.len()).position(|w| w == from).unwrap();
    bytes[at..at + to.len()].copy_from_slice(to);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("t5");
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join(name);
    std::fs::write(&p, bytes).unwrap();
    p
}

#[test]
fn a_patched_type_text_in_a_file_gives_a_column_fallback_warning_at_the_definition() {
    let p = patched_copy(
        "18c/t_decl_exp_18c.dmp",
        "type_text.dmp",
        b"NUMBER(10, 2)",
        b"NUMBZR(10, 2)",
    );
    let mut d = open(&p, Options::default()).unwrap();
    d.scan().unwrap();
    let t = table(&d, "T_DECL");
    assert!(!t.ddl_fallback);
    let c = col(&t, "D_DEC");
    assert_eq!(
        (c.declared, c.arrow_type.clone()),
        (None, DataType::Decimal128(38, 10))
    );
    assert_eq!(c.declared_text.as_deref(), Some("NUMBZR(10, 2)"));
    let w = ddl_warnings(&d, WarningCode::DdlFallback);
    assert_eq!(w.len(), 1);
    assert_eq!((w[0].table, w[0].column), (Some(t.ordinal), Some(7)));
    let bytes = std::fs::read(&p).unwrap();
    let at = usize::try_from(w[0].offset.unwrap()).unwrap();
    assert!(bytes[at..].starts_with(b"\"D_DEC\" NUMBZR"));
    // the other columns keep their declared types
    assert_eq!(col(&t, "N_38").arrow_type, DataType::Decimal128(38, 0));
}

#[test]
fn a_patched_name_in_a_file_gives_a_table_fallback_in_insert_order() {
    let p = patched_copy(
        "18c/t_decl_exp_18c.dmp",
        "name.dmp",
        b"\"V_DEF\" VARCHAR2",
        b"\"V_DEG\" VARCHAR2",
    );
    let mut d = open(&p, Options::default()).unwrap();
    d.scan().unwrap();
    let t = table(&d, "T_DECL");
    assert!(t.ddl_fallback);
    assert_eq!(t.columns.len(), 14);
    assert!(t.columns.iter().all(|c| c.declared.is_none()));
    assert!(t.columns.iter().all(|c| c.position == c.file_position));
    // every NUMBER is now the default, whatever it declared
    assert_eq!(col(&t, "N_38").arrow_type, DataType::Decimal128(38, 10));
    assert_eq!(col(&t, "F_126").arrow_type, DataType::Decimal128(38, 10));
    assert_eq!(
        col(&t, "TS_DEF").arrow_type,
        DataType::Timestamp(TimeUnit::Microsecond, None)
    );
    let w = ddl_warnings(&d, WarningCode::DdlFallback);
    assert_eq!(w.len(), 1);
    assert_eq!(w[0].column, None);
    assert_eq!(w[0].offset, Some(t.create_offset));
    // rows are still readable
    let mut rows = d.rows(TableRef::ByName("T_DECL")).unwrap();
    while rows.next_row().unwrap().is_some() {}
    assert_eq!(rows.rows_read(), t.num_rows);
}

#[test]
fn options_reach_the_catalog_through_open() {
    let options = Options {
        types: TypeOptions {
            number_default: NumberType::Float64,
            number_types: [("T_DECL.N_38".to_string(), NumberType::Utf8)].into(),
            timestamp_unit: TimestampUnit::Nano,
            ..TypeOptions::default()
        },
        ..Options::default()
    };
    let d = scan_file("18c/t_decl_exp_18c.dmp", options);
    let t = table(&d, "T_DECL");
    assert_eq!(col(&t, "N_PLAIN").arrow_type, DataType::Float64);
    assert_eq!(col(&t, "N_38").arrow_type, DataType::Utf8);
    assert_eq!(col(&t, "D_DEC").arrow_type, DataType::Decimal128(10, 2));
    assert_eq!(
        col(&t, "TS_DEF").arrow_type,
        DataType::Timestamp(TimeUnit::Nanosecond, None)
    );
}

// ---------------------------------------------------------------------------------------------
// Parser details and hostile input
// ---------------------------------------------------------------------------------------------

#[test]
fn the_parser_reads_names_with_commas_and_parentheses_and_the_interval_space() {
    let line = b"CREATE TABLE \"a\"\"b\" (\"low,er(x\" VARCHAR2(5), \"I\" INTERVAL YEAR (2) TO MONTH , \"J\" NUMBER(3, 0) NOT NULL ENABLE)  PCTFREE 10";
    // a name has no escape for a quote: it ends at the next quote, so this table name breaks the grammar
    assert!(parse_column_list(line).is_err());
    let line = b"CREATE TABLE \"T\" (\"low,er(x\" VARCHAR2(5), \"I\" INTERVAL YEAR (2) TO MONTH , \"J\" NUMBER(3, 0) NOT NULL ENABLE)  PCTFREE 10";
    let l = parse_column_list(line).unwrap();
    assert_eq!(l.columns.len(), 3);
    assert_eq!(l.columns[0].name, b"low,er(x");
    assert_eq!(l.columns[1].type_text, b"INTERVAL YEAR (2) TO MONTH ");
    assert_eq!(
        l.columns[1].kind,
        Some(DeclaredType::IntervalYearToMonth { precision: 2 })
    );
    assert!(l.columns[2].not_null);
    assert_eq!(&line[l.storage_offset..], b"PCTFREE 10");
}

#[test]
fn parse_failures_name_the_offset_and_never_panic() {
    for line in [
        &b""[..],
        b"CREATE TABLE",
        b"CREATE TABLE \"T\"",
        b"CREATE TABLE \"T\" (",
        b"CREATE TABLE \"T\" (\"A\"",
        b"CREATE TABLE \"T\" (\"A\" ",
        b"CREATE TABLE \"T\" (\"A\" NUMBER",
        b"CREATE TABLE \"T\" (\"A\" NUMBER(99999999999999999999, 1))  X",
        b"CREATE TABLE \"T\" (\"A\" NUMBER) X",
        b"CREATE TABLE \"T\" (\"A\" NUMBER,\"B\" DATE)  X",
        b"CREATE TABLE \"T\" (\"A\" NUMBER(5, -99999999999))  X",
    ] {
        match parse_column_list(line) {
            Err(f) => assert!(f.offset <= line.len()),
            Ok(l) => {
                // a huge number is "outside the forms", not a failure
                assert!(l.columns.iter().any(|c| c.kind.is_none()), "{:?}", line);
            }
        }
    }
}

proptest::proptest! {
    #![proptest_config(proptest::prelude::ProptestConfig {
        failure_persistence: None,
        ..proptest::prelude::ProptestConfig::default()
    })]

    #[test]
    fn arbitrary_bytes_never_panic_the_parser_or_the_resolver(
        tail in proptest::collection::vec(proptest::prelude::any::<u8>(), 0..120),
        cut in 0usize..90,
    ) {
        let mut line = ddl_line(&[("A", "NUMBER(5, 0)"), ("B", "VARCHAR2(10 CHAR)"), ("C", "TIMESTAMP (6) WITH TIME ZONE")]);
        line.truncate(cut);
        line.extend_from_slice(&tail);
        let _ = parse_column_list(&line);
        let d = [desc(2, 22, None, None), desc(1, 40, U, Some(1)), desc(181, 13, None, None)];
        let names = vec![b"A".to_vec(), b"B".to_vec(), b"C".to_vec()];
        let r = resolve_table(
            &ResolveInput {
                table_ordinal: 1,
                table_name: "T",
                ddl_raw: &line,
                ddl_offset: 0,
                descriptors: &d,
                insert_names: &names,
                path: PathKind::Direct,
                client: None,
            },
            &TypeOptions::default(),
        );
        proptest::prop_assert_eq!(r.columns.len(), 3);
    }
}
