#[derive(Debug)]
pub struct Framebuffer
{
    handle: *const crate::ffi::DrmFramebuffer,
    fb_id: libc::c_uint,
    width: libc::c_uint,
    height: libc::c_uint,
    pitch: libc::c_uint,
    bpp: libc::c_uint,
    depth: libc::c_uint,
    /* driver specific handle */
    handle_: libc::c_uint,
}

impl Framebuffer {
    pub fn new(fb: &crate::ffi::DrmFramebuffer) -> Self {
        Self {
            handle: fb as *const crate::ffi::DrmFramebuffer,
            fb_id: fb.fb_id,
            width: fb.width,
            height: fb.height,
            pitch: fb.pitch,
            bpp: fb.bpp,
            depth: fb.depth,
            handle_: fb.handle,
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::drmModeFreeFB(self.handle);
            println!("Framebuffer: {:?} droped", self.handle);
        }
    }
}