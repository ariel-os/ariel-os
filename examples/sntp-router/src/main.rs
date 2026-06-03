#![no_main]
#![no_std]

use ariel_os::{
    log::info,
    net,
    time::{Duration, Timer},
};
use ariel_os_sntp::NTP_PORT;
use core::net::{IpAddr, SocketAddr};

#[ariel_os::task(autostart)]
async fn main() {
    loop {
        match ariel_os_sntp::now() {
            Some(now) => info!("Current time: {}s", now.as_secs()),
            None => info!("SNTP clock not synchronized yet"),
        }

        Timer::after_secs(5 * 60).await;
    }
}
