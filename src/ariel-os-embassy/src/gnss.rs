use embassy_executor::Spawner;

use ariel_os_embassy_common::gnss::{
    Config, GnssData, GnssDataReceiver, GnssDataSender, GnssDataWatch, GnssOperationMode,
};

static WATCH: GnssDataWatch = GnssDataWatch::new();

pub(crate) fn config() -> Config {
    #[cfg(not(feature = "gnss-config-override"))]
    {
        Config::default()
    }
    #[cfg(feature = "gnss-config-override")]
    {
        unsafe extern "Rust" {
            fn __ariel_os_gnss_config() -> Config;
        }
        unsafe { __ariel_os_gnss_config() }
    }
}
// Initialize the GNSS with the provided configuration and spawner.
#[allow(dead_code, reason = "false positive during builds outside of laze")]
pub(crate) async fn init_gnss(spawner: Spawner) {
    let config = config();
    let sender: GnssDataSender<'_> = WATCH.sender();
    crate::hal::gnss::init_gnss(spawner, sender, config).await;
}

/// Get an `embassy_sync::watch::Receiver` to get updates on GNSS data.
///
/// If there is too many receivers active, it will return `None`.
///
/// In `GnssOperationMode::SingleShot`, this will return `None` as the GNSS fix is not continuously updated.
pub fn get_receiver<'a>() -> Option<GnssDataReceiver<'a>> {
    let config = config();

    if matches!(config.operation_mode, GnssOperationMode::SingleShot(_)) {
        return None;
    }
    WATCH.receiver()
}

/// Request a GNSS fix.
///
/// In single shot operation, this function will return after a fix has been obtained or the timeout has expired.
///
/// In continuous or periodic modes, it will return the latest GNSS data available.
pub async fn request_gnss_fix() -> GnssData {
    let config = config();

    if let GnssOperationMode::SingleShot(timeout) = config.operation_mode {
        crate::hal::gnss::single_shot_gnss_fix(timeout).await
    } else {
        // Get the latest value
        let mut receiver = WATCH.anon_receiver();
        receiver.try_get().unwrap_or_else(|| {
            // If the watch is not set, return an empty GnssData
            GnssData {
                position: None,
                velocity: None,
                datetime: None,
                recorded_at: embassy_time::Instant::now(),
            }
        })
    }
}
