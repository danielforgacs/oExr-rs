use crate::consts;
use super::vfield;

pub struct Exr {
    magic_number: [u8; 4],
    version_field: vfield::VersionField,
}

impl Exr {
    pub fn new() -> Self {
        Self {
            magic_number: consts::MAGIC_NUMBER,
            version_field: vfield::VersionField::new(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut exr_bytes = vec![];
        exr_bytes.extend(self.magic_number);
        exr_bytes.extend(self.version_field.serialize());
        exr_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exr() {
        let exr = Exr::new();
        let expected = vec![
            0x76, 0x2f, 0x31, 0x01,
            0x02, 0x00, 0x00, 0x00,
        ];
        assert_eq!(exr.serialize(), expected);
    }
}
