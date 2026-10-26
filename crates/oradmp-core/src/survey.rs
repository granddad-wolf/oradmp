//! The survey's counting core (architecture 5.2b, 8.5; tasks T-11 and T-12): per column, counts only, no value,
//! no minimum value, no maximum value. The command line that prints them is `python -m oradmp survey` (T-12).
//!
//! [`count_table`] walks the rows of every part of one table with a [`RowSink`] that looks at each field
//! just far enough to classify it. It builds no Arrow array and reads no LOB byte. What it counts, per column:
//!
//! * `values` and `nulls` (a LOB value counts at its place in the value area, not at its locator);
//! * `years_below_1`: DATE, TIMESTAMP, TIMESTAMP WITH TIME ZONE and WITH LOCAL TIME ZONE values whose year is
//!   below 1; they are `E-BAD-DATETIME` when read (ADR-28 (b));
//! * `months_beyond_32_bits`: INTERVAL YEAR TO MONTH values whose months do not fit 32 bits, `E-BAD-INTERVAL`
//!   when read (ADR-28 (a));
//! * `no_decimal128_holds`: a column declared `NUMBER` that no Decimal128 holds (ADR-30 item 3), which takes
//!   `number_default` with `W-DDL-FALLBACK` (a property of the declaration, not of the rows);
//! * `lob_refused_short` / `lob_refused_long`: the values of a CLOB / NCLOB column that ADR-25 refuses with
//!   `E-LOB-UNCONFIRMED` under the default options, by form (the same predicate as the reader's, so the two
//!   cannot differ);
//! * (T-12) `len_min`, `len_max`, `len_total`: the smallest, the largest and the sum of the stored lengths of the
//!   non-NULL values (bytes; a LONG chain and a LOB value by their total), from which the survey prints the mean;
//! * (T-12) `number`: the NUMBER profile of architecture 5.2b for a column of type 2: integers, values that fit
//!   Int64, per candidate type (Decimal128(38, 10), Decimal256(76, 38) and the resolved type if it is another
//!   decimal) the values that are exact, that `round` changes (of those, to 0) and whose integer part does not fit;
//!   the values of 39 or 40 significant digits (the computed-quotient signature [Y1]); the largest `sig`, `frac`
//!   and `int`, the smallest and largest `exp10`, the smallest and largest position of the leading digit. Counts
//!   and digit counts only: no value, no minimum, no maximum value.

use arrow_schema::DataType;

use crate::dump::{ColumnInfo, Dump, TableRef};
use crate::error::Result;
use crate::exp::rows::{LobAction, LobForm, RowSink};
use crate::reader::lob_value_unconfirmed;
use crate::types::declared::DeclaredType;
use crate::types::map::{declared_number, NumberFit, NumberType};
use crate::values::datetime::decode_date_fields;
use crate::values::interval::IntervalYm;
use crate::values::number::{Fitted, OraNumber, OverflowPolicy};

/// The counts of one candidate type of a NUMBER column (architecture 5.2b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateCounts {
    /// The type as text, for example `decimal(38,10)` or `decimal256(76,38)`.
    pub label: String,
    /// Values the type holds exactly.
    pub exact: u64,
    /// Values `round` would change (fractional digits below the scale; the integer part fits).
    pub rounded: u64,
    /// Of those, the values that become 0.
    pub rounded_to_zero: u64,
    /// Values whose integer part does not fit the type: a value error under every policy.
    pub int_too_large: u64,
}

