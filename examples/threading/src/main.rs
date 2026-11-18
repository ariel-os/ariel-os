#![no_main]
#![no_std]

use ariel_os::log::*;
use ariel_os_boards as _;

#[ariel_os::thread(autostart)]
fn thread0() {
    info!("Hello from thread 0");
}

// `stacksize` and `priority` can be arbitrary expressions.
#[ariel_os::thread(autostart, stacksize = 512, priority = 2)]
fn thread1() {
    info!("Hello from thread 1");
}
