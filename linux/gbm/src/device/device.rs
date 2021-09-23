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

    pub(crate) fn get_handle_raw(&self) -> *const crate::ffi::GbmDevice {
        self.handle
    }

    pub fn get_handle(&self) -> libc::c_uint {
        self.handle as libc::c_uint
    }

    pub fn is_format_supported(&self, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> bool {
        unsafe {
            crate::ffi::gbm_device_is_format_supported(self.handle, format, flags)
        }
    }
}

// impl Drop for Device {
//     fn drop(&mut self) {
//         unsafe {
//             crate::ffi::gbm_device_destroy(self.handle);
//             println!("Device: {:?} droped", self.handle);
//         }
//     }
// }