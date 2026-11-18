#![no_main]
#![no_std]

use ariel_os::{log::info, power, time::Timer};
use ariel_os_boards as _;

#[ariel_os::task(autostart)]
async fn main() {
    info!("Rebooting in 3 s");

    Timer::after_secs(3).await;
    power::reboot();
}
