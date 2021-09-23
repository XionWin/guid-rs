use std::os::unix::prelude::RawFd;

#[derive(Debug)]
#[repr(C)]
pub struct DrmFramebuffer
{
    pub fb_id: libc::c_uint,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub pitch: libc::c_uint,
    pub bpp: libc::c_uint,
    pub depth: libc::c_uint,
    /* driver specific handle */
    pub handle: libc::c_uint,
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetFB(fd: RawFd, fb_id: libc::c_uint) -> *const DrmFramebuffer;

    pub fn drmModeAddFB2(fd: RawFd, width: libc::c_uint, height: libc::c_uint,
        pixel_format: libc::c_uint, bo_handles: *const libc::c_uint,
        pitches: *const libc::c_uint, offsets: *const libc::c_uint,
        buf_id: *mut RawFd, flags: libc::c_uint) -> libc::c_int;
    

    // pub fn drmModeFreeFB(ptr: *const DrmFramebuffer);
}