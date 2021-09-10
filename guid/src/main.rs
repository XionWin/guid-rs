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


    let gbm = gbm::Gbm::new(drm);
    println!("{:#?}", gbm);

    for surface_format in gbm::SurfaceFormat::iter() {
        if gbm.get_device().is_format_supported(surface_format, gbm::SurfaceFlags::Linear) {
            println!("{:?}", surface_format);
        } 
    }
}
