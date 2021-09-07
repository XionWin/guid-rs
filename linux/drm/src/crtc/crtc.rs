#[derive(Debug)]
pub struct Crtc
{
    handle: *const crate::ffi::DrmCrtc,
    crtc_id: libc::c_uint,
    buffer_id: libc::c_uint,

    x: libc::c_uint,
    y: libc::c_uint,

    width: libc::c_uint,
    height: libc::c_uint,

    mode_valid: libc::c_int,

    mode: crate::common::ModeInfo,

    gamma_size: libc::c_int
}

impl Crtc {
    pub fn new(c: &crate::ffi::DrmCrtc) -> Self {
        Self {
            handle: c as *const crate::ffi::DrmCrtc,
            crtc_id : c.crtc_id,
            buffer_id : c.buffer_id,
            x : c.x,
            y : c.y,
            width : c.width,
            height : c.height,
            mode_valid : c.mode_valid,
            mode : get_mode(c),
            gamma_size : c.gamma_size
        }
    }

    pub fn get_id(&self) -> libc::c_uint {
        self.crtc_id
    }

    pub fn get_width(&self) -> libc::c_uint {
        self.crtc_id
    }

    pub fn get_height(&self) -> libc::c_uint {
        self.height
    }
}

impl Drop for Crtc {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::drmModeFreeCrtc(self.handle);
            println!("Crtc: {:?} droped", self.handle);
        }
    }
}

fn get_mode(c: &crate::ffi::DrmCrtc) -> crate::common::ModeInfo {
    crate::common::ModeInfo::new(&c.mode)
}