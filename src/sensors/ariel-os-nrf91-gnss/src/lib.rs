#![no_std]

pub mod config;

use core::f64::consts::PI;

use ariel_os_debug::log::{info, warn};
use ariel_os_sensors::{
    Category, Label, MeasurementUnit, Reading, Sensor,
    sensor::{
        ReadingChannel, ReadingChannels, ReadingError, ReadingResult, ReadingWaiter, Sample,
        SampleError, SampleMetadata, Samples, State,
    },
    signal::Signal,
};
use ariel_os_sensors_utils::AtomicState;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, mutex::Mutex};
use futures::StreamExt;
use time::{Date, Month, Time, UtcDateTime, macros::utc_datetime};

use crate::config::GnssOperationMode;

// From WGS 84, Mean Radius of the Three Semi-axes in meters
const EARTH_RADIUS: f64 = 6371008.7714;
// The fraction of degrees representing a meter for the latitude (and the longitude at the equator)
// Compute at build time to improve performance
const DEGREES_PER_METER_BASE: f64 = 360.0 / (EARTH_RADIUS * 2.0 * PI);

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

pub struct Nrf91Gnss {
    config: Mutex<CriticalSectionRawMutex, config::Config>,
    label: Option<&'static str>,
    state: AtomicState,

    command_channel: Channel<CriticalSectionRawMutex, Command, 1>,
    result_channel: Signal<ReadingResult<Samples>>,
}

