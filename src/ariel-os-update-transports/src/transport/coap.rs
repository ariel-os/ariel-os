use embedded_nal_coap::CoAPRuntimeClient;

const CONCURRENT_REQUESTS: usize = 1;

pub struct NetworkTransportClient<'uri> {
    client: CoAPRuntimeClient<'static, CONCURRENT_REQUESTS>,
    uri: &'uri str,
}
