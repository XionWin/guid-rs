use std::ffi::CStr;

#[derive(Debug)]
pub struct ModeInfo {
    raw: crate::ffi::DrmModeInfo,
}

impl ModeInfo {
    pub fn new(drm_mode_info: crate::ffi::DrmModeInfo) -> Self {
        Self {
            raw: drm_mode_info
        }
    }

    pub fn get_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.raw.name.as_ptr()).to_str().unwrap()
        }
        
    }

}