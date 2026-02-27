/// Label of a [`Sample`](crate::sensor::Sample) part of a
/// [`Samples`](crate::sensor::Samples) tuple.
///
/// # For sensor driver implementors
///
/// Missing variants can be added when required.
/// Please open an issue to discuss it.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Label {
    /// Acceleration along the X axis.
    AccelerationX,
    /// Acceleration along the Y axis.
    AccelerationY,
    /// Acceleration along the Z axis.
    AccelerationZ,
    /// Altitude.
    Altitude,
    /// Angular velocity about the X axis.
    AngularVelocityX,
    /// Angular velocity about the Y axis.
    AngularVelocityY,
    /// Angular velocity about the Z axis.
    AngularVelocityZ,
    /// CO<sub>2</sub> concentration.
    Co2,
    /// Ground speed.
    GroundSpeed,
    /// Latitude.
    Latitude,
    /// Longitude.
    Longitude,
    /// Opaque channel: the associated sample is intended for the sensor driver only, and no guarantees are provided.
    Opaque,
    /// Opaque channel marker used by `GnssTimeExt`.
    OpaqueGnssTime,
    /// Pressure.
    Pressure,
    /// Relative humidity.
    RelativeHumidity,
    /// Heading.
    Heading,
    /// Temperature.
    Temperature,
    /// Vertical speed.
    VerticalSpeed,
    /// X axis.
    X,
    /// Y axis.
    Y,
    /// Z axis.
    Z,
}

impl core::fmt::Display for Label {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AccelerationX => write!(f, "Acceleration X"),
            Self::AccelerationY => write!(f, "Acceleration Y"),
            Self::AccelerationZ => write!(f, "Acceleration Z"),
            Self::Altitude => write!(f, "Altitude"),
            Self::AngularVelocityX => write!(f, "Angular velocity X"),
            Self::AngularVelocityY => write!(f, "Angular velocity Y"),
            Self::AngularVelocityZ => write!(f, "Angular velocity Z"),
            Self::Co2 => write!(f, "CO2 concentration"),
            Self::GroundSpeed => write!(f, "Ground speed"),
            Self::Latitude => write!(f, "Latitude"),
            Self::Longitude => write!(f, "Longitude"),
            Self::Opaque | Self::OpaqueGnssTime => write!(f, "[opaque]"),
            Self::Pressure => write!(f, "Pressure"),
            Self::RelativeHumidity => write!(f, "Relative humidity"),
            Self::Heading => write!(f, "Heading"),
            Self::Temperature => write!(f, "Temperature"),
            Self::VerticalSpeed => write!(f, "Vertical speed"),
            Self::X => write!(f, "X"),
            Self::Y => write!(f, "Y"),
            Self::Z => write!(f, "Z"),
        }
    }
}
