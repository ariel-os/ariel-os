use core::net::IpAddr;

use embassy_executor::Spawner;
use embassy_net::{Ipv4Cidr, Stack};
use heapless::Vec;
use nrf_modem::embassy_net_modem::{NetDriver, Runner, State, context};
use static_cell::StaticCell;

pub use nrf_modem::embassy_net_modem::context::{AuthProt, Config, Status};

pub type NetworkDevice = NetDriver<'static>;

static LTEM_STATE: StaticCell<State> = StaticCell::new();
static LTEM_CONTROL: StaticCell<context::Control<'static>> = StaticCell::new();

#[embassy_executor::task]
async fn modem_task(runner: Runner<'static>) -> ! {
    runner.run().await
}

/// Task responsible of maintaining the connection status up to date.
/// Also configures the modem when starting (if a `config` is provided )
#[embassy_executor::task]
pub async fn control_task(
    control: &'static context::Control<'static>,
    config: Option<Config<'static>>,
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

/// Create an embassy-net  IPv4 config from an modem status update.
///
/// # Panics
/// Panics if the modem returns an invalid IPv4 or too many dns servers are returned.
#[must_use]
fn status_to_config(status: &Status) -> embassy_net::ConfigV4 {
    let Some(IpAddr::V4(addr)) = status.ip else {
        panic!("Unexpected IP address");
    };

    let gateway = match status.gateway {
        Some(IpAddr::V4(addr)) => Some(addr),
        _ => None,
    };

    let mut dns_servers = Vec::new();
    for dns in &status.dns {
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

/// Initializes the modem for LTE-M networking.
/// The control task needs to be spawned using [`control_task`].
///
/// # Panics
/// If the modem task cannot be spawned.
#[must_use]
pub async fn init<'a>(spawner: Spawner) -> (NetworkDevice, &'a context::Control<'a>) {
    let (driver, control, runner) =
        nrf_modem::embassy_net_modem::new(LTEM_STATE.init(State::new())).await;

    spawner.spawn(modem_task(runner)).unwrap();

    let control = LTEM_CONTROL.init(context::Control::new(control, 0).await);

    (driver, control)
}
