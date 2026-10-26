//! Layer 3, the item walker: from the first item at R + 0x104 to the second of two consecutive `EXIT`
//! lines [chapter 9 of the rules file]. It reads every length field and skips every value; it decodes
//! nothing and counts nothing the reader depends on [S7, R6].
//!
//! Procedure (architecture 2.5): a line that matches list K is followed by exactly one field in a form
//! the entry allows [I6]; any other line is looked at with G1 and, if a field follows, the field is
//! read and `W-UNKNOWN-INTRO` is raised. `TABLE "name"` directly followed by `CREATE TABLE` opens a
//! section [T1]; it ends at the next `TABLE "`, `ENDTABLE` or `METRICS...` line [T2]. `INSERT INTO`
//! inside a section starts a binary part [T3 to T6]; a partitioned table has one per `PARTITION "p"` line
//! or per `SUBPARTITION "s"` line inside a `COMPARTITION "p"` [T10, ADR-21]; elsewhere it is `Framing`.

use std::io::{Read, Seek};

use super::framing::{self, FieldForm};
use super::header::{read_first_item, read_header, Header};
use super::insert::{parse_insert_line, InsertLine};
use super::intro::IntroList;
use super::rows::{walk_rows, Env, LobSummary, RowsSummary};
use super::table::{is_lob_type, read_table_header, PathKind, TableHeader};
use crate::error::{Error, ErrorCode, Result, Warning, WarningCode};
use crate::io::{ByteStream, DEFAULT_BUFFER};

/// Default longest line (architecture 2.6).
pub const DEFAULT_MAX_LINE_BYTES: usize = 1 << 20;

/// Options of the walk.
#[derive(Debug, Clone)]
pub struct WalkOptions {
    /// A longer line is `Framing`.
    pub max_line_bytes: usize,
    /// List K; `None` is the embedded list. An empty list reads every field through G1.
    pub intro_list: Option<IntroList>,
    /// Keep one record per item in `WalkReport::items` (for tests; memory grows with the file).
    pub record_items: bool,
    /// Size of the I/O buffer.
    pub buffer_bytes: usize,
    /// Hash the raw fields of every table (`RowsSummary::raw_sha256`, task T-2).
    pub hash_fields: bool,
    /// Keep k of every row and every LOB value of every part (`RowsSummary::lob_detail`; for tests).
    pub record_lob: bool,
    /// ADR-22 under `strict`: with a first column of length 0 on the direct path, "text follows" alone is
    /// `Framing` (the session 2 rule) instead of the decision of [E2a]. (Task T-11 sets it from `strict`.)
    pub strict_e2a: bool,
}

impl Default for WalkOptions {
    fn default() -> Self {
        Self {
            max_line_bytes: DEFAULT_MAX_LINE_BYTES,
            intro_list: None,
            record_items: false,
            buffer_bytes: DEFAULT_BUFFER,
            hash_fields: false,
            record_lob: false,
            strict_e2a: false,
        }
    }
}

/// Kind of a recorded item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    /// A line.
    Line,
    /// A field in the given form.
    Field(FieldForm),
}

/// One item of the text region (only recorded with `record_items`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemRecord {
    /// Offset of the first byte.
    pub offset: u64,
    /// Offset behind the item (behind its LF).
    pub end: u64,
    /// Line or field.
    pub kind: ItemKind,
}

/// Fields read after the lines of one entry of list K, by form.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntryCount {
    /// The entry text (ASCII, lossless).
    pub text: String,
    /// Plain fields.
    pub plain: u64,
    /// Fields in pieces after `fd ff`.
    pub marked: u64,
    /// Pieces without marker.
    pub unmarked: u64,
}

impl EntryCount {
    /// All fields of the entry.
    pub fn total(&self) -> u64 {
        self.plain
            .saturating_add(self.marked)
            .saturating_add(self.unmarked)
    }
}

/// Items by kind, for diagnostics only; nothing in the reader depends on them (S7). The number of fields
/// read by G1 is part of it, so that a skipped item cannot go unnoticed (owner decision Q-2).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemCounts {
    /// Lines, `EXIT` lines and `INSERT INTO` lines included.
    pub lines: u64,
    /// One count per entry of list K, in the order of the list.
    pub per_entry: Vec<EntryCount>,
    /// Fields read through G1 after a line that is not in list K.
    pub g1_fields: u64,
}

