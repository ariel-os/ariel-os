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
    info!("We have an nRF modem:");

    /// Applies post-processing of `send_at("AT+CG…")` commands.
    ///
    /// * Unwraps them (because at least those commands don't really fail that way),
    /// * removes stray newlines, and
    /// * removes a trailing "OK".
    fn cleanup_atcg(arg: &Result<impl core::ops::Deref<Target = str>, nrf_modem::Error>) -> &str {
        let result: &str = arg
            .as_ref()
            // never observed to err, but often observed to produce "ERROR" values
            .unwrap()
            .as_ref();
        let result = result.strip_suffix("\r\n").unwrap_or(result);
        result.strip_suffix("\r\nOK").unwrap_or(result)
    }

    info!(
        "Model: {}",
        cleanup_atcg(&nrf_modem::send_at::<64>("AT+CGMI").await)
    );
    info!(
        "Revision: {}",
        cleanup_atcg(&nrf_modem::send_at::<64>("AT+CGMR").await)
    );
    info!(
        "Serial: {}",
        cleanup_atcg(&nrf_modem::send_at::<64>("AT+CGSN").await)
    );
}
