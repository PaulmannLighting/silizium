use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Eeprom {
    MfgVersionMismatch = 0x0038,
    StackVersionMismatch = 0x0039,
}
