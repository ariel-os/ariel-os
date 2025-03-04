//! Storage module wrapping [`sequential_storage`] in an object together with
//! a flash range and backend.
use core::ops::Range;

use embedded_storage_async::nor_flash::{ErrorType, MultiwriteNorFlash, NorFlash};
use sequential_storage::{
    cache::NoCache,
    erase_all,
    map::{fetch_item, remove_item, store_item, Value},
};

pub use crate::postcard_value::PostcardValue;
pub use serde::{Deserialize, Serialize};

/// Maximum key length.
pub const MAX_KEY_LEN: usize = 64usize;
/// Data buffer length.
pub const DATA_BUFFER_SIZE: usize = 128usize;

type InternalKey = CborKey;

/// Workhorse trait for [`Key`][super::Key].
///
/// This gives control over how a type is serialized into a [`sequential_storage`].
pub trait SealedKey {
    /// Converts the key into its serialized format.
    ///
    /// Initially, only string keys are supported, and converted as-is.
    fn key(&self) -> InternalKey;
}

/// Object holding an instance of a key-value pair storage.
///
/// You should probably look into using the global instance accessible via
/// `ariel_os_storage::storage::{get,insert,remove}`.
pub struct Storage<F> {
    flash: F,
    storage_range: Range<u32>,
}

impl<F: NorFlash> Storage<F> {
    /// Creates a new [`Storage`] instance.
    pub const fn new(flash: F, storage_range: Range<u32>) -> Storage<F> {
        Self {
            flash,
            storage_range,
        }
    }

    /// Gets a [`Value`] from this [`Storage`] instance.
    ///
    /// # Panics
    ///
    /// Currently panics if `key.len() > MAX_KEY_LEN`.
    pub async fn get_raw<V: for<'d> Value<'d>>(
        &mut self,
        key: impl super::Key,
    ) -> Result<Option<V>, sequential_storage::Error<<F as ErrorType>::Error>> {
        let key = key.key();
        let mut data_buffer = [0; DATA_BUFFER_SIZE];

        fetch_item::<_, V, _>(
            &mut self.flash,
            self.storage_range.clone(),
            &mut NoCache::new(),
            &mut data_buffer,
            &key,
        )
        .await
    }

    /// Inserts a [`Value`] into this [`Storage`] instance.
    ///
    /// # Panics
    ///
    /// Currently panics if `key.len() > MAX_KEY_LEN`.
    pub async fn insert_raw<'d, V: Value<'d>>(
        &mut self,
        key: impl super::Key,
        value: V,
    ) -> Result<(), sequential_storage::Error<<F as ErrorType>::Error>> {
        let key = key.key();
        let mut data_buffer = [0; DATA_BUFFER_SIZE];
        store_item(
            &mut self.flash,
            self.storage_range.clone(),
            &mut NoCache::new(),
            &mut data_buffer,
            &key,
            &value,
        )
        .await
    }

    /// Stores a key-value pair into flash memory.
    ///
    /// It will overwrite the last value that has the same key.
    pub async fn insert<'d, V>(
        &mut self,
        key: impl super::Key,
        value: V,
    ) -> Result<(), sequential_storage::Error<<F as ErrorType>::Error>>
    where
        V: Serialize + Deserialize<'d> + Into<PostcardValue<V>>,
    {
        self.insert_raw(key, value.into()).await
    }

    /// Gets the last stored value from the flash that is associated with the given key.
    ///
    /// If no value with the key is found, `None` is returned.
    ///
    /// # Panics
    ///
    /// Currently panics if `key.len() > MAX_KEY_LEN`.
    pub async fn get<V>(
        &mut self,
        key: impl super::Key,
    ) -> Result<Option<V>, sequential_storage::Error<<F as ErrorType>::Error>>
    where
        V: Serialize + for<'d> Deserialize<'d> + Into<PostcardValue<V>>,
    {
        let key = key.key();
        let mut data_buffer = [0; DATA_BUFFER_SIZE];

        let postcard_value = fetch_item::<_, PostcardValue<V>, _>(
            &mut self.flash,
            self.storage_range.clone(),
            &mut NoCache::new(),
            &mut data_buffer,
            &key,
        )
        .await?;
        Ok(postcard_value.map(PostcardValue::into_inner))
    }

    /// Resets the flash in the entire flash range of this [`Storage`] instance.
    pub async fn erase_all(
        &mut self,
    ) -> Result<(), sequential_storage::Error<<F as ErrorType>::Error>> {
        erase_all(&mut self.flash, self.storage_range.clone()).await
    }
}

