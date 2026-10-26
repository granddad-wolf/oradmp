//! The `INSERT INTO "table" ("col", ...) VALUES (:1, ...)` line that precedes the table data [T3].
//!
//! Identifiers are quoted; a name can hold `,`, `:`, `(`, a space and bytes of the client character set, so
//! the line is read by its grammar, never split on commas. A name ends at the next `"`.
//! The bytes are kept as they are; decoding is not done here.

/// The parts of an `INSERT INTO` line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InsertLine {
    /// Table name as written (client character set, without quotes).
    pub table: Vec<u8>,
    /// Column names as written, in order.
    pub columns: Vec<Vec<u8>>,
    /// Number of bind placeholders `:n` after `VALUES`.
    pub placeholders: usize,
}

struct Cursor<'a> {
    b: &'a [u8],
    i: usize,
}

impl<'a> Cursor<'a> {
    fn rest(&self) -> &'a [u8] {
        self.b.get(self.i..).unwrap_or(&[])
    }

    fn eat(&mut self, lit: &[u8]) -> bool {
        if self.rest().starts_with(lit) {
            self.i = self.i.saturating_add(lit.len());
            true
        } else {
            false
        }
    }

    /// `"name"` -> name.
    fn quoted(&mut self) -> Option<Vec<u8>> {
        if !self.eat(b"\"") {
            return None;
        }
        let rest = self.rest();
        let n = rest.iter().position(|&c| c == b'"')?;
        let name = rest.get(..n)?.to_vec();
        self.i = self.i.saturating_add(n).saturating_add(1);
        Some(name)
    }

    fn at_end(&self) -> bool {
        self.i >= self.b.len()
    }
}

/// Parses the line (without its LF). `None` if it does not have the form of rule T3.
pub fn parse_insert_line(line: &[u8]) -> Option<InsertLine> {
    let mut c = Cursor { b: line, i: 0 };
    if !c.eat(b"INSERT INTO ") {
        return None;
    }
    let table = c.quoted()?;
    if !c.eat(b" (") {
        return None;
    }
    let mut columns = vec![c.quoted()?];
    while c.eat(b", ") {
        columns.push(c.quoted()?);
    }
    if !c.eat(b") VALUES (") {
        return None;
    }
    let mut placeholders = 0usize;
    loop {
        if !c.eat(b":") {
            return None;
        }
        let digits = c.rest().iter().take_while(|d| d.is_ascii_digit()).count();
        if digits == 0 {
            return None;
        }
        c.i = c.i.saturating_add(digits);
        placeholders = placeholders.saturating_add(1);
        if !c.eat(b", ") {
            break;
        }
    }
    if c.eat(b")") && c.at_end() {
        Some(InsertLine {
            table,
            columns,
            placeholders,
        })
    } else {
        None
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

    #[test]
    fn names_may_hold_commas_colons_and_parentheses() {
        let l =
            parse_insert_line(br#"INSERT INTO "T" ("a,b", "c:1", "low,er(x") VALUES (:1, :2, :3)"#)
                .unwrap();
        assert_eq!(l.table, b"T");
        assert_eq!(
            l.columns,
            vec![b"a,b".to_vec(), b"c:1".to_vec(), b"low,er(x".to_vec()]
        );
        assert_eq!(l.placeholders, 3);
    }

    #[test]
    fn non_ascii_bytes_are_kept() {
        let l = parse_insert_line(b"INSERT INTO \"\xd2\xc0\" (\"\xca_1\") VALUES (:1)").unwrap();
        assert_eq!(l.table, b"\xd2\xc0");
        assert_eq!(l.columns, vec![b"\xca_1".to_vec()]);
    }

    #[test]
    fn damaged_lines_give_none_and_never_panic() {
        for bad in [
            &b""[..],
            b"INSERT INTO ",
            b"INSERT INTO \"T\"",
            b"INSERT INTO \"T\" (\"a\" VALUES (:1)",
            b"INSERT INTO \"T\" (\"a\") VALUES (:)",
            b"INSERT INTO \"T\" (\"a\") VALUES (:1",
            b"INSERT INTO \"T\" (\"a\") VALUES (:1) x",
            b"INSERT INTO \"T\" (\"a\", ) VALUES (:1)",
            b"INSERT INTO \"T (\"a\") VALUES (:1)",
        ] {
            assert!(parse_insert_line(bad).is_none(), "{bad:?}");
        }
    }
}
