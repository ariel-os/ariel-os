#![cfg_attr(not(test), no_std)]
// #![deny(missing_docs)]

#[featurecomb::comb]
mod _featurecomb {}

mod transport;

#[cfg(feature = "coap")]
mod coap {
    use embedded_nal_async::Dns;

    use crate::{Chunk, Error, transport};

    /// Returns a stream that fetches a payload from a server.
    pub async fn fetch_from_uri<'a, 'uri, 'buf, DNS: Dns, const N: usize, const CHUNK_SIZE: u32>(
        dns_client: &'a DNS,
        uri: &'uri str,
        buf: &'buf mut [u8; N],
        payload_size: u32,
    ) -> Result<FetchStream<'uri, 'buf, N, CHUNK_SIZE>, Error> {
        const {
            assert!(N >= CHUNK_SIZE as usize);
        }

        let client = transport::NetworkTransportClient::new(dns_client, uri).await?;

        Ok(FetchStream {
            chunk_index: 0,
            buf,
            bytes_received: 0,
            payload_size,
            client,
        })
    }

    pub struct FetchStream<'uri, 'buf, const N: usize, const CHUNK_SIZE: u32> {
        chunk_index: u32,
        buf: &'buf mut [u8; N],
        bytes_received: u32,
        payload_size: u32,
        client: transport::NetworkTransportClient<'uri>,
    }

    impl<'buf, const N: usize, const CHUNK_SIZE: u32> FetchStream<'_, 'buf, N, CHUNK_SIZE> {
        /// Fetches and returns the next chunk of the requested payload.
        pub async fn next(&mut self) -> Option<Result<Chunk<'_>, Error>> {
            if self.bytes_received >= self.payload_size {
                return None;
            }

            let buf = match self.client.get(self.chunk_index, self.buf).await {
                Ok(buf) => buf,
                Err(err) => {
                    return Some(Err(err));
                }
            };

            self.bytes_received += u32::try_from(buf.len()).unwrap();

            let index = self.chunk_index;
            self.chunk_index += 1;

            Some(Ok(Chunk { index, bytes: buf }))
        }
    }
}

#[cfg(feature = "http")]
mod http {
    use core::ops::Range;

    use embedded_nal_async::{Dns, TcpConnect};

    use crate::{Chunk, Error, transport};

    /// Returns a stream that fetches a payload from a server.
    // `N` needs to be at least `max(HTTP response headers size, chunk size)`.
    // N >= CHUNK_SIZE.
    pub async fn fetch_from_uri<
        'a,
        'uri,
        'buf,
        TCP: TcpConnect,
        DNS: Dns,
        const N: usize,
        const CHUNK_SIZE: u32,
    >(
        tcp_client: &'a TCP,
        dns_client: &'a DNS,
        uri: &'uri str,
        buf: &'buf mut [u8; N],
        payload_size: u32,
    ) -> Result<FetchStream<'a, 'uri, 'buf, TCP, DNS, N, CHUNK_SIZE>, Error> {
        const {
            assert!(N >= CHUNK_SIZE as usize);
        }

        let client = transport::NetworkTransportClient::new(tcp_client, dns_client, uri).await?;

        Ok(FetchStream {
            chunk_index: 0,
            buf,
            bytes_received: 0,
            payload_size,
            client,
        })
    }

    pub struct FetchStream<
        'a,
        'uri,
        'buf,
        TCP: TcpConnect,
        DNS: Dns,
        const N: usize,
        const CHUNK_SIZE: u32,
    > {
        chunk_index: u32,
        buf: &'buf mut [u8; N],
        bytes_received: u32,
        payload_size: u32,
        client: transport::NetworkTransportClient<'a, 'uri, TCP, DNS>,
    }

    impl<'buf, 'tcp, TCP: TcpConnect, DNS: Dns, const N: usize, const CHUNK_SIZE: u32>
        FetchStream<'_, '_, 'buf, TCP, DNS, N, CHUNK_SIZE>
    {
        /// Fetches and returns the next chunk of the requested payload.
        pub async fn next(&mut self) -> Option<Result<Chunk<'_>, Error>> {
            if self.bytes_received >= self.payload_size {
                return None;
            }

            let range = Range {
                start: self.bytes_received,
                end: self.bytes_received + CHUNK_SIZE,
            };

            let buf = match self.client.get(range, self.buf).await {
                Ok(buf) => buf,
                Err(err) => {
                    return Some(Err(err));
                }
            };

            self.bytes_received += u32::try_from(buf.len()).unwrap();

            let index = self.chunk_index;
            self.chunk_index += 1;

            Some(Ok(Chunk { index, bytes: buf }))
        }
    }
}

#[cfg(feature = "coap")]
pub use coap::*;

#[cfg(feature = "http")]
pub use http::*;

#[derive(Debug)]
pub struct Chunk<'bytes> {
    index: u32,
    bytes: &'bytes [u8],
}

impl<'bytes> Chunk<'bytes> {
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn bytes(&self) -> &'bytes [u8] {
        self.bytes
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The URI is invalid.
    InvalidUri,
    /// Fetch operation failed because of a transport error.
    Transport,
    /// Fetch operation failed because of a requested range was invalid.
    InvalidTransportRange,
}
