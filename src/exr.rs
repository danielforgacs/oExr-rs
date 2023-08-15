use crate::funcs;

// exr magin number bytes: 0x76, 0x2f, 0x31, 0x01
const MAGIC_NUMBER_U32: u32 = 20000630;

pub struct Exr {
    left_over_bytes: Vec<u8>,
    format_version_number: u32,
    multipart_bit: Parting,
}

#[derive(Clone, Debug)]
enum Parting {
    Singlepart,
    Multipart,
}

impl From<u32> for Parting {
    fn from(value: u32) -> Self {
        let multipart_bit = (value & 0b_00000000_00000000_00010000_00000000) >> 12;
        println!(":: multipart bit: {}", multipart_bit);
        match multipart_bit {
            0 => Self::Singlepart,
            1 => Self::Multipart,
            _ => panic!("Wrong multi part bit!"),
        }
    }
}

impl From<Parting> for u32 {
    fn from(value: Parting) -> Self {
        match value {
            Parting::Singlepart => 0 << 12,
            Parting::Multipart => 1 << 12,
        }

    }
}

impl Exr {
    pub fn deserialize(mut data: Vec<u8>) -> Exr {
        let magic_num = funcs::get_sized_int_4bytes(&mut data);
        if magic_num != MAGIC_NUMBER_U32 {
            panic!("The magic number is wrong!");
        }
        let version_field = u32::from_le_bytes(
            data.drain(..4)
            .collect::<Vec<u8>>()
            .to_owned()
            .try_into()
            .unwrap()
        );
        let format_version_number = version_field & 0b_00000000_00000000_00000000_11111111;
        println!(":: format version number: {}", format_version_number);
        let multipart_bit = Parting::from(version_field);
        Self {
            left_over_bytes: data,
            format_version_number,
            multipart_bit,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = MAGIC_NUMBER_U32.to_le_bytes().to_vec();
        let version_field = self.format_version_number | u32::from(self.multipart_bit.clone());
        data.extend(version_field.to_le_bytes());
        data.extend(self.left_over_bytes.clone());
        data
    }
}
