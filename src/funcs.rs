pub fn get_sized_int_4bytes(data: &mut Vec<u8>) -> u32 {
    let int_data = data.drain(..4).collect::<Vec<u8>>();
    let int_data: [u8; 4] = int_data.try_into().unwrap();
    u32::from_le_bytes(int_data)
}
