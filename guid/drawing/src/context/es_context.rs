pub struct ESContext {
    context: egl::Context
}

impl ESContext {
    pub fn new() -> Self {
        let fd = libc::File::new("/dev/dri/card1").get_fd();
        let r = drm::Resources::new(fd);
        println!("{:#?}", r);
        
        let drm = drm::Drm::new(
            r,
            |conn| conn.get_connection_status() == drm::def::ConnectionStatus::Connected
        );

        let mode = drm.get_mode();
        println!("{:#?}", drm);
        println!("{:#?}", mode);

        let gbm = gbm::Gbm::new(drm, gbm::def::SurfaceFormat::ARGB8888, vec![gbm::def::FormatModifier::DRM_FORMAT_MOD_LINEAR]);
        println!("{:#?}", gbm);

        for surface_format in gbm::def::SurfaceFormat::iter() {
            if gbm.get_surface().get_device().is_format_supported(surface_format, gbm::def::SurfaceFlags::Linear) {
                println!("{:?}", surface_format);
            } 
        }

        let mut context = egl::Context::new(gbm, true);
        println!("{:#?}", context);

        println!("GL Extensions: {:?}", gles::get_string(gles::def::StringName::Extensions));
        println!("GL Version: {:?}", gles::get_string(gles::def::StringName::Version));
        println!("GL Sharding Language Version: {:?}", gles::get_string(gles::def::StringName::ShadingLanguageVersion));
        println!("GL Vendor: {:?}", gles::get_string(gles::def::StringName::Vendor));
        println!("GL Renderer: {:?}", gles::get_string(gles::def::StringName::Renderer));

        context.initialize();
        Self {
            context
        }
    }

    pub fn get_context(&mut self) -> &mut egl::Context {
        &mut self.context
    }

    
    pub fn get_width(&self) -> libc::c_int {
        self.context.get_width()
    }

    pub fn get_height(&self) -> libc::c_int {
        self.context.get_height()
    }
}

#[macro_export]
macro_rules! begin_render {
    ($init:ident, $render:ident, $context:expr) => {
        let mut params = $init($context);
        loop {
            $render($context, &mut params);
            $context.get_context().update();
        }
    };
}

