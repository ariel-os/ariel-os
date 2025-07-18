//! Common types for GPS functionality across different HALs.
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, watch::Watch};

/// Maximum number of concurrent receivers for GPS data.
pub const MAX_WATCH_RECEIVERS: usize = 4;
/// Embassy Watch to hold the latest GPS data and notify receivers.
pub type GpsDataWatch = Watch<CriticalSectionRawMutex, GpsData, MAX_WATCH_RECEIVERS>;
/// Type alias for the receiver of GPS data (receiver of `GpsDataWatch`).
pub type GpsDataReceiver<'a> =
    embassy_sync::watch::Receiver<'a, CriticalSectionRawMutex, GpsData, MAX_WATCH_RECEIVERS>;
/// Type alias for the sender of GPS data (sender of `GpsDataWatch`).
#[doc(hidden)]
pub type GpsDataSender<'a> =
    embassy_sync::watch::Sender<'a, CriticalSectionRawMutex, GpsData, MAX_WATCH_RECEIVERS>;

/// When to request a GPS fix.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpsFixMode {
    /// Always keep the GPS fix active.
    Continuous,
    /// Update the GPS fix periodically.
    Periodic(u16), // Period in seconds
    /// Ttry to get a GPS fix only when requested, the stream functionality won't work.
    SingleShot(u16), // Timeout in seconds
}

/// Configuration for the GPS.
///
/// You can customize it using the `gps-config-override` feature.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
#[derive(Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
