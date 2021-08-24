
#[derive(Debug)]
pub struct Crtc
{
    ptr: *const crate::ffi::DrmCrtc,
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
    pub fn new(c: crate::ffi::DrmCrtc) -> Self {
        Self {
            ptr: &c as *const crate::ffi::DrmCrtc,
            crtc_id : c.crtc_id,
            buffer_id : c.buffer_id,
            x : c.x,
            y : c.y,
            width : c.width,
            height : c.height,
            mode_valid : c.mode_valid,
            mode : get_mode(&c),
            gamma_size : c.gamma_size
        }
    }
}

pub fn get_mode(c: &crate::ffi::DrmCrtc) -> crate::common::ModeInfo {
    crate::common::ModeInfo::new(*&c.mode)
}