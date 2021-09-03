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

    println!("{:#?}", drm);

    // let r = gbm::Device::new(fd);
    // println!("{:#?}", r);

    // let r = gbm::Surface::new_with_modifiers(&r, 1080, 1920, gbm::SurfaceFormat::ARGB8888, &vec![gbm::FormatModifier::DRM_FORMAT_MOD_LINEAR]);
    // println!("{:#?}", r);
}


