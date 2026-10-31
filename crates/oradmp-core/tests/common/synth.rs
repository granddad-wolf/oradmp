//! T-17 (ADR-80): a test-side generator of small synthetic legacy `exp` files, written from
//! `docs/formats/exp_reader_rules.md` (version 3.7) only.
//!
//! * It calls no function of `oradmp_core`: NUMBER ([Y1]), DATE ([Y2]), TIMESTAMP ([Y5]), UTF-16 BE and the
//!   single-byte set CL8MSWIN1251 ([C7], through `encoding_rs`) are encoded here, from the rules.
//! * Every construct it writes as valid input has a rule whose status is **confirmed** in the rules file; the rule
//!   id stands beside the code that writes it. The status table of the rules used is in
//!   `docs/design/implementer-notes.md` (T-17). Constructs of status suggested or unknown are never written as
//!   valid input: the only unconfirmed bytes are the filler of [H4] (10 bytes), the zero filler of the second
//!   record ([H6]) and the 2 bytes behind a LOB column entry ([L1]), all constant in the fixtures and skipped by
//!   the rules.
//! * The expected values are the `Spec`'s own (`expected_*`): the logical value that was written, never what the
//!   reader returned.
//!
//! The file layout, in file order: record 1 = the header, zero filled to R [H5]; record 2 starts with the 0x104
//! bytes of [H6]; the items follow from R + 0x104 as one byte stream [I1, R5]; the file ends with `EXIT` LF `EXIT`
//! LF and zero fill to a multiple of R [E4].
#![allow(dead_code)]

use std::collections::BTreeSet;

use arrow_schema::{DataType, TimeUnit};

// ---------------------------------------------------------------------------------------------------------
// A small deterministic random generator (splitmix64): the `Spec` of a seed is the same on every run.
// ---------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Rng(pub u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed ^ 0x9e37_79b9_7f4a_7c15)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    /// 0 to n - 1 (n > 0).
    pub fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
    /// lo to hi, both included.
    pub fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + self.below((hi - lo + 1) as u64) as usize
    }
    pub fn chance(&mut self, percent: u32) -> bool {
        self.below(100) < u64::from(percent)
    }
    pub fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        items[self.below(items.len() as u64) as usize]
    }
    pub fn bytes(&mut self, n: usize) -> Vec<u8> {
        (0..n).map(|_| self.next_u64() as u8).collect()
    }
}

// ---------------------------------------------------------------------------------------------------------
// The Spec
// ---------------------------------------------------------------------------------------------------------

/// The constructs the generator writes although no fixture shows them (ADR-80 item 1 as amended, ADR-86). A
/// construct the fixtures show is never labelled, and one they do not show is never left unlabelled. Every writing
/// site carries a line comment: the word UNCONFIRMED in capitals, the variant name in parentheses, a colon, the rule
/// ids, "accepted by" a rule, and "evidence:" with an R number or "none". The note counts the sites with `grep -c`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Unconfirmed {
    /// `ff ff` (E1) behind the path flag 1 of an empty table in an 18c **owner-mode** file. The fixtures show E1
    /// and E2 together in an 18c table-mode file (`seg_exp_direct_18c`, R-017: E1 for a table with a segment, E2
    /// for one without), so there the label no longer applies (ADR-94 item 2); owner mode has no such fixture and
    /// E1 there rests on E3. Evidence: none.
    E1AfterFlag1In18cOwnerMode,
    /// A conventional LONG or LONG RAW written in the chunked `fd ff` form with a total below 65,534 bytes. The
    /// fixtures show the form from 65,534 bytes only (`long4_exp_<v>` row 6; [R2] v3.9, ADR-99 item 2). Evidence
    /// route: none beyond the self-delimiting framing.
    LongChunkedBelow65534,
    /// A conventional LONG or LONG RAW of 65,522 to 65,524 bytes written as one field (the u16 0xfff2 to 0xfff4). The
    /// fixtures show 65,521 and 65,525 and no size between (R-048, `long4_exp_<v>`); the reader reads the bytes as one
    /// field and warns. Evidence route: R-052 (proposed, very low).
    LongOneFieldFff2ToFff4,
    /// A (client set, database set) pair with a version and mode for which no fixture has it. The client set never
    /// decodes row data [C4] and the generator's identifiers are ASCII, so only the header fields change.
    ClientSetNotInFixtures,
}

/// `EXPORT:V18.00.00` or `EXPORT:V11.02.00` [H3].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    V18,
    V11,
}

impl Version {
    pub fn line(self) -> &'static str {
        match self {
            Version::V18 => "EXPORT:V18.00.00",
            Version::V11 => "EXPORT:V11.02.00",
        }
    }
}

/// `RTABLES` / `RUSERS` [H3], first item `METRICST` / `METRICSU` [H6].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Tables,
    Users,
}

/// One header the generator can write: version [H3], mode [H3], client set [H1, H4], database set [H4]. The
/// unlabelled entries occur in the fixtures (test `header_sets_match_the_fixtures` reads all 229 EXP headers);
/// the entries labelled `ClientSetNotInFixtures` do not. No 18c entry has database set 171 (the rules file names a
/// single-byte database on 18c as not covered) and no entry pairs 18c with 171.
#[derive(Clone, Copy, Debug)]
pub struct HeaderChoice {
    pub version: Version,
    pub mode: Mode,
    pub client_set: u16,
    pub db_set: u16,
    pub label: Option<Unconfirmed>,
}

const fn hc(
    version: Version,
    mode: Mode,
    client_set: u16,
    db_set: u16,
    unconfirmed: bool,
) -> HeaderChoice {
    HeaderChoice {
        version,
        mode,
        client_set,
        db_set,
        label: if unconfirmed {
            Some(Unconfirmed::ClientSetNotInFixtures)
        } else {
            None
        },
    }
}

/// The one list `random_spec` draws its header from. Fixture counts (see `common::TOTALS`; the test's own reading of
/// H1 to H4): 11g RTABLES: clients 873/171/178/1 with db 171, and 873/178/1 with db 873; 11g RUSERS: 873/171/178/1
/// with db 171, and 873/178 with db 873; 18c RTABLES: 873/178/1 with db 873; 18c RUSERS: 873/178 with db 873.
pub const HEADER_CHOICES: [HeaderChoice; 24] = [
    // 11g, table mode
    hc(Version::V11, Mode::Tables, 873, 873, false),
    hc(Version::V11, Mode::Tables, 178, 873, false),
    hc(Version::V11, Mode::Tables, 1, 873, false),
    hc(Version::V11, Mode::Tables, 171, 873, true),
    hc(Version::V11, Mode::Tables, 873, 171, false),
    hc(Version::V11, Mode::Tables, 171, 171, false),
    hc(Version::V11, Mode::Tables, 178, 171, false),
    hc(Version::V11, Mode::Tables, 1, 171, false),
    // 11g, owner mode
    hc(Version::V11, Mode::Users, 873, 873, false),
    hc(Version::V11, Mode::Users, 178, 873, false),
    hc(Version::V11, Mode::Users, 1, 873, true),
    hc(Version::V11, Mode::Users, 171, 873, true),
    hc(Version::V11, Mode::Users, 873, 171, false),
    hc(Version::V11, Mode::Users, 171, 171, false),
    hc(Version::V11, Mode::Users, 178, 171, false),
    hc(Version::V11, Mode::Users, 1, 171, false),
    // 18c, table mode (database 873 only)
    hc(Version::V18, Mode::Tables, 873, 873, false),
    hc(Version::V18, Mode::Tables, 178, 873, false),
    hc(Version::V18, Mode::Tables, 1, 873, false),
    hc(Version::V18, Mode::Tables, 171, 873, true),
    // 18c, owner mode
    hc(Version::V18, Mode::Users, 873, 873, false),
    hc(Version::V18, Mode::Users, 178, 873, false),
    hc(Version::V18, Mode::Users, 1, 873, true),
    hc(Version::V18, Mode::Users, 171, 873, true),
];

/// The label of a header: `Some` when the list holds the pair with the label. A header outside the list is a
/// mistake of the test that built the `Spec`.
fn header_label(spec: &Spec) -> Option<Unconfirmed> {
    HEADER_CHOICES
        .iter()
        .find(|h| {
            h.version == spec.version
                && h.mode == spec.mode
                && h.client_set == spec.client_set
                && h.db_set == spec.db_set
        })
        .unwrap_or_else(|| {
            panic!(
                "header ({:?}, {:?}, client {}, database {}) is not in HEADER_CHOICES",
                spec.version, spec.mode, spec.client_set, spec.db_set
            )
        })
        .label
}

/// What stands in an empty table after the path flag 1: [E1] `ff ff` LF, or [E2] LF only (18c).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmptyForm {
    E1,
    E2,
}

/// The column types the generator writes (rules T5, T9; values Y1, Y2, Y5, Y6, C1, C3, C5, C6, Y9, L4, L5).
#[derive(Clone, Debug, PartialEq)]
pub enum ColType {
    /// `NUMBER` (no precision)
    NumberPlain,
    /// `NUMBER(p, 0)`
    NumberInt(u8),
    /// `NUMBER(p, s)`, 0 < s <= p
    NumberDec(u8, u8),
    Date,
    /// `TIMESTAMP (f)`
    Timestamp(u8),
    Raw(u16),
    Varchar2 {
        n: u16,
        chars: bool,
    },
    Char {
        n: u16,
        chars: bool,
    },
    NVarchar2(u16),
    NChar(u16),
    Long,
    LongRaw,
    Clob,
    Blob,
}

