pub type EglDisplay = libc::c_uint;
pub type EglConfig = libc::c_uint;
pub type EglContext = libc::c_uint;

#[link(name = "EGL")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn eglGetError() -> crate::def::ErrorCode;
    pub fn eglQueryString(display: EglDisplay, name: libc::c_int) -> libc::c_uint;
    pub fn eglGetProcAddress(func_name: *const libc::c_char) -> *const fn(libc::c_uint, libc::c_uint, *const libc::c_uint) -> libc::c_uint;
    pub fn eglInitialize(display: EglDisplay, major: *mut libc::c_int, minor: *mut libc::c_int) -> bool;
    pub fn eglBindAPI(api: crate::def::RenderAPI) -> bool;

    pub fn eglChooseConfig(display: EglDisplay, attrib_list: *const libc::c_int, configs: *mut EglConfig, config_size: libc::c_int, num_config: *mut libc::c_int) -> bool;
    pub fn eglCreateContext(display: EglDisplay, config: EglConfig, share_context: EglContext, attrib_list: *const libc::c_int) -> EglContext;


}