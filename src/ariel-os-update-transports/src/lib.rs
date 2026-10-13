#![cfg_attr(nightly, feature(doc_cfg))]
#![cfg_attr(not(test), no_std)]

#[featurecomb::comb]
mod _featurecomb {}

mod transport;

#[cfg(feature = "coap")]
mod coap {
    use embedded_nal_async::Dns;

    use crate::{Chunk, Error, transport};

    /// Returns a stream that fetches a payload from a server.
<<<<<<< HEAD
<<<<<<< HEAD
    ///
    /// # Errors
    ///
    /// See `transport::new_client`.
    pub async fn fetch_from_uri<'uri, 'buf, DNS: Dns, const N: usize, const CHUNK_SIZE: usize>(
        dns_client: &'_ DNS,
=======
    pub async fn fetch_from_uri<'a, 'uri, 'buf, DNS: Dns, const N: usize, const CHUNK_SIZE: usize>(
=======
    pub async fn fetch_from_uri<
        'a,
        'uri,
        'buf,
        DNS: Dns,
        const N: usize,
        const CHUNK_SIZE: usize,
    >(
>>>>>>> 0dc28f0f (feat: remove requirement on payload size in http)
        dns_client: &'a DNS,
>>>>>>> b5d21f19 (wip)
        uri: &'uri str,
        buf: &'buf mut [u8; N],
        payload_capacity: u32,
    ) -> Result<FetchStream<'uri, 'buf, N, CHUNK_SIZE>, Error> {
        const {
            assert!(N >= CHUNK_SIZE);
        }

<<<<<<< HEAD
<<<<<<< HEAD
        let client = transport::new_client::<_, CHUNK_SIZE>(dns_client, uri).await?;
=======
        let client = transport::new_client::<_, CHUNK_SIZE >(dns_client, uri).await?;
>>>>>>> b5d21f19 (wip)
=======
        let client = transport::new_client::<_, CHUNK_SIZE>(dns_client, uri).await?;
>>>>>>> 0dc28f0f (feat: remove requirement on payload size in http)

        Ok(FetchStream {
            buf,
            bytes_received: 0,
            client,
            has_more: true,
            payload_capacity,
        })
    }

    pub struct FetchStream<'uri, 'buf, const N: usize, const CHUNK_SIZE: usize> {
<<<<<<< HEAD
=======
        chunk_index: u32,
>>>>>>> b5d21f19 (wip)
        buf: &'buf mut [u8; N],
        bytes_received: u32,
        has_more: bool,
        client: transport::NetworkTransportClient<'uri, CHUNK_SIZE>,
<<<<<<< HEAD
        payload_capacity: u32,
    }

    impl<const N: usize, const CHUNK_SIZE: usize> FetchStream<'_, '_, N, CHUNK_SIZE> {
=======
    }

    impl<'buf, const N: usize, const CHUNK_SIZE: usize> FetchStream<'_, 'buf, N, CHUNK_SIZE> {
>>>>>>> b5d21f19 (wip)
        /// Fetches and returns the next chunk of the requested payload.
        ///
        /// # Panics
        ///
        /// if the chunk length is bigger than `u32::MAX`.
        pub async fn next(&mut self) -> Option<Result<Chunk<'_>, Error>> {
            if !self.has_more {
                return None;
            }

<<<<<<< HEAD
            let buf = match self.client.get(self.buf).await {
=======
            let buf = match self.client.get(self.chunk_index, self.buf).await {
>>>>>>> 9606c889 (fix: minor fix)
                Ok((buf, has_more)) => {
                    self.has_more = has_more;
                    buf
                }
                Err(err) => {
                    // Prevent further request after encountering an error.
                    self.has_more = false;
                    return Some(Err(err));
                }
            };
            self.bytes_received += u32::try_from(buf.len()).unwrap();

            if self.bytes_received > self.payload_capacity {
                self.has_more = true;
                return Some(Err(Error::ExceedsCapacity));
            }

            Some(Ok(Chunk { bytes: buf }))
        }
    }
}

#[cfg(feature = "http")]
mod http {
    use core::ops::Range;

    use embedded_nal_async::{Dns, TcpConnect};

    use crate::{Chunk, Error, transport};

