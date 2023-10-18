use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Command {
    IsInvalid = 0x0047,
    TooLong = 0x0048,
    Incomplete = 0x0049,
}
