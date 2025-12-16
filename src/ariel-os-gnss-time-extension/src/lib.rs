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
//!     Some sensors may have worse than nanosecond precision but this function
//!     will always return nanoseconds.
//!
//!
//! # For implementors
//!
//! Get parts with [`convert_datetime_to_parts()`].
//! You need to return a channel with the label [`Label::OpaqueGnssTime`] containing the first
//! part followed by a channel with the label [`Label::Opaque`] containing the second part.
#![cfg_attr(not(context = "native"), no_std)]

use ariel_os_sensors::{
    Label, Reading as _,
    sensor::{ReadingChannel, Sample, SampleError, Samples},
};
use time::{UtcDateTime, macros::utc_datetime};

const ARIEL_EPOCH: UtcDateTime = utc_datetime!(2024-01-01 0:00);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssTimeExtError {
    /// Error reading value from sample, see inner error.
    Reading(SampleError),
    /// Sensor does not provide the correct channels
    InvalidSensor,
}

pub trait GnssTimeExt {
    /// Returns the UTC time in seconds since UNIX epoch.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error
    fn time_of_fix_timestamp(&self) -> Result<i64, GnssTimeExtError>;

    /// Returns the time of the last fix in nanoseconds since UNIX epoch.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error
    fn time_of_fix_timestamp_nanos(&self) -> Result<i128, GnssTimeExtError> {
        Ok(
            i128::from(self.time_of_fix_timestamp()?) * 1_000_000_000i128
                + i128::from(self.time_of_fix_delta_ns()?),
        )
    }
    /// Returns the nanoseconds part of the time of the last fix.
    ///
    /// # Errors
    ///
    /// Returns errors if the reading is not compatible with this extension
    /// or one [`Sample`] returned an error
    fn time_of_fix_delta_ns(&self) -> Result<i64, GnssTimeExtError>;
}

impl GnssTimeExt for Samples {
    fn time_of_fix_timestamp(&self) -> Result<i64, GnssTimeExtError> {
        let sample = get_element_after_marker(self.samples(), Label::OpaqueGnssTime, 0)
            .ok_or(GnssTimeExtError::InvalidSensor)?;

        let since_ariel_epoch: i64 = sample.1.value().map_err(GnssTimeExtError::Reading)?.into();
        Ok(ARIEL_EPOCH.unix_timestamp() + since_ariel_epoch)
    }
    fn time_of_fix_delta_ns(&self) -> Result<i64, GnssTimeExtError> {
        let sample = get_element_after_marker(self.samples(), Label::OpaqueGnssTime, 1)
            .ok_or(GnssTimeExtError::InvalidSensor)?;

        Ok(sample.1.value().map_err(GnssTimeExtError::Reading)?.into())
    }
}

/// Convert time to parts to be put into Samples
///
/// # Panics
/// Panics if the seconds since Ariel's epoch overflow.
pub fn convert_datetime_to_parts(utc_datetime: UtcDateTime) -> (i32, i32) {
    let seconds_since_epoch = (utc_datetime - ARIEL_EPOCH).whole_seconds();
    let nanoseconds = utc_datetime.nanosecond() as i32;
    (seconds_since_epoch.try_into().unwrap(), nanoseconds)
}

fn get_element_after_marker(
    iter: impl Iterator<Item = (ReadingChannel, Sample)>,
    marker: Label,
    position: usize,
) -> Option<(ReadingChannel, Sample)> {
    let mut peekable = iter.peekable();
    while let Some((c, _)) = peekable.peek() {
        if c.label() == Label::OpaqueGnssTime {
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
