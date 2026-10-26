//! The column list of the `CREATE TABLE` line [T9], architecture 5.5 steps 1 and 2.
//!
//! Grammar (rule T9, `exp.md` 4.7): `CREATE TABLE ` qname ` (` coldef (`, ` coldef)* `)` + two spaces + storage
//! text; coldef = qname + one space + type + optionally ` NOT NULL ENABLE`; a qname runs from `"` to the next
//! `"`. The parser never splits on commas or parentheses outside this grammar: a name is read as a quoted
//! string, a type by its fixed forms.
//!
//! A type text outside the forms is kept (`ColumnDef::type_text`) with `kind: None`: the grammar does not say
//! where such a text ends, so this parser takes it up to the first `, "` or `)  ` (choice recorded in
//! `implementer-notes.md`, N-16). A line that breaks the frame (no `CREATE TABLE `, no quoted name, no space,
//! no `, ` or `)  ` after a definition) is a [`ParseFailure`] and the caller falls back for the whole table.
//! Nothing here reads or reports a value of the file.

/// Fractional-second / zone variant of a declared TIMESTAMP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimestampZone {
    /// `TIMESTAMP (f)`.
    None,
    /// `TIMESTAMP (f) WITH TIME ZONE`.
    WithTimeZone,
    /// `TIMESTAMP (f) WITH LOCAL TIME ZONE`.
    WithLocalTimeZone,
}

/// A type text of rule T9 and what it gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclaredType {
    /// `NUMBER` (none, none), `NUMBER(p, s)` (p, s), `NUMBER(*,s)` (none, s).
    Number {
        /// Precision; `None` for `NUMBER` and `NUMBER(*,s)`.
        precision: Option<u32>,
        /// Scale; `None` for plain `NUMBER`. May be negative or above the precision.
        scale: Option<i32>,
    },
    /// `FLOAT(b)`, binary precision b.
    Float {
        /// Binary precision.
        binary_precision: u32,
    },
    /// `VARCHAR2(n)` or `VARCHAR2(n CHAR)`.
    Varchar2 {
        /// n.
        n: u32,
        /// Character semantics (`n CHAR`).
        char_semantics: bool,
    },
    /// `NVARCHAR2(n)`.
    NVarchar2 {
        /// n characters.
        n: u32,
    },
    /// `CHAR(n)` or `CHAR(n CHAR)`.
    Char {
        /// n.
        n: u32,
        /// Character semantics (`n CHAR`).
        char_semantics: bool,
    },
    /// `NCHAR(n)`.
    NChar {
        /// n characters.
        n: u32,
    },
    /// `RAW(n)`.
    Raw {
        /// n bytes.
        n: u32,
    },
    /// `DATE`.
    Date,
    /// `TIMESTAMP (f)` with an optional zone suffix.
    Timestamp {
        /// Fractional-second precision f.
        fractional: u32,
        /// Zone variant.
        zone: TimestampZone,
    },
    /// `CLOB`.
    Clob,
    /// `NCLOB`.
    NClob,
    /// `BLOB`.
    Blob,
    /// `LONG`.
    Long,
    /// `LONG RAW`.
    LongRaw,
    /// `ROWID`.
    Rowid,
    /// `UROWID(n)`.
    Urowid {
        /// n.
        n: u32,
    },
    /// `INTERVAL YEAR (p) TO MONTH ` (the text ends with a space).
    IntervalYearToMonth {
        /// Leading precision p.
        precision: u32,
    },
    /// `INTERVAL DAY (p) TO SECOND (f)`.
    IntervalDayToSecond {
        /// Leading precision p.
        precision: u32,
        /// Fractional-second precision f.
        fractional: u32,
    },
}

impl DeclaredType {
    /// True for `LONG` and `LONG RAW`.
    pub fn is_long(&self) -> bool {
        matches!(self, Self::Long | Self::LongRaw)
    }
}

/// One column definition of the list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDef {
    /// The name as written (client character set, without quotes).
    pub name: Vec<u8>,
    /// Offset of the opening quote of the name inside the line.
    pub offset: usize,
    /// The type text as written, without ` NOT NULL ENABLE` (an interval text keeps its trailing space).
    pub type_text: Vec<u8>,
    /// The type, or `None` when the text is outside the forms of rule T9.
    pub kind: Option<DeclaredType>,
    /// ` NOT NULL ENABLE` followed the type.
    pub not_null: bool,
}

/// The parsed column list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnList {
    /// The table name as written.
    pub table: Vec<u8>,
    /// The definitions in declared order.
    pub columns: Vec<ColumnDef>,
    /// Offset inside the line of the storage text (after `)` and the two spaces).
    pub storage_offset: usize,
}

/// Why the line does not parse, with the offset inside the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseFailure {
    /// Offset inside the line where the grammar broke.
    pub offset: usize,
    /// Static reason.
    pub reason: &'static str,
}

