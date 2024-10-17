use std::fmt::{Display, Formatter, LowerHex, UpperHex};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

/// Space codes common across all platforms.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
#[repr(u32)]
pub enum Space {
    /// Generic space.
    Generic = 0x0000,
    /// Wifi space.
    Wifi = 0x0B00,
    /// Mask Space.
    Mask = 0xFF00,
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SlSpace{self:?}")
    }
}

impl From<Space> for u32 {
    fn from(space: Space) -> Self {
        space.to_u32().expect("could not convert Space to u32")
    }
}

impl LowerHex for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", u32::from(*self))
    }
}

impl TryFrom<u32> for Space {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_u32(value).ok_or(value)
    }
}

impl UpperHex for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010X}", u32::from(*self))
    }
}