/// The NUMBER profile of one column of type 2 (architecture 5.2b).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NumberProfile {
    /// Fields that are not valid NUMBER bytes (`E-BAD-NUMBER` when read); not in the other counts.
    pub invalid: u64,
    /// Values without fractional digits (zero included).
    pub integers: u64,
    /// Integer values that fit Int64.
    pub fit_int64: u64,
    /// Values with 39 or 40 significant digits (the computed-quotient signature [Y1]).
    pub sig_39_or_40: u64,
    /// The largest number of significant digits.
    pub max_sig: u32,
    /// The largest number of fractional digits.
    pub max_frac: u32,
    /// The largest number of integer digits.
    pub max_int: u32,
    /// The smallest power of ten of the coefficient (zero values are left out).
    pub min_exp10: Option<i16>,
    /// The largest power of ten of the coefficient (zero values are left out).
    pub max_exp10: Option<i16>,
    /// The smallest position of the leading digit, exp10 + sig - 1 (zero values are left out).
    pub min_lead: Option<i32>,
    /// The largest position of the leading digit.
    pub max_lead: Option<i32>,
    /// Decimal128(38, 10), Decimal256(76, 38) and, if it is another decimal, the resolved type.
    pub candidates: Vec<CandidateCounts>,
}

/// One candidate of a column: its label and how a value is classified.
#[derive(Debug, Clone, Copy)]
enum Cand {
    D128(u8, u8),
    D256(u8, u8),
}

impl Cand {
    fn label(self) -> String {
        match self {
            Self::D128(p, s) => format!("decimal({p},{s})"),
            Self::D256(p, s) => format!("decimal256({p},{s})"),
        }
    }

    /// Classifies one value: 0 exact, 1 rounded, 2 rounded to zero, 3 integer part too large.
    fn class(self, n: &OraNumber) -> Option<u8> {
        let r = match self {
            Self::D128(p, s) => n
                .to_decimal128(p, s, OverflowPolicy::Round)
                .map(|f| match f {
                    Fitted::Exact(_) => 0,
                    Fitted::Rounded { to_zero, .. } => {
                        if to_zero {
                            2
                        } else {
                            1
                        }
                    }
                    Fitted::Nulled => 0,
                }),
            Self::D256(p, s) => n
                .to_decimal256(p, s, OverflowPolicy::Round)
                .map(|f| match f {
                    Fitted::Exact(_) => 0,
                    Fitted::Rounded { to_zero, .. } => {
                        if to_zero {
                            2
                        } else {
                            1
                        }
                    }
                    Fitted::Nulled => 0,
                }),
        };
        match r {
            Ok(c) => Some(c),
            Err(e) if e.code == crate::error::ErrorCode::NumberOverflow => Some(3),
            Err(_) => None,
        }
    }
}

/// The candidates of a column: the two standard ones and the resolved type if it is another decimal.
fn candidates_of(c: &ColumnInfo) -> Vec<Cand> {
    let mut v = vec![Cand::D128(38, 10), Cand::D256(76, 38)];
    match c.arrow_type {
        DataType::Decimal128(p, s) if (p, s) != (38, 10) => {
            v.push(Cand::D128(p, u8::try_from(s).unwrap_or(0)));
        }
        DataType::Decimal256(p, s) if (p, s) != (76, 38) => {
            v.push(Cand::D256(p, u8::try_from(s).unwrap_or(0)));
        }
        _ => {}
    }
    v
}

impl NumberProfile {
    fn new(c: &ColumnInfo) -> Self {
        Self {
            candidates: candidates_of(c)
                .into_iter()
                .map(|k| CandidateCounts {
                    label: k.label(),
                    exact: 0,
                    rounded: 0,
                    rounded_to_zero: 0,
                    int_too_large: 0,
                })
                .collect(),
            ..Self::default()
        }
    }