impl ItemCounts {
    /// Fields of all kinds.
    pub fn fields(&self) -> u64 {
        self.per_entry
            .iter()
            .map(EntryCount::total)
            .fold(self.g1_fields, u64::saturating_add)
    }
}

/// A table section [T1].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    /// 1-based ordinal.
    pub ordinal: u32,
    /// Offset of the `TABLE "name"` line.
    pub table_line_offset: u64,
    /// Name as written in the `TABLE` line (client character set, not decoded; task T-3).
    pub name_raw: Vec<u8>,
    /// Offset of the `CREATE TABLE` line.
    pub create_offset: u64,
    /// Offset of the line that ended the section [T2] (or of the first `EXIT`).
    pub end_offset: u64,
    /// The `CREATE TABLE` line without its LF (client character set, not decoded; task T-3).
    pub ddl_raw: Vec<u8>,
    /// The binary parts in file order: none for a section without an `INSERT INTO` line, one for an
    /// ordinary table, one per partition for a partitioned table [T10].
    pub parts: Vec<Part>,
    /// A naming line or a `COMPARTITION "p"` line was seen in the section [T10, ADR-21].
    pub partitioned: bool,
    /// A `COMPARTITION "p"` line was seen in the section (composite partitioning) [T10, ADR-21].
    pub subpartitioned: bool,
}

/// A `PARTITION "p"`, `COMPARTITION "p"` or `SUBPARTITION "s"` line [T10].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionLine {
    /// Offset of the line.
    pub offset: u64,
    /// The name between the quotes, as written (client character set).
    pub name_raw: Vec<u8>,
}

/// One binary part of a section: the `INSERT INTO` line, descriptors, rows [T3 to T6, T10].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    /// Offset of the `INSERT INTO` line.
    pub insert_offset: u64,
    /// The `INSERT INTO` line: table name and column names [T3].
    pub insert: InsertLine,
    /// Descriptors and path.
    pub header: TableHeader,
    /// Rows and their end.
    pub rows: RowsSummary,
    /// The last `PARTITION "p"` line before the part, or the `COMPARTITION "p"` line that encloses it (a
    /// composite partition has no part of its own), if the section has any [ADR-21 item 6].
    pub partition: Option<PartitionLine>,
    /// The `SUBPARTITION "s"` line before the part, inside a `COMPARTITION`; `None` otherwise.
    pub subpartition: Option<PartitionLine>,
}

/// How the walk ended.
#[derive(Debug)]
pub enum WalkEnd {
    /// At the second `EXIT` [E4].
    Complete,
    /// Stopped by an unsupported column type [X1] or layout [X5] (ADR-12): the sections before it are listed.
    Stopped(Error),
}

/// The table section in which the walk stopped [X1, X5]: named from its `TABLE "name"` line, because the
/// error itself carries only the ordinal (no name from the file in an error).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoppedSection {
    /// 1-based ordinal of the section.
    pub ordinal: u32,
    /// Offset of the `TABLE "name"` line.
    pub table_line_offset: u64,
    /// Name as written in the `TABLE` line (client character set).
    pub name_raw: Vec<u8>,
}

/// The structure of a file.
#[derive(Debug)]
pub struct WalkReport {
    /// The header [H1 to H5].
    pub header: Header,
    /// Closed sections, in file order. A section stopped by X1 or X5 is not among them.
    pub sections: Vec<Section>,
    /// Counts of items.
    pub item_counts: ItemCounts,
    /// Warnings (header and walk).
    pub warnings: Vec<Warning>,
    /// Complete or stopped.
    pub end: WalkEnd,
    /// The section that was open when the walk stopped (`end` is `Stopped` and an error inside a section).
    pub stopped_section: Option<StoppedSection>,
    /// Offset behind the second `EXIT` LF (`Complete` only).
    pub end_offset: Option<u64>,
    /// Every item, if `record_items` was set.
    pub items: Vec<ItemRecord>,
}

impl WalkReport {
    /// All binary parts of all sections, in file order.
    pub fn parts(&self) -> impl Iterator<Item = &Part> {
        self.sections.iter().flat_map(|s| s.parts.iter())
    }

