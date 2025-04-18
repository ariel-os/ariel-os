use ariel_os::hal::{peripherals, uart};

#[cfg(context = "esp")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "esp")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_tx: GPIO16,
    uart_rx: GPIO17,
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
pub type TestUart<'a> = uart::SERIAL2;
#[cfg(context = "nrf5340")]
ariel_os::hal::define_peripherals!(Peripherals {});

#[cfg(context = "rp")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PIN_17,
    uart_tx: PIN_16,
});

// Side UART of Arduino v3 connector
#[cfg(context = "stm32c031c6")]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(context = "stm32c031c6")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PB7,
    uart_tx: PB6,
});

// Side UART of Arduino v3 connector
#[cfg(context = "stm32h755zi")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "stm32h755zi")]
ariel_os::hal::define_peripherals!(Peripherals {});

// Side UART of Arduino v3 connector
#[cfg(context = "stm32wb55rg")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "stm32wb55rg")]
ariel_os::hal::define_peripherals!(Peripherals {});

// Side UART of Arduino v3 connector
#[cfg(context = "stm32f401re")]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(context = "stm32f401re")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA10,
    uart_tx: PA9,
});
