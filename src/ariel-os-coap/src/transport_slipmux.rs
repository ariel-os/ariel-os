// FIXME: This does not populate coap_client; probably it even could (but only with weird IP
// address semantics).

use ariel_os_debug::log::{Hex, debug, warn};
use embassy_sync::signal::Signal;

#[cfg(feature = "coap-transport-slipmux-usb")]
mod usb_serial {
    use static_cell::StaticCell;

    use super::*;

    use ariel_os_embassy::reexports::embassy_usb;
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

        let class = USB_BUILDER_HOOK
            .with(|builder| {
                cdc_acm::CdcAcmClass::new(
                    builder,
                    STATE.init_with(cdc_acm::State::new),
                    MAX_FULL_SPEED_PACKET_SIZE.into(),
                )
            })
            .await;

        static SLIPMUX_AND_BUFFER: static_cell::StaticCell<(
            SingleFrameDecoder,
            [u8; MAX_FULL_SPEED_PACKET_SIZE as usize],
        )> = static_cell::StaticCell::new();
        let slipmux_and_buffer = SLIPMUX_AND_BUFFER.init_with(|| (Default::default(), [0; _]));
        let mut slipmux = &mut slipmux_and_buffer.0;
        let buf = &mut slipmux_and_buffer.1;

        // We don't really need the control channels: USB Serial is back-pressuring anyway.
        let (tx, rx) = class.split();
        let rx = rx.into_buffered(buf);
        let mut as_io = CdcAcmAsIo { tx, rx };

        loop {
            as_io.rx.wait_connection().await;
            debug!("Connected");

            let Err((_e, returned_slipmux)) = serve_slipmux(&mut as_io, slipmux).await;
            // FIXME: Could there be any *other* errors but disconnect errors that need to be
            // handled differently (i.e., that needs another action than just doing wait_connection
            // again)?

            slipmux = returned_slipmux;

            debug!("Disconnected");
        }
    }

    use embassy_usb::driver::Driver;

    struct CdcAcmAsIo<'d, D: Driver<'d>> {
        tx: cdc_acm::Sender<'d, D>,
        rx: cdc_acm::BufferedReceiver<'d, D>,
    }

    impl<'d, D: Driver<'d>> embedded_io_async::ErrorType for CdcAcmAsIo<'d, D> {
        type Error = EndpointError;
    }
    impl<'d, D: Driver<'d>> embedded_io_async::Read for CdcAcmAsIo<'d, D> {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            self.rx.read(buf).await
        }
    }
    impl<'d, D: Driver<'d>> embedded_io_async::Write for CdcAcmAsIo<'d, D> {
        async fn write(&mut self, mut buf: &[u8]) -> Result<usize, Self::Error> {
            let orig_len = buf.len();
            let max = self.tx.max_packet_size() as usize;
            while !buf.is_empty() {
                let to_write = buf.len().min(max);
                self.tx.write_packet(&buf[..to_write]).await?;
                buf = &buf[to_write..];
                if to_write == 64 && buf.len() == 0 {
                    // FIXME:
                    // * Determine where this requirement comes from (IIRC it was somewhere in
                    //   CdcAcmClass but I don't find it any more)
                    // * Can we uphold this with less write_all'ish behavior?
                    self.tx.write_packet(&[]).await?;
                }
            }
            Ok(orig_len)
        }
    }
}

#[cfg(feature = "coap-transport-slipmux-uart")]
mod over_uart {
    use static_cell::ConstStaticCell;

    use super::*;

    type UartAssignment = ariel_os_boards::pins::HOST_FACING_UART;

    #[ariel_os_macros::task(autostart, peripherals)]
    async fn pass_on_uart(peripherals: UartAssignment) -> ! {
        static RX_BUF: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
        static TX_BUF: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
        let rx_buf = RX_BUF.take();
        let tx_buf = TX_BUF.take();

        let mut config = ariel_os_hal::hal::uart::Config::default();
        config.baudrate = ariel_os_hal::uart::Baudrate::_115200;

        use ariel_os_hal::uart::Assignment;
        let (tx, rx) = peripherals.into_pins();

        let mut uart = <UartAssignment as Assignment>::Device::new(rx, tx, rx_buf, tx_buf, config)
            .expect("Invalid UART configuration");

        static SLIPMUX: static_cell::StaticCell<SingleFrameDecoder> =
            static_cell::StaticCell::new();
        let mut slipmux = SLIPMUX.init_with(Default::default);

        loop {
            let Err((_e, returned_slipmux)) = serve_slipmux(&mut uart, slipmux).await;
            slipmux = returned_slipmux;
            warn!("UART error, restarting Slipmux engine.");
        }
    }
}

