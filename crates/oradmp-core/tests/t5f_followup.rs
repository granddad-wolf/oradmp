//! Task T-5f: the T-5 follow-up and the session 6 fixes (ADR-30 items 1 and 3, ADR-27 item 2, ADR-33 items 3
//! and 4, ADR-32 item 3).
//!
//! Expected numbers: the test vectors are those of ADR-30 item 1; the four declarations are those of the plan;
//! the 6 rounded values of `t_num3_exp_18c` are the 6 rows of `truth_num3.txt`; `T_PART` has 3 parts and `T_PART2`
//! has 6 (the T-3b catalog, `docs/formats/tools/exp_parse.py` through `tests/golden/exp_raw.tsv`: 3 and 6 part
//! lines). Nothing here is read from a third-party parser.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::path::PathBuf;

use arrow_schema::DataType;
use common::*;
use oradmp_core::dump::{open, Options, TableRef};
use oradmp_core::exp::table::{ColumnDesc, PathKind};
use oradmp_core::reader::ReadOptions;
use oradmp_core::types::declared::{parse_column_list, DeclaredType};
use oradmp_core::types::map::{NumberType, TypeOptions, TypeSource};
use oradmp_core::types::resolve::{resolve_table, ResolveInput, ResolvedTable};
use oradmp_core::values::number::OverflowPolicy;
use oradmp_core::{ErrorCode, WarningCode};

fn desc(type_code: u16, max_len: u16) -> ColumnDesc {
    ColumnDesc {
        offset: 0x300,
        type_code,
        max_len,
        charset_id: None,
        form: None,
    }
}

fn resolve(line: &str, descs: &[ColumnDesc], names: &[&str], opts: &TypeOptions) -> ResolvedTable {
    let names: Vec<Vec<u8>> = names.iter().map(|n| n.as_bytes().to_vec()).collect();
    resolve_table(
        &ResolveInput {
            table_ordinal: 4,
            table_name: "T",
            ddl_raw: line.as_bytes(),
            ddl_offset: 0x2000,
            descriptors: descs,
            insert_names: &names,
            path: PathKind::Conventional,
            client: None,
        },
        opts,
    )
}

const STORAGE: &str = "PCTFREE 10 INITRANS 1 STORAGE(INITIAL 65536)";

// ---------------------------------------------------------------------------------------------
// (1) ADR-30 item 1: where an unknown type text ends
// ---------------------------------------------------------------------------------------------

#[test]
fn adr_30_item_1_the_three_test_vectors() {
    // `"X" FOO(3))  ` + storage keeps `FOO(3)` (N-16 said `FOO(3`; the test settles it)
    let l = parse_column_list(format!("CREATE TABLE \"T\" (\"X\" FOO(3))  {STORAGE}").as_bytes())
        .unwrap();
    assert_eq!(l.columns.len(), 1);
    assert_eq!(l.columns[0].type_text, b"FOO(3)");
    assert_eq!(l.columns[0].kind, None);

    // `"X" FOO(3), "Y" DATE)  ` keeps `FOO(3)` and parses Y
    let l = parse_column_list(
        format!("CREATE TABLE \"T\" (\"X\" FOO(3), \"Y\" DATE)  {STORAGE}").as_bytes(),
    )
    .unwrap();
    assert_eq!(l.columns.len(), 2);
    assert_eq!(l.columns[0].type_text, b"FOO(3)");
    assert_eq!(l.columns[0].kind, None);
    assert_eq!(l.columns[1].kind, Some(DeclaredType::Date));

    // `"X" F("A", "B"), "Y" DATE)  ` gives the table fallback with W-DDL-FALLBACK
    let line = format!("CREATE TABLE \"T\" (\"X\" F(\"A\", \"B\"), \"Y\" DATE)  {STORAGE}");
    let r = resolve(
        &line,
        &[desc(1, 3), desc(12, 7)],
        &["X", "Y"],
        &TypeOptions::default(),
    );
    assert!(r.fallback);
    assert_eq!(
        r.warnings.iter().map(|w| w.code).collect::<Vec<_>>(),
        [WarningCode::DdlFallback]
    );
    assert_eq!(r.warnings[0].table, Some(4));
}

// ---------------------------------------------------------------------------------------------
// (2) ADR-30 item 3: a declared NUMBER that no Decimal128 holds
// ---------------------------------------------------------------------------------------------