const NOT_NULL: &[u8] = b" NOT NULL ENABLE";

#[derive(Clone, Copy)]
struct Cur<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Cur<'a> {
    fn rest(&self) -> &'a [u8] {
        self.b.get(self.i..).unwrap_or(&[])
    }

    fn eat(&mut self, lit: &[u8]) -> Option<()> {
        if self.rest().starts_with(lit) {
            self.i = self.i.saturating_add(lit.len());
            Some(())
        } else {
            None
        }
    }

    fn digits(&mut self) -> Option<u32> {
        let mut n = 0u32;
        let mut any = false;
        while let Some(&c) = self.rest().first() {
            if !c.is_ascii_digit() {
                break;
            }
            n = n
                .checked_mul(10)?
                .checked_add(u32::from(c.saturating_sub(b'0')))?;
            any = true;
            self.i = self.i.saturating_add(1);
        }
        any.then_some(n)
    }

    fn signed(&mut self) -> Option<i32> {
        let neg = self.eat(b"-").is_some();
        let v = i32::try_from(self.digits()?).ok()?;
        if neg {
            v.checked_neg()
        } else {
            Some(v)
        }
    }

    /// `"name"`.
    fn quoted(&mut self) -> Option<Vec<u8>> {
        self.eat(b"\"")?;
        let rest = self.rest();
        let n = rest.iter().position(|&c| c == b'"')?;
        let name = rest.get(..n)?.to_vec();
        self.i = self.i.saturating_add(n).saturating_add(1);
        Some(name)
    }

    /// Runs `f`; on failure the cursor stays where it was.
    fn attempt<T>(&mut self, f: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        let start = *self;
        let r = f(self);
        if r.is_none() {
            *self = start;
        }
        r
    }
}

/// `NAME(n)` for the families with one length.
fn one_arg(c: &mut Cur<'_>, name: &[u8]) -> Option<u32> {
    c.attempt(|c| {
        c.eat(name)?;
        c.eat(b"(")?;
        let n = c.digits()?;
        c.eat(b")")?;
        Some(n)
    })
}

/// `NAME(n)` or `NAME(n CHAR)`.
fn length_semantics(c: &mut Cur<'_>, name: &[u8]) -> Option<(u32, bool)> {
    c.attempt(|c| {
        c.eat(name)?;
        c.eat(b"(")?;
        let n = c.digits()?;
        let chars = c.eat(b" CHAR").is_some();
        c.eat(b")")?;
        Some((n, chars))
    })
}

fn timestamp(c: &mut Cur<'_>) -> Option<DeclaredType> {
    c.attempt(|c| {
        c.eat(b"TIMESTAMP (")?;
        let fractional = c.digits()?;
        c.eat(b")")?;
        let zone = if c.eat(b" WITH TIME ZONE").is_some() {
            TimestampZone::WithTimeZone
        } else if c.eat(b" WITH LOCAL TIME ZONE").is_some() {
            TimestampZone::WithLocalTimeZone
        } else {
            TimestampZone::None
        };
        Some(DeclaredType::Timestamp { fractional, zone })
    })
}

/// A type of the forms of rule T9 at the cursor, or `None`. Does not look at what follows.
fn known_type(c: &mut Cur<'_>) -> Option<DeclaredType> {
    use DeclaredType as D;
    if let Some(t) = c.attempt(|c| {
        c.eat(b"NUMBER(")?;
        let p = c.digits()?;
        c.eat(b", ")?;
        let s = c.signed()?;
        c.eat(b")")?;
        Some(D::Number {
            precision: Some(p),
            scale: Some(s),
        })
    }) {
        return Some(t);
    }
    if let Some(t) = c.attempt(|c| {
        c.eat(b"NUMBER(*,")?;
        let s = c.signed()?;
        c.eat(b")")?;
        Some(D::Number {
            precision: None,
            scale: Some(s),
        })
    }) {
        return Some(t);
    }
    if let Some(b) = one_arg(c, b"FLOAT") {
        return Some(D::Float {
            binary_precision: b,
        });
    }
    // NVARCHAR2 and NCHAR cannot be confused with VARCHAR2 and CHAR: each form is matched from its first byte.
    if let Some((n, char_semantics)) = length_semantics(c, b"VARCHAR2") {
        return Some(D::Varchar2 { n, char_semantics });
    }
    if let Some(n) = one_arg(c, b"NVARCHAR2") {
        return Some(D::NVarchar2 { n });
    }
    if let Some((n, char_semantics)) = length_semantics(c, b"CHAR") {
        return Some(D::Char { n, char_semantics });
    }
    if let Some(n) = one_arg(c, b"NCHAR") {
        return Some(D::NChar { n });
    }
    if let Some(n) = one_arg(c, b"RAW") {
        return Some(D::Raw { n });
    }
    if let Some(t) = timestamp(c) {
        return Some(t);
    }
    for (lit, t) in [
        (&b"DATE"[..], D::Date),
        (b"NCLOB", D::NClob),
        (b"CLOB", D::Clob),
        (b"BLOB", D::Blob),
        (b"LONG RAW", D::LongRaw),
        (b"LONG", D::Long),
    ] {
        if c.eat(lit).is_some() {
            return Some(t);
        }
    }
    if let Some(n) = one_arg(c, b"UROWID") {
        return Some(D::Urowid { n });
    }
    if c.eat(b"ROWID").is_some() {
        return Some(D::Rowid);
    }
    if let Some(t) = c.attempt(|c| {
        c.eat(b"INTERVAL YEAR (")?;
        let precision = c.digits()?;
        c.eat(b") TO MONTH ")?;
        Some(D::IntervalYearToMonth { precision })
    }) {
        return Some(t);
    }
    if let Some(t) = c.attempt(|c| {
        c.eat(b"INTERVAL DAY (")?;
        let precision = c.digits()?;
        c.eat(b") TO SECOND (")?;
        let fractional = c.digits()?;
        c.eat(b")")?;
        Some(D::IntervalDayToSecond {
            precision,
            fractional,
        })
    }) {
        return Some(t);
    }
    c.eat(b"NUMBER")?;
    Some(D::Number {
        precision: None,
        scale: None,
    })
}

