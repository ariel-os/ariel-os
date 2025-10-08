use ariel_os::hal::{peripherals, uart};

// Side UART of Arduino v3 connector
#[cfg(context = "seeedstudio-lora-e5-mini")]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(context = "seeedstudio-lora-e5-mini")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PB7,
    uart_tx: PB6,
});

// Side UART of Arduino v3 connector
#[cfg(context = "st-nucleo-c031c6")]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(context = "st-nucleo-c031c6")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PB7,
    uart_tx: PB6,
});

// Side UART of Arduino v3 connector
#[cfg(context = "st-nucleo-f042k6")]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(context = "st-nucleo-f042k6")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA10,
    uart_tx: PA9,
});

// Side UART of Arduino v3 connector
#[cfg(context = "st-nucleo-f401re")]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(context = "st-nucleo-f401re")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA10,
    uart_tx: PA9,
});

// Side UART of Arduino v3 connector
#[cfg(context = "st-nucleo-h755zi-q")]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(context = "st-nucleo-h755zi-q")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PB7,
    uart_tx: PB6,
});

// Side UART of Arduino v3 connector
#[cfg(context = "st-b-l475e-iot01a")]
pub type TestUart<'a> = uart::UART4<'a>;
#[cfg(context = "st-b-l475e-iot01a")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA1,
    uart_tx: PA0,
});

// Side UART of Arduino v3 connector
#[cfg(context = "st-nucleo-wb55")]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(context = "st-nucleo-wb55")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA10,
    uart_tx: PA9,
});

// Side UART of Arduino v3 connector
#[cfg(context = "st-nucleo-wba55")]
pub type TestUart<'a> = uart::LPUART1<'a>;
#[cfg(context = "st-nucleo-wba55")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA10,
    uart_tx: PB5,
});

// JTAG UART
#[cfg(context = "st-steval-mkboxpro")]
pub type TestUart<'a> = uart::UART4<'a>;
#[cfg(context = "st-steval-mkboxpro")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA1,
    uart_tx: PA0,
});
