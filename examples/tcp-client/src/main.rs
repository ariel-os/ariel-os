// modification of https://github.com/embassy-rs/embassy/blob/main/examples/nrf9160/src/bin/modem_tcp_client.rs

#![no_main]
#![no_std]

use core::str::FromStr as _;

use embedded_io_async::Write as _;

use ariel_os::time::Timer;
use ariel_os::{
    log::{info, warn},
    net,
    reexports::embassy_net,
    time::Duration,
};

#[ariel_os::task(autostart)]
async fn tcp_echo() {
    let interface = net::network_interface().await.unwrap();

    let stack = interface.network_stack();

    // Increase the buffer size if you want to send bigger packets.
    let mut rx_buffer = [0; 256];
    let mut tx_buffer = [0; 256];

    info!("waiting for interface to come up...");

    loop {
        interface.enable();
        stack.wait_link_up().await;
        stack.wait_config_up().await;
        let mut socket = embassy_net::tcp::TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        info!("Connecting...");
        // Connect to https://tcpbin.com/ without using DNS
        let host_addr = embassy_net::Ipv4Address::from_str("45.79.112.203").unwrap();
        if let Err(e) = socket.connect((host_addr, 4242)).await {
            warn!("connect error: {:?}", e);
            Timer::after_secs(10).await;
            continue;
        }
        info!("Connected to {:?}", socket.remote_endpoint());

        let msg = b"Hello world!\n";
        for _ in 0..10 {
            #[allow(unused_variables, reason = "log macro sometimes doesn't use this")]
            if let Err(e) = socket.write_all(msg).await {
                warn!("write error: {:?}", e);
                break;
            }
            // We put trust in the server to return a valid utf8 string
            info!("txd: {}", core::str::from_utf8(msg).unwrap());
            Timer::after_secs(1).await;
        }
        interface.disable();
        Timer::after_secs(10).await;
    }
}
