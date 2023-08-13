use super::super::utils;

pub struct Header {
    attributes: Vec<Attribute>,
}

struct Attribute {
    name: String,
    attr_type: String,
    length: u32,
    data: Vec<u8>,
}

impl Attribute {
    fn deserialize(&self) -> Vec<u8> {
        let mut attr_data: Vec<u8> = Vec::new();
        attr_data.extend(self.name.bytes());
        attr_data.push(0);
        attr_data.extend(self.attr_type.bytes());
        attr_data.push(0);
        attr_data.extend(self.length.to_le_bytes());
        attr_data.extend(self.data.clone());
        attr_data
    }
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>) -> Self {
        let mut attributes = Vec::new();
        loop {
            // This loop is extracting attr name, attr type and attr data tuples.
            // The last data must (?) be "screenWindowWidth". Data after that
            // belongs to the offset tables
            let name = utils::parse_until_null(data);
            data.drain(..1);
            println!("attribute: {}", &name);

            let attr_type = utils::parse_until_null(data);
            data.drain(..1);
            println!("  attribute type: {}", &attr_type);

            let attr_bytes_count: Vec<u8> = data.drain(..4).collect();
            let attr_bytes_count = u32::from_le_bytes(attr_bytes_count.try_into().unwrap());
            println!("  attribute len: {}", &attr_bytes_count);

            let attr_data: Vec<u8> = data.drain(..attr_bytes_count as usize).collect();
            println!("  attribute data: {:02X?}", &attr_data);

            if &name == "dataWindow" {
                let mut temp_data = attr_data.clone();
                let x_min: [u8; 4] = temp_data.drain(..4).collect::<Vec<u8>>().try_into().unwrap();
                let x_min = u32::from_le_bytes(x_min);
                let y_min: [u8; 4] = temp_data.drain(..4).collect::<Vec<u8>>().try_into().unwrap();
                let y_min = u32::from_le_bytes(y_min);
                let x_max: [u8; 4] = temp_data.drain(..4).collect::<Vec<u8>>().try_into().unwrap();
                let x_max = u32::from_le_bytes(x_max);
                let y_max: [u8; 4] = temp_data.drain(..4).collect::<Vec<u8>>().try_into().unwrap();
                let y_max = u32::from_le_bytes(y_max);
                println!(":: data window: {}, {}, {}, {}", x_min, y_min, x_max, y_max);
            }

            attributes.push(
                Attribute {
                    name: name.clone(),
                    attr_type,
                    length: attr_bytes_count,
                    data: attr_data,
                }
            );

            if &name == "screenWindowWidth" {
                break;
            }
        }
        Self { attributes }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        for attr in self.attributes.iter() {
            data.extend(attr.deserialize());
        }
        data
    }
}
