#![cfg_attr(not(test), no_std)]

use uuid::Uuid;

use dress_up::component::Component;
use dress_up::{AsyncOperatingHooks, AuthState, Authenticated, New, SuitManifest};

const KEYS: &[u8] = include_bytes!("../key_cose_minicbor.cbor");
const MAX_ID_LENGTH: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
enum Error {
    AuthenticationFailure,
    InvalidManifestStructure,
    InvalidManifestSequence { position: usize },
    ConditionMatchFail { position: usize },
    MissingCommandSection { section: i16 },
    ManifestProcessingError,
    MissingParameter { position: usize },
    InvalidDirective { identifier: u32 },
    UnsupportedDigestAlgorithm { algorithm: i32 },
    UnsupportedComponent { identifier: i64 },
    UnsupportedParameter { parameter: i32 },
}

impl core::fmt::Display for Error {
    fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            _ => todo!(),
        }
    }
}

impl core::error::Error for Error {}

impl From<dress_up::error::Error> for Error {
    fn from(value: dress_up::error::Error) -> Self {
        match value {
            dress_up::error::Error::AuthenticationFailure => Self::AuthenticationFailure,
            dress_up::error::Error::CapacityError => Self::ManifestProcessingError,
            dress_up::error::Error::ConditionMatchFail { position } => {
                Self::ConditionMatchFail { position }
            }
            dress_up::error::Error::TryEachFail { position } => {
                Self::ConditionMatchFail { position }
            }
            dress_up::error::Error::EndOfInput => Self::ManifestProcessingError,
            dress_up::error::Error::InvalidAuthenticationStructure => Self::ManifestProcessingError,
            dress_up::error::Error::InvalidCommandSequence { position } => {
                Self::InvalidManifestSequence { position }
            }
            dress_up::error::Error::InvalidCommonSection => Self::InvalidManifestStructure,
            dress_up::error::Error::NoAuthObject => Self::InvalidManifestStructure,
            dress_up::error::Error::NoCommonSection => Self::InvalidManifestStructure,
            dress_up::error::Error::NoCommandSection { section } => {
                Self::MissingCommandSection { section }
            }
            dress_up::error::Error::NoComponentList => Self::InvalidManifestStructure,
            dress_up::error::Error::NoManifestObject => Self::InvalidManifestStructure,
            dress_up::error::Error::NoManifestVersion => Self::InvalidManifestStructure,
            dress_up::error::Error::NoSequenceNumber => Self::InvalidManifestStructure,
            dress_up::error::Error::ParameterNotSet { position } => {
                Self::MissingParameter { position }
            }
            dress_up::error::Error::SameSourceAndTarget { identifier } => {
                Self::InvalidDirective { identifier }
            }
            dress_up::error::Error::InvalidSourceComponent { identifier } => {
                Self::InvalidDirective { identifier }
            }
            dress_up::error::Error::UnexpectedCbor { .. } => Self::InvalidManifestStructure,
            dress_up::error::Error::UnexpectedIndefiniteLength { .. } => {
                Self::InvalidManifestStructure
            }
            dress_up::error::Error::UnsupportedCommand { command } => Self::InvalidDirective {
                identifier: command as u32,
            },
            dress_up::error::Error::UnsupportedComponentIdentifier { identifier } => {
                Self::UnsupportedComponent { identifier }
            }
            dress_up::error::Error::UnsupportedDigestAlgo { algorithm } => {
                Self::UnsupportedDigestAlgorithm {
                    algorithm: algorithm as i32,
                }
            }
            dress_up::error::Error::UnsupportedManifestVersion => Self::InvalidManifestStructure,
            dress_up::error::Error::UnsupportedParameter { parameter } => {
                Self::UnsupportedParameter { parameter }
            }
            dress_up::error::Error::Utf8Error { .. } => Self::InvalidManifestStructure,
        }
    }
}

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
    pub fn authenticate(self) -> Result<SuitProcessor<'a, Authenticated>, Error> {
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

        if let Err(e) = manifest.async_execute_full(self).await {
            return Err(e.into());
        }
        Ok(())
    }
}

impl<'a> AsyncOperatingHooks for SuitProcessor<'a, Authenticated> {
    type ReadWriteBufferSize = generic_array::typenum::U64;

    async fn match_vendor_id(
        &self,
        uuid: Uuid,
        component: &dress_up::component::Component<'_>,
    ) -> Result<bool, dress_up::error::Error> {
        todo!()
    }

    async fn match_class_id(
        &self,
        uuid: Uuid,
        component: &dress_up::component::Component<'_>,
    ) -> Result<bool, dress_up::error::Error> {
        todo!()
    }

    async fn component_read(
        &self,
        component: &dress_up::component::Component<'_>,
        slot: Option<u64>,
        offset: usize,
        bytes: &mut [u8],
    ) -> Result<(), dress_up::error::Error> {
        todo!()
    }

    async fn component_write(
        &self,
        component: &dress_up::component::Component<'_>,
        slot: Option<u64>,
        offset: usize,
        bytes: &[u8],
    ) -> Result<(), dress_up::error::Error> {
        todo!()
    }

    async fn component_size(
        &self,
        component: &dress_up::component::Component<'_>,
    ) -> Result<usize, dress_up::error::Error> {
        todo!()
    }

    async fn component_capacity(
        &self,
        component: &dress_up::component::Component<'_>,
    ) -> Result<usize, dress_up::error::Error> {
        todo!()
    }
}

fn storage_backend(
    component: Component<'_>,
) -> Result<ariel_os_update_storage::StorageBackend, Error> {
    let Some(Ok(first)) = component.iter_segments()?.next() else {
        return Err(Error::ManifestProcessingError);
    };
    const NVM: &[u8] = "nvm".as_bytes();
    match first {
        NVM => Ok(ariel_os_update_storage::StorageBackend::Nvm),
        _ => Err(Error::ManifestProcessingError),
    }
}

fn build_id(
    component: Component<'_>,
    slot: Option<u64>,
) -> Result<heapless::Vec<u8, MAX_ID_LENGTH>, Error> {
    let slot_num = match slot {
        Some(slot) => Some(
            u8::try_from(slot)
                .map_err(|_| Error::InvalidManifestStructure)?
                .to_be_bytes(),
        ),
        None => None,
    };
    let mut id = heapless::Vec::new();
    // skip the backend type
    for segment in component.iter_segments()?.skip(1) {
        id.extend_from_slice(segment?)
            .map_err(|_| Error::InvalidManifestStructure)?; // todo: needs a separate error
    }
    if let Some(slot_num) = slot_num {
        id.extend_from_slice(&slot_num)
            .map_err(|_| Error::InvalidManifestStructure)?; // todo: needs a separate error
    }
    Ok(id)
}
