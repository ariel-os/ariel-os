#![cfg_attr(not(test), no_std)]
// #![deny(missing_docs)]

mod transport;

pub async fn fetch_from_uri<'uri, 'buf, const N: usize>(
    uri: &'uri str,
    buf: &'buf [u8; N],
) -> Result<FetchIterator<'uri, 'buf, N>, Error> {
    Ok(FetchIterator {
        uri,
        chunk_index: 0,
        buf,
    })
}

#[derive(Debug)]
pub struct FetchIterator<'uri, 'buf, const N: usize> {
    uri: &'uri str,
    chunk_index: u32,
    buf: &'buf [u8; N],
}

impl<'buf, const N: usize> Iterator for FetchIterator<'_, 'buf, N> {
    type Item = Chunk<'buf>;

    fn next(&mut self) -> Option<Self::Item> {
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
