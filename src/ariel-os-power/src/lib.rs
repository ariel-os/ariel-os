//! Provides power management functionality.

#![deny(missing_docs)]
#![cfg_attr(not(context = "native"), no_std)]

mod reset;

pub use reset::*;

/// Enters standby mode.
///
/// In this mode, almost every clock of the microcontroller is off, and the RAM contents may or may
/// not be retained, requiring rebooting the application completely when waking up.
/// This function never returns to represent that.
///
/// # Wake-up conditions
///
/// Depending on the microcontroller, waking-up usually requires an RTC interrupt or an external
/// interrupt (sometimes on a limited set of pins).
pub fn enter_standby_mode() -> ! {
    #![allow(unsafe_code, reason = "only for STM32")]

    cfg_select! {
        context = "stm32" => {
            // NOTE: a critical section is used for atomicity.
            critical_section::with(|_| {
                // TODO: use the Shutdown mode when `stm32-metapac` supports it.

                // NOTE: each Reference Manual gets its own branch.
                cfg_select! {
                    // STM32C0: Table 28 of RM0490 Rev 5.
                    context = "stm32c031c6" => {
                        use embassy_stm32::pac::pwr::vals::Lpms;

                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_lpms(Lpms::STANDBY));
                    }
                    // STM32F0x2: Table 17 of TM0091 Rev 10.
                    context = "stm32f042k6" => {
                        use embassy_stm32::pac::pwr::vals::Pdds;

                        embassy_stm32::pac::PWR.cr().modify(|w| w.set_pdds(Pdds::STANDBY_MODE));
                    }
                    // STM32F303: Table 20 of RM0316 Rev 10.
                    any(context = "stm32f303cb", context = "stm32f303re") => {
                        use embassy_stm32::pac::pwr::vals::Pdds;

                        embassy_stm32::pac::PWR.cr().modify(|w| w.set_pdds(Pdds::STANDBY_MODE));
                    }
                    // STM32F401: Table 20 of RM0368 Rev 5.
                    context = "stm32f401re" => {
                        use embassy_stm32::pac::pwr::vals::Pdds;

                        // The RM calls this register `PWR_CR`.
                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_pdds(Pdds::STANDBY_MODE));
                    }
                    // STM32F411: Table 19 of RM0383 Rev 3.
                    context = "stm32f411re" => {
                        use embassy_stm32::pac::pwr::vals::Pdds;

                        // The RM calls this register `PWR_CR`.
                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_pdds(Pdds::STANDBY_MODE));
                    }
                    // STM32F76: Table 21 of RM0410 Rev 5.
                    context = "stm32f767zi" => {
                        use embassy_stm32::pac::pwr::vals::Pdds;

                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_pdds(Pdds::STANDBY_MODE));
                    }
                    // STM32H753: Table 39 and Table 46 of RM0433.
                    context = "stm32h753zi" => {
                        // FIXME: needs some power measurements to confirm this works as expected.
                        embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_run_d3(false));

                        embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_pdds_d1(true));
                        embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_pdds_d2(true));
                        embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_pdds_d3(true));
                    }
                    // STM32H755: Table 40 and Table 47 of RM0399 Rev 4.
                    context = "stm32h755zi" => {
                        // FIXME: needs some power measurements to confirm this works as expected.
                        embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_run_d3(false));

                        embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_pdds_d1(true));
                        embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_pdds_d2(true));
                        embassy_stm32::pac::PWR.cpucr().modify(|w| w.set_pdds_d3(true));
                    }
                    // STM32L47: Table 30 of RM0351 Rev 10.
                    context = "stm32l475vg" => {
                        use embassy_stm32::pac::pwr::vals::{Lpms, Rrs};

                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_lpms(Lpms::STANDBY));
                        embassy_stm32::pac::PWR.cr3().modify(|w| w.set_rrs(Rrs::POWER_OFF));
                    }
                    // STM32U0: Table 29 of RM0503 Rev 4.
                    any(context = "stm32u073kc", context = "stm32u083mc") => {
                        use embassy_stm32::pac::pwr::vals::Lpms;

                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_lpms(Lpms::STANDBY));
                        // Lose SRAM2 contents.
                        embassy_stm32::pac::PWR.cr3().modify(|w| w.set_rrs(false));
                    }
                    // STM32U5: Table 105 of RM0503 Rev 6.
                    context = "stm32u585ai" => {
                        use embassy_stm32::pac::pwr::vals::Lpms;

                        // TODO: use the Standby/Shutdown mode when `stm32-metapac` supports it.
                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_lpms(Lpms::STOP3));
                        // Lose SRAM2 contents.
                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_rrsb1(false));
                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_rrsb2(false));
                    }
                    // STM32WB: Table 33 of RM0434 Rev 14.
                    context = "stm32wb55rg" => {
                        use embassy_stm32::pac::pwr::vals::Lpms;

                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_lpms(Lpms::STANDBY));
                        embassy_stm32::pac::PWR.cr3().modify(|w| w.set_rrs(false));
                    }
                    // STM32WBA5: Table 94 of RM0493 Rev 7.
                    context = "stm32wba55cg" => {
                        use embassy_stm32::pac::pwr::vals::Lpms;

                        // FIXME: use the Standby/Shutdown mode when `stm32-metapac` supports it.
                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_lpms(Lpms::STOP1));
                    }
                    // STM32WBA6: Table 96 of RM0515 Rev 4.
                    context = "stm32wba65ri" => {
                        use embassy_stm32::pac::pwr::vals::Lpms;

                        // FIXME: use the Standby/Shutdown mode when `stm32-metapac` supports it.
                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_lpms(Lpms::STOP1));
                    }
                    // STM32WLE: Table 46 for RM0461 Rev 10.
                    context = "stm32wle5jc" => {
                        use embassy_stm32::pac::pwr::vals::Lpms;

                        embassy_stm32::pac::PWR.cr1().modify(|w| w.set_lpms(Lpms::STANDBY));
                        embassy_stm32::pac::PWR.cr3().modify(|w| w.set_rrs(false));
                    }
                }

                // SAFETY: the peripherals are obtained and used inside a single critical section.
                let mut p = unsafe { cortex_m::Peripherals::steal() };
                p.SCB.set_sleepdeep();
            });

            // A single iteration of this loop will be executed, but this satisfies the return
            // type.
            loop {
                cortex_m::asm::wfi();
            }
        }
        _ => {
            #[expect(clippy::empty_loop, reason = "for platform-independent tooling only")]
            loop {}
        }
    }
}
