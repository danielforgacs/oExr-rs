use std::fs::write;


const MAGIC_NUMBER: [u8;4] = [0x76, 0x2f, 0x31, 0x01];
const VERSION: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

struct Exr {
    magic_number: [u8;4],
    version: [u8;4],
}

impl Exr {
    fn new() -> Self {
        Self {
            magic_number: MAGIC_NUMBER,
            version: VERSION,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend(self.magic_number);
        buffer.extend(self.version);
        buffer
    }
}

fn main() {
    let exr = Exr::new();
    let data = exr.serialize();
    write("test_result.exr", data).unwrap();
}