#[test]
fn a_declared_number_no_decimal128_holds_takes_number_default_with_w_ddl_fallback() {
    for text in [
        "NUMBER(*,40)",
        "NUMBER(3, 50)",
        "NUMBER(38, -2)",
        "NUMBER(*,-2)",
    ] {
        // the forms of [T9]: `NUMBER(p, s)` with a comma and a space, `NUMBER(*,s)` without
        let line = format!("CREATE TABLE \"T\" (\"A\" {text}, \"B\" DATE)  {STORAGE}");
        let descs = [desc(2, 22), desc(12, 7)];
        let r = resolve(&line, &descs, &["A", "B"], &TypeOptions::default());
        let codes: Vec<_> = r.warnings.iter().map(|w| w.code).collect();
        // the declaration is understood (T9) but not honoured: default type, one warning at the definition
        assert_eq!(codes, [WarningCode::DdlFallback], "{text}");
        assert!(!r.fallback, "{text}");
        assert_eq!(
            r.columns[0].arrow_type,
            DataType::Decimal128(38, 10),
            "{text}"
        );
        assert_eq!(r.columns[0].type_source, TypeSource::Default, "{text}");
        assert!(
            r.columns[0].declared.is_some(),
            "{text}: declared keeps p and s"
        );
        let w = &r.warnings[0];
        assert_eq!((w.table, w.column), (Some(4), Some(1)), "{text}");
        let at = usize::try_from(w.offset.unwrap() - 0x2000).unwrap();
        assert!(line.as_bytes()[at..].starts_with(b"\"A\" NUMBER"), "{text}");
        // the other column keeps its declared type
        assert_eq!(r.columns[1].type_source, TypeSource::Declared);

        // `number_default` is what the column takes
        let opts = TypeOptions {
            number_default: "float64".parse().unwrap(),
            ..TypeOptions::default()
        };
        let r = resolve(&line, &descs, &["A", "B"], &opts);
        assert_eq!(r.columns[0].arrow_type, DataType::Float64, "{text}");

        // a `number_types` entry reads it exactly: no warning, source Override
        let mut opts = TypeOptions::default();
        opts.number_types
            .insert("T.A".into(), "decimal256(76,50)".parse().unwrap());
        let r = resolve(&line, &descs, &["A", "B"], &opts);
        assert!(r.warnings.is_empty(), "{text}: {:?}", r.warnings);
        assert_eq!(
            r.columns[0].arrow_type,
            DataType::Decimal256(76, 50),
            "{text}"
        );
        assert_eq!(r.columns[0].type_source, TypeSource::Override, "{text}");
    }
    // declarations that do fit are Declared and silent: NUMBER(*,38), NUMBER(36,-2), NUMBER(10, 2)
    for text in [
        "NUMBER(*,38)",
        "NUMBER(36, -2)",
        "NUMBER(10, 2)",
        "NUMBER(*,0)",
    ] {
        let line = format!("CREATE TABLE \"T\" (\"A\" {text})  {STORAGE}");
        let r = resolve(&line, &[desc(2, 22)], &["A"], &TypeOptions::default());
        assert!(r.warnings.is_empty(), "{text}");
        assert_eq!(r.columns[0].type_source, TypeSource::Declared, "{text}");
    }
}

/// The same through a file: `t_decl_exp_18c.dmp` has `"D_DEC" NUMBER(10, 2)`; `NUMBER(3, 50)` has the same
/// length, so the patched copy keeps every offset.
#[test]
fn the_warning_reaches_the_dump_at_the_definition() {
    let mut data = std::fs::read(fixtures().join("18c/t_decl_exp_18c.dmp")).unwrap();
    let from = b"NUMBER(10, 2)";
    let to = b"NUMBER(3, 50)";
    let at = data.windows(from.len()).position(|w| w == from).unwrap();
    data[at..at + to.len()].copy_from_slice(to);
    let p = target_dir("t5f").join("t_decl_3_50.dmp");
    std::fs::write(&p, data).unwrap();
    let mut d = open(&p, Options::default()).unwrap();
    d.scan().unwrap();
    let t = d
        .catalog()
        .unwrap()
        .tables
        .iter()
        .find(|t| t.name == "T_DECL")
        .unwrap()
        .clone();
    let c = t.columns.iter().find(|c| c.name == "D_DEC").unwrap();
    assert_eq!(c.arrow_type, DataType::Decimal128(38, 10));
    assert_eq!(c.type_source, TypeSource::Default);
    let w: Vec<_> = d
        .warnings()
        .iter()
        .filter(|w| w.code == WarningCode::DdlFallback)
        .collect();
    assert_eq!(w.len(), 1);
    assert_eq!((w[0].table, w[0].column), (Some(t.ordinal), Some(7)));
}

