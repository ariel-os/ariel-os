/// This module provides the hooks for `esp-wifi` to schedule its threads
/// with the Ariel OS scheduler.
use core::ffi::c_void;

use ariel_os_debug::log::{trace, warn};
use ariel_os_threads::{create_raw, current_tid, yield_same, THREAD_COUNT};
use esp_wifi::{preempt::Scheduler, TimeBase};
use esp_wifi_sys::include::malloc;

static THREAD_SEMAPHORES: [usize; THREAD_COUNT] = [0; THREAD_COUNT];

struct ArielScheduler {}

impl Scheduler for ArielScheduler {
    fn setup(&self, _timer: TimeBase) {
        trace!("{}:{} setup()", file!(), line!());
    }

    fn disable(&self) {
        trace!("{}:{} disable()", file!(), line!());
    }

    fn yield_task(&self) {
        yield_same();
    }

    fn current_task(&self) -> *mut c_void {
        // NOTE(no-panic): this is always called from within a thread, so the `unwrap()` doesn't
        // panic.
        usize::from(current_tid().unwrap()) as *mut c_void
    }

    fn task_create(
        &self,
        task: extern "C" fn(*mut c_void),
        param: *mut c_void,
        task_stack_size: usize,
    ) -> *mut c_void {
        trace!("{}:{} task_create()", file!(), line!());
        let stack = unsafe { malloc(task_stack_size as u32) };
        let stack_slice: &'static mut [u8] =
            unsafe { core::slice::from_raw_parts_mut(stack as *mut u8, task_stack_size as usize) };

        let prio = 8; // same as ariel executor thread
        let core_affinity = None;
        // # Safety
        // We know what we are doing.
        let tid = unsafe {
            create_raw(
                task as usize,
                param as usize,
                stack_slice,
                prio,
                core_affinity,
            )
        };
        usize::from(tid) as *const usize as *mut c_void
    }

    fn schedule_task_deletion(&self, _task_handle: *mut c_void) {
        // TODO: not called from `esp-wifi` until the stack is de-initialized,
        // which Ariel currently doesn't do. This is safe but leaks the stack.
        warn!(
            "{}:{} schedule_task_deletion(): probably leaking stack",
            file!(),
            line!()
        );
    }

    fn current_task_thread_semaphore(&self) -> *mut c_void {
        trace!("{}:{} current_task_thread_semaphore()", file!(), line!());
        // NOTE(no-panic): this is always called from within a thread, so the `unwrap()` doesn't
        // panic.
        let tid = usize::from(current_tid().unwrap());
        &THREAD_SEMAPHORES[tid] as *const usize as *mut c_void
    }
}

esp_wifi::scheduler_impl!(static SCHEDULER: ArielScheduler = ArielScheduler{});
