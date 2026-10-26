//! The declared types of one table against its descriptors (architecture 5.5 steps 3 to 5, ADR-6, ADR-24).
//!
//! Input: the `CREATE TABLE` line, the descriptors and the `INSERT INTO` names of one binary part. Output: for
//! each column, in **declared order**, its declaration (if the text was understood and agrees with the
//! descriptor), its position in the part and its Arrow type; plus the warnings. A line or column that does not
//! agree is never an error: the table or the column is mapped from its descriptor alone and
//! `W-DDL-FALLBACK` says so.

use arrow_schema::DataType;

use crate::charset::{decode_text, Charset};
use crate::error::{Warning, WarningCode};
use crate::exp::table::{ColumnDesc, PathKind};
use crate::types::declared::{parse_column_list, ColumnDef, DeclaredType};
use crate::types::map::{
    arrow_from_declared, arrow_from_descriptor, check_agreement, clob_charset_supported,
    text_charset_supported, Disagreement, IntervalAs, NumberType, TypeOptions, TypeSource,
};
use crate::values::number::OverflowPolicy;

/// What `resolve_table` needs to know about one table part.
#[derive(Debug, Clone, Copy)]
pub struct ResolveInput<'a> {
    /// 1-based ordinal of the table section.
    pub table_ordinal: u32,
    /// The table name (ASCII decoding), for `number_types` keys.
    pub table_name: &'a str,
    /// The `CREATE TABLE` line as written.
    pub ddl_raw: &'a [u8],
    /// File offset of that line.
    pub ddl_offset: u64,
    /// The descriptors in file order.
    pub descriptors: &'a [ColumnDesc],
    /// The `INSERT INTO` names in file order.
    pub insert_names: &'a [Vec<u8>],
    /// The path of the part [T6].
    pub path: PathKind,
    /// The client set, for the column names of `number_types` keys; `None`: ASCII decoding (a set the reader
    /// cannot decode).
    pub client: Option<Charset>,
}

/// One column after resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedColumn {
    /// Index of the column in file order (0-based): descriptor, `INSERT INTO` name and row field.
    pub file_index: usize,
    /// The declared type, if the text was understood and agrees with the descriptor.
    pub declared: Option<DeclaredType>,
    /// ` NOT NULL ENABLE` was present.
    pub not_null: bool,
    /// The type text as written (kept even if it was not understood); `None` on a table fallback.
    pub type_text: Option<Vec<u8>>,
    /// The Arrow type.
    pub arrow_type: DataType,
    /// Where the type came from (ADR-33 item 4); only the NUMBER columns (type code 2) use it.
    pub type_source: TypeSource,
    /// The overflow policy of the `number_types` entry that fixed the type (`TypeSource::Override`), if the
    /// entry names one; it wins over `ReadOptions.on_number_overflow` (ADR-33 item 5).
    pub overflow: Option<OverflowPolicy>,
}

/// The result for one table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedTable {
    /// Columns in declared order; on a table fallback in file order.
    pub columns: Vec<ResolvedColumn>,
    /// The whole table was mapped from the descriptors.
    pub fallback: bool,
    /// `W-DDL-FALLBACK`, `W-UNTESTED-LAYOUT`, `W-CHARSET-UNSUPPORTED`, `W-TSLTZ`. `column` is always the **file
    /// position** (ADR-29 item 1), also for a column matched to a definition at another declared position.
    pub warnings: Vec<Warning>,
    /// `W-UNTESTED-LAYOUT` for the column order was raised (it is raised once per table, ADR-31).
    pub order_warned: bool,
}

fn pos32(i: usize) -> u32 {
    u32::try_from(i.saturating_add(1)).unwrap_or(u32::MAX)
}

fn off(base: u64, inside: usize) -> u64 {
    base.saturating_add(u64::try_from(inside).unwrap_or(u64::MAX))
}

fn number_override(
    input: &ResolveInput<'_>,
    opts: &TypeOptions,
    file_index: usize,
) -> Option<NumberType> {
    if opts.number_types.is_empty() {
        return None;
    }
    let (col, _) = decode_text(input.client, input.insert_names.get(file_index)?);
    opts.number_types
        .get(&format!("{}.{}", input.table_name, col))
        .copied()
}

/// The overflow policy of the `number_types` entry of the column, if the entry names one.
fn number_policy(
    input: &ResolveInput<'_>,
    opts: &TypeOptions,
    file_index: usize,
) -> Option<OverflowPolicy> {
    if opts.number_overflow.is_empty() {
        return None;
    }
    let (col, _) = decode_text(input.client, input.insert_names.get(file_index)?);
    opts.number_overflow
        .get(&format!("{}.{}", input.table_name, col))
        .copied()
}

