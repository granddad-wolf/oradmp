//! The public entry of the core (architecture 3.2): `open`, `Dump::info`, `Dump::scan`, the catalog and the
//! table metadata, and a row cursor that starts at a table's `rows_offset`.
//!
//! `open` reads the first record and the first item at R + 0x104 and nothing else. `scan` walks the file once,
//! decodes no value and caches the result. Row counts are counted in the scan; `METRICET` / `METRICEU`
//! numbers are never read [R6]. A reader re-opens the path and refuses with `FileChanged` if size or
//! modification time differ from the file at `open`. Names and DDL are decoded with the client character
//! set of the header (or the `client_charset` override; architecture 6.1, 6.4); a byte the set does not allow
//! becomes U+FFFD and sets the `*_possibly_damaged` / `ddl_lossy` flags, and so does a `?` (0xbf with
//! WE8MSWIN1252) in a text that was converted at export (architecture 6.5).

use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use arrow_schema::DataType;

use crate::charset::{decode_text, has_replacement_char, Charset};
use crate::error::{Error, ErrorCode, Result, Warning, WarningCode};
use crate::exp::header::{read_first_item, read_header, Header, Mode};
use crate::exp::intro::IntroList;
use crate::exp::items::{walk_path, ItemCounts, PartitionLine, Section, WalkEnd, WalkOptions};
use crate::exp::rows::{EndMarker, Env, LobSummary, LongStats, RowCursor, RowSink, Step};
use crate::exp::table::PathKind;
use crate::io::{ByteStream, DEFAULT_BUFFER, MIN_BUFFER_BYTES};
use crate::reader::{ReadOptions, ReaderInit, TableBatches};
use crate::types::declared::DeclaredType;
use crate::types::map::{TypeOptions, TypeSource};
use crate::types::resolve::{
    first_part_order, part_order, resolve_table, PartLayout, ResolveInput,
};
use crate::values::number::OverflowPolicy;

/// Options of `open` (the part of architecture 3.2 that exists in task T-3).
#[derive(Debug, Clone)]
pub struct Options {
    /// A longer line is `Framing` (architecture 2.6).
    pub max_line_bytes: usize,
    /// Size of the I/O buffer.
    pub buffer_bytes: usize,
    /// Options that decide the Arrow types (`number_default`, `number_types`, `timestamp_unit`). Its
    /// `db_charset` field is set by `open` from [`Options::db_charset`]; a value given here is replaced.
    pub types: TypeOptions,
    /// `db_charset` (architecture 6.4): an Oracle name or an `encoding_rs` label. Replaces the descriptor's
    /// set for VARCHAR2 and CHAR values, LONG values and single-byte CLOB values; the caller's
    /// responsibility, recorded in [`DumpInfo::db_charset_override`]. An unknown name is `E-CONFIG`.
    pub db_charset: Option<String>,
    /// `client_charset` (architecture 6.4): the set of names, DDL and the other text items, instead of the
    /// header's. Recorded in [`DumpInfo::client_charset_override`].
    pub client_charset: Option<String>,
    /// `strict` (architecture 7.1, owner Q-2): every warning outside the confirmed rules is an error. `open`
    /// fails on the header's warnings, `scan` on the walker's and the resolution's, `read_table` and the
    /// iteration on the reader's; `W-NUMBER-ROUNDED` becomes `E-NUMBER-OVERFLOW`, `W-LOB-UNTESTED`
    /// `E-LOB-UNCONFIRMED`, `W-CHARSET-UNSUPPORTED` `E-UNSUPPORTED-CHARSET`, every other `E-STRICT`;
    /// `W-NUMBER-NULLED` and `W-VALUE-NULLED` report a chosen policy and stay warnings. The walker's
    /// [E2a] decision follows `strict` too (ADR-22, ADR-23).
    pub strict: bool,
    /// `extra_intro_lines` (architecture 2.4): entries added to list K, each `kind:length:forms:text` (for
    /// example `suffix:15:plain: MODIFY DEFAULT`). One entry per string; an invalid one is `E-CONFIG`.
    pub extra_intro_lines: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            max_line_bytes: crate::exp::items::DEFAULT_MAX_LINE_BYTES,
            buffer_bytes: DEFAULT_BUFFER,
            types: TypeOptions::default(),
            db_charset: None,
            client_charset: None,
            strict: false,
            extra_intro_lines: Vec::new(),
        }
    }
}

/// Mode of the export [H3].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportMode {
    /// `RTABLES`.
    Tables,
    /// `RUSERS`.
    Users,
    /// Anything else.
    Other,
}

/// What the first record says [H1 to H5].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DumpInfo {
    /// Always `exp` in phase 1.
    pub format: &'static str,
    /// The version line, for example `EXPORT:V11.02.00`.
    pub version: String,
    /// The user (client set decoding; `user_possibly_damaged` as for names).
    pub user: String,
    /// The user text is possibly damaged (architecture 6.5).
    pub user_possibly_damaged: bool,
    /// Export mode.
    pub mode: ExportMode,
    /// RECORDLENGTH.
    pub record_length: u64,
    /// The export was started with `direct=y` [H2] (not the path of a table [T6]).
    pub direct_flag: bool,
    /// Character set id of the client [H1].
    pub client_charset_id: u16,
    /// Character set id of the database [H4].
    pub db_charset_id: u16,
    /// National character set id [H4].
    pub national_charset_id: u16,
    /// The client set differs from the database set (the ids of the header): names, comments and DDL were
    /// converted when the file was written and may hold `?` where the client set lacked a character [C2,
    /// architecture 6.5]. Row data are not affected [C1].
    pub text_converted_at_export: bool,
    /// The `db_charset` option as given, if any.
    pub db_charset_override: Option<String>,
    /// The `client_charset` option as given, if any.
    pub client_charset_override: Option<String>,
    /// Size of the file when it was opened.
    pub file_size: u64,
}

