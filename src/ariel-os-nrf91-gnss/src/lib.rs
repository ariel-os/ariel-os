#![no_std]

pub mod config;
mod state_atomic;

use ariel_os_debug::log::warn;
use ariel_os_sensors::{
    Category, Label, MeasurementUnit, Sensor,
    sensor::{
        Accuracy, ReadingChannel, ReadingChannels, ReadingError, ReadingResult, ReadingWaiter,
        Sample, Samples, State, UnfilteredSamples, ValueError,
    },
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, mutex::Mutex};
use futures::StreamExt;
use time::{Date, Month, Time, UtcDateTime, macros::utc_datetime};

use crate::config::GnssOperationMode;
use crate::state_atomic::StateAtomic;

const ARIEL_EPOCH: UtcDateTime = utc_datetime!(2024-01-01 0:00);

enum Command {
    Trigger,
    Stop,
}

// Clamp to allowed u8 values and convert it to u8
fn clamp_to_u8(value: f32) -> u8 {
    value.clamp(u8::MIN.into(), u8::MAX.into()) as u8
}

fn default_gnss_config() -> nrf_modem::GnssConfig {
    nrf_modem::GnssConfig {
        elevation_threshold_angle: 5,
        use_case: nrf_modem::GnssUsecase {
            low_accuracy: false,
            scheduled_downloads_disable: false,
        },
        nmea_mask: nrf_modem::NmeaMask {
            gga: false,
            gll: false,
            gsa: false,
            gsv: false,
            rmc: false,
        },
        timing_source: nrf_modem::GnssTimingSource::Tcxo,
        power_mode: nrf_modem::GnssPowerSaveMode::Disabled,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssValueError {
    /// Error reading value from sample, see inner error.
    Reading(ValueError),
    /// Sensor does not provide the correct channels
    InvalidSensor,
}

pub trait Nrf91GnssExt {
    /// Returns the time of the last fix in seconds since UNIX epoch.
    fn time_of_fix_seconds(&self) -> Result<i64, GnssValueError>;

    /// Returns the time of the last fix in milliseconds since UNIX epoch.
    fn time_of_fix(&self) -> Result<i64, GnssValueError> {
        Ok(self.time_of_fix_seconds()? * 1000 + self.time_of_fix_delta_ms()?)
    }
    /// Returns the milliseconds part of the time of the last fix.
    fn time_of_fix_delta_ms(&self) -> Result<i64, GnssValueError>;
}

impl Nrf91GnssExt for Samples {
    fn time_of_fix_seconds(&self) -> Result<i64, GnssValueError> {
        let sample = self
            .unfiltered_samples()
            .nth(6) // 7th sample is TimeOfFix
            .ok_or(GnssValueError::InvalidSensor)?;

        let since_ariel_epoch: i64 = sample.value().map_err(GnssValueError::Reading)?.into();
        Ok(ARIEL_EPOCH.unix_timestamp() + since_ariel_epoch)
    }
    fn time_of_fix_delta_ms(&self) -> Result<i64, GnssValueError> {
        let sample = self
            .unfiltered_samples()
            .nth(7)
            .ok_or(GnssValueError::InvalidSensor)?;

        Ok(sample.value().map_err(GnssValueError::Reading)?.into())
    }
}

pub struct Nrf91Gnss {
    config: Mutex<CriticalSectionRawMutex, config::Config>,
    label: Option<&'static str>,
    state: StateAtomic,

    command_channel: Channel<CriticalSectionRawMutex, Command, 1>,
    result_channel: Channel<CriticalSectionRawMutex, ReadingResult<Samples>, 1>,
}

impl Nrf91Gnss {
    #[expect(clippy::new_without_default)]
    #[must_use]
    pub const fn new(label: Option<&'static str>) -> Self {
        Self {
            config: Mutex::new(config::Config::new(GnssOperationMode::Continuous)),
            label,
            state: StateAtomic::new(State::Uninitialized),
            command_channel: Channel::new(),
            result_channel: Channel::new(),
        }
    }

    /// At this point the sensor assume the modem is already initialized with the GNSS feature enabled.
    /// In single shot mode, taking a measurement will return until a fix is obtained or the timeout is reached.
    /// In continuous or periodic mode, taking a measurement will return the current status of the GNSS module, even if a fix has not been obtained yet.
    pub async fn init(&self, config: config::Config) {
        let mut c = self.config.lock().await;

        *c = config;
        self.state.set(State::Enabled);
    }

    pub async fn run(&'static self) {
        loop {
            let command = self.command_channel.receive().await;
            let gnss = nrf_modem::Gnss::new().await.unwrap();

            let (mut gnss_stream, mut triggered) = match command {
                Command::Trigger => {
                    let (stream, set_triggered) = match self.config.lock().await.operation_mode {
                        GnssOperationMode::Continuous => (
                            gnss.start_continuous_fix(default_gnss_config())
                                .expect("Continuous fix initialization"),
                            true,
                        ),
                        GnssOperationMode::Periodic(period) => (
                            gnss.start_periodic_fix(default_gnss_config(), period)
                                .expect("Periodic fix initialization"),
                            true,
                        ),
                        GnssOperationMode::SingleShot(timeout) => (
                            gnss.start_single_fix(default_gnss_config(), timeout)
                                .expect("Single shot fix initialization"),
                            false,
                        ),
                    };
                    (Some(stream), set_triggered)
                }
                Command::Stop => {
                    warn!("Trying to stop the GNSS module when it is already stopped");
                    (None, false)
                }
            };

            if let Some(mut stream) = gnss_stream.take() {
                // do the loop here

                let mut latest_data = None;
                while let Some(value) = stream.next().await {
                    match self.command_channel.try_receive() {
                        Ok(Command::Stop) => {
                            // Stop the GNSS operation
                            break;
                        }
                        Ok(Command::Trigger) => {
                            // Ignore, already running
                            if triggered == true
                                || matches!(
                                    self.config.lock().await.operation_mode,
                                    GnssOperationMode::SingleShot(_)
                                )
                            {
                                warn!("Received Trigger command while GNSS is already running");
                            } else {
                                triggered = true;
                            }
                        }
                        _ => {}
                    }
                    match value {
                        Ok(nrf_modem::GnssData::PositionVelocityTime(pos)) => {
                            if triggered {
                                let samples = self.convert_to_samples(&pos);
                                self.result_channel.clear();
                                self.result_channel.send(Ok(samples)).await;
                                triggered = false;
                            }
                            if matches!(
                                self.config.lock().await.operation_mode,
                                GnssOperationMode::SingleShot(_)
                            ) {
                                latest_data = Some(pos);
                            }
                        }
                        _ => { /* Ignore other data */ }
                    }
                }

                if matches!(
                    self.config.lock().await.operation_mode,
                    GnssOperationMode::SingleShot(_)
                ) {
                    self.result_channel.clear();
                    if let Some(data) = latest_data {
                        let samples = self.convert_to_samples(&data);
                        let _ = self.result_channel.send(Ok(samples)).await;
                    } else {
                        let _ = self
                            .result_channel
                            .send(Err(ReadingError::SensorAccess))
                            .await;
                    }
                }

                let _ = stream.deactivate().await;
            }
        }
    }
    fn convert_to_samples(
        &'static self,
        data: &nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame,
    ) -> Samples {
        let date = Date::from_calendar_date(
            data.datetime.year.into(),
            Month::try_from(data.datetime.month).unwrap(),
            data.datetime.day,
        )
        .unwrap();
        let time = Time::from_hms(
            data.datetime.hour,
            data.datetime.minute,
            data.datetime.seconds,
        )
        .unwrap();
        let datetime = UtcDateTime::new(date, time);

        let seconds_since_epoch = (datetime - ARIEL_EPOCH).whole_seconds();
        Samples::new_8(
            [
                Sample::new(
                    (data.latitude * 10_000_000f64) as i32,
                    if data.accuracy < 0.1 {
                        Accuracy::ValueTemporarilyUnavailable
                    } else {
                        Accuracy::SymmetricalError {
                            // Accuracy in meters, max value is 25,5. Need to watch out for overflows.
                            deviation: clamp_to_u8(data.accuracy * 10f32),
                            bias: 0,
                            scaling: -1,
                        }
                    },
                ),
                Sample::new(
                    (data.longitude * 10_000_000f64) as i32,
                    if data.accuracy < 0.1 {
                        Accuracy::ValueTemporarilyUnavailable
                    } else {
                        Accuracy::SymmetricalError {
                            deviation: clamp_to_u8(data.accuracy * 10f32),
                            bias: 0,
                            scaling: -1,
                        }
                    },
                ),
                Sample::new(
                    (data.altitude * 100f32) as i32,
                    if data.accuracy < 0.1 {
                        Accuracy::ValueTemporarilyUnavailable
                    } else {
                        Accuracy::SymmetricalError {
                            deviation: clamp_to_u8(data.altitude_accuracy * 10f32),
                            bias: 0,
                            scaling: -1,
                        }
                    },
                ),
                Sample::new(
                    (data.speed * 1_000_000f32) as i32,
                    if data.accuracy < 0.1 {
                        Accuracy::ValueTemporarilyUnavailable
                    } else {
                        Accuracy::SymmetricalError {
                            deviation: clamp_to_u8(data.speed_accuracy * 10f32),
                            bias: 0,
                            scaling: -1,
                        }
                    },
                ),
                Sample::new(
                    (data.vertical_speed * 1_000_000f32) as i32,
                    if data.accuracy < 0.1 {
                        Accuracy::ValueTemporarilyUnavailable
                    } else {
                        Accuracy::SymmetricalError {
                            deviation: clamp_to_u8(data.vertical_speed_accuracy * 10f32),
                            bias: 0,
                            scaling: -1,
                        }
                    },
                ),
                Sample::new(
                    (data.heading * 1_000_000f32) as i32,
                    if data.accuracy < 0.1 {
                        Accuracy::ValueTemporarilyUnavailable
                    } else {
                        Accuracy::SymmetricalError {
                            deviation: clamp_to_u8(data.heading_accuracy),
                            bias: 0,
                            scaling: 0,
                        }
                    },
                ),
                Sample::new(
                    seconds_since_epoch as i32,
                    // Default year if no GPS connection has been established yet.
                    if data.datetime.year == 1980 {
                        Accuracy::ValueTemporarilyUnavailable
                    } else {
                        Accuracy::Unknown
                    },
                ),
                Sample::new(
                    data.datetime.ms as i32,
                    // Default year if no GPS connection has been established yet.
                    if data.datetime.year == 1980 {
                        Accuracy::ValueTemporarilyUnavailable
                    } else {
                        Accuracy::Unknown
                    },
                ),
            ],
            self,
        )
    }
}

impl Sensor for Nrf91Gnss {
    fn trigger_measurement(&self) -> Result<(), ariel_os_sensors::sensor::TriggerMeasurementError> {
        if self.state.get() != State::Enabled {
            return Err(ariel_os_sensors::sensor::TriggerMeasurementError::NonEnabled);
        }

        // Clear the last value if there was one.
        self.result_channel.clear();
        self.command_channel.clear();

        // FIXME: return an error if the channel is full
        let _ = self.command_channel.try_send(Command::Trigger);

        Ok(())
    }

    fn wait_for_reading(&'static self) -> ariel_os_sensors::sensor::ReadingWaiter {
        if self.state.get() != State::Enabled {
            return ReadingWaiter::Err(ReadingError::NonEnabled);
        }

        ReadingWaiter::Waiter {
            waiter: self.result_channel.receive(),
        }
    }

    fn reading_channels(&self) -> ariel_os_sensors::sensor::ReadingChannels {
        ReadingChannels::from([
            ReadingChannel::new(
                // Accuracy is in meters.
                Label::Latitude,
                -7,
                MeasurementUnit::DecimalDegree,
            ),
            ReadingChannel::new(
                // Max value of an i32 is 2,147,483,647
                // The value ranges from -180 to 180, we can go to 10^-7, making the max possible value 214.
                // The smallest distance between two points at the equator is 40,075,016/360 * 10^-7 ~= 0.012 meters
                // Accuracy is in meters.
                Label::Longitude,
                -7,
                MeasurementUnit::DecimalDegree,
            ),
            ReadingChannel::new(
                // Smallest distance between two altitude reading: 0.01 meters.
                // Value ranging from -21,474,836 meters to 21,474,836 meters.
                Label::Altitude,
                -2,
                MeasurementUnit::Meter,
            ),
            ReadingChannel::new(
                // Max value is 2,147 m/s
                // Smallest distance between two speed readings: 0.000001 m/s
                Label::GroundSpeed,
                -6,
                MeasurementUnit::MeterPerSecond,
            ),
            ReadingChannel::new(
                // Max value is 2,147 m/s
                // Smallest distance between two speed readings: 0.000001 m/s
                Label::VerticalSpeed,
                -6,
                MeasurementUnit::MeterPerSecond,
            ),
            ReadingChannel::new(
                // Max value is 360 degrees
                // Smallest distance between two heading readings: 0.000001 degrees
                Label::Heading,
                -6,
                MeasurementUnit::Degree,
            ),
            ReadingChannel::new(
                // Seconds since Ariel epoch (2024-01-01)
                Label::Opaque,
                0,
                MeasurementUnit::Second,
            ),
            ReadingChannel::new(
                // Milliseconds
                Label::Opaque,
                -3,
                MeasurementUnit::Second,
            ),
        ])
    }

    fn set_mode(
        &self,
        mode: ariel_os_sensors::sensor::Mode,
    ) -> Result<ariel_os_sensors::sensor::State, ariel_os_sensors::sensor::SetModeError> {
        let new_state = self.state.set_mode(mode);

        if new_state == State::Sleeping {
            let _ = self.command_channel.try_send(Command::Stop);
        }

        if new_state == State::Uninitialized {
            Err(ariel_os_sensors::sensor::SetModeError::Uninitialized)
        } else {
            Ok(new_state)
        }
    }

    fn state(&self) -> ariel_os_sensors::sensor::State {
        self.state.get()
    }

    fn categories(&self) -> &'static [ariel_os_sensors::Category] {
        &[Category::Gnss]
    }

    fn label(&self) -> Option<&'static str> {
        self.label
    }

    fn display_name(&self) -> Option<&'static str> {
        Some("NRF91 GNSS")
    }

    fn part_number(&self) -> Option<&'static str> {
        None
    }

    fn version(&self) -> u8 {
        0
    }
}
