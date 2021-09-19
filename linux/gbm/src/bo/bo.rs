use crate::gbm;

#[derive(Debug)]
pub struct BufferObject {
    handle: *const crate::ffi::GbmBufferObject,
}

impl BufferObject {
    pub fn new(handle: *const crate::ffi::GbmBufferObject) -> Self {
        Self {
            handle
        }
    }

    pub fn create(handle: *const crate::ffi::GbmDevice, width: libc::c_uint, height: libc::c_uint, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_bo_create(handle, width, height, format, flags)
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_handle(&self) -> crate::ffi::GbmBufferObjectHandle {
        unsafe {
            crate::ffi::gbm_bo_get_handle(self.handle)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn get_handle_for_plane(&self, plane: libc::c_int) -> crate::ffi::GbmBufferObjectHandle {
        unsafe {
            crate::ffi::gbm_bo_get_handle_for_plane(self.handle, plane)
        }
    }

    pub(crate) fn get_fb(&self, device: &crate::Device) -> libc::c_int {
        let user_data = unsafe { crate::ffi::gbm_bo_get_user_data(self.handle) };
        if user_data != std::ptr::null() {
            
        }
        0
    }


}

impl Drop for BufferObject {
    fn drop(&mut self) {
        unsafe {
            if self.handle as u32 == 0 {
                println!("Err");
                return;
            }
            crate::ffi::gbm_bo_destroy(self.handle);
            println!("BufferObject: {:?} droped", self.handle);
        }
    }
}