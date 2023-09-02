mod structs;
mod consts;
mod prelude {
    pub use super::structs::exr;
    pub use half::f16;
}
use prelude::*;

fn main() {
    // {
    //     let exr = exr::Exr::new(4, 3);
    //     for b in exr.serialize() {
    //         print!("{}", b);
    //     }
    // }
    // {
    //     let exr = exr::Exr::new(4, 3);
    //     std::fs::write("generated.exr", exr.serialize()).unwrap();
    // }
}
