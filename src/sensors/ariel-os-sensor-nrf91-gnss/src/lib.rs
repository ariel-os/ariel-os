//! Driver for the GNSS modem of the [nRF91 SiP series] compatible with [`ariel_os_sensors::Sensor`].
//!
//! This driver has 3 operation modes selectable in [`config::Config`]:
//! - `Contiunuous`: the GNSS will always stay active and send an update approximately every 3 seconds.
//! - `Periodic`: acquires a fix every x seconds and sends an update.
//! - `SingleShot`: acquires a fix only when requested by [`Sensor::trigger_measurement()`] with a set timeout in seconds.
//!
//! In `Continuous` and `Periodic` mode the driver will try to obtain a fix as soon as it's initialized.
//! Using [`Sensor::trigger_measurement()`] then [`Sensor::wait_for_reading()`] will wait for the next update.
//!
//! If a fix cannot be obtained in the allowed amount of time, [`Sensor::wait_for_reading()`] will still return a reading, but some of the [`Sample`]s' values will be [`SampleError::TemporarilyUnavailable`].
//!
//! To access the time returned by the GNSS fix, you need to use the [`ariel_os_sensors_gnss_time_ext::GnssTimeExt`] trait.
//!
//! [nRF91 SiP series]: https://docs.nordicsemi.com/category/nrf-91-series
//! [`SampleError::TemporarilyUnavailable`]: ariel_os_sensors::sensor::SampleError::TemporarilyUnavailable

#![no_std]

pub mod config;

use core::f64::consts::PI;

use embassy_futures::select::{Either, select};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, once_lock::OnceLock,
};
use futures_util::StreamExt as _;
use nrf_modem::{Gnss, GnssData, GnssStream};
use time::{Date, Month, Time, UtcDateTime};

use ariel_os_log::{Debug2Format, debug, error, warn};
use ariel_os_sensors::{
    Category, Label, MeasurementUnit, Sensor,
    sensor::{
        Mode, ReadingChannel, ReadingChannels, ReadingError, ReadingResult, ReadingWaiter, Sample,
        SampleMetadata, Samples, State,
    },
    signal::Signal,
};
use ariel_os_sensors_utils::AtomicState;

use crate::config::{Config, GnssOperationMode, convert_gnss_config};

// From WGS 84, Mean Radius of the Three Semi-axes in meters.
// Source: table 3.5 in https://nsgreg.nga.mil/doc/view?i=4085
const EARTH_RADIUS: f64 = 6_371_008.771_4;
// The fraction of degrees representing a meter for the latitude (and the longitude at the equator).
// Computed at build time to improve performance.
const DEGREES_PER_METER_BASE: f64 = 360.0 / (EARTH_RADIUS * 2.0 * PI);

#[derive(Debug)]
enum Command {
    Start,
    Trigger,
    Stop,
}

// Clamp to allowed u8 values and convert it to u8
#[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn clamp_to_u8(value: f32) -> u8 {
    value.clamp(u8::MIN.into(), u8::MAX.into()) as u8
}

pub struct Nrf91Gnss {
    config: OnceLock<config::Config>,
    label: Option<&'static str>,
    state: AtomicState,
    command_channel: Channel<CriticalSectionRawMutex, Command, 1>,
    result_signal: Signal<ReadingResult<Samples>>,
}

impl Nrf91Gnss {
    /// Creates a new uninitialized GNSS driver with the corresponding label.
    #[must_use]
    pub const fn new(label: Option<&'static str>) -> Self {
        Self {
            config: OnceLock::new(),
            label,
            state: AtomicState::new(State::Uninitialized),
            command_channel: Channel::new(),
            result_signal: Signal::new(),
        }
    }

