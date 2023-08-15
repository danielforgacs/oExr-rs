use crate::funcs;
use crate::versionfield;
use crate::head;

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
    multipart_bit: versionfield::Parting,
    header: head::Header,
    left_over_bytes: Vec<u8>,
}

impl Exr {
    pub fn deserialize(mut data: Vec<u8>) -> Exr {
        let magic_num = funcs::get_sized_int_4bytes(&mut data);
        if magic_num != MAGIC_NUMBER_U32 {
            panic!("The magic number is wrong!");
        }
        let (format_version, multipart_bit) = versionfield::deserialize_version_field(&mut data);
        let header = head::Header::deserialize(&mut data, &multipart_bit);
        Self {
            format_version,
            multipart_bit,
            header,
            left_over_bytes: data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = MAGIC_NUMBER_U32.to_le_bytes().to_vec();
        data.extend(self.serialize_version_field());
        data.extend(self.header.serialize());
        data.extend(self.left_over_bytes.clone());
        data
    }

    fn serialize_version_field(&self) -> [u8; 4] {
        let version_field = self.format_version | u32::from(self.multipart_bit.clone());
        version_field.to_le_bytes()
    }
}
