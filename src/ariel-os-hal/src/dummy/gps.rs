use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};

use ariel_os_embassy_common::gps::{Config, GpsData, GpsDataSender};

pub async fn single_shot_gps_fix(timeout_seconds: u32) -> GpsData {
    GpsData {
        position: None,
        velocity: None,
        datetime: None,
        recorded_at: embassy_time::Instant::now(),
    }
}

pub async fn init_gps(_spawner: Spawner, _sender: GpsDataSender<'static>, _config: Config) {}
