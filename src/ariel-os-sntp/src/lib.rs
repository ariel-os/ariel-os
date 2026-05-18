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

use ariel_os_log::{debug, error};
use ariel_os_threads::sync::Mutex;
use core::ops::AddAssign;
use core::{fmt, net};
use embassy_executor;
use embassy_net::{
    Stack,
    udp::{PacketMetadata, UdpSocket},
};
use embassy_time::{Duration, Instant, Timer, with_timeout};
pub use sntpc::NtpResult;
use sntpc::{NtpContext, NtpTimestampGenerator, get_time};
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

/// Errors that can occur while fetching SNTP time.
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

// ---------------------------------------------------------------------------
// Global clock
// ---------------------------------------------------------------------------

/// A snapshot that anchors an NTP Unix-second value to an [`Instant`].
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
pub struct GlobalClock {
    inner: Mutex<Option<ClockSnapshot>>,
}

impl GlobalClock {
    /// Creates a new, unsynchronized global clock.
    ///
    /// Intended for use as a `static`.
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }

    /// Returns the estimated current Unix timestamp, or `None` if not synchronized yet.
    pub fn now(&self) -> Option<Instant> {
        self.inner.lock().as_ref().map(|s| s.now())
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

        *self.inner.lock() = Some(ClockSnapshot {
            unix: unix_secs,
            anchor: Instant::now(),
        });

        Ok(())
    }
}

/// The process-global SNTP clock.
///
/// Use [`start`] to begin periodic synchronization and [`now`] to read the time.
pub static GLOBAL_CLOCK: GlobalClock = GlobalClock::new();

/// Starts the SNTP background task.
///
/// Spawns a task that periodically synchronizes [`GLOBAL_CLOCK`] with the given
/// NTP server. The executor spawner is obtained internally – no external
/// `Spawner` needs to be passed.
///
/// Call this from any async context, e.g. an `#[ariel_os::task(autostart)]`:
///
/// ```rust
/// ariel_os_sntp::start(stack, server_addr, Duration::from_secs(60)).await;
/// ```
pub async fn start(stack: Stack<'static>, addr: net::SocketAddr, interval: Duration) {
    // SAFETY: same pattern used throughout ariel-os-embassy
    #![expect(unsafe_code)]
    let spawner = unsafe { embassy_executor::Spawner::for_current_executor().await };
    spawner.must_spawn(sntp_task(stack, addr, interval));
}

/// Returns the current Unix timestamp from [`GLOBAL_CLOCK`], or `None` if not
/// synchronized yet.
pub fn now() -> Option<Instant> {
    GLOBAL_CLOCK.now()
}

#[embassy_executor::task]
async fn sntp_task(stack: Stack<'static>, addr: net::SocketAddr, interval: Duration) {
    loop {
        match update_global_clock(stack, addr).await {
            Ok(_) => debug!("SNTP clock updated"),
            Err(_) => error!("SNTP update failed"),
        }

        Timer::after(interval).await;
    }
}

/// Synchronizes [`GLOBAL_CLOCK`] from an SNTP server.
pub async fn update_global_clock(
    stack: Stack<'static>,
    addr: net::SocketAddr,
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
    addr: net::SocketAddr,
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
