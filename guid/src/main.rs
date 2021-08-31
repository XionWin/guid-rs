fn main() {
    let file = libc::File::new("/dev/dri/card1");
    let fd = file.get_fd();
    println!("{:}: {}", file.get_path(), fd);
    let resources = drm::Resources::new(fd);
    println!("{:#?}", resources);

    unsafe {
        let r = gbm::gbm_create_device(fd);
        println!("{:#?}", r);
    }
}


