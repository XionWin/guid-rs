fn main() {
    
    let file = libc::File::new("/dev/dri/card1");
    let fd = file.get_fd();
    println!("{:}: {}", file.get_path(), fd);
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
        if gbm.get_device().is_format_supported(surface_format, gbm::def::SurfaceFlags::Linear) {
            println!("{:?}", surface_format);
        } 
    }

    let context = egl::Context::new(gbm);
    println!("{:#?}", context);

    let (major, minor) = context.get_egl_version();
    println!("EGL version major: {}, minor: {}", major, minor);
}
