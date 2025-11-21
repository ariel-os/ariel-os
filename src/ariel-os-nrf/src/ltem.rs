use core::net::IpAddr;
use embassy_executor::Spawner;
use embassy_net::{Ipv4Cidr, Stack};
use heapless::Vec;
use nrf_modem::embassy_net_modem::NetDriver;
use nrf_modem::embassy_net_modem::{Runner, State, context};
use static_cell::StaticCell;

pub use nrf_modem::embassy_net_modem::context::{AuthProt, Config, Status};
pub type NetworkDevice = NetDriver<'static>;

#[embassy_executor::task]
async fn modem_task(runner: Runner<'static>) -> ! {
    runner.run().await
}

#[embassy_executor::task]
pub async fn control_task(
    control: &'static context::Control<'static>,
    config: Option<context::Config<'static>>,
    stack: Stack<'static>,
) {
    if let Some(config) = config {
        control.configure(&config).await.unwrap();
    }

    control
        .run(|status| {
            stack.set_config_v4(status_to_config(status));
        })
        .await
        .unwrap();
}
pub fn status_to_config(status: &Status) -> embassy_net::ConfigV4 {
    let Some(IpAddr::V4(addr)) = status.ip else {
        panic!("Unexpected IP address");
    };

    let gateway = match status.gateway {
        Some(IpAddr::V4(addr)) => Some(addr),
        _ => None,
    };

    let mut dns_servers = Vec::new();
    for dns in status.dns.iter() {
        if let IpAddr::V4(ip) = dns {
            dns_servers.push(*ip).unwrap();
        }
    }

    embassy_net::ConfigV4::Static(embassy_net::StaticConfigV4 {
        address: Ipv4Cidr::new(addr, 32),
        gateway,
        dns_servers,
    })
}

pub async fn init<'a>(spawner: Spawner) -> (NetworkDevice, &'a context::Control<'a>) {
    static STATE: StaticCell<State> = StaticCell::new();
    let (driver, control, runner) =
        nrf_modem::embassy_net_modem::new(STATE.init(State::new())).await;

    spawner.spawn(modem_task(runner)).unwrap();

    static CONTROL: StaticCell<context::Control<'static>> = StaticCell::new();
    let control = CONTROL.init(context::Control::new(control, 0).await);

    (driver, control)
}
