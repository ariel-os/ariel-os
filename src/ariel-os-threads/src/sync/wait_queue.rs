//! This module provides a generic thread wait queue.

#![deny(missing_docs)]

use core::cell::UnsafeCell;

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
        critical_section::with(|cs| {
            let waiters = unsafe { &mut *self.waiters.get() };
            waiters.put_current(cs, ThreadState::WaitQueueBlocked);
        });
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
            waiters.pop(cs)
        });
    }
}

impl Default for WaitQueue {
    fn default() -> Self {
        Self::new()
    }
}
