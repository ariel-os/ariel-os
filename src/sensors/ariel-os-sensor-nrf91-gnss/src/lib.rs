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

const SAMPLE_UNAVAILABLE: Sample = Sample::new(0, SampleMetadata::ChannelTemporarilyUnavailable);

// From WGS 84, Mean Radius of the Three Semi-axes in meters.
// Source: table 3.5 in https://nsgreg.nga.mil/doc/view?i=4085
const EARTH_RADIUS: f64 = 6_371_008.771_4;
// The fraction of degrees representing a meter for the latitude (and the longitude at the equator).
// Computed at build time to improve performance.
const DEGREES_PER_METER_BASE: f64 = 360.0 / (EARTH_RADIUS * 2.0 * PI);

#[derive(Debug, PartialEq, Eq)]
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

struct StreamState {
    // Single shot mode, return data only when the stream gets closed.
    single_shot: bool,

    // If it should send an update the next time it receives data.
    send_update_on_next_data_received: bool,

    // Latest data received from nrf_modem, used in SingleShot mode.
    latest_data: Option<nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame>,
}

#[derive(Debug)]
enum NextAction {
    /// Do another iteration of the loop.
    Continue,
    /// Stop the processing loop.
    Break,
    /// Send data to the app.
    SendData(nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame),
    /// Send data and break.
    SendFinalData(nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame),
    /// Send error and break.
    SendFinalError(ReadingError),
}
impl StreamState {
    pub fn new(operation_mode: GnssOperationMode) -> StreamState {
        StreamState {
            single_shot: matches!(operation_mode, GnssOperationMode::SingleShot(_)),
            send_update_on_next_data_received: false,
            latest_data: None,
        }
    }

    /// Handle control flow commands received from the `command` channel, returns what the main loop should do.
    pub fn handle_command(&mut self, command: &Command) -> NextAction {
        match command {
            Command::Start => {
                warn!("GNSS sensor already started");
                NextAction::Continue
            }
            Command::Stop => NextAction::Break,
            Command::Trigger => {
                if self.send_update_on_next_data_received || self.single_shot {
                    warn!("Received Trigger command while already processing one");
                } else {
                    self.send_update_on_next_data_received = true;
                }
                NextAction::Continue
            }
        }
    }

    /// Handles data from nrf-modem, returns true if the stream processing should be stopped.
    pub fn handle_stream_data(
        &mut self,
        stream_data: Option<Result<GnssData, nrf_modem::Error>>,
    ) -> NextAction {
        let Some(data) = stream_data else {
            // If we're here that means the stream has been closed.

            let next_action = if self.single_shot {
                // In SingleShot mode that means we need to return some data.
                self.latest_data.map_or(
                    NextAction::SendFinalError(ReadingError::SensorAccess),
                    NextAction::SendFinalData,
                )
            } else {
                NextAction::Break
            };
            return next_action;
        };

        let data = match data {
            Ok(d) => d,
            Err(e) => {
                warn!("GNSS error: {}", e);
                return NextAction::Continue;
            }
        };

        match data {
            GnssData::PositionVelocityTime(pos) => {
                if self.send_update_on_next_data_received {
                    self.send_update_on_next_data_received = false;
                    NextAction::SendData(pos)
                } else {
                    self.latest_data = Some(pos);
                    NextAction::Continue
                }
            }
            GnssData::Nmea(nmea_message) => {
                debug!("NMEA: {}", nmea_message.as_str());
                NextAction::Continue
            }
            GnssData::Agps(_) => {
                //  ignored
                NextAction::Continue
            }
        }
    }
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
    /// In continuous mode, taking a measurement will return the current status of the GNSS module, even if a fix has not been obtained yet.
    /// In periodic mode, taking a measurement will return the current status of the GNSS module when it is currently active:
    /// when it wakes up it will send some invalid fixes before returning a valid fix and going to sleep.
    ///
    /// # Panics
    ///
    /// When the modem library refuses the config or fails to initialize the GNSS system.
    pub async fn run(&'static self) {
        let configuration = self.config.get().await;

        loop {
            // Wait until the state is Enabled.
            // `set_mode` updates `self.state` before sending a start command in `command_channel`.
            if self.state.get() != State::Enabled
                && self.command_channel.receive().await != Command::Start
            {
                continue;
            }

            // Calling Gnss::new().await powers up the modem, it can't be factored out.
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

            self.handle_stream_events(gnss_stream, configuration).await;
        }
    }

    // Return data to the application.
    fn send_data(&'static self, pos: &nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame) {
        let samples = self.convert_to_samples(pos);
        self.result_signal.signal(Ok(samples));
    }

