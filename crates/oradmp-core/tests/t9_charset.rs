//! Task T-9: character sets (architecture 6, ADR-5, ADR-16, ADR-33 items 2 and 8).
//!
//! Expected values come from the truth files of `corpus/fixtures` (`truth_text.txt`, `truth_nchar.txt`,
//! `truth_nchar2.txt`, `truth_probe11.txt`, `11g-cp1251/truth_ru.txt`, `truth_csmap.txt`), which hold the bytes
//! the database holds (`DUMP(x, 16)`). The test decodes those bytes on its own: UTF-8 and UTF-16 with the standard
//! library, CL8MSWIN1251 with the table Oracle itself produced (`truth_csmap.txt`, checked by the Analyst's
//! `tools/csmap_cmp.py` run on 18c: CL8MSWIN1251 equal to Python's cp1251 in 255 of 256 bytes and to the WHATWG
//! reading in 256 of 256; WE8MSWIN1252 251 and 256; US7ASCII 128 equal, 128 give U+FFFD), so the reader's decoders
//! are never compared with themselves. Patched copies are written below `target/`.
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

use arrow_array::RecordBatch;
use arrow_schema::DataType;
use common::*;
use oradmp_core::charset::{decode_text, has_replacement_char, Charset};
use oradmp_core::dump::{open, Options};
use oradmp_core::reader::{OnValueError, ReadOptions};
use oradmp_core::{ErrorCode, WarningCode};

// ---------------------------------------------------------------------------------------------
// Truth: DUMP fields and the Oracle map of CL8MSWIN1251
// ---------------------------------------------------------------------------------------------

/// The bytes of a `DUMP(x, 16)` field (`Typ=1 Len=3: 61,62,63`); `None` for `NULL`.
fn dump_bytes(field: &str) -> Option<Vec<u8>> {
    if field == "NULL" {
        return None;
    }
    let (_, hexes) = field.split_once(": ").unwrap();
    Some(if hexes.is_empty() {
        Vec::new()
    } else {
        hexes
            .split(',')
            .map(|h| u8::from_str_radix(h, 16).unwrap())
            .collect()
    })
}

fn utf16_be(b: &[u8]) -> String {
    assert_eq!(b.len() % 2, 0);
    let units: Vec<u16> = b
        .chunks(2)
        .map(|p| u16::from_be_bytes([p[0], p[1]]))
        .collect();
    String::from_utf16(&units).unwrap()
}

#[derive(Clone, Copy)]
enum Truth {
    Utf8,
    Utf16,
    Cp1251,
}

/// Oracle's map of the 256 bytes of one set (`truth_csmap.txt`): the UTF-16 units Oracle produced.
fn oracle_map(set: &str) -> Vec<String> {
    let mut map = vec![String::new(); 256];
    let mut n = 0;
    for l in truth("18c", "truth_csmap.txt") {
        if l[0] == set {
            let b = usize::from_str_radix(&l[1], 16).unwrap();
            map[b] = utf16_be(&dump_bytes(&l[2]).unwrap());
            n += 1;
        }
    }
    assert_eq!(n, 256, "{set}");
    map
}

fn cp1251_map() -> &'static Vec<String> {
    static MAP: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();
    MAP.get_or_init(|| oracle_map("CL8MSWIN1251"))
}

fn truth_text(field: &str, how: Truth) -> Option<String> {
    let b = dump_bytes(field)?;
    Some(match how {
        Truth::Utf8 => String::from_utf8(b).unwrap(),
        Truth::Utf16 => utf16_be(&b),
        Truth::Cp1251 => {
            let map = cp1251_map();
            b.iter().map(|x| map[usize::from(*x)].clone()).collect()
        }
    })
}

/// The database bytes of a text in CL8MSWIN1251, by the inverse of Oracle's map.
fn encode_cp1251(s: &str) -> Vec<u8> {
    static INVERSE: std::sync::OnceLock<HashMap<String, u8>> = std::sync::OnceLock::new();
    let inverse = INVERSE.get_or_init(|| {
        cp1251_map()
            .iter()
            .enumerate()
            .map(|(i, c)| (c.clone(), u8::try_from(i).unwrap()))
            .collect()
    });
    s.chars().map(|c| inverse[&c.to_string()]).collect()
}

fn str_col(batches: &[RecordBatch], name: &str) -> Vec<Option<String>> {
    let mut out = Vec::new();
    for b in batches {
        let i = b.schema().index_of(name).unwrap();
        let a = b.column(i);
        for r in 0..b.num_rows() {
            out.push((!a.is_null(r)).then(|| cell(a.as_ref(), r)));
        }
    }
    out
}

fn patched(rel: &str, name: &str, patch: impl FnOnce(&mut Vec<u8>)) -> PathBuf {
    let mut data = std::fs::read(fixtures().join(rel)).unwrap();
    patch(&mut data);
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/t9-tests");
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join(name);
    std::fs::write(&p, data).unwrap();
    p
}

fn find_one(data: &[u8], pattern: &[u8]) -> usize {
    let hits: Vec<usize> = data
        .windows(pattern.len())
        .enumerate()
        .filter(|(_, w)| *w == pattern)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(hits.len(), 1, "pattern {pattern:02x?}: {} hits", hits.len());
    hits[0]
}

