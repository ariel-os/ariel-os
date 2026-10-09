use crate::OptionalPeripherals;
use ariel_os_device_update::{DeviceUpdater, DeviceUpdaterError, DeviceUpdaterState};
use core::cell::RefCell;
use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use esp_bootloader_esp_idf::{
    ota::OtaImageState,
    ota_updater::{self, OtaUpdater},
    partitions::{FlashRegion, PARTITION_TABLE_MAX_LEN},
};
use esp_storage::{FlashStorage, FlashStorageError};
use static_cell::{ConstStaticCell, StaticCell};

pub type HalDeviceUpdater<'a> = EspDeviceUpdater<'a>;
pub type HalDeviceUpdaterState = EspDeviceUpdaterState;

const WRITE_SIZE: usize = <FlashStorage<'_> as NorFlash>::WRITE_SIZE;

static FLASH_STORAGE: StaticCell<FlashStorage<'static>> = StaticCell::new();
static FLASH_STORAGE_REF: Mutex<
    CriticalSectionRawMutex,
    RefCell<Option<&'static mut FlashStorage<'static>>>,
> = Mutex::new(RefCell::new(None));

// Initializes the NVMC needed for operating on the flash.
pub fn init(peripherals: &mut OptionalPeripherals) {
    let flash = peripherals.FLASH.take().unwrap();

    let storage_ref = FLASH_STORAGE.init(FlashStorage::new(flash));
    let _ = FLASH_STORAGE_REF.lock(|m| m.replace(Some(storage_ref)));
}

pub struct EspDeviceUpdaterState {
    aligned: [u8; PARTITION_TABLE_MAX_LEN],
}

impl EspDeviceUpdaterState {
    pub fn new() -> Self {
        Self {
            aligned: [0; PARTITION_TABLE_MAX_LEN],
        }
    }
}

impl DeviceUpdaterState for EspDeviceUpdaterState {
    fn updater(&mut self) -> impl DeviceUpdater {
        let flash_storage = FLASH_STORAGE_REF.lock(|a| a.take()).unwrap();

        let updater = OtaUpdater::new(flash_storage, &mut self.aligned).unwrap();

        EspDeviceUpdater { updater }
    }
}

pub struct EspDeviceUpdater<'a> {
    updater: OtaUpdater<'a, FlashStorage<'static>>,
}

impl<'a> DeviceUpdater for EspDeviceUpdater<'a> {
    type InnerError = esp_bootloader_esp_idf::partitions::Error;

    // Sometimes the minimum write size can be a bit small.
    const CHUNK_SIZE: usize = WRITE_SIZE * 4;

    async fn mark_booted(&mut self) -> Result<(), DeviceUpdaterError<Self::InnerError>> {
        self.updater
            .set_current_ota_state(OtaImageState::Valid)
            .map_err(DeviceUpdaterError::Inner)
    }

    async fn mark_updated(&mut self) -> Result<(), DeviceUpdaterError<Self::InnerError>> {
        // TODO: change error type to accomodate the errors from those functions ?
        self.updater
            .activate_next_partition()
            .map_err(DeviceUpdaterError::Inner)?;
        self.updater
            .set_current_ota_state(OtaImageState::New)
            .map_err(DeviceUpdaterError::Inner)
    }

    async fn read_firmware(
        &mut self,
        offset: u32,
        buffer: &mut [u8],
    ) -> Result<(), DeviceUpdaterError<Self::InnerError>> {
        let (mut partition, _t) = self
            .updater
            .next_partition()
            .map_err(DeviceUpdaterError::Inner)?;
        partition
            .read(offset, buffer)
            .map_err(DeviceUpdaterError::Inner)
    }
    async fn write_firmware(
        &mut self,
        offset: u32,
        data: &[u8],
    ) -> Result<(), DeviceUpdaterError<Self::InnerError>> {
        // Check if we correctly booted before starting an update.
        if self
            .updater
            .current_ota_state()
            .map_err(DeviceUpdaterError::Inner)?
            != OtaImageState::Valid
        {
            return Err(DeviceUpdaterError::BadState);
        }

        let (mut partition, _t) = self
            .updater
            .next_partition()
            .map_err(DeviceUpdaterError::Inner)?;
        partition
            .write(offset, data)
            .map_err(DeviceUpdaterError::Inner)
    }
}
