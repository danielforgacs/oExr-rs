use crate::consts;

pub struct Exr {
    magic_number: [u8; 4],
}

impl Exr {
    pub fn new() -> Self {
        Self {
            magic_number: consts::MAGIC_NUMBER,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut exr_bytes = vec![];
        exr_bytes.extend(self.magic_number);
        exr_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exr() {
        let exr = Exr::new();
        let expected = vec![0x76, 0x2f, 0x31, 0x01];
        assert_eq!(exr.serialize(), expected);
    }
}
