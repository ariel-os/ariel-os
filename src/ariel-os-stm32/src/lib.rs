//! Items specific to the STMicroelectronics STM32 MCUs.

#![no_std]
#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(feature = "rcc-config-override", expect(unsafe_code))]
#![deny(missing_docs)]

mod rcc;

pub mod gpio;

#[doc(hidden)]
pub mod peripheral {
    pub use embassy_stm32::Peri;
}

#[cfg(feature = "external-interrupts")]
#[doc(hidden)]
pub mod extint_registry;

#[cfg(feature = "i2c")]
pub mod i2c;

#[doc(hidden)]
pub mod identity;

#[cfg(feature = "spi")]
pub mod spi;

#[cfg(feature = "uart")]
pub mod uart;

#[cfg(feature = "storage")]
#[doc(hidden)]
pub mod storage;

#[cfg(feature = "usb")]
#[doc(hidden)]
pub mod usb;

#[cfg(feature = "ethernet")]
#[doc(hidden)]
pub mod ethernet;

use embassy_stm32::Config;

#[doc(hidden)]
pub use embassy_stm32::{OptionalPeripherals, Peri, PeripheralType, Peripherals, interrupt};

pub use embassy_stm32::peripherals;

#[cfg(feature = "executor-interrupt")]
pub(crate) use embassy_executor::InterruptExecutor as Executor;

#[cfg(feature = "hwrng")]
#[doc(hidden)]
pub mod hwrng;

#[cfg(feature = "executor-interrupt")]
include!(concat!(env!("OUT_DIR"), "/swi.rs"));

#[cfg(capability = "hw/stm32-dual-core")]
use {core::mem::MaybeUninit, embassy_stm32::SharedData};

// Ariel OS doesn't support the second core yet, but upstream needs this.
#[cfg(capability = "hw/stm32-dual-core")]
static SHARED_DATA: MaybeUninit<SharedData> = MaybeUninit::uninit();

#[cfg(feature = "executor-interrupt")]
#[doc(hidden)]
pub static EXECUTOR: Executor = Executor::new();

#[doc(hidden)]
pub trait IntoPeripheral<'a, T: PeripheralType>: private::Sealed {
    fn into_hal_peripheral(self) -> Peri<'a, T>;
}

impl<T: PeripheralType> private::Sealed for Peri<'_, T> {}

#[doc(hidden)]
impl<'a, T: PeripheralType> IntoPeripheral<'a, T> for Peri<'a, T> {
    fn into_hal_peripheral(self) -> Peri<'a, T> {
        self
    }
}

mod private {
    pub trait Sealed {}
}

#[doc(hidden)]
#[must_use]
pub fn init() -> OptionalPeripherals {
    let mut config = Config::default();
    board_config(&mut config);

    #[cfg(not(capability = "hw/stm32-dual-core"))]
    let peripherals = embassy_stm32::init(config);

    #[cfg(capability = "hw/stm32-dual-core")]
    let peripherals = embassy_stm32::init_primary(config, &SHARED_DATA);

    enable_flash_cache();

    OptionalPeripherals::from(peripherals)
}

fn board_config(config: &mut Config) {
    config.rcc = rcc::config();
}

fn enable_flash_cache() {
    // F2 and F4 support these
    #[cfg(any(context = "stm32f401re", context = "stm32f411re",))]
    {
        // reset the instruction cache
        embassy_stm32::pac::FLASH
            .acr()
            .modify(|w| w.set_icrst(true));
        // enable the instruction cache and prefetch
        embassy_stm32::pac::FLASH.acr().modify(|w| w.set_icen(true));
        embassy_stm32::pac::FLASH
            .acr()
            .modify(|w| w.set_prften(true));
        // reset the data cache
        embassy_stm32::pac::FLASH
            .acr()
            .modify(|w| w.set_dcrst(true));
        embassy_stm32::pac::FLASH
            .acr()
            .modify(|w| w.set_dcrst(false));
        // enable the data cache
        embassy_stm32::pac::FLASH.acr().modify(|w| w.set_dcen(true));
    }
}
