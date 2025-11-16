//! Pass non-Send objects around on same executor.
//!
//! This module provides [`SameExecutorCell`], a structure that allows passing around
//! non-Send objects from one async task to another, if they are on the same
//! executor. This is allowed because embassy executors are single threaded.
//!
//! [`SameExecutorCell`] checks for the correct executor *at runtime*.

#![expect(unsafe_code)]

use embassy_executor::Spawner;

// SAFETY:
// SameExecutorCell guarantees at runtime that its content stays on the same embassy
// executor. Those are single threaded, so it is guaranteed that the content
// stays on the same thread.
// While `SameExecutorCell::get()` allows passing any `Spawner` object, those are `!Send`,
// thus they are guaranteed to be for the current Executor.
unsafe impl<T> Send for SameExecutorCell<T> {}

/// A cell that allows sending of non-Send types *if they stay on the same executor*.
///
/// This is *checked at runtime*.
///
/// Both [`new()`](SameExecutorCell::new) and [`get()`](SameExecutorCell::get) have async versions ([`new_async()`](SameExecutorCell::new_async) and [`get_async()`](SameExecutorCell::get_async)) that get a
/// handle for the current [`Spawner`] themselves. They internally call the non-async versions. Use
/// the sync versions if a [`Spawner`] object is available or the async versions cannot be used,
/// e.g., in closures. Otherwise, the async versions are also fine.
#[derive(Debug, Clone)]
pub struct SameExecutorCell<T> {
    executor_id: usize,
    inner: T,
}

impl<T> SameExecutorCell<T> {
    /// Creates a new [`SameExecutorCell`].
    pub fn new(inner: T, spawner: Spawner) -> Self {
        Self {
            executor_id: spawner.executor_id(),
            inner,
        }
    }

    /// Gets the contents of this [`SameExecutorCell`].
    pub fn get(&self, spawner: Spawner) -> Option<&T> {
        if spawner.executor_id() == self.executor_id {
            Some(&self.inner)
        } else {
            None
        }
    }

    /// Gets the contents of this [`SameExecutorCell`] exclusively from an exclusive reference.
    pub fn get_mut(&mut self, spawner: Spawner) -> Option<&mut T> {
        if spawner.executor_id() == self.executor_id {
            Some(&mut self.inner)
        } else {
            None
        }
    }

    /// Creates a new [`SameExecutorCell`] (async version).
    ///
    /// Despite being async, this function never blocks/yields, it returns instantly.
    pub async fn new_async(inner: T) -> Self {
        // SAFETY: safe unless intentionally exploited.
        let spawner = unsafe { Spawner::for_current_executor().await };
        SameExecutorCell::new(inner, spawner)
    }

    /// Gets the contents of this [`SameExecutorCell`] (async version).
    ///
    /// Despite being async, this function never blocks/yields, it returns instantly.
    pub async fn get_async(&self) -> Option<&T> {
        // SAFETY: safe unless intentionally exploited.
        let spawner = unsafe { Spawner::for_current_executor().await };
        self.get(spawner)
    }

    /// Gets the contents of this [`SameExecutorCell`] exclusively from an exclusive reference.
    ///
    /// Despite being async, this function never blocks/yields, it returns instantly.
    pub async fn get_mut_async(&mut self) -> Option<&mut T> {
        // SAFETY: safe unless intentionally exploited.
        let spawner = unsafe { Spawner::for_current_executor().await };
        self.get_mut(spawner)
    }
}
