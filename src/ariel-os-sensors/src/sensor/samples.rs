use super::{InnerSamples, Sensor};

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
