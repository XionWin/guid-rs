fn main() {
    
    let file = libc::File::new("/dev/dri/card1");
    let fd = file.get_fd();
    println!("{:}: {}", file.get_path(), fd);
    let r = drm::Resources::new(fd);
    println!("{:#?}", r);
        
    let drm = drm::Drm::new(
        r,
        |conn| conn.get_connection_status() == drm::ConnectionStatus::Connected
    );

    let mode = drm.get_mode();

    println!("{:#?}", drm);
    println!("{:#?}", mode);

    let r = gbm::Device::new(fd);
    println!("{:#?}", r);

    for surface_format in gbm::SurfaceFormat::iter() {
        if r.is_format_supported(surface_format, gbm::SurfaceFlags::Linear) {
            println!("{:?}", surface_format);
        }
    }

    let crtc = drm.get_crtc();
    let r = gbm::Surface::new_with_modifiers(&r, crtc.get_width(), crtc.get_height(), gbm::SurfaceFormat::ARGB8888, &vec![gbm::FormatModifier::DRM_FORMAT_MOD_LINEAR]);
            println!("{:#?}", r);
}
