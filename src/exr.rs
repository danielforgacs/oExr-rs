use crate::funcs;
use crate::vfield;
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
    multipart_bit: vfield::Parting,
    header: head::Header,
    left_over_bytes: Vec<u8>,
}

impl Exr {
    pub fn from_bytes(mut data: Vec<u8>) -> Exr {
            println!("= initial data byte count: {}", data.len());
            let mut used_byte_count = 0;
            let mut previous_byte_count = data.len();
            println!("--> bytes: {}", used_byte_count);
        let magic_num = funcs::get_sized_int_4bytes(&mut data);
        if magic_num != MAGIC_NUMBER_U32 {
            panic!("The magic number is wrong!");
        }
            used_byte_count += previous_byte_count - data.len();
            previous_byte_count = data.len();
            println!("--> bytes: {} - after magic", used_byte_count);
        let (format_version, multipart_bit) = vfield::deserialize_version_field(&mut data);
            used_byte_count += previous_byte_count - data.len();
            previous_byte_count = data.len();
            println!("--> bytes: {} - after v field", used_byte_count);
        let header = head::Header::deserialize(&mut data, &multipart_bit);
            used_byte_count += previous_byte_count - data.len();
            previous_byte_count = data.len();
            println!("--> bytes: {} - after v field", used_byte_count);

        let header_byte_count = 4 + 4 + header.serialize().len() as u32;
        println!("++ header byte count: {}", header_byte_count);

        // let first_offset_value = funcs::get_unsigned_long_int_8bytes(&mut data);
        // println!("++ first offset value: {}", first_offset_value);

        let offset_count = header.get_res_y();
        println!("++ offset count (res y): {}", offset_count);

        let all_offset_bytes = offset_count * 8;
        println!("++ offset all_offset_bytes: {}", all_offset_bytes);

        // let offset_bytes = data.drain(..all_offset_bytes as usize);
        // let offset_bytes = data.drain(..5);
        let offset_bytes: Vec<u8> = data.drain(..all_offset_bytes as usize).collect();
        // data.drain(..5);

        // let data_len = data.len();

        print!("@ data left: {}, total: {}", &data.len(), header_byte_count + all_offset_bytes + data.len() as u32);
        Self {
            format_version,
            multipart_bit,
            header,
            left_over_bytes: data,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = MAGIC_NUMBER_U32.to_le_bytes().to_vec();
        data.extend(vfield::serialize_version_field(self.format_version, self.multipart_bit.clone()));
        data.extend(self.header.serialize());
        data.extend(self.left_over_bytes.clone());
        data
    }
}
