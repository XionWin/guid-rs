use std::{ffi::{CStr}, os::unix::prelude::RawFd};


pub struct Resources {
    
}

impl Resources {
    pub fn new(fd: RawFd) -> Self {
        let ptr_drm = unsafe {
            *crate::ffi::drmModeGetResources(fd)
        };
        println!("{:?}", ptr_drm);

        unsafe {
            let connectors = std::slice::from_raw_parts(ptr_drm.connectors, ptr_drm.count_connectors as usize);
            for connector in connectors {
                println!("connector id: {:?}", connector);
                let connector = *crate::ffi::drmModeGetConnector(fd, *connector);

                let connector = crate::connector::Connector::new(connector);

                let mode = connector.get_mode();
                println!("{:?}", mode);

                let name =  CStr::from_ptr(mode.name.as_ptr());

                println!("{:?}", name);
            }
        }
        Self {
        }
    }

    pub fn get_resources(&self) {
    }
}