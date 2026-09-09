use embassy_executor::Spawner;

use ariel_os_hal::hal::OptionalPeripherals;

fn write_bytes(_bytes: &[u8]) {}

fn flush() {}

/// Initialize the custom transport driver.
pub fn init(_peripherals: &mut OptionalPeripherals, _spawner: Spawner) {
    ariel_os_log::custom_transport::register_transport_functions(write_bytes, flush);
}
