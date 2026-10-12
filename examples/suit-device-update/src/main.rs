#![no_main]
#![no_std]

use ariel_os::{
    device_update::{DeviceUpdater, DeviceUpdaterState, HalDeviceUpdaterState},
    gpio::{Input, Pull},
    log::*,
};

use ariel_os_suit::SuitProcessor;

use cose_nostd::{
    iana::{Algorithm, EllipticCurve, KeyOperation, KeyType, key_labels},
    key::CoseKeyBuilder,
    signature::sign1::CoseSign1,
};


use ariel_os_boards::pins;

static SUIT_MANIFEST: &[u8] = include_bytes!("../manifest-ariel.cbor");


// --- snip
// --- snip

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::ButtonPeripherals) {
    info!("Wait for button to start update");
    let pull = Pull::Up;
    let mut btn0 = Input::builder(peripherals.button0, pull)
        .build_with_interrupt()
        .unwrap();

    let _ = btn0.wait_for_falling_edge().await;
    info!("Button pressed, initiating update...");

    let mut updater_state = HalDeviceUpdaterState::new();
    let updater = updater_state.updater();

    let suit_processor = SuitProcessor::<_,512, _>::new(&SUIT_MANIFEST, updater);

    let updater = match suit_processor.authenticate(verify_cose_signature) {
        Ok(suit_processor) => {
            match suit_processor.process_full_manifest().await {
                Ok(_) => {
                    Some(suit_processor.into_updater())
                },
                Err(e) => {
                    info!("[SUIT] Manifest Processing failed : {:?}", Debug2Format(&e));
                    None
                }
            }
        },
        Err(e) => {
            info!("[SUIT] Authenticate failed : {:?}", Debug2Format(&e));
            None
        }
    };

    info!("Testing");
    if let Some(mut updater) = updater {

        updater.mark_updated().await.unwrap();

        info!("Updater done.");
        info!("Waiting for another button press to update");
        let _ = btn0.wait_for_falling_edge().await;
        ariel_os::power::reboot();
    }
}


pub const PUBKEY_P256: &[u8; 65] = include_bytes!("../suit/demo-public-key-p256.bin");


fn verify_cose_signature(
    cose_sign1: &[u8],
    detached_payload: &[u8],
) -> Result<bool, dress_up::error::Error> {
    // Expected SEC1 uncompressed form (as in the const above):
    // 0x04 || x[32] || y[32]


    if PUBKEY_P256.len() != 65 || PUBKEY_P256[0] != 0x04 {
        error!("P-256 public key is not uncompressed SEC1 format");
        return Err(dress_up::error::Error::AuthenticationFailure);
    }

    let x = &PUBKEY_P256[1..33];
    let y = &PUBKEY_P256[33..65];

    let mut key_buf = [0u8; 128];

    let verification_key = CoseKeyBuilder::new(key_buf.as_mut_slice(), 6)
        .and_then(|b| {
            b.add_generic_params(
                KeyType::EC2,
                None,
                Some(Algorithm::Es256),
                Some(&[KeyOperation::Verify]),
                None,
            )
        })
        .and_then(|b| b.add_param(key_labels::ec::CRV, EllipticCurve::P256))
        .and_then(|b| b.add_param_bytes(key_labels::ec::X, x))
        .and_then(|b| b.add_param_bytes(key_labels::ec::Y, y))
        .and_then(|b| b.build())
        .map_err(|e| {
            error!(
                "[SUIT] failed to build COSE verification key: {:?}",
                Debug2Format(&e)
            );
            dress_up::error::Error::AuthenticationFailure
        })?;

    let verifier = CoseSign1::from_slice(cose_sign1).map_err(|e| {
        error!("[SUIT] failed to decode COSE_Sign1: {:?}", Debug2Format(&e));
        dress_up::error::Error::AuthenticationFailure
    })?;

    match verifier.verify_detached(detached_payload, &verification_key, None, None) {
        Ok(_) => Ok(true),
        Err(e) => {
            error!(
                "[SUIT] COSE_Sign1 verification failed: {:?}",
                Debug2Format(&e)
            );
            Ok(false)
        }
    }
}
