//! Layer 4, the binary part after an `INSERT INTO` line: column count, descriptors, m, the path flag and the
//! LOB column entries [T3 to T6, L1, X1, X5]. No value is decoded; names are not read here (task T-3).

use std::io::{Read, Seek};

use crate::error::{Error, ErrorCode, Result};
use crate::io::{ByteStream, MIN_BUFFER};

/// Type codes whose descriptor size is known [T5]: VARCHAR2 / NVARCHAR2 (1), NUMBER (2), LONG (8), DATE
/// (12), RAW (23), LONG RAW (24), ROWID (69), CHAR / NCHAR (96), CLOB / NCLOB (112), BLOB (113), TIMESTAMP
/// (180), TIMESTAMP WITH TIME ZONE (181), INTERVAL YEAR TO MONTH (182), INTERVAL DAY TO SECOND (183), UROWID
/// (208), TIMESTAMP WITH LOCAL TIME ZONE (231).
pub const SUPPORTED: [u16; 16] = [
    1, 2, 8, 12, 23, 24, 69, 96, 112, 113, 180, 181, 182, 183, 208, 231,
];
/// Types with four more descriptor bytes: character set id and form [T5].
const WIDE_TYPES: [u16; 3] = [1, 96, 112];

/// Is this a LOB type (CLOB, NCLOB, BLOB) [T5, L1]?
pub fn is_lob_type(code: u16) -> bool {
    code == 112 || code == 113
}

/// Is this LONG or LONG RAW [Y9]?
pub fn is_long_type(code: u16) -> bool {
    code == 8 || code == 24
}

/// Path of a table [T6].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathKind {
    /// Flag 0: `00 00` follows every row [R4].
    Conventional,
    /// Flag 1: nothing between rows [R4].
    Direct,
}

/// One column descriptor [T5].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnDesc {
    /// Offset of the descriptor (its type code).
    pub offset: u64,
    /// Type code.
    pub type_code: u16,
    /// Maximum byte length.
    pub max_len: u16,
    /// Character set id, for types 1, 96 and 112.
    pub charset_id: Option<u16>,
    /// Form (1 database set, 2 national set), for types 1, 96 and 112.
    pub form: Option<u16>,
}

/// One column entry of a LOB table [L1]: `u8 n`, n name bytes, 2 bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LobEntry {
    /// Offset of the entry (its `n` byte).
    pub offset: u64,
    /// The name bytes (empty for a column that is not a LOB; client character set).
    pub name: Vec<u8>,
    /// The 2 bytes after the name (`00 00` in every fixture; meaning unknown).
    pub tail: [u8; 2],
}

/// Everything between the `INSERT INTO` line and the first row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableHeader {
    /// Offset of the u16 column count [T4].
    pub descriptors_offset: u64,
    /// The descriptors in column order.
    pub columns: Vec<ColumnDesc>,
    /// Offset of the u16 m before the path flag [T6].
    pub m_offset: u64,
    /// m: 0, or the column count of a table with a LOB column [T6, L1].
    pub m: u16,
    /// Offset of the path flag [T6].
    pub flag_offset: u64,
    /// Path of the table.
    pub path: PathKind,
    /// The m column entries after the flag [L1] (empty when m = 0).
    pub lob_entries: Vec<LobEntry>,
    /// Offset of the first entry whose 2 bytes after the name are not `00 00` (`W-UNTESTED-LAYOUT`).
    pub odd_entry_tail: Option<u64>,
    /// Offset of the first byte after the flag and the entries: where the rows start [T6].
    pub rows_offset: u64,
}

impl TableHeader {
    /// Does a descriptor have a LOB type (112 or 113)?
    pub fn has_lob(&self) -> bool {
        self.columns.iter().any(|c| is_lob_type(c.type_code))
    }

    /// Does a descriptor have type 8 or 24?
    pub fn has_long(&self) -> bool {
        self.columns.iter().any(|c| is_long_type(c.type_code))
    }

    /// Descriptors, m and column entries without their offsets: what two parts of one table must share.
    pub fn same_layout(&self, other: &Self) -> bool {
        let key = |c: &ColumnDesc| (c.type_code, c.max_len, c.charset_id, c.form);
        self.m == other.m
            && self.path == other.path
            && self
                .columns
                .iter()
                .map(key)
                .eq(other.columns.iter().map(key))
            && self
                .lob_entries
                .iter()
                .map(|e| (&e.name, e.tail))
                .eq(other.lob_entries.iter().map(|e| (&e.name, e.tail)))
    }

