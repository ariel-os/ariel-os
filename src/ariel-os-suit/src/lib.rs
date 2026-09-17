#![cfg_attr(not(test), no_std)]

use dress_up::manifest::Manifest;
use dress_up::{AuthState, Authenticated, New, SuitManifest};

#[derive(Debug)]
enum Error {}

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
        self.authenticate()
    }
}

impl<'a> SuitProcessor<'a, Authenticated> {}
