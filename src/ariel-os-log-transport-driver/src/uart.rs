use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, pipe::Pipe};
use embedded_io_async::Write;
use static_cell::ConstStaticCell;

use ariel_os_embassy_common::uart::Baudrate;
use ariel_os_hal::hal::{OptionalPeripherals, TakePeripherals, uart::Uart};

type UartAssignment = ariel_os_boards::pins::HOST_FACING_UART;

const UART_PIPE_SIZE: usize = 1024;
const UART_BUF_SIZE: usize = 32;

static UART_LOG_PIPE: Pipe<CriticalSectionRawMutex, { UART_PIPE_SIZE }> = Pipe::new();

static RX_BUF: ConstStaticCell<[u8; UART_BUF_SIZE]> = ConstStaticCell::new([0u8; UART_BUF_SIZE]);
static TX_BUF: ConstStaticCell<[u8; UART_BUF_SIZE]> = ConstStaticCell::new([0u8; UART_BUF_SIZE]);

static UART: Mutex<CriticalSectionRawMutex, Option<Uart<'static>>> = Mutex::new(None);

/// Initialize UART log transport.
pub fn init(mut peripherals: &mut OptionalPeripherals, spawner: Spawner) {
    let mut config = ariel_os_hal::hal::uart::Config::default();
    config.baudrate = Baudrate::_115200;

    let tx_buf = TX_BUF.take();
    let rx_buf = RX_BUF.take();

    let assignment: UartAssignment = peripherals.take_peripherals();

    let uart = assignment
        .build_with_config(rx_buf, tx_buf, config)
        .unwrap();

    spawner.spawn(run().expect("start UART log task"));

    {
        let mut guard = UART.try_lock().unwrap();
        guard.replace(uart);
    }

    ariel_os_log::transport::register_custom_transport(write_bytes, flush);
    ariel_os_log::transport::register_finish_fn(finish);
}

// This is called before a power event (`exit`, `reboot`).
fn finish() {
    // TODO: figure out why `run` doesn't give back the lock when doing a `embassy_futures::yield_now().await;` just before exiting
    if let Ok(mut guard) = UART.try_lock()
        && let Some(mut uart) = guard.take()
    {
        embassy_futures::block_on(async {
            let mut buf = [0u8; UART_BUF_SIZE];
            loop {
                // Read until empty.
                let len = match UART_LOG_PIPE.try_read(&mut buf) {
                    Ok(len) => len,
                    // Error means empty.
                    Err(_) => break,
                };
                let _ = uart.write_all(&buf[..len]).await;
                let _ = uart.flush().await;
            }
        })
    }
}

#[embassy_executor::task]
async fn run() {
    let mut buf = [0u8; UART_BUF_SIZE];
    loop {
        let len = UART_LOG_PIPE.read(&mut buf).await;
        {
            let mut guard = UART.lock().await;
            if let Some(mut uart) = guard.take() {
                let _ = uart.write_all(&buf[..len]).await;
                let _ = uart.flush().await;

                guard.replace(uart);
            }
        }
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
