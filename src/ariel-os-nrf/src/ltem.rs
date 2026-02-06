use core::net::IpAddr;

use embassy_executor::Spawner;
use embassy_net::{Ipv4Cidr, Stack};
use heapless::Vec;
use nrf_modem::embassy_net_modem::{NetDriver, Runner, State, context};
use static_cell::StaticCell;

pub use nrf_modem::embassy_net_modem::context::{AuthProt, Config, Status};

use ariel_os_embassy_common::sim_card;

pub type NetworkDevice = NetDriver<'static>;

static LTEM_STATE: StaticCell<State> = StaticCell::new();
static LTEM_CONTROL: StaticCell<context::Control<'static>> = StaticCell::new();

// Packet Data Protocol context id, range 0-10
const PDP_CONTEXT_ID: u8 = 0;

#[embassy_executor::task]
async fn modem_task(runner: Runner<'static>) -> ! {
    runner.run().await
}

fn convert_sim_card_config(config: sim_card::Config<'static>) -> Config<'static> {
    let apn = config.apn.as_bytes();
    let auth_prot = match config.authentication_protocol {
        sim_card::AuthenticationProtocol::None => AuthProt::None,
        sim_card::AuthenticationProtocol::Pap => AuthProt::Pap,
        sim_card::AuthenticationProtocol::Chap => AuthProt::Chap,
    };
    let auth = config
        .credentials
        .map(|c| (c.username.as_bytes(), c.password.as_bytes()));
    let pin = config.pin.map(|p| p.as_bytes());
    Config {
        apn,
        auth_prot,
        auth,
        pin,
    }
}

/// Task responsible of maintaining the connection status up to date.
/// Also configures the modem when starting (if a `config` is provided )
#[embassy_executor::task]
pub async fn control_task(
    control: &'static context::Control<'static>,
    config: Option<sim_card::Config<'static>>,
    stack: Stack<'static>,
) {
    if let Some(config) = config {
        control
            .configure(&convert_sim_card_config(config))
            .await
            .unwrap();
    }

    control
        .run(|status| {
            stack.set_config_v4(status_to_config(status));
        })
        .await
        .unwrap();
}

/// Creates an embassy-net IPv4 config from a modem status update.
///
/// # Panics
/// Panics if the modem returns an invalid IPv4 address or too many DNS servers are returned.
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

    let control = LTEM_CONTROL.init(context::Control::new(control, PDP_CONTEXT_ID).await);

    (driver, control)
}