// ---------------------------------------------------------------------------------------------
// (3) W-TSLTZ once per TSLTZ column
// ---------------------------------------------------------------------------------------------

#[test]
fn w_tsltz_is_raised_once_per_tsltz_column_by_the_resolution() {
    let line = format!(
        "CREATE TABLE \"T\" (\"A\" TIMESTAMP (6) WITH LOCAL TIME ZONE, \"B\" DATE, \"C\" TIMESTAMP (0) WITH LOCAL TIME ZONE)  {STORAGE}"
    );
    let descs = [desc(231, 11), desc(12, 7), desc(231, 7)];
    let r = resolve(&line, &descs, &["A", "B", "C"], &TypeOptions::default());
    let w: Vec<_> = r
        .warnings
        .iter()
        .map(|w| (w.code, w.table, w.column, w.offset))
        .collect();
    assert_eq!(
        w,
        [
            (WarningCode::Tsltz, Some(4), Some(1), Some(0x300)),
            (WarningCode::Tsltz, Some(4), Some(3), Some(0x300)),
        ]
    );
    // a table fallback and a column fallback still warn, once per column
    let r = resolve("garbage", &descs[..1], &["A"], &TypeOptions::default());
    let codes: Vec<_> = r.warnings.iter().map(|w| w.code).collect();
    assert_eq!(codes, [WarningCode::DdlFallback, WarningCode::Tsltz]);
    // column fallback: the charset/TSLTZ warnings of the column come before the fallback of its definition
    let line = format!("CREATE TABLE \"T\" (\"A\" ZZZ(1))  {STORAGE}");
    let r = resolve(&line, &descs[..1], &["A"], &TypeOptions::default());
    let mut codes: Vec<_> = r.warnings.iter().map(|w| w.code.as_str()).collect();
    codes.sort_unstable();
    assert_eq!(codes, ["W-DDL-FALLBACK", "W-TSLTZ"]);
}

/// In the 229 fixtures every TSLTZ column (type code 231) has exactly one `W-TSLTZ` in `Dump::warnings()`, with
/// the table and the file position of the column; no other column has one. The number of columns is found in
/// the catalog; the test requires exactly 16 such columns (`collist.py`) so that it cannot pass on an empty set.
#[test]
fn every_tsltz_column_of_the_fixtures_has_one_warning_and_no_other_column_has() {
    let mut columns = 0;
    for folder in ["18c", "11g", "11g-cp1251"] {
        for e in std::fs::read_dir(fixtures().join(folder)).unwrap() {
            let name = e.unwrap().file_name().to_string_lossy().into_owned();
            if !(name.contains("_exp_") && name.ends_with(".dmp")) {
                continue;
            }
            let mut d = open(fixtures().join(folder).join(&name), Options::default()).unwrap();
            d.scan().unwrap();
            let mut want = Vec::new();
            for t in &d.catalog().unwrap().tables {
                for c in &t.columns {
                    if c.type_code == 231 {
                        want.push((t.ordinal, c.file_position));
                    }
                }
            }
            let mut got: Vec<(u32, u32)> = d
                .warnings()
                .iter()
                .filter(|w| w.code == WarningCode::Tsltz)
                .map(|w| (w.table.unwrap(), w.column.unwrap()))
                .collect();
            want.sort_unstable();
            got.sort_unstable();
            assert_eq!(got, want, "{folder}/{name}");
            columns += want.len();
        }
    }
    // N-9 (T-14 g): exact. `collist.py` (docs/formats/tools) prints "16 TIMESTAMP (n) WITH LOCAL TIME ZONE" in its
    // tally of the 2,075 column definitions (T-19b): 4 (the T_TSTZ sections) + 12 (3 columns L0, L3, L9 in each of
    // the 4 `tsltz2` files, `tests/golden/exp_catalog.tsv`).
    assert_eq!(columns, 16, "TSLTZ columns of the 229 files");
}

// ---------------------------------------------------------------------------------------------
// (4) ADR-33 item 4: type_source decides what on_number_overflow governs
// ---------------------------------------------------------------------------------------------

