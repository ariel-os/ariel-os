#![expect(unsafe_code)]

// AVR-specific runtime initialization and entry point.
// Uses avr-device's `entry` macro (enabled via `rt` feature) to provide
// the reset handler which initializes .bss/.data and sets up the stack,
// then calls `main()` which invokes `crate::startup()`.

use avr_device::asm;

/// Entry point for AVR — called by the reset vector after .bss/.data init.
///
/// The `avr-device` crate with the `rt` feature provides the `#[entry]`
/// macro which generates the appropriate reset vector entry point and
/// performs standard startup initialization (zeroing .bss, copying .data
/// from flash, setting up the stack pointer).
#[cfg(not(feature = "embedded-test"))]
#[avr_device::entry]
fn main() -> ! {
    crate::startup();
}

pub fn init() {}

/// Wait for interrupt — puts the AVR into sleep mode until an interrupt occurs.
///
/// This is the AVR equivalent of `wfi` (wait for interrupt) on ARM.
/// Used by the idle loop in `startup()` when the `threading` feature is not enabled.
pub fn wfi() {
    asm::sleep();
}

pub(crate) fn sp() -> usize {
    0
}

use crate::stack::Stack;
pub(crate) fn stack() -> crate::stack::Stack {
    Stack::default()
}
