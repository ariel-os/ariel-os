#![no_main]
#![no_std]

use portable_atomic::{AtomicUsize, Ordering};

use ariel_os::{
    debug::{ExitCode, exit},
    thread::{RunqueueId, ThreadId},
};

static RUN_ORDER: AtomicUsize = AtomicUsize::new(0);

static TEMP_THREAD1_PRIO: RunqueueId = RunqueueId::new(5);

#[ariel_os::thread(autostart, priority = 2)]
fn thread0() {
    let tid = ariel_os::thread::current_tid().unwrap();
    assert_eq!(
        ariel_os::thread::get_priority(tid),
        Some(RunqueueId::new(2))
    );

    assert_eq!(RUN_ORDER.fetch_add(1, Ordering::AcqRel), 0);

    let thread1_tid = ThreadId::new(1);
    assert_eq!(
        ariel_os::thread::get_priority(thread1_tid),
        Some(RunqueueId::new(1))
    );
    ariel_os::thread::set_priority(thread1_tid, TEMP_THREAD1_PRIO);

    // thread1 runs now.

    assert_eq!(RUN_ORDER.fetch_add(1, Ordering::AcqRel), 2);
    ariel_os::debug::log::info!("Test passed!");
    exit(ExitCode::Success);
}

#[ariel_os::thread(autostart, priority = 1)]
fn thread1() {
    // Thread can only run after thread0 increased its prio.
    assert_eq!(RUN_ORDER.fetch_add(1, Ordering::AcqRel), 1);
    // Prio is the temp increased prio.
    let tid = ariel_os::thread::current_tid().unwrap();
    assert_eq!(ariel_os::thread::get_priority(tid), Some(TEMP_THREAD1_PRIO));
    // Other thread prios didn't change.
    assert_eq!(
        ariel_os::thread::get_priority(ThreadId::new(0)),
        Some(RunqueueId::new(2))
    );

    // Reset priority.
    ariel_os::thread::set_priority(tid, RunqueueId::new(1));

    unreachable!("Core should be blocked by higher prio thread.")
}
