extern crate cc;
// use std::path::Path;


fn main() {
    if cfg!(target_os = "linux") {
        // let library_path = Path::new("/home/win/.raspi-build/tools/arm-bcm2708/arm-linux-gnueabihf/bin/");
        cc::Build::new()
            .file("library/libc/libc.c")
            // .include(library_path)
            .include("library/libc")
            .warnings(false)
            .compile("libc");
    }
}