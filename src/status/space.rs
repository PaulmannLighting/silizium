use core::fmt::{self, Display, LowerHex, UpperHex};

/// Space codes common across all platforms.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Space {
    /// Generic space.
    Generic = 0x0000,
    /// Wi-Fi space.
    Wifi = 0x0B00,
    /// Mask Space.
    Mask = 0xFF00,
}

impl Space {
    /// Return the name of the Space.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generic => "SL_STATUS_GENERIC_SPACE",
            Self::Wifi => "SL_STATUS_WIFI_SPACE",
            Self::Mask => "SL_STATUS_SPACE_MASK",
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl LowerHex for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010x}", *self as u32)
    }
}

impl UpperHex for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#010X}", *self as u32)
    }
}
