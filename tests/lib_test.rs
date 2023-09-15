use oexr::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_data_types() {
        let value: ingredients::basic_data_types::UnsignedChar = 1;
        assert_eq!(1_u8, value);
    }
}
