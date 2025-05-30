#![no_main]
#![no_std]

use ariel_os::debug::log::*;

#[ariel_os::thread(autostart)]
fn thread0() {
    info!("Hello from thread 0");
}

// `stacksize` and `priority` can be arbitrary expressions.
#[ariel_os::thread(autostart, stacksize = 512, priority = 2)]
fn thread1() {
    info!("Hello from thread 1");
}
