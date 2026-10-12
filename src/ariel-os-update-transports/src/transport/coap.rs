
use core::{net::SocketAddr, str::FromStr};

use ariel_os_log::error;
use coap_request::Stack;
use coap_request_implementations::AsUriPath as _;
use embedded_nal_async::Dns;
use embedded_nal_coap::RequestingCoAPClient;

use crate::Error;

const CONCURRENT_REQUESTS: usize = 3;

pub(crate) const fn chunk_size_to_block2szx(chunk_size: usize) -> Result<u8, ()> {
    match chunk_size {
        16 => Ok(0),
        32 => Ok(1),
        64 => Ok(2),
        128 => Ok(3),
        256 => Ok(4),
        512 => Ok(5),
        1024 => Ok(6),
        _ => Err(()),
    }
}


pub struct NetworkTransportClient<'uri, const CHUNK_SIZE: usize> {
    pub(crate) client: RequestingCoAPClient<'static, CONCURRENT_REQUESTS>,
    pub(crate) path: &'uri str,
}

#[must_use]
pub(crate) async fn new_client<'uri, DNS: Dns, const CHUNK_SIZE: usize>(
    dns_client: &DNS,
    uri: &'uri str
) -> Result<NetworkTransportClient<'uri, CHUNK_SIZE>, Error> {
        // let Ok(uri) = nourl::Url::parse(uri) else {
        //     return Err(Error::InvalidUri);
        // };
        const {
            assert!(
                chunk_size_to_block2szx(CHUNK_SIZE).is_ok(),
                "CoAP only supports chunk sizes of the form 2 **(x + 4) with 0<= x <= 6"
            );
        }

        let path = uri
            .strip_prefix("coap://")
            .ok_or(Error::InvalidUri)?;

        let slash_idx = path
            .find('/')
            .ok_or(Error::InvalidUri)?;
        let (addr_str, path) = path.split_at(slash_idx);

        let peer_socket_addr = if let Ok(socket_addr) = SocketAddr::from_str(addr_str) {
            socket_addr
        } else {
            let (host, port) = if let Some(colon_index) = addr_str.find(':') {
                let (host_str, port_plus_colon) = addr_str.split_at(colon_index);

                (host_str, port_plus_colon.split_at(1).1.parse::<u16>().map_err(|_e| Error::InvalidUri)?)
            } else {
                (addr_str, 5683)
            };
            let host = dns_client
                .get_host_by_name(host, embedded_nal_async::AddrType::Either).await.map_err(
                    |_e| Error::InvalidUri
                )?;
            SocketAddr::new(host, port)
        };

        // let peer_socket_addr = if let Some(socket_addr) = uri.host_socket_address() {
        //     socket_addr
        // } else {
        //     // TODO: might want to restrict to one type of IP addresses.
        //     if let Ok(ip_addr) = dns_client
        //         .get_host_by_name(uri.host(), embedded_nal_async::AddrType::Either)
        //         .await
        //     {
        //         // FIXME: requires patching nourl to support `coap://`.
        //         SocketAddr::new(ip_addr, uri.port_or_default())
        //     } else {
        //         // TODO: could introduce an error for DNS.
        //         return Err(Error::Transport);
        //     }
        // };

        let client = ariel_os_coap::coap_client().await;
        let client = client.to(peer_socket_addr);

        ariel_os_log::trace!("Addr: {}, path: {}", &peer_socket_addr, path);

        Ok(NetworkTransportClient { client, path})
    }


impl<'uri, const CHUNK_SIZE: usize> NetworkTransportClient<'uri, CHUNK_SIZE> {
    pub async fn get<'buf>(
        &mut self,
        chunk_index: u32,
        buf: &'buf mut [u8],
    ) -> Result<(&'buf mut [u8], bool), Error> {
        let request = SuitPayloadRequest::<CHUNK_SIZE>::new(self.path).set_blocknum(chunk_index);
        let Some((has_more, coap_chunk)) = self
            .client
            .request(request)
            .await
            .map_err(|_| Error::Transport)?
        else {
            return Err(Error::Transport);
        };

        let len = coap_chunk.len;
        buf[..len].copy_from_slice(&coap_chunk.bytes[..len]);

        Ok((&mut buf[..len], has_more))
    }
}

// Copied from
// <https://github.com/bergzand/Ariel-os/blob/0a44aaa8178083588f82b3f4187051ab4ec75365/examples/rustweek-demo/src/suit.rs#L41-L102>.
#[derive(Debug, Clone)]
struct SuitPayloadRequest<'a, const CHUNK_SIZE: usize> {
    block_num: u32,
    path: &'a str,
}

impl<'a, const CHUNK_SIZE: usize> SuitPayloadRequest<'a, CHUNK_SIZE> {
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

impl<'a, const CHUNK_SIZE: usize> coap_request::Request<RequestingCoAPClient<'static, CONCURRENT_REQUESTS>>
    for SuitPayloadRequest<'a, CHUNK_SIZE>
{
    type Output = Option<(bool, CoapChunk<CHUNK_SIZE>)>;

    type Carry = ();

    async fn build_request(
        &mut self,
        request: &mut <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestMessage<
            '_,
        >,
    ) -> Result<
        Self::Carry,
        <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestUnionError,
    > {
        use coap_message::MinimalWritableMessage;
        use coap_message::OptionNumber as _;
        // use coap_ext::OptionMessageWriter as _;

        let block2szx = chunk_size_to_block2szx(CHUNK_SIZE).unwrap();

        type Request<'a> = <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestMessage<'a>;
        type OptionNum<'a> = <Request<'a> as MinimalWritableMessage>::OptionNumber;
        request.set_code(coap_numbers::code::GET);
        for part in self.path.as_uri_path() {
            request.add_option(
                OptionNum::new(coap_numbers::option::URI_PATH)?,
                part.as_bytes(),
            )?;
        }
        request.add_option_uint(
            OptionNum::new(coap_numbers::option::BLOCK2)?,
                // NUM | M = 0 | szx (3 bits)
                self.block_num << 4 | u32::from(block2szx)
            )?;
        Ok(())
    }

    async fn process_response(
        &mut self,
        response: &<RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::ResponseMessage<
            '_,
        >,
        _carry: Self::Carry,
    ) -> Self::Output {
        use coap_message::{MessageOption as _, ReadableMessage as _};

        let Some(block2) = response.options().find(|o| {
            o.number() == coap_numbers::option::BLOCK2 && o.value_uint::<u32>().is_some()
        }) else {
            error!("No block number in response");
            return None;
        };

        let (blocknum, more, size) = parse_block2(block2.value_uint::<u32>().unwrap());

        if usize::from(size) != CHUNK_SIZE && blocknum != self.block_num {
            error!(
                "Unexpected block {} of size {}",
                blocknum,
                size
            );
            return None;
        }

        let payload = response.payload();
        let mut bytes = [0u8; CHUNK_SIZE];
        bytes[..payload.len()].copy_from_slice(payload);
        let out = CoapChunk {
            bytes,
            len: payload.len(),
        };
        Some((more, out))
    }
}

#[derive(Debug)]
struct CoapChunk<const N: usize> {
    bytes: [u8; N],
    len: usize,
}


fn parse_block2(block2: u32) -> (u32, bool, u16) {
    let szx = block2 & 0x7;

    let num = block2 >> 4;
    let more = (block2 & 0x8) != 0;
    let size = 1 << (4 + szx);
    return (num, more, size)
}
