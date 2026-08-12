use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pipe::Pipe};
use embedded_io_async_06::Write;
use static_cell::ConstStaticCell;

use ariel_os_embassy_common::uart::{Assignment, Baudrate};
use ariel_os_hal::hal::{OptionalPeripherals, TakePeripherals, uart::Uart};

type UartAssignment = ariel_os_boards::pins::HOST_FACING_UART;

const UART_PIPE_SIZE: usize = 1024;
const UART_BUF_SIZE: usize = 32;

static UART_LOG_PIPE: Pipe<CriticalSectionRawMutex, { UART_PIPE_SIZE }> = Pipe::new();

static RX_BUF: ConstStaticCell<[u8; UART_BUF_SIZE]> = ConstStaticCell::new([0u8; UART_BUF_SIZE]);
static TX_BUF: ConstStaticCell<[u8; UART_BUF_SIZE]> = ConstStaticCell::new([0u8; UART_BUF_SIZE]);

/// Initialize UART log transport.
pub fn init(mut peripherals: &mut OptionalPeripherals, spawner: Spawner) {
    let mut config = ariel_os_hal::hal::uart::Config::default();
    config.baudrate = Baudrate::_115200;

    let tx_buf = TX_BUF.take();
    let rx_buf = RX_BUF.take();

    let assignment: UartAssignment = peripherals.take_peripherals();

    let (tx, rx) = assignment.into_pins();

    let uart = <UartAssignment as Assignment>::Device::new(rx, tx, rx_buf, tx_buf, config).unwrap();

    spawner.spawn(run(uart)).expect("start UART log task");

    ariel_os_log::custom_transport::register_transport_functions(write_bytes, flush);
}

#[embassy_executor::task]
async fn run(mut uart: Uart<'static>) {
    let mut buf = [0u8; UART_BUF_SIZE];
    loop {
        let len = UART_LOG_PIPE.read(&mut buf).await;
        let _ = uart.write_all(&buf[..len]).await;
    }
}

fn write_bytes(bytes: &[u8]) {
    let end = bytes.len();

    let mut total = 0;
    while total < end {
        let n = match UART_LOG_PIPE.try_write(&bytes[total..end]) {
            Ok(n) => n,
            // Pipe full, drop the data.
            Err(_) => return,
        };
        total += n;
    }
}

fn flush() {}
