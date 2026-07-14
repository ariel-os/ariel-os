#![no_std]
#![deny(missing_docs)]

//! SNTP support for Ariel OS.
//!
//! This crate uses the `embassy-net` stack provided by Ariel OS and the
//! protocol implementation from `sntpc`.
//!
//! A global [`GlobalClock`] is provided so that the fetched network time is
//! available at any point after the initial synchronization. The monotonic
//! `embassy-time` clock is used to advance the stored timestamp between syncs.
//! With enabling the `sntp` feature in [`ariel-os`], the `sntp_task` is
//! automatically started with an interval of 1 hour to synchronize the clock.
//! The [`now`]-function provides POSIX time which isn't guaranteed to be monotonic.
//!
//! SECURITY: The time provided can only be trusted to the extent that the
//! NTP server can be trusted. SNTP requests are not authenticated.

use ariel_os_log::{debug, error};
use core::cell::Cell;
use core::fmt;
use core::net::{IpAddr, SocketAddr};
use core::ops::AddAssign;
use critical_section::Mutex;
use embassy_executor;
use embassy_net::{
    Stack,
    udp::{PacketMetadata, UdpSocket},
};
use embassy_time::{Duration, Instant, Timer, with_timeout};
pub use sntpc::NtpResult;
use sntpc::{NtpContext, get_time};
use sntpc_net_embassy::UdpSocketWrapper;
use sntpc_time_embassy::EmbassyTimestampGenerator;

/// SNTP port.
pub const NTP_PORT: u16 = 123;

/// Default timeout for a single SNTP request.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(3);

/// Size of the UDP RX payload buffer.
///
/// SNTP packets are 48 bytes, but the buffer is intentionally a bit larger.
pub const DEFAULT_RX_BUFFER_SIZE: usize = 64;

/// Size of the UDP TX payload buffer.
///
/// SNTP packets are 48 bytes, but the buffer is intentionally a bit larger.
pub const DEFAULT_TX_BUFFER_SIZE: usize = 64;

/// Maximum tolerated absolute difference between local estimate and SNTP.
///
/// A larger difference causes [`PlausibilityError::JumpTooLarge`].
pub const MAX_PLAUSIBLE_DIFF: Duration = Duration::from_millis(100);

/// The sleep interval for between synchronizations
pub const SYNC_INTERVAL: Duration = Duration::from_secs(60 * 60);

/// Errors that can occur while fetching SNTP time.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The local UDP socket could not be bound.
    Bind,

    /// The request timed out.
    Timeout,

    /// The SNTP server did not return a usable response.
    Protocol,

    /// The fetched time failed the plausibility check.
    Plausibility(PlausibilityError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind => f.write_str("failed to bind UDP socket"),
            Self::Timeout => f.write_str("SNTP request timed out"),
            Self::Protocol => f.write_str("SNTP protocol error"),
            Self::Plausibility(e) => write!(f, "plausibility check failed: {e}"),
        }
    }
}

/// Reason why a received SNTP timestamp was rejected.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlausibilityError {
    /// The new timestamp differs from the current estimate by more than
    /// [`MAX_PLAUSIBLE_DIFF`].
    JumpTooLarge,
}

impl fmt::Display for PlausibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::JumpTooLarge => write!(f, "timestamp jump too large"),
        }
    }
}

/// A snapshot that anchors an NTP Unix-second value to an [`Instant`].
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone, Copy)]
struct ClockSnapshot {
    /// Unix timestamp in whole seconds at the moment of the snapshot.
    unix: Instant,
    /// Monotonic instant at the moment of the snapshot.
    anchor: Instant,
}

impl ClockSnapshot {
    /// Compute the estimated current Unix timestamp.
    fn now(&self) -> Instant {
        let elapsed = Instant::now().duration_since(self.anchor);
        self.unix.saturating_add(elapsed)
    }
}

/// A global SNTP-backed wall clock.
///
/// After the first successful synchronization, [`GlobalClock::now`] returns the
/// estimated current Unix time derived from the monotonic `embassy-time` clock.
struct GlobalClock {
    inner: Mutex<Cell<Option<ClockSnapshot>>>,
}

