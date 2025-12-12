// FIXME: This does not populate coap_client; probably it even could (but only with weird IP
// address semantics).

use ariel_os_debug::log::{Debug2Format, Hex, debug, info, warn};
use embassy_sync::once_lock::OnceLock;
use static_cell::{ConstStaticCell, StaticCell};

#[cfg(false)]
mod usb_serial {
    use super::*;

    use ariel_os_embassy::{reexports::embassy_usb, usb::UsbDriver};
    use embassy_usb::{class::cdc_acm, driver::EndpointError};

    const MAX_FULL_SPEED_PACKET_SIZE: u8 = 64;

    #[ariel_os_macros::config(usb)]
    const USB_CONFIG: embassy_usb::Config<'_> = {
        let mut config = embassy_usb::Config::new(0x1209, 0x0009);
        config.manufacturer = Some(ariel_os_buildinfo::OS_NAME);
        config.product = Some("Generic Slipmux board");
        // FIXME pull from device identity
        //config.serial_number = Some("12345678");
        config.max_power = 100;
        config.max_packet_size_0 = MAX_FULL_SPEED_PACKET_SIZE;

        // Required for Windows support.
        config.composite_with_iads = true;
        config.device_class = 0xEF;
        config.device_sub_class = 0x02;
        config.device_protocol = 0x01;
        config
    };

    #[ariel_os_macros::task(autostart, usb_builder_hook)]
    async fn usb_main() {
        static STATE: StaticCell<cdc_acm::State<'_>> = StaticCell::new();

        let mut class = USB_BUILDER_HOOK
            .with(|builder| {
                cdc_acm::CdcAcmClass::new(
                    builder,
                    STATE.init_with(cdc_acm::State::new),
                    MAX_FULL_SPEED_PACKET_SIZE.into(),
                )
            })
            .await;

        let (tx, mut rx, cc) = class.split_with_control();

        static PIPE: static_cell::StaticCell<
            embassy_sync::pipe::Pipe<
                embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                { MAX_FULL_SPEED_PACKET_SIZE as usize },
            >,
        > = static_cell::StaticCell::new();
        let pipe = PIPE.init_with(embassy_sync::pipe::Pipe::new);

        let (pipe_reader, pipe_writer) = pipe.split();

        TAKE_THIS.tset((pipe_reader, rx));

        let mut buf = [0; MAX_FULL_SPEED_PACKET_SIZE as usize];
        loop {
            rx.wait_connection().await;
            info!("Connected");
            let mut decoder = slipmux::Decoder::new();
            let mut handler = DebugHandler;
            loop {
                let n = rx.read_packet(&mut buf).await.unwrap();
                for &byte in &buf[..n] {
                    // Ignoring the return value; handling everything inside the decoder.
                    let _ = decoder.decode(byte, &mut handler);
                }
            }
            info!("Disconnected");
        }
    }

    static TAKE_THIS: once_cell::sync::OnceCell<
        embassy_sync::blocking_mutex::Mutex<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            Option<(
                embassy_sync::pipe::Reader<
                    'static,
                    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
                    { MAX_FULL_SPEED_PACKET_SIZE as usize },
                >,
                cdc_acm::Sender<'static, ariel_os_embassy::usb::UsbDriver>,
            )>,
        >,
    > = once_cell::sync::OnceCell::new();

    pub(super) async fn take() -> impl embedded_io_async::Read + embedded_io_async::Write {}
}

mod over_uart {
    use super::*;

    use ariel_os_embassy::hal::peripherals;
    use ariel_os_hal::{define_peripherals, uart};

    type UartAssignment = ariel_os_boards::pins::HOST_FACING_UART;

    // FIXME There has to be a better idiom for a function deep down to take peripherals that does
    // not involve spawning a task that terminates at startup?

    static TAKE_THIS: OnceLock<core::cell::RefCell<Option<UartAssignment>>> = OnceLock::new();

    #[ariel_os_macros::task(autostart, peripherals)]
    async fn pass_on_uart(peripherals: UartAssignment) {
        TAKE_THIS
            .init(Some(peripherals).into())
            .ok()
            .expect("Autostart is only called once");
    }

