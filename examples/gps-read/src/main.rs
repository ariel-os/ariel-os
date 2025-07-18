#![no_main]
#![no_std]

use ariel_os::debug::log::*;
use ariel_os::reexports::nrf_modem::{self, GnssData};
use ariel_os::time::Timer;
use futures::StreamExt;
#[ariel_os::task(autostart)]
async fn main() {
    info!("Hello World!");

    let response = nrf_modem::send_at::<64>("AT+CGMI").await.unwrap();

    info!("Modem Manufacturer: {}", response.as_str());
    nrf_modem::send_at::<64>("AT%XCOEX0").await.unwrap();

    let mut receiver = ariel_os::gps::get_reciever().await.unwrap();
    loop {
        // let fix = ariel_os::gps::request_gps_fix().await;

        let fix = receiver.changed().await;
        info!("GPS Fix: {:?}", fix);

        // Timer::after_millis(1000).await;
    }

    // let gnss = nrf_modem::Gnss::new().await.unwrap();
    // info!("GNSS initialized");
    // let mut stream = gnss.start_continuous_fix(nrf_modem::GnssConfig {
    //     elevation_threshold_angle: 15,
    //     use_case: nrf_modem::GnssUsecase {
    //         low_accuracy: false,
    //         scheduled_downloads_disable: false,
    //     },
    //     nmea_mask: nrf_modem::NmeaMask {
    //         gga: true,
    //         gll: true,
    //         gsa: true,
    //         gsv: true,
    //         rmc: true,
    //     },
    //     timing_source: nrf_modem::GnssTimingSource::Tcxo,
    //     power_mode: nrf_modem::GnssPowerSaveMode::Disabled,
    // });
    // info!("GNSS stream started");
    // let mut stream = match stream {
    //     Ok(s) => s,
    //     Err(e) => {
    //         error!("Failed to start GNSS: {:?}", e);
    //         return;
    //     }
    // };

    // info!("GNSS stream is ready");
    // while let Some(value) = stream.next().await {
    //     debug!("GNSS event");
    //     if let Err(e) = value {
    //         error!("GNSS Error: {:?}", e);
    //         continue;
    //     }
    //     if let Ok(evt) = value {
    //         match evt {
    //             GnssData::Agps(agps) => {
    //                 info!("GNSS AGPS: {:?}", agps.data_flags);
    //             }
    //             GnssData::Nmea(nmea) => {
    //                 info!("GNSS NMEA: {}", nmea.as_str());
    //             }
    //             GnssData::PositionVelocityTime(pos) => {
    //                 info!(
    //                     "GNSS Position: {},{},{} | accuracy: {}",
    //                     pos.latitude, pos.longitude, pos.altitude, pos.accuracy
    //                 );
    //             }
    //         }
    //     }
    // }
}
