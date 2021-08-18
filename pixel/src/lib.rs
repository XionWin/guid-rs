pub fn test() {
    let fd = nix::libc::open("/dev/dri/card1", nix::libc::OpenMode::READ_WRITE);
    println!("fd {}", fd);
}
