use std::os::unix::prelude::RawFd;

use crate::Connector;

#[derive(Debug)]
pub struct Resources {
    connectors: Vec<Connector>,
}

impl Resources {
    pub fn new(fd: RawFd) -> Self {
        let ptr_drm = unsafe { *crate::ffi::drmModeGetResources(fd) };
        Self {
            connectors: get_connectors(fd, &ptr_drm),
        }
    }

}

pub fn get_connectors(fd: RawFd, r: &crate::ffi::DrmResources) -> Vec<super::Connector> {
    let mut result = Vec::<super::Connector>::new();
    foreach_ptr!(
        r.connectors,
        r.count_connectors as usize,
        (|connector_id: &u32| {
            let connector = *crate::ffi::drmModeGetConnector(fd, *connector_id);
            result.push(super::Connector::new(connector));
        })
    );
    result
}