use ariel_os::hal::{peripherals, uart};

#[cfg(context = "esp")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "esp")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_tx: GPIO16,
    uart_rx: GPIO17,
});
