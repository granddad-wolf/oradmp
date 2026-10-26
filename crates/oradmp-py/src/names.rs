//! Names for the numbers of the core: the core's errors and warnings carry the table ordinal, the part index and
//! the column's file position (ADR-29); Python shows names. The snapshot is taken from the catalog after the
//! scan and shared by the `Dump` and the readers it hands out.

use oradmp_core::dump::TableInfo;

#[derive(Debug, Default)]
struct PartNames {
    partition: Option<String>,
    subpartition: Option<String>,
    /// For each file position (0-based) the declared index of the column.
    file_order: Vec<u32>,
}

#[derive(Debug, Default)]
struct TableNames {
    ordinal: u32,
    name: String,
    parts: Vec<PartNames>,
    /// Column names in declared order, with their file positions in the first part.
    columns: Vec<(String, u32)>,
}

/// The names of all tables of a catalog.
#[derive(Debug, Default)]
pub struct Names {
    tables: Vec<TableNames>,
}

impl Names {
    pub fn from_tables(tables: &[TableInfo]) -> Self {
        Self {
            tables: tables
                .iter()
                .map(|t| TableNames {
                    ordinal: t.ordinal,
                    name: t.name.clone(),
                    parts: t
                        .parts
                        .iter()
                        .map(|p| PartNames {
                            partition: p.partition.clone(),
                            subpartition: p.subpartition.clone(),
                            file_order: p.file_order.clone(),
                        })
                        .collect(),
                    columns: t
                        .columns
                        .iter()
                        .map(|c| (c.name.clone(), c.file_position))
                        .collect(),
                })
                .collect(),
        }
    }

    fn table(&self, ordinal: u32) -> Option<&TableNames> {
        self.tables.iter().find(|t| t.ordinal == ordinal)
    }

    pub fn table_name(&self, ordinal: u32) -> Option<String> {
        self.table(ordinal).map(|t| t.name.clone())
    }

    /// The partition and subpartition names of the 1-based part `part`; `None` for an unpartitioned table.
    pub fn part(&self, table: u32, part: u32) -> Option<(Option<String>, Option<String>)> {
        let t = self.table(table)?;
        let index = usize::try_from(part.saturating_sub(1)).ok()?;
        let p = t.parts.get(index)?;
        if p.partition.is_none() && p.subpartition.is_none() {
            return None;
        }
        Some((p.partition.clone(), p.subpartition.clone()))
    }

    /// The name of the column at the 1-based file position `position` of the part `part` (the first part if none
    /// is given).
    pub fn column(&self, table: u32, part: Option<u32>, position: u32) -> Option<String> {
        let t = self.table(table)?;
        let file_index = usize::try_from(position.checked_sub(1)?).ok()?;
        match part {
            Some(part) => {
                let p = t.parts.get(usize::try_from(part.saturating_sub(1)).ok()?)?;
                let declared = usize::try_from(*p.file_order.get(file_index)?).ok()?;
                t.columns.get(declared).map(|c| c.0.clone())
            }
            None => t
                .columns
                .iter()
                .find(|c| c.1 == position)
                .map(|c| c.0.clone()),
        }
    }
}
