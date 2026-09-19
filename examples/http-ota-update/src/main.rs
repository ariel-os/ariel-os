#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
#![recursion_limit = "256"]

mod client;
mod routes;
mod suit;

use ariel_os::{asynch::Spawner, cell::StaticCell, net};

extern crate alloc;

use picoserve::AppBuilder;

const HTTP_PORT: u16 = 80;
const WEB_TASK_POOL_SIZE: usize = 1;
const SERVER_CONFIG: picoserve::Config = {
    use picoserve::time::Duration;

    picoserve::Config::new(picoserve::Timeouts {
        start_read_request: Duration::from_secs(5),
        persistent_start_read_request: Duration::from_secs(5),
        read_request: Duration::from_secs(1),
        write: Duration::from_secs(1),
    })
};

static APP: StaticCell<picoserve::Router<routes::AppRouter>> = StaticCell::new();

#[ariel_os::task(pool_size = WEB_TASK_POOL_SIZE)]
async fn web_task(task_id: usize, app: &'static picoserve::Router<routes::AppRouter>) -> ! {
    let stack = net::network_stack().await.unwrap();

    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    loop {
        let server = picoserve::Server::new(app, &SERVER_CONFIG, &mut http_buffer);

        let _ = server
            .listen_and_serve(
                task_id,
                stack,
                HTTP_PORT,
                &mut tcp_rx_buffer,
                &mut tcp_tx_buffer,
            )
            .await;
    }
}

#[ariel_os::spawner(autostart)]
fn main(spawner: Spawner) {
    let app = APP.init_with(|| routes::AppBuilder.build_app());

    for task_id in 0..WEB_TASK_POOL_SIZE {
        spawner.spawn(web_task(task_id, app)).unwrap();
    }
}
