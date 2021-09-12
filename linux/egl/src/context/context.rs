use std::ffi::CStr;

#[derive(Debug)]
pub struct Context {}
const EGL_PLATFORM_GBM_KHR: libc::c_uint = 0x31D7;
impl Context {
    pub fn new(gbm: gbm::Gbm) {
        let version = get_version_by_display(0x0);
        println!("version: {:?}", version);
        let vendor = get_vendor_by_display(0x0);
        println!("vendor: {:?}", vendor);
        let extensions = get_extensions_by_display(0x0);
        println!("extensions: {:?}", extensions);

        match extensions {
            Some(extensions) if extensions.contains("EGL_EXT_platform_base") => {
                let egl_get_platform_display_ext_func_handle = get_egl_get_platform_display_ext_func("eglGetPlatformDisplayEXT")(
                    EGL_PLATFORM_GBM_KHR,
                    gbm.get_device().get_handle(),
                    std::ptr::null(),
                );
                println!("egl display handle: {:#x}", egl_get_platform_display_ext_func_handle);
            }
            None => {
                panic!("Get egl display error");
            }
            _ => {
                panic!("Can't get \"EGL_EXT_platform_base\" in extensions");
            }
        }
    }
}

fn get_egl_get_platform_display_ext_func (
    func_name: &str,
) -> extern "C" fn(libc::c_uint, libc::c_uint, *const libc::c_uint) -> libc::c_uint {
    let mut func_name =  String::from(func_name).bytes().collect::<Vec<libc::c_char>>();
    func_name.push(b'\0');

    unsafe {
        std::mem::transmute(crate::ffi::eglGetProcAddress(func_name.as_ptr()))
    }
}

fn get_version_by_display(display: crate::ffi::EGLDisplay) -> Option<String> {
    unsafe {
        get_str_from_ptr(crate::ffi::eglQueryString(display, crate::def::Definition::VERSION) as _)
    }
}

fn get_vendor_by_display(display: crate::ffi::EGLDisplay) -> Option<String> {
    unsafe {
        get_str_from_ptr(crate::ffi::eglQueryString(display, crate::def::Definition::VENDOR) as _)
    }
}

fn get_extensions_by_display(display: crate::ffi::EGLDisplay) -> Option<String> {
    unsafe {
        get_str_from_ptr(
            crate::ffi::eglQueryString(display, crate::def::Definition::EXTENSIONS) as _,
        )
    }
}

pub fn get_str_from_ptr(ptr: *const libc::c_char) -> Option<String> {
    match ptr {
        ptr if ptr != std::ptr::null() => Some(String::from(
            unsafe { CStr::from_ptr(ptr) }.to_str().unwrap(),
        )),
        _ => None,
    }
}
