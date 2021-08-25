use std::{os::unix::prelude::RawFd};

use crate::{Connector, Crtc, Encoder, Framebuffer};

#[derive(Debug)]
pub struct Resources {
    ptr: *const crate::ffi::DrmResources,
    
    fbs: Vec<Framebuffer>,

    crtcs: Vec<Crtc>,

    connectors: Vec<Connector>,
    
    encoders: Vec<Encoder>,

    min_width: libc::c_uint,
    max_width: libc::c_uint,
    min_height: libc::c_uint,
    max_height: libc::c_uint,
}

impl Resources {
    pub fn new(fd: RawFd) -> Self {
        let r = unsafe { *crate::ffi::drmModeGetResources(fd) };

        Self {
            ptr: &r as *const crate::ffi::DrmResources,
            fbs: get_fbs(fd, &r),
            crtcs: get_crtcs(fd, &r),
            connectors: get_connectors(fd, &r),
            encoders: get_encoders(fd, &r),
            min_width: r.min_width,
            max_width: r.max_width,
            min_height: r.min_height,
            max_height: r.max_height,
        }
    }
}
pub fn get_fbs(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Framebuffer> {
    unsafe {
        std::slice::from_raw_parts(r.fbs, r.count_fbs as usize).iter().map(|x| {
            super::Framebuffer::new( *crate::ffi::drmModeGetFB(fd, *x))
        }).collect::<Vec<super::Framebuffer>>()
    }
}

pub fn get_crtcs(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Crtc> {
    unsafe {
        std::slice::from_raw_parts(r.crtcs, r.count_crtcs as usize).iter().map(|x| {
            super::Crtc::new( *crate::ffi::drmModeGetCrtc(fd, *x))
        }).collect::<Vec<super::Crtc>>()
    }
}

pub fn get_connectors(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Connector> {
    unsafe {
        std::slice::from_raw_parts(r.connectors, r.count_connectors as usize).iter().map(|x| {
            super::Connector::new(*crate::ffi::drmModeGetConnector(fd, *x))
        }).collect::<Vec<super::Connector>>()
    }
}

pub fn get_encoders(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Encoder> {
    unsafe {
        std::slice::from_raw_parts(r.encoders, r.count_encoders as usize).iter().map(|x| {
            super::Encoder::new(*crate::ffi::drmModeGetEncoder(fd, *x))
        }).collect::<Vec<super::Encoder>>()
    }
}
