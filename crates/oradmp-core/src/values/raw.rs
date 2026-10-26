//! RAW [Y6] and the check "longer than declared" [C8], architecture 5.1 and 5.4.

/// The value of a RAW field: the bytes, unchanged [Y6].
pub fn decode_raw(bytes: &[u8]) -> &[u8] {
    bytes
}

/// `W-LONGER-THAN-DECLARED` (ADR-26): for the types 1, 23 and 96 the descriptor length is a true maximum, so a
/// field longer than it is reported (once per column by the caller) and still delivered as read. The
/// comparison is between the **byte** length and the **descriptor** length, never the declared `n` [C8].
pub fn longer_than_declared(field_len: usize, descriptor_max: u16) -> bool {
    field_len > usize::from(descriptor_max)
}