impl<F: MultiwriteNorFlash> Storage<F> {
    /// Deletes an item from flash.
    ///
    /// Additional calls to [`Storage::get()`] with the same key will return `None` until
    /// a new one is stored again.
    ///
    /// <div class="warning">
    /// This is really slow!
    ///
    /// All items in flash have to be read and deserialized to find the items with the key.
    /// This is unlikely to be cached well.
    /// </div>
    ///
    /// # Panics
    ///
    /// Currently panics if `key.len() > MAX_KEY_LEN`.
    pub async fn remove(
        &mut self,
        key: impl super::Key,
    ) -> Result<(), sequential_storage::Error<<F as ErrorType>::Error>> {
        let key = key.key();
        let mut data_buffer = [0; DATA_BUFFER_SIZE];
        remove_item(
            &mut self.flash,
            self.storage_range.clone(),
            &mut NoCache::new(),
            &mut data_buffer,
            &key,
        )
        .await
    }
}

impl super::Key for &str {}
impl SealedKey for &str {
    fn key(&self) -> CborKey {
        let mut vec = heapless::Vec::new();
        let mut encoder =
            minicbor::encode::Encoder::new(minicbor_adapters::WriteToHeapless(&mut vec));
        encoder.encode(self).unwrap();
        CborKey(vec)
    }
}

impl super::Key for (&str, usize) {}
impl SealedKey for (&str, usize) {
    fn key(&self) -> CborKey {
        // At some point we could start impl'ing it for anything that is minicbor encodable, or
        // stay selective (and find a good way to express that they all go through minicbor
        // anyway without duplicating code verbatim).
        let mut vec = heapless::Vec::new();
        let mut encoder =
            minicbor::encode::Encoder::new(minicbor_adapters::WriteToHeapless(&mut vec));
        encoder.encode(self).unwrap();
        CborKey(vec)
    }
}

/// An owned buffer for key storage.
///
/// It is a panic-worthy invariant of this type that the data in the inner vector are CBOR encoded
/// (which is what determines the length).
#[derive(Clone, PartialEq, Eq)]
pub struct CborKey(heapless::Vec<u8, MAX_KEY_LEN>);

impl sequential_storage::map::Key for CborKey {
    fn serialize_into(
        &self,
        buffer: &mut [u8],
    ) -> Result<usize, sequential_storage::map::SerializationError> {
        buffer
            .get_mut(..self.0.len())
            .ok_or(sequential_storage::map::SerializationError::BufferTooSmall)?
            .copy_from_slice(self.0.as_ref());
        Ok(self.0.len())
    }

    fn deserialize_from(
        buffer: &[u8],
    ) -> Result<(Self, usize), sequential_storage::map::SerializationError> {
        // As long as we know that it's just a single byte string, we could determine the length
        // more cheaply by not covering all variants -- but in more complex builds, this is
        // something we'll already have at hand.
        let mut decoder = minicbor::decode::Decoder::new(buffer);
        decoder
            .skip()
            .map_err(|_| sequential_storage::map::SerializationError::InvalidData)?;
        let length = decoder.position();

        Ok((
            CborKey(
                #[allow(
                    clippy::indexing_slicing,
                    reason = "length came from successful decoding inside the buffer"
                )]
                #[allow(
                    clippy::ignored_unit_patterns,
                    reason = "unit error is not a recommended pattern, fixing it is a needless compatibility hazard when future heapless versions follow clippy recommendations"
                )]
                heapless::Vec::try_from(&buffer[..length])
                    .map_err(|_| sequential_storage::map::SerializationError::BufferTooSmall)?,
            ),
            length,
        ))
    }
}
