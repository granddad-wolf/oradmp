//! Declared types and the type mapping (architecture 5.1 and 5.5, task T-5).
//!
//! * [`declared`]: the column list of the `CREATE TABLE` line [T9].
//! * [`map`]: declared type and descriptor to Arrow type; the NUMBER options.
//! * [`resolve`]: one table: agreement with the descriptors, matching by name, column order, fallbacks.

pub mod declared;
pub mod map;
pub mod resolve;
