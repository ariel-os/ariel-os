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
    type InnerError = embassy_boot::FirmwareUpdaterError;

    // Sometimes the minimum write size can be a bit small.
    const CHUNK_SIZE: usize = WRITE_SIZE * 4;

    async fn write_firmware(
        &mut self,
        offset: u32,
        data: &[u8],
    ) -> Result<(), DeviceUpdaterError<Self::InnerError>> {
        self.updater
            .write_firmware(offset as usize, data)
            .map_err(|e| {
                if matches!(e, embassy_boot::FirmwareUpdaterError::BadState) {
                    DeviceUpdaterError::BadState
                } else {
                    DeviceUpdaterError::Inner(e)
                }
            })
    }
    async fn read_firmware(
        &mut self,
        offset: u32,
        buffer: &mut [u8],
    ) -> Result<(), DeviceUpdaterError<Self::InnerError>> {
        self.updater
            .read_dfu(offset, buffer)
            .map_err(DeviceUpdaterError::Inner)
    }

    async fn mark_updated(&mut self) -> Result<(), DeviceUpdaterError<Self::InnerError>> {
        self.updater
            .mark_updated()
            .map_err(DeviceUpdaterError::Inner)
    }

    async fn mark_booted(&mut self) -> Result<(), DeviceUpdaterError<Self::InnerError>> {
        self.updater
            .mark_booted()
            .map_err(DeviceUpdaterError::Inner)
    }
}
