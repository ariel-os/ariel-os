//! Items specific to the Infineon `PSoC` MCUs.

#![no_std]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

pub mod gpio;

#[doc(hidden)]
pub mod peripheral {
    pub use embassy_hal_internal::Peri;
}

#[doc(hidden)]
pub mod identity {
    use ariel_os_embassy_common::identity;

    pub type DeviceId = identity::NoDeviceId<identity::NotImplemented>;
}

use embassy_hal_internal::{Peri, PeripheralType};

#[cfg(feature = "executor-interrupt")]
#[doc(hidden)]
pub use embassy_executor::InterruptExecutor as Executor;

/// A trait implemented by GPIO pin peripheral singletons, exposing the pin's
/// port and pin number to the GPIO driver.
#[doc(hidden)]
pub trait PsocPin: PeripheralType {
    /// The pin's port number.
    const PORT: u8;
    /// The pin's offset within its port.
    const PIN: u8;
}

// Generates the pin peripheral singletons (`peripherals` module), the
// `Peripherals` and `OptionalPeripherals` structs, and the `PsocPin` impls.
macro_rules! psoc_peripherals {
    ($($name:ident => ($port:literal, $pin:literal)),* $(,)?) => {
        embassy_hal_internal::peripherals_definition!($($name),*);
        embassy_hal_internal::peripherals_struct!($($name),*);

        $(
            impl crate::PsocPin for peripherals::$name {
                const PORT: u8 = $port;
                const PIN: u8 = $pin;
            }
        )*

        /// Struct of `Option`s of all the peripheral singletons.
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub struct OptionalPeripherals {
            $(
                #[allow(missing_docs)]
                pub $name: Option<Peri<'static, peripherals::$name>>,
            )*
        }

        impl From<Peripherals> for OptionalPeripherals {
            fn from(p: Peripherals) -> Self {
                Self {
                    $($name: Some(p.$name),)*
                }
            }
        }
    };
}

#[cfg(context = "cy8c624abzi-s2d44")]
psoc_peripherals! {
    P0_0 => (0, 0), P0_1 => (0, 1), P0_2 => (0, 2), P0_3 => (0, 3),
    P0_4 => (0, 4), P0_5 => (0, 5),
    P1_0 => (1, 0), P1_1 => (1, 1), P1_2 => (1, 2), P1_3 => (1, 3),
    P1_4 => (1, 4), P1_5 => (1, 5),
    P2_0 => (2, 0), P2_1 => (2, 1), P2_2 => (2, 2), P2_3 => (2, 3),
    P2_4 => (2, 4), P2_5 => (2, 5), P2_6 => (2, 6), P2_7 => (2, 7),
    P3_0 => (3, 0), P3_1 => (3, 1), P3_2 => (3, 2), P3_3 => (3, 3),
    P3_4 => (3, 4), P3_5 => (3, 5),
    P4_0 => (4, 0), P4_1 => (4, 1),
    P5_0 => (5, 0), P5_1 => (5, 1), P5_2 => (5, 2), P5_3 => (5, 3),
    P5_4 => (5, 4), P5_5 => (5, 5), P5_6 => (5, 6), P5_7 => (5, 7),
    P6_0 => (6, 0), P6_1 => (6, 1), P6_2 => (6, 2), P6_3 => (6, 3),
    P6_4 => (6, 4), P6_5 => (6, 5), P6_6 => (6, 6), P6_7 => (6, 7),
    P7_0 => (7, 0), P7_1 => (7, 1), P7_2 => (7, 2), P7_3 => (7, 3),
    P7_4 => (7, 4), P7_5 => (7, 5), P7_6 => (7, 6), P7_7 => (7, 7),
    P8_0 => (8, 0), P8_1 => (8, 1), P8_2 => (8, 2), P8_3 => (8, 3),
    P8_4 => (8, 4), P8_5 => (8, 5), P8_6 => (8, 6), P8_7 => (8, 7),
    P9_0 => (9, 0), P9_1 => (9, 1), P9_2 => (9, 2), P9_3 => (9, 3),
    P9_4 => (9, 4), P9_5 => (9, 5), P9_6 => (9, 6), P9_7 => (9, 7),
    P10_0 => (10, 0), P10_1 => (10, 1), P10_2 => (10, 2), P10_3 => (10, 3),
    P10_4 => (10, 4), P10_5 => (10, 5), P10_6 => (10, 6), P10_7 => (10, 7),
    P11_0 => (11, 0), P11_1 => (11, 1), P11_2 => (11, 2), P11_3 => (11, 3),
    P11_4 => (11, 4), P11_5 => (11, 5), P11_6 => (11, 6), P11_7 => (11, 7),
    P12_0 => (12, 0), P12_1 => (12, 1), P12_2 => (12, 2), P12_3 => (12, 3),
    P12_4 => (12, 4), P12_5 => (12, 5), P12_6 => (12, 6), P12_7 => (12, 7),
    P13_0 => (13, 0), P13_1 => (13, 1), P13_2 => (13, 2), P13_3 => (13, 3),
    P13_4 => (13, 4), P13_5 => (13, 5), P13_6 => (13, 6), P13_7 => (13, 7),
    P14_0 => (14, 0), P14_1 => (14, 1),
}

#[doc(hidden)]
pub trait IntoPeripheral<'a, T: PeripheralType>: private::Sealed {
    fn into_hal_peripheral(self) -> Peri<'a, T>;
}

impl<T: PeripheralType> private::Sealed for Peri<'_, T> {}

impl<'a, T: PeripheralType> IntoPeripheral<'a, T> for Peri<'a, T> {
    fn into_hal_peripheral(self) -> Peri<'a, T> {
        self
    }
}

mod private {
    pub trait Sealed {}
}

#[expect(unsafe_code)]
#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    let device = psoc::device::Device::take();

    let mut clock = device.clock;
    clock.configure(&psoc::sys::clock::ClockConfig::DEFAULT, None);

    #[cfg(feature = "time")]
    psoc::sys::embassy_time::init(device.mcwdt.1);

    // SAFETY: `Device::take()` enforces this runs at most once, so the peripheral
    // singletons are likewise handed out at most once.
    OptionalPeripherals::from(unsafe { Peripherals::steal() })
}
