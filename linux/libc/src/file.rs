
use std::{os::unix::prelude::RawFd};

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
                crate::ffi::open(path.as_ptr(), 3)
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