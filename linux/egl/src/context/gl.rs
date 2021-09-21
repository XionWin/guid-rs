
#[link(name = "GLESv2")]
#[allow(improper_ctypes)]
extern "C" {
    
    pub fn glClearColor(red: libc::c_float, green: libc::c_float, blue: libc::c_float, alpha: libc::c_float);
    pub fn glClear(mask: libc::c_int);

}
