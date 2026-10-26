//! `oradmp-core`: reads Oracle legacy `exp` dump files without an Oracle server.
//!
//! Tasks T-1 to T-3c and T-5 (module [`types`]: declared types and the Arrow type mapping): byte stream, header, framing, the structural walker of rules v3.4 (16 type codes, LONG chains,
//! LOB values, composite partitions, the empty-table decisions), the catalog and the table metadata. Task T-3b: `TableInfo.parts` with `PartInfo` (partition, subpartition,
//! offsets, `file_order`), per-part checks by column name, `part` in errors and warnings, `ColumnInfo.lob` /
//! `ColumnInfo.long`. Task T-4 adds
//! [`values`]: pure decoders from the bytes of one field to typed values (NUMBER, dates, intervals, ROWID).
//! Task T-6 adds [`charset`], [`arrow_out`] (builders, field metadata) and [`reader`] (`Dump::read_table`,
//! `TableBatches`: the first complete read, all parts of a table, file order to declared order).
//! Task T-11: `strict`, `extra_intro_lines`, the per-column overflow policy, `W-NUMBER-ROUNDED`'s digits figure and
//! [`survey`] (the counting core of the survey).

#![forbid(unsafe_code)]

pub mod arrow_out;
pub mod charset;
pub mod dump;
pub mod error;
pub mod exp;
pub mod guard;
pub mod io;
pub mod reader;
pub mod survey;
pub mod types;
pub mod values;

pub use error::{Error, ErrorCode, Result, Warning, WarningCode};
