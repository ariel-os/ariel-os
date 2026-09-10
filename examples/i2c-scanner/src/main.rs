#![no_main]
#![no_std]

use ariel_os::{
    hal,
    i2c::controller::{Kilohertz, highest_freq_in},
    log::info,
};

use embedded_hal_async::i2c::I2c;

ariel_os::hal::group_peripherals!(Peripherals {
    i2c0: ariel_os_boards::pins::I2c0,
});

#[ariel_os::task(autostart, peripherals)]
async fn i2c_scanner(peripherals: Peripherals) {
    let mut i2c_config = hal::i2c::controller::Config::default();
    i2c_config.frequency = const { highest_freq_in(Kilohertz::kHz(100)..=Kilohertz::kHz(400)) };

    let mut i2c_bus = peripherals.i2c0.with_config(i2c_config);

    info!("Checking for I2C devices on the bus...");

    for addr in 1..=127 {
        if i2c_bus.write(addr, &[]).await.is_ok() {
            info!("Found device at address 0x{:x}", addr);
        }
    }

    info!("Done checking. Have a great day!");
}
