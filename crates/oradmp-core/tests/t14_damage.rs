//! Task T-14 (f): damage properties (architecture 8.3 (a)) over the exp files of `exp_catalog.tsv`.
//!
//! * `truncation_gives_ok_or_a_typed_error`: a file cut at a random offset t is opened, scanned and every table is
//!   read to its end with `on_value_error = Null`. Every result is `Ok` or an error whose code is in
//!   `ErrorCode::ALL`, whose text starts with `[E-` and holds no U+0000, and whose offset is at most t. No table
//!   yields more rows than the golden catalog's `num_rows` for the same ordinal (`tests/golden/exp_catalog.tsv`,
//!   the Analyst's `exp_parse.py` through `gen_exp_catalog.py`).
//! * `bit_flips_give_ok_or_a_typed_error`: 1 to 8 random bit flips; the same checks with the file length as
//!   the bound of every offset.
//!
//! The number of cases is proptest's default (256), overridable by `PROPTEST_CASES`. A shrunk failure is a
//! finding of T-14 (e); the shrunk input is in the failure message (failure persistence is off: no
//! `*.proptest-regressions` file is written).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

mod common;

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::OnceLock;

use common::*;
use oradmp_core::dump::{open, CatalogEnd, Options, TableRef};
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::{Error, ErrorCode, WarningCode};
use proptest::prelude::*;

/// The file name and the golden row counts that cap the rows of its tables.
type GoldenRows<'a> = Option<(&'a str, &'a HashMap<(String, u32), u64>)>;

type Golden = (Vec<String>, HashMap<(String, u32), u64>);

/// The files of the golden catalog (sorted, `folder/name`) and the golden row count per (file, ordinal).
fn golden() -> &'static Golden {
    static G: OnceLock<Golden> = OnceLock::new();
    G.get_or_init(|| {
        let text = std::fs::read_to_string(root().join("tests/golden/exp_catalog.tsv")).unwrap();
        let mut files: Vec<String> = Vec::new();
        let mut rows = HashMap::new();
        for l in text
            .lines()
            .filter(|l| !l.starts_with('#') && !l.starts_with("file\t") && !l.is_empty())
        {
            let c: Vec<&str> = l.split('\t').collect();
            // columns 0 file, 1 ordinal, 7 num_rows (header of the generator)
            if !files.iter().any(|f| f == c[0]) {
                files.push(c[0].to_string());
            }
            rows.insert(
                (c[0].to_string(), c[1].parse().unwrap()),
                c[7].parse().unwrap(),
            );
        }
        files.sort();
        (files, rows)
    })
}

fn check_error(e: &Error, bound: u64, what: &str) -> Result<(), TestCaseError> {
    let text = e.to_string();
    prop_assert!(text.starts_with("[E-"), "{what}: text of {:?}", e.code);
    prop_assert!(
        ErrorCode::ALL.contains(&e.code),
        "{what}: code {:?}",
        e.code
    );
    prop_assert!(
        e.code != ErrorCode::Internal,
        "{what}: the core built E-INTERNAL"
    );
    prop_assert!(
        !text.contains('\u{0}'),
        "{what}: NUL in the text of {:?}",
        e.code
    );
    if let Some(o) = e.offset {
        prop_assert!(
            o <= bound,
            "{what}: error {:?} offset {o} beyond {bound}",
            e.code
        );
    }
    Ok(())
}

/// ADR-65 item 3: a warning's offset is at most the bound too (the cut for a truncation, the file length for a
/// bit flip).
fn check_warning_offset(
    offset: impl Into<Option<u64>>,
    bound: u64,
    what: &str,
) -> Result<(), TestCaseError> {
    if let Some(o) = offset.into() {
        prop_assert!(o <= bound, "{what}: offset {o} beyond {bound}");
    }
    Ok(())
}

/// Opens, scans and reads every table of `bytes` (written below `target/`); `golden_rows` caps the rows per
/// ordinal. All errors are checked against `bound`.
fn run_damaged(
    bytes: &[u8],
    name: &str,
    bound: u64,
    golden_rows: GoldenRows<'_>,
) -> Result<(), TestCaseError> {
    let dir = target_dir("t14-damage");
    let path: PathBuf = dir.join(format!("{name}-{}.dmp", std::process::id()));
    std::fs::write(&path, bytes).unwrap();
    let r = run_path(&path, bound, golden_rows);
    let _ = std::fs::remove_file(&path);
    r
}

