use std::os::unix::prelude::RawFd;

#[derive(Debug)]
pub struct Device {
    handle: *const crate::ffi::GbmDevice
}

impl Device {
    pub fn new(fd: RawFd) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_create_device(fd)
            }
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::gbm_device_destroy(self.handle);
            println!("Device: {:?} droped", self.handle);
        }
    }
}