    fn add(&mut self, cands: &[Cand], bytes: &[u8]) {
        let Ok(n) = OraNumber::decode(bytes) else {
            self.invalid = self.invalid.saturating_add(1);
            return;
        };
        if n.is_integer() {
            self.integers = self.integers.saturating_add(1);
            if n.to_i64().is_some() {
                self.fit_int64 = self.fit_int64.saturating_add(1);
            }
        }
        if n.sig() >= 39 {
            self.sig_39_or_40 = self.sig_39_or_40.saturating_add(1);
        }
        self.max_sig = self.max_sig.max(n.sig());
        self.max_frac = self.max_frac.max(n.frac());
        self.max_int = self.max_int.max(n.int_digits());
        if let Some(lead) = n.leading_position() {
            let e = n.exp10();
            self.min_exp10 = Some(self.min_exp10.map_or(e, |m| m.min(e)));
            self.max_exp10 = Some(self.max_exp10.map_or(e, |m| m.max(e)));
            self.min_lead = Some(self.min_lead.map_or(lead, |m| m.min(lead)));
            self.max_lead = Some(self.max_lead.map_or(lead, |m| m.max(lead)));
        }
        for (k, out) in cands.iter().zip(self.candidates.iter_mut()) {
            match k.class(&n) {
                Some(0) => out.exact = out.exact.saturating_add(1),
                Some(1) => out.rounded = out.rounded.saturating_add(1),
                Some(2) => {
                    out.rounded = out.rounded.saturating_add(1);
                    out.rounded_to_zero = out.rounded_to_zero.saturating_add(1);
                }
                Some(_) => out.int_too_large = out.int_too_large.saturating_add(1),
                None => {}
            }
        }
    }
}

/// The counts of one column.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ColumnCounts {
    /// 1-based position in the declared order.
    pub position: u32,
    /// Type code of the descriptor.
    pub type_code: u16,
    /// Non-NULL values (LOB and LONG values included).
    pub values: u64,
    /// NULL fields.
    pub nulls: u64,
    /// DATE / TIMESTAMP family values with a year below 1.
    pub years_below_1: u64,
    /// INTERVAL YEAR TO MONTH values whose total months do not fit 32 bits.
    pub months_beyond_32_bits: u64,
    /// The column is declared `NUMBER` and no Decimal128 holds the declaration.
    pub no_decimal128_holds: bool,
    /// Short-form values of a CLOB / NCLOB column that `E-LOB-UNCONFIRMED` refuses.
    pub lob_refused_short: u64,
    /// Long-form values of a CLOB / NCLOB column that `E-LOB-UNCONFIRMED` refuses.
    pub lob_refused_long: u64,
    /// The smallest stored length (bytes) of a non-NULL value (a LONG chain and a LOB value by their total).
    pub len_min: Option<u64>,
    /// The largest stored length of a non-NULL value.
    pub len_max: Option<u64>,
    /// The sum of the stored lengths of the non-NULL values (the mean is `len_total / values`).
    pub len_total: u64,
    /// The NUMBER profile of a column of type 2.
    pub number: Option<NumberProfile>,
}

impl ColumnCounts {
    fn add_len(&mut self, len: u64) {
        self.len_min = Some(self.len_min.map_or(len, |m| m.min(len)));
        self.len_max = Some(self.len_max.map_or(len, |m| m.max(len)));
        self.len_total = self.len_total.saturating_add(len);
    }
}

/// The counts of one table, columns in declared order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCounts {
    /// Ordinal of the table section.
    pub ordinal: u32,
    /// Rows over all parts.
    pub rows: u64,
    /// One entry per column.
    pub columns: Vec<ColumnCounts>,
}

struct Counter<'a> {
    info: &'a [ColumnInfo],
    db_charset_id: u16,
    counts: Vec<ColumnCounts>,
    /// For each file position of the current part (index) the declared index.
    map: Vec<usize>,
    /// The candidate types of each column (empty unless the column is of type 2).
    cands: Vec<Vec<Cand>>,
    /// The total length of the chain in progress.
    chain_total: u64,
}

impl Counter<'_> {
    fn slot(&mut self, column: u32) -> Option<(usize, u16)> {
        let file = usize::try_from(column.saturating_sub(1)).ok()?;
        let i = *self.map.get(file)?;
        let t = self.info.get(i)?.type_code;
        Some((i, t))
    }

    fn bump(&mut self, i: usize, f: impl FnOnce(&mut ColumnCounts)) {
        if let Some(c) = self.counts.get_mut(i) {
            f(c);
        }
    }
}

