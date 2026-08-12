//! Generic USB logger.
//! cdc_acm::Sender is !Send so we have to use a Pipe, wiring this in the `writer` module is
//! more complex than just implementing the functions there, this also avoids unnecessary locks.
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use embassy_usb::{
    class::cdc_acm::{CdcAcmClass, CdcAcmError, State},
    driver::EndpointError,
};
use embedded_io_async_07::Write;

use ariel_os_hal::hal::usb::UsbDriver;

const USB_PIPE_SIZE: usize = 1024;
const MAX_USB_PACKET_SIZE: u16 = 64;

static USB_LOG_PIPE: Pipe<CriticalSectionRawMutex, USB_PIPE_SIZE> = Pipe::new();

#[embassy_executor::task]
async fn run(usb_cdc_acm: CdcAcmClass<'static, ariel_os_hal::hal::usb::UsbDriver>) {
    let (mut sender, mut receiver) = usb_cdc_acm.split();
    let sender_fut = async {
        sender.wait_connection().await;
        let mut buffer = [0; USB_PIPE_SIZE as usize];
        loop {
            let len = USB_LOG_PIPE.read(&mut buffer).await;

            if matches!(
                sender.write_all(&buffer[..len]).await,
                Err(CdcAcmError::NotConnected)
            ) {
                sender.wait_connection().await;
            };
        }
    };

    // We need to read from the USB otherwise some hosts won't be able to close the connection.
    let receiver_fut = async {
        receiver.wait_connection().await;
        let mut buffer = [0; MAX_USB_PACKET_SIZE as usize];
        loop {
            if Err(EndpointError::Disabled) == receiver.read_packet(&mut buffer).await {
                receiver.wait_connection().await;
            };
        }
    };

    embassy_futures::join::join(sender_fut, receiver_fut).await;
}

/// Initialize the generic USB transport.
pub fn init(usb_builder: &mut embassy_usb::Builder<'static, UsbDriver>, spawner: Spawner) {
    static CDC_ACM_STATE: static_cell::StaticCell<State<'_>> = static_cell::StaticCell::new();

    // Create classes on the builder.
    let usb_cdc_acm = CdcAcmClass::new(
        usb_builder,
        CDC_ACM_STATE.init_with(State::new),
        MAX_USB_PACKET_SIZE,
    );

    let _ = spawner.spawn(run(usb_cdc_acm));

    ariel_os_log::custom_transport::register_transport_functions(write_bytes, flush);
}

pub fn write_bytes(bytes: &[u8]) {
    let end = bytes.len();

    let mut total = 0;
    while total < end {
        let n = match USB_LOG_PIPE.try_write(&bytes[total..end]) {
            Ok(n) => n,
            // Pipe full, drop the data.
            Err(_) => return,
        };
        total += n;
    }
}

pub fn flush() {}
