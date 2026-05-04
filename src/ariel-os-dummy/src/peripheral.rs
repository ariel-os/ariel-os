#![deny(missing_docs)]

use core::marker::PhantomData;

/// Dummy type.
///
/// See the `OptionalPeripherals` type of your Embassy HAL crate instead.
#[doc(hidden)]
pub struct OptionalPeripherals;

/// This is an adapter trait necessary for compatibility with the underlying HALs.
///
/// The underlying HALs manage peripherals differently: for instance, the drivers from
/// Embassy HALs require a `Peri` instance constrained on the required peripheral ZST, while
/// drivers from `esp-hal` require the peripheral ZST directly.
///
/// In practice, this trait allows passing the peripheral ZSTs [obtained through Ariel OS
/// facilities][obtaining-peripheral-access-book] directly to Ariel OS drivers that have a trait
/// bound on [`IntoPeripheral`].
///
/// Note: It is never necessary to implement this trait outside of Ariel OS, and the trait is
/// therefore sealed.
///
/// [obtaining-peripheral-access-book]: https://ariel-os.github.io/ariel-os/dev/docs/book/application.html#obtaining-peripheral-access
// NOTE: Each Ariel OS HAL needs to provide its own definition of this trait, which should be sealed when possible.
pub trait IntoPeripheral<'a, P>: private::Sealed {
    /// Converts this peripheral instance into the type required by the HAL.
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

/// Dummy type.
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

pub struct Peri<'a, T> {
    _data: PhantomData<&'a T>,
}
