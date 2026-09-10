//! esp partition support.

//! Storage implementation.

use core::ops::Range;

use ariel_os_log::debug;

use esp_bootloader_esp_idf::partitions::{
    read_partition_table, DataPartitionSubType, PartitionType, PARTITION_TABLE_MAX_LEN,
};
use esp_hal::peripherals::OptionalPeripherals;
use esp_storage::FlashStorage;

/// Gets a [`Range`] from the partition table, based on the supplied partition_type.
/// Returns [`None`] when no matching partition is found.
///
/// # panics
///
/// Panics when the partition table cannot be read from flash
pub fn flash_range_from_partition_table(
    p: &mut OptionalPeripherals,
    partition_type: PartitionType,
) -> Option<Range<u32>> {
    let mut flash = FlashStorage::new(p.FLASH.as_mut().map(|f| f.reborrow()).unwrap());

    let mut buffer = [0u8; PARTITION_TABLE_MAX_LEN];
    let partition_table =
        read_partition_table(&mut flash, &mut buffer).expect("Failed to read partition table");

    partition_table
        .find_partition(partition_type)
        .ok()
        .flatten()
        .map(|part| {
            debug!(
                "Found data partition of length {} at offset {:x}",
                part.len(),
                part.offset()
            );
            part.offset()..part.offset() + part.len()
        })
}

/// Gets a [`Range`] from the partition table for storage.
///
/// This expects the partition table to contain a partition of type `data` and subtype `undefined`.
/// See <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-guides/partition-tables.html#subtype>
///
/// # Panics
///
/// Panics when the partition table cannot be read or when no storage partition is availble in
/// flash.
pub fn storage_partition(p: &mut OptionalPeripherals) -> Range<u32> {
    flash_range_from_partition_table(p, PartitionType::Data(DataPartitionSubType::Undefined))
        .unwrap()
}
