use super::{ChannelsSamplesZip, Reading, ReadingChannel, Sample, Sensor};

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
    samples: InnerSamples,
    sensor: &'static dyn Sensor,
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
            InnerSamples::V1(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-2")]
            InnerSamples::V2(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-3")]
            InnerSamples::V3(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-4")]
            InnerSamples::V4(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-5")]
            InnerSamples::V5(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-6")]
            InnerSamples::V6(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-7")]
            InnerSamples::V7(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-8")]
            InnerSamples::V8(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-9")]
            InnerSamples::V9(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-10")]
            InnerSamples::V10(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-11")]
            InnerSamples::V11(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
            #[cfg(feature = "max-sample-min-count-12")]
            InnerSamples::V12(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
        }
    }

    fn samples(
        &self,
    ) -> impl ExactSizeIterator<Item = (ReadingChannel, Sample)> + core::iter::FusedIterator {
        let reading_channels = self.sensor.reading_channels();
        ChannelsSamplesZip::new(reading_channels, self.samples)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum InnerSamples {
    V1([Sample; 1]),
    #[cfg(feature = "max-sample-min-count-2")]
    V2([Sample; 2]),
    #[cfg(feature = "max-sample-min-count-3")]
    V3([Sample; 3]),
    #[cfg(feature = "max-sample-min-count-4")]
    V4([Sample; 4]),
    #[cfg(feature = "max-sample-min-count-5")]
    V5([Sample; 5]),
    #[cfg(feature = "max-sample-min-count-6")]
    V6([Sample; 6]),
    #[cfg(feature = "max-sample-min-count-7")]
    V7([Sample; 7]),
    #[cfg(feature = "max-sample-min-count-8")]
    V8([Sample; 8]),
    #[cfg(feature = "max-sample-min-count-9")]
    V9([Sample; 9]),
    #[cfg(feature = "max-sample-min-count-10")]
    V10([Sample; 10]),
    #[cfg(feature = "max-sample-min-count-11")]
    V11([Sample; 11]),
    #[cfg(feature = "max-sample-min-count-12")]
    V12([Sample; 12]),
}

impl InnerSamples {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Sample> + core::iter::FusedIterator + '_ {
        match self {
            InnerSamples::V1(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-2")]
            InnerSamples::V2(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-3")]
            InnerSamples::V3(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-4")]
            InnerSamples::V4(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-5")]
            InnerSamples::V5(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-6")]
            InnerSamples::V6(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-7")]
            InnerSamples::V7(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-8")]
            InnerSamples::V8(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-9")]
            InnerSamples::V9(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-10")]
            InnerSamples::V10(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-11")]
            InnerSamples::V11(samples) => samples.iter().copied(),
            #[cfg(feature = "max-sample-min-count-12")]
            InnerSamples::V12(samples) => samples.iter().copied(),
        }
    }
}
