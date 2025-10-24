//! Items specific to the Espressif ESP MCUs.

#![no_std]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(missing_docs)]

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(all(feature = "threading", feature = "wifi"))]
mod preempt;

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

    #[cfg(any(feature = "hwrng", feature = "wifi-esp"))]
    let rng = esp_hal::rng::Rng::new(peripherals.RNG.take().unwrap());

    #[cfg(feature = "hwrng")]
    ariel_os_random::construct_rng(rng);

    #[cfg(feature = "wifi-esp")]
    {
        use esp_hal::timer::timg::TimerGroup;

        ariel_os_debug::log::debug!("ariel-os-embassy::hal::esp::init(): wifi");

        let timer = TimerGroup::new(peripherals.TIMG0.take().unwrap()).timer0;

        let init = esp_wifi::init(timer, rng, peripherals.RADIO_CLK.take().unwrap()).unwrap();

        wifi::esp_wifi::WIFI_INIT.set(init).unwrap();
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
