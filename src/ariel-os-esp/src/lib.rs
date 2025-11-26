//! Items specific to the Espressif ESP MCUs.

#![no_std]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

esp_bootloader_esp_idf::esp_app_desc!();

pub mod gpio;

#[cfg(feature = "hwrng")]
#[doc(hidden)]
pub mod hwrng {
    pub fn construct_rng(_peripherals: &mut crate::OptionalPeripherals) {
        // handled in `init()`
    }
}

#[cfg(feature = "i2c")]
pub mod i2c;

#[doc(hidden)]
pub mod identity {
    use ariel_os_embassy_common::identity;

    pub type DeviceId = identity::NoDeviceId<identity::NotImplemented>;
}

#[cfg(feature = "spi")]
pub mod spi;

#[cfg(feature = "uart")]
pub mod uart;

#[cfg(feature = "usb")]
#[doc(hidden)]
pub mod usb;

#[cfg(feature = "wifi")]
#[doc(hidden)]
pub mod wifi;

#[doc(hidden)]
pub mod peripheral {}

#[doc(hidden)]
pub mod peripherals {
    pub use esp_hal::peripherals::*;
}

#[cfg(feature = "time")]
mod time_driver;

#[doc(hidden)]
pub use esp_hal::peripherals::OptionalPeripherals;

// TODO(bump):
// - use this for all peripheral types (spi/i2c/uart) if needed
#[doc(hidden)]
pub trait IntoPeripheral<'a, T: 'a> {
    fn into_hal_peripheral(self) -> T;
}

#[doc(hidden)]
impl<'a, T: 'a> IntoPeripheral<'a, T> for T {
    fn into_hal_peripheral(self) -> T {
        self
    }
}

#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    let config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max());

    let mut peripherals = OptionalPeripherals::from(esp_hal::init(config));

    #[cfg(feature = "hwrng")]
    {
        let mut rng = esp_hal::rng::Rng::new();
        ariel_os_random::construct_rng(&mut ariel_os_random::RngAdapter(&mut rng));
    }

    let embassy_timer = {
        cfg_if::cfg_if! {
            if #[cfg(context = "esp32")] {
                use esp_hal::timer::timg::TimerGroup;
                TimerGroup::new(peripherals.TIMG1.take().unwrap()).timer0
            } else {
                use esp_hal::timer::systimer::{SystemTimer};
                SystemTimer::new(peripherals.SYSTIMER.take().unwrap()).alarm0
            }
        }
    };

    crate::time_driver::init(embassy_timer);

    peripherals
}

#[cfg(feature = "time")]
embassy_time_driver::time_driver_impl!(static TIMER_QUEUE: crate::time_driver::TimerQueue = crate::time_driver::TimerQueue::new());
