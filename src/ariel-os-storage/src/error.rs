//! Error types for storage
use arrayvec::CapacityError;
use embedded_storage_async::nor_flash::{NorFlashError, NorFlashErrorKind};
use sequential_storage::{map::SerializationError, Error as StorageError};

/// Storage-related errors.
///
/// These errors can have multiple causes:
///  - [`Error::FlashNotAligned`], [`Error::FlashOutOfBounds`] and [`Error::FlashOther`] cover low
///    level flash operation errors and can be caused by an incorrect
///    [`ariel_os_hal::storage::Flash`] configuration
///  - [`Error::SerializationBufferTooSmall`] is caused by attempting to store a too large item.
///    Either decrease the item size or increase `DATA_BUFFER_SIZE`.
///  - [`Error::KeyTooLarge`] is caused by a too long key string.
///    Either decrease the key length or increase `MAX_KEY_LENGTH`.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Flash operation arguments are not properly aligned.
    FlashNotAligned,
    /// Flash operation arguments are out of bounds.
    FlashOutOfBounds,
    /// Other flash implementation specific error.
    FlashOther,
    /// The item cannot be stored anymore because the storage is too full.
    FullStorage,
    /// Storage corruption has been detected.
    Corrupted,
    /// A provided buffer was too big to be used.
    BufferTooBig,
    /// A provided buffer was too small to be used.
    BufferTooSmall(usize),
    /// The item does not fit in the flash, ever.
    ItemTooBig,
    /// The data buffer length is too small to serialize the item.
    SerializationBufferTooSmall,
    /// The item serialization cannot succeed due to invalid data.
    SerializationInvalidData,
    /// The deserialization cannot succeed because the bytes are in an invalid format.
    SerializationInvalidFormat,
    /// Provided key is too large to serialize into storage.
    KeyTooLarge,
    /// Other storage error.
    Other,
}

impl core::error::Error for Error {}

impl<S: NorFlashError> From<StorageError<S>> for Error {
    fn from(err: StorageError<S>) -> Self {
        match err {
            StorageError::Storage { value } => match value.kind() {
                NorFlashErrorKind::NotAligned => Error::FlashNotAligned,
                NorFlashErrorKind::OutOfBounds => Error::FlashOutOfBounds,
                _ => Error::FlashOther,
            },
            StorageError::ItemTooBig => Error::ItemTooBig,
            StorageError::FullStorage => Error::FullStorage,
            StorageError::Corrupted {} => Error::Corrupted,
            StorageError::BufferTooBig => Error::BufferTooBig,
            StorageError::BufferTooSmall(value) => Error::BufferTooSmall(value),
            StorageError::SerializationError(value) => match value {
                SerializationError::BufferTooSmall => Error::SerializationBufferTooSmall,
                SerializationError::InvalidData => Error::SerializationInvalidData,
                SerializationError::InvalidFormat => Error::SerializationInvalidFormat,
                _ => Error::Other,
            },
            _ => Error::Other,
        }
    }
}

impl From<CapacityError<&str>> for Error {
    fn from(_err: CapacityError<&str>) -> Self {
        Self::KeyTooLarge
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::KeyTooLarge => write!(f, "storage key is too large"),
            Self::FullStorage => write!(f, "storage is full"),
            Self::Corrupted => write!(f, "storage is corrupted"),
            Self::BufferTooBig => write!(f, "a provided buffer was to big to be used"),
            Self::BufferTooSmall(needed) => write!(
                f,
                "a provided buffer was to small to be used. Needed was {needed}"
            ),
            Self::ItemTooBig => write!(f, "the item is too big to fit in the flash"),
            Self::Other => write!(f, "storage implementation specific error occurred"),

            Self::FlashNotAligned => write!(f, "flash arguments are not properly aligned"),
            Self::FlashOutOfBounds => write!(f, "flash arguments are out of bounds"),
            Self::FlashOther => write!(f, "flash implementation specific error occurred"),

            Self::SerializationBufferTooSmall => write!(f, "Buffer too small"),
            Self::SerializationInvalidData => write!(f, "Invalid data"),
            Self::SerializationInvalidFormat => write!(f, "Invalid format"),
        }
    }
}
