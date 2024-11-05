use std::fmt::{Display, Formatter, LowerHex, UpperHex};

/// Space codes common across all platforms.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
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

impl LowerHex for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", *self as u32)
    }
}

impl UpperHex for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010X}", *self as u32)
    }
}
