#[repr(C)]
pub struct GbmDevice
{
}

#[link(name = "gbm")]
extern "C" {
    #[allow(improper_ctypes)]
    pub fn gbm_create_device(fd: libc::c_int) -> *const GbmDevice;
}