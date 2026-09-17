mod coap_ext;

use ariel_os_log::error;
use coap_request::Stack;
use embedded_nal_coap::{CoAPRuntimeClient, RequestingCoAPClient};

use crate::Error;
use coap_ext::Block2Opt;

const CONCURRENT_REQUESTS: usize = 1;

// TODO: make this configurable.
const COAP_BLOCK_SIZE: usize = 64;

pub struct NetworkTransportClient<'uri> {
    client: CoAPRuntimeClient<'static, CONCURRENT_REQUESTS>,
    uri: &'uri nourl::Url<'uri>,
}

impl<'a, 'uri> NetworkTransportClient<'a, 'uri> {
    #[must_use]
    pub async fn new(uri: &'uri str) -> Result<Self, Error> {
        let Ok(uri) = nourl::Url::parse(uri) else {
            return Err(Error::InvalidUri);
        };

        let peer_socket_addr = if let Some(socket_addr) = uri.host_socket_address() {
            socket_addr
        } else {
            todo!("DNS request");
        };

        let client = ariel_os_coap::coap_client().await;
        let client = client.to(peer_socket_addr);

        Self { client, uri }
    }

    pub async fn get<'buf>(
        &mut self,
        chunk_index: u32,
        buf: &'buf mut [u8],
    ) -> Result<&'buf mut [u8], Error> {
        let request = SuitPayloadRequest::new(self.uri.path()).set_blocknum(chunk_index);
        let (_has_more, data) = self
            .client
            .request(request)
            .await
            .map_err(|_| Error::Transport)?;

        Ok(data)
    }
}

// Copied from
// <https://github.com/bergzand/Ariel-os/blob/0a44aaa8178083588f82b3f4187051ab4ec75365/examples/rustweek-demo/src/suit.rs#L41-L102>.
#[derive(Debug, Clone)]
struct SuitPayloadRequest<'a> {
    block_num: u32,
    path: &'a str,
}

impl<'a> SuitPayloadRequest<'a> {
    fn new(path: &'a str) -> Self {
        Self { block_num: 0, path }
    }

    fn set_blocknum(&self, block_num: u32) -> Self {
        Self {
            block_num,
            path: self.path,
        }
    }
}

// FIXME: what's 3?
impl<'a> coap_request::Request<RequestingCoAPClient<'static, 3>> for SuitPayloadRequest<'a> {
    type Output = Option<(bool, CoapChunk<COAP_BLOCK_SIZE>)>;

    type Carry = (u32, u8);

    async fn build_request(
        &mut self,
        request: &mut <RequestingCoAPClient<'static, 3> as Stack>::RequestMessage<'_>,
    ) -> Result<Self::Carry, <RequestingCoAPClient<'static, 3> as Stack>::RequestUnionError> {
        use coap_message::MinimalWritableMessage as _;

        use coap_ext::OptionMessageWriter as _;

        let szx = 2;
        request.set_code(coap_numbers::code::GET);
        request.add_option_uri_path(self.path)?;
        request.add_option_block2(szx, self.block_num)?;
        Ok((self.block_num, szx))
    }

    async fn process_response(
        &mut self,
        response: &<RequestingCoAPClient<'static, 3> as Stack>::ResponseMessage<'_>,
        carry: Self::Carry,
    ) -> Self::Output {
        use coap_message::{MessageOption as _, ReadableMessage as _};

        use coap_ext::Block2RequestDataExt as _;

        let (blocknum, _szx) = carry;
        let Some(block2) = response.options().find(|o| {
            o.number() == coap_numbers::option::BLOCK2 && o.value_uint::<u32>().is_some()
        }) else {
            error!("No block number in response");
            return None;
        };
        let block2: Block2Opt = block2.value_uint::<u32>().unwrap().into();

        if usize::from(block2.size()) != COAP_BLOCK_SIZE && block2.blocknum() != blocknum {
            error!(
                "Unexpected block {} or size {}",
                block2.blocknum(),
                block2.size()
            );
            return None;
        }

        let payload = response.payload();
        let mut bytes = [0u8; COAP_BLOCK_SIZE];
        bytes[..payload.len()].copy_from_slice(payload);
        let out = CoapChunk {
            bytes,
            len: payload.len(),
        };
        Some((block2.has_more(), out))
    }
}

#[derive(Debug)]
struct CoapChunk<const N: usize> {
    bytes: [u8; N],
    len: usize,
}
