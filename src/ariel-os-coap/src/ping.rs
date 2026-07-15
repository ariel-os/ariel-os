use ariel_os_embassy::api::time::Instant;
use ariel_os_embassy::asynch::Spawner;
use coap_handler::Handler;
use coap_message::Code;
use coap_message::{MinimalWritableMessage, MutableWritableMessage, ReadableMessage};
use coap_message_utils::Error;
use core::cell::RefCell;
use core::default;
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use core::sync::atomic::AtomicI32;
use core::sync::atomic::Ordering::Relaxed;
use embassy_net::icmp::PacketMetadata;
use embassy_net::icmp::ping::{PingManager, PingParams};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use heapless::vec::Vec;
use minicbor::decode::ArrayIterWithCtx;
use minicbor::{Decode, Decoder, Encode, Encoder};

//TODO: Investigate adding observe functionality to the coap server
//so that we dont need to keep state

/// Queue of pending ping requests shared by the worker pool.
static PING_QUEUE: Channel<CriticalSectionRawMutex, InternalTarget, 32> = Channel::new();

const PING_WORKERS: usize = 4;
const MAX_REQ_SZ: usize = 16;
static ID_COUNTER: AtomicI32 = AtomicI32::new(0);

//NOTE: We would really just want to have some sort of PingJob that the caller
//can define in the POST

/// Shared statistics storage.
/// Currently global for the proof-of-concept.
static STAT_STORAGE: critical_section::Mutex<RefCell<heapless::Vec<PingStats, 16>>> =
    critical_section::Mutex::new(RefCell::new(heapless::Vec::new()));

#[ariel_os_macros::spawner(autostart)]
fn spawn_task_pool(spawner: Spawner) {
    for _ in 0..PING_WORKERS {
        spawner.spawn(ping_worker()).unwrap();
    }
}

#[ariel_os_macros::task(pool_size = PING_WORKERS)]
async fn ping_worker() -> ! {
    //Coap run also does this. Im not sure we can recover
    let stack = ariel_os_embassy::net::network_stack().await.unwrap();

    let mut rx_buffer = [0; 256];
    let mut tx_buffer = [0; 256];
    let mut rx_meta = [PacketMetadata::EMPTY];
    let mut tx_meta = [PacketMetadata::EMPTY];

    let mut ping_manager = PingManager::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );

    loop {
        let internal_target = PING_QUEUE.receive().await;

        let mut ping_params = match internal_target.t.ip_addr {
            IpWrapper::V4(ip_v4_wrapper) => PingParams::new(ip_v4_wrapper.0),
            IpWrapper::V6(ip_v6_wrapper) => PingParams::new(ip_v6_wrapper.0),
        };

        ping_params
            .set_count(internal_target.t.count.unwrap_or(1))
            .set_hop_limit(internal_target.t.hop_limit);

        let timestamp = Instant::now();
        match ping_manager.ping(&ping_params).await {
            Ok(_) => {
                let ip: [u8; 16] = ping_params.target().map_or_else(
                    || [0; 16],
                    |x| match x {
                        IpAddr::V4(ipv4) => ipv4.to_ipv6_mapped().octets(),
                        IpAddr::V6(ipv6) => ipv6.octets(),
                    },
                );

                let elapsed = Instant::now()
                    .checked_duration_since(timestamp)
                    .expect("The clock wrapped around");

                let stats = PingStats::new(ip, elapsed.as_micros());
                critical_section::with(|cs| {
                    let mut storage = STAT_STORAGE.borrow_ref_mut(cs);

                    if let Some(existing) = storage.iter_mut().find(|s| s.ip == stats.ip) {
                        // Mutate the existing entry.
                        existing.update_from(&stats);
                    } else {
                        // Try to insert a new one. If full, ignore it.
                        let _ = storage.push(stats);
                    }
                });
            }

            Err(e) => todo!("We will have to add a error code section in the stats"),
        }
    }
}

enum IpWrapper {
    V4(IpV4Wrapper),
    V6(IpV6Wrapper),
}

//TODO: Despite net:: Ip stuff being in core for a while now minicbor still has
//it behind a std cfg flag
//UPDATE: PR is apparently in the works and will be pushed the week of july 13th
pub struct IpV4Wrapper(Ipv4Addr);
pub struct IpV6Wrapper(Ipv6Addr);

impl<'b, C> Decode<'b, C> for IpV4Wrapper {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let octets: minicbor::bytes::ByteArray<4> = Decode::decode(d, ctx)?;
        Ok(IpV4Wrapper(<[u8; 4]>::from(octets).into()))
    }
}

impl<'b, C> Decode<'b, C> for IpV6Wrapper {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let octets: minicbor::bytes::ByteArray<16> = Decode::decode(d, ctx)?;
        Ok(IpV6Wrapper(<[u8; 16]>::from(octets).into()))
    }
}