fn run_path(
    path: &std::path::Path,
    bound: u64,
    golden_rows: GoldenRows<'_>,
) -> Result<(), TestCaseError> {
    let mut dump = match open(path, Options::default()) {
        Ok(d) => d,
        Err(e) => return check_error(&e, bound, "open"),
    };
    let ordinals: Vec<u32> = match dump.scan() {
        Ok(c) => {
            if let CatalogEnd::Stopped(e) = &c.end {
                check_error(e, bound, "scan (stopped)")?;
            }
            c.tables.iter().map(|t| t.ordinal).collect()
        }
        Err(e) => return check_error(&e, bound, "scan"),
    };
    for w in dump.warnings() {
        check_warning_offset(w.offset, bound, "scan warning")?;
    }
    let options = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    for ordinal in ordinals {
        let mut batches = match dump.read_table(TableRef::ByOrdinal(ordinal), &options) {
            Ok(b) => b,
            Err(e) => {
                check_error(&e, bound, "read_table")?;
                continue;
            }
        };
        let mut rows = 0u64;
        loop {
            match batches.next_batch() {
                Ok(Some(b)) => rows += b.num_rows() as u64,
                Ok(None) => break,
                Err(e) => {
                    check_error(&e, bound, "next_batch")?;
                    break;
                }
            }
        }
        for w in batches.warnings() {
            check_warning_offset(w.offset, bound, "reader warning")?;
            prop_assert!(w.to_string().starts_with("[W-"), "warning text");
            prop_assert!(!w.to_string().contains('\u{0}'), "NUL in a warning text");
            prop_assert!(WarningCode::ALL.contains(&w.code));
        }
        if let Some((file, g)) = golden_rows {
            let cap = g.get(&(file.to_string(), ordinal));
            prop_assert!(
                cap.is_some(),
                "{file}: table {ordinal} is not in the golden catalog"
            );
            let cap = *cap.unwrap();
            prop_assert!(
                rows <= cap,
                "{file}: table {ordinal} yields {rows} rows, the catalog has {cap}"
            );
        }
    }
    Ok(())
}

proptest! {
    // no `*.proptest-regressions` file (review 06b SF-5): a failure prints its shrunk input in the message
    #![proptest_config(ProptestConfig {
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn truncation_gives_ok_or_a_typed_error(idx in any::<usize>(), cut in any::<u64>()) {
        let (files, rows) = golden();
        let file = &files[idx % files.len()];
        let bytes = std::fs::read(fixtures().join(file)).unwrap();
        let t = (cut % (bytes.len() as u64 + 1)) as usize;
        run_damaged(&bytes[..t], "truncation", t as u64, Some((file, rows)))?;
    }

    #[test]
    fn bit_flips_give_ok_or_a_typed_error(
        idx in any::<usize>(),
        flips in proptest::collection::vec((any::<u64>(), 0u8..8), 1..=8),
    ) {
        let (files, _) = golden();
        let file = &files[idx % files.len()];
        let mut bytes = std::fs::read(fixtures().join(file)).unwrap();
        let len = bytes.len();
        for (at, bit) in flips {
            let i = (at % len as u64) as usize;
            bytes[i] ^= 1 << bit;
        }
        run_damaged(&bytes, "bitflip", len as u64, None)?;
    }
}

/// The strategy ranges over all the files of the golden catalog (`idx % files.len()`, no literal): the catalog
/// lists exactly the files of `TOTALS` (T-19b: 229, the ten of Analyst session 17 and the 18 of sessions 19 and 20
/// included).
#[test]
fn the_golden_catalog_has_all_the_files() {
    let (files, rows) = golden();
    assert_eq!(files.len(), TOTALS.files);
    assert_eq!(files.len(), exp_files().len());
    for f in [
        "18c/long3_exp_18c.dmp",
        "11g/tsp_exp_direct_11g.dmp",
        "18c/seg_exp_direct_18c.dmp",
        "18c/long4_exp_18c.dmp",
        "11g/longraw5_exp_direct_11g.dmp",
        "11g/tsltz2_exp_11g.dmp",
    ] {
        assert!(files.iter().any(|x| x == f), "{f}");
    }
    assert_eq!(rows.len(), TOTALS.sections);
}

