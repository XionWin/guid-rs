use std::{os::unix::prelude::RawFd};

use crate::{Connector, Crtc, Encoder, Framebuffer};

#[derive(Debug)]
pub struct Resources {
    fd: RawFd,
    pub(crate) handle: *const crate::ffi::DrmResources,
    
    pub(crate) fbs: Vec<Framebuffer>,

    pub(crate) crtcs: Vec<Crtc>,

    pub(crate) connectors: Vec<Connector>,
    
    pub(crate) encoders: Vec<Encoder>,

    pub(crate) min_width: libc::c_uint,
    pub(crate) max_width: libc::c_uint,
    pub(crate) min_height: libc::c_uint,
    pub(crate) max_height: libc::c_uint,
}

impl Resources {
    pub fn new(fd: RawFd) -> Self {
        let handle = unsafe { crate::ffi::drmModeGetResources(fd) };
        let r = unsafe { handle.as_ref().unwrap() };
        
        Self {
            fd,
            handle,
            fbs: get_fbs(fd, r),
            crtcs: get_crtcs(fd, r),
            connectors: get_connectors(fd, r),
            encoders: get_encoders(fd, r),
            min_width: r.min_width,
            max_width: r.max_width,
            min_height: r.min_height,
            max_height: r.max_height,
        }
    }

    
    pub fn get_fd(&self) -> RawFd {
        self.fd
    }
}


impl Drop for Resources {
    fn drop(&mut self) {
        println!("Resources: {:?} droped", self.handle);
    }
}

fn get_fbs(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Framebuffer> {
    unsafe {
        std::slice::from_raw_parts(r.fbs, r.count_fbs as usize).iter().map(|x| {
            super::Framebuffer::new( crate::ffi::drmModeGetFB(fd, *x).as_ref().unwrap())
        }).collect::<Vec<super::Framebuffer>>()
    }
}

fn get_crtcs(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Crtc> {
    unsafe {
        std::slice::from_raw_parts(r.crtcs, r.count_crtcs as usize).iter().map(|x| {
            super::Crtc::new(crate::ffi::drmModeGetCrtc(fd, *x).as_ref().unwrap())
        }).collect::<Vec<super::Crtc>>()
    }
}

fn get_connectors(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Connector> {
    unsafe {
        std::slice::from_raw_parts(r.connectors, r.count_connectors as usize).iter().map(|x| {
            super::Connector::new(crate::ffi::drmModeGetConnector(fd, *x).as_ref().unwrap())
        }).collect::<Vec<super::Connector>>()
    }
}

fn get_encoders(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Encoder> {
    unsafe {
        std::slice::from_raw_parts(r.encoders, r.count_encoders as usize).iter().map(|x| {
            super::Encoder::new(crate::ffi::drmModeGetEncoder(fd, *x).as_ref().unwrap())
        }).collect::<Vec<super::Encoder>>()
    }
}
