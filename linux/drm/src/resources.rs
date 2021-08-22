use std::{os::unix::prelude::RawFd};

use crate::Connector;

#[derive(Debug)]
pub struct Resources {
    // pub count_fbs: libc::c_int,
    // pub fbs: *const libc::c_uint,
    
    // pub count_crtcs: libc::c_int,
    // pub crtcs: *const libc::c_uint,

    connectors: Vec<Connector>,

    // pub count_encoders: libc::c_int,
    // pub encoders: *const libc::c_uint,

    min_width: libc::c_uint,
    max_width: libc::c_uint,
    min_height: libc::c_uint,
    max_height: libc::c_uint,
}

impl Resources {
    pub fn new(fd: RawFd) -> Self {
        let r = unsafe { *crate::ffi::drmModeGetResources(fd) };
        Self {
            connectors: get_connectors(fd, &r),
            min_width: r.min_width,
            max_width: r.max_width,
            min_height: r.min_height,
            max_height: r.max_height,
        }
    }
}

pub fn get_connectors(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Connector> {
    unsafe {std::slice::from_raw_parts(r.connectors, r.count_connectors as usize)}.iter().map(|x| {
        super::Connector::new(unsafe {*crate::ffi::drmModeGetConnector(fd, *x)})
    }).collect::<Vec<super::Connector>>()
}
