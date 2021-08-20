use std::os::unix::prelude::RawFd;

pub struct Resources {}

impl Resources {
    pub fn new(fd: RawFd) -> Self {
        let ptr_drm = unsafe { *crate::ffi::drmModeGetResources(fd) };
        println!("{:?}", ptr_drm);

        foreach_ptr!(
            ptr_drm.connectors,
            ptr_drm.count_connectors as usize,
            (|connector_id: &u32| {
                println!("{:?}", connector_id);
                let connector = *crate::ffi::drmModeGetConnector(fd, *connector_id);

                let connector = crate::connector::Connector::new(connector);
                println!("[connector]: {:}", connector);

                let mode = connector.get_mode();
                println!("{:}", mode);

                println!("{:?}", mode.get_name());
            })
        );
        Self {}
    }

    pub fn get_resources(&self) {}
}
