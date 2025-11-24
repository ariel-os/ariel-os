use core::task::Waker;

use ariel_os_debug::log::trace;
use esp_hal::{
    Blocking,
    interrupt::{InterruptHandler, Priority},
    time::Duration,
    timer::{OneShotTimer, any::Degrade},
};
use esp_sync::NonReentrantMutex;

use crate::TIMER_QUEUE;

static TIMER_DRIVER: NonReentrantMutex<Option<TimeDriver>> = NonReentrantMutex::new(None);

type TimeBase = OneShotTimer<'static, Blocking>;

pub(crate) struct TimeDriver {
    timer: TimeBase,
    current_alarm: u64,
}
use esp_hal::timer::systimer::Alarm;
pub(crate) fn init(timer: Alarm<'static>) {
    let timer = TimeDriver::new(TimeBase::new(timer.degrade()));
    TIMER_DRIVER.with(|timer_driver| {
        *timer_driver = Some(timer);
    });
}

impl TimeDriver {
    pub(crate) fn new(mut timer: TimeBase) -> Self {
        // The timer needs to tick at Priority 1 to prevent accidentally interrupting
        // priority limited locks.
        let timer_priority = Priority::Priority1;

        let cb: extern "C" fn() = unsafe { core::mem::transmute(timer_handler as *const ()) };

        cfg_if::cfg_if! {
            if #[cfg(riscv)] {
                // Register the interrupt handler without nesting to satisfy the requirements of the
                // task switching code
                let handler = InterruptHandler::new_not_nested(cb, timer_priority);
            } else {
                let handler = InterruptHandler::new(cb, timer_priority);
            }
        };

        timer.set_interrupt_handler(handler);
        timer.listen();

        Self {
            timer,
            current_alarm: u64::MAX,
        }
    }

    pub(crate) fn arm_next_wakeup(&mut self, next_wakeup: u64) {
        // Only skip arming timer if the timestamp is the same. If the next wakeup changed to a
        // later timestamp, the tick handler may not trigger a scheduler run. This means that if we
        // did not arm here, the timer would not be re-armed.
        if next_wakeup == self.current_alarm {
            return;
        }

        self.current_alarm = next_wakeup;

        let sleep_duration = next_wakeup.saturating_sub(now());

        // assume 52-bit underlying timer. it's not a big deal to sleep for a shorter time
        let mut timeout = sleep_duration & ((1 << 52) - 1);

        trace!("Arming timer for {} (target = {})", timeout, next_wakeup);
        loop {
            match self.timer.schedule(Duration::from_micros(timeout)) {
                Ok(_) => break,
                Err(esp_hal::timer::Error::InvalidTimeout) if timeout != 0 => {
                    timeout /= 2;
                    continue;
                }
                Err(e) => panic!("Failed to schedule timer: {:?}", e),
            }
        }
    }
}

#[esp_hal::ram]
extern "C" fn timer_handler() {
    trace!("Timer tick");

    let now = now();

    TIMER_QUEUE.handle_alarm(now);

    TIMER_DRIVER.with(|timer| {
        if let Some(timer) = timer {
            timer.timer.clear_interrupt();
        }
    });
}

pub(crate) fn now() -> u64 {
    esp_hal::time::Instant::now()
        .duration_since_epoch()
        .as_micros()
}

pub(super) struct TimerQueueInner {
    queue: embassy_time_queue_utils::Queue,
    pub next_wakeup: u64,
}

impl TimerQueueInner {
    const fn new() -> Self {
        Self {
            queue: embassy_time_queue_utils::Queue::new(),
            next_wakeup: u64::MAX,
        }
    }

    pub(crate) fn handle_alarm(&mut self, now: u64) {
        if now >= self.next_wakeup {
            self.next_wakeup = self.queue.next_expiration(now);
        }
    }

    fn schedule_wake(&mut self, at: u64, waker: &Waker) -> bool {
        if self.queue.schedule_wake(at, waker) {
            self.next_wakeup = self.next_wakeup.min(at);
            true
        } else {
            false
        }
    }
}

pub(crate) struct TimerQueue {
    inner: NonReentrantMutex<TimerQueueInner>,
}

impl TimerQueue {
    pub(crate) const fn new() -> Self {
        Self {
            inner: NonReentrantMutex::new(TimerQueueInner::new()),
        }
    }

    pub(crate) fn handle_alarm(&self, now: u64) {
        self.inner.with(|inner| {
            inner.handle_alarm(now);
        });
    }
}

impl embassy_time_driver::Driver for TimerQueue {
    #[inline]
    fn now(&self) -> u64 {
        now()
    }

    #[inline]
    fn schedule_wake(&self, at: u64, waker: &Waker) {
        if self.inner.with(|inner| inner.schedule_wake(at, waker)) {
            ariel_os_debug::log::info!("schedule wake at {}", at);
            TIMER_DRIVER.with(|timer_driver| {
                if let Some(driver) = timer_driver.as_mut() {
                    driver.arm_next_wakeup(at);
                }
            });
        }
    }
}
