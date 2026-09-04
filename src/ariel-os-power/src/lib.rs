//! Provides power management functionality.

#![deny(missing_docs)]
#![cfg_attr(not(context = "native"), no_std)]
#![cfg_attr(context = "xtensa", feature(asm_experimental_arch))]

mod reset;

pub use reset::*;

/// Enters sleep mode.
///
/// In this mode, the clock of the CPU is off.
///
/// # Important Note
///
/// This is currently implemented on a best-effort basis.
/// Some microcontrollers may not support these low-power settings, they may not be implemented
/// yet, or they may be lacking testing.
/// Do measure the power consumption of your hardware when relevant for your application.
///
/// # Wake-up conditions
///
/// Any interrupt usually makes the microcontroller exit this mode.
pub fn enter_sleep_mode() {
    #![allow(unsafe_code, reason = "used on Xtensa")]

    cfg_select! {
        context = "cortex-m" => {
            cortex_m::asm::wfi();
        }
        context = "riscv" => {
            riscv::asm::wfi();
        }
        context = "xtensa" => {
            // The options are similar to those used for wfi on RISC-V and Cortex-M:
            // the instruction does not modify memory or the stack, and does preserve flags.
            // SAFETY: executing `waiti 0` is sound.
            unsafe {
                core::arch::asm!("waiti 0", options(nomem, nostack, preserves_flags));
            }
        }
    }
}