impl GlobalClock {
    /// Creates a new, unsynchronized global clock.
    ///
    /// Intended for use as a `static`.
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(Cell::new(None)),
        }
    }

    /// Returns the estimated current Unix timestamp, or `None` if not synchronized yet.
    pub fn now(&self) -> Option<Instant> {
        critical_section::with(|cs| self.inner.borrow(cs).get().map(|s| s.now()))
    }

    /// Validates and stores the given Unix timestamp.
    ///
    /// If the clock was already set, the absolute difference between the current
    /// estimate and the new SNTP value must be ≤ [`MAX_PLAUSIBLE_DIFF`].
    ///
    /// Returns `Err(PlausibilityError)` without updating the clock on failure.
    pub fn update(&self, unix_secs: Instant) -> Result<(), PlausibilityError> {
        if let Some(current) = self.now() {
            let diff = if unix_secs > current {
                unix_secs.duration_since(current)
            } else {
                current.duration_since(unix_secs)
            };
            debug!(
                "SNTP plausibility check: current estimate = {}ms, new value = {}ms, diff = {}ms",
                current.as_millis(),
                unix_secs.as_millis(),
                diff.as_millis()
            );
            if diff > MAX_PLAUSIBLE_DIFF {
                return Err(PlausibilityError::JumpTooLarge);
            }
        }

        critical_section::with(|cs| {
            self.inner.borrow(cs).set(Some(ClockSnapshot {
                unix: unix_secs,
                anchor: Instant::now(),
            }))
        });

        Ok(())
    }
}

/// The process-global SNTP clock.
///
/// Use [`start`] to begin periodic synchronization and [`now`] to read the time.
static GLOBAL_CLOCK: GlobalClock = GlobalClock::new();

/// Returns the current Unix timestamp from [`GLOBAL_CLOCK`], or `None` if not
/// synchronized yet.
pub fn now() -> Option<Instant> {
    GLOBAL_CLOCK.now()
}

/// Task for automated updating of the global clock via sntp
#[embassy_executor::task]
pub async fn sntp_task(stack: Stack<'static>) {
    stack.wait_config_up().await;
    #[cfg(all(feature = "ipv4", not(feature = "ipv6")))]
    let ip = stack.config_v4().unwrap().gateway.unwrap();
    #[cfg(feature = "ipv6")]
    let ip = config.config_v6().unwrap().gateway.unwrap();
    let addr = SocketAddr::new(IpAddr::from(ip), NTP_PORT);
    loop {
        match update_global_clock(stack, addr).await {
            Ok(_) => debug!("SNTP clock updated"),
            Err(_) => error!("SNTP update failed"),
        }

        Timer::after(SYNC_INTERVAL).await;
    }
}

/// Synchronizes [`GLOBAL_CLOCK`] from an SNTP server.
pub async fn update_global_clock(
    stack: Stack<'static>,
    addr: SocketAddr,
) -> Result<NtpResult, Error> {
    let result = fetch_time(stack, addr, DEFAULT_TIMEOUT).await?;
    let mut result_instant = Instant::from_secs(result.sec() as u64);
    result_instant.add_assign(ntp_fraction_to_duration(result.sec_fraction()));
    GLOBAL_CLOCK
        .update(result_instant)
        .map_err(Error::Plausibility)?;
    Ok(result)
}

/// Converts the NTP 32-bit second fraction field into a [`Duration`].
fn ntp_fraction_to_duration(sec_fraction: u32) -> Duration {
    // NTP fractional seconds are fixed-point with 32 fraction bits.
    let micros = ((sec_fraction as u64) * 1_000_000) >> 32;
    Duration::from_micros(micros)
}

/// Makes a single SNTP request to fetch the current time from the given server.
pub async fn fetch_time(
    stack: Stack<'static>,
    addr: SocketAddr,
    timeout: Duration,
) -> Result<NtpResult, Error> {
    let mut rx_meta = [PacketMetadata::EMPTY; 16];
    let mut tx_meta = [PacketMetadata::EMPTY; 16];
    let mut rx_buffer = [0_u8; DEFAULT_RX_BUFFER_SIZE];
    let mut tx_buffer = [0_u8; DEFAULT_TX_BUFFER_SIZE];

    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );

    socket.bind(NTP_PORT).map_err(|_| Error::Bind)?;

    let socket = UdpSocketWrapper::new(socket);
    let context = NtpContext::new(EmbassyTimestampGenerator::default());

    let response = with_timeout(timeout, get_time(addr, &socket, context))
        .await
        .map_err(|_| Error::Timeout)?
        .map_err(|_| Error::Protocol)?;

    Ok(response)
}
