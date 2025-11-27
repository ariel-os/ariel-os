//! This module provides a Semaphore implementation.

use core::cell::UnsafeCell;

use crate::{ThreadState, threadlist::ThreadList};

struct SemaphoreInner {
    current: usize,
    max: usize,
    wait_list: ThreadList,
}

impl SemaphoreInner {
    /// Creates new Semaphore.
    #[must_use]
    const fn new(initial: usize, max: usize) -> Self {
        Self {
            current: initial,
            max,
            wait_list: ThreadList::new(),
        }
    }

    fn current_count(&self) -> usize {
        self.current
    }

    fn try_give(&mut self) -> bool {
        if self.current < self.max {
            self.current += 1;
            true
        } else {
            false
        }
    }

    fn try_take(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            true
        } else {
            false
        }
    }
}

/// A counting semaphore.
pub struct Semaphore {
    inner: UnsafeCell<SemaphoreInner>,
}

unsafe impl Sync for Semaphore {}
unsafe impl Send for Semaphore {}

impl Semaphore {
    /// Creates new Semaphore.
    #[must_use]
    pub const fn new(initial: usize, max: usize) -> Self {
        Self {
            inner: UnsafeCell::new(SemaphoreInner::new(initial, max)),
        }
    }

    /// Return the currently available resources.
    pub fn current_count(&self) -> usize {
        critical_section::with(|_| {
            let inner = unsafe { &*self.inner.get() };
            inner.current_count()
        })
    }

    /// Unlock the semaphore.
    pub fn give(&self) -> bool {
        critical_section::with(|cs| {
            let inner = unsafe { &mut *self.inner.get() };
            // If there is a waiter, wake it up, no need to fiddle with the count.
            // Otherwise, `try_give()` to check if we'd go over the maximum.
            inner.wait_list.pop(cs).is_some() || inner.try_give()
        })
    }

    /// Try getting the semaphore (non-blocking).
    pub fn try_take(&self) -> bool {
        critical_section::with(|_| {
            let inner = unsafe { &mut *self.inner.get() };
            inner.try_take()
        })
    }

    /// Get the semaphore (blocking).
    ///
    /// # Panics
    ///
    /// Panics when called from an interrupt context.
    pub fn take(&self) {
        critical_section::with(|cs| {
            let inner = unsafe { &mut *self.inner.get() };
            if !inner.try_take() {
                inner
                    .wait_list
                    .put_current(cs, ThreadState::SemaphoreBlocked);
            }
        });
    }
}
