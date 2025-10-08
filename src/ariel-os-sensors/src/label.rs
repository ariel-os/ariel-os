/// Label of a [`Sample`](crate::sensor::Sample) part of a
/// [`Samples`](crate::sensor::Samples) tuple.
///
/// # For sensor driver implementors
///
/// Missing variants can be added when required.
/// Please open an issue to discuss it.
///
/// [`Label::Main`] must be used for sensor drivers returning a single
/// [`Sample`](crate::sensor::Sample), even if a more specific label exists for the
/// physical quantity.
/// This allows consumers displaying the label to ignore it for sensor drivers returning a single
/// [`Sample`](crate::sensor::Sample).
/// Other labels are reserved for sensor drivers returning multiple physical quantities.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum Label {
    /// Altitude.
    Altitude,
    /// Ground speed.
    GroundSpeed,
    /// Latitude.
    Latitude,
    /// Longitude.
    Longitude,
    /// Used for sensor drivers returning a single [`Sample`](crate::sensor::Sample).
    Main,
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
            Self::Altitude => write!(f, "Altitude"),
            Self::GroundSpeed => write!(f, "Ground speed"),
            Self::Latitude => write!(f, "Latitude"),
            Self::Longitude => write!(f, "Longitude"),
            Self::Main => write!(f, ""),
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
