pub fn get_sized_int_4bytes(data: &mut Vec<u8>) -> u32 {
    let int_data = data.drain(..4).collect::<Vec<u8>>();
    let int_data: [u8; 4] = int_data.try_into().unwrap();
    u32::from_le_bytes(int_data)
}

pub fn get_bytes_until_null(data: &mut Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    while data[0] != 0 {
        let current_byte = data.drain(..1).as_slice()[0];
        result.push(current_byte);
    }
    result
}
