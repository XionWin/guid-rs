#[derive(Debug)]
#[repr(C)]
pub struct DrmResources {
    pub count_fbs: libc::c_int,
    pub fbs: *const libc::c_uint,
    
    pub count_crtcs: libc::c_int,
    pub crtcs: *const libc::c_uint,

    pub count_connectors: libc::c_int,
    pub connectors: *const libc::c_uint,

    pub count_encoders: libc::c_int,
    pub encoders: *const libc::c_uint,

    pub min_width: libc::c_uint,
    pub max_width: libc::c_uint,
    pub min_height: libc::c_uint,
    pub max_height: libc::c_uint,
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetResources(fd: libc::c_int) -> *const DrmResources;
}