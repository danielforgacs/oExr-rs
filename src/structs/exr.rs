use crate::prelude::*;
use crate::consts;
use super::vfield;
use super::chan;
use super::chan::{Channel, ChannelType};

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

    pub fn serialize(&self) -> Vec<u8> {
        let mut exr_bytes = Vec::new();
        exr_bytes.extend(self.magic_number);
        exr_bytes.extend(self.version_field.serialize());
        {
            for y in 0..self.res_y {
                let mut scan_line = Vec::new();
                for channel in &self.channels {
                    let chan_bytes = channel.serialize_to_resolution(self.res_x, self.res_y);
                    scan_line.extend(chan_bytes[y as usize].clone());
                }
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
        let mut exr = Exr::new(4, 3);

        let pixel_data = vec![
            f16::from_le_bytes([0x00, 0x00]), f16::from_le_bytes([0x54, 0x29]), f16::from_le_bytes([0xd5, 0x35]), f16::from_le_bytes([0xe8, 0x2d]),
            f16::from_le_bytes([0x37, 0x38]), f16::from_le_bytes([0x76, 0x33]), f16::from_le_bytes([0x74, 0x3b]), f16::from_le_bytes([0x73, 0x38]),
            f16::from_le_bytes([0x23, 0x3a]), f16::from_le_bytes([0x0a, 0x34]), f16::from_le_bytes([0x02, 0x3b]), f16::from_le_bytes([0x5d, 0x3b]),
        ];
        let chan_g = Channel::new("G", ChannelType::Half(pixel_data));
        exr.channels.push(chan_g);

        let pixel_data = vec![
            f32::from_le_bytes([0x5c, 0x28, 0x81, 0x3a]), f32::from_le_bytes([0xcf, 0xe1, 0x34, 0x3e]), f32::from_le_bytes([0x8b, 0x0b, 0xbb, 0x3d]), f32::from_le_bytes([0x89, 0x74, 0xf9, 0x3e]),
            f32::from_le_bytes([0x7f, 0xab, 0xe8, 0x3e]), f32::from_le_bytes([0x8a, 0xcf, 0x54, 0x3f]), f32::from_le_bytes([0x5b, 0x6c, 0x11, 0x3f]), f32::from_le_bytes([0x20, 0x35, 0x50, 0x3d]),
            f32::from_le_bytes([0x38, 0xf3, 0x9a, 0x3c]), f32::from_le_bytes([0x4d, 0xad, 0x98, 0x3e]), f32::from_le_bytes([0x1c, 0x14, 0x08, 0x3f]), f32::from_le_bytes([0x4c, 0xf3, 0x03, 0x3f]),
        ];
        let chan_z = Channel::new("Z", ChannelType::FLoat(pixel_data));
        exr.channels.push(chan_z);


        let expected = vec![
            // magic num
            0x76, 0x2f, 0x31, 0x01,
            // version bytes
            0x02, 0x00, 0x00, 0x00,


            // scanline 0 pixel values
            0x00, 0x00, 0x54, 0x29, 0xd5, 0x35, 0xe8, 0x2d, 0x5c, 0x28, 0x81, 0x3a, 0xcf, 0xe1, 0x34, 0x3e, 0x8b, 0x0b, 0xbb, 0x3d, 0x89, 0x74, 0xf9, 0x3e,


            // scanline 1 pixel values
            0x37, 0x38, 0x76, 0x33, 0x74, 0x3b, 0x73, 0x38, 0x7f, 0xab, 0xe8, 0x3e, 0x8a, 0xcf, 0x54, 0x3f, 0x5b, 0x6c, 0x11, 0x3f, 0x20, 0x35, 0x50, 0x3d,

            // scanline 2 pixel values
            0x23, 0x3a, 0x0a, 0x34, 0x02, 0x3b, 0x5d, 0x3b, 0x38, 0xf3, 0x9a, 0x3c, 0x4d, 0xad, 0x98, 0x3e, 0x1c, 0x14, 0x08, 0x3f, 0x4c, 0xf3, 0x03, 0x3f,
        ];
        assert_eq!(exr.serialize(), expected);
    }
}
