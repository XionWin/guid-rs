use crate::BufferObject;

#[derive(Debug)]
pub struct Surface {
    pub(crate) handle: *const crate::ffi::GbmSurface,
    device: crate::Device,
    swap_callback: (fn(*const std::ffi::c_void, *const std::ffi::c_void) -> bool, *const std::ffi::c_void, *const std::ffi::c_void),

    bo_handle: *const crate::ffi::GbmBufferObject,
}

impl Surface {
    pub fn new(device: crate::Device, width: libc::c_int, height: libc::c_int, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create(device.get_handle_raw(), width, height, format, flags)
            },
            device,
            swap_callback: (|_, _|{ true }, std::ptr::null(), std::ptr::null()),
            bo_handle: std::ptr::null(),
        }
    }
    pub fn new_with_modifiers(device: crate::Device, width: libc::c_int, height: libc::c_int, format: crate::def::SurfaceFormat, modifiers: &[crate::def::FormatModifier]) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create_with_modifiers(device.get_handle_raw(), width, height, format, modifiers.as_ptr() as *const _, modifiers.len() as _)
            },
            device,
            swap_callback: (|_, _|{ true }, std::ptr::null(), std::ptr::null()),
            bo_handle: std::ptr::null(),
        }
    }
    
    pub fn get_device(&self) -> &crate::Device {
        &self.device
    }

    pub fn get_handle(&self) -> libc::c_int {
        self.handle as _
    }

    pub fn register_swap_callback(&mut self, swap_callback: (fn(*const std::ffi::c_void, *const std::ffi::c_void) -> bool, *const std::ffi::c_void, *const std::ffi::c_void)) {
        self.swap_callback = swap_callback;
    }

    pub fn initialize(&mut self, params: (libc::c_int, libc::c_uint, &Vec<libc::c_uint>, *const std::ffi::c_void), action: fn(params: (libc::c_int, libc::c_uint, &Vec<libc::c_uint>, *const std::ffi::c_void), crate::BufferObject, libc::c_int)) {
        self.lock(params, action);
    }

    pub fn swap_buffer(&mut self, params: (libc::c_int, libc::c_uint, &Vec<libc::c_uint>, *const std::ffi::c_void), action: fn(params: (libc::c_int, libc::c_uint, &Vec<libc::c_uint>, *const std::ffi::c_void), crate::BufferObject, libc::c_int)) {
        self.lock(params, action);
    }

    pub fn lock(&mut self, params: (libc::c_int, libc::c_uint, &Vec<libc::c_uint>, *const std::ffi::c_void), action: fn(params: (libc::c_int, libc::c_uint, &Vec<libc::c_uint>, *const std::ffi::c_void), crate::BufferObject, libc::c_int)) {
        self.swap();

        let last_bo_handle = self.bo_handle;
        self.bo_handle = match unsafe { crate::ffi::gbm_surface_lock_front_buffer(self.handle) } {
            handle if handle == std::ptr::null() => panic!("[GBM]: Failed to lock front buffer"),
            handle => handle,
        };

        let bo = BufferObject::new(self.bo_handle);
        println!("surface initialize bo {:?}", bo);
        let fb = bo.get_fb(&self.device);
        action(params, bo, fb);

        if last_bo_handle != std::ptr::null() {
            unsafe {
                crate::ffi::gbm_surface_release_buffer(self.handle, last_bo_handle);
            }
        }
    }

    fn swap(&self) {
        let (func, param_display, param_surface) =self.swap_callback;
        if !func(param_display, param_surface) {
            panic!("surface swap error");
        }
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