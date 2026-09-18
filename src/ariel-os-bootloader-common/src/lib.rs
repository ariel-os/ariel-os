#![no_main]
#![no_std]

use ariel_os_device_update::{DeviceUpdater, DeviceUpdaterError, DeviceUpdaterState};
use ariel_os_embassy_common::bootloader::{BootLoaderBackend, FlashConfig};
use ariel_os_hal::hal::bootloader::HalBootLoaderBackend;
use ariel_os_rt::memory::sections;
use embassy_boot::{AlignedBuffer, BlockingFirmwareUpdater};
use embedded_storage::nor_flash::NorFlash;

pub type HalDeviceUpdater<'a> = EmbassyBootDeviceUpdater<'a>;
pub type HalDeviceUpdaterState = EmbassyBootDeviceUpdaterState;

type DfuFlash = <HalBootLoaderBackend as BootLoaderBackend>::DFU;
type StateFlash = <HalBootLoaderBackend as BootLoaderBackend>::STATE;

const WRITE_SIZE: usize = <StateFlash as NorFlash>::WRITE_SIZE;

pub const fn flash_config() -> FlashConfig {
    FlashConfig {
        active: sections::ACTIVE,
        dfu: sections::DFU,
        bootloader_state: sections::BOOTLOADER_STATE,
    }
}

pub struct EmbassyBootDeviceUpdaterState {
    aligned: AlignedBuffer<WRITE_SIZE>,
}

impl EmbassyBootDeviceUpdaterState {
    pub fn new() -> Self {
        Self {
            aligned: AlignedBuffer { 0: [0; WRITE_SIZE] },
        }
    }
}

impl DeviceUpdaterState for EmbassyBootDeviceUpdaterState {
    fn updater(&mut self) -> impl DeviceUpdater {
        let flash_config = flash_config();

        let config = HalBootLoaderBackend::config_firmware_updater(&flash_config);

        let aligned = &mut self.aligned.0[..WRITE_SIZE];

        let updater = BlockingFirmwareUpdater::new(config, aligned);

        EmbassyBootDeviceUpdater { updater }
    }
}

pub struct EmbassyBootDeviceUpdater<'a> {
    updater: BlockingFirmwareUpdater<'a, DfuFlash, StateFlash>,
}

impl<'a> DeviceUpdater for EmbassyBootDeviceUpdater<'a> {
    const CHUNK_SIZE: usize = 4096;
    async fn write_firmware(
        &mut self,
        offset: usize,
        data: &[u8],
    ) -> Result<(), DeviceUpdaterError> {
        Ok(self.updater.write_firmware(offset, data).unwrap())
    }

    async fn mark_updated(&mut self) -> Result<(), DeviceUpdaterError> {
        Ok(self.updater.mark_updated().unwrap())
    }
}