    /// The columns of the two parts of one table as sets of (name, descriptor, LOB column entry): the same
    /// count, the same names, and per name the same type code, length, set id, form and LOB entry; the order
    /// and the path flag may differ [ADR-31 item 2, ADR-24, T10]. `names` are the `INSERT INTO` names in
    /// file order of each part.
    pub fn same_columns_by_name(
        &self,
        names: &[Vec<u8>],
        other: &Self,
        other_names: &[Vec<u8>],
    ) -> bool {
        #[allow(clippy::type_complexity)]
        fn keys<'a>(
            h: &'a TableHeader,
            names: &'a [Vec<u8>],
        ) -> Vec<(
            &'a [u8],
            (u16, u16, Option<u16>, Option<u16>),
            Option<(&'a [u8], [u8; 2])>,
        )> {
            let mut v: Vec<_> = h
                .columns
                .iter()
                .enumerate()
                .map(|(j, c)| {
                    (
                        names.get(j).map_or(&[][..], |n| n.as_slice()),
                        (c.type_code, c.max_len, c.charset_id, c.form),
                        h.lob_entries.get(j).map(|e| (e.name.as_slice(), e.tail)),
                    )
                })
                .collect();
            v.sort_unstable();
            v
        }
        self.m == other.m
            && self.columns.len() == other.columns.len()
            && names.len() == self.columns.len()
            && other_names.len() == other.columns.len()
            && keys(self, names) == keys(other, other_names)
    }
}

fn framing(message: &'static str, offset: u64, table: u32) -> Error {
    Error::new(ErrorCode::Framing, message)
        .at(offset)
        .in_table(table)
}

fn layout(message: &'static str, offset: u64, table: u32) -> Error {
    Error::new(ErrorCode::UnsupportedLayout, message)
        .at(offset)
        .in_table(table)
}

/// Reads `n` bytes (any n, in steps the smallest buffer allows).
fn read_vec<R: Read + Seek>(s: &mut ByteStream<R>, n: usize) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(n.min(MIN_BUFFER));
    let mut left = n;
    while left > 0 {
        let step = left.min(MIN_BUFFER);
        out.extend_from_slice(s.read_bytes(step)?);
        left = left.saturating_sub(step);
    }
    Ok(out)
}

/// Reads the binary part. `table` is the 1-based ordinal of the section, for error context. The first
/// type code outside [T5] stops with `UnsupportedType` at the offset of its descriptor [X1, ADR-12]; a
/// layout that [T6], [L1] and [X5] do not allow stops with `UnsupportedLayout`.
pub fn read_table_header<R: Read + Seek>(s: &mut ByteStream<R>, table: u32) -> Result<TableHeader> {
    let descriptors_offset = s.offset();
    let count = s.read_u16_le()?;
    if count == 0 {
        return Err(framing("table has no columns", descriptors_offset, table));
    }
    let mut columns = Vec::new();
    for position in 1..=u32::from(count) {
        let offset = s.offset();
        let type_code = s.read_u16_le()?;
        if !SUPPORTED.contains(&type_code) {
            return Err(
                Error::new(ErrorCode::UnsupportedType, "column type is not supported")
                    .at(offset)
                    .in_table(table)
                    .in_column(position)
                    .with_type_code(type_code),
            );
        }
        let max_len = s.read_u16_le()?;
        let (charset_id, form) = if WIDE_TYPES.contains(&type_code) {
            (Some(s.read_u16_le()?), Some(s.read_u16_le()?))
        } else {
            (None, None)
        };
        columns.push(ColumnDesc {
            offset,
            type_code,
            max_len,
            charset_id,
            form,
        });
    }
    let has_lob = columns.iter().any(|c| is_lob_type(c.type_code));
    // m, then the u16 path flag [T6].
    let m_offset = s.offset();
    let m = s.read_u16_le()?;
    let flag_offset = s.offset();
    let path = match s.read_u16_le()? {
        0 => PathKind::Conventional,
        1 => PathKind::Direct,
        _ => {
            return Err(layout("path flag is neither 0 nor 1", flag_offset, table));
        }
    };
    if m != 0 && usize::from(m) != columns.len() {
        return Err(
            layout("m is neither 0 nor the column count", m_offset, table)
                .with_length(u64::from(m)),
        );
    }
    if (m != 0) != has_lob {
        return Err(layout(
            "m and the LOB columns of the descriptors do not agree",
            m_offset,
            table,
        )
        .with_length(u64::from(m)));
    }
    if m != 0 && path == PathKind::Direct {
        return Err(layout(
            "a LOB table with path flag 1 is not in the fixtures",
            flag_offset,
            table,
        ));
    }
    let mut lob_entries = Vec::new();
    let mut odd_entry_tail = None;
    for (index, column) in columns.iter().enumerate().take(usize::from(m)) {
        let offset = s.offset();
        let n = s.read_u8()?;
        if (n > 0) != is_lob_type(column.type_code) {
            return Err(layout(
                "a column entry has a name for a column that is not a LOB, or none for a LOB column",
                offset,
                table,
            )
            .in_column(u32::try_from(index.saturating_add(1)).unwrap_or(u32::MAX))
            .with_length(u64::from(n)));
        }
        let name = read_vec(s, usize::from(n))?;
        let tail_at = s.offset();
        let tail: [u8; 2] = {
            let b = s.read_bytes(2)?;
            <[u8; 2]>::try_from(b).unwrap_or([0, 0])
        };
        if tail != [0, 0] && odd_entry_tail.is_none() {
            odd_entry_tail = Some(tail_at);
        }
        lob_entries.push(LobEntry { offset, name, tail });
    }
    Ok(TableHeader {
        descriptors_offset,
        columns,
        m_offset,
        m,
        flag_offset,
        path,
        lob_entries,
        odd_entry_tail,
        rows_offset: s.offset(),
    })
}
