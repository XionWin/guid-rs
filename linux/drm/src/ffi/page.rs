use std::os::unix::prelude::RawFd;

#[link(name = "drm")]
extern "C" {
    pub fn drmModePageFlip(fd: RawFd, crtc_id: libc::c_uint, fb_id: libc::c_uint,  flags: crate::def::PageFlipFlags, user_data: *mut libc::c_void) -> libc::c_int;
    pub fn drmHandleEvent(fd: RawFd, evt_context: *const crate::def::EventContext) -> libc::c_int;
}