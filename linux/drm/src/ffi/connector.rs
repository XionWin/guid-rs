#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct drmConnector
{
    pub connector_id: libc::c_uint,
    pub encoder_id: libc::c_uint,
    pub connector_type: ConnectorType,
    pub connector_type_id: libc::c_uint,
    pub connection: ConnectionStatus,
    pub mmWidth: libc::c_uint,
    pub mmHeight: libc::c_uint,
    pub subpixel: SubPixel,

    pub count_modes: libc::c_int,
    pub modes: *const ModeInfo,

    pub count_props: libc::c_int,
    pub props: *const libc::c_uint,
    pub prop_values: *const libc::c_ulong,

    pub count_encoders: libc::c_int,
    pub encoders: *const libc::c_uint,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ConnectorType
{
    Unknown = 0,
    VGA = 1,
    DVII = 2,
    DVID = 3,
    DVIA = 4,
    Composite = 5,
    SVIDEO = 6,
    LVDS = 7,
    Component = 8,
    PinDIN9 = 9,
    DisplayPort = 10,
    HDMIA = 11,
    HDMIB = 12,
    TV = 13,
    eDP = 14,
    VIRTUAL = 15,
    DSI = 16,
    DPI = 17
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum ConnectionStatus
{
    Connected = 1,
    Disconnected = 2,
    Unknown = 3
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum SubPixel
{
    Unknown = 1,
    HorizontalRgb = 2,
    HorizontalBgr = 3,
    VerticalRgb = 4,
    VerticalBgr = 5,
    None = 6
}


#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct ModeInfo
{
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
    pub type_: DrmModeType,
    pub name: [libc::c_char; 32usize],
}


#[repr(C)]
#[allow(non_snake_case)]
bitflags! {
    pub struct DrmModeType: u32 {
        const BuiltIn = 1<<0;
        const Cock_C = ((1<<1) | 1<<0);
        const CRTC_C = ((1<<2) | 1<<0);
        const Preferred = 1<<3;
        const Default = 1<<4;
        const UserDefault = 1<<5;
        const Driver = 1<<6;
    }
}

#[link(name = "drm")]
extern "C" {
    pub fn drmModeGetConnector(fd: libc::c_int, connector_id: libc::c_uint) -> *const drmConnector;
}