// FIXME: This does not populate coap_client; probably it even could (but only with weird IP
// address semantics).

use ariel_os_debug::log::{Hex, debug, warn};
use embassy_sync::once_lock::OnceLock;

mod over_uart {
    use ariel_os_embassy::cell::SameExecutorCell;
    use static_cell::ConstStaticCell;

    use super::*;

    type UartAssignment = ariel_os_boards::pins::HOST_FACING_UART;

    // FIXME There has to be a better idiom for a function deep down to take peripherals that does
    // not involve spawning a task that terminates at startup?
    //
    // (Therefore, not bothering with optimizing any of its access)
    static TAKE_THIS: OnceLock<SameExecutorCell<core::cell::RefCell<Option<UartAssignment>>>> =
        OnceLock::new();

    #[ariel_os_macros::task(autostart, peripherals)]
    async fn pass_on_uart(peripherals: UartAssignment) {
        TAKE_THIS
            .init(SameExecutorCell::new_async(Some(peripherals).into()).await)
            .ok()
            .expect("Autostart is only called once");
    }

    // FIXME: Move to BufRead once <https://github.com/ariel-os/ariel-os/pull/1600> is resolved.
    pub(super) async fn take() -> impl embedded_io_async::Read + embedded_io_async::Write {
        let peripherals: UartAssignment = TAKE_THIS
            .try_get()
            .unwrap()
            .get_async()
            .await
            .unwrap()
            .take()
            .unwrap();

        static RX_BUF: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
        static TX_BUF: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
        let rx_buf = RX_BUF.take();
        let tx_buf = TX_BUF.take();

        let mut config = ariel_os_hal::hal::uart::Config::default();
        config.baudrate = ariel_os_hal::uart::Baudrate::_115200;

        use ariel_os_hal::uart::Assignment;
        let (tx, rx) = peripherals.into_pins();

        <UartAssignment as Assignment>::Device::new(rx, tx, rx_buf, tx_buf, config)
            .expect("Invalid UART configuration")
    }
}

pub(crate) async fn coap_run_slipmux(mut handler: impl coap_handler::Handler) -> ! {
    use embedded_io_async::{Read, Write};
    use slipmux::DecodeStatus;

    // For the time being we play the simple game, where we just have a server, and can afford to
    // read until something has arrived, then write out what has been written, and continue.
    //
    // This will not be sustainable with the next generation coap-handler, and then we may need to
    // split the UARTs.

    let mut uart = over_uart::take().await;

    let mut decoder = slipmux::Decoder::new();
    let mut slipmux = SingleFrameDecoder::default();
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
            embedded_nal_minimal_coapserver::poll(
                &mut pseudostack,
                &mut PseudoSocket,
                &mut handler,
            )
            .unwrap(); // No `let Ok(()) = `, because there's the unavoidable nb::Error case.

            debug!("Taking out of CoAP stack: {}", Hex(&buffer));

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
