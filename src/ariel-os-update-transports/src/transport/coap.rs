<<<<<<< HEAD
use core::net::SocketAddr;
=======

use core::{net::SocketAddr, str::FromStr};
>>>>>>> b5d21f19 (wip)

use ariel_os_log::error;
use coap_message_utils::OptionsExt as _;
use coap_message_utils::option_value::{Block2RequestData, Block2ResponseData};
use coap_request::Stack;
use coap_request_implementations::AsUriPath as _;
use embedded_nal_async::Dns;
use embedded_nal_coap::RequestingCoAPClient;

use nourl::UrlScheme;

use crate::Error;

const CONCURRENT_REQUESTS: usize = 3;

<<<<<<< HEAD
pub struct NetworkTransportClient<'uri, const CHUNK_SIZE: usize> {
    pub(crate) client: RequestingCoAPClient<'static, CONCURRENT_REQUESTS>,
    // Reusing a single request instance because in there, not only is the path stored, but also
    // the state of the block-wise transfer.
    request: SuitPayloadRequest<'uri, CHUNK_SIZE>,
}

/// Create a new CoAP client to send requests to a given uri.
///
/// # Errors
///
/// * If the URI parsing fails.
/// * If the DNS resolution fails.
pub(crate) async fn new_client<'uri, DNS: Dns, const CHUNK_SIZE: usize>(
    dns_client: &DNS,
    uri: &'uri str,
) -> Result<NetworkTransportClient<'uri, CHUNK_SIZE>, Error> {
    let Ok(uri) = nourl::Url::parse(uri) else {
        return Err(Error::InvalidUri);
    };

    match uri.scheme() {
        UrlScheme::CoAP | UrlScheme::CoAPS => {}
        _ => return Err(Error::InvalidUri),
    }

    let peer_socket_addr = if let Some(socket_addr) = uri.host_socket_address() {
        socket_addr
    } else {
        // TODO: might want to restrict to one type of IP addresses.
        if let Ok(ip_addr) = dns_client
            .get_host_by_name(uri.host(), embedded_nal_async::AddrType::Either)
            .await
        {
            SocketAddr::new(ip_addr, uri.port_or_default())
        } else {
            // TODO: could introduce an error for DNS.
            return Err(Error::Transport);
        }
    };

    let client = ariel_os_coap::coap_client().await;
    let client = client.to(peer_socket_addr);

    ariel_os_log::trace!("Addr: {}, path: {}", &peer_socket_addr, uri.path());

    Ok(NetworkTransportClient {
        client,
        request: SuitPayloadRequest::<CHUNK_SIZE>::new(uri.path()),
    })
}

impl<const CHUNK_SIZE: usize> NetworkTransportClient<'_, CHUNK_SIZE> {
    /// Fetch and return a single chunk of data from the URI.
    ///
    /// # Errors
    ///
    /// * If the request fails or returns garbage data.
    pub async fn get<'buf, const N: usize>(
=======
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
>>>>>>> b5d21f19 (wip)
        &mut self,
        buf: &'buf mut [u8; N],
    ) -> Result<(&'buf mut [u8], bool), Error> {
<<<<<<< HEAD
        const {
            assert!(N >= CHUNK_SIZE);
        }
=======
        let request = SuitPayloadRequest::<CHUNK_SIZE>::new(self.path).set_blocknum(chunk_index);
>>>>>>> b5d21f19 (wip)
        let Some((has_more, coap_chunk)) = self
            .client
            .request(&mut self.request)
            .await
            .map_err(|_| Error::Transport)?
        else {
            return Err(Error::Transport);
        };

        // NOTE: len <= CHUNK_SIZE by construction
        let len = coap_chunk.len;
        buf[..len].copy_from_slice(&coap_chunk.bytes[..len]);

        Ok((&mut buf[..len], has_more))
    }
}

// Copied and enhanced from
// <https://github.com/bergzand/Ariel-os/blob/0a44aaa8178083588f82b3f4187051ab4ec75365/examples/rustweek-demo/src/suit.rs#L41-L102>.
#[derive(Debug, Clone)]
struct SuitPayloadRequest<'a, const CHUNK_SIZE: usize> {
<<<<<<< HEAD
    // Next block2 request option to send
    request_block: Block2RequestData,
=======
    block_num: u32,
>>>>>>> b5d21f19 (wip)
    path: &'a str,
}

impl<'a, const CHUNK_SIZE: usize> SuitPayloadRequest<'a, CHUNK_SIZE> {
    fn new(path: &'a str) -> Self {
        Self {
            request_block: Block2RequestData::new_initial(CHUNK_SIZE),
            path,
        }
    }
}

