//! This module provides the hooks for `esp-radio` to schedule its threads
//! with the Ariel OS scheduler.

#![expect(unsafe_code)]

use core::{ffi::c_void, ptr::NonNull, sync::atomic::AtomicUsize};

use ariel_os_debug::log::{debug, trace, warn};
use ariel_os_threads::{CoreAffinity, CoreId, THREAD_COUNT, create_raw, current_tid, yield_same};
use esp_radio_rtos_driver::Scheduler;
use esp_wifi_sys::include::malloc;

/// The `AtomicUsize`s make sure this static is not promoted to read-only data.
static THREAD_SEMAPHORES: [AtomicUsize; THREAD_COUNT] =
    [const { AtomicUsize::new(0) }; THREAD_COUNT];

struct ArielScheduler {}

impl Scheduler for ArielScheduler {
    fn yield_task(&self) {
        yield_same();
    }

    fn yield_task_from_isr(&self) {
        yield_same();
    }

    fn initialized(&self) -> bool {
        // In Ariel OS, the async executor initializing WiFi is run in a thread, so this is always
        // true.
        true
    }

    fn max_task_priority(&self) -> u32 {
        const fn usize_to_u32(x: usize) -> u32 {
            if x > u32::MAX as usize {
                panic!("value too large for u32");
            }
            x as u32
        }
        const { usize_to_u32(ariel_os_threads::SCHED_PRIO_LEVELS - 1) }
    }

    fn usleep(&self, us: u32) {
        warn!("not sleeping");
    }

    fn now(&self) -> u64 {
        esp_hal::time::Instant::now()
            .duration_since_epoch()
            .as_micros()
    }

    fn current_task(&self) -> *mut c_void {
        // NOTE(no-panic): this is always called from within a thread, so the `unwrap()` doesn't
        // panic.
        usize::from(current_tid().unwrap()) as *mut c_void
    }

    fn task_create(
        &self,
        _name: &str,
        task: extern "C" fn(*mut c_void),
        param: *mut c_void,
        priority: u32,
        pin_to_core: Option<u32>,
        task_stack_size: usize,
    ) -> *mut c_void {
        trace!("{}:{} task_create()", file!(), line!());
        // SAFETY: might return NULL, we assert it didn't below.
        let stack = unsafe { malloc(task_stack_size as u32) };
        assert!(!stack.is_null());

        // SAFETY: We checked that `stack` has been allocated (is not NULL). `malloc` also aligns
        // properly.
        let stack_slice: &'static mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(stack as *mut u8, task_stack_size as usize) };

        let core_affinity = pin_to_core.map(|x| CoreAffinity::one(CoreId::new(x as u8)));

        // SAFETY: *transmuting* between any two function pointers is fine.
        let task =
            unsafe { core::mem::transmute::<extern "C" fn(*mut c_void), extern "Rust" fn()>(task) };

        // SAFETY: Upholding `create_raw()` invariants: We know what we are doing.
        let tid = unsafe {
            create_raw(
                task,
                Some(param as usize),
                stack_slice,
                priority as u8,
                core_affinity,
            )
        };

        usize::from(tid) as *mut c_void
    }

    fn schedule_task_deletion(&self, _task_handle: *mut c_void) {
        // TODO: not called from `esp-wifi` until the stack is de-initialized,
        // which Ariel currently doesn't do. This is safe but leaks the stack.
        debug!(
            "{}:{} schedule_task_deletion(): leaking stack",
            file!(),
            line!()
        );
    }

    fn current_task_thread_semaphore(&self) -> NonNull<()> {
        trace!("{}:{} current_task_thread_semaphore()", file!(), line!());
        // NOTE(no-panic): this is always called from within a thread, so the `unwrap()` doesn't
        // panic.
        let tid = usize::from(current_tid().unwrap());

        NonNull::new(THREAD_SEMAPHORES[tid].as_ptr() as *mut ()).unwrap()
    }
}

esp_radio_rtos_driver::scheduler_impl!(static SCHEDULER: ArielScheduler = ArielScheduler{});
