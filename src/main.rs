use std::fs::write;
use std::collections::HashMap;


const MAGIC_NUMBER: [u8;4] = [0x76, 0x2f, 0x31, 0x01];
const VERSION: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

struct Exr {
    magic_number: [u8;4],
    version: [u8;4],
    header: Vec<HashMap<String, Header>>,
}

struct Header {
    name: String,
    size:  String,
}

impl Exr {
    fn new() -> Self {
        let header = HashMap::new();
        header.insert(
            "channels".to_string(),
            Header {

            }
        )
        Self {
            magic_number: MAGIC_NUMBER,
            version: VERSION,
            header: Header {
                name: "channels".to_string(),
                size: "chlist".to_string(),
            }
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
