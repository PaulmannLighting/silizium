#[cfg(feature = "le-stream")]
use le_stream::derive::{FromLeBytes, ToLeBytes};

pub type EmberEUI64 = [u8; 8];
pub type ManKey = [u8; 16];
pub type EmberKeyStructBitmask = u16;

#[cfg_attr(feature = "le-stream", derive(FromLeBytes, ToLeBytes))]
#[derive(Debug, Eq, PartialEq)]
pub struct ManContext {
    core_key_type: ManKey,
    key_index: u8,
    derived_type: u8,
    eui64: [u8; 8],
    multi_network_index: u8,
    flags: u8,
    psa_key_alg_permission: u32,
}

impl ManContext {
    #[must_use]
    pub const fn new(
        core_key_type: ManKey,
        key_index: u8,
        derived_type: u8,
        eui64: EmberEUI64,
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

    #[must_use]
    pub const fn core_key_type(&self) -> &ManKey {
        &self.core_key_type
    }

    #[must_use]
    pub const fn key_index(&self) -> u8 {
        self.key_index
    }

    #[must_use]
    pub const fn derived_type(&self) -> u8 {
        self.derived_type
    }

    #[must_use]
    pub const fn eui64(&self) -> &EmberEUI64 {
        &self.eui64
    }

    #[must_use]
    pub const fn multi_network_index(&self) -> u8 {
        self.multi_network_index
    }

    #[must_use]
    pub const fn flags(&self) -> u8 {
        self.flags
    }

    #[must_use]
    pub const fn psa_key_alg_permission(&self) -> u32 {
        self.psa_key_alg_permission
    }
}

#[cfg_attr(feature = "le-stream", derive(FromLeBytes, ToLeBytes))]
#[derive(Debug, Eq, PartialEq)]
pub struct ManNetworkKeyInfo {
    network_key_set: bool,
    alternate_network_key_set: bool,
    network_key_sequence_number: u8,
    alt_network_key_sequence_number: u8,
    network_key_frame_counter: u32,
}

impl ManNetworkKeyInfo {
    #[must_use]
    pub const fn new(
        network_key_set: bool,
        alternate_network_key_set: bool,
        network_key_sequence_number: u8,
        alt_network_key_sequence_number: u8,
        network_key_frame_counter: u32,
    ) -> Self {
        Self {
            network_key_set,
            alternate_network_key_set,
            network_key_sequence_number,
            alt_network_key_sequence_number,
            network_key_frame_counter,
        }
    }

    #[must_use]
    pub const fn network_key_set(&self) -> bool {
        self.network_key_set
    }

    #[must_use]
    pub const fn alternate_network_key_set(&self) -> bool {
        self.alternate_network_key_set
    }

    #[must_use]
    pub const fn network_key_sequence_number(&self) -> u8 {
        self.network_key_sequence_number
    }

    #[must_use]
    pub const fn alt_network_key_sequence_number(&self) -> u8 {
        self.alt_network_key_sequence_number
    }

    #[must_use]
    pub const fn network_key_frame_counter(&self) -> u32 {
        self.network_key_frame_counter
    }
}

#[cfg_attr(feature = "le-stream", derive(FromLeBytes, ToLeBytes))]
#[derive(Debug, Eq, PartialEq)]
pub struct ManApsKeyMetadata {
    bitmask: EmberKeyStructBitmask,
    outgoing_frame_counter: u32,
    incoming_frame_counter: u32,
    ttl_in_seconds: u16,
}

impl ManApsKeyMetadata {
    #[must_use]
    pub const fn new(
        bitmask: EmberKeyStructBitmask,
        outgoing_frame_counter: u32,
        incoming_frame_counter: u32,
        ttl_in_seconds: u16,
    ) -> Self {
        Self {
            bitmask,
            outgoing_frame_counter,
            incoming_frame_counter,
            ttl_in_seconds,
        }
    }

    #[must_use]
    pub const fn bitmask(&self) -> EmberKeyStructBitmask {
        self.bitmask
    }

    #[must_use]
    pub const fn outgoing_frame_counter(&self) -> u32 {
        self.outgoing_frame_counter
    }

    #[must_use]
    pub const fn incoming_frame_counter(&self) -> u32 {
        self.incoming_frame_counter
    }

    #[must_use]
    pub const fn ttl_in_seconds(&self) -> u16 {
        self.ttl_in_seconds
    }
}
