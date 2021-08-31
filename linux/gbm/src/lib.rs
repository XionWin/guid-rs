
#[derive(Debug)]
#[repr(C)]
pub struct GbmDevice
{
    _unused: [libc::c_int; 0]
}

#[link(name = "gbm")]
extern "C" {
    pub fn gbm_create_device(fd: libc::c_int) -> *const GbmDevice;
}