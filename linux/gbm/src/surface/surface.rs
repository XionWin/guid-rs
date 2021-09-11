#[derive(Debug)]
pub struct Surface {
    pub(crate) handle: *const crate::ffi::GbmSurface
}

impl Surface {
    pub fn new(device: &crate::Device, width: libc::c_uint, height: libc::c_uint, format: crate::def::SurfaceFormat, flags: crate::def::SurfaceFlags) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create(device.get_handle_raw(), width, height, format, flags)
            }
        }
    }
    pub fn new_with_modifiers(device: &crate::Device, width: libc::c_uint, height: libc::c_uint, format: crate::def::SurfaceFormat, modifiers: &[crate::def::FormatModifier]) -> Self {
        Self {
            handle: unsafe {
                crate::ffi::gbm_surface_create_with_modifiers(device.get_handle_raw(), width, height, format, modifiers.as_ptr() as *const _, modifiers.len() as _)
            }
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