impl<'b, C> Decode<'b, C> for IpWrapper {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let p = d.position();
        if Some(2) != d.array()? {
            return Err(minicbor::decode::Error::message("expected enum (2-element array)").at(p));
        }
        let p = d.position();
        match d.i64()? {
            0 => Ok(IpWrapper::V4(IpV4Wrapper::decode(d, ctx)?)),
            1 => Ok(IpWrapper::V6(IpV6Wrapper::decode(d, ctx)?)),
            n => Err(minicbor::decode::Error::unknown_variant(n).at(p)),
        }
    }
}

///This is the mininum CBOR element for us to do work with
#[derive(Decode)]
struct Target {
    #[n(0)]
    ip_addr: IpWrapper,
    #[n(1)]
    count: Option<u16>,
    #[n(2)]
    hop_limit: Option<u8>,
    // #[n(3)]
    // latency: Option<u32>,
}

struct InternalTarget {
    t: Target,
    id: i32,
}

struct CborPingSchema {
    targets: Vec<Target, MAX_REQ_SZ>,
}

impl<'b, 'c, C> Decode<'b, C> for CborPingSchema {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let arr_travle: ArrayIterWithCtx<'_, '_, C, Target> = d.array_iter_with(ctx)?;
        let mut out: Vec<Target, MAX_REQ_SZ> = Vec::new();
        for i in arr_travle {
            out.push(i?)
                .map_err(|_| minicbor::decode::Error::message("Vector push failed"))?;
        }
        Ok(CborPingSchema { targets: out })
    }
}

struct PingHandler {
    vals: CborPingSchema,
}

#[derive(Encode, Default, Debug, PartialEq)]
struct PingStats {
    #[n(0)]
    ip: [u8; 16],
    #[n(1)]
    ave_latency: u64,
    #[n(2)]
    ping_count: u64,
}

impl PingStats {
    fn new(ip: [u8; 16], ave_latency: u64) -> Self {
        Self {
            ip,
            ave_latency,
            ping_count: default::Default::default(),
        }
    }

    fn update_from(&mut self, stats: &Self) {
        self.ave_latency = stats.ave_latency;
        self.ping_count += 1;
    }
}

enum ReqData {
    P(CborPingSchema),
    G,
}

impl Handler for PingHandler {
    type RequestData = ReqData;

    type ExtractRequestError = Error;

    type BuildResponseError<M: MinimalWritableMessage> = M::UnionError;

    fn extract_request_data<M: ReadableMessage>(
        &mut self,
        request: &M,
    ) -> Result<Self::RequestData, Self::ExtractRequestError> {
        match request.code().into() {
            coap_numbers::code::GET => Ok(ReqData::G),
            coap_numbers::code::POST => {
                let payload: &[u8] = request.payload();
                if payload.is_empty() || payload.len() > MAX_REQ_SZ {
                    return Err(Error::bad_request());
                }
                let mut de = Decoder::new(payload);
                Ok(ReqData::P(
                    de.decode::<CborPingSchema>()
                        .map_err(|_| Error::unsupported_content_format())?,
                ))
            }
            _ => Err(Error::method_not_allowed()),
        }
    }

    fn estimate_length(&mut self, request: &Self::RequestData) -> usize {
        //Taken from coap_messages
        1280 - 40 - 4 // does this correclty calculate the IPv6 minimum MTU?
    }

    fn build_response<M: MutableWritableMessage>(
        &mut self,
        response: &mut M,
        request: Self::RequestData,
    ) -> Result<(), Self::BuildResponseError<M>> {
        match request {
            ReqData::P(cbor_ping_schema) => {
                for t in cbor_ping_schema.targets {
                    //Relaxed since its all on one thread ?
                    match PING_QUEUE.try_send(InternalTarget {
                        t,
                        id: ID_COUNTER.fetch_add(1, Relaxed),
                    }) {
                        Ok(()) => response.set_code(Code::new(coap_numbers::code::CREATED)?),
                        Err(_) => response.set_code(Code::new(coap_numbers::code::CONFLICT)?),
                    }
                }
            }
            ReqData::G => {
                // Temporary PoC.
                // The semantics of exposing statistics are intentionally left open.
                let stats = critical_section::with(|cs| STAT_STORAGE.borrow_ref_mut(cs).pop())
                    .unwrap_or_default();
                let mut buffer = [0u8; 512];
                let mut encoder = Encoder::new(&mut buffer[..]);
                match encoder.encode(stats) {
                    Ok(_) => response.set_payload(&buffer[..])?,
                    Err(_) => {
                        response.set_code(
                            Code::new(coap_numbers::code::INTERNAL_SERVER_ERROR).unwrap(),
                        );
                    }
                }
            }
            ReqData::BadRequest => {
                response.set_code(Code::new(coap_numbers::code::BAD_REQUEST).unwrap());
            }
        }

        Ok(())
    }
}
