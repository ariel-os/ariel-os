// taken from upstream esp_radio_rtos_driver, modified to use Ariel's `WaitQueue` directly.
// Source: https://github.com/esp-rs/esp-hal/blob/f5a5a408bf117e116fd16485706477352824aa30/esp-radio-rtos-driver/src/semaphore.rs#L450
// (Apache 2.0/MIT)
//
// This is mostly changing `CompatSemaphore::take_with_deadline()` to use Ariel's
// `wait_until_with_check()`.
// Upstream version was holding a critical section while initiating
// `WaitQueue::with_timeout()`, which doesn't work with Ariel's `WaitQueue` implementation.
// The actual change to `CompatSemaphore::take_with_deadline()` is pretty small. It required moving
// the waitqueue handle (`waiting`) out of `inner` into the main `CompatSemaphore` struct.

// Unfortunately upstream doesn't provide safety comments.
#![expect(unsafe_code)]

use alloc::boxed::Box;
use core::ptr::NonNull;

use ariel_os_debug::log::{debug, trace};
use ariel_os_threads::sync::WaitQueue;
use esp_radio_rtos_driver::{
    ThreadPtr, current_task, now,
    semaphore::{SemaphoreImplementation, SemaphoreKind, SemaphorePtr},
};
use esp_sync::NonReentrantMutex;

// SAFETY: These must match the upstream interface.
unsafe extern "Rust" {
    fn esp_rtos_task_priority(task: ThreadPtr) -> u32;
    fn esp_rtos_set_task_priority(task: ThreadPtr, priority: u32);
}

/// # Safety
///
/// The task pointer must be valid and point to a task that was created using
/// [`task_create`].
#[inline]
unsafe fn task_priority(task: ThreadPtr) -> u32 {
    // Safety: function defined above must match upstream definition.
    unsafe { esp_rtos_task_priority(task) }
}

/// # Safety
///
/// The task pointer must be valid and point to a task that was created using
/// [`task_create`].
#[inline]
unsafe fn set_task_priority(task: ThreadPtr, priority: u32) {
    // Safety: function defined above must match upstream definition.
    unsafe { esp_rtos_set_task_priority(task, priority) }
}

enum SemaphoreInner {
    Counting {
        current: u32,
        max: u32,
    },
    Mutex {
        recursive: bool,
        owner: Option<ThreadPtr>,
        original_priority: u32,
        lock_counter: u32,
    },
}

impl SemaphoreInner {
    fn try_take(&mut self) -> bool {
        match self {
            SemaphoreInner::Counting { current, .. } => {
                if *current > 0 {
                    *current -= 1;
                    true
                } else {
                    false
                }
            }
            SemaphoreInner::Mutex {
                recursive,
                owner,
                lock_counter,
                original_priority,
                ..
            } => {
                let current = current_task();
                if let Some(owner) = *owner {
                    if owner == current && *recursive {
                        *lock_counter += 1;
                        true
                    } else {
                        // We can't lock the mutex. Make sure the mutex holder has a high enough
                        // priority to avoid priority inversion.
                        let current_priority = unsafe { task_priority(current) };
                        let owner_priority = unsafe { task_priority(owner) };
                        if owner_priority < current_priority {
                            unsafe { set_task_priority(owner, current_priority) };
                        }
                        false
                    }
                } else {
                    *owner = Some(current);
                    *lock_counter += 1;
                    *original_priority = unsafe { task_priority(current) };
                    true
                }
            }
        }
    }

    fn try_take_from_isr(&mut self) -> bool {
        match self {
            SemaphoreInner::Counting { current, .. } => {
                if *current > 0 {
                    *current -= 1;
                    true
                } else {
                    false
                }
            }
            SemaphoreInner::Mutex {
                recursive,
                owner,
                lock_counter,
                ..
            } => {
                // In an ISR context we don't have a current task, so we can't implement
                // priority inheritance an we have to conjure up an owner.
                let current = NonNull::dangling();
                if let Some(owner) = owner {
                    if *owner == current && *recursive {
                        *lock_counter += 1;
                        true
                    } else {
                        false
                    }
                } else {
                    *owner = Some(current);
                    *lock_counter += 1;
                    true
                }
            }
        }
    }

