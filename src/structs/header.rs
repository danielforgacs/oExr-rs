use super::super::utils;

pub struct Header {
    data: Vec<u8>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>) -> Self {
        let mut raw_data: Vec<u8> = Vec::new();
        loop {
            let name = utils::parse_until_null(data);
            raw_data.extend(name.bytes().clone());
            data.drain(..1);
            raw_data.push(0);
            println!("attribute: {}", &name);

            let attr_type = utils::parse_until_null(data);
            raw_data.extend(attr_type.bytes().clone());
            data.drain(..1);
            raw_data.push(0);
            println!("  attribute type: {}", &attr_type);

            let attr_bytes_count: Vec<u8> = data.drain(..4).collect();
            raw_data.extend(attr_bytes_count.clone());
            let attr_bytes_count = u32::from_le_bytes(attr_bytes_count.try_into().unwrap());
            println!("  attribute len: {}", &attr_bytes_count);

            let attr_data: Vec<u8> = data.drain(..attr_bytes_count as usize).collect();
            raw_data.extend(attr_data.clone());
            println!("  attribute data: {:02X?}", &attr_data);

            if &name == "screenWindowWidth" {
                break;
            }
        }
        // let header = data
        //     .drain(..288)
        //     .collect::<Vec<u8>>()
        //     .try_into()
        //     .unwrap();
        Self { data: raw_data }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.data.clone()
    }
}
