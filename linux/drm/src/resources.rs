use std::os::unix::prelude::RawFd;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct DrmResources {
    pub count_fbs: libc::c_int,
    pub fbs: *const libc::c_uint,
    pub count_crtcs: libc::c_int,
    pub crtcs: *const libc::c_uint,
    pub count_connectors: libc::c_int,
    pub connectors: *const libc::c_int,
    pub count_encoders: libc::c_int,
    pub encoders: *const libc::c_uint,
    pub min_width: libc::c_uint,
    pub max_width: libc::c_uint,
    pub min_height: libc::c_uint,
    pub max_height: libc::c_uint,
}
pub struct Resources {
    resources: DrmResources,
}

impl Resources {
    pub fn new(fd: RawFd) -> Self {
        let ptr_drm = unsafe {
            crate::ffi::drmModeGetResources(fd)
        };
        Self {
            resources: unsafe {
                *ptr_drm
            }
        }
    }

    pub fn get_resources(&self) {
        println!("{:?}", self.resources);
    }
}