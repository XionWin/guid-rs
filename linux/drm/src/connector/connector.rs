pub struct Connector {
    raw: crate::ffi::DrmConnector,
}

impl Connector {
    pub fn new(drm_connector: crate::ffi::DrmConnector) -> Self {
        Self {
            raw: drm_connector
        }
    }

    pub fn get_mode(&self) -> super::ModeInfo {
        unsafe {
            super::ModeInfo::new(*self.raw.modes)
        }
    }

}