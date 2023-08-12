use super::chlist::*;

#[derive(Clone)]
pub struct Header {
    channels: Chlist,
}

impl Header {
    pub fn serialise(&self) -> Vec<u8> {
        vec![]
    }

    pub fn deserialise(data: Vec<u8>) -> Self {
        let channels = Chlist::deserialise(data);
        Self {
            channels,
         }
    }
}
