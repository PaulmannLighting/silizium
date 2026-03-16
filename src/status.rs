use core::fmt::{Display, Formatter, LowerHex, UpperHex};

use num_derive::FromPrimitive;

pub use self::space::Space;

mod space;

/// Status codes common across all platforms.
///
/// # Documentation
///
/// See [docs.silabs.com](https://docs.silabs.com/mcu/5.9/efr32bg1/group-sl-status) for further information.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[repr(u32)]
pub enum Status {
    /// No error.
    Ok = 0x0000,

    /// Generic error.
    Fail = 0x0001,

    /// Generic invalid state error.
    InvalidState = 0x0002,

    /// Module is not ready for requested operation.
    NotReady = 0x0003,

    /// Module is busy and cannot carry out requested operation.
    Busy = 0x0004,

    /// Operation is in progress and not yet complete (pass or fail).
    InProgress = 0x0005,

    /// Operation aborted.
    Abort = 0x0006,

    /// Operation timed out.
    Timeout = 0x0007,

    /// Operation not allowed per permissions.
    Permission = 0x0008,

    /// Non-blocking operation would block.
    WouldBlock = 0x0009,

    /// Operation/module is Idle, cannot carry requested operation.
    Idle = 0x000A,

    /// Operation cannot be done while construct is waiting.
    IsWaiting = 0x000B,

    /// No task/construct waiting/pending for that action/event.
    NoneWaiting = 0x000C,

    /// Operation cannot be done while construct is suspended.
    Suspended = 0x000D,

    /// Feature not available due to software configuration.
    NotAvailable = 0x000E,

    /// Feature not supported.
    NotSupported = 0x000F,

    /// Initialization failed.
    Initialization = 0x0010,

    /// Module has not been initialized.
    NotInitialized = 0x0011,

    /// Module has already been initialized.
    AlreadyInitialized = 0x0012,

    /// Object/construct has been deleted.
    Deleted = 0x0013,

    /// Illegal call from ISR.
    Isr = 0x0014,

    /// Illegal call because network is up.
    NetworkUp = 0x0015,

    /// Illegal call because network is down.
    NetworkDown = 0x0016,

    /// Failure due to not being joined in a network.
    NotJoined = 0x0017,

    /// Invalid operation as there are no beacons.
    NoBeacons = 0x0018,

    ///  Generic allocation error.
    AllocationFailed = 0x0019,

    /// No more resource available to perform the operation.
    NoMoreResource = 0x001A,

    /// Item/list/queue is empty.
    StatusEmpty = 0x001B,

    /// Item/list/queue is full.
    StatusFull = 0x001C,

    /// Item would overflow.
    WouldOverflow = 0x001D,

    /// Item/list/queue has been overflowed.
    HasOverflowed = 0x001E,

    /// Item/list/queue has been overflowed.
    Ownership = 0x001F,

    /// Already/still owning resource.
    IsOwner = 0x0020,

    /// Generic invalid argument or consequence of invalid argument.
    InvalidParameter = 0x0021,

    /// Invalid null pointer received as argument.
    NullPointer = 0x0022,

    /// Invalid configuration provided.
    InvalidConfiguration = 0x0023,

    /// Invalid mode.
    InvalidMode = 0x0024,

    /// Invalid handle.
    InvalidHandle = 0x0025,

    /// Invalid type for operation.
    InvalidType = 0x0026,

    /// Invalid index.
    InvalidIndex = 0x0027,

    /// Invalid range.
    InvalidRange = 0x0028,

    /// Invalid key.
    InvalidKey = 0x0029,

    /// Invalid credentials.
    InvalidCredentials = 0x002A,

    /// Invalid count.
    InvalidCount = 0x002B,

    /// Item could not be found.
    NotFound = 0x002D,

    /// Item already exists.
    AlreadyExists = 0x002E,

    /// Generic I/O failure.
    Io = 0x002F,

    /// I/O failure due to timeout.
    IoTimeout = 0x0030,

    /// Generic transmission error.
    Transmit = 0x0031,

    /// Transmit underflow occurred.
    TransmitUnderflow = 0x0032,

    /// Transmit is incomplete.
    TransmitIncomplete = 0x0033,

    /// Transmit is busy.
    TransmitBusy = 0x0034,

    /// Generic reception error.
    Receive = 0x0035,