/// After a definition: `, ` (more) or `)  ` (end). Returns true at the end.
fn separator(c: &mut Cur<'_>) -> Option<bool> {
    if c.eat(b", ").is_some() {
        Some(false)
    } else if c.eat(b")  ").is_some() {
        Some(true)
    } else {
        None
    }
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn fail<T>(c: &Cur<'_>, reason: &'static str) -> std::result::Result<T, ParseFailure> {
    Err(ParseFailure {
        offset: c.i,
        reason,
    })
}

/// Parses the column list of a `CREATE TABLE` line (without its LF).
pub fn parse_column_list(line: &[u8]) -> std::result::Result<ColumnList, ParseFailure> {
    let mut c = Cur { b: line, i: 0 };
    if c.eat(b"CREATE TABLE ").is_none() {
        return fail(&c, "the line does not start with CREATE TABLE");
    }
    let Some(table) = c.quoted() else {
        return fail(&c, "no quoted table name");
    };
    if c.eat(b" (").is_none() {
        return fail(&c, "no ' (' after the table name");
    }
    let mut columns = Vec::new();
    loop {
        let offset = c.i;
        let Some(name) = c.quoted() else {
            return fail(&c, "no quoted column name");
        };
        if c.eat(b" ").is_none() {
            return fail(&c, "no space after the column name");
        }
        let type_start = c.i;
        // A known type counts only if a definition can end behind it.
        let known = c.attempt(|c| {
            let kind = known_type(c)?;
            let type_end = c.i;
            let not_null = c.eat(NOT_NULL).is_some();
            // an interval with NOT NULL ENABLE: the form is not in the fixtures (R-039)
            let interval = matches!(
                kind,
                DeclaredType::IntervalYearToMonth { .. } | DeclaredType::IntervalDayToSecond { .. }
            );
            let interval_nn = interval && (not_null || c.rest().starts_with(b"NOT NULL ENABLE"));
            if interval_nn {
                return None;
            }
            let end = separator(c)?;
            Some((kind, type_end, not_null, end))
        });
        let (kind, type_text, not_null, end) = match known {
            Some((kind, type_end, not_null, end)) => (
                Some(kind),
                line.get(type_start..type_end).unwrap_or(&[]).to_vec(),
                not_null,
                end,
            ),
            None => {
                // outside the forms: the text runs to the next `, "` or `)  `
                c.i = type_start;
                let rest = c.rest();
                let comma = find(rest, b", \"");
                let close = find(rest, b")  ");
                let (len, end) = match (comma, close) {
                    (Some(a), Some(b)) if a < b => (a, false),
                    (_, Some(b)) => (b, true),
                    (Some(a), None) => (a, false),
                    (None, None) => return fail(&c, "a type text that no separator ends"),
                };
                let mut text = rest.get(..len).unwrap_or(&[]);
                let mut not_null = false;
                if let Some(t) = text.strip_suffix(NOT_NULL) {
                    text = t;
                    not_null = true;
                }
                c.i = c.i.saturating_add(len);
                let sep: &[u8] = if end { b")  " } else { b", " };
                c.eat(sep);
                (None, text.to_vec(), not_null, end)
            }
        };
        columns.push(ColumnDef {
            name,
            offset,
            type_text,
            kind,
            not_null,
        });
        if end {
            return Ok(ColumnList {
                table,
                columns,
                storage_offset: c.i,
            });
        }
    }
}
