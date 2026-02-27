//! Items specific to the Espressif ESP MCUs.

#![no_std]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

#[cfg(any(feature = "ble-esp", feature = "wifi"))]
extern crate alloc;

mod app_desc {
    esp_bootloader_esp_idf::esp_app_desc!();
}

#[cfg(any(feature = "ble-esp", feature = "wifi"))]
mod radio;
#[cfg(any(feature = "ble-esp", feature = "wifi"))]
mod scheduler;
#[cfg(any(feature = "ble-esp", feature = "wifi"))]
mod semaphore;
#[cfg(any(feature = "ble-esp", feature = "wifi"))]
mod wait_queue;

pub mod gpio;

#[cfg(feature = "ble-esp")]
#[doc(hidden)]
pub mod ble;

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

pub mod peripherals {
    //! Types for the peripheral singletons.

    pub use esp_hal::peripherals::*;
}

#[cfg(feature = "time")]
mod time_driver;

#[doc(hidden)]
pub use esp_hal::peripherals::OptionalPeripherals;

#[doc(hidden)]
pub trait IntoPeripheral<'a, T> {
    fn into_hal_peripheral(self) -> T;
}

#[doc(hidden)]
impl<T> IntoPeripheral<'_, T> for T {
    fn into_hal_peripheral(self) -> T {
        self
    }
}

#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    let config = esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max());

    #[allow(unused_mut, reason = "mut only needed for some features")]
    let mut peripherals = OptionalPeripherals::from(esp_hal::init(config));

    #[cfg(feature = "hwrng")]
    {
        ariel_os_debug::log::debug!("initializing hwrng");
        let mut rng = esp_hal::rng::Rng::new();
        ariel_os_random::construct_rng(&mut ariel_os_random::RngAdapter(&mut rng));
    }

    #[cfg(feature = "time")]
    {
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
    }

    peripherals
}

#[cfg(feature = "time")]
embassy_time_driver::time_driver_impl!(static TIMER_QUEUE: crate::time_driver::TimerQueue = crate::time_driver::TimerQueue::new());
