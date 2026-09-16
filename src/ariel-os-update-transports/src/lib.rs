#![cfg_attr(not(test), no_std)]
// #![deny(missing_docs)]

mod transport;

pub async fn fetch_from_uri<'uri, 'buf, const N: usize>(
    uri: &'uri str,
    buf: &'buf mut [u8; N],
) -> Result<FetchStream<'uri, 'buf, N>, Error> {
    let client = transport::NetworkTransportClient::new(uri).await;

    Ok(FetchStream {
        uri,
        chunk_index: 0,
        buf,
        client,
    })
}

#[derive(Debug)]
pub struct FetchStream<'uri, 'buf, const N: usize> {
    uri: &'uri str, // TODO: remove this.
    chunk_index: u32,
    buf: &'buf mut [u8; N],
    client: transport::NetworkTransportClient<'static>,
}

impl<'buf, 'tcp, const N: usize> FetchStream<'_, 'buf, N> {
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