    /// Binary parts without rows, on the (direct, conventional) path.
    pub fn empty_tables(&self) -> (u64, u64) {
        let mut direct = 0u64;
        let mut conventional = 0u64;
        for d in self.parts() {
            if d.rows.rows == 0 {
                match d.header.path {
                    PathKind::Direct => direct = direct.saturating_add(1),
                    PathKind::Conventional => conventional = conventional.saturating_add(1),
                }
            }
        }
        (direct, conventional)
    }

    /// The LOB value counts of all parts added up.
    pub fn lob_totals(&self) -> LobSummary {
        let mut t = LobSummary::default();
        for p in self.parts() {
            let l = &p.rows.lob;
            t.values = t.values.saturating_add(l.values);
            t.short = t.short.saturating_add(l.short);
            t.empty = t.empty.saturating_add(l.empty);
            t.long = t.long.saturating_add(l.long);
            t.stored_bytes = t.stored_bytes.saturating_add(l.stored_bytes);
            t.largest = t.largest.max(l.largest);
        }
        t
    }
}

/// A section being read.
struct Open {
    ordinal: u32,
    table_line_offset: u64,
    name_raw: Vec<u8>,
    create_offset: u64,
    ddl_raw: Vec<u8>,
    parts: Vec<Part>,
    /// The last `PARTITION` line, or the `COMPARTITION` line that is open or was the last one [T10].
    partition: Option<PartitionLine>,
    /// The last `SUBPARTITION` line that named a part.
    subpartition: Option<PartitionLine>,
    /// A naming line was seen: `PARTITION "p"`, or `SUBPARTITION "s"` while a `COMPARTITION` is open.
    naming_seen: bool,
    partition_seen: bool,
    compartition_seen: bool,
    /// A `COMPARTITION` is open (no `ENDCOMPARTITION` yet).
    com_open: bool,
    /// Binary parts read since the open `COMPARTITION` line.
    parts_in_com: u64,
    endpartition_seen: bool,
    /// `W-UNTESTED-LAYOUT` for the order of the partition lines was raised for this section.
    layout_warned: bool,
    /// `W-UNTESTED-LAYOUT` for the LOB column entries was raised for this section.
    entry_warned: bool,
    /// LONG columns for which the zero-length piece warning was raised in this section.
    zero_piece_warned: Vec<u32>,
    /// Columns already warned about a chunked conventional LONG below 65,534 bytes (T-19, T-19b).
    chunked_short_warned: Vec<u32>,
    /// Columns already warned about a conventional length of 0xfff2 to 0xfff4 (T-19b, ADR-101 item 1).
    fff2_warned: Vec<u32>,
    /// `W-UNTESTED-LAYOUT` for a long LOB value of total 0 was raised for this section.
    lob_zero_warned: bool,
}

impl Open {
    fn new(
        ordinal: u32,
        table_line_offset: u64,
        name_raw: Vec<u8>,
        create: u64,
        ddl: &[u8],
    ) -> Self {
        Self {
            ordinal,
            table_line_offset,
            name_raw,
            create_offset: create,
            ddl_raw: ddl.to_vec(),
            parts: Vec::new(),
            partition: None,
            subpartition: None,
            naming_seen: false,
            partition_seen: false,
            compartition_seen: false,
            com_open: false,
            parts_in_com: 0,
            endpartition_seen: false,
            layout_warned: false,
            entry_warned: false,
            zero_piece_warned: Vec::new(),
            chunked_short_warned: Vec::new(),
            fff2_warned: Vec::new(),
            lob_zero_warned: false,
        }
    }
}

struct Walker<'a, R: Read + Seek> {
    s: &'a mut ByteStream<R>,
    intro: IntroList,
    max_line: usize,
    record: bool,
    hash: bool,
    record_lob: bool,
    strict_e2a: bool,
    /// The previous line was a naming line (`PARTITION "p"`, or `SUBPARTITION "s"` in an open `COMPARTITION`).
    prev_naming: bool,
    sections: Vec<Section>,
    counts: ItemCounts,
    warnings: Vec<Warning>,
    items: Vec<ItemRecord>,
    end_offset: Option<u64>,
    cur: Option<Open>,
    /// A `TABLE "name"` line whose next item has not been seen: (line offset, name).
    pending: Option<(u64, Vec<u8>)>,
    ordinal: u32,
}

