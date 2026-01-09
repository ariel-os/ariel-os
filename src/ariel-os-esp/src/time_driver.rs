use core::task::Waker;

use esp_hal::{
    Blocking,
    interrupt::{InterruptHandler, Priority},
    time::Duration,
    timer::{OneShotTimer, any::Degrade as _},
};
use esp_sync::NonReentrantMutex;

use ariel_os_debug::log::trace;

use crate::TIMER_QUEUE;

static TIMER_DRIVER: NonReentrantMutex<Option<TimeDriver>> = NonReentrantMutex::new(None);

type TimeBase = OneShotTimer<'static, Blocking>;

pub(crate) struct TimeDriver {
    timer: TimeBase,
}

#[cfg(not(context = "esp32"))]
use esp_hal::timer::systimer::Alarm as Timer;

#[cfg(context = "esp32")]
use esp_hal::timer::timg::Timer;

pub(crate) fn init(timer: Timer<'static>) {
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

        cfg_if::cfg_if! {
            if #[cfg(riscv)] {
                // Register the interrupt handler without nesting to satisfy the requirements of the
                // task switching code
                let handler = InterruptHandler::new_not_nested(timer_handler, timer_priority);
            } else {
                let handler = InterruptHandler::new(timer_handler, timer_priority);
            }
        };

        timer.set_interrupt_handler(handler);
        timer.listen();

        Self { timer }
    }

    /// Set the next timer wakeup.
    ///
    /// # Panics
    /// - panics if the underlying timer's `schedule()` fails with an unexpected error
    pub(crate) fn arm_next_wakeup(&mut self, next_wakeup: u64) {
        let sleep_duration = next_wakeup.saturating_sub(now());

        // assume 52-bit underlying timer. it's not a big deal to sleep for a shorter time
        let mut timeout = sleep_duration & ((1 << 52) - 1);

        trace!("Arming timer for {} (target = {})", timeout, next_wakeup);

        loop {
            match self.timer.schedule(Duration::from_micros(timeout)) {
                Ok(()) => break,
                Err(esp_hal::timer::Error::InvalidTimeout) if timeout != 0 => {
                    timeout /= 2;
                }
                Err(e) => panic!("Failed to schedule timer: {:?}", e),
            }
        }
    }
}

#[esp_hal::ram]
extern "C" fn timer_handler() {
    ariel_os_debug::log::debug!("timer_handler()");

    let now = now();

    let next_wakeup = TIMER_QUEUE.handle_alarm(now);

    TIMER_DRIVER.with(|timer| {
        if let Some(timer) = timer {
            timer.timer.clear_interrupt();
            if next_wakeup < u64::MAX {
                timer.arm_next_wakeup(next_wakeup);
            }
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

    pub(crate) fn handle_alarm(&mut self, now: u64) -> u64 {
        if now >= self.next_wakeup {
            self.next_wakeup = self.queue.next_expiration(now);
        }

        self.next_wakeup
    }

    fn schedule_wake(&mut self, at: u64, waker: &Waker) -> bool {
        if self.queue.schedule_wake(at, waker) {
            self.next_wakeup = self.queue.next_expiration(now());
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

    pub(crate) fn handle_alarm(&self, now: u64) -> u64 {
        ariel_os_debug::log::debug!("handle_alarm() at {}", now);
        self.inner.with(|inner| inner.handle_alarm(now))
    }
}

impl embassy_time_driver::Driver for TimerQueue {
    #[inline]
    fn now(&self) -> u64 {
        now()
    }

    #[inline]
    fn schedule_wake(&self, at: u64, waker: &Waker) {
        self.inner.with(|inner| {
            if inner.schedule_wake(at, waker) {
                ariel_os_debug::log::debug!(
                    "schedule wake at {}, now={} next_wakeup={}",
                    at,
                    now(),
                    inner.next_wakeup
                );
                TIMER_DRIVER.with(|timer_driver| {
                    if let Some(driver) = timer_driver.as_mut() {
                        driver.arm_next_wakeup(inner.next_wakeup);
                    }
                });
            }
        });
    }
}
