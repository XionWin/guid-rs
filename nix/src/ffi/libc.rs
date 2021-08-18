use std::ffi::CStr;

bitflags!{
    #[repr(transparent)]
    pub struct OpenMode: u32 {
        const READ_ONLY = 0b0000;
        const WRITE_ONLY = 0b0001;
        const READ_WRITE = 0b0010;
        const NON_BLOCK = 0b1000;
        const CLOSE_ON_EXEC = 0x0080000;
    }
}

pub fn open(path: &str, mode: OpenMode) -> i32 {
    unsafe {
        let mut path = path.bytes().collect::<Vec<u8>>();
        path.push(b'\0');
        match CStr::from_bytes_with_nul(&path) {
            Ok(path) => {
                super::libc::libc_open(path.as_ptr(), mode)
            }
            Err(msg) => {
                println!("{:?}", msg);
                -2
            }
        }
    }
}

extern "C" {
    fn libc_open(device: *const u8, mode: OpenMode) -> i32;
}