/// The year of the DATE layout in the first 7 bytes of a value is below 1.
fn year_below_1(b: &[u8]) -> bool {
    b.get(..7)
        .and_then(|d| decode_date_fields(d).ok())
        .is_some_and(|d| d.year < 1)
}

impl RowSink for Counter<'_> {
    fn row_start(&mut self, _row: u64) {}

    fn null(&mut self, column: u32, _offset: u64) -> Result<()> {
        if let Some((i, _)) = self.slot(column) {
            self.bump(i, |c| c.nulls = c.nulls.saturating_add(1));
        }
        Ok(())
    }

    fn value(&mut self, column: u32, _offset: u64, bytes: &[u8]) -> Result<()> {
        let Some((i, type_code)) = self.slot(column) else {
            return Ok(());
        };
        match type_code {
            // a LOB locator: the value is counted in the value area
            112 | 113 => {}
            2 => {
                let len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
                let cands = self.cands.get(i).cloned().unwrap_or_default();
                self.bump(i, |c| {
                    c.values = c.values.saturating_add(1);
                    c.add_len(len);
                    if let Some(p) = c.number.as_mut() {
                        p.add(&cands, bytes);
                    }
                });
            }
            12 | 180 | 181 | 231 => {
                let below = year_below_1(bytes);
                let len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
                self.bump(i, |c| {
                    c.add_len(len);
                    c.values = c.values.saturating_add(1);
                    c.years_below_1 = c.years_below_1.saturating_add(u64::from(below));
                });
            }
            182 => {
                let beyond = IntervalYm::decode(bytes)
                    .ok()
                    .is_some_and(|v| v.total_months().is_err());
                let len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
                self.bump(i, |c| {
                    c.add_len(len);
                    c.values = c.values.saturating_add(1);
                    c.months_beyond_32_bits =
                        c.months_beyond_32_bits.saturating_add(u64::from(beyond));
                });
            }
            _ => {
                let len = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
                self.bump(i, |c| {
                    c.values = c.values.saturating_add(1);
                    c.add_len(len);
                });
            }
        }
        Ok(())
    }

    fn chain_piece(
        &mut self,
        column: u32,
        _offset: u64,
        piece: &[u8],
        first: bool,
        last: bool,
    ) -> Result<()> {
        if first {
            self.chain_total = 0;
        }
        self.chain_total = self
            .chain_total
            .saturating_add(u64::try_from(piece.len()).unwrap_or(u64::MAX));
        if first {
            if let Some((i, _)) = self.slot(column) {
                self.bump(i, |c| c.values = c.values.saturating_add(1));
            }
        }
        if last {
            let total = self.chain_total;
            if let Some((i, _)) = self.slot(column) {
                self.bump(i, |c| c.add_len(total));
            }
        }
        Ok(())
    }

    fn lob_start(
        &mut self,
        column: u32,
        _offset: u64,
        form: LobForm,
        total: u64,
    ) -> Result<LobAction> {
        if let Some((i, _)) = self.slot(column) {
            let refused = self
                .info
                .get(i)
                .is_some_and(|c| lob_value_unconfirmed(c, self.db_charset_id, form));
            self.bump(i, |c| {
                c.values = c.values.saturating_add(1);
                c.add_len(total);
                if refused {
                    match form {
                        LobForm::Short => {
                            c.lob_refused_short = c.lob_refused_short.saturating_add(1);
                        }
                        LobForm::Long => {
                            c.lob_refused_long = c.lob_refused_long.saturating_add(1);
                        }
                    }
                }
            });
        }
        Ok(LobAction::Skip)
    }

    fn row_end(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Is the column declared `NUMBER` in a way that no Decimal128 holds (ADR-30 item 3)?
fn declared_number_not_held(c: &ColumnInfo) -> bool {
    match c.declared {
        Some(DeclaredType::Number { precision, scale }) => matches!(
            declared_number(precision, scale, NumberType::DEFAULT),
            Some(NumberFit::NoDecimal128(_))
        ),
        _ => false,
    }
}

/// Counts the values of one table (all parts) as described in the module documentation. It reads every row
/// once, decodes nothing it does not need and writes nothing.
pub fn count_table(dump: &mut Dump, which: TableRef<'_>) -> Result<TableCounts> {
    let table = dump.table(which)?.clone();
    let db_charset_id = dump.info().db_charset_id;
    let counts = table
        .columns
        .iter()
        .map(|c| ColumnCounts {
            position: c.position,
            type_code: c.type_code,
            no_decimal128_holds: declared_number_not_held(c),
            number: (c.type_code == 2).then(|| NumberProfile::new(c)),
            ..ColumnCounts::default()
        })
        .collect();
    let cands = table
        .columns
        .iter()
        .map(|c| {
            if c.type_code == 2 {
                candidates_of(c)
            } else {
                Vec::new()
            }
        })
        .collect();
    let mut counter = Counter {
        info: &table.columns,
        db_charset_id,
        counts,
        map: Vec::new(),
        cands,
        chain_total: 0,
    };
    let mut rows = 0u64;
    for (k, part) in table.parts.iter().enumerate() {
        counter.map = part
            .file_order
            .iter()
            .map(|&d| usize::try_from(d).unwrap_or(usize::MAX))
            .collect();
        let mut cursor = dump.part_rows(TableRef::ByOrdinal(table.ordinal), k)?;
        while cursor.next_row_with(&mut counter)?.is_some() {}
        rows = rows.saturating_add(cursor.rows_read());
    }
    Ok(TableCounts {
        ordinal: table.ordinal,
        rows,
        columns: counter.counts,
    })
}

/// One candidate type over all NUMBER columns of a dump (the summary line of architecture 5.2b).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSummary {
    /// The type as text.
    pub label: String,
    /// Columns that have this candidate.
    pub columns: u64,
    /// Values the type holds exactly, over those columns.
    pub exact: u64,
    /// Values `round` would change.
    pub rounded: u64,
    /// Of those, the values that become 0.
    pub rounded_to_zero: u64,
    /// Values whose integer part does not fit.
    pub int_too_large: u64,
    /// Columns in which `round` changes a value or an integer part does not fit: those need an override
    /// (`number_types`) if the candidate is to be used.
    pub columns_needing_override: u64,
}

