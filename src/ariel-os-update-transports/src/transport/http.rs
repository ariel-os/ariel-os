use embassy_net::{
    dns::DnsSocket,
    tcp::client::TcpClient,
};
use reqwless::{
    client::HttpClient,
    request::Method,
};

use crate::Error;

pub const MAX_CONCURRENT_TCP_CONNECTIONS: usize = 1;

// TODO: adjust these.
const TCP_BUFFER_SIZE: usize = 1024;
const HTTP_BUFFER_SIZE: usize = 1024;

pub struct NetworkTransportClient<'stack, 'uri> {
    client: HttpClient<'stack, TcpClient<'stack, MAX_CONCURRENT_TCP_CONNECTIONS>, DnsSocket<'stack>>,
    uri: &'uri str,
}

impl<'a, 'uri> NetworkTransportClient<'a, 'uri> {
    #[must_use]
    pub async fn new(
            tcp_client: &'a TcpClient<'a, MAX_CONCURRENT_TCP_CONNECTIONS>,
            dns_client: &'a DnsSocket<'a>,
            uri: &'uri str
        ) -> Self {
        Self {
            client: HttpClient::new(tcp_client, dns_client),
            uri,
        }
    }

    pub async fn get<'buf>(&mut self, buf: &'buf mut [u8]) -> Result<&'buf mut [u8], Error> {
        let mut handle = self.client.request(Method::GET, self.uri).await
            .map_err(|_| Error::Transport)?;
        let response = handle.send(buf).await.map_err(|_| Error::Transport)?;

        // FIXME: check status.
        // info!("Response status: {}", response.status.0);

        // FIXME: check Content-Type.
        if let Some(content_type) = &response.content_type {

        }

        response.body().read_to_end().await.map_err(|_| Error::Transport)
    }
}