/// A copy of `t_num3_exp_18c.dmp` below `target/` whose `CREATE TABLE` line declares V as `NUMBER(38, 10)`.
/// (The plan names `t_num_exp_18c`; its rows 19 and later have integer parts that no Decimal128(38, 10) holds,
/// so that table stops with `E-NUMBER-OVERFLOW` under every policy; `T_NUM3` has the six 40-digit quotients
/// of `truth_num3.txt`, all with more than ten decimals.) The line keeps its length: `"V" NUMBER)` becomes
/// `"V" NUMBER(38, 10))` and ` LOGGING` (8 bytes) is dropped from the storage text.
fn num3_declared(name: &str) -> PathBuf {
    let mut data = std::fs::read(fixtures().join("18c/t_num3_exp_18c.dmp")).unwrap();
    let from: &[u8] = b"\"V\" NUMBER)  PCTFREE 10 PCTUSED 40 INITRANS 1 MAXTRANS 255 STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 FREELISTS 1 FREELIST GROUPS 1 BUFFER_POOL DEFAULT) TABLESPACE \"USERS\" LOGGING NOCOMPRESS";
    let to: &[u8] = b"\"V\" NUMBER(38, 10))  PCTFREE 10 PCTUSED 40 INITRANS 1 MAXTRANS 255 STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 FREELISTS 1 FREELIST GROUPS 1 BUFFER_POOL DEFAULT) TABLESPACE \"USERS\" NOCOMPRESS";
    assert_eq!(from.len(), to.len());
    let at = data.windows(from.len()).position(|w| w == from).unwrap();
    data[at..at + to.len()].copy_from_slice(to);
    let p = target_dir("t5f").join(name);
    std::fs::write(&p, data).unwrap();
    p
}

