#![no_main]
#![no_std]

use ariel_os::{
    debug::{ExitCode, exit, log::info},
    thread::block_on,
    time::Timer,
};

#[ariel_os::thread(autostart)]
fn main() {
    info!("Hello from main thread!");

    for i in 0..10 {
        block_on(Timer::after_millis(100));
        info!("Hello again #{}", i);
    }

    exit(ExitCode::SUCCESS);
}