fn charset_warning(
    input: &ResolveInput<'_>,
    opts: &TypeOptions,
    d: &ColumnDesc,
    position: usize,
    out: &mut Vec<Warning>,
) {
    if matches!(d.type_code, 1 | 96) && !text_charset_supported(d, opts) {
        out.push(
            Warning::new(
                WarningCode::CharsetUnsupported,
                "the character set of the column is none of the five the reader decodes: the column is returned as bytes",
                d.offset,
            )
            .in_table(input.table_ordinal)
            .in_column(pos32(position))
            .with_length(u64::from(d.charset_id.unwrap_or(0))),
        );
    }
    if d.type_code == 112 && d.form != Some(2) && !clob_charset_supported(d.charset_id) {
        out.push(
            Warning::new(
                WarningCode::CharsetUnsupported,
                "the CLOB character set is none of AL32UTF8, US7ASCII, WE8MSWIN1252, CL8MSWIN1251: the column is returned as bytes",
                d.offset,
            )
            .in_table(input.table_ordinal)
            .in_column(pos32(position)),
        );
    }
}

/// `W-TSLTZ`: the meaning of a TIMESTAMP WITH LOCAL TIME ZONE value (UTC or the database time zone) is
/// unknown (ADR-27 item 2). Once per column, whatever the declaration says; it depends on the type only.
fn tsltz_warning(
    input: &ResolveInput<'_>,
    d: &ColumnDesc,
    position: usize,
    out: &mut Vec<Warning>,
) {
    if d.type_code == 231 {
        out.push(
            Warning::new(
                WarningCode::Tsltz,
                "TIMESTAMP WITH LOCAL TIME ZONE is returned as stored, without a zone: UTC or the database time zone is unknown",
                d.offset,
            )
            .in_table(input.table_ordinal)
            .in_column(pos32(position)),
        );
    }
}

/// `interval_as="string"` on a table fallback: the precisions of the declaration are unknown, the text takes
/// the digits each value needs and 9 fractional digits (ADR-47), one `W-DDL-FALLBACK` per column. A column
/// whose own definition fell back has its warning already.
fn interval_text_warning(
    input: &ResolveInput<'_>,
    opts: &TypeOptions,
    d: &ColumnDesc,
    position: usize,
    out: &mut Vec<Warning>,
) {
    if opts.interval_as == IntervalAs::String && matches!(d.type_code, 182 | 183) {
        out.push(
            Warning::new(
                WarningCode::DdlFallback,
                "no usable declaration of the INTERVAL column: its text takes the digits each value needs and 9 fractional digits",
                d.offset,
            )
            .in_table(input.table_ordinal)
            .in_column(pos32(position))
            .with_length(u64::from(d.max_len)),
        );
    }
}

fn table_fallback(
    input: &ResolveInput<'_>,
    opts: &TypeOptions,
    at: u64,
    message: &'static str,
) -> ResolvedTable {
    let mut warnings = vec![Warning::new(WarningCode::DdlFallback, message, at)
        .in_table(input.table_ordinal)
        .with_length(u64::try_from(input.descriptors.len()).unwrap_or(u64::MAX))];
    let mut columns = Vec::new();
    for (i, d) in input.descriptors.iter().enumerate() {
        charset_warning(input, opts, d, i, &mut warnings);
        tsltz_warning(input, d, i, &mut warnings);
        interval_text_warning(input, opts, d, i, &mut warnings);
        let ovr = number_override(input, opts, i);
        columns.push(ResolvedColumn {
            file_index: i,
            declared: None,
            not_null: false,
            type_text: None,
            arrow_type: arrow_from_descriptor(d, opts, ovr),
            type_source: fallback_source(d, ovr),
            overflow: ovr.and(number_policy(input, opts, i)),
        });
    }
    ResolvedTable {
        columns,
        fallback: true,
        warnings,
        order_warned: false,
    }
}

/// The origin of the type of a column mapped from its descriptor alone.
fn fallback_source(d: &ColumnDesc, ovr: Option<NumberType>) -> TypeSource {
    if d.type_code == 2 {
        if ovr.is_some() {
            TypeSource::Override
        } else {
            TypeSource::Default
        }
    } else {
        TypeSource::Declared
    }
}

