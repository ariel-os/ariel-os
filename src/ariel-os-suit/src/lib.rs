#![cfg_attr(not(test), no_std)]

#[featurecomb::comb]
mod _featurecomb {}

use uuid::Uuid;

use dress_up::component::Component;
use dress_up::{AsyncOperatingHooks, AuthState, Authenticated, New, SuitManifest};

use ariel_os_device_update::{DeviceUpdater, DeviceUpdaterError, DeviceUpdaterState};

use ariel_os_bootloader_common::{HalDeviceUpdater, HalDeviceUpdaterState};

use ariel_os_embassy::api::net;

const KEYS: &[u8] = include_bytes!("../key_cose_minicbor.cbor");
const MAX_ID_LENGTH: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
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
    TransportError,
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

impl From<ariel_os_update_transports::Error> for Error {
    fn from(value: ariel_os_update_transports::Error) -> Self {
        match value {
            ariel_os_update_transports::Error::InvalidTransportRange => Self::TransportError,
            ariel_os_update_transports::Error::InvalidUri => Self::TransportError,
            ariel_os_update_transports::Error::Transport => Self::TransportError,
        }
    }
}


/// Entry point for the SUIT update procedure.
///
/// This struct receives the bytes that make up the manifest,
/// authenticates the manifest and then processes it,
/// including fetching the payload and writing it of a storage backend (only NVM at this point)
pub struct SuitProcessor<'a, STATE: AuthState, const CHUNK_SIZE: usize, U: DeviceUpdater>
{
    /// The SUIT manifest
    manifest: SuitManifest<'a, STATE>,

    ///
    updater: core::cell::RefCell<U>,
}

impl<'a, const CHUNK_SIZE: usize, U: DeviceUpdater> SuitProcessor<'a, New, CHUNK_SIZE, U> {
    /// Create a new Suit Manifest processor from a cbor-encoded SUIT manifest.
    pub fn new(cbor: &'a impl AsRef<[u8]>, updater: U) -> Self {
        Self {
            manifest: SuitManifest::from_bytes(cbor),
            updater: core::cell::RefCell::new(updater),
        }
    }

    /// Authenticate a new Suit Manifest.
    pub fn authenticate<F, E>(self, authenticate: F) -> Result<SuitProcessor<'a, Authenticated, CHUNK_SIZE, U>, Error>
    where
        F: Fn(&[u8], &[u8]) -> Result<bool, E>
    {
        Ok(SuitProcessor {
            manifest: self.manifest.authenticate(|cose, payload| {
                authenticate(cose, payload).map_err(|_e| dress_up::error::Error::AuthenticationFailure)
            })?,
            updater: self.updater,
        })
    }
}

impl<'a, const CHUNK_SIZE: usize, U: DeviceUpdater> SuitProcessor<'a, Authenticated, CHUNK_SIZE, U> {
    /// Process the manifest, including the fetching and writing into a storage backend.
    pub async fn process_full_manifest(&self) -> Result<(), Error> {
        let envelope = self.manifest.envelope()?;
        let manifest = envelope.manifest()?;

        if let Err(e) = manifest.async_execute_full(self).await {
            return Err(e.into());
        }
        Ok(())
    }

    pub fn into_updater(self) -> U {
        self.updater.into_inner()
    }
}

impl<'a, const CHUNK_SIZE: usize, U: DeviceUpdater> AsyncOperatingHooks for SuitProcessor<'a, Authenticated, CHUNK_SIZE, U> {
    type ReadWriteBufferSize = generic_array::typenum::U64;

    async fn match_vendor_id(
        &self,
        uuid: Uuid,
        component: &dress_up::component::Component<'_>,
    ) -> Result<bool, dress_up::error::Error> {
        Ok(true)
        // todo!()
    }

    async fn match_class_id(
        &self,
        uuid: Uuid,
        component: &dress_up::component::Component<'_>,
    ) -> Result<bool, dress_up::error::Error> {
        Ok(true)
        // todo!()
    }

    async fn component_read(
        &self,
        component: &dress_up::component::Component<'_>,
        slot: Option<u64>,
        offset: usize,
        bytes: &mut [u8],
    ) -> Result<(), dress_up::error::Error> {
        todo!();
        // Ok(())
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

    async fn fetch(
        &self,
        _component: &Component<'_>,
        _slot: Option<u64>,
        uri: &str,
    ) -> Result<(), dress_up::error::Error>
    {
        #[cfg(feature = "http")]
        {
            let stack = net::network_stack().await.unwrap();
            let mut payload_buffer = [0; CHUNK_SIZE];

            let tcp_state = embassy_net::tcp::client::TcpClientState::<1, CHUNK_SIZE, CHUNK_SIZE>::new();
            let tcp_client = embassy_net::tcp::client::TcpClient::new(stack, &tcp_state);

            let dns_client = embassy_net::dns::DnsSocket::new(stack);

            let stream = ariel_os_update_transports::fetch_from_uri(
                &tcp_client,
                &dns_client,
                uri,
                &mut payload_buffer,
                CHUNK_SIZE as u32
            ).await.map_err(|_| dress_up::error::Error::ConditionMatchFail { position: 0 })?;
            let mut offset = 0;
            while let Some(maybe_chunk) = stream.next().await {
                let chunk = maybe_chunk.map_err(|_| dress_up::error::Error::ConditionMatchFail { position: 1 })?;
                self.updater.write_firmware(offset, chunk.bytes()).await.map_err(
                    dress_up::error::Error::ConditionMatchFail { position: 2 }
                );
                offset += chunk.bytes().len();
            }
        }
        #[cfg(feature = "coap")]
        {
            let stack = net::network_stack().await.unwrap();
            let mut payload_buffer = [0; CHUNK_SIZE];

            let dns_client = embassy_net::dns::DnsSocket::new(stack);

            let mut stream: ariel_os_update_transports::FetchStream<'_, '_, CHUNK_SIZE, 64> = ariel_os_update_transports::fetch_from_uri(
                &dns_client,
                uri,
                &mut payload_buffer,
            ).await.map_err(|_| dress_up::error::Error::ConditionMatchFail { position: 0 })?;

            let mut offset = 0;
            while let Some(maybe_chunk) = stream.next().await {
                let chunk = maybe_chunk.map_err(|_|
                    dress_up::error::Error::ConditionMatchFail { position: 1 }
                )?;
                self.updater.borrow_mut().write_firmware(offset, chunk.bytes()).await.map_err(|_|
                    dress_up::error::Error::ConditionMatchFail { position: 2 }
                )?;
                offset += chunk.bytes().len();
            }
        }
        todo!()
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
