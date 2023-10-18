use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive, ToPrimitive)]
pub enum Wifi {
    InvalidKey = 0x0B01,
    FirmwareDownloadTimeout = 0xB02,
    UnsupportedMessageId = 0x0B03,
    Warning = 0x0B04,
    NoPacketToReceive = 0x0B05,
    SleepGranted = 0x0B08,
    SleepNotGranted = 0x0B09,
    SecureLinkMacKeyError = 0x0B10,
    SecureLinkMacKeyAlreadyBurned = 0x0B11,
    SecureLinkRamModeNotAllowed = 0x0B12,
    SecureLinkFailedUnknownMode = 0x0B13,
    SecureLinkExchangeFailed = 0x0B14,
    WrongState = 0x0B18,
    ChannelNotAllowed = 0x0B19,
    NoMatchingAp = 0x0B1A,
    ConnectionAborted = 0x0B1B,
    ConnectionTimeout = 0x0B1C,
    ConnectionRejectedByAp = 0x0B1D,
    ConnectionAuthFailure = 0x0B1E,
    RetryExceeded = 0x0B1F,
    TxLifetimeExceeded = 0x0B20,
}
