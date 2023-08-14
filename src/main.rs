mod exr;
mod funcs;
mod prelude {
    pub use std::fs::{write, read};
    pub use std::collections::HashMap;
}
use prelude::*;


fn main() {
    {
        let data = read("sample_file.exr").unwrap();
        let fullexr = exr::Exr::deserialize(data.clone());
        fullexr.serialize();
    }
    {
        let data = read("../../_temp/original.exr").unwrap();
        let fullexr = exr::Exr::deserialize(data.clone());
        fullexr.serialize();
    }
    {
        let data = read("../../_temp/original_multi-part.exr").unwrap();
        let fullexr = exr::Exr::deserialize(data.clone());
        fullexr.serialize();
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_re_serializing_ok__01() {
        let data = read("sample_file.exr").unwrap();
        let example_exr = exr::Exr::deserialize(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }

    #[test]
    fn test_re_serializing_ok__02() {
        let data = read("../../_temp/original.exr").unwrap();
        let example_exr = exr::Exr::deserialize(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }

    #[test]
    fn test_re_serializing_ok__03() {
        let data = read("../../_temp/original_multi-part.exr").unwrap();
        let example_exr = exr::Exr::deserialize(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }
}
