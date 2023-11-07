use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidStatus(u32),
    InvalidSpace(u32),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStatus(status) => write!(f, "invalid status: {status}"),
            Self::InvalidSpace(space) => write!(f, "invalid space: {space}"),
        }
    }
}
