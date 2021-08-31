#[repr(C)]
pub struct GbmDevice
{
}

#[link(name = "gbm")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn gbm_create_device(fd: libc::c_int) -> *const GbmDevice;
    pub fn gbm_device_destroy(handle: *const GbmDevice);
}