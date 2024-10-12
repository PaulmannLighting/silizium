//! Zigbee security management.

#[cfg(feature = "le-stream")]
use le_stream::derive::{FromLeStream, ToLeStream};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::time::Duration;

mod ember {
    pub type Eui64 = u64;
    pub type KeyStructBitmask = u16;
}

/// This data structure contains the key data that is passed into various other functions.
pub type ManKey = [u8; 16];

/// Context for Zigbee Security Manager operations.
#[cfg_attr(feature = "le-stream", derive(FromLeStream, ToLeStream))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManContext {
    core_key_type: ManKey,
    key_index: u8,
    derived_type: u8,
    eui64: ember::Eui64,
    multi_network_index: u8,
    flags: u8,
    psa_key_alg_permission: u32,
}

impl ManContext {
    /// Creates a new `ManContext`.
    #[must_use]
    pub const fn new(
        core_key_type: ManKey,
        key_index: u8,
        derived_type: u8,
        eui64: ember::Eui64,
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
    pub const fn core_key_type(&self) -> &ManKey {
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
    pub const fn eui64(&self) -> &ember::Eui64 {
        &self.eui64
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

/// This data structure contains the metadata pertaining to an network key.
#[cfg_attr(feature = "le-stream", derive(FromLeStream, ToLeStream))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManNetworkKeyInfo {
    network_key_set: bool,
    alternate_network_key_set: bool,
    network_key_sequence_number: u8,
    alt_network_key_sequence_number: u8,
    network_key_frame_counter: u32,
}

impl ManNetworkKeyInfo {
    /// Creates a new `ManNetworkKeyInfo`.
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

    /// Returns whether the network key is set.
    #[must_use]
    pub const fn network_key_set(&self) -> bool {
        self.network_key_set
    }

    /// Returns whether the alternate network key is set.
    #[must_use]
    pub const fn alternate_network_key_set(&self) -> bool {
        self.alternate_network_key_set
    }

    /// Returns the network key sequence number.
    #[must_use]
    pub const fn network_key_sequence_number(&self) -> u8 {
        self.network_key_sequence_number
    }

    /// Returns the alternate network key sequence number.
    #[must_use]
    pub const fn alt_network_key_sequence_number(&self) -> u8 {
        self.alt_network_key_sequence_number
    }

    /// Returns the network key frame counter.
    #[must_use]
    pub const fn network_key_frame_counter(&self) -> u32 {
        self.network_key_frame_counter
    }
}

/// This data structure contains the metadata pertaining to an APS key.
#[cfg_attr(feature = "le-stream", derive(FromLeStream, ToLeStream))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManApsKeyMetadata {
    bitmask: ember::KeyStructBitmask,
    outgoing_frame_counter: u32,
    incoming_frame_counter: u32,
    ttl_in_seconds: u16,
}

impl ManApsKeyMetadata {
    /// Creates a new `ManApsKeyMetadata`.
    #[must_use]
    pub const fn new(
        bitmask: ember::KeyStructBitmask,
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

    /// Returns the bitmask.
    #[must_use]
    pub const fn bitmask(&self) -> ember::KeyStructBitmask {
        self.bitmask
    }

    /// Returns the outgoing frame counter.
    #[must_use]
    pub const fn outgoing_frame_counter(&self) -> u32 {
        self.outgoing_frame_counter
    }

    /// Returns the incoming frame counter.
    #[must_use]
    pub const fn incoming_frame_counter(&self) -> u32 {
        self.incoming_frame_counter
    }

    /// Returns the time-to-live in seconds.
    #[must_use]
    pub const fn ttl_in_seconds(&self) -> u16 {
        self.ttl_in_seconds
    }

    /// Returns the time-to-live as a `Duration`.
    #[must_use]
    pub fn ttl(&self) -> Duration {
        Duration::from_secs(self.ttl_in_seconds.into())
    }
}

/// Security Manager context flags.
///
/// # Documentation
/// [Link](https://docs.silabs.com/d/zigbee-stack-api/7.2.2/zigbee-security-manager#sl-zigbee-sec-man-flags-t).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum ManFlags {
    /// No flags are set.
    None = 0x00,
    /// For export APIs, this flag indicates the `key_index` parameter is valid in the `sl_zb_sec_man_context_t` structure.
    ///
    /// This bit is set by the caller when intending to search for a key by `key_index`.
    ///
    /// This flag has no significance for import APIs.
    KeyIndexIsValid = 0x01,
    /// For export APIs, this flag indicates the eui64 parameter is valid in the `sl_zb_sec_man_context_t` structure.
    ///
    /// This bit is set by the caller when intending to search for a key by eui64.
    /// It is also set when searching by `key_index` and an entry is found.
    ///
    /// This flag has no significance for import APIs.
    EuiIsValid = 0x02,
    /// Internal use only.
    ///
    /// This indicates that the transient key being added is an unconfirmed, updated key.
    /// This bit is set when we add a transient key and the `EmberTcLinkKeyRequestPolicy` policy is
    /// `EMBER_ALLOW_TC_LINK_KEY_REQUEST_AND_GENERATE_NEW_KEY`, whose behavior dictates that we
    /// generate a new, unconfirmed key, send it to the requester, and await for a
    /// Verify Key Confirm message.
    UnconfirmedTransientKey = 0x04,
}

impl From<ManFlags> for u8 {
    fn from(man_flags: ManFlags) -> Self {
        man_flags as Self
    }
}

impl TryFrom<u8> for ManFlags {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
