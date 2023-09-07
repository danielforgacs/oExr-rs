use crate::prelude::*;
use crate::consts;
use super::vfield;
use super::chan;
// use super::chan::{Channel, ChannelType};

pub struct Exr {
    magic_number: [u8; 4],
    version_field: vfield::VersionField,
    channels: Vec<chan::Channel>,
    res_x: u32,
    res_y: u32,
}

impl Exr {
    pub fn new(res_x: u32, res_y: u32) -> Self {
        Self {
            magic_number: consts::MAGIC_NUMBER,
            version_field: vfield::VersionField::new(),
            channels: vec![],
            res_x,
            res_y,
        }
    }

    pub fn add_channel(&mut self, chan: chan::Channel) {
        self.channels.push(chan);
    }

    fn get_channels_attr_bytes(&self) -> Vec<u8> {
        let mut channels = Vec::new();
        for chan in &self.channels {
            channels.extend(chan.get_channel_attribute());
        }
        // the null byte after this attribute is counted in the size
        let channels_byte_count = {
            let mut size = channels.len() as i32;
            size += 1;
            size
        };
        dbg!(&channels_byte_count);
        let mut attr_data = Vec::new();
        attr_data.extend("channels".as_bytes());
        attr_data.push(0);
        attr_data.extend("chlist".as_bytes());
        attr_data.push(0);
        attr_data.extend(channels_byte_count.to_le_bytes());
        attr_data
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut exr_bytes = Vec::new();
        exr_bytes.extend(self.magic_number);
        exr_bytes.extend(self.version_field.serialize());
        exr_bytes.extend(self.get_channels_attr_bytes());
        {
            for y in 0..self.res_y {
                let mut scan_line = Vec::new();
                for channel in &self.channels {
                    let chan_bytes = channel.serialize_to_resolution(self.res_x, self.res_y);
                    scan_line.extend(chan_bytes[y as usize].clone());
                }
                exr_bytes.extend(y.to_le_bytes());
                let pixel_value_byte_count = (scan_line.len() as u32).to_le_bytes();
                exr_bytes.extend(pixel_value_byte_count);
                exr_bytes.extend(scan_line);
            }
        }
        exr_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exr() {
        let pixel_data_g = vec![
            f16::from_le_bytes([0x00, 0x00]), f16::from_le_bytes([0x54, 0x29]), f16::from_le_bytes([0xd5, 0x35]), f16::from_le_bytes([0xe8, 0x2d]),
            f16::from_le_bytes([0x37, 0x38]), f16::from_le_bytes([0x76, 0x33]), f16::from_le_bytes([0x74, 0x3b]), f16::from_le_bytes([0x73, 0x38]),
            f16::from_le_bytes([0x23, 0x3a]), f16::from_le_bytes([0x0a, 0x34]), f16::from_le_bytes([0x02, 0x3b]), f16::from_le_bytes([0x5d, 0x3b]),
        ];
        let pixel_data_z = vec![
            f32::from_le_bytes([0x5c, 0x28, 0x81, 0x3a]), f32::from_le_bytes([0xcf, 0xe1, 0x34, 0x3e]), f32::from_le_bytes([0x8b, 0x0b, 0xbb, 0x3d]), f32::from_le_bytes([0x89, 0x74, 0xf9, 0x3e]),
            f32::from_le_bytes([0x7f, 0xab, 0xe8, 0x3e]), f32::from_le_bytes([0x8a, 0xcf, 0x54, 0x3f]), f32::from_le_bytes([0x5b, 0x6c, 0x11, 0x3f]), f32::from_le_bytes([0x20, 0x35, 0x50, 0x3d]),
            f32::from_le_bytes([0x38, 0xf3, 0x9a, 0x3c]), f32::from_le_bytes([0x4d, 0xad, 0x98, 0x3e]), f32::from_le_bytes([0x1c, 0x14, 0x08, 0x3f]), f32::from_le_bytes([0x4c, 0xf3, 0x03, 0x3f]),
        ];

        let mut exr = Exr::new(4, 3);
        let chan_g = chan::Channel::new("G", chan::ChannelType::Half(pixel_data_g));
        let chan_z = chan::Channel::new("Z", chan::ChannelType::FLoat(pixel_data_z));
        exr.add_channel(chan_g);
        exr.add_channel(chan_z);

        let expected = vec![
            // magic num
            0x76, 0x2f, 0x31, 0x01,
            // version bytes
            0x02, 0x00, 0x00, 0x00,

            // Header attributes
                // "channels"
                0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73,
                // null byte
                0x00,
                // "chlist"
                0x63, 0x68, 0x6c, 0x69, 0x73, 0x74,
                // null byte
                0x00,
                // attribute size
                // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

                // Channel "G" name:
                0x47,
                // null byte
                0x00,
                // Channel "Z" name:
                0x5a,
                // null byte
                0x00,




            // scanline 0 pixel values
                // scan line index
                0x00, 0x00, 0x00, 0x00,
                // pixel data size
                0x18, 0x00, 0x00, 0x00,
                // channel G
                0x00, 0x00, 0x54, 0x29, 0xd5, 0x35, 0xe8, 0x2d,
                // channel Z
                0x5c, 0x28, 0x81, 0x3a, 0xcf, 0xe1, 0x34, 0x3e, 0x8b, 0x0b, 0xbb, 0x3d, 0x89, 0x74, 0xf9, 0x3e,


            // scanline 1 pixel values
                // scan line index
                0x01, 0x00, 0x00, 0x00,
                // pixel data size
                0x18, 0x00, 0x00, 0x00,
                // channel g
                0x37, 0x38, 0x76, 0x33, 0x74, 0x3b, 0x73, 0x38,
                // channel Z
                0x7f, 0xab, 0xe8, 0x3e, 0x8a, 0xcf, 0x54, 0x3f, 0x5b, 0x6c, 0x11, 0x3f, 0x20, 0x35, 0x50, 0x3d,

            // scanline 2 pixel values
                // scan line index
                0x02, 0x00, 0x00, 0x00,
                // pixel data size
                0x18, 0x00, 0x00, 0x00,
                // channel g
                0x23, 0x3a, 0x0a, 0x34, 0x02, 0x3b, 0x5d, 0x3b,
                // channel Z
                0x38, 0xf3, 0x9a, 0x3c, 0x4d, 0xad, 0x98, 0x3e, 0x1c, 0x14, 0x08, 0x3f, 0x4c, 0xf3, 0x03, 0x3f,
        ];
        // assert_eq!(exr.serialize(), expected);
    }

    #[test]
    fn test_get_channel_attr_bytes() {
        let pixel_data_g = vec![
            f16::from_le_bytes([0x00, 0x00]), f16::from_le_bytes([0x54, 0x29]), f16::from_le_bytes([0xd5, 0x35]), f16::from_le_bytes([0xe8, 0x2d]),
            f16::from_le_bytes([0x37, 0x38]), f16::from_le_bytes([0x76, 0x33]), f16::from_le_bytes([0x74, 0x3b]), f16::from_le_bytes([0x73, 0x38]),
            f16::from_le_bytes([0x23, 0x3a]), f16::from_le_bytes([0x0a, 0x34]), f16::from_le_bytes([0x02, 0x3b]), f16::from_le_bytes([0x5d, 0x3b]),
        ];
        let pixel_data_z = vec![
            f32::from_le_bytes([0x5c, 0x28, 0x81, 0x3a]), f32::from_le_bytes([0xcf, 0xe1, 0x34, 0x3e]), f32::from_le_bytes([0x8b, 0x0b, 0xbb, 0x3d]), f32::from_le_bytes([0x89, 0x74, 0xf9, 0x3e]),
            f32::from_le_bytes([0x7f, 0xab, 0xe8, 0x3e]), f32::from_le_bytes([0x8a, 0xcf, 0x54, 0x3f]), f32::from_le_bytes([0x5b, 0x6c, 0x11, 0x3f]), f32::from_le_bytes([0x20, 0x35, 0x50, 0x3d]),
            f32::from_le_bytes([0x38, 0xf3, 0x9a, 0x3c]), f32::from_le_bytes([0x4d, 0xad, 0x98, 0x3e]), f32::from_le_bytes([0x1c, 0x14, 0x08, 0x3f]), f32::from_le_bytes([0x4c, 0xf3, 0x03, 0x3f]),
        ];

        let mut exr = Exr::new(4, 3);
        let chan_g = chan::Channel::new("G", chan::ChannelType::Half(pixel_data_g));
        let chan_z = chan::Channel::new("Z", chan::ChannelType::FLoat(pixel_data_z));
        exr.add_channel(chan_g);
        exr.add_channel(chan_z);
        let expected = [
            // "channels"
            0x63, 0x68, 0x61, 0x6e, 0x6e, 0x65, 0x6c, 0x73,
            0x00,
            // "chlist"
            0x63, 0x68, 0x6c, 0x69, 0x73, 0x74,
            0x00,

            // attribute size
            0x25, 0x00, 0x00, 0x00,
        ];
        assert_eq!(exr.get_channels_attr_bytes(), expected);
    }
}
