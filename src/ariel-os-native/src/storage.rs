use std::io::{Read as _, Seek as _, Write as _};

use embedded_storage_async::nor_flash::{
    ErrorType, MultiwriteNorFlash, NorFlash, NorFlashErrorKind, ReadNorFlash,
};

use ariel_os_debug::log::{debug, trace};

pub type Flash = FileFlash;
pub type FlashError = FileFlashError;

/// Memory size with which a file will be created if none is present.
///
/// There is currently no means to override this, but if the file exists, its value is used.
const DEFAULT_LENGTH: u64 = 16 * 1024;

/// Write granularity advertised for the flash.
///
/// There is currently no means to override this.
const DEFAULT_WRITE_SIZE: usize = 1;

/// Erase granularity advertised for the flash.
///
/// There is currently no means to override this.
///
/// While the embedded storage API would allow this to be minimal (1). a large value is chosen
/// because the current implementation of storage requires some minimal size for practical writing
/// of data items.
const DEFAULT_ERASE_SIZE: usize = 256;

/// The value which flash takes when it is erased.
///
/// Currently, the embedded storage API implies that this has all bits set.
const ERASE_VALUE: u8 = 0xff;

/// Error type for [`FileFlash`].
#[derive(Debug)]
#[non_exhaustive]
pub enum FileFlashError {
    /// The requested data is out of bounds of the emulated flash storage.
    OutOfBounds,
    // Potential future errors are:
    // - file system errors (when the 'no monkey business' rule goes away)
    // - simulated errors to test application robustness against flash troubles
}

impl embedded_storage_async::nor_flash::NorFlashError for FileFlashError {
    fn kind(&self) -> NorFlashErrorKind {
        match self {
            FlashError::OutOfBounds => NorFlashErrorKind::OutOfBounds,
        }
    }
}

/// An emulated [`MultiwriteNorFlash`] backed by a file.
///
/// All the content of the file is presented 1:1 as flash content without any offset.
///
/// Various functions and method implementations in this would panic if there is any monkey
/// business with the file (deletion or modification by other processes at runtime).
pub struct FileFlash {
    file: std::fs::File,
    buffer: Box<[u8]>,
}

impl FileFlash {
    /// Returns the length of the storage.
    pub fn size(&mut self) -> usize {
        self.buffer.len()
    }

    /// Saves the buffer to the file.
    ///
    /// # Panics
    ///
    /// Panics if file writing fails.
    ///
    /// FIXME: Should this flush?
    pub fn save(&mut self) {
        self.file.rewind().unwrap();
        self.file.write_all(&self.buffer).unwrap();
        trace!("Changes to storage have been saved to file.");
    }

    /// Helper converting flash API's conventons into `.get(start..end)` implementation.
    ///
    /// # Errors
    ///
    /// … are emitted if the implied range is not completely within the underlying data.
    fn get_mut_from_length(
        &mut self,
        start: impl TryInto<usize>,
        length: impl TryInto<usize>,
    ) -> Result<&mut [u8], FlashError> {
        self.buffer
            .get_mut(start.try_into().map_err(|_| FileFlashError::OutOfBounds)?..)
            .ok_or(FileFlashError::OutOfBounds)?
            .get_mut(..length.try_into().map_err(|_| FileFlashError::OutOfBounds)?)
            .ok_or(FileFlashError::OutOfBounds)
    }
}

impl ErrorType for FileFlash {
    type Error = FlashError;
}

impl ReadNorFlash for FileFlash {
    const READ_SIZE: usize = 1;
    async fn read(&mut self, offset: u32, buffer: &mut [u8]) -> Result<(), FlashError> {
        buffer.copy_from_slice(self.get_mut_from_length(offset, buffer.len())?);
        Ok(())
    }
    fn capacity(&self) -> usize {
        self.buffer.len()
    }
}

impl NorFlash for FileFlash {
    const WRITE_SIZE: usize = DEFAULT_WRITE_SIZE;
    const ERASE_SIZE: usize = DEFAULT_ERASE_SIZE;
    async fn erase(
        &mut self,
        from: u32,
        to: u32,
    ) -> Result<(), <Self as embedded_storage_async::nor_flash::ErrorType>::Error> {
        self.get_mut_from_length(from, to)?
            .iter_mut()
            .for_each(|x| *x = ERASE_VALUE);
        self.save();
        Ok(())
    }
    async fn write(
        &mut self,
        offset: u32,
        bytes: &[u8],
    ) -> Result<(), <Self as embedded_storage_async::nor_flash::ErrorType>::Error> {
        self.get_mut_from_length(offset, bytes.len())?
            .iter_mut()
            .zip(bytes.iter())
            .for_each(|(storage, input)| *storage &= *input);
        self.save();
        Ok(())
    }
}

impl MultiwriteNorFlash for FileFlash {}

/// Initializes a flash instance from the environment variables.
///
/// (See the book for its detailed documentation).
///
/// # Panics
///
/// … if the file can not be loaded.
pub fn init(_peripherals: &mut crate::OptionalPeripherals) -> Flash {
    let filename =
        std::env::var("ARIEL_NATIVE_FLASH_FILE").unwrap_or_else(|_| "flash.bin".to_owned());
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&filename)
        .unwrap();
    if file.stream_position().unwrap() == 0 {
        debug!(
            "Storage backend file {} found absent or empty; initializing at default length of {}",
            filename, DEFAULT_LENGTH
        );
        file.set_len(DEFAULT_LENGTH).unwrap();
        // No need to rewind: set_len does not change the cursor.
    }
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    debug!(
        "Read {} byte from {} as initial content of storage.",
        buffer.len(),
        filename
    );
    FileFlash {
        file,
        buffer: buffer.into(),
    }
}
