use core::net::IpAddr;

use embassy_executor::Spawner;
#[cfg(feature = "ipv6")]
use embassy_net::Ipv6Cidr;
use embassy_net::{Ipv4Cidr, Stack};
use heapless::Vec;
use nrf_modem::embassy_net_modem::{
    NetDriver, Runner, State,
    context::{self, PdConfig, PdnAuth, PdpType},
};
use static_cell::StaticCell;

pub use nrf_modem::embassy_net_modem::context::{AuthProt, Status};

use ariel_os_embassy_common::cellular_networking;

pub type NetworkDevice = NetDriver<'static>;

static LTEM_STATE: StaticCell<State> = StaticCell::new();
static LTEM_CONTROL: StaticCell<context::Control<'static>> = StaticCell::new();

// Packet Data Protocol context id, range 0-10. On nrf9160 only cid 0 is already allocated.
const PDP_CONTEXT_ID: u8 = 0;

#[embassy_executor::task]
async fn modem_task(runner: Runner<'static>) -> ! {
    runner.run().await
}

fn convert_cellular_networking_config(
    config: cellular_networking::PdConfig<'static>,
) -> PdConfig<'static> {
    let apn = config.apn.map(str::as_bytes);
    let pdn_auth = config.pdn_auth.map(|auth_config| {
        let auth_prot = match auth_config.authentication_protocol {
            cellular_networking::AuthenticationProtocol::None => AuthProt::None,
            cellular_networking::AuthenticationProtocol::Pap => AuthProt::Pap,
            cellular_networking::AuthenticationProtocol::Chap => AuthProt::Chap,
        };

        let auth = auth_config
            .credentials
            .map(|c| (c.username.as_bytes(), c.password.as_bytes()));
        PdnAuth { auth, auth_prot }
    });

    let pdp_type = match config.pdp_type {
        cellular_networking::PdpType::Ip => PdpType::Ip,
        cellular_networking::PdpType::Ipv6 => PdpType::Ipv6,
        cellular_networking::PdpType::Ipv4v6 => PdpType::Ipv4v6,
        cellular_networking::PdpType::NonIp => PdpType::NonIp,
    };

    PdConfig {
        apn,
        pdn_auth,
        pdp_type,
    }
}

/// Task responsible of maintaining the connection status up to date.
/// Also configures the modem when starting (if a `config` is provided)
/// # Panics
///
/// When the configuration is invalid.
#[embassy_executor::task]
pub async fn control_task(
    control: &'static context::Control<'static>,
    config: cellular_networking::PdConfig<'static>,
    pin: Option<&'static str>,
    stack: Stack<'static>,
) {
    control
        .configure(
            &convert_cellular_networking_config(config),
            pin.map(str::as_bytes),
        )
        .await
        .unwrap();

    control
        .run(|status| {
            let config = status_to_config(status);

            #[cfg(feature = "ipv4")]
            stack.set_config_v4(config.ipv4);

            #[cfg(feature = "ipv6")]
            stack.set_config_v6(config.ipv6);
        })
        .await
        .unwrap();
}

/// Creates an embassy-net config from a modem status update.
///
/// # Panics
/// This will never panic, the size of Vec from embassy-net is larger than the
/// size of the Vec from nrf-modem.
#[must_use]
fn status_to_config(status: &Status) -> embassy_net::Config {
    #[cfg(feature = "ipv4")]
    let v4_gateway = match status.gateway {
        Some(IpAddr::V4(addr)) => Some(addr),
        _ => None,
    };

    #[cfg(feature = "ipv6")]
    let v6_gateway = match status.gateway {
        Some(IpAddr::V6(addr)) => Some(addr),
        _ => None,
    };

    #[cfg(feature = "ipv4")]
    let v4_address = match status.ip1 {
        Some(IpAddr::V4(addr)) => Some(addr),
        _ => None,
    };

    #[cfg(feature = "ipv6")]
    let v6_address = if let Some(IpAddr::V6(addr)) = status.ip1 {
        Some(addr)
    } else if let Some(IpAddr::V6(addr)) = status.ip2 {
        Some(addr)
    } else {
        None
    };

    #[cfg(feature = "ipv4")]
    let mut v4_dns_servers = Vec::new();
    #[cfg(feature = "ipv6")]
    let mut v6_dns_servers = Vec::new();
    for dns in &status.dns {
        #[allow(unused, reason = "conditional compilation")]
        match dns {
            IpAddr::V4(ip) => {
                // `embassy_net::StaticConfigV4` stores up to 3 DNS addresses,
                // `nrf_modem::embassy_net_modem::context::Status` contains a maximum of 2.
                // PANICS: This will never panic
                #[cfg(feature = "ipv4")]
                v4_dns_servers.push(*ip).unwrap();
            }
            IpAddr::V6(ip) => {
                // `embassy_net::StaticConfigV6` stores up to 3 DNS addresses,
                // `nrf_modem::embassy_net_modem::context::Status` contains a maximum of 2.
                // PANICS: This will never panic
                #[cfg(feature = "ipv6")]
                v6_dns_servers.push(*ip).unwrap();
            }
        }
    }

    let mut config = embassy_net::Config::default();

    #[cfg(feature = "ipv4")]
    if let Some(addr) = v4_address {
        config.ipv4 = embassy_net::ConfigV4::Static(embassy_net::StaticConfigV4 {
            // This is a point to point connection, the modem never gives a subnet mask.
            // `local_addr_and_subnet_mask` is not supported by the AT command +CGCONTRDP.
            // In the nRF SDK no mask is used. A /32 CIDR is the best equivalent to a
            // Zephyr NET_ADDR_MANUAL with no set mask.
            address: Ipv4Cidr::new(addr, 32),
            gateway: v4_gateway,
            dns_servers: v4_dns_servers,
        });
    }

    #[cfg(feature = "ipv6")]
    if let Some(addr) = v6_address {
        config.ipv6 = embassy_net::ConfigV6::Static(embassy_net::StaticConfigV6 {
            // This is a point to point connection, the modem never gives a subnet mask,
            // `local_addr_and_subnet_mask` is not supported by the AT command +CGCONTRDP.
            // In the nRF SDK no mask is used. A /128 CIDR is the best equivalent to a
            // Zephyr NET_ADDR_MANUAL with no set mask.
            address: Ipv6Cidr::new(addr, 128),
            gateway: v6_gateway,
            dns_servers: v6_dns_servers,
        });
    }
    config
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
