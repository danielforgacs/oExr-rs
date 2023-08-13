use super::header;

const MAGIC_NUMBER: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];
const VERSION: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

pub struct Exr {
    magic_number: [u8;4],
    version: [u8;4],
    header: header::Header,
    offset_tables: Vec<u8>,
    pixel_data: Vec<u8>,
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
        let header = header::Header::deserialize(&mut data);
        data.drain(..1);
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
            pixel_data: data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let length = self.magic_number.len()
            + self.version.len()
            + self.header.serialize().len();
        println!(":: magic length: {}", self.magic_number.len());
        println!(":: version length: {}", self.version.len());
        println!(":: header length: {}", self.header.serialize().len());
        println!(":: magic + version + header length: {}", length);
        buffer.extend(self.magic_number);
        buffer.extend(self.version);
        buffer.extend(self.header.serialize());
        buffer.push(0);
        buffer.extend(self.offset_tables.clone());
        buffer.extend(self.pixel_data.clone());
        buffer
    }
}
