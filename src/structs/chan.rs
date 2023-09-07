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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_pixel_type(&self) -> i32 {
        match self.pixel_values {
            ChannelType::Half(_) => 1,
            ChannelType::FLoat(_) => 2,
        }
    }

    pub fn get_channel_attribute(&self) -> Vec<u8> {
        let mut channel_attr = Vec::new();
        channel_attr.extend(self.name.as_bytes());
        channel_attr.push(0);
        channel_attr.extend(self.get_pixel_type().to_le_bytes());
        // pLinear
        channel_attr.push(0);
        // reserved, three char, should be zero
        channel_attr.extend([0, 0, 0]);
        // xSampling
        channel_attr.extend(1_i32.to_le_bytes());
        // ySampling
        channel_attr.extend(1_i32.to_le_bytes());
        channel_attr
    }

    pub fn serialize(&self, res_x: usize, res_y: usize) -> Vec<Vec<u8>> {
        let mut data: Vec<Vec<u8>> = Vec::new();
        match &self.pixel_values {
            ChannelType::Half(values) => {
                dbg!(&values.len());
                for y in 0..res_y {
                    let mut line = vec![];
                    for x in 0..res_x {
                        let index = (x+1)*(y+1)-1;
                        line.extend(values[index as usize].to_le_bytes());
                    }
                    data.push(line);
                }
            },
            ChannelType::FLoat(values) => {
            },
        }
        data
    }

    pub fn serialize_to_resolution(&self, res_x: u32, res_y: u32) -> Vec<Vec<u8>> {
        let mut data = Vec::new();
        let mut index = 0;
        for y in 0..res_y {
            let mut row = Vec::new();
            for x in 0..res_x {
                let bytes = match &self.pixel_values {
                    ChannelType::Half(values) => values[index].to_le_bytes().to_vec(),
                    ChannelType::FLoat(values) => values[index].to_le_bytes().to_vec(),
                };
                index += 1;
                row.extend(bytes);
            }
            data.push(row);
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_half() {
        let pixel_data = vec![
            f16::from_le_bytes([0x00, 0x00]), f16::from_le_bytes([0x54, 0x29]), f16::from_le_bytes([0xd5, 0x35]), f16::from_le_bytes([0xe8, 0x2d]),
            f16::from_le_bytes([0x37, 0x38]), f16::from_le_bytes([0x76, 0x33]), f16::from_le_bytes([0x74, 0x3b]), f16::from_le_bytes([0x73, 0x38]),
            f16::from_le_bytes([0x23, 0x3a]), f16::from_le_bytes([0x0a, 0x34]), f16::from_le_bytes([0x02, 0x3b]), f16::from_le_bytes([0x5d, 0x3b]),
        ];
        let expected = vec![
            [0x00, 0x00, 0x54, 0x29, 0xd5, 0x35, 0xe8, 0x2d],
            [0x37, 0x38, 0x76, 0x33, 0x74, 0x3b, 0x73, 0x38],
            [0x23, 0x3a, 0x0a, 0x34, 0x02, 0x3b, 0x5d, 0x3b],
        ];
        let value_byte_count = 2;
        let (res_x, res_y) = (expected[0].len() / value_byte_count, expected.len());
        let chan = Channel::new("chan", ChannelType::Half(pixel_data));
        assert_eq!(chan.serialize_to_resolution(res_x as u32, res_y as u32), expected);
    }

    #[test]
    fn test_channel_float() {
        let pixel_data = vec![
            f32::from_le_bytes([0x5c, 0x28, 0x81, 0x3a]), f32::from_le_bytes([0xcf, 0xe1, 0x34, 0x3e]), f32::from_le_bytes([0x8b, 0x0b, 0xbb, 0x3d]), f32::from_le_bytes([0x89, 0x74, 0xf9, 0x3e]),
            f32::from_le_bytes([0x7f, 0xab, 0xe8, 0x3e]), f32::from_le_bytes([0x8a, 0xcf, 0x54, 0x3f]), f32::from_le_bytes([0x5b, 0x6c, 0x11, 0x3f]), f32::from_le_bytes([0x20, 0x35, 0x50, 0x3d]),
            f32::from_le_bytes([0x38, 0xf3, 0x9a, 0x3c]), f32::from_le_bytes([0x4d, 0xad, 0x98, 0x3e]), f32::from_le_bytes([0x1c, 0x14, 0x08, 0x3f]), f32::from_le_bytes([0x4c, 0xf3, 0x03, 0x3f]),
        ];
        let expected = vec![
            [0x5c, 0x28, 0x81, 0x3a, 0xcf, 0xe1, 0x34, 0x3e, 0x8b, 0x0b, 0xbb, 0x3d, 0x89, 0x74, 0xf9, 0x3e],
            [0x7f, 0xab, 0xe8, 0x3e, 0x8a, 0xcf, 0x54, 0x3f, 0x5b, 0x6c, 0x11, 0x3f, 0x20, 0x35, 0x50, 0x3d],
            [0x38, 0xf3, 0x9a, 0x3c, 0x4d, 0xad, 0x98, 0x3e, 0x1c, 0x14, 0x08, 0x3f, 0x4c, 0xf3, 0x03, 0x3f],
        ];
        let value_byte_count = 4;
        let (res_x, res_y) = (expected[0].len() / value_byte_count, expected.len());
        let chan = Channel::new("chan", ChannelType::FLoat(pixel_data));
        assert_eq!(chan.serialize_to_resolution(res_x as u32, res_y as u32), expected);
    }

    #[test]
    fn test_get_channel_attribute_for_half() {
        let pixel_data = vec![f16::from_f32(0.5)];
        let expected = vec![
            // "G"
            0x47,
            // null byte
            0x00,
            // HALF
            0x01, 0x00, 0x00, 0x00,
            // pLinear
            0x00,
            // reserved, three char, should be zero
            0x00, 0x00, 0x00,
            // xSampling
            0x01, 0x00, 0x00, 0x00,
            // ySampling
            0x01, 0x00, 0x00, 0x00,
        ];
        let chan = Channel::new("G", ChannelType::Half(pixel_data));
        assert_eq!(chan.get_channel_attribute(), expected);
    }

    #[test]
    fn test_get_channel_attribute_for_float() {
        let pixel_data = vec![0.5_f32];
        let expected = vec![
            // "G"
            0x5a,
            // null byte
            0x00,
            // HALF
            0x02, 0x00, 0x00, 0x00,
            // pLinear
            0x00,
            // reserved, three char, should be zero
            0x00, 0x00, 0x00,
            // xSampling
            0x01, 0x00, 0x00, 0x00,
            // ySampling
            0x01, 0x00, 0x00, 0x00,
        ];
        let chan = Channel::new("Z", ChannelType::FLoat(pixel_data));
        assert_eq!(chan.get_channel_attribute(), expected);
    }
}