    /// Failed to read on/via given object.
    ObjectRead = 0x0036,

    /// Failed to write on/via given object.
    ObjectWrite = 0x0037,

    /// Message is too long.
    MessageTooLong = 0x0038,

    /// Manufacturer version mismatch.
    MfgVersionMismatch = 0x0039,

    /// Stack version mismatch.
    StackVersionMismatch = 0x003A,

    /// Flash write is inhibited.
    WriteInhibited = 0x003B,

    /// Flash verification failed.
    VerifyFailed = 0x003C,

    /// Flash programming failed.
    ProgramFailed = 0x003D,

    /// Flash erase failed.
    EraseFailed = 0x003E,

    /// No data received.
    MacNoData = 0x003F,

    /// No ACK received.
    MacNoAckReceived = 0x0040,

    /// Indirect timeout.
    MacIndirectTimeout = 0x0041,

    /// Unknown header type.
    MacUnknownHeaderType = 0x0042,

    /// ACK header type.
    MacAckHeaderType = 0x0043,

    /// Command transmit failure.
    CommandTransmitFailure = 0x0044,

    /// Error in open NVM.
    StorageNvmOpenError = 0x0045,

    /// Image checksum is not valid.
    ImageChecksumError = 0x0046,

    /// Decryption failed.
    DecryptError = 0x0047,

    /// Command was not recognized.
    IsInvalid = 0x0048,

    /// Command maximum length exceeded.
    TooLong = 0x0049,

    /// Data received does not form a complete command.
    CommandIncomplete = 0x004A,

    /// An invalid scan duration was supplied.
    BadScanDuration = 0x0050,

    /// Invalid firmware key set.
    WifiInvalidKey = 0x0B01,

    /// The firmware download took too long.
    WifiFirmwareDownloadTimeout = 0x0B02,

    /// Unknown request ID or wrong interface ID used.
    WifiUnsupportedMessageId = 0x0B03,

    /// The request is successful but some parameters have been ignored.
    WifiWarning = 0x0B04,

    /// No Packets waiting to be received.
    WifiNoPacketToReceive = 0x0B05,

    /// The sleep mode is granted.
    WifiSleepGranted = 0x0B08,

    /// The `WFx` does not go back to sleep.
    WifiSleepNotGranted = 0x0B09,

    /// The `SecureLink` MAC key was not found.
    WifiSecureLinkMacKeyError = 0x0B10,

    /// The `SecureLink` MAC key is already installed in OTP.
    WifiSecureLinkMacKeyAlreadyBurned = 0x0B11,

    /// The `SecureLink` MAC key cannot be installed in RAM.
    WifiSecureLinkRamModeNotAllowed = 0x0B12,

    /// The `SecureLink` MAC key installation failed.
    WifiSecureLinkFailedUnknownMode = 0x0B13,

    /// `SecureLink` key (re)negotiation failed.
    WifiSecureLinkExchangeFailed = 0x0B14,

    /// The device is in an inappropriate state to perform the request.
    WifiWrongState = 0x0B18,

    /// The request failed due to regulatory limitations.
    WifiChannelNotAllowed = 0x0B19,

    /// The connection request failed because no suitable AP was found.
    WifiNoMatchingAp = 0x0B1A,

    /// The connection request was aborted by host.
    WifiConnectionAborted = 0x0B1B,

    /// The connection request failed because of a timeout.
    WifiConnectionTimeout = 0x0B1C,

    /// The connection request failed because the AP rejected the device.
    WifiConnectionRejectedByAp = 0x0B1D,

    /// The connection request failed because the WPA handshake did not complete successfully.
    WifiConnectionAuthFailure = 0x0B1E,

    /// The request failed because the retry limit was exceeded.
    WifiRetryExceeded = 0x0B1F,

    /// The request failed because the `MSDU` lifetime was exceeded.
    WifiTxLifetimeExceeded = 0x0B20,
}

