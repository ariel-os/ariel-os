//! # GNSS Time Extension
//!
//! This extension allows you to read the time information from [`Samples`] produced
//! by a compatible GNSS driver.
//!
//! # For users
//!
//! Import the trait [`GnssTimeExt`], you then access the time information on [`Samples`] generated
//! by a compatible driver by using those functions:
//!
//! - [`GnssTimeExt::time_of_fix_timestamp()`]: UTC time in seconds since UNIX epoch.
//! - [`GnssTimeExt::time_of_fix_timestamp_nanos()`]: UTC time in nanoseconds since UNIX epoch.
//!   Some sensors may have worse than nanosecond precision but this function
//!   will always return nanoseconds.
//!
//!
//! # For implementors
//!
//! Get parts with [`convert_datetime_to_parts()`].
//! You need to return a channel with the label [`Label::OpaqueGnssTime`] containing the first
//! part *directly followed* by a channel with the label [`Label::Opaque`] containing the second part.
#![cfg_attr(not(any(test, context = "native")), no_std)]
#![deny(missing_docs)]

use ariel_os_sensors::{
    Label, Reading as _,
    sensor::{ReadingChannel, Sample, SampleError, Samples},
};

// 2025-01-01T00:00Z in seconds
// Obtained using `date --date "2025-01-01 0:00Z" -u +%s`
const ARIEL_EPOCH: i64 = 1_735_689_600;

/// Error returned when trying to access the time information on [`Samples`] coming from a GNSS sensor.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssTimeExtError {
    /// Error reading value from sample, see inner error.
    Reading(SampleError),
    /// Sensor does not provide the correct channels.
    InvalidSensor,
}

impl core::fmt::Display for GnssTimeExtError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GnssTimeExtError::Reading(e) => write!(f, "reading error: {e}"),
            GnssTimeExtError::InvalidSensor => write!(f, "invalid sensor"),
        }
    }
}

impl core::error::Error for GnssTimeExtError {}

/// Error returned when trying convert the time to two parts.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssTimeExtConvertError {
    /// Timestamp is either too far in the past or too far in the future.
    Overflow,
}

impl core::fmt::Display for GnssTimeExtConvertError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GnssTimeExtConvertError::Overflow => write!(f, "time conversion overflowed"),
        }
    }
}

impl core::error::Error for GnssTimeExtConvertError {}

// Only implement for Samples
mod private {
    use ariel_os_sensors::sensor::Samples;

    pub trait Sealed {}
    impl Sealed for Samples {}
}

/// Trait to use to access time information on [`Samples`] coming from a GNSS sensor.
pub trait GnssTimeExt: private::Sealed {
    /// Returns the UTC time in seconds since UNIX epoch.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error.
    fn time_of_fix_timestamp(&self) -> Result<i64, GnssTimeExtError>;

    /// Returns the time of the last fix in nanoseconds since UNIX epoch.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error.
    fn time_of_fix_timestamp_nanos(&self) -> Result<i128, GnssTimeExtError> {
        Ok(
            i128::from(self.time_of_fix_timestamp()?) * 1_000_000_000i128
                + i128::from(self.time_of_fix_delta_nanos()?),
        )
    }
    /// Returns the nanoseconds part of the time of the last fix.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error.
    fn time_of_fix_delta_nanos(&self) -> Result<i64, GnssTimeExtError>;
}

impl GnssTimeExt for Samples {
    fn time_of_fix_timestamp(&self) -> Result<i64, GnssTimeExtError> {
        let sample = get_element_after_marker(self.samples(), Label::OpaqueGnssTime, 0)
            .ok_or(GnssTimeExtError::InvalidSensor)?;

        let since_ariel_epoch: i64 = sample.1.value().map_err(GnssTimeExtError::Reading)?.into();
        Ok(ARIEL_EPOCH + since_ariel_epoch)
    }
    fn time_of_fix_delta_nanos(&self) -> Result<i64, GnssTimeExtError> {
        let sample = get_element_after_marker(self.samples(), Label::OpaqueGnssTime, 1)
            .ok_or(GnssTimeExtError::InvalidSensor)?;

        Ok(sample.1.value().map_err(GnssTimeExtError::Reading)?.into())
    }
}

