use embassy_executor::Spawner;

use ariel_os_debug::log::{error, info, trace};
use ariel_os_embassy_common::gps::{
    Config, GpsData, GpsDataReceiver, GpsDataSender, GpsDataWatch, GpsFixMode,
};

static WATCH: GpsDataWatch = Watch::new();

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
// Initialize the GPS with the provided configuration and spawner.
pub(crate) async fn init_gps(spawner: Spawner) {
    let config = config();
    let sender: GpsDataSender<'_> = WATCH.sender();
    crate::hal::gps::init_gps(spawner, sender, config).await;
}

#[doc(hidden)]
fn get_sender<'a>() -> GpsDataSender<'a> {
    WATCH.sender()
}

/// Get a Watch receiver to get updates on GPS data.
///
/// If there is too many receivers, it will return `None`.
///
/// In `GpsFixMode::SingleShot`, this will return `None` as the GPS fix is not continuously updated.
pub async fn get_receiver<'a>() -> Option<GpsDataReceiver<'a>> {
    let config = config();

    if matches!(config.mode, GpsFixMode::SingleShot(_)) {
        return None;
    }
    WATCH.receiver()
}

pub async fn request_gps_fix() -> GpsData {
    let config = config();

    match config.mode {
        GpsFixMode::SingleShot(timeout) => crate::hal::gps::single_shot_gps_fix(timeout).await,
        _ => {
            // Get the latest value
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
}
