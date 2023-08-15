#[derive(Clone, Debug, PartialEq)]
pub enum Parting {
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

pub fn deserialize_version_field(data: &mut Vec<u8>) -> (u32, Parting) {
    let version_field = u32::from_le_bytes(
        data.drain(..4)
            .collect::<Vec<u8>>()
            .to_owned()
            .try_into()
            .unwrap(),
    );
    let format_version = version_field & 0b_00000000_00000000_00000000_11111111;
    println!(":: format version number: {}", format_version);
    let multipart_bit = Parting::from(version_field);
    (format_version, multipart_bit)
}
