use std::os::unix::prelude::RawFd;

#[derive(Debug)]
#[repr(C)]
pub struct DrmEncoder
{
    pub encoder_id: libc::c_uint,
    pub encoder_type: crate::def::EncoderType,
    pub crtc_id: libc::c_uint,
    pub possible_crtcs: libc::c_uint,
    pub possible_clones: libc::c_uint,
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetEncoder(fd: RawFd, encoder_id: libc::c_uint) -> *const DrmEncoder;
    pub fn drmModeFreeEncoder(handle: *const DrmEncoder);
}