/// One column [T5], with its name from the `INSERT INTO` line [T3].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnInfo {
    /// 1-based position in the declared order (`CREATE TABLE`; the `INSERT INTO` order on a table fallback)
    /// [ADR-24].
    pub position: u32,
    /// 1-based position in the part: the order of the descriptors and of the row fields [T3].
    pub file_position: u32,
    /// Name, client set decoding.
    pub name: String,
    /// The name as written in the file.
    pub name_raw: Vec<u8>,
    /// The name is possibly damaged (architecture 6.5): a byte the client set does not allow, or a `?` /
    /// 0xbf in a text converted at export.
    pub name_possibly_damaged: bool,
    /// Type code of the descriptor.
    pub type_code: u16,
    /// Name of the Oracle type for the code (and form) [T5].
    pub oracle_type: &'static str,
    /// Maximum byte length of the descriptor.
    pub max_bytes: u16,
    /// Character set id (types 1 and 96).
    pub charset_id: Option<u16>,
    /// Form (types 1 and 96): 1 database set, 2 national set.
    pub form: Option<u16>,
    /// The declared type [T9], if the column definition was understood and agrees with the descriptor.
    pub declared: Option<DeclaredType>,
    /// The type text of the column definition as written (client set decoding), kept even when it is not
    /// understood; `None` on a table fallback.
    pub declared_text: Option<String>,
    /// ` NOT NULL ENABLE` stood in the column definition. Not used for the schema (every field is nullable).
    pub not_null: bool,
    /// The Arrow type (architecture 5.1).
    pub arrow_type: DataType,
    /// Where the type came from (ADR-33 item 4): the reader applies `on_number_overflow` to NUMBER columns
    /// that are `Default` or `Override`, not to `Declared` ones.
    pub type_source: TypeSource,
    /// The overflow policy of the `number_types` entry that fixed the type, if it names one (T-11): it wins
    /// over `ReadOptions.on_number_overflow`.
    pub overflow_policy: Option<OverflowPolicy>,
    /// For a LOB column (CLOB, NCLOB, BLOB): the counts of its values over all parts (counts only).
    pub lob: Option<LobStats>,
    /// For a LONG / LONG RAW column: the statistics of its values over all parts. `column` is the declared
    /// position; `first_zero_piece` is the first one of the first part that has one.
    pub long: Option<LongStats>,
}

/// The counts of the LOB values of one column (architecture 3.2 `LobStats`): values, empty, short, long,
/// stored bytes and the largest stored length.
pub type LobStats = LobSummary;

/// The path of a table: the path of its parts if they agree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TablePath {
    /// Every part has path flag 0.
    Conventional,
    /// Every part has path flag 1.
    Direct,
    /// The parts differ (a partitioned table whose partitions were exported in different ways).
    Mixed,
}

/// One binary part of a table: the whole data of an unpartitioned table, or one partition or
/// subpartition [T10, ADR-21].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartInfo {
    /// The partition (the enclosing composite partition of a subpartition), client set decoding; `None` for an
    /// unpartitioned table.
    pub partition: Option<String>,
    /// The name as written in the file (client character set).
    pub partition_raw: Option<Vec<u8>>,
    /// The partition name is possibly damaged (architecture 6.5).
    pub partition_possibly_damaged: bool,
    /// The subpartition, client set decoding; `None` unless a `SUBPARTITION` line names the part.
    pub subpartition: Option<String>,
    /// The name as written in the file.
    pub subpartition_raw: Option<Vec<u8>>,
    /// The subpartition name is possibly damaged (architecture 6.5).
    pub subpartition_possibly_damaged: bool,
    /// Conventional or direct [T6]: each part has its own flag.
    pub path: PathKind,
    /// Rows of the part, counted in the scan.
    pub num_rows: u64,
    /// Offset of the part's `INSERT INTO` line.
    pub insert_offset: u64,
    /// Offset of the first row (behind the path flag and the LOB column entries).
    pub rows_offset: u64,
    /// Offset behind the LF that closes the rows.
    pub rows_end_offset: u64,
    /// How the rows ended [E1, E2].
    pub end_marker: EndMarker,
    /// For each file position of this part (index, 0-based) the declared position (value, 0-based): the
    /// reader decodes the fields in file order and places them by this list (ADR-31). The identity for a part
    /// whose order is the declared order.
    pub file_order: Vec<u32>,
}

