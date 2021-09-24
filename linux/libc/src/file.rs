use std::{os::unix::prelude::RawFd};

bitflags! {
    #[repr(C)]
    pub struct OpenFlags: u32 {
        const READ_ONLY = 0x0000;
        const WRITE_ONLY = 0x0001;
        const READ_WRITE = 0x0002;
        const NON_BLOCK = 0x0800;
        const CLOSE_ON_EXEC = 0x0080000;
    }
}

#[derive(Debug)]
pub struct File {
    path: String,
    fd: RawFd,
}

impl File {
    pub fn new(path: &str) -> Self {
        let str_path = String::from(path);
        let mut path = path.bytes().collect::<Vec<libc::c_char>>();
        path.push(b'\0');
        Self {
            path: str_path,
            fd: unsafe {
                crate::ffi::open(path.as_ptr(), OpenFlags::READ_WRITE)
            }
        }
    }

    pub fn get_fd(&self) -> RawFd {
        self.fd
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}