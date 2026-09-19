use alloc::vec::Vec;
use ariel_os::{
    log::*,
    net,
    reexports::embassy_net,
};
use embassy_net::{
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
};
use reqwless::{
    client::{HttpClient, TlsConfig, TlsVerify},
    request::Method,
};

// RFC8449: TLS 1.3 encrypted records are limited to 16 KiB + 256 bytes.
const MAX_ENCRYPTED_TLS_13_RECORD_SIZE: usize = 16640;
// Required by `embedded_tls::TlsConnection::new()`.
const TLS_READ_BUFFER_SIZE: usize = MAX_ENCRYPTED_TLS_13_RECORD_SIZE;
// Can be smaller than the read buffer (could be adjusted: trade-off between memory usage and not
// splitting large writes into multiple records).
const TLS_WRITE_BUFFER_SIZE: usize = 4096;

const TCP_BUFFER_SIZE: usize = 1024;
const HTTP_BUFFER_SIZE: usize = 1024;

const MAX_CONCURRENT_CONNECTIONS: usize = 2;


pub async fn fetch_payload(uri: &str) -> Result<Vec<u8>, ()> {
    let stack = net::network_stack().await.unwrap();

    info!("Trying stuff");
    let tcp_client_state =
        TcpClientState::<MAX_CONCURRENT_CONNECTIONS, TCP_BUFFER_SIZE, TCP_BUFFER_SIZE>::new();
    let tcp_client = TcpClient::new(stack, &tcp_client_state);
    let dns_client = DnsSocket::new(stack);

    let tls_seed: u64 = rand_core::RngCore::next_u64(&mut ariel_os::random::crypto_rng());

    let mut tls_rx_buffer = [0; TLS_READ_BUFFER_SIZE];
    let mut tls_tx_buffer = [0; TLS_WRITE_BUFFER_SIZE];

    // We do not authenticate the server in this example, as that would require setting up a PSK
    // with the server.
    let tls_verify = TlsVerify::None;
    let tls_config = TlsConfig::new(tls_seed, &mut tls_rx_buffer, &mut tls_tx_buffer, tls_verify);

    let mut client = HttpClient::new_with_tls(&tcp_client, &dns_client, tls_config);

    stack.wait_config_up().await;

    match send_http_get_request(&mut client, uri).await {
        Err(err) => {
            error!(
                "Error while sending an HTTP request: {:?}",
                Debug2Format(&err)
            );
            return Err(())
        },
        Ok(payload) => return Ok(payload),
    }
}

async fn send_http_get_request(
    client: &mut HttpClient<'_, TcpClient<'_, MAX_CONCURRENT_CONNECTIONS>, DnsSocket<'_>>,
    url: &str,
) -> Result<Vec<u8>, reqwless::Error> {
    let mut http_rx_buf = [0; HTTP_BUFFER_SIZE];

    let mut handle = client.request(Method::GET, url).await?;
    let response = handle.send(&mut http_rx_buf).await?;

    info!("Response status: {}", response.status.0);

    if let Some(ref content_type) = response.content_type {
        info!("Response Content-Type: {}", content_type.as_str());
    }

    if let Ok(body) = response.body().read_to_end().await {
        return Ok(body.to_vec())
    }
    return Ok(Vec::new())
}
