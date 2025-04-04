#![no_main]
#![no_std]

use ariel_os::{
    debug::log::*,
    thread::{ThreadId, current_tid, sync::Channel, thread_flags},
};

static ID_EXCHANGE: Channel<ThreadId> = Channel::new();

#[ariel_os::thread(autostart)]
fn thread0() {
    let target_tid = ID_EXCHANGE.recv();
    ID_EXCHANGE.send(&current_tid().unwrap());

    match ariel_os::bench::benchmark(1000, || {
        thread_flags::set(target_tid, 1);
        thread_flags::wait_any(1);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(_) => warn!("benchmark returned error"),
    }
}

#[ariel_os::thread(autostart)]
fn thread1() {
    ID_EXCHANGE.send(&current_tid().unwrap());
    let target_tid = ID_EXCHANGE.recv();

    loop {
        thread_flags::set(target_tid, 1);
        thread_flags::wait_any(1);
    }
}
