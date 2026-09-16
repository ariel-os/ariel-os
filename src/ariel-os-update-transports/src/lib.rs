#![cfg_attr(not(test), no_std)]
// #![deny(missing_docs)]

use core::ops::Range;

mod transport;

#[cfg(not(feature = "http"))]
compile_error!("only HTTP is currently supported");

use embassy_net::{dns::DnsSocket, tcp::client::TcpClient};

/// Returns a stream that fetches a payload from a server.
// TODO: check that N >= CHUNK_SIZE.
// `N` needs to be at least `max(HTTP response headers size, chunk size)`.
pub async fn fetch_from_uri<'a, 'uri, 'buf, const N: usize, const CHUNK_SIZE: u32>(
    tcp_client: &'a TcpClient<'a, { transport::MAX_CONCURRENT_TCP_CONNECTIONS }>,
    dns_client: &'a DnsSocket<'a>,
    uri: &'uri str,
    buf: &'buf mut [u8; N],
) -> Result<FetchStream<'a, 'uri, 'buf, N, CHUNK_SIZE>, Error> {
    let client = transport::NetworkTransportClient::new(tcp_client, dns_client, uri).await;

    Ok(FetchStream {
        chunk_index: 0,
        buf,
        client,
    })
}

pub struct FetchStream<'a, 'uri, 'buf, const N: usize, const CHUNK_SIZE: u32> {
    chunk_index: u32,
    buf: &'buf mut [u8; N],
    client: transport::NetworkTransportClient<'a, 'uri>,
}

impl<'buf, 'tcp, const N: usize, const CHUNK_SIZE: u32> FetchStream<'_, '_, 'buf, N, CHUNK_SIZE> {
    /// Fetches and returns the next chunk of the requested payload.
    async fn next(&mut self) -> Option<Result<Chunk<'_>, Error>> {
        let range = Range {
            start: self.chunk_index * CHUNK_SIZE,
            end: (self.chunk_index + 1) * CHUNK_SIZE,
        };

        let buf = match self.client.get(range, self.buf).await {
            Ok(buf) => buf,
            Err(err) => {
                return Some(Err(err));
            }
        };

        let index = self.chunk_index;
        self.chunk_index += 1;

        Some(Ok(Chunk { index, bytes: buf }))
    }
}

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
pub enum Error {
    /// Fetch operation failed because of a transport error.
    Transport,
    /// Fetch operation failed because of a requested range was invalid.
    InvalidTransportRange,
}
