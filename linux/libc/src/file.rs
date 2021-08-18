
use std::{os::unix::prelude::RawFd};
pub struct File {
    fd: RawFd,
}

impl File {
    pub fn new(path: &str) -> Self {
        
        let mut path = path.bytes().collect::<Vec<libc::c_char>>();
        path.push(b'\0');
        Self {
            fd: unsafe {
                crate::ffi::open(path.as_ptr(), 3)
            }
        }
    }

    pub fn get_fd(&self) -> RawFd {
        self.fd
    }
}