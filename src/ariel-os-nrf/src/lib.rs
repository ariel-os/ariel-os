//! Items specific to the Nordic Semiconductor nRF MCUs.

#![no_std]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(missing_docs)]

pub mod gpio;

mod irqs;

#[doc(hidden)]
pub mod peripheral {
    pub use embassy_nrf::Peripheral;
}

#[cfg(feature = "ble")]
#[doc(hidden)]
pub mod ble;

#[cfg(feature = "external-interrupts")]
#[doc(hidden)]
pub mod extint_registry;

#[cfg(feature = "hwrng")]
#[doc(hidden)]
pub mod hwrng;

#[cfg(feature = "i2c")]
pub mod i2c;

#[doc(hidden)]
pub mod identity;

#[cfg(feature = "spi")]
pub mod spi;

#[cfg(feature = "storage")]
#[doc(hidden)]
pub mod storage;

#[cfg(feature = "usb")]
#[doc(hidden)]
pub mod usb;

#[cfg(feature = "executor-interrupt")]
#[doc(hidden)]
pub use embassy_executor::InterruptExecutor as Executor;

#[cfg(feature = "executor-interrupt")]
#[cfg(context = "nrf51")]
ariel_os_embassy_common::executor_swi!(SWI0);

#[cfg(feature = "executor-interrupt")]
#[cfg(context = "nrf52")]
ariel_os_embassy_common::executor_swi!(EGU0_SWI0);

#[cfg(feature = "executor-interrupt")]
#[cfg(any(context = "nrf53", context = "nrf91"))]
ariel_os_embassy_common::executor_swi!(EGU0);

use embassy_nrf::config::Config;

#[doc(hidden)]
pub use embassy_nrf::{OptionalPeripherals, interrupt};

pub use embassy_nrf::peripherals;

#[cfg(feature = "executor-interrupt")]
#[doc(hidden)]
pub static EXECUTOR: Executor = Executor::new();

#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    enable_flash_cache();

    let peripherals = embassy_nrf::init(Config::default());
    OptionalPeripherals::from(peripherals)
}

fn enable_flash_cache() {
    // (no flash cache on nrf51)
    cfg_if::cfg_if! {
        if #[cfg(any(
                context = "nrf52",
                context = "nrf5340-net",
                context = "nrf91"
            ))] {
            embassy_nrf::pac::NVMC
                .icachecnf()
                .write(|w| w.set_cacheen(true));
        }
        else if #[cfg(context = "nrf5340")] {
            embassy_nrf::pac::CACHE_S
                .enable().write(|w| w.set_enable(true));
        }
    }
}
