use embassy_embedded_hal::adapter::BlockingAsync;
use esp_storage::FlashStorage;

pub type Flash = BlockingAsync<FlashStorage>;
pub type FlashError = esp_storage::FlashStorageError;

pub fn init(_peripherals: &mut crate::OptionalPeripherals) -> Flash {
    let flash = FlashStorage::new();
    BlockingAsync::new(flash)
}