<<<<<<< HEAD
impl<const CHUNK_SIZE: usize>
    coap_request::Request<RequestingCoAPClient<'static, CONCURRENT_REQUESTS>>
    for &mut SuitPayloadRequest<'_, CHUNK_SIZE>
=======
impl<'a, const CHUNK_SIZE: usize> coap_request::Request<RequestingCoAPClient<'static, CONCURRENT_REQUESTS>>
    for SuitPayloadRequest<'a, CHUNK_SIZE>
>>>>>>> b5d21f19 (wip)
{
    type Output = Option<(bool, CoapChunk<CHUNK_SIZE>)>;

    type Carry = ();

    fn build_request(
        &mut self,
        request: &mut <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestMessage<
            '_,
        >,
    ) -> impl Future<
        Output = Result<
            Self::Carry,
            <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestUnionError,
        >,
    > {
        use coap_message::MinimalWritableMessage;
        use coap_message::OptionNumber as _;
<<<<<<< HEAD

        type Request<'a> =
            <RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::RequestMessage<'a>;
        type OptionNum<'a> = <Request<'a> as MinimalWritableMessage>::OptionNumber;

        request.set_code(coap_numbers::code::GET);
        for part in self.path.as_uri_path() {
            if let Err(e) = request.add_option(
                OptionNum::new(coap_numbers::option::URI_PATH).unwrap(),
                part.as_bytes(),
            ) {
                return core::future::ready(Err(e));
            }
        }
        if let Err(e) = request.add_option_uint(
            OptionNum::new(coap_numbers::option::BLOCK2).unwrap(),
            self.request_block.to_option_value(false),
        ) {
            return core::future::ready(Err(e));
        }
        core::future::ready(Ok(()))
=======
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
>>>>>>> b5d21f19 (wip)
    }

    fn process_response(
        &mut self,
        response: &<RequestingCoAPClient<'static, CONCURRENT_REQUESTS> as Stack>::ResponseMessage<
            '_,
        >,
        _carry: Self::Carry,
<<<<<<< HEAD
    ) -> impl Future<Output = Self::Output> {
        use coap_message::ReadableMessage as _;

        let mut block2: Option<Block2ResponseData> = None;
        if response
            .options()
            .take_into(&mut block2)
            .ignore_elective_others()
            .is_err()
        {
            error!("Unprocessed critical options in response");
            return core::future::ready(None);
        }

        let Some(block2) = block2 else {
=======
    ) -> Self::Output {
        use coap_message::{MessageOption as _, ReadableMessage as _};

        let Some(block2) = response.options().find(|o| {
            o.number() == coap_numbers::option::BLOCK2 && o.value_uint::<u32>().is_some()
        }) else {
>>>>>>> b5d21f19 (wip)
            error!("No block number in response");
            return core::future::ready(None);
        };

<<<<<<< HEAD
        if !block2.matches_request(&self.request_block, response.payload()) {
            error!(
                "Unexpected block {:?} (payload length {}) after requesting {:?}",
                block2,
                response.payload().len(),
                self.request_block
=======
        let (blocknum, more, size) = parse_block2(block2.value_uint::<u32>().unwrap());

        if usize::from(size) != CHUNK_SIZE && blocknum != self.block_num {
            error!(
                "Unexpected block {} of size {}",
                blocknum,
                size
>>>>>>> b5d21f19 (wip)
            );
            return core::future::ready(None);
        }

        let payload = response.payload();
<<<<<<< HEAD

        // Violation of block-wise transfer rules by the server
        if payload.len() > CHUNK_SIZE {
            error!("Unexpected payload of size {}", payload.len());
            return core::future::ready(None);
        }
=======
>>>>>>> b5d21f19 (wip)
        let mut bytes = [0u8; CHUNK_SIZE];
        bytes[..payload.len()].copy_from_slice(payload);
        let out = CoapChunk {
            bytes,
            len: payload.len(),
        };
<<<<<<< HEAD
        let next_block = block2.next_request();
        let Ok(next_block) = next_block else {
            error!("Firmware size exceeds what can be expressed at this block size");
            return core::future::ready(None);
        };
        if let Some(next_block) = next_block {
            self.request_block = next_block;
            core::future::ready(Some((true, out)))
        } else {
            // Not updating request_block, but then again, we won't be called any more anyway.
            core::future::ready(Some((false, out)))
        }
=======
        Some((more, out))
>>>>>>> b5d21f19 (wip)
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
