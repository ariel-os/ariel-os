//! This is a test for UART loopback operation

#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

mod pins;

use ariel_os::{
    debug::{ExitCode, exit, log::info},
    hal,
    time::Delay,
};

use embedded_hal_async::delay::DelayNs;
use embedded_io_async::Read;
use embedded_io_async::Write;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    // Delay to segment individual runs on the logic analyzer.
    Delay {}.delay_ms(500).await;
    info!("Starting UART test");

    let mut config = hal::uart::Config::default();
    config.baudrate = 9600;
    let mut rx_buf: [u8; 32] = [0u8; 32];
    let mut tx_buf: [u8; 32] = [0u8; 32];
    info!("Selected baud rate: {}", config.baudrate);
    let mut uart = pins::TestUart::new(
        peripherals.uart_rx,
        peripherals.uart_tx,
        &mut rx_buf,
        &mut tx_buf,
        config,
    );

    const OUT: &str = &"Test Message";
    let mut in_ = [0u8; OUT.len()];

    let _ = uart.write_all(OUT.as_bytes()).await;
    let _ = uart.flush().await;
    info!("written bytes");
    let _ = uart.read_exact(&mut in_).await;

    info!("got: '{:x}'", &in_);
    assert_eq!(OUT.as_bytes(), in_);
    info!("Test passed!");

    Delay {}.delay_ms(500).await;
    exit(ExitCode::SUCCESS);
}
