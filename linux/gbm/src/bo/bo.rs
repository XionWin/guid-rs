use std::os::unix::prelude::RawFd;

#[derive(Debug)]
pub struct BufferObject {
    handle: *const crate::ffi::GbmBufferObject,
}

impl BufferObject {
    pub fn new(handle: *const crate::ffi::GbmBufferObject) -> Self {
        Self { handle }
    }

    // pub fn create(
    //     handle: *const crate::ffi::GbmDevice,
    //     width: libc::c_uint,
    //     height: libc::c_uint,
    //     format: crate::def::SurfaceFormat,
    //     flags: crate::def::SurfaceFlags,
    // ) -> Self {
    //     Self {
    //         handle: unsafe { crate::ffi::gbm_bo_create(handle, width, height, format, flags) },
    //     }
    // }

    pub(crate) fn get_fb(&self, device: &crate::Device) -> RawFd {
        match unsafe { crate::ffi::gbm_bo_get_user_data(self.handle) } {
            user_data if user_data == std::ptr::null() => {
                let device_fb = unsafe { crate::ffi::gbm_device_get_fd(device.get_handle_raw()) };
                println!("device_fb: {:#?}", device_fb);

                let width = unsafe { crate::ffi::gbm_bo_get_width(self.handle) };
                let height = unsafe { crate::ffi::gbm_bo_get_height(self.handle) };
                let pixel_format = unsafe { crate::ffi::gbm_bo_get_format(self.handle) };
                let plane_count = unsafe { crate::ffi::gbm_bo_get_plane_count(self.handle) };

                let mut strides = Vec::<libc::c_uint>::new();
                let mut handles = Vec::<libc::c_uint>::new();
                let mut offsets = Vec::<libc::c_uint>::new();
                for plane_index in 0..plane_count {
                    strides.push(unsafe {
                        crate::ffi::gbm_bo_get_stride_for_plane(self.handle, plane_index)
                    });
                    handles.push(unsafe {
                        crate::ffi::gbm_bo_get_handle_for_plane(self.handle, plane_index).u32_
                    });
                    offsets
                        .push(unsafe { crate::ffi::gbm_bo_get_offset(self.handle, plane_index) });
                }
                let flags = 0;

                println!(
                    "width: {:?}, height: {:?}, pixel_format: {:?}, plane_count: {:?}",
                    width, height, pixel_format, plane_count
                );

                println!("strides: {:#?}", strides);
                println!("handles: {:#?}", handles);
                println!("handles: {:#?}", offsets);

                let fb = drm::get_fb2(
                    device_fb,
                    width,
                    height,
                    pixel_format as _,
                    handles.as_ptr(),
                    strides.as_ptr(),
                    offsets.as_ptr(),
                    flags,
                );

                println!("r: {:#x?}", fb);
                unsafe {
                    crate::ffi::gbm_bo_set_user_data(
                        self.handle,
                        fb as _,
                        destroy_user_data_callback,
                    );
                }
                fb
            }
            user_data => user_data as _,
        }
    }
}

extern "C" fn destroy_user_data_callback(
    bo: *const crate::ffi::GbmBufferObject,
    data: *const std::ffi::c_void,
) {
    println!("destroy_user_data_callback bo: {:?} data: {:?}", bo, data);
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
