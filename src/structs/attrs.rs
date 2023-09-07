pub enum Compression {
    No,
}

impl Compression {
    pub fn serialise(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let value = match self {
            Self::No => 0_u8.to_le_bytes(),
        };
        bytes.extend("compression".as_bytes());
        bytes.push(0);
        bytes.extend("compression".as_bytes());
        bytes.push(0);
        bytes.extend((value.len() as i32).to_le_bytes());
        bytes.extend(value);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_compresson() {
        let comp = Compression::No;
        let expected = {
            let mut temp = Vec::new();
            temp.extend("compression".bytes());
            temp.push(0);
            temp.extend("compression".bytes());
            temp.push(0);
            temp.extend([0x01, 0x00, 0x00, 0x00]);
            temp.push(0);
            temp
        };
        assert_eq!(comp.serialise(), expected);
    }
}
