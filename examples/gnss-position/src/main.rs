#![no_main]
#![no_std]

use ariel_os::debug::log::info;
use ariel_os::time::Timer;

#[ariel_os::task(autostart)]
async fn main() {
    let mut receiver = ariel_os::gnss::get_receiver().unwrap();
    Timer::after_millis(1000).await;
    let single_fix = ariel_os::gnss::request_gnss_fix().await;
    info!("Single GPS Fix: {:?}", single_fix);
    loop {
        let fix = receiver.changed().await;
        info!("GPS Fix: {:?}", fix);
    }
}
