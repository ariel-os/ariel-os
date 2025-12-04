//! HAL- and MCU-specific types for UART.
//!
//! This module provides a driver for each UART peripheral, the driver name being the same as the
//! peripheral; see the tests and examples to learn how to instantiate them.

/// Peripheral-agnostic UART driver implementing [`embedded_io_async::Read`]
/// and [`embedded_io_async::Write`].
///
/// This type is not meant to be instantiated directly; instead instantiate a peripheral-specific
/// driver provided by this module.
// NOTE: we keep this type public because it may still required in user-written type signatures.
pub enum Uart {
    // Make the docs show that this enum has variants, but do not show any because they are
    // MCU-specific.
    #[doc(hidden)]
    Hidden,
}

pub fn init(_peripherals: &mut crate::hal::OptionalPeripherals) {
    unimplemented!();
}
