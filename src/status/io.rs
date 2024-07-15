use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum Io {
    GenericFailure = 0x002E,
    Timeout = 0x002F,
    Transmit = 0x0030,
    TransmitUnderflow = 0x0031,
    TransmitIncomplete = 0x0032,
    TransmitBusy = 0x0033,
    Receive = 0x0034,
    ObjectRead = 0x0035,
    ObjectWrite = 0x0036,
    MessageTooLong = 0x0037,
}
