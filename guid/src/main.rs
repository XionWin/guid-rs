fn main() {
    let file = libc::File::new("/dev/dri/card1");
    let fd = file.get_fd();
    println!("{:}: {}", file.get_path(), fd);
    let resources = drm::Resources::new(fd);
    println!("{:#?}", resources);

    let r = gbm::Device::new(fd);
    println!("{:#?}", r);
    let r = gbm::BufferObject::new(&r);
    println!("{:#?}", r);
}


