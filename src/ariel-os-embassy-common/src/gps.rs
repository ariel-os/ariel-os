//! Common types for GPS functionality across different HALs.
use defmt::Format;

/// When to request a GPS fix.
#[derive(Debug, Clone, Copy, Format)]
pub enum GpsFixMode {
    /// Always keep the GPS fix active.
    Continuous,
    /// Update the GPS fix periodically.
    Periodic(u16), // Period in seconds
    /// Ttry to get a GPS fix only when requested, the stream functionnality won't work.
    SingleShot,
}

/// Configuration for the GPS.
///
/// You can customize it using the `gps-config-override` feature.
#[derive(Debug, Clone, Copy, Format)]
pub struct Config {
    /// The mode of GPS to use.
    pub mode: GpsFixMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: GpsFixMode::Continuous,
        }
    }
}

/// Represents position data from GPS.
#[derive(Debug, Clone, Format)]
pub struct GpsPosition {
    /// Latitude in degrees
    /// Positive values indicate north, negative values indicate south.
    pub latitude: f64,
    /// Longitude in degrees
    /// Positive values indicate east, negative values indicate west.
    pub longitude: f64,
    /// Altitude in meters above sea level
    pub altitude: f64,
    /// Accuracy of the position in meters
    pub accuracy: f64,
    /// Altitude accuracy in meters
    pub altitude_accuracy: f64,
}

/// Represents velocity data from GPS.
#[derive(Debug, Clone, Format)]
pub struct GpsVelocity {
    /// Speed in m/s
    pub speed: f64,
    /// Speed accuracy in m/s
    pub speed_accuracy: f64,

    /// Vertical speed in m/s
    pub vertical_speed: f64,
    /// Vertical speed accuracy in m/s
    pub vertical_speed_accuracy: f64,
    /// Heading in degrees (0-360)
    pub heading: f64,
    /// Heading accuracy in degrees
    pub heading_accuracy: f64,
}

// Based on NMEA RMC message
/// Represents date and time information from GPS.
#[derive(Debug, Clone, Format)]
pub struct GpsDateTime {
    /// Year
    pub year: u16,
    /// Month (1-12)
    pub month: u8,
    /// Day of the month (1-31)
    pub day: u8,
    /// Hour (0-23)
    pub hour: u8,
    /// Minute (0-59)
    pub minute: u8,
    /// Second (0-59)
    pub second: u8,
    /// Milliseconds (0-999)
    pub milliseconds: u16,
}

/// Represents GPS data that can be received from the GNSS module.
#[derive(Debug, Clone, Format)]
pub struct GpsData {
    /// The position data, if available.
    pub position: Option<GpsPosition>,
    /// The velocity data, if available.
    pub velocity: Option<GpsVelocity>,
    /// The date and time information, if available.
    pub datetime: Option<GpsDateTime>,
    /// The time when the data was recorded (from start of the MCU).
    pub recorded_at: embassy_time::Instant,
}
