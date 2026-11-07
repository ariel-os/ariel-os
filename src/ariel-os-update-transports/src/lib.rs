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
    ///
    /// # Errors
    ///
    /// See `transport::new_client`.
    pub async fn fetch_from_uri<'uri, 'buf, DNS: Dns, const N: usize, const CHUNK_SIZE: usize>(
        dns_client: &'_ DNS,
        uri: &'uri str,
        buf: &'buf mut [u8; N],
        // We currently don't evaluate Size2 in advance; capacity is enforced per chunk in `next()`.
        payload_capacity: u32,
    ) -> Result<FetchStream<'uri, 'buf, N, CHUNK_SIZE>, Error> {
        let client = transport::new_client::<_, N>(dns_client, uri, buf).await?;

        Ok(FetchStream {
            bytes_received: 0,
            payload_capacity,
            has_more: true,
            client,
            _phantom: core::marker::PhantomData,
        })
    }

    pub struct FetchStream<'uri, 'buf, const N: usize, const CHUNK_SIZE: usize> {
        bytes_received: u32,
        payload_capacity: u32,
        // Needed because the iterator needs to call next() again after the last block to know that
        // it was really the last -- semantics are "produces nothing more" and not "flags itself to
        // be last": this bit persists the information.
        has_more: bool,
        client: transport::NetworkTransportClient<'uri, 'buf, N>,
        // Generic parameter is only needed for HTTP
        _phantom: core::marker::PhantomData<[u8; CHUNK_SIZE]>,
    }

    impl<const N: usize, const CHUNK_SIZE: usize> FetchStream<'_, '_, N, CHUNK_SIZE> {
        /// Fetches and returns the next chunk of the requested payload.
        pub async fn next(&mut self) -> Option<Result<Chunk<'_>, Error>> {
            if !self.has_more {
                return None;
            }

            let buf = match self.client.get().await {
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

            if let Err(err) = crate::record_received_bytes(
                &mut self.bytes_received,
                buf.len(),
                self.payload_capacity,
            ) {
                self.has_more = false;
                return Some(Err(err));
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
        payload_capacity: u32,
    ) -> Result<FetchStream<'a, 'uri, 'buf, TCP, DNS, N, CHUNK_SIZE>, Error> {
        const {
            assert!(CHUNK_SIZE > 0);
            assert!(CHUNK_SIZE as u64 <= u32::MAX as u64);
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

    impl<TCP: TcpConnect, DNS: Dns, const N: usize, const CHUNK_SIZE: usize>
        FetchStream<'_, '_, '_, TCP, DNS, N, CHUNK_SIZE>
    {
        /// Fetches and returns the next chunk of the requested payload.
        pub async fn next(&mut self) -> Option<Result<Chunk<'_>, Error>> {
            // Early exit to prevent extra request after the fetching is done.
            if self.is_done {
                return None;
            }
            let chunk_size = const {
                assert!(CHUNK_SIZE > 0);
                assert!(CHUNK_SIZE as u64 <= u32::MAX as u64);
                #[allow(
                    clippy::cast_possible_truncation,
                    reason = "checked in const block above"
                )]
                {
                    CHUNK_SIZE as u32
                }
            };
            let range = Range {
                start: self.bytes_received,
                end: self.bytes_received.saturating_add(chunk_size),
            };

            ariel_os_log::trace!("GET-ing range: {:?} of the payload file", range);
            // NOTE: Since we don't know the size of the payload that we are
            // fetching, we rely on the fact that a GET request with a range whose
            // start is outside of the file leads to 416 Error which is caught below as
            // Error::InvalidTransportRange. If this occurs after receiving at least one
            // valid chunk, we consider the fetch to be done.
            let buf = match self.client.get(range, self.buf).await {
                Ok(buf) => buf,
                Err(Error::InvalidTransportRange) => {
                    self.is_done = true;
                    if self.received_any {
                        return None;
                    }

                    return Some(Err(Error::InvalidTransportRange));
                }
                Err(err) => {
                    self.is_done = true;
                    return Some(Err(err));
                }
            };
            self.received_any = true;
            if let Err(err) = crate::record_received_bytes(
                &mut self.bytes_received,
                buf.len(),
                self.payload_capacity,
            ) {
                self.is_done = true;
                return Some(Err(err));
            }

            Some(Ok(Chunk { bytes: buf }))
        }
    }
}

#[cfg(feature = "coap")]
pub use coap::*;

#[cfg(feature = "http")]
pub use http::*;

/// Accumulates `chunk_len` into `bytes_received`, checking against `payload_capacity`.
///
/// # Errors
///
/// Returns [`Error::ExceedsCapacity`] if adding `chunk_len` overflows `u32` or exceeds
/// `payload_capacity`.
fn record_received_bytes(
    bytes_received: &mut u32,
    chunk_len: usize,
    payload_capacity: u32,
) -> Result<(), Error> {
    let total = u32::try_from(chunk_len)
        .ok()
        .and_then(|len| bytes_received.checked_add(len))
        .filter(|&total| total <= payload_capacity)
        .ok_or(Error::ExceedsCapacity)?;

    *bytes_received = total;
    Ok(())
}

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

#[derive(Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_received_bytes_within_and_at_capacity() {
        let mut received = 0;
        assert_eq!(record_received_bytes(&mut received, 64, 100), Ok(()));
        assert_eq!(received, 64);
        assert_eq!(record_received_bytes(&mut received, 36, 100), Ok(()));
        assert_eq!(received, 100);
    }

    #[test]
    fn record_received_bytes_exceeds_capacity_and_overflow() {
        let mut received = 90;
        assert_eq!(
            record_received_bytes(&mut received, 11, 100),
            Err(Error::ExceedsCapacity)
        );
        assert_eq!(received, 90);

        let mut near_max = u32::MAX - 4;
        assert_eq!(
            record_received_bytes(&mut near_max, 10, u32::MAX),
            Err(Error::ExceedsCapacity)
        );
        assert_eq!(near_max, u32::MAX - 4);
    }
}
