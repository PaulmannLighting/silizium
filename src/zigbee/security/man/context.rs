use macaddr::MacAddr8;
use num_traits::FromPrimitive;

use crate::zigbee::security::man::{DerivedKeyType, Flags, KeyType};

/// Context for Zigbee Security Manager operations.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::FromLeStream, le_stream::ToLeStream)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Context {
    core_key_type: u8,
    key_index: u8,
    derived_type: u8,
    eui64: MacAddr8,
    multi_network_index: u8,
    flags: Flags,
    psa_key_alg_permission: u32,
}

impl Context {
    /// Creates a new `Context`.
    #[must_use]
    pub const fn new(
        core_key_type: KeyType,
        key_index: u8,
        derived_type: DerivedKeyType,
        eui64: MacAddr8,
        multi_network_index: u8,
        flags: Flags,
        psa_key_alg_permission: u32,
    ) -> Self {
        Self {
            core_key_type: core_key_type as u8,
            key_index,
            derived_type: derived_type as u8,
            eui64,
            multi_network_index,
            flags,
            psa_key_alg_permission,
        }
    }

    /// Returns the type of key being referenced.
    ///
    /// # Errors
    ///
    /// Returns an error if the key type is not recognized.
    pub fn core_key_type(&self) -> Result<KeyType, u8> {
        KeyType::from_u8(self.core_key_type).ok_or(self.core_key_type)
    }

    /// Returns the index of the referenced key.
    #[must_use]
    pub const fn key_index(&self) -> u8 {
        self.key_index
    }

    /// Returns the type of key derivation operation to perform on a key.
    ///
    /// # Errors
    ///
    /// Returns an error if the derived key type is not recognized.
    pub fn derived_type(&self) -> Result<DerivedKeyType, u8> {
        DerivedKeyType::from_u8(self.derived_type).ok_or(self.derived_type)
    }

    /// Return the EUI64 associated with this key.
    #[must_use]
    pub const fn eui64(&self) -> MacAddr8 {
        self.eui64
    }

    /// Returns the multi-network index.
    #[must_use]
    pub const fn multi_network_index(&self) -> u8 {
        self.multi_network_index
    }

    /// Returns the flag bitmask.
    #[must_use]
    pub const fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the algorithm to use with this key (for PSA APIs).
    #[must_use]
    pub const fn psa_key_alg_permission(&self) -> u32 {
        self.psa_key_alg_permission
    }
}