impl ColType {
    pub fn is_lob(&self) -> bool {
        matches!(self, ColType::Clob | ColType::Blob)
    }
    pub fn is_long(&self) -> bool {
        matches!(self, ColType::Long | ColType::LongRaw)
    }
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            ColType::NumberPlain | ColType::NumberInt(_) | ColType::NumberDec(..)
        )
    }
}

/// One cell: the value the reader must return.
#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Null,
    /// A decimal number, `unscaled / 10^scale`.
    Num(i128, u8),
    /// Character data (also CLOB, LONG). For CHAR / NCHAR the text before padding.
    Text(String),
    /// RAW, LONG RAW, BLOB.
    Bytes(Vec<u8>),
    /// DATE, TIMESTAMP: seconds since 1970-01-01 (proleptic Gregorian) and nanoseconds.
    Time {
        secs: i64,
        nanos: u32,
    },
}

#[derive(Clone, Debug)]
pub struct ColSpec {
    pub name: String,
    pub ty: ColType,
    /// ` NOT NULL ENABLE` in the `CREATE TABLE` line [T9]; only for a column without a NULL.
    pub not_null: bool,
}

/// An item that the reader only skips (rules I2 to I7, list K, T7, S1, S2).
#[derive(Clone, Debug)]
pub enum Noise {
    /// A complete line [I2, I6].
    Line(String),
    /// A line of list K followed by a plain field [I3].
    Plain { intro: String, field: Vec<u8> },
    /// A line of list K followed by a field in pieces [I4].
    Marked { intro: String, pieces: Vec<Vec<u8>> },
    /// A line of list K followed by pieces without marker [I5]; one piece only (the bytes are confirmed, the
    /// word "pieces" is not).
    Unmarked { intro: String, piece: Vec<u8> },
}

impl Noise {
    /// Fields after an introducing line.
    pub fn fields(&self) -> u64 {
        match self {
            Noise::Line(_) => 0,
            _ => 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TableSpec {
    pub name: String,
    pub columns: Vec<ColSpec>,
    pub rows: Vec<Vec<Cell>>,
    /// Used only for a table without rows on the direct path.
    pub empty_form: EmptyForm,
    /// Items between `CREATE TABLE` and `INSERT INTO` [T7]: `... MODIFY DEFAULT` + plain field.
    pub before: Vec<Noise>,
    /// Items after the rows [T7].
    pub after: Vec<Noise>,
}

impl TableSpec {
    pub fn has_lob(&self) -> bool {
        self.columns.iter().any(|c| c.ty.is_lob())
    }
    /// File order of the columns [T3]: the declared order, but on the direct path a LONG / LONG RAW column
    /// stands last in `INSERT INTO` and in the descriptors.
    pub fn file_order(&self, direct_path: bool) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.columns.len()).collect();
        if direct_path {
            order.sort_by_key(|&i| self.columns[i].ty.is_long());
        }
        order
    }
}

#[derive(Clone, Debug)]
pub struct Spec {
    pub version: Version,
    /// Client set id [H1]; the identifiers and text items are ASCII, so the id changes only the header.
    pub client_set: u16,
    /// Database set id [H4]: 873 (AL32UTF8) or 171 (CL8MSWIN1251).
    pub db_set: u16,
    pub national_set: u16,
    /// `direct=y` [H2]: every table without a LOB column has the path flag 1 [T6, L6].
    pub direct: bool,
    pub mode: Mode,
    pub user: String,
    pub record_length: u64,
    pub tables: Vec<TableSpec>,
    /// Decides sizes of the pieces of a direct-path LONG chain and the locators.
    pub seed: u64,
    /// Owner mode only: groups with fields (PROCACTION, views, statement groups with `ENDTABLE`) [T1, I3-I5].
    pub extras: bool,
    /// The piece sizes of every direct-path LONG chain, and the chunk sizes of every chunked conventional LONG,
    /// whose total equals their sum (review 06b N-2: the fixtures' shapes 156 + 4 x 7,961 for a chain, and for the
    /// chunked form 16 x 4,000 + 1,535 (65,535 bytes) or 25 x 4,000 (100,000 bytes), `long3_exp_<v>`); other
    /// values get random sizes.
    pub chain_shape: Option<Vec<usize>>,
    /// Writes every conventional LONG value in the chunked `fd ff` form, whatever its length. UNCONFIRMED
    /// (`LongChunkedBelow65534`) for a total below 65,534 bytes; used by the negative case only.
    pub chunk_everything: bool,
}

impl Spec {
    /// The path flag of a table [T6, L6, R7].
    pub fn table_direct(&self, t: &TableSpec) -> bool {
        self.direct && !t.has_lob()
    }

    pub fn expected_field_count(&self) -> u64 {
        let mut n = 0u64;
        for t in &self.tables {
            n += t.before.iter().map(Noise::fields).sum::<u64>();
            n += t.after.iter().map(Noise::fields).sum::<u64>();
        }
        n + self.extra_fields()
    }