/// Signal in which [`server_slipmux`] places the decoder buffer if it contains anything which CoAP
/// should process. The buffer is returned through `COAP2SLIPMUX` to avoid that the task that
/// places the buffer takes it back immediately without waiting for the other task.
///
/// For optimization, we could look into whether there is a way to yield to that other task, but
/// then we'd have to handle the case when that other task is not active yet, and just
/// yielding-for-everyone-else still returns us the unmodified buffer.
///
/// As there is only a single `&'static mut SingleFrameDecoder`, Signal's overwrite semantics don't
/// concern us: We don't have anything to overwrite the latest value with, unless it was taken out
/// and returned before.
static SLIPMUX2COAP: Signal<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    &'static mut SingleFrameDecoder,
> = Signal::new();
/// See [`SLIPMUX2COAP`]; this works the same, just in the opposite direction.
static COAP2SLIPMUX: Signal<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    &'static mut SingleFrameDecoder,
> = Signal::new();

/// Decodes frames from a slipmux peer, and sends them to the relevant tasks for further
/// processing, receiving frames in return.
///
/// In particular, whenever a CoAP frame is ready, it is signaled in [`SLIPMUX2COAP`] (to be
/// picked up by [`coap_run_slipmux()`], however that is called (autostarted or by the user once
/// the handler is ready), and this task is then idle until the CoAP response is returned through
/// [`COAP2SLIPMUX`].
///
/// When this module starts doing SLIP as well, the task will treat the network stack as a
/// signaling peer just like the CoAP stack.
///
/// This is auto-started by whichever module provides the slipmux back-end from that module's
/// autostart task. To recover well from errors, this does not allocate the static `slipmux` itself;
/// the calling task is asked to create that, eg. using:
///
/// ```
/// static SLIPMUX: static_cell::StaticCell<SingleFrameDecoder> = static_cell::StaticCell::new();
/// let mut slipmux = SLIPMUX.init_with(Default::default);
/// ```
///
/// # Errors
///
/// Along with the actual error, this also returns the passed-in slipmux. This is necessary because
/// this function may temporarily move out that buffer (e.g. to have it handled by CoAP): It can't
/// practically return with an error until it has gotten that item back. That is not a practical
/// inconvenience: UART errors only occur when reading from or writing to the UART, and for that to
/// happen, this function needs the buffer anyway.
async fn serve_slipmux<U>(
    uart: &mut U,
    mut slipmux: &'static mut SingleFrameDecoder,
) -> Result<core::convert::Infallible, (U::Error, &'static mut SingleFrameDecoder)>
where
    U: embedded_io_async::Read + embedded_io_async::Write,
{
    use slipmux::DecodeStatus;

    // For the time being we play the simple game, where we just have a server, and can afford to
    // read until something has arrived, then write out what has been written, and continue.
    //
    // This will not be sustainable with the next generation coap-handler, and then we may need to
    // split the UARTs.

    *slipmux = Default::default();

    let mut decoder = slipmux::Decoder::new();
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
        match decoder.decode(byte, slipmux) {
            Err(_) => {
                warn!("Decoding error; trying at the next byte.");
                //BR consume = 1;
                //BR break;
            }
            Ok(DecodeStatus::Incomplete) => {
                //BR continue
            }
            Ok(DecodeStatus::FrameCompleteDiagnostic) => {
                // Use up to the cursor, and silently ignore overflows.
                let (Ok(buffer) | Err(buffer)) = slipmux.data();
                let text = core::str::from_utf8(buffer);
                warn!(
                    "Peer sent diagnostic data. This will no be forwarded; content was {:?}{}",
                    text.map_err(|_| &buffer),
                    if slipmux.data().is_err() { "..." } else { "" },
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
            // If SLIP support is added, and CoAP becomes optional, this might detect at build time
            // that there is no CoAP that we should wait (indefinitely) for, and instead just send
            // the courtesy RSTs.
            //
            // If setting up the CoAP stack ever takes long (and maybe will even deadlock with
            // networking), this might start issuing RSTs (meh: peer might think they're permanent)
            // or 5.03 Max-Age: 1 responses.

            SLIPMUX2COAP.signal(slipmux);
            slipmux = COAP2SLIPMUX.wait().await;

            let buffer = &slipmux.buffer;

            if buffer.len() == 0 {
                // FIXME: Send RST if parsable to that point (or make
                // embedded_nal_minimal_coapserver emit one
                warn!("Request was completely unparsable, ignoring.");
            } else {
                let mut outbuf = [0; 16];
                let mut encoder =
                    slipmux::ChunkedEncoder::new(slipmux::FrameType::Configuration, &buffer);
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
}

pub(crate) async fn coap_run_slipmux(mut handler: impl coap_handler::Handler) -> ! {
    loop {
        let slipmux: &'static mut SingleFrameDecoder = SLIPMUX2COAP.wait().await;

        if slipmux.data().is_err() {
            warn!("Ignoring overly long CoAP request.");
            // FIXME: Respond accordingly (Request Entity Too Large and size1)
            continue;
        };
        // Reaching into raw value rather than through accessor because we need it mutable for
        // the pseudostack.
        let buffer = &mut slipmux.buffer;
        // Compensate for checksum -- FIXME: where should this best be done?
        let Some(actual_length) = buffer.len().checked_sub(2) else {
            warn!("Ignoring overly short CoAP request.");
            continue;
        };
        buffer.truncate(actual_length);

        debug!("Sending into CoAP stack: {}", Hex(&buffer));
        let mut pseudostack = PseudoStack(buffer);
        embedded_nal_minimal_coapserver::poll(&mut pseudostack, &mut PseudoSocket, &mut handler)
            // No `let Ok(()) = `, because there's the unavoidable nb::Error case.
            .unwrap();

        debug!("Taking out of CoAP stack: {}", Hex(&buffer));

        COAP2SLIPMUX.signal(slipmux);
    }
}

struct SingleFrameDecoder {
    // See https://github.com/t2trg/slipmux/issues/1 for expectations on how big this should be
    buffer: heapless::Vec<u8, 1280>,
    overflow: bool,
}

impl SingleFrameDecoder {
    /// Returns the decoded data if complete.
    ///
    /// # Errors
    ///
    /// If the buffer overflew, it returns the initial decoded bytes.
    fn data(&self) -> Result<&[u8], &[u8]> {
        if self.overflow {
            Err(&self.buffer)
        } else {
            Ok(&self.buffer)
        }
    }
}

impl Default for SingleFrameDecoder {
    fn default() -> Self {
        SingleFrameDecoder {
            buffer: Default::default(),
            overflow: false,
        }
    }
}

impl slipmux::FrameHandler for SingleFrameDecoder {
    fn begin_frame(&mut self, _: slipmux::FrameType) {
        self.buffer.clear();
        self.overflow = false;
    }
    fn write_byte(&mut self, byte: u8) {
        if self.buffer.push(byte).is_err() {
            self.overflow = true;
        }
    }
    fn end_frame(&mut self, _: Option<slipmux::Error>) {}
}

/// The CoAP items in slipmux behave 1:1 like in CoAP-over-UDP. Emulating an embedded-nal stack to
/// exchange them so we can funnel the traffic into a CoAP server.
///
/// As CoAP is currently not async and we don't send messages on our own, this now goes into the
/// simpler embedded-nal rather than embedded-nal-async.
struct PseudoStack<'a>(&'a mut heapless::Vec<u8, 1280>);

struct PseudoSocket;

#[derive(Debug)]
struct DoesNotFit;

use core::net::SocketAddr;

impl embedded_nal::UdpClientStack for PseudoStack<'_> {
    type UdpSocket = PseudoSocket;
    type Error = DoesNotFit;
    fn socket(&mut self) -> Result<PseudoSocket, DoesNotFit> {
        panic!("Unused");
    }
    fn connect(&mut self, _: &mut PseudoSocket, _: SocketAddr) -> Result<(), DoesNotFit> {
        panic!("Unused");
    }
    fn send(
        &mut self,
        _: &mut PseudoSocket,
        _: &[u8],
    ) -> Result<(), embedded_nal::nb::Error<DoesNotFit>> {
        panic!("Unused");
    }
    fn receive(
        &mut self,
        _: &mut PseudoSocket,
        outbuf: &mut [u8],
    ) -> Result<(usize, SocketAddr), embedded_nal::nb::Error<DoesNotFit>> {
        let len = self.0.len();
        let outslice = outbuf.get_mut(0..len).ok_or(DoesNotFit)?;
        outslice.copy_from_slice(self.0);

        // Mostly a precaution against sending the request back if ever the minimal CoAP server
        // should *not* send a single response.
        self.0.truncate(0);

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
    fn close(&mut self, _: PseudoSocket) -> Result<(), DoesNotFit> {
        panic!("Unused");
    }
}

impl embedded_nal::UdpFullStack for PseudoStack<'_> {
    fn bind(&mut self, _: &mut PseudoSocket, _: u16) -> Result<(), Self::Error> {
        panic!("Unused")
    }
    fn send_to(
        &mut self,
        _: &mut PseudoSocket,
        _: SocketAddr,
        data: &[u8],
    ) -> Result<(), embedded_nal::nb::Error<Self::Error>> {
        // FIXME: Abusing the configuration buffer seems like a convenient thing to do, given we're
        // in full control of .0 for the duration of processing the request -- anything more
        // elegant?

        self.0.clear();
        self.0.extend_from_slice(data).map_err(|_| DoesNotFit)?;
        Ok(())
    }
}
