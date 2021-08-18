

fn main() {
    let file = libc::File::new("/dev/dri/card1");
    let fd = file.get_fd();
    println!("{}", fd);
    let resources = drm::Resources::new(fd);
    resources.get_resources();
}

