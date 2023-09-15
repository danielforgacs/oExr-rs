//! Exr Magic Number
//! <https://openexr.com/en/latest/OpenEXRFileLayout.html#magic-number>
//! 20000630, int (decimal).
//! the first four bytes of an OpenEXR file are always
//! 0x76, 0x2f, 0x31 and 0x01.

use super::basic_data_types;

const MAGIC_NUMBER: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_number_matches_bytes_in_docs() {
        let expected: basic_data_types::Int = 20000630;
        assert_eq!(MAGIC_NUMBER, expected.to_le_bytes());
    }
}
