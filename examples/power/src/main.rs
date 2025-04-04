#![no_main]
#![no_std]

use ariel_os::{
    debug::log::info,
    power,
    time::{Duration, Timer},
};

#[ariel_os::task(autostart)]
async fn main() {
    info!("Rebooting in 3 s");

    Timer::after(Duration::from_secs(3)).await;
    power::reboot();
}
