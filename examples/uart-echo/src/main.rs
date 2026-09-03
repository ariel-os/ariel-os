//! This example shows how to use SBD defined uarts.

#![no_main]
#![no_std]

use ariel_os::{
    debug::{ExitCode, exit},
    hal,
    log::info,
    uart::{Assignment, Baudrate},
};

use embedded_io_async::{Read as _, Write as _};

type UartAssignment = ariel_os_boards::pins::HOST_FACING_UART;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: UartAssignment) {
    info!("Starting UART echo test");

    let mut config = hal::uart::Config::default();
    config.baudrate = Baudrate::_115200;

    info!("Selected configuration: {:?}", config);

    let mut rx_buf = [0u8; 32];
    let mut tx_buf = [0u8; 32];

    let (tx, rx) = peripherals.into_pins();

    let mut uart =
        <UartAssignment as Assignment>::Device::new(rx, tx, &mut rx_buf, &mut tx_buf, config)
            .expect("Invalid UART configuration");

    uart.write_all(b"Ariel OS uart echo started!\r\n")
        .await
        .unwrap();

    let mut input = [0u8; 32];

    'main: loop {
        let n = uart.read(&mut input).await.unwrap();

        for byte in input.iter_mut().take(n) {
            if *byte == 0x03 {
                info!("exiting");
                uart.write_all(b"exiting\r\n").await.unwrap();
                break 'main;
            }
        }

        let _ = uart.write(&input[0..n]).await.unwrap();
    }

    exit(ExitCode::SUCCESS);
}
