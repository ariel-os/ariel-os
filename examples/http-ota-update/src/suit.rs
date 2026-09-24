use core::cell::RefCell;

use ariel_os::log::{info, error, Debug2Format};


use dress_up::{
    AsyncOperatingHooks,
    component::Component,
    consts::{SuitCommand, SuitParameter},
    error::Error,
    SuitManifest, Authenticated, manifest::Manifest,
};

use cose_nostd::{
    iana::{Algorithm, EllipticCurve, KeyOperation, KeyType, key_labels},
    key::CoseKeyBuilder,
    signature::sign1::CoseSign1,
};

use alloc::vec::Vec;

use uuid::Uuid;

pub const PUBKEY_P256: &[u8; 65] = include_bytes!("../suit/demo-public-key-p256.bin");

pub fn suit_vendor_id() -> Uuid {
    Uuid::new_v5(&Uuid::NAMESPACE_DNS, "http-ota-update".as_bytes())
}

pub fn suit_class_id() -> Uuid {
    Uuid::new_v5(&suit_vendor_id(), "example".as_bytes())
}

// Simple, without support for slots
pub struct  ArielOSUpdateHook {
    // FIXME: use ArielOSUpdateStorage instead
    pub storage: RefCell<Vec<u8>>,
}

impl AsyncOperatingHooks for ArielOSUpdateHook {
    type ReadWriteBufferSize = generic_array::typenum::U512;

    async fn component_capacity(&self, _component: &Component<'_>) -> Result<usize, dress_up::error::Error> {
        Ok(self.storage.borrow().capacity())
    }

    async fn component_size(&self, _component: &Component<'_>) -> Result<usize, dress_up::error::Error> {
        Ok(self.storage.borrow().len())
    }

    async fn match_vendor_id(&self, uuid: uuid::Uuid, _component: &Component<'_>) -> Result<bool, dress_up::error::Error> {
        Ok(uuid == suit_vendor_id())
    }

    async fn match_class_id(&self, uuid: Uuid, _component: &Component<'_>) -> Result<bool, dress_up::error::Error> {
        Ok(uuid == suit_class_id())
    }

    async fn match_component_slot(
        &self,
        _component: &Component<'_>,
        _component_slot: u64,
    ) -> Result<bool, dress_up::error::Error>
    {
        Ok(true)
    }

    async fn component_read(
        &self,
        _component: &Component<'_>,
        _slot: Option<u64>,
        offset: usize,
        bytes: &mut [u8],
    ) -> Result<(), Error>
    {
        let staging = self.storage.borrow();
        let end = offset
            .checked_add(bytes.len())
            .ok_or_else(|| Error::CapacityError)?;

        let src = staging
            .get(offset..end)
            .ok_or_else(|| Error::EndOfInput)?;

        bytes.copy_from_slice(src);
        Ok(())
    }

    async fn component_write(
        &self,
        _component: &Component<'_>,
        _slot: Option<u64>,
        _offset: usize,
        _bytes: &[u8],
    ) -> Result<(), Error>
    {
        Err(Error::UnsupportedCommand { command: SuitCommand::WriteContent.into() })
    }

    async fn fetch(
        &self,
        _component: &Component<'_>,
        _slot: Option<u64>,
        uri: &str,
    ) -> Result<(), Error>
    {
        info!("URI: {}", uri);

        let payload = crate::client::fetch_payload(uri).await.map_err(|_| Error::UnsupportedParameter { parameter: SuitParameter::Uri.into() })?;
        info!("Retrieve payload of size: {}", payload.len());
        let mut storage = self.storage.borrow_mut();
        storage.clear();
        storage.extend(&payload);
        // info!("Retrieved payload: {:?}", Debug2Format(&payload));
        Ok(())
    }
}


pub async fn fetch_and_verify_update(
    manifest: dress_up::manifest::Manifest<'_, dress_up::Authenticated>,
) -> Result<alloc::vec::Vec<u8>, dress_up::error::Error> {

    let hooks = ArielOSUpdateHook { storage: RefCell::new(alloc::vec::Vec::with_capacity(0x4000)) };

    if manifest
        .has_payload_fetch()?
    {
        manifest
            .async_execute_payload_fetch(&hooks)
            .await?;
    }
    if manifest
        .has_payload_installation()?
    {
        manifest
            .async_execute_payload_installation(&hooks)
            .await?;
    }

    if manifest
        .has_image_validation()?
    {
        // Type parameter needed until fix lands upstream
        manifest
            .async_execute_image_validation::<ArielOSUpdateHook>(&hooks)
            .await?;
    }

    Ok(hooks.storage.into_inner())
}


pub fn build_and_authenticate_manifest<'a>(buf: &'a impl AsRef<[u8]>) -> Result<(Manifest<'a, Authenticated>, u64), dress_up::error::Error> {
    let suit = SuitManifest::from_bytes(buf);

    let suit = suit.authenticate(verify_cose_signature)?;

    info!("authenticated the manifest :)");
    let envelope = suit.envelope()?;
    let manifest = envelope.manifest()?;
    info!(
        "Manifest version {} sequence number {}",
        manifest.version()?,
        manifest.sequence_number()?
    );

    let seq = manifest.sequence_number()?;
    Ok((manifest, seq))
}



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
