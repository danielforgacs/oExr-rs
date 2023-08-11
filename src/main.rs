mod exr;
mod exrheader;
mod prelude {
    pub use crate::exr::*;
}
use prelude::*;

use std::fs::{write, read};

fn main() {
    let header = exrheader::Header {};
    let data = read("example_exr_from_docs.exr").unwrap();
    let example_exr = Exr::deserialize(data);
    let exr = Exr::new();
    let data = exr.serialize();
    write("test_result.exr", data).unwrap();
}
