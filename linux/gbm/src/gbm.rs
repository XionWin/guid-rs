use std::os::unix::prelude::RawFd;

use crate::{Device, Surface, SurfaceFormat, FormatModifier};

#[derive(Debug)]
pub struct Gbm {
    fd: RawFd,
    device: Device,
    surface: Surface,
    surface_format: SurfaceFormat,
    format_modifiers: Vec<FormatModifier>,
    width: libc::c_uint,
    height: libc::c_uint,
}

impl Gbm {
    pub fn new(drm: drm::Drm, surface_format: SurfaceFormat, format_modifiers: Vec<FormatModifier>) -> Self
    {
        let fd = drm.get_fd();
        let crtc = drm.get_crtc();
        let width = crtc.get_width();
        let height = crtc.get_height();

        let device = Device::new(fd);
        let surface = Surface::new_with_modifiers(&device, width, height, surface_format, &format_modifiers);
        Self{
            fd,
            device,
            surface,
            surface_format,
            format_modifiers,
            width,
            height,
        }
    }
    
    pub fn get_device(&self) -> &Device {
        &self.device
    }
}