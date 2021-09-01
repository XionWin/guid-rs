#[repr(C)]
pub struct GbmSurface
{
}


#[link(name = "gbm")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn gbm_surface_create(device: *const crate::ffi::GbmDevice, width: libc::c_uint, height: libc::c_uint, format: crate::ffi::def::SurfaceFormat, flags: crate::ffi::def::SurfaceFlags) -> *const GbmSurface;
    pub fn gbm_surface_create_with_modifiers(device: *const crate::ffi::GbmDevice, width: libc::c_uint, height: libc::c_uint, format: crate::ffi::def::SurfaceFormat, modifiers: *const libc::c_ulonglong, count: libc::c_uint) -> *const GbmSurface;
    
    
    pub fn gbm_surface_destroy(handle: *const GbmSurface);
}