impl Status {
    /// Return the name of the status.
    #[expect(clippy::too_many_lines)]
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "SL_STATUS_OK",
            Self::Fail => "SL_STATUS_FAIL",
            Self::InvalidState => "SL_STATUS_INVALID_STATE",
            Self::NotReady => "SL_STATUS_NOT_READY",
            Self::Busy => "SL_STATUS_BUSY",
            Self::InProgress => "SL_STATUS_IN_PROGRESS",
            Self::Abort => "SL_STATUS_ABORT",
            Self::Timeout => "SL_STATUS_TIMEOUT",
            Self::Permission => "SL_STATUS_PERMISSION",
            Self::WouldBlock => "SL_STATUS_WOULD_BLOCK",
            Self::Idle => "SL_STATUS_IDLE",
            Self::IsWaiting => "SL_STATUS_IS_WAITING",
            Self::NoneWaiting => "SL_STATUS_NONE_WAITING",
            Self::Suspended => "SL_STATUS_SUSPENDED",
            Self::NotAvailable => "SL_STATUS_NOT_AVAILABLE",
            Self::NotSupported => "SL_STATUS_NOT_SUPPORTED",
            Self::Initialization => "SL_STATUS_INITIALIZATION",
            Self::NotInitialized => "SL_STATUS_NOT_INITIALIZED",
            Self::AlreadyInitialized => "SL_STATUS_ALREADY_INITIALIZED",
            Self::Deleted => "SL_STATUS_DELETED",
            Self::Isr => "SL_STATUS_ISR",
            Self::NetworkUp => "SL_STATUS_NETWORK_UP",
            Self::NetworkDown => "SL_STATUS_NETWORK_DOWN",
            Self::NotJoined => "SL_STATUS_NOT_JOINED",
            Self::NoBeacons => "SL_STATUS_NO_BEACONS",
            Self::AllocationFailed => "SL_STATUS_ALLOCATION_FAILED",
            Self::NoMoreResource => "SL_STATUS_NO_MORE_RESOURCE",
            Self::StatusEmpty => "SL_STATUS_EMPTY",
            Self::StatusFull => "SL_STATUS_FULL",
            Self::WouldOverflow => "SL_STATUS_WOULD_OVERFLOW",
            Self::HasOverflowed => "SL_STATUS_HAS_OVERFLOWED",
            Self::Ownership => "SL_STATUS_OWNERSHIP",
            Self::IsOwner => "SL_STATUS_IS_OWNER",
            Self::InvalidParameter => "SL_STATUS_INVALID_PARAMETER",
            Self::NullPointer => "SL_STATUS_NULL_POINTER",
            Self::InvalidConfiguration => "SL_STATUS_INVALID_CONFIGURATION",
            Self::InvalidMode => "SL_STATUS_INVALID_MODE",
            Self::InvalidHandle => "SL_STATUS_INVALID_HANDLE",
            Self::InvalidType => "SL_STATUS_INVALID_TYPE",
            Self::InvalidIndex => "SL_STATUS_INVALID_INDEX",
            Self::InvalidRange => "SL_STATUS_INVALID_RANGE",
            Self::InvalidKey => "SL_STATUS_INVALID_KEY",
            Self::InvalidCredentials => "SL_STATUS_INVALID_CREDENTIALS",
            Self::InvalidCount => "SL_STATUS_INVALID_COUNT",
            Self::NotFound => "SL_STATUS_NOT_FOUND",
            Self::AlreadyExists => "SL_STATUS_ALREADY_EXISTS",
            Self::Io => "SL_STATUS_IO",
            Self::IoTimeout => "SL_STATUS_IO_TIMEOUT",
            Self::Transmit => "SL_STATUS_TRANSMIT",
            Self::TransmitUnderflow => "SL_STATUS_TRANSMIT_UNDERFLOW",
            Self::TransmitIncomplete => "SL_STATUS_TRANSMIT_INCOMPLETE",
            Self::TransmitBusy => "SL_STATUS_TRANSMIT_BUSY",
            Self::Receive => "SL_STATUS_RECEIVE",
            Self::ObjectRead => "SL_STATUS_OBJECT_READ",
            Self::ObjectWrite => "SL_STATUS_OBJECT_WRITE",
            Self::MessageTooLong => "SL_STATUS_MESSAGE_TOO_LONG",
            Self::MfgVersionMismatch => "SL_STATUS_MFG_VERSION_MISMATCH",
            Self::StackVersionMismatch => "SL_STATUS_STACK_VERSION_MISMATCH",
            Self::WriteInhibited => "SL_STATUS_WRITE_INHIBITED",
            Self::VerifyFailed => "SL_STATUS_VERIFY_FAILED",
            Self::ProgramFailed => "SL_STATUS_PROGRAM_FAILED",
            Self::EraseFailed => "SL_STATUS_ERASE_FAILED",
            Self::MacNoData => "SL_STATUS_MAC_NO_DATA",
            Self::MacNoAckReceived => "SL_STATUS_MAC_NO_ACK_RECEIVED",
            Self::MacIndirectTimeout => "SL_STATUS_MAC_INDIRECT_TIMEOUT",
            Self::MacUnknownHeaderType => "SL_STATUS_MAC_UNKNOWN_HEADER_TYPE",
            Self::MacAckHeaderType => "SL_STATUS_MAC_ACK_HEADER_TYPE",
            Self::CommandTransmitFailure => "SL_STATUS_COMMAND_TRANSMIT_FAILURE",
            Self::StorageNvmOpenError => "SL_STATUS_STORAGE_NVM_OPEN_ERROR",
            Self::ImageChecksumError => "SL_STATUS_IMAGE_CHECKSUM_ERROR",
            Self::DecryptError => "SL_STATUS_DECRYPT_ERROR",
            Self::IsInvalid => "SL_STATUS_IS_INVALID",
            Self::TooLong => "SL_STATUS_TOO_LONG",
            Self::CommandIncomplete => "SL_STATUS_COMMAND_INCOMPLETE",
            Self::BadScanDuration => "SL_STATUS_BAD_SCAN_DURATION",
            Self::WifiInvalidKey => "SL_STATUS_WIFI_INVALID_KEY",
            Self::WifiFirmwareDownloadTimeout => "SL_STATUS_WIFI_FIRMWARE_DOWNLOAD_TIMEOUT",
            Self::WifiUnsupportedMessageId => "SL_STATUS_WIFI_UNSUPPORTED_MESSAGE_ID",
            Self::WifiWarning => "SL_STATUS_WIFI_WARNING",
            Self::WifiNoPacketToReceive => "SL_STATUS_WIFI_NO_PACKET_TO_RECEIVE",
            Self::WifiSleepGranted => "SL_STATUS_WIFI_SLEEP_GRANTED",
            Self::WifiSleepNotGranted => "SL_STATUS_WIFI_SLEEP_NOT_GRANTED",
            Self::WifiSecureLinkMacKeyError => "SL_STATUS_WIFI_SECURE_LINK_MAC_KEY_ERROR",
            Self::WifiSecureLinkMacKeyAlreadyBurned => {
                "SL_STATUS_WIFI_SECURE_LINK_MAC_KEY_ALREADY_BURNED"
            }
            Self::WifiSecureLinkRamModeNotAllowed => {
                "SL_STATUS_WIFI_SECURE_LINK_RAM_MODE_NOT_ALLOWED"
            }
            Self::WifiSecureLinkFailedUnknownMode => {
                "SL_STATUS_WIFI_SECURE_LINK_FAILED_UNKNOWN_MODE"
            }
            Self::WifiSecureLinkExchangeFailed => "SL_STATUS_WIFI_SECURE_LINK_EXCHANGE_FAILED",
            Self::WifiWrongState => "SL_STATUS_WIFI_WRONG_STATE",
            Self::WifiChannelNotAllowed => "SL_STATUS_WIFI_CHANNEL_NOT_ALLOWED",
            Self::WifiNoMatchingAp => "SL_STATUS_WIFI_NO_MATCHING_AP",
            Self::WifiConnectionAborted => "SL_STATUS_WIFI_CONNECTION_ABORTED",
            Self::WifiConnectionTimeout => "SL_STATUS_WIFI_CONNECTION_TIMEOUT",
            Self::WifiConnectionRejectedByAp => "SL_STATUS_WIFI_CONNECTION_REJECTED_BY_AP",
            Self::WifiConnectionAuthFailure => "SL_STATUS_WIFI_CONNECTION_AUTH_FAILURE",
            Self::WifiRetryExceeded => "SL_STATUS_WIFI_RETRY_EXCEEDED",
            Self::WifiTxLifetimeExceeded => "SL_STATUS_WIFI_TX_LIFETIME_EXCEEDED",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl LowerHex for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#010x}", *self as u32)
    }
}

impl UpperHex for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#010X}", *self as u32)
    }
}
