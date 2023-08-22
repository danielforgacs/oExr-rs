mod attrib;
mod datawin;
mod exr;
mod funcs;
mod head;
mod vfield;
mod prelude {
    pub use std::collections::HashMap;
    pub use std::fs::{read, write};
}
use prelude::*;

fn main() {
    for exr_file in [
        "sample_file.exr",
        // "../../_temp/original.exr",
        // "../../_temp/original_manymeta.exr",
        // "../../_temp/original_metalong.exr",
        // "../../_temp/original_multi-part.exr",
    ] {
        println!("-- file: {}", exr_file);
        let data = read(exr_file).unwrap();
        println!("   bytes:{:<30}{}", "", data.len());
        let fullexr = exr::Exr::from_bytes(data.clone());
        println!("   format version:{:<30}{}", "", fullexr.get_format_version());
        println!("   is multi-part: {}", fullexr.is_multipart());
        let data = fullexr.serialize();
        let _result = write("debug.exr", data);
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_re_serializing_ok_01() {
        let data = read("sample_file.exr").unwrap();
        let example_exr = exr::Exr::from_bytes(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }

    #[test]
    fn test_re_serializing_ok_02() {
        let data = read("../../_temp/original.exr").unwrap();
        let example_exr = exr::Exr::from_bytes(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }

    #[test]
    fn test_re_serializing_ok_03() {
        let data = read("../../_temp/original_multi-part.exr").unwrap();
        let example_exr = exr::Exr::from_bytes(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }

    #[test]
    fn test_re_serializing_ok_04() {
        let data = read("../../_temp/original_manymeta.exr").unwrap();
        let example_exr = exr::Exr::from_bytes(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }

    #[test]
    fn test_re_serializing_ok_05() {
        let data = read("../../_temp/original_metalong.exr").unwrap();
        let example_exr = exr::Exr::from_bytes(data.clone());
        assert_eq!(example_exr.serialize(), data);
    }
}
