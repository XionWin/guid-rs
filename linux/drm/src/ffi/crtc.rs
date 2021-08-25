#[derive(Debug)]
#[repr(C)]
pub struct DrmCrtc
{
    pub crtc_id: libc::c_uint,
    pub buffer_id: libc::c_uint,

    pub x: libc::c_uint,
    pub y: libc::c_uint,

    pub width: libc::c_uint,
    pub height: libc::c_uint,

    pub mode_valid: libc::c_int,

    pub mode: super::mode_info::DrmModeInfo,

    pub gamma_size: libc::c_int
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetCrtc(fd: libc::c_int, crtc_id: libc::c_uint) -> *const DrmCrtc;
}