/// Convert time to parts to be put into Samples.
///
/// `timestamp_nanos` is an UTC timestamp as nanoseconds since UNIX epoch.
///
/// The first element of the tuple is to be inserted in a Channel that has the label
/// [`Label::OpaqueGnssTime`].
/// The second element must be inserted in a Channel right after the channel of the first element and
/// that has the label [`Label::Opaque`].
///
/// # Errors
/// Returns an error when `timestamp_nanos` is either too far in the past or too far in the future.
//
// # Implementation details
//
// The first part is the seconds difference with ARIEL_EPOCH, the second part is the sub-second
// part of timestamp_nanos, in nanoseconds.
//
// This is done so we can have the correct second and sub-second part without having to read both.
// Otherwise the seconds part will be off by 1 second when the timestamp is between the POSIX epoch and
// the Ariel epoch, handling this rounding adds complexity.
pub fn convert_datetime_to_parts(
    timestamp_nanos: i128,
) -> Result<(i32, i32), GnssTimeExtConvertError> {
    let timestamp_seconds = i64::try_from(timestamp_nanos / 1_000_000_000)
        .map_err(|_| GnssTimeExtConvertError::Overflow)?;

    let seconds_since_epoch = i32::try_from(
        timestamp_seconds
            .checked_sub(ARIEL_EPOCH)
            .ok_or(GnssTimeExtConvertError::Overflow)?,
    )
    .map_err(|_| GnssTimeExtConvertError::Overflow)?;

    let nanoseconds: i32 = (timestamp_nanos % 1_000_000_000)
        .try_into()
        .map_err(|_| GnssTimeExtConvertError::Overflow)?;

    Ok((seconds_since_epoch, nanoseconds))
}

fn get_element_after_marker(
    iter: impl Iterator<Item = (ReadingChannel, Sample)>,
    marker: Label,
    position: usize,
) -> Option<(ReadingChannel, Sample)> {
    let mut peekable = iter.peekable();
    while let Some((c, _)) = peekable.peek() {
        if c.label() == marker {
            break;
        }
        peekable.next();
    }
    let result = peekable.nth(position);

    // if its not an opaque channel we're doing something wrong
    if let Some((c, _)) = result
        && !(c.label() == Label::Opaque || c.label() == marker)
    {
        None
    } else {
        result
    }
}

// To obtain a timestamp with nanoseconds (works only for positive timestamps)
//  `date --date "2038-01-19T04:04:04.040404Z" -u +%s%N`
//
// For a negative timestamp, get the seconds timestamp with `date --date "1956-12-13T20:45:54.1343212Z" -u +%s`,
// if the sub-second part is non-zero, add 1. Then multiply the seconds part by 1_000_000_000, and add
// `date --date "1956-12-13T20:45:54.1343212Z" -u +%N` - 1_000_000_000.
#[cfg(test)]
mod tests {

    use ariel_os_sensors::{Sensor, sensor::ReadingChannels};

    use super::*;
    struct MockSensor {
        reading_channels: ariel_os_sensors::sensor::ReadingChannels,
    }
    impl MockSensor {
        const fn new(reading_channels: ReadingChannels) -> Self {
            Self { reading_channels }
        }
    }

