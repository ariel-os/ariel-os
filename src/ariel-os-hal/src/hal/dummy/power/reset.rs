//! Provides facilities related to the MCU reset.

/// Indicates why the microcontroller has reset.
///
/// # Note
///
/// This is a dummy type, HALs provide additional variants specific to their microcontrollers.
// NOTE: Marking this as `non_exhaustive` allows to make introducing *new* variants not a breaking
// change, especially on unaffected MCUs. However, returning the newly introduced variant
// instead of an already existing one is still likely to be one.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ResetReason {
    /// The reset has been triggered by a power cycle.
    /// This variant also acts as a default when the reset reason cannot be determined or
    /// distinguished from a power-on reset.
    /// In particular, many microcontrollers do not allow distinguishing brownout resets from
    /// power-on resets.
    #[default]
    PowerOnReset,
}
