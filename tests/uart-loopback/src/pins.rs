use ariel_os::hal::{peripherals, uart};

#[cfg(context = "esp")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "esp")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_tx: GPIO4,
    uart_rx: GPIO5,
});

#[cfg(context = "nrf52832")]
pub type TestUart<'a> = uart::UARTE0<'a>;
#[cfg(context = "nrf52832")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_12,
    uart_tx: P0_11,
});

#[cfg(context = "nrf52833")]
pub type TestUart<'a> = uart::UARTE0<'a>;
#[cfg(context = "nrf52833")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_02,
    uart_tx: P0_03,
});

#[cfg(context = "nrf52840")]
pub type TestUart<'a> = uart::UARTE0<'a>;
#[cfg(context = "nrf52840")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P1_01,
    uart_tx: P1_02,
});

#[cfg(context = "nrf5340")]
pub type TestUart<'a> = uart::SERIAL3<'a>;
#[cfg(context = "nrf5340")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P1_00,
    uart_tx: P1_01,
});

#[cfg(context = "nrf9151")]
pub type TestUart<'a> = uart::SERIAL3<'a>;
#[cfg(context = "nrf9151")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_00,
    uart_tx: P0_01,
});

#[cfg(context = "nrf9160")]
pub type TestUart<'a> = uart::SERIAL3<'a>;
#[cfg(context = "nrf9160")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: P0_00,
    uart_tx: P0_01,
});

#[cfg(context = "rp")]
pub type TestUart<'a> = uart::UART0<'a>;
#[cfg(context = "rp")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PIN_17,
    uart_tx: PIN_16,
});

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
#[cfg(any(context = "st-nucleo-f401re", context = "st-nucleo-f411re"))]
pub type TestUart<'a> = uart::USART1<'a>;
#[cfg(any(context = "st-nucleo-f401re", context = "st-nucleo-f411re"))]
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
#[cfg(context = "stm32u083c-dk")]
pub type TestUart<'a> = uart::USART2<'a>;
#[cfg(context = "stm32u083c-dk")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA3,
    uart_tx: PA2,
});

// Side UART of Arduino v3 connector
#[cfg(context = "st-nucleo-wb55")]
pub type TestUart<'a> = uart::LPUART1<'a>;
#[cfg(context = "st-nucleo-wb55")]
ariel_os::hal::define_peripherals!(Peripherals {
    uart_rx: PA3,
    uart_tx: PA2,
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
