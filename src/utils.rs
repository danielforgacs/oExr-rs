pub fn parse_until_null(data: &mut Vec<u8>) -> String {
    let mut text = String::new();
    loop {
        let letter = data.remove(0);
        if letter != 0 {
            text.push(letter as char);
            continue;
        }
        break;
    }
    text
}
