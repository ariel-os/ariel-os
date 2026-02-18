#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

mod routes;

use ariel_os::{asynch::Spawner, cell::StaticCell, net, time::Duration};

ariel_os::hal::group_peripherals!(Peripherals {
    #[cfg(feature = "button-reading")]
    buttons: ariel_os_boards::pins::ButtonPeripherals,
});

#[cfg(feature = "button-reading")]
use embassy_sync::once_lock::OnceLock;
use picoserve::AppBuilder;

const HTTP_PORT: u16 = 80;
const WEB_TASK_POOL_SIZE: usize = 2;
const SERVER_CONFIG: picoserve::Config<Duration> = picoserve::Config::new(picoserve::Timeouts {
    start_read_request: Some(Duration::from_secs(5)),
    read_request: Some(Duration::from_secs(1)),
    write: Some(Duration::from_secs(1)),
});

static APP: StaticCell<picoserve::Router<routes::AppRouter>> = StaticCell::new();

#[cfg(feature = "button-reading")]
static BUTTON_INPUT: OnceLock<ariel_os::gpio::Input> = OnceLock::new();

#[ariel_os::task(pool_size = WEB_TASK_POOL_SIZE)]
async fn web_task(task_id: usize, app: &'static picoserve::Router<routes::AppRouter>) -> ! {
    let stack = net::network_stack().await.unwrap();

    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        task_id,
        app,
        &SERVER_CONFIG,
        stack,
        HTTP_PORT,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}

#[cfg(feature = "dhcp-server")]
#[ariel_os::task]
async fn dhcp_server_task() {
    use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    use edge_dhcp::io::{self, DEFAULT_SERVER_PORT};
    use edge_dhcp::server::{Server, ServerOptions};
    use edge_nal::UdpBind;
    use edge_nal_embassy::{Udp, UdpBuffers};

    let mut buf = [0; 1500];
    let udp_socket_buffers = UdpBuffers::<1, 1472, 1472, 2>::new();
    let ip = Ipv4Addr::new(10, 42, 0, 1);
    let mut gw_buf = [Ipv4Addr::UNSPECIFIED];

    let stack = net::network_stack().await.unwrap();
    let udp = Udp::new(stack, &udp_socket_buffers);

    let mut socket = udp
        .bind(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::UNSPECIFIED,
            DEFAULT_SERVER_PORT,
        )))
        .await
        .unwrap();

    io::server::run(
        &mut Server::<_, 64>::new_with_et(ip), // Will give IP addresses in the range 10.42.0.50 - 10.42.0.200, subnet 255.255.255.0
        &ServerOptions::new(ip, Some(&mut gw_buf)),
        &mut socket,
        &mut buf,
    )
    .await
    .unwrap();
}

#[ariel_os::spawner(autostart, peripherals)]
fn main(spawner: Spawner, peripherals: Peripherals) {
    #[cfg(feature = "button-reading")]
    {
        use ariel_os::gpio::{Input, Pull};

        let button = Input::new(peripherals.buttons.button0, Pull::Up);
        let _ = BUTTON_INPUT.init(button);
    }
    #[cfg(not(feature = "button-reading"))]
    // Mark it used even when not.
    let _ = peripherals;

    let app = APP.init_with(|| routes::AppBuilder.build_app());

    #[cfg(feature = "dhcp-server")]
    {
        spawner.spawn(dhcp_server_task()).unwrap();
    }

    for task_id in 0..WEB_TASK_POOL_SIZE {
        spawner.spawn(web_task(task_id, app)).unwrap();
    }
}