    /// Initializes the driver with a configuration. Needs to be run before triggering measurements.
    #[expect(
        clippy::unused_async,
        reason = "uniformity with other drivers so it can be codegened"
    )]
    pub async fn init(&self, config: config::Config) {
        self.state.set(State::Enabled);

        let _ = self.config.init(config);

        debug!("nRF91 GNSS driver initialized");
    }

    /// At this point the sensor assume the modem is already initialized with the GNSS feature enabled.
    /// In single shot mode, taking a measurement will return until a fix is obtained or the timeout is reached.
    /// In continuous or periodic mode, taking a measurement will return the current status of the GNSS module, even if a fix has not been obtained yet.
    ///
    /// # Panics
    ///
    /// When the modem library refuses the config or fails to initialize the GNSS system.
    pub async fn run(&'static self) {
        let configuration = self.config.get().await;
        loop {
            // Wait until the state is Enabled.
            if self.state.get() != State::Enabled
                && !matches!(self.command_channel.receive().await, Command::Start)
            {
                continue;
            }

            let gnss_stream = match configuration.operation_mode {
                GnssOperationMode::Continuous => Gnss::new()
                    .await
                    .unwrap()
                    .start_continuous_fix(convert_gnss_config(configuration))
                    .expect("Continuous fix initialization"),
                GnssOperationMode::Periodic(period) => Gnss::new()
                    .await
                    .unwrap()
                    .start_periodic_fix(convert_gnss_config(configuration), period)
                    .expect("Periodic fix initialization"),

                GnssOperationMode::SingleShot(timeout) => {
                    match self.command_channel.receive().await {
                        Command::Trigger => Gnss::new()
                            .await
                            .unwrap()
                            .start_single_fix(convert_gnss_config(configuration), timeout)
                            .expect("Single shot fix initialization"),

                        Command::Stop => {
                            warn!("Trying to stop the GNSS module when it is already stopped");
                            continue;
                        }
                        Command::Start => {
                            debug!("Ignoring Start command in SingleShot mode");
                            continue;
                        }
                    }
                }
            };

            self.handle_gnss_stream(gnss_stream, configuration).await;
        }
    }

    async fn handle_gnss_stream(
        &'static self,
        mut gnss_stream: GnssStream,
        configuration: &Config,
    ) {
        let mut latest_data = None;
        let mut should_send_update = false;

        loop {
            if !matches!(self.state.get(), State::Enabled | State::Measuring) {
                error!("Invalid state found");
                break;
            }

            match select(self.command_channel.receive(), gnss_stream.next()).await {
                Either::First(Command::Start) => {
                    warn!("GNSS sensor already started");
                }
                Either::First(Command::Stop) => {
                    break;
                }
                Either::Second(None) => {
                    // In single shot mode, the stream ending means it has found a fix.
                    if matches!(
                        configuration.operation_mode,
                        GnssOperationMode::SingleShot(_)
                    ) {
                        self.result_signal.clear();
                        if let Some(data) = latest_data {
                            let samples = self.convert_to_samples(&data);
                            self.result_signal.signal(Ok(samples));
                        } else {
                            self.result_signal.signal(Err(ReadingError::SensorAccess));
                        }
                    }
                    break;
                }
                Either::First(Command::Trigger) => {
                    // Ignore, already running
                    if should_send_update
                        || matches!(
                            configuration.operation_mode,
                            GnssOperationMode::SingleShot(_)
                        )
                    {
                        warn!("Received Trigger command while already processing one");
                    } else {
                        should_send_update = true;
                    }
                }
                Either::Second(Some(Ok(message))) => match message {
                    GnssData::PositionVelocityTime(pos) => {
                        if should_send_update {
                            let samples = self.convert_to_samples(&pos);
                            self.result_signal.signal(Ok(samples));
                            should_send_update = false;
                        }

                        // Only matters if we're in SingleShot mode.
                        latest_data = Some(pos);
                    }
                    GnssData::Nmea(nmea_message) => {
                        debug!("NMEA: {}", nmea_message.as_str());
                    }
                    GnssData::Agps(_) => {
                        // Ignore AGPS data
                    }
                },
                Either::Second(Some(Err(e))) => {
                    warn!("GNSS error: {}", e);
                }
            }
        }
        let _ = gnss_stream.deactivate().await;
    }

    /// Convert time from `nrf_modem` to parts that can be put in samples.
    ///
    /// # Panics
    ///
    /// When the date is too far in the future / past.
    fn convert_to_time_parts(
        data: &nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame,
    ) -> Option<(i32, i32)> {
        let parsed_date = Date::from_calendar_date(
            data.datetime.year.into(),
            Month::try_from(data.datetime.month).ok()?,
            data.datetime.day,
        )
        .ok()?;

        let time = Time::from_hms_milli(
            data.datetime.hour,
            data.datetime.minute,
            data.datetime.seconds,
            data.datetime.ms,
        )
        .ok()?;

        // Default year when no GNSS fix.
        if data.datetime.year == 1980 {
            return None;
        }

        let timestamp = UtcDateTime::new(parsed_date, time).unix_timestamp_nanos();

        Some(ariel_os_sensors_gnss_time_ext::convert_datetime_to_parts(timestamp).unwrap())
    }

    #[expect(clippy::cast_possible_truncation)]
    fn convert_to_samples(
        &'static self,
        data: &nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame,
    ) -> Samples {
        let time_parts = Self::convert_to_time_parts(data);

        let (time_seconds_part, time_nanos_part) = if let Some(time_parts) = time_parts {
            (
                Sample::new(time_parts.0, SampleMetadata::UnknownAccuracy),
                Sample::new(time_parts.1, SampleMetadata::UnknownAccuracy),
            )
        } else {
            (
                Sample::new(0, SampleMetadata::ChannelTemporarilyUnavailable),
                Sample::new(0, SampleMetadata::ChannelTemporarilyUnavailable),
            )
        };

        let latitude_accuracy = (f64::from(data.accuracy) * DEGREES_PER_METER_BASE) as f32;

        // For longitude, the distance represented by a degree changes depending on the latitude

        // The perimeter of the circle formed by the latitude is `cos(latitude_radians) * EARTH_RADIUS * 2 * PI`
        // Full formula here is `longitude_accuracy = accuracy * 360 / (cos(latitude_radians) * EARTH_RADIUS * 2 * PI)`. We have 360 / (EARTH_RADIUS * 2 * PI) already pre-computed.
        let longitude_accuracy = (f64::from(data.accuracy) * DEGREES_PER_METER_BASE
            / libm::cos(data.latitude.to_radians())) as f32;

        // Convert for 10^-7 channel scaling.
        let latitude_value = (data.latitude * 10_000_000f64) as i32;
        // Convert for 10^-7 channel scaling.
        let longitude_value = (data.longitude * 10_000_000f64) as i32;
        // Convert for 10^-2 channel scaling.
        let altitude_value = (data.altitude * 100f32) as i32;

        let fix_valid = (u32::from(data.flags)
            & nrf_modem::nrfxlib_sys::NRF_MODEM_GNSS_PVT_FLAG_FIX_VALID)
            != 0;

        let (latitude, longitude, altitude) = if fix_valid {
            let latitude = Sample::new(
                latitude_value,
                SampleMetadata::SymmetricalError {
                    // One meter is approximately 0.000009 degrees. Accuracy value usually between 1 and 50 meters.
                    deviation: clamp_to_u8(latitude_accuracy * 100_000f32),
                    bias: 0,
                    // 10^-5 scaling for the error.
                    scaling: -5,
                },
            );
            let longitude = Sample::new(
                longitude_value,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(longitude_accuracy * 100_000f32),
                    bias: 0,
                    // 10^-5 scaling for the error.
                    scaling: -5,
                },
            );
            let altitude = Sample::new(
                altitude_value,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(data.altitude_accuracy * 10f32),
                    bias: 0,
                    // 10^-1 scaling for the error.
                    scaling: -1,
                },
            );
            (latitude, longitude, altitude)
        } else {
            (
                Sample::new(
                    latitude_value,
                    SampleMetadata::ChannelTemporarilyUnavailable,
                ),
                Sample::new(
                    longitude_value,
                    SampleMetadata::ChannelTemporarilyUnavailable,
                ),
                Sample::new(
                    altitude_value,
                    SampleMetadata::ChannelTemporarilyUnavailable,
                ),
            )
        };

        // Convert for 10^-6 channel scaling.
        let horizontal_speed_value = (data.speed * 1_000_000f32) as i32;
        // Convert for 10^-6 channel scaling.
        let vertical_speed_value = (data.vertical_speed * 1_000_000f32) as i32;
        // Convert for 10^-6 channel scaling.
        let heading_value = (data.heading * 1_000_000f32) as i32;

        let velocity_valid = (u32::from(data.flags)
            & nrf_modem::nrfxlib_sys::NRF_MODEM_GNSS_PVT_FLAG_VELOCITY_VALID)
            != 0;

        let (horizontal_speed, vertical_speed, heading) = if velocity_valid {
            let horizontal_speed = Sample::new(
                horizontal_speed_value,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(data.speed_accuracy * 10f32),
                    bias: 0,
                    // 10^-1 scaling for the error.
                    scaling: -1,
                },
            );

            let vertical_speed = Sample::new(
                vertical_speed_value,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(data.vertical_speed_accuracy * 10f32),
                    bias: 0,
                    // 10^-1 scaling for the error.
                    scaling: -1,
                },
            );

            let heading = Sample::new(
                heading_value,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(data.heading_accuracy),
                    bias: 0,
                    scaling: 0,
                },
            );
            (horizontal_speed, vertical_speed, heading)
        } else {
            (
                Sample::new(
                    horizontal_speed_value,
                    SampleMetadata::ChannelTemporarilyUnavailable,
                ),
                Sample::new(
                    vertical_speed_value,
                    SampleMetadata::ChannelTemporarilyUnavailable,
                ),
                Sample::new(heading_value, SampleMetadata::ChannelTemporarilyUnavailable),
            )
        };

        Samples::from_8(
            self,
            [
                time_seconds_part,
                time_nanos_part,
                latitude,
                longitude,
                altitude,
                horizontal_speed,
                vertical_speed,
                heading,
            ],
        )
    }
}

