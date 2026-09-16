#![cfg_attr(not(test), no_std)]
// #![deny(missing_docs)]

mod transport;

use embassy_net::{dns::DnsSocket, tcp::client::TcpClient};

pub async fn fetch_from_uri<'a, 'uri, 'buf, const N: usize>(
    tcp_client: &'a TcpClient<'a, { transport::MAX_CONCURRENT_TCP_CONNECTIONS }>,
    dns_client: &'a DnsSocket<'a>,
    uri: &'uri str,
    buf: &'buf mut [u8; N],
) -> Result<FetchStream<'a, 'uri, 'buf, N>, Error> {
    let client = transport::NetworkTransportClient::new(tcp_client, dns_client, uri).await;

    Ok(FetchStream {
        uri,
        chunk_index: 0,
        buf,
        client,
    })
}

pub struct FetchStream<'a, 'uri, 'buf, const N: usize> {
    uri: &'uri str, // TODO: remove this.
    chunk_index: u32,
    buf: &'buf mut [u8; N],
    client: transport::NetworkTransportClient<'a, 'uri>,
}

impl<'buf, 'tcp, const N: usize> FetchStream<'_, '_, 'buf, N> {
    async fn next(&mut self) -> Option<Chunk<'buf>> {
        // transport::get(self.uri)
        todo!()
    }
}

#[derive(Debug)]
pub struct Chunk<'bytes> {
    index: u32,
    bytes: &'bytes [u8],
}

impl Chunk<'_> {
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }
}

#[derive(Debug)]
pub enum Error {
    /// Fetch operation failed because of a transport error.
    Transport,
}
