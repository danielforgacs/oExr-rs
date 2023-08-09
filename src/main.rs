use std::io::prelude::*;
use std::fs::read;


const MAGIC_NUMBER: [u8;4] = [0x76, 0x2f, 0x31, 0x01];
const VERSION: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

struct Exr {
    magic_number: [u8;4],
    version: [u8;4],
}

fn main() {
    let data = read("test_images/exr_4x4_16f_rgb_uncompressed_white.exr");
    dbg!(&data);
}
