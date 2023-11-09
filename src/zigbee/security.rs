use le_stream::derive::{FromLeBytes, ToLeBytes};

#[derive(Debug, Eq, PartialEq, FromLeBytes, ToLeBytes)]
pub struct ManContext {
    core_key_type: [u8; 16],
    key_index: u8,
    derived_type: u8,
    eui64: [u8; 8],
    multi_network_index: u8,
    flags: u8,
    psa_key_alg_permission: u32,
}
