#![cfg_attr(not(test), no_std)]

#[featurecomb::comb]
mod _featurecomb {}

use uuid::Uuid;

use dress_up::component::Component;
use dress_up::consts::SuitCommand;
use dress_up::{AsyncOperatingHooks, AuthState, Authenticated, New, SuitManifest};

use ariel_os_device_update::DeviceUpdater;

use ariel_os_embassy::api::net;

// const MAX_ID_LENGTH: usize = 16;

/// Entry point for the SUIT update procedure.
///
/// This struct receives the bytes that make up the manifest,
/// authenticates the manifest and then processes it,
/// including fetching the payload and writing it of a storage backend (only NVM at this point).
pub struct SuitProcessor<'a, STATE: AuthState, const CHUNK_SIZE: usize, U> {
    /// The SUIT manifest.
    manifest: SuitManifest<'a, STATE>,

    /// Updater responsible for writing and reading the update components.
    updater: core::cell::RefCell<U>,

    /// Size of the component described by the manifest.
    component_size: core::cell::RefCell<usize>,
}

impl<'a, const CHUNK_SIZE: usize, U> SuitProcessor<'a, New, CHUNK_SIZE, U> {
    /// Create a new Suit Manifest processor from a cbor-encoded SUIT manifest.
    pub fn new(cbor: &'a impl AsRef<[u8]>, updater: U) -> Self {
        Self {
            manifest: SuitManifest::from_bytes(cbor),
            updater: core::cell::RefCell::new(updater),
            component_size: core::cell::RefCell::new(0),
        }
    }

    /// Authenticate a new Suit Manifest.
    ///
    /// # Errors
    ///
    /// When the function given as an argument errors.
    ///
    pub fn authenticate<F, E>(
        self,
        authenticate: F,
    ) -> Result<SuitProcessor<'a, Authenticated, CHUNK_SIZE, U>, dress_up::error::Error>
    where
        F: Fn(&[u8], &[u8]) -> Result<bool, E>,
    {
        Ok(SuitProcessor {
            manifest: self.manifest.authenticate(|cose, payload| {
                authenticate(cose, payload)
                    .map_err(|_e| dress_up::error::Error::AuthenticationFailure)
            })?,
            updater: self.updater,
            component_size: self.component_size,
        })
    }
}

impl<const CHUNK_SIZE: usize, U: DeviceUpdater>
    SuitProcessor<'_, Authenticated, CHUNK_SIZE, U>
{
    /// Process the manifest, including the fetching and writing into a storage backend.
    ///
    /// # Errors
    ///
    /// If any of the steps that occur when processing the manifest fail.
    ///
    pub async fn process_full_manifest(&self) -> Result<(), dress_up::error::Error> {
        let envelope = self.manifest.envelope()?;
        let manifest = envelope.manifest()?;

        let sequence_number = manifest.sequence_number()?;

        ariel_os_log::trace!("Found Manifest with sequence Number: {}", sequence_number);

        if manifest.has_payload_fetch()? {
            manifest.async_execute_payload_fetch(self).await?;
        }
        if manifest.has_payload_installation()? {
            manifest.async_execute_payload_installation(self).await?;
        }

        if manifest.has_image_validation()? {
            manifest.async_execute_image_validation(self).await?;
        }
        Ok(())
    }

    pub fn into_updater(self) -> U {
        self.updater.into_inner()
    }
}

