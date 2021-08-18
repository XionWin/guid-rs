#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct DrmResources {
    pub count_fbs: libc::c_int,
    pub fbs: *const libc::c_uint,
    pub count_crtcs: libc::c_int,
    pub crtcs: *const libc::c_uint,
    pub count_connectors: libc::c_int,
    pub connectors: *const libc::c_int,
    pub count_encoders: libc::c_int,
    pub encoders: *const libc::c_uint,
    pub min_width: libc::c_uint,
    pub max_width: libc::c_uint,
    pub min_height: libc::c_uint,
    pub max_height: libc::c_uint,
}

fn main() {
    unsafe {
        let mut path = "/dev/dri/card1".bytes().collect::<Vec<libc::c_char>>();
        path.push(b'\0');
        let fd = open(path.as_ptr(), 3);
        println!("{}", fd);
        let ptr_drm = drmModeGetResources(fd);
        println!("{:?}", *ptr_drm);
    }
}

#[link(name = "c")]
extern "C" {
    pub fn open(device: *const libc::c_char, mode: libc::c_int) -> libc::c_int;
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetResources(fd: libc::c_int) -> *const DrmResources;
}
