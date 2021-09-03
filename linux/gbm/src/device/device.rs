use std::os::unix::prelude::RawFd;

#[derive(Debug)]
pub struct Device {
    pub(crate) handle: *const crate::ffi::GbmDevice
}

impl Device {
    pub fn new(fd: RawFd) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_create_device(fd)
            }
        }
    }

    pub(crate) fn get_handle(&self) -> *const crate::ffi::GbmDevice {
        self.handle
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