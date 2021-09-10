use std::os::unix::prelude::RawFd;

use crate::{ModeInfo, Connector, Crtc, Encoder};

#[derive(Debug)]
pub struct Drm {
    fd: RawFd,

    connector: Connector,
    
    encoder: Encoder,

    crtc: Crtc,
}

impl Drm {
    pub fn new<T>(mut resource: crate::Resources, connector_selector: T) -> Self
    where 
        T: FnMut(&Connector) -> bool,
    {
        
        let connector = match resource.connectors.iter().position(connector_selector) {
            Some(index) => resource.connectors.remove(index),
            None => panic!("Connector not found")
        };

        let encoder = match resource.encoders.iter().position(|x| x.get_encoder_id() == connector.get_encoder_id()) {
            Some(index) => resource.encoders.remove(index),
            None => panic!("Encoder not found")
        };

        let crtc = match resource.crtcs.iter().position(|x| x.get_id() == encoder.get_crtc_id()) {
            Some(index) => resource.crtcs.remove(index),
            None => panic!("Crtc not found")
        };
        
        Self{
            fd: resource.get_fd(),
            connector,
            encoder,
            crtc,
        }
    }
    pub fn get_fd(&self) -> RawFd {
        self.fd
    }

    pub fn get_crtc(&self) -> &Crtc {
        &self.crtc
    }

    pub fn get_mode(&self) -> &ModeInfo {
        match self.connector.modes.iter().find(|x| bitwise_contains!(x.get_mode_type(), crate::common::DrmModeType::PREFERRED)) {
            Some(mode) => mode,
            None => panic!("Mode not found")
        }
    }
}