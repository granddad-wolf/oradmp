//! Task T-17 (ADR-80): synthetic `exp` files written from the rules (`common/synth.rs`) and read back.
//!
//! Common mode: the generator and the reader are both written from `docs/formats/exp_reader_rules.md`, so these
//! tests add coverage of combinations, not evidence; the evidence stays the fixtures and the truth files. The
//! expected values are the `Spec`'s own (what the generator wrote), never the reader's output, and the expected
//! offsets are the generator's own (`Layout`). The numbers in the tests (record lengths, the 32,000-byte LONG,
//! the column counts) come from the plan (T-17 "Done when") and the rules file ([Y9] 32,000 bytes; [H5] R in
//! {2048, 4096, 8192, 16384}); no number comes from the reader.
//!
//! Property tests run 64 cases by default; `PROPTEST_CASES=2000 cargo nextest run -E 'binary(t17_synth)'` runs
//! more. Failure persistence is off (no `*.proptest-regressions` file), and every failure message names the seed
//! and the parameters of the `Spec`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use arrow_schema::DataType;
use common::synth::*;
use common::{rows, target_dir, try_read};
use oradmp_core::dump::{open, CatalogEnd, ExportMode, Options, TablePath, TableRef};
use oradmp_core::exp::rows::EndMarker;
use oradmp_core::exp::table::PathKind;
use oradmp_core::reader::ReadOptions;
use oradmp_core::types::map::NumberType;
use oradmp_core::{Error, ErrorCode, WarningCode};
use proptest::prelude::*;

// ---------------------------------------------------------------------------------------------------------
// harness
// ---------------------------------------------------------------------------------------------------------

static COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A file below `target/t17-synth/`, removed on drop.
struct TempFile(PathBuf);

