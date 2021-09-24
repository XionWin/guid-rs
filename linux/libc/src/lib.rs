#[macro_use]
extern crate bitflags;
extern crate libc as c;

mod file;
pub(crate) mod ffi;

pub use c::{c_void, c_char, c_schar, c_uchar, c_short, c_ushort, c_int, c_uint, c_float, c_double, c_long, c_ulong, c_longlong, c_ulonglong};
pub use file::*;


