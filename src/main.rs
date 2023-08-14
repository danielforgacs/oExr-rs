mod structs;
mod utils;
mod prelude {
    pub use std::fs::{write, read};
    pub use std::collections::HashMap;
    pub use super::structs::exr;
}
use prelude::*;


fn main() {
    {
        let data = read("sample_file.exr").unwrap();
        let example_exr = exr::Exr::deserialize(data);
        let data = example_exr.serialize();
        write("sample_file-rewrite.exr", data).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_re_serializing_ok() {
        let data = read("sample_file.exr").unwrap();
        let example_exr = exr::Exr::deserialize(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }
}
