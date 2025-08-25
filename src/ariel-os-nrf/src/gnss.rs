use ariel_os_debug::log::{error, trace};
use ariel_os_embassy_common::gnss::{
    Config, GnssData, GnssDateTime, GnssOperationMode, GnssPosition, GnssVelocity,
};
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, once_lock::OnceLock};
use futures::StreamExt;

static GNSS_CONFIG: OnceLock<Config> = OnceLock::new();

pub async fn single_shot_gnss_fix(timeout_seconds: u16) -> GnssData {
    let gnss_config = gnss_config_from_config(*GNSS_CONFIG.get().await);

    let gnss = nrf_modem::Gnss::new().await.unwrap();

    let mut latest_pvt: Option<nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame> = None;
    let mut stream = gnss
        .start_single_fix(gnss_config, timeout_seconds as u16)
        .expect("Single shot fix initialization");

    // The modem will run until the timeout is reached or a fix is obtained.
    while let Some(evt) = stream.next().await {
        match evt {
            Ok(nrf_modem::GnssData::Agps(agps)) => {
                trace!("GNSS AGNSS: {:?}", agps.data_flags);
            }
            Ok(nrf_modem::GnssData::Nmea(nmea)) => {
                trace!("GNSS NMEA: {}", nmea.as_str());
            }
            Ok(nrf_modem::GnssData::PositionVelocityTime(pos)) => {
                latest_pvt = Some(pos);
            }
            Err(e) => {
                error!("GNSS Error: {:?}", e);
            }
        }
    }

    // Stops the GNSS module of the modem.
    stream.deactivate().await.unwrap();

    if let Some(pos) = latest_pvt {
        convert_pvt_to_gnss_data(pos)
    } else {
        GnssData {
            position: None,
            velocity: None,
            datetime: None,
            recorded_at: embassy_time::Instant::now(),
        }
    }
}

/// `nrf_modem::GnssConfig` cannot be cloned.
/// Because we want to save the settings after initialization it's easier to clone `Config`.
///
/// The idea is that later we may want to change some settings like `low_accuracy` or `elevation_threshold_angle`.
fn gnss_config_from_config(_config: Config) -> nrf_modem::GnssConfig {
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

pub async fn init_gnss(
    spawner: Spawner,
    sender: embassy_sync::watch::Sender<'static, CriticalSectionRawMutex, GnssData, 4>,
    config: Config,
) {
    let _ = GNSS_CONFIG.init(config);

    if matches!(config.operation_mode, GnssOperationMode::SingleShot(_)) {
        return;
    }

    let gnss_config = gnss_config_from_config(config);

    let gnss = nrf_modem::Gnss::new().await.unwrap();

    let stream = match config.operation_mode {
        GnssOperationMode::Continuous => gnss
            .start_continuous_fix(gnss_config)
            .expect("Continuous fix initialization"),
        GnssOperationMode::Periodic(period) => gnss
            .start_periodic_fix(gnss_config, period)
            .expect("Periodic fix initialization"),
        _ => {
            unreachable!()
        }
    };

    spawner.must_spawn(gnss_loop(stream, sender));
}

#[embassy_executor::task]
async fn gnss_loop(
    mut stream: nrf_modem::GnssStream,
    sender: embassy_sync::watch::Sender<'static, CriticalSectionRawMutex, GnssData, 4>,
) {
    while let Some(value) = stream.next().await {
        match value {
            Ok(nrf_modem::GnssData::PositionVelocityTime(pos)) => {
                let gnss_data = convert_pvt_to_gnss_data(pos);

                sender.send(gnss_data);
            }
            Ok(nrf_modem::GnssData::Nmea(nmea)) => {
                trace!("Received NMEA: {}", nmea.as_str());
            }
            Ok(nrf_modem::GnssData::Agps(agps)) => {
                trace!("Received AGPS: {:?}", agps.data_flags);
            }
            Err(e) => {
                error!("GNSS Error: {:?}", e);
            }
        }
    }
}

fn convert_pvt_to_gnss_data(
    pos: nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame,
) -> GnssData {
    GnssData {
        position: Some(GnssPosition {
            latitude: pos.latitude.into(),
            longitude: pos.longitude.into(),
            altitude: pos.altitude.into(),
            accuracy: pos.accuracy.into(),
            altitude_accuracy: pos.altitude_accuracy.into(),
        }),
        velocity: Some(GnssVelocity {
            speed: pos.speed.into(),
            speed_accuracy: pos.speed_accuracy.into(),
            vertical_speed: pos.vertical_speed.into(),
            vertical_speed_accuracy: pos.vertical_speed_accuracy.into(),
            heading: pos.heading.into(),
            heading_accuracy: pos.heading_accuracy.into(),
        }),
        datetime: Some(GnssDateTime {
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