/// One table with data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableInfo {
    /// 1-based ordinal of the table section in the file.
    pub ordinal: u32,
    /// Name from the `TABLE "name"` line, client set decoding.
    pub name: String,
    /// The name as written in the file.
    pub name_raw: Vec<u8>,
    /// The name is possibly damaged (architecture 6.5).
    pub name_possibly_damaged: bool,
    /// The columns in declared order (architecture 5.5 step 5, ADR-24). The rows are in `file_position` order.
    pub columns: Vec<ColumnInfo>,
    /// The `CREATE TABLE` column list could not be used and every column is mapped from its descriptor
    /// (`W-DDL-FALLBACK` in the dump's warnings).
    pub ddl_fallback: bool,
    /// The path of the parts if they agree, else `Mixed`.
    pub path: TablePath,
    /// The binary parts in file order: one for an unpartitioned table (ADR-21).
    pub parts: Vec<PartInfo>,
    /// A `PARTITION` line, or a `SUBPARTITION` line in a `COMPARTITION`, was seen [T10].
    pub partitioned: bool,
    /// A `COMPARTITION` line was seen: composite partitioning [T10, ADR-21].
    pub subpartitioned: bool,
    /// Rows of the table: the sum over the parts.
    pub num_rows: u64,
    /// Offset of the `CREATE TABLE` line.
    pub create_offset: u64,
    /// Offset of the line that ended the section [T2].
    pub section_end_offset: u64,
    /// No row and a table or column name that is possibly damaged: the export may have failed [X2, architecture
    /// 6.5]; `W-POSSIBLY-INCOMPLETE` in the dump's warnings. A "possibly", never a claim.
    pub possibly_incomplete: bool,
    /// The `CREATE TABLE` line as written in the file.
    pub ddl_raw: Vec<u8>,
    /// The `CREATE TABLE` line, client set decoding.
    pub ddl: String,
    /// The DDL is possibly damaged (architecture 6.5).
    pub ddl_lossy: bool,
}

/// How the scan ended.
#[derive(Debug)]
pub enum CatalogEnd {
    /// At the second `EXIT` [E4].
    Complete,
    /// Stopped by an unsupported column type [X1] or layout [X5] (ADR-12); the tables before it are listed and
    /// readable.
    Stopped(Error),
}

/// The table in which the scan stopped [X1, X5, ADR-12], named from its `TABLE` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoppedTable {
    /// 1-based ordinal of the table section.
    pub ordinal: u32,
    /// Name from the `TABLE "name"` line, client set decoding.
    pub name: String,
    /// The name as written in the file.
    pub name_raw: Vec<u8>,
    /// The name is possibly damaged (architecture 6.5).
    pub name_possibly_damaged: bool,
    /// Offset of the `TABLE "name"` line.
    pub table_line_offset: u64,
}

/// The result of the scan.
#[derive(Debug)]
pub struct Catalog {
    /// Tables with data, in file order.
    pub tables: Vec<TableInfo>,
    /// Complete or stopped. A stop carries the column position and type code (`UnsupportedType`) or the
    /// offset of the layout (`UnsupportedLayout`) in its error.
    pub end: CatalogEnd,
    /// The table in which the scan stopped, if it stopped inside a table section.
    pub stopped_table: Option<StoppedTable>,
    /// Offset behind the second `EXIT` line (`Complete` only).
    pub end_offset: Option<u64>,
    /// Counts of items; the fields read by G1 are part of them (owner decision Q-2).
    pub item_counts: ItemCounts,
}

/// Which table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableRef<'a> {
    /// By name; two tables of one name give `AmbiguousTableName`.
    ByName(&'a str),
    /// By the ordinal of the section.
    ByOrdinal(u32),
}

/// The size, the modification time and (Unix) the device and inode of the file, as of `open`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Snapshot {
    size: u64,
    modified: Option<SystemTime>,
    /// Device and inode (Unix): a file that replaces the original within the clock tick, with the same
    /// size and time, still has another inode.
    file_id: Option<(u64, u64)>,
}

impl Snapshot {
    fn of(path: &Path) -> Result<Self> {
        let m = std::fs::metadata(path).map_err(Error::io)?;
        Ok(Self {
            size: m.len(),
            modified: m.modified().ok(),
            file_id: file_id(&m),
        })
    }
}

#[cfg(unix)]
fn file_id(m: &std::fs::Metadata) -> Option<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    Some((m.dev(), m.ino()))
}

#[cfg(not(unix))]
fn file_id(_: &std::fs::Metadata) -> Option<(u64, u64)> {
    None
}

/// The sets that decode text items (architecture 6.1, 6.4, 6.5).
#[derive(Debug, Clone, Copy)]
struct TextEnv {
    /// The set of names and DDL: the override, else the header's id; `None` if the reader cannot decode it.
    client: Option<Charset>,
    /// Client id differs from database id in the header.
    converted: bool,
}

impl TextEnv {
    /// Decodes a name or text; the flag is the "possibly damaged" of architecture 6.5.
    fn text(&self, raw: &[u8]) -> (String, bool) {
        let (s, replaced) = decode_text(self.client, raw);
        let hint = self.converted && has_replacement_char(self.client, raw);
        (s, replaced || hint)
    }
}

/// An opened dump.
#[derive(Debug)]
pub struct Dump {
    path: PathBuf,
    options: Options,
    env: TextEnv,
    db_override: Option<Charset>,
    intro: IntroList,
    snapshot: Snapshot,
    header: Header,
    info: DumpInfo,
    warnings: Vec<Warning>,
    catalog: Option<Catalog>,
}

