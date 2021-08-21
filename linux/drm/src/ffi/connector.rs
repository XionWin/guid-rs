use crate::connector::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct DrmConnector {
    pub connector_id: libc::c_uint,
    pub encoder_id: libc::c_uint,
    pub connector_type: ConnectorType,
    pub connector_type_id: libc::c_uint,
    pub connection: ConnectionStatus,
    
    pub mm_width: libc::c_uint,
    pub mm_height: libc::c_uint,
    pub subpixel: SubPixel,

    pub count_modes: libc::c_int,
    pub modes: *const DrmModeInfo,

    pub count_props: libc::c_int,
    pub props: *const libc::c_uint,
    pub prop_values: *const libc::c_ulong,

    pub count_encoders: libc::c_int,
    pub encoders: *const libc::c_uint,
}


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct DrmModeInfo {
    pub clock: libc::c_uint,

    pub hdisplay: libc::c_ushort,
    pub hsync_start: libc::c_ushort,
    pub hsync_end: libc::c_ushort,
    pub htotal: libc::c_ushort,
    pub hskew: libc::c_ushort,
    pub vdisplay: libc::c_ushort,
    pub vsync_start: libc::c_ushort,
    pub vsync_end: libc::c_ushort,
    pub vtotal: libc::c_ushort,
    pub vscan: libc::c_ushort,

    pub vrefresh: libc::c_int,

    pub flags: libc::c_uint,
    pub mode_type: DrmModeType,
    pub name: [libc::c_char; 32usize],
}


#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetConnector(fd: libc::c_int, connector_id: libc::c_uint) -> *const DrmConnector;
}
