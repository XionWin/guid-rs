#[derive(Debug)]
#[repr(C)]
pub struct DrmEncoder
{
    pub encoder_id: libc::c_uint,
    pub encoder_type: crate::encoder::EncoderType,
    pub crtc_id: libc::c_uint,
    pub possible_crtcs: libc::c_uint,
    pub possible_clones: libc::c_uint,
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetEncoder(fd: libc::c_int, encoder_id: libc::c_uint) -> *const DrmEncoder;
    pub fn drmModeFreeEncoder(ptr: *const DrmEncoder);
}