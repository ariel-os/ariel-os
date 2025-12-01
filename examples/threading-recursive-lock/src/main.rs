#![no_main]
#![no_std]

use ariel_os::debug::{ExitCode, log::*};
use ariel_os::thread::{
    ThreadId,
    sync::{Lock, RecursiveLock},
    yield_same,
};

static LOCK: Lock = Lock::new();
static RLOCK: RecursiveLock = RecursiveLock::new();

fn thread_fn() {
    let my_id = ariel_os::thread::current_tid().unwrap();
    // Using a regular lock to order the threads in case of multi-core or infini-core.
    // This orders the starting of the threads.
    if LOCK.try_acquire() {
        LOCK.acquire();
    } else {
        LOCK.release();
    }

    for _ in 0..3 {
        info!("{:?}: getting lock...", my_id);
        RLOCK.acquire();
        yield_same();
    }
    loop {
        info!("{:?}: releasing lock...", my_id);
        let res = RLOCK.release();
        info!("{:?}: released. res={}", my_id, res);
        if res {
            break;
        }
    }
    info!("{:?}: done", my_id);
}

#[ariel_os::thread(autostart)]
fn thread0() {
    thread_fn();
}

#[ariel_os::thread(autostart)]
fn thread1() {
    thread_fn();
}
