//! This is a test for UART loopback operation

#![no_main]
#![no_std]

mod pins;

use ariel_os::{
    debug::{
        ExitCode, exit,
        log::{Hex, info},
    },
    hal,
    time::Duration,
    uart::Baud,
};

use ariel_os::reexports::embassy_time::WithTimeout;

use embedded_io_async::{Read as _, Write as _};

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    info!("Starting UART test");

    let mut config = hal::uart::Config::default();
    config.baudrate = Baud::_9600;
    info!("Selected configuration: {}", config);

    let mut rx_buf = [0u8; 32];
    let mut tx_buf = [0u8; 32];

    let mut uart = pins::TestUart::new(
        peripherals.uart_rx,
        peripherals.uart_tx,
        &mut rx_buf,
        &mut tx_buf,
        config,
    );

    const OUT: &str = "Test Message";
    let mut in_ = [0u8; OUT.len()];

    uart.write_all(OUT.as_bytes()).await.unwrap();
    uart.flush().await.unwrap();
    info!("Wrote bytes");
    let _ = uart
        .read_exact(&mut in_)
        .with_timeout(Duration::from_secs(5))
        .await;

    info!("Got: {}", Hex(in_));
    assert_eq!(OUT.as_bytes(), in_);
    info!("Test passed!");

    exit(ExitCode::SUCCESS);
}
