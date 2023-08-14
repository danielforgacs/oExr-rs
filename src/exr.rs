const MAGIC_NUMBER: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];

pub struct Exr {
    left_over_bytes: Vec<u8>,
}

impl Exr {
    pub fn deserialize(mut data: Vec<u8>) -> Exr {
        data.drain(..4);
        Self { left_over_bytes: data }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = MAGIC_NUMBER.to_vec();
        data.extend(self.left_over_bytes.clone());
        data
    }
}