/// The check is real: a table that yields one row more than the catalog lists is a failure (the cap is
/// compared with the rows read), an error offset beyond the bound is one too.
#[test]
fn the_checks_fail_on_a_wrong_cap_and_a_wrong_bound() {
    let (files, rows) = golden();
    let file = files
        .iter()
        .find(|f| f.ends_with("t_num_exp_18c.dmp"))
        .unwrap();
    let bytes = std::fs::read(fixtures().join(file)).unwrap();
    let len = bytes.len() as u64;
    assert!(run_damaged(&bytes, "selftest-ok", len, Some((file, rows))).is_ok());
    // every table of the file capped one row lower than its golden count
    let mut low = rows.clone();
    for ((f, _), v) in low.iter_mut() {
        if f == file {
            *v -= 1;
        }
    }
    assert!(run_damaged(&bytes, "selftest-cap", len, Some((file, &low))).is_err());
    // an error offset beyond the bound: cut the file at 100 and claim the bound is 10
    assert!(run_damaged(&bytes[..100], "selftest-bound", 10, None).is_err());
}

/// ADR-65 item 3 (review 04 N-3): one deterministic case per "file ends" site of the core
/// (`grep -n "file ends" crates/oradmp-core/src`), each a named fixture cut at an offset, with the exact code, message
/// and offset the site returns. Sampling cannot pin these: a site that one cut in millions reaches would survive a
/// mutation of its offset (a1, a2, c2 of the T-13i note). The expected offset of every case is where the failed read
/// stood (`io.rs` `truncated()`: the current position of the stream), not a rule of the rules file: the first byte of
/// the length field, the value bytes, the row or the item that should stand (the scan reads every row, so the error
/// comes from `scan`). So the "inside a line" case expects the end of the file, not the start of the line (review 05
/// N-3).
///
/// Offsets come from the Analyst's `docs/formats/tools/exp_parse.py FILE --rows` (read-only, run in T-13i):
///
/// `18c/t_num_exp_18c.dmp` (16,384 bytes), T_NUM:
///
/// ```text
///   row  16 0x22e4: @22e4 [2] c1 11 | @22e8 [2] c0 02
///   row  17 0x22ee: @22ee [2] c1 12 | @22f2 [5] c2 02 18 2e 3d      (the length u16 stands at @22ee and @22f2)
///   row  33 0x244d: @244d [2] c1 22 | @2451 NULL
///   0x2246 rows: 33; end marker ff ff at 0x2455; LF at 0x2457
///   0x254d line  [4+LF] 'EXIT'      0x2552 line  [4+LF] 'EXIT'      0x2557 zero fill to end of file (0x4000)
/// ```
///
/// `18c/lob_exp_18c.dmp`, T_LOB: `LOB value of row 5 col 2 @0x4358 long [200000] chunks 13` (`fc ff` at 0x4358, the
/// 8-byte total at 0x435a).
///
/// The 8-byte total of a long LOB value is read by `io.rs` `read_u64_le` (T-13j), so a short total is the
/// "file ends inside a field" case below (the last one). The site `exp/framing.rs` has a unit test over a `Cursor`
/// (no fixture has an unlisted introducing line).
#[test]
fn each_truncation_site_names_its_offset() {
    struct Case {
        site: &'static str,
        file: &'static str,
        cut: usize,
        message: &'static str,
        offset: u64,
        /// (table ordinal, row) when the error carries them
        place: Option<(u32, u64)>,
    }
    let t = "18c/t_num_exp_18c.dmp";
    let l = "18c/lob_exp_18c.dmp";
    let cases = [
        Case {
            site: "rows.rs row or end of rows: one byte of the row stands (@22ee, row 17)",
            file: t,
            cut: 0x22ef,
            message: "file ends where a row or the end of rows should stand",
            offset: 0x22ee,
            place: Some((1, 17)),
        },
        Case {
            site: "rows.rs row or end of rows: the file ends where row 17 should begin",
            file: t,
            cut: 0x22ee,
            message: "file ends where a row or the end of rows should stand",
            offset: 0x22ee,
            place: Some((1, 17)),
        },
        Case {
            site: "rows.rs row or end of rows: one byte of the end marker (@2455, row 34)",
            file: t,
            cut: 0x2456,
            message: "file ends where a row or the end of rows should stand",
            offset: 0x2455,
            place: Some((1, 34)),
        },
        Case {
            site: "io.rs inside a field: the V length u16 of row 17 (@22f2) has one byte",
            file: t,
            cut: 0x22f3,
            message: "file ends inside a field",
            offset: 0x22f2,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the V value of row 17 (@22f4, 5 bytes) has 2 bytes",
            file: t,
            cut: 0x22f6,
            message: "file ends inside a field",
            offset: 0x22f4,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the ID value of row 17 (@22f0, 2 bytes) is absent",
            file: t,
            cut: 0x22f0,
            message: "file ends inside a field",
            offset: 0x22f0,
            place: None,
        },
        Case {
            site: "io.rs inside a line: the first EXIT line (@254d) has no LF",
            file: t,
            cut: 0x2551,
            message: "file ends inside a line",
            offset: 0x2551,
            place: None,
        },
        Case {
            site: "items.rs before the second EXIT line: the file ends after the first (@2552)",
            file: t,
            cut: 0x2552,
            message: "file ends before the second EXIT line",
            offset: 0x2552,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the `fc ff` of a long LOB (@4358) has one byte",
            file: l,
            cut: 0x4359,
            message: "file ends inside a field",
            offset: 0x4358,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the 8-byte total of the long LOB (@435a, `read_u64_le`) has 3 bytes",
            file: l,
            cut: 0x435d,
            message: "file ends inside a field",
            offset: 0x435a,
            place: None,
        },
        // T-19 (ADR-95 item 2): the chunked conventional LONG of `long3_exp_18c.dmp` row 5 (`long3_check.py`: `fd ff`
        // at 0x3228f, first chunk u16 at 0x32291 = 4000, data from 0x32293, 16 chunks of 4,000 and one of 1,535, then
        // `00 00` at 0x3228f + 2 + 16 x 4,002 + 1,537 = 0x422b2)
        Case {
            site: "io.rs inside a field: the first chunk u16 of a chunked LONG (@32291) has one byte",
            file: "18c/long3_exp_18c.dmp",
            cut: 0x32292,
            message: "file ends inside a field",
            offset: 0x32291,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the first chunk of a chunked LONG (@32293, 4,000 bytes) has 10 bytes",
            file: "18c/long3_exp_18c.dmp",
            cut: 0x3229d,
            message: "file ends inside a field",
            offset: 0x32293,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the `00 00` that ends a chunked LONG (@422b2) has one byte",
            file: "18c/long3_exp_18c.dmp",
            cut: 0x422b3,
            message: "file ends inside a field",
            offset: 0x422b2,
            place: None,
        },
        // T-19b (ADR-99 item 1): the 4-byte length form of `long4_exp_18c.dmp` row 3 (`long4_check.py`: `fa ff` at
        // 0x22277, n = 0xfffa at 0x22279, z = `00 00` at 0x2227b, 65,530 bytes from 0x2227d)
        Case {
            site: "io.rs inside a field: the `fa ff` of a form (b) LONG (@22277) has one byte",
            file: "18c/long4_exp_18c.dmp",
            cut: 0x22278,
            message: "file ends inside a field",
            offset: 0x22277,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the n of a form (b) LONG (@22279) is absent",
            file: "18c/long4_exp_18c.dmp",
            cut: 0x22279,
            message: "file ends inside a field",
            offset: 0x22279,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the z of a form (b) LONG (@2227b) has one byte",
            file: "18c/long4_exp_18c.dmp",
            cut: 0x2227c,
            message: "file ends inside a field",
            offset: 0x2227b,
            place: None,
        },
        Case {
            site: "io.rs inside a field: the bytes of a form (b) LONG (@2227d, 65,530 bytes) are 4",
            file: "18c/long4_exp_18c.dmp",
            cut: 0x22281,
            message: "file ends inside a field",
            offset: 0x2227d,
            place: None,
        },
    ];
    for c in &cases {
        let bytes = std::fs::read(fixtures().join(c.file)).unwrap();
        let path = target_dir("t14-damage").join(format!("site-{}.dmp", std::process::id()));
        std::fs::write(&path, &bytes[..c.cut]).unwrap();
        let mut dump =
            open(&path, Options::default()).unwrap_or_else(|e| panic!("{}: open {e}", c.site));
        let e = dump
            .scan()
            .expect_err(&format!("{}: the scan of a cut file must fail", c.site));
        let _ = std::fs::remove_file(&path);
        assert_eq!(e.code, ErrorCode::Truncated, "{}: {e}", c.site);
        assert_eq!(e.message, c.message, "{}: {e}", c.site);
        assert_eq!(e.offset, Some(c.offset), "{}: {e}", c.site);
        assert!(c.offset <= c.cut as u64, "{}", c.site);
        if let Some((table, row)) = c.place {
            assert_eq!(
                (e.table, e.row),
                (Some(table), Some(row)),
                "{}: {e}",
                c.site
            );
        }
    }
}
