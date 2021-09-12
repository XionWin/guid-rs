pub type EGLDisplay = libc::c_uint;

#[link(name = "EGL")]
#[allow(improper_ctypes)]
extern "C" {
    pub fn eglQueryString(display: EGLDisplay, name: libc::c_int) -> libc::c_uint;
    pub fn eglGetProcAddress(func_name: *const libc::c_char) -> *const fn(libc::c_uint, libc::c_uint, *const libc::c_uint) -> libc::c_uint;
}