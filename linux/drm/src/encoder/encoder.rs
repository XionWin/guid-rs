#[derive(Debug)]
pub struct Encoder {
    ptr: *const crate::ffi::DrmEncoder,
    encoder_id: libc::c_uint,
    encoder_type: crate::encoder::EncoderType,
    crtc_id: libc::c_uint,
    possible_crtcs: libc::c_uint,
    possible_clones: libc::c_uint,
}

impl Encoder {
    pub fn new(e: &crate::ffi::DrmEncoder) -> Self {
        Self {
            ptr: e as *const crate::ffi::DrmEncoder,
            encoder_id: e.encoder_id,
            encoder_type: e.encoder_type,
            crtc_id: e.crtc_id,
            possible_crtcs: e.possible_crtcs,
            possible_clones: e.possible_clones,
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::drmModeFreeEncoder(self.ptr);
            println!("Encoder: {:?} droped", self.encoder_id);
        }
    }
}