use crate::funcs;
use crate::versionfield;

// exr magin number bytes: 0x76, 0x2f, 0x31, 0x01
const MAGIC_NUMBER_U32: u32 = 20000630;

pub struct Exr {
    format_version: u32,
    multipart_bit: versionfield::Parting,
    left_over_bytes: Vec<u8>,
}

impl Exr {
    pub fn deserialize(mut data: Vec<u8>) -> Exr {
        let magic_num = funcs::get_sized_int_4bytes(&mut data);
        if magic_num != MAGIC_NUMBER_U32 {
            panic!("The magic number is wrong!");
        }
        let (format_version, multipart_bit) = versionfield::deserialize_version_field(&mut data);
        Self {
            left_over_bytes: data,
            format_version,
            multipart_bit,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = MAGIC_NUMBER_U32.to_le_bytes().to_vec();
        data.extend(self.serialize_versiont_field());
        data.extend(self.left_over_bytes.clone());
        data
    }

    fn serialize_versiont_field(&self) -> [u8; 4] {
        let version_field = self.format_version | u32::from(self.multipart_bit.clone());
        version_field.to_le_bytes()
    }
}
