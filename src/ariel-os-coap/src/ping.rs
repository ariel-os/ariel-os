//! # Ping
//!
//! ## Ping Semantics
//! ### POST
//! Will create the job and queue it.
//!
//! ### GET
//! Will COPY out the job INCLUDING ALL THE IPs so don't spam this one. For
//! example a Ping Job (in its current format) is at max (16 ip's to ping) 512 bytes.
//!
//! ### DELETE
//! Will *wait* for the job to finish a *round*. *round* being a single decrement
//! of the count field in the POST request data used to instantiate the job in the first place.
//! *wait* meaning that the actualy Job will be removed from the Managers vector of Jobs. If there
//! is a in progress execution of the `run` method (of Manager) this run method makes a internal
//! copy of the Job and if the delete occurs after this copy has been made, the copy will execute
//! its job (Ping, Broadcast, scan, etc.) and the run function will return. The next call of the
//! run function will (asuming this job is not a oneshot) try and run the job again. This
//! will return None at which point the job can be considered completly delete.
//! ## Schema
//! TODO
//!
//! ## Examples
//! ```
//! TODO
//! ```
//!

use ariel_os_embassy::asynch::Spawner;
use coap_handler::Handler;
use coap_message::Code as _;
use coap_message::MessageOption as _;
use coap_message::{MinimalWritableMessage, MutableWritableMessage, ReadableMessage};
use coap_message_utils::Error;
use coap_numbers::code::DELETED;
use coap_numbers::code::INTERNAL_SERVER_ERROR;
use coap_numbers::code::{CONTENT, CREATED, NOT_FOUND};
use coap_numbers::option::URI_PATH;
use core::cell::RefCell;
use core::net::{Ipv4Addr, Ipv6Addr};
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering::Relaxed;
use critical_section::Mutex;
use embassy_net::icmp::PacketMetadata;
use embassy_net::icmp::ping::{PingManager, PingParams};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{self, Channel};
use heapless::vec::Vec;
use minicbor::Decode;
use minicbor::{Decoder, Encode, Encoder};

///Max length of the user request IP list, so for a single ping job you can only have 16 targets at
///one time.
pub(crate) const MAX_REQ_SZ: usize = 16;

///Num of tasks that can concurently operate on the ping job queue.
const PING_WORKERS: usize = 4;

static MEASURMENT_MANAGER: static_cell::StaticCell<MeasurmentManager> =
    static_cell::StaticCell::new();

///ID generator.
static ID_GEN: AtomicU64 = AtomicU64::new(0);

///Queue for the worker tasks to pull job IDs from.
static JOB_ID_QUEUE: Channel<CriticalSectionRawMutex, u64, 32> = channel::Channel::new();