    // Handle the events when the GNSS is running.
    async fn handle_stream_events(
        &'static self,
        mut gnss_stream: GnssStream,
        configuration: &Config,
    ) {
        let mut stream_state = StreamState::new(configuration.operation_mode);

        loop {
            if !matches!(self.state.get(), State::Enabled | State::Measuring) {
                error!("Invalid state found");
                break;
            }

            let next_action = match select(self.command_channel.receive(), gnss_stream.next()).await
            {
                Either::First(command) => stream_state.handle_command(&command),
                Either::Second(data) => stream_state.handle_stream_data(data),
            };
            match next_action {
                NextAction::Break => {
                    break;
                }
                NextAction::Continue => {}
                NextAction::SendData(pos) => self.send_data(&pos),
                NextAction::SendFinalData(pos) => {
                    self.send_data(&pos);
                    break;
                }
                NextAction::SendFinalError(e) => {
                    self.result_signal.signal(Err(e));
                    break;
                }
            }
        }

        // Deactivate the stream asynchronously to havoid blocking when dropping it.
        let _ = gnss_stream.deactivate().await;
    }

    #[expect(clippy::cast_possible_truncation)]
    fn convert_to_samples(
        &'static self,
        data: &nrf_modem::nrfxlib_sys::nrf_modem_gnss_pvt_data_frame,
    ) -> Samples {
        let (time_seconds_part, time_nanos_part) =
            if let Some((seconds, nanos)) = convert_to_time_parts(data) {
                (
                    Sample::new(seconds, SampleMetadata::UnknownAccuracy),
                    Sample::new(nanos, SampleMetadata::UnknownAccuracy),
                )
            } else {
                (SAMPLE_UNAVAILABLE, SAMPLE_UNAVAILABLE)
            };

        let fix_valid = (u32::from(data.flags)
            & nrf_modem::nrfxlib_sys::NRF_MODEM_GNSS_PVT_FLAG_FIX_VALID)
            != 0;

        let (latitude, longitude, altitude) = if fix_valid {
            let latitude_accuracy = (f64::from(data.accuracy) * DEGREES_PER_METER_BASE) as f32;

            let latitude = Sample::new(
                // Convert for 10^-7 channel scaling.
                (data.latitude * 10_000_000f64) as i32,
                SampleMetadata::SymmetricalError {
                    // One meter is approximately 0.000009 degrees. Accuracy value usually between 1 and 50 meters.
                    deviation: clamp_to_u8(latitude_accuracy * 100_000f32),
                    bias: 0,
                    // 10^-5 scaling for the error.
                    scaling: -5,
                },
            );

            // For longitude, the distance represented by a degree changes depending on the latitude

            // The perimeter of the circle formed by the latitude is `cos(latitude_radians) * EARTH_RADIUS * 2 * PI`
            // Full formula here is `longitude_accuracy = accuracy * 360 / (cos(latitude_radians) * EARTH_RADIUS * 2 * PI)`. We have 360 / (EARTH_RADIUS * 2 * PI) already pre-computed.
            let longitude_accuracy = (f64::from(data.accuracy) * DEGREES_PER_METER_BASE
                / libm::cos(data.latitude.to_radians()))
                as f32;

            let longitude = Sample::new(
                // Convert for 10^-7 channel scaling.
                (data.longitude * 10_000_000f64) as i32,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(longitude_accuracy * 100_000f32),
                    bias: 0,
                    // 10^-5 scaling for the error.
                    scaling: -5,
                },
            );
            let altitude = Sample::new(
                // Convert for 10^-2 channel scaling.
                (data.altitude * 100f32) as i32,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(data.altitude_accuracy * 10f32),
                    bias: 0,
                    // 10^-1 scaling for the error.
                    scaling: -1,
                },
            );
            (latitude, longitude, altitude)
        } else {
            (SAMPLE_UNAVAILABLE, SAMPLE_UNAVAILABLE, SAMPLE_UNAVAILABLE)
        };

        let velocity_valid = (u32::from(data.flags)
            & nrf_modem::nrfxlib_sys::NRF_MODEM_GNSS_PVT_FLAG_VELOCITY_VALID)
            != 0;

        let (horizontal_speed, vertical_speed, heading) = if velocity_valid {
            let horizontal_speed = Sample::new(
                // Convert for 10^-6 channel scaling.
                (data.speed * 1_000_000f32) as i32,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(data.speed_accuracy * 10f32),
                    bias: 0,
                    // 10^-1 scaling for the error.
                    scaling: -1,
                },
            );

            let vertical_speed = Sample::new(
                // Convert for 10^-6 channel scaling.
                (data.vertical_speed * 1_000_000f32) as i32,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(data.vertical_speed_accuracy * 10f32),
                    bias: 0,
                    // 10^-1 scaling for the error.
                    scaling: -1,
                },
            );

            let heading = Sample::new(
                // Convert for 10^-6 channel scaling.
                (data.heading * 1_000_000f32) as i32,
                SampleMetadata::SymmetricalError {
                    deviation: clamp_to_u8(data.heading_accuracy),
                    bias: 0,
                    scaling: 0,
                },
            );
            (horizontal_speed, vertical_speed, heading)
        } else {
            (SAMPLE_UNAVAILABLE, SAMPLE_UNAVAILABLE, SAMPLE_UNAVAILABLE)
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

        // Start / Stop the loop in `run()`.
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
