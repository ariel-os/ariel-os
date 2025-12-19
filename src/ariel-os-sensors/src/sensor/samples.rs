use super::{InnerSamples, Reading, Sample, Sensor};

/// Provides access to the sensor driver instance.
/// For driver implementors only.
pub trait SensorAccess: private::Sealed {
    /// Returns the sensor driver instance that produced these samples.
    /// For driver implementors only.
    fn sensor(&self) -> &'static dyn Sensor;
}

/// Avoid external implementations of [`SensorAccess`].
mod private {
    use super::Samples;
    pub trait Sealed {}

    impl Sealed for Samples {}
}

/// Samples returned by a sensor driver.
///
/// This type implements [`Reading`] to iterate over the samples.
///
/// # Note
///
/// This type is automatically generated, the number of [`Sample`]s that can be stored is
/// automatically adjusted.
#[derive(Copy, Clone)]
pub struct Samples {
    pub(super) samples: InnerSamples,
    pub(super) sensor: &'static dyn Sensor,
}

impl core::fmt::Debug for Samples {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Samples")
            .field("samples", &self.samples)
            .field("sensor", &"&dyn Sensor")
            .finish()
    }
}

impl SensorAccess for Samples {
    fn sensor(&self) -> &'static dyn Sensor {
        self.sensor
    }
}

impl Samples {
    /// Creates a new [`Samples`] containing 1 sample.
    pub fn from_1(sensor: &'static dyn Sensor, samples: [Sample; 1]) -> Self {
        Self {
            samples: InnerSamples::V1(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 2 samples.
    #[cfg(feature = "max-sample-min-count-2")]
    pub fn from_2(sensor: &'static dyn Sensor, samples: [Sample; 2]) -> Self {
        Self {
            samples: InnerSamples::V2(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 3 samples.
    #[cfg(feature = "max-sample-min-count-3")]
    pub fn from_3(sensor: &'static dyn Sensor, samples: [Sample; 3]) -> Self {
        Self {
            samples: InnerSamples::V3(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 4 samples.
    #[cfg(feature = "max-sample-min-count-4")]
    pub fn from_4(sensor: &'static dyn Sensor, samples: [Sample; 4]) -> Self {
        Self {
            samples: InnerSamples::V4(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 5 samples.
    #[cfg(feature = "max-sample-min-count-5")]
    pub fn from_5(sensor: &'static dyn Sensor, samples: [Sample; 5]) -> Self {
        Self {
            samples: InnerSamples::V5(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 6 samples.
    #[cfg(feature = "max-sample-min-count-6")]
    pub fn from_6(sensor: &'static dyn Sensor, samples: [Sample; 6]) -> Self {
        Self {
            samples: InnerSamples::V6(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 7 samples.
    #[cfg(feature = "max-sample-min-count-7")]
    pub fn from_7(sensor: &'static dyn Sensor, samples: [Sample; 7]) -> Self {
        Self {
            samples: InnerSamples::V7(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 8 samples.
    #[cfg(feature = "max-sample-min-count-8")]
    pub fn from_8(sensor: &'static dyn Sensor, samples: [Sample; 8]) -> Self {
        Self {
            samples: InnerSamples::V8(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 9 samples.
    #[cfg(feature = "max-sample-min-count-9")]
    pub fn from_9(sensor: &'static dyn Sensor, samples: [Sample; 9]) -> Self {
        Self {
            samples: InnerSamples::V9(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 10 samples.
    #[cfg(feature = "max-sample-min-count-10")]
    pub fn from_10(sensor: &'static dyn Sensor, samples: [Sample; 10]) -> Self {
        Self {
            samples: InnerSamples::V10(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 11 samples.
    #[cfg(feature = "max-sample-min-count-11")]
    pub fn from_11(sensor: &'static dyn Sensor, samples: [Sample; 11]) -> Self {
        Self {
            samples: InnerSamples::V11(samples),
            sensor,
        }
    }

    /// Creates a new [`Samples`] containing 12 samples.
    #[cfg(feature = "max-sample-min-count-12")]
    pub fn from_12(sensor: &'static dyn Sensor, samples: [Sample; 12]) -> Self {
        Self {
            samples: InnerSamples::V12(samples),
            sensor,
        }
    }
}

impl Reading for Samples {
    fn sample(&self) -> (ReadingChannel, Sample) {
        match self.samples {
            #(#samples_first_sample),*
        }
    }

    fn samples(&self) -> impl ExactSizeIterator<Item = (ReadingChannel, Sample)> + core::iter::FusedIterator {
        let reading_channels = self.sensor.reading_channels();
        ChannelsSamplesZip::new(reading_channels, self.samples)
    }
}