/// For each definition, the index of the one `INSERT INTO` name equal to its name; `None` unless that is a
/// one-to-one match.
fn match_by_name(defs: &[ColumnDef], names: &[Vec<u8>]) -> Option<Vec<usize>> {
    let mut used = vec![false; names.len()];
    let mut out = Vec::with_capacity(defs.len());
    for def in defs {
        let mut hits = names
            .iter()
            .enumerate()
            .filter(|(_, n)| **n == def.name)
            .map(|(j, _)| j);
        let (Some(j), None) = (hits.next(), hits.next()) else {
            return None;
        };
        let slot = used.get_mut(j)?;
        if *slot {
            return None;
        }
        *slot = true;
        out.push(j);
    }
    Some(out)
}

/// Architecture 5.5 steps 3 to 5.
pub fn resolve_table(input: &ResolveInput<'_>, opts: &TypeOptions) -> ResolvedTable {
    let ncols = input.descriptors.len();
    let list = match parse_column_list(input.ddl_raw) {
        Ok(l) => l,
        Err(f) => {
            return table_fallback(
                input,
                opts,
                off(input.ddl_offset, f.offset),
                "the column list of the CREATE TABLE line does not parse",
            )
        }
    };
    if list.columns.len() != ncols || input.insert_names.len() != ncols {
        return table_fallback(
            input,
            opts,
            input.ddl_offset,
            "the number of column definitions differs from the number of descriptors",
        );
    }
    let Some(file_of_def) = match_by_name(&list.columns, input.insert_names) else {
        return table_fallback(
            input,
            opts,
            input.ddl_offset,
            "the INSERT INTO names do not match the column definitions one to one",
        );
    };
    let mut warnings = Vec::new();
    let mut order_warned = false;

    // Order (ADR-24): identical, or on the direct path the LONG / LONG RAW columns moved last.
    let identity: Vec<usize> = (0..ncols).collect();
    if file_of_def != identity {
        let non_long = list.columns.iter().enumerate().filter(|(_, c)| !is_long(c));
        let long = list.columns.iter().enumerate().filter(|(_, c)| is_long(c));
        let expected: Vec<usize> = non_long.chain(long).map(|(i, _)| i).collect();
        // `file_of_def[i]` is the file position of definition i; the file order lists definitions by position
        let mut by_file: Vec<(usize, usize)> = file_of_def
            .iter()
            .enumerate()
            .map(|(def, file)| (*file, def))
            .collect();
        by_file.sort_unstable();
        let actual: Vec<usize> = by_file.into_iter().map(|(_, def)| def).collect();
        if !(input.path == PathKind::Direct && actual == expected) {
            warnings.push(
                Warning::new(
                    WarningCode::UntestedLayout,
                    "the INSERT INTO order differs from the CREATE TABLE order other than by a LONG column moved last on the direct path; columns are matched by name",
                    input.ddl_offset,
                )
                .in_table(input.table_ordinal)
                .in_part(1),
            );
            order_warned = true;
        }
    }

    let mut columns = Vec::with_capacity(ncols);
    for (def, &file_index) in list.columns.iter().zip(&file_of_def) {
        let Some(d) = input.descriptors.get(file_index) else {
            return table_fallback(input, opts, input.ddl_offset, "descriptor missing");
        };
        let ovr = if d.type_code == 2 {
            number_override(input, opts, file_index)
        } else {
            None
        };
        let outcome = match def.kind {
            None => Err("the type text is not one of the forms of rule T9"),
            Some(kind) => match check_agreement(&kind, d) {
                Err(Disagreement::Family) => {
                    Err("the declared type does not belong to the descriptor type code")
                }
                Err(Disagreement::Length) => {
                    Err("the declared length disagrees with the descriptor length")
                }
                Err(Disagreement::Range) => Err("the declared type is out of range"),
                Ok(()) => arrow_from_declared(&kind, d, opts, ovr)
                    .map(|m| (kind, m))
                    .ok_or("a precision, scale or length of the declared type is out of range"),
            },
        };
        charset_warning(input, opts, d, file_index, &mut warnings);
        tsltz_warning(input, d, file_index, &mut warnings);
        match outcome {
            Ok((kind, mapping)) => {
                if mapping.no_decimal128 {
                    warnings.push(
                        Warning::new(
                            WarningCode::DdlFallback,
                            "no Decimal128 holds NUMBER(p,s): the column takes number_default",
                            off(input.ddl_offset, def.offset),
                        )
                        .in_table(input.table_ordinal)
                        .in_column(pos32(file_index))
                        .with_length(u64::from(d.max_len)),
                    );
                }
                columns.push(ResolvedColumn {
                    file_index,
                    declared: Some(kind),
                    not_null: def.not_null,
                    type_text: Some(def.type_text.clone()),
                    arrow_type: mapping.arrow_type,
                    type_source: mapping.source,
                    overflow: ovr.and(number_policy(input, opts, file_index)),
                });
            }
            Err(message) => {
                warnings.push(
                    Warning::new(
                        WarningCode::DdlFallback,
                        message,
                        off(input.ddl_offset, def.offset),
                    )
                    .in_table(input.table_ordinal)
                    .in_column(pos32(file_index))
                    .with_length(u64::from(d.max_len)),
                );
                columns.push(ResolvedColumn {
                    file_index,
                    declared: None,
                    not_null: def.not_null,
                    type_text: Some(def.type_text.clone()),
                    arrow_type: arrow_from_descriptor(d, opts, ovr),
                    type_source: fallback_source(d, ovr),
                    overflow: ovr.and(number_policy(input, opts, file_index)),
                });
            }
        }
    }
    ResolvedTable {
        columns,
        fallback: false,
        warnings,
        order_warned,
    }
}