#[test]
fn a_declared_number_38_10_is_exact_and_plain_number_rounds() {
    // plain NUMBER: Default, rounds, counted
    let mut d = open_fixture("18c/t_num3_exp_18c.dmp");
    d.scan().unwrap();
    let t = d.table(TableRef::ByName("T_NUM3")).unwrap().clone();
    assert_eq!(t.columns[1].type_source, TypeSource::Default);
    let (_, w) = read(&mut d, "T_NUM3", &ReadOptions::default());
    let r: Vec<_> = w
        .iter()
        .filter(|w| w.code == WarningCode::NumberRounded)
        .collect();
    assert_eq!((r.len(), r[0].count), (1, Some(6)));

    // declared NUMBER(38, 10): the same Arrow type, source Declared, E-NUMBER-OVERFLOW under the default
    let p = num3_declared("num3_declared.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    d.scan().unwrap();
    assert!(d
        .warnings()
        .iter()
        .all(|w| w.code != WarningCode::DdlFallback));
    let t = d.table(TableRef::ByName("T_NUM3")).unwrap().clone();
    assert!(!t.ddl_fallback);
    assert_eq!(t.columns[1].arrow_type, DataType::Decimal128(38, 10));
    assert_eq!(t.columns[1].type_source, TypeSource::Declared);
    assert_eq!(
        t.columns[1].declared,
        Some(DeclaredType::Number {
            precision: Some(38),
            scale: Some(10)
        })
    );
    let e = try_read(&mut d, "T_NUM3", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::NumberOverflow);
    assert_eq!((e.row, e.column), (Some(1), Some(2)));

    // a `number_types` entry of decimal(38,10) on the declared column: Override, rounds like plain NUMBER
    let mut o = Options::default();
    o.types.number_types.insert(
        "T_NUM3.V".to_string(),
        NumberType::Decimal128 {
            precision: 38,
            scale: 10,
        },
    );
    let mut d = open(&p, o).unwrap();
    d.scan().unwrap();
    let t = d.table(TableRef::ByName("T_NUM3")).unwrap().clone();
    assert_eq!(t.columns[1].type_source, TypeSource::Override);
    let (b, w) = read(&mut d, "T_NUM3", &ReadOptions::default());
    assert_eq!(row_count(&b), 6);
    let r: Vec<_> = w
        .iter()
        .filter(|w| w.code == WarningCode::NumberRounded)
        .collect();
    assert_eq!((r.len(), r[0].count), (1, Some(6)));

    // the global policy `null` does not touch a Declared column either: the error stays
    let e = try_read(
        &mut open(&p, Options::default()).unwrap(),
        "T_NUM3",
        &ReadOptions {
            on_number_overflow: OverflowPolicy::Null,
            ..ReadOptions::default()
        },
    )
    .unwrap_err();
    assert_eq!(e.code, ErrorCode::NumberOverflow);
}

// ---------------------------------------------------------------------------------------------
// (5) ADR-32 item 3: Dump::rows on a table of several parts
// ---------------------------------------------------------------------------------------------

#[test]
fn rows_on_a_table_of_several_parts_is_e_config_with_the_number_of_parts() {
    for (file, name, parts) in [
        ("part_exp_18c", "T_PART", 3u64),
        ("part2_exp_18c", "T_PART2", 6),
    ] {
        let mut d = open_fixture(&format!("18c/{file}.dmp"));
        let ordinal = d.table(TableRef::ByName(name)).unwrap().ordinal;
        assert_eq!(
            d.table(TableRef::ByName(name)).unwrap().parts.len() as u64,
            parts
        );
        let e = d.rows(TableRef::ByName(name)).err().unwrap();
        assert_eq!(e.code, ErrorCode::Config, "{name}");
        assert_eq!((e.length, e.table), (Some(parts), Some(ordinal)), "{name}");
        // each part is still reachable through part_rows
        for k in 0..usize::try_from(parts).unwrap() {
            assert!(d.part_rows(TableRef::ByName(name), k).is_ok(), "{name} {k}");
        }
        // an index outside the parts is E-CONFIG too (not E-NO-SUCH-TABLE)
        let e = d
            .part_rows(TableRef::ByName(name), usize::try_from(parts).unwrap())
            .err()
            .unwrap();
        assert_eq!(e.code, ErrorCode::Config, "{name}");
        assert_eq!(e.length, Some(parts), "{name}");
    }
    // a table that does not exist is still E-NO-SUCH-TABLE
    let mut d = open_fixture("18c/part_exp_18c.dmp");
    assert_eq!(
        d.rows(TableRef::ByName("NOPE")).err().unwrap().code,
        ErrorCode::NoSuchTable
    );
    // every one-part table: unchanged (T_STAT1 of stat_exp_18c, T_LONG of long_exp_18c)
    let mut d = open_fixture("18c/stat_exp_18c.dmp");
    let mut r = d.rows(TableRef::ByName("T_STAT1")).unwrap();
    let mut n = 0;
    while r.next_row().unwrap().is_some() {
        n += 1;
    }
    assert_eq!(n, 50); // truth_stat.txt: 50 ROW| lines
}

// ---------------------------------------------------------------------------------------------
// (6) ADR-33 item 3: counted warnings are flushed when the reader returns an error
// ---------------------------------------------------------------------------------------------

/// `t_num3_exp_18c.dmp` with the second mantissa byte of row 2's V (`15 00 c0 43 43 ...`, length 0x0015 little-endian,, the 21-byte value of
/// `truth_num3.txt` row 2) changed from 0x43 to 0xf0: a base-100 digit above 100, which [Y1] does not allow
/// (`E-BAD-NUMBER`). Row 1 is rounded before it.
#[test]
fn counted_warnings_are_flushed_with_the_error() {
    let mut data = std::fs::read(fixtures().join("18c/t_num3_exp_18c.dmp")).unwrap();
    let pat = [0x15u8, 0x00, 0xc0, 0x43, 0x43, 0x43];
    let hits: Vec<usize> = data
        .windows(pat.len())
        .enumerate()
        .filter(|(_, w)| *w == pat)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(hits.len(), 1);
    data[hits[0] + 3] = 0xf0;
    let p = target_dir("t5f").join("num3_bad_row2.dmp");
    std::fs::write(&p, data).unwrap();
    let mut d = open(&p, Options::default()).unwrap();
    let mut r = d
        .read_table(TableRef::ByName("T_NUM3"), &ReadOptions::default())
        .unwrap();
    let e = r.next_batch().unwrap_err();
    assert_eq!(e.code, ErrorCode::BadNumber);
    assert_eq!((e.row, e.column), (Some(2), Some(2)));
    let w: Vec<_> = r
        .warnings()
        .iter()
        .filter(|w| w.code == WarningCode::NumberRounded)
        .collect();
    assert_eq!(w.len(), 1, "{:?}", r.warnings());
    assert_eq!(
        (w[0].count, w[0].count_to_zero, w[0].row, w[0].column),
        (Some(1), Some(0), Some(1), Some(2))
    );
    // after the error the reader is done and reports nothing twice
    assert!(r.next_batch().unwrap().is_none());
    assert_eq!(
        r.warnings()
            .iter()
            .filter(|w| w.code == WarningCode::NumberRounded)
            .count(),
        1
    );
}
