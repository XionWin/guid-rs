use std::{ffi::CStr, vec};

use crate::ffi::EglConfig;

const EGL_PLATFORM_GBM_KHR: libc::c_uint = 0x31D7;

#[derive(Debug)]
pub struct Context {
    gbm: gbm::Gbm,
    display: crate::ffi::EglDisplay,
    config: crate::ffi::EglConfig,
    context: crate::ffi::EglContext,
    surface: crate::ffi::EglSurface,
    version: (libc::c_int, libc::c_int),
    width: libc::c_int,
    height: libc::c_int,
    vertical_synchronization: bool,
}
impl Context {
    pub fn new(gbm: gbm::Gbm) -> Self {
        let display = get_display(&gbm);
        let (major, minor) = get_version(display);
        let config = get_config(display);

        let context_attrib = [
            crate::def::Definition::CONTEXT_CLIENT_VERSION,
            2,
            crate::def::Definition::NONE,
        ];
        let context = get_context(display, config, &context_attrib as _);
        let surface = get_surface(display, config, &gbm);

        Self {
            width: gbm.get_width(),
            height: gbm.get_height(),
            gbm,
            display,
            config,
            context,
            surface,
            version: (major, minor),
            vertical_synchronization: false,
        }
    }

    pub fn init(&self) {
        egl_make_current(self.display, self.surface, self.context);
    }
}

fn get_display(gbm: &gbm::Gbm) -> crate::ffi::EglDisplay {
    
    let version = get_version_by_display(0x0);
    println!("version: {:?}", version);
    let vendor = get_vendor_by_display(0x0);
    println!("vendor: {:?}", vendor);
    let extensions = get_extensions_by_display(0x0);
    println!("extensions: {:?}", extensions);

    let display = match extensions {
        Some(extensions) if extensions.contains("EGL_EXT_platform_base") => {
            get_egl_get_platform_display_ext_func("eglGetPlatformDisplayEXT")(
                EGL_PLATFORM_GBM_KHR,
                gbm.get_device().get_handle(),
                std::ptr::null(),
            )
        }
        None => {
            panic!("Get \"eglGetPlatformDisplayEXT\" error");
        }
        _ => {
            panic!("Can't get \"EGL_EXT_platform_base\" in extensions");
        }
    };

    match display {
        0 => panic!("[EGL] GetDisplay failed.: {:?}", unsafe {
            crate::ffi::eglGetError()
        }),
        display => display,
    }
}

fn get_version(display: crate::ffi::EglDisplay) -> (libc::c_int, libc::c_int) {
    let (mut major, mut minor) = (0, 0);
    if !unsafe { crate::ffi::eglInitialize(display, &mut major, &mut minor) } {
        panic!(
            "[EGL] Failed to initialize EGL display. Error code: {:?}",
            unsafe { crate::ffi::eglGetError() }
        );
    }
    (major, minor)
}

fn get_config(display: crate::ffi::EglDisplay) -> crate::ffi::EglConfig {
    match get_extensions_by_display(display) {
        Some(extensions) if !extensions.contains("EGL_EXT_image_dma_buf_import_modifiers") => {
            panic!("Can't get \"EGL_EXT_image_dma_buf_import_modifiers\" in extensions")
        }
        None => panic!("Get \"eglGetPlatformDisplayEXT\" error"),
        _ => {}
    };
    bind_egl_api(crate::def::RenderAPI::GLES);
    let desired_config = [
        crate::def::Definition::SURFACE_TYPE,
        crate::def::SurfaceType::OpenGLES as _,
        crate::def::Definition::RENDERABLE_TYPE,
        crate::def::Definition::OPENGL_ES3_BIT,
        crate::def::Definition::RED_SIZE,
        8,
        crate::def::Definition::GREEN_SIZE,
        8,
        crate::def::Definition::BLUE_SIZE,
        8,
        crate::def::Definition::ALPHA_SIZE,
        8,
        crate::def::Definition::STENCIL_SIZE,
        8,
        crate::def::Definition::SAMPLE_BUFFERS,
        0,
        crate::def::Definition::SAMPLES,
        0,
        crate::def::Definition::NONE,
    ];
    let size = get_egl_config_count(display, &desired_config as _);
    let mut configs = vec![0; size as _];
    get_egl_config_by_count(display, &desired_config as _, &mut configs);
    configs[0]
}

