// Getting bytes until the next null byte.
// the null byte is put back.
pub fn parse_until_null(data: &mut Vec<u8>) -> String {
    let mut text = String::new();
    loop {
        let letter = data.remove(0);
        if letter != 0 {
            text.push(letter as char);
            continue;
        }
        data.insert(0, 0);
        break;
    }
    text
}
