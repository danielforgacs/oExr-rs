use crate::prelude::*;
use crate::consts;
use super::vfield;
use super::chan;

pub struct Exr {
    magic_number: [u8; 4],
    version_field: vfield::VersionField,
    channels: Vec<chan::Channel>,
}

impl Exr {
    pub fn new() -> Self {
        Self {
            magic_number: consts::MAGIC_NUMBER,
            version_field: vfield::VersionField::new(),
            channels: vec![],
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut exr_bytes = vec![];
        exr_bytes.extend(self.magic_number);
        exr_bytes.extend(self.version_field.serialize());
        for y in 0..1 {
            let mut row_pixels: Vec<u8> = vec![];
            for chan in &self.channels {
                let serial_chan = chan.serialize(4, 1);
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
        let mut exr = Exr::new();
        let chan_G_pixel_values = vec![
            f16::from_le_bytes([0x00, 0x00]),
            f16::from_le_bytes([0x54, 0x29]),
            f16::from_le_bytes([0xd5, 0x35]),
            f16::from_le_bytes([0xe8, 0x2d]),
        ];
        let chan_Z_pixel_values = vec![
            f32::from_le_bytes([0x5c, 0x28, 0x81, 0x3a]),
            f32::from_le_bytes([0xcf, 0xe1, 0x34, 0x3e]),
            f32::from_le_bytes([0x8b, 0x0b, 0xbb, 0x3d]),
            f32::from_le_bytes([0x89, 0x74, 0xf9, 0x3e]),
        ];
        let chan_g = chan::Channel::new("G", chan::ChannelType::Half(chan_G_pixel_values));
        let chan_z = chan::Channel::new("Z", chan::ChannelType::FLoat(chan_Z_pixel_values));
        exr.channels.push(chan_g);
        let expected = vec![
            0x76, 0x2f, 0x31, 0x01,
            0x02, 0x00, 0x00, 0x00,
        ];
        assert_eq!(exr.serialize(), expected);
    }
}
