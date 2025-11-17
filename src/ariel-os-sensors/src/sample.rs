use crate::sensor::ReadingChannel;

/// Errors returned when trying to interpret a sample.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum SampleError {
    /// The channel is not available at the moment (e.g., part of the sensor is not ready yet).
    TemporarilyUnavailable,
    /// The channel is disabled by configuration.
    ChannelDisabled,
}

#[expect(clippy::doc_markdown)]
/// Represents a value obtained from a sensor device, along with its metadata.
///
/// # Scaling
///
/// The [scaling value](crate::sensor::ReadingChannel::scaling()) obtained from the sensor driver with
/// [`Sensor::reading_channels()`](crate::Sensor::reading_channels) must be taken into account using the
/// following formula:
///
/// <math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><mrow><mi mathvariant="monospace">Sample::value()</mi></mrow><mo>·</mo><msup><mn>10</mn><mrow><mi mathvariant="monospace">scaling</mi></mrow></msup></math>
///
/// For instance, in the case of a temperature sensor, if [`Self::value()`] returns `2225` and the
/// scaling value is `-2`, this means that the temperature measured and returned by the sensor
/// device is `22.25` (the [measurement error](SampleMetadata) must additionally be taken into
/// account).
/// This is required to avoid handling floats.
///
/// # Unit of measurement
///
/// The unit of measurement can be obtained using
/// [`ReadingChannel::unit()`](crate::sensor::ReadingChannel::unit).
///
/// # Accuracy
///
/// The accuracy can be obtained through [`Self::metadata()`].
// NOTE(derive): we do not implement `Eq` or `PartialOrd` on purpose: `Eq` would prevent us from
// possibly adding floats in the future and `PartialOrd` does not make sense because interpreting
// the sample requires the `ReadingChannel` associated with this `Sample`.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Sample {
    value: i32,
    metadata: SampleMetadata,
}

impl Sample {
    /// Creates a new sample.
    ///
    /// This constructor is intended for sensor driver implementors only.
    #[must_use]
    pub const fn new(value: i32, metadata: SampleMetadata) -> Self {
        Self { value, metadata }
    }

    /// Returns the sample value.
    ///
    /// # Errors
    ///
    /// [`SampleError`] is returned in case of error.
    pub fn value(&self) -> Result<i32, SampleError> {
        match self.metadata {
            SampleMetadata::ChannelTemporarilyUnavailable => {
                Err(SampleError::TemporarilyUnavailable)
            }
            SampleMetadata::ChannelDisabled => Err(SampleError::ChannelDisabled),
            SampleMetadata::NoMeasurementError
            | SampleMetadata::UnknownAccuracy
            | SampleMetadata::SymmetricalError { .. } => Ok(self.value),
        }
    }

    /// Returns the measurement metadata, including accuracy if available.
    #[must_use]
    pub fn metadata(&self) -> SampleMetadata {
        self.metadata
    }
}

/// Metadata associated with a [`Sample`].
///
/// Includes the measurement accuracy if available.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SampleMetadata {
    /// Unknown accuracy.
    UnknownAccuracy,
    /// No observational/measurement error (e.g., boolean values from a push button).
    NoMeasurementError,
    /// Measurement error symmetrical around the [`bias`](SampleMetadata::SymmetricalError::bias).
    ///
    /// The unit of measurement is provided by the [`ReadingChannel`](crate::sensor::ReadingChannel)
    /// associated with the [`Sample`].
    /// The `scaling` value is used for both `deviation` and `bias`.
    /// The accuracy error is thus given by the following formulas:
    ///
    /// <math xmlns="http://www.w3.org/1998/Math/MathML" display="block"><mo>+</mo><mo>(</mo><mrow><mi mathvariant="monospace">bias</mi></mrow><mo>+</mo><mrow><mi mathvariant="monospace">deviation</mi></mrow><mo>)</mo><mo>·</mo><msup><mn>10</mn><mrow><mi mathvariant="monospace">scaling</mi></mrow></msup>/<mo>-</mo><mo>(</mo><mrow><mi mathvariant="monospace">bias</mi></mrow><mo>-</mo><mrow><mi mathvariant="monospace">deviation</mi></mrow><mo>)</mo><mo>·</mo><msup><mn>10</mn><mrow><mi mathvariant="monospace">scaling</mi></mrow></msup></math>
    ///
    /// # Examples
    ///
    /// The DS18B20 temperature sensor accuracy error is <mo>+</mo><mn>0.05</mn>/<mo>-</mo><mn>0.45</mn>
    /// at 20 °C (see Figure 1 of its datasheet).
    /// [`SampleMetadata`] would thus be the following:
    ///
    /// ```
    /// # use ariel_os_sensors::sensor::SampleMetadata;
    /// SampleMetadata::SymmetricalError {
    ///     deviation: 25,
    ///     bias: -20,
    ///     scaling: -2,
    /// }
    /// # ;
    /// ```
    SymmetricalError {
        /// Deviation around the bias value.
        deviation: u8,
        /// Bias (mean accuracy error).
        bias: i8,
        /// Scaling of [`deviation`](SampleMetadata::SymmetricalError::deviation) and
        /// [`bias`](SampleMetadata::SymmetricalError::bias).
        scaling: i8,
    },
    /// Indicates that the channel is temporarily unavailable.
    /// This may happen when parts of a sensor are not ready yet, but data is already available for other channels.
    ChannelTemporarilyUnavailable,
    /// The channel is disabled by configuration.
    ChannelDisabled,
}

/// Implemented on [`Samples`](crate::sensor::Samples), returned by
/// [`Sensor::wait_for_reading()`](crate::Sensor::wait_for_reading).
pub trait Reading: core::fmt::Debug {
    /// Returns the first value returned by [`Reading::samples()`].
    fn sample(&self) -> (ReadingChannel, Sample);

    /// Returns an iterator over [`Sample`]s of a sensor reading.
    ///
    /// The order of [`Sample`]s is not significant, but is fixed.
    ///
    /// # For implementors
    ///
    /// The default implementation must be overridden on types containing multiple
    /// [`Sample`]s.
    fn samples(&self) -> impl ExactSizeIterator<Item = (ReadingChannel, Sample)> {
        [self.sample()].into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_type_sizes() {
        assert!(size_of::<SampleMetadata>() <= size_of::<u32>());
        // Make sure the type is small enough.
        assert!(size_of::<Sample>() <= 2 * size_of::<u32>());
    }
}
