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

    #[cfg(feature = "nrf-modem")]
    nrf_modem_info().await;

    exit(ExitCode::SUCCESS);
}

#[cfg(feature = "nrf-modem")]
async fn nrf_modem_info() {
    info!("We have an nRF modem");

    let manufacturer: &str = &nrf_modem::send_at::<64>("AT+CGMI").await.unwrap();
    let model: &str = &nrf_modem::send_at::<64>("AT+CGMM").await.unwrap();
    let revision: &str = &nrf_modem::send_at::<64>("AT+CGMR").await.unwrap();
    let serial: &str = &nrf_modem::send_at::<64>("AT+CGSN").await.unwrap();

    info!(
        "Modem details: {} model {} revision {}, S/N {}",
        manufacturer, model, revision, serial
    );
}
