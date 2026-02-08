//! Storage implementation.

use core::ops::Range;

use ariel_os_debug::log::{debug, warn};

use esp_bootloader_esp_idf::partitions::{
    DataPartitionSubType, PARTITION_TABLE_MAX_LEN, PartitionType, read_partition_table,
};
use esp_hal::peripherals::OptionalPeripherals;
use esp_storage::FlashStorage;

use embassy_embedded_hal::adapter::BlockingAsync;

pub type Flash = BlockingAsync<FlashStorage<'static>>;
pub type FlashError = esp_storage::FlashStorageError;

/// Gets a [`Range`] from the partition table.
///
/// This expects the partition table to contain a partition of type `data` and subtype `undefined`.
/// See <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/partition-tables.html#subtype>
pub fn flash_range_from_partition_table(p: &mut OptionalPeripherals) -> Range<u32> {
    let mut flash = FlashStorage::new(p.FLASH.as_mut().map(|f| f.reborrow()).unwrap());

    let mut buffer = [0u8; PARTITION_TABLE_MAX_LEN];
    let Ok(partition_table) = read_partition_table(&mut flash, &mut buffer) else {
        warn!("Failed to read partition table");
        return 0..0;
    };

    partition_table
        .find_partition(PartitionType::Data(DataPartitionSubType::Undefined))
        .ok()
        .flatten()
        .map_or_else(
            || {
                warn!("No data partition was found");
                0..0
            },
            |part| {
                debug!(
                    "Found data partition of length {} at offset {:x}",
                    part.len(),
                    part.offset()
                );
                part.offset()..part.offset() + part.len()
            },
        )
}

pub fn init(p: &mut crate::OptionalPeripherals) -> Flash {
    let flash = FlashStorage::new(p.FLASH.take().unwrap());
    BlockingAsync::new(flash)
}
