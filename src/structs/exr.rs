use super::header;

const MAGIC_NUMBER: [u8; 4] = [0x76, 0x2f, 0x31, 0x01];
const VERSION: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

pub struct Exr {
    format_version: [u8;4],
    header: header::Header,
    offset_tables: Vec<u8>,
    pixel_data: Vec<u8>,
}

impl Exr {
    pub fn deserialize(mut data: Vec<u8>) -> Exr {
        if data[..4] != MAGIC_NUMBER {
            panic!("Wrong magic number!");
        }
        data.drain(..4);
        let version: [u8; 4] = data
            .drain(..4)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        println!("{:#010x?}", &version);
        let version2 = u32::from_le_bytes(version);
        println!("{:#034b}", &version2);
        println!("{}", &version2);
        let header = header::Header::deserialize(&mut data);
        data.drain(..1);
        let offset_tables = data
            .drain(..25)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        Self {
            format_version: version,
            header,
            offset_tables,
            pixel_data: data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(MAGIC_NUMBER);
        buffer.extend(self.format_version);
        buffer.extend(self.header.serialize());
        buffer.push(0);
        buffer.extend(self.offset_tables.clone());
        buffer.extend(self.pixel_data.clone());
        buffer
    }
}
