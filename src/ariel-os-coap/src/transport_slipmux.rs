// FIXME: This does not populate coap_client; probably it even could (but only with weird IP
// address semantics).

use ariel_os_log::{Hex, debug, warn};
use embassy_sync::signal::Signal;

/// Buffer type in which all slipmux traffic is handled (usually through `&'static mut`).
///
/// This is public only when slipmux's user access to its IP traffic is enabled.
// See https://github.com/t2trg/slipmux/issues/1 for expectations on how big this should be
pub type SingleBuffer = heapless::Vec<u8, 1280>;
/// An indirection that, rather than borrowing the content, can also move it out temporarily.
///
/// This behaves largely like a smart pointer, and allows moving out content cheaply (eg. the move
/// writes a word rather moving a whole ~1kB buffer). The expectation is that content gets returned
/// before any further use; violating that leads to unceremoneous panics.
struct MoveOutBuffer(Option<&'static mut SingleBuffer>);
impl MoveOutBuffer {
    fn as_ref(&self) -> &SingleBuffer {
        self.0
            .as_ref()
            .expect("data was moved out and not returned")
    }
    fn as_mut(&mut self) -> &mut SingleBuffer {
        self.0
            .as_mut()
            .expect("data was moved out and not returned")
    }
}
impl embedded_io::ErrorType for MoveOutBuffer {
    type Error = <SingleBuffer as embedded_io::ErrorType>::Error;
}
impl embedded_io::Write for MoveOutBuffer {
    fn write(&mut self, data: &[u8]) -> Result<usize, Self::Error> {
        self.as_mut().write(data)
    }
    fn flush(&mut self) -> Result<(), Self::Error> {
        self.as_mut().flush()
    }
}
impl AsRef<[u8]> for MoveOutBuffer {
    fn as_ref(&self) -> &[u8] {
        Self::as_ref(self).as_ref()
    }
}
type SingleFrameDecoder = slipmux::SingleBufferHandler<MoveOutBuffer>;

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

        static SLIP_AND_USB_BUF: static_cell::StaticCell<(
            SingleBuffer,
            [u8; MAX_FULL_SPEED_PACKET_SIZE as usize],
        )> = static_cell::StaticCell::new();
        let slipmux_and_buffer = SLIP_AND_USB_BUF.init_with(|| (Default::default(), [0; _]));
        let mut slipbuf = &mut slipmux_and_buffer.0;
        let buf = &mut slipmux_and_buffer.1;

        // We don't really need the control channels: USB Serial is back-pressuring anyway.
        let (tx, rx) = class.split();
        let rx = rx.into_buffered(buf);
        let mut as_io = CdcAcmAsIo { tx, rx };

        loop {
            as_io.rx.wait_connection().await;
            debug!("Connected");

            let Err((_e, returned_slipmux)) = serve_slipmux(&mut as_io, slipbuf).await;
            // FIXME: Could there be any *other* errors but disconnect errors that need to be
            // handled differently (i.e., that needs another action than just doing wait_connection
            // again)?

            slipbuf = returned_slipmux;

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
            use embedded_io_async::Read;
            self.rx
                .read(buf)
                .await
                // FIXME: Uncertain conversion
                .map_err(|_| EndpointError::Disabled)
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

    type UartPeripherals = ariel_os_boards::pins::HOST_FACING_UART;

    #[ariel_os_macros::task(autostart, peripherals)]
    async fn pass_on_uart(peripherals: UartPeripherals) -> ! {
        static RX_BUF: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
        static TX_BUF: ConstStaticCell<[u8; 16]> = ConstStaticCell::new([0; 16]);
        let rx_buf = RX_BUF.take();
        let tx_buf = TX_BUF.take();

        let mut config = ariel_os_hal::hal::uart::Config::default();
        config.baudrate = ariel_os_hal::uart::Baudrate::_115200;

        let mut uart = peripherals
            .build_with_config(rx_buf, tx_buf, config)
            .expect("Invalid UART configuration");

        static SLIPBUF: static_cell::StaticCell<SingleBuffer> = static_cell::StaticCell::new();
        let mut slipbuf = SLIPBUF.init_with(Default::default);

        loop {
            let Err((_e, returned_slipbuf)) = serve_slipmux(&mut uart, slipbuf).await;
            slipbuf = returned_slipbuf;
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
    &'static mut SingleBuffer,
> = Signal::new();
/// See [`SLIPMUX2COAP`]; this works the same, just in the opposite direction.
static COAP2SLIPMUX: Signal<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    &'static mut SingleBuffer,
> = Signal::new();

/// Like `SLIPMUX2COAP` but for IP (which is needs to be picked up by the user if enabled)
///
/// Unlike there, the IP side may also take the initiative; if that happened, that buffer is
/// returned empty.
#[cfg(feature = "slipmux-ip")]
pub static SLIPMUX2IP: Signal<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    &'static mut SingleBuffer,
> = Signal::new();
/// Like `COAP2SLIPMUX`, but for IP
///
/// Unlike there, the IP side may also take the initiative; that side has its own buffer, which it
/// may send unprompted.
#[cfg(feature = "slipmux-ip")]
pub static IP2SLIPMUX: Signal<
    embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
    &'static mut SingleBuffer,
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
    slipbuf: &'static mut SingleBuffer,
) -> Result<core::convert::Infallible, (U::Error, &'static mut SingleBuffer)>
where
    U: embedded_io_async::Read + embedded_io_async::Write,
{
    use slipmux::DecodeStatus;

    let mut slipmux =
        SingleFrameDecoder::new_single_buffer(MoveOutBuffer(Some(slipbuf)), |b| b.as_mut().clear());

    // For the time being we play the simple game, where we just have a server, and can afford to
    // read until something has arrived, then write out what has been written, and continue.
    //
    // This will not be sustainable with the next generation coap-handler, and then we may need to
    // split the UARTs.

    let mut decoder = slipmux::Decoder::new();
    loop {
        // FIXME: This is an ugly reading mechanism, made just slightly more tolerable by
        // decoder.decode() taking things bytewise anyway.
        //
        // Once <https://github.com/ariel-os/ariel-os/pull/1600> is resolved, this paragraph should
        // be replaced by the `//BR` comments, which avoid going back and forth with the
        // peripheral.
        let mut bytebuf = [0];
        let read = match embassy_futures::select::select(uart.read(&mut bytebuf), IP2SLIPMUX.wait())
            .await
        {
            embassy_futures::select::Either::First(read) => read.unwrap(),
            embassy_futures::select::Either::Second(ip_buffer) => {
                warn!("Got sometehing to encode!, len {}", ip_buffer.len());
                if !ip_buffer.is_empty() {
                    // Usually it is not, but the tasks may race and the IP task's initiative
                    // message is handled in our initiative side's response … yeah, bigger
                    // bigger refactoring due
                    encode(
                        uart,
                        slipmux::ChunkedEncoder::new(slipmux::FrameType::Ip, ip_buffer),
                    )
                    .await;
                }
                ip_buffer.clear();
                SLIPMUX2IP.signal(ip_buffer);
                continue;
            }
        };
        assert_eq!(read, 1);
        let byte = bytebuf[0];

        //BR let mut buffer = uart.fill_buf().await.unwrap();
        //BR let mut consume = 0;

        let mut coap_ready = false;
        #[cfg(feature = "slipmux-ip")]
        let mut ip_ready = false;

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
                let (buffer, error) = slipmux.read_diagnostic_and_error();
                let text = core::str::from_utf8(buffer.as_ref());
                warn!(
                    "Peer sent diagnostic data. This will no be forwarded; content was {:?}{}",
                    text.map_err(|_| buffer.as_ref().as_ref()),
                    if error.is_some() { "..." } else { "" },
                );
                //BR break;
            }
            Ok(DecodeStatus::FrameCompleteIp) => {
                cfg_select! {
                    feature = "slipmux-ip" => {
                        ip_ready = true;
                        //BR break;
                    },
                    _ => warn!("Peer sent network data, which is unsupported."),
                }
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

            let (bufref, error) = slipmux.read_configuration_and_error();

            if error.is_some() {
                warn!("Ignoring overly long CoAP request.");
                // FIXME: Respond accordingly (Request Entity Too Large and size1)
                continue;
            };

            let mut buffer = bufref.0.take().expect("we're the only party taking this");

            // Compensate for checksum that was already checked inside slipmux
            let Some(actual_length) = buffer.len().checked_sub(2) else {
                warn!("Ignoring overly short CoAP request.");
                continue;
            };
            buffer.truncate(actual_length);

            SLIPMUX2COAP.signal(buffer);
            buffer = COAP2SLIPMUX.wait().await;

            bufref.0 = Some(buffer);

            if bufref.as_ref().is_empty() {
                // FIXME: Send RST if parsable to that point (or make
                // embedded_nal_minimal_coapserver emit one
                warn!("Request was completely unparsable, ignoring.");
            } else {
                encode(
                    uart,
                    slipmux::ChunkedEncoder::new(
                        slipmux::FrameType::Configuration,
                        &bufref.as_ref(),
                    ),
                )
                .await;
            }
        } else {
            // Slightly unfortuante indentation here -- but apparently the grammar doesn't like
            // `#[cfg()] else if`.

            #[cfg(feature = "slipmux-ip")]
            if ip_ready {
                let (bufref, error) = slipmux.read_ip_and_error();

                if error.is_some() {
                    warn!("Ignoring overly long IP packet.");
                    continue;
                };

                let buffer = bufref.0.take().expect("we're the only party taking this");
                SLIPMUX2IP.signal(buffer);
                bufref.0 = Some(IP2SLIPMUX.wait().await);
                if !bufref.as_ref().is_empty() {
                    encode(
                        uart,
                        slipmux::ChunkedEncoder::new(slipmux::FrameType::Ip, &bufref.as_ref()),
                    )
                    .await;
                }
            }
        }
    }
}

async fn encode<U>(uart: &mut U, mut encoder: slipmux::ChunkedEncoder<'_>)
where
    U: embedded_io_async::Read + embedded_io_async::Write,
{
    let mut outbuf = [0; 16];
    loop {
        let size = encoder.encode_chunk(&mut outbuf);
        if size == 0 {
            break;
        }
        uart.write_all(&outbuf[..size]).await.unwrap();
    }
}

pub(crate) async fn coap_run_slipmux(mut handler: impl coap_handler::Handler) -> ! {
    loop {
        let buffer = SLIPMUX2COAP.wait().await;

        debug!("Sending into CoAP stack: {}", Hex(&buffer));
        let mut pseudostack = PseudoStack(buffer);
        embedded_nal_minimal_coapserver::poll(&mut pseudostack, &mut PseudoSocket, &mut handler)
            // No `let Ok(()) = `, because there's the unavoidable nb::Error case.
            .unwrap();

        debug!("Taking out of CoAP stack: {}", Hex(&buffer));

        COAP2SLIPMUX.signal(buffer);
    }
}

/// The CoAP items in slipmux behave 1:1 like in CoAP-over-UDP. Emulating an embedded-nal stack to
/// exchange them so we can funnel the traffic into a CoAP server.
///
/// As CoAP is currently not async and we don't send messages on our own, this now goes into the
/// simpler embedded-nal rather than embedded-nal-async.
struct PseudoStack<'a>(&'a mut SingleBuffer);

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
