//! This module contains a custom [`Signal`] struct meant to be used in the [`ariel-os-sensors`][ariel-os-sensors] ecosystem

use core::{
    cell::Cell,
    pin::Pin,
    task::{Context, Poll, Waker},
};

use embassy_sync::blocking_mutex::{Mutex, raw::CriticalSectionRawMutex};

#[derive(Debug, Default)]
enum SignalState<T> {
    #[default]
    None,
    Waiting(Waker),
    Ready(T),
}

/// Custom signal struct inspired by [`embassy_sync::signal::Signal`] and [`embassy_sync::channel::Channel`]
///
/// This is meant for single-producer and single-consumer signaling.
///
/// This struct has been created for the [`ariel-os-sensors`][ariel-os-sensors] ecosystem.
///
/// [ariel-os-sensors]: crate
// This struct exists for multiple reasons:
// - Get a lightweight [`Future`] that can be easily stored in a struct like [`ReadingWaiter`][crate::sensor::ReadingWaiter]
// - Keep a stable API that doesn't change with [`embassy_sync`] versions
pub struct Signal<T> {
    inner: Mutex<CriticalSectionRawMutex, Cell<SignalState<T>>>,
}

impl<T> Default for Signal<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Signal<T> {
    /// Create a new empty [`Signal`]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(Cell::new(SignalState::<T>::None)),
        }
    }

    /// Signal that a new value is available and will replace the previous value if it wasn't read.
    pub fn signal(&self, new: T) {
        self.inner.lock(|cell| {
            let state = cell.take();
            match state {
                SignalState::None => {
                    cell.set(SignalState::Ready(new));
                }
                SignalState::Ready(_prev) => {
                    cell.set(SignalState::Ready(new));
                }
                SignalState::Waiting(read_waker) => {
                    cell.set(SignalState::Ready(new));
                    read_waker.wake();
                }
            }
        });
    }

    /// Returns a future that will return once a value is available.
    ///
    /// This is not meant to have multiple tasks waiting for a signal. If multiple tasks are waiting
    /// then a signal sent with [`Self::signal`] will reach only one task at random.
    pub fn wait(&'static self) -> ReceiveFuture<'static, T> {
        ReceiveFuture { signaling: self }
    }

    fn poll_wait(&self, cx: &mut Context<'_>) -> Poll<T> {
        self.inner.lock(|cell| {
            let state = cell.take();
            match state {
                SignalState::None => {
                    cell.set(SignalState::Waiting(cx.waker().clone()));
                    Poll::Pending
                }

                // Multiple tasks waiting for a reading, this shouldn't happen
                SignalState::Waiting(prev_waker) => {
                    if prev_waker.will_wake(cx.waker()) {
                        cell.set(SignalState::Waiting(prev_waker));
                    } else {
                        cell.set(SignalState::Waiting(cx.waker().clone()));

                        // We can't store multiple wakers, they will fight eachother until some data
                        // is sent.
                        // This should happen only if multiple tasks are waiting for a measurement.
                        prev_waker.wake();
                    }
                    Poll::Pending
                }
                SignalState::Ready(res) => Poll::Ready(res),
            }
        })
    }
}

/// A future that will resolve once a signal is sent.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct ReceiveFuture<'ch, T> {
    signaling: &'ch Signal<T>,
}

impl<T> Future for ReceiveFuture<'_, T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        self.signaling.poll_wait(cx)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use static_cell::StaticCell;

    #[test]
    fn future_returns() {
        static SIGNAL: StaticCell<Signal<u8>> = StaticCell::new();
        let signal = SIGNAL.init(Signal::new());
        let future = signal.wait();

        let wanted = 42u8;

        embassy_futures::block_on(async {
            embassy_futures::join::join(
                async {
                    signal.signal(wanted);
                },
                async {
                    assert_eq!(future.await, wanted);
                },
            )
            .await;
        });
    }

    #[test]
    fn manual_poll() {
        static SIGNAL: StaticCell<Signal<u8>> = StaticCell::new();
        let signal = &*SIGNAL.init(Signal::new());

        let mut receive_future = signal.wait();
        let wanted = 31;

        // arbitrary amount of polling, should always return Poll::Pending
        assert_eq!(
            embassy_futures::poll_once(&mut receive_future),
            Poll::Pending
        );
        assert_eq!(
            embassy_futures::poll_once(&mut receive_future),
            Poll::Pending
        );

        signal.signal(wanted);

        assert_eq!(
            embassy_futures::poll_once(receive_future),
            Poll::Ready(wanted)
        );
    }

    #[test]
    fn override_value() {
        static SIGNAL: StaticCell<Signal<u8>> = StaticCell::new();
        let signal = &*SIGNAL.init(Signal::new());
        let future = signal.wait();
        let wanted = 42u8;

        signal.signal(2);
        signal.signal(wanted);

        assert_eq!(embassy_futures::block_on(future), wanted);
    }
}
