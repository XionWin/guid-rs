use std::os::unix::prelude::RawFd;

use super::extension::*;

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
    pub fn new(gbm: gbm::Gbm, vertical_synchronization: bool) -> Self {
        let display = get_display(&gbm);
        let (major, minor) = egl_initialize(display);
        let config = get_config(display);

        let context_attrib = [
            crate::def::Definition::CONTEXT_CLIENT_VERSION,
            2,
            crate::def::Definition::NONE,
        ];
        let context = get_context(display, config, &context_attrib as _);
        let surface = get_surface(display, config, &gbm);

        egl_make_current(display, surface, context);

        Self {
            width: gbm.get_width(),
            height: gbm.get_height(),
            gbm,
            display,
            config,
            context,
            surface,
            version: (major, minor),
            vertical_synchronization,
        }
    }

    pub fn get_width(&self) -> libc::c_int {
        self.width
    }

    pub fn get_height(&self) -> libc::c_int {
        self.height
    }

    pub fn initialize(&mut self) {
        let surface = self.gbm.get_surface_mut();

        let display_handle = self.display;
        let surface_handle = self.surface;

        let func = |display: *const libc::c_void, surface: *const libc::c_void| {
            unsafe {
                crate::ffi::eglSwapBuffers(display as _, surface as _)
             }
        };
        surface.register_swap_callback((func, display_handle as _, surface_handle as _));

        
        let (_, fb) = surface.lock();
        let drm_fd = self.gbm.get_drm().get_fd();
        let drm_crtc_id = self.gbm.get_drm().get_crtc().get_id();
        let drm_connector_ids = &vec![self.gbm.get_drm().get_connector().get_id()];
        let drm_mode = self.gbm.get_drm().get_mode().get_handle();
        match drm::set_crtc(drm_fd, drm_crtc_id, fb as _, 0, 0, drm_connector_ids.as_ptr(), drm_connector_ids.len() as _, drm_mode) {
            result if result == 0 => result,
            _ => panic!("surface initialize set_crtc error")
        };
    }

    pub fn update(&mut self) {
        let fd = self.gbm.get_drm().get_fd();
        let crtc_id = self.gbm.get_drm().get_crtc().get_id();

        let surface = self.gbm.get_surface_mut();
        let (_, fb) = surface.lock();
        if self.vertical_synchronization {
            vertical_synchronization(fd, crtc_id, fb);
        }
    }

}

fn vertical_synchronization(fd: RawFd, crtc_id: libc::c_uint, fb: libc::c_uint) {
    let evt_context = drm::def::EventContext {
        version: DRM_CONTEXT_VERSION,
        vblank_handler,
        page_flip_handler,
    };

    let mut user_data = 1;
    match drm::page_flip( fd, crtc_id, fb as _, drm::def::PageFlipFlags::FLIP_EVENT, &mut user_data as *mut libc::c_int as _) {
        result if result != 0 => panic!("page_flip error"),
        _ => {}
    }

    while user_data != 0 {
        let r = drm::handle_event(fd, &evt_context as *const _ as _);
        if r != 0 {
            panic!("handle_event result: {:?}", r);
        }
    }
}


const DRM_CONTEXT_VERSION: libc::c_int = 2;
extern fn vblank_handler(
    _fd: libc::c_int,
    _sequence: libc::c_uint,
    _tv_sec: libc::c_uint,
    _tv_usec: libc::c_uint,
    _user_data: *mut libc::c_void,
) {}
extern fn page_flip_handler(
    _fd: libc::c_int,
    _sequence: libc::c_uint,
    _tv_sec: libc::c_uint,
    _tv_usec: libc::c_uint,
    user_data: *mut libc::c_void,
) {
    unsafe {
        *(user_data as *mut libc::c_int) = 0;
    }
}


