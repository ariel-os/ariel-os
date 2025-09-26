use ariel_os::hal::{peripherals, uart};

#[cfg(context = "nrf52832")]
pub type TestUart<'a> = uart::UARTE0<'a>;
#[cfg(context = "nrf52832")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_02,
    uart_tx: P0_03,
});

#[cfg(context = "nrf52833")]
pub type TestUart<'a> = uart::UARTE0<'a>;
#[cfg(context = "nrf52833")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_02,
    uart_tx: P0_03,
});

// Side UART of Arduino v3 connector
#[cfg(context = "nrf52840")]
pub type TestUart<'a> = uart::UARTE0<'a>;
#[cfg(context = "nrf52840")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P1_01,
    uart_tx: P1_02,
});

// Side UART of Arduino v3 connector
#[cfg(context = "nrf5340")]
pub type TestUart<'a> = uart::SERIAL3<'a>;
#[cfg(context = "nrf5340")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P1_00,
    uart_tx: P1_01,
});

// Side UART of Arduino v3 connector
#[cfg(context = "nrf9151")]
pub type TestUart<'a> = uart::SERIAL3<'a>;
#[cfg(context = "nrf9151")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_00,
    uart_tx: P0_01,
});

// Side UART of Arduino v3 connector
#[cfg(context = "nrf9160")]
pub type TestUart<'a> = uart::SERIAL3<'a>;
#[cfg(context = "nrf9160")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_00,
    uart_tx: P0_01,
});
