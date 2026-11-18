//! Adapted from the example in `trouble_host`.
#![no_main]
#![no_std]

use ariel_os::{log::info, reexports::embassy_time, time::Timer};
use ariel_os_boards as _;
use embassy_futures::join::join;
use embassy_time::Duration;
use trouble_host::advertise::{
    AdStructure, Advertisement, AdvertisementParameters, BR_EDR_NOT_SUPPORTED,
    LE_GENERAL_DISCOVERABLE,
};

#[ariel_os::task(autostart)]
async fn run_advertisement() {
    info!("starting ble stack");
    let stack = ariel_os::ble::ble_stack().await;
    let mut host = stack.build();

    info!("Using address: {}", ariel_os::ble::current_address().await);

    let mut adv_data = [0; 31];

    let len = AdStructure::encode_slice(
        &[
            AdStructure::CompleteLocalName(b"Ariel OS BLE"),
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
        ],
        &mut adv_data[..],
    )
    .unwrap();

    info!("Starting advertising");

    let _ = join(host.runner.run(), async {
        let params = AdvertisementParameters {
            interval_min: Duration::from_millis(100),
            interval_max: Duration::from_millis(100),
            ..Default::default()
        };

        let _advertiser = host
            .peripheral
            .advertise(
                &params,
                Advertisement::NonconnectableScannableUndirected {
                    adv_data: adv_data.get(..len).unwrap(),
                    scan_data: &[],
                },
            )
            .await;

        loop {
            info!("Still running");
            Timer::after_secs(60).await;
        }
    })
    .await;
}
