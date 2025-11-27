use core::task::{RawWaker, RawWakerVTable, Waker};

use critical_section::CriticalSection;
use embassy_time::Duration;

use crate::{SCHEDULER, ThreadId, ThreadState, thread_flags::ThreadFlags};

const THREAD_FLAG_TIMEOUT: ThreadFlags = 2; // TODO: find more appropriate value

fn wake(ptr: *const ()) {
    #[expect(clippy::cast_possible_truncation)]
    let thread_id = ThreadId::new(ptr as usize as u8);
    SCHEDULER.with_mut(|mut scheduler| {
        if let Some(deadline) = scheduler.threads[usize::from(thread_id)].deadline {
            let now = embassy_time_driver::now();
            if now >= deadline {
                ariel_os_debug::log::debug!(
                    "timer for {:?} expired, triggering thread (deadline={:?}, now={:?})",
                    thread_id,
                    deadline,
                    now
                );
                scheduler.threads[usize::from(thread_id)].deadline = None;
                scheduler.flag_set(thread_id, THREAD_FLAG_TIMEOUT);
                match scheduler.get_state(thread_id) {
                    Some(ThreadState::Running) => {}
                    _ => {
                        scheduler.set_state(thread_id, ThreadState::Running);
                    }
                }
            } else {
                ariel_os_debug::log::debug!(
                    "timer for {:?} not due yet (deadline={:?}, now={:?})",
                    thread_id,
                    deadline,
                    now
                );
            }
        } else {
            ariel_os_debug::log::debug!(
                "timer for {:?} now={:?} no deadline set)",
                thread_id,
                embassy_time_driver::now()
            );
        }
    });
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    // clone
    |ptr| RawWaker::new(ptr, &VTABLE),
    wake,
    wake,
    |_ptr| {},
);

fn schedule_thread_wakeup(thread_id: ThreadId, deadline: u64) {
    let raw_waker = RawWaker::new(usize::from(thread_id) as *const (), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw_waker) };
    embassy_time_driver::schedule_wake(deadline, &waker);
}

/// Sets up the deadline timer.
/// Returns true if the deadline was in the future.
///
/// # Panics
/// - panics when not called from a thread
fn set_deadline(cs: CriticalSection<'_>, deadline: u64) -> bool {
    // The two `SCHEDULER.with_mut_cs()` calls are necessary because schedule_thread_wakeup() might
    // call `wake()` on the waker, which also accesses SCHEDULER.
    let thread_id = SCHEDULER.with_mut_cs(cs, |mut scheduler| {
        let thread = scheduler.current().expect("must be called from a thread");
        thread.deadline = Some(deadline);
        thread.flags &= !THREAD_FLAG_TIMEOUT;
        thread.tid
    });

    ariel_os_debug::log::debug!("setting deadline for {:?} to {:?}", thread_id, deadline);
    schedule_thread_wakeup(thread_id, deadline);

    SCHEDULER.with_mut_cs(cs, |mut scheduler| {
        let thread = scheduler.current().expect("must be called from a thread");
        thread.deadline.is_some()
    })
}

/// Clears the current thread's deadline timer.
/// Returns true if the deadline had not been reached yet.
///
/// # Panics
/// - panics when not called from a thread
fn clear_deadline(cs: CriticalSection<'_>) -> bool {
    let (did_clear, thread_id) = SCHEDULER.with_mut_cs(cs, |mut scheduler| {
        let thread = scheduler.current().expect("must be called from a thread");
        let did_clear = thread.deadline.take().is_some();
        ariel_os_debug::log::debug!(
            "clear_deadline() for {:?}: {} at {}",
            thread.tid,
            true,
            embassy_time_driver::now()
        );
        (did_clear, thread.tid)
    });
    if did_clear {
        schedule_thread_wakeup(thread_id, 0);
    }
    did_clear
}

/// Runs the given function with a deadline timer.
///
/// If the deadline is in the past the function will not be called.
/// If the deadline is reached before the function returns, the thread will be set to runnable
/// state, and `on_timeout()` will be called after the function returns.
///
/// Returns false if the deadline was reached, and `on_timeout()` was called.
///
/// # Safety
/// This will set the calling thread to `Runnable` after the timeout expires. Caller must ensure safety implications of that.
pub(crate) unsafe fn with_deadline(
    deadline: embassy_time::Instant,
    f: impl FnOnce(CriticalSection<'_>),
    on_timeout: impl FnOnce(CriticalSection<'_>) -> bool,
) -> bool {
    let deadline = deadline.as_ticks();
    critical_section::with(|cs| {
        if set_deadline(cs, deadline) {
            // The deadline was in the future, so we can run the function
            f(cs);
            ariel_os_debug::log::debug!("with_deadline: cs at {}", embassy_time_driver::now());
            true
        } else {
            // The deadline was in the past, so we don't run the function
            ariel_os_debug::log::debug!("with_deadline: deadline {} was in the past", deadline);
            false
        }
    }) && critical_section::with(|cs| {
        if clear_deadline(cs) {
            ariel_os_debug::log::debug!("with_deadline: cleared deadline {}", deadline);
            false
        } else {
            ariel_os_debug::log::debug!("with_deadline: timeout deadline {}", deadline);
            on_timeout(cs)
        }
    })
}

/// Put the current thread to sleep for the given duration.
pub fn sleep(duration: Duration) {
    let deadline = embassy_time::Instant::now().saturating_add(duration);
    sleep_until(deadline);
}

/// Put the current thread to sleep until the given deadline.
pub fn sleep_until(deadline: embassy_time::Instant) {
    let deadline = deadline.as_ticks();
    critical_section::with(|cs: CriticalSection<'_>| {
        if set_deadline(cs, deadline) {
            crate::park();
        }
    });
}
