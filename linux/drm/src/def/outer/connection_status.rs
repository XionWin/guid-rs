#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum ConnectionStatus {
    Connected = 1,
    Disconnected = 2,
    Unknown = 3,
}