fn no_longer_than_declared(w: &[oradmp_core::Warning]) {
    assert!(
        w.iter().all(|w| w.code != WarningCode::LongerThanDeclared),
        "{w:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// T_TEXT: V, C, NV equal the truth for every client set (3 clients x 2 paths x 2 versions)
// ---------------------------------------------------------------------------------------------

#[test]
fn t_text_equals_truth_for_every_client_set_and_path() {
    let mut files = 0;
    for (folder, v) in [("18c", "18c"), ("11g", "11g")] {
        let rows = truth(folder, "truth_text.txt");
        assert_eq!(rows.len(), 10);
        for stem in [
            "t_text_exp",
            "t_text_exp_direct",
            "t_text_exp_w1252",
            "t_text_exp_direct_w1252",
            "t_text_exp_us7ascii",
            "t_text_exp_direct_us7ascii",
        ] {
            let rel = format!("{folder}/{stem}_{v}.dmp");
            let (b, w) = read_default(&rel, "T_TEXT");
            assert!(w.is_empty(), "{rel}: {w:?}");
            no_longer_than_declared(&w);
            let (id, vv, cc, nv) = (
                str_col(&b, "ID"),
                str_col(&b, "V"),
                str_col(&b, "C"),
                str_col(&b, "NV"),
            );
            assert_eq!(vv.len(), 10, "{rel}");
            for (i, t) in rows.iter().enumerate() {
                assert_eq!(id[i].as_deref(), Some(t[0].as_str()), "{rel}");
                // truth: id | v_chars | v_bytes | DUMP(v) | DUMP(c) | DUMP(nv)
                assert_eq!(
                    vv[i],
                    truth_text(&t[3], Truth::Utf8),
                    "{rel} V row {}",
                    t[0]
                );
                assert_eq!(
                    cc[i],
                    truth_text(&t[4], Truth::Utf8),
                    "{rel} C row {}",
                    t[0]
                );
                assert_eq!(
                    nv[i],
                    truth_text(&t[5], Truth::Utf16),
                    "{rel} NV row {}",
                    t[0]
                );
                // v_chars and v_bytes of the truth
                match &vv[i] {
                    Some(s) => {
                        assert_eq!(
                            t[1],
                            format!("v_chars={}", s.chars().count()),
                            "{rel} row {}",
                            t[0]
                        );
                        assert_eq!(t[2], format!("v_bytes={}", s.len()), "{rel} row {}", t[0]);
                    }
                    None => assert_eq!(t[1], "v_chars=<NULL>"),
                }
            }
            files += 1;
        }
    }
    assert_eq!(files, 12);
}

// ---------------------------------------------------------------------------------------------
// R_TEXT: all columns, 309 rows, four client sets x two paths (database CL8MSWIN1251)
// ---------------------------------------------------------------------------------------------

#[test]
fn r_text_equals_truth_for_every_client_set_and_path() {
    let rows: Vec<Vec<String>> = truth("11g-cp1251", "truth_ru.txt")
        .into_iter()
        .filter(|l| l[0] == "R_TEXT")
        .collect();
    assert_eq!(rows.len(), 309);
    let mut files = 0;
    for stem in [
        "ru_tbl_exp_al32utf8",
        "ru_tbl_exp_cp1251",
        "ru_tbl_exp_us7ascii",
        "ru_tbl_exp_w1252",
        "ru_tbl_exp_direct_al32utf8",
        "ru_tbl_exp_direct_cp1251",
        "ru_tbl_exp_direct_us7ascii",
        "ru_tbl_exp_direct_w1252",
    ] {
        let rel = format!("11g-cp1251/{stem}_11g.dmp");
        let (b, w) = read_default(&rel, "R_TEXT");
        assert!(w.is_empty(), "{rel}: {w:?}");
        let names: Vec<_> = b[0]
            .schema()
            .fields()
            .iter()
            .map(|f| f.name().clone())
            .collect();
        assert_eq!(names, ["ID", "V", "C", "VC", "N", "LV", "D"], "{rel}");
        let (id, v, c, vc, n, lv, d) = (
            str_col(&b, "ID"),
            str_col(&b, "V"),
            str_col(&b, "C"),
            str_col(&b, "VC"),
            str_col(&b, "N"),
            str_col(&b, "LV"),
            str_col(&b, "D"),
        );
        assert_eq!(id.len(), 309, "{rel}");
        // the table is read in storage order; the truth is ordered by ID
        let by_id: HashMap<&str, &Vec<String>> = rows.iter().map(|t| (t[1].as_str(), t)).collect();
        let mut seen = std::collections::HashSet::new();
        for i in 0..id.len() {
            let t = by_id[id[i].as_deref().unwrap()];
            assert!(seen.insert(t[1].clone()), "{rel}: ID {} twice", t[1]);
            // R_TEXT | ID | DUMP(v) | DUMP(c) | DUMP(vc) | DUMP(n) | LENGTHB(lv) | ORA_HASH(lv) | DUMP(lv,16,1,60) | D
            let ctx = format!("{rel} row {}", t[1]);
            assert_eq!(v[i], truth_text(&t[2], Truth::Cp1251), "{ctx} V");
            assert_eq!(c[i], truth_text(&t[3], Truth::Cp1251), "{ctx} C");
            assert_eq!(vc[i], truth_text(&t[4], Truth::Cp1251), "{ctx} VC");
            assert_eq!(n[i], truth_text(&t[5], Truth::Utf16), "{ctx} N");
            // LV: the truth has LENGTHB and the first 60 bytes (the hash is Oracle's, not recomputed)
            match &lv[i] {
                None => {
                    assert_eq!(t[6], "NULL", "{ctx}");
                    assert_eq!(t[8], "NULL", "{ctx}");
                }
                Some(s) => {
                    let bytes = encode_cp1251(s);
                    assert_eq!(t[6], bytes.len().to_string(), "{ctx} LENGTHB(LV)");
                    let head = dump_bytes(&t[8]).unwrap();
                    assert_eq!(&bytes[..head.len()], &head[..], "{ctx} LV head");
                }
            }
            // V, C, VC re-encoded reproduce the database bytes (the manifest's check)
            for (col, field) in [(&v[i], &t[2]), (&c[i], &t[3]), (&vc[i], &t[4])] {
                match col {
                    Some(s) => assert_eq!(Some(encode_cp1251(s)), dump_bytes(field), "{ctx}"),
                    None => assert_eq!(field, "NULL", "{ctx}"),
                }
            }
            let day = if t[9] == "NULL" {
                None
            } else {
                Some(format!("{} 00:00:00", t[9]))
            };
            assert_eq!(d[i], day, "{ctx} D");
        }
        files += 1;
    }
    assert_eq!(files, 8);
}

// ---------------------------------------------------------------------------------------------
// T_NCHAR, T_NCHAR2, T_PROBE11 and trim_char
// ---------------------------------------------------------------------------------------------

#[test]
fn t_nchar_equals_truth_with_padding_and_trims_with_trim_char() {
    for (folder, v) in [("18c", "18c"), ("11g", "11g")] {
        let rel = format!("{folder}/nchar_exp_{v}.dmp");
        let rows = truth(folder, "truth_nchar.txt");
        assert_eq!(rows.len(), 3);
        // truth: id | DUMP(nc) | DUMP(cc) | DUMP(nv)
        let (b, w) = read_default(&rel, "T_NCHAR");
        assert!(w.is_empty(), "{rel}: {w:?}");
        no_longer_than_declared(&w);
        for (i, t) in rows.iter().enumerate() {
            assert_eq!(
                str_col(&b, "NC")[i],
                truth_text(&t[1], Truth::Utf16),
                "{rel}"
            );
            assert_eq!(
                str_col(&b, "CC")[i],
                truth_text(&t[2], Truth::Utf8),
                "{rel}"
            );
            assert_eq!(
                str_col(&b, "NV")[i],
                truth_text(&t[3], Truth::Utf16),
                "{rel}"
            );
        }
        // padding is kept: NC `ab` + 3 spaces (NCHAR(5)), CC `ab` + 2 spaces (CHAR(4 CHAR))
        assert_eq!(str_col(&b, "NC")[0].as_deref(), Some("ab   "));
        assert_eq!(str_col(&b, "CC")[0].as_deref(), Some("ab  "));
        // trim_char: `ab` and U+0434 U+0436 for NC and CC; NV (NVARCHAR2) is never padded or trimmed
        let o = ReadOptions {
            trim_char: true,
            ..ReadOptions::default()
        };
        let (b, w) = read(&mut open_fixture(&rel), "T_NCHAR", &o);
        assert!(w.is_empty(), "{w:?}");
        no_longer_than_declared(&w);
        assert_eq!(str_col(&b, "NC")[0].as_deref(), Some("ab"));
        assert_eq!(str_col(&b, "CC")[0].as_deref(), Some("ab"));
        assert_eq!(str_col(&b, "NC")[1].as_deref(), Some("\u{434}\u{436}"));
        assert_eq!(str_col(&b, "CC")[1].as_deref(), Some("\u{434}\u{436}"));
        assert_eq!(str_col(&b, "NC")[2], None);
        for (i, t) in rows.iter().enumerate() {
            assert_eq!(
                str_col(&b, "NV")[i],
                truth_text(&t[3], Truth::Utf16),
                "{rel}"
            );
        }
    }
}

#[test]
fn trim_char_trims_char_only_never_varchar2() {
    // T_TEXT row 7: V `trailing spaces   ` (VARCHAR2) and NV (NVARCHAR2) keep their spaces, C `ab` + 8 spaces
    // is trimmed
    let o = ReadOptions {
        trim_char: true,
        ..ReadOptions::default()
    };
    let (b, _) = read(&mut open_fixture("18c/t_text_exp_18c.dmp"), "T_TEXT", &o);
    assert_eq!(str_col(&b, "V")[6].as_deref(), Some("trailing spaces   "));
    assert_eq!(str_col(&b, "NV")[6].as_deref(), Some("trailing spaces   "));
    assert_eq!(str_col(&b, "C")[6].as_deref(), Some("ab"));
    assert_eq!(str_col(&b, "C")[7].as_deref(), Some("abcdefghij"));
    // the default keeps the padding
    let (b, _) = read_default("18c/t_text_exp_18c.dmp", "T_TEXT");
    assert_eq!(str_col(&b, "C")[6].as_deref(), Some("ab        "));
}

#[test]
fn t_nchar2_surrogate_pairs_and_character_semantics() {
    for (folder, v) in [("18c", "18c"), ("11g", "11g")] {
        let rel = format!("{folder}/nchar2_exp_{v}.dmp");
        let t = &truth(folder, "truth_nchar2.txt")[0];
        // truth: id | DUMP(nc) | DUMP(nv) | DUMP(cc) | DUMP(vc) | ... (the LOB columns are T-6L's)
        let (b, w) = read_default(&rel, "T_NCHAR2");
        no_longer_than_declared(&w);
        assert_eq!(
            str_col(&b, "NC")[0],
            truth_text(&t[1], Truth::Utf16),
            "{rel}"
        );
        assert_eq!(
            str_col(&b, "NV")[0],
            truth_text(&t[2], Truth::Utf16),
            "{rel}"
        );
        assert_eq!(
            str_col(&b, "CC")[0],
            truth_text(&t[3], Truth::Utf8),
            "{rel}"
        );
        assert_eq!(
            str_col(&b, "VC")[0],
            truth_text(&t[4], Truth::Utf8),
            "{rel}"
        );
        // NC and NV: a + U+1F600, NCHAR(3) holds 3 code units and is not padded; CC: `a` + U+1F600 + space
        assert_eq!(str_col(&b, "NC")[0].as_deref(), Some("a\u{1f600}"));
        assert_eq!(str_col(&b, "NV")[0].as_deref(), Some("a\u{1f600}"));
        assert_eq!(str_col(&b, "CC")[0].as_deref(), Some("a\u{1f600} "));
        let o = ReadOptions {
            trim_char: true,
            ..ReadOptions::default()
        };
        let (b, _) = read(&mut open_fixture(&rel), "T_NCHAR2", &o);
        assert_eq!(str_col(&b, "CC")[0].as_deref(), Some("a\u{1f600}"));
        assert_eq!(str_col(&b, "NC")[0].as_deref(), Some("a\u{1f600}"));
    }
}

#[test]
fn t_probe11_equals_truth_on_both_paths_without_length_warnings() {
    for (folder, v) in [("18c", "18c"), ("11g", "11g")] {
        let rows: Vec<Vec<String>> = truth(folder, "truth_probe11.txt")
            .into_iter()
            .filter(|l| l[0] == "ROW")
            .collect();
        assert_eq!(rows.len(), 2);
        for stem in ["t_probe11_exp", "t_probe11_exp_direct"] {
            let rel = format!("{folder}/{stem}_{v}.dmp");
            let (b, w) = read_default(&rel, "T_PROBE11");
            assert!(w.is_empty(), "{rel}: {w:?}");
            no_longer_than_declared(&w);
            // truth: ROW | id | DUMP(b) | DUMP(c) | DUMP(k) | DUMP(w) | DUMP(x) | LENGTHB(y) | LENGTHB(z)
            for (i, t) in rows.iter().enumerate() {
                for (col, f) in [("B", 2), ("C", 3), ("K", 4), ("W", 5), ("X", 6)] {
                    assert_eq!(
                        str_col(&b, col)[i],
                        truth_text(&t[f], Truth::Utf8),
                        "{rel} {col} row {}",
                        t[1]
                    );
                }
                let y = str_col(&b, "Y")[i].clone().unwrap();
                let z = str_col(&b, "Z")[i].clone().unwrap();
                assert_eq!(y.len().to_string(), t[7], "{rel} Y");
                assert_eq!(z.len().to_string(), t[8], "{rel} Z");
            }
            // the lengths stated by the plan: 500 / 506 and 501 / 507 bytes
            assert_eq!(str_col(&b, "Y")[0].as_ref().unwrap().len(), 500);
            assert_eq!(str_col(&b, "Y")[1].as_ref().unwrap().len(), 506);
            assert_eq!(str_col(&b, "Z")[0].as_ref().unwrap().len(), 501);
            assert_eq!(str_col(&b, "Z")[1].as_ref().unwrap().len(), 507);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// The map test: 768 lines of truth_csmap.txt
// ---------------------------------------------------------------------------------------------

#[test]
fn map_test_768_lines_of_truth_csmap_in_both_folders() {
    let a = common::truth_text("18c", "truth_csmap.txt").into_bytes();
    let b = common::truth_text("11g", "truth_csmap.txt").into_bytes();
    assert_eq!(a, b, "the two folders are identical");
    let mut counts = [0usize; 3];
    for folder in ["18c", "11g"] {
        for l in truth(folder, "truth_csmap.txt") {
            let byte = u8::from_str_radix(&l[1], 16).unwrap();
            let oracle = utf16_be(&dump_bytes(&l[2]).unwrap());
            let set = Charset::from_name(&l[0]).unwrap();
            let one = [byte];
            let decoded = set.decode(&one);
            let (name_text, name_flag) = decode_text(Some(set), &one);
            match l[0].as_str() {
                "CL8MSWIN1251" | "WE8MSWIN1252" => {
                    // encoding_rs equals the truth code unit, `had_errors` false in all 512 lines
                    let enc = if l[0] == "CL8MSWIN1251" {
                        encoding_rs::WINDOWS_1251
                    } else {
                        encoding_rs::WINDOWS_1252
                    };
                    let (text, had_errors) = enc.decode_without_bom_handling(&one);
                    assert!(!had_errors, "{} {byte:02x}", l[0]);
                    assert_eq!(text, oracle, "{} {byte:02x}", l[0]);
                    assert_eq!(decoded.unwrap(), oracle, "{} {byte:02x}", l[0]);
                    assert_eq!((name_text.as_str(), name_flag), (oracle.as_str(), false));
                    counts[usize::from(l[0] == "WE8MSWIN1252")] += 1;
                }
                "US7ASCII" => {
                    if byte < 0x80 {
                        assert_eq!(decoded.unwrap(), oracle, "{byte:02x}");
                        assert_eq!(name_text, oracle);
                        assert!(!name_flag);
                    } else {
                        // row data: a value error (ADR-16); a name: U+FFFD with the flag, which is what
                        // Oracle's map gives
                        assert_eq!(decoded.unwrap_err().code, ErrorCode::BadText);
                        assert_eq!(oracle, "\u{fffd}");
                        assert_eq!(name_text, "\u{fffd}");
                        assert!(name_flag);
                    }
                    counts[2] += 1;
                }
                other => panic!("{other}"),
            }
        }
    }
    // 2 folders x 256 bytes x 3 sets
    assert_eq!(counts, [512, 512, 512]);
}

// ---------------------------------------------------------------------------------------------
// Unit behaviour of the decoders and the override names
// ---------------------------------------------------------------------------------------------

#[test]
fn override_names_are_oracle_names_or_encoding_rs_labels() {
    assert_eq!(Charset::from_name("cl8mswin1251"), Some(Charset::Win1251));
    assert_eq!(Charset::from_name("WE8MSWIN1252"), Some(Charset::Win1252));
    assert_eq!(Charset::from_name("windows-1251"), Some(Charset::Win1251));
    assert_eq!(Charset::from_name("utf-8"), Some(Charset::Utf8));
    assert_eq!(Charset::from_name("AL32UTF8"), Some(Charset::Utf8));
    assert_eq!(Charset::from_name("AL16UTF16"), Some(Charset::Utf16Be));
    // the WHATWG label "us-ascii" is windows-1252; here it means ASCII
    assert_eq!(Charset::from_name("us-ascii"), Some(Charset::Us7Ascii));
    let sj = Charset::from_name("shift_jis").unwrap();
    assert!(matches!(sj, Charset::Other(_)));
    assert_eq!(sj.name(), "Shift_JIS");
    assert_eq!(sj.decode(&[0x93, 0xfa]).unwrap(), "\u{65e5}");
    assert!(sj.decode(&[0x93]).is_err());
    assert_eq!(Charset::from_name("no-such-set"), None);
    assert_eq!(Charset::from_id(2000), Some(Charset::Utf16Be));
    assert_eq!(Charset::from_id(31), None);
}

#[test]
fn metadata_text_never_stops_and_flags_what_it_replaced() {
    // UTF-8 with a stray byte: U+FFFD and the flag; valid text without
    assert_eq!(
        decode_text(Some(Charset::Utf8), b"ab\xffc"),
        ("ab\u{fffd}c".to_string(), true)
    );
    assert_eq!(
        decode_text(Some(Charset::Utf8), "\u{422}\u{410}".as_bytes()),
        ("\u{422}\u{410}".to_string(), false)
    );
    // a client set the reader cannot decode (None): every byte above 0x7f is U+FFFD
    assert_eq!(
        decode_text(None, b"A\xd2B"),
        ("A\u{fffd}B".to_string(), true)
    );
    assert!(has_replacement_char(Some(Charset::Us7Ascii), b"???_???"));
    assert!(has_replacement_char(Some(Charset::Win1252), b"\xbf\xbf_1"));
    assert!(!has_replacement_char(Some(Charset::Win1251), b"\xbf_1"));
    assert!(!has_replacement_char(Some(Charset::Utf8), b"TABLE_1"));
}

// ---------------------------------------------------------------------------------------------
// Names and DDL: client set, flags, possibly_incomplete
// ---------------------------------------------------------------------------------------------

#[test]
fn names_are_decoded_with_the_client_set_and_cut_short_tables_are_flagged() {
    let name = "\u{422}\u{410}\u{411}_\u{420}\u{423}\u{421}"; // truth: d2,c0,c1,5f,d0,d3,d1 in CL8MSWIN1251
    let mut seen = 0;
    for (stem, ok) in [
        ("ru_exp_cp1251", true),
        ("ru_exp_al32utf8", true),
        ("ru_exp_us7ascii", false),
        ("ru_exp_w1252", false),
        ("ru_exp_direct_cp1251", true),
        ("ru_exp_direct_us7ascii", false),
    ] {
        let rel = format!("11g-cp1251/{stem}_11g.dmp");
        let mut d = open_fixture(&rel);
        let info = d.info().clone();
        assert_eq!(
            info.text_converted_at_export,
            info.client_charset_id != info.db_charset_id,
            "{rel}"
        );
        let tables = d.tables().unwrap().to_vec();
        let t = tables
            .iter()
            .find(|t| t.name_raw.iter().any(|b| *b >= 0x80 || *b == b'?'))
            .unwrap_or_else(|| panic!("{rel}: no table with a non-ASCII or replaced name"));
        if ok {
            // the letters survived: the client set decodes them, nothing is flagged, 2 rows
            assert_eq!(t.name, name, "{rel}");
            assert!(!t.name_possibly_damaged, "{rel}");
            assert!(!t.ddl_lossy, "{rel}");
            assert_eq!(t.num_rows, 2, "{rel}");
            assert!(!t.possibly_incomplete, "{rel}");
            assert!(t.columns.iter().all(|c| !c.name_possibly_damaged), "{rel}");
        } else {
            // `???_???` (US7ASCII) or `¿¿¿_¿¿¿` (WE8MSWIN1252): the table was cut short [X2, exp.md 14.3]
            let want = if stem.contains("us7ascii") {
                "???_???"
            } else {
                "\u{bf}\u{bf}\u{bf}_\u{bf}\u{bf}\u{bf}"
            };
            assert_eq!(t.name, want, "{rel}");
            assert!(t.name_possibly_damaged, "{rel}");
            assert!(t.ddl_lossy, "{rel}");
            assert_eq!(t.num_rows, 0, "{rel}");
            assert!(t.possibly_incomplete, "{rel}");
            assert!(t.columns.iter().all(|c| c.name_possibly_damaged), "{rel}");
            let ws: Vec<_> = d
                .warnings()
                .iter()
                .filter(|w| w.code == WarningCode::PossiblyIncomplete)
                .collect();
            assert_eq!(ws.len(), 1, "{rel}");
            assert_eq!(ws[0].table, Some(t.ordinal));
            assert_eq!(ws[0].offset, Some(t.create_offset));
            // the other tables of the file (R_TEXT with 309 rows, R_CLOB) are not flagged
            assert!(tables
                .iter()
                .filter(|o| o.ordinal != t.ordinal)
                .all(|o| !o.possibly_incomplete && !o.name_possibly_damaged));
            assert_eq!(w_string(&ws[0].code), "W-POSSIBLY-INCOMPLETE");
        }
        // the raw bytes are always kept
        assert!(!t.name_raw.is_empty());
        seen += 1;
    }
    assert_eq!(seen, 6);
}

fn w_string(c: &WarningCode) -> String {
    c.to_string()
}

#[test]
fn own_exp_w1252_is_converted_but_its_ddl_has_no_replacement_character() {
    // 18c and 11g: client WE8MSWIN1252, database AL32UTF8. The CREATE TABLE lines of O_CHILD, O_EMPTY and
    // O_PARENT hold neither `?` nor 0xbf (the comment with the replaced `д` is a COMMENT item, not DDL), so
    // `ddl_lossy` is false (architecture 6.5); the plan's "own_exp_w1252: ddl_lossy" is N-21.
    for (folder, v) in [("18c", "18c"), ("11g", "11g")] {
        let mut d = open_fixture(&format!("{folder}/own_exp_w1252_{v}.dmp"));
        assert!(d.info().text_converted_at_export);
        for t in d.tables().unwrap() {
            assert!(!t.ddl_lossy, "{}", t.name);
            assert!(!t.name_possibly_damaged);
            assert!(t.ddl.starts_with("CREATE TABLE "));
        }
        let d = open_fixture(&format!("{folder}/own_exp_{v}.dmp"));
        assert!(!d.info().text_converted_at_export);
    }
    // the flag is set by a `?` in a converted text and by nothing in an unconverted one: the TABLESPACE name
    // `USERS` becomes `USER?` (same length) in a copy of each file
    for (rel, name, converted) in [
        ("18c/own_exp_w1252_18c.dmp", "own_w1252_q.dmp", true),
        ("18c/own_exp_18c.dmp", "own_q.dmp", false),
    ] {
        // each file holds the text three times: patch all of them
        let p = patched(rel, name, |d| {
            let pat = b"TABLESPACE \"USERS\"";
            let mut n = 0;
            let mut i = 0;
            while i + pat.len() <= d.len() {
                if &d[i..i + pat.len()] == pat {
                    d[i + 16] = b'?';
                    n += 1;
                    i += pat.len();
                } else {
                    i += 1;
                }
            }
            assert!(n >= 3, "{n}");
        });
        let mut d = open(&p, Options::default()).unwrap();
        for t in d.tables().unwrap() {
            if t.ddl.contains("USER?") {
                assert_eq!(t.ddl_lossy, converted, "{name} {}", t.name);
            }
        }
        assert!(d.tables().unwrap().iter().any(|t| t.ddl.contains("USER?")));
    }
}

#[test]
fn client_charset_option_replaces_the_header_and_is_recorded() {
    // ru_exp_al32utf8 holds UTF-8 names; read as CL8MSWIN1251 they are other letters, read as UTF-8 (the
    // header's own set) they are the real name
    let rel = fixtures().join("11g-cp1251/ru_exp_al32utf8_11g.dmp");
    let mut d = open(
        &rel,
        Options {
            client_charset: Some("cp1251".to_string()),
            ..Options::default()
        },
    )
    .unwrap();
    assert_eq!(d.info().client_charset_override.as_deref(), Some("cp1251"));
    assert_eq!(d.info().db_charset_override, None);
    let t = d
        .tables()
        .unwrap()
        .iter()
        .find(|t| t.name_raw.iter().any(|b| *b >= 0x80))
        .unwrap()
        .clone();
    assert_ne!(t.name, "\u{422}\u{410}\u{411}_\u{420}\u{423}\u{421}");
    assert_eq!(t.name_raw.len(), 13);
    // an unknown name is E-CONFIG at open
    for o in [
        Options {
            client_charset: Some("no-such".to_string()),
            ..Options::default()
        },
        Options {
            db_charset: Some("no-such".to_string()),
            ..Options::default()
        },
    ] {
        assert_eq!(open(&rel, o).unwrap_err().code, ErrorCode::Config);
    }
}

#[test]
fn partition_names_are_decoded_with_the_client_set() {
    // part2 / part: ASCII partition names; flags false, names equal their raw bytes
    let mut d = open_fixture("18c/part2_exp_18c.dmp");
    let mut named = 0;
    for t in d.tables().unwrap() {
        for p in &t.parts {
            if let (Some(n), Some(r)) = (&p.partition, &p.partition_raw) {
                assert_eq!(n.as_bytes(), &r[..]);
                assert!(!p.partition_possibly_damaged);
                named += 1;
            }
            if let (Some(n), Some(r)) = (&p.subpartition, &p.subpartition_raw) {
                assert_eq!(n.as_bytes(), &r[..]);
                assert!(!p.subpartition_possibly_damaged);
            }
        }
    }
    assert_eq!(named, 6, "T_PART2 has 6 parts");
    // a copy where the partition name `PMAX` of T_PART becomes `PM\xe9X` (same length, in the CREATE TABLE text
    // and the PARTITION line): with the header's client set (AL32UTF8) the byte is malformed, with us-ascii it is
    // above 0x7f: U+FFFD and the flag, for that partition only; read as windows-1252 it is `PMéX`, not flagged
    let p = patched("18c/part_exp_18c.dmp", "part_name.dmp", |d| {
        let pat = b"\"PMAX\"";
        let mut n = 0;
        for i in 0..d.len() - pat.len() {
            if &d[i..i + pat.len()] == pat {
                d[i + 3] = 0xe9;
                n += 1;
            }
        }
        assert_eq!(n, 2);
    });
    for (client, name, damaged) in [
        (None, "PM\u{fffd}X", true),
        (Some("us-ascii"), "PM\u{fffd}X", true),
        (Some("windows-1252"), "PM\u{e9}X", false),
    ] {
        let mut d = open(
            &p,
            Options {
                client_charset: client.map(str::to_string),
                ..Options::default()
            },
        )
        .unwrap();
        let t = d.tables().unwrap();
        let parts = &t.iter().find(|t| t.name == "T_PART").unwrap().parts;
        assert_eq!(parts.len(), 3);
        for p in parts {
            let is_pmax = p.partition_raw.as_deref() == Some(&b"PM\xe9X"[..]);
            assert_eq!(
                p.partition_possibly_damaged,
                is_pmax && damaged,
                "{client:?}"
            );
            if is_pmax {
                assert_eq!(p.partition.as_deref(), Some(name), "{client:?}");
            }
        }
        assert_eq!(
            parts
                .iter()
                .filter(|p| p.partition_raw.as_deref() == Some(&b"PM\xe9X"[..]))
                .count(),
            1
        );
    }
}

// ---------------------------------------------------------------------------------------------
// Unsupported set: Binary column and W-CHARSET-UNSUPPORTED; overrides; US7ASCII; malformed bytes
// ---------------------------------------------------------------------------------------------

/// `T_TEXT.V`: `01 00 c8 00 69 03 01 00` (type 1, length 200, set 873, form 1); `T_TEXT.C`:
/// `60 00 0a 00 69 03 01 00`. Both appear once in the header of the file (the descriptor layout is rule T5).
const V_DESC: [u8; 8] = [0x01, 0x00, 0xc8, 0x00, 0x69, 0x03, 0x01, 0x00];
const C_DESC: [u8; 8] = [0x60, 0x00, 0x0a, 0x00, 0x69, 0x03, 0x01, 0x00];

fn patch_ids(d: &mut [u8], v_id: u16, c_id: u16) {
    let at = find_one(d, &V_DESC);
    d[at + 4..at + 6].copy_from_slice(&v_id.to_le_bytes());
    let at = find_one(d, &C_DESC);
    d[at + 4..at + 6].copy_from_slice(&c_id.to_le_bytes());
}

#[test]
fn an_unsupported_descriptor_set_gives_a_binary_column_and_a_warning() {
    for rel in ["11g/t_text_exp_11g.dmp", "11g/t_text_exp_direct_11g.dmp"] {
        // V gets set id 832 (not one of the five), C stays AL32UTF8
        let p = patched(rel, "unsupported_set.dmp", |d| patch_ids(d, 832, 873));
        let mut d = open(&p, Options::default()).unwrap();
        let t = d.tables().unwrap()[0].clone();
        assert_eq!(t.columns[1].arrow_type, DataType::Binary);
        assert_eq!(t.columns[2].arrow_type, DataType::Utf8);
        let ws: Vec<_> = d
            .warnings()
            .iter()
            .filter(|w| w.code == WarningCode::CharsetUnsupported)
            .collect();
        assert_eq!(ws.len(), 1, "{:?}", d.warnings());
        assert_eq!(ws[0].table, Some(t.ordinal));
        assert_eq!(ws[0].column, Some(2));
        assert_eq!(ws[0].length, Some(832));
        assert!(ws[0].offset.is_some());
        // the bytes are delivered as read: V of row 2 is the UTF-8 text of truth row 2
        let (b, w) = read(&mut d, "T_TEXT", &ReadOptions::default());
        assert!(w.iter().all(|w| w.code != WarningCode::CharsetUnsupported));
        assert_eq!(b[0].schema().field(1).data_type(), &DataType::Binary);
        let truth_v = dump_bytes(&truth("11g", "truth_text.txt")[1][3]).unwrap();
        assert_eq!(str_col(&b, "V")[1].as_deref(), Some(hex(&truth_v).as_str()));
        // C is still text
        assert_eq!(str_col(&b, "C")[0].as_deref(), Some("abc       "));
        // with db_charset the column is text again: no warning, decoded with the override
        let mut d = open(
            &p,
            Options {
                db_charset: Some("CL8MSWIN1251".to_string()),
                ..Options::default()
            },
        )
        .unwrap();
        d.tables().unwrap();
        assert!(d
            .warnings()
            .iter()
            .all(|w| w.code != WarningCode::CharsetUnsupported));
        assert_eq!(
            d.info().db_charset_override.as_deref(),
            Some("CL8MSWIN1251")
        );
        let (b, _) = read(&mut d, "T_TEXT", &ReadOptions::default());
        assert_eq!(b[0].schema().field(1).data_type(), &DataType::Utf8);
        assert_eq!(str_col(&b, "V")[0].as_deref(), Some("plain ascii"));
        // the override replaced the descriptor for C as well: row 2 `да` (UTF-8 d0 b4 d0 b0) as cp1251
        let want: String = truth_v_cp1251(&[0xd0, 0xb4, 0xd0, 0xb0]);
        assert_eq!(
            str_col(&b, "C")[1].as_deref().map(str::trim_end),
            Some(want.trim_end())
        );
    }
}

fn truth_v_cp1251(b: &[u8]) -> String {
    let map = cp1251_map();
    b.iter().map(|x| map[usize::from(*x)].clone()).collect()
}

/// The header's three set ids (client, database, national): `03 69 03 69 07 d0` in `t_text_exp_11g.dmp`.
const HEADER_IDS: [u8; 6] = [0x03, 0x69, 0x03, 0x69, 0x07, 0xd0];

/// A database of set US7ASCII (header database id 1, descriptor of V and C id 1) with the high byte 0xe4
/// patched into the V value `plain ascii` of row 1 (the fixtures have no US7ASCII database).
fn us7ascii_copy(name: &str) -> PathBuf {
    patched("11g/t_text_exp_11g.dmp", name, |d| {
        let at = find_one(d, &HEADER_IDS);
        d[at + 2..at + 4].copy_from_slice(&[0x00, 0x01]);
        patch_ids(d, 1, 1);
        let at = find_one(d, b"plain ascii");
        d[at + 2] = 0xe4;
    })
}

#[test]
fn us7ascii_high_byte_is_a_value_error_and_db_charset_reads_it() {
    let p = us7ascii_copy("us7ascii_db.dmp");
    let mut d = open(&p, Options::default()).unwrap();
    assert_eq!(d.info().db_charset_id, 1);
    // raise (default): E-BAD-TEXT with table, part, row, column and offset; the text names db_charset
    let e = try_read(&mut d, "T_TEXT", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadText);
    assert_eq!((e.table, e.row, e.column), (Some(1), Some(1), Some(2)));
    assert!(e.offset.is_some());
    assert!(e.to_string().contains("db_charset"), "{e}");
    assert!(!e.to_string().contains('\u{e4}'));
    // null: row 1 V is NULL, the rest of the table is read, one counted W-VALUE-NULLED
    let o = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let (b, w) = read(&mut d, "T_TEXT", &o);
    let v = str_col(&b, "V");
    assert_eq!(v[0], None);
    // rows 2 to 5 hold Cyrillic, CJK or accented text (UTF-8 bytes above 0x7f): they are value errors too
    assert!(v[1..5].iter().all(Option::is_none));
    assert_eq!(v[6].as_deref(), Some("trailing spaces   "));
    // N-4 (review 2): one counted warning per column, the counts from `truth_text.txt` (a value counts if its
    // DUMP has a byte above 0x7f in the US7ASCII copy): V is row 1 (patched to 0xe4) and the rows of the truth
    // with such a byte; C (also US7ASCII in this copy) is the rows of the truth with such a byte.
    let tr = truth("11g", "truth_text.txt");
    let high_rows = |field: usize| -> Vec<u64> {
        tr.iter()
            .filter(|t| dump_bytes(&t[field]).is_some_and(|b| b.iter().any(|x| *x > 0x7f)))
            .map(|t| t[0].parse().unwrap())
            .collect()
    };
    let (high_v, high_c) = (high_rows(3), high_rows(4));
    assert_eq!(high_v, [2, 3, 4, 5]);
    assert_eq!(high_c, [2, 3, 4], "row 5 of C is the plain x");
    let nulled = |column: u32| -> Vec<(Option<u64>, Option<u64>)> {
        w.iter()
            .filter(|w| w.code == WarningCode::ValueNulled && w.column == Some(column))
            .map(|w| (w.count, w.row))
            .collect()
    };
    // V (column 2): the patched row 1 and rows 2 to 5, first at row 1
    assert_eq!(nulled(2), [(Some(1 + high_v.len() as u64), Some(1))]);
    // C (column 3): rows 2 to 4, first at row 2; nothing in the other columns
    assert_eq!(nulled(3), [(Some(high_c.len() as u64), Some(high_c[0]))]);
    assert_eq!(
        w.iter()
            .filter(|w| w.code == WarningCode::ValueNulled)
            .count(),
        2
    );
    // db_charset reads the bytes as CL8MSWIN1251: 0xe4 is U+0434 in Oracle's map
    let mut d = open(
        &p,
        Options {
            db_charset: Some("CL8MSWIN1251".to_string()),
            ..Options::default()
        },
    )
    .unwrap();
    let (b, w) = read(&mut d, "T_TEXT", &ReadOptions::default());
    assert_eq!(str_col(&b, "V")[0].as_deref(), Some("pl\u{434}in ascii"));
    assert!(w.is_empty(), "{w:?}");
}

#[test]
fn malformed_utf8_follows_on_value_error() {
    // the `p` of `plain ascii` becomes 0xff (never valid UTF-8)
    let p = patched("11g/t_text_exp_11g.dmp", "bad_utf8.dmp", |d| {
        let at = find_one(d, b"plain ascii");
        d[at] = 0xff;
    });
    let mut d = open(&p, Options::default()).unwrap();
    let e = try_read(&mut d, "T_TEXT", &ReadOptions::default()).unwrap_err();
    assert_eq!(e.code, ErrorCode::BadText);
    assert_eq!((e.table, e.row, e.column), (Some(1), Some(1), Some(2)));
    let o = ReadOptions {
        on_value_error: OnValueError::Null,
        ..ReadOptions::default()
    };
    let (b, w) = read(&mut d, "T_TEXT", &o);
    assert_eq!(str_col(&b, "V")[0], None);
    // NV of row 1 (UTF-16) is untouched
    assert_eq!(str_col(&b, "NV")[0].as_deref(), Some("plain ascii"));
    assert_eq!(
        w.iter()
            .filter(|w| w.code == WarningCode::ValueNulled)
            .count(),
        1
    );
    // the same patch never gives a U+FFFD: no cell of the table contains one
    for r in rows(&b) {
        assert!(r.iter().all(|c| !c.contains('\u{fffd}')));
    }
}

#[test]
fn a_descriptor_set_other_than_the_database_set_warns_unless_db_charset_is_given() {
    // C of T_TEXT gets set id 171 while the header says 873: the descriptor wins, W-CHARSET-MISMATCH (T-6); with
    // the db_charset override the descriptor is not consulted, so no warning
    let p = patched("11g/t_text_exp_11g.dmp", "mismatch.dmp", |d| {
        patch_ids(d, 873, 171)
    });
    let mut d = open(&p, Options::default()).unwrap();
    let (_, w) = read(&mut d, "T_TEXT", &ReadOptions::default());
    // N-4: exactly one warning, for column 3 (C, the only column of T_TEXT with a set id: 171 in this copy)
    let mism: Vec<_> = w
        .iter()
        .filter(|w| w.code == WarningCode::CharsetMismatch)
        .collect();
    assert_eq!(mism.len(), 1, "{mism:?}");
    assert_eq!(mism[0].column, Some(3));
    let mut d = open(
        &p,
        Options {
            db_charset: Some("AL32UTF8".to_string()),
            ..Options::default()
        },
    )
    .unwrap();
    let (_, w) = read(&mut d, "T_TEXT", &ReadOptions::default());
    assert!(w.iter().all(|w| w.code != WarningCode::CharsetMismatch));
}
