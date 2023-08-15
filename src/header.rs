pub struct Header {
    leftover_bytes: Vec<u8>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>) -> Self {
        let leftover_bytes = data.drain(..1).collect::<Vec<u8>>();
        Self {
            leftover_bytes,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.leftover_bytes.clone()
    }
}
