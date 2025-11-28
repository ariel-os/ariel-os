//! This module provides a recursive lock implementation.

use core::cell::UnsafeCell;

use critical_section::CriticalSection;

use crate::{ThreadId, ThreadState, current_tid, threadlist::ThreadList};

struct RecursiveLockInner {
    owner: Option<ThreadId>,
    count: usize,
    wait_list: ThreadList,
}

impl RecursiveLockInner {
    /// Creates new `RecursiveLockInner`.
    #[must_use]
    const fn new() -> Self {
        Self {
            owner: None,
            count: 0,
            wait_list: ThreadList::new(),
        }
    }

    fn current_count(&self) -> usize {
        self.count
    }

    /// note: must be called from thread context
    /// # Panics
    /// Panics when called from an interrupt context.
    fn try_acquire(&mut self) -> bool {
        let current_thread_id = current_tid().unwrap();
        if self.count > 0 {
            if self.owner == Some(current_thread_id) {
                self.count += 1;
                true
            } else {
                false
            }
        } else {
            self.owner = Some(current_thread_id);
            self.count += 1;
            true
        }
    }

    /// Try releasing this lock
    /// # Panics
    /// Panics if called from ISR context
    fn try_release(&mut self, cs: CriticalSection<'_>) -> bool {
        if self.count > 0 {
            if self.owner == Some(current_tid().unwrap()) {
                if self.count == 1 {
                    if let Some((thread_id, _)) = self.wait_list.pop(cs) {
                        self.owner = Some(thread_id);
                        return true;
                    }
                }
                self.count -= 1;
                if self.count == 0 {
                    self.owner = None;
                    return true;
                }
            }
            false
        } else {
            true
        }
    }
}

/// A counting semaphore.
pub struct RecursiveLock {
    inner: UnsafeCell<RecursiveLockInner>,
}

unsafe impl Sync for RecursiveLock {}
unsafe impl Send for RecursiveLock {}

impl RecursiveLock {
    /// Creates new `RecursiveLock`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: UnsafeCell::new(RecursiveLockInner::new()),
        }
    }

    /// Release the lock.
    pub fn release(&self) -> bool {
        critical_section::with(|cs| {
            let inner = unsafe { &mut *self.inner.get() };
            inner.try_release(cs)
        })
    }

    /// Try getting the lock (non-blocking).
    pub fn try_acquire(&self) -> bool {
        critical_section::with(|_| {
            let inner = unsafe { &mut *self.inner.get() };
            inner.try_acquire()
        })
    }

    /// Get the lock (blocking).
    ///
    /// # Panics
    ///
    /// Panics when called from an interrupt context.
    pub fn acquire(&self) {
        critical_section::with(|cs| {
            let inner = unsafe { &mut *self.inner.get() };
            if !inner.try_acquire() {
                inner
                    .wait_list
                    .put_current(cs, ThreadState::RecursiveLockBlocked);
            }
        });
    }
}

impl Default for RecursiveLock {
    fn default() -> Self {
        Self::new()
    }
}
