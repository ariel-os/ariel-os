use embassy_embedded_hal::adapter::BlockingAsync;

use esp_storage::FlashStorage;

pub type Flash = BlockingAsync<FlashStorage<'static>>;
pub type FlashError = esp_storage::FlashStorageError;

pub fn init(p: &mut crate::OptionalPeripherals) -> Flash {
    let flash = FlashStorage::new(p.FLASH.take().unwrap());
    BlockingAsync::new(flash)
}
