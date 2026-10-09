#![no_main]
#![no_std]

use ariel_os::{
    device_update::{DeviceUpdater, DeviceUpdaterState, HalDeviceUpdaterState},
    gpio::{Input, Pull},
    log::*,
};

use ariel_os_boards::pins;

static APP_B: &[u8] = include_bytes!("../bins/esp-blinky.bin");

// --- snip
// --- snip

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::ButtonPeripherals) {
    info!("Updater started! Waiting for button press...");

    // Configure button
    // TODO: handle board specific pullup and active state
    let pull = Pull::Up;
    let mut btn0 = Input::builder(peripherals.button0, pull)
        .build_with_interrupt()
        .unwrap();

    // Wait for the button being pressed
    let _ = btn0.wait_for_falling_edge().await;

    info!("Button pressed, initiating update...");

    let mut updater_state = HalDeviceUpdaterState::new();
    let mut updater = updater_state.updater();

    let mut offset = 0;

    for chunk in APP_B.chunks(4096) {
        let mut buf = [0; 4096];
        buf[..chunk.len()].copy_from_slice(chunk);
        updater.write_firmware(offset, &buf).await.unwrap();
        offset += chunk.len() as u32;
    }

    updater.mark_updated().await.unwrap();

    info!("Updater done.");

    ariel_os::power::reboot();
}
