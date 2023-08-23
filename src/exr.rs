use crate::funcs;
use crate::head;
use crate::vfield;

/// exr magix number bytes: 0x76, 0x2f, 0x31, 0x01
const MAGIC_NUMBER_U32: u32 = 20000630;

/// The root level of exr data. File layout:
///
/// <https://openexr.com/en/latest/OpenEXRFileLayout.html#high-level-layout>
///
/// The `format version` and `parting` fields come from the version field.
/// The version field is the signed 4 bytes int after the magic number.
///
/// <https://openexr.com/en/latest/OpenEXRFileLayout.html#version-field>
pub struct Exr {
    /// version field least significant 8 bits.
    format_version: u32,
    /// version field bit 9
    multipart_bit: vfield::Parting,
    header: head::Header,
    left_over_bytes: Vec<u8>,
}

impl Exr {
    pub fn from_bytes(mut data: Vec<u8>) -> Exr {
        let magic_num = funcs::get_sized_int_4bytes(&mut data);
        if magic_num != MAGIC_NUMBER_U32 {
            panic!("The magic number is wrong!");
        }
        let (format_version, multipart_bit) = vfield::deserialize_version_field(&mut data);
        let header = head::Header::deserialize(&mut data, &multipart_bit);
        Self {
            format_version,
            multipart_bit,
            header,
            left_over_bytes: data,
        }
    }

    pub fn get_format_version(&self) -> u32 {
        self.format_version
    }

    pub fn is_multipart(&self) -> bool {
        self.multipart_bit.clone().into()
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = MAGIC_NUMBER_U32.to_le_bytes().to_vec();
        data.extend(vfield::serialize_version_field(
            self.format_version,
            self.multipart_bit.clone(),
        ));
        data.extend(self.header.serialize());

        println!("header byte count: {}", data.len());
        println!("res y: {}", self.header.get_res_y());
        println!("first offset: {}", data.len() as u32 + (8 * self.header.get_res_y()));

        // calculating offsets. - SAMPLE IMAGE ONLY!
        let x_res = 4;
        let y_res = 3;
        let scan_line_bytes = 4;
        let pixel_data_size = 4;
        let channel_count = 2;

        data
    }
}
