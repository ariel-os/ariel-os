use ariel_os::hal::{peripherals, uart};

#[cfg(context = "rp")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PIN_17,
    uart_tx: PIN_16,
});
