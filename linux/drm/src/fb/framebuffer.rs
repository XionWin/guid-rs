#[derive(Debug)]
pub struct Framebuffer
{
    ptr: *const crate::ffi::DrmFramebuffer,
    fb_id: libc::c_uint,
    width: libc::c_uint,
    height: libc::c_uint,
    pitch: libc::c_uint,
    bpp: libc::c_uint,
    depth: libc::c_uint,
    /* driver specific handle */
    handle: libc::c_uint,
}

impl Framebuffer {
    pub fn new(fb: crate::ffi::DrmFramebuffer) -> Self {
        Self {
            ptr: &fb as *const crate::ffi::DrmFramebuffer,
            fb_id: fb.fb_id,
            width: fb.width,
            height: fb.height,
            pitch: fb.pitch,
            bpp: fb.bpp,
            depth: fb.depth,
            handle: fb.handle,
        }
    }
}