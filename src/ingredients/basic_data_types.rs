//! Basic data types from the exr docs.
//! <https://openexr.com/en/latest/OpenEXRFileLayout.html#basic-data-types>

use super::super::prelude::*;

/// unsigned char, signed: no, size in bytes: 1
type UnsignedChar = u8;
/// short, signed: yes, size in bytes: 2
type Short = i16;
/// unsigned short, signed: no, size in bytes: 2
type UnsignedShort = u16;
/// int, signed: yes, size in bytes: 4
type Int = i32;
/// unsigned int, signed: no, size in bytes: 4
type UnsignedInt = u32;
/// unsigned long, signed: no, size in bytes: 8
type UnsignedLong = u64;

// Floating-Point Numbers

// size in bytes: 2
type Half = f16;
