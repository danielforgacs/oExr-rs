use super::chlist::*;

#[derive(Clone)]
pub struct Header {
    channels: Chlist,
}

impl Header {
    pub fn new() -> Self {
        let channels = Chlist::new();
        Self {
            channels,
        }
    }

    pub fn deserialize(data: Vec<u8>) -> Self {
        let channels = Chlist::deserialise(data);
        Self {
            channels,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.channels.serialise()
    }
}
