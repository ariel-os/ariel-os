#![cfg_attr(not(test), no_std)]
// #![deny(missing_docs)]

mod rt;
mod storage;

use embedded_storage_async::nor_flash::{NorFlash, ReadNorFlash};
use portable_atomic::{AtomicUsize, Ordering};

pub use rt::{StorageBackend, UpdateSlot};

// TODO: Size this depending on the device partitions. Currently this is only a reasonable maximum
// value.
const UPDATE_SLOT_COUNT: usize = 3;
// Counts the bytes written in each update slot.
static BYTES_WRITTEN: [AtomicUsize; UPDATE_SLOT_COUNT] =
    [const { AtomicUsize::new(0) }; UPDATE_SLOT_COUNT];

/// Reads the given update slot, starting from the offset (within the slot),
/// for the given length.
pub async fn read(update_slot: UpdateSlot<'_>, offset: u32, buf: &mut [u8]) -> Result<(), Error> {
    let address = update_slot_to_address(update_slot, offset)?
        .try_into()
        .map_err(|_| Error::Storage)?;

    let flash = &mut storage::STORAGE.get().await.lock().await.flash;

    flash.read(address, buf).await.map_err(|_| Error::Storage)
}

/// Writes the given payload to an update slot, at the given offset
/// within that slot.
// NOTE: `bytes` is a subslice of a SUIT payload.
pub async fn write(bytes: &[u8], update_slot: UpdateSlot<'_>, offset: u32) -> Result<(), Error> {
    let len = u32::try_from(bytes.len()).unwrap();

    if capacity(update_slot)? < offset + len {
        return Err(Error::InvalidByteSliceLength);
    }

    let address = update_slot_to_address(update_slot, offset)?
        .try_into()
        .map_err(|_| Error::Storage)?;

    let flash = &mut storage::STORAGE.get().await.lock().await.flash;

    flash
        .erase(address, address + len)
        .await
        .map_err(|_| Error::Storage)?;
    flash
        .write(address, bytes)
        .await
        .map_err(|_| Error::Storage)?;

    let update_slot_index: usize = update_slot.index()?;
    // When the write operation succeeds, all bytes have been written.
    let bytes_written = bytes.len();

    // TODO(ordering): ordering can likely be weaker as this value only gets *incremented*.
    BYTES_WRITTEN[update_slot_index].add(bytes_written, Ordering::SeqCst);

    Ok(())
}

/// Returns the length of the payload written since the last reboot or the last update.
pub fn len(update_slot: UpdateSlot<'_>) -> Result<u32, Error> {
    let update_slot_index: usize = update_slot.index()?;

    Ok(BYTES_WRITTEN[update_slot_index].load(Ordering::Acquire))
}

/// Returns the capacity of the given update slot.
pub fn capacity(update_slot: UpdateSlot<'_>) -> Result<u32, Error> {
     let range = update_slot.range().map_err(|source| match source {
         rt::Error::InvalidSlot => Error::InvalidSlot,
     })?;
    Ok(range.len())
}

fn update_slot_to_address(update_slot: UpdateSlot<'_>, offset: u32) -> Result<u32, Error> {
    let range = update_slot.range()?;

    if offset >= range.end {
        return Err(Error::OffsetOutOfBounds);
    }

    let address = range
        .start
        .checked_add(offset)
        .ok_or(Error::OffsetOutOfBounds)?;

    Ok(address)
}

#[derive(Debug)]
pub enum Error {
    /// Write failed because of a storage error.
    Storage,
    /// Invalid update slot (includes invalid backend).
    InvalidSlot,
    /// The offset is larger than the slot's size.
    OffsetOutOfBounds,
    /// The byte slice given is too large for the remaining space in the slot (starting at the offset).
    InvalidByteSliceLength,
}

impl From<rt::Error> for Error {
    fn from(err: rt::Error) -> Self {
        match err {
            rt::Error::InvalidSlot => Error::InvalidSlot,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}

impl core::error::Error for Error {}
