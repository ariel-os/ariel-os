pub use embassy_hal_internal::Peripheral;

/// Dummy type.
///
/// See the `OptionalPeripherals` type of your Embassy HAL crate instead.
#[doc(hidden)]
pub struct OptionalPeripherals;

/// Dummy type.
#[doc(hidden)]
pub struct Peripherals;

impl From<Peripherals> for OptionalPeripherals {
    fn from(_peripherals: Peripherals) -> Self {
        Self {}
    }
}
