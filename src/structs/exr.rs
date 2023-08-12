use super::header::*;

const MAGIC_NUMBER: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];
const VERSION: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

pub struct Exr {
    magic_number: [u8;4],
    version: [u8;4],
    // header: Header,
    leftover_bytes: Vec<u8>,
}

impl Exr {
    pub fn deserialize(mut data: Vec<u8>) -> Exr {
        let magic_number: [u8; 4] = data
            .drain(..4)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        if magic_number != MAGIC_NUMBER {
            panic!("Magic number does not match.");
        };
        let version: [u8; 4] = data
            .drain(..4)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        if version != VERSION {
            panic!("Version does not match.");
        };
        // let header = Header::deserialize(data);
        Self {
            magic_number,
            version,
            // header,
            leftover_bytes: data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(self.magic_number);
        buffer.extend(self.version);
        // buffer.extend(self.header.serialize());
        buffer.extend(self.leftover_bytes.clone());
        buffer
    }
}
