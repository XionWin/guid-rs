#[repr(C)]
pub union GbmBufferObjectHandle {
    pub ptr: *const libc::c_void,
    pub s32: libc::c_int,
    pub u32_: libc::c_uint,
    pub s64: libc::c_longlong,
    pub u64_: libc::c_ulonglong,
}

#[repr(C)]
pub struct GbmBufferObject
{
}

#[link(name = "gbm")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn gbm_bo_create(handle: *const crate::ffi::GbmDevice, width: libc::c_uint, height: libc::c_uint, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> *const GbmBufferObject;
    pub fn gbm_bo_get_handle_for_plane(handle: *const GbmBufferObject, plane: libc::c_int) -> GbmBufferObjectHandle;
    pub fn gbm_bo_get_handle(handle: *const GbmBufferObject) -> GbmBufferObjectHandle;

    pub fn gbm_bo_destroy(handle: *const GbmBufferObject);
}