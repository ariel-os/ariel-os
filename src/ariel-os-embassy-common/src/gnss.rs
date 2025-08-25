//! Common types for GNSS functionality across different HALs.
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    watch::{Receiver, Sender, Watch},
};

/// Maximum number of concurrent receivers for GNSS data.
pub const MAX_WATCH_RECEIVERS: usize = 4;
/// Embassy Watch to hold the latest GNSS data and notify receivers.
pub type GnssDataWatch = Watch<CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;
/// Type alias for the receiver of GNSS data (receiver of `GnssDataWatch`).
pub type GnssDataReceiver<'a> =
    Receiver<'a, CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;
/// Type alias for the sender of GNSS data (sender of `GnssDataWatch`).
#[doc(hidden)]
pub type GnssDataSender<'a> = Sender<'a, CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;

/// Operation modes for the GNSS module.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssOperationMode {
    /// Always keep the GNSS module active.
    Continuous,
    /// Update the GNSS fix periodically.
    Periodic(u16), // Period in seconds
    /// Try to get a GNSS fix only when requested, you won't be able to get updates using `get_receiver`.
    SingleShot(u16), // Timeout in seconds
}

/// Configuration for the GNSS.
///
/// You can customize it using the `gnss-config-override` feature.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// The mode of GNSS to use.
    pub operation_mode: GnssOperationMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            operation_mode: GnssOperationMode::Continuous,
        }
    }
}

/// Represents position data from GNSS.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssPosition {
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

/// Represents velocity data from GNSS.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssVelocity {
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
/// Represents date and time information from GNSS.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssDateTime {
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

/// Represents GNSS data that can be received from the GNSS module.
///
/// A field can be `None` if the GNSS module did not provide that information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssData {
    /// The position data, if available.
    pub position: Option<GnssPosition>,
    /// The velocity data, if available.
    pub velocity: Option<GnssVelocity>,
    /// The date and time information, if available.
    pub datetime: Option<GnssDateTime>,
    /// The time when the data was recorded (from start of the MCU).
    pub recorded_at: embassy_time::Instant,
}
