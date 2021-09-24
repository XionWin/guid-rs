use std::os::unix::prelude::RawFd;

pub fn page_flip(fd: RawFd, crtc_id: libc::c_uint, fb_id: libc::c_uint,  flags: crate::def::PageFlipFlags, user_data: *mut libc::c_void) -> libc::c_int {
    unsafe {
        crate::ffi::drmModePageFlip(fd, crtc_id, fb_id, flags, user_data)
    }
}

pub fn handle_event(fd: RawFd, evt_context: *const crate::def::EventContext) -> libc::c_int {
    unsafe {
        crate::ffi::drmHandleEvent(fd, evt_context)
    }
}