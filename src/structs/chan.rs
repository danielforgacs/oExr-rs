use crate::prelude::*;

pub enum ChannelType {
    /// 2 bytes
    Half(Vec<f16>),
    /// 4 bytes
    FLoat(Vec<f32>),
}

pub struct Channel {
    name: String,
    pixel_values: ChannelType,
}

impl Channel {
    pub fn new(name: impl ToString, values: ChannelType) -> Self {
        Self {
            name: name.to_string(),
            pixel_values: values,
        }
    }

}
