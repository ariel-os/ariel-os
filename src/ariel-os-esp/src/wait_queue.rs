#![expect(unsafe_code)]

use alloc::boxed::Box;
use core::ptr::NonNull;

use ariel_os_threads::sync::WaitQueue;
use esp_radio_rtos_driver::wait_queue::{WaitQueueImplementation, WaitQueuePtr};

pub(crate) struct ArielWaitQueue {
    inner: WaitQueue,
}

impl ArielWaitQueue {
    fn new() -> Self {
        Self {
            inner: WaitQueue::new(),
        }
    }
    fn from_ptr(ptr: WaitQueuePtr) -> &'static mut ArielWaitQueue {
        unsafe { &mut *(ptr.cast::<ArielWaitQueue>().as_ptr()) }
    }
}

impl WaitQueueImplementation for ArielWaitQueue {
    fn create() -> WaitQueuePtr {
        let q = Box::new(ArielWaitQueue::new());
        NonNull::from(Box::leak(q)).cast()
    }

    // Safety: caller must ensure not deleting an in-use wait queue
    unsafe fn delete(queue: WaitQueuePtr) {
        let q = unsafe { Box::from_raw(queue.cast::<ArielWaitQueue>().as_ptr()) };
        core::mem::drop(q);
    }

    unsafe fn wait_until(queue: WaitQueuePtr, deadline_instant: Option<u64>) {
        let queue = ArielWaitQueue::from_ptr(queue);
        if let Some(micros) = deadline_instant {
            queue
                .inner
                .wait_until(embassy_time::Instant::from_micros(micros));
        } else {
            queue.inner.wait();
        }
    }

    unsafe fn notify(queue: WaitQueuePtr) {
        let queue = ArielWaitQueue::from_ptr(queue);
        queue.inner.notify_one();
    }

    unsafe fn notify_from_isr(queue: WaitQueuePtr, _higher_prio_task_waken: Option<&mut bool>) {
        let queue = ArielWaitQueue::from_ptr(queue);
        queue.inner.notify_one();
    }
}
