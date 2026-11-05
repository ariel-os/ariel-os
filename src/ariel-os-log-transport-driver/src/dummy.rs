// Dummy definition of a transport, this only registers empty functions.

use embassy_executor::Spawner;

use ariel_os_hal::hal::OptionalPeripherals;

fn write_bytes(_bytes: &[u8]) {}

fn flush() {}

/// Initializes the custom transport driver.
pub fn init(_peripherals: &mut OptionalPeripherals, _spawner: Spawner) {
    ariel_os_log::transport::register_custom_transport(write_bytes, flush);
}
