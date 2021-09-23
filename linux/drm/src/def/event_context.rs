// #[repr(C)]
// pub struct EventContext {
//     pub version: libc::libc::c_int,
//     pub vblank_handler: *const libc::c_void,
//     pub page_flip_handler: *const libc::c_void,
//     pub page_flip_handler2: *const libc::c_void,
//     pub sequence_handler: *const libc::c_void,
// }

#[repr(C)]
pub struct EventContext {
    pub version: libc::c_int,
    pub vblank_handler: extern fn(fd: libc::c_int,
                                  sequence: libc::c_uint,
                                  tv_sec: libc::c_uint,
                                  tv_usec: libc::c_uint,
                                  user_data: *mut libc::c_void),

    pub page_flip_handler: extern fn(fd: libc::c_int,
                                     sequence: libc::c_uint,
                                     tv_sec: libc::c_uint,
                                     tv_usec: libc::c_uint,
                                     user_data: *mut libc::c_void),
    pub page_flip_handler2: extern fn(fd: libc::c_int,
                                     sequence: libc::c_uint,
                                     tv_sec: libc::c_uint,
                                     tv_usec: libc::c_uint,
                                     user_data: *mut libc::c_void),
}

// impl EventContext {
//     pub fn new(version: libc::c_int, page_flip_handler: *const libc::c_void) -> Self {
//         Self {
//             version,
//             vblank_handler: std::ptr::null_mut(),
//             page_flip_handler,
//         }
//     }
// }