    // FIXME: Move to BufRead once <https://github.com/ariel-os/ariel-os/pull/1600> is resolved.
    pub(super) async fn take() -> impl embedded_io_async::Read + embedded_io_async::Write {
        let peripherals: UartAssignment = TAKE_THIS.try_get().unwrap().take().unwrap();

        static RX_BUF: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
        static TX_BUF: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
        let rx_buf = RX_BUF.take();
        let tx_buf = TX_BUF.take();

        let mut config = ariel_os_hal::hal::uart::Config::default();
        config.baudrate = ariel_os_hal::uart::Baudrate::_115200;

        use ariel_os_hal::uart::Assignment;
        let (tx, rx) = peripherals.into_pins();

        let mut uart =
            <<MainUartPins as Assignment>::Device<'_>>::new(rx, tx, rx_buf, tx_buf, config)
                .expect("Invalid UART configuration");

        uart
    }
}

pub(crate) async fn coap_run_slipmux(mut handler: impl coap_handler::Handler) -> ! {
    use embedded_io_async::{Read, Write};
    use slipmux::{DecodeStatus, Decoder};

    // For the time being we play the simple game, where we just have a server, and can afford to
    // read until something has arrived, then write out what has been written, and continue.
    //
    // This will not be sustainable with the next generation coap-handler, and then we may need to
    // split the UARTs.

    //let uart = usb_serial::take().await;
    let mut uart = over_uart::take().await;

    let mut decoder = slipmux::Decoder::new();
    // Small -- just enough so we can show some bytes for diagnostics. (We can do larger if
    // ReferencedLatestFrame learns to use a single buffer).
    let mut diag_buf = [0; 8];
    // Could do less, but don't want to do the math right now.
    let mut coap_buf = [0; 1200];
    // No point in having anything while we don't forward.
    // (Or should we have a few bytes just so we can respond with ICMP No Sorry?)
    let mut packet_buf = [0; 0];
    let mut slipmux =
        slipmux::ReferencedLatestFrame::new(&mut diag_buf, &mut coap_buf, &mut packet_buf);
    loop {
        // FIXME: This is an ugly reading mechanism, made just slightly more tolerable by
        // decoder.decode() taking things bytewise anyway.
        //
        // Once <https://github.com/ariel-os/ariel-os/pull/1600> is resolved, this paragraph should
        // be replaced by the `//BR` comments, which avoid going back and forth with the
        // peripheral.
        let mut bytebuf = [0];
        let read = uart.read(&mut bytebuf).await.unwrap();
        assert_eq!(read, 1);
        let byte = bytebuf[0];

        //BR let mut buffer = uart.fill_buf().await.unwrap();
        //BR let mut consume = 0;

        let mut coap_ready = false;

        //BR while let Some(&byte) = buffer.split_off_first() {
        //BR consume += 1;

        // Ignoring the return value; handling everything inside the decoder.
        match decoder.decode(byte, &mut slipmux) {
            Err(e) => {
                warn!("Decoding error; trying at the next byte.");
                //BR consume = 1;
                //BR break;
            }
            Ok(DecodeStatus::Incomplete) => {
                //BR continue
            }
            Ok(DecodeStatus::FrameCompleteDiagnostic) => {
                // Use up to the cursor, and silently ignore overflows.
                let buffer = slipmux
                    .diagnostic_buffer
                    .get(0..slipmux.index)
                    .unwrap_or(slipmux.diagnostic_buffer);
                let text = core::str::from_utf8(buffer);
                warn!(
                    "Peer sent diagnostic data. This will no be forwarded; content was {:?}",
                    text.map_err(|e| &buffer)
                );
                //BR break;
            }
            Ok(DecodeStatus::FrameCompleteIp) => {
                warn!("Peer sent network data, which is unsupported.",);
                //BR break;
            }
            Ok(DecodeStatus::FrameCompleteConfiguration) => {
                coap_ready = true;
                //BR break;
            }
        }

        //BR }

        //BR uart.consume(consume);

        if coap_ready {
            // Compensate for checksum -- FIXME: where should this best be done?
            slipmux.index -= 2;

            debug!(
                "Sending into CoAP stack: {}",
                Hex(&slipmux.configuration_buffer[..slipmux.index])
            );
            let mut pseudostack = PseudoStack(&mut slipmux);
            embedded_nal_minimal_coapserver::poll(
                &mut pseudostack,
                &mut PseudoSocket,
                &mut handler,
            )
            .unwrap(); // No `let Ok(()) = `, because there's the unavoidable nb::Error case.

            debug!(
                "Taking out of CoAP stack: {}",
                Hex(&slipmux.configuration_buffer[..slipmux.index])
            );

            assert!(
                slipmux.index != 0,
                "minimal handler should have sent a response"
            );
            let message = &slipmux.configuration_buffer[..slipmux.index];
            // FIXME: Try whether larger or smaller makes any difference.
            let mut outbuf = [0; 16];
            let mut encoder =
                slipmux::ChunkedEncoder::new(slipmux::FrameType::Configuration, message);
            loop {
                let size = encoder.encode_chunk(&mut outbuf);
                if size == 0 {
                    break;
                }
                uart.write_all(&outbuf[..size]).await.unwrap();
            }
        }
    }
}