    /// Returns a stream that fetches a payload from a server.
    ///
    /// # Errors
    ///
    /// See `transport::NetworkTransportClient::new`.
    // `N` needs to be at least `max(HTTP response headers size, chunk size)`.
    // N >= CHUNK_SIZE.
    pub async fn fetch_from_uri<
        'a,
        'uri,
        'buf,
        TCP: TcpConnect,
        DNS: Dns,
        const N: usize,
        const CHUNK_SIZE: usize,
    >(
        tcp_client: &'a TCP,
        dns_client: &'a DNS,
        uri: &'uri str,
        buf: &'buf mut [u8; N],
<<<<<<< HEAD
        payload_capacity: u32,
=======
>>>>>>> 0dc28f0f (feat: remove requirement on payload size in http)
    ) -> Result<FetchStream<'a, 'uri, 'buf, TCP, DNS, N, CHUNK_SIZE>, Error> {
        const {
            assert!(N >= CHUNK_SIZE);
        }

        let client = transport::NetworkTransportClient::new(tcp_client, dns_client, uri).await?;

        Ok(FetchStream {
            buf,
            bytes_received: 0,
            received_any: false,
            client,
            payload_capacity,
            is_done: false,
        })
    }

    pub struct FetchStream<
        'a,
        'uri,
        'buf,
        TCP: TcpConnect,
        DNS: Dns,
        const N: usize,
        const CHUNK_SIZE: usize,
    > {
        buf: &'buf mut [u8; N],
        bytes_received: u32,
        received_any: bool,
        client: transport::NetworkTransportClient<'a, 'uri, TCP, DNS>,
        payload_capacity: u32,
        is_done: bool,
    }

<<<<<<< HEAD
    impl<TCP: TcpConnect, DNS: Dns, const N: usize, const CHUNK_SIZE: usize>
        FetchStream<'_, '_, '_, TCP, DNS, N, CHUNK_SIZE>
=======
    impl<'buf, 'tcp, TCP: TcpConnect, DNS: Dns, const N: usize, const CHUNK_SIZE: usize>
        FetchStream<'_, '_, 'buf, TCP, DNS, N, CHUNK_SIZE>
>>>>>>> b5d21f19 (wip)
    {
        /// Fetches and returns the next chunk of the requested payload.
        ///
        /// # Panics
        ///
        /// If the received buffer length or `CHUNK_SIZE` are bigger than `u32::MAX`.
        pub async fn next(&mut self) -> Option<Result<Chunk<'_>, Error>> {
<<<<<<< HEAD
            // Early exit to prevent extra request after the fetching is done.
            if self.is_done {
                return None;
            }
=======
>>>>>>> 0dc28f0f (feat: remove requirement on payload size in http)
            let range = Range {
                start: self.bytes_received,
<<<<<<< HEAD
                end: self.bytes_received + u32::try_from(CHUNK_SIZE).unwrap(),
            };

            ariel_os_log::trace!("GET-ing range: {:?} of the payload file", range);
            // NOTE: Since we don't know the size of the payload that we are
            // fetching, werely on the fact that a GET request with a range whose
            // start is outside of the file leads to 416 Error which is caught below as
            // Error::InvalidTransportRange. If this occurs after receiving at least one
            // valid chunk, we consider the fetch to be done.
=======
                end: self.bytes_received + CHUNK_SIZE as u32,
            };

            ariel_os_log::trace!("GET-ing range: {:?} of the payload file", range);
>>>>>>> b5d21f19 (wip)
            let buf = match self.client.get(range, self.buf).await {
                Ok(buf) => buf,
                Err(Error::InvalidTransportRange) => {
<<<<<<< HEAD
                    self.is_done = true;
                    if self.received_any {
                        return None;
                    }

=======
                    if self.received_any {
                        return None;
                    }
>>>>>>> 0dc28f0f (feat: remove requirement on payload size in http)
                    return Some(Err(Error::InvalidTransportRange));
                }
                Err(err) => {
                    self.is_done = true;
                    return Some(Err(err));
                }
            };
            self.received_any = true;
            self.bytes_received += u32::try_from(buf.len()).unwrap();

            if self.bytes_received > self.payload_capacity {
                self.is_done = true;
                return Some(Err(Error::ExceedsCapacity));
            }

            Some(Ok(Chunk { bytes: buf }))
        }
    }
}

#[cfg(feature = "coap")]
pub use coap::*;

#[cfg(feature = "http")]
pub use http::*;

#[derive(Debug)]
pub struct Chunk<'bytes> {
    bytes: &'bytes [u8],
}

impl<'bytes> Chunk<'bytes> {
    #[must_use]
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
    /// Fetch operation failed because the received size exceeds the allowed capacity.
    ExceedsCapacity,
}
