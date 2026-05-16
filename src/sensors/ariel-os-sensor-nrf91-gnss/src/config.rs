pub use nrf_modem::GnssPowerSaveMode;

/// Operation modes for the GNSS module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssOperationMode {
    /// Always keep the GNSS module active.
    Continuous,
    /// Update the GNSS fix periodically.
    /// The period is defined in seconds and should be in the range 10..=65535.
    Periodic(u16),
    /// Try to get a GNSS fix only when requested. Timeout is defined in seconds, 300 recommended.
    SingleShot(u16),
}

/// Configuration for the GNSS sensor.
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub struct Config {
    /// The GNSS operating mode to use.
    pub operation_mode: GnssOperationMode,
    /// Whether NMEA messages should be logged as logs (adds extra processing).
    pub log_nmea: bool,
    /// GNSS power saving mode.
    /// Power consumption: `GnssPowerSaveMode::Disabled` > `GnssPowerSaveMode::DutyCyclingPerformance` > `GnssPowerSaveMode::DutyCycling`.
    pub power_mode: GnssPowerSaveMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            operation_mode: GnssOperationMode::Continuous,
            log_nmea: false,
            power_mode: GnssPowerSaveMode::Disabled,
        }
    }
}

pub(crate) fn convert_gnss_config(config: &Config) -> nrf_modem::GnssConfig {
    nrf_modem::GnssConfig {
        // Satellites with elevation below that angle are not used.
        // Default value in nrfxlib is 5 degrees.
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
        // Tcxo offers more precise 1PPS but uses more energy, we don't use 1PPS so Rtc makes more sense.
        timing_source: nrf_modem::GnssTimingSource::Rtc,
        power_mode: config.power_mode.clone(),
    }
}
