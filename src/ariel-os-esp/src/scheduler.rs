//! This module provides the hooks for `esp-radio` to schedule its threads
//! with the Ariel OS scheduler.

#![expect(unsafe_code)]

use alloc::alloc::{Layout, alloc, handle_alloc_error};
use core::{
    ffi::c_void,
    ptr::NonNull,
    sync::atomic::{AtomicPtr, Ordering},
};

use ariel_os_debug::log::{debug, trace};
use ariel_os_threads::{
    CoreAffinity, CoreId, RunqueueId, THREAD_COUNT, ThreadId, create_raw, current_tid,
    get_priority, set_priority, yield_same,
};
use esp_radio_rtos_driver::{SchedulerImplementation, ThreadPtr};

/// The `AtomicPtr`s make sure this static is not promoted to read-only data.
static THREAD_SEMAPHORES: [AtomicPtr<()>; THREAD_COUNT] =
    [const { AtomicPtr::new(core::ptr::null_mut()) }; THREAD_COUNT];

pub(crate) struct ArielScheduler {}

fn thread_id_to_ptr(thread_id: ThreadId) -> ThreadPtr {
    // SAFETY: `thread_id` is guaranteed to be in the range `0..THREAD_COUNT`.
    unsafe { NonNull::new_unchecked((usize::from(thread_id) + 1) as *mut ()) }
}

fn thread_ptr_to_id(thread_ptr: ThreadPtr) -> ThreadId {
    ThreadId::new((usize::from(thread_ptr.addr()) - 1) as u8)
}

/// Allocate an N-byte, M-aligned u8 buffer and leak it as &'static mut [u8]
pub fn alloc_aligned_leaked_buffer(len: usize, align: usize) -> &'static mut [u8] {
    let layout = Layout::from_size_align(len, align).unwrap();

    // Allocate raw memory (uninitialized)
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        handle_alloc_error(layout);
    }

    // Turn the allocation into a slice
    let slice = unsafe { core::slice::from_raw_parts_mut(ptr, len) };

    // Leak by converting to a `'static` reference
    unsafe { &mut *(slice as *mut [u8]) }
}

impl SchedulerImplementation for ArielScheduler {
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

    fn now(&self) -> u64 {
        esp_hal::time::Instant::now()
            .duration_since_epoch()
            .as_micros()
    }

    fn current_task(&self) -> ThreadPtr {
        // NOTE(no-panic): this is always called from within a thread, so the `unwrap()` doesn't
        // panic.
        thread_id_to_ptr(current_tid().unwrap())
    }

    fn task_create(
        &self,
        _name: &str,
        task: extern "C" fn(*mut c_void),
        param: *mut c_void,
        priority: u32,
        pin_to_core: Option<u32>,
        task_stack_size: usize,
    ) -> ThreadPtr {
        trace!("task_create()");

        // upstream uses 16b as minimum alignment on both architectures
        let stack_slice = alloc_aligned_leaked_buffer(task_stack_size, 16);

        let core_affinity = pin_to_core.map(|x| CoreAffinity::one(CoreId::new(x as u8)));

        // SAFETY: *transmuting* between any two function pointers is fine.
        let task =
            unsafe { core::mem::transmute::<extern "C" fn(*mut c_void), extern "Rust" fn()>(task) };

        // SAFETY: Upholding `create_raw()` invariants: We know what we are doing.
        let thread_id = unsafe {
            create_raw(
                task,
                Some(param as usize),
                stack_slice,
                priority as u8,
                core_affinity,
            )
        };

        ariel_os_debug::log::debug!(
            "task_create() name={} thread_id={:?} priority={} stack size={}",
            _name,
            thread_id,
            priority,
            task_stack_size,
        );
        thread_id_to_ptr(thread_id)
    }

    fn schedule_task_deletion(&self, _task_handle: Option<ThreadPtr>) {
        // TODO: not called from `esp-wifi` until the stack is de-initialized,
        // which Ariel currently doesn't do. This is safe but leaks the stack.
        debug!(
            "{}:{} schedule_task_deletion(): leaking stack",
            file!(),
            line!()
        );
    }

    fn current_task_thread_semaphore(&self) -> NonNull<()> {
        use esp_radio_rtos_driver::semaphore::{SemaphoreKind, SemaphorePtr};
        unsafe extern "Rust" {
            fn esp_rtos_semaphore_create(kind: SemaphoreKind) -> SemaphorePtr;
        }

        trace!("{}:{} current_task_thread_semaphore()", file!(), line!());
        // NOTE(no-panic): this is always called from within a thread, so the `unwrap()` doesn't
        // panic.
        let tid = usize::from(current_tid().unwrap());
        critical_section::with(|_cs| {
            let current = THREAD_SEMAPHORES[tid].load(Ordering::Acquire);
            if current == core::ptr::null_mut() {
                let new = unsafe {
                    esp_rtos_semaphore_create(SemaphoreKind::Counting { max: 1, initial: 0 })
                };
                let ptr = new.addr().get() as *mut ();
                THREAD_SEMAPHORES[tid].store(ptr, Ordering::Release);
                new
            } else {
                // Safety:
                // * is not null, checked above
                unsafe { NonNull::new_unchecked(current) }
            }
        })
    }

    unsafe fn task_priority(&self, task: ThreadPtr) -> u32 {
        let thread_id = thread_ptr_to_id(task);
        usize::from(get_priority(thread_id).unwrap()) as u32
    }

    unsafe fn set_task_priority(&self, task: ThreadPtr, priority: u32) {
        let thread_id = thread_ptr_to_id(task);
        trace!(
            "setting {} to priority {}",
            usize::from(thread_id),
            priority
        );
        set_priority(thread_id, RunqueueId::new(priority as u8));
    }

    fn usleep(&self, us: u32) {
        ariel_os_threads::sleep(embassy_time::Duration::from_micros(us as u64));
    }

    fn usleep_until(&self, target: u64) {
        ariel_os_threads::sleep_until(embassy_time::Instant::from_micros(target));
    }
}
