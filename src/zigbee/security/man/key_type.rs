use std::fmt::{Display, LowerHex, UpperHex};

use num_derive::FromPrimitive;

/// The list of supported key types used by Zigbee Security Manager.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum KeyType {
    /// No key type.
    None,
    /// This is the network key, used for encrypting and decrypting network payloads.
    ///
    /// There is only one of these keys in storage.
    Network,
    /// This is the Trust Center Link Key.
    ///
    /// On the joining device, this is the APS key used to communicate with the trust center.
    /// On the trust center, this key can be used as a root key for APS encryption and decryption
    /// when communicating with joining devices (if the security policy has the
    /// `EMBER_TRUST_CENTER_USES_HASHED_LINK_KEY` bit set).
    ///
    /// There is only one of these keys in storage.
    TcLink,
    /// This is a Trust Center Link Key, but it times out after either
    ///
    ///   - `EMBER_TRANSIENT_KEY_TIMEOUT_S` or
    ///   - `EMBER_AF_PLUGIN_NETWORK_CREATOR_SECURITY_NETWORK_OPEN_TIME_S` (if defined),
    ///
    /// whichever is longer.
    ///
    /// This type of key is set on trust centers who wish to open joining with a temporary,
    /// or transient, APS key for devices to join with.
    ///
    /// Joiners who wish to try several keys when joining a network may set several of these types
    /// of keys before attempting to join.
    ///
    /// This is an indexed key, and local storage can fit as many keys as available RAM allows.
    TcLinkWithTimeout,
    /// This is an Application link key.
    ///
    /// On both joining devices and the trust center, this key is used in APS encryption and
    /// decryption when communicating to a joining device.
    ///
    /// This is an indexed key table of size `EMBER_KEY_TABLE_SIZE`, so long as there is sufficient
    /// nonvolatile memory to store keys.
    AppLink,
    /// This is the ZLL encryption key for use by algorithms that require it.
    ZllEncryptionKey,
    /// For ZLL, this is the pre-configured link key used during classical `ZigBee` commissioning.
    ZllPreconfiguredKey,
    /// This is a Green Power Device (GPD) key used on a Proxy device.
    GreenPowerProxyTableKey,
    /// This is a Green Power Device (GPD) key used on a Sink device.
    GreenPowerSinkTableKey,
    /// his is a generic key type intended to be loaded for one-time hashing or crypto operations.
    ///
    /// This key is not persisted.
    ///
    /// Intended for use by the Zigbee stack.
    Internal,
}

impl Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl LowerHex for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04x}", *self as u8)
    }
}

impl UpperHex for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04X}", *self as u8)
    }
}
