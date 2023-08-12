mod structs;
mod utils;
mod prelude {
    pub use std::fs::{write, read};
    pub use super::structs::exr;
    pub use super::utils::*;
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
