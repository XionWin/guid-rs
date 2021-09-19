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
        unsafe {
            let r = crate::ffi::eglSwapBuffers(self.display, self.surface);
            println!("rrrrr: {:?}", r);
        }
        self.gbm.get_surface_mut().initialize();
    }

}
