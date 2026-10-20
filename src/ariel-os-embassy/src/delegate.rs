//! Delegate or lend an object to another task.
//!
//! This is just a thin shim around asynchronous borrows (to keep the previous delegate API for
//! user-facing parts).
//!
//! # Alternatives
//!
//! If we could, at build time, set up a place for the Lender to already have a pointer to the
//! AsyncBorrowExchange, that could be wrapped in an AtomicRefCell<Option<_>>, and we could do
//! without so many wakers. But that would require that we create *and split* an exchange already
//! at build time, but that'd need unsafe or tricks like `cortex_m_rt`'s entry static muts for
//! static muts (i.e., a dedicated tool that shows how some ways of getting static mut are sound).
//!
//! … or async_borrow could ship multi-borrower tools right away. (But do they need deterministic
//! sequence? And the lender would still need to know how often to lend it out, for one task might
//! just be slow). `maitake-sync` would come in handy.
//!
//! But right now, the goal is to show that async_borrow can fundamentally replace delegations with
//! very narrow use of unsafe.

/// [`Delegate`]s or lends an object to another task.
///
/// This struct can be used to borrow a `&mut T` during startup across tasks for the duration of a
/// closure. After that, the `&mut T` is returned to its original place, which is an
/// [`async_borrow::Lender`].
///
/// The `Delegate` object performs three roles beyond being an [`async_borrow::Borrower`]:
/// * It wraps the delegated object into a [`SameExecutorCell`] (to support non-[`Send`] types,
///   which it can in practice due to all autostarted tasks being spawned on the same executor)
/// * It provides the `(&'static _).with()` API that has been in use in previous implementations.
/// * It caters the lack of introduction point and certain sequencing between the parts of
///   [`async_borrow`] exchange: As the current autostart APIs have no way to send the split parts
///   of a single static instance into the tasks, one side has to create the static, and the other
///   side has to wait (in our case, for the [`async_borrow::Lender`]) to be sent around at
///   runtime. See module-level documentation on how this might change.
///
/// This instance only caters for the borrower side. The lender side merely needs to `wait()` for
/// the signal that is set up at construction time, and then use the `async_borrow::Lender` APIs to
/// lend an item once.
pub struct Delegate<T: 'static>(
    static_cell::ConstStaticCell<
        async_borrow::AsyncBorrowExchange<crate::cell::SameExecutorCell<T>>,
    >,
    &'static embassy_sync::signal::Signal<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        async_borrow::Lender<crate::cell::SameExecutorCell<T>>,
    >,
);

impl<T: 'static> Delegate<T> {
    pub const fn new(
        lender_signal: &'static embassy_sync::signal::Signal<
            embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
            async_borrow::Lender<crate::cell::SameExecutorCell<T>>,
        >,
    ) -> Self {
        Self(
            static_cell::ConstStaticCell::new(async_borrow::AsyncBorrowExchange::new()),
            lender_signal,
        )
    }

    /// Calls a closure on a lent object.
    ///
    /// This blocks until [`Delegate::lend()`] is called on the instance.
    ///
    /// # Panics
    ///
    /// Panics if called multiple times on the same [`Delegate`] instance.
    ///
    /// Panics if called from a different executor than the data is sent from.
    pub async fn with<U>(&'static self, func: impl FnOnce(&mut T) -> U) -> U {
        let exchange = self.0.take();
        let (lender, mut borrower) = exchange.split();
        self.1.signal(lender);
        let mut borrowed = borrower.borrow().await;
        let borrowed_in_this_task = borrowed.get_mut_async().await.unwrap();
        func(borrowed_in_this_task)
    }
}