impl TempFile {
    fn write(bytes: &[u8]) -> Self {
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);
        let p = target_dir("t17-synth").join(format!("s-{}-{n}.dmp", std::process::id()));
        std::fs::write(&p, bytes).unwrap();
        Self(p)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn type_code(ty: &ColType) -> u16 {
    match ty {
        ColType::NumberPlain | ColType::NumberInt(_) | ColType::NumberDec(..) => 2,
        ColType::Date => 12,
        ColType::Timestamp(_) => 180,
        ColType::Raw(_) => 23,
        ColType::Varchar2 { .. } | ColType::NVarchar2(_) => 1,
        ColType::Char { .. } | ColType::NChar(_) => 96,
        ColType::Long => 8,
        ColType::LongRaw => 24,
        ColType::Clob => 112,
        ColType::Blob => 113,
    }
}

/// What differs from the default options in one check.
#[derive(Default, Clone)]
struct Variant {
    /// `ReadOptions.columns`
    columns: Option<Vec<String>>,
    /// `Options.types.number_types` entries (`TABLE.COLUMN` -> type, the input under test) and the Arrow type each
    /// must give, written as a literal by the test (review 06b SF-3)
    number_types: Vec<(String, NumberType, DataType)>,
    batch_rows: Option<usize>,
    strict: bool,
}

/// Opens, scans and reads every table of the built file and compares everything with the `Spec` and the
/// `Layout`. `Err` carries a message without file values.
fn verify(spec: &Spec, built: &Built, v: &Variant) -> Result<(), String> {
    verify_inner(spec, built, v)
        .map_err(|e| format!("{e} [unconfirmed: {:?}]", built.layout.unconfirmed))
}

fn verify_inner(spec: &Spec, built: &Built, v: &Variant) -> Result<(), String> {
    let file = TempFile::write(&built.bytes);
    let mut options = Options {
        strict: v.strict,
        ..Options::default()
    };
    let mut over: BTreeMap<String, DataType> = BTreeMap::new();
    for (key, nt, want) in &v.number_types {
        options.types.number_types.insert(key.clone(), *nt);
        over.insert(key.clone(), want.clone());
    }
    let mut ro = ReadOptions {
        columns: v.columns.clone(),
        ..ReadOptions::default()
    };
    if let Some(b) = v.batch_rows {
        ro.batch_rows = b;
    }
    let mut dump = open(&file.0, options).map_err(|e| format!("open: {e}"))?;
    // H1 to H5
    let info = dump.info().clone();
    let want_mode = match spec.mode {
        Mode::Tables => ExportMode::Tables,
        Mode::Users => ExportMode::Users,
    };
    let checks = [
        (info.version == spec.version.line(), "version"),
        (info.user == spec.user, "user"),
        (info.mode == want_mode, "mode"),
        (info.record_length == spec.record_length, "record length"),
        (info.direct_flag == spec.direct, "direct flag"),
        (info.client_charset_id == spec.client_set, "client set"),
        (info.db_charset_id == spec.db_set, "database set"),
        (
            info.national_charset_id == spec.national_set,
            "national set",
        ),
        (info.file_size == built.bytes.len() as u64, "file size"),
    ];
    for (ok, what) in checks {
        if !ok {
            return Err(format!("header: {what} differs from the Spec"));
        }
    }
    if !dump.warnings().is_empty() {
        return Err(format!("open warnings: {:?}", dump.warnings()));
    }
    let cat = dump.scan().map_err(|e| format!("scan: {e}"))?;
    if let CatalogEnd::Stopped(e) = &cat.end {
        return Err(format!("scan stopped: {e}"));
    }
    if cat.end_offset != Some(built.layout.data_end) {
        return Err(format!(
            "end offset {:?}, the generator wrote the second EXIT line to {}",
            cat.end_offset, built.layout.data_end
        ));
    }
    if cat.item_counts.g1_fields != 0 {
        return Err("a field was read by rule G1: an introducing line outside list K".into());
    }
    if cat.item_counts.fields() != spec.expected_field_count() {
        return Err(format!(
            "fields after introducing lines: reader {}, generator {}",
            cat.item_counts.fields(),
            spec.expected_field_count()
        ));
    }
    let infos = cat.tables.clone();
    if infos.len() != spec.tables.len() {
        return Err(format!(
            "tables: reader {}, Spec {}",
            infos.len(),
            spec.tables.len()
        ));
    }
    if !dump.warnings().is_empty() {
        return Err(format!("scan warnings: {:?}", dump.warnings()));
    }
    for (k, (t, ti)) in spec.tables.iter().zip(&infos).enumerate() {
        let tl = &built.layout.tables[k];
        let what = |m: &str| format!("table {} ({k}): {m}", t.name);
        if ti.name != t.name || ti.ordinal as usize != k + 1 {
            return Err(what("name or ordinal"));
        }
        if ti.columns.len() != t.columns.len() {
            return Err(what("column count"));
        }
        if ti.num_rows != t.rows.len() as u64 {
            return Err(what(&format!(
                "row count {} against {}",
                ti.num_rows,
                t.rows.len()
            )));
        }
        if ti.parts.len() != 1 || ti.partitioned {
            return Err(what("one part expected"));
        }
        let want_path = if tl.direct {
            TablePath::Direct
        } else {
            TablePath::Conventional
        };
        if ti.path != want_path {
            return Err(what("path"));
        }
        let p = &ti.parts[0];
        let want_marker = if tl.e2 {
            EndMarker::LfOnly
        } else {
            EndMarker::FfFf
        };
        let offsets_ok = ti.create_offset == tl.create_line
            && p.insert_offset == tl.insert_line
            && p.rows_offset == tl.rows_at
            && p.rows_end_offset == tl.after_part
            && p.end_marker == want_marker
            && p.path
                == if tl.direct {
                    PathKind::Direct
                } else {
                    PathKind::Conventional
                }
            && p.file_order == tl.file_order.iter().map(|&i| i as u32).collect::<Vec<_>>();
        if !offsets_ok {
            return Err(what(&format!(
                "offsets: create {} / {}, insert {} / {}, rows {} / {}, rows end {} / {}, marker {:?}, order {:?} / {:?}",
                ti.create_offset, tl.create_line, p.insert_offset, tl.insert_line, p.rows_offset, tl.rows_at,
                p.rows_end_offset, tl.after_part, p.end_marker, p.file_order, tl.file_order
            )));
        }
        for (i, (c, ci)) in t.columns.iter().zip(&ti.columns).enumerate() {
            let key = format!("{}.{}", t.name, c.name);
            let want_type = over.get(&key).cloned().unwrap_or_else(|| arrow_type(&c.ty));
            if ci.name != c.name
                || ci.type_code != type_code(&c.ty)
                || ci.arrow_type != want_type
                || ci.position as usize != i + 1
                || ci.not_null != c.not_null
            {
                return Err(what(&format!(
                    "column {i}: name / type code / Arrow type {:?} against {:?} / position / NOT NULL",
                    ci.arrow_type, want_type
                )));
            }
        }
    }
    // rows
    for t in &spec.tables {
        let (batches, warnings) =
            try_read(&mut dump, &t.name, &ro).map_err(|e| format!("read {}: {e}", t.name))?;
        let cols: Option<Vec<&str>> = v
            .columns
            .as_ref()
            .map(|c| c.iter().map(String::as_str).collect());
        // the one warning a clean file may give: `W-LONG-LENGTH` once per selected LONG column that holds a value
        // above 100,000 bytes (ADR-95 item 9); the number comes from the Spec
        let want_long = long_columns_over_100k(t, cols.as_deref());
        let got_long = warnings
            .iter()
            .filter(|w| w.code == WarningCode::LongLength)
            .count();
        if warnings.len() != got_long || got_long != want_long {
            return Err(format!(
                "read {}: warnings {warnings:?}, W-LONG-LENGTH expected {want_long}",
                t.name
            ));
        }
        let want = expected_rows(spec, t, cols.as_deref());
        let got = rows(&batches);
        if got.len() != want.len() {
            return Err(format!(
                "{}: {} rows read, {} written",
                t.name,
                got.len(),
                want.len()
            ));
        }
        for (r, (g, w)) in got.iter().zip(&want).enumerate() {
            if g != w {
                let c = g.iter().zip(w).position(|(a, b)| a != b).unwrap_or(0);
                return Err(format!(
                    "{}: row {} column {c} differs (lengths {} / {})",
                    t.name,
                    r + 1,
                    g.get(c).map_or(0, String::len),
                    w.get(c).map_or(0, String::len)
                ));
            }
        }
        if let Some(b) = v.batch_rows {
            let n = want.len();
            if batches.len() != n.div_ceil(b) || batches.iter().any(|x| x.num_rows() > b) {
                return Err(format!(
                    "{}: {} batches for {n} rows of batch_rows {b}",
                    t.name,
                    batches.len()
                ));
            }
        }
        // the schema of a batch: names in table order, types as in the catalog
        if let Some(first) = batches.first() {
            let schema = first.schema();
            let keep: Vec<usize> = (0..t.columns.len())
                .filter(|&i| {
                    cols.as_ref()
                        .is_none_or(|c| c.contains(&t.columns[i].name.as_str()))
                })
                .collect();
            if schema.fields().len() != keep.len() {
                return Err(format!(
                    "{}: {} fields in the schema",
                    t.name,
                    schema.fields().len()
                ));
            }
            for (f, &i) in schema.fields().iter().zip(&keep) {
                let key = format!("{}.{}", t.name, t.columns[i].name);
                let want_type = over
                    .get(&key)
                    .cloned()
                    .unwrap_or_else(|| arrow_type(&t.columns[i].ty));
                if f.name() != &t.columns[i].name || f.data_type() != &want_type {
                    return Err(format!(
                        "{}: schema field {i} is {:?}, expected {want_type:?}",
                        t.name,
                        f.data_type()
                    ));
                }
            }
        }
    }
    Ok(())
}

/// The LONG and LONG RAW columns of `t` (restricted to `cols`) with a value above 100,000 bytes: each gives one
/// `W-LONG-LENGTH` (ADR-99 item 6).
fn long_columns_over_100k(t: &TableSpec, cols: Option<&[&str]>) -> usize {
    t.columns
        .iter()
        .enumerate()
        .filter(|(_, c)| c.ty.is_long())
        .filter(|(_, c)| cols.is_none_or(|k| k.contains(&c.name.as_str())))
        .filter(|(i, _)| {
            t.rows.iter().any(|r| match &r[*i] {
                Cell::Text(s) => s.len() > 100_000,
                Cell::Bytes(b) => b.len() > 100_000,
                _ => false,
            })
        })
        .count()
}

fn check(spec: &Spec) {
    let built = build(spec);
    if let Err(e) = verify(spec, &built, &Variant::default()) {
        panic!("{e}");
    }
}

// ---------------------------------------------------------------------------------------------------------
// (c) the property test
// ---------------------------------------------------------------------------------------------------------

fn config() -> ProptestConfig {
    // (j) no `*.proptest-regressions` file
    let c = ProptestConfig {
        failure_persistence: None,
        ..ProptestConfig::default()
    };
    if std::env::var("PROPTEST_CASES").is_err() {
        // the default case count of `cargo nextest run`
        return ProptestConfig { cases: 64, ..c };
    }
    c
}

/// One `u64` mapped through `params_from_seed` (review 06b N-4): the coverage test measures this distribution.
fn params() -> impl Strategy<Value = Params> {
    any::<u64>().prop_map(params_from_seed)
}

proptest! {
    #![proptest_config(config())]

    /// (c) A random Spec -> open, scan, read_table of every table: names, column count, Arrow types, row counts and
    /// every value equal to the Spec, in file order; also the offsets of the layout. `strict` is on for odd seeds
    /// (a clean file gives no warning, so the result must not change).
    #[test]
    fn a_random_spec_is_read_back_as_written(p in params()) {
        let spec = random_spec(&p);
        let built = build(&spec);
        // `strict` turns W-LONG-LENGTH into E-STRICT, so a Spec with a LONG above 100,000 bytes runs lenient
        let over = spec.tables.iter().any(|t| long_columns_over_100k(t, None) > 0);
        let v = Variant { strict: p.seed % 2 == 1 && !over, ..Variant::default() };
        verify(&spec, &built, &v).map_err(|e| TestCaseError::fail(format!("seed {} params {:?}: {e}", p.origin, p)))?;
    }
}

/// The property test is only as good as its generator: over 400 seeds every column type, both empty forms, a
/// direct-path chain, a LOB table, a CL8MSWIN1251 database with Cyrillic text, owner mode, all four record
/// lengths, every kind of noise item and NULL cells appear.
#[test]
fn the_random_specs_cover_every_construct() {
    let mut seen: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut hit = |k: &'static str| *seen.entry(k).or_default() += 1;
    for n in 0..400u64 {
        let spec = random_spec(&params_from_seed(n));
        let built = build(&spec);
        hit(match spec.record_length {
            2048 => "R2048",
            4096 => "R4096",
            8192 => "R8192",
            _ => "R16384",
        });
        if spec.mode == Mode::Users {
            hit("owner");
        }
        if spec.db_set == 171 {
            hit("db171");
        }
        // SF-1: database set 171 only with an 11g header
        assert!(
            !(spec.version == Version::V18 && spec.db_set == 171),
            "seed {n}: an 18c header with database set 171"
        );
        for (t, tl) in spec.tables.iter().zip(&built.layout.tables) {
            if t.rows.is_empty() {
                hit(if tl.e2 {
                    "empty E2"
                } else if tl.direct {
                    "empty direct E1"
                } else {
                    "empty conv"
                });
            }
            if t.has_lob() {
                hit("lob table");
            }
            if t.columns.iter().any(|c| c.not_null) {
                hit("not null");
            }
            for n in t.before.iter().chain(&t.after) {
                hit(match n {
                    Noise::Line(l) if l.starts_with("GRANT") => "line GRANT",
                    Noise::Line(l) if l.starts_with("ANALCOMPUTE") => "line ANALCOMPUTE",
                    Noise::Line(_) => "line other",
                    Noise::Plain { intro, .. } if intro.starts_with("ANALSTATS") => {
                        "plain ANALSTATS"
                    }
                    Noise::Plain { intro, .. } if intro.starts_with("COMMENT") => "plain COMMENT",
                    Noise::Plain { .. } => "plain MODIFY DEFAULT",
                    Noise::Marked { intro, .. } if intro.starts_with("ANALSTATS") => {
                        "marked ANALSTATS"
                    }
                    Noise::Marked { .. } => "marked CALTER",
                    Noise::Unmarked { .. } => "unmarked",
                });
            }
            for c in &t.columns {
                hit(match c.ty {
                    ColType::NumberPlain => "NUMBER",
                    ColType::NumberInt(_) => "NUMBER(p,0)",
                    ColType::NumberDec(..) => "NUMBER(p,s)",
                    ColType::Date => "DATE",
                    // R-047 answered (ADR-94 item 1): f = 0 to 9 are drawn; the keys are `TIMESTAMP(f)`
                    ColType::Timestamp(0) => "TIMESTAMP(0)",
                    ColType::Timestamp(1) => "TIMESTAMP(1)",
                    ColType::Timestamp(2) => "TIMESTAMP(2)",
                    ColType::Timestamp(3) => "TIMESTAMP(3)",
                    ColType::Timestamp(4) => "TIMESTAMP(4)",
                    ColType::Timestamp(5) => "TIMESTAMP(5)",
                    ColType::Timestamp(6) => "TIMESTAMP(6)",
                    ColType::Timestamp(7) => "TIMESTAMP(7)",
                    ColType::Timestamp(8) => "TIMESTAMP(8)",
                    ColType::Timestamp(9) => "TIMESTAMP(9)",
                    ColType::Timestamp(f) => panic!("seed {n}: TIMESTAMP({f}) drawn"),
                    ColType::Raw(_) => "RAW",
                    ColType::Varchar2 { chars: false, .. } => "VARCHAR2",
                    ColType::Varchar2 { chars: true, .. } => "VARCHAR2 CHAR",
                    ColType::Char { chars: false, .. } => "CHAR",
                    ColType::Char { chars: true, .. } => "CHAR CHAR",
                    ColType::NVarchar2(_) => "NVARCHAR2",
                    ColType::NChar(_) => "NCHAR",
                    ColType::Long => "LONG",
                    ColType::LongRaw => "LONG RAW",
                    ColType::Clob => "CLOB",
                    ColType::Blob => "BLOB",
                });
            }
            if tl.direct
                && tl
                    .rows
                    .iter()
                    .any(|r| r.fields.iter().any(|f| f.pieces > 1))
            {
                hit("direct chain");
            }
            if tl.direct
                && t.columns.iter().any(|c| c.ty.is_long())
                && tl.file_order.windows(2).any(|w| w[0] > w[1])
            {
                hit("LONG moved last");
            }
            if t.rows.iter().flatten().any(|c| *c == Cell::Null) {
                hit("NULL cell");
            }
            if t.rows.iter().flatten().any(|c| matches!(c, Cell::Text(s) if s.chars().any(|ch| ('\u{400}'..'\u{500}').contains(&ch)))) {
                hit("Cyrillic text");
            }
            if t.rows
                .iter()
                .flatten()
                .any(|c| matches!(c, Cell::Text(s) if s.chars().any(|ch| ch as u32 > 0xffff)))
            {
                hit("non-BMP text");
            }
            if tl
                .rows
                .iter()
                .any(|r| r.fields.iter().any(|f| f.len >= 32_000))
            {
                hit("32,000 bytes");
            }
        }
    }
    for key in [
        "R2048",
        "R4096",
        "R8192",
        "R16384",
        "owner",
        "db171",
        "empty E2",
        "empty direct E1",
        "empty conv",
        "lob table",
        "not null",
        "line GRANT",
        "line ANALCOMPUTE",
        "line other",
        "plain ANALSTATS",
        "plain COMMENT",
        "plain MODIFY DEFAULT",
        "marked ANALSTATS",
        "marked CALTER",
        "NUMBER",
        "NUMBER(p,0)",
        "NUMBER(p,s)",
        "DATE",
        "TIMESTAMP(0)",
        "TIMESTAMP(1)",
        "TIMESTAMP(2)",
        "TIMESTAMP(3)",
        "TIMESTAMP(4)",
        "TIMESTAMP(5)",
        "TIMESTAMP(6)",
        "TIMESTAMP(7)",
        "TIMESTAMP(8)",
        "TIMESTAMP(9)",
        "RAW",
        "VARCHAR2",
        "VARCHAR2 CHAR",
        "CHAR",
        "CHAR CHAR",
        "NVARCHAR2",
        "NCHAR",
        "LONG",
        "LONG RAW",
        "CLOB",
        "BLOB",
        "direct chain",
        "LONG moved last",
        "NULL cell",
        "Cyrillic text",
        "non-BMP text",
        "32,000 bytes",
    ] {
        assert!(
            seen.get(key).copied().unwrap_or(0) >= 3,
            "the generator hardly produces {key}: {seen:?}"
        );
    }
}

/// (a)(4) The header choices of `HEADER_CHOICES` against the 229 EXP headers of the fixtures. The test reads the
/// bytes itself by [H1] to [H4] (byte 0 = 03, u16 BE client set, optional `D` LF, seven LF-terminated lines, three
/// u16 BE: client, database, national set); it calls nothing of `oradmp_core`. Expected: 229 files; every
/// unlabelled entry occurs, no entry labelled `ClientSetNotInFixtures` occurs, no entry pairs 18c with database 171.
/// Numbers of the occurring pairs (own script over the same bytes, 2026-10-24): 18 distinct (version, mode, client,
/// database) combinations in the 229 files (T-19: the ten new files add none; T-19b: the 18 new files add none).
#[test]
fn header_sets_match_the_fixtures() {
    let mut seen: std::collections::BTreeSet<(String, String, u16, u16)> = Default::default();
    let mut files = 0usize;
    for folder in common::FOLDERS {
        for e in std::fs::read_dir(common::fixtures().join(folder)).unwrap() {
            let path = e.unwrap().path();
            if path.extension().and_then(|x| x.to_str()) != Some("dmp") {
                continue;
            }
            let mut head = vec![0u8; 4096];
            let n =
                std::io::Read::read(&mut std::fs::File::open(&path).unwrap(), &mut head).unwrap();
            head.truncate(n);
            if head.first() != Some(&0x03) {
                continue;
            }
            let mut p = 3usize;
            if head.get(3..5) == Some(&b"D\n"[..]) {
                p = 5;
            }
            if !head[p..].starts_with(b"EXPORT:V") {
                continue;
            }
            let mut lines: Vec<String> = Vec::new();
            for _ in 0..7 {
                let end = p + head[p..].iter().position(|&b| b == 0x0a).unwrap();
                lines.push(String::from_utf8(head[p..end].to_vec()).unwrap());
                p = end + 1;
            }
            let u16be = |at: usize| u16::from_be_bytes([head[at], head[at + 1]]);
            let client = u16be(p);
            let database = u16be(p + 2);
            files += 1;
            seen.insert((lines[0].clone(), lines[2].clone(), client, database));
        }
    }
    assert_eq!(files, 229, "the fixtures hold 229 EXP files");
    for h in HEADER_CHOICES {
        let key = (
            h.version.line().to_string(),
            match h.mode {
                Mode::Tables => "RTABLES".to_string(),
                Mode::Users => "RUSERS".to_string(),
            },
            h.client_set,
            h.db_set,
        );
        match h.label {
            None => assert!(
                seen.contains(&key),
                "{h:?} is unlabelled but no fixture has it"
            ),
            Some(Unconfirmed::ClientSetNotInFixtures) => {
                assert!(
                    !seen.contains(&key),
                    "{h:?} is labelled but a fixture has it"
                )
            }
            Some(other) => panic!("{h:?}: the label {other:?} does not belong to a header"),
        }
        assert!(
            !(h.version == Version::V18 && h.db_set == 171),
            "{h:?}: an 18c header with database set 171"
        );
    }
}

/// SF-2 / [E2 (a)]: every empty direct-path table of the E2 form is followed (at `end_at + 1`) by bytes that start
/// with one of the 14 entries of the E2 (a) list. The 14 entries are literals from the rules file.
/// Seeds 0..S with `max_rows` 3 (only the structure matters); S, the time and the hits are in the note. The bytes
/// are checked (not the item list).
#[test]
fn no_e2_table_is_followed_by_an_item_outside_the_e2_list() {
    const LIST: [&[u8]; 14] = [
        b"TABLE \"",
        b"ENDTABLE",
        b"METRICS",
        b"CREATE ",
        b"ALTER ",
        b"COMMENT ON ",
        b"GRANT ",
        b"ANALSTATS ",
        b"CALTER TABLE",
        b"PARTITION \"",
        b"COMPARTITION \"",
        b"SUBPARTITION \"",
        b"ENDCOMPARTITION",
        b"ENDPARTITION",
    ];
    const S: u64 = 40_000;
    let mut e2_tables = 0usize;
    for n in 0..S {
        let mut p = params_from_seed(n);
        p.max_rows = 3;
        let spec = random_spec(&p);
        let may_have_e2 = spec
            .tables
            .iter()
            .any(|t| t.rows.is_empty() && t.empty_form == EmptyForm::E2);
        if !may_have_e2 {
            continue;
        }
        let built = build(&spec);
        for tl in built.layout.tables.iter().filter(|tl| tl.e2) {
            e2_tables += 1;
            let rest = &built.bytes[tl.end_at as usize + 1..];
            assert!(
                LIST.iter().any(|item| rest.starts_with(item)),
                "seed {n}: an E2 table at {} is followed by an item outside the E2 (a) list",
                tl.end_at
            );
        }
    }
    assert!(e2_tables >= 1000, "only {e2_tables} E2 tables in {S} seeds");
}

// ---------------------------------------------------------------------------------------------------------
// (d) deterministic cases
// ---------------------------------------------------------------------------------------------------------

fn id_val(n: i128) -> Cell {
    Cell::Num(n, 0)
}

fn txt(s: &str) -> Cell {
    Cell::Text(s.to_string())
}

fn three_rows() -> Vec<Vec<Cell>> {
    vec![
        vec![id_val(1), txt("one")],
        vec![id_val(2), txt("two")],
        vec![id_val(3), Cell::Null],
    ]
}

fn simple_cols() -> Vec<ColSpec> {
    vec![
        col("ID", ColType::NumberInt(5)),
        col(
            "V",
            ColType::Varchar2 {
                n: 30,
                chars: false,
            },
        ),
    ]
}

/// [E1, E2, E3]: both empty-table forms after the path flag 1 in one 18c file, each followed by another table or
/// by an item of the list of E2; a file of E1 only (the 11g form); a conventional empty table.
#[test]
fn both_empty_table_forms_after_a_direct_path_flag() {
    let mut a = table("A", simple_cols(), three_rows());
    let mut b = table("B_E2", simple_cols(), vec![]);
    b.empty_form = EmptyForm::E2;
    let c = table("C", simple_cols(), three_rows());
    let mut d = table("D_E1", simple_cols(), vec![]);
    d.empty_form = EmptyForm::E1;
    let mut e = table("E_E2", simple_cols(), vec![]);
    e.empty_form = EmptyForm::E2;
    e.after = vec![Noise::Line("GRANT SELECT ON \"E_E2\" TO \"X\"".into())];
    let mut f = table("F_E2_LAST", simple_cols(), vec![]);
    f.empty_form = EmptyForm::E2;
    a.after = vec![Noise::Line("GRANT SELECT ON \"A\" TO \"X\"".into())];
    let mut spec = spec_of(vec![a, b, c, d, e, f]);
    spec.direct = true;
    let built = build(&spec);
    // the generator wrote the LF directly after the flag for the E2 tables
    let l = &built.layout.tables;
    assert!(!l[0].e2 && l[1].e2 && !l[2].e2 && !l[3].e2 && l[4].e2 && l[5].e2);
    assert_eq!(built.bytes[l[1].end_at as usize], 0x0a);
    assert_eq!(
        &built.bytes[l[3].end_at as usize..l[3].end_at as usize + 2],
        &[0xff, 0xff]
    );
    // ADR-94 item 2: an 18c table-mode file with E1 (D_E1) and E2 tables behind flag 1 is what `seg_exp_direct_18c`
    // shows, so it holds no variant
    assert!(
        built.layout.unconfirmed.is_empty(),
        "an 18c table-mode file with an E1 empty table behind flag 1: {:?}",
        built.layout.unconfirmed
    );
    verify(&spec, &built, &Variant::default()).unwrap();

    // an 18c owner-mode file with the same tables keeps the label (ADR-86, ADR-94 item 2)
    let mut owner = spec.clone();
    owner.mode = Mode::Users;
    let built = build(&owner);
    assert_eq!(
        built.layout.unconfirmed,
        [Unconfirmed::E1AfterFlag1In18cOwnerMode]
            .into_iter()
            .collect(),
        "an 18c owner-mode file with an E1 empty table behind flag 1"
    );
    verify(&owner, &built, &Variant::default()).unwrap();

    // a file with only E2 tables holds no variant
    let mut only_e2 = table("A_E2", simple_cols(), vec![]);
    only_e2.empty_form = EmptyForm::E2;
    let mut spec = spec_of(vec![only_e2, table("C", simple_cols(), three_rows())]);
    spec.direct = true;
    let built = build(&spec);
    assert!(built.layout.tables[0].e2);
    assert!(
        built.layout.unconfirmed.is_empty(),
        "{:?}",
        built.layout.unconfirmed
    );
    verify(&spec, &built, &Variant::default()).unwrap();
}

/// [E1] in the 11g form: an empty table on the direct path ends with `ff ff` LF; on the conventional path too.
#[test]
fn an_empty_table_with_ff_ff_on_both_paths_in_an_11g_file() {
    for direct in [false, true] {
        let mut spec = spec_of(vec![
            table("A", simple_cols(), three_rows()),
            table("B", simple_cols(), vec![]),
            table("C", simple_cols(), three_rows()),
        ]);
        spec.version = Version::V11;
        spec.direct = direct;
        // an 11g file with E1 (the form the fixtures show) holds no unconfirmed construct
        assert!(build(&spec).layout.unconfirmed.is_empty());
        check(&spec);
    }
}

/// A one-table spec with three rows whose middle row has the cells the split cases cut.
fn split_spec(pad: usize, direct: bool, r: u64) -> Spec {
    let cols = vec![
        col("ID", ColType::NumberInt(5)),
        col(
            "V",
            ColType::Varchar2 {
                n: 40,
                chars: false,
            },
        ),
        col(
            "W",
            ColType::Varchar2 {
                n: 10,
                chars: false,
            },
        ),
    ];
    let rows = vec![
        vec![id_val(1), txt("first row"), txt("a")],
        vec![id_val(2), txt("abcdefghijklmnopqrst"), Cell::Null],
        vec![id_val(3), txt("last row"), txt("z")],
    ];
    let mut pad_table = table(
        "PAD",
        vec![col("P", ColType::NumberInt(3))],
        vec![vec![id_val(7)]],
    );
    pad_table.after = vec![Noise::Plain {
        intro: "COMMENT ON TABLE \"PAD\" IS ".into(),
        field: vec![b'x'; pad],
    }];
    let mut spec = spec_of(vec![pad_table, table("TARGET", cols, rows)]);
    spec.direct = direct;
    spec.record_length = r;
    spec
}

/// Builds the file in which a record boundary falls exactly on the byte offset `target(layout)`: the pad comment
/// in table PAD grows by one byte per byte, so the offset of everything behind it moves one for one.
fn with_boundary_at(direct: bool, r: u64, target: impl Fn(&Layout) -> u64) -> (Spec, Built) {
    let first = build(&split_spec(1, direct, r));
    let x1 = target(&first.layout);
    let pad = 1 + ((r - x1 % r) % r) as usize;
    let spec = split_spec(pad, direct, r);
    let built = build(&spec);
    let x = target(&built.layout);
    assert_eq!(x % r, 0, "the boundary must fall on the target");
    assert!(x > r + 0x104, "inside the text region");
    (spec, built)
}

/// [R5] A length field, a NULL marker, a value and a row end each split by a record boundary at every byte
/// position of the element, on both paths (the `00 00` exists on the conventional path only).
#[test]
fn a_record_boundary_at_every_byte_position_of_an_element_changes_nothing() {
    let r = 2048;
    for direct in [false, true] {
        // row 2 (index 1): ID = 2 (field 0), V = 20 bytes (field 1), W = NULL (field 2)
        let mut cases = 0;
        let row = |l: &Layout| l.tables[1].rows[1].clone();
        let field = |l: &Layout, i: usize| row(l).fields[i].clone();
        // the length field of V: before it, inside it, behind it
        for d in 0..=2u64 {
            let (spec, built) = with_boundary_at(direct, r, |l| field(l, 1).len_at + d);
            verify(&spec, &built, &Variant::default()).unwrap();
            cases += 1;
        }
        // every position of the 20 value bytes (0 = right after the length field, 20 = right behind the value)
        for j in 0..=20u64 {
            let (spec, built) = with_boundary_at(direct, r, |l| field(l, 1).value_at + j);
            verify(&spec, &built, &Variant::default()).unwrap();
            cases += 1;
        }
        // the NULL marker of W: before it, inside it, behind it
        for d in 0..=2u64 {
            let (spec, built) = with_boundary_at(direct, r, |l| field(l, 2).len_at + d);
            verify(&spec, &built, &Variant::default()).unwrap();
            cases += 1;
        }
        if !direct {
            // the `00 00` row end: inside it
            let (spec, built) = with_boundary_at(direct, r, |l| row(l).tail_at + 1);
            verify(&spec, &built, &Variant::default()).unwrap();
            cases += 1;
        }
        assert_eq!(cases, if direct { 27 } else { 28 });
    }
}

fn long_spec(direct: bool, ty: ColType, r: u64) -> Spec {
    let v = match ty {
        ColType::Long => Cell::Text("ab\u{e9}".repeat(8000)), // 4 bytes x 8000 = 32,000 bytes [Y9]
        _ => Cell::Bytes((0..32_000u32).map(|i| (i * 7 + 3) as u8).collect()),
    };
    // declared order: the LONG column first; on the direct path it stands last in the file [T3]
    let cols = vec![
        col("L", ty),
        col("ID", ColType::NumberInt(5)),
        col(
            "X",
            ColType::Varchar2 {
                n: 10,
                chars: false,
            },
        ),
    ];
    let rows = vec![
        vec![v.clone(), id_val(1), txt("x")],
        vec![Cell::Null, id_val(2), txt("y")],
        vec![v, id_val(3), Cell::Null],
    ];
    let mut spec = spec_of(vec![table("T_LONG", cols, rows)]);
    spec.direct = direct;
    spec.record_length = r;
    spec
}

/// [Y9, R2] A 32,000-byte LONG and LONG RAW on both paths (one field; on the direct path a chain of pieces).
#[test]
fn a_32000_byte_long_and_long_raw_on_both_paths() {
    for direct in [false, true] {
        for ty in [ColType::Long, ColType::LongRaw] {
            let spec = long_spec(direct, ty.clone(), 2048);
            let built = build(&spec);
            let f = &built.layout.tables[0].rows[0].fields;
            // file order: ID, X, then the LONG on the direct path; declared order on the conventional path
            let long_field = if direct { &f[2] } else { &f[0] };
            if ty == ColType::LongRaw {
                assert_eq!(long_field.len, 32_000);
            }
            if direct {
                assert!(
                    long_field.pieces >= 5,
                    "32,000 bytes need at least 5 pieces of at most 7,961"
                );
                assert_eq!(built.layout.tables[0].file_order, vec![1, 2, 0]);
            } else {
                assert_eq!(long_field.pieces, 1);
                assert_eq!(built.layout.tables[0].file_order, vec![0, 1, 2]);
            }
            verify(&spec, &built, &Variant::default()).unwrap();
        }
    }
}

/// N-2 of review 06b: the exact chain shape of the fixtures ([Y9], [R2]; `long_exp_direct_<v>`: 156 + 3 x 7,961 +
/// 7,961 = 32,000 bytes, headers `9c 80`, `19 9f` three times, `19 1f`) in a two-column LONG table on the direct
/// path. The five piece headers are checked at the offsets the generator recorded.
#[test]
fn a_long_chain_in_the_shape_of_the_fixtures() {
    let value = "ab\u{e9}".repeat(8000); // 4 bytes x 8000 = 32,000 bytes
    let cols = vec![col("ID", ColType::NumberInt(5)), col("L", ColType::Long)];
    let rows = vec![vec![id_val(1), Cell::Text(value)]];
    let mut spec = spec_of(vec![table("T_CHAIN", cols, rows)]);
    spec.direct = true;
    spec.chain_shape = Some(vec![156, 7961, 7961, 7961, 7961]);
    let built = build(&spec);
    let f = &built.layout.tables[0].rows[0].fields[1]; // file order: ID, then the LONG
    assert_eq!((f.pieces, f.len), (5, 32_000));
    let want_headers: [[u8; 2]; 5] = [
        [0x9c, 0x80],
        [0x19, 0x9f],
        [0x19, 0x9f],
        [0x19, 0x9f],
        [0x19, 0x1f],
    ];
    let sizes = [156usize, 7961, 7961, 7961, 7961];
    let mut at = f.len_at as usize;
    for (k, want) in want_headers.iter().enumerate() {
        assert_eq!(&built.bytes[at..at + 2], want, "piece header {k} at {at}");
        at += 2 + sizes[k];
    }
    verify(&spec, &built, &Variant::default()).unwrap();
}

/// Owner mode with three tables (table 2 empty on the direct path), and with the groups of the owner-mode extras
/// (PROCACTION, a view, statement groups closed by ENDTABLE).
#[test]
fn owner_mode_with_three_tables() {
    for direct in [false, true] {
        let mut t2 = table("T2", simple_cols(), vec![]);
        t2.empty_form = EmptyForm::E2;
        let mut spec = spec_of(vec![
            table("T1", simple_cols(), three_rows()),
            t2,
            table("T3", simple_cols(), three_rows()),
        ]);
        spec.mode = Mode::Users;
        spec.extras = true;
        spec.direct = direct;
        check(&spec);
        // four tables: the statement group leaves a TABLE line open at the end of the list
        spec.tables.push(table("T4", simple_cols(), three_rows()));
        check(&spec);
    }
}

/// R = 2048 and R = 16384 [H5]: a table that crosses many record boundaries, and the other two lengths of the fixtures.
#[test]
fn record_lengths_2048_and_16384() {
    for r in [2048u64, 4096, 8192, 16384] {
        let rows: Vec<Vec<Cell>> = (0..400)
            .map(|i| {
                vec![
                    id_val(i),
                    txt(&format!("row {i} {}", "x".repeat((i % 50) as usize))),
                ]
            })
            .collect();
        let cols = vec![
            col("ID", ColType::NumberInt(5)),
            col(
                "V",
                ColType::Varchar2 {
                    n: 100,
                    chars: false,
                },
            ),
        ];
        let mut spec = spec_of(vec![table("BIG", cols, rows)]);
        spec.record_length = r;
        let built = build(&spec);
        assert_eq!(built.bytes.len() as u64 % r, 0);
        assert!(
            built.bytes.len() as u64 > 2 * r,
            "the data cross at least one boundary"
        );
        verify(&spec, &built, &Variant::default()).unwrap();
    }
}

// ---------------------------------------------------------------------------------------------------------
// (b) every type of the list, with values at the edges
// ---------------------------------------------------------------------------------------------------------

/// One table with every generated type, on both paths where allowed, with edge values: NUMBER limits, DATE years 1
/// and 9999, TIMESTAMP with and without fraction, NVARCHAR2 / NCHAR with a non-BMP character, CHAR padding in both
/// semantics, 1251 text. The edge values are the rules' own (Y1: 38 digits, Y2: years, C5: surrogate pair).
#[test]
fn every_generated_type_with_edge_values() {
    let all_digits: i128 = 99_999_999_999_999_999_999_999_999_999_999_999_999; // 38 nines
    for db_set in [873u16, 171] {
        for direct in [false, true] {
            let text = if db_set == 873 {
                "caf\u{e9} \u{65e5}\u{672c} \u{1f600}"
            } else {
                "\u{413}\u{440}\u{435}\u{445}"
            };
            let cols = vec![
                col("N0", ColType::NumberPlain),
                col("N1", ColType::NumberInt(18)),
                col("N2", ColType::NumberInt(38)),
                col("N3", ColType::NumberDec(10, 4)),
                col("N4", ColType::NumberDec(38, 38)),
                col("D", ColType::Date),
                col("T0", ColType::Timestamp(0)),
                col("T6", ColType::Timestamp(6)),
                col("T9", ColType::Timestamp(9)),
                col("R", ColType::Raw(16)),
                col(
                    "V",
                    ColType::Varchar2 {
                        n: 60,
                        chars: false,
                    },
                ),
                col("VC", ColType::Varchar2 { n: 20, chars: true }),
                col(
                    "C",
                    ColType::Char {
                        n: 12,
                        chars: false,
                    },
                ),
                col("CC", ColType::Char { n: 6, chars: true }),
                col("NV", ColType::NVarchar2(20)),
                col("NC", ColType::NChar(5)),
            ];
            let t = |secs: i64, nanos: u32| Cell::Time { secs, nanos };
            let y1 = days_from_civil(1, 1, 1) * 86_400;
            let y9999 = days_from_civil(9999, 12, 31) * 86_400 + 86_399;
            let rows = vec![
                vec![
                    Cell::Num(0, 0),
                    Cell::Num(999_999_999_999_999_999, 0),
                    Cell::Num(all_digits, 0),
                    Cell::Num(-123_456, 4),
                    Cell::Num(1, 38),
                    t(y1, 0),
                    t(days_from_civil(2026, 10, 4) * 86_400 + 79_918, 0),
                    t(days_from_civil(2026, 10, 4) * 86_400 + 79_918, 123_456_000),
                    t(days_from_civil(2026, 10, 4) * 86_400 + 79_918, 123_456_789),
                    Cell::Bytes(vec![0xff, 0xff, 0x0a, 0x00, 0xfe, 0xff]),
                    txt(text),
                    txt(text),
                    txt("ab"),
                    txt("a"),
                    txt("n\u{e9}"),
                    txt("a"),
                ],
                vec![
                    Cell::Num(-5, 3),
                    Cell::Num(-1, 0),
                    Cell::Num(-all_digits, 0),
                    Cell::Num(10, 4),
                    Cell::Num(-99_999_999_999_999_999_999_999_999_999_999_999_999, 38),
                    t(y9999, 0),
                    t(0, 0),
                    t(-1, 999_999_000),
                    t(1, 1),
                    Cell::Bytes(vec![1]),
                    txt("x"),
                    txt("y"),
                    txt("z"),
                    txt("w"),
                    txt("v"),
                    txt("c"),
                ],
            ];
            // NCHAR(5) holding "a" + U+1F600 is 3 units: padded to 5 with two spaces; NVARCHAR2 with a non-BMP pair
            let mut rows = rows;
            if db_set == 873 {
                rows[1][14] = txt("\u{1f600}\u{1f600}");
                rows[1][15] = txt("a\u{1f600}");
            }
            let mut spec = spec_of(vec![table("ALLTYPES", cols, rows)]);
            spec.db_set = db_set;
            spec.client_set = if db_set == 171 { 171 } else { 873 };
            if db_set == 171 {
                // database set 171 only with an 11g header (SF-1; the fixtures have no 18c file with 171)
                spec.version = Version::V11;
            }
            spec.direct = direct;
            check(&spec);
        }
    }
}

/// [L1 to L5] A LOB table (conventional path, also in a `direct=y` file [L6]) with CLOB and BLOB values in the short
/// form, NULL and empty LOBs, and a table behind it.
#[test]
fn a_lob_table_in_both_kinds_of_file() {
    for direct in [false, true] {
        let cols = vec![
            col("ID", ColType::NumberInt(3)),
            col("C", ColType::Clob),
            col("B", ColType::Blob),
        ];
        let rows = vec![
            vec![
                id_val(1),
                txt("clob \u{e9}\u{1f600}"),
                Cell::Bytes(vec![0xde, 0xad, 0xbe, 0xef]),
            ],
            vec![id_val(2), Cell::Null, Cell::Bytes(vec![])],
            vec![id_val(3), txt(""), Cell::Null],
            vec![id_val(4), Cell::Null, Cell::Null],
            vec![
                id_val(5),
                txt(&"c".repeat(3000)),
                Cell::Bytes(vec![7; 4000]),
            ],
        ];
        let mut spec = spec_of(vec![
            table("T_LOB", cols, rows),
            table("AFTER", simple_cols(), three_rows()),
        ]);
        spec.direct = direct;
        let built = build(&spec);
        assert!(
            !built.layout.tables[0].direct,
            "L6: a LOB table is conventional"
        );
        assert_eq!(built.layout.tables[1].direct, direct);
        verify(&spec, &built, &Variant::default()).unwrap();
    }
}

// ---------------------------------------------------------------------------------------------------------
// (e) option sets of review 04 N-9 on generated files
// ---------------------------------------------------------------------------------------------------------

fn option_spec() -> Spec {
    let cols = vec![
        col("ID", ColType::NumberInt(10)),
        col("AMT", ColType::NumberDec(12, 2)),
        col("N", ColType::NumberPlain),
        col("BIG", ColType::NumberInt(25)),
        col("D", ColType::Date),
        col(
            "S",
            ColType::Varchar2 {
                n: 20,
                chars: false,
            },
        ),
        col("R", ColType::Raw(8)),
    ];
    let rows: Vec<Vec<Cell>> = (0..30)
        .map(|i| {
            vec![
                id_val(1000 + i),
                if i % 7 == 3 {
                    Cell::Null
                } else {
                    Cell::Num(i * 125 - 700, 2)
                },
                Cell::Num(i * 31 + 5, (i % 5) as u8),
                Cell::Num(i * 1_000_003, 0),
                Cell::Time {
                    secs: 1_700_000_000 + i as i64 * 86_400,
                    nanos: 0,
                },
                txt(&format!("s{i}")),
                if i % 4 == 0 {
                    Cell::Null
                } else {
                    Cell::Bytes(vec![i as u8; 3])
                },
            ]
        })
        .collect();
    let mut spec = spec_of(vec![
        table("OPT", cols, rows),
        table("OTHER", simple_cols(), three_rows()),
    ]);
    spec.record_length = 2048;
    spec
}

/// `columns` in a different order than the table, one column, a column in the middle skipped: the result keeps the
/// table order (documented on `ReadOptions.columns`).
#[test]
fn projection_keeps_the_table_order() {
    let spec = option_spec();
    let built = build(&spec);
    for cols in [
        vec!["S", "ID"],
        vec!["R"],
        vec!["D", "AMT", "ID"],
        vec!["N", "S"],
    ] {
        let v = Variant {
            columns: Some(cols.iter().map(|c| c.to_string()).collect()),
            ..Variant::default()
        };
        // the Variant applies to every table of the Spec, so the other table is read with names it lacks:
        // read one table at a time through a Spec of one table
        let mut one = spec.clone();
        one.tables.truncate(1);
        let b = build(&one);
        verify(&one, &b, &v).unwrap_or_else(|e| panic!("{cols:?}: {e}"));
    }
    verify(&spec, &built, &Variant::default()).unwrap();
}

/// `number_types`: `int64`, `string`, `decimal(p,s)` replace the Arrow type of a NUMBER column; the values equal
/// the Spec's decimals.
#[test]
fn number_types_override_the_arrow_type() {
    let mut spec = option_spec();
    spec.tables.truncate(1);
    let built = build(&spec);
    let v = Variant {
        number_types: vec![
            ("OPT.BIG".into(), NumberType::Int64, DataType::Int64),
            ("OPT.AMT".into(), NumberType::Utf8, DataType::Utf8),
            (
                "OPT.N".into(),
                NumberType::Decimal128 {
                    precision: 20,
                    scale: 4,
                },
                DataType::Decimal128(20, 4),
            ),
        ],
        ..Variant::default()
    };
    verify(&spec, &built, &v).unwrap();
}

/// `strict=true` on a clean file: no error, no warning, the same rows. And on the larger fixtures of this file.
#[test]
fn strict_on_a_clean_file() {
    let spec = option_spec();
    let built = build(&spec);
    verify(
        &spec,
        &built,
        &Variant {
            strict: true,
            ..Variant::default()
        },
    )
    .unwrap();
    let mut owner = spec.clone();
    owner.mode = Mode::Users;
    owner.extras = true;
    owner.direct = true;
    let built = build(&owner);
    verify(
        &owner,
        &built,
        &Variant {
            strict: true,
            ..Variant::default()
        },
    )
    .unwrap();
}

/// `batch_rows` 1 and 7: batch boundaries at every row and every seventh row.
#[test]
fn batch_rows_one_and_seven() {
    let spec = option_spec();
    let built = build(&spec);
    for b in [1usize, 7] {
        verify(
            &spec,
            &built,
            &Variant {
                batch_rows: Some(b),
                ..Variant::default()
            },
        )
        .unwrap();
    }
    // also on the direct path with a chain
    let direct = long_spec(true, ColType::Long, 4096);
    let built = build(&direct);
    verify(
        &direct,
        &built,
        &Variant {
            batch_rows: Some(1),
            ..Variant::default()
        },
    )
    .unwrap();
}

// ---------------------------------------------------------------------------------------------------------
// (f) negative cases with the generator's own offsets
// ---------------------------------------------------------------------------------------------------------

/// Where and how a patched file fails.
#[derive(Debug)]
enum Outcome {
    Open(Error),
    Scan(Error),
    Stopped(Error),
    Read(Error),
    Ok,
}

impl Outcome {
    fn error(&self) -> Option<&Error> {
        match self {
            Outcome::Open(e) | Outcome::Scan(e) | Outcome::Stopped(e) | Outcome::Read(e) => Some(e),
            Outcome::Ok => None,
        }
    }
}

fn outcome(bytes: &[u8]) -> Outcome {
    let f = TempFile::write(bytes);
    let mut d = match open(&f.0, Options::default()) {
        Ok(d) => d,
        Err(e) => return Outcome::Open(e),
    };
    let names: Vec<String> = match d.scan() {
        Err(e) => return Outcome::Scan(e),
        Ok(c) => {
            if let CatalogEnd::Stopped(e) = &c.end {
                return Outcome::Stopped(e.clone_without_source());
            }
            c.tables.iter().map(|t| t.name.clone()).collect()
        }
    };
    for n in names {
        let ro = ReadOptions::default();
        let mut r = match d.read_table(TableRef::ByName(&n), &ro) {
            Ok(r) => r,
            Err(e) => return Outcome::Read(e),
        };
        loop {
            match r.next_batch() {
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(e) => return Outcome::Read(e),
            }
        }
    }
    Outcome::Ok
}

trait CloneErr {
    fn clone_without_source(&self) -> Error;
}

impl CloneErr for Error {
    fn clone_without_source(&self) -> Error {
        let mut e = Error::new(self.code, self.message.clone());
        e.offset = self.offset;
        e.table = self.table;
        e.part = self.part;
        e.column = self.column;
        e.row = self.row;
        e.length = self.length;
        e.type_code = self.type_code;
        e.warning = self.warning;
        e
    }
}

/// A conventional table of three columns (ID, V, L LONG) and a LOB table, both with rows.
fn negative_spec() -> Spec {
    let t1 = table(
        "T_A",
        vec![
            col("ID", ColType::NumberInt(5)),
            col(
                "V",
                ColType::Varchar2 {
                    n: 30,
                    chars: false,
                },
            ),
            col("L", ColType::Long),
        ],
        (0..4)
            .map(|i| vec![id_val(i), txt(&format!("value {i}")), txt("long text")])
            .collect(),
    );
    let t2 = table(
        "T_B",
        vec![
            col("ID", ColType::NumberInt(5)),
            col("C", ColType::Clob),
            col("B", ColType::Blob),
        ],
        (0..3)
            .map(|i| {
                vec![
                    id_val(i),
                    txt(&format!("clob {i}")),
                    Cell::Bytes(vec![i as u8; 5]),
                ]
            })
            .collect(),
    );
    spec_of(vec![t1, t2])
}

fn expect(o: &Outcome, code: ErrorCode, offset: u64, what: &str) {
    let e = o
        .error()
        .unwrap_or_else(|| panic!("{what}: the file was accepted"));
    assert_eq!(e.code, code, "{what}: {e}");
    assert_eq!(e.offset, Some(offset), "{what}: {e}");
}

/// Replaces `n` bytes at `at` and keeps the file length (the tail zero fill absorbs a size change).
fn patch(bytes: &mut [u8], at: u64, with: &[u8]) {
    bytes[at as usize..at as usize + with.len()].copy_from_slice(with);
}

/// Removes `n` bytes at `at` and appends as many zeros: everything behind moves up, the length is kept.
fn cut_out(bytes: &mut Vec<u8>, at: u64, n: usize) {
    bytes.drain(at as usize..at as usize + n);
    bytes.extend(std::iter::repeat_n(0, n));
}

/// Inserts `data` at `at` and drops the same number of bytes from the end (the zero fill).
fn splice_in(bytes: &mut Vec<u8>, at: u64, data: &[u8]) {
    let n = data.len();
    for (i, b) in data.iter().enumerate() {
        bytes.insert(at as usize + i, *b);
    }
    assert!(
        bytes[bytes.len() - n..].iter().all(|b| *b == 0),
        "the fill must absorb the insert"
    );
    bytes.truncate(bytes.len() - n);
}

#[test]
fn the_generated_negative_base_is_clean() {
    let spec = negative_spec();
    let built = build(&spec);
    assert!(matches!(outcome(&built.bytes), Outcome::Ok));
}

/// Architecture 1.4, row by row: the code and the offset of each refusal. The offsets are the generator's own.
#[test]
fn each_refusal_of_architecture_1_4_has_its_code_and_offset() {
    let spec = negative_spec();
    let built = build(&spec);
    let l = &built.layout;
    let a = &l.tables[0];
    let b = &l.tables[1];
    let good = built.bytes.clone();

    // byte 0 is not 03 [H1] -> E-NOT-EXP-FILE at 0
    let mut f = good.clone();
    f[0] = 0x04;
    expect(&outcome(&f), ErrorCode::NotExpFile, 0, "byte 0");

    // compressed input (gzip magic) is refused with its own message (architecture 1.4, first row)
    let mut f = good.clone();
    patch(&mut f, 0, &[0x1f, 0x8b, 0x08]);
    expect(&outcome(&f), ErrorCode::NotExpFile, 0, "gzip magic");
    assert!(outcome(&f).error().unwrap().message.contains("compressed"));

    // line 1 does not start with EXPORT:V [H3] -> E-NOT-EXP-FILE at the line
    let mut f = good.clone();
    patch(&mut f, 3 + 7, b"X");
    expect(&outcome(&f), ErrorCode::NotExpFile, 3, "line 1");

    // a header line is missing [H3]: the file ends inside the header -> E-BAD-HEADER
    let f = good[..12].to_vec();
    assert_eq!(
        outcome(&f).error().map(|e| e.code),
        Some(ErrorCode::BadHeader),
        "header cut"
    );
    // the user line does not start with U [H3]
    let mut f = good.clone();
    patch(&mut f, 3 + 17, b"X");
    expect(&outcome(&f), ErrorCode::BadHeader, 3 + 17, "user line");

    // RECORDLENGTH is not a decimal number / is 0 / puts the second record beyond the file [H3, H5]
    for (text, what) in [(&b"20x8"[..], "not a number"), (&b"0000"[..], "zero")] {
        let mut f = good.clone();
        patch(&mut f, l.rl_at, text);
        expect(&outcome(&f), ErrorCode::BadRecordLength, l.rl_at, what);
    }
    let small = build(&spec_of(vec![table("S", simple_cols(), three_rows())]));
    assert!(
        small.bytes.len() < 9999,
        "beyond-the-file case needs a small file"
    );
    let mut f = small.bytes.clone();
    patch(&mut f, small.layout.rl_at, b"9999");
    expect(
        &outcome(&f),
        ErrorCode::BadRecordLength,
        small.layout.rl_at,
        "beyond the file",
    );

    // the first item is neither METRICST nor METRICSU [H6] -> E-MISSING-METRICS-LINE at R + 0x104
    let mut f = good.clone();
    patch(&mut f, l.first_item, b"X");
    expect(
        &outcome(&f),
        ErrorCode::MissingMetricsLine,
        l.first_item,
        "first item",
    );

    // INSERT INTO where no table section is open [T1, T3] -> E-FRAMING at the INSERT INTO line
    let mut f = good.clone();
    cut_out(
        &mut f,
        a.table_line,
        (a.insert_line - a.table_line) as usize,
    );
    expect(
        &outcome(&f),
        ErrorCode::Framing,
        a.table_line,
        "INSERT INTO without a table",
    );

    // the INSERT INTO line names one column fewer than the descriptors [T3, T4] -> E-FRAMING
    let line_end = a.count_at as usize; // the LF of the INSERT INTO line stands at count_at - 1
    let insert_text =
        String::from_utf8(good[a.insert_line as usize..line_end - 1].to_vec()).unwrap();
    let shorter = insert_text.replace(", \"L\"", "").replace(", :3", "");
    assert_ne!(shorter, insert_text);
    let removed = insert_text.len() - shorter.len();
    let mut f2 = good[..a.insert_line as usize].to_vec();
    f2.extend(shorter.as_bytes());
    f2.extend(&good[line_end - 1..]);
    f2.extend(std::iter::repeat_n(0, removed));
    let o = outcome(&f2);
    let e = o.error().expect("column count mismatch is refused");
    assert_eq!(e.code, ErrorCode::Framing, "{e}");
    assert_eq!(
        (e.offset, e.table, e.length),
        (Some(a.insert_line), Some(1), Some(2)),
        "{e}"
    );
    let _ = removed;

    // a second INSERT INTO in a section without a naming line [T10] -> E-FRAMING at the second INSERT INTO
    let mut f = good.clone();
    let part = good[a.insert_line as usize..a.after_part as usize].to_vec();
    splice_in(&mut f, a.after_part, &part);
    expect(
        &outcome(&f),
        ErrorCode::Framing,
        a.after_part,
        "second INSERT INTO",
    );

    // a column field of length 0 [R8] and 0xfff0 [R8] -> E-FRAMING at the length field
    for (bytes, what) in [
        ([0x00u8, 0x00], "length 0"),
        ([0xf0, 0xff], "length 0xfff0"),
    ] {
        let mut f = good.clone();
        let at = a.rows[1].fields[1].len_at;
        patch(&mut f, at, &bytes);
        expect(&outcome(&f), ErrorCode::Framing, at, what);
    }

    // a conventional-path LONG field with a u16 of 0xfffb or 0xfffc [R8] v3.9 (ADR-99 item 1) ->
    // E-UNSUPPORTED-LAYOUT at the length field (every other u16 from 0x8000 is a plain length or a form since R-040
    // and R-048: see `conventional_long_refusals_have_their_code_offset_and_message`)
    let at = a.rows[1].fields[2].len_at;
    for u in [0xfffbu16, 0xfffc] {
        let mut f = good.clone();
        patch(&mut f, at, &u.to_le_bytes());
        let o = outcome(&f);
        expect(
            &o,
            ErrorCode::UnsupportedLayout,
            at,
            "conventional LONG 0xfffb or 0xfffc",
        );
        assert_eq!(o.error().unwrap().length, Some(u64::from(u)));
    }

    // LOB value area: k differs from the non-NULL LOB columns of the row [L3] -> E-FRAMING; a u16 that is neither
    // a length nor `fc ff` [L4] -> E-FRAMING at the value
    let mut f = good.clone();
    let at = b.rows[1].tail_at;
    patch(&mut f, at, &[0x03, 0x00]);
    let o = outcome(&f);
    let e = o.error().expect("k mismatch is refused");
    assert_eq!(e.code, ErrorCode::Framing, "{e}");
    assert_eq!(
        (e.offset, e.table, e.row),
        (Some(at), Some(2), Some(2)),
        "{e}"
    );
    let mut f = good.clone();
    let at = b.rows[1].lob_values_at[0];
    patch(&mut f, at, &[0xfd, 0xff]);
    expect(&outcome(&f), ErrorCode::Framing, at, "LOB value u16");

    // a type code outside the 16 of T5 (BFILE 114) [X1] -> E-UNSUPPORTED-TYPE at the descriptor; the scan stops
    let mut f = good.clone();
    let at = a.desc_at[1];
    patch(&mut f, at, &[114, 0]);
    let o = outcome(&f);
    assert!(matches!(o, Outcome::Stopped(_)), "{o:?}");
    expect(&o, ErrorCode::UnsupportedType, at, "BFILE");
    assert_eq!(o.error().unwrap().type_code, Some(114));
    assert_eq!(o.error().unwrap().column, Some(2));

    // m neither 0 nor the column count [T6] -> E-UNSUPPORTED-LAYOUT at m
    let mut f = good.clone();
    patch(&mut f, a.m_at, &[0x05, 0x00]);
    expect(&outcome(&f), ErrorCode::UnsupportedLayout, a.m_at, "m = 5");
    // the path flag is neither 0 nor 1 [T6]
    let mut f = good.clone();
    patch(&mut f, a.flag_at, &[0x02, 0x00]);
    expect(
        &outcome(&f),
        ErrorCode::UnsupportedLayout,
        a.flag_at,
        "flag 2",
    );
    // a LOB table with the path flag 1 [X5]
    let mut f = good.clone();
    patch(&mut f, b.flag_at, &[0x01, 0x00]);
    expect(
        &outcome(&f),
        ErrorCode::UnsupportedLayout,
        b.flag_at,
        "LOB table with flag 1",
    );
    // a column entry with a name for a column that is not a LOB [L1]
    let mut f = good.clone();
    patch(&mut f, b.entry_at[0], &[0x01]);
    expect(
        &outcome(&f),
        ErrorCode::UnsupportedLayout,
        b.entry_at[0],
        "entry name for a non-LOB column",
    );

    // zero fill instead of the second EXIT line [E4] -> E-TRUNCATED
    let mut f = good.clone();
    patch(&mut f, l.exit2_at, &[0, 0, 0, 0, 0]);
    let e = outcome(&f)
        .error()
        .map(|e| (e.code, e.offset))
        .expect("refused");
    assert_eq!(e.0, ErrorCode::Truncated);
    assert_eq!(e.1, Some(l.exit2_at));
    // the file cut behind the first EXIT line
    let f = good[..l.exit2_at as usize].to_vec();
    let e = outcome(&f)
        .error()
        .map(|e| (e.code, e.offset))
        .expect("refused");
    assert_eq!(e.0, ErrorCode::Truncated);
}

// ---------------------------------------------------------------------------------------------------------
// T-19 (g): the conventional LONG forms of R-040 and their refusals (ADR-95), from the generator's own offsets
// ---------------------------------------------------------------------------------------------------------

/// A conventional table `(ID, L)` with one row per value; the LONG or LONG RAW column is declared second. An 11g
/// header: the combination of `long3_exp_11g` (table mode, 873 / 873).
fn long_form_spec(ty: ColType, values: Vec<Cell>) -> Spec {
    let rows: Vec<Vec<Cell>> = values
        .into_iter()
        .enumerate()
        .map(|(i, v)| vec![id_val(i as i128 + 1), v])
        .collect();
    let mut spec = spec_of(vec![table(
        "T_FORM",
        vec![col("ID", ColType::NumberInt(5)), col("L", ty)],
        rows,
    )]);
    spec.version = Version::V11;
    spec
}

fn text_of(n: usize) -> Cell {
    Cell::Text("y".repeat(n))
}

/// ADR-95 item 1, ADR-99 item 1, the shapes of the fixtures ([R2] v3.9): 32,767 and 32,768 bytes, 65,519 to 65,521 and
/// 65,525 to 65,529 bytes as one field (the u16 holds the whole length, bit 0x8000 included: `long3`, `long4`, `long5`
/// rows), for LONG and LONG RAW alike; no label.
#[test]
fn a_conventional_long_up_to_65529_bytes_is_one_field() {
    let sizes = [
        32_767usize,
        32_768,
        65_519,
        65_520,
        65_521,
        65_525,
        65_526,
        65_529,
    ];
    for ty in [ColType::Long, ColType::LongRaw] {
        let cells = sizes
            .iter()
            .map(|&n| {
                if ty == ColType::Long {
                    text_of(n)
                } else {
                    Cell::Bytes(vec![0xab; n])
                }
            })
            .collect();
        let spec = long_form_spec(ty.clone(), cells);
        let built = build(&spec);
        let tl = &built.layout.tables[0];
        for (row, &n) in sizes.iter().enumerate() {
            let f = &tl.rows[row].fields[1];
            assert!(!f.chunked && !f.four_byte, "{n} bytes are one field");
            let u = u16::from_le_bytes([
                built.bytes[f.len_at as usize],
                built.bytes[f.len_at as usize + 1],
            ]);
            assert_eq!(usize::from(u), n);
        }
        assert!(built.layout.unconfirmed.is_empty());
        // W-LONG-LENGTH is for a value above 100,000 bytes only (ADR-99 item 6): none of these gives one
        verify(&spec, &built, &Variant::default()).unwrap();
    }
}

/// ADR-99 item 1, the exact shapes of `long4_exp_<v>` rows 3 to 5 and `long5_exp_<v>` ([R2] v3.9): form (b) for 65,530
/// to 65,533 bytes is `fa ff`, the length as u16 n and `00 00`, then the bytes; `fa ff fa ff 00 00` + 65,530 bytes,
/// `fa ff fd ff 00 00` + 65,533 bytes. The value starts at the field's u16 + 6. For LONG and LONG RAW. No label.
#[test]
fn a_conventional_long_of_65530_to_65533_bytes_is_the_four_byte_form() {
    for ty in [ColType::Long, ColType::LongRaw] {
        for (n, head) in [
            (65_530usize, [0xfau8, 0xff, 0xfa, 0xff, 0x00, 0x00]),
            (65_531, [0xfa, 0xff, 0xfb, 0xff, 0x00, 0x00]),
            (65_532, [0xfa, 0xff, 0xfc, 0xff, 0x00, 0x00]),
            (65_533, [0xfa, 0xff, 0xfd, 0xff, 0x00, 0x00]),
        ] {
            let cell = if ty == ColType::Long {
                text_of(n)
            } else {
                Cell::Bytes(vec![0xab; n])
            };
            let spec = long_form_spec(ty.clone(), vec![cell]);
            let built = build(&spec);
            let f = &built.layout.tables[0].rows[0].fields[1];
            assert!(f.four_byte && !f.chunked);
            assert_eq!(f.len, n);
            let at = f.len_at as usize;
            assert_eq!(built.bytes[at..at + 6], head, "{n}");
            assert_eq!(f.value_at, f.len_at + 6);
            assert!(built.layout.unconfirmed.is_empty());
            verify(&spec, &built, &Variant::default()).unwrap();
        }
    }
}

/// ADR-95 item 2, ADR-99 item 2, the exact shapes of `long3_exp_<v>` rows 5 and 7 and `long4_exp_<v>` row 6: 65,534
/// bytes = 16 x 4,000 + 1,534, 65,535 bytes = 16 x 4,000 + 1,535 then `00 00`, 100,000 bytes = 25 x 4,000 then `00
/// 00`; also the other chunk sizes the seed draws (1 to 4,000); LONG and LONG RAW; no label.
#[test]
fn a_chunked_conventional_long_in_the_shapes_of_the_fixtures() {
    for ty in [ColType::Long, ColType::LongRaw] {
        for (total, shape) in [
            (65_534usize, [vec![4000usize; 16], vec![1534]].concat()),
            (65_535, [vec![4000usize; 16], vec![1535]].concat()),
            (100_000, vec![4000; 25]),
        ] {
            let cell = if ty == ColType::Long {
                text_of(total)
            } else {
                Cell::Bytes(vec![0xab; total])
            };
            let mut spec = long_form_spec(ty.clone(), vec![cell]);
            spec.chain_shape = Some(shape.clone());
            let built = build(&spec);
            let f = &built.layout.tables[0].rows[0].fields[1];
            assert!(f.chunked && !f.four_byte);
            assert_eq!((f.pieces, f.len), (shape.len(), total));
            assert_eq!(
                &built.bytes[f.len_at as usize..f.len_at as usize + 2],
                &[0xfd, 0xff]
            );
            // the value starts behind the first chunk's u16
            assert_eq!(f.value_at, f.len_at + 4);
            let mut at = f.len_at as usize + 2;
            for (k, sz) in shape.iter().enumerate() {
                assert_eq!(
                    built.bytes[at..at + 2],
                    (*sz as u16).to_le_bytes(),
                    "chunk {k}"
                );
                at += 2 + sz;
            }
            assert_eq!(at as u64, f.chunks_end_at);
            assert_eq!(&built.bytes[at..at + 2], &[0, 0]);
            assert!(built.layout.unconfirmed.is_empty());
            verify(&spec, &built, &Variant::default()).unwrap();
        }
        // a chunk size drawn by the seed: other shapes read back as well
        for total in [65_534usize, 65_535, 65_536, 150_000] {
            let cell = if ty == ColType::Long {
                text_of(total)
            } else {
                Cell::Bytes(vec![0xcd; total])
            };
            let spec = long_form_spec(ty.clone(), vec![cell]);
            let built = build(&spec);
            let f = &built.layout.tables[0].rows[0].fields[1];
            assert!(f.chunked && f.pieces >= total / 4000);
            verify(&spec, &built, &Variant::default()).unwrap();
        }
    }
}

/// The direct path: a chain of 100,000 bytes = 12 x 7,952 + 4,576 (`long3_exp_direct_<v>` row 7).
#[test]
fn a_direct_path_chain_of_100000_bytes() {
    let mut d = long_form_spec(ColType::Long, vec![text_of(100_000)]);
    d.direct = true;
    d.chain_shape = Some([vec![7952; 12], vec![4576]].concat());
    let built = build(&d);
    let f = &built.layout.tables[0].rows[0].fields[1];
    assert_eq!((f.pieces, f.len, f.chunked), (13, 100_000, false));
    // `9f10`: 7,952 | 0x8000; the last piece `11e0` = 4,576
    assert_eq!(
        &built.bytes[f.len_at as usize..f.len_at as usize + 2],
        &[0x10, 0x9f]
    );
    verify(&d, &built, &Variant::default()).unwrap();
}

/// ADR-99 item 6: a LONG or LONG RAW above 100,000 bytes gives `W-LONG-LENGTH` (the read of `verify` expects exactly
/// one for the column), 100,000 bytes none.
#[test]
fn a_long_above_100000_bytes_gives_w_long_length_and_100000_none() {
    for ty in [ColType::Long, ColType::LongRaw] {
        for (n, warns) in [(100_000usize, 0usize), (100_001, 1)] {
            let cell = if ty == ColType::Long {
                text_of(n)
            } else {
                Cell::Bytes(vec![0x11; n])
            };
            let spec = long_form_spec(ty.clone(), vec![cell]);
            assert_eq!(long_columns_over_100k(&spec.tables[0], None), warns);
            let built = build(&spec);
            verify(&spec, &built, &Variant::default()).unwrap();
        }
    }
}

/// ADR-99 items 1 to 3, the negative cases with the generator's own offsets: a conventional LONG or LONG RAW u16
/// 0xfffb or 0xfffc is `E-UNSUPPORTED-LAYOUT` at the u16 with the exact message; form (b) with n outside 0xfffa to
/// 0xfffd is `E-FRAMING` at n, with z != 0 at z; a chunk u16 from 0xfff0 is `E-FRAMING` at that u16; the u16 values
/// the retired R-048 stops covered (0xfff1 for LONG, 0x8000 and 0xfff0 for LONG RAW) read.
#[test]
fn conventional_long_refusals_have_their_code_offset_and_message() {
    const STOP: &str =
        "conventional LONG or LONG RAW field starting 0xfffb or 0xfffc, not in the fixtures [R8]";
    const NMSG: &str = "conventional LONG or LONG RAW 4-byte length outside 65,530 to 65,533";
    const ZMSG: &str =
        "conventional LONG or LONG RAW 4-byte length whose upper two bytes are not 00 00";
    for ty in [ColType::Long, ColType::LongRaw] {
        let cells = |a: usize, b: usize| {
            if ty == ColType::Long {
                vec![text_of(a), text_of(b)]
            } else {
                vec![Cell::Bytes(vec![1; a]), Cell::Bytes(vec![2; b])]
            }
        };
        let spec = long_form_spec(ty.clone(), cells(10, 20));
        let good = build(&spec);
        let at = good.layout.tables[0].rows[1].fields[1].len_at;
        for u in [0xfffbu16, 0xfffc] {
            let mut f = good.bytes.clone();
            patch(&mut f, at, &u.to_le_bytes());
            let o = outcome(&f);
            expect(&o, ErrorCode::UnsupportedLayout, at, "0xfffb or 0xfffc");
            let e = o.error().unwrap();
            assert_eq!(e.message, STOP);
            assert_eq!(
                (e.table, e.row, e.column, e.length),
                (Some(1), Some(2), Some(2), Some(u64::from(u)))
            );
        }
        // 0 and `ff ff` stay Framing
        for u in [0u16, 0xffff] {
            let mut f = good.bytes.clone();
            patch(&mut f, at, &u.to_le_bytes());
            expect(&outcome(&f), ErrorCode::Framing, at, "0 and ff ff");
        }
        assert!(matches!(outcome(&good.bytes), Outcome::Ok));

        // form (b): n outside 0xfffa..=0xfffd -> Framing at n; z != 0 -> Framing at z
        let spec = long_form_spec(ty.clone(), cells(65_531, 20));
        let good = build(&spec);
        let f0 = &good.layout.tables[0].rows[0].fields[1];
        assert!(f0.four_byte);
        for (off, bytes, msg, length) in [
            (2u64, 0xfff9u16, NMSG, 0xfff9u64),
            (2, 0xfffe, NMSG, 0xfffe),
            (4, 1, ZMSG, 1),
        ] {
            let mut f = good.bytes.clone();
            let at = f0.len_at + off;
            patch(&mut f, at, &bytes.to_le_bytes());
            let o = outcome(&f);
            expect(&o, ErrorCode::Framing, at, "form (b) n or z");
            let e = o.error().unwrap();
            assert_eq!(e.message, msg);
            assert_eq!(
                (e.table, e.row, e.column, e.length),
                (Some(1), Some(1), Some(2), Some(length))
            );
        }

        // a chunk u16 from 0xfff0 (the first chunk of a chunked value) -> E-FRAMING at that u16
        let cell = if ty == ColType::Long {
            text_of(70_000)
        } else {
            Cell::Bytes(vec![3; 70_000])
        };
        let spec = long_form_spec(ty.clone(), vec![cell]);
        let good = build(&spec);
        let f0 = &good.layout.tables[0].rows[0].fields[1];
        assert!(f0.chunked);
        for u in [0xfff0u16, 0xfffe] {
            let mut f = good.bytes.clone();
            let at = f0.len_at + 2;
            patch(&mut f, at, &u.to_le_bytes());
            let o = outcome(&f);
            expect(&o, ErrorCode::Framing, at, "chunk u16 from 0xfff0");
            let e = o.error().unwrap();
            assert_eq!(
                e.message,
                "conventional LONG or LONG RAW chunk length from 0xfff0"
            );
            assert_eq!((e.table, e.row, e.column), (Some(1), Some(1), Some(2)));
        }
        // a chunk u16 of 0xffef is the largest chunk: it is read (and misframes what follows, never a panic)
        assert!(matches!(outcome(&good.bytes), Outcome::Ok));
    }
}

/// ADR-95 item 2, ADR-99 item 2: a chunked total below 65,534 bytes (and `fd ff 00 00`, an empty value) is decoded
/// with one `W-UNTESTED-LAYOUT` per column at the first such value's first data byte; `strict` makes it `E-STRICT`.
/// The file carries the variant `LongChunkedBelow65534`. A total of 65,534 bytes gives none and no label (M6).
#[test]
fn a_chunked_total_below_65534_bytes_is_read_with_a_warning() {
    for ty in [ColType::Long, ColType::LongRaw] {
        let val = |n: usize| {
            if ty == ColType::Long {
                text_of(n)
            } else {
                Cell::Bytes(vec![0x5a; n])
            }
        };
        for (values, first_row) in [
            (vec![val(100), val(50_000), val(200)], 1usize),
            (vec![val(70_000), val(5)], 2),
            (vec![val(65_533)], 1),
            (vec![val(0)], 1),
        ] {
            let mut spec = long_form_spec(ty.clone(), values);
            spec.chunk_everything = true;
            let built = build(&spec);
            assert_eq!(
                built.layout.unconfirmed,
                [Unconfirmed::LongChunkedBelow65534].into_iter().collect()
            );
            let f = &built.layout.tables[0].rows[first_row - 1].fields[1];
            let file = TempFile::write(&built.bytes);
            let mut d = open(&file.0, Options::default()).unwrap();
            d.scan().unwrap();
            let ws: Vec<_> = d
                .warnings()
                .iter()
                .filter(|w| w.code == WarningCode::UntestedLayout)
                .collect();
            assert_eq!(ws.len(), 1, "{:?}", d.warnings());
            assert_eq!(ws[0].offset, Some(f.value_at));
            assert_eq!(
                (ws[0].table, ws[0].row, ws[0].column),
                (Some(1), Some(first_row as u64), Some(2))
            );
            assert_eq!(
                ws[0].message,
                "conventional LONG or LONG RAW in the chunked form with fewer than 65,534 bytes"
            );
            // the values are delivered as written
            let (b, _) = try_read(&mut d, "T_FORM", &ReadOptions::default()).unwrap();
            assert_eq!(rows(&b), expected_rows(&spec, &spec.tables[0], None));
            // strict: E-STRICT
            let o = Options {
                strict: true,
                ..Options::default()
            };
            let r = open(&file.0, o).and_then(|mut d| d.scan().map(|_| ()));
            assert_eq!(r.unwrap_err().code, ErrorCode::Strict);
        }
        // 65,534 bytes chunked: no warning, no label (the smallest chunked size of the fixtures)
        let mut spec = long_form_spec(ty.clone(), vec![val(65_534)]);
        spec.chunk_everything = true;
        let built = build(&spec);
        assert!(built.layout.unconfirmed.is_empty());
        let file = TempFile::write(&built.bytes);
        let mut d = open(&file.0, Options::default()).unwrap();
        d.scan().unwrap();
        assert!(d.warnings().is_empty(), "{:?}", d.warnings());
    }
}

/// R-052: 65,522 to 65,524 bytes are written as one field only with the label `LongOneFieldFff2ToFff4`; the reader
/// reads the whole value with one `W-UNTESTED-LAYOUT` per column at the field's u16 (table 1, row, column 2); `strict`
/// makes it `E-STRICT`; both types.
#[test]
fn a_length_of_0xfff2_to_0xfff4_is_written_labelled_and_read_with_a_warning() {
    for ty in [ColType::Long, ColType::LongRaw] {
        for n in [65_522usize, 65_523, 65_524] {
            let val = |m: usize| {
                if ty == ColType::Long {
                    text_of(m)
                } else {
                    Cell::Bytes(vec![0x5a; m])
                }
            };
            let spec = long_form_spec(ty.clone(), vec![val(10), val(n), val(65_522)]);
            let built = build(&spec);
            assert_eq!(
                built.layout.unconfirmed,
                [Unconfirmed::LongOneFieldFff2ToFff4].into_iter().collect()
            );
            let f = &built.layout.tables[0].rows[1].fields[1];
            assert!(!f.chunked && !f.four_byte);
            let u = u16::from_le_bytes([
                built.bytes[f.len_at as usize],
                built.bytes[f.len_at as usize + 1],
            ]);
            assert_eq!(usize::from(u), n);
            let file = TempFile::write(&built.bytes);
            let mut d = open(&file.0, Options::default()).unwrap();
            d.scan().unwrap();
            let ws: Vec<_> = d
                .warnings()
                .iter()
                .filter(|w| w.code == WarningCode::UntestedLayout)
                .collect();
            // once per column: the second value adds none
            assert_eq!(ws.len(), 1, "{n}: {:?}", d.warnings());
            assert_eq!(ws[0].offset, Some(f.len_at));
            assert_eq!(
                (ws[0].table, ws[0].row, ws[0].column),
                (Some(1), Some(2), Some(2))
            );
            assert_eq!(
                ws[0].message,
                "conventional LONG or LONG RAW length 0xfff2 to 0xfff4 read as one field, not in the fixtures (R-052)"
            );
            let (b, w) = try_read(&mut d, "T_FORM", &ReadOptions::default()).unwrap();
            assert!(w.is_empty());
            assert_eq!(rows(&b), expected_rows(&spec, &spec.tables[0], None));
            let o = Options {
                strict: true,
                ..Options::default()
            };
            let r = open(&file.0, o).and_then(|mut d| d.scan().map(|_| ()));
            assert_eq!(r.unwrap_err().code, ErrorCode::Strict);
        }
    }
}

/// ADR-94 item 2 / (g)(6): an 18c table-mode file with E1 after flag 1 holds no variant (the file `seg_exp_direct_18c`
/// shows it); it is the 18c owner-mode file that keeps the label.
#[test]
fn the_e1_label_applies_to_18c_owner_mode_only() {
    for (version, mode, labelled) in [
        (Version::V18, Mode::Tables, false),
        (Version::V18, Mode::Users, true),
        (Version::V11, Mode::Tables, false),
        (Version::V11, Mode::Users, false),
    ] {
        let mut e1 = table("E", simple_cols(), vec![]);
        e1.empty_form = EmptyForm::E1;
        let mut spec = spec_of(vec![table("A", simple_cols(), three_rows()), e1]);
        spec.direct = true;
        spec.version = version;
        spec.mode = mode;
        let built = build(&spec);
        assert_eq!(
            !built.layout.unconfirmed.is_empty(),
            labelled,
            "{version:?} {mode:?}: {:?}",
            built.layout.unconfirmed
        );
    }
}

/// (g)(1), (2), ADR-99: the large LONG and LONG RAW sizes the generator draws: 32,768 to 65,521 and 65,525 to 65,529
/// (one field), 65,530 to 65,533 (form (b)), 65,534 to 150,000 (chunked or a direct chain), never 65,522 to 65,524
/// (R-052); the edge values appear. Over the seeds, large values occur in the `long3` combination only and, for both
/// types, in every form.
#[test]
fn the_generator_draws_large_long_sizes_outside_the_fff2_gap_and_only_in_the_fixtures_combination()
{
    let mut rng = common::synth::Rng::new(7);
    let mut seen_edge = std::collections::BTreeSet::new();
    for _ in 0..50_000 {
        let n = big_long_size(&mut rng);
        assert!(
            (32_768..=65_521).contains(&n)
                || (65_525..=65_529).contains(&n)
                || (65_530..=65_533).contains(&n)
                || (65_534..=150_000).contains(&n),
            "a large LONG of {n} bytes is in or outside the gap"
        );
        seen_edge.insert(n);
    }
    for e in [
        32_768usize,
        65_521,
        65_525,
        65_529,
        65_530,
        65_533,
        65_534,
        65_535,
        100_000,
        150_000,
    ] {
        assert!(seen_edge.contains(&e), "edge {e} never drawn");
    }
    // [type][form]: plain, four-byte, chunked, chain
    let mut forms = [[0usize; 4]; 2];
    for n in 0..4_000u64 {
        let spec = random_spec(&params_from_seed(n));
        let combination = spec.mode == Mode::Tables && spec.client_set == 873 && spec.db_set == 873;
        // only a Spec with a large LONG value is built (building the others costs time and shows nothing here)
        let has_large = spec.tables.iter().any(|t| {
            t.columns.iter().enumerate().any(|(c, col)| {
                col.ty.is_long()
                    && t.rows.iter().any(|row| match &row[c] {
                        Cell::Text(s) => s.len() > 32_000,
                        Cell::Bytes(b) => b.len() > 32_000,
                        _ => false,
                    })
            })
        });
        if !has_large {
            continue;
        }
        let built = build(&spec);
        assert!(
            !built
                .layout
                .unconfirmed
                .contains(&Unconfirmed::LongOneFieldFff2ToFff4),
            "seed {n}: the random generator wrote 65,522 to 65,524 bytes"
        );
        for (t, tl) in spec.tables.iter().zip(&built.layout.tables) {
            for (c, ci) in t.columns.iter().enumerate().filter(|(_, c)| c.ty.is_long()) {
                let _ = ci;
                for (r, row) in t.rows.iter().enumerate() {
                    let len = match &row[c] {
                        Cell::Text(s) => s.len(),
                        Cell::Bytes(b) => b.len(),
                        _ => continue,
                    };
                    assert!(
                        !(65_522..=65_524).contains(&len),
                        "seed {n}: a LONG of {len} bytes (R-052)"
                    );
                    if len > 32_000 {
                        assert!(
                            combination,
                            "seed {n}: {len} bytes outside the fixtures' combination"
                        );
                        let order = &tl.file_order;
                        let f = &tl.rows[r].fields[order.iter().position(|&x| x == c).unwrap()];
                        let ty = usize::from(t.columns[c].ty == ColType::LongRaw);
                        if tl.direct {
                            forms[ty][3] += usize::from(f.pieces > 1);
                        } else if f.chunked {
                            forms[ty][2] += 1;
                        } else if f.four_byte {
                            forms[ty][1] += 1;
                        } else {
                            forms[ty][0] += 1;
                        }
                    }
                }
            }
        }
    }
    for (ty, f) in ["LONG", "LONG RAW"].iter().zip(&forms) {
        assert!(f.iter().all(|&n| n >= 3), "{ty}: {f:?}");
    }
}

proptest! {
    #![proptest_config(config())]

    /// Truncation at a random offset: a typed error whose offset is at most the file length (the rule of
    /// `t14_damage`), or a clean read of fewer rows. The Spec and the cut come from the seed.
    #[test]
    fn a_truncated_synthetic_file_gives_a_typed_error_within_the_file(p in params(), cut in any::<u64>()) {
        let spec = random_spec(&p);
        let built = build(&spec);
        let t = (cut % (built.bytes.len() as u64 + 1)) as usize;
        let o = outcome(&built.bytes[..t]);
        // a file is read completely exactly when both EXIT lines are present (a cut inside the zero fill)
        prop_assert_eq!(o.error().is_none(), t as u64 >= built.layout.data_end, "seed {} cut {} unconfirmed {:?}", p.origin, t, built.layout.unconfirmed);
        match o.error() {
            None => {}
            Some(e) => {
                prop_assert!(oradmp_core::ErrorCode::ALL.contains(&e.code), "seed {} unconfirmed {:?}", p.origin, built.layout.unconfirmed);
                prop_assert!(e.to_string().starts_with("[E-"), "seed {} unconfirmed {:?}", p.origin, built.layout.unconfirmed);
                if let Some(off) = e.offset {
                    prop_assert!(off <= t as u64, "seed {} unconfirmed {:?} cut {t}: offset {off} beyond the cut", p.origin, built.layout.unconfirmed);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// (g) fuzz seeds
// ---------------------------------------------------------------------------------------------------------

/// Writes `ORADMP_SEED_COUNT` (default 200) synthetic files into the folder named by `ORADMP_SEED_DIR` as
/// `synth__<n>.dmp`. Ignored in a normal run; `tools/fuzz_seeds.sh` runs it:
/// `cargo test -p oradmp-core --test t17_synth -- --ignored write_fuzz_seeds`.
#[test]
#[ignore = "run by tools/fuzz_seeds.sh"]
fn write_fuzz_seeds() {
    let dir = std::env::var("ORADMP_SEED_DIR").expect("ORADMP_SEED_DIR");
    let count: u64 = std::env::var("ORADMP_SEED_COUNT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);
    std::fs::create_dir_all(&dir).unwrap();
    for n in 0..count {
        let mut p = params_from_seed(0x5eed_f11e + n);
        p.max_rows = p.max_rows.min(60);
        let spec = random_spec(&p);
        let built = build(&spec);
        std::fs::write(
            PathBuf::from(&dir).join(format!("synth__{n:04}.dmp")),
            &built.bytes,
        )
        .unwrap();
    }
}
