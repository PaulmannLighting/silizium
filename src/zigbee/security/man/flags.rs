use core::fmt::{self, Display};

use bitflags::bitflags;

/// Security Manager context flags.
///
/// # Documentation
/// [Link](https://docs.silabs.com/d/zigbee-stack-api/7.2.2/zigbee-security-manager#sl-zigbee-sec-man-flags-t).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Flags(u8);

bitflags! {
    impl Flags: u8 {
        /// No flags are set.
        const NONE = 0x00;
        /// For export APIs, this flag indicates the `key_index` parameter is valid in the `sl_zb_sec_man_context_t` structure.
        ///
        /// This bit is set by the caller when intending to search for a key by `key_index`.
        ///
        /// This flag has no significance for import APIs.
        const KEY_INDEX_IS_VALID = 0b0000_0001;
        /// For export APIs, this flag indicates the eui64 parameter is valid in the `sl_zb_sec_man_context_t` structure.
        ///
        /// This bit is set by the caller when intending to search for a key by eui64.
        /// It is also set when searching by `key_index` and an entry is found.
        ///
        /// This flag has no significance for import APIs.
        const EUI_IS_VALID = 0b0000_0010;
        /// Internal use only.
        ///
        /// This indicates that the transient key being added is an unconfirmed, updated key.
        /// This bit is set when we add a transient key and the `EmberTcLinkKeyRequestPolicy` policy is
        /// `EMBER_ALLOW_TC_LINK_KEY_REQUEST_AND_GENERATE_NEW_KEY`, whose behavior dictates that we
        /// generate a new, unconfirmed key, send it to the requester, and await for a
        /// Verify Key Confirm message.
        const UNCONFIRMED_TRANSIENT_KEY = 0b0000_0100;
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