    /// Fields the owner-mode extras write (PROCACTION, CREATE VIEW).
    fn extra_fields(&self) -> u64 {
        if self.extras && self.mode == Mode::Users {
            2
        } else {
            0
        }
    }
}

// ---------------------------------------------------------------------------------------------------------
// Expected Arrow types and cell texts (the format of `common::cell`)
// ---------------------------------------------------------------------------------------------------------

/// The Arrow type of arch 5.1 for the default options.
pub fn arrow_type(ty: &ColType) -> DataType {
    match ty {
        ColType::NumberPlain => DataType::Decimal128(38, 10),
        ColType::NumberInt(p) if *p <= 18 => DataType::Int64,
        ColType::NumberInt(p) => DataType::Decimal128(*p, 0),
        ColType::NumberDec(p, s) => DataType::Decimal128(*p, *s as i8),
        ColType::Date => DataType::Timestamp(TimeUnit::Second, None),
        ColType::Timestamp(f) if *f <= 6 => DataType::Timestamp(TimeUnit::Microsecond, None),
        ColType::Timestamp(_) => DataType::Timestamp(TimeUnit::Nanosecond, None),
        ColType::Raw(_) => DataType::Binary,
        ColType::Varchar2 { .. }
        | ColType::Char { .. }
        | ColType::NVarchar2(_)
        | ColType::NChar(_) => DataType::Utf8,
        ColType::Long | ColType::Clob => DataType::LargeUtf8,
        ColType::LongRaw | ColType::Blob => DataType::LargeBinary,
    }
}

/// `unscaled / 10^scale` as plain decimal text without trailing zeros (`-12.5`, `0.05`, `100`).
pub fn decimal_text(unscaled: i128, scale: u8) -> String {
    let neg = unscaled < 0;
    let digits = unscaled.unsigned_abs().to_string();
    let scale = scale as usize;
    let padded = format!("{:0>w$}", digits, w = scale + 1);
    let (ip, fp) = padded.split_at(padded.len() - scale);
    let fp = fp.trim_end_matches('0');
    let mut t = String::new();
    if neg {
        t.push('-');
    }
    t.push_str(ip);
    if !fp.is_empty() {
        t.push('.');
        t.push_str(fp);
    }
    t
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// The text of a cell in the format of `common::cell` (NULL is `<NULL>`, binary is lower-case hex, a DATE is
/// `YYYY-MM-DD HH:MM:SS`, a TIMESTAMP is the integer of its unit).
pub fn expected_cell(spec: &Spec, col: &ColSpec, cell: &Cell) -> String {
    match cell {
        Cell::Null => "<NULL>".to_string(),
        Cell::Num(u, s) => decimal_text(*u, *s),
        Cell::Text(s) => padded_text(spec, &col.ty, s),
        Cell::Bytes(b) => hex(b),
        Cell::Time { secs, nanos } => match col.ty {
            ColType::Date => super::ts_text(*secs),
            ColType::Timestamp(f) if f <= 6 => {
                (secs * 1_000_000 + i64::from(*nanos) / 1000).to_string()
            }
            _ => (secs * 1_000_000_000 + i64::from(*nanos)).to_string(),
        },
    }
}

/// One table's rows as the reader must return them, in declared column order. `columns` restricts to the named
/// columns (kept in the table's order, as `ReadOptions.columns` documents).
pub fn expected_rows(spec: &Spec, t: &TableSpec, columns: Option<&[&str]>) -> Vec<Vec<String>> {
    let keep: Vec<usize> = (0..t.columns.len())
        .filter(|&i| columns.is_none_or(|c| c.contains(&t.columns[i].name.as_str())))
        .collect();
    t.rows
        .iter()
        .map(|r| {
            keep.iter()
                .map(|&i| expected_cell(spec, &t.columns[i], &r[i]))
                .collect()
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------------------
// Encoders written from the rules
// ---------------------------------------------------------------------------------------------------------

/// UTF-16 big-endian [C3, C5, L5].
pub fn utf16_be(s: &str) -> Vec<u8> {
    s.encode_utf16().flat_map(u16::to_be_bytes).collect()
}

/// The bytes of a text in the database set [C1, Y9]: UTF-8 for 873, CL8MSWIN1251 for 171 [C7].
pub fn db_bytes(db_set: u16, s: &str) -> Vec<u8> {
    match db_set {
        873 => s.as_bytes().to_vec(),
        171 => {
            let (b, _, bad) = encoding_rs::WINDOWS_1251.encode(s);
            assert!(!bad, "text not representable in CL8MSWIN1251");
            b.into_owned()
        }
        other => panic!("synth: database set {other} is not generated"),
    }
}

/// The text after the padding the format stores: CHAR with byte semantics to n bytes [C1], CHAR(n CHAR) to n
/// characters [C6], NCHAR to n UTF-16 code units with `00 20` [C5]. Other types unchanged.
pub fn padded_text(spec: &Spec, ty: &ColType, s: &str) -> String {
    match ty {
        ColType::Char { n, chars: false } => {
            let have = db_bytes(spec.db_set, s).len();
            let mut t = s.to_string();
            t.extend(std::iter::repeat_n(' ', (*n as usize).saturating_sub(have)));
            t
        }
        ColType::Char { n, chars: true } => {
            let have = s.chars().count();
            let mut t = s.to_string();
            t.extend(std::iter::repeat_n(' ', (*n as usize).saturating_sub(have)));
            t
        }
        ColType::NChar(n) => {
            let have = s.encode_utf16().count();
            let mut t = s.to_string();
            t.extend(std::iter::repeat_n(' ', (*n as usize).saturating_sub(have)));
            t
        }
        _ => s.to_string(),
    }
}

/// Oracle NUMBER bytes of `unscaled / 10^scale` [Y1]: 0 is `80`; base-100 digit bytes; positive: first byte
/// 0xC1 + E and digits + 1; negative: first byte 0x3E - E, digits as 101 - d, and a last byte `66` when fewer
/// than 20 digit bytes stand.
pub fn number_bytes(unscaled: i128, scale: u8) -> Vec<u8> {
    if unscaled == 0 {
        return vec![0x80];
    }
    let negative = unscaled < 0;
    let mut digits: Vec<u8> = unscaled
        .unsigned_abs()
        .to_string()
        .bytes()
        .map(|c| c - b'0')
        .collect();
    // value = 0.DDDD x 10^k
    let mut k = digits.len() as i32 - i32::from(scale);
    if k.rem_euclid(2) != 0 {
        digits.insert(0, 0);
        k += 1;
    }
    if !digits.len().is_multiple_of(2) {
        digits.push(0);
    }
    // base-100: value = 0.[g0 g1 ..] x 100^(k/2); the first digit byte has the weight 100^(E), E = k/2 - 1
    let e = k / 2 - 1;
    let mut groups: Vec<u8> = digits.chunks(2).map(|p| p[0] * 10 + p[1]).collect();
    while groups.last() == Some(&0) {
        groups.pop();
    }
    let mut out = Vec::new();
    if negative {
        out.push((0x3e - e) as u8);
        out.extend(groups.iter().map(|g| 101 - g));
        if groups.len() < 20 {
            out.push(0x66);
        }
    } else {
        out.push((0xc1 + e) as u8);
        out.extend(groups.iter().map(|g| g + 1));
    }
    out
}

/// Civil date of days since 1970-01-01 (proleptic Gregorian).
fn civil(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

/// Days since 1970-01-01 of a civil date.
pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// The 7 bytes of DATE [Y2] (years 1 to 9999): century + 100, year in century + 100, month, day, hour + 1,
/// minute + 1, second + 1.
pub fn date_bytes(secs: i64) -> Vec<u8> {
    let (y, m, d) = civil(secs.div_euclid(86_400));
    let s = secs.rem_euclid(86_400);
    vec![
        (y / 100 + 100) as u8,
        (y % 100 + 100) as u8,
        m as u8,
        d as u8,
        (s / 3600 + 1) as u8,
        (s / 60 % 60 + 1) as u8,
        (s % 60 + 1) as u8,
    ]
}

/// TIMESTAMP [Y5]: 7 bytes when the fraction is zero, otherwise 11 (DATE layout + u32 BE nanoseconds).
pub fn timestamp_bytes(secs: i64, nanos: u32) -> Vec<u8> {
    let mut b = date_bytes(secs);
    if nanos != 0 {
        b.extend(nanos.to_be_bytes());
    }
    b
}

// ---------------------------------------------------------------------------------------------------------
// Descriptors and the CREATE TABLE text
// ---------------------------------------------------------------------------------------------------------

fn le16(v: &mut Vec<u8>, x: u16) {
    v.extend(x.to_le_bytes());
}

fn be16(v: &mut Vec<u8>, x: u16) {
    v.extend(x.to_be_bytes());
}

/// The descriptor length of `TIMESTAMP(f)`, f = 0 to 9, as a literal table from [T5] v3.7 (R-047 answered by
/// `tsp_exp_<v>` and `truth_tsp.txt`: 7 for f = 0, 11 for f = 1 to 9). The values stored are 7 bytes when the
/// fraction is zero and 11 otherwise [Y5], whatever the descriptor says.
pub const TIMESTAMP_DESCRIPTOR_LEN: [u16; 10] = [7, 11, 11, 11, 11, 11, 11, 11, 11, 11];

/// The descriptor [T5]: type code, maximum length, and for types 1, 96, 112 character set id and form [C6].
pub fn descriptor(ty: &ColType, db_set: u16) -> Vec<u8> {
    let w: u32 = if db_set == 873 { 4 } else { 1 };
    let mut d = Vec::new();
    let mut plain = |code: u16, len: u16| {
        le16(&mut d, code);
        le16(&mut d, len);
    };
    match ty {
        ColType::NumberPlain | ColType::NumberInt(_) | ColType::NumberDec(..) => plain(2, 22),
        ColType::Date => plain(12, 7),
        // [T5] v3.7 (R-047, `tsp_exp_<v>`, `t_ts_exp_18c`, `t_decl`): the table below, never computed
        ColType::Timestamp(f) => plain(180, TIMESTAMP_DESCRIPTOR_LEN[usize::from(*f)]),
        ColType::Raw(n) => plain(23, *n),
        ColType::Long => plain(8, 0),
        ColType::LongRaw => plain(24, 0),
        ColType::Blob => plain(113, 72),
        ColType::Varchar2 { n, chars } => {
            let len = if *chars {
                (w * u32::from(*n)).min(4000) as u16
            } else {
                *n
            };
            plain(1, len);
            le16(&mut d, db_set);
            le16(&mut d, 1);
        }
        ColType::Char { n, chars } => {
            let len = if *chars {
                (w * u32::from(*n)).min(2000) as u16
            } else {
                *n
            };
            plain(96, len);
            le16(&mut d, db_set);
            le16(&mut d, 1);
        }
        ColType::NVarchar2(n) => {
            plain(1, n * 2);
            le16(&mut d, 2000);
            le16(&mut d, 2);
        }
        ColType::NChar(n) => {
            plain(96, n * 2);
            le16(&mut d, 2000);
            le16(&mut d, 2);
        }
        ColType::Clob => {
            plain(112, 72);
            le16(&mut d, db_set);
            le16(&mut d, 1);
        }
    }
    d
}

/// The type text of a column definition [T9].
pub fn type_text(ty: &ColType) -> String {
    match ty {
        ColType::NumberPlain => "NUMBER".into(),
        ColType::NumberInt(p) => format!("NUMBER({p}, 0)"),
        ColType::NumberDec(p, s) => format!("NUMBER({p}, {s})"),
        ColType::Date => "DATE".into(),
        ColType::Timestamp(f) => format!("TIMESTAMP ({f})"),
        ColType::Raw(n) => format!("RAW({n})"),
        ColType::Varchar2 { n, chars: false } => format!("VARCHAR2({n})"),
        ColType::Varchar2 { n, chars: true } => format!("VARCHAR2({n} CHAR)"),
        ColType::Char { n, chars: false } => format!("CHAR({n})"),
        ColType::Char { n, chars: true } => format!("CHAR({n} CHAR)"),
        ColType::NVarchar2(n) => format!("NVARCHAR2({n})"),
        ColType::NChar(n) => format!("NCHAR({n})"),
        ColType::Long => "LONG".into(),
        ColType::LongRaw => "LONG RAW".into(),
        ColType::Clob => "CLOB".into(),
        ColType::Blob => "BLOB".into(),
    }
}

/// Storage text after the two spaces of the `CREATE TABLE` line [T9]: free text for the reader (the fixtures'
/// `t_frame` line).
const STORAGE: &str = "PCTFREE 10 PCTUSED 40 INITRANS 1 MAXTRANS 255 STORAGE(INITIAL 65536 NEXT 1048576 MINEXTENTS 1 FREELISTS 1 FREELIST GROUPS 1 BUFFER_POOL DEFAULT) TABLESPACE \"USERS\" LOGGING NOCOMPRESS";

// ---------------------------------------------------------------------------------------------------------
// The layout: where the generator put things (offsets in the file)
// ---------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
pub struct FieldLayout {
    /// Offset of the u16 length (or of `fe ff`).
    pub len_at: u64,
    /// Offset of the first value byte (after the length; for a chain after the first piece u16).
    pub value_at: u64,
    /// Value bytes (all pieces).
    pub len: usize,
    pub null: bool,
    /// Pieces of a direct-path LONG chain, or data chunks of a chunked conventional LONG (1 for an ordinary field).
    pub pieces: usize,
    /// A conventional LONG in the chunked `fd ff` form ([R2] v3.7); `value_at` is then the first data byte, behind
    /// the u16 of the first chunk.
    pub chunked: bool,
    /// A conventional LONG or LONG RAW in the 4-byte length form (b) ([R2] v3.9): `fa ff`, `u16 n`, `00 00`, n bytes;
    /// `value_at` is `len_at + 6`.
    pub four_byte: bool,
    /// Offset of the `00 00` that ends a chunked value (0 otherwise).
    pub chunks_end_at: u64,
}

#[derive(Clone, Debug, Default)]
pub struct RowLayout {
    pub start: u64,
    /// In file order.
    pub fields: Vec<FieldLayout>,
    /// Conventional path: offset of the u16 behind the fields (`00 00`, or k of a LOB table).
    pub tail_at: u64,
    /// Offsets of the LOB values of the row.
    pub lob_values_at: Vec<u64>,
}

#[derive(Clone, Debug, Default)]
pub struct TableLayout {
    pub direct: bool,
    pub table_line: u64,
    pub create_line: u64,
    pub insert_line: u64,
    pub count_at: u64,
    /// Offsets of the descriptors, in file order.
    pub desc_at: Vec<u64>,
    /// File-order index to declared index.
    pub file_order: Vec<usize>,
    pub m_at: u64,
    pub flag_at: u64,
    /// Offsets of the L1 column entries (LOB tables).
    pub entry_at: Vec<u64>,
    pub rows_at: u64,
    pub rows: Vec<RowLayout>,
    /// Offset of the `ff ff`, or (E2) of the LF that stands in its place.
    pub end_at: u64,
    pub e2: bool,
    /// Offset behind the LF that ends the binary part.
    pub after_part: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Layout {
    pub r: u64,
    /// Offset of the RECORDLENGTH digits.
    pub rl_at: u64,
    /// R + 0x104.
    pub first_item: u64,
    pub tables: Vec<TableLayout>,
    pub exit1_at: u64,
    pub exit2_at: u64,
    /// Offset behind the second `EXIT` line.
    pub data_end: u64,
    /// The unconfirmed constructs this file contains (ADR-86); printed by every failure message of `t17_synth.rs`.
    pub unconfirmed: BTreeSet<Unconfirmed>,
}

pub struct Built {
    pub bytes: Vec<u8>,
    pub layout: Layout,
}

// ---------------------------------------------------------------------------------------------------------
// The builder
// ---------------------------------------------------------------------------------------------------------

struct W {
    out: Vec<u8>,
}

impl W {
    fn at(&self) -> u64 {
        self.out.len() as u64
    }
    /// A line [I2].
    fn line(&mut self, s: &str) {
        self.out.extend(s.as_bytes());
        self.out.push(0x0a);
    }
    fn raw(&mut self, b: &[u8]) {
        self.out.extend(b);
    }
    /// A plain field [I3]: u16 LE length, the bytes, LF.
    fn plain_field(&mut self, f: &[u8]) {
        le16(&mut self.out, f.len() as u16);
        self.out.extend(f);
        self.out.push(0x0a);
    }
    /// A field in pieces [I4]: `fd ff`, pieces u16 + bytes, a piece of length 0, LF.
    fn marked_field(&mut self, pieces: &[Vec<u8>]) {
        self.out.extend([0xfd, 0xff]);
        for p in pieces {
            le16(&mut self.out, p.len() as u16);
            self.out.extend(p);
        }
        le16(&mut self.out, 0);
        self.out.push(0x0a);
    }
    /// Pieces without marker [I5]: pieces, a piece of length 0, LF.
    fn unmarked_field(&mut self, piece: &[u8]) {
        le16(&mut self.out, piece.len() as u16);
        self.out.extend(piece);
        le16(&mut self.out, 0);
        self.out.push(0x0a);
    }
    fn noise(&mut self, n: &Noise) {
        match n {
            Noise::Line(l) => self.line(l),
            Noise::Plain { intro, field } => {
                self.line(intro);
                self.plain_field(field);
            }
            Noise::Marked { intro, pieces } => {
                self.line(intro);
                self.marked_field(pieces);
            }
            Noise::Unmarked { intro, piece } => {
                self.line(intro);
                self.unmarked_field(piece);
            }
        }
    }
}

/// The 102-to-139-byte locator of a LOB column [L2]: framed like any field, content not needed.
fn locator(rng: &mut Rng) -> Vec<u8> {
    let n = rng.range(102, 139);
    rng.bytes(n)
}

fn metrics_letter(m: Mode) -> char {
    match m {
        Mode::Tables => 'T',
        Mode::Users => 'U',
    }
}

fn write_header(spec: &Spec, w: &mut W, layout: &mut Layout) {
    let o = &mut w.out;
    o.push(0x03); // [H1]
                  // UNCONFIRMED(ClientSetNotInFixtures): H1, H4; accepted by C4 (the client set never decodes row data);
                  // evidence: none
    if let Some(label) = header_label(spec) {
        layout.unconfirmed.insert(label);
    }
    be16(o, spec.client_set); // [H1] u16 BE client set
    if spec.direct {
        o.extend(b"D\n"); // [H2]
    }
    // [H3] seven LF-terminated text lines
    for l in [spec.version.line().to_string(), format!("U{}", spec.user)] {
        o.extend(l.as_bytes());
        o.push(0x0a);
    }
    o.extend(
        match spec.mode {
            Mode::Tables => "RTABLES",
            Mode::Users => "RUSERS",
        }
        .as_bytes(),
    );
    o.push(0x0a);
    layout.rl_at = o.len() as u64;
    o.extend(spec.record_length.to_string().as_bytes());
    o.push(0x0a);
    for l in ["0", "72", "0"] {
        o.extend(l.as_bytes());
        o.push(0x0a);
    }
    // [H4] client, database, national set (u16 BE), 10 filler bytes (constant in the fixtures), u16 BE file name
    // length, one 00
    be16(o, spec.client_set);
    be16(o, spec.db_set);
    be16(o, spec.national_set);
    o.extend([0x00, 0x01, 0, 0, 0, 0, 0, 0, 0, 0]);
    let name = "/synth/spec.dmp";
    be16(o, name.len() as u16);
    o.push(0x00);
    // [H4a] informative: spaces, the export date text ending at 0x7f, the file name at 0x80
    let date = "Sun Oct 4 22:11:58 2026";
    let date_start = 0x80usize - date.len();
    if o.len() <= date_start {
        o.resize(date_start, b' ');
        o.extend(date.as_bytes());
        o.extend(name.as_bytes());
    }
    // [H5] the first record is R bytes, zero filled
    o.resize(spec.record_length as usize, 0);
    // [H6] R + 0x000 to R + 0x103: skipped by the reader; zeros and the closing LF. The first item follows.
    // CONTRADICTS FIXTURES (harmless: [H6] skip): R + 0x000 to R + 0x102 are zeros here; every fixture holds two
    // blocks of 102 bytes and 8 strings there. The LF at R + 0x103 is as in the fixtures.
    o.resize(spec.record_length as usize + 0x103, 0);
    o.push(0x0a);
    layout.first_item = w.at();
}

/// The CREATE TABLE line [T9]: `CREATE TABLE "t" (` coldef (`, ` coldef)* `)` + two spaces + storage text.
fn create_line(t: &TableSpec) -> String {
    let defs: Vec<String> = t
        .columns
        .iter()
        .map(|c| {
            format!(
                "\"{}\" {}{}",
                c.name,
                type_text(&c.ty),
                if c.not_null { " NOT NULL ENABLE" } else { "" }
            )
        })
        .collect();
    format!(
        "CREATE TABLE \"{}\" ({})  {STORAGE}",
        t.name,
        defs.join(", ")
    )
}

/// The field bytes of a cell, or None for NULL (`fe ff`) [R3]. A LOB cell gives its locator in `build_row`.
fn cell_bytes(spec: &Spec, col: &ColSpec, cell: &Cell) -> Option<Vec<u8>> {
    match cell {
        Cell::Null => None,
        Cell::Num(u, s) => Some(number_bytes(*u, *s)),
        Cell::Time { secs, nanos } => Some(match col.ty {
            ColType::Date => date_bytes(*secs),
            _ => timestamp_bytes(*secs, *nanos),
        }),
        Cell::Bytes(b) => Some(b.clone()),
        Cell::Text(s) => {
            let p = padded_text(spec, &col.ty, s);
            Some(match col.ty {
                ColType::NVarchar2(_) | ColType::NChar(_) => utf16_be(&p),
                // [L5] a CLOB of the AL32UTF8 database is UTF-16 BE
                ColType::Clob => utf16_be(&p),
                _ => db_bytes(spec.db_set, &p),
            })
        }
    }
}

/// Splits a LONG value into the pieces of a direct-path chain [R2]: sizes 1 to 7,961 chosen by the seed.
fn chain_pieces(rng: &mut Rng, total: usize, shape: Option<&Vec<usize>>) -> Vec<usize> {
    if let Some(sh) = shape {
        if sh.iter().sum::<usize>() == total {
            return sh.clone();
        }
    }
    let mut left = total;
    let mut v = Vec::new();
    while left > 0 {
        let take = rng.range(1, 7961).min(left);
        v.push(take);
        left -= take;
    }
    v
}

/// A conventional LONG or LONG RAW from this many bytes is written in the chunked form (`long4_exp_<v>` row 6: 65,534
/// bytes is the smallest value seen chunked; [R2] v3.9).
pub const CHUNKED_FROM: usize = 65_534;
/// A conventional LONG or LONG RAW of 65,530 to 65,533 bytes is written in the 4-byte length form (b) (`long4_exp_<v>`
/// rows 3 to 5, `long5_exp_<v>`; [R2] v3.9).
pub const FOUR_BYTE_FROM: usize = 65_530;
/// The largest conventional LONG or LONG RAW written as one field (`long5_exp_<v>` row 4: `f9 ff` = 65,529 bytes;
/// [R2] v3.9). 65,522 to 65,524 bytes are one field too but only with the label `LongOneFieldFff2ToFff4`.
pub const PLAIN_LONG_MAX: usize = 65_529;
/// The sizes no fixture shows between 65,521 and 65,525 (the u16 0xfff2 to 0xfff4): never drawn at random.
pub const FFF2_TO_FFF4: std::ops::RangeInclusive<usize> = 65_522..=65_524;

/// Splits a conventional LONG value into the chunks of the `fd ff` form: sizes 1 to 4,000 chosen by the seed (4,000
/// is what the fixtures show; the rules say never to assume it); `shape` is used when its sum equals the total.
fn chunk_sizes(rng: &mut Rng, total: usize, shape: Option<&Vec<usize>>) -> Vec<usize> {
    if let Some(sh) = shape {
        if sh.iter().sum::<usize>() == total {
            return sh.clone();
        }
    }
    let mut left = total;
    let mut v = Vec::new();
    while left > 0 {
        let take = rng.range(1, 4000).min(left);
        v.push(take);
        left -= take;
    }
    v
}

fn write_table(
    spec: &Spec,
    t: &TableSpec,
    rng: &mut Rng,
    w: &mut W,
    unconfirmed: &mut BTreeSet<Unconfirmed>,
) -> TableLayout {
    let mut tl = TableLayout {
        direct: spec.table_direct(t),
        ..TableLayout::default()
    };
    let order = t.file_order(tl.direct);
    tl.file_order = order.clone();
    // [T1] TABLE line directly followed by CREATE TABLE
    tl.table_line = w.at();
    w.line(&format!("TABLE \"{}\"", t.name));
    tl.create_line = w.at();
    w.line(&create_line(t));
    for n in &t.before {
        w.noise(n); // [T7] items between CREATE TABLE and INSERT INTO
    }
    // [T3] INSERT INTO with the names in file order
    tl.insert_line = w.at();
    let names: Vec<String> = order
        .iter()
        .map(|&i| format!("\"{}\"", t.columns[i].name))
        .collect();
    let binds: Vec<String> = (1..=order.len()).map(|i| format!(":{i}")).collect();
    w.line(&format!(
        "INSERT INTO \"{}\" ({}) VALUES ({})",
        t.name,
        names.join(", "),
        binds.join(", ")
    ));
    // [T4] column count, descriptors
    tl.count_at = w.at();
    le16(&mut w.out, order.len() as u16);
    for &i in &order {
        tl.desc_at.push(w.at());
        w.raw(&descriptor(&t.columns[i].ty, spec.db_set));
    }
    // [T6] m and the path flag
    let lob = t.has_lob();
    tl.m_at = w.at();
    le16(&mut w.out, if lob { order.len() as u16 } else { 0 });
    tl.flag_at = w.at();
    le16(&mut w.out, u16::from(tl.direct));
    if lob {
        // [L1] m column entries: u8 n, n name bytes, 2 bytes `00 00`
        for &i in &order {
            tl.entry_at.push(w.at());
            if t.columns[i].ty.is_lob() {
                w.out.push(t.columns[i].name.len() as u8);
                w.raw(t.columns[i].name.as_bytes());
            } else {
                w.out.push(0);
            }
            w.raw(&[0, 0]);
        }
    }
    tl.rows_at = w.at();
    // rows [R1]
    for row in &t.rows {
        let mut rl = RowLayout {
            start: w.at(),
            ..RowLayout::default()
        };
        let mut lob_values: Vec<Vec<u8>> = Vec::new();
        for &i in &order {
            let col = &t.columns[i];
            let cell = &row[i];
            let mut f = FieldLayout {
                len_at: w.at(),
                pieces: 1,
                ..FieldLayout::default()
            };
            let bytes: Option<Vec<u8>> = if col.ty.is_lob() {
                // [L2] a locator or NULL; [L3] the value follows the row
                match cell {
                    Cell::Null => None,
                    c => {
                        let mut v = Vec::new();
                        // [L4] short form: u16 n + n bytes (the generated values are far below one chunk)
                        let b = cell_bytes(spec, col, c).unwrap_or_default();
                        le16(&mut v, b.len() as u16);
                        v.extend(b);
                        lob_values.push(v);
                        Some(locator(rng))
                    }
                }
            } else {
                cell_bytes(spec, col, cell)
            };
            match bytes {
                None => {
                    f.null = true; // [R3]
                    w.raw(&[0xfe, 0xff]);
                    f.value_at = w.at();
                }
                Some(b) if tl.direct && col.ty.is_long() => {
                    // [R2] a piece chain: u16 n (bit 0x8000 = another piece follows), n & 0x7fff bytes
                    let sizes = chain_pieces(rng, b.len(), spec.chain_shape.as_ref());
                    f.pieces = sizes.len();
                    f.len = b.len();
                    let mut at = 0usize;
                    for (k, sz) in sizes.iter().enumerate() {
                        let more = k + 1 < sizes.len();
                        le16(&mut w.out, (*sz as u16) | if more { 0x8000 } else { 0 });
                        if k == 0 {
                            f.value_at = w.at();
                        }
                        w.raw(&b[at..at + sz]);
                        at += sz;
                    }
                }
                Some(b)
                    if col.ty.is_long() && (b.len() >= CHUNKED_FROM || spec.chunk_everything) =>
                {
                    // [R2] v3.9, [Y9] v3.9 (R-040, R-048, R-051: 65,534 to 100,000 bytes in `long3_exp_<v>`,
                    // `long4_exp_<v>`, `longraw4_exp_18c`, `longraw5_exp_<v>`): a conventional LONG or LONG RAW from
                    // 65,534 bytes is `fd ff`, chunks `u16 n + n bytes` (4,000 each in the fixtures, the remainder
                    // last), and a chunk `00 00`. The chunk sizes here are 1 to 4,000, chosen by the seed (the rules
                    // say never to assume 4,000).
                    if b.len() < CHUNKED_FROM {
                        // UNCONFIRMED(LongChunkedBelow65534): [R2] v3.9; accepted by [R2] ("a chunked value is
                        // self-delimiting", the chunk framing has one reading); evidence: none
                        unconfirmed.insert(Unconfirmed::LongChunkedBelow65534);
                    }
                    let sizes = chunk_sizes(rng, b.len(), spec.chain_shape.as_ref());
                    f.pieces = sizes.len();
                    f.len = b.len();
                    f.chunked = true;
                    w.raw(&[0xfd, 0xff]);
                    let mut at = 0usize;
                    for (k, sz) in sizes.iter().enumerate() {
                        le16(&mut w.out, *sz as u16);
                        if k == 0 {
                            f.value_at = w.at();
                        }
                        w.raw(&b[at..at + sz]);
                        at += sz;
                    }
                    if sizes.is_empty() {
                        f.value_at = w.at() + 2;
                    }
                    f.chunks_end_at = w.at();
                    le16(&mut w.out, 0);
                }
                Some(b) if col.ty.is_long() && b.len() >= FOUR_BYTE_FROM => {
                    // [R2] v3.9 (R-048: 65,530 to 65,533 bytes in `long4_exp_<v>`, `long5_exp_<v>`,
                    // `longraw5_exp_<v>`): form (b), `fa ff`, then the length as 4 bytes (u16 n, `00 00`), then the
                    // n bytes; value_at is the first data byte
                    assert!(b.len() < CHUNKED_FROM, "form (b) of {} bytes", b.len());
                    f.four_byte = true;
                    f.len = b.len();
                    w.raw(&[0xfa, 0xff]);
                    le16(&mut w.out, b.len() as u16);
                    le16(&mut w.out, 0);
                    f.value_at = w.at();
                    w.raw(&b);
                }
                Some(b) => {
                    // [R2] u16 LE length + value bytes. A conventional LONG or LONG RAW up to 65,529 bytes is one
                    // field with the length in all 16 bits ([R2] v3.9, `long3_exp_<v>` rows 1 to 4, `long4_exp_<v>`
                    // rows 1 and 2, `long5_exp_<v>` rows 1 to 4); 65,522 to 65,524 bytes no fixture shows.
                    match col.ty {
                        ColType::Long | ColType::LongRaw => {
                            assert!(b.len() <= PLAIN_LONG_MAX, "LONG of {} bytes", b.len());
                            if FFF2_TO_FFF4.contains(&b.len()) {
                                // UNCONFIRMED(LongOneFieldFff2ToFff4): [R2] v3.9 (a); accepted by [R2] (the sizes
                                // on both sides are one field); evidence: R-052
                                unconfirmed.insert(Unconfirmed::LongOneFieldFff2ToFff4);
                            }
                        }
                        _ => assert!(b.len() < 0xfff0),
                    }
                    le16(&mut w.out, b.len() as u16);
                    f.value_at = w.at();
                    f.len = b.len();
                    w.raw(&b);
                }
            }
            rl.fields.push(f);
        }
        if !tl.direct {
            // [R4] conventional path: the u16 behind the row is k (k = 0 is `00 00`), then k LOB values [L3]
            rl.tail_at = w.at();
            le16(&mut w.out, lob_values.len() as u16);
            for v in &lob_values {
                rl.lob_values_at.push(w.at());
                w.raw(v);
            }
        }
        tl.rows.push(rl);
    }
    // [E1] / [E2] end of rows
    let e2 = tl.direct && t.rows.is_empty() && t.empty_form == EmptyForm::E2;
    tl.e2 = e2;
    tl.end_at = w.at();
    if !e2 {
        // UNCONFIRMED(E1AfterFlag1In18cOwnerMode): E1, E3; accepted by E3 (E1 follows the path flag in 11g and, since
        // `seg_exp_direct_18c`, in 18c table mode); evidence: none for owner mode. An 18c table-mode file holds
        // no variant (ADR-94 item 2).
        if tl.direct
            && t.rows.is_empty()
            && spec.version == Version::V18
            && spec.mode == Mode::Users
        {
            unconfirmed.insert(Unconfirmed::E1AfterFlag1In18cOwnerMode);
        }
        w.raw(&[0xff, 0xff]);
    }
    w.out.push(0x0a);
    tl.after_part = w.at();
    for n in &t.after {
        w.noise(n);
    }
    tl
}

fn metrics_group(w: &mut W, m: Mode, title: &str, n: u64, body: impl FnOnce(&mut W)) {
    let l = metrics_letter(m);
    w.line(&format!("METRICS{l}{title}"));
    body(w);
    w.line(&format!("METRICE{l} {n}"));
}

/// Builds the file of a `Spec`.
pub fn build(spec: &Spec) -> Built {
    let mut layout = Layout {
        r: spec.record_length,
        ..Layout::default()
    };
    let mut w = W { out: Vec::new() };
    write_header(spec, &mut w, &mut layout);
    let mut rng = Rng::new(spec.seed);
    let m = spec.mode;
    let l = metrics_letter(m);
    // [H6] the first item: METRICST (table mode) or METRICSU (owner mode), pairing with the R line
    w.line(&format!("METRICS{l}"));
    if m == Mode::Users && spec.extras {
        // owner mode: groups before the tables (as `own_exp_*`): a PROCACTION field in pieces [I4]
        metrics_group(
            &mut w,
            m,
            "pre-schema procedural objects and actions",
            8,
            |w| {
                w.line("PROCACTION");
                w.marked_field(&[b"BEGIN ".to_vec(), b"NULL;".to_vec(), b" END;".to_vec()]);
            },
        );
        metrics_group(&mut w, m, "database links", 8, |_| {});
        metrics_group(&mut w, m, "sequence numbers", 9, |w| {
            w.line("CREATE SEQUENCE \"S1\" MINVALUE 1 MAXVALUE 999999 INCREMENT BY 1 START WITH 1 NOCACHE NOORDER NOCYCLE");
        });
        metrics_group(&mut w, m, "cluster definitions", 9, |_| {});
    }
    for t in spec.tables.iter() {
        let tl = write_table(spec, t, &mut rng, &mut w, &mut layout.unconfirmed);
        layout.tables.push(tl);
    }
    // trailer [T2: the section ends at a line that starts with METRICS]
    let rows: u64 = spec.tables.last().map_or(0, |t| t.rows.len() as u64);
    metrics_group(&mut w, m, "referential integrity constraints", rows, |_| {});
    metrics_group(&mut w, m, "triggers", rows, |_| {});
    if m == Mode::Users && spec.extras {
        // views: `CREATE VIEW` + pieces without marker [I5]
        metrics_group(&mut w, m, "views", 8, |w| {
            w.line("CREATE VIEW \"V1\" (\"A\") AS");
            w.unmarked_field(b"SELECT 1 FROM DUAL");
        });
        // [T1] statement groups: TABLE lines not followed by CREATE TABLE, closed by ENDTABLE; one ENDTABLE can
        // close several TABLE lines
        metrics_group(&mut w, m, "Enable enforce constraints ", 8, |w| {
            for (k, t) in spec.tables.iter().enumerate() {
                w.line(&format!("TABLE \"{}\"", t.name));
                if k % 2 == 0 {
                    w.line(&format!(
                        "ALTER TABLE \"{}\" ENABLE CONSTRAINT \"C1\"",
                        t.name
                    ));
                    w.line("ENDTABLE");
                }
            }
            // the last TABLE line is open when the number of tables is even
            if !spec.tables.is_empty() && spec.tables.len().is_multiple_of(2) {
                w.line("ENDTABLE");
            }
        });
    }
    metrics_group(
        &mut w,
        m,
        "bitmap, functional and extensible indexes",
        rows,
        |_| {},
    );
    metrics_group(&mut w, m, "posttables actions", rows, |_| {});
    metrics_group(&mut w, m, "Post-inst procedural actions ", rows, |_| {});
    w.line(&format!("METRICE{l}G0")); // end of the data
    layout.exit1_at = w.at();
    w.line("EXIT"); // [E4]
    layout.exit2_at = w.at();
    w.line("EXIT");
    layout.data_end = w.at();
    let r = spec.record_length as usize;
    let len = w.out.len().div_ceil(r) * r;
    w.out.resize(len, 0); // [E4] zero fill to a multiple of R
    Built {
        bytes: w.out,
        layout,
    }
}

// ---------------------------------------------------------------------------------------------------------
// Random Specs
// ---------------------------------------------------------------------------------------------------------

/// What a random `Spec` is built from: everything is derived from one `u64` (`params_from_seed`), so the property
/// test, the coverage test and the fuzz seeds draw from one distribution (review 06b N-4). A failure message
/// prints `origin`; shrinking a seed means nothing, the message is the replay.
#[derive(Clone, Debug)]
pub struct Params {
    /// The `u64` these `Params` were derived from.
    pub origin: u64,
    pub seed: u64,
    pub tables: usize,
    pub max_rows: usize,
    pub null_pct: u32,
    pub record_length: u64,
    /// Index into `HEADER_CHOICES` (version, mode, client set, database set).
    pub header: usize,
    pub direct: bool,
}

/// Params of a seed, over the ranges of the property test (used by the property tests, the coverage test and the
/// fuzz seeds).
pub fn params_from_seed(origin: u64) -> Params {
    let mut rng = Rng::new(origin);
    Params {
        origin,
        seed: rng.next_u64(),
        tables: rng.range(1, 4),
        max_rows: rng.range(0, 300),
        null_pct: rng.range(0, 100) as u32,
        record_length: rng.pick(&[2048u64, 4096, 8192, 16384]),
        header: rng.below(HEADER_CHOICES.len() as u64) as usize,
        direct: rng.chance(50),
    }
}

const ASCII_POOL: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 _-.,;:!?()[]{}'\"\\/@#$%&*+=<>";
const WIDE_POOL: [char; 14] = [
    'é',
    'ß',
    'я',
    'Ж',
    'ñ',
    '日',
    '本',
    '語',
    '€',
    '😀',
    '\u{10348}',
    '\n',
    '\t',
    '\u{00a0}',
];
const CYR_POOL: [char; 8] = ['А', 'Б', 'я', 'ж', 'Ё', 'ё', 'Щ', 'ы'];

/// A random text of `1..=max_chars` characters whose database-set encoding has at most `max_bytes` bytes.
fn gen_text(rng: &mut Rng, db_set: u16, max_chars: usize, max_bytes: usize) -> String {
    let want = if rng.chance(70) {
        rng.range(1, max_chars.min(12))
    } else {
        rng.range(1, max_chars)
    };
    let mut s = String::new();
    let mut bytes = 0usize;
    let mut chars = 0usize;
    while chars < want {
        let c = match rng.below(10) {
            0..=6 => ASCII_POOL
                .chars()
                .nth(rng.below(ASCII_POOL.chars().count() as u64) as usize)
                .unwrap_or('a'),
            7 | 8 => {
                if db_set == 873 {
                    WIDE_POOL[rng.below(WIDE_POOL.len() as u64) as usize]
                } else {
                    CYR_POOL[rng.below(CYR_POOL.len() as u64) as usize]
                }
            }
            _ => CYR_POOL[rng.below(CYR_POOL.len() as u64) as usize],
        };
        let b = if db_set == 873 { c.len_utf8() } else { 1 };
        if bytes + b > max_bytes {
            break;
        }
        bytes += b;
        chars += 1;
        s.push(c);
    }
    if s.is_empty() {
        s.push('x'); // [R8] an empty string is NULL, a column field never has length 0
    }
    s
}

/// Like `gen_text`, but the UTF-16 length is at most `max_units` code units (NVARCHAR2, NCHAR).
fn gen_text_units(rng: &mut Rng, max_units: usize) -> String {
    let mut s = gen_text(rng, 873, max_units, usize::MAX);
    while s.encode_utf16().count() > max_units {
        s.pop();
    }
    if s.is_empty() {
        s.push('x');
    }
    s
}

fn gen_digits(rng: &mut Rng, n: usize) -> i128 {
    let mut v: i128 = i128::from(rng.range(1, 9) as u8);
    for _ in 1..n {
        v = v * 10 + i128::from(rng.range(0, 9) as u8);
    }
    v
}

fn gen_number(rng: &mut Rng, ty: &ColType) -> Cell {
    let (digits_max, scale) = match ty {
        ColType::NumberInt(p) => (*p as usize, 0u8),
        ColType::NumberDec(p, s) => (*p as usize, *s),
        _ => {
            let sc = rng.range(0, 10) as u8;
            // Decimal128(38, 10): at most 28 digits before the point and 10 behind it
            (28 + sc as usize, sc)
        }
    };
    if rng.chance(8) {
        return Cell::Num(0, scale); // [Y1] zero is the single byte 80
    }
    let d = if rng.chance(50) {
        rng.range(1, digits_max.min(6))
    } else {
        rng.range(1, digits_max)
    };
    let mut v = gen_digits(rng, d);
    if rng.chance(40) {
        v = -v;
    }
    Cell::Num(v, scale)
}

fn gen_time(rng: &mut Rng, f: Option<u8>) -> Cell {
    let wide = f.is_none_or(|f| f <= 6);
    let year: i64 = if rng.chance(75) {
        rng.range(1950, 2050) as i64
    } else if wide {
        rng.range(1, 9999) as i64
    } else {
        rng.range(1700, 2250) as i64 // inside the range of a nanosecond Timestamp
    };
    let month = rng.range(1, 12) as i64;
    let dim = match month {
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    let day = rng.range(1, dim) as i64;
    let secs = days_from_civil(year, month, day) * 86_400
        + rng.range(0, 23) as i64 * 3600
        + rng.range(0, 59) as i64 * 60
        + rng.range(0, 59) as i64;
    let nanos = match f {
        Some(f) if f > 0 && !rng.chance(25) => {
            let unit = 10u64.pow(9 - u32::from(f));
            (rng.below(10u64.pow(u32::from(f))) * unit) as u32
        }
        _ => 0,
    };
    Cell::Time { secs, nanos }
}

/// The sizes of the large LONG and LONG RAW values (ADR-94 items 1, 2, ADR-80 and ADR-99 as amended): edge values of
/// `long3_exp_<v>`, `long4_exp_<v>`, `long5_exp_<v>` and their neighbours, then uniform draws from the four ranges.
/// Never 65,522 to 65,524 (the sizes of `FFF2_TO_FFF4`: R-052).
pub const BIG_LONG_EDGES: [usize; 15] = [
    32_768, 40_000, 65_519, 65_520, 65_521, 65_525, 65_529, 65_530, 65_533, 65_534, 65_535, 65_536,
    100_000, 100_001, 150_000,
];

/// One size of a large value, 32,768 to 65,521 or 65,525 to 65,529 (one field), 65,530 to 65,533 (form (b)) or
/// 65,534 to 150,000 (chunked or a direct chain).
pub fn big_long_size(rng: &mut Rng) -> usize {
    if rng.chance(25) {
        rng.pick(&BIG_LONG_EDGES)
    } else {
        match rng.below(4) {
            0 => rng.range(32_768, 65_521),
            1 => rng.range(65_525, PLAIN_LONG_MAX),
            2 => rng.range(FOUR_BYTE_FROM, CHUNKED_FROM - 1),
            _ => rng.range(CHUNKED_FROM, 150_000),
        }
    }
}

fn gen_cell(rng: &mut Rng, spec_db: u16, ty: &ColType, null_pct: u32, big_long: bool) -> Cell {
    if rng.chance(null_pct) {
        return Cell::Null;
    }
    match ty {
        ColType::NumberPlain | ColType::NumberInt(_) | ColType::NumberDec(..) => {
            gen_number(rng, ty)
        }
        ColType::Date => gen_time(rng, None),
        ColType::Timestamp(f) => gen_time(rng, Some(*f)),
        ColType::Raw(n) => {
            let len = rng.range(1, (*n as usize).min(60));
            Cell::Bytes(rng.bytes(len))
        }
        ColType::Varchar2 { n, chars } => {
            let w = if spec_db == 873 { 4 } else { 1 };
            let (max_chars, max_bytes) = if *chars {
                (*n as usize, (w * *n as usize).min(4000))
            } else {
                (*n as usize, *n as usize)
            };
            let cap = if rng.chance(95) {
                max_chars.min(60)
            } else {
                max_chars.min(1000)
            };
            Cell::Text(gen_text(rng, spec_db, cap, max_bytes))
        }
        ColType::Char { n, chars } => {
            let w = if spec_db == 873 { 4 } else { 1 };
            let max_bytes = if *chars {
                (w * *n as usize).min(2000)
            } else {
                *n as usize
            };
            Cell::Text(gen_text(rng, spec_db, *n as usize, max_bytes))
        }
        ColType::NVarchar2(n) => Cell::Text(gen_text_units(rng, (*n as usize).min(80))),
        ColType::NChar(n) => Cell::Text(gen_text_units(rng, *n as usize)),
        ColType::Long => {
            // [R2] v3.9: large LONG values only in the combination `long3_exp_<v>`, `long4_exp_<v>` and `long5_exp_<v>`
            // show (18c or 11g, table mode, database and client set 873); elsewhere at most 32,000 bytes
            let n = if big_long && rng.chance(15) {
                big_long_size(rng)
            } else if rng.chance(3) {
                32_000
            } else if rng.chance(3) {
                rng.range(8_000, 31_999)
            } else {
                rng.range(1, 300)
            };
            Cell::Text(long_text(rng, spec_db, n))
        }
        ColType::LongRaw => {
            // [R2] v3.9: large LONG RAW values only in the combination `longraw4_exp_18c` and `longraw5_exp_<v>` show
            // (18c or 11g, table mode, sets 873 / 873); elsewhere at most 32,000 bytes
            let n = if big_long && rng.chance(15) {
                big_long_size(rng)
            } else if rng.chance(3) {
                32_000
            } else if rng.chance(3) {
                rng.range(8_000, 31_999)
            } else {
                rng.range(1, 300)
            };
            Cell::Bytes(rng.bytes(n))
        }
        ColType::Clob => {
            if rng.chance(8) {
                Cell::Text(String::new()) // an empty LOB: a locator and a value of length 0 [L4]
            } else {
                let n = if rng.chance(10) {
                    rng.range(500, 3000)
                } else {
                    rng.range(1, 80)
                };
                Cell::Text(gen_text(rng, 873, n, usize::MAX))
            }
        }
        ColType::Blob => {
            if rng.chance(8) {
                Cell::Bytes(Vec::new())
            } else {
                let n = if rng.chance(10) {
                    rng.range(500, 4000)
                } else {
                    rng.range(1, 80)
                };
                Cell::Bytes(rng.bytes(n))
            }
        }
    }
}

/// A LONG text of exactly `bytes` bytes in the database set (so that 32,000 bytes can be hit).
fn long_text(rng: &mut Rng, db_set: u16, bytes: usize) -> String {
    let mut s = String::new();
    let mut have = 0usize;
    while have < bytes {
        let c = if db_set == 873 && rng.chance(10) {
            WIDE_POOL[rng.below(WIDE_POOL.len() as u64 - 2) as usize]
        } else {
            // [Y9] the database-set rule for LONG is confirmed for AL32UTF8 only: in a CL8MSWIN1251 database a
            // LONG holds ASCII (the reader warns W-LONG-CHARSET for a byte above 0x7f there)
            ASCII_POOL
                .chars()
                .nth(rng.below(ASCII_POOL.chars().count() as u64) as usize)
                .unwrap_or('a')
        };
        let b = if db_set == 873 { c.len_utf8() } else { 1 };
        if have + b > bytes {
            // fill with ASCII to land on the exact size
            s.push('z');
            have += 1;
        } else {
            s.push(c);
            have += b;
        }
    }
    s
}

fn gen_name(rng: &mut Rng, used: &mut Vec<String>, prefix: &str, special_pct: u32) -> String {
    loop {
        let mut n = format!("{prefix}{}", rng.range(0, 999));
        if rng.chance(special_pct) {
            // [T9, chapter 10] a quoted name can hold `,` and `(`
            n.push_str(rng.pick(&[", X", "(1)", "_$#", " Z"]));
        }
        if !used.contains(&n) {
            used.push(n.clone());
            return n;
        }
    }
}

fn gen_text_blob(rng: &mut Rng, max: usize) -> Vec<u8> {
    let n = rng.range(1, max);
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(match rng.below(20) {
            0 => b'\n',
            1 => b'"',
            2 => b'\'',
            _ => b' ' + rng.below(95) as u8,
        });
    }
    v
}

fn gen_noise_before(rng: &mut Rng, t: &str, cols: &[ColSpec]) -> Vec<Noise> {
    let mut v = Vec::new();
    for _ in 0..rng.range(0, 2) {
        let c = &cols[rng.below(cols.len() as u64) as usize].name;
        v.push(Noise::Plain {
            intro: format!("ALTER TABLE \"{t}\" MODIFY DEFAULT"), // list K: suffix ` MODIFY DEFAULT`, plain
            field: format!("ALTER TABLE \"{t}\" MODIFY (\"{c}\" DEFAULT 'n/a' )").into_bytes(),
        });
    }
    v
}

fn gen_noise_after(
    rng: &mut Rng,
    t: &str,
    cols: &[ColSpec],
    first_known_by_e2: bool,
) -> Vec<Noise> {
    let mut v: Vec<Noise> = Vec::new();
    let k = rng.range(0, 5);
    for _ in 0..k {
        let c = &cols[rng.below(cols.len() as u64) as usize].name;
        let n = match rng.below(8) {
            0 => Noise::Line(format!("GRANT SELECT ON \"{t}\" TO \"U{}\"", rng.range(0, 99))),
            1 => Noise::Line(format!(
                "CREATE INDEX \"{t}_IX\" ON \"{t}\" (\"{c}\" )  PCTFREE 10 INITRANS 2 MAXTRANS 255 TABLESPACE \"USERS\" LOGGING"
            )),
            2 => Noise::Plain {
                intro: format!("COMMENT ON TABLE \"{t}\" IS "), // list K: prefix `COMMENT ON `, plain
                field: gen_text_blob(rng, 400),
            },
            3 => Noise::Plain {
                intro: format!("ANALSTATS TABLE \"{t}\""), // list K: prefix `ANALSTATS `, plain or marked
                field: gen_text_blob(rng, 300),
            },
            4 => {
                let np = rng.range(1, 6);
                Noise::Marked {
                    intro: format!("ANALSTATS TABLE \"{t}\""),
                    pieces: (0..np).map(|_| gen_text_blob(rng, 200)).collect(),
                }
            }
            5 => {
                let np = rng.range(1, 6);
                Noise::Marked {
                    intro: "CALTER TABLE".to_string(), // list K: exact `CALTER TABLE`, marked
                    pieces: (0..np).map(|_| gen_text_blob(rng, 200)).collect(),
                }
            }
            6 => Noise::Line(format!("ALTER TABLE \"{t}\" ADD  PRIMARY KEY (\"{c}\") USING INDEX PCTFREE 10")),
            _ => Noise::Line(format!("ANALCOMPUTE TABLE \"{t}\"")), // [S2] a complete line
        };
        // [E2 (a)] an empty direct-path table of the E2 form must be followed by an item of the list of E2
        if v.is_empty()
            && first_known_by_e2
            && matches!(&n, Noise::Line(l) if l.starts_with("ANALCOMPUTE"))
        {
            continue;
        }
        v.push(n);
    }
    v
}

/// A random `Spec` (ADR-80 (c)).
pub fn random_spec(p: &Params) -> Spec {
    let mut rng = Rng::new(p.seed);
    let header = HEADER_CHOICES[p.header % HEADER_CHOICES.len()];
    let db_set: u16 = header.db_set;
    let client_set = header.client_set;
    let version = header.version;
    // the one combination the `long3` fixtures show (ADR-94 items 1, 2)
    let big_long = header.mode == Mode::Tables && header.client_set == 873 && header.db_set == 873;
    let mut used_tables: Vec<String> = Vec::new();
    let mut tables = Vec::new();
    for _ in 0..p.tables {
        let tname = gen_name(&mut rng, &mut used_tables, "T", 8);
        let mut used_cols: Vec<String> = Vec::new();
        let ncols = rng.range(1, 8);
        let mut cols: Vec<ColSpec> = Vec::new();
        let mut has_long = false;
        let mut has_lob = false;
        // the table kind: a LOB table (AL32UTF8 only, ADR-80), a LONG table, or neither
        let kind = rng.below(10);
        for i in 0..ncols {
            let mut ty = loop {
                let t = match rng.below(15) {
                    0 => ColType::NumberPlain,
                    1 => ColType::NumberInt(rng.range(1, 38) as u8),
                    2 => {
                        let pp = rng.range(1, 38) as u8;
                        ColType::NumberDec(pp, rng.range(1, pp as usize) as u8)
                    }
                    3 => ColType::Date,
                    // f = 0 to 9: R-047 answered by `tsp_exp_<v>` ([T5] v3.7, ADR-94 item 1)
                    4 => ColType::Timestamp(rng.below(10) as u8),
                    5 => ColType::Raw(rng.range(1, 200) as u16),
                    6 | 7 => ColType::Varchar2 {
                        n: if rng.chance(10) {
                            4000
                        } else {
                            rng.range(1, 100) as u16
                        },
                        chars: rng.chance(30),
                    },
                    8 => ColType::Char {
                        n: rng.range(1, 40) as u16,
                        chars: rng.chance(40),
                    },
                    9 => ColType::NVarchar2(rng.range(1, 100) as u16),
                    10 => ColType::NChar(rng.range(1, 30) as u16),
                    11 => ColType::NumberInt(rng.range(1, 18) as u8),
                    12 => {
                        if kind == 0 && !has_lob && i > 0 {
                            ColType::Long
                        } else if kind == 1 && !has_long && i > 0 {
                            ColType::LongRaw
                        } else {
                            ColType::Date
                        }
                    }
                    13 => {
                        if (kind == 2 || kind == 3) && db_set == 873 && !has_long {
                            ColType::Clob
                        } else {
                            ColType::Varchar2 {
                                n: 30,
                                chars: false,
                            }
                        }
                    }
                    _ => {
                        if (kind == 2 || kind == 3) && db_set == 873 && !has_long {
                            ColType::Blob
                        } else {
                            ColType::Raw(20)
                        }
                    }
                };
                // one LONG per table; a LONG table has no LOB column; LOB only in AL32UTF8
                if t.is_long() && (has_long || has_lob) {
                    continue;
                }
                if t.is_lob() && has_long {
                    continue;
                }
                break t;
            };
            if ty.is_long() {
                has_long = true;
            }
            if ty.is_lob() {
                has_lob = true;
            }
            // NVARCHAR2 / NCHAR descriptors: 2n must fit u16, always true here
            if let ColType::Char { n, chars: true } = ty {
                ty = ColType::Char {
                    n: n.min(40),
                    chars: true,
                };
            }
            let name = gen_name(&mut rng, &mut used_cols, "C", 6);
            cols.push(ColSpec {
                name,
                ty,
                not_null: false,
            });
        }
        // a LONG table has at least two columns [E2a is suggested, so no LONG-only table]
        if has_long && cols.len() == 1 {
            cols.push(ColSpec {
                name: gen_name(&mut rng, &mut used_cols, "C", 0),
                ty: ColType::Date,
                not_null: false,
            });
        }
        let nrows = if rng.chance(15) {
            0
        } else {
            rng.range(0, p.max_rows)
        };
        let mut rows = Vec::with_capacity(nrows);
        for _ in 0..nrows {
            rows.push(
                cols.iter()
                    .map(|c| gen_cell(&mut rng, db_set, &c.ty, p.null_pct, big_long))
                    .collect::<Vec<_>>(),
            );
        }
        for (i, c) in cols.iter_mut().enumerate() {
            c.not_null =
                !rows.is_empty() && rows.iter().all(|r| r[i] != Cell::Null) && rng.chance(50);
        }
        let empty_form = if version == Version::V18 && p.direct && rng.chance(50) {
            EmptyForm::E2
        } else {
            EmptyForm::E1
        };
        let before = gen_noise_before(&mut rng, &tname, &cols);
        let after = gen_noise_after(
            &mut rng,
            &tname,
            &cols,
            empty_form == EmptyForm::E2 && rows.is_empty(),
        );
        tables.push(TableSpec {
            name: tname,
            columns: cols,
            rows,
            empty_form,
            before,
            after,
        });
    }
    Spec {
        version,
        client_set,
        db_set,
        national_set: 2000,
        direct: p.direct,
        mode: header.mode,
        user: "CORPUS".to_string(),
        record_length: p.record_length,
        tables,
        seed: p.seed ^ 0x5eed,
        extras: header.mode == Mode::Users,
        chain_shape: None,
        chunk_everything: false,
    }
}

/// A one-column helper for deterministic cases.
pub fn col(name: &str, ty: ColType) -> ColSpec {
    ColSpec {
        name: name.to_string(),
        ty,
        not_null: false,
    }
}

/// A table with no noise.
pub fn table(name: &str, columns: Vec<ColSpec>, rows: Vec<Vec<Cell>>) -> TableSpec {
    TableSpec {
        name: name.to_string(),
        columns,
        rows,
        empty_form: EmptyForm::E1,
        before: Vec::new(),
        after: Vec::new(),
    }
}

/// A plain spec: table mode, client = database = AL32UTF8, conventional path.
pub fn spec_of(tables: Vec<TableSpec>) -> Spec {
    Spec {
        version: Version::V18,
        client_set: 873,
        db_set: 873,
        national_set: 2000,
        direct: false,
        mode: Mode::Tables,
        user: "CORPUS".to_string(),
        record_length: 2048,
        tables,
        seed: 1,
        extras: false,
        chain_shape: None,
        chunk_everything: false,
    }
}
