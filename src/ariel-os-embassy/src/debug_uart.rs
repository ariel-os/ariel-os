#![deny(clippy::pedantic)]

#[cfg(context = "nrf")]
type UartDriver = embassy_nrf::uarte::Uarte<'static, embassy_nrf::peripherals::UARTE0>;
#[cfg(context = "stm32")]
type UartDriver = embassy_stm32::usart::Uart<'static, embassy_stm32::mode::Blocking>;

static DEBUG_UART: embassy_sync::once_lock::OnceLock<
    embassy_sync::mutex::Mutex<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        UartDriver,
    >,
> = embassy_sync::once_lock::OnceLock::new();

#[expect(clippy::missing_panics_doc)]
pub fn init(peripherals: &mut crate::hal::OptionalPeripherals) {
    // TODO: this could later be replaced with our UART abstraction and app configuration.
    let uart = iot_lab::get_uart_driver(peripherals);

    let _ = DEBUG_UART.init(embassy_sync::mutex::Mutex::new(uart));

    let _ = ariel_os_debug::DEBUG_UART_WRITE_FN.init(write_debug_uart);
}

fn write_debug_uart(buffer: &[u8]) {
    #[cfg(context = "stm32")]
    use embedded_io::Write as _;
    #[cfg(context = "nrf")]
    use embedded_io_async::Write as _;

    // FIXME: do not unwrap
    embassy_futures::block_on(async {
        // This effectively drops any debug output until the UART driver is populated.
        // If we instead waited on it to be set, this would deadlock when trying to print
        // on the debug output before the driver is populated.
        if let Some(uart) = DEBUG_UART.try_get() {
            let mut uart = uart.lock().await;

            #[cfg(context = "nrf")]
            {
                uart.write(buffer).await.unwrap();
                // TODO: is flushing needed here?
                uart.flush().await.unwrap();
            }

            #[cfg(context = "stm32")]
            {
                uart.write(buffer).unwrap();
                // TODO: is flushing needed here?
                uart.flush().unwrap();
            }
        }
    });
}

mod iot_lab {
    //! UART configuration required for [IoT-LAB](https://www.iot-lab.info).

    #[cfg(context = "nrf")]
    pub fn get_uart_driver(peripherals: &mut crate::hal::OptionalPeripherals) -> super::UartDriver {
        let mut config = embassy_nrf::uarte::Config::default();

        embassy_nrf::bind_interrupts!(struct Irqs {
            UARTE0 => embassy_nrf::uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
        });

        #[cfg(any(context = "nrf52dk", context = "nrf52840dk"))]
        let (p, uart_rx, uart_tx) = {
            // https://www.iot-lab.info/docs/boards/nordic-nrf52840dk/
            config.baudrate = embassy_nrf::buffered_uarte::Baudrate::BAUD115200;
            (
                peripherals.UARTE0.take().unwrap(),
                peripherals.P0_08.take().unwrap(),
                peripherals.P0_06.take().unwrap(),
            )
        };

        embassy_nrf::uarte::Uarte::new(p, Irqs, uart_rx, uart_tx, config)
    }

    #[cfg(context = "stm32")]
    pub fn get_uart_driver(peripherals: &mut crate::hal::OptionalPeripherals) -> super::UartDriver {
        let mut config = embassy_stm32::usart::Config::default();

        #[cfg(any(context = "st-b-l475e-iot01a", context = "st-nucleo-wb55"))]
        let (p, uart_rx, uart_tx) = {
            // https://www.iot-lab.info/docs/boards/st-b-l475e-iot01a/
            config.baudrate = 115_200;
            (
                peripherals.USART1.take().unwrap(),
                peripherals.PB7.take().unwrap(),
                peripherals.PB6.take().unwrap(),
            )
        };

        #[cfg(context = "stm32u083c-dk")]
        let (p, uart_rx, uart_tx) = {
            config.baudrate = 115_200;
            (
                peripherals.USART2.take().unwrap(),
                peripherals.PA3.take().unwrap(),
                peripherals.PA2.take().unwrap(),
            )
        };

        embassy_stm32::usart::Uart::new_blocking(p, uart_rx, uart_tx, config).unwrap()
    }
}
