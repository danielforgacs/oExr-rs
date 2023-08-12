mod structs;
mod prelude {
    pub use std::fs::{write, read};
    pub use crate::structs::exr::*;
    pub use crate::structs::header::*;
}
use prelude::*;


fn main() {
    {
        let data = read("example_exr_from_docs.exr").unwrap();
        let example_exr = Exr::deserialize(data);
        let data = example_exr.serialize();
        write("test_result.exr", data).unwrap();
    }
}
