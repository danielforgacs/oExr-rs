#[derive(Clone, Debug, PartialEq)]
pub enum Parting {
    Singlepart,
    Multipart,
}

impl From<u32> for Parting {
    fn from(value: u32) -> Self {
        let multipart_bit = (value & 0b_00000000_00000000_00010000_00000000) >> 12;
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

impl From<Parting> for bool {
    fn from(value: Parting) -> Self {
        match value {
            Parting::Multipart => true,
            Parting::Singlepart => false,
        }
    }
}

pub fn deserialize_version_field(data: &mut Vec<u8>) -> (u32, Parting) {
    let version_field = u32::from_le_bytes(
        data.drain(..4)
            .collect::<Vec<u8>>()
            .to_owned()
            .try_into()
            .unwrap(),
    );
    let format_version = version_field & 0b_00000000_00000000_00000000_11111111;
    let multipart_bit = Parting::from(version_field);
    (format_version, multipart_bit)
}

pub fn serialize_version_field(version: u32, multipart_bit: Parting) -> [u8; 4] {
    let version_field = version | u32::from(multipart_bit);
    version_field.to_le_bytes()
}
