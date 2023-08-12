#[derive(Clone)]
pub struct Chlist {
    data: Vec<u8>,
}

impl Chlist {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn deserialise(mut data: Vec<u8>) -> Self {
        Self {
            data: data.drain(..=64).collect::<Vec<u8>>(),
        }
    }

    pub fn serialise(&self) -> Vec<u8> {
        self.data.clone()
    }
}
