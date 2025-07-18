use embassy_executor::Spawner;

use ariel_os_debug::log::{error, info, trace};
use ariel_os_embassy_common::gps::{
    Config, GpsData, GpsDateTime, GpsFixMode, GpsPosition, GpsVelocity,
};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, once_lock::OnceLock, watch::Watch,
};
use futures::StreamExt;
use nrf_modem::GnssData;

static GPS_CONFIG: OnceLock<Config> = OnceLock::new();

pub async fn single_shot_gps_fix(timeout_seconds: u32) -> GpsData {
    let gnss_config = gnss_config_from_config(*GPS_CONFIG.get().await);

    let gnss = nrf_modem::Gnss::new().await.unwrap();

    let mut latest_pvt: Option<nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame> = None;
    let mut stream = gnss
        .start_single_fix(gnss_config, timeout_seconds as u16)
        .expect("Single shot fix initialization");

    while let Some(evt) = stream.next().await {
        match evt {
            Ok(GnssData::Agps(agps)) => {
                defmt::info!("GNSS AGPS: {:?}", agps.data_flags);
            }
            Ok(GnssData::Nmea(nmea)) => {
                defmt::info!("GNSS NMEA: {}", nmea.as_str());
            }
            Ok(GnssData::PositionVelocityTime(pos)) => {
                latest_pvt = Some(pos);
            }
            Err(e) => {
                error!("GNSS Error: {:?}", e);
            }
        }
    }

    stream.deactivate().await.unwrap();

    if let Some(pos) = latest_pvt {
        convert_pvt_to_gps_data(pos)
    } else {
        GpsData {
            position: None,
            velocity: None,
            datetime: None,
            recorded_at: embassy_time::Instant::now(),
        }
    }
}

pub fn gnss_config_from_config(config: Config) -> nrf_modem::GnssConfig {
    nrf_modem::GnssConfig {
        elevation_threshold_angle: 5,
        use_case: nrf_modem::GnssUsecase {
            low_accuracy: false,
            scheduled_downloads_disable: false,
        },
        nmea_mask: nrf_modem::NmeaMask {
            gga: true,
            gll: true,
            gsa: true,
            gsv: true,
            rmc: true,
        },
        timing_source: nrf_modem::GnssTimingSource::Tcxo,
        power_mode: nrf_modem::GnssPowerSaveMode::Disabled,
    }
}

pub async fn init_gps(
    spawner: Spawner,
    sender: embassy_sync::watch::Sender<'static, CriticalSectionRawMutex, GpsData, 4>,
    config: Config,
) {
    let _ = GPS_CONFIG.init(config);

    if matches!(config.mode, GpsFixMode::SingleShot) {
        return;
    }

    let gnss_config = gnss_config_from_config(config);

    let gnss = nrf_modem::Gnss::new().await.unwrap();

    let mut stream = match config.mode {
        GpsFixMode::Continuous => gnss
            .start_continuous_fix(gnss_config)
            .expect("Continuous fix initialization"),
        GpsFixMode::Periodic(period) => gnss
            .start_periodic_fix(gnss_config, period)
            .expect("Periodic fix initialization"),
        _ => {
            unreachable!()
        }
    };

    spawner.must_spawn(gps_loop(stream, sender));
}

#[embassy_executor::task]
async fn gps_loop(
    mut stream: nrf_modem::GnssStream,
    sender: embassy_sync::watch::Sender<'static, CriticalSectionRawMutex, GpsData, 4>,
) {
    while let Some(value) = stream.next().await {
        match value {
            Ok(GnssData::PositionVelocityTime(pos)) => {
                let gps_data = convert_pvt_to_gps_data(pos);

                sender.send(gps_data);
            }
            Ok(GnssData::Nmea(nmea)) => {
                trace!("Received NMEA: {}", nmea.as_str());
            }
            Ok(GnssData::Agps(agps)) => {
                trace!("Received AGPS: {:?}", agps.data_flags);
            }
            Err(e) => {
                error!("GNSS Error: {:?}", e);
            }
        }
    }
}

fn convert_pvt_to_gps_data(pos: nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame) -> GpsData {
    GpsData {
        position: Some(GpsPosition {
            latitude: pos.latitude.into(),
            longitude: pos.longitude.into(),
            altitude: pos.altitude.into(),
            accuracy: pos.accuracy.into(),
            altitude_accuracy: pos.altitude_accuracy.into(),
        }),
        velocity: Some(GpsVelocity {
            speed: pos.speed.into(),
            speed_accuracy: pos.speed_accuracy.into(),
            vertical_speed: pos.vertical_speed.into(),
            vertical_speed_accuracy: pos.vertical_speed_accuracy.into(),
            heading: pos.heading.into(),
            heading_accuracy: pos.heading_accuracy.into(),
        }),
        datetime: Some(GpsDateTime {
            year: pos.datetime.year.into(),
            month: pos.datetime.month.into(),
            day: pos.datetime.day.into(),
            hour: pos.datetime.hour.into(),
            minute: pos.datetime.minute.into(),
            second: pos.datetime.seconds.into(),
            milliseconds: pos.datetime.ms.into(),
        }),
        recorded_at: embassy_time::Instant::now(),
    }
}