impl Sensor for Nrf91Gnss {
    fn trigger_measurement(&self) -> Result<(), ariel_os_sensors::sensor::TriggerMeasurementError> {
        // Clear the last value if there was one.
        self.result_signal.clear();
        match self.state.get() {
            State::Measuring => {
                warn!("GNSS: already measuring");
            }
            State::Enabled => {
                // Mark as measuring so we don't trigger twice.
                self.state.set(State::Measuring);

                if let Err(e) = self.command_channel.try_send(Command::Trigger) {
                    error!("Couldn't send trigger command: {:?} ", Debug2Format(&e));
                }
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
                ReadingWaiter::new(self.result_signal.wait())
            }
            State::Enabled => ReadingWaiter::new_err(ReadingError::NotMeasuring),
            State::Disabled | State::Uninitialized | State::Sleeping => {
                ReadingWaiter::new_err(ReadingError::NonEnabled)
            }
        }
    }

    fn reading_channels(&self) -> ariel_os_sensors::sensor::ReadingChannels {
        ReadingChannels::from([
            // Putting the time-related channels first so `GnssTimeExt` doesn't iterate much to find them.
            ReadingChannel::new(
                // Opaque: seconds since [`ariel_os_sensors_gnss_time_ext::ARIEL_EPOCH`].
                Label::OpaqueGnssTime,
                0,
                MeasurementUnit::Second,
            ),
            ReadingChannel::new(
                // Opaque: nanoseconds.
                Label::Opaque,
                // Scaling doesn't matter here as GnssTimeExt doesn't read it.
                0,
                MeasurementUnit::Second,
            ),
            ReadingChannel::new(
                // Latitude in degrees.
                // Resolution of this channel is 1*10^-7 degrees.
                Label::Latitude,
                -7,
                MeasurementUnit::DecimalDegree,
            ),
            ReadingChannel::new(
                // Longitude in degrees.
                // Resolution of this channel is 1*10^-7 degrees.
                Label::Longitude,
                -7,
                MeasurementUnit::DecimalDegree,
            ),
            ReadingChannel::new(
                // Altitude in meters. Value ranging from -21,474,836 meters to 21,474,836 meters.
                // Resolution of this channel is 0.01 meters.
                Label::Altitude,
                -2,
                MeasurementUnit::Meter,
            ),
            ReadingChannel::new(
                // Ground speed in m/s. Max value is 2,147 m/s.
                // Resolution of this channel is 1*10^-6 m/s.
                Label::GroundSpeed,
                -6,
                MeasurementUnit::MeterPerSecond,
            ),
            ReadingChannel::new(
                // Vertical speed in m/s. Max value is 2,147 m/s.
                // Resolution of this channel is 1*10^-6 m/s.
                Label::VerticalSpeed,
                -6,
                MeasurementUnit::MeterPerSecond,
            ),
            ReadingChannel::new(
                // Heading in degrees. From 0 to 360 degrees.
                // Resolution of this channel is 1*10^-6 degrees.
                Label::Heading,
                -6,
                MeasurementUnit::Degree,
            ),
        ])
    }

    fn set_mode(
        &self,
        mode: Mode,
    ) -> Result<ariel_os_sensors::sensor::State, ariel_os_sensors::sensor::SetModeError> {
        let old = self.state.set_mode(mode)?;

        let result = match mode {
            Mode::Enabled => self.command_channel.try_send(Command::Start),
            Mode::Disabled | Mode::Sleeping => self.command_channel.try_send(Command::Stop),
        };

        // We also check the state in the run loop.
        // An error here means that the channel is full, meaning the state update will be noticed once the run loop checks the new state.
        if let Err(err) = result {
            debug!("Couldn't send the command: {:?}", Debug2Format(&err));
        }

        Ok(old)
    }

    fn state(&self) -> State {
        self.state.get()
    }

    fn categories(&self) -> &'static [ariel_os_sensors::Category] {
        &[Category::Gnss]
    }

    fn label(&self) -> Option<&'static str> {
        self.label
    }

    fn display_name(&self) -> Option<&'static str> {
        Some("nRF91 GNSS")
    }

    fn part_number(&self) -> Option<&'static str> {
        Some("nRF91xx")
    }

    fn version(&self) -> u8 {
        0
    }
}
