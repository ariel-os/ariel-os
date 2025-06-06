#![no_main]
#![no_std]

use ariel_os::debug::log::info;
use embassy_futures::join::join;
use embassy_time::{Duration, Timer};

use trouble_host::prelude::*;

#[ariel_os::task(autostart)]
async fn run_advertisement() {
    info!("starting ble stack");
    let mut stack = ariel_os::ble::ble_stack().await.build();

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

    let _ = join(stack.runner.run(), async {
        loop {
            let mut params = AdvertisementParameters::default();

            params.interval_min = Duration::from_millis(100);
            params.interval_max = Duration::from_millis(100);

            let _advertiser = stack
                .peripheral
                .advertise(
                    &params,
                    Advertisement::NonconnectableScannableUndirected {
                        adv_data: &adv_data[..len],
                        scan_data: &[],
                    },
                )
                .await;

            loop {
                info!("Still running");

                Timer::after(Duration::from_secs(60)).await;
            }
        }
    })
    .await;
}
