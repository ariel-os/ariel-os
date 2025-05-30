#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, exit, log::*};

#[ariel_os::task(autostart)]
async fn main() {
    info!("Available information:");
    info!("Board type: {}", ariel_os::buildinfo::BOARD);
    if let Ok(id) = ariel_os::identity::device_id_bytes() {
        info!("Device ID: {}", Hex(id));
    } else {
        info!("Device ID is unavailable.");
    }
    if let Ok(eui48) = ariel_os::identity::interface_eui48(0) {
        info!("Device's first EUI-48 address: {}", eui48);
    }

    exit(ExitCode::SUCCESS);
}