const TABLE_PREFIX: &[u8] = b"TABLE \"";
const CREATE_TABLE: &[u8] = b"CREATE TABLE ";
const INSERT_INTO: &[u8] = b"INSERT INTO ";
const EXIT: &[u8] = b"EXIT";
const PARTITION_PREFIX: &[u8] = b"PARTITION \"";
const ENDPARTITION: &[u8] = b"ENDPARTITION";
const COMPARTITION_PREFIX: &[u8] = b"COMPARTITION \"";
const SUBPARTITION_PREFIX: &[u8] = b"SUBPARTITION \"";
const ENDCOMPARTITION: &[u8] = b"ENDCOMPARTITION";

impl<R: Read + Seek> Walker<'_, R> {
    fn note(&mut self, offset: u64, end: u64, kind: ItemKind) {
        if self.record {
            self.items.push(ItemRecord { offset, end, kind });
        }
    }

    fn close_section(&mut self, end_offset: u64) {
        if let Some(mut o) = self.cur.take() {
            if o.com_open {
                layout_warning(
                    &mut o,
                    &mut self.warnings,
                    end_offset,
                    "a section ends inside a COMPARTITION (no ENDCOMPARTITION)",
                );
            }
            if (o.naming_seen || o.compartition_seen) && !o.endpartition_seen {
                layout_warning(
                    &mut o,
                    &mut self.warnings,
                    end_offset,
                    "a section with partition lines ends without ENDPARTITION",
                );
            }
            self.sections.push(Section {
                ordinal: o.ordinal,
                table_line_offset: o.table_line_offset,
                name_raw: o.name_raw,
                create_offset: o.create_offset,
                ddl_raw: o.ddl_raw,
                end_offset,
                parts: o.parts,
                partitioned: o.naming_seen || o.compartition_seen,
                subpartitioned: o.compartition_seen,
            });
        }
    }

    fn run(&mut self) -> Result<()> {
        let mut line = Vec::new();
        let mut exits = 0u8;
        loop {
            let item_off = self.s.offset();
            match self.s.peek(1)?.first() {
                None => {
                    return Err(Error::new(
                        ErrorCode::Truncated,
                        "file ends before the second EXIT line",
                    )
                    .at(item_off));
                }
                Some(0) => {
                    return Err(Error::new(
                        ErrorCode::Truncated,
                        "zero fill before the second EXIT line",
                    )
                    .at(item_off));
                }
                Some(_) => {}
            }
            self.s.read_line(&mut line, self.max_line)?;
            self.counts.lines = self.counts.lines.saturating_add(1);
            self.note(item_off, self.s.offset(), ItemKind::Line);

            // T1: the item after a `TABLE "name"` line decides whether a section opens.
            if let Some((table_off, name)) = self.pending.take() {
                if line.starts_with(CREATE_TABLE) {
                    self.ordinal = self.ordinal.saturating_add(1);
                    self.cur = Some(Open::new(self.ordinal, table_off, name, item_off, &line));
                }
            }
            // T2: where a section ends.
            if line.starts_with(TABLE_PREFIX) || line == b"ENDTABLE" || line.starts_with(b"METRICS")
            {
                self.close_section(item_off);
            }
            if line.starts_with(TABLE_PREFIX) {
                let rest = line.get(TABLE_PREFIX.len()..).unwrap_or(&[]);
                let name = rest.strip_suffix(b"\"").unwrap_or(rest);
                self.pending = Some((item_off, name.to_vec()));
            }

            // E4: two consecutive EXIT lines end the data.
            if line == EXIT {
                exits = exits.saturating_add(1);
                if exits >= 2 {
                    self.close_section(item_off);
                    self.end_offset = Some(self.s.offset());
                    return self.check_trailing();
                }
                continue;
            }
            exits = 0;
            let after_naming = self.prev_naming;
            self.prev_naming = false;

            // T10, ADR-21 item 1: the five partition lines are recognised plain lines, never G1
            // candidates or list-K lines.
            if is_partition_line(&line) {
                self.partition_line(item_off, &line);
                continue;
            }
            if line.starts_with(INSERT_INTO) {
                self.binary_part(item_off, &line, after_naming)?;
                continue;
            }
            if let Some(k) = self.intro.find(&line) {
                let forms = self
                    .intro
                    .entries()
                    .get(k)
                    .map(|e| e.forms)
                    .unwrap_or_default();
                let field_off = self.s.offset();
                let form = framing::read_listed_field(self.s, forms)?;
                if let Some(c) = self.counts.per_entry.get_mut(k) {
                    match form {
                        FieldForm::Plain => c.plain = c.plain.saturating_add(1),
                        FieldForm::Marked => c.marked = c.marked.saturating_add(1),
                        FieldForm::Unmarked => c.unmarked = c.unmarked.saturating_add(1),
                    }
                }
                self.note(field_off, self.s.offset(), ItemKind::Field(form));
            } else if framing::looks_like_field(self.s)? {
                // G1: an unknown introducing line.
                let field_off = self.s.offset();
                let form = framing::read_unlisted_field(self.s)?;
                self.counts.g1_fields = self.counts.g1_fields.saturating_add(1);
                let mut w = Warning::new(
                    WarningCode::UnknownIntro,
                    "a line that is not in the list of introducing lines is followed by a field",
                    item_off,
                )
                .with_length(u64::try_from(line.len()).unwrap_or(u64::MAX));
                if let Some(c) = &self.cur {
                    w = w.in_table(c.ordinal);
                }
                self.warnings.push(w);
                self.note(field_off, self.s.offset(), ItemKind::Field(form));
            }
        }
    }

    /// One of the five partition lines [T10, ADR-21]. The shapes of ADR-21 item 2 give no warning; the
    /// other orders give `W-UNTESTED-LAYOUT` once per section (item 4); outside a section each line does.
    fn partition_line(&mut self, offset: u64, line: &[u8]) {
        let Some(o) = self.cur.as_mut() else {
            self.warnings.push(Warning::new(
                WarningCode::UntestedLayout,
                "a partition line outside a table section",
                offset,
            ));
            return;
        };
        let w = &mut self.warnings;
        if line.starts_with(PARTITION_PREFIX) {
            if o.compartition_seen {
                layout_warning(
                    o,
                    w,
                    offset,
                    "PARTITION and COMPARTITION lines in one section",
                );
            }
            o.partition = Some(named(offset, line, PARTITION_PREFIX));
            o.subpartition = None;
            o.partition_seen = true;
            o.naming_seen = true;
            self.prev_naming = true;
        } else if line.starts_with(COMPARTITION_PREFIX) {
            if o.com_open {
                layout_warning(
                    o,
                    w,
                    offset,
                    "COMPARTITION while another COMPARTITION is open",
                );
            }
            if o.partition_seen {
                layout_warning(
                    o,
                    w,
                    offset,
                    "PARTITION and COMPARTITION lines in one section",
                );
            }
            o.partition = Some(named(offset, line, COMPARTITION_PREFIX));
            o.subpartition = None;
            o.compartition_seen = true;
            o.com_open = true;
            o.parts_in_com = 0;
        } else if line.starts_with(SUBPARTITION_PREFIX) {
            if o.com_open {
                o.subpartition = Some(named(offset, line, SUBPARTITION_PREFIX));
                o.naming_seen = true;
                self.prev_naming = true;
            } else {
                layout_warning(o, w, offset, "SUBPARTITION with no open COMPARTITION");
            }
        } else if line == ENDCOMPARTITION {
            if !o.com_open {
                layout_warning(o, w, offset, "ENDCOMPARTITION with no open COMPARTITION");
            } else if o.parts_in_com == 0 {
                layout_warning(
                    o,
                    w,
                    offset,
                    "ENDCOMPARTITION of a COMPARTITION without a part",
                );
            }
            o.com_open = false;
        } else {
            // ENDPARTITION
            if o.com_open {
                layout_warning(o, w, offset, "ENDPARTITION while a COMPARTITION is open");
            }
            o.endpartition_seen = true;
        }
    }

    fn warn_in_part(
        &mut self,
        code: WarningCode,
        message: &'static str,
        offset: u64,
        table: u32,
        part: u32,
    ) {
        self.warnings.push(
            Warning::new(code, message, offset)
                .in_table(table)
                .in_part(part),
        );
    }

    /// `INSERT INTO` line seen: descriptors, then rows. `after_naming`: the line before it was a naming
    /// line.
    fn binary_part(&mut self, insert_offset: u64, line: &[u8], after_naming: bool) -> Result<()> {
        let part_no = self.cur.as_ref().map_or(1, |o| {
            u32::try_from(o.parts.len().saturating_add(1)).unwrap_or(u32::MAX)
        });
        let (ordinal, partition, subpartition, naming_seen) = match &self.cur {
            None => {
                return Err(
                    Error::new(ErrorCode::Framing, "INSERT INTO outside a table section")
                        .at(insert_offset),
                );
            }
            Some(o) if !o.parts.is_empty() && !o.naming_seen => {
                return Err(Error::new(
                    ErrorCode::Framing,
                    "second INSERT INTO in one table section without a naming partition line",
                )
                .at(insert_offset)
                .in_table(o.ordinal)
                .in_part(part_no));
            }
            Some(o) => (
                o.ordinal,
                o.partition.clone(),
                o.subpartition.clone(),
                o.naming_seen,
            ),
        };
        let insert = parse_insert_line(line).ok_or_else(|| {
            Error::new(
                ErrorCode::Framing,
                "INSERT INTO line does not have the form of rule T3",
            )
            .at(insert_offset)
            .in_table(ordinal)
            .in_part(part_no)
            .with_length(u64::try_from(line.len()).unwrap_or(u64::MAX))
        })?;
        let header = read_table_header(self.s, ordinal).map_err(|e| e.in_part(part_no))?;
        // The names and the descriptors describe the same columns (architecture 7.1, fatal class).
        if insert.columns.len() != header.columns.len()
            || insert.placeholders != insert.columns.len()
        {
            return Err(Error::new(
                ErrorCode::Framing,
                "column count of the INSERT INTO line differs from the descriptors",
            )
            .at(insert_offset)
            .in_table(ordinal)
            .in_part(part_no)
            .with_length(u64::try_from(insert.columns.len()).unwrap_or(u64::MAX)));
        }
        // Parts of one table share their columns, by name; the order and the path flag may differ
        // [T10, ADR-21, ADR-24, ADR-31 item 2].
        if let Some(first) = self.cur.as_ref().and_then(|o| o.parts.first()) {
            if !first
                .header
                .same_columns_by_name(&first.insert.columns, &header, &insert.columns)
                || first.insert.table != insert.table
            {
                return Err(Error::new(
                    ErrorCode::Framing,
                    "a part of a partitioned table differs from the first part in names, descriptors or LOB entries",
                )
                .at(insert_offset)
                .in_table(ordinal)
                .in_part(part_no));
            }
        }
        if naming_seen && !after_naming {
            self.warn_in_part(
                WarningCode::UntestedLayout,
                "a binary part is not directly preceded by a naming partition line",
                insert_offset,
                ordinal,
                part_no,
            );
        }
        if let Some(at) = header.odd_entry_tail {
            if self.cur.as_ref().is_some_and(|o| !o.entry_warned) {
                self.warn_in_part(
                    WarningCode::UntestedLayout,
                    "the 2 bytes after a LOB column entry are not 00 00",
                    at,
                    ordinal,
                    part_no,
                );
                self.set_entry_warned();
            }
        }
        let name_differs = header
            .lob_entries
            .iter()
            .zip(&insert.columns)
            .zip(&header.columns)
            .find(|((e, n), c)| is_lob_type(c.type_code) && &e.name != *n)
            .map(|((e, _), _)| e.offset);
        if let Some(at) = name_differs {
            if self.cur.as_ref().is_some_and(|o| !o.entry_warned) {
                self.warn_in_part(
                    WarningCode::UntestedLayout,
                    "the name in a LOB column entry differs from the INSERT INTO name",
                    at,
                    ordinal,
                    part_no,
                );
                self.set_entry_warned();
            }
        }
        let env = Env {
            intro: &self.intro,
            max_line: self.max_line,
            strict_e2a: self.strict_e2a,
        };
        let rows = walk_rows(self.s, &header, ordinal, self.hash, self.record_lob, &env)
            .map_err(|e| e.in_part(part_no))?;
        if rows.e2a_no_rows {
            self.warn_in_part(
                WarningCode::UntestedLayout,
                "no rows decided by rule E2a (first column of length 0 on the direct path)",
                rows.end_marker_offset,
                ordinal,
                part_no,
            );
        }
        self.part_warnings(&rows, ordinal, part_no);
        if let Some(o) = self.cur.as_mut() {
            o.parts_in_com = o.parts_in_com.saturating_add(1);
            o.parts.push(Part {
                insert_offset,
                insert,
                header,
                rows,
                partition,
                subpartition,
            });
        }
        Ok(())
    }

    /// Warnings from what the rows showed: a piece of length 0 in a LONG chain and a chunked conventional LONG
    /// below 65,534 bytes (each once per column and section, ADR-15, ADR-95) and a long LOB value of total 0 (once per section, ADR-23).
    fn part_warnings(&mut self, rows: &RowsSummary, ordinal: u32, part_no: u32) {
        let Some(o) = self.cur.as_mut() else {
            return;
        };
        for st in &rows.long {
            if let Some((row, at)) = st.first_zero_piece {
                if !o.zero_piece_warned.contains(&st.column) {
                    o.zero_piece_warned.push(st.column);
                    self.warnings.push(
                        Warning::new(
                            WarningCode::UntestedLayout,
                            "a piece of length 0 in a LONG chain",
                            at,
                        )
                        .in_table(ordinal)
                        .in_part(part_no)
                        .in_column(st.column)
                        .in_row(row),
                    );
                }
            }
        }
        for st in &rows.long {
            if let Some((row, at)) = st.first_chunked_short {
                if !o.chunked_short_warned.contains(&st.column) {
                    o.chunked_short_warned.push(st.column);
                    self.warnings.push(
                        Warning::new(
                            WarningCode::UntestedLayout,
                            "conventional LONG or LONG RAW in the chunked form with fewer than 65,534 bytes",
                            at,
                        )
                        .in_table(ordinal)
                        .in_part(part_no)
                        .in_column(st.column)
                        .in_row(row),
                    );
                }
            }
        }
        for st in &rows.long {
            if let Some((row, at)) = st.first_fff2_to_fff4 {
                if !o.fff2_warned.contains(&st.column) {
                    o.fff2_warned.push(st.column);
                    self.warnings.push(
                        Warning::new(
                            WarningCode::UntestedLayout,
                            "conventional LONG or LONG RAW length 0xfff2 to 0xfff4 read as one field, not in the fixtures (R-052)",
                            at,
                        )
                        .in_table(ordinal)
                        .in_part(part_no)
                        .in_column(st.column)
                        .in_row(row),
                    );
                }
            }
        }
        if let Some(z) = rows.lob_zero_total {
            if !o.lob_zero_warned {
                o.lob_zero_warned = true;
                self.warnings.push(
                    Warning::new(
                        WarningCode::UntestedLayout,
                        "a long LOB value with a total length of 0",
                        z.offset,
                    )
                    .in_table(ordinal)
                    .in_part(part_no)
                    .in_column(z.column)
                    .in_row(z.row)
                    .with_length(0),
                );
            }
        }
    }

    fn set_entry_warned(&mut self) {
        if let Some(o) = self.cur.as_mut() {
            o.entry_warned = true;
        }
    }

    /// Non-zero bytes after the second `EXIT` are a warning [E4].
    fn check_trailing(&mut self) -> Result<()> {
        loop {
            let at = self.s.offset();
            let chunk = self.s.peek(4096)?;
            if chunk.is_empty() {
                return Ok(());
            }
            let n = chunk.len();
            if let Some(i) = chunk.iter().position(|&b| b != 0) {
                self.warnings.push(Warning::new(
                    WarningCode::Trailing,
                    "non-zero bytes after the second EXIT line",
                    at.saturating_add(u64::try_from(i).unwrap_or(0)),
                ));
                return Ok(());
            }
            self.s.skip(u64::try_from(n).unwrap_or(0))?;
        }
    }
}

