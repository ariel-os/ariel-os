//! Utilities to handle datetime samples.
//!
//! Samples are limited to i32 values, a timestamp expressing microseconds since UNIX epoch needs to be stored in an i64.
//! To work around the samples limitation, we have to split the timestamp into two i32 samples.
//! You need to use [`combine_timestamp`] to reconstruct the i64 timestamp from the two samples labeled with [`Label::DateTimeUpper`] and [`Label::DateTimeLower`].
//!
//! For implementors you need to use [`split_timestamp`] to split an i64 timestamp into two samples.

use crate::sample::Sample;

/// Contains two samples representing a split i64 timestamp.
pub struct DateTimeSplit {
    /// Sample containing the upper 4 bytes
    pub upper: Sample,
    /// Sample containing the lower 4 bytes
    pub lower: Sample,
}

/// Splits a timestamp into two samples. Has to be used by sensor driver implementors to return a timestamp.
#[must_use]
pub fn split_timestamp(timestamp: i64) -> DateTimeSplit {
    let lower_bytes = (timestamp & 0xFFFF_FFFF) as i32;
    let upper_bytes = ((timestamp >> 32) & 0xFFFF_FFFF) as i32;
    let upper = Sample::new(upper_bytes, crate::sample::Accuracy::Unknown);
    let lower = Sample::new(lower_bytes, crate::sample::Accuracy::Unknown);

    DateTimeSplit { upper, lower }
}

/// Combines two samples into a timestamp.
/// Has to be used to reconstruct a timestamp obtained from two samples labeled with [`Label::DateTimeUpper`] and [`Label::DateTimeLower`].
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn combine_timestamp(split: &DateTimeSplit) -> i64 {
    let upper = (i64::from(split.upper.value())) << 32;
    let lower = i64::from(split.lower.value() as u32);
    upper | lower
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_and_combine() {
        let timestamps = [
            1,
            -1,
            0,
            1758028184047000,
            -1758028184047000,
            0xA0_FFFF_FFFF,
            0xA0_AAFF_FFFF,
            0xA0_0FFF_FFFF,
            i64::MAX,
            i64::MIN,
        ];
        for &timestamp in &timestamps {
            let split = split_timestamp(timestamp);
            let combined = combine_timestamp(&split);
            assert_eq!(timestamp, combined);
        }
    }
}
