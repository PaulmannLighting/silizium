/// This data structure contains the metadata pertaining to an network key.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::derive::FromLeStream, le_stream::derive::ToLeStream)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct NetworkKeyInfo {
    network_key_set: bool,
    alternate_network_key_set: bool,
    network_key_sequence_number: u8,
    alt_network_key_sequence_number: u8,
    network_key_frame_counter: u32,
}

impl NetworkKeyInfo {
    /// Creates a new `NetworkKeyInfo`.
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
