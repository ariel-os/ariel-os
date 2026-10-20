use core::ops::Range;

use ariel_os_log::{debug, error, trace};
use embedded_nal_async::{Dns, TcpConnect};
use reqwless::{
    client::HttpClient,
    request::{Method, RequestBuilder as _},
};

use crate::Error;

pub struct NetworkTransportClient<'stack, 'uri, TCP: TcpConnect, DNS: Dns> {
    client: HttpClient<'stack, TCP, DNS>,
    uri: &'uri str,
}

impl<'a, 'uri, TCP: TcpConnect, DNS: Dns> NetworkTransportClient<'a, 'uri, TCP, DNS> {
    /// # Errors
    ///
    /// Return `Error::InvalidUri` if the parsing of `uri` fails.
    pub fn new(
        tcp_client: &'a TCP,
        dns_client: &'a DNS,
        uri: &'uri str,
    ) -> impl Future<Output = Result<Self, Error>> {
        if nourl::Url::parse(uri).is_err() {
            return core::future::ready(Err(Error::InvalidUri));
        }

        core::future::ready(Ok(Self {
            client: HttpClient::new(tcp_client, dns_client),
            uri,
        }))
    }

    /// # Errors
    /// Returns `Error::InvalidTransportRange` in case of a 416 response code. Other errors in the
    /// sending or receiving of the requests return `Error::Transport`.
    pub async fn get<'buf>(
        &mut self,
        range: Range<u32>,
        buf: &'buf mut [u8],
    ) -> Result<&'buf mut [u8], Error> {
        let mut range_buf = [0; Range::<u32>::BUFFER_SIZE];
        let range = range.to_header_http_range(&mut range_buf);

        let headers = &[("Range", range)];
        #[allow(unused_variables, reason = "conditional compilation")]
        let mut handle = self
            .client
            .request(Method::GET, self.uri)
            .await
            .map_err(|e| {
                error!(
                    "Error in building the request {:?}",
                    ariel_os_log::Debug2Format(&e)
                );
                Error::Transport
            })?
            .headers(headers);

        #[allow(unused_variables, reason = "conditional compilation")]
        let response = handle.send(buf).await.map_err(|e| {
            error!(
                "Error in receiving the response {:?}",
                ariel_os_log::Debug2Format(&e)
            );
            Error::Transport
        })?;

        debug!("HTTP response status: {}", response.status.0);

        // Range Not Satisfiable.
        if response.status.0 == 416 {
            return Err(Error::InvalidTransportRange);
        }

        if !response.status.is_successful() {
            return Err(Error::Transport);
        }

        // NOTE: Content-Type is not checked.
        trace!(
            "HTTP response Content-Type: {:?}",
            response.content_type.as_ref().map(|c| c.as_str())
        );

        response
            .body()
            .read_to_end()
            .await
            .map_err(|_| Error::Transport)
    }
}

const HTTP_RANGE_BYTES_VALUE_PREFIX: &str = "bytes=";

trait ToHeaderHttpRange {
    const BUFFER_SIZE: usize;

    /// Converts the range to a string for use as a `bytes` value for the
    /// [HTTP header Range](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Range).
    fn to_header_http_range<'buf>(&self, buf: &'buf mut [u8]) -> &'buf str;
}

impl ToHeaderHttpRange for Range<u32> {
    // 2 range bounds + dash.
    const BUFFER_SIZE: usize = HTTP_RANGE_BYTES_VALUE_PREFIX.len() + 2 * 10 + 1;

    #[allow(clippy::indexing_slicing)]
    fn to_header_http_range<'buf>(&self, buf: &'buf mut [u8]) -> &'buf str {
        assert!(buf.len() >= Self::BUFFER_SIZE);
        buf[..HTTP_RANGE_BYTES_VALUE_PREFIX.len()]
            .copy_from_slice(HTTP_RANGE_BYTES_VALUE_PREFIX.as_bytes());
        let mut len = HTTP_RANGE_BYTES_VALUE_PREFIX.len();

        let mut num_buf = core::fmt::NumBuffer::new();
        let range_start_str = self.start.format_into(&mut num_buf);
        buf[len..len + range_start_str.len()].copy_from_slice(range_start_str.as_bytes());
        len += range_start_str.len();

        buf[len] = 0x2d; // Dash.
        len += 1;

        // TODO: handle the case where the range is empty.
        // Offset from Range's `bytes` are *inclusive*.
        let range_end_str = (self.end - 1).format_into(&mut num_buf);
        buf[len..len + range_end_str.len()].copy_from_slice(range_end_str.as_bytes());
        len += range_end_str.len();

        // NOTE(no-panic): by construction.
        str::from_utf8(&buf[..len]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_to_http_header_range() {
        let mut buf = [0; Range::<u32>::BUFFER_SIZE];
        assert_eq!((0..42).to_header_http_range(&mut buf), "bytes=0-41");
        assert_eq!(
            (42..65_536).to_header_http_range(&mut buf),
            "bytes=42-65535"
        );
    }
}
