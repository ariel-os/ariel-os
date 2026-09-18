#![cfg_attr(not(test), no_std)]

use ariel_os::log::*;

use dress_up::manifest::Manifest;
use dress_up::{AsyncOperatingHooks, AuthState, Authenticated, New, SuitManifest};

const KEYS: &[u8] = include_bytes!("../key_cose_minicbor.cbor");

#[derive(Debug)]
enum Error {
    AuthenticationFailure,
}

impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}

impl core::error::Error for Error {}

struct SuitProcessor<'a, STATE: AuthState> {
    manifest: SuitManifest<'a, STATE>,
}

impl<'a> SuitProcessor<'a, New> {
    /// Create a new Suit Manifest processor from a cbor-encoded SUIT manifest.
    pub fn new(cbor: &'a impl AsRef<[u8]>) -> Self {
        Self {
            manifest: SuitManifest::from_bytes(cbor),
        }
    }

    /// Authenticate a new Suit Manifest.
    pub fn authenticate(&self) -> Result<SuitProcessor<'a, Authenticated>, Error> {
        Ok(SuitProcessor {
            manifest: self.manifest.authenticate(|cose, payload| {
                let cose_untagged = cose
                    .get(1..)
                    .ok_or(dress_up::error::Error::UnexpectedCbor { position: 0 })?;
                let sign1: cose_minicbor::cose::CoseSign1<'_> = minicbor::decode(cose_untagged)
                    .map_err(|_| dress_up::error::Error::UnexpectedCbor { position: 0 })?;
                let res = sign1.suit_verify_cose_sign1(Some(payload), KEYS);

                //let sign1 = CoseSign1::from_bytes(cs).unwrap();
                //let key = CoseKey::from_bytes(KEY).unwrap();
                //let res = sign1.verify_detached(payload, &key, Some(Algorithm::Esp256), None);
                if let Err(e) = res {
                    error!("Cose error: {:?} ", e);
                    return Err(dress_up::error::Error::AuthenticationFailure);
                }
                Ok(true)
            })?,
        })
    }
}

impl<'a> SuitProcessor<'a, Authenticated> {
    pub async fn process_full_manifest(&self) -> Result<(), Error> {
        let envelope = self.manifest.envelope()?;
        let manifest = envelope.manifest()?;

        // check sequence number
        info!(
            "manifest version {}, sequence number {}",
            manifest.version()?,
            manifest.sequence_number()?
        );

        if let Err(e) = manifest.async_execute_full(self).await {
            error!("Could not process manifest: {}", e);
            return Err(e.into());
        }
        debug!("Completed manifest processing");
        Ok(())
    }
}

impl AsyncOperatingHooks for SuitProcessor<'a, Authenticated> {
    type ReadWriteBufferSize = generic_array::typenum::U64;

    async fn match_vendor_id(
        &self,
        uuid: Uuid,
        component: &dress_up::component::Component,
    ) -> Result<bool, dress_up::error::Error> {
        todo!()
    }

    async fn match_class_id(
        &self,
        uuid: Uuid,
        component: &dress_up::component::Component,
    ) -> Result<bool, dress_up::error::Error> {
        todo!()
    }

    async fn component_read(
        &self,
        component: &dress_up::component::Component,
        slot: Option<u64>,
        offset: usize,
        bytes: &mut [u8],
    ) -> Result<(), dress_up::error::Error> {
        todo!()
    }

    async fn component_write(
        &self,
        component: &dress_up::component::Component,
        slot: Option<u64>,
        offset: usize,
        bytes: &[u8],
    ) -> Result<(), dress_up::error::Error> {
        todo!()
    }

    async fn component_size(
        &self,
        component: &dress_up::component::Component,
    ) -> Result<usize, dress_up::error::Error> {
        todo!()
    }

    async fn component_capacity(
        &self,
        component: &dress_up::component::Component,
    ) -> Result<usize, dress_up::error::Error> {
        todo!()
    }
}
