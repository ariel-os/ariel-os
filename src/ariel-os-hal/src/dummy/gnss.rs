use embassy_executor::Spawner;

use ariel_os_embassy_common::gnss::{Config, GnssData, GnssDataSender};

#[expect(clippy::unused_async)]
pub async fn single_shot_gnss_fix(_timeout_seconds: u16) -> GnssData {
    unimplemented!();
}

#[expect(clippy::unused_async)]
pub async fn init_gnss(_spawner: Spawner, _sender: GnssDataSender<'static>, _config: Config) {
    unimplemented!();
}
