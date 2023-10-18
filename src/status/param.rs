use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Param {
    InvalidParameter = 0x0021,
    NullPointer = 0x0022,
    InvalidConfiguration = 0x0023,
    InvalidMode = 0x0024,
    InvalidHandle = 0x0025,
    InvalidType = 0x0026,
    InvalidIndex = 0x0027,
    InvalidRange = 0x0028,
    InvalidKey = 0x0029,
    InvalidCredentials = 0x002A,
    InvalidCount = 0x002B,
    NotFound = 0x002C,
    AlreadyExists = 0x002D,
}
