use std::convert::TryFrom;

/// Decodes byte characters as ascii characters.
/// if a byte cannot be parsed, it replaced as a character '?'.
pub fn decode_ascii_force(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|c| char::try_from(*c).unwrap_or('?'))
        .collect()
}
