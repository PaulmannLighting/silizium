use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum State {
    Invalid = 0x0002,
    NotReady = 0x0003,
    Busy = 0x0004,
    InProgress = 0x0005,
    Abort = 0x0006,
    Timeout = 0x0007,
    Permission = 0x0008,
    WouldBlock = 0x0009,
    Idle = 0x000A,
    IsWaiting = 0x000B,
    NoneWaiting = 0x000C,
    Suspended = 0x000D,
    NotAvailable = 0x000E,
    NotSupported = 0x000F,
    Initialization = 0x0010,
    NotInitialized = 0x0011,
    AlreadyInitialized = 0x0012,
    Deleted = 0x0013,
    Isr = 0x0014,
    NetworkUp = 0x0015,
    NetworkDown = 0x0016,
    NotJoined = 0x0017,
    NoBeacons = 0x0018,
}
