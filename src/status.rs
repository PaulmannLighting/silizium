mod alloc;
mod cli;
mod command;
mod eeprom;
mod flash;
mod io;
mod mac;
mod param;
mod security;
mod state;
mod wifi;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
#[repr(u32)]
pub enum Status {
    Ok = 0x0000,
    Fail = 0x0001,
    State(state::State),
    Alloc(alloc::Alloc),
    Param(param::Param),
    Io(io::Io),
    Eeprom(eeprom::Eeprom),
    Flash(flash::Flash),
    Mac(mac::Mac),
    Cli(cli::Cli),
    Security(security::Security),
    Command(command::Command),
    Wifi(wifi::Wifi),
}

impl FromPrimitive for Status {
    fn from_i64(n: i64) -> Option<Self> {
        u32::try_from(n).ok().and_then(Self::from_u32)
    }

    fn from_u32(n: u32) -> Option<Self> {
        match n {
            0x0000 => Some(Self::Ok),
            0x0001 => Some(Self::Fail),
            0x0002..=0x0018 => state::State::from_u32(n).map(Self::State),
            0x0019..=0x0020 => alloc::Alloc::from_u32(n).map(Self::Alloc),
            0x0021..=0x002D => param::Param::from_u32(n).map(Self::Param),
            0x002E..=0x0037 => io::Io::from_u32(n).map(Self::Io),
            0x0038..=0x0039 => eeprom::Eeprom::from_u32(n).map(Self::Eeprom),
            0x003A..=0x003D => flash::Flash::from_u32(n).map(Self::Flash),
            0x003E..=0x0043 => mac::Mac::from_u32(n).map(Self::Mac),
            0x0044 => cli::Cli::from_u32(n).map(Self::Cli),
            0x0045..=0x0046 => security::Security::from_u32(n).map(Self::Security),
            0x0047..=0x0049 => command::Command::from_u32(n).map(Self::Command),
            0x0B01..=0x0B20 => wifi::Wifi::from_u32(n).map(Self::Wifi),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        u32::try_from(n).ok().and_then(Self::from_u32)
    }
}

impl ToPrimitive for Status {
    fn to_i64(&self) -> Option<i64> {
        self.to_u32().map(i64::from)
    }

    fn to_u32(&self) -> Option<u32> {
        match self {
            Self::Ok => Some(0x0000),
            Self::Fail => Some(0x0001),
            Self::State(state) => state.to_u32(),
            Self::Alloc(alloc) => alloc.to_u32(),
            Self::Param(param) => param.to_u32(),
            Self::Io(io) => io.to_u32(),
            Self::Eeprom(eeprom) => eeprom.to_u32(),
            Self::Flash(flash) => flash.to_u32(),
            Self::Mac(mac) => mac.to_u32(),
            Self::Cli(cli) => cli.to_u32(),
            Self::Security(security) => security.to_u32(),
            Self::Command(command) => command.to_u32(),
            Self::Wifi(wifi) => wifi.to_u32(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        self.to_u32().map(u64::from)
    }
}

impl From<Status> for u32 {
    fn from(status: Status) -> Self {
        status.to_u32().expect("could not convert Status to u32")
    }
}

impl TryFrom<u32> for Status {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_u32(value).ok_or(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Space {
    Generic = 0x0000,
    Wifi = 0x0B00,
    Mask = 0xFF00,
}

impl From<Space> for u32 {
    fn from(space: Space) -> Self {
        space.to_u32().expect("could not convert Space to u32")
    }
}

impl TryFrom<u32> for Space {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_u32(value).ok_or(value)
    }
}