/// The columns of one binary part, as far as the order of its fields is concerned.
#[derive(Debug, Clone, Copy)]
pub struct PartLayout<'a> {
    /// The descriptors in file order.
    pub descriptors: &'a [ColumnDesc],
    /// The `INSERT INTO` names in file order.
    pub insert_names: &'a [Vec<u8>],
    /// The path flag of the part [T6].
    pub path: PathKind,
    /// File offset of the part's `INSERT INTO` line.
    pub insert_offset: u64,
}

/// The place of each field of a part in the declared order (ADR-31).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartOrder {
    /// For each file position (index) the declared position (value), both 0-based.
    pub file_order: Vec<u32>,
    /// `W-UNTESTED-LAYOUT`: the part's order differs from the declared order other than by a LONG / LONG RAW
    /// column moved last on a part with path flag 1. Raised only if the caller asked for it.
    pub warning: Option<Warning>,
}

fn idx32(i: usize) -> u32 {
    u32::try_from(i).unwrap_or(u32::MAX)
}

/// `PartInfo.file_order` of the first part: the declared index of each file position.
pub fn first_part_order(table: &ResolvedTable) -> Vec<u32> {
    let mut order = vec![0u32; table.columns.len()];
    for (declared, c) in table.columns.iter().enumerate() {
        if let Some(slot) = order.get_mut(c.file_index) {
            *slot = idx32(declared);
        }
    }
    order
}

/// ADR-31 items 1 and 2: matches the fields of another part to the declared columns by name. The names of
/// the declared columns are those of the first part's `INSERT INTO` line (the walker checked that the part
/// has the same names and descriptors, in any order). If the first part has two equal names (the table is on
/// the fallback, ADR-24 item 2) a name cannot identify a field: the order is then the position. `None` if
/// a name is missing, which the walker's check rules out. `warn` asks for the order warning (once per table:
/// the caller says whether it was raised already).
pub fn part_order(
    table: &ResolvedTable,
    first: &PartLayout<'_>,
    part: &PartLayout<'_>,
    ordinal: u32,
    part_no: u32,
    warn: bool,
) -> Option<PartOrder> {
    let n = first.insert_names.len();
    let mut file_order = vec![0u32; part.insert_names.len()];
    let unique = {
        let mut sorted: Vec<&Vec<u8>> = first.insert_names.iter().collect();
        sorted.sort_unstable();
        sorted.windows(2).all(|w| w.first() != w.get(1))
    };
    if !unique {
        for (j, slot) in file_order.iter_mut().enumerate() {
            *slot = idx32(j);
        }
    } else {
        if part.insert_names.len() != n {
            return None;
        }
        for (declared, c) in table.columns.iter().enumerate() {
            let name = first.insert_names.get(c.file_index)?;
            let j = part.insert_names.iter().position(|x| x == name)?;
            *file_order.get_mut(j)? = idx32(declared);
        }
    }
    let identity = file_order.iter().enumerate().all(|(j, d)| *d == idx32(j));
    let mut warning = None;
    if warn && !identity {
        // the LONG / LONG RAW columns of the declared order moved last, on the direct path [T3]
        let is_long = |declared: usize| {
            table
                .columns
                .get(declared)
                .and_then(|c| first.descriptors.get(c.file_index))
                .is_some_and(|d| d.type_code == 8 || d.type_code == 24)
        };
        let expected: Vec<u32> = (0..n)
            .filter(|i| !is_long(*i))
            .chain((0..n).filter(|i| is_long(*i)))
            .map(idx32)
            .collect();
        let moved_last = part.path == PathKind::Direct && file_order == expected;
        if !moved_last {
            warning = Some(
                Warning::new(
                    WarningCode::UntestedLayout,
                    "the INSERT INTO order of a part differs from the CREATE TABLE order other than by a LONG column moved last on the direct path; fields are matched by name",
                    part.insert_offset,
                )
                .in_table(ordinal)
                .in_part(part_no),
            );
        }
    }
    Some(PartOrder {
        file_order,
        warning,
    })
}

