//! Zigbee security management.

use std::time::Duration;

/// This data structure contains the key data that is passed into various other functions.
pub type ManKey = [u8; 16];

/// Context for Zigbee Security Manager operations.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::derive::FromLeStream, le_stream::derive::ToLeStream)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ManContext<Eui64>
where
    Eui64: Copy,
{
    core_key_type: ManKey,
    key_index: u8,
    derived_type: u8,
    eui64: Eui64,
    multi_network_index: u8,
    flags: u8,
    psa_key_alg_permission: u32,
}

impl<Eui64> ManContext<Eui64>
where
    Eui64: Copy,
{
    /// Creates a new `ManContext`.
    #[must_use]
    pub const fn new(
        core_key_type: ManKey,
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

/// This data structure contains the metadata pertaining to an network key.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::derive::FromLeStream, le_stream::derive::ToLeStream)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::derive::FromLeStream, le_stream::derive::ToLeStream)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ManApsKeyMetadata<Bitmask>
where
    Bitmask: Copy,
{
    bitmask: Bitmask,
    outgoing_frame_counter: u32,
    incoming_frame_counter: u32,
    ttl_in_seconds: u16,
}

impl<Bitmask> ManApsKeyMetadata<Bitmask>
where
    Bitmask: Copy,
{
    /// Creates a new `ManApsKeyMetadata`.
    #[must_use]
    pub const fn new(
        bitmask: Bitmask,
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
    pub const fn bitmask(&self) -> Bitmask {
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
#[cfg_attr(feature = "num-traits", derive(num_derive::FromPrimitive))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
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

/// The list of supported key types used by Zigbee Security Manager.
#[cfg_attr(feature = "num-traits", derive(num_derive::FromPrimitive))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum ManKeyType {
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
