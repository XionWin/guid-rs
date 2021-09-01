#[derive(Debug)]
pub struct BufferObject {
    handle: *const crate::ffi::GbmBufferObject
}

impl BufferObject {
    pub fn new(device: &crate::Device) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_bo_create(device.handle, 0u32, 0u32, crate::ffi::def::SurfaceFormat::ARGB8888, crate::ffi::def::SurfaceFlags::Linear)
            }
        }
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