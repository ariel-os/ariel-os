//! Items specific to the STMicroelectronics STM32 MCUs.

#![no_std]
#![cfg_attr(nightly, feature(doc_auto_cfg))]
#![deny(missing_docs)]

pub mod gpio;

#[doc(hidden)]
pub mod peripheral {
    pub use embassy_stm32::Peripheral;
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

#[cfg(feature = "storage")]
#[doc(hidden)]
pub mod storage;

#[cfg(feature = "usb")]
#[doc(hidden)]
pub mod usb;

#[cfg(feature = "eth")]
#[doc(hidden)]
pub mod eth;

use embassy_stm32::Config;

#[doc(hidden)]
pub use embassy_stm32::{OptionalPeripherals, Peripherals, interrupt};

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

// TODO: find better place for this
#[expect(clippy::too_many_lines)]
fn board_config(config: &mut Config) {
    #[cfg(context = "st-b-l475e-iot01a")]
    {
        use embassy_stm32::rcc::*;

        // This board has an LSE clock, we can use it to calibrate the MSI clock
        config.rcc.ls = LsConfig {
            rtc: RtcClockSource::LSE,
            lsi: false,
            lse: Some(LseConfig {
                frequency: embassy_stm32::time::Hertz(32768),
                mode: LseMode::Oscillator(LseDrive::MediumHigh),
            }),
        };
        config.rcc.hsi = false;
        // Setting the MSI range to 48 MHz crashes the system. If the source of the issue is found,
        // we can use MSI as the clock source for the usb peripheral directly and avoid using more PLLs.
        config.rcc.msi = Some(MSIRange::RANGE8M);
        config.rcc.pll = Some(Pll {
            source: PllSource::MSI,
            prediv: PllPreDiv::DIV1, // 8 Mhz
            mul: PllMul::MUL20,      // 160 MHz
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2), // sysclk 80Mhz (8 / 1  * 20 / 2)
        });
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.pllsai1 = Some(Pll {
            source: PllSource::MSI,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL12, // 8 MHz MSI * 12 = 96 MHz
            divp: None,
            divq: Some(PllQDiv::DIV2), // USB 48 MHz (8 / 1 * 12 / 2)
            divr: None,
        });
        // With a 32.768 kHz LSE, the MSI clock will be calibrated and considered accurate enough.
        // Embassy automatically enables MSIPLLEN if the LSE is configured.
        config.rcc.mux.clk48sel = mux::Clk48sel::PLLSAI1_Q;
    }

    #[cfg(context = "st-nucleo-wb55")]
    {
        use embassy_stm32::rcc::*;

        config.rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(32000000),
            mode: HseMode::Oscillator,
            prescaler: HsePrescaler::DIV1,
        });
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL10,
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2), // sysclk 80Mhz (32 / 2 * 10 / 2)
        });
        config.rcc.mux.clk48sel = mux::Clk48sel::HSI48;
    }

    #[cfg(context = "st-nucleo-f401re")]
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(8000000),
            mode: HseMode::Bypass,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL84,
            divp: Some(PllPDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }

    #[cfg(context = "st-nucleo-f767zi")]
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(8000000),
            mode: HseMode::Bypass,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL216,
            divp: Some(PllPDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV4;
        config.rcc.apb2_pre = APBPrescaler::DIV2;
        config.rcc.sys = Sysclk::PLL1_P;
    }

    #[cfg(context = "stm32h755zi")]
    {
        use embassy_stm32::rcc::*;

        config.rcc.hsi = Some(HSIPrescaler::DIV1);
        config.rcc.csi = true;
        config.rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            // Required for SPI (configured by `spi123sel`)
            divq: Some(PllDiv::DIV16), // FIXME: adjust this divider
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale1;
        // Set SMPS power config otherwise MCU will not powered after next power-off
        config.rcc.supply_config = SupplyConfig::DirectSMPS;
        config.rcc.mux.usbsel = mux::Usbsel::HSI48;
        // Select the clock signal used for SPI1, SPI2, and SPI3.
        // FIXME: what to do about SPI4, SPI5, and SPI6?
        config.rcc.mux.spi123sel = mux::Saisel::PLL1_Q; // Reset value
    }

    #[cfg(context = "stm32u083mc")]
    {
        use embassy_stm32::rcc::*;

        config.rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        // No HSE fitted on the stm32u083c-dk board
        config.rcc.hsi = true;
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.pll = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL7,
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2), // sysclk 56Mhz
        });
        config.rcc.mux.clk48sel = mux::Clk48sel::HSI48;
    }

    #[cfg(context = "st-steval-mkboxpro")]
    {
        use embassy_stm32::rcc::*;

        config.rcc.ls = LsConfig {
            rtc: RtcClockSource::LSE,
            lsi: true,
            lse: Some(LseConfig {
                peripherals_clocked: true,
                frequency: embassy_stm32::time::Hertz(32768),
                mode: LseMode::Oscillator(LseDrive::MediumHigh),
            }),
        };
        config.rcc.hsi = true;
        config.rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(16_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL10,
            divp: None,
            divq: None,
            divr: Some(PllDiv::DIV1), // sysclk 160Mhz (16 / 1 * 10 / 1)
        });
        config.rcc.sys = Sysclk::PLL1_R;
        config.rcc.mux.iclksel = mux::Iclksel::HSI48;
        config.rcc.voltage_range = VoltageScale::RANGE1;
    }

    #[cfg(context = "stm32f042k6")]
    {
        use embassy_stm32::rcc::*;

        config.rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        config.rcc.sys = Sysclk::HSI48;
        config.rcc.pll = Some(Pll {
            src: PllSource::HSI48,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL2,
        });
    }

    #[cfg(context = "seeedstudio-lora-e5-mini")]
    {
        use embassy_stm32::rcc::*;

        config.rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(32_000_000),
            mode: HseMode::Bypass,
            prescaler: HsePrescaler::DIV1,
        });
        config.rcc.ls = LsConfig::default_lse();
        config.rcc.msi = None;
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL6,
            divp: None,
            divq: Some(PllQDiv::DIV2), // PLL1_Q clock (32 / 2 * 6 / 2), used for RNG
            divr: Some(PllRDiv::DIV2), // sysclk 48Mhz clock (32 / 2 * 6 / 2)
        });

        config.rcc.sys = Sysclk::PLL1_R;
    }

    // mark used
    let _ = config;
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
