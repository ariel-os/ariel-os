#![deny(clippy::pedantic)]

pub static DEBUG_UART: embassy_sync::once_lock::OnceLock<
    embassy_sync::mutex::Mutex<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        embassy_nrf::uarte::Uarte<embassy_nrf::peripherals::UARTE0>,
        // embassy_stm32::usart::Uart<embassy_stm32::mode::Blocking>,
    >,
> = embassy_sync::once_lock::OnceLock::new();

#[expect(clippy::missing_panics_doc)]
pub fn init(peripherals: &mut crate::hal::OptionalPeripherals) {
    #[cfg(context = "nrf52840dk")]
    let uart = {
        let uart_rx = peripherals.P0_08.take().unwrap();
        let uart_tx = peripherals.P0_06.take().unwrap();

        embassy_nrf::bind_interrupts!(struct Irqs {
            UARTE0 => embassy_nrf::uarte::InterruptHandler<embassy_nrf::peripherals::UARTE0>;
        });

        embassy_nrf::uarte::Uarte::new(
            peripherals.UARTE0.take().unwrap(),
            Irqs,
            uart_rx,
            uart_tx,
            embassy_nrf::uarte::Config::default(),
        )
    };

    // FIXME: should be st-b-l072z-lrwan1
    #[cfg(context = "st-nucleo-h755zi-q")]
    let uart = {
        let uart_rx = peripherals.PA3.take().unwrap();
        let uart_tx = peripherals.PA2.take().unwrap();

        embassy_stm32::usart::Uart::new_blocking(
            peripherals.USART2.take().unwrap(),
            uart_rx,
            uart_tx,
            embassy_stm32::usart::Config::default(),
        )
        .unwrap()
    };

    let _ = DEBUG_UART.init(embassy_sync::mutex::Mutex::new(uart));

    let _ = ariel_os_debug::DEBUG_UART_WRITE_FN.init(write_debug_uart);
}

fn write_debug_uart(buffer: &[u8]) {
    use embedded_io_async::Write;
    // use embedded_io::Write;

    // FIXME: do not unwrap
    embassy_futures::block_on(async {
        // This effectively drops any debug output until the UART driver is populated.
        // If we instead waited on it to be set, this would deadlock when trying to print
        // on the debug output before the driver is populated.
        if let Some(uart) = DEBUG_UART.try_get() {
            let mut uart = uart.lock().await;
            uart.write(buffer).await.unwrap();
            // uart.write(buffer).unwrap();
            // TODO: is flushing needed here?
            uart.flush().await.unwrap();
            // uart.flush().unwrap();
        }
    });
}