pub(crate) static PING_JOB: &str = "ping";

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum IpWrapper {
    V4(IpV4Wrapper),
    V6(IpV6Wrapper),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IpV4Wrapper(pub(crate) Ipv4Addr);
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct IpV6Wrapper(pub(crate) Ipv6Addr);

#[derive(Clone, Debug, PartialEq, Decode)]
pub(crate) enum JobType {
    #[n(0)]
    Ping,
    #[n(1)]
    PortScan,
    #[n(2)]
    Broadcast,
}

pub(crate) struct CborPingSchema {
    pub(crate) kype: JobType,
    pub(crate) ip_list: Vec<(IpWrapper, Stats), MAX_REQ_SZ>,
    pub(crate) count: Option<u32>,
}

///Handeler, first thing that touches the incoming data.
///
///This is only way to enable the Ping funcitonality via CoAP. The schema/semantics are defined at
///the module level `ping` docs.
pub struct MeasurmentHandler {
    manager: &'static MeasurmentManager,
}

impl Default for MeasurmentHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl MeasurmentHandler {
    ///Constructor.
    #[must_use]
    pub fn new() -> Self {
        MeasurmentHandler {
            manager: MeasurmentManager::new(),
        }
    }
}

///Exposed due to trait
pub struct ReqData {
    x: Internal,
}

pub(crate) enum Internal {
    P,
    D(u64),
    G(u64),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PJob {
    pub(crate) count: u32,
    pub(crate) completed: i32,
    pub(crate) has_errors: bool,
    pub(crate) target: Vec<(IpWrapper, Stats), 16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Encode, Decode, Default)]
pub(crate) struct Stats {
    #[n(0)]
    ave_rtt: u64,
    #[n(1)]
    sent: u32,
    #[n(2)]
    received: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct JobEntry {
    pub(crate) id: u64,
    pub(crate) job: Job,
}

#[derive(Clone, Debug, PartialEq, Encode)]
pub(crate) enum Job {
    #[n(0)]
    Ping(#[n(1)] PJob),
    #[n(2)]
    TraceRoung(#[n(3)] PJob),
    #[n(4)]
    MuiltyCast(#[n(5)] PJob),
}

struct MeasurmentManager {
    id_table: critical_section::Mutex<RefCell<Vec<Option<JobEntry>, 16>>>,
}

impl MeasurmentManager {
    ///Panics if cell is full
    fn new() -> &'static Self {
        MEASURMENT_MANAGER.init_with(|| MeasurmentManager {
            id_table: Mutex::new(RefCell::new(Vec::new())),
        })
    }

    /// Initializez the [`MeasurmentManaget`]
    ///
    /// This function connects to the configured endpoint and waits for a response.
    ///
    /// # Panics
    ///
    /// Panics if the internal state is invalid or if a required invariant is violated.
    fn init(&'static self, config: CborPingSchema) -> Result<u64, ()> {
        if config.ip_list.is_empty() || config.ip_list.len() > 16 {
            return Err(());
        }

        //Single threaded
        let id = ID_GEN.fetch_add(1, Relaxed);
        let job = match config.kype {
            JobType::Ping => {
                let count = config.count.unwrap_or(1);
                Job::Ping(PJob {
                    target: config.ip_list,

                    count,
                    completed: 0,
                    has_errors: false,
                })
            }
            _ => todo!(),
        };

        critical_section::with(|cs| {
            let mut table = self.id_table.borrow_ref_mut(cs);
            let index = table
                .iter()
                .position(core::option::Option::is_none)
                .ok_or(())?;
            let job = JobEntry { id, job };
            *table.get_mut(index).unwrap() = Some(job);
            Ok(id)
        })
    }

    /// # Panics if somehow the index is wrong. This should be imposible.
    fn delete(&'static self, id: u64) -> Option<JobEntry> {
        critical_section::with(|cs| {
            let mut table = self.id_table.borrow_ref_mut(cs);
            let i = table.iter().position(|x| match x {
                Some(x) => x.id == id,
                None => false,
            })?;
            let val = table.get(i).unwrap().clone();
            table.remove(i);
            val
        })
    }

    /// # Panics if somehow the index is wrong. This should be imposible.
    fn get(&'static self, id: u64) -> Option<JobEntry> {
        critical_section::with(|cs| {
            let table = self.id_table.borrow_ref_mut(cs);
            let i = table.iter().position(|x| match x {
                Some(x) => x.id == id,
                None => false,
            })?;
            table.get(i).unwrap().clone()
        })
    }

    //TODO: There should be some Runnable trait probably you can have the output of the ip
    //param creation for loop be a asos type

    /// # Panics if somehow the index is wrong. This should be imposible.
    async fn run<'a>(&self, id: u64, ping_manager: &mut PingManager<'a>) -> Option<u32> {
        let mut val = critical_section::with(|cs| {
            let table = self.id_table.borrow_ref_mut(cs);
            let i = table.iter().position(|x| match x {
                Some(x) => x.id == id,
                None => false,
            })?;
            table.get(i).unwrap().clone()
        })?;

        match &mut val.job {
            Job::Ping(PJob {
                target,
                count,
                completed,
                has_errors,
            }) => {
                for (ip, stat) in target.iter_mut() {
                    let params = match ip {
                        IpWrapper::V4(ip_v4_wrapper) => PingParams::new(ip_v4_wrapper.0),
                        IpWrapper::V6(ip_v6_wrapper) => PingParams::new(ip_v6_wrapper.0),
                    };
                    match ping_manager.ping(&params).await {
                        Ok(x) => {
                            stat.ave_rtt = x.as_micros();
                            *completed += 1;
                            *count -= 1;
                        }
                        Err(_e) => {
                            *has_errors = true;
                            *completed -= 1;
                            *count -= 1;
                        }
                    }
                }
                Some(*count)
            }
            Job::TraceRoung(_pjob) => todo!(),
            Job::MuiltyCast(_pjob) => todo!(),
        }
    }
}

#[ariel_os_macros::task(pool_size = PING_WORKERS)]
async fn ping_worker(job_manager: &'static MeasurmentManager) -> ! {
    let stack = loop {
        if let Some(x) = ariel_os_embassy::net::network_stack().await {
            break x;
        }
    };

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
        let id = JOB_ID_QUEUE.receive().await;
        while job_manager.run(id, &mut ping_manager).await.is_some() {}
    }
}

#[ariel_os_macros::spawner(autostart)]
/// # Panics
/// If this panics then embassy cant spawn any more tasks and there are bigger problems to worry
/// about.
fn spawn_task_pool(spawner: Spawner) {
    let manager = MeasurmentManager::new();
    for _ in 0..PING_WORKERS {
        spawner.spawn(ping_worker(manager)).unwrap();
    }
}

impl Handler for MeasurmentHandler {
    type RequestData = ReqData;
    type ExtractRequestError = Error;
    type BuildResponseError<M: MinimalWritableMessage> = M::UnionError;

    fn extract_request_data<M: ReadableMessage>(
        &mut self,
        request: &M,
    ) -> Result<Self::RequestData, Self::ExtractRequestError> {
        match request.code().into() {
            coap_numbers::code::GET => {
                let id = request
                    .options()
                    .find(|opt| opt.number() == URI_PATH)
                    .and_then(|x| x.value_str().and_then(|y| y.parse::<u64>().ok()))
                    .ok_or(Error::bad_request())?;

                Ok(ReqData { x: Internal::G(id) })
            }

            coap_numbers::code::DELETE => {
                let id = request
                    .options()
                    .find(|opt| opt.number() == URI_PATH)
                    .and_then(|x| x.value_str().and_then(|y| y.parse::<u64>().ok()))
                    .ok_or(Error::bad_request())?;

                Ok(ReqData { x: Internal::D(id) })
            }

            coap_numbers::code::POST => {
                let payload: &[u8] = request.payload();
                let mut de = Decoder::new(payload);
                let config = de
                    .decode::<CborPingSchema>()
                    .map_err(|_| Error::unsupported_content_format())?;

                self.manager
                    .init(config)
                    .map_err(|()| Error::service_unavailable())?;

                Ok(ReqData { x: Internal::P })
            }
            _ => Err(Error::method_not_allowed()),
        }
    }

    fn estimate_length(&mut self, _request: &Self::RequestData) -> usize {
        //Taken from coap_messages
        1280 - 40 - 4 // does this correclty calculate the IPv6 minimum MTU?
    }

    fn build_response<M: MutableWritableMessage>(
        &mut self,
        response: &mut M,
        request: Self::RequestData,
    ) -> Result<(), Self::BuildResponseError<M>> {
        match request.x {
            Internal::P => {
                response.set_code(M::Code::new(CREATED)?);
                Ok(())
            }
            Internal::G(id) => {
                if let Some(x) = self.manager.get(id) {
                    let out = &mut [0; 1024];
                    let mut e = Encoder::new(&mut out[..]);

                    match e.encode(x) {
                        Ok(_) => {}
                        Err(_) => response.set_code(M::Code::new(INTERNAL_SERVER_ERROR)?),
                    }
                    response.set_code(M::Code::new(CONTENT)?);
                    response.set_payload(out)?;
                    Ok(())
                } else {
                    response.set_code(M::Code::new(NOT_FOUND)?);
                    Ok(())
                }
            }
            Internal::D(id) => {
                if self.manager.delete(id).is_some() {
                    response.set_code(M::Code::new(DELETED)?);
                    Ok(())
                } else {
                    response.set_code(M::Code::new(INTERNAL_SERVER_ERROR)?);
                    Ok(())
                }
            }
        }
    }
}
