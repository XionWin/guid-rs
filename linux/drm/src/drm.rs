use crate::{ModeInfo, Connector, Crtc, Encoder};

#[derive(Debug)]
pub struct Drm {

    connector: Option<Connector>,
    
    encoder: Option<Encoder>,

    crtc: Option<Crtc>,
}

impl Drm {
    pub fn new<T>(mut resource: crate::Resources, connector_selector: T) -> Self
    where 
        T: FnMut(&Connector) -> bool,
    {
        let connector = match resource.connectors.iter().position(connector_selector) {
            Some(index) => Some(resource.connectors.remove(index)),
            None => None
        };

        let encoder = match &connector {
            Some(connector) => match resource.encoders.iter().position(|x| x.get_encoder_id() == connector.get_encoder_id()) {
                Some(index) => Some(resource.encoders.remove(index)),
                None => None
            },
            None => None,
        };

        let crtc = match &encoder {
            Some(encoder) => match resource.crtcs.iter().position(|x| x.get_id() == encoder.get_crtc_id()) {
                Some(index) => Some(resource.crtcs.remove(index)),
                None => None
            },
            None => None,
        };
        
        Self{
            connector,
            encoder,
            crtc,
        }
    }

    pub fn get_crtc(&self) -> &Option<Crtc> {
        &self.crtc
    }

    pub fn get_mode(&self) -> Option<&ModeInfo> {
        match &self.connector {
            Some(connector) => {
                match connector.modes.iter().find(|x| bitwise_contains!(x.get_mode_type(), crate::common::DrmModeType::PREFERRED)) {
                    Some(mode) => Some(mode),
                    None => None
                }
            },
            None => None,
        }
    }
}