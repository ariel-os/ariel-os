use core::ops::Range;

use ariel_os_log::debug;
use embassy_net::{dns::DnsSocket, tcp::client::TcpClient};
use reqwless::{
    client::HttpClient,
    request::{Method, RequestBuilder as _},
};

use crate::Error;

pub const MAX_CONCURRENT_TCP_CONNECTIONS: usize = 1;

// TODO: adjust these.
const TCP_BUFFER_SIZE: usize = 1024;
const HTTP_BUFFER_SIZE: usize = 1024;

pub struct NetworkTransportClient<'stack, 'uri> {
    client:
        HttpClient<'stack, TcpClient<'stack, MAX_CONCURRENT_TCP_CONNECTIONS>, DnsSocket<'stack>>,
    uri: &'uri str,
}

impl<'a, 'uri> NetworkTransportClient<'a, 'uri> {
    #[must_use]
    pub async fn new(
        tcp_client: &'a TcpClient<'a, MAX_CONCURRENT_TCP_CONNECTIONS>,
        dns_client: &'a DnsSocket<'a>,
        uri: &'uri str,
    ) -> Self {
        Self {
            client: HttpClient::new(tcp_client, dns_client),
            uri,
        }
    }

    pub async fn get<'buf>(
        &mut self,
        range: Range<u32>,
        buf: &'buf mut [u8],
    ) -> Result<&'buf mut [u8], Error> {
        let mut range_buf = [0; Range::<u32>::BUFFER_SIZE];
        let range = range.to_header_http_range(&mut range_buf);

        let headers = &[("Range", range)];
        let mut handle = self
            .client
            .request(Method::GET, self.uri)
            .await
            .map_err(|_| Error::Transport)?
            .headers(headers);
        let response = handle.send(buf).await.map_err(|_| Error::Transport)?;

        debug!("HTTP response status: {}", response.status.0);

        // Range Not Satisfiable.
        if response.status.0 == 416 {
            return Err(Error::InvalidTransportRange);
        }

        if !response.status.is_successful() {
            return Err(Error::Transport);
        }

        // NOTE: Content-Type is not checked.
        debug!("HTTP response Content-Type: {}", response.content_type);

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

    fn to_header_http_range<'buf>(&self, buf: &'buf mut [u8]) -> &'buf str {
        buf[..HTTP_RANGE_BYTES_VALUE_PREFIX.len()]
            .copy_from_slice(HTTP_RANGE_BYTES_VALUE_PREFIX.as_bytes());
        let mut len = HTTP_RANGE_BYTES_VALUE_PREFIX.len();

        // TODO: only available since Rust 1.98.
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

        // NOTE(no-panic): by construction.
        str::from_utf8(buf).unwrap()
    }
}
