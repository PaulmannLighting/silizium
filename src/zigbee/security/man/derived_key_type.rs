use std::fmt::{Display, LowerHex, UpperHex};

use num_derive::FromPrimitive;

/// Derived keys are calculated when performing Zigbee crypto operations.
///
/// The stack makes use of these derivations.
///
/// Compounding derivations can be specified by using an or-equals on two derived types if
/// applicable; this is limited to performing the key-transport, key-load, or verify-key hashes on
/// either the TC Swap Out or TC Hashed Link keys.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u16)]
pub enum DerivedKeyType {
    /// Perform no derivation; use the key as is.
    None = 0x0000,
    /// Perform the Key-Transport-Key hash.
    TransportKey = 0x0001,
    /// Perform the Key-Load-Key hash.
    LoadKey = 0x0002,
    /// Perform the Verify Key hash.
    VerifyKey = 0x0004,
    /// Perform a simple AES hash of the key for TC backup.
    TcSwapOutKey = 0x0008,
    /// For a TC using hashed link keys, hashed the root key against the supplied EUI in context.
    TcHashedLinkKey = 0x0010,
}

impl Display for DerivedKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl LowerHex for DerivedKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06x}", *self as u16)
    }
}

impl UpperHex for DerivedKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#06X}", *self as u16)
    }
}
