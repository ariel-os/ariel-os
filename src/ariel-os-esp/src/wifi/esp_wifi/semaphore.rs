use core::ptr::NonNull;

use allocator_api2::boxed::Box;
use ariel_os_threads::sync::{self, Lock, RecursiveLock};

use esp_radio_rtos_driver::{
    register_semaphore_implementation,
    semaphore::{SemaphoreImplementation, SemaphoreKind, SemaphorePtr},
};

enum Semaphore {
    Counting { semaphore: sync::Semaphore },
    Lock { lock: Lock },
    RecursiveLock { recursive_lock: RecursiveLock },
}

impl Semaphore {
    fn new_counting(initial: u32, max: u32) -> Self {
        Self::Counting {
            semaphore: sync::Semaphore::new_counting(initial, max),
        }
    }

    fn new_mutex(recursive: bool) -> Self {
        if recursive {
            Self::RecursiveLock {
                recursive_lock: RecursiveLock::new(),
            }
        } else {
            Self::Lock { lock: Lock::new() }
        }
    }
}

impl SemaphoreImplementation for Semaphore {
    fn create(kind: SemaphoreKind) -> SemaphorePtr {
        let sem = Box::new(match kind {
            SemaphoreKind::Counting { max, initial } => Semaphore::new_counting(initial, max),
            SemaphoreKind::Mutex => Semaphore::new_mutex(false),
            SemaphoreKind::RecursiveMutex => Semaphore::new_mutex(true),
        });
        NonNull::from(Box::leak(sem)).cast()
    }

    unsafe fn delete(semaphore: SemaphorePtr) {
        let sem = unsafe { Box::from_raw(semaphore.cast::<Semaphore>().as_ptr()) };
        core::mem::drop(sem);
    }

    unsafe fn take(semaphore: SemaphorePtr, timeout_us: Option<u32>) -> bool {
        let semaphore = unsafe { Semaphore::from_ptr(semaphore) };

        semaphore.take(timeout_us)
    }

    unsafe fn give(semaphore: SemaphorePtr) -> bool {
        let semaphore = unsafe { Semaphore::from_ptr(semaphore) };

        semaphore.give()
    }

    unsafe fn current_count(semaphore: SemaphorePtr) -> u32 {
        let semaphore = unsafe { Semaphore::from_ptr(semaphore) };

        semaphore.current_count()
    }

    unsafe fn try_take(semaphore: SemaphorePtr) -> bool {
        let semaphore = unsafe { Semaphore::from_ptr(semaphore) };

        semaphore.try_take()
    }

    unsafe fn try_give_from_isr(semaphore: SemaphorePtr, _hptw: Option<&mut bool>) -> bool {
        let semaphore = unsafe { Semaphore::from_ptr(semaphore) };

        semaphore.try_give_from_isr()
    }

    unsafe fn try_take_from_isr(semaphore: SemaphorePtr, _hptw: Option<&mut bool>) -> bool {
        let semaphore = unsafe { Semaphore::from_ptr(semaphore) };

        semaphore.try_take_from_isr()
    }
}

register_semaphore_implementation!(Semaphore);