impl Nrf91Gnss {
    #[expect(clippy::new_without_default)]
    #[must_use]
    pub const fn new(label: Option<&'static str>) -> Self {
        Self {
            config: Mutex::new(config::Config::new(GnssOperationMode::Continuous)),
            label,
            state: AtomicState::new(State::Uninitialized),
            command_channel: Channel::new(),
            result_channel: Signal::new(),
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
                                self.result_channel.signal(Ok(samples));
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
                        let _ = self.result_channel.signal(Ok(samples));
                    } else {
                        let _ = self.result_channel.signal(Err(ReadingError::SensorAccess));
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
        let fix_valid =
            (data.flags as u32 & nrf_modem::nrfxlib_sys::NRF_MODEM_GNSS_PVT_FLAG_FIX_VALID) != 0;
        let velocity_valid = (data.flags as u32
            & nrf_modem::nrfxlib_sys::NRF_MODEM_GNSS_PVT_FLAG_VELOCITY_VALID)
            != 0;

        let date = Date::from_calendar_date(
            data.datetime.year.into(),
            Month::try_from(data.datetime.month).unwrap_or(Month::January),
            data.datetime.day,
        );

        let time = Time::from_hms_milli(
            data.datetime.hour,
            data.datetime.minute,
            data.datetime.seconds,
            data.datetime.ms,
        );

        // Default year if no GPS connection has been established yet.
        let time_parts = if data.datetime.year == 1980 {
            None
        } else if let Ok(date) = date
            && let Ok(time) = time
        {
            let datetime = UtcDateTime::new(date, time).unix_timestamp_nanos();

            Some(ariel_os_sensors_gnss_time_ext::convert_datetime_to_parts(
                datetime,
            ).unwrap())
        } else {
            None
        };

        let latitude_accuracy = f64::from(data.accuracy) * DEGREES_PER_METER_BASE;

        // For longitude, the distance represented by a degree changes depending on the latitude
        //
        // The perimeter of the circle formed by the latitude is `cos(latitude_radians) * EARTH_RADIUS * 2 * PI`
        // Full formula here is `longitude_accuracy = accuracy * 360 / (cos(latitude_radians) * EARTH_RADIUS * 2 * PI)`. We have 360 / (EARTH_RADIUS * 2 * PI) already pre-computed.
        let longitude_accuracy = f64::from(data.accuracy) * DEGREES_PER_METER_BASE
            / libm::cos(data.latitude.to_radians());

        Samples::from_8(
            self,
            [
                Sample::new(
                    time_parts.unwrap_or((0, 0)).0 as i32,
                    // Default year if no GPS connection has been established yet.
                    if time_parts.is_none() {
                        SampleMetadata::ChannelTemporarilyUnavailable
                    } else {
                        SampleMetadata::UnknownAccuracy
                    },
                ),
                Sample::new(
                    time_parts.unwrap_or((0, 0)).1 as i32,
                    // Default year if no GPS connection has been established yet.
                    if time_parts.is_none() {
                        SampleMetadata::ChannelTemporarilyUnavailable
                    } else {
                        SampleMetadata::UnknownAccuracy
                    },
                ),
                Sample::new(
                    (data.latitude * 10_000_000f64) as i32,
                    if fix_valid {
                        SampleMetadata::SymmetricalError {
                            // One meter is approximately 0.000009 degrees. Accuracy value usually between 1 and 50 meters.
                            deviation: clamp_to_u8(latitude_accuracy as f32 * 100_000f32),
                            bias: 0,
                            scaling: -5,
                        }
                    } else {
                        SampleMetadata::ChannelTemporarilyUnavailable
                    },
                ),
                Sample::new(
                    (data.longitude * 10_000_000f64) as i32,
                    if fix_valid {
                        SampleMetadata::SymmetricalError {
                            deviation: clamp_to_u8(longitude_accuracy as f32 * 100_000f32),
                            bias: 0,
                            scaling: -5,
                        }
                    } else {
                        SampleMetadata::ChannelTemporarilyUnavailable
                    },
                ),
                Sample::new(
                    (data.altitude * 100f32) as i32,
                    if fix_valid {
                        SampleMetadata::SymmetricalError {
                            deviation: clamp_to_u8(data.altitude_accuracy * 10f32),
                            bias: 0,
                            scaling: -1,
                        }
                    } else {
                        SampleMetadata::ChannelTemporarilyUnavailable
                    },
                ),
                Sample::new(
                    (data.speed * 1_000_000f32) as i32,
                    if velocity_valid {
                        SampleMetadata::SymmetricalError {
                            deviation: clamp_to_u8(data.speed_accuracy * 10f32),
                            bias: 0,
                            scaling: -1,
                        }
                    } else {
                        SampleMetadata::ChannelTemporarilyUnavailable
                    },
                ),
                Sample::new(
                    (data.vertical_speed * 1_000_000f32) as i32,
                    if velocity_valid {
                        SampleMetadata::SymmetricalError {
                            deviation: clamp_to_u8(data.vertical_speed_accuracy * 10f32),
                            bias: 0,
                            scaling: -1,
                        }
                    } else {
                        SampleMetadata::ChannelTemporarilyUnavailable
                    },
                ),
                Sample::new(
                    (data.heading * 1_000_000f32) as i32,
                    if velocity_valid {
                        SampleMetadata::SymmetricalError {
                            deviation: clamp_to_u8(data.heading_accuracy),
                            bias: 0,
                            scaling: 0,
                        }
                    } else {
                        SampleMetadata::ChannelTemporarilyUnavailable
                    },
                ),
            ],
        )
    }
}

impl Sensor for Nrf91Gnss {
    fn trigger_measurement(&self) -> Result<(), ariel_os_sensors::sensor::TriggerMeasurementError> {
        // Clear the last value if there was one.
        self.result_channel.clear();
        match self.state.get() {
            State::Measuring => {}
            State::Enabled => {
                self.state.set(State::Measuring);

                // Trigger the measurement.
                self.command_channel.clear();

                // This should never return an error as we previously cleared the command channel
                let _ = self.command_channel.try_send(Command::Trigger);
            }

            State::Disabled | State::Sleeping | State::Uninitialized => {
                return Err(ariel_os_sensors::sensor::TriggerMeasurementError::NonEnabled);
            }
        }

        Ok(())
    }

    fn wait_for_reading(&'static self) -> ariel_os_sensors::sensor::ReadingWaiter {
        match self.state.get() {
            State::Measuring => {
                self.state.set(State::Enabled);
                ReadingWaiter::new(self.result_channel.wait())
            }
            State::Enabled => ReadingWaiter::new_err(ReadingError::NotMeasuring),
            State::Disabled | State::Uninitialized | State::Sleeping => {
                ReadingWaiter::new_err(ReadingError::NonEnabled)
            }
        }
    }

    fn reading_channels(&self) -> ariel_os_sensors::sensor::ReadingChannels {
        ReadingChannels::from([
            // Putting these first so `GnssExt` doesn't spend more time searching for them.
            ReadingChannel::new(
                // Seconds since Ariel epoch (2024-01-01)
                Label::OpaqueGnssTime,
                0,
                MeasurementUnit::Second,
            ),
            ReadingChannel::new(
                // Milliseconds
                Label::Opaque,
                -3,
                MeasurementUnit::Second,
            ),
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
