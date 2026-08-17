#![deny(missing_docs)]

#[doc(hidden)]
pub struct OptionalPeripherals;

#[doc(hidden)]
pub trait IntoPeripheral<'a, P>: private::Sealed {
    #[must_use]
    fn into_hal_peripheral(self) -> Self;
}

#[doc(hidden)]
pub struct Peripheral;

impl private::Sealed for Peripheral {}

impl<T> IntoPeripheral<'_, T> for Peripheral {
    fn into_hal_peripheral(self) -> Peripheral {
        self
    }
}

#[doc(hidden)]
pub struct Peripherals;

impl From<Peripherals> for OptionalPeripherals {
    fn from(_peripherals: Peripherals) -> Self {
        Self {}
    }
}

mod private {
    pub trait Sealed {}
}