    fn try_give(&mut self) -> bool {
        match self {
            SemaphoreInner::Counting { current, max, .. } => {
                if *current < *max {
                    *current += 1;
                    true
                } else {
                    false
                }
            }
            SemaphoreInner::Mutex {
                owner,
                lock_counter,
                original_priority,
                ..
            } => {
                let current = current_task();

                if *owner == Some(current) && *lock_counter > 0 {
                    *lock_counter -= 1;
                    if *lock_counter == 0
                        && let Some(owner) = owner.take()
                    {
                        unsafe { set_task_priority(owner, *original_priority) };
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn try_give_from_isr(&mut self) -> bool {
        match self {
            SemaphoreInner::Counting { current, max, .. } => {
                if *current < *max {
                    *current += 1;
                    true
                } else {
                    false
                }
            }
            SemaphoreInner::Mutex {
                owner,
                lock_counter,
                ..
            } => {
                let current = NonNull::dangling();
                if *owner == Some(current) && *lock_counter > 0 {
                    *lock_counter -= 1;
                    if *lock_counter == 0 {
                        *owner = None;
                    }
                    true
                } else {
                    false
                }
            }
        }
    }

    fn current_count(&mut self) -> u32 {
        match self {
            SemaphoreInner::Counting { current, .. } => *current,
            SemaphoreInner::Mutex { .. } => {
                panic!("RecursiveMutex does not support current_count")
            }
        }
    }
}

/// Semaphore and mutex primitives.
pub struct CompatSemaphore {
    inner: NonReentrantMutex<SemaphoreInner>,
    waiting: WaitQueue,
}

unsafe impl Sync for CompatSemaphore {}

impl CompatSemaphore {
    /// Create a new counting semaphore.
    fn new_counting(initial: u32, max: u32) -> Self {
        CompatSemaphore {
            inner: NonReentrantMutex::new(SemaphoreInner::Counting {
                current: initial,
                max,
            }),
            waiting: WaitQueue::new(),
        }
    }

    /// Create a new mutex.
    ///
    /// If `recursive` is true, the mutex can be locked multiple times by the same task.
    fn new_mutex(recursive: bool) -> Self {
        CompatSemaphore {
            inner: NonReentrantMutex::new(SemaphoreInner::Mutex {
                recursive,
                owner: None,
                lock_counter: 0,
                original_priority: 0,
            }),
            waiting: WaitQueue::new(),
        }
    }

    unsafe fn from_ptr<'a>(ptr: SemaphorePtr) -> &'a Self {
        unsafe { ptr.cast::<Self>().as_ref() }
    }

    /// Try to take the semaphore.
    ///
    /// This is a non-blocking operation. The return value indicates whether the semaphore was
    /// successfully taken.
    fn try_take(&self) -> bool {
        self.inner.with(|sem| sem.try_take())
    }

    /// Try to take the semaphore from an ISR.
    ///
    /// This is a non-blocking operation. The return value indicates whether the semaphore was
    /// successfully taken.
    fn try_take_from_isr(&self) -> bool {
        self.inner.with(|sem| sem.try_take_from_isr())
    }

    /// Take the semaphore.
    ///
    /// This is a blocking operation.
    ///
    /// If the semaphore is already taken, the task will be blocked until the semaphore is
    /// released. Recursive mutexes can be locked multiple times by the mutex owner
    /// task.
    fn take_with_deadline(&self, deadline: Option<u64>) -> bool {
        let deadline = deadline.unwrap_or(u64::MAX);
        let deadline_instant = embassy_time::Instant::from_micros(deadline);
        loop {
            // Ariel OS adaptation here:
            let taken = self.waiting.wait_until_with_check(deadline_instant, |_cs| {
                self.inner.with(|sem| sem.try_take())
            });

            if taken {
                debug!("Semaphore - take - success");
                return true;
            }

            if now() > deadline {
                debug!("Semaphore - take - timed out");
                return false;
            }
        }
    }

    /// Return the current count of the semaphore.
    fn current_count(&self) -> u32 {
        self.inner.with(|sem| sem.current_count())
    }

    /// Unlock the semaphore.
    fn give(&self) -> bool {
        self.inner.with(|sem| {
            if sem.try_give() {
                self.notify();
                true
            } else {
                false
            }
        })
    }

    /// Try to unlock the semaphore from an ISR.
    ///
    /// The return value indicates whether the semaphore was successfully unlocked.
    fn try_give_from_isr(&self, higher_priority_task_waken: Option<&mut bool>) -> bool {
        self.inner.with(|sem| {
            if sem.try_give_from_isr() {
                self.notify_from_isr(higher_priority_task_waken);
                true
            } else {
                false
            }
        })
    }

    fn notify(&self) {
        trace!("Semaphore notify");
        self.waiting.notify_one()
    }

    fn notify_from_isr(&self, _higher_prio_task_waken: Option<&mut bool>) {
        trace!("Semaphore notify from ISR");
        self.waiting.notify_one()
    }
}

impl SemaphoreImplementation for CompatSemaphore {
    fn create(kind: SemaphoreKind) -> SemaphorePtr {
        let sem = Box::new(match kind {
            SemaphoreKind::Counting { max, initial } => Self::new_counting(initial, max),
            SemaphoreKind::Mutex => Self::new_mutex(false),
            SemaphoreKind::RecursiveMutex => Self::new_mutex(true),
        });
        NonNull::from(Box::leak(sem)).cast()
    }

    unsafe fn delete(semaphore: SemaphorePtr) {
        let sem = unsafe { Box::from_raw(semaphore.cast::<Self>().as_ptr()) };
        core::mem::drop(sem);
    }

    unsafe fn take(semaphore: SemaphorePtr, timeout_us: Option<u32>) -> bool {
        unsafe {
            <Self as SemaphoreImplementation>::take_with_deadline(
                semaphore,
                timeout_us.map(|us| now() + us as u64),
            )
        }
    }

    unsafe fn take_with_deadline(semaphore: SemaphorePtr, deadline_instant: Option<u64>) -> bool {
        let semaphore = unsafe { Self::from_ptr(semaphore) };

        semaphore.take_with_deadline(deadline_instant)
    }

    unsafe fn give(semaphore: SemaphorePtr) -> bool {
        let semaphore = unsafe { Self::from_ptr(semaphore) };

        semaphore.give()
    }

    unsafe fn current_count(semaphore: SemaphorePtr) -> u32 {
        let semaphore = unsafe { Self::from_ptr(semaphore) };

        semaphore.current_count()
    }

    unsafe fn try_take(semaphore: SemaphorePtr) -> bool {
        let semaphore = unsafe { Self::from_ptr(semaphore) };

        semaphore.try_take()
    }

    unsafe fn try_give_from_isr(
        semaphore: SemaphorePtr,
        higher_priority_task_waken: Option<&mut bool>,
    ) -> bool {
        let semaphore = unsafe { Self::from_ptr(semaphore) };

        semaphore.try_give_from_isr(higher_priority_task_waken)
    }

    unsafe fn try_take_from_isr(semaphore: SemaphorePtr, _hptw: Option<&mut bool>) -> bool {
        let semaphore = unsafe { Self::from_ptr(semaphore) };

        semaphore.try_take_from_isr()
    }
}