/// The summary over all NUMBER columns of the tables counted: the same counts in total, per candidate type, and
/// the number of columns that would need an override under each. The two standard candidates come first.
pub fn number_summary(tables: &[TableCounts]) -> Vec<CandidateSummary> {
    let mut out: Vec<CandidateSummary> = Vec::new();
    for t in tables {
        for c in &t.columns {
            let Some(p) = &c.number else { continue };
            for k in &p.candidates {
                let i = match out.iter().position(|s| s.label == k.label) {
                    Some(i) => i,
                    None => {
                        out.push(CandidateSummary {
                            label: k.label.clone(),
                            columns: 0,
                            exact: 0,
                            rounded: 0,
                            rounded_to_zero: 0,
                            int_too_large: 0,
                            columns_needing_override: 0,
                        });
                        out.len().saturating_sub(1)
                    }
                };
                if let Some(s) = out.get_mut(i) {
                    s.columns = s.columns.saturating_add(1);
                    s.exact = s.exact.saturating_add(k.exact);
                    s.rounded = s.rounded.saturating_add(k.rounded);
                    s.rounded_to_zero = s.rounded_to_zero.saturating_add(k.rounded_to_zero);
                    s.int_too_large = s.int_too_large.saturating_add(k.int_too_large);
                    if k.rounded > 0 || k.int_too_large > 0 {
                        s.columns_needing_override = s.columns_needing_override.saturating_add(1);
                    }
                }
            }
        }
    }
    let rank = |l: &str| match l {
        "decimal(38,10)" => 0,
        "decimal256(76,38)" => 1,
        _ => 2,
    };
    out.sort_by(|a, b| {
        rank(&a.label)
            .cmp(&rank(&b.label))
            .then(a.label.cmp(&b.label))
    });
    out
}
