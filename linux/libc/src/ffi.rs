#[link(name = "c")]
extern "C" {
    pub fn open(device: *const libc::c_char, mode: crate::OpenFlags) -> libc::c_int;
}