// Should likely be be part of `ariel-os-rt`.

use core::ops::Range;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StorageBackend {
    Nvm,
    // Ram,
    // Filesystem,
}

// NOTE: not to be confused with a SUIT Slot (which refers to a range within a SUIT Component).
#[derive(Debug, Copy, Clone)]
pub struct UpdateSlot<'id> {
    storage_backend: StorageBackend,
    // Opaque.
    // NOTE: This can be constructed from a SUIT Component Identifier and a SUIT Slot by concatenating them.
    id: &'id [u8],
}

impl<'id> UpdateSlot<'id> {
    #[must_use]
    pub fn new(storage_backend: StorageBackend, id: &'id [u8]) -> Self {
        Self {
            storage_backend,
            id,
        }
    }

    pub fn storage_backend(&self) -> StorageBackend {
        self.storage_backend
    }

    pub fn id(&self) -> &'id [u8] {
        self.id
    }
}

impl UpdateSlot<'_> {
    pub fn range(&self) -> Result<Range<u32>, Error> {
        // TODO: change/document the Component Identifiers.
        match self.id {
            // SUIT Component Identifier `[h00]`, SUIT Slot 0.
            [0x00, 0x00] => {
                // ACTIVE.
                Ok(ariel_os_rt::memory::sections::ACTIVE)
            }
            // SUIT Component Identifier `[h00]`, SUIT Slot 1.
            [0x00, 0x01] => {
                // DFU.
                Ok(ariel_os_rt::memory::sections::DFU)
            }
            _ => Err(Error::InvalidSlot),
        }
    }

    // Used to index an array of bytes written counters.
    pub fn index(&self) -> Result<usize, Error> {
        match self.id {
            // SUIT Component Identifier `[h00]`, SUIT Slot 0.
            [0x00, 0x00] => {
                // ACTIVE.
                Ok(0)
            }
            // SUIT Component Identifier `[h00]`, SUIT Slot 1.
            [0x00, 0x01] => {
                // DFU.
                Ok(1)
            }
            _ => Err(Error::InvalidSlot),
        }
    }
}

// TODO: should not be needed when actually reading addresses from symbols.
fn range_from_start_len<T: core::ops::Add<Output = T> + Copy>(start: T, len: T) -> Range<T> {
    Range {
        start,
        end: start + len,
    }
}

#[derive(Debug)]
pub enum Error {
    /// Invalid update slot (includes invalid backend).
    InvalidSlot,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}

impl core::error::Error for Error {}
