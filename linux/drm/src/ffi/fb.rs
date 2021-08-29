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
    pub fn drmModeGetFB(fd: libc::c_int, fb_id: libc::c_uint) -> *const DrmFramebuffer;

    pub fn drmModeFreeFB(ptr: *const DrmFramebuffer);
}