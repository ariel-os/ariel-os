use core::ffi::c_void;
use esp_radio_rtos_driver::{
    queue::CompatQueue, register_queue_implementation, register_scheduler_implementation,
    register_semaphore_implementation, register_timer_implementation,
    register_wait_queue_implementation, timer::CompatTimer,
};

use crate::scheduler::ArielScheduler;
use crate::semaphore::CompatSemaphore;
use crate::wait_queue::ArielWaitQueue;

register_scheduler_implementation!(static SCHEDULER: ArielScheduler = ArielScheduler{});
register_wait_queue_implementation!(ArielWaitQueue);
register_semaphore_implementation!(CompatSemaphore);
register_timer_implementation!(CompatTimer);
register_queue_implementation!(CompatQueue);
