#![no_main]
#![no_std]

use ariel_os::{debug::log::*, net, reexports::embassy_net};
use embassy_net::udp::{PacketMetadata, UdpSocket};

// UDP datagrams with payloads larger than this will be dropped and ignored, both when receiving
// and sending, so the size of the three buffers needs to be the same in this echo example.
const BUFFER_SIZE: usize = 128;

#[ariel_os::task(autostart)]
async fn udp_echo() {
    let stack = net::network_stack().await.unwrap();

    let mut rx_meta = [PacketMetadata::EMPTY; 1];
    let mut rx_buffer = [0; BUFFER_SIZE];
    let mut tx_meta = [PacketMetadata::EMPTY; 1];
    let mut tx_buffer = [0; BUFFER_SIZE];
    let mut buf = [0; BUFFER_SIZE];

    loop {
        let mut socket = UdpSocket::new(
            stack,
            &mut rx_meta,
            &mut rx_buffer,
            &mut tx_meta,
            &mut tx_buffer,
        );

        info!("Listening on UDP:1234...");
        if let Err(e) = socket.bind(1234) {
            info!("bind error: {:?}", e);
            continue;
        }

        loop {
            let (n, remote_endpoint) = match socket.recv_from(&mut buf).await {
                Ok((0, _)) => {
                    info!("read EOF");
                    break;
                }
                Ok((n, remote_endpoint)) => (n, remote_endpoint),
                Err(e) => {
                    info!("read error: {:?}", e);
                    break;
                }
            };

            info!("Received datagram from {:?}", remote_endpoint);

            match socket.send_to(&buf[..n], remote_endpoint).await {
                Ok(()) => {}
                Err(e) => {
                    info!("write error: {:?}", e);
                    break;
                }
            };
        }
    }
}
