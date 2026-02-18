#![no_main]
#![no_std]

mod sensors;

use ariel_os::{
    asynch::Spawner,
    debug::log::{debug, error, info},
    sensors::{Label, REGISTRY, Reading},
    time::Timer,
};
use ariel_os_sensors_gnss_time_ext::GnssTimeExt;

#[ariel_os::task(autostart)]
#[expect(unsafe_code)]
async fn main() {
    let spawner = unsafe { Spawner::for_current_executor().await };

    sensors::NRF91_GNSS
        .init(ariel_os_nrf91_gnss::config::Config::default())
        .await;
    spawner.spawn(sensors::nrf91_gnss_runner()).unwrap();

    loop {
        // Trigger measurements of each sensor
        for sensor in REGISTRY.sensors() {
            if let Err(err) = sensor.trigger_measurement() {
                error!("Error when triggering a measurement: {}", err);
            }
        }

        // Then, collect and display the readings one at a time
        for sensor in REGISTRY.sensors() {
            let reading = sensor.wait_for_reading().await;

            match reading {
                Ok(samples) => {
                    for (reading_channel, sample) in samples.samples() {
                        let value = sample.value().map(|v| {
                            v as f32 / 10i32.pow((-reading_channel.scaling()) as u32) as f32
                        });

                        if reading_channel.label() != Label::Opaque {
                            info!(
                                "label: {:?} reading: {} = {:?} {} (accuracy: {:?})",
                                sensor.label(),
                                reading_channel.label(),
                                value,
                                reading_channel.unit(),
                                sample.metadata()
                            );
                        } else {
                            debug!(
                                "label: {:?} opaque reading: {:?} {} (accuracy: {:?})",
                                sensor.label(),
                                value,
                                reading_channel.unit(),
                                sample.metadata()
                            )
                        }
                    }

                    info!("Time of fix: {:?}", samples.time_of_fix_timestamp());
                }
                Err(err) => {
                    error!("Error when reading: {:?}", err);
                }
            }
        }

        Timer::after_secs(2).await;
    }
}
