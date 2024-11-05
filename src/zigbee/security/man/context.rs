use crate::zigbee::security::man::Key;

/// Context for Zigbee Security Manager operations.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::derive::FromLeStream, le_stream::derive::ToLeStream)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Context<Eui64>
where
    Eui64: Copy,
{
    core_key_type: Key,
    key_index: u8,
    derived_type: u8,
    eui64: Eui64,
    multi_network_index: u8,
    flags: u8,
    psa_key_alg_permission: u32,
}

impl<Eui64> Context<Eui64>
where
    Eui64: Copy,
{
    /// Creates a new `Context`.
    #[must_use]
    pub const fn new(
        core_key_type: Key,
        key_index: u8,
        derived_type: u8,
        eui64: Eui64,
        multi_network_index: u8,
        flags: u8,
        psa_key_alg_permission: u32,
    ) -> Self {
        Self {
            core_key_type,
            key_index,
            derived_type,
            eui64,
            multi_network_index,
            flags,
            psa_key_alg_permission,
        }
    }

    /// Returns the type of key being referenced.
    #[must_use]
    pub const fn core_key_type(&self) -> &Key {
        &self.core_key_type
    }

    /// Returns the index of the referenced key.
    #[must_use]
    pub const fn key_index(&self) -> u8 {
        self.key_index
    }

    /// Returns the type of key derivation operation to perform on a key.
    #[must_use]
    pub const fn derived_type(&self) -> u8 {
        self.derived_type
    }

    /// Return the EUI64 associated with this key.
    #[must_use]
    pub const fn eui64(&self) -> Eui64 {
        self.eui64
    }

    /// Returns the multi-network index.
    #[must_use]
    pub const fn multi_network_index(&self) -> u8 {
        self.multi_network_index
    }

    /// Returns the flag bitmask.
    #[must_use]
    pub const fn flags(&self) -> u8 {
        self.flags
    }

    /// Returns the algorithm to use with this key (for PSA APIs).
    #[must_use]
    pub const fn psa_key_alg_permission(&self) -> u32 {
        self.psa_key_alg_permission
    }
}
