/// Operation modes for the GNSS module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssOperationMode {
    /// Always keep the GNSS module active.
    Continuous,
    /// Update the GNSS fix periodically. Period is defined in seconds.
    Periodic(u16),
    /// Try to get a GNSS fix only when requested. Timeout is defined in seconds, 300 recommended.
    SingleShot(u16),
}

impl core::fmt::Display for GnssOperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Configuration for the GNSS sensor.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// The GNSS operating mode to use.
    pub operation_mode: GnssOperationMode,
    /// If NMEA messages should be logged as debug (adds extra processing).
    pub log_nmea: bool,
}

impl Config {
    /// Creates a new `Config` with the specified options.
    #[must_use]
    pub const fn new(operation_mode: GnssOperationMode, log_nmea: bool) -> Self {
        Self {
            operation_mode,
            log_nmea,
        }
    }

    /// Creates a new `Config` with default options.
    #[must_use]
    pub const fn const_default() -> Self {
        Config::new(GnssOperationMode::Continuous, false)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::const_default()
    }
}

impl core::fmt::Display for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

pub(crate) fn convert_gnss_config(config: &Config) -> nrf_modem::GnssConfig {
    nrf_modem::GnssConfig {
        elevation_threshold_angle: 5,
        use_case: nrf_modem::GnssUsecase {
            low_accuracy: false,
            scheduled_downloads_disable: false,
        },
        nmea_mask: nrf_modem::NmeaMask {
            gga: config.log_nmea,
            gll: config.log_nmea,
            gsa: config.log_nmea,
            gsv: config.log_nmea,
            rmc: config.log_nmea,
        },
        timing_source: nrf_modem::GnssTimingSource::Tcxo,
        power_mode: nrf_modem::GnssPowerSaveMode::Disabled,
    }
}
