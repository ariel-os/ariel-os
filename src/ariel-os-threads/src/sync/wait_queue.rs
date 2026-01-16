//! This module provides a generic thread wait queue.

#![deny(missing_docs)]

use core::cell::UnsafeCell;

use critical_section::CriticalSection;

use crate::{ThreadState, threadlist::ThreadList};

/// An [`WaitQueue`], allowing threads to wait for or be notified by other threads.
///
/// Similar to [`Event`], but without any state.
pub struct WaitQueue {
    waiters: UnsafeCell<ThreadList>,
}

// Safety: `WaitQueue`'s methods are safe to call from multiple threads through using critical
// sections.
unsafe impl Sync for WaitQueue {}

impl WaitQueue {
    /// Creates a new [`WaitQueue`].
    #[must_use]
    pub const fn new() -> Self {
        Self {
            waiters: UnsafeCell::new(ThreadList::new()),
        }
    }

    /// Waits for this [`WaitQueue`] to be notified (blocking).
    /// # Panics
    ///
    /// Panics if this is called outside of a thread context.
    pub fn wait(&self) {
        critical_section::with(|cs| self.wait_cs(cs));
    }

    fn wait_cs(&self, cs: CriticalSection<'_>) {
        let waiters = unsafe { &mut *self.waiters.get() };
        waiters.put_current(cs, ThreadState::WaitQueueBlocked);
    }

    /// Waits for this [`WaitQueue`] to be notified, with deadline (blocking).
    ///
    /// # Panics
    ///
    /// Panics if this is called outside of a thread context.
    pub fn wait_until(&self, deadline: embassy_time::Instant) -> bool {
        ariel_os_debug::log::trace!("WaitQueue::wait_until()");
        // Safety:
        // `on_timeout` takes care of removing the thread from the threadlist.
        unsafe {
            crate::timeout::with_deadline(
                deadline,
                |cs| {
                    self.wait_cs(cs);
                },
                |cs| {
                    ariel_os_debug::log::trace!("WaitQueue::wait_until() timeout");
                    #[expect(unused_unsafe)]
                    let waiters = unsafe { &mut *self.waiters.get() };
                    !waiters.remove_current(cs)
                },
            )
        }
    }

    /// Waits for this [`WaitQueue`] to be notified, with deadline and check fn (blocking).
    ///
    /// # Panics
    ///
    /// Panics if this is called outside of a thread context.
    pub fn wait_until_with_check(
        &self,
        deadline: embassy_time::Instant,
        check: impl Fn(CriticalSection<'_>) -> bool,
    ) -> bool {
        ariel_os_debug::log::trace!("WaitQueue::wait_until()");
        // Safety:
        // `on_timeout` takes care of removing the thread from the threadlist.
        unsafe {
            crate::timeout::with_deadline_check(
                deadline,
                check,
                |cs| {
                    self.wait_cs(cs);
                },
                |cs| {
                    ariel_os_debug::log::trace!("WaitQueue::wait_until_with_check() timeout");
                    #[expect(unused_unsafe)]
                    let waiters = unsafe { &mut *self.waiters.get() };
                    waiters.remove_current(cs);
                },
            )
        }
    }

    /// Notify all waiters.
    pub fn notify_all(&self) {
        critical_section::with(|cs| {
            let waiters = unsafe { &mut *self.waiters.get() };
            while waiters.pop(cs).is_some() {}
        });
    }
    /// Notify one waiter.
    pub fn notify_one(&self) {
        critical_section::with(|cs| {
            let waiters = unsafe { &mut *self.waiters.get() };
            #[allow(unused_variables, reason = "log macro sometimes doesn't use this")]
            let res = waiters.pop(cs);
            ariel_os_debug::log::trace!("WaitQueue::notify_one() notifying {:?}", res);
        });
    }
}

impl Default for WaitQueue {
    fn default() -> Self {
        Self::new()
    }
}