/// The CoAP items in slipmux behave 1:1 like in CoAP-over-UDP. Emulating an embedded-nal stack to
/// exchange them so we can funnel the traffic into a CoAP server.
///
/// As CoAP is currently not async and we don't send messages on our own, this now goes into the
/// simpler embedded-nal rather than embedded-nal-async.
struct PseudoStack<'buf, 'a>(&'a mut slipmux::ReferencedLatestFrame<'buf>);

struct PseudoSocket;

use core::convert::Infallible;
use core::net::SocketAddr;

impl embedded_nal::UdpClientStack for PseudoStack<'_, '_> {
    type UdpSocket = PseudoSocket;
    type Error = Infallible;
    fn socket(&mut self) -> Result<PseudoSocket, Infallible> {
        panic!("Unused");
    }
    fn connect(&mut self, _: &mut PseudoSocket, _: SocketAddr) -> Result<(), Infallible> {
        panic!("Unused");
    }
    fn send(
        &mut self,
        _: &mut PseudoSocket,
        _: &[u8],
    ) -> Result<(), embedded_nal::nb::Error<Infallible>> {
        panic!("Unused");
    }
    fn receive(
        &mut self,
        _: &mut PseudoSocket,
        outbuf: &mut [u8],
    ) -> Result<(usize, SocketAddr), embedded_nal::nb::Error<Infallible>> {
        // FIXME size error handling
        let len = self.0.index;
        // Mostly a precaution against sending the request back if ever the minimal CoAP server
        // should *not* send a single response.
        self.0.index = 0;
        let data = &self.0.configuration_buffer[0..len];
        outbuf[0..len].copy_from_slice(data);
        Ok((
            len,
            SocketAddr::V6(core::net::SocketAddrV6::new(
                core::net::Ipv6Addr::UNSPECIFIED,
                0,
                0,
                0,
            )),
        ))
    }
    fn close(&mut self, _: PseudoSocket) -> Result<(), Infallible> {
        panic!("Unused");
    }
}

impl embedded_nal::UdpFullStack for PseudoStack<'_, '_> {
    fn bind(&mut self, _: &mut PseudoSocket, _: u16) -> Result<(), Self::Error> {
        panic!("Unused")
    }
    fn send_to(
        &mut self,
        _: &mut PseudoSocket,
        _: SocketAddr,
        data: &[u8],
    ) -> Result<(), embedded_nal::nb::Error<Self::Error>> {
        // FIXME size handling

        // FIXME: Abusing the configuration buffer seems like a convenient thing to do, given we're
        // in full control of .0 for the duration of processing the request -- anything more
        // elegant?
        self.0.configuration_buffer[0..data.len()].copy_from_slice(data);
        self.0.index = data.len();
        Ok(())
    }
}
