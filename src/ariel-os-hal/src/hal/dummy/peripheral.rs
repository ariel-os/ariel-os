/// Dummy type.
///
/// See the `OptionalPeripherals` type of your Embassy HAL crate instead.
#[doc(hidden)]
pub struct OptionalPeripherals;

#[doc(hidden)]
pub trait IntoPeripheral<'a, P> {
    fn into_hal_peripheral(self) -> Self;
}

#[doc(hidden)]
pub struct Peripheral;

impl<T> IntoPeripheral<'_, T> for Peripheral {
    fn into_hal_peripheral(self) -> Peripheral {
        self
    }
}

/// Dummy type.
#[doc(hidden)]
pub struct Peripherals;

impl From<Peripherals> for OptionalPeripherals {
    fn from(_peripherals: Peripherals) -> Self {
        Self {}
    }
}
