#[link(name = "c")]
extern "C" {
    pub fn open(device: *const libc::c_char, mode: libc::c_int) -> libc::c_int;
}