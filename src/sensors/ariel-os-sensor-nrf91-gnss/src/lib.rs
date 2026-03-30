#![no_std]

pub mod config;

use core::f64::consts::PI;

use embassy_futures::select::{Either, select};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, mutex::Mutex, watch::Watch,
};
use futures::StreamExt;
use nrf_modem::{Gnss, GnssData, GnssStream};
use time::{Date, Month, Time, UtcDateTime};

use ariel_os_debug::log::{Debug2Format, debug, error, warn};
use ariel_os_sensors::{
    Category, Label, MeasurementUnit, Sensor,
    sensor::{
        Mode, ReadingChannel, ReadingChannels, ReadingError, ReadingResult, ReadingWaiter, Sample,
        SampleMetadata, Samples, State,
    },
    signal::Signal,
};
use ariel_os_sensors_utils::AtomicState;

use crate::config::{GnssOperationMode, convert_gnss_config};

// From WGS 84, Mean Radius of the Three Semi-axes in meters
const EARTH_RADIUS: f64 = 6371008.7714;
// The fraction of degrees representing a meter for the latitude (and the longitude at the equator)
// Compute at build time to improve performance
const DEGREES_PER_METER_BASE: f64 = 360.0 / (EARTH_RADIUS * 2.0 * PI);

#[derive(Debug)]
enum Command {
    Start,
    Trigger,
    Stop,
}

// Clamp to allowed u8 values and convert it to u8
fn clamp_to_u8(value: f32) -> u8 {
    value.clamp(u8::MIN.into(), u8::MAX.into()) as u8
}

pub struct Nrf91Gnss {
    config: Mutex<CriticalSectionRawMutex, config::Config>,
    label: Option<&'static str>,
    state: AtomicState,

    command_channel: Channel<CriticalSectionRawMutex, Command, 1>,
    result_signal: Signal<ReadingResult<Samples>>,
    started: Watch<CriticalSectionRawMutex, bool, 2>,
}

impl Nrf91Gnss {
    /// Create a new GNSS driver with the corresponding label.
    #[expect(clippy::new_without_default)]
    #[must_use]
    pub const fn new(label: Option<&'static str>) -> Self {
        Self {
            config: Mutex::new(config::Config::const_default()),
            label,
            state: AtomicState::new(State::Uninitialized),
            command_channel: Channel::new(),
            result_signal: Signal::new(),

            started: Watch::new(),
        }
    }

    /// Initialize the driver with a configuration. Needs to be run before triggering measurements.
    ///
    /// `run()` needs to be spawned and awaited as a task parallel to this, otherwise `init` will block indefinitely.
    pub async fn init(&self, config: config::Config) {
        debug!("init called");
        {
            *self.config.lock().await = config;
        }
        self.command_channel.send(Command::Start).await;

        let mut receiver = self.started.receiver().unwrap();

        // Wait for the state to be enabled
        while !receiver.changed().await {}

        self.state.set(State::Enabled);

        debug!("init done");
    }

    /// At this point the sensor assume the modem is already initialized with the GNSS feature enabled.
    /// In single shot mode, taking a measurement will return until a fix is obtained or the timeout is reached.
    /// In continuous or periodic mode, taking a measurement will return the current status of the GNSS module, even if a fix has not been obtained yet.
    pub async fn run(&'static self) {
        debug!("run called");
        loop {
            let command = self.command_channel.receive().await;
            debug!("Command: {:?}", defmt::Debug2Format(&command));

            let gnss_stream = {
                let configuration = self.config.lock().await;
                match command {
                    Command::Start => {
                        self.started.sender().send(true);

                        match configuration.operation_mode {
                            GnssOperationMode::Continuous => Gnss::new()
                                .await
                                .unwrap()
                                .start_continuous_fix(convert_gnss_config(&configuration))
                                .expect("Continuous fix initialization"),
                            GnssOperationMode::Periodic(period) => Gnss::new()
                                .await
                                .unwrap()
                                .start_periodic_fix(convert_gnss_config(&configuration), period)
                                .expect("Periodic fix initialization"),

                            GnssOperationMode::SingleShot(_) => {
                                // Nothing to do, SingleShot is waiting for Trigger.
                                continue;
                            }
                        }
                    }

                    Command::Trigger => match configuration.operation_mode {
                        GnssOperationMode::Continuous | GnssOperationMode::Periodic(_) => {
                            // This shouldn't happen (handled in `trigger_measurement`), but we can handle it.
                            error!("Measurement triggered when the GNSS sensor is disabled");
                            let _ = self.result_signal.signal(Err(ReadingError::NonEnabled));
                            continue;
                        }

                        GnssOperationMode::SingleShot(timeout) => Gnss::new()
                            .await
                            .unwrap()
                            .start_single_fix(convert_gnss_config(&configuration), timeout)
                            .expect("Single shot fix initialization"),
                    },

                    Command::Stop => {
                        warn!("Trying to stop the GNSS module when it is already stopped");
                        continue;
                    }
                }
            };
            self.handle_gnss_stream(gnss_stream).await;
        }
    }

    async fn handle_gnss_stream(&'static self, mut gnss_stream: GnssStream) {
        let mut latest_data = None;
        let mut should_send_update = false;

        loop {
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
                        self.config.lock().await.operation_mode,
                        GnssOperationMode::SingleShot(_)
                    ) {
                        self.result_signal.clear();
                        if let Some(data) = latest_data {
                            let samples = self.convert_to_samples(&data);
                            let _ = self.result_signal.signal(Ok(samples));
                        } else {
                            let _ = self.result_signal.signal(Err(ReadingError::SensorAccess));
                        }
                    }
                    break;
                }
                Either::First(Command::Trigger) => {
                    // Ignore, already running
                    if should_send_update == true
                        || matches!(
                            self.config.lock().await.operation_mode,
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
                            self.result_signal.clear();
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

            Some(ariel_os_sensors_gnss_time_ext::convert_datetime_to_parts(datetime).unwrap())
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
        self.result_signal.clear();
        match self.state.get() {
            State::Measuring => {
                // /!\ Silently ignoring
            }
            State::Enabled => {
                // Mark as measuring so we don't trigger twice.
                self.state.set(State::Measuring);

                if let Err(e) = self.command_channel.try_send(Command::Trigger) {
                    error!("Couldn't send trigger command: {:?} ", Debug2Format(&e))
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
        mode: Mode,
    ) -> Result<ariel_os_sensors::sensor::State, ariel_os_sensors::sensor::SetModeError> {
        let old = self.state.set_mode(mode)?;

        match mode {
            Mode::Enabled => {
                if let Err(e) = self.command_channel.try_send(Command::Start) {
                    error!("Couldn't send the command: {:?}", Debug2Format(&e));
                }
            }
            Mode::Disabled | Mode::Sleeping => {
                if let Err(e) = self.command_channel.try_send(Command::Stop) {
                    error!("Couldn't send the command: {:?}", Debug2Format(&e));
                }
            }
        }

        return Ok(old);
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
        Some("NRF91 GNSS")
    }

    fn part_number(&self) -> Option<&'static str> {
        None
    }

    fn version(&self) -> u8 {
        0
    }
}
