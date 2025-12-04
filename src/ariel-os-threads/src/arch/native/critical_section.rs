//! critical_section implementation for std & infinicore
//!
//! In Ariel OS, most IPC code expects a context switch that is triggered within a
//! critical section to be deferred until the critical section ends, and then run
//! in an ISR.
//! This means a thread might get scheduled away right after a critical section,
//! maybe because it is not runnable anymore (but waiting for a lock, message or
//! flag).
//!
//! On native with each Ariel thread running in its own host kernel thread, we cannot preempt a
//! running thread, but that is also not necessary, because any higher-priority Ariel thread
//! runs in its own system thread.
//! It is possible though that a thread needs to pause itself, e.g., when going
//! blocked on a lock, message or flag. On our MCU platforms, this is handled by
//! pending an ISR that gets executed right after the end of the critical section.
//! The ISR might then schedule another thread, and when a thread is scheduled again,
//! it will continue right at the end of the critical section.
//!
//!
//! So for native, we need to emulate this behavior:
//! 1. We keep an `AtomicLock` per thread (in `THREAD_RUNNABLE`) instead of the regular runqueue for
//!    keeping track whether a thread is runnable-
//! 2. The scheduler locks/unlocks those `AtomicLock`s to specify whether a thread is runnable.
//! 3. We wrap the regular `StdCriticalSection` in `ArielCriticalSection`
//! 4. In the wrapped critical section, on `release()`, after the wrapped critical section ends, we
//!    block the thread on its `AtomicLock`.

#![expect(unsafe_code)]

mod upstream;
use upstream::StdCriticalSection;

critical_section::set_impl!(ArielCriticalSection);

struct ArielCriticalSection;

// SAFETY:
// This is delegating safety to the upstream implementation, see
// https://github.com/rust-embedded/critical-section/blob/cebd3d76cc5237d77abd0e03c090577fad1c689f/src/lib.rs#L178-L194
// and `critical_section/upstream.rs`.
unsafe impl critical_section::Impl for ArielCriticalSection {
    // SAFETY:
    // This is delegating safety to the upstream implementation.
    unsafe fn acquire() -> bool {
        // SAFETY: delegating safety to upstream implementation
        unsafe { StdCriticalSection::acquire() }
    }

    // SAFETY:
    // This is delegating safety to the upstream implementation.
    // The added `wait()` keeps the upstream contract.
    unsafe fn release(nested_cs: bool) {
        if !nested_cs {
            let thread_id = crate::ThreadData::ID.get();
            // SAFETY: delegating safety to upstream implementation
            unsafe { StdCriticalSection::release(false) }
            // The critical section technically ends here.
            // For infinicore, we now block until the thread is runnable again,
            // indicated by the threads entry in `THREAD_RUNNABLE`.
            if let Some(thread_id) = thread_id {
                atomic_wait::wait(&super::THREAD_RUNNABLE[usize::from(thread_id)], 0);
            }
        }
    }
}
