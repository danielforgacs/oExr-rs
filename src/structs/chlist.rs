#[derive(Clone)]
pub struct Chlist;

impl Chlist {
    pub fn new() -> Self {
        Self {}
    }

    pub fn deserialise(data: Vec<u8>) -> Self {
        Self {}
    }

    pub fn serialise(&self) -> Vec<u8> {
        vec![]
    }
}
