/// Chlist is a sequence of channels followed by a null byte.

enum PixelType {
    Uint,
    Half,
    Float,
}
struct Chlist {
    name: String,
    /// possible values are:
    /// UINT = 0, HALF = 1, FLOAT = 2
    pixel_type: PixelType,
    /// usigned char, possible values are 0 and 1
    p_linear: u32,
}
