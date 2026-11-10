#![no_main]
#![no_std]

use ariel_os::hal::bootloader::HalBootLoaderBackend;
use ariel_os_embassy_common::bootloader::BootLoaderBackend;
use embassy_boot::{AlignedBuffer, State};

#[ariel_os::task(autostart)]
async fn main() {
    ariel_os::log::debug!("bootloader started.");
    let flash_config = ariel_os_bootloader_common::flash_config();
    let config = HalBootLoaderBackend::config(&flash_config);

    // TODO: set up watchdog

    let mut aligned_buf =
        AlignedBuffer([0; <HalBootLoaderBackend as BootLoaderBackend>::ALIGNED_BUFFER_SIZE]);
    let mut bootloader = embassy_boot::BootLoader::new(config);

    ariel_os::log::debug!("bootloader: calling prepare_boot()");
    let state = bootloader.prepare_boot(aligned_buf.as_mut()).unwrap();

    if matches!(state, State::DfuDetach) {
        todo!("Implement our DFU mode");
    } else {
        ariel_os::log::debug!("bootloader: calling load_active()");
        HalBootLoaderBackend::load_active(&flash_config);
    }
}
