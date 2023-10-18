use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Alloc {
    AllocationFailed = 0x0019,
    NoMoreResource = 0x001A,
    StatusEmpty = 0x001B,
    StatusFull = 0x001C,
    WouldOverflow = 0x001D,
    HasOverflowed = 0x001E,
    Ownership = 0x001F,
    IsOwner = 0x0020,
}