impl<const CHUNK_SIZE: usize, U: DeviceUpdater> AsyncOperatingHooks
    for SuitProcessor<'_, Authenticated, CHUNK_SIZE, U>
{
    type ReadWriteBufferSize = generic_array::typenum::U64;

    #[allow(clippy::unused_async_trait_impl)]
    async fn match_vendor_id(
        &self,
        _uuid: Uuid,
        _component: &dress_up::component::Component<'_>,
    ) -> Result<bool, dress_up::error::Error> {
        Ok(true)
        // todo!()
    }

    #[allow(clippy::unused_async_trait_impl)]
    async fn match_class_id(
        &self,
        _uuid: Uuid,
        _component: &dress_up::component::Component<'_>,
    ) -> Result<bool, dress_up::error::Error> {
        Ok(true)
        // todo!()
    }

    #[allow(clippy::await_holding_refcell_ref)]
    async fn component_read(
        &self,
        _component: &dress_up::component::Component<'_>,
        _slot: Option<u64>,
        offset: usize,
        bytes: &mut [u8],
    ) -> Result<(), dress_up::error::Error> {

        self.updater
            .borrow_mut()
            .read_firmware(u32::try_from(offset).unwrap(), bytes)

            .await
            .map_err(|_e| dress_up::error::Error::ConditionMatchFail {
                command: SuitCommand::ImageMatch,
                position: 0,
            })
    }

    #[allow(clippy::unused_async_trait_impl)]
    async fn component_write(
        &self,
        _component: &dress_up::component::Component<'_>,
        _slot: Option<u64>,
        _offset: usize,
        _bytes: &[u8],
    ) -> Result<(), dress_up::error::Error> {
        Err(dress_up::error::Error::UnsupportedCommand {
            command: SuitCommand::WriteContent,
        })
    }

    #[allow(clippy::unused_async_trait_impl)]
    async fn component_size(
        &self,
        _component: &dress_up::component::Component<'_>,
    ) -> Result<usize, dress_up::error::Error> {
        ariel_os_log::trace!("{}", *self.component_size.borrow());
        Ok(*self.component_size.borrow())
    }

    #[allow(clippy::unused_async_trait_impl)]
    async fn component_capacity(
        &self,
        _component: &dress_up::component::Component<'_>,
    ) -> Result<usize, dress_up::error::Error> {
        Ok(usize::MAX)
    }

    #[allow(clippy::unused_async_trait_impl)]
    async fn match_component_slot(
        &self,
        _component: &Component<'_>,
        _component_slot: u64,
    ) -> Result<bool, dress_up::error::Error> {
        Ok(true)
    }

    #[allow(clippy::await_holding_refcell_ref)]
    async fn fetch(
        &self,
        _component: &Component<'_>,
        slot: Option<u64>,
        uri: &str,
    ) -> Result<(), dress_up::error::Error> {
        ariel_os_log::trace!("Fetching component in slot {:?} at uri {}", slot, uri);

        // FIXME: find a new way to
        let maximum_capacity = u32::MAX;

        let stack = net::network_stack().await.unwrap();

        let dns_client = embassy_net::dns::DnsSocket::new(stack);

        let mut chunk = [0u8; CHUNK_SIZE];

        #[cfg(feature = "http")]
        let tcp_state = embassy_net::tcp::client::TcpClientState::<1, CHUNK_SIZE, CHUNK_SIZE>::new();
        #[cfg(feature = "http")]
        let tcp_client = embassy_net::tcp::client::TcpClient::new(stack, &tcp_state);

        let mut stream = cfg_select! {
            feature = "http" => {
                ariel_os_update_transports::fetch_from_uri::<_,_, CHUNK_SIZE, CHUNK_SIZE>(
                    &tcp_client,
                    &dns_client,
                    uri,
                    &mut chunk,
                    maximum_capacity
                ).await
            },
            feature = "coap" => {
                ariel_os_update_transports::fetch_from_uri::<_, _, CHUNK_SIZE>(
                    &dns_client,
                    uri,
                    &mut chunk,
                    maximum_capacity
                ).await
            }
        }.map_err(|e| {
            ariel_os_log::error!("Error when initiating the transport: {:?}", e);
            dress_up::error::Error::ConditionMatchFail { command: SuitCommand::Fetch, position: 0 }
        })?;

        let mut offset = 0;
        while let Some(maybe_chunk) = stream.next().await {
            let chunk =
                maybe_chunk.map_err(|_| dress_up::error::Error::ConditionMatchFail {
                    command: SuitCommand::Fetch,
                    position: 0,
                })?;
            self.updater
                .borrow_mut()
                .write_firmware(offset, chunk.bytes())
                .await
                .map_err(|_| dress_up::error::Error::ConditionMatchFail {
                    command: SuitCommand::Fetch,
                    position: 0,
                })?;
            offset += u32::try_from(chunk.bytes().len()).unwrap();
        }
        self.component_size.replace(offset as usize);
        Ok(())
    }
}

// fn _build_id(
//     component: Component<'_>,
//     slot: Option<u64>,
// ) -> Result<heapless::Vec<u8, MAX_ID_LENGTH>, Error> {
//     let slot_num = match slot {
//         Some(slot) => Some(
//             u8::try_from(slot)
//                 .map_err(|_| Error::InvalidManifestStructure)?
//                 .to_be_bytes(),
//         ),
//         None => None,
//     };
//     let mut id = heapless::Vec::new();
//     // skip the backend type
//     for segment in component.iter_segments()?.skip(1) {
//         id.extend_from_slice(segment?)
//             .map_err(|_| Error::InvalidManifestStructure)?; // todo: needs a separate error
//     }
//     if let Some(slot_num) = slot_num {
//         id.extend_from_slice(&slot_num)
//             .map_err(|_| Error::InvalidManifestStructure)?; // todo: needs a separate error
//     }
//     Ok(id)
// }
