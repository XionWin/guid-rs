use std::fmt;

#[derive(Debug)]
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

impl fmt::Display for Connector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.get_mode())
    }
}