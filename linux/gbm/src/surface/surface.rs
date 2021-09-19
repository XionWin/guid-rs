use std::os::unix::prelude::RawFd;

use crate::BufferObject;

#[derive(Debug)]
pub struct Surface {
    pub(crate) handle: *const crate::ffi::GbmSurface,
    swap_callback: fn(),

    bo_handle: *const crate::ffi::GbmBufferObject,
}

impl Surface {
    pub fn new(device: &crate::Device, width: libc::c_int, height: libc::c_int, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create(device.get_handle_raw(), width, height, format, flags)
            },
            swap_callback: ||{},
            bo_handle: std::ptr::null(),
        }
    }
    pub fn new_with_modifiers(device: &crate::Device, width: libc::c_int, height: libc::c_int, format: crate::def::SurfaceFormat, modifiers: &[crate::def::FormatModifier]) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create_with_modifiers(device.get_handle_raw(), width, height, format, modifiers.as_ptr() as *const _, modifiers.len() as _)
            },
            swap_callback: ||{},
            bo_handle: std::ptr::null(),
        }
    }

    pub fn get_handle(&self) -> libc::c_int {
        self.handle as _
    }

    pub fn register_swap_callback(&mut self, callback: fn()) {
        self.swap_callback = callback;
    }

    pub fn initialize(&mut self) {
        // (self.swap_callback)();
        
        self.bo_handle = unsafe { crate::ffi::gbm_surface_lock_front_buffer(self.handle) };
        
        println!("e {:?}", self.bo_handle);
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::gbm_surface_destroy(self.handle);
            println!("Surface: {:?} droped", self.handle);
        }
    }
}