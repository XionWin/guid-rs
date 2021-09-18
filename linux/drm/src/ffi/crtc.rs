use std::os::unix::prelude::RawFd;

#[derive(Debug)]
#[repr(C)]
pub struct DrmCrtc
{
    pub crtc_id: libc::c_uint,
    pub buffer_id: libc::c_uint,

    pub x: libc::c_int,
    pub y: libc::c_int,

    pub width: libc::c_int,
    pub height: libc::c_int,

    pub mode_valid: libc::c_int,

    pub mode: super::mode_info::DrmModeInfo,

    pub gamma_size: libc::c_int
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetCrtc(fd: RawFd, crtc_id: libc::c_uint) -> *const DrmCrtc;
    pub fn drmModeFreeCrtc(handle: *const DrmCrtc);
}