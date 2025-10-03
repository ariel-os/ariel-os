#![no_main]
#![no_std]

mod pins;
mod sensors;

use ariel_os::{
    asynch::Spawner,
    debug::log::{debug, error, info},
    hal,
    i2c::controller::{I2cDevice, Kilohertz, highest_freq_in},
    sensors::{REGISTRY, Reading, sensor::Accuracy},
    time::Timer,
};
use embassy_sync::mutex::Mutex;

pub static I2C_BUS: once_cell::sync::OnceCell<
    Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, hal::i2c::controller::I2c>,
> = once_cell::sync::OnceCell::new();

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };
    debug!("Selected frequency: {:?}", i2c_config.frequency);

    let i2c_bus = pins::SensorI2c::new(peripherals.i2c_sda, peripherals.i2c_scl, i2c_config);

    let _ = I2C_BUS.set(Mutex::new(i2c_bus));

    let i2c_device = I2cDevice::new(I2C_BUS.get().unwrap());

    let spawner = Spawner::for_current_executor().await;

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
                    for (sample, reading_channel) in
                        samples.samples().zip(sensor.reading_channels().iter())
                    {
                        let channel_scaling = reading_channel.scaling();
                        let value = if channel_scaling < 0 {
                            sample.value() as f32 / 10i32.pow(-channel_scaling as u32) as f32
                        } else {
                            sample.value() as f32 * 10i32.pow(channel_scaling as u32) as f32
                        };

                        match sample.accuracy() {
                            Accuracy::SymmetricalError {
                                deviation,
                                bias,
                                scaling,
                            } => {
                                let raw_accuracy = (i16::from(bias) + i16::from(deviation))
                                    .max((i16::from(bias) - i16::from(deviation)).abs())
                                    as f32;
                                let accuracy = if scaling < 0 {
                                    raw_accuracy / 10i32.pow(-scaling as u32) as f32
                                } else {
                                    raw_accuracy * 10i32.pow(scaling as u32) as f32
                                };

                                info!(
                                    "{} ({}): {} {} ± {} {} ({})",
                                    sensor.display_name().unwrap_or("unknown"),
                                    sensor.label().unwrap_or("no label"),
                                    value,
                                    reading_channel.unit(),
                                    accuracy,
                                    reading_channel.unit(),
                                    reading_channel.label(),
                                );
                            }
                            Accuracy::NoError => {
                                info!(
                                    "{} ({}): {} {} ({})",
                                    sensor.display_name().unwrap_or("unknown"),
                                    sensor.label().unwrap_or("no label"),
                                    value,
                                    reading_channel.unit(),
                                    reading_channel.label(),
                                );
                            }
                            Accuracy::Unknown => {
                                todo!();
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("Error when reading: {}", err);
                }
            }
        }

        Timer::after_secs(2).await;
    }
}
