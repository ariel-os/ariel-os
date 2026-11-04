use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
#[allow(unused, reason = "nrf doesn't use it yet")]
use embedded_io::Write;

use ariel_os_hal::hal::{OptionalPeripherals, TakePeripherals};

type UartAssignment = ariel_os_boards::pins::HOST_FACING_UART;

static UART: Mutex<CriticalSectionRawMutex, Option<UartDriver>> = Mutex::new(None);

#[cfg(context = "esp")]
type UartDriver = esp_hal::uart::Uart<'static, esp_hal::Blocking>;
#[cfg(context = "nrf")]
type UartDriver = embassy_nrf::uarte::Uarte<'static>;
#[cfg(context = "rp")]
type UartDriver = embassy_rp::uart::Uart<'static, embassy_rp::uart::Blocking>;
#[cfg(context = "stm32")]
type UartDriver = embassy_stm32::usart::Uart<'static, embassy_stm32::mode::Blocking>;

/// Initialize UART log transport.
pub fn init(mut peripherals: &mut OptionalPeripherals, _spawner: Spawner) {
    let assignment: UartAssignment = peripherals.take_peripherals();

    let (tx, rx, peri) = assignment.retrieve_peripherals();

    let uart = cfg_select! {

        context = "nrf" => {
            {
                let mut config = embassy_nrf::uarte::Config::default();
                config.baudrate = embassy_nrf::buffered_uarte::Baudrate::Baud115200;

                embassy_nrf::uarte::Uarte::new(peri,rx,tx,UartAssignment::irq(),config)
            }

        }
        context = "esp" =>  {
            {
                let config = esp_hal::uart::Config::default().with_baudrate(115200);

                esp_hal::uart::Uart::new(peri,config).unwrap().with_tx(tx)
            }
        }
        context = "rp" => {
            {
                let mut config = embassy_rp::uart::Config::default();

                config.baudrate = 115200;
                embassy_rp::uart::Uart::new_blocking(peri, tx, rx, config)
            }
        }
        context = "stm32" => {
            {
                let mut config = embassy_stm32::usart::Config::default();

                config.baudrate = 115200;
                embassy_stm32::usart::Uart::new_blocking(peri, rx, tx, config).unwrap()
            }
        }
        _ => {
            compile_error!("logging-over-uart is not supported on this platform.")
        }
    };
    {
        let mut guard = UART.try_lock().unwrap();
        guard.replace(uart);
    }

    ariel_os_log::transport::register_custom_transport(write_bytes, flush);
}

fn write_bytes(bytes: &[u8]) {
    if let Ok(mut guard) = UART.try_lock() {
        if let Some(mut uart) = guard.take() {
            cfg_select! {
               context = "nrf" => {
                   let _ = uart.blocking_write(bytes);
                }
                _=> {
                    let _ = uart.write_all(bytes);
                    let _ = uart.flush();
                }
            }
            guard.replace(uart);
        }
    }
}

fn flush() {
    cfg_select! {
        context = "nrf" => {
            // nrf doesn't have a flush.
        }
        _ => {
            if let Ok(mut guard) = UART.try_lock() {
                if let Some(mut uart) = guard.take() {
                        let _ = uart.flush();
                }
                guard.replace(uart);
            }
        }
    }
}
