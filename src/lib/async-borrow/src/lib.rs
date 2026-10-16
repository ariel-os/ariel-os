//! This crate is temporary in that we a concrete tool to generalize Ariel OS's
//! [`Delegate`](https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/delegate/struct.Delegate.html)
//! -- once it is a bit more mature, it will be proposed for inclusion in embassy-sync.
//!
//! Compared to Delegate, it should be easy to use (no unsafe for callers). It is implicitly is
//! usable multiple times because of its fixed roles around the signal, but doesn't use the
//! same-executor restriction because it asks `T` to be Send.
//!
//! # To be explored
//!
//! * Can this be shrunk in terms of overhead, eg. by replacing the two signals with a single
//!   signal and a mutex'd (or atomic) "who is just waiting" indicator?
//! * Can we generalize from &mut T to other smart pointers? The type would need to implement a
//!   custom trait for the creation of the lifetime; that should be doable. In particular, this
//!   could be used for the same-executor functionality of Delegate.
//! * Rather than having split on 'static, can we have a lifetime there? It'd make it a bit
//!   easier to construct when both sides have a common caller.
//! * If we used queues, could we have multiple lenders? (Still single borrower that works
//!   items off one by one; how do lenders know to only wait for *their* return signal and not
//!   listen too early?) And: do we need that?
//! * Is there an easier way to distribute the lender and borrower as statics, for cases when they
//!   don't have a chain from a single caller throuogh which they can pass around their parts?
//!   There will always be the temptation to just do an unsafe "I promise I take this and that part
//!   out only once" dance.
#![expect(
    unsafe_code,
    reason = "lifetimes are used across boundaries that are not transparent to Rust"
)]

use core::ops::DerefMut;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

/// A [`Send`] wrapper around raw pointers used in [`AsyncBorrowExchange`]
struct Wrapped<T: Send>(*mut T);

// SAFETY: Only constructed for `AsyncBorrowExchange`, which synchronizes between tasks no matter
// which core/thread they are on.
unsafe impl<T: Send> Send for Wrapped<T> {}

/// Introduction point between two asynchronous components that allows sending a `&mut T` to
/// another task.
///
/// Note that `T` needs to be [`Send`] (as it is moved to another thread), and also `'static`,
/// for while we can come up with a short lifetime in the other thread to become a `&'short mut
/// T`, allowing the type itself to be shorter-lived would a) not be expressible in the other
/// thread, and b) even if we could, we'd only invent a lifetime for within the duration, and
/// would then risk having lifetimed data moving in.
///
/// ## Usage
///
/// Create a [`AsyncBorrowExchange`] for your type, split it, and distribute the lender and the
/// borrower to where they are needed.
///
/// As the split function requires a `&'static mut`, the instance's creation happens most
/// easily in a `static_cell`.
///
/// If both sides are callees of a single function, the sides can just be sent around.
/// Otherwise, one side can be sent to the receiving site eg. using a
/// `OnceCell<Mutex<Lender>>`.
// We could also do with less than 'static if the BorrowArossTasks itself is not static, but
// realistically, using it statically in a static_cell or a global mutex is the practical way.
pub struct AsyncBorrowExchange<T: 'static + Send> {
    // If you think about whether we can get away with a single Signal, think about what
    // happens during lending, and how the lender must immediately join the returning queue and
    // might thus steal the signal from the already waiting borrower.
    sharing: Signal<CriticalSectionRawMutex, Wrapped<T>>,
    returning: Signal<CriticalSectionRawMutex, ()>,
    // FIXME: Should we be generic over the raw mutex in both?
}

pub struct Lender<T: 'static + Send>(&'static AsyncBorrowExchange<T>);
pub struct Borrower<T: 'static + Send>(&'static AsyncBorrowExchange<T>);

impl<T: 'static + Send> AsyncBorrowExchange<T> {
    pub const fn new() -> Self {
        Self {
            sharing: Signal::new(),
            returning: Signal::new(),
        }
    }

    pub const fn split(&'static mut self) -> (Lender<T>, Borrower<T>) {
        let staticref = &*self;
        (Lender(staticref), Borrower(staticref))
    }
}

impl<T: 'static + Send> Lender<T> {
    pub async fn lend(&mut self, data: &mut T) {
        struct DropGuard;

        impl Drop for DropGuard {
            fn drop(&mut self) {
                panic!(
                    "Task that lends data must not be dropped, panicking in drop handler to avoid use-after-free."
                )
            }
        }

        let guard = DropGuard;

        let wrapped = Wrapped(data as *mut _);

        self.0.sharing.signal(wrapped);
        self.0.returning.wait().await;

        core::mem::forget(guard);
    }
}

impl<T: 'static + Send> Borrower<T> {
    pub async fn borrow(&mut self) -> impl DerefMut<Target = T> + use<'_, T> {
        // Using T as Wrapped<T> rather than the inner *mut T because why not send it on.
        struct Borrowed<'a, T: 'static + Send>(&'a Signal<CriticalSectionRawMutex, ()>, Wrapped<T>);

        impl<'a, T: 'static + Send> core::ops::Deref for Borrowed<'a, T> {
            type Target = T;

            fn deref(&self) -> &T {
                unsafe { &*self.1.0 }
            }
        }

        impl<'a, T: 'static + Send> core::ops::DerefMut for Borrowed<'a, T> {
            fn deref_mut(&mut self) -> &mut T {
                unsafe { &mut *self.1.0 }
            }
        }

        impl<'a, T: 'static + Send> Drop for Borrowed<'a, T> {
            fn drop(&mut self) {
                self.0.signal(())
            }
        }

        Borrowed(&self.0.returning, self.0.sharing.wait().await)
    }
}
