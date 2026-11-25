#![no_main]
#![no_std]

use ariel_os::{hal::bootloader::HalBootLoaderBackend, rt::jump_to_application};
use ariel_os_embassy_common::bootloader::BootLoaderBackend;
use embassy_boot::{AlignedBuffer, State};

#[ariel_os::task(autostart)]
#[allow(clippy::unused_async, clippy::missing_panics_doc)]
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
    }
    jump_to_application(flash_config.active.start);
}

// For clippy.
#[cfg(not(context = "ariel-os"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