fn get_context(display: crate::ffi::EglDisplay, config: crate::ffi::EglConfig, attrib_list: *const libc::c_int) -> crate::ffi::EglContext {
    get_egl_context(display, config, attrib_list)
}

fn get_surface(display: crate::ffi::EglDisplay, config: crate::ffi::EglConfig, gbm: &gbm::Gbm) -> crate::ffi::EglSurface {
    get_egl_surface(display, config, gbm.get_surface().get_handle() as _)
}

fn egl_make_current(
    display: crate::ffi::EglDisplay,
    surface: crate::ffi::EglSurface,
    context: crate::ffi::EglContext,
) {
    if !unsafe { crate::ffi::eglMakeCurrent(display, surface, surface, context) } {
        panic!("[EGL] Failed to make current, error {:?}", unsafe {
            crate::ffi::eglGetError()
        });
    }
}

fn get_egl_surface(
    display: crate::ffi::EglDisplay,
    config: EglConfig,
    native_wnd_handle: libc::c_uint,
) -> crate::ffi::EglSurface {
    match unsafe { crate::ffi::eglCreateWindowSurface(display, config, native_wnd_handle, 0 as _) }
    {
        handle if handle == 0 => panic!("[EGL] Failed to create egl surface, error {:?}", unsafe {
            crate::ffi::eglGetError()
        }),
        handle => handle,
    }
}

fn get_egl_context(
    display: crate::ffi::EglDisplay,
    config: EglConfig,
    attrib_list: *const libc::c_int,
) -> crate::ffi::EglContext {
    match unsafe { crate::ffi::eglCreateContext(display, config, 0 as _, attrib_list) } {
        handle if handle == 0 => panic!("[EGL] Failed to create egl context, error {:?}", unsafe {
            crate::ffi::eglGetError()
        }),
        handle => handle,
    }
}

fn get_egl_config_count(
    display: crate::ffi::EglDisplay,
    desired_config: *const libc::c_int,
) -> libc::c_uint {
    let mut num_configs = 0;

    match unsafe {
        crate::ffi::eglChooseConfig(display, desired_config, 0 as _, 0, &mut num_configs)
    } {
        true if num_configs == 0 => {
            panic!("No matched eglConfig");
        }
        false => {
            panic!("eglChooseConfig error");
        }
        _ => { /* expected, do nothing */ }
    };
    num_configs as _
}

fn get_egl_config_by_count(
    display: crate::ffi::EglDisplay,
    desired_config: *const libc::c_int,
    configs: &mut Vec<crate::ffi::EglConfig>,
) {
    let mut num_configs = 0;

    match unsafe {
        crate::ffi::eglChooseConfig(
            display,
            desired_config,
            configs.as_mut_ptr() as _,
            configs.len() as _,
            &mut num_configs,
        )
    } {
        true if num_configs < 1 => {
            panic!("No matched eglConfig");
        }
        false => {
            panic!("eglChooseConfig error");
        }
        _ => { /* expected, do nothing */ }
    }
}

fn bind_egl_api(render_api: crate::def::RenderAPI) {
    if !unsafe { crate::ffi::eglBindAPI(render_api) } {
        panic!("bind_egl_api error")
    }
}

fn get_egl_get_platform_display_ext_func(
    func_name: &str,
) -> extern "C" fn(libc::c_uint, libc::c_uint, *const libc::c_uint) -> libc::c_uint {
    let mut func_name = String::from(func_name)
        .bytes()
        .collect::<Vec<libc::c_char>>();
    func_name.push(b'\0');

    unsafe { std::mem::transmute(crate::ffi::eglGetProcAddress(func_name.as_ptr())) }
}

fn get_version_by_display(display: crate::ffi::EglDisplay) -> Option<String> {
    unsafe {
        get_str_from_ptr(crate::ffi::eglQueryString(display, crate::def::Definition::VERSION) as _)
    }
}

fn get_vendor_by_display(display: crate::ffi::EglDisplay) -> Option<String> {
    unsafe {
        get_str_from_ptr(crate::ffi::eglQueryString(display, crate::def::Definition::VENDOR) as _)
    }
}

fn get_extensions_by_display(display: crate::ffi::EglDisplay) -> Option<String> {
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
