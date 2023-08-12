use super::chlist::*;
use super::utils::*;

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

    pub fn deserialize(mut data: Vec<u8>) -> Self {
        let attr = parse_until_null(&mut data);
        let mut channels = Chlist::new();
        if &attr == "channels" {
            let channels = Chlist::deserialise(data);
        } else {
            panic!("not fixed yet.")
        }
        Self {
            channels,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.channels.serialise()
    }
}