/// Opens the file read-only, reads the first record and the first item (`METRICST` / `METRICSU`).
pub fn open(path: impl AsRef<Path>, mut options: Options) -> Result<Dump> {
    let path = path.as_ref().to_path_buf();
    let snapshot = Snapshot::of(&path)?;
    let f = File::open(&path).map_err(Error::io)?;
    let mut s = ByteStream::new(f, options.buffer_bytes.max(MIN_BUFFER_BYTES))?;
    let (header, mut warnings) = read_header(&mut s)?;
    warnings.extend(read_first_item(&mut s, &header)?);
    if options.strict {
        strict_check(&warnings)?;
    }
    let intro = IntroList::with_extra(&options.extra_intro_lines)?;
    check_number_overflow_keys(&options.types)?;
    let db_override = match &options.db_charset {
        None => None,
        Some(n) => Some(Charset::from_name(n).ok_or_else(|| {
            Error::new(
                ErrorCode::Config,
                "db_charset is neither an Oracle character set name nor a label of encoding_rs",
            )
        })?),
    };
    let client_override = match &options.client_charset {
        None => None,
        Some(n) => Some(Charset::from_name(n).ok_or_else(|| {
            Error::new(
                ErrorCode::Config,
                "client_charset is neither an Oracle character set name nor a label of encoding_rs",
            )
        })?),
    };
    let env = TextEnv {
        client: client_override.or_else(|| Charset::from_id(header.client_charset)),
        converted: header.client_charset != header.db_charset,
    };
    options.types.db_charset = db_override;
    let (user, user_possibly_damaged) = env.text(&header.user);
    let info = DumpInfo {
        format: "exp",
        version: env.text(&header.version).0,
        user,
        user_possibly_damaged,
        mode: match header.mode {
            Mode::Tables => ExportMode::Tables,
            Mode::Users => ExportMode::Users,
            Mode::Other(_) => ExportMode::Other,
        },
        record_length: header.record_length,
        direct_flag: header.direct_flag,
        client_charset_id: header.client_charset,
        db_charset_id: header.db_charset,
        national_charset_id: header.national_charset,
        text_converted_at_export: header.client_charset != header.db_charset,
        db_charset_override: options.db_charset.clone(),
        client_charset_override: options.client_charset.clone(),
        file_size: header.file_size,
    };
    Ok(Dump {
        path,
        options,
        env,
        db_override,
        intro,
        snapshot,
        header,
        info,
        warnings,
        catalog: None,
    })
}

