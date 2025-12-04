//! This file is a verbatim copy of the [upstream code](https://github.com/rust-embedded/critical-section/blob/cebd3d76cc5237d77abd0e03c090577fad1c689f/src/std.rs), with minor changes:
//! 1. Make StdCriticalSection `pub(crate)`.
//! 2. Don't register `StdCriticalSection` as the global impl.
//! 3. Allow static mut refs.
//! 4. Add unsafes around the static mut accesses.

#![expect(unsafe_code)]

use std::cell::Cell;
use std::mem::MaybeUninit;
use std::sync::{Mutex, MutexGuard};

static GLOBAL_MUTEX: Mutex<()> = Mutex::new(());

// This is initialized if a thread has acquired the CS, uninitialized otherwise.
static mut GLOBAL_GUARD: MaybeUninit<MutexGuard<'static, ()>> = MaybeUninit::uninit();

std::thread_local!(static IS_LOCKED: Cell<bool> = Cell::new(false));

pub(crate) struct StdCriticalSection;
// Ariel OS change: don't set this as global impl
//crate::set_impl!(StdCriticalSection);

unsafe impl critical_section::Impl for StdCriticalSection {
    unsafe fn acquire() -> bool {
        // Allow reentrancy by checking thread local state
        IS_LOCKED.with(|l| {
            if l.get() {
                // CS already acquired in the current thread.
                return true;
            }

            // Note: it is fine to set this flag *before* acquiring the mutex because it's thread local.
            // No other thread can see its value, there's no potential for races.
            // This way, we hold the mutex for slightly less time.
            l.set(true);

            // Not acquired in the current thread, acquire it.
            let guard = match GLOBAL_MUTEX.lock() {
                Ok(guard) => guard,
                Err(err) => {
                    // Ignore poison on the global mutex in case a panic occurred
                    // while the mutex was held.
                    err.into_inner()
                }
            };
            #[allow(static_mut_refs)]
            unsafe {
                GLOBAL_GUARD.write(guard)
            };

            false
        })
    }

    unsafe fn release(nested_cs: bool) {
        if !nested_cs {
            // SAFETY: As per the acquire/release safety contract, release can only be called
            // if the critical section is acquired in the current thread,
            // in which case we know the GLOBAL_GUARD is initialized.
            //
            // We have to `assume_init_read` then drop instead of `assume_init_drop` because:
            // - drop requires exclusive access (&mut) to the contents
            // - mutex guard drop first unlocks the mutex, then returns. In between those, there's a brief
            //   moment where the mutex is unlocked but a `&mut` to the contents exists.
            // - During this moment, another thread can go and use GLOBAL_GUARD, causing `&mut` aliasing.
            #[allow(static_mut_refs)]
            #[allow(let_underscore_lock)]
            let _ = unsafe { GLOBAL_GUARD.assume_init_read() };

            // Note: it is fine to clear this flag *after* releasing the mutex because it's thread local.
            // No other thread can see its value, there's no potential for races.
            // This way, we hold the mutex for slightly less time.
            IS_LOCKED.with(|l| l.set(false));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use crate as critical_section;

    #[cfg(feature = "std")]
    #[test]
    #[should_panic(expected = "Not a PoisonError!")]
    fn reusable_after_panic() {
        let _ = thread::spawn(|| {
            critical_section::with(|_| {
                panic!("Boom!");
            })
        })
        .join();

        critical_section::with(|_| {
            panic!("Not a PoisonError!");
        })
    }
}
