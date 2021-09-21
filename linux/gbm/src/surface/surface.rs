use crate::BufferObject;

#[derive(Debug)]
pub struct Surface {
    pub(crate) handle: *const crate::ffi::GbmSurface,
    device: crate::Device,
    swap_callback: (fn(*const std::ffi::c_void, *const std::ffi::c_void), *const std::ffi::c_void, *const std::ffi::c_void),

    bo_handle: *const crate::ffi::GbmBufferObject,
}

impl Surface {
    pub fn new(device: crate::Device, width: libc::c_int, height: libc::c_int, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create(device.get_handle_raw(), width, height, format, flags)
            },
            device,
            swap_callback: (|_, _|{}, std::ptr::null(), std::ptr::null()),
            bo_handle: std::ptr::null(),
        }
    }
    pub fn new_with_modifiers(device: crate::Device, width: libc::c_int, height: libc::c_int, format: crate::def::SurfaceFormat, modifiers: &[crate::def::FormatModifier]) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create_with_modifiers(device.get_handle_raw(), width, height, format, modifiers.as_ptr() as *const _, modifiers.len() as _)
            },
            device,
            swap_callback: (|_, _|{}, std::ptr::null(), std::ptr::null()),
            bo_handle: std::ptr::null(),
        }
    }
    
    pub fn get_device(&self) -> &crate::Device {
        &self.device
    }

    pub fn get_handle(&self) -> libc::c_int {
        self.handle as _
    }

    pub fn register_swap_callback(&mut self, swap_callback: (fn(*const std::ffi::c_void, *const std::ffi::c_void), *const std::ffi::c_void, *const std::ffi::c_void)) {
        self.swap_callback = swap_callback;
    }

    pub fn initialize(&mut self) {
        let (func, param_display, param_surface) =self.swap_callback;
        func(param_display, param_surface);
        
        self.bo_handle = unsafe { crate::ffi::gbm_surface_lock_front_buffer(self.handle) };

        println!("e {:?}", self.bo_handle);

        let bo = BufferObject::new(self.bo_handle);
        println!("e {:?}", bo);
        bo.get_fb(&self.device);

        
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