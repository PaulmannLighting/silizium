//! Library for common data structures used across platforms.
#![no_std]

mod status;
pub mod zigbee;

pub use status::{Space, Status};
