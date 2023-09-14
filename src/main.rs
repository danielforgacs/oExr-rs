mod structs;
mod consts;
mod prelude {
    pub use super::structs::exr;
    pub use super::structs::chan;
    pub use half::f16;
}
use prelude::*;

fn main() {
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

    let mut exr = exr::Exr::new(4, 3);
    let chan_g = chan::Channel::new("G", chan::ChannelType::Half(pixel_data_g));
    let chan_z = chan::Channel::new("Z", chan::ChannelType::FLoat(pixel_data_z));
    exr.add_channel(chan_g);
    exr.add_channel(chan_z);
    let _result = std::fs::write("__rnd.exr", exr.serialize());
}
