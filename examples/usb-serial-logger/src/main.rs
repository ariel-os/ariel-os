#![no_std]
#![no_main]

use ariel_os::{
    cell::StaticCell,
    log::{debug, error, info, println, trace, transport::register_custom_transport, warn},
    reexports::embassy_usb::{self, class::cdc_acm::CdcAcmError},
    time::Timer,
};
use ariel_os_boards as _;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
};
use embedded_io_async::Write;

const MAX_FULL_SPEED_PACKET_SIZE: u8 = 64;

const LOG_BUFFER_SIZE: usize = 1024;
static LOG_BUFFER: Pipe<CriticalSectionRawMutex, LOG_BUFFER_SIZE> = Pipe::new();

#[ariel_os::task(autostart)]
async fn logging() {
    loop {
        Timer::after_secs(1).await;
        info!("Testing USB CDC ACM logging");
        println!("-- this is printed via `println!()`");
        trace!("-- trace log level enabled");
        debug!("-- debug log level enabled");
        info!("-- info log level enabled");
        warn!("-- warn log level enabled");
        error!("-- error log level enabled (just testing)");
    }
}

#[ariel_os::task(autostart, usb_builder_hook)]
async fn main() {
    register_custom_transport(write_bytes, flush);

    info!("Hello World!");

    static STATE: StaticCell<State<'_>> = StaticCell::new();

    // Create and inject the USB class on the system USB builder.
    let class = USB_BUILDER_HOOK
        .with(|builder| {
            CdcAcmClass::new(
                builder,
                STATE.init_with(State::new),
                MAX_FULL_SPEED_PACKET_SIZE.into(),
            )
        })
        .await;

    let (mut sender, mut receiver) = class.split();
    loop {
        let sender_fut = async {
            sender.wait_connection().await;
            let mut buffer = [0; LOG_BUFFER_SIZE as usize];
            loop {
                let len = LOG_BUFFER.read(&mut buffer).await;

                if matches!(
                    sender.write_all(&buffer[..len]).await,
                    Err(CdcAcmError::NotConnected)
                ) {
                    // If the USB connection is disconnected, wait until it's reconnected.
                    sender.wait_connection().await;
                };
            }
        };

        // The serial has to be read otherwise the USB device hangs on some MCUs.
        let receiver_fut = async {
            receiver.wait_connection().await;
            let mut buffer = [0; MAX_FULL_SPEED_PACKET_SIZE as usize];
            loop {
                if matches!(
                    receiver.read_packet(&mut buffer).await,
                    Err(EndpointError::Disabled)
                ) {
                    receiver.wait_connection().await;
                }
            }
        };

        embassy_futures::join::join(sender_fut, receiver_fut).await;
    }
}

pub fn write_bytes(bytes: &[u8]) {
    let end = bytes.len();

    let mut total = 0;
    while total < end {
        let n = match LOG_BUFFER.try_write(&bytes[total..end]) {
            Ok(n) => n,
            // Pipe full, drop the data.
            Err(_) => return,
        };
        total += n;
    }
}

// No-op, flushing isn't possible with this setup.
pub fn flush() {}