/// The five partition lines [T10, ADR-21 item 1].
fn is_partition_line(line: &[u8]) -> bool {
    line.starts_with(PARTITION_PREFIX)
        || line.starts_with(COMPARTITION_PREFIX)
        || line.starts_with(SUBPARTITION_PREFIX)
        || line == ENDPARTITION
        || line == ENDCOMPARTITION
}

/// The name of a partition line: between the quotes, as written.
fn named(offset: u64, line: &[u8], prefix: &[u8]) -> PartitionLine {
    let rest = line.get(prefix.len()..).unwrap_or(&[]);
    let name = rest.strip_suffix(b"\"").unwrap_or(rest);
    PartitionLine {
        offset,
        name_raw: name.to_vec(),
    }
}

/// `W-UNTESTED-LAYOUT` for an order of partition lines outside the confirmed shapes: once per section, at
/// the first (ADR-21 item 4).
fn layout_warning(o: &mut Open, warnings: &mut Vec<Warning>, offset: u64, message: &'static str) {
    if !o.layout_warned {
        o.layout_warned = true;
        warnings
            .push(Warning::new(WarningCode::UntestedLayout, message, offset).in_table(o.ordinal));
    }
}

/// Walks a whole file from byte 0 to the second `EXIT`.
///
/// Header errors and framing errors are `Err`; an unsupported column type [X1] gives `Ok` with
/// `end = Stopped` and the sections before it.
pub fn walk<R: Read + Seek>(reader: R, options: &WalkOptions) -> Result<WalkReport> {
    let mut stream = ByteStream::new(reader, options.buffer_bytes)?;
    walk_stream(&mut stream, options)
}

