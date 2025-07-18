use crate::reexports::nrf_modem;
use crate::reexports::nrf_modem::GnssData;
use ariel_os_debug::log::{error, info, trace};
use ariel_os_embassy_common::gps::{
    Config, GpsData, GpsDateTime, GpsFixMode, GpsPosition, GpsVelocity,
};
use embassy_executor::Spawner;
use embassy_nrf::config;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};
use futures::StreamExt;

static WATCH: Watch<CriticalSectionRawMutex, GpsData, 4> = Watch::new();

pub(crate) fn config() -> Config {
    #[cfg(not(feature = "gps-config-override"))]
    {
        Config::default()
    }
    #[cfg(feature = "gps-config-override")]
    {
        unsafe extern "Rust" {
            fn __ariel_os_gps_config() -> Config;
        }
        unsafe { __ariel_os_gps_config() }
    }
}

pub(crate) async fn start_gps(spawner: Spawner) {
    let config = config();
    let sender: embassy_sync::watch::Sender<'_, CriticalSectionRawMutex, GpsData, 4> =
        WATCH.sender();
    crate::hal::gps::init_gps(spawner, sender, config).await;
}

#[doc(hidden)]
pub fn get_sender<'n>() -> embassy_sync::watch::Sender<'n, CriticalSectionRawMutex, GpsData, 4> {
    WATCH.sender()
}

/// Get a Watch receiver to get updates on GPS data.
///
/// If there is too many receivers, it will return `None`.
///
/// In `GpsFixMode::SingleShot`, this will return `None` as the GPS fix is not continuously updated.
pub async fn get_reciever<'a>()
-> Option<embassy_sync::watch::Receiver<'a, CriticalSectionRawMutex, GpsData, 4>> {
    let config = config();

    if matches!(config.mode, GpsFixMode::SingleShot) {
        return None;
    }
    WATCH.receiver()
}

pub async fn request_gps_fix() -> GpsData {
    // TODO: implement the SingleShot mode

    let config = config();
    if matches!(config.mode, GpsFixMode::SingleShot) {
        crate::hal::gps::single_shot_gps_fix(100).await
    } else {
        let mut receiver = WATCH.anon_receiver();
        receiver.try_get().unwrap_or_else(|| {
            // If the watch is not set, return an empty GpsData
            GpsData {
                position: None,
                velocity: None,
                datetime: None,
                recorded_at: embassy_time::Instant::now(),
            }
        })
    }
}