/// `strict`: the first warning that has an error under `strict` is returned as that error.
fn strict_check(warnings: &[Warning]) -> Result<()> {
    match warnings.iter().find_map(Error::from_warning) {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

/// Every key of `number_overflow` names a `number_types` entry (the policy belongs to that entry).
fn check_number_overflow_keys(t: &TypeOptions) -> Result<()> {
    if t.number_overflow
        .keys()
        .all(|k| t.number_types.contains_key(k))
    {
        Ok(())
    } else {
        Err(Error::new(
            ErrorCode::Config,
            "an overflow policy is given for a column that has no number_types entry",
        ))
    }
}

/// ASCII decoding: a byte above 0x7f becomes U+FFFD. The flag says whether that happened.
pub fn ascii_text(raw: &[u8]) -> (String, bool) {
    let mut damaged = false;
    let s = raw
        .iter()
        .map(|&b| {
            if b.is_ascii() {
                char::from(b)
            } else {
                damaged = true;
                '\u{fffd}'
            }
        })
        .collect();
    (s, damaged)
}

fn oracle_type(code: u16, form: Option<u16>) -> &'static str {
    match (code, form) {
        (1, Some(2)) => "NVARCHAR2",
        (1, _) => "VARCHAR2",
        (2, _) => "NUMBER",
        (8, _) => "LONG",
        (12, _) => "DATE",
        (23, _) => "RAW",
        (24, _) => "LONG RAW",
        (69, _) => "ROWID",
        (96, Some(2)) => "NCHAR",
        (96, _) => "CHAR",
        (112, Some(2)) => "NCLOB",
        (112, _) => "CLOB",
        (113, _) => "BLOB",
        (180, _) => "TIMESTAMP",
        (181, _) => "TIMESTAMP WITH TIME ZONE",
        (182, _) => "INTERVAL YEAR TO MONTH",
        (183, _) => "INTERVAL DAY TO SECOND",
        (208, _) => "UROWID",
        (231, _) => "TIMESTAMP WITH LOCAL TIME ZONE",
        _ => "UNKNOWN",
    }
}

type NamedPart = (Option<String>, Option<Vec<u8>>, bool);

fn named(env: &TextEnv, line: &Option<PartitionLine>) -> NamedPart {
    match line {
        Some(l) => {
            let (name, damaged) = env.text(&l.name_raw);
            (Some(name), Some(l.name_raw.clone()), damaged)
        }
        None => (None, None, false),
    }
}

fn add_lob(total: &mut LobSummary, part: &LobSummary) {
    total.values = total.values.saturating_add(part.values);
    total.short = total.short.saturating_add(part.short);
    total.empty = total.empty.saturating_add(part.empty);
    total.long = total.long.saturating_add(part.long);
    total.stored_bytes = total.stored_bytes.saturating_add(part.stored_bytes);
    total.largest = total.largest.max(part.largest);
}

fn add_long(total: &mut LongStats, part: &LongStats) {
    total.values = total.values.saturating_add(part.values);
    total.chained = total.chained.saturating_add(part.chained);
    total.chunked = total.chunked.saturating_add(part.chunked);
    total.chunked_short = total.chunked_short.saturating_add(part.chunked_short);
    if total.first_chunked_short.is_none() {
        total.first_chunked_short = part.first_chunked_short;
    }
    total.largest = total.largest.max(part.largest);
    total.above_32000 = total.above_32000.saturating_add(part.above_32000);
    total.zero_pieces = total.zero_pieces.saturating_add(part.zero_pieces);
    if total.first_zero_piece.is_none() {
        total.first_zero_piece = part.first_zero_piece;
    }
    total.fff2_to_fff4 = total.fff2_to_fff4.saturating_add(part.fff2_to_fff4);
    if total.first_fff2_to_fff4.is_none() {
        total.first_fff2_to_fff4 = part.first_fff2_to_fff4;
    }
    total.sizes_65521_to_65534 = total
        .sizes_65521_to_65534
        .saturating_add(part.sizes_65521_to_65534);
}

fn table_info(s: Section, opts: &TypeOptions, env: &TextEnv) -> Option<(TableInfo, Vec<Warning>)> {
    let first = s.parts.first()?;
    let (name, name_possibly_damaged) = env.text(&s.name_raw);
    let (ddl, ddl_lossy) = env.text(&s.ddl_raw);
    // the types are resolved once per table, on the first part (ADR-31 item 3)
    let resolved = resolve_table(
        &ResolveInput {
            table_ordinal: s.ordinal,
            table_name: &name,
            ddl_raw: &s.ddl_raw,
            ddl_offset: s.create_offset,
            descriptors: &first.header.columns,
            insert_names: &first.insert.columns,
            path: first.header.path,
            client: env.client,
        },
        opts,
    );
    let mut warnings = resolved.warnings.clone();
    // the place of every field of every part in the declared order (ADR-31 item 1)
    let first_layout = PartLayout {
        descriptors: &first.header.columns,
        insert_names: &first.insert.columns,
        path: first.header.path,
        insert_offset: first.insert_offset,
    };
    let mut order_warned = resolved.order_warned;
    let mut parts = Vec::with_capacity(s.parts.len());
    for (k, p) in s.parts.iter().enumerate() {
        let file_order = if k == 0 {
            first_part_order(&resolved)
        } else {
            let layout = PartLayout {
                descriptors: &p.header.columns,
                insert_names: &p.insert.columns,
                path: p.header.path,
                insert_offset: p.insert_offset,
            };
            let po = part_order(
                &resolved,
                &first_layout,
                &layout,
                s.ordinal,
                u32::try_from(k.saturating_add(1)).unwrap_or(u32::MAX),
                !order_warned,
            )?;
            if let Some(w) = po.warning {
                order_warned = true;
                warnings.push(w);
            }
            po.file_order
        };
        let (partition, partition_raw, partition_possibly_damaged) = named(env, &p.partition);
        let (subpartition, subpartition_raw, subpartition_possibly_damaged) =
            named(env, &p.subpartition);
        parts.push(PartInfo {
            partition,
            partition_raw,
            partition_possibly_damaged,
            subpartition,
            subpartition_raw,
            subpartition_possibly_damaged,
            path: p.header.path,
            num_rows: p.rows.rows,
            insert_offset: p.insert_offset,
            rows_offset: p.header.rows_offset,
            rows_end_offset: p.rows.rows_end_offset,
            end_marker: p.rows.end_marker,
            file_order,
        });
    }
    let mut columns = Vec::with_capacity(resolved.columns.len());
    for (i, r) in resolved.columns.iter().enumerate() {
        let d = first.header.columns.get(r.file_index)?;
        let name_raw = first.insert.columns.get(r.file_index)?;
        let (col_name, col_damaged) = env.text(name_raw);
        // statistics over the parts: a part's column is found through its own `file_order`
        let declared = u32::try_from(i).unwrap_or(u32::MAX);
        let mut lob: Option<LobStats> = None;
        let mut long: Option<LongStats> = None;
        for (p, info) in s.parts.iter().zip(&parts) {
            let file_index = info.file_order.iter().position(|&x| x == declared)?;
            let file_position = u32::try_from(file_index.saturating_add(1)).unwrap_or(u32::MAX);
            if let Some(c) = p
                .rows
                .lob_columns
                .iter()
                .find(|c| c.column == file_position)
            {
                add_lob(lob.get_or_insert_with(LobSummary::default), &c.stats);
            }
            if let Some(c) = p.rows.long.iter().find(|c| c.column == file_position) {
                let total = long.get_or_insert_with(|| LongStats {
                    column: declared.saturating_add(1),
                    ..LongStats::default()
                });
                add_long(total, c);
            }
        }
        columns.push(ColumnInfo {
            position: declared.saturating_add(1),
            file_position: u32::try_from(r.file_index.saturating_add(1)).unwrap_or(u32::MAX),
            name: col_name,
            name_raw: name_raw.clone(),
            name_possibly_damaged: col_damaged,
            type_code: d.type_code,
            oracle_type: oracle_type(d.type_code, d.form),
            max_bytes: d.max_len,
            charset_id: d.charset_id,
            form: d.form,
            declared: r.declared,
            declared_text: r.type_text.as_deref().map(|t| env.text(t).0),
            not_null: r.not_null,
            arrow_type: r.arrow_type.clone(),
            type_source: r.type_source,
            overflow_policy: r.overflow,
            lob,
            long,
        });
    }
    let conv = parts
        .iter()
        .filter(|p| p.path == PathKind::Conventional)
        .count();
    let path = if conv == parts.len() {
        TablePath::Conventional
    } else if conv == 0 {
        TablePath::Direct
    } else {
        TablePath::Mixed
    };
    let num_rows = parts.iter().fold(0u64, |a, p| a.saturating_add(p.num_rows));
    // X2: a table that looks empty and has a replacement character in a name may be a failed export
    let possibly_incomplete =
        num_rows == 0 && (name_possibly_damaged || columns.iter().any(|c| c.name_possibly_damaged));
    if possibly_incomplete {
        warnings.push(
            Warning::new(
                WarningCode::PossiblyIncomplete,
                "no rows and a possibly damaged name: the export of this table may have failed",
                s.create_offset,
            )
            .in_table(s.ordinal),
        );
    }
    let info = TableInfo {
        ordinal: s.ordinal,
        name,
        name_raw: s.name_raw,
        name_possibly_damaged,
        columns,
        ddl_fallback: resolved.fallback,
        path,
        parts,
        partitioned: s.partitioned,
        subpartitioned: s.subpartitioned,
        num_rows,
        create_offset: s.create_offset,
        section_end_offset: s.end_offset,
        possibly_incomplete,
        ddl_raw: s.ddl_raw,
        ddl,
        ddl_lossy,
    };
    Some((info, warnings))
}

impl Dump {
    /// The first record.
    pub fn info(&self) -> &DumpInfo {
        &self.info
    }

    /// The header with every field (raw texts included).
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// The path given to `open`.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Warnings of the header and, after the scan, of the walk.
    pub fn warnings(&self) -> &[Warning] {
        &self.warnings
    }

    /// Fails with `FileChanged` if the file is not the one that was opened.
    pub fn check_unchanged(&self) -> Result<()> {
        let now = Snapshot::of(&self.path)?;
        if now == self.snapshot {
            Ok(())
        } else {
            Err(Error::new(
                ErrorCode::FileChanged,
                "size or modification time of the file differ from the file that was opened",
            )
            .at(0)
            .with_length(now.size))
        }
    }

    /// The catalog if the scan has run.
    pub fn catalog(&self) -> Option<&Catalog> {
        self.catalog.as_ref()
    }

    /// Walks the whole file once (cached). Decodes no value.
    pub fn scan(&mut self) -> Result<&Catalog> {
        if self.catalog.is_none() {
            self.check_unchanged()?;
            let opts = WalkOptions {
                max_line_bytes: self.options.max_line_bytes,
                buffer_bytes: self.options.buffer_bytes,
                intro_list: Some(self.intro.clone()),
                strict_e2a: self.options.strict,
                ..WalkOptions::default()
            };
            let env = self.env;
            let report = walk_path(&self.path, &opts)?;
            // The walk reads the header again: its warnings are those of `open`, already in the list.
            // The list is committed only if the scan succeeds (under `strict` a warning fails it).
            let already = self.warnings.len();
            let mut new_warnings: Vec<Warning> =
                report.warnings.into_iter().skip(already).collect();
            let end = match report.end {
                WalkEnd::Complete => CatalogEnd::Complete,
                WalkEnd::Stopped(e) => CatalogEnd::Stopped(e),
            };
            let stopped_table = report.stopped_section.map(|s| {
                let (name, name_possibly_damaged) = env.text(&s.name_raw);
                StoppedTable {
                    ordinal: s.ordinal,
                    name,
                    name_raw: s.name_raw,
                    name_possibly_damaged,
                    table_line_offset: s.table_line_offset,
                }
            });
            let mut tables = Vec::new();
            for section in report.sections {
                if let Some((info, warnings)) = table_info(section, &self.options.types, &env) {
                    new_warnings.extend(warnings);
                    tables.push(info);
                }
            }
            if self.options.strict {
                strict_check(&new_warnings)?;
            }
            self.warnings.extend(new_warnings);
            self.catalog = Some(Catalog {
                tables,
                end,
                stopped_table,
                end_offset: report.end_offset,
                item_counts: report.item_counts,
            });
        }
        self.catalog
            .as_ref()
            .ok_or_else(|| Error::new(ErrorCode::Config, "catalog missing after the scan"))
    }

    /// The tables of the catalog.
    pub fn tables(&mut self) -> Result<&[TableInfo]> {
        Ok(&self.scan()?.tables)
    }

    /// Finds one table.
    pub fn table(&mut self, which: TableRef<'_>) -> Result<&TableInfo> {
        let tables = &self.scan()?.tables;
        let mut found = tables.iter().filter(|t| match which {
            TableRef::ByName(n) => t.name == n,
            TableRef::ByOrdinal(o) => t.ordinal == o,
        });
        match (found.next(), found.next()) {
            (Some(t), None) => Ok(t),
            (Some(_), Some(_)) => Err(Error::new(
                ErrorCode::AmbiguousTableName,
                "more than one table has this name",
            )),
            (None, _) => Err(Error::new(ErrorCode::NoSuchTable, "no such table")),
        }
    }

    /// The batches of one table, all parts in file order (architecture 3.2 `read_table`): re-opens the file,
    /// refuses with `FileChanged` if it is not the file that was scanned. The value options (`on_value_error`,
    /// `on_number_overflow`, `max_lob_bytes`) are in `options`.
    pub fn read_table(
        &mut self,
        which: TableRef<'_>,
        options: &ReadOptions,
    ) -> Result<TableBatches> {
        let table = self.table(which)?.clone();
        self.check_unchanged()?;
        TableBatches::new(&ReaderInit {
            path: &self.path,
            buffer_bytes: self.options.buffer_bytes,
            max_line: self.options.max_line_bytes,
            table: &table,
            db_charset_id: self.info.db_charset_id,
            db_override: self.db_override,
            intro: &self.intro,
            strict: self.options.strict,
            options,
        })
    }

    /// A row cursor over the whole data of one table that has exactly one part (every unpartitioned table, and
    /// a partitioned one with one part). A table of more than one part is `E-CONFIG` (`length` = the number of
    /// parts, ADR-32 item 3): use [`Self::part_rows`] or `read_table`. Re-opens the file, refuses with `FileChanged` if it is not the file that was
    /// scanned, and starts at the part's `rows_offset`. It owns its file handle.
    pub fn rows(&mut self, which: TableRef<'_>) -> Result<TableRows> {
        let info = self.table(which)?;
        let parts = info.parts.len();
        if parts != 1 {
            return Err(Error::new(
                ErrorCode::Config,
                "the table has more than one part; use part_rows or read_table",
            )
            .in_table(info.ordinal)
            .with_length(u64::try_from(parts).unwrap_or(u64::MAX)));
        }
        self.part_rows(which, 0)
    }

    /// A row cursor over one part (0-based index into `TableInfo.parts`) of one table.
    pub fn part_rows(&mut self, which: TableRef<'_>, part: usize) -> Result<TableRows> {
        let info = self.table(which)?;
        let ordinal = info.ordinal;
        let p = info.parts.get(part).ok_or_else(|| {
            Error::new(ErrorCode::Config, "the table has no such part")
                .in_table(ordinal)
                .with_length(u64::try_from(info.parts.len()).unwrap_or(u64::MAX))
        })?;
        let rows_offset = p.rows_offset;
        // the row fields follow the file order of the part, the catalog the declared order (ADR-24, ADR-31)
        let mut in_file_order: Vec<(u32, &ColumnInfo)> = Vec::with_capacity(info.columns.len());
        for (declared, c) in info.columns.iter().enumerate() {
            let declared = u32::try_from(declared).unwrap_or(u32::MAX);
            if let Some(file_index) = p.file_order.iter().position(|&x| x == declared) {
                in_file_order.push((u32::try_from(file_index).unwrap_or(u32::MAX), c));
            }
        }
        in_file_order.sort_by_key(|(f, _)| *f);
        let cursor = RowCursor::from_columns(
            p.path,
            in_file_order
                .iter()
                .map(|(_, c)| (c.type_code, c.max_bytes)),
            ordinal,
        );
        let part_no = u32::try_from(part.saturating_add(1)).unwrap_or(u32::MAX);
        self.check_unchanged()?;
        let f = File::open(&self.path).map_err(Error::io)?;
        let mut s = ByteStream::new(f, self.options.buffer_bytes.max(MIN_BUFFER_BYTES))?;
        s.seek_to(rows_offset)?;
        Ok(TableRows {
            stream: s,
            cursor,
            intro: self.intro.clone(),
            strict_e2a: self.options.strict,
            max_line: self.options.max_line_bytes,
            part: part_no,
            end: None,
        })
    }
}

/// Reads the rows of one table, row by row, skipping every value by its length (decoding is the job of
/// [`crate::reader::TableBatches`]; [`TableRows::next_row_with`] lends the field bytes to a sink).
#[derive(Debug)]
pub struct TableRows {
    stream: ByteStream<File>,
    cursor: RowCursor,
    intro: IntroList,
    strict_e2a: bool,
    max_line: usize,
    part: u32,
    end: Option<crate::exp::rows::RowsEnd>,
}

impl TableRows {
    /// The offset where the next row starts (or where the end marker stands).
    pub fn offset(&self) -> u64 {
        self.stream.offset()
    }

    /// Rows read so far.
    pub fn rows_read(&self) -> u64 {
        self.cursor.rows()
    }

    /// Reads the next row without decoding it; returns the offset where it started, `None` at the end of
    /// the rows. After `None` the stream stands behind the LF that closes the rows ([`Self::end`]).
    pub fn next_row(&mut self) -> Result<Option<u64>> {
        if self.end.is_some() {
            return Ok(None);
        }
        let env = Env {
            intro: &self.intro,
            max_line: self.max_line,
            strict_e2a: self.strict_e2a,
        };
        let step = self
            .cursor
            .step(&mut self.stream, None, &env)
            .map_err(|e| e.in_part(self.part))?;
        match step {
            Step::Row(at) => Ok(Some(at)),
            Step::End(e) => {
                self.end = Some(e);
                Ok(None)
            }
        }
    }

    /// As [`Self::next_row`], and gives every column field of the row to `sink` (the bytes of the fields, in
    /// the file order of the part; the sink sees the file position of each column). Used by tests that check
    /// the field bytes the row decoder sees (N-13).
    pub fn next_row_with(&mut self, sink: &mut dyn RowSink) -> Result<Option<u64>> {
        if self.end.is_some() {
            return Ok(None);
        }
        let env = Env {
            intro: &self.intro,
            max_line: self.max_line,
            strict_e2a: self.strict_e2a,
        };
        let step = self
            .cursor
            .step_with(&mut self.stream, None, Some(sink), &env)
            .map_err(|e| e.in_part(self.part))?;
        match step {
            Step::Row(at) => Ok(Some(at)),
            Step::End(e) => {
                self.end = Some(e);
                Ok(None)
            }
        }
    }

    /// How the rows ended, once `next_row` has returned `None`.
    pub fn end(&self) -> Option<&crate::exp::rows::RowsEnd> {
        self.end.as_ref()
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;
    use crate::exp::insert::InsertLine;
    use crate::exp::items::Part;
    use crate::exp::rows::{LobColumn, RowsSummary};
    use crate::exp::table::{ColumnDesc, TableHeader};

    fn desc(type_code: u16, max_len: u16) -> ColumnDesc {
        let wide = type_code == 112;
        ColumnDesc {
            offset: 0,
            type_code,
            max_len,
            charset_id: wide.then_some(873),
            form: wide.then_some(1),
        }
    }

    fn lob(values: u64, long: u64, largest: u64) -> LobSummary {
        LobSummary {
            values,
            short: values - long,
            empty: 0,
            long,
            stored_bytes: largest * values,
            largest,
        }
    }

    fn long_stats(column: u32, values: u64, largest: u64, zero: Option<(u64, u64)>) -> LongStats {
        LongStats {
            column,
            values,
            chained: values / 2,
            largest,
            above_32000: u64::from(largest > 32_000),
            zero_pieces: u64::from(zero.is_some()),
            first_zero_piece: zero,
            ..LongStats::default()
        }
    }

    /// A part of table `T (ID NUMBER(5,0), L LONG, C CLOB)`; `order` gives the columns in file order.
    fn part(order: [&str; 3], path: PathKind, rows: RowsSummary, at: u64) -> Part {
        let d = |n: &str| match n {
            "ID" => desc(2, 22),
            "L" => desc(8, 0),
            _ => desc(112, 72),
        };
        Part {
            insert_offset: at,
            insert: InsertLine {
                table: b"T".to_vec(),
                columns: order.iter().map(|n| n.as_bytes().to_vec()).collect(),
                placeholders: 3,
            },
            header: TableHeader {
                descriptors_offset: at,
                columns: order.iter().map(|n| d(n)).collect(),
                m_offset: at,
                m: 0,
                flag_offset: at,
                path,
                lob_entries: Vec::new(),
                odd_entry_tail: None,
                rows_offset: at + 100,
            },
            rows,
            partition: None,
            subpartition: None,
        }
    }

    fn rows(n: u64, long: Vec<LongStats>, lob_cols: Vec<LobColumn>) -> RowsSummary {
        RowsSummary {
            rows: n,
            fields: n * 3,
            end_marker: EndMarker::FfFf,
            end_marker_offset: 0,
            rows_end_offset: 0,
            nulls: 0,
            raw_sha256: None,
            lob: LobSummary::default(),
            lob_detail: None,
            e2a_no_rows: false,
            long,
            lob_zero_total: None,
            lob_columns: lob_cols,
            lob_sha256: None,
            long_sha256: Vec::new(),
        }
    }

    #[test]
    fn column_info_long_and_lob_are_the_sums_over_the_parts_found_through_each_file_order() {
        // part 1 (conventional): ID, L, C = file positions 1, 2, 3; part 2 (direct): ID, C, L = 1, 2, 3 with the
        // LONG column last, so L is file position 2 in part 1 and 3 in part 2
        let p1 = part(
            ["ID", "L", "C"],
            PathKind::Conventional,
            rows(
                4,
                vec![long_stats(2, 4, 100, None)],
                vec![LobColumn {
                    column: 3,
                    stats: lob(3, 1, 500),
                }],
            ),
            10,
        );
        let p2 = part(
            ["ID", "C", "L"],
            PathKind::Direct,
            rows(
                6,
                vec![long_stats(3, 5, 40_000, Some((2, 99)))],
                vec![LobColumn {
                    column: 2,
                    stats: lob(2, 2, 900),
                }],
            ),
            1000,
        );
        let section = Section {
            ordinal: 7,
            table_line_offset: 1,
            name_raw: b"T".to_vec(),
            create_offset: 2,
            end_offset: 3,
            ddl_raw:
                b"CREATE TABLE \"T\" (\"ID\" NUMBER(5, 0), \"L\" LONG, \"C\" CLOB)  PCTFREE 10"
                    .to_vec(),
            parts: vec![p1, p2],
            partitioned: true,
            subpartitioned: false,
        };
        let (t, warnings) = table_info(
            section,
            &TypeOptions::default(),
            &TextEnv {
                client: Some(Charset::Utf8),
                converted: false,
            },
        )
        .unwrap();
        assert!(warnings.is_empty(), "{warnings:?}");
        assert!(!t.ddl_fallback);
        assert_eq!(t.path, TablePath::Mixed);
        assert_eq!(t.num_rows, 10);
        assert_eq!(t.parts[0].file_order, [0, 1, 2]);
        assert_eq!(t.parts[1].file_order, [0, 2, 1]);
        let l = t.columns[1].long.as_ref().unwrap();
        assert_eq!(
            (
                l.column,
                l.values,
                l.chained,
                l.largest,
                l.above_32000,
                l.zero_pieces
            ),
            (2, 9, 2 + 2, 40_000, 1, 1)
        );
        assert_eq!(l.first_zero_piece, Some((2, 99)));
        let c = t.columns[2].lob.as_ref().unwrap();
        assert_eq!(
            (c.values, c.short, c.long, c.largest, c.stored_bytes),
            (5, 2, 3, 900, 3 * 500 + 2 * 900)
        );
        assert!(t.columns[0].long.is_none() && t.columns[0].lob.is_none());
        assert!(t.columns[1].lob.is_none() && t.columns[2].long.is_none());
    }
}
