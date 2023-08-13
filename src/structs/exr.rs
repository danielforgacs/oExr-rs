const MAGIC_NUMBER: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];
const VERSION: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

pub struct Exr {
    magic_number: [u8;4],
    version: [u8;4],
    header: Vec<u8>,
    offset_tables: Vec<u8>,
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
        let header = data
            .drain(..288)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let offset_tables = data
            .drain(..25)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        Self {
            magic_number,
            version,
            header,
            offset_tables,
            leftover_bytes: data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(self.magic_number);
        buffer.extend(self.version);
        buffer.extend(self.header.clone());
        buffer.extend(self.offset_tables.clone());
        buffer.extend(self.leftover_bytes.clone());
        buffer
    }
}
