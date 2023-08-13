use super::super::utils;

pub struct Header {
    data: Vec<u8>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>) -> Self {
        let mut header_bytes: Vec<u8> = Vec::new();
        loop {
            // This loop is extracting attr name, attr type and attr data tuples.
            // The last data must (?) be "screenWindowWidth". Data after that
            // belongs to the offset tables
            let name = utils::parse_until_null(data);
            header_bytes.extend(name.bytes().clone());
            data.drain(..1);
            header_bytes.push(0);
            println!("attribute: {}", &name);

            let attr_type = utils::parse_until_null(data);
            header_bytes.extend(attr_type.bytes().clone());
            data.drain(..1);
            header_bytes.push(0);
            println!("  attribute type: {}", &attr_type);

            let attr_bytes_count: Vec<u8> = data.drain(..4).collect();
            header_bytes.extend(attr_bytes_count.clone());
            let attr_bytes_count = u32::from_le_bytes(attr_bytes_count.try_into().unwrap());
            println!("  attribute len: {}", &attr_bytes_count);

            let attr_data: Vec<u8> = data.drain(..attr_bytes_count as usize).collect();
            header_bytes.extend(attr_data.clone());
            println!("  attribute data: {:02X?}", &attr_data);

            if &name == "screenWindowWidth" {
                data.push(0);
                header_bytes.push(0);
                break;
            }
        }
        Self { data: header_bytes }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.data.clone()
    }
}
