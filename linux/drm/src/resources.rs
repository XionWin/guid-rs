use std::{ffi::{CStr, CString}, os::unix::prelude::RawFd};


pub struct Resources {
    
}

impl Resources {
    pub fn new(fd: RawFd) -> Self {
        let ptr_drm = unsafe {
            *crate::ffi::drmModeGetResources(fd)
        };
        println!("{:?}", ptr_drm);

        for _connector_id in 0..ptr_drm.count_connectors {
            unsafe {
                let connector = *crate::ffi::drmModeGetConnector(fd, *ptr_drm.connectors);
                println!("{:?}", connector);

                let mode = *connector.modes;
                println!("{:?}", mode);

                let name =  CStr::from_ptr(&mode.name[0]);

                
                println!("{:?}", name);
                
            }
        }
        Self {
        }
    }

    pub fn get_resources(&self) {
    }
}