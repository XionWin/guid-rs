#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum ConnectorType {
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
    EDP = 14,
    VIRTUAL = 15,
    DSI = 16,
    DPI = 17,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum ConnectionStatus {
    Connected = 1,
    Disconnected = 2,
    Unknown = 3,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum SubPixel {
    Unknown = 1,
    HorizontalRgb = 2,
    HorizontalBgr = 3,
    VerticalRgb = 4,
    VerticalBgr = 5,
    None = 6,
}