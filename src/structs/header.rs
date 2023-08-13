pub struct Header {
    data: Vec<u8>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>) -> Self {
        let header = data
            .drain(..288)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        Self { data: header }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.data.clone()
    }
}
