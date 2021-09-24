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
    pub fn new(gbm: gbm::Gbm) -> Self {
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
            vertical_synchronization: false,
        }
    }

    pub fn initialize(&mut self) {
        let drm_fd = self.gbm.get_drm().get_fd();
        let drm_crtc_id = self.gbm.get_drm().get_crtc().get_id();
        let drm_connector_ids = &vec![self.gbm.get_drm().get_connector().get_id()];
        let drm_mode = self.gbm.get_drm().get_mode().get_handle();

        let surface = self.gbm.get_surface_mut();

        let display_handle = self.display;
        let surface_handle = self.surface;

        let func = |display: *const std::ffi::c_void, surface: *const std::ffi::c_void| {
            unsafe {
                crate::ffi::eglSwapBuffers(display as _, surface as _)
             }
        };
        surface.register_swap_callback((func, display_handle as _, surface_handle as _));

        surface.initialize((drm_fd, drm_crtc_id, drm_connector_ids, drm_mode), |params, _bo, fb| {
            let (drm_fd, drm_crtc_id, drm_connector_ids, drm_mode) = params;
            match drm::set_crtc(drm_fd, drm_crtc_id, fb as _, 0, 0, drm_connector_ids.as_ptr(), drm_connector_ids.len() as _, drm_mode) {
                result if result == 0 => result,
                _ => panic!("surface initialize set_crtc error")
            }
           ;
        });
    }

    pub fn render(&mut self) {
        let fd = self.gbm.get_drm().get_fd();
        let crtc_id = self.gbm.get_drm().get_crtc().get_id();

        let surface = self.gbm.get_surface_mut();
        let mut counter = 9u64;
        let mut value = 9u64;
        let mut direction = true;

        let mut last_tick = std::time::SystemTime::now();
        loop {
            unsafe {
                crate::context::gl::glClearColor(value as f32 / 255f32, value as f32 / 255f32, value as f32 / 255f32, 1.0);
                crate::context::gl::glClear(0x00004000);
            }
            surface.swap_buffers(fd, crtc_id);

            match direction {
                true => value += 1,
                false => value -= 1,
            }
            match value {
                v if v > 255 => direction = false,
                v if v <= 0 => direction = true,
                _ => {},
            }

            counter += 1;
            

            match last_tick.elapsed() {
                Ok(elapsed) if elapsed.as_secs() > 1 => {
                    let fps = counter as f64 / elapsed.as_millis() as f64 * 1000f64;
                    println!("fps: {:?}", fps as u32);
                    counter = 0;
                    last_tick = std::time::SystemTime::now();
                }
                _ => {}
            }
        }
    }

}


