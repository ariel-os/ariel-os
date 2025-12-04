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
    time::{Duration, with_timeout},
    uart::Baudrate,
};

use embedded_io_async::{Read as _, Write as _};

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    info!("Starting UART test");

    let mut config = hal::uart::Config::default();
    config.baudrate = Baudrate::_115200;
    info!("Selected configuration: {:?}", config);

    let mut rx_buf = [0u8; 32];
    let mut tx_buf = [0u8; 32];

    let mut uart = pins::TestUart::new(
        peripherals.uart_rx,
        peripherals.uart_tx,
        &mut rx_buf,
        &mut tx_buf,
        config,
    )
    .expect("Invalid UART configuration");

    const OUT: &str = "Test Message";
    let mut input = [0u8; OUT.len()];

    uart.write_all(OUT.as_bytes()).await.unwrap();
    uart.flush().await.unwrap();
    info!("Wrote bytes");
    let _ = with_timeout(Duration::from_secs(5), uart.read_exact(&mut input)).await;

    info!("Got: {}", Hex(input));
    assert_eq!(OUT.as_bytes(), input);
    info!("Test passed!");

    exit(ExitCode::SUCCESS);
}
