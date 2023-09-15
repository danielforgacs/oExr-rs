//! Basic data types from the exr docs.
//! <https://openexr.com/en/latest/OpenEXRFileLayout.html#basic-data-types>

/// unsigned char, signed: no, size in bytes: 1
pub type UnsignedChar = u8;
/// short, signed: yes, size in bytes: 2
pub type Short = i16;
/// unsigned short, signed: no, size in bytes: 2
pub type UnsignedShort = u16;
/// int, signed: yes, size in bytes: 4
pub type Int = i32;
/// unsigned int, signed: no, size in bytes: 4
pub type UnsignedInt = u32;
/// unsigned long, signed: no, size in bytes: 8
pub type UnsignedLong = u64;
