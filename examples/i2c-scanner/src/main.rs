#![no_main]
#![no_std]

use ariel_os::{
    hal,
    i2c::controller::{Kilohertz, highest_freq_in},
    log::info,
};

use embedded_hal_async::i2c::I2c;

use ariel_os_boards::pins;
use ariel_os::i2c::BusPart;

#[ariel_os::task(autostart, peripherals)]
async fn i2c_scanner(peripherals: pins::i2c0) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };

    let (sda, scl) = peripherals.into_pins();
    let mut i2c_bus = <pins::i2c0 as BusPart>::I2CPeri::new(sda, scl, i2c_config);

    info!("Checking for I2C devices on the bus...");

    for addr in 1..=127 {
        if i2c_bus.write(addr, &[]).await.is_ok() {
            info!("Found device at address 0x{:x}", addr);
        }
    }

    info!("Done checking. Have a great day!");
}
