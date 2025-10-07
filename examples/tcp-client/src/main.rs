#![no_main]
#![no_std]

// modification of https://github.com/embassy-rs/embassy/blob/main/examples/nrf9160/src/bin/modem_tcp_client.rs

use ariel_os::gpio::{Level, Output};
use ariel_os::time::Timer;
use ariel_os::{debug::log::*, net, reexports::embassy_net, time::Duration};
use core::str::FromStr;
use embedded_io_async::Write;
mod pins;

#[ariel_os::task(autostart, peripherals)]
async fn tcp_echo(peripherals: pins::LedPeripherals) {
    let stack = net::network_stack().await.unwrap();
    let mut led = Output::new(peripherals.led, Level::Low);

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    info!("waiting for interface to come up...");
    led.set_low();
    stack.wait_config_up().await;

    loop {
        led.set_high();

        let mut socket = embassy_net::tcp::TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        info!("Connecting...");
        let host_addr = embassy_net::Ipv4Address::from_str("45.79.112.203").unwrap();
        if let Err(e) = socket.connect((host_addr, 4242)).await {
            warn!("connect error: {:?}", e);
            Timer::after_secs(10).await;
            continue;
        }
        info!("Connected to {:?}", socket.remote_endpoint());

        let msg = b"Hello world!\n";
        for _ in 0..10 {
            if let Err(e) = socket.write_all(msg).await {
                warn!("write error: {:?}", e);
                break;
            }
            info!("txd: {}", core::str::from_utf8(msg).unwrap());
            Timer::after_secs(1).await;
        }
        led.set_low();

        Timer::after_secs(4).await;
    }
}
