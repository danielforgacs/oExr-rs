const MAGIC_NUMBER: [u8;4] = [0x76, 0x2f, 0x31, 0x01];
const VERSION: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

pub struct Exr {
    magic_number: [u8;4],
    version: [u8;4],
}

impl Exr {
    pub fn new() -> Self {
        Self {
            magic_number: MAGIC_NUMBER,
            version: VERSION,
        }
    }

    pub fn deserialize(mut data: Vec<u8>) -> Exr {
        let magic_number: [u8; 4] = data
            .drain(..4)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let version: [u8; 4] = data
            .drain(..4)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        Self {
            magic_number,
            version,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(self.magic_number);
        buffer.extend(self.version);
        buffer
    }
}