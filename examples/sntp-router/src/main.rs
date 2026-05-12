#![no_main]
#![no_std]

use core::net::{IpAddr, SocketAddr};
use ariel_os::{
    log::info,
    net,
    time::{Duration, Timer},
};
use ariel_os_sntp::NTP_PORT;

#[ariel_os::task(autostart)]
async fn main() {
    let stack = net::network_stack().await.unwrap();

    stack.wait_config_up().await;

    let router_ip = stack.config_v4().unwrap().gateway.unwrap();
    let server = SocketAddr::new(IpAddr::from(router_ip), NTP_PORT);

    ariel_os_sntp::start(stack, server, Duration::from_secs(60 * 60)).await;

    info!("Started SNTP sync task for {}", server);

    loop {
        match ariel_os_sntp::now() {
            Some(now) => info!("Current time: {}s", now.as_secs()),
            None => info!("SNTP clock not synchronized yet"),
        }

        Timer::after_secs(10*60).await;
    }
}
