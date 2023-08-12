mod structs;
mod prelude {
    pub use crate::structs::exr::*;
    pub use crate::structs::header::*;
}
use prelude::*;

use std::fs::{write, read};

fn main() {
    {
        let data = read("example_exr_from_docs.exr").unwrap();
        let example_exr = Exr::deserialize(data);
        let exr = Exr::new(example_exr.get_header());
        let data = exr.serialize();
        write("test_result.exr", data).unwrap();
    }
    let exr = Exr::new(
        Header::new(),
    );
    let data = exr.serialize();
    write("test_result_0.exr", data).unwrap();


}
