#![no_main]
#![no_std]

use ariel_os::{hal::bootloader::HalBootLoaderBackend, rt::memory::sections};
use ariel_os_embassy_common::bootloader::{BootLoaderBackend, FlashConfig};
use embassy_boot::{AlignedBuffer, State};

#[ariel_os::task(autostart)]
async fn main() {
    // TODO: get this from ariel-os-rt.
    let flash_config = FlashConfig {
        active: sections::ACTIVE,
        dfu: sections::DFU,
        bootloader_state: sections::BOOTLOADER_STATE,
    };

    let config = HalBootLoaderBackend::config(&flash_config);

    // TODO: set up watchdog

    let mut aligned_buf =
        AlignedBuffer([0; <HalBootLoaderBackend as BootLoaderBackend>::ALIGNED_BUFFER_SIZE]);
    let mut bootloader = embassy_boot::BootLoader::new(config);
    let state = bootloader.prepare_boot(aligned_buf.as_mut()).unwrap();

    if matches!(state, State::DfuDetach) {
        todo!("Implement our DFU mode");
    } else {
        HalBootLoaderBackend::load_active(&flash_config);
    }
}
