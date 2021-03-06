use std::os::unix::prelude::RawFd;

#[derive(Debug)]
#[repr(C)]
pub struct DrmConnector {
    pub connector_id: libc::c_uint,
    pub encoder_id: libc::c_uint,
    pub connector_type: crate::def::ConnectorType,
    pub connector_type_id: libc::c_uint,
    pub connection: crate::def::ConnectionStatus,
    
    pub mm_width: libc::c_uint,
    pub mm_height: libc::c_uint,
    pub subpixel: crate::def::SubPixel,

    pub count_modes: libc::c_int,
    pub modes: *const super::mode_info::DrmModeInfo,

    pub count_props: libc::c_int,
    pub props: *const libc::c_uint,
    pub prop_values: *const libc::c_ulong,

    pub count_encoders: libc::c_int,
    pub encoders: *const libc::c_uint,
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetConnector(fd: RawFd, connector_id: libc::c_uint) -> *const DrmConnector;
    pub fn drmModeFreeConnector(handle: *const DrmConnector);
}
