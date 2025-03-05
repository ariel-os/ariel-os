//! Error types for storage
use arrayvec::CapacityError;
use embedded_storage_async::nor_flash::NorFlashError;
use sequential_storage::Error as StorageError;

/// Storage-related errors.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error<S> {
    /// Error from sequential-storage.
    SequentialStorage {
        /// Exact sequential-storage error.
        error: StorageError<S>,
    },

    /// Provided key is too largo to serialize into storage.
    KeyTooLarge,
}

impl<S: NorFlashError> From<StorageError<S>> for Error<S> {
    fn from(err: StorageError<S>) -> Self {
        Self::SequentialStorage { error: err }
    }
}

impl<S: NorFlashError> From<CapacityError<&str>> for Error<S> {
    fn from(_err: CapacityError<&str>) -> Self {
        Self::KeyTooLarge
    }
}

impl<S> core::fmt::Display for Error<S>
where
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SequentialStorage { error } => write!(f, "sequential-storage error: {error}"),
            Self::KeyTooLarge => write!(f, "key too large"),
        }
    }
}
