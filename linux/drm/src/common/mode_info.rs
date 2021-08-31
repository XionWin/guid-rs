use std::{ffi::CStr};

#[derive(Debug)]
pub struct ModeInfo {
    handle: *const crate::ffi::DrmModeInfo,
    clock: libc::c_uint,

    hdisplay: libc::c_ushort,
    hsync_start: libc::c_ushort,
    hsync_end: libc::c_ushort,
    htotal: libc::c_ushort,
    hskew: libc::c_ushort,
    
    vdisplay: libc::c_ushort,
    vsync_start: libc::c_ushort,
    vsync_end: libc::c_ushort,
    vtotal: libc::c_ushort,
    vscan: libc::c_ushort,
    vrefresh: libc::c_int,

    flags: libc::c_uint,
    mode_type: super::define::DrmModeType,
    name: String,
}

impl ModeInfo {
    pub fn new(mi: &crate::ffi::DrmModeInfo) -> Self {
        Self {
            handle: mi as *const crate::ffi::DrmModeInfo,
            clock: mi.clock,
            hdisplay: mi.hdisplay,
            hsync_start: mi.hsync_start,
            hsync_end: mi.hsync_end,
            htotal: mi.htotal,
            hskew: mi.hskew,
            vdisplay: mi.vdisplay,
            vsync_start: mi.vsync_start,
            vsync_end: mi.vsync_end,
            vtotal: mi.vtotal,
            vscan: mi.vscan,
            vrefresh: mi.vrefresh,
            flags: mi.flags,
            mode_type: mi.mode_type,
            name: get_name(mi),
        }
    }
}

fn get_name(mi: &crate::ffi::DrmModeInfo) -> String {
    String::from(unsafe {CStr::from_ptr(mi.name.as_ptr())}.to_str().unwrap())
}

