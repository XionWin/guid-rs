extern crate libc as c;

mod file;
pub(crate) mod ffi;

pub use c::{c_char, c_schar, c_uchar, c_short, c_ushort, c_int, c_uint, c_float, c_double, c_ulong};
pub use file::*;


