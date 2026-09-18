#![no_main]
#![no_std]

use ariel_os_embassy_common::bootloader::{BootLoaderBackend, FlashConfig};
use ariel_os_hal::hal::bootloader::HalBootLoaderBackend;
use ariel_os_rt::memory::sections;
use embassy_boot::BlockingFirmwareUpdater;

pub const fn flash_config() -> FlashConfig {
    FlashConfig {
        active: sections::ACTIVE,
        dfu: sections::DFU,
        bootloader_state: sections::BOOTLOADER_STATE,
    }
}

pub async fn update(update: &[u8]) {
    let flash_config = flash_config();

    let config = HalBootLoaderBackend::config_firmware_updater(&flash_config);

    let mut magic = [0; 4];
    let mut updater = BlockingFirmwareUpdater::new(config, &mut magic);

    loop {
        let mut offset = 0;
        for chunk in update.chunks(4096) {
            let mut buf: [u8; 4096] = [0; 4096];
            buf[..chunk.len()].copy_from_slice(chunk);
            updater.write_firmware(offset, &buf).unwrap();
            offset += chunk.len();
        }
        updater.mark_updated().unwrap();
    }
}
