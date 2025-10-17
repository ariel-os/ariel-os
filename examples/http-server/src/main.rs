#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]

mod pins;
mod routes;

use ariel_os::{asynch::Spawner, cell::StaticCell, net, time::Duration};

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

#[ariel_os::spawner(autostart, peripherals)]
fn main(spawner: Spawner, peripherals: pins::Peripherals) {
    #[cfg(feature = "button-reading")]
    {
        use ariel_os::gpio::{Input, Pull};

        let button = Input::new(peripherals.button.btn1, Pull::Up);
        let _ = BUTTON_INPUT.init(button);
    }
    #[cfg(not(feature = "button-reading"))]
    // Mark it used even when not.
    let _ = peripherals;

    let app = APP.init_with(|| routes::AppB.build_app());

    for task_id in 0..WEB_TASK_POOL_SIZE {
        spawner.spawn(web_task(task_id, app)).unwrap();
    }
}