    impl Sensor for MockSensor {
        fn categories(&self) -> &'static [ariel_os_sensors::Category] {
            unimplemented!()
        }
        fn display_name(&self) -> Option<&'static str> {
            unimplemented!()
        }

        fn trigger_measurement(
            &self,
        ) -> Result<(), ariel_os_sensors::sensor::TriggerMeasurementError> {
            unimplemented!()
        }

        fn wait_for_reading(&'static self) -> ariel_os_sensors::sensor::ReadingWaiter {
            unimplemented!()
        }

        fn reading_channels(&self) -> ariel_os_sensors::sensor::ReadingChannels {
            self.reading_channels
        }

        fn set_mode(
            &self,
            _mode: ariel_os_sensors::sensor::Mode,
        ) -> Result<ariel_os_sensors::sensor::State, ariel_os_sensors::sensor::SetModeError>
        {
            unimplemented!()
        }

        fn state(&self) -> ariel_os_sensors::sensor::State {
            unimplemented!()
        }

        fn label(&self) -> Option<&'static str> {
            unimplemented!()
        }

        fn part_number(&self) -> Option<&'static str> {
            unimplemented!()
        }

        fn version(&self) -> u8 {
            unimplemented!()
        }
    }

    #[test]
    fn split_valid_timestamps() {
        struct TestData {
            pub timestamp_ns: i128,
            pub expected_first: i32,
            pub expected_second: i32,
        }

        let test_dates = [
            // 2026-01-15T15:17:15.42424242Z
            TestData {
                timestamp_ns: 1768490235424242420,
                expected_first: 32800635,
                expected_second: 424242420,
            },
            // In the past
            // 2004-12-21T16:19:28.543210Z
            TestData {
                timestamp_ns: 1103645968543210000,
                expected_first: -632043632,
                expected_second: 543210000,
            },
            // Close to the higher limit
            // 2093-01-19T04:14:06.999999999Z
            TestData {
                timestamp_ns: 3883173246999999999,
                expected_first: 2147483646,
                expected_second: 999999999,
            },
            // Close to the lower limit
            // 1956-12-13T20:45:54.543210Z
            TestData {
                timestamp_ns: -411_794_045_456_790_000,
                expected_first: -2147483645,
                expected_second: -456790000,
            },
        ];

        for date in test_dates {
            let (first, second) = convert_datetime_to_parts(date.timestamp_ns).unwrap();
            assert_eq!(first, date.expected_first);
            assert_eq!(second, date.expected_second);
        }
    }

    #[test]
    fn end_to_end() {
        static SENSOR: static_cell::StaticCell<MockSensor> = static_cell::StaticCell::new();
        let sensor = SENSOR.init(MockSensor::new(ReadingChannels::from([
            ReadingChannel::new(Label::Altitude, 0, ariel_os_sensors::MeasurementUnit::Meter),
            ReadingChannel::new(
                Label::OpaqueGnssTime,
                0,
                ariel_os_sensors::MeasurementUnit::Second,
            ),
            ReadingChannel::new(Label::Opaque, -9, ariel_os_sensors::MeasurementUnit::Second),
        ])));

        // 2038-01-19T04:04:04.040404Z
        let y2k38: i64 = 2147486644;
        let y2k38_ns: i128 = 2147486644040404000;

        let parts = convert_datetime_to_parts(y2k38_ns);
        assert!(parts.is_ok());
        let (first, second) = parts.unwrap();

        let samples = Samples::from_3(
            sensor,
            [
                Sample::new(0, ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy),
                Sample::new(
                    first,
                    ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy,
                ),
                Sample::new(
                    second,
                    ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy,
                ),
            ],
        );
        let ns = (y2k38_ns - y2k38 as i128 * 1_000_000_000) as i64;
        assert_eq!(samples.time_of_fix_delta_nanos().unwrap(), ns);
        assert_eq!(samples.time_of_fix_timestamp_nanos().unwrap(), y2k38_ns);
        assert_eq!(
            samples.time_of_fix_timestamp_nanos().unwrap() / 1_000_000_000,
            y2k38 as i128
        );
        assert_eq!(samples.time_of_fix_timestamp().unwrap(), y2k38);
    }

    #[test]
    fn end_to_end_past() {
        static SENSOR: static_cell::StaticCell<MockSensor> = static_cell::StaticCell::new();
        let sensor = SENSOR.init(MockSensor::new(ReadingChannels::from([
            ReadingChannel::new(
                Label::OpaqueGnssTime,
                0,
                ariel_os_sensors::MeasurementUnit::Second,
            ),
            ReadingChannel::new(Label::Opaque, -9, ariel_os_sensors::MeasurementUnit::Second),
            ReadingChannel::new(Label::Altitude, 0, ariel_os_sensors::MeasurementUnit::Meter),
        ])));

        // 2001-02-13T23:42:00.621Z
        let timestamp: i64 = 982107720;
        let timestamp_ns: i128 = 982107720621000000;

        let parts = convert_datetime_to_parts(timestamp_ns);
        assert!(parts.is_ok());
        let (first, second) = parts.unwrap();

        let samples = Samples::from_3(
            sensor,
            [
                Sample::new(
                    first,
                    ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy,
                ),
                Sample::new(
                    second,
                    ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy,
                ),
                Sample::new(0, ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy),
            ],
        );

        let ns = (timestamp_ns - timestamp as i128 * 1_000_000_000) as i64;
        assert_eq!(samples.time_of_fix_delta_nanos().unwrap(), ns);
        assert_eq!(samples.time_of_fix_timestamp_nanos().unwrap(), timestamp_ns);
        assert_eq!(
            samples.time_of_fix_timestamp_nanos().unwrap() / 1_000_000_000,
            timestamp as i128
        );
        assert_eq!(samples.time_of_fix_timestamp().unwrap(), timestamp);
    }

    #[test]
    fn end_to_end_negative_posix() {
        static SENSOR: static_cell::StaticCell<MockSensor> = static_cell::StaticCell::new();
        let sensor = SENSOR.init(MockSensor::new(ReadingChannels::from([
            ReadingChannel::new(Label::Altitude, 0, ariel_os_sensors::MeasurementUnit::Meter),
            ReadingChannel::new(
                Label::OpaqueGnssTime,
                0,
                ariel_os_sensors::MeasurementUnit::Second,
            ),
            ReadingChannel::new(Label::Opaque, -9, ariel_os_sensors::MeasurementUnit::Second),
        ])));

        // 1960-06-23T10:12:04.12345Z

        let timestamp: i64 = -300548875;
        let timestamp_ns: i128 = -300548875876550000;

        let parts = convert_datetime_to_parts(timestamp_ns);
        assert!(parts.is_ok());
        let (first, second) = parts.unwrap();

        let samples = Samples::from_3(
            sensor,
            [
                Sample::new(0, ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy),
                Sample::new(
                    first,
                    ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy,
                ),
                Sample::new(
                    second,
                    ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy,
                ),
            ],
        );

        let ns = (timestamp_ns - timestamp as i128 * 1_000_000_000) as i64;
        assert_eq!(samples.time_of_fix_delta_nanos().unwrap(), ns);
        assert_eq!(samples.time_of_fix_timestamp_nanos().unwrap(), timestamp_ns);
        assert_eq!(
            samples.time_of_fix_timestamp_nanos().unwrap() / 1_000_000_000,
            timestamp as i128
        );
        assert_eq!(samples.time_of_fix_timestamp().unwrap(), timestamp);
    }

    #[test]
    fn end_to_end_invalid_sensor() {
        static SENSOR_WRONG_ORDER: static_cell::StaticCell<MockSensor> =
            static_cell::StaticCell::new();
        let sensor_wrong_order = SENSOR_WRONG_ORDER.init(MockSensor::new(ReadingChannels::from([
            ReadingChannel::new(
                Label::OpaqueGnssTime,
                0,
                ariel_os_sensors::MeasurementUnit::Second,
            ),
            ReadingChannel::new(Label::Altitude, 0, ariel_os_sensors::MeasurementUnit::Meter),
            ReadingChannel::new(Label::Opaque, -9, ariel_os_sensors::MeasurementUnit::Second),
        ])));

        let samples_wrong_order = Samples::from_3(
            sensor_wrong_order,
            [
                Sample::new(0, ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy),
                Sample::new(0, ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy),
                Sample::new(0, ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy),
            ],
        );

        static SENSOR_NO_SECOND_PART: static_cell::StaticCell<MockSensor> =
            static_cell::StaticCell::new();
        let sensor_no_second_part =
            SENSOR_NO_SECOND_PART.init(MockSensor::new(ReadingChannels::from([
                ReadingChannel::new(
                    Label::OpaqueGnssTime,
                    0,
                    ariel_os_sensors::MeasurementUnit::Second,
                ),
                ReadingChannel::new(Label::Altitude, 0, ariel_os_sensors::MeasurementUnit::Meter),
            ])));

        let samples_no_second_part = Samples::from_2(
            sensor_no_second_part,
            [
                Sample::new(0, ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy),
                Sample::new(0, ariel_os_sensors::sensor::SampleMetadata::UnknownAccuracy),
            ],
        );
        assert!(matches!(
            samples_wrong_order.time_of_fix_delta_nanos(),
            Err(GnssTimeExtError::InvalidSensor)
        ));
        assert!(matches!(
            samples_wrong_order.time_of_fix_timestamp_nanos(),
            Err(GnssTimeExtError::InvalidSensor)
        ));
        assert!(matches!(
            samples_no_second_part.time_of_fix_delta_nanos(),
            Err(GnssTimeExtError::InvalidSensor)
        ));
        assert!(matches!(
            samples_no_second_part.time_of_fix_timestamp_nanos(),
            Err(GnssTimeExtError::InvalidSensor)
        ));
    }
}