fn is_long(c: &ColumnDef) -> bool {
    c.kind.as_ref().is_some_and(DeclaredType::is_long)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
mod part_order_tests {
    use super::*;

    fn desc(type_code: u16) -> ColumnDesc {
        ColumnDesc {
            offset: 0,
            type_code,
            max_len: if type_code == 8 { 0 } else { 10 },
            charset_id: None,
            form: None,
        }
    }

    fn names(n: &[&str]) -> Vec<Vec<u8>> {
        n.iter().map(|x| x.as_bytes().to_vec()).collect()
    }

    /// Declared order A (VARCHAR2), B (LONG), C (NUMBER), D (DATE); the first part in declared order.
    fn table() -> (ResolvedTable, Vec<ColumnDesc>, Vec<Vec<u8>>) {
        let columns = (0..4)
            .map(|i| ResolvedColumn {
                file_index: i,
                declared: None,
                not_null: false,
                type_text: None,
                arrow_type: DataType::Null,
                type_source: TypeSource::Declared,
                overflow: None,
            })
            .collect();
        (
            ResolvedTable {
                columns,
                fallback: false,
                warnings: Vec::new(),
                order_warned: false,
            },
            [1, 8, 2, 12].map(desc).to_vec(),
            names(&["A", "B", "C", "D"]),
        )
    }

    fn run(
        file_names: &[&str],
        file_types: &[u16],
        path: PathKind,
        warn: bool,
    ) -> Option<PartOrder> {
        let (t, d, n) = table();
        let first = PartLayout {
            descriptors: &d,
            insert_names: &n,
            path: PathKind::Conventional,
            insert_offset: 10,
        };
        let pd: Vec<ColumnDesc> = file_types.iter().map(|c| desc(*c)).collect();
        let pn = names(file_names);
        let part = PartLayout {
            descriptors: &pd,
            insert_names: &pn,
            path,
            insert_offset: 77,
        };
        part_order(&t, &first, &part, 3, 2, warn)
    }

    #[test]
    fn identity_and_long_moved_last_on_the_direct_path_do_not_warn() {
        let o = run(
            &["A", "B", "C", "D"],
            &[1, 8, 2, 12],
            PathKind::Direct,
            true,
        )
        .unwrap();
        assert_eq!((o.file_order, o.warning), (vec![0, 1, 2, 3], None));
        // LONG moved last, flag 1: no warning, and the order maps the fields back
        let o = run(
            &["A", "C", "D", "B"],
            &[1, 2, 12, 8],
            PathKind::Direct,
            true,
        )
        .unwrap();
        assert_eq!((o.file_order, o.warning), (vec![0, 2, 3, 1], None));
    }

    #[test]
    fn long_moved_last_on_a_conventional_part_and_any_other_order_warn() {
        for (n, ty, path) in [
            (["A", "C", "D", "B"], [1, 2, 12, 8], PathKind::Conventional),
            (["A", "C", "B", "D"], [1, 2, 8, 12], PathKind::Direct),
            (["D", "C", "B", "A"], [12, 2, 8, 1], PathKind::Direct),
        ] {
            let o = run(&n, &ty, path, true).unwrap();
            let w = o.warning.unwrap();
            assert_eq!(w.code, WarningCode::UntestedLayout);
            assert_eq!((w.table, w.part, w.offset), (Some(3), Some(2), Some(77)));
        }
        // the caller says it warned already: no second warning, the order is still found
        let o = run(
            &["D", "C", "B", "A"],
            &[12, 2, 8, 1],
            PathKind::Direct,
            false,
        )
        .unwrap();
        assert_eq!((o.file_order, o.warning), (vec![3, 2, 1, 0], None));
    }

    #[test]
    fn a_name_that_is_missing_gives_none_and_equal_names_use_the_position() {
        assert!(run(
            &["A", "B", "C", "X"],
            &[1, 8, 2, 12],
            PathKind::Direct,
            true
        )
        .is_none());
        let (mut t, mut d, _) = table();
        let dup = names(&["A", "A", "C", "D"]);
        t.fallback = true;
        d[1] = desc(2);
        let first = PartLayout {
            descriptors: &d,
            insert_names: &dup,
            path: PathKind::Conventional,
            insert_offset: 0,
        };
        let o = part_order(&t, &first, &first, 1, 2, true).unwrap();
        assert_eq!((o.file_order, o.warning), (vec![0, 1, 2, 3], None));
    }
}
