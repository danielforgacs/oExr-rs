enum ChannelType {
    /// 2 bytes
    Half,
    /// 4 bytes
    FLoat,
}

struct Channel<T> {
    x_res: u32,
    y_res: u32,
    name: String,
    pixel_values: Vec<T>,
    channel_type: ChannelType,
}

impl<T> Channel<T> {
    pub fn new(name: impl ToString, x_res: u32, y_res: u32, values: Vec<T>, chtype: ChannelType) -> Self {
        Self {
            x_res,
            y_res,
            name: name.to_string(),
            pixel_values: values,
            channel_type: chtype,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = vec![];
        for cy in 0..self.y_res {
            data.extend(cy.to_le_bytes());
            data.extend(match self.channel_type {
                ChannelType::Half => self.pixel_values
            })
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_and_serializing_channel() {
        let channel = Channel::new("R", 4, 3, vec![
            0.000, 0.042, 0.365, 0.092,
            0.527, 0.233, 0.932, 0.556,
            0.767, 0.252, 0.876, 0.920,
        ], ChannelType::Half);
        let expected = vec![
            0x00, 0x00, 0x00, 0x00,
            0x18, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x54, 0x29, 0xd5, 0x35, 0xe8, 0x2d,

            0x01, 0x00, 0x00, 0x00,
            0x18, 0x00, 0x00, 0x00,
            0x37, 0x38, 0x76, 0x33, 0x74, 0x3b, 0x73, 0x38,

            0x02, 0x00, 0x00, 0x00,
            0x18, 0x00, 0x00, 0x00,
            0x23, 0x3a, 0x0a, 0x34, 0x02, 0x3b, 0x5d, 0x3b,
        ];
        assert_eq!(channel.serialize(), expected);
    }
}
