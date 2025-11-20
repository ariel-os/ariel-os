#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, log::*};
use ariel_os::thread::{ThreadId, sync::WaitQueue};

static WAITQUEUE: WaitQueue = WaitQueue::new();

fn waiter() {
    let my_id = ariel_os::thread::current_tid().unwrap();
    let my_prio = ariel_os::thread::get_priority(my_id).unwrap();
    info!("[{:?}@{:?}] Waiting for event...", my_id, my_prio);

    WAITQUEUE.wait();
    info!("[{:?}@{:?}] Done.", my_id, my_prio);
}

#[ariel_os::thread(autostart, priority = 2)]
fn thread0() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 2)]
fn thread1() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 3)]
fn thread2() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 4)]
fn thread3() {
    waiter();
}

#[ariel_os::thread(autostart, priority = 1)]
fn thread4() {
    let my_id = ariel_os::thread::current_tid().unwrap();
    let my_prio = ariel_os::thread::get_priority(my_id).unwrap();
    info!("[{:?}@{:?}] Notifying wait queue...", my_id, my_prio);
    WAITQUEUE.notify_one();
    info!("[{:?}@{:?}] Wait queue notified.", my_id, my_prio);
}
