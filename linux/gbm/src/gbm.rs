use std::os::unix::prelude::RawFd;

use crate::{Device, Surface, def::{SurfaceFormat, FormatModifier}};

#[derive(Debug)]
pub struct Gbm {
    drm: drm::Drm,
    fd: RawFd,
    surface: Surface,
    surface_format: SurfaceFormat,
    format_modifiers: Vec<FormatModifier>,
    width: libc::c_int,
    height: libc::c_int,
}

impl Gbm {
    pub fn new(drm: drm::Drm, surface_format: SurfaceFormat, format_modifiers: Vec<FormatModifier>) -> Self
    {
        let fd = drm.get_fd();
        let crtc = drm.get_crtc();
        let width = crtc.get_width();
        let height = crtc.get_height();
        let surface = Surface::new_with_modifiers(Device::new(fd), width, height, surface_format, &format_modifiers);
        Self{
            drm,
            fd,
            surface,
            surface_format,
            format_modifiers,
            width,
            height,
        }
    }
    pub fn get_drm(&self) -> &drm::Drm {
        &self.drm
    }
    
    pub fn get_surface(&self) -> &Surface {
        &self.surface
    }
    pub fn get_surface_mut(&mut self) -> &mut Surface {
        &mut self.surface
    }

    
    pub fn get_width(&self) -> libc::c_int {
        self.width
    }
    pub fn get_height(&self) -> libc::c_int {
        self.height
    }
}