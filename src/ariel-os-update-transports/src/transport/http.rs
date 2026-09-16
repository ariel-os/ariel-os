use embassy_net::{
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
    Stack,
};
use reqwless::{
    client::HttpClient,
    request::Method,
};

use crate::Error;

const MAX_CONCURRENT_CONNECTIONS: usize = 1;

// TODO: adjust these.
const TCP_BUFFER_SIZE: usize = 1024;
const HTTP_BUFFER_SIZE: usize = 1024;

#[ouroboros::self_referencing]
pub struct NetworkTransportClientState<'a> {
    tcp_client: TcpClient<'a, MAX_CONCURRENT_CONNECTIONS>,
    dns_client: DnsSocket<'a>,
    #[borrows(tcp_client, dns_client)]
    tcp_client_state: TcpClientState<MAX_CONCURRENT_CONNECTIONS, TCP_BUFFER_SIZE, TCP_BUFFER_SIZE>,
}

// impl<'a> NetworkTransportClientState<'a> {
//     #[must_use]
//     pub async fn new(stack: Stack<'static>) -> Self {
//         let stack = ariel_os_embassy::net::network_stack().await.unwrap();
//
//         let tcp_client_state =
//             TcpClientState::<MAX_CONCURRENT_CONNECTIONS, TCP_BUFFER_SIZE, TCP_BUFFER_SIZE>::new();
//         let tcp_client = TcpClient::new(stack, &tcp_client_state);
//         let dns_client = DnsSocket::new(stack);
//
//         Self {
//             tcp_client_state,
//             tcp_client,
//             dns_client,
//         }
//     }
// }

#[ouroboros::self_referencing]
pub struct NetworkTransportClient<'stack, 'uri> {
    state: NetworkTransportClientState<'stack>,
    #[borrows(state)]
    client: HttpClient<'stack, TcpClient<'stack, MAX_CONCURRENT_CONNECTIONS>, DnsSocket<'stack>>,
    uri: &'uri str,
}

impl<'a, 'uri> NetworkTransportClient<'a, 'uri> {
    #[must_use]
    pub async fn new(state: NetworkTransportClientState<'a>, uri: &'uri str) -> Self {
        Self {
            state,
            client: HttpClient::new(state.borrow_tcp_client(), &state.borrow_dns_client()),
            uri,
        }
    }

    pub async fn get(&self, buf: &mut [u8]) -> Result<(), Error> {
        let mut handle = self.client.request(Method::GET, self.uri).await?;
        let response = handle.send(buf).await.map_err(|_| Error::Transport)?;

        // FIXME: check status.
        // info!("Response status: {}", response.status.0);

        // FIXME: check Content-Type.
        if let Some(ref content_type) = response.content_type {

        }

        response.body().read_to_end().await.map_err(|_| Error::Transport)
    }
}
