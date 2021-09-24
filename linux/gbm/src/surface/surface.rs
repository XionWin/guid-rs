use std::os::unix::prelude::RawFd;

use crate::BufferObject;

#[derive(Debug)]
pub struct Surface {
    pub(crate) handle: *const crate::ffi::GbmSurface,
    device: crate::Device,
    swap_callback: (
        fn(*const std::ffi::c_void, *const std::ffi::c_void) -> bool,
        *const std::ffi::c_void,
        *const std::ffi::c_void,
    ),

    bo_handle: *const crate::ffi::GbmBufferObject,
}

impl Surface {
    pub fn new(
        device: crate::Device,
        width: libc::c_int,
        height: libc::c_int,
        format: crate::def::SurfaceFormat,
        flags: crate::def::SurfaceFlags,
    ) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create(
                    device.get_handle_raw(),
                    width,
                    height,
                    format,
                    flags,
                )
            },
            device,
            swap_callback: (|_, _| true, std::ptr::null(), std::ptr::null()),
            bo_handle: std::ptr::null(),
        }
    }
    pub fn new_with_modifiers(
        device: crate::Device,
        width: libc::c_int,
        height: libc::c_int,
        format: crate::def::SurfaceFormat,
        modifiers: &[crate::def::FormatModifier],
    ) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create_with_modifiers(
                    device.get_handle_raw(),
                    width,
                    height,
                    format,
                    modifiers.as_ptr() as *const _,
                    modifiers.len() as _,
                )
            },
            device,
            swap_callback: (|_, _| true, std::ptr::null(), std::ptr::null()),
            bo_handle: std::ptr::null(),
        }
    }

    pub fn get_device(&self) -> &crate::Device {
        &self.device
    }

    pub fn get_handle(&self) -> libc::c_int {
        self.handle as _
    }

    pub fn register_swap_callback(
        &mut self,
        swap_callback: (
            fn(*const std::ffi::c_void, *const std::ffi::c_void) -> bool,
            *const std::ffi::c_void,
            *const std::ffi::c_void,
        ),
    ) {
        self.swap_callback = swap_callback;
    }

    pub fn initialize<T>(
        &mut self,
        params: T,
        action: fn(params: T, crate::BufferObject, libc::c_int),
    ) {
        self.lock(params, action);
    }

    pub fn swap_buffers(&mut self, fd: RawFd, crtc_id: libc::c_uint) {
        self.swap();

        let last_bo_handle = self.bo_handle;
        self.bo_handle = match unsafe { crate::ffi::gbm_surface_lock_front_buffer(self.handle) } {
            handle if handle == std::ptr::null() => panic!("[GBM]: Failed to lock front buffer"),
            handle => handle,
        };
        let bo = BufferObject::new(self.bo_handle);
        // println!("surface initialize bo {:?}", bo);
        let fb = bo.get_fb(&self.device);

        let evt_context = drm::def::EventContext {
            version: DRM_CONTEXT_VERSION,
            vblank_handler,
            page_flip_handler,
        };

        let mut user_data = 1;
        match drm::page_flip( fd, crtc_id, fb as _, drm::def::PageFlipFlags::FLIP_EVENT, &mut user_data as *mut libc::c_int as _) {
            result if result != 0 => panic!("page_flip error"),
            _ => {}
        }

        while user_data != 0 {
            let r = drm::handle_event(fd, &evt_context as *const _ as _);
            if r != 0 {
                panic!("handle_event result: {:?}", r);
            }
        }

        if last_bo_handle != std::ptr::null() {
            unsafe {
                crate::ffi::gbm_surface_release_buffer(self.handle, last_bo_handle);
            }
        }
    }

    pub fn lock<T>(&mut self, params: T, action: fn(params: T, crate::BufferObject, libc::c_int)) {
        self.swap();

        let last_bo_handle = self.bo_handle;
        self.bo_handle = match unsafe { crate::ffi::gbm_surface_lock_front_buffer(self.handle) } {
            handle if handle == std::ptr::null() => panic!("[GBM]: Failed to lock front buffer"),
            handle => handle,
        };
        let bo = BufferObject::new(self.bo_handle);
        let fb = bo.get_fb(&self.device);

        action(params, bo, fb);

        if last_bo_handle != std::ptr::null() {
            unsafe {
                crate::ffi::gbm_surface_release_buffer(self.handle, last_bo_handle);
            }
        }
    }

    fn swap(&self) {
        let (func, param_display, param_surface) = self.swap_callback;
        if !func(param_display, param_surface) {
            panic!("surface swap error");
        }
    }

    
}

// impl Drop for Surface {
//     fn drop(&mut self) {
//         unsafe {
//             crate::ffi::gbm_surface_destroy(self.handle);
//             println!("Surface: {:?} droped", self.handle);
//         }
//     }
// }

// pub unsafe extern "C" fn page_flip_handler(_fd: RawFd, _frame: libc::c_uint, _sec: libc::c_uint, _usec: libc::c_uint, data: *mut libc::c_void) {
//     let flag = data as *mut libc::c_int;
//     *flag = 0;
//     println!("123");
// }
// #[no_mangle]
// pub unsafe extern "C" fn page_flip_handler(_fd: RawFd, _frame: libc::c_uint, _sec: libc::c_uint, _usec: libc::c_uint, data: *mut libc::c_void) {
//     let flag = data as *mut libc::c_int;
//     *flag = 0;
//     println!("123");
// }
pub const DRM_CONTEXT_VERSION: libc::c_int = 2;
/**< Desired DRM event context version */

extern fn vblank_handler(
    _fd: libc::c_int,
    _sequence: libc::c_uint,
    _tv_sec: libc::c_uint,
    _tv_usec: libc::c_uint,
    _user_data: *mut libc::c_void,
) {
    
}

/// Helper function for handling page flips.
extern fn page_flip_handler(
    _fd: libc::c_int,
    _sequence: libc::c_uint,
    _tv_sec: libc::c_uint,
    _tv_usec: libc::c_uint,
    user_data: *mut libc::c_void,
) {
    unsafe {
        *(user_data as *mut libc::c_int) = 0;
    }
}