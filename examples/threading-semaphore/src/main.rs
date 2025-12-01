#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, log::*};
use ariel_os::thread::{ThreadId, sync::Semaphore};

static SEMAPHORE: Semaphore = Semaphore::new(0, 10);

fn waiter() {
    let my_id = ariel_os::thread::current_tid().unwrap();
    let my_prio = ariel_os::thread::get_priority(my_id).unwrap();

    info!("[{:?}@{:?}] Taking semaphore...", my_id, my_prio);

    SEMAPHORE.take();

    info!("[{:?}@{:?}] Done.", my_id, my_prio);
}

#[ariel_os::thread(autostart, priority = 3)]
fn thread0() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 2)]
fn thread1() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 1)]
fn thread2() {
    let my_id = ariel_os::thread::current_tid().unwrap();
    let my_prio = ariel_os::thread::get_priority(my_id).unwrap();

    for i in 0..3 {
        info!("[{:?}@{:?}] Giving semaphore...", my_id, my_prio);
        SEMAPHORE.give();
        info!("[{:?}@{:?}] Give semaphore returned.", my_id, my_prio);
    }
    waiter();

    if SEMAPHORE.current_count() == 0 {
        info!(
            "[{:?}@{:?}] All three threads should have reported \"Done.\". exiting.",
            my_id, my_prio
        );
        ariel_os::debug::exit(ExitCode::SUCCESS);
    }
}
