//! Layer 6 (architecture 2.1): pure functions from the bytes of one field to typed values.
//!
//! Nothing here reads a file or knows the `exp` framing: the input is the value bytes of one column field
//! (its length is given by the slice), the output is a lossless intermediate or a number, text or epoch
//! value. A value error carries a stable code and a static message and no context beyond that; the caller
//! (task T-6) adds file offset, table, column and row with [`Error::at`](crate::Error::at) and friends.
//! No message ever holds a value read from the file.
//!
//! * [`number`]: NUMBER [Y1], architecture 5.2 and 5.2a.
//! * [`datetime`]: DATE, TIMESTAMP, TIMESTAMP WITH (LOCAL) TIME ZONE [Y2, Y5, Y7, Y8], architecture 5.3.
//! * [`raw`]: RAW [Y6] and the "longer than declared" check [C8].
//! * [`long`]: LONG and LONG RAW values [Y9, R2] (assembly of a piece chain, length limits).
//! * [`interval`]: INTERVAL YEAR TO MONTH and DAY TO SECOND [Y10], architecture 5.7.
//! * [`rowid`]: ROWID and UROWID [Y11], architecture 5.7.

pub mod datetime;
pub mod interval;
pub mod long;
pub mod number;
pub mod raw;
pub mod rowid;
