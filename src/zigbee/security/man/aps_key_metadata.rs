use core::time::Duration;

/// This data structure contains the metadata pertaining to an APS key.
#[cfg_attr(
    feature = "le-stream",
    derive(le_stream::derive::FromLeStream, le_stream::derive::ToLeStream)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ApsKeyMetadata {
    bitmask: u16,
    outgoing_frame_counter: u32,
    incoming_frame_counter: u32,
    ttl_in_seconds: u16,
}

impl ApsKeyMetadata {
    /// Creates a new `ApsKeyMetadata`.
    #[must_use]
    pub const fn new(
        bitmask: u16,
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
    pub const fn bitmask(&self) -> u16 {
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
