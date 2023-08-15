pub struct Attribute {
    name: String,
    attrtype: String,
    len: u32,
    value: Vec<u8>,
}

impl Attribute {
    pub fn new(name: String, attrtype: String, len: u32, value: Vec<u8>) -> Self {
        Self { name, attrtype, len, value }
    }

    pub fn deserialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(self.name.bytes());
        data.push(0);
        data.extend(self.attrtype.bytes());
        data.push(0);
        data.extend(self.len.to_le_bytes());
        data.extend(self.value.clone());
        data
    }
}
