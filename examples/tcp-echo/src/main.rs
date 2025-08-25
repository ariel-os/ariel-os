#![no_main]
#![no_std]

use ariel_os::{debug::log::*, net, reexports::embassy_net, time::Duration};
use embassy_net::tcp::TcpSocket;
use embedded_io_async::Write;

// Setting this to a small value would make packet handling slow and choppy, but would not cause
// packets to be dropped.
const RX_BUFFER_SIZE: usize = 128;
// There is a memory–performance trade-off with these, but small values seem to be working fine for
// this example.
const TX_BUFFER_SIZE: usize = 8;
const RW_BUFFER_SIZE: usize = 8;

#[ariel_os::task(autostart)]
async fn tcp_echo() {
    let stack = net::network_stack().await.unwrap();

    let mut rx_buffer = [0; RX_BUFFER_SIZE];
    let mut tx_buffer = [0; TX_BUFFER_SIZE];
    let mut buf = [0; RW_BUFFER_SIZE];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        info!("Listening on TCP:1234...");
        if let Err(e) = socket.accept(1234).await {
            info!("accept error: {:?}", e);
            continue;
        }

        info!("Received connection from {:?}", socket.remote_endpoint());

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => {
                    info!("read EOF");
                    break;
                }
                Ok(n) => n,
                Err(e) => {
                    info!("read error: {:?}", e);
                    break;
                }
            };

            match socket.write_all(&buf[..n]).await {
                Ok(()) => {}
                Err(e) => {
                    info!("write error: {:?}", e);
                    break;
                }
            };
        }
    }
}
