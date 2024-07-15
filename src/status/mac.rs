use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum Mac {
    NoData = 0x003E,
    NoAckReceived = 0x003F,
    IndirectTimeout = 0x0040,
    UnknownHeaderType = 0x0041,
    AckHeaderType = 0x0042,
    CommandTransmitFailure = 0x0043,
}
