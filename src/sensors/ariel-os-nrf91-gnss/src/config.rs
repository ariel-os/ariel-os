/// Operation modes for the GNSS module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssOperationMode {
    /// Always keep the GNSS module active.
    Continuous,
    /// Update the GNSS fix periodically. Period is defined in seconds.
    Periodic(u16),
    /// Try to get a GNSS fix only when requested, you won't be able to get updates using `get_receiver`. Timeout is defined in seconds.
    SingleShot(u16),
}

impl core::fmt::Display for GnssOperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Configuration for the GNSS.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// The mode of GNSS to use.
    pub operation_mode: GnssOperationMode,
}

impl Config {
    /// Creates a new `Config` with the specified operation mode.
    #[must_use]
    pub const fn new(operation_mode: GnssOperationMode) -> Self {
        Self { operation_mode }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(GnssOperationMode::Continuous)
    }
}

impl core::fmt::Display for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
