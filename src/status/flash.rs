use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Flash {
    WriteInhibited = 0x003A,
    VerifyFailed = 0x003B,
    ProgramFailed = 0x003C,
    EraseFailed = 0x003D,
}