/// Opens a file read-only and walks it.
pub fn walk_path(path: &std::path::Path, options: &WalkOptions) -> Result<WalkReport> {
    let f = std::fs::File::open(path).map_err(Error::io)?;
    walk(f, options)
}

/// [`walk`] on an existing stream.
pub fn walk_stream<R: Read + Seek>(
    s: &mut ByteStream<R>,
    options: &WalkOptions,
) -> Result<WalkReport> {
    let (header, mut warnings) = read_header(s)?;
    warnings.extend(read_first_item(s, &header)?);
    let intro = match &options.intro_list {
        Some(l) => l.clone(),
        None => IntroList::builtin()?,
    };
    let per_entry = intro
        .entries()
        .iter()
        .map(|e| EntryCount {
            text: String::from_utf8_lossy(&e.text).into_owned(),
            ..EntryCount::default()
        })
        .collect();
    let mut w = Walker {
        s,
        intro,
        max_line: options.max_line_bytes,
        record: options.record_items,
        hash: options.hash_fields,
        record_lob: options.record_lob,
        strict_e2a: options.strict_e2a,
        prev_naming: false,
        sections: Vec::new(),
        counts: ItemCounts {
            lines: 1, // the METRICST / METRICSU line
            per_entry,
            g1_fields: 0,
        },
        warnings: Vec::new(),
        items: Vec::new(),
        end_offset: None,
        cur: None,
        pending: None,
        ordinal: 0,
    };
    if options.record_items {
        w.items.push(ItemRecord {
            offset: header.first_item_offset,
            end: w.s.offset(),
            kind: ItemKind::Line,
        });
    }
    let end = match w.run() {
        Ok(()) => WalkEnd::Complete,
        Err(e)
            if matches!(
                e.code,
                ErrorCode::UnsupportedType | ErrorCode::UnsupportedLayout
            ) =>
        {
            WalkEnd::Stopped(e)
        }
        Err(e) => return Err(e),
    };
    warnings.append(&mut w.warnings);
    let stopped_section = match end {
        WalkEnd::Stopped(_) => w.cur.as_ref().map(|o| StoppedSection {
            ordinal: o.ordinal,
            table_line_offset: o.table_line_offset,
            name_raw: o.name_raw.clone(),
        }),
        WalkEnd::Complete => None,
    };
    Ok(WalkReport {
        header,
        sections: w.sections,
        item_counts: w.counts,
        warnings,
        end,
        stopped_section,
        end_offset: w.end_offset,
        items: w.items,
    })
}
