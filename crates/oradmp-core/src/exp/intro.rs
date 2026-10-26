//! List K as data (ADR-9, architecture 2.4): the lines that introduce a field [I6].
//!
//! The embedded list is `data/exp_intro_lines.txt`; `IntroList::parse` reads the same format, which
//! `Options.extra_intro_lines` (task T-11) will use at run time.

use crate::error::{Error, ErrorCode, Result};

/// The embedded list, one entry per line: `kind:length:forms:text`.
const BUILTIN: &str = include_str!("../../data/exp_intro_lines.txt");

/// How the text of an entry is compared with a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchKind {
    /// The line starts with the text.
    Prefix,
    /// The line is the text.
    Exact,
    /// The line ends with the text.
    Suffix,
}

/// The forms of field an entry allows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FormSet {
    /// Plain: u16 LE length, bytes, LF [I3].
    pub plain: bool,
    /// Marked: `fd ff`, pieces, LF [I4].
    pub marked: bool,
    /// Unmarked pieces, LF [I5].
    pub unmarked: bool,
}

/// One entry of list K.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntroEntry {
    /// Comparison kind.
    pub kind: MatchKind,
    /// The text, bytes of the client character set (ASCII in every entry).
    pub text: Vec<u8>,
    /// Allowed field forms.
    pub forms: FormSet,
}

impl IntroEntry {
    /// Does the line (without its LF) match this entry?
    pub fn matches(&self, line: &[u8]) -> bool {
        match self.kind {
            MatchKind::Prefix => line.starts_with(&self.text),
            MatchKind::Exact => line == self.text.as_slice(),
            MatchKind::Suffix => line.ends_with(&self.text),
        }
    }
}

/// List K.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IntroList {
    entries: Vec<IntroEntry>,
}

fn config(message: &'static str) -> Error {
    Error::new(ErrorCode::Config, message)
}

impl IntroList {
    /// The embedded list.
    pub fn builtin() -> Result<Self> {
        Self::parse(BUILTIN)
    }

    /// The embedded list followed by the entries of `extra` (`Options.extra_intro_lines`, architecture 2.4):
    /// one entry per string in the form of the embedded file. A string that holds a line break, is empty, is
    /// a comment or does not parse is `E-CONFIG`.
    pub fn with_extra(extra: &[String]) -> Result<Self> {
        let mut list = Self::builtin()?;
        for line in extra {
            if line.is_empty() || line.starts_with('#') || line.contains(['\n', '\r']) {
                return Err(config(
                    "extra_intro_lines: one entry of the form kind:length:forms:text per string",
                ));
            }
            let one = Self::parse(line)?;
            if one.entries.len() != 1 {
                return Err(config("extra_intro_lines: one entry per string"));
            }
            list.entries.extend(one.entries);
        }
        Ok(list)
    }

    /// An empty list: every field is then found by rule G1.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Parses `kind:length:forms:text` lines. Blank lines and lines starting with `#` are ignored; the text
    /// is not trimmed and its byte length must equal the stated length.
    pub fn parse(text: &str) -> Result<Self> {
        let mut entries = Vec::new();
        for line in text.split('\n') {
            let line = line.strip_suffix('\r').unwrap_or(line);
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.splitn(4, ':');
            let (Some(kind), Some(len), Some(forms), Some(body)) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            else {
                return Err(config("intro list entry needs kind:length:forms:text"));
            };
            let kind = match kind {
                "prefix" => MatchKind::Prefix,
                "exact" => MatchKind::Exact,
                "suffix" => MatchKind::Suffix,
                _ => return Err(config("intro list entry: unknown kind")),
            };
            let len: usize = len
                .parse()
                .map_err(|_| config("intro list entry: length is not a number"))?;
            if body.len() != len {
                return Err(config("intro list entry: length does not match the text"));
            }
            let mut set = FormSet::default();
            for f in forms.split('|') {
                match f {
                    "plain" => set.plain = true,
                    "marked" => set.marked = true,
                    "unmarked" => set.unmarked = true,
                    _ => return Err(config("intro list entry: unknown form")),
                }
            }
            if !(set.plain || set.marked || set.unmarked) {
                return Err(config("intro list entry: no form"));
            }
            entries.push(IntroEntry {
                kind,
                text: body.as_bytes().to_vec(),
                forms: set,
            });
        }
        Ok(Self { entries })
    }

    /// Adds an entry at the end (run-time extension).
    pub fn push(&mut self, entry: IntroEntry) {
        self.entries.push(entry);
    }

    /// The entries in order.
    pub fn entries(&self) -> &[IntroEntry] {
        &self.entries
    }

    /// Index of the first entry that matches the line.
    pub fn find(&self, line: &[u8]) -> Option<usize> {
        self.entries.iter().position(|e| e.matches(line))
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
    fn builtin_parses_and_has_eight_entries() {
        let l = IntroList::builtin().unwrap();
        assert_eq!(l.entries().len(), 8);
        assert_eq!(l.find(b"ALTER TABLE \"T\" MODIFY DEFAULT"), Some(0));
        assert_eq!(l.find(b"ALTER TABLE \"T\" ADD"), None);
        assert_eq!(l.find(b"CALTER TABLE"), Some(3));
        assert_eq!(l.find(b"CALTER TABLE "), None);
    }

    #[test]
    fn length_and_form_are_checked() {
        assert!(IntroList::parse("prefix:3:plain:ab").is_err());
        assert!(IntroList::parse("prefix:2:flat:ab").is_err());
        assert!(IntroList::parse("middle:2:plain:ab").is_err());
        assert!(IntroList::parse("prefix:2:plain").is_err());
        assert!(IntroList::parse("prefix:3:plain:ab ").is_ok());
    }
}
